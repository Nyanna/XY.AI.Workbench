//! Capability detection per device kind, reduced to the kernels that exist.

use std::fmt;

use log::{info, warn};

use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::a::supported_precisions::precision_capabilities;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::l010::capability_model::{
    axes, values, CapabilitySet, Name,
};
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::l010::device_lane_event::{
    DeviceHandle, DeviceKind,
};
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::l010::target_env::ComputeCapability;

const DEFAULT_BATCH_SIZES: [u32; 3] = [1, 8, 32];
const DEFAULT_TILE_SIZES: [u32; 6] = [16, 32, 64, 128, 256, 512];
const DEFAULT_MAX_SEQ_LEN: u32 = 512;
const TENSOR_CORE_EXCLUSIONS: [&str; 4] = ["TU116", "TU117", "GTX 16", "GTX16"];
const MIN_FP16_CC: ComputeCapability = ComputeCapability::new(5, 3);
const MIN_DP4A_CC: ComputeCapability = ComputeCapability::new(6, 1);

/// Supplied by the GPU binding; bound to one device context.
pub trait GpuProbe: Send + Sync {
    /// `None` if the device context is not available.
    fn compute_capability(&self) -> Option<ComputeCapability>;
    /// Marketing or chip name; `None` if unknown.
    fn device_name(&self) -> Option<String>;
}

/// Detection input; `None` selects the default.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct CapabilityConfig {
    pub batch_sizes: Option<Vec<u32>>,
    pub tile_sizes: Option<Vec<u32>>,
    pub max_seq_len: Option<u32>,
    /// Given kernel set; `None` selects the built-in catalog.
    pub kernels: Option<CapabilitySet>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct CpuFeatures {
    pub avx2: bool,
    pub f16c: bool,
    pub fma: bool,
    pub vnni: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DetectedCapabilities {
    pub capabilities: CapabilitySet,
    pub max_seq_len: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DetectError {
    NoGpuSupport,
    NoDeviceContext,
    UnsupportedDevice(String),
}

impl fmt::Display for DetectError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DetectError::NoGpuSupport => write!(f, "gpu device requested but no gpu probe was given"),
            DetectError::NoDeviceContext => write!(f, "gpu probe has no device context"),
            DetectError::UnsupportedDevice(kind) => write!(f, "unsupported device kind: {kind}"),
        }
    }
}

impl std::error::Error for DetectError {}

pub fn detect_capabilities(
    device: DeviceHandle,
    gpu: Option<&dyn GpuProbe>,
    cfg: &CapabilityConfig,
) -> Result<DetectedCapabilities, DetectError> {
    let kind = device.kind();
    let (mut caps, subengine) = match kind {
        DeviceKind::Cpu => (cpu_capabilities(detect_cpu_features(), logical_cores()), "cpu"),
        DeviceKind::Gpu => (detect_gpu(gpu)?, "gpu"),
        #[allow(unreachable_patterns)]
        other => return Err(DetectError::UnsupportedDevice(format!("{other:?}"))),
    };

    let max_seq_len = add_numeric_axes(&mut caps, cfg);

    let caps = match &cfg.kernels {
        Some(kernels) => reduce_to_kernels(caps, kernels, subengine),
        None => reduce_to_kernels(caps, &builtin_catalog(kind), subengine),
    };

    info!(
        "capabilities subengine={} axes={} values={}",
        subengine,
        caps.axes().count(),
        caps.iter().count()
    );
    Ok(DetectedCapabilities {
        capabilities: caps,
        max_seq_len,
    })
}

fn detect_gpu(gpu: Option<&dyn GpuProbe>) -> Result<CapabilitySet, DetectError> {
    let probe = gpu.ok_or(DetectError::NoGpuSupport)?;
    let cc = probe
        .compute_capability()
        .ok_or(DetectError::NoDeviceContext)?;
    let name = probe.device_name();
    info!(
        "tensor cores: {} cc={}.{} name={}",
        if has_tensor_cores(cc, name.as_deref()) {
            "yes"
        } else {
            "no"
        },
        cc.major,
        cc.minor,
        name.as_deref().unwrap_or("unknown")
    );
    Ok(gpu_capabilities(cc))
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub fn detect_cpu_features() -> CpuFeatures {
    CpuFeatures {
        avx2: is_x86_feature_detected!("avx2"),
        f16c: is_x86_feature_detected!("f16c"),
        fma: is_x86_feature_detected!("fma"),
        vnni: is_x86_feature_detected!("avx512vnni") || is_x86_feature_detected!("avxvnni"),
    }
}

#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
pub fn detect_cpu_features() -> CpuFeatures {
    CpuFeatures::default()
}

pub fn logical_cores() -> usize {
    std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1)
}

