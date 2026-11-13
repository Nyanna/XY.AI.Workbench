# Specification: Kalibrierungs-Persistenz (build order 116)

## 1. Task, restrictions and boundaries

Rust module `calibration_store`. It does the following:
- It stores one calibration result per (subengine, model) as a readable text file. Load is tolerant and save is atomic.
- It builds the runtime fingerprint.
- At startup it resolves the state *fehlend* (missing), *kalibriert* (calibrated) or *veraltet* (stale). In the stale state it starts a recalibration on the low-priority background pool.
- It turns a `CalibrationReport` into a stored result.

Restrictions:
- Allowed crates: `log` and the dependency modules. No serde and no other crates.
- It does not measure, compute statistics, or derive the front or profiles itself. It does not enumerate capabilities.
- It adds no profile names, no extra policies, and no file locking. It uses no async runtime.
- It never panics.

## 2. Context

- **Kalibrierungslauf** (`calibration_run`): `run_calibration(...) -> Result<CalibrationReport, CalibrationError>`. It is sequential and blocking. This module calls it inside a background job.
- **Pareto-Front und Profilableitung**: `derive_profiles(&[Measurement], threshold)` turns measurements into `PrecisionProfile { position, config }`. This module stores these profiles.
- **Hintergrundausführung und Statistik**: `BackgroundPool::submit` returns a `JobFuture`. The caller injects the pool, and this module does not create it. The caller also supplies the `Statistics` implementation.
- **Capability-Erkennung**: `detect_cpu_features() -> CpuFeatures` and the capability set (`CapabilitySet`).
- **Modellbeschreibung**: `ModelDescriptor`, passed through to the calibration run.
- Consumers (facade and startup) call `resolve` once per (subengine, model) and use the returned result. The persisted profiles are the only thing they read.

## 3. Interface

