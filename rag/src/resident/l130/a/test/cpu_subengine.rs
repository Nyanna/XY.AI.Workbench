use super::*;
use std::path::Path;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Barrier, Condvar};
use std::thread;
use l090::a::lane_batching::{BatchKey, BatchLimit};
use l090::a::persistent_save::index_save::{IndexSaver, SaveConfig, StoreSource};
use l080::a::index_format::{TokenizerFingerprint, TOKENIZER_FP_ALGORITHM};
use l080::a::kernel_scoring::{LaunchRequest, ScoreLauncher, ScoreReadback, Scorer};
use l080::a::meta_codec;
use l080::subengine_loop::ResourceExec;
use l070::a::kernel_register::{
    CodeImage, CodeKind, CompileRequest, CompilerError, KernelCompiler, KernelFunction,
    LoadFailure, ModuleLoader,
};
use l070::a::notification_topology::NotificationTopology;
use l070::a::text_normalization::NormalizeError;
use l040::a::library_adapter::library_kernel_source;
use l030::a::calibration_run::{Combination, ProbeError};
use l020::a::index_contract::{
    AccessMethod, ByteRange, IndexError, Payload, PayloadRef, SharedBytes, Stamp, StampObserver,
    StampSignal,
};
use l020::a::memory_access::{AccessError, Region, RegionBackend};
use l010::capability_model::{axes, cpu_reference, values};
use l010::layout::Layout;
use l010::queue::{Queue, State};
use l010::target_env::ComputeCapability;
use l040::index_residency::Advice;

static_assertions::assert_impl_all!(CpuSubengine: Send, Sync);
static_assertions::assert_impl_all!(CpuManager: Send, Sync);
static_assertions::assert_impl_all!(CpuKernelRegister: Send, Sync);
static_assertions::assert_impl_all!(CpuTopology: Send, Sync);
static_assertions::assert_impl_all!(CpuError: Send, Sync);

const DIM: usize = 8;

// ---------------------------------------------------------------------------------------------
// Fixtures
// ---------------------------------------------------------------------------------------------

static COUNTER: AtomicUsize = AtomicUsize::new(0);

struct TempDir(PathBuf);

impl TempDir {
    fn new() -> TempDir {
        let n = COUNTER.fetch_add(1, Ordering::SeqCst);
        let path = std::env::temp_dir().join(format!("cpu_subengine_{}_{n}", std::process::id()));
        std::fs::create_dir_all(&path).unwrap();
        TempDir(path)
    }

