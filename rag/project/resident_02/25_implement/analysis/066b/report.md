# Operation node: cross-spec analysis (build order 66b) - report

## 0. Conventions, fact re-verification and new findings

### 0.1 Path aliases used in citations

| Alias | Path relative to repo root |
|---|---|
| `SPEC/` | `project/resident_02/23_batch/specs/` |
| `L110/` | `src/resident/l130/l120/l110/` |
| `L050/` | `src/resident/l130/l120/l110/l100/l090/l080/l070/l060/l050/` |
| `L060/` | `src/resident/l130/l120/l110/l100/l090/l080/l070/l060/` |
| `L020/` | `L050/l040/l030/l020/` |

Marks: **V** = VERIFIED (read at the cited place), **I** = INFERRED (derivation stated), **U** = UNKNOWN.
Test baseline: `runTests` on the unchanged tree: 813 + 8 tests pass (V, run on 2026-11-15).

### 0.2 Re-verification of F1 to F12

| ID | Result | Evidence |
|---|---|---|
| F1 | Confirmed. | `L020/l010/queue.rs#Queue::take_where` (scan from head, `item.state()==PENDING && pred && try_claim`), `#Drop for Claim` (`finish(ABANDONED, None)`), `#Queue::push` (bumps `channels.available`). V |
| F2 | Confirmed, with two corrections. (a) The list queue is the module `queue_list` in code (`L020/b/queue_list.rs`), while `SPEC/025` and `SPEC/092` call it module `queue`; the take queue is the module `queue` (`L020/l010/queue.rs`). Both specs therefore use the same module name for different types. (b) `queue_list` does not use `queue_reclaim` (`SPEC/024`); it has its own `ArcSwapOption` chain and **no removal operation** (`queue_list.rs#impl Queue`: only `new`, `enqueue`, `head`, `first`, `set_tail_hint_update`). | V |
| F3 | Confirmed. The Processor holds its four queues by value (`L110/a/processor.rs#Queues`), so the `Arc` gap of `SPEC/135#5` is real. | V |
| F4 | Confirmed. `Task` wraps `Arc<EntryState<D,T,E>>` (`L020/b/task_model.rs#TaskInner`). It is used as the *payload* of the take-family task queue, so the two families already coexist in the implemented Processor. | V |
| F5 | Confirmed and extended: list family also in `SPEC/023`, `SPEC/003`, `SPEC/130#2` (`iter_active` for "queue empty"), `SPEC/131#3` (`QueueNode` bound). `SPEC/132#2` names `take_where` but uses no queue API itself. | V |
| F6 | Confirmed. Additionally `SPEC/117#3 BatchRunner` ("the runner must complete or fail every task itself") and `SPEC/137#3 CpuPayload` ("Must complete/fail every task of the batch"). | V |
| F7 | Confirmed for all specs, including the formerly UNKNOWN ones: `SPEC/092`, `SPEC/107`, `SPEC/025`, `SPEC/137`, `SPEC/139` contain no ordering rule between steps of one task. | V |
| F8 | Confirmed. `SPEC/126#4 Materialization`, `SPEC/078#4 Start`. | V |
| F9 | Confirmed. In addition `SPEC/078` is self-contradictory: intermediate steps get `materialize == false`, yet the handler builds the next input from `output: RamData` (`SPEC/078#4 Continuation handler`). | V |
| F10 | Confirmed. Additionally `Task::cancel` succeeds only from `Enqueued` (`L020/l010/queue_state.rs#EntryState::cancel`, CAS `ENQUEUED -> CANCELLED`). | V |
| F11 | Confirmed with precision: a second `complete` panics in debug only if the observed state is `Completed`; for `Enqueued`/`Cancelled` it returns `Err(NotClaimed(..))` without panic (`queue_state.rs#EntryState::complete`). | V |
| F12 | Confirmed. | V |

### 0.3 New findings not listed in 66a or 66b

