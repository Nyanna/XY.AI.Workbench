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
            self,
            Self::ResourceEvicting { .. } | Self::Backoff { .. } | Self::CapacityExhausted
        )
    }
}
