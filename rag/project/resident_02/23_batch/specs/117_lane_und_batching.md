# Spec: Lane und Batching (module `lane_batching`, build order 117)

## 1. Task, restrictions and boundaries

This module is the resource side of a sub-engine. `LaneResource` is the `ResourceExec` handle that the Subengine-Loop calls. It does the following:
- It owns N lanes. Each lane has a worker thread and exclusive use of one `device::Lane`.
- It assigns each handed-over operation to a lane by the lane's dynamic throughput. Faster lanes get more work.
- It aggregates the operation with compatible operations from the queues into one batch. Encoder batches are sized by token budget. Packing and length-bucketing use `batch_building`.
- It runs the batch through a caller-supplied `BatchRunner`, then reports the result with one `notify_result()`.

Out of scope:
- Queue structure, enqueueing and the loop. The loop is the `subengine_loop::SubengineLoop` that calls `execute`.
- Resource lifecycle, loading and eviction.
- Tensors, inference and kernels. They live in the `BatchRunner`.
- Writing task results. The runner does that (`set_output`, `advance`, `complete`).
- Deadlines (`batch_size_for`, `ClassDeadline`) and priority interpretation.
- Any inter-lane handover other than through the queue.

Restrictions: no `unsafe`; `std` threads, `log` and the dependencies named in §5 only; `execute` and `has_capacity` never block.

## 2. Context

- The loop claims a node with `claim_entry` and calls `handle.execute(payload, lease)`. `execute` is non-blocking and infallible.
- Before it claims, the loop checks `has_capacity()`.
- The seed operation was already claimed by the loop. This module only claims additional entries, directly with `claim_entry` on queue items.
- Queue nodes carry opaque metadata `M`. The module cannot see a payload before claiming it, so `M` must expose the batch key and the row lengths (`BatchMeta`).
- The payload cannot give the module its queue position. "Forward through the list" is therefore implemented as an oldest-first scan of the active entries of the own queue, then the shared queue. The seed is already `Claimed`, so the scan skips it.
- Other lanes get work only by pulling from the queue, whether in the loop or in the batch scan. Nothing is handed from lane to lane.

## 3. Interface

```rust
use std::sync::Arc;
use device::{DeviceHandle, Lane};
use queue_iter::QueueNode;
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

/// Executes a batch on the lane (incl. stream use). Infallible from this module's view:
/// the runner must complete or fail every task itself.
pub trait BatchRunner<P>: Send + Sync + 'static {
    fn run(&self, lane: &mut Lane, batch: ExecBatch<P>);
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LaneSetError { NoLanes, InvalidLimit, Spawn(String) }
impl std::fmt::Display for LaneSetError {}
impl std::error::Error for LaneSetError {}

pub struct LaneResourceConfig<N: QueueNode> {
    pub own_head: Arc<N>,
    pub shared_head: Arc<N>,
    pub filter: PullFilter<N::Meta>,
    pub limit: BatchLimit,
    pub notifier: SubengineNotifier,
    pub kind: ResourceKind,
    pub device: DeviceHandle,
}

pub struct LaneResource<N: QueueNode> { /* private */ }

impl<N> LaneResource<N>
where N: QueueNode, N::Payload: BatchPayload, N::Meta: BatchMeta + HasResourceRequirements {
    /// Spawns one worker thread per lane.
    pub fn new(cfg: LaneResourceConfig<N>, lanes: Vec<Lane>,
               runner: Arc<dyn BatchRunner<N::Payload>>) -> Result<Arc<Self>, LaneSetError>;
    pub fn lane_count(&self) -> usize;
    /// Lanes with neither a running nor a pending job.
    pub fn idle_lanes(&self) -> usize;
}

impl<N> ResourceExec for LaneResource<N> where /* same bounds */ {
    type Payload = N::Payload;
    fn has_capacity(&self) -> bool;
    fn execute(&self, payload: N::Payload, lease: Lease<Self>);
}
impl<N> LoadReporting for LaneResource<N> where /* same bounds */ { fn report(&self) -> LoadReport; }
// LaneResource<N>: Send + Sync. Dropping it lets workers finish pending jobs, then stops and joins them.
```

## 4. Behavior and invariants

**Construction**
- `lanes` empty gives `Err(NoLanes)`.
- `Tokens` with `token_budget == 0` or `max_seq_len == 0`, or `Tasks(0)`, gives `Err(InvalidLimit)`.
- A thread that cannot be spawned gives `Err(Spawn)`.
- Each lane gets its own `LoadMeter::new(kind, device, 1)`.

