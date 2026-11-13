# Specification: Capability-Modell (build order 5)

## 1. Task, restrictions and boundaries

Implement a Rust library module with these parts:
- Extensible capability sets of axes and values. There is no profile enum.
- A requirement check against a set.
- Predefined axis and value names.
- Two reference sets, one for GPU and one for CPU, that serve as upper bounds.

Restrictions:
- The module has no dependencies on other modules.
- The module knows no profile names such as "gpu-fast".
- The module contains no numeric axis values in the reference sets. Numeric axes (batch size, tile size, thread count, density) are defined as axis names only. Their values are determined at runtime by others.
- The module does no I/O, no hardware detection, and no kernel or task logic.
- The module has no global mutable state.
- Optional crates: `serde` (derive) only.

## 2. Context

Other modules (facade, task interface, hardware backends) use this module to declare, compare and check what a backend or task supports or requires. A new hardware backend adds its own axes and values purely as strings. This requires no change to this module, the facade or the task interface. The module is the lowest layer, so it has no knowledge of its callers.

## 3. Interface

```rust
use std::sync::Arc;

/// Free-form identifier. Cheap clone (Arc<str>). Ord/Eq/Hash by string content (byte-wise lexicographic).
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Name(/* Arc<str> */);
impl Name {
    pub fn new(s: impl AsRef<str>) -> Self;
    pub fn as_str(&self) -> &str;
}
impl std::fmt::Display for Name {}            // prints the raw string
impl From<&str> for Name {}
impl From<String> for Name {}
impl From<u64> for Name {}                    // decimal string, e.g. 128 -> "128"
impl AsRef<str> for Name {}
impl std::borrow::Borrow<str> for Name {}
// serde: Serialize/Deserialize as plain string

pub type Axis = Name;
pub type Value = Name;
impl Name { pub fn from_number(n: u64) -> Self; }   // same as From<u64>

/// Capability set: axis -> non-empty set of values. Also used as the type of "Requirements".
#[derive(Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct CapabilitySet(/* BTreeMap<Axis, BTreeSet<Value>> */);
pub type Requirements = CapabilitySet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CapabilityError { EmptyValueSet { axis: Axis } }   // only from deserialization
impl std::fmt::Display for CapabilityError {}
impl std::error::Error for CapabilityError {}

impl CapabilitySet {
    pub fn new() -> Self;                                   // empty
    /// Adds value to axis (creates axis if absent). Returns true if newly inserted.
    pub fn add(&mut self, axis: impl Into<Axis>, value: impl Into<Value>) -> bool;
    pub fn with(self, axis: impl Into<Axis>, value: impl Into<Value>) -> Self;  // builder form of add
    pub fn contains(&self, axis: &Axis, value: &Value) -> bool;
    pub fn values(&self, axis: &Axis) -> Option<&std::collections::BTreeSet<Value>>; // None if axis absent
    pub fn axes(&self) -> impl Iterator<Item = &Axis>;                       // ascending
    pub fn iter(&self) -> impl Iterator<Item = (&Axis, &Value)>;             // ascending by (axis, value)
    pub fn is_empty(&self) -> bool;                         // true iff no axis
    /// true iff every (axis,value) of `req` is contained in self. Empty req -> true.
    pub fn satisfies(&self, req: &Requirements) -> bool;
    /// true iff self ⊆ upper, i.e. upper.satisfies(self).
    pub fn is_within(&self, upper: &CapabilitySet) -> bool;
}
impl FromIterator<(Axis, Value)> for CapabilitySet {}
impl Extend<(Axis, Value)> for CapabilitySet {}
// serde: Serialize as map axis -> sorted array of values; Deserialize rejects an empty array
// with CapabilityError::EmptyValueSet (as a serde custom error); duplicates in arrays are merged.

/// Predefined names (functions, because Name holds an Arc).
pub mod axes {
    pub fn operands() -> Axis;       // "operands"
    pub fn accumulator() -> Axis;    // "accumulator"
    pub fn scoring() -> Axis;        // "scoring"
    pub fn fusion() -> Axis;         // "fusion"
    pub fn batch_size() -> Axis;     // "batch_size"
    pub fn tile_size() -> Axis;      // "tile_size"
    pub fn thread_count() -> Axis;   // "thread_count"
    pub fn density() -> Axis;        // "density"
}
pub mod values {
    pub fn fp32() -> Value;                    // "fp32"
    pub fn fp16() -> Value;                    // "fp16"
    pub fn fp16_storage_fp32_compute() -> Value; // "fp16_storage_fp32_compute"
    pub fn int8() -> Value;                    // "int8"
    pub fn int8_dp4a() -> Value;               // "int8_dp4a"
    pub fn int8_vnni() -> Value;               // "int8_vnni"
    pub fn on() -> Value;                      // "on"
    pub fn off() -> Value;                     // "off"
}

pub fn gpu_reference() -> CapabilitySet;
pub fn cpu_reference() -> CapabilitySet;
```

## 4. Behavior and invariants

**Names**
- `Name` clone is O(1) and does not copy the string.
- Ordering, equality and hashing depend only on string content.
- `Name::from_number(n)` equals `Name::from(n.to_string())`. For example, `from_number(0)` is "0".
- Any string is valid, including an empty one. Construction never fails.
- Numeric names sort lexicographically, not numerically.

**CapabilitySet**
- Invariant: no axis ever maps to an empty value set. The API has no way to create one. An axis exists only if it has at least one value.
- `add` of an existing pair returns false and changes nothing.
- `contains` and `values` on an absent axis return false and `None`.
- Equality, hash, `iter`, `axes` and serialization do not depend on insertion order. Two sets with the same pairs are equal, hash equal, and serialize to identical output.
- `iter` yields pairs ordered by axis, then by value.
- `is_empty` is true only for a set with no axes.

**satisfies**
- True iff every required (axis, value) is present in the set.
- An empty requirement is always satisfied, including by an empty set.
- A required axis that is absent from the set makes the result false.
- Extra axes or values in the set are irrelevant.
- `satisfies` is a pure function.

**Reference sets**
- Each call returns a fresh, equal value.
- Each reference set is an upper bound. A concrete capability set is valid for that hardware iff `set.is_within(&ref)`.
- The reference sets contain only the following axes and values:
  - `gpu_reference()`:
    - operands = {fp32, fp16}
    - accumulator = {fp32, fp16}
    - scoring = {fp32, fp16, int8_dp4a}
    - fusion = {on, off}
  - `cpu_reference()`:
    - operands = {fp32, fp16_storage_fp32_compute}
    - scoring = {fp32, int8, int8_vnni}
- Neither reference set contains `batch_size`, `tile_size`, `thread_count` or `density`.
- The CPU "FP16 with conversion" is represented by `fp16_storage_fp32_compute`.
- The reference sets contain no profile names.

**Extensibility**
- New hardware defines further axes or values by using arbitrary `Name`s. The module needs no change for this.
- The predefined names are conveniences only. They are not a closed enum.

**Concurrency**
- `Name` and `CapabilitySet` are `Send + Sync`, because they use `Arc<str>` and standard collections.
- There is no interior mutability. Concurrent reads of a shared reference are safe. Mutation needs `&mut`.

**Errors**
- The only error is `CapabilityError::EmptyValueSet`, raised by deserializing an axis with an empty array. No operation panics.

## 5. Relied-on elements

None. The module has no dependencies.