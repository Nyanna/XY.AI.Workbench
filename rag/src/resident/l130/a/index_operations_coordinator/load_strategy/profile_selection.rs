use std::fmt;
use std::sync::atomic::{AtomicBool, Ordering};

use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::a::pareto_profiles::PrecisionProfile;

#[derive(Clone, Debug, PartialEq)]
pub enum SelectionError {
    /// Wish is NaN, or a profile position is NaN or outside [0,1].
    InvalidPrecision,
    /// No active profile (first failing task) or latch already set.
    NoActiveProfile,
}

impl fmt::Display for SelectionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SelectionError::InvalidPrecision => f.write_str("invalid precision"),
            SelectionError::NoActiveProfile => f.write_str("no active profile"),
        }
    }
}

impl std::error::Error for SelectionError {}

/// Clamps the wish to [0,1]; NaN is rejected, infinities are clamped.
pub fn clamp_wish(wish: f64) -> Result<f64, SelectionError> {
    if wish.is_nan() {
        return Err(SelectionError::InvalidPrecision);
    }
    Ok(if wish < 0.0 {
        0.0
    } else if wish > 1.0 {
        1.0
    } else {
        wish
    })
}

/// Per-subengine selector with a model-lock latch.
#[derive(Debug, Default)]
pub struct ProfileSelector {
    locked: AtomicBool,
}

impl ProfileSelector {
    pub fn new() -> Self {
        Self {
            locked: AtomicBool::new(false),
        }
    }

    pub fn select<'a>(
        &self,
        wish: f64,
        profiles: &'a [PrecisionProfile],
    ) -> Result<&'a PrecisionProfile, SelectionError> {
        let wish = clamp_wish(wish)?;
        if self.locked.load(Ordering::SeqCst) {
            return Err(SelectionError::NoActiveProfile);
        }
        if profiles.is_empty() {
            if !self.locked.swap(true, Ordering::SeqCst) {
                log::warn!("no active profile available; model locked for this subengine");
            }
            return Err(SelectionError::NoActiveProfile);
        }
        if profiles
            .iter()
            .any(|p| p.position.is_nan() || !(0.0..=1.0).contains(&p.position))
        {
            return Err(SelectionError::InvalidPrecision);
        }
        let mut best = &profiles[0];
        let mut best_dist = (best.position - wish).abs();
        for p in &profiles[1..] {
            let d = (p.position - wish).abs();
            if d < best_dist || (d == best_dist && p.position > best.position) {
                best = p;
                best_dist = d;
            }
        }
        Ok(best)
    }

    /// True iff at least one active profile exists.
    pub fn is_satisfiable(profiles: &[PrecisionProfile]) -> bool {
        !profiles.is_empty()
    }

    pub fn is_locked(&self) -> bool {
        self.locked.load(Ordering::SeqCst)
    }

    /// Releases the latch; returns true iff it was locked before.
    pub fn unlock(&self) -> bool {
        self.locked.swap(false, Ordering::SeqCst)
    }
}

#[cfg(test)]
#[path = "test/profile_selection.rs"]
mod tests;
