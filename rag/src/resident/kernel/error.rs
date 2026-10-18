use thiserror::Error;

use crate::resident::error::{IndexError, IsRetryable};

/// Kernel registry, scoring and launch errors.
#[derive(Error, Debug)]
#[non_exhaustive]
pub enum KernelError {
    #[error("kernel compilation failed: {log}")]
    Compile { log: String },
    #[error("kernel module load failed: {reason}")]
    ModuleLoad { reason: String },
    #[error("kernel function '{name}' not found")]
    FuncNotFound { name: String },
    #[error("kernel launch failed")]
    LaunchFailed {
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },
    #[error("parameter block overflow: {requested} bytes requested, capacity {capacity}")]
    ParamBlockOverflow { requested: usize, capacity: usize },
    #[error("shape mismatch: expected {expected} elements, got {actual}")]
    ShapeMismatch { expected: usize, actual: usize },
    #[error("slice {segment}:{slot_from}..{slot_to} exceeds the segment's {count} slots")]
    SliceOutOfRange { segment: u32, slot_from: u32, slot_to: u32, count: u32 },
    #[error("kernel backend unavailable: {reason}")]
    Unavailable { reason: String },
}

impl IsRetryable for KernelError {
    fn is_retryable(&self) -> bool {
        false
    }
}

impl From<KernelError> for IndexError {
    fn from(e: KernelError) -> Self {
        IndexError::KernelFailed { reason: e.to_string() }
    }
}

impl From<crate::resident::subengine::DeviceError> for KernelError {
    fn from(e: crate::resident::subengine::DeviceError) -> Self {
        use crate::resident::subengine::DeviceError;
        match e {
            DeviceError::ParamOverflow { capacity, requested } => {
                KernelError::ParamBlockOverflow { requested, capacity }
            }
            other => KernelError::LaunchFailed { source: Box::new(other) },
        }
    }
}
