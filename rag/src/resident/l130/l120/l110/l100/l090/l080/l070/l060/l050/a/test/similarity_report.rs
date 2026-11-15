use std::cell::RefCell;
use super::*;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::b::similarity_task::IndexId;
fn eid(n: u64) -> EntryId {
    EntryId(n)
}
fn hit(id: u64, score: f32, stale: bool) -> ScoredHit {
    ScoredHit {
        entry_id: eid(id),
        score,
        stale,
        source_link: Some("ignored".to_string()),
    }
}
fn input(hits: Vec<ScoredHit>) -> ScoredHitResult {
    ScoredHitResult {
        scored_count: hits.len(),
        dropped_count: 0,
        hits,
    }
}
fn params(top_k: usize) -> SimilarityParams {
    SimilarityParams::new(IndexId(7)).with_top_k(top_k)
}
fn opts() -> ReportOptions {
    ReportOptions { temperature: 1.0 }
}
fn no_link(_: EntryId) -> Option<String> {
    None
}
fn close(a: f64, b: f64) -> bool {
    (a - b).abs() < 1e-9
}
#[test]
fn missing_params_fails_without_lookup() {
    let calls = RefCell::new(0usize);
    let lookup = |_: EntryId| {
        *calls.borrow_mut() += 1;
        None
    };
    let r = build_report(
        TaskId(1),
        None,
        &input(vec![hit(1, 0.5, false)]),
        opts(),
        &lookup,
    );
    assert_eq!(r.unwrap_err(), ReportError::MissingSearchParams);
    assert_eq!(* calls.borrow(), 0);
}
#[test]
fn error_display_is_nonempty() {
    assert!(! ReportError::MissingSearchParams.to_string().is_empty());
}
#[test]
fn sorts_descending_with_id_tiebreak_and_truncates() {
    let inp = input(
        vec![
            hit(5, 0.5, false), hit(9, 0.9, false), hit(2, 0.5, false), hit(1, 0.1,
            false),
        ],
    );
    let p = params(3);
    let r = build_report(TaskId(3), Some(&p), &inp, opts(), &no_link).unwrap();
    let ids: Vec<u64> = r.hits.iter().map(|h| h.entry_id.0).collect();
    assert_eq!(ids, vec![9, 2, 5]);
    assert_eq!(r.task_id, TaskId(3));
    assert_eq!(r.candidate_count, 4);
    assert_eq!(r.params.top_k, 3);
    assert_eq!(inp.hits.len(), 4);
    assert_eq!(inp.hits[0].entry_id, eid(5));
}
#[test]
fn invalid_hits_are_dropped_and_counted() {
    let mut inp = input(
        vec![
            hit(1, f32::NAN, false), hit(2, f32::INFINITY, false), hit(3,
            f32::NEG_INFINITY, false), hit(4, 0.4, false),
        ],
    );
    inp.dropped_count = 2;
    inp.scored_count = 10;
    let p = params(10);
    let r = build_report(TaskId(1), Some(&p), &inp, opts(), &no_link).unwrap();
    assert_eq!(r.hits.len(), 1);
    assert_eq!(r.dropped_count, 5);
    assert_eq!(r.candidate_count, 10);
}
#[test]
fn dropped_count_saturates() {
    let mut inp = input(vec![hit(1, f32::NAN, false)]);
    inp.dropped_count = usize::MAX;
    let p = params(5);
    let r = build_report(TaskId(1), Some(&p), &inp, opts(), &no_link).unwrap();
    assert_eq!(r.dropped_count, usize::MAX);
}
#[test]
fn top_k_zero_gives_empty_report() {
    let p = params(0);
    let r = build_report(
            TaskId(1),
            Some(&p),
            &input(vec![hit(1, 0.5, true)]),
            opts(),
            &no_link,
        )
        .unwrap();
    assert!(r.hits.is_empty());
    assert!(r.metrics.is_none());
    assert_eq!(r.stale_count, 0);
}
#[test]
fn empty_input_and_large_top_k() {
    let p = params(100);
    let r = build_report(TaskId(1), Some(&p), &input(vec![]), opts(), &no_link).unwrap();
    assert!(r.hits.is_empty());
    assert!(r.metrics.is_none());
}
#[test]
fn duplicates_are_kept() {
    let p = params(10);
    let inp = input(vec![hit(1, 0.5, false), hit(1, 0.5, false)]);
    let r = build_report(TaskId(1), Some(&p), &inp, opts(), &no_link).unwrap();
    assert_eq!(r.hits.len(), 2);
}
#[test]
fn stale_counts_only_returned_hits() {
    let p = params(2);
    let inp = input(
        vec![
            hit(1, 0.9, true), hit(2, 0.8, false), hit(3, 0.7, true), hit(4, f32::NAN,
            true),
        ],
    );
    let r = build_report(TaskId(1), Some(&p), &inp, opts(), &no_link).unwrap();
    assert_eq!(r.stale_count, 1);
    assert!(r.hits[0].stale);
    assert!(! r.hits[1].stale);
}
#[test]
fn lookup_called_once_per_returned_hit_in_rank_order() {
    let calls = RefCell::new(Vec::new());
    let lookup = |id: EntryId| {
        calls.borrow_mut().push(id.0);
        if id.0 == 2 { None } else { Some(format!("link-{}", id.0)) }
    };
    let p = params(2);
    let inp = input(
        vec![
            hit(1, 0.1, false), hit(2, 0.9, false), hit(3, 0.5, false), hit(4, f32::NAN,
            false)
        ],
    );
    let r = build_report(TaskId(1), Some(&p), &inp, opts(), &lookup).unwrap();
    assert_eq!(* calls.borrow(), vec![2, 3]);
    assert_eq!(r.hits[0].source_link, None);
    assert_eq!(r.hits[1].source_link.as_deref(), Some("link-3"));
}
#[test]
fn input_source_link_is_ignored() {
    let p = params(1);
    let r = build_report(
            TaskId(1),
            Some(&p),
            &input(vec![hit(1, 0.5, false)]),
            opts(),
            &no_link,
        )
        .unwrap();
    assert_eq!(r.hits[0].source_link, None);
}
#[test]
fn metrics_basic_statistics() {
    let m = compute_metrics(
            &[0.9, 0.7, 0.5, 0.1],
            &[eid(1), eid(2), eid(3), eid(4)],
            1.0,
        )
        .unwrap();
    assert_eq!(m.count, 4);
    assert_eq!(m.best, 0.9);
    assert_eq!(m.worst, 0.1);
    let s = [0.9f32, 0.7, 0.5, 0.1].map(f64::from);
    let mean = s.iter().sum::<f64>() / 4.0;
    assert!(close(m.mean, mean));
    let sd = (s.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / 4.0).sqrt();
    assert!(close(m.std_dev, sd));
    assert!(close(m.gap.unwrap(), s[0] - s[1]));
    assert!(close(m.spread.unwrap(), s[0] - s[3]));
    assert!(close(m.z_score.unwrap(), (s[0] - mean) / sd));
}
#[test]
fn metrics_empty_is_none() {
    assert!(compute_metrics(& [], & [], 1.0).is_none());
}
#[test]
fn metrics_single_hit() {
    let m = compute_metrics(&[0.5], &[eid(1)], 1.0).unwrap();
    assert_eq!(m.count, 1);
    assert!(m.gap.is_none());
    assert!(m.spread.is_none());
    assert!(m.z_score.is_none());
    assert!(m.knee.is_none());
    let sm = m.softmax.unwrap();
    assert_eq!(sm.probabilities, vec![1.0]);
    assert_eq!(sm.top_probability, 1.0);
    assert_eq!(sm.entropy, 0.0);
    assert!(sm.entropy.is_sign_positive());
}
#[test]
fn z_score_none_for_constant_scores() {
    let m = compute_metrics(&[0.5, 0.5, 0.5], &[eid(1), eid(2), eid(3)], 1.0).unwrap();
    assert!(m.z_score.is_none());
    assert!(m.std_dev < 1e-9);
}
#[test]
fn softmax_sums_to_one_and_matches_formula() {
    let m = compute_metrics(&[1.0, 0.5, 0.0], &[eid(1), eid(2), eid(3)], 0.5).unwrap();
    let sm = m.softmax.unwrap();
    assert_eq!(sm.temperature, 0.5);
    assert!(close(sm.probabilities.iter().sum::< f64 > (), 1.0));
    let w = [1.0f64, (-1.0f64).exp(), (-2.0f64).exp()];
    let total: f64 = w.iter().sum();
    for (p, w) in sm.probabilities.iter().zip(w.iter()) {
        assert!(close(* p, w / total));
    }
    assert!(close(sm.top_probability, sm.probabilities[0]));
    let h: f64 = -sm.probabilities.iter().map(|p| p * p.ln()).sum::<f64>();
    assert!(close(sm.entropy, h));
    assert!(sm.entropy >= 0.0);
}
#[test]
fn softmax_uniform_entropy_is_ln_n() {
    let m = compute_metrics(&[0.5; 4], &[eid(1), eid(2), eid(3), eid(4)], 1.0).unwrap();
    let sm = m.softmax.unwrap();
    assert!(close(sm.entropy, 4f64.ln()));
}
#[test]
fn softmax_invalid_temperature_is_none() {
    for t in [0.0f32, -1.0, f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
        let m = compute_metrics(&[0.9, 0.5], &[eid(1), eid(2)], t).unwrap();
        assert!(m.softmax.is_none(), "temperature {t}");
        assert!(m.gap.is_some());
    }
}
#[test]
fn softmax_is_stable_for_extreme_values() {
    let m = compute_metrics(&[1e30, -1e30], &[eid(1), eid(2)], 1e-6).unwrap();
    let sm = m.softmax.unwrap();
    assert!(sm.probabilities.iter().all(| p | p.is_finite()));
    assert!(close(sm.top_probability, 1.0));
    assert!(sm.entropy >= 0.0);
}
#[test]
fn knee_needs_three_points() {
    let m = compute_metrics(&[0.9, 0.1], &[eid(1), eid(2)], 1.0).unwrap();
    assert!(m.knee.is_none());
}
#[test]
fn knee_detects_largest_depth() {
    let ids = [eid(10), eid(11), eid(12), eid(13), eid(14)];
    let m = compute_metrics(&[1.0, 0.9, 0.2, 0.15, 0.1], &ids, 1.0).unwrap();
    let k = m.knee.unwrap();
    assert_eq!(k.rank, 2);
    assert_eq!(k.entry_id, eid(12));
    let line = 1.0 + (0.1f32 as f64 - 1.0) * 2.0 / 4.0;
    assert!(close(k.depth, line - f64::from(0.2f32)));
    assert!(k.depth > 0.0);
}
#[test]
fn knee_none_for_flat_or_convex_curves() {
    let ids = [eid(1), eid(2), eid(3)];
    assert!(compute_metrics(& [0.5, 0.5, 0.5], & ids, 1.0).unwrap().knee.is_none());
    assert!(compute_metrics(& [1.0, 0.2, 0.0], & ids, 1.0).unwrap().knee.is_some());
    assert!(compute_metrics(& [1.0, 0.6, 0.0], & ids, 1.0).unwrap().knee.is_none());
}
#[test]
fn knee_tie_prefers_earlier_rank() {
    let ids = [eid(1), eid(2), eid(3), eid(4)];
    let m = compute_metrics(&[1.0, 0.0, 0.0, 0.0], &ids, 1.0).unwrap();
    let k = m.knee.unwrap();
    assert_eq!(k.rank, 1);
}
#[test]
fn report_metrics_follow_returned_hits_only() {
    let p = params(2);
    let inp = input(vec![hit(1, 0.9, false), hit(2, 0.7, false), hit(3, 0.0, false)]);
    let r = build_report(TaskId(1), Some(&p), &inp, opts(), &no_link).unwrap();
    let m = r.metrics.unwrap();
    assert_eq!(m.count, 2);
    assert_eq!(m.worst, 0.7);
    assert_eq!(m.softmax.unwrap().probabilities.len(), 2);
}
#[test]
fn report_is_deterministic() {
    let p = params(3);
    let inp = input(
        vec![
            hit(3, 0.3, false), hit(1, 0.9, true), hit(2, 0.3, false), hit(4, 0.1, false)
        ],
    );
    let a = build_report(TaskId(1), Some(&p), &inp, opts(), &no_link).unwrap();
    let b = build_report(TaskId(1), Some(&p), &inp, opts(), &no_link).unwrap();
    assert_eq!(a.hits, b.hits);
    assert_eq!(a.metrics, b.metrics);
}
#[test]
fn negative_zero_ties_with_zero_by_id() {
    let p = params(2);
    let inp = input(vec![hit(2, 0.0, false), hit(1, - 0.0, false)]);
    let r = build_report(TaskId(1), Some(&p), &inp, opts(), &no_link).unwrap();
    assert_eq!(r.hits[0].entry_id, eid(1));
}
#[test]
fn report_equals_its_clone_and_carries_params() {
    let p = params(2);
    let inp = input(vec![hit(1, 0.9, true), hit(2, 0.4, false)]);
    let r = build_report(TaskId(1), Some(&p), &inp, opts(), &no_link).unwrap();
    assert_eq!(r, r.clone());
    assert_eq!(r.params, p);
    let other = build_report(TaskId(2), Some(&p), &inp, opts(), &no_link).unwrap();
    assert_ne!(r, other);
}
