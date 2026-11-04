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
        matches!(k.function("nope", DType::F32, 1), Err(KernelError::FuncNotFound { ..
        }))
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
    let rounded: Vec<f32> = bits.iter().map(|&b| f16::from_bits(b).to_f32()).collect();
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
        assert!((got[s] - want).abs() < 1e-3, "slot {s}: got {} want {want}", got[s]);
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
/// Measurement gate: cosine scoring time against score error per document dtype,
/// interleaved rounds, median time. Run with `--nocapture`.
#[test]
#[ignore = "requires a CUDA device; measurement"]
fn gate_cosine_dtype_front() {
    use std::time::Instant;
    const D: usize = 384;
    let (n, nq, rounds) = (200_000usize, 8usize, 25usize);
    let (_, stream, k) = setup();
    let unit = |count: usize, seed: u32| -> Vec<f32> {
        let mut v = rnd(count * D, seed);
        for row in v.chunks_mut(D) {
            let norm = row.iter().map(|x| x * x).sum::<f32>().sqrt();
            row.iter_mut().for_each(|x| *x /= norm);
        }
        v
    };
    let docs = unit(n, 21);
    let queries = unit(nq, 22);
    let d_q = stream.clone_htod(&queries).unwrap();
    let d32 = stream.clone_htod(&docs).unwrap();
    let bits: Vec<u16> = docs.iter().map(|&v| f16::from_f32(v).to_bits()).collect();
    let d16 = stream.clone_htod(&bits).unwrap();
    let f32_fn = k.function(COSINE_DOT, DType::F32, D as u32).unwrap();
    let f16_fn = k.function(COSINE_DOT, DType::F16, D as u32).unwrap();
    let mut out32 = stream.alloc_zeros::<f32>(n * nq).unwrap();
    let mut out16 = stream.alloc_zeros::<f32>(n * nq).unwrap();
    let mut times = [Vec::new(), Vec::new()];
    for round in 0..rounds {
        let t = Instant::now();
        launch_cosine(&stream, &f32_fn, &d32, &d_q, &mut out32, n as u32, nq as u32)
            .unwrap();
        stream.synchronize().unwrap();
        let a = t.elapsed().as_secs_f64() * 1e3;
        let t = Instant::now();
        launch_cosine(&stream, &f16_fn, &d16, &d_q, &mut out16, n as u32, nq as u32)
            .unwrap();
        stream.synchronize().unwrap();
        let b = t.elapsed().as_secs_f64() * 1e3;
        if round >= 5 {
            times[0].push(a);
            times[1].push(b);
        }
    }
    let got32 = stream.clone_dtoh(&out32).unwrap();
    let got16 = stream.clone_dtoh(&out16).unwrap();
    let (mut e32, mut e16) = (0f64, 0f64);
    let mut sum16 = 0f64;
    let mut count = 0usize;
    for q in 0..nq {
        for s in (0..n).step_by(401) {
            let want: f64 = (0..D)
                .map(|i| f64::from(queries[q * D + i]) * f64::from(docs[s * D + i]))
                .sum();
            e32 = e32.max((f64::from(got32[q * n + s]) - want).abs());
            let err = (f64::from(got16[q * n + s]) - want).abs();
            e16 = e16.max(err);
            sum16 += err;
            count += 1;
        }
    }
    for (name, idx, err) in [("F32", 0usize, e32), ("F16", 1, e16)] {
        let t = &mut times[idx];
        t.sort_by(f64::total_cmp);
        println!(
            "G-COSINE {name}: median {:.3} ms min {:.3} ms max {:.3} ms abs_err_max {err:.3e}",
            t[t.len() / 2], t[0], t[t.len() - 1],
        );
    }
    println!(
        "G-COSINE docs={n} dim={D} queries={nq} F16 abs_err_mean {:.3e}", sum16 / count
        as f64
    );
}
/// Measurement gate: bandwidth-oriented scoring (one warp per document) for f32, f16 and
/// int8 via `dp4a`; time against score error. Run with `--nocapture`.
#[test]
#[ignore = "requires a CUDA device; measurement"]
fn gate_cosine_bandwidth_front() {
    use cudarc::driver::{LaunchConfig, PushKernelArg};
    use cudarc::nvrtc::{compile_ptx_with_opts, CompileOptions};
    use std::time::Instant;
    const D: usize = 384;
    const NQ: usize = 8;
    const SRC: &str = r#"
#define D 384
#define NQ 8
struct __align__(16) F4 { float x, y, z, w; };
struct __align__(8) U2 { unsigned x, y; };
__device__ __forceinline__ float h2f(unsigned h) {
    float f; unsigned short s = (unsigned short)h;
    asm("cvt.f32.f16 %0, %1;" : "=f"(f) : "h"(s)); return f;
}
__device__ __forceinline__ float wsum(float v) {
    for (int o = 16; o > 0; o >>= 1) v += __shfl_xor_sync(0xffffffffu, v, o);
    return v;
}
extern "C" __global__ void score_f32(const F4* docs, const F4* qs, float* out, unsigned n) {
    __shared__ F4 sq[NQ * D / 4];
    for (unsigned i = threadIdx.x; i < NQ * D / 4; i += blockDim.x) sq[i] = qs[i];
    __syncthreads();
    unsigned lane = threadIdx.x & 31;
    unsigned warp = (blockIdx.x * blockDim.x + threadIdx.x) >> 5;
    unsigned nw = (gridDim.x * blockDim.x) >> 5;
    for (unsigned d = warp; d < n; d += nw) {
        F4 r[3];
        for (int j = 0; j < 3; ++j) r[j] = docs[(size_t)d * (D / 4) + lane + 32 * j];
        for (int q = 0; q < NQ; ++q) {
            float a = 0.f;
            for (int j = 0; j < 3; ++j) {
                F4 v = sq[q * (D / 4) + lane + 32 * j];
                a += r[j].x * v.x + r[j].y * v.y + r[j].z * v.z + r[j].w * v.w;
            }
            a = wsum(a);
            if (lane == 0) out[(size_t)q * n + d] = a;
        }
    }
}
extern "C" __global__ void score_f16(const U2* docs, const F4* qs, float* out, unsigned n) {
    __shared__ F4 sq[NQ * D / 4];
    for (unsigned i = threadIdx.x; i < NQ * D / 4; i += blockDim.x) sq[i] = qs[i];
    __syncthreads();
    unsigned lane = threadIdx.x & 31;
    unsigned warp = (blockIdx.x * blockDim.x + threadIdx.x) >> 5;
    unsigned nw = (gridDim.x * blockDim.x) >> 5;
    for (unsigned d = warp; d < n; d += nw) {
        F4 r[3];
        for (int j = 0; j < 3; ++j) {
            U2 u = docs[(size_t)d * (D / 4) + lane + 32 * j];
            r[j].x = h2f(u.x & 0xffffu); r[j].y = h2f(u.x >> 16);
            r[j].z = h2f(u.y & 0xffffu); r[j].w = h2f(u.y >> 16);
        }
        for (int q = 0; q < NQ; ++q) {
            float a = 0.f;
            for (int j = 0; j < 3; ++j) {
                F4 v = sq[q * (D / 4) + lane + 32 * j];
                a += r[j].x * v.x + r[j].y * v.y + r[j].z * v.z + r[j].w * v.w;
            }
            a = wsum(a);
            if (lane == 0) out[(size_t)q * n + d] = a;
        }
    }
}
extern "C" __global__ void score_i8(const int* docs, const float* dscale, const int* qs,
                                    const float* qscale, float* out, unsigned n) {
    __shared__ int sq[NQ * D / 4];
    for (unsigned i = threadIdx.x; i < NQ * D / 4; i += blockDim.x) sq[i] = qs[i];
    __syncthreads();
    unsigned lane = threadIdx.x & 31;
    unsigned warp = (blockIdx.x * blockDim.x + threadIdx.x) >> 5;
    unsigned nw = (gridDim.x * blockDim.x) >> 5;
    for (unsigned d = warp; d < n; d += nw) {
        int r[3];
        for (int j = 0; j < 3; ++j) r[j] = docs[(size_t)d * (D / 4) + lane + 32 * j];
        float ds = dscale[d];
        for (int q = 0; q < NQ; ++q) {
            int a = 0;
            for (int j = 0; j < 3; ++j) a = __dp4a(r[j], sq[q * (D / 4) + lane + 32 * j], a);
            float f = wsum((float)a);
            if (lane == 0) out[(size_t)q * n + d] = f * ds * qscale[q];
        }
    }
}
"#;
    let (n, rounds) = (200_000usize, 20usize);
    let (ctx, stream, _) = setup();
    let ptx = compile_ptx_with_opts(
            SRC,
            CompileOptions {
                arch: Some("compute_75"),
                ..Default::default()
            },
        )
        .unwrap();
    let module = ctx.load_module(ptx).unwrap();
    let (f32_fn, f16_fn, i8_fn) = (
        module.load_function("score_f32").unwrap(),
        module.load_function("score_f16").unwrap(),
        module.load_function("score_i8").unwrap(),
    );
    let unit = |count: usize, seed: u32| -> Vec<f32> {
        let mut v = rnd(count * D, seed);
        for row in v.chunks_mut(D) {
            let norm = row.iter().map(|x| x * x).sum::<f32>().sqrt();
            row.iter_mut().for_each(|x| *x /= norm);
        }
        v
    };
    let quant = |v: &[f32]| -> (Vec<i8>, Vec<f32>) {
        let mut q = Vec::with_capacity(v.len());
        let mut scales = Vec::new();
        for row in v.chunks(D) {
            let s = row.iter().fold(0f32, |m, x| m.max(x.abs())) / 127.0;
            scales.push(s);
            q.extend(row.iter().map(|x| (x / s).round() as i8));
        }
        (q, scales)
    };
    let docs = unit(n, 21);
    let queries = unit(NQ, 22);
    let (docs8, dscale) = quant(&docs);
    let (q8, qscale) = quant(&queries);
    let bits: Vec<u16> = docs.iter().map(|&v| f16::from_f32(v).to_bits()).collect();
    let (d32, d16, d8) = (
        stream.clone_htod(&docs).unwrap(),
        stream.clone_htod(&bits).unwrap(),
        stream.clone_htod(&docs8).unwrap(),
    );
    let (dq, dq8, ds, qs) = (
        stream.clone_htod(&queries).unwrap(),
        stream.clone_htod(&q8).unwrap(),
        stream.clone_htod(&dscale).unwrap(),
        stream.clone_htod(&qscale).unwrap(),
    );
    let mut outs = [
        stream.alloc_zeros::<f32>(n * NQ).unwrap(),
        stream.alloc_zeros::<f32>(n * NQ).unwrap(),
        stream.alloc_zeros::<f32>(n * NQ).unwrap(),
    ];
    let cfg = LaunchConfig {
        grid_dim: (272, 1, 1),
        block_dim: (256, 1, 1),
        shared_mem_bytes: 0,
    };
    let nn = n as u32;
    let mut times = [Vec::new(), Vec::new(), Vec::new()];
    for round in 0..rounds {
        for k in 0..3 {
            let t = Instant::now();
            let [o0, o1, o2] = &mut outs;
            match k {
                0 => {
                    unsafe {
                        stream
                            .launch_builder(&f32_fn)
                            .arg(&d32)
                            .arg(&dq)
                            .arg(o0)
                            .arg(&nn)
                            .launch(cfg)
                    }
                }
                1 => {
                    unsafe {
                        stream
                            .launch_builder(&f16_fn)
                            .arg(&d16)
                            .arg(&dq)
                            .arg(o1)
                            .arg(&nn)
                            .launch(cfg)
                    }
                }
                _ => {
                    unsafe {
                        stream
                            .launch_builder(&i8_fn)
                            .arg(&d8)
                            .arg(&ds)
                            .arg(&dq8)
                            .arg(&qs)
                            .arg(o2)
                            .arg(&nn)
                            .launch(cfg)
                    }
                }
            }
                .unwrap();
            stream.synchronize().unwrap();
            if round >= 5 {
                times[k].push(t.elapsed().as_secs_f64() * 1e3);
            }
        }
    }
    let got: Vec<Vec<f32>> = outs
        .iter()
        .map(|o| stream.clone_dtoh(o).unwrap())
        .collect();
    let bytes = [n * D * 4, n * D * 2, n * D + n * 4];
    for (k, name) in ["F32", "F16", "I8-dp4a"].into_iter().enumerate() {
        let (mut max, mut sum, mut count) = (0f64, 0f64, 0usize);
        for q in 0..NQ {
            for s in (0..n).step_by(401) {
                let want: f64 = (0..D)
                    .map(|i| f64::from(queries[q * D + i]) * f64::from(docs[s * D + i]))
                    .sum();
                let err = (f64::from(got[k][q * n + s]) - want).abs();
                max = max.max(err);
                sum += err;
                count += 1;
            }
        }
        let t = &mut times[k];
        t.sort_by(f64::total_cmp);
        let med = t[t.len() / 2];
        println!(
            "G-BW {name}: median {med:.3} ms min {:.3} ms max {:.3} ms read {:.1} GB/s abs_err_max {max:.3e} mean {:.3e}",
            t[0], t[t.len() - 1], bytes[k] as f64 / med / 1e6, sum / count as f64,
        );
    }
}
