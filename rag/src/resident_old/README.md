# resident

Warm embedding and similarity engine of the `rag` crate. Models, tokenizer, kernels and index mirrors are created lazily on first use and then stay resident until an idle timeout evicts them.

The module is a **substrate**: tokenise → embed → store → score. Chunking, query formulation, reranking policy, cascades and result quality belong to the layers above the facade.

## Design in brief

- **Pull, not push.** The facade only enqueues tasks. Workers expand them into operations; CPU and GPU subengines claim the operations whose requirements they can serve. Load balances itself by real progress.
- **Task ≠ operation.** A task is a user request (host data + parameters). An operation is one step on one resource (tokenizer, encoder, scorer, index) with immutable parameters and a flag saying whether the result returns to the host or stays on the device.
- **The index is the source of truth.** It lives in RAM and knows no engine, kernel or profile. GPU memory is a derived, discardable mirror kept in sync by the index manager. Stale results are an accepted variable; consistency is never bought with locks.
- **Kernels are blind to entries.** They see segments, offsets and spans and return (address, score). Mapping back to entries is the manager's job.
- **Precision is a wish.** Callers ask for 0 (fast) … 1 (precise); each subengine maps the wish to its own calibrated profiles. Bit-exact reproducibility is not a goal.
- **No silent fallbacks.** Unsupported scopes, missing capabilities and unavailable scorers fail explicitly.
- **Extensions instead of core features.** Iteration, cascades and recursion are addons that enqueue ordinary tasks through the public API.

## Layers

| Layer | Modules | Responsibility |
|---|---|---|
| Public API | `facade` | Configure, start, submit tasks, collect results |
| Orchestration | `processor`, `planner` | Queues, workers, backpressure · task → operation chain |
| Execution resources | `subengine`, `model`, `tokenize`, `kernel`, `precision` | What a CPU/GPU side owns and runs · calibration and profiles |
| Index and data placement | `index`, `segment`, `manager`, `memory` | Index contract · device-side layout · RAM ↔ VRAM mirroring · arenas |
| Primitives | `queue`, `op` | Lock-free queue and task state · operation vocabulary |
| Cross-cutting | files in this directory | Backend choice, errors, tracing, fault injection, test doubles |

Layers describe responsibility, not a strict import order. Some pairs reference each other's basic types (for example `memory` and `subengine` for events and lanes, `processor` and `planner` for the facade's task types).

## Status

Building blocks exist for every layer. Neither the processor's engine interface nor the subengine trait has a production implementation in this tree yet: the facade installs placeholders (tasks fail fast) and only tests wire fake engines. Whoever composes the real CPU/GPU engines also supplies the planner's engine offers.

## Request path

1. `facade` puts the task into the task queue and returns an awaitable handle.
2. A `processor` worker runs a prioritised pass: take over finished device work, feed devices, run host work, expand new tasks (only when hinted or the operation buffer runs low).
3. Expansion goes through the `planner`: rules per model and task kind yield an ordered operation chain, each operation with its requirements.
4. Operations land in the CPU, GPU or shared operation queue; an engine claims matching ones and runs them on a lane against leased resources.
5. A finished operation builds its successor. The last result completes the task state, which resolves the handle.

Similarity additionally: the manager resolves the scope to an entry set and makes it resident, the kernel scores segments, and hits travel back as kernel hit (address) → segment hit (slot) → manager hit (entry, outdated flag) → scored hit (with source back-reference) → facade report.

## Same name, different thing

| Name | Where | Meaning |
|---|---|---|
| Pooling | `model`, `planner` | encoder-side mode vs. pipeline-config setting |
| ScorerKind | `facade`, `planner` | task parameter vs. pipeline-config default |
| Payload | `facade`, `index` | task input bundle vs. per-entry source back-reference |
| Resident | `facade`, `manager` | the public engine handle vs. a loaded-entry record |
| DeviceHandle | `subengine`, `op` | device/context identity vs. alias of the region lease |
| ResourceState | `processor`, `subengine` | state set of a synchronous cell vs. of an async lazy-init cell |
| Lease | `op`, `subengine`, `segment` | region pin vs. resource lease vs. generation lease |
| SegmentTable | `segment`, `manager` | published generation table vs. copy-on-write table behind a root pointer |
| Access | `index`, `manager` | file access mode vs. load access pattern |
| Resolution | `segment`, `precision` | hit-resolution outcome vs. calibration-store lookup |
| Clock, Measure | `mock.rs`, `precision` | generic test doubles vs. the calibrator's own measurement abstraction |
| Engine, Subengine, IndexManager | `processor`, `subengine`, `manager` | pull-side worker interface · resource and capacity of a CPU/GPU side · mirror of one index |

## Where to look

| Task | Start in |
|---|---|
| New or changed task parameter | `facade/params.rs`, then `planner/expand.rs` if the chain changes |
| How a task becomes operations | `planner` |
| New model | `model` (spec, registry), `tokenize` (mode scheme), planner rules |
| Scheduling, backpressure, worker loop | `processor` |
| New scoring implementation | `kernel` |
| Index format, stamps, scope | `index` |
| Loading, eviction, sync to the device | `manager` |
| Arenas, budget, staging | `memory` |
| Calibration, profiles, precision matching | `precision` |

## Files in this directory

| File | Role |
|---|---|
| `mod.rs` | Declares all submodules; `mock` only with `test` or feature `mock` |
| `backend.rs` | Compute backend abstraction and the single place that decides CPU vs. CUDA; falls back to CPU if GPU init fails |
| `error.rs` | One error enum per layer plus a top-level wrapper; shared retryable/fatal classification |
| `telemetry.rs` | Tracing initialisation and the span hierarchy (task → resource, manager cycle) with stable field names |
| `failpoint.rs` | Named fault-injection points at synchronisation boundaries; inert outside `test`/`mock` |
| `mock.rs` | Conventions and shared test doubles (clock, backend, noisy measurement, loom shim for CAS protocols) |

## Conventions

- **Features:** `cuda` (real CUDA paths; the CUDA adapter modules always compile and gate toolkit code internally), `candle` (Candle adapter in `model`), `mock` (test doubles outside tests).
- **Tests:** in a `test/` directory next to the sources, one file per covered source file. Exceptions are noted in the module READMEs.
- **Boundaries:** modules depend on narrow traits so hot-path logic runs against hand-written fakes without a GPU.
- **These READMEs:** reference files and principal types, never functions. Renaming a function does not make them stale; adding, removing or moving a file does.