```rust
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::collections::HashSet;
use calibration_run::{CalibrationBackend, CalibrationConfig, CalibrationError, Combination, Statistics, Target};
use capability::CapabilitySet;
use model_descriptor::ModelDescriptor;
use pareto_profiles::PrecisionProfile;            // Pareto-Front und Profilableitung
use background_exec::{BackgroundPool, JobFuture};  // Hintergrundausführung und Statistik

pub const SCHEMA_VERSION: u32 = 1;

/// Supplied by the GPU binding (driver API). Called only for the "gpu" subengine. None = unknown.
pub trait DriverInfo: Send + Sync {
    fn gpu_name(&self) -> Option<String>;
    fn compute_capability(&self) -> Option<(u32, u32)>;   // (major, minor)
    fn driver_version(&self) -> Option<String>;
}

/// All fields are text and are compared by plain equality. Defaults on read: schema_version 0, strings "".
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Fingerprint {
    pub schema_version: u32,
    pub subengine: String,          // "cpu" | "gpu"
    pub gpu_name: String,           // "" for cpu
    pub compute_capability: String, // "7.5"; "" for cpu
    pub driver_version: String,     // "" for cpu
    pub cpu_features: String,       // set flags of CpuFeatures in the order avx2,f16c,fma,vnni, comma-joined
    pub model_id: String,
    pub engine_version: String,
}
impl Fingerprint {
    /// Runtime fingerprint. schema_version = SCHEMA_VERSION. cpu_features come from
    /// `detect_cpu_features()` for both subengines. For subengine "gpu" the GPU fields come from
    /// `driver` (None or an unknown value gives "unknown"). For "cpu" the three GPU fields are "".
    pub fn detect(subengine: &str, model_id: &str, engine_version: &str, driver: Option<&dyn DriverInfo>) -> Fingerprint;
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Annotated { pub combination: Combination, pub reason: String }

#[derive(Clone, Debug, PartialEq)]
pub struct CalibrationRecord {
    pub fingerprint: Fingerprint,
    pub profiles: Vec<PrecisionProfile>,  // ascending position
    pub unstable: bool,
    pub discarded: Vec<Annotated>,
    pub pruned: Vec<Annotated>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum PersistError {
    NotFound,
    Unreadable(String),
    Io(String),
    Pool(String),                  // pool shut down or thread error
    Calibration(CalibrationError),
    NoProfiles,                    // the run produced no profile; nothing is saved
}
impl std::fmt::Display for PersistError {}
impl std::error::Error for PersistError {}

/// Everything a background recalibration needs. 'static + Send.
#[derive(Clone)]
pub struct CalibrationRequest {
    pub fingerprint: Fingerprint,           // runtime fingerprint; target = (fingerprint.subengine, fingerprint.model_id)
    pub capabilities: CapabilitySet,
    pub model: ModelDescriptor,
    pub backend: Arc<dyn CalibrationBackend>,
    pub stats: Arc<dyn Statistics>,
    pub config: CalibrationConfig,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MissingPolicy { MostPreciseValid, ExplicitCalibrate }

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MissingResolution { Fallback(Combination), CalibrateCommand }

pub enum Resolution {
    Calibrated { record: CalibrationRecord },
    Stale { record: CalibrationRecord,
            /// None if a recalibration for the same key is already running or submit failed.
            recalibration: Option<JobFuture<Result<CalibrationRecord, PersistError>>> },
    Missing(MissingResolution),
}

/// Send + Sync. Shareable via Arc.
pub struct CalibrationStore { /* dir, Arc<BackgroundPool>, Arc<Mutex<HashSet<(String,String)>>> */ }
impl CalibrationStore {
    pub fn new(dir: PathBuf, pool: Arc<BackgroundPool>) -> CalibrationStore;   // creates nothing
    pub fn dir(&self) -> &Path;
    /// `<dir>/<subengine>__<enc(model_id)>.calib.txt`; enc percent-encodes (%XX uppercase) every byte not in [A-Za-z0-9._-].
    pub fn path_for(&self, subengine: &str, model_id: &str) -> PathBuf;
    pub fn load(&self, subengine: &str, model_id: &str) -> Result<CalibrationRecord, PersistError>;
    /// Atomic. Creates missing directories (create_dir_all).
    pub fn save(&self, record: &CalibrationRecord) -> Result<(), PersistError>;
    /// Never fails and never panics.
    pub fn resolve(&self, req: &CalibrationRequest, policy: MissingPolicy) -> Resolution;
    /// Explicit trigger: background calibration + save. Err(Pool) if the pool is shut down.
    /// If a job for the same key is already running, Ok(None).
    pub fn request_calibration(&self, req: CalibrationRequest)
        -> Result<Option<JobFuture<Result<CalibrationRecord, PersistError>>>, PersistError>;
}

/// Blocking, synchronous: run_calibration + record construction (used by the background job). Does not save.
pub fn calibrate_record(req: &CalibrationRequest) -> Result<CalibrationRecord, PersistError>;
pub fn record_from_report(fp: Fingerprint, report: &calibration_run::CalibrationReport) -> Result<CalibrationRecord, PersistError>;
pub fn encode_record(r: &CalibrationRecord) -> String;
pub fn decode_record(text: &str) -> Option<CalibrationRecord>;   // None = unreadable
```

## 4. Behavior and invariants

### File format (`encode_record` / `decode_record`)
UTF-8, line-based `key=value`, split at the first `=`, lines separated by `\n`. Order when writing:
1. `# kalibrierung` (comment).
2. `schema_version`, `subengine`, `gpu_name`, `compute_capability`, `driver_version`, `cpu_features`, `model_id`, `engine_version`.
3. `unstable=true|false`.
4. One `profile=<position>|<axis>=<value>;<axis>=<value>` line per profile. The position is written with Rust `{}` formatting so it round-trips. The config iterates in `BTreeMap` order.
5. `discarded=<combination>|<reason>` lines.
6. `pruned=<combination>|<reason>` lines.

A combination is `a=b;c=d`, or the empty string for an empty combination.

- **Escaping.** In all text values, `%`, `|`, `;`, `=`, `\n` and `\r` are written as `%XX` (uppercase hex). Decode reverses `%XX`. An invalid `%` sequence stays literal.
- **Lenient read.**
  - Ignore blank lines, `#` lines, lines without `=`, and unknown keys. For a duplicate scalar key the last one wins.
  - Missing scalar keys get the defaults: `schema_version` 0, strings `""`, `unstable` false, lists empty.
  - A malformed `profile` line is skipped. It is malformed if it has no `|`, if the position is unparsable or non-finite or outside [0,1], or if a pair has no `=`. Malformed `discarded` and `pruned` lines are skipped the same way.
  - Profiles are sorted ascending by position (stable).
