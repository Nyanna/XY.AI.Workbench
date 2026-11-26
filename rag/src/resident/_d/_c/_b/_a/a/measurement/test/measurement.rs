use super::*;
use std::sync::atomic::{AtomicUsize, Ordering};
use super::bench_workload::WorkloadConfig;
#[test]
fn types_are_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<Combination>();
    assert_send_sync::<MeasurementResult>();
    assert_send_sync::<MeasureError>();
    assert_send_sync::<Hooks>();
    assert_send_sync::<PendingFlag>();
}
fn small_workload() -> Workload {
    Workload::build(WorkloadConfig {
            seed: 3,
            docs: 40,
            dim: 8,
            max_queries: 6,
            top_k: 3,
        })
        .unwrap()
}
fn no_precision(_: &Combination) -> Result<(), String> {
    Ok(())
}
#[test]
fn defaults_follow_the_constants() {
    let cfg = MeasureConfig::new(0.2);
    assert_eq!(cfg.window_ns, DEFAULT_WINDOW_NS);
    assert_eq!(cfg.chunk_budget_ns, DEFAULT_CHUNK_BUDGET_NS);
    assert_eq!(cfg.max_deviation, 0.2);
    assert_eq!(cfg.top_k, 5);
    assert_eq!(DEFAULT_WINDOW_NS, 10_000_000);
    assert_eq!(DEFAULT_CHUNK_BUDGET_NS, 5_000_000);
    assert_eq!(CPU_DOCS, 4_000);
    assert_eq!(GPU_FLUSH_BYTES, 64 * 1024 * 1024);
    assert_eq!(GPU_WARMUP_NS, 100_000_000);
    assert_eq!((MIN_REPS, MAX_REPS), (1, 10_000));
    assert_eq!(STABLE_MAX_SPREAD, 0.05);
}
#[test]
fn timing_and_deviation_configuration_is_validated() {
    assert!(MeasureConfig::new(0.1).validate_timing().is_ok());
    let mut cfg = MeasureConfig::new(0.1);
    cfg.window_ns = 0;
    assert!(matches!(cfg.validate_timing(), Err(MeasureError::Config(_))));
    let mut cfg = MeasureConfig::new(0.1);
    cfg.chunk_budget_ns = 0;
    assert!(matches!(cfg.validate_timing(), Err(MeasureError::Config(_))));
    assert!(MeasureConfig::new(0.1).deviation_config().is_ok());
    for bad in [f64::NAN, -0.5] {
        assert!(
            matches!(MeasureConfig::new(bad).deviation_config(),
            Err(MeasureError::Bench(BenchError::InvalidConfig { .. })))
        );
    }
    let mut cfg = MeasureConfig::new(0.1);
    cfg.top_k = 0;
    assert!(matches!(cfg.deviation_config(), Err(MeasureError::Bench(_))));
}
#[test]
fn errors_convert_and_display() {
    let bench = BenchError::TooManyQueries {
        requested: 9,
        max: 4,
    };
    assert_eq!(MeasureError::from(bench.clone()), MeasureError::Bench(bench.clone()));
    let gpu = GpuKernelError::InvalidParams("x".to_string());
    assert_eq!(MeasureError::from(gpu.clone()), MeasureError::Gpu(gpu));
    let all = [
        MeasureError::Capability("c".into()),
        MeasureError::Config("c".into()),
        MeasureError::NoSamples,
        MeasureError::Bench(bench),
        MeasureError::Gpu(GpuKernelError::InvalidParams("x".into())),
        MeasureError::Rig("r".into()),
        MeasureError::Compute("c".into()),
    ];
    for e in &all {
        assert!(! e.to_string().is_empty());
        assert!(! e.cause_text().is_empty());
    }
    assert_eq!(MeasureError::Rig("device lost".into()).cause_text(), "device lost");
}
#[test]
fn preflight_reports_the_first_failure() {
    let ok = Combination::new().with(axes::batch_size(), "3").unwrap();
    assert_eq!(preflight(& ok, 6, no_precision), Ok((3, ())));
    assert_eq!(preflight(& Combination::new(), 6, no_precision), Ok((6, ())));
    assert_eq!(preflight(& Combination::new(), 4, no_precision), Ok((4, ())));
    let both = Combination::new()
        .with(axes::batch_size(), "0")
        .unwrap()
        .with_seq_lens(vec![2, 0]);
    let text = preflight(&both, 6, |_| Err::<(), _>("precision".to_string()))
        .unwrap_err();
    assert!(text.contains("sequence length"));
    let bad_batch = Combination::new().with(axes::batch_size(), "0").unwrap();
    let text = preflight(&bad_batch, 6, |_| Err::<(), _>("precision".to_string()))
        .unwrap_err();
    assert!(text.contains("batch size"));
    let too_large = Combination::new().with(axes::batch_size(), "7").unwrap();
    assert!(preflight(& too_large, 6, no_precision).is_err());
    let text = preflight(&ok, 6, |_| Err::<(), _>("precision".to_string())).unwrap_err();
    assert_eq!(text, "precision");
}
#[test]
fn preflight_ignores_other_axes() {
    let c = Combination::new()
        .with(axes::tile_size(), "zzz")
        .unwrap()
        .with(axes::batch_size(), "2")
        .unwrap();
    assert_eq!(preflight(& c, 6, no_precision), Ok((2, ())));
}
#[test]
fn incompatible_outcomes_always_carry_text() {
    for text in ["why".to_string(), String::new()] {
        match incompatible6(text) {
            DeviationOutcome::Incompatible { validity } => {
                assert_eq!(validity.kind, ValidityKind::Degenerate);
                assert!(! validity.text.is_empty());
            }
            other => panic!("unexpected {other:?}"),
        }
    }
}
#[test]
fn judge_maps_every_validity() {
    let workload = small_workload();
    let cache = ReferenceCache::default();
    let cfg = DeviationConfig::new(1e-3).unwrap();
    let reference = workload.reference_scores(2).unwrap();
    match judge3(&workload, &cache, &cfg, 2, Ok(&reference)).unwrap() {
        DeviationOutcome::Measured { deviation, validity } => {
            assert_eq!(deviation, 0.0);
            assert_eq!(validity.kind, ValidityKind::Valid);
        }
        other => panic!("unexpected {other:?}"),
    }
    let kind_of = |candidate: Result<&[f32], String>| match judge3(
            &workload,
            &cache,
            &cfg,
            2,
            candidate,
        )
        .unwrap()
    {
        DeviationOutcome::Incompatible { validity } => {
            assert!(! validity.text.is_empty());
            validity.kind
        }
        other => panic!("unexpected {other:?}"),
    };
    let mut reversed = reference.clone();
    reversed.reverse();
    assert_eq!(kind_of(Ok(& reversed)), ValidityKind::ExceedsMax);
    let mut nan = reference.clone();
    nan[3] = f32::NAN;
    assert_eq!(kind_of(Ok(& nan)), ValidityKind::NonFinite);
    assert_eq!(kind_of(Ok(& reference[..5])), ValidityKind::Degenerate);
    assert_eq!(kind_of(Err("launch failed".to_string())), ValidityKind::ExecutionFailed);
}
#[test]
fn judge_matches_workload_evaluate() {
    let workload = small_workload();
    let cache = ReferenceCache::default();
    let cfg = DeviationConfig::new(1e-3).unwrap();
    let mut candidate = workload.reference_scores(3).unwrap();
    for (i, s) in candidate.iter_mut().enumerate() {
        *s += (i % 7) as f32 * 1e-3;
    }
    let assessment = workload.evaluate(3, Ok(&candidate), &cfg).unwrap();
    assert_eq!(
        judge3(& workload, & cache, & cfg, 3, Ok(& candidate)).unwrap(),
        outcome_of(assessment)
    );
}
#[test]
fn reference_cache_computes_once_per_query_count() {
    let workload = small_workload();
    let cache = ReferenceCache::default();
    let first = cache.get(&workload, 2).unwrap();
    assert!(Arc::ptr_eq(& first, & cache.get(& workload, 2).unwrap()));
    assert_eq!(* first, workload.reference_scores(2).unwrap());
    assert!(! Arc::ptr_eq(& first, & cache.get(& workload, 3).unwrap()));
    assert!(matches!(cache.get(& workload, 7), Err(BenchError::TooManyQueries { .. })));
}
#[test]
fn samples_are_normalized_and_floored() {
    assert_eq!(sample_ns(1000.0, 5, 2), 100.0);
    assert_eq!(sample_ns(0.0, 5, 2), MIN_SAMPLE_NS);
    assert_eq!(sample_ns(f64::NAN, 5, 2), MIN_SAMPLE_NS);
}
#[test]
fn gate_admits_one_caller_at_a_time() {
    let gate = Arc::new(Gate3::new());
    let inside = Arc::new(AtomicUsize::new(0));
    let handles: Vec<_> = (0..6)
        .map(|_| {
            let gate = Arc::clone(&gate);
            let inside = Arc::clone(&inside);
            std::thread::spawn(move || {
                for _ in 0..5 {
                    let _turn = gate.enter();
                    assert_eq!(inside.fetch_add(1, Ordering::SeqCst), 0);
                    std::thread::yield_now();
                    inside.fetch_sub(1, Ordering::SeqCst);
                }
            })
        })
        .collect();
    for h in handles {
        h.join().unwrap();
    }
}
#[test]
fn gate_is_released_by_a_panicking_holder() {
    let gate = Arc::new(Gate3::new());
    let other = Arc::clone(&gate);
    let result = std::thread::spawn(move || {
            let _turn = other.enter();
            panic!("holder failed");
        })
        .join();
    assert!(result.is_err());
    let _turn = gate.enter();
}
