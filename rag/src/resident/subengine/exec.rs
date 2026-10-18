//! Execution contexts: lanes, events, and per-lane parameter and scratch storage.
//!
//! A lane is an in-order execution queue. On the CPU it is a dedicated worker thread; on CUDA it
//! is a prioritised stream. Cross-lane ordering is expressed only through events.

use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::Arc;
use std::thread::JoinHandle;

use parking_lot::{Condvar, Mutex};

use crate::resident::memory::{DeviceKind, Region};
use crate::resident::subengine::DeviceError;

/// Work item executed on a lane. Device work is enqueued asynchronously from within the job.
pub type LaneJob = Box<dyn FnOnce() -> Result<(), DeviceError> + Send + 'static>;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum LaneClass {
    Query,
    Index,
    Copy,
}

impl LaneClass {
    /// CUDA stream priority; lower values are more urgent and `0` is the default. Priorities only
    /// influence block scheduling and never preempt running blocks.
    pub fn priority(self) -> i32 {
        match self {
            LaneClass::Query => -5,
            LaneClass::Copy => -1,
            LaneClass::Index => 0,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct LaneConfig {
    pub param_capacity: usize,
    /// Arena span the lane carves temporary allocations from.
    pub scratch: Region,
}

impl LaneConfig {
    pub const DEFAULT_PARAM_CAPACITY: usize = 4096;

    pub fn new(scratch: Region) -> Self {
        Self {
            param_capacity: Self::DEFAULT_PARAM_CAPACITY,
            scratch,
        }
    }
}

#[derive(Default)]
pub(crate) struct CpuEvent {
    done: Mutex<bool>,
    cv: Condvar,
}

impl CpuEvent {
    fn signal(&self) {
        *self.done.lock() = true;
        self.cv.notify_all();
    }

    fn wait(&self) {
        let mut done = self.done.lock();
        while !*done {
            self.cv.wait(&mut done);
        }
    }

    fn is_done(&self) -> bool {
        *self.done.lock()
    }
}

pub(crate) enum EventRepr {
    Cpu(CpuEvent),
    #[cfg(feature = "cuda")]
    Cuda(cudarc::driver::CudaEvent),
}

/// Opaque completion marker for all work submitted to a lane before it was recorded.
#[derive(Clone)]
pub struct EventHandle {
    repr: Arc<EventRepr>,
}

impl std::fmt::Debug for EventHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EventHandle")
            .field("complete", &self.is_complete())
            .finish()
    }
}

impl EventHandle {
    pub(crate) fn from_repr(repr: EventRepr) -> Self {
        Self {
            repr: Arc::new(repr),
        }
    }

    pub(crate) fn repr(&self) -> &EventRepr {
        &self.repr
    }

    pub fn is_complete(&self) -> bool {
        match &*self.repr {
            EventRepr::Cpu(e) => e.is_done(),
            #[cfg(feature = "cuda")]
            EventRepr::Cuda(e) => e.is_complete(),
        }
    }

    /// Blocks the calling thread until the event completes.
    pub fn synchronize(&self) -> Result<(), DeviceError> {
        match &*self.repr {
            EventRepr::Cpu(e) => {
                e.wait();
                Ok(())
            }
            #[cfg(feature = "cuda")]
            EventRepr::Cuda(e) => e.synchronize().map_err(|e| DeviceError::EventRecord {
                reason: e.to_string(),
            }),
        }
    }
}

pub trait ParamValue: Copy {
    const SIZE: usize;
    fn write(self, out: &mut Vec<u8>);
}

macro_rules! param_value {
    ($($t:ty),*) => {$(
        impl ParamValue for $t {
            const SIZE: usize = std::mem::size_of::<$t>();
            fn write(self, out: &mut Vec<u8>) {
                out.extend_from_slice(&self.to_ne_bytes());
            }
        }
    )*};
}

param_value!(u8, u16, u32, u64, i32, i64, f32);

/// Fixed-capacity buffer for dynamic kernel parameters, staged on the host and mirrored into a
/// small device region bound by the memory manager.
#[derive(Debug)]
pub struct ParamBlock {
    capacity: usize,
    buf: Vec<u8>,
    region: Option<Region>,
}

impl ParamBlock {
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            buf: Vec::with_capacity(capacity),
            region: None,
        }
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn bind(&mut self, region: Region) {
        self.region = Some(region);
    }

    pub fn region(&self) -> Option<Region> {
        self.region
    }

    pub fn clear(&mut self) {
        self.buf.clear();
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.buf
    }

