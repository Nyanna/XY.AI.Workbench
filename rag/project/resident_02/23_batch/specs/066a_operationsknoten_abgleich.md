# Specification: Operationsknoten, reconciliation (build order 66a)

This spec replaces the earlier text of build order 66a (revision 1). It amends build order 66 (`operation_node`) and adds the module `step_completion`. It follows the owner decisions D2 (sub-engines per 137/139 on worker-pool threads), D3-B (staged release) and D8 (idle notices only prioritise), and the recommended options D1-A take queues, D4-a `StepOutput`, D6-a no cancel after claim, D9-a `PlannedOperation` (`decisions.md` of the analysis 66b). `finish` and `fail` run on any worker-pool thread: for CPU work on the worker that executed it, for GPU work on the worker that handles the completion. §3 to §5 are the implementation. §6 is the register of findings. §7 points to the binding texts for other specs; they are not part of the implementation.

## 1. Task, restrictions and boundaries

Rework the Rust module `operation_node` and add the module `step_completion`. Both are in `src/resident/l130/l120/l110/l100/l090/l080/l070/l060/l050/a/`.

Changes to `operation_node`:
- **E0. Operation kind.** Replace the trait `OperationKind` (`&'static str`) with `HasOperationKind { fn operation_kind(&self) -> OperationKind }`. `OperationKind` is the existing enum `operation::OperationKind`. `new` requires `O: HasResourceRequirements + HasOperationKind` and logs the enum with `{:?}`.
- **E1. Chain-end marker.** Add the flag `ends_chain` (default `true`). A node without follow-up nodes completes the task only if the flag is `true`.
- **E2. Task ID.** The node keeps its task ID and exposes `task_id()`.
- **E3. Output sharing.** The node exposes a cloneable read-only `OutputHandle<T>` on its output.
- **E4. Reference operation type.** `PlannedOperation` (from `expander`) implements `HasResourceRequirements` and `HasOperationKind`. The alias `PlannedNode` names the node type of the reference path.

New module `step_completion`:
- **E5. Step completion.** `StepPayload` bundles a node, the `TaskContext` of its task and the planned rest of its chain. `finish` and `fail` run the sequence that follows the execution of a step: `set_output`, `advance`, registering and submitting the next planned step and run-time follow-ups, counter handling, error handling.
- **E6. Withdrawal.** `withdraw_pending` and `withdraw_planned` remove Pending entries of a task from a `queue::Queue` (for fan-out and for replacing steps that were already pushed).

Everything else of spec 66 is unchanged: parts, write-once output, continuation handler, errors.

Out of scope:
- All exclusions of spec 66 (executing operations, scheduling, queueing, interpreting the materialization flag, resolving resources, creating tasks, validating parts).
- Planner, `ValueBuilder`, sink implementations, workers, sub-engines, batch layer. Their binding rules are in `amendments.md` of 66b.
- Any change to a module other than the two named above and `mod.rs` of the same directory.

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

Decisions:
- Both chain-creation modes must be possible: the full chain created in advance, and the chain created stepwise by continuation handlers. A chain may also be modified during a run.
- The reference engine plans the full chain in one `expand` call. Only the first step is pushed; the remaining steps travel inside the payload (`StepPayload::rest`) and are pushed by `finish` after their predecessor's output is set (staged release). Queue order between the steps of a task therefore needs no readiness test in any worker.
- Queues are the take-based `queue::Queue<M,P,R,E>` (`push`, `take_where`, `Claim`). `M` (the placement) is opaque to `step_completion`; the reference uses `planner::OpMeta`.
- The output type `T` of the reference path is `chain_handover::StepOutput` (RAM or device). This module stays generic over `T`.
- The `Task` is completed only by the node whose `ends_chain()` is `true`. The `TaskContext` counter tells when every step is accounted for.

