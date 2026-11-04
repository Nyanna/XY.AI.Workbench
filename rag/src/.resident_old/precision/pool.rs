//! Dedicated low-priority threads for calibration and CPU kernels; kept apart from the tokio
//! runtime so measurements never occupy its workers.

use std::io;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::thread::JoinHandle;

use crossbeam::channel::{bounded, unbounded, Receiver, Sender};

/// Lowest scheduling priority (`nice 19`) for the calling thread. Returns whether it applied;
/// on platforms without support the thread keeps its priority.
pub fn lower_current_thread_priority() -> bool {
    #[cfg(target_os = "linux")]
    {
        // SAFETY: plain syscalls without pointer arguments.
        unsafe {
            let tid = libc::syscall(libc::SYS_gettid) as libc::id_t;
            libc::setpriority(libc::PRIO_PROCESS, tid, 19) == 0
        }
    }
    #[cfg(not(target_os = "linux"))]
    {
        false
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;

/// Result of a submitted job.
pub struct JobHandle<T>(Receiver<T>);

impl<T> JobHandle<T> {
    /// Blocks for the result; `None` if the job panicked or the pool was dropped first.
    pub fn join(self) -> Option<T> {
        self.0.recv().ok()
    }

    pub fn try_join(&self) -> Option<T> {
        self.0.try_recv().ok()
    }
}

pub struct LowPriorityPool {
    tx: Option<Sender<Job>>,
    workers: Vec<JoinHandle<()>>,
}

impl LowPriorityPool {
    pub fn new(name: &str, threads: usize) -> io::Result<Self> {
        let (tx, rx) = unbounded::<Job>();
        let mut workers = Vec::with_capacity(threads.max(1));
        for i in 0..threads.max(1) {
            let rx = rx.clone();
            let worker = std::thread::Builder::new().name(format!("{name}-{i}")).spawn(move || {
                if !lower_current_thread_priority() {
                    tracing::debug!("thread priority not lowered");
                }
                while let Ok(job) = rx.recv() {
                    // A panicking job must not take the worker down with it.
                    let _ = catch_unwind(AssertUnwindSafe(job));
                }
            })?;
            workers.push(worker);
        }
        Ok(Self { tx: Some(tx), workers })
    }

    pub fn submit<T: Send + 'static>(
        &self,
        job: impl FnOnce() -> T + Send + 'static,
    ) -> JobHandle<T> {
        let (tx, rx) = bounded(1);
        let boxed: Job = Box::new(move || {
            let _ = tx.send(job());
        });
        if let Some(queue) = &self.tx {
            let _ = queue.send(boxed);
        }
        JobHandle(rx)
    }
}

impl Drop for LowPriorityPool {
    fn drop(&mut self) {
        self.tx = None;
        for worker in self.workers.drain(..) {
            let _ = worker.join();
        }
    }
}

#[cfg(test)]
#[path = "test/pool.rs"]
mod tests;
