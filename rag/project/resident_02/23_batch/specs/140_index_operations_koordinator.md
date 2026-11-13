# Index-Operations-Koordinator: Technical Specification (build order 140)

Crate module `index_ops`.

## 1. Task, restrictions and boundaries

**Task.** The coordinator is the operations unit of one index operation. It connects these parts:
- `IndexManager` (lifecycle manager)
- `TableSwap`
- `MappingStore`
- `UpdateQueue`
- `Eviction`
- `SegmentArena`
- a queue of loaded segments (`LoadedQueue`)

Its duties:
- It publishes every segment the load reports, on its own, through its own `publish_pending`.
- It serializes structural changes.
- When a load reports "arena exhausted", it makes room through `Eviction` and retries exactly once.
- It maps management errors to `IndexOpError`.

**Restrictions.**
- It does not allocate or fill segments, and it does not load entries. Loading is a caller closure.
- It does not call `UpdateQueue::apply`. Queued bindings take effect in the next gap, which the caller applies.
- It does not run reconcile logic or own a `ResidencyDirectory`. A reconcile iteration is a caller closure.
- It does not spawn threads.
- It does not redefine types of other modules.
- It has no `unsafe` and no globals. Crates: `log` plus the project modules in §5.
- `GpuManager` is not called. Its error type is only mapped (§4.7).

## 2. Context

- A load (a caller closure, for example one built on a `GpuManager` or a loader) allocates `Segment`s from the `SegmentArena` and fills them. It reports each as a `LoadedSegment` into the `LoadedQueue`. It never publishes.
- The coordinator publishes in this order:
  1. a new table version (`TableSwap`), then
  2. a new mapping epoch (`MappingStore`).
- Then it tracks the segment in `Eviction` and enqueues the segment's bindings into the `UpdateQueue`.
- Segment IDs are `address_map::SegmentId(u32)`. The same number is used as `reconcile::SegmentId(u32)` in `SlotRef` and as `EvictKey(u64)`.

## 3. Interface

```rust
use std::sync::{Arc, Mutex};
use address_map::{AddressMap, MapError, MappingStore};
use allocator::Allocator;
use change_stamp::ChangeStamp;
use eviction::{Eviction, EvictionError};
use index_contract::{Index, IndexError};
use kernel_layout::{Addr, Slot};
use manager_lifecycle::{IndexManager, ManagerError};
use segment_arena::{Segment, SegmentArena, SegmentArenaError};
use slot_layout::{EntryRef, SegmentView, SlotAssignment, SlotStates};
use table_swap::{TableSwap, TableSwapError};
use update_queue::UpdateQueue;
use gpu_manager::GpuManagerError;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum IndexOpError {
    /// Display: "Arena erschöpft". Stays as is whenever a source reports exhaustion.
    ArenaExhausted { requested: u64, available: u64 },
    /// Every other management error (text of the cause).
    Transfer(String),
}
impl std::fmt::Display for IndexOpError {}
impl std::error::Error for IndexOpError {}
// Send + Sync + 'static.
impl From<ManagerError> for IndexOpError {}
impl From<TableSwapError> for IndexOpError {}
impl From<MapError> for IndexOpError {}
impl From<SegmentArenaError> for IndexOpError {}   // Exhausted -> ArenaExhausted
impl From<EvictionError> for IndexOpError {}       // Exhausted -> ArenaExhausted
impl From<GpuManagerError> for IndexOpError {}     // Arena(Exhausted) -> ArenaExhausted
impl From<IndexError> for IndexOpError {}

/// Binding delivered with a loaded segment; becomes an Update (Bind) in the UpdateQueue.
#[derive(Clone, Copy, Debug)]
pub struct LoadedBinding { pub entry: EntryRef, pub slot: Slot, pub stamp: ChangeStamp }

/// A filled segment reported by a load. Not yet visible to readers.
pub struct LoadedSegment {
    pub segment: Segment,                    // from SegmentArena::allocate
    pub view: SegmentView,
    pub device_base: Addr,
    pub states: Arc<SlotStates>,
    pub assignment: Arc<SlotAssignment>,
    pub bindings: Vec<LoadedBinding>,
}

/// FIFO queue of reported segments. Clone shares the queue. Send + Sync.
#[derive(Clone, Default)]
pub struct LoadedQueue { /* Arc<Mutex<VecDeque<LoadedSegment>>> */ }
impl LoadedQueue {
    pub fn report(&self, seg: LoadedSegment);   // append; never blocks on publish
    pub fn len(&self) -> usize;
    pub fn is_empty(&self) -> bool;
}

/// Send + Sync. All methods &self.
pub struct IndexOperations { /* ... */ }
impl IndexOperations {
    pub fn new(manager: Arc<IndexManager>, swap: Arc<TableSwap>, store: Arc<MappingStore>,
               updates: Arc<UpdateQueue>, eviction: Arc<Eviction>,
               arena: Arc<SegmentArena<Arc<Allocator>>>) -> IndexOperations;

    pub fn loaded_queue(&self) -> LoadedQueue;   // handle for the load to report into

    /// Runs `op` as a load (see §4.4). `op` may be called twice.
    pub fn load<R>(&self, op: impl FnMut(&Arc<dyn Index>, &LoadedQueue) -> Result<R, IndexOpError>)
        -> Result<R, IndexOpError>;

    /// Runs one reconcile iteration `op`, publishing newly loaded segments before returning (see §4.5).
    pub fn reconcile_iteration<R>(&self, op: impl FnOnce(&Arc<dyn Index>, &LoadedQueue) -> Result<R, IndexOpError>)
        -> Result<R, IndexOpError>;

    /// Publishes all queued segments. Ok(count published) if none failed, else Err(first error).
    pub fn publish_pending(&self) -> Result<usize, IndexOpError>;
}
```

