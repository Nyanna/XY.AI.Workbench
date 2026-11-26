//! These tests run on the real card. Without a CUDA device they fail instead of skipping.
use super::super::nvcc_compiler::NvccCompiler;
use super::*;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::a::kernel_register::{CompileRequest, KernelCompiler};
use std::sync::atomic::{AtomicU64, Ordering};
const PROBE: &str = "extern \"C\" __global__ void probe(float* p) { p[0] = 1.0f; }\n";
static DIRS: AtomicU64 = AtomicU64::new(0);
fn loader() -> DriverLoader {
    DriverLoader::new(0).expect("CUDA device 0 is available")
}
fn image(loader: &DriverLoader, kind: CodeKind) -> CodeImage {
    let dir = std::env::temp_dir()
        .join(
            format!(
                "xy_driver_loader_{}_{}", std::process::id(), DIRS.fetch_add(1,
                Ordering::SeqCst)
            ),
        );
    let compiler = NvccCompiler::discover(&dir)
        .unwrap_or_else(|| NvccCompiler::new("/usr/local/cuda-13.3/bin/nvcc", &dir));
    let arch = loader.device_capability().unwrap().jit_arch();
    let image = compiler
        .compile(
            &CompileRequest {
                file_name: "probe.cu".to_string(),
                source: PROBE.to_string(),
                arch,
                kind,
            },
        )
        .unwrap();
    let _ = std::fs::remove_dir_all(&dir);
    image
}
#[test]
fn capability_matches_the_device() {
    let loader = loader();
    let (major, minor) = loader.context().compute_capability().unwrap();
    let cc = loader.device_capability().unwrap();
    assert_eq!((cc.major, cc.minor), (major as u32, minor as u32));
    assert!(cc.major >= 5);
}
#[test]
fn unknown_device_is_an_error() {
    assert!(DriverLoader::new(99).is_err());
}
#[test]
fn ptx_loads_and_resolves_the_entry() {
    let loader = loader();
    let function = loader.load(&image(&loader, CodeKind::Ptx), "probe").unwrap();
    assert_eq!(function.symbol(), "probe");
    assert!(function.as_any().downcast_ref::< DriverFunction > ().is_some());
}
#[test]
fn cubin_loads_and_resolves_the_entry() {
    let loader = loader();
    let function = loader.load(&image(&loader, CodeKind::Cubin), "probe").unwrap();
    assert_eq!(function.symbol(), "probe");
}
#[test]
fn the_same_image_can_be_loaded_twice() {
    let loader = loader();
    let image = image(&loader, CodeKind::Ptx);
    let first = loader.load(&image, "probe").unwrap();
    let second = loader.load(&image, "probe").unwrap();
    let (a, b) = (
        first.as_any().downcast_ref::<DriverFunction>().unwrap(),
        second.as_any().downcast_ref::<DriverFunction>().unwrap(),
    );
    assert_ne!(a.raw(), b.raw());
}
#[test]
fn unknown_symbol_is_reported() {
    let loader = loader();
    let result = loader.load(&image(&loader, CodeKind::Ptx), "nope");
    assert!(matches!(result, Err(LoadFailure::MissingSymbol(name)) if name == "nope"));
}
#[test]
fn symbol_with_interior_nul_is_rejected() {
    let loader = loader();
    let result = loader.load(&image(&loader, CodeKind::Ptx), "pro\0be");
    assert!(matches!(result, Err(LoadFailure::Other(_))));
}
#[test]
fn invalid_ptx_is_rejected() {
    let image = CodeImage {
        kind: CodeKind::Ptx,
        bytes: b"not ptx".to_vec(),
    };
    assert!(matches!(loader().load(& image, "probe"), Err(LoadFailure::PtxRejected(_))));
}
#[test]
fn invalid_cubin_is_a_plain_load_failure() {
    let image = CodeImage {
        kind: CodeKind::Cubin,
        bytes: b"not a cubin".to_vec(),
    };
    assert!(matches!(loader().load(& image, "probe"), Err(LoadFailure::Other(_))));
}
#[test]
fn function_outlives_the_loader() {
    let function = {
        let loader = loader();
        loader.load(&image(&loader, CodeKind::Ptx), "probe").unwrap()
    };
    assert_eq!(function.symbol(), "probe");
}
