# Manager: Technical Specification (build order 136)

## 1. Task, restrictions and boundaries

**Task.** Implement the Rust module `manager`. It is the per-subengine manager of one index's device-resident data. It does the following:
- It maps each `EntryId` to a `(segment, slot)` address.
- It keeps the residency state of entries.
- It keeps the update queue.
- It uploads entries, evicts them, and applies updates.
- It maps addresses back to `EntryId`.

**Restrictions.**
- The Manager is a component of one sub-engine. It is not part of the facade and not part of the index.
- It is constructed with a `SubengineCore` and a lazily activated index.
- **It never blocks a kernel.**
  - No Manager call waits for a running kernel.
  - No kernel waits for the Manager.
  - Calls on the lookup path (§4.3 and §4.4) take only short state locks and never do IO, lane work or index calls.
- It spawns no threads. The caller drives `sync`, `upload` and `apply_updates`.
- It does not plan, choose queues, load models or kernels, search, or score. It creates no segments and no memory.
- It never resolves a `RegionHandle` to a device address. Copying bytes into a slot is done only by the injected `SlotTransfer`.
- It never activates the sub-engine. If the sub-engine is not active, lane work fails with `Subengine(NotActive)`.
- It does not modify the index. It only reads from it.
- No `unsafe` and no globals. Crates: `log`, plus the project modules in §5.

## 2. Context

- **Subengine core** (`subengine_core::SubengineCore`):
  - It owns the `IndexSegment`s (`RegionHandle` + `Layout`).
  - It provides `with_lane(LaneKind, FnOnce(&mut Lane))`.
  - It is activated by the Planner.
  - All device work of the Manager goes through `with_lane`: uploads on `CopyH2D`, updates on `Update`.
- **Index** (`index_contract::Index`):
  - It holds the authoritative entries, each with a `Stamp`.
  - `read` may block on IO.
  - `stamp` and `stamps` touch no vector data.
  - `subscribe` gives a level signal.
- **Speicherverwaltung** (memory):
  - It allocated the segment regions that the core owns.
  - The Manager does not call it. It only uses the segments the core reports.
- **Kernels and workers** (outside this module):
  - They read slots on the device.
  - They get addresses from `pin`/`Lease`.
  - They translate result slots back to `EntryId` with `entry_at`.
- The slot geometry is fixed by the config:
  - Each segment has `slots_per_segment` slots of `slot_bytes` bytes each.
  - The byte offset of a slot inside its segment is `slot * slot_bytes`.

## 3. Interface