| ID | Finding | Evidence |
|---|---|---|
| N1 | **Task data cannot be read without claiming.** `Task` offers `params`, `state`, `result`, `claim`, `complete`, `cancel`, `wait`, `notifier`; the data `D` is reachable only through `claim()`, which moves it out and sets `Claimed`. `SPEC/126` needs the data twice (`TaskResolver::resolve(&Task)` and `ValueBuilder::build(&Task, ..)`), and `SPEC/135#4` says "Claiming and completing is up to the injected components". Nobody is assigned to call `Task::claim`; without it every `advance` at the chain end returns `Complete(NotClaimed(Enqueued))`. | V `task_model.rs#impl Task`; I for the consequence |
| N2 | **Two complete sub-engine architectures.** T-stack: `SPEC/004`, `037`, `056`, `134` (`ComputeSubengine`, own lane threads, `take_where` + `wait_change`, synchronous `ComputeBackend::execute`), `135`. L-stack: `SPEC/003`, `023`, `024`, `025`, `071` (async `PullLoop`), `092`, `106` (`SubengineLoop`, lifecycle, idle notices), `107`, `117` (`LaneResource`, batching), `130`, `131` (`WorkerLoop`, own expand stage), `132` (`SubengineCore`), `137`, `139`. Both implement `planner::Subengine`; both define a type `SubengineConfig` and `SubengineError` (`SPEC/132#3`, `SPEC/134#3`). | V |
| N3 | **Immutable queue metadata vs. data-dependent batch keys.** `SPEC/117#3 BatchMeta::seq_lengths` must equal the payload's token rows and is read from metadata before claiming. For a pre-pushed Embedding step these rows are the output of the not yet executed Tokenization step. Metadata is fixed at push (`SPEC/004#3 push(meta, ..)`, `SPEC/126#3 OpMeta`: "Immutable"). | V (texts); I (conflict) |
| N4 | **Lease release vs. asynchronous GPU work.** `SPEC/117#4 Execution`: after `run` returns, the lease is dropped and the lane is marked idle. `SPEC/139#1`: the module "returns without waiting for the result"; `SPEC/139#4.4`: users keep `Arc<WeightSet>` alive until events complete. If `run` only launches, the lease ends while the GPU still reads the weights. | V (texts); I (conflict) |
| N5 | **Expansion control is not wired.** `SPEC/135#4 Task dispatcher` expands every taken task at once and never calls `ExpansionControl` (`SPEC/130`). `SPEC/131` has a second expansion driver (`ExpandStep`). `SPEC/130#2` derives "queue empty" from `iter_active`, `SPEC/056a#2` from `Queue::len()`. | V |
| N6 | **`FollowUpSink` cannot build queue entries.** 66a `FollowUpSink<N>::submit(Vec<N>)` receives bare nodes, but a queue entry needs `(QueueKind, OpMeta, V)` and `V = StepPayload` needs the `TaskContext`. | V (66a §3.2); I |
| N7 | **Task ID types differ in the withdrawal predicate.** `OpMeta.task_id: operation::TaskId` (newtype) vs. `OperationNode::task_id() -> u64`; 66a A6 compares them directly. | V `SPEC/126#3`, 66a §3.1 |
| N8 | **Dispatcher error path with a claimed task.** If the task is claimed inside `expand` (N1) and `expand` then fails (`BuilderMismatch`), `SPEC/135#4` calls `task.cancel()`, which fails from `Claimed`; the task never finishes. | I from N1 and `queue_state.rs#cancel` |
| N9 | Stale code comments: `L110/a/worker_pool.rs#WorkSource` ("Implemented by the Processor", contradicts `SPEC/056a`), `L020/l010/queue.rs#Queue` (refers to `l020::b::queue`, now `queue_list`). | V |

### 0.4 Status after the owner decisions of 2026-11-15

The owner decided D2, D3 and D8 (`decisions.md`). Where the sections below recommend something else, this section wins.

- **Q1 / D2:** the CPU and GPU sub-engines follow 137/139 (on 132, 106, 117, 131). Checked against `project/resident_02/21_modules_ordered.yml#Worker-Pool` (V): the worker-pool threads are the only workers of the engine (at most cores - 1), each serves itself from the queues, runs CPU work synchronously and only launches GPU work; a free worker processes the GPU result after the completion notification. Therefore 134 starts no threads and owns no memory, and 117 starts no lane threads.
- **Q2 / D3:** staged release, confirmed.
- **Q3:** the runner placement is (b): CPU batches finish on the worker that claimed them; GPU batches are launched there and finished in stage `GpuResults` of 131 through `LaneResource::on_finished` and `BatchRunner::collect`. The lease lives until then (N4 resolved).
- **Q5 / D8:** idle notifications only order the expansion; nothing is withheld (`21_modules_ordered.yml#Backpressure und Expansionssteuerung` says "nur bei", the owner reads it as prioritisation because there are three operations queues). Expansion stays the fourth stage of the Worker-Loop, as required by `21_modules_ordered.yml#Worker-Loop` (V); the former task dispatcher of 135 becomes that stage. Prioritisation needs the first resource kind of a waiting task before it is claimed (new decision D10).
- **Eviction:** RAM and VRAM both evict; RAM with the shorter period (V `21_modules_ordered.yml#Ressourcen-Lebenszyklus`).


---

## Q1. Queue families (66a C14). Owner decision D1, plus D2 (from N2)

### Facts
- Take family code: `L020/l010/queue.rs` (`Queue<M,P,R,E>`, `push`, `take`, `take_where`, `len`, `epoch`, `wait_change`, `Ticket`, `Claim`, RAII `Abandoned`, crossbeam-epoch reclamation via `advance_head`). V
- List family code: `L020/b/queue_list.rs` (enqueue/head/first, `tokio::sync::Notify` channels, no removal), `L020/b/queue_iter.rs` (`QueueNode`, `iter_all`, `iter_active`), `L020/l010/queue_state.rs` (`EntryState`, `claim_entry` with `try_lock` -> `Busy`, `complete`, `cancel` from `Enqueued` only), `L020/b/queue_reclaim.rs` (`Link`, `Node`, `unlink`; not used by `queue_list`). V
- Code consumers: take family -> `processor.rs` only; list family -> none outside the family (`queue_list` implements `queue_iter::QueueNode`). `task_handle`, `task_context`, `queue_reclaim`, `operation_node` have no production consumers. V (search over `src/` for the module paths)
- Spec requirements per consumer (V, read in full):

