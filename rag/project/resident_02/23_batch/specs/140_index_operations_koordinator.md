# Index-Operations-Koordinator: Technical Specification (build order 140)

Crate module `index_ops`.

## 1. Task, restrictions and boundaries

**Task.** The coordinator is the operations unit of one index operation. It connects these parts:

- `IndexManager` (lifecycle manager)
- `GpuManager` (138), which owns the publish steps, the eviction calls and the shared structures of the sub-engine
- a queue of loaded segments (`LoadedQueue`, defined in 138)

Its duties:

- It publishes every segment the load reports, on its own, by calling `GpuManager::publish_segment`.
- It serializes the publication of reported segments.
- When a load reports "arena exhausted", it publishes the pending segments, makes room through `GpuManager::make_room` and retries exactly once.
- It maps management errors to `IndexOpError`.
- It implements `residency::ResidencyBackend` (125) for the index, so that the `ResidencyController` can drive it.

**Restrictions.**

- It does not allocate or fill segments, and it does not load entries. Loading is a caller closure. `GpuManager::ensure` is such a closure.
- It contains no publish step of its own. Segment ID, table, mapping, manifest, eviction tracking, bindings and the rollback are in `GpuManager::publish_segment` (138 §4.6).
- It does not call `UpdateQueue::apply`. Queued bindings take effect in the next gap, which the Communication-Slot applies.
- It does not run reconcile logic or own a `ResidencyDirectory`. A reconcile iteration is a caller closure.
- It holds no structure lock. `GpuManager` takes the shared structure lock inside the calls that need it.
- It does not spawn threads.
- It does not redefine types of other modules.
- It has no `unsafe` and no globals. Crates: `log` plus the project modules in §5.

## 2. Context

- A load (a caller closure, for example `|_, _| gpu.ensure(&set)`) places entries in segments. `GpuManager` reports each filled segment as a `LoadedSegment` into the `LoadedQueue`. It never publishes.
- The coordinator publishes in the order of the queue. `GpuManager::publish_segment` publishes a new table version first, then the mapping, then the manifest. After that it tracks the segment in `Eviction` and enqueues the segment's bindings into the `UpdateQueue`.
- The queue given to `IndexOperations::new` is the same queue that was given to `GpuManager::new`.
- The `IndexManager` given to `IndexOperations::new` is the only source of the index. The `GpuManager` of the same index gets its `LazyIndex` from `ManagedIndex::new(manager)`. `get()` never activates anything beyond what `IndexManager::index()` does and does not cache a failure.
- Segment IDs are `segments::SegmentId(u64)`. `GpuManager` assigns them.
- `Eviction` removes evicted segments from the table and the map. `GpuManager::make_room` wraps that and cleans up the residency.

## 3. Interface

```rust
use std::sync::{Arc, Mutex};
use eviction::EvictionError;
use index_contract::{Index, IndexError, EntryId};
use manager::LazyIndex;
use manager_lifecycle::{IndexManager, ManagerError};
use gpu_manager::{GpuManager, GpuManagerError, LoadedQueue};
use residency::{EnsureSummary, ReconcileSummary, ResidencyBackend};
use scopes::Scope;

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
impl From<GpuManagerError> for IndexOpError {}    // Arena(Exhausted) -> ArenaExhausted
impl From<EvictionError> for IndexOpError {}      // Exhausted -> ArenaExhausted
impl From<IndexError> for IndexOpError {}

/// Gives `GpuManager` its lazily attached index from the `IndexManager`. Implements `manager::LazyIndex`.
pub struct ManagedIndex { /* Arc<IndexManager> */ }
impl ManagedIndex {
    pub fn new(manager: Arc<IndexManager>) -> ManagedIndex;
}
impl LazyIndex for ManagedIndex {}     // get() is IndexManager::index(); an error becomes its Display text

/// Send + Sync. All methods &self.
pub struct IndexOperations { /* ... */ }
impl IndexOperations {
    pub fn new(manager: Arc<IndexManager>, gpu: Arc<GpuManager>, queue: LoadedQueue) -> IndexOperations;

    pub fn loaded_queue(&self) -> LoadedQueue;   // handle for the load to report into

    /// Runs `op` as a load (see §4.2). `op` may be called twice.
    pub fn load<R>(&self, op: impl FnMut(&Arc<dyn Index>, &LoadedQueue) -> Result<R, IndexOpError>)
        -> Result<R, IndexOpError>;

    /// Runs one reconcile iteration `op`, publishing newly loaded segments before returning (see §4.3).
    pub fn reconcile_iteration<R>(&self, op: impl FnOnce(&Arc<dyn Index>, &LoadedQueue) -> Result<R, IndexOpError>)
        -> Result<R, IndexOpError>;

    /// Publishes all queued segments. Ok(count published) if none failed, else Err(first error).
    pub fn publish_pending(&self) -> Result<usize, IndexOpError>;
}

impl ResidencyBackend for IndexOperations {}      // see §4.4
```

## 4. Behavior and invariants

