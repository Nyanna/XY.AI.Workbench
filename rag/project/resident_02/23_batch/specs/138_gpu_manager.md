# GPU-Manager: Technical Specification (build order 138)

Crate module `gpu_manager`.

## 1. Task, restrictions and boundaries

**Task.** The GPU-Manager manages the GPU-resident part of one index as a table of VRAM segments.

- **Ensure:** it loads entry sets (`ensure`).
- **Reconcile:** it runs reconcile iterations.
- **Segments:** it publishes new segments and releases old ones.
- **Reporting:** it reports the segment count, the resident bytes and the pending updates.
- **Sync and mirror-back:** it syncs the RAM representation of the table to the VRAM representation for the kernels, and it mirrors processed GPU results back into the index.
- **Scopes and eviction:** it resolves scopes through the resolver registry, and it exposes an eviction step and room-making.

**Restrictions.**
- It is a component of the GPU sub-engine. It is not part of the facade and not part of the index.
- It is constructed with the sub-engine (`Arc<dyn GpuSubengine>`, §3) and a lazily activated index (`LazyIndex`).
- It spawns no threads. The caller drives every operation.
- It never resolves a `RegionHandle` itself. Memory access and device addresses come only from `GpuSubengine`.
- It does not plan queries, run kernels or search. It does not score, choose lanes or create lanes.
- It does not implement `Index` and does not own the `Eviction`, `TablePublisher` or `SegmentArena` instances.
- It writes to the index only in `mirror_back`.
- It never activates the sub-engine. Lane failures come back as `VramSync`.
- No `unsafe`, no globals. Crates: `log` plus the project modules in §5.

## 2. Context

- **Sub-engine.** It provides the segment allocator (`SegmentArena`), the table publication (`TablePublisher`/`TableSwap`), the `Eviction`, the memory access to regions, and the VRAM write of the kernel table (via `GpuBackend`). The module sees all of this only through `GpuSubengine`.
- **Index.** It is the authority, and "the index wins". Entry vectors have a fixed size `slot_bytes`.
- **Kernels.** `gpu_kernel` reads slots through a `SegmentEntry[]` table in VRAM and a slot ring of global slot indices. This module provides the table and the mapping between entries and global slots.
- **Neighbours.**
  - `Manager` (136) is the generic per-sub-engine manager. This module reuses only its `LazyIndex` trait.
  - `SegmentStore` holds slots and the slot↔entry mapping.
  - `ResidencyDirectory` and `Reconciler` hold the known stamps.
  - `Eviction` tracks segments (GDSF).
  - `ScopeResolverRegistry` resolves scopes. `load_aggregated` loads entries.

## 3. Interface

