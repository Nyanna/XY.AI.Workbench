# Planner: Technical Specification (build order 126)

## 1. Task, restrictions and boundaries

**Task.** Implement the Rust module `planner`. It has these jobs:
- It is the `Planner` implementation that the Processor calls once per task. It resolves the task, has the Expander decompose it, and assigns each planned operation to the CPU, GPU or shared queue.
- It manages the sub-engines:
  - It starts them.
  - It collects their capability sets and resource offers.
  - It blocks until the sub-engines an operation needs are active.
  - It tells them which resources an operation needs.
- It sets the materialization flag per operation.
- It reports the resource kind of a task kind's first operation in advance.

**Restrictions.**
- The Planner never pushes into queues. It returns `PlannedOp`s, and the Processor pushes them. A failed `expand` therefore enqueues nothing.
- It never claims, completes or cancels tasks.
- It does not execute operations, load resources or run workers.
- It does not call the Expander's internals. It only calls `expander::expand`, or an injected `Expander`.
- It knows neither the external neural-network library nor GPU execution graphs (CUDA Graphs).
- It has no globals and no `unsafe`. Extra crate: `log`.
- It invents no task kinds beyond those the Expander supports (Embedding, Similarity).

## 2. Context

- **Processor** (`processor`): calls `Planner::expand(&Task)` once per task and pushes each returned `PlannedOp{kind: QueueKind, meta, values}` into the queue of its kind, preserving order. The trait contract is `expand(&self, &Task<D,T,E>) -> Result<Vec<PlannedOp<O,V>>, Self::Error>`. It must not block on the Processor's queues. Blocking sub-engine activation is allowed.
- **Expander** (`expander`): the pure function `expand(&TaskSpec, &PipelineConfigs, &CapabilitySet, Option<&IdleNotification>) -> Result<Vec<PlannedOperation>, ExpandError>`. The order of the chain is the implicit graph. Each `PlannedOperation` has `kind`, `descriptor`, `resource` (`ResourceRequirements`) and `config`. It builds no `Operation<R>`, and the Planner supplies that part. The Embedding chain is tokenizer → encoder → (pooling) → (L2) → (index write). Similarity is scorer+TopK, or cross-encoder tokenization, inference and similarity.
- **Sub-engines**: pending spec. The Planner relies only on the `Subengine` trait in §3, which it defines. Workers of a queue filter on the `OpMeta` of the items.
- **Engine**: pending spec. It configures the Planner with sub-engines, models and indexes. It calls `start` and feeds the Idle-Notification via `set_idle`.
- **Queues**: Cpu, Gpu and Shared, as `QueueKind` of the Processor.

## 3. Interface

