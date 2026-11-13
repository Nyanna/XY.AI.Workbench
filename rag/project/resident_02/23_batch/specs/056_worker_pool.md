# Worker-Pool: Technical Specification (build order 56)

## 1. Task, restrictions and boundaries

**Task:** Provide in Rust a pool of OS threads (workers). The pool does three things:
- It starts a minimum number of idle workers.
- It parks idle workers and wakes them by notification.
- It stops and joins all workers on request.

Each worker serves itself from the queues by repeatedly calling a work function supplied by the Facade. The pool starts its threads through the internal executor.

**Restrictions:**
- `std` only. No `unsafe`. No thread pinning or core affinity. The worker limit only bounds context switches.
- The pool knows no operations, queues, devices or kinds. It sees only the opaque `WorkSource` trait.
- The pool does not execute CPU or GPU operations itself, does not poll events, and does not claim items. Claiming, execution and completion are done inside `WorkSource::run_once`, which the Facade implements.
- No special blocking handling: no timeouts, watchdogs, extra threads or work stealing. If a worker blocks (for example inside the compute library or its thread pool), that blocking is the backpressure.
- Out of scope: scheduling, routing, retry, cancellation, dynamic resizing, restart after stop.

## 2. Context

- The Queue module (`Queue<M,P,R,E>`) is lock-free and unbounded. It offers `take`, `take_where`, `push`, per-queue epochs and `wait_change`. The Processor only owns the queues (task, CPU, GPU, shared). The Facade implements `WorkSource` and chooses compatible items inside `run_once`, for example with `take_where` over `Operation::requirements()`.
- Because a worker must wake on a change in any queue, the pool owns one pool-wide `Notifier`. The owner of the `WorkSource` (the Facade) calls `notify()` after every state change: a push, a completion, a GPU completion event, or a stop request.
- CPU operation: inside `run_once` the worker runs the operation synchronously and completes the claim with the result.
- GPU operation: inside `run_once` the worker only launches the kernel or copy on its `Lane`. It records an `Event` and returns. After the completion notification, any free worker processes the result as a normal item. The Facade arranges this, not the pool.
- Engine-wide worker count = CPU cores − 1 (upper bound).

## 3. Interface

```rust
use std::sync::Arc;
use std::time::Duration;

/// Result of one work attempt.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Step {
    Worked, // an item was handled (or launched); call run_once again at once
    Idle,   // nothing runnable found; park until notified
}

/// Implemented by the Facade. Called concurrently from all workers.
pub trait WorkSource: Send + Sync + 'static {
    /// Try to claim and handle at most one item. May block (natural backpressure).
    /// CPU op: run synchronously and complete. GPU op: launch only, then return.
    /// `worker` is the worker index in 0..worker_count.
    fn run_once(&self, worker: usize) -> Step;
}

/// Pool-wide wake-up channel. Cheap clone (Arc), Send + Sync.
#[derive(Clone)]
pub struct Notifier { /* Arc<(Mutex<u64>, Condvar)> */ }
impl Notifier {
    /// Increment the epoch and wake all parked workers. Never blocks beyond a short lock.
    pub fn notify(&self);
    /// Current epoch (monotonic, wrapping u64).
    pub fn epoch(&self) -> u64;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PoolConfig {
    /// Number of idle workers started at init. Effective value is clamped, see §4.
    pub min_workers: usize,
    /// Engine-wide worker limit. None = max(1, available_parallelism − 1).
    pub max_workers: Option<usize>,
}

#[derive(Debug)]
pub enum PoolError {
    Spawn(std::io::Error), // OS refused to start a thread; already started workers are stopped and joined
}
impl std::fmt::Display for PoolError {}
impl std::error::Error for PoolError {}

/// Default engine-wide limit: max(1, available_parallelism().map_or(1, n) − 1).
pub fn default_max_workers() -> usize;

pub struct WorkerPool { /* notifier, stop flag, join handles */ }
// Send + Sync.

impl WorkerPool {
    /// Start workers. Creates a fresh Notifier. Workers begin idle-checking at once
    /// (they call run_once first, then park).
    pub fn start(cfg: PoolConfig, source: Arc<dyn WorkSource>) -> Result<WorkerPool, PoolError>;

    /// Handle for the WorkSource owner to wake workers after any state change.
    pub fn notifier(&self) -> Notifier;

    /// Number of workers started (fixed after start; 0 after stop).
    pub fn worker_count(&self) -> usize;

    pub fn is_running(&self) -> bool;

    /// Stop on request: set the stop flag, notify all, join all workers. Idempotent.
    /// Blocks until every worker thread has exited.
    pub fn stop(&self);
}
impl Drop for WorkerPool { /* calls stop() */ }
```

