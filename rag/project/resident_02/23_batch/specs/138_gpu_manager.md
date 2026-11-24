# GPU-Manager: Technical Specification (build order 138)

Crate module `gpu_manager`.

## 1. Task, restrictions and boundaries

**Task.** The GPU-Manager is the memory manager of the GPU-resident part of one index. It translates between the index (entries) and the kernel (memory regions, slots and positions), and keeps both in sync.

- **Ensure:** it loads entry sets into slots of segments (`ensure`).
- **Reconcile:** it runs reconcile iterations against the index (`reconcile_iteration`).
- **Publish:** it publishes a filled segment: segment ID, segment table, mapping, manifest, eviction tracking and bindings (`publish_segment`).
- **Segments:** it releases and evicts segments and cleans up their residency (`release_segment`, `evict_step`, `make_room`, `run_gap`).
- **Sync:** it writes the kernel segment table (`SegmentEntry[]`) to VRAM (`sync_to_vram`).
- **Write-back:** it writes processed GPU results into the index and registers them as resident (`write_back`, `mirrored_index`).
- **Translation:** it maps global slots (kernel positions) to entries and back.
- **Communication-Slot services:** it provides the sub-engine with the `MappingSource`, `SegmentDirectory` and update-queue adapter that the Communication-Slot (119) and the hit resolution (114) need.
- **Scopes and reporting:** it resolves scopes through the resolver registry and reports the segment count, the resident bytes and the pending updates.

**Restrictions.**

- It is a component of the GPU sub-engine. It is not part of the facade and not part of the index.
- It is constructed with the services of the sub-engine for this index (`Arc<dyn GpuServices>`, §3) and a lazily activated index (`LazyIndex`).
- It spawns no threads. The caller drives every operation.
- It never resolves a `RegionHandle` itself. Memory access and device addresses come only from `GpuServices`.
- It does not plan queries, run kernels or search. It does not score, choose lanes or create lanes.
- It does not own `Eviction`, `TableSwap`, `MappingStore`, `ManifestStore`, `UpdateQueue`, `SegmentArena` or the `ResidencyDirectory`. The sub-engine owns them and shares them with the coordinator (140) and the Communication-Slot (119).
- **There is exactly one slot mapping.** The mapping slot ↔ entry exists only in the `SlotAssignment` of each segment, and the mapping entry → slot and stamp only in the `ResidencyDirectory`. There is no `SegmentStore` and no second queue.
- Slots are append-only (101): a slot is never reused or overwritten. Rebuild and compaction of segments are not part of this module.
- It calls `UpdateQueue::apply` only inside the adapter `SlotUpdates`, which the Communication-Slot drives.
- It writes to the index only in `write_back` and through `mirrored_index`.
- It never activates the sub-engine. Lane failures come back as `VramSync`.
- No `unsafe`, no globals. Crates: `log`, `half` plus the project modules in §5.

## 2. Context

- **Sub-engine services.** The sub-engine builds one `GpuServices` value for each attached index. `TableSwap` and the segment allocator are shared by all indexes of the sub-engine. `MappingStore`, `ManifestStore`, `Eviction`, `UpdateQueue`, the residency directory and the `StructureLock` belong to one index. Together they are the shared structures: the segment allocator, `TableSwap`, `MappingStore`, `ManifestStore`, `Eviction`, `UpdateQueue`, the `ResidencyDirectory`, and the one `StructureLock`. It also writes bytes visibly into device memory and writes the kernel table. This module sees all of it only through `GpuServices`.
- **Index.** It is the authority, and "the index wins". An entry's vector is a token matrix: `tokens × dim` elements of the index dtype. The index serializes its writers itself (Last-Writer-Wins), so this module adds no write lock.
- **Kernel.** It runs as a persistent loop. It reads work from a ring and writes results to a ring. It knows no index, only memory regions, slots and positions. `gpu_kernel` (118) reads a `SegmentEntry[]` table in VRAM and a ring of global slot indices. This module provides the table and the translation between entries and global slots.
- **Slot model.** A segment is a `SegmentView` (062) with a `SlotAssignment` and `SlotStates`. A `SegmentWriter` (101) reserves slots in ascending order, so a slot is written once. An update of an entry takes a new slot, and the old slot is hidden afterwards.
- **Mapping changes.** Bindings change only through the `UpdateQueue` (109) in the gap between batches. The exception is the commit of a fresh slot in a segment, which only binds a slot that was unbound before (083).
- **Neighbours.**
  - The coordinator (140) serializes publishes, makes room on exhaustion and retries once.
  - `LazyIndex` comes from `manager` (136), which is reduced to that trait.
  - `ResidencyDirectory` and `Reconciler` hold the known stamps (100).
  - `Eviction` tracks segments (103) and removes evicted segments from the table and the map.
  - `ScopeResolverRegistry` resolves scopes. `load_aggregated` loads entries (124).

## 3. Interface

