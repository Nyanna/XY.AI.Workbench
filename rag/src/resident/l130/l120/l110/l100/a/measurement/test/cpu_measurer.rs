use super::*;
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use super::super::bench_workload::{BenchError, ValidityKind};
use super::super::{Clock, PendingTasks};
/// Advances by a fixed step on every call.
struct StepClock {
    now: AtomicU64,
    step: u64,
}
impl StepClock {
    fn new(step: u64) -> Arc<Self> {
        Arc::new(StepClock {
            now: AtomicU64::new(0),
            step,
        })
    }
}
impl Clock for StepClock {
    fn now_ns(&self) -> u64 {
        self.now.fetch_add(self.step, Ordering::SeqCst)
    }
}
/// Reports `true` from the given call on (1-based).
struct PendingFrom {
    calls: AtomicUsize,
    from: usize,
}
impl PendingFrom {
    fn new(from: usize) -> Arc<Self> {
        Arc::new(PendingFrom {
            calls: AtomicUsize::new(0),
            from,
        })
    }
    fn calls(&self) -> usize {
        self.calls.load(Ordering::SeqCst)
    }
}
impl PendingTasks for PendingFrom {
    fn productive_waiting(&self) -> bool {
        self.calls.fetch_add(1, Ordering::SeqCst) + 1 >= self.from
    }
}
fn measurer(
    threads: usize,
    clock: Arc<dyn Clock>,
    pending: Arc<dyn PendingTasks>,
) -> CpuMeasurer {
    let mut cfg = CpuConfig::new(1e-3);
    cfg.threads = threads;
    CpuMeasurer::new(cfg, Hooks { clock, pending }).unwrap()
}
fn plain(threads: usize) -> CpuMeasurer {
    CpuMeasurer::new(
            {
                let mut c = CpuConfig::new(1e-3);
                c.threads = threads;
                c
            },
            Hooks::default(),
        )
        .unwrap()
}
fn combo(batch: &str) -> Combination {
    Combination::new().with(axes::batch_size(), batch).unwrap()
}
#[test]
fn construction_checks_the_configuration() {
    let mut cfg = CpuConfig::new(0.1);
    cfg.threads = 0;
    assert!(
        matches!(CpuMeasurer::new(cfg, Hooks::default()), Err(MeasureError::Config(_)))
    );
    let mut cfg = CpuConfig::new(0.1);
    cfg.common.window_ns = 0;
    assert!(
        matches!(CpuMeasurer::new(cfg, Hooks::default()), Err(MeasureError::Config(_)))
    );
    let mut cfg = CpuConfig::new(0.1);
    cfg.common.chunk_budget_ns = 0;
    assert!(
        matches!(CpuMeasurer::new(cfg, Hooks::default()), Err(MeasureError::Config(_)))
    );
    assert_eq!(plain(1).workload.config().docs, CPU_DOCS);
}
#[test]
fn split_covers_the_documents_exactly() {
    for (docs, parts) in [(10, 1), (10, 3), (10, 10), (10, 16), (4000, 7), (1, 4)] {
        let ranges = split_ranges(docs, parts);
        assert!(ranges.len() <= parts.max(1));
        let mut next = 0;
        for r in &ranges {
            assert_eq!(r.start, next);
            assert!(r.end > r.start);
            next = r.end;
        }
        assert_eq!(next, docs);
    }
}
#[test]
fn scan_matches_the_serial_reference_for_any_thread_count() {
    let serial = plain(1);
    let reference = serial.workload.reference_scores(3).unwrap();
    assert_eq!(serial.scan(3).unwrap(), reference);
    for threads in [2, 3, 7] {
        let m = plain(threads);
        assert_eq!(m.scan(3).unwrap(), reference, "threads = {threads}");
    }
}
#[test]
fn deviation_of_fp32_is_measured_and_zero() {
    let m = plain(2);
    let first = m.deviation(&combo("4")).unwrap();
    match &first {
        DeviationOutcome::Measured { deviation, validity } => {
            assert_eq!(* deviation, 0.0);
            assert_eq!(validity.kind, ValidityKind::Valid);
        }
        other => panic!("unexpected {other:?}"),
    }
    assert_eq!(m.deviation(& combo("4")).unwrap(), first);
    let explicit = combo("4")
        .with(axes::scoring(), values::fp32())
        .unwrap()
        .with(axes::operands(), values::fp32())
        .unwrap();
    assert!(
        matches!(m.deviation(& explicit).unwrap(), DeviationOutcome::Measured { .. })
    );
}
#[test]
fn invalid_config_is_an_error_not_an_outcome() {
    let m = CpuMeasurer::new(CpuConfig::new(f64::NAN), Hooks::default()).unwrap();
    assert!(
        matches!(m.deviation(& combo("2")),
        Err(MeasureError::Bench(BenchError::InvalidConfig { .. })))
    );
}
#[test]
fn unsupported_combinations_are_incompatible() {
    let m = plain(1);
    let cases = [
        combo("2").with(axes::scoring(), values::fp16()).unwrap(),
        combo("2").with(axes::operands(), values::int8()).unwrap(),
        combo("0"),
        combo("33"),
        combo("abc"),
        combo("2").with_seq_lens(vec![4, 0]),
    ];
    for c in &cases {
        match m.deviation(c).unwrap() {
            DeviationOutcome::Incompatible { validity } => {
                assert_eq!(validity.kind, ValidityKind::Degenerate);
                assert!(! validity.text.is_empty());
            }
            other => panic!("unexpected {other:?} for {c}"),
        }
        assert!(matches!(m.time_probe(c), Err(MeasureError::Config(_))), "{c}");
    }
    match m
        .deviation(&combo("2").with(axes::scoring(), values::fp16()).unwrap())
        .unwrap()
    {
        DeviationOutcome::Incompatible { validity } => {
            assert!(validity.text.contains("scoring"))
        }
        other => panic!("unexpected {other:?}"),
    }
}
#[test]
fn probe_scales_to_the_window() {
    let m = measurer(1, StepClock::new(1_000_000), Arc::new(super::super::NeverPending));
    let probe = m.time_probe(&combo("4")).unwrap();
    assert_eq!(probe, TimeProbe::Time(1_000_000.0 * 10.0 / (10.0 * 4.0)));
}
#[test]
fn probe_without_batch_axis_uses_the_default_query_count() {
    let m = measurer(1, StepClock::new(1_000_000), Arc::new(super::super::NeverPending));
    let probe = m.time_probe(&Combination::new()).unwrap();
    assert_eq!(probe, TimeProbe::Time(1_000_000.0 / 8.0));
}
#[test]
fn long_passes_run_once_and_are_never_checked_between() {
    let pending = PendingFrom::new(usize::MAX);
    let m = measurer(1, StepClock::new(1_000_000_000), pending.clone());
    let probe = m.time_probe(&combo("2")).unwrap();
    assert_eq!(probe, TimeProbe::Time(1_000_000_000.0 / 2.0));
    assert_eq!(pending.calls(), 1);
}
#[test]
fn interruption_is_checked_before_and_between_passes() {
    let pending = PendingFrom::new(usize::MAX);
    let m = measurer(1, StepClock::new(1_000_000), pending.clone());
    assert!(matches!(m.time_probe(& combo("2")).unwrap(), TimeProbe::Time(_)));
    assert_eq!(pending.calls(), 10);
    let pending = PendingFrom::new(1);
    let m = measurer(1, StepClock::new(1_000_000), pending.clone());
    assert_eq!(m.time_probe(& combo("2")).unwrap(), TimeProbe::Interrupted);
    assert_eq!(pending.calls(), 1);
    let pending = PendingFrom::new(4);
    let m = measurer(1, StepClock::new(1_000_000), pending.clone());
    assert_eq!(m.time_probe(& combo("2")).unwrap(), TimeProbe::Interrupted);
    assert_eq!(pending.calls(), 4);
}
#[test]
fn flag_interrupts_and_a_later_probe_succeeds() {
    let flag = super::super::PendingFlag::new();
    let m = measurer(1, StepClock::new(1_000_000), Arc::new(flag.clone()));
    flag.set(true);
    assert_eq!(m.time_probe(& combo("2")).unwrap(), TimeProbe::Interrupted);
    flag.set(false);
    assert!(matches!(m.time_probe(& combo("2")).unwrap(), TimeProbe::Time(_)));
}
#[test]
fn warm_up_runs_once_per_combination_and_query_count() {
    let m = plain(1);
    assert!(lock(& m.warmed).is_empty());
    m.deviation(&combo("2")).unwrap();
    assert_eq!(lock(& m.warmed).len(), 1);
    m.deviation(&combo("2")).unwrap();
    m.time_probe(&combo("2")).unwrap();
    assert_eq!(lock(& m.warmed).len(), 1);
    m.time_probe(&combo("3")).unwrap();
    assert_eq!(lock(& m.warmed).len(), 2);
    let flag = super::super::PendingFlag::new();
    flag.set(true);
    let m = measurer(1, StepClock::new(1_000_000), Arc::new(flag));
    assert_eq!(m.time_probe(& combo("2")).unwrap(), TimeProbe::Interrupted);
    assert_eq!(lock(& m.warmed).len(), 1);
}
#[test]
fn parallel_callers_are_serialized() {
    let m = Arc::new(plain(2));
    let handles: Vec<_> = (0..4)
        .map(|_| {
            let m = Arc::clone(&m);
            std::thread::spawn(move || m.deviation(&combo("2")).unwrap())
        })
        .collect();
    for h in handles {
        assert!(matches!(h.join().unwrap(), DeviationOutcome::Measured { .. }));
    }
}
