use super::*;
use std::collections::BTreeMap;

fn prof(position: f64) -> PrecisionProfile {
    PrecisionProfile {
        position,
        config: BTreeMap::new(),
    }
}

#[test]
fn clamp_behaviour() {
    assert_eq!(clamp_wish(-1.0), Ok(0.0));
    assert_eq!(clamp_wish(2.0), Ok(1.0));
    assert_eq!(clamp_wish(f64::INFINITY), Ok(1.0));
    assert_eq!(clamp_wish(f64::NEG_INFINITY), Ok(0.0));
    assert_eq!(clamp_wish(0.3), Ok(0.3));
    assert_eq!(clamp_wish(f64::NAN), Err(SelectionError::InvalidPrecision));
}

#[test]
fn picks_closest_and_ties_to_more_precise() {
    let s = ProfileSelector::new();
    let ps = vec![prof(0.0), prof(0.5), prof(1.0)];
    assert_eq!(s.select(0.4, &ps).unwrap().position, 0.5);
    assert_eq!(s.select(0.25, &ps).unwrap().position, 0.5);
    assert_eq!(s.select(-5.0, &ps).unwrap().position, 0.0);
    assert_eq!(s.select(9.0, &ps).unwrap().position, 1.0);
}

#[test]
fn equal_positions_take_first() {
    let s = ProfileSelector::new();
    let mut a = prof(0.5);
    a.config.clear();
    let ps = vec![a, prof(0.5)];
    let r = s.select(0.5, &ps).unwrap();
    assert!(std::ptr::eq(r, &ps[0]));
}

#[test]
fn empty_latches_and_unlock_releases() {
    let s = ProfileSelector::new();
    assert_eq!(s.select(0.5, &[]), Err(SelectionError::NoActiveProfile));
    assert!(s.is_locked());
    let ps = vec![prof(0.5)];
    assert_eq!(s.select(0.5, &ps), Err(SelectionError::NoActiveProfile));
    assert_eq!(s.select(f64::NAN, &ps), Err(SelectionError::InvalidPrecision));
    assert!(s.is_locked());
    assert!(s.unlock());
    assert!(!s.unlock());
    assert!(s.select(0.5, &ps).is_ok());
    assert!(!s.is_locked());
}

#[test]
fn invalid_positions_do_not_latch() {
    let s = ProfileSelector::new();
    for bad in [f64::NAN, -0.1, 1.1] {
        let ps = vec![prof(0.5), prof(bad)];
        assert_eq!(s.select(0.5, &ps), Err(SelectionError::InvalidPrecision));
        assert!(!s.is_locked());
    }
}

#[test]
fn selectors_are_independent_and_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<ProfileSelector>();
    let a = ProfileSelector::new();
    let b = ProfileSelector::new();
    let _ = a.select(0.5, &[]);
    assert!(a.is_locked());
    assert!(!b.is_locked());
    assert!(ProfileSelector::is_satisfiable(&[prof(0.0)]));
    assert!(!ProfileSelector::is_satisfiable(&[]));
}

#[test]
fn concurrent_empty_selects_all_fail_and_latch() {
    let s = ProfileSelector::new();
    std::thread::scope(|sc| {
        for _ in 0..8 {
            sc.spawn(|| {
                assert_eq!(s.select(0.5, &[]), Err(SelectionError::NoActiveProfile));
            });
        }
    });
    assert!(s.is_locked());
}