```rust
use std::sync::Arc;
use std::time::Instant;
use index_contract::{EntryId, IndexError, Stamp};
use manager::LazyIndex;
use scopes::{EntrySet, Scope};
use scope_resolution::{CellSelector, ResolveError, ScopeResolverRegistry};
use reconcile::Reconciliation;
use segment_arena::{Readers, SegmentArena, SegmentArenaError};
use allocator::Allocator;
use bookkeeping::Region;
use eviction::{Eviction, EvictionError, EvictionOutcome};
use consistency::TablePublisher;
use table_swap::{TableSwap, TableSwapError};
use segments::SlotError;
use memory_access::Region as MemRegion;
use region_handle::RegionHandle;
use device_lane_event::Lane;

/// Implemented by the GPU sub-engine (on top of GpuBackend). Send + Sync.
pub trait GpuSubengine: Send + Sync {
    fn arena(&self) -> &SegmentArena<Arc<Allocator>>;   // arena IndexSegments
    fn publisher(&self) -> &TablePublisher;
    fn table_swap(&self) -> &TableSwap;                 // the swap behind publisher()
    fn eviction(&self) -> &Eviction;                    // Tier::Vram is used
    /// Bytes of the region; byte 0 = start of `handle`; mem.len() == handle.len().
    fn open(&self, handle: &RegionHandle) -> Result<MemRegion, String>;
    /// Device address of byte 0 of the region (for SegmentEntry.base).
    fn device_base(&self, handle: &RegionHandle) -> Option<u64>;
    /// Writes `table` (SegmentEntry[] bytes) to VRAM, ordered before work later submitted on `lane`.
    /// Returns the device address of the written table. Must not overwrite a table still in use by running kernels.
    fn write_kernel_table(&self, lane: &mut Lane, table: &[u8]) -> Result<u64, String>;
}

#[derive(Clone, Debug)]
pub struct GpuManagerConfig {
    pub slot_bytes: usize,     // vector size per slot, > 0
    pub segment_bytes: u64,    // requested length of a new segment, >= slot_bytes
    pub pad_len: u32,          // SegmentEntry.pad_len for every segment, >= 1
}

#[derive(Clone, Debug, PartialEq)]
pub enum GpuManagerError {
    InvalidConfig(String),                    // "ungültige GPU-Manager-Konfiguration: {0}"
    Index(String),                            // "Index nicht verfügbar: {0}"
    Resolve(ResolveError),
    InvalidVector { len: usize, expected: usize },
    NoSpace(String),                          // no slot and no segment obtainable
    SegmentLimit,                             // segment id > u32::MAX or global slot space (u32) exhausted
    UnknownSegment(u64),
    Publish(TableSwapError),
    Arena(SegmentArenaError),
    Slot(SlotError),
    Memory(String),                           // GpuSubengine::open failed
    VramSync(String),                         // write_kernel_table failed
    Write(IndexError),                        // mirror_back: index write failed
}
// Display + std::error::Error; Send + Sync + 'static.

#[derive(Debug, Default, Clone, PartialEq)]
pub struct EnsureReport {
    pub loaded: Vec<EntryId>,                 // newly placed, ascending
    pub already_resident: Vec<EntryId>,       // ascending
    pub absent_in_index: Vec<EntryId>,        // ascending
    pub failed: Vec<(EntryId, GpuManagerError)>,
    pub bytes_moved: u64,                     // sum of vector bytes written into slots
    pub segments_created: usize,
    pub segments_evicted: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ReconcileOutcome {
    pub reconciliation: Reconciliation,
    pub queued_deletes: usize,                // newly enqueued or replaced Delete items
    pub reloaded: usize,
    pub skipped_pending: usize,               // changed, but an update was already pending
    pub failed: usize,                        // reload failed; the next iteration re-detects it
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UpdateKind { Upsert, Delete }

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct UpdateReport { pub applied: usize, pub deleted: usize, pub dropped: usize, pub failed: usize }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GpuIndexStats { pub segments: usize, pub resident_bytes: u64, pub pending_updates: usize }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct VramTable { pub segments_addr: u64, pub segment_count: u32, pub version: u64 }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MirrorItem { pub id: EntryId, pub vector: Vec<u8>, pub meta: Vec<u8> }

#[derive(Debug, Default, Clone, PartialEq)]
pub struct MirrorReport {
    pub written: Vec<(EntryId, Stamp)>,
    pub failed: Vec<(EntryId, GpuManagerError)>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleasedSegment {
    pub id: u64,                              // segments::SegmentId.0
    pub region: Region,                       // region handed back to the allocator
    pub cleared: Vec<EntryId>,                // entries whose residency was removed, ascending
}

pub struct GpuManager { /* private */ }       // Send + Sync

impl GpuManager {
    /// Validates only. Loads nothing, allocates nothing, does not call LazyIndex::get.
    pub fn new(subengine: Arc<dyn GpuSubengine>, index: Arc<dyn LazyIndex>,
               resolver: Arc<ScopeResolverRegistry>, cell_selector: Option<Arc<dyn CellSelector>>,
               cfg: GpuManagerConfig) -> Result<GpuManager, GpuManagerError>;

    // Scope and load
    pub fn resolve_scope(&self, scope: &Scope) -> Result<EntrySet, GpuManagerError>;
    pub fn ensure(&self, set: &EntrySet) -> Result<EnsureReport, GpuManagerError>;

    // Reconcile and updates
    /// Ok(None): the index no longer exists.
    pub fn reconcile_iteration(&self) -> Result<Option<ReconcileOutcome>, GpuManagerError>;
    pub fn enqueue_update(&self, id: EntryId, kind: UpdateKind);
    pub fn apply_updates(&self, max: usize) -> Result<UpdateReport, GpuManagerError>;

    // Segments
    pub fn release_segment(&self, id: u64, readers: Readers) -> Result<ReleasedSegment, GpuManagerError>;
    pub fn evict_step(&self, now: Instant) -> EvictionOutcome;
    pub fn make_room(&self, need: u64) -> Result<EvictionOutcome, GpuManagerError>;

    // VRAM sync and mirror-back
    pub fn sync_to_vram(&self, lane: &mut Lane) -> Result<VramTable, GpuManagerError>;
    pub fn mirror_back(&self, items: &[MirrorItem]) -> Result<MirrorReport, GpuManagerError>;

    // Reporting and lookup
    pub fn stats(&self) -> GpuIndexStats;
    pub fn segment_count(&self) -> usize;
    pub fn resident_bytes(&self) -> u64;
    pub fn pending_updates(&self) -> usize;
    pub fn slot_of(&self, id: EntryId) -> Option<(u64 /*segment*/, u64 /*slot*/)>;
    pub fn global_slot_of(&self, id: EntryId) -> Option<u32>;
    pub fn entry_at_global_slot(&self, slot: u32) -> Option<EntryId>;
}
```

