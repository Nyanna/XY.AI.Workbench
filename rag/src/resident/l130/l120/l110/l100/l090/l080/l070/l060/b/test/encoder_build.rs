use super::*;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use serde_json::json;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::a::model_descriptor::{
    ComputeDTypes, MultiVectorConfig,
};
const HIDDEN: usize = 16;
const LAYERS: usize = 2;
const HEADS: usize = 2;
const INTERMEDIATE: usize = 32;
const VOCAB: usize = 50;
const MAX_POS: usize = 32;
const SEQ: usize = 16;
const PROJ_OUT: usize = 8;
const PROJ_NAME: &str = "linear.weight";
static COUNTER: AtomicUsize = AtomicUsize::new(0);
const fn assert_send_sync_static<T: Send + Sync + 'static>() {}
const _: () = {
    assert_send_sync_static::<BuiltEncoder>();
    assert_send_sync_static::<EncoderBuildError>();
};
struct TestDir(PathBuf);
impl TestDir {
    fn new() -> TestDir {
        let n = COUNTER.fetch_add(1, Ordering::SeqCst);
        let path = std::env::temp_dir()
            .join(format!("encoder_build_{}_{n}", std::process::id()));
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
fn weights(
    omit: Option<&str>,
    extra: &[(&str, Vec<usize>)],
) -> Vec<(String, Vec<usize>, Vec<f32>)> {
    let mut rng = Rng(42);
    let mut shapes = tensor_shapes();
    shapes.extend(extra.iter().map(|(n, s)| (n.to_string(), s.clone())));
    shapes
        .into_iter()
        .filter(|(name, _)| Some(name.as_str()) != omit)
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
fn fixture_with(omit: Option<&str>, extra: &[(&str, Vec<usize>)]) -> Fixture {
    let dir = TestDir::new();
    std::fs::write(dir.0.join("config.json"), config_json().to_string()).unwrap();
    write_safetensors(&dir.0.join("model.safetensors"), &weights(omit, extra));
    let mut desc = ModelDescriptor::e5_small_default(&dir.0, DType::F32);
    desc.dimension = HIDDEN;
    desc.max_seq_len = SEQ;
    Fixture { _dir: dir, desc }
}
fn plain() -> Fixture {
    fixture_with(None, &[])
}
fn with_projection() -> Fixture {
    let mut fx = fixture_with(None, &[(PROJ_NAME, vec![PROJ_OUT, HIDDEN])]);
    fx.desc.projection_layers = vec![layer(PROJ_NAME)];
    fx
}
fn layer(name: &str) -> ProjectionLayer {
    ProjectionLayer {
        tensor_name: name.to_string(),
    }
}
fn multi_vector(dimension: usize) -> MultiVectorConfig {
    MultiVectorConfig {
        max_query_len: 8,
        max_document_len: 16,
        query_marker: "[Q]".into(),
        document_marker: "[D]".into(),
        dimension,
        skiplist: Vec::new(),
    }
}
fn expect_err(fx: &Fixture) -> EncoderBuildError {
    match build_encoder(&fx.desc, DeviceClass::Cpu) {
        Ok(_) => panic!("build unexpectedly succeeded"),
        Err(e) => e,
    }
}
fn incompatible_reason(e: EncoderBuildError) -> String {
    match e {
        EncoderBuildError::Incompatible { reason } => reason,
        other => panic!("expected Incompatible, got {other:?}"),
    }
}
#[test]
fn normalization_defaults_to_l2() {
    assert_eq!(DEFAULT_NORMALIZATION, Normalization::L2);
    assert_eq!(resolve_normalization(None), Normalization::L2);
    assert_eq!(resolve_normalization(Some(Normalization::None)), Normalization::None);
    assert_eq!(resolve_normalization(Some(Normalization::L2)), Normalization::L2);
}
#[test]
fn error_display_strings() {
    let e = EncoderBuildError::Incompatible {
        reason: "x".into(),
    };
    assert_eq!(e.to_string(), "inkompatibel: x");
    let e = EncoderBuildError::Load {
        reason: "y".into(),
    };
    assert_eq!(e.to_string(), "Load-Error: y");
    let e = EncoderBuildError::WeightNameMismatch {
        name: "z".into(),
    };
    assert_eq!(e.to_string(), "Gewichtsname passt nicht: z");
    let _: &dyn std::error::Error = &e;
}
#[test]
fn plain_model_builds_without_projection() {
    let fx = plain();
    let built = build_encoder(&fx.desc, DeviceClass::Cpu).unwrap();
    assert!(built.projection().is_none());
    assert!(! built.is_multi_vector());
    assert_eq!(built.encoder().output_dim(), fx.desc.dimension);
    assert_eq!(built.encoder().storage_dtype(), DType::F32);
    assert_eq!(built.storage_dtype(), DType::F32);
    assert_eq!(built.device_class(), DeviceClass::Cpu);
    assert_eq!(built.compute_dtype(), fx.desc.compute_dtype(DeviceClass::Cpu));
    assert_eq!(built.compute_dtype(), built.model().compute_dtype());
    assert_eq!(built.model().dimension(), HIDDEN);
}
#[test]
fn storage_dtype_comes_from_descriptor() {
    for dtype in [DType::F16, DType::BF16] {
        let mut fx = plain();
        fx.desc.storage_dtype = dtype;
        let built = build_encoder(&fx.desc, DeviceClass::Cpu).unwrap();
        assert_eq!(built.storage_dtype(), dtype);
        assert_eq!(built.encoder().storage_dtype(), dtype);
    }
}
#[test]
fn compute_dtype_follows_descriptor() {
    let mut fx = plain();
    fx.desc.compute_dtypes = ComputeDTypes {
        cpu: DType::F16,
        gpu: DType::F32,
    };
    let built = build_encoder(&fx.desc, DeviceClass::Cpu).unwrap();
    assert_eq!(built.compute_dtype(), DType::F16);
    assert_eq!(built.model().compute_dtype(), DType::F16);
}
#[test]
fn normalization_none_is_accepted() {
    let mut fx = plain();
    fx.desc.normalization = Normalization::None;
    assert!(build_encoder(& fx.desc, DeviceClass::Cpu).is_ok());
}
#[test]
fn projection_changes_output_dim() {
    let fx = with_projection();
    let built = build_encoder(&fx.desc, DeviceClass::Cpu).unwrap();
    let p = built.projection().expect("projection");
    assert_eq!(p.input_dim(), HIDDEN);
    assert_eq!(p.output_dim(), PROJ_OUT);
    assert_eq!(built.encoder().output_dim(), PROJ_OUT);
    assert!(! built.is_multi_vector());
}
#[test]
fn multi_vector_builds_with_projection_head() {
    let mut fx = with_projection();
    fx.desc.multi_vector = Some(multi_vector(PROJ_OUT));
    let built = build_encoder(&fx.desc, DeviceClass::Cpu).unwrap();
    assert!(built.is_multi_vector());
    assert_eq!(built.encoder().output_dim(), PROJ_OUT);
}
#[test]
fn multi_vector_ignores_cls_pooling() {
    let mut fx = with_projection();
    fx.desc.multi_vector = Some(multi_vector(PROJ_OUT));
    fx.desc.pooling = Pooling::Cls;
    assert!(build_encoder(& fx.desc, DeviceClass::Cpu).is_ok());
}
#[test]
fn multi_vector_without_projection_is_incompatible() {
    let mut fx = plain();
    fx.desc.multi_vector = Some(multi_vector(HIDDEN));
    let reason = incompatible_reason(expect_err(&fx));
    assert!(reason.contains("Projection-Head fehlt"), "{reason}");
}
#[test]
fn multi_vector_dimension_must_match_projection_output() {
    let mut fx = with_projection();
    fx.desc.multi_vector = Some(multi_vector(PROJ_OUT + 1));
    let reason = incompatible_reason(expect_err(&fx));
    assert!(reason.contains(& PROJ_OUT.to_string()), "{reason}");
    assert!(reason.contains(& (PROJ_OUT + 1).to_string()), "{reason}");
}
#[test]
fn cls_pooling_is_incompatible() {
    let mut fx = plain();
    fx.desc.pooling = Pooling::Cls;
    let reason = incompatible_reason(expect_err(&fx));
    assert!(reason.contains("Cls"), "{reason}");
}
#[test]
fn invalid_descriptor_is_incompatible() {
    let mut fx = plain();
    fx.desc.dimension = 0;
    let expected = fx.desc.validate().unwrap_err();
    let ModelConfigError::Incompatible { reason: expected } = expected;
    assert_eq!(expect_err(& fx), EncoderBuildError::Incompatible { reason : expected });
}
#[test]
fn validation_comes_before_pooling_check() {
    let mut fx = plain();
    fx.desc.max_seq_len = 0;
    fx.desc.pooling = Pooling::Cls;
    let reason = incompatible_reason(expect_err(&fx));
    assert!(! reason.contains("Cls"), "{reason}");
}
#[test]
fn several_projection_layers_are_incompatible() {
    let mut fx = with_projection();
    fx.desc.projection_layers.push(layer(PROJ_NAME));
    let reason = incompatible_reason(expect_err(&fx));
    assert!(reason.contains('2'), "{reason}");
}
#[test]
fn projection_without_weight_files_is_a_load_error() {
    let mut fx = with_projection();
    fx.desc.weight_paths.clear();
    assert_eq!(
        expect_err(& fx), EncoderBuildError::Load { reason : "keine Gewichtsdateien"
        .into() }
    );
}
#[test]
fn missing_projection_tensor_is_reported_by_name() {
    let mut fx = with_projection();
    fx.desc.projection_layers = vec![layer("absent.weight")];
    assert_eq!(
        expect_err(& fx), EncoderBuildError::WeightNameMismatch { name : "absent.weight"
        .into() }
    );
}
#[test]
fn projection_tensor_must_be_rank_two() {
    let mut fx = fixture_with(None, &[("flat.weight", vec![HIDDEN])]);
    fx.desc.projection_layers = vec![layer("flat.weight")];
    let reason = incompatible_reason(expect_err(&fx));
    assert!(reason.contains("Rank"), "{reason}");
}
#[test]
fn projection_input_must_match_model_dimension() {
    let mut fx = fixture_with(None, &[("wide.weight", vec![PROJ_OUT, HIDDEN + 1])]);
    fx.desc.projection_layers = vec![layer("wide.weight")];
    let reason = incompatible_reason(expect_err(&fx));
    assert!(reason.contains("inkompatibel"), "{reason}");
    assert!(reason.contains(& (HIDDEN + 1).to_string()), "{reason}");
    assert!(reason.contains(& HIDDEN.to_string()), "{reason}");
}
#[test]
fn only_the_first_weight_file_is_searched_for_the_projection() {
    let fx = with_projection();
    let mut desc = fx.desc.clone();
    let other = fx._dir.0.join("other.safetensors");
    write_safetensors(&other, &[("x.weight".into(), vec![1], vec![0.0])]);
    let first = desc.weight_paths[0].clone();
    desc.weight_paths = vec![other, first];
    let err = build_encoder(&desc, DeviceClass::Cpu).err().unwrap();
    assert!(matches!(err, EncoderBuildError::WeightNameMismatch { .. }), "{err:?}");
}
#[test]
fn unreadable_first_weight_file_is_a_load_error() {
    let fx = with_projection();
    let mut desc = fx.desc.clone();
    let broken = fx._dir.0.join("broken.safetensors");
    std::fs::write(&broken, b"abc").unwrap();
    desc.weight_paths = vec![broken];
    let err = build_encoder(&desc, DeviceClass::Cpu).err().unwrap();
    assert!(matches!(err, EncoderBuildError::Load { .. }), "{err:?}");
}
#[test]
fn main_model_errors_are_mapped() {
    let mut fx = plain();
    fx.desc.architecture = "modernbert".into();
    assert!(matches!(expect_err(& fx), EncoderBuildError::Incompatible { .. }));
    let fx = fixture_with(Some("embeddings.word_embeddings.weight"), &[]);
    assert_eq!(
        expect_err(& fx), EncoderBuildError::WeightNameMismatch { name :
        "embeddings.word_embeddings.weight".into() }
    );
    let mut fx = plain();
    fx.desc.config_path = fx._dir.0.join("absent.json");
    assert!(matches!(expect_err(& fx), EncoderBuildError::Load { .. }));
    let mut fx = plain();
    fx.desc.weight_paths.clear();
    assert!(matches!(expect_err(& fx), EncoderBuildError::Load { .. }));
}
#[test]
fn main_model_is_loaded_before_the_projection() {
    let mut fx = with_projection();
    fx.desc.architecture = "modernbert".into();
    fx.desc.projection_layers = vec![layer("absent.weight")];
    assert!(matches!(expect_err(& fx), EncoderBuildError::Incompatible { .. }));
}
#[test]
fn non_float_storage_dtype_is_incompatible() {
    let mut fx = plain();
    fx.desc.storage_dtype = DType::U8;
    let reason = incompatible_reason(expect_err(&fx));
    assert!(reason.contains("storage_dtype"), "{reason}");
}
#[test]
fn build_does_not_modify_the_descriptor() {
    let fx = with_projection();
    let before = fx.desc.clone();
    let _ = build_encoder(&fx.desc, DeviceClass::Cpu).unwrap();
    assert_eq!(fx.desc, before);
}
#[test]
fn parallel_builds_share_one_descriptor() {
    let fx = with_projection();
    let handles: Vec<_> = std::thread::scope(|s| {
        let joins: Vec<_> = (0..4)
            .map(|_| {
                s.spawn(|| {
                    let b = build_encoder(&fx.desc, DeviceClass::Cpu).unwrap();
                    b.encoder().output_dim()
                })
            })
            .collect();
        joins.into_iter().map(|j| j.join().unwrap()).collect()
    });
    assert_eq!(handles, vec![PROJ_OUT; 4]);
}
#[test]
fn built_encoder_shares_model_and_projection_with_encoder() {
    let fx = with_projection();
    let built = build_encoder(&fx.desc, DeviceClass::Cpu).unwrap();
    assert!(Arc::strong_count(built.model()) >= 2);
    assert!(Arc::strong_count(built.projection().unwrap()) >= 2);
}
