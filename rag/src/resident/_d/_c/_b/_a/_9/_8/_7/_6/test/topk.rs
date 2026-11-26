use super::*;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::query_batch::RamSource;
use proptest::prelude::*;

const SEG: SegmentId = SegmentId(7);

/// Slot `i` holds one 1-dim vector, so its byte offset is `4 * i`.
fn source(slots: usize) -> RamSource {
    let mut src = RamSource::new(1).unwrap();
    src.insert_segment(SEG, vec![Some(vec![0.0]); slots]).unwrap();
    src
}

fn off(slot: u64) -> u64 {
    slot * 4
}

fn sorted(k: usize, items: &[(u64, f32)]) -> Vec<Hit> {
    let mut t = TopK::new(k);
    t.extend(items.iter().copied());
    t.into_sorted_vec()
}

fn reference(k: usize, items: &[(u64, f32)]) -> Vec<(u64, f32)> {
    let mut v: Vec<(u64, f32)> = items.iter().copied().filter(|(_, s)| !s.is_nan()).collect();
    v.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap().then(a.0.cmp(&b.0)));
    v.truncate(k);
    v
}

fn pairs(hits: &[Hit]) -> Vec<(u64, f32)> {
    hits.iter().map(|h| (h.offset, h.score)).collect()
}

fn is_send_sync<T: Send + Sync>() {}

#[test]
fn types_are_send_sync_and_errors_are_errors() {
    is_send_sync::<Hit>();
    is_send_sync::<TopK>();
    is_send_sync::<RangeScorer>();
    is_send_sync::<TopKError>();
    fn is_error<E: std::error::Error + Send + Sync>() {}
    is_error::<TopKError>();
}

#[test]
fn orders_by_score_desc_then_offset_asc() {
    let hits = sorted(10, &[(5, 1.0), (3, 2.0), (1, 1.0), (9, 2.0), (2, 0.5)]);
    assert_eq!(
        pairs(&hits),
        vec![(3, 2.0), (9, 2.0), (1, 1.0), (5, 1.0), (2, 0.5)]
    );
}

#[test]
fn keeps_only_best_k() {
    let hits = sorted(2, &[(1, 1.0), (2, 3.0), (3, 2.0), (4, 0.0)]);
    assert_eq!(pairs(&hits), vec![(2, 3.0), (3, 2.0)]);
}

#[test]
fn fewer_candidates_than_k_and_empty() {
    assert_eq!(sorted(5, &[(1, 1.0)]).len(), 1);
    assert!(sorted(5, &[]).is_empty());
}

#[test]
fn push_reports_admission_and_displacement() {
    let mut t = TopK::new(2);
    assert!(t.push(1, 1.0));
    assert!(t.push(2, 2.0));
    assert!(!t.push(3, 0.5));
    assert!(t.push(4, 3.0));
    assert_eq!(t.len(), 2);
    assert_eq!(pairs(&t.into_sorted_vec()), vec![(4, 3.0), (2, 2.0)]);
}

#[test]
fn tie_on_score_prefers_lower_offset() {
    let mut t = TopK::new(1);
    assert!(t.push(10, 1.0));
    assert!(!t.push(11, 1.0));
    assert!(t.push(9, 1.0));
    assert_eq!(pairs(&t.into_sorted_vec()), vec![(9, 1.0)]);
}

#[test]
fn equal_candidate_does_not_displace() {
    let mut t = TopK::new(1);
    assert!(t.push(4, 1.0));
    assert!(!t.push(4, 1.0));
    assert_eq!(t.len(), 1);
}

#[test]
fn duplicates_may_repeat_below_k() {
    let hits = sorted(3, &[(4, 1.0), (4, 1.0)]);
    assert_eq!(pairs(&hits), vec![(4, 1.0), (4, 1.0)]);
}