| Spec | API family | Needs |
|---|---|---|
| 004, 037, 056, 056a | take | claim by predicate, Ticket results, Abandoned, per-queue epochs, blocking wait |
| 134 | take | `take_where` per worker, `wait_change(Available, .., poll_interval)` |
| 135 | take | `take()` on the task queue, `Claim::complete`, `Arc` to ops queues |
| 025, 024, 023, 003 | list | append, iteration without claim, claim by entry, later unlink |
| 071 | list | iteration, `claim()`, `Busy` retry, one `Notify` per sub-engine |
| 092 | list | queue signals every sub-engine channel on enqueue |
| 106 | list | iteration in order, `claim_entry` after resource check |
| 117 | list | iteration + claim of batch members with a stateful fit check |
| 130 | list | "queue empty" via `iter_active` |
| 131, 137, 139 | list | `QueueNode` bound only (through 106/117) |

### Analysis
- Every list-family operation the consumers need can be expressed with the take family (I):
  - "iterate, filter, then claim" (071, 106) = `take_where(pred)`; the resource check that 106 does between filter and claim is a pure function of the metadata and fits into the predicate.
  - Batch scan (117): repeated `take_where` with a predicate that tests `batch_key == key && fits(state)`; the fit state is updated from `claim.meta()` after each successful claim, never inside the predicate (`SPEC/004#4`: the predicate may run on items another consumer wins). Cost O(n) per member.
  - "queue empty" (130) = `len() == 0` (as `SPEC/056a#2` already states).
  - Per-sub-engine wake-up (092): the take family has per-queue epochs; a sub-engine waits on its own notifier, and every producer (`Processor::submit/expand` caller, `FollowUpSink`, result paths) calls it after a push. This is the pattern `SPEC/056` already uses for the worker pool.
- What the list family lacks for the reference path (V/I): node removal (no `unlink` on `queue_list`, so the list and every scan grow with the total history), RAII release on panic (a `claim_entry` winner that panics leaves the entry `Claimed` forever; `EntryState` has no abandon transition), a blocking wait for `std` threads (only async `Notify`).
- What the take family lacks: iteration without claim (not required by any consumer once 130 uses `len`), external notification channels (replaced by producer-side notify), the `Busy` state (not needed).
- `Task`/`TaskHandle` (F4): they use `EntryState` only as the task's own state machine; that is independent of the queue that transports the handle and stays valid under every option (V: Processor already pushes `Task` as payload of a take queue).

### Options
- **A** Take family is the reference for task and ops queues; list-family specs are re-based. Batch scan = repeated `take_where`; 092 becomes producer-side notification; `Task`/`TaskHandle` unchanged.
- **B** List family is the reference; Processor, Worker-Pool, 004, 037, 056, 134, 135 re-based. Requires a removal operation for `queue_list` (new code), an abandon transition in `EntryState` (code change in an implemented module) and a blocking wait. Batch scan stays; 092 stays; `Task` unchanged.
- **C** Adapter. Take-as-list is impossible without changing `queue.rs` internals (`QueueNode::entry()` must return `&Arc<EntryState>`, the take item has no `EntryState`; V `queue.rs#Item`). List-as-take is possible (`take_where` = `iter_active` + `claim_entry`) but inherits the missing removal and abandon. 092 kept on the list side only.
- **D** Unified new API (take family plus `iter_pending` + `try_claim(&PendingRef)` + listener registration). Changes the lock-free core of an implemented module.

### Evaluation

| Criterion | A | B | C | D |
|---|---|---|---|---|
| Implemented modules changed | 0 | 3 (`processor`, `queue_list` removal, `queue_state` abandon) + tests | 1 new adapter | 1 (`queue.rs` core) |
| Unimplemented specs changed | 9 (071, 092, 106, 117, 130, 131, 137, 139, 025-note) | 5 (004-role, 037, 056, 134, 135) | 2-4 | most of both sides |
| Lost required capability | none (I) | none after new code | removal/abandon missing | none |
| Lock-freedom / reclamation | lock-free, epoch reclamation (V) | lock-free append, no reclamation today (V) | as B | as A, more risk |
| Testability | existing 813 tests cover the queue | new code needs new tests | adapter tests | new core tests |

### Recommendation
**A.** It keeps all implemented code unchanged, loses no required capability and gives RAII abandonment and memory reclamation. The list-family modules stay in the crate unchanged; they are not used on task or ops queues. No `queue.rs` change is required under the Q2 recommendation (Q2 option B needs no extra wake-up); under Q2 option A an additive `Queue::notify_available()` is recommended.