pub fn cpu_capabilities(f: CpuFeatures, logical_cores: usize) -> CapabilitySet {
    let mut caps = CapabilitySet::new();
    caps.add(axes::operands(), values::fp32());
    if f.f16c {
        caps.add(axes::operands(), values::fp16_storage_fp32_compute());
    }
    caps.add(axes::scoring(), values::fp32());
    if f.avx2 || f.vnni {
        caps.add(axes::scoring(), values::int8());
    }
    if f.vnni {
        caps.add(axes::scoring(), values::int8_vnni());
    }
    for n in thread_counts(logical_cores) {
        caps.add(axes::thread_count(), Name::from_number(u64::from(n)));
    }
    caps
}

pub fn gpu_capabilities(cc: ComputeCapability) -> CapabilitySet {
    let mut caps = CapabilitySet::new();
    caps.add(axes::operands(), values::fp32());
    caps.add(axes::accumulator(), values::fp32());
    caps.add(axes::scoring(), values::fp32());
    if cc >= MIN_FP16_CC {
        caps.add(axes::operands(), values::fp16());
        caps.add(axes::accumulator(), values::fp16());
        caps.add(axes::scoring(), values::fp16());
    }
    if cc >= MIN_DP4A_CC {
        caps.add(axes::scoring(), values::int8_dp4a());
    }
    caps.add(axes::fusion(), values::on());
    caps.add(axes::fusion(), values::off());
    caps
}

pub fn has_tensor_cores(cc: ComputeCapability, device_name: Option<&str>) -> bool {
    if cc.major < 7 {
        return false;
    }
    match device_name {
        None => true,
        Some(name) => {
            let upper = name.to_ascii_uppercase();
            !TENSOR_CORE_EXCLUSIONS.iter().any(|p| upper.contains(p))
        }
    }
}

pub fn thread_counts(logical_cores: usize) -> Vec<u32> {
    let n = u32::try_from(logical_cores.max(1)).unwrap_or(u32::MAX);
    let mut out = Vec::new();
    let mut p: u32 = 1;
    while p <= n {
        out.push(p);
        match p.checked_mul(2) {
            Some(next) => p = next,
            None => break,
        }
    }
    if out.last() != Some(&n) {
        out.push(n);
    }
    out
}

pub fn builtin_catalog(kind: DeviceKind) -> CapabilitySet {
    let mut catalog = precision_capabilities(kind);
    match kind {
        DeviceKind::Cpu => {
            catalog.add(axes::operands(), values::fp16_storage_fp32_compute());
            catalog.add(axes::scoring(), values::int8());
            catalog.add(axes::scoring(), values::int8_vnni());
        }
        DeviceKind::Gpu => {
            catalog.add(axes::scoring(), values::int8_dp4a());
            catalog.add(axes::accumulator(), values::fp32());
            catalog.add(axes::accumulator(), values::fp16());
            catalog.add(axes::fusion(), values::on());
            catalog.add(axes::fusion(), values::off());
        }
        #[allow(unreachable_patterns)]
        _ => {}
    }
    catalog
}

pub fn reduce_to_kernels(
    caps: CapabilitySet,
    kernels: &CapabilitySet,
    subengine: &str,
) -> CapabilitySet {
    let mut out = CapabilitySet::new();
    for axis in caps.axes() {
        let Some(have) = caps.values(axis) else {
            continue;
        };
        let Some(known) = kernels.values(axis) else {
            for value in have {
                out.add(axis.clone(), value.clone());
            }
            continue;
        };
        let mut kept = 0usize;
        for value in have {
            if known.contains(value) {
                out.add(axis.clone(), value.clone());
                kept += 1;
            } else {
                info!(
                    "capability removed subengine={subengine} axis={axis} value={value}: no kernel"
                );
            }
        }
        if kept == 0 {
            warn!("capability axis removed subengine={subengine} axis={axis}: no kernel");
        }
    }
    out
}

fn add_numeric_axes(caps: &mut CapabilitySet, cfg: &CapabilityConfig) -> u32 {
    let max_seq_len = cfg.max_seq_len.unwrap_or(DEFAULT_MAX_SEQ_LEN);
    let batch_sizes = cfg.batch_sizes.as_deref().unwrap_or(&DEFAULT_BATCH_SIZES);
    let tile_sizes = cfg.tile_sizes.as_deref().unwrap_or(&DEFAULT_TILE_SIZES);

    let batches = batch_sizes.iter().copied().filter(|&b| {
        let keep = b > 0;
        if !keep {
            warn!("batch_size value dropped: {b} must be > 0");
        }
        keep
    });
    add_axis(caps, axes::batch_size(), batches);

    let tiles = tile_sizes.iter().copied().filter(|&t| {
        let keep = t > 0 && t <= max_seq_len;
        if !keep {
            warn!("tile_size value dropped: {t} not in 1..={max_seq_len}");
        }
        keep
    });
    add_axis(caps, axes::tile_size(), tiles);
    max_seq_len
}

fn add_axis(caps: &mut CapabilitySet, axis: Name, vals: impl Iterator<Item = u32>) {
    let mut added = false;
    for v in vals {
        caps.add(axis.clone(), Name::from_number(u64::from(v)));
        added = true;
    }
    if !added {
        warn!("axis {axis} not created: no valid values");
    }
}

#[cfg(test)]
#[path = "test/capability_detection.rs"]
mod tests;
