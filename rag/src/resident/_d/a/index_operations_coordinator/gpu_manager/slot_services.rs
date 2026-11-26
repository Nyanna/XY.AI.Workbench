//! Services for the Communication-Slot and the hit resolution.
use std::sync::Arc;
use super::deps::*;
use super::state::Inner8;
use super::GpuManager;
/// Update-queue adapter for the Communication-Slot; the only caller of `UpdateQueue::apply`.
pub struct SlotUpdates {
    updates: Arc<UpdateQueue>,
    store: Arc<MappingStore>,
    inner: Arc<Inner8>,
}
impl UpdateApplier for SlotUpdates {
    fn depth(&self) -> usize {
        self.updates.depth()
    }
    fn is_pending(&self, entry: EntryId) -> bool {
        self.updates.contains(entry)
    }
    fn apply(&mut self, residency: &ResidencyDirectory) -> UpdateOutcome {
        let candidates = std::mem::take(&mut *lock(&self.inner.mirror_candidates));
        for (id, stamp) in candidates {
            self.inner.consider_mirror(residency, id, stamp);
        }
        let map = self.store.current();
        let applied = self.updates.apply(residency, &map);
        for warning in self.updates.take_warnings() {
            log::warn!(target : "update_queue", "{warning:?}");
        }
        UpdateOutcome { applied }
    }
}
struct ManagerMappingSource {
    inner: Arc<Inner8>,
}
impl MappingSource for ManagerMappingSource {
    fn snapshot(&self) -> MappingSnapshot {
        let previous = lock(&self.inner.state).previous_manifest.clone();
        MappingSnapshot {
            current: Arc::new(self.inner.services.manifests().current()),
            previous,
        }
    }
    fn lock_structure(&self) -> Box<dyn StructureHold + '_> {
        Box::new(self.inner.services.structure_lock().lock())
    }
    fn reclaim_deferred(&self) -> usize {
        self.inner.reclaim_deferred()
    }
}
struct ManagerSegmentDirectory {
    store: Arc<MappingStore>,
}
impl SegmentDirectory for ManagerSegmentDirectory {
    fn binding(&self, id: SegmentId) -> Option<SegmentBinding> {
        let map = self.store.current();
        let entry = map.get(id)?;
        Some(SegmentBinding {
            assignment: Arc::clone(entry.assignment()),
            states: Arc::clone(entry.states()),
        })
    }
}
impl GpuManager {
    pub fn slot_updates(&self) -> SlotUpdates {
        SlotUpdates {
            updates: Arc::clone(self.inner.services.updates()),
            store: Arc::clone(self.inner.services.mapping_store()),
            inner: Arc::clone(&self.inner),
        }
    }
    pub fn mapping_source(&self) -> Arc<dyn MappingSource> {
        Arc::new(ManagerMappingSource {
            inner: Arc::clone(&self.inner),
        })
    }
    pub fn segment_directory(&self) -> Arc<dyn SegmentDirectory> {
        Arc::new(ManagerSegmentDirectory {
            store: Arc::clone(self.inner.services.mapping_store()),
        })
    }
}
#[cfg(test)]
#[path = "test/slot_services.rs"]
mod tests;
