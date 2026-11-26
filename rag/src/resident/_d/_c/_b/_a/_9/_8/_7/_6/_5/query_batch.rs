use std::collections::HashMap;
use std::fmt;
use std::ops::Range;
pub use super::a::segments::SegmentId;
use super::_4::_3::a::kernel_layout::Slot;
const F32_BYTES: u64 = 4;
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ShapeError {
    ZeroDim,
    EmptyData,
    NotMultipleOfDim { len: usize, dim: u32 },
    DimMismatch { expected: u32, got: u32 },
}
impl fmt::Display for ShapeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ShapeError::ZeroDim => write!(f, "dimension must not be zero"),
            ShapeError::EmptyData => write!(f, "query data must not be empty"),
            ShapeError::NotMultipleOfDim { len, dim } => {
                write!(f, "data length {len} is not a multiple of dimension {dim}")
            }
            ShapeError::DimMismatch { expected, got } => {
                write!(f, "dimension mismatch: expected {expected}, got {got}")
            }
        }
    }
}
impl std::error::Error for ShapeError {}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SourceError {
    SlotRange { segment: SegmentId, start: u32, end: u32, slot_count: u32 },
}
impl fmt::Display for SourceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SourceError::SlotRange { segment, start, end, slot_count } => {
                write!(
                    f,
                    "slot range {start}..{end} is outside segment {} with {slot_count} slots",
                    segment.0
                )
            }
        }
    }
}
impl std::error::Error for SourceError {}
/// Token matrix (tokens x dim), row-major.
#[derive(Debug, Clone, PartialEq)]
pub struct Query {
    dim: u32,
    data: Vec<f32>,
}
impl Query {
    pub fn new(dim: u32, data: Vec<f32>) -> Result<Query, ShapeError> {
        if dim == 0 {
            return Err(ShapeError::ZeroDim);
        }
        if data.is_empty() {
            return Err(ShapeError::EmptyData);
        }
        if data.len() % dim as usize != 0 {
            return Err(ShapeError::NotMultipleOfDim {
                len: data.len(),
                dim,
            });
        }
        Ok(Query { dim, data })
    }
    pub fn dim(&self) -> u32 {
        self.dim
    }
    pub fn tokens(&self) -> usize {
        self.data.len() / self.dim as usize
    }
    pub fn data(&self) -> &[f32] {
        &self.data
    }
}
/// Queries of one common dimension, in insertion order.
#[derive(Debug, Clone, PartialEq)]
pub struct QueryBatch {
    dim: u32,
    queries: Vec<Query>,
}
impl QueryBatch {
    pub fn new(dim: u32) -> Result<QueryBatch, ShapeError> {
        if dim == 0 {
            return Err(ShapeError::ZeroDim);
        }
        Ok(QueryBatch {
            dim,
            queries: Vec::new(),
        })
    }
    pub fn from_queries(
        dim: u32,
        queries: Vec<Query>,
    ) -> Result<QueryBatch, ShapeError> {
        let mut batch = QueryBatch::new(dim)?;
        batch.queries.reserve(queries.len());
        for q in queries {
            batch.push(q)?;
        }
        Ok(batch)
    }
    pub fn push(&mut self, q: Query) -> Result<(), ShapeError> {
        if q.dim() != self.dim {
            return Err(ShapeError::DimMismatch {
                expected: self.dim,
                got: q.dim(),
            });
        }
        self.queries.push(q);
        Ok(())
    }
    pub fn dim(&self) -> u32 {
        self.dim
    }
    pub fn len(&self) -> usize {
        self.queries.len()
    }
    pub fn is_empty(&self) -> bool {
        self.queries.is_empty()
    }
    pub fn get(&self, i: usize) -> Option<&Query> {
        self.queries.get(i)
    }
    pub fn tokens(&self, i: usize) -> Option<usize> {
        self.get(i).map(Query::tokens)
    }
    pub fn data(&self, i: usize) -> Option<&[f32]> {
        self.get(i).map(Query::data)
    }
    pub fn iter(&self) -> std::slice::Iter<'_, Query> {
        self.queries.iter()
    }
}
impl<'a> IntoIterator for &'a QueryBatch {
    type Item = &'a Query;
    type IntoIter = std::slice::Iter<'a, Query>;
    fn into_iter(self) -> Self::IntoIter {
        self.queries.iter()
    }
}
/// Read-only source of document vectors.
pub trait DocSource: Send + Sync {
    fn dim(&self) -> u32;
    /// Unknown segment yields 0.
    fn slot_count(&self, segment: SegmentId) -> u32;
    /// `None` for an unknown segment or `slot >= slot_count`.
    fn slot_offset(&self, segment: SegmentId, slot: Slot) -> Option<u64>;
    /// Vectors of the valid tokens (`tokens * dim` values, row-major); `None` when absent.
    fn slot_vectors(&self, segment: SegmentId, slot: Slot) -> Option<&[f32]>;
}
/// Ok iff `range.start <= range.end <= src.slot_count(segment)`.
pub fn check_slot_range(
    src: &dyn DocSource,
    segment: SegmentId,
    range: Range<u32>,
) -> Result<(), SourceError> {
    let slot_count = src.slot_count(segment);
    if range.start <= range.end && range.end <= slot_count {
        Ok(())
    } else {
        Err(SourceError::SlotRange {
            segment,
            start: range.start,
            end: range.end,
            slot_count,
        })
    }
}
#[derive(Debug, Clone)]
pub struct RamSource {
    dim: u32,
    segments: HashMap<SegmentId, Vec<Option<Vec<f32>>>>,
    offsets: HashMap<SegmentId, Vec<u64>>,
}
impl RamSource {
    pub fn new(dim: u32) -> Result<RamSource, ShapeError> {
        if dim == 0 {
            return Err(ShapeError::ZeroDim);
        }
        Ok(RamSource {
            dim,
            segments: HashMap::new(),
            offsets: HashMap::new(),
        })
    }
    /// Replaces an existing segment; `None` marks an absent slot.
    pub fn insert_segment(
        &mut self,
        segment: SegmentId,
        slots: Vec<Option<Vec<f32>>>,
    ) -> Result<(), ShapeError> {
        let dim = self.dim as usize;
        for v in slots.iter().flatten() {
            if v.len() % dim != 0 {
                return Err(ShapeError::NotMultipleOfDim {
                    len: v.len(),
                    dim: self.dim,
                });
            }
        }
        let mut offsets = Vec::with_capacity(slots.len());
        let mut running: u64 = 0;
        for slot in &slots {
            offsets.push(running);
            let bytes = slot.as_ref().map_or(0, |v| v.len() as u64 * F32_BYTES);
            running = running.saturating_add(bytes);
        }
        self.offsets.insert(segment, offsets);
        self.segments.insert(segment, slots);
        Ok(())
    }
}
impl DocSource for RamSource {
    fn dim(&self) -> u32 {
        self.dim
    }
    fn slot_count(&self, segment: SegmentId) -> u32 {
        self.segments
            .get(&segment)
            .map_or(0, |s| u32::try_from(s.len()).unwrap_or(u32::MAX))
    }
    fn slot_offset(&self, segment: SegmentId, slot: Slot) -> Option<u64> {
        self.offsets.get(&segment)?.get(slot.0 as usize).copied()
    }
    fn slot_vectors(&self, segment: SegmentId, slot: Slot) -> Option<&[f32]> {
        self.segments.get(&segment)?.get(slot.0 as usize)?.as_deref()
    }
}
/// Half-open slot range `start..end`; the unit of one kernel launch.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Workspace {
    start: u32,
    end: u32,
}
impl Workspace {
    pub fn new(start: u32, end: u32) -> Workspace {
        Workspace {
            start,
            end: end.max(start),
        }
    }
    pub fn start(&self) -> u32 {
        self.start
    }
    pub fn end(&self) -> u32 {
        self.end
    }
    pub fn len(&self) -> u32 {
        self.end - self.start
    }
    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }
    pub fn range(&self) -> Range<u32> {
        self.start..self.end
    }
    /// Returns `(front, rest)` with `front.len() <= max_len` and `front ++ rest == self`.
    pub fn split_front(self, max_len: u32) -> (Workspace, Workspace) {
        let mid = self.start + self.len().min(max_len);
        (
            Workspace {
                start: self.start,
                end: mid,
            },
            Workspace {
                start: mid,
                end: self.end,
            },
        )
    }
}
#[cfg(test)]
#[path = "test/query_batch.rs"]
mod tests;