D2 (N2, sub-engine architecture), options:
- **D2-i** L-stack realisation as reference (132 core + 106 iteration + 117 lanes/batching + 131 worker stages + 137/139), re-based on take queues; 134 is reduced to the generic sub-engine host (configuration, worker threads, shutdown) that drives `SubengineLoop` iterations; `ComputeBackend` is dropped.
- **D2-ii** 134 `ComputeSubengine` is the only sub-engine; 137/139 become `ComputeBackend` implementations; 106/117/131 are dropped. Loses batching across tasks (117), lazy lifecycle and idle notices (106 -> 130).
- **D2-iii** keep both (two implementations of `planner::Subengine`). Two `SubengineConfig`/`SubengineError` types and two wake-up schemes remain.

Recommendation **D2-i**: batching (117), lifecycle (106) and idle notices (130) are modules of the fixed module tree and would be lost under D2-ii (I).

### Open owner decisions
D1 (queue family), D2 (sub-engine architecture).

---

## Q2. Readiness of planned steps (66a C13). Owner decision D3

### Facts
- No spec prevents claiming step `k+1` early (F7, now V for all specs).
- Selection places (V): `SPEC/134#4` worker predicate; `SPEC/106#4` per node; `SPEC/117#4` seed (selected by 106) and batch members; `SPEC/071#4` pass; `SPEC/135#4` dispatcher (task queue only, not affected); `SPEC/131` delegates to 106.
- Fixed decision (66b §2): the chain is created completely in one `expand`; "the Processor pushes all steps". V
- Metadata is immutable after push; `BatchMeta::seq_lengths` is data-dependent (N3). V/I
- `Queue::len()` counts every Pending item (`queue.rs#len`), and `SPEC/056a#2`/`SPEC/130` use "ops queue empty" as expansion permission. V
- A gate opening is not a push and bumps no epoch (`queue.rs#push` is the only `available.bump()`). V

### Analysis
- **Every selector must know the order.** With pre-pushed steps each of the five selection places above needs the readiness test, including every batch member; one missing test silently breaks the data flow (I).
- **Batching:** gated Embedding steps cannot carry their row lengths (N3); 117 token budgeting is impossible for them unless metadata gets interior mutability (I).
- **Load measure:** gated items keep `len() > 0` for the whole lifetime of every chain, so "ops queue empty" is false whenever any chain is in flight; expansion then depends only on idle notices (I). Scans also traverse all gated items (cost grows with tasks x chain length).
- **Wake-up:** with polling (134, 20 ms) each gated step adds up to one poll interval of latency; an Embedding chain of 5 steps adds up to 80 ms per task (I). A producer-side notify after `set_output` removes this.
- **Failure, withdrawal, cancellation:** a failed predecessor leaves its successors Pending forever unless they are withdrawn; withdrawal is best effort (claimed entries cannot be withdrawn) and must be counted exactly (66a §4.2).
- **Single-lane queue, all lanes busy:** no deadlock from gates, because a closed gate never occupies a lane (I). No constructible deadlock found; livelock-like busy polling is bounded by the poll interval.
- **Priority / starvation:** take queues are FIFO; gated items keep their position and do not block later items (`SPEC/004#4`). No starvation from gates (I).
- **Interaction with 130:** under option A gated steps count as load; under option B only runnable steps count.

### Options
- **A Gate** (66a): `OpMeta.ready: Gate`; every predicate tests it; wake-up by polling or `Queue::notify_available()`.
- **B Staged release:** the Planner still produces the complete chain in one `expand`, but only the first step is pushed. Steps 2..n are carried inside the payload (`StepPayload.rest`) and pushed by `finish` through the `FollowUpSink` after the predecessor's output is set. The Processor code stays unchanged (`expand` pushes what the Planner returns).
- **C Dependency resolved by the queue:** queue knows predecessor tickets; changes the lock-free core of `queue.rs`.
- **D Per-task serialization in the worker:** a shared per-task step counter in the metadata; equivalent to A with a counter instead of a gate.

### Evaluation

| Criterion | A | B | C | D |
|---|---|---|---|---|
| Implemented code changed | none (optional `notify_available`) | none | `queue.rs` | none |
| Predicates to extend | 5 places, every batch member | none | none | 5 places |
| 117 token batching (N3) | broken without mutable meta | works (meta built at push, lengths known) | broken | broken |
| "Queue empty" / load | distorted | exact | exact | distorted |
| Wake-up | polling or new queue call | push bumps `Available` (existing) | queue internal | polling |
| Failure cleanup | withdrawal with races | drop the carried rest | queue internal | withdrawal |
| Fixed decision "Processor pushes all steps" | kept literally | needs owner reading: all steps pass the Processor's queues, the head via `expand`, the rest via the sink | kept | kept |

### Recommendation
**B (staged release)**, because it is the only option that keeps 117 batching correct (N3) and needs no predicate discipline, no withdrawal on failure and no wake-up extension. It keeps the essential fixed decision (one complete expansion) but **deviates from the literal wording** "the Processor pushes all steps"; the owner must confirm this reading. If the owner rejects it, A is the fallback with: `Queue::notify_available()` called by `finish` after `set_output`, gate tests in all five selection places, and `BatchMeta` lengths taken from an interior `OnceLock` filled by the predecessor (amendments in `amendments.md` §Alt-A).

