# Specification: Operationsknoten (build order 66)

## 1. Task, restrictions and boundaries

Implement the Rust type `OperationNode`. It bundles these parts for one step of a chain:
- the task ID,
- an operation,
- an input,
- an output that is initially empty and write-once, with a cloneable read-only view (`OutputHandle`),
- a materialization flag,
- the associated `Task`,
- an optional continuation handler,
- a chain-end marker (`ends_chain`, default `true`).

The continuation handler creates follow-up nodes. If it creates none and the node ends the chain, the output becomes the task result.

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

- The `ValueBuilder` of the Planner builds the operation nodes of a planned chain; the stepwise hand-over (build order 78) builds them one by one. A sub-engine executes the operation of a node; the step completion (`step_completion`) sets the output and asks the node for its follow-up nodes.
- `Task` (Task-Modell) is a clonable handle. Its result is write-once and is set with `complete`.
- `HasResourceRequirements` (Resource-Requirements) provides the resource kind of the operation.
- `Task` has no ID accessor. The caller therefore passes the task ID to the constructor, and this module keeps it and logs it.

## 3. Interface

```rust
use std::sync::{Arc, OnceLock};
use task_model::Task;
use queue_state::CompleteError;
use operation::OperationKind;
use resource_requirements::{HasResourceRequirements, ResourceRequirements};
use expander::PlannedOperation;

/// Implemented by operations. Provides the operation kind.
pub trait HasOperationKind {
    fn operation_kind(&self) -> OperationKind;
}

impl HasResourceRequirements for PlannedOperation { /* &self.resource */ }
impl HasOperationKind for PlannedOperation { /* self.kind */ }

/// Creates zero or more follow-up nodes from the output and the task. Send + Sync, shared via Arc.
pub type ContinuationHandler<O, I, T, D, E> =
    Arc<dyn Fn(&T, &Task<D, T, E>) -> Vec<OperationNode<O, I, T, D, E>> + Send + Sync>;

/// Node type of the reference path.
pub type PlannedNode<I, T, D, E> = OperationNode<PlannedOperation, I, T, D, E>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdvanceError { OutputNotSet, Complete(CompleteError) }

/// Cloneable read-only view on the output cell of one node. Clones refer to the same cell.
pub struct OutputHandle<T> { /* Arc<OnceLock<T>> */ }
impl<T> Clone for OutputHandle<T> { /* no bound on T */ }
impl<T> std::fmt::Debug for OutputHandle<T> { /* shows only whether the output is set */ }
impl<T> OutputHandle<T> {
    pub fn get(&self) -> Option<&T>;
    pub fn is_set(&self) -> bool;
}

/// O = operation, I = input, T = output (= task result type), D = task data, E = task error.
pub struct OperationNode<O, I, T, D, E> { /* task_id: u64, operation, input, Arc<OnceLock<T>>,
                                             materialize: bool, task: Task<D,T,E>,
                                             Option<ContinuationHandler>, ends_chain: bool */ }

impl<O, I, T, D, E> OperationNode<O, I, T, D, E>
where O: HasResourceRequirements + HasOperationKind
{
    /// Output empty, no handler, ends_chain == true. Logs (debug) exactly one record with:
    /// task_id, operation_kind() (Debug), resource_requirements().kind() (Debug), materialize.
    pub fn new(task_id: u64, operation: O, input: I, materialize: bool, task: Task<D, T, E>) -> Self;
}

impl<O, I, T, D, E> OperationNode<O, I, T, D, E> {
    pub fn task_id(&self) -> u64;
    pub fn operation(&self) -> &O;
    pub fn input(&self) -> &I;
    pub fn materialize(&self) -> bool;
    pub fn task(&self) -> &Task<D, T, E>;
    pub fn output(&self) -> Option<&T>;
    pub fn output_handle(&self) -> OutputHandle<T>;

    /// Attaches a handler, replacing any previous one.
    pub fn set_continuation(&mut self, handler: ContinuationHandler<O, I, T, D, E>);
    /// Builder variant of `set_continuation`.
    pub fn with_continuation(self, handler: ContinuationHandler<O, I, T, D, E>) -> Self;
    pub fn has_continuation(&self) -> bool;

    /// Whether this node completes the task when it produces no follow-up nodes.
    pub fn set_ends_chain(&mut self, ends_chain: bool);
    pub fn with_ends_chain(self, ends_chain: bool) -> Self;
    pub fn ends_chain(&self) -> bool;

    /// Sets the output once. Ok(()) on the first call; any later call returns Err(value) unchanged.
    pub fn set_output(&self, value: T) -> Result<(), T>;
}

impl<O, I, T: Clone, D, E> OperationNode<O, I, T, D, E> {
    /// Requires the output (otherwise Err(OutputNotSet), no side effects).
    /// Calls the handler (if any) with (&output, &task).
    /// - Non-empty result: returns the follow-up nodes; the task is not completed.
    /// - No handler or empty result, ends_chain() == true: task.complete(Ok(output.clone()));
    ///   Err(e) -> Err(Complete(e)); Ok -> empty Vec.
    /// - No handler or empty result, ends_chain() == false: empty Vec; the task is not touched.
    pub fn advance(&self) -> Result<Vec<Self>, AdvanceError>;
}
```

