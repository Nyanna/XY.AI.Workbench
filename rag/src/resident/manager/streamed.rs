//! Index manager skeleton and its CPU and GPU variants. The index is attached lazily; until then
//! no resource is held and no subscription exists.
use std::future::Future;
use std::ops::Range;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use std::time::Instant;
use parking_lot::Mutex;
use tokio::runtime::Handle;
use tokio::sync::OnceCell;
use tracing::Instrument;
use crate::resident::backend::ComputeBackend;
use crate::resident::error::{IndexError, ManagerError};
use crate::resident::index::{EntryId, EntrySet, Index, Scope};
use crate::resident::kernel::{KernelHit, WrittenRange};
use crate::resident::manager::advice::{AdviceSink, Hint};
use crate::resident::manager::blocking::BlockingReads;
use crate::resident::manager::evict::{EvictPolicy, Eviction, Evictor, Occupancy, Tier};
use crate::resident::manager::residency::{Access, LoadOutcome, ResidencyLoader};
use crate::resident::manager::scope::{ResolveCtx, ResolverRegistry};
use crate::resident::manager::slot::{
    run_slot_at, Hit, MappingStore, RangeSource, UpdateQueue,
};
use crate::resident::manager::slot::Update;
use crate::resident::manager::swap::{Snapshot, TableSwap};
use crate::resident::manager::transfer::{LoadedSegment, RegionSource};
use crate::resident::manager::sync::{ScanOutcome, SyncScanner};
use crate::resident::manager::tracker::{ResidencyTracker, SlotRef};
pub type SharedIndex = Arc<dyn Index + Send + Sync>;
/// Interface shared by the CPU and GPU variants.
pub trait IndexManager: Send + Sync {
    /// `false` when an index is already attached; the first one stays.
    fn attach_index(&self, index: SharedIndex) -> bool;
    fn is_active(&self) -> bool;
}
pub struct StreamedIndexManager<B: ?Sized> {
    engine: Arc<B>,
    index_id: String,
    index: OnceCell<SharedIndex>,
    residency: Mutex<ResidencyTracker>,
}
impl<B: ComputeBackend + ?Sized> StreamedIndexManager<B> {
    pub fn new(engine: Arc<B>, index_id: impl Into<String>) -> Self {
        Self {
            engine,
            index_id: index_id.into(),
            index: OnceCell::new(),
            residency: Mutex::new(ResidencyTracker::new()),
        }
    }
    pub fn engine(&self) -> &B {
        &self.engine
    }
    pub fn index_id(&self) -> &str {
        &self.index_id
    }
    pub fn index(&self) -> Result<&SharedIndex, IndexError> {
        self.index
            .get()
            .ok_or_else(|| IndexError::IndexUnavailable {
                scope: self.index_id.clone(),
            })
    }
    /// Runs `init` on first use only; concurrent callers wait for the same initialization and a
    /// failed one leaves the manager inactive.
    pub async fn activate<F, Fut>(&self, init: F) -> Result<&SharedIndex, IndexError>
    where
        F: FnOnce() -> Fut,
        Fut: Future<Output = Result<SharedIndex, IndexError>>,
    {
        let index = self
            .index
            .get_or_try_init(|| async {
                let index = init().await?;
                tracing::info!(index_id = % self.index_id, "index activated");
                Ok::<_, IndexError>(index)
            })
            .await?;
        Ok(index)
    }
    pub fn cycle_span(&self) -> tracing::Span {
        tracing::info_span!("manager_cycle", index_id = % self.index_id)
    }
    pub fn resident_len(&self) -> usize {
        self.residency.lock().len()
    }
}
impl<B: ComputeBackend + ?Sized> IndexManager for StreamedIndexManager<B> {
    fn attach_index(&self, index: SharedIndex) -> bool {
        let attached = self.index.set(index).is_ok();
        if attached {
            tracing::info!(index_id = % self.index_id, "index activated");
        } else {
            tracing::warn!(index_id = % self.index_id, "index already attached");
        }
        attached
    }
    fn is_active(&self) -> bool {
        self.index.initialized()
    }
}
/// Access pattern of a multi-entry read; selects the page-cache hint.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReadPattern {
    Dense,
    Scattered,
}
fn covering(items: &[(EntryId, Range<usize>)]) -> Option<Range<usize>> {
    let start = items.iter().map(|(_, r)| r.start).min()?;
    let end = items.iter().map(|(_, r)| r.end).max()?;
    Some(start..end)
}
/// RAM is the primary instance: reads go straight to the index; the only job is page-cache
/// residency of mmap-backed data.
pub struct CpuIndexManager<B: ?Sized> {
    core: StreamedIndexManager<B>,
    reads: BlockingReads,
    advice: Option<Arc<dyn AdviceSink>>,
}
impl<B: ComputeBackend + ?Sized> CpuIndexManager<B> {
    pub fn new(
        core: StreamedIndexManager<B>,
        handle: Handle,
        advice: Option<Arc<dyn AdviceSink>>,
    ) -> Self {
        let mut reads = BlockingReads::new(handle);
        if let Some(sink) = &advice {
            reads = reads.with_advice(sink.clone());
        }
        Self { core, reads, advice }
    }
    pub fn core(&self) -> &StreamedIndexManager<B> {
        &self.core
    }
    /// `hint` is the byte range of the entry in the mapped file, used for the prefetch hint.
    pub async fn read(
        &self,
        entry: EntryId,
        hint: Range<usize>,
    ) -> Result<Vec<u8>, IndexError> {
        let index = self.core.index()?.clone();
        self.reads
            .read(hint, move || index.read(entry).as_bytes().to_vec())
            .instrument(self.core.cycle_span())
            .await
            .map_err(|e| IndexError::TransferFailed {
                reason: format!("read task aborted: {e}"),
            })
    }
    pub async fn read_set(
        &self,
        items: Vec<(EntryId, Range<usize>)>,
        pattern: ReadPattern,
    ) -> Result<Vec<Vec<u8>>, IndexError> {
        let index = self.core.index()?.clone();
        let Some(span) = covering(&items) else { return Ok(Vec::new()) };
        match pattern {
            ReadPattern::Dense => self.reads.advise(span, Hint::Sequential),
            ReadPattern::Scattered => {
                for (_, range) in &items {
                    self.reads.advise(range.clone(), Hint::Random);
                }
            }
        }
        let ids: Vec<EntryId> = items.iter().map(|(id, _)| *id).collect();
        self.reads
            .run(move || {
                ids.iter().map(|id| index.read(*id).as_bytes().to_vec()).collect()
            })
            .instrument(self.core.cycle_span())
            .await
            .map_err(|e| IndexError::TransferFailed {
                reason: format!("read task aborted: {e}"),
            })
    }
    /// Marks a range as unlikely to be needed soon, e.g. after an idle period.
    pub fn cool(&self, range: Range<usize>) {
        let Some(sink) = &self.advice else { return };
        match sink.advise(range.clone(), Hint::Cold) {
            Ok(()) => tracing::debug!(? range, "residency hint removed"),
            Err(e) => tracing::debug!(error = % e, "cold hint rejected"),
        }
    }
}
impl<B: ComputeBackend + ?Sized> IndexManager for CpuIndexManager<B> {
    fn attach_index(&self, index: SharedIndex) -> bool {
        self.core.attach_index(index)
    }
    fn is_active(&self) -> bool {
        self.core.is_active()
    }
}
/// Streaming manager: resolves scopes, reconciles the resident mirror with the index, loads and
/// evicts. The device transfer is injected through the loader, so the shell builds without `cuda`.
pub struct GpuIndexManager<B: ?Sized> {
    core: StreamedIndexManager<B>,
    resolvers: ResolverRegistry,
    loader: ResidencyLoader<String>,
    evictor: Mutex<Evictor<String>>,
    updates: Mutex<UpdateQueue>,
    next_segment: AtomicU32,
    scanner: tokio::sync::Mutex<Option<SyncScanner>>,
}
impl<B: ComputeBackend + ?Sized> GpuIndexManager<B> {
    pub fn new(
        core: StreamedIndexManager<B>,
        resolvers: ResolverRegistry,
        loader: ResidencyLoader<String>,
        evict: EvictPolicy,
    ) -> Self {
        Self {
            core,
            resolvers,
            loader,
            evictor: Mutex::new(Evictor::new(evict)),
            updates: Mutex::new(UpdateQueue::new()),
            next_segment: AtomicU32::new(0),
            scanner: tokio::sync::Mutex::new(None),
        }
    }
    pub fn core(&self) -> &StreamedIndexManager<B> {
        &self.core
    }
    pub fn resolve(
        &self,
        scope: &Scope,
        ctx: &ResolveCtx,
    ) -> Result<EntrySet, IndexError> {
        let index = self.core.index()?;
        self.resolvers.resolve(&**index, scope, ctx)
    }
    pub async fn ensure(
        &self,
        set: EntrySet,
        access: Access,
    ) -> Result<Arc<LoadOutcome>, IndexError> {
        self.core.index()?;
        let key = self.core.index_id().to_owned();
        let outcome = self
            .loader
            .ensure(key.clone(), set, access)
            .await
            .map_err(|e| index_error(&e))?;
        let now = Instant::now();
        let mut evictor = self.evictor.lock();
        if outcome.bytes > 0 {
            evictor.insert(key, outcome.bytes, Tier::Vector, outcome.bytes as f64, now);
            tracing::debug!(bytes = outcome.bytes, "entries loaded");
        } else {
            evictor.touch(&key, now);
        }
        Ok(outcome)
    }
    /// One reconciliation pass: finds foreign changes by stamp comparison, queues removals as
    /// mapping mutations and reloads changed entries. The index stays the source of truth.
    pub async fn cycle(&self) -> Result<ScanOutcome, IndexError> {
        async {
            let index = self.core.index()?.clone();
            let outcome = {
                let mut guard = self.scanner.lock().await;
                let scanner = guard.get_or_insert_with(|| SyncScanner::new(&*index));
                let residency = self.core.residency.lock();
                scanner.scan(&*index, &residency)
            };
            let reload = {
                let mut updates = self.updates.lock();
                outcome.enqueue_removals(&mut updates);
                outcome
                    .reload
                    .iter()
                    .copied()
                    .filter(|e| !updates.contains(*e))
                    .collect::<Vec<_>>()
            };
            if !reload.is_empty() {
                self.ensure(EntrySet::from_ids(reload), Access::Regular).await?;
            }
            Ok(outcome)
        }
            .instrument(self.core.cycle_span())
            .await
    }
    pub fn evict_tick(
        &self,
        now: Instant,
        occupancy: Occupancy,
    ) -> Vec<Eviction<String>> {
        let _span = self.core.cycle_span().entered();
        let evicted = self.evictor.lock().tick(now, occupancy);
        for e in &evicted {
            tracing::info!(
                key = % e.key, bytes = e.bytes, reason = ? e.reason, "evicted"
            );
        }
        evicted
    }
    /// Makes a loaded segment visible: table first, then the matching mapping epoch. The bindings
    /// are queued and take effect in the next slot gap. A failure returns the region.
    pub(crate) fn publish_loaded(
        &self,
        loaded: LoadedSegment,
        swap: &dyn TableSwap,
        store: &MappingStore,
        regions: &dyn RegionSource,
    ) -> Result<u32, ManagerError> {
        let id = self.next_segment.fetch_add(1, Ordering::Relaxed);
        let LoadedSegment { region, map: slots, binds } = loaded;
        let published = (|| {
            let mut table = swap.build_next();
            table.push(region);
            let mut map = store.snapshot().successor();
            if !map.add_segment(id, slots) {
                return Err(
                    ManagerError::from(IndexError::TransferFailed {
                        reason: format!("segment {id} overlaps a mapped segment"),
                    }),
                );
            }
            swap.publish(table)?;
            store.publish(map)
        })();
        if let Err(e) = published {
            let _ = regions.free_segment(region);
            return Err(e);
        }
        let mut updates = self.updates.lock();
        for (entry, slot, stamp) in binds {
            updates
                .push(Update::Bind {
                    entry,
                    slot: SlotRef { segment: id, slot },
                    stamp,
                });
        }
        tracing::debug!(segment = id, offset = region.offset, "segment published");
        Ok(id)
    }
    pub fn pending_updates(&self) -> usize {
        self.updates.lock().depth()
    }
    /// A `StaleMapping` error means the mapping moved on; retry with a fresh snapshot.
    pub(crate) fn complete_slot(
        &self,
        store: &MappingStore,
        at: &Snapshot,
        writes: &[WrittenRange],
        hits: &[KernelHit],
        source: &dyn RangeSource,
    ) -> Result<Vec<Hit>, ManagerError> {
        let index = self.core.index()?;
        let mut updates = self.updates.lock();
        let mut residency = self.core.residency.lock();
        run_slot_at(
            store,
            at,
            writes,
            hits,
            source,
            &**index,
            &mut updates,
            &mut residency,
        )
    }
}
impl<B: ComputeBackend + ?Sized> IndexManager for GpuIndexManager<B> {
    fn attach_index(&self, index: SharedIndex) -> bool {
        self.core.attach_index(index)
    }
    fn is_active(&self) -> bool {
        self.core.is_active()
    }
}
fn index_error(e: &ManagerError) -> IndexError {
    match e {
        ManagerError::ArenaExhausted { requested, available } => {
            IndexError::ArenaExhausted {
                requested: *requested as usize,
                available: *available as usize,
            }
        }
        other => {
            IndexError::TransferFailed {
                reason: other.to_string(),
            }
        }
    }
}
#[cfg(test)]
#[path = "test/streamed.rs"]
mod tests;
