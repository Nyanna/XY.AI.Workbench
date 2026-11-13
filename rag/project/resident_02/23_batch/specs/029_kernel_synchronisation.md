# Kernel-Synchronisation: Specification (build order 29)

Note: the specifications of `Device`, `Lane` and `Event` were still pending when this was written. This module therefore names them only as opaque imported types. It does not redefine or assume any of their members. Section 5 lists what the implementer must check once those specifications exist.

## 1. Task, restrictions and boundaries

**Task:** Apply changes (reconfiguration, state mutation) to the running system only inside gaps, and never block a kernel. A gap is either a batch end or a communication slot.

**Restrictions:**
- A kernel is never blocked, never waits on this module, and is never locked out of its resources.
- Changes are applied only while a gap is open on the affected lane. Outside a gap nothing is mutated.
- This module does not execute kernels, schedule batches, or define lanes, devices or events. It uses those types as imported.
- It invents no change types of its own. A change is an opaque closure supplied by the caller.
- Kernel-side calls (`gap_begin`, `gap_end`) are wait-free in the sense that they take no lock and perform no blocking I/O. Their cost is bounded by the budget described below.

## 2. Context

- **Device:** the execution target that owns lanes.
- **Lane:** an ordered execution stream on a Device. Kernels run on a lane. Gaps occur between kernels on a lane, at a batch end or in a communication slot.
- **Event:** the completion and ordering primitive. It is used to know that a lane has reached a gap, and to publish that a change has been applied.
- **This module (the gate):**
  - Control code submits changes to it.
  - The lane driver (the code that runs the kernels) tells it when a gap opens and closes.
  - The gate applies pending changes for that lane during the gap and signals completion through an Event.
- A kernel never calls into the pending queue. The driver only calls `gap_begin` and `gap_end` at boundaries it already owns.

## 3. Interface

The types `Device`, `Lane` and `Event` are imported from their modules. `LaneId` below must be whatever identifier `Lane` exposes. If `Lane` has no separate id type, use `Lane`'s own id type or a reference to it. Do not invent one.

```rust
use std::time::Duration;

/// Kind of gap that is open.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GapKind { BatchEnd, CommSlot }

/// A change to apply. Runs only inside a gap, on the thread that calls `gap_begin`.
pub type Change = Box<dyn FnOnce(&GapContext) -> Result<(), ChangeError> + Send + 'static>;

pub struct GapContext<'a> {
    pub lane: &'a Lane,
    pub device: &'a Device,
    pub kind: GapKind,
    // Remaining budget, see `remaining()`.
}
impl GapContext<'_> { pub fn remaining(&self) -> Duration; }

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChangeError { Failed(String), Cancelled }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SyncError { Closed, QueueFull }

/// Handle for a submitted change.
pub struct ChangeHandle { /* ... */ }
impl ChangeHandle {
    /// Non-blocking.
    pub fn status(&self) -> ChangeStatus;
    /// Event that is signalled when the change reaches a final status (Applied, Failed or Cancelled).
    pub fn event(&self) -> &Event;
    /// Withdraws the change if it has not started. Returns true if it was withdrawn.
    pub fn cancel(&self) -> bool;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChangeStatus { Pending, Applied, Failed(ChangeError), Cancelled }

pub struct KernelSync { /* ... */ }
impl KernelSync {
    /// `capacity` is the maximum number of pending changes per lane.
    pub fn new(capacity: usize) -> Self;

    /// Control side. Never blocks, any thread. Queues the change for the next gap of `lane`.
    pub fn submit(&self, lane: &Lane, change: Change) -> Result<ChangeHandle, SyncError>;

    /// Driver side. Called when a gap opens on `lane`.
    /// Applies pending changes for that lane in FIFO order, until the queue is empty or `budget` is used up.
    /// Returns the number of changes applied (or failed).
    pub fn gap_begin(&self, lane: &Lane, kind: GapKind, budget: Duration) -> usize;

    /// Driver side. Called when the gap ends. After it returns, no change runs for this gap.
    pub fn gap_end(&self, lane: &Lane);

    /// Number of pending changes for the lane. Non-blocking.
    pub fn pending(&self, lane: &Lane) -> usize;

    /// Rejects further submissions. Cancels all pending changes (status `Cancelled`, event signalled).
    pub fn close(&self);
}

unsafe impl Send for KernelSync {}
unsafe impl Sync for KernelSync {}
```

`KernelSync`, `ChangeHandle` and `Change` must be `Send`. `KernelSync` must be `Sync`. Prefer deriving these automatically. Use `unsafe impl` only if it is unavoidable, and then justify it in a comment.

## 4. Behavior and invariants

**Non-blocking**
- `submit`, `pending`, `status`, `cancel` and `close` never wait for a kernel, a gap or a change to finish. They use no lock that `gap_begin` or `gap_end` holds across a change's execution.
- A kernel is never delayed by a submission. A submission only touches the pending queue.
- `gap_begin` and `gap_end` never wait for control-side threads. A control thread that is submitting at the same time does not delay them.
- A full queue returns `Err(SyncError::QueueFull)` immediately. The module never waits for room.
- After `close`, `submit` returns `Err(SyncError::Closed)`.

**Gap-only mutation**
- A `Change` runs only between `gap_begin(lane, …)` and `gap_end(lane)` for the lane it was submitted to, and only on the calling thread of `gap_begin`.
- A change never runs concurrently with a kernel on that lane.
- If `gap_begin` is not called, no change runs. A submitted change simply stays `Pending`.
- Changes for one lane run in submission order (FIFO). Changes for different lanes are independent.

**Budget**
- `gap_begin` starts no new change once the elapsed time is at or above `budget`. The remaining changes stay `Pending` for the next gap, in the same order.
- A change that has already started is never interrupted. It can read `GapContext::remaining()` and must stay within it. The module does not enforce this.
- `budget == Duration::ZERO` applies nothing and returns 0.

**Results**
- Success sets `Applied`. A change returning `Err(e)` sets `Failed(e)`. A failure does not stop the following changes in the same gap.
- A panic inside a change is caught. It sets `Failed(ChangeError::Failed(..))`, and `gap_begin` continues.
- Each handle's `Event` is signalled exactly once, when the status becomes final. It is signalled after the status is visible.
- `cancel` succeeds only for `Pending` changes that have not started. A change that has started or finished returns false. A cancelled change never runs, and its slot no longer counts for `capacity`.

**Gap state**
- `gap_begin` while a gap is already open on the same lane is a no-op that returns 0. `gap_end` without an open gap is a no-op.
- `gap_end` does not run changes. It only marks the gap closed. Changes submitted during an open gap may run in that same gap if budget remains. Otherwise they wait for the next gap.
- `close` does not touch a change that is running. That change finishes normally.

## 5. Relied-on elements

Nothing could be taken verbatim, because the specifications of `Device`, `Lane` and `Event` were still pending. The implementer must check these points against them and adapt only the names:
- **`Lane`:** how to use it as a map key (id, `Hash`/`Eq`), and whether it is `Sync` so it can be shared by reference.
- **`Event`:** how to create an unsignalled event and how to signal it once. `ChangeHandle::event` must return the type that `Event` defines.
- **`Device`:** the reference that `GapContext` carries, and how to get it from a lane if `Lane` provides that.
- **If an assumed capability is missing,** keep the behavior in Section 4 and choose the nearest equivalent. Do not add behavior to those modules.