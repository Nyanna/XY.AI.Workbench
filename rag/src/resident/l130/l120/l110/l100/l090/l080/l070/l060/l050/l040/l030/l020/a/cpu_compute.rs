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
        return Err(CpuComputeError::Shape {
            what: ShapeSource::Query(0),
            len: query.len(),
            dim,
        });
    }
    if doc.len() % dim != 0 {
        return Err(CpuComputeError::Shape {
            what: ShapeSource::Doc,
            len: doc.len(),
            dim,
        });
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
/// Squared Euclidean distance; panics if `a.len() != b.len()`.
pub fn sq_dist(a: &[f32], b: &[f32]) -> f32 {
    assert_eq!(a.len(), b.len(), "sq_dist: length mismatch");
    let mut lanes = [0.0f32; 8];
    let ca = a.chunks_exact(8);
    let cb = b.chunks_exact(8);
    let ra = ca.remainder();
    let rb = cb.remainder();
    for (x, y) in ca.zip(cb) {
        for j in 0..8 {
            let d = x[j] - y[j];
            lanes[j] += d * d;
        }
    }
    let mut rest = 0.0f32;
    for (x, y) in ra.iter().zip(rb) {
        let d = x - y;
        rest += d * d;
    }
    let l = lanes;
    (((l[0] + l[1]) + (l[2] + l[3])) + ((l[4] + l[5]) + (l[6] + l[7]))) + rest
}
/// Instruction set used by the vectorised row kernels.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Isa {
    Scalar,
    #[cfg(target_arch = "x86_64")]
    Avx2Fma,
    #[cfg(target_arch = "aarch64")]
    Neon,
}
impl Isa {
    pub fn detect() -> Isa {
        #[cfg(target_arch = "x86_64")]
        {
            if std::is_x86_feature_detected!("avx2")
                && std::is_x86_feature_detected!("fma")
            {
                return Isa::Avx2Fma;
            }
        }
        #[cfg(target_arch = "aarch64")]
        {
            if std::arch::is_aarch64_feature_detected!("neon") {
                return Isa::Neon;
            }
        }
        Isa::Scalar
    }
}
/// Dot product on `isa`; panics if `a.len() != b.len()`.
pub fn dot_with(isa: Isa, a: &[f32], b: &[f32]) -> f32 {
    assert_eq!(a.len(), b.len(), "dot: length mismatch");
    match isa {
        #[cfg(target_arch = "x86_64")]
        Isa::Avx2Fma => unsafe { x86::dot(a, b) }
        #[cfg(target_arch = "aarch64")]
        Isa::Neon => unsafe { neon::dot(a, b) }
        Isa::Scalar => dot(a, b),
    }
}
/// Squared Euclidean distance on `isa`; panics if `a.len() != b.len()`.
pub fn sq_dist_with(isa: Isa, a: &[f32], b: &[f32]) -> f32 {
    assert_eq!(a.len(), b.len(), "sq_dist: length mismatch");
    match isa {
        #[cfg(target_arch = "x86_64")]
        Isa::Avx2Fma => unsafe { x86::sq_dist(a, b) }
        #[cfg(target_arch = "aarch64")]
        Isa::Neon => unsafe { neon::sq_dist(a, b) }
        Isa::Scalar => sq_dist(a, b),
    }
}
#[cfg(target_arch = "x86_64")]
mod x86 {
    use std::arch::x86_64::*;
    #[target_feature(enable = "avx2,fma")]
    unsafe fn hsum(v: __m256) -> f32 {
        let lo = _mm256_castps256_ps128(v);
        let hi = _mm256_extractf128_ps::<1>(v);
        let s = _mm_add_ps(lo, hi);
        let s = _mm_add_ps(s, _mm_movehl_ps(s, s));
        let s = _mm_add_ss(s, _mm_shuffle_ps::<0x55>(s, s));
        _mm_cvtss_f32(s)
    }
    #[target_feature(enable = "avx2,fma")]
    pub(super) unsafe fn dot(a: &[f32], b: &[f32]) -> f32 {
        let n = a.len().min(b.len());
        let (pa, pb) = (a.as_ptr(), b.as_ptr());
        let mut acc0 = _mm256_setzero_ps();
        let mut acc1 = _mm256_setzero_ps();
        let mut i = 0;
        while i + 16 <= n {
            acc0 = _mm256_fmadd_ps(
                _mm256_loadu_ps(pa.add(i)),
                _mm256_loadu_ps(pb.add(i)),
                acc0,
            );
            acc1 = _mm256_fmadd_ps(
                _mm256_loadu_ps(pa.add(i + 8)),
                _mm256_loadu_ps(pb.add(i + 8)),
                acc1,
            );
            i += 16;
        }
        if i + 8 <= n {
            acc0 = _mm256_fmadd_ps(
                _mm256_loadu_ps(pa.add(i)),
                _mm256_loadu_ps(pb.add(i)),
                acc0,
            );
            i += 8;
        }
        let mut sum = hsum(_mm256_add_ps(acc0, acc1));
        while i < n {
            sum += *pa.add(i) * *pb.add(i);
            i += 1;
        }
        sum
    }
    #[target_feature(enable = "avx2,fma")]
    pub(super) unsafe fn sq_dist(a: &[f32], b: &[f32]) -> f32 {
        let n = a.len().min(b.len());
        let (pa, pb) = (a.as_ptr(), b.as_ptr());
        let mut acc0 = _mm256_setzero_ps();
        let mut acc1 = _mm256_setzero_ps();
        let mut i = 0;
        while i + 16 <= n {
            let d0 = _mm256_sub_ps(
                _mm256_loadu_ps(pa.add(i)),
                _mm256_loadu_ps(pb.add(i)),
            );
            let d1 = _mm256_sub_ps(
                _mm256_loadu_ps(pa.add(i + 8)),
                _mm256_loadu_ps(pb.add(i + 8)),
            );
            acc0 = _mm256_fmadd_ps(d0, d0, acc0);
            acc1 = _mm256_fmadd_ps(d1, d1, acc1);
            i += 16;
        }
        if i + 8 <= n {
            let d = _mm256_sub_ps(
                _mm256_loadu_ps(pa.add(i)),
                _mm256_loadu_ps(pb.add(i)),
            );
            acc0 = _mm256_fmadd_ps(d, d, acc0);
            i += 8;
        }
        let mut sum = hsum(_mm256_add_ps(acc0, acc1));
        while i < n {
            let d = *pa.add(i) - *pb.add(i);
            sum += d * d;
            i += 1;
        }
        sum
    }
}
#[cfg(target_arch = "aarch64")]
mod neon {
    use std::arch::aarch64::*;
    #[target_feature(enable = "neon")]
    pub(super) unsafe fn dot(a: &[f32], b: &[f32]) -> f32 {
        let n = a.len().min(b.len());
        let (pa, pb) = (a.as_ptr(), b.as_ptr());
        let mut acc0 = vdupq_n_f32(0.0);
        let mut acc1 = vdupq_n_f32(0.0);
        let mut i = 0;
        while i + 8 <= n {
            acc0 = vfmaq_f32(acc0, vld1q_f32(pa.add(i)), vld1q_f32(pb.add(i)));
            acc1 = vfmaq_f32(acc1, vld1q_f32(pa.add(i + 4)), vld1q_f32(pb.add(i + 4)));
            i += 8;
        }
        if i + 4 <= n {
            acc0 = vfmaq_f32(acc0, vld1q_f32(pa.add(i)), vld1q_f32(pb.add(i)));
            i += 4;
        }
        let mut sum = vaddvq_f32(vaddq_f32(acc0, acc1));
        while i < n {
            sum += *pa.add(i) * *pb.add(i);
            i += 1;
        }
        sum
    }
    #[target_feature(enable = "neon")]
    pub(super) unsafe fn sq_dist(a: &[f32], b: &[f32]) -> f32 {
        let n = a.len().min(b.len());
        let (pa, pb) = (a.as_ptr(), b.as_ptr());
        let mut acc0 = vdupq_n_f32(0.0);
        let mut acc1 = vdupq_n_f32(0.0);
        let mut i = 0;
        while i + 8 <= n {
            let d0 = vsubq_f32(vld1q_f32(pa.add(i)), vld1q_f32(pb.add(i)));
            let d1 = vsubq_f32(vld1q_f32(pa.add(i + 4)), vld1q_f32(pb.add(i + 4)));
            acc0 = vfmaq_f32(acc0, d0, d0);
            acc1 = vfmaq_f32(acc1, d1, d1);
            i += 8;
        }
        if i + 4 <= n {
            let d = vsubq_f32(vld1q_f32(pa.add(i)), vld1q_f32(pb.add(i)));
            acc0 = vfmaq_f32(acc0, d, d);
            i += 4;
        }
        let mut sum = vaddvq_f32(vaddq_f32(acc0, acc1));
        while i < n {
            let d = *pa.add(i) - *pb.add(i);
            sum += d * d;
            i += 1;
        }
        sum
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
            return Err(CpuComputeError::Shape {
                what: ShapeSource::Query(i),
                len: q.len(),
                dim,
            });
        }
    }
    let n = range.len();
    let mut out = vec![0.0f32; queries.len() * n];
    for slot in range.clone() {
        let doc: &[f32] = slots[slot].unwrap_or(&[]);
        if doc.len() % dim != 0 {
            return Err(CpuComputeError::Shape {
                what: ShapeSource::Slot(slot),
                len: doc.len(),
                dim,
            });
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
