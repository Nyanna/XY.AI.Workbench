# Subengine: Technical Specification (build order 134)

## 1. Task, restrictions and boundaries

**Task.** Implement the Rust module `subengine`. It is a compute substrate for one hardware kind, either CPU or GPU (CUDA). One instance exists per hardware kind. The instance does three things:
- It owns the resources, memory, streams and kernels of its hardware kind.
- It provides execution capacity (worker lanes).
- It pulls its own work from the Processor's queues.

It implements the `planner::Subengine` trait, which the Planner uses to start, query, announce to and activate it.

**Restrictions.**
- It only pulls. It never receives pushed work. Nothing else pushes into or calls into its lanes.
- It reports only how much work it can take (`free_slots`). It reports nothing else about load or scheduling.
- It never pushes into queues, plans, expands tasks or decides queue assignment. It does not cancel tasks or retry them.
- Memory comes only from `speicherverwaltung::Speicher`, which is reserved once in `start`. There is no VRAM allocation or free during operation.
- It links no CUDA library directly. Device and kernel access sits behind the injected `ComputeBackend` trait. A CUDA backend and a CPU backend are implementations of that trait.
- CPU and GPU instances share no state. There are no globals.
- Crates: `std`, `log`. No `unsafe` except what a backend needs, and none in this module's own code.
- It may be used from several threads (the Planner and the workers).

## 2. Context

- **Queue** (`queue::Queue<M,P,R,E>`) is used as `Queue<OpMeta, V, R, E>`, with three instances created by the Processor (Cpu, Gpu, Shared). `take_where(pred)` claims the oldest Pending item whose metadata satisfies the predicate. The `Claim` is completed with `complete(Result<R,E>)`. If the `Claim` is dropped, the item is Abandoned.
- **Planner** defines `Subengine` and `OpMeta`. It calls `start`, then reads `capabilities` and `offers`. For each created operation it calls `announce(&ResourceRequirements)` and then `activate()`. It only reads `is_loaded`. Planner queue assignment, which this module must be consistent with:
  - A queue is chosen only if the union of capability sets of its sub-engines satisfies `resource.capabilities()`, and at least one sub-engine assigned to that queue offers `resource.kind()`.
  - `Any` goes preferably to Shared, otherwise Gpu, otherwise Cpu.
  - `GpuOnly` goes to Gpu. `CpuOnly` goes to Cpu.
- **Speicherverwaltung** provides the arenas (Persistent, Step, Dynamic) and stream-ordered reuse. This module is its only user here.
- **Resource-Requirements** and **Operation** supply the vocabulary of `OpMeta` and its fields.
- **Engine** (pending) wires the queues and the backend, and constructs the sub-engines.

## 3. Interface

```rust
use std::sync::Arc;
use std::time::Duration;
use capability_model::CapabilitySet;
use device::DeviceKind;
use planner::{OpMeta, Subengine};
use processor::QueueKind;
use queue::Queue;
use resource_requirements::{ResourceKind, ResourceRequirements, ModelId};
use speicherverwaltung::{Config as MemConfig, Fence, Speicher, StreamId};

/// Hardware-specific part, injected. Owns kernels, device handles and streams.
/// Both methods are called from several threads.
pub trait ComputeBackend<V, R, E>: Send + Sync + 'static {
    fn device_kind(&self) -> DeviceKind;
    /// Capability set of this hardware (offered to the Planner).
    fn capabilities(&self) -> CapabilitySet;
    /// Called once in start, before workers are spawned.
    fn init(&self, mem: &Speicher, lanes: usize) -> Result<(), String>;
    /// Loads/initializes the resource for `req` (e.g. model weights into Persistent memory).
    /// Called at most once per distinct (kind, model, index) while it succeeds.
    fn load(&self, req: &ResourceRequirements, mem: &Speicher) -> Result<(), String>;
    /// Runs one operation synchronously on the lane's stream. Returns when the stream work is done.
    /// `meta.materialize` tells whether the output must be written to RAM.
    fn execute(&self, meta: &OpMeta, payload: V, ctx: &ExecContext<'_>) -> Result<R, E>;
}

/// Per-execution view of the lane.
pub struct ExecContext<'a> { /* private */ }
impl<'a> ExecContext<'a> {
    pub fn lane(&self) -> usize;
    pub fn stream(&self) -> StreamId;          // StreamId(lane as u32)
    pub fn memory(&self) -> &Speicher;
    /// Next fence of this lane's stream (strictly increasing, starts at Fence(1)).
    pub fn next_fence(&self) -> Fence;
}

pub struct SubengineConfig {
    pub name: String,                    // unique, non-empty
    pub lanes: usize,                    // worker threads = streams = max parallel operations, >= 1
    pub memory: MemConfig,               // memory.max_streams must be >= lanes
    pub offers: Vec<ResourceKind>,       // initial resource kinds offered
    pub poll_interval: Duration,         // wait slice of a worker, > 0 (recommended 20 ms)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SubengineError {
    InvalidConfig(String),   // empty name, lanes == 0, max_streams < lanes, zero poll_interval, duplicate offers allowed
    MissingQueue(QueueKind), // the primary queue was not provided
    Memory(String),          // Speicher::new failed (Debug text of MemError)
}
// Display + std::error::Error.

pub struct SubengineBuilder<V, R, E> { /* private */ }
impl<V: Send + 'static, R: Send + 'static, E: Send + 'static> SubengineBuilder<V, R, E> {
    pub fn new(cfg: SubengineConfig, backend: Arc<dyn ComputeBackend<V, R, E>>) -> Self;
    /// Queue to pull from. Allowed: Cpu (CPU only), Gpu (GPU only), Shared (both).
    /// Primary queue = Cpu for DeviceKind::Cpu, Gpu for DeviceKind::Gpu (required).
    /// Shared is optional. A queue of the other hardware kind is ignored.
    pub fn queue(self, kind: QueueKind, q: Arc<Queue<OpMeta, V, R, E>>) -> Self;
    pub fn build(self) -> Result<ComputeSubengine<V, R, E>, SubengineError>; // validates, starts nothing
}

pub struct ComputeSubengine<V, R, E> { /* private */ }
// Send + Sync (V, R, E: Send + 'static). Dropping it calls shutdown().

impl<V, R, E> ComputeSubengine<V, R, E> {
    /// Idle worker lanes that may pull right now (0..=lanes). 0 before start and after shutdown.
    pub fn free_slots(&self) -> usize;
    pub fn lanes(&self) -> usize;
    pub fn memory(&self) -> &Speicher;      // available from build on
    /// Stops pulling, lets running operations finish, joins workers. Idempotent. Blocking.
    pub fn shutdown(&self);
}

impl<V, R, E> planner::Subengine for ComputeSubengine<V, R, E> { /* see §4 */ }
```

