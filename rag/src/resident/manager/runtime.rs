//! Owner of one index operation: ties the manager, the table swap, the mapping store and the
//! queue of loaded segments together and drives them in the order the mutation discipline needs.
use std::sync::Arc;
use std::time::Instant;
use parking_lot::Mutex;
use crate::resident::backend::ComputeBackend;
use crate::resident::error::{IndexError, ManagerError};
use crate::resident::index::EntrySet;
use crate::resident::kernel::{KernelHit, WrittenRange};
use crate::resident::manager::arena::IndexArena;
use crate::resident::manager::evict::{Eviction, Occupancy};
use crate::resident::manager::residency::{Access, LoadOutcome};
use crate::resident::manager::slot::{Hit, MappingStore, RangeSource};
use crate::resident::manager::streamed::{index_error, GpuIndexManager, SegmentKey};
use crate::resident::manager::swap::{Snapshot, SwapManager, TableSwap};
use crate::resident::manager::sync::ScanOutcome;
use crate::resident::manager::transfer::LoadedQueue;
const SLOT_RETRIES: u32 = 3;
/// `loaded` must be the sink of the manager's transfer: every segment the transfer reports is
/// published here, so no caller has to remember to do it.
pub struct IndexRuntime<'a, 'b, B: ?Sized> {
    manager: GpuIndexManager<B>,
    arena: &'a IndexArena<'b>,
    swap: SwapManager<'a, 'b>,
    store: MappingStore,
    loaded: Arc<LoadedQueue>,
    /// Table and mapping epochs advance together, so structural changes are serialized.
    structural: Mutex<()>,
}
impl<'a, 'b, B: ComputeBackend + ?Sized> IndexRuntime<'a, 'b, B> {
    pub fn new(
        manager: GpuIndexManager<B>,
        arena: &'a IndexArena<'b>,
        swap: SwapManager<'a, 'b>,
        loaded: Arc<LoadedQueue>,
    ) -> Self {
        Self {
            manager,
            arena,
            swap,
            store: MappingStore::new(),
            loaded,
            structural: Mutex::new(()),
        }
    }
    pub fn manager(&self) -> &GpuIndexManager<B> {
        &self.manager
    }
    pub fn swap(&self) -> &SwapManager<'a, 'b> {
        &self.swap
    }
    pub fn store(&self) -> &MappingStore {
        &self.store
    }
    /// Loads `set` and makes the loaded segments visible. When the arena is full, victims are
    /// evicted once and the load is retried.
    pub async fn ensure(
        &self,
        set: EntrySet,
        access: Access,
    ) -> Result<Arc<LoadOutcome>, IndexError> {
        let outcome = match self.manager.ensure(set.clone(), access).await {
            Err(IndexError::ArenaExhausted { requested, .. }) => {
                self.publish_pending().map_err(|e| index_error(&e))?;
                self.make_room(requested as u64).map_err(|e| index_error(&e))?;
                self.manager.ensure(set, access).await?
            }
            other => other?,
        };
        self.publish_pending().map_err(|e| index_error(&e))?;
        Ok(outcome)
    }
    /// One reconciliation pass; reloaded entries are published before it returns.
    pub async fn cycle(&self) -> Result<ScanOutcome, IndexError> {
        let outcome = self.manager.cycle().await?;
        self.publish_pending().map_err(|e| index_error(&e))?;
        Ok(outcome)
    }
    /// The slot gap of a kernel batch that started on `at`. A mapping that moved on since is
    /// retried against a fresh snapshot, because hits of retired regions resolve to nothing.
    pub fn complete_slot(
        &self,
        at: &Snapshot,
        writes: &[WrittenRange],
        hits: &[KernelHit],
        source: &dyn RangeSource,
    ) -> Result<Vec<Hit>, IndexError> {
        let mut at = *at;
        let mut attempts = 0;
        let out = loop {
            match self.manager.complete_slot(&self.store, &at, writes, hits, source) {
                Err(ManagerError::StaleMapping { .. }) if attempts < SLOT_RETRIES => {
                    attempts += 1;
                    at = self.swap.snapshot();
                }
                other => break other.map_err(|e| index_error(&e))?,
            }
        };
        self.reclaim();
        Ok(out)
    }
    /// Idle expiry and watermark relief against the current arena occupancy.
    pub fn evict(&self, now: Instant) -> Result<Vec<Eviction<SegmentKey>>, IndexError> {
        let occupancy = Occupancy::from_budget(&self.arena.budget());
        let evicted = self.manager.evict_tick(now, occupancy);
        self.unpublish(&evicted).map_err(|e| index_error(&e))?;
        Ok(evicted)
    }
    /// Frees retired regions that no reader can still see.
    pub fn reclaim(&self) -> usize {
        self.swap.reclaim(self.swap.epochs().published())
    }
    fn make_room(&self, need: u64) -> Result<(), ManagerError> {
        let occupancy = Occupancy::from_budget(&self.arena.budget());
        let evicted = self.manager.make_room(need, occupancy)?;
        self.unpublish(&evicted)?;
        self.reclaim();
        Ok(())
    }
    /// Publishes every segment the transfer reported since the last call; the first failure is
    /// returned after the remaining segments were attempted.
    fn publish_pending(&self) -> Result<usize, ManagerError> {
        let _structural = self.structural.lock();
        let mut published = 0;
        let mut failure = None;
        for segment in self.loaded.drain() {
            match self
                .manager
                .publish_loaded(segment, &self.swap, &self.store, self.arena)
            {
                Ok(_) => published += 1,
                Err(e) => failure = failure.or(Some(e)),
            }
        }
        failure.map_or(Ok(published), Err)
    }
    /// Removes evicted segments from the table and the mapping, then retires their regions
    /// behind the epoch of the new table. A failed removal puts the segment back under
    /// eviction control.
    fn unpublish(&self, evicted: &[Eviction<SegmentKey>]) -> Result<(), ManagerError> {
        let _structural = self.structural.lock();
        for eviction in evicted {
            let id = eviction.key.segment;
            let Some(region) = self.manager.segment_region(id) else { continue };
            let removed = (|| {
                let mut table = self.swap.build_next();
                if !table.remove(region) {
                    return Err(ManagerError::StaleRegion {
                        arena: region.arena.0,
                        gen: region.gen,
                    });
                }
                let mut map = self.store.snapshot().successor();
                map.remove_segment(id);
                let epoch = self.swap.publish(table)?;
                self.store.publish(map)?;
                Ok(epoch)
            })();
            match removed {
                Ok(epoch) => {
                    self.manager.release_segment(id);
                    self.arena.retire(region, epoch)?;
                }
                Err(e) => {
                    self.manager.track_segment(id, region);
                    return Err(e);
                }
            }
        }
        Ok(())
    }
}
#[cfg(test)]
#[path = "test/runtime.rs"]
mod tests;
