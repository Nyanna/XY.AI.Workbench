# Operation node: impact (build order 66b)

Aliases as in `report.md` §0.1. "after" = after applying `amendments.md` (owner decisions D2, D3, D8 and the recommended options).

## (a) Users of changed items

### Specs

| Spec | Changed items it uses | Kind of use |
|---|---|---|
| 004 queue | `Queue`, `take_where`, `Claim` | definition (unchanged) |
| 025 queue list | list `Queue`, `QueueNode`, `NotifyChannel` | definition; role note added |
| 035 task context | `TaskContext`, completion authority | definition; context text |
| 036 operation | `OperationKind`, `Operation<R>`, `OperationDescriptor` | definition; role of `Operation<R>` |
| 037 processor | `Queue`, `PlannedOp`, `Planner` | pushes what the Planner returns (unchanged; D10-a adds the first kind to the task-queue metadata) |
| 056 worker pool | `WorkSource`, `Notifier` | the engine's only worker threads |
| 065 expander | `PlannedOperation`, `OperationKind`, `Operation<R>` | producer of `PlannedOperation` |
| 066 operation node | `OperationKind` trait, `advance`, `ContinuationHandler` | definition (replaced by 66a r1) |
| 066a | E0-E6 | definition (replaced by r1) |
| 071 pull execution | `QueueNode`, `iter_active`, `NotifyChannel` | withdrawn |
| 078 chain hand-over | `OperationNode`, `OperationKind` trait, `T = RamData`, `ChainInput` | consumer and producer of nodes |
| 092 notification topology | list `Queue::new(channels)`, `NotifyChannel` | wake-up, now through the injected pool wake |
| 106 sub-engine loop | `QueueNode`, `iter_active`, `claim_entry`, `ResourceExec` | selector, runs on pool workers |
| 107 GPU completion | `SubengineNotifier::notify_result`, `FinishedLane` | GPU hand-over (API unchanged) |
| 117 lane and batching | `QueueNode`, `claim_entry`, `BatchMeta`, `BatchRunner`, own threads | selector, batch scan, runner; threads removed |
| 126 planner | `OpMeta`, `ValueBuilder`, `TaskResolver`, `PlannedOperation`, `Task` | producer of metadata and payloads |
| 130 backpressure | `may_expand`, `select`, `iter_active` | prioritisation hint only |
| 131 worker loop | `QueueNode`, `ExpandStep`, `GpuCompletion` | the worker's scheduling core on pool threads |
| 132 common sub-engine rules | `take_where` on `OpMeta` (text) | none (consistent) |
| 134 sub-engine | `ComputeSubengine`, own threads, `ComputeBackend` | becomes `SubengineHandle` |
| 135 engine facade | `Queue`, `OpMeta`, `ValueBuilder`, dispatcher, `V`, `T` | wiring, `WorkSource` = Worker-Loop, sink |
| 137 CPU sub-engine | `QueueNode` bound, `CpuPayload`, runner | lane resource and runner |
| 139 GPU sub-engine | `QueueNode` bound, asynchronous lane rule | lane resource and runner |

### Code modules (implemented)

| Module | Changed items it uses | Kind of use | Change needed |
|---|---|---|---|
| `L050/a/operation_node.rs` | `OperationKind` trait, `advance` | definition | yes, by 66a r1 (E0-E4) |
| `L050/a/step_completion.rs` | - | new | new, by 66a r1 (E5-E6) |
| `L050/a/expander.rs` | `PlannedOperation` | gets two trait impls (in `operation_node.rs`) | no |
| `L110/a/processor.rs` | take `Queue`, `Planner` trait, `TaskQueue` | owner of queues | only with D10-a (task-queue metadata) |
| `L110/a/worker_pool.rs` | `WorkSource`, `Notifier` | worker threads | no (stale comment N9) |
| `L020/l010/queue.rs` | take `Queue`, `Claim` | definition | no (stale comment N9) |
| `L020/l010/queue_state.rs` | `EntryState`, `CompleteError` | task state machine | no |
| `L020/b/task_model.rs` | `Task::claim/complete/cancel` | task handle | no |
| `L020/b/task_handle.rs` | `EntryState` | result access | no |
| `L020/b/queue_list.rs`, `queue_iter.rs`, `queue_reclaim.rs` | list family | not used on task/ops queues | no |
| `L050/l040/l030/b/task_context.rs` | `TaskContext` | counter and first error | no |
| `L050/l040/l030/b/operation.rs` | `OperationKind`, `TaskId` | vocabulary | no |
| `L050/l040/a/resource_requirements.rs` | `HasResourceRequirements` | trait | no |
| `L060/a/materialization.rs` | `DeviceResult` | used by `StepOutput` (78) | no |