```rust
use std::sync::{Arc, Mutex};
use std::time::Instant;
use index_contract::{EntryId, Index, IndexError, Stamp};   // Stamp is defined in change_stamp and re-exported
use scopes::{EntrySet, Scope};
use scope_resolution::{CellSelector, ResolveError, ScopeResolverRegistry};
use reconcile::{Reconciliation, ResidencyDirectory};
use segments::{SegmentId, SlotRef};                 // SegmentId(pub u64), SlotRef{segment, slot: Slot}
use segment_arena::{Readers, Segment, SegmentArena, SegmentArenaError};
use allocator::Allocator;
use bookkeeping::Region;
use eviction::{Eviction, EvictionError, EvictionOutcome};
use table_swap::{TableSwap, TableSwapError};
use address_map::{MapError, MappingStore};
use manifest::{ManifestError, ManifestStore};
use update_queue::UpdateQueue;
use segment_writer::{SegmentWriter, WriteError};
use slot_layout::{EntryRef, SegmentView, SlotAssignment, SlotClass, SlotError, SlotStates};
use kernel_layout::{Addr, Slot};
use layout::DType;
use region_handle::RegionHandle;
use device_lane_event::{Event, Lane};
use manager::LazyIndex;
use communication_slot::{MappingSnapshot, MappingSource, StructureGuard as SlotStructureGuard, UpdateOutcome};
use communication_slot::UpdateQueue as SlotUpdateQueue;   // the slot's trait, not update_queue::UpdateQueue
use hit_resolution::{SegmentBinding, SegmentDirectory};

/// The one structure lock of the sub-engine. Not reentrant.
pub struct StructureLock { /* Mutex<()> */ }
impl StructureLock {
    pub fn new() -> StructureLock;
    pub fn lock(&self) -> StructureGuard<'_>;      // recovers a poisoned lock
}
pub struct StructureGuard<'a> { /* MutexGuard<'a, ()> */ }
impl SlotStructureGuard for StructureGuard<'_> {}

/// Implemented by the GPU sub-engine (on top of GpuBackend), once per attached index. Send + Sync.
pub trait GpuServices: Send + Sync {
    fn arena(&self) -> &SegmentArena<Arc<Allocator>>;   // arena IndexSegments
    fn table_swap(&self) -> &Arc<TableSwap>;
    fn mapping_store(&self) -> &Arc<MappingStore>;
    fn manifests(&self) -> &Arc<ManifestStore>;
    fn eviction(&self) -> &Arc<Eviction>;                // Tier::Vram is used
    fn updates(&self) -> &Arc<UpdateQueue>;
    /// Shared with the Communication-Slot, which holds the mutex for a whole pass.
    fn residency(&self) -> &Arc<Mutex<ResidencyDirectory>>;
    fn structure_lock(&self) -> &Arc<StructureLock>;
    /// Device address of byte 0 of the region (the segment base).
    fn device_base(&self, handle: &RegionHandle) -> Option<Addr>;
    /// Writes `bytes` at `offset` (relative to byte 0 of `handle`). The returned event completes
    /// when the content is visible to the device. Writes into a segment that is not yet published.
    fn write_visible(&self, handle: &RegionHandle, offset: u64, bytes: &[u8]) -> Result<Event, String>;
    /// Writes `table` (SegmentEntry[] bytes) to VRAM, ordered before work later submitted on `lane`.
    /// Returns the device address of the written table. Must not overwrite a table still in use by running kernels.
    fn write_kernel_table(&self, lane: &mut Lane, table: &[u8]) -> Result<u64, String>;
}

#[derive(Clone, Debug)]
pub struct GpuManagerConfig {
    pub class: SlotClass,      // token limit of every slot of this index; SlotClass::custom(1) for one vector per entry (119c F1)
    pub dim: u32,              // > 0
    pub dtype: DType,          // F32, F16 or BF16; must equal Index::dtype()
    pub segment_bytes: u64,    // requested length of a new segment, >= slot size
}

#[derive(Clone, Debug, PartialEq)]
pub enum GpuManagerError {
    InvalidConfig(String),                       // "ungültige GPU-Manager-Konfiguration: {0}"
    Index(String),                               // "Index nicht verfügbar: {0}"
    Resolve(ResolveError),
    DTypeMismatch { index: DType, configured: DType },
    InvalidVector { len: usize, row_bytes: usize },  // len is not a multiple of dim * element size
    TooManyTokens { tokens: u32, max: u32 },
    Arena(SegmentArenaError),                    // includes Exhausted
    SegmentLimit,                                // global slot space (u32) exhausted
    UnknownSegment(u64),
    Slot(SlotError),
    SlotWrite(WriteError),
    Publish(TableSwapError),
    Mapping(MapError),
    Manifest(ManifestError),
    Memory(String),                              // device_base or write_visible failed
    VramSync(String),                            // write_kernel_table failed
    IndexWrite(IndexError),                      // write_back: index write failed
}
// Display + std::error::Error; Send + Sync + 'static.

/// Binding delivered with a loaded segment; becomes an Update (Bind) in the UpdateQueue.
#[derive(Clone, Copy, Debug)]
pub struct LoadedBinding { pub entry: EntryRef, pub slot: Slot, pub stamp: Stamp }

/// A filled segment that is not yet visible to readers.
pub struct LoadedSegment {
    pub segment: Segment,                    // from SegmentArena::allocate
    pub view: SegmentView,
    pub device_base: Addr,
    pub states: Arc<SlotStates>,
    pub assignment: Arc<SlotAssignment>,
    pub bindings: Vec<LoadedBinding>,
    /// Kept so that later slots can be appended and old slots hidden. None for read-only segments.
    pub writer: Option<Arc<SegmentWriter>>,
}

/// FIFO queue of reported segments. Clone shares the queue. Send + Sync.
#[derive(Clone, Default)]
pub struct LoadedQueue { /* Arc<Mutex<VecDeque<LoadedSegment>>> */ }
impl LoadedQueue {
    pub fn report(&self, seg: LoadedSegment);   // append; never blocks on publish
    pub fn drain(&self) -> Vec<LoadedSegment>;  // takes all, in order
    pub fn len(&self) -> usize;
    pub fn is_empty(&self) -> bool;
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct EnsureReport {
    pub loaded: Vec<EntryId>,                 // newly placed, ascending
    pub already_resident: Vec<EntryId>,       // resident, or placed and waiting for their bind; ascending
    pub absent_in_index: Vec<EntryId>,        // ascending
    pub failed: Vec<(EntryId, GpuManagerError)>,
    pub bytes_moved: u64,                     // sum of slot bytes written
    pub segments_created: usize,              // reported to the LoadedQueue, not yet published
}

#[derive(Debug, Clone, PartialEq)]
pub struct ReconcileOutcome {
    pub reconciliation: Reconciliation,
    pub queued_unbinds: usize,                // newly enqueued or replaced Unbind items
    pub reloaded: usize,                      // placed in a new slot and queued as Bind
    pub skipped_pending: usize,               // changed, but an update was already pending
    pub failed: usize,                        // reload failed; the next iteration re-detects it
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct GapReport { pub slots_hidden: usize, pub reclaimed: usize }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GpuIndexStats { pub segments: usize, pub resident_bytes: u64, pub pending_updates: usize }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct VramTable { pub segments_addr: u64, pub segment_count: u32, pub version: u64 }

/// A processed GPU result for the index. `placed` names the slot that already holds the data.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MirrorItem { pub id: EntryId, pub vector: Vec<u8>, pub meta: Vec<u8>, pub placed: Option<SlotRef> }

#[derive(Debug, Default, Clone, PartialEq)]
pub struct MirrorReport {
    pub written: Vec<(EntryId, Stamp)>,
    pub failed: Vec<(EntryId, GpuManagerError)>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleasedSegment {
    pub id: SegmentId,
    pub region: Region,
    pub cleared: Vec<EntryId>,                // entries whose residency was removed, ascending
}

/// Update-queue adapter for the Communication-Slot. Implements `communication_slot::UpdateQueue`.
pub struct SlotUpdates { /* Arc<UpdateQueue>, Arc<MappingStore> */ }

pub struct GpuManager { /* private */ }       // Send + Sync

impl GpuManager {
    /// Validates only. Loads nothing, allocates nothing, does not call LazyIndex::get.
    pub fn new(services: Arc<dyn GpuServices>, index: Arc<dyn LazyIndex>,
               resolver: Arc<ScopeResolverRegistry>, cell_selector: Option<Arc<dyn CellSelector>>,
               queue: LoadedQueue, cfg: GpuManagerConfig) -> Result<GpuManager, GpuManagerError>;

    // Scope and load
    pub fn resolve_scope(&self, scope: &Scope) -> Result<EntrySet, GpuManagerError>;
    pub fn ensure(&self, set: &EntrySet) -> Result<EnsureReport, GpuManagerError>;

    // Reconcile
    /// Ok(None): the index no longer exists.
    pub fn reconcile_iteration(&self) -> Result<Option<ReconcileOutcome>, GpuManagerError>;

    // Segments
    pub fn publish_segment(&self, seg: LoadedSegment) -> Result<SegmentId, GpuManagerError>;
    pub fn release_segment(&self, id: SegmentId) -> Result<ReleasedSegment, GpuManagerError>;
    pub fn evict_step(&self, now: Instant) -> EvictionOutcome;
    pub fn make_room(&self, need: u64) -> Result<EvictionOutcome, EvictionError>;
    /// Call once per gap, after the Communication-Slot completion.
    pub fn run_gap(&self) -> GapReport;

    // VRAM sync and write-back
    pub fn sync_to_vram(&self, lane: &mut Lane) -> Result<VramTable, GpuManagerError>;
    pub fn write_back(&self, items: &[MirrorItem]) -> Result<MirrorReport, GpuManagerError>;
    /// The index wrapped for the Communication-Slot (`attach_index`), see §4.11.
    pub fn mirrored_index(&self) -> Result<Arc<dyn Index>, GpuManagerError>;

    // Services for Communication-Slot (119) and hit resolution (114)
    pub fn slot_updates(&self) -> SlotUpdates;
    pub fn mapping_source(&self) -> Arc<dyn MappingSource>;
    pub fn segment_directory(&self) -> Arc<dyn SegmentDirectory>;

    // Reporting and lookup
    /// Is an unseen index signal pending? Never blocks. False before the first reconcile iteration.
    pub fn signal_pending(&self) -> bool;
    /// IDs of the published segments, ascending.
    pub fn segment_ids(&self) -> Vec<SegmentId>;
    pub fn stats(&self) -> GpuIndexStats;
    pub fn segment_count(&self) -> usize;
    pub fn resident_bytes(&self) -> u64;
    pub fn pending_updates(&self) -> usize;
    pub fn slot_of(&self, id: EntryId) -> Option<SlotRef>;
    pub fn global_slot_of(&self, id: EntryId) -> Option<u32>;
    pub fn entry_at_global_slot(&self, slot: u32) -> Option<EntryRef>;
}
```