`OperationNode` is `Send + Sync` whenever `O, I: Send + Sync`, `T: Send + Sync`, `D: Send` and `E: Send + Sync`. `OutputHandle<T>` is `Send + Sync` for `T: Send + Sync`. The module defines no item named `OperationKind`.


## 4. Behavior and invariants

**Construction**
- A new node has an empty output, no handler, `ends_chain() == true` and exactly the given parts. `task_id()` equals the constructor argument.
- `new` logs exactly one `debug!` record with the task ID, the `Debug` text of the `OperationKind` (for example `Embedding`), the `Debug` text of the `ResourceKind` (for example `Scorer`) and the materialization flag.
- `new` does not panic and does not touch the task state.
- The module never matches on `OperationKind` or `ResourceKind`. Only their `Debug` text is used.

**Output**
- `output()` is `None` until the first successful `set_output`.
- The first `set_output` stores the value and returns `Ok(())`.
- Every further `set_output` returns `Err(value)` with the passed value moved back unchanged. The stored output is never replaced.
- Under concurrent `set_output` calls, exactly one call wins. All others get their own value back.
- `set_output` does not complete the task.

**Output sharing**
- `output_handle()` can be called any number of times. All handles and the node see the same cell. `OutputHandle::get()` returns `None` until `set_output` succeeded, and then the same value as `output()`.
- Handles keep the output cell alive after the node is dropped.

**Chain-end marker**
- `set_ends_chain` and `with_ends_chain` change only the flag. The latest value wins.
- The module does not know the chain and does not check how many nodes end it.

**Continuation handler**
- A handler can be attached after construction, and again later. The latest one wins.
- Handlers are `Arc<dyn Fn + Send + Sync>`. They can be cloned into several nodes and called from several threads at the same time. The module adds no locking around the call.
- The handler receives the output and the task context. It returns zero or more follow-up nodes. The module neither validates nor modifies them.

**Advance**
- `advance()` before `set_output` returns `Err(OutputNotSet)`. The handler is not called and the task is unchanged.
- Follow-up nodes from the handler are returned; the task is not completed by this node. The caller owns the returned nodes.
- With `ends_chain == true` and no follow-up nodes the task is completed with `Ok(clone of the output)`.
  - If `complete` fails, for example because the task is not `Claimed`, the `CompleteError` is returned as `AdvanceError::Complete`. Task-Modell's rules apply: a second completion panics in debug builds.
- With `ends_chain == false` and no follow-up nodes `advance` returns an empty `Vec` and does not call `complete`. A repeated call does the same and never panics.
- `advance()` can be called more than once. Each call invokes the handler again. A repeated chain end (`ends_chain == true`) is a second completion: it panics in debug builds and returns `Complete(NotClaimed(Completed))` in release builds.

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

From Operation:
- `OperationKind` is `#[non_exhaustive]`, `Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord`. Only its `Debug` text is used.

From Expander:
- `PlannedOperation { kind, descriptor, resource, config }` is `Clone, Debug, PartialEq`.