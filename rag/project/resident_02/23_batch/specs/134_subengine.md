# Subengine: Technical Specification (build order 134)

## 1. Task, restrictions and boundaries

**Task.** Implement the Rust module `subengine`. A `SubengineHandle` wraps one concrete sub-engine (CPU or GPU) for the Engine-Fassade. It provides:
- the sub-engine's `planner::Subengine` implementation (forwarded unchanged),
- its feed steps: one `SubengineLoop` (build order 106) per queue role over the sub-engine's own queue and the shared queue, as `FeedStep`s for the `WorkerLoop` (build order 131),
- its device kind (GPU feeds go to the GPU stage, CPU feeds to the CPU stage),
- `LaneResource::on_finished` routing for GPU tags.

**Restrictions.** No threads, no memory, no queues of its own, no claiming. Resources, memory, streams and kernels belong to the concrete sub-engine. `std`, `log` only. No `unsafe`.
## 2. Context

- The sub-engine role (resources and memory per hardware kind, pull principle) is realised by the CPU-Subengine (build order 137) and the GPU-Subengine (build order 139). This module is the facade-side handle of one of them. It starts no threads and owns no memory.
- **Queue** (`queue::Queue`): the Processor creates three operations queues (Cpu, Gpu, Shared) with `OpMeta` as metadata. The handle takes nothing from them. Its `SubengineLoop`s (build order 106) do the claiming with `take_where`.
- **Planner** defines `Subengine` and `OpMeta`. It calls `start`, then reads `capabilities` and `offers`. For each created operation it calls `announce(&ResourceRequirements)` and then `activate()`. It only reads `is_loaded`. Planner queue assignment, which this module must be consistent with:
  - A queue is chosen only if the union of capability sets of its sub-engines satisfies `resource.capabilities()`, and at least one sub-engine assigned to that queue offers `resource.kind()`.
  - `Any` goes preferably to Shared, otherwise Gpu, otherwise Cpu.
  - `GpuOnly` goes to Gpu. `CpuOnly` goes to Cpu.
- **Speicherverwaltung** provides the arenas (Persistent, Step, Dynamic) and stream-ordered reuse. This module is its only user here.
- **Resource-Requirements** and **Operation** supply the vocabulary of `OpMeta` and its fields.
- **Engine** (pending) wires the queues and the backend, and constructs the sub-engines.

## 3. Interface

```rust
use std::sync::Arc;
use device::DeviceKind;
use gpu_completion::FinishedLane;
use planner::Subengine;
use worker_loop::FeedStep;

pub struct SubengineHandle { /* private */ }   // Send + Sync

impl SubengineHandle {
    /// `feeds` builds a fresh set of feed steps for one worker; called once per worker.
    /// `on_finished` returns false if the tag does not belong to this sub-engine.
    pub fn new(subengine: Arc<dyn Subengine>, device: DeviceKind,
               feeds: Arc<dyn Fn() -> Vec<Box<dyn FeedStep>> + Send + Sync>,
               on_finished: Arc<dyn Fn(FinishedLane) -> Result<(), FinishedLane> + Send + Sync>) -> Self;
    pub fn subengine(&self) -> &Arc<dyn Subengine>;
    pub fn device(&self) -> DeviceKind;
    pub fn feeds_for_worker(&self) -> Vec<Box<dyn FeedStep>>;
    /// Ok(()) if the lane was handled, Err(lane) if it belongs to another sub-engine.
    pub fn on_finished(&self, lane: FinishedLane) -> Result<(), FinishedLane>;
}
```

## 4. Behavior and invariants

- `feeds_for_worker` is called once per worker-pool thread by the Engine-Fassade at start. The returned feeds share the sub-engine's `SubengineLoop`s (`Arc`), so all workers pull from the same queues.
- `on_finished` forwards to the sub-engine's lane resources and returns the lane unchanged if no resource owns its tag.
- All methods take `&self` and are callable from any thread.
## 5. Relied-on elements

- `planner::Subengine` (unchanged trait), `worker_loop::FeedStep`, `gpu_completion::FinishedLane`, `device::DeviceKind`.
