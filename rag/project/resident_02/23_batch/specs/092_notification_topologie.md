# Spec: Notification-Topologie (module `notification_topology`, build order 92)

## 1. Task, restrictions and boundaries

Build the notification wiring for the sub-engines in Rust:
- Create one notification handle per sub-engine. Every notification wakes the worker pool, because the worker-pool threads run all sub-engines.
- Offer `notify_push()`, which every producer calls after it pushed into an operations queue, so that every push wakes every sub-engine.
- Give each sub-engine a handle that raises its own notification for these events: result (also of a batch), finished initialization, eviction timer, and an iteration with progress.
- Every notification calls the injected wake function (the worker pool's `Notifier::notify`). The workers that run the sub-engines park only on the worker pool.

Restrictions:
- No `unsafe`.
- Allowed crates: `std` and `log`. No project dependency; the wake function is injected.
- The module has no payload, no event kinds and no counters. A notification is only a wake-up.
- Not in this module:
  - Spawning threads or tasks and running timers. The caller owns the timer and calls `notify_eviction_timer`.
  - Queue logic, resource lifecycle, loop and iteration logic, scheduling.
  - Telling the sub-engines apart at wake-up. Every notification wakes the worker pool; any worker may handle the event.

## 2. Context

- The operations queues are take-based (`queue::Queue`, build order 4) and know no external channels. Producers (the expansion stage after `Processor::expand`, the engine's `FollowUpSink`) call `NotificationTopology::notify_push()` after each push.
- The wake function is `worker_pool::Notifier::notify` (build order 56): it increments an epoch and wakes all parked workers, so a signal is never lost.
- The resource lifecycle (`resource_lifecycle`) is synchronous and has no notifications. `LoadTicket::run` returns `Result<(), LifecycleError>`. The caller of `run` calls `notify_init_done` after `Ok`.
- The loop is the `WorkerLoop` (build order 131) on the worker-pool threads. This module supplies the self-trigger, not the wait and not the loop.
- Per sub-engine there is exactly one handle. `notify_push()` and the own events of every sub-engine call the same wake function.

## 3. Interface

```rust
use std::sync::Arc;

pub type Wake = Arc<dyn Fn() + Send + Sync>;

/// Per-sub-engine handle. Cheap to clone.
#[derive(Clone)]
pub struct SubengineNotifier { /* private: Wake */ }

impl SubengineNotifier {
    pub fn notify_result(&self);
    pub fn notify_init_done(&self);
    pub fn notify_eviction_timer(&self);
    /// Wakes only if `progress` is true.
    pub fn notify_iteration(&self, progress: bool);
}

pub struct NotificationTopology { /* private: Vec<SubengineNotifier>, Wake */ }

impl NotificationTopology {
    /// Creates `subengines` handles (may be 0), index = sub-engine id.
    pub fn new(subengines: usize, wake: Wake) -> Self;
    pub fn len(&self) -> usize;
    pub fn notifier(&self, index: usize) -> Option<SubengineNotifier>;
    /// Called by producers after a push into an operations queue.
    pub fn notify_push(&self);
}
```

`SubengineNotifier` and `NotificationTopology` are `Send + Sync`. No error types exist, and all functions are infallible.

## 4. Behavior and invariants

**Topology**
- `new(n, wake)` creates n handles. Every notification of a handle, and `notify_push()`, calls `wake` exactly once.
- With `new(0, wake)`, `notify_push()` still calls `wake`.

**Own notifications**
- `notify_result`, `notify_init_done` and `notify_eviction_timer` each call the wake function once. They never block and never fail.
- A batch result is one notification (one `notify_result` call). It is not one per task.
- `notify_iteration(true)` calls the wake function once. `notify_iteration(false)` does nothing, so an iteration without progress does not wake the workers by itself and cannot cause a busy loop.
- Signals sent while no worker is parked are not lost: the worker pool compares epochs before parking (build order 56). Several signals may lead to one wake-up.


**Concurrency**
- All methods are callable concurrently from any thread or task through `&self` or clones. No lock is held and nothing is lost.
- A signal wakes all parked workers. Any of them may handle the event.

## 5. Relied-on elements

From `worker_pool` (injected as `Wake`, not a code dependency): `Notifier::notify()` increments the epoch and wakes all parked workers; it never blocks beyond a short lock.

From `resource_lifecycle` (only as the occasion for `notify_init_done`):
- `LoadTicket::run(self, now: Instant) -> Result<(), LifecycleError>`. `Ok` means the state is `Ready`. This module does not call it.