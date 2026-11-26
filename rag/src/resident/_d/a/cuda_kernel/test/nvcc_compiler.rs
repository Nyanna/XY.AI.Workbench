//! These tests run the real `nvcc`. Without it they fail instead of skipping.
use super::*;
use std::path::Path;
const PROBE: &str = "extern \"C\" __global__ void probe(float* p) { p[0] = 1.0f; }\n";
static DIRS: AtomicU64 = AtomicU64::new(0);
fn work_dir() -> PathBuf {
    std::env::temp_dir()
        .join(
            format!(
                "xy_nvcc_compiler_{}_{}", std::process::id(), DIRS.fetch_add(1,
                Ordering::SeqCst)
            ),
        )
}
fn compiler(dir: &Path) -> NvccCompiler {
    NvccCompiler::discover(dir)
        .unwrap_or_else(|| NvccCompiler::new("/usr/local/cuda-13.3/bin/nvcc", dir))
}
fn request(kind: CodeKind) -> CompileRequest {
    CompileRequest {
        file_name: "probe.cu".to_string(),
        source: PROBE.to_string(),
        arch: "sm_75".to_string(),
        kind,
    }
}
fn entries(dir: &Path) -> usize {
    std::fs::read_dir(dir).unwrap().count()
}
#[test]
fn ptx_image_contains_the_entry() {
    let dir = work_dir();
    let image = compiler(&dir).compile(&request(CodeKind::Ptx)).unwrap();
    assert_eq!(image.kind, CodeKind::Ptx);
    let text = String::from_utf8(image.bytes).unwrap();
    assert!(text.contains(".entry probe"), "{text}");
    assert!(text.contains(".target sm_75"), "{text}");
    let _ = std::fs::remove_dir_all(&dir);
}
#[test]
fn cubin_image_is_an_elf_file() {
    let dir = work_dir();
    let image = compiler(&dir).compile(&request(CodeKind::Cubin)).unwrap();
    assert_eq!(image.kind, CodeKind::Cubin);
    assert_eq!(& image.bytes[..4], & [0x7f, b'E', b'L', b'F']);
    let _ = std::fs::remove_dir_all(&dir);
}
#[test]
fn source_errors_are_reported_as_failures() {
    let dir = work_dir();
    let mut req = request(CodeKind::Ptx);
    req.source = "this is not cuda".to_string();
    assert!(matches!(compiler(& dir).compile(& req), Err(CompilerError::Failed { .. })));
    let _ = std::fs::remove_dir_all(&dir);
}
#[test]
fn missing_compiler_is_unavailable() {
    let dir = work_dir();
    let missing = NvccCompiler::new("/nonexistent/bin/nvcc", &dir);
    assert!(
        matches!(missing.compile(& request(CodeKind::Ptx)),
        Err(CompilerError::Unavailable(_)))
    );
    let _ = std::fs::remove_dir_all(&dir);
}
#[test]
fn unusable_work_dir_is_unavailable() {
    let file = std::env::temp_dir()
        .join(
            format!(
                "xy_nvcc_compiler_file_{}_{}", std::process::id(), DIRS.fetch_add(1,
                Ordering::SeqCst)
            ),
        );
    std::fs::write(&file, b"x").unwrap();
    let result = NvccCompiler::new("nvcc", &file).compile(&request(CodeKind::Ptx));
    assert!(matches!(result, Err(CompilerError::Unavailable(_))));
    let _ = std::fs::remove_file(&file);
}
#[test]
fn file_names_with_paths_are_rejected() {
    let dir = work_dir();
    let compiler = compiler(&dir);
    for name in ["", ".", "..", "../probe.cu", "a/probe.cu", "a\\probe.cu"] {
        let mut req = request(CodeKind::Ptx);
        req.file_name = name.to_string();
        assert!(
            matches!(compiler.compile(& req), Err(CompilerError::Failed { .. })),
            "name {name:?}"
        );
    }
    assert!(! dir.exists());
}
#[test]
fn work_files_are_removed_after_success_and_failure() {
    let dir = work_dir();
    let compiler = compiler(&dir);
    assert!(! dir.exists());
    compiler.compile(&request(CodeKind::Ptx)).unwrap();
    assert_eq!(entries(& dir), 0);
    let mut bad = request(CodeKind::Ptx);
    bad.source = "broken".to_string();
    assert!(compiler.compile(& bad).is_err());
    assert_eq!(entries(& dir), 0);
    let _ = std::fs::remove_dir_all(&dir);
}
#[test]
fn parallel_compiles_of_one_file_name_do_not_clash() {
    let dir = work_dir();
    let compiler = compiler(&dir);
    let images: Vec<CodeImage> = std::thread::scope(|scope| {
        let handles: Vec<_> = (0..4)
            .map(|_| scope.spawn(|| compiler.compile(&request(CodeKind::Ptx)).unwrap()))
            .collect();
        handles.into_iter().map(|h| h.join().unwrap()).collect()
    });
    assert!(images.iter().all(| i | i.bytes == images[0].bytes));
    assert_eq!(entries(& dir), 0);
    let _ = std::fs::remove_dir_all(&dir);
}
