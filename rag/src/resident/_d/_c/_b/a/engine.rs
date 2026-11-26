//! Outward facade of the engine: collects the configuration, wires Processor, Planner, sub-engines
//! and worker pool, accepts tasks and shuts everything down in order. It only wires and delegates.
use std::fmt;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, OnceLock, PoisonError, RwLock, Weak};
use std::time::Instant;
use crate::resident::_d::_c::_b::a::backpressure::ExpansionControl;
use crate::resident::_d::_c::_b::_a::_9::a::gpu_completion::GpuCompletion;
use crate::resident::_d::_c::_b::a::processor::{ExpandError3, Processor};
use crate::resident::_d::_c::_b::_a::a::plan_contract::{
    PlannedOp, QueueKind, TaskPlanner,
};
use crate::resident::_d::_c::_b::a::subengine::SubengineHandle;
use crate::resident::_d::_c::_b::a::worker_loop::{
    ExpandStep, FinishedHandler, WorkerLoop,
};
use crate::resident::_d::_c::_b::a::worker_pool::{
    Notifier3, PoolConfig, Step2, WorkSource, WorkerPool,
};
use crate::resident::_d::_c::_b::_a::a::planner::{
    BatchInfo, OpMeta, PlanError, Planner, PlannerBuilder, PlannerConfigError, Subengine,
    TaskResolver, ValueBuilder,
};
use crate::resident::_d::_c::_b::_a::_9::a::model_registry::ModelRegistry;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::a::notification_topology::{
    NotificationTopology, SubengineNotifier, Wake,
};
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::a::chain_handover::{
    ChainInput, StepOutput,
};
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::a::expander::PlannedOperation;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::a::step_completion::{
    withdraw_pending, FollowUpSink, StepPayload,
};
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::a::resource_requirements::{
    IndexId, ModelId, ResourceKind, ResourceRequirements,
};
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::b::task_context::TaskId;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::a::index_contract::Index;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::a::model_descriptor::ModelDescriptor;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::b::task_model::{
    Task, TaskParams, TaskState,
};
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::_1::capability_model::CapabilitySet;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::_1::device_lane_event::DeviceKind;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::_1::queue::Queue;
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EngineError {
    Config { field: &'static str },
    DuplicateIndex(String),
    Planner(PlannerConfigError),
    Subengine { name: String, reason: String },
    Pool(String),
    ShutDown,
    ShutdownFailed(String),
}
impl fmt::Display for EngineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EngineError::Config { field } => {
                write!(f, "Konfigurationsfehler: Pflichtfeld '{field}' fehlt")
            }
            EngineError::DuplicateIndex(id) => {
                write!(f, "Index-ID bereits registriert: {id}")
            }
            EngineError::Planner(e) => write!(f, "{e}"),
            EngineError::Subengine { name, reason } => {
                write!(f, "Subengine '{name}' konnte nicht erzeugt werden: {reason}")
            }
            EngineError::Pool(text) => {
                write!(f, "Worker-Pool konnte nicht gestartet werden: {text}")
            }
            EngineError::ShutDown => write!(f, "Engine ist heruntergefahren"),
            EngineError::ShutdownFailed(text) => {
                write!(f, "Shutdown fehlgeschlagen: {text}")
            }
        }
    }
}
impl std::error::Error for EngineError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            EngineError::Planner(e) => Some(e),
            _ => None,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ShutdownReport {
    pub cancelled_tasks: usize,
}
/// Payload of an operations-queue step: the chain step of the reference path.
pub type EnginePayload<D, E> = StepPayload<
    OpMeta,
    PlannedOperation,
    ChainInput,
    StepOutput,
    D,
    E,
>;
pub type StepSink<D, E> = dyn FollowUpSink<OpMeta, EnginePayload<D, E>>;
/// Queue value of the reference path: the step together with the sink that continues its chain.
pub struct EngineStep<D, E> {
    pub step: EnginePayload<D, E>,
    pub sink: Arc<StepSink<D, E>>,
}
pub type PlanErrorFn<E> = Arc<dyn Fn(&PlanError) -> E + Send + Sync>;
/// Derives the batch information of a follow-up step from its payload.
pub type BatchProbe<D, E> = Arc<
    dyn Fn(&EnginePayload<D, E>) -> Option<BatchInfo> + Send + Sync,
>;
type Proc<D, T, E, V, R, OE> = Processor<
    D,
    T,
    E,
    OpMeta,
    V,
    R,
    OE,
    PlannerHandle<D, T, E, V>,
