//! CapabilitySet/Profile.
use std::collections::BTreeSet;
/// Named capabilities; an empty set is satisfied by any provider.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct CapabilitySet(BTreeSet<String>);
impl CapabilitySet {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with(mut self, cap: impl Into<String>) -> Self {
        self.0.insert(cap.into());
        self
    }
    pub fn contains(&self, cap: &str) -> bool {
        self.0.contains(cap)
    }
    /// `self` (required) is a subset of `available`.
    pub fn satisfies(&self, available: &CapabilitySet) -> bool {
        self.0.is_subset(&available.0)
    }
}
