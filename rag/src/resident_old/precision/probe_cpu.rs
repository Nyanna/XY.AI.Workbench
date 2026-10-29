//! CPU capability probe with an injectable feature source.

use crate::resident::error::PrecisionError;
use crate::resident::subengine::SubengineKind;

use super::capability::CapabilitySet;
use super::known::{
    ACCUMULATOR, FP16_TO_FP32, FP32, INT8, INT8_VNNI, OPERANDS, SCORING, THREADS, TILE,
};
use super::probe::{numeric_axis, CapabilityProbe, ProbeConfig};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct CpuFeatures {
    pub avx2: bool,
    pub f16c: bool,
    pub fma: bool,
    pub vnni: bool,
}

pub trait CpuFeatureSource: Send + Sync {
    fn features(&self) -> CpuFeatures;
    fn logical_cores(&self) -> usize;
}

pub struct HostCpu;

impl CpuFeatureSource for HostCpu {
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    fn features(&self) -> CpuFeatures {
        CpuFeatures {
            avx2: std::is_x86_feature_detected!("avx2"),
            f16c: std::is_x86_feature_detected!("f16c"),
            fma: std::is_x86_feature_detected!("fma"),
            vnni: std::is_x86_feature_detected!("avxvnni")
                || std::is_x86_feature_detected!("avx512vnni"),
        }
    }

    #[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
    fn features(&self) -> CpuFeatures {
        CpuFeatures::default()
    }

    fn logical_cores(&self) -> usize {
        std::thread::available_parallelism().map_or(1, |n| n.get())
    }
}

/// Fixed source for tests and for forcing a feature level.
pub struct FixedCpu {
    pub features: CpuFeatures,
    pub logical_cores: usize,
}

impl CpuFeatureSource for FixedCpu {
    fn features(&self) -> CpuFeatures {
        self.features
    }

    fn logical_cores(&self) -> usize {
        self.logical_cores
    }
}

pub struct CpuProbe {
    source: Box<dyn CpuFeatureSource>,
    cfg: ProbeConfig,
}

impl CpuProbe {
    pub fn new(source: Box<dyn CpuFeatureSource>, cfg: ProbeConfig) -> Self {
        Self { source, cfg }
    }
}

/// Thread counts `1, 2, 4, ...` up to and including `cores`.
fn thread_counts(cores: usize) -> Vec<u32> {
    let cores = u32::try_from(cores.max(1)).unwrap_or(u32::MAX);
    let mut counts = super::probe::powers_of_two(1, cores);
    counts.push(cores);
    counts
}

impl CapabilityProbe for CpuProbe {
    fn kind(&self) -> SubengineKind {
        SubengineKind::Cpu
    }

    fn probe(&self) -> Result<CapabilitySet, PrecisionError> {
        let f = self.source.features();
        let cores = self.source.logical_cores();
        let mut set = CapabilitySet::new();

        set.insert(&*OPERANDS, &*FP32);
        set.insert(&*ACCUMULATOR, &*FP32);
        set.insert(&*SCORING, &*FP32);
        if f.f16c {
            set.insert(&*OPERANDS, &*FP16_TO_FP32);
        }
        if f.avx2 || f.vnni {
            set.insert(&*SCORING, &*INT8);
        }
        if f.vnni {
            set.insert(&*SCORING, &*INT8_VNNI);
        }

        numeric_axis(&mut set, &THREADS, thread_counts(cores));
        numeric_axis(&mut set, &TILE, self.cfg.tile_values());

        tracing::debug!(?f, cores, "cpu features");
        Ok(set)
    }
}

#[cfg(test)]
#[path = "test/probe_cpu.rs"]
mod tests;
