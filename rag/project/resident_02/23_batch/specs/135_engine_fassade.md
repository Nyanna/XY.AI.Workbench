# Engine-Fassade: Technical Specification (build order 135)

## 1. Task, restrictions and boundaries

**Task.** Implement the Rust module `engine`, the outward facade of the engine. It does the following:
- An `EngineBuilder` collects the configuration. It has no effect until `start()` is called.
- `start()` wires Processor, Planner, sub-engines, models, indexes and the worker pool, and returns a ready `Engine`.
- `Engine` accepts tasks (`enqueue`, `enqueue_embedding`, `enqueue_similarity`) and registers indexes under unique IDs without loading them.
- `Engine::shutdown` (also run by `Drop`) stops the workers, joins them, then cancels every task no worker took over.

**Restrictions.**
- The facade implements no planning, expansion, operation execution, resource loading, scheduling or queue logic. It only wires and delegates.
- It never calls `Subengine::announce` or `activate`, `ModelRegistry::load_encoder` or `encoder_resource`, or `Index` methods.
- It never claims or completes a `Task`. It only cancels tasks (see §4, Shutdown, and the plan-error rule).
- No `unsafe`, no globals. Crates: `log`, `std`.
- Out of scope: restart after shutdown, dynamic model/index/sub-engine changes after start, feeding `Planner::set_idle` (callers may use `Engine::planner()`).

## 2. Context

- **Processor** owns the task queue, the three operations queues. `submit(data, params)` pushes a task. `expand(&task)` calls the Planner and pushes the planned ops. It has no worker loop. It is constructed as `Processor::new(planner)`.
- **Planner** (`planner::Planner<D,T,E,V>`) implements `processor::Planner<D,T,E,OpMeta,V>`. It is configured through `PlannerBuilder`: sub-engines, models, default model, indexes. `Planner::start` starts the sub-engines. It is the only place they are started.
- **Worker-Pool** (`worker_pool::WorkerPool`) runs threads that call `WorkSource::run_once`. The facade supplies the `WorkSource` (the task dispatcher). The worker count is only the number of dispatcher threads. Sub-engines pull operations with their own lanes.
- **Subengine** (`planner::Subengine`, e.g. `ComputeSubengine`) pulls operations from the Processor's ops queues. Dropping the last `Arc` of a `ComputeSubengine` shuts it down.
- **Modellregister** (`Arc<ModelRegistry>`) is passed to the Planner. The model set is fixed before start.
- **Index contract** (`index_contract::Index`) is what registered indexes implement. The facade only stores them.
- **Queue** and **Queue state** are used for the task queue: `Queue<TaskParams, Task<D,T,E>, (), ()>`.
- The Queue-Iteration module is not used by the facade.

## 3. Interface

