//! Golden comparison of a candidate scorer against a reference scorer.
//!
//! Exact equality is never required: scores may drift within a tolerance and ranks may swap only
//! where the reference scores are within that tolerance of each other.

use super::{KernelError, KernelHit, QueryBatch, Scorer, SliceRange};
use crate::resident::subengine::ExecContext;

#[derive(Clone, Copy, Debug)]
pub struct Tolerance {
    pub abs: f32,
}

impl Tolerance {
    pub const F32: Self = Self { abs: 1e-5 };
    pub const F16: Self = Self { abs: 1e-3 };
}

#[derive(Clone, Debug, PartialEq)]
pub enum Deviation {
    HitCount { query: usize, expected: usize, actual: usize },
    ScoreDrift { query: usize, rank: usize, delta: f32 },
    RankMismatch { query: usize, rank: usize, expected: u64, actual: u64 },
}

pub fn compare(
    reference: &[Vec<KernelHit>],
    candidate: &[Vec<KernelHit>],
    tol: Tolerance,
) -> Vec<Deviation> {
    let mut out = Vec::new();
    for (query, (r, c)) in reference.iter().zip(candidate).enumerate() {
        if r.len() != c.len() {
            out.push(Deviation::HitCount {
                query,
                expected: r.len(),
                actual: c.len(),
            });
            continue;
        }
        for (rank, (a, b)) in r.iter().zip(c).enumerate() {
            let delta = (a.score - b.score).abs();
            if delta > tol.abs {
                out.push(Deviation::ScoreDrift { query, rank, delta });
            } else if a.offset != b.offset && !near_tie(r, rank, tol) {
                out.push(Deviation::RankMismatch {
                    query,
                    rank,
                    expected: a.offset,
                    actual: b.offset,
                });
            }
        }
    }
    if reference.len() != candidate.len() {
        out.push(Deviation::HitCount {
            query: reference.len().min(candidate.len()),
            expected: reference.len(),
            actual: candidate.len(),
        });
    }
    out
}

fn near_tie(hits: &[KernelHit], rank: usize, tol: Tolerance) -> bool {
    let s = hits[rank].score;
    let before = rank > 0 && (hits[rank - 1].score - s).abs() <= tol.abs;
    let after = hits.get(rank + 1).is_some_and(|n| (n.score - s).abs() <= tol.abs);
    before || after
}

/// Runs both scorers over `ranges` and reports deviations as diagnostics, never as panics.
pub fn check(
    reference: &dyn Scorer,
    candidate: &dyn Scorer,
    queries: &QueryBatch,
    ranges: &[SliceRange],
    lane: &mut dyn ExecContext,
    tol: Tolerance,
) -> Result<Vec<Deviation>, KernelError> {
    let mut all = Vec::new();
    for &range in ranges {
        let r = reference.score(queries, range, lane)?;
        let c = candidate.score(queries, range, lane)?;
        let found = compare(&r, &c, tol);
        for d in &found {
            tracing::warn!(segment = range.segment, deviation = ?d, "golden deviation");
        }
        all.extend(found);
    }
    Ok(all)
}

#[cfg(test)]
#[path = "test/golden.rs"]
mod tests;
