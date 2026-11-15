# Specification: Operationsknoten, cross-spec analysis (build order 66b)

This is an **analysis assignment**, not a code task. It hands over the open findings of build order 66a for a deep, evidence-based analysis. The result is a set of documents (§3). No code and no existing spec is changed.

## 1. Task, restrictions and boundaries

Analyse the open findings listed in §4 across all affected specs and across the implemented code. For each finding:
- Establish the facts from the specs and the code (with citations).
- Compare the solution options with explicit criteria.
- Recommend one option and list exactly what it changes.
- Produce the amendment text for every spec that the recommendation touches.

The goal is a congruent set: after the amendments, no two specs may contradict each other on the items in §4, and every consumer of a changed item is covered.

Out of scope:
- Implementing anything. Changing code, tests or existing spec files.
- Deciding the questions marked "owner decision" in §4. Give options, evidence and a recommendation only.
- Re-opening the decisions already taken by the owner (§2, "Fixed").

Restrictions:
- Read-only on `src/` and on `project/resident_02/23_batch/specs/`. Writing is allowed only in the output directory of §3. Running `runTests` is allowed to confirm behavior of existing code.
- Documents and citations in English. Module, type and function names exactly as in the specs or the code.
- Every statement is marked **VERIFIED** (read in the cited place), **INFERRED** (derived from verified facts, say how) or **UNKNOWN** (not found). Do not upgrade a hypothesis to a fact.
- No retrieval through shell tools. Use the structured read/search tools of the environment.

## 2. Context

**Fixed (owner decisions, not to be re-opened).**
- Both chain-creation modes must be possible: the full chain created in advance, and the chain created stepwise by continuation handlers. A chain may also be modified during a run.
- The reference implementation uses the full-chain model: the Planner/Expander creates the complete chain in one `expand` call, and the Processor pushes all steps.

**Inputs.**
- Specs: `project/resident_02/23_batch/specs/`. The ones that matter most: 004 (queue), 025 (queue structure and enqueue), 024 (queue memory reclamation), 035 (task context), 036 (operation), 037 (processor), 054 (resource requirements), 056 and 056a (worker pool), 065 (expander), 066 and 066a (operation node), 071 (pull execution), 078 (stepwise handover), 092 (notification topology), 106 (sub-engine loop), 107 (GPU completion), 117 (lane and batching), 126 (planner), 130 (backpressure and expansion control), 131 (worker loop), 132 (common sub-engine rules), 134 (sub-engine), 135 (engine facade), 137 (CPU sub-engine), 139 (GPU sub-engine). Search the other specs for the items in §4 as well.
- Code: `src/resident/` (module paths below are relative to `l130/l120/l110/` unless stated). Implemented and relevant: `a/processor.rs`, `a/worker_pool.rs`, `l100/l090/l080/l070/l060/l050/a/{expander,operation_node,similarity_report}.rs`, and under `l050/l040/l030/`: `b/{operation,task_context}.rs`, `l020/b/{task_model,task_handle,queue_list,queue_iter,queue_reclaim}.rs`, `l020/l010/{queue,queue_state}.rs`.
- Implementation runs: `project/resident_02/25_implement/runs/` (prompts that were used for the implemented modules).

**Known facts from the previous analysis (re-verify each one before you rely on it).**

