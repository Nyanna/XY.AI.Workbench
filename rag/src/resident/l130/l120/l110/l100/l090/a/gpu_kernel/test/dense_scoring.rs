use super::*;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::a::index_contract::{
    AccessMethod, EntryId, Payload, PayloadRef, SharedBytes, Stamp, StampObserver, StampSignal, Summary,
};
use std::sync::{Arc, Mutex};

fn assert_send_sync<T: Send + Sync>() {}

fn approx(a: f32, b: f32) -> bool {
    (a - b).abs() <= 1e-6
}

fn f32_bytes(v: &[f32]) -> Vec<u8> {
    v.iter().flat_map(|x| x.to_le_bytes()).collect()
}

fn f16_bytes(bits: &[u16]) -> Vec<u8> {
    bits.iter().flat_map(|x| x.to_le_bytes()).collect()
}

struct MockIndex {
    dtype: DType,
    entries: Vec<(u64, Result<Vec<u8>, IndexError>)>,
    calls: Mutex<Vec<u64>>,
    signal: Arc<StampSignal>,
}

impl MockIndex {
    fn new(dtype: DType, entries: Vec<(u64, Result<Vec<u8>, IndexError>)>) -> Self {
        MockIndex {
            dtype,
            entries,
            calls: Mutex::new(Vec::new()),
            signal: StampSignal::new(Stamp(0)),
        }
    }

    fn calls(&self) -> Vec<u64> {
        self.calls.lock().unwrap().clone()
    }
}

impl Index for MockIndex {
    fn dtype(&self) -> DType {
        self.dtype
    }
    fn access_methods(&self) -> &[AccessMethod] {
        &[AccessMethod::Centroid]
    }
    fn stamp(&self, _id: EntryId) -> Option<Stamp> {
        None
    }
    fn stamps(&self) -> Vec<(EntryId, Stamp)> {
        Vec::new()
    }
    fn read(&self, _id: EntryId) -> Payload {
        Payload::default()
    }
    fn write(&self, _id: EntryId, _p: PayloadRef<'_>) -> Result<Stamp, IndexError> {
        Err(IndexError::Unsupported(AccessMethod::Prefix))
    }
    fn delete(&self, _id: EntryId) -> Result<Option<Stamp>, IndexError> {
        Ok(None)
    }
    fn subscribe(&self) -> StampObserver {
        self.signal.observer()
    }
    fn summary(&self, node: NodeId) -> Result<Summary, IndexError> {
        self.calls.lock().unwrap().push(node.0);
        match self.entries.iter().find(|(id, _)| *id == node.0) {
            Some((_, Ok(bytes))) => Ok(Summary {
                node,
                centroid: SharedBytes::from(bytes.clone()),
                count: 1,
                stamp: Stamp(0),
            }),
            Some((_, Err(e))) => Err(e.clone()),
            None => Err(IndexError::NotFound),
        }
    }
}

#[test]
fn types_are_send_sync() {
    assert_send_sync::<DenseScores>();
    assert_send_sync::<DenseError>();
    assert_send_sync::<Hit>();
    assert_send_sync::<CentroidScores>();
}

#[test]
fn dot_mode_returns_plain_dot_products() {
    let q: &[f32] = &[2.0, 0.0, 1.0];
    let s0: &[f32] = &[1.0, 5.0, 1.0];
    let s1: &[f32] = &[0.0, 1.0, -3.0];
    let r = score(&[q], &[s0, s1], 3, SimilarityMode::Dot).unwrap();
    assert_eq!(r.query_count(), 1);
    assert_eq!(r.slot_count(), 2);
    assert_eq!(r.row(0).unwrap(), &[3.0, -3.0]);
    assert_eq!(r.as_flat(), &[3.0, -3.0]);
}

#[test]
fn dot_mode_matches_cpu_dot_bitwise() {
    let dim = 19;
    let q: Vec<f32> = (0..dim).map(|i| (i as f32 * 0.37).sin()).collect();
    let s: Vec<f32> = (0..dim).map(|i| (i as f32 * 0.91).cos()).collect();
    let r = score(&[&q], &[&s], dim, SimilarityMode::Dot).unwrap();
    assert_eq!(r.row(0).unwrap()[0].to_bits(), dot(&q, &s).to_bits());
}

