//! Capability sets: axes mapped to the values a provider offers or a task requires.

use std::collections::{BTreeMap, BTreeSet};

use super::axis::{AxisId, ValueId};

/// Ordered containers keep iteration, hashing and serialization reproducible.
/// Axes never map to an empty value set.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct CapabilitySet(BTreeMap<AxisId, BTreeSet<ValueId>>);

impl CapabilitySet {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with(mut self, axis: impl Into<AxisId>, value: impl Into<ValueId>) -> Self {
        self.insert(axis, value);
        self
    }

    pub fn insert(&mut self, axis: impl Into<AxisId>, value: impl Into<ValueId>) -> bool {
        self.0.entry(axis.into()).or_default().insert(value.into())
    }

    pub fn contains(&self, axis: &AxisId, value: &ValueId) -> bool {
        self.0.get(axis).is_some_and(|vs| vs.contains(value))
    }

    pub fn values(&self, axis: &AxisId) -> Option<&BTreeSet<ValueId>> {
        self.0.get(axis)
    }

    pub fn axes(&self) -> impl Iterator<Item = &AxisId> {
        self.0.keys()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&AxisId, &BTreeSet<ValueId>)> {
        self.0.iter()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Every required value on every required axis is present in `self`.
    pub fn satisfies(&self, req: &CapabilitySet) -> bool {
        req.0
            .iter()
            .all(|(axis, wanted)| self.0.get(axis).is_some_and(|have| wanted.is_subset(have)))
    }
}

#[cfg(test)]
mod tests {
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
}