## 4. Behavior and invariants

### 4.1 Construction and state

- `new` returns `InvalidConfig` if `dim == 0`, `slot_layout::slot_size(class, dim, dtype)` fails or `dtype` is not F32, F16 or BF16, or `segment_bytes < slot_bytes`.
- `slot_bytes = slot_size(class, dim, dtype)` and `row_bytes = dim × dtype.size_in_bytes()`.
- The manager owns these parts, all created in `new`:
  - the segment records, a `next_id` counter (starting at 0) and a `next_first_slot` counter;
  - the list of unpublished segments (filled, not yet published);
  - the set `in_flight` of entries placed in an unpublished segment;
  - the list `retired_slots` of slots to hide in the next gap;
  - the deferred-return list of regions (§4.8);
  - an `Option<Reconciler>` and an `Option<IndexWriter>`, both created lazily;
  - a RAM table version counter and a cache of the last VRAM sync;
  - the previous manifest (§4.12).
- A segment record holds: `SegmentId`, `Segment`, `RegionHandle`, `SegmentView`, `device_base`, `Arc<SlotStates>`, `Arc<SlotAssignment>`, `Option<Arc<SegmentWriter>>` and a permanent `first_slot`.
  - `first_slot` is assigned when the segment is created, from a counter that only grows.
  - It is never reused and never compacted, so the global slot numbers of a segment never change.
