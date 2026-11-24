pub mod bench_workload;
pub mod calibration_adapter;
pub mod cpu_measurer;
pub mod gpu_measurer;
pub mod hooks;
pub mod result;
use std::collections::HashMap;
use std::fmt;
use std::sync::{Arc, Condvar, Mutex};
use crate::resident::l130::l120::l110::l100::l090::a::gpu_kernel::GpuKernelError;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::l010::capability_model::axes;
use self::bench_workload::{
    assess, execution_failed, query_count_from_value, Assessment, BenchError,
    DeviationConfig, Validity, ValidityKind, Workload, DEFAULT_TOP_K,
};
pub use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::l010::combination::{
    Combination, CombinationError,
};
pub use self::calibration_adapter::MeasureBackend;
pub use self::cpu_measurer::{CpuConfig, CpuMeasurer};
pub use self::gpu_measurer::{GpuMeasurer, GpuRig, TimingMark};
pub use self::hooks::{
    Clock, Hooks, MonotonicClock, NeverPending, PendingFlag, PendingTasks,
};
pub use self::result::{
    DeviationOutcome, MeasurementResult, Stability, TimeProbe, STABLE_MAX_SPREAD,
};
pub const DEFAULT_WINDOW_NS: u64 = 10_000_000;
pub const DEFAULT_CHUNK_BUDGET_NS: u64 = 5_000_000;
pub const CPU_DOCS: usize = 4_000;
pub const GPU_FLUSH_BYTES: u64 = 64 * 1024 * 1024;
pub const GPU_WARMUP_NS: u64 = 100_000_000;
pub const MIN_REPS: u32 = 1;
pub const MAX_REPS: u32 = 10_000;
/// Lower bound of a time sample, used when the clock reports no elapsed time.
const MIN_SAMPLE_NS: f64 = 1e-3;
#[derive(Debug, Clone, PartialEq)]
pub enum MeasureError {
    /// No GPU support.
    Capability(String),
    Config(String),
    NoSamples,
    Bench(BenchError),
    Gpu(GpuKernelError),
    /// Error text of the `GpuRig`.
    Rig(String),
    /// CPU scan or pool failure.
    Compute(String),
}
impl fmt::Display for MeasureError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MeasureError::Capability(text) => write!(f, "capability missing: {text}"),
            MeasureError::Config(text) => write!(f, "invalid configuration: {text}"),
            MeasureError::NoSamples => write!(f, "no usable samples"),
            MeasureError::Bench(e) => write!(f, "benchmark workload: {e}"),
            MeasureError::Gpu(e) => write!(f, "gpu kernel: {e}"),
            MeasureError::Rig(text) => write!(f, "gpu rig: {text}"),
            MeasureError::Compute(text) => write!(f, "cpu compute: {text}"),
        }
    }
}
impl std::error::Error for MeasureError {}
impl From<BenchError> for MeasureError {
    fn from(e: BenchError) -> Self {
        MeasureError::Bench(e)
    }
}
impl From<GpuKernelError> for MeasureError {
    fn from(e: GpuKernelError) -> Self {
        MeasureError::Gpu(e)
    }
}
impl MeasureError {
    /// The cause without the variant prefix.
    fn cause_text(&self) -> String {
        match self {
            MeasureError::Capability(t)
            | MeasureError::Config(t)
            | MeasureError::Rig(t)
            | MeasureError::Compute(t) => t.clone(),
            MeasureError::NoSamples => self.to_string(),
            MeasureError::Bench(e) => e.to_string(),
            MeasureError::Gpu(e) => e.to_string(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct MeasureConfig {
    /// Target duration of one probe; must be greater than zero.
    pub window_ns: u64,
    /// Longest stretch between two interruption checks; must be greater than zero.
    pub chunk_budget_ns: u64,
    /// Chosen by the caller.
    pub max_deviation: f64,
    pub top_k: usize,
}
impl MeasureConfig {
    pub fn new(max_deviation: f64) -> Self {
        MeasureConfig {
            window_ns: DEFAULT_WINDOW_NS,
            chunk_budget_ns: DEFAULT_CHUNK_BUDGET_NS,
            max_deviation,
            top_k: DEFAULT_TOP_K,
        }
    }
    fn validate_timing(&self) -> Result<(), MeasureError> {
        if self.window_ns == 0 {
            return Err(
                MeasureError::Config("window_ns must be greater than zero".to_string()),
            );
        }
        if self.chunk_budget_ns == 0 {
            return Err(
                MeasureError::Config(
                    "chunk_budget_ns must be greater than zero".to_string(),
                ),
            );
        }
        Ok(())
    }
    fn deviation_config(&self) -> Result<DeviationConfig, MeasureError> {
        Ok(DeviationConfig::new(self.max_deviation)?.with_top_k(self.top_k)?)
    }
}
pub trait Measure: Send + Sync {
    /// Deviation against the reference; warms up on first use.
    fn deviation(&self, combo: &Combination) -> Result<DeviationOutcome, MeasureError>;
    /// One time probe scaled to the time window; warms up on first use.
    fn time_probe(&self, combo: &Combination) -> Result<TimeProbe, MeasureError>;
}
/// Admits one measurement call at a time without holding a mutex while the call runs.
struct Gate {
    busy: Mutex<bool>,
    freed: Condvar,
}
struct GateGuard<'a> {
    gate: &'a Gate,
}
impl Gate {
    fn new() -> Self {
        Gate {
            busy: Mutex::new(false),
            freed: Condvar::new(),
        }
    }
    fn enter(&self) -> GateGuard<'_> {
        let mut busy = lock(&self.busy);
        while *busy {
            busy = self.freed.wait(busy).unwrap_or_else(|e| e.into_inner());
        }
        *busy = true;
        GateGuard { gate: self }
    }
}
impl Drop for GateGuard<'_> {
    fn drop(&mut self) {
        *lock(&self.gate.busy) = false;
        self.gate.freed.notify_one();
    }
}
/// Reference scores per query count, computed once.
#[derive(Default)]
struct ReferenceCache {
    scores: Mutex<HashMap<usize, Arc<Vec<f32>>>>,
}
impl ReferenceCache {
    fn get(&self, workload: &Workload, n: usize) -> Result<Arc<Vec<f32>>, BenchError> {
        let cached = lock(&self.scores).get(&n).cloned();
        if let Some(reference) = cached {
            return Ok(reference);
        }
        let reference = Arc::new(workload.reference_scores(n)?);
        lock(&self.scores).insert(n, Arc::clone(&reference));
        Ok(reference)
    }
}
/// Checks shared by both measurers; the first failure decides. Returns the query count and the
/// result of the measurer-specific precision check.
fn preflight<T>(
    combo: &Combination,
    max_queries: usize,
    precision: impl FnOnce(&Combination) -> Result<T, String>,
) -> Result<(usize, T), String> {
    if let Some(i) = combo.seq_lens.iter().position(|len| *len == 0) {
        return Err(format!("sequence length at position {i} is zero"));
    }
    let batch = combo.value(&axes::batch_size()).map(|v| v.as_str());
    let n = query_count_from_value(batch, max_queries).map_err(|e| e.to_string())?;
    let checked = precision(combo)?;
    Ok((n, checked))
}
fn incompatible(text: String) -> DeviationOutcome {
    let text = if text.is_empty() {
        "combination cannot be processed".to_string()
    } else {
        text
    };
    DeviationOutcome::Incompatible {
        validity: Validity {
            kind: ValidityKind::Degenerate,
            text,
        },
    }
}
fn outcome_of(assessment: Assessment) -> DeviationOutcome {
    match (assessment.validity.kind, assessment.worst_deviation) {
        (ValidityKind::Valid, Some(deviation)) => {
            DeviationOutcome::Measured {
                deviation,
                validity: assessment.validity,
            }
        }
        _ => {
            DeviationOutcome::Incompatible {
                validity: assessment.validity,
            }
        }
    }
}
/// Judges a candidate score matrix against the cached reference.
fn judge(
    workload: &Workload,
    references: &ReferenceCache,
    cfg: &DeviationConfig,
    n: usize,
    candidate: Result<&[f32], String>,
) -> Result<DeviationOutcome, MeasureError> {
    let assessment = match candidate {
        Err(text) => execution_failed(text),
        Ok(scores) => {
            let reference = references.get(workload, n)?;
            assess(&reference, scores, workload.config().docs, cfg)
        }
    };
    Ok(outcome_of(assessment))
}
/// Normalized sample in ns per query, never below `MIN_SAMPLE_NS`.
fn sample_ns(total_ns: f64, reps: u32, n: usize) -> f64 {
    let units = f64::from(reps) * n as f64;
    let sample = total_ns / units;
    if sample.is_finite() { sample.max(MIN_SAMPLE_NS) } else { MIN_SAMPLE_NS }
}
const fn assert_send_sync<T: Send + Sync>() {}
const _: () = {
    assert_send_sync::<Combination>();
    assert_send_sync::<MeasurementResult>();
    assert_send_sync::<MeasureError>();
    assert_send_sync::<Hooks>();
    assert_send_sync::<PendingFlag>();
};
#[cfg(test)]
#[path = "test/measurement.rs"]
mod tests;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::l010::sync_util::lock;
