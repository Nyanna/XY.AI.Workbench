//! Memory-backend contract shared by the CPU and GPU backends.

use std::fmt;

use super::super::_4::a::allocator::AllocatorError;
use super::super::_4::a::library_adapter::AdapterError;
use super::super::_4::_3::a::region_handle::RegionHandle;
use super::super::_4::_3::_2::a::arenas::{ArenaKind, Budget, BudgetError};
use super::super::_4::_3::_2::a::memory_access::AccessError;
use super::super::_4::_3::_2::_1::device_lane_event::Event;

/// Every allocation length is rounded up to a multiple of this value, so every region offset is one too.
pub const ALLOC_GRANULE: u64 = 256;
/// Largest supported alignment (power of two).
pub const MAX_ALIGN2: u64 = 256;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TransferDirection {
    D2D,
    H2D,
    D2H,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BackendError2 {
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

impl fmt::Display for BackendError2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BackendError2::OutOfMemory {
                arena,
                requested,
                free,
            } => write!(
                f,
                "out of memory in arena {arena:?}: requested {requested}, free {free}"
            ),
            BackendError2::AllocationFailed => f.write_str("Allokation fehlgeschlagen"),
            BackendError2::StaleRegion => f.write_str("stale Region"),
            BackendError2::DeviceMemoryUnavailable(reason) => {
                write!(f, "Device-Speicher nicht verfügbar: {reason}")
            }
            BackendError2::Transfer { direction, reason } => {
                write!(f, "transfer {direction:?} failed: {reason}")
            }
            BackendError2::ArenaMismatch { expected, found } => {
                write!(f, "arena mismatch: expected {expected:?}, found {found:?}")
            }
            BackendError2::Budget(e) => write!(f, "budget: {e}"),
            BackendError2::Access(e) => write!(f, "access: {e}"),
            BackendError2::Adapter(e) => write!(f, "adapter: {e}"),
        }
    }
}

impl std::error::Error for BackendError2 {}

impl From<AllocatorError> for BackendError2 {
    fn from(e: AllocatorError) -> Self {
        match e {
            AllocatorError::OutOfMemory {
                arena,
                requested,
                free,
            } => BackendError2::OutOfMemory {
                arena,
                requested,
                free,
            },
            AllocatorError::AllocationFailed => BackendError2::AllocationFailed,
            AllocatorError::StaleRegion => BackendError2::StaleRegion,
        }
    }
}

impl From<BudgetError> for BackendError2 {
    fn from(e: BudgetError) -> Self {
        BackendError2::Budget(e)
    }
}

impl From<AccessError> for BackendError2 {
    fn from(e: AccessError) -> Self {
        BackendError2::Access(e)
    }
}

impl From<AdapterError> for BackendError2 {
    fn from(e: AdapterError) -> Self {
        BackendError2::Adapter(e)
    }
}

/// Validates an allocation request and returns the length rounded up to `ALLOC_GRANULE`.
///
/// `size` must be non-zero, `align` a power of two in `1..=MAX_ALIGN`, and the rounding must not overflow.
pub fn rounded_len(size: u64, align: u64) -> Result<u64, BackendError2> {
    if size == 0 || !align.is_power_of_two() || align > MAX_ALIGN2 {
        return Err(BackendError2::AllocationFailed);
    }
    let padded = size
        .checked_add(ALLOC_GRANULE - 1)
        .ok_or(BackendError2::AllocationFailed)?;
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
    ) -> Result<RegionHandle, BackendError2>;

    /// `StaleRegion` for foreign, not live, already pending or already freed regions.
    fn free(&self, region: &RegionHandle) -> Result<(), BackendError2>;

    /// The region stays live and counts as used until `event` completes.
    fn free_deferred(&self, region: &RegionHandle, event: Event) -> Result<(), BackendError2>;

    /// Memory budget as the backend measures it: arena sums on the CPU, the driver's view on the GPU.
    /// Only compare values taken from the same backend.
    fn budget(&self) -> Result<Budget, BackendError2>;

    /// Free bytes of one arena; 0 for an arena without capacity.
    fn available(&self, arena: ArenaKind) -> Result<u64, BackendError2>;
}

static_assertions::assert_impl_all!(BackendError2: Send, Sync);
static_assertions::assert_obj_safe!(MemoryBackend);

#[cfg(test)]
#[path = "test/memory_backend.rs"]
mod tests;