## 4. Behavior and invariants

**Sizing**
- `limit = cfg.max_workers.unwrap_or(default_max_workers())`, then `limit = max(limit, 1)`.
- Effective count `n = clamp(cfg.min_workers, 1, limit)`. `start` spawns exactly `n` threads, all idle. The pool never grows or shrinks afterwards.
- The pool sets no thread affinity.

**Worker loop** (per thread, internal but observable)
1. If the stop flag is set, exit.
2. Read `e = notifier.epoch()` before calling `run_once`. This prevents a lost wakeup.
3. Call `run_once(worker)`.
   - On `Worked`, go to 1 at once.
   - On `Idle`, check the stop flag. If it is not set, park on the Notifier's condvar until `epoch != e` or stop is set, then go to 1.
- A `notify()` that happens between reading `e` and parking makes the epoch differ, so the worker does not park and no wakeup is lost.
- Parked workers use no CPU. Spurious condvar wakeups are re-checked against the epoch and do not call `run_once`.
- Every `notify()` wakes all parked workers (`notify_all`). Workers that find nothing return `Idle` and park again.
- Each worker owns its index only. Lanes and other per-worker state are the Facade's concern, keyed by `worker`.

**Execution semantics (contract with the Facade)**
- The pool never wraps or splits `run_once`. A call that blocks keeps its worker occupied. Other workers continue unaffected, and nothing intervenes.
- A panic inside `run_once` is caught (`catch_unwind`). The worker survives and continues the loop. Any `Claim` held at that moment is dropped by unwinding, so the item becomes `Abandoned`. The pool does not report the panic anywhere else.
- `run_once` must return `Idle` only when it found nothing runnable. If it keeps returning `Worked`, the worker loops without parking.

**Stop**
- `stop` sets the flag, calls `notify()`, then joins all workers. A worker currently inside `run_once` finishes that call first (no interruption). `stop` therefore blocks as long as the longest running call.
- No new `run_once` call starts once the worker has seen the stop flag.
- `stop` is idempotent and safe to call from several threads. Later calls return after the first has finished, or immediately if already stopped.
- If `stop` is called from a worker thread (inside `run_once`), that thread is not joined, to avoid self-deadlock. Other workers are joined.
- After `stop`: `is_running() == false`, `worker_count() == 0`, and `notifier().notify()` is still safe and has no effect on workers. A pool cannot be restarted. Create a new `WorkerPool`.
- `Drop` calls `stop`.

**Errors**
- `start` returns `Err(PoolError::Spawn)` if a thread cannot be spawned. In that case all already-started workers are stopped and joined before returning.
- No other function returns an error. Nothing panics on a poisoned lock: the inner state is recovered.

**Concurrency**
- `notify`, `epoch`, `notifier`, `worker_count`, `is_running` and `stop` are callable from any thread concurrently.
- `WorkSource::run_once` is called concurrently from up to `n` threads. The implementor must be thread-safe.
- `Notifier` clones refer to the same channel and may outlive the pool.

## 5. Relied-on elements

From Queue (used by the Facade's `WorkSource`, not by the pool itself):
- `Queue::take` and `Queue::take_where(pred)` claim an item atomically (at most one consumer per item). `Claim::complete(self, Result<R,E>)` finalizes it. Dropping a `Claim` without completing sets the state to `Abandoned`.
- `push` does not notify the pool. The owner of the `WorkSource` (the Facade) must call it after pushes and after completions.

From Operation (informational):
- `Operation::requirements()` is what the Facade filters on inside `run_once`.

From Device/Lane/Event (informational):
- `Lane::record_event() -> Result<Event, LaneError>` is what a GPU launch uses to produce a completion marker. The Facade calls `notify()` when that completion is observed. `Event: Send + Sync`.