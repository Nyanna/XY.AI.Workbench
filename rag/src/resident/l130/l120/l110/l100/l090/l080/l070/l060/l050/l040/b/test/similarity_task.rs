use super::*;

struct Fixed(Vec<Candidate>);

impl CandidateSource for Fixed {
    fn candidates(
        &self,
        _: IndexId,
        _: &Scope,
        _: Precision,
        _: &Extensions,
    ) -> Result<Vec<Candidate>, SourceError> {
        Ok(self.0.clone())
    }
}

struct Failing;

impl CandidateSource for Failing {
    fn candidates(
        &self,
        _: IndexId,
        _: &Scope,
        _: Precision,
        _: &Extensions,
    ) -> Result<Vec<Candidate>, SourceError> {
        Err(SourceError("boom".into()))
    }
}

fn cand(id: u64, v: &[f32], stale: bool) -> Candidate {
    Candidate {
        id: EntryId(id),
        vector: v.to_vec(),
        stale,
    }
}

fn params() -> SimilarityParams {
    SimilarityParams::new(IndexId(1))
}

#[test]
fn defaults() {
    let p = params();
    assert_eq!(p.top_k, 10);
    assert_eq!(p.scoring, Scoring::Cosine);
    assert_eq!(p.precision, Precision::F32);
    assert_eq!(p.scope, Scope::All);
}

#[test]
fn empty_queries_ok() {
    let out = run_similarity(&Failing, &[], &params()).unwrap();
    assert!(out.per_query.is_empty());
}

#[test]
fn empty_query_vector_errors_before_source() {
    let r = run_similarity(&Failing, &[vec![1.0], vec![]], &params());
    assert_eq!(r.unwrap_err(), SimilarityError::EmptyQueryVector { query: 1 });
}

#[test]
fn source_error_propagates() {
    let r = run_similarity(&Failing, &[vec![1.0]], &params());
    assert_eq!(
        r.unwrap_err(),
        SimilarityError::Source(SourceError("boom".into()))
    );
}

#[test]
fn ranking_ties_stale_and_dimension() {
    let src = Fixed(vec![
        cand(3, &[1.0, 0.0], false),
        cand(2, &[1.0, 0.0], true),
        cand(1, &[0.0, 1.0], false),
        cand(9, &[1.0], false),
        cand(2, &[0.0, 1.0], false),
    ]);
    let out = run_similarity(&src, &[vec![1.0, 0.0]], &params().with_top_k(2)).unwrap();
    let h = &out.per_query[0];
    assert_eq!(h.len(), 2);
    assert_eq!(h[0].entry, EntryId(2));
    assert!(h[0].stale);
    assert_eq!(h[1].entry, EntryId(3));
}

#[test]
fn scoring_variants() {
    let src = Fixed(vec![cand(1, &[3.0, 4.0], false)]);
    let q = vec![vec![0.0, 0.0]];
    let s = |sc| {
        run_similarity(&src, &q, &params().with_scoring(sc)).unwrap().per_query[0][0].score
    };
    assert_eq!(s(Scoring::Cosine), 0.0);
    assert_eq!(s(Scoring::DotProduct), 0.0);
    assert_eq!(s(Scoring::NegEuclidean), -5.0);
}

#[test]
fn nan_last_and_top_k_zero() {
    let src = Fixed(vec![cand(1, &[f32::NAN], false), cand(2, &[1.0], false)]);
    let out = run_similarity(&src, &[vec![1.0]], &params().with_scoring(Scoring::DotProduct)).unwrap();
    assert_eq!(out.per_query[0][0].entry, EntryId(2));
    assert_eq!(out.per_query[0][1].entry, EntryId(1));
    let out = run_similarity(&src, &[vec![1.0]], &params().with_top_k(0)).unwrap();
    assert!(out.per_query[0].is_empty());
}

#[test]
fn rerank_and_task_result() {
    let src = Fixed(vec![cand(1, &[1.0], false), cand(2, &[2.0], true)]);
    let p = params();
    let out = run_similarity(&src, &[vec![1.0], vec![1.0]], &p).unwrap();
    let r = p.rerank(&out, Scoring::DotProduct);
    assert_eq!(r.scoring, Scoring::DotProduct);
    assert_eq!(p.scoring, Scoring::Cosine);
    assert!(matches!(r.scope, Scope::Set(_)));
    match out.into_task_result() {
        TaskResult::Hits(h) => assert_eq!(h.len(), 4),
        _ => panic!("expected hits"),
    }
}
