//! Scores a query vector against slots (rows) of device segments.

use std::fmt;
use std::ops::Range;
use std::sync::Arc;

use crate::resident::l130::l120::l110::l100::l090::l080::l070::a::kernel_register::{
    Kernel, KernelError, KernelFunction, KernelRegister, KernelSource, Variant,
};
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::a::library_adapter::DeviceBlock;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::a::region_handle::RegionHandle;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::a::arenas::{
    ArenaKind, ArenaOffset,
};
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::l010::{
    device_lane_event::{Event, Lane, LaneError},
    layout::{DType, Layout},
};

pub const KERNEL_SCORE_DOT: &str = "score_dot";
pub const KERNEL_SCORE_NEGL2: &str = "score_negl2";

pub const ENTRY_SCORE_DOT: &str = "score_dot_rows";
pub const ENTRY_SCORE_NEGL2: &str = "score_negl2_rows";

const SCORE_DOT_BODY: &str = r#"
extern "C" __global__ void score_dot_rows(
    const T* rows, const T* query, float* out, unsigned int n_rows) {
    size_t warp = ((size_t)blockIdx.x * blockDim.x + threadIdx.x) >> 5;
    unsigned int lane = threadIdx.x & 31u;
    if (warp >= n_rows) return;
    const T* row = rows + warp * DIM;
    float acc = 0.0f;
    for (int i = (int)lane; i < DIM; i += 32) {
        acc += load_f(query, (size_t)i) * load_f(row, (size_t)i);
    }
    acc = warp_reduce_sum(acc);
    if (lane == 0) out[warp] = acc;
}
"#;

const SCORE_NEGL2_BODY: &str = r#"
extern "C" __global__ void score_negl2_rows(
    const T* rows, const T* query, float* out, unsigned int n_rows) {
    size_t warp = ((size_t)blockIdx.x * blockDim.x + threadIdx.x) >> 5;
    unsigned int lane = threadIdx.x & 31u;
    if (warp >= n_rows) return;
    const T* row = rows + warp * DIM;
    float acc = 0.0f;
    for (int i = (int)lane; i < DIM; i += 32) {
        float d = load_f(query, (size_t)i) - load_f(row, (size_t)i);
        acc += d * d;
    }
    acc = warp_reduce_sum(acc);
    if (lane == 0) out[warp] = -acc;
}
"#;

const SCORE_DTYPES: &[DType] = &[DType::F32, DType::F16];

/// Higher score is better for both metrics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Metric {
    Dot,
    NegL2Sq,
}

impl Metric {
    pub fn kernel_name(self) -> &'static str {
        match self {
            Metric::Dot => KERNEL_SCORE_DOT,
            Metric::NegL2Sq => KERNEL_SCORE_NEGL2,
        }
    }
}

/// Device region holding rows `[rows, dim]` (rank 2, contiguous, row-major).
#[derive(Debug, Clone)]
pub struct ScoreSegment {
    pub block: DeviceBlock,
    pub region: RegionHandle,
    pub layout: Layout,
}

#[derive(Debug, Clone)]
pub struct SegmentSlots {
    pub segment: ScoreSegment,
    pub slots: Range<u32>,
}

#[derive(Debug, Clone)]
pub struct ScoreRequest {
    pub metric: Metric,
    pub query: Vec<f32>,
    pub segments: Vec<SegmentSlots>,
}

/// Arena and byte offset of a slot row within the arena.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SlotAddr {
    arena: ArenaKind,
    offset: ArenaOffset,
}

impl SlotAddr {
    pub fn new(arena: ArenaKind, offset: ArenaOffset) -> SlotAddr {
        SlotAddr { arena, offset }
    }

    pub fn arena(&self) -> ArenaKind {
        self.arena
    }

    pub fn offset(&self) -> ArenaOffset {
        self.offset
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ScoredSlot {
    pub addr: SlotAddr,
    pub score: f32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScoreError {
    UnsupportedDType(DType),
    QueryDimMismatch { query: usize, dim: usize },
    BadLayout(String),
    OutOfBounds { slots: Range<u32>, rows: usize },
    Kernel(KernelError),
    Launch(String),
    Readback(String),
    Lane(LaneError),
}

impl fmt::Display for ScoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ScoreError::UnsupportedDType(d) => write!(f, "unsupported dtype for scoring: {d:?}"),
            ScoreError::QueryDimMismatch { query, dim } => {
                write!(f, "query length {query} does not match segment dim {dim}")
            }
            ScoreError::BadLayout(t) => write!(f, "bad segment layout: {t}"),
            ScoreError::OutOfBounds { slots, rows } => {
                write!(f, "slots {}..{} out of bounds for {rows} rows", slots.start, slots.end)
            }
            ScoreError::Kernel(e) => write!(f, "score kernel unavailable: {e}"),
            ScoreError::Launch(t) => write!(f, "score launch failed: {t}"),
            ScoreError::Readback(t) => write!(f, "score readback failed: {t}"),
            ScoreError::Lane(e) => write!(f, "lane error: {e}"),
        }
    }
}

