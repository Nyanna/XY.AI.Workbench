//! These tests run on the real card. Without a CUDA device they fail instead of skipping.
use super::super::driver_loader::DriverLoader;
use super::super::nvcc_compiler::NvccCompiler;
use super::*;
use crate::resident::l130::a::cuda_backend::CudaBackend;
use crate::resident::l130::a::gpu_subengine::GpuBackend;
use cudarc::driver::{sys, CudaContext};
use l010::device_lane_event::{
    DeviceHandle, GpuEventBackend, LaneConfig, LaneKind, Priority, StreamBackend,
};
use l070::a::kernel_register::{CodeKind, CompileRequest, KernelCompiler, ModuleLoader};
use std::any::Any;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Instant;
const WRITE_VALUES: &str = r#"
struct Params { unsigned long long out; unsigned int value; unsigned int count; unsigned long long spin; };
extern "C" __global__ void write_values(Params p) {
    if (blockIdx.x == 0 && threadIdx.x == 0 && p.spin > 0) {
        long long start = clock64();
        while (clock64() - start < (long long)p.spin) {}
    }
    unsigned int i = blockIdx.x * blockDim.x + threadIdx.x;
    if (i < p.count) ((unsigned int*)p.out)[i] = p.value + i;
}
"#;
const COUNT: u32 = 1000;
const SPIN_CYCLES: u64 = 300_000_000;
static DIRS: AtomicU64 = AtomicU64::new(0);
struct Buffer {
    ctx: Arc<CudaContext>,
    ptr: sys::CUdeviceptr,
}
impl Buffer {
    fn new(ctx: &Arc<CudaContext>, words: usize) -> Buffer {
        ctx.bind_to_thread().unwrap();
        let bytes = words * 4;
        let ptr = unsafe { drv::malloc_sync(bytes) }.unwrap();
        unsafe { drv::memset_d8_sync(ptr, 0, bytes) }.unwrap();
        Buffer {
            ctx: Arc::clone(ctx),
            ptr,
        }
    }
    fn read(&self, words: usize) -> Vec<u32> {
        self.ctx.bind_to_thread().unwrap();
        let mut out = vec![0u32; words];
        unsafe { drv::memcpy_dtoh_sync(&mut out, self.ptr) }.unwrap();
        out
    }
}
impl Drop for Buffer {
    fn drop(&mut self) {
        if self.ctx.bind_to_thread().is_ok() {
            let _ = unsafe { drv::free_sync(self.ptr) };
        }
    }
}
struct Rig {
    _backend: CudaBackend,
    ctx: Arc<CudaContext>,
    lane: Lane,
    function: Arc<dyn KernelFunction>,
    buffer: Buffer,
}
fn rig() -> Rig {
    let backend = CudaBackend::new();
    assert!(! backend.devices().expect("CUDA driver is available").is_empty());
    backend.retain_primary_context(0).unwrap();
    let stream = backend.create_stream(0, Priority::Normal).unwrap();
    let lane = Lane::new(
        LaneConfig::preset(LaneKind::Query, DeviceHandle::Gpu(0)),
        Some(stream),
    );
    let loader = DriverLoader::new(0).unwrap();
    let dir = std::env::temp_dir()
        .join(
            format!(
                "xy_driver_launcher_{}_{}", std::process::id(), DIRS.fetch_add(1,
                Ordering::SeqCst)
            ),
        );
    let compiler = NvccCompiler::discover(&dir)
        .unwrap_or_else(|| NvccCompiler::new("/usr/local/cuda-13.3/bin/nvcc", &dir));
    let image = compiler
        .compile(
            &CompileRequest {
                file_name: "write_values.cu".to_string(),
                source: WRITE_VALUES.to_string(),
                arch: loader.device_capability().unwrap().jit_arch(),
                kind: CodeKind::Ptx,
            },
        )
        .unwrap();
    let _ = std::fs::remove_dir_all(&dir);
    let function = loader.load(&image, "write_values").unwrap();
    let ctx = Arc::clone(loader.context());
    let buffer = Buffer::new(&ctx, COUNT as usize);
    Rig {
        _backend: backend,
        ctx,
        lane,
        function,
        buffer,
    }
}
fn params(out: u64, value: u32, spin: u64) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(24);
    bytes.extend_from_slice(&out.to_ne_bytes());
    bytes.extend_from_slice(&value.to_ne_bytes());
    bytes.extend_from_slice(&COUNT.to_ne_bytes());
    bytes.extend_from_slice(&spin.to_ne_bytes());
    bytes
}
fn config() -> LaunchConfig {
    LaunchConfig {
        grid: COUNT.div_ceil(128),
        block: 128,
        dynamic_shared: 0,
    }
}
fn run(rig: &Rig, value: u32, spin: u64) -> Result<(), String> {
    DriverLauncher
        .launch(
            &rig.function,
            &rig.lane,
            &config(),
            &params(rig.buffer.ptr as u64, value, spin),
        )
}
fn expect(rig: &Rig, base: u32) {
    let got = rig.buffer.read(COUNT as usize);
    let want: Vec<u32> = (0..COUNT).map(|i| base + i).collect();
    assert_eq!(got, want);
}
#[derive(Debug)]
struct ForeignFunction;
impl KernelFunction for ForeignFunction {
    fn symbol(&self) -> &str {
        "foreign"
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}
#[derive(Debug)]
struct ForeignStream;
impl StreamBackend for ForeignStream {
    fn record_event(&self) -> Result<Box<dyn GpuEventBackend>, String> {
        Err("not a real stream".to_string())
    }
}
fn assert_rejected(rig: &Rig, result: Result<(), String>) {
    assert!(result.is_err());
    assert!(rig.buffer.read(COUNT as usize).iter().all(| w | * w == 0));
}
#[test]
fn values_are_written_on_the_lane_stream() {
    let rig = rig();
    run(&rig, 7, 0).unwrap();
    rig.lane.record_event().unwrap().wait().unwrap();
    expect(&rig, 7);
}
#[test]
fn launch_does_not_wait_for_the_kernel() {
    let rig = rig();
    run(&rig, 1, 0).unwrap();
    rig.lane.record_event().unwrap().wait().unwrap();
    let start = Instant::now();
    run(&rig, 11, SPIN_CYCLES).unwrap();
    let launch_time = start.elapsed();
    rig.lane.record_event().unwrap().wait().unwrap();
    let total_time = start.elapsed();
    assert!(
        launch_time * 4 < total_time, "launch {launch_time:?}, total {total_time:?}"
    );
    expect(&rig, 11);
}
#[test]
fn launches_run_in_order_within_the_lane() {
    let rig = rig();
    run(&rig, 100, SPIN_CYCLES).unwrap();
    run(&rig, 500, 0).unwrap();
    rig.lane.record_event().unwrap().wait().unwrap();
    expect(&rig, 500);
}
#[test]
fn context_stays_usable_after_a_launch() {
    let rig = rig();
    run(&rig, 3, 0).unwrap();
    rig.lane.record_event().unwrap().wait().unwrap();
    rig.ctx.bind_to_thread().unwrap();
    run(&rig, 4, 0).unwrap();
    rig.lane.record_event().unwrap().wait().unwrap();
    expect(&rig, 4);
}
#[test]
fn foreign_functions_are_rejected() {
    let rig = rig();
    let foreign: Arc<dyn KernelFunction> = Arc::new(ForeignFunction);
    let result = DriverLauncher
        .launch(&foreign, &rig.lane, &config(), &params(rig.buffer.ptr as u64, 1, 0));
    assert_rejected(&rig, result);
}
#[test]
fn lane_without_a_stream_is_rejected() {
    let rig = rig();
    let lane = Lane::new(
        LaneConfig::preset(LaneKind::Query, DeviceHandle::Gpu(0)),
        None,
    );
    let result = DriverLauncher
        .launch(&rig.function, &lane, &config(), &params(rig.buffer.ptr as u64, 1, 0));
    assert_rejected(&rig, result);
}
#[test]
fn streams_of_other_bindings_are_rejected() {
    let rig = rig();
    let lane = Lane::new(
        LaneConfig::preset(LaneKind::Query, DeviceHandle::Gpu(0)),
        Some(Box::new(ForeignStream)),
    );
    let result = DriverLauncher
        .launch(&rig.function, &lane, &config(), &params(rig.buffer.ptr as u64, 1, 0));
    assert_rejected(&rig, result);
}
#[test]
fn invalid_configuration_is_rejected() {
    let rig = rig();
    let cfg = LaunchConfig {
        grid: 0,
        block: 128,
        dynamic_shared: 0,
    };
    let result = DriverLauncher
        .launch(&rig.function, &rig.lane, &cfg, &params(rig.buffer.ptr as u64, 1, 0));
    assert_rejected(&rig, result);
}
