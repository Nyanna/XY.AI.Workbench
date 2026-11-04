//! Capability values for which a scoring kernel exists, independent of the `cuda` feature.

use crate::resident::memory::DType;
use crate::resident::precision::known::{FP16, FP32, OPERANDS, SCORING};
use crate::resident::precision::{CapabilitySet, ValueId};
use crate::resident::subengine::SubengineKind;

/// Element types the NVRTC loader in `cuda.rs` can compile a kernel for.
pub const CUDA_KERNEL_DTYPES: [DType; 2] = [DType::F32, DType::F16];

/// Element types the host scoring loops in `cpu_simd.rs` operate on.
pub const CPU_KERNEL_DTYPES: [DType; 1] = [DType::F32];

fn dtype_value(dtype: DType) -> Option<&'static ValueId> {
    match dtype {
        DType::F32 => Some(&FP32),
        DType::F16 => Some(&FP16),
        _ => None,
    }
}

/// Axes governed by the kernels: operand and scoring values need an executable kernel.
/// Other axes (accumulator, fusion, batch, tile, threads) are not restricted by the catalog.
pub fn implemented_capabilities(kind: SubengineKind) -> CapabilitySet {
    let dtypes: &[DType] = match kind {
        SubengineKind::Cpu => &CPU_KERNEL_DTYPES,
        SubengineKind::Cuda => &CUDA_KERNEL_DTYPES,
    };
    let mut set = CapabilitySet::new();
    for value in dtypes.iter().filter_map(|d| dtype_value(*d)) {
        set.insert(&*OPERANDS, value);
        set.insert(&*SCORING, value);
    }
    set
}