- `decode_record` returns `None` if no line has a known key. `encode_record(decode_record(x))` round-trips every field that `encode_record` writes.

### Load and save
- `load`:
  - A missing file gives `NotFound`.
  - Other I/O errors, invalid UTF-8, or a `decode_record` result of `None` give `Unreadable(msg)`.
  - It never creates anything.
- `save` is atomic:
  1. `create_dir_all(dir)`.
  2. Write the content to the temp file `<final>.tmp.<pid>.<n>`. `<n>` is a process-wide atomic counter.
  3. Call `sync_all` on the temp file.
  4. `rename` it onto the final path.
  5. Best-effort `fsync` of the directory.
  - On any failure the temp file is removed (best effort) and `Io(msg)` is returned. The old file stays intact.
  - A reader sees either the complete old file or the complete new file.
  - The target is `path_for(record.fingerprint.subengine, record.fingerprint.model_id)`.

### Fingerprint
- Two fingerprints are equal only if all eight fields are equal. The fingerprint contains no other data, so deleting the file forces recalibration.
- The runtime fingerprint is computed at call time. GPU data comes only through `DriverInfo`.

### Resolve (never fails)
`resolve` loads `path_for(req.fingerprint.subengine, req.fingerprint.model_id)`. The outcome depends on what it finds:

| Load result | Resolution |
|---|---|
| `NotFound` or `Unreadable` (unreadable is logged at `warn!`) | `Missing` |
| Loaded, and `record.fingerprint == req.fingerprint` | `Calibrated { record }`. Nothing else happens. |
| Loaded, and fingerprints differ (this includes defaulted fields) | `Stale` |

- **Calibrated:** the record is used and no job is started.
- **Stale:**
  - It returns the old record and calls `request_calibration(req.clone())` for the background recalibration.
  - If that returns `Ok(f)`, `recalibration` is `f`. If it returns `Err`, it is `None` and the error is logged at `warn!`.
  - The old file stays in place until the new one replaces it.
- **Missing, `ExplicitCalibrate`:** returns `Missing(CalibrateCommand)`. Nothing is started and the caller is expected to run an explicit calibrate command.
- **Missing, `MostPreciseValid`:**
  - It logs a `warn!` and returns `Missing(Fallback(c))`. `c` is chosen per axis of `req.capabilities`.
  - A precision axis is one of `operands`, `accumulator`, `scoring`. Its value is the one with the highest precision rank: `fp32`=4, `fp16_storage_fp32_compute`=3, `fp16`=2, `int8`, `int8_vnni` and `int8_dp4a`=1, unknown=0 (compare against the `values::*` names). Ties go to the first value in this order: values that parse fully as `u64` first, numerically ascending, then the rest in byte order.
  - Every other axis gets the first value in that same order.
  - Empty capabilities give an empty combination.
  - The result is valid by construction, because it takes one value per axis from the set.
- Nothing is written to disk and no job is started in the `Missing` cases.

### Recalibration (background)
- `request_calibration` first checks the in-flight set. The key is `(subengine, model_id)`, per store instance.
  - If the key is present, it returns `Ok(None)`.
  - Otherwise it inserts the key and submits a job to the injected pool. The job removes the key via a drop guard, also on panic or error.
  - If `submit` fails, it removes the key and returns `Err(Pool(msg))`.
- **Job body:**
  1. `calibrate_record(&req)`.
  2. If the previous file is readable, remember its fingerprint.
  3. `save`.
  4. Log at `info!` the replacement, with subengine, model_id, profile count, the new fingerprint, and the old fingerprint if known.
  5. Return the record.
- If the run or the save fails, the job returns `Err` and logs at `warn!`. The old file stays unchanged.
- The call returns immediately and never blocks on the run. The job runs on the pool's low-priority worker, so a panic yields `JobError::Panicked` on the `JobFuture` only.
- `calibrate_record` calls `run_calibration(Target { subengine, model_id } from req.fingerprint, &req.capabilities, &req.model, &*req.backend, &*req.stats, &req.config)` and maps its error to `Calibration(e)`. It then calls `record_from_report`.

