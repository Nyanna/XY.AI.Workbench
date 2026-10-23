use super::*;

#[test]
fn mean_ignores_masked_positions() {
    let hidden = [1.0, 2.0, 3.0, 4.0, 100.0, 100.0];
    let out = mean_pool(&hidden, &[1, 1, 0], 1, 3, 2);
    assert_eq!(out, vec![2.0, 3.0]);
}

#[test]
fn fully_masked_row_is_zero() {
    assert_eq!(mean_pool(&[5.0, 5.0], &[0], 1, 1, 2), vec![0.0, 0.0]);
}

#[test]
fn rows_become_unit_length() {
    let mut rows = [3.0, 4.0, 0.0, 0.0];
    l2_normalize_rows(&mut rows, 2);
    assert!((rows[0] - 0.6).abs() < 1e-6 && (rows[1] - 0.8).abs() < 1e-6);
    assert_eq!(&rows[2..], &[0.0, 0.0]);
}
