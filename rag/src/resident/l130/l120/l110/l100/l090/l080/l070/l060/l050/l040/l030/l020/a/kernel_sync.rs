use std::collections::HashMap;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::atomic::{AtomicBool, AtomicU8, AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};
use arc_swap::ArcSwap;
use crossbeam_queue::SegQueue;
use parking_lot::Mutex;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::l010::device_lane_event::{
    DeviceHandle, Event, EventSignal, Lane,
};
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GapKind {
    BatchEnd,
    CommSlot,
}
pub type Change = Box<
    dyn FnOnce(&GapContext) -> Result<(), ChangeError> + Send + 'static,
>;
pub struct GapContext<'a> {
    pub lane: &'a Lane,
    pub device: &'a DeviceHandle,
    pub kind: GapKind,
    start: Instant,
    budget: Duration,
}
impl GapContext<'_> {
    pub fn remaining(&self) -> Duration {
        self.budget.saturating_sub(self.start.elapsed())
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChangeError {
    Failed(String),
    Cancelled,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SyncError {
    Closed,
    QueueFull,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChangeStatus {
    Pending,
    Applied,
    Failed(ChangeError),
    Cancelled,
}
const PHASE_PENDING: u8 = 0;
const PHASE_RUNNING: u8 = 1;
const PHASE_CANCELLED: u8 = 2;
struct Entry {
    phase: AtomicU8,
    change: Mutex<Option<Change>>,
    status: Mutex<ChangeStatus>,
    event: Event,
    signal: EventSignal,
    live: Arc<AtomicUsize>,
}
impl Entry {
    fn finish(&self, status: ChangeStatus) {
        *self.status.lock() = status;
        self.signal.signal();
    }
    fn try_start(&self) -> bool {
        let started = self
            .phase
            .compare_exchange(
                PHASE_PENDING,
                PHASE_RUNNING,
                Ordering::AcqRel,
                Ordering::Acquire,
            )
            .is_ok();
        if started {
            self.live.fetch_sub(1, Ordering::AcqRel);
        }
        started
    }
    fn try_cancel(&self) -> bool {
        let cancelled = self
            .phase
            .compare_exchange(
                PHASE_PENDING,
                PHASE_CANCELLED,
                Ordering::AcqRel,
                Ordering::Acquire,
            )
            .is_ok();
        if cancelled {
            self.live.fetch_sub(1, Ordering::AcqRel);
            self.change.lock().take();
            self.finish(ChangeStatus::Cancelled);
        }
        cancelled
    }
}
pub struct ChangeHandle {
    entry: Arc<Entry>,
}
impl ChangeHandle {
    pub fn status(&self) -> ChangeStatus {
        self.entry.status.lock().clone()
    }
    pub fn event(&self) -> &Event {
        &self.entry.event
    }
    pub fn cancel(&self) -> bool {
        self.entry.try_cancel()
    }
}
struct LaneState {
    queue: SegQueue<Arc<Entry>>,
    live: Arc<AtomicUsize>,
    open: AtomicBool,
}
impl LaneState {
    fn new() -> Self {
        Self {
            queue: SegQueue::new(),
            live: Arc::new(AtomicUsize::new(0)),
            open: AtomicBool::new(false),
        }
    }
}
type Registry = HashMap<usize, Arc<LaneState>>;
pub struct KernelSync {
    capacity: usize,
    lanes: ArcSwap<Registry>,
    register_guard: Mutex<()>,
    closed: AtomicBool,
}
fn lane_key(lane: &Lane) -> usize {
    lane as *const Lane as usize
}
impl KernelSync {
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            lanes: ArcSwap::from_pointee(HashMap::new()),
            register_guard: Mutex::new(()),
            closed: AtomicBool::new(false),
        }
    }
    fn lane_state(&self, key: usize) -> Option<Arc<LaneState>> {
        self.lanes.load().get(&key).cloned()
    }
    fn lane_state_or_insert(&self, key: usize) -> Arc<LaneState> {
        if let Some(state) = self.lane_state(key) {
            return state;
        }
        let _guard = self.register_guard.lock();
        if let Some(state) = self.lane_state(key) {
            return state;
        }
        let state = Arc::new(LaneState::new());
        let mut next: Registry = (**self.lanes.load()).clone();
        next.insert(key, state.clone());
        self.lanes.store(Arc::new(next));
        state
    }
    pub fn submit(
        &self,
        lane: &Lane,
        change: Change,
    ) -> Result<ChangeHandle, SyncError> {
        if self.closed.load(Ordering::Acquire) {
            return Err(SyncError::Closed);
        }
        let state = self.lane_state_or_insert(lane_key(lane));
        let reserved = state
            .live
            .try_update(
                Ordering::AcqRel,
                Ordering::Acquire,
                |n| { if n < self.capacity { Some(n + 1) } else { None } },
            );
        if reserved.is_err() {
            return Err(SyncError::QueueFull);
        }
        let (event, signal) = Event::cpu();
        let entry = Arc::new(Entry {
            phase: AtomicU8::new(PHASE_PENDING),
            change: Mutex::new(Some(change)),
            status: Mutex::new(ChangeStatus::Pending),
            event,
            signal,
            live: state.live.clone(),
        });
        state.queue.push(entry.clone());
        if self.closed.load(Ordering::Acquire) {
            entry.try_cancel();
            return Err(SyncError::Closed);
        }
        Ok(ChangeHandle { entry })
    }
    pub fn gap_begin(&self, lane: &Lane, kind: GapKind, budget: Duration) -> usize {
        let Some(state) = self.lane_state(lane_key(lane)) else {
            return 0;
        };
        if state.open.swap(true, Ordering::AcqRel) {
            return 0;
        }
        let start = Instant::now();
        let device = lane.device();
        let ctx = GapContext {
            lane,
            device: &device,
            kind,
            start,
            budget,
        };
        let mut done = 0usize;
        while start.elapsed() < budget {
            let Some(entry) = state.queue.pop() else {
                break;
            };
            if !entry.try_start() {
                continue;
            }
            let change = entry.change.lock().take();
            let status = match change {
                Some(change) => {
                    match catch_unwind(AssertUnwindSafe(|| change(&ctx))) {
                        Ok(Ok(())) => ChangeStatus::Applied,
                        Ok(Err(e)) => ChangeStatus::Failed(e),
                        Err(payload) => {
                            let msg = payload
                                .downcast_ref::<&str>()
                                .map(|s| s.to_string())
                                .or_else(|| payload.downcast_ref::<String>().cloned())
                                .unwrap_or_else(|| "panic in change".to_string());
                            ChangeStatus::Failed(ChangeError::Failed(msg))
                        }
                    }
                }
                None => {
                    ChangeStatus::Failed(
                        ChangeError::Failed("change missing".to_string()),
                    )
                }
            };
            entry.finish(status);
            done += 1;
        }
        done
    }
    pub fn gap_end(&self, lane: &Lane) {
        if let Some(state) = self.lane_state(lane_key(lane)) {
            state.open.store(false, Ordering::Release);
        }
    }
    pub fn pending(&self, lane: &Lane) -> usize {
        self.lane_state(lane_key(lane))
            .map(|s| s.live.load(Ordering::Acquire))
            .unwrap_or(0)
    }
    pub fn close(&self) {
        self.closed.store(true, Ordering::Release);
        let registry = self.lanes.load_full();
        for state in registry.values() {
            while let Some(entry) = state.queue.pop() {
                entry.try_cancel();
            }
        }
    }
}
#[cfg(test)]
#[path = "test/kernel_sync.rs"]
mod tests;
