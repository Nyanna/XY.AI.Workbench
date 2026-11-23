use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::a::resource_requirements::ResourceRequirements;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::a::index_contract::EntryId;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::b::embedding_task::EmbeddingMode;
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
/// Immutable after construction; usable as a key.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OperationParams {
    requirements: ResourceRequirements,
    mode: Option<EmbeddingMode>,
    top_k: Option<usize>,
    dimension: Option<usize>,
}
impl OperationParams {
    pub fn new(requirements: ResourceRequirements) -> Self {
        Self {
            requirements,
            mode: None,
            top_k: None,
            dimension: None,
        }
    }
    pub fn with_mode(mut self, mode: EmbeddingMode) -> Self {
        self.mode = Some(mode);
        self
    }
    pub fn with_top_k(mut self, k: usize) -> Self {
        self.top_k = Some(k);
        self
    }
    pub fn with_dimension(mut self, dim: usize) -> Self {
        self.dimension = Some(dim);
        self
    }
    pub fn with_requirements(mut self, requirements: ResourceRequirements) -> Self {
        self.requirements = requirements;
        self
    }
    pub fn requirements(&self) -> &ResourceRequirements {
        &self.requirements
    }
    pub fn mode(&self) -> Option<EmbeddingMode> {
        self.mode
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
#[cfg(test)]
#[path = "test/operation.rs"]
mod tests;