    fn file(&self, name: &str) -> PathBuf {
        self.0.join(name)
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

struct Lcg(u64);

impl Lcg {
    fn next(&mut self) -> f32 {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        ((self.0 >> 40) as f32 / (1u64 << 24) as f32) * 2.0 - 1.0
    }

    fn vector(&mut self, len: usize) -> Vec<f32> {
        (0..len).map(|_| self.next()).collect()
    }
}

struct StubCompiler;

impl KernelCompiler for StubCompiler {
    fn compile(&self, _req: &CompileRequest) -> Result<CodeImage, CompilerError> {
        Ok(CodeImage { kind: CodeKind::Ptx, bytes: vec![0] })
    }
}

#[derive(Debug)]
struct MockFunction;

impl KernelFunction for MockFunction {
    fn symbol(&self) -> &str {
        "test_entry"
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

struct StubLoader;

impl ModuleLoader for StubLoader {
    fn device_capability(&self) -> Option<ComputeCapability> {
        None
    }

    fn load(&self, _image: &CodeImage, _symbol: &str) -> Result<Arc<dyn KernelFunction>, LoadFailure> {
        Ok(Arc::new(MockFunction))
    }
}

struct StubLauncher;

impl ScoreLauncher for StubLauncher {
    fn launch(
        &self,
        _function: &Arc<dyn KernelFunction>,
        _lane: &Lane,
        _req: &LaunchRequest<'_>,
    ) -> Result<Box<dyn ScoreReadback>, String> {
        Err("stub".to_string())
    }
}

fn core_kernels() -> Arc<KernelRegister> {
    Arc::new(KernelRegister::new(
        DeviceHandle::Cpu,
        Arc::new(StubCompiler),
        Arc::new(StubLoader),
        library_kernel_source(),
    ))
}

fn registry_with(metrics: &[Metric]) -> Arc<ModelRegistry> {
    let mut registry = ModelRegistry::with_builtin();
    for metric in metrics {
        registry.register_scorer(*metric, Arc::new(Scorer::new(core_kernels(), Arc::new(StubLauncher))));
    }
    Arc::new(registry)
}

fn deps(models: Arc<ModelRegistry>) -> (CpuDeps, Arc<QueueRegistrations>) {
    let registrations = Arc::new(QueueRegistrations::new());
    let deps = CpuDeps {
        models,
        registrations: Arc::clone(&registrations),
        core_kernels: core_kernels(),
    };
    (deps, registrations)
}

fn config() -> CpuSubengineConfig {
    CpuSubengineConfig::new("cpu0", CpuTopology::single_socket(2, 2).unwrap())
}

fn sub_with(cfg: CpuSubengineConfig, models: Arc<ModelRegistry>) -> Arc<CpuSubengine> {
    CpuSubengine::new(cfg, deps(models).0).unwrap()
}

fn sub() -> Arc<CpuSubengine> {
    sub_with(config(), registry_with(&[]))
}

fn cpu_lane() -> Lane {
    Lane::new(LaneConfig::preset(LaneKind::Query, DeviceHandle::Cpu), None)
}

fn header(dim: usize) -> IndexHeader {
    let mut h = IndexHeader::from_descriptor(
        &ModelDescriptor::e5_small_repo_default(DType::F16),
        "rev-1",
        TokenizerFingerprint::new(TOKENIZER_FP_ALGORITHM, 0x2a),
        "prep",
    )
    .unwrap();
    h.dimension = dim;
    h
}

fn f16_bytes(values: &[f32]) -> Vec<u8> {
    values.iter().flat_map(|v| f16::from_f32(*v).to_le_bytes()).collect()
}

fn f16_round(values: &[f32]) -> Vec<f32> {
    values.iter().map(|v| f16::from_f32(*v).to_f32()).collect()
}

fn write_index(path: &Path, header: &IndexHeader, rows: &[(u64, Vec<f32>)]) {
    let checksum: ChecksumFn = Arc::new(compute_checksum_parts);
    let decoder = Arc::new(ParquetDecoder::new(header.clone(), checksum));
    let store = Arc::new(EntryStore::new(
        header.clone(),
        None,
        Arc::new(PayloadChecksums::new(decoder)),
    ));
    for (id, values) in rows {
        let meta = meta_codec::encode_meta_parts(&format!("f/{id}.txt"), id * 10, id * 10 + 5);
        store.upsert(EntryId(*id), &f16_bytes(values), &meta).unwrap();
    }
    let saver = IndexSaver::new(
        SaveConfig { path: path.to_path_buf(), header: header.clone(), row_group_rows: 64 },
        Arc::new(StoreSource::new(store)),
    );
    saver.save().unwrap();
}

fn spec(id: &str, path: PathBuf, header: IndexHeader) -> CpuIndexSpec {
    CpuIndexSpec {
        index: IndexId::new(id),
        path,
        header,
        open: OpenOptions::default(),
        advisor: None,
        idle_after: Duration::from_secs(1),
    }
}

fn dot_ref(q: &[f32], x: &[f32]) -> f64 {
    q.iter().zip(x).map(|(a, b)| *a as f64 * *b as f64).sum()
}

fn negl2_ref(q: &[f32], x: &[f32]) -> f64 {
    -q.iter().zip(x).map(|(a, b)| (*a as f64 - *b as f64).powi(2)).sum::<f64>()
}

fn assert_close(actual: f32, expected: f64) {
    let tolerance = 1e-4 * (1.0 + expected.abs());
    assert!(
        (actual as f64 - expected).abs() <= tolerance,
        "actual {actual} expected {expected}"
    );
}

struct MockIndex {
    rows: HashMap<u64, Vec<u8>>,
    signal: Arc<StampSignal>,
}

impl MockIndex {
    fn new(rows: Vec<(u64, Vec<u8>)>) -> Arc<Self> {
        Arc::new(Self { rows: rows.into_iter().collect(), signal: StampSignal::new(Stamp(0)) })
    }
}

impl Index for MockIndex {
    fn dtype(&self) -> DType {
        DType::F16
    }

    fn access_methods(&self) -> &[AccessMethod] {
        &[]
    }

    fn stamp(&self, _id: EntryId) -> Option<Stamp> {
        None
    }

    fn stamps(&self) -> Vec<(EntryId, Stamp)> {
        Vec::new()
    }

    fn read(&self, id: EntryId) -> Payload {
        match self.rows.get(&id.0) {
            Some(bytes) => Payload { vector: SharedBytes::from(bytes.clone()), meta: SharedBytes::default() },
            None => Payload::default(),
        }
    }

    fn write(&self, _id: EntryId, _payload: PayloadRef<'_>) -> Result<Stamp, IndexError> {
        Err(IndexError::Unsupported(AccessMethod::Range))
    }

    fn delete(&self, _id: EntryId) -> Result<Option<Stamp>, IndexError> {
        Ok(None)
    }

    fn subscribe(&self) -> StampObserver {
        self.signal.observer()
    }
}

fn inject(sub: &CpuSubengine, id: &IndexId, index: Arc<dyn Index>) {
    let entry = sub.manager.entry(id).unwrap();
    assert!(entry.lazy.slot().set(index).is_ok());
}

// ---------------------------------------------------------------------------------------------
// Topology, configuration, errors
// ---------------------------------------------------------------------------------------------

#[test]
fn topology_counts_and_validation() {
    let t = CpuTopology::new(vec![vec![2, 2], vec![1]]).unwrap();
    assert_eq!((t.sockets(), t.cores(), t.lane_count()), (2, 3, 5));
    let single = CpuTopology::single_socket(4, 2).unwrap();
    assert_eq!((single.sockets(), single.cores(), single.lane_count()), (1, 4, 8));
    assert_eq!(single, CpuTopology::new(vec![vec![2; 4]]).unwrap());
    for bad in [vec![], vec![vec![]], vec![vec![1, 0]], vec![vec![1], vec![]]] {
        assert!(matches!(CpuTopology::new(bad), Err(CpuError::InvalidConfig(_))));
    }
    assert!(CpuTopology::single_socket(0, 2).is_err());
    assert!(CpuTopology::single_socket(2, 0).is_err());
}

#[test]
fn config_defaults() {
    let cfg = config();
    assert_eq!(cfg.name, "cpu0");
    assert_eq!(cfg.lifecycle.ram_ttl(), Duration::from_secs(30));
    assert_eq!(cfg.lifecycle.vram_ttl(), Duration::from_secs(300));
    assert_eq!(cfg.lifecycle.backoff(), Duration::from_secs(5));
    assert!(cfg.extra_capabilities.is_empty());
    assert!(cfg.shared_capabilities.is_none());
    assert_eq!(
        cfg.offers,
        vec![
            ResourceKind::Tokenizer,
            ResourceKind::Normalizer,
            ResourceKind::Encoder,
            ResourceKind::Scorer,
            ResourceKind::Index,
        ]
    );
    assert_eq!(cfg.calibration_threads, 1);
}

#[test]
fn error_display_texts() {
    let id = IndexId::new("idx");
    let model = ModelId::new("m");
    assert_eq!(
        CpuError::InvalidConfig("x".into()).to_string(),
        "ungültige CPU-Subengine-Konfiguration: x"
    );
    assert_eq!(CpuError::UnknownModel(model.clone()).to_string(), "Modell m nicht registriert");
    assert_eq!(CpuError::AlreadyRegistered(model).to_string(), "Modell m bereits registriert");
    assert_eq!(CpuError::UnknownIndex(id.clone()).to_string(), "Index idx nicht angebunden");
    assert_eq!(CpuError::AlreadyAttached(id).to_string(), "Index idx bereits angebunden");
    assert_eq!(CpuError::Load("boom".into()).to_string(), "Laden fehlgeschlagen: boom");
    assert_eq!(CpuError::Backoff { until: Instant::now() }.to_string(), "Ressource im Backoff");
    assert_eq!(
        CpuError::Unavailable("enc:m".into()).to_string(),
        "Ressource nicht verfügbar: enc:m"
    );
    assert_eq!(
        CpuError::IndexOpen("kaputt".into()).to_string(),
        "Index öffnen fehlgeschlagen: kaputt"
    );
    assert_eq!(
        CpuError::NoKernel { name: "k".into(), dtype: DType::F16 }.to_string(),
        "kein CPU-Kernel k für F16"
    );
    assert_eq!(
        CpuError::QueryDimMismatch { query: 3, row_bytes: 16 }.to_string(),
        "Query-Dimension 3 passt nicht zu Vektorgröße 16"
    );
    assert!(std::error::Error::source(&CpuError::Lane(LaneSetError::NoLanes)).is_some());
    assert!(std::error::Error::source(&CpuError::Load("x".into())).is_none());
}

// ---------------------------------------------------------------------------------------------
// Kernels
// ---------------------------------------------------------------------------------------------

fn run_kernel(kernel: &dyn CpuScoreKernel, query: &[f32], rows: &[f32], dim: usize) -> Vec<f32> {
    let n = if dim == 0 { 3 } else { rows.len() / dim };
    let mut out = vec![f32::MIN; n];
    kernel.score(query, rows, dim, &mut out);
    out
}

#[test]
fn builtin_kernels_match_the_reference_for_many_dimensions() {
    let register = CpuKernelRegister::with_builtin();
    let dot = register.request("score_dot", DType::F32, Instant::now()).unwrap();
    let negl2 = register.request("score_negl2", DType::F32, Instant::now()).unwrap();
    let mut rng = Lcg(7);
    for dim in [1usize, 2, 3, 4, 7, 8, 9, 15, 16, 17, 23, 31, 32, 33, 63, 64, 65, 100, 384, 385] {
        let rows_n = 5;
        let query = rng.vector(dim);
        let rows = rng.vector(dim * rows_n);
        let d = run_kernel(dot.as_ref(), &query, &rows, dim);
        let l = run_kernel(negl2.as_ref(), &query, &rows, dim);
        for i in 0..rows_n {
            let row = &rows[i * dim..(i + 1) * dim];
            assert_close(d[i], dot_ref(&query, row));
            assert_close(l[i], negl2_ref(&query, row));
        }
    }
}

#[test]
fn simd_and_scalar_paths_agree() {
    let detected = Isa::detect();
    let mut rng = Lcg(11);
    for dim in [3usize, 8, 17, 64, 384] {
        let query = rng.vector(dim);
        let row = rng.vector(dim);
        for op in [Op::Dot, Op::NegL2] {
            let simd = row_score(detected, op, &query, &row);
            let scalar = row_score(Isa::Scalar, op, &query, &row);
            assert!((simd - scalar).abs() <= 1e-4 * (1.0 + scalar.abs()), "{op:?} dim {dim}");
        }
    }
}

#[test]
fn kernels_pass_nan_and_inf_through() {
    let kernels = [
        BuiltinKernel { op: Op::Dot, isa: Isa::detect() },
        BuiltinKernel { op: Op::Dot, isa: Isa::Scalar },
    ];
    for k in &kernels {
        let mut q = vec![1.0f32; 20];
        let mut row = vec![1.0f32; 20];
        assert_eq!(run_kernel(k, &q, &row, 20), vec![20.0]);
        row[5] = f32::NAN;
        assert!(run_kernel(k, &q, &row, 20)[0].is_nan());
        row[5] = f32::INFINITY;
        assert_eq!(run_kernel(k, &q, &row, 20)[0], f32::INFINITY);
        q[19] = f32::NEG_INFINITY;
        row[19] = 1.0;
        assert!(run_kernel(k, &q, &row, 20)[0].is_nan());
    }
    let negl2 = BuiltinKernel { op: Op::NegL2, isa: Isa::detect() };
    let mut row = vec![0.0f32; 12];
    row[3] = f32::INFINITY;
    assert_eq!(run_kernel(&negl2, &vec![0.0; 12], &row, 12)[0], f32::NEG_INFINITY);
    assert_eq!(run_kernel(&negl2, &vec![1.0; 12], &vec![1.0; 12], 12)[0], 0.0);
}

#[test]
fn kernels_reject_inconsistent_shapes_with_nan() {
    let k = BuiltinKernel { op: Op::Dot, isa: Isa::detect() };
    let mut out = vec![0.0; 2];
    k.score(&[1.0, 2.0], &[1.0, 2.0, 3.0], 2, &mut out);
    assert_eq!(out[0], 5.0);
    assert!(out[1].is_nan());
    k.score(&[1.0], &[1.0, 2.0], 2, &mut out);
    assert!(out.iter().all(|v| v.is_nan()));
    let mut empty = vec![9.0; 3];
    k.score(&[], &[], 0, &mut empty);
    assert_eq!(empty, vec![0.0; 3]);
}

struct ConstKernel(f32);

impl CpuScoreKernel for ConstKernel {
    fn score(&self, _query: &[f32], _rows: &[f32], _dim: usize, out: &mut [f32]) {
        out.fill(self.0);
    }
}

#[test]
fn kernel_register_registers_requests_and_evicts() {
    let register = CpuKernelRegister::with_builtin();
    let now = Instant::now();
    assert!(matches!(
        register.register("score_dot", DType::F32, Arc::new(ConstKernel(0.0))),
        Err(CpuError::InvalidConfig(_))
    ));
    register.register("score_dot", DType::F16, Arc::new(ConstKernel(1.0))).unwrap();
    assert!(matches!(
        register.request("missing", DType::F32, now),
        Err(CpuError::NoKernel { ref name, dtype: DType::F32 }) if name == "missing"
    ));
    let ttl = Duration::from_secs(30);
    let held = register.request("score_dot", DType::F32, now).unwrap();
    let released = register.request("score_negl2", DType::F32, now).unwrap();
    drop(released);
    assert!(register.evict_expired(now + ttl - Duration::from_millis(1), ttl).is_empty());
    assert_eq!(register.evict_expired(now + ttl, ttl), vec!["kernel:score_negl2:F32".to_string()]);
    assert!(register.evict_expired(now + ttl * 10, ttl).is_empty());
    let again = register.request("score_dot", DType::F32, now + ttl).unwrap();
    assert!(Arc::ptr_eq(&held, &again));
    drop((held, again));
    let later = now + ttl * 3;
    assert_eq!(register.evict_expired(later, ttl), vec!["kernel:score_dot:F32".to_string()]);
    let recreated = register.request("score_dot", DType::F32, later).unwrap();
    let mut out = [0.0];
    recreated.score(&[1.0, 2.0], &[3.0, 4.0], 2, &mut out);
    assert_eq!(out[0], 11.0);
    let custom = register.request("score_dot", DType::F16, later).unwrap();
    custom.score(&[], &[], 0, &mut out);
    assert_eq!(out[0], 1.0);
}

// ---------------------------------------------------------------------------------------------
// Core collaborators and construction
// ---------------------------------------------------------------------------------------------

#[test]
fn lane_factory_and_path_policy() {
    let lane = CpuLaneFactory
        .create(&LaneConfig::preset(LaneKind::Insert, DeviceHandle::Cpu))
        .unwrap();
    assert_eq!((lane.kind(), lane.device()), (LaneKind::Insert, DeviceHandle::Cpu));
    assert!(lane.stream().is_none());
    let caps = precision_capabilities(DeviceKind::Cpu);
    let select = |kind| {
        CpuPathPolicy.select(
            &ResourceRequirements::new(kind, l040::a::resource_requirements::ExecutionEnvironment::Any),
            &caps,
            DeviceHandle::Cpu,
        )
    };
    let path = |kernel: &str| ComputePath {
        kernel: kernel.to_string(),
        dtype: DType::F32,
        variant: Variant { dim: None, tile: None },
        lane: LaneKind::Query,
    };
    assert_eq!(select(ResourceKind::Scorer), Some(path("score_dot")));
    assert_eq!(select(ResourceKind::Encoder), Some(path("encoder")));
    assert_eq!(select(ResourceKind::CrossEncoder), Some(path("encoder")));
    assert_eq!(select(ResourceKind::Tokenizer), None);
    assert_eq!(select(ResourceKind::Index), None);
}

#[test]
fn new_builds_a_cpu_core_and_loads_nothing() {
    let (deps, _) = deps(registry_with(&[]));
    let sub = CpuSubengine::new(config(), deps).unwrap();
    let expected = precision_capabilities(DeviceKind::Cpu);
    let mut fp32 = CapabilitySet::new();
    fp32.add(axes::operands(), values::fp32());
    fp32.add(axes::scoring(), values::fp32());
    assert_eq!(expected, fp32);
    assert_eq!(
        Subengine::capabilities(&*sub),
        vec![(QueueKind::Cpu, expected.clone()), (QueueKind::Shared, expected)]
    );
    assert_eq!(Subengine::name(&*sub), "cpu0");
    assert_eq!(Subengine::offers(&*sub), config().offers);
    assert_eq!(sub.default_dtype(), DType::F32);
    assert_eq!(sub.topology().lane_count(), 4);
    assert_eq!(sub.core().device(), DeviceHandle::Cpu);
    assert!(!sub.core().is_active());
    for kind in [ResourceKind::Tokenizer, ResourceKind::Normalizer, ResourceKind::Encoder, ResourceKind::Scorer, ResourceKind::Index] {
        assert!(!sub.is_loaded(kind, None));
    }
}

#[test]
fn new_validates_configuration_first_error_wins() {
    let models = registry_with(&[]);
    let mut cfg = config();
    cfg.name = String::new();
    assert!(matches!(
        CpuSubengine::new(cfg, deps(models.clone()).0),
        Err(CpuError::InvalidConfig(_))
    ));

    let mut cfg = config();
    cfg.extra_capabilities.add(axes::scoring(), values::fp16());
    assert!(matches!(
        CpuSubengine::new(cfg, deps(models.clone()).0),
        Err(CpuError::Subengine(SubengineError::InvalidConfig(_)))
    ));

    let mut cfg = config();
    cfg.shared_capabilities = Some(CapabilitySet::new().with(axes::scoring(), values::int8()));
    assert!(matches!(
        CpuSubengine::new(cfg, deps(models).0),
        Err(CpuError::Subengine(SubengineError::InvalidConfig(_)))
    ));
}

#[test]
fn extra_and_shared_capabilities_are_merged_into_the_core() {
    let mut cfg = config();
    cfg.extra_capabilities = CapabilitySet::new().with(axes::scoring(), values::int8());
    let sub = sub_with(cfg, registry_with(&[]));
    let caps = Subengine::capabilities(&*sub);
    assert!(caps[0].1.contains(&axes::scoring(), &values::int8()));
    assert!(caps[0].1.contains(&axes::scoring(), &values::fp32()));
    assert!(caps[0].1.is_within(&cpu_reference()));
    assert_eq!(caps[1].1, caps[0].1);

    let mut cfg = config();
    cfg.extra_capabilities = CapabilitySet::new().with(axes::scoring(), values::int8());
    cfg.shared_capabilities = Some(precision_capabilities(DeviceKind::Cpu));
    let sub = sub_with(cfg, registry_with(&[]));
    let caps = Subengine::capabilities(&*sub);
    assert_eq!(caps[1].1, precision_capabilities(DeviceKind::Cpu));
}

#[test]
fn scorers_are_adopted_only_for_available_metrics() {
    let sub = sub_with(config(), registry_with(&[Metric::Dot]));
    assert!(sub.core().scorer(Metric::Dot).is_some());
    assert!(sub.core().scorer(Metric::NegL2Sq).is_none());
    let sub = sub_with(config(), registry_with(&[Metric::Dot, Metric::NegL2Sq]));
    assert!(sub.core().scorer(Metric::NegL2Sq).is_some());
    assert!(sub.is_loaded(ResourceKind::Scorer, None));
}

#[test]
fn trait_methods_delegate_and_activation_only_creates_lanes() {
    let (deps, registrations) = deps(registry_with(&[Metric::Dot]));
    let sub = CpuSubengine::new(config(), deps).unwrap();
    Subengine::start(&*sub).unwrap();
    Subengine::start(&*sub).unwrap();
    assert!(!sub.core().is_active());
    sub.announce(&ResourceRequirements::new(
        ResourceKind::Scorer,
        l040::a::resource_requirements::ExecutionEnvironment::Any,
    ));
    assert_eq!(sub.core().pending_announcements().len(), 1);
    Subengine::activate(&*sub).unwrap();
    Subengine::activate(&*sub).unwrap();
    assert!(sub.core().is_active());
    assert!(sub.core().pending_announcements().is_empty());
    assert_eq!(registrations.entries_of(QueueKind::Cpu, ResourceKind::Scorer).len(), 1);
    assert_eq!(registrations.entries_of(QueueKind::Shared, ResourceKind::Scorer).len(), 1);
    let lanes = &registrations.entries_of(QueueKind::Cpu, ResourceKind::Scorer)[0].lanes;
    let kinds: Vec<LaneKind> = lanes.iter().map(|(k, _)| *k).collect();
    assert_eq!(kinds, vec![LaneKind::Query, LaneKind::Insert, LaneKind::Update, LaneKind::CopyH2D]);
    let ran = sub.core().with_lane(LaneKind::CopyH2D, |lane| lane.stream().is_none()).unwrap();
    assert!(ran);
    assert!(!sub.is_loaded(ResourceKind::Encoder, None));
}

#[test]
fn kinds_outside_the_offers_are_never_loaded() {
    let mut cfg = config();
    cfg.offers = vec![ResourceKind::Scorer];
    let sub = sub_with(cfg, registry_with(&[Metric::Dot]));
    assert!(sub.is_loaded(ResourceKind::Scorer, None));
    assert!(!sub.is_loaded(ResourceKind::Index, None));
    assert!(!sub.is_loaded(ResourceKind::Tokenizer, None));
    assert!(!sub.is_loaded(ResourceKind::Encoder, Some(&ModelId::new("m"))));
}

// ---------------------------------------------------------------------------------------------
// Models, resources and lifecycle
// ---------------------------------------------------------------------------------------------

fn e5_dir() -> Option<PathBuf> {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("model/multilingual-e5-small");
    (dir.join("model.safetensors").exists() && dir.join("tokenizer.json").exists()).then_some(dir)
}

fn setup(descriptor: ModelDescriptor) -> ModelSetup {
    ModelSetup {
        descriptor: Arc::new(descriptor),
        normalization: NormalizationParams::default(),
        external_normalizer: None,
    }
}

fn missing_model_setup() -> ModelSetup {
    setup(ModelDescriptor::e5_small_default(Path::new("/nonexistent/cpu_subengine_model"), DType::F16))
}

#[test]
fn register_model_loads_nothing_and_rejects_duplicates() {
    let sub = sub();
    let model = ModelId::new("e5");
    sub.register_model(model.clone(), missing_model_setup()).unwrap();
    assert!(matches!(
        sub.register_model(model.clone(), missing_model_setup()),
        Err(CpuError::AlreadyRegistered(ref m)) if *m == model
    ));
    assert_eq!(sub.encoders.state("enc:e5"), Some(ResourceState::Uninitialized));
    assert_eq!(sub.tokenizers.state("tok:e5"), Some(ResourceState::Uninitialized));
    assert!(!sub.is_loaded(ResourceKind::Encoder, Some(&model)));
    assert!(!sub.is_loaded(ResourceKind::Tokenizer, None));
    assert!(!sub.is_loaded(ResourceKind::Normalizer, Some(&model)));
    let lane = cpu_lane();
    let ctx = CpuContext::new(&sub, &lane);
    assert!(matches!(ctx.encoder(&ModelId::new("other")), Err(CpuError::UnknownModel(_))));
    assert!(matches!(ctx.tokenizer(&ModelId::new("other")), Err(CpuError::UnknownModel(_))));
    assert!(matches!(ctx.preprocessor(&ModelId::new("other")), Err(CpuError::UnknownModel(_))));
}

#[test]
fn failed_loads_report_the_cause_and_then_back_off() {
    let sub = sub();
    let model = ModelId::new("e5");
    sub.register_model(model.clone(), missing_model_setup()).unwrap();
    let lane = cpu_lane();
    let ctx = CpuContext::new(&sub, &lane);
    assert!(matches!(ctx.encoder(&model), Err(CpuError::Load(_))));
    assert!(matches!(ctx.encoder(&model), Err(CpuError::Backoff { .. })));
    assert!(matches!(ctx.tokenizer(&model), Err(CpuError::Load(_))));
    assert!(matches!(ctx.tokenizer(&model), Err(CpuError::Backoff { .. })));
    assert!(!sub.is_loaded(ResourceKind::Encoder, Some(&model)));
    assert!(!sub.is_loaded(ResourceKind::Tokenizer, Some(&model)));
}

struct Lowercaser;

impl TextNormalizer for Lowercaser {
    fn normalize<'a>(&self, text: &'a str) -> Result<Cow<'a, str>, NormalizeError> {
        if text.contains("fail") {
            return Err(NormalizeError("rejected".to_string()));
        }
        Ok(Cow::Owned(text.to_lowercase()))
    }

    fn performed_steps(&self) -> NormalizationParams {
        NormalizationParams { lowercase: true, ..NormalizationParams::default() }
    }
}

#[test]
fn preprocessing_uses_the_model_normalizer() {
    let sub = sub();
    let plain = ModelId::new("plain");
    let lower = ModelId::new("lower");
    sub.register_model(plain.clone(), missing_model_setup()).unwrap();
    let mut with_external = missing_model_setup();
    with_external.external_normalizer = Some(Arc::new(Lowercaser));
    sub.register_model(lower.clone(), with_external).unwrap();
    let lane = cpu_lane();
    let ctx = CpuContext::new(&sub, &lane);
    assert!(matches!(ctx.preprocess(&plain, "Hello").unwrap(), Cow::Borrowed("Hello")));
    assert_eq!(ctx.preprocess(&lower, "Hello").unwrap(), "hello");
    assert_eq!(ctx.preprocessor(&lower).unwrap().apply("ABC").unwrap(), "abc");
    assert!(matches!(ctx.preprocess(&lower, "fail"), Err(CpuError::Normalize(ref m)) if m == "rejected"));
    assert!(matches!(ctx.preprocess(&ModelId::new("x"), "t"), Err(CpuError::UnknownModel(_))));
    assert!(sub.encoders.state("enc:lower").is_some());
    assert_eq!(sub.encoders.state("enc:plain"), Some(ResourceState::Uninitialized));
}

#[test]
fn tokenizer_loads_lazily_once_and_is_evicted_after_the_ram_ttl() {
    let Some(dir) = e5_dir() else { return };
    let sub = sub();
    let model = ModelId::new("e5");
    let mut model_setup = setup(ModelDescriptor::e5_small_default(&dir, DType::F16));
    model_setup.normalization = NormalizationParams { lowercase: true, ..NormalizationParams::default() };
    model_setup.external_normalizer = Some(Arc::new(Lowercaser));
    sub.register_model(model.clone(), model_setup).unwrap();
    let lane = cpu_lane();
    let ctx = CpuContext::new(&sub, &lane);
    assert!(!sub.is_loaded(ResourceKind::Tokenizer, Some(&model)));

    let lease = ctx.tokenizer(&model).unwrap();
    assert!(lease.handle().normalization().lowercase == false);
    assert!(lease.handle().max_seq_len() > 0);
    let first = Arc::clone(lease.handle());
    lease.release(Instant::now());
    assert!(sub.is_loaded(ResourceKind::Tokenizer, Some(&model)));
    assert!(sub.is_loaded(ResourceKind::Tokenizer, None));
    assert!(sub.is_loaded(ResourceKind::Normalizer, Some(&model)));
    assert!(!sub.is_loaded(ResourceKind::Encoder, Some(&model)));
    assert!(Arc::ptr_eq(&first, ctx.tokenizer(&model).unwrap().handle()));

    let base = Instant::now();
    assert!(sub.evict_expired(base + Duration::from_secs(10)).is_empty());
    drop(first);
    let evicted = sub.evict_expired(base + Duration::from_secs(31));
    assert_eq!(evicted, vec!["tok:e5".to_string()]);
    assert!(!sub.is_loaded(ResourceKind::Tokenizer, Some(&model)));
    assert!(!sub.is_loaded(ResourceKind::Normalizer, None));
    ctx.tokenizer(&model).unwrap();
    assert!(sub.is_loaded(ResourceKind::Normalizer, Some(&model)));
}

#[test]
fn live_leases_and_kernels_in_use_are_not_evicted() {
    let Some(dir) = e5_dir() else { return };
    let sub = sub();
    let model = ModelId::new("e5");
    sub.register_model(model.clone(), setup(ModelDescriptor::e5_small_default(&dir, DType::F16))).unwrap();
    let lane = cpu_lane();
    let ctx = CpuContext::new(&sub, &lane);
    let lease = ctx.tokenizer(&model).unwrap();
    let kernel = sub.kernels().request("score_dot", DType::F32, Instant::now()).unwrap();
    let released = sub.kernels().request("score_negl2", DType::F32, Instant::now()).unwrap();
    drop(released);
    let far = Instant::now() + Duration::from_secs(3600);
    assert_eq!(sub.evict_expired(far), vec!["kernel:score_negl2:F32".to_string()]);
    assert!(sub.is_loaded(ResourceKind::Tokenizer, Some(&model)));
    drop((lease, kernel));
    let mut evicted = sub.evict_expired(far);
    evicted.sort();
    assert_eq!(evicted, vec!["kernel:score_dot:F32".to_string(), "tok:e5".to_string()]);
}

#[test]
fn concurrent_first_access_loads_once() {
    let Some(dir) = e5_dir() else { return };
    let sub = sub();
    let model = ModelId::new("e5");
    sub.register_model(model.clone(), setup(ModelDescriptor::e5_small_default(&dir, DType::F16))).unwrap();
    let barrier = Arc::new(Barrier::new(6));
    let handles: Vec<_> = (0..6)
        .map(|_| {
            let (sub, model, barrier) = (Arc::clone(&sub), model.clone(), Arc::clone(&barrier));
            thread::spawn(move || {
                barrier.wait();
                let lane = cpu_lane();
                let lease = CpuContext::new(&sub, &lane).tokenizer(&model).unwrap();
                Arc::clone(lease.handle())
            })
        })
        .collect();
    let loaded: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();
    assert!(loaded.iter().all(|t| Arc::ptr_eq(t, &loaded[0])));
}

// ---------------------------------------------------------------------------------------------
// Index manager
// ---------------------------------------------------------------------------------------------

#[test]
fn attach_is_lazy_and_opens_on_first_access() {
    let dir = TempDir::new();
    let sub = sub();
    let id = IndexId::new("idx");
    let path = dir.file("index.parquet");
    sub.manager().attach(spec("idx", path.clone(), header(DIM))).unwrap();
    assert!(matches!(
        sub.manager().attach(spec("idx", path.clone(), header(DIM))),
        Err(CpuError::AlreadyAttached(ref i)) if *i == id
    ));
    assert!(!path.exists());
    let residency = sub.manager().residency(&id).unwrap();
    assert!(residency.read_direct(EntryId(1)).is_none());
    assert!(sub.manager().residency(&IndexId::new("nope")).is_none());
    assert!(matches!(sub.manager().index(&IndexId::new("nope")), Err(CpuError::UnknownIndex(_))));

    let first = sub.manager().index(&id).unwrap();
    let second = sub.manager().index(&id).unwrap();
    assert!(Arc::ptr_eq(&first, &second));
    assert_eq!(first.read(EntryId(1)), Payload::default());
    assert!(residency.read_direct(EntryId(1)).is_some());
    let ctx_lane = cpu_lane();
    let ctx = CpuContext::new(&sub, &ctx_lane);
    assert!(Arc::ptr_eq(&ctx.index(&id).unwrap(), &first));
}

#[test]
fn open_failures_are_not_cached() {
    let dir = TempDir::new();
    let sub = sub();
    let id = IndexId::new("idx");
    let path = dir.file("index.parquet");
    std::fs::write(&path, b"definitely not a parquet file").unwrap();
    sub.manager().attach(spec("idx", path.clone(), header(DIM))).unwrap();
    let residency = sub.manager().residency(&id).unwrap();
    for _ in 0..2 {
        match sub.manager().index(&id) {
            Err(CpuError::IndexOpen(text)) => assert!(text.contains("index.parquet"), "{text}"),
            other => panic!("unexpected {:?}", other.map(|_| ())),
        }
    }
    assert!(residency.read_direct(EntryId(1)).is_none());
    std::fs::remove_file(&path).unwrap();
    let index = sub.manager().index(&id).unwrap();
    assert!(index.stamps().is_empty());
    assert!(residency.read_direct(EntryId(1)).is_some());
}

#[test]
fn header_mismatch_is_reported_as_open_error() {
    let dir = TempDir::new();
    let path = dir.file("index.parquet");
    write_index(&path, &header(DIM), &[(1, vec![0.5; DIM])]);
    let sub = sub();
    sub.manager().attach(spec("idx", path, header(DIM * 2))).unwrap();
    assert!(matches!(sub.manager().index(&IndexId::new("idx")), Err(CpuError::IndexOpen(_))));
}

#[test]
fn concurrent_first_access_shares_one_index_instance() {
    let dir = TempDir::new();
    let path = dir.file("index.parquet");
    let rows: Vec<(u64, Vec<f32>)> = (1..=20).map(|i| (i, vec![i as f32 / 20.0; DIM])).collect();
    write_index(&path, &header(DIM), &rows);
    let sub = sub();
    let id = IndexId::new("idx");
    sub.manager().attach(spec("idx", path, header(DIM))).unwrap();
    let barrier = Arc::new(Barrier::new(8));
    let handles: Vec<_> = (0..8)
        .map(|_| {
            let (sub, id, barrier) = (Arc::clone(&sub), id.clone(), Arc::clone(&barrier));
            thread::spawn(move || {
                barrier.wait();
                sub.manager().index(&id).unwrap()
            })
        })
        .collect();
    let indexes: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();
    assert!(indexes.iter().all(|i| Arc::ptr_eq(i, &indexes[0])));
    assert_eq!(indexes[0].stamps().len(), 20);
    let from_residency = sub.manager().residency(&id).unwrap().read_direct(EntryId(3)).unwrap();
    assert_eq!(from_residency, indexes[0].read(EntryId(3)));
}

struct CountingAdvisor(Mutex<Vec<(u64, u64, Advice)>>);

impl MemoryAdvisor for CountingAdvisor {
    fn advise(&self, offset: u64, len: u64, advice: Advice) -> Result<(), String> {
        lock(&self.0).push((offset, len, advice));
        Ok(())
    }
}

#[test]
fn idle_ranges_are_marked_cold_without_evicting_anything() {
    let advisor = Arc::new(CountingAdvisor(Mutex::new(Vec::new())));
    let sub = sub();
    let mut with_advisor = spec("idx", PathBuf::from("/nonexistent/idx.parquet"), header(DIM));
    with_advisor.advisor = Some(advisor.clone());
    with_advisor.idle_after = Duration::from_secs(10);
    sub.manager().attach(with_advisor).unwrap();
    sub.manager().attach(spec("quiet", PathBuf::from("/nonexistent/q.parquet"), header(DIM))).unwrap();
    let residency = sub.manager().residency(&IndexId::new("idx")).unwrap();
    residency.secure_residency(ByteRange { first: EntryId(1), count: 2, offset: 100, len: 50 });
    residency.secure_residency(ByteRange { first: EntryId(3), count: 2, offset: 200, len: 50 });
    assert_eq!(lock(&advisor.0).len(), 2);
    assert_eq!(sub.manager().mark_idle_cold(Instant::now()), 0);
    let later = Instant::now() + Duration::from_secs(11);
    assert!(sub.evict_expired(later).is_empty());
    let cold: Vec<_> = lock(&advisor.0).iter().filter(|(_, _, a)| *a == Advice::Cold).cloned().collect();
    assert_eq!(cold.len(), 2);
    assert_eq!(sub.manager().mark_idle_cold(later), 0);
}

// ---------------------------------------------------------------------------------------------
// Scoring
// ---------------------------------------------------------------------------------------------

fn attached(rows: &[(u64, Vec<f32>)]) -> (TempDir, Arc<CpuSubengine>, IndexId) {
    let dir = TempDir::new();
    let path = dir.file("index.parquet");
    write_index(&path, &header(DIM), rows);
    let sub = sub();
    sub.manager().attach(spec("idx", path, header(DIM))).unwrap();
    (dir, sub, IndexId::new("idx"))
}

fn request<'a>(id: &'a IndexId, metric: Metric, query: &'a [f32], entries: &'a [EntryId]) -> CpuScoreRequest<'a> {
    CpuScoreRequest { index: id, metric, query, entries }
}

