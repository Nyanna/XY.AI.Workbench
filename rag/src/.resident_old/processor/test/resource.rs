use super::*;
use crate::resident::memory::{ArenaId, Region};
use crate::resident::op::Lease;
fn slot() -> (ResourceSlot, Arc<PinCount>) {
    let pins = PinCount::new();
    (ResourceSlot::new(pins.clone()), pins)
}
fn ready(s: &ResourceSlot) {
    assert!(s.begin_load(0));
    s.finish_load(true, 0, 0);
}
#[test]
fn single_loader_wins() {
    let (s, _) = slot();
    assert!(s.begin_load(0));
    assert!(! s.begin_load(0));
    assert_eq!(s.state(), ResourceState::Loading);
}
#[test]
fn failed_load_backs_off_until_deadline() {
    let (s, _) = slot();
    assert!(s.begin_load(0));
    s.finish_load(false, 10, 100);
    assert_eq!(s.state(), ResourceState::Backoff);
    assert!(! s.enter());
    assert!(! s.begin_load(50));
    assert!(s.begin_load(110));
}
#[test]
fn enter_requires_ready() {
    let (s, _) = slot();
    assert!(! s.enter());
    ready(&s);
    assert!(s.enter());
    assert_eq!(s.in_flight(), 1);
    s.leave(5);
    assert_eq!(s.in_flight(), 0);
}
#[test]
fn evict_waits_for_ttl_and_in_flight() {
    let (s, _) = slot();
    ready(&s);
    assert!(! s.try_evict(50, 100));
    assert!(s.enter());
    assert!(! s.try_evict(500, 100));
    assert_eq!(s.state(), ResourceState::Ready);
    s.leave(500);
    assert!(! s.try_evict(550, 100));
    assert!(s.try_evict(600, 100));
    assert!(! s.enter());
    s.finish_evict();
    assert_eq!(s.state(), ResourceState::Uninit);
}
#[test]
fn open_lease_blocks_eviction() {
    let (s, pins) = slot();
    ready(&s);
    let lease = Lease::new(
        Region {
            arena: ArenaId(0),
            offset: 0,
            len: 16,
            gen: 1,
        },
        pins,
        None,
    );
    assert!(! s.try_evict(1_000, 10));
    drop(lease);
    assert!(s.try_evict(1_000, 10));
}
