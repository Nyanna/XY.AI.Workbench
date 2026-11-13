# Spec: Notification-Topologie (module `notification_topology`, build order 92)

## 1. Task, restrictions and boundaries

Build the notification wiring for the sub-engines in Rust:
- Create one notification channel per sub-engine.
- Pass these channels to the queue when it is created, so that every queue append wakes every sub-engine.
- Give each sub-engine a handle that raises its own notification for these events: result (also of a batch), finished initialization, eviction timer, and an iteration with progress.
- The loop of a sub-engine is its own notification: it waits on that channel and is woken by every event above.

Restrictions:
- No `unsafe`.
- Allowed crates: `tokio` (feature `sync`) and `log`.
- Allowed project dependency: `queue` (`Queue`, `NotifyChannel`).
- The module has no payload, no event kinds and no counters. A notification is only a wake-up.
- Not in this module:
  - Spawning threads or tasks and running timers. The caller owns the timer and calls `notify_eviction_timer`.
  - Queue logic, resource lifecycle, loop and iteration logic, scheduling.
  - Notifying other sub-engines. Result, init, eviction and progress events wake only the sub-engine they belong to.

## 2. Context

- `queue::Queue::new(channels: Vec<NotifyChannel>)` stores the channels. Every `enqueue` calls `notify_one()` once per channel, after the append and without blocking. `NotifyChannel = Arc<tokio::sync::Notify>`. `Notify` stores one permit, so a signal is never lost, and several signals may coalesce into one.
- The resource lifecycle (`resource_lifecycle`) is synchronous and has no notifications. `LoadTicket::run` returns `Result<(), LifecycleError>`. The caller of `run` calls `notify_init_done` after `Ok`.
- A sub-engine loop is: wait for its notification, run an iteration, and repeat if the iteration made progress. This module supplies the wait and the self-trigger, not the loop.
- Per sub-engine there is exactly one `Notify`. It is the channel given to the queue and the channel of the handle. Queue appends and the own events therefore wake the same loop.

## 3. Interface

```rust
use std::sync::Arc;
use queue::{Queue, NotifyChannel};

/// Per-sub-engine handle. Cheap to clone; all clones share one channel.
#[derive(Clone)]
pub struct SubengineNotifier { /* private: NotifyChannel */ }

impl SubengineNotifier {
    /// Waits for the next notification (queue append or own event).
    /// Consumes a stored permit immediately if one exists. Cancel-safe.
    pub async fn notified(&self);

    /// A task result was written (single task or whole batch: one call per batch is enough).
    pub fn notify_result(&self);
    /// A resource initialization finished (call after `LoadTicket::run` returned Ok).
    pub fn notify_init_done(&self);
    /// The eviction timer fired.
    pub fn notify_eviction_timer(&self);
    /// Reports the end of an iteration. If `progress` is true, triggers own notification
    /// (the loop runs again). If false, does nothing.
    pub fn notify_iteration(&self, progress: bool);

    /// The underlying channel (the same one the queue signals).
    pub fn channel(&self) -> &NotifyChannel;
}

pub struct NotificationTopology { /* private: Vec<SubengineNotifier> */ }

impl NotificationTopology {
    /// Creates `subengines` fresh channels (may be 0), index = sub-engine id.
    pub fn new(subengines: usize) -> Self;
    pub fn len(&self) -> usize;
    /// Handle of sub-engine `index`; None if out of range. Clones share the channel.
    pub fn notifier(&self, index: usize) -> Option<SubengineNotifier>;
    /// All channels in index order (clones of the Arcs).
    pub fn channels(&self) -> Vec<NotifyChannel>;
    /// Creates the queue with exactly these channels: `Queue::new(self.channels())`.
    pub fn build_queue<M, P, T, E>(&self) -> Queue<M, P, T, E>;
}
```

`SubengineNotifier` and `NotificationTopology` are `Send + Sync`. No error types exist, and all functions are infallible.

## 4. Behavior and invariants

**Topology**
- `new(n)` creates n distinct channels (`Arc::ptr_eq` is false between different indices). `notifier(i).channel()` is `Arc::ptr_eq` to `channels()[i]` and to the i-th channel inside the queue from `build_queue`.
- Every `queue.enqueue` therefore stores a permit on every sub-engine's channel, and each `notified()` can be woken by it. This is verifiable by calling `enqueue` and then awaiting `notified()` on each notifier without a timeout.
- With `new(0)`, `build_queue` returns a queue without channels, and `enqueue` still works.
- `build_queue` may be called more than once. All queues then share the same channels.

**Own notifications**
- `notify_result`, `notify_init_done` and `notify_eviction_timer` each call `notify_one()` once on the channel of this notifier only. They never block, never fail and signal no other sub-engine.
- A batch result is one notification (one `notify_result` call). It is not one per task.
- `notify_iteration(true)` calls `notify_one()` once. `notify_iteration(false)` does nothing, so an iteration without progress does not wake the loop by itself and cannot cause a busy loop.
- Signals sent while the loop is not waiting are kept as one permit. Several signals before `notified()` coalesce into a single wake-up. The next `notified()` after that blocks again until a new signal.
- A signal sent from inside the loop's own iteration (for example `notify_iteration(true)`) makes the next `notified()` return immediately.

**Concurrency**
- All methods are callable concurrently from any thread or task through `&self` or clones. No lock is held and nothing is lost.
- A single `Notify` supports one waiter per loop. The caller must not run several loops on one notifier. If they do, each permit wakes only one of them.

## 5. Relied-on elements

From `queue`:
- `pub type NotifyChannel = Arc<tokio::sync::Notify>;`
- `Queue::<M,P,T,E>::new(channels: Vec<NotifyChannel>) -> Queue<M,P,T,E>`: any number of channels, including none.
- `Queue::enqueue(&self, meta, state) -> Arc<QueueNode<..>>`: after the append, it calls `notify_one()` on each channel exactly once. This is non-blocking and infallible.
- This module does not change, wrap or duplicate that signaling.

From `resource_lifecycle` (only as the occasion for `notify_init_done`):
- `LoadTicket::run(self, now: Instant) -> Result<(), LifecycleError>`. `Ok` means the state is `Ready`. This module does not call it.