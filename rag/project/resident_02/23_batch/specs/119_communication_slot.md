# Specification: Communication-Slot (build order 119)

Crate module `communication_slot`.

## 1. Task, restrictions and boundaries

**Task.** Complete a kernel batch in the gap between batches ("Abschluss"). One call does three things in a fixed order:
1. Read back the kernel's writes from the device and upsert them into the attached index.
2. Resolve the batch's hits.
3. Apply pending updates.

If the mapping has moved past the batch epoch, the call first reports "stale Mapping" without side effects. It then repeats the slot under the Structure-Lock with a fresh snapshot and reclaims deferred memory afterwards.

**Boundaries.**
- No kernel launch, no queue implementation, no Manifest or SlotAssignment ownership, no update-queue implementation, no allocator, no device code. All of these are reached via the types and traits in §3 and §5.
- Device readback goes only through the `DeviceReadback` trait. There is no `unsafe`.
- No invented policy: no deduplication, sorting or top-k of hits, and no retry beyond the single stale-mapping repeat.
- Dependencies are `index_contract`, `change_stamp` (indirect, via `reconcile`), `kernel_batch` (indirect, via `Manifest` and `SegmentDesc`), `hit_queues`, `reconcile`, `hit_resolution`, `device_transfer` (no API used), `segments` (no API used), `kernel_layout`, `manifest`, `slot_layout`, `address_map` and `log`. Their types are not redefined.

## 2. Context

- The kernel writes embeddings into device slots and reports hits as addresses. Mapping changes happen only in gaps between batches.
- Every result therefore sees the mapping state of its batch, and a reported address is current. The kernel protocol carries no epoch and no sequence number.
- The batch epoch is the `Manifest` the batch ran on. The caller holds its lease for the duration of the call.
- Writes arrive as `EmbeddingWrite { entry, segment, slot }` via `hit_queues`. A hit is queued only after its write, and `QueueConsumer::drain` returns writes before hits.
- The manager owns the mapping. It provides `SegmentDirectory`, `MappingSource`, the update queue, and the shared locks and residency directory.
- Lock order for every holder is Structure-Lock → updates lock → residency lock.

## 3. Interface

```rust
use std::sync::{Arc, Mutex};
use index_contract::{Index, EntryId, Stamp};
use hit_queues::{QueueConsumer, EmbeddingWrite};
use hit_resolution::{ResolvedHit, SegmentDirectory};
use kernel_layout::Addr;
use manifest::Manifest;
use reconcile::ResidencyDirectory;

/// Replaceable device readback. Reads dst.len() bytes starting at the device address.
pub trait DeviceReadback: Send + Sync {
    fn read_back(&self, addr: Addr, dst: &mut [u8]) -> Result<(), ReadbackError>;
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReadbackError(pub String);

/// Mapping state: current manifest plus the one before it (may be None).
#[derive(Clone)]
pub struct MappingSnapshot { pub current: Arc<Manifest>, pub previous: Option<Arc<Manifest>> }

/// Marker. Holding the guard blocks mapping/structure changes; drop releases it.
pub trait StructureGuard {}

/// Implemented by the manager.
pub trait MappingSource: Send + Sync {
    fn snapshot(&self) -> MappingSnapshot;
    fn lock_structure(&self) -> Box<dyn StructureGuard + '_>;
    /// Reclaims deferred-freed memory whose work is complete. Returns the count of regions reclaimed.
    fn reclaim_deferred(&self) -> usize;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct UpdateOutcome { pub applied: usize }

/// Implemented by the manager. Always accessed under its Mutex.
pub trait UpdateQueue: Send {
    fn depth(&self) -> usize;
    /// True if a not-yet-applied update exists for this entry.
    fn is_pending(&self, entry: EntryId) -> bool;
    /// Applies all pending updates. This changes the mapping. The residency lock is held by the caller.
    fn apply(&mut self, residency: &ResidencyDirectory) -> UpdateOutcome;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SlotError {
    /// No index attached. No side effects.
    NoIndexAttached,
    /// Mapping has moved past the batch epoch. Returned only by `try_complete`. No side effects.
    StaleMapping,
}
impl std::fmt::Display for SlotError {}      // "kein Index angehängt" / "stale Mapping"
impl std::error::Error for SlotError {}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SlotReport {
    pub writes_applied: usize,        // read back, upserted
    pub writes_skipped: usize,        // slot without entry, or zero-length
    pub writes_failed: usize,         // index upsert returned Err (write consumed)
    pub writes_deferred: usize,       // remainder held for the next slot
    pub progress_bytes: u64,          // sum over processed writes of max(1, bytes)
    pub defer_reason: Option<DeferReason>,
    pub hits_resolved: usize,         // length of output hits (includes stale-flagged)
    pub hits_discarded: usize,
    pub hits_stale: usize,            // output hits with stale == true
    pub write_queue_depth: usize,     // before drain
    pub hit_queue_depth: usize,       // before drain
    pub update_queue_depth: usize,    // before apply
    pub updates_applied: usize,
    pub retried_after_stale_mapping: bool,
    pub reclaimed: usize,             // 0 unless retried
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeferReason { UnmappedAddress(EmbeddingWrite), Readback(ReadbackError) }

#[derive(Debug, Clone, PartialEq)]
pub struct SlotOutput { pub hits: Vec<ResolvedHit>, pub report: SlotReport }

/// Single-owner state (deferred writes, queue consumer). Send, methods take &mut self.
pub struct CommunicationSlot { /* consumer, deferred: Vec<EmbeddingWrite>, index: Option<Arc<dyn Index>>, ... */ }

impl CommunicationSlot {
    pub fn new(
        consumer: QueueConsumer,
        readback: Arc<dyn DeviceReadback>,
        dir: Arc<dyn SegmentDirectory>,
        mapping: Arc<dyn MappingSource>,
        updates: Arc<Mutex<dyn UpdateQueue>>,
        residency: Arc<Mutex<ResidencyDirectory>>,
    ) -> CommunicationSlot;

    pub fn attach_index(&mut self, index: Arc<dyn Index>);
    pub fn detach_index(&mut self);
    /// Writes held over for the next slot.
    pub fn deferred_len(&self) -> usize;

    /// Full completion: attempt against the batch epoch; on stale mapping repeat under the Structure-Lock.
    pub fn complete(&mut self, batch: &Manifest) -> Result<SlotOutput, SlotError>;
    /// Single attempt against the batch epoch. Err(StaleMapping) without any side effect.
    pub fn try_complete(&mut self, batch: &Manifest) -> Result<SlotOutput, SlotError>;
}
```

