use super::*;
use std::path::Path;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::thread;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060 as l060;
use l060::a::chain_handover::{ChainInput, StepOutput};
use l060::l050::a::expander::{ConfigError, SimilaritySpec};
use l060::l050::a::operation_node::OperationNode;
use l060::l050::a::step_completion::{PlannedStep, StepPayload};
use l060::l050::l040::b::embedding_task::{
    EmbeddingMode, EmbeddingTask, Precision as EmbeddingPrecision,
};
use l060::l050::l040::b::similarity_task::{Scoring, SimilarityParams};
use l060::l050::l040::b::operation::RamData;
use l060::l050::l040::l030::b::task_context::TaskContext;
use l060::l050::l040::l030::l020::a::model_descriptor::E5_SMALL_MODEL_ID;
use l060::l050::l040::l030::l020::b::task_model::{TaskParams, TaskState};
use l060::l050::l040::l030::l020::l010::device_lane_event::DeviceKind;
use l060::l050::l040::l030::l020::l010::layout::DType;
use l060::l050::l040::l030::l020::l010::scopes_and_entry_sets::Scope;
type Payload = StepPayload<
    OpMeta,
    PlannedOperation,
    ChainInput,
    StepOutput,
    TaskSpec,
    String,
>;
type TestTask = Task<TaskSpec, StepOutput, String>;
type TestPlanner = Planner<TaskSpec, StepOutput, String, Payload>;
type TestBuilder = PlannerBuilder<TaskSpec, StepOutput, String, Payload>;
type Log = Arc<Mutex<Vec<String>>>;
type Planned = PlannedOp<OpMeta, Payload>;
static_assertions::assert_impl_all!(PlanError : Send, Sync, std::error::Error);
static_assertions::assert_impl_all!(TestPlanner : Send, Sync);
struct MockEngine {
    name: String,
    caps: Vec<(QueueKind, CapabilitySet)>,
    offers: Vec<ResourceKind>,
    loaded: AtomicBool,
    start_error: Option<String>,
    activate_error: Option<String>,
    log: Log,
    starts: AtomicUsize,
    activations: AtomicUsize,
}
impl MockEngine {
    fn new(name: &str, queue: QueueKind, offers: &[ResourceKind], log: &Log) -> Self {
        Self {
            name: name.to_string(),
            caps: vec![(queue, CapabilitySet::new())],
            offers: offers.to_vec(),
            loaded: AtomicBool::new(false),
            start_error: None,
            activate_error: None,
            log: log.clone(),
            starts: AtomicUsize::new(0),
            activations: AtomicUsize::new(0),
        }
    }
    fn with_caps(mut self, queue: QueueKind, caps: CapabilitySet) -> Self {
        self.caps = vec![(queue, caps)];
        self
    }
    fn failing_start(mut self, reason: &str) -> Self {
        self.start_error = Some(reason.to_string());
        self
    }
    fn failing_activate(mut self, reason: &str) -> Self {
        self.activate_error = Some(reason.to_string());
        self
    }
    fn starts(&self) -> usize {
        self.starts.load(Ordering::SeqCst)
    }
    fn activations(&self) -> usize {
        self.activations.load(Ordering::SeqCst)
    }
}
impl Subengine for MockEngine {
    fn name(&self) -> String {
        self.name.clone()
    }
    fn start(&self) -> Result<(), String> {
        self.log.lock().unwrap().push(format!("{}:start", self.name));
        if let Some(reason) = &self.start_error {
            return Err(reason.clone());
        }
        self.starts.fetch_add(1, Ordering::SeqCst);
        Ok(())
    }
    fn capabilities(&self) -> Vec<(QueueKind, CapabilitySet)> {
        self.caps.clone()
    }
    fn offers(&self) -> Vec<ResourceKind> {
        self.offers.clone()
    }
    fn announce(&self, req: &ResourceRequirements) {
        self.log
            .lock()
            .unwrap()
            .push(format!("{}:announce:{:?}", self.name, req.kind()));
    }
    fn activate(&self) -> Result<(), String> {
        self.log.lock().unwrap().push(format!("{}:activate", self.name));
        if let Some(reason) = &self.activate_error {
            return Err(reason.clone());
        }
        self.activations.fetch_add(1, Ordering::SeqCst);
        Ok(())
    }
    fn is_loaded(&self, _kind: ResourceKind, _model: Option<&ModelId>) -> bool {
        self.loaded.load(Ordering::SeqCst)
    }
}
struct FixedResolver;
impl TaskResolver<TaskSpec, StepOutput, String> for FixedResolver {
    fn resolve(&self, _task: &TestTask, data: &TaskSpec) -> (TaskId, TaskSpec) {
        (TaskId(7), data.clone())
    }
}
/// Builds the payload as the chain wiring contract describes.
struct ChainBuilder {
    batch: Option<BatchInfo>,
    seen: Mutex<Vec<TaskSpec>>,
}
impl ChainBuilder {
    fn new(batch: Option<BatchInfo>) -> Arc<Self> {
        Arc::new(Self {
            batch,
            seen: Mutex::new(Vec::new()),
        })
    }
    fn built(&self) -> usize {
        self.seen.lock().unwrap().len()
    }
}
impl ValueBuilder<TaskSpec, StepOutput, String, Payload> for ChainBuilder {
    fn build(
        &self,
        task: &TestTask,
        _data: TaskSpec,
        spec: &TaskSpec,
        steps: &[PlanStep],
    ) -> (Payload, Option<BatchInfo>) {
        self.seen.lock().unwrap().push(spec.clone());
        let id = steps[0].meta.task_id;
        let context = TaskContext::new(id, steps[0].meta.priority, 1);
        let mut next_input = Some(
            ChainInput::Data(
                StepOutput::Ram(RamData::TextList(vec!["text".to_string(),])),
            ),
        );
        let mut nodes = Vec::new();
        for step in steps {
            let input = next_input.take().unwrap();
            let node = OperationNode::new(
                id,
                step.planned.clone(),
                input,
                step.meta.materialize,
                task.clone(),
            );
            next_input = Some(ChainInput::Upstream(node.output_handle()));
            nodes.push(node);
        }
        let mut nodes = nodes.into_iter();
        let first = nodes.next().unwrap();
        let rest = nodes
            .zip(&steps[1..])
            .map(|(node, step)| PlannedStep {
                placement: step.meta.clone(),
                node,
            })
            .collect();
        (StepPayload::new(first, context, rest), self.batch.clone())
    }
}
struct DelegatingExpander {
    seen_models: Mutex<Vec<String>>,
}
impl Expander for DelegatingExpander {
    fn expand(
        &self,
        spec: &TaskSpec,
        ctx: &ExpansionContext<'_>,
    ) -> Result<Vec<PlannedOperation>, ExpandError> {
        self.seen_models.lock().unwrap().push(requested_model(spec));
        StandardExpander.expand(spec, ctx)
    }
}
struct RequireExpander;
impl Expander for RequireExpander {
    fn expand(
        &self,
        spec: &TaskSpec,
        ctx: &ExpansionContext<'_>,
    ) -> Result<Vec<PlannedOperation>, ExpandError> {
        let chain = StandardExpander.expand(spec, ctx)?;
        Ok(
            chain
                .into_iter()
                .map(|op| {
                    if op.resource().kind() != ResourceKind::Encoder {
                        return op;
                    }
                    let resource = op
                        .resource()
                        .clone()
                        .with_capability("operands", "fp16");
                    let params = op
                        .descriptor()
                        .params()
                        .clone()
                        .with_requirements(resource);
                    PlannedOperation::new(
                        OperationDescriptor::new(op.kind(), params),
                        op.config().clone(),
                    )
                })
                .collect(),
        )
    }
}
struct EmptyExpander;
impl Expander for EmptyExpander {
    fn expand(
        &self,
        _spec: &TaskSpec,
        _ctx: &ExpansionContext<'_>,
    ) -> Result<Vec<PlannedOperation>, ExpandError> {
        Ok(Vec::new())
    }
}
fn new_log() -> Log {
    Arc::new(Mutex::new(Vec::new()))
}
fn descriptor() -> ModelDescriptor {
    ModelDescriptor::e5_small_default(Path::new("/tmp/model"), DType::F16)
}
fn registry() -> Arc<ModelRegistry> {
    Arc::new(ModelRegistry::with_builtin())
}
fn config_err<T>(result: Result<T, PlannerConfigError>) -> PlannerConfigError {
    match result {
        Ok(_) => panic!("expected a configuration error"),
        Err(e) => e,
    }
}
fn builder(engines: &[Arc<MockEngine>], values: &Arc<ChainBuilder>) -> TestBuilder {
    let mut b = PlannerBuilder::new(registry(), Arc::new(FixedResolver), values.clone())
        .model(descriptor())
        .unwrap()
        .default_model(E5_SMALL_MODEL_ID)
        .unwrap();
    for engine in engines {
        b = b.subengine(engine.clone()).unwrap();
    }
    b
}
fn trio(log: &Log) -> [Arc<MockEngine>; 3] {
    use ResourceKind::*;
    [
        Arc::new(
            MockEngine::new(
                "cpu",
                QueueKind::Cpu,
                &[Tokenizer, Encoder, Index, Scorer, CrossEncoder],
                log,
            ),
        ),
        Arc::new(
            MockEngine::new("gpu", QueueKind::Gpu, &[Encoder, Scorer, CrossEncoder], log),
        ),
        Arc::new(
            MockEngine::new(
                "shared",
                QueueKind::Shared,
                &[Encoder, Index, Scorer, CrossEncoder],
                log,
            ),
        ),
    ]
}
fn started(engines: &[Arc<MockEngine>], values: &Arc<ChainBuilder>) -> TestPlanner {
    let planner = builder(engines, values).build();
    planner.start().unwrap();
    planner
}
fn embedding_spec(model: &str, target: Option<&str>) -> TaskSpec {
    TaskSpec::Embedding {
        task: EmbeddingTask::new(
            "hello".to_string(),
            ModelId::new(model),
            EmbeddingMode::Query,
            EmbeddingPrecision::F32,
        ),
        target_index: target.map(IndexId::new),
    }
}
fn similarity_spec(scorer: ScorerChoice, top_k: usize) -> TaskSpec {
    TaskSpec::Similarity(SimilaritySpec {
        params: SimilarityParams::new(IndexId::new("42"))
            .with_top_k(top_k)
            .with_scope(Scope::Subtree { root: 7 }),
        model: ModelId::new("ghost"),
        scorer,
    })
}
fn vector_spec() -> TaskSpec {
    similarity_spec(ScorerChoice::Vector(Scoring::Cosine), 5)
}
fn new_task(spec: TaskSpec) -> TestTask {
    Task::new(
        spec,
        TaskParams {
            priority: 3,
            ..TaskParams::default()
        },
    )
}
fn gpu_idle() -> IdleNotification {
    IdleNotification {
        idle_devices: vec![DeviceKind::Gpu],
    }
}
fn cpu_idle() -> IdleNotification {
    IdleNotification {
        idle_devices: vec![DeviceKind::Cpu],
    }
}
fn one(result: Result<Vec<Planned>, PlanError>) -> Planned {
    let mut ops = result.unwrap_or_else(|e| panic!("expand failed: {e}"));
    assert_eq!(ops.len(), 1);
    ops.remove(0)
}
fn plan_err(result: Result<Vec<Planned>, PlanError>) -> PlanError {
    match result {
        Ok(_) => panic!("expected a planning error"),
        Err(e) => e,
    }
}
fn chain_metas(op: &Planned) -> Vec<OpMeta> {
    let mut metas = vec![op.meta.clone()];
    metas.extend(op.values.rest().iter().map(|s| s.placement.clone()));
    metas
}
fn queues(op: &Planned) -> Vec<QueueKind> {
    chain_metas(op).iter().map(|m| m.queue).collect()
}
fn flags(op: &Planned) -> Vec<bool> {
    chain_metas(op).iter().map(|m| m.materialize).collect()
}
fn entries(log: &Log) -> Vec<String> {
    log.lock().unwrap().clone()
}
fn count(log: &Log, suffix: &str) -> usize {
    entries(log).iter().filter(|e| e.contains(suffix)).count()
}
#[test]
fn config_rejects_duplicates_and_invalid_models() {
    let log = new_log();
    let [cpu, ..] = trio(&log);
    let values = ChainBuilder::new(None);
    let b = builder(&[cpu.clone()], &values);
    assert_eq!(
        config_err(b.subengine(cpu.clone())),
        PlannerConfigError::DuplicateSubengine("cpu".to_string())
    );
    let fresh = || PlannerBuilder::new(
        registry(),
        Arc::new(FixedResolver),
        ChainBuilder::new(None),
    );
    let mut unsupported = descriptor();
    unsupported.architecture = "unknown-arch".to_string();
    assert_eq!(
        config_err(fresh().model(unsupported)),
        PlannerConfigError::InvalidModel(E5_SMALL_MODEL_ID.to_string())
    );
    let mut invalid = descriptor();
    invalid.dimension = 0;
    assert_eq!(
        config_err(fresh().model(invalid)),
        PlannerConfigError::InvalidModel(E5_SMALL_MODEL_ID.to_string())
    );
    assert_eq!(
        config_err(fresh().default_model("nope")),
        PlannerConfigError::UnknownDefaultModel("nope".to_string())
    );
}
#[test]
fn build_starts_nothing_and_start_follows_configuration_order() {
    let log = new_log();
    let engines = trio(&log);
    let values = ChainBuilder::new(None);
    let planner = builder(&engines, &values).build();
    assert!(! planner.is_started());
    assert!(entries(& log).is_empty());
    planner.start().unwrap();
    assert!(planner.is_started());
    assert_eq!(entries(& log), ["cpu:start", "gpu:start", "shared:start"]);
    planner.start().unwrap();
    assert_eq!(entries(& log).len(), 3);
    for engine in &engines {
        assert_eq!(engine.starts(), 1);
    }
}
#[test]
fn failed_start_keeps_planner_stopped() {
    let log = new_log();
    let [cpu, gpu, shared] = trio(&log);
    let broken = Arc::new(
        MockEngine::new("broken", QueueKind::Gpu, &[], &log).failing_start("no device"),
    );
    let values = ChainBuilder::new(None);
    let planner = builder(&[cpu.clone(), broken, gpu.clone(), shared], &values).build();
    assert_eq!(
        planner.start(), Err(PlannerConfigError::StartFailed { subengine : "broken"
        .to_string(), reason : "no device".to_string(), })
    );
    assert!(! planner.is_started());
    assert_eq!(cpu.starts(), 1);
    assert_eq!(gpu.starts(), 0);
    assert_eq!(
        plan_err(planner.expand(& new_task(vector_spec()))), PlanError::NotStarted
    );
    assert!(planner.capabilities().is_empty());
}
#[test]
fn capabilities_are_unions_per_queue_and_overall() {
    let log = new_log();
    let cpu = Arc::new(
        MockEngine::new("cpu", QueueKind::Cpu, &[ResourceKind::Tokenizer], &log)
            .with_caps(QueueKind::Cpu, CapabilitySet::new().with("operands", "fp32")),
    );
    let gpu_a = Arc::new(
        MockEngine::new("gpu_a", QueueKind::Gpu, &[], &log)
            .with_caps(QueueKind::Gpu, CapabilitySet::new().with("operands", "fp16")),
    );
    let gpu_b = Arc::new(
        MockEngine::new("gpu_b", QueueKind::Gpu, &[], &log)
            .with_caps(QueueKind::Gpu, CapabilitySet::new().with("fusion", "on")),
    );
    let values = ChainBuilder::new(None);
    let planner = started(&[cpu, gpu_a, gpu_b], &values);
    let gpu_caps = planner.queue_capabilities(QueueKind::Gpu);
    assert!(gpu_caps.contains(& "operands".into(), & "fp16".into()));
    assert!(gpu_caps.contains(& "fusion".into(), & "on".into()));
    assert!(! gpu_caps.contains(& "operands".into(), & "fp32".into()));
    assert!(planner.queue_capabilities(QueueKind::Shared).is_empty());
    let all = planner.capabilities();
    for (axis, value) in [("operands", "fp32"), ("operands", "fp16"), ("fusion", "on")] {
        assert!(all.contains(& axis.into(), & value.into()));
    }
}
#[test]
fn offers_only_grow_and_keep_early_additions() {
    let log = new_log();
    let engines = trio(&log);
    let values = ChainBuilder::new(None);
    let planner = builder(&engines, &values).build();
    assert_eq!(planner.offers("gpu"), Some(Vec::new()));
    planner.add_offer("gpu", ResourceKind::Normalizer).unwrap();
    planner.start().unwrap();
    let offers = planner.offers("gpu").unwrap();
    assert!(offers.contains(& ResourceKind::Normalizer));
    assert!(offers.contains(& ResourceKind::Encoder));
    planner.add_offer("gpu", ResourceKind::Normalizer).unwrap();
    planner.add_offer("gpu", ResourceKind::Tokenizer).unwrap();
    let grown = planner.offers("gpu").unwrap();
    assert_eq!(grown.len(), offers.len() + 1);
    assert!(offers.iter().all(| k | grown.contains(k)));
    assert_eq!(
        planner.add_offer("missing", ResourceKind::Encoder),
        Err(PlannerConfigError::UnknownSubengine("missing".to_string()))
    );
    assert_eq!(planner.offers("missing"), None);
}
#[test]
fn resource_status_reports_available_and_loaded() {
    let log = new_log();
    let engines = trio(&log);
    let values = ChainBuilder::new(None);
    let planner = builder(&engines, &values).build();
    let status = |p: &TestPlanner, kind| p.resource_status(kind, None);
    assert_eq!(
        status(& planner, ResourceKind::Tokenizer), ResourceStatus { available : false,
        loaded : false }
    );
    planner.start().unwrap();
    assert_eq!(
        status(& planner, ResourceKind::Tokenizer), ResourceStatus { available : true,
        loaded : false }
    );
    engines[0].loaded.store(true, Ordering::SeqCst);
    assert_eq!(
        status(& planner, ResourceKind::Tokenizer), ResourceStatus { available : true,
        loaded : true }
    );
    assert_eq!(
        status(& planner, ResourceKind::Normalizer), ResourceStatus { available : false,
        loaded : false }
    );
    assert_eq!(count(& log, "activate"), 0);
}
#[test]
fn first_resource_kind_matches_the_first_planned_operation() {
    let log = new_log();
    let engines = trio(&log);
    let values = ChainBuilder::new(None);
    let planner = started(&engines, &values);
    let specs = [
        (embedding_spec("m", None), Some(ResourceKind::Tokenizer)),
        (embedding_spec("m", Some("idx")), Some(ResourceKind::Tokenizer)),
        (vector_spec(), Some(ResourceKind::Scorer)),
        (
            similarity_spec(ScorerChoice::CrossEncoder("minilm".to_string()), 3),
            Some(ResourceKind::Tokenizer),
        ),
        (
            TaskSpec::Other {
                kind: "x".to_string(),
            },
            None,
        ),
    ];
    let caps = CapabilitySet::new().with("cross_encoder", "minilm");
    let mut models = ModelDescriptors::new();
    models.insert(E5_SMALL_MODEL_ID.to_string(), descriptor());
    for (spec, expected) in specs {
        assert_eq!(planner.first_resource_kind(& spec), expected);
        if expected.is_some() {
            let spec = with_model(spec, E5_SMALL_MODEL_ID);
            let chain = expander::expand(&spec, &models, &caps, None).unwrap();
            assert_eq!(Some(chain[0].resource().kind()), expected);
        }
    }
}
#[test]
fn embedding_prefers_the_shared_queue() {
    let log = new_log();
    let values = ChainBuilder::new(None);
    let planner = started(&trio(&log), &values);
    let op = one(planner.expand(&new_task(embedding_spec("m", Some("idx")))));
    assert_eq!(op.kind, QueueKind::Cpu);
    assert_eq!(op.meta.kind(), OperationKind::Tokenization);
    use QueueKind::*;
    assert_eq!(queues(& op), [Cpu, Shared, Shared, Shared, Shared]);
    assert_eq!(flags(& op), [true; 5]);
}
#[test]
fn gpu_idle_keeps_encoder_steps_on_the_device() {
    let log = new_log();
    let values = ChainBuilder::new(None);
    let planner = started(&trio(&log), &values);
    planner.set_idle(gpu_idle());
    let op = one(planner.expand(&new_task(embedding_spec("m", Some("idx")))));
    use QueueKind::*;
    assert_eq!(queues(& op), [Cpu, Gpu, Gpu, Gpu, Shared]);
    assert_eq!(flags(& op), [true, false, false, true, true]);
    let op = one(planner.expand(&new_task(embedding_spec("m", None))));
    assert_eq!(queues(& op), [Cpu, Gpu, Gpu, Gpu]);
    assert_eq!(flags(& op), [true, false, false, true]);
}
#[test]
fn set_idle_affects_only_later_expansions() {
    let log = new_log();
    let values = ChainBuilder::new(None);
    let planner = started(&trio(&log), &values);
    planner.set_idle(cpu_idle());
    let first = one(planner.expand(&new_task(embedding_spec("m", None))));
    planner.set_idle(gpu_idle());
    let second = one(planner.expand(&new_task(embedding_spec("m", None))));
    use QueueKind::*;
    assert_eq!(queues(& first), [Cpu, Cpu, Cpu, Cpu]);
    assert_eq!(queues(& second), [Cpu, Gpu, Gpu, Gpu]);
    assert_eq!(flags(& first), [true; 4]);
}
#[test]
fn gpu_is_preferred_over_cpu_when_no_shared_engine_offers_the_kind() {
    let log = new_log();
    let [cpu, gpu, _] = trio(&log);
    let shared = Arc::new(
        MockEngine::new("shared", QueueKind::Shared, &[ResourceKind::Index], &log),
    );
    let values = ChainBuilder::new(None);
    let planner = started(&[cpu, gpu, shared], &values);
    let op = one(planner.expand(&new_task(embedding_spec("m", Some("idx")))));
    use QueueKind::*;
    assert_eq!(queues(& op), [Cpu, Gpu, Gpu, Gpu, Shared]);
}
#[test]
fn queue_capabilities_must_satisfy_the_requirements() {
    let log = new_log();
    let [cpu, _, shared] = trio(&log);
    let gpu = Arc::new(
        MockEngine::new("gpu", QueueKind::Gpu, &[ResourceKind::Encoder], &log)
            .with_caps(QueueKind::Gpu, CapabilitySet::new().with("operands", "fp16")),
    );
    let values = ChainBuilder::new(None);
    let planner = builder(&[cpu.clone(), gpu.clone(), shared.clone()], &values)
        .expander(E5_SMALL_MODEL_ID, TaskKindTag::Embedding, Arc::new(RequireExpander))
        .build();
    planner.start().unwrap();
    let op = one(planner.expand(&new_task(embedding_spec("m", None))));
    use QueueKind::*;
    assert_eq!(queues(& op), [Cpu, Gpu, Gpu, Gpu]);
    let log = new_log();
    let engines = trio(&log);
    let planner = builder(&engines, &values)
        .expander(E5_SMALL_MODEL_ID, TaskKindTag::Embedding, Arc::new(RequireExpander))
        .build();
    planner.start().unwrap();
    assert_eq!(
        plan_err(planner.expand(& new_task(embedding_spec("m", None)))),
        PlanError::MissingPlanningCapability { kind : OperationKind::Embedding, resource
        : ResourceKind::Encoder, }
    );
    assert_eq!(count(& log, "announce"), 0);
    assert_eq!(count(& log, "activate"), 0);
}
#[test]
fn missing_queue_for_a_gpu_only_step_fails_without_side_effects() {
    let log = new_log();
    let [cpu, _, shared] = trio(&log);
    let values = ChainBuilder::new(None);
    let planner = started(&[cpu, shared], &values);
    planner.set_idle(gpu_idle());
    let task = new_task(embedding_spec("m", None));
    assert_eq!(
        plan_err(planner.expand(& task)), PlanError::MissingPlanningCapability { kind :
        OperationKind::Embedding, resource : ResourceKind::Encoder, }
    );
    assert_eq!(task.state(), TaskState::Claimed);
    assert_eq!(count(& log, "announce"), 0);
    assert_eq!(count(& log, "activate"), 0);
    assert_eq!(values.built(), 0);
}
#[test]
fn expand_before_start_claims_nothing() {
    let log = new_log();
    let values = ChainBuilder::new(None);
    let planner = builder(&trio(&log), &values).build();
    let task = new_task(vector_spec());
    assert_eq!(plan_err(planner.expand(& task)), PlanError::NotStarted);
    assert_eq!(task.state(), TaskState::Enqueued);
    assert_eq!(count(& log, "activate"), 0);
}
#[test]
fn claimed_and_cancelled_tasks_are_rejected() {
    let log = new_log();
    let values = ChainBuilder::new(None);
    let planner = started(&trio(&log), &values);
    let task = new_task(vector_spec());
    one(planner.expand(&task));
    assert_eq!(
        plan_err(planner.expand(& task)),
        PlanError::TaskNotClaimable(ClaimError::NotEnqueued(TaskState::Claimed))
    );
    let cancelled = new_task(vector_spec());
    assert!(cancelled.cancel());
    assert_eq!(
        plan_err(planner.expand(& cancelled)),
        PlanError::TaskNotClaimable(ClaimError::Cancelled)
    );
}
#[test]
fn foreign_task_kind_is_an_expand_error_and_leaves_the_task_claimed() {
    let log = new_log();
    let values = ChainBuilder::new(None);
    let planner = started(&trio(&log), &values);
    let task = new_task(TaskSpec::Other {
        kind: "ocr".to_string(),
    });
    assert_eq!(
        plan_err(planner.expand(& task)), PlanError::Expand(ExpandError::ForeignTaskKind
        { expected : "Embedding/Similarity".to_string(), got : "ocr".to_string(), })
    );
    assert_eq!(task.state(), TaskState::Claimed);
    assert_eq!(count(& log, "activate"), 0);
}
#[test]
fn model_is_always_the_default_model() {
    let log = new_log();
    let values = ChainBuilder::new(None);
    let recorder = Arc::new(DelegatingExpander {
        seen_models: Mutex::new(Vec::new()),
    });
    let planner = builder(&trio(&log), &values)
        .expander(E5_SMALL_MODEL_ID, TaskKindTag::Embedding, recorder.clone())
        .expander(E5_SMALL_MODEL_ID, TaskKindTag::Similarity, recorder.clone())
        .build();
    planner.start().unwrap();
    let op = one(planner.expand(&new_task(embedding_spec("ghost", None))));
    assert_eq!(op.meta.resource().model().map(ModelId::as_str), Some(E5_SMALL_MODEL_ID));
    one(planner.expand(&new_task(vector_spec())));
    let seen = recorder.seen_models.lock().unwrap().clone();
    assert_eq!(seen, [E5_SMALL_MODEL_ID, E5_SMALL_MODEL_ID]);
    let built: Vec<String> = values
        .seen
        .lock()
        .unwrap()
        .iter()
        .map(requested_model)
        .collect();
    assert_eq!(built, [E5_SMALL_MODEL_ID, E5_SMALL_MODEL_ID]);
}
#[test]
fn unknown_model_without_default_or_expander() {
    let log = new_log();
    let values = ChainBuilder::new(None);
    let no_default = PlannerBuilder::new(
            registry(),
            Arc::new(FixedResolver),
            values.clone(),
        )
        .model(descriptor())
        .unwrap()
        .subengine(trio(&log)[0].clone())
        .unwrap()
        .build();
    no_default.start().unwrap();
    assert_eq!(
        plan_err(no_default.expand(& new_task(embedding_spec("ghost", None)))),
        PlanError::UnknownModel { model : "ghost".to_string() }
    );
    let planner = builder(&trio(&log), &values)
        .without_expander(E5_SMALL_MODEL_ID, TaskKindTag::Similarity)
        .build();
    planner.start().unwrap();
    let error = plan_err(planner.expand(&new_task(vector_spec())));
    assert_eq!(error, PlanError::UnknownModel { model : E5_SMALL_MODEL_ID.to_string() });
    assert!(error.to_string().contains(E5_SMALL_MODEL_ID));
    one(planner.expand(&new_task(embedding_spec("m", None))));
}
#[test]
fn expander_errors_are_wrapped() {
    let log = new_log();
    let values = ChainBuilder::new(None);
    let planner = started(&trio(&log), &values);
    let zero = new_task(similarity_spec(ScorerChoice::Vector(Scoring::Cosine), 0));
    assert_eq!(
        plan_err(planner.expand(& zero)),
        PlanError::Expand(ExpandError::Config(ConfigError::ZeroTopK))
    );
    let cross = new_task(
        similarity_spec(ScorerChoice::CrossEncoder("minilm".to_string()), 3),
    );
    assert_eq!(
        plan_err(planner.expand(& cross)),
        PlanError::Expand(ExpandError::ScoringUnavailable { scorer : "minilm".to_string()
        })
    );
    assert_eq!(count(& log, "activate"), 0);
}
#[test]
fn cross_encoder_chain_is_planned_when_capable() {
    let log = new_log();
    let values = ChainBuilder::new(None);
    let cpu = Arc::new(
        MockEngine::new("cpu", QueueKind::Cpu, &[ResourceKind::Tokenizer], &log)
            .with_caps(
                QueueKind::Cpu,
                CapabilitySet::new().with("cross_encoder", "minilm"),
            ),
    );
    let gpu = Arc::new(
        MockEngine::new("gpu", QueueKind::Gpu, &[ResourceKind::CrossEncoder], &log),
    );
    let planner = started(&[cpu, gpu], &values);
    let spec = similarity_spec(ScorerChoice::CrossEncoder("minilm".to_string()), 3);
    let op = one(planner.expand(&new_task(spec)));
    use QueueKind::*;
    assert_eq!(queues(& op), [Cpu, Gpu, Gpu]);
    let kinds: Vec<_> = chain_metas(&op).iter().map(|m| m.kind()).collect();
    assert_eq!(
        kinds, [OperationKind::Tokenization, OperationKind::Inference,
        OperationKind::Similarity]
    );
}
#[test]
fn announces_everything_before_activating_each_engine_once() {
    let log = new_log();
    let values = ChainBuilder::new(None);
    let engines = trio(&log);
    let planner = started(&engines, &values);
    planner.set_idle(gpu_idle());
    log.lock().unwrap().clear();
    one(planner.expand(&new_task(embedding_spec("m", Some("idx")))));
    let log = entries(&log);
    let last_announce = log.iter().rposition(|e| e.contains("announce")).unwrap();
    let first_activate = log.iter().position(|e| e.contains("activate")).unwrap();
    assert!(last_announce < first_activate);
    assert_eq!(
        log.iter().filter(| e | e.contains("announce")).cloned().collect::< Vec < _ >>
        (), ["cpu:announce:Tokenizer", "gpu:announce:Encoder", "gpu:announce:Encoder",
        "gpu:announce:Encoder", "shared:announce:Index",]
    );
    assert_eq!(
        log.iter().filter(| e | e.contains("activate")).cloned().collect::< Vec < _ >>
        (), ["cpu:activate", "gpu:activate", "shared:activate"]
    );
}
#[test]
fn activation_failure_stops_at_the_first_failing_engine() {
    let log = new_log();
    let [cpu, _, shared] = trio(&log);
    let gpu = Arc::new(
        MockEngine::new("gpu", QueueKind::Gpu, &[ResourceKind::Encoder], &log)
            .failing_activate("boom"),
    );
    let values = ChainBuilder::new(None);
    let planner = started(&[cpu.clone(), gpu, shared.clone()], &values);
    planner.set_idle(gpu_idle());
    let task = new_task(embedding_spec("m", Some("idx")));
    assert_eq!(
        plan_err(planner.expand(& task)), PlanError::Activation { subengine : "gpu"
        .to_string(), reason : "boom".to_string() }
    );
    assert_eq!(task.state(), TaskState::Claimed);
    assert_eq!(cpu.activations(), 1);
    assert_eq!(shared.activations(), 0);
    assert_eq!(values.built(), 0);
}
#[test]
fn meta_carries_the_planned_values() {
    let log = new_log();
    let batch = BatchInfo {
        key: BatchKey("k".to_string()),
        seq_lengths: vec![4, 9],
    };
    let values = ChainBuilder::new(Some(batch.clone()));
    let planner = started(&trio(&log), &values);
    let op = one(planner.expand(&new_task(embedding_spec("m", Some("idx")))));
    let models = {
        let mut m = ModelDescriptors::new();
        m.insert(E5_SMALL_MODEL_ID.to_string(), descriptor());
        m
    };
    let expected = expander::expand(
            &with_model(embedding_spec("m", Some("idx")), E5_SMALL_MODEL_ID),
            &models,
            &CapabilitySet::new(),
            Some(&IdleNotification::default()),
        )
        .unwrap();
    let metas = chain_metas(&op);
    assert_eq!(metas.len(), expected.len());
    for (meta, planned) in metas.iter().zip(&expected) {
        assert_eq!(meta.task_id, TaskId(7));
        assert_eq!(meta.priority, 3);
        assert_eq!(meta.kind(), planned.kind());
        assert_eq!(& meta.descriptor, planned.descriptor());
        assert_eq!(meta.resource(), planned.resource());
        assert_eq!(meta.required(), planned.resource().capabilities());
    }
    assert_eq!(op.meta.batch, Some(batch.clone()));
    assert!(metas[1..].iter().all(| m | m.batch.is_none()));
    assert_eq!(op.meta.batch_key(), Some(BatchKey("k".to_string())));
    assert_eq!(op.meta.seq_lengths(), [4, 9]);
    assert_eq!(metas[1].batch_key(), None);
    assert!(metas[1].seq_lengths().is_empty());
    assert_eq!(op.meta.operation_kind(), OperationKind::Tokenization);
    assert_eq!(op.meta.resource_requirements().kind(), ResourceKind::Tokenizer);
}
#[test]
fn payload_follows_the_chain_wiring() {
    let log = new_log();
    let values = ChainBuilder::new(None);
    let planner = started(&trio(&log), &values);
    planner.set_idle(gpu_idle());
    let op = one(planner.expand(&new_task(embedding_spec("m", Some("idx")))));
    let payload = &op.values;
    assert_eq!(payload.rest().len(), 4);
    assert!(! payload.node().ends_chain());
    assert_eq!(payload.node().task_id(), TaskId(7));
    assert_eq!(payload.context().priority(), 3);
    assert_eq!(payload.context().open_operations(), 1);
    let rest = payload.rest();
    assert!(rest.iter().all(| s | s.node.ends_chain()));
    for step in rest {
        assert_eq!(step.node.materialize(), step.placement.materialize);
        assert_eq!(step.node.operation().kind(), step.placement.kind());
        assert!(matches!(step.node.input(), ChainInput::Upstream(_)));
    }
    assert_eq!(payload.node().materialize(), op.meta.materialize);
}
#[test]
fn equal_inputs_give_equal_meta_chains() {
    let log = new_log();
    let values = ChainBuilder::new(None);
    let planner = started(&trio(&log), &values);
    planner.set_idle(gpu_idle());
    let a = one(planner.expand(&new_task(embedding_spec("m", Some("idx")))));
    let b = one(planner.expand(&new_task(embedding_spec("m", Some("idx")))));
    assert_eq!(format!("{:?}", chain_metas(& a)), format!("{:?}", chain_metas(& b)));
}
#[test]
fn empty_expander_result_is_an_error() {
    let log = new_log();
    let values = ChainBuilder::new(None);
    let planner = builder(&trio(&log), &values)
        .expander(E5_SMALL_MODEL_ID, TaskKindTag::Embedding, Arc::new(EmptyExpander))
        .build();
    planner.start().unwrap();
    assert_eq!(
        plan_err(planner.expand(& new_task(embedding_spec("m", None)))),
        PlanError::EmptyChain
    );
}
#[test]
fn place_step_places_and_announces_without_activating() {
    let log = new_log();
    let values = ChainBuilder::new(None);
    let engines = trio(&log);
    let planner = builder(&engines, &values).build();
    let models = {
        let mut m = ModelDescriptors::new();
        m.insert(E5_SMALL_MODEL_ID.to_string(), descriptor());
        m
    };
    let chain = expander::expand(
            &with_model(embedding_spec("m", Some("idx")), E5_SMALL_MODEL_ID),
            &models,
            &CapabilitySet::new(),
            Some(&gpu_idle()),
        )
        .unwrap();
    assert_eq!(
        planner.place_step(TaskId(1), 2, & chain[1], Some(& chain[2])).unwrap_err(),
        PlanError::NotStarted
    );
    planner.start().unwrap();
    let meta = planner.place_step(TaskId(1), 2, &chain[1], Some(&chain[2])).unwrap();
    assert_eq!(meta.queue, QueueKind::Gpu);
    assert!(! meta.materialize);
    assert_eq!((meta.task_id, meta.priority), (TaskId(1), 2));
    assert!(meta.batch.is_none());
    let last = planner.place_step(TaskId(1), 2, &chain[4], None).unwrap();
    assert_eq!(last.queue, QueueKind::Shared);
    assert!(last.materialize);
    let before_index = planner
        .place_step(TaskId(1), 2, &chain[3], Some(&chain[4]))
        .unwrap();
    assert!(before_index.materialize);
    assert_eq!(count(& log, "announce"), 3);
    assert_eq!(count(& log, "activate"), 0);
    let narrow = started(&[trio(&new_log())[0].clone()], &values);
    assert_eq!(
        narrow.place_step(TaskId(1), 0, & chain[1], None).unwrap_err(),
        PlanError::MissingPlanningCapability { kind : OperationKind::Embedding, resource
        : ResourceKind::Encoder, }
    );
}
#[test]
fn concurrent_expansions_share_one_planner() {
    let log = new_log();
    let values = ChainBuilder::new(None);
    let planner = Arc::new(started(&trio(&log), &values));
    planner.set_idle(gpu_idle());
    let handles: Vec<_> = (0..8)
        .map(|_| {
            let planner = planner.clone();
            thread::spawn(move || {
                let task = new_task(embedding_spec("m", Some("idx")));
                let ops = planner.expand(&task).unwrap();
                (ops.len(), queues(&ops[0]))
            })
        })
        .collect();
    use QueueKind::*;
    for handle in handles {
        let (len, placed) = handle.join().unwrap();
        assert_eq!(len, 1);
        assert_eq!(placed, [Cpu, Gpu, Gpu, Gpu, Shared]);
    }
    assert_eq!(values.built(), 8);
}
#[test]
fn error_messages() {
    assert_eq!(PlanError::NotStarted.to_string(), "Planner nicht gestartet");
    assert_eq!(
        PlanError::UnknownModel { model : "m".to_string() } .to_string(),
        "unbekanntes Modell: m"
    );
    assert_eq!(
        PlanError::MissingPlanningCapability { kind : OperationKind::Embedding, resource
        : ResourceKind::Encoder, } .to_string(),
        "fehlende Planungs-Capability: Embedding (Encoder)"
    );
    assert_eq!(
        PlanError::Activation { subengine : "gpu".to_string(), reason : "boom"
        .to_string() } .to_string(), "Subengine gpu nicht aktivierbar: boom"
    );
    assert_eq!(
        PlanError::TaskNotClaimable(ClaimError::Cancelled).to_string(),
        "Task nicht beanspruchbar: Cancelled"
    );
    let inner = ExpandError::UnknownModel {
        model: "m".to_string(),
    };
    assert_eq!(PlanError::Expand(inner.clone()).to_string(), inner.to_string());
}
