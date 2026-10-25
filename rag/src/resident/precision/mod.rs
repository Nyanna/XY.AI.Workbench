//! CapabilitySet/Profile.
mod calibrate;
mod measure;
mod measure_cpu;
#[cfg(feature = "cuda")]
mod measure_cuda;
#[cfg(any(test, feature = "mock"))]
mod measure_mock;
mod metrics;
mod pool;
mod stats;
mod workload;
mod axis;
mod capability;
pub mod known;
mod pareto;
mod probe;
mod probe_cpu;
mod probe_cuda;
mod profile;
pub use axis::{AxisId, ValueId};
pub use calibrate::{
    CalibrationReport, CalibrationTarget, Calibrator, CalibratorConfig, Entry, HardCap,
    Outcome, ParetoPoint, Stage,
};
pub use measure::{
    make_measure, Cfg, Clock, DeviationReport, Measure, MeasureResult, NeverYield,
    Sample, SystemClock, Validity, WindowTuning, YieldCheck,
};
pub use measure_cpu::CpuScoringMeasure;
#[cfg(feature = "cuda")]
pub use measure_cuda::CudaScoringMeasure;
#[cfg(any(test, feature = "mock"))]
pub use measure_mock::{FakeCombo, FakeMeasure, ManualClock};
pub use metrics::{
    cosine_deviation, is_duplicate_free, judge_rows, relative_error, top_k_overlap,
    Plausibility,
};
pub use pareto::pareto_by;
pub use pool::{lower_current_thread_priority, JobHandle, LowPriorityPool};
pub use stats::{Estimator, Spread};
pub use workload::{ScoringWorkload, WorkloadSpec};
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
