use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;
use std::time::Duration;
use serde_json::json;
use crate::resident::l130::l120::l110::l100::l090 as l090;
use l090::l080 as l080;
use l080::l070 as l070;
use l070::l060 as l060;
use l060::l050 as l050;
use l050::l040 as l040;
use l040::l030 as l030;
use l030::l020 as l020;
use l020::l010 as l010;
use l010::queue::Queue;
use l020::a::index_contract::{Index, PayloadRef};
use l030::a::reference_index::ReferenceIndex;
use l040::a::resource_requirements::ExecutionEnvironment;
use l040::a::slot_layout::SlotClass;
use l060::b::encoder_build::{build_encoder, BuiltEncoder};
use l070::a::notification_topology::NotificationTopology;
use l090::a::gpu_completion::GpuCompletion;
use l090::a::lane_batching::{BatchKey, BatchLimit, ExecBatch, Launch};
use super::super::fixture::{config, device, shared, shared_with, subengine, MockBackend};
use super::*;
const HIDDEN: usize = 16;
const LAYERS: usize = 2;
const HEADS: usize = 2;
const INTERMEDIATE: usize = 32;
const VOCAB: usize = 50;
const MAX_POS: usize = 32;
const SEQ: usize = 16;
static COUNTER: AtomicUsize = AtomicUsize::new(0);
struct TestDir(PathBuf);
impl TestDir {
    fn new() -> TestDir {
        let n = COUNTER.fetch_add(1, Ordering::SeqCst);
        let path = std::env::temp_dir()
            .join(format!("gpu_subengine_{}_{n}", std::process::id()));
        std::fs::create_dir_all(&path).unwrap();
        TestDir(path)
    }
}
impl Drop for TestDir {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}
fn tensor_shapes() -> Vec<(String, Vec<usize>)> {
    let mut v: Vec<(String, Vec<usize>)> = vec![
        ("embeddings.word_embeddings.weight".into(), vec![VOCAB, HIDDEN]),
        ("embeddings.position_embeddings.weight".into(), vec![MAX_POS, HIDDEN]),
        ("embeddings.token_type_embeddings.weight".into(), vec![2, HIDDEN]),
        ("embeddings.LayerNorm.weight".into(), vec![HIDDEN]),
        ("embeddings.LayerNorm.bias".into(), vec![HIDDEN]),
    ];
    for i in 0..LAYERS {
        let dense = [
            ("attention.self.query", HIDDEN, HIDDEN),
            ("attention.self.key", HIDDEN, HIDDEN),
            ("attention.self.value", HIDDEN, HIDDEN),
            ("attention.output.dense", HIDDEN, HIDDEN),
            ("intermediate.dense", INTERMEDIATE, HIDDEN),
            ("output.dense", HIDDEN, INTERMEDIATE),
        ];
        for (part, out, inp) in dense {
            v.push((format!("encoder.layer.{i}.{part}.weight"), vec![out, inp]));
            v.push((format!("encoder.layer.{i}.{part}.bias"), vec![out]));
        }
        for part in ["attention.output.LayerNorm", "output.LayerNorm"] {
            v.push((format!("encoder.layer.{i}.{part}.weight"), vec![HIDDEN]));
            v.push((format!("encoder.layer.{i}.{part}.bias"), vec![HIDDEN]));
        }
    }
    v
}
fn write_model(dir: &TestDir) {
    let config = json!(
        { "model_type" : "bert", "hidden_size" : HIDDEN, "num_hidden_layers" : LAYERS,
        "num_attention_heads" : HEADS, "intermediate_size" : INTERMEDIATE, "vocab_size" :
        VOCAB, "max_position_embeddings" : MAX_POS, "hidden_act" : "gelu",
        "layer_norm_eps" : 1e-12, "type_vocab_size" : 2 }
    );
    std::fs::write(dir.0.join("config.json"), config.to_string()).unwrap();
    let mut state = 42u64;
    let mut next = move || {
        state = state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        ((state >> 40) as f32 / (1u64 << 24) as f32 - 0.5) * 0.4
    };
    let mut header = String::from("{");
    let mut data: Vec<u8> = Vec::new();
    for (i, (name, shape)) in tensor_shapes().iter().enumerate() {
        let count: usize = shape.iter().product();
        let ln_weight = name.contains("LayerNorm") && name.ends_with(".weight");
        let begin = data.len();
        for _ in 0..count {
            let v = if ln_weight { 1.0 + next() * 0.1 } else { next() };
            data.extend_from_slice(&v.to_le_bytes());
        }
        if i > 0 {
            header.push(',');
        }
        header
            .push_str(
                &format!(
                    "\"{name}\":{{\"dtype\":\"F32\",\"shape\":{shape:?},\"data_offsets\":[{begin},{}]}}",
                    data.len()
                ),
            );
    }
    header.push('}');
    while header.len() % 8 != 0 {
        header.push(' ');
    }
    let mut file = Vec::new();
    file.extend_from_slice(&(header.len() as u64).to_le_bytes());
    file.extend_from_slice(header.as_bytes());
    file.extend_from_slice(&data);
    std::fs::write(dir.0.join("model.safetensors"), file).unwrap();
}
fn descriptor(dir: &TestDir, architecture: &str) -> Arc<ModelDescriptor> {
    let mut desc = ModelDescriptor::e5_small_default(&dir.0, DType::F32);
    desc.dimension = HIDDEN;
    desc.max_seq_len = SEQ;
    desc.architecture = architecture.to_string();
    Arc::new(desc)
}
fn registry(calls: &Arc<AtomicUsize>, source: Arc<ModelDescriptor>) -> ModelRegistry {
    let calls = Arc::clone(calls);
    let mut registry = ModelRegistry::new();
    registry
        .register_architecture(
            "custom",
            Arc::new(move |_d: &ModelDescriptor, _class: DeviceClass| {
                calls.fetch_add(1, Ordering::SeqCst);
                build_encoder(&source, DeviceClass::Cpu)
                    .map(BuiltEncoder::into_encoder)
                    .map_err(|e| e.to_string())
            }),
        );
    registry
}
fn model_cfg() -> GpuConfig {
    let mut cfg = config();
    cfg.lifecycle = LifecycleConfig::new(
            Duration::from_secs(60),
            Duration::from_secs(120),
            Duration::from_secs(5),
        )
        .unwrap();
    cfg
}
struct ModelRig {
    _dir: TestDir,
    sub: Arc<GpuSubengine>,
    desc: Arc<ModelDescriptor>,
    calls: Arc<AtomicUsize>,
}
fn model_rig() -> ModelRig {
    let dir = TestDir::new();
    write_model(&dir);
    let calls = Arc::new(AtomicUsize::new(0));
    let backend = MockBackend::single();
    let sub = GpuSubengine::new(
            device(0),
            Arc::clone(&backend) as Arc<dyn GpuBackend>,
            shared_with(registry(&calls, descriptor(&dir, "bert"))),
            model_cfg(),
        )
        .unwrap();
    let desc = descriptor(&dir, "custom");
    ModelRig {
        _dir: dir,
        sub,
        desc,
        calls,
    }
}
fn model(id: &str) -> ModelId {
    ModelId::new(id)
}
fn started(sub: &GpuSubengine, id: &ModelId, now: Instant) -> LoadTicket<LoadedEncoder> {
    match sub.request_model(id, 1, now).unwrap() {
        InitRequest::Started(ticket) => ticket,
        InitRequest::Ready(_) => panic!("already ready"),
        InitRequest::Loading => panic!("already loading"),
        InitRequest::Backoff { .. } => panic!("in backoff"),
        InitRequest::Unavailable => panic!("unavailable"),
    }
}
fn state(sub: &GpuSubengine, key: &str) -> Option<ResourceState> {
    sub.encoders.state(key)
}
fn scorer_req() -> ResourceRequirements {
    ResourceRequirements::new(ResourceKind::Scorer, ExecutionEnvironment::Any)
}
#[test]
fn new_builds_the_core_and_touches_no_device() {
    let backend = MockBackend::single();
    let sub = subengine(&backend);
    assert!(lock(& backend.streams).is_empty());
    assert!(lock(& backend.reserved).is_empty());
    assert!(lock(& backend.retained).is_empty());
    assert!(! sub.core().is_active());
    assert!(! sub.vram().arenas_ready());
    assert_eq!(sub.name(), "gpu:0");
    assert_eq!(sub.device(), DeviceHandle::Gpu(0));
    assert_eq!(sub.info().ordinal, 0);
    assert_eq!(sub.compute_units(), sub.info().multiprocessors);
    assert_eq!(
        sub.offers(), vec![ResourceKind::Encoder, ResourceKind::Scorer,
        ResourceKind::Index]
    );
}
#[test]
fn capabilities_are_the_detected_ones_without_bf16() {
    let backend = MockBackend::single();
    let sub = subengine(&backend);
    let detected = sub.detected().capabilities.clone();
    assert!(detected.contains(& axes::operands(), & values::fp16()));
    assert!(detected.contains(& axes::operands(), & values::fp32()));
    assert!(detected.contains(& axes::scoring(), & values::int8_dp4a()));
    assert!(
        detected.iter().all(| (_, v) | ! v.as_str().to_lowercase().contains("bf16"))
    );
    let reported = sub.capabilities();
    assert!(reported.iter().any(| (q, _) | * q == QueueKind::Gpu));
    for (_, caps) in &reported {
        assert!(caps.is_within(& detected));
    }
    assert!(lock(& backend.streams).is_empty());
}
#[test]
fn compute_dtype_is_fp16_when_available_and_never_bf16() {
    let backend = MockBackend::single();
    assert_eq!(subengine(& backend).compute_dtype(), DType::F16);
    let mut cfg = config();
    cfg.default_dtype = DType::F32;
    assert_eq!(
        super::super::fixture::subengine_with(& backend, cfg).compute_dtype(), DType::F32
    );
    *lock(&backend.compute_capability) = ComputeCapability {
        major: 5,
        minor: 2,
    };
    let old = subengine(&backend);
    assert!(
        ! old.detected().capabilities.contains(& axes::operands(), & values::fp16())
    );
    assert_eq!(old.compute_dtype(), DType::F32);
}
#[test]
fn new_validates_before_it_touches_the_backend() {
    let backend = MockBackend::single();
    let mut cfg = config();
    cfg.default_dtype = DType::BF16;
    let err = GpuSubengine::new(
            device(0),
            Arc::clone(&backend) as Arc<dyn GpuBackend>,
            shared(),
            cfg,
        )
        .err()
        .unwrap();
    assert!(matches!(err, GpuError::InvalidConfig(_)), "{err:?}");
}
#[test]
fn new_reports_a_failed_capability_detection() {
    let backend = MockBackend::single();
    lock(&backend.no_context).insert(0);
    let err = GpuSubengine::new(
            device(0),
            Arc::clone(&backend) as Arc<dyn GpuBackend>,
            shared(),
            config(),
        )
        .err()
        .unwrap();
    assert!(
        matches!(err, GpuError::Capability(DetectError::NoDeviceContext)), "{err:?}"
    );
}
#[test]
fn lane_work_before_activation_fails_with_not_active() {
    let backend = MockBackend::single();
    let sub = subengine(&backend);
    let err = sub.core().with_lane(LaneKind::Query, |_| ()).unwrap_err();
    assert_eq!(err, SubengineError::NotActive);
}
#[test]
fn start_offers_and_announce_have_no_side_effects() {
    let backend = MockBackend::single();
    let sub = subengine(&backend);
    sub.start().unwrap();
    sub.start().unwrap();
    sub.announce(&scorer_req());
    assert!(lock(& backend.streams).is_empty());
    assert!(lock(& backend.reserved).is_empty());
    assert!(! sub.is_loaded(ResourceKind::Scorer, None));
    assert!(! sub.is_loaded(ResourceKind::Index, None));
}
#[test]
fn activation_reserves_the_arenas_and_creates_five_lanes_with_streams() {
    let backend = MockBackend::single();
    let sub = subengine(&backend);
    sub.activate().unwrap();
    assert!(sub.vram().arenas_ready());
    assert!(sub.core().is_active());
    assert_eq!(lock(& backend.reserved).len(), 6);
    let mut priorities: Vec<Priority> = lock(&backend.streams)
        .iter()
        .map(|(_, p)| *p)
        .collect();
    priorities.sort();
    assert_eq!(
        priorities, vec![Priority::Normal, Priority::Normal, Priority::Normal,
        Priority::Normal, Priority::High]
    );
    for kind in [
        LaneKind::Query,
        LaneKind::Insert,
        LaneKind::Update,
        LaneKind::CopyH2D,
        LaneKind::CopyD2H,
    ] {
        let has_stream = sub
            .core()
            .with_lane(kind, |lane| lane.stream().is_some())
            .unwrap();
        assert!(has_stream, "{kind:?}");
    }
}
#[test]
fn activation_is_idempotent_and_serialized() {
    let backend = MockBackend::single();
    let sub = subengine(&backend);
    thread::scope(|scope| {
        for _ in 0..4 {
            scope.spawn(|| sub.activate().unwrap());
        }
    });
    sub.activate().unwrap();
    assert_eq!(lock(& backend.reserved).len(), 6);
    assert_eq!(lock(& backend.streams).len(), 5);
}
#[test]
fn an_activation_failure_is_not_cached() {
    let backend = MockBackend::single();
    let sub = subengine(&backend);
    backend.free.store(1 << 20, Ordering::SeqCst);
    let err = sub.activate().unwrap_err();
    assert!(err.contains("Budget"), "{err}");
    assert!(! sub.core().is_active());
    assert!(lock(& backend.streams).is_empty());
    assert!(lock(& backend.reserved).is_empty());
    backend.free.store(64 << 20, Ordering::SeqCst);
    sub.activate().unwrap();
    assert!(sub.core().is_active());
}
#[test]
fn a_stream_failure_fails_the_activation_and_a_retry_succeeds() {
    let backend = MockBackend::single();
    let sub = subengine(&backend);
    backend.streams_before_failure.store(2, Ordering::SeqCst);
    let err = sub.activate().unwrap_err();
    assert!(err.contains("stream limit reached"), "{err}");
    assert!(! sub.core().is_active());
    assert!(sub.vram().arenas_ready());
    backend.streams_before_failure.store(usize::MAX, Ordering::SeqCst);
    sub.activate().unwrap();
    assert!(sub.core().is_active());
    assert_eq!(lock(& backend.reserved).len(), 6);
}
#[test]
fn activation_loads_no_model_kernel_or_index() {
    let rig = model_rig();
    rig.sub.register_model(model("m"), Arc::clone(&rig.desc)).unwrap();
    rig.sub.activate().unwrap();
    assert_eq!(rig.calls.load(Ordering::SeqCst), 0);
    assert!(! rig.sub.is_loaded(ResourceKind::Encoder, Some(& model("m"))));
    assert!(! rig.sub.is_loaded(ResourceKind::Encoder, None));
    assert!(! rig.sub.is_loaded(ResourceKind::Scorer, None));
    assert!(! rig.sub.is_loaded(ResourceKind::Index, None));
}
#[test]
fn the_core_selects_fp16_for_scorers_and_nothing_for_encoders() {
    let backend = MockBackend::single();
    let sub = subengine(&backend);
    let path = sub.core().select_path(&scorer_req()).unwrap();
    assert_eq!(path.dtype, DType::F16);
    assert_eq!(path.kernel, "score_dot");
    assert_eq!(path.lane, LaneKind::Query);
    let encoder = ResourceRequirements::new(
        ResourceKind::Encoder,
        ExecutionEnvironment::Any,
    );
    assert_eq!(
        sub.core().select_path(& encoder).unwrap_err(), SubengineError::NoPath { kind :
        ResourceKind::Encoder }
    );
    let cpu_only = ResourceRequirements::new(
        ResourceKind::Scorer,
        ExecutionEnvironment::CpuOnly,
    );
    assert!(sub.core().select_path(& cpu_only).is_err());
}
#[test]
fn kinds_outside_the_offers_are_never_loaded() {
    let backend = MockBackend::single();
    let sub = subengine(&backend);
    assert!(! sub.is_loaded(ResourceKind::Tokenizer, None));
    assert!(! sub.is_loaded(ResourceKind::CrossEncoder, None));
}
#[test]
fn a_registered_model_is_uninitialized_and_duplicates_are_rejected() {
    let rig = model_rig();
    let m = model("m");
    rig.sub.register_model(m.clone(), Arc::clone(&rig.desc)).unwrap();
    assert_eq!(state(& rig.sub, "gpu:0:m"), Some(ResourceState::Uninitialized));
    assert_eq!(rig.calls.load(Ordering::SeqCst), 0);
    let err = rig.sub.register_model(m.clone(), Arc::clone(&rig.desc)).unwrap_err();
    assert!(matches!(err, GpuError::AlreadyRegistered(ref id) if * id == m), "{err:?}");
    rig.sub.register_model(model("n"), Arc::clone(&rig.desc)).unwrap();
    assert_eq!(state(& rig.sub, "gpu:0:n"), Some(ResourceState::Uninitialized));
}
#[test]
fn models_are_loaded_lazily_and_only_once() {
    let rig = model_rig();
    let m = model("m");
    let now = Instant::now();
    assert!(
        matches!(rig.sub.request_model(& m, 1, now), Err(GpuError::UnknownModel(ref id))
        if * id == m)
    );
    rig.sub.register_model(m.clone(), Arc::clone(&rig.desc)).unwrap();
    let ticket = started(&rig.sub, &m, now);
    assert_eq!(state(& rig.sub, "gpu:0:m"), Some(ResourceState::Loading));
    assert!(matches!(rig.sub.request_model(& m, 2, now).unwrap(), InitRequest::Loading));
    assert!(! rig.sub.is_loaded(ResourceKind::Encoder, Some(& m)));
    rig.sub.load_model(&m, ticket, now).unwrap();
    assert_eq!(rig.calls.load(Ordering::SeqCst), 1);
    assert_eq!(state(& rig.sub, "gpu:0:m"), Some(ResourceState::Ready));
    assert!(rig.sub.is_loaded(ResourceKind::Encoder, Some(& m)));
    assert!(rig.sub.is_loaded(ResourceKind::Encoder, None));
    assert!(! rig.sub.is_loaded(ResourceKind::Encoder, Some(& model("other"))));
    let InitRequest::Ready(lease) = rig.sub.request_model(&m, 3, now).unwrap() else {
        panic!("not ready");
    };
    assert_eq!(lease.handle().encoder().output_dim(), HIDDEN);
    assert_eq!(lease.handle().device_class(), DeviceClass::Gpu);
    assert_eq!(rig.calls.load(Ordering::SeqCst), 1);
}
#[test]
fn models_of_one_architecture_do_not_collide() {
    let rig = model_rig();
    let (a, b) = (model("a"), model("b"));
    rig.sub.register_model(a.clone(), Arc::clone(&rig.desc)).unwrap();
    rig.sub.register_model(b.clone(), Arc::clone(&rig.desc)).unwrap();
    let now = Instant::now();
    let ticket = started(&rig.sub, &a, now);
    rig.sub.load_model(&a, ticket, now).unwrap();
    assert!(rig.sub.is_loaded(ResourceKind::Encoder, Some(& a)));
    assert!(! rig.sub.is_loaded(ResourceKind::Encoder, Some(& b)));
    assert!(rig.sub.is_loaded(ResourceKind::Encoder, None));
}
#[test]
fn a_failed_load_reports_the_cause_and_backs_off() {
    let rig = model_rig();
    let m = model("broken");
    let broken = descriptor(&rig._dir, "no-such-architecture");
    rig.sub.register_model(m.clone(), broken).unwrap();
    let now = Instant::now();
    let ticket = started(&rig.sub, &m, now);
    let err = rig.sub.load_model(&m, ticket, now).unwrap_err();
    assert!(matches!(err, GpuError::Lifecycle(LifecycleError::Failed(_))), "{err:?}");
    assert_eq!(state(& rig.sub, "gpu:0:broken"), Some(ResourceState::Backoff));
    assert!(
        matches!(rig.sub.request_model(& m, 2, now).unwrap(), InitRequest::Backoff { ..
        })
    );
    assert!(! rig.sub.is_loaded(ResourceKind::Encoder, Some(& m)));
}
#[test]
fn a_ticket_of_an_unregistered_model_is_dropped_unrun() {
    let rig = model_rig();
    let m = model("m");
    rig.sub.register_model(m.clone(), Arc::clone(&rig.desc)).unwrap();
    let now = Instant::now();
    let ticket = started(&rig.sub, &m, now);
    let err = rig.sub.load_model(&model("ghost"), ticket, now).unwrap_err();
    assert!(
        matches!(err, GpuError::UnknownModel(ref id) if * id == model("ghost")),
        "{err:?}"
    );
    assert_eq!(rig.calls.load(Ordering::SeqCst), 0);
    assert_eq!(state(& rig.sub, "gpu:0:m"), Some(ResourceState::Uninitialized));
}
#[test]
fn expired_models_are_evicted_and_swapped_in_again() {
    let rig = model_rig();
    let m = model("m");
    rig.sub.register_model(m.clone(), Arc::clone(&rig.desc)).unwrap();
    let t0 = Instant::now();
    let ticket = started(&rig.sub, &m, t0);
    rig.sub.load_model(&m, ticket, t0).unwrap();
    assert!(rig.sub.evict_expired_models(t0 + Duration::from_secs(119)).is_empty());
    assert!(rig.sub.is_loaded(ResourceKind::Encoder, Some(& m)));
    let InitRequest::Ready(lease) = rig.sub.request_model(&m, 2, t0).unwrap() else {
        panic!("not ready");
    };
    assert!(rig.sub.evict_expired_models(t0 + Duration::from_secs(500)).is_empty());
    lease.release(t0 + Duration::from_secs(500));
    assert!(rig.sub.evict_expired_models(t0 + Duration::from_secs(600)).is_empty());
    let evicted = rig.sub.evict_expired_models(t0 + Duration::from_secs(621));
    assert_eq!(evicted, vec![m.clone()]);
    assert_eq!(state(& rig.sub, "gpu:0:m"), Some(ResourceState::Uninitialized));
    assert!(! rig.sub.is_loaded(ResourceKind::Encoder, Some(& m)));
    let again = started(&rig.sub, &m, t0 + Duration::from_secs(700));
    rig.sub.load_model(&m, again, t0 + Duration::from_secs(700)).unwrap();
    assert_eq!(rig.calls.load(Ordering::SeqCst), 2);
}
#[test]
fn models_in_backoff_are_not_evicted() {
    let rig = model_rig();
    let m = model("broken");
    rig.sub
        .register_model(m.clone(), descriptor(&rig._dir, "no-such-architecture"))
        .unwrap();
    let t0 = Instant::now();
    let ticket = started(&rig.sub, &m, t0);
    assert!(rig.sub.load_model(& m, ticket, t0).is_err());
    assert!(rig.sub.evict_expired_models(t0 + Duration::from_secs(10_000)).is_empty());
    assert_eq!(state(& rig.sub, "gpu:0:broken"), Some(ResourceState::Backoff));
}
fn index_spec() -> IndexSpec {
    IndexSpec {
        class: SlotClass::custom(2).unwrap(),
        dim: 4,
        dtype: DType::F32,
        segment_bytes: 1024,
    }
}
struct IndexRig {
    backend: Arc<MockBackend>,
    sub: Arc<GpuSubengine>,
    id: IndexId,
    manager: Arc<IndexManager>,
    index: Arc<ReferenceIndex>,
}
fn index_rig_with(cfg: GpuConfig) -> IndexRig {
    let backend = MockBackend::single();
    let sub = super::super::fixture::subengine_with(&backend, cfg);
    let id = IndexId::new("docs");
    let manager = Arc::new(IndexManager::new(id.clone()));
    let index = ReferenceIndex::new_shared(DType::F32);
    assert!(manager.attach(Arc::clone(& index) as Arc < dyn Index >));
    IndexRig {
        backend,
        sub,
        id,
        manager,
        index,
    }
}
fn index_rig() -> IndexRig {
    index_rig_with(config())
}
fn put(rig: &IndexRig, ids: &[u64]) {
    for id in ids {
        let vector: Vec<u8> = (0..8)
            .flat_map(|i| (*id as f32 + i as f32).to_le_bytes())
            .collect();
        rig.index
            .write(
                EntryId(*id),
                PayloadRef {
                    vector: &vector,
                    meta: b"meta",
                },
            )
            .unwrap();
    }
}
fn entries(ids: &[u64]) -> Vec<EntryId> {
    ids.iter().map(|id| EntryId(*id)).collect()
}
#[test]
fn attaching_needs_the_reserved_arenas() {
    let rig = index_rig();
    let err = rig
        .sub
        .attach_index(rig.id.clone(), Arc::clone(&rig.manager), index_spec())
        .err()
        .unwrap();
    assert!(matches!(err, GpuError::InvalidConfig(_)), "{err:?}");
    assert!(rig.sub.index_memory(& rig.id).is_none());
}
#[test]
fn attach_stores_the_memory_and_loads_nothing() {
    let rig = index_rig();
    rig.sub.activate().unwrap();
    let writes_before = rig.backend.visible_writes.load(Ordering::SeqCst);
    let memory = rig
        .sub
        .attach_index(rig.id.clone(), Arc::clone(&rig.manager), index_spec())
        .unwrap();
    assert!(Arc::ptr_eq(& memory, & rig.sub.index_memory(& rig.id).unwrap()));
    assert_eq!(rig.backend.visible_writes.load(Ordering::SeqCst), writes_before);
    assert_eq!(memory.gpu.segment_count(), 0);
    assert_eq!(memory.gpu.resident_bytes(), 0);
    assert!(! rig.sub.is_loaded(ResourceKind::Index, None));
    assert!(rig.sub.index_memory(& IndexId::new("other")).is_none());
}
#[test]
fn an_index_can_be_attached_only_once() {
    let rig = index_rig();
    rig.sub.activate().unwrap();
    rig.sub
        .attach_index(rig.id.clone(), Arc::clone(&rig.manager), index_spec())
        .unwrap();
    let err = rig
        .sub
        .attach_index(rig.id.clone(), Arc::clone(&rig.manager), index_spec())
        .err()
        .unwrap();
    assert!(
        matches!(err, GpuError::AlreadyAttached(ref id) if * id == rig.id), "{err:?}"
    );
}
#[test]
fn an_invalid_spec_stores_nothing() {
    let rig = index_rig();
    rig.sub.activate().unwrap();
    let mut spec = index_spec();
    spec.dim = 0;
    let err = rig
        .sub
        .attach_index(rig.id.clone(), Arc::clone(&rig.manager), spec)
        .err()
        .unwrap();
    assert!(
        matches!(err, GpuError::IndexMemory(GpuManagerError::InvalidConfig(_))),
        "{err:?}"
    );
    assert!(rig.sub.index_memory(& rig.id).is_none());
    rig.sub
        .attach_index(rig.id.clone(), Arc::clone(&rig.manager), index_spec())
        .unwrap();
}
#[test]
fn two_indexes_share_one_segment_arena() {
    let rig = index_rig();
    rig.sub.activate().unwrap();
    let other = IndexId::new("notes");
    let other_manager = Arc::new(IndexManager::new(other.clone()));
    let other_index = ReferenceIndex::new_shared(DType::F32);
    assert!(other_manager.attach(Arc::clone(& other_index) as Arc < dyn Index >));
    rig.sub
        .attach_index(rig.id.clone(), Arc::clone(&rig.manager), index_spec())
        .unwrap();
    rig.sub.attach_index(other.clone(), other_manager, index_spec()).unwrap();
    put(&rig, &[1, 2]);
    let report = rig.sub.ensure_entries(&rig.id, &entries(&[1, 2])).unwrap();
    assert_eq!(report.loaded, entries(& [1, 2]));
    let first = rig.sub.index_memory(&rig.id).unwrap();
    let second = rig.sub.index_memory(&other).unwrap();
    assert!(first.gpu.segment_count() >= 1);
    assert_eq!(second.gpu.segment_count(), 0);
    let used = rig
        .sub
        .segments
        .lock()
        .unwrap()
        .as_ref()
        .map(|s| s.allocators().shared(ArenaKind::IndexSegments).unwrap().available());
    assert!(used.unwrap() < config().arenas.get(ArenaKind::IndexSegments));
}
#[test]
fn entries_are_loaded_in_batches_in_first_occurrence_order() {
    let mut cfg = config();
    cfg.upload_batch = 2;
    let rig = index_rig_with(cfg);
    rig.sub.activate().unwrap();
    rig.sub
        .attach_index(rig.id.clone(), Arc::clone(&rig.manager), index_spec())
        .unwrap();
    put(&rig, &[1, 2, 3, 4, 5]);
    let ids = entries(&[3, 1, 3, 2, 5, 4, 1]);
    let report = rig.sub.ensure_entries(&rig.id, &ids).unwrap();
    assert_eq!(report.loaded, entries(& [1, 3, 2, 5, 4]));
    assert!(report.failed.is_empty());
    assert!(report.absent_in_index.is_empty());
    assert!(report.bytes_moved > 0);
    assert!(report.segments_created >= 1);
    assert!(rig.backend.visible_writes.load(Ordering::SeqCst) >= 5);
}
#[test]
fn resident_entries_are_not_copied_again() {
    let rig = index_rig();
    rig.sub.activate().unwrap();
    rig.sub
        .attach_index(rig.id.clone(), Arc::clone(&rig.manager), index_spec())
        .unwrap();
    put(&rig, &[1, 2, 3]);
    let ids = entries(&[1, 2, 3]);
    rig.sub.ensure_entries(&rig.id, &ids).unwrap();
    let writes = rig.backend.visible_writes.load(Ordering::SeqCst);
    let again = rig.sub.ensure_entries(&rig.id, &ids).unwrap();
    assert!(again.loaded.is_empty());
    assert_eq!(again.already_resident, ids);
    assert_eq!(again.bytes_moved, 0);
    assert_eq!(rig.backend.visible_writes.load(Ordering::SeqCst), writes);
}
#[test]
fn absent_entries_do_not_abort_the_others() {
    let rig = index_rig();
    rig.sub.activate().unwrap();
    rig.sub
        .attach_index(rig.id.clone(), Arc::clone(&rig.manager), index_spec())
        .unwrap();
    put(&rig, &[1, 2]);
    let report = rig.sub.ensure_entries(&rig.id, &entries(&[1, 9, 2])).unwrap();
    assert_eq!(report.loaded, entries(& [1, 2]));
    assert_eq!(report.absent_in_index, entries(& [9]));
}
#[test]
fn ensure_entries_checks_the_index_and_skips_empty_requests() {
    let rig = index_rig();
    rig.sub.activate().unwrap();
    let unknown = rig.sub.ensure_entries(&rig.id, &entries(&[1])).unwrap_err();
    assert!(
        matches!(unknown, GpuError::UnknownIndex(ref id) if * id == rig.id),
        "{unknown:?}"
    );
    rig.sub
        .attach_index(rig.id.clone(), Arc::clone(&rig.manager), index_spec())
        .unwrap();
    let report = rig.sub.ensure_entries(&rig.id, &[]).unwrap();
    assert_eq!(report, EnsureReport::default());
    assert_eq!(rig.backend.visible_writes.load(Ordering::SeqCst), 0);
}
#[test]
fn the_residency_factory_attaches_and_returns_the_operations() {
    let rig = index_rig();
    rig.sub.activate().unwrap();
    let factory: Arc<dyn BackendFactory> = Arc::clone(&rig.sub)
        as Arc<dyn BackendFactory>;
    let backend = factory
        .create(&rig.id, &index_spec(), Arc::clone(&rig.manager))
        .unwrap();
    let memory = rig.sub.index_memory(&rig.id).unwrap();
    assert!(! backend.pending());
    assert!(! backend.is_resident(EntryId(1)));
    assert_eq!(memory.gpu.segment_count(), 0);
    let err = factory
        .create(&rig.id, &index_spec(), Arc::clone(&rig.manager))
        .err()
        .unwrap();
    assert!(err.contains("bereits angebunden"), "{err}");
}
struct Job;
impl BatchPayload for Job {
    fn batch_key(&self) -> Option<BatchKey> {
        None
    }
    fn sequences(&self) -> &[Vec<u32>] {
        &[]
    }
}
struct Meta {
    requirements: ResourceRequirements,
}
impl BatchMeta for Meta {
    fn batch_key(&self) -> Option<BatchKey> {
        None
    }
    fn seq_lengths(&self) -> &[usize] {
        &[]
    }
}
impl HasResourceRequirements for Meta {
    fn resource_requirements(&self) -> &ResourceRequirements {
        &self.requirements
    }
}
struct NoopRunner;
impl BatchRunner<Job> for NoopRunner {
    fn run(&self, _lane: &mut Lane, _batch: &mut ExecBatch<Job>) -> Launch {
        Launch::Pending
    }
    fn collect(
        &self,
        _lane: &mut Lane,
        _batch: ExecBatch<Job>,
        _result: Result<(), l010::device_lane_event::EventError>,
    ) {}
}
fn lane_cfg(
    device: DeviceHandle,
    with_completion: bool,
    limit: BatchLimit,
) -> LaneResourceConfig<Meta, Job> {
    let topology = NotificationTopology::new(1, Arc::new(|| {}));
    let notifier = topology.notifier(0).unwrap();
    LaneResourceConfig {
        own: Arc::new(Queue::new()),
        shared: None,
        filter: Arc::new(|_: &Meta| true),
        completion: with_completion
            .then(|| Arc::new(GpuCompletion::new(notifier.clone()))),
        limit,
        notifier,
        kind: ResourceKind::Scorer,
        device,
    }
}
#[test]
fn the_lane_resource_gets_worker_lanes_with_their_own_streams() {
    let backend = MockBackend::single();
    let mut cfg = config();
    cfg.worker_lanes = 3;
    let sub = super::super::fixture::subengine_with(&backend, cfg);
    let resource = sub
        .build_lane_resource(
            lane_cfg(DeviceHandle::Gpu(0), true, BatchLimit::Tasks(4)),
            Arc::new(NoopRunner),
        )
        .unwrap();
    assert_eq!(resource.lane_count(), 3);
    assert_eq!(resource.idle_lanes(), 3);
    assert_eq!(* lock(& backend.streams), vec![(0, Priority::High); 3]);
    assert!(! sub.core().is_active());
    sub.activate().unwrap();
    assert_eq!(lock(& backend.streams).len(), 3 + 5);
}
#[test]
fn the_lane_resource_checks_device_and_completion() {
    let backend = MockBackend::single();
    let sub = subengine(&backend);
    let wrong = sub
        .build_lane_resource(
            lane_cfg(DeviceHandle::Gpu(1), true, BatchLimit::Tasks(4)),
            Arc::new(NoopRunner),
        );
    assert!(matches!(wrong, Err(GpuError::InvalidConfig(_))));
    let cpu = sub
        .build_lane_resource(
            lane_cfg(DeviceHandle::Cpu, true, BatchLimit::Tasks(4)),
            Arc::new(NoopRunner),
        );
    assert!(matches!(cpu, Err(GpuError::InvalidConfig(_))));
    let untracked = sub
        .build_lane_resource(
            lane_cfg(DeviceHandle::Gpu(0), false, BatchLimit::Tasks(4)),
            Arc::new(NoopRunner),
        );
    assert!(matches!(untracked, Err(GpuError::InvalidConfig(_))));
    assert!(lock(& backend.streams).is_empty());
}
#[test]
fn lane_resource_errors_are_mapped() {
    let backend = MockBackend::single();
    let sub = subengine(&backend);
    let limit = sub
        .build_lane_resource(
            lane_cfg(DeviceHandle::Gpu(0), true, BatchLimit::Tasks(0)),
            Arc::new(NoopRunner),
        );
    assert!(matches!(limit, Err(GpuError::LaneSet(LaneSetError::InvalidLimit))));
    backend.streams_before_failure.store(1, Ordering::SeqCst);
    let streams = sub
        .build_lane_resource(
            lane_cfg(DeviceHandle::Gpu(0), true, BatchLimit::Tasks(4)),
            Arc::new(NoopRunner),
        );
    assert!(matches!(streams, Err(GpuError::Backend(_))));
}
