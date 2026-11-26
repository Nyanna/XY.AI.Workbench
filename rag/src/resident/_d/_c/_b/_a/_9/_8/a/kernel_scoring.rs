//! Score metrics and the JIT sources of the score kernels.
use crate::resident::_d::_c::_b::_a::_9::_8::_7::a::kernel_register::{
    KernelError, KernelRegister, KernelSource,
};
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::_1::layout::DType;
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
/// Registers the `score_dot` and `score_negl2` JIT sources.
pub fn register_score_kernels(register: &KernelRegister) -> Result<(), KernelError> {
    register
        .register_source(KernelSource {
            name: KERNEL_SCORE_DOT,
            body: SCORE_DOT_BODY,
            entry: ENTRY_SCORE_DOT,
            dtypes: SCORE_DTYPES,
            precompiled: None,
        })?;
    register
        .register_source(KernelSource {
            name: KERNEL_SCORE_NEGL2,
            body: SCORE_NEGL2_BODY,
            entry: ENTRY_SCORE_NEGL2,
            dtypes: SCORE_DTYPES,
            precompiled: None,
        })
}
#[cfg(test)]
#[path = "test/kernel_scoring.rs"]
mod tests;