### Open owner decisions
D3 (readiness model), including the reading of "the Processor pushes all steps".

---

## Q3. Payload, runner and data between steps (66a C7). Owner decision D4 (data model), D5 (follow-up placement)

### Facts
- `SPEC/117#3`: `BatchPayload` (payload: `batch_key`, `sequences`), `BatchMeta` (meta: `batch_key`, `seq_lengths`), `ExecBatch<P>{tasks, spans, batches}`, `BatchRunner::run(&mut Lane, ExecBatch<P>)`; after `run`: meter, drop lease, idle, one `notify_result`. V
- `SPEC/137#3 CpuPayload::execute_batch(ExecBatch<Self>, &CpuContext)` must complete/fail every task. V
- `SPEC/134#3 ComputeBackend::execute(&OpMeta, V, &ExecContext) -> Result<R,E>` returns "when the stream work is done" (synchronous). V
- `SPEC/139#1` returns without waiting; `SPEC/107` hands finished lanes to `SPEC/131` stage `GpuResults`. V
- Implemented `L060/a/materialization.rs`: `Materialized { Host(TensorRef), Device(DeviceResult) }` (`Clone`), `materialize(flag, DeviceResult)`. V
- `SPEC/036#2`: data flows by sharing a `RamSlot` or a device region (`IoRegion::Device{pin: Arc<Pin<R>>}`) between neighbours; a device region must exist when the operation is built. V
- `SPEC/126#4 Materialization`: the last operation always has `materialize == true`. V
- `SPEC/078`: `T = RamData`; intermediate steps `materialize == false` (contradiction, F9). V
- `SPEC/126` has no function that plans a single step; `announce`/`activate` happen only inside `expand` (step 7). V

### Analysis
- **`V` for the reference engine** is a payload that owns one node, the task context and, under Q2-B, the carried rest of the chain. It implements `BatchPayload`: `batch_key` from the operation (kind + model/index), `sequences` from the input, which under Q2-B is readable because the predecessor's output is set before the push (I).
- **One runner call for several tasks:** for each `tasks[i]` the runner calls `finish(output_i, sink)` or `fail(error_i, withdraw)` independently; one task's failure does not touch the others; `notify_result` once per batch stays as in 117 (I).
- **Asynchronous GPU path:** `finish` must run after the device result is complete. Two consistent placements exist (I): (a) the lane worker thread of 117 waits for the stream event and calls `finish` before `run` returns, so lease and claims live exactly as long as `run`; (b) `run` only launches, and the record `{claims, payloads, lease, lane}` is moved into an in-flight table keyed by the `GpuCompletion` tag; `finish` runs in the `on_finished` handler of 131 stage `GpuResults`. (a) needs no change of 117/131/107 and keeps N4 safe; it satisfies 139's "returns without waiting" at the level of `LaneResource::execute`, which only enqueues to the lane worker. Recommendation: (a) for the reference; (b) only if lane threads must not block.
- **F9 is a real conflict (I):** with `materialize == false` the step output is device-resident, and `T = RamData` cannot hold it. Because the last step is always materialized (`SPEC/126#4`), the task result is always host data, so a sum type for `T` is sufficient and the node's `advance` needs no change.
- Options for the data model:
  - **D4-a sum type** `StepOutput { Ram(RamData), Device(DeviceResult) }` as `T`; the result consumer accepts only `Ram`. No node change. `StepOutput` lives next to `materialization` (`L060/a`), because the node module (`L050/a`) is below it and stays generic.
  - **D4-b separate result type** (`OperationNode<O,I,T,R,D,E>` + finalizer): changes the node and every spec that names its parameters (66, 78, 135).
  - **D4-c data outside the node** (`IoRegion` slots of 36): device regions must be allocated at plan time, which contradicts `SPEC/134#4 Memory` (per-operation dynamic memory) and lazy loading (I).
  - **D4-d no device hand-over:** all `materialize == true`; contradicts `SPEC/126#4` and removes a planned optimisation.
- **Metadata for follow-ups:** queue choice, `materialize`, `required`, `descriptor` and `announce` are Planner rules (`SPEC/126#4` steps 5-7). Under Q2-B the Planner already builds every step's `OpMeta` in `expand`, so the sink needs no planning for planned successors. For stepwise follow-ups (78) a new non-planning call is needed: `Planner::place_step(task_id, priority, &PlannedOperation, next: Option<&PlannedOperation>) -> Result<(QueueKind, OpMeta), PlanError>` (queue assignment, materialization, announce; activation see D5).

### Options (summary)
D4: a / b / c / d as above. D5 (activation for stepwise follow-ups): (i) `place_step` activates (blocking on the runner thread); (ii) no activation, rely on lazy init of 106 (only under D2-i); (iii) stepwise follow-ups may only use resources already activated by `expand`.

### Evaluation
D4-a changes no implemented code and only 78/135 texts; D4-b changes the implemented node and three specs; D4-c and D4-d contradict 134/126. D5-ii is free under D2-i; D5-i blocks a lane worker for the load time.

### Recommendation
D4-a; runner placement (a); D5-ii under D2-i, D5-i otherwise; new `Planner::place_step`.

