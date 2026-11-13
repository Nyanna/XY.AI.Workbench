# Spec: GPU-Completion (module `gpu_completion`, build order 107)

## 1. Task, restrictions and boundaries

Implement in Rust the hand-over of finished GPU lanes in two ways:
- The stream-end callback only records a completion entry and raises the notification.
- The result-processing step additionally asks the lane's event (`Event::is_complete`) and takes a finished lane by atomic swap. This covers missed callbacks.

Restrictions:
- Use only `std`, `log`, and the project modules `device_lane_event` (`Lane`, `Event`, `EventError`) and `notification_topology` (`SubengineNotifier`).
- No `unsafe`.
- **The callback code (`CompletionCallback::fire`) must not call any GPU API.** That includes `Event::is_complete`, `Event::wait`, and every `Lane` method that touches a stream. It must not lock a mutex or allocate.
- Not in this module:
  - Registering the callback with the stream. The GPU binding does that, using `Stream::add_callback`-style APIs that this module does not define.
  - Submitting work, recording events, and the sub-engine loop.
  - Timers, scheduling and result processing itself.
  - Reusing lanes after they are taken. The caller decides that.

## 2. Context

- A sub-engine submits work on a `Lane`, calls `lane.record_event()`, and hands lane and event to this module with `submit`. It gets back a `CompletionCallback`. The GPU binding runs the callback on the driver thread when the stream reaches that point.
- The sub-engine loop is woken by `SubengineNotifier`. The callback calls `notify_result()` once. The loop then calls `poll` in its result step.
- A callback can be missed, delayed, or never run. `poll` therefore also queries each in-flight lane's event.
- Exactly one of the two paths hands over a given lane. This is decided by an atomic swap on a per-submission state.

## 3. Interface

```rust
use std::sync::Arc;
use device_lane_event::{Lane, Event, EventError};
use notification_topology::SubengineNotifier;

/// A lane that finished.
pub struct FinishedLane {
    pub lane: Lane,
    pub event: Event,
    /// Caller tag given to `submit`.
    pub tag: u64,
    /// Ok(()) = work completed. Err = the event query reported a failure.
    pub result: Result<(), EventError>,
    /// Which path handed the lane over.
    pub via: TakenVia,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TakenVia { Callback, EventQuery }

/// Callback to be run at the end of the stream. Send + 'static. Single use.
pub struct CompletionCallback { /* Arc<Slot>, SubengineNotifier */ }
impl CompletionCallback {
    /// Records the completion entry and raises `notify_result()`. No GPU API, no lock, no allocation.
    pub fn fire(self);
}

/// Tracker of in-flight lanes. Send + Sync.
pub struct GpuCompletion { /* Mutex<Vec<Arc<Slot>>>, SubengineNotifier */ }

impl GpuCompletion {
    pub fn new(notifier: SubengineNotifier) -> Self;
    /// Puts an in-flight lane under tracking. `event` is the event recorded on `lane` after the submitted work.
    pub fn submit(&self, lane: Lane, event: Event, tag: u64) -> CompletionCallback;
    /// Takes all lanes that are finished by callback entry or by event query. Non-blocking.
    pub fn poll(&self) -> Vec<FinishedLane>;
    /// Number of tracked lanes not yet taken.
    pub fn in_flight(&self) -> usize;
}
```

`GpuCompletion`, `CompletionCallback` and `FinishedLane` are `Send`. `GpuCompletion` is also `Sync`. No function returns an error type. Event errors appear in `FinishedLane::result`.

## 4. Behavior and invariants

**Slot state.** Each `submit` creates one slot with an atomic state `Pending → Done → Taken`.
- The callback moves `Pending → Done` by compare-exchange.
- A taker moves any state to `Taken` by `swap`.
- A taker owns the lane only if the previous state was not `Taken`.
- Therefore a lane is handed over exactly once, even when callback and `poll` race, or when `poll` runs on several threads.

**Callback**
- `fire` does a compare-exchange `Pending → Done` on its slot, then calls `notifier.notify_result()` once. It always calls `notify_result()`, even if the slot is already `Done` or `Taken`.
- If the slot is already `Taken`, `fire` changes nothing. A callback that arrives after `poll` took the lane is harmless.
- `fire` never blocks and never panics.
- A callback that is never fired, or fired late, does not stop `poll` from finding the lane through the event.
- Dropping a `CompletionCallback` without `fire` does nothing.

**Poll**
- `poll` walks the tracked slots. A slot is finished if its state is `Done`, or if its state is `Pending` and `event.is_complete()` returns `Ok(true)`.
- If `event.is_complete()` returns `Err(e)`, the slot is finished with `result = Err(e)`. The lane is handed over so it is not lost.
- For a finished slot, `poll` does `swap(Taken)`. Only if the previous state was not `Taken` does it remove the slot and return `FinishedLane`.
- `via` is `Callback` if the previous state was `Done`. It is `EventQuery` if it was `Pending`.
- For a `Done` slot, `result` is `Ok(())` and no event query is made.
- A slot with `Ok(false)` stays tracked and is not returned.
- `poll` never blocks. It does not call `Event::wait`.
- Several finished lanes are returned in submit order.
- `poll` notifies nobody.
- `poll` on an empty tracker returns an empty `Vec`.

**Concurrency**
- `submit`, `poll` and `in_flight` are callable concurrently through `&self`.
- `fire` runs on any thread (driver thread) without any lock.
- The tracker lock is held only for list changes and the scan. It is never held while the callback runs.
- Poisoned locks are recovered by taking the inner state.

**Invariants**
- Every submitted lane is returned by `poll` exactly once, as soon as its callback entry exists or its event is complete.
- `in_flight()` = submitted − returned.

## 5. Relied-on elements

From `device_lane_event`:
- `Lane` (`Send + Sync`) is moved into the tracker and returned unchanged.
- `Event` is `Clone + Send + Sync`.
- `Event::is_complete(&self) -> Result<bool, EventError>` is non-blocking.
- `EventError { device: DeviceHandle, cause: String }` is `Clone`.
- For a CPU or mock event, `is_complete` works without a GPU.

From `notification_topology`:
- `SubengineNotifier` is `Clone + Send + Sync`.
- `notify_result(&self)` calls `notify_one()` once. It never blocks and never fails.
- Signals sent while the loop is not waiting are kept as one permit and may coalesce, so one wake-up can cover several finished lanes. The loop must therefore drain with `poll`.