```rust
use std::sync::Arc;
use index_contract::Index;
use planner::{OpMeta, Planner, Subengine, TaskResolver, ValueBuilder, PlannerConfigError};
use processor::QueueKind;
use queue::Queue;
use resource_requirements::{IndexId, ResourceKind};
use model_descriptor::ModelDescriptor;
use model_registry::ModelRegistry;
use task_model::{Task, TaskParams};
use worker_pool::PoolConfig;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EngineError {
    /// "Konfigurationsfehler: Pflichtfeld '{field}' fehlt"
    Config { field: &'static str },
    /// "Index-ID bereits registriert: {0}"
    DuplicateIndex(String),
    /// Display of the inner error
    Planner(PlannerConfigError),
    /// "Subengine '{name}' konnte nicht erzeugt werden: {reason}"
    Subengine { name: String, reason: String },
    /// "Worker-Pool konnte nicht gestartet werden: {0}"
    Pool(String),
    /// "Engine ist heruntergefahren"
    ShutDown,
    /// "Shutdown fehlgeschlagen: {0}"
    ShutdownFailed(String),
}
// Display + std::error::Error.

/// Handle the factories use to reach the (Engine-owned) ops queues.
pub struct EngineWiring<D, T, E, V, R, OE> { /* private */ }
impl<D, T, E, V, R, OE> EngineWiring<D, T, E, V, R, OE> {
    /// Shareable handle to the Processor's ops queue of `kind`; the same instance on every call.
    pub fn ops_queue(&self, kind: QueueKind) -> Arc<Queue<OpMeta, V, R, OE>>;
}

pub type SubengineFactory<D, T, E, V, R, OE> =
    Box<dyn FnOnce(&EngineWiring<D, T, E, V, R, OE>) -> Result<Arc<dyn Subengine>, String> + Send>;

pub struct EngineBuilder<D, T, E, V, R, OE> { /* private; collects only, creates nothing */ }

impl<D, T, E, V, R, OE> EngineBuilder<D, T, E, V, R, OE>
where D: Send + 'static, T: Send + Sync + 'static, E: Send + Sync + 'static,
      V: Send + 'static, R: Send + 'static, OE: Send + 'static
{
    pub fn new() -> Self;
    /// Optional. Default: `ModelRegistry::with_builtin()`.
    pub fn registry(self, r: Arc<ModelRegistry>) -> Self;
    /// Required (at least one call). Adds a model to the fixed set. Validation happens in `start`.
    pub fn model(self, d: ModelDescriptor) -> Self;
    /// Optional. Default model = the first `model()` call.
    pub fn default_model(self, model_id: impl Into<String>) -> Self;
    /// Required. Passed to `PlannerBuilder::new`.
    pub fn task_resolver(self, r: Arc<dyn TaskResolver<D, T, E>>) -> Self;
    /// Required. Passed to `PlannerBuilder::new`.
    pub fn value_builder(self, v: Arc<dyn ValueBuilder<D, T, E, V>>) -> Self;
    /// Prebuilt sub-engine (the caller wired its queues). Configuration order is kept.
    pub fn subengine(self, s: Arc<dyn Subengine>) -> Self;
    /// Sub-engine created at `start` from the Engine's queues. `name` must equal `Subengine::name()`.
    pub fn subengine_with(self, name: impl Into<String>, f: SubengineFactory<D, T, E, V, R, OE>) -> Self;
    /// Registers a searchable index under a unique ID. Neither loads nor uses it.
    pub fn index(self, id: IndexId, index: Arc<dyn Index>) -> Self;
    /// Optional. Default: `PoolConfig { min_workers: 1, max_workers: None }`.
    pub fn workers(self, cfg: PoolConfig) -> Self;
    /// The explicit start call: validates, wires, initializes. Consumes the builder.
    pub fn start(self) -> Result<Engine<D, T, E, V, R, OE>, EngineError>;
}

pub struct Engine<D, T, E, V, R, OE> { /* private */ }
// Send + Sync when D: Send, T,E: Send+Sync, V,R,OE: Send. Drop calls shutdown().

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ShutdownReport { pub cancelled_tasks: usize }

impl<D, T, E, V, R, OE> Engine<D, T, E, V, R, OE> {
    pub fn builder() -> EngineBuilder<D, T, E, V, R, OE>;      // == EngineBuilder::new()

    /// Generic entry. Pushes the task into the task queue and wakes the dispatcher workers.
    pub fn enqueue(&self, data: D, params: TaskParams) -> Result<Task<D, T, E>, EngineError>;
    /// Embedding task. `data` must resolve (via the TaskResolver) to an Embedding task.
    pub fn enqueue_embedding(&self, data: D, params: TaskParams) -> Result<Task<D, T, E>, EngineError>;
    /// Similarity task. `data` must resolve to a Similarity task.
    pub fn enqueue_similarity(&self, data: D, params: TaskParams) -> Result<Task<D, T, E>, EngineError>;

    /// Registered index, None if the ID is unknown.
    pub fn index(&self, id: &IndexId) -> Option<Arc<dyn Index>>;
    /// Registered IDs in registration order.
    pub fn index_ids(&self) -> Vec<IndexId>;
    /// None if unknown. Some(false) directly after start. Later: any sub-engine reports `is_loaded(ResourceKind::Index, None)`.
    pub fn is_index_loaded(&self, id: &IndexId) -> Option<bool>;

    pub fn planner(&self) -> &Planner<D, T, E, V>;
    pub fn is_running(&self) -> bool;

    /// Ordered shutdown, idempotent, blocking. Ok = positive feedback.
    pub fn shutdown(&self) -> Result<ShutdownReport, EngineError>;
}
```

## 4. Behavior and invariants

**Builder and start**
- Builder methods only record values. Before `start()`, no thread, queue, Processor, Planner or sub-engine exists, and no factory is called.
- `start()` checks the required fields in this order. The first missing one returns `Err(EngineError::Config { field })`:
  1. `"model"` (no `model()` call),
  2. `"task_resolver"`,
  3. `"value_builder"`.