- Global slot = `first_slot + slot`.

### 4.2 Residency and mapping invariants

- An entry is resident iff the `ResidencyDirectory` contains it. The directory is filled only by `UpdateQueue::apply` (Bind).
- An entry that is placed in a slot but whose bind is not applied yet is not resident, but it is not missing either. It is in `in_flight` (segment not published) or has a pending Bind in the queue (`updates.get(id)`). `ensure` and `reconcile_iteration` treat such an entry as already present.
- The slot of an entry is `directory.get(id).slot` (`segments::SlotRef`). `slot_of`, `global_slot_of` and `entry_at_global_slot` are exact inverses over resident entries.
  - `entry_at_global_slot(g)` finds the published record with `first_slot <= g < first_slot + slot_count` and returns `assignment.lookup(Slot(g - first_slot))`.
  - They read the current state, do no IO and do not wait for ensure or for index calls.
- `stats()` reports `segments` = the number of published records, `resident_bytes` = `Eviction::bytes(Tier::Vram)` (the eviction view, live bytes) and `pending_updates` = `updates.depth()`. The individual methods return the same three values.
- Every entry has `EntryRef { entry, section: 0 }` in this module.
- `signal_pending()` is `Reconciler::pending()`. It uses `try_lock` on the Reconciler mutex and never waits. If a `reconcile` is running, it returns true. It is false while no Reconciler exists.
- `segment_ids()` returns the IDs of the published records in ascending order.

### 4.3 Scope resolution

- `resolve_scope` calls `ScopeResolverRegistry::resolve(scope, &ResolveContext{index: Some(&*index), cell_selector})`.
  - The index comes from `LazyIndex::get`.
  - An error from `get` gives `Index(e)`. A resolver error gives `Resolve(e)`.
  - There is no fallback to all entries.

### 4.4 Ensure

`ensure(set)` runs in this order:

1. Get the index with `LazyIndex::get`. An error gives `Index(e)` and nothing changes. If `index.dtype() != cfg.dtype`, return `DTypeMismatch`.
2. `missing` = the members of `set` (ascending) that are not in the directory, not in `in_flight` and have no pending Bind in the queue. The others go to `already_resident`. Staleness is not checked, because reconcile owns that.
3. If `missing` is not empty, call `load_aggregated` with one request `LoadRequest{index: (), scope: Scope::Set(missing as EntrySet)}`.
   - An `Err` result gives `Resolve(e)`. In that case nothing is placed.
   - No manager lock is held during this step. The loader may block on IO.
4. For each loaded entry, in ascending order:
   - `None`: add it to `absent_in_index`.
   - `Some(le)`: `vector.len()` must be a multiple of `row_bytes`, else `failed(InvalidVector)`. `tokens = len / row_bytes` must be `<= class.max_tokens()`, else `failed(TooManyTokens)`. Then place it (§4.5).
5. After the loop, report every unpublished segment that received slots to the `LoadedQueue` (§4.5) and call `Eviction::touch(EvictKey(id), now)` for all published records.
6. Log with `log::info!`: the entry count, `bytes_moved` and the segments created.

Further rules:

- A per-entry failure goes into `failed`. It does not abort the other entries.
- `ensure` does not publish. The coordinator (140) publishes the reported segments.
- If a segment cannot be allocated because the arena is exhausted, `ensure` first reports what it has filled so far, then returns `Err(Arena(Exhausted{..}))`. Calling `ensure` again is safe: placed entries count as present.
- `ensure` does not make room itself. The retry after making room belongs to the coordinator.
- Concurrent `ensure` calls for the same entry place it once: the reservation step re-checks `in_flight` and the directory under the records lock.

### 4.5 Placement and segment creation

**Placing one entry** (no structure lock; the records lock is held only for the steps that say so):

1. Under the records lock, take the newest record or unpublished segment that has a writer and a free slot. `writer.reserve(EntryRef{id, 0}, tokens, None)` returns the reservation. On `NoFreeSlot`, create a new segment (below) and reserve once more. Add the entry to `in_flight` if its segment is not published.
2. Without a lock: decode the payload rows to f32 (`half` for F16 and BF16), build the slot with `view.encode_slot(tokens, &values)` (padding rows get the mask value), and call `subengine.write_visible(&handle, res.target.start - handle.offset(), &bytes)`. An error gives `Memory(e)`.
3. `event.wait()`, then `writer.commit(&res, &event)`. The commit binds the slot and sets the valid bit last. A failure gives `SlotWrite(e)`. The entry stays out of `in_flight`, and its reserved slot stays unused.
4. Record the binding `(EntryRef, slot, le.stamp)`.
   - If the segment is published: `updates.push_bind(entry_ref, SlotRef{segment: id, slot}, stamp)` right away.
   - If it is not published: keep the binding with the unpublished segment.