```rust
use std::sync::Arc;
use capability_model::{CapabilitySet, Requirements};
use expander::{TaskSpec, PlannedOperation, PipelineConfig, ExpandError, IdleNotification, PipelineConfigs};
use model_descriptor::ModelDescriptor;
use model_registry::ModelRegistry;
use operation::{OperationDescriptor, OperationKind, TaskId};
use processor::{PlannedOp, QueueKind};
use resource_requirements::{IndexId, ModelId, ResourceKind, ResourceRequirements, ExecutionEnvironment};
use task_model::Task;

/// Which expander applies to (model, task kind).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TaskKindTag { Embedding, Similarity }

/// Contract with a sub-engine. All methods are callable from any thread.
pub trait Subengine: Send + Sync {
    fn name(&self) -> String;                                   // unique within one Planner
    /// Start (e.g. spawn workers). Idempotent. Called once by Planner::start.
    fn start(&self) -> Result<(), String>;
    /// Capability sets per queue. Called after a successful start.
    fn capabilities(&self) -> Vec<(QueueKind, CapabilitySet)>;
    /// Initial resource kinds offered (available = registered capability).
    fn offers(&self) -> Vec<ResourceKind>;
    /// Told which resources an operation needs. Non-blocking.
    fn announce(&self, req: &ResourceRequirements);
    /// Blocks until ready for operations. Idempotent.
    fn activate(&self) -> Result<(), String>;
    /// Loaded = initialized and in memory.
    fn is_loaded(&self, kind: ResourceKind, model: Option<&ModelId>) -> bool;
}

/// Decomposition for one (model, task kind). Pure.
pub trait Expander: Send + Sync {
    fn expand(&self, spec: &TaskSpec, ctx: &ExpansionContext<'_>) -> Result<Vec<PlannedOperation>, ExpandError>;
}
/// Calls `expander::expand(spec, ctx.pipelines, ctx.capabilities, Some(ctx.idle))`.
pub struct StandardExpander;
impl Expander for StandardExpander { /* ... */ }

/// Everything kept together for an expansion.
pub struct ExpansionContext<'a> {
    pub model: &'a ModelDescriptor,        // the chosen (default) model
    pub pipelines: &'a PipelineConfigs,    // key = model_id
    pub capabilities: &'a CapabilitySet,   // union of all sub-engine sets
    pub offers: &'a [(String, Vec<ResourceKind>)], // per sub-engine name
    pub idle: &'a IdleNotification,        // snapshot
}

/// Reads task kind and parameters from a task (the Planner cannot read task data).
pub trait TaskResolver<D, T, E>: Send + Sync {
    fn resolve(&self, task: &Task<D, T, E>) -> (TaskId, TaskSpec);
}

/// Turns the planned chain into queue payloads (e.g. Operation<R>/OperationNode wiring
/// of input/output between neighbours). Must return exactly one V per step, in order.
pub trait ValueBuilder<D, T, E, V>: Send + Sync {
    fn build(&self, task: &Task<D, T, E>, steps: &[PlanStep]) -> Vec<V>;
}

/// Queue metadata of an operation (the `O` of the Processor). Immutable.
#[derive(Clone, Debug)]
pub struct OpMeta {
    pub task_id: TaskId,
    pub priority: i32,                    // task.params().priority
    pub queue: QueueKind,
    pub kind: OperationKind,
    pub descriptor: OperationDescriptor,
    pub resource: ResourceRequirements,
    pub required: Requirements,           // == resource.capabilities().clone()
    pub materialize: bool,
}
#[derive(Clone, Debug)]
pub struct PlanStep { pub meta: OpMeta, pub planned: PlannedOperation }

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ResourceStatus { pub available: bool, pub loaded: bool }

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlanError {
    /// "Planner nicht gestartet"
    NotStarted,
    /// "unbekanntes Modell: {model}"
    UnknownModel { model: String },
    /// "fehlende Planungs-Capability: {kind:?} ({resource:?})"
    MissingPlanningCapability { kind: OperationKind, resource: ResourceKind },
    /// Display of the inner error
    Expand(ExpandError),
    /// "Subengine {subengine} nicht aktivierbar: {reason}"
    Activation { subengine: String, reason: String },
    /// ValueBuilder returned a different number of values than steps
    BuilderMismatch { expected: usize, got: usize },
}
// Display + std::error::Error. PlanError: Send + Sync + 'static.

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlannerConfigError {
    DuplicateSubengine(String),
    InvalidModel(String),              // descriptor.validate() failed or architecture not in ModelRegistry
    UnknownDefaultModel(String),
    UnknownSubengine(String),
    StartFailed { subengine: String, reason: String },
}
// Display + std::error::Error.

pub struct PlannerBuilder<D, T, E, V> { /* private */ }
impl<D, T, E, V> PlannerBuilder<D, T, E, V> {
    pub fn new(registry: Arc<ModelRegistry>,
               resolver: Arc<dyn TaskResolver<D, T, E>>,
               values: Arc<dyn ValueBuilder<D, T, E, V>>) -> Self;
    pub fn subengine(self, s: Arc<dyn Subengine>) -> Result<Self, PlannerConfigError>;
    /// Registers a model: validate() Ok and registry.supports_architecture(architecture).
    /// Pipeline config = PipelineConfig::from_descriptor, key = model_id.
    /// Also registers StandardExpander for (model_id, Embedding) and (model_id, Similarity).
    pub fn model(self, d: ModelDescriptor) -> Result<Self, PlannerConfigError>;
    /// Replaces/adds the expander of one combination (e.g. to remove none: use `without_expander`).
    pub fn expander(self, model_id: &str, kind: TaskKindTag, e: Arc<dyn Expander>) -> Self;
    pub fn without_expander(self, model_id: &str, kind: TaskKindTag) -> Self;
    pub fn index(self, id: IndexId) -> Self;
    pub fn default_model(self, model_id: &str) -> Result<Self, PlannerConfigError>; // must be configured
    pub fn build(self) -> Planner<D, T, E, V>;
}

pub struct Planner<D, T, E, V> { /* private */ }
// Send + Sync when D: Send, T,E: Send+Sync, V: Send.

impl<D, T, E, V> Planner<D, T, E, V> {
    /// Starts every configured sub-engine (in configuration order), then collects
    /// capabilities and offers. Idempotent after success. On the first failure returns
    /// StartFailed and the Planner stays not started.
    pub fn start(&self) -> Result<(), PlannerConfigError>;
    pub fn is_started(&self) -> bool;

    /// Extends a sub-engine's offers (stepwise extensible, idempotent).
    pub fn add_offer(&self, subengine: &str, kind: ResourceKind) -> Result<(), PlannerConfigError>;
    pub fn offers(&self, subengine: &str) -> Option<Vec<ResourceKind>>;
    /// Union of the capability sets of all sub-engines.
    pub fn capabilities(&self) -> CapabilitySet;
    /// Union of the capability sets assigned to one queue.
    pub fn queue_capabilities(&self, q: QueueKind) -> CapabilitySet;
    /// Replaces the Idle-Notification used by following expansions.
    pub fn set_idle(&self, idle: IdleNotification);
    pub fn indexes(&self) -> Vec<IndexId>;

    /// available = a sub-engine offers `kind`; loaded = a started sub-engine that offers it reports is_loaded.
    pub fn resource_status(&self, kind: ResourceKind, model: Option<&ModelId>) -> ResourceStatus;
    /// Resource kind of the first operation of this task kind, reported in advance.
    pub fn first_resource_kind(&self, spec: &TaskSpec) -> Option<ResourceKind>;
    pub fn first_resource_kind_of_task(&self, task: &Task<D, T, E>) -> Option<ResourceKind>;
}

impl<D, T, E, V> processor::Planner<D, T, E, OpMeta, V> for Planner<D, T, E, V> {
    type Error = PlanError;
    fn expand(&self, task: &Task<D, T, E>) -> Result<Vec<PlannedOp<OpMeta, V>>, PlanError>;
}
```

