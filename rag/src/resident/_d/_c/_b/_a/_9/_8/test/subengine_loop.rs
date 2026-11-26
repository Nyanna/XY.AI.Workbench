use super::*;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;
use std::time::Duration;

use parking_lot::Mutex;

use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::a::resource_lifecycle::{
    LifecycleConfig, MemoryClass, ResourceLoader,
};
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::a::resource_requirements::ExecutionEnvironment;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::_1::queue::State;

static_assertions::assert_impl_all!(SubengineLoop<TestMeta, u32, TestHandle>: Send, Sync);

const ENC: &str = "enc";
const SCORE: &str = "score";

struct TestMeta {
    requirements: ResourceRequirements,
    allowed: bool,
}

impl HasResourceRequirements for TestMeta {
    fn resource_requirements(&self) -> &ResourceRequirements {
        &self.requirements
    }
}

fn requirements(kind: ResourceKind) -> ResourceRequirements {
    ResourceRequirements::new(kind, ExecutionEnvironment::Any)
}

fn meta(kind: ResourceKind) -> TestMeta {
    TestMeta { requirements: requirements(kind), allowed: true }
}

struct TestHandle {
    remaining: AtomicUsize,
    log: Arc<Mutex<Vec<u32>>>,
}

impl ResourceExec for TestHandle {
    type Entry = Claim<TestMeta, u32, (), ()>;

    fn has_capacity(&self) -> bool {
        self.remaining.load(Ordering::SeqCst) > 0
    }

    fn execute(&self, mut entry: Self::Entry, lease: Lease<Self>) {
        self.remaining.fetch_sub(1, Ordering::SeqCst);
        let payload = entry.take_payload().unwrap();
        self.log.lock().push(payload);
        entry.complete(Ok(()));
        drop(lease);
    }
}

struct TestLoader {
    capacity: usize,
    fail: bool,
    log: Arc<Mutex<Vec<u32>>>,
}

impl ResourceLoader<TestHandle> for TestLoader {
    fn load(&self) -> Result<TestHandle, String> {
        if self.fail {
            return Err("load failed".to_string());
        }
        Ok(TestHandle {
            remaining: AtomicUsize::new(self.capacity),
            log: self.log.clone(),
        })
    }
}

type Q = Queue<TestMeta, u32, (), ()>;

struct Fixture {
    own: Arc<Q>,
    shared: Arc<Q>,
    registry: Arc<ResourceRegistry<TestHandle>>,
    log: Arc<Mutex<Vec<u32>>>,
    tickets: Arc<Mutex<Vec<LoadTicket<TestHandle>>>>,
    filter_calls: Arc<AtomicUsize>,
    engine: SubengineLoop<TestMeta, u32, TestHandle>,
}

fn registry() -> Arc<ResourceRegistry<TestHandle>> {
    let config =
        LifecycleConfig::new(Duration::from_secs(60), Duration::from_secs(120), Duration::from_secs(30))
            .unwrap();
    Arc::new(ResourceRegistry::new(config))
}

fn resolver() -> ResourceResolver {
    Arc::new(|req: &ResourceRequirements| match req.kind() {
        ResourceKind::Encoder => Some(ENC.to_string()),
        ResourceKind::Scorer => Some(SCORE.to_string()),
        ResourceKind::Index => Some("ghost".to_string()),
        _ => None,
    })
}

fn build(capacity: usize, fail: bool, with_slots: bool, filter: PullFilter<TestMeta>) -> Fixture {
    let own: Arc<Q> = Arc::new(Queue::new());
    let shared: Arc<Q> = Arc::new(Queue::new());
    let registry = registry();
    let log = Arc::new(Mutex::new(Vec::new()));
    for key in [ENC, SCORE] {
        let loader = Arc::new(TestLoader { capacity, fail, log: log.clone() });
        registry.register(key.to_string(), MemoryClass::Ram, loader).unwrap();
    }
    let tickets: Arc<Mutex<Vec<LoadTicket<TestHandle>>>> = Arc::new(Mutex::new(Vec::new()));
    let sink = tickets.clone();
    let init: InitStarter<TestHandle> = Arc::new(move |ticket| sink.lock().push(ticket));
    let slots = if with_slots {
        vec![ResourceSlot { key: ENC.to_string(), requirements: requirements(ResourceKind::Encoder) }]
    } else {
        Vec::new()
    };
    let filter_calls = Arc::new(AtomicUsize::new(0));
    let engine = SubengineLoop::new(
        own.clone(),
        Some(shared.clone()),
        registry.clone(),
        slots,
        resolver(),
        filter,
        init,
    );
    Fixture { own, shared, registry, log, tickets, filter_calls, engine }
}

