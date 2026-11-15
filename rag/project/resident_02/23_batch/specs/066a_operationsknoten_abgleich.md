# Specification: Operationsknoten, reconciliation (build order 66a)

This spec amends build order 66 (`operation_node`). Spec 66 stays valid except where this spec replaces it. It also adds one new module, `step_completion`. §3 to §5 are the implementation. §6 is the register of findings with their resolutions. §7 lists binding contracts and amendments for other specs. They are not part of the implementation.

## 1. Task, restrictions and boundaries

Rework the Rust module `operation_node` and add the module `step_completion`. Both are in `src/resident/l130/l120/l110/l100/l090/l080/l070/l060/l050/a/`.

Changes to `operation_node`:
- **E0. Operation kind.** Replace spec 66's trait `OperationKind` (`&'static str`) with `HasOperationKind { fn operation_kind(&self) -> OperationKind }`. `OperationKind` is the existing enum `operation::OperationKind`. `new` requires `O: HasResourceRequirements + HasOperationKind` and logs the enum with `{:?}`.
- **E1. Chain-end marker.** Add the flag `ends_chain` (default `true`). A node without follow-up nodes completes the task only if the flag is `true`.
- **E2. Task ID.** The node keeps its task ID and exposes `task_id()`.
- **E3. Output sharing and readiness.** The node exposes a cloneable read-only `OutputHandle<T>` on its output and a type-erased `Gate` derived from it.
- **E4. Reference operation type.** `PlannedOperation` (from `expander`) implements `HasResourceRequirements` and `HasOperationKind`. The alias `PlannedNode` names the node type of the reference path.

New module `step_completion`:
- **E5. Step completion.** `StepPayload` bundles a node with the `TaskContext` of its task. Its methods `finish` and `fail` run the sequence that follows the execution of a step (`set_output`, `advance`, registering and submitting follow-up nodes, counter handling, error handling).
- **E6. Chain modification.** `withdraw_pending` and `withdraw_planned` remove not yet started entries of a task from a `queue::Queue`. Follow-up nodes go through the `FollowUpSink` trait.

Everything else of spec 66 is unchanged: parts, write-once output, continuation handler, errors.

Out of scope:
- All exclusions of spec 66 (executing operations, scheduling, queueing, interpreting the materialization flag, resolving resources, creating tasks, validating parts).
- Planner, `ValueBuilder`, workers, sub-engines, batch layer. Their binding rules are in §7.
- Any change to a module other than the two named above and `mod.rs` of the same directory. No spec file is changed.

Restrictions:
- No `unsafe`. Nothing in these modules blocks a thread.
- Use the `log` crate, `debug!` level.
- Comments and documents in English, minimal, no references to plan elements or plan numbers.
- Tests live in `test/<module>.rs`. Each source file ends with:
  ```rust
  #[cfg(test)]
  #[path = "test/<module>.rs"]
  mod tests;
  ```

## 2. Context

**Decisions of the spec owner.**
- Both chain-creation modes must be possible: the full chain created in advance, and the chain created stepwise by continuation handlers. A chain may also be modified during a run.
- The reference implementation uses the **full-chain model**: the Planner/Expander creates the complete chain in one `expand` call, and the Processor pushes all steps (specs 37, 126).

**Decisions derived from it** (the owner may override them; see §6):
- In the reference path the executable operation `O` is `PlannedOperation`. `Operation<R>` (spec 36) is not used there, because the node already carries input, output and task.
- The ops queues are the take-based `queue::Queue<M,P,R,E>` that the Processor owns (`push`, `take_where`, `Claim`). Step `k+1` is pushed before step `k` has run. Its readiness is expressed by a `Gate` in the queue metadata, which workers must test in their `take_where` predicate.
- A planned step is pushed with `V = StepPayload`. Its input refers to the predecessor's `OutputHandle`.
- Removing planned entries uses only existing queue operations: `take_where` with a predicate, then dropping the `Claim` (the item becomes `Abandoned`).

Current state (verified in the code base):
- `operation_node.rs` exists with the spec 66 behavior and the string trait `OperationKind`. `advance()` completes the task whenever there is no handler or the handler returns an empty list. All 813 tests of the crate pass.
- `operation::OperationKind` (`.../l050/l040/l030/b/operation.rs`) is `#[non_exhaustive]`, `Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord`, with the variants Tokenization, Embedding, Pooling, L2Normalization, Similarity, TopK, IndexWrite, Inference, DeviceToRam.
- `expander::PlannedOperation { pub kind: OperationKind, pub descriptor, pub resource: resource_requirements::ResourceRequirements, pub config: OperationConfig }` is `Clone, Debug, PartialEq`, `Send + Sync + 'static`. `OperationConfig` implements `Default`.
- No production type implements `HasResourceRequirements` yet.
- `TaskContext<E>` (`.../l050/l040/l030/b/task_context.rs`): `new(id: u64, priority, initial_open) -> Arc<Self>`, `id()`, `open_operations()`, `is_cancelled()`, `first_error()`, `register_follow_ups(n)`, `complete_operation() -> bool` (true only if this call took the count from 1 to 0 and the task is not cancelled; calling it at 0 open operations is a `debug_assert` failure), `fail(error) -> bool` (true for the first error; sets cancelled).
- `queue::Queue<M,P,R,E>` (`.../l050/l040/l030/l020/l010/queue.rs`): `push(meta, payload) -> Ticket`, `take()`, `take_where(FnMut(&M)->bool) -> Option<Claim>`, `len()` (Pending only). Dropping a `Claim` without `complete` sets the item to `Abandoned`. `push` bumps `Channel::Available`. The `Processor` owns three such queues and pushes every `PlannedOp` with `push(meta, values)`.
- `Task::complete` called a second time panics in debug builds with "entry completed more than once". It returns `Err(NotClaimed(state))` for `Enqueued` and `Cancelled`.
- `CompleteError` and `QueueState` live in `.../l050/l040/l030/l020/l010/queue_state.rs`. `task_model` does not re-export them.

