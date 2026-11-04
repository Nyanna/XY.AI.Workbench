use super::*;
use std::collections::HashSet;
use std::sync::Arc;

fn assert_send_sync<T: Send + Sync>() {}
fn assert_copy<T: Copy>() {}

#[test]
fn auto_traits() {
    assert_send_sync::<StampCounter>();
    assert_send_sync::<ChangeStamp>();
    assert_copy::<ChangeStamp>();
}

#[test]
fn new_counter_is_empty_and_counts_from_one() {
    let c = StampCounter::new();
    assert_eq!(c.max(), None);
    assert_eq!(c.max_raw(), None);
    assert_eq!(c.next().unwrap().as_raw(), 1);
    assert_eq!(c.next().unwrap().as_raw(), 2);
    assert_eq!(c.max_raw(), Some(2));
}

#[test]
fn max_matches_last_issued() {
    let c = StampCounter::default();
    let s = c.next().unwrap();
    assert_eq!(c.max(), Some(s));
}

#[test]
fn resume_uses_larger_input() {
    let c = StampCounter::resume(Some(5), Some(9));
    assert_eq!(c.max_raw(), Some(9));
    assert_eq!(c.next().unwrap().as_raw(), 10);
    let c = StampCounter::resume(Some(9), Some(5));
    assert_eq!(c.next().unwrap().as_raw(), 10);
    let c = StampCounter::resume(Some(4), None);
    assert_eq!(c.next().unwrap().as_raw(), 5);
}

#[test]
fn resume_empty_behaves_like_new() {
    for c in [
        StampCounter::resume(None, None),
        StampCounter::resume(Some(0), Some(0)),
    ] {
        assert_eq!(c.max_raw(), None);
        assert_eq!(c.next().unwrap().as_raw(), 1);
    }
}

#[test]
fn exhaustion_is_stable() {
    let c = StampCounter::resume(Some(u64::MAX - 1), None);
    assert_eq!(c.next().unwrap().as_raw(), u64::MAX);
    assert_eq!(c.next(), Err(StampError::Exhausted));
    assert_eq!(c.next(), Err(StampError::Exhausted));
    assert_eq!(c.max_raw(), Some(u64::MAX));
    assert!(!StampError::Exhausted.to_string().is_empty());
}

#[test]
fn raw_round_trip() {
    assert_eq!(ChangeStamp::from_raw(42).as_raw(), 42);
    assert_eq!(ChangeStamp::from_raw(7), ChangeStamp::from_raw(7));
    assert_ne!(ChangeStamp::from_raw(7), ChangeStamp::from_raw(8));
}

#[test]
fn concurrent_next_is_unique_and_gapless() {
    let c = Arc::new(StampCounter::new());
    let handles: Vec<_> = (0..8)
        .map(|_| {
            let c = Arc::clone(&c);
            std::thread::spawn(move || {
                (0..1000)
                    .map(|_| c.next().unwrap().as_raw())
                    .collect::<Vec<_>>()
            })
        })
        .collect();
    let mut all = HashSet::new();
    for h in handles {
        for v in h.join().unwrap() {
            assert!(all.insert(v));
        }
    }
    assert_eq!(all.len(), 8000);
    assert!((1..=8000u64).all(|v| all.contains(&v)));
    assert_eq!(c.max_raw(), Some(8000));
}
