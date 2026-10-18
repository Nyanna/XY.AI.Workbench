//! CPU scoring primitives; loops are shaped for autovectorisation (8-wide accumulators).

use std::sync::Arc;

use super::slots::check_range;
use super::{KernelError, QueryBatch, SliceRange, SlotScorer, SlotSource};
use crate::resident::memory::F16_MASK_VALUE;

pub fn dot(a: &[f32], b: &[f32]) -> f32 {
    debug_assert_eq!(a.len(), b.len());
    let ca = a.chunks_exact(8);
    let cb = b.chunks_exact(8);
    let (ra, rb) = (ca.remainder(), cb.remainder());
    let mut acc = [0f32; 8];
    for (x, y) in ca.zip(cb) {
        for i in 0..8 {
            acc[i] += x[i] * y[i];
        }
    }
    let tail: f32 = ra.iter().zip(rb).map(|(x, y)| x * y).sum();
    acc.iter().sum::<f32>() + tail
}

/// Sum over query tokens of the best dot product against any document token. A document without
/// tokens scores every query token with the f16-safe mask value, matching the GPU path.
pub fn maxsim(query: &[f32], doc: &[f32], dim: usize) -> f32 {
    let q_tokens = query.len() / dim;
    if doc.is_empty() {
        return F16_MASK_VALUE * q_tokens as f32;
    }
    query
        .chunks_exact(dim)
        .map(|q| doc.chunks_exact(dim).fold(f32::NEG_INFINITY, |best, d| best.max(dot(q, d))))
        .sum()
}

pub fn normalize(v: &mut [f32]) {
    let norm = dot(v, v).sqrt();
    if norm > 0.0 {
        v.iter_mut().for_each(|x| *x /= norm);
    }
}

/// Slot-wise scorer: `score(query_i, slot)`; slots are the outer loop so a slot is read once for
/// every query of the batch.
pub(crate) fn scan_slots(
    source: &dyn SlotSource,
    queries: &QueryBatch,
    range: SliceRange,
    mut score: impl FnMut(&[f32], &[f32]) -> Result<f32, KernelError>,
) -> Result<Vec<Vec<f32>>, KernelError> {
    check_range(source, range)?;
    if queries.dim() != source.dim() {
        return Err(KernelError::ShapeMismatch { expected: source.dim(), actual: queries.dim() });
    }
    let mut out = vec![Vec::with_capacity(range.len() as usize); queries.len()];
    for slot in range.slot_from..range.slot_to {
        let doc = source.slot_vectors(range.segment, slot).unwrap_or(&[]);
        if doc.len() % queries.dim() != 0 {
            return Err(KernelError::ShapeMismatch { expected: queries.dim(), actual: doc.len() });
        }
        for (qi, scores) in out.iter_mut().enumerate() {
            scores.push(score(queries.data(qi), doc)?);
        }
    }
    Ok(out)
}

pub struct CpuMaxSim {
    source: Arc<dyn SlotSource>,
}

impl CpuMaxSim {
    pub fn new(source: Arc<dyn SlotSource>) -> Self {
        Self { source }
    }
}

impl SlotScorer for CpuMaxSim {
    fn scores(&self, queries: &QueryBatch, range: SliceRange) -> Result<Vec<Vec<f32>>, KernelError> {
        let dim = queries.dim();
        scan_slots(self.source.as_ref(), queries, range, |q, d| Ok(maxsim(q, d, dim)))
    }

    fn slot_offset(&self, segment: u32, slot: u32) -> u64 {
        self.source.slot_offset(segment, slot)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dot_handles_remainders() {
        for n in [0usize, 1, 7, 8, 9, 17, 384] {
            let a: Vec<f32> = (0..n).map(|i| i as f32 * 0.5).collect();
            let b: Vec<f32> = (0..n).map(|i| 1.0 - i as f32 * 0.25).collect();
            let want: f32 = a.iter().zip(&b).map(|(x, y)| x * y).sum();
            assert!((dot(&a, &b) - want).abs() <= 1e-3 * want.abs().max(1.0), "n={n}");
        }
    }

    #[test]
    fn maxsim_matches_hand_computation() {
        let q = [1.0, 0.0, 0.0, 1.0];
        let d = [0.5, 0.2, 0.1, 0.9, 0.3, 0.3];
        assert!((maxsim(&q, &d, 2) - (0.5 + 0.9)).abs() < 1e-6);
    }

    #[test]
    fn empty_document_scores_mask_value_per_query_token() {
        let q = [1.0, 0.0, 0.0, 1.0];
        let s = maxsim(&q, &[], 2);
        assert_eq!(s, 2.0 * F16_MASK_VALUE);
        assert!(s.is_finite());
    }

    #[test]
    fn normalize_yields_unit_length() {
        let mut v = vec![3.0, 4.0];
        normalize(&mut v);
        assert!((dot(&v, &v) - 1.0).abs() < 1e-6);
        let mut z = vec![0.0; 3];
        normalize(&mut z);
        assert_eq!(z, vec![0.0; 3]);
    }
}
