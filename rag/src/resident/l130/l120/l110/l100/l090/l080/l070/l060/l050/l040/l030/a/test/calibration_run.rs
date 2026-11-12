use super::*;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::l010::layout::DType;
use std::collections::HashMap;

struct MockBackend {
    table: HashMap<Combination, (f64, u64)>,
    fatal_on: Option<Combination>,
    unavailable_time: bool,
}

impl MockBackend {
    fn new(entries: Vec<(Combination, f64, u64)>) -> Self {
        MockBackend {
            table: entries.into_iter().map(|(c, d, t)| (c, (d, t))).collect(),
            fatal_on: None,
            unavailable_time: false,
        }
    }
}

impl CalibrationBackend for MockBackend {
    fn deviation(&self, c: &Combination, _: &[usize]) -> Result<f64, ProbeError> {
        if self.fatal_on.as_ref() == Some(c) {
            return Err(ProbeError::Fatal("boom".into()));
        }
        Ok(self.table.get(c).map_or(0.0, |e| e.0))
    }

    fn time(
        &self,
        c: &Combination,
        _: &[usize],
        _: Duration,
        _: Duration,
    ) -> Result<Duration, ProbeError> {
        if self.unavailable_time {
            return Err(ProbeError::Unavailable);
        }
        Ok(Duration::from_micros(self.table.get(c).map_or(100, |e| e.1)))
    }
}

struct MockStats;

fn median(samples: &[f64]) -> f64 {
    let mut v = samples.to_vec();
    v.sort_by(|a, b| a.total_cmp(b));
    v[v.len() / 2]
}

impl Statistics for MockStats {
    fn dispersion(&self, samples: &[f64], kind: DispersionKind) -> Option<f64> {
        if samples.is_empty() {
            return None;
        }
        match kind {
            DispersionKind::Cv => {
                let n = samples.len() as f64;
                let mean = samples.iter().sum::<f64>() / n;
                if mean == 0.0 {
                    return None;
                }
                let var = samples.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / n;
                Some(var.sqrt() / mean)
            }
            DispersionKind::RangeOverMedian => {
                let m = median(samples);
                if m == 0.0 {
                    return None;
                }
                let max = samples.iter().cloned().fold(f64::MIN, f64::max);
                let min = samples.iter().cloned().fold(f64::MAX, f64::min);
                Some((max - min) / m)
            }
        }
    }

    fn estimate(&self, samples: &[f64], kind: EstimatorKind) -> Option<f64> {
        if samples.is_empty() {
            return None;
        }
        match kind {
            EstimatorKind::Median => Some(median(samples)),
            EstimatorKind::Minimum => samples.iter().cloned().fold(None, |a, x| match a {
                None => Some(x),
                Some(m) => Some(if x < m { x } else { m }),
            }),
        }
    }
}

fn combo(pairs: &[(&str, &str)]) -> Combination {
    Combination(
        pairs
            .iter()
            .map(|(a, v)| (Axis::new(a), Value::new(v)))
            .collect(),
    )
}

fn bt(batch: &str, tile: &str) -> Combination {
    Combination(
        [
            (axes::batch_size(), Value::new(batch)),
            (axes::tile_size(), Value::new(tile)),
        ]
        .into_iter()
        .collect(),
    )
}

fn caps() -> CapabilitySet {
    CapabilitySet::new()
        .with(axes::batch_size(), "1")
        .with(axes::batch_size(), "8")
        .with(axes::tile_size(), "16")
}

fn model() -> ModelDescriptor {
    ModelDescriptor::e5_small_repo_default(DType::F32)
}

fn target() -> Target {
    Target {
        subengine: "gpu".into(),
        model_id: model().model_id,
    }
}

fn config() -> CalibrationConfig {
    CalibrationConfig {
        backoff: Duration::ZERO,
        ..CalibrationConfig::default()
    }
}

fn backend() -> MockBackend {
    MockBackend::new(vec![
        (bt("1", "16"), 0.01, 200),
        (bt("8", "16"), 0.01, 100),
    ])
}

#[test]
fn sequence_lengths_cases() {
    assert_eq!(sequence_lengths(512, 16, 4), vec![16, 64, 256, 512]);
    assert_eq!(sequence_lengths(256, 16, 4), vec![16, 64, 256]);
    assert_eq!(sequence_lengths(8, 16, 4), vec![8]);
}

