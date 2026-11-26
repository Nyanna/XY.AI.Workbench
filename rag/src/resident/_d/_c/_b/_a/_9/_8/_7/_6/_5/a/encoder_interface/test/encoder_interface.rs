use super::*;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc::TryRecvError;
use serde_json::json;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::a::memory_access::{
    AccessError, AccessRegion, RegionBackend,
};
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::a::model_descriptor::{
    DeviceClass, ModelDescriptor, ProjectionLayer,
};
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::_1::device_lane_event::{
    DeviceHandle, LaneConfig, LaneKind,
};
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::_1::layout::Layout;
const HIDDEN: usize = 16;
const LAYERS: usize = 2;
const HEADS: usize = 2;
const INTERMEDIATE: usize = 32;
const VOCAB: usize = 50;
const MAX_POS: usize = 32;
const SEQ: usize = 16;
const OUT: usize = 4;
static COUNTER: AtomicUsize = AtomicUsize::new(0);
const fn assert_traits<T: Send + Sync + 'static>() {}
const _: () = {
    assert_traits::<Encoder>();
    assert_traits::<CompletionEvent>();
    assert_traits::<EncoderError>();
};
struct TestDir(PathBuf);
impl TestDir {
    fn new() -> TestDir {
        let n = COUNTER.fetch_add(1, Ordering::SeqCst);
        let path = std::env::temp_dir()
            .join(format!("encoder_interface_{}_{n}", std::process::id()));
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
    let mut v = vec![
        ("embeddings.word_embeddings.weight".to_string(), vec![VOCAB, HIDDEN]),
        ("embeddings.position_embeddings.weight".to_string(), vec![MAX_POS, HIDDEN]),
        ("embeddings.token_type_embeddings.weight".to_string(), vec![2, HIDDEN]),
        ("embeddings.LayerNorm.weight".to_string(), vec![HIDDEN]),
        ("embeddings.LayerNorm.bias".to_string(), vec![HIDDEN]),
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
fn safetensors_bytes(tensors: &[(String, Vec<usize>, Vec<f32>)]) -> Vec<u8> {
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
    file
}
fn write_model(dir: &Path) {
    let mut rng = Rng(42);
    let weights: Vec<_> = tensor_shapes()
        .into_iter()
        .map(|(name, shape)| {
            let n: usize = shape.iter().product();
            let ln_weight = name.contains("LayerNorm") && name.ends_with(".weight");
            let values = (0..n)
                .map(|_| if ln_weight { 1.0 + rng.next() * 0.1 } else { rng.next() })
                .collect();
            (name, shape, values)
        })
        .collect();
    let config = json!(
        { "model_type" : "bert", "hidden_size" : HIDDEN, "num_hidden_layers" : LAYERS,
        "num_attention_heads" : HEADS, "intermediate_size" : INTERMEDIATE, "vocab_size" :
        VOCAB, "max_position_embeddings" : MAX_POS, "hidden_act" : "gelu",
        "layer_norm_eps" : 1e-12, "type_vocab_size" : 2 }
    );
    std::fs::write(dir.join("config.json"), config.to_string()).unwrap();
    std::fs::write(dir.join("model.safetensors"), safetensors_bytes(&weights)).unwrap();
}
fn load_model() -> (TestDir, Arc<TransformerModel>) {
    let dir = TestDir::new();
    write_model(&dir.0);
    let mut desc = ModelDescriptor::e5_small_default(&dir.0, DType::F32);
    desc.dimension = HIDDEN;
    desc.max_seq_len = SEQ;
    let model = TransformerModel::load(&desc, DeviceClass::Cpu).unwrap();
    (dir, Arc::new(model))
}
fn projection() -> Arc<Projection> {
    let mut rng = Rng(7);
    let w: Vec<f32> = (0..OUT * HIDDEN).map(|_| rng.next()).collect();
    let bytes = safetensors_bytes(&[("proj.weight".to_string(), vec![OUT, HIDDEN], w)]);
    let layer = ProjectionLayer {
        tensor_name: "proj.weight".to_string(),
    };
    Arc::new(Projection::from_bytes(&bytes, &layer, HIDDEN, OUT).unwrap())
}
#[derive(Debug)]
struct MemBackend(Mutex<Vec<u8>>);
impl RegionBackend for MemBackend {
    fn len(&self) -> usize {
        self.0.lock().unwrap().len()
    }
    fn read_raw(&self, offset: usize, dst: &mut [u8]) -> Result<(), AccessError> {
        let b = self.0.lock().unwrap();
        dst.copy_from_slice(&b[offset..offset + dst.len()]);
        Ok(())
    }
    fn write_raw(&self, offset: usize, src: &[u8]) -> Result<(), AccessError> {
        let mut b = self.0.lock().unwrap();
        b[offset..offset + src.len()].copy_from_slice(src);
        Ok(())
    }
}
fn region(len: usize) -> AccessRegion {
    AccessRegion::new(Arc::new(MemBackend(Mutex::new(vec![0; len]))))
}
fn lane() -> Lane {
    Lane::new(LaneConfig::preset(LaneKind::Query, DeviceHandle::Cpu), None)
}
fn tensor(shape: &[usize], dtype: DType) -> TensorRef {
    let layout = Layout::contiguous(shape, dtype);
    TensorRef::new(region(layout.byte_len()), layout).unwrap()
}
fn batch(ids: &[u32], mask: &[u32], rows: usize, seq: usize) -> TensorRef {
    let t = tensor(&[2, rows, seq], DType::U32);
    let bytes: Vec<u8> = ids.iter().chain(mask).flat_map(|v| v.to_le_bytes()).collect();
    if !bytes.is_empty() {
        t.region().write(0, &bytes).unwrap();
    }
    t
}
fn sample(rows: usize, seq: usize) -> (Vec<u32>, Vec<u32>) {
    let mut ids = Vec::new();
    let mut mask = Vec::new();
    for r in 0..rows {
        let valid = seq - r.min(seq - 1);
        for s in 0..seq {
            ids.push(((r * 7 + s * 3) % (VOCAB - 1) + 1) as u32);
            mask.push(u32::from(s < valid));
        }
    }
    (ids, mask)
}
fn read_all(t: &TensorRef) -> Vec<u8> {
    let mut buf = vec![0u8; t.layout().byte_len()];
    if !buf.is_empty() {
        t.region().read(t.offset_bytes(), &mut buf).unwrap();
    }
    buf
}
fn f32s(bytes: &[u8]) -> Vec<f32> {
    bytes.chunks_exact(4).map(|c| f32::from_le_bytes([c[0], c[1], c[2], c[3]])).collect()
}
fn max_diff(a: &[f32], b: &[f32]) -> f32 {
    assert_eq!(a.len(), b.len());
    a.iter().zip(b).map(|(x, y)| (x - y).abs()).fold(0.0, f32::max)
}
#[test]
fn error_display_strings() {
    let e = EncoderError::Incompatible {
        what: "input",
        expected: "[2, 4, 32]".into(),
        actual: "[2, 4, 8]".into(),
    };
    assert_eq!(
        e.to_string(), "inkompatibel: input: erwartet [2, 4, 32], tatsächlich [2, 4, 8]"
    );
    assert_eq!(EncoderError::Device("x".into()).to_string(), "Device-Error: x");
    assert_eq!(EncoderError::Memory("x".into()).to_string(), "Memory-Error: x");
    assert_eq!(
        EncoderError::Processing("x".into()).to_string(), "Verarbeitungsfehler: x"
    );
}
#[test]
fn f16_rounding() {
    let cases: [(f32, u16); 14] = [
        (0.0, 0x0000),
        (-0.0, 0x8000),
        (1.0, 0x3C00),
        (-2.0, 0xC000),
        (65504.0, 0x7BFF),
        (65520.0, 0x7C00),
        (1e-8, 0x0000),
        (2f32.powi(-24), 0x0001),
        (2f32.powi(-25), 0x0000),
        (3.0 * 2f32.powi(-25), 0x0002),
        (1.0 + 2f32.powi(-11), 0x3C00),
        (1.0 + 3.0 * 2f32.powi(-11), 0x3C02),
        (f32::INFINITY, 0x7C00),
        (f32::NEG_INFINITY, 0xFC00),
    ];
    for (v, bits) in cases {
        assert_eq!(f32_to_f16_bits3(v), bits, "value {v}");
    }
    let nan = f32_to_f16_bits3(f32::NAN);
    assert_eq!(nan & 0x7C00, 0x7C00);
    assert_ne!(nan & 0x03FF, 0);
}
#[test]
fn bf16_rounding() {
    assert_eq!(f32_to_bf16_bits3(1.0), 0x3F80);
    assert_eq!(f32_to_bf16_bits3(f32::from_bits(0x3F80_8000)), 0x3F80);
    assert_eq!(f32_to_bf16_bits3(f32::from_bits(0x3F81_8000)), 0x3F82);
    assert_eq!(f32_to_bf16_bits3(f32::from_bits(0x3F80_8001)), 0x3F81);
    assert_eq!(f32_to_bf16_bits3(f32::INFINITY), 0x7F80);
    assert_eq!(f32_to_bf16_bits3(f32::NEG_INFINITY), 0xFF80);
    let nan = f32_to_bf16_bits3(f32::NAN);
    assert_eq!(nan & 0x7F80, 0x7F80);
    assert_ne!(nan & 0x007F, 0);
}
#[test]
fn new_validates_arguments() {
    let (_d, model) = load_model();
    for dt in [DType::U8, DType::U32, DType::I64] {
        let r = Encoder::new(model.clone(), None, Pooling::Mean, Normalization::L2, dt);
        assert!(matches!(r, Err(EncoderError::Incompatible { .. })));
    }
    let bad = Arc::new(
        Projection::from_bytes(
                &safetensors_bytes(&[("p".to_string(), vec![2, 3], vec![0.0; 6])]),
                &ProjectionLayer {
                    tensor_name: "p".to_string(),
                },
                3,
                2,
            )
            .unwrap(),
    );
    let r = Encoder::new(
        model.clone(),
        Some(bad),
        Pooling::Mean,
        Normalization::L2,
        DType::F32,
    );
    assert!(matches!(r, Err(EncoderError::Incompatible { .. })));
    let e = Encoder::new(
            model.clone(),
            None,
            Pooling::Mean,
            Normalization::L2,
            DType::F16,
        )
        .unwrap();
    assert_eq!(e.output_dim(), HIDDEN);
    assert_eq!(e.storage_dtype(), DType::F16);
    let p = Encoder::new(
            model,
            Some(projection()),
            Pooling::PerToken,
            Normalization::None,
            DType::BF16,
        )
        .unwrap();
    assert_eq!(p.output_dim(), OUT);
}
#[test]
fn precheck_order_and_no_side_effects() {
    let (_d, model) = load_model();
    let enc = Encoder::new(model, None, Pooling::Mean, Normalization::L2, DType::F32)
        .unwrap();
    let lane = lane();
    let good_in = batch(&[1; 4], &[1; 4], 2, 2);
    let good_out = tensor(&[2, HIDDEN], DType::F32);
    let cases: Vec<(TensorRef, TensorRef, &str, &str, &str)> = vec![
        (tensor(& [2, 2, 3], DType::U32), tensor(& [2, HIDDEN], DType::U8), "input",
        "[2, 2, 2]", "[2, 2, 3]"), (tensor(& [2, 2, 2], DType::F32), tensor(& [3,
        HIDDEN], DType::F32), "input", "U32", "F32"), (good_in.clone(), tensor(& [3,
        HIDDEN], DType::U8), "output", "[2, 16]", "[3, 16]"), (good_in.clone(), tensor(&
        [2, HIDDEN], DType::F16), "output", "F32", "F16"),
    ];
    for (i, o, what, exp, act) in cases {
        let (tx, rx) = oneshot();
        match enc.encode_async(&lane, &i, &o, 2, 2, vec![tx]) {
            Err(EncoderError::Incompatible { what: w, expected, actual }) => {
                assert_eq!((w, expected.as_str(), actual.as_str()), (what, exp, act));
            }
            other => panic!("unexpected {:?}", other.map(| _ | ())),
        }
        assert_eq!(rx.try_recv(), Err(TryRecvError::Disconnected));
    }
    let layout = Layout::new(vec![2, 2, 2], vec![32, 8, 4], DType::U32).unwrap();
    let strided = TensorRef::new(region(128), layout).unwrap();
    match enc.encode_async(&lane, &strided, &good_out, 2, 2, vec![]) {
        Err(EncoderError::Incompatible { what, expected, actual }) => {
            assert_eq!(what, "input");
            assert_eq!(expected, "contiguous");
            assert!(actual.contains("[32, 8, 4]"));
        }
        other => panic!("unexpected {:?}", other.map(| _ | ())),
    }
    assert!(read_all(& good_out).iter().all(| b | * b == 0));
}
#[test]
fn mean_pooling_matches_reference() {
    let (_d, model) = load_model();
    let enc = Encoder::new(
            model.clone(),
            None,
            Pooling::Mean,
            Normalization::L2,
            DType::F32,
        )
        .unwrap();
    let (rows, seq) = (3, 5);
    let (ids, mask) = sample(rows, seq);
    let input = batch(&ids, &mask, rows, seq);
    let output = tensor(&[rows, HIDDEN], DType::F32);
    let (tx, rx) = oneshot();
    let (tx2, rx2) = oneshot();
    let done = enc
        .encode_async(&lane(), &input, &output, rows, seq, vec![tx, tx2])
        .unwrap();
    done.wait().unwrap();
    assert_eq!(done.is_complete(), Ok(true));
    assert_eq!(rx.try_recv(), Ok(Ok(())));
    assert_eq!(rx2.try_recv(), Ok(Ok(())));
    assert_eq!(done.wait(), Ok(()));
    let hidden = model.encode(&ids, &mask, rows, seq).unwrap();
    let mask8: Vec<u8> = mask.iter().map(|&m| m as u8).collect();
    let reference = PoolNorm::new(Pooling::Mean, Normalization::L2, HIDDEN)
        .unwrap()
        .apply(&hidden, &mask8, rows, seq)
        .unwrap();
    let got = f32s(&read_all(&output));
    assert!(max_diff(& got, & reference.vectors) < 1e-6);
}
#[test]
fn projection_without_pooling_in_f16_and_offset() {
    let (_d, model) = load_model();
    let proj = projection();
    let enc = Encoder::new(
            model.clone(),
            Some(proj.clone()),
            Pooling::PerToken,
            Normalization::None,
            DType::F16,
        )
        .unwrap();
    let (rows, seq) = (2, 4);
    let (ids, mask) = sample(rows, seq);
    let input = batch(&ids, &mask, rows, seq);
    let layout = Layout::contiguous(&[rows, seq, OUT], DType::F16);
    let offset = 8;
    let output = TensorRef::with_offset(
            region(offset + layout.byte_len() + 8),
            layout,
            offset,
        )
        .unwrap();
    let done = enc.encode_async(&lane(), &input, &output, rows, seq, vec![]).unwrap();
    done.wait().unwrap();
    let hidden = model.encode(&ids, &mask, rows, seq).unwrap();
    let tokens: Vec<Vec<f32>> = hidden.chunks(HIDDEN).map(<[f32]>::to_vec).collect();
    let projected: Vec<f32> = proj
        .apply(&tokens)
        .unwrap()
        .into_iter()
        .flatten()
        .collect();
    let mask8: Vec<u8> = mask.iter().map(|&m| m as u8).collect();
    let pooled = PoolNorm::new(Pooling::PerToken, Normalization::None, OUT)
        .unwrap()
        .apply(&projected, &mask8, rows, seq)
        .unwrap();
    let expected = to_storage_bytes(&pooled.vectors, DType::F16).unwrap();
    assert_eq!(read_all(& output), expected);
    let mut head = [0xFFu8; 8];
    output.region().read(0, &mut head).unwrap();
    assert_eq!(head, [0u8; 8]);
}
#[test]
fn processing_error_is_distributed_once() {
    let (_d, model) = load_model();
    let enc = Encoder::new(model, None, Pooling::Mean, Normalization::L2, DType::F32)
        .unwrap();
    let input = batch(&[VOCAB as u32 + 3, 1], &[1, 1], 1, 2);
    let output = tensor(&[1, HIDDEN], DType::F32);
    let (tx, rx) = oneshot();
    let (tx_dropped, rx_dropped) = oneshot();
    drop(rx_dropped);
    let done = enc
        .encode_async(&lane(), &input, &output, 1, 2, vec![tx_dropped, tx])
        .unwrap();
    let clone = done.clone();
    let first = done.wait();
    assert!(matches!(first, Err(EncoderError::Processing(_))), "{first:?}");
    assert_eq!(clone.wait(), Ok(()));
    assert_eq!(done.wait(), Ok(()));
    assert_eq!(done.is_complete(), Ok(true));
    assert!(done.event().wait().is_ok());
    assert!(matches!(rx.try_recv(), Ok(Err(EncoderError::Processing(_)))));
    assert!(read_all(& output).iter().all(| b | * b == 0));
}
#[test]
fn is_complete_does_not_consume_error() {
    let (_d, model) = load_model();
    let enc = Encoder::new(model, None, Pooling::Mean, Normalization::L2, DType::F32)
        .unwrap();
    let input = batch(&[VOCAB as u32, 1], &[1, 1], 1, 2);
    let output = tensor(&[1, HIDDEN], DType::F32);
    let done = enc.encode_async(&lane(), &input, &output, 1, 2, vec![]).unwrap();
    done.event().wait().unwrap();
    assert_eq!(done.is_complete(), Ok(true));
    assert!(done.wait().is_err());
    assert_eq!(done.wait(), Ok(()));
}
#[test]
fn runs_on_calling_thread_and_is_complete_on_return() {
    let (_d, model) = load_model();
    let enc = Encoder::new(model, None, Pooling::Mean, Normalization::L2, DType::F32)
        .unwrap();
    let (ids, mask) = sample(2, 3);
    let input = batch(&ids, &mask, 2, 3);
    let output = tensor(&[2, HIDDEN], DType::F32);
    let (tx, rx) = oneshot();
    let done = enc.encode_async(&lane(), &input, &output, 2, 3, vec![tx]).unwrap();
    assert_eq!(done.is_complete(), Ok(true));
    assert_eq!(rx.try_recv(), Ok(Ok(())));
    assert!(read_all(& output).iter().any(| b | * b != 0));
}
#[test]
fn empty_batches_are_accepted() {
    let (_d, model) = load_model();
    let enc = Encoder::new(model, None, Pooling::Mean, Normalization::L2, DType::F32)
        .unwrap();
    for (rows, seq) in [(0usize, 3usize), (2, 0), (0, 0)] {
        let input = batch(&[], &[], rows, seq);
        let output = tensor(&[rows, HIDDEN], DType::F32);
        let (tx, rx) = oneshot();
        let done = enc
            .encode_async(&lane(), &input, &output, rows, seq, vec![tx])
            .unwrap();
        let waited = done.wait();
        let sent = rx.try_recv();
        if rows == 0 {
            assert_eq!(waited, Ok(()));
            assert_eq!(sent, Ok(Ok(())));
        } else {
            assert_eq!(sent.is_ok(), true);
        }
    }
}
#[test]
fn concurrent_calls_share_encoder_and_lane() {
    let (_d, model) = load_model();
    let enc = Arc::new(
        Encoder::new(model, None, Pooling::Mean, Normalization::L2, DType::F32).unwrap(),
    );
    let lane = Arc::new(lane());
    let handles: Vec<_> = (0..4)
        .map(|_| {
            let (enc, lane) = (enc.clone(), lane.clone());
            std::thread::spawn(move || {
                let (ids, mask) = sample(2, 4);
                let input = batch(&ids, &mask, 2, 4);
                let output = tensor(&[2, HIDDEN], DType::F32);
                let done = enc
                    .encode_async(&lane, &input, &output, 2, 4, vec![])
                    .unwrap();
                done.wait().unwrap();
                read_all(&output)
            })
        })
        .collect();
    let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();
    assert!(results.windows(2).all(| w | w[0] == w[1]));
    assert!(results[0].iter().any(| b | * b != 0));
}
