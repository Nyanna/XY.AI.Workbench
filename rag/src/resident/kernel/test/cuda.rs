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
