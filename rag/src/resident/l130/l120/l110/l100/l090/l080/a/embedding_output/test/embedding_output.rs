use super::segment_writer::SegmentWriter;
use super::*;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::a::kernel_register::{
    CodeImage, CompileRequest, CompilerError, KernelCompiler, KernelFunction,
    LoadFailure, ModuleLoader,
};
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::a::library_adapter::library_kernel_source;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::a::slot_layout::{
    EntryRef, SegmentView, SlotClass,
};
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::a::model_descriptor::Pooling;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::a::region_handle::{
    Generation, RegionHandle,
};
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::a::arenas::ArenaKind;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::a::index_contract::EntryId;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::a::memory_access::RegionBackend;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::l010::{
    device_lane_event::{
        DeviceHandle, GpuEventBackend, LaneConfig, LaneKind, StreamBackend,
    },
    target_env::ComputeCapability,
};
use static_assertions::assert_impl_all;
use std::any::Any;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
assert_impl_all!(OutputError : Send, Sync);
assert_impl_all!(WriteReport : Send, Sync);
assert_impl_all!(GpuWriteTicket : Send, Sync);
fn assert_static<T: 'static>() {}
fn f16_to_f32(h: u16) -> f32 {
    let sign = if h & 0x8000 != 0 { -1.0f32 } else { 1.0 };
    let exp = ((h >> 10) & 0x1F) as i32;
    let man = (h & 0x3FF) as f32;
    match exp {
        0 => sign * man * 2f32.powi(-24),
        31 if man == 0.0 => sign * f32::INFINITY,
        31 => f32::NAN,
        _ => sign * (1.0 + man / 1024.0) * 2f32.powi(exp - 15),
    }
}
fn bf16_to_f32(h: u16) -> f32 {
    f32::from_bits((h as u32) << 16)
}
fn f16_table() -> Vec<f64> {
    (0..=0x7BFFu16).map(|h| f16_to_f32(h) as f64).collect()
}
struct Lcg(u64);
impl Lcg {
    fn next(&mut self) -> u32 {
        self.0 = self
            .0
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        (self.0 >> 32) as u32
    }
    fn unit(&mut self) -> f32 {
        (self.next() as f32 / u32::MAX as f32) * 2.0 - 1.0
    }
}
#[test]
fn supported_output_dtypes() {
    for d in [DType::F32, DType::F16, DType::BF16] {
        assert!(is_supported_output(d));
    }
    for d in [DType::U8, DType::U32, DType::I64] {
        assert!(! is_supported_output(d));
    }
}
#[test]
fn error_types_are_send_sync_static() {
    assert_static::<OutputError>();
    assert_static::<WriteReport>();
    assert_static::<GpuWriteTicket>();
}
#[test]
fn encode_is_little_endian_and_sized() {
    let v = [1.0f32, -2.5, 0.0];
    let f32b = encode_f32_slice(&v, DType::F32).unwrap();
    assert_eq!(f32b.len(), 12);
    assert_eq!(& f32b[0..4], & 1.0f32.to_le_bytes());
    assert_eq!(& f32b[4..8], & (- 2.5f32).to_le_bytes());
    let f16b = encode_f32_slice(&v, DType::F16).unwrap();
    assert_eq!(f16b.len(), 6);
    assert_eq!(& f16b[0..2], & [0x00, 0x3C]);
    let bf = encode_f32_slice(&v, DType::BF16).unwrap();
    assert_eq!(bf.len(), 6);
    assert_eq!(& bf[0..2], & [0x80, 0x3F]);
    assert!(encode_f32_slice(& [], DType::F16).unwrap().is_empty());
}
#[test]
fn encode_f32_keeps_bits() {
    let v = [f32::from_bits(0x7FC0_1234), f32::from_bits(1), -0.0, f32::INFINITY];
    let b = encode_f32_slice(&v, DType::F32).unwrap();
    for (i, x) in v.iter().enumerate() {
        assert_eq!(& b[i * 4..i * 4 + 4], & x.to_bits().to_le_bytes());
    }
}
#[test]
fn encode_rejects_unsupported_dtypes() {
    for d in [DType::U8, DType::U32, DType::I64] {
        assert_eq!(encode_f32_slice(& [1.0], d), Err(OutputError::Incompatible(d)));
    }
}
#[test]
fn error_display_and_conversions() {
    assert_eq!(
        OutputError::Incompatible(DType::U8).to_string(),
        "inkompatibel: Output-Datentyp U8"
    );
    let all = [
        OutputError::Memory(AccessError::Backend("x".into())),
        OutputError::PoolNorm(PoolNormError::ZeroDimension),
        OutputError::InvalidResult("bad".into()),
        OutputError::SlotTooSmall {
            required: 8,
            available: 4,
        },
        OutputError::Kernel(KernelError::UnknownKernel("k".into())),
        OutputError::Lane(LaneError::NoStreamBackend),
    ];
    for e in &all {
        assert!(! e.to_string().is_empty());
    }
    let s = all[3].to_string();
    assert!(s.contains('8') && s.contains('4'));
    assert!(std::error::Error::source(& all[0]).is_some());
    assert!(std::error::Error::source(& all[2]).is_none());
    assert_eq!(
        OutputError::from(AccessError::Backend("x".into())),
        OutputError::Memory(AccessError::Backend("x".into()))
    );
    assert_eq!(
        OutputError::from(PoolNormError::ZeroDimension),
        OutputError::PoolNorm(PoolNormError::ZeroDimension)
    );
}
#[derive(Debug)]
struct MemBackend {
    data: Mutex<Vec<u8>>,
    writes: AtomicUsize,
    fail: Option<String>,
}
impl RegionBackend for MemBackend {
    fn len(&self) -> usize {
        self.data.lock().unwrap().len()
    }
    fn read_raw(&self, offset: usize, dst: &mut [u8]) -> Result<(), AccessError> {
        dst.copy_from_slice(&self.data.lock().unwrap()[offset..offset + dst.len()]);
        Ok(())
    }
    fn write_raw(&self, offset: usize, src: &[u8]) -> Result<(), AccessError> {
        self.writes.fetch_add(1, Ordering::SeqCst);
        if let Some(t) = &self.fail {
            return Err(AccessError::Backend(t.clone()));
        }
        self.data.lock().unwrap()[offset..offset + src.len()].copy_from_slice(src);
        Ok(())
    }
}
fn mem_with(len: usize, fail: Option<&str>) -> (AccessRegion, Arc<MemBackend>) {
    let b = Arc::new(MemBackend {
        data: Mutex::new(vec![0xAA; len]),
        writes: AtomicUsize::new(0),
        fail: fail.map(str::to_string),
    });
    (AccessRegion::new(b.clone()), b)
}
fn mem(len: usize) -> (AccessRegion, Arc<MemBackend>) {
    mem_with(len, None)
}
fn pn(norm: Normalization, dim: usize) -> PoolNorm {
    PoolNorm::new(Pooling::Mean, norm, dim).unwrap()
}
fn bytes(b: &MemBackend) -> Vec<u8> {
    b.data.lock().unwrap().clone()
}
#[test]
fn host_f32_without_normalization_is_copied_bitwise() {
    let (r, b) = mem(64);
    let mut v = vec![1.0f32, 2.0, 3.0, f32::from_bits(0x7FC0_0001), 5.0, 6.0];
    let orig = v.clone();
    let rep = write_embeddings(&r, 8, &mut v, &pn(Normalization::None, 3), DType::F32)
        .unwrap();
    assert_eq!(
        rep, WriteReport { num_vectors : 2, bytes_written : 24, degenerate_count : 0, }
    );
    assert_eq!(v[..3], orig[..3]);
    assert_eq!(v[3].to_bits(), orig[3].to_bits());
    let d = bytes(&b);
    assert_eq!(& d[..8], & [0xAA; 8]);
    assert_eq!(& d[8..32], & encode_f32_slice(& orig, DType::F32).unwrap() [..]);
    assert_eq!(& d[32..], & [0xAA; 32] [..]);
    assert_eq!(b.writes.load(Ordering::SeqCst), 1);
}
#[test]
fn host_l2_normalizes_in_place_and_writes_dense_vectors() {
    for dtype in [DType::F32, DType::F16, DType::BF16] {
        let es = dtype.size_in_bytes();
        let (r, b) = mem(64);
        let mut v = vec![3.0f32, 4.0, 0.0, 1.0, 0.0, 2.0];
        let rep = write_embeddings(&r, 4, &mut v, &pn(Normalization::L2, 2), dtype)
            .unwrap();
        assert_eq!(rep.num_vectors, 3);
        assert_eq!(rep.bytes_written, 6 * es);
        assert_eq!(rep.degenerate_count, 0);
        assert!((v[0] - 0.6).abs() < 1e-6 && (v[1] - 0.8).abs() < 1e-6);
        assert_eq!(& v[2..], & [0.0, 1.0, 0.0, 1.0]);
        let d = bytes(&b);
        assert_eq!(& d[..4], & [0xAA; 4]);
        assert_eq!(& d[4..4 + 6 * es], & encode_f32_slice(& v, dtype).unwrap() [..]);
        assert!(d[4 + 6 * es..].iter().all(| x | * x == 0xAA));
    }
}
#[test]
fn host_vector_i_sits_at_offset_plus_i_dim_elem() {
    let (r, b) = mem(128);
    let mut v: Vec<f32> = (0..15).map(|i| i as f32 + 1.0).collect();
    let src = v.clone();
    write_embeddings(&r, 10, &mut v, &pn(Normalization::None, 5), DType::F16).unwrap();
    let d = bytes(&b);
    for i in 0..3 {
        let want = encode_f32_slice(&src[i * 5..i * 5 + 5], DType::F16).unwrap();
        let at = 10 + i * 5 * 2;
        assert_eq!(& d[at..at + 10], & want[..]);
    }
}
#[test]
fn host_degenerate_vectors_are_counted_but_succeed() {
    let (r, b) = mem(64);
    let mut v = vec![0.0f32, 0.0, 3.0, 4.0, 1e-20, 0.0, f32::NAN, 1.0];
    let rep = write_embeddings(&r, 0, &mut v, &pn(Normalization::L2, 2), DType::F32)
        .unwrap();
    assert_eq!(rep.degenerate_count, 2);
    assert_eq!(rep.num_vectors, 4);
    let d = bytes(&b);
    assert_eq!(& d[..8], & [0u8; 8]);
    let out: Vec<f32> = d[..32]
        .chunks(4)
        .map(|c| f32::from_le_bytes(c.try_into().unwrap()))
        .collect();
    assert!(out[6].is_nan());
    assert_eq!(out[7], 1.0);
}
#[test]
fn host_degenerate_not_counted_without_normalization() {
    let (r, _b) = mem(16);
    let mut v = vec![0.0f32; 4];
    let rep = write_embeddings(&r, 0, &mut v, &pn(Normalization::None, 2), DType::F32)
        .unwrap();
    assert_eq!(rep.degenerate_count, 0);
}
#[test]
fn host_empty_input_succeeds_without_write() {
    let (r, b) = mem(8);
    let rep = write_embeddings(&r, 1000, &mut [], &pn(Normalization::L2, 4), DType::F16)
        .unwrap();
    assert_eq!(
        rep, WriteReport { num_vectors : 0, bytes_written : 0, degenerate_count : 0, }
    );
    assert_eq!(b.writes.load(Ordering::SeqCst), 0);
    assert_eq!(bytes(& b), vec![0xAA; 8]);
}
#[test]
fn host_shape_mismatch_is_a_pool_norm_error() {
    let (r, b) = mem(64);
    let mut v = vec![1.0f32; 5];
    let e = write_embeddings(&r, 0, &mut v, &pn(Normalization::L2, 2), DType::F32)
        .unwrap_err();
    assert!(matches!(e, OutputError::PoolNorm(PoolNormError::ShapeMismatch(_))));
    assert_eq!(b.writes.load(Ordering::SeqCst), 0);
    assert_eq!(v, vec![1.0f32; 5]);
    let e = write_embeddings(&r, 0, &mut v, &pn(Normalization::None, 2), DType::F32)
        .unwrap_err();
    assert!(matches!(e, OutputError::PoolNorm(PoolNormError::ShapeMismatch(_))));
}
#[test]
fn host_zero_dim_is_a_pool_norm_error() {
    let (r, b) = mem(16);
    let norm = PoolNorm {
        pooling: Pooling::Mean,
        normalization: Normalization::None,
        dim: 0,
    };
    let e = write_embeddings(&r, 0, &mut [1.0], &norm, DType::F32).unwrap_err();
    assert_eq!(e, OutputError::PoolNorm(PoolNormError::ZeroDimension));
    assert_eq!(b.writes.load(Ordering::SeqCst), 0);
}
#[test]
fn host_incompatible_dtype_touches_nothing() {
    for d in [DType::U8, DType::U32, DType::I64] {
        let (r, b) = mem(64);
        let mut v = vec![3.0f32, 4.0, 5.0];
        let e = write_embeddings(&r, 0, &mut v, &pn(Normalization::L2, 3), d)
            .unwrap_err();
        assert_eq!(e, OutputError::Incompatible(d));
        assert_eq!(v, vec![3.0, 4.0, 5.0]);
        assert_eq!(b.writes.load(Ordering::SeqCst), 0);
        let mut bad_shape = vec![1.0f32; 4];
        let e = write_embeddings(&r, 0, &mut bad_shape, &pn(Normalization::L2, 3), d)
            .unwrap_err();
        assert_eq!(e, OutputError::Incompatible(d));
    }
}
#[test]
fn host_out_of_bounds_is_passed_through() {
    let (r, b) = mem(7);
    let mut v = vec![1.0f32, 2.0];
    let e = write_embeddings(&r, 0, &mut v, &pn(Normalization::None, 2), DType::F32)
        .unwrap_err();
    assert_eq!(
        e, OutputError::Memory(AccessError::OutOfBounds { offset : 0, len : 8, region_len
        : 7, })
    );
    assert_eq!(bytes(& b), vec![0xAA; 7]);
    let e = write_embeddings(
            &r,
            usize::MAX,
            &mut v,
            &pn(Normalization::None, 2),
            DType::F16,
        )
        .unwrap_err();
    assert!(matches!(e, OutputError::Memory(AccessError::OutOfBounds { .. })));
}
#[test]
fn host_backend_error_is_passed_through() {
    let (r, b) = mem_with(64, Some("disk gone"));
    let mut v = vec![1.0f32, 2.0];
    let e = write_embeddings(&r, 0, &mut v, &pn(Normalization::None, 2), DType::F32)
        .unwrap_err();
    assert_eq!(e, OutputError::Memory(AccessError::Backend("disk gone".into())));
    assert_eq!(b.writes.load(Ordering::SeqCst), 1);
}
#[test]
fn host_parallel_writes_to_disjoint_ranges() {
    let (r, b) = mem(8 * 16);
    let handles: Vec<_> = (0..8usize)
        .map(|t| {
            let r = r.clone();
            std::thread::spawn(move || {
                let mut v = vec![t as f32 + 1.0; 4];
                write_embeddings(
                        &r,
                        t * 16,
                        &mut v,
                        &pn(Normalization::None, 4),
                        DType::F32,
                    )
                    .unwrap();
            })
        })
        .collect();
    for h in handles {
        h.join().unwrap();
    }
    let d = bytes(&b);
    for t in 0..8usize {
        let want = encode_f32_slice(&[t as f32 + 1.0; 4], DType::F32).unwrap();
        assert_eq!(& d[t * 16..t * 16 + 16], & want[..]);
    }
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
    requests: Mutex<Vec<CompileRequest>>,
    fail: Option<CompilerError>,
}
impl KernelCompiler for MockCompiler {
    fn compile(&self, req: &CompileRequest) -> Result<CodeImage, CompilerError> {
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
struct MockLoader;
impl ModuleLoader for MockLoader {
    fn device_capability(&self) -> Option<ComputeCapability> {
        Some(ComputeCapability::new(7, 5))
    }
    fn load(
        &self,
        _: &CodeImage,
        symbol: &str,
    ) -> Result<Arc<dyn KernelFunction>, LoadFailure> {
        Ok(Arc::new(Func(symbol.to_string())))
    }
}
type Log = Arc<Mutex<Vec<&'static str>>>;
#[derive(Debug, Clone)]
struct Call {
    symbol: String,
    key_dtype: DType,
    key_variant: Variant,
    args: WriteKernelArgs,
    dims: Vec<usize>,
}
struct MockLauncher {
    calls: Mutex<Vec<Call>>,
    fail: Option<String>,
    log: Log,
}
impl WriteKernelLauncher for MockLauncher {
    fn launch(
        &self,
        _lane: &Lane,
        kernel: &GpuKernel,
        args: &WriteKernelArgs,
        result: &LibTensor,
    ) -> Result<(), String> {
        self.log.lock().unwrap().push("launch");
        self.calls
            .lock()
            .unwrap()
            .push(Call {
                symbol: kernel.function.symbol().to_string(),
                key_dtype: kernel.key.dtype,
                key_variant: kernel.key.variant,
                args: *args,
                dims: result.dims().to_vec(),
            });
        match &self.fail {
            Some(t) => Err(t.clone()),
            None => Ok(()),
        }
    }
}
#[derive(Debug)]
struct FakeEvent {
    ready: Arc<AtomicBool>,
    syncs: Arc<AtomicUsize>,
}
impl GpuEventBackend for FakeEvent {
    fn query(&self) -> Result<bool, String> {
        Ok(self.ready.load(Ordering::SeqCst))
    }
    fn synchronize(&self) -> Result<(), String> {
        self.syncs.fetch_add(1, Ordering::SeqCst);
        self.ready.store(true, Ordering::SeqCst);
        Ok(())
    }
}
#[derive(Debug)]
struct FakeStream {
    ready: Arc<AtomicBool>,
    syncs: Arc<AtomicUsize>,
    fail_record: bool,
    log: Log,
}
impl StreamBackend for FakeStream {
    fn record_event(&self) -> Result<Box<dyn GpuEventBackend>, String> {
        self.log.lock().unwrap().push("record");
        if self.fail_record {
            return Err("record failed".to_string());
        }
        Ok(
            Box::new(FakeEvent {
                ready: self.ready.clone(),
                syncs: self.syncs.clone(),
            }),
        )
    }
}
struct Env {
    register: KernelRegister,
    compiler: Arc<MockCompiler>,
    launcher: MockLauncher,
    log: Log,
    ready: Arc<AtomicBool>,
    syncs: Arc<AtomicUsize>,
}
impl Env {
    fn lane_with(&self, fail_record: bool) -> Lane {
        Lane::new(
            LaneConfig::preset(LaneKind::Query, DeviceHandle::Gpu(0)),
            Some(
                Box::new(FakeStream {
                    ready: self.ready.clone(),
                    syncs: self.syncs.clone(),
                    fail_record,
                    log: self.log.clone(),
                }),
            ),
        )
    }
    fn lane(&self) -> Lane {
        self.lane_with(false)
    }
    fn calls(&self) -> Vec<Call> {
        self.launcher.calls.lock().unwrap().clone()
    }
}
fn env_with(
    device: DeviceHandle,
    register_kernel: bool,
    compile_fail: Option<CompilerError>,
    launch_fail: Option<&str>,
) -> Env {
    let compiler = Arc::new(MockCompiler {
        requests: Mutex::new(Vec::new()),
        fail: compile_fail,
    });
    let register = KernelRegister::new(
        device,
        compiler.clone(),
        Arc::new(MockLoader),
        library_kernel_source(),
    );
    if register_kernel {
        register_write_kernel(&register).unwrap();
    }
    let log: Log = Arc::new(Mutex::new(Vec::new()));
    Env {
        register,
        compiler,
        launcher: MockLauncher {
            calls: Mutex::new(Vec::new()),
            fail: launch_fail.map(str::to_string),
            log: log.clone(),
        },
        log,
        ready: Arc::new(AtomicBool::new(false)),
        syncs: Arc::new(AtomicUsize::new(0)),
    }
}
fn env() -> Env {
    env_with(DeviceHandle::Gpu(0), true, None, None)
}
fn tensor_of(dtype: candle_core::DType, shape: &[usize]) -> LibTensor {
    LibTensor::from_lib(
        candle_core::Tensor::zeros(shape, dtype, &candle_core::Device::Cpu).unwrap(),
    )
}
fn result(n: usize, dim: usize) -> LibTensor {
    tensor_of(candle_core::DType::F32, &[n, dim])
}
fn writer() -> SegmentWriter {
    let region = RegionHandle::new(
            ArenaKind::IndexSegments,
            4096,
            1 << 22,
            Generation(1),
        )
        .unwrap();
    let view = SegmentView::new(region, 4, SlotClass::C32, 4, DType::F32, Generation(2))
        .unwrap();
    SegmentWriter::new(view, Addr(1 << 20)).unwrap()
}
fn reservation(bytes: u64) -> Reservation {
    let mut res = writer()
        .reserve(
            EntryRef {
                entry: EntryId(1),
                section: 0,
            },
            1,
            None,
        )
        .unwrap();
    res.target = 100..100 + bytes;
    res
}
fn gpu_err(r: Result<GpuWriteTicket, OutputError>) -> OutputError {
    match r {
        Ok(_) => panic!("expected an error"),
        Err(e) => e,
    }
}
fn assert_untouched(env: &Env) {
    assert!(env.calls().is_empty());
    assert!(! env.log.lock().unwrap().contains(& "record"));
}
#[test]
fn gpu_success_launches_once_then_records_the_event() {
    let e = env();
    let res = reservation(1024);
    let r = result(3, 4);
    let t = write_embeddings_gpu(
            &e.register,
            &e.launcher,
            &e.lane(),
            &r,
            &pn(Normalization::L2, 4),
            DType::F16,
            &res,
        )
        .unwrap();
    let calls = e.calls();
    assert_eq!(calls.len(), 1);
    let c = &calls[0];
    assert_eq!(c.symbol, "embedding_write");
    assert_eq!(c.key_dtype, DType::F32);
    assert_eq!(c.key_variant, Variant { dim : Some(4), tile : None });
    assert_eq!(
        c.args, WriteKernelArgs { num_vectors : 3, dim : 4, normalize : true, out_dtype :
        DType::F16, dst : res.addr, }
    );
    assert_eq!(c.dims, vec![3, 4]);
    assert_eq!(* e.log.lock().unwrap(), vec!["launch", "record"]);
    assert_eq!(t.is_complete(), Ok(false));
    e.ready.store(true, Ordering::SeqCst);
    assert_eq!(t.is_complete(), Ok(true));
    assert!(t.event().is_complete().unwrap());
    t.wait().unwrap();
}
#[test]
fn gpu_normalize_flag_follows_the_normalization() {
    let e = env();
    let res = reservation(1024);
    let r = result(1, 4);
    e.ready.store(true, Ordering::SeqCst);
    for (norm, want) in [(Normalization::None, false), (Normalization::L2, true)] {
        write_embeddings_gpu(
                &e.register,
                &e.launcher,
                &e.lane(),
                &r,
                &pn(norm, 4),
                DType::F32,
                &res,
            )
            .unwrap();
        assert_eq!(e.calls().last().unwrap().args.normalize, want);
    }
}
#[test]
fn gpu_dtype_codes() {
    let mut a = WriteKernelArgs {
        num_vectors: 1,
        dim: 1,
        normalize: false,
        out_dtype: DType::F32,
        dst: Addr(0),
    };
    assert_eq!(a.out_dtype_code(), 0);
    a.out_dtype = DType::F16;
    assert_eq!(a.out_dtype_code(), 1);
    a.out_dtype = DType::BF16;
    assert_eq!(a.out_dtype_code(), 2);
}
#[test]
fn gpu_incompatible_dtype_comes_first() {
    let e = env_with(DeviceHandle::Gpu(0), false, None, None);
    let res = reservation(0);
    let bad = tensor_of(candle_core::DType::F16, &[2, 3]);
    for d in [DType::U8, DType::U32, DType::I64] {
        let err = gpu_err(
            write_embeddings_gpu(
                &e.register,
                &e.launcher,
                &e.lane(),
                &bad,
                &pn(Normalization::L2, 4),
                d,
                &res,
            ),
        );
        assert_eq!(err, OutputError::Incompatible(d));
    }
    assert_untouched(&e);
}
#[test]
fn gpu_result_must_be_f32_n_by_dim() {
    let e = env();
    let res = reservation(1 << 20);
    let norm = pn(Normalization::L2, 4);
    for bad in [
        tensor_of(candle_core::DType::F16, &[2, 4]),
        tensor_of(candle_core::DType::U8, &[2, 4]),
        tensor_of(candle_core::DType::F32, &[8]),
        tensor_of(candle_core::DType::F32, &[2, 5]),
        tensor_of(candle_core::DType::F32, &[1, 2, 4]),
    ] {
        let err = gpu_err(
            write_embeddings_gpu(
                &e.register,
                &e.launcher,
                &e.lane(),
                &bad,
                &norm,
                DType::F32,
                &res,
            ),
        );
        assert!(matches!(err, OutputError::InvalidResult(_)), "{err:?}");
    }
    assert_untouched(&e);
}
#[test]
fn gpu_zero_dim_is_a_pool_norm_error() {
    let e = env();
    let norm = PoolNorm {
        pooling: Pooling::Mean,
        normalization: Normalization::None,
        dim: 0,
    };
    let err = gpu_err(
        write_embeddings_gpu(
            &e.register,
            &e.launcher,
            &e.lane(),
            &result(1, 0),
            &norm,
            DType::F32,
            &reservation(64),
        ),
    );
    assert_eq!(err, OutputError::PoolNorm(PoolNormError::ZeroDimension));
    assert_untouched(&e);
}
#[test]
fn gpu_slot_size_is_checked_per_output_dtype() {
    let e = env();
    let r = result(3, 4);
    let norm = pn(Normalization::None, 4);
    for (dtype, need) in [(DType::F32, 48u64), (DType::F16, 24), (DType::BF16, 24)] {
        let err = gpu_err(
            write_embeddings_gpu(
                &e.register,
                &e.launcher,
                &e.lane(),
                &r,
                &norm,
                dtype,
                &reservation(need - 1),
            ),
        );
        assert_eq!(
            err, OutputError::SlotTooSmall { required : need, available : need - 1 }
        );
        write_embeddings_gpu(
                &e.register,
                &e.launcher,
                &e.lane(),
                &r,
                &norm,
                dtype,
                &reservation(need),
            )
            .unwrap();
    }
    assert_eq!(e.calls().len(), 3);
}
#[test]
fn gpu_inverted_slot_range_counts_as_too_small() {
    let e = env();
    let mut res = reservation(0);
    res.target = 200..100;
    let err = gpu_err(
        write_embeddings_gpu(
            &e.register,
            &e.launcher,
            &e.lane(),
            &result(1, 4),
            &pn(Normalization::None, 4),
            DType::F32,
            &res,
        ),
    );
    assert_eq!(err, OutputError::SlotTooSmall { required : 16, available : 0 });
    assert_untouched(&e);
}
#[test]
fn gpu_slot_check_runs_before_the_kernel_request() {
    let e = env();
    let err = gpu_err(
        write_embeddings_gpu(
            &e.register,
            &e.launcher,
            &e.lane(),
            &result(1, 4),
            &pn(Normalization::None, 4),
            DType::F32,
            &reservation(1),
        ),
    );
    assert!(matches!(err, OutputError::SlotTooSmall { .. }));
    assert!(e.compiler.requests.lock().unwrap().is_empty());
}
#[test]
fn gpu_unregistered_kernel_is_reported() {
    let e = env_with(DeviceHandle::Gpu(0), false, None, None);
    let err = gpu_err(
        write_embeddings_gpu(
            &e.register,
            &e.launcher,
            &e.lane(),
            &result(1, 4),
            &pn(Normalization::None, 4),
            DType::F32,
            &reservation(64),
        ),
    );
    assert_eq!(
        err, OutputError::Kernel(KernelError::UnknownKernel(WRITE_KERNEL_NAME
        .to_string()))
    );
    assert_untouched(&e);
}
#[test]
fn gpu_compile_error_is_passed_through() {
    let e = env_with(
        DeviceHandle::Gpu(0),
        true,
        Some(CompilerError::Failed {
            log: "boom".into(),
        }),
        None,
    );
    let err = gpu_err(
        write_embeddings_gpu(
            &e.register,
            &e.launcher,
            &e.lane(),
            &result(1, 4),
            &pn(Normalization::None, 4),
            DType::F32,
            &reservation(64),
        ),
    );
    assert_eq!(
        err, OutputError::Kernel(KernelError::CompileError { log : "boom".into() })
    );
    assert_untouched(&e);
}
#[test]
fn gpu_cpu_device_register_reports_unavailable() {
    let e = env_with(DeviceHandle::Cpu, true, None, None);
    let err = gpu_err(
        write_embeddings_gpu(
            &e.register,
            &e.launcher,
            &e.lane(),
            &result(1, 4),
            &pn(Normalization::None, 4),
            DType::F32,
            &reservation(64),
        ),
    );
    assert!(matches!(err, OutputError::Kernel(KernelError::Unavailable(_))));
    assert_untouched(&e);
}
#[test]
fn gpu_launch_failure_becomes_a_memory_error_without_event() {
    let e = env_with(DeviceHandle::Gpu(0), true, None, Some("launch refused"));
    let err = gpu_err(
        write_embeddings_gpu(
            &e.register,
            &e.launcher,
            &e.lane(),
            &result(2, 4),
            &pn(Normalization::None, 4),
            DType::F32,
            &reservation(64),
        ),
    );
    assert_eq!(err, OutputError::Memory(AccessError::Backend("launch refused".into())));
    assert_eq!(* e.log.lock().unwrap(), vec!["launch"]);
}
#[test]
fn gpu_zero_vectors_skip_the_launch_but_record_the_event() {
    let e = env();
    let t = write_embeddings_gpu(
            &e.register,
            &e.launcher,
            &e.lane(),
            &result(0, 4),
            &pn(Normalization::L2, 4),
            DType::BF16,
            &reservation(0),
        )
        .unwrap();
    assert!(e.calls().is_empty());
    assert_eq!(* e.log.lock().unwrap(), vec!["record"]);
    drop(t);
}
#[test]
fn gpu_record_failure_is_a_lane_error() {
    let e = env();
    let err = gpu_err(
        write_embeddings_gpu(
            &e.register,
            &e.launcher,
            &e.lane_with(true),
            &result(1, 4),
            &pn(Normalization::None, 4),
            DType::F32,
            &reservation(64),
        ),
    );
    assert!(matches!(err, OutputError::Lane(LaneError::Record(_))));
    let cpu_lane = Lane::new(
        LaneConfig::preset(LaneKind::Query, DeviceHandle::Cpu),
        None,
    );
    let err = gpu_err(
        write_embeddings_gpu(
            &e.register,
            &e.launcher,
            &cpu_lane,
            &result(1, 4),
            &pn(Normalization::None, 4),
            DType::F32,
            &reservation(64),
        ),
    );
    assert_eq!(err, OutputError::Lane(LaneError::NoStreamBackend));
}
#[test]
fn gpu_kernel_is_built_once_per_dim() {
    let e = env();
    let res = reservation(1 << 16);
    for _ in 0..3 {
        write_embeddings_gpu(
                &e.register,
                &e.launcher,
                &e.lane(),
                &result(1, 8),
                &pn(Normalization::None, 8),
                DType::F32,
                &res,
            )
            .unwrap();
    }
    write_embeddings_gpu(
            &e.register,
            &e.launcher,
            &e.lane(),
            &result(1, 16),
            &pn(Normalization::None, 16),
            DType::F32,
            &res,
        )
        .unwrap();
    assert_eq!(e.compiler.requests.lock().unwrap().len(), 2);
}
#[test]
fn ticket_drop_waits_for_an_incomplete_event() {
    let e = env();
    let t = write_embeddings_gpu(
            &e.register,
            &e.launcher,
            &e.lane(),
            &result(1, 4),
            &pn(Normalization::None, 4),
            DType::F32,
            &reservation(64),
        )
        .unwrap();
    assert_eq!(e.syncs.load(Ordering::SeqCst), 0);
    drop(t);
    assert_eq!(e.syncs.load(Ordering::SeqCst), 1);
}
#[test]
fn ticket_drop_does_not_wait_for_a_completed_event() {
    let e = env();
    let t = write_embeddings_gpu(
            &e.register,
            &e.launcher,
            &e.lane(),
            &result(1, 4),
            &pn(Normalization::None, 4),
            DType::F32,
            &reservation(64),
        )
        .unwrap();
    e.ready.store(true, Ordering::SeqCst);
    assert_eq!(t.is_complete(), Ok(true));
    drop(t);
    assert_eq!(e.syncs.load(Ordering::SeqCst), 0);
}
#[test]
fn ticket_event_clone_shares_the_state() {
    let e = env();
    let t = write_embeddings_gpu(
            &e.register,
            &e.launcher,
            &e.lane(),
            &result(1, 4),
            &pn(Normalization::None, 4),
            DType::F32,
            &reservation(64),
        )
        .unwrap();
    let ev = t.event().clone();
    assert_eq!(ev.is_complete(), Ok(false));
    t.wait().unwrap();
    assert_eq!(ev.is_complete(), Ok(true));
}
#[test]
fn gpu_writes_from_many_threads_with_separate_lanes() {
    let e = Arc::new(env());
    let handles: Vec<_> = (0..8)
        .map(|_| {
            let e = e.clone();
            std::thread::spawn(move || {
                for _ in 0..20 {
                    let t = write_embeddings_gpu(
                            &e.register,
                            &e.launcher,
                            &e.lane(),
                            &result(2, 4),
                            &pn(Normalization::L2, 4),
                            DType::F16,
                            &reservation(64),
                        )
                        .unwrap();
                    t.wait().unwrap();
                }
            })
        })
        .collect();
    for h in handles {
        h.join().unwrap();
    }
    assert_eq!(e.calls().len(), 160);
}
#[test]
fn kernel_source_describes_the_write_kernel() {
    let s = write_kernel_source();
    assert_eq!(s.name, WRITE_KERNEL_NAME);
    assert_eq!(s.entry, "embedding_write");
    assert_eq!(s.dtypes, & [DType::F32]);
    assert!(s.precompiled.is_none());
    assert!(
        s.body
        .contains("extern \"C\" __global__ void embedding_write(\n    const T* src, unsigned char* dst, unsigned long long n,\n    unsigned int normalize, unsigned int out_dtype)")
    );
    for used in ["DIM", "load_f", "warp_reduce_sum"] {
        assert!(s.body.contains(used), "{used}");
    }
    for banned in ["TILE", "__CUDA_ARCH__", "printf", "cuda_fp16", "__half"] {
        assert!(! s.body.contains(banned), "{banned}");
    }
}
#[test]
fn register_write_kernel_rejects_a_second_registration() {
    let e = env();
    assert!(
        matches!(register_write_kernel(& e.register), Err(KernelError::Unavailable(_)))
    );
}
fn captured_source(dim: u32) -> CompileRequest {
    let e = env();
    e.register
        .request(
            WRITE_KERNEL_NAME,
            DType::F32,
            Variant {
                dim: Some(dim),
                tile: None,
            },
        )
        .unwrap();
    let req = e.compiler.requests.lock().unwrap()[0].clone();
    req
}
#[test]
fn jit_source_carries_dim_and_no_tile() {
    let req = captured_source(384);
    assert!(req.source.contains("#define DIM 384"));
    assert!(! req.source.contains("#define TILE"));
    assert!(req.source.contains("typedef float T;"));
    assert!(req.source.contains("embedding_write"));
}
fn find_nvcc() -> Option<String> {
    let candidates = [
        std::env::var("CUDA_HOME").map(|h| format!("{h}/bin/nvcc")).unwrap_or_default(),
        "/usr/local/cuda-13.3/bin/nvcc".to_string(),
        "nvcc".to_string(),
    ];
    candidates
        .into_iter()
        .filter(|c| !c.is_empty())
        .find(|c| {
            std::process::Command::new(c)
                .arg("--version")
                .output()
                .map(|o| o.status.success())
                .unwrap_or(false)
        })
}
fn scratch_dir(tag: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir()
        .join(format!("embedding_output_{tag}_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}
#[test]
fn jit_source_compiles_with_nvcc_when_available() {
    let Some(nvcc) = find_nvcc() else {
        eprintln!("skipped: nvcc not found");
        return;
    };
    let dir = scratch_dir("ptx");
    for dim in [1u32, 33, 384] {
        let req = captured_source(dim);
        let src = dir.join(format!("write_{dim}.cu"));
        std::fs::write(&src, &req.source).unwrap();
        let out = std::process::Command::new(&nvcc)
            .arg("-ptx")
            .arg(format!("-arch={}", req.arch))
            .arg("-o")
            .arg(dir.join("out.ptx"))
            .arg(&src)
            .output()
            .unwrap();
        assert!(
            out.status.success(), "nvcc failed: {}", String::from_utf8_lossy(& out
            .stderr)
        );
    }
    let _ = std::fs::remove_dir_all(&dir);
}
const HARNESS: &str = r#"
#include <cstdio>
#include <cstdlib>
#include <vector>
int main(int argc, char** argv) {
    if (argc < 6) return 2;
    unsigned long long n = strtoull(argv[3], 0, 10);
    unsigned int normalize = (unsigned int)atoi(argv[4]);
    unsigned int dt = (unsigned int)atoi(argv[5]);
    int count = 0;
    if (cudaGetDeviceCount(&count) != cudaSuccess || count == 0) return 77;
    size_t in_elems = (size_t)n * DIM;
    size_t width = dt == 0 ? 4 : 2;
    size_t out_bytes = in_elems * width;
    size_t total = out_bytes + 32;
    std::vector<float> h(in_elems);
    FILE* f = fopen(argv[1], "rb");
    if (!f) return 3;
    if (fread(h.data(), 4, in_elems, f) != in_elems) return 4;
    fclose(f);
    float* d_src = 0;
    unsigned char* d_dst = 0;
    if (cudaMalloc(&d_src, in_elems * 4) != cudaSuccess) return 5;
    if (cudaMalloc(&d_dst, total) != cudaSuccess) return 5;
    cudaMemcpy(d_src, h.data(), in_elems * 4, cudaMemcpyHostToDevice);
    cudaMemset(d_dst, 0xCD, total);
    embedding_write<<<(unsigned int)n, 32>>>(d_src, d_dst + 3, n, normalize, dt);
    if (cudaGetLastError() != cudaSuccess) return 6;
    if (cudaDeviceSynchronize() != cudaSuccess) return 7;
    std::vector<unsigned char> o(total);
    cudaMemcpy(o.data(), d_dst, total, cudaMemcpyDeviceToHost);
    FILE* g = fopen(argv[2], "wb");
    if (!g) return 8;
    fwrite(o.data(), 1, total, g);
    fclose(g);
    return 0;
}
"#;
struct GpuRun {
    exe: std::path::PathBuf,
    dir: std::path::PathBuf,
}
fn build_gpu_run(dim: u32, tag: &str) -> Option<GpuRun> {
    let nvcc = find_nvcc()?;
    let dir = scratch_dir(tag);
    let req = captured_source(dim);
    let src = dir.join("run.cu");
    std::fs::write(&src, format!("{}\n{}", req.source, HARNESS)).unwrap();
    let exe = dir.join("run");
    let out = std::process::Command::new(nvcc)
        .arg(format!("-arch={}", req.arch))
        .arg("-o")
        .arg(&exe)
        .arg(&src)
        .output()
        .unwrap();
    assert!(
        out.status.success(), "nvcc failed: {}", String::from_utf8_lossy(& out.stderr)
    );
    Some(GpuRun { exe, dir })
}
impl GpuRun {
    fn run(&self, input: &[f32], normalize: bool, dtype: DType) -> Option<Vec<u8>> {
        let dim = self.dim_of(input.len());
        let n = input.len() / dim;
        let in_path = self.dir.join("in.bin");
        let out_path = self.dir.join("out.bin");
        let raw: Vec<u8> = input.iter().flat_map(|x| x.to_le_bytes()).collect();
        std::fs::write(&in_path, raw).unwrap();
        let code = match dtype {
            DType::F32 => 0,
            DType::F16 => 1,
            _ => 2,
        };
        let status = std::process::Command::new(&self.exe)
            .arg(&in_path)
            .arg(&out_path)
            .arg(n.to_string())
            .arg((normalize as u32).to_string())
            .arg(code.to_string())
            .status()
            .unwrap();
        if status.code() == Some(77) {
            eprintln!("skipped: no CUDA device");
            return None;
        }
        assert!(status.success(), "harness failed: {status:?}");
        Some(std::fs::read(&out_path).unwrap())
    }
    fn dim_of(&self, _len: usize) -> usize {
        std::fs::read_to_string(self.dir.join("dim")).unwrap().trim().parse().unwrap()
    }
}
fn gpu_run(dim: u32, tag: &str) -> Option<GpuRun> {
    let run = build_gpu_run(dim, tag);
    match &run {
        Some(r) => std::fs::write(r.dir.join("dim"), dim.to_string()).unwrap(),
        None => eprintln!("skipped: nvcc not found"),
    }
    run
}
#[test]
fn kernel_conversion_is_bit_exact_on_the_device() {
    let dim = 40usize;
    let Some(run) = gpu_run(dim as u32, "exact") else { return };
    let table = f16_table();
    let mut input: Vec<f32> = Vec::new();
    for lo in (0..0x7BFFusize).step_by(7) {
        let mid = ((table[lo] + table[lo + 1]) / 2.0) as f32;
        let b = mid.to_bits();
        input.extend([f32::from_bits(b - 1), mid, f32::from_bits(b + 1)]);
    }
    let mut rng = Lcg(99);
    for _ in 0..30_000 {
        input.push(f32::from_bits(rng.next()));
    }
    input
        .extend([
            0.0,
            -0.0,
            1.0,
            65504.0,
            65520.0,
            1e10,
            -1e10,
            f32::INFINITY,
            f32::NEG_INFINITY,
            f32::NAN,
            2f32.powi(-24),
            2f32.powi(-25),
            f32::MAX,
            f32::MIN_POSITIVE,
        ]);
    let signed: Vec<f32> = input.iter().flat_map(|x| [*x, -*x]).collect();
    input = signed;
    while input.len() % dim != 0 {
        input.push(0.5);
    }
    for dtype in [DType::F32, DType::F16, DType::BF16] {
        let Some(out) = run.run(&input, false, dtype) else { break };
        let want = encode_f32_slice(&input, dtype).unwrap();
        assert_eq!(& out[..3], & [0xCD; 3], "guard before");
        assert_eq!(& out[3..3 + want.len()], & want[..], "{dtype:?}");
        assert!(
            out[3 + want.len()..].iter().all(| b | * b == 0xCD), "guard after {dtype:?}"
        );
    }
    let _ = std::fs::remove_dir_all(&run.dir);
}
#[test]
fn kernel_normalization_matches_the_host_on_the_device() {
    let dim = 33usize;
    let Some(run) = gpu_run(dim as u32, "norm") else { return };
    let mut rng = Lcg(5);
    let mut input: Vec<f32> = Vec::new();
    for _ in 0..40 {
        let scale = [1e-3f32, 1.0, 50.0][(rng.next() % 3) as usize];
        input.extend((0..dim).map(|_| rng.unit() * scale));
    }
    input.extend(vec![0.0; dim]);
    let mut with_nan = vec![1.0f32; dim];
    with_nan[3] = f32::NAN;
    input.extend(with_nan);
    input.extend((0..dim).map(|i| if i == 0 { 1e-20 } else { 0.0 }));
    let n = input.len() / dim;
    let norm = pn(Normalization::L2, dim);
    let mut host = input.clone();
    norm.normalize(&mut host).unwrap();
    for dtype in [DType::F32, DType::F16, DType::BF16] {
        let Some(out) = run.run(&input, true, dtype) else { break };
        let es = dtype.size_in_bytes();
        let body = &out[3..3 + n * dim * es];
        assert!(out[3 + n * dim * es..].iter().all(| b | * b == 0xCD));
        for (i, h) in host.iter().enumerate() {
            let c = &body[i * es..i * es + es];
            let got = match dtype {
                DType::F32 => f32::from_le_bytes(c.try_into().unwrap()),
                DType::F16 => f16_to_f32(u16::from_le_bytes(c.try_into().unwrap())),
                _ => bf16_to_f32(u16::from_le_bytes(c.try_into().unwrap())),
            };
            if h.is_nan() {
                assert!(got.is_nan(), "{dtype:?} #{i}");
                continue;
            }
            let tol = match dtype {
                DType::F32 => 1e-6,
                DType::F16 => 2e-3,
                _ => 1.6e-2,
            };
            assert!(
                (got - h).abs() <= tol * h.abs() + 1e-6,
                "{dtype:?} #{i}: device {got} host {h}"
            );
        }
    }
    let _ = std::fs::remove_dir_all(&run.dir);
}
