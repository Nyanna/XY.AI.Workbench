use super::*;
use candle_core::{Device, Tensor, D};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::a::kernel_batch::RangeCursor;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::a::library_adapter::to_lib_dtype;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::a::region_handle::Generation;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::a::arenas::ArenaKind;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::a::cpu_compute;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::query_batch::RamSource;
static_assertions::assert_impl_all!(TiledMaxSim : Send, Sync);
static_assertions::assert_impl_all!(FusedMaxSim < Exec >: Send, Sync);
static_assertions::assert_impl_all!(MaxSimError : Send, Sync);
static_assertions::assert_impl_all!(TopKRequest <'static >: Send, Sync);
const DIM: usize = 4;
const SEG: SegmentId = SegmentId(7);
fn lib(e: candle_core::Error) -> AdapterError {
    AdapterError::Library(e.to_string())
}
fn region(n: u64, len: u64) -> RegionHandle {
    RegionHandle::new(ArenaKind::MappedHost, n * 4096, len, Generation(0)).unwrap()
}
#[derive(Default)]
struct CpuOps {
    matmuls: AtomicUsize,
    max_elems: AtomicUsize,
}
impl TileOps for CpuOps {
    fn upload(
        &self,
        data: &[f32],
        rows: usize,
        cols: usize,
        dtype: DType,
    ) -> Result<LibTensor, AdapterError> {
        let t = Tensor::from_slice(data, (rows, cols), &Device::Cpu).map_err(lib)?;
        Ok(LibTensor::from_lib(t.to_dtype(to_lib_dtype(dtype)).map_err(lib)?))
    }
    fn doc_tile_t(
        &self,
        docs: &LibTensor,
        row_start: usize,
        rows: usize,
        tile: usize,
    ) -> Result<LibTensor, AdapterError> {
        let t = docs.lib().narrow(0, row_start, rows).map_err(lib)?;
        let t = if tile > rows {
            let pad = Tensor::zeros((tile - rows, DIM), t.dtype(), t.device())
                .map_err(lib)?;
            Tensor::cat(&[&t, &pad], 0).map_err(lib)?
        } else {
            t
        };
        Ok(LibTensor::from_lib(t.t().map_err(lib)?.contiguous().map_err(lib)?))
    }
    fn tile_mask(
        &self,
        valid: usize,
        tile: usize,
        dtype: DType,
    ) -> Result<LibTensor, AdapterError> {
        let mut v = vec![0f32; valid];
        v.resize(tile, MASK_VALUE);
        let t = Tensor::from_vec(v, (1, tile), &Device::Cpu).map_err(lib)?;
        Ok(LibTensor::from_lib(t.to_dtype(to_lib_dtype(dtype)).map_err(lib)?))
    }
    fn matmul(&self, a: &LibTensor, b: &LibTensor) -> Result<LibTensor, AdapterError> {
        let r = a.lib().matmul(b.lib()).map_err(lib)?;
        self.matmuls.fetch_add(1, Ordering::SeqCst);
        self.max_elems.fetch_max(r.elem_count(), Ordering::SeqCst);
        Ok(LibTensor::from_lib(r))
    }
    fn add_broadcast(
        &self,
        scores: &LibTensor,
        mask: &LibTensor,
    ) -> Result<LibTensor, AdapterError> {
        Ok(LibTensor::from_lib(scores.lib().broadcast_add(mask.lib()).map_err(lib)?))
    }
    fn max_last_dim(&self, scores: &LibTensor) -> Result<LibTensor, AdapterError> {
        Ok(LibTensor::from_lib(scores.lib().max(D::Minus1).map_err(lib)?))
    }
    fn maximum(&self, a: &LibTensor, b: &LibTensor) -> Result<LibTensor, AdapterError> {
        Ok(LibTensor::from_lib(a.lib().maximum(b.lib()).map_err(lib)?))
    }
    fn sum_f32(&self, v: &LibTensor) -> Result<f32, AdapterError> {
        let s = v
            .lib()
            .to_dtype(candle_core::DType::F32)
            .map_err(lib)?
            .sum_all()
            .map_err(lib)?;
        s.to_scalar::<f32>().map_err(lib)
    }
}
fn tiled(ops: &Arc<CpuOps>, tile: usize) -> TiledMaxSim {
    let ops = Arc::clone(ops);
    TiledMaxSim::new(
            DeviceHandle::Cpu,
            DType::F32,
            tile,
            &move |_| {
                let o: Arc<dyn TileOps> = ops.clone();
                Ok(o)
            },
        )
        .unwrap()
}
fn val(seed: usize) -> f32 {
    ((seed * 37 % 17) as f32 - 8.0) / 8.0
}
struct Fixture {
    src: RamSource,
    slots: Vec<Vec<f32>>,
    offsets: Vec<u64>,
    tensor: LibTensor,
}
fn fixture(tokens: &[usize]) -> Fixture {
    let mut counter = 0;
    let slots: Vec<Vec<f32>> = tokens
        .iter()
        .map(|&t| {
            (0..t * DIM)
                .map(|_| {
                    counter += 1;
                    val(counter)
                })
                .collect()
        })
        .collect();
    let mut src = RamSource::new(DIM as u32).unwrap();
    src.insert_segment(SEG, slots.iter().cloned().map(Some).collect()).unwrap();
    let mut offsets = vec![0u64];
    for s in &slots {
        offsets.push(offsets.last().unwrap() + s.len() as u64 * 4);
    }
    let flat: Vec<f32> = slots.iter().flatten().copied().collect();
    let rows = flat.len() / DIM;
    let t = Tensor::from_vec(flat, (rows, DIM), &Device::Cpu).unwrap();
    Fixture {
        src,
        slots,
        offsets,
        tensor: LibTensor::from_lib(t),
    }
}
impl Fixture {
    fn variable(&self) -> SegmentDesc {
        SegmentDesc {
            base: 0,
            entries: self.slots.len() as u64,
            layout: SegmentLayout::Variable {
                offsets: region(9, 64),
            },
        }
    }
    fn fixed(&self, stride: u64) -> SegmentDesc {
        SegmentDesc {
            base: 0,
            entries: self.slots.len() as u64,
            layout: SegmentLayout::Fixed { stride },
        }
    }
    fn req<'a>(
        &'a self,
        desc: SegmentDesc,
        range: Range<u32>,
        queries: &'a [&'a [f32]],
        k: usize,
    ) -> TopKRequest<'a> {
        TopKRequest {
            src: &self.src,
            segment: SEG,
            docs: DocSegment {
                desc,
                offsets: &self.offsets,
                tokens: &self.tensor,
            },
            range,
            queries,
            dim: DIM,
            k,
        }
    }
    fn score_matrix(&self, range: Range<u32>, queries: &[&[f32]]) -> Vec<Vec<f32>> {
        queries
            .iter()
            .map(|q| {
                range
                    .clone()
                    .map(|i| {
                        cpu_compute::max_sim(q, &self.slots[i as usize], DIM).unwrap()
                    })
                    .collect()
            })
            .collect()
    }
    fn expected(
        &self,
        range: Range<u32>,
        queries: &[&[f32]],
        k: usize,
    ) -> Vec<Vec<(u64, f32)>> {
        self.score_matrix(range.clone(), queries)
            .into_iter()
            .map(|row| {
                let mut hits: Vec<(u64, f32)> = row
                    .into_iter()
                    .zip(range.clone())
                    .map(|(s, i)| (self.offsets[i as usize], s))
                    .collect();
                hits.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap().then(a.0.cmp(&b.0)));
                hits.truncate(k);
                hits
            })
            .collect()
    }
}
fn assert_hits(got: &[Vec<Hit3>], want: &[Vec<(u64, f32)>]) {
    assert_eq!(got.len(), want.len());
    for (g, w) in got.iter().zip(want) {
        assert_eq!(g.len(), w.len());
        for (a, b) in g.iter().zip(w) {
            assert_eq!(a.offset, b.0);
            assert!(
                (a.score - b.1).abs() <= 1e-4 * (1.0 + b.1.abs()), "{} vs {}", a.score, b
                .1
            );
        }
    }
}
fn queries() -> (Vec<f32>, Vec<f32>, Vec<f32>) {
    let q0: Vec<f32> = (0..2 * DIM).map(|i| val(i + 101)).collect();
    let q1: Vec<f32> = (0..DIM).map(|i| val(i + 202)).collect();
    (q0, q1, Vec::new())
}
#[test]
fn variable_layout_matches_reference_for_all_tile_sizes() {
    let f = fixture(&[3, 0, 5, 2]);
    let (q0, q1, q2) = queries();
    let qs: Vec<&[f32]> = vec![& q0, & q1, & q2];
    for tile in [1, 2, 3, 4, 5, 100] {
        let ops = Arc::new(CpuOps::default());
        let op = tiled(&ops, tile);
        let r = f.req(f.variable(), 0..4, &qs, 10);
        let got = op.top_k(&r).unwrap();
        assert_hits(&got, &f.expected(0..4, &qs, 10));
        assert!(ops.max_elems.load(Ordering::SeqCst) <= 2 * tile);
    }
}
#[test]
fn fixed_layout_matches_reference_on_subrange() {
    let f = fixture(&[3, 3, 3, 3]);
    let (q0, q1, _) = queries();
    let qs: Vec<&[f32]> = vec![& q0, & q1];
    let ops = Arc::new(CpuOps::default());
    let op = tiled(&ops, 2);
    let r = f.req(f.fixed(48), 1..4, &qs, 2);
    assert_hits(&op.top_k(&r).unwrap(), &f.expected(1..4, &qs, 2));
}
#[test]
fn empty_entry_scores_mask_value_without_library_call() {
    let f = fixture(&[0, 2]);
    let (q0, _, _) = queries();
    let qs: Vec<&[f32]> = vec![& q0];
    let ops = Arc::new(CpuOps::default());
    let op = tiled(&ops, 4);
    let r = f.req(f.variable(), 0..1, &qs, 3);
    let got = op.top_k(&r).unwrap();
    assert_eq!(got[0].len(), 1);
    assert_eq!(got[0] [0].score, MASK_VALUE * 2.0);
    assert_eq!(ops.matmuls.load(Ordering::SeqCst), 0);
}
#[test]
fn empty_query_scores_zero() {
    let f = fixture(&[0, 2]);
    let empty: Vec<f32> = Vec::new();
    let qs: Vec<&[f32]> = vec![& empty];
    let op = tiled(&Arc::new(CpuOps::default()), 4);
    let got = op.top_k(&f.req(f.variable(), 0..2, &qs, 5)).unwrap();
    assert!(got[0].iter().all(| h | h.score == 0.0));
    assert_eq!(got[0].len(), 2);
}
#[test]
fn trivial_requests_give_empty_lists() {
    let f = fixture(&[3, 2]);
    let (q0, q1, _) = queries();
    let qs: Vec<&[f32]> = vec![& q0, & q1];
    let op = tiled(&Arc::new(CpuOps::default()), 2);
    assert_eq!(
        op.top_k(& f.req(f.variable(), 0..2, & qs, 0)).unwrap(), vec![Vec::<Hit3
        >::new(); 2]
    );
    assert_eq!(
        op.top_k(& f.req(f.variable(), 1..1, & qs, 3)).unwrap(), vec![Vec::<Hit3
        >::new(); 2]
    );
    let none: Vec<&[f32]> = Vec::new();
    assert!(op.top_k(& f.req(f.variable(), 0..2, & none, 3)).unwrap().is_empty());
}
struct HoleSource(RamSource);
impl DocSource for HoleSource {
    fn dim(&self) -> u32 {
        self.0.dim()
    }
    fn slot_count(&self, s: SegmentId) -> u32 {
        self.0.slot_count(s)
    }
    fn slot_offset(&self, s: SegmentId, slot: Slot5) -> Option<u64> {
        if slot.0 == 1 { None } else { self.0.slot_offset(s, slot) }
    }
    fn slot_vectors(&self, s: SegmentId, slot: Slot5) -> Option<&[f32]> {
        self.0.slot_vectors(s, slot)
    }
}
#[test]
fn missing_slot_offset_is_skipped() {
    let f = fixture(&[2, 2, 2]);
    let hole = HoleSource(f.src.clone());
    let (q0, _, _) = queries();
    let qs: Vec<&[f32]> = vec![& q0];
    let op = tiled(&Arc::new(CpuOps::default()), 2);
    let mut r = f.req(f.variable(), 0..3, &qs, 5);
    r.src = &hole;
    let got = op.top_k(&r).unwrap();
    let offsets: Vec<u64> = got[0].iter().map(|h| h.offset).collect();
    assert_eq!(offsets.len(), 2);
    assert!(! offsets.contains(& f.offsets[1]));
}
#[test]
fn construction_rules() {
    let calls = AtomicUsize::new(0);
    let open = |_: DeviceHandle| -> Result<Arc<dyn TileOps>, String> {
        calls.fetch_add(1, Ordering::SeqCst);
        Ok(Arc::new(CpuOps::default()))
    };
    assert!(
        matches!(TiledMaxSim::new(DeviceHandle::Cpu, DType::F16, 4, & open),
        Err(MaxSimError::Unavailable(_)))
    );
    assert!(
        matches!(TiledMaxSim::new(DeviceHandle::Gpu(0), DType::BF16, 4, & open),
        Err(MaxSimError::Unavailable(_)))
    );
    assert_eq!(calls.load(Ordering::SeqCst), 0);
    let op = TiledMaxSim::new(DeviceHandle::Cpu, DType::F32, 0, &open).unwrap();
    assert_eq!(op.tile_size(), 1);
    assert_eq!(op.precision(), DType::F32);
    let failing = |_: DeviceHandle| -> Result<Arc<dyn TileOps>, String> {
        Err("no device".into())
    };
    assert!(
        matches!(TiledMaxSim::new(DeviceHandle::Cpu, DType::F32, 4, & failing),
        Err(MaxSimError::Unavailable(_)))
    );
}
#[test]
fn validation_errors_in_order_and_before_library_calls() {
    let f = fixture(&[3, 3, 3, 3]);
    let (q0, _, _) = queries();
    let bad_q: Vec<f32> = vec![0.0; DIM + 1];
    let ok: Vec<&[f32]> = vec![& q0];
    let bad: Vec<&[f32]> = vec![& q0, & bad_q];
    let ops = Arc::new(CpuOps::default());
    let op = tiled(&ops, 2);
    let mut r = f.req(f.variable(), 0..99, &bad, 3);
    r.dim = 0;
    assert_eq!(op.top_k(& r), Err(MaxSimError::InvalidDim));
    let r = f.req(f.fixed(48), 0..99, &bad, 3);
    assert!(matches!(op.top_k(& r), Err(MaxSimError::Source(_))));
    let mut short = f.fixed(48);
    short.entries = 2;
    let r = f.req(short, 0..3, &ok, 3);
    assert!(matches!(op.top_k(& r), Err(MaxSimError::Source(_))));
    let r = f.req(f.fixed(48), 0..4, &bad, 3);
    assert_eq!(
        op.top_k(& r), Err(MaxSimError::Shape { what : "query", index : 1, len : 5, dim :
        DIM })
    );
    let r = f.req(f.fixed(50), 0..4, &ok, 3);
    assert!(matches!(op.top_k(& r), Err(MaxSimError::Shape { what : "stride", .. })));
    let r = f.req(f.fixed(96), 0..4, &ok, 3);
    assert!(matches!(op.top_k(& r), Err(MaxSimError::Mismatch(_))));
    let mut offsets = f.offsets.clone();
    offsets[1] = 5;
    let mut r = f.req(f.variable(), 0..2, &ok, 3);
    r.docs.offsets = &offsets;
    assert!(
        matches!(op.top_k(& r), Err(MaxSimError::Shape { what : "offset", index : 1, ..
        }))
    );
    let mut r = f.req(f.variable(), 0..2, &ok, 3);
    r.docs.offsets = &f.offsets[..2];
    assert!(matches!(op.top_k(& r), Err(MaxSimError::Mismatch(_))));
    let f16 = LibTensor::from_lib(
        f.tensor.lib().to_dtype(candle_core::DType::F16).unwrap(),
    );
    let mut r = f.req(f.fixed(48), 0..4, &ok, 3);
    r.docs.tokens = &f16;
    assert!(matches!(op.top_k(& r), Err(MaxSimError::Mismatch(_))));
    let three = LibTensor::from_lib(
        Tensor::zeros((4, 3), candle_core::DType::F32, &Device::Cpu).unwrap(),
    );
    let mut r = f.req(f.fixed(48), 0..4, &ok, 3);
    r.docs.tokens = &three;
    assert!(matches!(op.top_k(& r), Err(MaxSimError::Mismatch(_))));
    let cube = LibTensor::from_lib(
        Tensor::zeros((2, 2, DIM), candle_core::DType::F32, &Device::Cpu).unwrap(),
    );
    let mut r = f.req(f.fixed(48), 0..4, &ok, 3);
    r.docs.tokens = &cube;
    assert!(matches!(op.top_k(& r), Err(MaxSimError::Mismatch(_))));
    assert_eq!(ops.matmuls.load(Ordering::SeqCst), 0);
}
#[test]
fn concurrent_calls_agree() {
    let f = fixture(&[3, 4, 5]);
    let (q0, q1, _) = queries();
    let qs: Vec<&[f32]> = vec![& q0, & q1];
    let op = tiled(&Arc::new(CpuOps::default()), 2);
    let want = f.expected(0..3, &qs, 3);
    std::thread::scope(|s| {
        for _ in 0..4 {
            s.spawn(|| {
                let got = op.top_k(&f.req(f.variable(), 0..3, &qs, 3)).unwrap();
                assert_hits(&got, &want);
            });
        }
    });
}
struct Exec {
    outcome: Result<LaunchOutcome, String>,
    specs: Arc<Mutex<Vec<BatchSpec>>>,
}
impl BatchExecutor for Exec {
    fn launch(
        &self,
        _id: BatchId,
        spec: &BatchSpec,
        inputs: &BatchInputs,
        _queries_done: u32,
        ranges: &mut [RangeCursor],
    ) -> Result<LaunchOutcome, String> {
        assert_eq!(inputs.regions.len(), 1);
        assert_eq!(ranges.len(), 1);
        self.specs.lock().unwrap().push(spec.clone());
        self.outcome.clone()
    }
}
struct MockIo {
    scores: Vec<f32>,
    next: AtomicUsize,
    uploads: AtomicUsize,
    alloc_bytes: Mutex<Vec<u64>>,
    released: Mutex<Vec<RegionHandle>>,
    fail_upload: bool,
    fail_alloc: bool,
}
impl MockIo {
    fn new(scores: Vec<f32>) -> Arc<MockIo> {
        Arc::new(MockIo {
            scores,
            next: AtomicUsize::new(1),
            uploads: AtomicUsize::new(0),
            alloc_bytes: Mutex::new(Vec::new()),
            released: Mutex::new(Vec::new()),
            fail_upload: false,
            fail_alloc: false,
        })
    }
    fn fresh(&self) -> RegionHandle {
        region(self.next.fetch_add(1, Ordering::SeqCst) as u64 + 20, 16)
    }
}
impl FusedIo for MockIo {
    fn upload_queries(
        &self,
        _q: &[&[f32]],
        _dim: usize,
        _p: DType,
    ) -> Result<RegionHandle, String> {
        self.uploads.fetch_add(1, Ordering::SeqCst);
        if self.fail_upload {
            return Err("upload failed".into());
        }
        Ok(self.fresh())
    }
    fn alloc_scores(&self, bytes: u64) -> Result<RegionHandle, String> {
        self.alloc_bytes.lock().unwrap().push(bytes);
        if self.fail_alloc {
            return Err("alloc failed".into());
        }
        Ok(self.fresh())
    }
    fn read_scores(&self, _r: RegionHandle, n: usize) -> Result<Vec<f32>, String> {
        assert_eq!(n, self.scores.len());
        Ok(self.scores.clone())
    }
    fn release(&self, region: RegionHandle) {
        self.released.lock().unwrap().push(region);
    }
}
fn fused(
    outcome: Result<LaunchOutcome, String>,
    io: &Arc<MockIo>,
    params: Option<RegionHandle>,
) -> (FusedMaxSim<Exec>, Arc<Mutex<Vec<BatchSpec>>>) {
    let specs = Arc::new(Mutex::new(Vec::new()));
    let iface = Arc::new(
        BatchInterface::new(Exec {
            outcome,
            specs: Arc::clone(&specs),
        }),
    );
    let io: Arc<dyn FusedIo> = io.clone();
    (
        FusedMaxSim::new(DeviceHandle::Gpu(0), DType::F32, iface, io, params).unwrap(),
        specs,
    )
}
fn flat(m: &[Vec<f32>]) -> Vec<f32> {
    m.iter().flatten().copied().collect()
}
#[test]
fn fused_fixed_subrange() {
    let f = fixture(&[3, 3, 3, 3]);
    let (q0, q1, _) = queries();
    let qs: Vec<&[f32]> = vec![& q0, & q1];
    let io = MockIo::new(flat(&f.score_matrix(1..4, &qs)));
    let params = Some(region(5, 8));
    let (op, specs) = fused(Ok(LaunchOutcome::Hits { hits: 0 }), &io, params);
    let got = op.top_k(&f.req(f.fixed(48), 1..4, &qs, 2)).unwrap();
    assert_hits(&got, &f.expected(1..4, &qs, 2));
    let specs = specs.lock().unwrap();
    assert_eq!(specs.len(), 1);
    assert_eq!(specs[0].query_count, 2);
    assert_eq!(specs[0].params, params);
    assert_eq!(specs[0].segments[0].base, 48);
    assert_eq!(specs[0].segments[0].entries, 3);
    assert_eq!(* io.alloc_bytes.lock().unwrap(), vec![2 * 3 * 4]);
    assert_eq!(io.released.lock().unwrap().len(), 2);
}
#[test]
fn fused_variable_scores_whole_segment() {
    let f = fixture(&[3, 0, 5, 2]);
    let (q0, q1, _) = queries();
    let qs: Vec<&[f32]> = vec![& q0, & q1];
    let io = MockIo::new(flat(&f.score_matrix(0..4, &qs)));
    let (op, specs) = fused(Ok(LaunchOutcome::Hits { hits: 0 }), &io, None);
    let got = op.top_k(&f.req(f.variable(), 1..4, &qs, 5)).unwrap();
    assert_hits(&got, &f.expected(1..4, &qs, 5));
    assert_eq!(specs.lock().unwrap() [0].segments[0], f.variable());
    assert_eq!(* io.alloc_bytes.lock().unwrap(), vec![2 * 4 * 4]);
}
#[test]
fn fused_failures_release_regions() {
    let f = fixture(&[3, 3]);
    let (q0, _, _) = queries();
    let qs: Vec<&[f32]> = vec![& q0];
    for outcome in [
        Ok(LaunchOutcome::Partial {
            queries_done: 0,
            hits: 0,
        }),
        Err("kernel failed".to_string()),
    ] {
        let io = MockIo::new(vec![0.0; 2]);
        let (op, _) = fused(outcome, &io, None);
        let r = op.top_k(&f.req(f.fixed(48), 0..2, &qs, 2));
        assert!(matches!(r, Err(MaxSimError::Launch(_))));
        assert_eq!(io.released.lock().unwrap().len(), 2);
    }
    let io = Arc::new(MockIo {
        fail_alloc: true,
        ..Arc::try_unwrap(MockIo::new(vec![])).ok().unwrap()
    });
    let (op, specs) = fused(Ok(LaunchOutcome::Hits { hits: 0 }), &io, None);
    assert!(
        matches!(op.top_k(& f.req(f.fixed(48), 0..2, & qs, 2)),
        Err(MaxSimError::Launch(_)))
    );
    assert_eq!(io.released.lock().unwrap().len(), 1);
    assert!(specs.lock().unwrap().is_empty());
    let io = Arc::new(MockIo {
        fail_upload: true,
        ..Arc::try_unwrap(MockIo::new(vec![])).ok().unwrap()
    });
    let (op, _) = fused(Ok(LaunchOutcome::Hits { hits: 0 }), &io, None);
    assert!(
        matches!(op.top_k(& f.req(f.fixed(48), 0..2, & qs, 2)),
        Err(MaxSimError::Launch(_)))
    );
}
#[test]
fn fused_validates_and_short_circuits_before_io() {
    let f = fixture(&[3, 3]);
    let (q0, _, _) = queries();
    let qs: Vec<&[f32]> = vec![& q0];
    let io = MockIo::new(vec![]);
    let (op, specs) = fused(Ok(LaunchOutcome::Hits { hits: 0 }), &io, None);
    assert!(
        matches!(op.top_k(& f.req(f.fixed(48), 0..9, & qs, 2)),
        Err(MaxSimError::Source(_)))
    );
    assert!(
        matches!(op.top_k(& f.req(f.fixed(50), 0..2, & qs, 2)), Err(MaxSimError::Shape {
        what : "stride", .. }))
    );
    let none: Vec<&[f32]> = Vec::new();
    assert!(op.top_k(& f.req(f.fixed(48), 0..2, & none, 2)).unwrap().is_empty());
    assert_eq!(
        op.top_k(& f.req(f.fixed(48), 0..2, & qs, 0)).unwrap(), vec![Vec::<Hit3>::new()]
    );
    assert_eq!(io.uploads.load(Ordering::SeqCst), 0);
    assert!(specs.lock().unwrap().is_empty());
}
#[test]
fn fused_unavailable_precisions() {
    let io: Arc<dyn FusedIo> = MockIo::new(vec![]);
    let iface = Arc::new(
        BatchInterface::new(Exec {
            outcome: Ok(LaunchOutcome::Hits { hits: 0 }),
            specs: Arc::new(Mutex::new(Vec::new())),
        }),
    );
    for (device, dtype) in [
        (DeviceHandle::Cpu, DType::F16),
        (DeviceHandle::Gpu(0), DType::BF16),
    ] {
        let r = FusedMaxSim::new(
            device,
            dtype,
            Arc::clone(&iface),
            Arc::clone(&io),
            None,
        );
        assert!(matches!(r, Err(MaxSimError::Unavailable(_))));
    }
}
