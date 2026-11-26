use super::*;

#[test]
fn median_basic() {
    assert_eq!(median(&[]), None);
    assert_eq!(median(&[f64::NAN, f64::INFINITY]), None);
    assert_eq!(median(&[3.0, 1.0, 2.0]), Some(2.0));
    assert_eq!(median(&[4.0, 1.0, f64::NAN, 3.0, 2.0]), Some(2.5));
}

#[test]
fn minimum_basic() {
    assert_eq!(minimum(&[]), None);
    assert_eq!(minimum(&[f64::NEG_INFINITY, 5.0, 2.0, f64::NAN]), Some(2.0));
    assert_eq!(estimate(&[5.0, 2.0, 9.0], Estimator::Minimum), Some(2.0));
    assert_eq!(estimate(&[5.0, 2.0, 9.0], Estimator::Median), Some(5.0));
}

#[test]
fn dispersion_too_few_or_nonpositive() {
    for k in [Dispersion::CoefficientOfVariation, Dispersion::TrimmedRange] {
        assert!(dispersion(&[1.0, 2.0], k).is_infinite());
        assert!(dispersion(&[1.0, 2.0, f64::NAN], k).is_infinite());
        assert!(dispersion(&[-1.0, -2.0, -3.0], k).is_infinite());
        assert!(dispersion(&[0.0, 0.0, 0.0], k).is_infinite());
        assert_eq!(dispersion(&[2.0, 2.0, 2.0], k), 0.0);
    }
}

#[test]
fn coefficient_of_variation_value() {
    let d = dispersion(&[1.0, 2.0, 3.0], Dispersion::CoefficientOfVariation);
    assert!((d - 0.5).abs() < 1e-12);
}

#[test]
fn trimmed_range_drops_slowest_fifth() {
    let s = [10.0, 11.0, 12.0, 10.0, 1000.0];
    // n=5 -> drop 1 -> kept [10,10,11,12], median 10.5, range 2
    let d = dispersion(&s, Dispersion::TrimmedRange);
    assert!((d - 2.0 / 10.5).abs() < 1e-12);
    // n=4 -> nothing dropped
    let d4 = dispersion(&[1.0, 2.0, 3.0, 5.0], Dispersion::TrimmedRange);
    assert!((d4 - 4.0 / 2.5).abs() < 1e-12);
}

#[test]
fn input_not_modified() {
    let s = vec![3.0, 1.0, 2.0];
    let _ = median(&s);
    assert_eq!(s, vec![3.0, 1.0, 2.0]);
}
