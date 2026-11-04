use thiserror::Error;

use crate::resident::error::IsRetryable;
use crate::resident::memory::MemoryError;
use crate::resident::subengine::DeviceError;

/// Model loading and encoding errors.
#[derive(Error, Debug)]
#[non_exhaustive]
pub enum ModelError {
    #[error("model load failed: {reason}")]
    Load { reason: String },
    #[error("tensor `{expected}` not found in checkpoint")]
    WeightNameMismatch { expected: String },
    #[error("unsupported architecture `{arch}`")]
    UnsupportedArch { arch: String },
    #[error("incompatible model configuration: {reason}")]
    Incompatible { reason: String },
    #[error("encoding failed: {reason}")]
    EncodeFailed { reason: String },
    #[error(transparent)]
    Memory(#[from] MemoryError),
    #[error(transparent)]
    Device(#[from] DeviceError),
}

impl IsRetryable for ModelError {
    fn is_retryable(&self) -> bool {
        match self {
            Self::Memory(e) => e.is_retryable(),
            Self::Device(e) => e.is_retryable(),
            _ => false,
        }
    }
}
