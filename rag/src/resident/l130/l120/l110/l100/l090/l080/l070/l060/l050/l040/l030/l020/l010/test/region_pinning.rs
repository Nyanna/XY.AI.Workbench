use super::*;
use std::sync::mpsc::channel;
use std::thread;

#[test]
fn new_count_is_zero() {
    let c = PinCount::new();
    assert_eq!(c.count(), 0);
    assert!(c.is_unpinned());
}

#[test]
fn pin_increments_and_drop_decrements() {
    let c = PinCount::new();
    let clone = c.clone();
    let p = Pin::new(7u32, &c);
    assert_eq!(*p.region(), 7);
    assert_eq!(clone.count(), 1);
    assert!(!clone.is_unpinned());
    let q: DeviceHandle<u32> = Pin::new(8, &clone);
    assert_eq!(c.count(), 2);
    drop(p);
    drop(q);
    assert!(c.is_unpinned());
}

#[test]
fn release_channel_sees_reduced_count() {
    let c = PinCount::new();
    let (tx, rx) = channel();
    let p = Pin::with_release_channel(5u32, &c, tx);
    drop(p);
    assert_eq!(rx.recv().unwrap(), 5);
    assert_eq!(c.count(), 0);
    assert!(rx.try_recv().is_err());
}

#[test]
fn dropped_receiver_is_ignored() {
    let c = PinCount::new();
    let (tx, rx) = channel();
    drop(rx);
    let p = Pin::with_release_channel(1u8, &c, tx);
    drop(p);
    assert_eq!(c.count(), 0);
}

#[test]
fn independent_counters() {
    let a = PinCount::new();
    let b = PinCount::new();
    let _p = Pin::new((), &a);
    assert_eq!(a.count(), 1);
    assert_eq!(b.count(), 0);
}

#[test]
fn forget_keeps_count_raised() {
    let c = PinCount::new();
    std::mem::forget(Pin::new(1, &c));
    assert_eq!(c.count(), 1);
}

#[test]
fn concurrent_pins_do_not_lose_updates() {
    let c = PinCount::new();
    let handles: Vec<_> = (0..8)
        .map(|i| {
            let c = c.clone();
            thread::spawn(move || {
                for j in 0..1000 {
                    let _p = Pin::new(i * j, &c);
                }
            })
        })
        .collect();
    for h in handles {
        h.join().unwrap();
    }
    assert!(c.is_unpinned());
}