`CommunicationSlot` is `Send`. `SlotOutput`, `SlotReport` and `SlotError` are `Send + Sync`.

## 4. Behavior and invariants

### Entry and lock order

`try_complete` runs in this order:
1. No attached index → `Err(NoIndexAttached)`.
2. Lock `updates`, then `residency`. Both stay held until the end of the pass. Poisoned locks are taken over (`into_inner`), and there are no panics.
3. `snap = mapping.snapshot()`. If `snap.current.generation() != batch.generation()` → release the locks and return `Err(StaleMapping)`. At that point nothing has been drained, read back, written to the index, or applied.
4. Drain the queue, then run the pass (below).

### Pass

The pass runs against `snap`. In `try_complete`, `snap.current` is the batch epoch.

**Drain.** Record the queue depths. Call `consumer.drain()`. The work list is `deferred` (older) followed by `Drained.writes` (in order). `Drained.hits` are resolved in step 2.

**Step 1: kernel writes, in order, slot by slot.** For each write `w`:
1. Find the segment in `snap.current.segments()` with `id == manifest::SegmentId(u64::from(w.segment.0))`, and get `dir.binding(id)`.
2. If either is missing → the address is unmapped.
3. Otherwise compute `lookup(w.slot)` on the binding's `SlotAssignment`.
   - `None` (unbound, invalid, or out of range) → skipped. Count it in `writes_skipped` and add 1 to `progress_bytes`.
   - `Some(EntryRef)` → the index entry is `EntryRef.entry`. The assignment is authoritative, and `w.entry` is used only for logging.
4. For a bound slot, get `addr = device_addr(slot)` and `len = byte_range(slot).end - start`. An `Err` from either, or a `len` that does not fit `usize`, → the address is unmapped.
5. `len == 0` → skipped, as in 3.
6. Otherwise:
   - Call `readback.read_back(addr, &mut buf[..len])`.
   - Upsert with `index.write(entry, PayloadRef{ vector: &buf, meta: <meta of index.read(entry), empty if absent> })`.
   - On `Ok(stamp)`, call `residency.set_known_stamp(entry, stamp)`. The returned stamp is the one from the write, never one from a later `stamp()` or `read()`. A `false` result (not resident) is ignored.
   - Count it in `writes_applied` and add `max(1, len)` to `progress_bytes`.
   - On `Err(IndexError)`, log `warn!`, count it in `writes_failed`, and consume the write.

- Unmapped address (2, 4): this write and all following ones become the new `deferred`, order kept. `defer_reason = UnmappedAddress(w)`. The step stops. Steps 2 and 3 still run.
- Readback `Err`: same as unmapped, with the failing write first in `deferred` and `defer_reason = Readback(e)`.
- Without deferral, `deferred` becomes empty.
- The step is idempotent per write. The same slot is read again if it is deferred and re-processed.

**Step 2: resolve hits.**
- Call `hit_resolution::resolve_hits(batch, &snap.current, snap.previous.as_deref(), &*dir, &hits)`.
- For each returned `ResolvedHit` in order, apply two checks:
  - If `index.stamp(entry.entry) == None`, discard it and add 1 to `hits_discarded`.
  - Otherwise, if `updates.is_pending(entry.entry)`, set `stale = true`. This is a diagnostic and not an error. Otherwise keep the hit unchanged, with the `stale` flag from `resolve_hits`.
- `hits_discarded` includes `Resolution.discarded` from `resolve_hits`.
- Output order equals the input order. No sorting and no deduplication.
- In `try_complete` the generations are equal, so `resolve_hits` yields `stale = false`.

