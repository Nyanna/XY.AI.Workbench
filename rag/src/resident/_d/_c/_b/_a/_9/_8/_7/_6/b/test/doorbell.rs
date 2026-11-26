use super::*;
use std::thread;
use std::time::Duration;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::_1::device_lane_event::DeviceHandle;
#[test]
fn types_are_send_sync() {
    fn assert_send_sync<T: Send + Sync + 'static>() {}
    assert_send_sync::<DoorbellCell>();
    assert_send_sync::<Doorbell>();
    assert_send_sync::<DoorbellError>();
}
fn setup() -> (Doorbell, DoorbellCell, DoorbellCell) {
    let c = DoorbellCell::new_local();
    let a = DoorbellCell::new_local();
    let (c2, a2) = (c.share(), a.share());
    (Doorbell::new(c, a), c2, a2)
}
#[test]
fn poll_without_gap() {
    let (mut db, _c, _a) = setup();
    assert!(db.poll().unwrap().is_none());
    assert_eq!(db.acked(), 0);
}
#[test]
fn single_gap() {
    let (mut db, c, _a) = setup();
    c.test_store(1, Ordering::Release);
    let gap = db.poll().unwrap().unwrap();
    assert_eq!((gap.first(), gap.last(), gap.len()), (1, 1, 1));
    gap.close();
    assert_eq!(db.acked(), 1);
}
#[test]
fn coalesced_gap() {
    let (mut db, c, _a) = setup();
    c.test_store(5, Ordering::Release);
    let gap = db.poll().unwrap().unwrap();
    assert_eq!((gap.first(), gap.last(), gap.len()), (1, 5, 5));
    gap.close();
    assert_eq!(db.acked(), 5);
}
#[test]
fn close_then_new_poll() {
    let (mut db, c, a) = setup();
    c.test_store(2, Ordering::Release);
    db.poll().unwrap().unwrap().close();
    assert!(db.poll().unwrap().is_none());
    c.test_store(4, Ordering::Release);
    let gap = db.poll().unwrap().unwrap();
    assert_eq!((gap.first(), gap.last(), gap.len()), (3, 4, 2));
    gap.close();
    assert_eq!(a.test_load(Ordering::Acquire), 4);
}
#[test]
fn later_completion_not_acked_early() {
    let (mut db, c, a) = setup();
    c.test_store(2, Ordering::Release);
    let gap = db.poll().unwrap().unwrap();
    c.test_store(3, Ordering::Release);
    gap.close();
    assert_eq!(a.test_load(Ordering::Acquire), 2);
    let gap = db.poll().unwrap().unwrap();
    assert_eq!(gap.last(), 3);
    gap.close();
}
#[test]
fn inconsistent() {
    let (mut db, c, a) = setup();
    c.test_store(3, Ordering::Release);
    a.test_store(5, Ordering::Release);
    assert_eq!(
        db.poll().err(), Some(DoorbellError::Inconsistent { completed : 3, acked : 5 })
    );
    assert_eq!(c.test_load(Ordering::Acquire), 3);
    assert_eq!(a.test_load(Ordering::Acquire), 5);
}
#[test]
fn wait_with_pending_mock_event() {
    let (mut db, c, _a) = setup();
    let (event, ctl) = Event::mock();
    let t = thread::spawn(move || {
        thread::sleep(Duration::from_millis(30));
        c.test_store(1, Ordering::Release);
        ctl.complete();
    });
    let gap = db.wait(&event).unwrap().unwrap();
    assert_eq!(gap.last(), 1);
    gap.close();
    t.join().unwrap();
}
#[test]
fn wait_with_completed_event_and_no_gap() {
    let (mut db, _c, _a) = setup();
    let (event, ctl) = Event::mock();
    ctl.complete();
    assert!(db.wait(& event).unwrap().is_none());
}
#[test]
fn wait_failure_does_not_touch_cells() {
    let (mut db, c, a) = setup();
    c.test_store(2, Ordering::Release);
    let (event, ctl) = Event::mock();
    ctl.fail("x");
    let err = db.wait(&event).err().unwrap();
    assert_eq!(
        err, DoorbellError::Event(EventError { device : DeviceHandle::Cpu, cause : "x"
        .to_string() })
    );
    assert_eq!(c.test_load(Ordering::Acquire), 2);
    assert_eq!(a.test_load(Ordering::Acquire), 0);
}
#[test]
#[should_panic(expected = "without close")]
fn dropping_open_gap_panics() {
    let (mut db, c, _a) = setup();
    c.test_store(1, Ordering::Release);
    let _gap = db.poll().unwrap().unwrap();
}
#[test]
fn open_gap_during_unwind_does_not_ack_or_double_panic() {
    let (mut db, c, a) = setup();
    c.test_store(1, Ordering::Release);
    let r = std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| {
            let _gap = db.poll().unwrap().unwrap();
            panic!("boom");
        }),
    );
    assert!(r.is_err());
    assert_eq!(a.test_load(Ordering::Acquire), 0);
}
#[test]
fn release_acquire_message_passing() {
    let (mut db, c, _a) = setup();
    let data = Arc::new(AtomicU64::new(0));
    let d = Arc::clone(&data);
    let t = thread::spawn(move || {
        for i in 1..=2000u64 {
            d.store(i, Ordering::Relaxed);
            c.test_store(i, Ordering::Release);
        }
    });
    let mut seen = 0;
    while seen < 2000 {
        if let Some(gap) = db.poll().unwrap() {
            assert!(data.load(Ordering::Relaxed) >= gap.last());
            seen = gap.last();
            gap.close();
        }
    }
    t.join().unwrap();
}
#[test]
fn host_writes_visible_after_ack() {
    let (mut db, c, a) = setup();
    let result = Arc::new(AtomicU64::new(0));
    let r = Arc::clone(&result);
    let device = thread::spawn(move || {
        while a.test_load(Ordering::Acquire) < 1 {
            std::hint::spin_loop();
        }
        assert_eq!(r.load(Ordering::Relaxed), 42);
    });
    c.test_store(1, Ordering::Release);
    let gap = loop {
        if let Some(g) = db.poll().unwrap() {
            break g;
        }
    };
    result.store(42, Ordering::Relaxed);
    gap.close();
    device.join().unwrap();
}
