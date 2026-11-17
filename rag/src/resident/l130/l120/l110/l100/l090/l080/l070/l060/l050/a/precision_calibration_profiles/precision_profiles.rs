//! Maps a precision wish to a per-subengine profile (device + element type) and calibrates
//! profile speed by background benchmarking.
use std::collections::HashMap;
use std::fmt;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, MutexGuard, RwLock};
use std::time::{Duration, Instant};
use super::batch_compute::{BatchCompute, BatchKernel, FloatBatch};
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::a::pareto_profiles::pareto_front;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::a::supported_precisions::scoring_dtypes;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::l010::device_lane_event::{
    DeviceHandle, DeviceKind, Event, EventSignal,
};
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::l010::layout::DType;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::l010::statistics::median;
fn lock<T>(m: &Mutex<T>) -> MutexGuard<'_, T> {
    m.lock().unwrap_or_else(|e| e.into_inner())
}
/// Precision wish in `[0,1]`. 0 = fastest, 1 = most precise combination.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Precision(f32);
impl Precision {
    pub const FASTEST: Precision = Precision(0.0);
    pub const MOST_PRECISE: Precision = Precision(1.0);
    pub fn new(p: f32) -> Result<Precision, PrecisionError> {
        if p.is_finite() && (0.0..=1.0).contains(&p) {
            Ok(Precision(p + 0.0))
        } else {
            Err(PrecisionError::OutOfRange)
        }
    }
    pub fn clamped(p: f32) -> Precision {
        if p.is_nan() {
            return Precision(0.5);
        }
        Precision(p.clamp(0.0, 1.0) + 0.0)
    }
    pub fn get(self) -> f32 {
        self.0
    }
}
impl Default for Precision {
    fn default() -> Self {
        Precision(0.5)
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PrecisionError {
    OutOfRange,
}
impl fmt::Display for PrecisionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PrecisionError::OutOfRange => {
                write!(f, "Precision muss eine Zahl im Bereich [0, 1] sein")
            }
        }
    }
}
impl std::error::Error for PrecisionError {}
/// Identifier of a subengine (key of the profile tables).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SubengineId(pub &'static str);
/// Device + element type. A pure choice, no promise about rounding.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Profile {
    pub device: DeviceHandle,
    pub dtype: DType,
}
impl Profile {
    /// Nominal level, F16 => 1, F32 => 2. A prior used only when no deviation was measured.
    /// Other types are not candidates.
    pub fn precision_level(dtype: DType) -> Option<u8> {
        match dtype {
            DType::F16 => Some(1),
            DType::F32 => Some(2),
            _ => None,
        }
    }
}
/// All candidates for the devices: per device every scoring type, in device then type order.
/// Duplicate devices count once.
pub fn candidates_for(devices: &[DeviceHandle]) -> Vec<Profile> {
    let mut seen: Vec<DeviceHandle> = Vec::new();
    let mut out = Vec::new();
    for &device in devices {
        if seen.contains(&device) {
            continue;
        }
        seen.push(device);
        for &dtype in scoring_dtypes(device.kind()) {
            if Profile::precision_level(dtype).is_some() {
                out.push(Profile { device, dtype });
            }
        }
    }
    out
}
#[derive(Clone, Debug, PartialEq)]
pub struct ProfileEntry {
    pub profile: Profile,
    /// Position of the entry's precision. Derived from the measured deviation when the table
    /// was built from deviations, otherwise the nominal type level.
    pub level: u8,
    /// Median time per measured run; `None` for the unmeasured default table.
    pub cost: Option<Duration>,
    /// Measured deviation from the reference result (lower is more precise).
    pub deviation: Option<f64>,
}
/// Ordered, immutable table: Pareto front, ascending by precision and strictly ascending by
/// time. Precision is the measured deviation when available, otherwise the nominal level.
#[derive(Clone, Debug, PartialEq)]
pub struct ProfileTable {
    entries: Vec<ProfileEntry>,
    calibrated: bool,
}
impl ProfileTable {
    pub fn entries(&self) -> &[ProfileEntry] {
        &self.entries
    }
    pub fn calibrated(&self) -> bool {
        self.calibrated
    }
    pub fn pick(&self, p: Precision) -> Profile {
        let last = self.entries.len().saturating_sub(1);
        let idx = ((p.get() as f64) * (last as f64)).round() as usize;
        self.entries[idx.min(last)].profile
    }
    fn from_candidates(candidates: &[Profile]) -> Option<ProfileTable> {
        let mut items: Vec<(u8, Profile)> = sanitize(candidates)
            .into_iter()
            .filter_map(|p| Profile::precision_level(p.dtype).map(|l| (l, p)))
            .collect();
        if items.is_empty() {
            return None;
        }
        items.sort_by_key(|(level, p)| (*level, p.device.kind() != DeviceKind::Gpu));
        let mut entries: Vec<ProfileEntry> = Vec::new();
        for (level, profile) in items {
            if entries.last().map(|e| e.level) == Some(level) {
                continue;
            }
            entries
                .push(ProfileEntry {
                    profile,
                    level,
                    cost: None,
                    deviation: None,
                });
        }
        Some(ProfileTable {
            entries,
            calibrated: false,
        })
    }
    /// Builds from measurements when every candidate has a valid deviation; otherwise the
    /// deviations are ignored and the nominal type levels order the candidates.
    fn from_measurements(measured: &[Measured]) -> Option<ProfileTable> {
        let usable: Vec<Measured> = measured
            .iter()
            .filter(|(p, _, _)| Profile::precision_level(p.dtype).is_some())
            .copied()
            .collect();
        if usable.is_empty() {
            return None;
        }
        if usable.iter().all(|m| m.2.map_or(false, valid_deviation)) {
            return Some(Self::by_deviation(&usable));
        }
        Self::by_level(&usable)
    }
    fn by_deviation(items: &[Measured]) -> ProfileTable {
        let front = pareto_front(
            items,
            |&(_, cost, dev)| { (cost.as_secs_f64(), dev.unwrap_or(f64::INFINITY)) },
        );
        let entries = front
            .into_iter()
            .enumerate()
            .map(|(i, (profile, cost, deviation))| ProfileEntry {
                profile,
                level: u8::try_from(i + 1).unwrap_or(u8::MAX),
                cost: Some(cost),
                deviation,
            })
            .collect();
        ProfileTable {
            entries,
            calibrated: true,
        }
    }
    fn by_level(items: &[Measured]) -> Option<ProfileTable> {
        let entries: Vec<ProfileEntry> = items
            .iter()
            .filter_map(|&(profile, cost, deviation)| {
                Profile::precision_level(profile.dtype)
                    .map(|level| ProfileEntry {
                        profile,
                        level,
                        cost: Some(cost),
                        deviation,
                    })
            })
            .collect();
        let mut entries = pareto_front(
            &entries,
            |e| {
                let secs = e.cost.map_or(f64::INFINITY, |c| c.as_secs_f64());
                (secs, -f64::from(e.level))
            },
        );
        if entries.is_empty() {
            return None;
        }
        entries.sort_by_key(|e| e.level);
        Some(ProfileTable {
            entries,
            calibrated: true,
        })
    }
}
/// Drops candidates without a precision level and duplicates, keeping input order.
fn sanitize(candidates: &[Profile]) -> Vec<Profile> {
    let mut out: Vec<Profile> = Vec::new();
    for &c in candidates {
        if Profile::precision_level(c.dtype).is_some() && !out.contains(&c) {
            out.push(c);
        }
    }
    out
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ProfileError {
    UnknownSubengine(SubengineId),
    NoCandidates,
}
impl fmt::Display for ProfileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProfileError::UnknownSubengine(id) => {
                write!(f, "unknown subengine: {}", id.0)
            }
            ProfileError::NoCandidates => write!(f, "no valid profile candidates"),
        }
    }
}
impl std::error::Error for ProfileError {}
/// Shared registry of profile tables.
pub struct ProfileRegistry {
    tables: RwLock<HashMap<SubengineId, Arc<ProfileTable>>>,
}
impl ProfileRegistry {
    pub fn new() -> ProfileRegistry {
        ProfileRegistry {
            tables: RwLock::new(HashMap::new()),
        }
    }
    /// Installs the unmeasured default table, replacing any existing table.
    pub fn register(
        &self,
        id: SubengineId,
        candidates: &[Profile],
    ) -> Result<(), ProfileError> {
        let table = ProfileTable::from_candidates(candidates)
            .ok_or(ProfileError::NoCandidates)?;
        self.write().insert(id, Arc::new(table));
        Ok(())
    }
    pub fn select(
        &self,
        id: SubengineId,
        p: Precision,
    ) -> Result<Profile, ProfileError> {
        self.table(id).map(|t| t.pick(p)).ok_or(ProfileError::UnknownSubengine(id))
    }
    pub fn table(&self, id: SubengineId) -> Option<Arc<ProfileTable>> {
        self.tables.read().unwrap_or_else(|e| e.into_inner()).get(&id).cloned()
    }
    fn write(
        &self,
    ) -> std::sync::RwLockWriteGuard<'_, HashMap<SubengineId, Arc<ProfileTable>>> {
        self.tables.write().unwrap_or_else(|e| e.into_inner())
    }
    fn install(&self, id: SubengineId, table: ProfileTable) {
        self.write().insert(id, Arc::new(table));
    }
    fn install_default_if_absent(&self, id: SubengineId, candidates: &[Profile]) {
        if let Some(table) = ProfileTable::from_candidates(candidates) {
            self.write().entry(id).or_insert_with(|| Arc::new(table));
        }
    }
}
impl Default for ProfileRegistry {
    fn default() -> Self {
        ProfileRegistry::new()
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BenchError {
    Unsupported,
    Failed(String),
}
impl fmt::Display for BenchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BenchError::Unsupported => {
                write!(f, "profile not supported by the benchmark")
            }
            BenchError::Failed(m) => write!(f, "benchmark failed: {m}"),
        }
    }
}
impl std::error::Error for BenchError {}
/// Benchmark workload provided by the owner of a subengine.
pub trait BenchTarget: Send + Sync {
    /// Runs one representative operation with the profile. `Ok(Some(event))` means asynchronous
    /// work the calibrator waits for; `Ok(None)` means the work is complete on return.
    fn run(&self, profile: &Profile) -> Result<Option<Event>, BenchError>;
    /// Deviation of the profile's result from a trusted reference, lower is more precise.
    /// `Ok(None)` means unknown. Called once per candidate, outside the timed runs.
    fn deviation(&self, _profile: &Profile) -> Result<Option<f64>, BenchError> {
        Ok(None)
    }
}
pub struct CalibrationJob {
    pub id: SubengineId,
    pub candidates: Vec<Profile>,
    pub target: Arc<dyn BenchTarget>,
}
#[derive(Clone, Debug)]
pub struct CalibrationConfig {
    pub warmup_runs: u32,
    pub measured_runs: u32,
    pub budget_per_candidate: Duration,
}
impl Default for CalibrationConfig {
    fn default() -> Self {
        CalibrationConfig {
            warmup_runs: 1,
            measured_runs: 5,
            budget_per_candidate: Duration::from_millis(200),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CalibrationStatus {
    Running,
    Finished,
    Cancelled,
    Failed(String),
}
struct CalibrationShared {
    status: Mutex<CalibrationStatus>,
    cancel: AtomicBool,
    done: Event,
    signal: EventSignal,
}
impl CalibrationShared {
    fn new() -> CalibrationShared {
        let (done, signal) = Event::cpu();
        CalibrationShared {
            status: Mutex::new(CalibrationStatus::Running),
            cancel: AtomicBool::new(false),
            done,
            signal,
        }
    }
    fn is_cancelled(&self) -> bool {
        self.cancel.load(Ordering::Acquire)
    }
    fn finish(&self, status: CalibrationStatus) {
        {
            let mut current = lock(&self.status);
            if *current == CalibrationStatus::Running {
                *current = status;
            }
        }
        self.signal.signal();
    }
}
/// Handle of a background calibration. Dropping it neither cancels nor waits.
#[derive(Clone)]
pub struct CalibrationHandle {
    shared: Arc<CalibrationShared>,
}
impl CalibrationHandle {
    pub fn status(&self) -> CalibrationStatus {
        lock(&self.shared.status).clone()
    }
    pub fn done(&self) -> Event {
        self.shared.done.clone()
    }
    pub fn cancel(&self) {
        self.shared.cancel.store(true, Ordering::Release);
    }
}
pub struct Calibrator;
impl Calibrator {
    /// Starts a background thread and returns immediately. Never fails.
    pub fn start(
        registry: Arc<ProfileRegistry>,
        jobs: Vec<CalibrationJob>,
        cfg: CalibrationConfig,
    ) -> CalibrationHandle {
        let shared = Arc::new(CalibrationShared::new());
        let handle = CalibrationHandle {
            shared: shared.clone(),
        };
        if jobs.is_empty() {
            shared.finish(CalibrationStatus::Finished);
            return handle;
        }
        let worker = shared.clone();
        let spawned = std::thread::Builder::new()
            .name("precision-calibration".into())
            .spawn(move || {
                let outcome = catch_unwind(
                    AssertUnwindSafe(|| { run_jobs(&registry, &jobs, &cfg, &worker) }),
                );
                let status = match outcome {
                    Ok(status) => status,
                    Err(_) => {
                        CalibrationStatus::Failed("calibration thread panicked".into())
                    }
                };
                worker.finish(status);
            });
        if let Err(e) = spawned {
            shared
                .finish(
                    CalibrationStatus::Failed(
                        format!("cannot start calibration thread: {e}"),
                    ),
                );
        }
        handle
    }
}
enum Measurement {
    Cost(Duration, Option<f64>),
    Excluded,
    Cancelled,
}
type Measured = (Profile, Duration, Option<f64>);
fn valid_deviation(d: f64) -> bool {
    d.is_finite() && d >= 0.0
}
fn run_jobs(
    registry: &ProfileRegistry,
    jobs: &[CalibrationJob],
    cfg: &CalibrationConfig,
    shared: &CalibrationShared,
) -> CalibrationStatus {
    for job in jobs {
        if shared.is_cancelled() {
            return CalibrationStatus::Cancelled;
        }
        let candidates = sanitize(&job.candidates);
        let mut measured: Vec<Measured> = Vec::new();
        for profile in &candidates {
            if shared.is_cancelled() {
                return CalibrationStatus::Cancelled;
            }
            match measure(job.target.as_ref(), profile, cfg, shared) {
                Measurement::Cost(cost, deviation) => {
                    measured.push((*profile, cost, deviation))
                }
                Measurement::Excluded => {}
                Measurement::Cancelled => return CalibrationStatus::Cancelled,
            }
        }
        if shared.is_cancelled() {
            return CalibrationStatus::Cancelled;
        }
        match ProfileTable::from_measurements(&measured) {
            Some(table) => registry.install(job.id, table),
            None => registry.install_default_if_absent(job.id, &candidates),
        }
    }
    CalibrationStatus::Finished
}
fn run_once(target: &dyn BenchTarget, profile: &Profile) -> Option<Duration> {
    let started = Instant::now();
    let outcome = catch_unwind(
        AssertUnwindSafe(|| match target.run(profile) {
            Ok(Some(event)) => event.wait().is_ok(),
            Ok(None) => true,
            Err(_) => false,
        }),
    );
    match outcome {
        Ok(true) => Some(started.elapsed()),
        _ => None,
    }
}
fn measure_deviation(target: &dyn BenchTarget, profile: &Profile) -> Option<f64> {
    catch_unwind(AssertUnwindSafe(|| target.deviation(profile)))
        .ok()?
        .ok()?
        .filter(|d| valid_deviation(*d))
}
fn measure(
    target: &dyn BenchTarget,
    profile: &Profile,
    cfg: &CalibrationConfig,
    shared: &CalibrationShared,
) -> Measurement {
    for _ in 0..cfg.warmup_runs {
        if shared.is_cancelled() {
            return Measurement::Cancelled;
        }
        if run_once(target, profile).is_none() {
            return Measurement::Excluded;
        }
    }
    let runs = cfg.measured_runs.max(1);
    let phase_start = Instant::now();
    let mut times: Vec<Duration> = Vec::new();
    for i in 0..runs {
        if shared.is_cancelled() {
            return Measurement::Cancelled;
        }
        if i > 0 && phase_start.elapsed() >= cfg.budget_per_candidate {
            break;
        }
        match run_once(target, profile) {
            Some(t) => times.push(t),
            None => return Measurement::Excluded,
        }
    }
    Measurement::Cost(median_duration(&times), measure_deviation(target, profile))
}
fn median_duration(times: &[Duration]) -> Duration {
    let secs: Vec<f64> = times.iter().map(Duration::as_secs_f64).collect();
    median(&secs)
        .and_then(|m| Duration::try_from_secs_f64(m).ok())
        .or_else(|| times.first().copied())
        .unwrap_or(Duration::ZERO)
}
const BENCH_DIM: usize = 64;
/// Ready-made benchmark workload: `MatMul` on fixed `f32` batches through `BatchCompute`.
/// The deviation is measured against an `f64` product of the same operands.
pub struct BatchBench {
    cpu: Arc<dyn BatchCompute>,
    gpu: Option<(u32, Arc<dyn BatchCompute>)>,
    operands: Option<(FloatBatch, FloatBatch)>,
    reference: Option<Vec<f64>>,
}
impl BatchBench {
    pub fn new(
        cpu: Arc<dyn BatchCompute>,
        gpu: Option<(u32, Arc<dyn BatchCompute>)>,
    ) -> BatchBench {
        let fill = |seed: usize| -> Vec<f32> {
            (0..BENCH_DIM * BENCH_DIM)
                .map(|i| (((i * 31 + seed * 17) % 101) as f32) / 101.0 - 0.5)
                .collect()
        };
        let a = FloatBatch::new(BENCH_DIM, BENCH_DIM, fill(1));
        let b = FloatBatch::new(BENCH_DIM, BENCH_DIM, fill(2));
        let (operands, reference) = match (a, b) {
            (Ok(a), Ok(b)) => {
                let reference = reference_matmul(&a, &b);
                (Some((a, b)), Some(reference))
            }
            _ => (None, None),
        };
        BatchBench {
            cpu,
            gpu,
            operands,
            reference,
        }
    }
    fn execute(&self, profile: &Profile) -> Result<FloatBatch, BenchError> {
        if profile.dtype != DType::F32 {
            return Err(BenchError::Unsupported);
        }
        let backend: &Arc<dyn BatchCompute> = match profile.device {
            DeviceHandle::Cpu => &self.cpu,
            DeviceHandle::Gpu(n) => {
                match &self.gpu {
                    Some((ordinal, backend)) if *ordinal == n => backend,
                    _ => return Err(BenchError::Unsupported),
                }
            }
        };
        let (a, b) = self
            .operands
            .as_ref()
            .ok_or_else(|| BenchError::Failed("benchmark operands unavailable".into()))?;
        backend
            .run(BatchKernel::MatMul, &[a, b])
            .map_err(|e| BenchError::Failed(e.to_string()))
    }
}
fn reference_matmul(a: &FloatBatch, b: &FloatBatch) -> Vec<f64> {
    let (m, k, n) = (a.rows(), a.cols(), b.cols());
    let (a, b) = (a.as_slice(), b.as_slice());
    let mut out = vec![0.0f64; m * n];
    for i in 0..m {
        for p in 0..k {
            let av = f64::from(a[i * k + p]);
            for j in 0..n {
                out[i * n + j] += av * f64::from(b[p * n + j]);
            }
        }
    }
    out
}
impl BenchTarget for BatchBench {
    fn run(&self, profile: &Profile) -> Result<Option<Event>, BenchError> {
        self.execute(profile)?;
        Ok(None)
    }
    fn deviation(&self, profile: &Profile) -> Result<Option<f64>, BenchError> {
        let result = self.execute(profile)?;
        let reference = self
            .reference
            .as_ref()
            .ok_or_else(|| BenchError::Failed(
                "benchmark reference unavailable".into(),
            ))?;
        let got = result.as_slice();
        if got.len() != reference.len() {
            return Err(BenchError::Failed("result shape differs from reference".into()));
        }
        let scale = reference.iter().fold(0.0f64, |m, r| m.max(r.abs()));
        let worst = got
            .iter()
            .zip(reference)
            .fold(
                0.0f64,
                |m, (g, r)| {
                    let e = (f64::from(*g) - r).abs();
                    if e.is_nan() { f64::INFINITY } else { m.max(e) }
                },
            );
        Ok(Some(if scale > 0.0 { worst / scale } else { worst }))
    }
}
#[cfg(test)]
#[path = "test/precision_profiles.rs"]
mod tests;
