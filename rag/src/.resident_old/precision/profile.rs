//! Unnamed profiles: a position on the fast-to-precise scale plus a configuration.
use std::collections::BTreeMap;
use tracing::{debug, info};
use super::axis::{AxisId, ValueId};
use super::calibrate::ParetoPoint;
use super::pareto::positions;
use crate::resident::error::PrecisionError;
/// Exactly one value per chosen axis.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct ProfileConfig(BTreeMap<AxisId, ValueId>);
impl ProfileConfig {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with(mut self, axis: impl Into<AxisId>, value: impl Into<ValueId>) -> Self {
        self.0.insert(axis.into(), value.into());
        self
    }
    pub fn get(&self, axis: &AxisId) -> Option<&ValueId> {
        self.0.get(axis)
    }
    pub fn iter(&self) -> impl Iterator<Item = (&AxisId, &ValueId)> {
        self.0.iter()
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct Profile {
    /// 0 is fastest, 1 is most precise.
    pub position: f32,
    pub config: ProfileConfig,
}
impl Profile {
    /// Position of a lone profile.
    pub const SINGLE_POSITION: f32 = 0.5;
    pub fn new(position: f32, config: ProfileConfig) -> Result<Self, PrecisionError> {
        if !(0.0..=1.0).contains(&position) {
            return Err(PrecisionError::InvalidPrecision {
                requested: position,
            });
        }
        Ok(Self { position, config })
    }
}
/// The only tuning knob of profile formation, in the normalized arc-length scale `[0, 1]`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ProfilingConfig {
    /// Neighbours on the front closer than this merge into one profile.
    pub spread_threshold: f64,
}
impl Default for ProfilingConfig {
    fn default() -> Self {
        Self { spread_threshold: 0.1 }
    }
}
/// Derives unnamed profiles from a front ordered by ascending time and strictly
/// decreasing deviation. The count follows from the spread alone; an empty result
/// means no usable profile.
pub fn derive_profiles(front: &[ParetoPoint], cfg: &ProfilingConfig) -> Vec<Profile> {
    let points = significant(front);
    if points.is_empty() {
        return Vec::new();
    }
    let coords = coordinates(&points);
    let pos = positions(&coords, true);
    let mut clusters: Vec<Vec<usize>> = vec![vec![0]];
    for i in 1..points.len() {
        if pos[i] - pos[i - 1] < cfg.spread_threshold {
            clusters.last_mut().expect("clusters is never empty").push(i);
        } else {
            clusters.push(vec![i]);
        }
    }
    let last = points.len() - 1;
    let reps: Vec<usize> = clusters
        .iter()
        .map(|c| {
            let rep = if c.contains(&last) { last } else { c[0] };
            debug!(
                merged = c.len(), representative = % points[rep].cfg, "profile cluster"
            );
            rep
        })
        .collect();
    let rep_points: Vec<&ParetoPoint> = reps.iter().map(|&i| points[i]).collect();
    let final_pos = positions(&coordinates(&rep_points), true);
    let profiles: Vec<Profile> = rep_points
        .iter()
        .zip(final_pos)
        .filter_map(|(p, pos)| {
            Profile::new(pos.clamp(0.0, 1.0) as f32, p.cfg.combo.clone()).ok()
        })
        .collect();
    info!(
        profiles = profiles.len(), positions = ? profiles.iter().map(| p | p.position)
        .collect::< Vec < _ >> (), "profiles derived"
    );
    profiles
}
fn coordinates(points: &[&ParetoPoint]) -> Vec<(f64, f64)> {
    points.iter().map(|p| (p.time.as_secs_f64(), p.deviation)).collect()
}
/// Drops finite-invalid points and every point that is less precise than a peer without
/// being significantly faster, i.e. whose time gap lies within the measurement uncertainty.
fn significant(front: &[ParetoPoint]) -> Vec<&ParetoPoint> {
    let valid: Vec<&ParetoPoint> = front
        .iter()
        .filter(|p| p.deviation.is_finite() && p.uncertainty.is_finite())
        .collect();
    valid
        .iter()
        .enumerate()
        .filter(|&(i, p)| {
            let redundant = valid
                .iter()
                .enumerate()
                .any(|(j, o)| {
                    j != i && o.deviation < p.deviation
                        && o.time.as_secs_f64()
                            <= p.time.as_secs_f64()
                                * (1.0 + p.uncertainty + o.uncertainty)
                });
            if redundant {
                debug!(
                    cfg = % p.cfg,
                    "profile candidate dropped: not faster than a more precise peer"
                );
            }
            !redundant
        })
        .map(|(_, p)| *p)
        .collect()
}
#[cfg(test)]
#[path = "test/profile.rs"]
mod tests;
