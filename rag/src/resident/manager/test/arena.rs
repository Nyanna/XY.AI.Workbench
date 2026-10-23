use super::*;
use crate::resident::memory::arena::testing::Gate;
use crate::resident::memory::{ArenaKind, DeviceKind};
fn host_arena(cap: u64) -> ArenaAllocator {
    ArenaAllocator::new(ArenaKind::IndexSegment, DeviceKind::Cpu, cap)
}
#[test]
fn offsets_are_aligned_disjoint_and_reused() {
    let alloc = host_arena(1 << 20);
    let arena = IndexArena::new(&alloc, 0);
    let regions: Vec<Region> = [100, 1000, 4096, 7]
        .iter()
        .map(|b| arena.alloc_segment(*b, 1).unwrap())
        .collect();
    for r in &regions {
        assert_eq!(r.offset % SEGMENT_ALIGN, 0);
    }
    for (i, a) in regions.iter().enumerate() {
        for b in &regions[i + 1..] {
            assert!(a.offset + a.len <= b.offset || b.offset + b.len <= a.offset);
        }
    }
    let first = regions[0];
    arena.free(first).unwrap();
    assert_eq!(arena.alloc_segment(100, 1).unwrap().offset, first.offset);
}
#[test]
fn admission_keeps_cow_headroom() {
    let alloc = host_arena(4096);
    let arena = IndexArena::new(&alloc, 0);
    for _ in 0..3 {
        arena.alloc_segment(1024, 256).unwrap();
    }
    let err = arena.alloc_segment(1024, 256).unwrap_err();
    assert!(matches!(err, ManagerError::ArenaExhausted { requested : 1024, .. }));
    let cow = arena.cow_budget();
    assert_eq!((cow.live, cow.headroom(), cow.required()), (3072, 1024, 4096));
}
#[test]
fn reserve_is_never_handed_out() {
    let alloc = host_arena(4096);
    let arena = IndexArena::new(&alloc, 2048);
    assert!(
        matches!(arena.alloc_segment(1536, 256), Err(ManagerError::ArenaExhausted {
        .. }))
    );
    arena.alloc_segment(1024, 256).unwrap();
}
#[test]
fn old_copy_is_released_only_after_its_epoch_ends() {
    let alloc = host_arena(4096);
    let arena = IndexArena::new(&alloc, 0);
    let a = arena.alloc_segment(1024, 256).unwrap();
    let free_before = alloc.available();
    let ticket = arena.begin_cow(a).unwrap();
    assert_ne!(ticket.new_region().offset, a.offset);
    let b = ticket.commit(5).unwrap();
    assert_eq!(alloc.available(), free_before - 1024);
    assert_eq!(arena.reclaim(4), 0);
    assert_eq!(arena.budget().retired, 1024);
    assert_eq!(arena.reclaim(5), 1);
    assert_eq!(alloc.available(), free_before);
    assert!(arena.free(a).is_err());
    arena.free(b).unwrap();
}
#[test]
fn cow_is_serial_and_dropped_ticket_discards_the_copy() {
    let alloc = host_arena(8192);
    let arena = IndexArena::new(&alloc, 0);
    let a = arena.alloc_segment(1024, 256).unwrap();
    let free_before = alloc.available();
    let first = arena.begin_cow(a).unwrap();
    assert!(arena.try_begin_cow(a).unwrap().is_none());
    drop(first);
    assert_eq!(alloc.available(), free_before);
    assert!(arena.try_begin_cow(a).unwrap().is_some());
}
#[test]
fn retired_region_cannot_be_freed_early() {
    let alloc = host_arena(4096);
    let arena = IndexArena::new(&alloc, 0);
    let a = arena.alloc_segment(512, 256).unwrap();
    arena.retire(a, 1).unwrap();
    assert!(matches!(arena.free(a), Err(ManagerError::StaleRegion { .. })));
    assert!(matches!(arena.retire(a, 2), Err(ManagerError::StaleRegion { .. })));
}
#[test]
fn free_after_waits_for_the_event() {
    let alloc = host_arena(2048);
    let arena = IndexArena::new(&alloc, 0);
    let a = arena.alloc_segment(512, 256).unwrap();
    let gate = Gate::closed();
    arena.free_after(a, gate.event.clone()).unwrap();
    assert_eq!(alloc.pending(), 1);
    gate.open();
    arena.alloc_segment(512, 256).unwrap();
    assert_eq!(alloc.pending(), 0);
}
#[test]
fn writable_region_claims_aligned_ranges_until_full() {
    let alloc = host_arena(8192);
    let arena = IndexArena::new(&alloc, 0);
    let mut w = arena.reserve_writable(1024).unwrap();
    let base = w.region().offset;
    assert_eq!(w.claim(10, 1).unwrap(), base..base + 10);
    assert_eq!(w.claim(100, 64).unwrap(), base + 64..base + 164);
    assert_eq!(w.written(), 164);
    assert!(matches!(w.claim(2000, 1), Err(ManagerError::ArenaExhausted { .. })));
    assert_eq!(w.remaining(), 1024 - 164);
    let (region, written) = w.into_parts();
    assert_eq!((region.len, written), (1024, 164));
}