## 4. Behavior and invariants

### 4.1 Construction and state
- `new` returns `InvalidConfig` if `slot_bytes == 0`, `segment_bytes < slot_bytes` or `pad_len == 0`.
- The manager owns these parts, all created in `new`:
  - a `SegmentStore::new(slot_bytes)`;
  - a `ResidencyDirectory`;
  - a segment record list;
  - the update queue;
  - an `Option<Reconciler>` and an `Option<IndexWriter>`, both created lazily;
  - a RAM table version counter and a cache of the last VRAM sync.
- Each segment record holds: store `SegmentId`, `segment_arena::Segment`, `RegionHandle`, and a permanent `first_slot`.
  - `first_slot` is assigned at creation from a counter that only grows.
  - It is never reused and never compacted, so a segment's global slot numbers never change.
- Global slot = `first_slot + slot`.

### 4.2 Residency and mapping invariants
- An entry is resident iff the `ResidencyDirectory` contains it. The `SegmentStore` and the directory always agree.
  - The directory holds `SlotRef{segment: reconcile::SegmentId(sid as u32), slot: Slot(slot as u32)}`.
  - It holds the stamp that `load_entry` returned for the data in the slot.
- `slot_of`, `global_slot_of` and `entry_at_global_slot` are exact inverses over resident entries.
  - They read the current RAM state.
  - They do no IO and do not wait for ensure or for index calls.
- `stats()` reports:
  - `segments` = the number of segment records;
  - `resident_bytes` = `Eviction::bytes(Tier::Vram)` (the eviction view, live bytes);
  - `pending_updates` = the update queue length.
- `segment_count()`, `resident_bytes()` and `pending_updates()` return the same three values individually.

### 4.3 Scope resolution
- `resolve_scope` calls `ScopeResolverRegistry::resolve(scope, &ResolveContext{index: Some(&*index), cell_selector})`.
  - The index comes from `LazyIndex::get`.
  - An error from `get` gives `Index(e)`. A resolver error gives `Resolve(e)`.
  - There is no fallback to all entries.

