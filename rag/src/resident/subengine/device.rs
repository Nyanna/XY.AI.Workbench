//! Device/Lane, execution context, loaded encoder, GPU memory arenas.
use crate::resident::backend::ComputeBackend;
/// Real CUDA backend; the single Candle/CUDA adapter point for the subengine (per
/// `resident_subengine.md` §2). Construction fails if no compatible GPU is available,
/// which lets `create_backend` fall back to `CpuBackend`.
pub struct CudaBackend;
impl CudaBackend {
    pub fn try_new() -> Result<Self, crate::resident::error::IndexError> {
        Err(crate::resident::error::IndexError::TransferFailed {
            reason: "CUDA backend not yet implemented".to_string(),
        })
    }
}
impl ComputeBackend for CudaBackend {
    fn run_batch(&self, _input: &[f32]) -> Vec<f32> {
        unimplemented!("real CUDA kernel dispatch lands in D6/H1")
    }
}
