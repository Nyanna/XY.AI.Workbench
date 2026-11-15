# Operation node: spec amendments (build order 66b)

The amendments implement the owner decisions D2 (sub-engines per 137/139), D3-B (staged release) and D8 (idle notices are a pure prioritisation hint), plus the recommended options D1-A, D4-a, D5-ii, D6-a, D7-a, D9-a (`decisions.md`). They also follow the module requirements in `project/resident_02/21_modules_ordered.yml`: the worker-pool threads are the only workers of the engine; a worker runs CPU work synchronously, only launches GPU work, and a free worker processes a GPU result after the completion notification.

Old text is quoted exactly as found in `project/resident_02/23_batch/specs/`. "Insert after" names an anchor that is quoted exactly. Where a whole block is replaced, its first line is quoted and the complete new block is given. Spec 065 is written in German; its new text is given in English as required for project documents.

## Status of the 66a amendments

| 66a | Status | Reason |
|---|---|---|
| A1 (66) | changed | 66 §3/§4 are replaced by the revised 66a (`066a_r1_draft.md` §3.1/§4.1): no `Gate` (D3-B), `OutputHandle` kept. |
| A2 (78) | changed | Import and bound change confirmed; `T` becomes `StepOutput` (D4-a); `ChainInput` gets `Upstream` and `Data(StepOutput)`. |
| A3 (126) | changed | `OpMeta` impls confirmed; `ready: Gate` dropped (D3-B); `TaskContext::new(.., 1)`; Planner claims the task (N1); `batch` field (N3); `place_step` (D5). |
| A4 (65) | confirmed | `DeviceToRam` added. |
| A5 (78) | changed | Note kept and extended by `StepOutput::Device` for intermediate steps (F9). |
| A6 (134, 117) | changed | Runner contract in 117 and 137/139: CPU batches finish on the worker thread, GPU batches finish in the `GpuResults` stage of 131. No withdrawal for planned chains (D3-B); task id comparison `meta.task_id.0 == node.task_id()` (N7) where withdrawal is used. |
| A7 (135) | changed | `V = EngineStep` with `StepPayload<OpMeta, ..>` and a sink with placement (N6); `T = StepOutput`; the facade's `WorkSource` runs the Worker-Loop. |
| A8 (gate predicates) | dropped | Not needed under D3-B. |
| A9 (re-base) | changed | Extended to 092, 130, 131, 134 and the role note in 025; 071 is withdrawn. |

---

## 025 Queue-Struktur und Enqueue

**§1, insert after** "Implement, in Rust, the queue data structure and its append operation."

New paragraph:
> Role: this list queue is implemented as module `queue_list`. It is not used for the task queue or the operations queues of the engine; those use the take-based `queue::Queue` (build order 4).

## 035 Task-Kontext

**§2**, old:
> The system uses one `TaskContext` per task. Every operation of the task holds an `Arc` clone, and operations are queue entries (`Task`/`EntryState`).

New:
> The system uses one `TaskContext` per task. Every operation of the task holds an `Arc` clone. Operations are items of the take-based operations queues (`queue::Queue`); their payload (`step_completion::StepPayload`) holds the clone.

**§2**, old:
> - **Claim check:** The queue or worker checks `is_cancelled()` when claiming. Operations of a cancelled task must fail the claim. This module only provides the flag. The claim itself is done by `queue_state`.

New:
> - **Cancel check:** The step completion (`StepPayload::finish`) checks `is_cancelled()` after the operation ran and discards its output. The operations queue has no cancel state. This module only provides the flag.

**§2**, old:
> - **Completion tracking:** Task completion is tracked only by this counter, never by the result of the `queue_state` entry.

New:
> - **Completion tracking:** The counter tells when every operation of the task is accounted for. The `Task` itself is completed by the operation node that ends the chain (`OperationNode::advance` with `ends_chain() == true`). In a linear chain the `true` of `complete_operation()` belongs to that same step.

**§5**, old:
> - Operations that find the task cancelled via `is_cancelled()` must fail the claim. The claim error for that case is `ClaimError::Cancelled` of `queue_state`. Producing it is not the job of this module.

New:
> - Operations that find the task cancelled via `is_cancelled()` discard their output (`StepPayload::finish`). This module does not produce that behavior.

## 036 Operation

**§2**, old:
> - Workers filter on `OperationParams`/`requirements()` before claiming.

New:
> - Workers filter on the queue metadata (`planner::OpMeta`) before claiming. `OpMeta.descriptor` carries this module's `OperationDescriptor`.

**§2, insert after** "- A Task (Task-Modell) holds its operations as data. Operation order within the task is the implicit graph: the output of operation *n* feeds operation *n+1*, wired by sharing a `RamSlot` or a device region between them."

New bullet:
> - The reference engine does not use `Operation<R>`. Its steps are `operation_node::OperationNode<PlannedOperation, ..>`; the hand-over uses `OutputHandle` and `chain_handover::StepOutput`. `OperationKind`, `OperationParams`, `OperationDescriptor`, `RamData`, `DataShape` and the scored-hit types stay in use.

## 056 Worker-Pool

**§2**, old:
> The Facade implements `WorkSource` and chooses compatible items inside `run_once`, for example with `take_where` over `Operation::requirements()`.

New:
> The Facade implements `WorkSource`: `run_once(worker)` runs one step of that worker's `WorkerLoop` (build order 131), which feeds the GPU and CPU sub-engines and expands tasks.

**§5**, old:
> From Operation (informational):
> - `Operation::requirements()` is what the Facade filters on inside `run_once`.

New:
> From Worker-Loop (informational):
> - `WorkerLoop::run_once(now) -> Option<Stage>`; `Some` maps to `Step::Worked`, `None` to `Step::Idle`.

## 065 Expander

**§1**, old:
> - Es führt keine Operation aus und baut keine `Operation<R>` (keine Regionen, keine `TaskId`, keine Priority). Der Planner baut daraus die `Operation<R>`.

New:
> - It executes no operation and builds no `Operation<R>` (no regions, no `TaskId`, no priority). In the reference engine `PlannedOperation` itself is the operation of an operation node (`OperationNode<PlannedOperation, ..>`); nobody builds an `Operation<R>` from it.

