use super::*;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

fn hash_of(c: &CapabilitySet) -> u64 {
    let mut h = DefaultHasher::new();
    c.hash(&mut h);
    h.finish()
}

#[test]
fn satisfies_is_axiswise_subset() {
    let have = CapabilitySet::new()
        .with("operands", "fp32")
        .with("operands", "fp16")
        .with("fusion", "on");
    assert!(have.satisfies(&CapabilitySet::new().with("operands", "fp16")));
    assert!(have.satisfies(
        &CapabilitySet::new()
            .with("operands", "fp16")
            .with("operands", "fp32")
            .with("fusion", "on")
    ));
    assert!(!have.satisfies(&CapabilitySet::new().with("operands", "bf16")));
    assert!(!have.satisfies(
        &CapabilitySet::new()
            .with("operands", "fp16")
            .with("operands", "bf16")
    ));
}

#[test]
fn missing_axis_fails_and_empty_requirement_passes() {
    let have = CapabilitySet::new().with("operands", "fp16");
    assert!(!have.satisfies(&CapabilitySet::new().with("scoring", "fp16")));
    assert!(have.satisfies(&CapabilitySet::new()));
    assert!(CapabilitySet::new().satisfies(&CapabilitySet::new()));
    assert!(!CapabilitySet::new().satisfies(&have));
}

#[test]
fn insertion_order_does_not_affect_identity() {
    let a = CapabilitySet::new()
        .with("scoring", "fp16")
        .with("operands", "fp32")
        .with("operands", "fp16");
    let b = CapabilitySet::new()
        .with("operands", "fp16")
        .with("operands", "fp32")
        .with("scoring", "fp16");
    assert_eq!(a, b);
    assert_eq!(hash_of(&a), hash_of(&b));
    assert_eq!(format!("{a:?}"), format!("{b:?}"));
    assert_eq!(a.axes().map(AxisId::as_str).collect::<Vec<_>>(), ["operands", "scoring"]);
}

#[test]
fn unknown_axes_and_values_are_accepted() {
    let c = CapabilitySet::new().with("future-axis", "future-value");
    assert!(c.contains(&AxisId::new("future-axis"), &ValueId::new("future-value")));
    assert!(c.satisfies(&c.clone()));
}