Current state (verified):
- `operation_node.rs` exists with the spec 66 behavior and the string trait `OperationKind`. `advance()` completes the task whenever there is no handler or the handler returns an empty list.
- `operation::OperationKind` is `#[non_exhaustive]`, `Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord`, variants Tokenization, Embedding, Pooling, L2Normalization, Similarity, TopK, IndexWrite, Inference, DeviceToRam.
- `expander::PlannedOperation { pub kind, pub descriptor, pub resource: resource_requirements::ResourceRequirements, pub config: OperationConfig }` is `Clone, Debug, PartialEq`; `OperationConfig` implements `Default`.
- `TaskContext<E>`: `new(id, priority, initial_open) -> Arc<Self>`, `id()`, `open_operations()`, `is_cancelled()`, `first_error()`, `register_follow_ups(n)`, `complete_operation() -> bool` (true only from 1 to 0 and not cancelled; at 0 a `debug_assert` failure), `fail(error) -> bool` (true for the first error; sets cancelled).
- `queue::Queue<M,P,R,E>`: `push(meta, payload) -> Ticket`, `take()`, `take_where(FnMut(&M)->bool) -> Option<Claim>`, `len()` (Pending only). Dropping a `Claim` without `complete` sets `Abandoned`.
- `Task::complete` a second time panics in debug builds (observed `Completed`); for `Enqueued` and `Cancelled` it returns `Err(NotClaimed(state))`. `Task::cancel` succeeds only from `Enqueued`.
- `CompleteError` and `QueueState` live in `.../l050/l040/l030/l020/l010/queue_state.rs`.

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

    pub fn set_continuation(&mut self, handler: ContinuationHandler<O, I, T, D, E>);
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

### 3.2 `step_completion`

```rust
use std::sync::Arc;
use operation_node::{AdvanceError, OperationNode};
use task_context::TaskContext;
use queue::Queue;

/// A planned successor: its queue placement (opaque here) and its node.
pub struct PlannedStep<M, O, I, T, D, E> {
    pub placement: M,
    pub node: OperationNode<O, I, T, D, E>,
}

/// Pushes the payload of a follow-up step. `placement` is `None` for nodes that a continuation
/// handler created at run time; the implementation determines their placement itself.
pub trait FollowUpSink<M, P>: Send + Sync {
    fn submit(&self, placement: Option<M>, payload: P);
}

/// One queue entry: a node, the context of its task and the planned rest of its chain.
pub struct StepPayload<M, O, I, T, D, E> { /* node, context, rest: Vec<PlannedStep<..>> */ }

impl<M, O, I, T, D, E> StepPayload<M, O, I, T, D, E> {
    /// If `rest` is not empty, sets `node.ends_chain()` to false; otherwise leaves it unchanged.
    pub fn new(node: OperationNode<O, I, T, D, E>, context: Arc<TaskContext<E>>,
               rest: Vec<PlannedStep<M, O, I, T, D, E>>) -> Self;
    pub fn node(&self) -> &OperationNode<O, I, T, D, E>;
    pub fn context(&self) -> &Arc<TaskContext<E>>;
    /// rest()[0] is the next step.
    pub fn rest(&self) -> &[PlannedStep<M, O, I, T, D, E>];
    /// Replaces the planned rest and returns the old one. Sets `ends_chain` to `rest.is_empty()`.
    pub fn replace_rest(&mut self, rest: Vec<PlannedStep<M, O, I, T, D, E>>) -> Vec<PlannedStep<M, O, I, T, D, E>>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StepError { OutputAlreadySet, Advance(AdvanceError) }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StepReport {
    /// Number of payloads submitted.
    pub follow_ups: usize,
    /// Result of the last `complete_operation()` of this call.
    pub task_finished: bool,
    /// The task was already cancelled: the output was not stored and `advance` was not called.
    pub discarded: bool,
}

impl<M, O, I, T: Clone, D, E> StepPayload<M, O, I, T, D, E> {
    /// Stores the result of a successfully executed step and continues the chain.
    pub fn finish<S>(&mut self, output: T, sink: &S) -> Result<StepReport, StepError>
    where S: FollowUpSink<M, StepPayload<M, O, I, T, D, E>> + ?Sized;
}

impl<M, O, I, T, D, E: Clone> StepPayload<M, O, I, T, D, E> {
    /// Handles a failed step. `withdraw` removes Pending entries of the task that were already
    /// pushed and returns how many it removed (see `withdraw_pending`).
    pub fn fail<W: FnOnce() -> usize>(&mut self, error: E, withdraw: W) -> StepReport;
}

/// Claims and drops (-> Abandoned) every Pending item accepted by `matches`. Returns the count.
pub fn withdraw_pending<M, P, R, E>(queue: &Queue<M, P, R, E>, matches: impl FnMut(&M) -> bool) -> usize;

/// `withdraw_pending`, plus one `complete_operation()` on `context` per removed item.
pub fn withdraw_planned<M, P, R, E, X>(queue: &Queue<M, P, R, E>, matches: impl FnMut(&M) -> bool,
                                       context: &TaskContext<X>) -> usize;
```
(The bounds on `M, P, R, E` of the two functions are those of `Queue`; take them from `queue.rs`.)