## 4. Behavior and invariants

**Configuration and start**
- A duplicate sub-engine name gives `DuplicateSubengine`. A model failing `validate()` or whose architecture is unsupported gives `InvalidModel(model_id)`. `default_model` of an unconfigured model gives `UnknownDefaultModel`.
- `build` does not start anything. `start` is the only place sub-engines are started, and nothing starts lazily.
- `start` calls `start()` on each sub-engine once. It then stores `capabilities()` per (sub-engine, queue) and `offers()`. `add_offer` before `start` is kept. `add_offer` for an unknown name gives `UnknownSubengine`.
- Offers only grow. Nothing removes them.
- `capabilities()` (the set given to the Expander) is the union over all queues and sub-engines.

**Resource status**
- *available* means registered in the offers. *loaded* means `Subengine::is_loaded` is true.
- loaded implies the sub-engine offers the kind; a kind that is not offered has `available == false` and `loaded == false`.
- Querying never activates or loads anything.

**First resource kind** (pure, and independent of `start`)
- Embedding → `Tokenizer`.
- Similarity with a vector scorer → `Scorer`.
- Similarity with a cross-encoder (inference) → `Tokenizer`.
- `TaskSpec::Other` → `None`.
- This equals the `resource.kind()` of the first operation the Expander produces for that kind.

**expand(task)**: strictly in this order. The first error wins, and an error leaves nothing pushed and nothing activated.
1. Not started → `NotStarted`.
2. `resolver.resolve(task)` gives `(task_id, spec)`. `TaskSpec::Other{kind}` → `Expand(ForeignTaskKind{expected:"Embedding/Similarity", got:kind})`.
3. Model: the chosen model is always the configured default model, whatever the model in the spec. The Planner replaces the model in the spec (`EmbeddingTask.model` or `SimilaritySpec.model`) with the default. A missing default, an unconfigured model, or no expander for (model, `TaskKindTag` of the spec) gives `UnknownModel{model}`. The message names the model.
4. The expander runs with a snapshot of the idle value and the pipelines. Its `ExpandError` is wrapped as `Expand(..)`.
5. Queue assignment for every operation, in chain order. Candidates depend on `resource.environment()`:
   - `GpuOnly` → [Gpu].
   - `CpuOnly` → [Cpu].
   - `Any` → [Shared, Gpu, Cpu]. The shared queue is preferred.
   
   A candidate queue `q` is compatible iff all of these hold:
   - `queue_capabilities(q).satisfies(resource.capabilities())`;
   - at least one sub-engine assigned to `q` offers `resource.kind()`.
   
   The first compatible candidate wins. If none is compatible, the result is `MissingPlanningCapability{kind, resource}`. Every operation needs a queue. No partial result is returned.
