//! SegmentDesc/Layout/KernelHit, Kernel trait.

#[cfg(feature = "cuda")]
mod cuda;
#[cfg(feature = "cuda")]
pub use cuda::*;
