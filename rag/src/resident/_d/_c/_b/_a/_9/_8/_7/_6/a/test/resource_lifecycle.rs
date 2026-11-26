use super::*;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::b::task_model::{
    TaskError, TaskParams, TaskState,
};
const RAM_TTL: Duration = Duration::from_secs(10);
const VRAM_TTL: Duration = Duration::from_secs(60);
const BACKOFF: Duration = Duration::from_secs(5);
struct CountingLoader {
    calls: AtomicUsize,
    fail: Mutex<Option<String>>,
    panic: bool,
}
impl CountingLoader {
    fn ok() -> Arc<Self> {
        Arc::new(Self {
            calls: AtomicUsize::new(0),
            fail: Mutex::new(None),
            panic: false,
        })
    }
    fn failing(message: &str) -> Arc<Self> {
        Arc::new(Self {
            calls: AtomicUsize::new(0),
            fail: Mutex::new(Some(message.to_string())),
            panic: false,
        })
    }
    fn panicking() -> Arc<Self> {
        Arc::new(Self {
            calls: AtomicUsize::new(0),
            fail: Mutex::new(None),
            panic: true,
        })
    }
    fn calls(&self) -> usize {
        self.calls.load(Ordering::SeqCst)
    }
}
impl ResourceLoader<u32> for CountingLoader {
    fn load(&self) -> Result<u32, String> {
        let n = self.calls.fetch_add(1, Ordering::SeqCst) as u32;
        if self.panic {
            panic!("boom");
        }
        match self.fail.lock().clone() {
            Some(message) => Err(message),
            None => Ok(n),
        }
    }
}
fn config() -> LifecycleConfig {
    LifecycleConfig::new(RAM_TTL, VRAM_TTL, BACKOFF).unwrap()
}
fn registry_with(
    key: &str,
    class: MemoryClass,
    loader: Arc<CountingLoader>,
) -> ResourceRegistry<u32> {
    let registry = ResourceRegistry::new(config());
    registry.register(key.to_string(), class, loader).unwrap();
    registry
}
fn load(registry: &ResourceRegistry<u32>, key: &str, now: Instant) {
    match registry.request_init(key, 0, now) {
        InitRequest::Started(ticket) => ticket.run(now).unwrap(),
        _ => panic!("expected Started"),
    }
}
#[test]
fn config_requires_vram_ttl_longer_than_ram_ttl() {
    assert_eq!(
        LifecycleConfig::new(RAM_TTL, RAM_TTL, BACKOFF),
        Err(LifecycleError::InvalidConfig)
    );
    assert_eq!(
        LifecycleConfig::new(VRAM_TTL, RAM_TTL, BACKOFF),
        Err(LifecycleError::InvalidConfig)
    );
    assert!(LifecycleConfig::new(RAM_TTL, VRAM_TTL, BACKOFF).is_ok());
}
#[test]
fn register_starts_uninitialized_and_loads_nothing() {
    let loader = CountingLoader::ok();
    let registry = registry_with("m", MemoryClass::Ram, loader.clone());
    assert_eq!(registry.state("m"), Some(ResourceState::Uninitialized));
    assert!(registry.is_available("m"));
    assert!(! registry.is_loaded("m"));
    assert_eq!(loader.calls(), 0);
    assert_eq!(registry.state("other"), None);
}
#[test]
fn register_twice_is_rejected() {
    let registry = registry_with("m", MemoryClass::Ram, CountingLoader::ok());
    assert_eq!(
        registry.register("m".into(), MemoryClass::Vram, CountingLoader::ok()),
        Err(LifecycleError::AlreadyRegistered("m".into()))
    );
}
#[test]
fn unknown_key_errors() {
    let registry: ResourceRegistry<u32> = ResourceRegistry::new(config());
    let now = Instant::now();
    assert_eq!(
        registry.set_enabled("x", false), Err(LifecycleError::UnknownResource("x"
        .into()))
    );
    assert_eq!(
        registry.report_failure("x", 1, "r", now),
        Err(LifecycleError::UnknownResource("x".into()))
    );
    assert!(matches!(registry.request_init("x", 1, now), InitRequest::Unavailable));
    assert!(! registry.is_available("x"));
}
#[test]
fn first_request_starts_and_run_makes_ready() {
    let loader = CountingLoader::ok();
    let registry = registry_with("m", MemoryClass::Ram, loader.clone());
    let now = Instant::now();
    let ticket = match registry.request_init("m", 1, now) {
        InitRequest::Started(ticket) => ticket,
        _ => panic!("expected Started"),
    };
    assert_eq!(registry.state("m"), Some(ResourceState::Loading));
    assert!(matches!(registry.request_init("m", 2, now), InitRequest::Loading));
    ticket.run(now).unwrap();
    assert_eq!(registry.state("m"), Some(ResourceState::Ready));
    assert!(registry.is_loaded("m"));
    assert_eq!(loader.calls(), 1);
    match registry.request_init("m", 3, now) {
        InitRequest::Ready(lease) => assert_eq!(** lease.handle(), 0),
        _ => panic!("expected Ready"),
    }
    assert_eq!(loader.calls(), 1);
}
#[test]
fn dropped_ticket_returns_to_uninitialized() {
    let loader = CountingLoader::ok();
    let registry = registry_with("m", MemoryClass::Ram, loader.clone());
    let now = Instant::now();
    match registry.request_init("m", 1, now) {
        InitRequest::Started(ticket) => drop(ticket),
        _ => panic!("expected Started"),
    }
    assert_eq!(registry.state("m"), Some(ResourceState::Uninitialized));
    assert_eq!(loader.calls(), 0);
    assert!(matches!(registry.request_init("m", 2, now), InitRequest::Started(_)));
}
#[test]
fn failed_load_enters_backoff_and_retries_after_expiry() {
    let loader = CountingLoader::failing("disk gone");
    let registry = registry_with("m", MemoryClass::Ram, loader.clone());
    let t0 = Instant::now();
    let result = match registry.request_init("m", 1, t0) {
        InitRequest::Started(ticket) => ticket.run(t0),
        _ => panic!("expected Started"),
    };
    assert_eq!(result, Err(LifecycleError::Failed("disk gone".into())));
    assert_eq!(registry.state("m"), Some(ResourceState::Backoff));
    assert!(registry.is_available("m"));
    match registry.request_init("m", 2, t0 + BACKOFF - Duration::from_millis(1)) {
        InitRequest::Backoff { until } => assert_eq!(until, t0 + BACKOFF),
        _ => panic!("expected Backoff"),
    }
    assert_eq!(loader.calls(), 1);
    match registry.request_init("m", 3, t0 + BACKOFF) {
        InitRequest::Started(ticket) => {
            assert_eq!(registry.state("m"), Some(ResourceState::Loading));
            drop(ticket);
        }
        _ => panic!("expected Started after backoff"),
    }
}
#[test]
fn loader_panic_is_treated_as_error() {
    let loader = CountingLoader::panicking();
    let registry = registry_with("m", MemoryClass::Ram, loader);
    let now = Instant::now();
    let result = match registry.request_init("m", 1, now) {
        InitRequest::Started(ticket) => ticket.run(now),
        _ => panic!("expected Started"),
    };
    match result {
        Err(LifecycleError::Failed(message)) => assert!(message.contains("boom")),
        other => panic!("unexpected result {other:?}"),
    }
    assert_eq!(registry.state("m"), Some(ResourceState::Backoff));
}
#[test]
fn disabled_resource_is_unavailable_and_keeps_state() {
    let registry = registry_with("m", MemoryClass::Ram, CountingLoader::ok());
    let now = Instant::now();
    load(&registry, "m", now);
    registry.set_enabled("m", false).unwrap();
    assert!(! registry.is_available("m"));
    assert!(registry.is_loaded("m"));
    assert!(matches!(registry.request_init("m", 1, now), InitRequest::Unavailable));
    registry.set_enabled("m", true).unwrap();
    assert!(matches!(registry.request_init("m", 1, now), InitRequest::Ready(_)));
}
#[test]
fn disabled_uninitialized_does_not_change_state() {
    let registry = registry_with("m", MemoryClass::Ram, CountingLoader::ok());
    registry.set_enabled("m", false).unwrap();
    assert!(
        matches!(registry.request_init("m", 1, Instant::now()), InitRequest::Unavailable)
    );
    assert_eq!(registry.state("m"), Some(ResourceState::Uninitialized));
}
#[test]
fn report_failure_enters_backoff_and_keeps_leases_valid() {
    let registry = registry_with("m", MemoryClass::Ram, CountingLoader::ok());
    let t0 = Instant::now();
    load(&registry, "m", t0);
    let lease = match registry.request_init("m", 1, t0) {
        InitRequest::Ready(lease) => lease,
        _ => panic!("expected Ready"),
    };
    registry.report_failure("m", 1, "launch failed", t0).unwrap();
    assert_eq!(registry.state("m"), Some(ResourceState::Backoff));
    assert_eq!(** lease.handle(), 0);
    assert!(registry.is_available("m"));
    match registry.request_init("m", 2, t0) {
        InitRequest::Backoff { until } => assert_eq!(until, t0 + BACKOFF),
        _ => panic!("expected Backoff"),
    }
}
#[test]
fn report_failure_from_uninitialized_is_allowed() {
    let registry = registry_with("m", MemoryClass::Ram, CountingLoader::ok());
    let t0 = Instant::now();
    registry.report_failure("m", 1, "x", t0).unwrap();
    assert_eq!(registry.state("m"), Some(ResourceState::Backoff));
}
#[test]
fn stale_ticket_does_not_overwrite_reported_failure() {
    let registry = registry_with("m", MemoryClass::Ram, CountingLoader::ok());
    let t0 = Instant::now();
    let ticket = match registry.request_init("m", 1, t0) {
        InitRequest::Started(ticket) => ticket,
        _ => panic!("expected Started"),
    };
    registry.report_failure("m", 2, "x", t0).unwrap();
    assert!(ticket.run(t0).is_err());
    assert_eq!(registry.state("m"), Some(ResourceState::Backoff));
}
#[test]
fn eviction_respects_ttl_per_class() {
    let registry = ResourceRegistry::new(config());
    registry.register("ram".into(), MemoryClass::Ram, CountingLoader::ok()).unwrap();
    registry.register("vram".into(), MemoryClass::Vram, CountingLoader::ok()).unwrap();
    let t0 = Instant::now();
    load(&registry, "ram", t0);
    load(&registry, "vram", t0);
    assert!(registry.evict_expired(t0 + RAM_TTL - Duration::from_millis(1)).is_empty());
    assert_eq!(registry.evict_expired(t0 + RAM_TTL), vec!["ram".to_string()]);
    assert_eq!(registry.state("ram"), Some(ResourceState::Uninitialized));
    assert!(registry.is_available("ram"));
    assert!(registry.is_loaded("vram"));
    assert!(registry.evict_expired(t0 + RAM_TTL).is_empty());
    assert_eq!(registry.evict_expired(t0 + VRAM_TTL), vec!["vram".to_string()]);
}
#[test]
fn evicted_resource_reloads_on_next_request() {
    let loader = CountingLoader::ok();
    let registry = registry_with("m", MemoryClass::Ram, loader.clone());
    let t0 = Instant::now();
    load(&registry, "m", t0);
    registry.evict_expired(t0 + RAM_TTL);
    load(&registry, "m", t0 + RAM_TTL);
    assert_eq!(loader.calls(), 2);
    assert!(registry.is_loaded("m"));
}
#[test]
fn live_lease_blocks_eviction_and_release_refreshes_last_used() {
    let registry = registry_with("m", MemoryClass::Ram, CountingLoader::ok());
    let t0 = Instant::now();
    load(&registry, "m", t0);
    let lease = match registry.request_init("m", 1, t0) {
        InitRequest::Ready(lease) => lease,
        _ => panic!("expected Ready"),
    };
    assert!(registry.evict_expired(t0 + RAM_TTL * 5).is_empty());
    let t1 = t0 + RAM_TTL * 5;
    lease.release(t1);
    assert!(registry.evict_expired(t1 + RAM_TTL - Duration::from_millis(1)).is_empty());
    assert_eq!(registry.evict_expired(t1 + RAM_TTL), vec!["m".to_string()]);
}
#[test]
fn plain_drop_keeps_last_used() {
    let registry = registry_with("m", MemoryClass::Ram, CountingLoader::ok());
    let t0 = Instant::now();
    load(&registry, "m", t0);
    let lease = match registry.request_init("m", 1, t0) {
        InitRequest::Ready(lease) => lease,
        _ => panic!("expected Ready"),
    };
    drop(lease);
    assert_eq!(registry.evict_expired(t0 + RAM_TTL), vec!["m".to_string()]);
}
#[test]
fn request_init_refreshes_last_used() {
    let registry = registry_with("m", MemoryClass::Ram, CountingLoader::ok());
    let t0 = Instant::now();
    load(&registry, "m", t0);
    let t1 = t0 + Duration::from_secs(8);
    drop(registry.request_init("m", 1, t1));
    assert!(registry.evict_expired(t0 + RAM_TTL).is_empty());
    assert_eq!(registry.evict_expired(t1 + RAM_TTL).len(), 1);
}
#[test]
fn loading_and_backoff_are_never_evicted() {
    let registry = ResourceRegistry::new(config());
    registry.register("loading".into(), MemoryClass::Ram, CountingLoader::ok()).unwrap();
    registry.register("backoff".into(), MemoryClass::Ram, CountingLoader::ok()).unwrap();
    let t0 = Instant::now();
    let ticket = match registry.request_init("loading", 1, t0) {
        InitRequest::Started(ticket) => ticket,
        _ => panic!("expected Started"),
    };
    registry.report_failure("backoff", 2, "x", t0).unwrap();
    assert!(registry.evict_expired(t0 + VRAM_TTL * 10).is_empty());
    assert_eq!(registry.state("loading"), Some(ResourceState::Loading));
    assert_eq!(registry.state("backoff"), Some(ResourceState::Backoff));
    drop(ticket);
}
#[test]
fn concurrent_requests_yield_exactly_one_started() {
    let loader = CountingLoader::ok();
    let registry = Arc::new(registry_with("m", MemoryClass::Ram, loader.clone()));
    let now = Instant::now();
    let barrier = Arc::new(std::sync::Barrier::new(8));
    let handles: Vec<_> = (0..8)
        .map(|id| {
            let registry = Arc::clone(&registry);
            let barrier = Arc::clone(&barrier);
            thread::spawn(move || {
                barrier.wait();
                match registry.request_init("m", id, now) {
                    InitRequest::Started(ticket) => {
                        barrier.wait();
                        ticket.run(now).unwrap();
                        1usize
                    }
                    InitRequest::Loading => {
                        barrier.wait();
                        0
                    }
                    _ => panic!("unexpected result"),
                }
            })
        })
        .collect();
    let started: usize = handles.into_iter().map(|h| h.join().unwrap()).sum();
    assert_eq!(started, 1);
    assert_eq!(loader.calls(), 1);
    assert!(registry.is_loaded("m"));
}
#[test]
fn slow_loader_does_not_block_other_resources() {
    struct BlockingLoader(Mutex<std::sync::mpsc::Receiver<()>>);
    impl ResourceLoader<u32> for BlockingLoader {
        fn load(&self) -> Result<u32, String> {
            self.0.lock().recv().map_err(|e| e.to_string())?;
            Ok(7)
        }
    }
    let (tx, rx) = std::sync::mpsc::channel();
    let registry = Arc::new(ResourceRegistry::new(config()));
    registry
        .register(
            "slow".into(),
            MemoryClass::Ram,
            Arc::new(BlockingLoader(Mutex::new(rx))),
        )
        .unwrap();
    registry.register("fast".into(), MemoryClass::Ram, CountingLoader::ok()).unwrap();
    let now = Instant::now();
    let worker = {
        let registry = Arc::clone(&registry);
        thread::spawn(move || match registry.request_init("slow", 1, now) {
            InitRequest::Started(ticket) => ticket.run(now).unwrap(),
            _ => panic!("expected Started"),
        })
    };
    while registry.state("slow") != Some(ResourceState::Loading) {
        thread::yield_now();
    }
    load(&registry, "fast", now);
    assert!(registry.is_loaded("fast"));
    assert!(matches!(registry.request_init("slow", 2, now), InitRequest::Loading));
    tx.send(()).unwrap();
    worker.join().unwrap();
    assert!(registry.is_loaded("slow"));
}
type TestTask = Task<u8, u32, String>;
fn claimed_task() -> TestTask {
    let task = TestTask::new(0, TaskParams::default());
    task.claim().unwrap();
    task
}
#[test]
fn fail_batch_completes_every_task_with_error() {
    let batch = vec![(1, claimed_task()), (2, claimed_task())];
    let rejected = fail_batch(&batch, "model missing", |reason| format!("E:{reason}"));
    assert!(rejected.is_empty());
    for (_, task) in &batch {
        assert_eq!(task.state(), TaskState::Completed);
        assert_eq!(
            task.result(), Some(Err(TaskError::Failed(& "E:model missing".to_string())))
        );
    }
}
#[test]
fn fail_batch_reports_rejected_tasks_and_continues() {
    let unclaimed = TestTask::new(0, TaskParams::default());
    let batch = vec![(1, claimed_task()), (2, unclaimed.clone()), (3, claimed_task())];
    let rejected = fail_batch(&batch, "x", |reason| reason.to_string());
    assert_eq!(rejected, vec![(2, CompleteError::NotClaimed(TaskState::Enqueued))]);
    assert_eq!(batch[0].1.state(), TaskState::Completed);
    assert_eq!(batch[2].1.state(), TaskState::Completed);
    assert_eq!(unclaimed.state(), TaskState::Enqueued);
}
#[test]
fn fail_batch_reports_cancelled_task() {
    let task = TestTask::new(0, TaskParams::default());
    assert!(task.cancel());
    let rejected = fail_batch(&[(5, task)], "x", |reason| reason.to_string());
    assert_eq!(rejected, vec![(5, CompleteError::NotClaimed(TaskState::Cancelled))]);
}
#[test]
fn fail_batch_empty_returns_empty() {
    let batch: Vec<(u64, TestTask)> = Vec::new();
    assert!(fail_batch(& batch, "x", | reason | reason.to_string()).is_empty());
}
