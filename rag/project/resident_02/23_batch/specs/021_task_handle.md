# Spec: Task-Handle (module `task_handle`, build order 21)

## 1. Task, restrictions and boundaries
Implement in Rust the caller-side handle of one queued task. The handle has three jobs:
- Query the state without waiting (polling).
- Wait for completion, either unbounded or with a caller-owned timeout.
- Cancel the task on drop if no worker has claimed it yet.

Restrictions:
- Do not implement the queue, the workers, the state machine or the notification primitive. Use the existing `EntryState` as is. Do not redefine its types.
- Do not add a result store or any new state. "Cancelled" is derived locally from `QueueState::Cancelled`.
- No `unsafe`.
- No blocking calls. The waits are `async`.
- The only external crates are `tokio` (features `sync`, `time`) and `log`.

## 2. Context
- The queue creates `Arc<EntryState<P,T,E>>` per entry. It keeps one clone for the workers and gives the other to the caller. This module wraps the caller's clone.
- A worker calls `claim()`, runs the work, then calls `complete(Result<T,E>)`. `complete` writes the result first, then publishes `Completed`, then calls `notify_waiters()`.
- A caller-side `cancel()` succeeds only from `Enqueued`. It also calls `notify_waiters()`.
- `EntryState::wait_finished()` is the actual signal source. It registers its `Notified` future before it checks the state.
- Abschluss-Warten (`CompletionNotifier`) is not wired into `EntryState`. Nothing calls its `notify_all()` for this entry. Therefore this module does **not** use it as its signal. It waits via `EntryState::wait_finished`.

## 3. Interface
```rust
use std::sync::Arc;
use std::time::Duration;
use queue_state::{EntryState, QueueState};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HandleError {
    /// Task was cancelled (display text: "cancelled").
    Cancelled,
    /// No result yet; only from try_result (display: "noch kein Ergebnis").
    NotReady,
    /// Caller's deadline exceeded (display: "timeout").
    Timeout,
}
// impl std::fmt::Display + std::error::Error for HandleError

/// Interval of the periodic state re-check while waiting.
pub const RECHECK_INTERVAL: Duration = Duration::from_millis(100);

/// Owned access to a finished task's result.
pub struct TaskOutcome<P, T, E> { /* Arc<EntryState<P,T,E>> */ }
impl<P, T, E> TaskOutcome<P, T, E> {
    /// The task's Output (Ok) or its error (Err).
    pub fn result(&self) -> &Result<T, E>;
}

pub struct TaskHandle<P, T, E> { /* Arc<EntryState<P,T,E>> */ }

impl<P, T, E> TaskHandle<P, T, E> {
    pub fn new(entry: Arc<EntryState<P, T, E>>) -> Self;

    /// Non-blocking state query.
    pub fn state(&self) -> QueueState;

    /// Polling without waiting.
    pub fn try_result(&self) -> Result<&Result<T, E>, HandleError>;

    /// Wait without a limit for completion/cancellation.
    pub async fn wait(self) -> Result<TaskOutcome<P, T, E>, HandleError>;

    /// Wait at most `timeout` (the caller's own deadline).
    pub async fn wait_timeout(self, timeout: Duration)
        -> Result<TaskOutcome<P, T, E>, HandleError>;
}

impl<P, T, E> Drop for TaskHandle<P, T, E> { /* cancel if still Enqueued */ }
```
- `TaskHandle` and `TaskOutcome` are `Send + Sync` when `EntryState<P,T,E>` is.
- The `wait` and `wait_timeout` futures are `Send` under the same condition.
- `wait` and `wait_timeout` need a Tokio runtime with the time driver.

## 4. Behavior and invariants

**try_result (no waiting)**
- `Completed` and `result()` is `Some(r)`: returns `Ok(r)`. `r` is the worker's `Ok(output)` or `Err(error)`.
- `Cancelled`: returns `Err(Cancelled)`.
- `Enqueued` or `Claimed`: returns `Err(NotReady)`.
- `Completed` but `result()` is `None`: treated as cancelled, returns `Err(Cancelled)`. Never panic.
- It never blocks and has no side effects. Repeated calls return the same result.
- `Enqueued`/`Claimed` is decided from `state()` alone. A `Some` result seen while the state is still `Claimed` is **not** delivered (`NotReady`).

**wait / wait_timeout**
- Algorithm per iteration:
  1. Create the `wait_finished()` future and poll it once. This registers the notification first.
  2. Only after that, read the state.
  3. If the state is final, evaluate it as `try_result` does and return.
  4. Otherwise wait for the signal or for `RECHECK_INTERVAL`, whichever comes first, then repeat.
  - Result: no notification is lost, and a missed signal is caught by the periodic re-check.
- Already final at call time: returns at once without suspending for a timer.
- Success returns `Ok(TaskOutcome)`. The outcome wraps the result, which may itself be `Err(E)` from the worker.
- A cancelled task returns `Err(Cancelled)`. `Completed` without a result also returns `Err(Cancelled)`.
- `wait` never returns `Timeout` or `NotReady`.
- `wait_timeout`:
  - The timeout is the caller's own deadline. It is measured from the call and is independent of the queue and the worker.
  - Final state reached before the deadline: normal result.
  - Deadline exceeded: returns `Err(Timeout)`. The handle was consumed and is dropped, so the drop rule below applies. An unclaimed task is cancelled and a claimed task keeps running.
  - `Duration::ZERO`: checks the state once. If it is not final, returns `Err(Timeout)`.
  - If the state becomes final at the same moment as the deadline, the final state wins. The state is checked once more after the deadline fires.
- Dropping the wait future mid-wait drops the handle, so the drop rule applies. Nothing else leaks.
- Several handles or waiters on the same entry are independent.

**Drop**
- `drop` calls `entry.cancel()` and ignores the `bool`.
  - `Enqueued` becomes `Cancelled`.
  - In every other state `cancel()` returns `false` and changes nothing. A task a worker has claimed keeps running, and its result is discarded with the entry.
  - Finished (Completed/Cancelled) handles are unaffected.
- `drop` is non-blocking, never panics and may be called from any thread.
- It is race-free because `cancel()` is atomic against `claim()`. Exactly one side wins.
- `wait` and `wait_timeout` consume the handle. After a successful `Ok(TaskOutcome)` the entry is `Completed`, so the drop's `cancel()` is a no-op.

**Errors**
- No panics occur in this module, except the one `EntryState::state()` documents for an invalid internal value.
- Do not log at `warn` or above for the normal paths.

## 5. Relied-on elements
From `queue_state` (use as is):
- `EntryState<P,T,E>` and `Arc<EntryState<..>>` as the shared entry.
- `enum QueueState { Enqueued, Claimed, Completed, Cancelled }`.
- `fn state(&self) -> QueueState`.
- `fn result(&self) -> Option<&Result<T,E>>`.
  - `Some` only once the result is set.
  - The result may already be `Some` while the state is still `Claimed`.
  - Never `Some` for a `Cancelled` entry.
  - Visible whenever `Completed` is observed.
- `fn cancel(&self) -> bool`.
  - `true` only for `Enqueued` → `Cancelled`.
  - Atomic against `claim`.
  - After a worker's claim it returns `false`.
- `async fn wait_finished(&self) -> QueueState`.
  - Returns `Completed` or `Cancelled`.
  - Does not miss a wake-up.
  - Returns at once if the state is already final.
- `Completed` and `Cancelled` are final states.
- `EntryState` is `Send + Sync` if `P: Send, T: Send + Sync, E: Send + Sync`.

From Abschluss-Warten: nothing is used. `CompletionNotifier` is not triggered by `EntryState`.