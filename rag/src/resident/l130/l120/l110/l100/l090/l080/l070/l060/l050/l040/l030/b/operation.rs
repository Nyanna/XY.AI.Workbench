use std::sync::{Arc, OnceLock};
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::l010::region_pinning::Pin;
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum OperationKind {
    Tokenization,
    Embedding,
    Pooling,
    L2Normalization,
    Similarity,
    TopK,
    IndexWrite,
    Inference,
    DeviceToRam,
}
/// Opaque back-reference from an operation to its owning task.
///
/// Two types named `TaskId` exist and they are not interchangeable:
/// - this struct, the typed ID stored in `Operation` and used in reports;
/// - `task_context::TaskId`, a plain `u64` alias used by `TaskContext`.
///
/// Both carry the same numeric value. Conversion is explicit: `.0` to get the raw `u64`, and
/// `TaskId(x)` to wrap one. Any further type with this name must be told apart the same way.
/// The value is assigned by the creator of the task and is never interpreted here.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TaskId(pub u64);
/// Identifier of an index entry as seen by operation results (`ScoredHit`, `RamData::HitList`).
///
/// Several types named `EntryId` exist and none of them is interchangeable:
/// - this struct, the operation-level ID used in scored results and reports;
/// - `index_contract::EntryId`, the struct used by the index contract and by task results;
/// - `scopes_and_entry_sets::EntryId`, a plain `u64` alias used for entry sets and load plans.
///
/// All three carry the same numeric value but are distinct types. Conversion is explicit at the
/// module boundary that needs it: `.0` to get the raw `u64`, and `EntryId(x)` to wrap one.
/// Any further type with this name must be told apart the same way and never re-exported from here.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EntryId(pub u64);
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct ResourceRequirements {
    pub resource: Option<String>,
    pub model: Option<String>,
    pub mode: Option<String>,
}
/// Immutable after construction; usable as a key.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OperationParams {
    requirements: ResourceRequirements,
    top_k: Option<usize>,
    dimension: Option<usize>,
}
impl OperationParams {
    pub fn new(requirements: ResourceRequirements) -> Self {
        Self {
            requirements,
            top_k: None,
            dimension: None,
        }
    }
    pub fn with_top_k(mut self, k: usize) -> Self {
        self.top_k = Some(k);
        self
    }
    pub fn with_dimension(mut self, dim: usize) -> Self {
        self.dimension = Some(dim);
        self
    }
    pub fn requirements(&self) -> &ResourceRequirements {
        &self.requirements
    }
    pub fn top_k(&self) -> Option<usize> {
        self.top_k
    }
    pub fn dimension(&self) -> Option<usize> {
        self.dimension
    }
}
/// Pure description of an operation; usable as a key.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OperationDescriptor {
    kind: OperationKind,
    params: OperationParams,
}
impl OperationDescriptor {
    pub fn new(kind: OperationKind, params: OperationParams) -> Self {
        Self { kind, params }
    }
    pub fn kind(&self) -> OperationKind {
        self.kind
    }
    pub fn params(&self) -> &OperationParams {
        &self.params
    }
    pub fn requirements(&self) -> &ResourceRequirements {
        self.params.requirements()
    }
}
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DataShape {
    TextList,
    TokenSequences,
    Floats,
    HitList,
    ScoredHitResult,
}
#[derive(Debug, Clone, PartialEq)]
pub struct ScoredHit {
    pub entry_id: EntryId,
    pub score: f32,
    pub stale: bool,
    pub source_link: Option<String>,
}
#[derive(Debug, Clone, PartialEq)]
pub struct ScoredHitResult {
    pub scored_count: usize,
    pub dropped_count: usize,
    pub hits: Vec<ScoredHit>,
}
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
pub enum RamData {
    TextList(Vec<String>),
    TokenSequences(Vec<Vec<u32>>),
    Floats(Vec<f32>),
    HitList(Vec<(EntryId, f32)>),
    ScoredHitResult(ScoredHitResult),
}
impl RamData {
    pub fn shape(&self) -> DataShape {
        match self {
            RamData::TextList(_) => DataShape::TextList,
            RamData::TokenSequences(_) => DataShape::TokenSequences,
            RamData::Floats(_) => DataShape::Floats,
            RamData::HitList(_) => DataShape::HitList,
            RamData::ScoredHitResult(_) => DataShape::ScoredHitResult,
        }
    }
}
/// Shared write-once RAM slot. Clones refer to the same slot.
#[derive(Debug, Clone)]
pub struct RamSlot {
    cell: Arc<OnceLock<RamData>>,
    shape: DataShape,
}
impl RamSlot {
    pub fn empty(shape: DataShape) -> Self {
        Self {
            cell: Arc::new(OnceLock::new()),
            shape,
        }
    }
    pub fn filled(data: RamData) -> Self {
        let shape = data.shape();
        let cell = OnceLock::new();
        let _ = cell.set(data);
        Self {
            cell: Arc::new(cell),
            shape,
        }
    }
    pub fn shape(&self) -> DataShape {
        self.shape
    }
    pub fn get(&self) -> Option<&RamData> {
        self.cell.get()
    }
    pub fn set(&self, data: RamData) -> Result<(), RamData> {
        if data.shape() != self.shape {
            return Err(data);
        }
        self.cell.set(data)
    }
}
/// Input or output: data in RAM or a pin of a device region.
pub enum IoRegion<R> {
    Ram(RamSlot),
    Device { pin: Arc<Pin<R>>, shape: DataShape },
}
impl<R> Clone for IoRegion<R> {
    fn clone(&self) -> Self {
        match self {
            IoRegion::Ram(slot) => IoRegion::Ram(slot.clone()),
            IoRegion::Device { pin, shape } => {
                IoRegion::Device {
                    pin: Arc::clone(pin),
                    shape: *shape,
                }
            }
        }
    }
}
impl<R> IoRegion<R> {
    pub fn shape(&self) -> DataShape {
        match self {
            IoRegion::Ram(slot) => slot.shape(),
            IoRegion::Device { shape, .. } => *shape,
        }
    }
}
pub struct Operation<R> {
    descriptor: OperationDescriptor,
    input: IoRegion<R>,
    output: IoRegion<R>,
    priority: i32,
    task_id: TaskId,
}
impl<R> Operation<R> {
    pub fn new(
        task_id: TaskId,
        priority: i32,
        descriptor: OperationDescriptor,
        input: IoRegion<R>,
        output: IoRegion<R>,
    ) -> Self {
        Self {
            descriptor,
            input,
            output,
            priority,
            task_id,
        }
    }
    pub fn task_id(&self) -> TaskId {
        self.task_id
    }
    pub fn priority(&self) -> i32 {
        self.priority
    }
    pub fn descriptor(&self) -> &OperationDescriptor {
        &self.descriptor
    }
    pub fn kind(&self) -> OperationKind {
        self.descriptor.kind()
    }
    pub fn params(&self) -> &OperationParams {
        self.descriptor.params()
    }
    pub fn requirements(&self) -> &ResourceRequirements {
        self.descriptor.requirements()
    }
    pub fn input(&self) -> &IoRegion<R> {
        &self.input
    }
    pub fn output(&self) -> &IoRegion<R> {
        &self.output
    }
}
#[cfg(test)]
#[path = "test/operation.rs"]
mod tests;