#[test]
fn exact_mode_normalizes_by_both_lengths() {
    let q: &[f32] = &[3.0, 0.0];
    let parallel: &[f32] = &[10.0, 0.0];
    let ortho: &[f32] = &[0.0, 4.0];
    let opposite: &[f32] = &[-0.5, 0.0];
    let diag: &[f32] = &[1.0, 1.0];
    let r = score(&[q], &[parallel, ortho, opposite, diag], 2, SimilarityMode::Exact).unwrap();
    let row = r.row(0).unwrap();
    assert!(approx(row[0], 1.0));
    assert!(approx(row[1], 0.0));
    assert!(approx(row[2], -1.0));
    assert!(approx(row[3], std::f32::consts::FRAC_1_SQRT_2));
}

#[test]
fn dot_mode_does_not_normalize() {
    let q: &[f32] = &[3.0, 0.0];
    let s: &[f32] = &[10.0, 0.0];
    let r = score(&[q], &[s], 2, SimilarityMode::Dot).unwrap();
    assert_eq!(r.row(0).unwrap()[0], 30.0);
}

#[test]
fn zero_slot_scores_zero_in_both_modes() {
    let q: &[f32] = &[1.0, 2.0];
    let z: &[f32] = &[0.0, 0.0];
    for mode in [SimilarityMode::Dot, SimilarityMode::Exact] {
        let r = score(&[q], &[z], 2, mode).unwrap();
        assert_eq!(r.row(0).unwrap()[0], 0.0);
    }
}

#[test]
fn exact_mode_non_finite_denominator_scores_zero() {
    let q: &[f32] = &[1.0, 1.0];
    let inf: &[f32] = &[f32::INFINITY, 1.0];
    let nan: &[f32] = &[f32::NAN, 1.0];
    let r = score(&[q], &[inf, nan], 2, SimilarityMode::Exact).unwrap();
    assert_eq!(r.row(0).unwrap(), &[0.0, 0.0]);
}

#[test]
fn dot_mode_passes_non_finite_values_through() {
    let q: &[f32] = &[1.0, 1.0];
    let inf: &[f32] = &[f32::INFINITY, 1.0];
    let nan: &[f32] = &[f32::NAN, 1.0];
    let r = score(&[q], &[inf, nan], 2, SimilarityMode::Dot).unwrap();
    let row = r.row(0).unwrap();
    assert_eq!(row[0], f32::INFINITY);
    assert!(row[1].is_nan());
}

#[test]
fn rows_follow_query_and_slot_order() {
    let q0: &[f32] = &[1.0, 0.0];
    let q1: &[f32] = &[0.0, 1.0];
    let s0: &[f32] = &[1.0, 2.0];
    let s1: &[f32] = &[3.0, 4.0];
    let s2: &[f32] = &[5.0, 6.0];
    let r = score(&[q0, q1], &[s0, s1, s2], 2, SimilarityMode::Dot).unwrap();
    assert_eq!(r.row(0).unwrap(), &[1.0, 3.0, 5.0]);
    assert_eq!(r.row(1).unwrap(), &[2.0, 4.0, 6.0]);
    assert_eq!(r.as_flat(), &[1.0, 3.0, 5.0, 2.0, 4.0, 6.0]);
    assert!(r.row(2).is_none());
}

#[test]
fn inputs_are_not_modified() {
    let q = vec![3.0f32, 4.0];
    let s = vec![1.0f32, 1.0];
    let (q0, s0) = (q.clone(), s.clone());
    score(&[&q], &[&s], 2, SimilarityMode::Exact).unwrap();
    assert_eq!(q, q0);
    assert_eq!(s, s0);
}

#[test]
fn result_is_deterministic() {
    let q: Vec<f32> = (0..37).map(|i| (i as f32).sin() + 0.1).collect();
    let s: Vec<f32> = (0..37).map(|i| (i as f32 * 1.3).cos()).collect();
    let a = score(&[&q], &[&s], 37, SimilarityMode::Exact).unwrap();
    let b = score(&[&q], &[&s], 37, SimilarityMode::Exact).unwrap();
    assert_eq!(a, b);
}

#[test]
fn empty_queries_or_slots_yield_empty_matrix() {
    let s: &[f32] = &[1.0, 0.0];
    let q: &[f32] = &[1.0, 0.0];
    let no_queries = score(&[], &[s], 2, SimilarityMode::Dot).unwrap();
    assert_eq!(no_queries.query_count(), 0);
    assert!(no_queries.as_flat().is_empty());
    assert!(no_queries.row(0).is_none());
    let no_slots = score(&[q], &[], 2, SimilarityMode::Exact).unwrap();
    assert_eq!(no_slots.query_count(), 1);
    assert_eq!(no_slots.slot_count(), 0);
    assert_eq!(no_slots.row(0).unwrap(), &[] as &[f32]);
    assert_eq!(no_slots.top_k(0, 3).unwrap(), vec![]);
}

