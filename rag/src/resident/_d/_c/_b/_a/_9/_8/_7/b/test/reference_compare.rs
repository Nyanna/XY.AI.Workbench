use super::*;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::query_batch::{
    Query, RamSource,
};
const SEG: SegmentId = SegmentId(7);
fn hit(offset: u64, score: f32) -> Hit3 {
    Hit3 { offset, score }
}
fn source(slots: usize) -> RamSource {
    let mut src = RamSource::new(1).unwrap();
    src.insert_segment(SEG, (0..slots).map(|_| Some(vec![0.0f32])).collect()).unwrap();
    src
}
fn batch(queries: usize) -> QueryBatch {
    let qs = (0..queries).map(|_| Query::new(1, vec![1.0]).unwrap()).collect();
    QueryBatch::from_queries(1, qs).unwrap()
}
struct Fixed(Vec<f32>);
impl DocRangeScorer for Fixed {
    fn score_range(
        &self,
        _src: &dyn DocSource,
        _segment: SegmentId,
        ws: Workspace,
        batch: &QueryBatch,
    ) -> Result<Vec<f32>, ScorerError> {
        let w = ws.len() as usize;
        let mut out = Vec::new();
        for _ in 0..batch.len() {
            out.extend_from_slice(&self.0[ws.start() as usize..ws.start() as usize + w]);
        }
        Ok(out)
    }
}
struct Failing;
impl DocRangeScorer for Failing {
    fn score_range(
        &self,
        _: &dyn DocSource,
        _: SegmentId,
        _: Workspace,
        _: &QueryBatch,
    ) -> Result<Vec<f32>, ScorerError> {
        Err("boom".into())
    }
}
struct WrongLen;
impl DocRangeScorer for WrongLen {
    fn score_range(
        &self,
        _: &dyn DocSource,
        _: SegmentId,
        _: Workspace,
        _: &QueryBatch,
    ) -> Result<Vec<f32>, ScorerError> {
        Ok(vec![0.0])
    }
}
#[test]
fn tolerances() {
    assert_eq!(tolerance(DType::F32), Some(1e-5));
    assert_eq!(tolerance(DType::F16), Some(1e-3));
    assert_eq!(tolerance(DType::BF16), None);
    assert_eq!(tolerance(DType::U8), None);
}
#[test]
fn unsupported_dtype_fails_before_scoring() {
    let src = source(2);
    let ws = [Workspace::new(0, 2)];
    let e = compare_scorers(
            &Failing,
            &Failing,
            &src,
            SEG,
            &ws,
            &batch(1),
            1,
            DType::BF16,
        )
        .unwrap_err();
    assert!(matches!(e, CompareError::UnsupportedDType(DType::BF16)));
    assert!(e.to_string().contains("BF16"));
}
#[test]
fn identical_lists_are_clean() {
    let l = [hit(0, 1.0), hit(4, 0.5)];
    assert!(compare_hits(SEG, 0, & l, & l, 1e-5).is_empty());
    assert!(compare_hits(SEG, 0, & [], & [], 1e-5).is_empty());
}
#[test]
fn within_tolerance_is_equal() {
    let c = [hit(0, 1.0 + 5e-6)];
    let r = [hit(0, 1.0)];
    assert!(compare_hits(SEG, 0, & c, & r, 1e-5).is_empty());
}
#[test]
fn value_mismatch() {
    let c = [hit(0, 1.0)];
    let r = [hit(0, 0.9)];
    let d = compare_hits(SEG, 3, &c, &r, 1e-5);
    assert_eq!(d.len(), 1);
    assert_eq!(d[0].segment, SEG);
    assert_eq!(d[0].query, 3);
    match d[0].kind {
        DeviationKind::ValueMismatch { rank, candidate, reference, diff } => {
            assert_eq!(rank, 0);
            assert_eq!(candidate, 1.0);
            assert_eq!(reference, 0.9);
            assert!((diff - 0.1).abs() < 1e-6);
        }
        ref k => panic!("unexpected {k:?}"),
    }
}
#[test]
fn rank_mismatch_reports_reference_rank() {
    let c = [hit(8, 1.0), hit(4, 0.5)];
    let r = [hit(4, 1.0), hit(8, 0.5)];
    let d = compare_hits(SEG, 0, &c, &r, 1e-5);
    assert_eq!(d.len(), 2);
    assert_eq!(
        d[0].kind, DeviationKind::RankMismatch { rank : 0, candidate_offset : 8,
        reference_offset : 4, reference_rank_of_candidate : Some(1), }
    );
}
#[test]
fn near_tie_is_not_reported() {
    let c = [hit(8, 1.0), hit(4, 1.0)];
    let r = [hit(4, 1.0), hit(8, 1.0 - 5e-6)];
    assert!(compare_hits(SEG, 0, & c, & r, 1e-5).is_empty());
}
#[test]
fn near_tie_chain_must_be_continuous() {
    let c = [hit(12, 1.0), hit(4, 1.0), hit(8, 1.0)];
    let r = [hit(4, 1.0), hit(8, 1.0), hit(12, 1.0 - 1e-3)];
    let d = compare_hits(SEG, 0, &c, &r, 1e-5);
    assert!(
        d.iter().any(| d | matches!(d.kind, DeviationKind::RankMismatch { rank : 0,
        reference_rank_of_candidate : Some(2), .. }))
    );
}
#[test]
fn candidate_offset_absent_in_reference() {
    let c = [hit(99, 1.0)];
    let r = [hit(4, 1.0)];
    let d = compare_hits(SEG, 0, &c, &r, 1e-5);
    assert_eq!(
        d[0].kind, DeviationKind::RankMismatch { rank : 0, candidate_offset : 99,
        reference_offset : 4, reference_rank_of_candidate : None, }
    );
}
#[test]
fn count_mismatch() {
    let c = [hit(0, 1.0)];
    let r = [hit(0, 1.0), hit(4, 0.5)];
    let d = compare_hits(SEG, 0, &c, &r, 1e-5);
    assert_eq!(d.len(), 1);
    assert_eq!(
        d[0].kind, DeviationKind::CountMismatch { candidate_len : 1, reference_len : 2,
        from_rank : 1, missing_on : Side::Candidate, }
    );
    let d = compare_hits(SEG, 0, &r, &c, 1e-5);
    assert!(
        matches!(d[0].kind, DeviationKind::CountMismatch { missing_on : Side::Reference,
        .. })
    );
}
#[test]
fn count_mismatch_comes_last() {
    let c = [hit(0, 0.0)];
    let r = [hit(0, 1.0), hit(4, 0.5)];
    let d = compare_hits(SEG, 0, &c, &r, 1e-5);
    assert_eq!(d.len(), 2);
    assert!(matches!(d[0].kind, DeviationKind::ValueMismatch { .. }));
    assert!(matches!(d[1].kind, DeviationKind::CountMismatch { .. }));
}
#[test]
fn identical_scorers_are_clean() {
    let src = source(4);
    let s = Fixed(vec![0.1, 0.4, 0.3, 0.2]);
    let ws = [Workspace::new(0, 2), Workspace::new(2, 2), Workspace::new(2, 4)];
    let b = batch(2);
    let rep = compare_scorers(&s, &s, &src, SEG, &ws[..1], &b, 3, DType::F32).unwrap();
    assert!(rep.is_clean());
    assert_eq!(rep.candidate, rep.reference);
    assert_eq!(rep.candidate.len(), 2);
    let rep = compare_scorers(
            &s,
            &s,
            &src,
            SEG,
            &[ws[0], ws[1], ws[2]][..2],
            &b,
            3,
            DType::F32,
        )
        .unwrap();
    assert!(rep.is_clean());
    assert_eq!(rep.candidate[0].len(), 2);
}
#[test]
fn differing_scorers_report_but_succeed() {
    let src = source(3);
    let c = Fixed(vec![0.1, 0.5, 0.3]);
    let r = Fixed(vec![0.1, 0.5, 0.2]);
    let ws = [Workspace::new(0, 3)];
    let rep = compare_scorers(&c, &r, &src, SEG, &ws, &batch(1), 3, DType::F16).unwrap();
    assert!(! rep.is_clean());
    assert!(
        matches!(rep.deviations[0].kind, DeviationKind::ValueMismatch { rank : 1, .. })
    );
}
#[test]
fn errors_are_forwarded() {
    let src = source(2);
    let ok = Fixed(vec![0.0, 1.0]);
    let ws = [Workspace::new(0, 2)];
    let b = batch(1);
    let e = compare_scorers(&Failing, &ok, &src, SEG, &ws, &b, 1, DType::F32)
        .unwrap_err();
    assert!(matches!(& e, CompareError::Candidate(x) if x.to_string() == "boom"));
    let e = compare_scorers(&ok, &Failing, &src, SEG, &ws, &b, 1, DType::F32)
        .unwrap_err();
    assert!(matches!(& e, CompareError::Reference(x) if x.to_string() == "boom"));
    let e = compare_scorers(&WrongLen, &ok, &src, SEG, &ws, &b, 1, DType::F32)
        .unwrap_err();
    assert!(matches!(e, CompareError::TopK(TopKError::ScoreLen { .. })));
}
#[test]
fn empty_workspaces_skip_scorers() {
    let src = source(2);
    let ws = [Workspace::new(1, 1)];
    let rep = compare_scorers(
            &Failing,
            &Failing,
            &src,
            SEG,
            &ws,
            &batch(1),
            2,
            DType::F32,
        )
        .unwrap();
    assert!(rep.is_clean());
    assert_eq!(rep.candidate, vec![Vec::<Hit3>::new()]);
}
#[test]
fn zero_k_and_empty_batch() {
    let src = source(2);
    let s = Fixed(vec![0.0, 1.0]);
    let ws = [Workspace::new(0, 2)];
    let rep = compare_scorers(&s, &s, &src, SEG, &ws, &batch(2), 0, DType::F32).unwrap();
    assert!(rep.is_clean());
    let rep = compare_scorers(&s, &s, &src, SEG, &ws, &batch(0), 2, DType::F32).unwrap();
    assert!(rep.is_clean());
    assert!(rep.candidate.is_empty() && rep.reference.is_empty());
}
