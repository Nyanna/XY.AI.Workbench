use super::*;
use std::collections::HashSet;
use std::sync::atomic::AtomicU64;
use std::time::{Duration, Instant};
struct Counting {
    calls: AtomicU64,
    budget: AtomicU64,
    seen: Mutex<HashSet<usize>>,
}
impl Counting {
    fn new(budget: u64) -> Arc<Self> {
        Arc::new(Counting {
            calls: AtomicU64::new(0),
            budget: AtomicU64::new(budget),
            seen: Mutex::new(HashSet::new()),
        })
    }
}
impl WorkSource for Counting {
    fn run_once(&self, worker: usize) -> Step {
        self.calls.fetch_add(1, Ordering::SeqCst);
        lock(&self.seen).insert(worker);
        let ok = self
            .budget
            .try_update(Ordering::SeqCst, Ordering::SeqCst, |b| b.checked_sub(1))
            .is_ok();
        if ok { Step::Worked } else { Step::Idle }
    }
}
fn cfg(min: usize, max: Option<usize>) -> PoolConfig {
    PoolConfig {
        min_workers: min,
        max_workers: max,
    }
}
fn wait_until(f: impl Fn() -> bool) {
    let t = Instant::now();
    while !f() {
        assert!(t.elapsed() < Duration::from_secs(10), "timeout");
        thread::sleep(Duration::from_millis(2));
    }
}
#[test]
fn sizing_is_clamped() {
    let p = WorkerPool::start(cfg(0, Some(4)), Counting::new(0)).unwrap();
    assert_eq!(p.worker_count(), 1);
    let p = WorkerPool::start(cfg(10, Some(3)), Counting::new(0)).unwrap();
    assert_eq!(p.worker_count(), 3);
    let p = WorkerPool::start(cfg(5, Some(0)), Counting::new(0)).unwrap();
    assert_eq!(p.worker_count(), 1);
    assert!(default_max_workers() >= 1);
}
#[test]
fn idle_workers_park_until_notified() {
    let src = Counting::new(0);
    let p = WorkerPool::start(cfg(2, Some(2)), src.clone()).unwrap();
    wait_until(|| src.calls.load(Ordering::SeqCst) >= 2);
    thread::sleep(Duration::from_millis(50));
    let before = src.calls.load(Ordering::SeqCst);
    thread::sleep(Duration::from_millis(100));
    assert_eq!(src.calls.load(Ordering::SeqCst), before);
    p.notifier().notify();
    wait_until(|| src.calls.load(Ordering::SeqCst) >= before + 2);
}
#[test]
fn worked_loops_without_parking() {
    let src = Counting::new(1000);
    let _p = WorkerPool::start(cfg(1, Some(1)), src.clone()).unwrap();
    wait_until(|| src.budget.load(Ordering::SeqCst) == 0);
}
#[test]
fn stop_is_idempotent_and_final() {
    let p = WorkerPool::start(cfg(3, Some(3)), Counting::new(0)).unwrap();
    let n = p.notifier();
    assert!(p.is_running());
    p.stop();
    p.stop();
    assert!(! p.is_running());
    assert_eq!(p.worker_count(), 0);
    n.notify();
}
#[test]
fn concurrent_stop() {
    let p = Arc::new(WorkerPool::start(cfg(3, Some(3)), Counting::new(0)).unwrap());
    let hs: Vec<_> = (0..4)
        .map(|_| {
            let p = Arc::clone(&p);
            thread::spawn(move || p.stop())
        })
        .collect();
    for h in hs {
        h.join().unwrap();
    }
    assert!(! p.is_running());
}
struct Panicky {
    calls: AtomicU64,
}
impl WorkSource for Panicky {
    fn run_once(&self, _worker: usize) -> Step {
        if self.calls.fetch_add(1, Ordering::SeqCst) == 0 {
            panic!("boom");
        }
        Step::Idle
    }
}
#[test]
fn panic_in_run_once_keeps_worker_alive() {
    let src = Arc::new(Panicky {
        calls: AtomicU64::new(0),
    });
    let p = WorkerPool::start(cfg(1, Some(1)), src.clone()).unwrap();
    wait_until(|| src.calls.load(Ordering::SeqCst) >= 2);
    p.notifier().notify();
    wait_until(|| src.calls.load(Ordering::SeqCst) >= 3);
}
struct SelfStopper {
    pool: Mutex<Option<Arc<WorkerPool>>>,
    done: AtomicBool,
}
impl WorkSource for SelfStopper {
    fn run_once(&self, _worker: usize) -> Step {
        let pool = lock(&self.pool).clone();
        if let Some(p) = pool {
            p.stop();
            self.done.store(true, Ordering::SeqCst);
        }
        Step::Idle
    }
}
#[test]
fn stop_from_worker_does_not_deadlock() {
    let src = Arc::new(SelfStopper {
        pool: Mutex::new(None),
        done: AtomicBool::new(false),
    });
    let p = Arc::new(WorkerPool::start(cfg(2, Some(2)), src.clone()).unwrap());
    *lock(&src.pool) = Some(Arc::clone(&p));
    p.notifier().notify();
    wait_until(|| src.done.load(Ordering::SeqCst));
    *lock(&src.pool) = None;
    p.stop();
    assert!(! p.is_running());
}
#[test]
fn notifier_outlives_pool() {
    let p = WorkerPool::start(cfg(1, Some(1)), Counting::new(0)).unwrap();
    let n = p.notifier();
    let e = n.epoch();
    drop(p);
    n.notify();
    assert_ne!(n.epoch(), e);
}