### Open owner decisions
D4, D5.

---

## Q4. Task, context and cancellation (F10, F11)

### Facts
- `Processor::submit` creates the task (`Enqueued`) and pushes it (`processor.rs#submit`). V
- Dispatcher (`SPEC/135#4`): `take()` on the task queue, skip if `Completed`/`Cancelled`, `expand`, on error `task.cancel()`, always `claim.complete(Ok(()))`. V
- `Task::claim` moves `D` out; `Task::cancel` only from `Enqueued`; `Task::complete` only from `Claimed` (V, `task_model.rs`, `queue_state.rs`).
- `TaskContext` (`L050/l040/l030/b/task_context.rs`): counter and first error; `complete_operation` debug-asserts at 0; `fail` sets cancelled once. No link to `Task`. V
- `SPEC/035#2`: "Task completion is tracked only by this counter"; 66a: the node with `ends_chain == true` completes the task. V

### Analysis: life of a task (recommended model)
1. `Engine::enqueue` -> `Processor::submit` -> task `Enqueued`, task queue item Pending.
2. Dispatcher checks permission (Q5), `take()`s the task item and calls `Processor::expand(&task)`.
3. `Planner::expand`: after the started check it **claims** the task (`task.claim()`, N1) and keeps the data `d`; `TaskResolver::resolve(&task, &d)`; expander, queue assignment, materialization, announce/activate; `ValueBuilder::build(&task, d, &spec, steps)` creates one `TaskContext::new(id, priority, 1)`, all nodes in chain order (input `k+1` = `OutputHandle` of `k`) and returns one `V` (head payload carrying the rest).
4. Processor pushes the head. Owner of the `TaskContext`: every `StepPayload` holds an `Arc`; it lives until the last payload of the task is dropped.
5. Worker claims (`take_where`), executes, `finish` -> `set_output`, `advance`, `register_follow_ups(1)`, sink pushes the next payload, `complete_operation`.
6. Last step: `advance` completes the task with `Ok(output)`; `complete_operation` returns `true`.
7. `Claim::complete` of each ops item (queue-level result only). Waiters on `Task::wait`/`TaskHandle` get the result. The last payload drop releases context and nodes.

### Analysis: cancellation and races
- **User cancel before expansion:** `Task::cancel` succeeds; dispatcher skips. V/I
- **User cancel after claim:** impossible today (`cancel` fails from `Claimed`); steps run to the end (I). Needs a decision (D6).
- **`fail` concurrent with the last `finish`:** in a linear chain under Q2-B only one step of a task is live at a time, so they cannot overlap (I). Under Q2-A a successor can run while its predecessor is still inside `finish` (gate opens at `set_output`); if that predecessor then fails (e.g. a panic after `set_output`), `fail` checks `state() == Claimed`, the successor completes the task in between, and the second `complete` panics in debug (TOCTOU in 66a §4.2 `fail` step 1) (I). With fan-out from handlers the same race exists in both models.
- **`withdraw_planned` vs. a claiming worker:** consistent, because withdrawn items are counted by the withdrawer and claimed items by their own `finish`/`fail` (`discarded`) (I).
- **Counting invariant `initial_open == pushed steps`:** under Q2-B `initial_open == 1` and every successor is registered before its push; holds by construction (I). Under Q2-A the context is created with `steps.len()` before the Processor pushes; `expand` pushes all or none (V `processor.rs#expand`), so it holds too.
- **Panic between `register_follow_ups` and `submit`:** the counter stays too high; the task result is unaffected because completion is done by the node (I). Accept and document.
- **Expansion failure after claim (N8):** the dispatcher must complete the claimed task with `Err`, which needs a mapping from `PlanError` to `E` (I).
- **Two completion authorities:** keep the node (`ends_chain`) as the only one that completes the `Task`; the counter only tells when all steps are accounted for. `SPEC/035#2` must say so.

### Options
D6 user cancel after claim: (a) not supported in the reference, documented (steps run to the end, the result is delivered); (b) `Engine::cancel(task)` via a registry task -> `Arc<TaskContext>` that calls `context.fail(cancel_error)` and completes the task with `Err`; (c) add `Claimed -> Cancelled` to `EntryState` (change of implemented code). D7 error mapping for N8: (a) `EngineBuilder::plan_error(Arc<dyn Fn(&PlanError) -> E>)`; (b) bound `E: From<PlanError>`.

### Evaluation
D6-b needs an identity for `Task` handles to find the context: `Task` has neither an id accessor nor a pointer comparison (V `task_model.rs#impl Task`), so `task_model` (implemented) must change. D6-c touches `queue_state` and the state model of every entry. D6-a changes no code and matches what `SPEC/035`/`SPEC/135` already allow (only `Enqueued` tasks are cancelled). D7-a avoids a bound that leaks the planner type into the task model.

### Recommendation
The Planner claims the task in `expand` (N1). D6-a for the reference, with D6-b as a later extension once `Task` has an identity. D7-a. Tests: see Q8.

### Open owner decisions
D6, D7.

---

