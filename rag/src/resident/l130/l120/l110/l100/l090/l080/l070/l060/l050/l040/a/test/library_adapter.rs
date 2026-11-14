use super::*;
use candle_core::Device;
use super::super::super::l030::a::region_handle::Generation;
use super::super::super::l030::l020::a::arenas::ArenaKind;
const _: fn() = || {
    fn check<T: Send + Sync + 'static>() {}
    check::<LibTensor>();
    check::<DeviceBlock>();
    check::<LibraryKernelSource>();
    check::<LibKernelEntry>();
    check::<AdapterError>();
};
const ALL_DTYPES: [DType; 6] = [
    DType::F32,
    DType::F16,
    DType::BF16,
    DType::U8,
    DType::U32,
    DType::I64,
];
fn region(offset: u64, len: u64) -> RegionHandle {
    RegionHandle::new(ArenaKind::Workspace, offset, len, Generation::INITIAL).unwrap()
}
fn f32_bytes(v: &[f32]) -> Vec<u8> {
    v.iter().flat_map(|x| x.to_ne_bytes()).collect()
}
fn bytes_f32(b: &[u8]) -> Vec<f32> {
    b.chunks_exact(4).map(|c| f32::from_ne_bytes(c.try_into().unwrap())).collect()
}
fn cpu_f32(v: Vec<f32>, shape: &[usize]) -> LibTensor {
    LibTensor::from_lib(Tensor::from_vec(v, shape, &Device::Cpu).unwrap())
}
fn cpu_u32(v: Vec<u32>, shape: &[usize]) -> LibTensor {
    LibTensor::from_lib(Tensor::from_vec(v, shape, &Device::Cpu).unwrap())
}
fn run(name: &str, args: &[LibTensor]) -> Result<Vec<LibTensor>, AdapterError> {
    let source = library_kernel_source();
    let entry = source.get(name).unwrap();
    (entry.run)(args)
}
#[test]
fn dtype_round_trip_for_all_variants() {
    for d in ALL_DTYPES {
        assert_eq!(from_lib_dtype(to_lib_dtype(d)), Ok(d));
    }
    assert_eq!(to_lib_dtype(DType::F32), candle_core::DType::F32);
    assert_eq!(to_lib_dtype(DType::I64), candle_core::DType::I64);
    assert_eq!(to_lib_dtype(DType::BF16), candle_core::DType::BF16);
}
#[test]
fn unsupported_lib_dtypes_are_rejected() {
    assert_eq!(
        from_lib_dtype(candle_core::DType::F64),
        Err(AdapterError::UnsupportedLibDType("F64".into()))
    );
    assert!(
        matches!(from_lib_dtype(candle_core::DType::I16),
        Err(AdapterError::UnsupportedLibDType(_)))
    );
    assert!(
        matches!(from_lib_dtype(candle_core::DType::I32),
        Err(AdapterError::UnsupportedLibDType(_)))
    );
}
#[test]
fn size_conversions() {
    assert_eq!(elems_to_bytes(DType::F32, 3), Ok(12));
    assert_eq!(elems_to_bytes(DType::U8, 7), Ok(7));
    assert_eq!(elems_to_bytes(DType::I64, 0), Ok(0));
    assert_eq!(elems_to_bytes(DType::I64, u64::MAX), Err(AdapterError::Overflow));
    assert_eq!(elems_to_bytes(DType::U8, u64::MAX), Ok(u64::MAX));
    assert_eq!(bytes_to_elems(DType::F16, 0), Ok(0));
    assert_eq!(bytes_to_elems(DType::F32, 12), Ok(3));
    assert_eq!(
        bytes_to_elems(DType::F32, 10), Err(AdapterError::NotElementMultiple { bytes :
        10, elem_size : 4 })
    );
}
#[test]
fn size_conversions_are_inverse() {
    for d in ALL_DTYPES {
        for elems in [0u64, 1, 2, 17, 1 << 40] {
            let bytes = elems_to_bytes(d, elems).unwrap();
            assert_eq!(bytes_to_elems(d, bytes), Ok(elems));
        }
    }
}
#[test]
fn validate_accepts_exact_and_larger_region() {
    let layout = Layout::contiguous(&[2, 3], DType::F32);
    assert_eq!(validate_wrap(1024, 256, & region(16, 24), & layout), Ok(()));
    assert_eq!(validate_wrap(1024, 256, & region(16, 64), & layout), Ok(()));
    assert_eq!(validate_wrap(40, 0, & region(16, 24), & layout), Ok(()));
}
#[test]
fn validate_out_of_bounds() {
    let layout = Layout::contiguous(&[2, 3], DType::F32);
    assert_eq!(
        validate_wrap(32, 0, & region(16, 24), & layout), Err(AdapterError::OutOfBounds {
        offset : 16, len : 24, block_len : 32 })
    );
}
#[test]
fn validate_not_contiguous() {
    let layout = Layout::new(vec![2, 3], vec![4, 8], DType::F32).unwrap();
    assert_eq!(
        validate_wrap(1024, 0, & region(0, 64), & layout),
        Err(AdapterError::NotContiguous)
    );
}
#[test]
fn validate_layout_exceeds_region() {
    let layout = Layout::contiguous(&[2, 3], DType::F32);
    assert_eq!(
        validate_wrap(1024, 0, & region(0, 20), & layout),
        Err(AdapterError::LayoutExceedsRegion { required : 24, region_len : 20 })
    );
}
#[test]
fn validate_misaligned() {
    let layout = Layout::contiguous(&[2], DType::F32);
    assert_eq!(
        validate_wrap(1024, 4, & region(2, 8), & layout), Err(AdapterError::Misaligned {
        addr : 6, align : 4 })
    );
    assert_eq!(
        validate_wrap(1024, 2, & region(4, 8), & layout), Err(AdapterError::Misaligned {
        addr : 6, align : 4 })
    );
    let bytes = Layout::contiguous(&[3], DType::U8);
    assert_eq!(validate_wrap(1024, 1, & region(1, 3), & bytes), Ok(()));
}
#[test]
fn validate_address_overflow() {
    let layout = Layout::contiguous(&[2], DType::U8);
    assert_eq!(
        validate_wrap(1024, u64::MAX - 1, & region(4, 8), & layout),
        Err(AdapterError::Overflow)
    );
}
#[test]
fn validate_shape_overflow() {
    let layout = Layout::new(vec![usize::MAX, 4], vec![4, 1], DType::U8).unwrap();
    assert_eq!(
        validate_wrap(1024, 0, & region(0, 8), & layout), Err(AdapterError::Overflow)
    );
}
#[test]
fn validate_first_violation_wins() {
    let strided = Layout::new(vec![2, 3], vec![4, 8], DType::F32).unwrap();
    assert!(
        matches!(validate_wrap(8, 2, & region(0, 24), & strided),
        Err(AdapterError::OutOfBounds { .. }))
    );
    assert_eq!(
        validate_wrap(1024, 2, & region(0, 4), & strided),
        Err(AdapterError::NotContiguous)
    );
    let layout = Layout::contiguous(&[2, 3], DType::F32);
    assert!(
        matches!(validate_wrap(1024, 2, & region(0, 4), & layout),
        Err(AdapterError::LayoutExceedsRegion { .. }))
    );
}
#[test]
fn validate_empty_layout_only_checks_bounds() {
    let empty = Layout::contiguous(&[0, 3], DType::F32);
    assert_eq!(validate_wrap(64, 3, & region(64, 0), & empty), Ok(()));
    assert_eq!(validate_wrap(64, 3, & region(5, 0), & empty), Ok(()));
    assert_eq!(
        validate_wrap(64, 0, & region(65, 0), & empty), Err(AdapterError::OutOfBounds {
        offset : 65, len : 0, block_len : 64 })
    );
    let huge_empty = Layout::new(
            vec![usize::MAX, usize::MAX, 0],
            vec![0, 0, 0],
            DType::F32,
        )
        .unwrap();
    assert_eq!(validate_wrap(64, 0, & region(0, 0), & huge_empty), Ok(()));
}
#[test]
fn error_display_and_trait() {
    let e: Box<dyn std::error::Error> = Box::new(AdapterError::NotContiguous);
    assert!(! e.to_string().is_empty());
    assert!(AdapterError::UnknownKernel("x".into()).to_string().contains('x'));
}
#[test]
fn from_lib_is_owned_and_reports_metadata() {
    let t = cpu_f32(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], &[2, 3]);
    assert!(! t.is_borrowed());
    assert_eq!(t.dims(), & [2, 3]);
    assert_eq!(t.dtype(), Ok(DType::F32));
    assert_eq!(t.to_host_bytes().unwrap(), f32_bytes(& [1.0, 2.0, 3.0, 4.0, 5.0, 6.0]));
    let lib = t.clone().into_lib();
    assert_eq!(lib.dims(), & [2, 3]);
    assert_eq!(t.lib().dims(), & [2, 3]);
}
#[test]
fn to_host_bytes_all_dtypes() {
    let t = LibTensor::from_lib(
        Tensor::from_vec(vec![1u8, 2, 3], 3, &Device::Cpu).unwrap(),
    );
    assert_eq!(t.to_host_bytes().unwrap(), vec![1, 2, 3]);
    let t = LibTensor::from_lib(
        Tensor::from_vec(vec![- 5i64, 7], 2, &Device::Cpu).unwrap(),
    );
    let expected: Vec<u8> = [-5i64, 7].iter().flat_map(|x| x.to_ne_bytes()).collect();
    assert_eq!(t.to_host_bytes().unwrap(), expected);
    let h = vec![half::f16::from_f32(1.5), half::f16::from_f32(- 2.0)];
    let expected: Vec<u8> = h.iter().flat_map(|x| x.to_ne_bytes()).collect();
    let t = LibTensor::from_lib(Tensor::from_vec(h, 2, &Device::Cpu).unwrap());
    assert_eq!(t.dtype(), Ok(DType::F16));
    assert_eq!(t.to_host_bytes().unwrap(), expected);
    let b = vec![half::bf16::from_f32(3.0)];
    let expected: Vec<u8> = b.iter().flat_map(|x| x.to_ne_bytes()).collect();
    let t = LibTensor::from_lib(Tensor::from_vec(b, 1, &Device::Cpu).unwrap());
    assert_eq!(t.to_host_bytes().unwrap(), expected);
}
#[test]
fn owned_copy_is_independent() {
    let a = cpu_f32(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);
    let c = a.to_owned_copy().unwrap();
    assert!(! c.is_borrowed());
    assert_eq!(c.to_host_bytes().unwrap(), a.to_host_bytes().unwrap());
    c.write_from(&cpu_f32(vec![9.0; 4], &[2, 2])).unwrap();
    assert_eq!(bytes_f32(& a.to_host_bytes().unwrap()), vec![1.0, 2.0, 3.0, 4.0]);
    assert_eq!(bytes_f32(& c.to_host_bytes().unwrap()), vec![9.0; 4]);
}
#[test]
fn owned_copy_of_offset_view_and_transpose() {
    let base = Tensor::from_vec(vec![0f32, 1.0, 2.0, 3.0, 4.0, 5.0], 6, &Device::Cpu)
        .unwrap();
    let view = LibTensor::from_lib(base.narrow(0, 2, 3).unwrap());
    let c = view.to_owned_copy().unwrap();
    assert_eq!(bytes_f32(& c.to_host_bytes().unwrap()), vec![2.0, 3.0, 4.0]);
    let m = Tensor::from_vec(vec![1f32, 2.0, 3.0, 4.0, 5.0, 6.0], &[2, 3], &Device::Cpu)
        .unwrap();
    let t = LibTensor::from_lib(m.t().unwrap());
    let c = t.to_owned_copy().unwrap();
    assert_eq!(c.dims(), & [3, 2]);
    assert_eq!(
        bytes_f32(& c.to_host_bytes().unwrap()), vec![1.0, 4.0, 2.0, 5.0, 3.0, 6.0]
    );
}
#[test]
fn owned_copy_of_scalar_and_empty() {
    let s = LibTensor::from_lib(Tensor::new(4.5f32, &Device::Cpu).unwrap());
    let c = s.to_owned_copy().unwrap();
    assert_eq!(c.dims(), & [] as & [usize]);
    assert_eq!(bytes_f32(& c.to_host_bytes().unwrap()), vec![4.5]);
    let e = LibTensor::from_lib(
        Tensor::zeros((0, 3), candle_core::DType::F32, &Device::Cpu).unwrap(),
    );
    let c = e.to_owned_copy().unwrap();
    assert_eq!(c.dims(), & [0, 3]);
    assert!(c.to_host_bytes().unwrap().is_empty());
    c.write_from(&e).unwrap();
}
#[test]
fn write_from_touches_only_the_view() {
    let base = Tensor::from_vec(vec![0f32, 1.0, 2.0, 3.0, 4.0, 5.0], 6, &Device::Cpu)
        .unwrap();
    let view = LibTensor::from_lib(base.narrow(0, 2, 2).unwrap());
    view.write_from(&cpu_f32(vec![20.0, 30.0], &[2])).unwrap();
    let all = LibTensor::from_lib(base);
    assert_eq!(
        bytes_f32(& all.to_host_bytes().unwrap()), vec![0.0, 1.0, 20.0, 30.0, 4.0, 5.0]
    );
}
#[test]
fn write_from_rejects_mismatches() {
    let dst = cpu_f32(vec![0.0; 4], &[2, 2]);
    assert!(
        matches!(dst.write_from(& cpu_f32(vec![0.0; 4], & [4])),
        Err(AdapterError::Mismatch(_)))
    );
    assert!(
        matches!(dst.write_from(& cpu_u32(vec![0; 4], & [2, 2])),
        Err(AdapterError::Mismatch(_)))
    );
    let strided = LibTensor::from_lib(
        Tensor::from_vec(vec![0f32; 6], &[2, 3], &Device::Cpu).unwrap().t().unwrap(),
    );
    assert!(
        matches!(strided.write_from(& cpu_f32(vec![0.0; 6], & [3, 2])),
        Err(AdapterError::Mismatch(_)))
    );
}
#[test]
fn kernel_source_listing() {
    let source = library_kernel_source();
    assert_eq!(source.source_name(), "candle");
    let listed: Vec<(&str, usize)> = source
        .kernels()
        .iter()
        .map(|e| (e.name, e.arity))
        .collect();
    assert_eq!(
        listed, vec![("matmul", 2), ("add", 2), ("mul", 2), ("softmax_last_dim", 1)]
    );
    let again = library_kernel_source();
    let again_listed: Vec<(&str, usize)> = again
        .kernels()
        .iter()
        .map(|e| (e.name, e.arity))
        .collect();
    assert_eq!(listed, again_listed);
    assert_eq!(source.get("add").unwrap().name, "add");
    assert_eq!(
        source.get("conv2d").unwrap_err(), AdapterError::UnknownKernel("conv2d".into())
    );
}
#[test]
fn kernel_matmul_add_mul() {
    let a = cpu_f32(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);
    let b = cpu_f32(vec![5.0, 6.0, 7.0, 8.0], &[2, 2]);
    let out = run("matmul", &[a.clone(), b.clone()]).unwrap();
    assert_eq!(out.len(), 1);
    assert!(! out[0].is_borrowed());
    assert_eq!(
        bytes_f32(& out[0].to_host_bytes().unwrap()), vec![19.0, 22.0, 43.0, 50.0]
    );
    let out = run("add", &[a.clone(), b.clone()]).unwrap();
    assert!(! out[0].is_borrowed());
    assert_eq!(bytes_f32(& out[0].to_host_bytes().unwrap()), vec![6.0, 8.0, 10.0, 12.0]);
    let out = run("mul", &[a, b]).unwrap();
    assert!(! out[0].is_borrowed());
    assert_eq!(
        bytes_f32(& out[0].to_host_bytes().unwrap()), vec![5.0, 12.0, 21.0, 32.0]
    );
}
#[test]
fn kernel_integer_add() {
    let out = run("add", &[cpu_u32(vec![1, 2], &[2]), cpu_u32(vec![3, 4], &[2])])
        .unwrap();
    let expected: Vec<u8> = [4u32, 6].iter().flat_map(|x| x.to_ne_bytes()).collect();
    assert_eq!(out[0].to_host_bytes().unwrap(), expected);
}
#[test]
fn kernel_matmul_batched_and_rectangular() {
    let a = cpu_f32((0..12).map(|x| x as f32).collect(), &[2, 2, 3]);
    let b = cpu_f32(vec![1.0; 12], &[2, 3, 2]);
    let out = run("matmul", &[a, b]).unwrap();
    assert_eq!(out[0].dims(), & [2, 2, 2]);
    assert_eq!(
        bytes_f32(& out[0].to_host_bytes().unwrap()), vec![3.0, 3.0, 12.0, 12.0, 21.0,
        21.0, 30.0, 30.0]
    );
}
#[test]
fn kernel_softmax_last_dim() {
    let x = cpu_f32(vec![0.0, 0.0, 1.0, 2.0], &[2, 2]);
    let out = run("softmax_last_dim", &[x]).unwrap();
    assert!(! out[0].is_borrowed());
    let v = bytes_f32(&out[0].to_host_bytes().unwrap());
    let e = 1.0f32.exp();
    let expected = [0.5, 0.5, 1.0 / (1.0 + e), e / (1.0 + e)];
    for (got, want) in v.iter().zip(expected) {
        assert!((got - want).abs() < 1e-5, "{got} vs {want}");
    }
    let empty = LibTensor::from_lib(
        Tensor::zeros((0, 3), candle_core::DType::F32, &Device::Cpu).unwrap(),
    );
    let out = run("softmax_last_dim", &[empty]).unwrap();
    assert_eq!(out[0].dims(), & [0, 3]);
    assert!(! out[0].is_borrowed());
}
#[test]
fn kernel_operand_errors() {
    let a = cpu_f32(vec![1.0; 4], &[2, 2]);
    let v = cpu_f32(vec![1.0; 3], &[3]);
    let u = cpu_u32(vec![1; 4], &[2, 2]);
    for name in ["matmul", "add", "mul", "softmax_last_dim"] {
        assert!(
            matches!(run(name, & []), Err(AdapterError::Mismatch(_))), "{name} arity"
        );
        assert!(
            matches!(run(name, & [a.clone(), a.clone(), a.clone()]),
            Err(AdapterError::Mismatch(_))), "{name} arity"
        );
    }
    assert!(matches!(run("add", & [a.clone()]), Err(AdapterError::Mismatch(_))));
    assert!(
        matches!(run("softmax_last_dim", & [a.clone(), a.clone()]),
        Err(AdapterError::Mismatch(_)))
    );
    assert!(
        matches!(run("add", & [a.clone(), v.clone()]), Err(AdapterError::Mismatch(_)))
    );
    assert!(
        matches!(run("mul", & [a.clone(), u.clone()]), Err(AdapterError::Mismatch(_)))
    );
    assert!(
        matches!(run("matmul", & [a.clone(), v.clone()]), Err(AdapterError::Mismatch(_)))
    );
    assert!(
        matches!(run("matmul", & [u.clone(), u.clone()]), Err(AdapterError::Mismatch(_)))
    );
    let wide = cpu_f32(vec![1.0; 6], &[3, 2]);
    assert!(
        matches!(run("matmul", & [a.clone(), wide]), Err(AdapterError::Mismatch(_)))
    );
    assert!(matches!(run("softmax_last_dim", & [u]), Err(AdapterError::Mismatch(_))));
    let scalar = LibTensor::from_lib(Tensor::new(1f32, &Device::Cpu).unwrap());
    assert!(
        matches!(run("softmax_last_dim", & [scalar]), Err(AdapterError::Mismatch(_)))
    );
    let f64t = LibTensor::from_lib(Tensor::new(&[1f64, 2.0], &Device::Cpu).unwrap());
    assert!(
        matches!(run("add", & [f64t.clone(), f64t]), Err(AdapterError::Mismatch(_)))
    );
}
#[test]
fn kernels_run_concurrently() {
    let source = library_kernel_source();
    std::thread::scope(|s| {
        for i in 0..8 {
            let source = source.clone();
            s.spawn(move || {
                let a = cpu_f32(vec![i as f32; 4], &[2, 2]);
                let b = cpu_f32(vec![1.0; 4], &[2, 2]);
                let out = (source.get("add").unwrap().run)(&[a, b]).unwrap();
                assert_eq!(
                    bytes_f32(& out[0].to_host_bytes().unwrap()), vec![i as f32 + 1.0; 4]
                );
            });
        }
    });
}
mod gpu {
    use super::*;
    use cudarc::driver::{CudaSlice, CudaStream, DevicePtr};
    struct Fixture {
        device: CudaDevice,
        stream: Arc<CudaStream>,
        buf: CudaSlice<u8>,
        block: DeviceBlock,
    }
    impl Fixture {
        fn read(&self) -> Vec<u8> {
            self.stream.clone_dtoh(&self.buf).unwrap()
        }
        fn write(&mut self, data: &[u8]) {
            self.stream.memcpy_htod(data, &mut self.buf).unwrap();
        }
        fn tensor(&self, v: Vec<f32>, shape: &[usize]) -> LibTensor {
            let t = Tensor::from_vec(v, shape, &Device::Cuda(self.device.clone()))
                .unwrap();
            LibTensor::from_lib(t)
        }
    }
    fn fixture(bytes: &[u8]) -> Option<Fixture> {
        let device = std::panic::catch_unwind(|| CudaDevice::new_with_stream(0))
            .ok()?
            .ok()?;
        let stream = device.cuda_stream();
        let buf = stream.clone_htod(bytes).ok()?;
        let base_addr = {
            let (ptr, _sync) = buf.device_ptr(&stream);
            ptr
        };
        let block = DeviceBlock {
            device: device.clone(),
            base_addr,
            len_bytes: bytes.len() as u64,
        };
        Some(Fixture {
            device,
            stream,
            buf,
            block,
        })
    }
    fn sequence() -> Vec<u8> {
        f32_bytes(&(0..16).map(|x| x as f32).collect::<Vec<_>>())
    }
    macro_rules! fixture_or_skip {
        ($bytes:expr) => {
            match fixture($bytes) { Some(f) => f, None => {
            eprintln!("no CUDA device available, skipping"); return; } }
        };
    }
    #[test]
    fn wrap_reads_exact_subrange_at_offset() {
        let bytes = sequence();
        let fx = fixture_or_skip!(& bytes);
        let layout = Layout::contiguous(&[2, 3], DType::F32);
        let t = unsafe { wrap_region(&fx.block, &region(16, 24), &layout) }.unwrap();
        assert!(t.is_borrowed());
        assert_eq!(t.dims(), & [2, 3]);
        assert_eq!(t.dtype(), Ok(DType::F32));
        assert!(t.lib().device().is_cuda());
        assert_eq!(t.to_host_bytes().unwrap(), bytes[16..40].to_vec());
    }
    #[test]
    fn wrap_all_dtypes_read_raw_bytes() {
        let bytes: Vec<u8> = (0u8..64).collect();
        let fx = fixture_or_skip!(& bytes);
        for d in ALL_DTYPES {
            let size = d.size_in_bytes() as u64;
            let layout = Layout::contiguous(&[3], d);
            let t = unsafe {
                wrap_region(&fx.block, &region(size * 2, size * 3), &layout)
            }
                .unwrap();
            assert_eq!(t.dtype(), Ok(d));
            let from = (size * 2) as usize;
            assert_eq!(
                t.to_host_bytes().unwrap(), bytes[from..from + (size * 3) as usize]
                .to_vec()
            );
        }
    }
    #[test]
    fn dropping_tensor_keeps_block_intact() {
        let bytes = sequence();
        let mut fx = fixture_or_skip!(& bytes);
        let layout = Layout::contiguous(&[4], DType::F32);
        let t = unsafe { wrap_region(&fx.block, &region(8, 16), &layout) }.unwrap();
        let clone = t.clone();
        let view = t.lib().narrow(0, 1, 2).unwrap();
        drop(t);
        drop(clone);
        drop(view);
        assert_eq!(fx.read(), bytes);
        let fresh: Vec<u8> = (0u8..64).rev().collect();
        fx.write(&fresh);
        assert_eq!(fx.read(), fresh);
        let again = unsafe { wrap_region(&fx.block, &region(8, 16), &layout) }.unwrap();
        assert_eq!(again.to_host_bytes().unwrap(), fresh[8..24].to_vec());
    }
    #[test]
    fn copy_has_its_own_memory() {
        let bytes = sequence();
        let mut fx = fixture_or_skip!(& bytes);
        let layout = Layout::contiguous(&[2, 3], DType::F32);
        let w = unsafe { wrap_region(&fx.block, &region(16, 24), &layout) }.unwrap();
        let c = w.to_owned_copy().unwrap();
        assert!(! c.is_borrowed());
        assert_eq!(c.to_host_bytes().unwrap(), bytes[16..40].to_vec());
        fx.write(&vec![0u8; 64]);
        assert_eq!(c.to_host_bytes().unwrap(), bytes[16..40].to_vec());
        assert_eq!(w.to_host_bytes().unwrap(), vec![0u8; 24]);
        c.write_from(&fx.tensor(vec![7.0; 6], &[2, 3])).unwrap();
        assert_eq!(fx.read(), vec![0u8; 64]);
        assert_eq!(bytes_f32(& c.to_host_bytes().unwrap()), vec![7.0; 6]);
    }
    #[test]
    fn write_from_hits_only_the_subrange() {
        let bytes = sequence();
        let fx = fixture_or_skip!(& bytes);
        let layout = Layout::contiguous(&[2, 3], DType::F32);
        let w = unsafe { wrap_region(&fx.block, &region(16, 24), &layout) }.unwrap();
        let new = vec![100.0f32, 101.0, 102.0, 103.0, 104.0, 105.0];
        w.write_from(&fx.tensor(new.clone(), &[2, 3])).unwrap();
        let after = fx.read();
        assert_eq!(after[..16], bytes[..16]);
        assert_eq!(after[40..], bytes[40..]);
        assert_eq!(after[16..40].to_vec(), f32_bytes(& new));
        assert_eq!(w.to_host_bytes().unwrap(), f32_bytes(& new));
    }
    #[test]
    fn write_from_on_wrapped_view_leaves_neighbours() {
        let bytes = sequence();
        let fx = fixture_or_skip!(& bytes);
        let layout = Layout::contiguous(&[16], DType::F32);
        let w = unsafe { wrap_region(&fx.block, &region(0, 64), &layout) }.unwrap();
        let part = LibTensor::from_lib(w.lib().narrow(0, 5, 2).unwrap());
        part.write_from(&fx.tensor(vec![- 1.0, - 2.0], &[2])).unwrap();
        let mut expected = bytes.clone();
        expected[20..28].copy_from_slice(&f32_bytes(&[-1.0, -2.0]));
        assert_eq!(fx.read(), expected);
    }
    #[test]
    fn write_from_rejects_other_device() {
        let bytes = sequence();
        let fx = fixture_or_skip!(& bytes);
        let layout = Layout::contiguous(&[2], DType::F32);
        let w = unsafe { wrap_region(&fx.block, &region(0, 8), &layout) }.unwrap();
        let host = cpu_f32(vec![1.0, 2.0], &[2]);
        assert!(matches!(w.write_from(& host), Err(AdapterError::Mismatch(_))));
        assert!(
            matches!(w.write_from(& fx.tensor(vec![1.0; 3], & [3])),
            Err(AdapterError::Mismatch(_)))
        );
        assert_eq!(fx.read(), bytes);
    }
    #[test]
    fn empty_layout_wraps_without_device_access() {
        let bytes = sequence();
        let fx = fixture_or_skip!(& bytes);
        let layout = Layout::contiguous(&[0, 4], DType::F32);
        let t = unsafe { wrap_region(&fx.block, &region(64, 0), &layout) }.unwrap();
        assert!(t.is_borrowed());
        assert_eq!(t.dims(), & [0, 4]);
        assert!(t.to_host_bytes().unwrap().is_empty());
        drop(t);
        assert_eq!(fx.read(), bytes);
    }
    #[test]
    fn wrap_rejects_invalid_regions_before_device_access() {
        let bytes = sequence();
        let fx = fixture_or_skip!(& bytes);
        let layout = Layout::contiguous(&[4], DType::F32);
        assert!(
            matches!(unsafe { wrap_region(& fx.block, & region(56, 16), & layout) },
            Err(AdapterError::OutOfBounds { .. }))
        );
        assert!(
            matches!(unsafe { wrap_region(& fx.block, & region(2, 16), & layout) },
            Err(AdapterError::Misaligned { .. }))
        );
    }
    #[test]
    fn kernels_on_wrapped_operands_return_owned_results() {
        let bytes = sequence();
        let fx = fixture_or_skip!(& bytes);
        let layout = Layout::contiguous(&[2, 2], DType::F32);
        let a = unsafe { wrap_region(&fx.block, &region(0, 16), &layout) }.unwrap();
        let b = unsafe { wrap_region(&fx.block, &region(16, 16), &layout) }.unwrap();
        let out = run("add", &[a.clone(), b.clone()]).unwrap();
        assert!(! out[0].is_borrowed());
        assert_eq!(
            bytes_f32(& out[0].to_host_bytes().unwrap()), vec![4.0, 6.0, 8.0, 10.0]
        );
        let out = run("matmul", &[a, b]).unwrap();
        assert!(! out[0].is_borrowed());
        assert_eq!(
            bytes_f32(& out[0].to_host_bytes().unwrap()), vec![6.0, 7.0, 26.0, 31.0]
        );
        assert_eq!(fx.read(), bytes);
    }
    #[test]
    fn concurrent_wrapping() {
        let bytes = sequence();
        let fx = fixture_or_skip!(& bytes);
        let block = &fx.block;
        let expected = &bytes;
        std::thread::scope(|s| {
            for i in 0..8u64 {
                s.spawn(move || {
                    let layout = Layout::contiguous(&[2], DType::F32);
                    let off = (i % 4) * 8;
                    let t = unsafe { wrap_region(block, &region(off, 8), &layout) }
                        .unwrap();
                    let from = off as usize;
                    assert_eq!(
                        t.to_host_bytes().unwrap(), expected[from..from + 8].to_vec()
                    );
                });
            }
        });
    }
}
