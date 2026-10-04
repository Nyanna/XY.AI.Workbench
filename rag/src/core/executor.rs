//! Rayon-backed CPU executor shared by all layers.
//!
//! `tokio` in this crate schedules I/O-bound/async work; CPU-bound work
//! (parsing, embedding math, hashing, etc.) must not run directly on a
//! tokio worker thread, as it would block other queries' progress.
//! `CpuExecutor` bridges the two: it owns a dedicated rayon thread pool
//! sized to all available CPU cores (rayon's default) and exposes an
//! `async fn spawn` that offloads a closure onto that pool and resolves
//! once it completes, without occupying a tokio worker while the CPU work
//! runs.

use std::sync::Arc;

use anyhow::{anyhow, Result};
use rayon::{ThreadPool, ThreadPoolBuilder};

/// Shared handle to a process-wide rayon thread pool.
///
/// One instance is created by the `Engine` and handed to every layer via
/// `LayerContext`/`BackgroundContext`, so all layers compete for and share
/// the same bounded set of CPU threads rather than each spinning up its
/// own pool.
pub struct CpuExecutor {
    pool: ThreadPool,
}

impl CpuExecutor {
    /// Builds a pool using rayon's default sizing, i.e. one worker thread
    /// per available CPU core - maximal CPU utilization for offloaded
    /// work without oversubscription.
    pub fn new() -> Result<Self> {
        let pool = ThreadPoolBuilder::new()
            .thread_name(|i| format!("xy-rag-cpu-{i}"))
            .build()
            .map_err(|e| anyhow!("failed to build rayon thread pool: {e}"))?;
        Ok(Self { pool })
    }

    /// Number of worker threads backing this pool.
    pub fn num_threads(&self) -> usize {
        self.pool.current_num_threads()
    }

    /// Runs `f` on the rayon pool and awaits its result without blocking
    /// the calling tokio worker thread.
    ///
    /// Panics inside `f` are propagated as an `Err` rather than poisoning
    /// the pool or the caller's task.
    pub async fn spawn<F, R>(&self, f: F) -> Result<R>
    where
        F: FnOnce() -> R + Send + 'static,
        R: Send + 'static,
    {
        let (tx, rx) = tokio::sync::oneshot::channel();
        self.pool.spawn(move || {
            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(f));
            let _ = tx.send(result);
        });
        match rx.await {
            Ok(Ok(r)) => Ok(r),
            Ok(Err(panic)) => {
                let msg = panic
                    .downcast_ref::<&str>()
                    .map(|s| s.to_string())
                    .or_else(|| panic.downcast_ref::<String>().cloned())
                    .unwrap_or_else(|| "CPU task panicked".to_string());
                Err(anyhow!("rayon task panicked: {msg}"))
            }
            Err(_) => Err(anyhow!("rayon task dropped sender without producing a result")),
        }
    }

    /// Runs a rayon `par_iter`/scope-style closure with the pool installed
    /// as the current thread's rayon context, blocking the calling thread
    /// until it completes.
    ///
    /// Intended for use from inside a `CpuExecutor::spawn` closure (i.e.
    /// already off the tokio runtime) when a layer wants fine-grained data
    /// parallelism (e.g. `rayon::prelude::*` iterator methods) rather than
    /// a single offloaded closure.
    pub fn install<F, R>(&self, f: F) -> R
    where
        F: FnOnce() -> R + Send,
        R: Send,
    {
        self.pool.install(f)
    }
}

pub type SharedCpuExecutor = Arc<CpuExecutor>;
