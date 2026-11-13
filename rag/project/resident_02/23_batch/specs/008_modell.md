# Modell: Specification

## 1. Task, restrictions and boundaries
**Task:** Crate module `model` with two parts.
- A resourceless, immutable description of a model as a pipeline (encoder stage, then scorer stage).
- Generic registries that resolve the names in a description to concrete encoders and scorers.

**Restrictions:**
- The module owns no resources: no buffers, device handles, files, threads, indices or kernels.
- It has no dependency on any other project module, and no reference to CPU, GPU, kernel or index types.
- It does not run encoding or scoring. It does not load weights or data.
- It does not define what an encoder or scorer does. The resolved entry types `E` and `S` are chosen by the consumer (a subengine) as generic parameters.
- It invents no further behavior, such as serialization formats or file loading.

## 2. Context
- The CPU and GPU subengines both use this module. Each creates its own `Registry<E>` and `Registry<S>` with its own entry types, for example a CPU encoder or a GPU scorer factory.
- Each subengine registers its entries, then resolves a `ModelSpec` into a `ResolvedModel<E, S>`. The result is a pair of entries, which the subengine uses.
- Because the description owns nothing, one `ModelSpec` can be shared between subengines (`Clone + Send + Sync`).

## 3. Interface
```rust
use std::{collections::BTreeMap, sync::Arc};

/// Registry key. Non-empty, compared exactly (case-sensitive).
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Name(/* String */);
impl Name {
    pub fn new(s: impl Into<String>) -> Result<Name, ModelError>; // Err(EmptyName) if empty
    pub fn as_str(&self) -> &str;
}

/// Plain parameter value attached to a stage.
#[derive(Clone, Debug, PartialEq)]
pub enum Param { Bool(bool), Int(i64), Float(f64), Str(String) }

/// One pipeline stage: a registry name plus parameters. Owns nothing else.
#[derive(Clone, Debug, PartialEq, Default)]
pub struct StageSpec {
    pub name: Option<Name>, // None only via Default; ModelSpec::new requires a name
    pub params: BTreeMap<String, Param>,
}
impl StageSpec {
    pub fn new(name: Name) -> Self;
    pub fn with_param(self, key: impl Into<String>, value: Param) -> Self;
}

/// Resourceless model description. Immutable after construction.
#[derive(Clone, Debug, PartialEq)]
pub struct ModelSpec { /* id, encoder, scorer */ }
impl ModelSpec {
    pub fn new(id: Name, encoder: StageSpec, scorer: StageSpec) -> Result<ModelSpec, ModelError>;
    pub fn id(&self) -> &Name;
    pub fn encoder(&self) -> &StageSpec;
    pub fn scorer(&self) -> &StageSpec;
    pub fn resolve<E, S>(&self, encoders: &Registry<E>, scorers: &Registry<S>)
        -> Result<ResolvedModel<E, S>, ModelError>;
}

/// Name -> entry map. Entries are shared via Arc.
pub struct Registry<T> { /* BTreeMap<Name, Arc<T>> */ }
impl<T> Registry<T> {
    pub fn new() -> Self;
    pub fn register(&mut self, name: Name, entry: Arc<T>) -> Result<(), ModelError>; // DuplicateName
    pub fn get(&self, name: &Name) -> Option<&Arc<T>>;
    pub fn contains(&self, name: &Name) -> bool;
    pub fn names(&self) -> impl Iterator<Item = &Name>; // ascending order
    pub fn len(&self) -> usize;
    pub fn is_empty(&self) -> bool;
}
impl<T> Default for Registry<T> {}

pub type EncoderRegistry<E> = Registry<E>;
pub type ScorerRegistry<S> = Registry<S>;

/// Result of resolution: the description plus shared references to the entries.
pub struct ResolvedModel<E, S> {
    pub spec: ModelSpec,    // clone of the resolved spec
    pub encoder: Arc<E>,
    pub scorer: Arc<S>,
}
impl<E, S> Clone for ResolvedModel<E, S> {}

#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum ModelError {
    #[error("empty name")] EmptyName,
    #[error("duplicate name: {0}")] DuplicateName(String),
    #[error("unknown encoder: {0}")] UnknownEncoder(String),
    #[error("unknown scorer: {0}")] UnknownScorer(String),
}
```
Bounds:
- `ModelSpec`, `StageSpec`, `Name` and `Param` are `Send + Sync + 'static`.
- `Registry<T>` and `ResolvedModel<E, S>` are `Send + Sync` whenever `T`, `E` and `S` are `Send + Sync`. This follows from `Arc` and requires no unsafe code.

## 4. Behavior and invariants
- `Name::new("")` returns `Err(EmptyName)`. No trimming or normalization is applied.
- `ModelSpec::new` fails with `EmptyName` if either stage has `name == None`. Otherwise it stores the arguments unchanged.
- `register` on an existing name returns `Err(DuplicateName(name))` and leaves the existing entry unchanged. There is no overwrite.
- `resolve` looks up the encoder name in `encoders` first, then the scorer name in `scorers`.
  - A missing encoder returns `Err(UnknownEncoder(name))`. This check wins when both are missing.
  - A missing scorer returns `Err(UnknownScorer(name))`.
  - On success the returned `Arc`s are clones of the registered ones (`Arc::ptr_eq` holds).
- `resolve` is pure. It does not mutate the spec or the registries, allocate resources or call entries.
- The same spec can be resolved any number of times and against different registries, for example one for CPU and one for GPU. The results are independent.
- Concurrency:
  - Registries are populated through `&mut` and read through `&`.
  - After population they can be shared across threads (for example in an `Arc`) for concurrent `get` and `resolve`. No internal locking is needed.
- `names()` iterates in ascending `Name` order, so it is deterministic.
- Dropping a `ResolvedModel` or `ModelSpec` releases nothing beyond the `Arc` reference counts.

## 5. Relied-on elements
None (no dependencies).