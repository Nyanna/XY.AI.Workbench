//! Copy-on-write map from arena addresses to segment entries and the
//! versioned store holding the published map.

use std::fmt;
use std::ops::Range;
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::a::table_swap::Epoch;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::a::slot_layout::{
    SegmentView, SlotAssignment, SlotStates,
};
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::a::kernel_layout::{
    Addr, Slot,
};

/// Segment id, unique within one map.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SegmentId(pub u32);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MapError {
    EmptySegment,
    DuplicateId(SegmentId),
    Overlap { with: SegmentId },
    RangeOverflow,
    EpochOverflow,
    StaleMapping { base: Epoch, published: Epoch },
}

impl fmt::Display for MapError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MapError::EmptySegment => write!(f, "empty segment range"),
            MapError::DuplicateId(id) => write!(f, "duplicate segment id {}", id.0),
            MapError::Overlap { with } => write!(f, "segment overlaps segment {}", with.0),
            MapError::RangeOverflow => write!(f, "segment range overflows u64"),
            MapError::EpochOverflow => write!(f, "epoch overflow"),
            MapError::StaleMapping { .. } => write!(f, "stale Mapping"),
        }
    }
}

impl std::error::Error for MapError {}

/// One segment of the map; cheap to clone.
#[derive(Clone)]
pub struct Entry {
    id: SegmentId,
    view: SegmentView,
    device_base: Addr,
    states: Arc<SlotStates>,
    assignment: Arc<SlotAssignment>,
    range: Range<u64>,
}

impl fmt::Debug for Entry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Entry")
            .field("id", &self.id)
            .field("device_base", &self.device_base)
            .field("range", &self.range)
            .finish()
    }
}

impl Entry {
    pub fn new(
        id: SegmentId,
        view: SegmentView,
        device_base: Addr,
        states: Arc<SlotStates>,
        assignment: Arc<SlotAssignment>,
    ) -> Result<Entry, MapError> {
        let start = view.region().offset();
        let end = start
            .checked_add(view.total_bytes())
            .ok_or(MapError::RangeOverflow)?;
        Ok(Entry {
            id,
            view,
            device_base,
            states,
            assignment,
            range: start..end,
        })
    }

    pub fn id(&self) -> SegmentId {
        self.id
    }

    pub fn view(&self) -> &SegmentView {
        &self.view
    }

    pub fn device_base(&self) -> Addr {
        self.device_base
    }

    pub fn states(&self) -> &Arc<SlotStates> {
        &self.states
    }

    pub fn assignment(&self) -> &Arc<SlotAssignment> {
        &self.assignment
    }

    /// Arena range `offset .. offset + total_bytes`.
    pub fn range(&self) -> Range<u64> {
        self.range.clone()
    }

    /// Slot containing `addr`; `None` outside `range()`.
    pub fn slot_at(&self, addr: u64) -> Option<Slot> {
        if !self.range.contains(&addr) {
            return None;
        }
        let index = (addr - self.range.start) / u64::from(self.view.slot_bytes());
        u32::try_from(index).ok().map(Slot)
    }
}

/// Copy-on-write map with an epoch; a clone is a stable snapshot.
#[derive(Clone)]
pub struct AddressMap {
    epoch: Epoch,
    segments: Arc<Vec<Entry>>,
}

impl fmt::Debug for AddressMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AddressMap")
            .field("epoch", &self.epoch)
            .field("segments", &self.segments)
            .finish()
    }
}

impl AddressMap {
    pub fn new(epoch: Epoch) -> AddressMap {
        AddressMap {
            epoch,
            segments: Arc::new(Vec::new()),
        }
    }

    pub fn epoch(&self) -> Epoch {
        self.epoch
    }

    /// Clone with epoch + 1 and the same segments.
    pub fn successor(&self) -> Result<AddressMap, MapError> {
        let epoch = self.epoch.checked_add(1).ok_or(MapError::EpochOverflow)?;
        Ok(AddressMap {
            epoch,
            segments: Arc::clone(&self.segments),
        })
    }

