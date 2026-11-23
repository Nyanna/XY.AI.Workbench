use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Instant;

/// Monotonic, non-decreasing time source in nanoseconds.
pub trait Clock: Send + Sync {
    fn now_ns(&self) -> u64;
}

/// Clock based on `Instant`; the epoch is the creation of the value.
#[derive(Debug, Clone, Copy)]
pub struct MonotonicClock {
    epoch: Instant,
}

impl MonotonicClock {
    pub fn new() -> Self {
        MonotonicClock { epoch: Instant::now() }
    }
}

impl Default for MonotonicClock {
    fn default() -> Self {
        MonotonicClock::new()
    }
}

impl Clock for MonotonicClock {
    fn now_ns(&self) -> u64 {
        u64::try_from(self.epoch.elapsed().as_nanos()).unwrap_or(u64::MAX)
    }
}

/// Tells whether productive tasks are waiting for the device.
pub trait PendingTasks: Send + Sync {
    fn productive_waiting(&self) -> bool;
}

#[derive(Debug, Clone, Copy, Default)]
pub struct NeverPending;

impl PendingTasks for NeverPending {
    fn productive_waiting(&self) -> bool {
        false
    }
}

/// Shared switch; clones share the state.
#[derive(Debug, Clone, Default)]
pub struct PendingFlag(Arc<AtomicBool>);

impl PendingFlag {
    pub fn new() -> Self {
        PendingFlag::default()
    }

    pub fn set(&self, waiting: bool) {
        self.0.store(waiting, Ordering::SeqCst);
    }

    pub fn get(&self) -> bool {
        self.0.load(Ordering::SeqCst)
    }
}

impl PendingTasks for PendingFlag {
    fn productive_waiting(&self) -> bool {
        self.get()
    }
}

#[derive(Clone)]
pub struct Hooks {
    pub clock: Arc<dyn Clock>,
    pub pending: Arc<dyn PendingTasks>,
}

impl Default for Hooks {
    fn default() -> Self {
        Hooks {
            clock: Arc::new(MonotonicClock::new()),
            pending: Arc::new(NeverPending),
        }
    }
}

#[cfg(test)]
#[path = "test/hooks.rs"]
mod tests;
