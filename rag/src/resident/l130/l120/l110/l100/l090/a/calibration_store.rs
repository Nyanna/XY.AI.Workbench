//! Persistence of calibration results per (subengine, model) with fingerprint-based staleness.
use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::ffi::OsString;
use std::fmt;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use crate::resident::l130::l120::l110::l100::l090::l080::a::capability_detection::{
    detect_cpu_features, CpuFeatures,
};
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::a::calibration_run::{
    run_calibration, CalibrationBackend, CalibrationConfig, CalibrationError,
    CalibrationReport, Combination, DiscardReason, Outcome, Stage, Target,
};
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::a::model_descriptor::ModelDescriptor;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::a::pareto_profiles::{
    derive_profiles, Measurement, PrecisionProfile, DEFAULT_MERGE_THRESHOLD,
};
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::l010::background_pool::{
    BackgroundPool, JobFuture,
};
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::l010::capability_model::{
    axes, values, Axis, CapabilitySet, Name, Value,
};
pub const SCHEMA_VERSION: u32 = 1;
const UNKNOWN: &str = "unknown";
const FILE_SUFFIX: &str = ".calib.txt";
static TMP_COUNTER: AtomicU64 = AtomicU64::new(0);
/// Supplied by the GPU binding; `None` means unknown.
pub trait DriverInfo: Send + Sync {
    fn gpu_name(&self) -> Option<String>;
    fn compute_capability(&self) -> Option<(u32, u32)>;
    fn driver_version(&self) -> Option<String>;
}
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Fingerprint {
    pub schema_version: u32,
    pub subengine: String,
    pub gpu_name: String,
    pub compute_capability: String,
    pub driver_version: String,
    pub cpu_features: String,
    pub model_id: String,
    pub engine_version: String,
}
fn cpu_features_text(f: CpuFeatures) -> String {
    [(f.avx2, "avx2"), (f.f16c, "f16c"), (f.fma, "fma"), (f.vnni, "vnni")]
        .iter()
        .filter(|(set, _)| *set)
        .map(|(_, name)| *name)
        .collect::<Vec<_>>()
        .join(",")
}
impl Fingerprint {
    pub fn detect(
        subengine: &str,
        model_id: &str,
        engine_version: &str,
        driver: Option<&dyn DriverInfo>,
    ) -> Fingerprint {
        let (gpu_name, compute_capability, driver_version) = if subengine == "gpu" {
            (
                driver.and_then(|d| d.gpu_name()).unwrap_or_else(|| UNKNOWN.to_string()),
                driver
                    .and_then(|d| d.compute_capability())
                    .map(|(major, minor)| format!("{major}.{minor}"))
                    .unwrap_or_else(|| UNKNOWN.to_string()),
                driver
                    .and_then(|d| d.driver_version())
                    .unwrap_or_else(|| UNKNOWN.to_string()),
            )
        } else {
            (String::new(), String::new(), String::new())
        };
        Fingerprint {
            schema_version: SCHEMA_VERSION,
            subengine: subengine.to_string(),
            gpu_name,
            compute_capability,
            driver_version,
            cpu_features: cpu_features_text(detect_cpu_features()),
            model_id: model_id.to_string(),
            engine_version: engine_version.to_string(),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Annotated {
    pub combination: Combination,
    pub reason: String,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CalibrationRecord {
    pub fingerprint: Fingerprint,
    pub profiles: Vec<PrecisionProfile>,
    pub unstable: bool,
    pub discarded: Vec<Annotated>,
    pub pruned: Vec<Annotated>,
}
#[derive(Clone, Debug, PartialEq)]
pub enum PersistError {
    NotFound,
    Unreadable(String),
    Io(String),
    Pool(String),
    Calibration(CalibrationError),
    NoProfiles,
}
impl fmt::Display for PersistError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PersistError::NotFound => write!(f, "calibration file not found"),
            PersistError::Unreadable(m) => write!(f, "calibration file unreadable: {m}"),
            PersistError::Io(m) => write!(f, "calibration i/o error: {m}"),
            PersistError::Pool(m) => write!(f, "background pool error: {m}"),
            PersistError::Calibration(e) => write!(f, "calibration failed: {e}"),
            PersistError::NoProfiles => write!(f, "calibration produced no profile"),
        }
    }
}
impl std::error::Error for PersistError {}
#[derive(Clone)]
pub struct CalibrationRequest {
    pub fingerprint: Fingerprint,
    pub capabilities: CapabilitySet,
    pub model: ModelDescriptor,
    pub backend: Arc<dyn CalibrationBackend>,
    pub config: CalibrationConfig,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MissingPolicy {
    MostPreciseValid,
    ExplicitCalibrate,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MissingResolution {
    Fallback(Combination),
    CalibrateCommand,
}
pub enum Resolution {
    Calibrated { record: CalibrationRecord },
    Stale {
        record: CalibrationRecord,
        /// `None` if a recalibration for the same key is already running or submit failed.
        recalibration: Option<JobFuture<Result<CalibrationRecord, PersistError>>>,
    },
    Missing(MissingResolution),
}
type InFlightKey = (String, String);
struct InFlightGuard {
    set: Arc<Mutex<HashSet<InFlightKey>>>,
    key: InFlightKey,
}
impl Drop for InFlightGuard {
    fn drop(&mut self) {
        lock(&self.set).remove(&self.key);
    }
}
/// Send + Sync; shareable via `Arc`.
pub struct CalibrationStore {
    dir: PathBuf,
    pool: Arc<BackgroundPool>,
    in_flight: Arc<Mutex<HashSet<InFlightKey>>>,
}
impl CalibrationStore {
    pub fn new(dir: PathBuf, pool: Arc<BackgroundPool>) -> CalibrationStore {
        CalibrationStore {
            dir,
            pool,
            in_flight: Arc::new(Mutex::new(HashSet::new())),
        }
    }
    pub fn dir(&self) -> &Path {
        &self.dir
    }
    pub fn path_for(&self, subengine: &str, model_id: &str) -> PathBuf {
        record_path(&self.dir, subengine, model_id)
    }
    pub fn load(
        &self,
        subengine: &str,
        model_id: &str,
    ) -> Result<CalibrationRecord, PersistError> {
        load_file(&self.path_for(subengine, model_id))
    }
    pub fn save(&self, record: &CalibrationRecord) -> Result<(), PersistError> {
        save_record(&self.dir, record)
    }
    pub fn resolve(
        &self,
        req: &CalibrationRequest,
        policy: MissingPolicy,
    ) -> Resolution {
        let fp = &req.fingerprint;
        match self.load(&fp.subengine, &fp.model_id) {
            Ok(record) if record.fingerprint == *fp => Resolution::Calibrated { record },
            Ok(record) => {
                let recalibration = match self.request_calibration(req.clone()) {
                    Ok(future) => future,
                    Err(e) => {
                        log::warn!(
                            "recalibration not started subengine={} model_id={}: {e}", fp
                            .subengine, fp.model_id
                        );
                        None
                    }
                };
                Resolution::Stale {
                    record,
                    recalibration,
                }
            }
            Err(e) => {
                if let PersistError::Unreadable(m) = &e {
                    log::warn!(
                        "calibration file unreadable subengine={} model_id={}: {m}", fp
                        .subengine, fp.model_id
                    );
                }
                match policy {
                    MissingPolicy::ExplicitCalibrate => {
                        Resolution::Missing(MissingResolution::CalibrateCommand)
                    }
                    MissingPolicy::MostPreciseValid => {
                        log::warn!(
                            "no calibration subengine={} model_id={}; using most precise combination",
                            fp.subengine, fp.model_id
                        );
                        Resolution::Missing(
                            MissingResolution::Fallback(
                                most_precise_combination(&req.capabilities),
                            ),
                        )
                    }
                }
            }
        }
    }
    pub fn request_calibration(
        &self,
        req: CalibrationRequest,
    ) -> Result<
        Option<JobFuture<Result<CalibrationRecord, PersistError>>>,
        PersistError,
    > {
        let key = (req.fingerprint.subengine.clone(), req.fingerprint.model_id.clone());
        if !lock(&self.in_flight).insert(key.clone()) {
            return Ok(None);
        }
        let guard = InFlightGuard {
            set: Arc::clone(&self.in_flight),
            key,
        };
        let dir = self.dir.clone();
        let submitted = self
            .pool
            .submit(move || {
                let _guard = guard;
                run_job(&dir, &req)
            });
        match submitted {
            Ok(future) => Ok(Some(future)),
            Err(e) => Err(PersistError::Pool(e.to_string())),
        }
    }
}
fn run_job(
    dir: &Path,
    req: &CalibrationRequest,
) -> Result<CalibrationRecord, PersistError> {
    let fp = &req.fingerprint;
    let outcome = calibrate_record(req)
        .and_then(|record| {
            let old = load_file(&record_path(dir, &fp.subengine, &fp.model_id))
                .ok()
                .map(|r| r.fingerprint);
            save_record(dir, &record)?;
            Ok((record, old))
        });
    match outcome {
        Ok((record, old)) => {
            log::info!(
                "calibration replaced subengine={} model_id={} profiles={} new={:?} old={:?}",
                fp.subengine, fp.model_id, record.profiles.len(), record.fingerprint, old
            );
            Ok(record)
        }
        Err(e) => {
            log::warn!(
                "recalibration failed subengine={} model_id={}: {e}", fp.subengine, fp
                .model_id
            );
            Err(e)
        }
    }
}
pub fn calibrate_record(
    req: &CalibrationRequest,
) -> Result<CalibrationRecord, PersistError> {
    let target = Target {
        subengine: req.fingerprint.subengine.clone(),
        model_id: req.fingerprint.model_id.clone(),
    };
    let report = run_calibration(
            target,
            &req.capabilities,
            &req.model,
            &*req.backend,
            &req.config,
        )
        .map_err(PersistError::Calibration)?;
    record_from_report(req.fingerprint.clone(), &report)
}
fn discard_text(reason: &DiscardReason) -> String {
    match reason {
        DiscardReason::Failed(m) => format!("failed: {m}"),
        DiscardReason::NonFinite => "non-finite deviation".to_string(),
        DiscardReason::ExceedsMax => "exceeds max deviation".to_string(),
        DiscardReason::Other(m) => m.clone(),
    }
}
pub fn record_from_report(
    fp: Fingerprint,
    report: &CalibrationReport,
) -> Result<CalibrationRecord, PersistError> {
    let measurements: Vec<Measurement> = report
        .summary
        .front
        .iter()
        .map(|p| Measurement {
            config: p.combination.values().clone(),
            time: p.time.as_secs_f64(),
            deviation: p.deviation,
        })
        .collect();
    let profiles = match derive_profiles(&measurements, DEFAULT_MERGE_THRESHOLD) {
        Ok(p) if !p.is_empty() => p,
        _ => return Err(PersistError::NoProfiles),
    };
    let mut unstable = false;
    let mut discarded = Vec::new();
    let mut pruned = Vec::new();
    for entry in &report.entries {
        match &entry.outcome {
            Outcome::Measured {
                stable: false,
                ..
            } if matches!(entry.stage, Stage::Verification | Stage::Full) => {
                unstable = true;
            }
            Outcome::Discarded(reason) => {
                discarded
                    .push(Annotated {
                        combination: entry.combination.clone(),
                        reason: discard_text(reason),
                    })
            }
            Outcome::Dominated => {
                pruned
                    .push(Annotated {
                        combination: entry.combination.clone(),
                        reason: "dominated".to_string(),
                    })
            }
            _ => {}
        }
    }
    Ok(CalibrationRecord {
        fingerprint: fp,
        profiles,
        unstable,
        discarded,
        pruned,
    })
}
fn is_precision_axis(axis: &Axis) -> bool {
    *axis == axes::operands() || *axis == axes::accumulator() || *axis == axes::scoring()
}
fn precision_rank(v: &Value) -> u8 {
    if *v == values::fp32() {
        4
    } else if *v == values::fp16_storage_fp32_compute() {
        3
    } else if *v == values::fp16() {
        2
    } else if *v == values::int8() || *v == values::int8_vnni()
        || *v == values::int8_dp4a()
    {
        1
    } else {
        0
    }
}
/// Numeric values first (ascending), then the rest in byte order.
fn ordered_values(set: &BTreeSet<Value>) -> Vec<Value> {
    let mut out: Vec<Value> = set.iter().cloned().collect();
    out.sort_by_cached_key(|v| {
        let n = v.as_str().parse::<u64>().ok();
        (n.is_none(), n.unwrap_or(0), v.clone())
    });
    out
}
fn most_precise_combination(caps: &CapabilitySet) -> Combination {
    let mut map = BTreeMap::new();
    for axis in caps.axes() {
        let Some(set) = caps.values(axis) else {
            continue;
        };
        let ordered = ordered_values(set);
        let pick = if is_precision_axis(axis) {
            ordered
                .iter()
                .fold(
                    None,
                    |best: Option<&Value>, v| match best {
                        Some(b) if precision_rank(b) >= precision_rank(v) => Some(b),
                        _ => Some(v),
                    },
                )
        } else {
            ordered.first()
        };
        if let Some(v) = pick {
            map.insert(axis.clone(), v.clone());
        }
    }
    Combination::from_values(map).unwrap_or_default()
}
fn path_component(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for &b in s.as_bytes() {
        if b.is_ascii_alphanumeric() || matches!(b, b'.' | b'_' | b'-') {
            out.push(b as char);
        } else {
            out.push_str(&format!("%{b:02X}"));
        }
    }
    out
}
fn record_path(dir: &Path, subengine: &str, model_id: &str) -> PathBuf {
    dir.join(
        format!(
            "{}__{}{FILE_SUFFIX}", path_component(subengine), path_component(model_id)
        ),
    )
}
fn is_escaped(c: char) -> bool {
    matches!(c, '%' | '|' | ';' | '=' | '\n' | '\r')
}
fn escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        if is_escaped(c) {
            out.push_str(&format!("%{:02X}", c as u8));
        } else {
            out.push(c);
        }
    }
    out
}
fn hex_digit(b: u8) -> Option<u8> {
    match b {
        b'0'..=b'9' => Some(b - b'0'),
        b'a'..=b'f' => Some(b - b'a' + 10),
        b'A'..=b'F' => Some(b - b'A' + 10),
        _ => None,
    }
}
fn unescape(s: &str) -> String {
    let b = s.as_bytes();
    let mut out = Vec::with_capacity(b.len());
    let mut i = 0;
    while i < b.len() {
        if b[i] == b'%' && i + 2 < b.len() {
            if let (Some(hi), Some(lo)) = (hex_digit(b[i + 1]), hex_digit(b[i + 2])) {
                out.push((hi << 4) | lo);
                i += 3;
                continue;
            }
        }
        out.push(b[i]);
        i += 1;
    }
    String::from_utf8_lossy(&out).into_owned()
}
fn encode_pairs(map: &BTreeMap<Axis, Value>) -> String {
    map.iter()
        .map(|(a, v)| format!("{}={}", escape(a.as_str()), escape(v.as_str())))
        .collect::<Vec<_>>()
        .join(";")
}
fn decode_pairs(s: &str) -> Option<BTreeMap<Axis, Value>> {
    let mut map = BTreeMap::new();
    if s.is_empty() {
        return Some(map);
    }
    for part in s.split(';') {
        let (a, v) = part.split_once('=')?;
        map.insert(Name::new(unescape(a)), Name::new(unescape(v)));
    }
    Some(map)
}
fn decode_profile(v: &str) -> Option<PrecisionProfile> {
    let (pos, cfg) = v.split_once('|')?;
    let position: f64 = pos.trim().parse().ok()?;
    if !position.is_finite() || !(0.0..=1.0).contains(&position) {
        return None;
    }
    Some(PrecisionProfile {
        position,
        config: decode_pairs(cfg)?,
    })
}
fn decode_annotated(v: &str) -> Option<Annotated> {
    let (comb, reason) = v.split_once('|')?;
    Some(Annotated {
        combination: Combination::from_values(decode_pairs(comb)?).ok()?,
        reason: unescape(reason),
    })
}
fn push_annotated(out: &mut String, key: &str, items: &[Annotated]) {
    for a in items {
        out.push_str(
            &format!(
                "{key}={}|{}\n", encode_pairs(a.combination.values()), escape(& a.reason)
            ),
        );
    }
}
pub fn encode_record(r: &CalibrationRecord) -> String {
    let fp = &r.fingerprint;
    let mut out = String::from("# kalibrierung\n");
    out.push_str(&format!("schema_version={}\n", fp.schema_version));
    for (key, value) in [
        ("subengine", &fp.subengine),
        ("gpu_name", &fp.gpu_name),
        ("compute_capability", &fp.compute_capability),
        ("driver_version", &fp.driver_version),
        ("cpu_features", &fp.cpu_features),
        ("model_id", &fp.model_id),
        ("engine_version", &fp.engine_version),
    ] {
        out.push_str(&format!("{key}={}\n", escape(value)));
    }
    out.push_str(&format!("unstable={}\n", r.unstable));
    for p in &r.profiles {
        out.push_str(&format!("profile={}|{}\n", p.position, encode_pairs(& p.config)));
    }
    push_annotated(&mut out, "discarded", &r.discarded);
    push_annotated(&mut out, "pruned", &r.pruned);
    out
}
pub fn decode_record(text: &str) -> Option<CalibrationRecord> {
    let mut record = CalibrationRecord {
        fingerprint: Fingerprint::default(),
        profiles: Vec::new(),
        unstable: false,
        discarded: Vec::new(),
        pruned: Vec::new(),
    };
    let mut known = false;
    for line in text.lines() {
        if line.trim().is_empty() || line.starts_with('#') {
            continue;
        }
        let Some((key, value)) = line.split_once('=') else {
            continue;
        };
        let fp = &mut record.fingerprint;
        match key.trim() {
            "schema_version" => {
                fp.schema_version = unescape(value).trim().parse().unwrap_or(0);
            }
            "subengine" => fp.subengine = unescape(value),
            "gpu_name" => fp.gpu_name = unescape(value),
            "compute_capability" => fp.compute_capability = unescape(value),
            "driver_version" => fp.driver_version = unescape(value),
            "cpu_features" => fp.cpu_features = unescape(value),
            "model_id" => fp.model_id = unescape(value),
            "engine_version" => fp.engine_version = unescape(value),
            "unstable" => record.unstable = unescape(value).trim() == "true",
            "profile" => record.profiles.extend(decode_profile(value)),
            "discarded" => record.discarded.extend(decode_annotated(value)),
            "pruned" => record.pruned.extend(decode_annotated(value)),
            _ => continue,
        }
        known = true;
    }
    if !known {
        return None;
    }
    record
        .profiles
        .sort_by(|a, b| {
            a.position.partial_cmp(&b.position).unwrap_or(std::cmp::Ordering::Equal)
        });
    Some(record)
}
fn load_file(path: &Path) -> Result<CalibrationRecord, PersistError> {
    let bytes = fs::read(path)
        .map_err(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                PersistError::NotFound
            } else {
                PersistError::Unreadable(e.to_string())
            }
        })?;
    let text = String::from_utf8(bytes)
        .map_err(|e| PersistError::Unreadable(e.to_string()))?;
    decode_record(&text)
        .ok_or_else(|| PersistError::Unreadable("no known keys".to_string()))
}
fn write_then_rename(tmp: &Path, target: &Path, content: &str) -> std::io::Result<()> {
    let mut file = fs::File::create(tmp)?;
    file.write_all(content.as_bytes())?;
    file.sync_all()?;
    drop(file);
    fs::rename(tmp, target)
}
fn save_record(dir: &Path, record: &CalibrationRecord) -> Result<(), PersistError> {
    let io = |e: std::io::Error| PersistError::Io(e.to_string());
    let fp = &record.fingerprint;
    let target = record_path(dir, &fp.subengine, &fp.model_id);
    fs::create_dir_all(dir).map_err(io)?;
    let mut tmp_name: OsString = target.as_os_str().to_owned();
    tmp_name
        .push(
            format!(
                ".tmp.{}.{}", std::process::id(), TMP_COUNTER.fetch_add(1,
                Ordering::Relaxed)
            ),
        );
    let tmp = PathBuf::from(tmp_name);
    if let Err(e) = write_then_rename(&tmp, &target, &encode_record(record)) {
        let _ = fs::remove_file(&tmp);
        return Err(io(e));
    }
    let _ = fs::File::open(dir).and_then(|d| d.sync_all());
    Ok(())
}
#[cfg(test)]
#[path = "test/calibration_store.rs"]
mod tests;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::l010::sync_util::lock;