## Q5. Expansion control, dispatcher and workers (specs 56, 130, 131, 135)

### Facts
- `SPEC/135#4`: the dispatcher runs on `WorkerPool` threads and expands each taken task at once. V
- `SPEC/130#1`: expansion only if an idle notice exists or the ops queue is empty; `select` prefers underserved first kinds. V
- `SPEC/131#4`: a second expansion driver (`Expand` stage) inside each sub-engine worker. V
- Idle notices are produced only by `SPEC/106` (`IterationOutcome.idle`). V

### Analysis
- Chain pushed at once (Q2-A): `len()` stays positive while any chain lives, so 130 allows expansion only through idle notices; without 106 (D2-ii) there are none, and expansion would stop (I).
- Follow-up pushes (all models) bypass expansion control by design; they continue admitted tasks. Under Q2-B the load measure (`len`) counts only runnable steps, which is what 130 intends (I).
- With 135 as written, 130 is unused, so the number of tasks in flight is unbounded (I).

### Options
(a) 135 dispatcher consults `ExpansionControl::select` before `take()`ing a task (needs `may_expand(sum of ops len == 0)` and the first kind from `Planner::first_resource_kind_of_task`); 131 `Expand` stage is removed. (b) 131 `Expand` stage is the only driver and 135's dispatcher only wakes. (c) No admission control.

### Evaluation
(a) keeps the implemented `WorkerPool` as the only expansion thread pool and matches 56a; (b) puts blocking activation on sub-engine workers, which 131 already accepts but which delays results; (c) loses the 130 capability.

### Recommendation
(a). Owner decision D8. Limitation (I): the take-family task queue lets the dispatcher see only `TaskParams` before claiming, and `TaskParams` carries no task kind, so `ExpansionControl::select` cannot prefer underserved first kinds; the dispatcher gates with `may_expand` and keeps FIFO order. The preference needs the first resource kind in the task-queue metadata (a later change of `Processor::submit`, not proposed here). Whoever feeds `report_iteration` with idle notices must also call `worker_pool::Notifier::notify()`, otherwise an `Idle` dispatcher is not woken when expansion becomes allowed.

### Open owner decisions
D8.

---

## Q6. Operation type and trait bounds (66a C3)

### Facts
- `PlannedOperation { kind, descriptor, resource, config }` and `OperationConfig { index, tokenizer, model, scorer, pooling, prefix, target_dimension, top_k, max_seq_len, dtype, scope }`, both `Clone, Debug, PartialEq` in code (spec 65 lists only `Clone, Debug`). V `L050/a/expander.rs`
- `SPEC/065#1`: "Der Planner baut daraus die `Operation<R>`"; `SPEC/126#2`: the Expander builds no `Operation<R>` and "the Planner supplies that part"; `SPEC/126#3 ValueBuilder`: "e.g. Operation<R>/OperationNode wiring". V
- `HasResourceRequirements` has no production implementor. V
- Bounds that need it: `SPEC/071#3 RequirementsFilter` (`M`), `SPEC/106#3` and `SPEC/117#3` (`N::Meta`), node `O` (66). `HasOperationKind`: node `O` only. V

### Analysis
- Executing backends need kind, resource (model, index), config, materialize, data. Kind/resource/config are in `PlannedOperation`; materialize and data are in the node; task id/priority are in node/context (I). Nothing in the reference path needs `Operation<R>`: its regions (`IoRegion`) are replaced by `OutputHandle` + `StepOutput` (Q3), its descriptor is already inside `PlannedOperation` and `OpMeta` (I).
- `Operation<R>` stays as a valid type for other users (keys `OperationParams`/`OperationDescriptor`, `RamData`, `RamSlot`, `DataShape`, `ScoredHitResult` are used widely). Only the sentences that make the Planner build it must change.
- Implementors required: `OpMeta: HasResourceRequirements + HasOperationKind + BatchMeta`; `PlannedOperation: HasResourceRequirements + HasOperationKind`.

### Options
(a) `O = PlannedOperation` (66a); (b) `O = Operation<R>` with a new `HasResourceRequirements` impl (string requirements cannot provide `resource_requirements::ResourceRequirements`, V `operation.rs#ResourceRequirements`); (c) a new wrapper type.

### Evaluation
(a) needs only two trait impls; (b) is impossible without mapping strings to typed requirements; (c) adds a type without benefit.

### Recommendation
(a), confirming 66a E4. Amend 65/126 wording. Owner decision D9 is only a confirmation.

### Open owner decisions
D9 (confirmation of `O = PlannedOperation`).

---

## Q7. Spec corrections and amendments (66a A1 to A9)

### Facts
Verification of each amendment against the full spec text (V, specs read in full):

