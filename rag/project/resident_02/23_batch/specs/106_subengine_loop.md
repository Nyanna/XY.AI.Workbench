# Spec: Subengine-Loop (module `subengine_loop`, build order 106)

## 1. Task, restrictions and boundaries

Implement one synchronous iteration of a sub-engine lane (`SubengineLoop::run_iteration`). The iteration works through the own queue, then the shared queue, oldest first. For each node it does the following:
- filter the node by its metadata,
- resolve the resource from the requirements,
- trigger lazy initialization if needed,
- claim the node when the resource is ready and has free capacity,
- hand the payload to the resource.

At the end of the iteration it triggers eviction and reports idle resources.

Out of scope:
- Waiting for notifications, the async loop, timers, and running the load ticket (the caller supplies `InitStarter`).
- Queue structure, enqueueing, resource lifecycle logic, and the execution itself (it lives in the resource).
- Tensors, the external neural-network library, and GPU execution graphs (CUDA Graphs). This module knows none of them.
- Reservation, pre-claim, and head-of-line blocking. This module has none of them.
- Priority interpretation. Priority comes only from the filter and the queue order.
- No `unsafe`, no blocking, no spawning. Use the `log` crate.

## 2. Context

- A sub-engine loop waits on its `SubengineNotifier`, calls `run_iteration(now)`, then calls `notify_iteration(outcome.progress())`. The loop is outside this module.
- The queues provide `iter_active(head)` over `QueueNode`s with opaque metadata `M`. Here `M` must expose the requirements (`HasResourceRequirements`).
- Resources are entries in a `ResourceRegistry<H>`. `H` is the handle type and is also the executor.
- Lazy loading is triggered only here. It never happens on enqueue, `register`, or `set_enabled`.
- The task stays `Enqueued` in the queue while its resource is not `Ready`.

## 3. Interface

```rust
use std::sync::Arc;
use std::time::Instant;
use queue_iter::{QueueNode, QueueItem, iter_active};
use queue_state::{QueueState, claim_entry};
use resource_requirements::{HasResourceRequirements, ResourceKind, ResourceRequirements};
use resource_lifecycle::{ResourceRegistry, ResourceKey, InitRequest, LoadTicket, Lease};

/// Resource handle that executes nodes. Implemented by the sub-engine's resource.
pub trait ResourceExec: Sized + Send + Sync + 'static {
    type Payload: Send + 'static;
    /// Non-blocking. True iff a further node could be accepted right now.
    fn has_capacity(&self) -> bool;
    /// Takes over the claimed payload and the lease (hold it until execution ends).
    /// Non-blocking, infallible from the loop's view: the resource runs the node
    /// (or fails its task) itself.
    fn execute(&self, payload: Self::Payload, lease: Lease<Self>);
}

/// Pull filter on the node metadata (params/requirements, incl. placement hint
/// `requirements().affinity()` as a hint only). false = skip the node.
pub type PullFilter<M> = Arc<dyn Fn(&M) -> bool + Send + Sync>;
/// Determines the resource entry from the requirements. None = no entry (skip).
pub type ResourceResolver = Arc<dyn Fn(&ResourceRequirements) -> Option<ResourceKey> + Send + Sync>;
/// Called with the ticket when this iteration won the lazy init. Must run the
/// ticket (`run(now)`, then `notify_init_done` on Ok) without blocking the loop,
/// e.g. on another thread. Dropping the ticket unrun returns the resource to Uninitialized.
pub type InitStarter<H> = Arc<dyn Fn(LoadTicket<H>) + Send + Sync>;

/// A resource of this sub-engine (all of them make up the "all busy" set).
#[derive(Debug, Clone)]
pub struct ResourceSlot { pub key: ResourceKey, pub requirements: ResourceRequirements }

/// Idle-notification occasion: a free resource and no hit.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IdleNotice { pub key: ResourceKey, pub kind: ResourceKind, pub requirements: ResourceRequirements }

#[derive(Debug, Default)]
pub struct IterationOutcome {
    pub claimed: usize,
    pub inits_started: usize,
    pub idle: Vec<IdleNotice>,
    pub evicted: Vec<ResourceKey>,
}
impl IterationOutcome { pub fn progress(&self) -> bool { self.claimed > 0 } }

pub struct SubengineLoop<N, H>
where N: QueueNode<Payload = H::Payload>, N::Meta: HasResourceRequirements, H: ResourceExec { /* private */ }

impl<N, H> SubengineLoop<N, H>
where N: QueueNode<Payload = H::Payload>, N::Meta: HasResourceRequirements, H: ResourceExec {
    pub fn new(own_head: Arc<N>, shared_head: Arc<N>, registry: Arc<ResourceRegistry<H>>,
               slots: Vec<ResourceSlot>, resolver: ResourceResolver,
               filter: PullFilter<N::Meta>, init: InitStarter<H>) -> Self;

    /// One iteration. Never blocks, never panics by itself.
    pub fn run_iteration(&self, now: Instant) -> IterationOutcome;
}
// SubengineLoop is Send + Sync.
```

