use std::collections::{HashMap, VecDeque};
use std::fmt;
use std::ops::Range;
use std::sync::atomic::{AtomicU32, AtomicU64, Ordering};
use std::sync::{Arc, Mutex, MutexGuard, RwLock, RwLockReadGuard, RwLockWriteGuard};
use log::warn;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::a::slot_layout::{
    EntryRef, SegmentView, SlotAssignment, SlotError, SlotStates,
};
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::a::kernel_layout::{
    Addr, Slot,
};
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::l010::device_lane_event::{
    Event, EventError,
};
const MAX_TOKENS: u32 = 65_535;
const RESERVATION_TTL: Duration = Duration::from_secs(120);
const SWEEP_INTERVAL: Duration = Duration::from_secs(10);
const NO_CENTROID: u32 = u32::MAX;
static NEXT_WRITER_ID: AtomicU64 = AtomicU64::new(1);
/// Centroid assignment of a slot. `u32::MAX` is reserved as the internal "none" marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CentroidId(pub u32);
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WriteError {
    /// `tokens` exceeds 65_535.
    TokensOutOfRange { tokens: u32 },
    /// `tokens` exceeds the class limit of the view.
    TokensExceedClass { tokens: u32, max: u32 },
    /// The cursor reached `slot_count`.
    NoFreeSlot,
    /// Transfer event not complete yet (`commit` only); the reservation stays valid.
    NotReady,
    /// The reservation belongs to another writer.
    ForeignReservation,
    /// The slot is not in the reserved state (committed, queued or discarded).
    NotReserved { slot: Slot },
    /// `CentroidId` equals `u32::MAX`.
    InvalidCentroid,
    /// Rebuild refused: reserved or queued commits are still open.
    Busy,
    /// Rebuild target has a different class/dim/dtype or too few slots.
    RebuildTarget,
    /// Event failure during `commit`; the reservation stays valid.
    Event(EventError),
    Slot(SlotError),
}
impl fmt::Display for WriteError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WriteError::TokensOutOfRange { tokens } => {
                write!(f, "token count {tokens} exceeds {MAX_TOKENS}")
            }
            WriteError::TokensExceedClass { tokens, max } => {
                write!(f, "token count {tokens} exceeds class limit {max}")
            }
            WriteError::NoFreeSlot => f.write_str("no free slot left in the segment"),
            WriteError::NotReady => f.write_str("transfer event is not complete"),
            WriteError::ForeignReservation => {
                f.write_str("reservation belongs to another writer")
            }
            WriteError::NotReserved { slot } => {
                write!(f, "slot {} is not in the reserved state", slot.0)
            }
            WriteError::InvalidCentroid => {
                f.write_str("centroid id u32::MAX is reserved")
            }
            WriteError::Busy => f.write_str("open reservations or queued commits"),
            WriteError::RebuildTarget => f.write_str("rebuild target does not fit"),
            WriteError::Event(e) => write!(f, "event error: {e}"),
            WriteError::Slot(e) => write!(f, "slot error: {e}"),
        }
    }
}
impl std::error::Error for WriteError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            WriteError::Event(e) => Some(e),
            WriteError::Slot(e) => Some(e),
            _ => None,
        }
    }
}
impl From<SlotError> for WriteError {
    fn from(e: SlotError) -> Self {
        WriteError::Slot(e)
    }
}
/// Pre-assigned slot. Invisible until committed.
#[derive(Debug)]
pub struct Reservation {
    pub slot: Slot,
    pub entry: EntryRef,
    pub tokens: u32,
    pub centroid: Option<CentroidId>,
    /// Device address of the slot start.
    pub addr: Addr,
    /// Absolute byte range of the slot in the arena offset space.
    pub target: Range<u64>,
    writer_id: u64,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SlotInfo {
    pub valid: bool,
    pub tokens: u32,
    /// 0 while never committed; otherwise the running write number (>= 1).
    pub write_no: u64,
    pub centroid: Option<CentroidId>,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SlotMove {
    pub from: Slot,
    pub to: Slot,
    pub entry: EntryRef,
    pub tokens: u32,
    pub centroid: Option<CentroidId>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompactionPlan {
    pub moves: Vec<SlotMove>,
    pub reclaimed_slots: u32,
}
#[derive(Clone, Copy)]
struct PendingSlot {
    entry: EntryRef,
    tokens: u32,
    centroid: Option<CentroidId>,
    queued: bool,
    reserved_at: Instant,
}
struct WriterState {
    cursor: u32,
    next_write_no: u64,
    pending: HashMap<u32, PendingSlot>,
    queue: VecDeque<(Slot, Event)>,
    last_sweep: Instant,
    ttl: Duration,
    sweep_interval: Duration,
}
/// Write path of one segment.
pub struct SegmentWriter {
    id: u64,
    view: SegmentView,
    state: Mutex<WriterState>,
    states: Arc<SlotStates>,
    assignment: Arc<SlotAssignment>,
    replaced: RwLock<HashMap<u32, EntryRef>>,
    write_nos: Vec<AtomicU64>,
    centroids: Vec<AtomicU32>,
}
impl SegmentWriter {
    pub fn new(
        view: SegmentView,
        device_base: Addr,
    ) -> Result<SegmentWriter, WriteError> {
        let states = Arc::new(SlotStates::new(&view));
        let assignment = Arc::new(SlotAssignment::fixed(&view, device_base)?);
        SegmentWriter::from_parts(view, states, assignment, 0, 1)
    }
    /// Slots below `used_slots` are never handed out; `next_write_no` (>= 1) is the next write number.
    pub fn from_parts(
        view: SegmentView,
        states: Arc<SlotStates>,
        assignment: Arc<SlotAssignment>,
        used_slots: u32,
        next_write_no: u64,
    ) -> Result<SegmentWriter, WriteError> {
        let count = view.slot_count();
        if used_slots > count {
            return Err(
                WriteError::Slot(SlotError::SlotOutOfRange {
                    slot: used_slots,
                    count,
                }),
            );
        }
        if states.slot_count() != count {
            return Err(
                WriteError::Slot(SlotError::SlotOutOfRange {
                    slot: states.slot_count(),
                    count,
                }),
            );
        }
        if assignment.slot_count() != count {
            return Err(
                WriteError::Slot(SlotError::SlotOutOfRange {
                    slot: assignment.slot_count(),
                    count,
                }),
            );
        }
        if next_write_no == 0 {
            return Err(
                WriteError::Slot(SlotError::ValueCountMismatch {
                    expected: 1,
                    got: 0,
                }),
            );
        }
        Ok(SegmentWriter {
            id: NEXT_WRITER_ID.fetch_add(1, Ordering::Relaxed),
            view,
            state: Mutex::new(WriterState {
                cursor: used_slots,
                next_write_no,
                pending: HashMap::new(),
                queue: VecDeque::new(),
                last_sweep: Instant::now(),
                ttl: RESERVATION_TTL,
                sweep_interval: SWEEP_INTERVAL,
            }),
            states,
            assignment,
            replaced: RwLock::new(HashMap::new()),
            write_nos: (0..count).map(|_| AtomicU64::new(0)).collect(),
            centroids: (0..count).map(|_| AtomicU32::new(NO_CENTROID)).collect(),
        })
    }
    pub fn view(&self) -> SegmentView {
        self.view
    }
    pub fn states(&self) -> Arc<SlotStates> {
        Arc::clone(&self.states)
    }
    pub fn assignment(&self) -> Arc<SlotAssignment> {
        Arc::clone(&self.assignment)
    }
    fn lock(&self) -> MutexGuard<'_, WriterState> {
        self.state.lock().unwrap_or_else(|e| e.into_inner())
    }
    fn replaced_read(&self) -> RwLockReadGuard<'_, HashMap<u32, EntryRef>> {
        self.replaced.read().unwrap_or_else(|e| e.into_inner())
    }
    fn replaced_write(&self) -> RwLockWriteGuard<'_, HashMap<u32, EntryRef>> {
        self.replaced.write().unwrap_or_else(|e| e.into_inner())
    }
    fn centroid_at(&self, idx: usize) -> Option<CentroidId> {
        match self.centroids[idx].load(Ordering::Relaxed) {
            NO_CENTROID => None,
            v => Some(CentroidId(v)),
        }
    }
    /// Reserves the next free slot; it stays invisible until committed.
    pub fn reserve(
        &self,
        entry: EntryRef,
        tokens: u32,
        centroid: Option<CentroidId>,
    ) -> Result<Reservation, WriteError> {
        if centroid == Some(CentroidId(NO_CENTROID)) {
            return Err(WriteError::InvalidCentroid);
        }
        if tokens > MAX_TOKENS {
            return Err(WriteError::TokensOutOfRange {
                tokens,
            });
        }
        let max = self.view.class().max_tokens();
        if tokens > max {
            return Err(WriteError::TokensExceedClass {
                tokens,
                max,
            });
        }
        let mut st = self.lock();
        Self::sweep_expired(&mut st, false);
        if st.cursor >= self.view.slot_count() {
            return Err(WriteError::NoFreeSlot);
        }
        let slot = Slot(st.cursor);
        let addr = self.assignment.device_addr(slot)?;
        let target = self.assignment.byte_range(slot)?;
        st.cursor += 1;
        st.pending
            .insert(
                slot.0,
                PendingSlot {
                    entry,
                    tokens,
                    centroid,
                    queued: false,
                    reserved_at: Instant::now(),
                },
            );
        Ok(Reservation {
            slot,
            entry,
            tokens,
            centroid,
            addr,
            target,
            writer_id: self.id,
        })
    }
    /// Applies the commit immediately; requires a completed transfer event.
    pub fn commit(&self, res: &Reservation, transfer: &Event) -> Result<(), WriteError> {
        self.check_owner(res)?;
        let mut st = self.lock();
        match st.pending.get(&res.slot.0) {
            Some(p) if !p.queued => {}
            _ => {
                return Err(WriteError::NotReserved {
                    slot: res.slot,
                });
            }
        }
        match transfer.is_complete() {
            Err(e) => Err(WriteError::Event(e)),
            Ok(false) => Err(WriteError::NotReady),
            Ok(true) => self.apply(&mut st, res.slot),
        }
    }
    /// Queues the commit; it takes effect in `drain` once `event` is complete.
    pub fn commit_deferred(
        &self,
        res: &Reservation,
        event: Event,
    ) -> Result<(), WriteError> {
        self.check_owner(res)?;
        let mut st = self.lock();
        match st.pending.get_mut(&res.slot.0) {
            Some(p) if !p.queued => p.queued = true,
            _ => {
                return Err(WriteError::NotReserved {
                    slot: res.slot,
                });
            }
        }
        st.queue.push_back((res.slot, event));
        Ok(())
    }
    /// Applies queued commits in FIFO order; returns the number of slots that became visible.
    pub fn drain(&self) -> usize {
        let mut st = self.lock();
        let mut applied = 0;
        loop {
            let (slot, event) = match st.queue.front() {
                Some((slot, event)) => (*slot, event.clone()),
                None => break,
            };
            match event.is_complete() {
                Ok(false) => break,
                Ok(true) => {
                    st.queue.pop_front();
                    match self.apply(&mut st, slot) {
                        Ok(()) => applied += 1,
                        Err(e) => {
                            warn!("dropping queued commit of slot {}: {e}", slot.0);
                            st.pending.remove(&slot.0);
                        }
                    }
                }
                Err(e) => {
                    st.queue.pop_front();
                    st.pending.remove(&slot.0);
                    warn!("dropping queued commit of slot {}: {e}", slot.0);
                }
            }
        }
        applied
    }
    /// Hides the entry (valid bit cleared, data untouched). `true` if it had a valid slot.
    pub fn tombstone(&self, entry: EntryRef) -> bool {
        let _st = self.lock();
        let Some(slot) = self.assignment.slot_of_entry(entry) else {
            return false;
        };
        self.replaced_write().insert(slot.0, entry);
        if self.assignment.unbind(slot).is_err() {
            self.replaced_write().remove(&slot.0);
            return false;
        }
        if let Err(e) = self.states.invalidate(slot) {
            warn!("invalidate of slot {} failed: {e}", slot.0);
        }
        true
    }
    /// Current binding of the slot, else the replaced or hidden entry.
    pub fn resolve(&self, slot: Slot) -> Option<EntryRef> {
        self.assignment
            .lookup(slot)
            .or_else(|| self.replaced_read().get(&slot.0).copied())
    }
    pub fn slot_info(&self, slot: Slot) -> Result<SlotInfo, WriteError> {
        let meta = self.states.meta(slot)?;
        let idx = slot.0 as usize;
        Ok(SlotInfo {
            valid: meta.valid,
            tokens: meta.tokens,
            write_no: self.write_nos[idx].load(Ordering::Relaxed),
            centroid: self.centroid_at(idx),
        })
    }
    pub fn used_slots(&self) -> u32 {
        self.lock().cursor
    }
    pub fn free_slots(&self) -> u32 {
        self.view.slot_count().saturating_sub(self.used_slots())
    }
    pub fn pending_count(&self) -> usize {
        self.lock().pending.len()
    }
    /// Snapshot plan: all valid slots in ascending order, packed densely onto `0..n`.
    pub fn compaction_plan(&self) -> CompactionPlan {
        let st = self.lock();
        self.plan_locked(&st)
    }
    /// Creates a writer for `new_view` holding one reservation per surviving slot.
    pub fn rebuild(
        &self,
        new_view: SegmentView,
        new_device_base: Addr,
    ) -> Result<(SegmentWriter, Vec<(Slot, Reservation)>), WriteError> {
        let mut st = self.lock();
        Self::sweep_expired(&mut st, true);
        if !st.pending.is_empty() {
            return Err(WriteError::Busy);
        }
        let plan = self.plan_locked(&st);
        let fits = new_view.class() == self.view.class()
            && new_view.dim() == self.view.dim() && new_view.dtype() == self.view.dtype()
            && new_view.slot_count() as usize >= plan.moves.len();
        if !fits {
            return Err(WriteError::RebuildTarget);
        }
        let writer = SegmentWriter::new(new_view, new_device_base)?;
        let mut reservations = Vec::with_capacity(plan.moves.len());
        for m in &plan.moves {
            let res = writer.reserve(m.entry, m.tokens, m.centroid)?;
            reservations.push((m.from, res));
        }
        Ok((writer, reservations))
    }
    /// Drops open (not queued) reservations older than the TTL; runs at most once per interval unless forced.
    fn sweep_expired(st: &mut WriterState, force: bool) {
        let now = Instant::now();
        if !force && now.duration_since(st.last_sweep) < st.sweep_interval {
            return;
        }
        st.last_sweep = now;
        let ttl = st.ttl;
        st.pending
            .retain(|slot, p| {
                let expired = !p.queued && now.duration_since(p.reserved_at) >= ttl;
                if expired {
                    warn!("dropping expired reservation of slot {slot}");
                }
                !expired
            });
    }
    fn check_owner(&self, res: &Reservation) -> Result<(), WriteError> {
        if res.writer_id == self.id {
            Ok(())
        } else {
            Err(WriteError::ForeignReservation)
        }
    }
    fn plan_locked(&self, st: &WriterState) -> CompactionPlan {
        let mut moves: Vec<SlotMove> = Vec::new();
        for (word_idx, word) in self.states.snapshot_words().into_iter().enumerate() {
            let mut bits = word;
            while bits != 0 {
                let idx = word_idx * 64 + bits.trailing_zeros() as usize;
                bits &= bits - 1;
                let slot = Slot(idx as u32);
                let Some(entry) = self.assignment.lookup(slot) else {
                    continue;
                };
                let Ok(meta) = self.states.meta(slot) else {
                    continue;
                };
                moves
                    .push(SlotMove {
                        from: slot,
                        to: Slot(moves.len() as u32),
                        entry,
                        tokens: meta.tokens,
                        centroid: self.centroid_at(idx),
                    });
            }
        }
        let reclaimed_slots = st.cursor.saturating_sub(moves.len() as u32);
        CompactionPlan {
            moves,
            reclaimed_slots,
        }
    }
    fn apply(&self, st: &mut WriterState, slot: Slot) -> Result<(), WriteError> {
        let Some(p) = st.pending.get(&slot.0).copied() else {
            return Err(WriteError::NotReserved { slot });
        };
        let idx = slot.0 as usize;
        let write_no = st.next_write_no;
        self.write_nos[idx].store(write_no, Ordering::Relaxed);
        self.centroids[idx]
            .store(p.centroid.map_or(NO_CENTROID, |c| c.0), Ordering::Relaxed);
        let old = self.assignment.slot_of_entry(p.entry);
        let result = self.publish(slot, p, old);
        if let Err(e) = result {
            self.write_nos[idx].store(0, Ordering::Relaxed);
            self.centroids[idx].store(NO_CENTROID, Ordering::Relaxed);
            return Err(e);
        }
        if let Some(old) = old {
            if let Err(e) = self.states.invalidate(old) {
                warn!("invalidate of replaced slot {} failed: {e}", old.0);
            }
        }
        st.next_write_no = write_no.saturating_add(1);
        st.pending.remove(&slot.0);
        Ok(())
    }
    fn publish(
        &self,
        slot: Slot,
        p: PendingSlot,
        old: Option<Slot>,
    ) -> Result<(), WriteError> {
        if let Some(old) = old {
            self.replaced_write().insert(old.0, p.entry);
            if let Err(e) = self.assignment.unbind(old) {
                self.replaced_write().remove(&old.0);
                return Err(e.into());
            }
        }
        let outcome = self
            .assignment
            .bind(slot, p.entry)
            .and_then(|()| {
                self.states
                    .set_filled(slot, p.tokens)
                    .inspect_err(|_| {
                        let _ = self.assignment.unbind(slot);
                    })
            });
        if let Err(e) = outcome {
            if let Some(old) = old {
                let _ = self.assignment.bind(old, p.entry);
                self.replaced_write().remove(&old.0);
            }
            return Err(e.into());
        }
        Ok(())
    }
}
#[cfg(test)]
#[path = "test/segment_writer.rs"]
mod tests;
use std::time::{Duration, Instant};
