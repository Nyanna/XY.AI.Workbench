use std::collections::VecDeque;
use std::fmt;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::{Arc, Condvar, Mutex, MutexGuard};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

fn lock<T>(m: &Mutex<T>) -> MutexGuard<'_, T> {
    m.lock().unwrap_or_else(|e| e.into_inner())
}

#[derive(Debug)]
pub enum PoolError {
    /// Thread creation failed.
    Spawn(std::io::Error),
    /// The pool has been shut down; no new jobs are accepted.
    ShutDown,
}

impl fmt::Display for PoolError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PoolError::Spawn(e) => write!(f, "failed to spawn worker thread: {e}"),
            PoolError::ShutDown => write!(f, "pool has been shut down"),
        }
    }
}

impl std::error::Error for PoolError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            PoolError::Spawn(e) => Some(e),
            PoolError::ShutDown => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JobError {
    /// The job panicked; contains the panic message.
    Panicked(String),
}

impl fmt::Display for JobError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JobError::Panicked(msg) => write!(f, "job panicked: {msg}"),
        }
    }
}

impl std::error::Error for JobError {}

enum Slot<T> {
    Pending,
    Ready(Result<T, JobError>),
    Taken,
}

struct FutureShared<T> {
    slot: Mutex<Slot<T>>,
    cv: Condvar,
}

/// Result container of one job.
pub struct JobFuture<T> {
    shared: Arc<FutureShared<T>>,
}

impl<T: Send + 'static> JobFuture<T> {
    pub fn wait(self) -> Result<T, JobError> {
        let mut slot = lock(&self.shared.slot);
        loop {
            match std::mem::replace(&mut *slot, Slot::Taken) {
                Slot::Ready(r) => return r,
                other => *slot = other,
            }
            slot = self.shared.cv.wait(slot).unwrap_or_else(|e| e.into_inner());
        }
    }

    pub fn wait_timeout(self, timeout: Duration) -> Result<Result<T, JobError>, Self> {
        let deadline = Instant::now().checked_add(timeout);
        {
            let mut slot = lock(&self.shared.slot);
            loop {
                match std::mem::replace(&mut *slot, Slot::Taken) {
                    Slot::Ready(r) => return Ok(r),
                    other => *slot = other,
                }
                let remaining = match deadline {
                    Some(d) => {
                        let now = Instant::now();
                        if now >= d {
                            break;
                        }
                        d - now
                    }
                    None => Duration::from_secs(3600),
                };
                slot = self
                    .shared
                    .cv
                    .wait_timeout(slot, remaining)
                    .unwrap_or_else(|e| e.into_inner())
                    .0;
            }
        }
        Err(self)
    }

    pub fn try_get(&mut self) -> Option<Result<T, JobError>> {
        let mut slot = lock(&self.shared.slot);
        match std::mem::replace(&mut *slot, Slot::Taken) {
            Slot::Ready(r) => Some(r),
            other => {
                *slot = other;
                None
            }
        }
    }

    pub fn is_done(&self) -> bool {
        !matches!(*lock(&self.shared.slot), Slot::Pending)
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;

struct QueueState {
    jobs: VecDeque<Job>,
    closed: bool,
}

struct PoolShared {
    state: Mutex<QueueState>,
    cv: Condvar,
}

/// Named pool of low-priority worker threads.
pub struct BackgroundPool {
    name: String,
    threads: usize,
    shared: Arc<PoolShared>,
    workers: Mutex<Vec<JoinHandle<()>>>,
}

fn panic_message(payload: Box<dyn std::any::Any + Send>) -> String {
    if let Some(s) = payload.downcast_ref::<&str>() {
        (*s).to_string()
    } else if let Some(s) = payload.downcast_ref::<String>() {
        s.clone()
    } else {
        "unknown panic".to_string()
    }
}

#[cfg(target_os = "linux")]
fn lower_priority() {
    // SAFETY: gettid takes no arguments; setpriority only changes this thread's nice value.
    let rc = unsafe {
        let tid = libc::syscall(libc::SYS_gettid) as libc::id_t;
        libc::setpriority(libc::PRIO_PROCESS, tid, 19)
    };
    if rc != 0 {
        log::warn!(
            "failed to set nice value 19 for background worker: {}",
            std::io::Error::last_os_error()
        );
    }
}

#[cfg(not(target_os = "linux"))]
fn lower_priority() {
    log::warn!("lowering background worker priority is not supported on this platform");
}

fn worker_loop(shared: Arc<PoolShared>) {
    lower_priority();
    loop {
        let job = {
            let mut st = lock(&shared.state);
            loop {
                if let Some(j) = st.jobs.pop_front() {
                    break Some(j);
                }
                if st.closed {
                    break None;
                }
                st = shared.cv.wait(st).unwrap_or_else(|e| e.into_inner());
            }
        };
        match job {
            Some(j) => j(),
            None => return,
        }
    }
}

impl BackgroundPool {
    pub fn new(name: &str, threads: usize) -> Result<BackgroundPool, PoolError> {
        let threads = threads.max(1);
        let shared = Arc::new(PoolShared {
            state: Mutex::new(QueueState {
                jobs: VecDeque::new(),
                closed: false,
            }),
            cv: Condvar::new(),
        });
        let pool = BackgroundPool {
            name: name.to_string(),
            threads,
            shared,
            workers: Mutex::new(Vec::with_capacity(threads)),
        };
        for i in 0..threads {
            let s = Arc::clone(&pool.shared);
            let spawned = thread::Builder::new()
                .name(format!("{name}-{i}"))
                .spawn(move || worker_loop(s));
            match spawned {
                Ok(h) => lock(&pool.workers).push(h),
                Err(e) => {
                    pool.shutdown();
                    return Err(PoolError::Spawn(e));
                }
            }
        }
        Ok(pool)
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn thread_count(&self) -> usize {
        self.threads
    }

    pub fn submit<T, F>(&self, job: F) -> Result<JobFuture<T>, PoolError>
    where
        F: FnOnce() -> T + Send + 'static,
        T: Send + 'static,
    {
        let fshared = Arc::new(FutureShared {
            slot: Mutex::new(Slot::Pending),
            cv: Condvar::new(),
        });
        let producer = Arc::clone(&fshared);
        let boxed: Job = Box::new(move || {
            let result = catch_unwind(AssertUnwindSafe(job))
                .map_err(|p| JobError::Panicked(panic_message(p)));
            *lock(&producer.slot) = Slot::Ready(result);
            producer.cv.notify_all();
        });
        {
            let mut st = lock(&self.shared.state);
            if st.closed {
                return Err(PoolError::ShutDown);
            }
            st.jobs.push_back(boxed);
        }
        self.shared.cv.notify_one();
        Ok(JobFuture { shared: fshared })
    }

    pub fn shutdown(&self) {
        lock(&self.shared.state).closed = true;
        self.shared.cv.notify_all();
        let mut workers = lock(&self.workers);
        let current = thread::current().id();
        for h in workers.drain(..) {
            if h.thread().id() == current {
                continue;
            }
            let _ = h.join();
        }
    }
}

impl Drop for BackgroundPool {
    fn drop(&mut self) {
        self.shutdown();
    }
}

#[cfg(test)]
#[path = "test/background_pool.rs"]
mod tests;