fn allow_flag() -> PullFilter<TestMeta> {
    Arc::new(|m: &TestMeta| m.allowed)
}

fn warm(fixture: &Fixture, now: Instant) {
    for key in [ENC, SCORE] {
        match fixture.registry.request_init(key, 0, now) {
            InitRequest::Started(ticket) => ticket.run(now).unwrap(),
            _ => panic!("expected Started"),
        }
    }
}

#[test]
fn own_queue_before_shared_oldest_first() {
    let f = build(100, false, false, allow_flag());
    let now = Instant::now();
    warm(&f, now);
    f.shared.push(meta(ResourceKind::Encoder), 3);
    f.own.push(meta(ResourceKind::Encoder), 1);
    f.own.push(meta(ResourceKind::Encoder), 2);
    let out = f.engine.run_iteration(now);
    assert_eq!(out.claimed, 3);
    assert!(out.progress());
    assert!(out.idle.is_empty());
    assert_eq!(*f.log.lock(), vec![1, 2, 3]);
}

#[test]
fn skipped_entry_does_not_block_later_entries() {
    let f = build(100, false, false, allow_flag());
    let now = Instant::now();
    warm(&f, now);
    let mut blocked = meta(ResourceKind::Encoder);
    blocked.allowed = false;
    let skipped = f.own.push(blocked, 1);
    f.own.push(meta(ResourceKind::Encoder), 2);
    f.own.push(meta(ResourceKind::Scorer), 3);
    let out = f.engine.run_iteration(now);
    assert_eq!(out.claimed, 2);
    assert_eq!(skipped.state(), State::Pending);
    let mut log = f.log.lock().clone();
    log.sort();
    assert_eq!(log, vec![2, 3]);
}

#[test]
fn unresolved_and_unavailable_entries_stay_pending() {
    let f = build(100, false, false, allow_flag());
    let now = Instant::now();
    warm(&f, now);
    let unresolved = f.own.push(meta(ResourceKind::Tokenizer), 1);
    let ghost = f.own.push(meta(ResourceKind::Index), 2);
    let out = f.engine.run_iteration(now);
    assert_eq!(out.claimed, 0);
    assert_eq!(out.inits_started, 0);
    assert_eq!(unresolved.state(), State::Pending);
    assert_eq!(ghost.state(), State::Pending);
}

#[test]
fn lazy_init_starts_once_and_entry_stays_pending() {
    let f = build(100, false, false, allow_flag());
    let now = Instant::now();
    let ticket = f.own.push(meta(ResourceKind::Encoder), 7);

    let first = f.engine.run_iteration(now);
    assert_eq!(first.inits_started, 1);
    assert_eq!(first.claimed, 0);
    assert_eq!(ticket.state(), State::Pending);
    assert_eq!(f.tickets.lock().len(), 1);

    let second = f.engine.run_iteration(now);
    assert_eq!(second.inits_started, 0);
    assert_eq!(f.tickets.lock().len(), 1);

    let started = f.tickets.lock().pop().unwrap();
    started.run(now).unwrap();
    let third = f.engine.run_iteration(now);
    assert_eq!(third.claimed, 1);
    assert_eq!(ticket.state(), State::Succeeded);
    assert_eq!(*f.log.lock(), vec![7]);
}

#[test]
fn no_lazy_init_without_matching_entry() {
    let f = build(100, false, false, allow_flag());
    let out = f.engine.run_iteration(Instant::now());
    assert_eq!(out.inits_started, 0);
    assert!(f.tickets.lock().is_empty());
    assert!(out.idle.is_empty());
}

#[test]
fn failed_load_leads_to_backoff_skip() {
    let f = build(100, true, false, allow_flag());
    let now = Instant::now();
    let ticket = f.own.push(meta(ResourceKind::Encoder), 1);
    assert_eq!(f.engine.run_iteration(now).inits_started, 1);
    let started = f.tickets.lock().pop().unwrap();
    assert!(started.run(now).is_err());
    let out = f.engine.run_iteration(now);
    assert_eq!(out.inits_started, 0);
    assert_eq!(out.claimed, 0);
    assert_eq!(ticket.state(), State::Pending);
}

#[test]
fn dropped_ticket_returns_resource_to_uninitialized() {
    let f = build(100, false, false, allow_flag());
    let now = Instant::now();
    f.own.push(meta(ResourceKind::Encoder), 1);
    assert_eq!(f.engine.run_iteration(now).inits_started, 1);
    f.tickets.lock().clear();
    assert_eq!(f.engine.run_iteration(now).inits_started, 1);
}