impl std::error::Error for ScoreError {}

pub struct LaunchSegment<'a> {
    pub block: &'a DeviceBlock,
    /// Device address of the first scored row.
    pub first_row_addr: u64,
    pub rows: u32,
}

pub struct LaunchRequest<'a> {
    pub metric: Metric,
    /// `F32` or `F16`.
    pub dtype: DType,
    pub dim: u32,
    /// The binding uploads the query and converts it to `dtype` if needed.
    pub query: &'a [f32],
    pub segments: &'a [LaunchSegment<'a>],
}

/// Implemented by the GPU binding.
///
/// Kernel ABI, per segment launch: `(const T* rows, const T* query, float* out, unsigned int n_rows)`
/// with one warp per row. The block size is a multiple of 32 and the grid covers `n_rows * 32`
/// threads. `rows` is `first_row_addr`; `out` points at the segment's part of the output buffer.
pub trait ScoreLauncher: Send + Sync {
    /// Enqueues upload, kernel and output buffer on the lane without blocking.
    /// Output: one f32 score per row, concatenated in segment and row order.
    fn launch(
        &self,
        function: &Arc<dyn KernelFunction>,
        lane: &Lane,
        req: &LaunchRequest<'_>,
    ) -> Result<Box<dyn ScoreReadback>, String>;
}

pub trait ScoreReadback: Send + Sync {
    /// Called only after the job event completed.
    fn read(&self) -> Result<Vec<f32>, String>;
}

/// Registers the `score_dot` and `score_negl2` JIT sources.
pub fn register_score_kernels(register: &KernelRegister) -> Result<(), KernelError> {
    register.register_source(KernelSource {
        name: KERNEL_SCORE_DOT,
        body: SCORE_DOT_BODY,
        entry: ENTRY_SCORE_DOT,
        dtypes: SCORE_DTYPES,
        precompiled: None,
    })?;
    register.register_source(KernelSource {
        name: KERNEL_SCORE_NEGL2,
        body: SCORE_NEGL2_BODY,
        entry: ENTRY_SCORE_NEGL2,
        dtypes: SCORE_DTYPES,
        precompiled: None,
    })
}

struct Pending {
    readback: Box<dyn ScoreReadback>,
    addrs: Vec<SlotAddr>,
}

pub struct ScoreJob {
    event: Event,
    pending: Option<Pending>,
}

static_assertions::assert_impl_all!(ScoreJob: Send);

impl ScoreJob {
    pub fn event(&self) -> &Event {
        &self.event
    }

    pub fn is_complete(&self) -> bool {
        self.event.is_complete().unwrap_or(true)
    }

    pub fn finish(self) -> Result<Vec<ScoredSlot>, ScoreError> {
        self.event
            .wait()
            .map_err(|e| ScoreError::Lane(LaneError::Record(e)))?;
        let Some(pending) = self.pending else {
            return Ok(Vec::new());
        };
        let scores = pending.readback.read().map_err(ScoreError::Readback)?;
        if scores.len() != pending.addrs.len() {
            return Err(ScoreError::Readback("length".to_string()));
        }
        Ok(pending
            .addrs
            .into_iter()
            .zip(scores)
            .map(|(addr, score)| ScoredSlot { addr, score })
            .collect())
    }
}

pub struct Scorer {
    register: Arc<KernelRegister>,
    launcher: Arc<dyn ScoreLauncher>,
}

static_assertions::assert_impl_all!(Scorer: Send, Sync);

impl Scorer {
    pub fn new(register: Arc<KernelRegister>, launcher: Arc<dyn ScoreLauncher>) -> Scorer {
        Scorer { register, launcher }
    }

    /// Non-blocking: validates, fetches the kernel, launches on `lane` and records the event.
    pub fn score(&self, lane: &Lane, req: &ScoreRequest) -> Result<ScoreJob, ScoreError> {
        let Some((dtype, dim)) = validate(req)? else {
            return Ok(completed_job());
        };
        let plan = plan(req, dtype, dim)?;
        if plan.segments.is_empty() {
            return Ok(completed_job());
        }

        let function = match self.register.request(
            req.metric.kernel_name(),
            dtype,
            Variant {
                dim: Some(dim),
                tile: None,
            },
        ) {
            Ok(Kernel::Gpu(k)) => Arc::clone(&k.function),
            Ok(Kernel::Library(_)) => {
                return Err(ScoreError::Kernel(KernelError::Unavailable(format!(
                    "kernel '{}' resolved to a library kernel",
                    req.metric.kernel_name()
                ))))
            }
            Err(e) => return Err(ScoreError::Kernel(e)),
        };

        let launch = LaunchRequest {
            metric: req.metric,
            dtype,
            dim,
            query: &req.query,
            segments: &plan.segments,
        };
        let readback = self
            .launcher
            .launch(&function, lane, &launch)
            .map_err(ScoreError::Launch)?;
        let event = lane.record_event().map_err(ScoreError::Lane)?;
        Ok(ScoreJob {
            event,
            pending: Some(Pending {
                readback,
                addrs: plan.addrs,
            }),
        })
    }
}

