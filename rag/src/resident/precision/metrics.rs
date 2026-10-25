//! Deviation metrics against the fp32 reference and plausibility checks.

use std::collections::HashSet;
use std::hash::Hash;

use super::measure::Validity;

/// `1 - cos` below this is rounding noise (it even goes negative) and counts as equal.
pub const NOISE_FLOOR: f64 = 1e-6;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Plausibility {
    /// Candidate norm relative to the reference norm must stay inside this range.
    pub norm_ratio: (f64, f64),
    /// Variance relative to the mean square; below this the vector is constant.
    pub min_rel_variance: f64,
}

impl Default for Plausibility {
    fn default() -> Self {
        Self { norm_ratio: (0.5, 2.0), min_rel_variance: 1e-9 }
    }
}

fn floor_noise(d: f64) -> f64 {
    if d < NOISE_FLOOR {
        0.0
    } else {
        d
    }
}

fn norm(v: &[f32]) -> f64 {
    v.iter().map(|&x| f64::from(x) * f64::from(x)).sum::<f64>().sqrt()
}

/// `max(0, 1 - cos(a, reference))`, 1 when either vector is zero.
pub fn cosine_deviation(a: &[f32], reference: &[f32]) -> f64 {
    let dot: f64 = a.iter().zip(reference).map(|(&x, &y)| f64::from(x) * f64::from(y)).sum();
    let (na, nb) = (norm(a), norm(reference));
    if na == 0.0 || nb == 0.0 {
        return 1.0;
    }
    floor_noise((1.0 - dot / (na * nb)).max(0.0))
}

/// `||a - reference|| / ||reference||`.
pub fn relative_error(a: &[f32], reference: &[f32]) -> f64 {
    let diff: f64 = a
        .iter()
        .zip(reference)
        .map(|(&x, &y)| (f64::from(x) - f64::from(y)).powi(2))
        .sum::<f64>()
        .sqrt();
    let n = norm(reference);
    if n == 0.0 {
        return if diff == 0.0 { 0.0 } else { f64::INFINITY };
    }
    diff / n
}

fn top_k(scores: &[f32], k: usize) -> Vec<usize> {
    let mut idx: Vec<usize> = (0..scores.len()).collect();
    idx.sort_by(|&a, &b| scores[b].total_cmp(&scores[a]).then(a.cmp(&b)));
    idx.truncate(k);
    idx
}

/// Share of the reference top-`k` that the candidate also ranks in its top-`k`.
pub fn top_k_overlap(a: &[f32], reference: &[f32], k: usize) -> f64 {
    let k = k.min(a.len()).min(reference.len());
    if k == 0 {
        return 1.0;
    }
    let want: HashSet<usize> = top_k(reference, k).into_iter().collect();
    let hits = top_k(a, k).into_iter().filter(|i| want.contains(i)).count();
    hits as f64 / k as f64
}

fn is_constant(v: &[f32], min_rel_variance: f64) -> bool {
    if v.is_empty() {
        return true;
    }
    let n = v.len() as f64;
    let mean = v.iter().map(|&x| f64::from(x)).sum::<f64>() / n;
    let mean_sq = v.iter().map(|&x| f64::from(x) * f64::from(x)).sum::<f64>() / n;
    let variance = mean_sq - mean * mean;
    mean_sq == 0.0 || variance <= min_rel_variance * mean_sq
}

/// Judges `candidate` against `reference`, both made of rows of `dim` values.
/// Returns the worst row deviation (cosine, plus `1 - top-k overlap` when `rank_k` is set)
/// and the first failing validity.
pub fn judge_rows(
    candidate: &[f32],
    reference: &[f32],
    dim: usize,
    rank_k: Option<usize>,
    p: &Plausibility,
) -> (f64, Validity) {
    if dim == 0 || candidate.len() != reference.len() || candidate.len() % dim != 0 {
        return (f64::INFINITY, Validity::Degenerate);
    }
    if candidate.iter().any(|x| !x.is_finite()) {
        return (f64::INFINITY, Validity::NonFinite);
    }
    let mut worst = 0.0f64;
    let mut validity = Validity::Ok;
    for (c, r) in candidate.chunks(dim).zip(reference.chunks(dim)) {
        let ratio = norm(c) / norm(r).max(f64::MIN_POSITIVE);
        if is_constant(c, p.min_rel_variance) || ratio < p.norm_ratio.0 || ratio > p.norm_ratio.1
        {
            validity = Validity::Degenerate;
        }
        let mut dev = cosine_deviation(c, r);
        if let Some(k) = rank_k {
            dev = dev.max(floor_noise(1.0 - top_k_overlap(c, r, k)));
        }
        worst = worst.max(dev);
    }
    (worst, validity)
}

/// Rank metrics are meaningless with ties, so test sets must not contain duplicates.
pub fn is_duplicate_free<T: Hash + Eq>(items: &[T]) -> bool {
    let mut seen = HashSet::with_capacity(items.len());
    items.iter().all(|i| seen.insert(i))
}

#[cfg(test)]
#[path = "test/metrics.rs"]
mod tests;