5. Add `slot_bytes` to `bytes_moved`.

The stamp is the one that `load_entry` returned for the data in the slot. It is never a later `stamp()` result.

**Creating a segment (unpublished)**

1. `arena.allocate(cfg.segment_bytes)`. An error gives `Arena(e)`, and `Exhausted` passes through unchanged.
2. `RegionHandle::new(ArenaKind::IndexSegments, seg.offset, seg.len, Generation(seg.region.generation as u64))`.
3. `device_base = subengine.device_base(&handle)`. `None` gives `Memory`.
4. `slot_count = seg.len / slot_bytes`. A value of 0 or above `u32::MAX` is `InvalidConfig` or `SegmentLimit`.
5. Check `first_slot + slot_count <= u32::MAX`, else `SegmentLimit`.
6. `SegmentView::new(handle, slot_count, class, dim, dtype, Generation(..))` and `SegmentWriter::new(view, device_base)`.
7. Take `first_slot` from `next_first_slot` and advance it. The segment ID is assigned only at publish.

On a failure in steps 2 to 6, release the allocated segment with `arena.release(&seg, Readers::None)`. Nothing stays registered.

**Reporting.** An unpublished segment with at least one committed slot is reported once as a `LoadedSegment` (with its writer, states, assignment and bindings). Later placements into a reported but still unpublished segment are not allowed: such a segment is closed for new slots until it is published.

### 4.6 Publish

`publish_segment(seg)` takes the structure lock and holds no index call. It runs in this order:

1. `id = SegmentId(next_id)`.
2. `Entry::new(id, view, device_base, states, assignment)`. Then `m = store.next()` and `m.add(entry)`. This is a pre-check on a local map, so overlap and duplicate IDs are caught before anything is published.
3. Table: `t = swap.next_table()`, then `t.add(segment.region)`, then `swap.publish(t)`. On `StaleMapping`, retry with a fresh `next_table()`, up to 3 attempts in total.
4. Mapping: `store.publish(m)`. On `StaleMapping`, rebuild `m = store.next()` plus `add` and retry, up to 3 attempts in total.
5. Manifest: build `SegmentSpec::from_view(id, &view, device_base)` for all published records plus the new one, then `next = manifests.successor(&manifests.current(), specs)` and `manifests.publish(next)`. On `StaleGeneration`, retry up to 3 attempts in total. The replaced manifest is kept as `previous`.
6. On success:
   - `next_id += 1`. A failed publish consumes no ID.
   - The record is added and the RAM table version increases by 1.
   - `eviction.track(EvictKey(id.0), TrackSpec{tier: Tier::Vram, segment: id, region: segment.region, size: segment.region.len, reload_cost: segment.region.len as f64, summary: false}, Instant::now())`.
   - For each binding, in order: `updates.push_bind(b.entry, SlotRef{segment: id, slot: b.slot}, b.stamp)`. They take effect in the next gap. Their entries leave `in_flight`.
   - `log::info!` with the ID and the three epochs.

**Order.** The table is published first, then the mapping, then the manifest. `track` and the binding enqueue happen only after all three publishes succeed.

**Errors.**

- A failure before the table publish: release the segment immediately with `arena.release(&segment, Readers::None)`. Clear its entries from `in_flight`. They are loaded again by a later `ensure`.
- A failure after the table publish (mapping or manifest): undo in reverse order. Remove the segment from the map and from the table and publish both. Put `(segment, e)` on the deferred-return list, where `e` is the epoch of the table rollback. If a rollback publish itself fails, log it with `log::error!` and keep the region, because it is still published.
- A failed `arena.release` is logged with `log::warn!`. It never replaces the original error.
- Every error is a `GpuManagerError`. `Mapping(MapError::StaleMapping)` and `Publish(TableSwapError::StaleMapping)` are reported after the retries.

### 4.7 Reconcile iteration

`reconcile_iteration` runs in this order:

1. Get the index with `LazyIndex::get`. An error gives `Index(e)`.
2. On the first call, create `Reconciler::new(&index)` and keep it. It holds the index weakly. The Reconciler mutex is held only during the `reconcile` call.
3. `reconciler.reconcile(&directory)` with the residency mutex held for the call. `None` gives `Ok(None)`.
4. For every removed entry:
   - Read its `SlotRef` and stamp from the directory.
   - `updates.push_unbind(EntryRef{id, 0}, slot_ref, stamp)`, and add `(slot_ref, id)` to `retired_slots`.
   - `queued_unbinds` counts these.
5. For every entry in `reload`:
   - If an update is pending for it (`updates.contains(id)`), skip it and count `skipped_pending`.
   - Otherwise load it with `reconcile::load_entry`.
     - `None`: handle it like a removed entry.
     - `Some(le)` with a wrong vector length or too many tokens: count it in `failed`.
     - `Some(le)` otherwise: place it in a new slot (§4.5). `placed` queues the Bind. Add the old `(slot_ref, id)` to `retired_slots` if the old slot is in another segment. Count it in `reloaded`.
   - On a load or write failure the directory stays unchanged, and the next iteration finds the difference again.