#[test]
fn scoring_reads_the_index_and_keeps_request_order() {
    let mut rng = Lcg(3);
    let rows: Vec<(u64, Vec<f32>)> = (1..=6).map(|i| (i, rng.vector(DIM))).collect();
    let (_dir, sub, id) = attached(&rows);
    let lane = cpu_lane();
    let ctx = CpuContext::new(&sub, &lane);
    let query = rng.vector(DIM);
    let entries = [EntryId(4), EntryId(99), EntryId(2), EntryId(4), EntryId(6), EntryId(77), EntryId(1), EntryId(99)];
    for metric in [Metric::Dot, Metric::NegL2Sq] {
        let result = ctx.score(&request(&id, metric, &query, &entries)).unwrap();
        let order: Vec<u64> = result.scored.iter().map(|s| s.entry.0).collect();
        assert_eq!(order, vec![4, 2, 6, 1]);
        assert_eq!(result.absent, vec![EntryId(99), EntryId(77)]);
        assert!(result.malformed.is_empty());
        for scored in &result.scored {
            let row = f16_round(&rows[scored.entry.0 as usize - 1].1);
            let expected = match metric {
                Metric::Dot => dot_ref(&query, &row),
                Metric::NegL2Sq => negl2_ref(&query, &row),
            };
            assert_close(scored.score, expected);
        }
    }
    let empty = ctx.score(&request(&id, Metric::Dot, &query, &[])).unwrap();
    assert_eq!(empty, CpuScoreResult::default());
}

