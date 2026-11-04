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
    #[error("task {task} timed out")]
    Timeout { task: u64 },
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
#[path = "test/error.rs"]
mod tests;
