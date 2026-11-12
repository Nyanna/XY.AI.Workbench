use super::*;

fn pn(p: PoolingKind, n: Normalization, dim: usize) -> PoolNorm {
    PoolNorm::new(p, n, dim).unwrap()
}

#[test]
fn new_zero_dim() {
    assert_eq!(
        PoolNorm::new(PoolingKind::Mean, Normalization::L2, 0),
        Err(PoolNormError::ZeroDimension)
    );
}

#[test]
fn mean_ignores_padding_even_nan() {
    let p = pn(PoolingKind::Mean, Normalization::None, 2);
    let hidden = [1.0, 2.0, 3.0, 4.0, f32::NAN, f32::INFINITY];
    let out = p.pool(&hidden, &[1, 1, 0], 1, 3).unwrap();
    assert_eq!(out, vec![2.0, 3.0]);
}

#[test]
fn mean_all_masked_is_zero() {
    let p = pn(PoolingKind::Mean, Normalization::L2, 2);
    let out = p.apply(&[f32::NAN; 4], &[0, 0], 1, 2).unwrap();
    assert_eq!(out.vectors.len(), 2);
    assert_eq!(out.num_vectors, 1);
    assert_eq!(out.degenerate_count, 1);
    assert!(out.vectors.iter().all(|v| *v == 0.0));
}

#[test]
fn mean_seq_zero() {
    let p = pn(PoolingKind::Mean, Normalization::L2, 3);
    let out = p.apply(&[], &[], 2, 0).unwrap();
    assert_eq!(out.vectors, vec![0.0; 6]);
    assert_eq!(out.degenerate_count, 2);
}

#[test]
fn none_pooling_copies_and_counts_padding() {
    let p = pn(PoolingKind::None, Normalization::L2, 2);
    let hidden = [3.0, 4.0, 0.0, 0.0];
    let out = p.apply(&hidden, &[1, 0], 1, 2).unwrap();
    assert_eq!(out.num_vectors, 2);
    assert_eq!(out.degenerate_count, 1);
    assert!((out.vectors[0] - 0.6).abs() < 1e-6);
    assert!((out.vectors[1] - 0.8).abs() < 1e-6);
}

#[test]
fn l2_unit_norm_and_idempotent() {
    let p = pn(PoolingKind::Mean, Normalization::L2, 3);
    let mut v = vec![1.0, 2.0, 2.0];
    assert_eq!(p.normalize(&mut v).unwrap(), 0);
    let n: f32 = v.iter().map(|x| x * x).sum::<f32>().sqrt();
    assert!((n - 1.0).abs() < 1e-6);
}

#[test]
fn nan_vector_untouched() {
    let p = pn(PoolingKind::Mean, Normalization::L2, 2);
    let mut v = vec![f32::NAN, 1.0];
    assert_eq!(p.normalize(&mut v).unwrap(), 0);
    assert!(v[0].is_nan());
    assert_eq!(v[1], 1.0);
}

#[test]
fn normalization_none_unchanged() {
    let p = pn(PoolingKind::Mean, Normalization::None, 2);
    let mut v = vec![0.0, 0.0, 5.0, 5.0];
    assert_eq!(p.normalize(&mut v).unwrap(), 0);
    assert_eq!(v, vec![0.0, 0.0, 5.0, 5.0]);
}

#[test]
fn shape_errors() {
    let p = pn(PoolingKind::Mean, Normalization::L2, 2);
    assert!(matches!(p.pool(&[1.0; 3], &[1], 1, 1), Err(PoolNormError::ShapeMismatch(_))));
    assert!(matches!(p.pool(&[1.0; 2], &[1, 1], 1, 1), Err(PoolNormError::ShapeMismatch(_))));
    assert!(matches!(p.normalize(&mut [1.0; 3]), Err(PoolNormError::ShapeMismatch(_))));
    assert_eq!(p.normalize(&mut []).unwrap(), 0);
    let n = pn(PoolingKind::None, Normalization::None, 2);
    assert!(matches!(n.pool(&[1.0; 2], &[1, 1], 1, 1), Err(PoolNormError::ShapeMismatch(_))));
}

#[test]
fn empty_batch() {
    let p = pn(PoolingKind::Mean, Normalization::L2, 4);
    let out = p.apply(&[], &[], 0, 5).unwrap();
    assert_eq!(out.num_vectors, 0);
    assert!(out.vectors.is_empty());
}

#[test]
fn error_display() {
    assert_eq!(PoolNormError::ZeroDimension.to_string(), "dimension muss größer als 0 sein");
    assert_eq!(
        PoolNormError::UnsupportedPooling("Cls".into()).to_string(),
        "nicht unterstütztes Pooling: Cls"
    );
}
