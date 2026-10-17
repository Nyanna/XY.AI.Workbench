//! Compute backend abstraction and the single `#[cfg(feature = "cuda")]` decision point.
//!
//! Everything outside `create_backend` runs against the `ComputeBackend` trait and stays
//! feature-independent; only this factory and the CUDA adapter modules (`kernel::cuda`,
//! `subengine::device`) are gated.

use crate::resident::subengine::CpuBackend;
#[cfg(feature = "cuda")]
use crate::resident::subengine::CudaBackend;

/// Backend abstraction the orchestration layer runs against.
pub trait ComputeBackend: Send + Sync {
    fn run_batch(&self, input: &[f32]) -> Vec<f32>;
}

/// Constructs the active backend: CPU without `cuda`; with `cuda`, attempts the CUDA backend
/// and falls back to CPU if GPU initialization fails (per `resident_subengine.md` §6).
pub fn create_backend() -> Box<dyn ComputeBackend> {
    #[cfg(feature = "cuda")]
    {
        if let Ok(backend) = CudaBackend::try_new() {
            return Box::new(backend);
        }
    }
    Box::new(CpuBackend)
}
