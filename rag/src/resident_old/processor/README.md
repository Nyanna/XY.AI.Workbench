# processor

Orchestration: owns the worker threads, the operation queues, wake-ups, backpressure and per-task completion. It decides *when* work happens, not *what* the work is.

## Concept

- **One worker pool, two sides.** Workers belong to the CPU or the GPU side; their counts are configured per side. They park on one wake-up per side, woken by submits, operation enqueues and GPU lane signals. A blocking operation is the natural backpressure signal and gets no special handling. Each runs a prioritised pass, re-evaluated after every operation: (1) take over finished device work, (2) feed devices (sync indexes, launch on a free lane), (3) claim and run host work, (4) expand new tasks. GPU precedence limits itself: feeding needs a free lane, expansion needs a hint or an empty queue.
- **The two sides are interfaces.** An engine interface (three entry points: take over, feed, claim-and-run) is implemented by the CPU/GPU engines; an expander interface turns a task into an operation chain and is implemented by the planner's adapter. The processor knows neither Candle nor kernels.
- **Backpressure by starvation.** An engine iteration that finds a free resource but nothing to pick marks a decaying per-kind hint. Hints are a set per side, not a queue; a pick of the kind or an expansion that used the hint clears them, otherwise they expire after a TTL. Expansion runs only for a hint or when the queued operations are below the configured minimum, and prefers a task whose first operation needs the hinted resource. No fixed fill threshold.
- **Queues.** CPU, GPU and shared operation queues. Shared is wired in but never filled by routing; an engine checks its own queue first, then shared.
- **Task completion.** A counter of open operations; the last one completes the task. The first error cancels the task, remaining operations are dropped at claim. A deadline fails overdue tasks with a timeout; it is checked when one of the task's operations is claimed or enqueues follow-ups. A device handle at claim is checked only by a debug assertion (region check set on the hub).
- **GPU completion.** A lane board tracks each lane (idle → reserved → launched → completing). Completion arrives by a host callback (push, no CUDA calls inside) or by polling an event in the take-over step, which covers callbacks that never fire; both race on a CAS, so each lane is taken over exactly once.
- **Resource cell.** A synchronous CAS lifecycle cell (uninit → loading → ready → evicting; a failed load goes to backoff and may retry after a delay) with in-flight count, pins and TTL eviction at no operation in flight, no pins and an expired TTL.

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

## Deviations

1. Worker pool split
   - Design: one pool sized to CPU cores for the whole engine; any free worker processes GPU completions.
   - Code: separate `cpu_workers` / `gpu_workers` (WorkerConfig); each worker is bound to one side/engine for life. Default is 1/0. No derivation from core count.

2. Starved hint lacks params
   - Design: `Starved(resource_kind, params)`.
   - Code: only a per-kind bit (StarvedSet/StarvedSnapshot); no params are stored or passed to the expander.

3. Hint expiry rules
   - Design: hints expire on pick or occupied resource.
   - Code: cleared on pick (Hub::claim) and after any expansion that used the hint (clear_all). No clearing on "resource occupied" within this package.

4. Expansion trigger
   - Design: hint or empty operation queue.
   - Code: hint or total queued ops (CPU+GPU+shared, not per side) < `min_op_buffer` (default 2).

5. Task timeout coverage
   - Design: timeout aborts blocked tasks and releases leases.
   - Code: the deadline is only checked on claim (Counted::claim) and in complete_op with follow-ups. It is not applied to tasks still in the task queue, to a running (blocked) operation, or to operations stuck unclaimed (e.g. resource Loading). Lease release is not performed in this package.

6. Resource states
   - Design: Uninit|Loading|Ready|Backoff, internally Unloaded→Loading→Resident→Leased(n)→Idle→Evicting, plus Failed.
   - Code (ResourceSlot): Uninit, Loading, Ready, Backoff, Evicting only. Leases are a pin counter and in_flight is a counter, not states. No `Failed` state.

7. GPU init failure
   - Design: mark GPU as unavailable (retryable).
   - Code: GPU availability is fixed at start (`gpu_workers > 0` and a gpu engine given). `EngineUnavailable` is returned at enqueue; there is no runtime mark and no retry.

8. First-error result
   - Design: task gets `Cancelled` and `Err`.
   - Code: TaskRun::fail completes the task with `TaskError::Engine(err)`; remaining ops are skipped at claim. Handle release is not done in this package.

9. Eviction TTL ("VRAM much longer than RAM")
   - Code: ResourceSlot::try_evict takes ttl as a parameter; no distinction in the processor package.

10. Span per task-id and resource
    - Design: span per task ID and resource.
    - Code: the task span exists only around expansion (telemetry::task_span) and a worker span per thread. No resource span in this package.

11. Unused config
    - `WorkerConfig.candle_rayon_threads` is stored but not used in this package.

12. Unused error variants
    - OrchestrationError::{Cancelled, Backoff, CompletionLost, Shutdown} are not raised anywhere in this package.