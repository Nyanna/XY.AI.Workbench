use std::sync::atomic::{AtomicU64, AtomicU8, AtomicUsize, Ordering};
use std::sync::Arc;
use crate::resident::op::PinCount;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResourceState {
    Uninit,
    Loading,
    Ready,
    Backoff,
    Evicting,
}
impl ResourceState {
    fn code(self) -> u8 {
        self as u8
    }
    fn from_code(v: u8) -> Self {
        match v {
            0 => Self::Uninit,
            1 => Self::Loading,
            2 => Self::Ready,
            3 => Self::Backoff,
            _ => Self::Evicting,
        }
    }
}
/// Lifecycle cell of one lazily initialised resource. Time is passed in by the caller.
pub struct ResourceSlot {
    state: AtomicU8,
    in_flight: AtomicUsize,
    pins: Arc<PinCount>,
    last_used_ms: AtomicU64,
    retry_at_ms: AtomicU64,
}
impl ResourceSlot {
    pub fn new(pins: Arc<PinCount>) -> Self {
        Self {
            state: AtomicU8::new(ResourceState::Uninit.code()),
            in_flight: AtomicUsize::new(0),
            pins,
            last_used_ms: AtomicU64::new(0),
            retry_at_ms: AtomicU64::new(0),
        }
    }
    pub fn state(&self) -> ResourceState {
        ResourceState::from_code(self.state.load(Ordering::Acquire))
    }
    fn transition(&self, from: ResourceState, to: ResourceState) -> bool {
        self.state
            .compare_exchange(from.code(), to.code(), Ordering::SeqCst, Ordering::SeqCst)
            .is_ok()
    }
    /// Only the caller that wins this transition performs the load.
    pub fn begin_load(&self, now_ms: u64) -> bool {
        if self.transition(ResourceState::Uninit, ResourceState::Loading) {
            return true;
        }
        self.retry_at_ms.load(Ordering::Acquire) <= now_ms
            && self.transition(ResourceState::Backoff, ResourceState::Loading)
    }
    pub fn finish_load(&self, ok: bool, now_ms: u64, backoff_ms: u64) {
        self.last_used_ms.store(now_ms, Ordering::Release);
        if ok {
            self.state.store(ResourceState::Ready.code(), Ordering::SeqCst);
        } else {
            self.retry_at_ms.store(now_ms + backoff_ms, Ordering::Release);
            self.state.store(ResourceState::Backoff.code(), Ordering::SeqCst);
        }
    }
    /// Registers an operation in flight; fails unless the resource is `Ready`.
    pub fn enter(&self) -> bool {
        self.in_flight.fetch_add(1, Ordering::SeqCst);
        if self.state() == ResourceState::Ready {
            return true;
        }
        self.in_flight.fetch_sub(1, Ordering::SeqCst);
        false
    }
    pub fn leave(&self, now_ms: u64) {
        self.last_used_ms.store(now_ms, Ordering::Release);
        let prev = self.in_flight.fetch_sub(1, Ordering::SeqCst);
        debug_assert!(prev > 0, "leave without enter");
    }
    pub fn in_flight(&self) -> usize {
        self.in_flight.load(Ordering::Acquire)
    }
    /// `Ready -> Evicting` only when idle, unleased and past the TTL.
    pub fn try_evict(&self, now_ms: u64, ttl_ms: u64) -> bool {
        let idle_for = now_ms.saturating_sub(self.last_used_ms.load(Ordering::Acquire));
        if idle_for < ttl_ms
            || !self.transition(ResourceState::Ready, ResourceState::Evicting)
        {
            return false;
        }
        if self.in_flight.load(Ordering::SeqCst) == 0 && self.pins.is_unpinned() {
            return true;
        }
        self.state.store(ResourceState::Ready.code(), Ordering::SeqCst);
        false
    }
    pub fn finish_evict(&self) {
        debug_assert_eq!(self.state(), ResourceState::Evicting);
        self.state.store(ResourceState::Uninit.code(), Ordering::SeqCst);
    }
}
#[cfg(test)]
#[path = "test/resource.rs"]
mod tests;