6. Report unpublished segments (§4.5). Return `Ok(Some(ReconcileOutcome{..}))`. The `Reconciliation` is passed through unchanged.

Rules for this operation:

- The iteration never loads non-resident entries and never modifies the index.
- The old slot of a changed entry is not overwritten. A new slot takes the data, and the Bind in the queue moves the directory entry and releases the old binding in the next gap.
- An empty directory gives a clean `Reconciliation`.

### 4.8 Releasing and evicting segments

**`release_segment(id)`** takes the structure lock and runs in this order:

1. Look up the published record. If it is missing, return `UnknownSegment(id.0)` and change nothing.
2. Publish a table without the region (`t.remove`) and a map without the segment (`m.remove`). Errors give `Publish` or `Mapping`, and nothing else changes.
3. Publish a successor manifest without the segment.
4. `Eviction::remove(EvictKey(id.0))`.
5. Clean up the segment (below).
6. Put `(segment, e)` on the deferred-return list. `e` is the epoch of the table publish.

The result is `ReleasedSegment{id, region, cleared}`.

**`evict_step(now)`** takes the structure lock:

1. `Eviction::reclaim(swap.min_reader_epoch())`.
2. `Eviction::tick(now)`.
3. For every decision whose victim is one of this manager's records, clean up the segment and publish a successor manifest without the evicted segments.
4. The `EvictionOutcome` is returned unchanged.

**`make_room(need)`** takes the structure lock:

1. `Eviction::reclaim(...)`.
2. `Eviction::make_room(Tier::Vram, need)`. `Err(Exhausted)` is returned unchanged and changes nothing.
3. Clean up the decisions as in `evict_step`.
4. `Eviction::reclaim(...)` again.

**Cleaning up a segment** removes the record, increases the RAM table version and:

- removes the directory entries of all entries whose `SlotRef.segment` is this segment, and returns them in `cleared`;
- removes the segment's slots from `retired_slots` and drops its entries from `in_flight`;
- leaves queued updates for the segment alone. They fail harmlessly: a Bind or Unbind into an unmapped segment produces a warning, never an error or a panic.

Rules for eviction:

- `Eviction` itself removes the segment from the table and the map and defers its region. The manager never calls `arena.release` for evicted segments.
- Keys that are not records of this manager are ignored.
- `Eviction` frees regions through the allocators and not through `SegmentArena`. `SegmentArena` drops such regions from its own state by itself (119c C1), so the manager does nothing for it.
- An evicted or released segment keeps its global slot numbers reserved. They are not reused.

### 4.9 Gap work

`run_gap()` runs after the Communication-Slot completion, once per gap:

1. For every entry in `retired_slots` whose update has been applied (`updates.contains(id)` is false), call `states.invalidate(slot)` on its segment and remove it from the list. A slot of a segment that no longer exists is dropped. `slots_hidden` counts them.
2. Reclaim deferred memory (`reclaim_deferred`, §4.12). `reclaimed` counts the regions.

The valid bit is cleared here and not at the time of the update, because mapping and visibility change only in gaps.

### 4.10 VRAM sync

`sync_to_vram(lane)`:

- It takes the sync lock. It reads the RAM version and a snapshot of the published records together, under the records lock.
- If the version equals the last synced version, it returns the cached `VramTable` without a lane operation. This is the "effective" sync.
- Otherwise it builds `SegmentEntry` records, in ascending segment order:
  - `base = device_base.0` of the record.
  - `lens_addr = 0`, so every row up to `pad_len` counts. Padding rows carry the mask value.
  - `stride_bytes = view.slot_bytes()`.
  - `first_slot` and `slot_count` come from the record.
  - `pad_len = view.class().max_tokens()`.
  - `_reserved = 0`.
- It serializes the records with `gpu_kernel::encode_segment_table` (119c): `repr(C)` layout, 40 bytes each, native-endian.
- It calls `subengine.write_kernel_table(lane, bytes)`. An error gives `VramSync(e)`, and the cache is unchanged.
- On success it stores `(version, addr)` and returns `VramTable{segments_addr, segment_count, version}`.
- With zero segments it writes nothing and returns `segments_addr: 0, segment_count: 0`.
- The caller must call it with the lane before it starts an operation on that lane. The caller puts `segments_addr` and `segment_count` into `KernelParams`.

### 4.11 Write-back and the mirrored index

**`write_back(items)`:**

1. Get the index with `LazyIndex::get`. An error gives `Index(e)`. The `IndexWriter` is created on the first call and then reused.
2. For each item, in order:
   - A vector length that is not a multiple of `row_bytes` gives `failed(InvalidVector)`.
   - Otherwise call `IndexWriter::write(id, PayloadRef{vector, meta})`.
     - An `IndexError` gives `failed(IndexWrite(e))`.
     - Success gives `written.push((id, identity.stamp))`.
     - If `placed` is `Some(slot_ref)`, call `updates.push_bind(EntryRef{id, 0}, slot_ref, identity.stamp)`. The entry becomes resident in the next gap with exactly the stamp of this write.
     - Else if the entry is resident, call `directory.set_known_stamp(id, stamp)` with exactly the stamp returned by the write. This is echo suppression, so the next reconcile does not reload the entry.
     - A non-resident entry without `placed` only gets the index write.
3. The slot is not rewritten, because the processed result already lives in VRAM. The caller guarantees that the slot data equals the written vector.
4. A failure of one item does not abort the others.

