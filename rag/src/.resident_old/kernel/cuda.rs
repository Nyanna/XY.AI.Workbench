//! Real CUDA kernels / scorer: NVRTC-compiled sources behind the kernel registry.
#![cfg(feature = "cuda")]
use std::ffi::{c_char, CString};
use std::sync::Arc;
use cudarc::driver::{
    CudaContext, CudaFunction, CudaSlice, CudaStream, DeviceRepr, LaunchConfig,
    PushKernelArg,
};
use cudarc::nvrtc::{
    compile_ptx_with_opts, result as nvrtc_result, sys as nvrtc_sys, CompileOptions, Ptx,
};
use super::registry::{KernelKey, KernelLoader, KernelRegistry};
use super::KernelError;
use crate::resident::memory::DType;
pub const COSINE_DOT: &str = "cosine_dot";
pub const MAXSIM_FUSED: &str = "maxsim_fused";
const PRELUDE: &str = r#"
#if defined(DTYPE_F16)
typedef unsigned short elem_t;
__device__ __forceinline__ float ld(elem_t v) {
    float f;
    asm("cvt.f32.f16 %0, %1;" : "=f"(f) : "h"(v));
    return f;
}
#else
typedef float elem_t;
__device__ __forceinline__ float ld(elem_t v) { return v; }
#endif