**§3**, old:
> /// Eine geplante Operation samt Konfiguration.
> #[derive(Clone, Debug)]
> pub struct PlannedOperation {

New:
> /// A planned operation with its configuration.
> #[derive(Clone, Debug, PartialEq)]
> pub struct PlannedOperation {

**§3**, old:
> /// Nicht belegte Felder sind None.
> #[derive(Clone, Debug, Default)]
> pub struct OperationConfig {

New:
> /// Fields that are not used are None.
> #[derive(Clone, Debug, Default, PartialEq)]
> pub struct OperationConfig {

**§5**, old:
> - `OperationKind` (`#[non_exhaustive]`) mit den Varianten `Tokenization`, `Embedding`, `Pooling`, `L2Normalization`, `Similarity`, `TopK`, `IndexWrite`, `Inference`.

New:
> - `OperationKind` (`#[non_exhaustive]`) with the variants `Tokenization`, `Embedding`, `Pooling`, `L2Normalization`, `Similarity`, `TopK`, `IndexWrite`, `Inference`, `DeviceToRam`. The mapping treats the enum as `#[non_exhaustive]`; the expander never produces `DeviceToRam`.

## 066 Operationsknoten

**§3 and §4**: replaced as a whole by §3.1 and §4.1 of `066a_r1_draft.md`.

**§2**, old:
> - The Planner builds operation nodes. A sub-engine executes the operation of a node, sets the output, and then asks the node for its follow-up nodes.

New:
> - The `ValueBuilder` of the Planner builds the operation nodes of a planned chain; the stepwise hand-over (build order 78) builds them one by one. A sub-engine executes the operation of a node; the step completion (`step_completion`) sets the output and asks the node for its follow-up nodes.

## 066a Operationsknoten, reconciliation

Replaced as a whole by `066a_r1_draft.md` (same build order 66a, revision 1). The differences:
- E3: `OutputHandle` stays; `Gate` and `OutputHandle::gate()` are removed.
- E5: `StepPayload<M, O, I, T, D, E>` gets `rest: Vec<PlannedStep<M, O, I, T, D, E>>`; `finish`/`fail` take `&mut self`; `StepPayload::new` sets `ends_chain` to `false` when `rest` is not empty; `replace_rest`; run-time follow-ups get `ends_chain == true` only for the last one and only without a planned rest; `FollowUpSink<M, P>::submit(Option<M>, P)`.
- E6: `withdraw_pending` stays; `withdraw_planned` stays as a tool for fan-out and replacement of already pushed steps.
- §6 and §7 updated to the results of this analysis.

## 071 Pull-Ausführung

**§1**, old:
> Implement the pull loop of **one sub-engine**. The loop has exactly one wait condition, the sub-engine's notification. After each wake-up it iterates over the queues, claims suitable entries, executes them and completes them.

New:
> Status: withdrawn. The engine's operations queues are take-based (`queue::Queue`, build order 4). The pull loop is the `WorkerLoop` (build order 131) on the worker-pool threads; its only wait condition is the worker pool's notification (build order 56), and every notification of a sub-engine (build order 92) leads to it. Nothing is implemented for this build order.

## 078 Schrittweise Übergabe und Ergebnisaufbereitung

**§2**, old:
> - The handler returns the follow-up node, or nothing at the end of the chain. Without a follow-up, `OperationNode::advance` completes the `Task` with `Ok(output.clone())`. The task result type is therefore `RamData`.

New:
> - The handler returns the follow-up node, or nothing at the end of the chain. Without a follow-up, `OperationNode::advance` completes the `Task` with `Ok(output.clone())`. The task result type is therefore `StepOutput`. Intermediate outputs may stay on the device (`StepOutput::Device`); the last output is always in RAM.

**§3**, old:
> use operation::{RamData, DataShape, EntryId as OpEntryId};

New:
> use operation::{RamData, DataShape, EntryId as OpEntryId};
> use materialization::DeviceResult;

**§3**, old:
> use operation_node::{OperationNode, OperationKind};     // OperationNode<O,I,T,D,E>

New:
> use operation_node::{OperationNode, HasOperationKind, OutputHandle};     // OperationNode<O,I,T,D,E>

**§3**, old:
> /// Input of a node. Embedding/Similarity: `Data`. Inference (first step): `TextPair(query, passage)`.
> #[derive(Debug, Clone, PartialEq)]
> pub enum ChainInput { Data(RamData), TextPair(String, String) }
>
> /// Node type used by this module: I = ChainInput, T = RamData.
> pub type ChainNode<O, D, E> = OperationNode<O, ChainInput, RamData, D, E>;

New:
> /// Output of one step. `Device` only for steps with `materialize == false`.
> #[derive(Debug, Clone)]
> pub enum StepOutput { Ram(RamData), Device(DeviceResult) }
>
> /// Input of a node. `Data`: initial data or the actual output of the previous step.
> /// `Upstream`: output of a planned predecessor, read when the step executes.
> /// Inference (first step): `TextPair(query, passage)`.
> #[derive(Debug, Clone)]
> pub enum ChainInput { Data(StepOutput), Upstream(OutputHandle<StepOutput>), TextPair(String, String) }
>
> /// Node type used by this module and by the planned reference chain: I = ChainInput, T = StepOutput.
> pub type ChainNode<O, D, E> = OperationNode<O, ChainInput, StepOutput, D, E>;

**§3**, old:
> pub enum ResultError {
>     /// No result: empty chain, last operation never stored one, or already taken.
>     MissingResult,

New:
> pub enum ResultError {
>     /// No result: empty chain, last operation never stored one, or already taken.
>     MissingResult,
>     /// The stored result is still on the device (`StepOutput::Device`).
>     NotMaterialized,

**§3**, old:
> pub struct Collector { /* Arc<Mutex<Option<RamData>>>, ChainKind, task_id: u64 */ }

New:
> pub struct Collector { /* Arc<Mutex<Option<StepOutput>>>, ChainKind, task_id: u64 */ }

**§3**, old:
>     task: Task<D, RamData, E>,
> ) -> ChainStart<O, D, E>
> where O: HasResourceRequirements + OperationKind + Send + Sync + 'static,

New:
>     task: Task<D, StepOutput, E>,
> ) -> ChainStart<O, D, E>
> where O: HasResourceRequirements + HasOperationKind + Send + Sync + 'static,

**§4 Start**, old:
> - `materialize` is `true` only for the last step of the chain, otherwise `false`.

New:
> - `materialize` is `true` only for the last step of the chain, otherwise `false`. This rule applies only to chains built by `start_chain`. For planned chains the flag comes from the Planner (build order 126). The nodes built here keep `ends_chain() == true`.

**§4 Start**, old:
>   - **Embedding:** `ChainInput::Data(RamData::TextList(task.input.texts().to_vec()))`. The text count is stored in the `Collector`. An empty list is allowed here.
>   - **Similarity:** `Data(RamData::Floats(..))`. All query vectors are concatenated in query order into one flat sequence. Dimensions are not checked. No queries gives an empty `Floats`.

New:
>   - **Embedding:** `ChainInput::Data(StepOutput::Ram(RamData::TextList(task.input.texts().to_vec())))`. The text count is stored in the `Collector`. An empty list is allowed here.
>   - **Similarity:** `Data(StepOutput::Ram(RamData::Floats(..)))`. All query vectors are concatenated in query order into one flat sequence. Dimensions are not checked. No queries gives an empty `Floats`.

**§4 Collector, insert after** "- Conversion per kind:"

New bullet (first in the list):
>   - The rules below apply to `StepOutput::Ram(data)`. `StepOutput::Device(_)` returns `Err(NotMaterialized)`.

**§5**, old:
>   - `new(task_id: u64, operation: O, input: I, materialize: bool, task: Task<D,T,E>)`, requires `O: HasResourceRequirements + OperationKind`.

New:
>   - `new(task_id: u64, operation: O, input: I, materialize: bool, task: Task<D,T,E>)`, requires `O: HasResourceRequirements + HasOperationKind`.

**§5**, old:
>   - `advance()`: with no handler or an empty result, it completes the task with `Ok(output.clone())`. `T: Clone` is required.

New:
>   - `advance()`: with no handler or an empty result and `ends_chain() == true` (the default), it completes the task with `Ok(output.clone())`. `T: Clone` is required.
>   - `OutputHandle<T>`: cloneable read-only view on a node's output (`get()`).
> - `materialization::DeviceResult`: `Clone + Debug`, a finished device result (region handle and tensor reference).

## 092 Notification-Topologie

The module keeps one notification handle per sub-engine, as required, but every notification wakes the worker pool, because the worker-pool threads run all sub-engines.

**§1**, old:
> - Pass these channels to the queue when it is created, so that every queue append wakes every sub-engine.

New:
> - Offer `notify_push()`, which every producer calls after it pushed into an operations queue, so that every push wakes every sub-engine.

**§1**, old:
> - The loop of a sub-engine is its own notification: it waits on that channel and is woken by every event above.

New:
> - Every notification calls the injected wake function (the worker pool's `Notifier::notify`). The workers that run the sub-engines park only on the worker pool.

**§1 Restrictions**, old:
> - Allowed crates: `tokio` (feature `sync`) and `log`.
> - Allowed project dependency: `queue` (`Queue`, `NotifyChannel`).

New:
> - Allowed crates: `std` and `log`. No project dependency; the wake function is injected.

**§2**, old:
> - `queue::Queue::new(channels: Vec<NotifyChannel>)` stores the channels. Every `enqueue` calls `notify_one()` once per channel, after the append and without blocking. `NotifyChannel = Arc<tokio::sync::Notify>`. `Notify` stores one permit, so a signal is never lost, and several signals may coalesce into one.

New:
> - The operations queues are take-based (`queue::Queue`, build order 4) and know no external channels. Producers (the expansion stage after `Processor::expand`, the engine's `FollowUpSink`) call `NotificationTopology::notify_push()` after each push.
> - The wake function is `worker_pool::Notifier::notify` (build order 56): it increments an epoch and wakes all parked workers, so a signal is never lost.

**§2**, old:
> - A sub-engine loop is: wait for its notification, run an iteration, and repeat if the iteration made progress. This module supplies the wait and the self-trigger, not the loop.

New:
> - The loop is the `WorkerLoop` (build order 131) on the worker-pool threads. This module supplies the self-trigger, not the wait and not the loop.

**§2**, old:
> - Per sub-engine there is exactly one `Notify`. It is the channel given to the queue and the channel of the handle. Queue appends and the own events therefore wake the same loop.

New:
> - Per sub-engine there is exactly one handle. `notify_push()` and the own events of every sub-engine call the same wake function.

**§3**: the whole block from "use std::sync::Arc;" to the closing brace of `impl NotificationTopology` is replaced by:
```rust
use std::sync::Arc;

pub type Wake = Arc<dyn Fn() + Send + Sync>;

/// Per-sub-engine handle. Cheap to clone.
#[derive(Clone)]
pub struct SubengineNotifier { /* private: Wake */ }

impl SubengineNotifier {
    pub fn notify_result(&self);
    pub fn notify_init_done(&self);
    pub fn notify_eviction_timer(&self);
    /// Wakes only if `progress` is true.
    pub fn notify_iteration(&self, progress: bool);
}

pub struct NotificationTopology { /* private: Vec<SubengineNotifier>, Wake */ }

impl NotificationTopology {
    /// Creates `subengines` handles (may be 0), index = sub-engine id.
    pub fn new(subengines: usize, wake: Wake) -> Self;
    pub fn len(&self) -> usize;
    pub fn notifier(&self, index: usize) -> Option<SubengineNotifier>;
    /// Called by producers after a push into an operations queue.
    pub fn notify_push(&self);
}
```

**§4 Topology**: the whole block is replaced by:
> - `new(n, wake)` creates n handles. Every notification of a handle, and `notify_push()`, calls `wake` exactly once.
> - With `new(0, wake)`, `notify_push()` still calls `wake`.

**§4 Own notifications**, old:
> - Signals sent while the loop is not waiting are kept as one permit. Several signals before `notified()` coalesce into a single wake-up. The next `notified()` after that blocks again until a new signal.
> - A signal sent from inside the loop's own iteration (for example `notify_iteration(true)`) makes the next `notified()` return immediately.

New:
> - Signals sent while no worker is parked are not lost: the worker pool compares epochs before parking (build order 56). Several signals may lead to one wake-up.

**§4 Concurrency**, old:
> - A single `Notify` supports one waiter per loop. The caller must not run several loops on one notifier. If they do, each permit wakes only one of them.

New:
> - A signal wakes all parked workers. Any of them may handle the event.

**§5**: the block "From `queue`:" with its four bullets is replaced by:
> From `worker_pool` (injected as `Wake`, not a code dependency): `Notifier::notify()` increments the epoch and wakes all parked workers; it never blocks beyond a short lock.

## 106 Subengine-Loop

**§1**, old:
> Implement one synchronous iteration of a sub-engine lane (`SubengineLoop::run_iteration`). The iteration works through the own queue, then the shared queue, oldest first. For each node it does the following:

New:
> Implement one synchronous iteration of a sub-engine (`SubengineLoop::run_iteration`). It runs on a worker-pool thread inside the `WorkerLoop` (build order 131). The iteration works through the own queue, then the shared queue (take-based `queue::Queue`, build order 4), oldest first. For each entry it does the following:

**§1 Out of scope**, old:
> - No `unsafe`, no blocking, no spawning. Use the `log` crate.

New:
> - No `unsafe`, no spawning. Use the `log` crate. The iteration itself never waits; `ResourceExec::execute` of a CPU resource runs the work synchronously on the calling worker (blocking there is the natural backpressure).

**§2**, old:
> - The queues provide `iter_active(head)` over `QueueNode`s with opaque metadata `M`. Here `M` must expose the requirements (`HasResourceRequirements`).

New:
> - The queues provide `take_where(pred)` over opaque metadata `M`. Here `M` must expose the requirements (`HasResourceRequirements`). A claimed entry cannot return to Pending (dropping its `Claim` makes it `Abandoned`), so the lease is obtained before the claim.

**§2**, old:
> - The task stays `Enqueued` in the queue while its resource is not `Ready`.

New:
> - The entry stays Pending in the queue while its resource is not `Ready`.

**§3**, old:
> use queue_iter::{QueueNode, QueueItem, iter_active};
> use queue_state::{QueueState, claim_entry};

New:
> use queue::{Queue, Claim};

**§3**, old:
> pub trait ResourceExec: Sized + Send + Sync + 'static {
>     type Payload: Send + 'static;
>     /// Non-blocking. True iff a further node could be accepted right now.
>     fn has_capacity(&self) -> bool;
>     /// Takes over the claimed payload and the lease (hold it until execution ends).
>     /// Non-blocking, infallible from the loop's view: the resource runs the node
>     /// (or fails its task) itself.
>     fn execute(&self, payload: Self::Payload, lease: Lease<Self>);
> }

New:
> pub trait ResourceExec: Sized + Send + Sync + 'static {
>     /// The claimed queue entry, normally `queue::Claim<M, P, (), ()>`.
>     type Entry: Send + 'static;
>     /// Non-blocking. True iff a further entry could be accepted right now.
>     fn has_capacity(&self) -> bool;
>     /// Takes over the claim and the lease (hold both until the work is finished).
>     /// CPU resources run the work synchronously on the calling thread; GPU resources only
>     /// launch it and return. Infallible from the loop's view.
>     fn execute(&self, entry: Self::Entry, lease: Lease<Self>);
> }

**§3**, old:
> pub struct SubengineLoop<N, H>
> where N: QueueNode<Payload = H::Payload>, N::Meta: HasResourceRequirements, H: ResourceExec { /* private */ }
>
> impl<N, H> SubengineLoop<N, H>
> where N: QueueNode<Payload = H::Payload>, N::Meta: HasResourceRequirements, H: ResourceExec {
>     pub fn new(own_head: Arc<N>, shared_head: Arc<N>, registry: Arc<ResourceRegistry<H>>,
>                slots: Vec<ResourceSlot>, resolver: ResourceResolver,
>                filter: PullFilter<N::Meta>, init: InitStarter<H>) -> Self;

New:
> pub struct SubengineLoop<M, P, H>
> where M: HasResourceRequirements + Send + Sync + 'static, P: Send + 'static,
>       H: ResourceExec<Entry = Claim<M, P, (), ()>> { /* private */ }
>
> impl<M, P, H> SubengineLoop<M, P, H>
> where M: HasResourceRequirements + Send + Sync + 'static, P: Send + 'static,
>       H: ResourceExec<Entry = Claim<M, P, (), ()>> {
>     pub fn new(own: Arc<Queue<M, P, (), ()>>, shared: Option<Arc<Queue<M, P, (), ()>>>,
>                registry: Arc<ResourceRegistry<H>>, slots: Vec<ResourceSlot>,
>                resolver: ResourceResolver, filter: PullFilter<M>, init: InitStarter<H>) -> Self;

**§4 Order** and **§4 Per node, in this order**: both blocks are replaced by:
> **Order**
> - Process the own queue completely first, then the shared queue (if any).
> - A skipped entry never blocks later entries. There is no reservation and no pre-claim.
>
> **Per queue, in this order**
> 1. Survey: call `take_where` with a predicate that always returns `false`. For every metadata `m` it sees, if `filter(m)` is true and `resolver(m.resource_requirements())` is `Some(key)`, record `key` once, in first-seen order. Nothing is claimed.
> 2. For each recorded key, in that order, unless the key is marked busy, call `registry.request_init(key, 0, now)`:
>    - `Ready(lease)`:
>      - If `handle().has_capacity()` is false, mark the key busy for this iteration, drop the lease and continue with the next key.
>      - Otherwise call `take_where(|m| filter(m) && resolver(m.resource_requirements()) == Some(key))`. On `Some(claim)`, call `handle.execute(claim, lease)`, increment `claimed` and repeat step 2 for the same key. On `None`, drop the lease.
>      - Mark the key free-observed when capacity was true.
>    - `Started(ticket)`: call `init(ticket)` and increment `inits_started`. The entries stay Pending. This is the only place where lazy initialization is triggered.
>    - `Loading`, `Backoff{..}`, `Unavailable`: skip the key.
> - A claim happens only after a lease of a Ready resource with capacity was obtained. It never happens earlier.

**§4 Concurrency**, old:
> - `run_iteration` takes `&self` and holds no lock across calls. Several lanes (several loops) can run on the same queues. Exactly one claimer wins per node (via `claim_entry`). The others skip that node.

New:
> - `run_iteration` takes `&self` and holds no lock across calls. Several workers can run it on the same queues at the same time. Exactly one claimer wins per entry (`take_where`). A worker that loses an entry gets `None` and drops its lease.

**§4 Errors**, old:
> - There are no error values and no panics of its own. Claim and init problems only lead to skipping, with `debug!` logs. Panics from `QueueItem::state()` propagate.

New:
> - There are no error values and no panics of its own. Claim and init problems only lead to skipping, with `debug!` logs. Panics of `filter` or `resolver` propagate; the queue stays consistent (build order 4).

**§5**: the two bullets `queue_iter` and `queue_state` are replaced by:
> - `queue`: `Queue<M,P,R,E>::take_where(FnMut(&M) -> bool) -> Option<Claim>` claims the oldest Pending entry accepted by the predicate; the predicate may run on entries another consumer claims at the same time. `Claim::{meta, take_payload, complete}`; dropping a `Claim` without `complete` makes the entry `Abandoned`.

## 117 Lane und Batching

The lane resource starts no threads. A batch is formed and run on the worker-pool thread that calls `execute`. CPU batches finish there. GPU batches are launched there, and a free worker finishes them after the completion notification (stage `GpuResults` of build order 131).

**§1**, old:
> - It owns N lanes. Each lane has a worker thread and exclusive use of one `device::Lane`.

New:
> - It owns N lanes. Each lane is exclusive use of one `device::Lane`. It starts no threads: work runs on the worker-pool thread that calls `execute`.

**§1**, old:
> - It runs the batch through a caller-supplied `BatchRunner`, then reports the result with one `notify_result()`.

New:
> - It runs the batch through a caller-supplied `BatchRunner`. A CPU batch finishes before `execute` returns. A GPU batch is launched and handed to `GpuCompletion`; `on_finished` finishes it later. Each finished batch is reported with one `notify_result()`.

**§1**, old:
> - Writing task results. The runner does that (`set_output`, `advance`, `complete`).

New:
> - Writing task results. The runner does that through the step completion (`StepPayload::finish` or `fail`, build order 66a).

**§1 Restrictions**, old:
> Restrictions: no `unsafe`; `std` threads, `log` and the dependencies named in §5 only; `execute` and `has_capacity` never block.

New:
> Restrictions: no `unsafe`; no threads; `log` and the dependencies named in §5 only; `has_capacity` never blocks; `execute` blocks only while a CPU batch runs or while it waits for a lane lock.

**§2**, old:
> - The loop claims a node with `claim_entry` and calls `handle.execute(payload, lease)`. `execute` is non-blocking and infallible.

New:
> - The loop claims an entry with `take_where` and calls `handle.execute(claim, lease)` on the worker thread. `execute` is infallible.

**§2**, old:
> - The seed operation was already claimed by the loop. This module only claims additional entries, directly with `claim_entry` on queue items.

New:
> - The seed operation was already claimed by the loop. This module only claims additional entries, with `take_where` on the own and the shared queue.

**§2**, old:
> - The payload cannot give the module its queue position. "Forward through the list" is therefore implemented as an oldest-first scan of the active entries of the own queue, then the shared queue. The seed is already `Claimed`, so the scan skips it.

New:
> - The payload cannot give the module its queue position. "Forward through the list" is therefore implemented as repeated `take_where` (oldest first) on the own queue, then on the shared queue. The seed is no longer Pending, so it is never returned again.
> - `BatchMeta` is part of the queue metadata. Under staged release (build order 126) a step is pushed only after its predecessor finished, so the pusher fills the batch key and the row lengths from the payload before the push.

**§3**, old:
> use queue_iter::QueueNode;

New:
> use queue::{Queue, Claim};
> use gpu_completion::{GpuCompletion, FinishedLane};
> use device::EventError;

**§3**, old:
> /// Executes a batch on the lane (incl. stream use). Infallible from this module's view:
> /// the runner must complete or fail every task itself.
> pub trait BatchRunner<P>: Send + Sync + 'static {
>     fn run(&self, lane: &mut Lane, batch: ExecBatch<P>);
> }

New:
> /// Result of `BatchRunner::run`.
> #[derive(Debug, Clone, Copy, PartialEq, Eq)]
> pub enum Launch { Finished, Pending }
>
> /// Executes a batch on the lane (incl. stream use). Infallible from this module's view:
> /// the runner finishes or fails every task itself (`StepPayload::finish`/`fail`).
> pub trait BatchRunner<P>: Send + Sync + 'static {
>     /// CPU: runs synchronously, finishes every task, returns `Finished`.
>     /// GPU: launches the work on the lane's stream and returns `Pending`.
>     fn run(&self, lane: &mut Lane, batch: &mut ExecBatch<P>) -> Launch;
>     /// GPU only, after the lane's event completed: reads the results and finishes
>     /// or fails every task (`result` is the event outcome).
>     fn collect(&self, lane: &mut Lane, batch: ExecBatch<P>, result: Result<(), EventError>);
> }

**§3**, old:
> pub struct LaneResourceConfig<N: QueueNode> {
>     pub own_head: Arc<N>,
>     pub shared_head: Arc<N>,
>     pub filter: PullFilter<N::Meta>,

New:
> pub struct LaneResourceConfig<M, P> {
>     pub own: Arc<Queue<M, P, (), ()>>,
>     pub shared: Option<Arc<Queue<M, P, (), ()>>>,
>     pub filter: PullFilter<M>,
>     /// Some for GPU lane resources: launched batches are tracked here.
>     pub completion: Option<Arc<GpuCompletion>>,

**§3**, old:
> pub struct LaneResource<N: QueueNode> { /* private */ }
>
> impl<N> LaneResource<N>
> where N: QueueNode, N::Payload: BatchPayload, N::Meta: BatchMeta + HasResourceRequirements {
>     /// Spawns one worker thread per lane.
>     pub fn new(cfg: LaneResourceConfig<N>, lanes: Vec<Lane>,
>                runner: Arc<dyn BatchRunner<N::Payload>>) -> Result<Arc<Self>, LaneSetError>;

New:
> pub struct LaneResource<M, P> { /* private */ }
>
> impl<M, P> LaneResource<M, P>
> where P: BatchPayload, M: BatchMeta + HasResourceRequirements + Send + Sync + 'static {
>     /// Takes the lanes. Starts no threads.
>     pub fn new(cfg: LaneResourceConfig<M, P>, lanes: Vec<Lane>,
>                runner: Arc<dyn BatchRunner<P>>) -> Result<Arc<Self>, LaneSetError>;
>     /// Called by the `GpuResults` stage for a finished lane whose tag belongs to this resource.
>     /// Returns false if the tag is unknown here.
>     pub fn on_finished(&self, finished: FinishedLane) -> bool;

**§3**, old:
> impl<N> ResourceExec for LaneResource<N> where /* same bounds */ {
>     type Payload = N::Payload;
>     fn has_capacity(&self) -> bool;
>     fn execute(&self, payload: N::Payload, lease: Lease<Self>);
> }
> impl<N> LoadReporting for LaneResource<N> where /* same bounds */ { fn report(&self) -> LoadReport; }
> // LaneResource<N>: Send + Sync. Dropping it lets workers finish pending jobs, then stops and joins them.

New:
> impl<M, P> ResourceExec for LaneResource<M, P> where /* same bounds */ {
>     type Entry = Claim<M, P, (), ()>;
>     fn has_capacity(&self) -> bool;
>     fn execute(&self, entry: Claim<M, P, (), ()>, lease: Lease<Self>);
> }
> impl<M, P> LoadReporting for LaneResource<M, P> where /* same bounds */ { fn report(&self) -> LoadReport; }
> // LaneResource<M, P>: Send + Sync. Dropping it drops batches still in flight only after `GpuCompletion` returned their lanes.

**§4 Construction**, old:
> - A thread that cannot be spawned gives `Err(Spawn)`.

New:
> - `Spawn` is not used (no threads).

**§4 Capacity and lane choice**: the whole block is replaced by:
> **Capacity and lane choice**
> - `has_capacity()` is `idle_lanes() > 0`. It is non-blocking. A lane is idle when it runs no batch and has no batch in flight.
> - `execute` takes, among idle lanes, the one with the highest `LoadMeter::throughput()` (ties: lowest index). If no lane is idle (a race after `has_capacity`), it waits for the lock of the lane with the highest throughput. The claim is never dropped or rejected.
> - The lane choice is re-evaluated on every `execute`, so faster lanes get more work.

**§4 Batch formation**, old:
> **Batch formation** (on the lane worker, per job)

New:
> **Batch formation** (inside `execute`, on the calling worker thread)

**§4 Batch formation**, old:
> 1. The batch starts with the seed payload. The seed is always accepted, even if it alone exceeds the budget.

New:
> 1. The batch starts with the seed payload, taken from the seed claim with `take_payload` (`None`: drop the claim and the lease, nothing runs). The seed is always accepted, even if it alone exceeds the budget.

**§4 Batch formation**: item 3 (from "3. Scan `iter_active(own_head)`" to the end of its sub-bullets) is replaced by:
> 3. For the own queue, then the shared queue: repeat `take_where(|m| filter(m) && m.batch_key() == Some(key) && fits(m))`, where `fits` reads the current batch state only. After each `Some(claim)`, take the payload (`None`: drop the claim and continue), append it, and update the batch state from `claim.meta()`. Stop on this queue when `take_where` returns `None`. A rejected entry never ends the scan.

**§4 Batch formation**, old:
> 6. Never mix keys. Never claim an entry that was not accepted by `filter` and `key`.

New:
> 6. Never mix keys. Never claim an entry that was not accepted by `filter` and `key`. Keep the claims of all tasks of the batch until the batch is finished.

**§4 Execution and notification**: the whole block is replaced by:
> **Execution and notification**
> - `execute` calls `runner.run(&mut lane, &mut batch)` with exclusive `&mut Lane`. A panic in `run` or `collect` is caught (`catch_unwind`) and logged with `error!`; the batch's claims are dropped (`Abandoned`) and its lease is released.
> - `Launch::Finished` (CPU): in this order: `lane_meter.record(entries, elapsed)` (entries = total rows, or the number of tasks if total rows is 0; skipped if `elapsed` is zero); complete every claim with `Ok(())` in task order; drop the lease; mark the lane idle; `notifier.notify_result()` once.
> - `Launch::Pending` (GPU): record the lane's event, store `{batch, claims, lease, start}` under a new tag, hand lane and event to `completion.submit(lane, event, tag)` and return. The lane counts as busy.
> - `on_finished(finished)`: take the record of `finished.tag`, call `runner.collect(&mut finished.lane, batch, finished.result)`, then the same order as for `Finished` (meter, claims, lease, lane back and idle, `notify_result`).
> - One lease covers the whole batch, because joined tasks use the same resource. It is held until the batch is finished, so device data stays alive while the device reads it.

**§4 Concurrency**, old:
> - Exactly one claimer wins per queue entry (`claim_entry`). A concurrent loss in the scan is a silent skip.

New:
> - Exactly one claimer wins per queue entry (`take_where`). A concurrent loss in the scan is a silent skip.

**§4 Concurrency**, old:
> - On drop, pending jobs are still run, and then the workers are joined.

New:
> - There are no pending jobs and no own threads.

**§5**: the bullets `queue_iter` and `queue_state` with their sub-bullets are replaced by:
> - `queue`: `Queue<M,P,(),()>::take_where`, `Claim::{meta, take_payload, complete}`; dropping a `Claim` makes the entry `Abandoned`.
> - `gpu_completion`: `GpuCompletion::submit(lane, event, tag) -> CompletionCallback`; `FinishedLane { lane, event, tag, result, via }`.

## 126 Planner

**§1 Restrictions**, old:
> - It never claims, completes or cancels tasks.

New:
> - It claims the task exactly once, in `expand`, to read its data (`Task::claim`). It never completes or cancels tasks.

**§2**, old:
> - **Processor** (`processor`): calls `Planner::expand(&Task)` once per task and pushes each returned `PlannedOp{kind: QueueKind, meta, values}` into the queue of its kind, preserving order.

New:
> - **Processor** (`processor`): calls `Planner::expand(&Task)` once per task and pushes each returned `PlannedOp{kind: QueueKind, meta, values}` into the queue of its kind, preserving order. The Planner returns one `PlannedOp` per chain (staged release): the first step. The other steps travel inside its payload and are pushed by the step completion after their predecessor finished.

**§3**, old:
> /// Reads task kind and parameters from a task (the Planner cannot read task data).
> pub trait TaskResolver<D, T, E>: Send + Sync {
>     fn resolve(&self, task: &Task<D, T, E>) -> (TaskId, TaskSpec);
> }
>
> /// Turns the planned chain into queue payloads (e.g. Operation<R>/OperationNode wiring
> /// of input/output between neighbours). Must return exactly one V per step, in order.
> pub trait ValueBuilder<D, T, E, V>: Send + Sync {
>     fn build(&self, task: &Task<D, T, E>, steps: &[PlanStep]) -> Vec<V>;
> }

New:
> /// Reads task kind and parameters from the claimed task data.
> pub trait TaskResolver<D, T, E>: Send + Sync {
>     fn resolve(&self, task: &Task<D, T, E>, data: &D) -> (TaskId, TaskSpec);
> }
>
> /// Turns the planned chain into the payload of its first step. The payload carries the
> /// other steps (node and OpMeta each), see "Chain wiring" in section 4.
> pub trait ValueBuilder<D, T, E, V>: Send + Sync {
>     /// Also returns the batch information of the first step's payload (`None`: not batchable).
>     fn build(&self, task: &Task<D, T, E>, data: D, spec: &TaskSpec, steps: &[PlanStep]) -> (V, Option<BatchInfo>);
> }

**§3**, old:
> /// Queue metadata of an operation (the `O` of the Processor). Immutable.
> #[derive(Clone, Debug)]
> pub struct OpMeta {
>     pub task_id: TaskId,
>     pub priority: i32,                    // task.params().priority
>     pub queue: QueueKind,
>     pub kind: OperationKind,
>     pub descriptor: OperationDescriptor,
>     pub resource: ResourceRequirements,
>     pub required: Requirements,           // == resource.capabilities().clone()
>     pub materialize: bool,
> }

New:
> /// Queue metadata of an operation (the `O` of the Processor). Immutable after the push.
> #[derive(Clone, Debug)]
> pub struct OpMeta {
>     pub task_id: TaskId,
>     pub priority: i32,                    // task.params().priority
>     pub queue: QueueKind,
>     pub kind: OperationKind,
>     pub descriptor: OperationDescriptor,
>     pub resource: ResourceRequirements,
>     pub required: Requirements,           // == resource.capabilities().clone()
>     pub materialize: bool,
>     pub batch: Option<BatchInfo>,         // None when planned; set by the pusher from the payload
> }
> /// Batch key and untruncated row lengths of the payload (build order 117).
> #[derive(Clone, Debug, PartialEq, Eq)]
> pub struct BatchInfo { pub key: lane_batching::BatchKey, pub seq_lengths: Vec<usize> }
> impl resource_requirements::HasResourceRequirements for OpMeta { /* &self.resource */ }
> impl operation_node::HasOperationKind for OpMeta { /* self.kind */ }
> impl lane_batching::BatchMeta for OpMeta { /* from self.batch; None / &[] when unset */ }

**§3**, old:
>     /// ValueBuilder returned a different number of values than steps
>     BuilderMismatch { expected: usize, got: usize },

New:
>     /// "Task nicht beanspruchbar: {0:?}" (cancelled or already claimed)
>     TaskNotClaimable(ClaimError),

**§3, insert after** "    pub fn first_resource_kind_of_task(&self, task: &Task<D, T, E>) -> Option<ResourceKind>;"

New:
>     /// Placement of one step created at run time (stepwise mode). Queue assignment and
>     /// materialization as in `expand` (`next` = the following step, if known); announces the
>     /// resource. Never blocks and never activates (lazy initialization, build order 106).
>     pub fn place_step(&self, task_id: TaskId, priority: i32, op: &PlannedOperation,
>                       next: Option<&PlannedOperation>) -> Result<OpMeta, PlanError>;

**§4 expand**, old:
> 2. `resolver.resolve(task)` gives `(task_id, spec)`. `TaskSpec::Other{kind}` → `Expand(ForeignTaskKind{expected:"Embedding/Similarity", got:kind})`.

New:
> 2. `task.claim()` gives the data `d`. An error gives `TaskNotClaimable(e)`. `resolver.resolve(task, &d)` gives `(task_id, spec)`. `TaskSpec::Other{kind}` → `Expand(ForeignTaskKind{expected:"Embedding/Similarity", got:kind})`. Every error from here on leaves the task `Claimed`; the caller completes it (build order 135).

**§4 expand**, old:
> 8. `ValueBuilder::build(task, steps)` runs. A wrong number of values gives `BuilderMismatch`.
> 9. The result is one `PlannedOp{kind: queue, meta, values}` per step, in chain order. `meta.priority` equals `task.params().priority`. `meta.descriptor` and `meta.resource` are the `PlannedOperation` values unchanged. `meta.required == resource.capabilities()`.

New:
> 8. `ValueBuilder::build(task, d, &spec, steps)` runs and returns the payload of the first step and its batch information.
> 9. The result is exactly one `PlannedOp{kind: steps[0].meta.queue, meta, values}` with `meta = steps[0].meta` and `meta.batch` set to the batch information returned by the `ValueBuilder` (the Planner cannot read `V`). For every step `meta.priority` equals `task.params().priority`, `meta.descriptor` and `meta.resource` are the `PlannedOperation` values unchanged, and `meta.required == resource.capabilities()`.

**§4, insert after** the block **Materialization** (after its last bullet "- It is decided from the Expander's environments before queue assignment, and queue choice does not change it.")

New block:
> **Chain wiring (contract of the reference `ValueBuilder`)**
> - It creates `TaskContext::new(task_id, priority, 1)` once per chain.
> - It builds the nodes in chain order: `OperationNode::new(task_id, steps[k].planned.clone(), input_k, steps[k].meta.materialize, task.clone())`. `input_0` is prepared from `d` as in build order 78; `input_{k+1} = ChainInput::Upstream(nodes[k].output_handle())`, taken before the node is moved.
> - It returns `StepPayload::new(nodes[0], context, rest)` with `rest[k-1] = PlannedStep { placement: steps[k].meta, node: nodes[k] }` for k >= 1. `StepPayload::new` sets `ends_chain` to `false` for every node that has a rest; the last node keeps `true`.
> - It never reads or writes queues.

## 130 Backpressure und Expansionssteuerung

Idle notifications are a prioritisation hint for the expansion. Nothing is withheld, delayed or refused: there are three operations queues, so no single "queue empty" condition exists.

**§1**, old:
> - It decides whether a task may be expanded now, and which waiting task is chosen first. A task may be expanded only if there is an Idle-Notification or the operations queue is empty. A task whose first operation needs an underserved resource is preferred.

New:
> - It decides which waiting task is expanded first: a task whose first operation needs an underserved resource is preferred. It never withholds, delays or refuses an expansion.

**§2**, old:
> - **Queue-Iteration** is not called directly. The caller derives "operations queue empty" from it, e.g. `iter_active(head).next().is_none()` on the operations queues, and passes it in as `bool`.

New:
> - **Queues** are not used.

**§2**, old:
> - **Caller** (the engine or processor feed side): it calls `report_iteration` after each `run_iteration`, `observe_load` for load reports, and `select` before each expansion. The caller then expands the chosen task through the Planner.

New:
> - **Caller** (the Engine-Fassade): its feed steps call `report_iteration` after each `run_iteration`; it calls `observe_load` for load reports. The expansion stage of the Worker-Loop (build order 131) asks `underserved()` and prefers a waiting task whose first resource kind is in that set; otherwise it takes the oldest task.

**§3**, old:
>     /// true iff operations_queue_empty || has_idle().
>     pub fn may_expand(&self, operations_queue_empty: bool) -> bool;
>
>     /// Chooses the index in `candidates` to expand next, or None if expansion is not allowed
>     /// (or candidates is empty). Does not change any state.
>     pub fn select<T>(&self, operations_queue_empty: bool, candidates: &[Candidate<T>]) -> Option<usize>;

New:
>     /// Chooses the index in `candidates` to expand next; None only if `candidates` is empty.
>     /// Does not change any state and never withholds an expansion.
>     pub fn select<T>(&self, candidates: &[Candidate<T>]) -> Option<usize>;

**§4 Expansion permission**: the whole block is replaced by:
> **Selection**
> - `select` returns `None` only if `candidates` is empty.
> - Otherwise it returns the index of the first candidate whose `first_kind` is `Some(k)` with `k` in `underserved()`. If there is none, it returns index `0`, the oldest candidate; this also applies to candidates with `first_kind == None`.
> - Candidate order is the caller's fairness order, and `select` keeps it among equals.
> - `select` is pure and repeatable for an unchanged state.

**§5**, old:
> - Queue-Iteration (used by the caller): `iter_active(head)` is non-blocking, and an empty iteration means no active entry.

New:
> - Queues: not used.

## 131 Worker-Loop

The four stages stay. The loop runs on the worker-pool threads.

**§1 Restrictions**, old:
> - Waiting on notifications, the async loop, and timers. The caller does that and calls `run_until_idle` after each wake-up.

New:
> - Waiting on notifications and timers. The caller is `WorkSource::run_once` of the Engine-Fassade on a worker-pool thread; it calls `run_once` and maps `Some` to `Step::Worked` and `None` to `Step::Idle`. Parking and waking is done by the worker pool (build order 56).

**§2**, old:
> - The caller (the worker thread) waits on `SubengineNotifier`, calls `run_until_idle`, then calls `notify_iteration(progress)` using the returned `WorkerReport`.

New:
> - The caller is the worker-pool thread (see §1). CPU work runs synchronously inside a feed call. GPU work is only launched inside a feed call; after the completion notification any free worker finishes it in `GpuResults`, where `on_finished` passes each `FinishedLane` to the lane resource that owns its tag (`LaneResource::on_finished`, build order 117).

**§3**, old:
> use queue_iter::QueueNode;

New:
> use queue::Claim;

**§3**, old:
> impl<N, H> FeedStep for SubengineLoop<N, H>
> where N: QueueNode<Payload = H::Payload>, N::Meta: HasResourceRequirements, H: ResourceExec
> { /* calls run_iteration(now) */ }

New:
> impl<M, P, H> FeedStep for std::sync::Arc<SubengineLoop<M, P, H>>
> where M: HasResourceRequirements + Send + Sync + 'static, P: Send + 'static,
>       H: ResourceExec<Entry = Claim<M, P, (), ()>>
> { /* calls run_iteration(now) */ }

**§3**, old:
> /// Expansion of at most one new task per call (takes it from the task source,
> /// calls `Planner::expand`, hands the PlannedOps to the Processor, and handles
> /// an expansion error itself, e.g. by failing the task).

New:
> /// Expansion of at most one new task per call (takes it from the task source, preferring a
> /// task for an underserved resource, calls `Processor::expand`, and handles an expansion
> /// error itself). Implemented by the Engine-Fassade.

## 134 Subengine

The sub-engine role (resources and memory per hardware kind, pull principle) is realised by the CPU-Subengine (build order 137) and the GPU-Subengine (build order 139). This module is the facade-side handle of one of them. It starts no threads and owns no memory.

**§1**: the whole section text from "**Task.** Implement the Rust module `subengine`." to the end of the restrictions is replaced by:
> **Task.** Implement the Rust module `subengine`. A `SubengineHandle` wraps one concrete sub-engine (CPU or GPU) for the Engine-Fassade. It provides:
> - the sub-engine's `planner::Subengine` implementation (forwarded unchanged),
> - its feed steps: one `SubengineLoop` (build order 106) per queue role over the sub-engine's own queue and the shared queue, as `FeedStep`s for the `WorkerLoop` (build order 131),
> - its device kind (GPU feeds go to the GPU stage, CPU feeds to the CPU stage),
> - `LaneResource::on_finished` routing for GPU tags.
>
> **Restrictions.** No threads, no memory, no queues of its own, no claiming. Resources, memory, streams and kernels belong to the concrete sub-engine. `std`, `log` only. No `unsafe`.

**§3**: the whole interface block is replaced by:
```rust
use std::sync::Arc;
use device::DeviceKind;
use gpu_completion::FinishedLane;
use planner::Subengine;
use worker_loop::FeedStep;

pub struct SubengineHandle { /* private */ }   // Send + Sync

impl SubengineHandle {
    /// `feeds` builds a fresh set of feed steps for one worker; called once per worker.
    /// `on_finished` returns false if the tag does not belong to this sub-engine.
    pub fn new(subengine: Arc<dyn Subengine>, device: DeviceKind,
               feeds: Arc<dyn Fn() -> Vec<Box<dyn FeedStep>> + Send + Sync>,
               on_finished: Arc<dyn Fn(FinishedLane) -> Result<(), FinishedLane> + Send + Sync>) -> Self;
    pub fn subengine(&self) -> &Arc<dyn Subengine>;
    pub fn device(&self) -> DeviceKind;
    pub fn feeds_for_worker(&self) -> Vec<Box<dyn FeedStep>>;
    /// Ok(()) if the lane was handled, Err(lane) if it belongs to another sub-engine.
    pub fn on_finished(&self, lane: FinishedLane) -> Result<(), FinishedLane>;
}
```

**§4**: the whole section is replaced by:
> - `feeds_for_worker` is called once per worker-pool thread by the Engine-Fassade at start. The returned feeds share the sub-engine's `SubengineLoop`s (`Arc`), so all workers pull from the same queues.
> - `on_finished` forwards to the sub-engine's lane resources and returns the lane unchanged if no resource owns its tag.
> - All methods take `&self` and are callable from any thread.

**§5**: the whole section is replaced by:
> - `planner::Subengine` (unchanged trait), `worker_loop::FeedStep`, `gpu_completion::FinishedLane`, `device::DeviceKind`.

## 135 Engine-Fassade

**§2**, old:
> - **Worker-Pool** (`worker_pool::WorkerPool`) runs threads that call `WorkSource::run_once`. The facade supplies the `WorkSource` (the task dispatcher). The worker count is only the number of dispatcher threads. Sub-engines pull operations with their own lanes.

New:
> - **Worker-Pool** (`worker_pool::WorkerPool`) runs the engine's only worker threads, which call `WorkSource::run_once`. The facade's `WorkSource` holds one `WorkerLoop` (build order 131) per worker: GPU results, GPU feeds, CPU feeds and task expansion.

**§2**, old:
> - **Subengine** (`planner::Subengine`, e.g. `ComputeSubengine`) pulls operations from the Processor's ops queues. Dropping the last `Arc` of a `ComputeSubengine` shuts it down.

New:
> - **Subengine** (`subengine::SubengineHandle`, build order 134) wraps a concrete CPU or GPU sub-engine (build orders 137, 139). Its feeds pull operations from the Processor's ops queues inside the workers' `WorkerLoop`.

**§2, insert after** "- **Queue** and **Queue state** are used for the task queue: `Queue<TaskParams, Task<D,T,E>, (), ()>`."

New bullets:
> - **Step completion** (`step_completion`, build order 66a): the reference payload is `EngineStep { step: StepPayload<OpMeta, PlannedOperation, ChainInput, StepOutput, D, E>, sink: Arc<dyn FollowUpSink<OpMeta, StepPayload<OpMeta, PlannedOperation, ChainInput, StepOutput, D, E>>> }`, and the operations queues are `Queue<OpMeta, EngineStep<D, E>, (), ()>`. The Engine implements the sink: `submit(Some(meta), payload)` wraps the payload into an `EngineStep` with the same sink, sets `meta.batch` from it, pushes into `ops_queue(meta.queue)` and calls `NotificationTopology::notify_push()`; `submit(None, payload)` first calls `Planner::place_step` for the node's operation.
> - **Expansion control** (`backpressure`, build order 130): the feed steps report every iteration with `report_iteration`; the expansion stage prefers tasks for underserved resources. Nothing is withheld.
> - **Notification** (`notification_topology`, build order 92): created with the worker pool's `Notifier::notify` as wake function.

**§3**, old:
>     /// Required. Passed to `PlannerBuilder::new`.
>     pub fn value_builder(self, v: Arc<dyn ValueBuilder<D, T, E, V>>) -> Self;

New:
>     /// Required. Passed to `PlannerBuilder::new`.
>     pub fn value_builder(self, v: Arc<dyn ValueBuilder<D, T, E, V>>) -> Self;
>     /// Required. Maps an expansion error to the task error for tasks the Planner already claimed.
>     pub fn plan_error(self, f: Arc<dyn Fn(&PlanError) -> E + Send + Sync>) -> Self;

**§4 Builder and start**, old:
>   3. `"value_builder"`.

New:
>   3. `"value_builder"`,
>   4. `"plan_error"`.

**§4 Builder and start**, old:
>   5. `worker_pool::WorkerPool::start(cfg, dispatcher)`. A failure becomes `Pool(text)`.

New:
>   5. Build one `WorkerLoop` per worker (feeds from every `SubengineHandle::feeds_for_worker`, the shared `GpuCompletion`, `on_finished` routing over the handles, and the expansion step) and `worker_pool::WorkerPool::start(cfg, work_source)`. A failure becomes `Pool(text)`.

**§4 Task dispatcher (`WorkSource`, internal, observable)**: the whole block is replaced by:
> **Work source and expansion stage (internal, observable)**
> - `run_once(worker)` locks that worker's `WorkerLoop` and calls `run_once(Instant::now())`. `Some(_)` gives `Step::Worked`, `None` gives `Step::Idle`.
> - The expansion stage (`ExpandStep::expand_next`) does, in this order:
>   1. If `closed` is set, return `false`.
>   2. Take a task from the task queue: if `ExpansionControl::underserved()` is not empty and the task-queue metadata carries the first resource kind, first try `take_where` for a task whose first kind is underserved; otherwise, or if that gives `None`, `take()`. `None` returns `false`.
>   3. Take the payload (the `Task` handle). If the task is already `Completed` or `Cancelled`, skip expansion. Otherwise call `Processor::expand(&task)` once, drop the tickets and call `NotificationTopology::notify_push()`.
>   4. If `expand` fails (`ExpandError::Planner(e)`), log `warn!`. If the task is `Claimed`, call `task.complete(Err(plan_error(&e)))`; otherwise call `task.cancel()`.
>   5. Always `claim.complete(Ok(()))`, then return `true`.
> - The facade never claims a `Task`. The Planner claims it inside `expand`; the operation node that ends the chain completes it. The facade completes it only after an expansion error (step 4).
> - A task that the Planner claimed cannot be cancelled with `Task::cancel` (it returns `false`); its chain runs to the end.
> - `expand` may block in `Subengine::activate`. This is backpressure and is not interrupted.

## 137 CPU-Subengine

**§2**, old:
> - **Subengine-Loop / Lane-Batching.** `LaneResource` (one worker thread per `Lane`) executes claimed payloads. `CpuSubengine` supplies the lanes (topology) and the `BatchRunner` (`cpu_runner`). Inside the runner every operation runs synchronously on the lane's worker thread. It returns its result, to the task and to index callers, before the runner returns.

New:
> - **Subengine-Loop / Lane-Batching.** `LaneResource` executes claimed payloads on the worker-pool thread that claimed them. `CpuSubengine` supplies the lanes (topology) and the `BatchRunner` (`cpu_runner`). Inside the runner every operation runs synchronously on that worker thread, and `run` returns `Launch::Finished`. It returns its result, to the task and to index callers, before the runner returns.

**§3**, old:
> use queue_iter::QueueNode;

New:
> use queue::Queue;

**§3**, old:
>     pub fn lane_resource<N>(self: &Arc<Self>, cfg: LaneResourceConfig<N>,
>                             runner: Arc<dyn BatchRunner<N::Payload>>) -> Result<Arc<LaneResource<N>>, CpuError>
>     where N: QueueNode, N::Payload: BatchPayload, N::Meta: lane_batching::BatchMeta + resource_requirements::HasResourceRequirements;

New:
>     pub fn lane_resource<M, P>(self: &Arc<Self>, cfg: LaneResourceConfig<M, P>,
>                                runner: Arc<dyn BatchRunner<P>>) -> Result<Arc<LaneResource<M, P>>, CpuError>
>     where P: BatchPayload, M: lane_batching::BatchMeta + resource_requirements::HasResourceRequirements + Send + Sync + 'static;

**§4.8**, old:
> - It adds no scheduling, batching or locking. The context is cheap to create and holds no state.

New:
> - It adds no scheduling, batching or locking. The context is cheap to create and holds no state.
> - For the reference payload (`EngineStep`, build order 135) `execute_batch` runs each task's operation and then calls `step.finish(output, &*sink)`; on an execution error, or when `finish` returns an error, it calls `step.fail(error, || 0)`. One task's error never affects the other tasks of the batch. `collect` is never called for CPU lanes.

## 139 GPU-Subengine

**§1 Restrictions**, old:
> - The module starts an operation on a free lane and **returns without waiting for the result**. A worker processes the result after the completion notification (`LaneResource` and `notify_result`). The module adds no polling and no blocking waits.

New:
> - The module starts an operation on a free lane and **returns without waiting for the result**: its `BatchRunner::run` launches the batch and returns `Launch::Pending`; the `LaneResource` hands lane and event to `GpuCompletion`. After the completion notification a free worker processes the result (stage `GpuResults` of build order 131, `LaneResource::on_finished`, `BatchRunner::collect`), which finishes or fails every task (`StepPayload::finish`/`fail`). The lease is held until then, so weights stay alive while the device reads them. The module adds no polling and no blocking waits.

**§3**, old:
> use queue_iter::QueueNode;

New:
> use queue::Queue;

**§3**, old:
>     pub fn build_lane_resource<N>(&self, cfg: LaneResourceConfig<N>, runner: Arc<dyn BatchRunner<N::Payload>>)
>         -> Result<Arc<LaneResource<N>>, GpuError>
>     where N: QueueNode, N::Payload: BatchPayload, N::Meta: BatchMeta + HasResourceRequirements;

New:
>     pub fn build_lane_resource<M, P>(&self, cfg: LaneResourceConfig<M, P>, runner: Arc<dyn BatchRunner<P>>)
>         -> Result<Arc<LaneResource<M, P>>, GpuError>
>     where P: BatchPayload, M: BatchMeta + HasResourceRequirements + Send + Sync + 'static;

**§4.5 Lane resource**, old:
>   - creates `worker_lanes` lanes of kind `Query` with `LaneConfig::preset(Query, device)`. Each has its own stream from the backend and `lane_workspace_bytes` workspace;

New:
>   - creates `worker_lanes` lanes of kind `Query` with `LaneConfig::preset(Query, device)`. Each has its own stream from the backend and `lane_workspace_bytes` workspace. No threads are started; `cfg.completion` must be `Some` (else `InvalidConfig`);