**`mirrored_index()`** returns an `Index` that wraps the index from `LazyIndex::get`. It forwards every method unchanged, with one addition for `write` and `write_many`:

- After a successful write with stamp `s`, if the entry is not in the directory and has no pending update, look for a slot of the entry in the published records (`assignment.slot_of_entry(EntryRef{id, 0})` with a set valid bit).
- If there is one, call `updates.push_bind(EntryRef{id, 0}, SlotRef{segment, slot}, s)`.
- Reason: the Communication-Slot (119) calls `index.write` for slots that the kernel filled. It records the stamp with `set_known_stamp`, which ignores entries that are not resident. This addition makes a new GPU-built entry resident in the next gap with the stamp of its own write, so it is neither lost nor reloaded.
- The wrapper adds no lock to the index. The index serializes its writers itself.
- The Communication-Slot is attached with `attach_index(mirrored_index()?)`.

### 4.12 Services for the Communication-Slot and hit resolution

- **`SlotUpdates`** implements `communication_slot::UpdateQueue`:
  - `depth()` is `updates.depth()`. `is_pending(e)` is `updates.contains(e)`.
  - `apply(&mut self, residency)` takes `map = store.current()`, calls `updates.apply(residency, &map)`, logs every warning from `take_warnings()` with `log::warn!(target: "update_queue", ..)` and returns `UpdateOutcome{applied}`.
  - It is the only place in production that calls `UpdateQueue::apply`. Producers push at any time without a lock. `depth()` and `is_pending()` are snapshots.
  - It does not change the manifest generation.
- **`mapping_source()`** implements `MappingSource`:
  - `snapshot()` is `MappingSnapshot{current: Arc::new(manifests.current()), previous}`, with `previous` from the last manifest publish.
  - `lock_structure()` is `structure_lock.lock()` as a boxed guard.
  - `reclaim_deferred()` runs: `Eviction::reclaim(swap.min_reader_epoch())`; then `ManifestStore::reclaim()`; then the deferred-return list: every `(segment, e)` with `e <= swap.min_reader_epoch()` is released with `arena.release(&segment, Readers::None)`. It returns the sum of all three counts. It takes no structure lock, because the Communication-Slot calls it after releasing it.
  - The regions that `ManifestStore::reclaim()` returns are counted and not released again here. `Eviction` and the deferred-return list own the release.
- **`segment_directory()`** implements `SegmentDirectory`: `binding(id)` is `store.current().get(id)` mapped to `SegmentBinding{assignment: entry.assignment().clone(), states: entry.states().clone()}`.
- **Epoch rule.** Only the manifest generation is visible to hits. Publishing a segment and removing one change it. `UpdateQueue::apply` does not. A batch that ran on an older generation gets `StaleMapping`, and the Communication-Slot repeats under the structure lock.

### 4.13 Concurrency

- All methods take `&self` and may be called from any thread.
- Lock order is: structure lock → updates mutex (held only by the Communication-Slot) → residency mutex → records lock (leaf) → internals of the shared structures.
  - The sync lock is taken before the records lock and held across `write_kernel_table`. The structure lock is not held for it.
  - The Reconciler mutex covers only the `reconcile` call.
- The structure lock is taken by `publish_segment`, `release_segment`, `evict_step` and `make_room`. It is not reentrant, so the coordinator must not hold it when it calls them.
- The records lock is never held across `LazyIndex::get`, `load_aggregated`, `load_entry`, `Index` calls, `write_visible`, `Event::wait` or `write_kernel_table`.
- Lookups (`slot_of`, `global_slot_of`, `entry_at_global_slot`, `stats`, counters) take only short locks. They never wait for ensure, reconcile, sync or index IO.
- Poisoned locks are recovered. There is no panic on valid use. All errors are `GpuManagerError`.
- Equal call sequences against equal index contents give equal slot assignments, because slots are handed out in ascending order.

## 5. Relied-on elements

- **`manager`:** `trait LazyIndex: Send + Sync { fn get(&self) -> Result<Arc<dyn Index>, String>; }`. `get` activates on its first success and then returns the same `Arc`. A failure is not cached.
- **`segments`:** `SegmentId(pub u64)` and `SlotRef { segment: SegmentId, slot: kernel_layout::Slot }`. They are defined once. `reconcile`, `address_map` and `manifest` re-export them, so no conversion is needed anywhere.
- **`slot_layout` (062):**
  - `slot_size(SlotClass, dim, DType) -> Result<u32, SlotError>`.
  - `SegmentView::new(RegionHandle, slot_count, SlotClass, dim, DType, Generation)`, with `slot_count()`, `class()`, `slot_bytes()`, `region()` and `encode_slot(tokens, &[f32]) -> Result<Vec<u8>, SlotError>` (padding rows hold the mask value).
  - `SlotStates` with `new`, `is_valid`, `invalidate`, `snapshot_words`.
  - `SlotAssignment` with `fixed`, `lookup(Slot) -> Option<EntryRef>`, `slot_of_entry`, `byte_range`.
  - `EntryRef { entry: EntryId, section: u32 }`.
- **`segment_writer` (101):** `SegmentWriter::new(view, device_base)`, `reserve(entry, tokens, centroid) -> Reservation{slot, target: Range<u64>, ..}`, `commit(&Reservation, &Event)`, `free_slots()`, `states()`, `assignment()`. Slots are handed out in ascending order and never reused. A commit binds the slot and sets the valid bit last.
- **`update_queue` (109):**
  - `UpdateQueue::{push_bind, push_unbind}(&self, EntryRef, SlotRef, Stamp)`.
  - `contains`, `get`, `depth`, `apply(&self, &ResidencyDirectory, &AddressMap) -> usize`, `take_warnings`.
  - One update per entry; the newest replaces the older. `apply` runs Unbinds first, then Binds, and fills or empties the directory. It never changes `SlotStates`.
