//! Region/Layout/DType, the `MemoryManager` contract, and per-class device arenas.

pub mod arena;
pub mod budget;
pub mod cpu;
#[cfg(feature = "cuda")]
pub mod cuda;
pub mod dtype;
pub mod error;
pub mod freelist;
pub mod layout;
pub mod manager;
pub mod region;
pub mod staging;
#[cfg(all(test, feature = "cuda"))]
mod wrap_gate;

pub use arena::{ArenaAllocator, ArenaSet, DeviceMemory};
pub use budget::{ArenaKind, ArenaSizes, Budget, BudgetProbe, DEFAULT_HEADROOM};
pub use cpu::CpuMemoryManager;
pub use dtype::{DType, UnsupportedDType, F16_MASK_VALUE};
pub use error::{MemoryError, TransferDir};
pub use manager::{compute_dtype, BookkeepingArena, DeviceKind, MemoryManager};
pub use region::{ArenaId, Region};
pub use staging::StagingRing;
