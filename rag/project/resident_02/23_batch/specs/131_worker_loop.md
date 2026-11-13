# Spec: Worker-Loop (module `worker_loop`, build order 131)

## 1. Task, restrictions and boundaries

Implement the synchronous scheduling core of a worker. After every operation it re-checks four stages in a fixed priority order and runs the first one that makes progress:

1. Process GPU results.
2. Feed GPU resources.
3. Feed CPU resources.
4. Expand new tasks through the Planner.

Restrictions:
- Use only `std`, `log`, and the project modules `gpu_completion`, `subengine_loop`, and `planner`/`processor` (only through the `ExpandStep` implementation, see §3).
- No `unsafe`, no spawning, no blocking of its own, no globals.
- Not in this module:
  - Waiting on notifications, the async loop, and timers. The caller does that and calls `run_until_idle` after each wake-up.
  - Queue structure and pushing. The Processor does that.
  - Resource lifecycle and execution. The `SubengineLoop` and the resources do that.
  - Interpreting `FinishedLane`s. The injected handler does that.
  - Any priority logic other than the fixed stage order.

## 2. Context

- `GpuCompletion::poll()` hands over finished GPU lanes. Notifications may coalesce, so the loop must drain with `poll`.
- `SubengineLoop::run_iteration(now)` is one feed pass over the own queue and the shared queue. `outcome.progress()` is true iff at least one node was claimed.
- `Planner::expand(&Task)` decomposes one task and returns `PlannedOp`s, which the Processor pushes. It may block in `Subengine::activate`. Task expansion is therefore the lowest priority, so that results and feeding are never starved by planning.
- The caller (the worker thread) waits on `SubengineNotifier`, calls `run_until_idle`, then calls `notify_iteration(progress)` using the returned `WorkerReport`.

## 3. Interface

```rust
use std::sync::Arc;
use std::time::Instant;
use gpu_completion::{GpuCompletion, FinishedLane};
use subengine_loop::{SubengineLoop, ResourceExec, IterationOutcome};
use queue_iter::QueueNode;
use resource_requirements::HasResourceRequirements;

/// One feed pass of a sub-engine. Non-blocking.
pub trait FeedStep: Send {
    fn feed(&mut self, now: Instant) -> IterationOutcome;
}
impl<N, H> FeedStep for SubengineLoop<N, H>
where N: QueueNode<Payload = H::Payload>, N::Meta: HasResourceRequirements, H: ResourceExec
{ /* calls run_iteration(now) */ }

/// Expansion of at most one new task per call (takes it from the task source,
/// calls `Planner::expand`, hands the PlannedOps to the Processor, and handles
/// an expansion error itself, e.g. by failing the task).
pub trait ExpandStep: Send {
    /// true = one task was taken and handled, whether its expansion succeeded or failed.
    /// false = no task available. May block only inside Planner::expand (sub-engine activation).
    fn expand_next(&mut self) -> bool;
}

/// Handler for a finished GPU lane. Called on the worker thread, in poll order.
pub type FinishedHandler = Box<dyn FnMut(FinishedLane) + Send>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stage { GpuResults, GpuFeed, CpuFeed, Expand }

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct WorkerReport {
    /// Number of operations executed (each = one stage run with progress).
    pub operations: usize,
    pub gpu_results: usize, // FinishedLanes handled
    pub claimed: usize,     // sum of IterationOutcome::claimed over GPU and CPU feeds
    pub expanded: usize,    // tasks taken by expand_next
}
impl WorkerReport { pub fn progress(&self) -> bool { self.operations > 0 } }

pub struct WorkerLoop { /* private */ }
// WorkerLoop: Send (not required to be Sync).

impl WorkerLoop {
    pub fn new(
        completion: Arc<GpuCompletion>,
        on_finished: FinishedHandler,
        gpu_feeds: Vec<Box<dyn FeedStep>>,
        cpu_feeds: Vec<Box<dyn FeedStep>>,
        expand: Box<dyn ExpandStep>,
    ) -> Self;

    /// Executes exactly one operation: the first stage in priority order that makes progress.
    /// None = no stage made progress (idle).
    pub fn run_once(&mut self, now: Instant) -> Option<Stage>;

    /// Repeats `run_once` (with `Instant::now()` per call) until it returns None.
    pub fn run_until_idle(&mut self) -> WorkerReport;
}
```

