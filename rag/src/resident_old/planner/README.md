# Planner

Stateless expansion of a task into an ordered chain of operations. Same inputs, same operations; no queues, no workers, no notification. Scheduling, the counter of open operations and cancellation belong to the processor; the planner only decides what has to happen and with which parameters.
## Concept

- **Inputs:** the task's parameters (never its data, which enters the chain only as the first operation's input), the rules, the engine offers (which resource kinds the CPU and GPU side can serve) and the current starved hint. Nothing else influences the plan, which keeps expansion a pure function that can be tested on its own.
- **Rules = data + code.** Per-model pipeline data (prefixes, pooling, normalisation, maximum sequence length, dimension, dtype, scorer) is plain data, loadable from TOML and validated. The shape of a chain lives in one expander per model × task kind; the registry can be overridden. A new model therefore needs only data, a new chain shape a registered expander. Rules sit behind an atomic swap, so a reload never disturbs a running expansion.
- **Operations.** Each planned operation has a description (kind and parameters) and a `materialize` flag. The parameters hold the requirements a worker filters on before claiming: resource kind, model, engine selector (`Cpu`, `Gpu`, `Either`) and, for index and scorer steps, the index. Embed and similarity steps also carry the dimension, top-k carries `k`. An operation belongs to exactly one task; the graph is implicit in the order.
- **Chains.**

  | Task | Operations (resource kind) |
  |---|---|
  | Embedding | tokenize (tokenizer) → embed (encoder) → pool (encoder, unless pooling is `none`) → L2-normalise (encoder, if configured) → write to index (index, if a target index is set) |
  | Similarity | score (scorer) → top-k (scorer); both bound to the target index; `top_k` must be positive |
  | Inference | tokenise (tokenizer) → infer (cross-encoder) |

  Similarity works on ready vectors; the query embedding is a separate embedding task. A similarity task with a cross-encoder scorer is currently rejected. A task starts with the tokenizer (embedding, inference) or the scorer (similarity).
- **Model.** Inference tasks use the model they name, otherwise the default; embedding and similarity use the default. A model without pipeline data is an error.
- **Side choice.** The planner enters only the resource kind and an engine selector; which device serves it is settled at the claim. GPU is preferred when both sides offer a kind, CPU when only it does, and neither is a missing capability. A starved kind widens the choice to `Either`. The hint is a preference between equivalent variants: it never changes the operation set, so it cannot change a task's result. Each kind exists at most once per side, so the requirements identify the engine unambiguously and no affinity to handles is needed.
- **Materialisation.** The planner knows the successor, so it decides whether a result returns to the host or stays on the device as a handle; the operation only carries the flag out. A result stays only between two GPU operations. Before a CPU step, at the end of a task or before an index write it is on the host.
- **Adapter.** The processor-facing adapter plans a task once and enqueues only the first operation. Each finished operation yields the next through its successor, with the real predecessor output as input, so a failed step never leaves queued operations without input. The final host data is converted into the facade's result type (embeddings split per text, hits, similarity report, inference scores).
- **Errors and logging.** Structured `PlannerError`: invalid config (including a wrongly routed task kind), missing capability, unknown model, unavailable scorer. Each expansion logs task kind and operation count at debug level.## Files

| File | Role |
|---|---|
| `mod.rs` | The planner: rule snapshot, reload, expansion entry |
| `rules.rs` | Rule set: pipelines per model, expander registry, TOML loading, model selection |
| `config.rs` | Per-model pipeline data (planner-side types, see root README on name clashes) |
| `expand.rs` | Task view, planned operation, expander contract and the three standard expanders |
| `offers.rs` | Which resource kinds each side offers; side choice |
| `adapter.rs` | Bridge to the processor's expander interface; result conversion |

Tests: `test/` (`mod.rs` for expansion, `adapter.rs` for the bridge).

# Resident Planner: deviations from the design

Deviations of `rag/src/resident/planner/` from `project/resident/resident_planner.md`, and design aspects not mapped in the code. Outlook items are excluded.

## D1. Pipeline config is not mapped to operation parameters

- **Design:** the planner translates task parameters into concrete operation parameters: tokenizer, model, scorer, pooling, prefix. Pipeline config per model: prefixes, pooling, normalisation, `max_seq_len`, dtype.
- **Code:** `PipelineConfig` (`config.rs`) holds `prefixes`, `max_seq_len`, `dtype`, `scorer`. They are loaded and validated, but no expander copies them into `OpParams` (`expand.rs`). Only `pooling`, `normalize` and `dim` affect the plan.

## D2. Similarity with CrossEncoder is not expanded

- **Design:** pairs (query, text from payload) → `Infer(CrossEncoder)` → scores; scope from an earlier result.
- **Code:** `SimilarityExpander` returns `PlannerError::ScorerUnavailable` for `ScorerKind::CrossEncoder(_)`. `initial_input` (`adapter.rs`) builds only a flat `F32` vector list for similarity payloads. `into_output` has no result type for this case.

## D3. Priority and task ID are not part of the planned operation

- **Design:** the operation carries priority and task ID as metadata.
- **Code:** `PlannedOp` has only `desc` and `materialize`. The task link comes from `TaskCtx` in `Chain::node` (`adapter.rs`). Priority is set nowhere in the planner module.

## D4. Query similarity chain

- **Design:** `Tokenize → Embed → Similarity(Scope) → TopK`.
- **Code:** `SimilarityExpander` plans `Similarity → TopK` on ready vectors (`initial_input` flattens `input.vectors()`). The query embedding is a separate embedding task.

## D5. `materialize` rule

- **Design:** `true` if the successor runs on the CPU, the task ends or the result is mirrored into the index; `false` if a GPU successor follows.
- **Code:** `Chain::finish` (`expand.rs`) sets `false` only if the operation and its successor both require `Gpu`. With a CPU or `Either` producer and a GPU successor, the result is materialised.

## D6. Engine selection in `requires()`

- **Design:** the planner enters only the resource kind; the claim of the subengine decides the device. The starved hint is a preference.
- **Code:** `EngineOffers::choose` (`offers.rs`) writes `Cpu`, `Gpu` or `Either` into `requires()`. GPU is preferred, and `Either` is used for a starved kind offered on both sides.