### 4.4 Ensure
`ensure(set)` runs in this order:
1. Get the index with `LazyIndex::get`. An error gives `Index(e)` and nothing changes.
2. `missing` = the members of `set` (ascending) that are not in the directory. The others go to `already_resident`. Staleness is not checked, because reconcile owns that.
3. If `missing` is not empty, call `load_aggregated` with one request `LoadRequest{index: (), scope: Scope::Set(missing as EntrySet)}`.
   - `registry` is the resolver registry, `lookup` returns the index, and `cell_selector` is passed through.
   - An `Err` result gives `Resolve(e)`. In that case nothing is placed.
   - No manager lock is held during this step. The loader may block on IO.
4. For each loaded entry, in ascending order:
   - `None`: add it to `absent_in_index`.
   - `Some(le)`: place it (§4.5).
5. After success, call `Eviction::touch(EvictKey(sid), Instant::now())` for **all** segment records, not only for the segments touched by this call. This also happens when nothing was loaded.
6. Log with `log::info!`: the entry count, `bytes_moved`, and the segments created and evicted.

Further rules:
- A per-entry failure goes into `failed`. It does not abort the other entries.
- Concurrent `ensure` calls for the same entry place it once. Placement re-checks residency under the structure lock. If the entry is already resident, it counts as `already_resident`.
- A vector length other than `slot_bytes` gives `failed(InvalidVector{len, expected: slot_bytes})`.

### 4.5 Placement, segment creation and publication
All of this runs under the structure lock `place_lock` and holds no index call.

**Placing one entry**
1. `SegmentStore::put(entry, vector)` picks the lowest free slot.
2. On `NoFreeSlot`, run `create_segment` and retry `put`.
3. On success, `directory.insert(id, SlotRef, le.stamp)` and add the vector length to `bytes_moved`.

**`create_segment`**
1. `arena.allocate(cfg.segment_bytes)`.
   - On `Exhausted{requested,..}`: call `Eviction::reclaim(swap.min_reader_epoch())`, then internal `make_room(requested)`, then retry the allocation once.
   - If it still fails, the entry fails with `NoSpace`.
   - Other arena errors give `Arena(e)`.
2. Build `RegionHandle::new(ArenaKind::IndexSegments, seg.offset, seg.len, Generation(seg.region.generation as u64))`.
3. `subengine.open(&handle)`. An error gives `Memory`.
4. `store.add_segment(handle, mem)`. An error gives `Slot`.
5. Check that the id is `<= u32::MAX` and that `first_slot + slot_count <= u32::MAX`. Otherwise give `SegmentLimit`.
6. Publish with `publisher.apply(|t| { t.add(seg.region); })`. An error gives `Publish(e)`.
7. `Eviction::track(EvictKey(id.0), TrackSpec{tier: Vram, segment: address_map::SegmentId(id.0 as u32), region: seg.region, size: seg.region.len, reload_cost: seg.region.len as f64, summary: false}, now)`.
8. Add the record and increase the RAM table version by 1.

If steps 3 to 6 fail, roll back. Call `store.remove_segment` if it was added, then `arena.release(&seg, Readers::None)`. Nothing stays tracked or published.

### 4.6 Reconcile iteration
`reconcile_iteration` runs in this order:
1. Get the index with `LazyIndex::get`. An error gives `Index(e)`.
2. On the first call, create `Reconciler::new(&index)` and keep it. It holds the index weakly. The Reconciler mutex is held only during the `reconcile` call.
3. `reconciler.reconcile(&directory)`. `None` gives `Ok(None)`.
4. For every removed entry (`for_each_delete`), enqueue `Delete`. `queued_deletes` counts these.
5. For every entry in `reload`:
   - If an update is pending for it (the queue contains an item), skip it and count `skipped_pending`.
   - Otherwise reload it regularly with `reconcile::load_entry`:
     - `None`: enqueue `Delete`.
     - `Some(le)` with a wrong vector length: count it in `failed`.
     - `Some(le)` otherwise: under the structure lock, if the entry is still resident, call `store.put` (same slot, overwritten in place) and `directory.insert(id, same slot, le.stamp)`. Count it in `reloaded`. If the entry is no longer resident, drop it silently.
   - On a load or write failure the directory stays unchanged, and the next iteration finds the difference again.
