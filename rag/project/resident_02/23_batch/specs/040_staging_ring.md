# Spec: Staging-Ring (build order 40)

## 1. Task, restrictions and boundaries
Implement the Rust module `staging_ring`. It manages a fixed number of host staging slots for H2D copies. Each slot holds a payload buffer and has one of three states: free, occupied, or in flight (with an `Event`). A slot is reused only after its event has completed. The ring is thread-safe and testable without a GPU by using `Event::mock()`.

- Use only `std` and the types from the Device/Lane/Event module (`Event`, `EventError`) and the Arenas module (`Arena`, `ArenaKind`).
- Out of scope:
  - Submitting copies, recording events, and lanes.
  - Arena or budget accounting.
  - Real page-locking (pinning) calls. Pinning is done by the GPU binding. This module only provides the stable host buffers.
  - D2H staging.
  - Any slot-selection policy beyond the round-robin described below.

## 2. Context
- A copy module acquires a slot, fills the payload, submits the H2D copy on a CopyH2D lane, and records an `Event` with `Lane::record_event`. It then calls `begin_transfer(slot, event)`.
- When the event completes, the slot becomes reusable. Dependents also call `release` for slots that were never transferred.
- The Arenas module's `ArenaKind::Staging` arena is the host region this ring is sized from. `Arena` is a logical region only (kind + capacity).
- `Event` is `Clone + Send + Sync`, and all clones share one marker. Tests drive events with `Event::mock()` and `MockEventControl`.

## 3. Interface
```rust
use std::fmt;
use crate::{Event, EventError};          // Device/Lane/Event module
use crate::arenas::{Arena, ArenaKind};   // Arenas module

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SlotId(pub usize);            // index 0..slot_count

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SlotState { Free, Occupied, InFlight }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum TransferDirection { H2D, D2H }

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TransferError { pub direction: TransferDirection, pub cause: EventError }

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum StagingError {
    Exhausted,                 // Display: "Staging erschöpft"
    InvalidSlot,               // Display: "ungültiger Slot"
    Transfer(TransferError),   // waiting failed; direction is always H2D here
    InvalidConfig,             // slot_count == 0, or arena kind != Staging
}
impl fmt::Display for StagingError {}
impl std::error::Error for StagingError {}
impl fmt::Display for TransferError {}
impl std::error::Error for TransferError {}

pub struct StagingRing { /* Mutex<state>, stable payload buffers */ }
// StagingRing: Send + Sync (static-assert it)

impl StagingRing {
    /// slot_count slots of slot_size bytes each, zero-initialised, all Free.
    pub fn new(slot_count: usize, slot_size: usize) -> Result<StagingRing, StagingError>;
    /// arena.kind must be ArenaKind::Staging. slot_size = arena.capacity / slot_count (integer division, may be 0).
    pub fn from_arena(arena: &Arena, slot_count: usize) -> Result<StagingRing, StagingError>;

    pub fn slot_count(&self) -> usize;
    pub fn slot_size(&self) -> usize;

    /// Blocking acquire. Free -> Occupied.
    pub fn acquire(&self) -> Result<SlotId, StagingError>;

    /// Occupied only. Runs f with exclusive access to the payload (length slot_size).
    /// f must not call back into this ring (it may deadlock).
    pub fn with_payload<R>(&self, slot: SlotId, f: impl FnOnce(&mut [u8]) -> R) -> Result<R, StagingError>;
    /// Occupied only. Pointer to the slot's payload start for DMA. It is stable for the ring's lifetime,
    /// and it may be used only while the slot is Occupied or InFlight and owned by the caller.
    pub fn payload_ptr(&self, slot: SlotId) -> Result<*mut u8, StagingError>;

    /// Occupied -> InFlight(event).
    pub fn begin_transfer(&self, slot: SlotId, event: Event) -> Result<(), StagingError>;
    /// Occupied -> Free (no transfer was made).
    pub fn release(&self, slot: SlotId) -> Result<(), StagingError>;

    pub fn in_flight_count(&self) -> usize;
    pub fn in_flight_slots(&self) -> Vec<SlotId>;   // oldest begin_transfer first
    pub fn slot_state(&self, slot: SlotId) -> Option<SlotState>; // None if out of range
}
```

