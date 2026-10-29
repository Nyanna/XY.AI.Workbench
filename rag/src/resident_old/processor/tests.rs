use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};
use parking_lot::Mutex;
use crate::resident::error::{PlannerError, ResidentError};
use crate::resident::facade::{
    EmbeddingParams, Output, Payload, SimilarityParams, TaskError, TaskKind, TaskMeta,
    TaskQueue, TextInput, VectorInput,
};
use crate::resident::op::{EngineSelector, IndexId, OperationNode, ResourceKind, TaskCtx};
use crate::resident::queue::{Queue, TaskState};
use super::ops::testutil::{node, run as new_run};
use crate::resident::memory::{ArenaId, Region};
use crate::resident::op::{Lease, Op, OpDesc, OpKind, OpParams, PinCount, Requirements, Value};
use super::worker::pass;
use super::*;
type TaskStateRef = Arc<TaskState<Output, TaskError>>;
struct TestExpander {
    ops_per_task: usize,
    fail_expand: bool,
    expanded: Mutex<Vec<u64>>,
}
impl TestExpander {
    fn new(ops_per_task: usize) -> Arc<Self> {
        Arc::new(Self {
            ops_per_task,
            fail_expand: false,
            expanded: Mutex::new(vec![]),
        })
    }
}
impl Expander for TestExpander {
    fn first_resource(&self, meta: &TaskMeta) -> Option<ResourceKind> {
        match meta.kind {
            TaskKind::Similarity => Some(ResourceKind::Scorer),
            _ => None,
        }
    }
    fn expand(
        &self,
        meta: &TaskMeta,
        _payload: Payload,
        ctx: &Arc<TaskCtx>,
    ) -> Result<Expansion, ResidentError> {
        self.expanded.lock().push(meta.id);
        if self.fail_expand {
            return Err(
                PlannerError::UnknownModel {
                    model: "m".into(),
                }
                    .into(),
            );
        }
        let n = self.ops_per_task;
        let ops: Vec<Arc<OperationNode>> = (0..n)
            .map(|_| node(ctx, ResourceKind::Tokenizer, EngineSelector::Cpu))
            .collect();
        Ok(Expansion {
            ops,
            collect: Box::new(move || Ok(Output::Scores(vec![n as f32]))),
        })
    }
}
#[derive(Clone, Copy)]
enum Mode {
    Run,
    FailAll,
    FollowUp,
    Idle(Option<ResourceKind>),
}
struct TestEngine {
    side: Side,
    mode: Mode,
    executed: AtomicUsize,
    harvest: AtomicUsize,
    feed: AtomicUsize,
    log: Mutex<Vec<&'static str>>,
}
impl TestEngine {
    fn new(side: Side, mode: Mode) -> Arc<Self> {
        Arc::new(Self {
            side,
            mode,
            executed: AtomicUsize::new(0),
            harvest: AtomicUsize::new(0),
            feed: AtomicUsize::new(0),
            log: Mutex::new(vec![]),
        })
    }
    fn take(counter: &AtomicUsize) -> bool {
        counter
            .try_update(Ordering::AcqRel, Ordering::Acquire, |v| v.checked_sub(1))
            .is_ok()
    }
}
impl Engine for TestEngine {
    fn harvest(&self, _hub: &Hub) -> bool {
        let hit = Self::take(&self.harvest);
        if hit {
            self.log.lock().push("harvest");
        }
        hit
    }
    fn feed(&self, _hub: &Hub) -> bool {
        let hit = Self::take(&self.feed);
        if hit {
            self.log.lock().push("feed");
        }
        hit
    }
    fn step(&self, hub: &Hub) -> Step {
        if let Mode::Idle(kind) = self.mode {
            return Step::Idle(kind);
        }
        let Some(job) = hub.claim(self.side, &|_| true) else {
            return Step::Idle(None);
        };
        self.log.lock().push("step");
        self.executed.fetch_add(1, Ordering::AcqRel);
        match self.mode {
            Mode::FailAll => {
                hub.fail_op(
                    job,
                    PlannerError::UnknownModel {
                        model: "x".into(),
                    }
                        .into(),
                );
            }
            Mode::FollowUp if job.params().requires.resource
                == ResourceKind::Tokenizer => {
                let next = node(
                    job.run.ctx(),
                    ResourceKind::Normalizer,
                    EngineSelector::Cpu,
                );
                hub.complete_op(job, vec![next]);
            }
            _ => hub.complete_op(job, vec![]),
        }
        Step::Worked
    }
}
fn embedding(id: u64) -> (TaskMeta, Payload) {
    (
        TaskMeta {
            id,
            kind: TaskKind::Embedding,
        },
        Payload::Embedding {
            input: TextInput::new(["a"]),
            params: EmbeddingParams::default(),
        },
    )
}
fn similarity(id: u64) -> (TaskMeta, Payload) {
    (
        TaskMeta {
            id,
            kind: TaskKind::Similarity,
        },
        Payload::Similarity {
            input: VectorInput::new(vec![vec![0.0]]),
            params: SimilarityParams::new(IndexId("i".into())),
        },
    )
}
fn submit(tasks: &TaskQueue, (meta, payload): (TaskMeta, Payload)) -> TaskStateRef {
    let state = Arc::new(TaskState::new());
    tasks.push(meta, state.clone(), payload);
    state
}
fn hub(cfg: WorkerConfig, expander: Arc<dyn Expander>) -> (Hub, Arc<TaskQueue>) {
    let wakeups = Wakeups::new();
    let tasks = Arc::new(Queue::new(wakeups.relays()));
    (Hub::new(cfg, tasks.clone(), wakeups, expander), tasks)
}
fn await_result(state: &TaskStateRef) -> Result<Output, TaskError> {
    let deadline = Instant::now() + Duration::from_secs(5);
    while state.result().is_none() {
        assert!(Instant::now() < deadline, "task did not complete");
        std::thread::sleep(Duration::from_millis(1));
    }
    state.result().unwrap().clone()
}
fn start(
    cfg: WorkerConfig,
    expander: Arc<dyn Expander>,
    cpu: Arc<TestEngine>,
) -> (Processor, Arc<TaskQueue>) {
    let wakeups = Wakeups::new();
    let tasks = Arc::new(Queue::new(wakeups.relays()));
    let p = Processor::start(cfg, tasks.clone(), wakeups, expander, cpu, None).unwrap();
    (p, tasks)
}
#[test]
fn worker_threads_run_tasks_to_completion() {
    let cpu = TestEngine::new(Side::Cpu, Mode::Run);
    let (_p, tasks) = start(
        WorkerConfig::new(2, 0).with_cpu_nice(0),
        TestExpander::new(3),
        cpu.clone(),
    );
    let states: Vec<_> = (0..20).map(|i| submit(&tasks, embedding(i))).collect();
    for s in &states {
        assert_eq!(await_result(s).unwrap(), Output::Scores(vec![3.0]));
    }
    assert_eq!(cpu.executed.load(Ordering::Acquire), 60);
}
#[test]
fn first_failing_op_fails_task_and_remaining_ops_are_dropped() {
    let cpu = TestEngine::new(Side::Cpu, Mode::FailAll);
    let (_p, tasks) = start(
        WorkerConfig::new(1, 0).with_cpu_nice(0),
        TestExpander::new(4),
        cpu.clone(),
    );
    let s = submit(&tasks, embedding(1));
    assert!(matches!(await_result(& s), Err(TaskError::Engine(_))));
    std::thread::sleep(Duration::from_millis(50));
    assert_eq!(cpu.executed.load(Ordering::Acquire), 1);
}
#[test]
fn followup_ops_keep_task_open_until_they_finish() {
    let cpu = TestEngine::new(Side::Cpu, Mode::FollowUp);
    let (_p, tasks) = start(
        WorkerConfig::new(1, 0).with_cpu_nice(0),
        TestExpander::new(1),
        cpu.clone(),
    );
    let s = submit(&tasks, embedding(1));
    assert!(await_result(& s).is_ok());
    assert_eq!(cpu.executed.load(Ordering::Acquire), 2);
}
#[test]
fn expansion_failure_completes_task_with_error() {
    let expander = Arc::new(TestExpander {
        ops_per_task: 1,
        fail_expand: true,
        expanded: Mutex::new(vec![]),
    });
    let cpu = TestEngine::new(Side::Cpu, Mode::Run);
    let (_p, tasks) = start(WorkerConfig::new(1, 0).with_cpu_nice(0), expander, cpu);
    let s = submit(&tasks, embedding(1));
    assert!(matches!(await_result(& s), Err(TaskError::Engine(_))));
}
#[test]
fn gpu_ops_without_gpu_workers_fail_the_task() {
    struct GpuExpander;
    impl Expander for GpuExpander {
        fn expand(
            &self,
            _m: &TaskMeta,
            _p: Payload,
            ctx: &Arc<TaskCtx>,
        ) -> Result<Expansion, ResidentError> {
            Ok(Expansion {
                ops: vec![node(ctx, ResourceKind::Encoder, EngineSelector::Gpu)],
                collect: Box::new(|| Ok(Output::Scores(vec![]))),
            })
        }
    }
    let cpu = TestEngine::new(Side::Cpu, Mode::Run);
    let (_p, tasks) = start(
        WorkerConfig::new(1, 0).with_cpu_nice(0),
        Arc::new(GpuExpander),
        cpu,
    );
    let s = submit(&tasks, embedding(1));
    assert!(matches!(await_result(& s), Err(TaskError::Engine(_))));
}
#[test]
fn shutdown_joins_idle_workers() {
    let cpu = TestEngine::new(Side::Cpu, Mode::Run);
    let (mut p, _tasks) = start(
        WorkerConfig::new(3, 0).with_cpu_nice(0),
        TestExpander::new(1),
        cpu,
    );
    std::thread::sleep(Duration::from_millis(20));
    p.shutdown();
}
#[test]
fn pass_prefers_harvest_then_feed_then_step() {
    let cfg = WorkerConfig::new(1, 1).with_min_op_buffer(1);
    let (hub, tasks) = hub(cfg, TestExpander::new(1));
    let engine = TestEngine::new(Side::Cpu, Mode::Run);
    engine.harvest.store(1, Ordering::Release);
    engine.feed.store(1, Ordering::Release);
    submit(&tasks, embedding(1));
    let mut passes = 0;
    while pass(&hub, Side::Cpu, &*engine) {
        passes += 1;
        assert!(passes < 20);
    }
    assert_eq!(* engine.log.lock(), vec!["harvest", "feed", "step"]);
}
#[test]
fn expansion_only_on_hint_or_low_buffer() {
    let cfg = WorkerConfig::new(1, 0).with_min_op_buffer(0);
    let expander = TestExpander::new(1);
    let (hub, tasks) = hub(cfg, expander.clone());
    let quiet = TestEngine::new(Side::Cpu, Mode::Idle(None));
    submit(&tasks, embedding(1));
    assert!(! pass(& hub, Side::Cpu, &* quiet));
    assert!(expander.expanded.lock().is_empty());
    let hungry = TestEngine::new(Side::Cpu, Mode::Idle(Some(ResourceKind::Encoder)));
    assert!(pass(& hub, Side::Cpu, &* hungry));
    assert_eq!(* expander.expanded.lock(), vec![1]);
    assert!(hub.starved(Side::Cpu).snapshot().is_empty());
}
#[test]
fn low_buffer_triggers_expansion_without_hint() {
    let cfg = WorkerConfig::new(1, 0).with_min_op_buffer(2);
    let expander = TestExpander::new(1);
    let (hub, tasks) = hub(cfg, expander.clone());
    let quiet = TestEngine::new(Side::Cpu, Mode::Idle(None));
    submit(&tasks, embedding(1));
    submit(&tasks, embedding(2));
    assert!(pass(& hub, Side::Cpu, &* quiet));
    assert!(pass(& hub, Side::Cpu, &* quiet));
    assert!(! pass(& hub, Side::Cpu, &* quiet));
    assert_eq!(* expander.expanded.lock(), vec![1, 2]);
}
#[test]
fn hint_selects_task_needing_the_starved_resource() {
    let cfg = WorkerConfig::new(1, 0).with_min_op_buffer(0);
    let expander = TestExpander::new(1);
    let (hub, tasks) = hub(cfg, expander.clone());
    submit(&tasks, embedding(1));
    submit(&tasks, similarity(2));
    let scorer_hungry = TestEngine::new(
        Side::Cpu,
        Mode::Idle(Some(ResourceKind::Scorer)),
    );
    assert!(pass(& hub, Side::Cpu, &* scorer_hungry));
    assert_eq!(* expander.expanded.lock(), vec![2]);
}
#[test]
fn pick_of_a_kind_clears_its_hint() {
    let cfg = WorkerConfig::new(1, 0).with_min_op_buffer(1);
    let (hub, tasks) = hub(cfg, TestExpander::new(1));
    submit(&tasks, embedding(1));
    let quiet = TestEngine::new(Side::Cpu, Mode::Idle(None));
    assert!(pass(& hub, Side::Cpu, &* quiet));
    hub.starved(Side::Cpu).mark(ResourceKind::Tokenizer);
    hub.starved(Side::Cpu).mark(ResourceKind::Encoder);
    let job = hub.claim(Side::Cpu, &|_| true).unwrap();
    let snap = hub.starved(Side::Cpu).snapshot();
    assert!(! snap.contains(ResourceKind::Tokenizer));
    assert!(snap.contains(ResourceKind::Encoder));
    hub.complete_op(job, vec![]);
}

