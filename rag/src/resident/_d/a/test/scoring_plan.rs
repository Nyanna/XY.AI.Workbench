use super::*;

fn neg_l2sq(a: &[f32], b: &[f32]) -> f32 {
    -a.iter().zip(b).map(|(x, y)| (x - y) * (x - y)).sum::<f32>()
}

#[test]
fn scoring_maps_to_metric_and_transform() {
    assert_eq!(metric_for(Scoring::Cosine), (Metric::Dot, ScoreTransform::Identity));
    assert_eq!(metric_for(Scoring::DotProduct), (Metric::Dot, ScoreTransform::Identity));
    assert_eq!(metric_for(Scoring::NegEuclidean), (Metric::NegL2Sq, ScoreTransform::NegRoot));
}

#[test]
fn identity_keeps_every_bit_pattern() {
    for s in [0.0f32, -0.0, 1.5, -2.5, f32::INFINITY, f32::NEG_INFINITY] {
        assert_eq!(ScoreTransform::Identity.apply(s).to_bits(), s.to_bits());
    }
    assert!(ScoreTransform::Identity.apply(f32::NAN).is_nan());
}

#[test]
fn neg_root_restores_negative_distance() {
    assert_eq!(ScoreTransform::NegRoot.apply(-4.0), -2.0);
    assert_eq!(ScoreTransform::NegRoot.apply(-0.0), 0.0);
    assert_eq!(ScoreTransform::NegRoot.apply(f32::NEG_INFINITY), f32::NEG_INFINITY);
    assert!(ScoreTransform::NegRoot.apply(f32::NAN).is_nan());
    assert_eq!(ScoreTransform::NegRoot.apply(1e-9), 0.0);
}

#[test]
fn neg_root_matches_euclidean_definition_and_keeps_order() {
    let q = [1.0f32, 2.0, 3.0];
    let rows = [[1.0f32, 2.0, 3.0], [0.0, 2.0, 5.0], [4.0, 4.0, 4.0], [-1.0, 0.0, 1.0]];
    let raw: Vec<f32> = rows.iter().map(|r| neg_l2sq(&q, r)).collect();
    let mapped: Vec<f32> = raw.iter().map(|s| ScoreTransform::NegRoot.apply(*s)).collect();
    for (r, m) in rows.iter().zip(&mapped) {
        let dist: f32 = q.iter().zip(r).map(|(a, b)| (a - b) * (a - b)).sum::<f32>().sqrt();
        assert!((m + dist).abs() < 1e-6);
    }
    let order = |v: &[f32]| {
        let mut idx: Vec<usize> = (0..v.len()).collect();
        idx.sort_by(|a, b| v[*b].partial_cmp(&v[*a]).unwrap());
        idx
    };
    assert_eq!(order(&raw), order(&mapped));
}

#[test]
fn precision_maps_to_dtype() {
    assert_eq!(dtype_for(Precision::F32), Ok(DType::F32));
    assert_eq!(dtype_for(Precision::F16), Ok(DType::F16));
}

#[test]
fn precisions_without_kernel_are_errors() {
    assert_eq!(dtype_for(Precision::Int8), Err(PrecisionError("Int8")));
    assert_eq!(dtype_for(Precision::Bf16), Err(PrecisionError("Bf16")));
    assert_eq!(
        PrecisionError("Bf16").to_string(),
        "precision Bf16 has no scoring kernel"
    );
}

#[test]
fn plan_combines_both_mappings() {
    let p = plan3(Scoring::NegEuclidean, Precision::F16).unwrap();
    assert_eq!(
        p,
        ScoringPlan {
            metric: Metric::NegL2Sq,
            transform: ScoreTransform::NegRoot,
            dtype: DType::F16,
        }
    );
    let p = plan3(Scoring::Cosine, Precision::F32).unwrap();
    assert_eq!((p.metric, p.dtype), (Metric::Dot, DType::F32));
    assert_eq!(plan3(Scoring::Cosine, Precision::Int8), Err(PrecisionError("Int8")));
}

#[test]
fn types_are_shareable() {
    fn assert_traits<T: Send + Sync + 'static>() {}
    assert_traits::<ScoreTransform>();
    assert_traits::<ScoringPlan>();
    assert_traits::<PrecisionError>();
}