    /// Appends `value` naturally aligned and returns its byte offset.
    pub fn push<T: ParamValue>(&mut self, value: T) -> Result<usize, DeviceError> {
        let offset = self.buf.len().next_multiple_of(T::SIZE);
        let end = offset + T::SIZE;
        if end > self.capacity {
            return Err(DeviceError::ParamOverflow {
                capacity: self.capacity,
                requested: end,
            });
        }
        self.buf.resize(offset, 0);
        value.write(&mut self.buf);
        Ok(offset)
    }
}

/// Bump allocator over a lane-private arena span; reset once the lane has drained.
#[derive(Debug)]
pub struct ScratchArena {
    base: Region,
    cursor: Mutex<u64>,
}

impl ScratchArena {
    pub fn new(base: Region) -> Self {
        Self {
            base,
            cursor: Mutex::new(0),
        }
    }

    pub fn capacity(&self) -> u64 {
        self.base.len
    }

    pub fn used(&self) -> u64 {
        *self.cursor.lock()
    }

    pub fn alloc(&self, bytes: u64, align: u64) -> Result<Region, DeviceError> {
        if align == 0 || !align.is_power_of_two() {
            return Err(DeviceError::InvalidAlign { align });
        }
        let mut cursor = self.cursor.lock();
        let start = (self.base.offset + *cursor).next_multiple_of(align) - self.base.offset;
        let end = start.checked_add(bytes).filter(|e| *e <= self.base.len);
        let Some(end) = end else {
            return Err(DeviceError::ScratchExhausted {
                requested: bytes,
                available: self.base.len.saturating_sub(*cursor),
            });
        };
        *cursor = end;
        Ok(Region {
            arena: self.base.arena,
            offset: self.base.offset + start,
            len: bytes,
            gen: self.base.gen,
        })
    }

    /// Invalidates all regions handed out so far.
    pub fn reset(&self) {
        *self.cursor.lock() = 0;
    }
}

/// In-order execution queue of one device. Device handles, streams and pointers never leave the
/// implementation.
pub trait ExecContext: Send {
    fn class(&self) -> LaneClass;
    fn device(&self) -> DeviceKind;
    fn submit(&self, job: LaneJob) -> Result<(), DeviceError>;
    /// Completes once everything submitted so far has finished.
    fn record_event(&self) -> Result<EventHandle, DeviceError>;
    /// Orders all later work on this lane after `event`.
    fn wait(&self, event: &EventHandle) -> Result<(), DeviceError>;
    /// Blocks until the lane has drained; returns the first job failure since the last call.
    fn synchronize(&self) -> Result<(), DeviceError>;
    fn param_block(&mut self) -> &mut ParamBlock;
    fn scratch(&self) -> &ScratchArena;
}

enum Msg {
    Job(LaneJob),
    Signal(EventHandle),
    WaitFor(EventHandle),
}

/// CPU lane: one worker thread, so submission order is execution order.
pub struct CpuLane {
    class: LaneClass,
    tx: Option<Sender<Msg>>,
    worker: Option<JoinHandle<()>>,
    failure: Arc<Mutex<Option<DeviceError>>>,
    params: ParamBlock,
    scratch: ScratchArena,
}

impl CpuLane {
    pub fn new(class: LaneClass, cfg: LaneConfig) -> Result<Self, DeviceError> {
        let (tx, rx) = mpsc::channel();
        let failure = Arc::new(Mutex::new(None));
        let span = tracing::info_span!("lane", lane = ?class, device = "cpu");
        let worker = std::thread::Builder::new()
            .name(format!("lane-cpu-{class:?}").to_lowercase())
            .spawn({
                let failure = failure.clone();
                move || span.in_scope(|| run_worker(rx, &failure))
            })
            .map_err(|e| DeviceError::Init {
                reason: e.to_string(),
            })?;
        Ok(Self {
            class,
            tx: Some(tx),
            worker: Some(worker),
            failure,
            params: ParamBlock::new(cfg.param_capacity),
            scratch: ScratchArena::new(cfg.scratch),
        })
    }

    fn send(&self, msg: Msg) -> Result<(), DeviceError> {
        self.tx
            .as_ref()
            .ok_or(DeviceError::LaneClosed)?
            .send(msg)
            .map_err(|_| DeviceError::LaneClosed)
    }
}

