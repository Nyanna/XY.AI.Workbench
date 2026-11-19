use super::*;
use std::any::Any;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Mutex, OnceLock};
use crate::resident::l130::l120::l110::l100::l090::l080::l070::a::kernel_register::{
    CodeImage, CompileRequest, CompilerError, KernelCompiler, LoadFailure, ModuleLoader,
};
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::a::library_adapter::library_kernel_source;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::a::region_handle::Generation;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::l010::{
    device_lane_event::{
        DeviceHandle, GpuEventBackend, LaneConfig, LaneKind, StreamBackend,
    },
    target_env::ComputeCapability,
};
const BASE: u64 = 0x1000_0000;
fn gpu() -> Option<candle_core::CudaDevice> {
    static DEV: OnceLock<Option<candle_core::CudaDevice>> = OnceLock::new();
    DEV.get_or_init(|| {
            std::panic::catch_unwind(|| candle_core::CudaDevice::new_with_stream(0))
                .ok()
                .and_then(|r| r.ok())
        })
        .clone()
}
macro_rules! gpu_or_skip {
    () => {
        match gpu() { Some(d) => d, None => { eprintln!("skipped: no CUDA device");
        return; } }
    };
}
#[derive(Debug)]
struct Func(String);
impl KernelFunction for Func {
    fn symbol(&self) -> &str {
        &self.0
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}
struct MockCompiler {
    calls: AtomicUsize,
    requests: Mutex<Vec<CompileRequest>>,
    fail: Option<CompilerError>,
}
impl KernelCompiler for MockCompiler {
    fn compile(&self, req: &CompileRequest) -> Result<CodeImage, CompilerError> {
        self.calls.fetch_add(1, Ordering::SeqCst);
        self.requests.lock().unwrap().push(req.clone());
        match &self.fail {
            Some(e) => Err(e.clone()),
            None => {
                Ok(CodeImage {
                    kind: req.kind,
                    bytes: vec![0],
                })
            }
        }
    }
}
struct MockLoader {
    cc: Option<ComputeCapability>,
}
impl ModuleLoader for MockLoader {
    fn device_capability(&self) -> Option<ComputeCapability> {
        self.cc
    }
    fn load(
        &self,
        _: &CodeImage,
        symbol: &str,
    ) -> Result<Arc<dyn KernelFunction>, LoadFailure> {
        Ok(Arc::new(Func(symbol.to_string())))
    }
}
#[derive(Debug, Clone)]
struct Recorded {
    metric: Metric,
    dtype: DType,
    dim: u32,
    query: Vec<f32>,
    segments: Vec<(u64, u32)>,
    symbol: String,
}
struct MockReadback(Result<Vec<f32>, String>);
impl ScoreReadback for MockReadback {
    fn read(&self) -> Result<Vec<f32>, String> {
        self.0.clone()
    }
}
#[derive(Default)]
struct MockLauncher {
    calls: Mutex<Vec<Recorded>>,
    launch_error: Option<String>,
    readback: Mutex<Option<Result<Vec<f32>, String>>>,
}
impl ScoreLauncher for MockLauncher {
    fn launch(
        &self,
        function: &Arc<dyn KernelFunction>,
        _lane: &Lane,
        req: &LaunchRequest<'_>,
    ) -> Result<Box<dyn ScoreReadback>, String> {
        let total: u32 = req.segments.iter().map(|s| s.rows).sum();
        self.calls
            .lock()
            .unwrap()
            .push(Recorded {
                metric: req.metric,
                dtype: req.dtype,
                dim: req.dim,
                query: req.query.to_vec(),
                segments: req
                    .segments
                    .iter()
                    .map(|s| (s.first_row_addr, s.rows))
                    .collect(),
                symbol: function.symbol().to_string(),
            });
        if let Some(e) = &self.launch_error {
            return Err(e.clone());
        }
        let result = match self.readback.lock().unwrap().clone() {
            Some(r) => r,
            None => Ok((0..total).map(|i| i as f32).collect()),
        };
        Ok(Box::new(MockReadback(result)))
    }
}
#[derive(Debug)]
struct FakeEvent {
    ready: Arc<AtomicBool>,
    fail: bool,
}
impl GpuEventBackend for FakeEvent {
    fn query(&self) -> Result<bool, String> {
        if self.fail {
            return Err("query failed".to_string());
        }
        Ok(self.ready.load(Ordering::SeqCst))
    }
    fn synchronize(&self) -> Result<(), String> {
        if self.fail {
            return Err("sync failed".to_string());
        }
        Ok(())
    }
}
#[derive(Debug)]
struct FakeStream {
    ready: Arc<AtomicBool>,
    fail_event: bool,
    fail_record: bool,
}
impl StreamBackend for FakeStream {
    fn record_event(&self) -> Result<Box<dyn GpuEventBackend>, String> {
        if self.fail_record {
            return Err("record failed".to_string());
        }
        Ok(
            Box::new(FakeEvent {
                ready: Arc::clone(&self.ready),
                fail: self.fail_event,
            }),
        )
    }
}
fn lane_cfg() -> LaneConfig {
    LaneConfig::preset(LaneKind::Query, DeviceHandle::Gpu(0))
}
fn lane() -> Lane {
    lane_with(Arc::new(AtomicBool::new(true)), false, false)
}
fn lane_with(ready: Arc<AtomicBool>, fail_event: bool, fail_record: bool) -> Lane {
    Lane::new(
        lane_cfg(),
        Some(
            Box::new(FakeStream {
                ready,
                fail_event,
                fail_record,
            }),
        ),
    )
}
struct Env {
    scorer: Scorer,
    register: Arc<KernelRegister>,
    compiler: Arc<MockCompiler>,
    launcher: Arc<MockLauncher>,
}
fn env_with(
    device: DeviceHandle,
    register_kernels: bool,
    fail: Option<CompilerError>,
    launcher: MockLauncher,
) -> Env {
    let compiler = Arc::new(MockCompiler {
        calls: AtomicUsize::new(0),
        requests: Mutex::new(Vec::new()),
        fail,
    });
    let loader = Arc::new(MockLoader {
        cc: Some(ComputeCapability::new(7, 5)),
    });
    let register = Arc::new(
        KernelRegister::new(device, compiler.clone(), loader, library_kernel_source()),
    );
    if register_kernels {
        register_score_kernels(&register).unwrap();
    }
    let launcher = Arc::new(launcher);
    let scorer = Scorer::new(register.clone(), launcher.clone());
    Env {
        scorer,
        register,
        compiler,
        launcher,
    }
}
fn env() -> Env {
    env_with(DeviceHandle::Gpu(0), true, None, MockLauncher::default())
}
fn block(dev: &candle_core::CudaDevice, len: u64) -> DeviceBlock {
    DeviceBlock {
        device: dev.clone(),
        base_addr: BASE,
        len_bytes: len,
    }
}
fn region(arena: ArenaKind, offset: u64, len: u64) -> RegionHandle {
    RegionHandle::new(arena, offset, len, Generation::INITIAL).unwrap()
}
fn seg_in(
    dev: &candle_core::CudaDevice,
    arena: ArenaKind,
    offset: u64,
    rows: usize,
    dim: usize,
    dtype: DType,
) -> ScoreSegment {
    let layout = Layout::contiguous(&[rows, dim], dtype);
    let bytes = layout.byte_len() as u64;
    ScoreSegment {
        block: block(dev, offset + bytes + 4096),
        region: region(arena, offset, bytes),
        layout,
    }
}
fn seg(
    dev: &candle_core::CudaDevice,
    rows: usize,
    dim: usize,
    dtype: DType,
) -> ScoreSegment {
    seg_in(dev, ArenaKind::IndexSegments, 256, rows, dim, dtype)
}
fn with_slots(segment: ScoreSegment, slots: Range<u32>) -> SegmentSlots {
    SegmentSlots { segment, slots }
}
fn request(metric: Metric, dim: usize, segments: Vec<SegmentSlots>) -> ScoreRequest {
    ScoreRequest {
        metric,
        query: (0..dim).map(|i| i as f32).collect(),
        segments,
    }
}
fn err(r: Result<ScoreJob, ScoreError>) -> ScoreError {
    match r {
        Ok(_) => panic!("expected an error"),
        Err(e) => e,
    }
}
fn assert_untouched(env: &Env) {
    assert_eq!(env.compiler.calls.load(Ordering::SeqCst), 0);
    assert!(env.launcher.calls.lock().unwrap().is_empty());
}
#[test]
fn metric_kernel_names() {
    assert_eq!(Metric::Dot.kernel_name(), "score_dot");
    assert_eq!(Metric::NegL2Sq.kernel_name(), "score_negl2");
}
#[test]
fn slot_addr_accessors() {
    let a = SlotAddr::new(ArenaKind::Weights, 96);
    assert_eq!(a.arena(), ArenaKind::Weights);
    assert_eq!(a.offset(), 96);
    assert_eq!(a, SlotAddr::new(ArenaKind::Weights, 96));
    assert_ne!(a, SlotAddr::new(ArenaKind::Staging, 96));
}
#[test]
fn error_display_is_not_empty() {
    let errors = [
        ScoreError::UnsupportedDType(DType::BF16),
        ScoreError::QueryDimMismatch {
            query: 1,
            dim: 2,
        },
        ScoreError::BadLayout("x".into()),
        ScoreError::OutOfBounds {
            slots: 1..9,
            rows: 3,
        },
        ScoreError::Kernel(KernelError::Unavailable("x".into())),
        ScoreError::Launch("x".into()),
        ScoreError::Readback("x".into()),
        ScoreError::Lane(LaneError::NoStreamBackend),
    ];
    for e in errors {
        assert!(! e.to_string().is_empty());
    }
}
#[test]
fn register_adds_two_sources_and_rejects_a_second_call() {
    let e = env_with(DeviceHandle::Gpu(0), false, None, MockLauncher::default());
    register_score_kernels(&e.register).unwrap();
    for name in [KERNEL_SCORE_DOT, KERNEL_SCORE_NEGL2] {
        for dtype in [DType::F32, DType::F16] {
            let v = Variant {
                dim: Some(8),
                tile: None,
            };
            assert!(matches!(e.register.request(name, dtype, v), Ok(Kernel::Gpu(_))));
        }
        let v = Variant {
            dim: Some(8),
            tile: None,
        };
        assert!(
            matches!(e.register.request(name, DType::BF16, v),
            Err(KernelError::Unavailable(_)))
        );
    }
    assert!(
        matches!(register_score_kernels(& e.register), Err(KernelError::Unavailable(_)))
    );
}
#[test]
fn kernel_bodies_use_only_the_prelude() {
    for (body, entry) in [
        (SCORE_DOT_BODY, ENTRY_SCORE_DOT),
        (SCORE_NEGL2_BODY, ENTRY_SCORE_NEGL2),
    ] {
        assert!(body.contains(& format!("extern \"C\" __global__ void {entry}(")));
        assert!(body.contains("DIM"));
        assert!(body.contains("load_f"));
        assert!(body.contains("warp_reduce_sum"));
        for banned in ["TILE", "bf16", "bfloat", "mma", "__CUDA_ARCH__", "printf"] {
            assert!(! body.contains(banned), "{banned} in {entry}");
        }
    }
}
#[test]
fn jit_source_carries_dim_and_no_tile() {
    let dev = gpu_or_skip!();
    let e = env();
    let req = request(
        Metric::NegL2Sq,
        4,
        vec![with_slots(seg(& dev, 4, 4, DType::F32), 0..4)],
    );
    e.scorer.score(&lane(), &req).unwrap().finish().unwrap();
    let sources = e.compiler.requests.lock().unwrap();
    assert_eq!(sources.len(), 1);
    assert!(sources[0].source.contains("#define DIM 4"));
    assert!(! sources[0].source.contains("#define TILE"));
    assert!(sources[0].source.contains(ENTRY_SCORE_NEGL2));
}
#[test]
fn jit_sources_compile_with_nvcc_when_available() {
    let candidates = [
        std::env::var("CUDA_HOME").map(|h| format!("{h}/bin/nvcc")).unwrap_or_default(),
        "/usr/local/cuda-13.3/bin/nvcc".to_string(),
        "nvcc".to_string(),
    ];
    let dir = std::env::temp_dir()
        .join(format!("kernel_scoring_nvcc_{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let mut ran = false;
    for (metric, dtype, dim) in [
        (Metric::Dot, DType::F32, 384u32),
        (Metric::Dot, DType::F16, 384),
        (Metric::NegL2Sq, DType::F32, 33),
        (Metric::NegL2Sq, DType::F16, 1),
    ] {
        let e = env();
        let v = Variant {
            dim: Some(dim),
            tile: None,
        };
        e.register.request(metric.kernel_name(), dtype, v).unwrap();
        let req = e.compiler.requests.lock().unwrap()[0].clone();
        let src = dir.join(format!("{}_{:?}_{dim}.cu", metric.kernel_name(), dtype));
        let out = dir.join("out.ptx");
        std::fs::write(&src, &req.source).unwrap();
        let mut status = None;
        for nvcc in candidates.iter().filter(|c| !c.is_empty()) {
            let run = std::process::Command::new(nvcc)
                .arg("-ptx")
                .arg(format!("-arch={}", req.arch))
                .arg("-o")
                .arg(&out)
                .arg(&src)
                .output();
            if let Ok(o) = run {
                status = Some(o);
                break;
            }
        }
        match status {
            None => {
                eprintln!("skipped: nvcc not found");
                break;
            }
            Some(o) => {
                ran = true;
                assert!(
                    o.status.success(), "nvcc failed for {}: {}", src.display(),
                    String::from_utf8_lossy(& o.stderr)
                );
            }
        }
    }
    let _ = std::fs::remove_dir_all(&dir);
    let _ = ran;
}
#[test]
fn result_follows_segment_and_slot_order_with_addresses() {
    let dev = gpu_or_skip!();
    let e = env();
    let a = seg_in(&dev, ArenaKind::IndexSegments, 256, 10, 4, DType::F32);
    let b = seg_in(&dev, ArenaKind::Weights, 1024, 6, 4, DType::F32);
    let req = request(Metric::Dot, 4, vec![with_slots(a, 2..5), with_slots(b, 0..2)]);
    let job = e.scorer.score(&lane(), &req).unwrap();
    let out = job.finish().unwrap();
    assert_eq!(out.len(), 5);
    let expect = [
        (ArenaKind::IndexSegments, 256 + 32),
        (ArenaKind::IndexSegments, 256 + 48),
        (ArenaKind::IndexSegments, 256 + 64),
        (ArenaKind::Weights, 1024),
        (ArenaKind::Weights, 1024 + 16),
    ];
    for (i, (slot, (arena, offset))) in out.iter().zip(expect).enumerate() {
        assert_eq!(slot.addr, SlotAddr::new(arena, offset));
        assert_eq!(slot.score, i as f32);
    }
    let calls = e.launcher.calls.lock().unwrap();
    assert_eq!(calls.len(), 1);
    let c = &calls[0];
    assert_eq!(c.metric, Metric::Dot);
    assert_eq!(c.dtype, DType::F32);
    assert_eq!(c.dim, 4);
    assert_eq!(c.query, vec![0.0, 1.0, 2.0, 3.0]);
    assert_eq!(c.symbol, ENTRY_SCORE_DOT);
    assert_eq!(c.segments, vec![(BASE + 256 + 32, 3), (BASE + 1024, 2)]);
}
#[test]
fn f16_addresses_use_two_byte_elements() {
    let dev = gpu_or_skip!();
    let e = env();
    let s = seg_in(&dev, ArenaKind::IndexSegments, 66, 8, 8, DType::F16);
    let req = request(Metric::NegL2Sq, 8, vec![with_slots(s, 1..3)]);
    let out = e.scorer.score(&lane(), &req).unwrap().finish().unwrap();
    assert_eq!(out[0].addr.offset(), 66 + 16);
    assert_eq!(out[1].addr.offset(), 66 + 32);
    let calls = e.launcher.calls.lock().unwrap();
    assert_eq!(calls[0].dtype, DType::F16);
    assert_eq!(calls[0].symbol, ENTRY_SCORE_NEGL2);
    assert_eq!(calls[0].segments, vec![(BASE + 66 + 16, 2)]);
}
#[test]
fn empty_ranges_are_skipped_in_the_launch() {
    let dev = gpu_or_skip!();
    let e = env();
    let req = request(
        Metric::Dot,
        4,
        vec![
            with_slots(seg(& dev, 4, 4, DType::F32), 3..3), with_slots(seg(& dev, 4, 4,
            DType::F32), 1..3),
        ],
    );
    let out = e.scorer.score(&lane(), &req).unwrap().finish().unwrap();
    assert_eq!(out.len(), 2);
    let calls = e.launcher.calls.lock().unwrap();
    assert_eq!(calls[0].segments, vec![(BASE + 256 + 16, 2)]);
}
#[test]
fn all_empty_ranges_give_a_completed_job_without_kernel_or_launch() {
    let dev = gpu_or_skip!();
    let e = env();
    let no_stream = Lane::new(lane_cfg(), None);
    let req = request(
        Metric::Dot,
        4,
        vec![with_slots(seg(& dev, 4, 4, DType::F32), 0..0)],
    );
    let job = e.scorer.score(&no_stream, &req).unwrap();
    assert!(job.is_complete());
    assert_eq!(job.event().is_complete(), Ok(true));
    assert_eq!(job.finish().unwrap(), vec![]);
    assert_untouched(&e);
}
#[test]
fn no_segments_gives_a_completed_job() {
    let e = env();
    let no_stream = Lane::new(lane_cfg(), None);
    let job = e.scorer.score(&no_stream, &request(Metric::Dot, 4, vec![])).unwrap();
    assert!(job.is_complete());
    assert_eq!(job.finish().unwrap(), vec![]);
    assert_untouched(&e);
}
#[test]
fn validation_runs_for_empty_ranges() {
    let dev = gpu_or_skip!();
    let e = env();
    let req = request(
        Metric::Dot,
        4,
        vec![with_slots(seg(& dev, 4, 4, DType::BF16), 0..0)],
    );
    assert_eq!(
        err(e.scorer.score(& lane(), & req)), ScoreError::UnsupportedDType(DType::BF16)
    );
    assert_untouched(&e);
}
#[test]
fn unsupported_dtypes_are_rejected() {
    let dev = gpu_or_skip!();
    let e = env();
    for dtype in [DType::BF16, DType::U8, DType::U32, DType::I64] {
        let req = request(
            Metric::Dot,
            4,
            vec![with_slots(seg(& dev, 4, 4, dtype), 0..2)],
        );
        assert_eq!(
            err(e.scorer.score(& lane(), & req)), ScoreError::UnsupportedDType(dtype)
        );
    }
    assert_untouched(&e);
}
#[test]
fn mixed_dtypes_are_a_bad_layout() {
    let dev = gpu_or_skip!();
    let e = env();
    let req = request(
        Metric::Dot,
        4,
        vec![
            with_slots(seg(& dev, 4, 4, DType::F32), 0..2), with_slots(seg(& dev, 4, 4,
            DType::F16), 0..2),
        ],
    );
    assert!(matches!(err(e.scorer.score(& lane(), & req)), ScoreError::BadLayout(_)));
    assert_untouched(&e);
}
#[test]
fn rank_and_contiguity_are_checked() {
    let dev = gpu_or_skip!();
    let e = env();
    let mut s = seg(&dev, 4, 4, DType::F32);
    s.layout = Layout::contiguous(&[2, 2, 4], DType::F32);
    let req = request(Metric::Dot, 4, vec![with_slots(s, 0..2)]);
    assert!(matches!(err(e.scorer.score(& lane(), & req)), ScoreError::BadLayout(_)));
    let mut s = seg(&dev, 4, 4, DType::F32);
    s.layout = Layout::new(vec![4, 4], vec![32, 4], DType::F32).unwrap();
    let req = request(Metric::Dot, 4, vec![with_slots(s, 0..2)]);
    assert!(matches!(err(e.scorer.score(& lane(), & req)), ScoreError::BadLayout(_)));
    assert_untouched(&e);
}
#[test]
fn query_dim_mismatch_and_zero_dim() {
    let dev = gpu_or_skip!();
    let e = env();
    let mut req = request(
        Metric::Dot,
        3,
        vec![with_slots(seg(& dev, 4, 4, DType::F32), 0..2)],
    );
    assert_eq!(
        err(e.scorer.score(& lane(), & req)), ScoreError::QueryDimMismatch { query : 3,
        dim : 4 }
    );
    req.query.clear();
    assert_eq!(
        err(e.scorer.score(& lane(), & req)), ScoreError::QueryDimMismatch { query : 0,
        dim : 4 }
    );
    let req = request(
        Metric::Dot,
        0,
        vec![with_slots(seg(& dev, 4, 0, DType::F32), 0..0)],
    );
    assert!(matches!(err(e.scorer.score(& lane(), & req)), ScoreError::BadLayout(_)));
    assert_untouched(&e);
}
#[test]
fn layout_must_fit_region_and_block() {
    let dev = gpu_or_skip!();
    let e = env();
    let mut s = seg(&dev, 4, 4, DType::F32);
    s.region = region(ArenaKind::IndexSegments, 256, 63);
    let req = request(Metric::Dot, 4, vec![with_slots(s, 0..2)]);
    assert!(matches!(err(e.scorer.score(& lane(), & req)), ScoreError::BadLayout(_)));
    let mut s = seg(&dev, 4, 4, DType::F32);
    s.block.len_bytes = 256 + 63;
    let req = request(Metric::Dot, 4, vec![with_slots(s, 0..2)]);
    assert!(matches!(err(e.scorer.score(& lane(), & req)), ScoreError::BadLayout(_)));
    assert_untouched(&e);
}
#[test]
fn device_address_must_be_aligned_and_overflow_safe() {
    let dev = gpu_or_skip!();
    let e = env();
    let mut s = seg(&dev, 4, 4, DType::F32);
    s.region = region(ArenaKind::IndexSegments, 258, 64);
    s.block.len_bytes = 4096;
    let req = request(Metric::Dot, 4, vec![with_slots(s, 0..2)]);
    assert!(matches!(err(e.scorer.score(& lane(), & req)), ScoreError::BadLayout(_)));
    let mut s = seg(&dev, 4, 4, DType::F16);
    s.region = region(ArenaKind::IndexSegments, 257, 32);
    s.block.len_bytes = 4096;
    let req = request(Metric::Dot, 4, vec![with_slots(s, 0..2)]);
    assert!(matches!(err(e.scorer.score(& lane(), & req)), ScoreError::BadLayout(_)));
    let mut s = seg(&dev, 4, 4, DType::F32);
    s.block.base_addr = u64::MAX - 3;
    let req = request(Metric::Dot, 4, vec![with_slots(s, 0..2)]);
    assert!(matches!(err(e.scorer.score(& lane(), & req)), ScoreError::BadLayout(_)));
    let mut s = seg(&dev, 4, 4, DType::F32);
    s.block.base_addr = u64::MAX - 63;
    s.region = region(ArenaKind::IndexSegments, 0, 64);
    s.block.len_bytes = 4096;
    let req = request(Metric::Dot, 4, vec![with_slots(s, 0..2)]);
    assert!(matches!(err(e.scorer.score(& lane(), & req)), ScoreError::BadLayout(_)));
    assert_untouched(&e);
}
#[test]
fn slot_ranges_are_bounds_checked() {
    let dev = gpu_or_skip!();
    let e = env();
    #[allow(clippy::reversed_empty_ranges)]
    let reversed = 3..1;
    for slots in [0..5, 4..9, reversed] {
        let req = request(
            Metric::Dot,
            4,
            vec![with_slots(seg(& dev, 4, 4, DType::F32), slots.clone())],
        );
        assert_eq!(
            err(e.scorer.score(& lane(), & req)), ScoreError::OutOfBounds { slots, rows :
            4 }
        );
    }
    let req = request(
        Metric::Dot,
        4,
        vec![with_slots(seg(& dev, 4, 4, DType::F32), 0..4)],
    );
    assert_eq!(e.scorer.score(& lane(), & req).unwrap().finish().unwrap().len(), 4);
}
#[test]
fn first_failing_check_wins() {
    let dev = gpu_or_skip!();
    let e = env();
    let req = request(
        Metric::Dot,
        3,
        vec![with_slots(seg(& dev, 4, 4, DType::BF16), 0..9)],
    );
    assert_eq!(
        err(e.scorer.score(& lane(), & req)), ScoreError::UnsupportedDType(DType::BF16)
    );
    let req = request(
        Metric::Dot,
        3,
        vec![with_slots(seg(& dev, 4, 4, DType::F32), 0..9)],
    );
    assert!(
        matches!(err(e.scorer.score(& lane(), & req)), ScoreError::QueryDimMismatch { ..
        })
    );
    let mut s = seg(&dev, 4, 4, DType::F32);
    s.region = region(ArenaKind::IndexSegments, 256, 8);
    let req = request(Metric::Dot, 4, vec![with_slots(s, 0..9)]);
    assert!(matches!(err(e.scorer.score(& lane(), & req)), ScoreError::BadLayout(_)));
    assert_untouched(&e);
}
#[test]
fn cpu_device_register_reports_unavailable() {
    let dev = gpu_or_skip!();
    let e = env_with(DeviceHandle::Cpu, true, None, MockLauncher::default());
    let req = request(
        Metric::Dot,
        4,
        vec![with_slots(seg(& dev, 4, 4, DType::F32), 0..2)],
    );
    assert!(
        matches!(err(e.scorer.score(& lane(), & req)),
        ScoreError::Kernel(KernelError::Unavailable(_)))
    );
    assert!(e.launcher.calls.lock().unwrap().is_empty());
}
#[test]
fn unregistered_kernel_is_passed_through() {
    let dev = gpu_or_skip!();
    let e = env_with(DeviceHandle::Gpu(0), false, None, MockLauncher::default());
    let req = request(
        Metric::Dot,
        4,
        vec![with_slots(seg(& dev, 4, 4, DType::F32), 0..2)],
    );
    assert!(
        matches!(err(e.scorer.score(& lane(), & req)),
        ScoreError::Kernel(KernelError::UnknownKernel(_)))
    );
    assert!(e.launcher.calls.lock().unwrap().is_empty());
}
#[test]
fn compile_error_is_passed_through() {
    let dev = gpu_or_skip!();
    let fail = CompilerError::Failed {
        log: "bad".into(),
    };
    let e = env_with(DeviceHandle::Gpu(0), true, Some(fail), MockLauncher::default());
    let req = request(
        Metric::Dot,
        4,
        vec![with_slots(seg(& dev, 4, 4, DType::F32), 0..2)],
    );
    assert_eq!(
        err(e.scorer.score(& lane(), & req)),
        ScoreError::Kernel(KernelError::CompileError { log : "bad".into() })
    );
    assert!(e.launcher.calls.lock().unwrap().is_empty());
}
#[test]
fn launch_error_is_reported() {
    let dev = gpu_or_skip!();
    let launcher = MockLauncher {
        launch_error: Some("no memory".into()),
        ..MockLauncher::default()
    };
    let e = env_with(DeviceHandle::Gpu(0), true, None, launcher);
    let req = request(
        Metric::Dot,
        4,
        vec![with_slots(seg(& dev, 4, 4, DType::F32), 0..2)],
    );
    assert_eq!(
        err(e.scorer.score(& lane(), & req)), ScoreError::Launch("no memory".into())
    );
}
#[test]
fn lane_errors_are_reported() {
    let dev = gpu_or_skip!();
    let e = env();
    let req = request(
        Metric::Dot,
        4,
        vec![with_slots(seg(& dev, 4, 4, DType::F32), 0..2)],
    );
    let no_stream = Lane::new(lane_cfg(), None);
    assert_eq!(
        err(e.scorer.score(& no_stream, & req)),
        ScoreError::Lane(LaneError::NoStreamBackend)
    );
    let failing = lane_with(Arc::new(AtomicBool::new(true)), false, true);
    assert!(
        matches!(err(e.scorer.score(& failing, & req)),
        ScoreError::Lane(LaneError::Record(_)))
    );
}
#[test]
fn finish_reports_event_failure() {
    let dev = gpu_or_skip!();
    let e = env();
    let req = request(
        Metric::Dot,
        4,
        vec![with_slots(seg(& dev, 4, 4, DType::F32), 0..2)],
    );
    let failing = lane_with(Arc::new(AtomicBool::new(true)), true, false);
    let job = e.scorer.score(&failing, &req).unwrap();
    assert!(job.is_complete());
    assert!(matches!(job.finish(), Err(ScoreError::Lane(LaneError::Record(_)))));
}
#[test]
fn finish_reports_readback_failures() {
    let dev = gpu_or_skip!();
    let e = env();
    let req = request(
        Metric::Dot,
        4,
        vec![with_slots(seg(& dev, 4, 4, DType::F32), 0..2)],
    );
    *e.launcher.readback.lock().unwrap() = Some(Err("copy failed".into()));
    assert_eq!(
        e.scorer.score(& lane(), & req).unwrap().finish(),
        Err(ScoreError::Readback("copy failed".into()))
    );
    *e.launcher.readback.lock().unwrap() = Some(Ok(vec![1.0, 2.0, 3.0]));
    assert_eq!(
        e.scorer.score(& lane(), & req).unwrap().finish(),
        Err(ScoreError::Readback("length".into()))
    );
    *e.launcher.readback.lock().unwrap() = Some(Ok(vec![1.0]));
    assert_eq!(
        e.scorer.score(& lane(), & req).unwrap().finish(),
        Err(ScoreError::Readback("length".into()))
    );
}
#[test]
fn non_finite_scores_pass_through_unchanged() {
    let dev = gpu_or_skip!();
    let e = env();
    let req = request(
        Metric::Dot,
        4,
        vec![with_slots(seg(& dev, 4, 4, DType::F32), 0..4)],
    );
    *e.launcher.readback.lock().unwrap() = Some(
        Ok(vec![f32::NAN, f32::INFINITY, f32::NEG_INFINITY, - 0.5]),
    );
    let out = e.scorer.score(&lane(), &req).unwrap().finish().unwrap();
    assert!(out[0].score.is_nan());
    assert_eq!(out[1].score, f32::INFINITY);
    assert_eq!(out[2].score, f32::NEG_INFINITY);
    assert_eq!(out[3].score, - 0.5);
}
#[test]
fn is_complete_follows_the_lane_event() {
    let dev = gpu_or_skip!();
    let e = env();
    let req = request(
        Metric::Dot,
        4,
        vec![with_slots(seg(& dev, 4, 4, DType::F32), 0..2)],
    );
    let ready = Arc::new(AtomicBool::new(false));
    let job = e.scorer.score(&lane_with(ready.clone(), false, false), &req).unwrap();
    assert!(! job.is_complete());
    ready.store(true, Ordering::SeqCst);
    assert!(job.is_complete());
    assert_eq!(job.finish().unwrap().len(), 2);
}
#[test]
fn scoring_from_many_threads_with_separate_lanes() {
    let dev = gpu_or_skip!();
    let e = env();
    std::thread::scope(|scope| {
        let handles: Vec<_> = (0..4u32)
            .map(|t| {
                let scorer = &e.scorer;
                let dev = &dev;
                scope
                    .spawn(move || {
                        let lane = lane();
                        for _ in 0..8 {
                            let req = request(
                                Metric::Dot,
                                4,
                                vec![with_slots(seg(dev, 16, 4, DType::F32), 0.. (t + 1))],
                            );
                            let job = scorer.score(&lane, &req).unwrap();
                            let out = std::thread::spawn(move || job.finish())
                                .join()
                                .unwrap();
                            assert_eq!(out.unwrap().len(), (t + 1) as usize);
                        }
                    })
            })
            .collect();
        for h in handles {
            h.join().unwrap();
        }
    });
    assert_eq!(e.launcher.calls.lock().unwrap().len(), 32);
    assert_eq!(e.compiler.calls.load(Ordering::SeqCst), 1);
}
