use super::*;
use crate::resident::index::payload::Payload as SourceRef;
use crate::resident::index::{EntryId, Scope};
use crate::resident::manager::slot::Hit;
use crate::resident::op::{IndexId, ModelId};
use crate::resident::facade::ScorerKind;

fn hit(entry: u64, score: f32) -> Hit {
    Hit { entry: EntryId::new(entry), score, outdated: false }
}

fn params(k: usize) -> SimilarityParams {
    SimilarityParams::new(IndexId("idx".into())).top_k(k)
}

fn no_source(_: EntryId) -> Option<SourceRef> {
    None
}

fn report(k: usize, scores: &[f32]) -> SimilarityReport {
    let hits = scores.iter().enumerate().map(|(i, &s)| hit(i as u64, s)).collect();
    SimilarityReport::build(1, &params(k), scores.len() as u64, hits, 0, &no_source)
}

fn close(a: f32, b: f32) -> bool {
    (a - b).abs() < 1e-5
}

#[test]
fn orders_truncates_and_computes_summary_over_top_k() {
    let r = report(3, &[0.4, 0.9, 0.7, 0.8]);
    let scores: Vec<f32> = r.hits.iter().map(|h| h.score).collect();
    assert_eq!(scores, [0.9, 0.8, 0.7]);
    assert_eq!(r.count(), 3);
    assert_eq!(r.candidates, 4);
    assert!(close(r.s1().unwrap(), 0.9));
    assert!(close(r.sk().unwrap(), 0.7));
    assert!(close(r.mean().unwrap(), 0.8));
    assert!(close(r.std().unwrap(), (0.02f32 / 3.0).sqrt()));
    assert!(close(r.gap_top2().unwrap(), 0.1));
    assert!(close(r.gap_to_k().unwrap(), 0.2));
    assert!(close(r.z_top1().unwrap(), 0.1 / r.std().unwrap()));
}

#[test]
fn ties_prefer_lower_entry_and_nan_is_discarded() {
    let hits = vec![hit(5, 0.5), hit(2, 0.5), hit(9, f32::NAN)];
    let r = SimilarityReport::build(7, &params(5), 3, hits, 1, &no_source);
    let entries: Vec<u64> = r.hits.iter().map(|h| h.entry.get()).collect();
    assert_eq!(entries, [2, 5]);
    assert_eq!(r.discarded, 2);
    assert_eq!(r.task_id, 7);
}

#[test]
fn payload_and_outdated_flag_are_carried() {
    let hits = vec![
        hit(1, 0.9),
        Hit { entry: EntryId::new(2), score: 0.8, outdated: true },
    ];
    let source = |e: EntryId| (e.get() == 1).then(|| SourceRef::new("a.md", 10, 42));
    let r = SimilarityReport::build(1, &params(10), 2, hits, 0, &source);
    assert_eq!(r.hits[0].source, Some(SourceRef::new("a.md", 10, 42)));
    assert_eq!(r.hits[1].source, None);
    assert_eq!(r.outdated, 1);
    assert!(r.hits[1].outdated);
}

#[test]
fn empty_result_has_no_statistics() {
    let r = report(5, &[]);
    assert_eq!(r.count(), 0);
    assert!(r.s1().is_none() && r.sk().is_none() && r.mean().is_none() && r.std().is_none());
    assert!(r.gap_top2().is_none() && r.gap_to_k().is_none() && r.z_top1().is_none());
    assert!(r.softmax(0.05).is_none() && r.elbow().is_none());
}

#[test]
fn single_hit_has_no_gap_and_flat_scores_have_no_z() {
    let one = report(5, &[0.8]);
    assert!(one.gap_top2().is_none() && one.gap_to_k().is_none());
    assert!(close(one.s1().unwrap(), one.sk().unwrap()));
    assert!(report(5, &[0.8, 0.8, 0.8]).z_top1().is_none());
}

#[test]
fn softmax_is_a_distribution_and_entropy_orders_confidence() {
    let clear = report(4, &[0.95, 0.80, 0.78, 0.75]);
    let unsure = report(4, &[0.90, 0.89, 0.89, 0.88]);
    let p = clear.softmax(0.02).unwrap();
    assert!(close(p.iter().sum(), 1.0));
    assert!(clear.top1_mass(0.02).unwrap() > unsure.top1_mass(0.02).unwrap());
    assert!(clear.entropy(0.02).unwrap() < unsure.entropy(0.02).unwrap());
    assert!(unsure.entropy(0.02).unwrap() <= (4f32).ln() + 1e-5);
}

#[test]
fn softmax_rejects_invalid_temperature() {
    let r = report(3, &[0.9, 0.8, 0.7]);
    for t in [0.0, -1.0, f32::NAN, f32::INFINITY] {
        assert!(r.softmax(t).is_none(), "t={t}");
    }
}

#[test]
fn softmax_is_stable_for_tiny_temperature() {
    let r = report(3, &[0.9, 0.5, 0.1]);
    let p = r.softmax(1e-4).unwrap();
    assert!(p.iter().all(|x| x.is_finite()));
    assert!(close(p[0], 1.0));
}

#[test]
fn elbow_marks_the_first_hit_after_the_drop() {
    let r = report(6, &[0.90, 0.89, 0.88, 0.50, 0.49, 0.48]);
    assert_eq!(r.elbow(), Some(3));
    assert!(report(6, &[0.9, 0.8]).elbow().is_none());
    assert!(report(6, &[0.7, 0.7, 0.7]).elbow().is_none());
    assert!(report(6, &[0.9, 0.8, 0.7]).elbow().is_none());
}

#[test]
fn scorer_defaults_to_cosine_and_scope_kind_is_labelled() {
    let p = params(3);
    assert_eq!(p.get_scorer(), &ScorerKind::Cosine);
    assert_eq!(p.get_scorer().label(), "cosine");
    assert_eq!(Scope::All.kind(), "all");
    let ce = ScorerKind::CrossEncoder(ModelId("m".into()));
    assert_eq!(ce.label(), "cross_encoder");
    assert_eq!(p.scorer(ce.clone()).get_scorer(), &ce);
}
