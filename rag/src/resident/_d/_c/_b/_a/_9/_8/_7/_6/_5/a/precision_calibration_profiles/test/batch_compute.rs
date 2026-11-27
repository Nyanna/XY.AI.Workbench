use super::*;
use std::sync::Arc;
#[test]
fn compute_types_are_send_sync() {
    fn check<T: Send + Sync>() {}
    check::<FloatBatch>();
    check::<BatchKernel>();
    check::<DeviceKind>();
    check::<ComputeError>();
    check::<CpuBatchCompute>();
    check::<GpuBatchCompute>();
}
const ALL: [BatchKernel; 4] = [
    BatchKernel::MatMul,
    BatchKernel::Add,
    BatchKernel::Mul,
    BatchKernel::SoftmaxLastDim,
];
fn fb(rows: usize, cols: usize, data: &[f32]) -> FloatBatch {
    FloatBatch::new(rows, cols, data.to_vec()).unwrap()
}
fn pattern(rows: usize, cols: usize, seed: f32) -> FloatBatch {
    let data = (0..rows * cols)
        .map(|i| ((i as f32 * 0.37 + seed).sin()) * 3.0)
        .collect();
    FloatBatch::new(rows, cols, data).unwrap()
}
fn close(a: &[f32], b: &[f32]) -> bool {
    a.len() == b.len()
        && a
            .iter()
            .zip(b)
            .all(|(x, y)| (x - y).abs() <= 1e-4 * x.abs().max(y.abs()).max(1e-2))
}
#[test]
fn batch_new_validates_length() {
    assert!(FloatBatch::new(2, 3, vec![0.0; 6]).is_ok());
    assert!(
        matches!(FloatBatch::new(2, 3, vec![0.0; 5]), Err(ComputeError::Mismatch(_)))
    );
    assert_eq!(FloatBatch::new(usize::MAX, 2, Vec::new()), Err(ComputeError::Overflow));
}
#[test]
fn batch_accessors() {
    let b = fb(2, 2, &[1.0, 2.0, 3.0, 4.0]);
    assert_eq!((b.rows(), b.cols()), (2, 2));
    assert_eq!(b.as_slice(), & [1.0, 2.0, 3.0, 4.0]);
    assert_eq!(b.into_vec(), vec![1.0, 2.0, 3.0, 4.0]);
}
#[test]
fn kernel_names_and_arity() {
    assert_eq!(BatchKernel::MatMul.name(), "matmul");
    assert_eq!(BatchKernel::Add.name(), "add");
    assert_eq!(BatchKernel::Mul.name(), "mul");
    assert_eq!(BatchKernel::SoftmaxLastDim.name(), "softmax_last_dim");
    assert_eq!(BatchKernel::MatMul.arity(), 2);
    assert_eq!(BatchKernel::Add.arity(), 2);
    assert_eq!(BatchKernel::Mul.arity(), 2);
    assert_eq!(BatchKernel::SoftmaxLastDim.arity(), 1);
}
#[test]
fn error_display_is_nonempty() {
    let errs = [
        ComputeError::Mismatch("m".into()),
        ComputeError::Overflow,
        ComputeError::GpuUnavailable("g".into()),
        ComputeError::Backend("b".into()),
    ];
    for e in errs {
        assert!(! e.to_string().is_empty());
        let _: &dyn std::error::Error = &e;
    }
}
#[test]
fn trait_is_object_safe() {
    let boxed: Box<dyn BatchCompute> = Box::new(CpuBatchCompute::new());
    let arc: Arc<dyn BatchCompute> = Arc::new(CpuBatchCompute::default());
    assert_eq!(boxed.kind(), DeviceKind::Cpu);
    assert_eq!(arc.kind(), DeviceKind::Cpu);
}
#[test]
fn cpu_add_and_mul() {
    let c = CpuBatchCompute::new();
    let a = fb(2, 2, &[1.0, 2.0, 3.0, 4.0]);
    let b = fb(2, 2, &[10.0, 20.0, 30.0, 40.0]);
    let sum = c.run(BatchKernel::Add, &[&a, &b]).unwrap();
    assert_eq!(sum, fb(2, 2, & [11.0, 22.0, 33.0, 44.0]));
    let prod = c.run(BatchKernel::Mul, &[&a, &b]).unwrap();
    assert_eq!(prod, fb(2, 2, & [10.0, 40.0, 90.0, 160.0]));
}
#[test]
fn cpu_matmul_rectangular() {
    let c = CpuBatchCompute::new();
    let a = fb(2, 3, &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
    let b = fb(3, 2, &[7.0, 8.0, 9.0, 10.0, 11.0, 12.0]);
    let out = c.run(BatchKernel::MatMul, &[&a, &b]).unwrap();
    assert_eq!(out, fb(2, 2, & [58.0, 64.0, 139.0, 154.0]));
}
#[test]
fn cpu_matmul_result_shape() {
    let c = CpuBatchCompute::new();
    let out = c
        .run(BatchKernel::MatMul, &[&pattern(3, 4, 0.1), &pattern(4, 5, 0.2)])
        .unwrap();
    assert_eq!((out.rows(), out.cols()), (3, 5));
}
#[test]
fn cpu_softmax_rows_sum_to_one() {
    let c = CpuBatchCompute::new();
    let x = fb(2, 3, &[1.0, 2.0, 3.0, 1000.0, 1000.0, 1000.0]);
    let out = c.run(BatchKernel::SoftmaxLastDim, &[&x]).unwrap();
    for row in out.as_slice().chunks(3) {
        assert!((row.iter().sum::< f32 > () - 1.0).abs() < 1e-5);
    }
    let s = out.as_slice();
    assert!(s[0] < s[1] && s[1] < s[2]);
    assert!((s[3] - 1.0 / 3.0).abs() < 1e-6);
}
#[test]
fn cpu_softmax_matches_formula() {
    let c = CpuBatchCompute::new();
    let out = c.run(BatchKernel::SoftmaxLastDim, &[&fb(1, 2, &[0.0, 1.0])]).unwrap();
    let e = std::f32::consts::E;
    assert!(close(out.as_slice(), & [1.0 / (1.0 + e), e / (1.0 + e)]));
}
#[test]
fn arity_mismatch_for_every_kernel() {
    let c = CpuBatchCompute::new();
    let a = fb(1, 1, &[1.0]);
    for k in ALL {
        let too_many = vec![& a; k.arity() + 1];
        let none: [&FloatBatch; 0] = [];
        assert!(matches!(c.run(k, & too_many), Err(ComputeError::Mismatch(_))));
        assert!(matches!(c.run(k, & none), Err(ComputeError::Mismatch(_))));
    }
}
#[test]
fn shape_mismatch_cases() {
    let c = CpuBatchCompute::new();
    let a = pattern(2, 3, 0.0);
    let b = pattern(3, 2, 0.0);
    assert!(
        matches!(c.run(BatchKernel::Add, & [& a, & b]), Err(ComputeError::Mismatch(_)))
    );
    assert!(
        matches!(c.run(BatchKernel::Mul, & [& a, & b]), Err(ComputeError::Mismatch(_)))
    );
    assert!(
        matches!(c.run(BatchKernel::MatMul, & [& a, & a]),
        Err(ComputeError::Mismatch(_)))
    );
}
#[test]
fn inputs_are_unchanged() {
    let c = CpuBatchCompute::new();
    let a = pattern(3, 3, 0.5);
    let b = pattern(3, 3, 1.5);
    let (a0, b0) = (a.clone(), b.clone());
    for k in ALL {
        let ins: Vec<&FloatBatch> = [&a, &b].into_iter().take(k.arity()).collect();
        c.run(k, &ins).unwrap();
    }
    assert_eq!((a, b), (a0, b0));
}
#[test]
fn empty_batches_are_valid() {
    let c = CpuBatchCompute::new();
    let e03 = fb(0, 3, &[]);
    let e30 = fb(3, 0, &[]);
    let r = c.run(BatchKernel::Add, &[&e03, &e03]).unwrap();
    assert_eq!((r.rows(), r.cols(), r.as_slice().len()), (0, 3, 0));
    let r = c.run(BatchKernel::Mul, &[&e30, &e30]).unwrap();
    assert_eq!((r.rows(), r.cols(), r.as_slice().len()), (3, 0, 0));
    let r = c.run(BatchKernel::SoftmaxLastDim, &[&e30]).unwrap();
    assert_eq!((r.rows(), r.cols(), r.as_slice().len()), (3, 0, 0));
    let r = c.run(BatchKernel::SoftmaxLastDim, &[&e03]).unwrap();
    assert_eq!((r.rows(), r.cols(), r.as_slice().len()), (0, 3, 0));
    let r = c.run(BatchKernel::MatMul, &[&e03, &pattern(3, 2, 0.0)]).unwrap();
    assert_eq!((r.rows(), r.cols(), r.as_slice().len()), (0, 2, 0));
}
#[test]
fn matmul_with_zero_inner_dimension_is_zero_matrix() {
    let c = CpuBatchCompute::new();
    let r = c.run(BatchKernel::MatMul, &[&fb(2, 0, &[]), &fb(0, 3, &[])]).unwrap();
    assert_eq!(r, fb(2, 3, & [0.0; 6]));
}
#[test]
fn dimension_overflow_is_reported() {
    let c = CpuBatchCompute::new();
    let a = FloatBatch::new(usize::MAX, 0, Vec::new()).unwrap();
    let b = FloatBatch::new(0, 2, Vec::new()).unwrap();
    assert_eq!(c.run(BatchKernel::MatMul, & [& a, & b]), Err(ComputeError::Overflow));
}
#[test]
fn concurrent_runs_on_one_instance() {
    let c: Arc<dyn BatchCompute> = Arc::new(CpuBatchCompute::new());
    let a = Arc::new(pattern(8, 8, 0.3));
    let expected = c.run(BatchKernel::MatMul, &[&a, &a]).unwrap();
    let handles: Vec<_> = (0..4)
        .map(|_| {
            let (c, a) = (c.clone(), a.clone());
            std::thread::spawn(move || c.run(BatchKernel::MatMul, &[&a, &a]).unwrap())
        })
        .collect();
    for h in handles {
        assert_eq!(h.join().unwrap(), expected);
    }
}
#[test]
fn default_matches_gpu_availability() {
    match GpuBatchCompute::new() {
        Ok(_) => assert_eq!(create_default().kind(), DeviceKind::Gpu),
        Err(e) => {
            assert!(matches!(e, ComputeError::GpuUnavailable(_)));
            assert_eq!(create_default().kind(), DeviceKind::Cpu);
        }
    }
}
mod gpu_parity {
    use super::*;
    fn gpu_or_skip() -> Option<GpuBatchCompute> {
        match GpuBatchCompute::new() {
            Ok(g) => Some(g),
            Err(e) => {
                assert!(matches!(e, ComputeError::GpuUnavailable(_)));
                eprintln!("no usable GPU, skipping: {e}");
                None
            }
        }
    }
    #[test]
    fn gpu_matches_cpu() {
        let Some(gpu) = gpu_or_skip() else { return };
        assert_eq!(gpu.kind(), DeviceKind::Gpu);
        let cpu = CpuBatchCompute::new();
        let a = pattern(5, 7, 0.1);
        let b = pattern(7, 4, 0.9);
        let s1 = pattern(5, 7, 1.7);
        let cases: Vec<(BatchKernel, Vec<&FloatBatch>)> = vec![
            (BatchKernel::MatMul, vec![& a, & b]), (BatchKernel::Add, vec![& a, & s1]),
            (BatchKernel::Mul, vec![& a, & s1]), (BatchKernel::SoftmaxLastDim, vec![&
            a]),
        ];
        for (k, ins) in cases {
            let g = gpu.run(k, &ins).unwrap();
            let c = cpu.run(k, &ins).unwrap();
            assert_eq!((g.rows(), g.cols()), (c.rows(), c.cols()), "{k:?}");
            assert!(close(g.as_slice(), c.as_slice()), "{k:?}");
        }
    }
    #[test]
    fn gpu_errors_match_cpu() {
        let Some(gpu) = gpu_or_skip() else { return };
        let a = pattern(2, 3, 0.0);
        assert!(
            matches!(gpu.run(BatchKernel::MatMul, & [& a, & a]),
            Err(ComputeError::Mismatch(_)))
        );
        assert!(
            matches!(gpu.run(BatchKernel::Add, & [& a]), Err(ComputeError::Mismatch(_)))
        );
    }
    #[test]
    fn gpu_empty_batches() {
        let Some(gpu) = gpu_or_skip() else { return };
        let r = gpu.run(BatchKernel::MatMul, &[&fb(2, 0, &[]), &fb(0, 3, &[])]).unwrap();
        assert_eq!(r, fb(2, 3, & [0.0; 6]));
    }
    #[test]
    fn gpu_concurrent_runs() {
        let Some(gpu) = gpu_or_skip() else { return };
        let gpu = Arc::new(gpu);
        let a = Arc::new(pattern(6, 6, 0.4));
        let expected = CpuBatchCompute::new()
            .run(BatchKernel::MatMul, &[&a, &a])
            .unwrap();
        let handles: Vec<_> = (0..4)
            .map(|_| {
                let (g, a) = (gpu.clone(), a.clone());
                std::thread::spawn(move || {
                    g.run(BatchKernel::MatMul, &[&a, &a]).unwrap()
                })
            })
            .collect();
        for h in handles {
            assert!(close(h.join().unwrap().as_slice(), expected.as_slice()));
        }
    }
}