## (b) Cross-spec consistency table

Each cell shows what the spec uses: before -> after. A conflict would be a row group with two different "after" values; there is none.

### B1 Operations queue type and claim

| Spec | Before | After |
|---|---|---|
| 004 | `queue::Queue<M,P,R,E>`, `take_where` | same |
| 037 | `Queue<O,V,R,OE>`, push | same |
| 056 | `take_where` over `Operation::requirements()` | `WorkSource` runs the Worker-Loop |
| 071 | list `QueueNode`, `claim()` | withdrawn |
| 092 | list `Queue::new(channels)` | no queue API; `notify_push()` |
| 106 | `iter_active` + `claim_entry` | `take_where` (survey + claim), `Claim` |
| 117 | `iter_active` + `claim_entry` | `take_where`, `Claim<M,P,(),()>` |
| 130 | `iter_active(..).next().is_none()` | not used |
| 131 | `QueueNode` bound | `Claim<M,P,(),()>` bound |
| 134 | `take_where` + `wait_change` | none (handle) |
| 135 | take `Queue<OpMeta,V,R,OE>` | `Queue<OpMeta,EngineStep,(),()>` |
| 137, 139 | `QueueNode` bound | `LaneResourceConfig<M,P>` |

### B2 Threads and wake-up

| Spec | Before | After |
|---|---|---|
| 056 | pool threads run the dispatcher | pool threads are the only workers; park on `Notifier` |
| 092 | per-sub-engine `tokio::Notify`, signalled by the queue | per-sub-engine handle; every signal calls the pool `Notifier::notify` |
| 106 | non-blocking iteration on a sub-engine lane | iteration on a pool worker; CPU `execute` runs synchronously there |
| 117 | one own thread per lane | no threads |
| 131 | caller waits on `SubengineNotifier` | caller is `WorkSource::run_once` on a pool worker |
| 134 | own lane threads, `wait_change(.., poll_interval)` | no threads, no wait |
| 135 | `Notifier::notify()` after submit | same, plus `notify_push()` after expand and in the sink |

### B3 Operation type and bounds

| Spec | Before | After |
|---|---|---|
| 036 | `Operation<R>` executed by workers (implied) | not used in the reference path |
| 065 | Planner builds `Operation<R>` | `PlannedOperation` is the node's operation |
| 066 | `O: HasResourceRequirements + OperationKind` (trait) | `O: HasResourceRequirements + HasOperationKind` |
| 078 | `O: HasResourceRequirements + OperationKind + Send + Sync + 'static` | `O: HasResourceRequirements + HasOperationKind + Send + Sync + 'static` |
| 126 | `OpMeta` without impls | `OpMeta: HasResourceRequirements + HasOperationKind + BatchMeta` |
| 106/117 | `N::Meta: HasResourceRequirements (+ BatchMeta)` | `M: HasResourceRequirements (+ BatchMeta)` |

### B4 Output type `T` and task result type

| Spec | Before | After |
|---|---|---|
| 066 | generic `T` | generic `T` |
| 078 | `T = RamData`, intermediate `materialize == false` | `T = StepOutput` |
| 126 | `materialize == false` between GPU steps | same; output `StepOutput::Device` |
| 135 | generic `T` | reference `T = StepOutput` |
| 036 | `IoRegion` hand-over | not used in the reference path |

