use super::bench_workload::Validity;
use super::MeasureError;
/// Deviation against the reference for one combination.
#[derive(Debug, Clone, PartialEq)]
pub enum DeviationOutcome {
    /// `Valid`; `deviation` is the worst deviation.
    Measured { deviation: f64, validity: Validity },
    /// Any other validity, or a combination that cannot be processed.
    Incompatible { validity: Validity },
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TimeProbe {
    /// Normalized time in ns per work unit (per query over the whole document set).
    Time(f64),
    /// A productive task asked for the device; the probe is discarded.
    Interrupted,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stability {
    Stable,
    Unstable,
}
pub const STABLE_MAX_SPREAD: f64 = 0.05;
const MAD_SCALE: f64 = 1.4826;
#[derive(Debug, Clone, PartialEq)]
pub struct MeasurementResult {
    pub robust_time_ns: f64,
    /// `None` when the combination is incompatible.
    pub deviation: Option<f64>,
    pub validity: Validity,
    pub stability: Stability,
    pub samples: usize,
    pub rel_uncertainty: f64,
}
fn median_of_sorted(sorted: &[f64]) -> f64 {
    let n = sorted.len();
    if n == 0 {
        return 0.0;
    }
    if n % 2 == 1 { sorted[n / 2] } else { (sorted[n / 2 - 1] + sorted[n / 2]) / 2.0 }
}
fn sorted_copy(values: &[f64]) -> Vec<f64> {
    let mut v = values.to_vec();
    v.sort_by(|a, b| a.total_cmp(b));
    v
}
impl MeasurementResult {
    pub fn from_samples(
        samples: &[f64],
        deviation: &DeviationOutcome,
    ) -> Result<Self, MeasureError> {
        if samples.is_empty() || samples.iter().any(|s| !s.is_finite() || *s < 0.0) {
            return Err(MeasureError::NoSamples);
        }
        let n = samples.len();
        let median = median_of_sorted(&sorted_copy(samples));
        let spread = if n < 2 {
            0.0
        } else {
            let deviations: Vec<f64> = samples
                .iter()
                .map(|s| (s - median).abs())
                .collect();
            let mad = median_of_sorted(&sorted_copy(&deviations));
            if median > 0.0 { MAD_SCALE * mad / median } else { 0.0 }
        };
        let rel_uncertainty = if n >= 2 {
            spread / (n as f64).sqrt()
        } else {
            f64::INFINITY
        };
        let stability = if n >= 2 && spread <= STABLE_MAX_SPREAD {
            Stability::Stable
        } else {
            Stability::Unstable
        };
        let (deviation, validity) = match deviation {
            DeviationOutcome::Measured { deviation, validity } => {
                (Some(*deviation), validity.clone())
            }
            DeviationOutcome::Incompatible { validity } => (None, validity.clone()),
        };
        Ok(MeasurementResult {
            robust_time_ns: median,
            deviation,
            validity,
            stability,
            samples: n,
            rel_uncertainty,
        })
    }
}
#[cfg(test)]
#[path = "test/result.rs"]
mod tests;
