use super::*;
use crate::resident::memory::region::ArenaId;

fn region() -> Region {
    Region { arena: ArenaId(0), offset: 0, len: 16, gen: 1 }
}

#[test]
fn drop_decrements_pin() {
    let pin = PinCount::new();
    let a = Lease::new(region(), pin.clone(), None);
    let b = Lease::new(region(), pin.clone(), None);
    assert_eq!(pin.get(), 2);
    drop(a);
    assert!(!pin.is_unpinned());
    drop(b);
    assert!(pin.is_unpinned());
}

#[test]
fn panic_does_not_leak_pin() {
    let pin = PinCount::new();
    let p = pin.clone();
    let r = std::panic::catch_unwind(move || {
        let _l = Lease::new(region(), p, None);
        panic!("boom");
    });
    assert!(r.is_err());
    assert!(pin.is_unpinned());
}

#[test]
fn drop_sends_region_without_blocking() {
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
    let lease = Lease::new(region(), PinCount::new(), Some(tx));
    drop(lease);
    assert_eq!(rx.try_recv().unwrap(), region());
}