| ID | Fact | Status |
|---|---|---|
| F1 | `queue.rs` defines `Queue<M,P,R,E>` with `push`, `take`, `take_where`, `Claim`, `Ticket`, states Pending/Running/Succeeded/Failed/Abandoned, channels Available/Completed. Dropping a `Claim` without `complete` gives `Abandoned`. | VERIFIED in code |
| F2 | `queue_list.rs` defines a different `Queue<M,P,T,E>` with `enqueue(meta, Arc<EntryState>)`, `head()`, `first()` and a `QueueNode` that implements `queue_iter::QueueNode`. `queue::Queue` does not implement `queue_iter::QueueNode`. | VERIFIED in code |
| F3 | `Processor` owns three `queue::Queue<O,V,R,OE>` (take family) plus a task queue `Queue<TaskParams, Task<D,T,E>, (), ()>`. `expand` pushes every `PlannedOp` with `push(meta, values)`. | VERIFIED in code |
| F4 | `Task` (`task_model.rs`) and `TaskHandle` are built on `queue_state::EntryState` (list family state machine). | VERIFIED in code (imports) |
| F5 | Specs 71, 106, 117, 131, 137 and 139 import `queue_iter::QueueNode` (list family). Specs 4, 37, 56, 134 and 135 use `queue::Queue` with `take_where` (take family). Spec 92 describes `queue::Queue::new(channels: Vec<NotifyChannel>)` (list family API under the take family's module name). | VERIFIED in specs (search hits only; read the full specs) |
| F6 | Spec 117 says the runner writes task results with "`set_output`, `advance`, `complete`". No spec defines the concrete payload type `V` or the runner. | VERIFIED in the searched specs |
| F7 | Spec 126 pushes one `PlannedOp{kind, meta: OpMeta, values: V}` per step, all at once, and describes `ValueBuilder` as wiring "input/output between neighbours". In the searched specs (037, 126, 134, 132, 106, 117, 131, 135, 056, 004, 035, 036, 065, 071, 130) nothing prevents a worker from claiming step `k+1` before step `k` has finished. 134 states "no pre-taking or reservation". | VERIFIED for the listed specs. UNKNOWN for 092, 107, 025, 137, 139 |
| F8 | Spec 126 §Materialization: `meta.materialize` is `false` only if this and the next operation are both `GpuOnly` and the next one is not `IndexWrite`. Spec 78 sets `materialize` `true` only for the last step. | VERIFIED in specs |
| F9 | `OperationNode<O,I,T,D,E>` has one type `T` that is both the output handed to the next step and the task result (`task.complete(Ok(output.clone()))`). Spec 78 fixes `T = RamData` and `RamData` is host data. Spec 36 has `IoRegion<R> = Ram(RamSlot) \| Device{pin: Arc<Pin<R>>, shape}`. | VERIFIED in specs/code. The consequence in Q3 is INFERRED |
| F10 | `TaskContext::complete_operation()` returns `true` only if it takes the open count from 1 to 0 and the task is not cancelled. `fail(e)` records the first error and cancels the context. Neither `Task::cancel` nor the queue knows the context. | VERIFIED in code. The interplay is the subject of Q4 |
| F11 | Task completion has two independent levels: `Claim::complete(Result<R,E>)` for a queue entry, and `Task::complete(Result<T,E>)` for the task. A second `Task::complete` panics in debug builds. | VERIFIED in code |
| F12 | Spec 66a (this assignment's predecessor) already assumes: the take-based family, `O = PlannedOperation`, a `Gate` in `OpMeta`, polling instead of wake-up for opened gates, and the call sequence of `StepPayload::finish`/`fail`. These are proposals, not facts. | VERIFIED (read 66a) |

## 3. Deliverables

Write these files to `project/resident_02/25_implement/analysis/066b/`:

1. `report.md`: one section per question of §4 (Q1 to Q8), in this order inside each: Facts (with citations), Analysis, Options, Evaluation, Recommendation, Open owner decisions.
2. `decisions.md`: a table of all owner decisions, one row each: ID, question, options (short), recommendation, consequence of each option in number of specs and code modules touched.
3. `amendments.md`: for every spec that the recommendations touch, the exact replacement text (old text quoted, new text given), grouped by spec number. Include the amendments A1 to A9 of 66a, each marked as confirmed, changed or dropped, with the reason.
4. `impact.md`: (a) a table of all specs and code modules that use any changed item, with the kind of use; (b) a cross-spec consistency table (item × spec) in which every cell shows the type, bound or signature that the spec uses, so contradictions are visible; (c) the effect on 66a, listing which of its elements (E0 to E6) stay, change or must wait.

Citation format: `<path relative to repo root>#<section or symbol>` for specs and code, plus the line when it helps.

## 4. Questions

For every question: the options named here are the minimum. Add options that you find.

**Q1. Queue families (66a C14). Owner decision.**
- Establish for every spec in §2 and every implemented module which queue API it uses, and which operations it needs (iteration without claiming, claim by predicate, claim with state transitions, abandon, notification channels, memory reclamation, ticket/result handling).
- Establish what the specs 4, 24, 25, 56, 71, 92, 106, 117, 131 require of the queue, and what `queue.rs`, `queue_list.rs`, `queue_iter.rs` and `queue_reclaim.rs` actually provide.
- Options: (A) take family is the reference, the list-family specs are re-based; (B) list family is the reference, the Processor and the specs 4, 37, 56, 134, 135 are re-based; (C) both stay and an adapter makes one visible as the other; (D) a unified third API. Include for each: what the batch scan of spec 117 becomes, how spec 92's notification topology is kept, and what happens to `Task`/`TaskHandle` (F4).
- Criteria: number of implemented modules that change, number of unimplemented specs that change, loss of a required capability, lock-freedom and reclamation guarantees, testability.

**Q2. Readiness of planned steps (66a C13). Owner decision.**
- Verify F7 in the specs marked UNKNOWN. Find every place where a worker, loop or batch scan selects an entry, and state what it would need to respect an order between steps.
- Options: (A) `Gate` in the queue metadata, tested in every selection predicate; (B) the Processor pushes only steps whose predecessor is done, the rest are held back and released by the runner (a staged release); (C) a dependency field resolved by the queue; (D) per-task serialization in the worker.
- Analyse: priority handling, batch formation across tasks (are batch members checked), starvation, behavior when a predecessor fails, is withdrawn or the task is cancelled, behavior when all lanes of a single-lane queue are busy, and the wake-up path (polling vs notification, spec 56 `Notifier`, spec 92). Include any deadlock or livelock scenario that you can construct.
- Check the interaction with spec 130 (expansion control and backpressure): do gated steps count as load?

**Q3. Payload, runner and data between steps (66a C7). Owner decision for the data model.**
- Define what `V` must be. Check `BatchPayload`, `BatchMeta`, `ExecBatch` and `CpuPayload` (specs 117, 137, 139) against a payload that wraps an operation node. Determine how one runner invocation that serves several tasks calls `finish` and `fail` per task.
- Analyse the asynchronous path of GPU work (spec 107, 131 stage semantics, 139): on which thread and at which moment `finish` runs, and what the lease and the claim do meanwhile.
- Analyse F9. If the node's `T` must carry data that stays on the device (steps with `materialize == false`) and also the final task result, then `T = RamData` cannot hold both. Establish whether this is a real conflict. Options: a sum type for step outputs, a separate type for the task result (which needs a change of the node's `advance`), device data held outside the node (slots/regions of spec 36), or no device-resident hand-over in the reference model. Check how specs 36, 71, 78, 126, 134 and 139 treat the output of a step with `materialize == false`.
- Establish who creates the queue metadata (`OpMeta`) for a follow-up node returned by `advance` (queue choice, `announce`, `activate`, `required`, `descriptor`), and whether the Planner needs a new function for it.

**Q4. Task, context and cancellation (F10, F11).**
- Trace the life of a task end to end: `submit`, `expand`, push, claim, execute, `finish`/`fail`, task completion, handle result, drop. Name the owner of the `TaskContext` and its lifetime.
- Determine what happens when the user cancels a `Task` (`Task::cancel`) while steps are pending or running, and when `TaskContext::fail` is called while another worker finishes the last step.
- Find races in the sequences of 66a §4.2 (`finish`, `fail`, `withdraw_planned`), including the counting invariant `initial_open == pushed steps`. Use the available concurrency test tools of the repository (`loom`, `shuttle`) in your proposal for tests.

**Q5. Expansion control, dispatcher and workers (specs 56, 130, 131, 135).**
- Establish how `Processor::expand`, the task dispatcher (`WorkSource::run_once`), the expansion control and the ops queues interact when a chain is pushed at once and when follow-ups are pushed later.
- Establish whether follow-up pushes bypass the expansion control, and what that does to its load measure.

**Q6. Operation type and trait bounds (66a C3).**
- Verify that `PlannedOperation` is a sufficient operation type for every consumer: the executing backends need what from it (kind, resource, descriptor parameters, config such as model, index, top-k, dimension)?
- Establish the role that remains for `Operation<R>` (spec 36) and `OperationDescriptor`, including their use for keys and filtering. Decide whether anything in the specs still requires `Operation<R>` in the reference path.
- Establish which types must implement `HasResourceRequirements` and `HasOperationKind` (workers' `N::Meta` bound, node's `O`), and check all bounds in the specs for consistency.

**Q7. Spec corrections and amendments (66a A1 to A9).**
- Verify each amendment of 66a §7 against the full text of the spec it changes. Report conflicts that the amendment would create or fail to remove.
- Search all specs for further uses of the changed names and items (`OperationKind`, `OperationNode`, `HasResourceRequirements`, `OpMeta`, `QueueNode`, `claim_entry`, `iter_active`, `take_where`, `Claim`, `ChainInput`, `ValueBuilder`). Report every hit that is affected.
- Check the smaller inconsistencies already known: spec 65 omits `DeviceToRam`; two `ResourceRequirements` types; two `TaskId` types; spec 135 keeps `T` generic while spec 78 needs `T = RamData`.

**Q8. Verification plan.**
- Propose the test strategy for the recommended solution: which scenarios, which tool (unit, `proptest`, `loom`, `shuttle`), which invariants. Cover at least: the full chain across several queues and lanes, failure in the middle, user cancellation, withdrawal races, run-time insertion, and batch execution across tasks.

## 5. Method rules and acceptance

Method:
1. Read each spec of §2 completely (all sections), not only the hits of a search. Read the cited code.
2. Re-verify F1 to F12 first. If a fact is wrong, say so at the top of `report.md` and correct the dependent analysis.
3. Build the cross-spec table of `impact.md` (b) before you write recommendations. Use it to find contradictions.
4. Evaluate options with the stated criteria. Prefer the option that keeps implemented code unchanged and spec changes smallest, unless it loses a required capability. State when you deviate from that.
5. Do not decide owner decisions. Mark them and give a recommendation.

Acceptance:
- Every question has all six parts, and every claim has a citation and a VERIFIED/INFERRED/UNKNOWN mark.
- `decisions.md` lists every owner decision with the consequence of each option.
- `amendments.md` gives exact text for every touched spec. Applying all recommended amendments leaves no row of the consistency table (`impact.md` (b)) with conflicting entries.
- `impact.md` (c) states for each of E0 to E6 of 66a whether it stays, changes or must wait for an owner decision.
- No file outside the output directory is changed.

## 6. Relation to 66a

66a can be implemented independently for the parts that do not depend on this analysis: E0 (operation kind), E1 (`ends_chain`), E2 (`task_id`) and E4 (`PlannedOperation` impls). The parts E3 (`Gate`, `OutputHandle`), E5 (`StepPayload`) and E6 (withdrawal) rest on assumptions that Q1 to Q4 examine (take-based family, readiness by `Gate`, `T` as the single output type, the call sequence). If an owner decision of Q1 to Q3 contradicts them, those parts must change. The result of this analysis is the input for the next revision of 66a.
