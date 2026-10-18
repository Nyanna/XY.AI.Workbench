//! Pareto front and arc-length positions over (time, deviation) measurements.

/// Minimizes time and deviation. Non-finite points are dropped; the result is
/// ordered by ascending time with strictly decreasing deviation.
pub fn pareto(mut pts: Vec<(f64, f64)>) -> Vec<(f64, f64)> {
    pts.retain(|p| p.0.is_finite() && p.1.is_finite());
    pts.sort_by(|a, b| a.0.total_cmp(&b.0).then(a.1.total_cmp(&b.1)));
    let mut out: Vec<(f64, f64)> = Vec::new();
    for p in pts {
        if out.last().is_none_or(|l| p.1 < l.1) {
            out.push(p);
        }
    }
    out
}

/// Positions in `[0, 1]` for a front from [`pareto`], by cumulative arc length
/// over min-max normalized axes. Time is normalized logarithmically when
/// `log_time` is set and all times are positive.
pub fn positions(front: &[(f64, f64)], log_time: bool) -> Vec<f64> {
    match front.len() {
        0 => return Vec::new(),
        1 => return vec![0.5],
        _ => {}
    }
    let use_log = log_time && front.iter().all(|p| p.0 > 0.0);
    let t: Vec<f64> = front
        .iter()
        .map(|p| if use_log { p.0.ln() } else { p.0 })
        .collect();
    let d: Vec<f64> = front.iter().map(|p| p.1).collect();
    let (t, d) = (normalize(&t), normalize(&d));

    let mut cum = Vec::with_capacity(front.len());
    let mut s = 0.0;
    cum.push(s);
    for i in 1..front.len() {
        s += (t[i] - t[i - 1]).hypot(d[i] - d[i - 1]);
        cum.push(s);
    }
    if s > 0.0 {
        cum.iter().map(|c| c / s).collect()
    } else {
        let last = (front.len() - 1) as f64;
        (0..front.len()).map(|i| i as f64 / last).collect()
    }
}

fn normalize(v: &[f64]) -> Vec<f64> {
    let min = v.iter().copied().fold(f64::INFINITY, f64::min);
    let max = v.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let range = max - min;
    if range > 0.0 {
        v.iter().map(|x| (x - min) / range).collect()
    } else {
        vec![0.0; v.len()]
    }
}

#[cfg(test)]
mod tests {
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
}
