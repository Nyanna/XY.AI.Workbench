use super::*;
use std::collections::HashSet;
use std::sync::atomic::AtomicUsize;
use std::sync::Condvar;
use std::thread;
use std::time::Duration;
use crate::resident::l130::l120::l110::a::gpu_completion::TakenVia;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::a::notification_topology::NotificationTopology;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::a::resource_lifecycle::{
    InitRequest, LifecycleConfig, MemoryClass, ResourceLoader, ResourceRegistry,
};
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::a::resource_requirements::{
    ExecutionEnvironment, ResourceRequirements,
};
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::l010::device_lane_event::{
    Event, GpuEventBackend, LaneConfig, LaneKind, StreamBackend,
};
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::l010::queue::{
    State, Ticket,
};
static_assertions::assert_impl_all!(LaneResource < TestMeta, TestPayload >: Send, Sync);
struct TestMeta {
    key: Option<BatchKey>,
    lens: Vec<usize>,
    allowed: bool,
    requirements: ResourceRequirements,
}
impl BatchMeta for TestMeta {
    fn batch_key(&self) -> Option<BatchKey> {
        self.key.clone()
    }
    fn seq_lengths(&self) -> &[usize] {
        &self.lens
    }
}
impl HasResourceRequirements for TestMeta {
    fn resource_requirements(&self) -> &ResourceRequirements {
        &self.requirements
    }
}
struct TestPayload {
    id: u32,
    key: Option<BatchKey>,
    seqs: Vec<Vec<u32>>,
}
impl BatchPayload for TestPayload {
    fn batch_key(&self) -> Option<BatchKey> {
        self.key.clone()
    }
    fn sequences(&self) -> &[Vec<u32>] {
        &self.seqs
    }
}
type Res = LaneResource<TestMeta, TestPayload>;
type Q = Queue<TestMeta, TestPayload, (), ()>;
type Tk = Ticket<TestMeta, TestPayload, (), ()>;
struct Gate {
    open: Mutex<bool>,
    cv: Condvar,
}
impl Gate {
    fn new() -> Self {
        Gate {
            open: Mutex::new(false),
            cv: Condvar::new(),
        }
    }
    fn wait(&self) {
        let mut g = self.open.lock().unwrap();
        while !*g {
            g = self.cv.wait(g).unwrap();
        }
    }
    fn open(&self) {
        *self.open.lock().unwrap() = true;
        self.cv.notify_all();
    }
}
#[derive(Clone, Debug)]
struct RunLog {
    kind: LaneKind,
    ids: Vec<u32>,
    spans: Vec<RowSpan>,
    batches: Option<Vec<Batch>>,
}
struct TestRunner {
    launch: Launch,
    gated: HashSet<u32>,
    gate: Gate,
    entered: AtomicUsize,
    sleep: Vec<(LaneKind, Duration)>,
    panic_run: bool,
    panic_collect: bool,
    runs: Mutex<Vec<RunLog>>,
    collects: Mutex<Vec<(Vec<u32>, Result<(), EventError>)>>,
}
impl TestRunner {
    fn new(launch: Launch) -> Self {
        TestRunner {
            launch,
            gated: HashSet::new(),
            gate: Gate::new(),
            entered: AtomicUsize::new(0),
            sleep: Vec::new(),
            panic_run: false,
            panic_collect: false,
            runs: Mutex::new(Vec::new()),
            collects: Mutex::new(Vec::new()),
        }
    }
    fn runs(&self) -> Vec<RunLog> {
        self.runs.lock().unwrap().clone()
    }
    fn run_ids(&self) -> Vec<Vec<u32>> {
        self.runs().into_iter().map(|r| r.ids).collect()
    }
}
impl BatchRunner<TestPayload> for TestRunner {
    fn run(&self, lane: &mut Lane, batch: &mut ExecBatch<TestPayload>) -> Launch {
        let ids: Vec<u32> = batch.tasks.iter().map(|t| t.id).collect();
        self.runs
            .lock()
            .unwrap()
            .push(RunLog {
                kind: lane.kind(),
                ids: ids.clone(),
                spans: batch.spans.clone(),
                batches: batch.batches.clone(),
            });
        self.entered.fetch_add(1, Ordering::SeqCst);
        if self.panic_run {
            panic!("run failed");
        }
        if self.gated.contains(&ids[0]) {
            self.gate.wait();
        }
        if let Some((_, d)) = self.sleep.iter().find(|(k, _)| *k == lane.kind()) {
            thread::sleep(*d);
        }
        self.launch
    }
    fn collect(
        &self,
        _lane: &mut Lane,
        batch: ExecBatch<TestPayload>,
        result: Result<(), EventError>,
    ) {
        let ids = batch.tasks.iter().map(|t| t.id).collect();
        self.collects.lock().unwrap().push((ids, result));
        if self.panic_collect {
            panic!("collect failed");
        }
    }
}
#[derive(Debug)]
struct MockStream {
    done: Arc<AtomicBool>,
    fail: Arc<AtomicBool>,
}
#[derive(Debug)]
struct MockEvent {
    done: Arc<AtomicBool>,
    fail: Arc<AtomicBool>,
}
impl StreamBackend for MockStream {
    fn record_event(&self) -> Result<Box<dyn GpuEventBackend>, String> {
        Ok(
            Box::new(MockEvent {
                done: self.done.clone(),
                fail: self.fail.clone(),
            }),
        )
    }
}
impl GpuEventBackend for MockEvent {
    fn query(&self) -> Result<bool, String> {
        if self.fail.load(Ordering::SeqCst) {
            return Err("device lost".to_string());
        }
        Ok(self.done.load(Ordering::SeqCst))
    }
    fn synchronize(&self) -> Result<(), String> {
        while !self.done.load(Ordering::SeqCst) {
            thread::yield_now();
        }
        Ok(())
    }
}
fn cpu_lane(kind: LaneKind) -> Lane {
    Lane::new(LaneConfig::preset(kind, DeviceHandle::Cpu), None)
}
fn gpu_lane(done: &Arc<AtomicBool>, fail: &Arc<AtomicBool>) -> Lane {
    let stream = MockStream {
        done: done.clone(),
        fail: fail.clone(),
    };
    Lane::new(
        LaneConfig::preset(LaneKind::Query, DeviceHandle::Gpu(0)),
        Some(Box::new(stream)),
    )
}
fn tokens(token_budget: usize, max_seq_len: usize) -> BatchLimit {
    BatchLimit::Tokens {
        token_budget,
        max_seq_len,
        pad_token_id: 0,
    }
}
struct OnceLoader(Mutex<Option<Arc<Res>>>);
impl ResourceLoader<Res> for OnceLoader {
    fn load(&self) -> Result<Res, String> {
        let arc = self.0.lock().unwrap().take().ok_or("loaded twice")?;
        Arc::try_unwrap(arc).map_err(|_| "resource is shared".to_string())
    }
}
fn lease_from(registry: &ResourceRegistry<Res>) -> Lease<Res> {
    match registry.request_init("lanes", 1, Instant::now()) {
        InitRequest::Ready(lease) => lease,
        _ => panic!("resource not ready"),
    }
}
struct Fx {
    res: Arc<Res>,
    registry: Arc<ResourceRegistry<Res>>,
    own: Arc<Q>,
    shared: Arc<Q>,
    runner: Arc<TestRunner>,
    wakes: Arc<AtomicUsize>,
    completion: Option<Arc<GpuCompletion>>,
}
impl Fx {
    fn lease(&self) -> Lease<Res> {
        lease_from(&self.registry)
    }
    fn exec(&self, queue: &Q) {
        let claim = queue.take().expect("queue is empty");
        self.res.execute(claim, self.lease());
    }
    fn exec_own(&self) {
        self.exec(&self.own);
    }
}
fn build(
    limit: BatchLimit,
    lanes: Vec<Lane>,
    runner: TestRunner,
    with_completion: bool,
) -> Fx {
    let wakes = Arc::new(AtomicUsize::new(0));
    let w = wakes.clone();
    let topology = NotificationTopology::new(
        1,
        Arc::new(move || {
            w.fetch_add(1, Ordering::SeqCst);
        }),
    );
    let notifier = topology.notifier(0).unwrap();
    let completion = with_completion
        .then(|| Arc::new(GpuCompletion::new(notifier.clone())));
    let own = Arc::new(Q::new());
    let shared = Arc::new(Q::new());
    let runner = Arc::new(runner);
    let cfg = LaneResourceConfig {
        own: own.clone(),
        shared: Some(shared.clone()),
        filter: Arc::new(|m: &TestMeta| m.allowed),
        completion: completion.clone(),
        limit,
        notifier,
        kind: ResourceKind::Encoder,
        device: if with_completion { DeviceHandle::Gpu(0) } else { DeviceHandle::Cpu },
    };
    let res = Res::new(cfg, lanes, runner.clone()).unwrap();
    let config = LifecycleConfig::new(
            Duration::from_secs(60),
            Duration::from_secs(120),
            Duration::from_secs(30),
        )
        .unwrap();
    let registry = Arc::new(ResourceRegistry::new(config));
    registry
        .register(
            "lanes".to_string(),
            MemoryClass::Ram,
            Arc::new(OnceLoader(Mutex::new(Some(res)))),
        )
        .unwrap();
    let now = Instant::now();
    let InitRequest::Started(ticket) = registry.request_init("lanes", 1, now) else {
        panic!("load not started");
    };
    ticket.run(now).unwrap();
    let res = lease_from(&registry).handle().clone();
    Fx {
        res,
        registry,
        own,
        shared,
        runner,
        wakes,
        completion,
    }
}
fn cpu_fx(limit: BatchLimit) -> Fx {
    build(
        limit,
        vec![cpu_lane(LaneKind::Query)],
        TestRunner::new(Launch::Finished),
        false,
    )
}
fn entry_with(
    queue: &Q,
    id: u32,
    key: Option<&str>,
    lens: &[usize],
    allowed: bool,
) -> Tk {
    let key = key.map(|k| BatchKey(k.to_string()));
    let meta = TestMeta {
        key: key.clone(),
        lens: lens.to_vec(),
        allowed,
        requirements: ResourceRequirements::new(
            ResourceKind::Encoder,
            ExecutionEnvironment::Any,
        ),
    };
    let seqs = lens.iter().map(|&l| vec![7u32; l]).collect();
    queue.push(meta, TestPayload { id, key, seqs })
}
fn entry(queue: &Q, id: u32, key: Option<&str>, lens: &[usize]) -> Tk {
    entry_with(queue, id, key, lens, true)
}
fn wait_until(cond: impl Fn() -> bool) {
    for _ in 0..2000 {
        if cond() {
            return;
        }
        thread::sleep(Duration::from_millis(1));
    }
    panic!("condition not reached");
}
fn try_new(limit: BatchLimit, lanes: Vec<Lane>) -> Result<Arc<Res>, LaneSetError> {
    let topology = NotificationTopology::new(1, Arc::new(|| {}));
    let cfg = LaneResourceConfig {
        own: Arc::new(Q::new()),
        shared: None,
        filter: Arc::new(|_: &TestMeta| true),
        completion: None,
        limit,
        notifier: topology.notifier(0).unwrap(),
        kind: ResourceKind::Encoder,
        device: DeviceHandle::Cpu,
    };
    Res::new(cfg, lanes, Arc::new(TestRunner::new(Launch::Finished)))
}
#[test]
fn construction_is_validated() {
    let lane = || vec![cpu_lane(LaneKind::Query)];
    assert_eq!(try_new(BatchLimit::Tasks(1), vec![]).err(), Some(LaneSetError::NoLanes));
    assert_eq!(try_new(tokens(0, 8), lane()).err(), Some(LaneSetError::InvalidLimit));
    assert_eq!(try_new(tokens(8, 0), lane()).err(), Some(LaneSetError::InvalidLimit));
    assert_eq!(
        try_new(BatchLimit::Tasks(0), lane()).err(), Some(LaneSetError::InvalidLimit)
    );
    let res = try_new(
            tokens(8, 8),
            vec![cpu_lane(LaneKind::Query), cpu_lane(LaneKind::Insert)],
        )
        .unwrap();
    assert_eq!(res.lane_count(), 2);
    assert_eq!(res.idle_lanes(), 2);
    assert!(res.has_capacity());
}
#[test]
fn unbatchable_seed_runs_alone() {
    let fx = cpu_fx(BatchLimit::Tasks(8));
    entry(&fx.own, 1, None, &[]);
    let t2 = entry(&fx.own, 2, Some("a"), &[]);
    entry(&fx.own, 3, Some("a"), &[]);
    fx.exec_own();
    assert_eq!(fx.runner.run_ids(), vec![vec![1]]);
    assert_eq!(t2.state(), State::Pending);
}
#[test]
fn single_task_limit_runs_alone() {
    let fx = cpu_fx(BatchLimit::Tasks(1));
    entry(&fx.own, 1, Some("a"), &[]);
    let t2 = entry(&fx.own, 2, Some("a"), &[]);
    fx.exec_own();
    assert_eq!(fx.runner.run_ids(), vec![vec![1]]);
    assert_eq!(t2.state(), State::Pending);
}
#[test]
fn task_limit_joins_in_claim_order_and_reports_once() {
    let fx = cpu_fx(BatchLimit::Tasks(3));
    let tickets: Vec<Tk> = (1..=5).map(|i| entry(&fx.own, i, Some("a"), &[2])).collect();
    fx.exec_own();
    assert_eq!(fx.runner.run_ids(), vec![vec![1, 2, 3]]);
    let log = &fx.runner.runs()[0];
    assert!(log.batches.is_none());
    assert_eq!(
        log.spans, vec![RowSpan { first_row : 0, rows : 1 }, RowSpan { first_row : 1,
        rows : 1 }, RowSpan { first_row : 2, rows : 1 },]
    );
    let states: Vec<State> = tickets.iter().map(Ticket::state).collect();
    assert_eq!(
        states, vec![State::Succeeded, State::Succeeded, State::Succeeded,
        State::Pending, State::Pending]
    );
    assert_eq!(fx.wakes.load(Ordering::SeqCst), 1);
    assert_eq!(fx.res.idle_lanes(), 1);
}
#[test]
fn keys_are_not_mixed_and_filter_is_honoured() {
    let fx = cpu_fx(BatchLimit::Tasks(10));
    entry(&fx.own, 1, Some("a"), &[]);
    let other_key = entry(&fx.own, 2, Some("b"), &[]);
    let filtered = entry_with(&fx.own, 3, Some("a"), &[], false);
    entry(&fx.own, 4, Some("a"), &[]);
    fx.exec_own();
    assert_eq!(fx.runner.run_ids(), vec![vec![1, 4]]);
    assert_eq!(other_key.state(), State::Pending);
    assert_eq!(filtered.state(), State::Pending);
}
#[test]
fn own_queue_is_scanned_before_shared_queue() {
    let fx = cpu_fx(BatchLimit::Tasks(10));
    entry(&fx.own, 1, Some("a"), &[]);
    entry(&fx.shared, 2, Some("a"), &[]);
    entry(&fx.own, 3, Some("a"), &[]);
    entry(&fx.shared, 4, Some("a"), &[]);
    fx.exec_own();
    assert_eq!(fx.runner.run_ids(), vec![vec![1, 3, 2, 4]]);
}
#[test]
fn token_budget_rejection_does_not_end_scan() {
    let fx = cpu_fx(tokens(128, 512));
    entry(&fx.own, 1, Some("a"), &[10]);
    let too_many = entry(&fx.own, 2, Some("a"), &[10, 10, 10, 10]);
    entry(&fx.own, 3, Some("a"), &[10]);
    let too_long = entry(&fx.own, 4, Some("a"), &[100]);
    entry(&fx.own, 5, Some("a"), &[]);
    entry(&fx.own, 6, Some("a"), &[10]);
    fx.exec_own();
    let runs = fx.runner.runs();
    assert_eq!(runs[0].ids, vec![1, 3, 5, 6]);
    assert_eq!(
        runs[0].spans, vec![RowSpan { first_row : 0, rows : 1 }, RowSpan { first_row : 1,
        rows : 1 }, RowSpan { first_row : 2, rows : 0 }, RowSpan { first_row : 2, rows :
        1 },]
    );
    let batches = runs[0].batches.clone().unwrap();
    assert_eq!(batches.len(), 1);
    assert_eq!(batches[0].rows(), 3);
    assert_eq!(batches[0].seq_len(), 32);
    let mut positions = batches[0].positions().to_vec();
    positions.sort_unstable();
    assert_eq!(positions, vec![0, 1, 2]);
    assert_eq!(too_many.state(), State::Pending);
    assert_eq!(too_long.state(), State::Pending);
}
#[test]
fn seed_over_budget_is_accepted_and_nothing_joins() {
    let fx = cpu_fx(tokens(64, 512));
    entry(&fx.own, 1, Some("a"), &[10; 5]);
    let row = entry(&fx.own, 2, Some("a"), &[10]);
    entry(&fx.own, 3, Some("a"), &[]);
    fx.exec_own();
    let runs = fx.runner.runs();
    assert_eq!(runs[0].ids, vec![1, 3]);
    let batches = runs[0].batches.clone().unwrap();
    assert_eq!(batches.len(), 3);
    let mut positions: Vec<u32> = batches
        .iter()
        .flat_map(|b| b.positions().to_vec())
        .collect();
    positions.sort_unstable();
    assert_eq!(positions, vec![0, 1, 2, 3, 4]);
    assert_eq!(row.state(), State::Pending);
}
#[test]
fn seed_without_payload_runs_nothing() {
    let fx = cpu_fx(BatchLimit::Tasks(4));
    let t = entry(&fx.own, 1, Some("a"), &[]);
    let mut claim = fx.own.take().unwrap();
    assert!(claim.take_payload().is_some());
    fx.res.execute(claim, fx.lease());
    assert!(fx.runner.runs().is_empty());
    assert_eq!(t.state(), State::Abandoned);
    assert_eq!(fx.res.idle_lanes(), 1);
    assert_eq!(fx.wakes.load(Ordering::SeqCst), 0);
}
#[test]
fn faster_lane_gets_more_work() {
    let mut runner = TestRunner::new(Launch::Finished);
    runner.gated = [1].into();
    runner.sleep = vec![(LaneKind::Insert, Duration::from_millis(1))];
    let fx = build(
        BatchLimit::Tasks(1),
        vec![cpu_lane(LaneKind::Query), cpu_lane(LaneKind::Insert)],
        runner,
        false,
    );
    for id in 1..=3 {
        entry(&fx.own, id, Some("a"), &[1]);
    }
    let claim = fx.own.take().unwrap();
    let lease = fx.lease();
    let res = fx.res.clone();
    let worker = thread::spawn(move || res.execute(claim, lease));
    wait_until(|| fx.runner.entered.load(Ordering::SeqCst) == 1);
    assert_eq!(fx.res.idle_lanes(), 1);
    assert!(fx.res.has_capacity());
    fx.exec_own();
    thread::sleep(Duration::from_millis(60));
    fx.runner.gate.open();
    worker.join().unwrap();
    fx.exec_own();
    let kinds: Vec<LaneKind> = fx.runner.runs().iter().map(|r| r.kind).collect();
    assert_eq!(kinds, vec![LaneKind::Query, LaneKind::Insert, LaneKind::Insert]);
}
#[test]
fn busy_lane_makes_execute_wait() {
    let mut runner = TestRunner::new(Launch::Finished);
    runner.gated = [1].into();
    let fx = build(BatchLimit::Tasks(1), vec![cpu_lane(LaneKind::Query)], runner, false);
    let t1 = entry(&fx.own, 1, Some("a"), &[]);
    let t2 = entry(&fx.own, 2, Some("b"), &[]);
    let first = fx.own.take().unwrap();
    let first_lease = fx.lease();
    let res = fx.res.clone();
    let a = thread::spawn(move || res.execute(first, first_lease));
    wait_until(|| fx.runner.entered.load(Ordering::SeqCst) == 1);
    assert!(! fx.res.has_capacity());
    assert_eq!(fx.res.idle_lanes(), 0);
    let second = fx.own.take().unwrap();
    let second_lease = fx.lease();
    let res = fx.res.clone();
    let b = thread::spawn(move || res.execute(second, second_lease));
    thread::sleep(Duration::from_millis(40));
    assert_eq!(fx.runner.runs().len(), 1);
    assert_eq!(t2.state(), State::Running);
    fx.runner.gate.open();
    a.join().unwrap();
    b.join().unwrap();
    assert_eq!(fx.runner.run_ids(), vec![vec![1], vec![2]]);
    assert_eq!(t1.state(), State::Succeeded);
    assert_eq!(t2.state(), State::Succeeded);
    assert_eq!(fx.res.idle_lanes(), 1);
}
fn gpu_fx(
    limit: BatchLimit,
    runner: TestRunner,
) -> (Fx, Arc<AtomicBool>, Arc<AtomicBool>) {
    let done = Arc::new(AtomicBool::new(false));
    let fail = Arc::new(AtomicBool::new(false));
    let fx = build(limit, vec![gpu_lane(& done, & fail)], runner, true);
    (fx, done, fail)
}
#[test]
fn gpu_batch_is_finished_by_on_finished() {
    let (fx, done, _) = gpu_fx(BatchLimit::Tasks(2), TestRunner::new(Launch::Pending));
    let t1 = entry(&fx.own, 1, Some("a"), &[3]);
    let t2 = entry(&fx.own, 2, Some("a"), &[3]);
    fx.exec_own();
    let completion = fx.completion.clone().unwrap();
    assert_eq!(completion.in_flight(), 1);
    assert_eq!(fx.res.idle_lanes(), 0);
    assert!(! fx.res.has_capacity());
    assert_eq!(t1.state(), State::Running);
    assert_eq!(t2.state(), State::Running);
    assert!(fx.runner.collects.lock().unwrap().is_empty());
    assert_eq!(fx.wakes.load(Ordering::SeqCst), 0);
    assert!(completion.poll().is_empty());
    done.store(true, Ordering::SeqCst);
    let mut finished = completion.poll();
    assert_eq!(finished.len(), 1);
    assert!(fx.res.on_finished(finished.remove(0)).is_ok());
    let collects = fx.runner.collects.lock().unwrap().clone();
    assert_eq!(collects.len(), 1);
    assert_eq!(collects[0].0, vec![1, 2]);
    assert!(collects[0].1.is_ok());
    assert_eq!(t1.state(), State::Succeeded);
    assert_eq!(t2.state(), State::Succeeded);
    assert_eq!(fx.res.idle_lanes(), 1);
    assert_eq!(fx.wakes.load(Ordering::SeqCst), 1);
    assert_eq!(completion.in_flight(), 0);
}
#[test]
fn failed_event_reaches_collect() {
    let (fx, _, fail) = gpu_fx(BatchLimit::Tasks(1), TestRunner::new(Launch::Pending));
    entry(&fx.own, 1, Some("a"), &[]);
    fx.exec_own();
    fail.store(true, Ordering::SeqCst);
    let mut finished = fx.completion.clone().unwrap().poll();
    assert_eq!(finished.len(), 1);
    assert!(fx.res.on_finished(finished.remove(0)).is_ok());
    assert!(fx.runner.collects.lock().unwrap() [0].1.is_err());
    assert_eq!(fx.res.idle_lanes(), 1);
}
#[test]
fn unknown_tag_is_rejected() {
    let fx = cpu_fx(BatchLimit::Tasks(1));
    let finished = FinishedLane {
        lane: cpu_lane(LaneKind::Query),
        event: Event::mock().0,
        tag: u64::MAX,
        result: Ok(()),
        via: TakenVia::Callback,
    };
    assert!(fx.res.on_finished(finished).is_err());
    assert!(fx.runner.collects.lock().unwrap().is_empty());
    assert_eq!(fx.wakes.load(Ordering::SeqCst), 0);
}
#[test]
fn pending_without_tracker_is_finished_inline() {
    let done = Arc::new(AtomicBool::new(true));
    let fail = Arc::new(AtomicBool::new(false));
    let fx = build(
        BatchLimit::Tasks(1),
        vec![gpu_lane(& done, & fail)],
        TestRunner::new(Launch::Pending),
        false,
    );
    let t = entry(&fx.own, 1, Some("a"), &[]);
    fx.exec_own();
    assert_eq!(fx.runner.collects.lock().unwrap().len(), 1);
    assert!(fx.runner.collects.lock().unwrap() [0].1.is_ok());
    assert_eq!(t.state(), State::Succeeded);
    assert_eq!(fx.res.idle_lanes(), 1);
}
#[test]
fn pending_on_lane_without_stream_collects_with_error() {
    let fx = build(
        BatchLimit::Tasks(1),
        vec![cpu_lane(LaneKind::Query)],
        TestRunner::new(Launch::Pending),
        true,
    );
    let t = entry(&fx.own, 1, Some("a"), &[]);
    fx.exec_own();
    assert!(fx.runner.collects.lock().unwrap() [0].1.is_err());
    assert_eq!(t.state(), State::Succeeded);
    assert_eq!(fx.res.idle_lanes(), 1);
    assert_eq!(fx.completion.clone().unwrap().in_flight(), 0);
}
#[test]
fn run_panic_abandons_claims_and_frees_lane() {
    let mut runner = TestRunner::new(Launch::Finished);
    runner.panic_run = true;
    let fx = build(BatchLimit::Tasks(2), vec![cpu_lane(LaneKind::Query)], runner, false);
    let t1 = entry(&fx.own, 1, Some("a"), &[]);
    let t2 = entry(&fx.own, 2, Some("a"), &[]);
    fx.exec_own();
    assert_eq!(t1.state(), State::Abandoned);
    assert_eq!(t2.state(), State::Abandoned);
    assert_eq!(fx.res.idle_lanes(), 1);
    assert_eq!(fx.wakes.load(Ordering::SeqCst), 1);
}
#[test]
fn collect_panic_abandons_claims_and_frees_lane() {
    let mut runner = TestRunner::new(Launch::Pending);
    runner.panic_collect = true;
    let (fx, done, _) = gpu_fx(BatchLimit::Tasks(1), runner);
    let t = entry(&fx.own, 1, Some("a"), &[]);
    fx.exec_own();
    done.store(true, Ordering::SeqCst);
    let mut finished = fx.completion.clone().unwrap().poll();
    assert!(fx.res.on_finished(finished.remove(0)).is_ok());
    assert_eq!(t.state(), State::Abandoned);
    assert_eq!(fx.res.idle_lanes(), 1);
}
#[test]
fn report_tracks_capacity_and_throughput() {
    let mut runner = TestRunner::new(Launch::Finished);
    runner.sleep = vec![(LaneKind::Query, Duration::from_millis(5))];
    let fx = build(BatchLimit::Tasks(1), vec![cpu_lane(LaneKind::Query)], runner, false);
    let before = fx.res.report();
    assert_eq!(before.throughput, 0.0);
    assert_eq!(before.capacity, 1);
    assert_eq!(before.kind, ResourceKind::Encoder);
    assert_eq!(before.device, DeviceHandle::Cpu);
    entry(&fx.own, 1, Some("a"), &[4]);
    fx.exec_own();
    let after = fx.res.report();
    assert!(after.throughput.is_finite() && after.throughput > 0.0);
    assert_eq!(after.capacity, 1);
}
#[test]
fn concurrent_execution_completes_each_entry_once() {
    let fx = build(
        BatchLimit::Tasks(4),
        vec![cpu_lane(LaneKind::Query), cpu_lane(LaneKind::Insert)],
        TestRunner::new(Launch::Finished),
        false,
    );
    let tickets: Vec<Tk> = (0..60).map(|i| entry(&fx.own, i, Some("a"), &[2])).collect();
    let workers: Vec<_> = (0..4)
        .map(|_| {
            let res = fx.res.clone();
            let own = fx.own.clone();
            let registry = fx.registry.clone();
            thread::spawn(move || {
                while let Some(claim) = own.take() {
                    res.execute(claim, lease_from(&registry));
                }
            })
        })
        .collect();
    for w in workers {
        w.join().unwrap();
    }
    assert!(tickets.iter().all(| t | t.state() == State::Succeeded));
    let mut ids: Vec<u32> = fx.runner.run_ids().into_iter().flatten().collect();
    ids.sort_unstable();
    assert_eq!(ids, (0..60).collect::< Vec < u32 >> ());
    assert_eq!(fx.wakes.load(Ordering::SeqCst), fx.runner.runs().len());
    assert_eq!(fx.res.idle_lanes(), 2);
}
#[test]
fn owns_tag_follows_the_batch_in_flight() {
    let (fx, done, _) = gpu_fx(BatchLimit::Tasks(1), TestRunner::new(Launch::Pending));
    entry(&fx.own, 1, Some("a"), &[]);
    fx.exec_own();
    done.store(true, Ordering::SeqCst);
    let mut finished = fx.completion.clone().unwrap().poll();
    let tag = finished[0].tag;
    assert!(fx.res.owns_tag(tag));
    assert!(! fx.res.owns_tag(tag + 1_000_000));
    assert!(fx.res.on_finished(finished.remove(0)).is_ok());
    assert!(! fx.res.owns_tag(tag));
}
#[test]
fn abort_in_flight_abandons_claims_and_releases_leases() {
    let (fx, done, _) = gpu_fx(BatchLimit::Tasks(2), TestRunner::new(Launch::Pending));
    let t1 = entry(&fx.own, 1, Some("a"), &[]);
    let t2 = entry(&fx.own, 2, Some("a"), &[]);
    fx.exec_own();
    assert_eq!(t1.state(), State::Running);
    let holders = Arc::strong_count(&fx.res);
    assert_eq!(fx.res.abort_in_flight(), 1);
    assert_eq!(Arc::strong_count(& fx.res), holders - 1);
    assert_eq!(t1.state(), State::Abandoned);
    assert_eq!(t2.state(), State::Abandoned);
    assert_eq!(fx.res.abort_in_flight(), 0);
    done.store(true, Ordering::SeqCst);
    let mut finished = fx.completion.clone().unwrap().poll();
    assert!(fx.res.on_finished(finished.remove(0)).is_err());
    assert!(fx.runner.collects.lock().unwrap().is_empty());
}
