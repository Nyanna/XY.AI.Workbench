//! Index-segment use of the shared arena allocator: aligned segments, copy-on-write headroom
//! and epoch-gated release of replaced versions. No allocator of its own.
use std::ops::Range;
use parking_lot::{Mutex, MutexGuard};
use crate::resident::error::ManagerError;
use crate::resident::memory::{ArenaAllocator, MemoryError, Region};
use crate::resident::subengine::EventHandle;
/// Minimum alignment of every segment offset.
pub const SEGMENT_ALIGN: u64 = 256;
/// Space that must stay free so a replaced segment can be rebuilt next to its predecessor.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CowBudget {
    pub live: u64,
    pub max_concurrent_cow: u32,
    pub largest_segment: u64,
}
impl CowBudget {
    pub fn headroom(&self) -> u64 {
        u64::from(self.max_concurrent_cow).saturating_mul(self.largest_segment)
    }
    pub fn required(&self) -> u64 {
        self.live.saturating_add(self.headroom())
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ArenaBudget {
    pub capacity: u64,
    pub reserve: u64,
    pub live: u64,
    /// Replaced segments still readable by an older epoch.
    pub retired: u64,
    pub cow_headroom: u64,
    /// Bytes a regular allocation may still claim.
    pub available: u64,
}
struct Retired {
    region: Region,
    epoch: u64,
}
struct State {
    live: u64,
    largest: u64,
    retired: Vec<Retired>,
}
impl State {
    fn is_retired(&self, region: Region) -> bool {
        self.retired.iter().any(|r| r.region == region)
    }
}
pub struct IndexArena<'a> {
    inner: &'a ArenaAllocator,
    reserve: u64,
    max_concurrent_cow: u32,
    state: Mutex<State>,
    cow_gate: Mutex<()>,
}
impl<'a> IndexArena<'a> {
    /// `reserve` bytes of the arena are never handed out.
    pub fn new(inner: &'a ArenaAllocator, reserve: u64) -> Self {
        Self {
            inner,
            reserve,
            max_concurrent_cow: 1,
            state: Mutex::new(State {
                live: 0,
                largest: 0,
                retired: Vec::new(),
            }),
            cow_gate: Mutex::new(()),
        }
    }
    pub fn alloc_segment(&self, bytes: u64, align: u64) -> Result<Region, ManagerError> {
        let mut st = self.state.lock();
        let largest = st.largest.max(bytes);
        let headroom = u64::from(self.max_concurrent_cow).saturating_mul(largest);
        let region = self.admit_and_alloc(bytes, align, headroom)?;
        st.live += region.len;
        st.largest = largest;
        tracing::debug!(
            offset = region.offset, bytes, live = st.live, headroom, "segment allocated"
        );
        Ok(region)
    }
    /// Segment the build path fills incrementally through [`WritableRegion::claim`].
    pub fn reserve_writable(&self, cap: u64) -> Result<WritableRegion, ManagerError> {
        let region = self.alloc_segment(cap, SEGMENT_ALIGN)?;
        Ok(WritableRegion {
            region,
            cursor: 0,
        })
    }
    /// Immediate release of a region no reader or in-flight work can reference.
    pub fn free(&self, region: Region) -> Result<(), ManagerError> {
        let mut st = self.state.lock();
        self.ensure_owned(&st, region)?;
        self.inner.free(region).map_err(map_memory)?;
        st.live = st.live.saturating_sub(region.len);
        tracing::debug!(offset = region.offset, bytes = region.len, "segment freed");
        Ok(())
    }
    /// Release once all work recorded before `event` has finished.
    pub fn free_after(
        &self,
        region: Region,
        event: EventHandle,
    ) -> Result<(), ManagerError> {
        let mut st = self.state.lock();
        self.ensure_owned(&st, region)?;
        self.inner.release_after(region, event).map_err(map_memory)?;
        st.live = st.live.saturating_sub(region.len);
        Ok(())
    }
    /// Marks `region` as replaced by the version published at `epoch`.
    pub fn retire(&self, region: Region, epoch: u64) -> Result<(), ManagerError> {
        let mut st = self.state.lock();
        self.ensure_owned(&st, region)?;
        st.live = st.live.saturating_sub(region.len);
        st.retired.push(Retired { region, epoch });
        Ok(())
    }
    /// Frees every retired region that no reader at `min_reader_epoch` or later can still see.
    pub fn reclaim(&self, min_reader_epoch: u64) -> usize {
        let due: Vec<Region> = {
            let mut st = self.state.lock();
            let (due, keep): (Vec<_>, Vec<_>) = st
                .retired
                .drain(..)
                .partition(|r| r.epoch <= min_reader_epoch);
            st.retired = keep;
            due.into_iter().map(|r| r.region).collect()
        };
        let mut freed = 0;
        for region in due {
            match self.inner.free(region) {
                Ok(()) => freed += 1,
                Err(e) => tracing::warn!(error = % e, "retired region already gone"),
            }
        }
        if freed > 0 {
            tracing::debug!(freed, min_reader_epoch, "retired segments released");
        }
        freed
    }
    /// Starts a copy-on-write replacement of `old`; waits while another one is in progress.
    pub fn begin_cow(&self, old: Region) -> Result<CowTicket<'_, 'a>, ManagerError> {
        let gate = self.cow_gate.lock();
        self.start_cow(gate, old)
    }
    /// Like [`begin_cow`](Self::begin_cow), but `Ok(None)` when another replacement is running.
    pub fn try_begin_cow(
        &self,
        old: Region,
    ) -> Result<Option<CowTicket<'_, 'a>>, ManagerError> {
        match self.cow_gate.try_lock() {
            Some(gate) => self.start_cow(gate, old).map(Some),
            None => Ok(None),
        }
    }
    pub fn cow_budget(&self) -> CowBudget {
        let st = self.state.lock();
        CowBudget {
            live: st.live,
            max_concurrent_cow: self.max_concurrent_cow,
            largest_segment: st.largest,
        }
    }
    pub fn budget(&self) -> ArenaBudget {
        self.inner.reclaim();
        let st = self.state.lock();
        let capacity = self.inner.capacity();
        let used = capacity - self.inner.available();
        let headroom = u64::from(self.max_concurrent_cow).saturating_mul(st.largest);
        let retired = st.retired.iter().map(|r| r.region.len).sum();
        ArenaBudget {
            capacity,
            reserve: self.reserve,
            live: st.live,
            retired,
            cow_headroom: headroom,
            available: capacity
                .saturating_sub(used)
                .saturating_sub(self.reserve)
                .saturating_sub(headroom),
        }
    }
    pub fn trace_utilization(&self) {
        let b = self.budget();
        tracing::debug!(
            capacity = b.capacity, live = b.live, retired = b.retired, headroom = b
            .cow_headroom, available = b.available, "index arena utilization"
        );
    }
    fn start_cow<'g>(
        &'g self,
        gate: MutexGuard<'g, ()>,
        old: Region,
    ) -> Result<CowTicket<'g, 'a>, ManagerError> {
        let mut st = self.state.lock();
        self.ensure_owned(&st, old)?;
        let new = self.admit_and_alloc(old.len, SEGMENT_ALIGN, 0)?;
        st.live += new.len;
        tracing::debug!(live = st.live, bytes = new.len, "copy-on-write started");
        Ok(CowTicket {
            arena: self,
            _gate: gate,
            old,
            new: Some(new),
        })
    }
    fn admit_and_alloc(
        &self,
        bytes: u64,
        align: u64,
        headroom: u64,
    ) -> Result<Region, ManagerError> {
        self.inner.reclaim();
        let capacity = self.inner.capacity();
        let used = capacity - self.inner.available();
        let needed = used
            .saturating_add(bytes)
            .saturating_add(headroom)
            .saturating_add(self.reserve);
        if needed > capacity {
            let available = capacity
                .saturating_sub(used)
                .saturating_sub(headroom)
                .saturating_sub(self.reserve);
            tracing::warn!(
                requested = bytes, available, headroom, "index arena admission refused"
            );
            return Err(ManagerError::ArenaExhausted {
                requested: bytes,
                available,
            });
        }
        self.inner.alloc(bytes, align.max(SEGMENT_ALIGN)).map_err(map_memory)
    }
    fn ensure_owned(&self, st: &State, region: Region) -> Result<(), ManagerError> {
        if st.is_retired(region) || !self.inner.is_live(region) {
            return Err(ManagerError::StaleRegion {
                arena: region.arena.0,
                gen: region.gen,
            });
        }
        Ok(())
    }
}
/// Replacement segment under construction. Dropping it without `commit` discards the copy.
pub struct CowTicket<'a, 'b> {
    arena: &'a IndexArena<'b>,
    _gate: MutexGuard<'a, ()>,
    old: Region,
    new: Option<Region>,
}
impl CowTicket<'_, '_> {
    pub fn old_region(&self) -> Region {
        self.old
    }
    pub fn new_region(&self) -> Region {
        self.new.expect("ticket holds its region until commit")
    }
    /// Publishes the copy as the version of `epoch`; the old segment is released by `reclaim`.
    pub fn commit(mut self, epoch: u64) -> Result<Region, ManagerError> {
        let new = self.new.take().expect("ticket holds its region until commit");
        if let Err(e) = self.arena.retire(self.old, epoch) {
            self.new = Some(new);
            return Err(e);
        }
        Ok(new)
    }
}
impl Drop for CowTicket<'_, '_> {
    fn drop(&mut self) {
        if let Some(new) = self.new.take() {
            if let Err(e) = self.arena.free(new) {
                tracing::warn!(
                    error = % e, "discarded copy-on-write segment could not be freed"
                );
            }
        }
    }
}
/// Segment filled front to back by a single writer.
#[derive(Debug)]
pub struct WritableRegion {
    region: Region,
    cursor: u64,
}
impl WritableRegion {
    pub fn region(&self) -> Region {
        self.region
    }
    pub fn written(&self) -> u64 {
        self.cursor
    }
    pub fn remaining(&self) -> u64 {
        self.region.len - self.cursor
    }
    /// Claims `len` bytes at `align`; returns the absolute arena offsets.
    pub fn claim(&mut self, len: u64, align: u64) -> Result<Range<u64>, ManagerError> {
        let align = align.max(1);
        let start = (self.region.offset + self.cursor).next_multiple_of(align);
        let end = start.saturating_add(len);
        if end > self.region.offset + self.region.len {
            return Err(ManagerError::ArenaExhausted {
                requested: len,
                available: (self.region.offset + self.region.len).saturating_sub(start),
            });
        }
        self.cursor = end - self.region.offset;
        Ok(start..end)
    }
    pub fn into_parts(self) -> (Region, u64) {
        (self.region, self.cursor)
    }
}
fn map_memory(e: MemoryError) -> ManagerError {
    match e {
        MemoryError::ArenaExhausted { requested, free, .. } => {
            ManagerError::ArenaExhausted {
                requested,
                available: free,
            }
        }
        MemoryError::StaleRegion { arena, gen } => {
            ManagerError::StaleRegion {
                arena,
                gen,
            }
        }
        other => ManagerError::Memory(other),
    }
}
#[cfg(test)]
#[path = "test/arena.rs"]
mod tests;
