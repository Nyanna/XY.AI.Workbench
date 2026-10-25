//! Persistence of calibration results per model x subengine.
//!
//! A stored record is compared with the current system by its fingerprint:
//! equal is `Calibrated`, different is `Stale` (old result stays usable while a
//! background run replaces it), no file is `Missing`.

use std::collections::BTreeMap;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::resident::error::PrecisionError;
use crate::resident::subengine::{DeviceHandle, SubengineKind};

use super::calibrate::{CalibrationReport, Outcome};
use super::pool::{JobHandle, LowPriorityPool};
use super::probe_cpu::CpuFeatures;
use super::profile::{Profile, ProfileConfig};

pub const SCHEMA_VERSION: u32 = 1;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StoreState {
    Calibrated,
    Stale,
    Missing,
}

/// GPU identity as far as the driver reports it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GpuIdentity {
    pub name: String,
    pub compute_capability: (u32, u32),
    pub driver_version: u32,
}

/// Deliberately minimal: a driver update without a new CUDA driver version goes
/// unnoticed; deleting the calibration file forces a new run.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct Fingerprint {
    pub schema_version: u32,
    pub engine: String,
    pub gpu_name: Option<String>,
    pub gpu_cc: Option<(u32, u32)>,
    pub cuda_driver_version: Option<u32>,
    pub cpu_features: Vec<String>,
    pub model_id: String,
    pub engine_version: String,
}

fn engine_name(kind: SubengineKind) -> &'static str {
    match kind {
        SubengineKind::Cpu => "cpu",
        SubengineKind::Cuda => "cuda",
    }
}

impl Fingerprint {
    pub fn new(
        engine: SubengineKind,
        model_id: &str,
        gpu: Option<GpuIdentity>,
        cpu: CpuFeatures,
    ) -> Self {
        let features = [
            ("avx2", cpu.avx2),
            ("f16c", cpu.f16c),
            ("fma", cpu.fma),
            ("vnni", cpu.vnni),
        ];
        Self {
            schema_version: SCHEMA_VERSION,
            engine: engine_name(engine).to_owned(),
            gpu_name: gpu.as_ref().map(|g| g.name.clone()),
            gpu_cc: gpu.as_ref().map(|g| g.compute_capability),
            cuda_driver_version: gpu.as_ref().map(|g| g.driver_version),
            cpu_features: features
                .iter()
                .filter(|(_, on)| *on)
                .map(|(name, _)| (*name).to_owned())
                .collect(),
            model_id: model_id.to_owned(),
            engine_version: env!("CARGO_PKG_VERSION").to_owned(),
        }
    }

    /// Fingerprint of the running system. GPU fields stay `None` without a CUDA device.
    pub fn current(engine: SubengineKind, model_id: &str, device: Option<&DeviceHandle>) -> Self {
        use super::probe_cpu::{CpuFeatureSource, HostCpu};
        let gpu = device.and_then(gpu_identity);
        Self::new(engine, model_id, gpu, HostCpu.features())
    }
}

#[cfg(feature = "cuda")]
fn gpu_identity(device: &DeviceHandle) -> Option<GpuIdentity> {
    let ctx = device.cuda_context()?;
    let (major, minor) = ctx.compute_capability().ok()?;
    let name = ctx.name().ok()?;
    let mut version: std::os::raw::c_int = 0;
    // SAFETY: the out-pointer is valid for the duration of the call.
    let status = unsafe { cudarc::driver::sys::cuDriverGetVersion(&mut version) };
    if status != cudarc::driver::sys::CUresult::CUDA_SUCCESS {
        return None;
    }
    Some(GpuIdentity {
        name,
        compute_capability: (u32::try_from(major).ok()?, u32::try_from(minor).ok()?),
        driver_version: u32::try_from(version).ok()?,
    })
}

#[cfg(not(feature = "cuda"))]
fn gpu_identity(_device: &DeviceHandle) -> Option<GpuIdentity> {
    None
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StoredProfile {
    pub position: f32,
    pub config: BTreeMap<String, String>,
}

fn config_map(config: &ProfileConfig) -> BTreeMap<String, String> {
    config.iter().map(|(a, v)| (a.to_string(), v.to_string())).collect()
}

impl From<&Profile> for StoredProfile {
    fn from(profile: &Profile) -> Self {
        Self { position: profile.position, config: config_map(&profile.config) }
    }
}

impl StoredProfile {
    pub fn to_profile(&self) -> Result<Profile, PrecisionError> {
        let config = self
            .config
            .iter()
            .fold(ProfileConfig::new(), |cfg, (axis, value)| cfg.with(axis.as_str(), value.as_str()));
        Profile::new(self.position, config)
    }
}

/// A combination discarded for validity reasons (maximum deviation, degenerate output, ...).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RejectedCombo {
    pub config: BTreeMap<String, String>,
    pub reason: String,
}

/// A combination that was not fully measured.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrunedCombo {
    pub config: BTreeMap<String, String>,
    pub reason: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct CalibrationRecord {
    pub fingerprint: Fingerprint,
    pub profiles: Vec<StoredProfile>,
    pub unstable: bool,
    pub rejections: Vec<RejectedCombo>,
    pub pruning: Vec<PrunedCombo>,
}

