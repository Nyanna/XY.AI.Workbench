use super::*;

fn m(prec: &str, time: f64, deviation: f64) -> Measurement {
    let mut config = BTreeMap::new();
    config.insert(Axis::from("operands"), Value::from(prec));
    Measurement { config, time, deviation }
}

#[test]
fn front_basic_and_non_finite() {
    let f = pareto_front_pairs(&[
        (3.0, 0.1),
        (1.0, 0.9),
        (2.0, 0.95),
        (2.0, 0.5),
        (4.0, 0.1),
        (f64::NAN, 0.0),
        (1.0, f64::INFINITY),
    ]);
    assert_eq!(f, vec![(1.0, 0.9), (2.0, 0.5), (3.0, 0.1)]);
    assert!(pareto_front_pairs(&[]).is_empty());
    assert!(pareto_front_pairs(&[(f64::NAN, 1.0)]).is_empty());
}

#[test]
fn front_duplicates_keep_first() {
    let items = vec![("a", 1.0, 0.5), ("b", 1.0, 0.5)];
    let f = pareto_front(&items, |x| (x.1, x.2));
    assert_eq!(f, vec![("a", 1.0, 0.5)]);
}

#[test]
fn positions_edge_cases() {
    assert!(positions(&[], false).is_empty());
    assert_eq!(positions(&[(1.0, 1.0)], true), vec![0.5]);
    assert_eq!(positions(&[(1.0, 1.0), (1.0, 1.0), (1.0, 1.0)], false), vec![0.0, 0.5, 1.0]);
    let p = positions(&[(1.0, 1.0), (2.0, 0.5), (4.0, 0.0)], false);
    assert_eq!(p.len(), 3);
    assert_eq!(p[0], 0.0);
    assert_eq!(p[2], 1.0);
    assert!(p[0] <= p[1] && p[1] <= p[2]);
    let q = positions(&[(0.0, 1.0), (1.0, 0.0)], true);
    assert_eq!(q, vec![0.0, 1.0]);
}

#[test]
fn invalid_threshold() {
    for t in [f64::NAN, f64::INFINITY, -0.1] {
        assert!(matches!(derive_profiles(&[], t), Err(ProfileError::InvalidThreshold(_))));
    }
    assert!(derive_profiles(&[], 0.0).unwrap().is_empty());
}

#[test]
fn empty_and_non_finite_give_empty() {
    assert!(derive_profiles(&[], DEFAULT_MERGE_THRESHOLD).unwrap().is_empty());
    let ms = [m("a", f64::NAN, 0.1), m("b", -1.0, 0.1), m("c", 0.0, 0.1)];
    assert!(derive_profiles(&ms, DEFAULT_MERGE_THRESHOLD).unwrap().is_empty());
}

#[test]
fn single_gets_middle_position() {
    let r = derive_profiles(&[m("a", 1.0, 0.1)], DEFAULT_MERGE_THRESHOLD).unwrap();
    assert_eq!(r.len(), 1);
    assert_eq!(r[0].position, 0.5);
}

#[test]
fn filter_removes_slow_imprecise_neighbours() {
    let ms = [m("fast", 1.0, 0.9), m("near", 1.01, 0.5), m("slow", 100.0, 0.01)];
    let r = derive_profiles(&ms, DEFAULT_MERGE_THRESHOLD).unwrap();
    let names: Vec<_> = r.iter().map(|p| p.config.values().next().unwrap().as_str().to_string()).collect();
    assert_eq!(names, vec!["near", "slow"]);
    assert_eq!(r[0].position, 0.0);
    assert_eq!(r[1].position, 1.0);
}

#[test]
fn zero_threshold_merges_nothing() {
    let ms = [m("a", 1.0, 0.9), m("b", 1.001, 0.5), m("c", 1.002, 0.1)];
    let r = derive_profiles(&ms, 0.0).unwrap();
    assert_eq!(r.len(), 3);
}

#[test]
fn order_independent_and_bounded_count() {
    let mut ms: Vec<Measurement> = (0..50)
        .map(|i| m(&format!("p{i}"), 1.0 + i as f64 * 3.0, 1.0 / (1.0 + i as f64)))
        .collect();
    let a = derive_profiles(&ms, DEFAULT_MERGE_THRESHOLD).unwrap();
    ms.reverse();
    let b = derive_profiles(&ms, DEFAULT_MERGE_THRESHOLD).unwrap();
    assert_eq!(a, b);
    assert!(a.len() <= 11 && !a.is_empty());
    assert!(a.windows(2).all(|w| w[0].position <= w[1].position));
    assert!(a.iter().all(|p| (0.0..=1.0).contains(&p.position)));
}

#[test]
fn types_are_send_sync() {
    fn check<T: Send + Sync>() {}
    check::<Measurement>();
    check::<PrecisionProfile>();
}
