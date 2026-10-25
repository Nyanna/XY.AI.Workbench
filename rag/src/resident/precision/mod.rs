//! CapabilitySet/Profile.
mod axis;
mod capability;
pub mod known;
mod pareto;
mod probe;
mod probe_cpu;
mod probe_cuda;
mod profile;
pub use axis::{AxisId, ValueId};
pub use capability::CapabilitySet;
pub use pareto::{pareto, positions};
pub use probe::{
    intersect_with_kernels, make_probe, CapabilityProbe, KernelFilteredProbe, ProbeConfig,
};
#[cfg(any(test, feature = "mock"))]
pub use probe::FakeCapabilityProbe;
pub use probe_cpu::{CpuFeatureSource, CpuFeatures, CpuProbe, FixedCpu, HostCpu};
pub use probe_cuda::{cuda_capabilities, has_tensor_cores, CudaProbe};
pub use profile::{Profile, ProfileConfig};