fn run_worker(rx: Receiver<Msg>, failure: &Mutex<Option<DeviceError>>) {
    for msg in rx {
        match msg {
            Msg::Job(job) => {
                let result = catch_unwind(AssertUnwindSafe(job)).unwrap_or_else(|_| {
                    Err(DeviceError::JobFailed {
                        reason: "job panicked".into(),
                    })
                });
                if let Err(e) = result {
                    tracing::warn!(error = %e, "lane job failed");
                    failure.lock().get_or_insert(e);
                }
            }
            Msg::Signal(event) => {
                if let EventRepr::Cpu(e) = event.repr() {
                    e.signal();
                }
            }
            Msg::WaitFor(event) => {
                if let Err(e) = event.synchronize() {
                    failure.lock().get_or_insert(e);
                }
            }
        }
    }
}

impl ExecContext for CpuLane {
    fn class(&self) -> LaneClass {
        self.class
    }

    fn device(&self) -> DeviceKind {
        DeviceKind::Cpu
    }

    fn submit(&self, job: LaneJob) -> Result<(), DeviceError> {
        self.send(Msg::Job(job))
    }

    fn record_event(&self) -> Result<EventHandle, DeviceError> {
        let event = EventHandle::from_repr(EventRepr::Cpu(CpuEvent::default()));
        self.send(Msg::Signal(event.clone()))?;
        Ok(event)
    }

    fn wait(&self, event: &EventHandle) -> Result<(), DeviceError> {
        self.send(Msg::WaitFor(event.clone()))
    }

    fn synchronize(&self) -> Result<(), DeviceError> {
        self.record_event()?.synchronize()?;
        match self.failure.lock().take() {
            Some(e) => Err(e),
            None => Ok(()),
        }
    }

    fn param_block(&mut self) -> &mut ParamBlock {
        &mut self.params
    }

    fn scratch(&self) -> &ScratchArena {
        &self.scratch
    }
}

impl Drop for CpuLane {
    fn drop(&mut self) {
        self.tx.take();
        if let Some(worker) = self.worker.take() {
            let _ = worker.join();
        }
    }
}

/// Lane double for downstream tests: runs jobs inline and logs every call.
#[cfg(any(test, feature = "mock"))]
pub struct RecordingLane {
    class: LaneClass,
    calls: Arc<Mutex<Vec<&'static str>>>,
    params: ParamBlock,
    scratch: ScratchArena,
}

#[cfg(any(test, feature = "mock"))]
impl RecordingLane {
    pub fn new(class: LaneClass, cfg: LaneConfig) -> Self {
        Self {
            class,
            calls: Arc::default(),
            params: ParamBlock::new(cfg.param_capacity),
            scratch: ScratchArena::new(cfg.scratch),
        }
    }

    pub fn calls(&self) -> Vec<&'static str> {
        self.calls.lock().clone()
    }
}

#[cfg(any(test, feature = "mock"))]
impl ExecContext for RecordingLane {
    fn class(&self) -> LaneClass {
        self.class
    }

    fn device(&self) -> DeviceKind {
        DeviceKind::Cpu
    }

    fn submit(&self, job: LaneJob) -> Result<(), DeviceError> {
        self.calls.lock().push("submit");
        job()
    }

    fn record_event(&self) -> Result<EventHandle, DeviceError> {
        self.calls.lock().push("record");
        let event = CpuEvent::default();
        event.signal();
        Ok(EventHandle::from_repr(EventRepr::Cpu(event)))
    }

    fn wait(&self, _event: &EventHandle) -> Result<(), DeviceError> {
        self.calls.lock().push("wait");
        Ok(())
    }

    fn synchronize(&self) -> Result<(), DeviceError> {
        self.calls.lock().push("sync");
        Ok(())
    }

    fn param_block(&mut self) -> &mut ParamBlock {
        &mut self.params
    }

