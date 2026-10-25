use super::*;
use crate::resident::mock::NoiseModel;
use crate::resident::precision::measure_mock::{FakeCombo, FakeMeasure, ManualClock};
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
const SEEDS: [u64; 3] = [1, 7, 42];
fn target() -> CalibrationTarget {
    CalibrationTarget {
        engine: SubengineKind::Cuda,
        model_id: "model".into(),
    }
}
fn quick() -> CalibratorConfig {
    CalibratorConfig {
        hard_cap: HardCap {
            max_reps: 12,
            max_time: None,
        },
        total_budget: None,
        interrupt_backoff: Duration::ZERO,
        ..CalibratorConfig::default()
    }
}
fn caps(axis: &str, values: &[&str]) -> CapabilitySet {
    values.iter().fold(CapabilitySet::new(), |c, v| c.with(axis, *v))
}
fn val(cfg: &Cfg, axis: &str) -> String {
    cfg.combo.get(&AxisId::new(axis)).unwrap().to_string()
}
fn entry<'a>(r: &'a CalibrationReport, axis: &str, value: &str) -> &'a Entry {
    r.entries.iter().find(|e| val(&e.cfg, axis) == value).unwrap()
}
fn measured(e: &Entry) -> &MeasureResult {
    match &e.outcome {
        Outcome::Measured(r) => r,
        other => panic!("expected a measurement, got {other:?}"),
    }
}
fn ms(d: Duration) -> f64 {
    d.as_secs_f64() * 1e3
}
fn single(
    seed: u64,
    combo: FakeCombo,
    cfg: CalibratorConfig,
) -> (CalibrationReport, Calibrator<FakeMeasure>) {
    let fake = FakeMeasure::new(seed, move |_| combo.clone());
    let mut cal = Calibrator::new(cfg, fake);
    let report = cal.run(&target(), &caps("operands", &["a"])).unwrap();
    (report, cal)
}
#[test]
fn low_noise_converges_at_the_minimum_repetitions() {
    for seed in SEEDS {
        let (r, _) = single(seed, FakeCombo::new(10.0, 0.0), quick());
        let m = measured(&r.entries[0]);
        assert!(m.stable && m.samples == 3, "seed {seed}");
        assert!((ms(m.time) - 10.0).abs() < 0.2);
        assert_eq!(r.unstable(), 0);
    }
}
#[test]
fn outliers_are_absorbed_by_median_and_trimmed_spread() {
    for seed in SEEDS {
        let noise = NoiseModel::Outlier {
            every_n: 3,
            magnitude: 40.0,
        };
        let (r, _) = single(seed, FakeCombo::new(10.0, 0.0).noise(noise), quick());
        let m = measured(&r.entries[0]);
        assert!(m.stable, "seed {seed}");
        assert_eq!(m.samples, 5);
        assert!((ms(m.time) - 10.0).abs() < 0.2);
    }
}
#[test]
fn minimum_estimator_takes_the_fastest_sample() {
    let noise = NoiseModel::Outlier {
        every_n: 3,
        magnitude: 40.0,
    };
    let cfg = CalibratorConfig {
        estimator: Estimator::Min,
        ..quick()
    };
    let (r, _) = single(3, FakeCombo::new(10.0, 0.0).noise(noise), cfg);
    assert!(ms(measured(& r.entries[0]).time) <= 10.01);
}
#[test]
fn drifting_series_ends_unstable_with_the_best_intermediate_state() {
    for seed in SEEDS {
        let noise = NoiseModel::Drift {
            per_sample: 3.0,
        };
        let (r, _) = single(seed, FakeCombo::new(10.0, 0.0).noise(noise), quick());
        let m = measured(&r.entries[0]);
        assert!(! m.stable, "seed {seed}");
        assert_eq!(m.samples, 12);
        assert_eq!(r.unstable(), 1);
        assert_eq!(r.front.len(), 1);
        assert!(! r.front[0].stable);
    }
}
#[test]
fn bimodal_series_does_not_settle() {
    for seed in SEEDS {
        let noise = NoiseModel::Bimodal {
            offset: 30.0,
            p: 0.5,
        };
        let cfg = CalibratorConfig {
            min_reps: 8,
            ..quick()
        };
        let (r, _) = single(seed, FakeCombo::new(10.0, 0.0).noise(noise), cfg);
        let m = measured(&r.entries[0]);
        assert!(! m.stable, "seed {seed}");
        assert!((9.9..= 40.1).contains(& ms(m.time)));
    }
}
#[test]
fn never_stable_series_stops_at_the_repetition_cap() {
    for seed in SEEDS {
        let noise = NoiseModel::Erratic {
            amplitude: 8.0,
        };
        let (r, _) = single(seed, FakeCombo::new(10.0, 0.0).noise(noise), quick());
        let m = measured(&r.entries[0]);
        assert!(! m.stable && m.samples == 12, "seed {seed}");
    }
}
#[test]
fn time_cap_ends_a_measurement_without_error() {
    let clock = Arc::new(ManualClock::new());
    let noise = NoiseModel::Drift {
        per_sample: 3.0,
    };
    let fake = FakeMeasure::new(
            1,
            move |_| FakeCombo::new(10.0, 0.0).noise(noise.clone()),
        )
        .with_clock(clock.clone());
    let cfg = CalibratorConfig {
        hard_cap: HardCap {
            max_reps: 1000,
            max_time: Some(Duration::from_millis(35)),
        },
        ..quick()
    };
    let mut cal = Calibrator::new(cfg, fake).with_clock(clock);
    let r = cal.run(&target(), &caps("operands", &["a"])).unwrap();
    let m = measured(&r.entries[0]);
    assert!(! m.stable);
    assert_eq!(m.samples, 3);
}
#[test]
fn exhausted_total_budget_keeps_partial_results() {
    let clock = Arc::new(ManualClock::new());
    let fake = FakeMeasure::new(
            1,
            |c| {
                FakeCombo::new(10.0, if val(c, "operands") == "a" { 1e-2 } else { 0.0 })
            },
        )
        .with_clock(clock.clone());
    let cfg = CalibratorConfig {
        total_budget: Some(Duration::from_millis(25)),
        ..quick()
    };
    let mut cal = Calibrator::new(cfg, fake).with_clock(clock);
    let r = cal.run(&target(), &caps("operands", &["a", "b"])).unwrap();
    assert!(r.budget_exhausted);
    for e in &r.entries {
        let m = measured(e);
        assert!(! m.stable && m.samples == 2);
    }
}
#[test]
fn invalid_combinations_are_rejected_without_timing_them() {
    let fake = FakeMeasure::new(
        5,
        |c| match val(c, "operands").as_str() {
            "ok" => FakeCombo::new(10.0, 0.0),
            "nan" => FakeCombo::new(10.0, 0.0).valid(Validity::NonFinite),
            "nan_dev" => FakeCombo::new(10.0, f64::NAN),
            "flat" => FakeCombo::new(10.0, 0.0).valid(Validity::Degenerate),
            "wide" => FakeCombo::new(5.0, 0.2),
            _ => FakeCombo::new(10.0, 0.0).incompatible(),
        },
    );
    let mut cal = Calibrator::new(quick(), fake);
    let all = ["ok", "nan", "nan_dev", "flat", "wide", "boom"];
    let r = cal.run(&target(), &caps("operands", &all)).unwrap();
    let want = [
        ("nan", Validity::NonFinite),
        ("nan_dev", Validity::NonFinite),
        ("flat", Validity::Degenerate),
        ("wide", Validity::OverMaxDeviation),
        ("boom", Validity::Failed),
    ];
    for (name, validity) in want {
        let e = entry(&r, "operands", name);
        assert_eq!(e.outcome, Outcome::Rejected(validity), "{name}");
        assert_eq!(cal.measure().samples_taken(& e.cfg), 0, "{name}");
    }
    assert_eq!(r.rejected(), 5);
    assert_eq!(r.front.len(), 1);
    assert_eq!(val(& r.front[0].cfg, "operands"), "ok");
}
#[test]
fn front_keeps_only_non_dominated_points_in_order() {
    let fake = FakeMeasure::new(
        9,
        |c| match val(c, "operands").as_str() {
            "slow" => FakeCombo::new(100.0, 0.0),
            "mid" => FakeCombo::new(50.0, 1e-3),
            "fast" => FakeCombo::new(20.0, 3e-2),
            _ => FakeCombo::new(120.0, 4e-2),
        },
    );
    let mut cal = Calibrator::new(quick(), fake);
    let r = cal
        .run(&target(), &caps("operands", &["slow", "mid", "fast", "bad"]))
        .unwrap();
    let names: Vec<String> = r.front.iter().map(|p| val(&p.cfg, "operands")).collect();
    assert_eq!(names, ["fast", "mid", "slow"]);
    for w in r.front.windows(2) {
        assert!(w[0].time < w[1].time && w[0].deviation > w[1].deviation);
    }
}
#[test]
fn dominated_combination_stops_at_the_minimum_repetitions() {
    let fake = FakeMeasure::new(
        2,
        |c| match val(c, "operands").as_str() {
            "a_fast" => FakeCombo::new(10.0, 0.0),
            _ => {
                FakeCombo::new(120.0, 0.0)
                    .noise(NoiseModel::Erratic {
                        amplitude: 8.0,
                    })
            }
        },
    );
    let mut cal = Calibrator::new(quick(), fake);
    let r = cal.run(&target(), &caps("operands", &["a_fast", "b_slow"])).unwrap();
    let slow = entry(&r, "operands", "b_slow");
    assert_eq!(slow.outcome, Outcome::Dominated);
    assert_eq!(cal.measure().samples_taken(& slow.cfg), 3);
    assert!(measured(entry(& r, "operands", "a_fast")).stable);
    assert_eq!(r.dominated(), 1);
}
#[test]
fn combinations_are_measured_interleaved_in_abba_order() {
    let fake = FakeMeasure::new(
        4,
        |c| match val(c, "operands").as_str() {
            "a" => FakeCombo::new(10.0, 3e-2),
            "b" => FakeCombo::new(20.0, 2e-2),
            _ => FakeCombo::new(30.0, 1e-2),
        },
    );
    let mut cal = Calibrator::new(quick(), fake);
    cal.run(&target(), &caps("operands", &["a", "b", "c"])).unwrap();
    let order: Vec<String> = cal
        .measure()
        .order()
        .iter()
        .map(|c| val(c, "operands"))
        .collect();
    assert_eq!(order.join(""), "abccbaabc");
}
#[test]
fn stability_is_judged_on_ratios_not_on_absolute_times() {
    let script = |c: &Cfg| match val(c, "operands").as_str() {
        "a" => FakeCombo::new(10.0, 1e-2),
        _ => FakeCombo::new(20.0, 0.0),
    };
    let load = |round: usize| 1.0 + 0.4 * (round % 2) as f64;
    let shared = FakeMeasure::new(1, script).with_shared_load(2, load);
    let mut cal = Calibrator::new(quick(), shared);
    let r = cal.run(&target(), &caps("operands", &["a", "b"])).unwrap();
    for e in &r.entries {
        let m = measured(e);
        assert!(m.stable && m.samples == 3);
    }
    let alone = FakeMeasure::new(1, script).with_shared_load(2, load);
    let mut cal = Calibrator::new(quick(), alone);
    let r = cal.run(&target(), &caps("operands", &["a"])).unwrap();
    assert!(! measured(& r.entries[0]).stable);
}
fn separable_model(c: &Cfg) -> FakeCombo {
    let op = match val(c, "operands").as_str() {
        "fp32" => (100.0, 0.0),
        _ => (60.0, 1e-3),
    };
    let batch = match val(c, "batch").as_str() {
        "1" => 3.0,
        "8" => 1.5,
        _ => 1.0,
    };
    let tile = match val(c, "tile").as_str() {
        "16" => 1.4,
        "64" => 1.0,
        _ => 1.2,
    };
    FakeCombo::new(op.0 * batch * tile, op.1)
}
fn separable_caps() -> CapabilitySet {
    let mut c = caps("operands", &["fp32", "fp16"]);
    for b in ["1", "8", "32"] {
        c.insert("batch", b);
    }
    for t in ["16", "64", "256"] {
        c.insert("tile", t);
    }
    c
}
#[test]
fn pruned_calibration_matches_full_enumeration_with_fewer_samples() {
    for seed in SEEDS {
        let mut pruned = Calibrator::new(
            quick(),
            FakeMeasure::new(seed, separable_model),
        );
        let rp = pruned.run(&target(), &separable_caps()).unwrap();
        let full_cfg = CalibratorConfig {
            exhaustive: true,
            ..quick()
        };
        let mut full = Calibrator::new(
            full_cfg,
            FakeMeasure::new(seed, separable_model),
        );
        let rf = full.run(&target(), &separable_caps()).unwrap();
        let key = |r: &CalibrationReport| {
            let mut v: Vec<String> = r.front.iter().map(|p| p.cfg.to_string()).collect();
            v.sort();
            v
        };
        assert_eq!(key(& rp), key(& rf), "seed {seed}");
        assert_eq!(rf.front.len(), 2);
        assert_eq!(rp.exhaustive_size, 18);
        assert!(
            pruned.measure().order().len() < full.measure().order().len(), "seed {seed}"
        );
    }
}
#[test]
fn axis_winners_are_chosen_per_leading_axis_value() {
    let model = |c: &Cfg| {
        let slow = (val(c, "operands") == "fp16") == (val(c, "batch") == "32");
        let fp16 = val(c, "operands") == "fp16";
        let fast = if fp16 { 30.0 } else { 40.0 };
        FakeCombo::new(if slow { 90.0 } else { fast }, if fp16 { 1e-3 } else { 0.0 })
    };
    let caps = {
        let mut c = caps("operands", &["fp32", "fp16"]);
        c.insert("batch", "1");
        c.insert("batch", "32");
        c
    };
    let mut cal = Calibrator::new(quick(), FakeMeasure::new(3, model));
    let r = cal.run(&target(), &caps).unwrap();
    let best = |dtype: &str| {
        r.front
            .iter()
            .find(|p| val(&p.cfg, "operands") == dtype)
            .map(|p| val(&p.cfg, "batch"))
    };
    assert_eq!(best("fp32").as_deref(), Some("32"));
    assert_eq!(best("fp16").as_deref(), Some("1"));
}
#[test]
fn waiting_task_delays_sampling_without_failing_the_run() {
    struct Countdown(AtomicU32);
    impl YieldCheck for Countdown {
        fn should_yield(&self) -> bool {
            self.0
                .try_update(Ordering::SeqCst, Ordering::SeqCst, |n| n.checked_sub(1))
                .is_ok()
        }
    }
    let fake = FakeMeasure::new(1, |_| FakeCombo::new(10.0, 0.0));
    let mut cal = Calibrator::new(quick(), fake)
        .with_yield_check(Arc::new(Countdown(AtomicU32::new(2))));
    let r = cal.run(&target(), &caps("operands", &["a"])).unwrap();
    let m = measured(&r.entries[0]);
    assert!(m.stable && m.samples == 3);
}
#[test]
fn permanent_preemption_pressure_skips_instead_of_blocking() {
    let fake = FakeMeasure::new(1, |_| FakeCombo::new(10.0, 0.0));
    let flag = Arc::new(AtomicBool::new(true));
    let mut cal = Calibrator::new(quick(), fake).with_yield_check(flag);
    let r = cal.run(&target(), &caps("operands", &["a"])).unwrap();
    assert_eq!(r.entries[0].outcome, Outcome::Skipped);
    assert_eq!(cal.measure().order().len(), 0);
    assert!(r.front.is_empty());
}
#[test]
fn empty_capability_set_is_an_error() {
    let fake = FakeMeasure::new(1, |_| FakeCombo::new(10.0, 0.0));
    let mut cal = Calibrator::new(quick(), fake);
    let err = cal.run(&target(), &CapabilitySet::new()).unwrap_err();
    assert!(matches!(err, PrecisionError::Capability { .. }));
}
#[test]
fn minimum_repetitions_never_drop_below_three() {
    let cfg = CalibratorConfig {
        min_reps: 1,
        ..quick()
    };
    let cal = Calibrator::new(cfg, FakeMeasure::new(1, |_| FakeCombo::new(1.0, 0.0)));
    assert_eq!(cal.config().min_reps, 3);
}
#[test]
fn test_set_lengths_reach_the_maximum_sequence_length() {
    assert_eq!(CalibratorConfig::seq_lens_up_to(512), [16, 64, 256, 512]);
    assert_eq!(CalibratorConfig::seq_lens_up_to(100), [16, 64, 100]);
    assert_eq!(CalibratorConfig::seq_lens_up_to(16), [16]);
    let (r, _) = single(1, FakeCombo::new(10.0, 0.0), quick());
    assert_eq!(r.entries[0].cfg.seq_lens.last(), Some(& 512));
}
#[test]
fn default_separable_axes_are_the_numeric_configuration_axes() {
    let d = CalibratorConfig::default();
    for axis in ["batch", "tile", "threads"] {
        assert!(d.separable_axes.contains(& AxisId::new(axis)));
    }
    assert!(! d.separable_axes.contains(& AxisId::new("operands")));
}