#[test]
fn nan_is_discarded_without_state_change() {
    let mut t = TopK::new(2);
    assert!(t.push(1, 1.0));
    assert!(!t.push(2, f32::NAN));
    assert_eq!(t.len(), 1);
    let mut full = TopK::new(1);
    full.push(1, 1.0);
    assert!(!full.push(0, f32::NAN));
    assert_eq!(pairs(&full.into_sorted_vec()), vec![(1, 1.0)]);
}

#[test]
fn zero_k_takes_nothing() {
    let mut t = TopK::new(0);
    assert!(!t.push(1, 1.0));
    assert_eq!(t.k(), 0);
    assert_eq!(t.len(), 0);
    assert!(t.is_empty());
    assert!(t.into_sorted_vec().is_empty());
}

#[test]
fn infinities_are_valid_and_signed_zeros_equal() {
    let hits = sorted(
        4,
        &[(1, f32::NEG_INFINITY), (2, f32::INFINITY), (3, 0.0), (0, -0.0)],
    );
    assert_eq!(hits[0].offset, 2);
    assert_eq!(hits[1].offset, 0);
    assert_eq!(hits[2].offset, 3);
    assert_eq!(hits[3].offset, 1);
}

#[test]
fn huge_k_does_not_preallocate() {
    let mut t = TopK::new(usize::MAX);
    t.push(1, 1.0);
    t.push(0, 2.0);
    assert_eq!(pairs(&t.into_sorted_vec()), vec![(0, 2.0), (1, 1.0)]);
}

#[test]
fn merge_keeps_own_k() {
    let mut a = TopK::new(2);
    a.extend([(1, 1.0), (2, 2.0)]);
    let mut b = TopK::new(5);
    b.extend([(3, 3.0), (4, 4.0), (5, 0.0)]);
    a.merge(b);
    assert_eq!(a.k(), 2);
    assert_eq!(pairs(&a.into_sorted_vec()), vec![(4, 4.0), (3, 3.0)]);
}

#[test]
fn add_range_maps_scores_to_slot_offsets() {
    let src = source(6);
    let mut rs = RangeScorer::new(2, 2);
    rs.add_range(&src, SEG, Workspace::new(2, 5), &[0.1, 0.9, 0.5, 0.7, 0.2, 0.8])
        .unwrap();
    let out = rs.into_results();
    assert_eq!(pairs(&out[0]), vec![(off(3), 0.9), (off(4), 0.5)]);
    assert_eq!(pairs(&out[1]), vec![(off(4), 0.8), (off(2), 0.7)]);
}

#[test]
fn add_range_rejects_bad_range_and_keeps_state() {
    let src = source(3);
    let mut rs = RangeScorer::new(1, 2);
    rs.add_range(&src, SEG, Workspace::new(0, 1), &[1.0]).unwrap();
    let err = rs
        .add_range(&src, SEG, Workspace::new(1, 4), &[1.0, 2.0, 3.0])
        .unwrap_err();
    assert!(matches!(err, TopKError::Source(_)));
    let err = rs
        .add_range(&src, SegmentId(99), Workspace::new(0, 1), &[1.0])
        .unwrap_err();
    assert!(matches!(err, TopKError::Source(_)));
    assert_eq!(pairs(&rs.into_results()[0]), vec![(off(0), 1.0)]);
}

#[test]
fn source_error_is_checked_before_score_length() {
    let src = source(1);
    let mut rs = RangeScorer::new(1, 1);
    let err = rs.add_range(&src, SEG, Workspace::new(0, 5), &[]).unwrap_err();
    assert!(matches!(err, TopKError::Source(_)));
}

#[test]
fn add_range_rejects_wrong_score_length_and_keeps_state() {
    let src = source(4);
    let mut rs = RangeScorer::new(2, 3);
    let err = rs
        .add_range(&src, SEG, Workspace::new(0, 2), &[1.0, 2.0, 3.0])
        .unwrap_err();
    assert_eq!(err, TopKError::ScoreLen { expected: 4, got: 3 });
    assert!(rs.into_results().iter().all(Vec::is_empty));
}

