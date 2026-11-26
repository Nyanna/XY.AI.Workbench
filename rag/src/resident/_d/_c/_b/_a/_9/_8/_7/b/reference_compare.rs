use std::fmt;
use log::warn;
use super::super::_6::_5::query_batch::{DocSource, QueryBatch, SegmentId, Workspace};
use super::super::_6::_5::_4::_3::_2::_1::layout::DType;
use super::super::_6::topk::{Hit, RangeScorer, TopKError};
/// Absolute score tolerance for the compute type; `None` for types without a scoring path.
pub fn tolerance(dtype: DType) -> Option<f32> {
    match dtype {
        DType::F32 => Some(1e-5),
        DType::F16 => Some(1e-3),
        _ => None,
    }
}
pub type ScorerError = Box<dyn std::error::Error + Send + Sync>;
pub trait DocRangeScorer: Send + Sync {
    /// Raw scores, query-major: `out[q * ws.len() + i]` is the score of query `q`
    /// for slot `ws.start() + i`.
    fn score_range(
        &self,
        src: &dyn DocSource,
        segment: SegmentId,
        ws: Workspace,
        batch: &QueryBatch,
    ) -> Result<Vec<f32>, ScorerError>;
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Candidate,
    Reference,
}
#[derive(Debug, Clone, PartialEq)]
pub enum DeviationKind {
    CountMismatch {
        candidate_len: usize,
        reference_len: usize,
        from_rank: usize,
        missing_on: Side,
    },
    ValueMismatch { rank: usize, candidate: f32, reference: f32, diff: f32 },
    RankMismatch {
        rank: usize,
        candidate_offset: u64,
        reference_offset: u64,
        reference_rank_of_candidate: Option<usize>,
    },
}
#[derive(Debug, Clone, PartialEq)]
pub struct Deviation {
    pub segment: SegmentId,
    pub query: usize,
    pub kind: DeviationKind,
}
impl fmt::Display for Deviation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f, "reference compare deviation: segment={} query={} ", self.segment.0, self
            .query
        )?;
        match &self.kind {
            DeviationKind::CountMismatch {
                candidate_len,
                reference_len,
                from_rank,
                missing_on,
            } => {
                write!(
                    f,
                    "count mismatch: candidate_len={candidate_len} reference_len={reference_len} from_rank={from_rank} missing_on={missing_on:?}"
                )
            }
            DeviationKind::ValueMismatch { rank, candidate, reference, diff } => {
                write!(
                    f,
                    "value mismatch: rank={rank} candidate={candidate} reference={reference} diff={diff}"
                )
            }
            DeviationKind::RankMismatch {
                rank,
                candidate_offset,
                reference_offset,
                reference_rank_of_candidate,
            } => {
                write!(
                    f,
                    "rank mismatch: rank={rank} candidate_offset={candidate_offset} reference_offset={reference_offset} reference_rank_of_candidate={reference_rank_of_candidate:?}"
                )
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct CompareReport {
    pub candidate: Vec<Vec<Hit>>,
    pub reference: Vec<Vec<Hit>>,
    pub deviations: Vec<Deviation>,
}
impl CompareReport {
    pub fn is_clean(&self) -> bool {
        self.deviations.is_empty()
    }
}
#[derive(Debug)]
pub enum CompareError {
    Candidate(ScorerError),
    Reference(ScorerError),
    TopK(TopKError),
    UnsupportedDType(DType),
}
impl fmt::Display for CompareError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CompareError::Candidate(e) => write!(f, "candidate scorer failed: {e}"),
            CompareError::Reference(e) => write!(f, "reference scorer failed: {e}"),
            CompareError::TopK(e) => write!(f, "top-k reduction failed: {e}"),
            CompareError::UnsupportedDType(d) => {
                write!(f, "no comparison tolerance for {d:?}")
            }
        }
    }
}
impl std::error::Error for CompareError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            CompareError::Candidate(e) | CompareError::Reference(e) => Some(e.as_ref()),
            CompareError::TopK(e) => Some(e),
            CompareError::UnsupportedDType(_) => None,
        }
    }
}
impl From<TopKError> for CompareError {
    fn from(e: TopKError) -> Self {
        CompareError::TopK(e)
    }
}
#[allow(clippy::too_many_arguments)]
pub fn compare_scorers(
    candidate: &dyn DocRangeScorer,
    reference: &dyn DocRangeScorer,
    src: &dyn DocSource,
    segment: SegmentId,
    workspaces: &[Workspace],
    batch: &QueryBatch,
    k: usize,
    dtype: DType,
) -> Result<CompareReport, CompareError> {
    let tol = tolerance(dtype).ok_or(CompareError::UnsupportedDType(dtype))?;
    let mut cand_top = RangeScorer::new(batch.len(), k);
    let mut ref_top = RangeScorer::new(batch.len(), k);
    for &ws in workspaces {
        if ws.is_empty() {
            continue;
        }
        let scores = candidate
            .score_range(src, segment, ws, batch)
            .map_err(CompareError::Candidate)?;
        cand_top.add_range(src, segment, ws, &scores)?;
        let scores = reference
            .score_range(src, segment, ws, batch)
            .map_err(CompareError::Reference)?;
        ref_top.add_range(src, segment, ws, &scores)?;
    }
    let candidate_hits = cand_top.into_results();
    let reference_hits = ref_top.into_results();
    let mut deviations = Vec::new();
    for (query, (c, r)) in candidate_hits.iter().zip(&reference_hits).enumerate() {
        deviations.extend(compare_hits(segment, query, c, r, tol));
    }
    for d in &deviations {
        warn!("{d}");
    }
    Ok(CompareReport {
        candidate: candidate_hits,
        reference: reference_hits,
        deviations,
    })
}
fn within(a: f32, b: f32, tol: f32) -> bool {
    (f64::from(a) - f64::from(b)).abs() <= f64::from(tol)
}
pub fn compare_hits(
    segment: SegmentId,
    query: usize,
    candidate: &[Hit],
    reference: &[Hit],
    tol: f32,
) -> Vec<Deviation> {
    let n = candidate.len().min(reference.len());
    let mut out = Vec::new();
    for i in 0..n {
        let (c, r) = (candidate[i], reference[i]);
        if !within(c.score, r.score, tol) {
            out.push(Deviation {
                segment,
                query,
                kind: DeviationKind::ValueMismatch {
                    rank: i,
                    candidate: c.score,
                    reference: r.score,
                    diff: (f64::from(c.score) - f64::from(r.score)).abs() as f32,
                },
            });
        } else if c.offset != r.offset {
            let j = reference.iter().position(|h| h.offset == c.offset);
            let near_tie = j
                .is_some_and(|j| {
                    let (lo, hi) = (i.min(j), i.max(j));
                    (lo..hi)
                        .all(|m| within(reference[m].score, reference[m + 1].score, tol))
                });
            if !near_tie {
                out.push(Deviation {
                    segment,
                    query,
                    kind: DeviationKind::RankMismatch {
                        rank: i,
                        candidate_offset: c.offset,
                        reference_offset: r.offset,
                        reference_rank_of_candidate: j,
                    },
                });
            }
        }
    }
    if candidate.len() != reference.len() {
        let missing_on = if candidate.len() < reference.len() {
            Side::Candidate
        } else {
            Side::Reference
        };
        out.push(Deviation {
            segment,
            query,
            kind: DeviationKind::CountMismatch {
                candidate_len: candidate.len(),
                reference_len: reference.len(),
                from_rank: n,
                missing_on,
            },
        });
    }
    out
}
#[cfg(test)]
#[path = "test/reference_compare.rs"]
mod tests;
