# Specification: Operationsknoten (build order 66)

## 1. Task, restrictions and boundaries

Implement the Rust type `OperationNode`. It bundles these parts for one step of a chain:
- an operation,
- an input,
- an output that is initially empty and write-once,
- a materialization flag,
- the associated `Task`,
- an optional continuation handler.

The continuation handler creates follow-up nodes. If it creates none, the output becomes the task result.

Out of scope:
- Executing operations.
- Scheduling and queueing.
- Interpreting the materialization flag.
- Resolving resources.
- Creating tasks.
- Any validation of the parts.

Restrictions:
- No `unsafe`.
- Nothing in this module blocks a thread.
- Use the `log` crate. Log at `debug!` level.

## 2. Context

- The Planner builds operation nodes. A sub-engine executes the operation of a node, sets the output, and then asks the node for its follow-up nodes.
- `Task` (Task-Modell) is a clonable handle. Its result is write-once and is set with `complete`.
- `HasResourceRequirements` (Resource-Requirements) provides the resource kind of the operation.
- `Task` has no ID accessor. The caller therefore passes the task ID to the constructor, and this module only logs it.

## 3. Interface

```rust
use std::sync::{Arc, OnceLock};
use task_model::{Task, CompleteError};
use resource_requirements::{HasResourceRequirements, ResourceKind};

/// Implemented by operations. Provides the operation kind used for logging.
pub trait OperationKind {
    fn operation_kind(&self) -> &'static str;
}

/// Creates zero or more follow-up nodes from the output and the task context.
/// Usable concurrently (Send + Sync, shared via Arc).
pub type ContinuationHandler<O, I, T, D, E> =
    Arc<dyn Fn(&T, &Task<D, T, E>) -> Vec<OperationNode<O, I, T, D, E>> + Send + Sync>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdvanceError { OutputNotSet, Complete(CompleteError) }

/// O = operation, I = input, T = output (= task result type), D = task data, E = task error.
pub struct OperationNode<O, I, T, D, E> { /* operation, input, OnceLock<T>, materialize: bool,
                                             task: Task<D,T,E>, Option<ContinuationHandler> */ }

impl<O, I, T, D, E> OperationNode<O, I, T, D, E>
where O: HasResourceRequirements + OperationKind
{
    /// Output empty, no handler. Logs (debug): task_id, operation_kind(),
    /// resource_requirements().kind() (ResourceKind, Debug), materialize.
    pub fn new(task_id: u64, operation: O, input: I, materialize: bool, task: Task<D, T, E>) -> Self;
}

impl<O, I, T, D, E> OperationNode<O, I, T, D, E> {
    pub fn operation(&self) -> &O;
    pub fn input(&self) -> &I;
    pub fn materialize(&self) -> bool;
    pub fn task(&self) -> &Task<D, T, E>;
    pub fn output(&self) -> Option<&T>;

    /// Attaches a handler, replacing any previous one.
    pub fn set_continuation(&mut self, handler: ContinuationHandler<O, I, T, D, E>);
    /// Builder variant of `set_continuation`.
    pub fn with_continuation(self, handler: ContinuationHandler<O, I, T, D, E>) -> Self;
    pub fn has_continuation(&self) -> bool;

    /// Sets the output once. Ok(()) on the first call.
    /// Any later call returns Err(value) with the rejected value, unchanged. The stored output stays unchanged.
    pub fn set_output(&self, value: T) -> Result<(), T>;
}

impl<O, I, T: Clone, D, E> OperationNode<O, I, T, D, E> {
    /// Requires the output to be set (otherwise Err(OutputNotSet), no side effects).
    /// Calls the handler (if any) with (&output, &task).
    /// - Non-empty result: returns the follow-up nodes. The task is not completed.
    /// - No handler, or an empty result: the chain ends. Calls
    ///   `task.complete(Ok(output.clone()))`. On Err(e) returns Err(Complete(e)).
    ///   On success returns an empty Vec.
    pub fn advance(&self) -> Result<Vec<Self>, AdvanceError>;
}
```

`OperationNode` is `Send + Sync` whenever `O, I: Send + Sync`, `T: Send + Sync`, `D: Send` and `E: Send + Sync`.

## 4. Behavior and invariants

**Construction**
- A new node has an empty output, no handler, and exactly the given parts.
- `new` logs exactly one `debug!` record. It contains the task ID, the operation kind, the resource kind and the materialization flag.
- `new` does not panic and does not touch the task state.

**Output**
- `output()` is `None` until the first successful `set_output`.
- The first `set_output` stores the value and returns `Ok(())`.
- Every further `set_output` returns `Err(value)` with the passed value moved back unchanged. The stored output is never replaced.
- Under concurrent `set_output` calls, exactly one call wins. All others get their own value back.
- `set_output` does not complete the task.

**Continuation handler**
- A handler can be attached after construction, and again later. The latest one wins.
- Handlers are `Arc<dyn Fn + Send + Sync>`. They can be cloned into several nodes and called from several threads at the same time. The module adds no locking around the call.
- The handler receives the output and the task context. It returns zero or more follow-up nodes. The module neither validates nor modifies them.

**Advance**
- `advance()` before `set_output` returns `Err(OutputNotSet)`. The handler is not called and the task is unchanged.
- Chain end (no handler or an empty result): the task is completed with `Ok(clone of the output)`.
  - If `complete` fails, for example because the task is not `Claimed`, the `CompleteError` is returned as `AdvanceError::Complete`. Task-Modell's rules apply: a second completion panics in debug builds.
- With follow-up nodes the task is not completed by this node. The caller owns the returned nodes.
- `advance()` can be called more than once. Each call invokes the handler again. A repeated chain end then fails with `Complete(NotClaimed(Completed))`.

**Errors**
- The only error types are `AdvanceError` (with `CompleteError` passed through unchanged) and the `Err(T)` of `set_output`.

## 5. Relied-on elements

From Task-Modell (`Task<D,T,E>`):
- `Task` is `Clone` (a handle to the same task), `Send + Sync` if `D: Send, T: Send + Sync, E: Send + Sync`.
- `fn complete(&self, result: Result<T, E>) -> Result<(), CompleteError>`
  - It is valid only from `Claimed`. It sets the result once and wakes waiters.
  - A second completion returns `Err(CompleteError::NotClaimed(Completed))`.
  - A completion on `Enqueued` or `Cancelled` returns `Err(NotClaimed(state))`.
- `CompleteError` is `enum { NotClaimed(QueueState) }` and is `Debug, Clone, Copy, PartialEq, Eq`.
- `Task` has no ID.

From Resource-Requirements:
- `trait HasResourceRequirements { fn resource_requirements(&self) -> &ResourceRequirements; }`
- `ResourceRequirements::kind(&self) -> ResourceKind`
- `ResourceKind` is `#[non_exhaustive]`, `Copy, Debug, ...`. Its variants are Tokenizer, Normalizer, Encoder, Scorer, Index and CrossEncoder.