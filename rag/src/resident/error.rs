//! Error taxonomy.
//!
//! One enum per layer; upper layers wrap lower ones via `#[from]`/`#[source]`. All variants
//! are `Send + Sync + 'static` and carry context as typed fields, never pre-formatted strings.
//! Public enums are `#[non_exhaustive]` so new variants don't break downstream matches.
use thiserror::Error;
pub use crate::resident::processor::OrchestrationError;
/// Classifies an error as retryable (transient) vs. fatal, for backoff decisions.
pub trait IsRetryable {
    fn is_retryable(&self) -> bool;
}
/// Index/scope layer errors.
#[derive(Error, Debug)]
#[non_exhaustive]
pub enum IndexError {
    #[error("access method '{method}' is not supported for this index")]
    UnsupportedScope { method: String },
    #[error("index for scope '{scope}' is not available")]
    IndexUnavailable { scope: String },
    #[error("arena exhausted: requested {requested} bytes, available {available} bytes")]
    ArenaExhausted { requested: usize, available: usize },
    #[error("transfer failed: {reason}")]
    TransferFailed { reason: String },
    #[error("kernel failed: {reason}")]
    KernelFailed { reason: String },
    #[error("unknown kernel batch {batch}")]
    UnknownBatch { batch: u64 },
}
impl IsRetryable for IndexError {
    fn is_retryable(&self) -> bool {
        matches!(
            self, IndexError::IndexUnavailable { .. } | IndexError::ArenaExhausted { .. }
        )
    }
}
/// Internal index-manager errors; mapped to a public variant or retried at the API boundary,
/// never exposed directly.
#[derive(Error, Debug)]
#[non_exhaustive]
pub(crate) enum ManagerError {
    #[error("stale region in arena {arena} (gen {gen})")]
    StaleRegion { arena: u32, gen: u32 },
}
impl IsRetryable for ManagerError {
    fn is_retryable(&self) -> bool {
        matches!(self, ManagerError::StaleRegion { .. })
    }
}
/// Precision/calibration layer errors.
#[derive(Error, Debug)]
#[non_exhaustive]
pub enum PrecisionError {
    #[error("incompatible precision/layout: {reason}")]
    Incompatible { reason: String },
    #[error("calibration unstable: {reason}")]
    CalibrationUnstable { reason: String },
    #[error("no active calibration profile")]
    NoActiveProfile,
    #[error("invalid precision requested: {requested}")]
    InvalidPrecision { requested: f32 },
}
impl IsRetryable for PrecisionError {
    fn is_retryable(&self) -> bool {
        matches!(self, PrecisionError::CalibrationUnstable { .. })
    }
}
/// Planner/task-expansion layer errors.
#[derive(Error, Debug)]
#[non_exhaustive]
pub enum PlannerError {
    #[error("invalid planner config: {reason}")]
    InvalidConfig { reason: String },
    #[error("missing capability: {capability}")]
    MissingCapability { capability: String },
    #[error("unknown model: {model}")]
    UnknownModel { model: String },
}
impl IsRetryable for PlannerError {
    fn is_retryable(&self) -> bool {
        false
    }
}
/// Capability/profile-matching layer errors.
#[derive(Error, Debug)]
#[non_exhaustive]
pub enum CapabilityError {
    #[error("missing capability: {capability}")]
    Missing { capability: String },
    #[error("unknown capability: {capability}")]
    Unknown { capability: String },
}
/// Facade/configuration errors raised by the builder and `init()`.
#[derive(Error, Debug)]
#[non_exhaustive]
pub enum ConfigError {
    #[error("missing required field: {field}")]
    MissingField { field: String },
    #[error("unknown model: {model}")]
    UnknownModel { model: String },
    #[error("invalid value for '{field}': {reason}")]
    InvalidValue { field: String, reason: String },
}
impl IsRetryable for CapabilityError {
    fn is_retryable(&self) -> bool {
        false
    }
}
impl IsRetryable for ConfigError {
    fn is_retryable(&self) -> bool {
        false
    }
}
/// Top-level resident engine error, wrapping every layer error via `#[from]`.
#[derive(Error, Debug)]
#[non_exhaustive]
pub enum ResidentError {
    #[error(transparent)]
    Index(#[from] IndexError),
    #[error(transparent)]
    Precision(#[from] PrecisionError),
    #[error(transparent)]
    Planner(#[from] PlannerError),
    #[error(transparent)]
    Config(#[from] ConfigError),
    #[error(transparent)]
    Capability(#[from] CapabilityError),
    #[error(transparent)]
    Orchestration(#[from] OrchestrationError),
}
impl IsRetryable for ResidentError {
    fn is_retryable(&self) -> bool {
        match self {
            ResidentError::Index(e) => e.is_retryable(),
            ResidentError::Precision(e) => e.is_retryable(),
            ResidentError::Planner(e) => e.is_retryable(),
            ResidentError::Config(e) => e.is_retryable(),
            ResidentError::Capability(e) => e.is_retryable(),
            ResidentError::Orchestration(e) => e.is_retryable(),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn assert_send_sync_static<T: Send + Sync + 'static>() {}
    #[test]
    fn all_errors_are_send_sync_static() {
        assert_send_sync_static::<ResidentError>();
        assert_send_sync_static::<IndexError>();
        assert_send_sync_static::<ManagerError>();
        assert_send_sync_static::<PrecisionError>();
        assert_send_sync_static::<PlannerError>();
        assert_send_sync_static::<CapabilityError>();
        assert_send_sync_static::<ConfigError>();
    }
    #[test]
    fn layer_errors_convert_into_resident_error() {
        let e: ResidentError = IndexError::UnknownBatch {
            batch: 3,
        }
            .into();
        assert!(
            matches!(e, ResidentError::Index(IndexError::UnknownBatch { batch : 3 }))
        );
        let e: ResidentError = ConfigError::MissingField {
            field: "model".into(),
        }
            .into();
        assert!(matches!(e, ResidentError::Config(_)));
        let e: ResidentError = PrecisionError::NoActiveProfile.into();
        assert!(matches!(e, ResidentError::Precision(_)));
        let e: ResidentError = PlannerError::UnknownModel {
            model: "m".into(),
        }
            .into();
        assert!(matches!(e, ResidentError::Planner(_)));
        let e: ResidentError = CapabilityError::Missing {
            capability: "c".into(),
        }
            .into();
        assert!(matches!(e, ResidentError::Capability(_)));
    }
    #[test]
    fn transparent_wrapper_keeps_message() {
        let inner = IndexError::ArenaExhausted {
            requested: 8,
            available: 2,
        };
        let msg = inner.to_string();
        assert_eq!(ResidentError::from(inner).to_string(), msg);
        assert!(msg.contains('8') && msg.contains('2'));
    }
    #[test]
    fn retryability_classification() {
        assert!(
            IndexError::ArenaExhausted { requested : 1, available : 0 } .is_retryable()
        );
        assert!(IndexError::IndexUnavailable { scope : "s".into() } .is_retryable());
        assert!(! IndexError::KernelFailed { reason : "r".into() } .is_retryable());
        assert!(ManagerError::StaleRegion { arena : 1, gen : 2 } .is_retryable());
        assert!(
            PrecisionError::CalibrationUnstable { reason : "r".into() } .is_retryable()
        );
        assert!(! PrecisionError::NoActiveProfile.is_retryable());
        assert!(
            ! ConfigError::InvalidValue { field : "f".into(), reason : "r".into() }
            .is_retryable()
        );
        assert!(! CapabilityError::Unknown { capability : "c".into() } .is_retryable());
        let wrapped: ResidentError = IndexError::ArenaExhausted {
            requested: 1,
            available: 0,
        }
            .into();
        assert!(wrapped.is_retryable());
        let fatal: ResidentError = PlannerError::InvalidConfig {
            reason: "r".into(),
        }
            .into();
        assert!(! fatal.is_retryable());
    }
}
