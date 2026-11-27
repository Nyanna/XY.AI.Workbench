use super::*;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::_1::capability_model::{
    cpu_reference, gpu_reference, Axis, Value,
};
#[test]
fn capability_types_are_thread_safe() {
    fn check<T: Send + Sync>() {}
    fn check_error<T: Send + Sync + std::error::Error>() {}
    check::<CapabilityConfig>();
    check::<CpuFeatures>();
    check::<DetectedCapabilities>();
    check_error::<DetectError>();
}
struct FakeProbe {
    cc: Option<ComputeCapability>,
    name: Option<String>,
}
impl GpuProbe for FakeProbe {
    fn compute_capability(&self) -> Option<ComputeCapability> {
        self.cc
    }
    fn device_name(&self) -> Option<String> {
        self.name.clone()
    }
}
fn reference_probe() -> FakeProbe {
    FakeProbe {
        cc: Some(ComputeCapability::new(7, 5)),
        name: Some("NVIDIA GeForce GTX 1660 SUPER".to_owned()),
    }
}
fn has(caps: &CapabilitySet, axis: Axis, value: Value) -> bool {
    caps.contains(&axis, &value)
}
fn nums(caps: &CapabilitySet, axis: &Axis) -> Vec<u64> {
    caps.values(axis)
        .map(|vs| vs.iter().map(|v| v.as_str().parse().unwrap()).collect())
        .unwrap_or_default()
}
fn sorted(mut v: Vec<u64>) -> Vec<u64> {
    v.sort_unstable();
    v
}
fn all_features() -> CpuFeatures {
    CpuFeatures {
        avx2: true,
        f16c: true,
        fma: true,
        vnni: true,
    }
}
#[test]
fn thread_counts_examples() {
    assert_eq!(thread_counts(1), vec![1]);
    assert_eq!(thread_counts(6), vec![1, 2, 4, 6]);
    assert_eq!(thread_counts(8), vec![1, 2, 4, 8]);
    assert_eq!(thread_counts(0), vec![1]);
    assert_eq!(thread_counts(2), vec![1, 2]);
    assert_eq!(thread_counts(3), vec![1, 2, 3]);
    assert_eq!(thread_counts(16), vec![1, 2, 4, 8, 16]);
}
#[test]
fn thread_counts_extremes_do_not_panic() {
    let v = thread_counts(usize::MAX);
    assert_eq!(v.first(), Some(& 1));
    assert_eq!(v.last(), Some(& u32::MAX));
    assert!(v.windows(2).all(| w | w[0] < w[1]));
}
#[test]
fn cpu_default_features_has_fp32_only() {
    let caps = cpu_capabilities(CpuFeatures::default(), 0);
    assert!(has(& caps, axes::operands(), values::fp32()));
    assert!(has(& caps, axes::scoring(), values::fp32()));
    assert_eq!(caps.values(& axes::operands()).unwrap().len(), 1);
    assert_eq!(caps.values(& axes::scoring()).unwrap().len(), 1);
    assert_eq!(nums(& caps, & axes::thread_count()), vec![1]);
    assert!(caps.values(& axes::accumulator()).is_none());
    assert!(caps.values(& axes::fusion()).is_none());
    assert!(caps.values(& axes::batch_size()).is_none());
    assert!(caps.values(& axes::tile_size()).is_none());
}
#[test]
fn cpu_feature_dependent_values() {
    let f16c = cpu_capabilities(
        CpuFeatures {
            f16c: true,
            ..Default::default()
        },
        4,
    );
    assert!(has(& f16c, axes::operands(), values::fp16_storage_fp32_compute()));
    assert!(! has(& f16c, axes::scoring(), values::int8()));
    let avx2 = cpu_capabilities(
        CpuFeatures {
            avx2: true,
            ..Default::default()
        },
        4,
    );
    assert!(has(& avx2, axes::scoring(), values::int8()));
    assert!(! has(& avx2, axes::scoring(), values::int8_vnni()));
    assert!(! has(& avx2, axes::operands(), values::fp16_storage_fp32_compute()));
    let vnni = cpu_capabilities(
        CpuFeatures {
            vnni: true,
            ..Default::default()
        },
        4,
    );
    assert!(has(& vnni, axes::scoring(), values::int8()));
    assert!(has(& vnni, axes::scoring(), values::int8_vnni()));
    let fma = cpu_capabilities(
        CpuFeatures {
            fma: true,
            ..Default::default()
        },
        4,
    );
    assert_eq!(fma, cpu_capabilities(CpuFeatures::default(), 4));
}
#[test]
fn cpu_full_features_within_reference_ignoring_threads() {
    let mut caps = cpu_capabilities(all_features(), 6);
    assert_eq!(sorted(nums(& caps, & axes::thread_count())), vec![1, 2, 4, 6]);
    let reduced = reduce_to_kernels(caps.clone(), &cpu_reference(), "cpu");
    assert_eq!(
        reduced.values(& axes::thread_count()), caps.values(& axes::thread_count())
    );
    caps = reduced;
    assert!(has(& caps, axes::scoring(), values::int8_vnni()));
}
#[test]
fn gpu_capabilities_by_cc() {
    let old = gpu_capabilities(ComputeCapability::new(5, 2));
    assert!(! has(& old, axes::operands(), values::fp16()));
    assert!(! has(& old, axes::scoring(), values::int8_dp4a()));
    assert!(has(& old, axes::scoring(), values::fp32()));
    let fp16_only = gpu_capabilities(ComputeCapability::new(5, 3));
    assert!(has(& fp16_only, axes::operands(), values::fp16()));
    assert!(has(& fp16_only, axes::accumulator(), values::fp16()));
    assert!(has(& fp16_only, axes::scoring(), values::fp16()));
    assert!(! has(& fp16_only, axes::scoring(), values::int8_dp4a()));
    let dp4a = gpu_capabilities(ComputeCapability::new(6, 1));
    assert!(has(& dp4a, axes::scoring(), values::int8_dp4a()));
    for cc in [(5, 2), (5, 3), (6, 1), (7, 5), (12, 0)] {
        let caps = gpu_capabilities(ComputeCapability::new(cc.0, cc.1));
        assert!(caps.is_within(& gpu_reference()));
        assert!(has(& caps, axes::fusion(), values::on()));
        assert!(has(& caps, axes::fusion(), values::off()));
        assert!(caps.values(& axes::thread_count()).is_none());
    }
}
#[test]
fn tensor_core_rules() {
    let cc75 = ComputeCapability::new(7, 5);
    assert!(! has_tensor_cores(cc75, Some("NVIDIA GeForce GTX 1660 SUPER")));
    assert!(! has_tensor_cores(cc75, Some("nvidia geforce gtx 1650")));
    assert!(! has_tensor_cores(cc75, Some("GTX16xx")));
    assert!(! has_tensor_cores(cc75, Some("chip tu116-300")));
    assert!(! has_tensor_cores(cc75, Some("Tu117")));
    assert!(has_tensor_cores(cc75, Some("NVIDIA GeForce RTX 2060")));
    assert!(has_tensor_cores(cc75, None));
    assert!(has_tensor_cores(ComputeCapability::new(8, 6), Some("RTX 3090")));
    assert!(! has_tensor_cores(ComputeCapability::new(6, 1), None));
    assert!(! has_tensor_cores(ComputeCapability::new(6, 1), Some("Tesla P100")));
}
#[test]
fn builtin_catalog_contents() {
    let cpu = builtin_catalog(DeviceKind::Cpu);
    assert!(has(& cpu, axes::operands(), values::fp32()));
    assert!(has(& cpu, axes::operands(), values::fp16_storage_fp32_compute()));
    assert!(has(& cpu, axes::scoring(), values::fp32()));
    assert!(has(& cpu, axes::scoring(), values::int8()));
    assert!(has(& cpu, axes::scoring(), values::int8_vnni()));
    assert_eq!(cpu, cpu_reference());
    let gpu = builtin_catalog(DeviceKind::Gpu);
    assert_eq!(gpu, gpu_reference());
    for axis in [axes::batch_size(), axes::tile_size(), axes::thread_count()] {
        assert!(cpu.values(& axis).is_none());
        assert!(gpu.values(& axis).is_none());
    }
}
#[test]
fn reduce_keeps_unknown_axes_and_removes_unsupported_values() {
    let caps = CapabilitySet::new()
        .with(axes::scoring(), values::fp32())
        .with(axes::scoring(), values::int8())
        .with(axes::thread_count(), Name::from_number(4));
    let kernels = CapabilitySet::new().with(axes::scoring(), values::fp32());
    let out = reduce_to_kernels(caps, &kernels, "cpu");
    assert!(has(& out, axes::scoring(), values::fp32()));
    assert!(! has(& out, axes::scoring(), values::int8()));
    assert!(has(& out, axes::thread_count(), Name::from_number(4)));
}
#[test]
fn reduce_removes_empty_axis_and_never_adds() {
    let caps = CapabilitySet::new().with(axes::scoring(), values::int8());
    let kernels = CapabilitySet::new()
        .with(axes::scoring(), values::fp32())
        .with(axes::fusion(), values::on());
    let out = reduce_to_kernels(caps.clone(), &kernels, "cpu");
    assert!(out.is_empty());
    assert!(out.values(& axes::fusion()).is_none());
    assert!(out.is_within(& caps));
}
#[test]
fn gpu_reference_device_default_catalog() {
    let probe = reference_probe();
    let det = detect_capabilities(
            DeviceHandle::Gpu(0),
            Some(&probe),
            &CapabilityConfig::default(),
        )
        .unwrap();
    let c = &det.capabilities;
    assert_eq!(det.max_seq_len, 512);
    assert!(has(c, axes::operands(), values::fp32()));
    assert!(has(c, axes::operands(), values::fp16()));
    assert!(has(c, axes::scoring(), values::int8_dp4a()));
    assert!(has(c, axes::fusion(), values::on()));
    assert!(has(c, axes::fusion(), values::off()));
    assert_eq!(sorted(nums(c, & axes::batch_size())), vec![1, 8, 32]);
    assert_eq!(sorted(nums(c, & axes::tile_size())), vec![16, 32, 64, 128, 256, 512]);
    assert!(c.values(& axes::thread_count()).is_none());
    assert_eq!(c.axes().count(), 6);
    assert_eq!(c.iter().count(), 18);
    assert!(
        c.iter().all(| (_, v) | ! v.as_str().to_ascii_lowercase().contains("tensor"))
    );
}
#[test]
fn gpu_errors() {
    let cfg = CapabilityConfig::default();
    assert_eq!(
        detect_capabilities(DeviceHandle::Gpu(0), None, & cfg),
        Err(DetectError::NoGpuSupport)
    );
    let probe = FakeProbe { cc: None, name: None };
    assert_eq!(
        detect_capabilities(DeviceHandle::Gpu(0), Some(& probe), & cfg),
        Err(DetectError::NoDeviceContext)
    );
}
#[test]
fn gpu_probe_is_ignored_for_cpu() {
    let probe = FakeProbe { cc: None, name: None };
    let det = detect_capabilities(
            DeviceHandle::Cpu,
            Some(&probe),
            &CapabilityConfig::default(),
        )
        .unwrap();
    assert!(has(& det.capabilities, axes::operands(), values::fp32()));
    assert!(has(& det.capabilities, axes::scoring(), values::fp32()));
    assert!(det.capabilities.values(& axes::thread_count()).is_some());
}
#[test]
fn cpu_detection_is_deterministic_and_valid() {
    let cfg = CapabilityConfig::default();
    let a = detect_capabilities(DeviceHandle::Cpu, None, &cfg).unwrap();
    let b = detect_capabilities(DeviceHandle::Cpu, None, &cfg).unwrap();
    assert_eq!(a, b);
    for axis in a.capabilities.axes() {
        assert!(! a.capabilities.values(axis).unwrap().is_empty());
    }
    assert!(has(& a.capabilities, axes::scoring(), values::fp32()));
}
#[test]
fn numeric_axes_filtering_and_dedup() {
    let probe = reference_probe();
    let cfg = CapabilityConfig {
        batch_sizes: Some(vec![0, 4, 4, 2]),
        tile_sizes: Some(vec![0, 24, 100, 100, 200, 300]),
        max_seq_len: Some(200),
        kernels: None,
    };
    let det = detect_capabilities(DeviceHandle::Gpu(0), Some(&probe), &cfg).unwrap();
    assert_eq!(det.max_seq_len, 200);
    assert_eq!(sorted(nums(& det.capabilities, & axes::batch_size())), vec![2, 4]);
    assert_eq!(
        sorted(nums(& det.capabilities, & axes::tile_size())), vec![24, 100, 200]
    );
}
#[test]
fn empty_numeric_axes_are_not_created() {
    let probe = reference_probe();
    let cfg = CapabilityConfig {
        batch_sizes: Some(vec![]),
        tile_sizes: Some(vec![1000]),
        max_seq_len: Some(64),
        kernels: None,
    };
    let det = detect_capabilities(DeviceHandle::Gpu(0), Some(&probe), &cfg).unwrap();
    assert!(det.capabilities.values(& axes::batch_size()).is_none());
    assert!(det.capabilities.values(& axes::tile_size()).is_none());
    assert!(has(& det.capabilities, axes::scoring(), values::fp32()));
}
#[test]
fn given_kernels_replace_catalog_including_numeric_axes() {
    let probe = reference_probe();
    let kernels = CapabilitySet::new()
        .with(axes::scoring(), values::fp32())
        .with(axes::batch_size(), Name::from_number(8))
        .with(axes::tile_size(), Name::from_number(64));
    let cfg = CapabilityConfig {
        kernels: Some(kernels),
        ..Default::default()
    };
    let det = detect_capabilities(DeviceHandle::Gpu(0), Some(&probe), &cfg).unwrap();
    let c = &det.capabilities;
    assert_eq!(nums(c, & axes::batch_size()), vec![8]);
    assert_eq!(nums(c, & axes::tile_size()), vec![64]);
    assert!(has(c, axes::scoring(), values::fp32()));
    assert!(! has(c, axes::scoring(), values::fp16()));
    assert!(! has(c, axes::scoring(), values::int8_dp4a()));
    assert!(has(c, axes::operands(), values::fp16()));
    assert!(has(c, axes::fusion(), values::on()));
}
#[test]
fn given_kernels_can_remove_fp32_on_cpu() {
    let kernels = CapabilitySet::new().with(axes::scoring(), values::fp16());
    let cfg = CapabilityConfig {
        kernels: Some(kernels),
        ..Default::default()
    };
    let det = detect_capabilities(DeviceHandle::Cpu, None, &cfg).unwrap();
    assert!(! has(& det.capabilities, axes::scoring(), values::fp32()));
    assert!(det.capabilities.values(& axes::scoring()).is_none());
}
#[test]
fn detect_error_display() {
    assert!(DetectError::NoGpuSupport.to_string().contains("gpu"));
    assert!(DetectError::NoDeviceContext.to_string().contains("context"));
    assert!(DetectError::UnsupportedDevice("x".into()).to_string().contains('x'));
}
#[test]
fn concurrent_calls_are_independent() {
    let handles: Vec<_> = (0..4)
        .map(|_| {
            std::thread::spawn(|| {
                let probe = reference_probe();
                detect_capabilities(
                        DeviceHandle::Gpu(0),
                        Some(&probe),
                        &CapabilityConfig::default(),
                    )
                    .unwrap()
            })
        })
        .collect();
    let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();
    assert!(results.windows(2).all(| w | w[0] == w[1]));
}
