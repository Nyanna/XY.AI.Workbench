# Spec: Lane und Batching (module `lane_batching`, build order 117)

## 1. Task, restrictions and boundaries

This module is the resource side of a sub-engine. `LaneResource` is the `ResourceExec` handle that the Subengine-Loop calls. It does the following:
- It owns N lanes. Each lane is exclusive use of one `device::Lane`. It starts no threads: work runs on the worker-pool thread that calls `execute`.
- It assigns each handed-over operation to a lane by the lane's dynamic throughput. Faster lanes get more work.
- It aggregates the operation with compatible operations from the queues into one batch. Encoder batches are sized by token budget. Packing and length-bucketing use `batch_building`.
- It runs the batch through a caller-supplied `BatchRunner`. A CPU batch finishes before `execute` returns. A GPU batch is launched and handed to `GpuCompletion`; `on_finished` finishes it later. Each finished batch is reported with one `notify_result()`.

Out of scope:
- Queue structure, enqueueing and the loop. The loop is the `subengine_loop::SubengineLoop` that calls `execute`.
- Resource lifecycle, loading and eviction.
- Tensors, inference and kernels. They live in the `BatchRunner`.
- Writing task results. The runner does that through the step completion (`StepPayload::finish` or `fail`, build order 66a).
- Deadlines (`batch_size_for`, `ClassDeadline`) and priority interpretation.
- Any inter-lane handover other than through the queue.

Restrictions: no `unsafe`; no threads; `log` and the dependencies named in §5 only; `has_capacity` never blocks; `execute` blocks only while a CPU batch runs or while it waits for a lane lock.

## 2. Context

- The lane resource starts no threads. A batch is formed and run on the worker-pool thread that calls `execute`. CPU batches finish there. GPU batches are launched there, and a free worker finishes them after the completion notification (stage `GpuResults` of build order 131).
- The loop claims an entry with `take_where` and calls `handle.execute(claim, lease)` on the worker thread. `execute` is infallible.
- Before it claims, the loop checks `has_capacity()`.
- The seed operation was already claimed by the loop. This module only claims additional entries, with `take_where` on the own and the shared queue.
- Queue nodes carry opaque metadata `M`. The module cannot see a payload before claiming it, so `M` must expose the batch key and the row lengths (`BatchMeta`).
- The payload cannot give the module its queue position. "Forward through the list" is therefore implemented as repeated `take_where` (oldest first) on the own queue, then on the shared queue. The seed is no longer Pending, so it is never returned again.
- `BatchMeta` is part of the queue metadata. Under staged release (build order 126) a step is pushed only after its predecessor finished, so the pusher fills the batch key and the row lengths from the payload before the push.
- Other lanes get work only by pulling from the queue, whether in the loop or in the batch scan. Nothing is handed from lane to lane.

## 3. Interface

