use super::*;
use std::any::Any;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::path::Path;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Barrier, OnceLock};
use std::thread;
use std::time::Duration;
use crate::resident::_d::_c::_b::_a::_9::a::model_registry::ModelRegistry;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::a::kernel_register::{
    KernelRegister, ArchLevel, CodeImage, CodeKind, CompileRequest, CompilerError,
    KernelCompiler, KernelFunction, KernelSource, LoadFailure, ModuleLoader,
};
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::a::library_adapter::library_kernel_source;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::a::region_handle::Generation;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::a::arenas::ArenaKind;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::a::model_descriptor::{
    DeviceClass, ModelDescriptor,
};
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::_1::capability_model::{
    axes, values,
};
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::_1::device_lane_event::{
    GpuEventBackend, StreamBackend,
};
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::_1::target_env::ComputeCapability;
static_assertions::assert_impl_all!(SubengineCore : Send, Sync);
static_assertions::assert_impl_all!(QueueRegistrations : Send, Sync);
const fn assert_send_sync_static<T: Send + Sync + 'static>() {}
const _: () = assert_send_sync_static::<SubengineError>();
static_assertions::assert_impl_all!(Registration : Send, Sync);
#[derive(Debug)]
struct MockStream {
    live: Arc<AtomicUsize>,
}
impl Drop for MockStream {
    fn drop(&mut self) {
        self.live.fetch_sub(1, Ordering::SeqCst);
    }
}
impl StreamBackend for MockStream {
    fn record_event(&self) -> Result<Box<dyn GpuEventBackend>, String> {
        Err("mock stream".to_string())
    }
    fn notify_on_complete(&self, _f: Box<dyn FnOnce() + Send>) -> Result<bool, String> {
        Ok(false)
    }
}
struct TestFactory {
    calls: AtomicUsize,
    created: AtomicUsize,
    live_streams: Arc<AtomicUsize>,
    stream: AtomicBool,
    fail: Mutex<Option<LaneKind>>,
    delay_ms: AtomicUsize,
}
impl TestFactory {
    fn new(stream: bool) -> Arc<Self> {
        Arc::new(Self {
            calls: AtomicUsize::new(0),
            created: AtomicUsize::new(0),
            live_streams: Arc::new(AtomicUsize::new(0)),
            stream: AtomicBool::new(stream),
            fail: Mutex::new(None),
            delay_ms: AtomicUsize::new(0),
        })
    }
}
impl LaneFactory for TestFactory {
    fn create(&self, cfg: &LaneConfig) -> Result<Lane, String> {
        self.calls.fetch_add(1, Ordering::SeqCst);
        let delay = self.delay_ms.load(Ordering::SeqCst);
        if delay > 0 {
            thread::sleep(Duration::from_millis(delay as u64));
        }
        if *lock(&self.fail) == Some(cfg.kind) {
            return Err("boom".to_string());
        }
        let stream: Option<Box<dyn StreamBackend>> = if self
            .stream
            .load(Ordering::SeqCst) && cfg.device.kind() == DeviceKind::Gpu
        {
            self.live_streams.fetch_add(1, Ordering::SeqCst);
            Some(
                Box::new(MockStream {
                    live: Arc::clone(&self.live_streams),
                }),
            )
        } else {
            None
        };
        self.created.fetch_add(1, Ordering::SeqCst);
        Ok(Lane::new(cfg.clone(), stream))
    }
}
struct TestPolicy {
    calls: AtomicUsize,
    answer: ComputePath,
}
impl PathPolicy for TestPolicy {
    fn select(
        &self,
        req: &ResourceRequirements,
        _caps: &CapabilitySet,
        _device: DeviceHandle,
    ) -> Option<ComputePath> {
        self.calls.fetch_add(1, Ordering::SeqCst);
        (req.kind() == ResourceKind::Scorer).then(|| self.answer.clone())
    }
}
struct StubCompiler;
impl KernelCompiler for StubCompiler {
    fn compile(&self, _req: &CompileRequest) -> Result<CodeImage, CompilerError> {
        Ok(CodeImage {
            kind: CodeKind::Ptx,
            bytes: vec![0],
        })
    }
}
#[derive(Debug)]
struct MockFunction;
impl KernelFunction for MockFunction {
    fn symbol(&self) -> &str {
        "test_entry"
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}
struct StubLoader {
    capability: Option<ComputeCapability>,
}
impl ModuleLoader for StubLoader {
    fn device_capability(&self) -> Option<ComputeCapability> {
        self.capability
    }
    fn load(
        &self,
        _image: &CodeImage,
        _symbol: &str,
    ) -> Result<Arc<dyn KernelFunction>, LoadFailure> {
        Ok(Arc::new(MockFunction))
    }
}
fn cpu_register() -> Arc<KernelRegister> {
    Arc::new(
        KernelRegister::new(
            DeviceHandle::Cpu,
            Arc::new(StubCompiler),
            Arc::new(StubLoader { capability: None }),
            library_kernel_source(),
        ),
    )
}
fn gpu_register() -> Arc<KernelRegister> {
    let register = KernelRegister::new(
        DeviceHandle::Gpu(0),
        Arc::new(StubCompiler),
        Arc::new(StubLoader {
            capability: Some(ArchLevel::Sm75.capability()),
        }),
        library_kernel_source(),
    );
    register
        .register_source(KernelSource {
            name: "test_kernel",
            body: "",
            entry: "test_entry",
            dtypes: &[DType::F32],
            precompiled: None,
        })
        .unwrap();
    Arc::new(register)
}
fn load_encoder() -> Arc<LoadedEncoder> {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("model/multilingual-e5-small");
    let desc = ModelDescriptor::e5_small_default(&dir, DType::F32);
    Arc::new(
        ModelRegistry::with_builtin()
            .load_encoder(&desc, DeviceClass::Cpu)
            .expect("e5-small loads on the CPU"),
    )
}
fn real_encoder() -> Arc<LoadedEncoder> {
    static ENCODER: OnceLock<Arc<LoadedEncoder>> = OnceLock::new();
    Arc::clone(ENCODER.get_or_init(load_encoder))
}
fn own_caps() -> CapabilitySet {
    CapabilitySet::new()
        .with(axes::operands(), values::fp32())
        .with(axes::scoring(), values::fp32())
}
fn shared_caps() -> CapabilitySet {
    CapabilitySet::new().with(axes::operands(), values::fp32())
}
fn all_offers() -> Vec<ResourceKind> {
    vec![
        ResourceKind::Encoder, ResourceKind::Scorer, ResourceKind::Index,
        ResourceKind::Tokenizer,
    ]
}
fn config(name: &str, device: DeviceHandle) -> SubengineConfig {
    SubengineConfig {
        name: name.to_string(),
        device,
        own_queue: match device {
            DeviceHandle::Cpu => QueueKind::Cpu,
            DeviceHandle::Gpu(_) => QueueKind::Gpu,
        },
        own_capabilities: own_caps(),
        shared_capabilities: shared_caps(),
        offers: all_offers(),
        lanes: vec![
            LaneConfig::preset(LaneKind::Query, device),
            LaneConfig::preset(LaneKind::Insert, device),
        ],
    }
}
fn scorer_path() -> ComputePath {
    ComputePath {
        kernel: "test_kernel".to_string(),
        dtype: DType::F32,
        variant: Variant::default(),
        lane: LaneKind::Query,
    }
}
struct Rig {
    core: SubengineCore,
    regs: Arc<QueueRegistrations>,
    factory: Arc<TestFactory>,
    policy: Arc<TestPolicy>,
}
fn rig_with(cfg: SubengineConfig, regs: Arc<QueueRegistrations>) -> Rig {
    let gpu = cfg.device.kind() == DeviceKind::Gpu;
    let factory = TestFactory::new(gpu);
    let policy = Arc::new(TestPolicy {
        calls: AtomicUsize::new(0),
        answer: scorer_path(),
    });
    let kernels = if gpu { gpu_register() } else { cpu_register() };
    let core = SubengineCore::new(
            cfg,
            factory.clone(),
            policy.clone(),
            kernels,
            regs.clone(),
        )
        .unwrap_or_else(|e| panic!("valid configuration rejected: {e}"));
    Rig { core, regs, factory, policy }
}
fn rig(cfg: SubengineConfig) -> Rig {
    rig_with(cfg, Arc::new(QueueRegistrations::new()))
}
fn cpu_rig() -> Rig {
    rig(config("cpu0", DeviceHandle::Cpu))
}
fn gpu_rig() -> Rig {
    rig(config("gpu0", DeviceHandle::Gpu(0)))
}
fn config_error(cfg: SubengineConfig) -> SubengineError {
    let result = SubengineCore::new(
        cfg,
        TestFactory::new(false),
        Arc::new(TestPolicy {
            calls: AtomicUsize::new(0),
            answer: scorer_path(),
        }),
        cpu_register(),
        Arc::new(QueueRegistrations::new()),
    );
    match result {
        Err(e) => e,
        Ok(_) => panic!("configuration was accepted"),
    }
}
fn invalid_text(e: SubengineError) -> String {
    match e {
        SubengineError::InvalidConfig(text) => text,
        other => panic!("unexpected error {other:?}"),
    }
}
fn summary(
    regs: &[Registration],
) -> Vec<(ResourceKind, Option<String>, Option<String>)> {
    regs.iter()
        .map(|r| {
            (
                r.kind,
                r.model.as_ref().map(|m| m.as_str().to_string()),
                r.index.as_ref().map(|i| i.as_str().to_string()),
            )
        })
        .collect()
}
fn segment(index: &str, offset: u64, rows: usize) -> IndexSegment {
    IndexSegment {
        index: IndexId::new(index),
        region: RegionHandle::new(ArenaKind::IndexSegments, offset, 4096, Generation(0))
            .unwrap(),
        layout: Layout::contiguous(&[rows, 4], DType::F32),
    }
}
#[test]
fn rejects_shared_and_mismatching_own_queue() {
    let mut cfg = config("x", DeviceHandle::Cpu);
    cfg.own_queue = QueueKind::Shared;
    assert!(invalid_text(config_error(cfg)).contains("own queue"));
    let mut cfg = config("x", DeviceHandle::Cpu);
    cfg.own_queue = QueueKind::Gpu;
    assert!(matches!(config_error(cfg), SubengineError::InvalidConfig(_)));
    let mut cfg = config("x", DeviceHandle::Gpu(0));
    cfg.own_queue = QueueKind::Cpu;
    assert!(matches!(config_error(cfg), SubengineError::InvalidConfig(_)));
}
#[test]
fn rejects_shared_beyond_own() {
    let mut cfg = config("x", DeviceHandle::Cpu);
    cfg.own_capabilities = shared_caps();
    cfg.shared_capabilities = own_caps();
    assert!(invalid_text(config_error(cfg)).contains("shared"));
}
#[test]
fn rejects_capabilities_beyond_the_reference() {
    let mut cfg = config("x", DeviceHandle::Cpu);
    cfg.own_capabilities = own_caps().with(axes::operands(), "fp128");
    assert!(invalid_text(config_error(cfg)).contains("reference"));
    let mut cfg = config("x", DeviceHandle::Gpu(0));
    cfg.own_capabilities = own_caps().with(axes::scoring(), values::int8_vnni());
    assert!(invalid_text(config_error(cfg)).contains("reference"));
}
#[test]
fn tuning_axes_are_not_bounded_by_the_reference() {
    for device in [DeviceHandle::Cpu, DeviceHandle::Gpu(0)] {
        let mut cfg = config("x", device);
        cfg.own_capabilities = own_caps()
            .with(axes::batch_size(), "1")
            .with(axes::batch_size(), "4096")
            .with(axes::tile_size(), "64")
            .with(axes::thread_count(), "12");
        cfg.shared_capabilities = own_caps();
        let result = SubengineCore::new(
            cfg,
            TestFactory::new(false),
            Arc::new(TestPolicy {
                calls: AtomicUsize::new(0),
                answer: scorer_path(),
            }),
            cpu_register(),
            Arc::new(QueueRegistrations::new()),
        );
        assert!(result.is_ok(), "{device:?}");
    }
    let mut cfg = config("x", DeviceHandle::Gpu(0));
    cfg.own_capabilities = own_caps()
        .with(axes::batch_size(), "8")
        .with(axes::operands(), "fp128");
    assert!(invalid_text(config_error(cfg)).contains("reference"));
}
#[test]
fn rejects_foreign_and_duplicate_lanes() {
    let mut cfg = config("x", DeviceHandle::Cpu);
    cfg.lanes.push(LaneConfig::preset(LaneKind::Update, DeviceHandle::Gpu(0)));
    assert!(invalid_text(config_error(cfg)).contains("configured for device"));
    let mut cfg = config("x", DeviceHandle::Cpu);
    cfg.lanes.push(LaneConfig::preset(LaneKind::Query, DeviceHandle::Cpu));
    assert!(invalid_text(config_error(cfg)).contains("twice"));
}
#[test]
fn rejects_empty_name() {
    let cfg = config("", DeviceHandle::Cpu);
    assert!(invalid_text(config_error(cfg)).contains("name"));
}
#[test]
fn first_configuration_error_wins() {
    let mut cfg = config("", DeviceHandle::Cpu);
    cfg.own_queue = QueueKind::Shared;
    cfg.lanes.push(LaneConfig::preset(LaneKind::Query, DeviceHandle::Cpu));
    assert!(invalid_text(config_error(cfg)).contains("own queue"));
    let mut cfg = config("", DeviceHandle::Cpu);
    cfg.own_capabilities = shared_caps();
    cfg.shared_capabilities = own_caps();
    cfg.lanes.push(LaneConfig::preset(LaneKind::Query, DeviceHandle::Cpu));
    assert!(invalid_text(config_error(cfg)).contains("shared"));
    let mut cfg = config("", DeviceHandle::Cpu);
    cfg.lanes.push(LaneConfig::preset(LaneKind::Query, DeviceHandle::Cpu));
    assert!(invalid_text(config_error(cfg)).contains("twice"));
}
#[test]
fn new_and_start_create_nothing() {
    let r = cpu_rig();
    assert!(! r.core.is_active());
    assert_eq!(r.core.device(), DeviceHandle::Cpu);
    assert_eq!(r.core.name(), "cpu0");
    assert!(r.core.start().is_ok());
    assert!(r.core.start().is_ok());
    assert_eq!(r.factory.calls.load(Ordering::SeqCst), 0);
    assert!(! r.core.is_active());
    assert!(r.regs.entries(QueueKind::Cpu).is_empty());
}
#[test]
fn capabilities_list_own_and_shared() {
    let r = cpu_rig();
    let caps = r.core.capabilities();
    assert!(
        caps == vec![(QueueKind::Cpu, own_caps()), (QueueKind::Shared, shared_caps())]
    );
    assert!(caps == r.core.capabilities());
    let g = gpu_rig();
    assert!(g.core.capabilities() [0].0 == QueueKind::Gpu);
}
#[test]
fn empty_shared_capabilities_are_omitted() {
    let mut cfg = config("cpu0", DeviceHandle::Cpu);
    cfg.shared_capabilities = CapabilitySet::new();
    let r = rig(cfg);
    assert!(r.core.capabilities() == vec![(QueueKind::Cpu, own_caps())]);
    r.core.adopt_scorer();
    r.core.activate().unwrap();
    assert_eq!(r.regs.entries(QueueKind::Cpu).len(), 1);
    assert!(r.regs.entries(QueueKind::Shared).is_empty());
}
#[test]
fn offers_are_unchanged_and_without_duplicates() {
    let mut cfg = config("cpu0", DeviceHandle::Cpu);
    cfg.offers = vec![ResourceKind::Scorer, ResourceKind::Encoder, ResourceKind::Scorer];
    let r = rig(cfg);
    assert_eq!(r.core.offers(), vec![ResourceKind::Scorer, ResourceKind::Encoder]);
    r.core.activate().unwrap();
    assert_eq!(r.core.offers(), vec![ResourceKind::Scorer, ResourceKind::Encoder]);
}
#[test]
fn announce_only_records_deduplicated() {
    let r = cpu_rig();
    let a = ResourceRequirements::new(ResourceKind::Encoder, ExecutionEnvironment::Any)
        .with_model(ModelId::new("m"));
    let b = ResourceRequirements::new(
        ResourceKind::Scorer,
        ExecutionEnvironment::CpuOnly,
    );
    r.core.announce(&a);
    r.core.announce(&b);
    r.core.announce(&a);
    assert!(r.core.pending_announcements() == vec![a, b]);
    assert_eq!(r.factory.calls.load(Ordering::SeqCst), 0);
    assert!(r.regs.entries(QueueKind::Cpu).is_empty());
    assert!(! r.core.is_active());
    r.core.activate().unwrap();
    assert!(r.core.pending_announcements().is_empty());
}
#[test]
fn unmatched_announcements_are_not_an_error() {
    let r = cpu_rig();
    r.core
        .announce(
            &ResourceRequirements::new(ResourceKind::Encoder, ExecutionEnvironment::Any)
                .with_model(ModelId::new("nobody-owns-this")),
        );
    assert!(r.core.activate().is_ok());
    assert!(r.regs.entries(QueueKind::Cpu).is_empty());
}
#[test]
fn activate_creates_each_lane_once_and_is_idempotent() {
    let r = cpu_rig();
    assert_eq!(
        r.core.with_lane(LaneKind::Query, | _ | ()), Err(SubengineError::NotActive)
    );
    r.core.activate().unwrap();
    r.core.activate().unwrap();
    assert!(r.core.is_active());
    assert_eq!(r.factory.calls.load(Ordering::SeqCst), 2);
    assert_eq!(r.core.with_lane(LaneKind::Insert, | l | l.kind()), Ok(LaneKind::Insert));
    assert_eq!(
        r.core.with_lane(LaneKind::CopyH2D, | _ | ()),
        Err(SubengineError::UnknownLane(LaneKind::CopyH2D))
    );
}
#[test]
fn activation_never_registers_without_resources() {
    let r = cpu_rig();
    r.core.activate().unwrap();
    assert!(r.regs.entries(QueueKind::Cpu).is_empty());
    assert!(r.regs.entries(QueueKind::Shared).is_empty());
}
#[test]
fn gpu_lane_without_stream_fails_and_retries() {
    let r = gpu_rig();
    r.factory.stream.store(false, Ordering::SeqCst);
    let err = r.core.activate().unwrap_err();
    assert_eq!(
        err, SubengineError::LaneWithoutStream { kind : LaneKind::Query } .to_string()
    );
    assert!(err.contains("ohne Stream"));
    assert!(! r.core.is_active());
    assert_eq!(
        r.core.with_lane(LaneKind::Query, | _ | ()), Err(SubengineError::NotActive)
    );
    r.factory.stream.store(true, Ordering::SeqCst);
    r.core.activate().unwrap();
    assert!(r.core.is_active());
    assert!(r.core.with_lane(LaneKind::Query, | l | l.stream().is_some()).unwrap());
}
#[test]
fn lane_creation_error_leaves_nothing_behind() {
    let r = gpu_rig();
    *lock(&r.factory.fail) = Some(LaneKind::Insert);
    let err = r.core.activate().unwrap_err();
    assert_eq!(err, "Lane Insert nicht erstellbar: boom");
    assert!(! r.core.is_active());
    assert_eq!(r.factory.live_streams.load(Ordering::SeqCst), 0);
    assert!(r.regs.entries(QueueKind::Gpu).is_empty());
    *lock(&r.factory.fail) = None;
    r.core.activate().unwrap();
    assert_eq!(r.factory.live_streams.load(Ordering::SeqCst), 2);
}
#[test]
fn failed_activation_registers_nothing() {
    let r = gpu_rig();
    r.core.adopt_scorer();
    *lock(&r.factory.fail) = Some(LaneKind::Query);
    assert!(r.core.activate().is_err());
    assert!(r.regs.entries(QueueKind::Gpu).is_empty());
    assert!(r.regs.entries(QueueKind::Shared).is_empty());
}
#[test]
fn concurrent_activations_are_serialized() {
    let r = gpu_rig();
    r.factory.delay_ms.store(20, Ordering::SeqCst);
    r.core.adopt_scorer();
    thread::scope(|s| {
        for _ in 0..8 {
            s.spawn(|| {
                r.core.activate().unwrap();
                assert!(r.core.is_active());
            });
        }
    });
    assert_eq!(r.factory.created.load(Ordering::SeqCst), 2);
    assert_eq!(r.regs.entries(QueueKind::Gpu).len(), 1);
    assert_eq!(r.regs.entries(QueueKind::Shared).len(), 1);
}
#[test]
fn state_queries_do_not_wait_for_activation() {
    let r = gpu_rig();
    r.factory.delay_ms.store(150, Ordering::SeqCst);
    thread::scope(|s| {
        s.spawn(|| r.core.activate().unwrap());
        thread::sleep(Duration::from_millis(40));
        let started = std::time::Instant::now();
        r.core
            .announce(
                &ResourceRequirements::new(
                    ResourceKind::Index,
                    ExecutionEnvironment::Any,
                ),
            );
        assert!(! r.core.is_loaded(ResourceKind::Index, None));
        assert!(! r.core.is_active());
        let _ = r.core.capabilities();
        let _ = r.core.offers();
        assert!(started.elapsed() < Duration::from_millis(100));
    });
    assert!(r.core.is_active());
}
#[test]
fn scorer_registered_at_activation_in_both_queues() {
    let r = cpu_rig();
    r.core.adopt_scorer();
    assert!(r.regs.entries(QueueKind::Cpu).is_empty());
    r.core.activate().unwrap();
    let own = r.regs.entries_of(QueueKind::Cpu, ResourceKind::Scorer);
    let shared = r.regs.entries_of(QueueKind::Shared, ResourceKind::Scorer);
    assert_eq!((own.len(), shared.len()), (1, 1));
    assert_eq!(own[0].subengine, "cpu0");
    assert_eq!(own[0].device, DeviceHandle::Cpu);
    assert!(own[0].capabilities == own_caps());
    assert!(shared[0].capabilities == shared_caps());
    assert!(own[0].model.is_none() && own[0].index.is_none());
    let kinds: Vec<LaneKind> = own[0].lanes.iter().map(|(k, _)| *k).collect();
    assert_eq!(kinds, vec![LaneKind::Query, LaneKind::Insert]);
    assert!(r.regs.entries_of(QueueKind::Cpu, ResourceKind::Encoder).is_empty());
    assert!(r.regs.entries(QueueKind::Gpu).is_empty());
}
#[test]
fn adoption_after_activation_registers_at_once_without_duplicates() {
    let r = cpu_rig();
    r.core.activate().unwrap();
    r.core.adopt_scorer();
    assert_eq!(r.regs.entries(QueueKind::Cpu).len(), 1);
    r.core.adopt_scorer();
    r.core.activate().unwrap();
    assert_eq!(r.regs.entries(QueueKind::Cpu).len(), 1);
    assert_eq!(r.regs.entries(QueueKind::Shared).len(), 1);
}
#[test]
fn registered_lanes_are_the_cores_lanes() {
    let r = cpu_rig();
    r.core.adopt_scorer();
    r.core.activate().unwrap();
    let reg = r.regs.entries(QueueKind::Cpu).remove(0);
    reg.lanes[0].1.lock().unwrap().params_mut()[0] = 9;
    assert_eq!(r.core.with_lane(LaneKind::Query, | l | l.params() [0]), Ok(9));
}
#[test]
fn not_offered_kinds_are_rejected_or_owned_unregistered() {
    let mut cfg = config("cpu0", DeviceHandle::Cpu);
    cfg.offers = vec![ResourceKind::Encoder];
    let r = rig(cfg);
    assert!(
        matches!(r.core.adopt_index_segment(segment("i", 0, 2)),
        Err(SubengineError::Incompatible(_)))
    );
    r.core.adopt_scorer();
    r.core.activate().unwrap();
    assert!(r.regs.entries(QueueKind::Cpu).is_empty());
    assert!(! r.core.is_loaded(ResourceKind::Scorer, None));
    let mut cfg = config("cpu1", DeviceHandle::Cpu);
    cfg.offers = vec![ResourceKind::Scorer];
    let r = rig(cfg);
    assert!(
        matches!(r.core.adopt_encoder(ModelId::new("m"), real_encoder()),
        Err(SubengineError::Incompatible(_)))
    );
    assert!(r.core.encoder(& ModelId::new("m")).is_none());
}
#[test]
fn encoders_are_owned_registered_and_replaced_in_place() {
    let enc = load_encoder();
    let base = Arc::strong_count(&enc);
    let r = cpu_rig();
    let (m1, m2) = (ModelId::new("m1"), ModelId::new("m2"));
    r.core.adopt_encoder(m1.clone(), enc.clone()).unwrap();
    assert!(Arc::ptr_eq(& r.core.encoder(& m1).unwrap(), & enc));
    assert!(r.core.encoder(& m2).is_none());
    assert!(r.regs.entries(QueueKind::Cpu).is_empty());
    r.core.activate().unwrap();
    r.core.adopt_encoder(m2.clone(), enc.clone()).unwrap();
    r.core.adopt_encoder(m1.clone(), enc.clone()).unwrap();
    let own = r.regs.entries_of(QueueKind::Cpu, ResourceKind::Encoder);
    assert_eq!(
        own.iter().map(| x | x.model.clone().unwrap()).collect::< Vec < _ >> (), vec![m1
        .clone(), m2.clone()]
    );
    assert_eq!(r.regs.entries_of(QueueKind::Shared, ResourceKind::Encoder).len(), 2);
    assert!(Arc::strong_count(& enc) > base);
    drop(r);
    assert_eq!(Arc::strong_count(& enc), base);
}
#[test]
fn index_segments_are_owned_per_index_and_replaced_by_region() {
    let r = cpu_rig();
    let i1 = IndexId::new("i1");
    r.core.adopt_index_segment(segment("i1", 0, 2)).unwrap();
    r.core.adopt_index_segment(segment("i1", 4096, 2)).unwrap();
    r.core.adopt_index_segment(segment("i2", 0, 2)).unwrap();
    r.core.adopt_index_segment(segment("i1", 0, 8)).unwrap();
    let segs = r.core.index_segments(&i1);
    assert_eq!(segs.len(), 2);
    assert_eq!(segs[0].layout, Layout::contiguous(& [8, 4], DType::F32));
    assert!(r.core.index_segments(& IndexId::new("none")).is_empty());
    r.core.activate().unwrap();
    r.core.adopt_index_segment(segment("i1", 8192, 1)).unwrap();
    let own = r.regs.entries_of(QueueKind::Cpu, ResourceKind::Index);
    assert_eq!(
        summary(& own), vec![(ResourceKind::Index, None, Some("i1".to_string())),
        (ResourceKind::Index, None, Some("i2".to_string())),]
    );
    assert_eq!(r.core.index_segments(& i1).len(), 3);
}
#[test]
fn is_loaded_follows_ownership_and_never_activates() {
    let r = cpu_rig();
    let m = ModelId::new("m");
    for kind in [
        ResourceKind::Encoder,
        ResourceKind::Scorer,
        ResourceKind::Index,
        ResourceKind::Tokenizer,
    ] {
        assert!(! r.core.is_loaded(kind, None));
    }
    r.core.adopt_encoder(m.clone(), real_encoder()).unwrap();
    r.core.adopt_index_segment(segment("i", 0, 1)).unwrap();
    assert!(r.core.is_loaded(ResourceKind::Encoder, Some(& m)));
    assert!(r.core.is_loaded(ResourceKind::Encoder, None));
    assert!(! r.core.is_loaded(ResourceKind::Encoder, Some(& ModelId::new("other"))));
    assert!(r.core.is_loaded(ResourceKind::Index, None));
    assert!(! r.core.is_loaded(ResourceKind::Tokenizer, None));
    assert!(! r.core.is_active());
    r.core.activate().unwrap();
    assert!(r.core.is_loaded(ResourceKind::Tokenizer, None));
    assert!(! r.core.is_loaded(ResourceKind::Normalizer, None));
    assert!(! r.core.is_loaded(ResourceKind::CrossEncoder, None));
}
#[test]
fn scorer_is_available_when_registered_and_loaded_once_its_kernel_is_resident() {
    let r = gpu_rig();
    r.core.adopt_scorer();
    r.core.activate().unwrap();
    assert_eq!(r.regs.entries_of(QueueKind::Gpu, ResourceKind::Scorer).len(), 1);
    assert!(! r.core.is_loaded(ResourceKind::Scorer, None));
    r.core.kernel(&scorer_path()).unwrap();
    assert!(r.core.is_loaded(ResourceKind::Scorer, None));
}
#[test]
fn scorer_without_adoption_is_not_loaded_even_if_its_kernel_is_resident() {
    let r = gpu_rig();
    r.core.kernel(&scorer_path()).unwrap();
    assert!(! r.core.is_loaded(ResourceKind::Scorer, None));
    assert!(r.regs.entries(QueueKind::Gpu).is_empty());
}
#[test]
fn select_path_checks_environment_before_the_policy() {
    let r = cpu_rig();
    let gpu_only = ResourceRequirements::new(
        ResourceKind::Scorer,
        ExecutionEnvironment::GpuOnly,
    );
    assert_eq!(
        r.core.select_path(& gpu_only), Err(SubengineError::NoPath { kind :
        ResourceKind::Scorer })
    );
    assert_eq!(r.policy.calls.load(Ordering::SeqCst), 0);
    let cpu = ResourceRequirements::new(
        ResourceKind::Scorer,
        ExecutionEnvironment::CpuOnly,
    );
    assert_eq!(r.core.select_path(& cpu), Ok(scorer_path()));
    assert_eq!(r.policy.calls.load(Ordering::SeqCst), 1);
    let index = ResourceRequirements::new(
        ResourceKind::Index,
        ExecutionEnvironment::Any,
    );
    let err = r.core.select_path(&index).unwrap_err();
    assert_eq!(err, SubengineError::NoPath { kind : ResourceKind::Index });
    assert_eq!(err.to_string(), "kein Compute-Pfad für Index");
    assert_eq!(r.policy.calls.load(Ordering::SeqCst), 2);
}
#[test]
fn kernels_come_only_from_the_register() {
    let r = gpu_rig();
    assert!(matches!(r.core.kernel(& scorer_path()), Ok(Kernel::Gpu(_))));
    let unknown = ComputePath {
        kernel: "nope".to_string(),
        ..scorer_path()
    };
    assert_eq!(
        r.core.kernel(& unknown).err(),
        Some(SubengineError::Kernel(KernelError::UnknownKernel("nope".to_string())))
    );
    let bf16 = ComputePath {
        dtype: DType::BF16,
        ..scorer_path()
    };
    assert!(
        matches!(r.core.kernel(& bf16),
        Err(SubengineError::Kernel(KernelError::Unavailable(_))))
    );
}
#[test]
fn one_lane_is_never_used_twice_at_once() {
    let r = gpu_rig();
    r.core.activate().unwrap();
    let current = AtomicUsize::new(0);
    let peak = AtomicUsize::new(0);
    thread::scope(|s| {
        for _ in 0..4 {
            s.spawn(|| {
                r.core
                    .with_lane(
                        LaneKind::Query,
                        |_| {
                            let now = current.fetch_add(1, Ordering::SeqCst) + 1;
                            peak.fetch_max(now, Ordering::SeqCst);
                            thread::sleep(Duration::from_millis(10));
                            current.fetch_sub(1, Ordering::SeqCst);
                        },
                    )
                    .unwrap();
            });
        }
    });
    assert_eq!(peak.load(Ordering::SeqCst), 1);
}
#[test]
fn lanes_of_different_kinds_run_concurrently() {
    let r = gpu_rig();
    r.core.activate().unwrap();
    let barrier = Barrier::new(2);
    thread::scope(|s| {
        for kind in [LaneKind::Query, LaneKind::Insert] {
            let (core, barrier) = (&r.core, &barrier);
            s.spawn(move || {
                core.with_lane(
                        kind,
                        |_| {
                            barrier.wait();
                        },
                    )
                    .unwrap();
            });
        }
    });
}
#[test]
fn lane_state_is_exclusive_and_survives_a_panic() {
    let r = gpu_rig();
    r.core.activate().unwrap();
    r.core
        .with_lane(
            LaneKind::Query,
            |l| {
                l.params_mut()[1] = 5;
                l.ensure_workspace(64);
                l.workspace_mut()[0] = 3;
            },
        )
        .unwrap();
    let caught = catch_unwind(
        AssertUnwindSafe(|| {
            let _ = r
                .core
                .with_lane(
                    LaneKind::Query,
                    |_: &mut Lane| {
                        panic!("closure failure");
                    },
                );
        }),
    );
    assert!(caught.is_err());
    let state = r.core.with_lane(LaneKind::Query, |l| (l.params()[1], l.workspace()[0]));
    assert_eq!(state, Ok((5, 3)));
}
#[test]
fn registrations_outlive_the_core() {
    let r = cpu_rig();
    r.core.adopt_scorer();
    r.core.activate().unwrap();
    let Rig { core, regs, .. } = r;
    let reg = regs.entries(QueueKind::Cpu).remove(0);
    drop(core);
    assert_eq!(reg.lanes.len(), 2);
    assert_eq!(reg.lanes[0].1.lock().unwrap().kind(), LaneKind::Query);
    assert_eq!(regs.entries(QueueKind::Cpu).len(), 1);
}
#[test]
fn sub_engines_share_one_registration_store() {
    let regs = Arc::new(QueueRegistrations::new());
    let a = rig_with(config("cpu0", DeviceHandle::Cpu), regs.clone());
    let b = rig_with(config("gpu0", DeviceHandle::Gpu(0)), regs.clone());
    for r in [&b, &a] {
        r.core.adopt_scorer();
        r.core.activate().unwrap();
    }
    let shared = regs.entries(QueueKind::Shared);
    let names: Vec<&str> = shared.iter().map(|r| r.subengine.as_str()).collect();
    assert_eq!(names, vec!["gpu0", "cpu0"]);
    assert_eq!(regs.entries(QueueKind::Gpu).len(), 1);
    assert_eq!(regs.entries(QueueKind::Cpu).len(), 1);
    assert_eq!(regs.entries(QueueKind::Gpu) [0].device, DeviceHandle::Gpu(0));
}
#[test]
fn equal_inputs_give_equal_registrations() {
    let build = || {
        let r = gpu_rig();
        r.core.adopt_scorer();
        r.core.adopt_index_segment(segment("i", 0, 2)).unwrap();
        r.core.adopt_index_segment(segment("j", 0, 2)).unwrap();
        r.core.activate().unwrap();
        (
            summary(&r.regs.entries(QueueKind::Gpu)),
            summary(&r.regs.entries(QueueKind::Shared)),
        )
    };
    assert_eq!(build(), build());
}
#[test]
fn error_texts_and_trait_error_mapping() {
    assert_eq!(
        SubengineError::InvalidConfig("x".to_string()).to_string(),
        "ungültige Subengine-Konfiguration: x"
    );
    assert_eq!(
        SubengineError::LaneCreation { kind : LaneKind::Update, reason : "r".to_string()
        } .to_string(), "Lane Update nicht erstellbar: r"
    );
    assert_eq!(
        SubengineError::LaneWithoutStream { kind : LaneKind::CopyD2H } .to_string(),
        "GPU-Lane CopyD2H ohne Stream (Default-Stream ist verboten)"
    );
    assert_eq!(SubengineError::NotActive.to_string(), "Subengine nicht aktiviert");
    assert_eq!(
        SubengineError::UnknownLane(LaneKind::Query).to_string(),
        "Lane Query nicht vorhanden"
    );
    let inner = KernelError::UnknownKernel("k".to_string());
    assert_eq!(SubengineError::Kernel(inner.clone()).to_string(), inner.to_string());
    assert_eq!(
        SubengineError::Incompatible("y".to_string()).to_string(),
        "Ressource passt nicht zur Subengine: y"
    );
}
