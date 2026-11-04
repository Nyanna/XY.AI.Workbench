use super::*;
#[test]
fn mean_ignores_masked_positions() {
    let hidden = [1.0, 2.0, 3.0, 4.0, 100.0, 100.0];
    let out = mean_pool(&hidden, &[1, 1, 0], 1, 3, 2);
    assert_eq!(out, vec![2.0, 3.0]);
}
#[test]
fn fully_masked_row_is_zero() {
    assert_eq!(mean_pool(& [5.0, 5.0], & [0], 1, 1, 2), vec![0.0, 0.0]);
}
#[test]
fn rows_become_unit_length() {
    let mut rows = [3.0, 4.0, 0.0, 0.0];
    l2_normalize_rows(&mut rows, 2);
    assert!((rows[0] - 0.6).abs() < 1e-6 && (rows[1] - 0.8).abs() < 1e-6);
    assert_eq!(& rows[2..], & [0.0, 0.0]);
}
#[test]
fn zero_rows_are_counted_and_never_nan() {
    let mut rows = [0.0, 0.0, 3.0, 4.0, 0.0, 0.0];
    assert_eq!(l2_normalize_rows(& mut rows, 2), 2);
    assert!(rows.iter().all(| x | x.is_finite()));
}
#[test]
fn padding_does_not_change_the_mean() {
    let hidden = [1.0, 2.0, 3.0, 4.0, 9.0, 9.0, 1.0, 2.0, 3.0, 4.0];
    let short = mean_pool(&hidden[..4], &[1, 1], 1, 2, 2);
    let padded = mean_pool(&hidden[..6], &[1, 1, 0], 1, 3, 2);
    assert_eq!(short, padded);
}