**Step 3: apply updates, last.**
- Record `updates.depth()`.
- Call `updates.apply(&residency)` and store `applied` as `updates_applied`.
- Step 3 changes the mapping. Therefore steps 1 and 2 never see its effect within the same pass.

**Logging.** Use the `log` target `"communication_slot"`.
- At `debug!`/`info!` level, log per pass:
  - applied writes
  - resolved, discarded and stale hits
  - the three queue depths
  - deferred count
  - applied updates (the changes)
- Log the stale mapping with `warn!`.
- Logging never alters results.

### `complete` (stale-mapping repeat)

1. Call `try_complete`. If the result is anything other than `Err(StaleMapping)`, return it.
2. Otherwise:
   - Log `warn!("stale Mapping")`. No side effect has occurred.
   - `g = mapping.lock_structure()`.
   - Lock `updates`, then `residency`.
   - Take a fresh `snap = mapping.snapshot()`.
   - Run the same pass without the epoch check. Writes use `snap.current`. Hits go through `resolve_hits(batch, &snap.current, snap.previous.as_deref(), …)`, so `stale` is set by `transfer` where `batch.generation() < current.generation()`, in addition to the pending-update rule.
   - Release the locks, then `g`. Call `mapping.reclaim_deferred()` and store the result in `reclaimed`.
   - Set `retried_after_stale_mapping = true`.
3. The Structure-Lock is never held during the first attempt. The acquisition order is Structure-Lock → updates → residency.
4. No attempt after the repeat. `complete` cannot return `StaleMapping`.

### Concurrency and invariants

- One completion at a time (`&mut self`). Different `CommunicationSlot`s sharing the same `updates` and `residency` serialize on those locks.
- While the updates lock is held, the mapping cannot change, because mapping changes happen only in `UpdateQueue::apply`. The epoch check in step 3 of the entry order therefore covers the whole pass.
- Every drained write is either processed (applied, skipped or failed) or deferred. None is lost or duplicated. `writes_applied + writes_skipped + writes_failed + writes_deferred` equals the work-list length.
- No hit is lost after a drain. Steps 2 and 3 run even when step 1 deferred.
- Hits whose entry vanished from the index are discarded, never delivered.
- `deferred_len()` equals the last `writes_deferred`.
- Zero capacity, empty queues and an empty work list are valid. They give an empty result, and step 3 still runs.

## 5. Relied-on elements

- **`hit_queues`**
  - `EmbeddingWrite { entry: EntryId, segment: address_map::SegmentId(pub u32), slot: Slot }`: `Debug, Clone, Copy, PartialEq, Eq`.
  - `QueueConsumer::drain(&mut self) -> Drained { writes: Vec<EmbeddingWrite>, hits: Vec<KernelHit> }`: writes first, hit count snapshotted before.
  - `write_depth()` and `hit_depth()` return `usize`. `QueueConsumer` is `Send`.
- **`hit_resolution`**
  - `SegmentDirectory: Send + Sync { fn binding(&self, manifest::SegmentId) -> Option<SegmentBinding> }`.
  - `SegmentBinding { assignment: Arc<SlotAssignment>, states: Arc<SlotStates> }`.
  - `resolve_hits(hit_manifest: &Manifest, current: &Manifest, previous: Option<&Manifest>, dir: &dyn SegmentDirectory, hits: &[KernelHit]) -> Resolution { hits: Vec<ResolvedHit>, discarded: usize }`.
  - `ResolvedHit { entry: EntryRef, score: f32, stale: bool }`. `stale = hit_generation < current.generation()`.
- **`slot_layout`**
  - `SlotAssignment`: `lookup(Slot) -> Option<EntryRef>` (None for unbound and for invalid slots).
  - `device_addr(Slot) -> Result<Addr, SlotError>` and `byte_range(Slot) -> Result<Range<u64>, SlotError>`.
  - `EntryRef { entry: EntryId, section: u32 }`.
- **`manifest`**
  - `Manifest::generation() -> ManifestGeneration` (u64, `Eq`) and `segments() -> &[SegmentSpec]`.
  - `SegmentSpec::id() -> manifest::SegmentId(pub u64)`.
- **`kernel_layout`**: `Addr(pub u64)`, `Slot(pub u32)`.
- **`index_contract`**
  - `Index: Send + Sync`.
  - `write(&self, EntryId, PayloadRef<'_>) -> Result<Stamp, IndexError>`. The returned stamp belongs to exactly this write. `write` replaces vector and meta fully.
  - `PayloadRef { vector: &[u8], meta: &[u8] }`.
  - `read(EntryId) -> Payload { vector, meta: SharedBytes }`, empty if absent. `stamp(EntryId) -> Option<Stamp>`.
  - Invalid vector length yields `IndexError::InvalidVectorLength`.
- **`reconcile`**
  - `ResidencyDirectory`: `Send + Sync`, `&self` methods.
  - `set_known_stamp(&self, EntryId, Stamp) -> bool` (false if not resident; the slot is unchanged).
  - This module never calls `insert` or `remove` on the directory.