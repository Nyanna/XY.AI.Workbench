//! Maps a task's precision wish onto the nearest active profile of one subengine.
//!
//! Scale: `0.0` is fastest, `1.0` is most precise. The scale is relative to the
//! subengine: `0.5` denotes different accuracy levels on CPU and GPU.

use std::collections::HashSet;
use std::sync::Mutex;

use crate::resident::error::PrecisionError;

use super::profile::Profile;

/// Returns the profile with minimal `|t - position|`, where `t` is `precision`
/// clamped to `[0, 1]`. Equal distances resolve to the more precise profile.
///
/// `NaN` is rejected before clamping (`f32::clamp` would pass it through).
pub fn match_profile(precision: f32, profiles: &[Profile]) -> Result<&Profile, PrecisionError> {
    if precision.is_nan() {
        return Err(PrecisionError::InvalidPrecision {
            requested: precision,
        });
    }
    let t = f64::from(precision.clamp(0.0, 1.0));
    profiles
        .iter()
        .min_by(|a, b| {
            let da = (t - f64::from(a.position)).abs();
            let db = (t - f64::from(b.position)).abs();
            da.total_cmp(&db)
                .then(b.position.total_cmp(&a.position))
        })
        .ok_or(PrecisionError::NoActiveProfile)
}

/// Per-subengine record of models without any active profile.
///
/// The first task for such a model fails with `NoActiveProfile`; later tasks
/// fail immediately without repeating the profile lookup.
#[derive(Debug, Default)]
pub struct ProfileSelector {
    blocked: Mutex<HashSet<String>>,
}

impl ProfileSelector {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_blocked(&self, model_id: &str) -> bool {
        self.lock().contains(model_id)
    }

    /// Lifts the block, e.g. after a successful recalibration.
    pub fn unblock(&self, model_id: &str) -> bool {
        self.lock().remove(model_id)
    }

    pub fn select<'a>(
        &self,
        model_id: &str,
        precision: f32,
        profiles: &'a [Profile],
    ) -> Result<&'a Profile, PrecisionError> {
        if self.is_blocked(model_id) {
            return Err(PrecisionError::NoActiveProfile);
        }
        match match_profile(precision, profiles) {
            Ok(profile) => {
                tracing::debug!(
                    model_id,
                    requested = precision,
                    chosen = profile.position,
                    "profile chosen"
                );
                Ok(profile)
            }
            Err(PrecisionError::NoActiveProfile) => {
                self.lock().insert(model_id.to_owned());
                tracing::warn!(model_id, "no active profile, model blocked");
                Err(PrecisionError::NoActiveProfile)
            }
            Err(e) => Err(e),
        }
    }

    fn lock(&self) -> std::sync::MutexGuard<'_, HashSet<String>> {
        self.blocked.lock().unwrap_or_else(|e| e.into_inner())
    }
}

#[cfg(test)]
#[path = "test/matching.rs"]
mod tests;
