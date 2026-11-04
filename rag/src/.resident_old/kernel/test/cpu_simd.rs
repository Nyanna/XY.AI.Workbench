use super::*;

#[test]
fn dot_handles_remainders() {
    for n in [0usize, 1, 7, 8, 9, 17, 384] {
        let a: Vec<f32> = (0..n).map(|i| i as f32 * 0.5).collect();
        let b: Vec<f32> = (0..n).map(|i| 1.0 - i as f32 * 0.25).collect();
        let want: f32 = a.iter().zip(&b).map(|(x, y)| x * y).sum();
        assert!((dot(&a, &b) - want).abs() <= 1e-3 * want.abs().max(1.0), "n={n}");
    }
}

#[test]
fn maxsim_matches_hand_computation() {
    let q = [1.0, 0.0, 0.0, 1.0];
    let d = [0.5, 0.2, 0.1, 0.9, 0.3, 0.3];
    assert!((maxsim(&q, &d, 2) - (0.5 + 0.9)).abs() < 1e-6);
}

#[test]
fn empty_document_scores_mask_value_per_query_token() {
    let q = [1.0, 0.0, 0.0, 1.0];
    let s = maxsim(&q, &[], 2);
    assert_eq!(s, 2.0 * F16_MASK_VALUE);
    assert!(s.is_finite());
}

#[test]
fn normalize_yields_unit_length() {
    let mut v = vec![3.0, 4.0];
    normalize(&mut v);
    assert!((dot(&v, &v) - 1.0).abs() < 1e-6);
    let mut z = vec![0.0; 3];
    normalize(&mut z);
    assert_eq!(z, vec![0.0; 3]);
}
