//! Memory-layer errors.

use thiserror::Error;

use super::budget::ArenaKind;
use crate::resident::error::IsRetryable;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TransferDir {
    HostToDevice,
    DeviceToHost,
    DeviceToDevice,
}

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum MemoryError {
    #[error("arena {arena:?} exhausted: requested {requested} bytes, {free} free")]
    ArenaExhausted { arena: ArenaKind, requested: u64, free: u64 },
    #[error("allocation failed in arena {arena:?}: {reason}")]
    AllocFailed { arena: ArenaKind, reason: String },
    #[error("{dir:?} transfer failed: {reason}")]
    TransferFailed { dir: TransferDir, reason: String },
    #[error("stale region in arena {arena} (gen {gen})")]
    StaleRegion { arena: u32, gen: u32 },
    #[error("invalid staging slot {index}")]
    InvalidSlot { index: usize },
    #[error("device memory probe failed: {reason}")]
    BudgetProbe { reason: String },
    #[error("arena plan needs {requested} bytes but only {usable} are usable")]
    BudgetExceeded { requested: u64, usable: u64 },
    #[error("device memory unavailable: {reason}")]
    Unavailable { reason: String },
}

impl IsRetryable for MemoryError {
    fn is_retryable(&self) -> bool {
        matches!(self, Self::ArenaExhausted { .. } | Self::StaleRegion { .. })
    }
}