    pub fn add(&mut self, entry: Entry) -> Result<(), MapError> {
        let range = entry.range();
        if range.start >= range.end {
            return Err(MapError::EmptySegment);
        }
        if self.segments.iter().any(|e| e.id == entry.id) {
            return Err(MapError::DuplicateId(entry.id));
        }
        let pos = self.segments.partition_point(|e| e.range.start < range.start);
        if pos > 0 {
            let prev = &self.segments[pos - 1];
            if prev.range.end > range.start {
                return Err(MapError::Overlap { with: prev.id });
            }
        }
        if let Some(next) = self.segments.get(pos) {
            if next.range.start < range.end {
                return Err(MapError::Overlap { with: next.id });
            }
        }
        Arc::make_mut(&mut self.segments).insert(pos, entry);
        Ok(())
    }

    /// `true` when the segment was present and has been removed.
    pub fn remove(&mut self, id: SegmentId) -> bool {
        match self.segments.iter().position(|e| e.id == id) {
            Some(pos) => {
                Arc::make_mut(&mut self.segments).remove(pos);
                true
            }
            None => false,
        }
    }

    /// Segments in ascending address order.
    pub fn segments(&self) -> &[Entry] {
        &self.segments
    }

    pub fn len(&self) -> usize {
        self.segments.len()
    }

    pub fn is_empty(&self) -> bool {
        self.segments.is_empty()
    }

    pub fn get(&self, id: SegmentId) -> Option<&Entry> {
        self.segments.iter().find(|e| e.id == id)
    }

    /// Segment whose range contains the arena address.
    pub fn lookup(&self, addr: u64) -> Option<&Entry> {
        let pos = self.segments.partition_point(|e| e.range.start <= addr);
        let entry = self.segments.get(pos.checked_sub(1)?)?;
        (addr < entry.range.end).then_some(entry)
    }
}

/// Versioned holder of the published map.
pub struct MappingStore {
    current: RwLock<AddressMap>,
}

impl fmt::Debug for MappingStore {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MappingStore")
            .field("epoch", &self.epoch())
            .finish()
    }
}

impl MappingStore {
    pub fn new() -> MappingStore {
        MappingStore {
            current: RwLock::new(AddressMap::new(0)),
        }
    }

    fn read(&self) -> RwLockReadGuard<'_, AddressMap> {
        self.current.read().unwrap_or_else(|e| e.into_inner())
    }

    fn write(&self) -> RwLockWriteGuard<'_, AddressMap> {
        self.current.write().unwrap_or_else(|e| e.into_inner())
    }

    pub fn epoch(&self) -> Epoch {
        self.read().epoch()
    }

    /// Snapshot only when `base` equals the published epoch.
    pub fn snapshot(&self, base: Epoch) -> Result<AddressMap, MapError> {
        let guard = self.read();
        if guard.epoch() == base {
            Ok(guard.clone())
        } else {
            Err(MapError::StaleMapping {
                base,
                published: guard.epoch(),
            })
        }
    }

    pub fn current(&self) -> AddressMap {
        self.read().clone()
    }

    /// Successor of the current map; basis for a new publication.
    pub fn next(&self) -> Result<AddressMap, MapError> {
        self.read().successor()
    }

    /// Publishes only when `map.epoch() == published + 1`; returns the new epoch.
    pub fn publish(&self, map: AddressMap) -> Result<Epoch, MapError> {
        let mut guard = self.write();
        let published = guard.epoch();
        if published.checked_add(1) == Some(map.epoch()) {
            let epoch = map.epoch();
            *guard = map;
            Ok(epoch)
        } else {
            Err(MapError::StaleMapping {
                base: map.epoch().saturating_sub(1),
                published,
            })
        }
    }
}

impl Default for MappingStore {
    fn default() -> Self {
        MappingStore::new()
    }
}

#[cfg(test)]
#[path = "test/address_map.rs"]
mod tests;