## 3. Interface

### 3.1 `operation_node`

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

/// The operation of the reference path.
impl HasResourceRequirements for PlannedOperation { /* &self.resource */ }
impl HasOperationKind for PlannedOperation { /* self.kind */ }

/// Creates zero or more follow-up nodes from the output and the task context.
/// Usable concurrently (Send + Sync, shared via Arc).
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
impl<T: Send + Sync + 'static> OutputHandle<T> {
    /// Type-erased readiness view: open once the output is set.
    pub fn gate(&self) -> Gate;
}

/// Type-erased readiness signal. Cloneable and cheap. Default = always open.
#[derive(Clone, Default)]
pub struct Gate { /* Option<Arc<dyn Fn() -> bool + Send + Sync>> */ }
impl Gate {
    /// A gate that is always open.
    pub fn open() -> Self;
    pub fn is_open(&self) -> bool;
}
impl std::fmt::Debug for Gate { /* shows only is_open() */ }

/// O = operation, I = input, T = output (= task result type), D = task data, E = task error.
pub struct OperationNode<O, I, T, D, E> { /* task_id: u64, operation, input, Arc<OnceLock<T>>,
                                             materialize: bool, task: Task<D,T,E>,
                                             Option<ContinuationHandler>, ends_chain: bool */ }

impl<O, I, T, D, E> OperationNode<O, I, T, D, E>
where O: HasResourceRequirements + HasOperationKind
{
    /// Output empty, no handler, ends_chain == true. Logs (debug) exactly one record with:
    /// task_id, operation_kind() (OperationKind, Debug), resource_requirements().kind()
    /// (ResourceKind, Debug), materialize.
    pub fn new(task_id: u64, operation: O, input: I, materialize: bool, task: Task<D, T, E>) -> Self;
}

impl<O, I, T, D, E> OperationNode<O, I, T, D, E> {
    pub fn task_id(&self) -> u64;
    pub fn operation(&self) -> &O;
    pub fn input(&self) -> &I;
    pub fn materialize(&self) -> bool;
    pub fn task(&self) -> &Task<D, T, E>;
    pub fn output(&self) -> Option<&T>;
    /// Read-only handle on this node's output cell.
    pub fn output_handle(&self) -> OutputHandle<T>;

    /// Attaches a handler, replacing any previous one.
    pub fn set_continuation(&mut self, handler: ContinuationHandler<O, I, T, D, E>);
    pub fn with_continuation(self, handler: ContinuationHandler<O, I, T, D, E>) -> Self;
    pub fn has_continuation(&self) -> bool;