6. Return `Ok(Some(ReconcileOutcome{..}))`. The `Reconciliation` is passed through unchanged.

Rules for this operation:
- The iteration never uploads non-resident entries.
- It does not modify the index.
- An empty directory gives a clean `Reconciliation`.

### 4.7 Update queue
- Items are `Upsert` or `Delete`, FIFO by first enqueue, with at most one item per entry.
- A newer item for the same entry replaces the older one and keeps its position.
  - `Delete` replaces `Upsert`.
  - `Upsert` never replaces `Delete`.
- `apply_updates(max)` processes up to `max` items from the head. For each item:
  - Entry not resident: drop the item silently (`dropped`).
  - `Delete`: `store.remove(entry)` and `directory.remove(entry)`. Count it in `deleted`.
  - `Upsert`: call `load_entry`.
    - `None` is handled like `Delete`.
    - Otherwise write in place and update the directory stamp (`applied`).
    - On a length or write error, count it in `failed`. The item is removed.
  - A segment that was removed in the meantime makes the item fail harmlessly (`dropped`). It never produces an error or a panic.
- `apply_updates` needs the index only for `Upsert` items. An index error gives `Index(e)`.

### 4.8 Releasing and evicting segments
**`release_segment(id, readers)`** runs under the structure lock, in this order:
1. Look up the record. If it is missing, return `UnknownSegment(id)` and change nothing.
2. `publisher.apply(|t| { t.remove(&region); })`. An error gives `Publish(e)` and nothing is changed.
3. `Eviction::remove(EvictKey(id))`.
4. `store.remove_segment`. Remove the residency (`directory.remove`) of **every** entry in `RemovedSegment.entries`. Drop their queue items. Remove the record and increase the RAM table version.
5. `arena.release(&segment, readers)` returns the region to the allocator. An error here comes back as `Arena(e)`, but the state from steps 2 to 4 stays cleaned.

The result is `ReleasedSegment{id, region, cleared}`.

**`evict_step(now)`** runs under the structure lock:
1. `Eviction::reclaim(swap.min_reader_epoch())`.
2. `Eviction::tick(now)`.
3. For every decision whose key is a segment record of this manager, clean up:
   - `store.remove_segment`, remove the residency of its entries, drop their queue items, remove the record, and increase the RAM table version.
4. The `EvictionOutcome` is returned unchanged.

Rules for eviction:
- Eviction itself removes the segment from the table and defers the region. The manager never calls `arena.release` for evicted segments.
- Keys that are not records of this manager are ignored.

**`make_room(need)`** runs under the structure lock:
1. `Eviction::reclaim(...)`.
2. `Eviction::make_room(Tier::Vram, need)`.
3. Clean up the decisions as in `evict_step`.
4. `Err(Exhausted)` gives `GpuManagerError::NoSpace` and changes nothing.

### 4.9 VRAM sync
`sync_to_vram(lane)`:
- It takes the sync lock. It reads the RAM version and a snapshot of the records together, under the state lock.
- If the version equals the last synced version, it returns the cached `VramTable` without a lane operation. This is the "effective" sync.
- Otherwise it builds `SegmentEntry` records, in ascending store-id order:
  - `base = subengine.device_base(&handle)`. A missing base gives `VramSync`.
  - `lens_addr = 0`.
  - `stride_bytes = slot_bytes`.
  - `first_slot` and `slot_count` come from the record.
  - `pad_len = cfg.pad_len`.
  - `_reserved = 0`.
- It serializes the records as `repr(C)`, 40 bytes each, native-endian.
- It calls `subengine.write_kernel_table(lane, bytes)`. An error gives `VramSync(e)`, and the cache is unchanged.
- On success it stores `(version, addr)` and returns `VramTable{segments_addr, segment_count, version}`.
- With zero segments it writes nothing and returns `segments_addr: 0, segment_count: 0`.
- The caller must call it with the lane before it starts an operation on that lane. The caller puts `segments_addr` and `segment_count` into `KernelParams`.

