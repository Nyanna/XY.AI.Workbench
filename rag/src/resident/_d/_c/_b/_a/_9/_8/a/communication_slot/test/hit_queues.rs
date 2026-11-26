use super::*;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::a::kernel_layout::Addr;
use static_assertions::{assert_impl_all, assert_not_impl_any};
assert_impl_all!(WriteProducer : Send);
assert_impl_all!(HitProducer : Send);
assert_impl_all!(QueueConsumer : Send);
assert_not_impl_any!(WriteProducer : Clone);
assert_not_impl_any!(HitProducer : Clone);
assert_not_impl_any!(QueueConsumer : Clone);
fn write(n: u64) -> EmbeddingWrite {
    EmbeddingWrite {
        entry: EntryId(n),
        segment: SegmentId(n),
        slot: Slot5(n as u32),
    }
}
fn hit(n: u64) -> KernelHit {
    KernelHit {
        addr: Addr(n),
        score: n as f32,
    }
}
#[test]
fn zero_capacity_becomes_one() {
    let (mut w, h, _c) = channel(0, 0);
    assert_eq!(w.capacity(), 1);
    assert_eq!(h.capacity(), 1);
    assert!(w.push(write(1)).is_ok());
    assert!(w.push(write(2)).is_err());
}
#[test]
fn full_returns_item_and_keeps_queue() {
    let (mut w, _h, mut c) = channel(2, 2);
    w.push(write(1)).unwrap();
    w.push(write(2)).unwrap();
    let err = w.push(write(3)).unwrap_err();
    assert_eq!(err, PushError { item : write(3), error : QueueError::Full });
    assert_eq!(err.to_string(), "queue full");
    assert_eq!(w.depth(), 2);
    assert_eq!(c.drain().writes, vec![write(1), write(2)]);
}
#[test]
fn fifo_with_wraparound() {
    let (mut w, mut h, mut c) = channel(3, 3);
    let mut next = 0;
    for round in 0..10 {
        for _ in 0..3 {
            w.push(write(next)).unwrap();
            h.push(hit(next)).unwrap();
            next += 1;
        }
        let d = c.drain();
        let base = round * 3;
        assert_eq!(d.writes, (base..base + 3).map(write).collect::< Vec < _ >> ());
        assert_eq!(d.hits, (base..base + 3).map(hit).collect::< Vec < _ >> ());
    }
}
#[test]
fn depth_tracking() {
    let (mut w, mut h, mut c) = channel(4, 4);
    w.push(write(1)).unwrap();
    h.push(hit(1)).unwrap();
    assert_eq!((w.depth(), h.depth()), (1, 1));
    assert_eq!((c.write_depth(), c.hit_depth()), (1, 1));
    c.drain();
    assert_eq!((c.write_depth(), c.hit_depth()), (0, 0));
}
#[test]
fn drain_empty_is_default() {
    let (_w, _h, mut c) = channel(2, 2);
    assert_eq!(c.drain(), Drained::default());
}
#[test]
fn endpoints_survive_peer_drop() {
    let (mut w, h, mut c) = channel(2, 2);
    w.push(write(7)).unwrap();
    drop(w);
    drop(h);
    assert_eq!(c.drain().writes, vec![write(7)]);
    let (mut w, mut h, c) = channel(2, 2);
    drop(c);
    assert!(w.push(write(1)).is_ok());
    assert!(h.push(hit(1)).is_ok());
}
#[test]
fn threaded_write_before_hit_order() {
    let (mut w, mut h, mut c) = channel(8, 8);
    const N: u64 = 20_000;
    let wt = std::thread::spawn(move || {
        for i in 0..N {
            while w.push(write(i)).is_err() {
                std::thread::yield_now();
            }
            while h.push(hit(i)).is_err() {
                std::thread::yield_now();
            }
        }
    });
    let mut seen_writes = 0u64;
    let mut seen_hits = 0u64;
    while seen_hits < N {
        let d = c.drain();
        for (i, x) in d.writes.iter().enumerate() {
            assert_eq!(* x, write(seen_writes + i as u64));
        }
        seen_writes += d.writes.len() as u64;
        for (i, x) in d.hits.iter().enumerate() {
            assert_eq!(* x, hit(seen_hits + i as u64));
        }
        seen_hits += d.hits.len() as u64;
        assert!(seen_writes >= seen_hits);
    }
    wt.join().unwrap();
}
