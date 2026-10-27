# processor

Orchestration: owns the worker threads, the operation queues, wake-ups, backpressure and per-task completion. It decides *when* work happens, not *what* the work is.

## Concept

- **One worker pool, two sides.** Workers belong to the CPU or the GPU side. Each runs a prioritised pass, re-evaluated after every operation: (1) take over finished device work, (2) feed devices (sync indexes, launch on a free lane), (3) claim and run host work, (4) expand new tasks. GPU precedence limits itself: feeding needs a free lane, expansion needs a hint or an empty queue.
- **The two sides are interfaces.** An engine interface (three entry points: take over, feed, claim-and-run) is implemented by the CPU/GPU engines; an expander interface turns a task into an operation chain and is implemented by the planner's adapter. The processor knows neither Candle nor kernels.
- **Backpressure by starvation.** An engine iteration that finds a free resource but nothing to pick marks a decaying per-kind hint. Expansion runs only for a hint or when the operation buffer is below its minimum. No fixed fill threshold.
- **Queues.** CPU, GPU and shared operation queues. Shared is wired in but never filled by routing; an engine checks its own queue first, then shared.
- **Task completion.** A counter of open operations; the last one completes the task. The first error cancels the task, remaining operations are dropped at claim. A deadline fails overdue tasks.
- **GPU completion.** A lane board tracks each lane (idle → reserved → launched → completing). Completion arrives by a host callback (push, no CUDA calls inside) or by polling an event; each lane is taken over exactly once.
- **Resource cell.** A synchronous CAS lifecycle cell (unloaded → loading → ready → evicting) with in-flight count, pins and TTL eviction.

## Files

| File | Role |
|---|---|
| `mod.rs` | Overview and re-exports |
| `hub.rs` | Shared coordination point handed to engines: engine and expander interfaces, wake-ups, claim, finish/fail, expansion trigger |
| `worker.rs` | Processor start/shutdown and the prioritised worker pass |
| `ops.rs` | Per-task run bookkeeping, operation jobs, the three operation queues and routing |
| `starved.rs` | Decaying per-kind demand hints |
| `resource.rs` | Synchronous lifecycle cell for lazily initialised resources |
| `completion.rs` | Lane board and completion-detection interface, manual double for tests |
| `completion_cuda.rs` | Callback-based and poll-based completion on CUDA streams (feature `cuda`) |
| `config.rs` | Worker counts, minimum operation buffer, hint TTL, task timeout, priority and thread limits |
| `error.rs` | Orchestration errors |

Tests: `test/` per file; `tests.rs` holds scenario tests of the whole processor with fake engines.
