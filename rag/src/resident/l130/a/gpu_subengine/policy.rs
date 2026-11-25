use std::collections::HashMap;
use std::sync::Arc;

use super::backend::GpuBackend;
use super::deps::*;

/// FP16-first path policy of the GPU. BF16 is never chosen.
pub struct Fp16FirstPolicy {
    default_dtype: DType,
    kernel_names: HashMap<ResourceKind, String>,
}

impl Fp16FirstPolicy {
    pub fn new(default_dtype: DType, kernel_names: HashMap<ResourceKind, String>) -> Self {
        Fp16FirstPolicy {
            default_dtype,
            kernel_names,
        }
    }
}

/// The requirements accept fp32 operands only.
fn demands_fp32_only(req: &ResourceRequirements) -> bool {
    let caps = req.capabilities();
    caps.contains(&axes::operands(), &values::fp32())
        && !caps.contains(&axes::operands(), &values::fp16())
}

impl PathPolicy for Fp16FirstPolicy {
    fn select(
        &self,
        req: &ResourceRequirements,
        caps: &CapabilitySet,
        device: DeviceHandle,
    ) -> Option<ComputePath> {
        if device.kind() != DeviceKind::Gpu {
            return None;
        }
        let kernel = self.kernel_names.get(&req.kind())?.clone();
        let half = self.default_dtype == DType::F16
            && caps.contains(&axes::operands(), &values::fp16())
            && !demands_fp32_only(req);
        let dtype = if half {
            DType::F16
        } else if caps.contains(&axes::operands(), &values::fp32()) {
            DType::F32
        } else {
            return None;
        };
        Some(ComputePath {
            kernel,
            dtype,
            variant: Variant::default(),
            lane: LaneKind::Query,
        })
    }
}

/// Creates lanes that always own a stream of the device.
pub struct GpuLaneFactory {
    backend: Arc<dyn GpuBackend>,
    ordinal: u32,
}

impl GpuLaneFactory {
    pub fn new(backend: Arc<dyn GpuBackend>, ordinal: u32) -> Self {
        GpuLaneFactory { backend, ordinal }
    }
}

impl LaneFactory for GpuLaneFactory {
    fn create(&self, cfg: &LaneConfig) -> Result<Lane, String> {
        let stream = self.backend.create_stream(self.ordinal, cfg.priority)?;
        Ok(Lane::new(cfg.clone(), Some(stream)))
    }
}

#[cfg(test)]
#[path = "test/policy.rs"]
mod tests;
