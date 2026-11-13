# Specification: Resource-Requirements (build order 54)

## 1. Task, restrictions and boundaries

Implement the Rust module `resource_requirements`. It is a pure value module that describes which resource an operation needs. The Planner passes this description to the sub-engines.

The module:
- defines the resource kind, model ID, index ID, execution environment, required capabilities and placement hint of an operation;
- checks whether a device can satisfy these requirements;
- provides the accessor trait through which a sub-engine reads an operation's requirements.

Out of scope:
- Planning, scheduling, device discovery and loading models or indexes.
- Resolving or validating region handles.
- Pull-filter logic. The hint is only carried along.
- Redefining `CapabilitySet`, `DeviceKind`, `DeviceHandle` or `RegionHandle`.
- I/O, global state, `unsafe`, and any behavior not listed here.

Dependencies are the Capability-Modell, Device/Lane/Event and Region-Handle modules. No other crates are needed.

## 2. Context

- **Planner.** It builds a `ResourceRequirements` value for each operation and hands it to the sub-engines.
- **Sub-engines.** Each one reads an operation's requirements through `HasResourceRequirements` and uses them to pick its responsible resource entry (Tokenizer, Normalizer, Encoder, Scorer, Index or Cross-Encoder). They also use `is_satisfied_by` to test candidate devices.
- **Identification.**
  - Models and indexes are identified by string IDs.
  - Devices are identified by a number (the GPU ordinal in `DeviceHandle`).
- **Pull filters.** They use the placement hint (affinity to a region or a device) only as a hint.
- **Dependencies.** `CapabilitySet` and `DeviceKind` are used verbatim from the dependency modules. `DeviceHandle` and `RegionHandle` appear only inside `Affinity`.

## 3. Interface

```rust
use capability_model::{Axis, CapabilitySet, Requirements, Value};
use device::{DeviceHandle, DeviceKind};
use region_handle::RegionHandle;

/// Resource kind (extensible).
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ResourceKind { Tokenizer, Normalizer, Encoder, Scorer, Index, CrossEncoder }

/// Execution environment (extensible).
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ExecutionEnvironment { CpuOnly, GpuOnly, Any }
impl ExecutionEnvironment {
    /// Any => true for every device kind; CpuOnly => only Cpu; GpuOnly => only Gpu.
    pub fn accepts(&self, kind: DeviceKind) -> bool;
}

/// String IDs. Compared, hashed and ordered by string content.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ModelId(String);
impl ModelId {
    pub fn new(id: impl Into<String>) -> Self;
    pub fn as_str(&self) -> &str;
}
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct IndexId(String);
impl IndexId {
    pub fn new(id: impl Into<String>) -> Self;
    pub fn as_str(&self) -> &str;
}
// Both: Display prints the raw string; From<&str>, From<String>.

/// Placement hint (extensible). Hint only, never a requirement.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Affinity {
    Region(RegionHandle),
    Device(DeviceHandle),
}
// Ord is implemented manually, see section 4.
impl PartialOrd for Affinity {}
impl Ord for Affinity {}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ResourceRequirements { /* kind, model, index, environment, capabilities, affinity */ }
// Ord is implemented manually, see section 4.
impl PartialOrd for ResourceRequirements {}
impl Ord for ResourceRequirements {}

impl ResourceRequirements {
    /// Only kind and environment are mandatory. model/index/affinity = None, capabilities = empty.
    pub fn new(kind: ResourceKind, environment: ExecutionEnvironment) -> Self;

    // Optional parts, added one at a time (builder style, consume and return self).
    pub fn with_model(self, model: ModelId) -> Self;                    // replaces any previous model
    pub fn with_index(self, index: IndexId) -> Self;                    // replaces any previous index
    pub fn with_affinity(self, affinity: Affinity) -> Self;             // replaces any previous affinity
    pub fn with_capability(self, axis: impl Into<Axis>, value: impl Into<Value>) -> Self; // adds one pair
    pub fn with_capabilities(self, caps: CapabilitySet) -> Self;        // merges (union) into existing

    pub fn kind(&self) -> ResourceKind;
    pub fn environment(&self) -> ExecutionEnvironment;
    pub fn model(&self) -> Option<&ModelId>;
    pub fn index(&self) -> Option<&IndexId>;
    pub fn capabilities(&self) -> &Requirements;
    pub fn affinity(&self) -> Option<&Affinity>;

    /// true iff environment.accepts(kind) && device_capabilities.satisfies(self.capabilities()).
    /// The affinity is ignored.
    pub fn is_satisfied_by(&self, kind: DeviceKind, device_capabilities: &CapabilitySet) -> bool;
}

/// Implemented by operations. A sub-engine uses the result to determine the
/// responsible resource entry.
pub trait HasResourceRequirements {
    fn resource_requirements(&self) -> &ResourceRequirements;
}
```

