use std::collections::HashMap;
use std::fmt;
use std::ops::Range;
use std::sync::Arc;
use std::time::Duration;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::a::kernel_batch::{
    BatchExecutor, BatchId, BatchInputs, BatchInterface, BatchMode, BatchSpec,
    LaunchOutcome, SegmentDesc, SegmentLayout,
};
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::a::library_adapter::{
    AdapterError, LibTensor,
};
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::a::kernel_layout::Slot5;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::a::region_handle::RegionHandle;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::a::supported_precisions::has_scoring_kernel;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::_1::device_lane_event::DeviceHandle;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::_1::layout::{
    DType, MASK_VALUE,
};
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::query_batch::{
    check_slot_range, DocSource, SegmentId, SourceError3,
};
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::topk::{Hit3, TopK};
const DEFAULT_TIME_BUDGET: Duration = Duration::from_secs(30);
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MaxSimError {
    Unavailable(String),
    Source(SourceError3),
    InvalidDim,
    Shape { what: &'static str, index: usize, len: u64, dim: usize },
    Mismatch(String),
    Launch(String),
}
impl fmt::Display for MaxSimError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MaxSimError::Unavailable(s) => write!(f, "max-sim unavailable: {s}"),
            MaxSimError::Source(e) => write!(f, "invalid slot range: {e}"),
            MaxSimError::InvalidDim => write!(f, "dim must be greater than zero"),
            MaxSimError::Shape { what, index, len, dim } => {
                write!(f, "{what} {index} has length {len}, not divisible by {dim}")
            }
            MaxSimError::Mismatch(s) => write!(f, "document mismatch: {s}"),
            MaxSimError::Launch(s) => write!(f, "max-sim launch failed: {s}"),
        }
    }
}
impl std::error::Error for MaxSimError {}
impl From<SourceError3> for MaxSimError {
    fn from(e: SourceError3) -> Self {
        MaxSimError::Source(e)
    }
}
impl From<AdapterError> for MaxSimError {
    fn from(e: AdapterError) -> Self {
        MaxSimError::Launch(e.to_string())
    }
}
pub struct DocSegment<'a> {
    pub desc: SegmentDesc,
    pub offsets: &'a [u64],
    pub tokens: &'a LibTensor,
}
pub struct TopKRequest<'a> {
    pub src: &'a dyn DocSource,
    pub segment: SegmentId,
    pub docs: DocSegment<'a>,
    pub range: Range<u32>,
    pub queries: &'a [&'a [f32]],
    pub dim: usize,
    pub k: usize,
}
pub trait MaxSimOp: Send + Sync {
    fn top_k(&self, req: &TopKRequest<'_>) -> Result<Vec<Vec<Hit3>>, MaxSimError>;
}
pub trait TileOps: Send + Sync {
    fn upload(
        &self,
        data: &[f32],
        rows: usize,
        cols: usize,
        dtype: DType,
    ) -> Result<LibTensor, AdapterError>;
    fn doc_tile_t(
        &self,
        docs: &LibTensor,
        row_start: usize,
        rows: usize,
        tile: usize,
    ) -> Result<LibTensor, AdapterError>;
    fn tile_mask(
        &self,
        valid: usize,
        tile: usize,
        dtype: DType,
    ) -> Result<LibTensor, AdapterError>;
    fn matmul(&self, a: &LibTensor, b: &LibTensor) -> Result<LibTensor, AdapterError>;
    fn add_broadcast(
        &self,
        scores: &LibTensor,
        mask: &LibTensor,
    ) -> Result<LibTensor, AdapterError>;
    fn max_last_dim(&self, scores: &LibTensor) -> Result<LibTensor, AdapterError>;
    fn maximum(&self, a: &LibTensor, b: &LibTensor) -> Result<LibTensor, AdapterError>;
    fn sum_f32(&self, v: &LibTensor) -> Result<f32, AdapterError>;
}
enum Bounds<'a> {
    Fixed { tokens_per_entry: u64 },
    Variable { offsets: &'a [u64], row_bytes: u64 },
}
struct Geometry2<'a> {
    bounds: Bounds<'a>,
}
impl Geometry2<'_> {
    /// (first row, row count) of a validated slot.
    fn rows(&self, slot: u32) -> (usize, usize) {
        match &self.bounds {
            Bounds::Fixed { tokens_per_entry } => {
                (
                    u64::from(slot).wrapping_mul(*tokens_per_entry) as usize,
                    *tokens_per_entry as usize,
                )
            }
            Bounds::Variable { offsets, row_bytes } => {
                let i = slot as usize;
                let start = offsets[i] / row_bytes;
                let end = offsets[i + 1] / row_bytes;
                (start as usize, (end - start) as usize)
            }
        }
    }
}
fn validate2<'a>(
    req: &TopKRequest<'a>,
    precision: DType,
) -> Result<Geometry2<'a>, MaxSimError> {
    let dim = req.dim;
    if dim == 0 {
        return Err(MaxSimError::InvalidDim);
    }
    check_slot_range(req.src, req.segment, req.range.clone())?;
    let desc = req.docs.desc;
    if u64::from(req.range.end) > desc.entries {
        return Err(
            MaxSimError::Source(SourceError3::SlotRange {
                segment: req.segment,
                start: req.range.start,
                end: req.range.end,
                slot_count: desc.entries.min(u64::from(u32::MAX)) as u32,
            }),
        );
    }
    for (index, q) in req.queries.iter().enumerate() {
        if q.len() % dim != 0 {
            return Err(MaxSimError::Shape {
                what: "query",
                index,
                len: q.len() as u64,
                dim,
            });
        }
    }
    let tokens = req.docs.tokens;
    let dtype = tokens.dtype().map_err(|e| MaxSimError::Mismatch(e.to_string()))?;
    if dtype != precision {
        return Err(
            MaxSimError::Mismatch(
                format!("doc dtype {dtype:?} != precision {precision:?}"),
            ),
        );
    }
    let dims = tokens.dims();
    if dims.len() != 2 {
        return Err(
            MaxSimError::Mismatch(
                format!("doc tensor has {} dims, expected 2", dims.len()),
            ),
        );
    }
    if dims[1] != dim {
        return Err(MaxSimError::Mismatch(format!("doc dim {} != {dim}", dims[1])));
    }
    if !desc.offsets_valid(req.docs.offsets) {
        return Err(MaxSimError::Mismatch("offset table invalid".to_string()));
    }
    let row_bytes = (dim as u64)
        .checked_mul(precision.size_in_bytes() as u64)
        .ok_or_else(|| MaxSimError::Mismatch("row size overflow".to_string()))?;
    let total_rows = dims[0] as u64;
    let non_empty = req.range.start < req.range.end;
    match desc.layout {
        SegmentLayout::Fixed { stride } => {
            if stride % row_bytes != 0 {
                return Err(MaxSimError::Shape {
                    what: "stride",
                    index: 0,
                    len: stride,
                    dim,
                });
            }
            let tokens_per_entry = stride / row_bytes;
            if non_empty {
                let needed = u64::from(req.range.end)
                    .checked_mul(tokens_per_entry)
                    .ok_or_else(|| MaxSimError::Mismatch(
                        "row count overflow".to_string(),
                    ))?;
                if needed > total_rows {
                    return Err(
                        MaxSimError::Mismatch(
                            format!("doc tensor has {total_rows} rows, {needed} needed"),
                        ),
                    );
                }
            }
            Ok(Geometry2 {
                bounds: Bounds::Fixed { tokens_per_entry },
            })
        }
        SegmentLayout::Variable { .. } => {
            let offsets = req.docs.offsets;
            let first = req.range.start as usize;
            let last = req.range.end as usize;
            for index in first..=last {
                if offsets[index] % row_bytes != 0 {
                    return Err(MaxSimError::Shape {
                        what: "offset",
                        index,
                        len: offsets[index],
                        dim,
                    });
                }
            }
            if non_empty && offsets[last] / row_bytes > total_rows {
                return Err(
                    MaxSimError::Mismatch(
                        format!(
                            "doc tensor has {total_rows} rows, {} needed", offsets[last]
                            / row_bytes
                        ),
                    ),
                );
            }
            Ok(Geometry2 {
                bounds: Bounds::Variable {
                    offsets,
                    row_bytes,
                },
            })
        }
    }
}
fn empty_result(req: &TopKRequest<'_>) -> bool {
    req.k == 0 || req.range.is_empty() || req.queries.is_empty()
}
fn collect_top_k(
    req: &TopKRequest<'_>,
    mut score_of: impl FnMut(u32) -> Result<Vec<f32>, MaxSimError>,
) -> Result<Vec<Vec<Hit3>>, MaxSimError> {
    let mut tops: Vec<TopK> = req.queries.iter().map(|_| TopK::new(req.k)).collect();
    for i in 0..(req.range.end - req.range.start) {
        let slot = req.range.start + i;
        let Some(offset) = req.src.slot_offset(req.segment, Slot5(slot)) else {
            continue;
        };
        let scores = score_of(i)?;
        for (top, score) in tops.iter_mut().zip(scores) {
            top.push(offset, score);
        }
    }
    Ok(tops.into_iter().map(TopK::into_sorted_vec).collect())
}
struct PreparedQuery {
    tokens: usize,
    tensor: Option<LibTensor>,
}
pub struct TiledMaxSim {
    device: DeviceHandle,
    precision: DType,
    tile_size: usize,
    ops: Arc<dyn TileOps>,
}
impl TiledMaxSim {
    pub fn new(
        device: DeviceHandle,
        precision: DType,
        tile_size: usize,
        open: &dyn Fn(DeviceHandle) -> Result<Arc<dyn TileOps>, String>,
    ) -> Result<TiledMaxSim, MaxSimError> {
        if !has_scoring_kernel(device.kind(), precision) {
            return Err(
                MaxSimError::Unavailable(
                    format!(
                        "precision {precision:?} not available on {:?}", device.kind()
                    ),
                ),
            );
        }
        let ops = open(device).map_err(MaxSimError::Unavailable)?;
        Ok(TiledMaxSim {
            device,
            precision,
            tile_size: tile_size.max(1),
            ops,
        })
    }
    pub fn tile_size(&self) -> usize {
        self.tile_size
    }
    pub fn precision(&self) -> DType {
        self.precision
    }
    pub fn device(&self) -> DeviceHandle {
        self.device
    }
    fn entry_scores(
        &self,
        docs: &LibTensor,
        queries: &[PreparedQuery],
        row_start: usize,
        rows: usize,
        full_masks: &mut HashMap<usize, LibTensor>,
    ) -> Result<Vec<f32>, MaxSimError> {
        if rows == 0 {
            return Ok(queries.iter().map(|q| MASK_VALUE * q.tokens as f32).collect());
        }
        let tile = self.tile_size.min(rows);
        let mut running: Vec<Option<LibTensor>> = queries.iter().map(|_| None).collect();
        let mut done = 0;
        while done < rows {
            let valid = (rows - done).min(tile);
            let docs_t = self.ops.doc_tile_t(docs, row_start + done, valid, tile)?;
            let partial_mask;
            let mask = if valid == tile {
                if !full_masks.contains_key(&tile) {
                    full_masks
                        .insert(tile, self.ops.tile_mask(tile, tile, self.precision)?);
                }
                &full_masks[&tile]
            } else {
                partial_mask = self.ops.tile_mask(valid, tile, self.precision)?;
                &partial_mask
            };
            for (slot, q) in running.iter_mut().zip(queries) {
                let Some(qt) = &q.tensor else { continue };
                let scores = self.ops.matmul(qt, &docs_t)?;
                let scores = self.ops.add_broadcast(&scores, mask)?;
                let tile_max = self.ops.max_last_dim(&scores)?;
                *slot = Some(
                    match slot.take() {
                        None => tile_max,
                        Some(prev) => self.ops.maximum(&prev, &tile_max)?,
                    },
                );
            }
            done += valid;
        }
        let mut out = Vec::with_capacity(queries.len());
        for slot in &running {
            out.push(
                match slot {
                    Some(v) => self.ops.sum_f32(v)?,
                    None => 0.0,
                },
            );
        }
        Ok(out)
    }
}
impl MaxSimOp for TiledMaxSim {
    fn top_k(&self, req: &TopKRequest<'_>) -> Result<Vec<Vec<Hit3>>, MaxSimError> {
        let geometry = validate2(req, self.precision)?;
        if empty_result(req) {
            return Ok(vec![Vec::new(); req.queries.len()]);
        }
        let mut prepared = Vec::with_capacity(req.queries.len());
        for q in req.queries {
            let tokens = q.len() / req.dim;
            let tensor = if tokens == 0 {
                None
            } else {
                Some(self.ops.upload(q, tokens, req.dim, self.precision)?)
            };
            prepared.push(PreparedQuery { tokens, tensor });
        }
        let mut full_masks = HashMap::new();
        collect_top_k(
            req,
            |i| {
                let (row_start, rows) = geometry.rows(req.range.start + i);
                self.entry_scores(
                    req.docs.tokens,
                    &prepared,
                    row_start,
                    rows,
                    &mut full_masks,
                )
            },
        )
    }
}
pub trait FusedIo: Send + Sync {
    fn upload_queries(
        &self,
        queries: &[&[f32]],
        dim: usize,
        precision: DType,
    ) -> Result<RegionHandle, String>;
    fn alloc_scores(&self, bytes: u64) -> Result<RegionHandle, String>;
    fn read_scores(
        &self,
        region: RegionHandle,
        f32_count: usize,
    ) -> Result<Vec<f32>, String>;
    fn release(&self, region: RegionHandle);
}
pub struct FusedMaxSim<E: BatchExecutor> {
    device: DeviceHandle,
    precision: DType,
    iface: Arc<BatchInterface<E>>,
    io: Arc<dyn FusedIo>,
    params: Option<RegionHandle>,
    time_budget: Duration,
}
impl<E: BatchExecutor> FusedMaxSim<E> {
    pub fn new(
        device: DeviceHandle,
        precision: DType,
        iface: Arc<BatchInterface<E>>,
        io: Arc<dyn FusedIo>,
        params: Option<RegionHandle>,
    ) -> Result<Self, MaxSimError> {
        if !has_scoring_kernel(device.kind(), precision) {
            return Err(
                MaxSimError::Unavailable(
                    format!(
                        "precision {precision:?} not available on {:?}", device.kind()
                    ),
                ),
            );
        }
        Ok(FusedMaxSim {
            device,
            precision,
            iface,
            io,
            params,
            time_budget: DEFAULT_TIME_BUDGET,
        })
    }
    pub fn with_time_budget(mut self, time_budget: Duration) -> Self {
        self.time_budget = time_budget;
        self
    }
    pub fn device(&self) -> DeviceHandle {
        self.device
    }
    pub fn precision(&self) -> DType {
        self.precision
    }
    /// Descriptor handed to the kernel, first scored slot, and scored slot count.
    fn kernel_segment(
        req: &TopKRequest<'_>,
    ) -> Result<(SegmentDesc, usize, usize), MaxSimError> {
        let desc = req.docs.desc;
        match desc.layout {
            SegmentLayout::Fixed { stride } => {
                let len = u64::from(req.range.end - req.range.start);
                let shift = u64::from(req.range.start)
                    .checked_mul(stride)
                    .and_then(|s| desc.base.checked_add(s))
                    .ok_or_else(|| MaxSimError::Mismatch(
                        "segment base overflow".to_string(),
                    ))?;
                let narrowed = SegmentDesc {
                    base: shift,
                    entries: len,
                    layout: desc.layout,
                };
                Ok((narrowed, 0, len as usize))
            }
            SegmentLayout::Variable { .. } => {
                Ok((desc, req.range.start as usize, desc.entries as usize))
            }
        }
    }
    fn run_batch(
        &self,
        spec: BatchSpec,
        queries_region: RegionHandle,
        scores_region: RegionHandle,
        f32_count: usize,
    ) -> Result<Vec<f32>, MaxSimError> {
        let launch = |e: String| MaxSimError::Launch(e);
        let id: BatchId = self
            .iface
            .begin(spec, BatchMode::Normal)
            .map_err(|e| launch(e.to_string()))?;
        let run = (|| {
            self.iface
                .assign_ranges(id, &[scores_region])
                .map_err(|e| launch(e.to_string()))?;
            let inputs = BatchInputs {
                regions: vec![queries_region],
            };
            match self.iface.run(id, &inputs).map_err(|e| launch(e.to_string()))? {
                LaunchOutcome::Hits { .. } => Ok(()),
                LaunchOutcome::Partial { queries_done, .. } => {
                    Err(launch(format!("partial result after {queries_done} queries")))
                }
            }
        })();
        let ended = self.iface.end(id).map_err(|e| launch(e.to_string()));
        run?;
        ended?;
        let scores = self.io.read_scores(scores_region, f32_count).map_err(launch)?;
        if scores.len() < f32_count {
            return Err(
                MaxSimError::Launch(
                    format!("expected {f32_count} scores, got {}", scores.len()),
                ),
            );
        }
        Ok(scores)
    }
}
impl<E: BatchExecutor> MaxSimOp for FusedMaxSim<E> {
    fn top_k(&self, req: &TopKRequest<'_>) -> Result<Vec<Vec<Hit3>>, MaxSimError> {
        validate2(req, self.precision)?;
        if empty_result(req) {
            return Ok(vec![Vec::new(); req.queries.len()]);
        }
        let query_count = u32::try_from(req.queries.len())
            .map_err(|_| MaxSimError::Launch("too many queries".to_string()))?;
        let (desc, skip, slots) = Self::kernel_segment(req)?;
        let f32_count = req
            .queries
            .len()
            .checked_mul(slots)
            .ok_or_else(|| MaxSimError::Launch("score count overflow".to_string()))?;
        let score_bytes = (f32_count as u64)
            .checked_mul(4)
            .ok_or_else(|| MaxSimError::Launch("score size overflow".to_string()))?;
        let queries_region = self
            .io
            .upload_queries(req.queries, req.dim, self.precision)
            .map_err(MaxSimError::Launch)?;
        let scores_region = match self.io.alloc_scores(score_bytes) {
            Ok(r) => r,
            Err(e) => {
                self.io.release(queries_region);
                return Err(MaxSimError::Launch(e));
            }
        };
        let spec = BatchSpec {
            segments: vec![desc],
            query_count,
            params: self.params,
            time_budget: self.time_budget,
        };
        let scores = self.run_batch(spec, queries_region, scores_region, f32_count);
        self.io.release(queries_region);
        self.io.release(scores_region);
        let scores = scores?;
        collect_top_k(
            req,
            |i| {
                let column = skip + i as usize;
                Ok((0..req.queries.len()).map(|q| scores[q * slots + column]).collect())
            },
        )
    }
}
#[cfg(test)]
#[path = "test/max_similarity_operation.rs"]
mod tests;
