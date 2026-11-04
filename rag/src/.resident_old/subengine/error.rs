use thiserror::Error;
use crate::resident::error::IsRetryable;
/// Subengine resource lifecycle errors.
#[derive(Error, Debug)]
#[non_exhaustive]
pub enum SubengineError {
    #[error("resource {resource} failed to initialise: {reason}")]
    ResourceInit { resource: String, reason: String },
    #[error("resource {resource} is being evicted")]
    ResourceEvicting { resource: String },
    #[error("resource {resource} is backing off until {until_ms} ms")]
    Backoff { resource: String, until_ms: u64 },
    #[error("subengine capacity exhausted")]
    CapacityExhausted,
    #[error("incompatible resource: {reason}")]
    Incompatible { reason: String },
}
impl IsRetryable for SubengineError {
    fn is_retryable(&self) -> bool {
        matches!(
            self, Self::ResourceEvicting { .. } | Self::Backoff { .. } |
            Self::CapacityExhausted
        )
    }
}
/// Device and execution-lane errors.
#[derive(Error, Debug)]
#[non_exhaustive]
pub enum DeviceError {
    #[error("device initialisation failed: {reason}")]
    Init { reason: String },
    #[error("CUDA support is not compiled in")]
    CudaUnavailable,
    #[error("stream creation failed: {reason}")]
    StreamCreate { reason: String },
    #[error("event operation failed: {reason}")]
    EventRecord { reason: String },
    #[error("stream priorities are not supported on this device")]
    PriorityUnsupported,
    #[error("cudarc version mismatch: candle uses {candle}, project uses {project}")]
    VersionMismatch { candle: String, project: String },
    #[error(
        "parameter block overflow: {requested} bytes requested, capacity {capacity}"
    )]
    ParamOverflow { capacity: usize, requested: usize },
    #[error(
        "scratch arena exhausted: {requested} bytes requested, {available} available"
    )]
    ScratchExhausted { requested: u64, available: u64 },
    #[error("invalid alignment {align}")]
    InvalidAlign { align: u64 },
    #[error("lane worker is closed")]
    LaneClosed,
    #[error("lane job failed: {reason}")]
    JobFailed { reason: String },
}
impl IsRetryable for DeviceError {
    fn is_retryable(&self) -> bool {
        matches!(self, Self::ScratchExhausted { .. })
    }
}