`ResourceRequirements`, `ResourceKind`, `ExecutionEnvironment`, `ModelId`, `IndexId` and `Affinity` are `Send + Sync + 'static`. A static test checks this.

## 4. Behavior and invariants

**Construction**
- `new(kind, env)` succeeds without any other data. It sets model, index and affinity to `None` and the capabilities to an empty set.
- Each `with_*` call changes only its own part:
  - `with_model`, `with_index` and `with_affinity` overwrite.
  - `with_capability` and `with_capabilities` only add. They never remove capabilities.
  - The call order does not matter for the result.
- There is no validation. Any combination is valid, and construction never fails or panics.

**Execution environment**
- `Any.accepts(k)` is `true` for every `DeviceKind`, including future ones.
- `CpuOnly` accepts only `DeviceKind::Cpu`.
- `GpuOnly` accepts only `DeviceKind::Gpu`.
- `CpuOnly` and `GpuOnly` return `false` for any other or unknown kind (wildcard arm, because `DeviceKind` is `#[non_exhaustive]`).

**Satisfiability**
- `is_satisfied_by(kind, caps)` is true iff both conditions hold:
  - the environment accepts `kind`;
  - `caps.satisfies(self.capabilities())`.
- Empty required capabilities are satisfied by every capability set, including an empty one.
- The affinity never influences the result. Two requirements that differ only in affinity give the same answer for every device.
- The function is pure and has no side effects.

**Comparison and key**
- `Eq` and `Hash` cover all six parts: kind, model, index, environment, capabilities and affinity.
  - The capability part is compared as a set. It is independent of insertion order.
  - The hash is consistent with `Eq`.
- Requirements that differ in any single part, including affinity, are unequal.
- `Ord` is a total order that is consistent with `Eq`. It is lexicographic in this part order:
  - kind;
  - model, where `None` sorts before `Some`;
  - index, with the same rule;
  - environment;
  - capabilities, compared by the sequences of their `iter()` pairs `(axis, value)`;
  - affinity, where `None` sorts before `Some`.
- Declaration order decides the `Ord` of `ResourceKind` and `ExecutionEnvironment`.
- `Affinity` order:
  - `Region` sorts before `Device`.
  - Regions are ordered by `RegionHandle::cmp`.
  - Devices are ordered with `Cpu` before every `Gpu(n)`, and `Gpu(n)` by ascending `n`.
- `ResourceRequirements` can be used as a `HashMap` or `BTreeMap` key.
- `Clone` is a deep copy of independent values.

**Accessor trait**
- `resource_requirements()` returns a reference to the operation's requirements.
- Repeated calls on an unchanged operation return equal values.

**Concurrency and errors**
- There is no internal state, no interior mutability, no locks and no globals.
- Shared references can be read concurrently.
- The module has no error type, and no operation panics.

## 5. Relied-on elements

From Capability-Modell (used unchanged):
```rust
pub type Axis = Name;  pub type Value = Name;   // Name: From<&str>/From<String>
pub type Requirements = CapabilitySet;
// CapabilitySet: Clone, PartialEq, Eq, Hash, Debug, Default. It does not implement Ord.
impl CapabilitySet {
    pub fn new() -> Self;
    pub fn add(&mut self, axis: impl Into<Axis>, value: impl Into<Value>) -> bool;
    pub fn with(self, axis: impl Into<Axis>, value: impl Into<Value>) -> Self;
    pub fn iter(&self) -> impl Iterator<Item = (&Axis, &Value)>;   // ascending by (axis, value)
    pub fn satisfies(&self, req: &Requirements) -> bool;           // empty req -> true
}
impl Extend<(Axis, Value)> for CapabilitySet {}
```
- `Name` is `Ord`. The set is `Send + Sync`.
- `with_capabilities` can merge by iterating `caps.iter()` and calling `add` with clones.

From Device und Lane und Event:
```rust
#[non_exhaustive] #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DeviceKind { Cpu, Gpu }
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DeviceHandle { Cpu, Gpu(u32) }
impl DeviceHandle { pub fn kind(&self) -> DeviceKind; pub fn ordinal(&self) -> Option<u32>; }
```
- `DeviceHandle` does not implement `Ord`, so `Affinity` needs a manual `Ord`.

From Region-Handle:
```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RegionHandle { /* arena, offset, len, generation */ }
```
- `RegionHandle` is a `Copy` value, `Send + Sync + 'static`. This module does not resolve it or check its generation.