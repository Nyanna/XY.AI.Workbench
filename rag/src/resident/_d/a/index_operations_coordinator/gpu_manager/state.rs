//! Shared state of one GPU manager and the helpers used by several operations.
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};
use super::deps::*;
use super::services::GpuServices;
use super::types::*;
pub(super) const PUBLISH_ATTEMPTS: usize = 3;
/// A published segment.
pub(super) struct Record {
    pub id: SegmentId,
    pub segment: Segment,
    pub view: SegmentView,
    pub device_base: Addr,
    pub states: Arc<SlotStates>,
    pub assignment: Arc<SlotAssignment>,
    pub writer: Option<Arc<SegmentWriter>>,
    /// Permanent; global slot = `first_slot + slot`.
    pub first_slot: u32,
}
/// A segment that is being filled and not yet reported.
pub(super) struct Unpublished {
    pub segment: Segment,
    pub view: SegmentView,
    pub device_base: Addr,
    pub states: Arc<SlotStates>,
    pub assignment: Arc<SlotAssignment>,
    pub writer: Arc<SegmentWriter>,
    pub first_slot: u32,
    pub bindings: Vec<LoadedBinding>,
}
#[derive(Default)]
pub(super) struct State10 {
    /// Ascending by segment ID.
    pub records: Vec<Record>,
    pub unpublished: Vec<Unpublished>,
    /// Entries placed in an unpublished segment.
    pub in_flight: HashSet<EntryId>,
    /// Slots to hide in the next gap.
    pub retired_slots: Vec<(SlotRef, EntryId)>,
    pub next_id: u64,
    pub next_first_slot: u64,
    /// First slots of reported segments that are not published yet.
    pub first_slots: HashMap<RegionHandle, u32>,
    /// RAM table version; grows with every change of the published records.
    pub table_version: u64,
    pub previous_manifest: Option<Arc<Manifest>>,
}
pub(super) struct Inner8 {
    pub services: Arc<dyn GpuServices>,
    pub index: Arc<dyn LazyIndex>,
    pub resolver: Arc<ScopeResolverRegistry>,
    pub cell_selector: Option<Arc<dyn CellSelector>>,
    pub queue: LoadedQueue,
    pub cfg: GpuManagerConfig,
    pub slot_bytes: u64,
    pub row_bytes: usize,
    /// The records lock; a leaf in the lock order.
    pub state: Mutex<State10>,
    /// Serializes slot placement of `ensure` and `reconcile_iteration`.
    pub place_lock: Mutex<()>,
    /// Segments to hand back to the arena once their table epoch is unreferenced.
    pub deferred: Mutex<Vec<(Segment, Epoch)>>,
    pub sync: Mutex<Option<VramTable>>,
    pub reconciler: Mutex<Option<Reconciler>>,
    pub writer: Mutex<Option<Arc<IndexWriter>>>,
    /// Writes seen while the residency mutex was busy; resolved in `SlotUpdates::apply`.
    pub mirror_candidates: Mutex<Vec<(EntryId, Stamp)>>,
}
pub(super) fn entry_ref(id: EntryId) -> EntryRef {
    EntryRef { entry: id, section: 0 }
}
impl Inner8 {
    pub(super) fn index(&self) -> Result<Arc<dyn Index>, GpuManagerError> {
        self.index.get().map_err(GpuManagerError::Index)
    }
    /// Splits `ids` into entries that are present (resident, in flight or waiting for their
    /// bind) and missing ones. Both lists keep the input order.
    pub(super) fn classify(
        &self,
        ids: impl Iterator<Item = EntryId>,
    ) -> (Vec<EntryId>, Vec<EntryId>) {
        let directory = lock(self.services.residency());
        let state = lock(&self.state);
        let updates = self.services.updates();
        let mut present = Vec::new();
        let mut missing = Vec::new();
        for id in ids {
            let binding = matches!(
                updates.get(id), Some(u) if u.kind == UpdateKind::Bind
            );
            if directory.get(id).is_some() || state.in_flight.contains(&id) || binding {
                present.push(id);
            } else {
                missing.push(id);
            }
        }
        (present, missing)
    }
    pub(super) fn retire(&self, slot: SlotRef, id: EntryId) {
        lock(&self.state).retired_slots.push((slot, id));
    }
    /// Newest published slot of the entry whose valid bit is set.
    pub(super) fn find_valid_slot(&self, id: EntryId) -> Option<SlotRef> {
        let state = lock(&self.state);
        for record in state.records.iter().rev() {
            if let Some(slot) = record.assignment.slot_of_entry(entry_ref(id)) {
                if record.states.is_valid(slot) {
                    return Some(SlotRef {
                        segment: record.id,
                        slot,
                    });
                }
            }
        }
        None
    }
    /// Queues a Bind for an entry whose slot was filled on the device and which was just
    /// written to the index with `stamp`.
    pub(super) fn consider_mirror(
        &self,
        directory: &ResidencyDirectory,
        id: EntryId,
        stamp: Stamp,
    ) {
        let updates = self.services.updates();
        if directory.get(id).is_some() || updates.contains(id) {
            return;
        }
        if let Some(slot) = self.find_valid_slot(id) {
            updates.push_bind(entry_ref(id), slot, stamp);
        }
    }
    /// Specs of all published records except `skip`, plus `extra`.
    pub(super) fn manifest_specs(
        &self,
        skip: &[SegmentId],
        extra: Option<(SegmentId, &SegmentView, Addr)>,
    ) -> Result<Vec<SegmentSpec>, GpuManagerError> {
        let state = lock(&self.state);
        let mut specs = Vec::with_capacity(state.records.len() + 1);
        for record in state.records.iter().filter(|r| !skip.contains(&r.id)) {
            let spec = SegmentSpec::from_view(
                    record.id,
                    &record.view,
                    record.device_base,
                )
                .map_err(GpuManagerError::Slot)?;
            specs.push(spec);
        }
        if let Some((id, view, base)) = extra {
            specs
                .push(
                    SegmentSpec::from_view(id, view, base)
                        .map_err(GpuManagerError::Slot)?,
                );
        }
        Ok(specs)
    }
    /// Publishes a successor of the current manifest holding exactly `specs`.
    pub(super) fn publish_manifest(
        &self,
        specs: &[SegmentSpec],
    ) -> Result<(), GpuManagerError> {
        let store = self.services.manifests();
        let mut attempt = 1;
        loop {
            let base = store.current();
            let next = store
                .successor(&base, specs.to_vec())
                .map_err(GpuManagerError::Manifest)?;
            match store.publish(next) {
                Ok(previous) => {
                    lock(&self.state).previous_manifest = Some(Arc::new(previous));
                    return Ok(());
                }
                Err(
                    ManifestError::StaleGeneration { .. },
                ) if attempt < PUBLISH_ATTEMPTS => {
                    attempt += 1;
                }
                Err(e) => return Err(GpuManagerError::Manifest(e)),
            }
        }
    }
    /// Publishes a successor of the current table after `edit`; retries on a stale epoch.
    pub(super) fn publish_table(
        &self,
        mut edit: impl FnMut(&mut SegmentTable),
    ) -> Result<Epoch, GpuManagerError> {
        let swap = self.services.table_swap();
        let mut attempt = 1;
        loop {
            let mut table = swap.next_table();
            edit(&mut table);
            match swap.publish(table) {
                Ok(epoch) => return Ok(epoch),
                Err(
                    TableSwapError::StaleMapping { .. },
                ) if attempt < PUBLISH_ATTEMPTS => {
                    attempt += 1;
                }
                Err(e) => return Err(GpuManagerError::Publish(e)),
            }
        }
    }
    /// Publishes a successor of the current map after `edit`; retries on a stale epoch.
    pub(super) fn publish_map(
        &self,
        mut edit: impl FnMut(&mut AddressMap) -> Result<(), MapError>,
    ) -> Result<Epoch, GpuManagerError> {
        let store = self.services.mapping_store();
        let mut attempt = 1;
        loop {
            let mut map = store.next().map_err(GpuManagerError::Mapping)?;
            edit(&mut map).map_err(GpuManagerError::Mapping)?;
            match store.publish(map) {
                Ok(epoch) => return Ok(epoch),
                Err(MapError::StaleMapping { .. }) if attempt < PUBLISH_ATTEMPTS => {
                    attempt += 1;
                }
                Err(e) => return Err(GpuManagerError::Mapping(e)),
            }
        }
    }
}
impl Drop for Inner8 {
    fn drop(&mut self) {
        let state = self.state.get_mut().unwrap_or_else(|e| e.into_inner());
        for open in std::mem::take(&mut state.unpublished) {
            if let Err(e) = self.services.arena().release(&open.segment, Readers::None) {
                log::warn!("gpu_manager: unpublished segment not released: {e}");
            }
        }
    }
}