```rust
use std::sync::Arc;
use index_contract::{EntryId, Index, Stamp};
use resource_requirements::IndexId;
use subengine_core::{IndexSegment, SubengineCore, SubengineError};
use device_lane_event::Lane;

/// Lazily activated index. `get` activates on its first successful call and then returns the same Arc.
/// A failure is not cached. Called only by upload/sync/apply_updates, never by lookups.
pub trait LazyIndex: Send + Sync {
    fn get(&self) -> Result<Arc<dyn Index>, String>;
}

/// Copies one entry vector into a slot. Implemented by the sub-engine's backend.
/// Called inside `SubengineCore::with_lane` (CopyH2D or Update lane) and never under a Manager lock.
/// `vector.len() == slot_bytes`.
pub trait SlotTransfer: Send + Sync {
    fn write_slot(&self, lane: &mut Lane, segment: &IndexSegment, slot: u32, byte_offset: u64, vector: &[u8])
        -> Result<(), String>;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SlotAddr { pub segment: u32, pub slot: u32 }   // segment = position in the segment list

#[derive(Clone, Debug)]
pub struct ManagerConfig { pub slots_per_segment: u32, pub slot_bytes: usize }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Residency { Absent, Loading, Resident, Updating }

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ManagerError {
    /// "ungültige Manager-Konfiguration: {0}"
    InvalidConfig(String),
    /// "Index nicht verfügbar: {0}"
    Index(String),
    /// "Vektorlänge {len} passt nicht zu slot_bytes {expected}"
    InvalidVector { len: usize, expected: usize },
    /// "kein freier Slot (alle gepinnt, in Arbeit oder Kapazität 0)"
    NoFreeSlot,
    /// "Eintrag wird bereits geladen"
    InFlight,
    /// "Eintrag nicht im Index"
    NotInIndex,
    /// "Transfer fehlgeschlagen: {0}"
    Transfer(String),
    /// Display of the inner error
    Subengine(SubengineError),
}
// Display + std::error::Error. Send + Sync + 'static.

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct UploadReport {
    pub uploaded: Vec<EntryId>,
    pub already_resident: Vec<EntryId>,
    pub evicted: Vec<EntryId>,
    pub failed: Vec<(EntryId, ManagerError)>,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct UpdateReport { pub applied: usize, pub deleted: usize, pub deferred: usize, pub failed: usize }

/// Pins slots. Pinned slots are neither evicted, overwritten nor freed. Drop unpins.
pub struct Lease { /* Arc to manager state, pinned entries */ }
impl Lease {
    pub fn resident(&self) -> &[(EntryId, SlotAddr)];   // in request order, duplicates removed
    pub fn absent(&self) -> &[EntryId];                 // not Resident at pin time
}
// Lease: Send + Sync.

pub struct Manager { /* private */ }                    // Send + Sync

impl Manager {
    /// Validates only. Calls `core.index_segments(&index_id)` once and fixes the segment list.
    /// Does not activate the index, uploads nothing, spawns nothing.
    pub fn new(core: Arc<SubengineCore>, index_id: IndexId, index: Arc<dyn LazyIndex>,
               transfer: Arc<dyn SlotTransfer>, cfg: ManagerConfig) -> Result<Self, ManagerError>;

    pub fn capacity(&self) -> usize;                    // segments * slots_per_segment
    pub fn resident_count(&self) -> usize;              // Resident + Updating
    pub fn pending_updates(&self) -> usize;
    pub fn residency(&self, id: EntryId) -> Residency;

    // Lookups: short lock only, no IO, no lane, no index call, never activate the index.
    pub fn slot_of(&self, id: EntryId) -> Option<SlotAddr>;                 // Some only if Resident or Updating
    pub fn entry_at(&self, addr: SlotAddr) -> Option<EntryId>;              // reverse mapping; Some only if Resident or Updating
    pub fn entry_at_offset(&self, segment: u32, byte_offset: u64) -> Option<EntryId>;
    pub fn entries_at(&self, addrs: &[SlotAddr]) -> Vec<Option<EntryId>>;
    pub fn pin(&self, ids: &[EntryId]) -> Lease;

    // Residency and updates: caller-driven, they may block on index IO.
    pub fn upload(&self, ids: &[EntryId]) -> UploadReport;
    pub fn evict(&self, ids: &[EntryId]) -> Vec<EntryId>;   // returns the evicted ones
    pub fn sync(&self) -> Result<usize, ManagerError>;       // number of newly enqueued updates
    pub fn apply_updates(&self, max: usize) -> UpdateReport;
}
```

## 4. Behavior and invariants

### 4.1 Construction
- Errors are `InvalidConfig`.
  - `slots_per_segment == 0` is an error.
  - `slot_bytes == 0` is an error.
  - `core.offers()` without `ResourceKind::Index` is an error.
- Zero segments is allowed. Capacity is then 0.
- The segment list is fixed for the lifetime of the Manager.
- Segment positions are the order returned by `core.index_segments`.
- The slot count is not validated against `Layout`.
- The initial state is all slots free, no entry resident, an empty queue.

### 4.2 Residency states and slot invariants
- Every entry has exactly one `Residency`.
  - `Absent`: no slot.
  - `Loading`: slot reserved, upload under way.
  - `Resident`: slot holds the data of stamp `s` (the resident stamp).
  - `Updating`: slot is being rewritten in place.
