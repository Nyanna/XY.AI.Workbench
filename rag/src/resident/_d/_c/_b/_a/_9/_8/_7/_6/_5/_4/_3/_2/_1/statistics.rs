#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Estimator {
    Median,
    Minimum,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dispersion {
    CoefficientOfVariation,
    TrimmedRange,
}

/// Minimum number of valid samples for `dispersion`.
pub const MIN_DISPERSION_SAMPLES: usize = 3;

fn sorted_valid(samples: &[f64]) -> Vec<f64> {
    let mut v: Vec<f64> = samples.iter().copied().filter(|x| x.is_finite()).collect();
    v.sort_by(|a, b| a.total_cmp(b));
    v
}

fn median_of_sorted(v: &[f64]) -> f64 {
    let n = v.len();
    if n % 2 == 1 {
        v[n / 2]
    } else {
        let (a, b) = (v[n / 2 - 1], v[n / 2]);
        a + (b - a) / 2.0
    }
}

pub fn median(samples: &[f64]) -> Option<f64> {
    let v = sorted_valid(samples);
    if v.is_empty() {
        None
    } else {
        Some(median_of_sorted(&v))
    }
}

pub fn minimum(samples: &[f64]) -> Option<f64> {
    samples
        .iter()
        .copied()
        .filter(|x| x.is_finite())
        .min_by(|a, b| a.total_cmp(b))
}

pub fn estimate(samples: &[f64], estimator: Estimator) -> Option<f64> {
    match estimator {
        Estimator::Median => median(samples),
        Estimator::Minimum => minimum(samples),
    }
}

pub fn dispersion(samples: &[f64], kind: Dispersion) -> f64 {
    let v = sorted_valid(samples);
    let n = v.len();
    if n < MIN_DISPERSION_SAMPLES {
        return f64::INFINITY;
    }
    let (spread, center) = match kind {
        Dispersion::CoefficientOfVariation => {
            let mean = v.iter().sum::<f64>() / n as f64;
            let var = v.iter().map(|x| (x - mean) * (x - mean)).sum::<f64>() / (n - 1) as f64;
            (var.sqrt(), mean)
        }
        Dispersion::TrimmedRange => {
            let kept = &v[..n - n / 5];
            (kept[kept.len() - 1] - kept[0], median_of_sorted(kept))
        }
    };
    if !(center > 0.0) {
        return f64::INFINITY;
    }
    let r = spread / center;
    if r.is_finite() {
        r
    } else {
        f64::INFINITY
    }
}

#[cfg(test)]
#[path = "test/statistics.rs"]
mod tests;
