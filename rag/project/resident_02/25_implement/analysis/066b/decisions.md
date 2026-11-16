# Operation node: owner decisions (build order 66b)

Counts are numbers of specs (S) and implemented code modules (C) that change if the option is chosen. Code counts exclude new modules. Details: `report.md`, texts: `amendments.md`.

## Decided by the owner (2026-11-15)

| ID | Question | Decision | Note |
|---|---|---|---|
| D2 | Which description of the CPU and GPU sub-engines is valid | Specs 137/139 on 132, 106, 117, 131. Spec 134 becomes the facade-side handle without threads or memory. | The worker-pool threads (56) are the only workers; CPU work runs synchronously on them, GPU work is launched there and finished by a free worker after the completion notification. Lane resources (117) start no threads. |
| D3 | Readiness of planned steps | B, staged release. "The Processor pushes all steps" is read as: the head via `expand`, the rest via the sink. | |
| D8 | Role of idle notifications | Pure prioritisation hint for the expansion. Nothing is withheld, delayed or refused (three operations queues). The expansion stays the fourth stage of the Worker-Loop (131). | Replaces the earlier gate recommendation. |
| D10 | How the expansion learns the first resource kind of a waiting task, needed for the prioritisation of D8 | a: `Engine::enqueue_embedding`/`enqueue_similarity` pass the kind into the task-queue metadata. | Implemented in `processor.rs` (`TaskMeta { params, first_kind }`, `submit(data, params, first_kind)`). Spec texts in `amendments.md` (037, 135, 126). |
| - | Eviction | Resources are evicted from RAM as well as from VRAM; the RAM period is shorter. | Matches the module requirements; no amendment needed. |

## Recommended, not yet confirmed

| ID | Question | Options (short) | Recommendation | Consequence per option |
|---|---|---|---|---|
| D1 | Queue family for task and ops queues | A take family; B list family; C adapter; D unified API | **A** | A: S 9, C 0. B: S 5 + 024/025, C 3 (`processor`, `queue_list`, `queue_state`). C: S 2-4, C 0 + adapter, removal/abandon missing. D: C 1 (`queue.rs` core). |
| D4 | Type of step output and task result | a `StepOutput{Ram, Device}`; b separate result type; c data outside the node; d no device hand-over | **a** | a: S 3, C 0. b: S 4, C 1 (`operation_node`). c: S 5, needs plan-time VRAM. d: S 2, no device hand-over. |
| D5 | Activation of stepwise follow-ups | i `place_step` activates; ii lazy init in 106; iii only activated resources | **ii** | all: new `Planner::place_step`; ii needs no activation. |
| D6 | User cancellation after the Planner claimed the task | a not supported, documented; b `Engine::cancel` via context registry; c `Claimed -> Cancelled` in `EntryState` | **a** (b later) | a: S 1, C 0. b: S 2, C 1 (`task_model` needs a task identity). c: S 3, C 2. |
| D7 | Mapping of `PlanError` to the task error | a mapper in `EngineBuilder`; b bound `E: From<PlanError>` | **a** | a: S 1, C 0. b: S 2, C 0, planner type in every `E`. |
| D9 | Operation type of the node in the reference path | a `PlannedOperation`; b `Operation<R>`; c wrapper | **a** | a: S 2, C 0. b: C 1. c: S 3. |

