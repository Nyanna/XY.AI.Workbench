use super::*;
use std::collections::HashMap;

fn arena(kind: ArenaKind, capacity: u64) -> Arena {
    Arena { kind, capacity }
}

fn assert_send_sync<T: Send + Sync + 'static>() {}

#[test]
fn static_properties() {
    assert_send_sync::<RegionHandle>();
    assert_send_sync::<Generation>();
    assert!(std::mem::size_of::<RegionHandle>() <= 32);
}

#[test]
fn new_overflow_and_empty() {
    let g = Generation::INITIAL;
    assert_eq!(
        RegionHandle::new(ArenaKind::Weights, u64::MAX, 1, g),
        Err(RegionError::Overflow)
    );
    let h = RegionHandle::new(ArenaKind::Weights, 10, 0, g).unwrap();
    assert!(h.is_empty());
    assert_eq!(h.end(), 10);
    let h = RegionHandle::new(ArenaKind::Weights, u64::MAX, 0, g).unwrap();
    assert_eq!(h.end(), u64::MAX);
}

#[test]
fn new_in_checks() {
    let a = arena(ArenaKind::Workspace, 100);
    let g = Generation(3);
    let h = RegionHandle::new_in(&a, 40, 60, g).unwrap();
    assert_eq!(h.arena(), ArenaKind::Workspace);
    assert_eq!((h.offset(), h.len(), h.end()), (40, 60, 100));
    assert_eq!(h.generation(), g);
    assert_eq!(
        RegionHandle::new_in(&a, 41, 60, g),
        Err(RegionError::OutOfBounds { offset: 41, len: 60, capacity: 100 })
    );
    assert_eq!(RegionHandle::new_in(&a, u64::MAX, 2, g), Err(RegionError::Overflow));
    let z = arena(ArenaKind::Staging, 0);
    assert!(RegionHandle::new_in(&z, 0, 0, g).is_ok());
}

#[test]
fn check_in_order() {
    let a = arena(ArenaKind::Weights, 100);
    let h = RegionHandle::new(ArenaKind::Weights, 90, 20, Generation(1)).unwrap();
    assert_eq!(
        h.check_in(&a),
        Err(RegionError::OutOfBounds { offset: 90, len: 20, capacity: 100 })
    );
    let other = arena(ArenaKind::Staging, 10);
    assert_eq!(
        h.check_in(&other),
        Err(RegionError::ArenaMismatch {
            handle: ArenaKind::Weights,
            arena: ArenaKind::Staging
        })
    );
    let ok = RegionHandle::new(ArenaKind::Weights, 0, 100, Generation(9)).unwrap();
    assert_eq!(ok.check_in(&a), Ok(()));
}

#[test]
fn generation_semantics() {
    assert_eq!(Generation::default(), Generation::INITIAL);
    assert_eq!(Generation(1).next(), Some(Generation(2)));
    assert_eq!(Generation(u64::MAX).next(), None);
    let h = RegionHandle::new(ArenaKind::Weights, 1, 2, Generation(5)).unwrap();
    assert!(h.is_current(Generation(5)));
    assert!(!h.is_current(Generation(4)));
    assert!(!h.is_current(Generation(6)));
    let h2 = h.with_generation(Generation(6));
    assert_eq!(h.generation(), Generation(5));
    assert_eq!((h2.arena(), h2.offset(), h2.len()), (h.arena(), h.offset(), h.len()));
    assert_ne!(h, h2);
}

#[test]
fn key_and_ordering() {
    let g = Generation(0);
    let a = RegionHandle::new(ArenaKind::Weights, 5, 5, g).unwrap();
    let b = a.with_generation(Generation(1));
    let mut m = HashMap::new();
    m.insert(a, 1);
    m.insert(b, 2);
    assert_eq!(m.len(), 2);
    assert_eq!(m[&a], 1);
    let c = RegionHandle::new(ArenaKind::IndexSegments, 0, 0, g).unwrap();
    assert!(a < c);
    assert!(a < b);
    let d = RegionHandle::new(ArenaKind::Weights, 5, 6, g).unwrap();
    assert!(b < d);
}

#[test]
fn error_display() {
    assert!(!RegionError::Overflow.to_string().is_empty());
    let e: Box<dyn std::error::Error> = Box::new(RegionError::Overflow);
    assert!(!e.to_string().is_empty());
}