#[test]
fn zero_dim_is_invalid_even_with_empty_inputs() {
    assert_eq!(score(&[], &[], 0, SimilarityMode::Dot), Err(DenseError::InvalidDim));
    let empty: &[f32] = &[];
    assert_eq!(
        score(&[empty], &[empty], 0, SimilarityMode::Exact),
        Err(DenseError::InvalidDim)
    );
}

#[test]
fn query_shape_errors_report_lowest_index_first() {
    let ok: &[f32] = &[1.0, 0.0];
    let short: &[f32] = &[1.0];
    let long: &[f32] = &[1.0, 0.0, 0.0];
    assert_eq!(
        score(&[ok, long, short], &[ok], 2, SimilarityMode::Dot),
        Err(DenseError::Shape { what: ShapeSource::Query(1), len: 3, dim: 2 })
    );
}

#[test]
fn empty_query_slice_is_a_shape_error() {
    let ok: &[f32] = &[1.0, 0.0];
    let empty: &[f32] = &[];
    assert_eq!(
        score(&[ok, empty], &[ok], 2, SimilarityMode::Dot),
        Err(DenseError::Shape { what: ShapeSource::Query(1), len: 0, dim: 2 })
    );
}

#[test]
fn query_shape_is_checked_before_empty_embedding() {
    let zero: &[f32] = &[0.0, 0.0];
    let bad: &[f32] = &[1.0];
    assert_eq!(
        score(&[zero, bad], &[zero], 2, SimilarityMode::Dot),
        Err(DenseError::Shape { what: ShapeSource::Query(1), len: 1, dim: 2 })
    );
}

#[test]
fn empty_embedding_is_rejected_in_both_modes() {
    let ok: &[f32] = &[1.0, 0.0];
    let zero: &[f32] = &[0.0, 0.0];
    let tiny: &[f32] = &[1e-10, 0.0];
    let nan: &[f32] = &[f32::NAN, 1.0];
    let inf: &[f32] = &[f32::INFINITY, 1.0];
    for mode in [SimilarityMode::Dot, SimilarityMode::Exact] {
        assert_eq!(
            score(&[ok, zero], &[ok], 2, mode),
            Err(DenseError::EmptyEmbedding { query_index: 1 })
        );
        assert_eq!(
            score(&[tiny], &[ok], 2, mode),
            Err(DenseError::EmptyEmbedding { query_index: 0 })
        );
        assert_eq!(
            score(&[ok, nan, zero], &[ok], 2, mode),
            Err(DenseError::EmptyEmbedding { query_index: 1 })
        );
        assert_eq!(
            score(&[inf], &[ok], 2, mode),
            Err(DenseError::EmptyEmbedding { query_index: 0 })
        );
    }
}

#[test]
fn small_but_valid_query_is_accepted() {
    let q: &[f32] = &[1e-4, 0.0];
    let s: &[f32] = &[1.0, 0.0];
    assert!(score(&[q], &[s], 2, SimilarityMode::Exact).is_ok());
}

#[test]
fn empty_embedding_is_checked_before_slot_shape() {
    let zero: &[f32] = &[0.0, 0.0];
    let bad_slot: &[f32] = &[1.0];
    assert_eq!(
        score(&[zero], &[bad_slot], 2, SimilarityMode::Dot),
        Err(DenseError::EmptyEmbedding { query_index: 0 })
    );
}

#[test]
fn slot_shape_errors_report_lowest_index_first() {
    let ok: &[f32] = &[1.0, 0.0];
    let empty: &[f32] = &[];
    let long: &[f32] = &[1.0, 0.0, 1.0];
    assert_eq!(
        score(&[ok], &[ok, empty, long], 2, SimilarityMode::Exact),
        Err(DenseError::Shape { what: ShapeSource::Slot(1), len: 0, dim: 2 })
    );
    assert_eq!(
        score(&[], &[ok, long], 2, SimilarityMode::Dot),
        Err(DenseError::Shape { what: ShapeSource::Slot(1), len: 3, dim: 2 })
    );
}

#[test]
fn empty_embedding_display_mentions_text_and_index() {
    let text = DenseError::EmptyEmbedding { query_index: 7 }.to_string();
    assert!(text.contains("leeres Embedding"));
    assert!(text.contains('7'));
}

