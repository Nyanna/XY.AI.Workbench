use std::collections::BTreeMap;
use std::fmt;
use std::str::FromStr;

use super::capability_model::{Axis, CapabilitySet, Value};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CombinationError {
    /// Empty, containing ',' or '=', or with leading/trailing whitespace.
    InvalidName(String),
    DuplicateAxis(Axis),
    NotSingleValued { axis: Axis, count: usize },
    Parse(String),
}

impl fmt::Display for CombinationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CombinationError::InvalidName(name) => write!(f, "invalid name {name:?}"),
            CombinationError::DuplicateAxis(axis) => write!(f, "duplicate axis {axis}"),
            CombinationError::NotSingleValued { axis, count } => {
                write!(f, "axis {axis} carries {count} values, expected one")
            }
            CombinationError::Parse(text) => write!(f, "cannot parse combination: {text}"),
        }
    }
}

impl std::error::Error for CombinationError {}

/// One value per axis, plus the sequence lengths of the test set.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct Combination {
    values: BTreeMap<Axis, Value>,
    pub seq_lens: Vec<usize>,
}

/// Axis and value names must survive the text form.
pub fn check_name(name: &str) -> Result<(), CombinationError> {
    let bad = name.is_empty()
        || name.contains(',')
        || name.contains('=')
        || name.trim() != name;
    if bad {
        return Err(CombinationError::InvalidName(name.to_string()));
    }
    Ok(())
}

impl Combination {
    pub fn new() -> Self {
        Combination::default()
    }

    /// Validates every name; `seq_lens` stays empty.
    pub fn from_values(values: BTreeMap<Axis, Value>) -> Result<Self, CombinationError> {
        for (axis, value) in &values {
            check_name(axis.as_str())?;
            check_name(value.as_str())?;
        }
        Ok(Combination {
            values,
            seq_lens: Vec::new(),
        })
    }

    /// Replaces an existing value of the axis.
    pub fn set(
        &mut self,
        axis: impl Into<Axis>,
        value: impl Into<Value>,
    ) -> Result<(), CombinationError> {
        let axis = axis.into();
        let value = value.into();
        check_name(axis.as_str())?;
        check_name(value.as_str())?;
        self.values.insert(axis, value);
        Ok(())
    }

    pub fn with(
        mut self,
        axis: impl Into<Axis>,
        value: impl Into<Value>,
    ) -> Result<Self, CombinationError> {
        self.set(axis, value)?;
        Ok(self)
    }

    pub fn with_seq_lens(mut self, lens: Vec<usize>) -> Self {
        self.seq_lens = lens;
        self
    }

    pub fn value(&self, axis: &Axis) -> Option<&Value> {
        self.values.get(axis)
    }

    pub fn values(&self) -> &BTreeMap<Axis, Value> {
        &self.values
    }

    /// Ascending by axis.
    pub fn iter(&self) -> impl Iterator<Item = (&Axis, &Value)> {
        self.values.iter()
    }

    /// Every axis must carry exactly one value; `seq_lens` stays empty.
    pub fn from_capability(set: &CapabilitySet) -> Result<Self, CombinationError> {
        let mut combo = Combination::new();
        for axis in set.axes() {
            let count = set.values(axis).map_or(0, |v| v.len());
            let single = match set.values(axis).and_then(|v| v.iter().next()) {
                Some(v) if count == 1 => v,
                _ => {
                    return Err(CombinationError::NotSingleValued { axis: axis.clone(), count });
                }
            };
            combo.set(axis.clone(), single.clone())?;
        }
        Ok(combo)
    }

    pub fn to_capability(&self) -> CapabilitySet {
        let mut set = CapabilitySet::new();
        for (axis, value) in &self.values {
            set.add(axis.clone(), value.clone());
        }
        set
    }

    /// `axis=value` pairs ascending by axis, separated by `", "`; `seq_lens` is not part of it.
    pub fn to_text(&self) -> String {
        let pairs: Vec<String> = self
            .values
            .iter()
            .map(|(axis, value)| format!("{axis}={value}"))
            .collect();
        pairs.join(", ")
    }
}

impl fmt::Display for Combination {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_text())
    }
}

impl FromStr for Combination {
    type Err = CombinationError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let mut combo = Combination::new();
        if text.trim().is_empty() {
            return Ok(combo);
        }
        for pair in text.split(',') {
            let pair = pair.trim();
            let Some((axis, value)) = pair.split_once('=') else {
                return Err(CombinationError::Parse(format!("pair {pair:?} has no '='")));
            };
            if axis.is_empty() || value.is_empty() {
                return Err(CombinationError::Parse(format!("pair {pair:?} has an empty part")));
            }
            let axis = Axis::new(axis);
            if combo.value(&axis).is_some() {
                return Err(CombinationError::DuplicateAxis(axis));
            }
            combo.set(axis, Value::new(value))?;
        }
        Ok(combo)
    }
}

#[cfg(test)]
#[path = "test/combination.rs"]
mod tests;