- Each slot belongs to at most one entry, and each entry has at most one slot. `slot_of` and `entry_at` are exact inverses over Resident and Updating entries.
- `Loading` slots are invisible to `slot_of`, `entry_at` and `pin`. The entry counts as absent there.
- `Updating` slots stay visible to `slot_of` and `entry_at`. `pin` treats the entry as absent, so a new lease never covers a slot being rewritten.
- Entries are `Updating` only while an update is being written.

### 4.3 Lookup and pin
- `entry_at(addr)` returns `None` for a free or Loading slot and for an out-of-range segment or slot.
- `entry_at_offset(seg, off)`:
  - It returns the entry of the slot containing `off`: `slot = off / slot_bytes`, with bounds checks.
  - `off` need not be slot-aligned.
  - Out of range gives `None`.
- `entries_at` is element-wise `entry_at`.
- `pin(ids)` is non-blocking.
  - Resident entries are pinned (pin count + 1) and returned in `resident()`.
  - All others go to `absent()`.
  - Their LRU tick is updated.
- A pinned slot is never evicted, rewritten or freed until all its leases are dropped.
- Kernels must read a slot only while holding a lease on it.
- Pin counts are per entry and support many concurrent leases.

### 4.4 Upload
- `upload(ids)` processes `ids` in order, with duplicates removed. Per entry:
  1. If the entry is `Resident` or `Updating`, add it to `already_resident` and update its LRU tick.
  2. If the entry is `Loading`, add it to `failed` with `InFlight`. There is no waiting.
  3. Otherwise, reserve a slot under the lock, taking the first of:
     - the free slot with the lowest `(segment, slot)`;
     - else the `Resident` and unpinned entry with the oldest LRU tick (ties go to the lower `(segment, slot)`). It is evicted and appended to `evicted`.
  4. If there is no such slot, the entry fails with `NoFreeSlot`.
  5. Outside the lock:
     - Get the index via `LazyIndex::get`. An error gives `Index(e)`, and the slot is released.
     - Read `t = index.stamp(id)` first. `None` gives `NotInIndex`.
     - Then call `index.read(id)`.
     - A vector length other than `slot_bytes` gives `InvalidVector`.
     - Write on lane `CopyH2D` via `core.with_lane` and `SlotTransfer::write_slot`. Errors give `Subengine(e)` or `Transfer(s)`.
  6. On failure, release the slot (it is free again). The evicted victim stays evicted.
  7. On success, the entry becomes `Resident` with resident stamp `t`, and is added to `uploaded`.
- The stamp is read before the data. The resident stamp may therefore be older than the data in the slot, never newer. `sync` then re-queues at most one harmless extra update.
- Uploads never enqueue updates themselves.
- A failure of one entry does not abort the others.

### 4.5 Eviction
- `evict(ids)` frees the slots of entries that are `Resident` and unpinned. It returns exactly those entries.
- Pinned, Loading, Updating and Absent entries are skipped.
- An evicted entry becomes `Absent`. Any queued update for it is dropped.
- No device work is done and no data is erased.

### 4.6 Update queue
- Items are `Update(stamp)` or `Delete` per entry.
- The queue is FIFO by first enqueue and has at most one item per entry.
  - A newer item for the same entry replaces the older one and keeps the old position.
  - `Delete` replaces `Update`.
  - `Update` never replaces `Delete`.
- `sync()`:
  - It gets the index via `LazyIndex::get`. An error gives `Index`.
  - It reads `observer.current()` first, using a `StampObserver` from `index.subscribe()` that is created once and kept.
  - If this is not the first sync and the value is not greater than the last-seen value, it returns `Ok(0)` without calling `stamps()`.
  - Otherwise it takes `index.stamps()` and sets last-seen to the value read before the snapshot.
  - For every `Resident` entry:
    - If its entry is missing from the snapshot, enqueue `Delete`.
    - If the index stamp is greater than the resident stamp, enqueue `Update(stamp)`.
  - Non-resident entries are ignored. `sync` never uploads.
  - It returns the number of newly enqueued or replaced items.
