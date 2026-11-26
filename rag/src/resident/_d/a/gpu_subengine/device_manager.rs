use std::collections::BTreeSet;
use std::sync::Arc;

use log::warn;

use super::backend::{GpuBackend3, GpuDeviceInfo};
use super::config::{GpuDeviceManagerConfig, GpuShared};
use super::deps::*;
use super::error::GpuError;
use super::subengine::GpuSubengine;

/// Manager of the GPU devices: retains one primary context per usable device and keeps it
/// for its own lifetime.
pub struct GpuDeviceManager {
    _backend: Arc<dyn GpuBackend3>,
    infos: Vec<GpuDeviceInfo>,
    engines: Vec<Arc<GpuSubengine>>,
}

static_assertions::assert_impl_all!(GpuDeviceManager : Send, Sync);

impl GpuDeviceManager {
    /// Enumerates devices, retains one primary context per selected device, detects the
    /// capabilities and builds one sub-engine per device. Loads nothing, reserves no VRAM.
    pub fn new(
        backend: Arc<dyn GpuBackend3>,
        shared: GpuShared,
        cfg: GpuDeviceManagerConfig,
    ) -> Result<GpuDeviceManager, GpuError> {
        cfg.gpu.validate()?;
        let available = backend.devices().map_err(GpuError::Backend)?;
        let explicit = cfg.devices.is_some();
        let selected: Vec<GpuDeviceInfo> = match &cfg.devices {
            Some(list) => {
                let wanted: BTreeSet<u32> = list.iter().copied().collect();
                let mut chosen = Vec::with_capacity(wanted.len());
                for ordinal in wanted {
                    let info = available.iter().find(|d| d.ordinal == ordinal).ok_or_else(|| {
                        GpuError::InvalidConfig(format!("device {ordinal} does not exist"))
                    })?;
                    chosen.push(info.clone());
                }
                chosen
            }
            None => {
                let mut all = available;
                all.sort_by_key(|d| d.ordinal);
                all.dedup_by_key(|d| d.ordinal);
                all
            }
        };
        let mut infos = Vec::with_capacity(selected.len());
        let mut engines = Vec::with_capacity(selected.len());
        for info in selected {
            let ordinal = info.ordinal;
            if let Err(reason) = backend.retain_primary_context(ordinal) {
                if explicit {
                    return Err(GpuError::Backend(reason));
                }
                warn!("gpu:{ordinal}: primary context not retained, device skipped: {reason}");
                continue;
            }
            let probe = backend.probe(ordinal);
            let detected = match detect_capabilities(
                DeviceHandle::Gpu(ordinal),
                Some(&*probe),
                &cfg.gpu.capability,
            ) {
                Ok(detected) => detected,
                Err(e) if explicit => return Err(GpuError::Capability(e)),
                Err(e) => {
                    warn!("gpu:{ordinal}: capability detection failed, device skipped: {e}");
                    continue;
                }
            };
            let engine = GpuSubengine::build(
                info.clone(),
                Arc::clone(&backend),
                shared.clone(),
                cfg.gpu.clone(),
                detected,
            )?;
            infos.push(info);
            engines.push(engine);
        }
        if engines.is_empty() {
            return Err(GpuError::NoDevice);
        }
        Ok(GpuDeviceManager {
            _backend: backend,
            infos,
            engines,
        })
    }

    /// The usable devices.
    pub fn devices(&self) -> &[GpuDeviceInfo] {
        &self.infos
    }

    /// Ordered by ordinal.
    pub fn subengines(&self) -> Vec<Arc<GpuSubengine>> {
        self.engines.clone()
    }

    pub fn subengine(&self, ordinal: u32) -> Option<Arc<GpuSubengine>> {
        self.engines.iter().find(|e| e.info().ordinal == ordinal).cloned()
    }
}

#[cfg(test)]
#[path = "test/device_manager.rs"]
mod tests;
