use super::*;
use crate::resident::memory::arena::testing::Gate;

#[test]
fn slots_cycle_round_robin() {
    let ring = StagingRing::new([0u8, 1, 2]);
    let a = ring.acquire().unwrap();
    let b = ring.acquire().unwrap();
    assert_ne!(a, b);
    ring.release(a).unwrap();
    ring.release(b).unwrap();
    assert_eq!(ring.slots(), 3);
}

#[test]
fn slot_is_not_reused_before_its_event_completes() {
    let ring = StagingRing::new([()]);
    let gate = Gate::closed();
    let i = ring.acquire().unwrap();
    ring.publish(i, gate.event.clone()).unwrap();
    assert_eq!(ring.in_flight(), 1);
    assert!(ring.acquire().is_none());
    gate.open();
    assert_eq!(ring.acquire(), Some(i));
}

#[test]
fn acquire_wait_blocks_on_oldest_transfer() {
    let ring = StagingRing::new([()]);
    let gate = Gate::closed();
    let i = ring.acquire().unwrap();
    ring.publish(i, gate.event.clone()).unwrap();
    let waiter = std::thread::scope(|s| {
        let h = s.spawn(|| ring.acquire_wait());
        std::thread::sleep(std::time::Duration::from_millis(30));
        assert!(!h.is_finished());
        gate.open();
        h.join().unwrap()
    });
    assert_eq!(waiter.unwrap(), i);
}

#[test]
fn acquire_wait_fails_when_all_slots_are_held() {
    let ring = StagingRing::new([()]);
    ring.acquire().unwrap();
    assert!(matches!(ring.acquire_wait(), Err(MemoryError::ArenaExhausted { .. })));
}

#[test]
fn payload_access_requires_a_held_slot() {
    let ring = StagingRing::new([5u32]);
    assert!(matches!(ring.with_slot(0, |_| ()), Err(MemoryError::InvalidSlot { .. })));
    let i = ring.acquire().unwrap();
    ring.with_slot(i, |p| *p += 1).unwrap();
    assert_eq!(ring.with_slot(i, |p| *p).unwrap(), 6);
    assert!(ring.publish(9, Gate::closed().event).is_err());
}