- `apply_updates(max)` processes up to `max` items from the queue head, in order. For each:
  - The entry is no longer `Resident`: drop the item silently (it is not counted).
  - The entry is pinned: the item stays in the queue and `deferred += 1`. Processing continues with later items.
  - `Delete`: free the slot and make the entry `Absent`. `deleted += 1`.
  - `Update`:
    - Set `Updating`.
    - Outside the lock, read `t = index.stamp(id)`.
    - If `t` is `None`, treat the item as `Delete`.
    - Otherwise read the entry and write it on lane `Update`.
    - On success the entry becomes `Resident` with stamp `t`, and `applied += 1`.
    - On failure the entry becomes `Resident` again with its old stamp and `failed += 1`. The item is removed. The next `sync` re-detects the difference.
- Deferred items are not counted against `max`. Each queue item is visited at most once per call.
- `pending_updates()` is the current queue length.

### 4.7 Concurrency
- All methods take `&self` and are callable from any thread.
- State (mapping, slots, pins, LRU ticks, queue) sits behind short-lived locks.
- No Manager lock is held across `LazyIndex::get`, `Index` calls, `with_lane` or `SlotTransfer`.
- Concurrent `upload` calls never give one slot to two entries.
- Concurrent `upload` calls for the same entry cause at most one transfer. The other call reports `InFlight`.
- Lookups (`slot_of`, `entry_at*`, `pin`, `residency`, counters) never wait for upload, update or sync.
- A `Lease` can be dropped on any thread. Its drop is non-blocking and never panics.
- A poisoned lock is recovered.
- Equal call sequences with equal index contents give equal slot assignments (the slot choice is deterministic).
- No panic on valid use. Errors are `ManagerError`.

## 5. Relied-on elements

- **Subengine core:**
  - `SubengineCore: Send + Sync`.
  - `index_segments(&self, &IndexId) -> Vec<IndexSegment>`.
  - `offers() -> Vec<ResourceKind>`, from the `Subengine` trait.
  - `with_lane<T>(&self, LaneKind, impl FnOnce(&mut Lane) -> T) -> Result<T, SubengineError>`.
    - It gives exclusive per-lane access.
    - It returns `NotActive` if the engine is not active, and `UnknownLane` for an unconfigured kind.
    - A lane of one kind is never used by two closures at once.
  - `IndexSegment { index: IndexId, region: RegionHandle, layout: Layout }` (`Clone`, `Eq`).
  - `SubengineError` (`Clone`, `Eq`, Display, `Send + Sync`).
- **Index-Vertrag:**
  - `EntryId(u64)`, `Stamp(u64)`, both `Copy + Ord + Hash`. `Stamp(0)` is never assigned.
  - `trait Index: Send + Sync`:
    - `stamp(EntryId) -> Option<Stamp>` and `stamps() -> Vec<(EntryId, Stamp)>`, sorted by `EntryId`. Neither touches vector data.
    - `read(EntryId) -> Payload`, which may block. A missing entry gives an empty vector. The data is that of the stamp or of a later write.
    - `subscribe() -> StampObserver`.
  - `Payload { vector: SharedBytes, meta: SharedBytes }`, where `SharedBytes` derefs to `[u8]`.
  - `StampObserver::current() -> Stamp` is a level signal and is monotonic.
- **Device/Lane:** `Lane` and `LaneKind::{CopyH2D, Update}`.
- **Resource-Requirements:** `IndexId` (`Clone + Eq + Hash`) and `ResourceKind::Index`.
- **Region-Handle:** pointer-free and never resolved here.