#[test]
fn higher_scores_are_better_for_both_metrics() {
    let rows = vec![(1, vec![1.0; DIM]), (2, vec![0.0; DIM]), (3, vec![-1.0; DIM])];
    let (_dir, sub, id) = attached(&rows);
    let lane = cpu_lane();
    let ctx = CpuContext::new(&sub, &lane);
    let query = vec![1.0f32; DIM];
    let entries = [EntryId(1), EntryId(2), EntryId(3)];
    let by = |metric| -> Vec<f32> {
        ctx.score(&request(&id, metric, &query, &entries)).unwrap().scored.iter().map(|s| s.score).collect()
    };
    let dot = by(Metric::Dot);
    assert!(dot[0] > dot[1] && dot[1] > dot[2]);
    let negl2 = by(Metric::NegL2Sq);
    assert_eq!(negl2[0], 0.0);
    assert!(negl2[0] > negl2[1] && negl2[1] > negl2[2]);
}

#[test]
fn scoring_validates_in_order() {
    let rows = vec![(1, vec![0.5; DIM])];
    let (_dir, sub, id) = attached(&rows);
    let lane = cpu_lane();
    let ctx = CpuContext::new(&sub, &lane);
    let unknown = IndexId::new("nope");
    let query = vec![1.0f32; DIM];
    let entries = [EntryId(1)];
    assert!(matches!(
        ctx.score(&request(&unknown, Metric::Dot, &[], &entries)),
        Err(CpuError::UnknownIndex(_))
    ));
    assert!(matches!(
        ctx.score(&request(&id, Metric::Dot, &[], &entries)),
        Err(CpuError::QueryDimMismatch { query: 0, row_bytes }) if row_bytes == DIM * 2
    ));
    assert!(matches!(
        ctx.score(&request(&id, Metric::Dot, &query[..DIM - 1], &entries)),
        Err(CpuError::QueryDimMismatch { query, .. }) if query == DIM - 1
    ));
    assert!(ctx.score(&request(&id, Metric::Dot, &query, &entries)).is_ok());

    let dir = TempDir::new();
    let broken = dir.file("broken.parquet");
    std::fs::write(&broken, b"garbage").unwrap();
    sub.manager().attach(spec("broken", broken, header(DIM))).unwrap();
    assert!(matches!(
        ctx.score(&request(&IndexId::new("broken"), Metric::Dot, &[], &entries)),
        Err(CpuError::IndexOpen(_))
    ));
}

