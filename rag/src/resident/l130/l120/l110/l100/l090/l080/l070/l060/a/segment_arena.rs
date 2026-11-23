//! Aligned segment allocation on top of an `Allocator`, with reserve and
//! replacement headroom, replacement by copy, and bump segments for one writer.
use std::borrow::Borrow;
use std::collections::HashMap;
use std::fmt;
use std::sync::{Mutex, MutexGuard, PoisonError};
use log::{debug, warn};
use super::super::l050::a::table_swap::{Epoch, TableSwap, TableSwapError};
use super::super::l050::l040::a::allocator::{Allocator, AllocatorError};
use super::super::l050::l040::l030::a::region_handle::RegionHandle;
use super::super::l050::l040::l030::l020::l010::device_lane_event::Event;
pub const MIN_ALIGNMENT: u64 = 256;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SegmentArenaConfig {
    /// Power of two, at least `MIN_ALIGNMENT`.
    pub alignment: u64,
    /// Reserve in basis points of the capacity (0..=10_000).
    pub reserve_bp: u32,
    /// Number of concurrent replacements assumed by the headroom calculation.
    pub concurrent_replacements: u32,
}
impl Default for SegmentArenaConfig {
    fn default() -> Self {
        SegmentArenaConfig {
            alignment: MIN_ALIGNMENT,
            reserve_bp: 0,
            concurrent_replacements: 1,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SegmentArenaError {
    Exhausted { requested: u64, available: u64 },
    StaleRegion,
    NoReplacement,
    ReplacementInProgress,
    ZeroLength,
    InvalidAlignment,
    InvalidConfig,
    /// Publication failed; the replacement stays pending.
    Publish(TableSwapError),
    /// The old segment is missing in the next table; the replacement stays pending.
    OldNotInTable,
    Allocator(AllocatorError),
}
impl fmt::Display for SegmentArenaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SegmentArenaError::Exhausted { requested, available } => {
                write!(
                    f,
                    "Arena erschöpft: requested {requested} bytes, available {available} bytes"
                )
            }
            SegmentArenaError::StaleRegion => write!(f, "stale Region"),
            SegmentArenaError::NoReplacement => write!(f, "kein Replacement"),
            SegmentArenaError::ReplacementInProgress => {
                write!(f, "replacement in progress")
            }
            SegmentArenaError::ZeroLength => write!(f, "zero length"),
            SegmentArenaError::InvalidAlignment => write!(f, "invalid alignment"),
            SegmentArenaError::InvalidConfig => write!(f, "invalid segment arena config"),
            SegmentArenaError::Publish(e) => write!(f, "publish failed: {e}"),
            SegmentArenaError::OldNotInTable => {
                write!(f, "old segment not in next table")
            }
            SegmentArenaError::Allocator(e) => write!(f, "allocator error: {e}"),
        }
    }
}
impl std::error::Error for SegmentArenaError {}
/// Aligned segment inside an allocator region (which includes the slack).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Segment {
    pub region: RegionHandle,
    /// Absolute, aligned arena offset (>= `region.offset()`).
    pub offset: u64,
    /// Usable length: requested length rounded up to the alignment.
    pub len: u64,
}
pub enum Readers {
    None,
    After(Event),
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ReplacementBudget {
    pub live_bytes: u64,
    pub replacements: u32,
    pub largest_segment: u64,
}
impl ReplacementBudget {
    pub fn headroom(&self) -> u64 {
        u64::from(self.replacements).saturating_mul(self.largest_segment)
    }
    pub fn swap_need(&self, incoming: u64) -> Option<u64> {
        self.live_bytes.checked_add(incoming)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ArenaBudget {
    pub capacity: u64,
    pub reserve: u64,
    pub live: u64,
    pub deferred: u64,
    pub headroom: u64,
    pub available: u64,
}
impl ArenaBudget {
    /// Share of the capacity held by live and deferred segments, in 0.0..=1.0.
    pub fn utilization(&self) -> f64 {
        if self.capacity == 0 {
            return 0.0;
        }
        let used = self.live.saturating_add(self.deferred);
        (used as f64 / self.capacity as f64).clamp(0.0, 1.0)
    }
}
impl fmt::Display for ArenaBudget {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "capacity={} reserve={} live={} deferred={} headroom={} available={} utilization={:.1}%",
            self.capacity, self.reserve, self.live, self.deferred, self.headroom, self
            .available, self.utilization() * 100.0
        )
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SegmentStats {
    pub live_bytes: u64,
    pub live_segments: usize,
    pub largest_segment: u64,
    pub deferred_segments: usize,
    pub deferred_bytes: u64,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ReplacementInfo {
    pub old: Segment,
    pub new: Segment,
}
struct Deferred {
    region: RegionHandle,
    epoch: Epoch,
}
#[derive(Default)]
struct State {
    live: HashMap<RegionHandle, Segment>,
    deferred: Vec<Deferred>,
    replacement: Option<ReplacementInfo>,
}
impl State {
    fn largest(&self) -> u64 {
        self.live
            .keys()
            .map(|r| r.len())
            .chain(self.deferred.iter().map(|d| d.region.len()))
            .max()
            .unwrap_or(0)
    }
    fn live_bytes(&self) -> u64 {
        self.live.keys().fold(0u64, |s, r| s.saturating_add(r.len()))
    }
    fn deferred_bytes(&self) -> u64 {
        self.deferred.iter().fold(0u64, |s, d| s.saturating_add(d.region.len()))
    }
}
fn align_up(value: u64, align: u64) -> Option<u64> {
    value.checked_add(align - 1).map(|v| v & !(align - 1))
}
fn map_allocator_error(e: AllocatorError, requested: u64) -> SegmentArenaError {
    match e {
        AllocatorError::OutOfMemory { free, .. } => {
            SegmentArenaError::Exhausted {
                requested,
                available: free,
            }
        }
        AllocatorError::StaleRegion => SegmentArenaError::StaleRegion,
        other => SegmentArenaError::Allocator(other),
    }
}
pub struct SegmentArena<A: Borrow<Allocator>> {
    allocator: A,
    config: SegmentArenaConfig,
    capacity: u64,
    reserve: u64,
    state: Mutex<State>,
}
impl<A: Borrow<Allocator>> SegmentArena<A> {
    pub fn new(
        allocator: A,
        config: SegmentArenaConfig,
    ) -> Result<Self, SegmentArenaError> {
        if !config.alignment.is_power_of_two() || config.alignment < MIN_ALIGNMENT
            || config.reserve_bp > 10_000 || config.concurrent_replacements == 0
        {
            return Err(SegmentArenaError::InvalidConfig);
        }
        let capacity = allocator.borrow().capacity();
        let reserve = ((u128::from(capacity) * u128::from(config.reserve_bp) + 9_999)
            / 10_000) as u64;
        Ok(SegmentArena {
            allocator,
            config,
            capacity,
            reserve,
            state: Mutex::new(State::default()),
        })
    }
    fn alloc(&self) -> &Allocator {
        self.allocator.borrow()
    }
    fn lock(&self) -> MutexGuard<'_, State> {
        self.state.lock().unwrap_or_else(PoisonError::into_inner)
    }
    /// Drops `live` entries whose region was freed at the allocator directly
    /// (e.g. by eviction); the running replacement is never touched.
    fn prune_freed(&self, st: &mut State) {
        let alloc = self.alloc();
        let protected = st.replacement.map(|r| (r.old.region, r.new.region));
        let before = st.live.len();
        st.live
            .retain(|region, _| {
                alloc.is_live(region)
                    || protected
                        .is_some_and(|(old, new)| *region == old || *region == new)
            });
        let removed = before - st.live.len();
        if removed > 0 {
            debug!("pruned {removed} segment(s) freed outside the arena");
        }
    }
    fn occupied(&self) -> u64 {
        self.capacity.saturating_sub(self.alloc().available())
    }
    fn headroom_for(&self, largest: u64) -> u64 {
        u64::from(self.config.concurrent_replacements).saturating_mul(largest)
    }
    fn budget_locked(&self, st: &State) -> ArenaBudget {
        let occupied = self.occupied();
        let headroom = self.headroom_for(st.largest());
        ArenaBudget {
            capacity: self.capacity,
            reserve: self.reserve,
            live: st.live_bytes(),
            deferred: st.deferred_bytes(),
            headroom,
            available: self
                .capacity
                .saturating_sub(occupied)
                .saturating_sub(self.reserve)
                .saturating_sub(headroom),
        }
    }
    /// Allocates a region of `region_len` bytes and aligns a segment of `len` bytes inside it.
    fn place(&self, region_len: u64, len: u64) -> Result<Segment, SegmentArenaError> {
        let region = self
            .alloc()
            .allocate(region_len)
            .map_err(|e| map_allocator_error(e, region_len))?;
        let aligned = align_up(region.offset(), self.config.alignment)
            .and_then(|offset| {
                let end = offset.checked_add(len)?;
                let region_end = region.offset().checked_add(region.len())?;
                (end <= region_end).then_some(offset)
            });
        match aligned {
            Some(offset) => Ok(Segment { region, offset, len }),
            None => {
                if let Err(e) = self.alloc().free(&region) {
                    warn!("misfit region at offset {} not freed: {e}", region.offset());
                }
                Err(SegmentArenaError::Allocator(AllocatorError::AllocationFailed))
            }
        }
    }
    pub fn allocate(&self, len: u64) -> Result<Segment, SegmentArenaError> {
        if len == 0 {
            return Err(SegmentArenaError::ZeroLength);
        }
        let mut st = self.lock();
        self.alloc().reap();
        self.prune_freed(&mut st);
        let occupied = self.occupied();
        let alignment = self.config.alignment;
        let sizes = align_up(len, alignment)
            .and_then(|l| l.checked_add(alignment - 1).map(|a| (l, a)));
        let Some((seg_len, region_len)) = sizes else {
            return Err(SegmentArenaError::Exhausted {
                requested: len,
                available: self
                    .capacity
                    .saturating_sub(occupied)
                    .saturating_sub(self.reserve)
                    .saturating_sub(self.headroom_for(st.largest())),
            });
        };
        let headroom = self.headroom_for(st.largest().max(region_len));
        let fits = occupied
            .checked_add(region_len)
            .and_then(|v| v.checked_add(headroom))
            .and_then(|v| v.checked_add(self.reserve))
            .is_some_and(|v| v <= self.capacity);
        if !fits {
            return Err(SegmentArenaError::Exhausted {
                requested: region_len,
                available: self
                    .capacity
                    .saturating_sub(occupied)
                    .saturating_sub(self.reserve)
                    .saturating_sub(headroom),
            });
        }
        let seg = self.place(region_len, seg_len)?;
        st.live.insert(seg.region, seg);
        debug!("segment allocated: {}", self.budget_locked(& st));
        Ok(seg)
    }
    pub fn allocate_bump(&self, len: u64) -> Result<BumpSegment, SegmentArenaError> {
        self.allocate(len).map(BumpSegment::new)
    }
    pub fn release(
        &self,
        seg: &Segment,
        readers: Readers,
    ) -> Result<(), SegmentArenaError> {
        let mut st = self.lock();
        let region = seg.region;
        if !st.live.contains_key(&region) || !self.alloc().is_live(&region) {
            return Err(SegmentArenaError::StaleRegion);
        }
        if let Some(r) = &st.replacement {
            if r.old.region == region || r.new.region == region {
                return Err(SegmentArenaError::ReplacementInProgress);
            }
        }
        let result = match readers {
            Readers::None => self.alloc().free(&region),
            Readers::After(ev) => self.alloc().free_deferred(&region, ev),
        };
        result.map_err(|e| map_allocator_error(e, region.len()))?;
        st.live.remove(&region);
        debug!("segment released: {}", self.budget_locked(& st));
        Ok(())
    }
    pub fn begin_replace(&self, old: &Segment) -> Result<Segment, SegmentArenaError> {
        let mut st = self.lock();
        if st.replacement.is_some() {
            return Err(SegmentArenaError::ReplacementInProgress);
        }
        self.alloc().reap();
        let tracked = match st.live.get(&old.region) {
            Some(s) if self.alloc().is_live(&old.region) => *s,
            _ => return Err(SegmentArenaError::StaleRegion),
        };
        let size = tracked.region.len();
        let occupied = self.occupied();
        let need = ReplacementBudget {
            live_bytes: occupied,
            replacements: self.config.concurrent_replacements,
            largest_segment: st.largest(),
        }
            .swap_need(size);
        let fits = need
            .and_then(|n| n.checked_add(self.reserve))
            .is_some_and(|n| n <= self.capacity);
        if !fits {
            return Err(SegmentArenaError::Exhausted {
                requested: size,
                available: self
                    .capacity
                    .saturating_sub(occupied)
                    .saturating_sub(self.reserve),
            });
        }
        let new = self.place(size, tracked.len)?;
        st.live.insert(new.region, new);
        st.replacement = Some(ReplacementInfo {
            old: tracked,
            new,
        });
        debug!("replacement started: {}", self.budget_locked(& st));
        Ok(new)
    }
    pub fn finish_replace(&self, swap: &TableSwap) -> Result<Epoch, SegmentArenaError> {
        let mut st = self.lock();
        let info = st.replacement.ok_or(SegmentArenaError::NoReplacement)?;
        let mut table = swap.next_table();
        if !table.replace(&info.old.region, info.new.region) {
            return Err(SegmentArenaError::OldNotInTable);
        }
        let epoch = swap.publish(table).map_err(SegmentArenaError::Publish)?;
        st.replacement = None;
        st.live.remove(&info.old.region);
        st.deferred
            .push(Deferred {
                region: info.old.region,
                epoch,
            });
        debug!("replacement published: {}", self.budget_locked(& st));
        Ok(epoch)
    }
    pub fn discard_replacement(&self) -> Result<(), SegmentArenaError> {
        let mut st = self.lock();
        let info = st.replacement.take().ok_or(SegmentArenaError::NoReplacement)?;
        st.live.remove(&info.new.region);
        self.alloc()
            .free(&info.new.region)
            .map_err(|e| map_allocator_error(e, info.new.region.len()))
    }
    pub fn replacement(&self) -> Option<ReplacementInfo> {
        self.lock().replacement
    }
    pub fn reclaim(&self, min_reader_epoch: Epoch) -> usize {
        let mut st = self.lock();
        let (due, kept): (Vec<_>, Vec<_>) = std::mem::take(&mut st.deferred)
            .into_iter()
            .partition(|d| d.epoch <= min_reader_epoch);
        st.deferred = kept;
        let mut freed = 0;
        for d in due {
            match self.alloc().free(&d.region) {
                Ok(()) => freed += 1,
                Err(e) => {
                    warn!(
                        "deferred region at offset {} already gone: {e}", d.region
                        .offset()
                    )
                }
            }
        }
        freed
    }
    pub fn budget(&self) -> ArenaBudget {
        let mut st = self.lock();
        self.prune_freed(&mut st);
        self.budget_locked(&st)
    }
    pub fn replacement_budget(&self) -> ReplacementBudget {
        let st = self.lock();
        ReplacementBudget {
            live_bytes: self.occupied(),
            replacements: self.config.concurrent_replacements,
            largest_segment: st.largest(),
        }
    }
    pub fn stats(&self) -> SegmentStats {
        let mut st = self.lock();
        self.prune_freed(&mut st);
        SegmentStats {
            live_bytes: st.live_bytes(),
            live_segments: st.live.len(),
            largest_segment: st.largest(),
            deferred_segments: st.deferred.len(),
            deferred_bytes: st.deferred_bytes(),
        }
    }
}
/// Bump allocation inside one segment for a single writer.
#[derive(Debug)]
pub struct BumpSegment {
    segment: Segment,
    cursor: u64,
}
impl BumpSegment {
    pub fn new(segment: Segment) -> BumpSegment {
        BumpSegment { segment, cursor: 0 }
    }
    /// Returns the absolute arena address aligned to `align` (power of two >= 1).
    pub fn claim(&mut self, len: u64, align: u64) -> Result<u64, SegmentArenaError> {
        if !align.is_power_of_two() {
            return Err(SegmentArenaError::InvalidAlignment);
        }
        let base = self.segment.offset;
        let placed = base
            .checked_add(self.cursor)
            .and_then(|p| align_up(p, align))
            .and_then(|addr| addr.checked_add(len).map(|end| (addr, end)))
            .filter(|&(_, end)| {
                base.checked_add(self.segment.len).is_some_and(|limit| end <= limit)
            });
        match placed {
            Some((addr, end)) => {
                self.cursor = end - base;
                Ok(addr)
            }
            None => {
                Err(SegmentArenaError::Exhausted {
                    requested: len,
                    available: self.remaining(),
                })
            }
        }
    }
    pub fn written(&self) -> u64 {
        self.cursor
    }
    pub fn remaining(&self) -> u64 {
        self.segment.len - self.cursor
    }
    pub fn segment(&self) -> &Segment {
        &self.segment
    }
    pub fn into_parts(self) -> (Segment, u64) {
        (self.segment, self.cursor)
    }
}
#[cfg(test)]
#[path = "test/segment_arena.rs"]
mod tests;
