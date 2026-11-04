use super::*;

#[test]
fn reference_matches_new() {
    assert_eq!(ComputeCapability::new(7, 5), REFERENCE_CC);
}

#[test]
fn jit_arch_strings() {
    assert_eq!(ComputeCapability::new(7, 5).jit_arch(), "sm_75");
    assert_eq!(ComputeCapability::new(8, 6).jit_arch(), "sm_86");
}

#[test]
fn acquire_support() {
    assert!(ComputeCapability::new(7, 0).supports_acquire_loads());
    assert!(ComputeCapability::new(7, 5).supports_acquire_loads());
    assert!(ComputeCapability::new(8, 0).supports_acquire_loads());
    assert!(!ComputeCapability::new(6, 1).supports_acquire_loads());
}

#[test]
fn jit_target() {
    assert_eq!(jit_target_for(REFERENCE_CC), Ok("sm_75".to_string()));
    assert_eq!(jit_target_for(ComputeCapability::new(12, 0)), Ok("sm_120".to_string()));
    let cc = ComputeCapability::new(6, 1);
    assert_eq!(jit_target_for(cc), Err(TargetEnvError::UnsupportedCapability(cc)));
}

#[test]
fn paths_not_offered() {
    assert_eq!(require_tensor_core_path(), Err(TargetEnvError::PathNotOffered("tensor_core")));
    assert_eq!(require_bf16_compute(), Err(TargetEnvError::PathNotOffered("bf16_compute")));
    assert!(!TENSOR_CORE_PATH && !BF16_COMPUTE);
}

#[test]
fn display_contents() {
    let s = TargetEnvError::UnsupportedCapability(ComputeCapability::new(6, 1)).to_string();
    assert!(s.contains("6.1"));
    assert!(TargetEnvError::PathNotOffered("tensor_core").to_string().contains("tensor_core"));
}

#[test]
fn send_sync() {
    fn check<T: Send + Sync + 'static>() {}
    check::<ComputeCapability>();
    check::<TargetEnvError>();
}