## 4. Behavior and invariants

### 4.1 Segment ID and publish of one loaded segment
Steps run under the structure lock (§4.6). The segment ID is `next_id` (a counter starting at 0).
1. `Entry::new(SegmentId(id), view, device_base, states, assignment)`.
2. `m = store.next()`, then `m.add(entry)`. This is a pre-check on a local map, so overlap and duplicate IDs are caught before anything is published.
3. Table: `t = swap.next_table()`, then `t.add(segment.region)`, then `swap.publish(t)`. On `StaleMapping`, retry with a fresh `next_table()`, up to 3 attempts in total.
4. Mapping: `store.publish(m)`. On `MapError::StaleMapping`, rebuild `m = store.next()` plus `add` and retry, up to 3 attempts in total.
5. On success:
   - `next_id += 1`. A failed publish consumes no ID. If `id > u32::MAX`, the result is `Transfer`.
   - `eviction.track(EvictKey(id as u64), TrackSpec{tier: Tier::Vram, segment: address_map::SegmentId(id), region: segment.region, size: segment.region.len, reload_cost: segment.region.len as f64, summary: false}, Instant::now())`.
   - For each binding, in order: `updates.push_bind(b.entry, SlotRef{segment: reconcile::SegmentId(id), slot: b.slot}, b.stamp)`. They take effect in the next gap, because `apply` is not called here.
   - `log::info!` with the ID, the table epoch and the mapping epoch.

**Order.** The table is always published before the mapping. `track` and the binding enqueue happen only after both publishes succeed.

### 4.2 Errors and return of the region
Every error is an `IndexOpError` (§4.7). The region is returned to the arena as follows:
- **Failure before the table publish** (step 1, 2 or 3, including an overlap):
  - The region was never published.
  - Release it immediately with `arena.release(&segment, Readers::None)`.
  - Nothing is tracked and no binding is queued.
- **Mapping failure after the table publish** (step 4):
  - Roll back the table: `t = next_table()`, then `t.remove(&region)`, then `publish`. Epoch `e` is the result.
  - Put `(segment, e)` on an internal deferred-return list.
  - Release the segment with `Readers::None` once `swap.min_reader_epoch() >= e`. This check runs at the start of every `publish_pending` and `make_room`.
  - If the rollback publish itself fails, log it with `log::error!` and do not release the region, because it is still published.
- **Release errors:** a failed `arena.release` is logged with `log::warn!`. It never replaces the original error.

### 4.3 publish_pending
- It takes the structure lock, then drains the `LoadedQueue` and publishes each segment in queue order. IDs therefore ascend in queue order.
- It tries all segments. A failing segment does not stop the others.
- It returns `Ok(n)` with the number published if there was no error. Otherwise it returns `Err(first error)`, and the other segments are still published.
- An empty queue gives `Ok(0)`.
- It never returns the same region twice and never publishes a segment twice.

### 4.4 load
1. `manager.run_cycle("load", ...)` supplies the index and the diagnostic span.
   - `ManagerError` becomes `Transfer` (via `From`).
   - `op` is not called if no index is attached.
2. Attempt 1: call `op(&index, &queue)`. Then always call `publish_pending()`. This publishes every segment the load reported, whether `op` returned `Ok` or `Err`.
3. Result of attempt 1:
   - `op` Ok and publish Ok: `Ok(r)`.
   - `op` Ok and publish Err: `Err(publish error)`.
   - `op` Err other than `ArenaExhausted`: that error (a publish error is only logged).
   - `op` Err `ArenaExhausted{requested, ..}`: go to step 4.
4. Making room:
   - The pending segments were already published in step 2. A failure there is logged and does not stop step 4.
   - Under the structure lock, call `eviction.reclaim(swap.min_reader_epoch())`, then `eviction.make_room(Tier::Vram, requested)`, then `reclaim` again.
   - `Err(Exhausted)` is returned as `ArenaExhausted` and there is no retry.
   - If `outcome.aborted.is_some()`, return `Transfer` and do not retry.