#[test]
fn scoring_processes_more_rows_than_one_block() {
    let mut rng = Lcg(5);
    let total = SCORE_BLOCK_ROWS * 2 + 37;
    let rows: Vec<(u64, Vec<f32>)> = (1..=total as u64).map(|i| (i, rng.vector(DIM))).collect();
    let (_dir, sub, id) = attached(&rows);
    let lane = cpu_lane();
    let ctx = CpuContext::new(&sub, &lane);
    let query = rng.vector(DIM);
    let entries: Vec<EntryId> = (1..=total as u64).rev().map(EntryId).collect();
    let result = ctx.score(&request(&id, Metric::Dot, &query, &entries)).unwrap();
    assert_eq!(result.scored.len(), total);
    for (scored, expected_id) in result.scored.iter().zip((1..=total as u64).rev()) {
        assert_eq!(scored.entry, EntryId(expected_id));
        assert_close(scored.score, dot_ref(&query, &f16_round(&rows[expected_id as usize - 1].1)));
    }
}

#[test]
fn malformed_and_absent_entries_are_separated() {
    let sub = sub();
    let id = IndexId::new("idx");
    sub.manager().attach(spec("idx", PathBuf::from("/nonexistent/idx.parquet"), header(DIM))).unwrap();
    let good = f16_bytes(&vec![1.0; DIM]);
    inject(
        &sub,
        &id,
        MockIndex::new(vec![
            (1, good.clone()),
            (2, vec![0u8; 3]),
            (3, good.iter().chain(&[0u8, 0u8]).copied().collect()),
            (4, good.clone()),
        ]),
    );
    let lane = cpu_lane();
    let ctx = CpuContext::new(&sub, &lane);
    let entries = [EntryId(2), EntryId(1), EntryId(5), EntryId(3), EntryId(4)];
    let result = ctx.score(&request(&id, Metric::Dot, &vec![1.0; DIM], &entries)).unwrap();
    assert_eq!(result.malformed, vec![EntryId(2), EntryId(3)]);
    assert_eq!(result.absent, vec![EntryId(5)]);
    assert_eq!(
        result.scored,
        vec![
            CpuScored { entry: EntryId(1), score: DIM as f32 },
            CpuScored { entry: EntryId(4), score: DIM as f32 },
        ]
    );
}

