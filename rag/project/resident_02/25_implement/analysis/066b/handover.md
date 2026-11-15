# Handover: operation node reconciliation (build orders 66, 66a, 66b)

This file replaces the previous session. Read it completely before you act. Do not read the run prompt files in `project/resident_02/25_implement/runs/` that belong to this work (`066a_clarify_ops.md`); the owner rejects that.

## 1. Environment and rules

- Crate root: `/home/user/xyan/xy.ai.workbench/rag` (`Cargo.toml` there; `src/lib.rs` already has `pub mod resident;`).
- Code target: `src/resident/l130/...`; the module directory tree is fixed (see `project/resident_02/27.tree.yml` and the tree in the original task). Direct child modules live in subdirectories.
- Model: `/home/user/xyan/xy.ai.workbench/rag/model/multilingual-e5-small`. GPU: GTX 1660 SUPER, sm_75, CUDA 13.3 (`/usr/local/cuda-13.3/bin/nvcc`).
- Project documents and code comments in English only; minimal comments; no references to plan elements or plan numbers in comments. Module, type and function names in English, chosen by you.
- Tests of a module live in `<level>/test/<name>.rs`; the source file ends with
  ```rust
  #[cfg(test)]
  #[path = "test/<name>.rs"]
  mod tests;
  ```
- No Bash for retrieval. Use the structured tools. Predefined commands (`run_command`): `runTests` (cargo test), `read.Cargo.toml`.
- Verify the actual implementation of every referenced element. Find conflicts first against already done specs, then against future consumers.
- Talk to the owner in German, short and in plain words; the owner dislikes jargon. No closing summary.

Tool notes:
- Markdown: `ast_read` does not expand bullet lists. Use `ast_find` with the root section id (from `ast_outline`) to get the full text of a spec. Numeric ids (for example spec 130 root `234159`, yml entries) break `ast_find(id)` and `ast_read`; use `ast_find` with `regexp` instead (for yml: `regexp: "Name: <module>\n\s*Order"`).
- After an `ast_replace` in a markdown file, the ids of sibling nodes change; re-find before the next edit.

## 2. Sources

- Specs: `project/resident_02/23_batch/specs/` (066, 066a, 066b and all specs named below).
- Original module requirements: `project/resident_02/21_modules_ordered.yml` (authoritative intent; check every decision against it).
- Analysis results (written in the previous session): `project/resident_02/25_implement/analysis/066b/`
  - `report.md` (Q1-Q8, facts with V/I/U marks; §0.4 = status after owner decisions, wins over older text below it)
  - `decisions.md` (decided and open decisions)
  - `amendments.md` (exact replacement texts per spec; consistent with all decisions)
  - `impact.md` (users, consistency table, effect on 66a E0-E6)
  - `066a_r1_draft.md` (revised 66a: the implementation spec for the next step)

## 3. State of the code (verified)

- 66a is not implemented. `src/resident/l130/l120/l110/l100/l090/l080/l070/l060/l050/a/operation_node.rs` still has the spec-66 string trait `OperationKind`; `step_completion.rs` does not exist. `mod.rs` there registers `encoder_interface`, `expander`, `operation_node`, `similarity_report`.
- `runTests`: 813 + 8 tests pass.
- Take-family queue `L020/l010/queue.rs` is used only by `L110/a/processor.rs`. List family (`L020/b/queue_list.rs`, `queue_iter.rs`, `queue_reclaim.rs`), `task_handle`, `task_context` have no production consumers.
- `Task` (`L020/b/task_model.rs`): data only via `claim()` (moves it out); `cancel()` only from `Enqueued`; second `complete` panics in debug when already `Completed`.
- Aliases: `L110/` = `src/resident/l130/l120/l110/`, `L050/` = `L110/l100/l090/l080/l070/l060/l050/`, `L060/` = `L110/l100/l090/l080/l070/l060/`, `L020/` = `L050/l040/l030/l020/`.

## 4. Decisions

Decided by the owner:
- **D2**: CPU and GPU sub-engines follow specs 137/139 (on 132, 106, 117, 131). Worker-pool threads (56) are the only workers (at most cores - 1). CPU work runs synchronously on the worker (thread disappears in Candle). GPU work is only launched; a free worker finishes it after the completion notification (131 stage `GpuResults`, `LaneResource::on_finished`, `BatchRunner::collect`). 134 becomes `SubengineHandle` without threads or memory; 117 starts no threads.
- **D3**: staged release. The chain is planned completely in one `expand`; only the first step is pushed; the rest travels in `StepPayload.rest` and is pushed by `finish`. Reading accepted: "the Processor pushes all steps" = head via `expand`, rest via the sink.
- **D8**: idle notifications are a pure prioritisation hint for the expansion. Nothing is withheld, delayed or refused (three operations queues). Expansion stays the fourth stage of the Worker-Loop.
- Eviction happens from RAM and VRAM (RAM shorter period).
- Batching: a worker takes one request (seed) and collects further compatible ones into the batch.

Recommended, not explicitly confirmed (treat as accepted unless the owner objects): D1-A (take queues), D4-a (`StepOutput{Ram, Device}` as `T`, defined in `chain_handover`/78), D5-ii (`Planner::place_step`, no activation), D6-a (no cancel after the Planner claimed the task), D7-a (`EngineBuilder::plan_error`), D9-a (`O = PlannedOperation`). The Planner claims the task in `expand`.

Open, question asked, no answer yet:
- **D10**: to prioritise, the expansion must know a waiting task's first resource kind before claiming it. Proposal: `enqueue_embedding`/`enqueue_similarity` put the kind into the task-queue metadata (small change of the implemented `processor.rs`). Alternative: no prioritisation, oldest first. Does not block 66a.

## 5. Next step (asked, waiting for the owner's go)

Implement `066a_r1_draft.md` completely (E0-E6): rework `operation_node.rs` (+ `test/operation_node.rs`), create `step_completion.rs` (+ `test/step_completion.rs`), register `pub mod step_completion;` in `.../l050/a/mod.rs`. Only these files may change. Then `runTests`; all tests must pass. Ask the owner about D10 if still open; it does not affect this step.

Key points of the draft to keep:
- `HasOperationKind` replaces the string trait; impls for `PlannedOperation`; alias `PlannedNode`.
- Node: `task_id()`, `ends_chain` (default `true`), `OutputHandle<T>` (no `Gate`).
- `StepPayload<M, O, I, T, D, E> { node, context, rest: Vec<PlannedStep<M,..>> }`; `new` sets `ends_chain = false` if `rest` is not empty; `replace_rest`.
- `FollowUpSink<M, P>::submit(Option<M>, P)`; `finish(&mut self, output, &S)`: cancelled check, `set_output`, `advance`, register `n` follow-ups before submitting, planned next first (`Some(placement)`), then handler nodes (`None`; only the last one ends the chain and only without a planned rest), then `complete_operation`.
- `fail(&mut self, error, withdraw)`: first error completes the task with `Err` if `Claimed`; rest dropped (never counted); withdrawn items counted; one `complete_operation` for the step.
- `withdraw_pending` / `withdraw_planned` over `queue::Queue` via `take_where` + dropping the claim.
