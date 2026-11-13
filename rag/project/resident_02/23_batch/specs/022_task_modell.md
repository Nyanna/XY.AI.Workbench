# Spec: Task-Modell (build order 22)

## 1. Task, restrictions and boundaries

Implement in Rust the `Task` type. It is a cheaply clonable handle that encapsulates the following for one task:
- data
- parameters (metadata)
- state
- operations
- a write-once result

**Restrictions**
- A task knows no iteration, cascade, recursion or parent/child relation. It has no intermediate states between tasks.
- Addon tasks and application tasks use the same type. No field, flag or type parameter distinguishes them.
- This module has no queue, no scheduling, no workers, no timeouts and no spawning.
- Priority, resource, model and mode are plain metadata. This module neither interprets nor validates them.
- No `unsafe`.
- Dependencies are `queue_state` (Queue-Zustandsmodell) and `completion_wait` (Abschluss-Warten). The task logic stays thin and delegates all atomic state transitions to `queue_state`.

## 2. Context

The queue creates tasks and hands out clones of the handle. A worker does the following:
1. It claims the task and receives the data by move.
2. It does the work.
3. It completes the task with `Ok(output)` or `Err(error)`.

Callers poll, wait for the result, or cancel while the task is still enqueued.

`Task` wraps an `Arc<EntryState<D, T, E>>` (from `queue_state`) and a `CompletionNotifier` (from `completion_wait`). The state machine, the claim/cancel exclusion and the write-once result all come from `EntryState`. Waiters are woken through the `CompletionNotifier`. `Task` calls `notify_all()` after every successful terminal transition (complete or cancel).

## 3. Interface

```rust
use std::sync::Arc;
use completion_wait::CompletionNotifier;
use queue_state::{ClaimError, CompleteError, EntryState, QueueState};

/// A task has exactly the queue states: Enqueued, Claimed, Completed, Cancelled.
pub type TaskState = QueueState;

/// Ordinary metadata. Immutable after creation. Not interpreted by this module.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TaskParams {
    pub priority: i32,                 // higher = more important (meaning is up to the queue)
    pub resource: Option<String>,      // compatible resource
    pub model: Option<String>,
    pub mode: Option<String>,
}

/// Error side of the result. Cancel yields `Cancelled`; a worker error yields `Failed(E)`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TaskError<E> { Cancelled, Failed(E) }

/// D = data (moved to the claimer once), T = output, E = worker error.
/// Clone = another handle to the SAME task (Arc).
pub struct Task<D, T, E> { /* Arc<inner { params, entry: Arc<EntryState<D,T,E>>, notifier: CompletionNotifier }> */ }
impl<D, T, E> Clone for Task<D, T, E> { /* no bounds on D,T,E */ }

impl<D, T, E> Task<D, T, E> {
    /// New task in state Enqueued holding `data`.
    pub fn new(data: D, params: TaskParams) -> Self;

    pub fn params(&self) -> &TaskParams;
    pub fn state(&self) -> TaskState;

    /// None until the result is set. Cancelled -> Some(Err(TaskError::Cancelled)).
    /// Completed -> Some(Ok(&T)) or Some(Err(TaskError::Failed(&E))).
    pub fn result(&self) -> Option<Result<&T, TaskError<&E>>>;

    /// Enqueued -> Claimed. Moves the data out (exactly once). Never blocks.
    pub fn claim(&self) -> Result<D, ClaimError>;
    /// Claimed -> Completed. Sets the result (once) and wakes waiters.
    pub fn complete(&self, result: Result<T, E>) -> Result<(), CompleteError>;
    /// Enqueued -> Cancelled. true = success (result is now Err(Cancelled), waiters woken); false = no change.
    pub fn cancel(&self) -> bool;

    /// Resolves once the task is Completed or Cancelled; immediately if it already is.
    /// Returns the same value that `result()` returns, never None.
    pub async fn wait(&self) -> Result<&T, TaskError<&E>>;

    /// Clone of the notifier. `subscribe()` lets event/notification consumers
    /// wait for the terminal state, then fetch with `result()`.
    pub fn notifier(&self) -> CompletionNotifier;
}
```

