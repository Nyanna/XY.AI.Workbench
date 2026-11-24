//! Services the GPU sub-engine provides for one attached index.

use std::sync::{Arc, Mutex, MutexGuard};

use super::deps::*;

/// The one structure lock of the sub-engine. Not reentrant.
#[derive(Debug, Default)]
pub struct StructureLock {
    inner: Mutex<()>,
}

impl StructureLock {
    pub fn new() -> StructureLock {
        StructureLock { inner: Mutex::new(()) }
    }

    /// Recovers a poisoned lock.
    pub fn lock(&self) -> StructureGuard<'_> {
        StructureGuard { _guard: lock(&self.inner) }
    }
}

pub struct StructureGuard<'a> {
    _guard: MutexGuard<'a, ()>,
}

impl StructureHold for StructureGuard<'_> {}

/// Implemented by the GPU sub-engine, once per attached index.
pub trait GpuServices: Send + Sync {
    /// Arena `IndexSegments`.
    fn arena(&self) -> &SegmentArena<Arc<Allocator>>;
    fn table_swap(&self) -> &Arc<TableSwap>;
    fn mapping_store(&self) -> &Arc<MappingStore>;
    fn manifests(&self) -> &Arc<ManifestStore>;
    /// `Tier::Vram` is used.
    fn eviction(&self) -> &Arc<Eviction>;
    fn updates(&self) -> &Arc<UpdateQueue>;
    /// Shared with the Communication-Slot, which holds the mutex for a whole pass.
    fn residency(&self) -> &Arc<Mutex<ResidencyDirectory>>;
    fn structure_lock(&self) -> &Arc<StructureLock>;
    /// Device address of byte 0 of the region (the segment base).
    fn device_base(&self, handle: &RegionHandle) -> Option<Addr>;
    /// Writes `bytes` at `offset` (relative to byte 0 of `handle`). The returned event
    /// completes when the content is visible to the device.
    fn write_visible(
        &self,
        handle: &RegionHandle,
        offset: u64,
        bytes: &[u8],
    ) -> Result<Event, String>;
    /// Writes `table` (`SegmentEntry[]` bytes) to VRAM, ordered before work later submitted
    /// on `lane`. Returns the device address of the written table.
    fn write_kernel_table(&self, lane: &mut Lane, table: &[u8]) -> Result<u64, String>;
}
