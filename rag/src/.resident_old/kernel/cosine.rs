//! Dense scoring path: cosine similarity as the dot product of L2-normalised vectors.
use std::sync::Arc;
use super::cpu_simd::{dot, scan_slots};
use super::{KernelError, QueryBatch, SliceRange, SlotScorer, SlotSource, TopKScorer};
/// Squared query norm below which a query counts as empty (norm 1e-9).
const MIN_NORM_SQ: f32 = 1e-18;
pub struct CosineSlots {
    source: Arc<dyn SlotSource>,
    exact: bool,
}
impl CosineSlots {
    /// Plain dot product; valid for unit-length vectors.
    pub fn new(source: Arc<dyn SlotSource>) -> Self {
        Self { source, exact: false }
    }
    /// Divides by both norms, for slot vectors whose norm drifted (e.g. f16 storage).
    pub fn exact(source: Arc<dyn SlotSource>) -> Self {
        Self { source, exact: true }
    }
}
impl SlotScorer for CosineSlots {
    fn scores(
        &self,
        queries: &QueryBatch,
        range: SliceRange,
    ) -> Result<Vec<Vec<f32>>, KernelError> {
        let dim = queries.dim();
        for qi in 0..queries.len() {
            if queries.tokens(qi) != 1 {
                return Err(KernelError::ShapeMismatch {
                    expected: dim,
                    actual: queries.data(qi).len(),
                });
            }
            let q = queries.data(qi);
            let norm_sq = dot(q, q);
            if !norm_sq.is_finite() || norm_sq < MIN_NORM_SQ {
                return Err(KernelError::EmptyEmbedding {
                    query: qi,
                });
            }
        }
        let exact = self.exact;
        scan_slots(
            self.source.as_ref(),
            queries,
            range,
            |q, d| {
                if d.len() != dim {
                    return Err(KernelError::ShapeMismatch {
                        expected: dim,
                        actual: d.len(),
                    });
                }
                let raw = dot(q, d);
                if !exact {
                    return Ok(raw);
                }
                let denom = (dot(q, q) * dot(d, d)).sqrt();
                Ok(if denom > 0.0 && denom.is_finite() { raw / denom } else { 0.0 })
            },
        )
    }
    fn slot_offset(&self, segment: u32, slot: u32) -> u64 {
        self.source.slot_offset(segment, slot)
    }
}
pub type CosineScorer = TopKScorer<CosineSlots>;
pub fn cosine_scorer(source: Arc<dyn SlotSource>, k: usize) -> CosineScorer {
    TopKScorer::new(CosineSlots::new(source), k)
}
pub fn cosine_scorer_exact(source: Arc<dyn SlotSource>, k: usize) -> CosineScorer {
    TopKScorer::new(CosineSlots::exact(source), k)
}
#[cfg(test)]
#[path = "test/cosine.rs"]
mod tests;
