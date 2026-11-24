use std::fmt;
use std::sync::atomic::{AtomicU64, Ordering};
/// Totally ordered change stamp. `Stamp(0)` is never assigned to a write or delete.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Stamp(pub u64);
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
    pub fn resume(
        stored_max: Option<u64>,
        highest_present: Option<u64>,
    ) -> StampCounter {
        let base = stored_max.unwrap_or(0).max(highest_present.unwrap_or(0));
        StampCounter {
            last: AtomicU64::new(base),
        }
    }
    pub fn next(&self) -> Result<Stamp, StampError> {
        self.last
            .try_update(Ordering::AcqRel, Ordering::Acquire, |v| v.checked_add(1))
            .map(|prev| Stamp(prev + 1))
            .map_err(|_| StampError::Exhausted)
    }
    pub fn max(&self) -> Option<Stamp> {
        self.max_raw().map(Stamp)
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
