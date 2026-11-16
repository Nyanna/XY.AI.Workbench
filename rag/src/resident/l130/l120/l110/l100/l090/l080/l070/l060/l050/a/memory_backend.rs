//! Memory-backend contract shared by the CPU and GPU backends.

use std::fmt;

use super::super::l040::a::allocator::AllocatorError;
use super::super::l040::a::library_adapter::AdapterError;
use super::super::l040::l030::a::region_handle::RegionHandle;
use super::super::l040::l030::l020::a::arenas::{ArenaKind, Budget, BudgetError};
use super::super::l040::l030::l020::a::memory_access::AccessError;
use super::super::l040::l030::l020::l010::device_lane_event::Event;

/// Every allocation length is rounded up to a multiple of this value, so every region offset is one too.
pub const ALLOC_GRANULE: u64 = 256;
/// Largest supported alignment (power of two).
pub const MAX_ALIGN: u64 = 256;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TransferDirection {
    D2D,
    H2D,
    D2H,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BackendError {
    /// Exhaustion. `requested` is the rounded length, `free` is `Allocator::available`.
    OutOfMemory {
        arena: ArenaKind,
        requested: u64,
        free: u64,
    },
    /// Invalid request or a failure that is not exhaustion.
    AllocationFailed,
    StaleRegion,
    DeviceMemoryUnavailable(String),
    Transfer {
        direction: TransferDirection,
        reason: String,
    },
    ArenaMismatch {
        expected: ArenaKind,
        found: ArenaKind,
    },
    Budget(BudgetError),
    Access(AccessError),
    Adapter(AdapterError),
}

impl fmt::Display for BackendError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BackendError::OutOfMemory {
                arena,
                requested,
                free,
            } => write!(
                f,
                "out of memory in arena {arena:?}: requested {requested}, free {free}"
            ),
            BackendError::AllocationFailed => f.write_str("Allokation fehlgeschlagen"),
            BackendError::StaleRegion => f.write_str("stale Region"),
            BackendError::DeviceMemoryUnavailable(reason) => {
                write!(f, "Device-Speicher nicht verfügbar: {reason}")
            }
            BackendError::Transfer { direction, reason } => {
                write!(f, "transfer {direction:?} failed: {reason}")
            }
            BackendError::ArenaMismatch { expected, found } => {
                write!(f, "arena mismatch: expected {expected:?}, found {found:?}")
            }
            BackendError::Budget(e) => write!(f, "budget: {e}"),
            BackendError::Access(e) => write!(f, "access: {e}"),
            BackendError::Adapter(e) => write!(f, "adapter: {e}"),
        }
    }
}

impl std::error::Error for BackendError {}

impl From<AllocatorError> for BackendError {
    fn from(e: AllocatorError) -> Self {
        match e {
            AllocatorError::OutOfMemory {
                arena,
                requested,
                free,
            } => BackendError::OutOfMemory {
                arena,
                requested,
                free,
            },
            AllocatorError::AllocationFailed => BackendError::AllocationFailed,
            AllocatorError::StaleRegion => BackendError::StaleRegion,
        }
    }
}

impl From<BudgetError> for BackendError {
    fn from(e: BudgetError) -> Self {
        BackendError::Budget(e)
    }
}

impl From<AccessError> for BackendError {
    fn from(e: AccessError) -> Self {
        BackendError::Access(e)
    }
}

impl From<AdapterError> for BackendError {
    fn from(e: AdapterError) -> Self {
        BackendError::Adapter(e)
    }
}

/// Validates an allocation request and returns the length rounded up to `ALLOC_GRANULE`.
///
/// `size` must be non-zero, `align` a power of two in `1..=MAX_ALIGN`, and the rounding must not overflow.
pub fn rounded_len(size: u64, align: u64) -> Result<u64, BackendError> {
    if size == 0 || !align.is_power_of_two() || align > MAX_ALIGN {
        return Err(BackendError::AllocationFailed);
    }
    let padded = size
        .checked_add(ALLOC_GRANULE - 1)
        .ok_or(BackendError::AllocationFailed)?;
    Ok(padded / ALLOC_GRANULE * ALLOC_GRANULE)
}

/// Allocation interface common to all backends.
///
/// Regions are addressed by `RegionHandle`; no address of backing memory crosses this interface.
/// Data movement is backend specific (synchronous byte access on the CPU, asynchronous transfers
/// with events on the GPU) and therefore not part of this trait.
pub trait MemoryBackend: Send + Sync {
    /// Allocates `rounded_len(size, align)` bytes in `arena`; the offset is a multiple of `ALLOC_GRANULE`.
    fn allocate(
        &self,
        arena: ArenaKind,
        size: u64,
        align: u64,
    ) -> Result<RegionHandle, BackendError>;

    /// `StaleRegion` for foreign, not live, already pending or already freed regions.
    fn free(&self, region: &RegionHandle) -> Result<(), BackendError>;

    /// The region stays live and counts as used until `event` completes.
    fn free_deferred(&self, region: &RegionHandle, event: Event) -> Result<(), BackendError>;

    /// Memory budget as the backend measures it: arena sums on the CPU, the driver's view on the GPU.
    /// Only compare values taken from the same backend.
    fn budget(&self) -> Result<Budget, BackendError>;

    /// Free bytes of one arena; 0 for an arena without capacity.
    fn available(&self, arena: ArenaKind) -> Result<u64, BackendError>;
}

static_assertions::assert_impl_all!(BackendError: Send, Sync);
static_assertions::assert_obj_safe!(MemoryBackend);

#[cfg(test)]
#[path = "test/memory_backend.rs"]
mod tests;
