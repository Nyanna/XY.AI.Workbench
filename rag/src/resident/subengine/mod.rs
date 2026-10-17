//! Shared subengine lifecycle & pull model, tokenizer/normalizer (CPU).
use crate::resident::backend::ComputeBackend;
/// CPU backend; default without `cuda`, and the fallback target if CUDA init fails.
/// Placeholder until the real embedding path (D4/D5) is wired in.
pub struct CpuBackend;
impl ComputeBackend for CpuBackend {
    fn run_batch(&self, input: &[f32]) -> Vec<f32> {
        input.to_vec()
    }
}
#[cfg(feature = "cuda")]
mod device;
#[cfg(feature = "cuda")]
pub use device::*;
