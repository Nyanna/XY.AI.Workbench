# Spec: Doorbell-Synchronisation (build order 84)

## 1. Task, restrictions and boundaries
Implement in Rust the host side of the doorbell protocol between the device and the host:
- Read the number of completed batches that the device publishes, using Acquire semantics.
- Detect a gap between `completed` and `acked`.
- Close the gap and publish the ack with Release semantics, without waiting for the kernel.

**Out of scope:**
- Device-side code and kernels.
- Allocating the cells. The caller provides them, for example in MappedHost memory.
- Submitting batches.
- Creating events.
- Any retry or fallback.

**Dependencies:**
- Use only `std` plus `device::{Event, EventError}`.
- Do not call the CUDA API.
- Do not edit `Cargo.toml`.

## 2. Context
- The device increments the monotonic counter `completed` (u64, starting at 0) after each batch. Increments may coalesce, so the host may see several batches at once.
- The host publishes `acked` (u64, starting at 0, ≤ `completed`) to tell the device that it has consumed those batches.
- Both cells may be fixed, dedicated cells in mapped host memory (the `MappedHost` arena of `GpuBackend`, which has no raw atomic accessor). The caller therefore supplies them as raw pointers via `DoorbellCell::from_raw`.
- The host mutates shared data (results, assignments) only between the completion of a batch and its ack. It never waits for the kernel, only for a batch `Event`.
- The target is Turing (CC 7.5). Acquire loads are available from CC 7.0.
- Neighbours:
  - `device` supplies `Event`.
  - `gpu_backend` supplies the mapped memory.
  - The scheduler or kernel module calls this module.

## 3. Interface
```rust
use std::sync::atomic::AtomicU64;
use std::ptr::NonNull;
use std::sync::Arc;
use device::{Event, EventError};

/// One 8-byte, 8-aligned u64 cell shared with the device. Send + Sync.
pub struct DoorbellCell { /* NonNull<AtomicU64> + keepalive Arc */ }
impl DoorbellCell {
    /// Host-only cell (heap), initial value 0. For tests and CPU use.
    pub fn new_local() -> DoorbellCell;
    /// # Safety: `ptr` is 8-aligned, valid for reads and writes of 8 bytes and accessed
    /// only atomically, and stays valid as long as `keepalive` (held by the cell) lives.
    /// The cell content is the caller's responsibility (initially 0).
    pub unsafe fn from_raw(ptr: NonNull<AtomicU64>,
                           keepalive: Arc<dyn std::any::Any + Send + Sync>) -> DoorbellCell;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DoorbellError {
    /// Device error from the batch event, passed on unchanged.
    Event(EventError),
    /// completed < acked (protocol violation).
    Inconsistent { completed: u64, acked: u64 },
}
impl std::fmt::Display for DoorbellError {}
impl std::error::Error for DoorbellError {}

/// Host side of the doorbell. Send + Sync. Poll methods take `&mut self`.
pub struct Doorbell { /* completed: DoorbellCell, acked: DoorbellCell */ }
impl Doorbell {
    pub fn new(completed: DoorbellCell, acked: DoorbellCell) -> Doorbell;
    /// Non-blocking. Ok(None) if completed == acked.
    pub fn poll(&mut self) -> Result<Option<Gap<'_>>, DoorbellError>;
    /// Waits first for `event` (Event::wait), then behaves like `poll`.
    pub fn wait(&mut self, event: &Event) -> Result<Option<Gap<'_>>, DoorbellError>;
    /// Current acked value (Acquire load). For diagnostics.
    pub fn acked(&self) -> u64;
}

/// Open gap (acked, completed]. Exclusively borrows the Doorbell.
#[must_use = "an open gap must be closed"]
pub struct Gap<'a> { /* &'a mut Doorbell, first, last, closed */ }
impl Gap<'_> {
    pub fn first(&self) -> u64;  // acked + 1, lowest unacked batch number
    pub fn last(&self) -> u64;   // completed as read, highest batch number
    pub fn len(&self) -> u64;    // last - first + 1, always >= 1
    /// Publishes ack = last with Release; the gap is closed.
    pub fn close(self);
}
impl Drop for Gap<'_> {}  // panics if not closed (unless the thread is already panicking)
```
`DoorbellCell`, `Doorbell` and `DoorbellError` are `Send + Sync + 'static`. Assert this at compile time.

