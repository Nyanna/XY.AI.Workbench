//! Error taxonomy.
//!
//! One enum per layer; upper layers wrap lower ones via `#[from]`/`#[source]`. All variants
//! are `Send + Sync + 'static` and carry context as typed fields, never pre-formatted strings.
//! Public enums are `#[non_exhaustive]` so new variants don't break downstream matches.
use thiserror::Error;
pub use crate::resident::processor::OrchestrationError;
pub use crate::resident::subengine::SubengineError;
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
    #[error(
        "index schema does not match the active model configuration (expected {expected}; found {found})"
    )]
    IncompatibleSchema { expected: String, found: String },
    #[error("checksum mismatch for entry {entry}")]
    ChecksumMismatch { entry: u64 },
}
impl IsRetryable for IndexError {
    fn is_retryable(&self) -> bool {
        matches!(
            self, IndexError::IndexUnavailable { .. } | IndexError::ArenaExhausted { .. }
        )
    }
}
/// Index-manager layer errors. The facade maps them to public variants or retries; code that
/// builds on the manager layer directly (engines) sees them as is.
#[derive(Error, Debug)]
#[non_exhaustive]
pub enum ManagerError {
    #[error("stale region in arena {arena} (gen {gen})")]
    StaleRegion { arena: u32, gen: u32 },
    #[error(
        "index arena exhausted: requested {requested} bytes, {available} bytes available after copy-on-write headroom"
    )]
    ArenaExhausted { requested: u64, available: u64 },
    #[error("stale table build: based on epoch {base}, published epoch is {published}")]
    StaleMapping { base: u64, published: u64 },
    #[error(transparent)]
    Memory(#[from] crate::resident::memory::MemoryError),
    #[error(transparent)]
    Kernel(#[from] IndexError),
}
impl IsRetryable for ManagerError {
    fn is_retryable(&self) -> bool {
        match self {
            ManagerError::StaleRegion { .. } | ManagerError::ArenaExhausted { .. } => {
                true
            }
            ManagerError::StaleMapping { .. } => true,
            ManagerError::Memory(e) => e.is_retryable(),
            ManagerError::Kernel(e) => e.is_retryable(),
        }
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
    #[error("capability probe failed: {reason}")]
    Capability { reason: String },
    #[error("calibration store failed: {reason}")]
    Store { reason: String },
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
pub use crate::resident::subengine::DeviceError;
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
    #[error(transparent)]
    Subengine(#[from] SubengineError),
    #[error(transparent)]
    Device(#[from] DeviceError),
    #[error(transparent)]
    Memory(#[from] crate::resident::memory::MemoryError),
    #[error(transparent)]
    Model(#[from] crate::resident::model::ModelError),
    #[error(transparent)]
    Tokenize(#[from] crate::resident::tokenize::TokenizeError),
    #[error(transparent)]
    Kernel(#[from] crate::resident::kernel::KernelError),
    #[error(transparent)]
    Segment(#[from] crate::resident::segment::SegmentError),
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
            ResidentError::Subengine(e) => e.is_retryable(),
            ResidentError::Device(e) => e.is_retryable(),
            ResidentError::Memory(e) => e.is_retryable(),
            ResidentError::Model(e) => e.is_retryable(),
            ResidentError::Tokenize(e) => e.is_retryable(),
            ResidentError::Kernel(e) => e.is_retryable(),
            ResidentError::Segment(e) => e.is_retryable(),
        }
    }
}
#[cfg(test)]
#[path = "test/error.rs"]
mod tests;
