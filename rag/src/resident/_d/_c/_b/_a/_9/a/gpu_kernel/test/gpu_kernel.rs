use super::*;
use std::any::Any;
use std::mem::{align_of, offset_of, size_of};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex as StdMutex;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::a::kernel_register::{
    CodeImage, CompileRequest, CompilerError, KernelCompiler, LoadFailure, ModuleLoader,
};
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::a::library_adapter::library_kernel_source;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::_1::device_lane_event::{
    DeviceHandle, GpuEventBackend, LaneConfig, LaneKind, StreamBackend,
};
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::_1::target_env::ComputeCapability;
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
struct CapturingCompiler {
    calls: AtomicUsize,
    requests: StdMutex<Vec<CompileRequest>>,
}
impl KernelCompiler for CapturingCompiler {
    fn compile(&self, req: &CompileRequest) -> Result<CodeImage, CompilerError> {
        self.calls.fetch_add(1, Ordering::SeqCst);
        self.requests.lock().unwrap().push(req.clone());
        Ok(CodeImage {
            kind: req.kind,
            bytes: vec![1],
        })
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
        _image: &CodeImage,
        symbol: &str,
    ) -> Result<Arc<dyn KernelFunction>, LoadFailure> {
        Ok(Arc::new(Func(symbol.to_string())))
    }
}
#[derive(Debug)]
struct MockEvent;
impl GpuEventBackend for MockEvent {
    fn query(&self) -> Result<bool, String> {
        Ok(true)
    }
    fn synchronize(&self) -> Result<(), String> {
        Ok(())
    }
}
#[derive(Debug)]
struct MockStream;
impl StreamBackend for MockStream {
    fn record_event(&self) -> Result<Box<dyn GpuEventBackend>, String> {
        Ok(Box::new(MockEvent))
    }
}
#[derive(Clone)]
struct Call {
    symbol: String,
    cfg: LaunchConfig,
    params: Vec<u8>,
}
#[derive(Default)]
struct MockLauncher {
    calls: StdMutex<Vec<Call>>,
    fail: StdMutex<Option<String>>,
}
impl GpuLauncher for MockLauncher {
    fn launch(
        &self,
        function: &Arc<dyn KernelFunction>,
        _lane: &Lane,
        cfg: &LaunchConfig,
        params: &[u8],
    ) -> Result<(), String> {
        if let Some(cause) = self.fail.lock().unwrap().clone() {
            return Err(cause);
        }
        self.calls
            .lock()
            .unwrap()
            .push(Call {
                symbol: function.symbol().to_string(),
                cfg: cfg.clone(),
                params: params.to_vec(),
            });
        Ok(())
    }
}
struct Fixture<const DIM: usize> {
    kernels: GpuKernels<DIM>,
    compiler: Arc<CapturingCompiler>,
    launcher: Arc<MockLauncher>,
}
fn fixture_on<const DIM: usize>(
    device: DeviceHandle,
    cc: Option<ComputeCapability>,
) -> Fixture<DIM> {
    let compiler = Arc::new(CapturingCompiler {
        calls: AtomicUsize::new(0),
        requests: StdMutex::new(Vec::new()),
    });
    let register = Arc::new(
        KernelRegister::new(
            device,
            compiler.clone(),
            Arc::new(MockLoader { cc }),
            library_kernel_source(),
        ),
    );
    register_gpu_kernels(&register).unwrap();
    let launcher = Arc::new(MockLauncher::default());
    Fixture {
        kernels: GpuKernels::new(register, launcher.clone()),
        compiler,
        launcher,
    }
}
fn fixture<const DIM: usize>() -> Fixture<DIM> {
    fixture_on(DeviceHandle::Gpu(0), Some(ComputeCapability::new(7, 5)))
}
fn lane() -> Lane {
    Lane::new(
        LaneConfig::preset(LaneKind::Query, DeviceHandle::Gpu(0)),
        Some(Box::new(MockStream)),
    )
}
fn params(head: u32, tail: u32) -> KernelParams {
    KernelParams {
        slots_addr: 0x1000,
        segments_addr: 0x2000,
        query_addr: 0x3000,
        out_addr: 0x4000,
        completed_addr: 0x5000,
        block_ctr_addr: 0x6000,
        ring_cap: 64,
        head,
        tail,
        segment_count: 1,
        query_len: 3,
        _reserved: 0,
    }
}
fn invalid(r: Result<Event, GpuKernelError>) -> String {
    match r {
        Err(GpuKernelError::InvalidParams(m)) => m,
        other => panic!("expected InvalidParams, got {other:?}"),
    }
}
#[test]
fn param_layout() {
    assert_eq!(size_of::< KernelParams > (), 72);
    assert_eq!(align_of::< KernelParams > (), 8);
    assert_eq!(offset_of!(KernelParams, slots_addr), 0);
    assert_eq!(offset_of!(KernelParams, segments_addr), 8);
    assert_eq!(offset_of!(KernelParams, query_addr), 16);
    assert_eq!(offset_of!(KernelParams, out_addr), 24);
    assert_eq!(offset_of!(KernelParams, completed_addr), 32);
    assert_eq!(offset_of!(KernelParams, block_ctr_addr), 40);
    assert_eq!(offset_of!(KernelParams, ring_cap), 48);
    assert_eq!(offset_of!(KernelParams, head), 52);
    assert_eq!(offset_of!(KernelParams, tail), 56);
    assert_eq!(offset_of!(KernelParams, segment_count), 60);
    assert_eq!(offset_of!(KernelParams, query_len), 64);
    assert_eq!(offset_of!(KernelParams, _reserved), 68);
    assert_eq!(size_of::< SegmentEntry > (), 40);
    assert_eq!(align_of::< SegmentEntry > (), 8);
    assert_eq!(offset_of!(SegmentEntry, base), 0);
    assert_eq!(offset_of!(SegmentEntry, lens_addr), 8);
    assert_eq!(offset_of!(SegmentEntry, stride_bytes), 16);
    assert_eq!(offset_of!(SegmentEntry, first_slot), 24);
    assert_eq!(offset_of!(SegmentEntry, slot_count), 28);
    assert_eq!(offset_of!(SegmentEntry, pad_len), 32);
    assert_eq!(offset_of!(SegmentEntry, _reserved), 36);
}
#[test]
fn serialized_bytes_follow_offsets() {
    let p = params(7, 12);
    let b = p.to_bytes();
    assert_eq!(u64::from_ne_bytes(b[0..8].try_into().unwrap()), 0x1000);
    assert_eq!(u64::from_ne_bytes(b[40..48].try_into().unwrap()), 0x6000);
    assert_eq!(u32::from_ne_bytes(b[48..52].try_into().unwrap()), 64);
    assert_eq!(u32::from_ne_bytes(b[52..56].try_into().unwrap()), 7);
    assert_eq!(u32::from_ne_bytes(b[56..60].try_into().unwrap()), 12);
    assert_eq!(u32::from_ne_bytes(b[60..64].try_into().unwrap()), 1);
    assert_eq!(u32::from_ne_bytes(b[64..68].try_into().unwrap()), 3);
    assert_eq!(u32::from_ne_bytes(b[68..72].try_into().unwrap()), 0);
}
fn entry_from_bytes(b: &[u8]) -> SegmentEntry {
    let u64_at = |at: usize| u64::from_ne_bytes(b[at..at + 8].try_into().unwrap());
    let u32_at = |at: usize| u32::from_ne_bytes(b[at..at + 4].try_into().unwrap());
    SegmentEntry {
        base: u64_at(offset_of!(SegmentEntry, base)),
        lens_addr: u64_at(offset_of!(SegmentEntry, lens_addr)),
        stride_bytes: u64_at(offset_of!(SegmentEntry, stride_bytes)),
        first_slot: u32_at(offset_of!(SegmentEntry, first_slot)),
        slot_count: u32_at(offset_of!(SegmentEntry, slot_count)),
        pad_len: u32_at(offset_of!(SegmentEntry, pad_len)),
        _reserved: u32_at(offset_of!(SegmentEntry, _reserved)),
    }
}
fn sample_entry(i: u32) -> SegmentEntry {
    SegmentEntry {
        base: 0x1000 * u64::from(i + 1),
        lens_addr: 0x2000_0000 + u64::from(i),
        stride_bytes: 512 + u64::from(i),
        first_slot: 7 + i,
        slot_count: 9 + i,
        pad_len: 3 + i,
        _reserved: 0xA5A5_0000 + i,
    }
}
#[test]
fn segment_entry_bytes_follow_offsets() {
    assert_eq!(SegmentEntry::BYTES, size_of::< SegmentEntry > ());
    let e = sample_entry(0);
    let b = e.to_bytes();
    assert_eq!(b.len(), 40);
    assert_eq!(u64::from_ne_bytes(b[0..8].try_into().unwrap()), e.base);
    assert_eq!(u64::from_ne_bytes(b[8..16].try_into().unwrap()), e.lens_addr);
    assert_eq!(u64::from_ne_bytes(b[16..24].try_into().unwrap()), e.stride_bytes);
    assert_eq!(u32::from_ne_bytes(b[24..28].try_into().unwrap()), e.first_slot);
    assert_eq!(u32::from_ne_bytes(b[28..32].try_into().unwrap()), e.slot_count);
    assert_eq!(u32::from_ne_bytes(b[32..36].try_into().unwrap()), e.pad_len);
    assert_eq!(u32::from_ne_bytes(b[36..40].try_into().unwrap()), e._reserved);
    assert_eq!(entry_from_bytes(& b), e);
}
#[test]
fn segment_table_encoding_is_contiguous() {
    assert!(encode_segment_table(& []).is_empty());
    let entries = [sample_entry(0), sample_entry(1), sample_entry(2)];
    let bytes = encode_segment_table(&entries);
    assert_eq!(bytes.len(), 120);
    for (i, e) in entries.iter().enumerate() {
        assert_eq!(& bytes[40 * i..40 * i + 40], & e.to_bytes() [..]);
        assert_eq!(entry_from_bytes(& bytes[40 * i..40 * i + 40]), * e);
    }
}
#[test]
fn constants() {
    assert_eq!(BLOCK_THREADS, 128);
    assert_eq!(NO_VALID_TOKEN_SCORE, - 10_000.0);
    assert_eq!(NO_VALID_TOKEN_SCORE, MASK_VALUE);
}
#[test]
fn first_batch_launches_and_writes_params() {
    let f = fixture::<8>();
    let mut lane = lane();
    lane.params_mut()[100] = 0xAB;
    let p = params(0, 5);
    f.kernels.dense_dot(&mut lane, DType::F32, &p, 0).unwrap();
    assert_eq!(f.kernels.launched(), 1);
    let calls = f.launcher.calls.lock().unwrap();
    assert_eq!(calls.len(), 1);
    assert_eq!(calls[0].symbol, "xy_dense_dot");
    assert_eq!(calls[0].params, p.to_bytes().to_vec());
    assert_eq!(& lane.params() [..72], & p.to_bytes() [..]);
    assert_eq!(lane.params() [100], 0xAB);
    assert_eq!(lane.params().len(), 256);
}
#[test]
fn second_batch_requires_ack() {
    let f = fixture::<8>();
    let mut lane = lane();
    let p = params(0, 2);
    f.kernels.dense_dot(&mut lane, DType::F32, &p, 0).unwrap();
    let err = f.kernels.dense_dot(&mut lane, DType::F32, &p, 0).unwrap_err();
    assert_eq!(err, GpuKernelError::AckBehind { required : 1, acked : 0 });
    assert_eq!(f.kernels.launched(), 1);
    assert_eq!(f.launcher.calls.lock().unwrap().len(), 1);
    f.kernels.dense_dot(&mut lane, DType::F32, &p, 1).unwrap();
    assert_eq!(f.kernels.launched(), 2);
}
#[test]
fn ack_check_spans_both_kernels() {
    let f = fixture::<8>();
    let mut lane = lane();
    let p = params(0, 2);
    f.kernels.dense_dot(&mut lane, DType::F32, &p, 0).unwrap();
    let err = f.kernels.fused_max_sim(&mut lane, DType::F32, &p, 0).unwrap_err();
    assert_eq!(err, GpuKernelError::AckBehind { required : 1, acked : 0 });
    f.kernels.fused_max_sim(&mut lane, DType::F16, &p, 1).unwrap();
    let calls = f.launcher.calls.lock().unwrap();
    assert_eq!(calls[1].symbol, "xy_fused_maxsim");
}
#[test]
fn launch_config_follows_slot_total() {
    let f = fixture::<8>();
    let mut lane = lane();
    let mut acked = 0;
    for (head, tail, grid) in [(0u32, 0u32, 1u32), (0, 1, 1), (3, 8, 5), (10, 10, 1)] {
        f.kernels
            .fused_max_sim(&mut lane, DType::F32, &params(head, tail), acked)
            .unwrap();
        acked += 1;
        let calls = f.launcher.calls.lock().unwrap();
        let cfg = &calls.last().unwrap().cfg;
        assert_eq!(cfg.grid, grid);
        assert_eq!(cfg.block, 128);
        assert_eq!(cfg.dynamic_shared, 0);
    }
}
#[test]
fn counters_wrap() {
    let p = params(u32::MAX - 1, 2);
    assert_eq!(p.slot_total(), 4);
    let f = fixture::<8>();
    let mut lane = lane();
    f.kernels.dense_dot(&mut lane, DType::F32, &p, 0).unwrap();
    assert_eq!(f.launcher.calls.lock().unwrap() [0].cfg.grid, 4);
}
#[test]
fn launcher_error_keeps_cause_and_counter() {
    let f = fixture::<8>();
    let mut lane = lane();
    let p = params(0, 3);
    *f.launcher.fail.lock().unwrap() = Some(
        "CUDA_ERROR_LAUNCH_OUT_OF_RESOURCES".to_string(),
    );
    let err = f.kernels.dense_dot(&mut lane, DType::F32, &p, 0).unwrap_err();
    assert_eq!(
        err, GpuKernelError::Launch { kernel : KERNEL_DENSE_DOT, grid : 3, block : 128,
        cause : "CUDA_ERROR_LAUNCH_OUT_OF_RESOURCES".to_string(), }
    );
    assert!(err.to_string().contains("CUDA_ERROR_LAUNCH_OUT_OF_RESOURCES"));
    assert_eq!(f.kernels.launched(), 0);
    *f.launcher.fail.lock().unwrap() = None;
    f.kernels.dense_dot(&mut lane, DType::F32, &p, 0).unwrap();
    assert_eq!(f.kernels.launched(), 1);
}
#[test]
fn bf16_is_unsupported_and_nothing_is_fetched() {
    let f = fixture::<8>();
    let mut lane = lane();
    let p = params(0, 2);
    for r in [
        f.kernels.dense_dot(&mut lane, DType::BF16, &p, 0),
        f.kernels.fused_max_sim(&mut lane, DType::BF16, &p, 0),
    ] {
        assert_eq!(r.unwrap_err(), GpuKernelError::UnsupportedDType(DType::BF16));
    }
    let bad = KernelParams { query_len: 0, ..p };
    assert_eq!(
        f.kernels.dense_dot(& mut lane, DType::U8, & bad, 0).unwrap_err(),
        GpuKernelError::UnsupportedDType(DType::U8)
    );
    assert_eq!(f.compiler.calls.load(Ordering::SeqCst), 0);
    assert!(f.launcher.calls.lock().unwrap().is_empty());
    assert_eq!(f.kernels.launched(), 0);
}
#[test]
fn invalid_params_are_rejected_before_any_fetch() {
    let f = fixture::<8>();
    let mut lane = lane();
    let ok = params(0, 2);
    let cases = [
        KernelParams { query_len: 0, ..ok },
        KernelParams { ring_cap: 0, ..ok },
        KernelParams { ring_cap: 1, ..ok },
        KernelParams {
            head: 0,
            tail: i32::MAX as u32 + 1,
            ring_cap: u32::MAX,
            ..ok
        },
        KernelParams {
            segment_count: 0,
            ..ok
        },
    ];
    for p in cases {
        invalid(f.kernels.dense_dot(&mut lane, DType::F32, &p, 0));
        invalid(f.kernels.fused_max_sim(&mut lane, DType::F16, &p, 0));
    }
    assert_eq!(f.compiler.calls.load(Ordering::SeqCst), 0);
    assert!(f.launcher.calls.lock().unwrap().is_empty());
    assert_eq!(f.kernels.launched(), 0);
}
#[test]
fn empty_batch_needs_no_segments() {
    let f = fixture::<8>();
    let mut lane = lane();
    let p = KernelParams {
        segment_count: 0,
        ..params(4, 4)
    };
    f.kernels.dense_dot(&mut lane, DType::F32, &p, 0).unwrap();
    assert_eq!(f.launcher.calls.lock().unwrap() [0].cfg.grid, 1);
}
#[test]
fn small_param_block_is_rejected() {
    let f = fixture::<8>();
    let mut cfg = LaneConfig::preset(LaneKind::Query, DeviceHandle::Gpu(0));
    cfg.param_block_size = 64;
    let mut lane = Lane::new(cfg, Some(Box::new(MockStream)));
    invalid(f.kernels.dense_dot(&mut lane, DType::F32, &params(0, 1), 0));
    assert_eq!(f.compiler.calls.load(Ordering::SeqCst), 0);
}
#[test]
fn ack_error_precedes_kernel_fetch() {
    let f = fixture::<8>();
    let mut lane = lane();
    let p = params(0, 2);
    f.kernels.dense_dot(&mut lane, DType::F32, &p, 0).unwrap();
    let before = f.compiler.calls.load(Ordering::SeqCst);
    let _ = f.kernels.dense_dot(&mut lane, DType::F16, &p, 0).unwrap_err();
    assert_eq!(f.compiler.calls.load(Ordering::SeqCst), before);
}
#[test]
fn unavailable_kernel_maps_to_kernel_error() {
    let f = fixture_on::<8>(DeviceHandle::Cpu, None);
    let mut lane = lane();
    let err = f.kernels.dense_dot(&mut lane, DType::F32, &params(0, 2), 0).unwrap_err();
    assert!(matches!(err, GpuKernelError::Kernel(KernelError::Unavailable(_))));
    assert_eq!(f.kernels.launched(), 0);
}
#[test]
fn unregistered_source_maps_to_kernel_error() {
    let compiler = Arc::new(CapturingCompiler {
        calls: AtomicUsize::new(0),
        requests: StdMutex::new(Vec::new()),
    });
    let register = Arc::new(
        KernelRegister::new(
            DeviceHandle::Gpu(0),
            compiler.clone(),
            Arc::new(MockLoader {
                cc: Some(ComputeCapability::new(7, 5)),
            }),
            library_kernel_source(),
        ),
    );
    let launcher = Arc::new(MockLauncher::default());
    let kernels = GpuKernels::<8>::new(register, launcher.clone());
    let mut lane = lane();
    let err = kernels
        .fused_max_sim(&mut lane, DType::F32, &params(0, 2), 0)
        .unwrap_err();
    assert_eq!(
        err, GpuKernelError::Kernel(KernelError::UnknownKernel("fused_maxsim"
        .to_string()))
    );
    assert_eq!(kernels.launched(), 0);
    assert!(launcher.calls.lock().unwrap().is_empty());
}
#[test]
fn lane_without_stream_counts_the_batch() {
    let f = fixture::<8>();
    let mut lane = Lane::new(
        LaneConfig::preset(LaneKind::Query, DeviceHandle::Gpu(0)),
        None,
    );
    let err = f.kernels.dense_dot(&mut lane, DType::F32, &params(0, 2), 0).unwrap_err();
    assert_eq!(err, GpuKernelError::Lane(LaneError::NoStreamBackend));
    assert_eq!(f.kernels.launched(), 1);
    assert_eq!(f.launcher.calls.lock().unwrap().len(), 1);
}
#[test]
fn event_is_returned() {
    let f = fixture::<8>();
    let mut lane = lane();
    let ev = f.kernels.fused_max_sim(&mut lane, DType::F32, &params(0, 2), 0).unwrap();
    assert!(ev.is_complete().unwrap());
}
#[test]
fn second_registration_fails() {
    let f = fixture::<8>();
    let register = Arc::new(
        KernelRegister::new(
            DeviceHandle::Gpu(0),
            f.compiler.clone(),
            Arc::new(MockLoader {
                cc: Some(ComputeCapability::new(7, 5)),
            }),
            library_kernel_source(),
        ),
    );
    register_gpu_kernels(&register).unwrap();
    assert!(
        matches!(register_gpu_kernels(& register), Err(KernelError::Unavailable(_)))
    );
    assert_eq!(register.len(), 0);
}
#[test]
fn concurrent_calls_get_distinct_batch_numbers() {
    let f = Arc::new(fixture::<8>());
    let mut handles = Vec::new();
    for _ in 0..4 {
        let f = f.clone();
        handles
            .push(
                std::thread::spawn(move || {
                    let mut lane = lane();
                    for _ in 0..25 {
                        f.kernels
                            .dense_dot(&mut lane, DType::F32, &params(0, 2), u64::MAX)
                            .unwrap();
                    }
                }),
            );
    }
    for h in handles {
        h.join().unwrap();
    }
    assert_eq!(f.kernels.launched(), 100);
    assert_eq!(f.launcher.calls.lock().unwrap().len(), 100);
}
fn requested_sources<const DIM: usize>() -> Vec<CompileRequest> {
    let f = fixture::<DIM>();
    let mut lane = lane();
    let p = params(0, 2);
    for dtype in [DType::F32, DType::F16] {
        f.kernels.dense_dot(&mut lane, dtype, &p, u64::MAX).unwrap();
        f.kernels.fused_max_sim(&mut lane, dtype, &p, u64::MAX).unwrap();
    }
    let requests = f.compiler.requests.lock().unwrap().clone();
    requests
}
#[test]
fn kernel_sources_are_dim_specific_and_static() {
    let sources = requested_sources::<384>();
    assert_eq!(sources.len(), 4);
    for req in &sources {
        let s = &req.source;
        assert!(s.contains("#define DIM 384"));
        assert!(! s.contains("TILE"));
        assert!(! s.contains("extern __shared__"));
        assert!(! s.contains("extern  __shared__"));
        assert!(s.contains("extern \"C\" __global__"));
        assert!(s.contains("atom.add.release.sys.u64"));
        assert!(s.contains("__threadfence()"));
    }
    assert!(DENSE_DOT_BODY.contains("__shared__ float sums[4]"));
    assert!(! DENSE_DOT_BODY.contains("extern __shared__"));
    assert!(! FUSED_MAXSIM_BODY.contains("extern __shared__"));
    assert!(DENSE_DOT_BODY.contains(ENTRY_DENSE_DOT));
    assert!(FUSED_MAXSIM_BODY.contains(ENTRY_FUSED_MAXSIM));
    assert!(DENSE_DOT_BODY.contains(& format!("#define BLOCK_THREADS {BLOCK_THREADS}")));
}
fn find_nvcc() -> Option<std::path::PathBuf> {
    let fixed = std::path::PathBuf::from("/usr/local/cuda-13.3/bin/nvcc");
    if fixed.exists() {
        return Some(fixed);
    }
    std::process::Command::new("nvcc")
        .arg("--version")
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|_| std::path::PathBuf::from("nvcc"))
}
#[test]
fn kernel_sources_compile_with_nvcc() {
    let nvcc = find_nvcc().unwrap();
    let dir = std::env::temp_dir()
        .join(format!("xy_gpu_kernel_nvcc_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    for (i, req) in requested_sources::<384>().iter().enumerate() {
        let src = dir.join(format!("k{i}.cu"));
        let out = dir.join(format!("k{i}.cubin"));
        std::fs::write(&src, &req.source).unwrap();
        let result = std::process::Command::new(&nvcc)
            .arg("-cubin")
            .arg("-arch=sm_75")
            .arg("-o")
            .arg(&out)
            .arg(&src)
            .output()
            .unwrap();
        assert!(
            result.status.success(), "nvcc failed for {}: {}", req.file_name,
            String::from_utf8_lossy(& result.stderr)
        );
    }
    let _ = std::fs::remove_dir_all(&dir);
}
mod device {
    use super::*;
    use std::ffi::{c_void, CString};
    use std::path::PathBuf;
    use std::ptr::NonNull;
    use std::sync::atomic::AtomicU64;
    use std::time::{Duration, Instant};
    use cudarc::driver::result as drv;
    use cudarc::driver::{sys, CudaContext};
    use crate::resident::_d::_c::_b::_a::_9::_8::_7::a::kernel_register::CodeKind;
    use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::b::doorbell::{
        Doorbell, DoorbellCell,
    };
    const SENTINEL: f32 = 12345.0;
    const POISON: f32 = 30_000.0;
    const SPIN_LIMIT: Duration = Duration::from_secs(20);
    fn text(e: drv::DriverError) -> String {
        e.to_string()
    }
    struct StreamHandle {
        ctx: Arc<CudaContext>,
        raw: sys::CUstream,
    }
    unsafe impl Send for StreamHandle {}
    unsafe impl Sync for StreamHandle {}
    impl std::fmt::Debug for StreamHandle {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "StreamHandle({:p})", self.raw)
        }
    }
    impl Drop for StreamHandle {
        fn drop(&mut self) {
            if self.ctx.bind_to_thread().is_ok() {
                let _ = unsafe { drv::stream::destroy(self.raw) };
            }
        }
    }
    #[derive(Debug)]
    struct DriverStream {
        owner: Arc<StreamHandle>,
    }
    struct DriverEvent {
        ctx: Arc<CudaContext>,
        raw: sys::CUevent,
    }
    unsafe impl Send for DriverEvent {}
    unsafe impl Sync for DriverEvent {}
    impl std::fmt::Debug for DriverEvent {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "DriverEvent({:p})", self.raw)
        }
    }
    impl Drop for DriverEvent {
        fn drop(&mut self) {
            if self.ctx.bind_to_thread().is_ok() {
                let _ = unsafe { drv::event::destroy(self.raw) };
            }
        }
    }
    impl GpuEventBackend for DriverEvent {
        fn query(&self) -> Result<bool, String> {
            self.ctx.bind_to_thread().map_err(text)?;
            match unsafe { drv::event::query(self.raw) } {
                Ok(()) => Ok(true),
                Err(e) if e.0 == sys::CUresult::CUDA_ERROR_NOT_READY => Ok(false),
                Err(e) => Err(text(e)),
            }
        }
        fn synchronize(&self) -> Result<(), String> {
            self.ctx.bind_to_thread().map_err(text)?;
            unsafe { drv::event::synchronize(self.raw) }.map_err(text)
        }
    }
    impl StreamBackend for DriverStream {
        fn record_event(&self) -> Result<Box<dyn GpuEventBackend>, String> {
            let ctx = &self.owner.ctx;
            ctx.bind_to_thread().map_err(text)?;
            let raw = drv::event::create(sys::CUevent_flags::CU_EVENT_DISABLE_TIMING)
                .map_err(text)?;
            let event = DriverEvent {
                ctx: Arc::clone(ctx),
                raw,
            };
            unsafe { drv::event::record(event.raw, self.owner.raw) }.map_err(text)?;
            Ok(Box::new(event))
        }
    }
    struct ModuleOwner {
        ctx: Arc<CudaContext>,
        raw: sys::CUmodule,
    }
    unsafe impl Send for ModuleOwner {}
    unsafe impl Sync for ModuleOwner {}
    impl Drop for ModuleOwner {
        fn drop(&mut self) {
            if self.ctx.bind_to_thread().is_ok() {
                let _ = unsafe { drv::module::unload(self.raw) };
            }
        }
    }
    struct DriverFunction {
        symbol: String,
        raw: sys::CUfunction,
        _module: Arc<ModuleOwner>,
    }
    unsafe impl Send for DriverFunction {}
    unsafe impl Sync for DriverFunction {}
    impl std::fmt::Debug for DriverFunction {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "DriverFunction({})", self.symbol)
        }
    }
    impl KernelFunction for DriverFunction {
        fn symbol(&self) -> &str {
            &self.symbol
        }
        fn as_any(&self) -> &dyn Any {
            self
        }
    }
    struct DriverLoader {
        ctx: Arc<CudaContext>,
        cc: ComputeCapability,
    }
    impl ModuleLoader for DriverLoader {
        fn device_capability(&self) -> Option<ComputeCapability> {
            Some(self.cc)
        }
        fn load(
            &self,
            image: &CodeImage,
            symbol: &str,
        ) -> Result<Arc<dyn KernelFunction>, LoadFailure> {
            self.ctx.bind_to_thread().map_err(|e| LoadFailure::Other(text(e)))?;
            let mut data = image.bytes.clone();
            data.push(0);
            let raw = unsafe { drv::module::load_data(data.as_ptr().cast()) }
                .map_err(|e| {
                    if image.kind == CodeKind::Ptx {
                        LoadFailure::PtxRejected(text(e))
                    } else {
                        LoadFailure::Other(text(e))
                    }
                })?;
            let module = Arc::new(ModuleOwner {
                ctx: Arc::clone(&self.ctx),
                raw,
            });
            let name = CString::new(symbol)
                .map_err(|e| LoadFailure::Other(e.to_string()))?;
            let func = unsafe { drv::module::get_function(raw, name) }
                .map_err(|e| {
                    if e.0 == sys::CUresult::CUDA_ERROR_NOT_FOUND {
                        LoadFailure::MissingSymbol(symbol.to_string())
                    } else {
                        LoadFailure::Other(text(e))
                    }
                })?;
            Ok(
                Arc::new(DriverFunction {
                    symbol: symbol.to_string(),
                    raw: func,
                    _module: module,
                }),
            )
        }
    }
    struct NvccCompiler {
        nvcc: PathBuf,
        dir: PathBuf,
    }
    impl KernelCompiler for NvccCompiler {
        fn compile(&self, req: &CompileRequest) -> Result<CodeImage, CompilerError> {
            let unavailable = |e: std::io::Error| CompilerError::Unavailable(
                e.to_string(),
            );
            let src = self.dir.join(&req.file_name);
            let ext = match req.kind {
                CodeKind::Ptx => "ptx",
                CodeKind::Cubin => "cubin",
            };
            let out = self.dir.join(format!("{}.{ext}", req.file_name));
            std::fs::write(&src, &req.source).map_err(unavailable)?;
            let mut cmd = std::process::Command::new(&self.nvcc);
            match req.kind {
                CodeKind::Ptx => {
                    cmd.arg("-ptx")
                        .arg(
                            format!("-arch={}", req.arch.replacen("sm_", "compute_", 1)),
                        );
                }
                CodeKind::Cubin => {
                    cmd.arg("-cubin").arg(format!("-arch={}", req.arch));
                }
            }
            let output = cmd
                .arg("-o")
                .arg(&out)
                .arg(&src)
                .output()
                .map_err(unavailable)?;
            if !output.status.success() {
                return Err(CompilerError::Failed {
                    log: String::from_utf8_lossy(&output.stderr).into_owned(),
                });
            }
            let bytes = std::fs::read(&out).map_err(unavailable)?;
            Ok(CodeImage { kind: req.kind, bytes })
        }
    }
    struct DriverLauncher {
        ctx: Arc<CudaContext>,
        owner: Arc<StreamHandle>,
    }
    impl GpuLauncher for DriverLauncher {
        fn launch(
            &self,
            function: &Arc<dyn KernelFunction>,
            _lane: &Lane,
            cfg: &LaunchConfig,
            params: &[u8],
        ) -> Result<(), String> {
            let f = function
                .as_any()
                .downcast_ref::<DriverFunction>()
                .ok_or_else(|| "function was not loaded by this driver".to_string())?;
            self.ctx.bind_to_thread().map_err(text)?;
            let mut bytes = params.to_vec();
            let mut args = [bytes.as_mut_ptr().cast::<c_void>()];
            unsafe {
                drv::launch_kernel(
                    f.raw,
                    (cfg.grid, 1, 1),
                    (cfg.block, 1, 1),
                    cfg.dynamic_shared,
                    self.owner.raw,
                    &mut args,
                )
            }
                .map_err(text)
        }
    }
    struct MappedCell {
        ctx: Arc<CudaContext>,
        host: *mut c_void,
        dev: u64,
    }
    unsafe impl Send for MappedCell {}
    unsafe impl Sync for MappedCell {}
    impl MappedCell {
        fn new(ctx: &Arc<CudaContext>) -> MappedCell {
            ctx.bind_to_thread().unwrap();
            let host = unsafe { drv::malloc_host(8, sys::CU_MEMHOSTALLOC_DEVICEMAP) }
                .unwrap();
            let mut dev: sys::CUdeviceptr = 0;
            unsafe { sys::cuMemHostGetDevicePointer_v2(&mut dev, host, 0) }
                .result()
                .unwrap();
            unsafe { std::ptr::write_bytes(host.cast::<u8>(), 0, 8) };
            MappedCell {
                ctx: Arc::clone(ctx),
                host,
                dev: dev as u64,
            }
        }
    }
    impl Drop for MappedCell {
        fn drop(&mut self) {
            if self.ctx.bind_to_thread().is_ok() {
                let _ = unsafe { drv::free_host(self.host) };
            }
        }
    }
    struct DevBuf {
        ctx: Arc<CudaContext>,
        ptr: sys::CUdeviceptr,
    }
    impl DevBuf {
        fn from_slice<T: Copy>(ctx: &Arc<CudaContext>, data: &[T]) -> DevBuf {
            ctx.bind_to_thread().unwrap();
            let bytes = std::mem::size_of_val(data).max(8);
            let ptr = unsafe { drv::malloc_sync(bytes) }.unwrap();
            unsafe { drv::memset_d8_sync(ptr, 0, bytes) }.unwrap();
            if !data.is_empty() {
                unsafe { drv::memcpy_htod_sync(ptr, data) }.unwrap();
            }
            DevBuf {
                ctx: Arc::clone(ctx),
                ptr,
            }
        }
        fn zeroed(ctx: &Arc<CudaContext>, bytes: usize) -> DevBuf {
            DevBuf::from_slice(ctx, &vec![0u8; bytes])
        }
        fn addr(&self) -> u64 {
            self.ptr as u64
        }
        fn fill<T: Copy>(&self, data: &[T]) {
            self.ctx.bind_to_thread().unwrap();
            unsafe { drv::memcpy_htod_sync(self.ptr, data) }.unwrap();
        }
        fn read<T: Copy + Default>(&self, n: usize) -> Vec<T> {
            self.ctx.bind_to_thread().unwrap();
            let mut v = vec![T::default(); n];
            unsafe { drv::memcpy_dtoh_sync(&mut v, self.ptr) }.unwrap();
            v
        }
    }
    impl Drop for DevBuf {
        fn drop(&mut self) {
            if self.ctx.bind_to_thread().is_ok() {
                let _ = unsafe { drv::free_sync(self.ptr) };
            }
        }
    }
    struct Rig<const DIM: usize> {
        ctx: Arc<CudaContext>,
        dtype: DType,
        kernels: GpuKernels<DIM>,
        lane: Lane,
        doorbell: Doorbell,
        completed_dev: u64,
        block_ctr: DevBuf,
        dir: PathBuf,
    }
    impl<const DIM: usize> Drop for Rig<DIM> {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.dir);
        }
    }
    static RIG_COUNTER: AtomicUsize = AtomicUsize::new(0);
    impl<const DIM: usize> Rig<DIM> {
        fn new(dtype: DType) -> Rig<DIM> {
            let nvcc = find_nvcc().unwrap();
            let ctx = CudaContext::new(0).unwrap();
            ctx.bind_to_thread().unwrap();
            let (major, minor) = ctx.compute_capability().unwrap();
            let dir = std::env::temp_dir()
                .join(
                    format!(
                        "xy_gpu_kernel_launch_{}_{}", std::process::id(), RIG_COUNTER
                        .fetch_add(1, Ordering::SeqCst)
                    ),
                );
            let _ = std::fs::remove_dir_all(&dir);
            std::fs::create_dir_all(&dir).unwrap();
            let raw = drv::stream::create(drv::stream::StreamKind::NonBlocking).unwrap();
            let owner = Arc::new(StreamHandle {
                ctx: Arc::clone(&ctx),
                raw,
            });
            let register = Arc::new(
                KernelRegister::new(
                    DeviceHandle::Gpu(0),
                    Arc::new(NvccCompiler {
                        nvcc,
                        dir: dir.clone(),
                    }),
                    Arc::new(DriverLoader {
                        ctx: Arc::clone(&ctx),
                        cc: ComputeCapability::new(major as u32, minor as u32),
                    }),
                    library_kernel_source(),
                ),
            );
            register_gpu_kernels(&register).unwrap();
            let launcher = Arc::new(DriverLauncher {
                ctx: Arc::clone(&ctx),
                owner: Arc::clone(&owner),
            });
            let lane = Lane::new(
                LaneConfig::preset(LaneKind::Query, DeviceHandle::Gpu(0)),
                Some(Box::new(DriverStream { owner })),
            );
            let cell = Arc::new(MappedCell::new(&ctx));
            let completed_dev = cell.dev;
            let ptr = NonNull::new(cell.host.cast::<AtomicU64>()).unwrap();
            let completed = unsafe { DoorbellCell::from_raw(ptr, cell) };
            let block_ctr = DevBuf::zeroed(&ctx, 8);
            Rig {
                ctx,
                dtype,
                kernels: GpuKernels::new(register, launcher),
                lane,
                doorbell: Doorbell::new(completed, DoorbellCell::new_local()),
                completed_dev,
                block_ctr,
                dir,
            }
        }
        fn launch(
            &mut self,
            fused: bool,
            p: &KernelParams,
        ) -> Result<Event, GpuKernelError> {
            let acked = self.doorbell.acked();
            if fused {
                self.kernels.fused_max_sim(&mut self.lane, self.dtype, p, acked)
            } else {
                self.kernels.dense_dot(&mut self.lane, self.dtype, p, acked)
            }
        }
        fn ack(&mut self) {
            let n = self.kernels.launched();
            let gap = self.doorbell.poll().unwrap().expect("completed did not advance");
            assert_eq!(gap.last(), n);
            assert_eq!(gap.len(), 1);
            gap.close();
        }
        fn wait_by_spinning(&mut self) {
            let start = Instant::now();
            loop {
                if let Some(gap) = self.doorbell.poll().unwrap() {
                    assert_eq!(gap.last(), self.kernels.launched());
                    gap.close();
                    return;
                }
                assert!(start.elapsed() < SPIN_LIMIT, "doorbell did not advance");
                std::hint::spin_loop();
            }
        }
        fn params(
            &self,
            ring: &DevBuf,
            table: Option<&Table>,
            query: &DevBuf,
            out: &DevBuf,
            (ring_cap, head, total, query_len): (u32, u32, u32, u32),
        ) -> KernelParams {
            KernelParams {
                slots_addr: ring.addr(),
                segments_addr: table.map_or(0, |t| t.table.addr()),
                query_addr: query.addr(),
                out_addr: out.addr(),
                completed_addr: self.completed_dev,
                block_ctr_addr: self.block_ctr.addr(),
                ring_cap,
                head,
                tail: head.wrapping_add(total),
                segment_count: table.map_or(0, |t| t.count),
                query_len,
                _reserved: 0,
            }
        }
    }
    #[derive(Clone, Copy, PartialEq)]
    enum Wait {
        Event,
        EventAfterAckProbe,
        Spin,
    }
    fn settle<const DIM: usize>(
        rig: &mut Rig<DIM>,
        p: &KernelParams,
        ev: Event,
        wait: Wait,
    ) {
        match wait {
            Wait::Spin => rig.wait_by_spinning(),
            Wait::Event | Wait::EventAfterAckProbe => {
                ev.wait().unwrap();
                if wait == Wait::EventAfterAckProbe {
                    let n = rig.kernels.launched();
                    let behind = GpuKernelError::AckBehind {
                        required: n,
                        acked: n - 1,
                    };
                    assert_eq!(rig.launch(false, p).unwrap_err(), behind);
                    assert_eq!(rig.launch(true, p).unwrap_err(), behind);
                    assert_eq!(rig.kernels.launched(), n);
                }
                rig.ack();
            }
        }
        assert_eq!(rig.block_ctr.read::< u32 > (1) [0], 0);
    }
    struct Lcg(u64);
    impl Lcg {
        fn next(&mut self) -> f32 {
            self.0 = self
                .0
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            ((self.0 >> 40) as f32 / (1u64 << 24) as f32) * 2.0 - 1.0
        }
    }
    fn elem(dtype: DType) -> usize {
        match dtype {
            DType::F32 => 4,
            DType::F16 => 2,
            other => panic!("unexpected dtype {other:?}"),
        }
    }
    fn quantize(dtype: DType, x: f32) -> f32 {
        match dtype {
            DType::F16 => half::f16::from_f32(x).to_f32(),
            _ => x,
        }
    }
    fn encode(dtype: DType, v: &[f32]) -> Vec<u8> {
        match dtype {
            DType::F32 => v.iter().flat_map(|x| x.to_ne_bytes()).collect(),
            DType::F16 => {
                v.iter()
                    .flat_map(|x| half::f16::from_f32(*x).to_bits().to_ne_bytes())
                    .collect()
            }
            other => panic!("unexpected dtype {other:?}"),
        }
    }
    fn random_doc(rng: &mut Lcg, dtype: DType, n: usize) -> Vec<f32> {
        (0..n).map(|_| quantize(dtype, rng.next())).collect()
    }
    struct Seg {
        first: u32,
        count: u32,
        pad_len: u32,
        lens: Option<Vec<u32>>,
    }
    struct Table {
        table: DevBuf,
        count: u32,
        _keep: Vec<DevBuf>,
    }
    fn build_table(
        ctx: &Arc<CudaContext>,
        dtype: DType,
        docs: &[Vec<f32>],
        segs: &[Seg],
    ) -> Table {
        let mut entries = Vec::new();
        let mut keep = Vec::new();
        for s in segs {
            let slots = &docs[s.first as usize..(s.first + s.count) as usize];
            let stride = slots[0].len() * elem(dtype);
            let bytes: Vec<u8> = slots.iter().flat_map(|d| encode(dtype, d)).collect();
            let buf = DevBuf::from_slice(ctx, &bytes);
            let lens_addr = match &s.lens {
                Some(l) => {
                    let b = DevBuf::from_slice(ctx, l);
                    let a = b.addr();
                    keep.push(b);
                    a
                }
                None => 0,
            };
            entries
                .push(SegmentEntry {
                    base: buf.addr(),
                    lens_addr,
                    stride_bytes: stride as u64,
                    first_slot: s.first,
                    slot_count: s.count,
                    pad_len: s.pad_len,
                    _reserved: 0,
                });
            keep.push(buf);
        }
        Table {
            table: DevBuf::from_slice(ctx, &entries),
            count: entries.len() as u32,
            _keep: keep,
        }
    }
    fn build_ring(ctx: &Arc<CudaContext>, cap: u32, head: u32, order: &[u32]) -> DevBuf {
        let mut host = vec![0xFFFF_FFF0u32; cap as usize];
        for (b, s) in order.iter().enumerate() {
            host[(head.wrapping_add(b as u32) % cap) as usize] = *s;
        }
        DevBuf::from_slice(ctx, &host)
    }
    fn assert_close(got: &[f32], want: &[f64], what: &str) {
        assert_eq!(got.len(), want.len(), "{what}: length");
        for (i, (g, w)) in got.iter().zip(want).enumerate() {
            let tol = 1e-4 * (1.0 + w.abs());
            assert!((f64::from(* g) - w).abs() <= tol, "{what}[{i}]: got {g}, want {w}");
        }
    }
    struct DenseSpec<'a> {
        ranges: &'a [(u32, u32)],
        reverse_table: bool,
        ring_cap: u32,
        head: u32,
        order: &'a [u32],
        query_len: u32,
    }
    fn run_dense<const DIM: usize>(
        rig: &mut Rig<DIM>,
        rng: &mut Lcg,
        spec: DenseSpec<'_>,
        wait: Wait,
    ) {
        let dtype = rig.dtype;
        let slot_count: u32 = spec.ranges.iter().map(|r| r.1).sum();
        let docs: Vec<Vec<f32>> = (0..slot_count)
            .map(|_| random_doc(rng, dtype, DIM))
            .collect();
        let mut segs: Vec<Seg> = spec
            .ranges
            .iter()
            .map(|&(first, count)| Seg {
                first,
                count,
                pad_len: 0,
                lens: None,
            })
            .collect();
        if spec.reverse_table {
            segs.reverse();
        }
        let table = build_table(&rig.ctx, dtype, &docs, &segs);
        let ring = build_ring(&rig.ctx, spec.ring_cap, spec.head, spec.order);
        let q = spec.query_len as usize;
        let queries: Vec<f32> = (0..q * DIM).map(|_| rng.next()).collect();
        let query_buf = DevBuf::from_slice(&rig.ctx, &queries);
        let total = spec.order.len();
        let out = DevBuf::from_slice(&rig.ctx, &vec![SENTINEL; q * total + 1]);
        let p = rig
            .params(
                &ring,
                Some(&table),
                &query_buf,
                &out,
                (spec.ring_cap, spec.head, total as u32, spec.query_len),
            );
        assert_eq!(p.slot_total() as usize, total);
        let ev = rig.launch(false, &p).unwrap();
        settle(rig, &p, ev, wait);
        let got = out.read::<f32>(q * total + 1);
        assert_eq!(got[q * total], SENTINEL, "write past the output");
        let mut want = vec![0f64; q * total];
        for (b, slot) in spec.order.iter().enumerate() {
            for qi in 0..q {
                want[qi * total + b] = (0..DIM)
                    .map(|i| {
                        f64::from(queries[qi * DIM + i])
                            * f64::from(docs[*slot as usize][i])
                    })
                    .sum();
            }
        }
        assert_close(&got[..q * total], &want, "dense_dot");
    }
    struct FusedSpec {
        segs: Vec<Seg>,
        ring_cap: u32,
        head: u32,
        order: Vec<u32>,
        query_len: u32,
    }
    fn run_fused<const DIM: usize>(
        rig: &mut Rig<DIM>,
        rng: &mut Lcg,
        spec: FusedSpec,
        wait: Wait,
    ) {
        let dtype = rig.dtype;
        let slot_count = spec.segs.iter().map(|s| s.first + s.count).max().unwrap();
        let mut docs = vec![Vec::new(); slot_count as usize];
        let mut effective = vec![0usize; slot_count as usize];
        for s in &spec.segs {
            for k in 0..s.count {
                let len = match &s.lens {
                    Some(l) => l[k as usize].min(s.pad_len),
                    None => s.pad_len,
                } as usize;
                let mut rows = random_doc(rng, dtype, len * DIM);
                rows.resize(s.pad_len as usize * DIM, quantize(dtype, POISON));
                docs[(s.first + k) as usize] = rows;
                effective[(s.first + k) as usize] = len;
            }
        }
        let table = build_table(&rig.ctx, dtype, &docs, &spec.segs);
        let ring = build_ring(&rig.ctx, spec.ring_cap, spec.head, &spec.order);
        let q = spec.query_len as usize;
        let queries: Vec<f32> = (0..q * DIM).map(|_| rng.next()).collect();
        let query_buf = DevBuf::from_slice(&rig.ctx, &queries);
        let total = spec.order.len();
        let out = DevBuf::from_slice(&rig.ctx, &vec![SENTINEL; total + 1]);
        let p = rig
            .params(
                &ring,
                Some(&table),
                &query_buf,
                &out,
                (spec.ring_cap, spec.head, total as u32, spec.query_len),
            );
        let ev = rig.launch(true, &p).unwrap();
        settle(rig, &p, ev, wait);
        let got = out.read::<f32>(total + 1);
        assert_eq!(got[total], SENTINEL, "write past the output");
        let want = fused_reference(&queries, &docs, &effective, &spec.order, q, DIM);
        assert_close(&got[..total], &want, "fused_maxsim");
    }
    fn run_empty<const DIM: usize>(rig: &mut Rig<DIM>, fused: bool) {
        let ring = DevBuf::zeroed(&rig.ctx, 16);
        let query = DevBuf::from_slice(&rig.ctx, &[1.0f32; DIM]);
        let out = DevBuf::from_slice(&rig.ctx, &[SENTINEL; 4]);
        let p = rig.params(&ring, None, &query, &out, (4, 9, 0, 1));
        let ev = rig.launch(fused, &p).unwrap();
        drop(ev);
        rig.wait_by_spinning();
        assert_eq!(rig.block_ctr.read::< u32 > (1) [0], 0);
        assert_eq!(out.read::< f32 > (4), vec![SENTINEL; 4]);
    }
    fn fused_reference(
        queries: &[f32],
        docs: &[Vec<f32>],
        effective: &[usize],
        order: &[u32],
        query_len: usize,
        dim: usize,
    ) -> Vec<f64> {
        order
            .iter()
            .map(|slot| {
                let doc = &docs[*slot as usize];
                let len = effective[*slot as usize];
                (0..query_len)
                    .map(|t| {
                        if len == 0 {
                            return f64::from(NO_VALID_TOKEN_SCORE);
                        }
                        (0..len)
                            .map(|d| {
                                (0..dim)
                                    .map(|i| {
                                        f64::from(queries[t * dim + i])
                                            * f64::from(doc[d * dim + i])
                                    })
                                    .sum::<f64>()
                            })
                            .fold(f64::NEG_INFINITY, f64::max)
                    })
                    .sum()
            })
            .collect()
    }
    /// Repeats many-block batches and reads the output as soon as the doorbell moves; the
    /// output is reset to a sentinel before each launch, so a result that is not yet visible
    /// when `completed` advances shows up as a sentinel value.
    fn stress<const DIM: usize>(dtype: DType, iterations: usize) {
        let mut rig = Rig::<DIM>::new(dtype);
        let mut rng = Lcg(0xfeed_beef_0123_4567);
        let n = 1024u32;
        let docs: Vec<Vec<f32>> = (0..n)
            .map(|_| random_doc(&mut rng, dtype, DIM))
            .collect();
        let segs = [
            Seg {
                first: 0,
                count: n,
                pad_len: 0,
                lens: None,
            },
        ];
        let table = build_table(&rig.ctx, dtype, &docs, &segs);
        let order: Vec<u32> = (0..n).rev().collect();
        let ring = build_ring(&rig.ctx, n, 5, &order);
        let queries: Vec<f32> = (0..DIM).map(|_| rng.next()).collect();
        let query_buf = DevBuf::from_slice(&rig.ctx, &queries);
        let out = DevBuf::from_slice(&rig.ctx, &vec![SENTINEL; n as usize]);
        let p = rig.params(&ring, Some(&table), &query_buf, &out, (n, 5, n, 1));
        let want: Vec<f64> = order
            .iter()
            .map(|s| {
                (0..DIM)
                    .map(|i| f64::from(queries[i]) * f64::from(docs[*s as usize][i]))
                    .sum()
            })
            .collect();
        for it in 0..iterations {
            out.fill(&vec![SENTINEL; n as usize]);
            drop(rig.launch(false, &p).unwrap());
            rig.wait_by_spinning();
            let got = out.read::<f32>(n as usize);
            assert_close(&got, &want, &format!("dense iteration {it}"));
        }
        let m = 512u32;
        let pad = 4u32;
        let lens: Vec<u32> = (0..m).map(|i| i % 5).collect();
        let effective: Vec<usize> = lens.iter().map(|l| *l as usize).collect();
        let mut fdocs = Vec::new();
        for l in &lens {
            let mut rows = random_doc(&mut rng, dtype, *l as usize * DIM);
            rows.resize(pad as usize * DIM, quantize(dtype, POISON));
            fdocs.push(rows);
        }
        let fsegs = [
            Seg {
                first: 0,
                count: m,
                pad_len: pad,
                lens: Some(lens),
            },
        ];
        let ftable = build_table(&rig.ctx, dtype, &fdocs, &fsegs);
        let forder: Vec<u32> = (0..m).collect();
        let fring = build_ring(&rig.ctx, m, 0, &forder);
        let q = 3usize;
        let fqueries: Vec<f32> = (0..q * DIM).map(|_| rng.next()).collect();
        let fquery_buf = DevBuf::from_slice(&rig.ctx, &fqueries);
        let fout = DevBuf::from_slice(&rig.ctx, &vec![SENTINEL; m as usize]);
        let fp = rig
            .params(&fring, Some(&ftable), &fquery_buf, &fout, (m, 0, m, q as u32));
        let fwant = fused_reference(&fqueries, &fdocs, &effective, &forder, q, DIM);
        for it in 0..iterations {
            fout.fill(&vec![SENTINEL; m as usize]);
            drop(rig.launch(true, &fp).unwrap());
            rig.wait_by_spinning();
            let got = fout.read::<f32>(m as usize);
            assert_close(&got, &fwant, &format!("fused iteration {it}"));
        }
        assert_eq!(rig.kernels.launched(), 2 * iterations as u64);
        assert_eq!(rig.doorbell.acked(), 2 * iterations as u64);
    }
    fn scenario<const DIM: usize>(dtype: DType) {
        let mut rig = Rig::<DIM>::new(dtype);
        let mut rng = Lcg(0x1234_5678_9abc_def0);
        run_dense(
            &mut rig,
            &mut rng,
            DenseSpec {
                ranges: &[(0, 4), (4, 3)],
                reverse_table: true,
                ring_cap: 16,
                head: u32::MAX - 2,
                order: &[5, 0, 6, 2, 4, 1, 3],
                query_len: 3,
            },
            Wait::EventAfterAckProbe,
        );
        run_fused(
            &mut rig,
            &mut rng,
            FusedSpec {
                segs: vec![
                    Seg { first : 0, count : 4, pad_len : 5, lens : Some(vec![5, 3, 0,
                    1]), }, Seg { first : 4, count : 3, pad_len : 3, lens : None, },
                ],
                ring_cap: 8,
                head: 6,
                order: vec![6, 1, 2, 0, 5, 4, 3],
                query_len: 4,
            },
            Wait::Event,
        );
        let big: Vec<u32> = (0..300u32).map(|i| i * 7 % 300).collect();
        run_dense(
            &mut rig,
            &mut rng,
            DenseSpec {
                ranges: &[(0, 300)],
                reverse_table: false,
                ring_cap: 512,
                head: 400,
                order: &big,
                query_len: 2,
            },
            Wait::Spin,
        );
        run_fused(
            &mut rig,
            &mut rng,
            FusedSpec {
                segs: vec![
                    Seg { first : 0, count : 257, pad_len : 4, lens : Some((0..257u32)
                    .map(| i | i % 7).collect()), }
                ],
                ring_cap: 300,
                head: 0,
                order: (0..257u32).rev().collect(),
                query_len: 3,
            },
            Wait::Spin,
        );
        run_empty(&mut rig, false);
        run_empty(&mut rig, true);
        assert_eq!(rig.kernels.launched(), 6);
        assert_eq!(rig.doorbell.acked(), 6);
        assert!(rig.doorbell.poll().unwrap().is_none());
    }
    #[test]
    fn real_launch_f32() {
        scenario::<200>(DType::F32);
    }
    #[test]
    fn real_launch_f16() {
        scenario::<200>(DType::F16);
    }
    #[test]
    fn real_launch_odd_dim_f16() {
        scenario::<37>(DType::F16);
    }
    #[test]
    fn real_launch_model_dim_f32() {
        scenario::<384>(DType::F32);
    }
    #[test]
    fn real_launch_model_dim_f16() {
        scenario::<384>(DType::F16);
    }
    #[test]
    fn doorbell_stress_f32() {
        stress::<384>(DType::F32, 300);
    }
    #[test]
    fn doorbell_stress_f16() {
        stress::<384>(DType::F16, 300);
    }
}
