//! Region/Layout/DType and MemoryManager trait.

pub mod dtype;
pub mod freelist;
pub mod layout;
pub mod manager;
pub mod region;

pub use dtype::{DType, UnsupportedDType};
pub use freelist::FreeList;
pub use layout::Layout;
pub use manager::{compute_dtype, BookkeepingArena, DeviceKind, MemoryManager};
pub use region::{ArenaId, Region};
