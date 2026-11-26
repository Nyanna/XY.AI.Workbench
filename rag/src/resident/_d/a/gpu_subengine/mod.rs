//! CUDA realisation of a sub-engine: device manager, VRAM manager and one sub-engine per GPU.

mod backend;
mod config;
mod deps;
mod device_manager;
mod error;
mod index_services;
mod policy;
mod subengine;
mod vram;

#[cfg(test)]
#[path = "test/fixture.rs"]
pub(crate) mod fixture;

pub use backend::{GpuBackend3, GpuDeviceInfo};
pub use config::{GpuConfig, GpuDeviceManagerConfig, GpuShared};
pub use device_manager::GpuDeviceManager;
pub use error::GpuError;
pub use policy::{Fp16FirstPolicy, GpuLaneFactory};
pub use subengine::{GpuSubengine, IndexMemory};
pub use vram::{VramManager, WeightSet};
