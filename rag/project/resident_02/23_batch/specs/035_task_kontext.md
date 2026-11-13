# Spec: Task-Kontext (module `task_context`, build order 35)

## 1. Task, restrictions and boundaries

Implement, in Rust, the per-task shared context `TaskContext<E>`. It holds:
- ID
- priority
- counter of open operations
- cancel flag
- first error

The module also provides these operations:
- Register follow-up operations before they are enqueued.
- Report completion of one operation.
- Report a failure.

Restrictions:
- The module has no queue, no enqueueing, no workers, no claim logic and no waiting or notification. It performs no `Task` or `EntryState` transitions.
- It does not interpret priority.
- It uses no `unsafe` and never blocks. All operations are lock-free or short-lock, callable from sync and async code.
- External crate: `log`.
- It needs no code from `Task-Modell` or `Queue-Zustandsmodell`. It only fits their conventions (see section 2).

## 2. Context

The system uses one `TaskContext` per task. Every operation of the task holds an `Arc` clone, and operations are queue entries (`Task`/`EntryState`).

How the neighbors interact with this module:
- **Creation:** The queue or operation creator creates the context with the initial number of open operations.
- **Follow-ups:** An operation that produces follow-up operations calls `register_follow_ups(n)` **before** it enqueues them and **before** it calls `complete_operation()` for itself. The queue module appends the follow-ups at the end of the queue.
- **Completion:** A worker calls `complete_operation()` after finishing an operation. A `true` return means the task has completed successfully.
- **Failure:** A worker calls `fail(e)` on an error. This cancels the task.
- **Claim check:** The queue or worker checks `is_cancelled()` when claiming. Operations of a cancelled task must fail the claim. This module only provides the flag. The claim itself is done by `queue_state`.
- **Completion tracking:** Task completion is tracked only by this counter, never by the result of the `queue_state` entry.
- **Types:** `priority: i32` matches `TaskParams.priority` of Task-Modell. `E` is the worker error type, as in `Task<D,T,E>`.

## 3. Interface

```rust
use std::sync::Arc;

pub type TaskId = u64;

pub struct TaskContext<E> { /* id, priority, AtomicUsize open, AtomicBool cancelled, OnceLock<E> first_error */ }

impl<E> TaskContext<E> {
    /// New context: not cancelled, no error, `open` = `initial_open` open operations.
    pub fn new(id: TaskId, priority: i32, initial_open: usize) -> Arc<Self>;

    pub fn id(&self) -> TaskId;
    pub fn priority(&self) -> i32;                // immutable after creation
    pub fn open_operations(&self) -> usize;       // current counter value
    pub fn is_cancelled(&self) -> bool;
    pub fn first_error(&self) -> Option<&E>;      // None until the first fail()

    /// Increments the open counter by `n`. Call BEFORE enqueueing the follow-ups
    /// and BEFORE complete_operation() of the generating operation. n == 0 is a no-op.
    pub fn register_follow_ups(&self, n: usize);

    /// One operation finished: decrements the counter.
    /// Returns true iff this was the last open operation (counter went 1 -> 0)
    /// AND the task is not cancelled; only then is the task successfully completed.
    pub fn complete_operation(&self) -> bool;

    /// Records an error. If it is the first: store `error`, set cancelled, log, return true.
    /// Otherwise: discard `error`, change nothing, return false.
    pub fn fail(&self, error: E) -> bool;
}
```

`TaskContext<E>` is `Send + Sync` whenever `E: Send + Sync`. Use `Arc<TaskContext<E>>` as the shared handle. `E` needs no `Clone` or `Debug`. The `log` message of `fail` must therefore not require `E: Debug`: log the task ID only.

## 4. Behavior and invariants

**Creation**
- `new` stores the given ID, priority and initial counter. The cancel flag is false and the error is `None`.
- `initial_open == 0` is allowed. A later `complete_operation()` on it is the programming error described below.

**Counter**
- `register_follow_ups(n)` atomically adds `n`.
  - It is also valid on a cancelled task. It still adds, so that counting stays consistent.
  - Overflow is not handled specially.
- `complete_operation()` atomically decrements the counter.
  - The counter never underflows.
  - If the counter is already 0, the call is a programming error: `debug_assert!(false, ..)` panics in debug builds. In release builds it leaves the counter at 0, logs `warn!`, and returns `false`.
  - Otherwise it returns `true` iff the new value is 0 and `is_cancelled()` is false (checked after the decrement).
  - Across all calls the counter reaches 0 from 1 at most once per "registration epoch", so at most one call returns `true` unless new operations are registered after reaching 0. This is not prevented, and the caller must avoid it.
  - A cancelled task never yields `true`, including on its last operation.
- Correct use (register, then enqueue, then complete the generator) keeps the counter above 0 while follow-ups exist.

**First error and cancel**
- `fail(e)` is decided by a single atomic write-once operation (`OnceLock::set`). Exactly one concurrent caller gets `true` and its error is stored.
- The error is stored **before** the cancel flag is set (SeqCst/Release-Acquire). A reader that sees `is_cancelled() == true` after `fail` also sees `first_error() == Some(..)`.
- The first call logs the cancellation (`warn!` or `info!`, with the task ID). Later calls log at most at `debug!`.
- Later errors never overwrite the stored error.
- The cancel flag is never reset. It stays true for the lifetime of the context.
- `fail` does not change the counter. The remaining operations are still counted down by their own `complete_operation()` calls. They return `false` because the task is cancelled.
- `first_error()` returns the same reference for the whole lifetime of the context after it is set.
- Race: if `fail` runs concurrently with the last `complete_operation()`, either outcome is valid.
  - If `complete_operation()` returned `true` before `fail` took effect, `fail` still returns `true` for its own error.
  - A `true` from `complete_operation()` is never revoked.

**Concurrency**
- All reads and writes are safe from any number of threads, with no data races or lost updates. The counter increments and decrements are atomic RMW operations.

## 5. Relied-on elements

None are taken verbatim. The module only aligns with the following, from Task-Modell and Queue-Zustandsmodell:
- `TaskParams.priority: i32`.
- The error type `E` is the same one used in `Task<D,T,E>`.
- Operations that find the task cancelled via `is_cancelled()` must fail the claim. The claim error for that case is `ClaimError::Cancelled` of `queue_state`. Producing it is not the job of this module.