#[test]
fn scoring_passes_nan_and_inf_of_the_index_through() {
    let sub = sub();
    let id = IndexId::new("idx");
    sub.manager().attach(spec("idx", PathBuf::from("/nonexistent/idx.parquet"), header(DIM))).unwrap();
    let mut with_nan = vec![1.0f32; DIM];
    with_nan[2] = f32::NAN;
    let mut with_inf = vec![1.0f32; DIM];
    with_inf[0] = f32::INFINITY;
    inject(&sub, &id, MockIndex::new(vec![(1, f16_bytes(&with_nan)), (2, f16_bytes(&with_inf))]));
    let lane = cpu_lane();
    let ctx = CpuContext::new(&sub, &lane);
    let result = ctx
        .score(&request(&id, Metric::Dot, &vec![1.0; DIM], &[EntryId(1), EntryId(2)]))
        .unwrap();
    assert!(result.scored[0].score.is_nan());
    assert_eq!(result.scored[1].score, f32::INFINITY);
}

// ---------------------------------------------------------------------------------------------
// Runner and lane resource
// ---------------------------------------------------------------------------------------------

struct ScoreJob {
    id: u32,
    index: IndexId,
    query: Vec<f32>,
    entries: Vec<EntryId>,
    key: Option<BatchKey>,
    log: Arc<Mutex<Vec<(u32, thread::ThreadId, Result<CpuScoreResult, String>)>>>,
}

impl BatchPayload for ScoreJob {
    fn batch_key(&self) -> Option<BatchKey> {
        self.key.clone()
    }

    fn sequences(&self) -> &[Vec<u32>] {
        &[]
    }
}

impl CpuPayload for ScoreJob {
    fn execute_batch(batch: ExecBatch<Self>, ctx: &CpuContext<'_>) {
        for job in batch.tasks {
            let result = ctx
                .score(&request(&job.index, Metric::Dot, &job.query, &job.entries))
                .map_err(|e| e.to_string());
            lock(&job.log).push((job.id, thread::current().id(), result));
        }
    }
}

