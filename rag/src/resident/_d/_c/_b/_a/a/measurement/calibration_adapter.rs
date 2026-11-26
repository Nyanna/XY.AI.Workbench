use std::sync::Arc;
use std::time::Duration;
use super::result::{DeviationOutcome, TimeProbe};
use super::{Measure, MeasureConfig, MeasureError, DEFAULT_TOP_K};
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::a::calibration_run::{
    CalibrationBackend, CalibrationConfig, Combination, ProbeError,
};
const NANOS_PER_SECOND: f64 = 1e9;
fn nanos(d: Duration) -> u64 {
    u64::try_from(d.as_nanos()).unwrap_or(u64::MAX)
}
impl MeasureConfig {
    /// Timing and deviation limits taken from a calibration configuration.
    pub fn for_calibration(config: &CalibrationConfig) -> Self {
        MeasureConfig {
            window_ns: nanos(config.time_window),
            chunk_budget_ns: nanos(config.chunk_budget),
            max_deviation: config.max_deviation,
            top_k: DEFAULT_TOP_K,
        }
    }
}
fn probe_error(error: MeasureError) -> ProbeError {
    match error {
        MeasureError::Capability(_) => ProbeError::Unavailable,
        MeasureError::Config(text) => ProbeError::Fatal(text),
        other => ProbeError::Failed(other.to_string()),
    }
}
/// Runs a calibration against a `Measure` implementation.
///
/// Window and chunk budget are fixed by the `MeasureConfig` the measurer was built with;
/// see `MeasureConfig::for_calibration`.
pub struct MeasureBackend {
    measure: Arc<dyn Measure>,
}
impl MeasureBackend {
    pub fn new(measure: Arc<dyn Measure>) -> Self {
        MeasureBackend { measure }
    }
}
fn with_lengths(c: &Combination, seq_lens: &[usize]) -> Combination {
    c.clone().with_seq_lens(seq_lens.to_vec())
}
impl CalibrationBackend for MeasureBackend {
    fn deviation(&self, c: &Combination, seq_lens: &[usize]) -> Result<f64, ProbeError> {
        match self.measure.deviation(&with_lengths(c, seq_lens)) {
            Ok(DeviationOutcome::Measured { deviation, .. }) => Ok(deviation),
            Ok(DeviationOutcome::Incompatible { validity }) => {
                Err(
                    ProbeError::Rejected(
                        format!("{:?}: {}", validity.kind, validity.text),
                    ),
                )
            }
            Err(e) => Err(probe_error(e)),
        }
    }
    fn time(
        &self,
        c: &Combination,
        seq_lens: &[usize],
        _window: Duration,
        _chunk_budget: Duration,
    ) -> Result<Duration, ProbeError> {
        match self.measure.time_probe(&with_lengths(c, seq_lens)) {
            Ok(TimeProbe::Time(ns)) if ns.is_finite() && ns >= 0.0 => {
                Duration::try_from_secs_f64(ns / NANOS_PER_SECOND)
                    .map_err(|e| ProbeError::Failed(e.to_string()))
            }
            Ok(TimeProbe::Time(ns)) => {
                Err(ProbeError::Failed(format!("invalid time {ns}")))
            }
            Ok(TimeProbe::Interrupted) => Err(ProbeError::Interrupted),
            Err(e) => Err(probe_error(e)),
        }
    }
}
#[cfg(test)]
#[path = "test/calibration_adapter.rs"]
mod tests;
