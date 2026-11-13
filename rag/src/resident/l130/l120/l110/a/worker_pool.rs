//! Fixed-size pool of OS threads that repeatedly call an opaque work function and park when idle.

use std::fmt;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Condvar, Mutex, MutexGuard};
use std::thread::{self, JoinHandle, ThreadId};

/// Result of one work attempt.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Step {
    Worked,
    Idle,
}

/// Implemented by the Processor. Called concurrently from all workers.
pub trait WorkSource: Send + Sync + 'static {
    fn run_once(&self, worker: usize) -> Step;
}

fn lock<T>(m: &Mutex<T>) -> MutexGuard<'_, T> {
    m.lock().unwrap_or_else(|e| e.into_inner())
}

/// Pool-wide wake-up channel.
#[derive(Clone)]
pub struct Notifier {
    inner: Arc<(Mutex<u64>, Condvar)>,
}

impl Notifier {
    fn new() -> Self {
        Notifier {
            inner: Arc::new((Mutex::new(0), Condvar::new())),
        }
    }

    pub fn notify(&self) {
        {
            let mut e = lock(&self.inner.0);
            *e = e.wrapping_add(1);
        }
        self.inner.1.notify_all();
    }

    pub fn epoch(&self) -> u64 {
        *lock(&self.inner.0)
    }

    fn park_while_unchanged(&self, seen: u64, stop: &AtomicBool) {
        let mut guard = lock(&self.inner.0);
        while *guard == seen && !stop.load(Ordering::SeqCst) {
            guard = self
                .inner
                .1
                .wait(guard)
                .unwrap_or_else(|e| e.into_inner());
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PoolConfig {
    pub min_workers: usize,
    pub max_workers: Option<usize>,
}

#[derive(Debug)]
pub enum PoolError {
    Spawn(std::io::Error),
}

impl fmt::Display for PoolError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PoolError::Spawn(e) => write!(f, "failed to spawn worker thread: {e}"),
        }
    }
}

impl std::error::Error for PoolError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            PoolError::Spawn(e) => Some(e),
        }
    }
}

pub fn default_max_workers() -> usize {
    let cpus = thread::available_parallelism().map_or(1, |n| n.get());
    cpus.saturating_sub(1).max(1)
}

struct Shared {
    notifier: Notifier,
    stop: AtomicBool,
    source: Arc<dyn WorkSource>,
}

struct StopState {
    handles: Vec<JoinHandle<()>>,
    started: bool,
    done: bool,
}

pub struct WorkerPool {
    shared: Arc<Shared>,
    count: AtomicUsize,
    worker_ids: Vec<ThreadId>,
    state: Mutex<StopState>,
    finished: Condvar,
}

fn worker_loop(shared: Arc<Shared>, index: usize) {
    loop {
        if shared.stop.load(Ordering::SeqCst) {
            return;
        }
        let epoch = shared.notifier.epoch();
        let step = catch_unwind(AssertUnwindSafe(|| shared.source.run_once(index)));
        match step {
            Ok(Step::Idle) => {
                if shared.stop.load(Ordering::SeqCst) {
                    return;
                }
                shared.notifier.park_while_unchanged(epoch, &shared.stop);
            }
            Ok(Step::Worked) | Err(_) => {}
        }
    }
}

fn stop_and_join(shared: &Shared, handles: Vec<JoinHandle<()>>) {
    shared.stop.store(true, Ordering::SeqCst);
    shared.notifier.notify();
    let me = thread::current().id();
    for h in handles {
        if h.thread().id() == me {
            continue;
        }
        let _ = h.join();
    }
}

impl WorkerPool {
    pub fn start(cfg: PoolConfig, source: Arc<dyn WorkSource>) -> Result<WorkerPool, PoolError> {
        let limit = cfg.max_workers.unwrap_or_else(default_max_workers).max(1);
        let n = cfg.min_workers.clamp(1, limit);
        let shared = Arc::new(Shared {
            notifier: Notifier::new(),
            stop: AtomicBool::new(false),
            source,
        });
        let mut handles = Vec::with_capacity(n);
        for index in 0..n {
            let s = Arc::clone(&shared);
            let spawned = thread::Builder::new()
                .name(format!("rag-worker-{index}"))
                .spawn(move || worker_loop(s, index));
            match spawned {
                Ok(h) => handles.push(h),
                Err(e) => {
                    stop_and_join(&shared, handles);
                    return Err(PoolError::Spawn(e));
                }
            }
        }
        let worker_ids = handles.iter().map(|h| h.thread().id()).collect();
        Ok(WorkerPool {
            shared,
            count: AtomicUsize::new(n),
            worker_ids,
            state: Mutex::new(StopState {
                handles,
                started: false,
                done: false,
            }),
            finished: Condvar::new(),
        })
    }

    pub fn notifier(&self) -> Notifier {
        self.shared.notifier.clone()
    }

    pub fn worker_count(&self) -> usize {
        self.count.load(Ordering::SeqCst)
    }

    pub fn is_running(&self) -> bool {
        !self.shared.stop.load(Ordering::SeqCst)
    }

    pub fn stop(&self) {
        let me = thread::current().id();
        let from_worker = self.worker_ids.contains(&me);
        let handles = {
            let mut st = lock(&self.state);
            if st.started {
                if !from_worker {
                    while !st.done {
                        st = self.finished.wait(st).unwrap_or_else(|e| e.into_inner());
                    }
                }
                return;
            }
            st.started = true;
            std::mem::take(&mut st.handles)
        };
        self.count.store(0, Ordering::SeqCst);
        stop_and_join(&self.shared, handles);
        lock(&self.state).done = true;
        self.finished.notify_all();
    }
}

impl Drop for WorkerPool {
    fn drop(&mut self) {
        self.stop();
    }
}

#[cfg(test)]
#[path = "test/worker_pool.rs"]
mod tests;
