//! Compiles kernel sources into PTX or cubin images with the external `nvcc`.

use std::path::PathBuf;
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};

use crate::resident::l130::l120::l110::l100::l090 as l090;
use l090::l080 as l080;
use l080::l070 as l070;

use l070::a::kernel_register::{
    CodeImage, CodeKind, CompileRequest, CompilerError, KernelCompiler,
};

static NEXT_JOB: AtomicU64 = AtomicU64::new(0);

#[derive(Debug, Clone)]
pub struct NvccCompiler {
    nvcc: PathBuf,
    work_dir: PathBuf,
}

struct Scratch(Vec<PathBuf>);

impl Drop for Scratch {
    fn drop(&mut self) {
        for path in &self.0 {
            let _ = std::fs::remove_file(path);
        }
    }
}

fn unavailable(e: impl ToString) -> CompilerError {
    CompilerError::Unavailable(e.to_string())
}

impl NvccCompiler {
    pub fn new(nvcc: impl Into<PathBuf>, work_dir: impl Into<PathBuf>) -> Self {
        NvccCompiler {
            nvcc: nvcc.into(),
            work_dir: work_dir.into(),
        }
    }

    /// Looks for `nvcc` under `CUDA_HOME`, then `CUDA_PATH`, then on the `PATH`.
    pub fn discover(work_dir: impl Into<PathBuf>) -> Option<Self> {
        let from_env = ["CUDA_HOME", "CUDA_PATH"]
            .iter()
            .filter_map(std::env::var_os)
            .map(|root| PathBuf::from(root).join("bin").join("nvcc"))
            .find(|path| path.is_file());
        let nvcc = from_env
            .or_else(|| {
                Command::new("nvcc")
                    .arg("--version")
                    .output()
                    .ok()
                    .filter(|o| o.status.success())
                    .map(|_| PathBuf::from("nvcc"))
            })?;
        Some(NvccCompiler::new(nvcc, work_dir))
    }
}

impl KernelCompiler for NvccCompiler {
    fn compile(&self, req: &CompileRequest) -> Result<CodeImage, CompilerError> {
        let name = &req.file_name;
        if name.is_empty() || name == "." || name == ".." || name.contains(['/', '\\']) {
            return Err(CompilerError::Failed {
                log: format!("invalid source file name '{name}'"),
            });
        }
        let (mode, ext, arch) = match req.kind {
            CodeKind::Ptx => ("-ptx", "ptx", req.arch.replacen("sm_", "compute_", 1)),
            CodeKind::Cubin => ("-cubin", "cubin", req.arch.clone()),
        };
        std::fs::create_dir_all(&self.work_dir).map_err(unavailable)?;
        let job = format!(
            "{}_{}_{name}",
            std::process::id(),
            NEXT_JOB.fetch_add(1, Ordering::Relaxed)
        );
        let src = self.work_dir.join(&job);
        let out = self.work_dir.join(format!("{job}.{ext}"));
        let _scratch = Scratch(vec![src.clone(), out.clone()]);
        std::fs::write(&src, &req.source).map_err(unavailable)?;
        let output = Command::new(&self.nvcc)
            .args(["-x", "cu", mode])
            .arg(format!("-arch={arch}"))
            .arg("-o")
            .arg(&out)
            .arg(&src)
            .output()
            .map_err(unavailable)?;
        if !output.status.success() {
            let mut log = String::from_utf8_lossy(&output.stderr).into_owned();
            if log.trim().is_empty() {
                log = String::from_utf8_lossy(&output.stdout).into_owned();
            }
            return Err(CompilerError::Failed { log });
        }
        let bytes = std::fs::read(&out).map_err(unavailable)?;
        Ok(CodeImage {
            kind: req.kind,
            bytes,
        })
    }
}

#[cfg(test)]
#[path = "test/nvcc_compiler.rs"]
mod tests;
