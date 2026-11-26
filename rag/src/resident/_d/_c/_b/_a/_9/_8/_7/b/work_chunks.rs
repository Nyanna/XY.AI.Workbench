use std::fmt;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, Instant};

use super::super::_6::_5::query_batch::{DocSource, QueryBatch, SegmentId, Workspace};
use super::super::_6::topk::{RangeScorer, TopKError};

pub const DEFAULT_MIN_LEN: u32 = 16;
pub const DEFAULT_MAX_LEN: u32 = 1 << 20;
pub const DEFAULT_TARGET: Duration = Duration::from_millis(2);
pub const EMA_ALPHA: f64 = 0.3;

const MIN_ELAPSED: Duration = Duration::from_micros(1);

/// Sizes chunks from the smoothed throughput (slots per second).
#[derive(Debug, Clone)]
pub struct ChunkSizer {
    min_len: u32,
    max_len: u32,
    target: Duration,
    throughput: Option<f64>,
}

impl ChunkSizer {
    pub fn new() -> ChunkSizer {
        ChunkSizer::with_limits(DEFAULT_MIN_LEN, DEFAULT_MAX_LEN, DEFAULT_TARGET)
    }

    pub fn with_limits(min_len: u32, max_len: u32, target: Duration) -> ChunkSizer {
        let min_len = min_len.max(1);
        ChunkSizer {
            min_len,
            max_len: max_len.max(min_len),
            target,
            throughput: None,
        }
    }

    pub fn next_len(&self) -> u32 {
        match self.throughput {
            None => self.min_len,
            Some(t) => {
                let raw = (t * self.target.as_secs_f64()).round();
                let clamped = raw.clamp(self.min_len as f64, self.max_len as f64);
                (clamped as u32).clamp(self.min_len, self.max_len)
            }
        }
    }

    pub fn throughput(&self) -> Option<f64> {
        self.throughput
    }

    pub fn observe(&mut self, slots: u32, elapsed: Duration) {
        if slots == 0 {
            return;
        }
        let secs = elapsed.max(MIN_ELAPSED).as_secs_f64();
        let sample = slots as f64 / secs;
        self.throughput = Some(match self.throughput {
            None => sample,
            Some(t) => EMA_ALPHA * sample + (1.0 - EMA_ALPHA) * t,
        });
    }

    pub fn min_len(&self) -> u32 {
        self.min_len
    }

    pub fn max_len(&self) -> u32 {
        self.max_len
    }

    pub fn target(&self) -> Duration {
        self.target
    }
}

impl Default for ChunkSizer {
    fn default() -> Self {
        ChunkSizer::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TaskClass {
    Query,
    Indexing,
}

/// Shared state giving query tasks precedence over indexing chunks.
#[derive(Debug, Default)]
pub struct PreemptionGate {
    waiting_queries: AtomicUsize,
}

impl PreemptionGate {
    pub fn new() -> PreemptionGate {
        PreemptionGate::default()
    }

    /// Registers a waiting or running query task until the guard is dropped.
    pub fn enter_query(&self) -> QueryGuard<'_> {
        self.waiting_queries.fetch_add(1, Ordering::SeqCst);
        QueryGuard { gate: self }
    }

    pub fn waiting_queries(&self) -> usize {
        self.waiting_queries.load(Ordering::SeqCst)
    }

    pub fn should_preempt(&self, class: TaskClass) -> bool {
        match class {
            TaskClass::Query => false,
            TaskClass::Indexing => self.waiting_queries() > 0,
        }
    }
}

#[derive(Debug)]
pub struct QueryGuard<'a> {
    gate: &'a PreemptionGate,
}

impl Drop for QueryGuard<'_> {
    fn drop(&mut self) {
        self.gate.waiting_queries.fetch_sub(1, Ordering::SeqCst);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StopReason {
    TimeBudget,
    Preemption,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChunkOutcome {
    pub done_slots: u32,
    pub rest: Workspace,
    pub stop: Option<StopReason>,
}

/// Runs `range` chunk by chunk; stops only at chunk boundaries.
pub fn run_chunks<E, F>(
    sizer: &mut ChunkSizer,
    range: Workspace,
    budget: Duration,
    class: TaskClass,
    gate: &PreemptionGate,
    mut run_chunk: F,
) -> Result<ChunkOutcome, E>
where
    F: FnMut(Workspace) -> Result<(), E>,
{
    let mut rest = range;
    let mut done_slots: u32 = 0;
    let started = Instant::now();
    while !rest.is_empty() {
        let (chunk, tail) = rest.split_front(sizer.next_len());
        let chunk_started = Instant::now();
        run_chunk(chunk)?;
        sizer.observe(chunk.len(), chunk_started.elapsed());
        done_slots += chunk.len();
        rest = tail;
        if rest.is_empty() {
            break;
        }
        if started.elapsed() >= budget {
            return Ok(ChunkOutcome {
                done_slots,
                rest,
                stop: Some(StopReason::TimeBudget),
            });
        }
        if gate.should_preempt(class) {
            return Ok(ChunkOutcome {
                done_slots,
                rest,
                stop: Some(StopReason::Preemption),
            });
        }
    }
    Ok(ChunkOutcome {
        done_slots,
        rest,
        stop: None,
    })
}

/// Scores one tile against all queries of the batch.
pub trait TileScorer: Send + Sync {
    type Error: std::error::Error + Send + Sync + 'static;

    /// Query-major scores: `scores[q * tile.len() + i]` for slot `tile.start() + i`.
    fn score_tile(
        &self,
        batch: &QueryBatch,
        segment: SegmentId,
        tile: Workspace,
    ) -> Result<Vec<f32>, Self::Error>;
}

#[derive(Debug)]
pub enum ChunkError<E> {
    Score(E),
    TopK(TopKError),
}

impl<E: fmt::Display> fmt::Display for ChunkError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ChunkError::Score(e) => write!(f, "tile scoring failed: {e}"),
            ChunkError::TopK(e) => write!(f, "top-k accumulation failed: {e}"),
        }
    }
}

impl<E: std::error::Error + 'static> std::error::Error for ChunkError<E> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ChunkError::Score(e) => Some(e),
            ChunkError::TopK(e) => Some(e),
        }
    }
}

impl<E> From<TopKError> for ChunkError<E> {
    fn from(e: TopKError) -> Self {
        ChunkError::TopK(e)
    }
}

#[allow(clippy::too_many_arguments)]
pub fn score_range<S: TileScorer>(
    sizer: &mut ChunkSizer,
    scorer: &S,
    src: &dyn DocSource,
    batch: &QueryBatch,
    segment: SegmentId,
    range: Workspace,
    budget: Duration,
    class: TaskClass,
    gate: &PreemptionGate,
    rs: &mut RangeScorer,
) -> Result<ChunkOutcome, ChunkError<S::Error>> {
    run_chunks(sizer, range, budget, class, gate, |tile| {
        let scores = scorer
            .score_tile(batch, segment, tile)
            .map_err(ChunkError::Score)?;
        rs.add_range(src, segment, tile, &scores)?;
        Ok(())
    })
}

#[cfg(test)]
#[path = "test/work_chunks.rs"]
mod tests;