5. Retry exactly once: call `op` again, then `publish_pending()`, and map the result as in step 3. A second `ArenaExhausted` is returned as is. There is never a third attempt.
6. No lock is held while `op` runs. `op` may call `LoadedQueue::report` and may run concurrently with other operations.

### 4.5 reconcile_iteration
- It runs `op` inside `manager.run_cycle("reconcile", ...)` and then always calls `publish_pending()` before it returns.
- There is no exhaustion retry.
- Result:
  - If `op` returned Err, that error is returned and a publish error is only logged.
  - Else if publish returned Err, that error is returned.
  - Else the value of `op` is returned.
- No loaded segment stays in the `LoadedQueue` after the call returns.

### 4.6 Concurrency
- One structure mutex serializes all structural changes: segment publish, rollback, make-room and the deferred returns.
- Lock order: structure lock → internals of `TableSwap`, `MappingStore`, `Eviction`, `UpdateQueue`, `SegmentArena`. The queue mutex is a leaf lock.
- The structure lock is never held across `op`.
- Poisoned locks are recovered and nothing panics on valid use.
- All types are `Send + Sync`, which a static test checks.

### 4.7 Error mapping
- `ArenaExhausted{requested, available}` comes from:
  - `SegmentArenaError::Exhausted{requested, available}`
  - `EvictionError::Exhausted{requested, available}`
  - `GpuManagerError::Arena(SegmentArenaError::Exhausted{..})`
- Every other variant of those errors becomes `Transfer(e.to_string())`. This includes `GpuManagerError::NoSpace`.
- All other source errors also become `Transfer(e.to_string())`. This covers `ManagerError`, `TableSwapError`, `MapError` (including an overlap) and `IndexError`.

## 5. Relied-on elements

- **`manager_lifecycle`:**
  - `IndexManager::run_cycle<R,F: FnOnce(&Arc<dyn Index>) -> R>(&self, name: &str, f: F) -> Result<R, ManagerError>`. It returns `IndexUnavailable` without an attached index, does not call `f` in that case, and holds no lock while `f` runs.
  - `ManagerError` is `IndexUnavailable | InitFailed`. It has `Display` and is `Error`.
- **`table_swap`:**
  - `TableSwap::next_table() -> SegmentTable` (epoch = published + 1).
  - `TableSwap::publish(SegmentTable) -> Result<Epoch, TableSwapError>`. It returns `StaleMapping` if the base is outdated.
  - `TableSwap::min_reader_epoch() -> Epoch`.
  - `SegmentTable::{add(Region), remove(&Region) -> bool}`.
  - `TableSwapError` is `Clone + PartialEq`.
- **`address_map`:**
  - `Entry::new(SegmentId, SegmentView, Addr, Arc<SlotStates>, Arc<SlotAssignment>) -> Result<Entry, MapError>`.
  - `MappingStore::next() -> Result<AddressMap, MapError>` and `publish(AddressMap) -> Result<Epoch, MapError>`.
  - `AddressMap::add(Entry) -> Result<(), MapError>`. It can fail with `EmptySegment`, `DuplicateId`, `Overlap`.
  - `MapError::StaleMapping{..}` is the retry case.
  - `address_map::SegmentId(pub u32)`.
- **`update_queue`:** `UpdateQueue::push_bind(&self, EntryRef, SlotRef, ChangeStamp)`. It keeps one update per entry, the newest replaces the older. It is `Send + Sync`.
- **`reconcile`:** `SlotRef{segment: reconcile::SegmentId(u32), slot: Slot}`.
- **`eviction`:**
  - `Eviction::track(EvictKey, TrackSpec, Instant)`.
  - `TrackSpec{tier, segment: address_map::SegmentId, region, size, reload_cost: f64, summary: bool}`.
  - `Tier::Vram`.
  - `make_room(Tier, need) -> Result<EvictionOutcome, EvictionError>`. It evicts nothing on `Exhausted`. `EvictionOutcome.aborted: Option<Abort>`.
  - `reclaim(min_reader_epoch) -> usize`.
  - Eviction frees the regions it evicts itself.
- **`segment_arena`:**
  - `Segment{region: Region, offset, len}` (`Copy`).
  - `SegmentArena<Arc<Allocator>>::release(&Segment, Readers::None) -> Result<(), SegmentArenaError>`.
  - `SegmentArenaError::Exhausted{requested, available}`.
- **`bookkeeping::Region`** has fields `arena, offset, len: u64, generation: u32`.
- **`slot_layout`:** `SegmentView`, `SlotStates`, `SlotAssignment` and `EntryRef`.
- **`kernel_layout`:** `Addr`, `Slot`.
- **`change_stamp::ChangeStamp`** is `Copy`.
- **`index_contract`:** `Index`, `IndexError`.
- **`gpu_manager::GpuManagerError`** is used only for the error mapping.
- **Not used:** `Zustandsbasierter Abgleich`/`Ladestrategie` types appear only inside caller closures, and the coordinator does not call them.
- **Logging:** the `log` crate (`info!`, `warn!`, `error!`).