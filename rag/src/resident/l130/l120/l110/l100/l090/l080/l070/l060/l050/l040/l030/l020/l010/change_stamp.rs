use std::fmt;
use std::sync::atomic::{AtomicU64, Ordering};

/// Opaque change marker. Only equality is semantically meaningful.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct ChangeStamp(u64);

impl ChangeStamp {
    /// Raw representation for persistence and notification.
    pub fn as_raw(&self) -> u64 {
        self.0
    }

    /// Rebuilds a stamp from a raw value without checking that it was issued.
    pub fn from_raw(raw: u64) -> ChangeStamp {
        ChangeStamp(raw)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum StampError {
    /// The counter has already issued `u64::MAX`.
    Exhausted,
}

impl fmt::Display for StampError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StampError::Exhausted => write!(f, "change stamp counter exhausted"),
        }
    }
}

impl std::error::Error for StampError {}

/// Per-index stamp counter holding the last issued raw value (0 = empty).
#[derive(Debug)]
pub struct StampCounter {
    last: AtomicU64,
}

impl StampCounter {
    pub fn new() -> StampCounter {
        StampCounter {
            last: AtomicU64::new(0),
        }
    }

    /// Continues above `max(stored_max, highest_present)`.
    pub fn resume(stored_max: Option<u64>, highest_present: Option<u64>) -> StampCounter {
        let base = stored_max.unwrap_or(0).max(highest_present.unwrap_or(0));
        StampCounter {
            last: AtomicU64::new(base),
        }
    }

    pub fn next(&self) -> Result<ChangeStamp, StampError> {
        self.last
            .fetch_update(Ordering::AcqRel, Ordering::Acquire, |v| v.checked_add(1))
            .map(|prev| ChangeStamp(prev + 1))
            .map_err(|_| StampError::Exhausted)
    }

    pub fn max(&self) -> Option<ChangeStamp> {
        self.max_raw().map(ChangeStamp)
    }

    pub fn max_raw(&self) -> Option<u64> {
        match self.last.load(Ordering::Acquire) {
            0 => None,
            v => Some(v),
        }
    }
}

impl Default for StampCounter {
    fn default() -> Self {
        StampCounter::new()
    }
}

#[cfg(test)]
#[path = "test/change_stamp.rs"]
mod tests;
