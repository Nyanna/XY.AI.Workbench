//! Lane assignment and batch aggregation: the resource handle called by the sub-engine loop.
use std::collections::HashMap;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Condvar, Mutex, PoisonError, TryLockError};
use std::time::Instant;
use log::{error, warn};
use crate::resident::_d::_c::_b::_a::_9::a::gpu_completion::{
    FinishedLane, GpuCompletion,
};
use crate::resident::_d::_c::_b::_a::_9::_8::_7::a::notification_topology::SubengineNotifier;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::a::resource_lifecycle::Lease2;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::a::load_meter::{
    LoadMeter, LoadReport, LoadReporting,
};
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::a::resource_requirements::{
    HasResourceRequirements, ResourceKind,
};
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::_1::batching::{
    build_batches, padded_len, Batch, BatchConfig,
};
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::_1::device_lane_event::{
    DeviceHandle, EventError, Lane, LaneError,
};
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::_1::queue::{
    Claim2, Queue,
};
use crate::resident::_d::_c::_b::_a::_9::_8::subengine_loop::{
    PullFilter, ResourceExec,
};
/// Tags are unique per process so that a tag unknown to one resource is never known to another.
static NEXT_TAG: AtomicU64 = AtomicU64::new(1);
/// Compatibility class: same index or model (and operation kind) that can share a batch.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BatchKey(pub String);
/// Implemented by the payload (the claimed work).
pub trait BatchPayload: Send + 'static {
    /// `None`: not batchable, runs alone.
    fn batch_key(&self) -> Option<BatchKey>;
    /// Token sequences (rows) of this task; empty for non-token operations.
    fn sequences(&self) -> &[Vec<u32>];
}
/// Implemented by the queue metadata (readable without claiming).
pub trait BatchMeta {
    /// `None`: never joins a batch.
    fn batch_key(&self) -> Option<BatchKey>;
    /// Untruncated length per row; must equal the payload's `sequences()`.
    fn seq_lengths(&self) -> &[usize];
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BatchLimit {
    /// Encoder: rows * padded sequence length <= budget.
    Tokens { token_budget: usize, max_seq_len: usize, pad_token_id: u32 },
    /// Non-encoder: at most `n` tasks per batch.
    Tasks(usize),
}
/// Row range of one task in the concatenated sequence list of the batch.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RowSpan {
    pub first_row: usize,
    pub rows: usize,
}
pub struct ExecBatch<P> {
    /// Seed first, then joined tasks in claim order.
    pub tasks: Vec<P>,
    /// `spans[i]` belongs to `tasks[i]`.
    pub spans: Vec<RowSpan>,
    /// `Some` for `BatchLimit::Tokens`; `Batch::positions()` index the concatenated rows.
    pub batches: Option<Vec<Batch>>,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Launch {
    Finished,
    Pending,
}
/// Executes a batch on a lane. The runner finishes or fails every task itself.
pub trait BatchRunner<P>: Send + Sync + 'static {
    /// CPU: runs synchronously and returns `Finished`. GPU: launches on the lane's stream
    /// and returns `Pending`.
    fn run(&self, lane: &mut Lane, batch: &mut ExecBatch<P>) -> Launch;
    /// GPU only, after the lane's event completed. `result` is the event outcome.
    fn collect(
        &self,
        lane: &mut Lane,
        batch: ExecBatch<P>,
        result: Result<(), EventError>,
    );
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LaneSetError {
    NoLanes,
    InvalidLimit,
    Spawn(String),
}
impl std::fmt::Display for LaneSetError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LaneSetError::NoLanes => write!(f, "no lanes given"),
            LaneSetError::InvalidLimit => write!(f, "invalid batch limit"),
            LaneSetError::Spawn(cause) => write!(f, "spawn failed: {cause}"),
        }
    }
}
impl std::error::Error for LaneSetError {}
pub struct LaneResourceConfig<M, P> {
    pub own: Arc<Queue<M, P, (), ()>>,
    pub shared: Option<Arc<Queue<M, P, (), ()>>>,
    pub filter: PullFilter<M>,
    /// `Some` for GPU lane resources: launched batches are tracked here.
    pub completion: Option<Arc<GpuCompletion>>,
    pub limit: BatchLimit,
    pub notifier: SubengineNotifier,
    pub kind: ResourceKind,
    pub device: DeviceHandle,
}
type ClaimOf<M, P> = Claim2<M, P, (), ()>;
struct LaneSlot {
    /// `None` while the lane is running a batch or is in flight.
    lane: Mutex<Option<Lane>>,
    freed: Condvar,
    busy: AtomicBool,
    meter: LoadMeter,
}
impl LaneSlot {
    fn try_take(&self) -> Option<Lane> {
        let mut guard = match self.lane.try_lock() {
            Ok(guard) => guard,
            Err(TryLockError::Poisoned(p)) => p.into_inner(),
            Err(TryLockError::WouldBlock) => return None,
        };
        let lane = guard.take()?;
        self.busy.store(true, Ordering::SeqCst);
        Some(lane)
    }
    fn take_blocking(&self) -> Lane {
        let mut guard = lock(&self.lane);
        loop {
            if let Some(lane) = guard.take() {
                self.busy.store(true, Ordering::SeqCst);
                return lane;
            }
            guard = self.freed.wait(guard).unwrap_or_else(PoisonError::into_inner);
        }
    }
    fn give_back(&self, lane: Lane) {
        let mut guard = lock(&self.lane);
        *guard = Some(lane);
        self.busy.store(false, Ordering::SeqCst);
        drop(guard);
        self.freed.notify_one();
    }
}
#[derive(Default)]
struct FitState {
    tasks: usize,
    rows: usize,
    padded: usize,
}
impl FitState {
    fn is_full(&self, limit: &BatchLimit) -> bool {
        matches!(limit, BatchLimit::Tasks(n) if self.tasks >= * n)
    }
    fn fits(&self, limit: &BatchLimit, lens: &[usize]) -> bool {
        match limit {
            BatchLimit::Tasks(n) => self.tasks < *n,
            BatchLimit::Tokens { token_budget, max_seq_len, .. } => {
                if lens.is_empty() {
                    return true;
                }
                let p = lens
                    .iter()
                    .map(|&l| padded_len(l, *max_seq_len))
                    .fold(self.padded, usize::max);
                self.rows.saturating_add(lens.len()).saturating_mul(p) <= *token_budget
            }
        }
    }
    fn absorb(&mut self, limit: &BatchLimit, lens: &[usize]) {
        self.tasks += 1;
        self.rows += lens.len();
        if let BatchLimit::Tokens { max_seq_len, .. } = limit {
            self.padded = lens
                .iter()
                .map(|&l| padded_len(l, *max_seq_len))
                .fold(self.padded, usize::max);
        }
    }
}
struct Formed<M, P> {
    batch: ExecBatch<P>,
    claims: Vec<ClaimOf<M, P>>,
    entries: usize,
}
struct InFlight<M, P> {
    slot: usize,
    batch: ExecBatch<P>,
    claims: Vec<ClaimOf<M, P>>,
    lease: Lease2<LaneResource<M, P>>,
    start: Instant,
    entries: usize,
}
pub struct LaneResource<M, P> {
    own: Arc<Queue<M, P, (), ()>>,
    shared: Option<Arc<Queue<M, P, (), ()>>>,
    filter: PullFilter<M>,
    completion: Option<Arc<GpuCompletion>>,
    limit: BatchLimit,
    notifier: SubengineNotifier,
    kind: ResourceKind,
    device: DeviceHandle,
    runner: Arc<dyn BatchRunner<P>>,
    slots: Vec<LaneSlot>,
    in_flight: Mutex<HashMap<u64, InFlight<M, P>>>,
}
impl<M, P> LaneResource<M, P>
where
    P: BatchPayload,
    M: BatchMeta + HasResourceRequirements + Send + Sync + 'static,
{
    /// Takes the lanes. Starts no threads.
    pub fn new(
        cfg: LaneResourceConfig<M, P>,
        lanes: Vec<Lane>,
        runner: Arc<dyn BatchRunner<P>>,
    ) -> Result<Arc<Self>, LaneSetError> {
        if lanes.is_empty() {
            return Err(LaneSetError::NoLanes);
        }
        let invalid = match &cfg.limit {
            BatchLimit::Tokens { token_budget, max_seq_len, .. } => {
                *token_budget == 0 || *max_seq_len == 0
            }
            BatchLimit::Tasks(n) => *n == 0,
        };
        if invalid {
            return Err(LaneSetError::InvalidLimit);
        }
        let slots = lanes
            .into_iter()
            .map(|lane| LaneSlot {
                lane: Mutex::new(Some(lane)),
                freed: Condvar::new(),
                busy: AtomicBool::new(false),
                meter: LoadMeter::new(cfg.kind, cfg.device, 1),
            })
            .collect();
        Ok(
            Arc::new(Self {
                own: cfg.own,
                shared: cfg.shared,
                filter: cfg.filter,
                completion: cfg.completion,
                limit: cfg.limit,
                notifier: cfg.notifier,
                kind: cfg.kind,
                device: cfg.device,
                runner,
                slots,
                in_flight: Mutex::new(HashMap::new()),
            }),
        )
    }
    /// Called for a finished lane whose tag belongs to this resource. `Err`: unknown tag, lane returned unchanged.
    pub fn on_finished(&self, finished: FinishedLane) -> Result<(), FinishedLane> {
        let Some(record) = lock(&self.in_flight).remove(&finished.tag) else {
            return Err(finished);
        };
        let FinishedLane { lane, result, .. } = finished;
        self.collect_and_conclude(
            record.slot,
            lane,
            record.batch,
            record.claims,
            record.lease,
            record.entries,
            record.start,
            result,
        );
        Ok(())
    }
    /// True iff `tag` belongs to a batch in flight of this resource.
    pub fn owns_tag(&self, tag: u64) -> bool {
        lock(&self.in_flight).contains_key(&tag)
    }
    /// Drops all batches in flight: claims become `Abandoned`, leases are released. Lanes stay
    /// with `GpuCompletion`. Only valid once the device no longer reads the batch data
    /// (shutdown after stream synchronisation). Returns the number of dropped batches.
    pub fn abort_in_flight(&self) -> usize {
        let records = std::mem::take(&mut *lock(&self.in_flight));
        records.len()
    }
    pub fn lane_count(&self) -> usize {
        self.slots.len()
    }
    /// Lanes with neither a running nor a pending job.
    pub fn idle_lanes(&self) -> usize {
        self.slots.iter().filter(|s| !s.busy.load(Ordering::SeqCst)).count()
    }
    fn acquire_lane(&self) -> (usize, Lane) {
        let throughputs: Vec<f64> = self
            .slots
            .iter()
            .map(|s| s.meter.throughput())
            .collect();
        let mut idle: Vec<usize> = (0..self.slots.len())
            .filter(|&i| !self.slots[i].busy.load(Ordering::SeqCst))
            .collect();
        idle.sort_by(|&a, &b| throughputs[b].total_cmp(&throughputs[a]));
        for i in idle {
            if let Some(lane) = self.slots[i].try_take() {
                return (i, lane);
            }
        }
        let mut best = 0;
        for i in 1..throughputs.len() {
            if throughputs[i].total_cmp(&throughputs[best]).is_gt() {
                best = i;
            }
        }
        (best, self.slots[best].take_blocking())
    }
    fn form_batch(&self, seed_claim: ClaimOf<M, P>, seed: P) -> Formed<M, P> {
        let key = seed.batch_key();
        let mut state = FitState::default();
        let seed_lens: Vec<usize> = seed.sequences().iter().map(Vec::len).collect();
        state.absorb(&self.limit, &seed_lens);
        let mut payloads = vec![seed];
        let mut claims = vec![seed_claim];
        if let Some(key) = key {
            self.scan(&key, &mut state, &mut payloads, &mut claims);
        }
        let mut spans = Vec::with_capacity(payloads.len());
        let mut first_row = 0;
        for payload in &payloads {
            let rows = payload.sequences().len();
            spans.push(RowSpan { first_row, rows });
            first_row += rows;
        }
        let entries = if first_row == 0 { payloads.len() } else { first_row };
        let batches = match &self.limit {
            BatchLimit::Tokens { token_budget, max_seq_len, pad_token_id } => {
                let sequences: Vec<Vec<u32>> = payloads
                    .iter()
                    .flat_map(|p| p.sequences().iter().cloned())
                    .collect();
                let config = BatchConfig {
                    token_budget: *token_budget,
                    max_seq_len: *max_seq_len,
                    pad_token_id: *pad_token_id,
                };
                Some(
                    build_batches(&sequences, &config)
                        .unwrap_or_else(|e| {
                            error!("lane batching: build_batches failed: {e:?}");
                            Vec::new()
                        }),
                )
            }
            BatchLimit::Tasks(_) => None,
        };
        Formed {
            batch: ExecBatch {
                tasks: payloads,
                spans,
                batches,
            },
            claims,
            entries,
        }
    }
    fn scan(
        &self,
        key: &BatchKey,
        state: &mut FitState,
        payloads: &mut Vec<P>,
        claims: &mut Vec<ClaimOf<M, P>>,
    ) {
        for queue in std::iter::once(&self.own).chain(self.shared.iter()) {
            while !state.is_full(&self.limit) {
                let claimed = queue
                    .take_where(|m| {
                        (self.filter)(m) && m.batch_key().as_ref() == Some(key)
                            && state.fits(&self.limit, m.seq_lengths())
                    });
                let Some(mut claim) = claimed else {
                    break;
                };
                let Some(payload) = claim.take_payload() else {
                    continue;
                };
                if claim.meta().seq_lengths().len() != payload.sequences().len() {
                    warn!("lane batching: metadata row count differs from payload");
                }
                state.absorb(&self.limit, claim.meta().seq_lengths());
                payloads.push(payload);
                claims.push(claim);
            }
        }
    }
    fn launch(
        &self,
        slot: usize,
        mut lane: Lane,
        formed: Formed<M, P>,
        lease: Lease2<Self>,
    ) {
        let Formed { mut batch, claims, entries } = formed;
        let start = Instant::now();
        let outcome = catch_unwind(
            AssertUnwindSafe(|| self.runner.run(&mut lane, &mut batch)),
        );
        match outcome {
            Err(_) => {
                error!("lane batching: runner panicked in run");
                drop(batch);
                self.abandon(slot, lane, claims, lease);
            }
            Ok(Launch::Finished) => {
                drop(batch);
                self.conclude(slot, lane, claims, lease, entries, start);
            }
            Ok(Launch::Pending) => {
                self.park(slot, lane, batch, claims, lease, entries, start);
            }
        }
    }
    fn park(
        &self,
        slot: usize,
        lane: Lane,
        batch: ExecBatch<P>,
        claims: Vec<ClaimOf<M, P>>,
        lease: Lease2<Self>,
        entries: usize,
        start: Instant,
    ) {
        let event = match lane.record_event() {
            Ok(event) => event,
            Err(e) => {
                let cause = match e {
                    LaneError::Record(inner) => inner,
                    LaneError::NoStreamBackend => {
                        EventError {
                            device: lane.device(),
                            cause: "lane has no stream backend".to_string(),
                        }
                    }
                };
                error!("lane batching: cannot record event: {cause}");
                self.collect_and_conclude(
                    slot,
                    lane,
                    batch,
                    claims,
                    lease,
                    entries,
                    start,
                    Err(cause),
                );
                return;
            }
        };
        let Some(completion) = &self.completion else {
            warn!(
                "lane batching: pending batch without completion tracker, waiting inline"
            );
            let result = event.wait();
            self.collect_and_conclude(
                slot,
                lane,
                batch,
                claims,
                lease,
                entries,
                start,
                result,
            );
            return;
        };
        let tag = NEXT_TAG.fetch_add(1, Ordering::Relaxed);
        lock(&self.in_flight)
            .insert(
                tag,
                InFlight {
                    slot,
                    batch,
                    claims,
                    lease,
                    start,
                    entries,
                },
            );
        drop(completion.submit(lane, event, tag));
    }
    #[allow(clippy::too_many_arguments)]
    fn collect_and_conclude(
        &self,
        slot: usize,
        mut lane: Lane,
        batch: ExecBatch<P>,
        claims: Vec<ClaimOf<M, P>>,
        lease: Lease2<Self>,
        entries: usize,
        start: Instant,
        result: Result<(), EventError>,
    ) {
        let outcome = catch_unwind(
            AssertUnwindSafe(|| { self.runner.collect(&mut lane, batch, result) }),
        );
        match outcome {
            Ok(()) => self.conclude(slot, lane, claims, lease, entries, start),
            Err(_) => {
                error!("lane batching: runner panicked in collect");
                self.abandon(slot, lane, claims, lease);
            }
        }
    }
    fn conclude(
        &self,
        slot: usize,
        lane: Lane,
        claims: Vec<ClaimOf<M, P>>,
        lease: Lease2<Self>,
        entries: usize,
        start: Instant,
    ) {
        let elapsed = start.elapsed();
        if !elapsed.is_zero() {
            self.slots[slot].meter.record(entries, elapsed);
        }
        for claim in claims {
            claim.complete(Ok(()));
        }
        drop(lease);
        self.slots[slot].give_back(lane);
        self.notifier.notify_result();
    }
    fn abandon(
        &self,
        slot: usize,
        lane: Lane,
        claims: Vec<ClaimOf<M, P>>,
        lease: Lease2<Self>,
    ) {
        drop(claims);
        drop(lease);
        self.slots[slot].give_back(lane);
        self.notifier.notify_result();
    }
}
impl<M, P> ResourceExec for LaneResource<M, P>
where
    P: BatchPayload,
    M: BatchMeta + HasResourceRequirements + Send + Sync + 'static,
{
    type Entry = Claim2<M, P, (), ()>;
    fn has_capacity(&self) -> bool {
        self.idle_lanes() > 0
    }
    fn execute(&self, mut entry: Claim2<M, P, (), ()>, lease: Lease2<Self>) {
        let Some(seed) = entry.take_payload() else {
            return;
        };
        let (slot, lane) = self.acquire_lane();
        let formed = catch_unwind(AssertUnwindSafe(|| self.form_batch(entry, seed)));
        match formed {
            Ok(formed) => self.launch(slot, lane, formed, lease),
            Err(_) => {
                error!("lane batching: panic while forming a batch");
                drop(lease);
                self.slots[slot].give_back(lane);
                self.notifier.notify_result();
            }
        }
    }
}
impl<M, P> LoadReporting for LaneResource<M, P>
where
    P: BatchPayload,
    M: BatchMeta + HasResourceRequirements + Send + Sync + 'static,
{
    fn report(&self) -> LoadReport {
        let sum: f64 = self.slots.iter().map(|s| s.meter.throughput()).sum();
        LoadReport {
            kind: self.kind,
            device: self.device,
            capacity: self.idle_lanes(),
            throughput: if sum.is_finite() { sum.max(0.0) } else { f64::MAX },
        }
    }
}
#[cfg(test)]
#[path = "test/lane_batching.rs"]
mod tests;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::_1::sync_util::lock;
