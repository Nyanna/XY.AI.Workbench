# op

The vocabulary between planner and subengines: what an operation is, what it needs and what flows between operations.

## Concept

- **Operation parameters are immutable and hashable.** They serve as the pre-claim filter (does this engine qualify?) and as a cache/graph key.
- **Requirements** state resource kind, allowed engine side (CPU, GPU, either), optionally model, index and required capabilities. Affinity exists as a field but is deliberately ignored when matching.
- **The chain is implicit.** Each operation carries a successor that builds the next one from its output; an empty answer ends the chain and the output becomes the task result.
- **A value is host data or a device handle, never a tensor or raw pointer.** Whether an output is brought back to the host is decided by the planner and executed by the operation.
- **Task context** is the state shared by all operations of one task: open-operation counter, first error, cancellation. Follow-ups are counted before they are enqueued so the counter cannot reach zero early.
- **Leases** pin a device region while an operation uses it; pinned regions are not evicted.

## Files

| File | Role |
|---|---|
| `mod.rs` | Operation kinds, parameters, description, values, scored-hit results, task context, operation node, successor contract |
| `requirements.rs` | Resource kind, engine selector, model/index/device ids, affinity, requirements and their matching |
| `lease.rs` | RAII region lease and pin counter |