```rust
use std::sync::Arc;
use device::{DeviceHandle, Lane};
use queue::{Queue, Claim};
use gpu_completion::{GpuCompletion, FinishedLane};
use device::EventError;
use resource_requirements::{HasResourceRequirements, ResourceKind};
use resource_lifecycle::Lease;
use subengine_loop::{ResourceExec, PullFilter};
use notification_topology::SubengineNotifier;
use auslastungsmeldung::{LoadReport, LoadReporting};
use batch_building::Batch;

/// Compatibility class: same index or model (and operation kind) that can/must share a batch.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BatchKey(pub String);

/// Implemented by the payload (the claimed work).
pub trait BatchPayload: Send + 'static {
    /// None = not batchable: runs alone, no scan.
    fn batch_key(&self) -> Option<BatchKey>;
    /// Token sequences (rows) of this task; empty for non-token operations.
    fn sequences(&self) -> &[Vec<u32>];
}

/// Implemented by the queue metadata (readable without claiming).
pub trait BatchMeta {
    fn batch_key(&self) -> Option<BatchKey>;          // None = never joins a batch
    fn seq_lengths(&self) -> &[usize];                // untruncated length per row; must equal the payload's sequences()
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BatchLimit {
    /// Encoder: rows * padded seq_len <= budget. budget >= 1, max_seq_len >= 1.
    Tokens { token_budget: usize, max_seq_len: usize, pad_token_id: u32 },
    /// Non-encoder: at most n tasks per batch. n >= 1.
    Tasks(usize),
}

/// Row range of one task in the concatenated sequence list of the batch (in task order).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RowSpan { pub first_row: usize, pub rows: usize }

pub struct ExecBatch<P> {
    /// Seed first, then joined tasks in claim order.
    pub tasks: Vec<P>,
    /// spans[i] belongs to tasks[i]; first_row = sum of rows of earlier tasks.
    pub spans: Vec<RowSpan>,
    /// Some for BatchLimit::Tokens (from batch_building::build_batches over the concatenated
    /// sequences); None for Tasks. Batch::positions() are indices into the concatenated list,
    /// so spans map rows back to tasks.
    pub batches: Option<Vec<Batch>>,
}

/// Result of `BatchRunner::run`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Launch { Finished, Pending }

/// Executes a batch on the lane (incl. stream use). Infallible from this module's view:
/// the runner finishes or fails every task (`StepPayload::finish`/`fail`).
pub trait BatchRunner<P>: Send + Sync + 'static {
    /// CPU: runs synchronously, finishes every task, returns `Finished`.
    /// GPU: launches the work on the lane's stream and returns `Pending`.
    fn run(&self, lane: &mut Lane, batch: &mut ExecBatch<P>) -> Launch;
    /// GPU only, after the lane's event completed: reads the results and finishes
    /// or fails every task (`result` is the event outcome).
    fn collect(&self, lane: &mut Lane, batch: ExecBatch<P>, result: Result<(), EventError>);
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LaneSetError { NoLanes, InvalidLimit, Spawn(String) }
impl std::fmt::Display for LaneSetError {}
impl std::error::Error for LaneSetError {}

pub struct LaneResourceConfig<M, P> {
    pub own: Arc<Queue<M, P, (), ()>>,
    pub shared: Option<Arc<Queue<M, P, (), ()>>>,
    pub filter: PullFilter<M>,
    /// Some for GPU lane resources: launched batches are tracked here.
    pub completion: Option<Arc<GpuCompletion>>,
    pub limit: BatchLimit,
    pub notifier: SubengineNotifier,
    pub kind: ResourceKind,
    pub device: DeviceHandle,
}

pub struct LaneResource<M, P> { /* private */ }

impl<M, P> LaneResource<M, P>
where P: BatchPayload, M: BatchMeta + HasResourceRequirements + Send + Sync + 'static {
    /// Takes the lanes. Starts no threads.
    pub fn new(cfg: LaneResourceConfig<M, P>, lanes: Vec<Lane>,
               runner: Arc<dyn BatchRunner<P>>) -> Result<Arc<Self>, LaneSetError>;
    /// Called by the `GpuResults` stage for a finished lane whose tag belongs to this resource.
    /// Returns false if the tag is unknown here.
    pub fn on_finished(&self, finished: FinishedLane) -> bool;
    pub fn lane_count(&self) -> usize;
    /// Lanes with neither a running nor a pending job.
    pub fn idle_lanes(&self) -> usize;
}

impl<M, P> ResourceExec for LaneResource<M, P> where /* same bounds */ {
    type Entry = Claim<M, P, (), ()>;
    fn has_capacity(&self) -> bool;
    fn execute(&self, entry: Claim<M, P, (), ()>, lease: Lease<Self>);
}
impl<M, P> LoadReporting for LaneResource<M, P> where /* same bounds */ { fn report(&self) -> LoadReport; }
// LaneResource<M, P>: Send + Sync. Dropping it drops batches still in flight only after `GpuCompletion` returned their lanes.
```

## 4. Behavior and invariants

**Construction**
- `lanes` empty gives `Err(NoLanes)`.
- `Tokens` with `token_budget == 0` or `max_seq_len == 0`, or `Tasks(0)`, gives `Err(InvalidLimit)`.
- `Spawn` is not used (no threads).
- Each lane gets its own `LoadMeter::new(kind, device, 1)`.

**Capacity and lane choice**
- `has_capacity()` is `idle_lanes() > 0`. It is non-blocking. A lane is idle when it runs no batch and has no batch in flight.
- `execute` takes, among idle lanes, the one with the highest `LoadMeter::throughput()` (ties: lowest index). If no lane is idle (a race after `has_capacity`), it waits for the lock of the lane with the highest throughput. The claim is never dropped or rejected.
- The lane choice is re-evaluated on every `execute`, so faster lanes get more work.

**Batch formation** (inside `execute`, on the calling worker thread)
1. The batch starts with the seed payload, taken from the seed claim with `take_payload` (`None`: drop the claim and the lease, nothing runs). The seed is always accepted, even if it alone exceeds the budget.
2. Take `key = seed.batch_key()`. If it is `None`, or `Tasks(1)` is set, run the seed alone with no scan.
3. For the own queue, then the shared queue: repeat `take_where(|m| filter(m) && m.batch_key() == Some(key) && fits(m))`, where `fits` reads the current batch state only. After each `Some(claim)`, take the payload (`None`: drop the claim and continue), append it, and update the batch state from `claim.meta()`. Stop on this queue when `take_where` returns `None`. A rejected entry never ends the scan.
4. The scan ends at the end of both queues, or earlier when `Tasks(n)` is reached.
5. Fit rules:
   - `Tasks(n)`: it fits if `tasks < n`.
   - `Tokens`: let `p = max over all rows of padded_len(len, max_seq_len)`, including the candidate's rows. It fits if `(rows_total + candidate_rows) * p <= token_budget`. A candidate with 0 rows always fits.
