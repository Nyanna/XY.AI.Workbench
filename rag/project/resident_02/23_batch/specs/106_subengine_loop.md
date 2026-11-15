# Spec: Subengine-Loop (module `subengine_loop`, build order 106)

## 1. Task, restrictions and boundaries

Implement one synchronous iteration of a sub-engine (`SubengineLoop::run_iteration`). It runs on a worker-pool thread inside the `WorkerLoop` (build order 131). The iteration works through the own queue, then the shared queue (take-based `queue::Queue`, build order 4), oldest first. For each entry it does the following:
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
- No `unsafe`, no spawning. Use the `log` crate. The iteration itself never waits; `ResourceExec::execute` of a CPU resource runs the work synchronously on the calling worker (blocking there is the natural backpressure).

## 2. Context

- A sub-engine loop waits on its `SubengineNotifier`, calls `run_iteration(now)`, then calls `notify_iteration(outcome.progress())`. The loop is outside this module.
- The queues provide `take_where(pred)` over opaque metadata `M`. Here `M` must expose the requirements (`HasResourceRequirements`). A claimed entry cannot return to Pending (dropping its `Claim` makes it `Abandoned`), so the lease is obtained before the claim.
- Resources are entries in a `ResourceRegistry<H>`. `H` is the handle type and is also the executor.
- Lazy loading is triggered only here. It never happens on enqueue, `register`, or `set_enabled`.
- The entry stays Pending in the queue while its resource is not `Ready`.

## 3. Interface

```rust
use std::sync::Arc;
use std::time::Instant;
use queue::{Queue, Claim};
use resource_requirements::{HasResourceRequirements, ResourceKind, ResourceRequirements};
use resource_lifecycle::{ResourceRegistry, ResourceKey, InitRequest, LoadTicket, Lease};

/// Resource handle that executes nodes. Implemented by the sub-engine's resource.
pub trait ResourceExec: Sized + Send + Sync + 'static {
    /// The claimed queue entry, normally `queue::Claim<M, P, (), ()>`.
    type Entry: Send + 'static;
    /// Non-blocking. True iff a further entry could be accepted right now.
    fn has_capacity(&self) -> bool;
    /// Takes over the claim and the lease (hold both until the work is finished).
    /// CPU resources run the work synchronously on the calling thread; GPU resources only
    /// launch it and return. Infallible from the loop's view.
    fn execute(&self, entry: Self::Entry, lease: Lease<Self>);
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

pub struct SubengineLoop<M, P, H>
where M: HasResourceRequirements + Send + Sync + 'static, P: Send + 'static,
      H: ResourceExec<Entry = Claim<M, P, (), ()>> { /* private */ }

impl<M, P, H> SubengineLoop<M, P, H>
where M: HasResourceRequirements + Send + Sync + 'static, P: Send + 'static,
      H: ResourceExec<Entry = Claim<M, P, (), ()>> {
    pub fn new(own: Arc<Queue<M, P, (), ()>>, shared: Option<Arc<Queue<M, P, (), ()>>>,
               registry: Arc<ResourceRegistry<H>>, slots: Vec<ResourceSlot>,
               resolver: ResourceResolver, filter: PullFilter<M>, init: InitStarter<H>) -> Self;

    /// One iteration. Never blocks, never panics by itself.
    pub fn run_iteration(&self, now: Instant) -> IterationOutcome;
}
// SubengineLoop is Send + Sync.
```

## 4. Behavior and invariants

**Order**
- Process the own queue completely first, then the shared queue (if any).
- A skipped entry never blocks later entries. There is no reservation and no pre-claim.

**Per queue, in this order**
1. Survey: call `take_where` with a predicate that always returns `false`. For every metadata `m` it sees, if `filter(m)` is true and `resolver(m.resource_requirements())` is `Some(key)`, record `key` once, in first-seen order. Nothing is claimed.
2. For each recorded key, in that order, unless the key is marked busy, call `registry.request_init(key, 0, now)`:
   - `Ready(lease)`:
     - If `handle().has_capacity()` is false, mark the key busy for this iteration, drop the lease and continue with the next key.
     - Otherwise call `take_where(|m| filter(m) && resolver(m.resource_requirements()) == Some(key))`. On `Some(claim)`, call `handle.execute(claim, lease)`, increment `claimed` and repeat step 2 for the same key. On `None`, drop the lease.
     - Mark the key free-observed when capacity was true.
   - `Started(ticket)`: call `init(ticket)` and increment `inits_started`. The entries stay Pending. This is the only place where lazy initialization is triggered.
   - `Loading`, `Backoff{..}`, `Unavailable`: skip the key.
- A claim happens only after a lease of a Ready resource with capacity was obtained. It never happens earlier.

**Termination**
- The iteration ends when the queues are exhausted. It also ends as soon as every `ResourceSlot.key` is marked busy in this iteration. A busy mark holds for the rest of the iteration. With an empty `slots`, this early end never applies.

**End of iteration (always, also on early end)**
- Call `registry.evict_expired(now)` exactly once and put the result into `evicted`.
- Idle notification: if `claimed == 0`, put one `IdleNotice` (key, `requirements.kind()`, slot requirements) into `idle` for each slot observed free (Ready with capacity) during this iteration. Slots that were not visited, or that were Loading, Backoff, or busy, are not reported.
  - Reason: a probe at the end would bump `last_used` and defeat the eviction TTL.
  - An iteration that visits no node reports nothing.
- `claimed > 0` means no idle report.

**Concurrency**
- `run_iteration` takes `&self` and holds no lock across calls. Several workers can run it on the same queues at the same time. Exactly one claimer wins per entry (`take_where`). A worker that loses an entry gets `None` and drops its lease.
- The loop makes no wake-up calls. The caller notifies via `notify_iteration(outcome.progress())`.

**Errors**
- There are no error values and no panics of its own. Claim and init problems only lead to skipping, with `debug!` logs. Panics of `filter` or `resolver` propagate; the queue stays consistent (build order 4).

## 5. Relied-on elements

- `queue`: `Queue<M,P,R,E>::take_where(FnMut(&M) -> bool) -> Option<Claim>` claims the oldest Pending entry accepted by the predicate; the predicate may run on entries another consumer claims at the same time. `Claim::{meta, take_payload, complete}`; dropping a `Claim` without `complete` makes the entry `Abandoned`.
- `resource_requirements`: `HasResourceRequirements::resource_requirements(&self) -> &ResourceRequirements`. `ResourceRequirements::{kind() -> ResourceKind, affinity() -> Option<&Affinity>}`. It is `Clone + Debug + Eq`.
- `resource_lifecycle`:
  - `ResourceRegistry<H>::request_init(&self, key: &str, task_id: u64, now: Instant) -> InitRequest<H>`. It never blocks and never touches tasks.
  - `InitRequest::{Ready(Lease<H>), Started(LoadTicket<H>), Loading, Backoff{until}, Unavailable}`. `Started` is given to exactly one concurrent caller.
  - `Lease<H>::handle() -> &Arc<H>`. A plain drop ends the use.
  - `LoadTicket<H>` is `#[must_use]`. `run(self, now) -> Result<(), LifecycleError>`. A drop without `run` returns the state to `Uninitialized`.
  - `evict_expired(&self, now) -> Vec<ResourceKey>`. It evicts only resources that are Ready, have no lease, and have an expired TTL.
- `notification_topology` (used by the caller, not here): `SubengineNotifier::notify_iteration(progress: bool)` and `notify_init_done()`.