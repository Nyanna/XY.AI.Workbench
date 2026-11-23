use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::fmt;
use std::sync::{PoisonError, RwLock, RwLockReadGuard, RwLockWriteGuard};
use super::super::l040::a::kernel_batch::{DeviceAddr, SegmentDesc, SegmentLayout};
use super::super::l040::l030::a::kernel_layout::Slot;
use super::super::l040::l030::a::region_handle::RegionHandle;
use super::super::l040::l030::l020::a::arenas::ArenaKind;
use super::super::l040::l030::l020::a::index_contract::EntryId;
use super::super::l040::l030::l020::a::memory_access::{AccessError, AccessRegion};
/// Assigned in ascending order, never reused.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SegmentId(pub u64);
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SlotRef {
    pub segment: SegmentId,
    pub slot: Slot,
}
/// Slots of one segment are addressed by `Slot(u32)`; larger segments are truncated to this count.
pub const MAX_SLOTS_PER_SEGMENT: u64 = u32::MAX as u64 + 1;
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SlotError {
    InvalidSlotSize,
    WrongArena(ArenaKind),
    SegmentTooSmall { len: u64, slot_size: u64 },
    BackingMismatch { handle_len: u64, mem_len: u64 },
    UnknownSegment(SegmentId),
    DuplicateRegion(RegionHandle),
    NoFreeSlot,
    PayloadTooLarge { len: usize, slot_size: u64 },
    UnknownEntry(EntryId),
    OutOfBounds { len: usize, slot_size: u64 },
    Access(AccessError),
}
impl fmt::Display for SlotError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SlotError::InvalidSlotSize => write!(f, "slot size must be non-zero"),
            SlotError::WrongArena(kind) => {
                write!(f, "region belongs to arena {:?}, expected IndexSegments", kind)
            }
            SlotError::SegmentTooSmall { len, slot_size } => {
                write!(
                    f, "region of {len} bytes cannot hold a slot of {slot_size} bytes"
                )
            }
            SlotError::BackingMismatch { handle_len, mem_len } => {
                write!(
                    f, "handle length {handle_len} differs from backing length {mem_len}"
                )
            }
            SlotError::UnknownSegment(id) => write!(f, "unknown segment {}", id.0),
            SlotError::DuplicateRegion(handle) => {
                write!(f, "region already registered: {:?}", handle)
            }
            SlotError::NoFreeSlot => write!(f, "no free slot"),
            SlotError::PayloadTooLarge { len, slot_size } => {
                write!(f, "payload of {len} bytes exceeds slot size {slot_size}")
            }
            SlotError::UnknownEntry(entry) => write!(f, "entry {} has no slot", entry.0),
            SlotError::OutOfBounds { len, slot_size } => {
                write!(f, "read of {len} bytes exceeds slot size {slot_size}")
            }
            SlotError::Access(err) => write!(f, "region access failed: {err}"),
        }
    }
}
impl std::error::Error for SlotError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            SlotError::Access(err) => Some(err),
            _ => None,
        }
    }
}
impl From<AccessError> for SlotError {
    fn from(err: AccessError) -> Self {
        SlotError::Access(err)
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RemovedSegment {
    pub id: SegmentId,
    pub handle: RegionHandle,
    /// Ascending; their mapping is gone.
    pub entries: Vec<EntryId>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SegmentStats {
    pub segments: usize,
    pub slots_total: u64,
    pub slots_used: u64,
}
struct Segment {
    handle: RegionHandle,
    mem: AccessRegion,
    slot_count: u64,
    /// Slots below this index have been handed out at least once.
    fresh_from: u64,
    /// Released slots below `fresh_from`.
    released: BTreeSet<u64>,
    occupied: BTreeMap<u64, EntryId>,
}
impl Segment {
    fn lowest_free(&self) -> Option<u64> {
        if let Some(slot) = self.released.first() {
            return Some(*slot);
        }
        (self.fresh_from < self.slot_count).then_some(self.fresh_from)
    }
    fn has_free(&self) -> bool {
        self.lowest_free().is_some()
    }
    fn claim(&mut self, slot: u64, entry: EntryId) {
        if !self.released.remove(&slot) && slot >= self.fresh_from {
            for gap in self.fresh_from..slot {
                self.released.insert(gap);
            }
            self.fresh_from = slot.saturating_add(1);
        }
        self.occupied.insert(slot, entry);
    }
    fn release(&mut self, slot: u64) {
        if self.occupied.remove(&slot).is_some() {
            self.released.insert(slot);
        }
    }
}
struct Inner {
    slot_size: u64,
    next_id: u64,
    segments: BTreeMap<SegmentId, Segment>,
    handles: HashSet<RegionHandle>,
    with_free: BTreeSet<SegmentId>,
    entries: HashMap<EntryId, SlotRef>,
}
impl Inner {
    fn slot_range(&self, slot: u64) -> Result<usize, SlotError> {
        let out_of_bounds = SlotError::OutOfBounds {
            len: 0,
            slot_size: self.slot_size,
        };
        let offset = slot.checked_mul(self.slot_size).ok_or(out_of_bounds.clone())?;
        usize::try_from(offset).map_err(|_| out_of_bounds)
    }
    fn write_slot(
        &self,
        segment: &Segment,
        slot: u64,
        bytes: &[u8],
    ) -> Result<(), SlotError> {
        let offset = self.slot_range(slot)?;
        let size = usize::try_from(self.slot_size)
            .map_err(|_| SlotError::PayloadTooLarge {
                len: bytes.len(),
                slot_size: self.slot_size,
            })?;
        let mut buf = vec![0u8; size];
        buf[..bytes.len()].copy_from_slice(bytes);
        segment.mem.write(offset, &buf)?;
        Ok(())
    }
    fn refresh_free(&mut self, id: SegmentId) {
        let free = self.segments.get(&id).is_some_and(Segment::has_free);
        if free {
            self.with_free.insert(id);
        } else {
            self.with_free.remove(&id);
        }
    }
}
/// Thread-safe slot store over VRAM segments.
pub struct SegmentStore {
    slot_size: u64,
    inner: RwLock<Inner>,
}
impl fmt::Debug for SegmentStore {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SegmentStore")
            .field("slot_size", &self.slot_size)
            .finish_non_exhaustive()
    }
}
fn read_lock(lock: &RwLock<Inner>) -> RwLockReadGuard<'_, Inner> {
    lock.read().unwrap_or_else(PoisonError::into_inner)
}
fn write_lock(lock: &RwLock<Inner>) -> RwLockWriteGuard<'_, Inner> {
    lock.write().unwrap_or_else(PoisonError::into_inner)
}
impl SegmentStore {
    pub fn new(slot_size: u64) -> Result<SegmentStore, SlotError> {
        if slot_size == 0 {
            return Err(SlotError::InvalidSlotSize);
        }
        Ok(SegmentStore {
            slot_size,
            inner: RwLock::new(Inner {
                slot_size,
                next_id: 0,
                segments: BTreeMap::new(),
                handles: HashSet::new(),
                with_free: BTreeSet::new(),
                entries: HashMap::new(),
            }),
        })
    }
    pub fn slot_size(&self) -> u64 {
        self.slot_size
    }
    /// `mem` byte 0 is the start of `handle`.
    pub fn add_segment(
        &self,
        handle: RegionHandle,
        mem: AccessRegion,
    ) -> Result<SegmentId, SlotError> {
        if handle.arena() != ArenaKind::IndexSegments {
            return Err(SlotError::WrongArena(handle.arena()));
        }
        if handle.len() < self.slot_size {
            return Err(SlotError::SegmentTooSmall {
                len: handle.len(),
                slot_size: self.slot_size,
            });
        }
        let mem_len = mem.len() as u64;
        if mem_len != handle.len() {
            return Err(SlotError::BackingMismatch {
                handle_len: handle.len(),
                mem_len,
            });
        }
        let mut inner = write_lock(&self.inner);
        if inner.handles.contains(&handle) {
            return Err(SlotError::DuplicateRegion(handle));
        }
        let next = inner.next_id.checked_add(1).ok_or(SlotError::NoFreeSlot)?;
        let id = SegmentId(inner.next_id);
        inner.next_id = next;
        inner.handles.insert(handle);
        inner
            .segments
            .insert(
                id,
                Segment {
                    handle,
                    mem,
                    slot_count: (handle.len() / self.slot_size)
                        .min(MAX_SLOTS_PER_SEGMENT),
                    fresh_from: 0,
                    released: BTreeSet::new(),
                    occupied: BTreeMap::new(),
                },
            );
        inner.with_free.insert(id);
        Ok(id)
    }
    /// Removes the segment and the mapping of its entries; frees nothing.
    pub fn remove_segment(&self, id: SegmentId) -> Result<RemovedSegment, SlotError> {
        let mut inner = write_lock(&self.inner);
        let segment = inner.segments.remove(&id).ok_or(SlotError::UnknownSegment(id))?;
        inner.handles.remove(&segment.handle);
        inner.with_free.remove(&id);
        let mut entries: Vec<EntryId> = segment.occupied.values().copied().collect();
        for entry in &entries {
            inner.entries.remove(entry);
        }
        entries.sort_unstable();
        Ok(RemovedSegment {
            id,
            handle: segment.handle,
            entries,
        })
    }
    pub fn segment_handle(&self, id: SegmentId) -> Option<RegionHandle> {
        read_lock(&self.inner).segments.get(&id).map(|s| s.handle)
    }
    pub fn slot_count(&self, id: SegmentId) -> Option<u64> {
        read_lock(&self.inner).segments.get(&id).map(|s| s.slot_count)
    }
    pub fn segment_ids(&self) -> Vec<SegmentId> {
        read_lock(&self.inner).segments.keys().copied().collect()
    }
    /// Maps `entry` to a slot and writes `bytes`, zero-padding the rest of the slot.
    pub fn put(&self, entry: EntryId, bytes: &[u8]) -> Result<SlotRef, SlotError> {
        if u64::try_from(bytes.len()).map_or(true, |len| len > self.slot_size) {
            return Err(SlotError::PayloadTooLarge {
                len: bytes.len(),
                slot_size: self.slot_size,
            });
        }
        let mut inner = write_lock(&self.inner);
        if let Some(existing) = inner.entries.get(&entry).copied() {
            let segment = inner
                .segments
                .get(&existing.segment)
                .ok_or(SlotError::UnknownSegment(existing.segment))?;
            inner.write_slot(segment, u64::from(existing.slot.0), bytes)?;
            return Ok(existing);
        }
        let id = inner.with_free.first().copied().ok_or(SlotError::NoFreeSlot)?;
        let segment = inner.segments.get(&id).ok_or(SlotError::UnknownSegment(id))?;
        let index = segment.lowest_free().ok_or(SlotError::NoFreeSlot)?;
        let slot = Slot(u32::try_from(index).map_err(|_| SlotError::NoFreeSlot)?);
        inner.write_slot(segment, index, bytes)?;
        if let Some(segment) = inner.segments.get_mut(&id) {
            segment.claim(index, entry);
        }
        let slot_ref = SlotRef { segment: id, slot };
        inner.entries.insert(entry, slot_ref);
        inner.refresh_free(id);
        Ok(slot_ref)
    }
    /// Drops the mapping only; the slot bytes stay until the next `put`.
    pub fn remove(&self, entry: EntryId) -> Option<SlotRef> {
        let mut inner = write_lock(&self.inner);
        let slot_ref = inner.entries.remove(&entry)?;
        if let Some(segment) = inner.segments.get_mut(&slot_ref.segment) {
            segment.release(u64::from(slot_ref.slot.0));
        }
        inner.refresh_free(slot_ref.segment);
        Some(slot_ref)
    }
    /// Reads `dst.len()` bytes from the start of the entry's slot.
    pub fn read(&self, entry: EntryId, dst: &mut [u8]) -> Result<(), SlotError> {
        let inner = read_lock(&self.inner);
        let slot_ref = inner
            .entries
            .get(&entry)
            .copied()
            .ok_or(SlotError::UnknownEntry(entry))?;
        if u64::try_from(dst.len()).map_or(true, |len| len > self.slot_size) {
            return Err(SlotError::OutOfBounds {
                len: dst.len(),
                slot_size: self.slot_size,
            });
        }
        if dst.is_empty() {
            return Ok(());
        }
        let segment = inner
            .segments
            .get(&slot_ref.segment)
            .ok_or(SlotError::UnknownSegment(slot_ref.segment))?;
        let offset = inner.slot_range(u64::from(slot_ref.slot.0))?;
        segment.mem.read(offset, dst)?;
        Ok(())
    }
    pub fn slot_of(&self, entry: EntryId) -> Option<SlotRef> {
        read_lock(&self.inner).entries.get(&entry).copied()
    }
    /// `None`: free or unknown.
    pub fn entry_at(&self, slot: SlotRef) -> Option<EntryId> {
        read_lock(&self.inner)
            .segments
            .get(&slot.segment)?
            .occupied
            .get(&u64::from(slot.slot.0))
            .copied()
    }
    /// Occupied slots in slot order.
    pub fn entries_in(&self, id: SegmentId) -> Result<Vec<(Slot, EntryId)>, SlotError> {
        let inner = read_lock(&self.inner);
        let segment = inner.segments.get(&id).ok_or(SlotError::UnknownSegment(id))?;
        Ok(
            segment
                .occupied
                .iter()
                .filter_map(|(slot, entry)| Some((
                    Slot(u32::try_from(*slot).ok()?),
                    *entry,
                )))
                .collect(),
        )
    }
    /// `base` is not validated.
    pub fn segment_desc(
        &self,
        id: SegmentId,
        base: DeviceAddr,
    ) -> Result<SegmentDesc, SlotError> {
        let inner = read_lock(&self.inner);
        let segment = inner.segments.get(&id).ok_or(SlotError::UnknownSegment(id))?;
        Ok(SegmentDesc {
            base,
            entries: segment.slot_count,
            layout: SegmentLayout::Fixed {
                stride: self.slot_size,
            },
        })
    }
    /// `None`: outside, free slot or unknown segment.
    pub fn entry_of_addr(
        &self,
        id: SegmentId,
        base: DeviceAddr,
        addr: DeviceAddr,
    ) -> Option<EntryId> {
        let inner = read_lock(&self.inner);
        let segment = inner.segments.get(&id)?;
        let desc = SegmentDesc {
            base,
            entries: segment.slot_count,
            layout: SegmentLayout::Fixed {
                stride: self.slot_size,
            },
        };
        let slot = desc.entry_of(addr, &[])?;
        segment.occupied.get(&slot).copied()
    }
    pub fn stats(&self) -> SegmentStats {
        let inner = read_lock(&self.inner);
        SegmentStats {
            segments: inner.segments.len(),
            slots_total: inner
                .segments
                .values()
                .fold(0u64, |acc, s| acc.saturating_add(s.slot_count)),
            slots_used: inner.entries.len() as u64,
        }
    }
}
#[cfg(test)]
#[path = "test/segments.rs"]
mod tests;