>;
struct PlannerHandle<D, T, E, V>(Arc<Planner<D, T, E, V>>);
impl<D, T, E, V> TaskPlanner<D, T, E, OpMeta, V> for PlannerHandle<D, T, E, V> {
    type Error = PlanError;
    fn expand(
        &self,
        task: &Task<D, T, E>,
    ) -> Result<Vec<PlannedOp<OpMeta, V>>, PlanError> {
        self.0.expand(task)
    }
}
/// A sub-engine created after the Processor exists; forwards to the bound instance.
struct DeferredSubengine {
    name: String,
    inner: RwLock<Option<Arc<dyn Subengine>>>,
}
impl DeferredSubengine {
    fn new(name: String) -> Arc<Self> {
        Arc::new(Self {
            name,
            inner: RwLock::new(None),
        })
    }
    fn bind(&self, s: Arc<dyn Subengine>) {
        *self.inner.write().unwrap_or_else(PoisonError::into_inner) = Some(s);
    }
    fn unbind(&self) {
        let old = self.inner.write().unwrap_or_else(PoisonError::into_inner).take();
        drop(old);
    }
    fn bound(&self) -> Option<Arc<dyn Subengine>> {
        self.inner.read().unwrap_or_else(PoisonError::into_inner).clone()
    }
}
impl Subengine for DeferredSubengine {
    fn name(&self) -> String {
        self.name.clone()
    }
    fn start(&self) -> Result<(), String> {
        self.bound().map_or_else(|| Err("nicht gebunden".to_string()), |s| s.start())
    }
    fn capabilities(&self) -> Vec<(QueueKind, CapabilitySet)> {
        self.bound().map_or_else(Vec::new, |s| s.capabilities())
    }
    fn offers(&self) -> Vec<ResourceKind> {
        self.bound().map_or_else(Vec::new, |s| s.offers())
    }
    fn announce(&self, req: &ResourceRequirements) {
        if let Some(s) = self.bound() {
            s.announce(req);
        }
    }
    fn activate(&self) -> Result<(), String> {
        self.bound().map_or_else(|| Err("nicht gebunden".to_string()), |s| s.activate())
    }
    fn is_loaded(&self, kind: ResourceKind, model: Option<&ModelId>) -> bool {
        self.bound().is_some_and(|s| s.is_loaded(kind, model))
    }
}
/// Sink of the reference path. Pushes follow-up steps into the operations queues of the Engine.
pub struct EngineSink<D, T, E, V, R, OE> {
    me: Weak<Self>,
    planner: Weak<Planner<D, T, E, V>>,
    queues: [Weak<Queue<OpMeta, V, R, OE>>; 3],
    topology: Arc<NotificationTopology>,
    plan_error: PlanErrorFn<E>,
    batch_probe: Option<BatchProbe<D, E>>,
}
fn queue_slot(kind: QueueKind) -> usize {
    match kind {
        QueueKind::Cpu => 0,
        QueueKind::Gpu => 1,
        QueueKind::Shared => 2,
    }
}
impl<D, T, E, V, R, OE> EngineSink<D, T, E, V, R, OE>
where
    D: Send + 'static,
    T: Send + Sync + 'static,
    E: Send + Sync + 'static,
    V: Send + 'static,
    R: Send + 'static,
    OE: Send + 'static,
{
    fn new(
        planner: &Arc<Planner<D, T, E, V>>,
        processor: &Proc<D, T, E, V, R, OE>,
        topology: &Arc<NotificationTopology>,
        plan_error: &PlanErrorFn<E>,
        batch_probe: &Option<BatchProbe<D, E>>,
    ) -> Arc<Self> {
        let weak = |kind| Arc::downgrade(processor.ops_queue_arc(kind));
        Arc::new_cyclic(|me| Self {
            me: me.clone(),
            planner: Arc::downgrade(planner),
            queues: [
                weak(QueueKind::Cpu),
                weak(QueueKind::Gpu),
                weak(QueueKind::Shared),
            ],
            topology: Arc::clone(topology),
            plan_error: Arc::clone(plan_error),
            batch_probe: batch_probe.clone(),
        })
    }
    fn withdraw(&self, task_id: TaskId) -> usize {
        self.queues
            .iter()
            .filter_map(Weak::upgrade)
            .map(|q| withdraw_pending(&q, |m: &OpMeta| m.task_id == task_id))
            .sum()
    }
}
impl<D, T, E, V, R, OE> FollowUpSink<OpMeta, EnginePayload<D, E>>
for EngineSink<D, T, E, V, R, OE>
where
    D: Send + 'static,
    T: Send + Sync + 'static,
    E: Clone + Send + Sync + 'static,
    V: From<EngineStep<D, E>> + Send + 'static,
    R: Send + 'static,
    OE: Send + 'static,
{
    fn submit(&self, placement: Option<OpMeta>, mut payload: EnginePayload<D, E>) {
        let (Some(me), Some(planner)) = (self.me.upgrade(), self.planner.upgrade()) else {
            log::debug!("engine sink: engine gone, follow-up step dropped");
            return;
        };
        let mut meta = match placement {
            Some(meta) => meta,
            None => {
                let node = payload.node();
                let task_id = node.task_id();
                let priority = node.task().params().priority;
                let next = payload.rest().first().map(|s| s.node.operation());
                let placed = planner
                    .place_step(task_id, priority, node.operation(), next);
                match placed {
                    Ok(meta) => meta,
                    Err(e) => {
                        log::warn!(
                            "engine sink: step of task {task_id} not placeable: {e}"
                        );
                        let error = (self.plan_error)(&e);
                        payload.fail(error, || self.withdraw(task_id));
                        return;
                    }
                }
            }
        };
        if let Some(probe) = &self.batch_probe {
            meta.batch = probe(&payload);
        }
        let Some(queue) = self.queues[queue_slot(meta.queue)].upgrade() else {
            log::debug!("engine sink: queue gone, follow-up step dropped");
            return;
        };
        let sink: Arc<StepSink<D, E>> = me;
        queue.push(meta, V::from(EngineStep { step: payload, sink }));
        self.topology.notify_push();
    }
}
/// Sink that can be handed out before the Engine exists and bound afterwards.
pub struct LateSink<D, E> {
    inner: Arc<OnceLock<Arc<StepSink<D, E>>>>,
}
impl<D, E> Clone for LateSink<D, E> {
    fn clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
        }
    }
}
impl<D, E> Default for LateSink<D, E> {
    fn default() -> Self {
        Self::new()
    }
}
impl<D, E> LateSink<D, E> {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(OnceLock::new()),
        }
    }
    /// `false`: already bound; the first binding stays.
    pub fn bind(&self, sink: Arc<StepSink<D, E>>) -> bool {
        self.inner.set(sink).is_ok()
    }
    pub fn is_bound(&self) -> bool {
        self.inner.get().is_some()
    }
}
impl<D, E> FollowUpSink<OpMeta, EnginePayload<D, E>> for LateSink<D, E> {
    fn submit(&self, placement: Option<OpMeta>, payload: EnginePayload<D, E>) {
        match self.inner.get() {
            Some(sink) => sink.submit(placement, payload),
            None => log::warn!("late sink: not bound, follow-up step dropped"),
        }
    }
}
/// Handle the factories use to reach the (Engine-owned) ops queues and shared engine parts.
pub struct EngineWiring<D, T, E, V, R, OE> {
    processor: Arc<Proc<D, T, E, V, R, OE>>,
    planner: Arc<Planner<D, T, E, V>>,
    topology: Arc<NotificationTopology>,
    completion: Arc<GpuCompletion>,
    control: Arc<ExpansionControl>,
    names: Vec<String>,
    plan_error: PlanErrorFn<E>,
    batch_probe: Option<BatchProbe<D, E>>,
}
impl<D, T, E, V, R, OE> EngineWiring<D, T, E, V, R, OE>
where
    D: Send + 'static,
    T: Send + Sync + 'static,
    E: Send + Sync + 'static,
    V: Send + 'static,
    R: Send + 'static,
    OE: Send + 'static,
{
    /// Shareable handle to the Processor's ops queue of `kind`; the same instance on every call.
    pub fn ops_queue(&self, kind: QueueKind) -> Arc<Queue<OpMeta, V, R, OE>> {
        Arc::clone(self.processor.ops_queue_arc(kind))
    }
    /// Tracker of in-flight GPU lanes, polled by the workers.
    pub fn completion(&self) -> Arc<GpuCompletion> {
        Arc::clone(&self.completion)
    }
    /// Expansion control the feed steps report their iterations to.
    pub fn expansion_control(&self) -> Arc<ExpansionControl> {
        Arc::clone(&self.control)
    }
    /// Notifier of the configured sub-engine `name`.
    pub fn notifier(&self, name: &str) -> Option<SubengineNotifier> {
        let index = self.names.iter().position(|n| n == name)?;
        self.topology.notifier(index)
    }
    /// Wake-up for producers that push into an operations queue directly.
    pub fn notify_push(&self) {
        self.topology.notify_push();
    }
}
impl<D, T, E, V, R, OE> EngineWiring<D, T, E, V, R, OE>
where
    D: Send + 'static,
    T: Send + Sync + 'static,
    E: Clone + Send + Sync + 'static,
    V: From<EngineStep<D, E>> + Send + 'static,
    R: Send + 'static,
    OE: Send + 'static,
{
    /// Sink that pushes follow-up steps into the Engine's operations queues.
    pub fn sink(&self) -> Arc<StepSink<D, E>> {
        EngineSink::new(
            &self.planner,
            &self.processor,
            &self.topology,
            &self.plan_error,
            &self.batch_probe,
        )
    }
}
pub type SubengineFactory<D, T, E, V, R, OE> = Box<
    dyn FnOnce(
        &EngineWiring<D, T, E, V, R, OE>,
    ) -> Result<Arc<dyn Subengine>, String> + Send,
