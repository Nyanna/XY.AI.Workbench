# Spec: Operation (build order 36)

## 1. Task, restrictions and boundaries

Implement in Rust the description of one computation step, `Operation<R>`, and its comparable key types `OperationParams` and `OperationDescriptor`.

**In scope**
- An operation has a kind, parameters, an input region, an output region, a priority and a task ID.
- The parameters carry resource requirements and optionally top-k and dimension.
- The descriptor is the pure description (kind plus parameters). It and the parameters are comparable and usable as keys.
- The module defines the RAM data shapes and the scored-hit types.

**Out of scope**
- Executing operations, scheduling, claiming, caches, dependency graphs and filtering logic. Those components only use the keys defined here.
- Ranking and summarizing hits.
- Tensors and raw pointers. They never appear in this API.
- `unsafe`.
- Interpreting or validating priority.

**Dependencies**
- Region-Pinning: `Pin<R>`.
- Task-Modell: only the vocabulary (the priority is `i32`, and `resource`, `model` and `mode` are `Option<String>`, as in `TaskParams`).

## 2. Context

- A Task (Task-Modell) holds its operations as data. Operation order within the task is the implicit graph: the output of operation *n* feeds operation *n+1*, wired by sharing a `RamSlot` or a device region between them.
- Workers filter on `OperationParams`/`requirements()` before claiming.
- Caches and dependency graphs use `OperationParams` and `OperationDescriptor` as keys.
- Device regions are generic (`R`). A device-side input or output is an `Arc<Pin<R>>`, so the region cannot be evicted while the operation references it.
- A scored-hit result is an intermediate result. Ranking and summarizing happen after it.
- An operation does not hold a `Task` handle, which avoids an Arc cycle (the task owns its operations). Its back reference is the `TaskId`.

## 3. Interface

```rust
use std::sync::{Arc, OnceLock};
use region_pinning::Pin;

/// Extensible.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum OperationKind {
    Tokenization, Embedding, Pooling, L2Normalization,
    Similarity, TopK, IndexWrite, Inference,
    DeviceToRam,            // explicit transfer device -> RAM
}

/// Opaque back-reference to the owning task. Assigned by the creator of the task/operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TaskId(pub u64);

/// Entry ID at manager level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EntryId(pub u64);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct ResourceRequirements {
    pub resource: Option<String>,
    pub model: Option<String>,
    pub mode: Option<String>,
}

/// Immutable after construction. Key-capable.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OperationParams { /* private: requirements, top_k: Option<usize>, dimension: Option<usize> */ }
impl OperationParams {
    /// top_k and dimension are None.
    pub fn new(requirements: ResourceRequirements) -> Self;
    /// Builder: returns a new value (consumes self).
    pub fn with_top_k(self, k: usize) -> Self;
    pub fn with_dimension(self, dim: usize) -> Self;
    pub fn requirements(&self) -> &ResourceRequirements;
    pub fn top_k(&self) -> Option<usize>;
    pub fn dimension(&self) -> Option<usize>;
}

/// Pure description. Key-capable.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OperationDescriptor { /* private: kind, params */ }
impl OperationDescriptor {
    pub fn new(kind: OperationKind, params: OperationParams) -> Self;
    pub fn kind(&self) -> OperationKind;
    pub fn params(&self) -> &OperationParams;
    /// == self.params().requirements()
    pub fn requirements(&self) -> &ResourceRequirements;
}

/// Extensible. Describes a data form.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DataShape { TextList, TokenSequences, Floats, HitList, ScoredHitResult }

#[derive(Debug, Clone, PartialEq)]
pub struct ScoredHit {
    pub entry_id: EntryId,
    pub score: f32,
    pub stale: bool,
    pub source_link: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ScoredHitResult {
    pub scored_count: usize,   // entries scored in the resolved scope
    pub dropped_count: usize,  // hits discarded at resolution (entry removed)
    pub hits: Vec<ScoredHit>,  // remaining hits
}

/// Extensible. Data in RAM.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
pub enum RamData {
    TextList(Vec<String>),
    TokenSequences(Vec<Vec<u32>>),
    Floats(Vec<f32>),
    HitList(Vec<(EntryId, f32)>),
    ScoredHitResult(ScoredHitResult),
}
impl RamData { pub fn shape(&self) -> DataShape; }

/// Shared write-once RAM slot. Clone = same slot.
#[derive(Debug, Clone)]
pub struct RamSlot { /* Arc<OnceLock<RamData>> + shape: DataShape */ }
impl RamSlot {
    pub fn empty(shape: DataShape) -> Self;
    pub fn filled(data: RamData) -> Self;          // shape = data.shape()
    pub fn shape(&self) -> DataShape;
    pub fn get(&self) -> Option<&RamData>;
    /// Err(data) if already set or data.shape() != self.shape().
    pub fn set(&self, data: RamData) -> Result<(), RamData>;
}

/// Input or output: data in RAM or a pin of a device region. Never tensors or pointers.
pub enum IoRegion<R> {
    Ram(RamSlot),
    Device { pin: Arc<Pin<R>>, shape: DataShape },
}
impl<R> Clone for IoRegion<R> { /* no bound on R */ }
impl<R> IoRegion<R> { pub fn shape(&self) -> DataShape; }

pub struct Operation<R> { /* descriptor, input, output, priority, task_id */ }
impl<R> Operation<R> {
    pub fn new(task_id: TaskId, priority: i32, descriptor: OperationDescriptor,
               input: IoRegion<R>, output: IoRegion<R>) -> Self;
    pub fn task_id(&self) -> TaskId;
    pub fn priority(&self) -> i32;
    pub fn descriptor(&self) -> &OperationDescriptor;
    pub fn kind(&self) -> OperationKind;
    pub fn params(&self) -> &OperationParams;
    pub fn requirements(&self) -> &ResourceRequirements;
    pub fn input(&self) -> &IoRegion<R>;
    pub fn output(&self) -> &IoRegion<R>;
}
```