    /// Whether this node completes the task when it produces no follow-up nodes.
    /// `false` marks a node that has planned successors outside of it.
    pub fn set_ends_chain(&mut self, ends_chain: bool);
    pub fn with_ends_chain(self, ends_chain: bool) -> Self;
    pub fn ends_chain(&self) -> bool;

    /// Sets the output once. Ok(()) on the first call.
    /// Any later call returns Err(value) with the rejected value, unchanged. The stored output stays unchanged.
    pub fn set_output(&self, value: T) -> Result<(), T>;
}

impl<O, I, T: Clone, D, E> OperationNode<O, I, T, D, E> {
    /// Requires the output to be set (otherwise Err(OutputNotSet), no side effects).
    /// Calls the handler (if any) with (&output, &task).
    /// - Non-empty result: returns the follow-up nodes. The task is not completed,
    ///   whatever `ends_chain` is.
    /// - No handler, or an empty result, and `ends_chain() == true`: calls
    ///   `task.complete(Ok(output.clone()))`. On Err(e) returns Err(Complete(e)).
    ///   On success returns an empty Vec.
    /// - No handler, or an empty result, and `ends_chain() == false`: returns an empty Vec.
    ///   The task is not touched.
    pub fn advance(&self) -> Result<Vec<Self>, AdvanceError>;
}
```

`OperationNode` is `Send + Sync` whenever `O, I: Send + Sync`, `T: Send + Sync`, `D: Send` and `E: Send + Sync`. `OutputHandle<T>` and `Gate` are `Send + Sync` for `T: Send + Sync`. The module defines no item named `OperationKind`.

### 3.2 `step_completion`

```rust
use std::sync::Arc;
use operation_node::{AdvanceError, OperationNode};
use task_context::TaskContext;
use queue::Queue;

/// Receives the follow-up nodes of a finished step. Implementations enqueue them.
pub trait FollowUpSink<N>: Send + Sync {
    fn submit(&self, nodes: Vec<N>);
}

