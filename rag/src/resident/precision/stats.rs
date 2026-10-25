//! Robust statistics for timing series. Load only ever slows a run down, so estimates take
//! the median or the minimum and spread ignores the slowest fifth of the samples.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Spread {
    /// Standard deviation over mean.
    Cv,
    /// `(max - min) / median`.
    RangeOverMedian,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Estimator {
    Median,
    Min,
}

fn sorted(values: &[f64]) -> Vec<f64> {
    let mut v = values.to_vec();
    v.sort_by(f64::total_cmp);
    v
}

pub fn median(values: &[f64]) -> f64 {
    if values.is_empty() {
        return f64::NAN;
    }
    let v = sorted(values);
    let mid = v.len() / 2;
    if v.len() % 2 == 1 {
        v[mid]
    } else {
        (v[mid - 1] + v[mid]) / 2.0
    }
}

pub fn estimate(values: &[f64], how: Estimator) -> f64 {
    match how {
        Estimator::Median => median(values),
        Estimator::Min => values.iter().copied().fold(f64::NAN, f64::min),
    }
}

/// Relative scatter of `values` after dropping the slowest `len / 5`; infinite when there
/// are fewer than two samples or no positive centre.
pub fn spread(values: &[f64], kind: Spread) -> f64 {
    let mut v = sorted(values);
    v.truncate(v.len() - v.len() / 5);
    if v.len() < 2 || v.iter().any(|x| !x.is_finite()) {
        return f64::INFINITY;
    }
    match kind {
        Spread::Cv => {
            let n = v.len() as f64;
            let mean = v.iter().sum::<f64>() / n;
            if mean <= 0.0 {
                return f64::INFINITY;
            }
            let var = v.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / n;
            var.sqrt() / mean
        }
        Spread::RangeOverMedian => {
            let m = median(&v);
            if m <= 0.0 {
                return f64::INFINITY;
            }
            (v[v.len() - 1] - v[0]) / m
        }
    }
}

#[cfg(test)]
#[path = "test/stats.rs"]
mod tests;