#[test]
fn index_error_is_exposed_as_source() {
    use std::error::Error;
    let e = DenseError::Index(IndexError::NotFound);
    assert!(e.source().is_some());
    assert!(e.to_string().contains("index"));
    assert!(DenseError::InvalidDim.source().is_none());
}

fn hits(v: &[Hit]) -> Vec<(u32, f32)> {
    v.iter().map(|h| (h.slot.0, h.score)).collect()
}

#[test]
fn top_k_sorts_descending() {
    let r = top_k(&[0.1, 0.9, 0.5, 0.7], 3);
    assert_eq!(hits(&r), vec![(1, 0.9), (3, 0.7), (2, 0.5)]);
}

#[test]
fn top_k_breaks_ties_by_smaller_index() {
    let r = top_k(&[0.5, 0.9, 0.5, 0.9, 0.5], 4);
    assert_eq!(hits(&r), vec![(1, 0.9), (3, 0.9), (0, 0.5), (2, 0.5)]);
    let all_equal = top_k(&[1.0; 6], 2);
    assert_eq!(hits(&all_equal), vec![(0, 1.0), (1, 1.0)]);
}

#[test]
fn top_k_skips_nan() {
    let r = top_k(&[f32::NAN, 0.2, f32::NAN, 0.4], 10);
    assert_eq!(hits(&r), vec![(3, 0.4), (1, 0.2)]);
    assert!(top_k(&[f32::NAN, f32::NAN], 2).is_empty());
}

#[test]
fn top_k_zero_and_oversized_k() {
    assert!(top_k(&[1.0, 2.0], 0).is_empty());
    assert!(top_k(&[], 5).is_empty());
    assert_eq!(top_k(&[1.0, 2.0], usize::MAX).len(), 2);
    assert_eq!(top_k(&[1.0, 2.0, 3.0], 3).len(), 3);
}

#[test]
fn top_k_handles_infinities_and_signed_zero() {
    let r = top_k(&[f32::NEG_INFINITY, f32::INFINITY, -0.0, 0.0], 4);
    assert_eq!(
        hits(&r).iter().map(|(i, _)| *i).collect::<Vec<_>>(),
        vec![1, 2, 3, 0]
    );
}

#[test]
fn top_k_matches_full_sort_on_pseudo_random_input() {
    let mut state = 12345u32;
    let scores: Vec<f32> = (0..500)
        .map(|_| {
            state = state.wrapping_mul(1664525).wrapping_add(1013904223);
            ((state >> 16) % 50) as f32
        })
        .collect();
    let mut expected: Vec<(u32, f32)> = scores.iter().enumerate().map(|(i, s)| (i as u32, *s)).collect();
    expected.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap().then(a.0.cmp(&b.0)));
    for k in [1usize, 7, 50, 500] {
        assert_eq!(hits(&top_k(&scores, k)), expected[..k].to_vec());
    }
}

#[test]
fn dense_scores_top_k_uses_row_and_rejects_unknown_query() {
    let q0: &[f32] = &[1.0, 0.0];
    let q1: &[f32] = &[0.0, 1.0];
    let s0: &[f32] = &[1.0, 0.0];
    let s1: &[f32] = &[0.0, 1.0];
    let s2: &[f32] = &[1.0, 1.0];
    for mode in [SimilarityMode::Dot, SimilarityMode::Exact] {
        let r = score(&[q0, q1], &[s0, s1, s2], 2, mode).unwrap();
        let t0 = r.top_k(0, 1).unwrap();
        assert_eq!(t0[0].slot, Slot(0));
        let t1 = r.top_k(1, 1).unwrap();
        assert_eq!(t1[0].slot, Slot(1));
        assert!(r.top_k(2, 1).is_none());
    }
}

#[test]
fn f16_conversion_known_values() {
    assert_eq!(f16_to_f32(0x3C00), 1.0);
    assert_eq!(f16_to_f32(0xC000), -2.0);
    assert_eq!(f16_to_f32(0x3800), 0.5);
    assert_eq!(f16_to_f32(0x7BFF), 65504.0);
    assert_eq!(f16_to_f32(0x0400), 2.0f32.powi(-14));
    assert_eq!(f16_to_f32(0x0001), 2.0f32.powi(-24));
    assert_eq!(f16_to_f32(0x83FF), -(1023.0 * 2.0f32.powi(-24)));
    assert_eq!(f16_to_f32(0x7C00), f32::INFINITY);
    assert_eq!(f16_to_f32(0xFC00), f32::NEG_INFINITY);
    assert!(f16_to_f32(0x7E00).is_nan());
    assert!(f16_to_f32(0x7C01).is_nan());
    assert_eq!(f16_to_f32(0x0000).to_bits(), 0.0f32.to_bits());
    assert_eq!(f16_to_f32(0x8000).to_bits(), (-0.0f32).to_bits());
}

