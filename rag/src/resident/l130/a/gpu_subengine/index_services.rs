use std::sync::{Arc, Mutex};

use super::backend::GpuBackend;
use super::deps::*;
use super::error::GpuError;

/// Structures shared by all indexes of one sub-engine: the segment arena and the one table.
pub(super) struct IndexShared {
    arena: SegmentArena<Arc<Allocator>>,
    swap: Arc<TableSwap>,
    allocators: Arc<AllocatorSet>,
}

impl IndexShared {
    /// Both the arena and the table use the one `IndexSegments` allocator.
    pub(super) fn new(
        allocators: Arc<AllocatorSet>,
        sink: Arc<dyn TableSink>,
        root: Arc<dyn RootSlot>,
    ) -> Result<IndexShared, GpuError> {
        let allocator = allocators
            .shared(ArenaKind::IndexSegments)
            .ok_or(GpuError::Memory(AllocatorError::AllocationFailed))?;
        let arena = SegmentArena::new(Arc::clone(&allocator), SegmentArenaConfig::default())
            .map_err(|e| GpuError::IndexMemory(GpuManagerError::Arena(e)))?;
        let swap = Arc::new(TableSwap::new(allocator, sink, root));
        Ok(IndexShared {
            arena,
            swap,
            allocators,
        })
    }

    pub(super) fn swap(&self) -> &Arc<TableSwap> {
        &self.swap
    }

    pub(super) fn allocators(&self) -> &Arc<AllocatorSet> {
        &self.allocators
    }
}

/// The `GpuServices` of one attached index.
pub(super) struct IndexServices {
    ordinal: u32,
    backend: Arc<dyn GpuBackend>,
    shared: Arc<IndexShared>,
    store: Arc<MappingStore>,
    manifests: Arc<ManifestStore>,
    eviction: Arc<Eviction>,
    updates: Arc<UpdateQueue>,
    residency: Arc<Mutex<ResidencyDirectory>>,
    structure: Arc<StructureLock>,
}

impl IndexServices {
    pub(super) fn new(
        ordinal: u32,
        backend: Arc<dyn GpuBackend>,
        shared: Arc<IndexShared>,
        store: Arc<MappingStore>,
        eviction: Arc<Eviction>,
    ) -> IndexServices {
        IndexServices {
            ordinal,
            backend,
            shared,
            store,
            manifests: Arc::new(ManifestStore::new(StoreConfig::default())),
            eviction,
            updates: Arc::new(UpdateQueue::new()),
            residency: Arc::new(Mutex::new(ResidencyDirectory::new())),
            structure: Arc::new(StructureLock::new()),
        }
    }
}

impl GpuServices for IndexServices {
    fn arena(&self) -> &SegmentArena<Arc<Allocator>> {
        &self.shared.arena
    }

    fn table_swap(&self) -> &Arc<TableSwap> {
        &self.shared.swap
    }

    fn mapping_store(&self) -> &Arc<MappingStore> {
        &self.store
    }

    fn manifests(&self) -> &Arc<ManifestStore> {
        &self.manifests
    }

    fn eviction(&self) -> &Arc<Eviction> {
        &self.eviction
    }

    fn updates(&self) -> &Arc<UpdateQueue> {
        &self.updates
    }

    fn residency(&self) -> &Arc<Mutex<ResidencyDirectory>> {
        &self.residency
    }

    fn structure_lock(&self) -> &Arc<StructureLock> {
        &self.structure
    }

    fn device_base(&self, handle: &RegionHandle) -> Option<Addr> {
        self.backend.device_base(self.ordinal, handle)
    }

    fn write_visible(
        &self,
        handle: &RegionHandle,
        offset: u64,
        bytes: &[u8],
    ) -> Result<Event, String> {
        self.backend.write_visible(self.ordinal, handle, offset, bytes)
    }

    fn write_kernel_table(&self, lane: &mut Lane, table: &[u8]) -> Result<u64, String> {
        self.backend.write_kernel_table(self.ordinal, lane, table)
    }
}

#[cfg(test)]
#[path = "test/index_services.rs"]
mod tests;
