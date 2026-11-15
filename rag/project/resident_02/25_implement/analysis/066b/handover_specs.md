# Handover: spec update after the implementation of 66a and D10 (build orders 66, 66a, 66b)

This file continues `handover.md` in the same directory. Read `handover.md` first for environment, rules and tool notes, then this file completely.

## 1. Done (code, verified)

`runTests`: 838 + 8 tests pass.

- 66a implemented as in `066a_r1_draft.md` (E0 to E6). Files in `src/resident/l130/l120/l110/l100/l090/l080/l070/l060/l050/a/`: `operation_node.rs` (+ `test/operation_node.rs`) reworked, `step_completion.rs` (+ `test/step_completion.rs`) new, `mod.rs` registers `pub mod step_completion;`. No `trait OperationKind` is left in `src/resident`.
- D10 implemented (owner gave the go): `src/resident/l130/l120/l110/a/processor.rs` has the new type `TaskMeta { params: TaskParams, first_kind: Option<ResourceKind> }`; `TaskQueue<D,T,E> = Queue<TaskMeta, Task<D,T,E>, (), ()>`; `Processor::submit(data, params, first_kind: Option<ResourceKind>)`. `test/processor.rs` adapted, new test `task_queue_can_be_filtered_by_first_kind`.
- Not tested on purpose: fan-out failure after the end node ran (the draft only forbids it).

## 2. Task now: bring the spec files in line (the owner said: "change the specs now")

Source of the texts: `amendments.md` in this directory (exact old/new texts per spec). Read it with `read_file` (the owner allows this for markdown; for code use the AST tools, the owner rejected `read_file` for code). Specs: `project/resident_02/23_batch/specs/`.

### 2.1 Spec files already changed (all amendment items applied)
025, 035, 036, 056, 065, 066, 066a, 071, 078, 092, 106, 117, 126, 130, 131.

Special cases in that set:
- 066: sections 1 to 4 replaced as a whole (the draft's section 4.1 is a delta, so a complete, merged section 4 was written); section 5 got relied-on entries for `OperationKind` and `PlannedOperation`.
- 066a: whole file replaced by the draft text (title without "draft"/"revision 1"; removed the sentences "No spec file is changed" and "All 813 unit tests ...").
- 071: marked withdrawn (only the first paragraph of section 1 replaced).
- Edits beyond the literal amendment texts (tell the owner): 092 section 1 ("one notification handle per sub-engine", the restriction bullet about notifying other sub-engines now says every notification wakes the worker pool) and section 4 (the `notify_one()` wording changed to "the wake function"; the "signal sent from inside the loop's own iteration" bullet was deleted by an empty replacement, check for a leftover empty bullet); 117 section 5 (the `ResourceExec` bullet updated to `Entry`); 130 section 2 (the introductory note of the amendments became the first bullet); 066 sections 1, 2, 5.

### 2.2 Still to do
1. **134** (whole sections 1, 3, 4, 5 replaced, plus the intro sentence), **135** (many items, see `amendments.md`), **137**, **139**. Apply exactly as in `amendments.md`.
2. **D10 texts** (new, not yet in `amendments.md`; add them there too and mark D10 as decided in `decisions.md`):
   - **037** (processor): section 2 (task queue is `Queue<TaskMeta, Task<D,T,E>, (), ()>`), section 3 (`TaskMeta`, `TaskQueue`, `submit(data, params, first_kind: Option<ResourceKind>)`), section 4 submit (the queue item's meta equals `TaskMeta { params, first_kind }`; the Processor does not interpret `first_kind`), section 5 (`ResourceKind` from `resource_requirements`, `Copy`).
   - **135** (engine facade): `enqueue_embedding`/`enqueue_similarity` pass `Planner::first_resource_kind_of_task(..)` as `first_kind` to `Processor::submit`; the expansion stage filters with `take_where(|m| m.first_kind.map_or(false, |k| underserved.contains(&k)))` (the amendment text for the dispatcher already says "the task-queue metadata carries the first resource kind"; make it concrete: `TaskMeta.first_kind`).
   - **126** (planner): note that `first_resource_kind_of_task` is what the Engine puts into `TaskMeta.first_kind` (no code change of the Planner).
3. **Verification pass** over every changed spec (read the sections back with `ast_find` by section id). Points to check: 065 code block (two edits ran on a code-block node whose id changed in between; confirm `PlannedOperation` and `OperationConfig` derive `PartialEq` and have the English doc comments); 078 section 3 (the `start_chain` signature edit replaced the lines from `task: Task<D, StepOutput, E>,` to the `where` clause; confirm that the signature is intact); 126 section 3 (`OpMeta`, `BatchInfo`, impls, `TaskNotClaimable`, `place_step`, `ValueBuilder`); 092 section 4 (no empty bullet); 117 section 4 (item 1 was fixed with `ast_edit_block` after an ambiguous end marker).
4. `066b_operationsknoten_analyse.md` in the specs directory was not touched (it is not named in `amendments.md`). Ask the owner only if you find a conflict.
5. Do not run Rust tests for spec-only changes; run `runTests` only if code is touched again.