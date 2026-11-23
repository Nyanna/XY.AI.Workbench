//! Task planning: resolves a task, has the expander decompose it, assigns every operation to a
//! queue and manages the sub-engines. It never pushes into queues.
use std::collections::HashMap;
use std::fmt;
use std::sync::{
    Arc, Mutex, MutexGuard, PoisonError, RwLock, RwLockReadGuard, RwLockWriteGuard,
};
use crate::resident::l130::l120::l110::a::processor::{PlannedOp, QueueKind, TaskPlanner};
use crate::resident::l130::l120::l110::l100::l090::a::lane_batching::{
    BatchKey, BatchMeta,
};
use crate::resident::l130::l120::l110::l100::l090::a::model_registry::ModelRegistry;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::a::expander::{
    self, ExpandError, IdleNotification, PipelineConfig, PipelineConfigs,
    PlannedOperation, ScorerChoice, TaskSpec,
};
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::a::operation_node::HasOperationKind;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::a::resource_requirements::{
    ExecutionEnvironment, HasResourceRequirements, IndexId, ModelId, ResourceKind,
    ResourceRequirements,
};
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::b::operation::{
    OperationDescriptor, OperationKind, TaskId,
};
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::a::model_descriptor::ModelDescriptor;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::b::task_model::{
    ClaimError, Task,
};
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::l010::capability_model::{
    CapabilitySet, Requirements,
};
const EXPECTED_TASK_KINDS: &str = "Embedding/Similarity";
/// Which expander applies to (model, task kind).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TaskKindTag {
    Embedding,
    Similarity,
}
/// Contract with a sub-engine. All methods are callable from any thread.
pub trait Subengine: Send + Sync {
    /// Unique within one planner.
    fn name(&self) -> String;
    /// Starts the sub-engine (e.g. spawns workers). Idempotent.
    fn start(&self) -> Result<(), String>;
    /// Capability sets per queue. Called after a successful start.
    fn capabilities(&self) -> Vec<(QueueKind, CapabilitySet)>;
    /// Initial resource kinds offered.
    fn offers(&self) -> Vec<ResourceKind>;
    /// Tells which resources an operation needs. Non-blocking.
    fn announce(&self, req: &ResourceRequirements);
    /// Blocks until the sub-engine is ready for operations. Idempotent.
    fn activate(&self) -> Result<(), String>;
    /// Loaded means initialized and in memory.
    fn is_loaded(&self, kind: ResourceKind, model: Option<&ModelId>) -> bool;
}
/// Everything kept together for one expansion.
pub struct ExpansionContext<'a> {
    pub model: &'a ModelDescriptor,
    pub pipelines: &'a PipelineConfigs,
    pub capabilities: &'a CapabilitySet,
    pub offers: &'a [(String, Vec<ResourceKind>)],
    pub idle: &'a IdleNotification,
}
/// Decomposition for one (model, task kind). Pure.
pub trait Expander: Send + Sync {
    fn expand(
        &self,
        spec: &TaskSpec,
        ctx: &ExpansionContext<'_>,
    ) -> Result<Vec<PlannedOperation>, ExpandError>;
}
/// Delegates to `expander::expand`.
pub struct StandardExpander;
impl Expander for StandardExpander {
    fn expand(
        &self,
        spec: &TaskSpec,
        ctx: &ExpansionContext<'_>,
    ) -> Result<Vec<PlannedOperation>, ExpandError> {
        expander::expand(spec, ctx.pipelines, ctx.capabilities, Some(ctx.idle))
    }
}
/// Reads task kind and parameters from the claimed task data.
pub trait TaskResolver<D, T, E>: Send + Sync {
    fn resolve(&self, task: &Task<D, T, E>, data: &D) -> (TaskId, TaskSpec);
}
/// Turns the planned chain into the payload of its first step; the payload carries the other
/// steps.
pub trait ValueBuilder<D, T, E, V>: Send + Sync {
    /// Also returns the batch information of the first step's payload (`None`: not batchable).
    fn build(
        &self,
        task: &Task<D, T, E>,
        data: D,
        spec: &TaskSpec,
        steps: &[PlanStep],
    ) -> (V, Option<BatchInfo>);
}
/// Batch key and untruncated row lengths of a payload.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BatchInfo {
    pub key: BatchKey,
    pub seq_lengths: Vec<usize>,
}
/// Queue metadata of an operation. Immutable after the push.
#[derive(Clone, Debug)]
pub struct OpMeta {
    pub task_id: TaskId,
    pub priority: i32,
    pub queue: QueueKind,
    pub descriptor: OperationDescriptor,
    pub materialize: bool,
    /// `None` when planned; set from the payload before the push.
    pub batch: Option<BatchInfo>,
}
impl OpMeta {
    pub fn kind(&self) -> OperationKind {
        self.descriptor.kind()
    }
    pub fn resource(&self) -> &ResourceRequirements {
        self.descriptor.requirements()
    }
    pub fn required(&self) -> &Requirements {
        self.resource().capabilities()
    }
}
impl HasResourceRequirements for OpMeta {
    fn resource_requirements(&self) -> &ResourceRequirements {
        self.resource()
    }
}
impl HasOperationKind for OpMeta {
    fn operation_kind(&self) -> OperationKind {
        self.kind()
    }
}
impl BatchMeta for OpMeta {
    fn batch_key(&self) -> Option<BatchKey> {
        self.batch.as_ref().map(|b| b.key.clone())
    }
    fn seq_lengths(&self) -> &[usize] {
        self.batch.as_ref().map_or(&[], |b| b.seq_lengths.as_slice())
    }
}
#[derive(Clone, Debug)]
pub struct PlanStep {
    pub meta: OpMeta,
    pub planned: PlannedOperation,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ResourceStatus {
    pub available: bool,
    pub loaded: bool,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlanError {
    NotStarted,
    UnknownModel { model: String },
    MissingPlanningCapability { kind: OperationKind, resource: ResourceKind },
    Expand(ExpandError),
    Activation { subengine: String, reason: String },
    /// Cancelled or already claimed.
    TaskNotClaimable(ClaimError),
    /// The expander returned no operation at all.
    EmptyChain,
}
impl fmt::Display for PlanError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PlanError::NotStarted => write!(f, "Planner nicht gestartet"),
            PlanError::UnknownModel { model } => write!(f, "unbekanntes Modell: {model}"),
            PlanError::MissingPlanningCapability { kind, resource } => {
                write!(f, "fehlende Planungs-Capability: {kind:?} ({resource:?})")
            }
            PlanError::Expand(e) => write!(f, "{e}"),
            PlanError::Activation { subengine, reason } => {
                write!(f, "Subengine {subengine} nicht aktivierbar: {reason}")
            }
            PlanError::TaskNotClaimable(e) => {
                write!(f, "Task nicht beanspruchbar: {e:?}")
            }
            PlanError::EmptyChain => write!(f, "leere Operationskette"),
        }
    }
}
impl std::error::Error for PlanError {}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlannerConfigError {
    DuplicateSubengine(String),
    /// `validate()` failed or the architecture is not in the model registry.
    InvalidModel(String),
    UnknownDefaultModel(String),
    UnknownSubengine(String),
    StartFailed { subengine: String, reason: String },
}
impl fmt::Display for PlannerConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PlannerConfigError::DuplicateSubengine(n) => {
                write!(f, "doppeltes Subengine: {n}")
            }
            PlannerConfigError::InvalidModel(m) => write!(f, "ungültiges Modell: {m}"),
            PlannerConfigError::UnknownDefaultModel(m) => {
                write!(f, "unbekanntes Standardmodell: {m}")
            }
            PlannerConfigError::UnknownSubengine(n) => {
                write!(f, "unbekanntes Subengine: {n}")
            }
            PlannerConfigError::StartFailed { subengine, reason } => {
                write!(f, "Subengine {subengine} nicht startbar: {reason}")
            }
        }
    }
}
impl std::error::Error for PlannerConfigError {}
fn lock<T>(m: &Mutex<T>) -> MutexGuard<'_, T> {
    m.lock().unwrap_or_else(PoisonError::into_inner)
}
fn read<T>(l: &RwLock<T>) -> RwLockReadGuard<'_, T> {
    l.read().unwrap_or_else(PoisonError::into_inner)
}
fn write<T>(l: &RwLock<T>) -> RwLockWriteGuard<'_, T> {
    l.write().unwrap_or_else(PoisonError::into_inner)
}
fn union_into(target: &mut CapabilitySet, source: &CapabilitySet) {
    target.extend(source.iter().map(|(a, v)| (a.clone(), v.clone())));
}
fn push_unique<T: PartialEq>(list: &mut Vec<T>, item: T) {
    if !list.contains(&item) {
        list.push(item);
    }
}
fn candidate_queues(environment: ExecutionEnvironment) -> &'static [QueueKind] {
    if environment == ExecutionEnvironment::GpuOnly {
        &[QueueKind::Gpu]
    } else if environment == ExecutionEnvironment::CpuOnly {
        &[QueueKind::Cpu]
    } else {
        &[QueueKind::Shared, QueueKind::Gpu, QueueKind::Cpu]
    }
}
/// An operation keeps its device-side output only if it and its successor are GPU-only and the
/// successor does not write into an index.
fn materialize_flag(op: &PlannedOperation, next: Option<&PlannedOperation>) -> bool {
    let Some(next) = next else {
        return true;
    };
    let both_gpu = op.resource().environment() == ExecutionEnvironment::GpuOnly
        && next.resource().environment() == ExecutionEnvironment::GpuOnly;
    !(both_gpu && next.kind() != OperationKind::IndexWrite)
}
fn make_meta(
    task_id: TaskId,
    priority: i32,
    queue: QueueKind,
    op: &PlannedOperation,
    materialize: bool,
) -> OpMeta {
    OpMeta {
        task_id,
        priority,
        queue,
        descriptor: op.descriptor().clone(),
        materialize,
        batch: None,
    }
}
fn with_model(spec: TaskSpec, model: &str) -> TaskSpec {
    match spec {
        TaskSpec::Embedding { mut task, target_index } => {
            task.model = ModelId::new(model);
            TaskSpec::Embedding {
                task,
                target_index,
            }
        }
        TaskSpec::Similarity(mut similarity) => {
            similarity.model = ModelId::new(model);
            TaskSpec::Similarity(similarity)
        }
        other => other,
    }
}
fn requested_model(spec: &TaskSpec) -> String {
    match spec {
        TaskSpec::Embedding { task, .. } => task.model.as_str().to_string(),
        TaskSpec::Similarity(similarity) => similarity.model.as_str().to_string(),
        TaskSpec::Other { .. } => String::new(),
    }
}
#[derive(Default)]
struct EngineState {
    capabilities: Vec<(QueueKind, CapabilitySet)>,
    offers: Vec<ResourceKind>,
}
#[derive(Default)]
struct State {
    started: bool,
    engines: Vec<EngineState>,
    capabilities: CapabilitySet,
    queue_capabilities: HashMap<QueueKind, CapabilitySet>,
}
struct EngineView {
    name: String,
    engine: Arc<dyn Subengine>,
    queues: Vec<QueueKind>,
    offers: Vec<ResourceKind>,
}
/// Immutable copy of the planner state; used so that no lock is held while calling out.
struct View {
    started: bool,
    engines: Vec<EngineView>,
    capabilities: CapabilitySet,
    queue_capabilities: HashMap<QueueKind, CapabilitySet>,
}
impl View {
    fn assign(&self, resource: &ResourceRequirements) -> Option<QueueKind> {
        candidate_queues(resource.environment())
            .iter()
            .copied()
            .find(|q| self.compatible(*q, resource))
    }
    fn compatible(&self, queue: QueueKind, resource: &ResourceRequirements) -> bool {
        let satisfied = self
            .queue_capabilities
            .get(&queue)
            .is_some_and(|caps| caps.satisfies(resource.capabilities()));
        satisfied && self.targets(queue, resource.kind()).next().is_some()
    }
    fn targets(
        &self,
        queue: QueueKind,
        kind: ResourceKind,
    ) -> impl Iterator<Item = &EngineView> {
        self.engines
            .iter()
            .filter(move |e| e.queues.contains(&queue) && e.offers.contains(&kind))
    }
    /// Announces every placed resource to its target sub-engines; returns those sub-engines
    /// without duplicates, in order of first appearance.
    fn announce<'a>(
        &self,
        placed: impl Iterator<Item = (QueueKind, &'a ResourceRequirements)>,
    ) -> Vec<(String, Arc<dyn Subengine>)> {
        let mut targets: Vec<(String, Arc<dyn Subengine>)> = Vec::new();
        for (queue, resource) in placed {
            for engine in self.targets(queue, resource.kind()) {
                engine.engine.announce(resource);
                if !targets.iter().any(|(name, _)| *name == engine.name) {
                    targets.push((engine.name.clone(), engine.engine.clone()));
                }
            }
        }
        targets
    }
}
pub struct PlannerBuilder<D, T, E, V> {
    registry: Arc<ModelRegistry>,
    resolver: Arc<dyn TaskResolver<D, T, E>>,
    values: Arc<dyn ValueBuilder<D, T, E, V>>,
    engines: Vec<(String, Arc<dyn Subengine>)>,
    models: HashMap<String, ModelDescriptor>,
    pipelines: PipelineConfigs,
    expanders: HashMap<(String, TaskKindTag), Arc<dyn Expander>>,
    indexes: Vec<IndexId>,
    default_model: Option<String>,
}
impl<D, T, E, V> PlannerBuilder<D, T, E, V> {
    pub fn new(
        registry: Arc<ModelRegistry>,
        resolver: Arc<dyn TaskResolver<D, T, E>>,
        values: Arc<dyn ValueBuilder<D, T, E, V>>,
    ) -> Self {
        Self {
            registry,
            resolver,
            values,
            engines: Vec::new(),
            models: HashMap::new(),
            pipelines: PipelineConfigs::new(),
            expanders: HashMap::new(),
            indexes: Vec::new(),
            default_model: None,
        }
    }
    pub fn subengine(
        mut self,
        s: Arc<dyn Subengine>,
    ) -> Result<Self, PlannerConfigError> {
        let name = s.name();
        if self.engines.iter().any(|(n, _)| *n == name) {
            return Err(PlannerConfigError::DuplicateSubengine(name));
        }
        self.engines.push((name, s));
        Ok(self)
    }
    /// Registers a model together with its pipeline configuration and the standard expanders.
    pub fn model(mut self, d: ModelDescriptor) -> Result<Self, PlannerConfigError> {
        if d.validate().is_err() || !self.registry.supports_architecture(&d.architecture)
        {
            return Err(PlannerConfigError::InvalidModel(d.model_id));
        }
        let id = d.model_id.clone();
        self.pipelines.insert(id.clone(), PipelineConfig::from_descriptor(&d));
        for tag in [TaskKindTag::Embedding, TaskKindTag::Similarity] {
            self.expanders.insert((id.clone(), tag), Arc::new(StandardExpander));
        }
        self.models.insert(id, d);
        Ok(self)
    }
    pub fn expander(
        mut self,
        model_id: &str,
        kind: TaskKindTag,
        e: Arc<dyn Expander>,
    ) -> Self {
        self.expanders.insert((model_id.to_string(), kind), e);
        self
    }
    pub fn without_expander(mut self, model_id: &str, kind: TaskKindTag) -> Self {
        self.expanders.remove(&(model_id.to_string(), kind));
        self
    }
    pub fn index(mut self, id: IndexId) -> Self {
        push_unique(&mut self.indexes, id);
        self
    }
    pub fn default_model(mut self, model_id: &str) -> Result<Self, PlannerConfigError> {
        if !self.models.contains_key(model_id) {
            return Err(PlannerConfigError::UnknownDefaultModel(model_id.to_string()));
        }
        self.default_model = Some(model_id.to_string());
        Ok(self)
    }
    /// Starts nothing.
    pub fn build(self) -> Planner<D, T, E, V> {
        let state = State {
            engines: self.engines.iter().map(|_| EngineState::default()).collect(),
            ..State::default()
        };
        Planner {
            resolver: self.resolver,
            values: self.values,
            engines: self.engines,
            models: self.models,
            pipelines: self.pipelines,
            expanders: self.expanders,
            indexes: self.indexes,
            default_model: self.default_model,
            start_lock: Mutex::new(()),
            state: RwLock::new(state),
            idle: Mutex::new(IdleNotification::default()),
        }
    }
}
pub struct Planner<D, T, E, V> {
    resolver: Arc<dyn TaskResolver<D, T, E>>,
    values: Arc<dyn ValueBuilder<D, T, E, V>>,
    engines: Vec<(String, Arc<dyn Subengine>)>,
    models: HashMap<String, ModelDescriptor>,
    pipelines: PipelineConfigs,
    expanders: HashMap<(String, TaskKindTag), Arc<dyn Expander>>,
    indexes: Vec<IndexId>,
    default_model: Option<String>,
    start_lock: Mutex<()>,
    state: RwLock<State>,
    idle: Mutex<IdleNotification>,
}
impl<D, T, E, V> Planner<D, T, E, V> {
    /// Starts every sub-engine in configuration order, then collects capabilities and offers.
    /// Idempotent after success; on the first failure the planner stays not started.
    pub fn start(&self) -> Result<(), PlannerConfigError> {
        let _serial = lock(&self.start_lock);
        if self.is_started() {
            return Ok(());
        }
        for (name, engine) in &self.engines {
            engine
                .start()
                .map_err(|reason| PlannerConfigError::StartFailed {
                    subengine: name.clone(),
                    reason,
                })?;
        }
        let collected: Vec<(Vec<(QueueKind, CapabilitySet)>, Vec<ResourceKind>)> = self
            .engines
            .iter()
            .map(|(_, engine)| (engine.capabilities(), engine.offers()))
            .collect();
        let mut state = write(&self.state);
        let mut capabilities = CapabilitySet::new();
        let mut queue_capabilities: HashMap<QueueKind, CapabilitySet> = HashMap::new();
        for (entry, (caps, offers)) in state.engines.iter_mut().zip(collected) {
            for (queue, set) in &caps {
                union_into(&mut capabilities, set);
                union_into(queue_capabilities.entry(*queue).or_default(), set);
            }
            entry.capabilities = caps;
            for kind in offers {
                push_unique(&mut entry.offers, kind);
            }
        }
        state.capabilities = capabilities;
        state.queue_capabilities = queue_capabilities;
        state.started = true;
        log::debug!("planner started with {} sub-engines", self.engines.len());
        Ok(())
    }
    pub fn is_started(&self) -> bool {
        read(&self.state).started
    }
    /// Extends a sub-engine's offers. Offers only grow.
    pub fn add_offer(
        &self,
        subengine: &str,
        kind: ResourceKind,
    ) -> Result<(), PlannerConfigError> {
        let index = self
            .engine_index(subengine)
            .ok_or_else(|| PlannerConfigError::UnknownSubengine(subengine.to_string()))?;
        push_unique(&mut write(&self.state).engines[index].offers, kind);
        Ok(())
    }
    pub fn offers(&self, subengine: &str) -> Option<Vec<ResourceKind>> {
        let index = self.engine_index(subengine)?;
        Some(read(&self.state).engines[index].offers.clone())
    }
    /// Union of the capability sets of all sub-engines.
    pub fn capabilities(&self) -> CapabilitySet {
        read(&self.state).capabilities.clone()
    }
    /// Union of the capability sets assigned to one queue.
    pub fn queue_capabilities(&self, q: QueueKind) -> CapabilitySet {
        read(&self.state).queue_capabilities.get(&q).cloned().unwrap_or_default()
    }
    /// Replaces the idle notification used by expansions that start afterwards.
    pub fn set_idle(&self, idle: IdleNotification) {
        *lock(&self.idle) = idle;
    }
    pub fn indexes(&self) -> Vec<IndexId> {
        self.indexes.clone()
    }
    /// Available: a sub-engine offers `kind`. Loaded: a sub-engine of a started planner that
    /// offers it reports it loaded. Never activates or loads anything.
    pub fn resource_status(
        &self,
        kind: ResourceKind,
        model: Option<&ModelId>,
    ) -> ResourceStatus {
        let view = self.view();
        let offering: Vec<&EngineView> = view
            .engines
            .iter()
            .filter(|e| e.offers.contains(&kind))
            .collect();
        let loaded = view.started
            && offering.iter().any(|e| e.engine.is_loaded(kind, model));
        ResourceStatus {
            available: !offering.is_empty(),
            loaded,
        }
    }
    /// Resource kind of the first operation of this task kind. Pure.
    pub fn first_resource_kind(&self, spec: &TaskSpec) -> Option<ResourceKind> {
        match spec {
            TaskSpec::Embedding { .. } => Some(ResourceKind::Tokenizer),
            TaskSpec::Similarity(similarity) => {
                match similarity.scorer {
                    ScorerChoice::Vector(_) => Some(ResourceKind::Scorer),
                    ScorerChoice::CrossEncoder(_) => Some(ResourceKind::Tokenizer),
                }
            }
            TaskSpec::Other { .. } => None,
        }
    }
    /// Placement of one step created at run time. Announces the resource; never blocks and
    /// never activates.
    pub fn place_step(
        &self,
        task_id: TaskId,
        priority: i32,
        op: &PlannedOperation,
        next: Option<&PlannedOperation>,
    ) -> Result<OpMeta, PlanError> {
        let view = self.view();
        if !view.started {
            return Err(PlanError::NotStarted);
        }
        let queue = Self::queue_for(&view, op)?;
        let meta = make_meta(task_id, priority, queue, op, materialize_flag(op, next));
        view.announce(std::iter::once((queue, meta.resource())));
        Ok(meta)
    }
    fn engine_index(&self, name: &str) -> Option<usize> {
        self.engines.iter().position(|(n, _)| n == name)
    }
    fn view(&self) -> View {
        let state = read(&self.state);
        let engines = self
            .engines
            .iter()
            .zip(&state.engines)
            .map(|((name, engine), entry)| {
                let mut queues = Vec::new();
                for (queue, _) in &entry.capabilities {
                    push_unique(&mut queues, *queue);
                }
                EngineView {
                    name: name.clone(),
                    engine: engine.clone(),
                    queues,
                    offers: entry.offers.clone(),
                }
            })
            .collect();
        View {
            started: state.started,
            engines,
            capabilities: state.capabilities.clone(),
            queue_capabilities: state.queue_capabilities.clone(),
        }
    }
    fn queue_for(view: &View, op: &PlannedOperation) -> Result<QueueKind, PlanError> {
        view.assign(op.resource())
            .ok_or(PlanError::MissingPlanningCapability {
                kind: op.kind(),
                resource: op.resource().kind(),
            })
    }
    fn plan_steps(
        view: &View,
        chain: Vec<PlannedOperation>,
        task_id: TaskId,
        priority: i32,
    ) -> Result<Vec<PlanStep>, PlanError> {
        let flags: Vec<bool> = (0..chain.len())
            .map(|i| materialize_flag(&chain[i], chain.get(i + 1)))
            .collect();
        let mut steps = Vec::with_capacity(chain.len());
        for (planned, materialize) in chain.into_iter().zip(flags) {
            let queue = Self::queue_for(view, &planned)?;
            steps
                .push(PlanStep {
                    meta: make_meta(task_id, priority, queue, &planned, materialize),
                    planned,
                });
        }
        Ok(steps)
    }
    fn plan_task(
        &self,
        task: &Task<D, T, E>,
    ) -> Result<Vec<PlannedOp<OpMeta, V>>, PlanError> {
        let view = self.view();
        if !view.started {
            return Err(PlanError::NotStarted);
        }
        let data = task.claim().map_err(PlanError::TaskNotClaimable)?;
        let (task_id, spec) = self.resolver.resolve(task, &data);
        let tag = match &spec {
            TaskSpec::Embedding { .. } => TaskKindTag::Embedding,
            TaskSpec::Similarity(_) => TaskKindTag::Similarity,
            TaskSpec::Other { kind } => {
                return Err(
                    PlanError::Expand(ExpandError::ForeignTaskKind {
                        expected: EXPECTED_TASK_KINDS.to_string(),
                        got: kind.clone(),
                    }),
                );
            }
        };
        let Some(model_id) = self.default_model.clone() else {
            return Err(PlanError::UnknownModel {
                model: requested_model(&spec),
            });
        };
        let unknown = || PlanError::UnknownModel {
            model: model_id.clone(),
        };
        let descriptor = self.models.get(&model_id).ok_or_else(unknown)?;
        let expander = self
            .expanders
            .get(&(model_id.clone(), tag))
            .ok_or_else(unknown)?
            .clone();
        let spec = with_model(spec, &model_id);
        let idle = lock(&self.idle).clone();
        let offers: Vec<(String, Vec<ResourceKind>)> = view
            .engines
            .iter()
            .map(|e| (e.name.clone(), e.offers.clone()))
            .collect();
        let ctx = ExpansionContext {
            model: descriptor,
            pipelines: &self.pipelines,
            capabilities: &view.capabilities,
            offers: &offers,
            idle: &idle,
        };
        let chain = expander.expand(&spec, &ctx).map_err(PlanError::Expand)?;
        if chain.is_empty() {
            return Err(PlanError::EmptyChain);
        }
        let priority = task.params().priority;
        let steps = Self::plan_steps(&view, chain, task_id, priority)?;
        let targets = view
            .announce(steps.iter().map(|s| (s.meta.queue, s.meta.resource())));
        for (name, engine) in &targets {
            engine
                .activate()
                .map_err(|reason| PlanError::Activation {
                    subengine: name.clone(),
                    reason,
                })?;
        }
        let (values, batch) = self.values.build(task, data, &spec, &steps);
        let mut meta = steps[0].meta.clone();
        meta.batch = batch;
        log::debug!(
            "planned task {}: {} operations, first on {:?}", task_id.0, steps.len(), meta
            .queue
        );
        Ok(vec![PlannedOp { kind : meta.queue, meta, values }])
    }
}
impl<D, T, E, V> TaskPlanner<D, T, E, OpMeta, V> for Planner<D, T, E, V> {
    type Error = PlanError;
    fn expand(
        &self,
        task: &Task<D, T, E>,
    ) -> Result<Vec<PlannedOp<OpMeta, V>>, PlanError> {
        self.plan_task(task)
    }
}
#[cfg(test)]
#[path = "test/planner.rs"]
mod tests;
