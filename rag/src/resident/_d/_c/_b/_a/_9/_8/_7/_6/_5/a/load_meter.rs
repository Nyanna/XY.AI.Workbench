use std::sync::{Mutex, MutexGuard, PoisonError};
use std::time::Duration;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::a::resource_requirements::ResourceKind;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::_1::device_lane_event::DeviceHandle;
pub const DEFAULT_ALPHA: f64 = 0.2;
const MIN_ALPHA: f64 = 0.01;
const MAX_ALPHA: f64 = 1.0;
/// Snapshot of the load of one sub-engine instance.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LoadReport {
    pub kind: ResourceKind,
    pub device: DeviceHandle,
    /// Entries the sub-engine can accept now.
    pub capacity: usize,
    /// Smoothed throughput in entries per second. Always finite and >= 0.0.
    pub throughput: f64,
}
/// Read side used by the work allocation.
pub trait LoadReporting: Send + Sync {
    fn report(&self) -> LoadReport;
}
#[derive(Clone, Copy, Debug)]
struct State4 {
    capacity: usize,
    throughput: f64,
    seeded: bool,
}
/// Tracks capacity and an exponentially smoothed throughput.
#[derive(Debug)]
pub struct LoadMeter {
    kind: ResourceKind,
    device: DeviceHandle,
    alpha: f64,
    state: Mutex<State4>,
}
impl LoadMeter {
    pub fn new(kind: ResourceKind, device: DeviceHandle, capacity: usize) -> Self {
        Self::with_alpha(kind, device, capacity, DEFAULT_ALPHA)
    }
    /// `alpha` is clamped to [0.01, 1.0]; NaN becomes `DEFAULT_ALPHA`.
    pub fn with_alpha(
        kind: ResourceKind,
        device: DeviceHandle,
        capacity: usize,
        alpha: f64,
    ) -> Self {
        let alpha = if alpha.is_nan() {
            DEFAULT_ALPHA
        } else {
            alpha.clamp(MIN_ALPHA, MAX_ALPHA)
        };
        Self {
            kind,
            device,
            alpha,
            state: Mutex::new(State4 {
                capacity,
                throughput: 0.0,
                seeded: false,
            }),
        }
    }
    pub fn kind(&self) -> ResourceKind {
        self.kind
    }
    pub fn device(&self) -> DeviceHandle {
        self.device
    }
    pub fn set_capacity(&self, capacity: usize) {
        self.lock().capacity = capacity;
    }
    pub fn capacity(&self) -> usize {
        self.lock().capacity
    }
    /// Records that `entries` were processed in `elapsed`.
    pub fn record(&self, entries: usize, elapsed: Duration) {
        if elapsed.is_zero() {
            return;
        }
        let sample = entries as f64 / elapsed.as_secs_f64();
        if !sample.is_finite() {
            return;
        }
        let mut state = self.lock();
        let next = if state.seeded {
            self.alpha * sample + (1.0 - self.alpha) * state.throughput
        } else {
            sample
        };
        if !next.is_finite() {
            return;
        }
        state.throughput = next.max(0.0);
        state.seeded = true;
    }
    pub fn throughput(&self) -> f64 {
        self.lock().throughput
    }
    fn lock(&self) -> MutexGuard<'_, State4> {
        self.state.lock().unwrap_or_else(PoisonError::into_inner)
    }
}
impl LoadReporting for LoadMeter {
    fn report(&self) -> LoadReport {
        let state = self.lock();
        LoadReport {
            kind: self.kind,
            device: self.device,
            capacity: state.capacity,
            throughput: state.throughput,
        }
    }
}
#[cfg(test)]
#[path = "test/load_meter.rs"]
mod tests;
