use std::sync::Arc;
use std::thread;
use tokio::sync::Notify;
use super::*;
type Q = Queue<(usize, usize), u32, (), ()>;
fn collect(q: &Q) -> Vec<(usize, usize)> {
    let mut out = Vec::new();
    let mut cur = q.first();
    while let Some(n) = cur {
        out.push(*n.meta().unwrap());
        cur = n.next();
    }
    out
}
#[test]
fn empty_queue_has_sentinel_only() {
    let q = Q::new(Vec::new());
    assert!(q.head().is_sentinel());
    assert!(q.head().meta().is_none());
    assert!(q.head().state().is_none());
    assert!(q.first().is_none());
}
#[test]
fn enqueue_preserves_order_and_identity() {
    let q = Q::new(Vec::new());
    let a = q.enqueue((0, 0), EntryState::new(1));
    let b = q.enqueue((0, 1), EntryState::new(2));
    assert!(Arc::ptr_eq(& q.first().unwrap(), & a));
    assert!(Arc::ptr_eq(& a.next().unwrap(), & b));
    assert!(b.next().is_none());
    assert!(! a.is_sentinel());
}
#[test]
fn stale_tail_hint_still_appends_at_end() {
    let q = Q::new(Vec::new());
    q.set_tail_hint_update(false);
    for i in 0..50 {
        q.enqueue((0, i), EntryState::new(0));
    }
    assert_eq!(collect(& q), (0..50).map(| i | (0, i)).collect::< Vec < _ >> ());
}
#[test]
fn concurrent_producers_lose_nothing() {
    for hint in [true, false] {
        let q = Arc::new(Q::new(Vec::new()));
        q.set_tail_hint_update(hint);
        let handles: Vec<_> = (0..8)
            .map(|p| {
                let q = Arc::clone(&q);
                thread::spawn(move || {
                    for k in 0..200 {
                        q.enqueue((p, k), EntryState::new(0));
                    }
                })
            })
            .collect();
        for h in handles {
            h.join().unwrap();
        }
        let all = collect(&q);
        assert_eq!(all.len(), 1600);
        for p in 0..8 {
            let ks: Vec<_> = all.iter().filter(|e| e.0 == p).map(|e| e.1).collect();
            assert_eq!(ks, (0..200).collect::< Vec < _ >> ());
        }
    }
}
#[tokio::test]
async fn every_channel_is_signaled() {
    let c1 = Arc::new(Notify::new());
    let c2 = Arc::new(Notify::new());
    let q = Q::new(vec![Arc::clone(& c1), Arc::clone(& c2)]);
    q.enqueue((0, 0), EntryState::new(0));
    c1.notified().await;
    c2.notified().await;
}
#[test]
fn long_list_drops_without_stack_overflow() {
    thread::Builder::new()
        .stack_size(2 * 1024 * 1024)
        .spawn(|| {
            let q = Q::new(Vec::new());
            for i in 0..100_000 {
                q.enqueue((0, i), EntryState::new(0));
            }
            drop(q);
        })
        .unwrap()
        .join()
        .unwrap();
}
#[test]
fn held_suffix_survives_queue_drop() {
    let q = Q::new(Vec::new());
    q.enqueue((0, 0), EntryState::new(0));
    let held = q.enqueue((0, 1), EntryState::new(0));
    let last = q.enqueue((0, 2), EntryState::new(0));
    drop(q);
    assert_eq!(held.meta(), Some(& (0, 1)));
    assert!(Arc::ptr_eq(& held.next().unwrap(), & last));
}
