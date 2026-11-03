//! Extensible capability sets of axes and values with requirement checks.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::sync::Arc;

use serde::de::{Deserializer, Error as _};
use serde::{Deserialize, Serialize, Serializer};

/// Free-form identifier with cheap clones.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Name(Arc<str>);

impl Name {
    pub fn new(s: impl AsRef<str>) -> Self {
        Name(Arc::from(s.as_ref()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn from_number(n: u64) -> Self {
        Name::new(n.to_string())
    }
}

impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl From<&str> for Name {
    fn from(s: &str) -> Self {
        Name::new(s)
    }
}

impl From<String> for Name {
    fn from(s: String) -> Self {
        Name(Arc::from(s))
    }
}

impl From<u64> for Name {
    fn from(n: u64) -> Self {
        Name::from_number(n)
    }
}

impl AsRef<str> for Name {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::borrow::Borrow<str> for Name {
    fn borrow(&self) -> &str {
        &self.0
    }
}

impl Serialize for Name {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(&self.0)
    }
}

impl<'de> Deserialize<'de> for Name {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        String::deserialize(d).map(Name::from)
    }
}

pub type Axis = Name;
pub type Value = Name;

/// Map from axis to a non-empty set of values; also used as requirements.
#[derive(Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct CapabilitySet(BTreeMap<Axis, BTreeSet<Value>>);

pub type Requirements = CapabilitySet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CapabilityError {
    EmptyValueSet { axis: Axis },
}

impl fmt::Display for CapabilityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CapabilityError::EmptyValueSet { axis } => {
                write!(f, "axis '{}' has an empty value set", axis)
            }
        }
    }
}

impl std::error::Error for CapabilityError {}

impl CapabilitySet {
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a value to an axis; returns true if newly inserted.
    pub fn add(&mut self, axis: impl Into<Axis>, value: impl Into<Value>) -> bool {
        self.0.entry(axis.into()).or_default().insert(value.into())
    }

    pub fn with(mut self, axis: impl Into<Axis>, value: impl Into<Value>) -> Self {
        self.add(axis, value);
        self
    }

    pub fn contains(&self, axis: &Axis, value: &Value) -> bool {
        self.0.get(axis).is_some_and(|v| v.contains(value))
    }

    pub fn values(&self, axis: &Axis) -> Option<&BTreeSet<Value>> {
        self.0.get(axis)
    }

    pub fn axes(&self) -> impl Iterator<Item = &Axis> {
        self.0.keys()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&Axis, &Value)> {
        self.0.iter().flat_map(|(a, vs)| vs.iter().map(move |v| (a, v)))
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// True iff every (axis, value) of `req` is contained in self.
    pub fn satisfies(&self, req: &Requirements) -> bool {
        req.0.iter().all(|(axis, vals)| match self.0.get(axis) {
            Some(have) => vals.is_subset(have),
            None => false,
        })
    }

    /// True iff self is a subset of `upper`.
    pub fn is_within(&self, upper: &CapabilitySet) -> bool {
        upper.satisfies(self)
    }
}

impl FromIterator<(Axis, Value)> for CapabilitySet {
    fn from_iter<I: IntoIterator<Item = (Axis, Value)>>(iter: I) -> Self {
        let mut set = CapabilitySet::new();
        set.extend(iter);
        set
    }
}

impl Extend<(Axis, Value)> for CapabilitySet {
    fn extend<I: IntoIterator<Item = (Axis, Value)>>(&mut self, iter: I) {
        for (a, v) in iter {
            self.add(a, v);
        }
    }
}

impl Serialize for CapabilitySet {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.0.serialize(s)
    }
}

impl<'de> Deserialize<'de> for CapabilitySet {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let raw = BTreeMap::<Axis, Vec<Value>>::deserialize(d)?;
        let mut set = CapabilitySet::new();
        for (axis, vals) in raw {
            if vals.is_empty() {
                return Err(D::Error::custom(CapabilityError::EmptyValueSet { axis }));
            }
            for v in vals {
                set.add(axis.clone(), v);
            }
        }
        Ok(set)
    }
}

pub mod axes {
    use super::Axis;

    pub fn operands() -> Axis {
        Axis::new("operands")
    }
    pub fn accumulator() -> Axis {
        Axis::new("accumulator")
    }
    pub fn scoring() -> Axis {
        Axis::new("scoring")
    }
    pub fn fusion() -> Axis {
        Axis::new("fusion")
    }
    pub fn batch_size() -> Axis {
        Axis::new("batch_size")
    }
    pub fn tile_size() -> Axis {
        Axis::new("tile_size")
    }
    pub fn thread_count() -> Axis {
        Axis::new("thread_count")
    }
    pub fn density() -> Axis {
        Axis::new("density")
    }
}

pub mod values {
    use super::Value;

    pub fn fp32() -> Value {
        Value::new("fp32")
    }
    pub fn fp16() -> Value {
        Value::new("fp16")
    }
    pub fn fp16_storage_fp32_compute() -> Value {
        Value::new("fp16_storage_fp32_compute")
    }
    pub fn int8() -> Value {
        Value::new("int8")
    }
    pub fn int8_dp4a() -> Value {
        Value::new("int8_dp4a")
    }
    pub fn int8_vnni() -> Value {
        Value::new("int8_vnni")
    }
    pub fn on() -> Value {
        Value::new("on")
    }
    pub fn off() -> Value {
        Value::new("off")
    }
}

/// Upper bound of what any GPU backend may support.
pub fn gpu_reference() -> CapabilitySet {
    use axes::*;
    use values::*;
    let mut s = CapabilitySet::new();
    for v in [fp32(), fp16()] {
        s.add(operands(), v.clone());
        s.add(accumulator(), v);
    }
    for v in [fp32(), fp16(), int8_dp4a()] {
        s.add(scoring(), v);
    }
    for v in [on(), off()] {
        s.add(fusion(), v);
    }
    s
}

/// Upper bound of what any CPU backend may support.
pub fn cpu_reference() -> CapabilitySet {
    use axes::*;
    use values::*;
    let mut s = CapabilitySet::new();
    for v in [fp32(), fp16_storage_fp32_compute()] {
        s.add(operands(), v);
    }
    for v in [fp32(), int8(), int8_vnni()] {
        s.add(scoring(), v);
    }
    s
}

#[cfg(test)]
#[path = "test/capability_model.rs"]
mod tests;
