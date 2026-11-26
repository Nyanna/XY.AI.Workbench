use super::*;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, Instant};
use serde_json::json;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::a::resource_lifecycle::{
    InitRequest, LifecycleConfig, MemoryClass, ResourceRegistry, ResourceState,
};
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::a::model_descriptor::Pooling;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::_1::layout::DType;
const HIDDEN: usize = 16;
const LAYERS: usize = 2;
const HEADS: usize = 2;
const INTERMEDIATE: usize = 32;
const VOCAB: usize = 50;
const MAX_POS: usize = 32;
const SEQ: usize = 16;
static COUNTER: AtomicUsize = AtomicUsize::new(0);
const fn assert_send_sync_static<T: Send + Sync + 'static>() {}
const _: () = {
    assert_send_sync_static::<LoadedEncoder>();
    assert_send_sync_static::<ModelRegistry>();
    assert_send_sync_static::<EncoderResource>();
    assert_send_sync_static::<RegistryError>();
};
struct TestDir(PathBuf);
impl TestDir {
    fn new() -> TestDir {
        let n = COUNTER.fetch_add(1, Ordering::SeqCst);
        let path = std::env::temp_dir()
            .join(format!("model_registry_{}_{n}", std::process::id()));
        std::fs::create_dir_all(&path).unwrap();
        TestDir(path)
    }
}
impl Drop for TestDir {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}
struct Rng(u64);
impl Rng {
    fn next(&mut self) -> f32 {
        self.0 = self
            .0
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        ((self.0 >> 40) as f32 / (1u64 << 24) as f32 - 0.5) * 0.4
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
fn write_safetensors(path: &Path, tensors: &[(String, Vec<usize>, Vec<f32>)]) {
    let mut header = String::from("{");
    let mut data: Vec<u8> = Vec::new();
    for (i, (name, shape, values)) in tensors.iter().enumerate() {
        let begin = data.len();
        for v in values {
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
    std::fs::write(path, file).unwrap();
}
fn weights() -> Vec<(String, Vec<usize>, Vec<f32>)> {
    let mut rng = Rng(42);
    tensor_shapes()
        .into_iter()
        .map(|(name, shape)| {
            let n: usize = shape.iter().product();
            let ln_weight = name.contains("LayerNorm") && name.ends_with(".weight");
            let values = (0..n)
                .map(|_| if ln_weight { 1.0 + rng.next() * 0.1 } else { rng.next() })
                .collect();
            (name, shape, values)
        })
        .collect()
}
fn config_json() -> serde_json::Value {
    json!(
        { "model_type" : "bert", "hidden_size" : HIDDEN, "num_hidden_layers" : LAYERS,
        "num_attention_heads" : HEADS, "intermediate_size" : INTERMEDIATE, "vocab_size" :
        VOCAB, "max_position_embeddings" : MAX_POS, "hidden_act" : "gelu",
        "layer_norm_eps" : 1e-12, "type_vocab_size" : 2 }
    )
}
struct Fixture {
    _dir: TestDir,
    desc: ModelDescriptor,
}
fn fixture() -> Fixture {
    let dir = TestDir::new();
    std::fs::write(dir.0.join("config.json"), config_json().to_string()).unwrap();
    write_safetensors(&dir.0.join("model.safetensors"), &weights());
    let mut desc = ModelDescriptor::e5_small_default(&dir.0, DType::F32);
    desc.dimension = HIDDEN;
    desc.max_seq_len = SEQ;
    Fixture { _dir: dir, desc }
}
struct CountingFactory {
    calls: Arc<AtomicUsize>,
    source: ModelDescriptor,
    fail: Option<String>,
}
impl EncoderFactory for CountingFactory {
    fn build(
        &self,
        _descriptor: &ModelDescriptor,
        class: DeviceClass,
    ) -> Result<Encoder, String> {
        self.calls.fetch_add(1, Ordering::SeqCst);
        match &self.fail {
            Some(reason) => Err(reason.clone()),
            None => build_bert_encoder(&self.source, class),
        }
    }
}
fn counting(
    fx: &Fixture,
    fail: Option<&str>,
) -> (Arc<CountingFactory>, Arc<AtomicUsize>) {
    let calls = Arc::new(AtomicUsize::new(0));
    let factory = Arc::new(CountingFactory {
        calls: Arc::clone(&calls),
        source: fx.desc.clone(),
        fail: fail.map(str::to_string),
    });
    (factory, calls)
}
fn custom_descriptor(fx: &Fixture) -> ModelDescriptor {
    let mut desc = fx.desc.clone();
    desc.architecture = "custom".to_string();
    desc
}
fn registry_with_custom(
    fx: &Fixture,
    fail: Option<&str>,
) -> (ModelRegistry, Arc<AtomicUsize>) {
    let (factory, calls) = counting(fx, fail);
    let mut registry = ModelRegistry::new();
    registry.register_architecture("custom", factory);
    (registry, calls)
}
fn expect_err(result: Result<LoadedEncoder, RegistryError>) -> RegistryError {
    match result {
        Ok(_) => panic!("expected an error"),
        Err(e) => e,
    }
}
#[test]
fn key_format_per_class() {
    assert_eq!(encoder_key("bert", DeviceClass::Gpu), "bert:gpu");
    assert_eq!(encoder_key("bert", DeviceClass::Cpu), "bert:cpu");
}
#[test]
fn keys_differ_by_class_and_architecture() {
    let a = encoder_key("bert", DeviceClass::Cpu);
    assert_eq!(a, encoder_key("bert", DeviceClass::Cpu));
    assert_ne!(a, encoder_key("bert", DeviceClass::Gpu));
    assert_ne!(a, encoder_key("other", DeviceClass::Cpu));
}
#[test]
fn new_registry_is_empty_and_default_matches() {
    for registry in [ModelRegistry::new(), ModelRegistry::default()] {
        assert!(registry.architectures().is_empty());
        assert!(! registry.supports_architecture("bert"));
    }
}
#[test]
fn with_builtin_contains_only_bert() {
    let registry = ModelRegistry::with_builtin();
    assert_eq!(registry.architectures(), vec!["bert".to_string()]);
    assert!(registry.supports_architecture("bert"));
}
#[test]
fn registration_returns_replaced_factory() {
    let fx = fixture();
    let (first, _) = counting(&fx, None);
    let (second, _) = counting(&fx, None);
    let first: Arc<dyn EncoderFactory> = first;
    let second: Arc<dyn EncoderFactory> = second;
    let mut registry = ModelRegistry::new();
    assert!(registry.register_architecture("custom", Arc::clone(& first)).is_none());
    let replaced = registry
        .register_architecture(String::from("custom"), second)
        .unwrap();
    assert!(Arc::ptr_eq(& replaced, & first));
    assert_eq!(registry.architectures().len(), 1);
}
#[test]
fn architectures_are_sorted_and_lookup_is_case_sensitive() {
    let fx = fixture();
    let mut registry = ModelRegistry::new();
    for id in ["zeta", "alpha", "Mid"] {
        registry.register_architecture(id, counting(&fx, None).0);
    }
    assert_eq!(registry.architectures(), vec!["Mid", "alpha", "zeta"]);
    assert!(registry.supports_architecture("alpha"));
    assert!(! registry.supports_architecture("Alpha"));
    assert!(! registry.supports_architecture(""));
}
#[test]
fn registration_does_not_call_the_factory() {
    let fx = fixture();
    let (registry, calls) = registry_with_custom(&fx, None);
    assert!(registry.supports_architecture("custom"));
    assert_eq!(calls.load(Ordering::SeqCst), 0);
}
#[test]
fn error_display_strings() {
    assert_eq!(
        RegistryError::UnsupportedArchitecture("foo".into()).to_string(),
        "nicht unterstützte Architektur: foo"
    );
    assert_eq!(
        RegistryError::Load { architecture : "bert".into(), reason : "kaputt".into() }
        .to_string(), "Encoder-Laden fehlgeschlagen (bert): kaputt"
    );
    let inner = ModelConfigError::Incompatible {
        reason: "x".into(),
    };
    assert_eq!(RegistryError::Config(inner.clone()).to_string(), inner.to_string());
}
#[test]
fn load_returns_descriptor_class_and_encoder() {
    let fx = fixture();
    let (registry, calls) = registry_with_custom(&fx, None);
    let desc = custom_descriptor(&fx);
    let loaded = registry.load_encoder(&desc, DeviceClass::Cpu).ok().unwrap();
    assert_eq!(loaded.descriptor(), & desc);
    assert_eq!(loaded.device_class(), DeviceClass::Cpu);
    assert_eq!(loaded.encoder().output_dim(), HIDDEN);
    assert_eq!(loaded.encoder().storage_dtype(), DType::F32);
    assert_eq!(calls.load(Ordering::SeqCst), 1);
}
#[test]
fn invalid_descriptor_fails_before_the_factory() {
    let fx = fixture();
    let (registry, calls) = registry_with_custom(&fx, None);
    let mut desc = custom_descriptor(&fx);
    desc.dimension = 0;
    let expected = desc.validate().err().unwrap();
    let e = expect_err(registry.load_encoder(&desc, DeviceClass::Cpu));
    assert_eq!(e, RegistryError::Config(expected));
    assert_eq!(calls.load(Ordering::SeqCst), 0);
}
#[test]
fn validation_wins_over_unknown_architecture() {
    let fx = fixture();
    let mut desc = custom_descriptor(&fx);
    desc.architecture = "unknown".to_string();
    desc.max_seq_len = 0;
    let e = expect_err(ModelRegistry::new().load_encoder(&desc, DeviceClass::Cpu));
    assert!(matches!(e, RegistryError::Config(_)));
}
#[test]
fn unknown_architecture_names_the_id() {
    let fx = fixture();
    let mut desc = fx.desc.clone();
    desc.architecture = "mystery".to_string();
    let e = expect_err(ModelRegistry::new().load_encoder(&desc, DeviceClass::Cpu));
    assert_eq!(e, RegistryError::UnsupportedArchitecture("mystery".to_string()));
    assert!(e.to_string().contains("mystery"));
}
#[test]
fn empty_architecture_is_unsupported() {
    let fx = fixture();
    let mut desc = fx.desc.clone();
    desc.architecture = String::new();
    let e = expect_err(
        ModelRegistry::with_builtin().load_encoder(&desc, DeviceClass::Cpu),
    );
    assert_eq!(e, RegistryError::UnsupportedArchitecture(String::new()));
}
#[test]
fn factory_error_becomes_load_error() {
    let fx = fixture();
    let (registry, calls) = registry_with_custom(&fx, Some("kein Speicher"));
    let e = expect_err(registry.load_encoder(&custom_descriptor(&fx), DeviceClass::Cpu));
    assert_eq!(
        e, RegistryError::Load { architecture : "custom".into(), reason : "kein Speicher"
        .into() }
    );
    assert_eq!(calls.load(Ordering::SeqCst), 1);
}
#[test]
fn factory_panic_is_not_caught() {
    let mut registry = ModelRegistry::new();
    let boom = |_: &ModelDescriptor, _: DeviceClass| -> Result<Encoder, String> {
        panic!("boom")
    };
    registry.register_architecture("custom", Arc::new(boom));
    let fx = fixture();
    let desc = custom_descriptor(&fx);
    let outcome = catch_unwind(
        AssertUnwindSafe(|| registry.load_encoder(&desc, DeviceClass::Cpu).is_ok()),
    );
    assert!(outcome.is_err());
}
#[test]
fn closure_factory_receives_descriptor_and_class() {
    let fx = fixture();
    let seen = Arc::new(std::sync::Mutex::new(Vec::new()));
    let source = fx.desc.clone();
    let log = Arc::clone(&seen);
    let closure = move |d: &ModelDescriptor, c: DeviceClass| -> Result<Encoder, String> {
        log.lock().unwrap().push((d.architecture.clone(), c));
        build_bert_encoder(&source, c)
    };
    let mut registry = ModelRegistry::new();
    registry.register_architecture("custom", Arc::new(closure));
    assert!(registry.load_encoder(& custom_descriptor(& fx), DeviceClass::Cpu).is_ok());
    assert_eq!(* seen.lock().unwrap(), vec![("custom".to_string(), DeviceClass::Cpu)]);
}
#[test]
fn output_dimension_is_not_checked_against_the_descriptor() {
    let fx = fixture();
    let (registry, _) = registry_with_custom(&fx, None);
    let mut desc = custom_descriptor(&fx);
    desc.dimension = HIDDEN + 5;
    let loaded = registry.load_encoder(&desc, DeviceClass::Cpu).ok().unwrap();
    assert_eq!(loaded.encoder().output_dim(), HIDDEN);
    assert_eq!(loaded.descriptor().dimension, HIDDEN + 5);
}
#[test]
fn builtin_bert_loads_on_cpu() {
    let fx = fixture();
    let loaded = ModelRegistry::with_builtin()
        .load_encoder(&fx.desc, DeviceClass::Cpu)
        .ok()
        .unwrap();
    assert_eq!(loaded.encoder().output_dim(), HIDDEN);
    assert_eq!(loaded.descriptor(), & fx.desc);
}
#[test]
fn builtin_bert_rejects_cls_pooling() {
    let fx = fixture();
    let mut desc = fx.desc.clone();
    desc.pooling = Pooling::Cls;
    let e = expect_err(
        ModelRegistry::with_builtin().load_encoder(&desc, DeviceClass::Cpu),
    );
    match e {
        RegistryError::Load { architecture, reason } => {
            assert_eq!(architecture, "bert");
            assert!(reason.contains("Cls"));
        }
        other => panic!("unexpected {other:?}"),
    }
}
#[test]
fn builtin_bert_reports_missing_weights() {
    let fx = fixture();
    let mut desc = fx.desc.clone();
    desc.weight_paths = vec![fx._dir.0.join("absent.safetensors")];
    let e = expect_err(
        ModelRegistry::with_builtin().load_encoder(&desc, DeviceClass::Cpu),
    );
    assert!(
        matches!(e, RegistryError::Load { ref architecture, .. } if architecture ==
        "bert")
    );
}
#[test]
fn resource_carries_key_cost_and_loads_nothing() {
    let fx = fixture();
    let (registry, calls) = registry_with_custom(&fx, None);
    let registry = Arc::new(registry);
    let desc = Arc::new(custom_descriptor(&fx));
    let resource = registry.encoder_resource(Arc::clone(&desc), DeviceClass::Gpu);
    assert_eq!(resource.key, "custom:gpu");
    assert_eq!(resource.key, encoder_key(& desc.architecture, DeviceClass::Gpu));
    assert_eq!(resource.init_cost, ENCODER_INIT_COST);
    assert_eq!(resource.init_cost, 100);
    assert_eq!(calls.load(Ordering::SeqCst), 0);
}
#[test]
fn resource_for_unknown_architecture_fails_only_on_load() {
    let registry = Arc::new(ModelRegistry::new());
    let fx = fixture();
    let mut desc = fx.desc.clone();
    desc.architecture = "ghost".to_string();
    let resource = registry.encoder_resource(Arc::new(desc), DeviceClass::Cpu);
    assert_eq!(resource.key, "ghost:cpu");
    let err = resource.loader.load().err().unwrap();
    assert_eq!(err, RegistryError::UnsupportedArchitecture("ghost".into()).to_string());
}
#[test]
fn loader_builds_a_new_encoder_on_every_call() {
    let fx = fixture();
    let (registry, calls) = registry_with_custom(&fx, None);
    let resource = Arc::new(registry)
        .encoder_resource(Arc::new(custom_descriptor(&fx)), DeviceClass::Cpu);
    let first = resource.loader.load().ok().unwrap();
    let second = resource.loader.load().ok().unwrap();
    assert!(! std::ptr::eq(first.encoder(), second.encoder()));
    assert_eq!(first.device_class(), DeviceClass::Cpu);
    assert_eq!(calls.load(Ordering::SeqCst), 2);
}
#[test]
fn loader_maps_errors_to_display_text() {
    let fx = fixture();
    let (registry, _) = registry_with_custom(&fx, Some("defekt"));
    let resource = Arc::new(registry)
        .encoder_resource(Arc::new(custom_descriptor(&fx)), DeviceClass::Cpu);
    assert_eq!(
        resource.loader.load().err().unwrap(),
        "Encoder-Laden fehlgeschlagen (custom): defekt"
    );
}
fn lifecycle() -> ResourceRegistry<LoadedEncoder> {
    let config = LifecycleConfig::new(
            Duration::from_secs(60),
            Duration::from_secs(120),
            Duration::from_secs(5),
        )
        .unwrap();
    ResourceRegistry::new(config)
}
#[test]
fn resource_loads_through_the_lifecycle_registry() {
    let fx = fixture();
    let (registry, calls) = registry_with_custom(&fx, None);
    let desc = Arc::new(custom_descriptor(&fx));
    let resource = Arc::new(registry)
        .encoder_resource(Arc::clone(&desc), DeviceClass::Cpu);
    let lifecycle = lifecycle();
    lifecycle.register(resource.key.clone(), MemoryClass::Ram, resource.loader).unwrap();
    assert_eq!(calls.load(Ordering::SeqCst), 0);
    let now = Instant::now();
    match lifecycle.request_init(&resource.key, 1, now) {
        InitRequest::Started(ticket) => ticket.run(now).unwrap(),
        _ => panic!("expected a load ticket"),
    }
    match lifecycle.request_init(&resource.key, 2, now) {
        InitRequest::Ready(lease) => {
            assert_eq!(lease.handle().descriptor(), desc.as_ref());
            assert_eq!(lease.handle().device_class(), DeviceClass::Cpu);
        }
        _ => panic!("expected a ready resource"),
    }
    assert_eq!(calls.load(Ordering::SeqCst), 1);
}
#[test]
fn loader_panic_is_caught_by_the_ticket() {
    let mut registry = ModelRegistry::new();
    let boom = |_: &ModelDescriptor, _: DeviceClass| -> Result<Encoder, String> {
        panic!("boom")
    };
    registry.register_architecture("custom", Arc::new(boom));
    let fx = fixture();
    let resource = Arc::new(registry)
        .encoder_resource(Arc::new(custom_descriptor(&fx)), DeviceClass::Cpu);
    let lifecycle = lifecycle();
    lifecycle.register(resource.key.clone(), MemoryClass::Ram, resource.loader).unwrap();
    let now = Instant::now();
    match lifecycle.request_init(&resource.key, 1, now) {
        InitRequest::Started(ticket) => assert!(ticket.run(now).is_err()),
        _ => panic!("expected a load ticket"),
    }
    assert_eq!(lifecycle.state(& resource.key), Some(ResourceState::Backoff));
}
#[test]
fn concurrent_loads_are_not_serialized_or_shared() {
    let fx = fixture();
    let (registry, calls) = registry_with_custom(&fx, None);
    let registry = Arc::new(registry);
    let desc = Arc::new(custom_descriptor(&fx));
    let handles: Vec<_> = (0..4)
        .map(|_| {
            let registry = Arc::clone(&registry);
            let desc = Arc::clone(&desc);
            std::thread::spawn(move || {
                registry
                    .load_encoder(&desc, DeviceClass::Cpu)
                    .ok()
                    .unwrap()
                    .encoder()
                    .output_dim()
            })
        })
        .collect();
    for h in handles {
        assert_eq!(h.join().unwrap(), HIDDEN);
    }
    assert_eq!(calls.load(Ordering::SeqCst), 4);
}
