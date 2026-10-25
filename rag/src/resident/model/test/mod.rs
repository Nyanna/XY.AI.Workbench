use super::*;
use crate::resident::memory::{ArenaKind, ArenaSizes, DeviceMemory};
use crate::resident::subengine::{CpuLane, LaneClass, LaneConfig, RecordingLane};
const DIM: usize = 4;
/// Hidden state of token `t` in row `b` is `[b + 1, t, 1, 0]`.
struct FakeBackbone;
impl Backbone for FakeBackbone {
    fn dim(&self) -> usize {
        DIM
    }
    fn forward(
        &self,
        ids: &[u32],
        _mask: &[u32],
        batch: usize,
        seq: usize,
    ) -> Result<Vec<f32>, ModelError> {
        assert_eq!(ids.len(), batch * seq);
        Ok(
            (0..batch)
                .flat_map(|b| {
                    (0..seq).flat_map(move |t| [b as f32 + 1.0, t as f32, 1.0, 0.0])
                })
                .collect(),
        )
    }
}
struct Fixture {
    mem: Arc<CpuMemoryManager>,
    lane_cfg: LaneConfig,
}
fn fixture() -> Fixture {
    let sizes = ArenaSizes {
        weights: 1 << 10,
        index_segment: 1 << 10,
        workspace: 1 << 16,
        staging: 1 << 16,
        param_block: 1 << 12,
        mapped: 0,
    };
    let mem = Arc::new(CpuMemoryManager::new(sizes, 1));
    let scratch = mem.alloc_in(ArenaKind::Workspace, 1024, 16).unwrap();
    Fixture {
        mem,
        lane_cfg: LaneConfig::new(scratch),
    }
}
fn put_tokens(f: &Fixture, batch: &Batch) -> TensorRef {
    let layout = Layout::contiguous(&[2, batch.rows(), batch.seq_len], DType::U32);
    let region = f
        .mem
        .alloc_in(ArenaKind::Staging, layout.byte_len() as u64, 16)
        .unwrap();
    f.mem.write(region, &batch.pack()).unwrap();
    TensorRef { region, layout }
}
fn out_ref(f: &Fixture, shape: &[usize], dtype: DType) -> TensorRef {
    let layout = Layout::contiguous(shape, dtype);
    let region = f
        .mem
        .alloc_in(ArenaKind::Workspace, layout.byte_len() as u64, 16)
        .unwrap();
    TensorRef { region, layout }
}
fn read_f32(f: &Fixture, out: &TensorRef) -> Vec<f32> {
    let mut raw = vec![0u8; out.layout.byte_len()];
    f.mem.read(out.region, &mut raw).unwrap();
    raw.chunks_exact(4).map(|b| f32::from_le_bytes([b[0], b[1], b[2], b[3]])).collect()
}
fn batch(seqs: &[Vec<u32>]) -> Batch {
    plan_batches(
            seqs,
            &BatchConfig {
                token_budget: 1 << 20,
                max_seq_len: 512,
                pad_token_id: 0,
            },
        )
        .remove(0)
}
#[test]
fn mean_pooled_rows_are_unit_length() {
    let f = fixture();
    let enc = PooledEncoder::new(FakeBackbone, None, Pooling::MeanMasked, DType::F32)
        .unwrap();
    let b = batch(&[vec![1, 2, 3], vec![4]]);
    let (tokens, out) = (put_tokens(&f, &b), out_ref(&f, &[2, DIM], DType::F32));
    let lane = RecordingLane::new(LaneClass::Query, f.lane_cfg);
    let mem: Arc<dyn RegionIo> = f.mem.clone();
    enc.encode_into(&mem, &tokens, &out, &lane).unwrap().wait().unwrap();
    let v = read_f32(&f, &out);
    for row in v.chunks_exact(DIM) {
        assert!((row.iter().map(| x | x * x).sum::< f32 > () - 1.0).abs() < 1e-5);
    }
    assert_eq!(b.indices, vec![1, 0]);
    let n = 6f32.sqrt();
    assert!((v[DIM] - 2.0 / n).abs() < 1e-5 && (v[DIM + 1] - 1.0 / n).abs() < 1e-5);
}
#[test]
fn output_is_converted_to_slot_dtype() {
    let f = fixture();
    let enc = PooledEncoder::new(FakeBackbone, None, Pooling::MeanMasked, DType::F16)
        .unwrap();
    let b = batch(&[vec![1, 2]]);
    let (tokens, out) = (put_tokens(&f, &b), out_ref(&f, &[1, DIM], DType::F16));
    let lane = RecordingLane::new(LaneClass::Query, f.lane_cfg);
    let mem: Arc<dyn RegionIo> = f.mem.clone();
    enc.encode_into(&mem, &tokens, &out, &lane).unwrap().wait().unwrap();
    let mut raw = vec![0u8; DIM * 2];
    f.mem.read(out.region, &mut raw).unwrap();
    let norm: f32 = raw
        .chunks_exact(2)
        .map(|b| half::f16::from_le_bytes([b[0], b[1]]).to_f32().powi(2))
        .sum();
    assert!((norm - 1.0).abs() < 1e-2);
}
#[test]
fn per_token_pooling_with_head_writes_every_position() {
    let f = fixture();
    let w: Vec<u8> = [1.0f32, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0]
        .iter()
        .flat_map(|v| v.to_le_bytes())
        .collect();
    let bytes = safetensors::serialize(
            [
                (
                    "linear.weight",
                    safetensors::tensor::TensorView::new(
                            safetensors::Dtype::F32,
                            vec![2, DIM],
                            &w,
                        )
                        .unwrap(),
                ),
            ],
            None,
        )
        .unwrap();
    let spec = HeadSpec {
        weight_name: "linear.weight".into(),
        in_dim: DIM,
        out_dim: 2,
        bias: false,
    };
    let head = ProjectionHead::from_safetensors(&bytes, &spec).unwrap();
    let enc = PooledEncoder::new(FakeBackbone, Some(head), Pooling::PerToken, DType::F32)
        .unwrap();
    assert_eq!(enc.dim(), 2);
    let b = batch(&[vec![1, 2, 3]]);
    let (tokens, out) = (
        put_tokens(&f, &b),
        out_ref(&f, &[1, b.seq_len, 2], DType::F32),
    );
    let lane = RecordingLane::new(LaneClass::Query, f.lane_cfg);
    let mem: Arc<dyn RegionIo> = f.mem.clone();
    enc.encode_into(&mem, &tokens, &out, &lane).unwrap().wait().unwrap();
    let v = read_f32(&f, &out);
    assert!((v[2] - std::f32::consts::FRAC_1_SQRT_2).abs() < 1e-5);
}
#[test]
fn runs_on_a_cpu_lane_and_completes_through_the_event() {
    let f = fixture();
    let enc = PooledEncoder::new(FakeBackbone, None, Pooling::MeanMasked, DType::F32)
        .unwrap();
    let b = batch(&[vec![1, 2, 3]]);
    let (tokens, out) = (put_tokens(&f, &b), out_ref(&f, &[1, DIM], DType::F32));
    let lane = CpuLane::new(LaneClass::Index, f.lane_cfg).unwrap();
    let mem: Arc<dyn RegionIo> = f.mem.clone();
    let ev = enc.encode_into(&mem, &tokens, &out, &lane).unwrap();
    ev.wait().unwrap();
    assert!(ev.is_complete());
    assert!(
        (read_f32(& f, & out).iter().map(| x | x * x).sum::< f32 > () - 1.0).abs() < 1e-5
    );
}
#[test]
fn mismatching_output_layout_is_rejected() {
    let f = fixture();
    let enc = PooledEncoder::new(FakeBackbone, None, Pooling::MeanMasked, DType::F32)
        .unwrap();
    let b = batch(&[vec![1]]);
    let tokens = put_tokens(&f, &b);
    let lane = RecordingLane::new(LaneClass::Query, f.lane_cfg);
    let mem: Arc<dyn RegionIo> = f.mem.clone();
    for bad in [
        out_ref(&f, &[1, DIM + 1], DType::F32),
        out_ref(&f, &[1, DIM], DType::F16),
    ] {
        assert!(
            matches!(enc.encode_into(& mem, & tokens, & bad, & lane),
            Err(ModelError::Incompatible { .. }))
        );
    }
}
#[test]
fn backbone_failure_surfaces_on_wait() {
    struct Failing;
    impl Backbone for Failing {
        fn dim(&self) -> usize {
            DIM
        }
        fn forward(
            &self,
            _: &[u32],
            _: &[u32],
            _: usize,
            _: usize,
        ) -> Result<Vec<f32>, ModelError> {
            Err(ModelError::EncodeFailed {
                reason: "boom".into(),
            })
        }
    }
    let f = fixture();
    let enc = PooledEncoder::new(Failing, None, Pooling::MeanMasked, DType::F32)
        .unwrap();
    let b = batch(&[vec![1]]);
    let (tokens, out) = (put_tokens(&f, &b), out_ref(&f, &[1, DIM], DType::F32));
    let lane = RecordingLane::new(LaneClass::Query, f.lane_cfg);
    let mem: Arc<dyn RegionIo> = f.mem.clone();
    let ev = enc.encode_into(&mem, &tokens, &out, &lane).unwrap();
    assert!(matches!(ev.wait(), Err(ModelError::EncodeFailed { .. })));
}
#[test]
fn unknown_architecture_is_rejected() {
    let spec = ModelSpec::e5_small(Path::new("/nonexistent"), DType::F32);
    let reg = ModelRegistry::new();
    assert!(
        matches!(reg.load(& spec, DeviceKind::Cpu), Err(ModelError::UnsupportedArch { ..
        }))
    );
}
#[test]
fn registered_factory_produces_a_keyed_resource() {
    let mut reg = ModelRegistry::new();
    reg.register(
        "bert",
        Box::new(|_, _| Ok(
            Arc::new(
                PooledEncoder::new(FakeBackbone, None, Pooling::MeanMasked, DType::F32)?,
            ) as Arc<dyn Encoder>,
        )),
    );
    let spec = ModelSpec::e5_small(Path::new("/nonexistent"), DType::F32);
    let loaded = reg.load(&spec, DeviceKind::Cpu).unwrap();
    assert_eq!(loaded.encoder().dim(), DIM);
    assert_eq!(loaded.key(), ResourceKey::new("model:bert:Cpu"));
}