__device__ __forceinline__ float warp_sum(float v) {
    for (int o = 16; o > 0; o >>= 1) v += __shfl_down_sync(0xffffffffu, v, o);
    return v;
}
__device__ __forceinline__ float warp_max(float v) {
    for (int o = 16; o > 0; o >>= 1) v = fmaxf(v, __shfl_down_sync(0xffffffffu, v, o));
    return v;
}
"#;
/// One block per slot; scores every query against the slot's single vector. Output is
/// `n_queries x n_slots`.
const COSINE_SRC: &str = r#"
extern "C" __global__ void cosine_dot(
    const elem_t* __restrict__ docs, const float* __restrict__ queries, float* __restrict__ out,
    unsigned n_slots, unsigned n_queries)
{
    unsigned slot = blockIdx.x;
    if (slot >= n_slots) return;
    const elem_t* d = docs + (size_t)slot * DIM;
    __shared__ float part[32];
    unsigned lane = threadIdx.x & 31, warp = threadIdx.x >> 5, warps = (blockDim.x + 31) >> 5;
    for (unsigned q = 0; q < n_queries; ++q) {
        const float* qv = queries + (size_t)q * DIM;
        float acc = 0.f;
        for (unsigned i = threadIdx.x; i < DIM; i += blockDim.x) acc += ld(d[i]) * qv[i];
        acc = warp_sum(acc);
        if (lane == 0) part[warp] = acc;
        __syncthreads();
        if (threadIdx.x == 0) {
            float s = 0.f;
            for (unsigned w = 0; w < warps; ++w) s += part[w];
            out[(size_t)q * n_slots + slot] = s;
        }
        __syncthreads();
    }
}
"#;
/// One block per document, one query. Documents are padded to `doc_tokens`; `doc_len` holds the
/// valid token count. A token without any valid document token scores the f16-safe mask value.
const MAXSIM_SRC: &str = r#"
extern "C" __global__ void maxsim_fused(
    const elem_t* __restrict__ docs, const unsigned* __restrict__ doc_len,
    const float* __restrict__ query, float* __restrict__ out,
    unsigned n_slots, unsigned doc_tokens, unsigned q_tokens)
{
    unsigned slot = blockIdx.x;
    if (slot >= n_slots) return;
    const elem_t* d = docs + (size_t)slot * doc_tokens * DIM;
    unsigned valid = min(doc_len[slot], doc_tokens);
    __shared__ float part[32];
    unsigned lane = threadIdx.x & 31, warp = threadIdx.x >> 5, warps = (blockDim.x + 31) >> 5;
    float total = 0.f;
    for (unsigned t = 0; t < q_tokens; ++t) {
        const float* q = query + (size_t)t * DIM;
        float best = -1e4f;
        for (unsigned j = threadIdx.x; j < valid; j += blockDim.x) {
            const elem_t* row = d + (size_t)j * DIM;
            float acc = 0.f;
            for (unsigned i = 0; i < DIM; ++i) acc += ld(row[i]) * q[i];
            best = fmaxf(best, acc);
        }
        best = warp_max(best);
        if (lane == 0) part[warp] = best;
        __syncthreads();
        if (threadIdx.x == 0) {
            float m = -1e4f;
            for (unsigned w = 0; w < warps; ++w) m = fmaxf(m, part[w]);
            total += m;
        }
        __syncthreads();
    }
    if (threadIdx.x == 0) out[slot] = total;
}
"#;
fn body(name: &str) -> Option<&'static str> {
    match name {
        COSINE_DOT => Some(COSINE_SRC),
        MAXSIM_FUSED => Some(MAXSIM_SRC),
        _ => None,
    }
}
fn defines(key: &KernelKey) -> Result<Vec<String>, KernelError> {
    let ty = match key.dtype {
        DType::F16 => "-DDTYPE_F16",
        DType::F32 => "-DDTYPE_F32",
        other => {
            return Err(KernelError::Unavailable {
                reason: format!("no kernel for dtype {other:?}"),
            });
        }
    };
    Ok(vec![ty.to_string(), format!("-DDIM={}", key.variant)])
}
fn log_of(prog: cudarc::nvrtc::sys::nvrtcProgram) -> String {
    unsafe { nvrtc_result::get_program_log(prog) }
        .map(|raw| {
            let bytes: Vec<u8> = raw
                .iter()
                .take_while(|&&c| c != 0)
                .map(|&c| c as u8)
                .collect();
            String::from_utf8_lossy(&bytes).into_owned()
        })
        .unwrap_or_default()
}
/// Machine code for exactly `sm_<sm>`; usable when the driver cannot JIT the PTX version.
fn compile_cubin(src: &str, sm: u32, opts: &[String]) -> Result<Vec<u8>, KernelError> {
    let c_src = CString::new(src)
        .map_err(|e| KernelError::Compile {
            log: e.to_string(),
        })?;
    let prog = nvrtc_result::create_program(&c_src, None)
        .map_err(|e| KernelError::Compile {
            log: e.to_string(),
        })?;
    let mut options = vec![format!("--gpu-architecture=sm_{sm}")];
    options.extend_from_slice(opts);
    let result = (|| {
        unsafe { nvrtc_result::compile_program(prog, &options) }
            .map_err(|_| KernelError::Compile {
                log: log_of(prog),
            })?;
        let mut size = 0usize;
        unsafe { nvrtc_sys::nvrtcGetCUBINSize(prog, &mut size) }
            .result()
            .map_err(|e| KernelError::Compile {
                log: e.to_string(),
            })?;
        let mut cubin = vec![0u8; size];
        unsafe { nvrtc_sys::nvrtcGetCUBIN(prog, cubin.as_mut_ptr() as *mut c_char) }
            .result()
            .map_err(|e| KernelError::Compile {
                log: e.to_string(),
            })?;
        Ok(cubin)
    })();
    let _ = unsafe { nvrtc_result::destroy_program(prog) };
    result
}
/// Compiles to PTX for `compute_<sm>` and falls back to a CUBIN when the driver rejects the PTX.
pub struct NvrtcLoader {
    ctx: Arc<CudaContext>,
}
impl NvrtcLoader {
    pub fn new(ctx: Arc<CudaContext>) -> Self {
        Self { ctx }
    }
    fn module(
        &self,
        key: &KernelKey,
        src: &str,
    ) -> Result<Arc<cudarc::driver::CudaModule>, KernelError> {
        let opts = defines(key)?;
        let ptx = compile_ptx_with_opts(
                src,
                CompileOptions {
                    arch: Some(arch_for(key.sm)),
                    options: opts.clone(),
                    ..Default::default()
                },
            )
            .map_err(|e| KernelError::Compile {
                log: e.to_string(),
            })?;
        match self.ctx.load_module(ptx) {
            Ok(m) => Ok(m),
            Err(ptx_err) => {
                tracing::warn!(
                    kernel = key.name, error = % ptx_err,
                    "PTX rejected, retrying as CUBIN"
                );
                let cubin = compile_cubin(src, key.sm, &opts)?;
                self.ctx
                    .load_module(Ptx::from_binary(cubin))
                    .map_err(|e| KernelError::ModuleLoad {
                        reason: e.to_string(),
                    })
            }
        }
    }
}
fn arch_for(sm: u32) -> &'static str {
    match sm {
        61 => "compute_61",
        70 => "compute_70",
        75 => "compute_75",
        80 => "compute_80",
        86 => "compute_86",
        89 => "compute_89",
        _ => "compute_75",
    }
}
impl KernelLoader<CudaFunction> for NvrtcLoader {
    fn load(&self, key: &KernelKey) -> Result<CudaFunction, KernelError> {
        let body = body(key.name)
            .ok_or_else(|| KernelError::FuncNotFound {
                name: key.name.into(),
            })?;
        let src = format!("{PRELUDE}{body}");
        let module = self.module(key, &src)?;
        module
            .load_function(key.name)
            .map_err(|_| KernelError::FuncNotFound {
                name: key.name.into(),
            })
    }
}
/// Registry plus the device's compute capability.
pub struct CudaKernels {
    registry: KernelRegistry<CudaFunction>,
    sm: u32,
}
impl CudaKernels {
    pub fn new(ctx: &Arc<CudaContext>) -> Result<Self, KernelError> {
        let (major, minor) = ctx
            .compute_capability()
            .map_err(|e| KernelError::Unavailable {
                reason: e.to_string(),
            })?;
        let loader: Arc<dyn KernelLoader<CudaFunction>> = Arc::new(
            NvrtcLoader::new(ctx.clone()),
        );
        Ok(Self {
            registry: KernelRegistry::new(loader),
            sm: (major * 10 + minor) as u32,
        })
    }
    pub fn sm(&self) -> u32 {
        self.sm
    }
    pub fn registry(&self) -> &KernelRegistry<CudaFunction> {
        &self.registry
    }
    pub fn function(
        &self,
        name: &'static str,
        dtype: DType,
        dim: u32,
    ) -> Result<CudaFunction, KernelError> {
        self.registry.get(&KernelKey::new(name, dtype, self.sm, dim))
    }
}
fn launch_err(e: impl std::error::Error + Send + Sync + 'static) -> KernelError {
    KernelError::LaunchFailed {
        source: Box::new(e),
    }
}
const BLOCK: u32 = 128;
fn cfg(blocks: u32) -> LaunchConfig {
    LaunchConfig {
        grid_dim: (blocks.max(1), 1, 1),
        block_dim: (BLOCK, 1, 1),
        shared_mem_bytes: 0,
    }
}
/// Output layout `n_queries x n_slots`.
pub(crate) fn launch_cosine<T: DeviceRepr>(
    stream: &Arc<CudaStream>,
    f: &CudaFunction,
    docs: &CudaSlice<T>,
    queries: &CudaSlice<f32>,
    out: &mut CudaSlice<f32>,
    n_slots: u32,
    n_queries: u32,
) -> Result<(), KernelError> {
    let mut b = stream.launch_builder(f);
    b.arg(docs).arg(queries).arg(out).arg(&n_slots).arg(&n_queries);
    unsafe { b.launch(cfg(n_slots)) }.map(|_| ()).map_err(launch_err)
}
pub(crate) fn launch_maxsim<T: DeviceRepr>(
    stream: &Arc<CudaStream>,
    f: &CudaFunction,
    docs: &CudaSlice<T>,
    doc_len: &CudaSlice<u32>,
    query: &CudaSlice<f32>,
    out: &mut CudaSlice<f32>,
    n_slots: u32,
    doc_tokens: u32,
    q_tokens: u32,
) -> Result<(), KernelError> {
    let mut b = stream.launch_builder(f);
    b.arg(docs)
        .arg(doc_len)
        .arg(query)
        .arg(out)
        .arg(&n_slots)
        .arg(&doc_tokens)
        .arg(&q_tokens);
    unsafe { b.launch(cfg(n_slots)) }.map(|_| ()).map_err(launch_err)
}
#[cfg(test)]
#[path = "test/cuda.rs"]
mod tests;
