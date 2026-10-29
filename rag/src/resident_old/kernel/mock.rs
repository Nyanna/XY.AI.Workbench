//! Deterministic hand-written kernel fake for GPU-free tests.
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Mutex;
use xxhash_rust::xxh3::xxh3_64;
use super::{
    BatchDesc, BatchId, Kernel, KernelHit, KernelOutput, RegionSpan, WrittenRange,
};
use crate::resident::error::IndexError;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Call {
    Begin(BatchId),
    Run { batch: BatchId, spans: usize },
    End(BatchId),
}
#[derive(Clone, Copy, Debug)]
enum Mode {
    Scoring,
    Embedding,
}
struct Batch {
    queries: u32,
    progress: AtomicU64,
    written: Vec<WrittenRange>,
}
struct State {
    batches: HashMap<u64, Batch>,
    calls: Vec<Call>,
    capacity: u64,
}
/// Scoring mode returns one hit per input span; embedding mode "writes" the input spans into a
/// bounded capacity and reports `Partial` once it is exhausted.
pub struct MockKernel {
    mode: Mode,
    next_id: AtomicU64,
    runs: AtomicU64,
    fail_after: Option<u64>,
    state: Mutex<State>,
}
impl MockKernel {
    pub fn scoring() -> Self {
        Self::new(Mode::Scoring, 0)
    }
    pub fn embedding(capacity: u64) -> Self {
        Self::new(Mode::Embedding, capacity)
    }
    fn new(mode: Mode, capacity: u64) -> Self {
        Self {
            mode,
            next_id: AtomicU64::new(0),
            runs: AtomicU64::new(0),
            fail_after: None,
            state: Mutex::new(State {
                batches: HashMap::new(),
                calls: Vec::new(),
                capacity,
            }),
        }
    }
    /// `run` succeeds `n` times, then fails on every further call.
    pub fn fail_after(mut self, n: u64) -> Self {
        self.fail_after = Some(n);
        self
    }
    /// Grants additional writable capacity, as the manager does after a `Partial`.
    pub fn grant(&self, bytes: u64) {
        self.state.lock().unwrap().capacity += bytes;
    }
    pub fn calls(&self) -> Vec<Call> {
        self.state.lock().unwrap().calls.clone()
    }
}
fn score(queries: u32, span: &RegionSpan) -> f32 {
    let mut bytes = [0u8; 20];
    bytes[..8].copy_from_slice(&span.offset.to_le_bytes());
    bytes[8..16].copy_from_slice(&span.len.to_le_bytes());
    bytes[16..].copy_from_slice(&queries.to_le_bytes());
    (xxh3_64(&bytes) >> 40) as f32 / (1u64 << 24) as f32
}
impl Kernel for MockKernel {
    fn begin(&self, batch: &BatchDesc) -> Result<BatchId, IndexError> {
        let id = BatchId(self.next_id.fetch_add(1, Ordering::SeqCst));
        let mut state = self.state.lock().unwrap();
        state
            .batches
            .insert(
                id.0,
                Batch {
                    queries: batch.queries,
                    progress: AtomicU64::new(0),
                    written: Vec::new(),
                },
            );
        state.calls.push(Call::Begin(id));
        tracing::debug!(batch = id.0, "kernel batch begin");
        Ok(id)
    }
    fn run(
        &self,
        batch: BatchId,
        input: &[RegionSpan],
    ) -> Result<KernelOutput, IndexError> {
        let prior = self.runs.fetch_add(1, Ordering::SeqCst);
        let mut state = self.state.lock().unwrap();
        state
            .calls
            .push(Call::Run {
                batch,
                spans: input.len(),
            });
        if matches!(self.fail_after, Some(n) if prior >= n) {
            return Err(IndexError::KernelFailed {
                reason: "injected failure".into(),
            });
        }
        let State { batches, capacity, .. } = &mut *state;
        let b = batches
            .get_mut(&batch.0)
            .ok_or(IndexError::UnknownBatch {
                batch: batch.0,
            })?;
        match self.mode {
            Mode::Scoring => {
                let hits: Vec<KernelHit> = input
                    .iter()
                    .map(|s| KernelHit {
                        offset: s.offset,
                        score: score(b.queries, s),
                    })
                    .collect();
                b.progress.fetch_add(input.len() as u64, Ordering::SeqCst);
                tracing::debug!(batch = batch.0, hits = hits.len(), "kernel run");
                Ok(KernelOutput::Hits(hits))
            }
            Mode::Embedding => {
                for span in input {
                    let take = span.len.min(*capacity);
                    if take > 0 {
                        b.written
                            .push(WrittenRange {
                                offset: span.offset,
                                len: take,
                            });
                        *capacity -= take;
                        b.progress.fetch_add(take, Ordering::SeqCst);
                    }
                    if take < span.len {
                        tracing::debug!(batch = batch.0, "kernel run partial");
                        return Ok(KernelOutput::Partial);
                    }
                }
                Ok(KernelOutput::Hits(Vec::new()))
            }
        }
    }
    fn grant_region(
        &self,
        batch: BatchId,
        region: RegionSpan,
    ) -> Result<(), IndexError> {
        let mut state = self.state.lock().unwrap();
        if !state.batches.contains_key(&batch.0) {
            return Err(IndexError::UnknownBatch {
                batch: batch.0,
            });
        }
        state.capacity += region.len;
        Ok(())
    }
    fn progress(&self, batch: BatchId) -> u64 {
        let state = self.state.lock().unwrap();
        state.batches.get(&batch.0).map_or(0, |b| b.progress.load(Ordering::SeqCst))
    }
    fn end(&self, batch: BatchId) -> Result<Vec<WrittenRange>, IndexError> {
        let mut state = self.state.lock().unwrap();
        state.calls.push(Call::End(batch));
        let b = state
            .batches
            .remove(&batch.0)
            .ok_or(IndexError::UnknownBatch {
                batch: batch.0,
            })?;
        tracing::debug!(batch = batch.0, ranges = b.written.len(), "kernel batch end");
        Ok(b.written)
    }
}
