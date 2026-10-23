//! Unnamed profiles: a position on the fast-to-precise scale plus a configuration.

use std::collections::BTreeMap;

use super::axis::{AxisId, ValueId};
use crate::resident::error::PrecisionError;

/// Exactly one value per chosen axis.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct ProfileConfig(BTreeMap<AxisId, ValueId>);

impl ProfileConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with(mut self, axis: impl Into<AxisId>, value: impl Into<ValueId>) -> Self {
        self.0.insert(axis.into(), value.into());
        self
    }

    pub fn get(&self, axis: &AxisId) -> Option<&ValueId> {
        self.0.get(axis)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&AxisId, &ValueId)> {
        self.0.iter()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Profile {
    /// 0 is fastest, 1 is most precise.
    pub position: f32,
    pub config: ProfileConfig,
}

impl Profile {
    /// Position of a lone profile.
    pub const SINGLE_POSITION: f32 = 0.5;

    pub fn new(position: f32, config: ProfileConfig) -> Result<Self, PrecisionError> {
        if !(0.0..=1.0).contains(&position) {
            return Err(PrecisionError::InvalidPrecision { requested: position });
        }
        Ok(Self { position, config })
    }
}

#[cfg(test)]
#[path = "test/profile.rs"]
mod tests;