#[test]
fn exhausted_capacity_marks_busy_and_ends_early() {
    let f = build(1, false, true, allow_flag());
    let now = Instant::now();
    warm(&f, now);
    let a = f.own.push(meta(ResourceKind::Encoder), 1);
    let b = f.own.push(meta(ResourceKind::Encoder), 2);
    let c = f.shared.push(meta(ResourceKind::Encoder), 3);
    let out = f.engine.run_iteration(now);
    assert_eq!(out.claimed, 1);
    assert!(out.idle.is_empty());
    assert_eq!(a.state(), State::Succeeded);
    assert_eq!(b.state(), State::Pending);
    assert_eq!(c.state(), State::Pending);
    assert_eq!(f.shared.len(), 1);
}

#[test]
fn busy_key_is_skipped_in_shared_queue_but_others_continue() {
    let f = build(1, false, false, allow_flag());
    let now = Instant::now();
    warm(&f, now);
    f.own.push(meta(ResourceKind::Encoder), 1);
    f.own.push(meta(ResourceKind::Encoder), 2);
    f.shared.push(meta(ResourceKind::Encoder), 3);
    f.shared.push(meta(ResourceKind::Scorer), 4);
    let out = f.engine.run_iteration(now);
    assert_eq!(out.claimed, 2);
    let mut log = f.log.lock().clone();
    log.sort();
    assert_eq!(log, vec![1, 4]);
}

#[test]
fn idle_reported_for_free_slot_without_hit() {
    let calls = Arc::new(AtomicUsize::new(0));
    let counter = calls.clone();
    let filter: PullFilter<TestMeta> = Arc::new(move |_| counter.fetch_add(1, Ordering::SeqCst) == 0);
    let mut f = build(5, false, true, filter);
    f.filter_calls = calls;
    let now = Instant::now();
    warm(&f, now);
    let entry = f.own.push(meta(ResourceKind::Encoder), 1);
    let out = f.engine.run_iteration(now);
    assert_eq!(out.claimed, 0);
    assert!(!out.progress());
    assert_eq!(entry.state(), State::Pending);
    assert_eq!(
        out.idle,
        vec![IdleNotice {
            key: ENC.to_string(),
            kind: ResourceKind::Encoder,
            requirements: requirements(ResourceKind::Encoder),
        }]
    );
    assert!(f.filter_calls.load(Ordering::SeqCst) >= 2);
}

#[test]
fn no_idle_report_when_nothing_visited() {
    let f = build(5, false, true, allow_flag());
    let now = Instant::now();
    warm(&f, now);
    let out = f.engine.run_iteration(now);
    assert!(out.idle.is_empty());
    assert_eq!(out.claimed, 0);
}

#[test]
fn eviction_runs_once_at_end_without_probing() {
    let f = build(5, false, true, allow_flag());
    let t0 = Instant::now();
    warm(&f, t0);
    let out = f.engine.run_iteration(t0 + Duration::from_secs(1));
    assert!(out.evicted.is_empty());
    let out = f.engine.run_iteration(t0 + Duration::from_secs(1000));
    assert_eq!(out.evicted, vec![ENC.to_string(), SCORE.to_string()]);
    assert!(out.idle.is_empty());
}

#[test]
fn eviction_runs_on_early_end() {
    let f = build(1, false, true, allow_flag());
    let t0 = Instant::now();
    warm(&f, t0);
    f.own.push(meta(ResourceKind::Encoder), 1);
    f.own.push(meta(ResourceKind::Encoder), 2);
    let out = f.engine.run_iteration(t0 + Duration::from_secs(1000));
    assert_eq!(out.claimed, 1);
    assert_eq!(out.evicted, vec![SCORE.to_string()]);
}

#[test]
fn concurrent_workers_claim_each_entry_exactly_once() {
    let f = build(10_000, false, true, allow_flag());
    let now = Instant::now();
    warm(&f, now);
    for i in 0..200u32 {
        f.own.push(meta(ResourceKind::Encoder), i);
    }
    for i in 200..400u32 {
        f.shared.push(meta(ResourceKind::Scorer), i);
    }
    let engine = Arc::new(f.engine);
    let total = Arc::new(AtomicUsize::new(0));
    let workers: Vec<_> = (0..4)
        .map(|_| {
            let engine = engine.clone();
            let total = total.clone();
            thread::spawn(move || {
                total.fetch_add(engine.run_iteration(now).claimed, Ordering::SeqCst);
            })
        })
        .collect();
    for worker in workers {
        worker.join().unwrap();
    }
    assert_eq!(total.load(Ordering::SeqCst), 400);
    let mut log = f.log.lock().clone();
    log.sort();
    assert_eq!(log, (0..400).collect::<Vec<u32>>());
    assert!(f.own.is_empty() && f.shared.is_empty());
}