#[test]
fn f16_conversion_is_exact_for_all_finite_values() {
    for bits in 0..=u16::MAX {
        let exp = i32::from((bits >> 10) & 0x1f);
        if exp == 31 {
            continue;
        }
        let man = f64::from(bits & 0x3ff);
        let sign = if bits & 0x8000 != 0 { -1.0 } else { 1.0 };
        let expected = if exp == 0 {
            sign * man / 1024.0 * 2f64.powi(-14)
        } else {
            sign * (1.0 + man / 1024.0) * 2f64.powi(exp - 15)
        };
        assert_eq!(f64::from(f16_to_f32(bits)), expected, "bits {bits:#06x}");
    }
}

fn node_ids(ids: &[u64]) -> Vec<NodeId> {
    ids.iter().map(|i| NodeId(*i)).collect()
}

#[test]
fn centroids_f32_are_scored_per_node() {
    let idx = MockIndex::new(
        DType::F32,
        vec![
            (10, Ok(f32_bytes(&[1.0, 0.0]))),
            (20, Ok(f32_bytes(&[0.0, 2.0]))),
            (30, Ok(f32_bytes(&[1.0, 1.0]))),
        ],
    );
    let q: &[f32] = &[0.0, 1.0];
    let nodes = node_ids(&[30, 10, 20]);
    let r = score_centroids(&idx, &nodes, &[q], 2, SimilarityMode::Dot).unwrap();
    assert_eq!(r.nodes, nodes);
    assert_eq!(r.scores.row(0).unwrap(), &[1.0, 0.0, 2.0]);
    assert_eq!(idx.calls(), vec![30, 10, 20]);
    let top = r.scores.top_k(0, 1).unwrap();
    assert_eq!(top[0].slot, Slot(2));
    assert_eq!(r.nodes[top[0].slot.0 as usize], NodeId(20));
}

#[test]
fn centroids_exact_mode_matches_score() {
    let c0 = [3.0f32, 4.0];
    let c1 = [0.0f32, -2.0];
    let idx = MockIndex::new(
        DType::F32,
        vec![(1, Ok(f32_bytes(&c0))), (2, Ok(f32_bytes(&c1)))],
    );
    let q: &[f32] = &[1.0, 1.0];
    let r = score_centroids(&idx, &node_ids(&[1, 2]), &[q], 2, SimilarityMode::Exact).unwrap();
    let expected = score(&[q], &[&c0, &c1], 2, SimilarityMode::Exact).unwrap();
    assert_eq!(r.scores, expected);
}

#[test]
fn centroids_f16_are_widened_to_f32() {
    // 1.0, -2.0, 0.5
    let idx = MockIndex::new(
        DType::F16,
        vec![(5, Ok(f16_bytes(&[0x3C00, 0xC000, 0x3800])))],
    );
    let q: &[f32] = &[1.0, 1.0, 2.0];
    let r = score_centroids(&idx, &node_ids(&[5]), &[q], 3, SimilarityMode::Dot).unwrap();
    assert_eq!(r.scores.row(0).unwrap(), &[1.0 - 2.0 + 1.0]);
}

#[test]
fn centroids_empty_nodes_give_zero_columns() {
    let idx = MockIndex::new(DType::F32, vec![]);
    let q: &[f32] = &[1.0, 0.0];
    let r = score_centroids(&idx, &[], &[q], 2, SimilarityMode::Dot).unwrap();
    assert!(r.nodes.is_empty());
    assert_eq!(r.scores.query_count(), 1);
    assert_eq!(r.scores.slot_count(), 0);
    assert!(idx.calls().is_empty());
}

#[test]
fn centroids_reject_unsupported_dtype() {
    let idx = MockIndex::new(DType::BF16, vec![(1, Ok(vec![0; 4]))]);
    let q: &[f32] = &[1.0, 0.0];
    assert_eq!(
        score_centroids(&idx, &node_ids(&[1]), &[q], 2, SimilarityMode::Dot),
        Err(DenseError::UnsupportedDType(DType::BF16))
    );
    assert!(idx.calls().is_empty());
}

