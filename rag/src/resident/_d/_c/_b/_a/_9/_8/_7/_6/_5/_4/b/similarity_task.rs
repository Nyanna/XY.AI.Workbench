use std::cmp::Ordering;
use std::collections::HashSet;
use std::fmt;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::a::resource_requirements::IndexId;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::a::result_kinds::{
    Hit, TaskResult,
};
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::a::index_contract::EntryId;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::_1::extensions::Extensions;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::_1::scopes_and_entry_sets::{
    EntrySet, Scope,
};
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::b::embedding_task::Precision;
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Scoring {
    #[default]
    Cosine,
    DotProduct,
    NegEuclidean,
}
#[derive(Debug, Clone, PartialEq)]
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
            index: self.index.clone(),
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
        index: &IndexId,
        scope: &Scope,
        precision: Precision,
        extensions: &Extensions,
    ) -> Result<Vec<Candidate>, SourceError>;
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceError(pub String);
#[derive(Debug, Clone, Default)]
pub struct SimilarityOutput {
    pub per_query: Vec<Vec<Hit>>,
}
impl SimilarityOutput {
    pub fn narrowed_scope(&self) -> Scope {
        Scope::Set(
            EntrySet::from_entry_ids(self.per_query.iter().flatten().map(|h| h.id)),
        )
    }
    pub fn into_task_result(self) -> TaskResult {
        TaskResult::Hits(self.per_query.into_iter().flatten().collect())
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
        Scoring::NegEuclidean => -sq_dist(a, b).sqrt(),
        _ => {
            let na = dot(a, a).sqrt();
            let nb = dot(b, b).sqrt();
            if na == 0.0 || nb == 0.0 { 0.0 } else { dot(a, b) / (na * nb) }
        }
    }
}
fn rank_order(a: &Hit, b: &Hit) -> Ordering {
    match (a.score.is_nan(), b.score.is_nan()) {
        (true, true) => a.id.cmp(&b.id),
        (true, false) => Ordering::Greater,
        (false, true) => Ordering::Less,
        (false, false) => {
            b.score
                .partial_cmp(&a.score)
                .unwrap_or(Ordering::Equal)
                .then_with(|| a.id.cmp(&b.id))
        }
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
        return Err(SimilarityError::EmptyQueryVector {
            query,
        });
    }
    let candidates = source
        .candidates(&params.index, &params.scope, params.precision, &params.extensions)
        .map_err(SimilarityError::Source)?;
    let mut seen = HashSet::with_capacity(candidates.len());
    let unique: Vec<&Candidate> = candidates
        .iter()
        .filter(|c| seen.insert(c.id))
        .collect();
    let per_query = queries
        .iter()
        .map(|q| {
            if params.top_k == 0 {
                return Vec::new();
            }
            let mut hits: Vec<Hit> = unique
                .iter()
                .filter(|c| c.vector.len() == q.len())
                .map(|c| Hit {
                    id: c.id,
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
    assert_traits::<SimilarityOutput>();
    assert_traits::<SimilarityError>();
};
#[cfg(test)]
#[path = "test/similarity_task.rs"]
mod tests;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::a::cpu_compute::{
    dot, sq_dist,
};
