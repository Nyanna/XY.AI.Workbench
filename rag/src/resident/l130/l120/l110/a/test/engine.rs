use super::*;
use std::path::Path;
use std::sync::atomic::AtomicUsize;
use std::sync::Condvar;
use std::thread;
use std::time::Duration;
use crate::resident::l130::l120::l110::l100::l090::l080 as l080;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060 as l060;
use l060::l050::a::expander::{ScorerChoice, SimilaritySpec, TaskSpec};
use l060::l050::a::operation_node::OperationNode;
use l060::l050::a::step_completion::PlannedStep;
use l060::l050::l040::a::resource_requirements::ExecutionEnvironment;
use l060::l050::l040::b::embedding_task::{
    EmbeddingMode, EmbeddingTask, Precision as EmbeddingPrecision,
};
use l060::l050::l040::b::similarity_task::{
    IndexId as SimilarityIndexId, Scoring, SimilarityParams,
};
use l060::l050::l040::l030::b::operation::RamData;
use l060::l050::l040::l030::b::task_context::TaskContext;
use l060::l050::l040::l030::l020::a::index_contract::{
    AccessMethod, EntryId, IndexError, Payload, PayloadRef, Stamp, StampObserver,
    StampSignal,
};
use l060::l050::l040::l030::l020::a::model_descriptor::E5_SMALL_MODEL_ID;
use l060::l050::l040::l030::l020::b::task_model::TaskError;
use l060::l050::l040::l030::l020::l010::layout::DType;
use l060::l050::l040::l030::l020::l010::scopes_and_entry_sets::Scope;
use l080::subengine_loop::{IdleNotice, IterationOutcome};
use crate::resident::l130::l120::l110::l100::a::planner::PlanStep;
type Step = EngineStep<TaskSpec, String>;
type TestEngine = Engine<TaskSpec, StepOutput, String, Step, (), ()>;
type TestBuilder = EngineBuilder<TaskSpec, StepOutput, String, Step, (), ()>;
type TestTask = Task<TaskSpec, StepOutput, String>;
type Log = Arc<Mutex<Vec<String>>>;
static_assertions::assert_impl_all!(TestEngine : Send, Sync);
fn wait_until(f: impl Fn() -> bool) {
    let end = Instant::now() + Duration::from_secs(10);
    while !f() {
        assert!(Instant::now() < end, "condition not reached in time");
        thread::sleep(Duration::from_millis(2));
    }
}
struct Probe {
    name: String,
    queue: QueueKind,
    offers: Vec<ResourceKind>,
    log: Log,
    start_error: Option<String>,
    loaded: AtomicBool,
    activations: AtomicUsize,
    blocked: Mutex<bool>,
    wake: Condvar,
}
impl Probe {
    fn new(name: &str, queue: QueueKind, offers: &[ResourceKind], log: &Log) -> Self {
        Self {
            name: name.to_string(),
            queue,
            offers: offers.to_vec(),
            log: Arc::clone(log),
            start_error: None,
            loaded: AtomicBool::new(false),
            activations: AtomicUsize::new(0),
            blocked: Mutex::new(false),
            wake: Condvar::new(),
        }
    }
    fn blocking(self) -> Self {
        *lock(&self.blocked) = true;
        self
    }
    fn failing_start(mut self, reason: &str) -> Self {
        self.start_error = Some(reason.to_string());
        self
    }
    fn release(&self) {
        *lock(&self.blocked) = false;
        self.wake.notify_all();
    }
    fn activations(&self) -> usize {
        self.activations.load(Ordering::SeqCst)
    }
}
impl Subengine for Probe {
    fn name(&self) -> String {
        self.name.clone()
    }
    fn start(&self) -> Result<(), String> {
        lock(&self.log).push(format!("{}:start", self.name));
        self.start_error.clone().map_or(Ok(()), Err)
    }
    fn capabilities(&self) -> Vec<(QueueKind, CapabilitySet)> {
        vec![(self.queue, CapabilitySet::new())]
    }
    fn offers(&self) -> Vec<ResourceKind> {
        self.offers.clone()
    }
    fn announce(&self, _req: &ResourceRequirements) {
        lock(&self.log).push(format!("{}:announce", self.name));
    }
    fn activate(&self) -> Result<(), String> {
        lock(&self.log).push(format!("{}:activate", self.name));
        self.activations.fetch_add(1, Ordering::SeqCst);
        let mut blocked = lock(&self.blocked);
        while *blocked {
            let (next, timeout) = self
                .wake
                .wait_timeout(blocked, Duration::from_secs(5))
                .unwrap_or_else(PoisonError::into_inner);
            blocked = next;
            if timeout.timed_out() {
                break;
            }
        }
        Ok(())
    }
    fn is_loaded(&self, kind: ResourceKind, _model: Option<&ModelId>) -> bool {
        kind == ResourceKind::Index && self.loaded.load(Ordering::SeqCst)
    }
}
struct ReleaseOnDrop(Arc<Probe>);
impl Drop for ReleaseOnDrop {
    fn drop(&mut self) {
        self.0.release();
    }
}
struct CountingIndex {
    calls: AtomicUsize,
    signal: Arc<StampSignal>,
}
impl CountingIndex {
    fn new() -> Arc<Self> {
        Arc::new(Self {
            calls: AtomicUsize::new(0),
            signal: StampSignal::new(Stamp(0)),
        })
    }
    fn hit(&self) {
        self.calls.fetch_add(1, Ordering::SeqCst);
    }
    fn calls(&self) -> usize {
        self.calls.load(Ordering::SeqCst)
    }
}
impl Index for CountingIndex {
    fn dtype(&self) -> DType {
        self.hit();
        DType::F32
    }
    fn access_methods(&self) -> &[AccessMethod] {
        self.hit();
        &[]
    }
    fn stamp(&self, _id: EntryId) -> Option<Stamp> {
        self.hit();
        None
    }
    fn stamps(&self) -> Vec<(EntryId, Stamp)> {
        self.hit();
        Vec::new()
    }
    fn read(&self, _id: EntryId) -> Payload {
        self.hit();
        Payload::default()
    }
    fn write(
        &self,
        _id: EntryId,
        _payload: PayloadRef<'_>,
    ) -> Result<Stamp, IndexError> {
        self.hit();
        Ok(Stamp(0))
    }
    fn delete(&self, _id: EntryId) -> Result<Option<Stamp>, IndexError> {
        self.hit();
        Ok(None)
    }
    fn subscribe(&self) -> StampObserver {
        self.hit();
        self.signal.observer()
    }
}
struct FixedResolver;
impl TaskResolver<TaskSpec, StepOutput, String> for FixedResolver {
    fn resolve(&self, _task: &TestTask, data: &TaskSpec) -> (TaskId, TaskSpec) {
        (TaskId(7), data.clone())
    }
}
struct EngineValues {
    late: LateSink<TaskSpec, String>,
}
impl ValueBuilder<TaskSpec, StepOutput, String, Step> for EngineValues {
    fn build(
        &self,
        task: &TestTask,
        _data: TaskSpec,
        _spec: &TaskSpec,
        steps: &[PlanStep],
    ) -> (Step, Option<BatchInfo>) {
        let id = steps[0].meta.task_id.0;
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
        let sink: Arc<StepSink<TaskSpec, String>> = Arc::new(self.late.clone());
        let step = EngineStep {
            step: StepPayload::new(first, context, rest),
            sink,
        };
        (step, None)
    }
}
fn new_log() -> Log {
    Arc::new(Mutex::new(Vec::new()))
}
fn entries(log: &Log) -> Vec<String> {
    lock(log).clone()
}
fn descriptor() -> ModelDescriptor {
    ModelDescriptor::e5_small_default(
        &Path::new(env!("CARGO_MANIFEST_DIR")).join("model/multilingual-e5-small"),
        DType::F16,
    )
}
fn embedding_spec() -> TaskSpec {
    TaskSpec::Embedding {
        task: EmbeddingTask::new(
            "hello".to_string(),
            ModelId::new("m"),
            EmbeddingMode::Query,
            EmbeddingPrecision::F32,
        ),
        target_index: None,
    }
}
fn similarity_spec() -> TaskSpec {
    TaskSpec::Similarity(SimilaritySpec {
        params: SimilarityParams::new(SimilarityIndexId(42))
            .with_top_k(5)
            .with_scope(Scope::Subtree { root: 7 }),
        model: ModelId::new("ghost"),
        scorer: ScorerChoice::Vector(Scoring::Cosine),
    })
}
fn foreign_spec() -> TaskSpec {
    TaskSpec::Other {
        kind: "ocr".to_string(),
    }
}
fn params(priority: u8) -> TaskParams {
    TaskParams {
        priority: priority.into(),
        ..TaskParams::default()
    }
}
fn base(late: &LateSink<TaskSpec, String>) -> TestBuilder {
    EngineBuilder::new()
        .model(descriptor())
        .task_resolver(Arc::new(FixedResolver))
        .value_builder(Arc::new(EngineValues { late: late.clone() }))
        .plan_error(Arc::new(|e: &PlanError| format!("plan:{e}")))
}
fn trio(log: &Log) -> [Arc<Probe>; 3] {
    use ResourceKind::*;
    [
        Arc::new(
            Probe::new(
                "cpu",
                QueueKind::Cpu,
                &[Tokenizer, Encoder, Index, Scorer, CrossEncoder],
                log,
            ),
        ),
        Arc::new(
            Probe::new("gpu", QueueKind::Gpu, &[Encoder, Scorer, CrossEncoder], log),
        ),
        Arc::new(
            Probe::new(
                "shared",
                QueueKind::Shared,
                &[Encoder, Index, Scorer, CrossEncoder],
                log,
            ),
        ),
    ]
}
fn start_with(probes: &[Arc<Probe>]) -> TestEngine {
    let late = LateSink::new();
    let mut builder = base(&late);
    for probe in probes {
        builder = builder.subengine(probe.clone());
    }
    let engine = builder.start().unwrap();
    assert!(late.bind(engine.sink()));
    engine
}
fn start_error(result: Result<TestEngine, EngineError>) -> EngineError {
    match result {
        Ok(_) => panic!("start unexpectedly succeeded"),
        Err(e) => e,
    }
}
fn blocked_fixture() -> (TestEngine, Arc<Probe>, ReleaseOnDrop) {
    let log = new_log();
    let [cpu, gpu, shared] = trio(&log);
    let cpu = Arc::new(
        Probe::new("cpu", QueueKind::Cpu, &cpu.offers.clone(), &log).blocking(),
    );
    let engine = start_with(&[cpu.clone(), gpu, shared]);
    let guard = ReleaseOnDrop(cpu.clone());
    (engine, cpu, guard)
}
fn pending_first_kinds(engine: &TestEngine) -> Vec<Option<ResourceKind>> {
    let mut seen = Vec::new();
    let none = engine
        .processor
        .task_queue()
        .take_where(|m| {
            seen.push(m.first_kind);
            false
        });
    assert!(none.is_none());
    seen
}
#[test]
fn error_messages() {
    assert_eq!(
        EngineError::Config { field : "model" } .to_string(),
        "Konfigurationsfehler: Pflichtfeld 'model' fehlt"
    );
    assert_eq!(
        EngineError::DuplicateIndex("a".to_string()).to_string(),
        "Index-ID bereits registriert: a"
    );
    assert_eq!(
        EngineError::Subengine { name : "gpu".to_string(), reason : "boom".to_string(), }
        .to_string(), "Subengine 'gpu' konnte nicht erzeugt werden: boom"
    );
    assert_eq!(
        EngineError::Pool("x".to_string()).to_string(),
        "Worker-Pool konnte nicht gestartet werden: x"
    );
    assert_eq!(EngineError::ShutDown.to_string(), "Engine ist heruntergefahren");
    assert_eq!(
        EngineError::ShutdownFailed("y".to_string()).to_string(),
        "Shutdown fehlgeschlagen: y"
    );
    let inner = PlannerConfigError::StartFailed {
        subengine: "e".to_string(),
        reason: "r".to_string(),
    };
    assert_eq!(EngineError::Planner(inner.clone()).to_string(), inner.to_string());
}
#[test]
fn required_fields_are_checked_in_order() {
    let late = LateSink::new();
    let e = start_error(TestBuilder::new().start());
    assert_eq!(e, EngineError::Config { field : "model" });
    assert!(e.to_string().contains("model"));
    let e = start_error(TestBuilder::new().model(descriptor()).start());
    assert_eq!(e, EngineError::Config { field : "task_resolver" });
    let e = start_error(
        TestBuilder::new()
            .model(descriptor())
            .task_resolver(Arc::new(FixedResolver))
            .start(),
    );
    assert_eq!(e, EngineError::Config { field : "value_builder" });
    let e = start_error(
        TestBuilder::new()
            .model(descriptor())
            .task_resolver(Arc::new(FixedResolver))
            .value_builder(Arc::new(EngineValues { late: late.clone() }))
            .start(),
    );
    assert_eq!(e, EngineError::Config { field : "plan_error" });
}
#[test]
fn builder_creates_nothing_before_start() {
    let calls = Arc::new(AtomicUsize::new(0));
    let counter = Arc::clone(&calls);
    let log = new_log();
    let probe = Arc::new(
        Probe::new("cpu", QueueKind::Cpu, &[ResourceKind::Tokenizer], &log),
    );
    let late = LateSink::new();
    let builder = base(&late)
        .subengine(probe)
        .subengine_with(
            "gpu",
            Box::new(move |_| {
                counter.fetch_add(1, Ordering::SeqCst);
                Err("unused".to_string())
            }),
        )
        .index(IndexId::new("a"), CountingIndex::new())
        .workers(PoolConfig {
            min_workers: 3,
            max_workers: Some(3),
        });
    drop(builder);
    assert_eq!(calls.load(Ordering::SeqCst), 0);
    assert!(entries(& log).is_empty());
}
#[test]
fn duplicate_index_ids_fail_before_anything_is_created() {
    let calls = Arc::new(AtomicUsize::new(0));
    let counter = Arc::clone(&calls);
    let late = LateSink::new();
    let e = start_error(
        base(&late)
            .subengine_with(
                "gpu",
                Box::new(move |_| {
                    counter.fetch_add(1, Ordering::SeqCst);
                    Err("unused".to_string())
                }),
            )
            .index(IndexId::new("a"), CountingIndex::new())
            .index(IndexId::new("b"), CountingIndex::new())
            .index(IndexId::new("a"), CountingIndex::new())
            .start(),
    );
    assert_eq!(e, EngineError::DuplicateIndex("a".to_string()));
    assert_eq!(calls.load(Ordering::SeqCst), 0);
}
#[test]
fn planner_configuration_errors_are_wrapped() {
    let late = LateSink::new();
    let e = start_error(base(&late).default_model("no-such-model").start());
    assert!(matches!(e, EngineError::Planner(_)));
}
#[test]
fn start_creates_no_resources_and_registers_indexes() {
    let log = new_log();
    let [cpu, gpu, shared] = trio(&log);
    let first = CountingIndex::new();
    let second = CountingIndex::new();
    let late = LateSink::new();
    let gpu_probe = gpu.clone();
    let engine = base(&late)
        .subengine(cpu.clone())
        .subengine_with("gpu", Box::new(move |_| Ok(gpu_probe as Arc<dyn Subengine>)))
        .subengine(shared.clone())
        .index(IndexId::new("b"), first.clone())
        .index(IndexId::new("a"), second.clone())
        .start()
        .unwrap();
    assert_eq!(entries(& log), ["cpu:start", "gpu:start", "shared:start"]);
    assert!(engine.is_running());
    assert_eq!(engine.index_ids(), [IndexId::new("b"), IndexId::new("a")]);
    let mut planned = engine.planner().indexes();
    planned.sort_by(|x, y| x.as_str().cmp(y.as_str()));
    assert_eq!(planned, [IndexId::new("a"), IndexId::new("b")]);
    let got = engine.index(&IndexId::new("b")).unwrap();
    let expected: Arc<dyn Index> = first.clone();
    assert!(Arc::ptr_eq(& got, & expected));
    assert!(engine.index(& IndexId::new("zzz")).is_none());
    assert_eq!(first.calls() + second.calls(), 0);
    assert_eq!(engine.is_index_loaded(& IndexId::new("a")), Some(false));
    assert_eq!(engine.is_index_loaded(& IndexId::new("b")), Some(false));
    assert_eq!(engine.is_index_loaded(& IndexId::new("zzz")), None);
    assert_eq!(cpu.activations() + gpu.activations() + shared.activations(), 0);
    shared.loaded.store(true, Ordering::SeqCst);
    assert_eq!(engine.is_index_loaded(& IndexId::new("a")), Some(true));
    assert_eq!(first.calls() + second.calls(), 0);
}
#[test]
fn factory_failures_are_reported() {
    let late = LateSink::new();
    let e = start_error(
        base(&late).subengine_with("gpu", Box::new(|_| Err("boom".to_string()))).start(),
    );
    assert_eq!(
        e, EngineError::Subengine { name : "gpu".to_string(), reason : "boom"
        .to_string(), }
    );
    let log = new_log();
    let probe = Arc::new(
        Probe::new("other", QueueKind::Cpu, &[ResourceKind::Tokenizer], &log),
    );
    let e = start_error(
        base(&late)
            .subengine_with("gpu", Box::new(move |_| Ok(probe as Arc<dyn Subengine>)))
            .start(),
    );
    assert!(matches!(e, EngineError::Subengine { ref name, .. } if name == "gpu"));
    assert!(entries(& log).is_empty());
}
#[test]
fn factories_share_the_engine_queues() {
    let late = LateSink::new();
    let seen = Arc::new(Mutex::new(Vec::new()));
    let sink = Arc::clone(&seen);
    let log = new_log();
    let probe = Arc::new(
        Probe::new("cpu", QueueKind::Cpu, &[ResourceKind::Tokenizer], &log),
    );
    let engine = base(&late)
        .subengine_with(
            "cpu",
            Box::new(move |wiring| {
                lock(&sink)
                    .push((
                        wiring.ops_queue(QueueKind::Cpu),
                        wiring.ops_queue(QueueKind::Cpu),
                    ));
                lock(&sink)
                    .push((
                        wiring.ops_queue(QueueKind::Gpu),
                        wiring.ops_queue(QueueKind::Shared),
                    ));
                Ok(probe as Arc<dyn Subengine>)
            }),
        )
        .start()
        .unwrap();
    let seen = lock(&seen);
    assert!(Arc::ptr_eq(& seen[0].0, & seen[0].1));
    assert!(! Arc::ptr_eq(& seen[1].0, & seen[1].1));
    assert!(Arc::ptr_eq(& seen[0].0, & engine.wiring().ops_queue(QueueKind::Cpu)));
    assert_eq!(entries(& log), ["cpu:start"]);
}
#[test]
fn failed_planner_start_aborts() {
    let log = new_log();
    let probe = Probe::new("cpu", QueueKind::Cpu, &[ResourceKind::Tokenizer], &log)
        .failing_start("nope");
    let late = LateSink::new();
    let e = start_error(base(&late).subengine(Arc::new(probe)).start());
    assert!(matches!(e, EngineError::Planner(PlannerConfigError::StartFailed { .. })));
}
#[test]
fn deferred_subengine_forwards_after_binding_only() {
    let log = new_log();
    let probe = Arc::new(
        Probe::new("cpu", QueueKind::Cpu, &[ResourceKind::Tokenizer], &log),
    );
    let wrapper = DeferredSubengine::new("cpu".to_string());
    assert_eq!(wrapper.name(), "cpu");
    assert_eq!(wrapper.start(), Err("nicht gebunden".to_string()));
    assert_eq!(wrapper.activate(), Err("nicht gebunden".to_string()));
    assert!(wrapper.offers().is_empty());
    assert!(wrapper.capabilities().is_empty());
    assert!(! wrapper.is_loaded(ResourceKind::Index, None));
    wrapper.bind(probe.clone());
    assert_eq!(wrapper.offers(), [ResourceKind::Tokenizer]);
    assert_eq!(wrapper.capabilities().len(), 1);
    assert_eq!(wrapper.start(), Ok(()));
    assert_eq!(wrapper.activate(), Ok(()));
    assert_eq!(probe.activations(), 1);
    probe.loaded.store(true, Ordering::SeqCst);
    assert!(wrapper.is_loaded(ResourceKind::Index, None));
    wrapper.unbind();
    assert_eq!(wrapper.start(), Err("nicht gebunden".to_string()));
}
#[test]
fn late_sink_binds_once() {
    let late: LateSink<TaskSpec, String> = LateSink::new();
    assert!(! late.is_bound());
    let engine = start_with(&[]);
    assert!(late.bind(engine.sink()));
    assert!(late.is_bound());
    assert!(! late.clone().bind(engine.sink()));
}
#[test]
fn enqueue_returns_enqueued_tasks_and_records_first_kind() {
    let (engine, cpu, _release) = blocked_fixture();
    let first = engine.enqueue_embedding(embedding_spec(), params(1)).unwrap();
    wait_until(|| cpu.activations() >= 1);
    assert_eq!(first.state(), TaskState::Claimed);
    let generic = engine.enqueue(foreign_spec(), params(2)).unwrap();
    let embedding = engine.enqueue_embedding(embedding_spec(), params(3)).unwrap();
    let similarity = engine.enqueue_similarity(similarity_spec(), params(4)).unwrap();
    for (task, priority) in [(&generic, 2), (&embedding, 3), (&similarity, 4)] {
        assert_eq!(task.state(), TaskState::Enqueued);
        assert_eq!(task.params().priority, params(priority).priority);
    }
    assert_eq!(
        pending_first_kinds(& engine), [None, Some(ResourceKind::Tokenizer), None]
    );
}
#[test]
fn enqueued_task_is_expanded_by_a_worker() {
    let log = new_log();
    let probes = trio(&log);
    let engine = start_with(&probes);
    let task = engine.enqueue_embedding(embedding_spec(), params(1)).unwrap();
    wait_until(|| task.state() == TaskState::Claimed);
    assert_eq!(engine.wiring().ops_queue(QueueKind::Cpu).len(), 1);
    let log = entries(&log);
    let activations: Vec<_> = log.iter().filter(|e| e.ends_with(":activate")).collect();
    assert_eq!(activations, ["cpu:activate", "shared:activate"]);
    assert!(engine.processor.task_queue().is_empty());
}
#[test]
fn expansion_error_completes_a_claimed_task() {
    let log = new_log();
    let engine = start_with(&trio(&log));
    let task = engine.enqueue(foreign_spec(), params(1)).unwrap();
    wait_until(|| task.state() == TaskState::Completed);
    match task.result() {
        Some(Err(TaskError::Failed(message))) => assert!(message.starts_with("plan:")),
        _ => panic!("expected a plan error"),
    }
    assert_eq!(entries(& log).iter().filter(| e | e.ends_with(":activate")).count(), 0);
}
#[test]
fn cancelled_task_is_not_expanded() {
    let (engine, cpu, release) = blocked_fixture();
    let first = engine.enqueue_embedding(embedding_spec(), params(1)).unwrap();
    wait_until(|| cpu.activations() >= 1);
    let second = engine.enqueue_embedding(embedding_spec(), params(1)).unwrap();
    assert!(second.cancel());
    release.0.release();
    wait_until(|| engine.processor.task_queue().is_empty());
    assert_eq!(second.state(), TaskState::Cancelled);
    assert_eq!(first.state(), TaskState::Claimed);
    assert_eq!(cpu.activations(), 1);
}
#[test]
fn shutdown_cancels_tasks_no_worker_took() {
    let (engine, cpu, _release) = blocked_fixture();
    let first = engine.enqueue_embedding(embedding_spec(), params(1)).unwrap();
    wait_until(|| cpu.activations() >= 1);
    let second = engine.enqueue_embedding(embedding_spec(), params(1)).unwrap();
    let third = engine.enqueue_similarity(similarity_spec(), params(1)).unwrap();
    let gate = Arc::clone(&engine.gate);
    let report = thread::scope(|s| {
        s.spawn(|| {
            wait_until(|| gate.is_closed());
            cpu.release();
        });
        engine.shutdown()
    });
    assert_eq!(report, Ok(ShutdownReport { cancelled_tasks : 2 }));
    assert!(! engine.is_running());
    assert_eq!(first.state(), TaskState::Claimed);
    for task in [&second, &third] {
        assert_eq!(task.state(), TaskState::Cancelled);
        assert!(matches!(task.result(), Some(Err(TaskError::Cancelled))));
    }
    assert!(engine.processor.task_queue().is_empty());
}
#[test]
fn shutdown_is_idempotent_and_closes_the_engine() {
    let log = new_log();
    let engine = start_with(&trio(&log));
    assert_eq!(engine.shutdown(), Ok(ShutdownReport { cancelled_tasks : 0 }));
    assert_eq!(engine.shutdown(), Ok(ShutdownReport { cancelled_tasks : 0 }));
    assert!(! engine.is_running());
    assert!(
        matches!(engine.enqueue(foreign_spec(), params(1)), Err(EngineError::ShutDown))
    );
    assert!(
        matches!(engine.enqueue_embedding(embedding_spec(), params(1)),
        Err(EngineError::ShutDown))
    );
    assert!(
        matches!(engine.enqueue_similarity(similarity_spec(), params(1)),
        Err(EngineError::ShutDown))
    );
    assert!(engine.processor.task_queue().is_empty());
}
#[test]
fn concurrent_shutdowns_all_succeed() {
    let log = new_log();
    let engine = start_with(&trio(&log));
    let reports: Vec<_> = thread::scope(|s| {
        let handles: Vec<_> = (0..4).map(|_| s.spawn(|| engine.shutdown())).collect();
        handles.into_iter().map(|h| h.join().unwrap()).collect()
    });
    assert!(reports.iter().all(| r | r.is_ok()));
    assert!(! engine.is_running());
}
#[test]
fn drop_cancels_pending_tasks() {
    let (engine, cpu, _release) = blocked_fixture();
    engine.enqueue_embedding(embedding_spec(), params(1)).unwrap();
    wait_until(|| cpu.activations() >= 1);
    let pending = engine.enqueue_embedding(embedding_spec(), params(1)).unwrap();
    let gate = Arc::clone(&engine.gate);
    thread::scope(|s| {
        s.spawn(|| {
            wait_until(|| gate.is_closed());
            cpu.release();
        });
        drop(engine);
    });
    assert_eq!(pending.state(), TaskState::Cancelled);
}
#[test]
fn no_accepted_task_is_lost_during_shutdown() {
    let log = new_log();
    let late = LateSink::new();
    let mut builder = base(&late)
        .workers(PoolConfig {
            min_workers: 2,
            max_workers: Some(2),
        });
    for probe in trio(&log) {
        builder = builder.subengine(probe);
    }
    let engine = builder.start().unwrap();
    let tasks: Vec<TestTask> = thread::scope(|s| {
        let handles: Vec<_> = (0..4)
            .map(|_| {
                s.spawn(|| {
                    let mut accepted = Vec::new();
                    loop {
                        match engine.enqueue(foreign_spec(), params(1)) {
                            Ok(task) => accepted.push(task),
                            Err(EngineError::ShutDown) => break accepted,
                            Err(other) => panic!("unexpected error: {other}"),
                        }
                    }
                })
            })
            .collect();
        thread::sleep(Duration::from_millis(20));
        assert!(engine.shutdown().is_ok());
        handles.into_iter().flat_map(|h| h.join().unwrap()).collect()
    });
    assert!(! tasks.is_empty());
    assert!(tasks.iter().all(| t | t.state() != TaskState::Enqueued));
}
fn expansion_parts(
    probes: &[Arc<Probe>],
    started: bool,
) -> (
    Expansion<TaskSpec, StepOutput, String, Step, (), ()>,
    Arc<Proc<TaskSpec, StepOutput, String, Step, (), ()>>,
    Arc<ExpansionControl>,
    Arc<ClosedGate>,
) {
    let late = LateSink::new();
    let mut builder = PlannerBuilder::new(
            Arc::new(ModelRegistry::with_builtin()),
            Arc::new(FixedResolver),
            Arc::new(EngineValues { late }),
        )
        .model(descriptor())
        .unwrap()
        .default_model(E5_SMALL_MODEL_ID)
        .unwrap();
    for probe in probes {
        builder = builder.subengine(probe.clone()).unwrap();
    }
    let planner = Arc::new(builder.build());
    if started {
        planner.start().unwrap();
    }
    let processor: Arc<Proc<TaskSpec, StepOutput, String, Step, (), ()>> = Processor::new(
        PlannerHandle(planner),
    );
    let wake: Wake = Arc::new(|| {});
    let control = Arc::new(ExpansionControl::new());
    let gate = Arc::new(ClosedGate::new());
    let expansion = Expansion {
        processor: Arc::clone(&processor),
        topology: Arc::new(NotificationTopology::new(1, wake)),
        control: Arc::clone(&control),
        gate: Arc::clone(&gate),
        plan_error: Arc::new(|e: &PlanError| format!("plan:{e}")),
    };
    (expansion, processor, control, gate)
}
#[test]
fn expansion_takes_tasks_in_order_and_stops_when_closed_or_empty() {
    let log = new_log();
    let (mut expansion, processor, _control, gate) = expansion_parts(&trio(&log), true);
    assert!(! expansion.expand_next());
    let first = processor.submit(embedding_spec(), params(1), None);
    let second = processor.submit(embedding_spec(), params(1), None);
    assert!(expansion.expand_next());
    assert_eq!(first.state(), TaskState::Claimed);
    assert_eq!(second.state(), TaskState::Enqueued);
    gate.close();
    assert!(! expansion.expand_next());
    assert_eq!(second.state(), TaskState::Enqueued);
    assert_eq!(processor.task_queue().len(), 1);
}
#[test]
fn expansion_prefers_tasks_of_underserved_resources() {
    let log = new_log();
    let (mut expansion, processor, control, _gate) = expansion_parts(&trio(&log), true);
    let plain = processor.submit(embedding_spec(), params(1), None);
    let wanted = processor
        .submit(embedding_spec(), params(1), Some(ResourceKind::Tokenizer));
    let notice = IdleNotice {
        key: "k".to_string(),
        kind: ResourceKind::Scorer,
        requirements: ResourceRequirements::new(
            ResourceKind::Scorer,
            ExecutionEnvironment::Any,
        ),
    };
    control
        .report_iteration(
            ResourceKind::Tokenizer,
            &IterationOutcome {
                idle: vec![notice],
                ..Default::default()
            },
        );
    assert_eq!(control.underserved(), [ResourceKind::Tokenizer]);
    assert!(expansion.expand_next());
    assert_eq!(wanted.state(), TaskState::Claimed);
    assert_eq!(plain.state(), TaskState::Enqueued);
    assert!(expansion.expand_next());
    assert_eq!(plain.state(), TaskState::Claimed);
}
#[test]
fn expansion_without_matching_task_falls_back_to_the_oldest() {
    let log = new_log();
    let (mut expansion, processor, control, _gate) = expansion_parts(&trio(&log), true);
    let oldest = processor.submit(embedding_spec(), params(1), None);
    let newer = processor.submit(embedding_spec(), params(1), None);
    let notice = IdleNotice {
        key: "k".to_string(),
        kind: ResourceKind::Scorer,
        requirements: ResourceRequirements::new(
            ResourceKind::Scorer,
            ExecutionEnvironment::Any,
        ),
    };
    control
        .report_iteration(
            ResourceKind::Tokenizer,
            &IterationOutcome {
                idle: vec![notice],
                ..Default::default()
            },
        );
    assert!(expansion.expand_next());
    assert_eq!(oldest.state(), TaskState::Claimed);
    assert_eq!(newer.state(), TaskState::Enqueued);
}
#[test]
fn expansion_skips_finished_tasks_and_cancels_unclaimed_failures() {
    let log = new_log();
    let (mut expansion, processor, _control, _gate) = expansion_parts(&trio(&log), true);
    let cancelled = processor.submit(embedding_spec(), params(1), None);
    assert!(cancelled.cancel());
    assert!(expansion.expand_next());
    assert_eq!(cancelled.state(), TaskState::Cancelled);
    assert!(entries(& log).iter().all(| e | ! e.ends_with(":activate")));
    assert!(processor.task_queue().is_empty());
    let (mut idle_expansion, idle_processor, _c, _g) = expansion_parts(&[], false);
    let unplanned = idle_processor.submit(embedding_spec(), params(1), None);
    assert!(idle_expansion.expand_next());
    assert_eq!(unplanned.state(), TaskState::Cancelled);
}
