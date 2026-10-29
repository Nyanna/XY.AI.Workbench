//! Segment-layer errors.

use thiserror::Error;

use crate::resident::error::IsRetryable;
use crate::resident::memory::dtype::DType;
use crate::resident::subengine::DeviceError;

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum SegmentError {
    #[error("segment generation {gen} is not part of the current manifest")]
    StaleGeneration { gen: u64 },
    #[error("slot {slot} out of range ({slots} slots)")]
    SlotOutOfRange { slot: u32, slots: u32 },
    #[error("no entry mapped for slot {slot} of segment generation {gen}")]
    MappingMissing { gen: u64, slot: u32 },
    #[error("segment has no free slot ({slots} slots)")]
    SegmentFull { slots: u32 },
    #[error("{tokens} tokens exceed slot capacity {max}")]
    TokenOverflow { tokens: u32, max: u32 },
    #[error("expected {expected} values, got {got}")]
    ShapeMismatch { expected: usize, got: usize },
    #[error("region of {have} bytes cannot hold {needed} bytes")]
    RegionTooSmall { needed: u64, have: u64 },
    #[error("slot geometry overflows")]
    GeometryOverflow,
    #[error("dtype {dtype:?} cannot hold slot vectors")]
    UnsupportedDType { dtype: DType },
    #[error("slot write has not completed")]
    NotReady,
    #[error("segment sync failed")]
    SyncFailed {
        #[source]
        source: DeviceError,
    },
}

impl IsRetryable for SegmentError {
    fn is_retryable(&self) -> bool {
        match self {
            Self::StaleGeneration { .. } | Self::SegmentFull { .. } | Self::NotReady => true,
            Self::SyncFailed { source } => source.is_retryable(),
            _ => false,
        }
    }
}

/// Internal lookup failure; surfaced as `SegmentError` at the API boundary.
#[derive(Error, Debug)]
pub(crate) enum MappingError {
    #[error("stale mapping for segment generation {gen}")]
    StaleMapping { gen: u64 },
}

impl From<MappingError> for SegmentError {
    fn from(e: MappingError) -> Self {
        match e {
            MappingError::StaleMapping { gen } => SegmentError::StaleGeneration { gen },
        }
    }
}