#[test]
fn overdue_task_times_out_at_claim() {
    let cfg = WorkerConfig::new(1, 0)
        .with_min_op_buffer(1)
        .with_task_timeout(Duration::ZERO);
    let (hub, tasks) = hub(cfg, TestExpander::new(2));
    let state = submit(&tasks, embedding(1));
    let quiet = TestEngine::new(Side::Cpu, Mode::Idle(None));
    assert!(pass(&hub, Side::Cpu, &*quiet));
    assert!(hub.claim(Side::Cpu, &|_| true).is_none());
    assert!(matches!(state.result().unwrap(), Err(TaskError::Timeout)));
    assert_eq!(hub.queues().total_queued(), 0);
}

fn device_job(gen: u32) -> (OpJob, Arc<TaskRun>) {
    let (run, _state) = new_run(1);
    run.ctx().add_ops(1);
    let params = OpParams::new(Requirements::new(ResourceKind::Scorer, EngineSelector::Cpu));
    let op: Arc<dyn Op> = Arc::new(OpDesc::new(OpKind::Similarity, params));
    let region = Region { arena: ArenaId(0), offset: 0, len: 16, gen };
    let lease = Lease::new(region, PinCount::new(), None);
    let node = OperationNode::new(op, Value::Device(lease), false, run.ctx().clone());
    (OpJob::new(Arc::new(node), run.clone()), run)
}

#[test]
fn current_device_handle_passes_region_check() {
    let (hub, _tasks) = hub(WorkerConfig::new(1, 0), TestExpander::new(1));
    hub.set_region_check(Arc::new(|r| r.gen == 2));
    let (job, _run) = device_job(2);
    hub.queues().push(job).unwrap();
    assert!(hub.claim(Side::Cpu, &|_| true).is_some());
}

#[test]
#[cfg(debug_assertions)]
#[should_panic(expected = "stale device handle")]
fn stale_device_handle_trips_debug_assertion() {
    let (hub, _tasks) = hub(WorkerConfig::new(1, 0), TestExpander::new(1));
    hub.set_region_check(Arc::new(|r| r.gen == 2));
    let (job, _run) = device_job(1);
    hub.queues().push(job).unwrap();
    let _ = hub.claim(Side::Cpu, &|_| true);
}
