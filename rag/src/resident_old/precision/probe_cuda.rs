//! CUDA capability probe. The hardware table is pure and testable without a GPU; only reading
//! the device requires the `cuda` feature.

use crate::resident::error::PrecisionError;
use crate::resident::subengine::{DeviceHandle, SubengineKind};

use super::capability::CapabilitySet;
use super::known::{
    ACCUMULATOR, BATCH, FP16, FP32, FUSION, INT8_DP4A, OFF, ON, OPERANDS, SCORING, TILE,
};
use super::probe::{numeric_axis, CapabilityProbe, ProbeConfig};

/// Device names without tensor cores although they report compute capability 7.x (TU116/TU117).
/// Tensor cores are never derived from the capability alone.
const NO_TENSOR_CORE_NAMES: &[&str] = &[
    "GTX 16", "GTX 1650", "MX450", "MX550", "T400", "T500", "T600", "T1000", "T1200",
];

pub fn has_tensor_cores(major: i32, name: &str) -> bool {
    major >= 7 && !NO_TENSOR_CORE_NAMES.iter().any(|n| name.contains(n))
}

/// Optimistic capability set of a CUDA device. No tensor-core (WMMA/MMA) path and no bf16 are
/// offered: bf16 is not native before sm_80 and WMMA ran an order of magnitude below f16x2 FMA on
/// TU116. The benefit of `int8-dp4a` in a real scoring kernel is left to calibration.
pub fn cuda_capabilities(major: i32, minor: i32, cfg: &ProbeConfig) -> CapabilitySet {
    let cc = (major, minor);
    let mut set = CapabilitySet::new();
    set.insert(&*OPERANDS, &*FP32);
    set.insert(&*ACCUMULATOR, &*FP32);
    set.insert(&*SCORING, &*FP32);
    if cc >= (5, 3) {
        set.insert(&*OPERANDS, &*FP16);
        set.insert(&*ACCUMULATOR, &*FP16);
        set.insert(&*SCORING, &*FP16);
    }
    if cc >= (6, 1) {
        set.insert(&*SCORING, &*INT8_DP4A);
    }
    set.insert(&*FUSION, &*ON);
    set.insert(&*FUSION, &*OFF);
    numeric_axis(&mut set, &BATCH, cfg.batch_values());
    numeric_axis(&mut set, &TILE, cfg.tile_values());
    set
}

pub struct CudaProbe {
    #[cfg_attr(not(feature = "cuda"), allow(dead_code))]
    device: DeviceHandle,
    #[cfg_attr(not(feature = "cuda"), allow(dead_code))]
    cfg: ProbeConfig,
}

pub(super) fn cuda_probe(device: &DeviceHandle, cfg: ProbeConfig) -> Result<CudaProbe, PrecisionError> {
    #[cfg(feature = "cuda")]
    {
        Ok(CudaProbe { device: device.clone(), cfg })
    }
    #[cfg(not(feature = "cuda"))]
    {
        let _ = (device, cfg);
        Err(PrecisionError::Capability { reason: "cuda feature disabled".into() })
    }
}

impl CapabilityProbe for CudaProbe {
    fn kind(&self) -> SubengineKind {
        SubengineKind::Cuda
    }

    #[cfg(feature = "cuda")]
    fn probe(&self) -> Result<CapabilitySet, PrecisionError> {
        let fail = |e: cudarc::driver::DriverError| PrecisionError::Capability { reason: e.to_string() };
        let ctx = self.device.cuda_context().ok_or_else(|| PrecisionError::Capability {
            reason: "device has no cuda context".into(),
        })?;
        let (major, minor) = ctx.compute_capability().map_err(fail)?;
        let name = ctx.name().map_err(fail)?;
        let set = cuda_capabilities(major, minor, &self.cfg);
        tracing::info!(
            device = %name,
            cc = %format!("{major}.{minor}"),
            tensor_cores = has_tensor_cores(major, &name),
            "cuda capabilities"
        );
        Ok(set)
    }

    #[cfg(not(feature = "cuda"))]
    fn probe(&self) -> Result<CapabilitySet, PrecisionError> {
        Err(PrecisionError::Capability { reason: "cuda feature disabled".into() })
    }
}

#[cfg(test)]
#[path = "test/probe_cuda.rs"]
mod tests;