## 4. Behavior and invariants

**Priority and re-check**
- `run_once` checks the stages in the order `GpuResults`, `GpuFeed`, `CpuFeed`, `Expand`. It stops at the first stage that made progress. The stages below it are not run in that call.
- After every operation the next check starts again at `GpuResults`. A lower stage never runs while a higher stage still has progress.
- `run_until_idle` ends only when a full pass finds no progress in any of the four stages.

**Stage semantics**
- `GpuResults`:
  - Calls `completion.poll()` once.
  - If the result is non-empty, it passes every `FinishedLane` to `on_finished` in the returned order.
  - Progress = non-empty. This is one operation, however many lanes it returns.
  - Lanes with `result = Err(_)` are also passed on unchanged. The loop does not interpret them.
- `GpuFeed`:
  - Calls `feed(now)` on each entry of `gpu_feeds`, in vector order, and stops after the first entry whose outcome has `progress()` true.
  - Entries after that one are not called in this operation.
  - Progress = some outcome has `claimed > 0`. `inits_started`, `idle` and `evicted` alone are not progress.
  - Each executed feed call is one `run_iteration`. At least one feed is called whenever the stage is reached, so the end-of-iteration eviction and idle reporting of the `SubengineLoop` run as specified there.
- `CpuFeed`: same as `GpuFeed`, with `cpu_feeds`.
- `Expand`: calls `expand.expand_next()` once. Progress = it returns true. A task whose expansion failed counts as progress because it was consumed. This guarantees that the loop terminates.

**Counters**
- `operations` counts `run_once` calls that returned `Some`.
- `claimed` sums `IterationOutcome::claimed` over all feed calls made.
- `gpu_results` counts `FinishedLane`s handled.
- `expanded` counts `expand_next` calls that returned true.

**Idle**
- `run_once` returns `None` only if all four stages were checked without progress.
- Idle checking has the side effects of the checked calls (`poll`, `run_iteration` with its eviction and idle notices, and `expand_next`). The loop adds no other side effects.

**Concurrency and errors**
- `WorkerLoop` is owned by one worker thread. `&mut self` guarantees there is no concurrent `run_once`.
- Several `WorkerLoop`s may share the same `GpuCompletion` and queues. `GpuCompletion` hands over each lane exactly once. Exactly one claimer wins per queue node (see the `SubengineLoop` rules).
- The loop holds no lock and does not block, except in `expand_next` as described in §3.
- There are no error values. Panics of injected components propagate. Logging uses `log::trace!`/`debug!` only.

## 5. Relied-on elements

- `gpu_completion`:
  - `GpuCompletion::poll(&self) -> Vec<FinishedLane>` is non-blocking. It returns each finished lane exactly once, in submit order, whether it finished by callback or by event query.
  - `FinishedLane { lane: Lane, event: Event, tag: u64, result: Result<(), EventError>, via: TakenVia }`.
  - `GpuCompletion` is `Send + Sync`. `FinishedLane` is `Send`.
  - Notifications may coalesce, so the loop drains with `poll`. `poll` itself notifies nobody.
- `subengine_loop`:
  - `SubengineLoop<N,H>::run_iteration(&self, now: Instant) -> IterationOutcome` never blocks.
  - `IterationOutcome { claimed: usize, inits_started: usize, idle: Vec<IdleNotice>, evicted: Vec<ResourceKey> }`.
  - `progress()` is `claimed > 0`.
  - `SubengineLoop` is `Send + Sync`.
  - It makes no wake-up calls. The caller notifies via `notify_iteration(outcome.progress())`.
- `planner`: `processor::Planner::expand(&self, &Task) -> Result<Vec<PlannedOp<OpMeta, V>>, PlanError>`.
  - It never pushes into queues. The Processor pushes in the returned order, and an `Err` pushes nothing.
  - It may block only in `Subengine::activate`.
  - The `ExpandStep` implementation calls it. This module does not call the Planner directly.