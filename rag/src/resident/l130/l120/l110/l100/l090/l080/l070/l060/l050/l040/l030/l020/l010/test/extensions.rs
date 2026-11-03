use super::*;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

#[test]
fn new_is_empty() {
    let e = Extensions::new();
    assert_eq!(e.len(), 0);
    assert!(e.is_empty());
    assert!(Extensions::default().is_empty());
}

#[test]
fn insert_get_replace() {
    let mut e = Extensions::new();
    e.insert(5u32);
    assert_eq!(e.get::<u32>(), Some(&5));
    e.insert(6u32);
    assert_eq!(e.get::<u32>(), Some(&6));
    assert_eq!(e.len(), 1);
    e.insert(7u64);
    e.insert(String::from("a"));
    e.insert("b");
    assert_eq!(e.len(), 4);
    assert_eq!(e.get::<u64>(), Some(&7));
    assert_eq!(e.get::<&'static str>(), Some(&"b"));
    assert!(e.get::<i8>().is_none());
}

#[test]
fn remove_works() {
    let mut e = Extensions::new();
    e.insert(1u8);
    assert!(!e.remove::<u16>());
    assert_eq!(e.len(), 1);
    assert!(e.remove::<u8>());
    assert!(e.get::<u8>().is_none());
    assert!(e.is_empty());
}

struct NotClone(u32);

#[test]
fn clone_shares_values_independent_maps() {
    let mut a = Extensions::new();
    a.insert(NotClone(3));
    let mut b = a.clone();
    let pa = a.get::<NotClone>().unwrap() as *const NotClone;
    let pb = b.get::<NotClone>().unwrap() as *const NotClone;
    assert_eq!(pa, pb);
    b.insert(1u8);
    assert!(a.get::<u8>().is_none());
    assert!(b.remove::<NotClone>());
    assert_eq!(a.get::<NotClone>().unwrap().0, 3);
}

struct DropCount(Arc<AtomicUsize>);

impl Drop for DropCount {
    fn drop(&mut self) {
        self.0.fetch_add(1, Ordering::SeqCst);
    }
}

#[test]
fn dropped_with_last_holder() {
    let n = Arc::new(AtomicUsize::new(0));
    let mut a = Extensions::new();
    a.insert(DropCount(n.clone()));
    let b = a.clone();
    drop(a);
    assert_eq!(n.load(Ordering::SeqCst), 0);
    drop(b);
    assert_eq!(n.load(Ordering::SeqCst), 1);
}

#[test]
fn threads_and_debug() {
    let mut e = Extensions::new();
    e.insert(NotClone(9));
    let c = e.clone();
    let h = std::thread::spawn(move || c.get::<NotClone>().unwrap().0);
    assert_eq!(h.join().unwrap(), 9);
    assert!(format!("{:?}", e).contains("len"));
}