## 4. Behavior and invariants

**Order**
- Iterate `iter_active(own_head)` completely first, then `iter_active(shared_head)`. Within a queue, go oldest to youngest, node by node.
- A skipped node never blocks later nodes. There is no reservation and no pre-claim.
- A node is considered only if `item.state() == Enqueued`. Otherwise skip it.

**Per node, in this order**
1. If `filter(item.meta())` is false, skip. Nothing else happens for this node.
2. `resolver(meta.resource_requirements())` returns `None`: skip.
3. Call `registry.request_init(key, task_id, now)`. The `task_id` is used only for logging, so pass 0 if none is known. Handle the result:
   - `Ready(lease)`:
     - If `handle().has_capacity()` is false, mark the key busy for this iteration, drop the lease, and skip.
     - Otherwise call `claim_entry(item.entry())`. On `Ok(payload)`, call `handle.execute(payload, lease)` and increment `claimed`. On `Err(_)` (Busy, Cancelled, NotEnqueued, NoPayload), drop the lease and skip without error.
     - After a claim, re-checking the capacity is left to the next `request_init` for that key.
     - Mark the key free-observed when capacity was true.
   - `Started(ticket)`: call `init(ticket)` and increment `inits_started`. The node stays in the queue and is skipped. This is the only place where lazy initialization is triggered.
   - `Loading`, `Backoff{..}`, `Unavailable`: skip. Do not claim, do not change tasks.
- Claim happens only when the resource is Ready, has capacity, and `claim_entry` succeeds. It never happens earlier.

**Termination**
- The iteration ends when the queues are exhausted. It also ends as soon as every `ResourceSlot.key` is marked busy in this iteration. A busy mark holds for the rest of the iteration. With an empty `slots`, this early end never applies.

**End of iteration (always, also on early end)**
- Call `registry.evict_expired(now)` exactly once and put the result into `evicted`.
- Idle notification: if `claimed == 0`, put one `IdleNotice` (key, `requirements.kind()`, slot requirements) into `idle` for each slot observed free (Ready with capacity) during this iteration. Slots that were not visited, or that were Loading, Backoff, or busy, are not reported.
  - Reason: a probe at the end would bump `last_used` and defeat the eviction TTL.
  - An iteration that visits no node reports nothing.
- `claimed > 0` means no idle report.

**Concurrency**
- `run_iteration` takes `&self` and holds no lock across calls. Several lanes (several loops) can run on the same queues. Exactly one claimer wins per node (via `claim_entry`). The others skip that node.
- The loop makes no wake-up calls. The caller notifies via `notify_iteration(outcome.progress())`.

**Errors**
- There are no error values and no panics of its own. Claim and init problems only lead to skipping, with `debug!` logs. Panics from `QueueItem::state()` propagate.

## 5. Relied-on elements

- `queue_iter`: `trait QueueNode { type Payload; type Output; type Error; type Meta; … }`. `iter_active<N>(head: &Arc<N>) -> ActiveIter<N>`, yielding `QueueItem<N>` oldest first, only for entries that are neither Completed nor Cancelled. `QueueItem::{entry() -> &Arc<EntryState<..>>, meta() -> &Meta, state() -> QueueState}`. Iteration is non-blocking.
- `queue_state`: `claim_entry(&EntryState<P,T,E>) -> Result<P, ClaimError>`. Errors are `NotEnqueued(QueueState)`, `Cancelled`, `NoPayload`, `Busy`. A failure leaves the state unchanged. A success moves the payload out exactly once.
- `resource_requirements`: `HasResourceRequirements::resource_requirements(&self) -> &ResourceRequirements`. `ResourceRequirements::{kind() -> ResourceKind, affinity() -> Option<&Affinity>}`. It is `Clone + Debug + Eq`.
- `resource_lifecycle`:
  - `ResourceRegistry<H>::request_init(&self, key: &str, task_id: u64, now: Instant) -> InitRequest<H>`. It never blocks and never touches tasks.
  - `InitRequest::{Ready(Lease<H>), Started(LoadTicket<H>), Loading, Backoff{until}, Unavailable}`. `Started` is given to exactly one concurrent caller.
  - `Lease<H>::handle() -> &Arc<H>`. A plain drop ends the use.
  - `LoadTicket<H>` is `#[must_use]`. `run(self, now) -> Result<(), LifecycleError>`. A drop without `run` returns the state to `Uninitialized`.
  - `evict_expired(&self, now) -> Vec<ResourceKey>`. It evicts only resources that are Ready, have no lease, and have an expired TTL.
- `notification_topology` (used by the caller, not here): `SubengineNotifier::notify_iteration(progress: bool)` and `notify_init_done()`.