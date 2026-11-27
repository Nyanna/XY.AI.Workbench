use super::*;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
#[test]
fn staging_ring_is_send_sync() {
    fn check<T: Send + Sync>() {}
    check::<StagingRing>();
}
#[test]
fn invalid_config() {
    assert_eq!(StagingRing::new(0, 8).err(), Some(StagingError::InvalidConfig));
    let a = Arena {
        kind: ArenaKind::Staging,
        capacity: 100,
    };
    assert_eq!(StagingRing::from_arena(& a, 0).err(), Some(StagingError::InvalidConfig));
    let r = StagingRing::from_arena(&a, 3).unwrap();
    assert_eq!(r.slot_size(), 33);
    assert_eq!(r.slot_count(), 3);
    assert!(StagingRing::new(2, 0).is_ok());
}
#[test]
fn round_robin_and_exhausted() {
    let r = StagingRing::new(3, 4).unwrap();
    let ids: Vec<_> = (0..3).map(|_| r.acquire().unwrap()).collect();
    assert_eq!(ids, vec![SlotId(0), SlotId(1), SlotId(2)]);
    assert_eq!(r.acquire(), Err(StagingError::Exhausted));
    r.release(SlotId(1)).unwrap();
    assert_eq!(r.acquire(), Err(StagingError::Exhausted));
    r.release(SlotId(0)).unwrap();
    assert_eq!(r.acquire().unwrap(), SlotId(0));
}
#[test]
fn payload_and_invalid_slot() {
    let r = StagingRing::new(1, 4).unwrap();
    assert_eq!(r.with_payload(SlotId(0), | _ | ()), Err(StagingError::InvalidSlot));
    let s = r.acquire().unwrap();
    r.with_payload(s, |b| b.copy_from_slice(&[1, 2, 3, 4])).unwrap();
    assert!(! r.payload_ptr(s).unwrap().is_null());
    assert_eq!(r.with_payload(SlotId(9), | _ | ()), Err(StagingError::InvalidSlot));
    assert_eq!(r.slot_state(SlotId(9)), None);
    r.release(s).unwrap();
    assert_eq!(r.release(s), Err(StagingError::InvalidSlot));
    let s = r.acquire().unwrap();
    assert_eq!(r.with_payload(s, | b | b.to_vec()).unwrap(), vec![1, 2, 3, 4]);
}
#[test]
fn transfer_reclaim_and_order() {
    let r = StagingRing::new(3, 1).unwrap();
    let a = r.acquire().unwrap();
    let b = r.acquire().unwrap();
    let (ea, ca) = Event::mock();
    let (eb, cb) = Event::mock();
    r.begin_transfer(b, eb).unwrap();
    r.begin_transfer(a, ea).unwrap();
    assert_eq!(r.in_flight_slots(), vec![b, a]);
    assert_eq!(r.in_flight_count(), 2);
    assert_eq!(r.begin_transfer(a, Event::mock().0), Err(StagingError::InvalidSlot));
    cb.complete();
    assert_eq!(r.slot_state(b), Some(SlotState::Free));
    assert_eq!(r.in_flight_slots(), vec![a]);
    ca.complete();
    assert_eq!(r.in_flight_count(), 0);
}
#[test]
fn acquire_blocks_until_complete() {
    let r = Arc::new(StagingRing::new(1, 1).unwrap());
    let s = r.acquire().unwrap();
    let (e, c) = Event::mock();
    r.begin_transfer(s, e).unwrap();
    let r2 = r.clone();
    let h = thread::spawn(move || r2.acquire());
    thread::sleep(Duration::from_millis(50));
    assert_eq!(r.in_flight_count(), 1);
    c.complete();
    assert_eq!(h.join().unwrap(), Ok(SlotId(0)));
}
#[test]
fn acquire_still_reports_exhausted_when_all_occupied() {
    let r = StagingRing::new(2, 1).unwrap();
    r.acquire().unwrap();
    r.acquire().unwrap();
    assert_eq!(r.acquire(), Err(StagingError::Exhausted));
}
#[test]
fn acquire_wait_blocks_until_release() {
    let r = Arc::new(StagingRing::new(1, 1).unwrap());
    let s = r.acquire().unwrap();
    let done = Arc::new(std::sync::atomic::AtomicBool::new(false));
    let (r2, done2) = (r.clone(), done.clone());
    let h = thread::spawn(move || {
        let got = r2.acquire_wait();
        done2.store(true, std::sync::atomic::Ordering::SeqCst);
        got
    });
    thread::sleep(Duration::from_millis(50));
    assert!(! done.load(std::sync::atomic::Ordering::SeqCst));
    r.release(s).unwrap();
    assert_eq!(h.join().unwrap(), Ok(SlotId(0)));
}
#[test]
fn acquire_wait_follows_a_slot_that_starts_a_transfer() {
    let r = Arc::new(StagingRing::new(2, 1).unwrap());
    let s0 = r.acquire().unwrap();
    let _s1 = r.acquire().unwrap();
    let r2 = r.clone();
    let h = thread::spawn(move || r2.acquire_wait());
    thread::sleep(Duration::from_millis(50));
    let (e, c) = Event::mock();
    r.begin_transfer(s0, e).unwrap();
    thread::sleep(Duration::from_millis(50));
    c.complete();
    assert_eq!(h.join().unwrap(), Ok(s0));
}
#[test]
fn acquire_wait_reports_failed_event() {
    let r = StagingRing::new(1, 1).unwrap();
    let s = r.acquire().unwrap();
    let (e, c) = Event::mock();
    r.begin_transfer(s, e).unwrap();
    c.fail("boom");
    match r.acquire_wait() {
        Err(StagingError::Transfer(t)) => assert_eq!(t.direction, TransferDirection::H2D),
        other => panic!("unexpected {:?}", other),
    }
    assert_eq!(r.slot_state(s), Some(SlotState::InFlight));
}
#[test]
fn acquire_wait_serves_more_threads_than_slots() {
    let r = Arc::new(StagingRing::new(2, 1).unwrap());
    let handles: Vec<_> = (0..16)
        .map(|_| {
            let r = r.clone();
            thread::spawn(move || {
                let s = r.acquire_wait().unwrap();
                thread::sleep(Duration::from_millis(5));
                r.release(s).unwrap();
            })
        })
        .collect();
    for h in handles {
        h.join().unwrap();
    }
    assert_eq!(r.in_flight_count(), 0);
    assert!(r.acquire().is_ok());
}
#[test]
fn failed_event_reports_transfer_error() {
    let r = StagingRing::new(1, 1).unwrap();
    let s = r.acquire().unwrap();
    let (e, c) = Event::mock();
    r.begin_transfer(s, e).unwrap();
    c.fail("boom");
    match r.acquire() {
        Err(StagingError::Transfer(t)) => assert_eq!(t.direction, TransferDirection::H2D),
        other => panic!("unexpected {:?}", other),
    }
    assert_eq!(r.slot_state(s), Some(SlotState::InFlight));
}
#[test]
fn concurrent_acquirers_get_distinct_slots() {
    let r = Arc::new(StagingRing::new(8, 1).unwrap());
    let hs: Vec<_> = (0..8)
        .map(|_| {
            let r = r.clone();
            thread::spawn(move || r.acquire().unwrap())
        })
        .collect();
    let mut ids: Vec<_> = hs.into_iter().map(|h| h.join().unwrap()).collect();
    ids.sort();
    ids.dedup();
    assert_eq!(ids.len(), 8);
}
