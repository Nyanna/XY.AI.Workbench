use std::cmp::Ordering;
use std::collections::HashSet;
use std::fmt;

use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::a::result_kinds::{Hit, TaskResult};
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::a::index_contract::EntryId;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::l010::extensions::Extensions;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::l010::scopes_and_entry_sets::{EntrySet, Scope};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IndexId(pub u64);

#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Scoring {
    #[default]
    Cosine,
    DotProduct,
    NegEuclidean,
}

#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Precision {
    #[default]
    F32,
    F16,
    Int8,
}

#[derive(Debug, Clone)]
pub struct SimilarityParams {
    pub index: IndexId,
    pub top_k: usize,
    pub scope: Scope,
    pub scoring: Scoring,
    pub precision: Precision,
    pub extensions: Extensions,
}

impl SimilarityParams {
    pub fn new(index: IndexId) -> Self {
        Self {
            index,
            top_k: 10,
            scope: Scope::All,
            scoring: Scoring::Cosine,
            precision: Precision::F32,
            extensions: Extensions::default(),
        }
    }

    pub fn with_top_k(mut self, k: usize) -> Self {
        self.top_k = k;
        self
    }

    pub fn with_scope(mut self, s: Scope) -> Self {
        self.scope = s;
        self
    }

    pub fn with_scoring(mut self, s: Scoring) -> Self {
        self.scoring = s;
        self
    }

    pub fn with_precision(mut self, p: Precision) -> Self {
        self.precision = p;
        self
    }

    pub fn with_extensions(mut self, e: Extensions) -> Self {
        self.extensions = e;
        self
    }

    pub fn rerank(&self, prev: &SimilarityOutput, scoring: Scoring) -> SimilarityParams {
        SimilarityParams {
            index: self.index,
            top_k: self.top_k,
            scope: prev.narrowed_scope(),
            scoring,
            precision: self.precision,
            extensions: self.extensions.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Candidate {
    pub id: EntryId,
    pub vector: Vec<f32>,
    pub stale: bool,
}

pub trait CandidateSource: Send + Sync {
    fn candidates(
        &self,
        index: IndexId,
        scope: &Scope,
        precision: Precision,
        extensions: &Extensions,
    ) -> Result<Vec<Candidate>, SourceError>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceError(pub String);

#[derive(Debug, Clone, Copy)]
pub struct SimilarityHit {
    pub entry: EntryId,
    pub score: f32,
    pub stale: bool,
}

#[derive(Debug, Clone, Default)]
pub struct SimilarityOutput {
    pub per_query: Vec<Vec<SimilarityHit>>,
}

impl SimilarityOutput {
    pub fn narrowed_scope(&self) -> Scope {
        Scope::Set(EntrySet::from_ids(
            self.per_query.iter().flatten().map(|h| h.entry.0),
        ))
    }

    pub fn into_task_result(self) -> TaskResult {
        TaskResult::Hits(
            self.per_query
                .into_iter()
                .flatten()
                .map(|h| Hit {
                    id: h.entry,
                    score: h.score,
                })
                .collect(),
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SimilarityError {
    EmptyQueryVector { query: usize },
    Source(SourceError),
}

impl fmt::Display for SourceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "candidate source error: {}", self.0)
    }
}

impl std::error::Error for SourceError {}

impl fmt::Display for SimilarityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SimilarityError::EmptyQueryVector { query } => {
                write!(f, "query vector {} is empty", query)
            }
            SimilarityError::Source(e) => write!(f, "{}", e),
        }
    }
}

impl std::error::Error for SimilarityError {}

fn score(scoring: Scoring, a: &[f32], b: &[f32]) -> f32 {
    match scoring {
        Scoring::DotProduct => dot(a, b),
        Scoring::NegEuclidean => {
            let s: f32 = a
                .iter()
                .zip(b)
                .map(|(x, y)| {
                    let d = x - y;
                    d * d
                })
                .sum();
            -s.sqrt()
        }
        _ => {
            let na = dot(a, a).sqrt();
            let nb = dot(b, b).sqrt();
            if na == 0.0 || nb == 0.0 {
                0.0
            } else {
                dot(a, b) / (na * nb)
            }
        }
    }
}

fn dot(a: &[f32], b: &[f32]) -> f32 {
    a.iter().zip(b).map(|(x, y)| x * y).sum()
}

fn rank_order(a: &SimilarityHit, b: &SimilarityHit) -> Ordering {
    match (a.score.is_nan(), b.score.is_nan()) {
        (true, true) => a.entry.cmp(&b.entry),
        (true, false) => Ordering::Greater,
        (false, true) => Ordering::Less,
        (false, false) => b
            .score
            .partial_cmp(&a.score)
            .unwrap_or(Ordering::Equal)
            .then_with(|| a.entry.cmp(&b.entry)),
    }
}

pub fn run_similarity(
    source: &dyn CandidateSource,
    queries: &[Vec<f32>],
    params: &SimilarityParams,
) -> Result<SimilarityOutput, SimilarityError> {
    if queries.is_empty() {
        return Ok(SimilarityOutput::default());
    }
    if let Some(query) = queries.iter().position(|q| q.is_empty()) {
        return Err(SimilarityError::EmptyQueryVector { query });
    }
    let candidates = source
        .candidates(
            params.index,
            &params.scope,
            params.precision,
            &params.extensions,
        )
        .map_err(SimilarityError::Source)?;

    let mut seen = HashSet::with_capacity(candidates.len());
    let unique: Vec<&Candidate> = candidates.iter().filter(|c| seen.insert(c.id)).collect();

    let per_query = queries
        .iter()
        .map(|q| {
            if params.top_k == 0 {
                return Vec::new();
            }
            let mut hits: Vec<SimilarityHit> = unique
                .iter()
                .filter(|c| c.vector.len() == q.len())
                .map(|c| SimilarityHit {
                    entry: c.id,
                    score: score(params.scoring, q, &c.vector),
                    stale: c.stale,
                })
                .collect();
            hits.sort_by(rank_order);
            hits.truncate(params.top_k);
            hits
        })
        .collect();
    Ok(SimilarityOutput { per_query })
}

const _: fn() = || {
    fn assert_traits<T: Send + Sync + 'static>() {}
    assert_traits::<IndexId>();
    assert_traits::<Scoring>();
    assert_traits::<Precision>();
    assert_traits::<SimilarityParams>();
    assert_traits::<Candidate>();
    assert_traits::<SourceError>();
    assert_traits::<SimilarityHit>();
    assert_traits::<SimilarityOutput>();
    assert_traits::<SimilarityError>();
};

#[cfg(test)]
#[path = "test/similarity_task.rs"]
mod tests;
