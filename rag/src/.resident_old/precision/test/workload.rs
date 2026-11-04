use super::*;
use crate::resident::precision::metrics::is_duplicate_free;
use crate::resident::precision::profile::ProfileConfig;
use crate::resident::precision::measure::Validity;

fn small() -> WorkloadSpec {
    WorkloadSpec { docs: 200, dim: 32, max_queries: 8, ..WorkloadSpec::default() }
}

fn cfg_with_batch(b: &str) -> Cfg {
    Cfg::new(ProfileConfig::new().with(&*BATCH, b), vec![])
}

#[test]
fn rows_are_unit_length_and_distinct() {
    let w = ScoringWorkload::generate(small());
    let rows: Vec<Vec<u32>> = w
        .docs()
        .chunks(32)
        .map(|r| r.iter().map(|x| x.to_bits()).collect())
        .collect();
    assert!(is_duplicate_free(&rows));
    for r in w.docs().chunks(32) {
        let n: f32 = r.iter().map(|x| x * x).sum::<f32>().sqrt();
        assert!((n - 1.0).abs() < 1e-5);
    }
}

#[test]
fn generation_is_deterministic() {
    let a = ScoringWorkload::generate(small());
    let b = ScoringWorkload::generate(small());
    assert_eq!(a.docs(), b.docs());
    assert_eq!(a.reference(8), b.reference(8));
}

#[test]
fn reference_as_candidate_is_a_perfect_match() {
    let w = ScoringWorkload::generate(small());
    let r = w.judge(w.reference(4), 4);
    assert_eq!(r.valid, Validity::Ok);
    assert_eq!(r.deviation, 0.0);
}

#[test]
fn nan_scores_are_flagged() {
    let w = ScoringWorkload::generate(small());
    let mut s = w.reference(2).to_vec();
    s[5] = f32::NAN;
    assert_eq!(w.judge(&s, 2).valid, Validity::NonFinite);
}

#[test]
fn constant_scores_are_degenerate() {
    let w = ScoringWorkload::generate(small());
    let s = vec![0.25f32; 2 * 200];
    assert_eq!(w.judge(&s, 2).valid, Validity::Degenerate);
}

#[test]
fn batch_axis_selects_the_query_count() {
    let w = ScoringWorkload::generate(small());
    assert_eq!(w.query_count(&cfg_with_batch("4")).unwrap(), 4);
    assert_eq!(
        w.query_count(&Cfg::new(ProfileConfig::new(), vec![])).unwrap(),
        WorkloadSpec::DEFAULT_QUERIES
    );
    assert!(w.query_count(&cfg_with_batch("9")).is_err());
    assert!(w.query_count(&cfg_with_batch("0")).is_err());
    assert!(w.query_count(&cfg_with_batch("x")).is_err());
}
