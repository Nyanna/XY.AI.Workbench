use super::*;
use crate::resident::kernel::catalog::implemented_capabilities;
use crate::resident::precision::known::{self, FP16, FP32, FUSION, INT8_DP4A, ON, OPERANDS, SCORING};

#[test]
fn intersect_drops_values_without_kernel() {
    let kernels = implemented_capabilities(SubengineKind::Cuda);
    let out = intersect_with_kernels(&known::gpu_sm75(), &kernels);
    assert!(out.contains(&SCORING, &FP16));
    assert!(!out.contains(&SCORING, &INT8_DP4A));
}

#[test]
fn ungoverned_axes_pass_unchanged() {
    let kernels = implemented_capabilities(SubengineKind::Cuda);
    let out = intersect_with_kernels(&known::gpu_sm75(), &kernels);
    assert_eq!(out.values(&FUSION), known::gpu_sm75().values(&FUSION));
}

#[test]
fn governed_axis_without_remaining_values_disappears() {
    let set = CapabilitySet::new().with(&*SCORING, &*INT8_DP4A).with(&*FUSION, &*ON);
    let kernels = CapabilitySet::new().with(&*SCORING, &*FP32);
    let out = intersect_with_kernels(&set, &kernels);
    assert!(out.values(&SCORING).is_none());
    assert!(out.contains(&FUSION, &ON));
}

#[test]
fn new_axes_and_values_need_no_signature_change() {
    let set = CapabilitySet::new().with("future-axis", "future-value");
    let kernels = CapabilitySet::new().with("future-axis", "future-value");
    assert_eq!(intersect_with_kernels(&set, &kernels), set);
}

#[test]
fn kernel_catalog_matches_implemented_dtypes() {
    let cpu = implemented_capabilities(SubengineKind::Cpu);
    assert!(cpu.contains(&OPERANDS, &FP32));
    assert!(!cpu.contains(&OPERANDS, &FP16));
    let cuda = implemented_capabilities(SubengineKind::Cuda);
    assert!(cuda.contains(&OPERANDS, &FP16));
}

#[test]
fn tiles_are_bounded_by_max_seq_len() {
    let cfg = ProbeConfig { max_seq_len: 64, ..ProbeConfig::default() };
    assert_eq!(cfg.tile_values().collect::<Vec<_>>(), [16, 32, 64]);
    assert_eq!(powers_of_two(16, 512), [16, 32, 64, 128, 256, 512]);
}

#[test]
fn filtered_probe_applies_kernel_set() {
    let fake = FakeCapabilityProbe { kind: SubengineKind::Cuda, set: known::gpu_sm75() };
    let probe = KernelFilteredProbe::new(fake, implemented_capabilities(SubengineKind::Cuda));
    let set = probe.probe().unwrap();
    assert_eq!(probe.kind(), SubengineKind::Cuda);
    assert!(!set.contains(&SCORING, &INT8_DP4A));
}

#[test]
fn cpu_factory_yields_cpu_probe_limited_to_kernels() {
    let probe = make_probe(&DeviceHandle::cpu(), ProbeConfig::default()).unwrap();
    assert_eq!(probe.kind(), SubengineKind::Cpu);
    let set = probe.probe().unwrap();
    assert!(set.contains(&OPERANDS, &FP32));
    assert!(!set.contains(&SCORING, &known::INT8));
}

#[cfg(not(feature = "cuda"))]
#[test]
fn cuda_probe_unavailable_without_feature() {
    assert!(super::super::probe_cuda::cuda_probe(&DeviceHandle::cpu(), ProbeConfig::default()).is_err());
}
