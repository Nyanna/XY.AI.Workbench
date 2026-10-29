//! Real CPU measurement of the fp32 scoring path. Only the axes this workload reacts to are
//! read from a combination: `batch` (queries per pass), `threads`, and the operand/scoring
//! type, which must be fp32 because the host kernels exist for nothing else.

use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::time::{Duration, Instant};

use rayon::prelude::*;

use crate::resident::error::PrecisionError;
use crate::resident::kernel::cpu_simd::dot;

use super::known::{FP32, OPERANDS, SCORING, THREADS};
use super::measure::{Cfg, DeviationReport, Measure, Sample, WindowTuning, YieldCheck};
use super::workload::{ScoringWorkload, WorkloadSpec};

pub struct CpuScoringMeasure {
    load: ScoringWorkload,
    tuning: WindowTuning,
    yield_check: Arc<dyn YieldCheck>,
    pools: HashMap<usize, rayon::ThreadPool>,
    warmed: HashSet<(usize, usize)>,
}

impl CpuScoringMeasure {
    pub fn new(
        spec: WorkloadSpec,
        tuning: WindowTuning,
        yield_check: Arc<dyn YieldCheck>,
    ) -> Result<Self, PrecisionError> {
        Ok(Self {
            load: ScoringWorkload::generate(spec),
            tuning,
            yield_check,
            pools: HashMap::new(),
            warmed: HashSet::new(),
        })
    }

    fn threads(cfg: &Cfg) -> Result<usize, PrecisionError> {
        match cfg.combo.get(&THREADS) {
            None => Ok(1),
            Some(v) => v.as_str().parse::<usize>().ok().filter(|n| *n > 0).ok_or_else(|| {
                PrecisionError::Incompatible { reason: format!("threads value `{v}` invalid") }
            }),
        }
    }

    fn check_types(cfg: &Cfg) -> Result<(), PrecisionError> {
        for axis in [&*OPERANDS, &*SCORING] {
            if let Some(v) = cfg.combo.get(axis) {
                if v != &*FP32 {
                    return Err(PrecisionError::Incompatible {
                        reason: format!("no cpu kernel for {axis}={v}"),
                    });
                }
            }
        }
        Ok(())
    }

    /// Returns the query count and thread count after validating the combination.
    fn plan(&mut self, cfg: &Cfg) -> Result<(usize, usize), PrecisionError> {
        Self::check_types(cfg)?;
        let nq = self.load.query_count(cfg)?;
        let threads = Self::threads(cfg)?;
        if !self.pools.contains_key(&threads) {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(threads)
                .thread_name(|i| format!("cpu-measure-{i}"))
                .build()
                .map_err(|e| PrecisionError::Incompatible { reason: e.to_string() })?;
            self.pools.insert(threads, pool);
        }
        Ok((nq, threads))
    }

    /// Scores into `out`, document-major (`docs x nq`).
    fn score(&self, threads: usize, nq: usize, out: &mut [f32]) {
        let dim = self.load.spec().dim;
        let queries = self.load.queries(nq);
        self.pools[&threads].install(|| {
            out.par_chunks_mut(nq).zip(self.load.docs().par_chunks(dim)).for_each(|(row, doc)| {
                for (q, slot) in row.iter_mut().enumerate() {
                    *slot = dot(&queries[q * dim..(q + 1) * dim], doc);
                }
            });
        });
    }
}

impl Measure for CpuScoringMeasure {
    fn deviation(&mut self, cfg: &Cfg) -> Result<DeviationReport, PrecisionError> {
        let (nq, threads) = self.plan(cfg)?;
        let docs = self.load.spec().docs;
        let mut doc_major = vec![0f32; docs * nq];
        self.score(threads, nq, &mut doc_major);
        let mut scores = vec![0f32; docs * nq];
        for (s, row) in doc_major.chunks(nq).enumerate() {
            for (q, v) in row.iter().enumerate() {
                scores[q * docs + s] = *v;
            }
        }
        Ok(self.load.judge(&scores, nq))
    }

    fn sample(&mut self, cfg: &Cfg) -> Result<Sample, PrecisionError> {
        let (nq, threads) = self.plan(cfg)?;
        let mut out = vec![0f32; self.load.spec().docs * nq];
        if self.warmed.insert((threads, nq)) {
            self.score(threads, nq, &mut out);
        }
        let window = Duration::from_secs_f64(self.tuning.window_ms / 1e3);
        let start = Instant::now();
        let mut passes = 0u32;
        loop {
            self.score(threads, nq, &mut out);
            passes += 1;
            if start.elapsed() >= window {
                break;
            }
            if self.yield_check.should_yield() {
                return Ok(Sample::Interrupted);
            }
        }
        std::hint::black_box(&out);
        Ok(Sample::Time(start.elapsed().div_f64(f64::from(passes) * nq as f64)))
    }
}

#[cfg(test)]
#[path = "test/measure_cpu.rs"]
mod tests;