#[test]
fn centroids_propagate_index_errors_in_node_order() {
    let idx = MockIndex::new(
        DType::F32,
        vec![
            (1, Ok(f32_bytes(&[1.0, 0.0]))),
            (2, Err(IndexError::Unsupported(AccessMethod::Centroid))),
        ],
    );
    let q: &[f32] = &[1.0, 0.0];
    assert_eq!(
        score_centroids(&idx, &node_ids(&[1, 2, 99]), &[q], 2, SimilarityMode::Dot),
        Err(DenseError::Index(IndexError::Unsupported(AccessMethod::Centroid)))
    );
    assert_eq!(idx.calls(), vec![1, 2]);
    assert_eq!(
        score_centroids(&idx, &node_ids(&[99]), &[q], 2, SimilarityMode::Dot),
        Err(DenseError::Index(IndexError::NotFound))
    );
}

#[test]
fn centroids_shape_errors_use_node_position() {
    let idx = MockIndex::new(
        DType::F32,
        vec![
            (1, Ok(f32_bytes(&[1.0, 0.0]))),
            (2, Ok(Vec::new())),
            (3, Ok(f32_bytes(&[1.0, 0.0, 0.0]))),
            (4, Ok(vec![0u8; 7])),
        ],
    );
    let q: &[f32] = &[1.0, 0.0];
    let run = |ids: &[u64]| score_centroids(&idx, &node_ids(ids), &[q], 2, SimilarityMode::Dot);
    assert_eq!(
        run(&[1, 2, 3]),
        Err(DenseError::Shape { what: ShapeSource::Slot(1), len: 0, dim: 2 })
    );
    assert_eq!(
        run(&[1, 3]),
        Err(DenseError::Shape { what: ShapeSource::Slot(1), len: 3, dim: 2 })
    );
    assert_eq!(
        run(&[4]),
        Err(DenseError::Shape { what: ShapeSource::Slot(0), len: 1, dim: 2 })
    );
}

#[test]
fn centroids_f16_odd_byte_length_is_shape_error() {
    let idx = MockIndex::new(DType::F16, vec![(1, Ok(vec![0u8; 5]))]);
    let q: &[f32] = &[1.0, 0.0];
    assert_eq!(
        score_centroids(&idx, &node_ids(&[1]), &[q], 2, SimilarityMode::Dot),
        Err(DenseError::Shape { what: ShapeSource::Slot(0), len: 2, dim: 2 })
    );
}

#[test]
fn centroid_query_checks_run_before_summary_calls() {
    let idx = MockIndex::new(DType::F32, vec![(1, Ok(f32_bytes(&[1.0, 0.0])))]);
    let zero: &[f32] = &[0.0, 0.0];
    let short: &[f32] = &[1.0];
    let ok: &[f32] = &[1.0, 0.0];
    let nodes = node_ids(&[1]);
    assert_eq!(
        score_centroids(&idx, &nodes, &[ok], 0, SimilarityMode::Dot),
        Err(DenseError::InvalidDim)
    );
    assert_eq!(
        score_centroids(&idx, &nodes, &[short], 2, SimilarityMode::Dot),
        Err(DenseError::Shape { what: ShapeSource::Query(0), len: 1, dim: 2 })
    );
    assert_eq!(
        score_centroids(&idx, &nodes, &[ok, zero], 2, SimilarityMode::Exact),
        Err(DenseError::EmptyEmbedding { query_index: 1 })
    );
    assert!(idx.calls().is_empty());
}

#[test]
fn concurrent_calls_agree() {
    let q: Vec<f32> = (0..64).map(|i| (i as f32).sin() + 0.5).collect();
    let slots: Vec<Vec<f32>> = (0..20)
        .map(|j| (0..64).map(|i| ((i * j) as f32).cos()).collect())
        .collect();
    let reference = {
        let refs: Vec<&[f32]> = slots.iter().map(|s| s.as_slice()).collect();
        score(&[&q], &refs, 64, SimilarityMode::Exact).unwrap()
    };
    std::thread::scope(|scope| {
        for _ in 0..4 {
            scope.spawn(|| {
                let refs: Vec<&[f32]> = slots.iter().map(|s| s.as_slice()).collect();
                let r = score(&[&q], &refs, 64, SimilarityMode::Exact).unwrap();
                assert_eq!(r, reference);
            });
        }
    });
}