`StepPayload` is `Send` when `M: Send` and the node is `Send`; it is `Sync` when `M: Sync` and the node is `Sync`.

## 4. Behavior and invariants

### 4.1 `operation_node`

Identical to spec 66 §4, with these changes.

**Construction**
- `new` logs exactly one `debug!` record with the task ID, the `Debug` text of the `OperationKind` (for example `Embedding`), the `Debug` text of the `ResourceKind` (for example `Scorer`) and the materialization flag. It does not panic and does not touch the task state.
- A new node has `ends_chain() == true` and `task_id()` equal to the argument.
- The module never matches on `OperationKind` or `ResourceKind`. Only their `Debug` text is used.

**Output sharing**
- `output_handle()` can be called any number of times. All handles and the node see the same cell. `OutputHandle::get()` returns `None` until `set_output` succeeded, and then the same value as `output()`.
- Handles keep the output cell alive after the node is dropped.

**Chain-end marker**
- `set_ends_chain` and `with_ends_chain` change only the flag. The latest value wins.
- The module does not know the chain and does not check how many nodes end it.

**Advance**
- `advance()` before `set_output` returns `Err(OutputNotSet)`. It does not call the handler and leaves the task unchanged.
- Follow-up nodes from the handler are returned; the task is not completed.
- With `ends_chain == true` and no follow-up nodes the task is completed with `Ok(clone of the output)`. A `CompleteError` is returned as `AdvanceError::Complete`.
- With `ends_chain == false` and no follow-up nodes `advance` returns an empty `Vec` and does not call `complete`. A repeated call does the same and never panics.
- `advance()` can be called repeatedly and invokes the handler each time. A repeated chain end (`ends_chain == true`) is a second completion: it panics in debug builds and returns `Complete(NotClaimed(Completed))` in release builds.

Unchanged: a new node has an empty output, no handler and exactly the given parts. `set_output` succeeds once; under concurrent calls exactly one wins. `set_output` does not complete the task. The latest handler wins. Handlers are called without added locking.

### 4.2 `step_completion`

**`new` and `replace_rest`**
- `new` sets `ends_chain` to `false` if `rest` is not empty; with an empty `rest` the node's flag is kept, so nodes of the stepwise mode keep their own setting.
- `replace_rest` returns the old rest unchanged (it is neither registered nor submitted) and sets `ends_chain` to `rest.is_empty()`. It is the way to modify the not yet pushed part of a planned chain.

**`finish(output, sink)`**, in this order:
1. If `context.is_cancelled()`: call `context.complete_operation()` once, drop the rest, call neither `set_output` nor `advance`, and return `Ok(StepReport { follow_ups: 0, task_finished: <that result>, discarded: true })`.
2. `node.set_output(output)`. On `Err` return `Err(OutputAlreadySet)`. Counters and rest are untouched.
3. `node.advance()`. On `Err(e)` return `Err(Advance(e))`. Counters and rest are untouched. The caller normally calls `fail`.
4. Let `extra` be the returned nodes (`m` of them) and `next` the first element of the rest, if any. `n = m + (1 if next exists)`. If `n > 0`, call `context.register_follow_ups(n)` before any submission.
5. If `next` exists: `sink.submit(Some(next.placement), StepPayload::new(next.node, context.clone(), remaining rest))`. Then, for each node of `extra` in order: set its `ends_chain` to `true` only if no `next` exists and it is the last of `extra`, otherwise to `false`; then `sink.submit(None, StepPayload::new(node, context.clone(), Vec::new()))`.
6. `context.complete_operation()` for this step, always after step 5.
7. Return `Ok(StepReport { follow_ups: n, task_finished: <result of step 6>, discarded: false })`.

**`fail(error, withdraw)`**, in this order:
1. `context.fail(error.clone())`. If it returned `true` and `node.task().state() == Claimed`, call `node.task().complete(Err(error))`. A `CompleteError` is only logged at `debug!`.
2. Drop the rest. Its steps were never registered, so the counter is not touched for them.
3. `let removed = withdraw();` then call `context.complete_operation()` `removed` times.
4. `context.complete_operation()` once for this step.
5. Return `StepReport { follow_ups: 0, task_finished: <result of step 4>, discarded: false }`; `task_finished` is `false` because the context is cancelled.

