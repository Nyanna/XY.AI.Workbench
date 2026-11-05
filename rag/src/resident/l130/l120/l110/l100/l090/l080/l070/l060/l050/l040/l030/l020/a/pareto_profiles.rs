//! Pareto front over (time, deviation) and derivation of unnamed precision profiles.

use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::fmt;

use super::super::l010::capability_model::{Axis, Value};

pub const DEFAULT_MERGE_THRESHOLD: f64 = 0.1;

/// One calibration measurement.
#[derive(Clone, Debug, PartialEq)]
pub struct Measurement {
    pub config: BTreeMap<Axis, Value>,
    pub time: f64,
    pub deviation: f64,
}

/// Unnamed precision profile; position 0 is fastest, 1 most precise.
#[derive(Clone, Debug, PartialEq)]
pub struct PrecisionProfile {
    pub position: f64,
    pub config: BTreeMap<Axis, Value>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ProfileError {
    InvalidThreshold(f64),
}

impl fmt::Display for ProfileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProfileError::InvalidThreshold(v) => {
                write!(f, "invalid merge threshold: {v} (must be finite and >= 0)")
            }
        }
    }
}

impl std::error::Error for ProfileError {}

pub fn pareto_front_pairs(points: &[(f64, f64)]) -> Vec<(f64, f64)> {
    pareto_front(points, |p| *p)
}

pub fn pareto_front<T: Clone>(items: &[T], key: impl Fn(&T) -> (f64, f64)) -> Vec<T> {
    let mut keyed: Vec<(usize, f64, f64)> = items
        .iter()
        .enumerate()
        .filter_map(|(i, item)| {
            let (t, d) = key(item);
            (t.is_finite() && d.is_finite()).then_some((i, t, d))
        })
        .collect();
    keyed.sort_by(|a, b| {
        a.1.partial_cmp(&b.1)
            .unwrap_or(Ordering::Equal)
            .then(a.2.partial_cmp(&b.2).unwrap_or(Ordering::Equal))
            .then(a.0.cmp(&b.0))
    });
    let mut best = f64::INFINITY;
    let mut out = Vec::new();
    for (i, _, d) in keyed {
        if d < best {
            best = d;
            out.push(items[i].clone());
        }
    }
    out
}

fn normalize(values: &[f64]) -> Vec<f64> {
    let min = values.iter().copied().fold(f64::INFINITY, f64::min);
    let max = values.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let range = max - min;
    values
        .iter()
        .map(|&v| {
            if range > 0.0 && range.is_finite() {
                let n = (v - min) / range;
                if n.is_finite() {
                    n.clamp(0.0, 1.0)
                } else {
                    0.0
                }
            } else {
                0.0
            }
        })
        .collect()
}

pub fn positions(points: &[(f64, f64)], log_time: bool) -> Vec<f64> {
    let n = points.len();
    match n {
        0 => return Vec::new(),
        1 => return vec![0.5],
        _ => {}
    }
    let times: Vec<f64> = points
        .iter()
        .map(|p| {
            if log_time {
                p.0.max(f64::MIN_POSITIVE).ln()
            } else {
                p.0
            }
        })
        .collect();
    let devs: Vec<f64> = points.iter().map(|p| p.1).collect();
    let xs = normalize(&times);
    let ys = normalize(&devs);

    let mut cumulative = Vec::with_capacity(n);
    let mut total = 0.0_f64;
    cumulative.push(0.0);
    for i in 1..n {
        total += (xs[i] - xs[i - 1]).hypot(ys[i] - ys[i - 1]);
        cumulative.push(total);
    }
    if !(total > 0.0) || !total.is_finite() {
        let denom = (n - 1) as f64;
        return (0..n).map(|i| i as f64 / denom).collect();
    }
    let mut out: Vec<f64> = cumulative
        .into_iter()
        .map(|c| (c / total).clamp(0.0, 1.0))
        .collect();
    out[n - 1] = 1.0;
    out
}

pub fn derive_profiles(
    measurements: &[Measurement],
    merge_threshold: f64,
) -> Result<Vec<PrecisionProfile>, ProfileError> {
    if !merge_threshold.is_finite() || merge_threshold < 0.0 {
        return Err(ProfileError::InvalidThreshold(merge_threshold));
    }

    let usable: Vec<Measurement> = measurements
        .iter()
        .filter(|m| m.time.is_finite() && m.deviation.is_finite() && m.time > 0.0)
        .cloned()
        .collect();
    let front = pareto_front(&usable, |m| (m.time, m.deviation));
    if front.is_empty() {
        return Ok(Vec::new());
    }

    let log_times: Vec<f64> = front.iter().map(|m| m.time.ln()).collect();
    let t_hat = normalize(&log_times);
    let last = front.len() - 1;
    let mut remaining: Vec<&Measurement> = Vec::with_capacity(front.len());
    for (i, m) in front.iter().enumerate() {
        // t_hat is non-decreasing, so the next point is the closest slower one.
        let removed = i < last && t_hat[i + 1] - t_hat[i] < merge_threshold;
        if removed {
            log::info!(
                "profile filter removed config={:?} time={} deviation={}",
                m.config,
                m.time,
                m.deviation
            );
        } else {
            remaining.push(m);
        }
    }
    if remaining.is_empty() {
        return Ok(Vec::new());
    }

    let pairs: Vec<(f64, f64)> = remaining.iter().map(|m| (m.time, m.deviation)).collect();
    let pos = positions(&pairs, true);

    let mut representatives: Vec<&Measurement> = Vec::new();
    let mut group_best = 0usize;
    for k in 1..=remaining.len() {
        let boundary = k == remaining.len() || pos[k] - pos[k - 1] >= merge_threshold;
        if boundary {
            representatives.push(remaining[group_best]);
            if k < remaining.len() {
                group_best = k;
            }
        } else if remaining[k].deviation < remaining[group_best].deviation {
            group_best = k;
        }
    }

    let rep_pairs: Vec<(f64, f64)> = representatives
        .iter()
        .map(|m| (m.time, m.deviation))
        .collect();
    let rep_pos = positions(&rep_pairs, true);
    Ok(representatives
        .into_iter()
        .zip(rep_pos)
        .map(|(m, position)| PrecisionProfile {
            position,
            config: m.config.clone(),
        })
        .collect())
}

#[cfg(test)]
#[path = "test/pareto_profiles.rs"]
mod tests;
