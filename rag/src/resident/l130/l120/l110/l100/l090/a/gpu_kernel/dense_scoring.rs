use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::fmt;

use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::a::kernel_layout::Slot;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::a::cpu_compute::dot;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::a::index_contract::{
    Index, IndexError, NodeId,
};
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::l010::layout::DType;

/// Squared query norm below which a query counts as an empty embedding.
const EMPTY_NORM_SQ: f32 = 1e-18;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimilarityMode {
    Dot,
    Exact,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShapeSource {
    Query(usize),
    /// i-th slot, or i-th node when scoring centroids.
    Slot(usize),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DenseError {
    InvalidDim,
    /// Length differs from `dim`, or is zero.
    Shape {
        what: ShapeSource,
        len: usize,
        dim: usize,
    },
    EmptyEmbedding {
        query_index: usize,
    },
    UnsupportedDType(DType),
    Index(IndexError),
}

impl fmt::Display for DenseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DenseError::InvalidDim => write!(f, "invalid dimension: dim must be non-zero"),
            DenseError::Shape { what, len, dim } => match what {
                ShapeSource::Query(i) => {
                    write!(f, "query {i} has length {len}, expected {dim}")
                }
                ShapeSource::Slot(i) => {
                    write!(f, "slot {i} has length {len}, expected {dim}")
                }
            },
            DenseError::EmptyEmbedding { query_index } => {
                write!(f, "leeres Embedding (query {query_index})")
            }
            DenseError::UnsupportedDType(d) => {
                write!(f, "unsupported centroid dtype {d:?}")
            }
            DenseError::Index(e) => write!(f, "index error: {e}"),
        }
    }
}

impl std::error::Error for DenseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            DenseError::Index(e) => Some(e),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Hit {
    pub slot: Slot,
    pub score: f32,
}

/// Row-major: index = query * slot_count + slot.
#[derive(Debug, Clone, PartialEq)]
pub struct DenseScores {
    queries: usize,
    slots: usize,
    data: Vec<f32>,
}

impl DenseScores {
    pub fn query_count(&self) -> usize {
        self.queries
    }

    pub fn slot_count(&self) -> usize {
        self.slots
    }

    pub fn row(&self, query: usize) -> Option<&[f32]> {
        if query >= self.queries {
            return None;
        }
        let start = query * self.slots;
        self.data.get(start..start + self.slots)
    }

    pub fn as_flat(&self) -> &[f32] {
        &self.data
    }

