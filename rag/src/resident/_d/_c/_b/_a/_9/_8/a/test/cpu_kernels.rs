use super::*;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::a::kernel_register::KernelRegister;
#[test]
fn cpu_kernel_register_is_send_sync() {
    fn check<T: Send + Sync>() {}
    check::<CpuKernelRegister>();
}
struct Lcg(u64);
impl Lcg {
    fn next(&mut self) -> f32 {
        self.0 = self
            .0
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        ((self.0 >> 40) as f32 / (1u64 << 24) as f32) * 2.0 - 1.0
    }
    fn vector(&mut self, len: usize) -> Vec<f32> {
        (0..len).map(|_| self.next()).collect()
    }
}
struct ConstKernel(f32);
impl CpuScoreKernel for ConstKernel {
    fn score(&self, _query: &[f32], _rows: &[f32], _dim: usize, out: &mut [f32]) {
        out.fill(self.0);
    }
}
fn dot_ref(q: &[f32], x: &[f32]) -> f64 {
    q.iter().zip(x).map(|(a, b)| *a as f64 * *b as f64).sum()
}
fn negl2_ref(q: &[f32], x: &[f32]) -> f64 {
    -q.iter().zip(x).map(|(a, b)| (*a as f64 - *b as f64).powi(2)).sum::<f64>()
}
fn assert_close(actual: f32, expected: f64) {
    let tolerance = 1e-4 * (1.0 + expected.abs());
    assert!(
        (actual as f64 - expected).abs() <= tolerance,
        "actual {actual} expected {expected}"
    );
}
fn run_kernel(
    kernel: &dyn CpuScoreKernel,
    query: &[f32],
    rows: &[f32],
    dim: usize,
) -> Vec<f32> {
    let n = if dim == 0 { 3 } else { rows.len() / dim };
    let mut out = vec![f32::MIN; n];
    kernel.score(query, rows, dim, &mut out);
    out
}
#[test]
fn builtin_kernels_match_the_reference_for_many_dimensions() {
    let register = CpuKernelRegister::with_builtin();
    let dot = register.request_at("score_dot", DType::F32, Instant::now()).unwrap();
    let negl2 = register.request_at("score_negl2", DType::F32, Instant::now()).unwrap();
    let mut rng = Lcg(7);
    for dim in [
        1usize, 2, 3, 4, 7, 8, 9, 15, 16, 17, 23, 31, 32, 33, 63, 64, 65, 100, 384, 385,
    ] {
        let rows_n = 5;
        let query = rng.vector(dim);
        let rows = rng.vector(dim * rows_n);
        let d = run_kernel(dot.as_ref(), &query, &rows, dim);
        let l = run_kernel(negl2.as_ref(), &query, &rows, dim);
        for i in 0..rows_n {
            let row = &rows[i * dim..(i + 1) * dim];
            assert_close(d[i], dot_ref(&query, row));
            assert_close(l[i], negl2_ref(&query, row));
        }
    }
}
#[test]
fn simd_and_scalar_paths_agree() {
    let detected = Isa::detect();
    let mut rng = Lcg(11);
    for dim in [3usize, 8, 17, 64, 384] {
        let query = rng.vector(dim);
        let row = rng.vector(dim);
        for metric in [Metric::Dot, Metric::NegL2Sq] {
            let simd = row_score(detected, metric, &query, &row);
            let scalar = row_score(Isa::Scalar, metric, &query, &row);
            assert!(
                (simd - scalar).abs() <= 1e-4 * (1.0 + scalar.abs()),
                "{metric:?} dim {dim}"
            );
        }
    }
}
#[test]
fn kernels_pass_nan_and_inf_through() {
    let kernels = [
        BuiltinKernel {
            metric: Metric::Dot,
            isa: Isa::detect(),
        },
        BuiltinKernel {
            metric: Metric::Dot,
            isa: Isa::Scalar,
        },
    ];
    for k in &kernels {
        let mut q = vec![1.0f32; 20];
        let mut row = vec![1.0f32; 20];
        assert_eq!(run_kernel(k, & q, & row, 20), vec![20.0]);
        row[5] = f32::NAN;
        assert!(run_kernel(k, & q, & row, 20) [0].is_nan());
        row[5] = f32::INFINITY;
        assert_eq!(run_kernel(k, & q, & row, 20) [0], f32::INFINITY);
        q[19] = f32::NEG_INFINITY;
        row[19] = 1.0;
        assert!(run_kernel(k, & q, & row, 20) [0].is_nan());
    }
    let negl2 = BuiltinKernel {
        metric: Metric::NegL2Sq,
        isa: Isa::detect(),
    };
    let mut row = vec![0.0f32; 12];
    row[3] = f32::INFINITY;
    assert_eq!(run_kernel(& negl2, & vec![0.0; 12], & row, 12) [0], f32::NEG_INFINITY);
    assert_eq!(run_kernel(& negl2, & vec![1.0; 12], & vec![1.0; 12], 12) [0], 0.0);
}
#[test]
fn kernels_reject_inconsistent_shapes_with_nan() {
    let k = BuiltinKernel {
        metric: Metric::Dot,
        isa: Isa::detect(),
    };
    let mut out = vec![0.0; 2];
    k.score(&[1.0, 2.0], &[1.0, 2.0, 3.0], 2, &mut out);
    assert_eq!(out[0], 5.0);
    assert!(out[1].is_nan());
    k.score(&[1.0], &[1.0, 2.0], 2, &mut out);
    assert!(out.iter().all(| v | v.is_nan()));
    let mut empty = vec![9.0; 3];
    k.score(&[], &[], 0, &mut empty);
    assert_eq!(empty, vec![0.0; 3]);
}
#[test]
fn register_registers_requests_and_evicts() {
    let register = CpuKernelRegister::with_builtin();
    let now = Instant::now();
    assert!(
        matches!(register.register("score_dot", DType::F32, Arc::new(ConstKernel(0.0))),
        Err(KernelError::Unavailable(_)))
    );
    register.register("score_dot", DType::F16, Arc::new(ConstKernel(1.0))).unwrap();
    assert!(
        matches!(register.request_at("missing", DType::F32, now),
        Err(KernelError::UnknownKernel(ref n)) if n.contains("missing"))
    );
    let ttl = Duration::from_secs(30);
    let held = register.request_at("score_dot", DType::F32, now).unwrap();
    let released = register.request_at("score_negl2", DType::F32, now).unwrap();
    drop(released);
    assert!(
        register.evict_expired(now + ttl - Duration::from_millis(1), ttl).is_empty()
    );
    assert_eq!(
        register.evict_expired(now + ttl, ttl), vec!["kernel:score_negl2:F32"
        .to_string()]
    );
    assert!(register.evict_expired(now + ttl * 10, ttl).is_empty());
    let again = register.request_at("score_dot", DType::F32, now + ttl).unwrap();
    assert!(Arc::ptr_eq(& held, & again));
    drop((held, again));
    let later = now + ttl * 3;
    assert_eq!(
        register.evict_expired(later, ttl), vec!["kernel:score_dot:F32".to_string()]
    );
    let recreated = register.request_at("score_dot", DType::F32, later).unwrap();
    let mut out = [0.0];
    recreated.score(&[1.0, 2.0], &[3.0, 4.0], 2, &mut out);
    assert_eq!(out[0], 11.0);
    let custom = register.request_at("score_dot", DType::F16, later).unwrap();
    custom.score(&[], &[], 0, &mut out);
    assert_eq!(out[0], 1.0);
}
#[test]
fn residency_follows_live_kernels() {
    let register = CpuKernelRegister::with_builtin();
    let now = Instant::now();
    assert!(! register.is_resident("score_dot", DType::F32));
    let kernel = register.request_at("score_dot", DType::F32, now).unwrap();
    assert!(register.is_resident("score_dot", DType::F32));
    assert!(! register.is_resident("score_dot", DType::F16));
    drop(kernel);
    let ttl = Duration::from_secs(30);
    register.evict_expired(now + ttl, ttl);
    assert!(! register.is_resident("score_dot", DType::F32));
}
#[test]
fn provider_returns_cpu_kernels_and_ignores_the_variant() {
    let register = CpuKernelRegister::with_builtin();
    let variant = Variant {
        dim: Some(8),
        tile: None,
    };
    let kernel = KernelProvider::request(&register, "score_dot", DType::F32, variant)
        .unwrap();
    let Kernel::Cpu(k) = kernel else { panic!("expected a cpu kernel") };
    assert_eq!(run_kernel(k.as_ref(), & [1.0, 2.0], & [3.0, 4.0], 2), vec![11.0]);
    assert!(
        matches!(KernelProvider::request(& register, "score_dot", DType::F16, variant),
        Err(KernelError::UnknownKernel(_)))
    );
}
#[test]
fn provider_kernels_count_as_in_use_until_dropped() {
    let register = CpuKernelRegister::with_builtin();
    let variant = Variant { dim: None, tile: None };
    let held = KernelProvider::request(&register, "score_dot", DType::F32, variant)
        .unwrap();
    let far = Instant::now() + Duration::from_secs(3600);
    assert!(register.evict_expired(far, Duration::from_secs(1)).is_empty());
    drop(held);
    assert_eq!(
        register.evict_expired(far, Duration::from_secs(1)), vec!["kernel:score_dot:F32"
        .to_string()]
    );
}
#[test]
fn both_registers_are_providers() {
    fn is_provider<T: KernelProvider>() {}
    is_provider::<CpuKernelRegister>();
    is_provider::<KernelRegister>();
}
