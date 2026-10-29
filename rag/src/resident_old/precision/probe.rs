//! Capability probing: what a subengine's hardware and kernel set can offer.
//! Probing is optimistic and measures nothing; unusable combinations are rejected by calibration.

use std::collections::BTreeSet;

use crate::resident::error::PrecisionError;
use crate::resident::kernel::catalog::implemented_capabilities;
use crate::resident::memory::DeviceKind;
use crate::resident::subengine::{DeviceHandle, SubengineKind};

use super::axis::{AxisId, ValueId};
use super::capability::CapabilitySet;
use super::probe_cpu::{CpuProbe, HostCpu};
use super::probe_cuda;

pub trait CapabilityProbe: Send + Sync {
    fn kind(&self) -> SubengineKind;
    fn probe(&self) -> Result<CapabilitySet, PrecisionError>;
}

/// Values for the configuration-driven axes (`batch`, `tile`).
#[derive(Clone, Debug)]
pub struct ProbeConfig {
    pub batches: Vec<u32>,
    pub tiles: Vec<u32>,
    /// Tiles above this length are not offered; calibration checks processability up to it.
    pub max_seq_len: u32,
    /// Kernels the subengine can execute. `None` uses the built-in catalog of the engine kind.
    pub kernels: Option<CapabilitySet>,
}

impl Default for ProbeConfig {
    fn default() -> Self {
        Self {
            batches: vec![1, 8, 32],
            tiles: powers_of_two(16, 512),
            max_seq_len: 512,
            kernels: None,
        }
    }
}

impl ProbeConfig {
    pub(crate) fn batch_values(&self) -> impl Iterator<Item = u32> + '_ {
        self.batches.iter().copied().filter(|b| *b > 0)
    }

    pub(crate) fn tile_values(&self) -> impl Iterator<Item = u32> + '_ {
        let limit = self.max_seq_len;
        self.tiles.iter().copied().filter(move |t| *t > 0 && *t <= limit)
    }
}

pub(crate) fn powers_of_two(from: u32, to: u32) -> Vec<u32> {
    let mut out = Vec::new();
    let mut v = from.max(1).next_power_of_two();
    while v <= to {
        out.push(v);
        match v.checked_mul(2) {
            Some(next) => v = next,
            None => break,
        }
    }
    out
}

/// Keeps only values for which a kernel exists. Axes absent from `kernels` are not governed by it
/// and pass unchanged; a governed axis left without values is removed.
pub fn intersect_with_kernels(set: &CapabilitySet, kernels: &CapabilitySet) -> CapabilitySet {
    let mut out = CapabilitySet::new();
    for (axis, values) in set.iter() {
        let governed = kernels.values(axis);
        for value in values {
            if governed.is_none_or(|ks| ks.contains(value)) {
                out.insert(axis, value);
            } else {
                tracing::debug!(%axis, %value, "capability dropped: no kernel");
            }
        }
    }
    out
}

/// Applies the kernel intersection to the hardware set of an inner probe.
pub struct KernelFilteredProbe<P> {
    inner: P,
    kernels: CapabilitySet,
}

impl<P: CapabilityProbe> KernelFilteredProbe<P> {
    pub fn new(inner: P, kernels: CapabilitySet) -> Self {
        Self { inner, kernels }
    }
}

impl<P: CapabilityProbe> CapabilityProbe for KernelFilteredProbe<P> {
    fn kind(&self) -> SubengineKind {
        self.inner.kind()
    }

    fn probe(&self) -> Result<CapabilitySet, PrecisionError> {
        let hardware = self.inner.probe()?;
        let set = intersect_with_kernels(&hardware, &self.kernels);
        tracing::info!(
            engine = ?self.inner.kind(),
            axes = set.axes().count(),
            values = set.iter().map(|(_, v)| v.len()).sum::<usize>(),
            "capability set probed"
        );
        Ok(set)
    }
}

pub(crate) fn numeric_axis(set: &mut CapabilitySet, axis: &AxisId, values: impl IntoIterator<Item = u32>) {
    let unique: BTreeSet<u32> = values.into_iter().collect();
    for v in unique {
        set.insert(axis, ValueId::numeric(v));
    }
}

/// The only place that branches on the `cuda` feature for probing.
pub fn make_probe(
    device: &DeviceHandle,
    cfg: ProbeConfig,
) -> Result<Box<dyn CapabilityProbe>, PrecisionError> {
    match device.kind() {
        DeviceKind::Cpu => {
            let kernels = cfg
                .kernels
                .clone()
                .unwrap_or_else(|| implemented_capabilities(SubengineKind::Cpu));
            let probe = CpuProbe::new(Box::new(HostCpu), cfg);
            Ok(Box::new(KernelFilteredProbe::new(probe, kernels)))
        }
        DeviceKind::Gpu => {
            let kernels = cfg
                .kernels
                .clone()
                .unwrap_or_else(|| implemented_capabilities(SubengineKind::Cuda));
            let probe = probe_cuda::cuda_probe(device, cfg)?;
            Ok(Box::new(KernelFilteredProbe::new(probe, kernels)))
        }
    }
}

/// Deterministic probe for calibration and profile tests.
#[cfg(any(test, feature = "mock"))]
pub struct FakeCapabilityProbe {
    pub kind: SubengineKind,
    pub set: CapabilitySet,
}

#[cfg(any(test, feature = "mock"))]
impl CapabilityProbe for FakeCapabilityProbe {
    fn kind(&self) -> SubengineKind {
        self.kind
    }

    fn probe(&self) -> Result<CapabilitySet, PrecisionError> {
        Ok(self.set.clone())
    }
}

#[cfg(test)]
#[path = "test/probe.rs"]
mod tests;