| Amendment | Check against full text | Result |
|---|---|---|
| A1 (66) | 66 §3/§4 replaced by 66a §3.1/§4.1. Gate part depends on D3. | changed |
| A2 (78) | Old text quoted correctly in 78 §3 and §5. Does not fix F9 (78 internal). `Upstream` belongs to the shared input type. | changed |
| A3 (126) | `OpMeta` impls fine; `ready: Gate` only under D3-A; `TaskContext::new(.., steps.len())` only under D3-A; misses N1 (claim), N3 (`BatchMeta`), `place_step`. | changed |
| A4 (65) | 65 §5 lacks `DeviceToRam`. | confirmed |
| A5 (78) | Note on `materialize`; must also state `T = StepOutput` (D4). | changed |
| A6 (134, 117) | Withdrawal predicate compares `operation::TaskId` with `u64` (N7); under D3-B no withdrawal for planned chains; must cover batches. | changed |
| A7 (135) | `FollowUpSink` lacks context and metadata (N6); `T` becomes `StepOutput`. | changed |
| A8 (gate predicates) | Needed only under D3-A. | dropped (kept as Alt-A) |
| A9 (re-base) | Missing specs 092 (channels), 130 (`len`), 131 (bounds), 023/024/025 role note; 071 has no remaining role. | changed |

Further affected hits (V, search over `SPEC/`): `SPEC/056#2,#5` ("take_where over `Operation::requirements()`"), `SPEC/036#2` ("Workers filter on `OperationParams`"), `SPEC/035#2` ("operations are queue entries (`Task`/`EntryState`)", completion authority), `SPEC/132#2` ("workers take items with `take_where` on `OpMeta`"), `SPEC/134#2,#5` (`OpMeta` field list), `SPEC/135#4` (who claims), `SPEC/130#2,#5` (`iter_active`), `SPEC/106`, `SPEC/117`, `SPEC/131`, `SPEC/137`, `SPEC/139` (`QueueNode`), `SPEC/071` (whole module), `SPEC/092` (`queue::Queue::new(channels)`), `SPEC/078` (`OperationKind` trait).
Known smaller inconsistencies: spec 65 omits `DeviceToRam` (V); two `ResourceRequirements` (`operation` strings vs `resource_requirements` typed; V) - no action, the node and `OpMeta` use the typed one; two `TaskId` (`operation::TaskId(u64)` vs `task_context::TaskId = u64`; V) - N7 rule `meta.task_id.0 == node.task_id()`; `T` generic in 135 vs `RamData` in 78 - resolved by D4-a (`StepOutput` in both). Code comments N9 (not changed here).

### Analysis, Options, Evaluation
The amendments are text changes; their options are those of D1-D9. Applying the recommended set leaves no conflicting row in `impact.md` (b) (checked row by row there).

### Recommendation
Apply `amendments.md` for the recommended options; A8 is replaced by Alt-A text kept for the case D3 = A.

### Open owner decisions
None beyond D1-D9.

---

## Q8. Verification plan

### Facts
`proptest = 1`, `loom = 0.7`, `shuttle = 0.7` are dev-dependencies (V `Cargo.toml`). `loom` is used only in `src/.resident_old` with local `loom::sync` models (V). The implemented queue and task modules use `std` atomics and crossbeam-epoch, which `loom`/`shuttle` cannot instrument (I).

### Analysis
Model checking must therefore target small protocol models written against `loom::sync`/`shuttle::sync` (as `.resident_old` does), while the real modules are covered by unit, property and stress tests with real threads.

### Options and evaluation
- unit tests per module (deterministic sequences),
- `proptest` for chain shapes and failure positions,
- `loom` for the counting/completion protocol model (exhaustive, small),
- `shuttle` for randomized schedules with more threads (PCT scheduler),
- stress tests with real `std::thread` on the real types.

### Recommendation (scenarios and invariants)

| # | Scenario | Tool | Invariants |
|---|---|---|---|
| T1 | Full chain of 5 steps over Cpu/Gpu/Shared queues, 1 and 4 lanes per queue | unit + stress | steps run in chain order; `Task` completed once with last output; `open_operations()==0` at end; every ops `Ticket` final `Succeeded` |
| T2 | Failure at step k (k = 1..n) | proptest | task `Err(first error)`; no later step executed; carried rest dropped; counter 0; no debug panic |
| T3 | User cancel before expansion, during expansion and after the claim (D6-a) | shuttle | before: `Cancelled`, nothing pushed; after the claim: `cancel()` returns `false` and the chain delivers its result; exactly one `complete` |
| T4 | Withdrawal race: `withdraw_pending` vs. 3 claiming workers | loom model + stress | each item counted exactly once (withdrawn xor finished); counter never underflows |
| T5 | Run-time insertion (handler returns m extra nodes) and replacement of the carried rest | unit + proptest | registered before submitted; counter = live entries; final completion only by the `ends_chain` node |
| T6 | Batch across tasks: 8 tasks, mixed batch keys, one member fails | unit with mock runner | members never mix keys; per-task `finish`/`fail`; one `notify_result` per batch; failing member does not affect others |
| T7 | Readiness (D3-B): successor is not in the queue before predecessor's `set_output` | unit | `len()` of the target queue rises only after `finish` |
| T8 | Dispatcher error after claim (N8) | unit | task completed with mapped `Err`; no hang |
| T9 | Concurrent `fail` and last `finish` with fan-out | loom model | at most one `Task::complete`; first error kept |