`Task<D,T,E>` is `Send + Sync` whenever `D: Send, T: Send + Sync, E: Send + Sync`. `D` need not be `Sync` or `Clone`.

## 4. Behavior and invariants

**State and result**
- A new task is `Enqueued`, has no result and has the given params.
- The only transitions are Enqueued→Claimed, Claimed→Completed and Enqueued→Cancelled. Completed and Cancelled are final. The transitions are those of `EntryState` and are atomic.
- The result is set exactly once, either as `Ok(output)`, as `Err(Failed(e))`, or as `Err(Cancelled)` on cancel.
- `result()` is `None` while the task is Enqueued or Claimed.
- Once `state()` is `Completed` or `Cancelled`, `result()` is `Some` and never changes.
- Every handle clone sees the same state, params and result.

**Claim**
- `claim()` delegates to `EntryState::claim` and returns its `ClaimError` unchanged. In every failure case the state stays unchanged.
- On success the data is returned by move, exactly once.
- With concurrent claims exactly one wins.
- After a successful claim, `cancel()` returns false.
- After a successful cancel, `claim()` returns `Err(ClaimError::Cancelled)`.

**Complete**
- The task is completed only from `Claimed`, by delegating to `EntryState::complete`.
- After a successful `complete`, `notify_all()` is called on the notifier, so the state is published before the wake-up.
- A second completion returns `Err(CompleteError::NotClaimed(Completed))`, keeps the first result, and sends no notification. The dependency runs `debug_assert!(false)` in this case, so this panics in debug builds and not in release builds.
- Completion on an `Enqueued` or `Cancelled` task returns `Err(NotClaimed(state))`. It sets no result and does not panic.

**Cancel**
- `cancel()` returns `true` only for a successful Enqueued→Cancelled, and only then calls `notify_all()`.
- In every other state it returns false, changes nothing and sends no notification.
- The data stays inside the task and is dropped with the last handle.

**Waiting**
- `wait()` is implemented with `notifier.wait_for_terminal(|| state is Completed or Cancelled)`. Because the subscription is registered before each state check, no wake-up is lost.
- `wait()` does not suspend if the task is already terminal. Dropping the future at any point is safe.
- Any number of waiters on any handle clone are all woken by one terminal transition. Waiters that arrive later return immediately.
- `wait()` and `result()` return identical values for a terminal task.
- Nothing in this module blocks a thread.

**Errors**
- There are no new error types. `ClaimError` and `CompleteError` come from `queue_state` and are passed through unchanged. The invalid-internal-state panic of `EntryState::state()` is passed through as well.

## 5. Relied-on elements

From `queue_state`:
- `enum QueueState { Enqueued, Claimed, Completed, Cancelled }` (`Debug, Clone, Copy, PartialEq, Eq`).
- `enum ClaimError { NotEnqueued(QueueState), Cancelled, NoPayload, Busy }` and `enum CompleteError { NotClaimed(QueueState) }`. Both are `Debug, Clone, Copy, PartialEq, Eq`.
- `EntryState<P,T,E>::new(P) -> Arc<Self>`.
- `state(&self) -> QueueState`.
- `result(&self) -> Option<&Result<T,E>>`. It is `Some` only after a completion and is never set for cancelled entries. It may already be `Some` while the state is still `Claimed`.
- `claim(&self) -> Result<P, ClaimError>`. It never blocks and returns `Busy` if the payload lock is held.
- `complete(&self, Result<T,E>) -> Result<(), CompleteError>`. It stores the result, then publishes `Completed` (Release/Acquire).
- `cancel(&self) -> bool`.
- `EntryState` is `Send + Sync` if `P: Send, T: Send + Sync, E: Send + Sync`.
- Do not use `EntryState::wait_finished`. Waiting goes through `CompletionNotifier`.

From `completion_wait`:
- `CompletionNotifier` is `Clone + Default + Send + Sync`. It has `new()`, `notify_all()` (never blocks, callable from sync code), `subscribe() -> Subscription` and `async fn wait_for_terminal<S, F: FnMut() -> Option<S>>(&self, check: F) -> S`.
- Ordering contract: the terminal state must be visible to `check` before `notify_all()` is called.
- The future returned by `wait_for_terminal` is `Send` if `F` and `S` are `Send`.