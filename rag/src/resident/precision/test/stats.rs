use super::*;

#[test]
fn median_handles_odd_even_and_empty() {
    assert_eq!(median(&[3.0, 1.0, 2.0]), 2.0);
    assert_eq!(median(&[4.0, 1.0, 2.0, 3.0]), 2.5);
    assert!(median(&[]).is_nan());
}

#[test]
fn estimators_pick_median_or_minimum() {
    let v = [5.0, 1.0, 9.0, 2.0, 3.0];
    assert_eq!(estimate(&v, Estimator::Median), 3.0);
    assert_eq!(estimate(&v, Estimator::Min), 1.0);
}

#[test]
fn constant_series_has_zero_spread() {
    for kind in [Spread::Cv, Spread::RangeOverMedian] {
        assert_eq!(spread(&[2.0; 6], kind), 0.0);
    }
}

#[test]
fn one_slow_outlier_in_five_is_ignored() {
    let v = [1.0, 1.0, 1.0, 1.0, 50.0];
    for kind in [Spread::Cv, Spread::RangeOverMedian] {
        assert_eq!(spread(&v, kind), 0.0);
    }
}

#[test]
fn short_or_invalid_series_never_count_as_stable() {
    assert_eq!(spread(&[1.0], Spread::Cv), f64::INFINITY);
    assert_eq!(spread(&[], Spread::Cv), f64::INFINITY);
    assert_eq!(spread(&[1.0, f64::NAN], Spread::Cv), f64::INFINITY);
    assert_eq!(spread(&[0.0, 0.0], Spread::Cv), f64::INFINITY);
}

#[test]
fn spread_grows_with_scatter() {
    let tight = spread(&[1.0, 1.01, 0.99, 1.0], Spread::Cv);
    let wide = spread(&[1.0, 1.5, 0.6, 1.0], Spread::Cv);
    assert!(wide > 10.0 * tight);
}
