use super::*;
use crate::resident::precision::calibrate::{CalibrationTarget, Calibrator, CalibratorConfig, Outcome};
use crate::resident::precision::known::BATCH;
use crate::resident::precision::measure::{NeverYield, Validity};
use crate::resident::precision::profile::ProfileConfig;
use crate::resident::precision::CapabilitySet;
use crate::resident::subengine::SubengineKind;

fn measure(spec: WorkloadSpec, tuning: WindowTuning) -> CudaScoringMeasure {
    let dev = DeviceHandle::cuda(0).expect("cuda device");
    CudaScoringMeasure::new(&dev, spec, tuning, Arc::new(NeverYield)).unwrap()
}

fn small() -> WorkloadSpec {
    WorkloadSpec { docs: 4_000, dim: 64, max_queries: 8, ..WorkloadSpec::default() }
}

fn cfg(dtype: &crate::resident::precision::ValueId, batch: &str) -> Cfg {
    Cfg::new(ProfileConfig::new().with(&*SCORING, dtype).with(&*BATCH, batch), vec![])
}

#[test]
#[ignore = "requires a CUDA device"]
fn fp32_matches_the_reference_and_fp16_stays_close() {
    let mut m = measure(small(), WindowTuning::default());
    let r32 = m.deviation(&cfg(&FP32, "4")).unwrap();
    assert_eq!(r32.valid, Validity::Ok);
    assert!(r32.deviation < 1e-4, "{}", r32.deviation);
    let r16 = m.deviation(&cfg(&FP16, "4")).unwrap();
    assert_eq!(r16.valid, Validity::Ok);
    assert!(r16.deviation < 5e-2, "{}", r16.deviation);
}

#[test]
#[ignore = "requires a CUDA device"]
fn samples_scale_to_the_window_and_stay_positive() {
    let mut m = measure(small(), WindowTuning::default());
    for _ in 0..3 {
        match m.sample(&cfg(&FP32, "4")).unwrap() {
            Sample::Time(t) => assert!(t > Duration::ZERO && t < Duration::from_millis(50)),
            Sample::Interrupted => panic!("unexpected interrupt"),
        }
    }
    let plan = m.plans[&(false, 4)];
    assert!(plan.group >= 1 && plan.group <= plan.iters);
}

#[test]
#[ignore = "requires a CUDA device"]
fn a_waiting_task_interrupts_between_groups() {
    use std::sync::atomic::{AtomicBool, Ordering};
    let flag = Arc::new(AtomicBool::new(false));
    let dev = DeviceHandle::cuda(0).unwrap();
    // Window far above the slice budget forces several groups.
    let tuning = WindowTuning { window_ms: 200.0, slice_budget_ms: 1.0 };
    let mut m = CudaScoringMeasure::new(&dev, small(), tuning, flag.clone()).unwrap();
    assert!(matches!(m.sample(&cfg(&FP32, "4")).unwrap(), Sample::Time(_)));
    flag.store(true, Ordering::Relaxed);
    assert_eq!(m.sample(&cfg(&FP32, "4")).unwrap(), Sample::Interrupted);
}

#[test]
#[ignore = "requires a CUDA device"]
fn unsupported_types_are_incompatible() {
    let mut m = measure(small(), WindowTuning::default());
    let int8 = crate::resident::precision::known::INT8_DP4A.clone();
    assert!(matches!(m.deviation(&cfg(&int8, "4")), Err(PrecisionError::Incompatible { .. })));
}

/// End to end on the device: probe-style capability set, real measurement, Pareto front.
#[test]
#[ignore = "requires a CUDA device; measurement"]
fn gate_calibration_front_on_device() {
    let caps = CapabilitySet::new()
        .with(&*SCORING, &*FP32)
        .with(&*SCORING, &*FP16)
        .with(&*BATCH, "1")
        .with(&*BATCH, "8");
    let spec = WorkloadSpec { docs: 20_000, dim: 384, max_queries: 8, ..WorkloadSpec::default() };
    let cfg = CalibratorConfig::default();
    let m = measure(spec, cfg.tuning());
    let mut cal = Calibrator::new(cfg, m);
    let target = CalibrationTarget { engine: SubengineKind::Cuda, model_id: "gate".into() };
    let report = cal.run(&target, &caps).unwrap();
    for e in &report.entries {
        match &e.outcome {
            Outcome::Measured(r) => println!(
                "G-CALIB {:?} {} time {:?} dev {:.3e} stable {} samples {}",
                e.stage, e.cfg, r.time, r.deviation, r.stable, r.samples
            ),
            other => println!("G-CALIB {:?} {} {other:?}", e.stage, e.cfg),
        }
    }
    for p in &report.front {
        println!("G-CALIB front {} time {:?} dev {:.3e} stable {}", p.cfg, p.time, p.deviation, p.stable);
    }
    assert!(!report.front.is_empty());
    assert_eq!(report.rejected(), 0);
}