#[test]
fn empty_workspace_and_zero_queries() {
    let src = source(2);
    let mut rs = RangeScorer::new(2, 3);
    rs.add_range(&src, SEG, Workspace::new(1, 1), &[]).unwrap();
    assert!(rs
        .add_range(&src, SEG, Workspace::new(1, 1), &[1.0])
        .is_err());
    assert!(rs.into_results().iter().all(Vec::is_empty));

    let mut none = RangeScorer::new(0, 3);
    none.add_range(&src, SEG, Workspace::new(0, 2), &[]).unwrap();
    assert!(none
        .add_range(&src, SEG, Workspace::new(0, 2), &[1.0])
        .is_err());
    assert!(none.into_results().is_empty());
}

#[test]
fn unknown_segment_accepts_only_empty_range() {
    let src = source(1);
    let mut rs = RangeScorer::new(1, 1);
    rs.add_range(&src, SegmentId(1), Workspace::new(0, 0), &[]).unwrap();
    assert!(rs
        .add_range(&src, SegmentId(1), Workspace::new(0, 1), &[1.0])
        .is_err());
}

#[test]
fn zero_k_scorer_yields_empty_lists() {
    let src = source(2);
    let mut rs = RangeScorer::new(2, 0);
    rs.add_range(&src, SEG, Workspace::new(0, 2), &[1.0, 2.0, 3.0, 4.0])
        .unwrap();
    let out = rs.into_results();
    assert_eq!(out.len(), 2);
    assert!(out.iter().all(Vec::is_empty));
}

#[test]
fn nan_scores_are_dropped_in_ranges() {
    let src = source(3);
    let mut rs = RangeScorer::new(1, 5);
    rs.add_range(&src, SEG, Workspace::new(0, 3), &[f32::NAN, 1.0, f32::NAN])
        .unwrap();
    assert_eq!(pairs(&rs.into_results()[0]), vec![(off(1), 1.0)]);
}

#[test]
fn slots_without_vectors_share_offset_and_are_not_special() {
    let mut src = RamSource::new(1).unwrap();
    src.insert_segment(SEG, vec![Some(vec![0.0]), None, Some(vec![0.0])])
        .unwrap();
    let mut rs = RangeScorer::new(1, 3);
    rs.add_range(&src, SEG, Workspace::new(0, 3), &[1.0, 2.0, 3.0])
        .unwrap();
    assert_eq!(
        pairs(&rs.into_results()[0]),
        vec![(4, 3.0), (4, 2.0), (0, 1.0)]
    );
}

struct HoleSource;

impl DocSource for HoleSource {
    fn dim(&self) -> u32 {
        1
    }
    fn slot_count(&self, _: SegmentId) -> u32 {
        3
    }
    fn slot_offset(&self, _: SegmentId, slot: Slot) -> Option<u64> {
        (slot.0 != 1).then_some(u64::from(slot.0) * 4)
    }
    fn slot_vectors(&self, _: SegmentId, _: Slot) -> Option<&[f32]> {
        None
    }
}

#[test]
fn missing_offset_skips_slot_without_panic() {
    let mut rs = RangeScorer::new(1, 5);
    rs.add_range(&HoleSource, SEG, Workspace::new(0, 3), &[1.0, 9.0, 2.0])
        .unwrap();
    assert_eq!(pairs(&rs.into_results()[0]), vec![(8, 2.0), (0, 1.0)]);
}