### 4.1 publish\_pending

- It takes the publish mutex, drains the `LoadedQueue` and calls `gpu.publish_segment` for each segment in queue order. IDs therefore ascend in queue order.
- It tries all segments. A failing segment does not stop the others. `GpuManager` has already returned the region of the failed segment and rolled back what it had published (138 §4.6).
- It returns `Ok(n)` with the number published if there was no error. Otherwise it returns `Err(first error)`, and the other segments are still published.
- An empty queue gives `Ok(0)`.
- A segment is drained once. It is never published twice and its region is never returned twice.

### 4.2 load

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
   - Call `gpu.make_room(requested)`. It reclaims, makes room through `Eviction` and reclaims again (138 §4.8).
   - `Err(Exhausted)` is returned as `ArenaExhausted` and there is no retry.
   - If `outcome.aborted.is_some()`, return `Transfer` and do not retry.
5. Retry exactly once: call `op` again, then `publish_pending()`, and map the result as in step 3. A second `ArenaExhausted` is returned as is. There is never a third attempt.
6. No lock is held while `op` runs. `op` may call `LoadedQueue::report` and may run concurrently with other operations.

### 4.3 reconcile\_iteration

- It runs `op` inside `manager.run_cycle("reconcile", ...)` and then always calls `publish_pending()` before it returns.
- There is no exhaustion retry.
- Result:
  - If `op` returned Err, that error is returned and a publish error is only logged.
  - Else if publish returned Err, that error is returned.
  - Else the value of `op` is returned.
- No loaded segment stays in the `LoadedQueue` after the call returns.

### 4.4 ResidencyBackend

- `ensure(scope)` is `load(|_, _| { let set = gpu.resolve_scope(scope)?; gpu.ensure(&set) })`. It returns `EnsureSummary{loaded, already_resident, missing, failed}` with the lengths of the report lists. An error becomes its `Display` text.
- `reconcile()` is `reconcile_iteration(|_, _| gpu.reconcile_iteration())`. `Ok(None)` stays `None`. Otherwise the result is `ReconcileSummary{removed: reconciliation.removed.len(), reloaded, current}`.
- `pending()` is `gpu.signal_pending()`. It never blocks.
- `is_resident(entry)` is `gpu.slot_of(entry).is_some()`.
- `run_gap()` calls `publish_pending()` (an error is only logged) and then `gpu.run_gap()`. The result is the number published plus `slots_hidden` plus `reclaimed`.
- `shutdown()` calls `publish_pending()`, then `gpu.release_segment(id)` for each id in `gpu.segment_ids()`. It tries all and returns the number released. If one release failed, it returns the first error after the rest.

### 4.5 Concurrency

- One publish mutex serializes `publish_pending` and `shutdown`. It is the outermost lock.
- Lock order: publish mutex → the locks inside `GpuManager` (structure lock → updates mutex → residency mutex → records lock → internals of the shared structures, see 138 §4.13).
- The publish mutex is never held across `op`.
- Poisoned locks are recovered and nothing panics on valid use.
- All types are `Send + Sync`, which a static test checks.

### 4.6 Error mapping

- `ArenaExhausted{requested, available}` comes from:
  - `GpuManagerError::Arena(SegmentArenaError::Exhausted{requested, available})`
  - `EvictionError::Exhausted{requested, available}`
- Every other variant of those errors becomes `Transfer(e.to_string())`. This includes `GpuManagerError::Publish`, `Mapping` and `Manifest`.
- All other source errors also become `Transfer(e.to_string())`. This covers `ManagerError` and `IndexError`.

## 5. Relied-on elements

- **`manager_lifecycle`:**
  - `IndexManager::run_cycle<R,F: FnOnce(&Arc<dyn Index>) -> R>(&self, name: &str, f: F) -> Result<R, ManagerError>`. It returns `IndexUnavailable` without an attached index, does not call `f` in that case, and holds no lock while `f` runs.
  - `ManagerError` is `IndexUnavailable | InitFailed`. It has `Display` and is `Error`.
- **`gpu_manager` (138):**
  - `LoadedQueue::{report, drain, len, is_empty}` and `Clone`.
  - `GpuManager::{resolve_scope, ensure, reconcile_iteration, publish_segment, release_segment, make_room, run_gap, signal_pending, segment_ids, slot_of}`.
  - `GpuManagerError`, with `Arena(SegmentArenaError)`.
  - `EnsureReport`, `ReconcileOutcome` and `GapReport`.
- **`eviction`:** `EvictionError::Exhausted{requested, available}` and `EvictionOutcome.aborted: Option<Abort>`. `make_room` evicts nothing on `Exhausted`.
- **`segment_arena`:** `SegmentArenaError::Exhausted{requested, available}`.
- **`residency` (125):** `ResidencyBackend`, `EnsureSummary` and `ReconcileSummary`.
- **`scopes`:** `Scope`.
- **`index_contract`:** `Index`, `IndexError`, `EntryId`.
- **Logging:** the `log` crate (`info!`, `warn!`, `error!`).
