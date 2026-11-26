use super::*;
use std::any::Any;
use std::sync::{Arc, Mutex};
use crate::resident::_d::_c::_b::_a::_9::_8::_7::a::kernel_register::{
    CodeImage, CompileRequest, CompilerError, Kernel, KernelCompiler, KernelFunction,
    LoadFailure, ModuleLoader, Variant,
};
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::a::library_adapter::library_kernel_source;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::_1::{
    device_lane_event::DeviceHandle, target_env::ComputeCapability,
};
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
}
impl KernelCompiler for MockCompiler {
    fn compile(&self, req: &CompileRequest) -> Result<CodeImage, CompilerError> {
        self.requests.lock().unwrap().push(req.clone());
        Ok(CodeImage {
            kind: req.kind,
            bytes: vec![0],
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
        _: &CodeImage,
        symbol: &str,
    ) -> Result<Arc<dyn KernelFunction>, LoadFailure> {
        Ok(Arc::new(Func(symbol.to_string())))
    }
}
struct Env {
    register: Arc<KernelRegister>,
    compiler: Arc<MockCompiler>,
}
fn env_with(register_kernels: bool) -> Env {
    let compiler = Arc::new(MockCompiler {
        requests: Mutex::new(Vec::new()),
    });
    let loader = Arc::new(MockLoader {
        cc: Some(ComputeCapability::new(7, 5)),
    });
    let register = Arc::new(
        KernelRegister::new(
            DeviceHandle::Gpu(0),
            compiler.clone(),
            loader,
            library_kernel_source(),
        ),
    );
    if register_kernels {
        register_score_kernels(&register).unwrap();
    }
    Env { register, compiler }
}
fn env() -> Env {
    env_with(true)
}
fn variant(dim: u32) -> Variant {
    Variant {
        dim: Some(dim),
        tile: None,
    }
}
#[test]
fn metric_kernel_names() {
    assert_eq!(Metric::Dot.kernel_name(), "score_dot");
    assert_eq!(Metric::NegL2Sq.kernel_name(), "score_negl2");
}
#[test]
fn register_adds_two_sources_and_rejects_a_second_call() {
    let e = env_with(false);
    register_score_kernels(&e.register).unwrap();
    for name in [KERNEL_SCORE_DOT, KERNEL_SCORE_NEGL2] {
        for dtype in [DType::F32, DType::F16] {
            assert!(
                matches!(e.register.request(name, dtype, variant(8)), Ok(Kernel::Gpu(_)))
            );
        }
        assert!(
            matches!(e.register.request(name, DType::BF16, variant(8)),
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
    let e = env();
    e.register.request(KERNEL_SCORE_NEGL2, DType::F32, variant(4)).unwrap();
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
    for (metric, dtype, dim) in [
        (Metric::Dot, DType::F32, 384u32),
        (Metric::Dot, DType::F16, 384),
        (Metric::NegL2Sq, DType::F32, 33),
        (Metric::NegL2Sq, DType::F16, 1),
    ] {
        let e = env();
        e.register.request(metric.kernel_name(), dtype, variant(dim)).unwrap();
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
                assert!(
                    o.status.success(), "nvcc failed for {}: {}", src.display(),
                    String::from_utf8_lossy(& o.stderr)
                );
            }
        }
    }
    let _ = std::fs::remove_dir_all(&dir);
}