## 4. Behavior and invariants

**Configuration and start**
- `build` validates `SubengineConfig` and creates the `Speicher` (this is the one-time memory reservation). A `Speicher::new` failure gives `Memory`. A missing primary queue gives `MissingQueue`. `build` starts no threads.
- `name()` returns `cfg.name`.
- `start()` is idempotent. The first call does the following in order:
  - It calls `backend.init(&mem, lanes)`.
  - It spawns `lanes` worker threads, with lane *i* using `StreamId(i)`.
  - It returns `Ok(())`.
  - If `init` fails it returns `Err(reason)`, spawns no workers and stays not started, so a later `start` may retry.
  - After `shutdown`, `start` returns `Err`.
- `capabilities()` is `[(primary, caps), (Shared, caps)]` with `caps = backend.capabilities()`. Shared is included only if a Shared queue was provided. The result is the same on every call.
- `offers()` returns `cfg.offers` without duplicates, in configured order.

**Announce, activate, loaded**
- `announce(req)` is non-blocking. It only records `req` in a pending set (short lock) and never loads anything. Equal requirements are recorded once.
- `activate()` is blocking and idempotent. Calls are serialized.
  - It loads every pending requirement not yet loaded, in announcement order. It calls `backend.load(req, mem)` for each.
  - Each requirement whose `kind()` is not in `offers()` makes `activate` return `Err("resource kind not offered: {kind:?}")`. This happens before it loads anything for that requirement.
  - On the first failure it returns `Err(reason)`. The failed requirement and all later ones stay pending, so a retry is possible. Already loaded ones stay loaded.
  - With nothing pending it returns `Ok(())` at once.
  - It does not require `start` to have been called. It returns `Err("not started")` if `start` was not called.
- A requirement counts as loaded after `load` returned `Ok`. The key is (kind, model, index).
- `is_loaded(kind, model)` is true iff a loaded entry exists with that `kind`, and either `model` is `None` or the entry's model equals `model`. It never loads or blocks on loading, apart from a short lock.

**Pull principle (workers)**

Each worker repeats the following until shutdown:
1. It reads `epoch(Available)` of the primary queue.
2. It calls `take_where(pred)` on the primary queue. If the result is `None` and a Shared queue exists, it calls `take_where(pred)` on Shared.
3. If both give `None`, it calls `wait_change(Available, seen, Some(poll_interval))` on the primary queue. Because of the poll slice, Shared items are also noticed.
4. On `Some(claim)`, it executes, completes and starts the loop again.

Further rules:
- A worker pulls only when it is idle. So at most `lanes` operations run at once, and `free_slots() == lanes - running`.
- **Predicate** `pred(meta)` is true iff all of the following hold:
  - `meta.resource.environment().accepts(device_kind)`
  - `caps.satisfies(&meta.required)`
  - `meta.resource.kind()` is in `offers()`
  - a loaded entry exists matching (kind, model, index) of `meta.resource`
