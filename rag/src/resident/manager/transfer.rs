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
use crate::resident::manager::upload::{
    Placement, Prepared, StagedGather, StagingBuffer, UploadPort,
};
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
pub trait MappedHost: Send + Sync {
    fn map_alloc(&self, bytes: u64, align: u64) -> Result<Region, MemoryError>;
    fn map_write(
        &self,
        region: Region,
        offset: u64,
        src: &[u8],
    ) -> Result<(), MemoryError>;
    fn map_free(&self, region: Region) -> Result<(), MemoryError>;
}
impl MappedHost for crate::resident::memory::cuda::CudaMemoryManager {
    fn map_alloc(&self, bytes: u64, align: u64) -> Result<Region, MemoryError> {
        use crate::resident::memory::{budget::ArenaKind, DeviceMemory};
        self.alloc_in(ArenaKind::Mapped, bytes, align)
    }
    fn map_write(
        &self,
        region: Region,
        offset: u64,
        src: &[u8],
    ) -> Result<(), MemoryError> {
        self.write_mapped(region, offset, src)
    }
    fn map_free(&self, region: Region) -> Result<(), MemoryError> {
        crate::resident::memory::DeviceMemory::free(self, region)
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
/// Whole segments and gathers go through the staged copy; whole segments are the entries the
/// geometry assigns to them. Zero-copy writes straight into mapped host memory and falls back to
/// the copy when no mapped arena is attached or it has no room.
pub struct GatherTransfer<S: StagingBuffer> {
    index: SharedIndex,
    geometry: Arc<dyn LoadGeometry>,
    gather: StagedGather<S>,
    port: Arc<dyn UploadPort>,
    regions: Arc<dyn RegionSource>,
    sink: Arc<dyn SegmentSink>,
    align: u64,
    mapped: Option<Arc<dyn MappedHost>>,
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
        Self {
            index,
            geometry,
            gather,
            port,
            regions,
            sink,
            align,
            mapped: None,
        }
    }
    pub fn with_mapped(mut self, mapped: Arc<dyn MappedHost>) -> Self {
        self.mapped = Some(mapped);
        self
    }
    fn stamps_of(&self, entries: &[EntryId]) -> HashMap<EntryId, Stamp> {
        entries.iter().filter_map(|e| self.index.stamp(*e).map(|s| (*e, s))).collect()
    }
    fn upload(&self, entries: &[EntryId]) -> Result<u64, ManagerError> {
        let stamps = self.stamps_of(entries);
        let mut rest = entries;
        let mut moved = 0;
        while !rest.is_empty() {
            let prepared = self.gather.gather(&*self.index, rest, self.align)?;
            let (taken, bytes) = (prepared.taken(), prepared.bytes());
            if taken == 0 {
                return Err(
                    MemoryError::TransferFailed {
                        dir: TransferDir::HostToDevice,
                        reason: "gather made no progress".into(),
                    }
                        .into(),
                );
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
    fn send(
        &self,
        prepared: Prepared<'_, S>,
        region: Region,
    ) -> Result<(), ManagerError> {
        let event = prepared.submit(&*self.port, region)?;
        event
            .synchronize()
            .map_err(|e| MemoryError::TransferFailed {
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
    LoadedSegment {
        region,
        map: SlotMap::variable(offsets),
        binds,
    }
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
        let Some(mapped) = &self.mapped else {
            tracing::debug!(entries = set.len(), "no mapped arena, gathering");
            return self.load_gather(set);
        };
        let entries: Vec<EntryId> = set.iter().collect();
        let stamps = self.stamps_of(&entries);
        let data: Vec<_> = entries.iter().map(|e| (*e, self.index.read(*e))).collect();
        let align = self.align.max(1);
        let mut placements = Vec::new();
        let mut bytes = 0u64;
        for (entry, d) in &data {
            let len = d.as_bytes().len() as u64;
            if len == 0 {
                continue;
            }
            let start = bytes.div_ceil(align) * align;
            placements
                .push(Placement {
                    entry: *entry,
                    range: start..start + len,
                });
            bytes = start + len;
        }
        if bytes == 0 {
            return Ok(0);
        }
        let region = match mapped.map_alloc(bytes, align) {
            Ok(region) => region,
            Err(e) => {
                tracing::debug!(
                    error = % e, bytes, "mapped arena unavailable, gathering"
                );
                return self.load_gather(set);
            }
        };
        let written = data
            .iter()
            .filter(|(_, d)| !d.as_bytes().is_empty())
            .zip(&placements)
            .try_for_each(|((_, d), p)| {
                mapped.map_write(region, p.range.start, d.as_bytes())
            });
        if let Err(e) = written {
            let _ = mapped.map_free(region);
            return Err(e.into());
        }
        tracing::debug!(entries = placements.len(), bytes, "segment mapped");
        self.sink.loaded(layout(region, &placements, bytes, &stamps));
        Ok(bytes)
    }
}
#[cfg(test)]
#[path = "test/transfer.rs"]
mod tests;
