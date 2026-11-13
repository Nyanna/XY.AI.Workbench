# Spec: Pull-Ausführung (module `pull_exec`, build order 71)

## 1. Task, restrictions and boundaries

Implement the pull loop of **one sub-engine**. The loop has exactly one wait condition, the sub-engine's notification. After each wake-up it iterates over the queues, claims suitable entries, executes them and completes them.

- One `PullLoop` is one loop for one sub-engine. The caller spawns it, for example with `tokio::spawn(loop.run())`. The module does not spawn tasks.
- The loop sequentially pulls from the sub-engine's own queues and the shared queues.
- Out of scope:
  - the queue structure, enqueue and unlinking
  - the state transitions themselves
  - planning and priority interpretation
  - what an executor computes
  - device discovery and worker pools
  - retries, timeouts and error handling beyond what is stated here
- Allowed crates: `tokio` (feature `sync`, plus `task::yield_now`), `log`, `std`. Allowed project dependencies: `queue_iter`, `queue_state`, `queue` (for `NotifyChannel`), `resource_requirements`. No `unsafe`.
- The loop never waits on anything except `Notify::notified()`. It has no timer, no polling sleep and no second channel. It does not spin. `yield_now` is allowed only as stated in section 4.
- Metadata is opaque. Only the injected `PullFilter` interprets it.

## 2. Context

- Producers call `Queue::enqueue`. Every enqueue calls `notify_one()` once on each registered `NotifyChannel`. `Notify` stores one permit, and several signals coalesce into one. After a wake-up the loop must therefore drain the queues instead of assuming one entry per wake-up.
- The caller creates one `NotifyChannel` (`Arc<Notify>`) per sub-engine and registers the same channel in every queue the sub-engine pulls from (its own and the shared ones). The caller passes the queues' head nodes (`Queue::head()`) to this module.
- Entries are claimed with `EntryState::claim()` and completed with `complete()`. Both are specified in `queue_state`.
- Neighbours:
  - `queue` provides the nodes and the channels.
  - `queue_iter` provides the iteration.
  - The Planner and the sub-engines supply the filter and the executor.
  - `resource_requirements` provides the ready-made `RequirementsFilter`.

## 3. Interface

```rust
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, atomic::AtomicBool};
use queue::NotifyChannel;
use queue_iter::QueueNode;

pub type BoxFuture<'a, O> = Pin<Box<dyn Future<Output = O> + Send + 'a>>;

/// Decides from the opaque metadata alone whether this sub-engine takes the entry.
/// Pure, non-blocking. Rejected entries are left untouched.
pub trait PullFilter<M>: Send + Sync {
    fn accepts(&self, meta: &M) -> bool;
}
impl<M, F: Fn(&M) -> bool + Send + Sync> PullFilter<M> for F {}

/// Executes a claimed payload (moved in) and returns the result stored via `complete`.
pub trait Executor<P, T, E>: Send + Sync {
    fn execute(&self, payload: P) -> BoxFuture<'_, Result<T, E>>;
}
impl<P, T, E, F> Executor<P, T, E> for F
    where F: Fn(P) -> BoxFuture<'static, Result<T, E>> + Send + Sync {}

/// Ready-made filter for the requirements type of `resource_requirements`.
/// accepts(meta) = meta.resource_requirements().kind() == kind
///   && meta.resource_requirements().is_satisfied_by(device_kind, &device_capabilities).
/// Affinity is ignored (hint only).
pub struct RequirementsFilter { /* kind, device_kind, device_capabilities */ }
impl RequirementsFilter {
    pub fn new(kind: resource_requirements::ResourceKind,
               device_kind: device::DeviceKind,
               device_capabilities: capability_model::CapabilitySet) -> Self;
}
impl<M: resource_requirements::HasResourceRequirements> PullFilter<M> for RequirementsFilter {}

/// Stop handle. `stop()` sets the flag, then calls `notify_one()` on the loop's channel. Cloneable, non-blocking.
#[derive(Clone)]
pub struct StopHandle { /* Arc<AtomicBool>, NotifyChannel */ }
impl StopHandle { pub fn stop(&self); }

pub struct PullLoop<N: QueueNode> { /* private */ }

impl<N: QueueNode> PullLoop<N> {
    /// `own`: heads (sentinels) of the sub-engine's own queues. `shared`: heads of the shared queues.
    /// `notify`: this sub-engine's channel, which is registered in all these queues.
    pub fn new(
        notify: NotifyChannel,
        own: Vec<Arc<N>>,
        shared: Vec<Arc<N>>,
        filter: Arc<dyn PullFilter<N::Meta>>,
        executor: Arc<dyn Executor<N::Payload, N::Output, N::Error>>,
    ) -> Self;

    pub fn stop_handle(&self) -> StopHandle;

    /// Runs the loop until stopped. Returns `()`. There is no error type.
    pub async fn run(self);
}
```

`PullLoop<N>` is `Send` when `N: QueueNode` and `N::Payload: Send`, `N::Output: Send + Sync` and `N::Error: Send + Sync`. The future returned by `run` is `Send` under the same conditions. `StopHandle` is `Send + Sync`.

## 4. Behavior and invariants

**Loop structure**
- `run` repeats the following until stopped:
  1. `notify.notified().await`. This is the only wait.
  2. If the stop flag is set, return.
  3. Drain: repeat passes (see below) until a pass ends with no executed entry and no `Busy`.
  4. Go back to step 1.
