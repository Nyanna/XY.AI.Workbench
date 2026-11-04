use super::*;
use crate::resident::precision::known::{FUSION, ON};
use crate::resident::precision::{AxisId, ValueId};
fn tu116() -> CapabilitySet {
    cuda_capabilities(7, 5, &ProbeConfig::default())
}
#[test]
fn sm75_lists_fp16_and_dp4a_but_no_bf16() {
    let set = tu116();
    assert!(set.contains(& OPERANDS, & FP16));
    assert!(set.contains(& ACCUMULATOR, & FP16));
    assert!(set.contains(& SCORING, & INT8_DP4A));
    assert!(set.contains(& FUSION, & ON));
    assert!(! set.contains(& OPERANDS, & ValueId::new("bf16")));
}
#[test]
fn no_tensor_core_path_is_listed() {
    let set = tu116();
    for (_, values) in set.iter() {
        assert!(
            values.iter().all(| v | ! v.as_str().contains("wmma") && ! v.as_str()
            .contains("mma"))
        );
    }
}
#[test]
fn capability_gates_follow_compute_capability() {
    let cfg = ProbeConfig::default();
    let sm52 = cuda_capabilities(5, 2, &cfg);
    assert!(! sm52.contains(& OPERANDS, & FP16));
    assert!(! sm52.contains(& SCORING, & INT8_DP4A));
    let sm60 = cuda_capabilities(6, 0, &cfg);
    assert!(sm60.contains(& OPERANDS, & FP16));
    assert!(! sm60.contains(& SCORING, & INT8_DP4A));
    assert!(cuda_capabilities(6, 1, & cfg).contains(& SCORING, & INT8_DP4A));
}
#[test]
fn batch_and_tile_come_from_config() {
    let cfg = ProbeConfig {
        batches: vec![4],
        tiles: vec![32, 1024],
        max_seq_len: 128,
        kernels: None,
    };
    let set = cuda_capabilities(7, 5, &cfg);
    assert_eq!(set.values(& AxisId::new("batch")).unwrap().len(), 1);
    assert_eq!(set.values(& AxisId::new("tile")).unwrap().len(), 1);
}
#[test]
fn tensor_cores_are_never_derived_from_compute_capability_alone() {
    assert!(! has_tensor_cores(7, "NVIDIA GeForce GTX 1660 SUPER"));
    assert!(! has_tensor_cores(7, "NVIDIA GeForce GTX 1650"));
    assert!(has_tensor_cores(7, "NVIDIA GeForce RTX 2060"));
    assert!(! has_tensor_cores(6, "Tesla P100"));
}
#[cfg(feature = "cuda")]
#[test]
fn real_device_probe_matches_hardware_table() {
    use crate::resident::precision::make_probe;
    let Ok(device) = DeviceHandle::cuda(0) else {
        return;
    };
    let probe = make_probe(&device, ProbeConfig::default()).unwrap();
    let set = probe.probe().unwrap();
    assert_eq!(probe.kind(), SubengineKind::Cuda);
    assert!(set.contains(& OPERANDS, & FP32));
    assert!(set.contains(& OPERANDS, & FP16));
    assert!(! set.contains(& SCORING, & INT8_DP4A));
}
