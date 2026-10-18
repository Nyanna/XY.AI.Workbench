//! Shared subengine lifecycle & pull model, tokenizer/normalizer (CPU).
use crate::resident::backend::ComputeBackend;
/// CPU backend; default without `cuda`, and the fallback target if CUDA init fails.
/// Placeholder until the real embedding path is wired in.
pub struct CpuBackend;
impl ComputeBackend for CpuBackend {
    fn run_batch(&self, input: &[f32]) -> Vec<f32> {
        input.to_vec()
    }
}
mod device;
mod exec;
mod error;
mod lifecycle;
mod resource;
pub use device::*;
pub use exec::{
    CpuLane, EventHandle, ExecContext, LaneClass, LaneConfig, LaneJob, ParamBlock,
    ParamValue, ScratchArena,
};
#[cfg(any(test, feature = "mock"))]
pub use exec::RecordingLane;
pub use error::{DeviceError, SubengineError};
pub use lifecycle::{
    CellConfig, InitLimiter, OpGuard, ResourceCell, ResourceLease, ResourceState,
};
pub use resource::{
    coalesce, Capacity, Claim, ClaimFilter, Ewma, PullConfig, PullController, Resource,
    ResourceKey,
};
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SubengineKind {
    Cpu,
    Cuda,
}
/// Resources and execution capacity of one compute substrate. Work arrives through the
/// processor's op queues; the subengine only reports how much it can take.
pub trait Subengine: Send + Sync {
    fn kind(&self) -> SubengineKind;
    fn capacity(&self) -> Capacity;
    /// Smoothed items per second.
    fn throughput(&self) -> f32;
    /// Throughput-derived batch size, independent of current load.
    fn batch_size(&self) -> usize;
    /// Upper bound on the jobs to hand this subengine in one `Claim::claim` call.
    fn pull_budget(&self) -> usize {
        self.capacity().free().min(self.batch_size())
    }
}
