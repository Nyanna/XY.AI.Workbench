use super::*;

fn ramp(n: usize) -> Vec<f32> {
    (0..n).map(|i| (i as f32 * 0.37).sin() + 0.1 * i as f32).collect()
}

#[test]
fn identical_vectors_have_zero_deviation() {
    let v = ramp(64);
    assert_eq!(cosine_deviation(&v, &v), 0.0);
    assert_eq!(relative_error(&v, &v), 0.0);
}

#[test]
fn rounding_noise_below_floor_counts_as_equal() {
    let v = ramp(64);
    let w: Vec<f32> = v.iter().map(|x| x * (1.0 + 1e-5)).collect();
    assert_eq!(cosine_deviation(&w, &v), 0.0);
}

#[test]
fn orthogonal_vectors_deviate_by_one() {
    assert!((cosine_deviation(&[1.0, 0.0], &[0.0, 1.0]) - 1.0).abs() < 1e-12);
}

#[test]
fn zero_vector_deviates_fully() {
    assert_eq!(cosine_deviation(&[0.0; 4], &[1.0; 4]), 1.0);
}

#[test]
fn relative_error_scales_with_difference() {
    let e = relative_error(&[1.1, 0.0], &[1.0, 0.0]);
    assert!((e - 0.1).abs() < 1e-6);
    assert_eq!(relative_error(&[1.0], &[0.0]), f64::INFINITY);
}

#[test]
fn top_k_overlap_counts_shared_ranks() {
    let reference = [0.9, 0.8, 0.7, 0.1, 0.0];
    assert_eq!(top_k_overlap(&reference, &reference, 3), 1.0);
    let swapped = [0.9, 0.1, 0.7, 0.8, 0.0];
    assert!((top_k_overlap(&swapped, &reference, 3) - 2.0 / 3.0).abs() < 1e-12);
}

#[test]
fn non_finite_candidates_are_rejected() {
    let r = ramp(8);
    let mut c = r.clone();
    c[3] = f32::NAN;
    let (_, v) = judge_rows(&c, &r, 8, None, &Plausibility::default());
    assert_eq!(v, Validity::NonFinite);
    c[3] = f32::INFINITY;
    let (_, v) = judge_rows(&c, &r, 8, None, &Plausibility::default());
    assert_eq!(v, Validity::NonFinite);
}

#[test]
fn constant_vectors_are_degenerate() {
    let r = ramp(8);
    let (_, v) = judge_rows(&[0.5; 8], &r, 8, None, &Plausibility::default());
    assert_eq!(v, Validity::Degenerate);
}

#[test]
fn norm_outside_reference_ratio_is_degenerate() {
    let r = ramp(8);
    let big: Vec<f32> = r.iter().map(|x| x * 3.0).collect();
    let (_, v) = judge_rows(&big, &r, 8, None, &Plausibility::default());
    assert_eq!(v, Validity::Degenerate);
    let small: Vec<f32> = r.iter().map(|x| x * 0.2).collect();
    let (_, v) = judge_rows(&small, &r, 8, None, &Plausibility::default());
    assert_eq!(v, Validity::Degenerate);
}

#[test]
fn worst_row_decides_the_deviation() {
    let r: Vec<f32> = ramp(16);
    let mut c = r.clone();
    for x in &mut c[8..] {
        *x += 0.5 * (*x).signum() * x.abs().sqrt();
    }
    let (dev, v) = judge_rows(&c, &r, 8, None, &Plausibility::default());
    assert_eq!(v, Validity::Ok);
    assert!(dev > 0.0);
    assert_eq!(dev, cosine_deviation(&c[8..], &r[8..]));
}

#[test]
fn rank_metric_raises_the_deviation_of_reordered_scores() {
    let reference = [0.9, 0.8, 0.7, 0.1, 0.0, 0.3];
    let reordered = [0.9, 0.1, 0.7, 0.8, 0.0, 0.3];
    let (plain, _) = judge_rows(&reordered, &reference, 6, None, &Plausibility::default());
    let (ranked, _) = judge_rows(&reordered, &reference, 6, Some(3), &Plausibility::default());
    assert!(ranked > plain);
}

#[test]
fn shape_mismatch_is_degenerate() {
    let (_, v) = judge_rows(&[1.0; 5], &[1.0; 6], 3, None, &Plausibility::default());
    assert_eq!(v, Validity::Degenerate);
    let (_, v) = judge_rows(&[1.0; 5], &[1.0; 5], 0, None, &Plausibility::default());
    assert_eq!(v, Validity::Degenerate);
}

#[test]
fn duplicates_are_detected() {
    assert!(is_duplicate_free(&["a", "b", "c"]));
    assert!(!is_duplicate_free(&["a", "b", "a"]));
}
