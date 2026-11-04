use super::*;
#[test]
fn engine_and_caps_matching() {
    let req = Requirements::new(ResourceKind::Encoder, EngineSelector::Gpu)
        .with_caps(CapabilitySet::new().with("operands", "fp16"));
    let have = CapabilitySet::new().with("operands", "fp16").with("fusion", "on");
    assert!(req.satisfied_by(DeviceKind::Gpu, & have));
    assert!(! req.satisfied_by(DeviceKind::Cpu, & have));
    assert!(! req.satisfied_by(DeviceKind::Gpu, & CapabilitySet::new()));
}
#[test]
fn either_accepts_both_and_empty_caps_pass() {
    let req = Requirements::new(ResourceKind::Tokenizer, EngineSelector::Either);
    let none = CapabilitySet::new();
    assert!(req.satisfied_by(DeviceKind::Cpu, & none));
    assert!(req.satisfied_by(DeviceKind::Gpu, & none));
}
#[test]
fn affinity_does_not_affect_matching() {
    let req = Requirements::new(ResourceKind::Scorer, EngineSelector::Either)
        .with_affinity(Affinity::Device(DeviceId(3)));
    assert!(req.satisfied_by(DeviceKind::Cpu, & CapabilitySet::new()));
}
