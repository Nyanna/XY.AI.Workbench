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
#[path = "test/capability.rs"]
mod tests;