## 4. Behavior and invariants
**States and transitions**
- Every slot is always in exactly one state. The only transitions are:
  - Free → Occupied (`acquire`)
  - Occupied → InFlight (`begin_transfer`)
  - Occupied → Free (`release`)
  - InFlight → Free (event completed)
- `with_payload`, `payload_ptr`, `begin_transfer`, and `release` on a slot that is not Occupied return `Err(InvalidSlot)`. This includes Free and InFlight slots and an out-of-range `SlotId`. The state is left unchanged.
- InFlight → Free happens only after the slot's event has completed, either through `Event::is_complete() == Ok(true)` or a successful `Event::wait()`. A slot is never reused before that.
- The payload is not cleared on reuse. Initially it is zeroed.

**Round-robin acquire**
- The cursor `last` starts so that the first candidate is slot 0. The candidate is always `(last + 1) % slot_count`.
- `acquire` grants only the candidate slot, and never skips to another slot. A grant sets `last` to that slot.
- Algorithm (one loop iteration; the state lock is held except while waiting):
  1. Reclaim: set every InFlight slot with `is_complete() == Ok(true)` to Free. An `Err` from the poll is ignored and the slot stays InFlight.
  2. If the candidate is Free, mark it Occupied, update `last`, and return it.
  3. Otherwise, if no slot is InFlight, return `Err(Exhausted)`.
  4. Otherwise take a clone of the event of the oldest InFlight slot (smallest `begin_transfer` order). Release the lock and call `Event::wait()`.
     - On `Ok`: relock. If that slot is still InFlight with the same transfer, set it Free. Go to step 1.
     - On `Err(e)`: return `Err(Transfer(TransferError{ direction: H2D, cause: e }))`. The slot stays InFlight and is not reused.
- Concurrent acquirers each get a distinct slot. Two threads never receive the same slot.
- Waiting never holds the internal lock. Other threads can call `begin_transfer`, `release`, queries, and `acquire` while one thread waits.

**Queries**
- `in_flight_count`, `in_flight_slots`, and `slot_state` first run the reclaim step (non-blocking), so slots with completed events are no longer reported as InFlight.
- `in_flight_slots().len() == in_flight_count()` for a quiescent ring.

**Construction**
- `new(0, _)` returns `Err(InvalidConfig)`. `from_arena` returns `Err(InvalidConfig)` if `arena.kind != Staging` or `slot_count == 0`.
- `slot_size == 0` is allowed.

**Concurrency**
- All state changes are atomic under one internal mutex. A poisoned lock is recovered with `into_inner`, and nothing panics because of poisoning.
- The payload buffers are allocated once and never move or resize.

**Tests without a GPU (must work)**
- Use `Event::mock()` for `begin_transfer`, with `MockEventControl::complete` and `fail`.
- `acquire` on a ring whose candidate is InFlight with a pending mock blocks until another thread calls `complete()`.
- A failed mock gives `Err(Transfer{direction: H2D, ..})`.
- With all slots Occupied and none InFlight, `acquire` returns `Exhausted`.

## 5. Relied-on elements
From Device/Lane/Event:
- `Event: Clone + Send + Sync`. Clones share one marker.
- `Event::is_complete(&self) -> Result<bool, EventError>` never blocks.
- `Event::wait(&self) -> Result<(), EventError>` blocks until complete. It returns `Err(EventError{device, cause})` on failure.
- `Event::mock() -> (Event, MockEventControl)`.
- `MockEventControl::complete()`, `fail(cause)`, and `reset()`.
- `EventError { pub device: DeviceHandle, pub cause: String }`: `Clone + Debug + PartialEq + Eq + Display + Error`.
- `Lane::record_event(&self) -> Result<Event, LaneError>` is used by callers to obtain the event passed to `begin_transfer`.

From Arenas:
- `Arena { pub kind: ArenaKind, pub capacity: u64 }`.
- `ArenaKind::Staging` is the host arena. It has no budget check.