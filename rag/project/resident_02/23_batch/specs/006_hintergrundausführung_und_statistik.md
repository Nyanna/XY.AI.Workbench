# Specification: Hintergrundausführung und Statistik (build order 6)

## 1. Task, restrictions and boundaries

Implement one Rust library module with two independent parts:

1. **Background pools.** Named pools of OS threads run submitted jobs at the lowest scheduling priority. They are separate from any async runtime. Each job returns a result container (future) that can be queried blocking or non-blocking.
2. **Robust statistics.** The module provides the estimators median and minimum, and two dispersion measures: coefficient of variation and range-over-median with the slowest fifth removed.

Restrictions:
- The module has no dependencies on other modules.
- External crates allowed: `libc` (Linux only, `cfg(target_os = "linux")`) and `log`. Nothing else.
- The module must not start or require an async runtime (tokio etc.).
- It does no I/O, has no global state, and defines no configuration files.
- It adds no features beyond those listed (no priorities per job, no cancellation, no work stealing).

## 2. Context

Other modules use this module for two purposes. They run low-priority batch work (such as measurements) off the async runtime. They also reduce timing samples to a representative value and a stability measure. In those samples, system load only makes runs slower, never faster. This is why the minimum and the median are the robust estimators, and why the slowest samples are trimmed.

## 3. Interface

```rust
use std::time::Duration;

// ---------- Pools ----------
#[derive(Debug)]
pub enum PoolError {
    /// Thread creation failed.
    Spawn(std::io::Error),
    /// The pool has been shut down; no new jobs are accepted.
    ShutDown,
}
impl std::fmt::Display for PoolError { /* ... */ }
impl std::error::Error for PoolError {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JobError {
    /// The job panicked; contains the panic message
    /// (&str/String payload, otherwise "unknown panic").
    Panicked(String),
}
impl std::fmt::Display for JobError { /* ... */ }
impl std::error::Error for JobError {}

/// Result container of one job. Send, not Clone.
pub struct JobFuture<T> { /* ... */ }
impl<T: Send + 'static> JobFuture<T> {
    /// Blocks until the job has finished; consumes the future.
    pub fn wait(self) -> Result<T, JobError>;
    /// Blocks up to `timeout`. Returns Err(self) on timeout so the caller can retry.
    pub fn wait_timeout(self, timeout: Duration) -> Result<Result<T, JobError>, Self>;
    /// Non-blocking. Some(result) exactly once when finished; None if still
    /// pending, or if the result has already been taken.
    pub fn try_get(&mut self) -> Option<Result<T, JobError>>;
    /// Non-blocking. True if the job has finished (result may already be taken).
    pub fn is_done(&self) -> bool;
}

/// Named pool of low-priority worker threads. Send + Sync.
pub struct BackgroundPool { /* ... */ }
impl BackgroundPool {
    /// `threads` below 1 is raised to 1. Worker threads are named
    /// "{name}-{index}" (index from 0).
    pub fn new(name: &str, threads: usize) -> Result<BackgroundPool, PoolError>;
    pub fn name(&self) -> &str;
    pub fn thread_count(&self) -> usize;
    pub fn submit<T, F>(&self, job: F) -> Result<JobFuture<T>, PoolError>
    where
        F: FnOnce() -> T + Send + 'static,
        T: Send + 'static;
    /// Stops accepting jobs, lets all already submitted jobs run to the end,
    /// then joins the workers. Idempotent. Takes &self.
    pub fn shutdown(&self);
}
impl Drop for BackgroundPool { /* calls shutdown() */ }

// ---------- Statistics ----------
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Estimator { Median, Minimum }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dispersion { CoefficientOfVariation, TrimmedRange }

/// Minimum number of valid samples for `dispersion`.
pub const MIN_DISPERSION_SAMPLES: usize = 3;

pub fn median(samples: &[f64]) -> Option<f64>;
pub fn minimum(samples: &[f64]) -> Option<f64>;
pub fn estimate(samples: &[f64], estimator: Estimator) -> Option<f64>;
pub fn dispersion(samples: &[f64], kind: Dispersion) -> f64;
```

## 4. Behavior and invariants

### Pools
- `new` starts exactly `max(threads, 1)` worker threads. If a thread cannot be created, `new` returns `PoolError::Spawn`. It first shuts down the workers it already started.
- Workers are independent OS threads and never use an async runtime.
- Priority is set at the start of each worker thread:
  - On Linux the worker sets its own nice value to 19 (`setpriority(PRIO_PROCESS, gettid, 19)` via `libc`, with the tid from `syscall(SYS_gettid)`).
  - If the call fails, or on a non-Linux platform, the worker logs it with `log::warn!` and continues normally. This is never fatal.
- Jobs are taken from one shared FIFO queue. Each job runs on exactly one worker.
- With one thread, jobs run in submission order.
- `submit` never blocks on job execution. The queue is unbounded.
- A panic in a job is caught (`catch_unwind(AssertUnwindSafe(..))`).
  - The future yields `Err(JobError::Panicked(msg))`.
  - The worker survives and takes the next job.
  - The pool keeps its full thread count.
- Dropping a `JobFuture` does not cancel the job. The job still runs and its result is discarded.
- `try_get` returns `Some` once. After that it returns `None`. `wait` and `wait_timeout` consume the future, so they cannot be called after `try_get` has taken the result.
- After `shutdown` begins, `submit` returns `Err(PoolError::ShutDown)`.
- Jobs accepted before that point are all executed (drain). `shutdown` returns only after all of them have finished and the workers have been joined.
- Concurrent `shutdown` calls are safe. Each returns after the drain is complete.
- `shutdown` must not be called from inside a job of the same pool, because it would deadlock. Implementations may skip joining the calling thread.
- `Drop` performs the same drain.
- `submit`, `shutdown` and `name` can be called concurrently from multiple threads (`BackgroundPool: Send + Sync`).

### Statistics
- All functions first discard non-finite samples (NaN, ±∞). Only the remaining "valid" samples count. The input slice is not modified.
- `median`:
  - Returns `None` if there are no valid samples.
  - For an even count it returns the mean of the two middle values.
- `minimum`: returns the smallest valid sample, or `None` if there are none.
- `estimate` dispatches to `median` or `minimum`.
- `dispersion` returns `f64::INFINITY` in each of these cases:
  - There are fewer than `MIN_DISPERSION_SAMPLES` (3) valid samples.
  - The center is not positive (center ≤ 0).
  - The result is not finite.
- `Dispersion::CoefficientOfVariation`:
  - The result is `s / mean`, where `s` is the sample standard deviation (divisor n−1).
  - The center is the mean.
- `Dispersion::TrimmedRange`:
  1. Sort the valid samples ascending.
  2. Drop the slowest (largest) `floor(n/5)` samples. The kept count is `n − floor(n/5)`.
  3. The result is `(max_kept − min_kept) / median_kept`.
  - The center is `median_kept`.
- Dispersion is dimensionless and non-negative. Identical positive samples give `0.0`.
- All statistics functions are pure and thread-safe.

## 5. Relied-on elements

None.