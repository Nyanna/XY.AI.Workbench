use super::*;

fn hit(offset: u64, score: f32) -> KernelHit {
    KernelHit { offset, score }
}

#[test]
fn identical_results_have_no_deviation() {
    let r = vec![vec![hit(1, 0.9), hit(2, 0.5)]];
    assert!(compare(&r, &r.clone(), Tolerance::F32).is_empty());
}

#[test]
fn drift_within_tolerance_is_accepted() {
    let r = vec![vec![hit(1, 0.9), hit(2, 0.5)]];
    let c = vec![vec![hit(1, 0.9004), hit(2, 0.4996)]];
    assert!(compare(&r, &c, Tolerance::F16).is_empty());
    assert!(!compare(&r, &c, Tolerance::F32).is_empty());
}

#[test]
fn swap_at_near_tie_is_accepted() {
    let r = vec![vec![hit(1, 0.9000), hit(2, 0.8995), hit(3, 0.2)]];
    let c = vec![vec![hit(2, 0.9000), hit(1, 0.8995), hit(3, 0.2)]];
    assert!(compare(&r, &c, Tolerance::F16).is_empty());
}

#[test]
fn swap_of_distinct_scores_is_reported() {
    let r = vec![vec![hit(1, 0.9), hit(2, 0.5)]];
    let c = vec![vec![hit(2, 0.9), hit(1, 0.5)]];
    let d = compare(&r, &c, Tolerance::F16);
    assert!(d.iter().any(|d| matches!(d, Deviation::RankMismatch { rank: 0, .. })));
}

#[test]
fn hit_count_mismatch_is_reported() {
    let r = vec![vec![hit(1, 0.9)]];
    let c = vec![vec![]];
    assert_eq!(
        compare(&r, &c, Tolerance::F16),
        vec![Deviation::HitCount { query: 0, expected: 1, actual: 0 }]
    );
}
