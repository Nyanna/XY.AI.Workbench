use super::*;

fn small_cfg(seed: u64) -> WorkloadConfig {
    WorkloadConfig { seed, docs: 50, dim: 16, max_queries: 6, top_k: 3 }
}

#[test]
fn build_is_deterministic_and_normalized() {
    let a = Workload::build(small_cfg(7)).unwrap();
    let b = Workload::build(small_cfg(7)).unwrap();
    for i in 0..6 {
        assert_eq!(a.queries().data(i), b.queries().data(i));
        let n = norm(a.queries().data(i).unwrap());
        assert!((n - 1.0).abs() < 1e-5);
    }
    assert_eq!(a.reference_scores(6).unwrap(), b.reference_scores(6).unwrap());
    let c = Workload::build(small_cfg(8)).unwrap();
    assert_ne!(a.queries().data(0), c.queries().data(0));
}

#[test]
fn build_rejects_invalid_and_non_unique() {
    let mut cfg = small_cfg(1);
    cfg.docs = 0;
    assert!(matches!(Workload::build(cfg), Err(BenchError::InvalidConfig { .. })));
    let cfg = WorkloadConfig { seed: 1, docs: 5, dim: 1, max_queries: 1, top_k: 1 };
    assert_eq!(Workload::build(cfg).err(), Some(BenchError::CannotBuildUnique { what: "documents" }));
}

#[test]
fn reference_shapes_and_limits() {
    let w = Workload::build(small_cfg(3)).unwrap();
    assert_eq!(w.reference_scores(2).unwrap().len(), 100);
    assert!(w.reference_scores(0).unwrap().is_empty());
    assert_eq!(
        w.reference_scores(7),
        Err(BenchError::TooManyQueries { requested: 7, max: 6 })
    );
    let top = w.reference_top_k(2).unwrap();
    assert_eq!(top.len(), 2);
    assert_eq!(top[0].len(), 3);
    let first = w.reference_scores(1).unwrap();
    let both = w.reference_scores(2).unwrap();
    assert_eq!(&both[..50], &first[..]);
}

#[test]
fn query_count_rules() {
    assert_eq!(query_count_from_value(None, 32), Ok(8));
    assert_eq!(query_count_from_value(None, 4), Ok(4));
    assert_eq!(query_count_from_value(Some("016"), 32), Ok(16));
    assert_eq!(query_count_from_value(Some("0"), 32), Err(BatchError::Zero));
    assert!(matches!(query_count_from_value(Some(""), 32), Err(BatchError::NotNumeric { .. })));
    assert!(matches!(query_count_from_value(Some("+3"), 32), Err(BatchError::NotNumeric { .. })));
    assert!(matches!(query_count_from_value(Some(" 3"), 32), Err(BatchError::NotNumeric { .. })));
    assert!(matches!(query_count_from_value(Some("33"), 32), Err(BatchError::TooLarge { .. })));
    assert!(matches!(
        query_count_from_value(Some("99999999999999999999999"), 32),
        Err(BatchError::TooLarge { .. })
    ));
}

#[test]
fn query_count_from_combo() {
    let empty = CapabilitySet::default();
    assert_eq!(query_count(&empty, 32), Ok(8));
    let one = CapabilitySet::default().with(axes::batch_size(), "4");
    assert_eq!(query_count(&one, 32), Ok(4));
    let two = CapabilitySet::default()
        .with(axes::batch_size(), "4")
        .with(axes::batch_size(), "8");
    assert_eq!(query_count(&two, 32), Err(BatchError::Ambiguous { count: 2 }));
}

#[test]
fn metric_basics() {
    let a = [1.0f32, 2.0, 3.0];
    assert_eq!(cosine_deviation(&a, &a).unwrap(), 0.0);
    assert_eq!(cosine_deviation(&[], &[]).unwrap(), 1.0);
    assert_eq!(cosine_deviation(&[0.0], &a[..1]).unwrap(), 1.0);
    assert_eq!(cosine_deviation(&a, &a[..2]), Err(LengthMismatch { left: 3, right: 2 }));
    assert!(is_noise(1e-7) && !is_noise(1e-6));
    assert_eq!(relative_error(&a, &a).unwrap(), 0.0);
    assert_eq!(relative_error(&[0.0, 0.0], &[0.0, 0.0]).unwrap(), 0.0);
    assert_eq!(relative_error(&[0.0, 0.0], &[1.0, 0.0]).unwrap(), f64::INFINITY);
    assert_eq!(top_k_overlap(&a, &a, 2).unwrap(), 1.0);
    assert_eq!(top_k_overlap(&a, &[3.0, 2.0, 1.0], 1).unwrap(), 0.0);
    assert_eq!(top_k_overlap(&a, &a, 0).unwrap(), 1.0);
    assert_eq!(top_k_overlap(&[], &[], 3).unwrap(), 1.0);
}

