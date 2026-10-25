//! Measurement abstraction between the calibrator and a concrete compute path.
use std::fmt;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};
use crate::resident::error::PrecisionError;
use crate::resident::subengine::DeviceHandle;
use super::profile::ProfileConfig;
/// One combination to measure: a value per axis plus the sequence lengths of the test set.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Cfg {
    pub combo: ProfileConfig,
    /// Lengths up to the maximum, to check processability (memory, overflow), not quality.
    pub seq_lens: Vec<usize>,
}
impl Cfg {
    pub fn new(combo: ProfileConfig, seq_lens: Vec<usize>) -> Self {
        Self { combo, seq_lens }
    }
}
impl fmt::Display for Cfg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, (axis, value)) in self.combo.iter().enumerate() {
            if i > 0 {
                f.write_str(",")?;
            }
            write!(f, "{axis}={value}")?;
        }
        Ok(())
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Validity {
    Ok,
    NonFinite,
    Degenerate,
    OverMaxDeviation,
    /// The combination could not run at all (allocation, launch, unsupported value).
    Failed,
}
impl Validity {
    pub fn is_ok(self) -> bool {
        self == Validity::Ok
    }
    pub fn as_str(self) -> &'static str {
        match self {
            Validity::Ok => "ok",
            Validity::NonFinite => "non-finite values",
            Validity::Degenerate => "degenerate vectors",
            Validity::OverMaxDeviation => "deviation above maximum",
            Validity::Failed => "combination failed to run",
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DeviationReport {
    /// Against the fp32 reference of the same test set.
    pub deviation: f64,
    pub valid: Validity,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sample {
    /// Time per work item, already normalised by the implementation.
    Time(Duration),
    /// A pending production task asked for the device between slices; nothing was measured.
    Interrupted,
}
/// The only way the calibrator obtains deviation and time. Real measurements and fakes
/// implement the same trait, so the calibration logic runs without a GPU.
pub trait Measure {
    /// Runs the combination once against the fp32 reference and judges the output.
    /// `Err(PrecisionError::Incompatible)` marks the combination as unusable.
    fn deviation(&mut self, cfg: &Cfg) -> Result<DeviationReport, PrecisionError>;
    /// One timing sample. The implementation warms up on first use, scales its iteration
    /// count to [`WindowTuning::window_ms`] and keeps single launches within
    /// [`WindowTuning::slice_budget_ms`].
    fn sample(&mut self, cfg: &Cfg) -> Result<Sample, PrecisionError>;
}
impl<T: Measure + ?Sized> Measure for Box<T> {
    fn deviation(&mut self, cfg: &Cfg) -> Result<DeviationReport, PrecisionError> {
        (**self).deviation(cfg)
    }
    fn sample(&mut self, cfg: &Cfg) -> Result<Sample, PrecisionError> {
        (**self).sample(cfg)
    }
}
/// Aggregated result of one combination.
#[derive(Clone, Debug, PartialEq)]
pub struct MeasureResult {
    /// Robust estimate (median or minimum) over the samples.
    pub time: Duration,
    pub deviation: f64,
    pub valid: Validity,
    /// `false` when the repetition or time cap ended the measurement first.
    pub stable: bool,
    pub samples: u32,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WindowTuning {
    /// Target length of one timing window.
    pub window_ms: f64,
    /// Upper bound for one uninterruptible launch group; the device cannot be preempted.
    pub slice_budget_ms: f64,
}
impl Default for WindowTuning {
    fn default() -> Self {
        Self {
            window_ms: 10.0,
            slice_budget_ms: 5.0,
        }
    }
}
/// Polled between slices: whether a production task is waiting for the device.
pub trait YieldCheck: Send + Sync {
    fn should_yield(&self) -> bool;
}
pub struct NeverYield;
impl YieldCheck for NeverYield {
    fn should_yield(&self) -> bool {
        false
    }
}
impl YieldCheck for AtomicBool {
    fn should_yield(&self) -> bool {
        self.load(Ordering::Relaxed)
    }
}
/// Monotonic time source, injectable so budgets are testable.
pub trait Clock: Send + Sync {
    fn elapsed(&self) -> Duration;
}
pub struct SystemClock {
    start: Instant,
}
impl SystemClock {
    pub fn new() -> Self {
        Self { start: Instant::now() }
    }
}
impl Default for SystemClock {
    fn default() -> Self {
        Self::new()
    }
}
impl Clock for SystemClock {
    fn elapsed(&self) -> Duration {
        self.start.elapsed()
    }
}
/// Real measurement for the device behind `device`. This is the only place that branches
/// on the `cuda` feature for measuring.
pub fn make_measure(
    device: &DeviceHandle,
    tuning: WindowTuning,
    yield_check: Arc<dyn YieldCheck>,
) -> Result<Box<dyn Measure>, PrecisionError> {
    use crate::resident::memory::DeviceKind;
    match device.kind() {
        DeviceKind::Cpu => {
            Ok(
                Box::new(
                    super::measure_cpu::CpuScoringMeasure::new(
                        super::workload::WorkloadSpec {
                            docs: 4_000,
                            ..Default::default()
                        },
                        tuning,
                        yield_check,
                    )?,
                ),
            )
        }
        DeviceKind::Gpu => {
            #[cfg(feature = "cuda")]
            {
                Ok(
                    Box::new(
                        super::measure_cuda::CudaScoringMeasure::new(
                            device,
                            super::workload::WorkloadSpec::default(),
                            tuning,
                            yield_check,
                        )?,
                    ),
                )
            }
            #[cfg(not(feature = "cuda"))]
            {
                let _ = (tuning, yield_check);
                Err(PrecisionError::Capability {
                    reason: "cuda feature disabled".into(),
                })
            }
        }
    }
}
