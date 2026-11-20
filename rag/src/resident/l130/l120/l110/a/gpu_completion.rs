use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::{Arc, Mutex, MutexGuard};
use crate::resident::l130::l120::l110::l100::l090::l080::l070::a::notification_topology::SubengineNotifier;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::l010::device_lane_event::{
    Event, EventError, Lane,
};
const PENDING: u8 = 0;
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
struct Slot {
    state: AtomicU8,
}
struct Entry {
    slot: Arc<Slot>,
    lane: Lane,
    event: Event,
    tag: u64,
}
/// Callback to be run at the end of the stream. Single use. `submit` already registers one with
/// the lane's stream if the stream supports it; firing this one as well is harmless.
pub struct CompletionCallback {
    slot: Arc<Slot>,
    notifier: SubengineNotifier,
}
impl CompletionCallback {
    /// Records the completion entry and raises `notify_result()`.
    /// Calls no GPU API, takes no lock and does not allocate.
    pub fn fire(self) {
        let _ = self
            .slot
            .state
            .compare_exchange(PENDING, DONE, Ordering::AcqRel, Ordering::Acquire);
        let _ = catch_unwind(AssertUnwindSafe(|| self.notifier.notify_result()));
    }
}
fn lock<T>(m: &Mutex<T>) -> MutexGuard<'_, T> {
    m.lock().unwrap_or_else(|p| p.into_inner())
}
/// Tracker of in-flight lanes.
pub struct GpuCompletion {
    entries: Mutex<Vec<Entry>>,
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
        let slot = Arc::new(Slot {
            state: AtomicU8::new(PENDING),
        });
        let mut entries = lock(&self.entries);
        entries
            .push(Entry {
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
                PENDING => {
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