#[test]
fn assess_verdicts() {
    let cfg = DeviationConfig::new(0.1).unwrap();
    let r = [1.0f32, 2.0, 3.0, 4.0];
    let ok = assess(&r, &r, 4, &cfg);
    assert_eq!(ok.validity.kind, ValidityKind::Valid);
    assert_eq!(ok.worst_deviation, Some(0.0));
    assert!(!ok.validity.text.is_empty());

    assert_eq!(assess(&r, &r[..2], 4, &cfg).validity.kind, ValidityKind::Degenerate);
    assert_eq!(assess(&r, &r, 0, &cfg).validity.kind, ValidityKind::Degenerate);
    assert_eq!(assess(&r, &r, 3, &cfg).validity.kind, ValidityKind::Degenerate);
    assert_eq!(assess(&[], &[], 4, &cfg).validity.kind, ValidityKind::Degenerate);

    let nan = [1.0f32, f32::NAN, 3.0, 4.0];
    let a = assess(&r, &nan, 4, &cfg);
    assert_eq!(a.validity.kind, ValidityKind::NonFinite);
    assert!(a.rows.is_empty() && a.worst_deviation.is_none());

    let constant = [2.0f32; 4];
    assert_eq!(assess(&r, &constant, 4, &cfg).validity.kind, ValidityKind::Degenerate);
    let scaled = [10.0f32, 20.0, 30.0, 40.0];
    assert_eq!(assess(&r, &scaled, 4, &cfg).validity.kind, ValidityKind::Degenerate);

    let swapped = [4.0f32, 3.0, 2.0, 1.0];
    let cfg1 = DeviationConfig::new(0.1).unwrap().with_top_k(1).unwrap();
    let bad = assess(&r, &swapped, 4, &cfg1);
    assert_eq!(bad.validity.kind, ValidityKind::ExceedsMax);
    assert_eq!(bad.rows.len(), 1);
    assert_eq!(bad.worst_deviation, Some(1.0));
}

#[test]
fn invalid_config_is_degenerate() {
    assert!(DeviationConfig::new(f64::NAN).is_err());
    assert!(DeviationConfig::new(-1.0).is_err());
    assert!(DeviationConfig::new(0.1).unwrap().with_ratio_range(2.0, 1.0).is_err());
    assert!(DeviationConfig::new(0.1).unwrap().with_top_k(0).is_err());
    let mut cfg = DeviationConfig::new(0.1).unwrap();
    cfg.top_k = 0;
    let r = [1.0f32, 2.0, 3.0];
    let a = assess(&r, &r, 3, &cfg);
    assert_eq!(a.validity.kind, ValidityKind::Degenerate);
    assert_eq!(a.validity.text, "invalid configuration");
}

#[test]
fn evaluate_paths() {
    let w = Workload::build(small_cfg(5)).unwrap();
    let cfg = DeviationConfig::new(1e-3).unwrap();
    let reference = w.reference_scores(3).unwrap();
    let ok = w.evaluate(3, Ok(&reference), &cfg).unwrap();
    assert_eq!(ok.validity.kind, ValidityKind::Valid);
    assert_eq!(ok.rows.len(), 3);
    let failed = w.evaluate(3, Err(String::new()), &cfg).unwrap();
    assert_eq!(failed.validity.kind, ValidityKind::ExecutionFailed);
    assert_eq!(failed.validity.text, "execution failed");
    let empty = w.evaluate(0, Ok(&[]), &cfg).unwrap();
    assert_eq!(empty.validity.kind, ValidityKind::Degenerate);
    assert!(matches!(
        w.evaluate(7, Ok(&reference), &cfg),
        Err(BenchError::TooManyQueries { .. })
    ));
}
