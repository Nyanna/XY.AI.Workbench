//! Configuration, errors, reports and the queue of loaded segments.

use std::collections::VecDeque;
use std::fmt;
use std::sync::{Arc, Mutex};

use super::deps::*;

#[derive(Clone, Debug)]
pub struct GpuManagerConfig {
    /// Token limit of every slot; `SlotClass::custom(1)` for one vector per entry.
    pub class: SlotClass,
    pub dim: u32,
    /// F32, F16 or BF16; must equal `Index::dtype()`.
    pub dtype: DType,
    /// Requested length of a new segment, at least one slot.
    pub segment_bytes: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub enum GpuManagerError {
    InvalidConfig(String),
    Index(String),
    Resolve(ResolveError),
    DTypeMismatch { index: DType, configured: DType },
    InvalidVector { len: usize, row_bytes: usize },
    TooManyTokens { tokens: u32, max: u32 },
    Arena(SegmentArenaError),
    SegmentLimit,
    UnknownSegment(u64),
    Slot(SlotError),
    SlotWrite(WriteError),
    Publish(TableSwapError),
    Mapping(MapError),
    Manifest(ManifestError),
    Memory(String),
    VramSync(String),
    IndexWrite(IndexError),
}

impl fmt::Display for GpuManagerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GpuManagerError::InvalidConfig(m) => {
                write!(f, "ungültige GPU-Manager-Konfiguration: {m}")
            }
            GpuManagerError::Index(m) => write!(f, "Index nicht verfügbar: {m}"),
            GpuManagerError::Resolve(e) => write!(f, "scope resolution failed: {e}"),
            GpuManagerError::DTypeMismatch { index, configured } => {
                write!(f, "dtype mismatch: index {index:?}, configured {configured:?}")
            }
            GpuManagerError::InvalidVector { len, row_bytes } => {
                write!(f, "vector of {len} bytes is not a multiple of the row size {row_bytes}")
            }
            GpuManagerError::TooManyTokens { tokens, max } => {
                write!(f, "{tokens} tokens exceed the slot limit of {max}")
            }
            GpuManagerError::Arena(e) => write!(f, "segment arena: {e}"),
            GpuManagerError::SegmentLimit => f.write_str("global slot space exhausted"),
            GpuManagerError::UnknownSegment(id) => write!(f, "unknown segment {id}"),
            GpuManagerError::Slot(e) => write!(f, "slot layout: {e}"),
            GpuManagerError::SlotWrite(e) => write!(f, "slot write: {e}"),
            GpuManagerError::Publish(e) => write!(f, "table publish: {e}"),
            GpuManagerError::Mapping(e) => write!(f, "mapping publish: {e}"),
            GpuManagerError::Manifest(e) => write!(f, "manifest publish: {e}"),
            GpuManagerError::Memory(m) => write!(f, "device memory: {m}"),
            GpuManagerError::VramSync(m) => write!(f, "VRAM sync: {m}"),
            GpuManagerError::IndexWrite(e) => write!(f, "index write: {e}"),
        }
    }
}

impl std::error::Error for GpuManagerError {}

/// Binding delivered with a loaded segment; becomes a Bind in the update queue.
#[derive(Clone, Copy, Debug)]
pub struct LoadedBinding {
    pub entry: EntryRef,
    pub slot: Slot5,
    pub stamp: Stamp,
}

/// A filled segment that is not yet visible to readers.
pub struct LoadedSegment3 {
    pub segment: Segment,
    pub view: SegmentView,
    pub device_base: Addr,
    pub states: Arc<SlotStates>,
    pub assignment: Arc<SlotAssignment>,
    pub bindings: Vec<LoadedBinding>,
    /// Kept so that later slots can be appended; `None` for read-only segments.
    pub writer: Option<Arc<SegmentWriter>>,
}

/// FIFO queue of reported segments. Clones share the queue.
#[derive(Clone, Default)]
pub struct LoadedQueue {
    queue: Arc<Mutex<VecDeque<LoadedSegment3>>>,
}

impl LoadedQueue {
    pub fn report(&self, seg: LoadedSegment3) {
        lock(&self.queue).push_back(seg);
    }

    pub fn drain(&self) -> Vec<LoadedSegment3> {
        lock(&self.queue).drain(..).collect()
    }

    pub fn len(&self) -> usize {
        lock(&self.queue).len()
    }

    pub fn is_empty(&self) -> bool {
        lock(&self.queue).is_empty()
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct EnsureReport {
    /// Newly placed, ascending.
    pub loaded: Vec<EntryId>,
    /// Resident, or placed and waiting for their bind; ascending.
    pub already_resident: Vec<EntryId>,
    pub absent_in_index: Vec<EntryId>,
    pub failed: Vec<(EntryId, GpuManagerError)>,
    /// Sum of slot bytes written.
    pub bytes_moved: u64,
    /// Reported to the `LoadedQueue`, not yet published.
    pub segments_created: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ReconcileOutcome {
    pub reconciliation: Reconciliation,
    pub queued_unbinds: usize,
    pub reloaded: usize,
    pub skipped_pending: usize,
    pub failed: usize,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct GapReport3 {
    pub slots_hidden: usize,
    pub reclaimed: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GpuIndexStats {
    pub segments: usize,
    pub resident_bytes: u64,
    pub pending_updates: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct VramTable {
    pub segments_addr: u64,
    pub segment_count: u32,
    pub version: u64,
}

/// A processed GPU result for the index. `placed` names the slot that already holds the data.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MirrorItem {
    pub id: EntryId,
    pub vector: Vec<u8>,
    pub meta: Vec<u8>,
    pub placed: Option<SlotRef>,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct MirrorReport {
    pub written: Vec<(EntryId, Stamp)>,
    pub failed: Vec<(EntryId, GpuManagerError)>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleasedSegment {
    pub id: SegmentId,
    pub region: RegionHandle,
    /// Entries whose residency was removed, ascending.
    pub cleared: Vec<EntryId>,
}

#[cfg(test)]
#[path = "test/types.rs"]
mod tests;