#[test]
fn multiple_ranges_and_segments_accumulate() {
    let mut src = source(4);
    src.insert_segment(SegmentId(8), vec![Some(vec![0.0]); 2]).unwrap();
    let mut rs = RangeScorer::new(1, 3);
    rs.add_range(&src, SEG, Workspace::new(0, 2), &[1.0, 5.0]).unwrap();
    rs.add_range(&src, SegmentId(8), Workspace::new(0, 2), &[5.0, 4.0])
        .unwrap();
    rs.add_range(&src, SEG, Workspace::new(2, 4), &[3.0, 2.0]).unwrap();
    assert_eq!(
        pairs(&rs.into_results()[0]),
        vec![(off(0), 5.0), (off(1), 5.0), (off(1), 4.0)]
    );
}

#[test]
fn merge_rejects_mismatch_and_keeps_state() {
    let src = source(2);
    let mut a = RangeScorer::new(1, 2);
    a.add_range(&src, SEG, Workspace::new(0, 2), &[1.0, 2.0]).unwrap();
    assert_eq!(
        a.merge(RangeScorer::new(1, 3)),
        Err(TopKError::Mismatch { what: "k" })
    );
    assert_eq!(
        a.merge(RangeScorer::new(2, 2)),
        Err(TopKError::Mismatch {
            what: "query count"
        })
    );
    assert_eq!(a.num_queries(), 1);
    assert_eq!(a.k(), 2);
    assert_eq!(a.into_results()[0].len(), 2);
}

#[test]
fn merge_combines_partial_results() {
    let src = source(4);
    let mut a = RangeScorer::new(1, 2);
    let mut b = RangeScorer::new(1, 2);
    a.add_range(&src, SEG, Workspace::new(0, 2), &[1.0, 4.0]).unwrap();
    b.add_range(&src, SEG, Workspace::new(2, 4), &[3.0, 2.0]).unwrap();
    a.merge(b).unwrap();
    assert_eq!(pairs(&a.into_results()[0]), vec![(off(1), 4.0), (off(2), 3.0)]);
}

proptest! {
    #[test]
    fn matches_reference_for_any_order_and_split(
        k in 0usize..8,
        items in proptest::collection::vec((0u64..6, -3i32..4), 0..40),
        split in 0usize..40,
    ) {
        let items: Vec<(u64, f32)> = items.into_iter().map(|(o, s)| (o, s as f32)).collect();
        let expected = reference(k, &items);
        prop_assert_eq!(pairs(&sorted(k, &items)), expected.clone());

        let mut reversed = items.clone();
        reversed.reverse();
        prop_assert_eq!(pairs(&sorted(k, &reversed)), expected.clone());

        let cut = split.min(items.len());
        let mut left = TopK::new(k);
        left.extend(items[..cut].iter().copied());
        let mut right = TopK::new(k);
        right.extend(items[cut..].iter().copied());
        left.merge(right);
        prop_assert_eq!(pairs(&left.into_sorted_vec()), expected);
    }

    #[test]
    fn range_scorer_matches_direct_selection(
        k in 0usize..6,
        nq in 0usize..4,
        len in 0u32..12,
        cut in 0u32..12,
        raw in proptest::collection::vec(-3i32..4, 48),
    ) {
        let src = source(len as usize);
        let cut = cut.min(len);
        let scores: Vec<f32> = raw.iter().map(|&s| s as f32).collect();
        let width = len as usize;
        let mut rs = RangeScorer::new(nq, k);
        for ws in [Workspace::new(0, cut), Workspace::new(cut, len)] {
            let w = ws.len() as usize;
            let mut chunk = Vec::with_capacity(nq * w);
            for q in 0..nq {
                for i in 0..w {
                    chunk.push(scores[(q * width + ws.start() as usize + i) % scores.len()]);
                }
            }
            rs.add_range(&src, SEG, ws, &chunk).unwrap();
        }
        let out = rs.into_results();
        prop_assert_eq!(out.len(), nq);
        for (q, hits) in out.iter().enumerate() {
            let all: Vec<(u64, f32)> = (0..width)
                .map(|i| (off(i as u64), scores[(q * width + i) % scores.len()]))
                .collect();
            prop_assert_eq!(pairs(hits), reference(k, &all));
        }
    }
}