/// The payload of one queue entry: a node plus the context of its task.
pub struct StepPayload<O, I, T, D, E> {
    pub node: OperationNode<O, I, T, D, E>,
    pub context: Arc<TaskContext<E>>,
}
impl<O, I, T, D, E> StepPayload<O, I, T, D, E> {
    pub fn new(node: OperationNode<O, I, T, D, E>, context: Arc<TaskContext<E>>) -> Self;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StepError { OutputAlreadySet, Advance(AdvanceError) }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StepReport {
    /// Number of follow-up nodes submitted.
    pub follow_ups: usize,
    /// Result of the last `complete_operation()` of this call.
    pub task_finished: bool,
    /// The task was already cancelled: the output was not stored and `advance` was not called.
    pub discarded: bool,
}

impl<O, I, T: Clone, D, E> StepPayload<O, I, T, D, E> {
    /// Stores the result of a successfully executed step and continues the chain.
    pub fn finish<S>(&self, output: T, sink: &S) -> Result<StepReport, StepError>
    where S: FollowUpSink<OperationNode<O, I, T, D, E>>;
}

impl<O, I, T, D, E: Clone> StepPayload<O, I, T, D, E> {
    /// Handles a failed step. `withdraw` removes the not yet started entries of the task
    /// and returns how many it removed (see `withdraw_pending`).
    pub fn fail<W: FnOnce() -> usize>(&self, error: E, withdraw: W) -> StepReport;
}

/// Claims and drops (-> Abandoned) every Pending item accepted by `matches`.
/// Returns the number removed. Items already claimed by a worker are not touched.
pub fn withdraw_pending<M, P, R, E>(queue: &Queue<M, P, R, E>, matches: impl FnMut(&M) -> bool) -> usize;

/// `withdraw_pending`, plus one `complete_operation()` on `context` per removed item.
pub fn withdraw_planned<M, P, R, E, X>(
    queue: &Queue<M, P, R, E>,
    matches: impl FnMut(&M) -> bool,
    context: &TaskContext<X>,
) -> usize;
```
(The bounds on `M, P, R, E` are those of `Queue`; take them from `queue.rs`.)

## 4. Behavior and invariants

### 4.1 `operation_node`

Identical to spec 66 §4, with these changes.

**Construction**
- `new` logs exactly one `debug!` record with the task ID, the `Debug` text of the `OperationKind` (for example `Embedding`), the `Debug` text of the `ResourceKind` (for example `Scorer`) and the materialization flag. It does not panic and does not touch the task state.
- A new node has `ends_chain() == true` and `task_id()` equal to the argument.
- The module never matches on `OperationKind` or `ResourceKind`. Both are `#[non_exhaustive]`; only their `Debug` text is used.

**Output sharing and readiness**
- `output_handle()` can be called any number of times. All handles and the node see the same cell. `OutputHandle::get()` returns `None` until `set_output` succeeded, and then the same value as `output()`.
- `gate()` is closed until the output is set, and open afterwards. It never closes again. `Gate::open()` and `Gate::default()` are always open.
- Handles and gates keep the output cell alive after the node is dropped.

**Chain-end marker**
- `set_ends_chain` and `with_ends_chain` change only the flag. The latest value wins.
- The module does not know the chain. It does not check that exactly one node has `ends_chain == true`. Setting the flag is the job of whoever builds the chain.

**Advance**
- `advance()` before `set_output` returns `Err(OutputNotSet)`. It does not call the handler and leaves the task unchanged.
- If the handler returns follow-up nodes, they are returned and the task is not completed. This is the only way a node adds nodes at run time, in both modes. The module neither validates, orders nor places them.
- With `ends_chain == true` and no follow-up nodes the task is completed with `Ok(clone of the output)`. A `CompleteError` is returned as `AdvanceError::Complete`.
- With `ends_chain == false` and no follow-up nodes `advance` returns an empty `Vec` and does not call `complete`. A repeated call does the same and never panics.
- `advance()` can be called repeatedly and invokes the handler each time. A repeated chain end (`ends_chain == true`) is a second completion: it panics in debug builds and returns `Complete(NotClaimed(Completed))` in release builds.

Unchanged: a new node has an empty output, no handler and exactly the given parts. `output()` is `None` until the first successful `set_output`. Further calls return `Err(value)` with the value moved back unchanged. Under concurrent calls exactly one wins. `set_output` does not complete the task. A handler can be attached again; the latest one wins. Handlers are shared via `Arc` and called without added locking.

### 4.2 `step_completion`

**`finish(output, sink)`**, in this order:
1. If `context.is_cancelled()`: call `context.complete_operation()` once, store nothing, call neither `set_output` nor `advance`, and return `Ok(StepReport { follow_ups: 0, task_finished: <that result>, discarded: true })`.
2. `node.set_output(output)`. If it returns `Err`, return `Err(OutputAlreadySet)`. Counters are untouched.
3. `node.advance()`. On `Err(e)` return `Err(Advance(e))`. Counters are untouched. The caller decides what to do (normally `fail`).
4. Let `n` be the number of returned nodes. If `n > 0`: first `context.register_follow_ups(n)`, then `sink.submit(nodes)`. The registration must come before the submission.
5. `context.complete_operation()` for this step, always after step 4.
6. Return `Ok(StepReport { follow_ups: n, task_finished: <result of step 5>, discarded: false })`.

**`fail(error, withdraw)`**, in this order:
1. `context.fail(error.clone())`. If it returned `true` (first error) and `node.task().state() == Claimed`, call `node.task().complete(Err(error))`. A `CompleteError` is only logged at `debug!`. The state check avoids the debug-build panic for a completed task.
2. `let removed = withdraw();` then call `context.complete_operation()` `removed` times.
3. `context.complete_operation()` once for this step.
4. Return `StepReport { follow_ups: 0, task_finished: <result of step 3>, discarded: false }`. Because the context is cancelled, `task_finished` is `false`.

**Counting invariant.** `TaskContext::new(id, priority, initial_open)` must be created with `initial_open` equal to the number of pushed steps. Every pushed step is accounted for exactly once: by `finish`, by `fail` or by withdrawal. Each follow-up node is registered before it is submitted.

**`withdraw_pending`** loops `queue.take_where(&mut matches)` until it returns `None`, drops each claim and counts them. Dropped claims become `Abandoned`. It never blocks. It does not wait for running items. `withdraw_planned` additionally calls `context.complete_operation()` once per removed item, after the removal.

**Modification of a chain at run time** is composed from the primitives. Replace planned entries: `withdraw_planned(...)`, then `context.register_follow_ups(new_count)`, then `sink.submit(new_nodes)`. Insert additional entries: the handler of a node returns them, `finish` registers and submits them. Entries that a worker has already claimed cannot be withdrawn. The result is best effort and the count says how many were removed.

**Concurrency.** All functions can be called from any thread. No lock is held. `FollowUpSink::submit` may be called from any worker thread.

## 5. Relied-on elements, implementation steps, acceptance

Paths are relative to `crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050`.
- `l040::l030::b::operation::OperationKind` (only `Debug` is used).
- `l040::a::resource_requirements::{HasResourceRequirements, ResourceKind, ResourceRequirements, ExecutionEnvironment}`. `ResourceKind` is `#[non_exhaustive]`, `Copy + Debug`.
- `a::expander::{PlannedOperation, OperationConfig}` and what its tests use to build values.
- `l040::l030::l020::b::task_model::{Task, TaskParams}`. `Task::new(data, TaskParams)`, `claim()`, `complete()`, `state()`, `result()`. `TaskParams` implements `Default`.
- `l040::l030::l020::l010::queue_state::{CompleteError, QueueState}`.
- `l040::l030::b::task_context::TaskContext` (§2).
- `l040::l030::l020::l010::queue::{Queue, State}`. `Ticket::state()` is used by tests to see `Abandoned`.
- `log` crate.

Implementation steps:
1. `operation_node.rs`: implement E0 to E4 as in §3.1. Store the output in an `Arc<OnceLock<T>>`. Add `ends_chain` and `task_id`. Add the two impls for `PlannedOperation` and the aliases.
2. `test/operation_node.rs`: change `TestOp` to hold an `OperationKind` and a `ResourceRequirements` and to implement `HasOperationKind`. Keep all existing cases. Replace the log assertion: the record must contain the task ID, the `Debug` text of the chosen `OperationKind`, the `Debug` text of the chosen `ResourceKind` and the flag. Choose a pair that cannot match by accident (for example `Pooling` and `Scorer`). Keep the log test tolerant when a global logger is already installed. Keep the repeated-completion test split by `cfg(debug_assertions)` (`should_panic` in debug, `NotClaimed(Completed)` in release).
3. Add node tests:
   - `operation().operation_kind()` and `task_id()` return the values the node was built with. `ends_chain()` is `true` for a new node, and the setter and the builder change it.
   - `ends_chain == false`, no handler or handler returning an empty list: `advance` returns an empty `Vec`, the task stays `Claimed` without result. Repeated calls give the same result and do not panic. The handler is called once per `advance`.
   - `ends_chain == false`, handler returning nodes: the nodes are returned, the task is not completed. `advance` before `set_output` returns `Err(OutputNotSet)`.
   - Full-chain scenario: three nodes on one claimed task with `ends_chain` `false, false, true`. Only the third `advance` completes the task, with the output of the third node.
   - `output_handle()`: `get()` is `None` before and equals `output()` after `set_output`. Clones see the same value. The handle still reads the value after the node is dropped. A second `set_output` does not change it.
   - `gate()`: closed before `set_output`, open after, stays open. `Gate::open()` and `Gate::default()` are open. `Debug` of both types does not require `T: Debug`.
   - A successor node whose `I` holds the predecessor's `OutputHandle` reads the predecessor's output after the predecessor's `set_output`.
   - `PlannedOperation` implements both traits with `resource.kind()` and `kind`. A `PlannedNode` can be built from a real `PlannedOperation` (`OperationConfig::default()` and the `plan`-like construction used in the expander tests).
   - Compile-time check: `OperationNode<TestOp, u32, String, String, String>`, `OutputHandle<String>` and `Gate` are `Send + Sync`.
4. `step_completion.rs` and `test/step_completion.rs`: implement §3.2 and §4.2. Register `pub mod step_completion;` in `.../l050/a/mod.rs`. Tests (use a recording `FollowUpSink`, a `TaskContext<String>` and a real `queue::Queue`):
   - `finish` on a node with a handler returning two nodes: the sink receives both, and `open_operations()` at the moment of `submit` already includes the two registered follow-ups. The final count is `initial + 2 - 1`.
   - `finish` with no follow-ups on a node with `ends_chain == false`: nothing submitted, count decreases by one, the task is not completed.
   - Full-chain scenario: three steps pushed into a real `Queue` with `meta` containing a `Gate`, `TaskContext` with `initial_open == 3`, and a worker loop in the test that uses `take_where(|m| m.gate.is_open())`. The third step is not taken before the second has finished. Only the third `finish` returns `task_finished == true`, and the task result is the third output.
   - `finish` on a cancelled context: `discarded == true`, no output stored, the counter decreases by one.
   - `finish` when the output is already set: `Err(OutputAlreadySet)`, counters untouched. `finish` when `advance` fails (task not `Claimed`): `Err(Advance(Complete(..)))`, counters untouched.
   - `fail`: the first error completes the task with `Err`, the second `fail` for the same task does not call `complete` and does not panic. `withdraw` is called once. `open_operations()` ends at 0 after all steps are accounted for. `task_finished` is `false`.
   - `fail` on a task that is already `Completed` does not panic in debug builds.
   - `withdraw_pending`: removes exactly the matching Pending items, leaves the others Pending in order, leaves a claimed (Running) item untouched, returns the count, and the removed tickets report `Abandoned`. `withdraw_planned` also reduces `open_operations()` by the count.
   - Replace scenario: `withdraw_planned`, `register_follow_ups`, `submit`. The counter equals the number of live entries.
5. Run `runTests`. All tests must pass. Only these files may change: `operation_node.rs`, `step_completion.rs`, their two test files and `.../l050/a/mod.rs`.

Acceptance:
- `grep` finds no `trait OperationKind` in the crate.
- A node that never calls `set_ends_chain` behaves like spec 66.
- `operation_node` exports `HasOperationKind`, `ContinuationHandler`, `PlannedNode`, `AdvanceError`, `OperationNode`, `OutputHandle`, `Gate`. `step_completion` exports `FollowUpSink`, `StepPayload`, `StepError`, `StepReport`, `withdraw_pending`, `withdraw_planned`.

## 6. Register of findings and resolutions

| ID | Finding | Resolution |
|---|---|---|
| C1 | Spec 66 defined a trait `OperationKind` that collides with the enum used by specs 36, 65, 126, 132, 134 and the code. | E0. |
| C2 | Spec 78 imports and bounds on the old trait. | A2. |
| C3 | No type satisfied the node's bounds. `Operation<R>` has no `HasResourceRequirements`, and its `requirements()` is the string-only descriptor type. `OpMeta` (spec 126) lacks the impl that the workers require. | E4: `O = PlannedOperation` in the reference path. `Operation<R>` is not used there. `OpMeta` gets both impls (A3). |
| C4 | Two chain-creation modes. | Decided by the owner: full chain is the reference. Stepwise creation and run-time modification stay possible (§2). |
| C5 | Materialization: spec 78 says `true` only for the last step, spec 126 has a rule per operation. | For planned chains the Planner's rule is the only source. The node's flag equals `meta.materialize` (A3). Spec 78's rule applies only to chains it builds itself (A5). |
| C6 | The node did not keep its task ID. | E2. |
| C7 | The payload `V`, the runner's call sequence and the relation between `Claim::complete` and `Task::complete` were undefined. | E5, and the rules in §7 (A6, A7). `Claim::complete(Result<R,E>)` completes the queue entry. It is independent of `Task::complete`. Only the node with `ends_chain == true` completes the task, through `advance`. |
| C8 | Spec 65 lists `OperationKind` without `DeviceToRam`. | A4. |
| C9 | Two `ResourceRequirements` types and two `TaskId` types exist. | No action. The node uses only `resource_requirements::ResourceRequirements` and the raw `u64`. |
| C10 | Spec 66 completes the task for every node without follow-ups, so a pre-built chain would complete at the first step. | E1. |
| C11 | A pre-built successor cannot read the output of its predecessor. | E3: the successor's input holds the predecessor's `OutputHandle`. The `ValueBuilder` creates the nodes in chain order and takes the handle before it moves the predecessor into the result (A3). |
| C12 | Run-time modification of planned chains had no support below the node. | E6, and the composition rules in §4.2. |
| C13 | **New.** In the full-chain model all steps are pushed at once, and no spec stops a worker from claiming step `k+1` while step `k` is still running or not yet started. | E3: `Gate`. `OpMeta` carries `ready: Gate` (the predecessor's `gate()`; `Gate::open()` for the first step). Every worker predicate in `take_where` must require `meta.ready.is_open()` (A3, A8). Wake-up: a gate opening does not bump `Channel::Available`. Workers re-check with their `poll_interval` (spec 134, recommended 20 ms). A `Queue::notify_available()` is an optional later optimization and not part of this spec. |
| C14 | **New.** Two incompatible queue families exist. The take-based `queue::Queue` (spec 4; code `queue.rs`) is used by the Processor, the worker pool and the specs 134 and 135. The list-based family (spec 25 `queue_list`, `queue_iter::QueueNode`, `claim_entry`) is used by the specs 71, 106, 117, 131, 137 and 139. `queue::Queue` does not implement `queue_iter::QueueNode`, so `CpuSubengine::lane_resource<N: QueueNode>` cannot be fed with the Processor's queues. | Decision (derived, owner may override): the reference path uses the take-based family because the Processor is implemented and specs 134 and 135 use it. The specs 71, 106, 117, 131, 137 and 139 must be re-based on `take_where` and `Claim` (A9). The batch scan of spec 117 becomes repeated `take_where` with a batch-key predicate on `meta`. |
| C15 | **New.** No failure path existed: what happens to the task, the counters and the pending successors when a step fails. | E5 `fail` and E6. |

## 7. Binding contracts and amendments for other specs (not for the implementation agent)

The specs below do not exist as code yet, except for 37 and 65 where noted. Each amendment is the text that the spec owner applies.

- **A1, spec 66.** Replace the interface block of §3 and the rules of §4 with §3.1 and §4.1 of this spec.
- **A2, spec 78.** In §3 replace `use operation_node::{OperationNode, OperationKind};` with `use operation_node::{OperationNode, HasOperationKind};`. Replace the bound `O: HasResourceRequirements + OperationKind + Send + Sync + 'static` with `O: HasResourceRequirements + HasOperationKind + Send + Sync + 'static`. In §5 replace `requires O: HasResourceRequirements + OperationKind` with `requires O: HasResourceRequirements + HasOperationKind`. Its nodes keep `ends_chain == true`. Add to `ChainInput` the variant `Upstream(OutputHandle<RamData>)` for steps whose input is the output of a planned predecessor. The handover code reads it when the step is executed.
- **A3, spec 126.**
  - `OpMeta` implements `HasResourceRequirements` (returns `&self.resource`) and `HasOperationKind` (returns `self.kind`).
  - `OpMeta` gets the field `ready: Gate`. The first step of a chain has `Gate::open()`. Every other step has the `gate()` of its predecessor's `OutputHandle`.
  - The `ValueBuilder` builds the nodes in chain order, creates `TaskContext::new(task_id, priority, steps.len())` once per chain, and returns `V = StepPayload<PlannedOperation, I, T, D, E>` for every step.
  - For every node it sets `ends_chain == false` except for the last one, and sets `materialize` equal to `meta.materialize`. The input of step `k+1` holds `nodes[k].output_handle()`.
  - The Planner never reads a `Gate`. It only passes it through.
- **A4, spec 65.** Add `DeviceToRam` to the variant list and state that the mapping treats the enum as `#[non_exhaustive]`.
- **A5, spec 78.** Add a note: the rule "`materialize` is `true` only for the last step" applies to chains built by `start_chain` itself. For a planned chain the flag comes from the Planner.
- **A6, specs 134 and 117 (runner).** The runner executes the operation of the claimed `StepPayload`, then calls `payload.finish(output, sink)`. If the execution fails, or `finish` returns an error, it calls `payload.fail(error, withdraw)` with `withdraw` built from `withdraw_pending` over all three ops queues and the predicate `m.task_id == payload.node.task_id()`. Afterwards it completes the queue `Claim`. The `Claim::complete(Result<R,E>)` result is only the queue-entry result.
- **A7, spec 135.** `V` is `StepPayload<PlannedOperation, I, T, D, E>` for the reference engine. The engine supplies the `FollowUpSink`, which pushes follow-up nodes to the queue chosen by the Planner. If the stepwise mode of spec 78 is used, `T` must be `RamData`.
- **A8, specs 134, 106, 117, 131, 137, 139 (worker predicates).** Every predicate that selects an item must also require `meta.ready.is_open()`. This holds for the seed claim and for batch members.
- **A9, specs 71, 106, 117, 131, 137, 139, 92, 25.** Re-base on the take-based `queue::Queue`: the loop claims with `take_where`, and `BatchMeta` and `HasResourceRequirements` are read from `Claim::meta()` and the predicate. `claim_entry`, `queue_iter::QueueNode` and `iter_active` are not used on the ops queues.

**Residual decisions the owner may override:** `O = PlannedOperation` (C3), the take-based queue family (C14), polling instead of a wake-up for opened gates (C13).