- **`address_map` (083):** `Entry::new(SegmentId, SegmentView, Addr, Arc<SlotStates>, Arc<SlotAssignment>)`, `MappingStore::{next, publish, current}`, `AddressMap::{add, remove, get}` and `MapError::StaleMapping`.
- **`manifest` (102):** `ManifestStore::{current, successor, publish, reclaim}`, `SegmentSpec::from_view(id, &view, device_base)`, `ManifestError::StaleGeneration`.
- **`table_swap` (074):** `TableSwap::{next_table, publish, published_epoch, min_reader_epoch}`, `SegmentTable::{add, remove}` and `TableSwapError`.
- **`eviction` (103):**
  - `track(EvictKey, TrackSpec, Instant)`, `touch`, `remove(key) -> Option<u64>`, `bytes(Tier::Vram)`.
  - `tick(now) -> EvictionOutcome`, `make_room(Tier, need) -> Result<EvictionOutcome, EvictionError>`, `reclaim(min_reader_epoch)`.
  - `EvictionOutcome.decisions[].victim: EvictKey(u64)`.
  - Eviction removes the segment from the table and the map, defers the region and frees it later through the allocators. It does not touch the manifest, the directory or `SegmentArena`.
  - `EvictKey(pub u64)`, `Tier::Vram` and `TrackSpec{tier, segment: SegmentId, region: Region, size, reload_cost: f64, summary}` are used as in the Eviction spec.
- **`reconcile` (100):**
  - `ResidencyDirectory` (`insert`, `remove`, `get`, `set_known_stamp(id, Stamp) -> bool`, `iter`), `&self`, `Send + Sync`.
  - `Reconciler::new(&Arc<dyn Index>)` and `reconcile(&mut self, &ResidencyDirectory) -> Option<Reconciliation>`. It acks the signal before comparing. It never modifies the directory.
  - `Reconciliation{removed, reload, current, watermark, clean}`.
  - `load_entry(&dyn Index, EntryId) -> Option<LoadedEntry{stamp, payload, retries}>`. Order is stamp, read, stamp. The returned stamp is never newer than the data.
  - The stamp from a write's return value goes into the directory (echo suppression). A later `stamp()` result must never be entered as an echo.
- **`scope_resolution`:** `ScopeResolverRegistry::resolve(&self, &Scope, &ResolveContext{index: Option<&dyn Index>, cell_selector}) -> Result<EntrySet, ResolveError>`. It never falls back to all entries.
- **`load_aggregation` (124):** `load_aggregated<I>(Vec<LoadRequest{index: I, scope}>, &ScopeResolverRegistry, &(dyn Fn(&I) -> Option<Arc<dyn Index>> + Sync), Option<&dyn CellSelector>) -> Vec<IndexLoad{index, result: Result<Loaded, ResolveError>}>`. `Loaded = Vec<(EntryId, Option<LoadedEntry>)>`, ascending. It blocks until the load is done.
- **`consistency` (086):** `IndexWriter::new(Arc<dyn Index>)` and `write(id, PayloadRef{vector, meta}) -> Result<EntryIdentity{id, stamp: Stamp}, IndexError>`.
- **`index_contract`:** `EntryId(u64)`, `Stamp(u64)` (totally ordered, defined in `change_stamp`; it is the only stamp type), `IndexError`, `PayloadRef{vector, meta}`, `Index: Send + Sync` with `dtype`, `stamp`, `stamps`, `read`, `write`, `write_many`, `delete`, `subscribe` and the optional access methods. The index serializes its writers.
- **`segment_arena`:** `SegmentArena<A>::allocate(len) -> Result<Segment{region: Region, offset, len}, SegmentArenaError>`, `release(&Segment, Readers::None)` and `Exhausted{requested, available}`. `offset` is 256-aligned and lies inside `region`.
- **`region_handle`:** `RegionHandle::new(ArenaKind, offset, len, Generation(u64)) -> Result<RegionHandle, RegionError>`. A `bookkeeping::Region` generation is `u32`.
- **`device_lane_event`:** `Event` (`wait`, `is_complete`) and `Lane`.
- **`communication_slot` (119):** the traits `MappingSource`, `StructureGuard` and `UpdateQueue`, and `MappingSnapshot` and `UpdateOutcome`.
- **`hit_resolution` (114):** `SegmentDirectory` and `SegmentBinding{assignment, states}`.
- **`gpu_kernel` (118, 119c):** `encode_segment_table(&[SegmentEntry]) -> Vec<u8>` and `SegmentEntry::to_bytes`. `SegmentEntry` (`repr(C)`, 40 bytes): `base u64, lens_addr u64, stride_bytes u64, first_slot u32, slot_count u32, pad_len u32, _reserved u32`.
  - The kernel finds the segment of a ring slot via `first_slot <= slot < first_slot + slot_count`.
  - The slot address is `base + (slot - first_slot) * stride_bytes`.
  - The effective length is `min(lens[slot - first_slot], pad_len)`. With `lens_addr == 0` it is `pad_len`.
- **`half`:** F16 and BF16 to f32 conversion.
