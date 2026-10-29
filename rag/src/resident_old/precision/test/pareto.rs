use super::*;

#[test]
fn dominated_points_are_dropped() {
    let front = pareto(vec![(3.0, 0.1), (1.0, 0.9), (2.0, 0.95), (2.0, 0.5), (4.0, 0.1)]);
    assert_eq!(front, vec![(1.0, 0.9), (2.0, 0.5), (3.0, 0.1)]);
    assert!(front.windows(2).all(|w| w[0].0 < w[1].0 && w[0].1 > w[1].1));
}

#[test]
fn non_finite_points_are_ignored() {
    let front = pareto(vec![(f64::NAN, 0.1), (1.0, f64::INFINITY), (2.0, 0.3)]);
    assert_eq!(front, vec![(2.0, 0.3)]);
    assert!(pareto(Vec::new()).is_empty());
}

#[test]
fn positions_follow_arc_length() {
    let p = positions(&[(0.0, 1.0), (1.0, 0.0), (2.0, -1.0)], false);
    assert_eq!(p.len(), 3);
    assert_eq!(p[0], 0.0);
    assert!((p[1] - 0.5).abs() < 1e-12);
    assert_eq!(p[2], 1.0);

    let skewed = positions(&[(0.0, 1.0), (1.0, 0.9), (10.0, 0.0)], false);
    assert!(skewed[1] < 0.5);
}

#[test]
fn single_point_sits_in_the_middle() {
    assert_eq!(positions(&[(1.0, 1.0)], false), vec![0.5]);
    assert!(positions(&[], false).is_empty());
}

#[test]
fn constant_axis_does_not_divide_by_zero() {
    let p = positions(&[(1.0, 0.5), (2.0, 0.4), (3.0, 0.3)], false);
    assert!(p.iter().all(|x| x.is_finite()));
    assert_eq!(p[0], 0.0);
    assert_eq!(*p.last().unwrap(), 1.0);

    let flat = positions(&[(1.0, 1.0), (1.0, 1.0)], false);
    assert_eq!(flat, vec![0.0, 1.0]);
}

#[test]
fn log_time_compresses_large_ratios() {
    let front = [(1.0, 1.0), (10.0, 0.5), (1000.0, 0.0)];
    let lin = positions(&front, false);
    let log = positions(&front, true);
    assert!(log[1] > lin[1]);
    let non_positive = positions(&[(0.0, 1.0), (1.0, 0.0)], true);
    assert_eq!(non_positive, positions(&[(0.0, 1.0), (1.0, 0.0)], false));
}