### 4.10 Mirror-back
`mirror_back(items)`:
1. Get the index with `LazyIndex::get`. An error gives `Index(e)`. The `IndexWriter` is created on the first call and then reused.
2. For each item, in order:
   - A vector length other than `slot_bytes` gives `failed(InvalidVector)`.
   - Otherwise call `IndexWriter::write(id, PayloadRef{vector, meta})`.
     - An `IndexError` gives `failed(Write(e))`.
     - Success gives `written.push((id, Stamp(identity.stamp.as_raw())))`.
     - If the entry is resident, call `directory.set_known_stamp(id, stamp)` with exactly the stamp returned by the write. This is echo suppression, so the next reconcile does not reload the entry.
     - A non-resident entry only gets the index write.
3. The slot is not rewritten, because the processed result already lives in VRAM. The caller guarantees that the slot data equals the mirrored vector.
4. A failure of one item does not abort the others.

### 4.11 Concurrency
- All methods take `&self` and may be called from any thread.
- There are four lock groups:
  - `place_lock` serializes changes to the segment structure: creation, placement writes, release and eviction cleanup.
  - The state mutexes (records, queue, caches) are short leaf locks.
  - The sync lock serializes `sync_to_vram`.
  - The Reconciler mutex covers only the `reconcile` call.
- Lock order is: sync lock → state locks, and `place_lock` → state locks.
  - `place_lock` may be held while calling the sub-engine, arena, publisher, eviction, store and directory.
  - It is never held across `LazyIndex::get`, `load_aggregated`, `load_entry` or `Index` calls.
  - It is never held across `write_kernel_table`. The sync lock is.
- Lookups (`slot_of`, `global_slot_of`, `entry_at_global_slot`, `stats`, counters) take only short state locks. They never wait for ensure, reconcile, sync or index IO.
- Poisoned locks are recovered. There is no panic on valid use. All errors are `GpuManagerError`.
- Equal call sequences against equal index contents give equal slot assignments, because the store picks the lowest free slot.

## 5. Relied-on elements

- **`manager`:** `trait LazyIndex: Send + Sync { fn get(&self) -> Result<Arc<dyn Index>, String>; }`. `get` activates on its first success and then returns the same `Arc`. A failure is not cached.
- **`segments` (`SegmentStore`, `Send + Sync`):**
  - `new(slot_size: u64)`.
  - `add_segment(RegionHandle, MemRegion) -> Result<SegmentId(u64), SlotError>`. The handle must be in arena `IndexSegments`, and `mem.len() == handle.len()`.
  - `remove_segment(id) -> Result<RemovedSegment{id, handle, entries(asc)}, SlotError>`. It removes the mapping and frees nothing.
  - `put(EntryId, &[u8]) -> Result<SlotRef{segment, slot: u64}, SlotError>`.
    - A new entry takes the lowest free `(segment, slot)`. An existing entry is overwritten in the same slot.
    - Errors are `NoFreeSlot` and `PayloadTooLarge`.
  - `remove(EntryId) -> Option<SlotRef>`.
  - `slot_of(EntryId) -> Option<SlotRef>`.
  - `entry_at(SlotRef) -> Option<EntryId>`.
  - `slot_count(SegmentId) -> Option<u64>`.
  - `stats()`.
- **`reconcile`:**
  - `ResidencyDirectory` (`new/insert(id, SlotRef, Stamp)/remove/get/stamp/set_known_stamp(id, Stamp) -> bool/len/iter`) with `SlotRef{segment: SegmentId(u32), slot: kernel_layout::Slot(u32)}`.
  - `Reconciler::new(&Arc<dyn Index>)` and `reconcile(&mut self, &ResidencyDirectory) -> Option<Reconciliation>`. It acks the signal before comparing. It never modifies the directory.
  - `Reconciliation{removed, reload, current, watermark, clean}` and `for_each_delete`.
  - `load_entry(&dyn Index, EntryId) -> Option<LoadedEntry{stamp, payload, retries}>`. Order is stamp, read, stamp. The returned stamp is never newer than the data.
  - The stamp from the write's return value goes into the directory (echo suppression). A later `stamp()` result must never be entered as an echo.