fn completed_job() -> ScoreJob {
    let (event, signal) = Event::cpu();
    signal.signal();
    ScoreJob {
        event,
        pending: None,
    }
}

fn elem_size(dtype: DType) -> u64 {
    dtype.size_in_bytes() as u64
}

fn bad(text: impl Into<String>) -> ScoreError {
    ScoreError::BadLayout(text.into())
}

/// Returns the common dtype and dim, or `None` without segments. Each check runs over all
/// segments before the next one starts.
fn validate(req: &ScoreRequest) -> Result<Option<(DType, u32)>, ScoreError> {
    let segments = &req.segments;
    let Some(first) = segments.first() else {
        return Ok(None);
    };
    let dtype = first.segment.layout.dtype();

    for s in segments {
        let d = s.segment.layout.dtype();
        if !matches!(d, DType::F32 | DType::F16) {
            return Err(ScoreError::UnsupportedDType(d));
        }
        if d != dtype {
            return Err(bad("segments have different dtypes"));
        }
    }

    for s in segments {
        let layout = &s.segment.layout;
        if layout.rank() != 2 {
            return Err(bad(format!("rank {} instead of 2", layout.rank())));
        }
        if !layout.is_contiguous() {
            return Err(bad("layout is not contiguous"));
        }
    }

    let query_len = req.query.len();
    for s in segments {
        let dim = s.segment.layout.shape()[1];
        if dim != query_len {
            return Err(ScoreError::QueryDimMismatch {
                query: query_len,
                dim,
            });
        }
        if dim == 0 {
            return Err(bad("dimension 0"));
        }
        if u32::try_from(dim).is_err() {
            return Err(bad("dimension exceeds u32"));
        }
    }

    let size = elem_size(dtype);
    for s in segments {
        let seg = &s.segment;
        let bytes = layout_bytes(&seg.layout, size)?;
        if bytes > seg.region.len() {
            return Err(bad("layout is larger than the region"));
        }
        if seg.region.end() > seg.block.len_bytes {
            return Err(bad("region exceeds the device block"));
        }
    }

    for s in segments {
        let seg = &s.segment;
        let bytes = layout_bytes(&seg.layout, size)?;
        let start = region_addr(seg)?;
        if start % size != 0 {
            return Err(bad("region address is not element aligned"));
        }
        if start.checked_add(bytes).is_none() {
            return Err(bad("region address overflow"));
        }
    }

    for s in segments {
        let rows = s.segment.layout.shape()[0];
        let slots = &s.slots;
        if slots.start > slots.end || slots.end as usize > rows {
            return Err(ScoreError::OutOfBounds {
                slots: slots.clone(),
                rows,
            });
        }
    }

    Ok(Some((dtype, query_len as u32)))
}

fn layout_bytes(layout: &Layout, size: u64) -> Result<u64, ScoreError> {
    let shape = layout.shape();
    (shape[0] as u64)
        .checked_mul(shape[1] as u64)
        .and_then(|elems| elems.checked_mul(size))
        .ok_or_else(|| bad("layout byte length overflow"))
}

fn region_addr(seg: &ScoreSegment) -> Result<u64, ScoreError> {
    seg.block
        .base_addr
        .checked_add(seg.region.offset())
        .ok_or_else(|| bad("region address overflow"))
}

struct Plan<'a> {
    segments: Vec<LaunchSegment<'a>>,
    addrs: Vec<SlotAddr>,
}

fn plan(req: &ScoreRequest, dtype: DType, dim: u32) -> Result<Plan<'_>, ScoreError> {
    let row_bytes = u64::from(dim) * elem_size(dtype);
    let total: usize = req.segments.iter().map(|s| s.slots.len()).sum();
    let mut segments = Vec::new();
    let mut addrs = Vec::with_capacity(total);

    for s in &req.segments {
        if s.slots.is_empty() {
            continue;
        }
        let seg = &s.segment;
        let first_off = u64::from(s.slots.start)
            .checked_mul(row_bytes)
            .and_then(|o| o.checked_add(seg.region.offset()))
            .ok_or_else(|| bad("slot offset overflow"))?;
        let first_row_addr = seg
            .block
            .base_addr
            .checked_add(first_off)
            .ok_or_else(|| bad("row address overflow"))?;
        segments.push(LaunchSegment {
            block: &seg.block,
            first_row_addr,
            rows: s.slots.end - s.slots.start,
        });
        addrs.extend(s.slots.clone().map(|slot| {
            SlotAddr::new(
                seg.region.arena(),
                seg.region.offset() + u64::from(slot) * row_bytes,
            )
        }));
    }
    Ok(Plan { segments, addrs })
}

#[cfg(test)]
#[path = "test/kernel_scoring.rs"]
mod tests;