## 4. Behavior and invariants
**Reading**
- `poll` loads `completed` exactly once with `Ordering::Acquire`. It then loads `acked` (the host is its only writer).
- If `completed == acked`, `poll` returns `Ok(None)`.
- If `completed > acked`, `poll` returns `Ok(Some(gap))` with `first = acked + 1` and `last = completed`.
- If `completed < acked`, `poll` returns `Err(Inconsistent{completed, acked})`. No gap is opened and no cell is changed.
- Coalesced completions (completed − acked > 1) give one gap with `len() > 1` and `last()` equal to the highest batch number.
- Every write by the device that precedes its increment of `completed` is visible to the host after a gap with that `last` has been observed. This holds through the Acquire load.
- `poll` never blocks and never calls the event.

**Waiting**
- `wait` calls `event.wait()` first.
- On `Err(e)`, `wait` returns `Err(DoorbellError::Event(e))`. It does not read the cells and changes nothing.
- On `Ok`, it behaves like `poll`. `Ok(None)` is possible.
- `wait` blocks only on the event, never on the kernel or on the cell values.

**Closing**
- `Gap::close` stores `acked = last` once, with `Ordering::Release`. It does not wait and does not read `completed` again.
- Host writes of results and assignments that happen before `close` are visible to the device once the device reads the new `acked` with Acquire.
- If `completed` has advanced further in the meantime, the next `poll` opens a new gap. A larger `last` is never acked early.
- After `close`, `acked` equals `last`. `acked` never decreases and is only ever written by `close`.
- A `Gap` that is dropped without `close` is a programming error and panics. There is no implicit ack. During unwinding, `Drop` does not panic and does not ack.
- Because `Gap` exclusively borrows the `Doorbell`, at most one gap is open at a time. The compiler enforces this.

**Cells**
- `new_local` cells start at 0. Tests simulate the device by incrementing the `completed` cell. The tests need an accessor (for example a `pub(crate)` or test-only `store`/`load` on `DoorbellCell`, or access to the underlying `AtomicU64`). The accessor is an implementation detail.
- All cell accesses are atomic on `AtomicU64`. Plain `memcpy` access to the cells is forbidden.
- The cell keeps `keepalive` until it is dropped.

**Concurrency**
- The device (or a test thread) may increment `completed` at any time. The host never writes it.
- The poll methods need `&mut self`. A `Doorbell` can be moved between threads.
- No locks are used and nothing panics on poisoning.

**Tests that run without a GPU**
- poll with no gap.
- A single gap.
- A coalesced gap with `last` = highest number.
- Close followed by a new poll.
- `Inconsistent`.
- `wait` with a pending `Event::mock` that is completed from another thread.
- `wait` with `fail("x")` returning `Event(EventError{device: Cpu, ..})` without reading the cells.
- Drop of an open gap panics.
- A Release/Acquire message-passing test across two threads.

## 5. Relied-on elements
- `device::Event`: `Clone + Send + Sync`.
  - `wait(&self) -> Result<(), EventError>` is blocking and returns immediately if the event is already complete.
  - `Event::mock() -> (Event, MockEventControl)` offers `complete()`, `fail(cause)` and `reset()`.
  - `Event::cpu() -> (Event, EventSignal)` offers `signal()`.
- `device::EventError { pub device: DeviceHandle, pub cause: String }`: `Clone + Debug + PartialEq + Eq + Display + Error`.
- Acquire loads are available from compute capability 7.0 (`target_env::MIN_ACQUIRE_CC`). The reference target is CC 7.5, so no fallback is needed.
- `gpu_backend::GpuBackend` offers MappedHost memory (pinned, zeroed, device-mapped). The cells may live there. This module does not call it.