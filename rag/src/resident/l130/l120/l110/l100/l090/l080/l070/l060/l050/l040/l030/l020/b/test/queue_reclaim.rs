use super::*;
use std::sync::atomic::{AtomicUsize, Ordering};
type N = Node<u32, (), ()>;
type L = Link<u32, (), ()>;
fn node(v: u32) -> Arc<N> {
    Node::new(EntryState::new(v))
}
fn claimed(v: u32) -> Arc<N> {
    let n = node(v);
    n.entry().claim().unwrap();
    n
}
#[test]
fn link_basics() {
    let l: L = Link::default();
    assert!(l.load().is_none());
    let a = node(1);
    assert!(l.compare_and_swap(None, Some(a.clone())).is_ok());
    assert!(Arc::ptr_eq(& l.load().unwrap(), & a));
    let b = node(2);
    let err = l.compare_and_swap(None, Some(b.clone())).unwrap_err();
    assert!(Arc::ptr_eq(& err.unwrap(), & a));
    assert!(l.compare_and_swap(Some(& a), Some(b.clone())).is_ok());
    let prev = l.swap(None).unwrap();
    assert!(Arc::ptr_eq(& prev, & b));
    assert!(l.load().is_none());
}
#[test]
fn unlink_rules() {
    let head: L = Link::empty();
    let a = claimed(1);
    let b = node(2);
    let c = node(3);
    a.next().swap(Some(b.clone()));
    b.next().swap(Some(c.clone()));
    head.swap(Some(a.clone()));
    assert!(! unlink(& head, & b));
    assert!(! unlink(& head, & c));
    assert!(unlink(& head, & a));
    assert!(! unlink(& head, & a));
    assert!(Arc::ptr_eq(& head.load().unwrap(), & b));
    assert!(Arc::ptr_eq(& a.next().load().unwrap(), & b));
}
#[test]
fn long_chain_drops_iteratively() {
    let head: L = Link::empty();
    let first = node(0);
    let mut tail = first.clone();
    head.swap(Some(first));
    for i in 1..100_000u32 {
        let n = node(i);
        tail.next().swap(Some(n.clone()));
        tail = n;
    }
    drop(tail);
    drop(head);
}
#[test]
fn shared_suffix_survives_prefix_drop() {
    let head: L = Link::empty();
    let a = node(1);
    let b = node(2);
    let c = node(3);
    a.next().swap(Some(b.clone()));
    b.next().swap(Some(c.clone()));
    head.swap(Some(a));
    drop(c);
    drop(head);
    assert!(b.next().load().is_some());
    assert_eq!(b.next().load().unwrap().entry().state(), QueueState::Enqueued);
}
#[test]
fn concurrent_drops_free_each_once() {
    static DROPS: AtomicUsize = AtomicUsize::new(0);
    struct D;
    impl Drop for D {
        fn drop(&mut self) {
            DROPS.fetch_add(1, Ordering::SeqCst);
        }
    }
    let total = 10_000;
    let first = Node::<D, (), ()>::new(EntryState::new(D));
    let mut tail = first.clone();
    for _ in 1..total {
        let n = Node::<D, (), ()>::new(EntryState::new(D));
        tail.next().swap(Some(n.clone()));
        tail = n;
    }
    drop(tail);
    let mut handles = Vec::new();
    for _ in 0..4 {
        let mut cur = Some(first.clone());
        handles
            .push(
                std::thread::spawn(move || {
                    while let Some(n) = cur {
                        cur = n.next().load();
                    }
                }),
            );
    }
    drop(first);
    for h in handles {
        h.join().unwrap();
    }
    assert_eq!(DROPS.load(Ordering::SeqCst), total);
}