**Counting invariant.** The context is created with `initial_open == 1` for a planned chain (only the head is pushed). Every submitted payload is registered before it is submitted. Every pushed step is accounted for exactly once: by `finish`, by `fail` or by withdrawal. The carried rest is never counted.

**Fan-out.** If several nodes end up without follow-ups, only the one marked in step 5 completes the task. The other branches must not be needed for the task result.

**`withdraw_pending`** loops `queue.take_where(&mut matches)` until it returns `None`, drops each claim and counts them. Dropped claims become `Abandoned`. It never blocks and does not wait for running items. `withdraw_planned` additionally calls `context.complete_operation()` once per removed item, after the removal. With `planner::OpMeta` as metadata the predicate compares `m.task_id.0 == node.task_id()`.

**Concurrency.** All functions can be called from any thread. No lock is held. In a linear chain only one step of a task exists in a queue or in execution at any time, so `finish` and `fail` of the same task never overlap. With fan-out they can; then a `fail` may observe `Claimed` while another branch completes the task, and the second `complete` panics in debug builds. Fan-out branches must therefore not fail after the task end node may have run.

## 5. Relied-on elements, implementation steps, acceptance

Paths are relative to `crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050`.
- `l040::l030::b::operation::OperationKind` (only `Debug` is used).
- `l040::a::resource_requirements::{HasResourceRequirements, ResourceKind, ResourceRequirements, ExecutionEnvironment}`.
- `a::expander::{PlannedOperation, OperationConfig}` and what its tests use to build values.
- `l040::l030::l020::b::task_model::{Task, TaskParams}`: `Task::new(data, TaskParams)`, `claim()`, `complete()`, `state()`, `result()`. `TaskParams` implements `Default`.
- `l040::l030::l020::l010::queue_state::{CompleteError, QueueState}`.
- `l040::l030::b::task_context::TaskContext`.
- `l040::l030::l020::l010::queue::{Queue, State}`. `Ticket::state()` is used by tests to see `Abandoned`.
- `log` crate.

Implementation steps:
1. `operation_node.rs`: implement E0 to E4 as in §3.1. Store the output in an `Arc<OnceLock<T>>`. Add `ends_chain` and `task_id`. Add the two impls for `PlannedOperation` and the alias.
2. `test/operation_node.rs`: change `TestOp` to hold an `OperationKind` and a `ResourceRequirements` and to implement `HasOperationKind`. Keep all existing cases. Replace the log assertion: the record must contain the task ID, the `Debug` text of the chosen `OperationKind`, the `Debug` text of the chosen `ResourceKind` and the flag (for example `Pooling` and `Scorer`). Keep the log test tolerant when a global logger is already installed. Keep the repeated-completion test split by `cfg(debug_assertions)`.
3. Node tests:
   - `operation().operation_kind()` and `task_id()` return the construction values. `ends_chain()` is `true` for a new node; setter and builder change it.
   - `ends_chain == false`, no handler or empty handler result: `advance` returns an empty `Vec`, the task stays `Claimed` without result; repeated calls give the same result; the handler is called once per `advance`.
   - `ends_chain == false`, handler returning nodes: the nodes are returned, the task is not completed.
   - Three nodes on one claimed task with `ends_chain` `false, false, true`: only the third `advance` completes the task, with the third output.
   - `output_handle()`: `None` before, equal to `output()` after `set_output`; clones see the same value; the handle still reads the value after the node is dropped; a second `set_output` does not change it. `Debug` does not require `T: Debug`.
   - A successor whose `I` holds the predecessor's `OutputHandle` reads the predecessor's output after the predecessor's `set_output`.
   - `PlannedOperation` implements both traits with `resource.kind()` and `kind`; a `PlannedNode` can be built from a real `PlannedOperation`.
   - Compile-time check: `OperationNode<TestOp, u32, String, String, String>` and `OutputHandle<String>` are `Send + Sync`.