- Display of the missing-model error: `Konfigurationsfehler: Pflichtfeld 'model' fehlt`. The field name `model` appears in the message.
- Duplicate index IDs return `DuplicateIndex(id)`. This is checked before anything is created.
- Start sequence after validation. The first error aborts, and everything already created is dropped (sub-engines stop via Drop):
  1. Create `PlannerBuilder::new(registry, resolver, values)`. Add the models in call order, then the default model (the explicit one, else the first), the indexes (`.index(id)`), and the prebuilt sub-engines and deferred sub-engine wrappers in configuration order. `PlannerConfigError` becomes `Planner(e)`.
  2. Build the `Planner`, share it as `Arc`, and create the `Processor` through a thin forwarding newtype that implements `processor::Planner`.
  3. Run each `subengine_with` factory with the `EngineWiring`, in configuration order, and bind the result into its deferred wrapper. A factory error becomes `Subengine{name, reason}`. A wrapper's `name()` is the configured name. A wrapper forwards all `Subengine` methods to the bound instance. Before binding, `start()` and `activate()` return `Err("nicht gebunden")`. Binding happens before step 4, so the Planner never sees an unbound wrapper.
  4. `Planner::start()`. A failure becomes `Planner(StartFailed{..})`.
  5. `worker_pool::WorkerPool::start(cfg, dispatcher)`. A failure becomes `Pool(text)`.
- After a successful `start()` the engine is ready. `start` creates no model or device resources:
  - The facade calls neither `announce` nor `activate` nor any registry load.
  - Every `is_index_loaded` is `Some(false)`.
  - Resources are created only through the first task (Planner `expand` → `announce`/`activate`).

**Index registration**
- Each index is registered under its unique `IndexId`, and `index(id)` returns the registered `Arc`. No `Index` method is called during registration or start. Immediately after registration the index counts as not loaded.
- The same IDs are passed to the Planner (`PlannerBuilder::index`), so `engine.planner().indexes()` lists them.

**Enqueue**
- `enqueue` calls `Processor::submit(data, params)`, then `Notifier::notify()` of the Worker-Pool. It never blocks and returns the task handle in state `Enqueued` with the given params.
- `enqueue_embedding` and `enqueue_similarity` behave exactly like `enqueue`. The facade does not inspect `data`. The task kind is determined by the `TaskResolver` at expansion.
- After shutdown has begun, `enqueue*` returns `Err(ShutDown)` and submits nothing.
- Enqueues are guarded by a read lock on the `closed` flag, held across submit and notify. Shutdown sets the flag under the write lock. Therefore every task accepted before shutdown is either taken by a worker or cancelled by shutdown, and none is lost.

**Task dispatcher (`WorkSource`, internal, observable)**
- `run_once` does the following:
  1. `task_queue().take()`. If it returns `None`, the result is `Step::Idle`.
  2. Take the payload (the `Task` handle).
  3. If the task is already `Completed` or `Cancelled`, skip expansion. Otherwise call `Processor::expand(&task)` once and drop the tickets.
  4. If `expand` fails (`ExpandError::Planner`), log `warn!` and call `task.cancel()`. Waiters then get `Cancelled`.
  5. Always `claim.complete(Ok(()))`, then return `Step::Worked`.
- The dispatcher never claims or completes a `Task`. Claiming and completing is up to the injected components (ValueBuilder/operations).
- If `closed` is set, `run_once` returns `Idle` without taking anything.
- `expand` may block in `Subengine::activate`. This is backpressure and is not interrupted.

**Shutdown (strict order)**
1. Set `closed` (new `enqueue*` → `ShutDown`).
2. `WorkerPool::stop()`: stop the workers and join them. A running `run_once` finishes first.
3. Drain the task queue with `take()`. For every remaining item take the `Task`, call `task.cancel()` and `claim.complete(Ok(()))`. Waiting callers receive `TaskError::Cancelled`. Count them in `cancelled_tasks`. If cancelling a task panics, continue with the rest and return `Err(ShutdownFailed(text))` at the end.
4. Release the Engine's references to Processor, Planner and sub-engines (sub-engines owned only by the Engine shut down via their Drop).
- Success returns `Ok(ShutdownReport { cancelled_tasks })`. This is the positive feedback.
- `shutdown` is idempotent and thread-safe. Later or concurrent calls wait for the first to finish and return `Ok(ShutdownReport { cancelled_tasks: 0 })`.
- `Drop` runs the same sequence, ignores the result and logs failures with `warn!`.
- After shutdown `is_running() == false`. Task handles obtained earlier stay valid.