    fn scratch(&self) -> &ScratchArena {
        &self.scratch
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resident::memory::ArenaId;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::time::Duration;

    fn cfg() -> LaneConfig {
        LaneConfig::new(Region {
            arena: ArenaId(1),
            offset: 8,
            len: 256,
            gen: 3,
        })
    }

    fn lane(class: LaneClass) -> CpuLane {
        CpuLane::new(class, cfg()).unwrap()
    }

    #[test]
    fn jobs_run_in_submission_order() {
        let lane = lane(LaneClass::Query);
        let log = Arc::new(Mutex::new(Vec::new()));
        for i in 0..32 {
            let log = log.clone();
            lane.submit(Box::new(move || {
                log.lock().push(i);
                Ok(())
            }))
            .unwrap();
        }
        lane.synchronize().unwrap();
        assert_eq!(*log.lock(), (0..32).collect::<Vec<_>>());
    }

    #[test]
    fn event_completes_after_prior_jobs() {
        let lane = lane(LaneClass::Index);
        let hits = Arc::new(AtomicUsize::new(0));
        let h = hits.clone();
        lane.submit(Box::new(move || {
            std::thread::sleep(Duration::from_millis(20));
            h.fetch_add(1, Ordering::SeqCst);
            Ok(())
        }))
        .unwrap();
        let event = lane.record_event().unwrap();
        event.synchronize().unwrap();
        assert!(event.is_complete());
        assert_eq!(hits.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn wait_orders_work_across_lanes() {
        let producer = lane(LaneClass::Copy);
        let consumer = lane(LaneClass::Query);
        let value = Arc::new(AtomicUsize::new(0));
        let seen = Arc::new(AtomicUsize::new(usize::MAX));

        let v = value.clone();
        producer
            .submit(Box::new(move || {
                std::thread::sleep(Duration::from_millis(30));
                v.store(7, Ordering::SeqCst);
                Ok(())
            }))
            .unwrap();
        let event = producer.record_event().unwrap();

        consumer.wait(&event).unwrap();
        let (v, s) = (value.clone(), seen.clone());
        consumer
            .submit(Box::new(move || {
                s.store(v.load(Ordering::SeqCst), Ordering::SeqCst);
                Ok(())
            }))
            .unwrap();
        consumer.synchronize().unwrap();
        assert_eq!(seen.load(Ordering::SeqCst), 7);
    }

    #[test]
    fn first_failure_is_reported_once() {
        let lane = lane(LaneClass::Index);
        lane.submit(Box::new(|| Err(DeviceError::LaneClosed))).unwrap();
        lane.submit(Box::new(|| panic!("boom"))).unwrap();
        assert!(matches!(lane.synchronize(), Err(DeviceError::LaneClosed)));
        assert!(lane.synchronize().is_ok());
    }

    #[test]
    fn drop_drains_pending_jobs() {
        let hits = Arc::new(AtomicUsize::new(0));
        {
            let lane = lane(LaneClass::Index);
            let h = hits.clone();
            lane.submit(Box::new(move || {
                h.fetch_add(1, Ordering::SeqCst);
                Ok(())
            }))
            .unwrap();
        }
        assert_eq!(hits.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn priorities_rank_query_above_copy_above_index() {
        assert!(LaneClass::Query.priority() < LaneClass::Copy.priority());
        assert!(LaneClass::Copy.priority() < LaneClass::Index.priority());
    }

    #[test]
    fn param_block_aligns_and_bounds() {
        let mut p = ParamBlock::new(16);
        assert_eq!(p.push(1u8).unwrap(), 0);
        assert_eq!(p.push(2u32).unwrap(), 4);
        assert_eq!(p.push(3u64).unwrap(), 8);
        assert_eq!(p.as_bytes().len(), 16);
        assert!(matches!(
            p.push(1u8),
            Err(DeviceError::ParamOverflow { capacity: 16, .. })
        ));
        p.clear();
        assert_eq!(p.push(1.5f32).unwrap(), 0);
    }

    #[test]
    fn scratch_respects_alignment_bounds_and_reset() {
        let s = ScratchArena::new(cfg().scratch);
        let a = s.alloc(10, 16).unwrap();
        assert_eq!(a.offset % 16, 0);
        assert_eq!((a.arena, a.gen), (ArenaId(1), 3));
        let b = s.alloc(10, 1).unwrap();
        assert!(b.offset >= a.offset + a.len);
        assert!(matches!(
            s.alloc(1024, 1),
            Err(DeviceError::ScratchExhausted { .. })
        ));
        assert!(matches!(
            s.alloc(1, 3),
            Err(DeviceError::InvalidAlign { align: 3 })
        ));
        s.reset();
        assert_eq!(s.used(), 0);
        assert!(s.alloc(256 - 8, 1).is_ok());
    }

    #[test]
    fn recording_lane_logs_calls() {
        let mut lane = RecordingLane::new(LaneClass::Query, cfg());
        lane.submit(Box::new(|| Ok(()))).unwrap();
        let e = lane.record_event().unwrap();
        lane.wait(&e).unwrap();
        lane.synchronize().unwrap();
        lane.param_block().push(1u32).unwrap();
        assert_eq!(lane.calls(), vec!["submit", "record", "wait", "sync"]);
    }

    #[test]
    fn lanes_work_as_trait_objects() {
        let mut lanes: Vec<Box<dyn ExecContext>> = vec![
            Box::new(lane(LaneClass::Query)),
            Box::new(RecordingLane::new(LaneClass::Copy, cfg())),
        ];
        for l in &mut lanes {
            l.param_block().push(1u32).unwrap();
            l.synchronize().unwrap();
        }
    }
}
