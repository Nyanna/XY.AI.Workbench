use std::fmt;
use std::ops::Range;

use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::l010::layout::MASK_VALUE;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CpuComputeError {
    InvalidDim,
    RangeOutOfBounds { start: usize, end: usize, slots: usize },
    Shape { what: ShapeSource, len: usize, dim: usize },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShapeSource {
    Query(usize),
    Slot(usize),
    Doc,
}

impl fmt::Display for ShapeSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ShapeSource::Query(i) => write!(f, "query {i}"),
            ShapeSource::Slot(i) => write!(f, "slot {i}"),
            ShapeSource::Doc => write!(f, "doc"),
        }
    }
}

impl fmt::Display for CpuComputeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CpuComputeError::InvalidDim => write!(f, "dim must be non-zero"),
            CpuComputeError::RangeOutOfBounds { start, end, slots } => {
                write!(f, "slot range {start}..{end} invalid for {slots} slots")
            }
            CpuComputeError::Shape { what, len, dim } => {
                write!(f, "{what} length {len} is not divisible by dim {dim}")
            }
        }
    }
}

impl std::error::Error for CpuComputeError {}

/// Panics if `a.len() != b.len()`.
pub fn dot(a: &[f32], b: &[f32]) -> f32 {
    assert_eq!(a.len(), b.len(), "dot: length mismatch");
    let mut lanes = [0.0f32; 8];
    let ca = a.chunks_exact(8);
    let cb = b.chunks_exact(8);
    let ra = ca.remainder();
    let rb = cb.remainder();
    for (x, y) in ca.zip(cb) {
        for j in 0..8 {
            lanes[j] += x[j] * y[j];
        }
    }
    let mut rest = 0.0f32;
    for (x, y) in ra.iter().zip(rb) {
        rest += x * y;
    }
    let l = lanes;
    (((l[0] + l[1]) + (l[2] + l[3])) + ((l[4] + l[5]) + (l[6] + l[7]))) + rest
}

fn max_sim_checked(query: &[f32], doc: &[f32], dim: usize) -> f32 {
    let q_tokens = query.len() / dim;
    if q_tokens == 0 {
        return 0.0;
    }
    if doc.is_empty() {
        return MASK_VALUE * q_tokens as f32;
    }
    let mut total = 0.0f32;
    for q in query.chunks_exact(dim) {
        let mut m = f32::NEG_INFINITY;
        for d in doc.chunks_exact(dim) {
            let s = dot(q, d);
            if s > m {
                m = s;
            }
        }
        total += m;
    }
    total
}

pub fn max_sim(query: &[f32], doc: &[f32], dim: usize) -> Result<f32, CpuComputeError> {
    if dim == 0 {
        return Err(CpuComputeError::InvalidDim);
    }
    if query.len() % dim != 0 {
        return Err(CpuComputeError::Shape { what: ShapeSource::Query(0), len: query.len(), dim });
    }
    if doc.len() % dim != 0 {
        return Err(CpuComputeError::Shape { what: ShapeSource::Doc, len: doc.len(), dim });
    }
    Ok(max_sim_checked(query, doc, dim))
}

pub fn l2_normalize(v: &mut [f32]) {
    let norm = v.iter().map(|x| x * x).sum::<f32>().sqrt();
    if norm == 0.0 {
        return;
    }
    for x in v.iter_mut() {
        *x /= norm;
    }
}

pub fn scan_slots(
    queries: &[&[f32]],
    slots: &[Option<&[f32]>],
    range: Range<usize>,
    dim: usize,
) -> Result<Vec<f32>, CpuComputeError> {
    if dim == 0 {
        return Err(CpuComputeError::InvalidDim);
    }
    if range.start > range.end || range.end > slots.len() {
        return Err(CpuComputeError::RangeOutOfBounds {
            start: range.start,
            end: range.end,
            slots: slots.len(),
        });
    }
    for (i, q) in queries.iter().enumerate() {
        if q.len() % dim != 0 {
            return Err(CpuComputeError::Shape { what: ShapeSource::Query(i), len: q.len(), dim });
        }
    }
    let n = range.len();
    let mut out = vec![0.0f32; queries.len() * n];
    for slot in range.clone() {
        let doc: &[f32] = slots[slot].unwrap_or(&[]);
        if doc.len() % dim != 0 {
            return Err(CpuComputeError::Shape { what: ShapeSource::Slot(slot), len: doc.len(), dim });
        }
        for (qi, q) in queries.iter().enumerate() {
            out[qi * n + (slot - range.start)] = max_sim_checked(q, doc, dim);
        }
    }
    Ok(out)
}

#[cfg(test)]
#[path = "test/cpu_compute.rs"]
mod tests;
