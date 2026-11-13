# Spec: Ressourcen-Lebenszyklus (module `resource_lifecycle`, build order 79)

## 1. Task, restrictions and boundaries

Implement the lazy lifecycle of loadable resources (models, indexes, tokenizers, kernels) in Rust. The module covers four things:
- The state machine `Uninitialized → Loading → Ready`, plus `Backoff` after a failure.
- The atomic trigger of lazy initialization.
- Eviction of unused resources after a TTL.
- Writing init and start errors into the task results of a batch and putting the resource into backoff.

Out of scope:
- Queue, scheduling, lanes and iteration logic. The caller is the iteration.
- Executing operations and choosing resources.
- Spawning threads or tasks. The module never spawns and never blocks on I/O itself. It runs the loader synchronously only inside `LoadTicket::run`, on the caller's thread.
- Registering capabilities. The caller calls `register`.

Restrictions:
- No `unsafe`.
- Use the `log` crate.
- Time is always passed in as `std::time::Instant`. The module never reads the clock, so it is testable.
- Dependencies are `task_model` (`Task`, `CompleteError`) and `queue_state` (`QueueState`).
- Operationsknoten is only the origin of the batch tasks. This module imports nothing from it.

## 2. Context

A lane takes an operation from the queue. The iteration needs the operation's resource and asks `request_init`. If the resource is not loaded, **the task stays in the queue** and is not claimed. Initialization runs in the background through the returned `LoadTicket`. Later iterations find the resource `Ready` and take a `Lease` for the duration of the operation.

*Available* means registered and enabled. *Loaded* means state `Ready`, with the handle in memory. Unloading never removes the registration or the enabled flag.

If init or start fails, the tasks of the affected batch are already claimed. The caller passes them to `fail_batch`, which completes them with `Err`. At the end of each iteration the caller calls `evict_expired`.

## 3. Interface

```rust
use std::sync::Arc;
use std::time::{Duration, Instant};
use task_model::{Task, CompleteError};

pub type ResourceKey = String;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MemoryClass { Ram, Vram }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResourceState { Uninitialized, Loading, Ready, Backoff }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LifecycleConfig {
    pub ram_ttl: Duration,
    pub vram_ttl: Duration,      // must be > ram_ttl; enforced by LifecycleConfig::new
    pub backoff: Duration,       // wait after a failure before a retry is allowed
}
impl LifecycleConfig {
    pub fn new(ram_ttl: Duration, vram_ttl: Duration, backoff: Duration)
        -> Result<Self, LifecycleError>;   // Err(InvalidConfig) if vram_ttl <= ram_ttl
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LifecycleError {
    InvalidConfig,
    UnknownResource(ResourceKey),
    AlreadyRegistered(ResourceKey),
    /// Error text of a failed load/start (message for logs and batch errors).
    Failed(String),
}

/// Loads the resource. Runs synchronously inside `LoadTicket::run`.
pub trait ResourceLoader<H>: Send + Sync {
    fn load(&self) -> Result<H, String>;
}

pub struct ResourceRegistry<H> { /* private */ }
// Send + Sync when H: Send + Sync

impl<H: Send + Sync + 'static> ResourceRegistry<H> {
    pub fn new(config: LifecycleConfig) -> Self;
    /// Registers an enabled resource in state Uninitialized (nothing is loaded).
    pub fn register(&self, key: ResourceKey, class: MemoryClass,
                    loader: Arc<dyn ResourceLoader<H>>) -> Result<(), LifecycleError>;
    /// Enables or disables availability. Does not load or unload.
    pub fn set_enabled(&self, key: &str, enabled: bool) -> Result<(), LifecycleError>;
    pub fn is_available(&self, key: &str) -> bool;   // registered && enabled
    pub fn is_loaded(&self, key: &str) -> bool;      // state == Ready
    pub fn state(&self, key: &str) -> Option<ResourceState>;  // None = not registered

    /// Called by the iteration when a free lane draws an operation that needs `key`.
    /// Never blocks, never touches tasks. `task_id` is only used for logging.
    pub fn request_init(&self, key: &str, task_id: u64, now: Instant) -> InitRequest<H>;

    /// Start error of an operation on a loaded resource: any state -> Backoff
    /// (until now + backoff). Drops the registry's handle; existing leases stay valid.
    /// Logs error with task_id.
    pub fn report_failure(&self, key: &str, task_id: u64, reason: &str, now: Instant)
        -> Result<(), LifecycleError>;

    /// End-of-iteration eviction. Returns the evicted keys.
    pub fn evict_expired(&self, now: Instant) -> Vec<ResourceKey>;
}

pub enum InitRequest<H> {
    /// Resource is Ready. The lease marks it as in use.
    Ready(Lease<H>),
    /// This caller won Uninitialized->Loading (or Backoff expired->Loading) and must run the ticket.
    Started(LoadTicket<H>),
    /// Another caller is already loading.
    Loading,
    /// In Backoff; retry is allowed at `until`.
    Backoff { until: Instant },
    /// Not registered or disabled.
    Unavailable,
}

#[must_use]
pub struct LoadTicket<H> { /* key, task_id, registry ref */ }
impl<H: Send + Sync + 'static> LoadTicket<H> {
    /// Runs loader.load() synchronously. Ok -> state Ready, last_used = now.
    /// Err(msg) -> state Backoff (until now + backoff), logs error with task_id,
    /// returns Err(LifecycleError::Failed(msg)).
    /// A panic in the loader is caught (catch_unwind) and treated as Err.
    pub fn run(self, now: Instant) -> Result<(), LifecycleError>;
}

/// Marks the resource as in use. Dropping it ends the use.
pub struct Lease<H> { /* Arc<H> + registry counter */ }
impl<H> Lease<H> {
    pub fn handle(&self) -> &Arc<H>;
    /// Ends the use explicitly and sets last_used = now. A plain drop ends it
    /// without a time update (last_used stays at the value from acquire).
    pub fn release(self, now: Instant);
}

/// Writes `Err(make_err(msg))` into every task of the batch via `Task::complete`.
/// Returns the tasks whose `complete` failed (e.g. not Claimed) with the error.
/// Logs error per task with task_id.
pub fn fail_batch<D, T, E>(
    batch: &[(u64, Task<D, T, E>)],
    reason: &str,
    make_err: impl Fn(&str) -> E,
) -> Vec<(u64, CompleteError)>;
```