**Concurrency**
- All `Engine` methods take `&self` and are callable from any thread. Only `shutdown` (and `Drop`) and `start` block.
- No lock is held across `Processor::expand` or `Subengine` calls.

## 5. Relied-on elements

- `processor`:
  - `Processor::new(planner) -> Arc<Self>`.
  - `submit(&self, D, TaskParams) -> Task`: never blocks or fails.
  - `expand(&self, &Task) -> Result<Vec<(QueueKind, Ticket)>, ExpandError<PL::Error>>`: does not claim, complete or cancel the task.
  - `task_queue() -> &Queue<TaskParams, Task<D,T,E>, (), ()>`.
  - `ops_queue(QueueKind) -> &Queue<O,V,R,OE>`.
  - `processor::Planner<D,T,E,O,V>` trait with `type Error` and `expand`.
  - `QueueKind { Cpu, Gpu, Shared }`.
  - **Gap to resolve in the Processor/Engine wiring:** `EngineWiring::ops_queue` needs an `Arc` to the Processor's ops queues. The Processor spec exposes only `&OpsQueue`. The implementer must obtain the sharable `Arc` from the Processor (e.g. by having the Processor hold its queues in `Arc`s and expose them).
- `planner`:
  - `PlannerBuilder::new(Arc<ModelRegistry>, Arc<dyn TaskResolver>, Arc<dyn ValueBuilder>)`.
  - Builder methods `subengine(Arc<dyn Subengine>) -> Result`, `model(d) -> Result`, `default_model(&str) -> Result`, `index(IndexId)`, `build()`.
  - `Planner::start() -> Result<(), PlannerConfigError>`: the only place sub-engines are started.
  - `Planner::indexes()`, `resource_status`.
  - `PlannerConfigError` (`Debug, Clone, PartialEq, Eq, Display, Error`).
  - `OpMeta`.
  - `trait Subengine: Send + Sync { name, start, capabilities, offers, announce, activate, is_loaded(ResourceKind, Option<&ModelId>) }`.
  - Planner `expand` is called by the Processor, once per task. It blocks only in `activate`.
- `worker_pool`:
  - `WorkerPool::start(PoolConfig, Arc<dyn WorkSource>) -> Result<WorkerPool, PoolError>`.
  - `notifier() -> Notifier` with `notify()`.
  - `stop()`: idempotent, joins all workers, and the current `run_once` finishes first.
  - `Drop` calls `stop`.
  - `PoolConfig { min_workers, max_workers: Option<usize> }`.
  - `trait WorkSource: Send + Sync + 'static { fn run_once(&self, worker: usize) -> Step }`.
  - `Step::{Worked, Idle}`.
  - A panic in `run_once` is caught by the pool.
  - The pool does not wake on `Processor::submit`. The facade must call `Notifier::notify()` after each submit.
  - `queue`:
  - `Queue::take() -> Option<Claim>`.
  - `Claim::{take_payload(&mut self) -> Option<P>, complete(self, Result<R,E>)}`.
  - A dropped `Claim` becomes Abandoned.
  - Items are FIFO.
- `task_model`:
  - `Task<D,T,E>`: `Clone` handle.
  - `TaskParams { priority, resource, model, mode }`: `Clone + Default`.
  - `state()`, `cancel() -> bool` (true only from Enqueued; waiters then get `TaskError::Cancelled`).
  - `Task` is `Send + Sync` when `D: Send, T,E: Send+Sync`.
- `queue_state::QueueState { Enqueued, Claimed, Completed, Cancelled }`.
- `model_registry::ModelRegistry`: `with_builtin()`, shared as `Arc`.
- `model_descriptor::ModelDescriptor`: validated by the Planner (`InvalidModel`).
- `index_contract::Index`: `Send + Sync`, object-safe, used as `Arc<dyn Index>`. The facade only stores it.
- `resource_requirements`: `IndexId` (`Eq + Hash + Clone`, `as_str()`), `ResourceKind::Index`.