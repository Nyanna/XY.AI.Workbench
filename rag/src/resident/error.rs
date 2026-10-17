//! Error taxonomy.
//!
//! One enum per layer; upper layers wrap lower ones via `#[from]`/`#[source]`. All variants
//! are `Send + Sync + 'static` and carry context as typed fields, never pre-formatted strings.
//! Public enums are `#[non_exhaustive]` so new variants don't break downstream matches.
use thiserror::Error;
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
    #[error("stale mapping for scope '{scope}'")]
    StaleMapping { scope: String },
}
impl IsRetryable for ManagerError {
    fn is_retryable(&self) -> bool {
        matches!(self, ManagerError::StaleMapping { .. })
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
}
impl IsRetryable for ResidentError {
    fn is_retryable(&self) -> bool {
        match self {
            ResidentError::Index(e) => e.is_retryable(),
            ResidentError::Precision(e) => e.is_retryable(),
            ResidentError::Planner(e) => e.is_retryable(),
            ResidentError::Config(e) => e.is_retryable(),
            ResidentError::Capability(e) => e.is_retryable(),
        }
    }
}
