//! Pareto front and arc-length positions over (time, deviation) measurements.
/// Minimizes time and deviation. Non-finite points are dropped; the result is
/// ordered by ascending time with strictly decreasing deviation.
pub fn pareto(pts: Vec<(f64, f64)>) -> Vec<(f64, f64)> {
    pareto_by(pts, |p| *p)
}
/// Same front as [`pareto`] over arbitrary items; `key` yields `(time, deviation)`.
pub fn pareto_by<T>(mut items: Vec<T>, key: impl Fn(&T) -> (f64, f64)) -> Vec<T> {
    items
        .retain(|i| {
            let k = key(i);
            k.0.is_finite() && k.1.is_finite()
        });
    items
        .sort_by(|a, b| {
            let (a, b) = (key(a), key(b));
            a.0.total_cmp(&b.0).then(a.1.total_cmp(&b.1))
        });
    let mut out: Vec<T> = Vec::new();
    let mut best = f64::INFINITY;
    for item in items {
        let dev = key(&item).1;
        if dev < best {
            best = dev;
            out.push(item);
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
#[path = "test/pareto.rs"]
mod tests;
