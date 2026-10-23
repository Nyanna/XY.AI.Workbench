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