- **`scope_resolution`:**
  - `ScopeResolverRegistry::resolve(&self, &Scope, &ResolveContext{index: Option<&dyn Index>, cell_selector: Option<&dyn CellSelector>}) -> Result<EntrySet, ResolveError>`.
  - It never falls back to all entries.
  - `ResolveError: Clone + PartialEq`.
- **`load_aggregation`:**
  - `load_aggregated<I>(Vec<LoadRequest{index: I, scope}>, &ScopeResolverRegistry, &(dyn Fn(&I) -> Option<Arc<dyn Index>> + Sync), Option<&dyn CellSelector>) -> Vec<IndexLoad{index, result: Result<Loaded, ResolveError>}>`.
  - `Loaded = Vec<(EntryId, Option<LoadedEntry>)>`, ascending. `None` means the entry is missing in the index.
  - It blocks until the load is done.
- **`consistency`:**
  - `TablePublisher::apply(FnMut(&mut SegmentTable)) -> Result<Epoch, TableSwapError>` is serialized. It retries `StaleMapping` at most 3 times.
  - `IndexWriter::new(Arc<dyn Index>)` and `write(id, PayloadRef{vector, meta}) -> Result<EntryIdentity{id, stamp: ChangeStamp}, IndexError>`.
  - `ChangeStamp::as_raw()`.
- **`table_swap`:** `SegmentTable::{add(region), remove(&Region) -> bool}` and `TableSwap::min_reader_epoch()`. `TableSwapError` is `Clone + PartialEq`.
- **`eviction`:**
  - `track(EvictKey, TrackSpec, Instant)`, `touch`, `remove(key) -> Option<u64>`, `bytes(Tier::Vram)`.
  - `tick(now) -> EvictionOutcome`, `make_room(Tier, need) -> Result<EvictionOutcome, EvictionError>`, and `reclaim(min_reader_epoch)`.
  - `EvictionOutcome.decisions[].victim: EvictKey(u64)`.
  - Eviction removes the segment from the table, defers the region and later frees it through the allocators. `SegmentArena::release` is not used for it.
  - `EvictKey(pub u64)`, `Tier::Vram`, and `TrackSpec{tier, segment: address_map::SegmentId(u32), region: Region, size, reload_cost: f64, summary}` are used as in the Eviction spec.
- **`segment_arena`:**
  - `SegmentArena<A>::allocate(len) -> Result<Segment{region: Region, offset, len}, SegmentArenaError>`.
  - `release(&Segment, Readers::{None, After(Event)})`.
  - `Exhausted{requested, available}`.
  - The segment is ready for use after `allocate`. `offset` is 256-aligned and lies inside `region`.
- **`region_handle`:** `RegionHandle::new(ArenaKind, offset, len, Generation(u64)) -> Result<RegionHandle, RegionError>`. A `bookkeeping::Region` generation is `u32`.
- **`gpu_kernel`:** `SegmentEntry` (`repr(C)`, 40 bytes): `base u64, lens_addr u64, stride_bytes u64, first_slot u32, slot_count u32, pad_len u32, _reserved u32`.
  - The kernel finds the segment of a ring slot via `first_slot <= slot < first_slot + slot_count`.
  - The document address is `base + (slot - first_slot) * stride_bytes`.
  - For `fused_maxsim`, `pad_len >= 1`.
- **`gpu_backend`:** the sub-engine implements `GpuSubengine` on top of `GpuBackend`. Device addresses leave the backend only through `GpuSubengine::device_base` and `write_kernel_table`.
- **`index_contract`:** `EntryId(u64)`, `Stamp(u64)`, `IndexError`, `PayloadRef{vector, meta}`. `Index` is `Send + Sync`.