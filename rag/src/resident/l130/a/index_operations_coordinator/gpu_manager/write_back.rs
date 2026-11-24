//! Write-back of processed GPU results and the index wrapper for the Communication-Slot.

use std::sync::{Arc, TryLockError};

use super::deps::*;
use super::state::{entry_ref, Inner};
use super::types::*;
use super::GpuManager;

impl GpuManager {
    /// Writes processed results into the index and registers them as resident.
    pub fn write_back(&self, items: &[MirrorItem]) -> Result<MirrorReport, GpuManagerError> {
        let inner = &*self.inner;
        let index = inner.index()?;
        let writer = {
            let mut slot = lock(&inner.writer);
            Arc::clone(slot.get_or_insert_with(|| Arc::new(IndexWriter::new(Arc::clone(&index)))))
        };
        let updates = inner.services.updates();
        let mut report = MirrorReport::default();
        for item in items {
            if item.vector.len() % inner.row_bytes != 0 {
                report
                    .failed
                    .push((
                        item.id,
                        GpuManagerError::InvalidVector {
                            len: item.vector.len(),
                            row_bytes: inner.row_bytes,
                        },
                    ));
                continue;
            }
            let payload = PayloadRef { vector: &item.vector, meta: &item.meta };
            let identity = match writer.write(item.id, payload) {
                Ok(identity) => identity,
                Err(e) => {
                    report.failed.push((item.id, GpuManagerError::IndexWrite(e)));
                    continue;
                }
            };
            report.written.push((item.id, identity.stamp));
            match item.placed {
                Some(slot) => {
                    let previous = lock(inner.services.residency()).get(item.id);
                    if let Some(previous) = previous {
                        if previous.slot != slot {
                            inner.retire(previous.slot, item.id);
                        }
                    }
                    updates.push_bind(entry_ref(item.id), slot, identity.stamp);
                }
                None => {
                    lock(inner.services.residency()).set_known_stamp(item.id, identity.stamp);
                }
            }
        }
        Ok(report)
    }

    /// The index wrapped for the Communication-Slot: GPU-built entries become resident with
    /// the stamp of their own write.
    pub fn mirrored_index(&self) -> Result<Arc<dyn Index>, GpuManagerError> {
        let base = self.inner.index()?;
        Ok(Arc::new(MirroredIndex { inner: Arc::clone(&self.inner), base }))
    }
}

impl Inner {
    /// Called after a successful index write through the wrapper. The residency mutex may be
    /// held by the caller of the wrapper, so it is never waited for: a busy mutex defers the
    /// decision to `SlotUpdates::apply`, which receives the directory.
    fn note_write(&self, id: EntryId, stamp: Stamp) {
        match self.services.residency().try_lock() {
            Ok(directory) => self.consider_mirror(&directory, id, stamp),
            Err(TryLockError::Poisoned(poisoned)) => {
                self.consider_mirror(&poisoned.into_inner(), id, stamp);
            }
            Err(TryLockError::WouldBlock) => {
                if self.find_valid_slot(id).is_some() {
                    lock(&self.mirror_candidates).push((id, stamp));
                }
            }
        }
    }
}

/// Forwards every call; adds the bind of GPU-built entries after a write.
struct MirroredIndex {
    inner: Arc<Inner>,
    base: Arc<dyn Index>,
}

impl Index for MirroredIndex {
    fn dtype(&self) -> DType {
        self.base.dtype()
    }

    fn access_methods(&self) -> &[AccessMethod] {
        self.base.access_methods()
    }

    fn stamp(&self, id: EntryId) -> Option<Stamp> {
        self.base.stamp(id)
    }

    fn stamps(&self) -> Vec<(EntryId, Stamp)> {
        self.base.stamps()
    }

    fn read(&self, id: EntryId) -> Payload {
        self.base.read(id)
    }

    fn write(&self, id: EntryId, payload: PayloadRef<'_>) -> Result<Stamp, IndexError> {
        let stamp = self.base.write(id, payload)?;
        self.inner.note_write(id, stamp);
        Ok(stamp)
    }

    fn write_many(
        &self,
        items: &[(EntryId, PayloadRef<'_>)],
    ) -> Result<Vec<(EntryId, Stamp)>, IndexError> {
        let written = self.base.write_many(items)?;
        for (id, stamp) in &written {
            self.inner.note_write(*id, *stamp);
        }
        Ok(written)
    }

    fn delete(&self, id: EntryId) -> Result<Option<Stamp>, IndexError> {
        self.base.delete(id)
    }

    fn subscribe(&self) -> StampObserver {
        self.base.subscribe()
    }

    fn summary(&self, node: NodeId) -> Result<Summary, IndexError> {
        self.base.summary(node)
    }

    fn children(&self, node: NodeId) -> Result<Vec<NodeId>, IndexError> {
        self.base.children(node)
    }

    fn root(&self) -> Result<NodeId, IndexError> {
        self.base.root()
    }

    fn cell_entries(&self, cell: CellId) -> Result<Vec<EntryId>, IndexError> {
        self.base.cell_entries(cell)
    }

    fn byte_ranges(&self, first: EntryId, last: EntryId) -> Result<Vec<ByteRange>, IndexError> {
        self.base.byte_ranges(first, last)
    }
}

#[cfg(test)]
#[path = "test/write_back.rs"]
mod tests;