- A permit stored before `run` starts, or while the loop is working, makes the next `notified().await` return immediately. No enqueue signal is lost.
- On start `run` waits first. Entries enqueued before `run` started are found because the channel's permit was stored by their enqueue. If the caller registered the channel in a queue that already holds entries and no signal was sent, the caller must call `notify.notify_one()` once. The module does not do this itself.

**Pass**
- A pass scans the queues in this order: all `own` queues in the given order, then all `shared` queues in the given order.
- Each queue is scanned with `queue_iter::iter_active(head)`, oldest first. Queue order is the only order. Priority is not interpreted.
- For each yielded item:
  1. If `filter.accepts(item.meta())` is false, skip it.
  2. Otherwise call `item.entry().claim()`.
- Claim outcomes:
  - `Ok(payload)`:
    1. Run `executor.execute(payload).await`.
    2. Call `entry.complete(result)`.
    3. End the pass as "executed". The next pass restarts at the first own queue, so own work comes before shared work.
  - `Err(NotEnqueued(_))`, `Err(Cancelled)` or `Err(NoPayload)`: skip the item without a retry. Another worker or a cancel was faster.
  - `Err(Busy)`: skip the item and mark the pass as "busy". The state is unchanged, so the entry may still be claimable.
- A pass that scanned all queues without executing anything ends as "empty", or as "busy" if a `Busy` was seen.
- After a "busy" pass the loop calls `tokio::task::yield_now().await` and runs another pass. This is bounded by the contention and is not a wait condition.
- After an "empty" pass the drain ends.
- Entries added to a queue during the drain are either seen by the ongoing pass or covered by the permit their enqueue stored. A new wake-up follows in the latter case.

**Execution and completion**
- Exactly one entry is executed at a time per loop. Parallelism comes from several loops.
- The payload is moved from the claim into the executor, exactly once.
- The result of the executor (`Ok` or `Err`) is passed unchanged to `complete`. `Err(CompleteError)` from `complete` is only logged with `warn!` and does not stop the loop.
- Executor panics are not caught and propagate out of `run`. The module does not auto-restart.
- Filtered-out entries are never claimed and never change state.

**Stop**
- `StopHandle::stop()` sets the flag with `Release` and then calls `notify_one()` on the loop's channel.
- The loop checks the flag with `Acquire` after each wake-up and after finishing each executed entry. It returns at the first check that sees the flag.
- An entry that is already claimed is executed and completed before the loop returns. Claimed entries are never abandoned.
- After `run` returns, no entry is left in `Claimed` by this loop.
- Stopping an already stopped loop has no effect.

**Concurrency**
- Many loops may run on the same queues at the same time. For each entry, `claim()` guarantees that at most one loop executes it.
- A loop does not hold locks. It does not block a thread, because it only uses `.await` and `yield_now`.
- Empty `own` and `shared` lists are valid. The loop then only waits and returns on stop.

**Errors**
- No error type. All claim failures are handled as described above. `debug!` logs skips, and `debug!` or `info!` logs executions.

## 5. Relied-on elements

From `queue_iter` (used as specified):
```rust
pub trait QueueNode: Send + Sync + 'static {
    type Payload; type Output; type Error; type Meta;
    fn is_sentinel(&self) -> bool;
    fn next(&self) -> Option<Arc<Self>>;
    fn entry(&self) -> &Arc<EntryState<Self::Payload, Self::Output, Self::Error>>;
    fn meta(&self) -> &Self::Meta;
}
pub struct QueueItem<N: QueueNode>(..); // entry() -> &Arc<EntryState<..>>, meta() -> &N::Meta, state(), node()
pub fn iter_active<N: QueueNode>(head: &Arc<N>) -> ActiveIter<N>; // Iterator<Item = QueueItem<N>>, Send, non-blocking,
                                                                  // yields Enqueued/Claimed, skips sentinels, starts at head.next()
```

From `queue` (used as specified):
- `pub type NotifyChannel = Arc<Notify>`. Every `enqueue` calls `notify_one()` once per registered channel after the append. `Queue::head()` returns the sentinel node.
- Its node type implements `queue_iter::QueueNode`.

From `queue_state` (used as specified):
- `EntryState::claim(&self) -> Result<P, ClaimError>`. It never blocks and moves the payload out exactly once.
- `ClaimError { NotEnqueued(QueueState), Cancelled, NoPayload, Busy }`. Every failure leaves the state unchanged. `Busy` means the payload compartment was locked.
- `EntryState::complete(&self, Result<T, E>) -> Result<(), CompleteError>`. It is only valid from `Claimed` and stores the result before publishing `Completed`.
- A second `complete` logs a warning and panics in debug builds via `debug_assert!`. This loop calls `complete` exactly once per claim.
- `CompleteError::NotClaimed(QueueState)`.

From `resource_requirements`:
```rust
pub trait HasResourceRequirements { fn resource_requirements(&self) -> &ResourceRequirements; }
// ResourceRequirements: kind() -> ResourceKind;
// is_satisfied_by(&self, kind: DeviceKind, device_capabilities: &CapabilitySet) -> bool  (pure; affinity ignored)
// ResourceKind: Copy + PartialEq (#[non_exhaustive]: Tokenizer, Normalizer, Encoder, Scorer, Index, CrossEncoder)
```
`DeviceKind` comes from `device` and `CapabilitySet` from `capability_model`. Both are used unchanged.

From `Operation`:
- An `Operation<R>` is a typical payload `P`. This module treats `P` as opaque. The `Operation` module's own `ResourceRequirements` (strings) is a different type from `resource_requirements::ResourceRequirements` and is not used here.