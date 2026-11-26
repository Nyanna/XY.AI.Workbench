use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::{Arc, Mutex};
use crate::resident::_d::_c::_b::_a::_9::_8::_7::a::notification_topology::SubengineNotifier;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::_1::device_lane_event::{
    Event, EventError, Lane,
};
const PENDING3: u8 = 0;
const DONE: u8 = 1;
const TAKEN: u8 = 2;
/// A lane that finished.
pub struct FinishedLane {
    pub lane: Lane,
    pub event: Event,
    /// Caller tag given to `submit`.
    pub tag: u64,
    /// `Ok(())`: work completed. `Err`: the event query reported a failure.
    pub result: Result<(), EventError>,
    /// Which path handed the lane over.
    pub via: TakenVia,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TakenVia {
    Callback,
    EventQuery,
}
/// Per-submission state shared between the tracker and the callback.
/// It holds only the atomic state so the callback never owns GPU resources.
struct Slot10 {
    state: AtomicU8,
}
struct Entry8 {
    slot: Arc<Slot10>,
    lane: Lane,
    event: Event,
    tag: u64,
}
/// Callback to be run at the end of the stream. Single use. `submit` already registers one with
/// the lane's stream if the stream supports it; firing this one as well is harmless.
pub struct CompletionCallback {
    slot: Arc<Slot10>,
    notifier: SubengineNotifier,
}
impl CompletionCallback {
    /// Records the completion entry and raises `notify_result()`.
    /// Calls no GPU API, takes no lock and does not allocate.
    pub fn fire(self) {
        let _ = self
            .slot
            .state
            .compare_exchange(PENDING3, DONE, Ordering::AcqRel, Ordering::Acquire);
        let _ = catch_unwind(AssertUnwindSafe(|| self.notifier.notify_result()));
    }
}
/// Tracker of in-flight lanes.
pub struct GpuCompletion {
    entries: Mutex<Vec<Entry8>>,
    notifier: SubengineNotifier,
}
impl GpuCompletion {
    pub fn new(notifier: SubengineNotifier) -> Self {
        Self {
            entries: Mutex::new(Vec::new()),
            notifier,
        }
    }
    /// Puts an in-flight lane under tracking. `event` is the event recorded on
    /// `lane` after the submitted work.
    pub fn submit(&self, lane: Lane, event: Event, tag: u64) -> CompletionCallback {
        let slot = Arc::new(Slot10 {
            state: AtomicU8::new(PENDING3),
        });
        let mut entries = lock(&self.entries);
        entries
            .push(Entry8 {
                slot: Arc::clone(&slot),
                lane,
                event,
                tag,
            });
        let registered = CompletionCallback {
            slot: Arc::clone(&slot),
            notifier: self.notifier.clone(),
        };
        if let Some(stream) = entries.last().and_then(|e| e.lane.stream()) {
            if let Err(cause) = stream
                .notify_on_complete(Box::new(move || registered.fire()))
            {
                log::warn!(
                    "gpu completion: stream notification failed (tag {tag}): {cause}"
                );
            }
        }
        drop(entries);
        CompletionCallback {
            slot,
            notifier: self.notifier.clone(),
        }
    }
    /// Takes all lanes that are finished by callback entry or by event query.
    /// Non-blocking.
    pub fn poll(&self) -> Vec<FinishedLane> {
        let mut finished = Vec::new();
        let mut entries = lock(&self.entries);
        let mut i = 0;
        while i < entries.len() {
            let entry = &entries[i];
            let queried = match entry.slot.state.load(Ordering::Acquire) {
                DONE => Ok(()),
                PENDING3 => {
                    match entry.event.is_complete() {
                        Ok(true) => Ok(()),
                        Ok(false) => {
                            i += 1;
                            continue;
                        }
                        Err(e) => Err(e),
                    }
                }
                _ => {
                    i += 1;
                    continue;
                }
            };
            let previous = entry.slot.state.swap(TAKEN, Ordering::AcqRel);
            if previous == TAKEN {
                i += 1;
                continue;
            }
            let entry = entries.remove(i);
            let (via, result) = if previous == DONE {
                (TakenVia::Callback, Ok(()))
            } else {
                (TakenVia::EventQuery, queried)
            };
            match &result {
                Err(e) => {
                    log::warn!(
                        "gpu completion: event query failed (tag {}): {}", entry.tag, e
                    )
                }
                Ok(()) if via == TakenVia::EventQuery => {
                    log::debug!("gpu completion: tag {} found by event query", entry.tag)
                }
                Ok(()) => {}
            }
            finished
                .push(FinishedLane {
                    lane: entry.lane,
                    event: entry.event,
                    tag: entry.tag,
                    result,
                    via,
                });
        }
        finished
    }
    /// Number of tracked lanes not yet taken.
    pub fn in_flight(&self) -> usize {
        lock(&self.entries).len()
    }
}
#[cfg(test)]
#[path = "test/gpu_completion.rs"]
mod tests;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::_1::sync_util::lock;