struct JobMeta {
    key: Option<BatchKey>,
    requirements: ResourceRequirements,
}

impl BatchMeta for JobMeta {
    fn batch_key(&self) -> Option<BatchKey> {
        self.key.clone()
    }

    fn seq_lengths(&self) -> &[usize] {
        &[]
    }
}

impl HasResourceRequirements for JobMeta {
    fn resource_requirements(&self) -> &ResourceRequirements {
        &self.requirements
    }
}

type JobLog = Arc<Mutex<Vec<(u32, thread::ThreadId, Result<CpuScoreResult, String>)>>>;

fn job(id: u32, index: &IndexId, log: &JobLog, key: Option<&str>) -> ScoreJob {
    ScoreJob {
        id,
        index: index.clone(),
        query: vec![1.0; DIM],
        entries: vec![EntryId(1), EntryId(2)],
        key: key.map(|k| BatchKey(k.to_string())),
        log: Arc::clone(log),
    }
}

#[test]
fn runner_executes_the_whole_batch_synchronously_on_the_calling_thread() {
    let rows = vec![(1, vec![1.0; DIM]), (2, vec![0.5; DIM])];
    let (_dir, sub, id) = attached(&rows);
    let runner = cpu_runner::<ScoreJob>(Arc::clone(&sub));
    let log: JobLog = Arc::new(Mutex::new(Vec::new()));
    let mut batch = ExecBatch {
        tasks: vec![job(1, &id, &log, Some("k")), job(2, &id, &log, Some("k"))],
        spans: Vec::new(),
        batches: None,
    };
    let mut lane = cpu_lane();
    assert_eq!(runner.run(&mut lane, &mut batch), Launch::Finished);
    assert!(batch.tasks.is_empty());
    let log = lock(&log);
    assert_eq!(log.iter().map(|(id, _, _)| *id).collect::<Vec<_>>(), vec![1, 2]);
    for (_, thread_id, result) in log.iter() {
        assert_eq!(*thread_id, thread::current().id());
        let scored = &result.as_ref().unwrap().scored;
        assert_eq!(scored.len(), 2);
        assert_close(scored[0].score, DIM as f64);
        assert_close(scored[1].score, DIM as f64 * 0.5);
    }
}

#[test]
fn lane_resource_has_one_lane_per_topology_lane_and_runs_claims() {
    let rows = vec![(1, vec![1.0; DIM]), (2, vec![0.5; DIM])];
    let (_dir, sub, id) = attached(&rows);
    let topology = NotificationTopology::new(1, Arc::new(|| {}));
    let own: Arc<Queue<JobMeta, ScoreJob, (), ()>> = Arc::new(Queue::new());
    let make_cfg = |device| LaneResourceConfig {
        own: Arc::clone(&own),
        shared: None,
        filter: Arc::new(|_: &JobMeta| true),
        completion: None,
        limit: BatchLimit::Tasks(4),
        notifier: topology.notifier(0).unwrap(),
        kind: ResourceKind::Scorer,
        device,
    };
    let runner = cpu_runner::<ScoreJob>(Arc::clone(&sub));
    assert!(matches!(
        sub.lane_resource(make_cfg(DeviceHandle::Gpu(0)), Arc::clone(&runner)),
        Err(CpuError::InvalidConfig(_))
    ));
    let resource = sub.lane_resource(make_cfg(DeviceHandle::Cpu), runner).unwrap();
    assert_eq!(resource.lane_count(), sub.topology().lane_count());
    assert_eq!(resource.idle_lanes(), 4);

    let log: JobLog = Arc::new(Mutex::new(Vec::new()));
    let meta = || JobMeta {
        key: Some(BatchKey("k".to_string())),
        requirements: ResourceRequirements::new(
            ResourceKind::Scorer,
            l040::a::resource_requirements::ExecutionEnvironment::Any,
        ),
    };
    let tickets: Vec<_> = (1..=3).map(|i| own.push(meta(), job(i, &id, &log, Some("k")))).collect();

    let registry = Arc::new(ResourceRegistry::new(config().lifecycle));
    struct Once(Mutex<Option<Arc<LaneResource<JobMeta, ScoreJob>>>>);
    impl l060::a::resource_lifecycle::ResourceLoader<LaneResource<JobMeta, ScoreJob>> for Once {
        fn load(&self) -> Result<LaneResource<JobMeta, ScoreJob>, String> {
            let arc = lock(&self.0).take().ok_or("loaded twice")?;
            Arc::try_unwrap(arc).map_err(|_| "resource is shared".to_string())
        }
    }
    registry
        .register("lanes".to_string(), MemoryClass::Ram, Arc::new(Once(Mutex::new(Some(resource)))))
        .unwrap();
    let now = Instant::now();
    let InitRequest::Started(ticket) = registry.request_init("lanes", 1, now) else {
        panic!("load not started");
    };
    ticket.run(now).unwrap();
    let InitRequest::Ready(lease) = registry.request_init("lanes", 1, now) else {
        panic!("not ready");
    };
    let resource = Arc::clone(lease.handle());
    let claim = own.take().expect("queue is empty");
    resource.execute(claim, lease);
    assert!(tickets.iter().all(|t| t.state() == State::Succeeded));
    let mut ids: Vec<u32> = lock(&log).iter().map(|(id, _, _)| *id).collect();
    ids.sort();
    assert_eq!(ids, vec![1, 2, 3]);
    assert_eq!(resource.idle_lanes(), 4);
}

// ---------------------------------------------------------------------------------------------
// Encoding (real model)
// ---------------------------------------------------------------------------------------------

#[derive(Debug)]
struct MemBackend(Mutex<Vec<u8>>);

impl RegionBackend for MemBackend {
    fn len(&self) -> usize {
        lock(&self.0).len()
    }

    fn read_raw(&self, offset: usize, dst: &mut [u8]) -> Result<(), AccessError> {
        dst.copy_from_slice(&lock(&self.0)[offset..offset + dst.len()]);
        Ok(())
    }

    fn write_raw(&self, offset: usize, src: &[u8]) -> Result<(), AccessError> {
        lock(&self.0)[offset..offset + src.len()].copy_from_slice(src);
        Ok(())
    }
}

fn tensor(shape: &[usize], dtype: DType) -> TensorRef {
    let layout = Layout::contiguous(shape, dtype);
    let region = Region::new(Arc::new(MemBackend(Mutex::new(vec![0; layout.byte_len()]))));
    TensorRef::new(region, layout).unwrap()
}

fn embed(ctx: &CpuContext<'_>, model: &ModelId, texts: &[&str]) -> Vec<Vec<f32>> {
    let tokens = ctx.tokenizer(model).unwrap();
    let encodings: Vec<_> = texts
        .iter()
        .map(|t| {
            let text = ctx.preprocess(model, t).unwrap();
            tokens.handle().tokenizer().encode(text.as_ref(), true).unwrap()
        })
        .collect();
    let seq = encodings.iter().map(|e| e.get_ids().len()).max().unwrap();
    let pad = tokens.handle().pad_token_id();
    let mut ids = Vec::new();
    let mut mask = Vec::new();
    for e in &encodings {
        for i in 0..seq {
            ids.push(e.get_ids().get(i).copied().unwrap_or(pad));
            mask.push(u32::from(i < e.get_ids().len()));
        }
    }
    let rows = texts.len();
    let input = tensor(&[2, rows, seq], DType::U32);
    let bytes: Vec<u8> = ids.iter().chain(&mask).flat_map(|v| v.to_le_bytes()).collect();
    input.region().write(0, &bytes).unwrap();
    let encoder = ctx.encoder(model).unwrap();
    let dimension = encoder.handle().encoder().output_dim();
    let output = tensor(&[rows, dimension], DType::F16);
    encoder.release(Instant::now());
    ctx.encode(model, &input, &output, rows, seq).unwrap();
    let mut raw = vec![0u8; rows * dimension * 2];
    output.region().read(output.offset_bytes(), &mut raw).unwrap();
    raw.chunks_exact(dimension * 2)
        .map(|row| row.chunks_exact(2).map(|c| f16::from_le_bytes([c[0], c[1]]).to_f32()).collect())
        .collect()
}