- The predicate has no side effects and takes no lock held across a blocking call. Items it rejects stay Pending and are untouched, so another sub-engine may take them. The primary queue is tried before Shared.
- No pre-taking or reservation. An item is claimed only at the moment a worker is idle and ready to run it.
- **Execution of a claim:**
  - The worker takes the payload with `take_payload` and calls `backend.execute(meta, payload, &ctx)`.
  - It then calls `mem.complete(stream, last_fence)`, where `last_fence` is the highest fence issued on the stream via `ctx.next_fence()`. This is skipped if none was issued.
  - Finally it calls `claim.complete(result)`.
- A panic in `execute` is caught. The `Claim` is dropped, so the item becomes Abandoned. The worker survives and logs the panic. Panics in `pred` leave the item Pending (Queue guarantee) and are handled the same way.
- If `take_payload` returns `None`, the claim is dropped (Abandoned).

**Memory**
- All memory of this instance comes from its own `Speicher`. Its calls are thread-safe. Persistent memory is intended for loaded resources, and Dynamic memory for per-operation temporaries. Backends use `ctx.next_fence()` as the fence for `free`.
- Two lanes never share a `StreamId`. Fences per stream are strictly increasing.

**Shutdown**
- `shutdown` sets a stop flag, so no new claims are taken. A running operation finishes and is completed normally. Pending queue items are left in the queues, and the workers are joined.
- After `shutdown`, `free_slots() == 0`.

**Concurrency**
- All methods take `&self` and can be called concurrently. `announce`, `is_loaded`, `capabilities`, `offers` and `free_slots` are non-blocking, apart from short locks. Only `activate`, `start` and `shutdown` block.
- No lock is held while calling `backend.execute`, `Queue` blocking waits or `claim.complete`.
- `backend.load` is not called concurrently for the same instance.

**Edge cases**
- A hardware kind that offers no matching resource never takes the item (predicate false). The item stays for others.
- An empty queue makes the workers wait. They do not busy-spin.
- Calling `start` on an instance whose Shared queue is absent is valid.

## 5. Relied-on elements

- `planner::Subengine` (all methods `&self`, `Send + Sync`):
  - `name() -> String`
  - `start() -> Result<(), String>` (idempotent)
  - `capabilities() -> Vec<(QueueKind, CapabilitySet)>`
  - `offers() -> Vec<ResourceKind>`
  - `announce(&ResourceRequirements)` (non-blocking)
  - `activate() -> Result<(), String>` (blocking, idempotent)
  - `is_loaded(ResourceKind, Option<&ModelId>) -> bool`
- `planner::OpMeta { task_id, priority: i32, queue: QueueKind, kind: OperationKind, descriptor, resource: ResourceRequirements, required: Requirements, materialize: bool }`. It is `Clone + Debug` and immutable. `required == resource.capabilities()`.
- `processor::QueueKind { Cpu, Gpu, Shared }`.
- `queue::Queue<M,P,R,E>`:
  - `take_where(FnMut(&M)->bool) -> Option<Claim>`
  - `epoch(Channel)`
  - `wait_change(Channel, seen, Option<Duration>) -> u64`
  - `Channel::Available`
- `Claim`:
  - `meta()`
  - `take_payload() -> Option<P>`
  - `complete(self, Result<R,E>)`
  - dropping without `complete` makes the item Abandoned
- `capability_model::CapabilitySet::satisfies(&Requirements) -> bool`. An empty requirement is always satisfied.
- `resource_requirements`:
  - `ResourceRequirements::{kind(), environment(), model() -> Option<&ModelId>, index() -> Option<&IndexId>, capabilities()}`
  - `ExecutionEnvironment::accepts(DeviceKind) -> bool`
  - `ResourceKind`: `Copy + Eq + Hash`, `non_exhaustive`
  - `ModelId`, `IndexId`: `Eq + Hash + Clone`
- `device::DeviceKind { Cpu, Gpu }` (`non_exhaustive`, `Copy + Eq`).
- `speicherverwaltung`:
  - `Speicher::new(Config) -> Result<Speicher, MemError>`
  - `Speicher::alloc(&self, Lifetime, size, Option<align>, StreamId) -> Result<Region, MemError>`
  - `Speicher::free(&self, Region, StreamId, Fence)`
  - `Speicher::complete(&self, StreamId, Fence)`
  - `Speicher` is `Send + Sync`
  - `Config { persistent_bytes, step_bytes, dynamic_bytes, max_streams: u32, max_dynamic_blocks, default_align }`
  - `StreamId(pub u32)` must be `< max_streams`
  - `Fence(pub u64)` is increasing per stream
  - `NotReady` means a different stream may not yet reuse the memory
  - The module never allocates VRAM itself. The backend resolves a `Region` to an address.