**Capacity and lane choice**
- `has_capacity()` is `idle_lanes() > 0`. It is non-blocking.
- `execute` picks a lane by the following rules:
  1. Among idle lanes, take the one with the highest `LoadMeter::throughput()`. Ties go to the lowest index.
  2. If no lane is idle (a race after `has_capacity`), take the lane with the fewest pending jobs. Ties go to the highest throughput. The job waits in that lane's FIFO mailbox, and no job is ever dropped or rejected.
- `execute` only enqueues the job to the lane worker and wakes it. It never runs work on the caller's thread and never blocks.
- A lane runs one job at a time, in mailbox order.
- The lane choice is dynamic. It is re-evaluated on every `execute` from the current throughput, so faster lanes get more work.

**Batch formation** (on the lane worker, per job)
1. The batch starts with the seed payload. The seed is always accepted, even if it alone exceeds the budget.
2. Take `key = seed.batch_key()`. If it is `None`, or `Tasks(1)` is set, run the seed alone with no scan.
3. Scan `iter_active(own_head)`, then `iter_active(shared_head)`, oldest to youngest. For each item, in this order:
   - skip it if `state() != Enqueued`;
   - skip it if `!filter(meta)`;
   - skip it if `meta.batch_key() != Some(key)`;
   - skip it if it does not fit (see below). A skipped item never ends the scan, so later smaller items may still join;
   - call `claim_entry(item.entry())`. On `Err(_)`, skip without error. On `Ok(payload)`, append it.
4. The scan ends at the end of both queues, or earlier when `Tasks(n)` is reached.
5. Fit rules:
   - `Tasks(n)`: it fits if `tasks < n`.
   - `Tokens`: let `p = max over all rows of padded_len(len, max_seq_len)`, including the candidate's rows. It fits if `(rows_total + candidate_rows) * p <= token_budget`. A candidate with 0 rows always fits.
6. Never mix keys. Never claim an entry that was not accepted by `filter` and `key`.
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
- The lane worker calls `runner.run(&mut lane, batch)` with exclusive `&mut Lane`. The Lane's stream is used only through the runner.
- A panic in `run` is caught (`catch_unwind`) and logged with `error!`. The worker survives and continues with the next job.
- After `run` returns, the following happens in this order:
  1. `lane_meter.record(entries, elapsed)`, where `entries` is the total rows, or the number of tasks if total rows is 0, and `elapsed` is the wall time of `run`. This is skipped if `elapsed` is zero.
  2. Drop the lease. One lease covers the whole batch, because joined tasks use the same resource.
  3. Mark the lane idle.
  4. Call `notifier.notify_result()` exactly once per batch, not per task.
- The notification is sent even after a panic.

**Report**
- `report()` returns `LoadReport { kind, device, capacity: idle_lanes(), throughput: sum of lane throughputs }`.
- Throughput is finite and `>= 0`.

**Concurrency**
- All methods take `&self` and are callable from any thread. Several loops may call `execute` at the same time.
- Exactly one claimer wins per queue entry (`claim_entry`). A concurrent loss in the scan is a silent skip.
- No lock is held while scanning the queues or while running `run`.
- Lanes interact only through the queue.
- On drop, pending jobs are still run, and then the workers are joined.

**Errors**
- Only `LaneSetError` from `new`. `execute`, `has_capacity` and `report` never fail or panic on their own.

## 5. Relied-on elements

- `subengine_loop::ResourceExec: Sized + Send + Sync + 'static { type Payload: Send + 'static; fn has_capacity(&self) -> bool; fn execute(&self, payload: Self::Payload, lease: Lease<Self>); }`. `execute` is non-blocking and the lease is held until execution ends.
- `subengine_loop::PullFilter<M> = Arc<dyn Fn(&M) -> bool + Send + Sync>`.
- `queue_iter`:
  - `QueueNode { type Payload; type Output; type Error; type Meta; … }`.
  - `iter_active(&Arc<N>) -> ActiveIter<N>` yields `QueueItem<N>` oldest first, non-blocking.
  - `QueueItem::{entry(), meta(), state()}`.
- `queue_state`:
  - `claim_entry(&EntryState) -> Result<P, ClaimError>`. A failure leaves the state unchanged, and a success moves the payload out exactly once.
  - `QueueState::Enqueued`.
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