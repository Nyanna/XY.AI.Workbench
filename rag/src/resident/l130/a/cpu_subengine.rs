//! CPU realisation of a sub-engine: a `SubengineCore` on the host with lazily loaded RAM resources,
//! its own SIMD score kernels, a registry of attached file-backed indexes and low-priority calibration.
use std::borrow::Cow;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::marker::PhantomData;
use std::path::PathBuf;
use std::sync::{Arc, Mutex, MutexGuard, PoisonError};
use std::time::{Duration, Instant};
use half::f16;
use log::warn;
use crate::resident::l130::l120::l110;
use crate::resident::l130::l120::manager::LazyIndex;
use l110::l100::a::plan_contract::QueueKind;
use l110::a::subengine_core::{
    ComputePath, LaneFactory, PathPolicy, QueueRegistrations, SubengineConfig,
    SubengineCore, SubengineError,
};
use l110::l100;
use l100::a::planner::Subengine;
use l100::l090;
use l090::a::entry_store::{EntryStore, PayloadChecksums};
use l090::a::lane_batching::{
    BatchMeta, BatchPayload, BatchRunner, ExecBatch, LaneResource, LaneResourceConfig,
    LaneSetError, Launch,
};
use l090::a::model_registry::{LoadedEncoder, ModelRegistry};
use l090::a::persistent_save::entry_payload::compute_checksum_parts;
use l090::l080;
use l080::a::index_format::IndexHeader;
use l080::a::kernel_scoring::{Metric, KERNEL_SCORE_DOT};
use l080::a::parquet_decoder::{ChecksumFn, ParquetDecoder};
use l080::l070;
use l070::a::kernel_register::{KernelRegister, Variant};
use l070::a::text_normalization::{
    NormalizationParams, NormalizerLoader, TextNormalizer, TextPreprocessor,
};
use l070::a::tokenizer_loader::{LoadedTokenizer, TokenizerLoader};
use l070::l060;
use l060::a::resource_lifecycle::{
    InitRequest, Lease, LifecycleConfig, LifecycleError, MemoryClass, ResourceKey,
    ResourceRegistry, ResourceState,
};
use l060::l050;
use l050::a::encoder_interface::{oneshot, EncoderError};
use l050::l040;
use l040::a::resource_requirements::{
    HasResourceRequirements, IndexId, ModelId, ResourceKind, ResourceRequirements,
};
use l040::index_residency::{IndexResidency, IndexSlot};
use l040::l030;
use l030::a::calibration_run::{
    run_calibration, CalibrationBackend, CalibrationConfig, CalibrationError,
    CalibrationReport, Target,
};
use l030::a::index_open::{LazyIndexFile, OpenOptions};
use l030::l020;
use l020::a::index_contract::{EntryId, Index};
use l020::a::memory_access::TensorRef;
use l020::a::model_descriptor::{DeviceClass, ModelDescriptor};
use l020::a::supported_precisions::precision_capabilities;
use l020::l010;
use l010::background_pool::{BackgroundPool, JobFuture, PoolError};
use l010::capability_model::CapabilitySet;
use l010::device_lane_event::{DeviceHandle, DeviceKind, Lane, LaneConfig, LaneKind};
use l010::layout::DType;
use l010::page_advice::{AdviceSlot, AdviceTarget};
const SCORE_BLOCK_ROWS: usize = 1024;
const LOADING_POLL: Duration = Duration::from_millis(1);
const ENCODER_KERNEL: &str = "encoder";
fn lock<T>(m: &Mutex<T>) -> MutexGuard<'_, T> {
    m.lock().unwrap_or_else(PoisonError::into_inner)
}
#[derive(Debug)]
pub enum CpuError {
    InvalidConfig(String),
    Subengine(SubengineError),
    UnknownModel(ModelId),
    UnknownIndex(IndexId),
    AlreadyAttached(IndexId),
    AlreadyRegistered(ModelId),
    Load(String),
    Backoff { until: Instant },
    Unavailable(ResourceKey),
    Encoder(EncoderError),
    Normalize(String),
    IndexOpen(String),
    NoKernel { name: String, dtype: DType },
    QueryDimMismatch { query: usize, row_bytes: usize },
    Pool(PoolError),
    Lane(LaneSetError),
}
impl fmt::Display for CpuError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CpuError::InvalidConfig(m) => {
                write!(f, "ungültige CPU-Subengine-Konfiguration: {m}")
            }
            CpuError::Subengine(e) => write!(f, "{e}"),
            CpuError::UnknownModel(m) => write!(f, "Modell {m} nicht registriert"),
            CpuError::UnknownIndex(i) => write!(f, "Index {i} nicht angebunden"),
            CpuError::AlreadyAttached(i) => write!(f, "Index {i} bereits angebunden"),
            CpuError::AlreadyRegistered(m) => write!(f, "Modell {m} bereits registriert"),
            CpuError::Load(m) => write!(f, "Laden fehlgeschlagen: {m}"),
            CpuError::Backoff { .. } => write!(f, "Ressource im Backoff"),
            CpuError::Unavailable(k) => write!(f, "Ressource nicht verfügbar: {k}"),
            CpuError::Encoder(e) => write!(f, "{e}"),
            CpuError::Normalize(m) => write!(f, "Normalisierung fehlgeschlagen: {m}"),
            CpuError::IndexOpen(m) => write!(f, "Index öffnen fehlgeschlagen: {m}"),
            CpuError::NoKernel { name, dtype } => {
                write!(f, "kein CPU-Kernel {name} für {dtype:?}")
            }
            CpuError::QueryDimMismatch { query, row_bytes } => {
                write!(
                    f, "Query-Dimension {query} passt nicht zu Vektorgröße {row_bytes}"
                )
            }
            CpuError::Pool(e) => write!(f, "{e}"),
            CpuError::Lane(e) => write!(f, "{e}"),
        }
    }
}
impl std::error::Error for CpuError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            CpuError::Subengine(e) => Some(e),
            CpuError::Encoder(e) => Some(e),
            CpuError::Pool(e) => Some(e),
            CpuError::Lane(e) => Some(e),
            _ => None,
        }
    }
}
/// Sockets -> cores (resources) -> lanes. A lane is one thread of the processor.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CpuTopology {
    sockets: Vec<Vec<usize>>,
}
impl CpuTopology {
    pub fn new(lanes_per_core_per_socket: Vec<Vec<usize>>) -> Result<Self, CpuError> {
        if lanes_per_core_per_socket.is_empty() {
            return Err(CpuError::InvalidConfig("topology has no socket".to_string()));
        }
        for (s, cores) in lanes_per_core_per_socket.iter().enumerate() {
            if cores.is_empty() {
                return Err(CpuError::InvalidConfig(format!("socket {s} has no core")));
            }
            if let Some(c) = cores.iter().position(|&lanes| lanes == 0) {
                return Err(
                    CpuError::InvalidConfig(
                        format!("core {c} of socket {s} has no lane"),
                    ),
                );
            }
        }
        Ok(Self {
            sockets: lanes_per_core_per_socket,
        })
    }
    pub fn single_socket(cores: usize, lanes_per_core: usize) -> Result<Self, CpuError> {
        Self::new(vec![vec![lanes_per_core; cores]])
    }
    pub fn sockets(&self) -> usize {
        self.sockets.len()
    }
    pub fn cores(&self) -> usize {
        self.sockets.iter().map(Vec::len).sum()
    }
    pub fn lane_count(&self) -> usize {
        self.sockets.iter().flatten().sum()
    }
}
#[derive(Clone, Debug)]
pub struct CpuSubengineConfig {
    pub name: String,
    pub topology: CpuTopology,
    pub lifecycle: LifecycleConfig,
    pub extra_capabilities: CapabilitySet,
    pub shared_capabilities: Option<CapabilitySet>,
    pub offers: Vec<ResourceKind>,
    pub calibration_threads: usize,
}
impl CpuSubengineConfig {
    pub fn new(name: impl Into<String>, topology: CpuTopology) -> Self {
        let lifecycle = LifecycleConfig::new(
                Duration::from_secs(30),
                Duration::from_secs(300),
                Duration::from_secs(5),
            )
            .expect("default lifecycle durations are ordered");
        Self {
            name: name.into(),
            topology,
            lifecycle,
            extra_capabilities: CapabilitySet::new(),
            shared_capabilities: None,
            offers: vec![
                ResourceKind::Tokenizer, ResourceKind::Normalizer, ResourceKind::Encoder,
                ResourceKind::Scorer, ResourceKind::Index,
            ],
            calibration_threads: 1,
        }
    }
}
#[derive(Clone)]
pub struct CpuDeps {
    pub models: Arc<ModelRegistry>,
    pub registrations: Arc<QueueRegistrations>,
    /// Register owned by this sub-engine and handed to the core only; CPU kernels do not use it.
    pub core_kernels: Arc<KernelRegister>,
}
#[derive(Clone)]
pub struct ModelSetup {
    pub descriptor: Arc<ModelDescriptor>,
    pub normalization: NormalizationParams,
    pub external_normalizer: Option<Arc<dyn TextNormalizer>>,
}
#[derive(Clone)]
pub struct CpuIndexSpec {
    pub index: IndexId,
    pub path: PathBuf,
    pub header: IndexHeader,
    pub open: OpenOptions,
    pub advisor: Option<Arc<dyn AdviceTarget>>,
    pub idle_after: Duration,
}
/// Creates plain host lanes without a stream.
#[derive(Clone, Copy, Debug, Default)]
pub struct CpuLaneFactory;
impl LaneFactory for CpuLaneFactory {
    fn create(&self, cfg: &LaneConfig) -> Result<Lane, String> {
        Ok(Lane::new(cfg.clone(), None))
    }
}
/// The only compute path of the CPU: f32 on the query lane.
#[derive(Clone, Copy, Debug, Default)]
pub struct CpuPathPolicy;
impl PathPolicy for CpuPathPolicy {
    fn select(
        &self,
        req: &ResourceRequirements,
        _caps: &CapabilitySet,
        _device: DeviceHandle,
    ) -> Option<ComputePath> {
        let kernel = match req.kind() {
            ResourceKind::Scorer => KERNEL_SCORE_DOT,
            ResourceKind::Encoder | ResourceKind::CrossEncoder => ENCODER_KERNEL,
            _ => return None,
        };
        Some(ComputePath {
            kernel: kernel.to_string(),
            dtype: DType::F32,
            variant: Variant { dim: None, tile: None },
            lane: LaneKind::Query,
        })
    }
}
pub trait CpuScoreKernel: Send + Sync {
    /// `rows`: n*dim contiguous f32 values; `out.len() == n`; f32 accumulation.
    fn score(&self, query: &[f32], rows: &[f32], dim: usize, out: &mut [f32]);
}
type KernelKey = (String, DType);
struct LiveKernel {
    handle: Arc<dyn CpuScoreKernel>,
    last_used: Instant,
}
struct KernelHandle(Arc<dyn CpuScoreKernel>);
impl CpuScoreKernel for KernelHandle {
    fn score(&self, query: &[f32], rows: &[f32], dim: usize, out: &mut [f32]) {
        self.0.score(query, rows, dim, out);
    }
}
#[derive(Default)]
struct KernelState {
    defined: HashMap<KernelKey, Arc<dyn CpuScoreKernel>>,
    live: HashMap<KernelKey, LiveKernel>,
}
/// Register of CPU score kernels, separate from the GPU register.
pub struct CpuKernelRegister {
    state: Mutex<KernelState>,
}
impl CpuKernelRegister {
    pub fn with_builtin() -> Self {
        let register = Self {
            state: Mutex::new(KernelState::default()),
        };
        let isa = Isa::detect();
        let builtin: [(&str, Op); 2] = [
            (Metric::Dot.kernel_name(), Op::Dot),
            (Metric::NegL2Sq.kernel_name(), Op::NegL2),
        ];
        for (name, op) in builtin {
            let kernel: Arc<dyn CpuScoreKernel> = Arc::new(BuiltinKernel { op, isa });
            register
                .register(name, DType::F32, kernel)
                .expect("builtin kernel names are unique");
        }
        register
    }
    pub fn register(
        &self,
        name: &str,
        dtype: DType,
        kernel: Arc<dyn CpuScoreKernel>,
    ) -> Result<(), CpuError> {
        let mut st = lock(&self.state);
        let key = (name.to_string(), dtype);
        if st.defined.contains_key(&key) {
            return Err(
                CpuError::InvalidConfig(
                    format!("kernel {name} for {dtype:?} is already registered"),
                ),
            );
        }
        st.defined.insert(key, kernel);
        Ok(())
    }
    pub fn request(
        &self,
        name: &str,
        dtype: DType,
        now: Instant,
    ) -> Result<Arc<dyn CpuScoreKernel>, CpuError> {
        let mut st = lock(&self.state);
        let key = (name.to_string(), dtype);
        if let Some(live) = st.live.get_mut(&key) {
            live.last_used = now;
            return Ok(Arc::clone(&live.handle));
        }
        let Some(defined) = st.defined.get(&key).cloned() else {
            return Err(CpuError::NoKernel {
                name: name.to_string(),
                dtype,
            });
        };
        let handle: Arc<dyn CpuScoreKernel> = Arc::new(KernelHandle(defined));
        st.live
            .insert(
                key,
                LiveKernel {
                    handle: Arc::clone(&handle),
                    last_used: now,
                },
            );
        Ok(handle)
    }
    /// Drops unused active kernels idle for at least `ttl`. Returns `kernel:{name}:{dtype:?}` keys.
    pub fn evict_expired(&self, now: Instant, ttl: Duration) -> Vec<String> {
        let mut st = lock(&self.state);
        let expired: Vec<KernelKey> = st
            .live
            .iter()
            .filter(|(_, live)| {
                Arc::strong_count(&live.handle) == 1
                    && now.saturating_duration_since(live.last_used) >= ttl
            })
            .map(|(key, _)| key.clone())
            .collect();
        for key in &expired {
            st.live.remove(key);
        }
        let mut keys: Vec<String> = expired
            .into_iter()
            .map(|(name, dtype)| format!("kernel:{name}:{dtype:?}"))
            .collect();
        keys.sort();
        keys
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Op {
    Dot,
    NegL2,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Isa {
    Scalar,
    #[cfg(target_arch = "x86_64")]
    Avx2Fma,
    #[cfg(target_arch = "aarch64")]
    Neon,
}
impl Isa {
    fn detect() -> Isa {
        #[cfg(target_arch = "x86_64")]
        {
            if std::is_x86_feature_detected!("avx2")
                && std::is_x86_feature_detected!("fma")
            {
                return Isa::Avx2Fma;
            }
        }
        #[cfg(target_arch = "aarch64")]
        {
            if std::arch::is_aarch64_feature_detected!("neon") {
                return Isa::Neon;
            }
        }
        Isa::Scalar
    }
}
struct BuiltinKernel {
    op: Op,
    isa: Isa,
}
impl CpuScoreKernel for BuiltinKernel {
    fn score(&self, query: &[f32], rows: &[f32], dim: usize, out: &mut [f32]) {
        for (i, slot) in out.iter_mut().enumerate() {
            let row = i
                .checked_mul(dim)
                .and_then(|start| {
                    start.checked_add(dim).and_then(|end| rows.get(start..end))
                });
            *slot = match row {
                Some(row) if row.len() == query.len() => {
                    row_score(self.isa, self.op, query, row)
                }
                _ => f32::NAN,
            };
        }
    }
}
fn row_score(isa: Isa, op: Op, query: &[f32], row: &[f32]) -> f32 {
    match isa {
        #[cfg(target_arch = "x86_64")]
        Isa::Avx2Fma => {
            unsafe {
                match op {
                    Op::Dot => x86::dot(query, row),
                    Op::NegL2 => -x86::sq_dist(query, row),
                }
            }
        }
        #[cfg(target_arch = "aarch64")]
        Isa::Neon => {
            unsafe {
                match op {
                    Op::Dot => neon::dot(query, row),
                    Op::NegL2 => -neon::sq_dist(query, row),
                }
            }
        }
        Isa::Scalar => {
            match op {
                Op::Dot => dot_scalar(query, row),
                Op::NegL2 => -sq_dist_scalar(query, row),
            }
        }
    }
}
fn dot_scalar(a: &[f32], b: &[f32]) -> f32 {
    a.iter().zip(b).map(|(x, y)| x * y).sum()
}
fn sq_dist_scalar(a: &[f32], b: &[f32]) -> f32 {
    a.iter()
        .zip(b)
        .map(|(x, y)| {
            let d = x - y;
            d * d
        })
        .sum()
}
#[cfg(target_arch = "x86_64")]
mod x86 {
    use std::arch::x86_64::*;
    #[target_feature(enable = "avx2,fma")]
    unsafe fn hsum(v: __m256) -> f32 {
        let lo = _mm256_castps256_ps128(v);
        let hi = _mm256_extractf128_ps::<1>(v);
        let s = _mm_add_ps(lo, hi);
        let s = _mm_add_ps(s, _mm_movehl_ps(s, s));
        let s = _mm_add_ss(s, _mm_shuffle_ps::<0x55>(s, s));
        _mm_cvtss_f32(s)
    }
    #[target_feature(enable = "avx2,fma")]
    pub(super) unsafe fn dot(a: &[f32], b: &[f32]) -> f32 {
        let n = a.len().min(b.len());
        let (pa, pb) = (a.as_ptr(), b.as_ptr());
        let mut acc0 = _mm256_setzero_ps();
        let mut acc1 = _mm256_setzero_ps();
        let mut i = 0;
        while i + 16 <= n {
            acc0 = _mm256_fmadd_ps(
                _mm256_loadu_ps(pa.add(i)),
                _mm256_loadu_ps(pb.add(i)),
                acc0,
            );
            acc1 = _mm256_fmadd_ps(
                _mm256_loadu_ps(pa.add(i + 8)),
                _mm256_loadu_ps(pb.add(i + 8)),
                acc1,
            );
            i += 16;
        }
        if i + 8 <= n {
            acc0 = _mm256_fmadd_ps(
                _mm256_loadu_ps(pa.add(i)),
                _mm256_loadu_ps(pb.add(i)),
                acc0,
            );
            i += 8;
        }
        let mut sum = hsum(_mm256_add_ps(acc0, acc1));
        while i < n {
            sum += *pa.add(i) * *pb.add(i);
            i += 1;
        }
        sum
    }
    #[target_feature(enable = "avx2,fma")]
    pub(super) unsafe fn sq_dist(a: &[f32], b: &[f32]) -> f32 {
        let n = a.len().min(b.len());
        let (pa, pb) = (a.as_ptr(), b.as_ptr());
        let mut acc0 = _mm256_setzero_ps();
        let mut acc1 = _mm256_setzero_ps();
        let mut i = 0;
        while i + 16 <= n {
            let d0 = _mm256_sub_ps(
                _mm256_loadu_ps(pa.add(i)),
                _mm256_loadu_ps(pb.add(i)),
            );
            let d1 = _mm256_sub_ps(
                _mm256_loadu_ps(pa.add(i + 8)),
                _mm256_loadu_ps(pb.add(i + 8)),
            );
            acc0 = _mm256_fmadd_ps(d0, d0, acc0);
            acc1 = _mm256_fmadd_ps(d1, d1, acc1);
            i += 16;
        }
        if i + 8 <= n {
            let d = _mm256_sub_ps(
                _mm256_loadu_ps(pa.add(i)),
                _mm256_loadu_ps(pb.add(i)),
            );
            acc0 = _mm256_fmadd_ps(d, d, acc0);
            i += 8;
        }
        let mut sum = hsum(_mm256_add_ps(acc0, acc1));
        while i < n {
            let d = *pa.add(i) - *pb.add(i);
            sum += d * d;
            i += 1;
        }
        sum
    }
}
#[cfg(target_arch = "aarch64")]
mod neon {
    use std::arch::aarch64::*;
    #[target_feature(enable = "neon")]
    pub(super) unsafe fn dot(a: &[f32], b: &[f32]) -> f32 {
        let n = a.len().min(b.len());
        let (pa, pb) = (a.as_ptr(), b.as_ptr());
        let mut acc0 = vdupq_n_f32(0.0);
        let mut acc1 = vdupq_n_f32(0.0);
        let mut i = 0;
        while i + 8 <= n {
            acc0 = vfmaq_f32(acc0, vld1q_f32(pa.add(i)), vld1q_f32(pb.add(i)));
            acc1 = vfmaq_f32(acc1, vld1q_f32(pa.add(i + 4)), vld1q_f32(pb.add(i + 4)));
            i += 8;
        }
        if i + 4 <= n {
            acc0 = vfmaq_f32(acc0, vld1q_f32(pa.add(i)), vld1q_f32(pb.add(i)));
            i += 4;
        }
        let mut sum = vaddvq_f32(vaddq_f32(acc0, acc1));
        while i < n {
            sum += *pa.add(i) * *pb.add(i);
            i += 1;
        }
        sum
    }
    #[target_feature(enable = "neon")]
    pub(super) unsafe fn sq_dist(a: &[f32], b: &[f32]) -> f32 {
        let n = a.len().min(b.len());
        let (pa, pb) = (a.as_ptr(), b.as_ptr());
        let mut acc0 = vdupq_n_f32(0.0);
        let mut acc1 = vdupq_n_f32(0.0);
        let mut i = 0;
        while i + 8 <= n {
            let d0 = vsubq_f32(vld1q_f32(pa.add(i)), vld1q_f32(pb.add(i)));
            let d1 = vsubq_f32(vld1q_f32(pa.add(i + 4)), vld1q_f32(pb.add(i + 4)));
            acc0 = vfmaq_f32(acc0, d0, d0);
            acc1 = vfmaq_f32(acc1, d1, d1);
            i += 8;
        }
        if i + 4 <= n {
            let d = vsubq_f32(vld1q_f32(pa.add(i)), vld1q_f32(pb.add(i)));
            acc0 = vfmaq_f32(acc0, d, d);
            i += 4;
        }
        let mut sum = vaddvq_f32(vaddq_f32(acc0, acc1));
        while i < n {
            let d = *pa.add(i) - *pb.add(i);
            sum += d * d;
            i += 1;
        }
        sum
    }
}
/// Opens the index file on the first successful `get` and then always returns the same instance.
pub struct FileLazyIndex {
    path: PathBuf,
    header: IndexHeader,
    open: OpenOptions,
    slot: IndexSlot,
    advice: Arc<AdviceSlot>,
}
impl FileLazyIndex {
    pub fn new(
        path: PathBuf,
        header: IndexHeader,
        open: OpenOptions,
        slot: IndexSlot,
    ) -> Self {
        Self {
            path,
            header,
            open,
            slot,
            advice: Arc::new(AdviceSlot::default()),
        }
    }
    /// Binds the slot that receives the hint target of the mapped file.
    pub fn with_advice_slot(mut self, advice: Arc<AdviceSlot>) -> Self {
        self.advice = advice;
        self
    }
    pub fn slot(&self) -> &IndexSlot {
        &self.slot
    }
    fn build(&self) -> Result<Arc<dyn Index>, String> {
        let checksum: ChecksumFn = Arc::new(compute_checksum_parts);
        let decoder = Arc::new(ParquetDecoder::new(self.header.clone(), checksum));
        let checksums = Arc::new(PayloadChecksums::new(Arc::clone(&decoder)));
        let file = if self.path.exists() {
            let file = LazyIndexFile::new(&self.path, decoder, self.open.clone())
                .with_advice_slot(Arc::clone(&self.advice));
            file.ensure_open().map_err(|e| e.to_string())?;
            Some(Arc::new(file))
        } else {
            None
        };
        Ok(Arc::new(EntryStore::new(self.header.clone(), file, checksums)))
    }
}
impl LazyIndex for FileLazyIndex {
    fn get(&self) -> Result<Arc<dyn Index>, String> {
        if let Some(index) = self.slot.get() {
            return Ok(Arc::clone(index));
        }
        let opened = self.build()?;
        let _ = self.slot.set(opened);
        self.slot
            .get()
            .cloned()
            .ok_or_else(|| "index slot is empty after opening".to_string())
    }
}
struct AttachedIndex {
    lazy: FileLazyIndex,
    residency: Arc<IndexResidency>,
    vector_bytes: usize,
}
/// Registry of the attached indexes. It is not a slot manager and keeps no copy of index data.
#[derive(Default)]
pub struct CpuManager {
    attached: Mutex<HashMap<IndexId, Arc<AttachedIndex>>>,
}
impl CpuManager {
    pub fn new() -> Self {
        Self::default()
    }
    /// Builds the slot, the lazy index and the residency. Opens and loads nothing.
    pub fn attach(&self, spec: CpuIndexSpec) -> Result<(), CpuError> {
        let mut attached = lock(&self.attached);
        if attached.contains_key(&spec.index) {
            return Err(CpuError::AlreadyAttached(spec.index));
        }
        let slot: IndexSlot = IndexSlot::default();
        let advice = Arc::new(AdviceSlot::default());
        let advisor: Arc<dyn AdviceTarget> = match spec.advisor {
            Some(explicit) => explicit,
            None => Arc::clone(&advice) as Arc<dyn AdviceTarget>,
        };
        let residency = Arc::new(
            IndexResidency::new(Arc::clone(&slot), Some(advisor), spec.idle_after),
        );
        let vector_bytes = spec.header.vector_bytes();
        let lazy = FileLazyIndex::new(spec.path, spec.header, spec.open, slot)
            .with_advice_slot(advice);
        attached
            .insert(
                spec.index,
                Arc::new(AttachedIndex {
                    lazy,
                    residency,
                    vector_bytes,
                }),
            );
        Ok(())
    }
    /// Opens the index on the first call and returns the same instance afterwards.
    pub fn index(&self, index: &IndexId) -> Result<Arc<dyn Index>, CpuError> {
        let entry = self.entry(index)?;
        entry.lazy.get().map_err(CpuError::IndexOpen)
    }
    pub fn residency(&self, index: &IndexId) -> Option<Arc<IndexResidency>> {
        lock(&self.attached).get(index).map(|e| Arc::clone(&e.residency))
    }
    /// Byte size of one index vector (`dimension * 2`), if the index is attached.
    pub fn vector_bytes(&self, index: &IndexId) -> Option<usize> {
        lock(&self.attached).get(index).map(|e| e.vector_bytes)
    }
    /// Marks idle residency ranges of every attached index as cold; returns the number of ranges.
    pub fn mark_idle_cold(&self, now: Instant) -> usize {
        let residencies: Vec<Arc<IndexResidency>> = lock(&self.attached)
            .values()
            .map(|e| Arc::clone(&e.residency))
            .collect();
        residencies.iter().map(|r| r.mark_idle_cold(now)).sum()
    }
    fn entry(&self, index: &IndexId) -> Result<Arc<AttachedIndex>, CpuError> {
        lock(&self.attached)
            .get(index)
            .cloned()
            .ok_or_else(|| CpuError::UnknownIndex(index.clone()))
    }
}
#[derive(Clone, Copy, Debug)]
pub struct CpuScoreRequest<'a> {
    pub index: &'a IndexId,
    pub metric: Metric,
    pub query: &'a [f32],
    pub entries: &'a [EntryId],
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CpuScored {
    pub entry: EntryId,
    pub score: f32,
}
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CpuScoreResult {
    /// Request order, one per scorable entry; not sorted.
    pub scored: Vec<CpuScored>,
    /// Entries that are not in the index.
    pub absent: Vec<EntryId>,
    /// Entries whose vector length differs from the index vector size.
    pub malformed: Vec<EntryId>,
}
struct ModelEntry {
    preprocessor: TextPreprocessor,
    encoder_key: ResourceKey,
    tokenizer_key: ResourceKey,
}
pub struct CpuSubengine {
    name: String,
    topology: CpuTopology,
    core: Arc<SubengineCore>,
    models: Arc<ModelRegistry>,
    own_capabilities: CapabilitySet,
    offers: Vec<ResourceKind>,
    ram_ttl: Duration,
    kernels: CpuKernelRegister,
    manager: CpuManager,
    encoders: ResourceRegistry<LoadedEncoder>,
    tokenizers: ResourceRegistry<LoadedTokenizer>,
    entries: Mutex<HashMap<ModelId, ModelEntry>>,
    calibration: BackgroundPool,
}
impl CpuSubengine {
    /// Validates, builds the core, the registries and the calibration pool. Loads nothing.
    pub fn new(cfg: CpuSubengineConfig, deps: CpuDeps) -> Result<Arc<Self>, CpuError> {
        if cfg.name.is_empty() {
            return Err(CpuError::InvalidConfig("name is empty".to_string()));
        }
        let mut own = precision_capabilities(DeviceKind::Cpu);
        own.extend(cfg.extra_capabilities.iter().map(|(a, v)| (a.clone(), v.clone())));
        let shared = cfg.shared_capabilities.clone().unwrap_or_else(|| own.clone());
        let lanes = [
            LaneKind::Query,
            LaneKind::Insert,
            LaneKind::Update,
            LaneKind::CopyH2D,
        ]
            .into_iter()
            .map(|kind| LaneConfig::preset(kind, DeviceHandle::Cpu))
            .collect();
        let core = SubengineCore::new(
                SubengineConfig {
                    name: cfg.name.clone(),
                    device: DeviceHandle::Cpu,
                    own_queue: QueueKind::Cpu,
                    own_capabilities: own.clone(),
                    shared_capabilities: shared,
                    offers: cfg.offers.clone(),
                    lanes,
                },
                Arc::new(CpuLaneFactory),
                Arc::new(CpuPathPolicy),
                Arc::clone(&deps.core_kernels),
                Arc::clone(&deps.models),
                Arc::clone(&deps.registrations),
            )
            .map_err(CpuError::Subengine)?;
        for metric in [Metric::Dot, Metric::NegL2Sq] {
            if deps.models.scorer_available(metric) {
                core.adopt_scorer(metric).map_err(CpuError::Subengine)?;
            }
        }
        let calibration = BackgroundPool::new(
                &format!("{}-calib", cfg.name),
                cfg.calibration_threads.max(1),
            )
            .map_err(CpuError::Pool)?;
        let offers = Subengine::offers(&core);
        Ok(
            Arc::new(Self {
                name: cfg.name,
                topology: cfg.topology,
                core: Arc::new(core),
                models: deps.models,
                own_capabilities: own,
                offers,
                ram_ttl: cfg.lifecycle.ram_ttl(),
                kernels: CpuKernelRegister::with_builtin(),
                manager: CpuManager::new(),
                encoders: ResourceRegistry::new(cfg.lifecycle.clone()),
                tokenizers: ResourceRegistry::new(cfg.lifecycle),
                entries: Mutex::new(HashMap::new()),
                calibration,
            }),
        )
    }
    pub fn core(&self) -> &Arc<SubengineCore> {
        &self.core
    }
    pub fn manager(&self) -> &CpuManager {
        &self.manager
    }
    pub fn kernels(&self) -> &CpuKernelRegister {
        &self.kernels
    }
    pub fn topology(&self) -> &CpuTopology {
        &self.topology
    }
    pub fn default_dtype(&self) -> DType {
        DType::F32
    }
    /// Registers the encoder and tokenizer resources of the model (RAM). Loads nothing.
    pub fn register_model(
        &self,
        model: ModelId,
        setup: ModelSetup,
    ) -> Result<(), CpuError> {
        let mut entries = lock(&self.entries);
        if entries.contains_key(&model) {
            return Err(CpuError::AlreadyRegistered(model));
        }
        let preprocessor = TextPreprocessor::new(setup.external_normalizer);
        let encoder_key: ResourceKey = format!("enc:{model}");
        let tokenizer_key: ResourceKey = format!("tok:{model}");
        let encoder = self
            .models
            .encoder_resource(Arc::clone(&setup.descriptor), DeviceClass::Cpu);
        let tokenizer = TokenizerLoader::from_descriptor(&setup.descriptor)
            .with_normalization(
                Arc::new(
                    NormalizerLoader::new(setup.normalization, preprocessor.clone()),
                ),
            );
        self.encoders
            .register(encoder_key.clone(), MemoryClass::Ram, encoder.loader)
            .map_err(|e| CpuError::InvalidConfig(format!("{e:?}")))?;
        self.tokenizers
            .register(tokenizer_key.clone(), MemoryClass::Ram, Arc::new(tokenizer))
            .map_err(|e| CpuError::InvalidConfig(format!("{e:?}")))?;
        entries
            .insert(
                model,
                ModelEntry {
                    preprocessor,
                    encoder_key,
                    tokenizer_key,
                },
            );
        Ok(())
    }
    /// Builds the lane resource with one stream-less lane per topology lane.
    pub fn lane_resource<M, P>(
        self: &Arc<Self>,
        cfg: LaneResourceConfig<M, P>,
        runner: Arc<dyn BatchRunner<P>>,
    ) -> Result<Arc<LaneResource<M, P>>, CpuError>
    where
        P: BatchPayload,
        M: BatchMeta + HasResourceRequirements + Send + Sync + 'static,
    {
        if cfg.device != DeviceHandle::Cpu {
            return Err(
                CpuError::InvalidConfig(
                    format!("lane resource device is {}, expected the CPU", cfg.device),
                ),
            );
        }
        let lanes = (0..self.topology.lane_count())
            .map(|_| Lane::new(
                LaneConfig::preset(LaneKind::Query, DeviceHandle::Cpu),
                None,
            ))
            .collect();
        LaneResource::new(cfg, lanes, runner).map_err(CpuError::Lane)
    }
    /// End-of-iteration eviction of encoders, tokenizers and kernels; also marks idle ranges cold.
    pub fn evict_expired(&self, now: Instant) -> Vec<ResourceKey> {
        let mut evicted = self.encoders.evict_expired(now);
        evicted.extend(self.tokenizers.evict_expired(now));
        evicted.extend(self.kernels.evict_expired(now, self.ram_ttl));
        self.manager.mark_idle_cold(now);
        evicted
    }
    /// Starts calibration on the low-priority pool. Does not block and applies nothing.
    pub fn calibrate(
        &self,
        model: Arc<ModelDescriptor>,
        backend: Arc<dyn CalibrationBackend>,
        config: CalibrationConfig,
    ) -> Result<JobFuture<Result<CalibrationReport, CalibrationError>>, CpuError> {
        let subengine = self.name.clone();
        let capabilities = self.own_capabilities.clone();
        self.calibration
            .submit(move || {
                let target = Target {
                    subengine,
                    model_id: model.model_id.clone(),
                };
                run_calibration(target, &capabilities, &model, backend.as_ref(), &config)
            })
            .map_err(CpuError::Pool)
    }
    fn model_keys(
        &self,
        model: Option<&ModelId>,
        key_of: fn(&ModelEntry) -> &ResourceKey,
    ) -> Vec<ResourceKey> {
        let entries = lock(&self.entries);
        match model {
            Some(m) => entries.get(m).map(|e| key_of(e).clone()).into_iter().collect(),
            None => entries.values().map(|e| key_of(e).clone()).collect(),
        }
    }
    fn entry_key(
        &self,
        model: &ModelId,
        key_of: fn(&ModelEntry) -> &ResourceKey,
    ) -> Result<ResourceKey, CpuError> {
        lock(&self.entries)
            .get(model)
            .map(|e| key_of(e).clone())
            .ok_or_else(|| CpuError::UnknownModel(model.clone()))
    }
}
fn encoder_key_of(entry: &ModelEntry) -> &ResourceKey {
    &entry.encoder_key
}
fn tokenizer_key_of(entry: &ModelEntry) -> &ResourceKey {
    &entry.tokenizer_key
}
fn any_ready<H: Send + Sync + 'static>(
    registry: &ResourceRegistry<H>,
    keys: &[ResourceKey],
) -> bool {
    keys.iter().any(|k| registry.state(k) == Some(ResourceState::Ready))
}
impl Subengine for CpuSubengine {
    fn name(&self) -> String {
        self.core.name()
    }
    fn start(&self) -> Result<(), String> {
        self.core.start()
    }
    fn capabilities(&self) -> Vec<(QueueKind, CapabilitySet)> {
        self.core.capabilities()
    }
    fn offers(&self) -> Vec<ResourceKind> {
        self.core.offers()
    }
    fn announce(&self, req: &ResourceRequirements) {
        self.core.announce(req);
    }
    fn activate(&self) -> Result<(), String> {
        self.core.activate()
    }
    fn is_loaded(&self, kind: ResourceKind, model: Option<&ModelId>) -> bool {
        if !self.offers.contains(&kind) {
            return false;
        }
        match kind {
            ResourceKind::Encoder => {
                any_ready(&self.encoders, &self.model_keys(model, encoder_key_of))
            }
            ResourceKind::Tokenizer | ResourceKind::Normalizer => {
                any_ready(&self.tokenizers, &self.model_keys(model, tokenizer_key_of))
            }
            _ => self.core.is_loaded(kind, model),
        }
    }
}
fn load_error(error: LifecycleError) -> CpuError {
    match error {
        LifecycleError::Failed(message) => CpuError::Load(message),
        other => CpuError::Load(format!("{other:?}")),
    }
}
fn acquire<H: Send + Sync + 'static>(
    registry: &ResourceRegistry<H>,
    key: &str,
) -> Result<Lease<H>, CpuError> {
    loop {
        match registry.request_init(key, 0, Instant::now()) {
            InitRequest::Ready(lease) => return Ok(lease),
            InitRequest::Started(ticket) => {
                ticket.run(Instant::now()).map_err(load_error)?
            }
            InitRequest::Loading => std::thread::sleep(LOADING_POLL),
            InitRequest::Backoff { until } => return Err(CpuError::Backoff { until }),
            InitRequest::Unavailable => {
                return Err(CpuError::Unavailable(key.to_string()));
            }
        }
    }
}
fn append_f16_row(dst: &mut Vec<f32>, bytes: &[u8]) {
    dst.extend(bytes.chunks_exact(2).map(|c| f16::from_le_bytes([c[0], c[1]]).to_f32()));
}
/// Execution context of one lane thread. Cheap to create and holds no state.
pub struct CpuContext<'a> {
    sub: &'a CpuSubengine,
    lane: &'a Lane,
}
impl<'a> CpuContext<'a> {
    pub fn new(sub: &'a CpuSubengine, lane: &'a Lane) -> Self {
        Self { sub, lane }
    }
    pub fn encoder(&self, model: &ModelId) -> Result<Lease<LoadedEncoder>, CpuError> {
        let key = self.sub.entry_key(model, encoder_key_of)?;
        acquire(&self.sub.encoders, &key)
    }
    pub fn tokenizer(
        &self,
        model: &ModelId,
    ) -> Result<Lease<LoadedTokenizer>, CpuError> {
        let key = self.sub.entry_key(model, tokenizer_key_of)?;
        acquire(&self.sub.tokenizers, &key)
    }
    /// The model's text preprocessor; identity if the model has no external normalizer.
    pub fn preprocessor(&self, model: &ModelId) -> Result<TextPreprocessor, CpuError> {
        lock(&self.sub.entries)
            .get(model)
            .map(|e| e.preprocessor.clone())
            .ok_or_else(|| CpuError::UnknownModel(model.clone()))
    }
    pub fn preprocess<'t>(
        &self,
        model: &ModelId,
        text: &'t str,
    ) -> Result<Cow<'t, str>, CpuError> {
        self.preprocessor(model)?.apply(text).map_err(|e| CpuError::Normalize(e.0))
    }
    /// Loads the encoder if needed, encodes and returns once the output region is written.
    pub fn encode(
        &self,
        model: &ModelId,
        input: &TensorRef,
        output: &TensorRef,
        rows: usize,
        seq_len: usize,
    ) -> Result<(), CpuError> {
        let lease = self.encoder(model)?;
        let result = (|| {
            let (sender, receiver) = oneshot();
            let event = lease
                .handle()
                .encoder()
                .encode_async(self.lane, input, output, rows, seq_len, vec![sender])
                .map_err(CpuError::Encoder)?;
            event.wait().map_err(CpuError::Encoder)?;
            match receiver.recv() {
                Ok(Ok(())) => Ok(()),
                Ok(Err(e)) => Err(CpuError::Encoder(e)),
                Err(_) => {
                    Err(
                        CpuError::Encoder(
                            EncoderError::Processing(
                                "encoder result channel closed".to_string(),
                            ),
                        ),
                    )
                }
            }
        })();
        lease.release(Instant::now());
        result
    }
    pub fn score(&self, req: &CpuScoreRequest<'_>) -> Result<CpuScoreResult, CpuError> {
        let manager = &self.sub.manager;
        let index = manager.index(req.index)?;
        let row_bytes = manager
            .vector_bytes(req.index)
            .ok_or_else(|| CpuError::UnknownIndex(req.index.clone()))?;
        let dim = req.query.len();
        if dim == 0 || dim * 2 != row_bytes {
            return Err(CpuError::QueryDimMismatch {
                query: dim,
                row_bytes,
            });
        }
        let kernel = self
            .sub
            .kernels
            .request(req.metric.kernel_name(), DType::F32, Instant::now())?;
        let block = SCORE_BLOCK_ROWS.min(req.entries.len().max(1));
        let mut seen: HashSet<EntryId> = HashSet::with_capacity(req.entries.len());
        let mut result = CpuScoreResult::default();
        let mut ids: Vec<EntryId> = Vec::with_capacity(block);
        let mut rows: Vec<f32> = Vec::with_capacity(block * dim);
        let mut out: Vec<f32> = Vec::with_capacity(block);
        let mut flush = |
            ids: &mut Vec<EntryId>,
            rows: &mut Vec<f32>,
            result: &mut CpuScoreResult|
        {
            if ids.is_empty() {
                return;
            }
            out.clear();
            out.resize(ids.len(), 0.0);
            kernel.score(req.query, rows, dim, &mut out);
            result
                .scored
                .extend(
                    ids
                        .iter()
                        .zip(out.iter())
                        .map(|(&entry, &score)| CpuScored { entry, score }),
                );
            ids.clear();
            rows.clear();
        };
        for &id in req.entries {
            if !seen.insert(id) {
                continue;
            }
            let payload = index.read(id);
            let vector: &[u8] = &payload.vector;
            if vector.is_empty() {
                result.absent.push(id);
                continue;
            }
            if vector.len() != row_bytes {
                warn!(
                    "entry {} of index {}: vector has {} bytes, expected {}", id.0, req
                    .index, vector.len(), row_bytes
                );
                result.malformed.push(id);
                continue;
            }
            append_f16_row(&mut rows, vector);
            ids.push(id);
            if ids.len() == SCORE_BLOCK_ROWS {
                flush(&mut ids, &mut rows, &mut result);
            }
        }
        flush(&mut ids, &mut rows, &mut result);
        Ok(result)
    }
    pub fn index(&self, index: &IndexId) -> Result<Arc<dyn Index>, CpuError> {
        self.sub.manager.index(index)
    }
}
/// Payload executable on the CPU. Must complete or fail every task of the batch before returning.
pub trait CpuPayload: BatchPayload + Sized {
    fn execute_batch(batch: ExecBatch<Self>, ctx: &CpuContext<'_>);
}
struct CpuRunner<P> {
    sub: Arc<CpuSubengine>,
    _payload: PhantomData<fn() -> P>,
}
impl<P: CpuPayload> BatchRunner<P> for CpuRunner<P> {
    fn run(&self, lane: &mut Lane, batch: &mut ExecBatch<P>) -> Launch {
        let owned = std::mem::replace(
            batch,
            ExecBatch {
                tasks: Vec::new(),
                spans: Vec::new(),
                batches: None,
            },
        );
        let ctx = CpuContext::new(&self.sub, lane);
        P::execute_batch(owned, &ctx);
        Launch::Finished
    }
    fn collect(
        &self,
        _lane: &mut Lane,
        _batch: ExecBatch<P>,
        _result: Result<(), l010::device_lane_event::EventError>,
    ) {
        warn!(
            "cpu runner: collect called for a batch that always finishes synchronously"
        );
    }
}
pub fn cpu_runner<P: CpuPayload>(sub: Arc<CpuSubengine>) -> Arc<dyn BatchRunner<P>> {
    Arc::new(CpuRunner {
        sub,
        _payload: PhantomData,
    })
}
#[cfg(test)]
#[path = "test/cpu_subengine.rs"]
mod tests;