### Record from report (`record_from_report`)
- **Measurements.** One `Measurement` per `report.summary.front` point. Its config is `combination.0.clone()`, `time` is `time.as_secs_f64()`, and `deviation` is carried over.
- **Profiles.** `derive_profiles(&measurements, DEFAULT_MERGE_THRESHOLD)`. If it returns `Err` (not expected) or an empty vec, the result is `Err(NoProfiles)`.
- **`unstable`.** True iff any `Measured { stable: false }` entry exists with stage `Verification` or `Full`.
- **`discarded`.** Every `Outcome::Discarded(r)` entry of any stage, in report order. The reason text is:
  - `Failed(m)` → `failed: m`
  - `NonFinite` → `non-finite deviation`
  - `ExceedsMax` → `exceeds max deviation`
  - `Other(m)` → `m`
- **`pruned`.** Every `Outcome::Dominated` entry of any stage, in report order, with reason `dominated`.
- **`Skipped` entries.** They are not stored.
- **Fingerprint.** The record carries `fp` unchanged.

### Concurrency
- `CalibrationStore` is `Send + Sync`. `load`, `save` and `resolve` can run concurrently.
- Concurrent saves to the same target never corrupt the file: each uses its own temp file and the last rename wins.
- At most one background calibration per key runs per store instance. There is no cross-process lock, and processes are protected only by the temp file plus rename.
- The pool is injected. After the pool is shut down, `request_calibration` returns `Err(Pool)`.

### Errors
All failures are `PersistError`. Only `resolve` and `decode_record` are infallible by contract.

## 5. Relied-on elements

- **Kalibrierungslauf (verbatim):**
  - `pub fn run_calibration(target: Target, capabilities: &CapabilitySet, model: &ModelDescriptor, backend: &dyn CalibrationBackend, stats: &dyn Statistics, config: &CalibrationConfig) -> Result<CalibrationReport, CalibrationError>`
  - `Target { subengine: String, model_id: String }` and `Combination(pub BTreeMap<Axis, Value>)` (`Clone, Eq, Ord, Hash`).
  - `CalibrationReport { entries: Vec<ReportEntry>, summary: Summary { front: Vec<FrontPoint { combination, time: Duration, deviation: f64 }>, .. }, budget, .. }`.
  - `ReportEntry { stage: Stage {Axis, Verification, Full}, combination, outcome }`.
  - `Outcome::{Dominated, Discarded(DiscardReason), Skipped(SkipReason), Measured { time, deviation, stable, samples, uncertainty }}`.
  - `DiscardReason::{Failed(String), NonFinite, ExceedsMax, Other(String)}`.
  - `CalibrationBackend: Send + Sync` and `Statistics: Send + Sync` (injected). `CalibrationConfig: Clone`, with a `Default`. `CalibrationError` is `Clone + PartialEq`.
- **Pareto-Front und Profilableitung (verbatim):**
  - `Measurement { config: BTreeMap<Axis, Value>, time: f64, deviation: f64 }`.
  - `PrecisionProfile { position: f64 /* [0,1] */, config: BTreeMap<Axis, Value> }` (`Clone, Debug, PartialEq`).
  - `derive_profiles(&[Measurement], f64) -> Result<Vec<PrecisionProfile>, ProfileError>`. The result is ordered by ascending position, and it is empty if no usable measurement exists.
  - `DEFAULT_MERGE_THRESHOLD = 0.1`.
- **Hintergrundausführung (verbatim):**
  - `BackgroundPool: Send + Sync` with `submit<T, F>(&self, job: F) -> Result<JobFuture<T>, PoolError>`. `F: FnOnce() -> T + Send + 'static`, `T: Send + 'static`.
  - `JobFuture<T>` has `wait`, `try_get` and `is_done`. A job panic yields `Err(JobError::Panicked)`. After shutdown, `submit` returns `Err(PoolError::ShutDown)`.
  - Workers run at nice 19.
- **Capability-Erkennung:** `detect_cpu_features() -> CpuFeatures { avx2, f16c, fma, vnni: bool }`.
- **Capability-Modell:**
  - `Name` (`Axis = Value = Name`) has `as_str()` and `new(&str)`, and orders by byte-wise lexicographic string order.
  - `CapabilitySet::{axes(), values(&Axis) -> Option<&BTreeSet<Value>>, is_empty()}`.
  - `values::{fp32, fp16, fp16_storage_fp32_compute, int8, int8_vnni, int8_dp4a}` and `axes::{operands, accumulator, scoring}`.
- **Modellbeschreibung:** `ModelDescriptor` (`Clone`, `model_id: String`, `validate()`).