### B5 Payload `V`, runner and step sequence

| Spec | Before | After |
|---|---|---|
| 066a | `StepPayload{node, context}`, `FollowUpSink<N>` | `StepPayload<M,..>{node, context, rest}`, `FollowUpSink<M,P>` |
| 117 | `run(&mut Lane, ExecBatch)` on a lane thread, then lease dropped | `run -> Launch::{Finished, Pending}` on the pool worker; `collect` after the GPU event; claims completed and lease dropped when the batch is finished |
| 126 | `Vec<V>`, one per step | one `V` (head) carrying the rest; `(V, Option<BatchInfo>)` |
| 131 | `on_finished` interprets lanes | `on_finished` routes to `LaneResource::on_finished` |
| 134 | `ComputeBackend::execute(&OpMeta, V, ..)` | removed |
| 135 | `V` generic | `EngineStep{step, sink}` |
| 137 | `CpuPayload::execute_batch` completes tasks | calls `finish`/`fail`; `Launch::Finished` |
| 139 | returns without waiting (open how) | `Launch::Pending`, finished in `GpuResults` |

### B6 Readiness and batch metadata

| Spec | Before | After |
|---|---|---|
| 066a | `Gate` in `OpMeta`, polling | no gate; staged release |
| 117 | `BatchMeta::seq_lengths` from immutable meta | filled by the pusher before the push |
| 126 | all steps pushed | head pushed, rest carried |
| 134 | no ordering | ordering by construction |

### B7 Task claim, completion and cancellation

| Spec | Before | After |
|---|---|---|
| 035 | completion tracked only by the counter | node completes the task; counter accounts steps |
| 066/66a | `ends_chain` node completes | same |
| 126 | never claims | claims once in `expand` |
| 135 | dispatcher never claims; on error `cancel()` | Planner claims; on error `complete(Err(plan_error(e)))` if `Claimed`; cancel only before the claim |
| 078 | node completes with `Ok(output)` | same |

### B8 Expansion and prioritisation

| Spec | Before | After |
|---|---|---|
| 130 | expansion only with idle notice or empty queue | hint only: `underserved()`, `select(candidates)` |
| 131 | `Expand` stage | same stage, prefers tasks for underserved resources |
| 135 | separate dispatcher expands every task at once | the dispatcher logic is the `Expand` stage of the Worker-Loop |
| 037 | task-queue metadata `TaskParams` | with D10-a: plus the first resource kind |

### B9 Task id in metadata

| Spec | Before | After |
|---|---|---|
| 126 | `OpMeta.task_id: operation::TaskId` | same |
| 066a | compares with `u64` | `m.task_id.0 == node.task_id()` |
| 035 | `TaskId = u64` | same |

### B10 Sub-engines

| Spec | Before | After |
|---|---|---|
| 132 | `SubengineCore` implements `Subengine` | same |
| 134 | `ComputeSubengine` with own threads, backend and memory | `SubengineHandle`: forwards `Subengine`, provides feeds and GPU routing |
| 137/139 | built on 132/106/117 | same, on take queues and pool workers |

## (c) Effect on 66a (E0 to E6)

| Element | Status | Note |
|---|---|---|
| E0 `HasOperationKind` | stays | |
| E1 `ends_chain` | stays | used by the planned rest and the fan-out rule |
| E2 `task_id()` | stays | withdrawal compares `m.task_id.0` |
| E3 `OutputHandle` / `Gate` | changes | `OutputHandle` stays; `Gate` removed (D3-B decided) |
| E4 `PlannedOperation` impls, `PlannedNode` | stays | |
| E5 `StepPayload`, `finish`, `fail` | changes | `rest`, `&mut self`, `FollowUpSink<M,P>` with placement, as in `066a_r1_draft.md` |
| E6 `withdraw_pending`, `withdraw_planned` | stays (role reduced) | used for fan-out and replacement of pushed steps |

All of E0-E6 can now be implemented as in `066a_r1_draft.md`. None of them depends on D10.
