use std::hash::{Hash, Hasher};

use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::a::index_contract::EntryId;

#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ResultKind {
    Embeddings,
    EntryRefs,
    Hits,
    SimilarityReport,
    Scores,
}

impl ResultKind {
    pub fn name(self) -> &'static str {
        match self {
            ResultKind::Embeddings => "embeddings",
            ResultKind::EntryRefs => "entry_refs",
            ResultKind::Hits => "hits",
            ResultKind::SimilarityReport => "similarity_report",
            ResultKind::Scores => "scores",
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Hit {
    pub id: EntryId,
    pub score: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct SimilarityPair {
    pub a: EntryId,
    pub b: EntryId,
    pub score: f32,
}

#[derive(Debug, Clone, Default)]
pub struct SimilarityReport {
    pub pairs: Vec<SimilarityPair>,
}

#[non_exhaustive]
#[derive(Debug, Clone)]
pub enum TaskResult {
    Embeddings(Vec<Vec<f32>>),
    EntryRefs(Vec<EntryId>),
    Hits(Vec<Hit>),
    SimilarityReport(SimilarityReport),
    Scores(Vec<f32>),
}

impl TaskResult {
    pub fn kind(&self) -> ResultKind {
        match self {
            TaskResult::Embeddings(_) => ResultKind::Embeddings,
            TaskResult::EntryRefs(_) => ResultKind::EntryRefs,
            TaskResult::Hits(_) => ResultKind::Hits,
            TaskResult::SimilarityReport(_) => ResultKind::SimilarityReport,
            TaskResult::Scores(_) => ResultKind::Scores,
        }
    }

    pub fn as_embeddings(&self) -> Option<&[Vec<f32>]> {
        match self {
            TaskResult::Embeddings(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_entry_refs(&self) -> Option<&[EntryId]> {
        match self {
            TaskResult::EntryRefs(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_hits(&self) -> Option<&[Hit]> {
        match self {
            TaskResult::Hits(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_report(&self) -> Option<&SimilarityReport> {
        match self {
            TaskResult::SimilarityReport(r) => Some(r),
            _ => None,
        }
    }

    pub fn as_scores(&self) -> Option<&[f32]> {
        match self {
            TaskResult::Scores(v) => Some(v),
            _ => None,
        }
    }
}

fn slice_bits_eq(a: &[f32], b: &[f32]) -> bool {
    a.len() == b.len() && a.iter().zip(b).all(|(x, y)| x.to_bits() == y.to_bits())
}

fn hash_f32_slice<H: Hasher>(v: &[f32], state: &mut H) {
    v.len().hash(state);
    for x in v {
        x.to_bits().hash(state);
    }
}

impl PartialEq for Hit {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.score.to_bits() == other.score.to_bits()
    }
}
impl Eq for Hit {}
impl Hash for Hit {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.score.to_bits().hash(state);
    }
}

impl PartialEq for SimilarityPair {
    fn eq(&self, other: &Self) -> bool {
        self.a == other.a && self.b == other.b && self.score.to_bits() == other.score.to_bits()
    }
}
impl Eq for SimilarityPair {}
impl Hash for SimilarityPair {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.a.hash(state);
        self.b.hash(state);
        self.score.to_bits().hash(state);
    }
}

impl PartialEq for SimilarityReport {
    fn eq(&self, other: &Self) -> bool {
        self.pairs == other.pairs
    }
}
impl Eq for SimilarityReport {}
impl Hash for SimilarityReport {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.pairs.hash(state);
    }
}

impl PartialEq for TaskResult {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (TaskResult::Embeddings(a), TaskResult::Embeddings(b)) => {
                a.len() == b.len() && a.iter().zip(b).all(|(x, y)| slice_bits_eq(x, y))
            }
            (TaskResult::EntryRefs(a), TaskResult::EntryRefs(b)) => a == b,
            (TaskResult::Hits(a), TaskResult::Hits(b)) => a == b,
            (TaskResult::SimilarityReport(a), TaskResult::SimilarityReport(b)) => a == b,
            (TaskResult::Scores(a), TaskResult::Scores(b)) => slice_bits_eq(a, b),
            _ => false,
        }
    }
}
impl Eq for TaskResult {}
impl Hash for TaskResult {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.kind().hash(state);
        match self {
            TaskResult::Embeddings(v) => {
                v.len().hash(state);
                for inner in v {
                    hash_f32_slice(inner, state);
                }
            }
            TaskResult::EntryRefs(v) => v.hash(state),
            TaskResult::Hits(v) => v.hash(state),
            TaskResult::SimilarityReport(r) => r.hash(state),
            TaskResult::Scores(v) => hash_f32_slice(v, state),
        }
    }
}

const _: fn() = || {
    fn assert_traits<T: Send + Sync + 'static>() {}
    assert_traits::<ResultKind>();
    assert_traits::<Hit>();
    assert_traits::<SimilarityPair>();
    assert_traits::<SimilarityReport>();
    assert_traits::<TaskResult>();
};

#[cfg(test)]
#[path = "test/result_kinds.rs"]
mod tests;
