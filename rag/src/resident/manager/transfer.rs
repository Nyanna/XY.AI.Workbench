//! Device-side execution of load plans. Entries are packed on the host, leave as one copy per
//! staging slot and are reported as segments whose slot bindings follow through the update queue.

use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use parking_lot::Mutex;

use crate::resident::error::ManagerError;
use crate::resident::index::{EntryId, EntrySet, Stamp};
use crate::resident::manager::arena::IndexArena;
use crate::resident::manager::residency::{LoadGeometry, LoadTransfer, SegmentId};
use crate::resident::manager::streamed::SharedIndex;
use crate::resident::manager::upload::{Placement, Prepared, StagedGather, StagingBuffer, UploadPort};
use crate::resident::memory::error::{MemoryError, TransferDir};
use crate::resident::memory::Region;
use crate::resident::segment::mapping::SlotMap;

pub trait RegionSource: Send + Sync {
    fn alloc_segment(&self, bytes: u64, align: u64) -> Result<Region, ManagerError>;
    fn free_segment(&self, region: Region) -> Result<(), ManagerError>;
}

impl RegionSource for IndexArena<'_> {
    fn alloc_segment(&self, bytes: u64, align: u64) -> Result<Region, ManagerError> {
        IndexArena::alloc_segment(self, bytes, align)
    }

    fn free_segment(&self, region: Region) -> Result<(), ManagerError> {
        self.free(region)
    }
}

/// A segment that reached the device. The slot map is unbound; `binds` lists the bindings to
/// queue, each with the stamp read before the entry's data.
pub struct LoadedSegment {
    pub region: Region,
    pub map: SlotMap,
    pub binds: Vec<(EntryId, u32, Stamp)>,
}

pub trait SegmentSink: Send + Sync {
    fn loaded(&self, segment: LoadedSegment);
}

#[derive(Default)]
pub struct LoadedQueue {
    items: Mutex<Vec<LoadedSegment>>,
}

impl LoadedQueue {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn drain(&self) -> Vec<LoadedSegment> {
        std::mem::take(&mut *self.items.lock())
    }

    pub fn len(&self) -> usize {
        self.items.lock().len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl SegmentSink for LoadedQueue {
    fn loaded(&self, segment: LoadedSegment) {
        self.items.lock().push(segment);
    }
}

/// Every plan shape goes through the staged gather: whole segments are the entries the geometry
/// assigns to them, and zero-copy mapping falls back to a copy because no mapped host memory is
/// available to the device here.
pub struct GatherTransfer<S: StagingBuffer> {
    index: SharedIndex,
    geometry: Arc<dyn LoadGeometry>,
    gather: StagedGather<S>,
    port: Arc<dyn UploadPort>,
    regions: Arc<dyn RegionSource>,
    sink: Arc<dyn SegmentSink>,
    align: u64,
}

impl<S: StagingBuffer> GatherTransfer<S> {
    pub fn new(
        index: SharedIndex,
        geometry: Arc<dyn LoadGeometry>,
        gather: StagedGather<S>,
        port: Arc<dyn UploadPort>,
        regions: Arc<dyn RegionSource>,
        sink: Arc<dyn SegmentSink>,
        align: u64,
    ) -> Self {
        Self { index, geometry, gather, port, regions, sink, align }
    }

    fn upload(&self, entries: &[EntryId]) -> Result<u64, ManagerError> {
        // Stamps are read before the data: a concurrent write then leaves an older stamp
        // registered, which costs one redundant reload instead of a missed one.
        let stamps: HashMap<EntryId, Stamp> = entries
            .iter()
            .filter_map(|e| self.index.stamp(*e).map(|s| (*e, s)))
            .collect();
        let mut rest = entries;
        let mut moved = 0;
        while !rest.is_empty() {
            let prepared = self.gather.gather(&*self.index, rest, self.align)?;
            let (taken, bytes) = (prepared.taken(), prepared.bytes());
            if taken == 0 {
                return Err(MemoryError::TransferFailed {
                    dir: TransferDir::HostToDevice,
                    reason: "gather made no progress".into(),
                }
                .into());
            }
            if bytes > 0 {
                let placements = prepared.placements().to_vec();
                let region = self.regions.alloc_segment(bytes, self.align)?;
                if let Err(e) = self.send(prepared, region) {
                    let _ = self.regions.free_segment(region);
                    return Err(e);
                }
                tracing::debug!(entries = placements.len(), bytes, "segment loaded");
                self.sink.loaded(layout(region, &placements, bytes, &stamps));
                moved += bytes;
            }
            rest = &rest[taken..];
        }
        Ok(moved)
    }

    fn send(&self, prepared: Prepared<'_, S>, region: Region) -> Result<(), ManagerError> {
        let event = prepared.submit(&*self.port, region)?;
        event.synchronize().map_err(|e| MemoryError::TransferFailed {
            dir: TransferDir::HostToDevice,
            reason: e.to_string(),
        })?;
        Ok(())
    }
}

fn layout(
    region: Region,
    placements: &[Placement],
    bytes: u64,
    stamps: &HashMap<EntryId, Stamp>,
) -> LoadedSegment {
    let mut offsets: Vec<u64> = placements
        .iter()
        .map(|p| region.offset + p.range.start)
        .collect();
    offsets.push(region.offset + bytes);
    let binds = placements
        .iter()
        .enumerate()
        .filter_map(|(slot, p)| stamps.get(&p.entry).map(|s| (p.entry, slot as u32, *s)))
        .collect();
    LoadedSegment { region, map: SlotMap::variable(offsets), binds }
}

impl<S: StagingBuffer + 'static> LoadTransfer for GatherTransfer<S> {
    fn load_segments(&self, segments: &[SegmentId]) -> Result<u64, ManagerError> {
        let wanted: HashSet<SegmentId> = segments.iter().copied().collect();
        let mut entries: Vec<EntryId> = self
            .index
            .stamps()
            .map(|(e, _)| e)
            .filter(|e| wanted.contains(&self.geometry.segment_of(*e)))
            .collect();
        entries.sort_unstable();
        self.upload(&entries)
    }

    fn load_gather(&self, set: &EntrySet) -> Result<u64, ManagerError> {
        self.upload(&set.iter().collect::<Vec<_>>())
    }

    fn map_zero_copy(&self, set: &EntrySet) -> Result<u64, ManagerError> {
        tracing::debug!(entries = set.len(), "zero-copy unavailable, gathering");
        self.load_gather(set)
    }
}

#[cfg(test)]
#[path = "test/transfer.rs"]
mod tests;