6. Materialization (see below).
7. For the whole chain, each operation announces itself to the sub-engines that are assigned to its queue and offer its kind, via `announce(&resource)`. Then the Planner activates the deduplicated set of those sub-engines with `activate()`, blocking and one after another. The first failure gives `Activation{subengine, reason}`. Activation is idempotent and happens only when an operation is created, never at `start` or `build`.
8. `ValueBuilder::build(task, steps)` runs. A wrong number of values gives `BuilderMismatch`.
9. The result is one `PlannedOp{kind: queue, meta, values}` per step, in chain order. `meta.priority` equals `task.params().priority`. `meta.descriptor` and `meta.resource` are the `PlannedOperation` values unchanged. `meta.required == resource.capabilities()`.

**Materialization** (`meta.materialize`)
- It is `false` only if both of these hold:
  - this operation and the immediately following one both have `environment() == GpuOnly`;
  - the following operation is not an `IndexWrite`.
- In all other cases it is `true`: the last operation (task end), a successor on `CpuOnly` or `Any`, a successor `IndexWrite` (mirroring into the index), or this operation itself on `CpuOnly` or `Any`.
- It is decided from the Expander's environments before queue assignment, and queue choice does not change it.

**Concurrency**
- `expand` and all accessors take `&self` and can run concurrently from any thread. State sits behind short-lived locks. No lock is held across `activate`, `announce`, the expander or the `ValueBuilder`.
- `set_idle` affects only expansions that start after it. A running expansion keeps its snapshot.
- `expand` blocks only in `Subengine::activate`. It never touches queues.
- Equal task, configuration, idle value and offers give equal `meta` chains.

**Errors**: no panic on valid use. The errors are `PlanError` (expand) and `PlannerConfigError` (configuration and start). A panic inside injected components propagates. Nothing is pushed in that case.

## 5. Relied-on elements

- `processor::Planner<D,T,E,O,V>`: `type Error: std::error::Error + Send + Sync + 'static; fn expand(&self, &Task<D,T,E>) -> Result<Vec<PlannedOp<O,V>>, Self::Error>`. `PlannedOp<O,V>{kind: QueueKind, meta: O, values: V}`. `QueueKind{Cpu, Gpu, Shared}`. The Processor pushes in the returned order, and an `Err` pushes nothing.
- `task_model`: `Task<D,T,E>` is a clone handle. `params() -> &TaskParams{priority:i32, resource, model, mode}`. The Planner never reads the task data.
- `capability_model`: `CapabilitySet` (`new`, `add`, `satisfies(&Requirements) -> bool`, `Extend<(Axis,Value)>`, `iter`, `Clone`, `Default`). An empty requirement is always satisfied.
- `resource_requirements`:
  - `ResourceRequirements::{kind(), environment(), model() -> Option<&ModelId>, index(), capabilities() -> &Requirements}`.
  - `ResourceKind` and `ExecutionEnvironment` are `non_exhaustive`, `Copy + Eq + Hash`. Handle unknown variants with a wildcard.
  - `ExecutionEnvironment::{CpuOnly, GpuOnly, Any}`.
  - `ModelId::new/as_str`, `IndexId::new/as_str`.
- `expander`: the signatures of `expand`, `TaskSpec`, `PlannedOperation{kind, descriptor, resource, config}`, `PipelineConfig::from_descriptor`, `PipelineConfigs = HashMap<String, PipelineConfig>`, `IdleNotification{idle_devices}`, `ExpandError` as specified there. Chain order is the data flow. The environments are fixed by the Expander (the encoder, pooling and L2 operations follow the idle-device rule).
- `operation`: `OperationKind` (`non_exhaustive`, `Copy + Debug`), `OperationDescriptor` (`Clone`), `TaskId(pub u64)`.
- `model_descriptor`: `ModelDescriptor::validate() -> Result<(), ModelConfigError>`, with the fields `model_id` and `architecture`.
- `model_registry`: `ModelRegistry::supports_architecture(&str) -> bool`, shared as `Arc<ModelRegistry>`. The Planner does not load encoders itself.
- `resource_lifecycle`: loaded (`Ready`, in memory) differs from available (registered). The Planner takes the loaded flag only from `Subengine::is_loaded` and drives no lifecycle.
- `queue`: the Planner uses no queue API. It only produces `QueueKind` assignments.