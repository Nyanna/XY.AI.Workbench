use std::sync::Arc;

use super::deps::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GpuDeviceInfo {
    pub ordinal: u32,
    pub name: String,
    pub compute_capability: ComputeCapability,
    /// Streaming multiprocessors (compute units).
    pub multiprocessors: u32,
    pub total_memory: u64,
}

/// The only place that touches CUDA. Implemented by the CUDA binding.
pub trait GpuBackend: Send + Sync {
    fn devices(&self) -> Result<Vec<GpuDeviceInfo>, String>;
    /// Retains the primary context of the device. Called at most once per device by `GpuDeviceManager`.
    fn retain_primary_context(&self, ordinal: u32) -> Result<(), String>;
    fn probe(&self, ordinal: u32) -> Arc<dyn GpuProbe>;
    /// New stream in the primary context of the device.
    fn create_stream(
        &self,
        ordinal: u32,
        priority: Priority,
    ) -> Result<Box<dyn StreamBackend>, String>;
    fn memory_query(&self, ordinal: u32) -> Arc<dyn MemoryQuery>;
    /// Reserves the memory of an arena once: VRAM for device arenas, pinned host memory for
    /// Staging and MappedHost. Dropping the region frees it. Offsets in the region start at 0.
    fn reserve_arena(&self, ordinal: u32, arena: Arena) -> Result<AccessRegion, String>;
    /// Device address of byte 0 of a region of the IndexSegments arena.
    fn device_base(&self, ordinal: u32, handle: &RegionHandle) -> Option<Addr>;
    /// Copies `bytes` into the region at `offset` on the CopyH2D path. The event completes when
    /// the content is visible to the device.
    fn write_visible(
        &self,
        ordinal: u32,
        handle: &RegionHandle,
        offset: u64,
        bytes: &[u8],
    ) -> Result<Event, String>;
    /// Writes the kernel table (`SegmentEntry[]` bytes) to VRAM, ordered before work later
    /// submitted on `lane`. Returns the device address of the table.
    fn write_kernel_table(
        &self,
        ordinal: u32,
        lane: &mut Lane,
        table: &[u8],
    ) -> Result<u64, String>;
    fn table_sink(&self, ordinal: u32) -> Arc<dyn TableSink>;
    fn root_slot(&self, ordinal: u32) -> Arc<dyn RootSlot>;
}