#[test]
fn value_order_numeric_before_text() {
    let set: std::collections::BTreeSet<Value> = ["10", "9", "abc", "2", "B"]
        .iter()
        .map(|s| Value::new(s))
        .collect();
    let sorted: Vec<String> = sort_values(&set)
        .iter()
        .map(|v| v.as_str().to_string())
        .collect();
    assert_eq!(sorted, vec!["2", "9", "10", "B", "abc"]);
}

#[test]
fn rejects_invalid_inputs() {
    let b = backend();
    let empty = CapabilitySet::new();
    assert_eq!(
        run_calibration(target(), &empty, &model(), &b, &MockStats, &config()).unwrap_err(),
        CalibrationError::Capability
    );
    let mut t = target();
    t.model_id = "other".into();
    assert_eq!(
        run_calibration(t, &caps(), &model(), &b, &MockStats, &config()).unwrap_err(),
        CalibrationError::TargetMismatch
    );
    let mut m = model();
    m.max_seq_len = 0;
    assert!(matches!(
        run_calibration(target(), &caps(), &m, &b, &MockStats, &config()).unwrap_err(),
        CalibrationError::Model(_)
    ));
}

#[test]
fn normal_mode_selects_faster_candidate() {
    let r = run_calibration(target(), &caps(), &model(), &backend(), &MockStats, &config()).unwrap();
    assert_eq!(r.product_size, 2);
    assert_eq!(r.summary.candidate_count, 1);
    assert_eq!(r.summary.front.len(), 1);
    assert_eq!(r.summary.front[0].combination, bt("8", "16"));
    assert_eq!(r.entries.iter().filter(|e| e.stage == Stage::Axis).count(), 2);
    assert_eq!(r.counters.measured, 3);
    assert_eq!(r.counters.stable, 3);
    assert!(!r.budget.exhausted);
}

#[test]
fn full_mode_measures_whole_product() {
    let cfg = CalibrationConfig {
        full_product: true,
        ..config()
    };
    let r = run_calibration(target(), &caps(), &model(), &backend(), &MockStats, &cfg).unwrap();
    assert!(r.entries.iter().all(|e| e.stage == Stage::Full));
    assert_eq!(r.summary.candidate_count, 2);
    assert_eq!(r.summary.front.len(), 1);
    assert_eq!(r.summary.front[0].combination, bt("8", "16"));
}

#[test]
fn deviation_above_max_is_discarded() {
    let b = MockBackend::new(vec![(bt("1", "16"), 0.5, 200), (bt("8", "16"), 0.01, 100)]);
    let cfg = CalibrationConfig {
        full_product: true,
        ..config()
    };
    let r = run_calibration(target(), &caps(), &model(), &b, &MockStats, &cfg).unwrap();
    assert_eq!(
        r.entries[0].outcome,
        Outcome::Discarded(DiscardReason::ExceedsMax)
    );
    assert_eq!(r.counters.discarded, 1);
    assert_eq!(r.summary.front.len(), 1);
}

#[test]
fn fatal_aborts_run() {
    let mut b = backend();
    b.fatal_on = Some(bt("8", "16"));
    let err = run_calibration(target(), &caps(), &model(), &b, &MockStats, &config()).unwrap_err();
    assert_eq!(err, CalibrationError::Backend("boom".into()));
}

#[test]
fn zero_budget_skips_everything() {
    let cfg = CalibrationConfig {
        total_budget: Some(Duration::ZERO),
        ..config()
    };
    let r = run_calibration(target(), &caps(), &model(), &backend(), &MockStats, &cfg).unwrap();
    assert!(r.budget.exhausted);
    assert!(r.summary.front.is_empty());
    assert!(r
        .entries
        .iter()
        .all(|e| e.outcome == Outcome::Skipped(SkipReason::Budget)));
}

#[test]
fn unavailable_device_is_skipped() {
    let mut b = backend();
    b.unavailable_time = true;
    let cfg = CalibrationConfig {
        full_product: true,
        ..config()
    };
    let r = run_calibration(target(), &caps(), &model(), &b, &MockStats, &cfg).unwrap();
    assert!(r
        .entries
        .iter()
        .all(|e| e.outcome == Outcome::Skipped(SkipReason::DeviceUnavailable)));
}

#[test]
fn core_axes_multiply_groups() {
    let c = caps().with(axes::operands(), "fp32").with(axes::operands(), "fp16");
    let b = MockBackend::new(vec![]);
    let r = run_calibration(target(), &c, &model(), &b, &MockStats, &config()).unwrap();
    assert_eq!(r.product_size, 4);
    let axis_entries = r.entries.iter().filter(|e| e.stage == Stage::Axis).count();
    assert_eq!(axis_entries, 4);
    let _ = combo(&[]);
}