`Operation<R>` is `Send + Sync` iff `R: Send + Sync`. All other types are `Send + Sync`. `RamSlot` is `Send + Sync`.

## 4. Behavior and invariants

**Keys**
- `OperationParams` and `OperationDescriptor` implement `Eq` and `Hash` consistently. They are equal exactly when all fields are equal.
- `top_k` and `dimension` are `None` after `new`. `None` is not equal to `Some(_)`, and `with_top_k(3) != with_top_k(4)`.
- A key never changes. Builders consume `self`, and there are no setters.
- Priority, task ID and regions are not part of the keys. Two operations with equal kind and params have equal descriptors.
- `requirements()` always equals the requirements stored in the params. There is no separate source for them.

**Operation**
- An operation belongs to exactly one task, identified by the `task_id` it was constructed with. The metadata (priority, task_id) is immutable.
- Operation order within the task is the only graph. This module stores no edges.
- `new` does not validate that kind and shapes fit together, and it never panics.

**RAM slots**
- `RamSlot::set` succeeds exactly once. A second `set`, or a `set` with a different shape, returns `Err(data)` and leaves the slot unchanged.
- Concurrent `set` calls on one slot: exactly one wins.
- `get` returns `None` until set, and afterwards always the same value.
- A clone shares the slot, so a write via one handle is visible via all.
- `RamData::shape()` maps each variant to its `DataShape`: `TextList`, `TokenSequences`, `Floats`, `HitList` and `ScoredHitResult`.

**Device regions**
- While an `IoRegion::Device` (or a clone of it) is alive, the `Pin` stays alive, so the region is not evicted. The pin is released when the last clone is dropped.

**Scored result**
- `ScoredHitResult` carries counts and remaining hits only. It does no ranking and no sorting.
- `hits.len() <= scored_count` is expected, but not enforced.
- `ScoredHit.stale` and `source_link` are plain data.

**Concurrency and errors**
- All accessors are `&self` and lock-free. An operation can be read from many threads at once.
- No function panics. The only error is the `Err(RamData)` of `RamSlot::set`.

## 5. Relied-on elements

From Region-Pinning:
- `pub struct Pin<R>` (alias `DeviceHandle<R>`). It is RAII and not `Clone`. `Pin::region(&self) -> &R`.
- While a pin exists, the region's pin count is above 0 and the region must not be evicted.
- `Pin<R>` is `Send + Sync` iff `R: Send + Sync`.

From Task-Modell (vocabulary only):
- Priority is `i32` (higher means more important). `resource`, `model` and `mode` are `Option<String>`.
- A task holds its data, which can include operations. This module needs no `Task` type.