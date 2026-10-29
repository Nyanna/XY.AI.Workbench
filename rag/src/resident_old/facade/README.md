# facade

Public API of the engine: configure, start, submit tasks, collect results.

## Concept

- **Two-step start.** A builder collects configuration (model, worker counts, planner, CPU engine, optional GPU engine); init only starts the worker threads. Model and device resources are created by the first task, so an idle engine holds nothing. GPU workers start only if a GPU engine is supplied.
- **Three task kinds:** embedding (text → vectors, optionally written to an index), similarity (vectors against an index scope → scored hits), inference (query/chunk pairs for a cross-encoder). Rerank is not a task kind; it is a similarity task with another scorer on a narrowed scope.
- **Parameters** are typed per kind and carry a precision wish (default: precise) plus an open, type-keyed extension map that travels unchanged through queue and planner. Addons use only this and the normal submit path, so the tasks they derive are indistinguishable from application tasks. `test/addon.rs` is the reference cascade.
- **Results** arrive through an awaitable handle. Dropping an unfinished handle cancels the task if no worker has claimed it. The blocking wrapper owns a private runtime and must not be used from inside another tokio runtime.
- **Similarity returns facts, not decisions.** Raw scores plus descriptive statistics (count, gaps, spread, entropy, elbow) let the caller decide on follow-up tasks. Thresholds stay outside.
- **Indexes are registered, not loaded.** The first task that touches an index triggers loading.
- **Fail fast without wiring.** Without a supplied planner or engine, placeholders make tasks fail instead of waiting forever.
- Errors reaching waiters are cloneable so one outcome can serve several waiters.

## Files

| File | Role |
|---|---|
| `mod.rs` | Public surface and re-exports |
| `resident.rs` | Configuration, builder, the engine handle, index registry, shutdown, placeholders |
| `blocking.rs` | Synchronous wrapper around the engine handle |
| `task.rs` | Task kinds, task metadata, payload, task queue type |
| `params.rs` | Inputs, per-kind parameters, result type |
| `handle.rs` | Awaitable task handle with cancel-on-drop |
| `extensions.rs` | Type-keyed parameter extensions |
| `report.rs` | Similarity report: ranked hits and derived score statistics |
| `error.rs` | Task-level error delivered to waiters |

Tests: `test/` (`addon.rs` doubles as documentation of the extension mechanism).
