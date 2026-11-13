use std::collections::HashMap;
use std::fmt;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex, MutexGuard};
use std::time::Duration;

use super::super::l030::a::region_handle::RegionHandle;
use super::super::l030::l020::l010::device_lane_event::{Event, EventSignal};

pub type DeviceAddr = u64;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct BatchId(pub u64);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SegmentLayout {
    /// Bytes per entry.
    Fixed { stride: u64 },
    /// Table of `entries + 1` non-decreasing `u64` offsets relative to `base`, stored in this region.
    Variable { offsets: RegionHandle },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SegmentDesc {
    pub base: DeviceAddr,
    pub entries: u64,
    pub layout: SegmentLayout,
}

impl SegmentDesc {
    /// Fixed layout ignores `offsets`; variable layout expects the offset table contents.
    pub fn entry_of(&self, addr: DeviceAddr, offsets: &[u64]) -> Option<u64> {
        if self.entries == 0 || addr < self.base {
            return None;
        }
        let rel = addr - self.base;
        match self.layout {
            SegmentLayout::Fixed { stride } => {
                if stride == 0 {
                    return None;
                }
                let idx = rel / stride;
                (idx < self.entries).then_some(idx)
            }
            SegmentLayout::Variable { .. } => {
                if !self.offsets_valid(offsets) {
                    return None;
                }
                let end = *offsets.last()?;
                if rel >= end {
                    return None;
                }
                let covered = offsets.partition_point(|&o| o <= rel);
                let idx = covered.checked_sub(1)?;
                Some(idx as u64)
            }
        }
    }

    /// Variable layout: `offsets.len() == entries + 1` and non-decreasing. Fixed: always true.
    pub fn offsets_valid(&self, offsets: &[u64]) -> bool {
        match self.layout {
            SegmentLayout::Fixed { .. } => true,
            SegmentLayout::Variable { .. } => {
                let expected = match self.entries.checked_add(1) {
                    Some(n) => n,
                    None => return false,
                };
                offsets.len() as u64 == expected && offsets.windows(2).all(|w| w[0] <= w[1])
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BatchSpec {
    pub segments: Vec<SegmentDesc>,
    pub query_count: u32,
    pub params: Option<RegionHandle>,
    pub time_budget: Duration,
}

/// Read access to the progress counter in mapped host memory.
pub trait ProgressCounter: Send + Sync + fmt::Debug {
    /// Completed queries; non-blocking.
    fn load(&self) -> u64;
}

#[derive(Clone, Debug)]
pub enum BatchMode {
    Normal,
    Persistent { counter: Arc<dyn ProgressCounter> },
}

/// Write cursor inside a pre-assigned region; never allocates.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RangeCursor {
    region: RegionHandle,
    pos: u64,
}

impl RangeCursor {
    pub fn new(region: RegionHandle) -> RangeCursor {
        RangeCursor { region, pos: 0 }
    }

    pub fn region(&self) -> RegionHandle {
        self.region
    }

    pub fn used(&self) -> u64 {
        self.pos
    }

    pub fn remaining(&self) -> u64 {
        self.region.len().saturating_sub(self.pos)
    }

    /// Reserves `len` bytes and returns the offset relative to the region start.
    pub fn advance(&mut self, len: u64) -> Result<u64, RangeFull> {
        let full = RangeFull { requested: len, remaining: self.remaining() };
        match self.pos.checked_add(len) {
            Some(next) if next <= self.region.len() => {
                let start = self.pos;
                self.pos = next;
                Ok(start)
            }
            _ => Err(full),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RangeFull {
    pub requested: u64,
    pub remaining: u64,
}

impl fmt::Display for RangeFull {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "range full: requested {} bytes, {} remaining", self.requested, self.remaining)
    }
}

impl std::error::Error for RangeFull {}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UsedRange {
    pub region: RegionHandle,
    pub used: u64,
}

#[derive(Clone, Debug)]
pub struct BatchInputs {
    pub regions: Vec<RegionHandle>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LaunchOutcome {
    /// All queries finished.
    Hits { hits: u64 },
    /// The assigned ranges were not sufficient.
    Partial { queries_done: u32, hits: u64 },
}

pub type RunOutcome = LaunchOutcome;

/// Implemented by the backend; runs the kernel synchronously until completion or partial result.
pub trait BatchExecutor: Send + Sync {
    /// `queries_done` is the number of already completed queries (resume after partial).
    /// Embeddings are written only through `ranges`, in assignment order.
    fn launch(
        &self,
        id: BatchId,
        spec: &BatchSpec,
        inputs: &BatchInputs,
        queries_done: u32,
        ranges: &mut [RangeCursor],
    ) -> Result<LaunchOutcome, String>;
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BatchError {
    UnknownBatch(BatchId),
    InvalidSpec(String),
    /// A run is already in flight.
    Busy,
    /// No unused range is assigned.
    NeedsRanges,
    /// The batch already produced its complete result.
    AlreadyFinished,
    Executor(String),
}

impl fmt::Display for BatchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BatchError::UnknownBatch(id) => write!(f, "unknown batch {}", id.0),
            BatchError::InvalidSpec(msg) => write!(f, "invalid batch spec: {msg}"),
            BatchError::Busy => write!(f, "batch is running"),
            BatchError::NeedsRanges => write!(f, "batch needs new ranges"),
            BatchError::AlreadyFinished => write!(f, "batch already finished"),
            BatchError::Executor(msg) => write!(f, "executor failed: {msg}"),
        }
    }
}

impl std::error::Error for BatchError {}

struct BatchState {
    spec: Arc<BatchSpec>,
    mode: BatchMode,
    event: Option<Event>,
    signal: Option<EventSignal>,
    ranges: Vec<RangeCursor>,
    fresh_start: usize,
    queries_done: u32,
    reported: u64,
    running: bool,
    finished: bool,
}

pub struct BatchInterface<E: BatchExecutor> {
    executor: E,
    batches: Mutex<HashMap<BatchId, BatchState>>,
    next_id: AtomicU64,
}

impl<E: BatchExecutor> BatchInterface<E> {
    pub fn new(executor: E) -> Self {
        BatchInterface { executor, batches: Mutex::new(HashMap::new()), next_id: AtomicU64::new(1) }
    }

    fn lock(&self) -> MutexGuard<'_, HashMap<BatchId, BatchState>> {
        match self.batches.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        }
    }

    pub fn begin(&self, spec: BatchSpec, mode: BatchMode) -> Result<BatchId, BatchError> {
        if spec.query_count == 0 {
            return Err(BatchError::InvalidSpec("query_count must be greater than zero".to_string()));
        }
        let (event, signal) = match mode {
            BatchMode::Normal => {
                let (event, signal) = Event::cpu();
                (Some(event), Some(signal))
            }
            BatchMode::Persistent { .. } => (None, None),
        };
        let state = BatchState {
            spec: Arc::new(spec),
            mode,
            event,
            signal,
            ranges: Vec::new(),
            fresh_start: 0,
            queries_done: 0,
            reported: 0,
            running: false,
            finished: false,
        };
        let mut map = self.lock();
        let id = BatchId(self.next_id.fetch_add(1, Ordering::SeqCst));
        map.insert(id, state);
        Ok(id)
    }

    pub fn assign_ranges(&self, id: BatchId, ranges: &[RegionHandle]) -> Result<(), BatchError> {
        let mut map = self.lock();
        let state = map.get_mut(&id).ok_or(BatchError::UnknownBatch(id))?;
        if state.running {
            return Err(BatchError::Busy);
        }
        if state.finished {
            return Err(BatchError::AlreadyFinished);
        }
        state.ranges.extend(ranges.iter().copied().map(RangeCursor::new));
        Ok(())
    }

    pub fn run(&self, id: BatchId, inputs: &BatchInputs) -> Result<RunOutcome, BatchError> {
        let (spec, first, mut cursors, done) = {
            let mut map = self.lock();
            let state = map.get_mut(&id).ok_or(BatchError::UnknownBatch(id))?;
            if state.running {
                return Err(BatchError::Busy);
            }
            if state.finished {
                return Err(BatchError::AlreadyFinished);
            }
            if state.fresh_start >= state.ranges.len() {
                return Err(BatchError::NeedsRanges);
            }
            state.running = true;
            (
                Arc::clone(&state.spec),
                state.fresh_start,
                state.ranges[state.fresh_start..].to_vec(),
                state.queries_done,
            )
        };

        let result = catch_unwind(AssertUnwindSafe(|| {
            self.executor.launch(id, &spec, inputs, done, &mut cursors)
        }));
        let result = match result {
            Ok(r) => r,
            Err(_) => Err("executor panicked".to_string()),
        };

        let mut map = self.lock();
        let state = map.get_mut(&id).ok_or(BatchError::UnknownBatch(id))?;
        state.running = false;
        let outcome = result.map_err(BatchError::Executor)?;

        let end = first + cursors.len();
        state.ranges[first..end].copy_from_slice(&cursors);
        state.fresh_start = state.ranges.len();
        let total = state.spec.query_count;
        match outcome {
            LaunchOutcome::Hits { hits } => {
                state.finished = true;
                state.queries_done = total;
                state.reported = u64::from(total);
                if let Some(signal) = &state.signal {
                    signal.signal();
                }
                Ok(LaunchOutcome::Hits { hits })
            }
            LaunchOutcome::Partial { queries_done, hits } => {
                let clamped = queries_done.min(total).max(state.queries_done);
                state.queries_done = clamped;
                state.reported = state.reported.max(u64::from(clamped));
                Ok(LaunchOutcome::Partial { queries_done: clamped, hits })
            }
        }
    }

    pub fn progress(&self, id: BatchId) -> Result<u64, BatchError> {
        let mut map = self.lock();
        let state = map.get_mut(&id).ok_or(BatchError::UnknownBatch(id))?;
        let total = u64::from(state.spec.query_count);
        let current = match &state.mode {
            BatchMode::Normal => u64::from(state.queries_done),
            BatchMode::Persistent { counter } => counter.load(),
        };
        state.reported = state.reported.max(current.min(total)).min(total);
        Ok(state.reported)
    }

    /// Normal mode: the batch event. Persistent mode: `InvalidSpec`.
    pub fn event(&self, id: BatchId) -> Result<Event, BatchError> {
        let map = self.lock();
        let state = map.get(&id).ok_or(BatchError::UnknownBatch(id))?;
        state.event.clone().ok_or_else(|| {
            BatchError::InvalidSpec("persistent batches have no event".to_string())
        })
    }

    pub fn end(&self, id: BatchId) -> Result<Vec<UsedRange>, BatchError> {
        let mut map = self.lock();
        match map.get(&id) {
            None => return Err(BatchError::UnknownBatch(id)),
            Some(state) if state.running => return Err(BatchError::Busy),
            Some(_) => {}
        }
        let state = map.remove(&id).ok_or(BatchError::UnknownBatch(id))?;
        Ok(state
            .ranges
            .iter()
            .map(|c| UsedRange { region: c.region(), used: c.used() })
            .collect())
    }
}

#[cfg(test)]
#[path = "test/kernel_batch.rs"]
mod tests;