    pub fn top_k(&self, query: usize, k: usize) -> Option<Vec<Hit>> {
        self.row(query).map(|row| top_k(row, k))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CentroidScores {
    pub nodes: Vec<NodeId>,
    /// Column `i` belongs to `nodes[i]`.
    pub scores: DenseScores,
}

/// Heap candidate; the greatest element is the worst hit.
#[derive(Clone, Copy)]
struct Candidate {
    score: f32,
    index: u32,
}

impl Candidate {
    fn worse_cmp(&self, other: &Self) -> Ordering {
        other
            .score
            .partial_cmp(&self.score)
            .unwrap_or(Ordering::Equal)
            .then(self.index.cmp(&other.index))
    }
}

impl PartialEq for Candidate {
    fn eq(&self, other: &Self) -> bool {
        self.worse_cmp(other) == Ordering::Equal
    }
}

impl Eq for Candidate {}

impl PartialOrd for Candidate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Candidate {
    fn cmp(&self, other: &Self) -> Ordering {
        self.worse_cmp(other)
    }
}

/// Top-k of one score row: descending by score, ties by smaller index, NaN skipped.
pub fn top_k(scores: &[f32], k: usize) -> Vec<Hit> {
    if k == 0 {
        return Vec::new();
    }
    let mut heap: BinaryHeap<Candidate> = BinaryHeap::with_capacity(k.min(scores.len()).saturating_add(1));
    for (i, &score) in scores.iter().enumerate().take(u32::MAX as usize + 1) {
        if score.is_nan() {
            continue;
        }
        let cand = Candidate { score, index: i as u32 };
        if heap.len() < k {
            heap.push(cand);
        } else if let Some(mut worst) = heap.peek_mut() {
            if cand < *worst {
                *worst = cand;
            }
        }
    }
    heap.into_sorted_vec()
        .into_iter()
        .map(|c| Hit { slot: Slot(c.index), score: c.score })
        .collect()
}

/// Validates dim and queries (steps 1-3); returns the squared norm of each query.
fn validate_queries(queries: &[&[f32]], dim: usize) -> Result<Vec<f32>, DenseError> {
    if dim == 0 {
        return Err(DenseError::InvalidDim);
    }
    for (i, q) in queries.iter().enumerate() {
        if q.len() != dim {
            return Err(DenseError::Shape { what: ShapeSource::Query(i), len: q.len(), dim });
        }
    }
    let mut norms = Vec::with_capacity(queries.len());
    for (i, q) in queries.iter().enumerate() {
        let n2 = dot(q, q);
        if !n2.is_finite() || n2 < EMPTY_NORM_SQ {
            return Err(DenseError::EmptyEmbedding { query_index: i });
        }
        norms.push(n2);
    }
    Ok(norms)
}

fn compute(
    queries: &[&[f32]],
    query_norms_sq: &[f32],
    slots: &[&[f32]],
    mode: SimilarityMode,
) -> DenseScores {
    let mut data = Vec::with_capacity(queries.len().saturating_mul(slots.len()));
    match mode {
        SimilarityMode::Dot => {
            for q in queries {
                data.extend(slots.iter().map(|s| dot(q, s)));
            }
        }
        SimilarityMode::Exact => {
            let slot_norms: Vec<f32> = slots.iter().map(|s| dot(s, s).sqrt()).collect();
            for (q, n2) in queries.iter().zip(query_norms_sq) {
                let qn = n2.sqrt();
                data.extend(slots.iter().zip(&slot_norms).map(|(s, sn)| {
                    let denom = qn * sn;
                    if !(denom > 0.0) || !denom.is_finite() {
                        0.0
                    } else {
                        dot(q, s) / denom
                    }
                }));
            }
        }
    }
    DenseScores { queries: queries.len(), slots: slots.len(), data }
}

fn check_slots(slots: &[&[f32]], dim: usize) -> Result<(), DenseError> {
    for (j, s) in slots.iter().enumerate() {
        if s.len() != dim {
            return Err(DenseError::Shape { what: ShapeSource::Slot(j), len: s.len(), dim });
        }
    }
    Ok(())
}

/// Cosine score per query and slot.
pub fn score(
    queries: &[&[f32]],
    slots: &[&[f32]],
    dim: usize,
    mode: SimilarityMode,
) -> Result<DenseScores, DenseError> {
    let norms = validate_queries(queries, dim)?;
    check_slots(slots, dim)?;
    Ok(compute(queries, &norms, slots, mode))
}

fn f16_to_f32(bits: u16) -> f32 {
    let sign = (u32::from(bits) & 0x8000) << 16;
    let exp = u32::from((bits >> 10) & 0x1f);
    let man = u32::from(bits & 0x3ff);
    match exp {
        0 => {
            let magnitude = man as f32 * (1.0 / 16_777_216.0);
            f32::from_bits(magnitude.to_bits() | sign)
        }
        31 => f32::from_bits(sign | 0x7f80_0000 | (man << 13)),
        _ => f32::from_bits(sign | ((exp + 112) << 23) | (man << 13)),
    }
}

fn decode_centroid(
    bytes: &[u8],
    dtype: DType,
    node: usize,
    dim: usize,
    out: &mut Vec<f32>,
) -> Result<(), DenseError> {
    let elem = dtype.size_in_bytes();
    let len = bytes.len() / elem;
    if bytes.len() % elem != 0 || len != dim {
        return Err(DenseError::Shape { what: ShapeSource::Slot(node), len, dim });
    }
    match dtype {
        DType::F32 => out.extend(
            bytes
                .chunks_exact(4)
                .map(|c| f32::from_le_bytes([c[0], c[1], c[2], c[3]])),
        ),
        _ => out.extend(
            bytes
                .chunks_exact(2)
                .map(|c| f16_to_f32(u16::from_le_bytes([c[0], c[1]]))),
        ),
    }
    Ok(())
}

/// Scores queries against the centroids of `nodes` (column `i` is `nodes[i]`).
pub fn score_centroids(
    index: &dyn Index,
    nodes: &[NodeId],
    queries: &[&[f32]],
    dim: usize,
    mode: SimilarityMode,
) -> Result<CentroidScores, DenseError> {
    let norms = validate_queries(queries, dim)?;
    let mut flat: Vec<f32> = Vec::new();
    if !nodes.is_empty() {
        let dtype = index.dtype();
        if !matches!(dtype, DType::F32 | DType::F16) {
            return Err(DenseError::UnsupportedDType(dtype));
        }
        for (i, node) in nodes.iter().enumerate() {
            let summary = index.summary(*node).map_err(DenseError::Index)?;
            decode_centroid(&summary.centroid, dtype, i, dim, &mut flat)?;
        }
    }
    let slots: Vec<&[f32]> = flat.chunks_exact(dim).collect();
    let scores = compute(queries, &norms, &slots, mode);
    Ok(CentroidScores { nodes: nodes.to_vec(), scores })
}

#[cfg(test)]
#[path = "test/dense_scoring.rs"]
mod tests;
