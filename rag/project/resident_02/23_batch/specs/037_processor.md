# Processor: Technical Specification (build order 37)

## 1. Task, restrictions and boundaries

**Task:** Implement in Rust the `Processor`. It owns and wires together:
- one task queue,
- three separate operations queues (CPU, GPU, shared).

It calls the Planner to expand tasks into operations.

**Restrictions**
- The Processor does not execute work, so it contains no worker loop. Workers (other modules) use the Processor's queues.
- It does not interpret task params or operation metadata. Compatibility is decided by the predicate the worker passes to `take_where`.
- The queue is used as-is and is not reimplemented. It is instantiated four times, and each instance has its own channels and shares no state with the others (no globals).
- The queues are unbounded, so `submit`, `expand` and `push` never block or fail for lack of capacity. No capacity limit is added.
- The Planner's internals are out of scope. The Processor knows only the `Planner` trait in §3.
- No `unsafe`. Dependencies: `Queue`, `Task-Modell`, and a `Planner` implementation injected at construction.

## 2. Context

- `Queue<M,P,R,E>` is a lock-free MPMC queue with the channels `Available` and `Completed` and with `take`, `take_where`, `epoch` and `wait_change`.
- `Task<D,T,E>` is a cheap handle with its own state and result. Its params are `TaskParams`.
- Producers call `Processor::submit`. A task is pushed into the task queue as `Queue<TaskParams, Task<D,T,E>, (), ()>`, with the task handle as payload.
- The Planner turns a task into operations. The Processor pushes these into the operations queue of the matching kind: `Queue<O, V, R, OE>`. `O` is the operation metadata, and `V` is the values payload.
- Workers take from the queues, and waking is done by `worker_pool::Notifier`, which the caller of `submit` triggers.
- The Planner module's spec was pending when this spec was written. The `Planner` trait in §3 is the only contract the Processor assumes of it.

## 3. Interface

```rust
use std::sync::Arc;
use queue::{Queue, Ticket, Channel};
use task_model::{Task, TaskParams};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum QueueKind { Cpu, Gpu, Shared }

/// One operation produced by the Planner.
pub struct PlannedOp<O, V> { pub kind: QueueKind, pub meta: O, pub values: V }

/// Contract the Processor expects from the Planner.
pub trait Planner<D, T, E, O, V>: Send + Sync {
    type Error: std::error::Error + Send + Sync + 'static;
    /// Expand a task into operations. Must not block on the Processor's queues.
    fn expand(&self, task: &Task<D, T, E>) -> Result<Vec<PlannedOp<O, V>>, Self::Error>;
}

#[derive(Debug)]
pub enum ExpandError<PE> { Planner(PE) }

pub type TaskQueue<D, T, E> = Queue<TaskParams, Task<D, T, E>, (), ()>;
pub type OpsQueue<O, V, R, OE> = Queue<O, V, R, OE>;

pub struct Processor<D, T, E, O, V, R, OE, PL> { /* private */ }
// Send + Sync when D: Send, T,E: Send+Sync, O: Send+Sync, V,R,OE: Send, PL: Planner (Send+Sync).

impl<D, T, E, O, V, R, OE, PL> Processor<D, T, E, O, V, R, OE, PL>
where PL: Planner<D, T, E, O, V> {
    pub fn new(planner: PL) -> Arc<Self>;

    /// Create a Task, push (params.clone(), task.clone()) into the task queue,
    /// and return the handle. Never blocks, never fails.
    pub fn submit(&self, data: D, params: TaskParams) -> Task<D, T, E>;

    /// Call the Planner for `task` and push each PlannedOp into the operations queue of its kind
    /// (order of the returned Vec preserved). Returns a ticket per op, in order.
    /// On Planner error: nothing is pushed.
    pub fn expand(&self, task: &Task<D, T, E>)
        -> Result<Vec<(QueueKind, Ticket<O, V, R, OE>)>, ExpandError<PL::Error>>;

    pub fn task_queue(&self) -> &TaskQueue<D, T, E>;
    pub fn ops_queue(&self, kind: QueueKind) -> &OpsQueue<O, V, R, OE>;
}
```

## 4. Behavior and invariants

**Ownership and structure**
- The Processor has exactly 4 independent `Queue` instances: task, CPU, GPU, shared. Pushing into one never changes the epochs of another.
- `task_queue()` and `ops_queue(k)` return the same instance on every call. `ops_queue(Cpu)`, `ops_queue(Gpu)` and `ops_queue(Shared)` are three distinct instances.

**submit**
- The new task is `Enqueued` and has the given params. The task queue gets exactly one item whose meta equals `params` and whose payload is a clone of the returned handle (same task).
- `Channel::Available` of the task queue is bumped by the push. Nothing else is notified.

**expand**
- It calls `planner.expand(task)` exactly once per call. The Processor neither retries nor caches.
- Every `PlannedOp` goes to the queue of its `kind` via `push(meta, values)`, in Planner order. Each push bumps `Channel::Available` of that queue. Nothing else is notified.
- An empty Vec is `Ok(vec![])`.
- On `Err(e)`, the result is `Err(ExpandError::Planner(e))`. Nothing is pushed, and the task is not modified (no cancel, no complete).
- The Processor does not claim, complete or cancel the task. Doing so is the caller's job.
- Concurrent `submit` and `expand` calls from any thread are safe. Calls are not serialized against each other.

**Concurrency and errors**
- All methods are callable from any thread concurrently. `submit`, `expand` (outside the Planner) and the accessors are non-blocking.
- The only error is `ExpandError::Planner`. A panic inside the Planner propagates to the caller of `expand` and leaves the queues consistent, because nothing has been pushed yet.
- Dropping the `Processor` drops its queues. Handles, tickets and claims obtained earlier stay valid (Queue guarantee).

## 5. Relied-on elements

From `Queue`:
- `Queue<M,P,R,E>` with `new()`, `push(&self, M, P) -> Ticket`, `take()`, `take_where(FnMut(&M)->bool)`, `len()`, `epoch(Channel)`, `wait_change(Channel, u64, Option<Duration>) -> u64`.
- `Channel::{Available, Completed}`.
- Bounds: `M: Send+Sync, P: Send, R: Send, E: Send`.
- Push is unbounded and never blocks. Items are FIFO. There is no pre-taking.
- `Queue` is not `Clone`, so it is shared via the Processor.

From `Task-Modell`:
- `Task<D,T,E>` is `Clone` (same task) and `Send + Sync` when `D: Send, T: Send+Sync, E: Send+Sync`.
- `Task::new(data, TaskParams) -> Task`, `params() -> &TaskParams`.
- `TaskParams { priority: i32, resource: Option<String>, model: Option<String>, mode: Option<String> }` is `Clone + Default`.
- `claim`, `complete`, `cancel` and `wait` exist. The Processor does not call them.
