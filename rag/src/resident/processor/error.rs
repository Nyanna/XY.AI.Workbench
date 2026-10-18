use std::sync::Arc;

use thiserror::Error;

use crate::resident::error::{IsRetryable, ResidentError};
use crate::resident::op::{EngineSelector, ResourceKind};

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum OrchestrationError {
    #[error("task {task} cancelled")]
    Cancelled { task: u64 },
    #[error("resource {resource:?} is backing off")]
    Backoff { resource: ResourceKind },
    #[error("no workers configured for {engine:?} operations")]
    EngineUnavailable { engine: EngineSelector },
    #[error("lane {lane} lost its completion signal")]
    CompletionLost { lane: u32 },
    #[error("operation of task {task} failed")]
    OpFailed {
        task: u64,
        #[source]
        source: Arc<ResidentError>,
    },
    #[error("processor is shutting down")]
    Shutdown,
}

impl IsRetryable for OrchestrationError {
    fn is_retryable(&self) -> bool {
        match self {
            Self::Backoff { .. } => true,
            Self::OpFailed { source, .. } => source.is_retryable(),
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resident::error::IndexError;

    #[test]
    fn retryability_follows_cause() {
        assert!(OrchestrationError::Backoff { resource: ResourceKind::Encoder }.is_retryable());
        assert!(!OrchestrationError::Cancelled { task: 1 }.is_retryable());
        let transient: ResidentError = IndexError::ArenaExhausted { requested: 1, available: 0 }.into();
        let e = OrchestrationError::OpFailed { task: 1, source: Arc::new(transient) };
        assert!(e.is_retryable());
    }

    #[test]
    fn converts_into_resident_error() {
        let e: ResidentError = OrchestrationError::Shutdown.into();
        assert!(matches!(e, ResidentError::Orchestration(OrchestrationError::Shutdown)));
    }
}
