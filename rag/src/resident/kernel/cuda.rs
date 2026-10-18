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
mod tests {
    use super::*;
    use crate::resident::kernel::cpu_simd::{dot, maxsim};
    use crate::resident::subengine::DeviceHandle;
    use half::f16;
    const DIM: usize = 64;
    fn rnd(n: usize, seed: u32) -> Vec<f32> {
        (0..n)
            .map(|i| {
                ((i as u32).wrapping_mul(2654435761).wrapping_add(seed.wrapping_mul(97))
                    % 1000) as f32 / 500.0 - 1.0
            })
            .collect()
    }
    fn setup() -> (Arc<CudaContext>, Arc<CudaStream>, CudaKernels) {
        let dev = DeviceHandle::cuda(0).expect("cuda device");
        let ctx = dev.cuda_context().unwrap().clone();
        let stream = ctx.new_stream().unwrap();
        let kernels = CudaKernels::new(&ctx).unwrap();
        (ctx, stream, kernels)
    }
    #[test]
    #[ignore = "requires a CUDA device"]
    fn registry_compiles_for_device_sm_and_caches() {
        let (_, _, k) = setup();
        assert!(k.sm() >= 75);
        k.function(COSINE_DOT, DType::F16, DIM as u32).unwrap();
        k.function(COSINE_DOT, DType::F16, DIM as u32).unwrap();
        k.function(MAXSIM_FUSED, DType::F32, DIM as u32).unwrap();
        assert_eq!(k.registry().len(), 2);
        assert!(
            matches!(k.function("nope", DType::F32, 1), Err(KernelError::FuncNotFound {
            .. }))
        );
    }
    #[test]
    #[ignore = "requires a CUDA device"]
    fn cubin_path_loads_and_runs() {
        let (ctx, stream, k) = setup();
        let key = KernelKey::new(COSINE_DOT, DType::F32, k.sm(), DIM as u32);
        let src = format!("{PRELUDE}{COSINE_SRC}");
        let cubin = compile_cubin(&src, k.sm(), &defines(&key).unwrap()).unwrap();
        let f = ctx
            .load_module(Ptx::from_binary(cubin))
            .unwrap()
            .load_function(COSINE_DOT)
            .unwrap();
        let docs = rnd(3 * DIM, 1);
        let q = rnd(DIM, 2);
        let d_docs = stream.clone_htod(&docs).unwrap();
        let d_q = stream.clone_htod(&q).unwrap();
        let mut d_out = stream.alloc_zeros::<f32>(3).unwrap();
        launch_cosine(&stream, &f, &d_docs, &d_q, &mut d_out, 3, 1).unwrap();
        let got = stream.clone_dtoh(&d_out).unwrap();
        for s in 0..3 {
            let want = dot(&q, &docs[s * DIM..(s + 1) * DIM]);
            assert!((got[s] - want).abs() < 1e-3);
        }
    }
    #[test]
    #[ignore = "requires a CUDA device"]
    fn cosine_kernel_matches_cpu_for_f32_and_f16() {
        let (_, stream, k) = setup();
        let (n, nq) = (37usize, 3usize);
        let docs = rnd(n * DIM, 3);
        let queries = rnd(nq * DIM, 4);
        let d_q = stream.clone_htod(&queries).unwrap();
        let f32_fn = k.function(COSINE_DOT, DType::F32, DIM as u32).unwrap();
        let d_docs = stream.clone_htod(&docs).unwrap();
        let mut out = stream.alloc_zeros::<f32>(n * nq).unwrap();
        launch_cosine(&stream, &f32_fn, &d_docs, &d_q, &mut out, n as u32, nq as u32)
            .unwrap();
        let got = stream.clone_dtoh(&out).unwrap();
        for q in 0..nq {
            for s in 0..n {
                let want = dot(
                    &queries[q * DIM..(q + 1) * DIM],
                    &docs[s * DIM..(s + 1) * DIM],
                );
                assert!((got[q * n + s] - want).abs() < 1e-3, "f32 q={q} s={s}");
            }
        }
        let f16_fn = k.function(COSINE_DOT, DType::F16, DIM as u32).unwrap();
        let bits: Vec<u16> = docs.iter().map(|&v| f16::from_f32(v).to_bits()).collect();
        let rounded: Vec<f32> = bits
            .iter()
            .map(|&b| f16::from_bits(b).to_f32())
            .collect();
        let d_docs16 = stream.clone_htod(&bits).unwrap();
        let mut out16 = stream.alloc_zeros::<f32>(n * nq).unwrap();
        launch_cosine(&stream, &f16_fn, &d_docs16, &d_q, &mut out16, n as u32, nq as u32)
            .unwrap();
        let got16 = stream.clone_dtoh(&out16).unwrap();
        for q in 0..nq {
            for s in 0..n {
                let want = dot(
                    &queries[q * DIM..(q + 1) * DIM],
                    &rounded[s * DIM..(s + 1) * DIM],
                );
                assert!((got16[q * n + s] - want).abs() < 1e-3, "f16 q={q} s={s}");
            }
        }
    }
    #[test]
    #[ignore = "requires a CUDA device"]
    fn fused_maxsim_matches_cpu_with_padding() {
        let (_, stream, k) = setup();
        let (n, dt, qt) = (9usize, 16usize, 5usize);
        let lens: Vec<u32> = vec![16, 1, 7, 0, 16, 3, 9, 12, 2];
        let docs = rnd(n * dt * DIM, 5);
        let query = rnd(qt * DIM, 6);
        let f = k.function(MAXSIM_FUSED, DType::F32, DIM as u32).unwrap();
        let d_docs = stream.clone_htod(&docs).unwrap();
        let d_len = stream.clone_htod(&lens).unwrap();
        let d_q = stream.clone_htod(&query).unwrap();
        let mut out = stream.alloc_zeros::<f32>(n).unwrap();
        launch_maxsim(
                &stream,
                &f,
                &d_docs,
                &d_len,
                &d_q,
                &mut out,
                n as u32,
                dt as u32,
                qt as u32,
            )
            .unwrap();
        let got = stream.clone_dtoh(&out).unwrap();
        for s in 0..n {
            let valid = &docs[s * dt * DIM..(s * dt + lens[s] as usize) * DIM];
            let want = maxsim(&query, valid, DIM);
            assert!(
                (got[s] - want).abs() < 1e-3, "slot {s}: got {} want {want}", got[s]
            );
        }
    }
    /// Measurement gate: batched Candle chain (`matmul -> mask -> max -> sum` over a whole slice)
    /// against the fused kernel on a ColBERT-sized workload. Run with `--nocapture`.
    #[test]
    #[ignore = "requires a CUDA device; measurement"]
    fn gate_maxsim_chain_vs_fused() {
        use candle_core::{DType as CD, Device, Tensor};
        use std::time::Instant;
        const D: usize = 128;
        let (n, dt, qt, iters) = (4096usize, 128usize, 32usize, 20u32);
        let (_, stream, k) = setup();
        let dev = Device::new_cuda(0).unwrap();
        let lens: Vec<u32> = (0..n).map(|i| 24 + (i * 7 % 105) as u32).collect();
        let docs = rnd(n * dt * D, 11);
        let query = rnd(qt * D, 12);
        let bits: Vec<u16> = docs.iter().map(|&v| f16::from_f32(v).to_bits()).collect();
        let d_docs = stream.clone_htod(&bits).unwrap();
        let d_len = stream.clone_htod(&lens).unwrap();
        let d_q = stream.clone_htod(&query).unwrap();
        let mut out = stream.alloc_zeros::<f32>(n).unwrap();
        let fused = k.function(MAXSIM_FUSED, DType::F16, D as u32).unwrap();
        let docs_t = Tensor::from_vec(docs.clone(), (n, dt, D), &dev)
            .unwrap()
            .to_dtype(CD::F16)
            .unwrap()
            .transpose(1, 2)
            .unwrap()
            .contiguous()
            .unwrap();
        let q = Tensor::from_vec(query.clone(), (qt, D), &dev)
            .unwrap()
            .to_dtype(CD::F16)
            .unwrap();
        let mask: Vec<f32> = lens
            .iter()
            .flat_map(|&l| {
                (0..dt)
                    .map(move |j| {
                        if (j as u32) < l {
                            0.0
                        } else {
                            crate::resident::memory::F16_MASK_VALUE
                        }
                    })
            })
            .collect();
        let mask = Tensor::from_vec(mask, (n, 1, dt), &dev)
            .unwrap()
            .to_dtype(CD::F16)
            .unwrap();
        let chain = || {
            q.broadcast_matmul(&docs_t)
                .unwrap()
                .broadcast_add(&mask)
                .unwrap()
                .max(2)
                .unwrap()
                .to_dtype(CD::F32)
                .unwrap()
                .sum(1)
                .unwrap()
        };
        let run_fused = |out: &mut CudaSlice<f32>| {
            launch_maxsim(
                    &stream,
                    &fused,
                    &d_docs,
                    &d_len,
                    &d_q,
                    out,
                    n as u32,
                    dt as u32,
                    qt as u32,
                )
                .unwrap()
        };
        let chain_scores = chain().to_vec1::<f32>().unwrap();
        run_fused(&mut out);
        let fused_scores = stream.clone_dtoh(&out).unwrap();
        for s in (0..n).step_by(97) {
            let tol = 0.5 + 0.02 * fused_scores[s].abs();
            assert!((chain_scores[s] - fused_scores[s]).abs() < tol, "slot {s}");
        }
        let t = Instant::now();
        for _ in 0..iters {
            let _ = chain();
        }
        dev.synchronize().unwrap();
        let chain_ms = t.elapsed().as_secs_f64() * 1e3 / f64::from(iters);
        let t = Instant::now();
        for _ in 0..iters {
            run_fused(&mut out);
        }
        stream.synchronize().unwrap();
        let fused_ms = t.elapsed().as_secs_f64() * 1e3 / f64::from(iters);
        let sim_mb = (n * qt * dt * 2) as f64 / 1e6;
        println!(
            "G-MAXSIM docs={n} doc_tokens={dt} q_tokens={qt} dim={D}: chain {chain_ms:.3} ms, fused {fused_ms:.3} ms, sim-matrix traffic {sim_mb:.1} MB per pass"
        );
    }
}