4. `step_completion.rs` and `test/step_completion.rs`: implement §3.2 and §4.2. Register `pub mod step_completion;` in `.../l050/a/mod.rs`. Tests use a recording sink, a sink that pushes into a real `queue::Queue<u32, StepPayload<..>, (), ()>`, and a `TaskContext<String>`:
   - Planned chain of three steps: the head is pushed alone; `len()` of the queue is 1; the second step appears only after the first `finish`; only the third `finish` returns `task_finished == true`; the task result is the third output; `open_operations()` is 0 at the end.
   - `new` with a non-empty rest sets `ends_chain` to `false`; with an empty rest it keeps the flag; `replace_rest` returns the old rest and updates the flag.
   - `finish` with a handler returning two nodes and an empty rest: both are submitted with placement `None`; only the last has `ends_chain == true`; `open_operations()` at the moment of the first `submit` already includes both; the final count is `initial + 2 - 1`.
   - `finish` with a handler returning one node and a non-empty rest: the planned next step is submitted first with `Some(placement)`; the extra node has `ends_chain == false`; the count rises by two before the submissions.
   - `finish` on a cancelled context: `discarded == true`, no output stored, the counter decreases by one, nothing is submitted.
   - `finish` when the output is already set: `Err(OutputAlreadySet)`, counters and rest untouched. `finish` when `advance` fails (task not `Claimed`): `Err(Advance(Complete(..)))`, counters untouched.
   - `fail`: the first error completes the task with `Err`; the rest is dropped and not counted; a second `fail` for the same task does not call `complete` and does not panic; `withdraw` is called once; `open_operations()` ends at 0; `task_finished` is `false`.
   - `fail` on a task that is already `Completed` does not panic in debug builds.
   - `withdraw_pending`: removes exactly the matching Pending items, leaves the others Pending in order, leaves a claimed item untouched, returns the count; removed tickets report `Abandoned`. `withdraw_planned` also reduces `open_operations()` by the count.
5. Run `runTests`. All tests must pass. Only these files may change: `operation_node.rs`, `step_completion.rs`, their two test files and `.../l050/a/mod.rs`.

Acceptance:
- No `trait OperationKind` exists in the crate.
- A node that never calls `set_ends_chain` behaves like spec 66.
- `operation_node` exports `HasOperationKind`, `ContinuationHandler`, `PlannedNode`, `AdvanceError`, `OperationNode`, `OutputHandle`. `step_completion` exports `PlannedStep`, `FollowUpSink`, `StepPayload`, `StepError`, `StepReport`, `withdraw_pending`, `withdraw_planned`.

## 6. Register of findings and resolutions

| ID | Finding | Resolution |
|---|---|---|
| C1 | Spec 66's trait `OperationKind` collides with the enum. | E0. |
| C2 | Spec 78 imports the old trait. | Amendment to 78. |
| C3 | No type satisfied the node's bounds. | E4; `OpMeta` gets both impls (amendment to 126). |
| C4 | Two chain-creation modes. | Both supported: planned rest (E5) and handler follow-ups. |
| C5 | Materialization rules of 78 and 126 differ. | Planned chains use the Planner's flag; 78's rule applies to its own chains; intermediate outputs may be `StepOutput::Device`. |
| C6 | The node did not keep its task ID. | E2. |
| C7 | Payload, runner call sequence and `Claim::complete` vs. `Task::complete` were undefined. | E5; runner contract in 117/137/139; claims are completed with `Ok(())` after the runner returned; only the `ends_chain` node completes the task. |
| C8 | Spec 65 lists `OperationKind` without `DeviceToRam`. | Amendment to 65. |
| C9 | Two `ResourceRequirements` and two `TaskId` types. | The node uses the typed `ResourceRequirements` and raw `u64`; withdrawal compares `m.task_id.0`. |
| C10 | Spec 66 completes the task at every node without follow-ups. | E1; `StepPayload::new` sets the flag for planned nodes. |
| C11 | A planned successor cannot read its predecessor's output. | E3: the successor's input holds the predecessor's `OutputHandle`. |
| C12 | Run-time modification of planned chains. | `replace_rest` for the carried part; handler follow-ups for insertion; `withdraw_*` for already pushed entries. |
| C13 | Steps of one task could be claimed out of order. | Staged release: only runnable steps are pushed. No readiness test in workers. |
| C14 | Two incompatible queue families. | Take family for task and ops queues; list-family specs re-based (66b). |
| C15 | No failure path. | `fail`: first error completes the task, the rest is dropped, already pushed entries can be withdrawn. |
| C16 | Token-length batching needs data-dependent metadata. | Staged release lets the pusher fill `OpMeta.batch` from the payload. |
| C17 | Nobody claimed the task; data was unreadable without a claim. | The Planner claims the task in `expand` (amendment to 126 and 135). |
| C18 | `FollowUpSink` could not build queue entries. | Sink receives the placement and a complete `StepPayload`. |

## 7. Binding texts for other specs

The exact amendments are in `project/resident_02/25_implement/analysis/066b/amendments.md`. They are not part of this implementation.
