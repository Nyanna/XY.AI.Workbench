//! Synthetic scoring workload shared by the real measurements: duplicate-free unit vectors
//! and an fp32 host reference.

use crate::resident::error::PrecisionError;
use crate::resident::kernel::cpu_simd::{dot, normalize};

use super::known::BATCH;
use super::measure::{Cfg, DeviationReport};
use super::metrics::{judge_rows, Plausibility};

#[derive(Clone, Debug)]
pub struct WorkloadSpec {
    pub docs: usize,
    pub dim: usize,
    /// Upper bound of the `batch` axis; the queries of a smaller batch are a prefix.
    pub max_queries: usize,
    pub top_k: usize,
    pub seed: u64,
    pub plausibility: Plausibility,
}

impl WorkloadSpec {
    /// Queries per launch when the combination has no `batch` value.
    pub const DEFAULT_QUERIES: usize = 8;
}

impl Default for WorkloadSpec {
    fn default() -> Self {
        Self {
            docs: 20_000,
            dim: 384,
            max_queries: 32,
            top_k: 5,
            seed: 0x5eed_c0de,
            plausibility: Plausibility::default(),
        }
    }
}

fn xorshift(state: &mut u64) -> u64 {
    *state ^= *state >> 12;
    *state ^= *state << 25;
    *state ^= *state >> 27;
    state.wrapping_mul(0x2545_f491_4f6c_dd1d)
}

fn unit_rows(rows: usize, dim: usize, seed: u64) -> Vec<f32> {
    let mut state = seed | 1;
    let mut out = Vec::with_capacity(rows * dim);
    for _ in 0..rows {
        let start = out.len();
        out.extend((0..dim).map(|_| (xorshift(&mut state) >> 40) as f32 / (1u64 << 23) as f32 - 1.0));
        normalize(&mut out[start..]);
    }
    out
}

pub struct ScoringWorkload {
    spec: WorkloadSpec,
    docs: Vec<f32>,
    queries: Vec<f32>,
    /// Row-major `max_queries x docs`.
    reference: Vec<f32>,
}

impl ScoringWorkload {
    pub fn generate(spec: WorkloadSpec) -> Self {
        let docs = unit_rows(spec.docs, spec.dim, spec.seed);
        let queries = unit_rows(spec.max_queries, spec.dim, spec.seed ^ 0x9e37_79b9_7f4a_7c15);
        let mut reference = Vec::with_capacity(spec.max_queries * spec.docs);
        for q in queries.chunks(spec.dim) {
            reference.extend(docs.chunks(spec.dim).map(|d| dot(q, d)));
        }
        Self { spec, docs, queries, reference }
    }

    pub fn spec(&self) -> &WorkloadSpec {
        &self.spec
    }

    pub fn docs(&self) -> &[f32] {
        &self.docs
    }

    /// Query count of a combination, from its `batch` value.
    pub fn query_count(&self, cfg: &Cfg) -> Result<usize, PrecisionError> {
        let n = match cfg.combo.get(&BATCH) {
            None => WorkloadSpec::DEFAULT_QUERIES.min(self.spec.max_queries),
            Some(v) => v.as_str().parse::<usize>().map_err(|_| PrecisionError::Incompatible {
                reason: format!("batch value `{v}` is not a number"),
            })?,
        };
        if n == 0 || n > self.spec.max_queries {
            return Err(PrecisionError::Incompatible {
                reason: format!("batch {n} outside 1..={}", self.spec.max_queries),
            });
        }
        Ok(n)
    }

    pub fn queries(&self, count: usize) -> &[f32] {
        &self.queries[..count * self.spec.dim]
    }

    pub fn reference(&self, count: usize) -> &[f32] {
        &self.reference[..count * self.spec.docs]
    }

    /// `scores` is row-major `count x docs`.
    pub fn judge(&self, scores: &[f32], count: usize) -> DeviationReport {
        let (deviation, valid) = judge_rows(
            scores,
            self.reference(count),
            self.spec.docs,
            Some(self.spec.top_k),
            &self.spec.plausibility,
        );
        DeviationReport { deviation, valid }
    }
}

#[cfg(test)]
#[path = "test/workload.rs"]
mod tests;