## 4. Behavior and invariants

**States and transitions**
- Registered resources start `Uninitialized`. Allowed transitions:
  - Uninitialized→Loading
  - Loading→Ready
  - Loading→Backoff
  - Ready→Uninitialized (eviction)
  - Ready→Backoff (`report_failure`)
  - Backoff→Loading (only when `now >= until`)
- Every transition is atomic per resource (one lock or CAS per resource). Under concurrent `request_init` calls exactly one caller gets `Started`. The others get `Loading`.
- A `LoadTicket` is the only way into `Loading`. If it is dropped without `run`, the state returns to `Uninitialized`.

**request_init**
- `Ready` returns `Ready(Lease)`: `in_use += 1` and `last_used = now`.
- Uninitialized returns `Started`. Backoff with `now < until` returns `Backoff{until}`. Backoff with `now >= until` returns `Started`, which is the retry.
- Disabled or unknown keys return `Unavailable` and do not change state.
- `request_init` never modifies tasks. The task stays in the queue while the resource is `Loading` or `Backoff`. The caller decides what to do with it.
- Only the first task needs to create the resource. Nothing is loaded at `register`, `set_enabled` or startup.

**Failure**
- On a load failure, `LoadTicket::run` sets `Backoff` with `until = now + backoff`, drops no tasks and logs `error!` with `task_id`, key and message.
- `report_failure` logs the same way. Backoff does not unregister or disable the resource.
- `fail_batch` calls `task.complete(Err(make_err(reason)))` for each task and logs `error!` with the task_id.
  - A failure of one task does not stop the others.
  - Returned `CompleteError`s are not panics. With a double completion, `task_model` panics in debug builds, and this module passes that through.
  - An empty batch returns an empty Vec.
- The caller is responsible for calling `report_failure` or `LoadTicket::run` for the state change and `fail_batch` for the results. The two do not call each other.

**Eviction (`evict_expired`)**
- A resource is evicted only if all of these hold:
  - the state is `Ready`
  - `in_use == 0` (no lease alive)
  - `now - last_used >= ttl(class)`, with `ram_ttl` for `Ram` and `vram_ttl` for `Vram`
- Eviction drops the registry's `Arc<H>` and sets `Uninitialized`.
- The resource stays registered and enabled, and a later `request_init` loads it again.
- Resources in `Loading` or `Backoff` and resources with a live lease are never evicted.
- Resources whose TTL has not expired are never evicted.
- The VRAM TTL is strictly longer than the RAM TTL.
- `evict_expired` is idempotent for the same `now`. It logs `debug!` per eviction and does not block.
- `Lease::release(now)` sets `last_used = now`. Without it, `last_used` is the time of the last `request_init` or load.

**Concurrency**
- The registry is `Send + Sync` and shared by `&self` or `Arc`.
- Locks are held only briefly. The loader runs without holding the registry-wide lock. The per-resource state is `Loading` during the load, so other resources are not blocked.

**Errors**
- `register` with an existing key returns `AlreadyRegistered`.
- `set_enabled` and `report_failure` with an unknown key return `UnknownResource`.
- `LifecycleConfig::new` with `vram_ttl <= ram_ttl` returns `InvalidConfig`.

## 5. Relied-on elements

From Task-Modell:
- `Task<D,T,E>` is a `Clone` handle.
- `fn complete(&self, result: Result<T,E>) -> Result<(), CompleteError>`.
  - It is valid only from `Claimed`.
  - It sets the result once and wakes waiters.
  - A second completion returns `Err(NotClaimed(Completed))` and panics in debug builds.
  - A completion on `Enqueued` or `Cancelled` returns `Err(NotClaimed(state))`.
- `Task` has no ID accessor, so the caller passes `u64` task IDs.
- `CompleteError` is `enum { NotClaimed(QueueState) }`, `Debug, Clone, Copy, PartialEq, Eq`.

From Queue-Zustandsmodell:
- `QueueState { Enqueued, Claimed, Completed, Cancelled }`.
- Tasks are claimed (state `Claimed`) before `fail_batch`.
- Unclaimed tasks stay `Enqueued` during `Loading` and `Backoff`.