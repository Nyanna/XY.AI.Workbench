//! Reconcile iterations against the index.
use std::sync::TryLockError;
use super::deps::*;
use super::placement::Placement;
use super::state::entry_ref;
use super::types::*;
use super::GpuManager;
const LOAD_CHUNK: usize = 64;
impl GpuManager {
    /// `Ok(None)`: the index no longer exists.
    pub fn reconcile_iteration(
        &self,
    ) -> Result<Option<ReconcileOutcome>, GpuManagerError> {
        let inner = &*self.inner;
        let index = inner.index()?;
        let updates = inner.services.updates();
        let (reconciliation, removed, reload) = {
            let mut guard = lock(&inner.reconciler);
            let reconciler = guard.get_or_insert_with(|| Reconciler::new(&index));
            let directory = lock(inner.services.residency());
            let Some(reconciliation) = reconciler.reconcile(&directory) else {
                return Ok(None);
            };
            let removed: Vec<(EntryId, Option<Resident2>)> = reconciliation
                .removed
                .iter()
                .map(|id| (*id, directory.get(*id)))
                .collect();
            let reload: Vec<(EntryId, Option<Resident2>)> = reconciliation
                .reload
                .iter()
                .map(|id| (*id, directory.get(*id)))
                .collect();
            (reconciliation, removed, reload)
        };
        let mut outcome = ReconcileOutcome {
            reconciliation,
            queued_unbinds: 0,
            reloaded: 0,
            skipped_pending: 0,
            failed: 0,
        };
        for (id, resident) in removed {
            if let Some(resident) = resident {
                self.queue_unbind(id, resident);
                outcome.queued_unbinds += 1;
            }
        }
        let mut exhausted = false;
        for chunk in reload.chunks(LOAD_CHUNK) {
            let mut loaded = Vec::with_capacity(chunk.len());
            for (id, resident) in chunk {
                let Some(resident) = resident else {
                    continue;
                };
                if updates.contains(*id) {
                    outcome.skipped_pending += 1;
                    continue;
                }
                if exhausted {
                    outcome.failed += 1;
                    continue;
                }
                loaded.push((*id, *resident, load_entry(&*index, *id)));
            }
            let _place = lock(&inner.place_lock);
            for (id, resident, entry) in loaded {
                let Some(entry) = entry else {
                    self.queue_unbind(id, resident);
                    outcome.queued_unbinds += 1;
                    continue;
                };
                let vector: &[u8] = &entry.payload.vector;
                let Ok(tokens) = inner.validate_vector(vector.len()) else {
                    outcome.failed += 1;
                    continue;
                };
                match inner.place(id, vector, tokens, entry.stamp, false) {
                    Ok(Placement::Placed(placed)) => {
                        outcome.reloaded += 1;
                        if placed.segment != Some(resident.slot.segment) {
                            inner.retire(resident.slot, id);
                        }
                    }
                    Ok(Placement::AlreadyPresent) => {}
                    Err(GpuManagerError::Arena(_)) => {
                        exhausted = true;
                        outcome.failed += 1;
                    }
                    Err(e) => {
                        log::warn!(
                            "gpu_manager reconcile: entry {} not reloaded: {e}", id.0
                        );
                        outcome.failed += 1;
                    }
                }
            }
        }
        {
            let _place = lock(&inner.place_lock);
            inner.report_unpublished();
        }
        Ok(Some(outcome))
    }
    /// Is an unseen index signal pending? Never blocks.
    pub fn signal_pending(&self) -> bool {
        match self.inner.reconciler.try_lock() {
            Ok(guard) => guard.as_ref().is_some_and(|r| r.pending()),
            Err(TryLockError::Poisoned(poisoned)) => {
                poisoned.into_inner().as_ref().is_some_and(|r| r.pending())
            }
            Err(TryLockError::WouldBlock) => true,
        }
    }
    fn queue_unbind(&self, id: EntryId, resident: Resident2) {
        self.inner
            .services
            .updates()
            .push_unbind(entry_ref(id), resident.slot, resident.stamp);
        self.inner.retire(resident.slot, id);
    }
}
#[cfg(test)]
#[path = "test/reconcile_pass.rs"]
mod tests;
