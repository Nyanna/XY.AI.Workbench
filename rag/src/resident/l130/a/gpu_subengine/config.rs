use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use super::deps::*;
use super::error::GpuError;

#[derive(Clone, Debug)]
pub struct GpuConfig {
    pub arenas: ArenaCapacities,
    pub headroom: u64,
    pub generations: u32,
    pub lifecycle: LifecycleConfig,
    pub ram_init_deadline: Duration,
    pub vram_init_deadline: Duration,
    pub default_dtype: DType,
    pub capability: CapabilityConfig,
    pub worker_lanes: usize,
    pub lane_workspace_bytes: usize,
    pub upload_batch: usize,
}

impl GpuConfig {
    /// Checks in order; the first violation is returned.
    pub fn validate(&self) -> Result<(), GpuError> {
        let invalid = |text: String| Err(GpuError::InvalidConfig(text));
        if !matches!(self.default_dtype, DType::F16 | DType::F32) {
            return invalid(
                format!("default dtype {:?} is neither F16 nor F32", self.default_dtype),
            );
        }
        if self.worker_lanes < 1 {
            return invalid("worker_lanes must be at least 1".to_string());
        }
        if self.upload_batch < 1 {
            return invalid("upload_batch must be at least 1".to_string());
        }
        let ram = self.lifecycle.ram_ttl();
        let vram = self.lifecycle.vram_ttl();
        if vram < ram.saturating_mul(2) {
            return invalid(
                format!("vram_ttl {vram:?} must be at least twice ram_ttl {ram:?}"),
            );
        }
        if self.vram_init_deadline < self.ram_init_deadline.saturating_mul(2) {
            return invalid(
                format!(
                    "vram_init_deadline {:?} must be at least twice ram_init_deadline {:?}",
                    self.vram_init_deadline, self.ram_init_deadline
                ),
            );
        }
        Ok(())
    }
}

impl Default for GpuConfig {
    fn default() -> Self {
        let lifecycle = LifecycleConfig::new(
                Duration::from_secs(60),
                Duration::from_secs(600),
                Duration::from_secs(5),
            )
            .expect("default lifecycle durations are valid");
        GpuConfig {
            arenas: ArenaCapacities::default(),
            headroom: DEFAULT_HEADROOM,
            generations: 1,
            lifecycle,
            ram_init_deadline: Duration::from_secs(30),
            vram_init_deadline: Duration::from_secs(300),
            default_dtype: DType::F16,
            capability: CapabilityConfig::default(),
            worker_lanes: 2,
            lane_workspace_bytes: 0,
            upload_batch: 256,
        }
    }
}

#[derive(Clone, Debug)]
pub struct GpuDeviceManagerConfig {
    /// `None`: all devices of the backend.
    pub devices: Option<Vec<u32>>,
    pub gpu: GpuConfig,
}

impl Default for GpuDeviceManagerConfig {
    fn default() -> Self {
        GpuDeviceManagerConfig {
            devices: None,
            gpu: GpuConfig::default(),
        }
    }
}

/// Shared by all sub-engines of the process.
#[derive(Clone)]
pub struct GpuShared {
    pub kernels: Arc<KernelRegister>,
    pub registry: Arc<ModelRegistry>,
    pub registrations: Arc<QueueRegistrations>,
    pub resolver: Arc<ScopeResolverRegistry>,
    pub cell_selector: Option<Arc<dyn CellSelector>>,
}

pub(super) fn default_kernel_names() -> HashMap<ResourceKind, String> {
    HashMap::from([(ResourceKind::Scorer, KERNEL_SCORE_DOT.to_string())])
}

#[cfg(test)]
#[path = "test/config.rs"]
mod tests;
