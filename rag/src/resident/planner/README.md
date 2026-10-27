# planner

Stateless expansion of a task into an ordered chain of operations. Same inputs, same operations; no queues, no workers, no notification.

## Concept

- **Inputs:** the task's parameters (never its data), the rules, the engine offers (which resource kinds the CPU and GPU side can serve) and the current starved hint.
- **Rules = data + code.** Per-model pipeline data (prefixes, pooling, normalisation, maximum sequence length, dimension, dtype, scorer) can be loaded from TOML and is validated. Logic lives in one expander per model × task kind; the registry can be overridden. Rules sit behind an atomic swap, so a reload never disturbs a running expansion.
- **Chains.** Embedding: tokenise → embed → pool (unless disabled) → L2-normalise (if configured) → write to index (if a target index is set). Similarity: score → top-k. Inference: tokenise → infer. A similarity task with a cross-encoder scorer is currently rejected.
- **Side choice.** GPU preferred when both sides offer a kind; CPU when only it does. A starved hint widens the choice to either side. The hint only picks between equivalent variants; it never changes the operation set.
- **Materialisation.** A result stays on the device only when the next operation runs on the GPU; otherwise it returns to the host.
- **Adapter.** The processor-facing adapter plans a task once, hands out the chain one operation at a time through successors and converts the final host data into the facade's result type (similarity → report).

## Files

| File | Role |
|---|---|
| `mod.rs` | The planner: rule snapshot, reload, expansion entry |
| `rules.rs` | Rule set: pipelines per model, expander registry, TOML loading, model selection |
| `config.rs` | Per-model pipeline data (planner-side types, see root README on name clashes) |
| `expand.rs` | Task view, planned operation, expander contract and the three standard expanders |
| `offers.rs` | Which resource kinds each side offers; side choice |
| `adapter.rs` | Bridge to the processor's expander interface; result conversion |

Tests: `test/` (`mod.rs` for expansion, `adapter.rs` for the bridge).