6. Never mix keys. Never claim an entry that was not accepted by `filter` and `key`. Keep the claims of all tasks of the batch until the batch is finished.
7. `spans` record, per task, `first_row` (sum of rows of earlier tasks) and `rows = sequences().len()`.
8. `Tokens` mode:
   - Build `batches = build_batches(concatenated sequences in task order, BatchConfig{token_budget, max_seq_len, pad_token_id})`.
   - The result may hold several `Batch`es when the seed alone exceeds the budget.
   - If `build_batches` returns `Err`, which cannot happen after validation, log `error!` and use `Some(vec![])`.
   - `Tasks` mode: `batches = None`.
9. `Tokens` mode:
   - Rows are never split across tasks.
   - Length-bucketing and padding come only from `batch_building`.
   - This module does no padding of its own.

**Execution and notification**
- `execute` calls `runner.run(&mut lane, &mut batch)` with exclusive `&mut Lane`. A panic in `run` or `collect` is caught (`catch_unwind`) and logged with `error!`; the batch's claims are dropped (`Abandoned`) and its lease is released.
- `Launch::Finished` (CPU): in this order: `lane_meter.record(entries, elapsed)` (entries = total rows, or the number of tasks if total rows is 0; skipped if `elapsed` is zero); complete every claim with `Ok(())` in task order; drop the lease; mark the lane idle; `notifier.notify_result()` once.
- `Launch::Pending` (GPU): record the lane's event, store `{batch, claims, lease, start}` under a new tag, hand lane and event to `completion.submit(lane, event, tag)` and return. The lane counts as busy.
- `on_finished(finished)`: take the record of `finished.tag`, call `runner.collect(&mut finished.lane, batch, finished.result)`, then the same order as for `Finished` (meter, claims, lease, lane back and idle, `notify_result`).
- One lease covers the whole batch, because joined tasks use the same resource. It is held until the batch is finished, so device data stays alive while the device reads it.

**Report**
- `report()` returns `LoadReport { kind, device, capacity: idle_lanes(), throughput: sum of lane throughputs }`.
- Throughput is finite and `>= 0`.

**Concurrency**
- All methods take `&self` and are callable from any thread. Several loops may call `execute` at the same time.
- Exactly one claimer wins per queue entry (`take_where`). A concurrent loss in the scan is a silent skip.
- No lock is held while scanning the queues or while running `run`.
- Lanes interact only through the queue.
- There are no pending jobs and no own threads.

**Errors**
- Only `LaneSetError` from `new`. `execute`, `has_capacity` and `report` never fail or panic on their own.

## 5. Relied-on elements

- `subengine_loop::ResourceExec: Sized + Send + Sync + 'static { type Entry: Send + 'static; fn has_capacity(&self) -> bool; fn execute(&self, entry: Self::Entry, lease: Lease<Self>); }`. CPU `execute` runs the work synchronously, GPU `execute` only launches it; the lease is held until the work is finished.
- `subengine_loop::PullFilter<M> = Arc<dyn Fn(&M) -> bool + Send + Sync>`.
- `queue`: `Queue<M,P,(),()>::take_where`, `Claim::{meta, take_payload, complete}`; dropping a `Claim` makes the entry `Abandoned`.
- `gpu_completion`: `GpuCompletion::submit(lane, event, tag) -> CompletionCallback`; `FinishedLane { lane, event, tag, result, via }`.
- `resource_requirements`: `HasResourceRequirements`, `ResourceKind` (`Copy`).
- `resource_lifecycle::Lease<H>`: a plain drop ends the use.
- `notification_topology::SubengineNotifier::notify_result(&self)`: non-blocking; one call per batch is enough.
- `auslastungsmeldung`:
  - `LoadMeter::{new(kind, device, capacity), record(entries, elapsed), throughput()}`.
  - `LoadReport { kind, device, capacity, throughput }`.
  - `LoadReporting::report`.
- `device`:
  - `Lane` is `Send + Sync`, with `&mut` for exclusive state.
  - `DeviceHandle` is `Copy`.
- `batch_building`:
  - `build_batches(&[Vec<u32>], &BatchConfig) -> Result<Vec<Batch>, BatchError>`.
  - `BatchConfig { token_budget, max_seq_len, pad_token_id }`.
  - `padded_len(len, max_seq_len) -> usize`.
  - `Batch::positions()` gives original input indices.