impl CalibrationRecord {
    pub fn from_report(fingerprint: Fingerprint, report: &CalibrationReport, profiles: &[Profile]) -> Self {
        let mut rejections = Vec::new();
        let mut pruning = Vec::new();
        for entry in &report.entries {
            let config = config_map(&entry.cfg.combo);
            match &entry.outcome {
                Outcome::Rejected(validity) => {
                    rejections.push(RejectedCombo { config, reason: validity.as_str().to_owned() });
                }
                Outcome::Dominated => pruning.push(PrunedCombo { config, reason: "dominated".into() }),
                Outcome::Skipped => pruning.push(PrunedCombo { config, reason: "skipped".into() }),
                Outcome::Measured(_) => {}
            }
        }
        Self {
            fingerprint,
            profiles: profiles.iter().map(StoredProfile::from).collect(),
            unstable: report.unstable() > 0,
            rejections,
            pruning,
        }
    }

    pub fn active_profiles(&self) -> Result<Vec<Profile>, PrecisionError> {
        self.profiles.iter().map(StoredProfile::to_profile).collect()
    }
}

pub trait CalibrationStore: Send + Sync {
    /// `None` if nothing is stored.
    fn load(&self) -> Result<Option<CalibrationRecord>, PrecisionError>;

    /// Replaces the stored record such that readers see either the old or the new one.
    fn save_atomic(&self, record: &CalibrationRecord) -> Result<(), PrecisionError>;
}

pub fn state(record: Option<&CalibrationRecord>, current: &Fingerprint) -> StoreState {
    match record {
        None => StoreState::Missing,
        Some(r) if r.fingerprint == *current => StoreState::Calibrated,
        Some(_) => StoreState::Stale,
    }
}

/// What a subengine starts with, and whether a background run is due.
#[derive(Clone, Debug)]
pub struct Resolution {
    pub state: StoreState,
    /// Present for `Calibrated` and `Stale`; `Missing` means: use the most precise valid combination.
    pub record: Option<CalibrationRecord>,
}

impl Resolution {
    pub fn needs_calibration(&self) -> bool {
        self.state != StoreState::Calibrated
    }
}

/// Never fails: an unreadable file is treated like a missing one.
pub fn resolve(store: &dyn CalibrationStore, current: &Fingerprint) -> Resolution {
    let record = store.load().unwrap_or_else(|e| {
        tracing::warn!(error = %e, "calibration record unreadable");
        None
    });
    let state = state(record.as_ref(), current);
    match state {
        StoreState::Calibrated => {
            tracing::debug!(model = %current.model_id, schema = current.schema_version, "calibration loaded");
        }
        StoreState::Stale => {
            tracing::info!(model = %current.model_id, schema = current.schema_version, "calibration stale, using old result");
        }
        StoreState::Missing => {
            tracing::warn!(model = %current.model_id, "no calibration, using most precise combination");
        }
    }
    let record = if state == StoreState::Missing { None } else { record };
    Resolution { state, record }
}

/// Runs `calibrate` on the low-priority pool and replaces the stored record atomically.
pub fn recalibrate_in_background<F>(
    pool: &LowPriorityPool,
    store: Arc<dyn CalibrationStore>,
    calibrate: F,
) -> JobHandle<Result<(), PrecisionError>>
where
    F: FnOnce() -> Result<CalibrationRecord, PrecisionError> + Send + 'static,
{
    pool.submit(move || {
        let record = calibrate()?;
        store.save_atomic(&record)?;
        tracing::info!(
            model = %record.fingerprint.model_id,
            schema = record.fingerprint.schema_version,
            unstable = record.unstable,
            "calibration record replaced"
        );
        Ok(())
    })
}

/// One JSON file per model and subengine; the application chooses the path.
#[derive(Clone, Debug)]
pub struct FileCalibrationStore {
    path: PathBuf,
}

fn store_error(e: impl std::fmt::Display) -> PrecisionError {
    PrecisionError::Store { reason: e.to_string() }
}

impl FileCalibrationStore {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    fn temp_path(&self) -> PathBuf {
        let mut name = self.path.file_name().map(|n| n.to_os_string()).unwrap_or_default();
        name.push(format!(".{}.tmp", std::process::id()));
        self.path.with_file_name(name)
    }
}

impl CalibrationStore for FileCalibrationStore {
    fn load(&self) -> Result<Option<CalibrationRecord>, PrecisionError> {
        match fs::read(&self.path) {
            Ok(bytes) => serde_json::from_slice(&bytes).map(Some).map_err(store_error),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(e) => Err(store_error(e)),
        }
    }

    fn save_atomic(&self, record: &CalibrationRecord) -> Result<(), PrecisionError> {
        let json = serde_json::to_vec_pretty(record).map_err(store_error)?;
        let tmp = self.temp_path();
        let write = || -> std::io::Result<()> {
            if let Some(dir) = self.path.parent().filter(|d| !d.as_os_str().is_empty()) {
                fs::create_dir_all(dir)?;
            }
            let mut file = fs::File::create(&tmp)?;
            file.write_all(&json)?;
            file.sync_all()?;
            fs::rename(&tmp, &self.path)
        };
        write().map_err(|e| {
            let _ = fs::remove_file(&tmp);
            store_error(e)
        })
    }
}

#[cfg(test)]
#[path = "test/store.rs"]
mod tests;
