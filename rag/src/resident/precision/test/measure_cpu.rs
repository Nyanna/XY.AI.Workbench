use super::*;
use crate::resident::precision::known::{BATCH, FP16};
use crate::resident::precision::measure::{NeverYield, Validity};
use crate::resident::precision::profile::ProfileConfig;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;

fn spec() -> WorkloadSpec {
    WorkloadSpec { docs: 300, dim: 32, max_queries: 8, ..WorkloadSpec::default() }
}

fn measure() -> CpuScoringMeasure {
    CpuScoringMeasure::new(spec(), WindowTuning::default(), Arc::new(NeverYield)).unwrap()
}

fn cfg(threads: &str, batch: &str) -> Cfg {
    Cfg::new(
        ProfileConfig::new()
            .with(&*OPERANDS, &*FP32)
            .with(&*THREADS, threads)
            .with(&*BATCH, batch),
        vec![],
    )
}

#[test]
fn fp32_scoring_matches_the_reference_for_any_thread_count() {
    let mut m = measure();
    for threads in ["1", "2", "3"] {
        let r = m.deviation(&cfg(threads, "4")).unwrap();
        assert_eq!(r.valid, Validity::Ok, "threads={threads}");
        assert!(r.deviation < 1e-6, "threads={threads}");
    }
}

#[test]
fn samples_are_positive_times() {
    let mut m = measure();
    match m.sample(&cfg("2", "4")).unwrap() {
        Sample::Time(t) => assert!(t > Duration::ZERO),
        Sample::Interrupted => panic!("unexpected interrupt"),
    }
}

#[test]
fn non_fp32_types_are_incompatible() {
    let mut m = measure();
    let c = Cfg::new(ProfileConfig::new().with(&*OPERANDS, &*FP16), vec![]);
    assert!(matches!(m.deviation(&c), Err(PrecisionError::Incompatible { .. })));
    assert!(matches!(m.sample(&c), Err(PrecisionError::Incompatible { .. })));
}

#[test]
fn invalid_axis_values_are_incompatible() {
    let mut m = measure();
    assert!(m.deviation(&cfg("0", "4")).is_err());
    assert!(m.deviation(&cfg("x", "4")).is_err());
    assert!(m.deviation(&cfg("1", "99")).is_err());
}

#[test]
fn a_waiting_task_interrupts_a_long_window() {
    let flag = Arc::new(AtomicBool::new(true));
    let tuning = WindowTuning { window_ms: 10_000.0, slice_budget_ms: 5.0 };
    let mut m = CpuScoringMeasure::new(spec(), tuning, flag.clone()).unwrap();
    assert_eq!(m.sample(&cfg("1", "2")).unwrap(), Sample::Interrupted);
    flag.store(false, Ordering::Relaxed);
}
