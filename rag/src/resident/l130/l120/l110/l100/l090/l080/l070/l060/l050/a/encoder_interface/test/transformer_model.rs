use super::*;
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};
use serde_json::json;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::a::model_descriptor::ComputeDTypes;
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
            .join(format!("transformer_model_{}_{n}", std::process::id()));
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
fn tensor_shapes(prefix: &str) -> Vec<(String, Vec<usize>)> {
    let p = |s: &str| format!("{prefix}{s}");
    let mut v = vec![
        (p("embeddings.word_embeddings.weight"), vec![VOCAB, HIDDEN]),
        (p("embeddings.position_embeddings.weight"), vec![MAX_POS, HIDDEN]),
        (p("embeddings.token_type_embeddings.weight"), vec![2, HIDDEN]),
        (p("embeddings.LayerNorm.weight"), vec![HIDDEN]),
        (p("embeddings.LayerNorm.bias"), vec![HIDDEN]),
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
            v.push((p(&format!("encoder.layer.{i}.{part}.weight")), vec![out, inp]));
            v.push((p(&format!("encoder.layer.{i}.{part}.bias")), vec![out]));
        }
        for part in ["attention.output.LayerNorm", "output.LayerNorm"] {
            v.push((p(&format!("encoder.layer.{i}.{part}.weight")), vec![HIDDEN]));
            v.push((p(&format!("encoder.layer.{i}.{part}.bias")), vec![HIDDEN]));
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
fn weights(prefix: &str, omit: Option<&str>) -> Vec<(String, Vec<usize>, Vec<f32>)> {
    let mut rng = Rng(42);
    tensor_shapes(prefix)
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
fn fixture_with(
    prefix: &str,
    omit: Option<&str>,
    config: &serde_json::Value,
) -> Fixture {
    let dir = TestDir::new();
    std::fs::write(dir.0.join("config.json"), config.to_string()).unwrap();
    write_safetensors(&dir.0.join("model.safetensors"), &weights(prefix, omit));
    let mut desc = ModelDescriptor::e5_small_default(&dir.0, DType::F32);
    desc.dimension = HIDDEN;
    desc.max_seq_len = SEQ;
    Fixture { _dir: dir, desc }
}
fn fixture() -> Fixture {
    fixture_with("", None, &config_json())
}
fn with_dtype(desc: &ModelDescriptor, dtype: DType) -> ModelDescriptor {
    let mut d = desc.clone();
    d.compute_dtypes = ComputeDTypes {
        cpu: dtype,
        gpu: dtype,
    };
    d
}
fn load_cpu(fx: &Fixture) -> TransformerModel {
    TransformerModel::load(&fx.desc, DeviceClass::Cpu).unwrap()
}
fn inputs(rows: usize, seq: usize) -> (Vec<u32>, Vec<u32>) {
    let ids = (0..rows * seq)
        .map(|i| ((i * 7 + (i / seq) * 3 + 1) % VOCAB) as u32)
        .collect();
    (ids, vec![1; rows * seq])
}
fn max_diff(a: &[f32], b: &[f32]) -> f32 {
    assert_eq!(a.len(), b.len());
    a.iter().zip(b).map(|(x, y)| (x - y).abs()).fold(0.0, f32::max)
}
fn config_with(key: &str, value: serde_json::Value) -> serde_json::Value {
    let mut c = config_json();
    c[key] = value;
    c
}
#[test]
fn model_is_send_sync_static() {
    fn assert_traits<T: Send + Sync + 'static>() {}
    assert_traits::<TransformerModel>();
}
#[test]
fn load_exposes_descriptor_values() {
    let fx = fixture();
    let model = load_cpu(&fx);
    assert_eq!(model.dimension(), HIDDEN);
    assert_eq!(model.max_seq_len(), SEQ);
    assert_eq!(model.device(), DeviceHandle::Cpu);
    assert_eq!(model.compute_dtype(), DType::F32);
}
#[test]
fn wrong_model_type_is_incompatible() {
    let fx = fixture_with("", None, &config_with("model_type", json!("xlm-roberta")));
    match TransformerModel::load(&fx.desc, DeviceClass::Cpu) {
        Err(TransformerError::Incompatible { reason }) => {
            assert!(reason.contains("xlm-roberta"), "{reason}");
            assert!(reason.contains("bert"), "{reason}");
        }
        other => panic!("unexpected: {:?}", other.err()),
    }
}
#[test]
fn wrong_hidden_size_is_incompatible() {
    let mut fx = fixture();
    fx.desc.dimension = 24;
    match TransformerModel::load(&fx.desc, DeviceClass::Cpu) {
        Err(TransformerError::Incompatible { reason }) => {
            assert!(reason.contains("16") && reason.contains("24"), "{reason}");
        }
        other => panic!("unexpected: {:?}", other.err()),
    }
}
#[test]
fn too_large_max_seq_len_is_incompatible() {
    let mut fx = fixture();
    fx.desc.max_seq_len = MAX_POS + 1;
    match TransformerModel::load(&fx.desc, DeviceClass::Cpu) {
        Err(TransformerError::Incompatible { reason }) => {
            assert!(reason.contains("33") && reason.contains("32"), "{reason}");
        }
        other => panic!("unexpected: {:?}", other.err()),
    }
}
#[test]
fn wrong_architecture_is_incompatible() {
    let mut fx = fixture();
    fx.desc.architecture = "roberta".to_string();
    match TransformerModel::load(&fx.desc, DeviceClass::Cpu) {
        Err(TransformerError::Incompatible { reason }) => {
            assert!(reason.contains("roberta"))
        }
        other => panic!("unexpected: {:?}", other.err()),
    }
}
#[test]
fn indivisible_heads_are_incompatible() {
    let fx = fixture_with("", None, &config_with("num_attention_heads", json!(3)));
    assert!(
        matches!(TransformerModel::load(& fx.desc, DeviceClass::Cpu),
        Err(TransformerError::Incompatible { .. }))
    );
}
#[test]
fn zero_heads_are_a_load_error() {
    let fx = fixture_with("", None, &config_with("num_attention_heads", json!(0)));
    assert!(
        matches!(TransformerModel::load(& fx.desc, DeviceClass::Cpu),
        Err(TransformerError::Load { .. }))
    );
}
#[test]
fn non_float_dtype_is_incompatible() {
    let fx = fixture();
    for dtype in [DType::U8, DType::U32, DType::I64] {
        let desc = with_dtype(&fx.desc, dtype);
        match TransformerModel::load(&desc, DeviceClass::Cpu) {
            Err(TransformerError::Incompatible { reason }) => {
                assert!(reason.contains(& format!("{dtype:?}")), "{reason}");
            }
            other => panic!("unexpected: {:?}", other.err()),
        }
    }
}
#[test]
fn missing_tensor_is_reported_by_name() {
    let name = "encoder.layer.1.output.dense.bias";
    let fx = fixture_with("", Some(name), &config_json());
    assert_eq!(
        TransformerModel::load(& fx.desc, DeviceClass::Cpu).err(),
        Some(TransformerError::WeightNameMismatch { name : name.to_string() })
    );
}
#[test]
fn missing_tensor_reports_prefixed_name() {
    let name = "bert.embeddings.LayerNorm.weight";
    let fx = fixture_with("bert.", Some(name), &config_json());
    assert_eq!(
        TransformerModel::load(& fx.desc, DeviceClass::Cpu).err(),
        Some(TransformerError::WeightNameMismatch { name : name.to_string() })
    );
}
#[test]
fn first_missing_tensor_wins() {
    let fx = fixture_with("", Some("embeddings.word_embeddings.weight"), &config_json());
    assert_eq!(
        TransformerModel::load(& fx.desc, DeviceClass::Cpu).err(),
        Some(TransformerError::WeightNameMismatch { name :
        "embeddings.word_embeddings.weight".to_string() })
    );
}
#[test]
fn unreadable_config_is_a_load_error() {
    let mut fx = fixture();
    fx.desc.config_path = fx.desc.config_path.with_file_name("missing.json");
    match TransformerModel::load(&fx.desc, DeviceClass::Cpu) {
        Err(TransformerError::Load { reason }) => {
            assert!(reason.contains("missing.json"))
        }
        other => panic!("unexpected: {:?}", other.err()),
    }
}
#[test]
fn invalid_json_and_missing_fields_are_load_errors() {
    let fx = fixture();
    std::fs::write(&fx.desc.config_path, "{ not json").unwrap();
    assert!(
        matches!(TransformerModel::load(& fx.desc, DeviceClass::Cpu),
        Err(TransformerError::Load { .. }))
    );
    let mut config = config_json();
    config.as_object_mut().unwrap().remove("intermediate_size");
    std::fs::write(&fx.desc.config_path, config.to_string()).unwrap();
    match TransformerModel::load(&fx.desc, DeviceClass::Cpu) {
        Err(TransformerError::Load { reason }) => {
            assert!(reason.contains("intermediate_size"))
        }
        other => panic!("unexpected: {:?}", other.err()),
    }
    std::fs::write(
            &fx.desc.config_path,
            config_with("vocab_size", json!("x")).to_string(),
        )
        .unwrap();
    assert!(
        matches!(TransformerModel::load(& fx.desc, DeviceClass::Cpu),
        Err(TransformerError::Load { .. }))
    );
}
#[test]
fn unknown_hidden_act_is_a_load_error() {
    let fx = fixture_with("", None, &config_with("hidden_act", json!("swish")));
    assert!(
        matches!(TransformerModel::load(& fx.desc, DeviceClass::Cpu),
        Err(TransformerError::Load { .. }))
    );
}
#[test]
fn optional_config_fields_use_defaults() {
    let mut config = config_json();
    let obj = config.as_object_mut().unwrap();
    obj.remove("hidden_act");
    obj.remove("layer_norm_eps");
    obj.remove("type_vocab_size");
    let fx = fixture_with("", None, &config);
    let model = load_cpu(&fx);
    let (ids, mask) = inputs(1, 4);
    assert_eq!(model.encode(& ids, & mask, 1, 4).unwrap().len(), 4 * HIDDEN);
}
#[test]
fn empty_weight_paths_and_bad_files_are_load_errors() {
    let mut fx = fixture();
    let good = fx.desc.weight_paths.clone();
    fx.desc.weight_paths.clear();
    assert!(
        matches!(TransformerModel::load(& fx.desc, DeviceClass::Cpu),
        Err(TransformerError::Load { .. }))
    );
    fx.desc.weight_paths = vec![good[0].with_file_name("absent.safetensors")];
    assert!(
        matches!(TransformerModel::load(& fx.desc, DeviceClass::Cpu),
        Err(TransformerError::Load { .. }))
    );
    let broken = good[0].with_file_name("broken.safetensors");
    std::fs::write(&broken, [1u8, 2, 3]).unwrap();
    fx.desc.weight_paths = vec![broken];
    assert!(
        matches!(TransformerModel::load(& fx.desc, DeviceClass::Cpu),
        Err(TransformerError::Load { .. }))
    );
}
#[test]
fn shape_mismatch_in_weights_is_a_load_error() {
    let fx = fixture_with("", None, &config_with("intermediate_size", json!(48)));
    assert!(
        matches!(TransformerModel::load(& fx.desc, DeviceClass::Cpu),
        Err(TransformerError::Load { .. }))
    );
}
#[test]
fn tensors_may_be_spread_over_several_files() {
    let dir = TestDir::new();
    std::fs::write(dir.0.join("config.json"), config_json().to_string()).unwrap();
    let all = weights("", None);
    let (first, second) = all.split_at(all.len() / 2);
    write_safetensors(&dir.0.join("a.safetensors"), first);
    write_safetensors(&dir.0.join("b.safetensors"), second);
    let mut desc = ModelDescriptor::e5_small_default(&dir.0, DType::F32);
    desc.dimension = HIDDEN;
    desc.max_seq_len = SEQ;
    desc.weight_paths = vec![dir.0.join("a.safetensors"), dir.0.join("b.safetensors")];
    let split = TransformerModel::load(&desc, DeviceClass::Cpu).unwrap();
    let single = load_cpu(&fixture());
    let (ids, mask) = inputs(2, 5);
    let a = split.encode(&ids, &mask, 2, 5).unwrap();
    let b = single.encode(&ids, &mask, 2, 5).unwrap();
    assert!(max_diff(& a, & b) < 1e-6);
}
#[test]
fn bert_prefix_gives_same_result() {
    let plain = load_cpu(&fixture());
    let prefixed_fx = fixture_with("bert.", None, &config_json());
    let prefixed = load_cpu(&prefixed_fx);
    let (ids, mask) = inputs(2, 6);
    let a = plain.encode(&ids, &mask, 2, 6).unwrap();
    let b = prefixed.encode(&ids, &mask, 2, 6).unwrap();
    assert!(max_diff(& a, & b) < 1e-6);
}
#[test]
fn output_length_and_layout() {
    let model = load_cpu(&fixture());
    let (rows, seq) = (3, 5);
    let (ids, mask) = inputs(rows, seq);
    let out = model.encode(&ids, &mask, rows, seq).unwrap();
    assert_eq!(out.len(), rows * seq * HIDDEN);
    assert!(out.iter().all(| v | v.is_finite()));
    assert!(max_diff(& out[..HIDDEN], & out[HIDDEN..2 * HIDDEN]) > 1e-4);
    for r in 0..rows {
        let single = model
            .encode(&ids[r * seq..(r + 1) * seq], &mask[r * seq..(r + 1) * seq], 1, seq)
            .unwrap();
        let chunk = &out[r * seq * HIDDEN..(r + 1) * seq * HIDDEN];
        assert!(max_diff(chunk, & single) < 1e-5, "row {r}");
    }
}
#[test]
fn masked_token_does_not_affect_valid_tokens() {
    let model = load_cpu(&fixture());
    let (rows, seq) = (2, 6);
    let (ids, mut mask) = inputs(rows, seq);
    mask[4] = 0;
    mask[5] = 0;
    let base = model.encode(&ids, &mask, rows, seq).unwrap();
    let mut changed = ids.clone();
    changed[4] = (ids[4] + 11) % VOCAB as u32;
    changed[5] = (ids[5] + 5) % VOCAB as u32;
    let other = model.encode(&changed, &mask, rows, seq).unwrap();
    assert!(max_diff(& base[..4 * HIDDEN], & other[..4 * HIDDEN]) < 1e-5);
    assert!(max_diff(& base[seq * HIDDEN..], & other[seq * HIDDEN..]) < 1e-6);
    assert!(base.iter().all(| v | v.is_finite()));
    assert!(other.iter().all(| v | v.is_finite()));
}
#[test]
fn mask_changes_result_for_valid_keys() {
    let model = load_cpu(&fixture());
    let (ids, mask) = inputs(1, 6);
    let full = model.encode(&ids, &mask, 1, 6).unwrap();
    let mut partial_mask = mask.clone();
    partial_mask[5] = 0;
    let partial = model.encode(&ids, &partial_mask, 1, 6).unwrap();
    assert!(max_diff(& full[..HIDDEN], & partial[..HIDDEN]) > 1e-6);
}
#[test]
fn fully_masked_row_is_finite() {
    let model = load_cpu(&fixture());
    let (ids, _) = inputs(1, 4);
    let out = model.encode(&ids, &[0; 4], 1, 4).unwrap();
    assert_eq!(out.len(), 4 * HIDDEN);
    assert!(out.iter().all(| v | v.is_finite()));
}
#[test]
fn nonzero_mask_values_count_as_valid() {
    let model = load_cpu(&fixture());
    let (ids, mask) = inputs(1, 5);
    let scaled: Vec<u32> = mask.iter().map(|m| m * 7).collect();
    let a = model.encode(&ids, &mask, 1, 5).unwrap();
    let b = model.encode(&ids, &scaled, 1, 5).unwrap();
    assert!(max_diff(& a, & b) < 1e-7);
}
#[test]
fn reduced_precision_returns_f32_vectors() {
    let fx = fixture();
    let reference = load_cpu(&fx);
    let (rows, seq) = (2, 6);
    let (ids, mask) = inputs(rows, seq);
    let expected = reference.encode(&ids, &mask, rows, seq).unwrap();
    for (dtype, tolerance) in [(DType::F16, 0.1f32), (DType::BF16, 0.3f32)] {
        let model = TransformerModel::load(
                &with_dtype(&fx.desc, dtype),
                DeviceClass::Cpu,
            )
            .unwrap();
        assert_eq!(model.compute_dtype(), dtype);
        let out = match model.encode(&ids, &mask, rows, seq) {
            Ok(out) => out,
            Err(TransformerError::EncodingFailed { .. }) if dtype == DType::BF16 => {
                continue;
            }
            Err(e) => panic!("{dtype:?}: {e}"),
        };
        assert_eq!(out.len(), rows * seq * HIDDEN);
        assert!(out.iter().all(| v | v.is_finite()));
        assert!(max_diff(& out, & expected) < tolerance, "{dtype:?}");
    }
}
#[test]
fn invalid_inputs_are_encoding_errors() {
    let model = load_cpu(&fixture());
    let (ids, mask) = inputs(2, 4);
    let is_err = |r: Result<Vec<f32>, TransformerError>| {
        matches!(r, Err(TransformerError::EncodingFailed { .. }))
    };
    assert!(is_err(model.encode(& ids[1..], & mask, 2, 4)));
    assert!(is_err(model.encode(& ids, & mask[1..], 2, 4)));
    assert!(is_err(model.encode(& ids, & mask, 2, 5)));
    assert!(is_err(model.encode(& ids, & mask, 3, 4)));
    let long_ids = vec![1u32; SEQ + 1];
    assert!(is_err(model.encode(& long_ids, & long_ids, 1, SEQ + 1)));
    let mut bad = ids.clone();
    bad[3] = VOCAB as u32;
    match model.encode(&bad, &mask, 2, 4) {
        Err(TransformerError::EncodingFailed { reason }) => {
            assert!(reason.contains(& VOCAB.to_string()), "{reason}");
        }
        other => panic!("unexpected: {other:?}"),
    }
    assert!(is_err(model.encode(& [], & [], usize::MAX, 2)));
    assert!(is_err(model.encode(& [], & [], 2, usize::MAX)));
}
#[test]
fn empty_batches_return_empty_vectors() {
    let model = load_cpu(&fixture());
    assert_eq!(model.encode(& [], & [], 0, 0).unwrap(), Vec::< f32 >::new());
    assert_eq!(model.encode(& [], & [], 0, 4).unwrap(), Vec::< f32 >::new());
    assert_eq!(model.encode(& [], & [], 3, 0).unwrap(), Vec::< f32 >::new());
}
#[test]
fn full_length_sequences_are_accepted() {
    let model = load_cpu(&fixture());
    let (ids, mask) = inputs(1, SEQ);
    assert_eq!(model.encode(& ids, & mask, 1, SEQ).unwrap().len(), SEQ * HIDDEN);
}
#[test]
fn encoding_is_deterministic_and_thread_safe() {
    let model = load_cpu(&fixture());
    let (rows, seq) = (2, 7);
    let (ids, mask) = inputs(rows, seq);
    let baseline = model.encode(&ids, &mask, rows, seq).unwrap();
    assert_eq!(baseline, model.encode(& ids, & mask, rows, seq).unwrap());
    std::thread::scope(|scope| {
        let handles: Vec<_> = (0..4)
            .map(|_| scope.spawn(|| model.encode(&ids, &mask, rows, seq).unwrap()))
            .collect();
        for h in handles {
            assert!(max_diff(& h.join().unwrap(), & baseline) < 1e-6);
        }
    });
}
#[test]
fn models_on_one_file_coexist() {
    let fx = fixture();
    let a = load_cpu(&fx);
    let b = TransformerModel::load(&with_dtype(&fx.desc, DType::F16), DeviceClass::Cpu)
        .unwrap();
    let (ids, mask) = inputs(1, 4);
    assert_eq!(a.encode(& ids, & mask, 1, 4).unwrap().len(), 4 * HIDDEN);
    assert_eq!(b.encode(& ids, & mask, 1, 4).unwrap().len(), 4 * HIDDEN);
}
#[test]
fn error_display_strings() {
    assert_eq!(
        TransformerError::Incompatible { reason : "x".into() } .to_string(),
        "inkompatible Modellkonfiguration: x"
    );
    assert_eq!(
        TransformerError::WeightNameMismatch { name : "n".into() } .to_string(),
        "Gewichtsname passt nicht: n"
    );
    assert_eq!(
        TransformerError::Load { reason : "r".into() } .to_string(), "Load-Error: r"
    );
    assert_eq!(
        TransformerError::EncodingFailed { reason : "e".into() } .to_string(),
        "Encoding fehlgeschlagen: e"
    );
}
mod gpu {
    use super::*;
    #[test]
    fn gpu_matches_cpu() {
        let fx = fixture();
        let gpu = match TransformerModel::load(&fx.desc, DeviceClass::Gpu) {
            Ok(m) => m,
            Err(TransformerError::Load { .. }) => return,
            Err(e) => panic!("unexpected: {e}"),
        };
        assert_eq!(gpu.device(), DeviceHandle::Gpu(0));
        let cpu = load_cpu(&fx);
        let (rows, seq) = (2, 6);
        let (ids, mut mask) = inputs(rows, seq);
        mask[5] = 0;
        let a = gpu.encode(&ids, &mask, rows, seq).unwrap();
        let b = cpu.encode(&ids, &mask, rows, seq).unwrap();
        assert_eq!(a.len(), b.len());
        assert!(max_diff(& a, & b) < 1e-3);
    }
}
