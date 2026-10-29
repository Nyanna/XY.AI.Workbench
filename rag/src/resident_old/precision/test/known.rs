use super::*;

#[test]
fn gpu_offers_no_bf16() {
    let gpu = gpu_sm75();
    assert!(gpu.contains(&OPERANDS, &FP16));
    assert!(gpu.contains(&SCORING, &INT8_DP4A));
    assert!(!gpu.contains(&OPERANDS, &ValueId::new("bf16")));
    assert!(!gpu.contains(&ACCUMULATOR, &ValueId::new("bf16")));
}

#[test]
fn gpu_satisfies_cpu_independent_requirements() {
    let req = CapabilitySet::new().with(&*OPERANDS, &*FP16).with(&*FUSION, &*ON);
    assert!(gpu_sm75().satisfies(&req));
    assert!(!cpu().satisfies(&req));
}
