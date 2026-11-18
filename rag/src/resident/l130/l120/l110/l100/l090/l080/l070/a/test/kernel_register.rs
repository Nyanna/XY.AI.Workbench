use super::*;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex as StdMutex;
use std::time::Duration;
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
type CompileFn = dyn Fn(
    &CompileRequest,
) -> Result<CodeImage, CompilerError> + Send + Sync;
type LoadFn = dyn Fn(&CodeImage, &str) -> Result<(), LoadFailure> + Send + Sync;
struct MockCompiler {
    calls: AtomicUsize,
    requests: StdMutex<Vec<CompileRequest>>,
    f: Box<CompileFn>,
}
impl KernelCompiler for MockCompiler {
    fn compile(&self, req: &CompileRequest) -> Result<CodeImage, CompilerError> {
        self.calls.fetch_add(1, Ordering::SeqCst);
        self.requests.lock().unwrap().push(req.clone());
        (self.f)(req)
    }
}
struct MockLoader {
    cc: Option<ComputeCapability>,
    calls: AtomicUsize,
    f: Box<LoadFn>,
}
impl ModuleLoader for MockLoader {
    fn device_capability(&self) -> Option<ComputeCapability> {
        self.cc
    }
    fn load(
        &self,
        image: &CodeImage,
        symbol: &str,
    ) -> Result<Arc<dyn KernelFunction>, LoadFailure> {
        self.calls.fetch_add(1, Ordering::SeqCst);
        (self.f)(image, symbol)?;
        Ok(Arc::new(Func(symbol.to_string())))
    }
}
fn ok_image(req: &CompileRequest) -> Result<CodeImage, CompilerError> {
    Ok(CodeImage {
        kind: req.kind,
        bytes: vec![1, 2, 3],
    })
}
static ALL: &[DType] = &[DType::F32, DType::F16, DType::U8];
static PRE: &[(ArchLevel, &[u8])] = &[(ArchLevel::Sm75, &[9, 9])];
fn source(name: &'static str) -> KernelSource {
    KernelSource {
        name,
        body: "extern \"C\" __global__ void k() {}",
        entry: "k",
        dtypes: ALL,
        precompiled: None,
    }
}
struct Fixture {
    reg: Arc<KernelRegister>,
    compiler: Arc<MockCompiler>,
    loader: Arc<MockLoader>,
}
fn fixture_with(
    device: DeviceHandle,
    cc: Option<ComputeCapability>,
    c: Box<CompileFn>,
    l: Box<LoadFn>,
) -> Fixture {
    let compiler = Arc::new(MockCompiler {
        calls: AtomicUsize::new(0),
        requests: StdMutex::new(Vec::new()),
        f: c,
    });
    let loader = Arc::new(MockLoader {
        cc,
        calls: AtomicUsize::new(0),
        f: l,
    });
    let reg = Arc::new(
        KernelRegister::new(
            device,
            compiler.clone(),
            loader.clone(),
            crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::a::library_adapter::library_kernel_source(),
        ),
    );
    Fixture { reg, compiler, loader }
}
fn fixture() -> Fixture {
    fixture_with(
        DeviceHandle::Gpu(0),
        Some(ComputeCapability::new(7, 5)),
        Box::new(ok_image),
        Box::new(|_, _| Ok(())),
    )
}
fn key(name: &str) -> KernelKey {
    KernelKey {
        name: name.to_string(),
        dtype: DType::F32,
        arch: ArchLevel::Sm75,
        variant: Variant::default(),
    }
}
fn gpu(k: Kernel) -> Arc<GpuKernel> {
    match k {
        Kernel::Gpu(g) => g,
        Kernel::Library(_) => panic!("expected gpu kernel"),
    }
}
#[test]
fn arch_mapping() {
    let cases = [
        ((6, 1), ArchLevel::Sm61),
        ((7, 0), ArchLevel::Sm70),
        ((7, 5), ArchLevel::Sm75),
        ((8, 0), ArchLevel::Sm80),
        ((8, 6), ArchLevel::Sm86),
        ((8, 9), ArchLevel::Sm89),
        ((9, 0), ArchLevel::Sm75),
        ((6, 0), ArchLevel::Sm75),
        ((7, 2), ArchLevel::Sm75),
    ];
    for ((ma, mi), lvl) in cases {
        assert_eq!(arch_level_for(ComputeCapability::new(ma, mi)), lvl);
    }
    assert_eq!(ArchLevel::Sm61.jit_arch(), "sm_61");
    assert_eq!(ArchLevel::Sm89.jit_arch(), "sm_89");
    assert_eq!(ArchLevel::Sm86.capability(), ComputeCapability::new(8, 6));
}
#[test]
fn capability_only_on_gpu_device() {
    let f = fixture();
    assert_eq!(f.reg.capability(), Some(ComputeCapability::new(7, 5)));
    assert_eq!(f.reg.target_arch(), Some(ArchLevel::Sm75));
    assert_eq!(f.reg.device(), DeviceHandle::Gpu(0));
    let cpu = fixture_with(
        DeviceHandle::Cpu,
        Some(ComputeCapability::new(7, 5)),
        Box::new(ok_image),
        Box::new(|_, _| Ok(())),
    );
    assert_eq!(cpu.reg.capability(), None);
    assert_eq!(cpu.reg.target_arch(), None);
}
#[test]
fn compile_cache_and_identity() {
    let f = fixture();
    f.reg.register_source(source("scale")).unwrap();
    let a = gpu(f.reg.get(&key("scale")).unwrap());
    let b = gpu(f.reg.get(&key("scale")).unwrap());
    assert!(Arc::ptr_eq(& a, & b));
    assert_eq!(a.key, key("scale"));
    assert_eq!(a.image_kind, CodeKind::Ptx);
    assert_eq!(f.compiler.calls.load(Ordering::SeqCst), 1);
    assert_eq!(f.reg.len(), 1);
    assert!(! f.reg.is_empty());
    let mut k2 = key("scale");
    k2.variant = Variant {
        dim: Some(384),
        tile: Some(16),
    };
    let c = gpu(f.reg.get(&k2).unwrap());
    assert!(! Arc::ptr_eq(& a, & c));
    assert_eq!(f.reg.len(), 2);
}
#[test]
fn source_text_and_file_name() {
    let f = fixture();
    f.reg.register_source(source("scale")).unwrap();
    let mut k = key("scale");
    k.dtype = DType::F16;
    k.variant = Variant {
        dim: Some(384),
        tile: Some(16),
    };
    f.reg.get(&k).unwrap();
    let req = f.compiler.requests.lock().unwrap()[0].clone();
    assert_eq!(req.file_name, "scale_f16_sm_75_d384_t16.cu");
    assert_eq!(req.arch, "sm_75");
    assert_eq!(req.kind, CodeKind::Ptx);
    assert!(req.source.starts_with("#include <cuda_fp16.h>"));
    assert!(req.source.contains("typedef __half T;"));
    assert!(req.source.contains("#define DIM 384"));
    assert!(req.source.contains("#define TILE 16"));
    assert!(req.source.contains("warp_reduce_sum"));
    assert!(req.source.contains("warp_reduce_max"));
    assert!(req.source.ends_with("void k() {}"));
    f.reg.get(&key("scale")).unwrap();
    let req = f.compiler.requests.lock().unwrap()[1].clone();
    assert_eq!(req.file_name, "scale_f32_sm_75.cu");
    assert!(! req.source.contains("#define DIM"));
    assert!(! req.source.contains("#define TILE"));
    assert!(req.source.contains("typedef float T;"));
}
#[test]
fn dtype_checks_precede_compilation() {
    let f = fixture();
    f.reg.register_source(source("scale")).unwrap();
    for d in [DType::BF16, DType::I64] {
        let mut k = key("scale");
        k.dtype = d;
        assert!(matches!(f.reg.get(& k), Err(KernelError::Unavailable(_))));
    }
    assert_eq!(f.compiler.calls.load(Ordering::SeqCst), 0);
}
#[test]
fn library_fallback_and_unknown() {
    let f = fixture();
    match f.reg.get(&key("matmul")).unwrap() {
        Kernel::Library(e) => assert_eq!(e.name, "matmul"),
        _ => panic!("expected library kernel"),
    }
    assert_eq!(f.reg.len(), 0);
    assert_eq!(
        f.reg.get(& key("nope")).unwrap_err(), KernelError::UnknownKernel("nope"
        .to_string())
    );
    f.reg.register_source(source("add")).unwrap();
    assert!(matches!(f.reg.get(& key("add")).unwrap(), Kernel::Gpu(_)));
}
#[test]
fn no_capability_blocks_jit_only() {
    let f = fixture_with(
        DeviceHandle::Cpu,
        None,
        Box::new(ok_image),
        Box::new(|_, _| Ok(())),
    );
    f.reg.register_source(source("scale")).unwrap();
    assert!(
        matches!(f.reg.request("scale", DType::F32, Variant::default()),
        Err(KernelError::Unavailable(_)))
    );
    assert!(
        matches!(f.reg.request("softmax_last_dim", DType::F32, Variant::default()),
        Ok(Kernel::Library(_)))
    );
}
#[test]
fn request_uses_target_arch() {
    let f = fixture_with(
        DeviceHandle::Gpu(0),
        Some(ComputeCapability::new(8, 6)),
        Box::new(ok_image),
        Box::new(|_, _| Ok(())),
    );
    f.reg.register_source(source("scale")).unwrap();
    let k = gpu(f.reg.request("scale", DType::F32, Variant::default()).unwrap());
    assert_eq!(k.key.arch, ArchLevel::Sm86);
}
#[test]
fn duplicate_source_rejected() {
    let f = fixture();
    f.reg.register_source(source("scale")).unwrap();
    assert!(
        matches!(f.reg.register_source(source("scale")),
        Err(KernelError::Unavailable(_)))
    );
}
#[test]
fn ptx_rejected_falls_back_to_cubin_for_real_cc() {
    let f = fixture_with(
        DeviceHandle::Gpu(0),
        Some(ComputeCapability::new(7, 2)),
        Box::new(ok_image),
        Box::new(|img, _| {
            if img.kind == CodeKind::Ptx {
                Err(LoadFailure::PtxRejected("too new".into()))
            } else {
                Ok(())
            }
        }),
    );
    f.reg.register_source(source("scale")).unwrap();
    let k = gpu(f.reg.request("scale", DType::F32, Variant::default()).unwrap());
    assert_eq!(k.image_kind, CodeKind::Cubin);
    let reqs = f.compiler.requests.lock().unwrap();
    assert_eq!(reqs.len(), 2);
    assert_eq!(reqs[0].arch, "sm_75");
    assert_eq!(reqs[1].arch, "sm_72");
    assert_eq!(reqs[1].kind, CodeKind::Cubin);
}
#[test]
fn cubin_fallback_failure_is_load_error_and_not_cached() {
    let f = fixture_with(
        DeviceHandle::Gpu(0),
        Some(ComputeCapability::new(7, 5)),
        Box::new(ok_image),
        Box::new(|_, _| Err(LoadFailure::PtxRejected("no".into()))),
    );
    f.reg.register_source(source("scale")).unwrap();
    let e = f.reg.get(&key("scale")).unwrap_err();
    assert!(matches!(e, KernelError::ModuleLoadError(_)));
    assert_eq!(f.reg.len(), 0);
    let _ = f.reg.get(&key("scale"));
    assert_eq!(f.compiler.calls.load(Ordering::SeqCst), 4);
}
#[test]
fn compiler_unavailable_uses_precompiled() {
    let f = fixture_with(
        DeviceHandle::Gpu(0),
        Some(ComputeCapability::new(7, 5)),
        Box::new(|_| Err(CompilerError::Unavailable("no nvrtc".into()))),
        Box::new(|img, _| {
            assert_eq!(img.kind, CodeKind::Cubin);
            assert_eq!(img.bytes, vec![9, 9]);
            Ok(())
        }),
    );
    let mut with = source("pre");
    with.precompiled = Some(PRE);
    f.reg.register_source(with).unwrap();
    f.reg.register_source(source("bare")).unwrap();
    let k = gpu(f.reg.get(&key("pre")).unwrap());
    assert_eq!(k.image_kind, CodeKind::Cubin);
    assert_eq!(
        f.reg.get(& key("bare")).unwrap_err(), KernelError::ModuleLoadError("no nvrtc"
        .into())
    );
}
#[test]
fn compile_failure_carries_log_without_fallback() {
    let f = fixture_with(
        DeviceHandle::Gpu(0),
        Some(ComputeCapability::new(7, 5)),
        Box::new(|_| Err(CompilerError::Failed {
            log: "syntax".into(),
        })),
        Box::new(|_, _| Ok(())),
    );
    f.reg.register_source(source("scale")).unwrap();
    assert_eq!(
        f.reg.get(& key("scale")).unwrap_err(), KernelError::CompileError { log :
        "syntax".into() }
    );
    assert_eq!(f.compiler.calls.load(Ordering::SeqCst), 1);
    assert_eq!(f.loader.calls.load(Ordering::SeqCst), 0);
}
#[test]
fn load_failure_mapping() {
    let f = fixture_with(
        DeviceHandle::Gpu(0),
        Some(ComputeCapability::new(7, 5)),
        Box::new(ok_image),
        Box::new(|_, sym| {
            if sym == "k" {
                Err(LoadFailure::MissingSymbol(sym.into()))
            } else {
                Err(LoadFailure::Other("boom".into()))
            }
        }),
    );
    f.reg.register_source(source("a")).unwrap();
    assert_eq!(
        f.reg.get(& key("a")).unwrap_err(), KernelError::MissingFunction { kernel : "a"
        .into(), symbol : "k".into() }
    );
    let mut other = source("b");
    other.entry = "z";
    f.reg.register_source(other).unwrap();
    assert_eq!(
        f.reg.get(& key("b")).unwrap_err(), KernelError::ModuleLoadError("boom".into())
    );
}
#[test]
fn panics_become_errors_and_do_not_wedge() {
    let f = fixture_with(
        DeviceHandle::Gpu(0),
        Some(ComputeCapability::new(7, 5)),
        Box::new(|_| panic!("compiler exploded")),
        Box::new(|_, _| panic!("loader exploded")),
    );
    f.reg.register_source(source("scale")).unwrap();
    match f.reg.get(&key("scale")).unwrap_err() {
        KernelError::CompileError { log } => assert!(log.contains("compiler exploded")),
        e => panic!("unexpected {e:?}"),
    }
    assert!(f.reg.get(& key("scale")).is_err());
    let g = fixture_with(
        DeviceHandle::Gpu(0),
        Some(ComputeCapability::new(7, 5)),
        Box::new(ok_image),
        Box::new(|_, _| panic!("loader exploded")),
    );
    g.reg.register_source(source("scale")).unwrap();
    match g.reg.get(&key("scale")).unwrap_err() {
        KernelError::ModuleLoadError(t) => assert!(t.contains("loader exploded")),
        e => panic!("unexpected {e:?}"),
    }
}
#[test]
fn clear_drops_entries_and_recompiles() {
    let f = fixture();
    f.reg.register_source(source("scale")).unwrap();
    let first = gpu(f.reg.get(&key("scale")).unwrap());
    assert_eq!(f.reg.clear(), 1);
    assert_eq!(f.reg.clear(), 0);
    assert!(f.reg.is_empty());
    assert_eq!(first.key, key("scale"));
    let second = gpu(f.reg.get(&key("scale")).unwrap());
    assert!(! Arc::ptr_eq(& first, & second));
    assert_eq!(f.compiler.calls.load(Ordering::SeqCst), 2);
}
#[test]
fn concurrent_requests_single_flight() {
    let f = fixture_with(
        DeviceHandle::Gpu(0),
        Some(ComputeCapability::new(7, 5)),
        Box::new(|r| {
            std::thread::sleep(Duration::from_millis(100));
            ok_image(r)
        }),
        Box::new(|_, _| Ok(())),
    );
    f.reg.register_source(source("scale")).unwrap();
    let handles: Vec<_> = (0..8)
        .map(|_| {
            let reg = f.reg.clone();
            std::thread::spawn(move || gpu(reg.get(&key("scale")).unwrap()))
        })
        .collect();
    let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();
    assert_eq!(f.compiler.calls.load(Ordering::SeqCst), 1);
    assert_eq!(f.loader.calls.load(Ordering::SeqCst), 1);
    for r in &results[1..] {
        assert!(Arc::ptr_eq(& results[0], r));
    }
}
#[test]
fn concurrent_failure_shared() {
    let f = fixture_with(
        DeviceHandle::Gpu(0),
        Some(ComputeCapability::new(7, 5)),
        Box::new(|_| {
            std::thread::sleep(Duration::from_millis(100));
            Err(CompilerError::Failed {
                log: "bad".into(),
            })
        }),
        Box::new(|_, _| Ok(())),
    );
    f.reg.register_source(source("scale")).unwrap();
    let handles: Vec<_> = (0..4)
        .map(|_| {
            let reg = f.reg.clone();
            std::thread::spawn(move || reg.get(&key("scale")).unwrap_err())
        })
        .collect();
    for h in handles {
        assert_eq!(h.join().unwrap(), KernelError::CompileError { log : "bad".into() });
    }
    assert_eq!(f.compiler.calls.load(Ordering::SeqCst), 1);
}
#[test]
fn resource_registration() {
    use std::time::Duration as D;
    let registry: ResourceRegistry<KernelRegister> = ResourceRegistry::new(
        crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::a::resource_lifecycle::LifecycleConfig::new(
                D::from_secs(1),
                D::from_secs(2),
                D::from_millis(10),
            )
            .unwrap(),
    );
    let created = Arc::new(AtomicUsize::new(0));
    let c = created.clone();
    let factory: Arc<dyn Fn() -> Result<KernelRegister, String> + Send + Sync> = Arc::new(move ||
    {
        c.fetch_add(1, Ordering::SeqCst);
        Err("no device".to_string())
    });
    register_resource(&registry, factory.clone()).unwrap();
    assert_eq!(created.load(Ordering::SeqCst), 0);
    assert!(
        matches!(register_resource(& registry, factory),
        Err(LifecycleError::AlreadyRegistered(_)))
    );
}