>;
/// Like `SubengineFactory`, but also yields the feed steps and lane routing of the sub-engine.
pub type HandleFactory<D, T, E, V, R, OE> = Box<
    dyn FnOnce(
        &EngineWiring<D, T, E, V, R, OE>,
    ) -> Result<SubengineHandle, String> + Send,
>;
enum SubengineSpec<D, T, E, V, R, OE> {
    Ready(Arc<dyn Subengine>),
    ReadyHandle(SubengineHandle),
    Factory(String, SubengineFactory<D, T, E, V, R, OE>),
    HandleFactory(String, HandleFactory<D, T, E, V, R, OE>),
}
enum Job4<D, T, E, V, R, OE> {
    Plain(SubengineFactory<D, T, E, V, R, OE>),
    Handle(HandleFactory<D, T, E, V, R, OE>),
}
pub struct EngineBuilder<D, T, E, V, R, OE> {
    registry: Option<Arc<ModelRegistry>>,
    models: Vec<ModelDescriptor>,
    default_model: Option<String>,
    resolver: Option<Arc<dyn TaskResolver<D, T, E>>>,
    values: Option<Arc<dyn ValueBuilder<D, T, E, V>>>,
    plan_error: Option<PlanErrorFn<E>>,
    batch_probe: Option<BatchProbe<D, E>>,
    subengines: Vec<SubengineSpec<D, T, E, V, R, OE>>,
    indexes: Vec<(IndexId, Arc<dyn Index>)>,
    workers: Option<PoolConfig>,
}
impl<D, T, E, V, R, OE> Default for EngineBuilder<D, T, E, V, R, OE>
where
    D: Send + 'static,
    T: Send + Sync + 'static,
    E: Send + Sync + 'static,
    V: Send + 'static,
    R: Send + 'static,
    OE: Send + 'static,
{
    fn default() -> Self {
        Self::new()
    }
}
impl<D, T, E, V, R, OE> EngineBuilder<D, T, E, V, R, OE>
where
    D: Send + 'static,
    T: Send + Sync + 'static,
    E: Send + Sync + 'static,
    V: Send + 'static,
    R: Send + 'static,
    OE: Send + 'static,
{
    pub fn new() -> Self {
        Self {
            registry: None,
            models: Vec::new(),
            default_model: None,
            resolver: None,
            values: None,
            plan_error: None,
            batch_probe: None,
            subengines: Vec::new(),
            indexes: Vec::new(),
            workers: None,
        }
    }
    /// Optional. Default: `ModelRegistry::with_builtin()`.
    pub fn registry(mut self, r: Arc<ModelRegistry>) -> Self {
        self.registry = Some(r);
        self
    }
    /// Required (at least one call). Validation happens in `start`.
    pub fn model(mut self, d: ModelDescriptor) -> Self {
        self.models.push(d);
        self
    }
    /// Optional. Default model = the first `model()` call.
    pub fn default_model(mut self, model_id: impl Into<String>) -> Self {
        self.default_model = Some(model_id.into());
        self
    }
    pub fn task_resolver(mut self, r: Arc<dyn TaskResolver<D, T, E>>) -> Self {
        self.resolver = Some(r);
        self
    }
    pub fn value_builder(mut self, v: Arc<dyn ValueBuilder<D, T, E, V>>) -> Self {
        self.values = Some(v);
        self
    }
    /// Maps an expansion error to the task error for tasks the Planner already claimed.
    pub fn plan_error(mut self, f: Arc<dyn Fn(&PlanError) -> E + Send + Sync>) -> Self {
        self.plan_error = Some(f);
        self
    }
    /// Optional. Sets `OpMeta::batch` of follow-up steps from their payload; without it the
    /// planned metadata is pushed unchanged.
    pub fn batch_probe(mut self, f: BatchProbe<D, E>) -> Self {
        self.batch_probe = Some(f);
        self
    }
    /// Prebuilt sub-engine (the caller wired its queues). It contributes no feed steps.
    pub fn subengine(mut self, s: Arc<dyn Subengine>) -> Self {
        self.subengines.push(SubengineSpec::Ready(s));
        self
    }
    /// Sub-engine created at `start` from the Engine's queues. `name` must equal `Subengine::name()`.
    pub fn subengine_with(
        mut self,
        name: impl Into<String>,
        f: SubengineFactory<D, T, E, V, R, OE>,
    ) -> Self {
        self.subengines.push(SubengineSpec::Factory(name.into(), f));
        self
    }
    /// Prebuilt sub-engine together with its feed steps and lane routing.
    pub fn subengine_handle(mut self, h: SubengineHandle) -> Self {
        self.subengines.push(SubengineSpec::ReadyHandle(h));
        self
    }
    /// Sub-engine handle created at `start` from the Engine's queues. `name` must equal the
    /// name of the handle's sub-engine.
    pub fn handle_with(
        mut self,
        name: impl Into<String>,
        f: HandleFactory<D, T, E, V, R, OE>,
    ) -> Self {
        self.subengines.push(SubengineSpec::HandleFactory(name.into(), f));
        self
    }
    /// Registers a searchable index under a unique ID. Neither loads nor uses it.
    pub fn index(mut self, id: IndexId, index: Arc<dyn Index>) -> Self {
        self.indexes.push((id, index));
        self
    }
    /// Optional. Default: `PoolConfig { min_workers: 1, max_workers: None }`.
    pub fn workers(mut self, cfg: PoolConfig) -> Self {
        self.workers = Some(cfg);
        self
    }
    /// The explicit start call: validates, wires, initializes. Consumes the builder.
    pub fn start(self) -> Result<Engine<D, T, E, V, R, OE>, EngineError> {
        if self.models.is_empty() {
            return Err(EngineError::Config {
                field: "model",
            });
        }
        let resolver = self
            .resolver
            .ok_or(EngineError::Config {
                field: "task_resolver",
            })?;
        let values = self
            .values
            .ok_or(EngineError::Config {
                field: "value_builder",
            })?;
        let plan_error = self
            .plan_error
            .ok_or(EngineError::Config {
                field: "plan_error",
            })?;
        for (i, (id, _)) in self.indexes.iter().enumerate() {
            if self.indexes[..i].iter().any(|(other, _)| other == id) {
                return Err(EngineError::DuplicateIndex(id.as_str().to_string()));
            }
        }
        let registry = self
            .registry
            .unwrap_or_else(|| Arc::new(ModelRegistry::with_builtin()));
        let default_model = self
            .default_model
            .unwrap_or_else(|| self.models[0].model_id.clone());
        let mut builder = PlannerBuilder::new(registry, resolver, values);
        for model in self.models {
            builder = builder.model(model).map_err(EngineError::Planner)?;
        }
        builder = builder.default_model(&default_model).map_err(EngineError::Planner)?;
        for (id, _) in &self.indexes {
            builder = builder.index(id.clone());
        }
        let mut subengines: Vec<Arc<dyn Subengine>> = Vec::with_capacity(
            self.subengines.len(),
        );
        let mut handles: Vec<Option<SubengineHandle>> = Vec::with_capacity(
            self.subengines.len(),
        );
        let mut deferred: Vec<Arc<DeferredSubengine>> = Vec::new();
        let mut jobs: Vec<
            (usize, String, Arc<DeferredSubengine>, Job4<D, T, E, V, R, OE>),
        > = Vec::new();
        for spec in self.subengines {
            let index = subengines.len();
            match spec {
                SubengineSpec::Ready(s) => {
                    subengines.push(s);
                    handles.push(None);
                }
                SubengineSpec::ReadyHandle(h) => {
                    subengines.push(Arc::clone(h.subengine()));
                    handles.push(Some(h));
                }
                SubengineSpec::Factory(name, f) => {
                    let wrapper = DeferredSubengine::new(name.clone());
                    deferred.push(Arc::clone(&wrapper));
                    subengines.push(Arc::clone(&wrapper) as Arc<dyn Subengine>);
                    handles.push(None);
                    jobs.push((index, name, wrapper, Job4::Plain(f)));
                }
                SubengineSpec::HandleFactory(name, f) => {
                    let wrapper = DeferredSubengine::new(name.clone());
                    deferred.push(Arc::clone(&wrapper));
                    subengines.push(Arc::clone(&wrapper) as Arc<dyn Subengine>);
                    handles.push(None);
                    jobs.push((index, name, wrapper, Job4::Handle(f)));
                }
            }
        }
        for s in &subengines {
            builder = builder.subengine(Arc::clone(s)).map_err(EngineError::Planner)?;
        }
        let planner = Arc::new(builder.build());
        let processor: Arc<Proc<D, T, E, V, R, OE>> = Processor::new(
            PlannerHandle(Arc::clone(&planner)),
        );
        let wake_target: Arc<OnceLock<Notifier3>> = Arc::new(OnceLock::new());
        let wake: Wake = {
            let target = Arc::clone(&wake_target);
            Arc::new(move || {
                if let Some(n) = target.get() {
                    n.notify();
                }
            })
        };
        let names: Vec<String> = subengines.iter().map(|s| s.name()).collect();
        let topology = Arc::new(NotificationTopology::new(names.len() + 1, wake));
        let completion_notifier = topology
            .notifier(names.len())
            .ok_or_else(|| EngineError::Pool(
                "notification topology too small".to_string(),
            ))?;
        let completion = Arc::new(GpuCompletion::new(completion_notifier));
        let control = Arc::new(ExpansionControl::new());
        let wiring = EngineWiring {
            processor: Arc::clone(&processor),
            planner: Arc::clone(&planner),
            topology: Arc::clone(&topology),
            completion: Arc::clone(&completion),
            control: Arc::clone(&control),
            names,
            plan_error: Arc::clone(&plan_error),
            batch_probe: self.batch_probe,
        };
        for (index, name, wrapper, job) in jobs {
            let created = match job {
                Job4::Plain(f) => f(&wiring).map(|s| (s, None)),
                Job4::Handle(f) => {
                    f(&wiring).map(|h| (Arc::clone(h.subengine()), Some(h)))
                }
            };
            let fail = |reason: String| EngineError::Subengine {
                name: name.clone(),
                reason,
            };
            let (instance, handle) = created.map_err(fail)?;
            if instance.name() != name {
                return Err(
                    fail(
                        format!(
                            "configured name differs from the sub-engine name '{}'",
                            instance.name()
                        ),
                    ),
                );
            }
            wrapper.bind(instance);
            handles[index] = handle;
        }
        planner.start().map_err(EngineError::Planner)?;
        let cfg = self
            .workers
            .unwrap_or(PoolConfig {
                min_workers: 1,
                max_workers: None,
            });
        let gate = Arc::new(ClosedGate::new());
        let routes: Arc<Vec<SubengineHandle>> = Arc::new(
            handles.into_iter().flatten().collect(),
        );
        let loops = (0..cfg.effective_workers())
            .map(|_| {
                Mutex::new(
                    build_worker_loop(
                        &routes,
                        &completion,
                        Expansion {
                            processor: Arc::clone(&processor),
                            topology: Arc::clone(&topology),
                            control: Arc::clone(&control),
                            gate: Arc::clone(&gate),
                            plan_error: Arc::clone(&plan_error),
                        },
                    ),
                )
            })
            .collect();
        let pool = WorkerPool::start(cfg, Arc::new(EngineWork { loops }))
            .map_err(|e| EngineError::Pool(e.to_string()))?;
        let notifier = pool.notifier();
        let _ = wake_target.set(notifier.clone());
        Ok(Engine {
            processor,
            planner,
            wiring,
            notifier,
            gate,
            indexes: self.indexes,
            subengines: Mutex::new(subengines),
            deferred,
            pool: Mutex::new(Some(pool)),
            shutdown_done: Mutex::new(false),
        })
    }
}
struct ClosedGate {
    gate: RwLock<()>,
    closed: AtomicBool,
}
impl ClosedGate {
    fn new() -> Self {
        Self {
            gate: RwLock::new(()),
            closed: AtomicBool::new(false),
        }
    }
    fn is_closed(&self) -> bool {
        self.closed.load(Ordering::SeqCst)
    }
    fn close(&self) {
        let _exclusive = self.gate.write().unwrap_or_else(PoisonError::into_inner);
        self.closed.store(true, Ordering::SeqCst);
    }
}
struct Expansion<D, T, E, V, R, OE> {
    processor: Arc<Proc<D, T, E, V, R, OE>>,
    topology: Arc<NotificationTopology>,
    control: Arc<ExpansionControl>,
    gate: Arc<ClosedGate>,
    plan_error: PlanErrorFn<E>,
}
impl<D, T, E, V, R, OE> ExpandStep for Expansion<D, T, E, V, R, OE>
where
    D: Send + 'static,
    T: Send + Sync + 'static,
    E: Send + Sync + 'static,
    V: Send + 'static,
    R: Send + 'static,
    OE: Send + 'static,
{
    fn expand_next(&mut self) -> bool {
        if self.gate.is_closed() {
            return false;
        }
        let queue = self.processor.task_queue();
        let underserved = self.control.underserved();
        let preferred = if underserved.is_empty() {
            None
        } else {
            queue.take_where(|m| m.first_kind.is_some_and(|k| underserved.contains(&k)))
        };
        let Some(mut claim) = preferred.or_else(|| queue.take()) else {
            return false;
        };
        if let Some(task) = claim.take_payload() {
            self.expand_task(&task);
        }
        claim.complete(Ok(()));
        true
    }
}
impl<D, T, E, V, R, OE> Expansion<D, T, E, V, R, OE>
where
    D: Send + 'static,
    T: Send + Sync + 'static,
    E: Send + Sync + 'static,
    V: Send + 'static,
    R: Send + 'static,
    OE: Send + 'static,
{
    fn expand_task(&self, task: &Task<D, T, E>) {
        if matches!(task.state(), TaskState::Completed | TaskState::Cancelled) {
            return;
        }
        match self.processor.expand(task) {
            Ok(tickets) => {
                drop(tickets);
                self.topology.notify_push();
            }
            Err(ExpandError3::Planner(e)) => {
                log::warn!("task expansion failed: {e}");
                if task.state() == TaskState::Claimed {
                    let _ = task.complete(Err((self.plan_error)(&e)));
                } else {
                    task.cancel();
                }
            }
        }
    }
}
fn build_worker_loop<X: ExpandStep + 'static>(
    routes: &Arc<Vec<SubengineHandle>>,
    completion: &Arc<GpuCompletion>,
    expand: X,
) -> WorkerLoop {
    let mut gpu_feeds = Vec::new();
    let mut cpu_feeds = Vec::new();
    for handle in routes.iter() {
        let feeds = handle.feeds_for_worker();
        if handle.device() == DeviceKind::Gpu {
            gpu_feeds.extend(feeds);
        } else {
            cpu_feeds.extend(feeds);
        }
    }
    let routes = Arc::clone(routes);
    let on_finished: FinishedHandler = Box::new(move |mut lane| {
        for handle in routes.iter() {
            match handle.on_finished(lane) {
                Ok(()) => return,
                Err(unhandled) => lane = unhandled,
            }
        }
        log::warn!("finished lane with tag {} belongs to no sub-engine", lane.tag);
    });
    WorkerLoop::new(
        Arc::clone(completion),
        on_finished,
        gpu_feeds,
        cpu_feeds,
        Box::new(expand),
    )
}
struct EngineWork {
    loops: Vec<Mutex<WorkerLoop>>,
}
impl WorkSource for EngineWork {
    fn run_once(&self, worker: usize) -> Step2 {
        let Some(slot) = self.loops.get(worker) else {
            return Step2::Idle;
        };
        match lock(slot).run_once(Instant::now()) {
            Some(_) => Step2::Worked,
            None => Step2::Idle,
        }
    }
}
pub struct Engine<D, T, E, V, R, OE>
where
    D: Send + 'static,
    T: Send + Sync + 'static,
    E: Send + Sync + 'static,
    V: Send + 'static,
    R: Send + 'static,
    OE: Send + 'static,
{
    processor: Arc<Proc<D, T, E, V, R, OE>>,
    planner: Arc<Planner<D, T, E, V>>,
    wiring: EngineWiring<D, T, E, V, R, OE>,
    notifier: Notifier3,
    gate: Arc<ClosedGate>,
    indexes: Vec<(IndexId, Arc<dyn Index>)>,
    subengines: Mutex<Vec<Arc<dyn Subengine>>>,
    deferred: Vec<Arc<DeferredSubengine>>,
    pool: Mutex<Option<WorkerPool>>,
    shutdown_done: Mutex<bool>,
}
impl<D, T, E, V, R, OE> Engine<D, T, E, V, R, OE>
where
    D: Send + 'static,
    T: Send + Sync + 'static,
    E: Send + Sync + 'static,
    V: Send + 'static,
    R: Send + 'static,
    OE: Send + 'static,
{
    pub fn builder() -> EngineBuilder<D, T, E, V, R, OE> {
        EngineBuilder::new()
    }
    /// Generic entry. Pushes the task into the task queue and wakes the dispatcher workers.
    pub fn enqueue(
        &self,
        data: D,
        params: TaskParams,
    ) -> Result<Task<D, T, E>, EngineError> {
        self.submit(data, params, None)
    }
    /// Embedding task. `data` must resolve (via the TaskResolver) to an Embedding task.
    pub fn enqueue_embedding(
        &self,
        data: D,
        params: TaskParams,
    ) -> Result<Task<D, T, E>, EngineError> {
        self.submit(data, params, Some(ResourceKind::Tokenizer))
    }
    /// Similarity task. `data` must resolve to a Similarity task.
    pub fn enqueue_similarity(
        &self,
        data: D,
        params: TaskParams,
    ) -> Result<Task<D, T, E>, EngineError> {
        self.submit(data, params, None)
    }
    fn submit(
        &self,
        data: D,
        params: TaskParams,
        first_kind: Option<ResourceKind>,
    ) -> Result<Task<D, T, E>, EngineError> {
        let _open = self.gate.gate.read().unwrap_or_else(PoisonError::into_inner);
        if self.gate.is_closed() {
            return Err(EngineError::ShutDown);
        }
        let task = self.processor.submit(data, params, first_kind);
        self.notifier.notify();
        Ok(task)
    }
    pub fn index(&self, id: &IndexId) -> Option<Arc<dyn Index>> {
        self.indexes
            .iter()
            .find(|(known, _)| known == id)
            .map(|(_, index)| Arc::clone(index))
    }
    /// Registered IDs in registration order.
    pub fn index_ids(&self) -> Vec<IndexId> {
        self.indexes.iter().map(|(id, _)| id.clone()).collect()
    }
    /// `None` if unknown. Otherwise whether any sub-engine reports an index as loaded.
    pub fn is_index_loaded(&self, id: &IndexId) -> Option<bool> {
        self.index(id)?;
        let engines = lock(&self.subengines).clone();
        Some(engines.iter().any(|s| s.is_loaded(ResourceKind::Index, None)))
    }
    pub fn planner(&self) -> &Planner<D, T, E, V> {
        &self.planner
    }
    /// Wiring the factories received; gives access to queues, completion and expansion control.
    pub fn wiring(&self) -> &EngineWiring<D, T, E, V, R, OE> {
        &self.wiring
    }
    pub fn is_running(&self) -> bool {
        !self.gate.is_closed()
    }
    /// Ordered shutdown, idempotent, blocking. Ok = positive feedback.
    pub fn shutdown(&self) -> Result<ShutdownReport, EngineError> {
        let mut done = lock(&self.shutdown_done);
        if *done {
            return Ok(ShutdownReport {
                cancelled_tasks: 0,
            });
        }
        self.gate.close();
        let pool = lock(&self.pool).take();
        if let Some(pool) = &pool {
            pool.stop();
        }
        drop(pool);
        let mut cancelled = 0;
        let mut failures: Vec<String> = Vec::new();
        while let Some(mut claim) = self.processor.task_queue().take() {
            if let Some(task) = claim.take_payload() {
                match catch_unwind(AssertUnwindSafe(|| task.cancel())) {
                    Ok(_) => {}
                    Err(_) => failures.push("cancelling a task panicked".to_string()),
                }
            }
            claim.complete(Ok(()));
            cancelled += 1;
        }
        lock(&self.subengines).clear();
        for wrapper in &self.deferred {
            wrapper.unbind();
        }
        *done = true;
        if failures.is_empty() {
            Ok(ShutdownReport {
                cancelled_tasks: cancelled,
            })
        } else {
            Err(EngineError::ShutdownFailed(failures.join("; ")))
        }
    }
}
impl<D, T, E, V, R, OE> Engine<D, T, E, V, R, OE>
where
    D: Send + 'static,
    T: Send + Sync + 'static,
    E: Clone + Send + Sync + 'static,
    V: From<EngineStep<D, E>> + Send + 'static,
    R: Send + 'static,
    OE: Send + 'static,
{
    /// Sink that pushes follow-up steps into the Engine's operations queues.
    pub fn sink(&self) -> Arc<StepSink<D, E>> {
        self.wiring.sink()
    }
}
impl<D, T, E, V, R, OE> Drop for Engine<D, T, E, V, R, OE>
where
    D: Send + 'static,
    T: Send + Sync + 'static,
    E: Send + Sync + 'static,
    V: Send + 'static,
    R: Send + 'static,
    OE: Send + 'static,
{
    fn drop(&mut self) {
        if let Err(e) = self.shutdown() {
            log::warn!("engine shutdown during drop: {e}");
        }
    }
}
#[cfg(test)]
#[path = "test/engine.rs"]
mod tests;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::_1::sync_util::lock;