#[test]
fn encoding_and_scoring_work_end_to_end_with_the_real_model() {
    let Some(dir) = e5_dir() else { return };
    let models = Arc::new(ModelRegistry::with_builtin());
    let sub = sub_with(config(), models);
    let model = ModelId::new("e5");
    let descriptor = ModelDescriptor::e5_small_default(&dir, DType::F16);
    let dimension = descriptor.dimension;
    sub.register_model(model.clone(), setup(descriptor.clone())).unwrap();
    let lane = cpu_lane();
    let ctx = CpuContext::new(&sub, &lane);
    assert!(!sub.is_loaded(ResourceKind::Encoder, Some(&model)));

    let passages = [
        "passage: Rust is a systems programming language focused on safety.",
        "passage: Bananas are yellow fruits that grow in tropical regions.",
        "passage: Paris is the capital and largest city of France.",
    ];
    let vectors = embed(&ctx, &model, &passages);
    assert!(sub.is_loaded(ResourceKind::Encoder, Some(&model)));
    assert!(sub.is_loaded(ResourceKind::Encoder, None));
    for v in &vectors {
        assert_eq!(v.len(), dimension);
        assert!(v.iter().all(|x| x.is_finite()));
        let norm: f32 = v.iter().map(|x| x * x).sum::<f32>().sqrt();
        assert!((norm - 1.0).abs() < 0.01, "norm {norm}");
    }

    let dir_index = TempDir::new();
    let path = dir_index.file("index.parquet");
    let head = header(dimension);
    let rows: Vec<(u64, Vec<f32>)> = vectors.iter().enumerate().map(|(i, v)| (i as u64 + 1, v.clone())).collect();
    write_index(&path, &head, &rows);
    let id = IndexId::new("docs");
    sub.manager().attach(spec("docs", path, head)).unwrap();

    let query = embed(&ctx, &model, &["query: which language is built for memory safety?"]).remove(0);
    let entries = [EntryId(1), EntryId(2), EntryId(3)];
    let result = ctx.score(&request(&id, Metric::Dot, &query, &entries)).unwrap();
    let best = result
        .scored
        .iter()
        .max_by(|a, b| a.score.total_cmp(&b.score))
        .unwrap();
    assert_eq!(best.entry, EntryId(1));

    let evicted = sub.evict_expired(Instant::now() + Duration::from_secs(31));
    assert!(evicted.contains(&"enc:e5".to_string()));
    assert!(evicted.contains(&"tok:e5".to_string()));
    assert!(!sub.is_loaded(ResourceKind::Encoder, Some(&model)));
}

#[test]
fn encode_reports_precheck_violations_without_running() {
    let Some(dir) = e5_dir() else { return };
    let sub = sub();
    let model = ModelId::new("e5");
    sub.register_model(model.clone(), setup(ModelDescriptor::e5_small_default(&dir, DType::F16))).unwrap();
    let lane = cpu_lane();
    let ctx = CpuContext::new(&sub, &lane);
    let input = tensor(&[2, 1, 4], DType::U32);
    let wrong_output = tensor(&[1, 3], DType::F16);
    let result = ctx.encode(&model, &input, &wrong_output, 1, 4);
    assert!(matches!(result, Err(CpuError::Encoder(EncoderError::Incompatible { .. }))));
    let mut untouched = vec![0u8; 6];
    wrong_output.region().read(0, &mut untouched).unwrap();
    assert_eq!(untouched, vec![0u8; 6]);
    assert!(matches!(
        ctx.encode(&ModelId::new("other"), &input, &wrong_output, 1, 4),
        Err(CpuError::UnknownModel(_))
    ));
}

// ---------------------------------------------------------------------------------------------
// Calibration
// ---------------------------------------------------------------------------------------------

#[test]
fn robust_statistics_map_to_the_shared_functions() {
    let stats = RobustStatistics;
    let samples = [10.0, 11.0, 9.0, 10.5, 9.5, 10.2];
    assert_eq!(stats.estimate(&samples, EstimatorKind::Median), statistics::median(&samples));
    assert_eq!(stats.estimate(&samples, EstimatorKind::Minimum), Some(9.0));
    assert_eq!(
        stats.dispersion(&samples, DispersionKind::Cv),
        Some(statistics::dispersion(&samples, Dispersion::CoefficientOfVariation))
    );
    assert_eq!(
        stats.dispersion(&samples, DispersionKind::RangeOverMedian),
        Some(statistics::dispersion(&samples, Dispersion::TrimmedRange))
    );
    assert_eq!(stats.dispersion(&[1.0], DispersionKind::Cv), None);
    assert_eq!(stats.dispersion(&[], DispersionKind::RangeOverMedian), None);
    assert_eq!(stats.estimate(&[], EstimatorKind::Median), None);
}

struct ProbeBackend {
    threads: Mutex<Vec<Option<String>>>,
    gate: Option<Arc<(Mutex<bool>, Condvar)>>,
    panic: bool,
}

impl ProbeBackend {
    fn new(gate: Option<Arc<(Mutex<bool>, Condvar)>>, panic: bool) -> Arc<Self> {
        Arc::new(Self { threads: Mutex::new(Vec::new()), gate, panic })
    }

    fn note(&self) {
        lock(&self.threads).push(thread::current().name().map(str::to_string));
        if self.panic {
            panic!("probe failed");
        }
        if let Some(gate) = &self.gate {
            let mut open = lock(&gate.0);
            while !*open {
                open = gate.1.wait(open).unwrap_or_else(PoisonError::into_inner);
            }
        }
    }
}

impl CalibrationBackend for ProbeBackend {
    fn deviation(&self, _c: &Combination, _seq_lens: &[usize]) -> Result<f64, ProbeError> {
        self.note();
        Ok(0.0)
    }

    fn time(
        &self,
        _c: &Combination,
        _seq_lens: &[usize],
        _window: Duration,
        _chunk_budget: Duration,
    ) -> Result<Duration, ProbeError> {
        self.note();
        Ok(Duration::from_micros(100))
    }
}

fn quick_calibration() -> CalibrationConfig {
    CalibrationConfig { backoff: Duration::ZERO, ..CalibrationConfig::default() }
}

#[test]
fn calibration_runs_on_the_low_priority_pool_and_does_not_block() {
    let sub = sub();
    let gate = Arc::new((Mutex::new(false), Condvar::new()));
    let backend = ProbeBackend::new(Some(Arc::clone(&gate)), false);
    let descriptor = Arc::new(ModelDescriptor::e5_small_repo_default(DType::F32));
    let started = Instant::now();
    let future = sub
        .calibrate(Arc::clone(&descriptor), backend.clone(), quick_calibration())
        .unwrap();
    assert!(started.elapsed() < Duration::from_secs(5));
    assert!(!future.is_done());
    *lock(&gate.0) = true;
    gate.1.notify_all();
    let report = future.wait().unwrap().unwrap();
    assert_eq!(report.target.subengine, "cpu0");
    assert_eq!(report.target.model_id, descriptor.model_id);
    assert!(report.product_size >= 1);
    let threads = lock(&backend.threads);
    assert!(!threads.is_empty());
    for name in threads.iter() {
        let name = name.as_deref().unwrap_or_default();
        assert!(name.starts_with("cpu0-calib"), "ran on {name}");
    }
}

#[test]
fn calibration_errors_and_panics_reach_the_future() {
    let sub = sub();
    let backend = ProbeBackend::new(None, true);
    let descriptor = Arc::new(ModelDescriptor::e5_small_repo_default(DType::F32));
    let panicked = sub.calibrate(Arc::clone(&descriptor), backend, quick_calibration()).unwrap();
    assert!(matches!(
        panicked.wait(),
        Err(l010::background_pool::JobError::Panicked(ref m)) if m.contains("probe failed")
    ));

    let mut invalid = ModelDescriptor::e5_small_repo_default(DType::F32);
    invalid.dimension = 0;
    let failed = sub
        .calibrate(Arc::new(invalid), ProbeBackend::new(None, false), quick_calibration())
        .unwrap();
    assert!(matches!(failed.wait().unwrap(), Err(CalibrationError::Model(_))));
    let again = sub
        .calibrate(descriptor, ProbeBackend::new(None, false), quick_calibration())
        .unwrap();
    assert!(again.wait().unwrap().is_ok());
}
