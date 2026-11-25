//! Memory manager of the GPU-resident part of one index: translates between entries and
//! kernel slots and keeps both in sync.
use std::sync::{Arc, Mutex};
mod deps;
mod ensure;
mod lifecycle;
mod placement;
mod reconcile_pass;
mod services;
mod slot_services;
mod state;
mod types;
mod vram_sync;
mod write_back;
pub mod load_aggregation;
pub mod scope_resolution;
#[cfg(test)]
#[path = "test/fixture.rs"]
pub(crate) mod fixture;
pub use services::{GpuServices, StructureGuard, StructureLock};
pub use slot_services::SlotUpdates;
pub use types::{
    EnsureReport, GapReport, GpuIndexStats, GpuManagerConfig, GpuManagerError,
    LoadedBinding, LoadedQueue, LoadedSegment, MirrorItem, MirrorReport,
    ReconcileOutcome, ReleasedSegment, VramTable,
};
use deps::*;
use state::{Inner, State};
/// Memory manager of one index. All methods take `&self` and may be called from any thread.
pub struct GpuManager {
    inner: Arc<Inner>,
}
static_assertions::assert_impl_all!(GpuManager : Send, Sync);
impl GpuManager {
    /// Validates only. Loads nothing, allocates nothing and does not call `LazyIndex::get`.
    pub fn new(
        services: Arc<dyn GpuServices>,
        index: Arc<dyn LazyIndex>,
        resolver: Arc<ScopeResolverRegistry>,
        cell_selector: Option<Arc<dyn CellSelector>>,
        queue: LoadedQueue,
        cfg: GpuManagerConfig,
    ) -> Result<GpuManager, GpuManagerError> {
        let invalid = |text: String| GpuManagerError::InvalidConfig(text);
        if cfg.dim == 0 {
            return Err(invalid("dimension must be non-zero".into()));
        }
        if !matches!(cfg.dtype, DType::F32 | DType::F16 | DType::BF16) {
            return Err(invalid(format!("unsupported dtype {:?}", cfg.dtype)));
        }
        let slot_bytes = u64::from(
            slot_size(cfg.class, cfg.dim, cfg.dtype).map_err(|e| invalid(e.to_string()))?,
        );
        if cfg.segment_bytes < slot_bytes {
            return Err(
                invalid(
                    format!(
                        "segment of {} bytes is shorter than a slot of {slot_bytes} bytes",
                        cfg.segment_bytes
                    ),
                ),
            );
        }
        let row_bytes = cfg.dim as usize * cfg.dtype.size_in_bytes();
        Ok(GpuManager {
            inner: Arc::new(Inner {
                services,
                index,
                resolver,
                cell_selector,
                queue,
                cfg,
                slot_bytes,
                row_bytes,
                state: Mutex::new(State::default()),
                place_lock: Mutex::new(()),
                deferred: Mutex::new(Vec::new()),
                sync: Mutex::new(None),
                reconciler: Mutex::new(None),
                writer: Mutex::new(None),
                mirror_candidates: Mutex::new(Vec::new()),
            }),
        })
    }
    /// IDs of the published segments, ascending.
    pub fn segment_ids(&self) -> Vec<SegmentId> {
        lock(&self.inner.state).records.iter().map(|r| r.id).collect()
    }
    pub fn stats(&self) -> GpuIndexStats {
        GpuIndexStats {
            segments: self.segment_count(),
            resident_bytes: self.resident_bytes(),
            pending_updates: self.pending_updates(),
        }
    }
    pub fn segment_count(&self) -> usize {
        lock(&self.inner.state).records.len()
    }
    /// Live bytes of the eviction view.
    pub fn resident_bytes(&self) -> u64 {
        self.inner.services.eviction().bytes(Tier::Vram)
    }
    pub fn pending_updates(&self) -> usize {
        self.inner.services.updates().depth()
    }
    pub fn slot_of(&self, id: EntryId) -> Option<SlotRef> {
        let resident = lock(self.inner.services.residency()).get(id)?;
        Some(resident.slot)
    }
    pub fn global_slot_of(&self, id: EntryId) -> Option<u32> {
        let slot = self.slot_of(id)?;
        let state = lock(&self.inner.state);
        let record = state.records.iter().find(|r| r.id == slot.segment)?;
        record.first_slot.checked_add(slot.slot.0)
    }
    pub fn entry_at_global_slot(&self, slot: u32) -> Option<EntryRef> {
        let (assignment, local) = {
            let state = lock(&self.inner.state);
            let record = state
                .records
                .iter()
                .find(|r| {
                    slot >= r.first_slot && slot - r.first_slot < r.view.slot_count()
                })?;
            (Arc::clone(&record.assignment), slot - record.first_slot)
        };
        assignment.lookup(Slot(local))
    }
}
#[cfg(test)]
#[path = "test/gpu_manager.rs"]
mod tests;
