# queue

Generic lock-free queue and the claim state machine of its nodes. Instantiated four times by the engine: the task queue and the CPU, GPU and shared operation queues.

## Concept

- Singly linked, append-only; append is a CAS on the tail, no lock guards the chain. Order equals age.
- Unlinking is logical: finished nodes stay in the chain and are skipped by iteration. Memory is reclaimed only when the queue drops.
- The queue is blind to meaning. Priority, resource, model and mode are metadata in the node's parameters.
- A node is claimed by exactly one winner (queued → taken → finished). Cancel only works while a node is still queued. The result cell is set once, by the claim winner.
- Operation queues reuse the same building block; their completion is tracked by the task's operation counter, not by the node result.

## Files

| File | Role |
|---|---|
| `mod.rs` | Module overview and re-exports |
| `queue.rs` | The queue: sentinel head, CAS append, iteration over live nodes |
| `node.rs` | Chain node (parameters, payload, link) |
| `state.rs` | Claim state machine and the result cell |
| `task.rs` | Notes on how task and operation queues are instantiated (no code of its own) |
