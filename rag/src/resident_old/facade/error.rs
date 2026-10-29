use std::sync::Arc;

use thiserror::Error;

use crate::resident::error::ResidentError;

/// Cloneable so one outcome can be delivered to several waiters.
#[derive(Error, Debug, Clone)]
#[non_exhaustive]
pub enum TaskError {
    #[error("task cancelled")]
    Cancelled,
    #[error("timed out waiting for task result")]
    Timeout,
    #[error("precision must not be NaN")]
    InvalidPrecision,
    #[error(transparent)]
    Engine(Arc<ResidentError>),
}

impl From<ResidentError> for TaskError {
    fn from(err: ResidentError) -> Self {
        Self::Engine(Arc::new(err))
    }
}
