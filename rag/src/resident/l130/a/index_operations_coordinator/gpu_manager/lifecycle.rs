//! Publishing, releasing and evicting segments; work in the gap between batches.
use std::sync::Arc;
use std::time::Instant;
use super::deps::*;
use super::state::{Inner, Record};
use super::types::*;
use super::GpuManager;
impl GpuManager {
    /// Makes a filled segment visible: table, mapping, manifest, eviction tracking, binds.
    pub fn publish_segment(
        &self,
        seg: LoadedSegment,
    ) -> Result<SegmentId, GpuManagerError> {
        let inner = &*self.inner;
        let _structure = inner.services.structure_lock().lock();
        let id = SegmentId(lock(&inner.state).next_id);
        let region = seg.segment.region;
        let first_slot = match inner.first_slot_of(&seg) {
            Ok(first) => first,
            Err(e) => {
                inner.abandon(&seg, true);
                return Err(e);
            }
        };
        let entry = match Entry::new(
            id,
            seg.view,
            seg.device_base,
            Arc::clone(&seg.states),
            Arc::clone(&seg.assignment),
        ) {
            Ok(entry) => entry,
            Err(e) => {
                inner.abandon(&seg, true);
                return Err(GpuManagerError::Mapping(e));
            }
        };
        let precheck = inner
            .services
            .mapping_store()
            .next()
            .and_then(|mut map| map.add(entry.clone()));
        if let Err(e) = precheck {
            inner.abandon(&seg, true);
            return Err(GpuManagerError::Mapping(e));
        }
        let table_epoch = match inner.publish_table(|table| table.add(region)) {
            Ok(epoch) => epoch,
            Err(e) => {
                inner.abandon(&seg, true);
                return Err(e);
            }
        };
        let map_epoch = match inner.publish_map(|map| map.add(entry.clone())) {
            Ok(epoch) => epoch,
            Err(e) => {
                inner.rollback_table(&seg.segment);
                inner.abandon(&seg, false);
                return Err(e);
            }
        };
        let manifest = inner
            .manifest_specs(&[], Some((id, &seg.view, seg.device_base)))
            .and_then(|specs| inner.publish_manifest(&specs));
        if let Err(e) = manifest {
            match inner
                .publish_map(|map| {
                    map.remove(id);
                    Ok(())
                })
            {
                Ok(_) => inner.rollback_table(&seg.segment),
                Err(re) => {
                    log::error!(
                        "gpu_manager: mapping rollback failed, region kept: {re}"
                    );
                }
            }
            inner.abandon(&seg, false);
            return Err(e);
        }
        let LoadedSegment {
            segment,
            view,
            device_base,
            states,
            assignment,
            bindings,
            writer,
        } = seg;
        {
            let mut state = lock(&inner.state);
            state
                .records
                .push(Record {
                    id,
                    segment,
                    view,
                    device_base,
                    states,
                    assignment,
                    writer,
                    first_slot,
                });
            state.next_id += 1;
            state.table_version += 1;
        }
        inner
            .services
            .eviction()
            .track(
                EvictKey(id.0),
                TrackSpec {
                    tier: Tier::Vram,
                    segment: id,
                    region,
                    size: region.len(),
                    reload_cost: region.len() as f64,
                    summary: false,
                },
                Instant::now(),
            );
        let updates = inner.services.updates();
        for binding in &bindings {
            updates
                .push_bind(
                    binding.entry,
                    SlotRef {
                        segment: id,
                        slot: binding.slot,
                    },
                    binding.stamp,
                );
        }
        {
            let mut state = lock(&inner.state);
            for binding in &bindings {
                state.in_flight.remove(&binding.entry.entry);
            }
            state.first_slots.remove(&region);
        }
        log::info!(
            "gpu_manager: segment {} published (table epoch {table_epoch}, map epoch {map_epoch}, \
             manifest generation {})",
            id.0, inner.services.manifests().current().generation()
        );
        Ok(id)
    }
    /// Removes a published segment and cleans up its residency.
    pub fn release_segment(
        &self,
        id: SegmentId,
    ) -> Result<ReleasedSegment, GpuManagerError> {
        let inner = &*self.inner;
        let _structure = inner.services.structure_lock().lock();
        let (segment, entry) = {
            let state = lock(&inner.state);
            let record = state
                .records
                .iter()
                .find(|r| r.id == id)
                .ok_or(GpuManagerError::UnknownSegment(id.0))?;
            let entry = Entry::new(
                    id,
                    record.view,
                    record.device_base,
                    Arc::clone(&record.states),
                    Arc::clone(&record.assignment),
                )
                .map_err(GpuManagerError::Mapping)?;
            (record.segment, entry)
        };
        let region = segment.region;
        let table_epoch = inner
            .publish_table(|table| {
                table.remove(&region);
            })?;
        let removed = inner
            .publish_map(|map| {
                map.remove(id);
                Ok(())
            });
        if let Err(e) = removed {
            inner.restore(&entry, region, false);
            return Err(e);
        }
        let manifest = inner
            .manifest_specs(&[id], None)
            .and_then(|specs| inner.publish_manifest(&specs));
        if let Err(e) = manifest {
            inner.restore(&entry, region, true);
            return Err(e);
        }
        inner.services.eviction().remove(EvictKey(id.0));
        let cleared = inner.cleanup_segment(id);
        lock(&inner.deferred).push((segment, table_epoch));
        log::info!(
            "gpu_manager: segment {} released, {} entries cleared", id.0, cleared.len()
        );
        Ok(ReleasedSegment {
            id,
            region,
            cleared,
        })
    }
    /// Idle and pressure eviction. `Eviction` removes the segments from table and map.
    pub fn evict_step(&self, now: Instant) -> EvictionOutcome {
        let inner = &*self.inner;
        let _structure = inner.services.structure_lock().lock();
        let eviction = inner.services.eviction();
        eviction.reclaim(inner.services.table_swap().min_reader_epoch());
        let outcome = eviction.tick(now);
        inner.after_eviction(&outcome);
        outcome
    }
    /// Evicts until `need` bytes fit; `Err(Exhausted)` changes nothing.
    pub fn make_room(&self, need: u64) -> Result<EvictionOutcome, EvictionError> {
        let inner = &*self.inner;
        let _structure = inner.services.structure_lock().lock();
        let eviction = inner.services.eviction();
        let swap = inner.services.table_swap();
        eviction.reclaim(swap.min_reader_epoch());
        let outcome = eviction.make_room(Tier::Vram, need)?;
        inner.after_eviction(&outcome);
        eviction.reclaim(swap.min_reader_epoch());
        Ok(outcome)
    }
    /// Call once per gap, after the Communication-Slot completion.
    pub fn run_gap(&self) -> GapReport {
        let inner = &*self.inner;
        let retired = std::mem::take(&mut lock(&inner.state).retired_slots);
        let mut keep = Vec::new();
        let mut slots_hidden = 0;
        {
            let directory = lock(inner.services.residency());
            let updates = inner.services.updates();
            for (slot, id) in retired {
                if updates.contains(id) {
                    keep.push((slot, id));
                    continue;
                }
                if directory.get(id).map(|r| r.slot) == Some(slot) {
                    continue;
                }
                let states = lock(&inner.state)
                    .records
                    .iter()
                    .find(|r| r.id == slot.segment)
                    .map(|r| Arc::clone(&r.states));
                let Some(states) = states else {
                    continue;
                };
                match states.invalidate(slot.slot) {
                    Ok(()) => slots_hidden += 1,
                    Err(e) => {
                        log::warn!("gpu_manager: slot {} not hidden: {e}", slot.slot.0)
                    }
                }
            }
        }
        lock(&inner.state).retired_slots.extend(keep);
        GapReport {
            slots_hidden,
            reclaimed: inner.reclaim_deferred(),
        }
    }
}
impl Inner {
    /// The permanent first slot of a segment; segments of unknown origin get a new range.
    fn first_slot_of(&self, seg: &LoadedSegment) -> Result<u32, GpuManagerError> {
        let mut state = lock(&self.state);
        if let Some(first) = state.first_slots.get(&seg.segment.region) {
            return Ok(*first);
        }
        let first = state.next_first_slot;
        let end = first + u64::from(seg.view.slot_count());
        if end > u64::from(u32::MAX) {
            return Err(GpuManagerError::SegmentLimit);
        }
        state.next_first_slot = end;
        state.first_slots.insert(seg.segment.region, first as u32);
        Ok(first as u32)
    }
    /// Drops a segment that was not published; `release` hands it back to the arena.
    fn abandon(&self, seg: &LoadedSegment, release: bool) {
        {
            let mut state = lock(&self.state);
            for binding in &seg.bindings {
                state.in_flight.remove(&binding.entry.entry);
            }
            state.first_slots.remove(&seg.segment.region);
        }
        if release {
            if let Err(e) = self.services.arena().release(&seg.segment, Readers::None) {
                log::warn!("gpu_manager: segment not released: {e}");
            }
        }
    }
    /// Takes the region out of the table again; it is returned once the epoch is unreferenced.
    fn rollback_table(&self, segment: &Segment) {
        let region = segment.region;
        match self
            .publish_table(|table| {
                table.remove(&region);
            })
        {
            Ok(epoch) => lock(&self.deferred).push((*segment, epoch)),
            Err(e) => {
                log::error!("gpu_manager: table rollback failed, region kept: {e}");
            }
        }
    }
    /// Puts a segment back into table and, if it was removed there, into the map.
    fn restore(&self, entry: &Entry, region: RegionHandle, map_removed: bool) {
        if let Err(e) = self.publish_table(|table| table.add(region)) {
            log::error!("gpu_manager: table restore failed: {e}");
        }
        if map_removed {
            if let Err(e) = self.publish_map(|map| map.add(entry.clone())) {
                log::error!("gpu_manager: mapping restore failed: {e}");
            }
        }
    }
    /// Removes the record and the residency of its entries; returns the cleared entries.
    pub(super) fn cleanup_segment(&self, id: SegmentId) -> Vec<EntryId> {
        let cleared: Vec<EntryId> = {
            let directory = lock(self.services.residency());
            let ids: Vec<EntryId> = directory
                .iter()
                .into_iter()
                .filter(|(_, resident)| resident.slot.segment == id)
                .map(|(entry, _)| entry)
                .collect();
            for entry in &ids {
                directory.remove(*entry);
            }
            ids
        };
        let mut state = lock(&self.state);
        state.records.retain(|r| r.id != id);
        state.table_version += 1;
        state.retired_slots.retain(|(slot, _)| slot.segment != id);
        for entry in &cleared {
            state.in_flight.remove(entry);
        }
        cleared
    }
    /// Cleans up the evicted segments of this manager and republishes the manifest.
    fn after_eviction(&self, outcome: &EvictionOutcome) {
        let mut evicted = 0;
        for decision in &outcome.decisions {
            let id = SegmentId(decision.victim.0);
            let known = lock(&self.state).records.iter().any(|r| r.id == id);
            if known {
                self.cleanup_segment(id);
                evicted += 1;
            }
        }
        if evicted == 0 {
            return;
        }
        let result = self
            .manifest_specs(&[], None)
            .and_then(|specs| self.publish_manifest(&specs));
        if let Err(e) = result {
            log::error!("gpu_manager: manifest after eviction not published: {e}");
        }
    }
    /// Reclaims memory that readers no longer reference; returns the number of regions.
    pub(super) fn reclaim_deferred(&self) -> usize {
        let minimum = self.services.table_swap().min_reader_epoch();
        let mut count = self.services.eviction().reclaim(minimum);
        count += self.services.manifests().reclaim().len();
        let ready: Vec<(Segment, Epoch)> = {
            let mut deferred = lock(&self.deferred);
            let (ready, keep) = std::mem::take(&mut *deferred)
                .into_iter()
                .partition(|(_, epoch)| *epoch <= minimum);
            *deferred = keep;
            ready
        };
        for (segment, _) in ready {
            match self.services.arena().release(&segment, Readers::None) {
                Ok(()) => count += 1,
                Err(e) => log::warn!("gpu_manager: deferred segment not released: {e}"),
            }
        }
        count
    }
}
#[cfg(test)]
#[path = "test/lifecycle.rs"]
mod tests;
