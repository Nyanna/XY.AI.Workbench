use std::collections::HashSet;
use std::ops::Range;
use std::sync::{Arc, Mutex};
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::a::kernel_layout::Slot5;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::a::cpu_compute::scan_slots;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::_1::background_pool::BackgroundPool;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::_1::capability_model::{
    axes, values,
};
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::query_batch::DocSource;
use super::bench_workload::{Workload, WorkloadConfig};
use super::{
    incompatible6, judge3, lock, preflight, sample_ns, Combination, DeviationOutcome, Gate3,
    Hooks, Measure, MeasureConfig, MeasureError, ReferenceCache, TimeProbe, CPU_DOCS,
    MAX_REPS,
};
pub const DEFAULT_SEED: u64 = 0x5EED_0127;
#[derive(Debug, Clone)]
pub struct CpuConfig {
    pub common: MeasureConfig,
    /// Number of pool workers; must be at least 1.
    pub threads: usize,
    pub seed: u64,
}
impl CpuConfig {
    pub fn new(max_deviation: f64) -> Self {
        CpuConfig {
            common: MeasureConfig::new(max_deviation),
            threads: 1,
            seed: DEFAULT_SEED,
        }
    }
}
pub struct CpuMeasurer {
    cfg: CpuConfig,
    hooks: Hooks,
    workload: Arc<Workload>,
    pool: BackgroundPool,
    gate: Gate3,
    warmed: Mutex<HashSet<(String, usize)>>,
    references: ReferenceCache,
}
const POOL_NAME: &str = "measure-cpu";
/// Only fp32 is supported by the cpu scorer.
fn cpu_precision(combo: &Combination) -> Result<(), String> {
    for axis in [axes::scoring(), axes::operands()] {
        if let Some(value) = combo.value(&axis) {
            if *value != values::fp32() {
                return Err(
                    format!(
                        "axis {axis}: value {value} is not supported by the cpu scorer (fp32 only)"
                    ),
                );
            }
        }
    }
    Ok(())
}
/// Contiguous, non-empty ranges that cover `0..docs` exactly.
fn split_ranges(docs: usize, parts: usize) -> Vec<Range<usize>> {
    let parts = parts.clamp(1, docs.max(1));
    let base = docs / parts;
    let extra = docs % parts;
    let mut ranges = Vec::with_capacity(parts);
    let mut start = 0;
    for i in 0..parts {
        let len = base + usize::from(i < extra);
        if len > 0 {
            ranges.push(start..start + len);
        }
        start += len;
    }
    ranges
}
fn scan_range(
    workload: &Workload,
    n: usize,
    range: Range<usize>,
) -> Result<Vec<f32>, String> {
    let source = workload.source();
    let segment = workload.segment();
    let mut queries: Vec<&[f32]> = Vec::with_capacity(n);
    for i in 0..n {
        queries
            .push(
                workload
                    .queries()
                    .data(i)
                    .ok_or_else(|| format!("query {i} is missing"))?,
            );
    }
    let slots: Vec<Option<&[f32]>> = range
        .map(|i| source.slot_vectors(segment, Slot5(i as u32)))
        .collect();
    scan_slots(&queries, &slots, 0..slots.len(), workload.config().dim)
        .map_err(|e| e.to_string())
}
impl CpuMeasurer {
    pub fn new(cfg: CpuConfig, hooks: Hooks) -> Result<Self, MeasureError> {
        if cfg.threads == 0 {
            return Err(MeasureError::Config("threads must be at least 1".to_string()));
        }
        cfg.common.validate_timing()?;
        let mut wc = WorkloadConfig::new(cfg.seed);
        wc.docs = CPU_DOCS;
        let workload = Arc::new(Workload::build(wc)?);
        let pool = BackgroundPool::new(POOL_NAME, cfg.threads)
            .map_err(|e| MeasureError::Compute(
                format!("cannot create worker pool: {e}"),
            ))?;
        Ok(CpuMeasurer {
            cfg,
            hooks,
            workload,
            pool,
            gate: Gate3::new(),
            warmed: Mutex::new(HashSet::new()),
            references: ReferenceCache::default(),
        })
    }
    /// Query-major score matrix (`q * docs + d`) of the first `n` queries over all documents.
    fn scan(&self, n: usize) -> Result<Vec<f32>, String> {
        let docs = self.workload.config().docs;
        let ranges = split_ranges(docs, self.cfg.threads);
        let mut jobs = Vec::with_capacity(ranges.len());
        for range in &ranges {
            let workload = Arc::clone(&self.workload);
            let range = range.clone();
            let job = self
                .pool
                .submit(move || scan_range(&workload, n, range))
                .map_err(|e| e.to_string())?;
            jobs.push(job);
        }
        let mut parts: Vec<Vec<f32>> = Vec::with_capacity(jobs.len());
        let mut failure: Option<String> = None;
        for job in jobs {
            match job.wait() {
                Ok(Ok(part)) => parts.push(part),
                Ok(Err(text)) => {
                    failure.get_or_insert(text);
                }
                Err(e) => {
                    failure.get_or_insert(e.to_string());
                }
            }
        }
        if let Some(text) = failure {
            return Err(text);
        }
        if parts.len() == 1 {
            return Ok(parts.swap_remove(0));
        }
        let mut out = vec![0.0f32; n * docs];
        for (range, part) in ranges.iter().zip(&parts) {
            let len = range.len();
            for q in 0..n {
                let dst = q * docs + range.start;
                out[dst..dst + len].copy_from_slice(&part[q * len..(q + 1) * len]);
            }
        }
        Ok(out)
    }
    fn warm_up(&self, key: &(String, usize)) -> Result<(), String> {
        if lock(&self.warmed).contains(key) {
            return Ok(());
        }
        self.scan(key.1)?;
        lock(&self.warmed).insert(key.clone());
        Ok(())
    }
    /// Duration of one scan in ns, taken from the clock.
    fn timed_pass(&self, n: usize) -> Result<f64, MeasureError> {
        let begin = self.hooks.clock.now_ns();
        let scores = self.scan(n).map_err(MeasureError::Compute)?;
        let end = self.hooks.clock.now_ns();
        drop(scores);
        Ok(end.saturating_sub(begin) as f64)
    }
}
impl Measure for CpuMeasurer {
    fn deviation(&self, combo: &Combination) -> Result<DeviationOutcome, MeasureError> {
        let _turn = self.gate.enter();
        let max_queries = self.workload.config().max_queries;
        let (n, ()) = match preflight(combo, max_queries, cpu_precision) {
            Ok(checked) => checked,
            Err(text) => return Ok(incompatible6(text)),
        };
        let dev_cfg = self.cfg.common.deviation_config()?;
        let key = (combo.to_text(), n);
        let scores = self.warm_up(&key).and_then(|()| self.scan(n));
        judge3(
            &self.workload,
            &self.references,
            &dev_cfg,
            n,
            scores.as_deref().map_err(|e| e.clone()),
        )
    }
    fn time_probe(&self, combo: &Combination) -> Result<TimeProbe, MeasureError> {
        let _turn = self.gate.enter();
        let max_queries = self.workload.config().max_queries;
        let (n, ()) = preflight(combo, max_queries, cpu_precision)
            .map_err(MeasureError::Config)?;
        let key = (combo.to_text(), n);
        self.warm_up(&key).map_err(MeasureError::Compute)?;
        if self.hooks.pending.productive_waiting() {
            return Ok(TimeProbe::Interrupted);
        }
        let first = self.timed_pass(n)?;
        let window = self.cfg.common.window_ns as f64;
        let passes = if first > 0.0 {
            (window / first).ceil().clamp(1.0, f64::from(MAX_REPS)) as u32
        } else {
            MAX_REPS
        };
        let mut total = first;
        for _ in 1..passes {
            if self.hooks.pending.productive_waiting() {
                return Ok(TimeProbe::Interrupted);
            }
            total += self.timed_pass(n)?;
        }
        Ok(TimeProbe::Time(sample_ns(total, passes, n)))
    }
}
const _: () = super::assert_send_sync::<CpuMeasurer>();
#[cfg(test)]
#[path = "test/cpu_measurer.rs"]
mod tests;
