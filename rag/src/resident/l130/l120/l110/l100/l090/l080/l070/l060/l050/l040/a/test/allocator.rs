use super::*;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::a::arenas::{
    ArenaCapacities, ArenaSet, Budget,
};
use std::sync::atomic::{AtomicUsize, Ordering};

const CAP: u64 = 1024;

fn setup() -> (Arc<Bookkeeping>, Allocator, Allocator) {
    let mut caps = ArenaCapacities::default();
    caps.set(ArenaKind::Weights, CAP);
    caps.set(ArenaKind::Staging, 512);
    let set = Arc::new(ArenaSet::new(caps));
    let budget = Budget {
        free: u64::MAX / 2,
        total: u64::MAX / 2,
        generation_reserve: 0,
    };
    let weights = set.ensure(ArenaKind::Weights, &budget).unwrap();
    let staging = set.ensure(ArenaKind::Staging, &budget).unwrap();
    let bk = Arc::new(Bookkeeping::new(set));
    let a = Allocator::new(weights, DeviceKind::Gpu, Arc::clone(&bk));
    let b = Allocator::new(staging, DeviceKind::Cpu, Arc::clone(&bk));
    (bk, a, b)
}

#[test]
fn accessors() {
    let (_bk, a, b) = setup();
    assert_eq!(a.arena(), ArenaKind::Weights);
    assert_eq!(a.capacity(), CAP);
    assert_eq!(a.device_kind(), DeviceKind::Gpu);
    assert_eq!(b.arena(), ArenaKind::Staging);
    assert_eq!(b.capacity(), 512);
    assert_eq!(b.device_kind(), DeviceKind::Cpu);
    assert_eq!(a.available(), CAP);
    assert_eq!(a.pending_count(), 0);
}

#[test]
fn display_texts() {
    assert_eq!(
        AllocatorError::AllocationFailed.to_string(),
        "Allokation fehlgeschlagen"
    );
    assert_eq!(AllocatorError::StaleRegion.to_string(), "stale Region");
    let oom = AllocatorError::OutOfMemory {
        arena: ArenaKind::Weights,
        requested: 5,
        free: 3,
    };
    assert!(oom.to_string().contains("requested 5"));
}

#[test]
fn allocate_and_available() {
    let (_bk, a, _b) = setup();
    let r = a.allocate(100).unwrap();
    assert_eq!(r.arena, ArenaKind::Weights);
    assert_eq!(r.len, 100);
    assert_eq!(a.available(), CAP - 100);
    assert!(a.is_live(&r));
}

#[test]
fn allocate_errors() {
    let (_bk, a, _b) = setup();
    assert_eq!(a.allocate(0), Err(AllocatorError::AllocationFailed));
    let _r = a.allocate(1000).unwrap();
    assert_eq!(
        a.allocate(100),
        Err(AllocatorError::OutOfMemory {
            arena: ArenaKind::Weights,
            requested: 100,
            free: 24,
        })
    );
    assert_eq!(a.available(), 24);
}

#[test]
fn allocate_arena_not_created() {
    let (bk, _a, _b) = setup();
    let lone = Allocator::new(
        Arena {
            kind: ArenaKind::Workspace,
            capacity: 64,
        },
        DeviceKind::Gpu,
        bk,
    );
    assert_eq!(lone.allocate(8), Err(AllocatorError::AllocationFailed));
}

#[test]
fn injected_fault_maps_to_out_of_memory() {
    let (bk, a, _b) = setup();
    bk.inject_alloc_failure(ArenaKind::Weights, 1);
    assert!(matches!(
        a.allocate(10),
        Err(AllocatorError::OutOfMemory {
            arena: ArenaKind::Weights,
            requested: 10,
            free: CAP,
        })
    ));
    assert!(a.allocate(10).is_ok());
}

#[test]
fn immediate_free() {
    let (_bk, a, _b) = setup();
    let r = a.allocate(200).unwrap();
    assert_eq!(a.free(&r), Ok(()));
    assert_eq!(a.available(), CAP);
    assert!(!a.is_live(&r));
    assert_eq!(a.free(&r), Err(AllocatorError::StaleRegion));
    assert_eq!(a.available(), CAP);
}

#[test]
fn free_foreign_region_is_stale() {
    let (_bk, a, b) = setup();
    let r = b.allocate(64).unwrap();
    assert_eq!(a.free(&r), Err(AllocatorError::StaleRegion));
    let (ev, _c) = Event::mock();
    assert_eq!(a.free_deferred(&r, ev), Err(AllocatorError::StaleRegion));
    assert!(b.is_live(&r));
    assert_eq!(b.available(), 512 - 64);
}

#[test]
fn deferred_free_waits_for_event() {
    let (_bk, a, _b) = setup();
    let r = a.allocate(300).unwrap();
    let (ev, ctl) = Event::mock();
    a.free_deferred(&r, ev).unwrap();
    assert_eq!(a.pending_count(), 1);
    assert!(a.is_live(&r));
    assert_eq!(a.available(), CAP - 300);
    assert_eq!(a.reap(), 0);
    ctl.complete();
    assert_eq!(a.reap(), 1);
    assert_eq!(a.pending_count(), 0);
    assert!(!a.is_live(&r));
    assert_eq!(a.available(), CAP);
    assert_eq!(a.reap(), 0);
}

#[test]
fn deferred_free_with_completed_event_frees_immediately() {
    let (_bk, a, _b) = setup();
    let r = a.allocate(300).unwrap();
    let (ev, ctl) = Event::mock();
    ctl.complete();
    a.free_deferred(&r, ev).unwrap();
    assert_eq!(a.pending_count(), 0);
    assert!(!a.is_live(&r));
    assert_eq!(a.available(), CAP);
}

#[test]
fn pending_region_blocks_free_and_refree() {
    let (_bk, a, _b) = setup();
    let r = a.allocate(100).unwrap();
    let (ev, _ctl) = Event::mock();
    a.free_deferred(&r, ev.clone()).unwrap();
    assert_eq!(a.free(&r), Err(AllocatorError::StaleRegion));
    assert_eq!(a.free_deferred(&r, ev), Err(AllocatorError::StaleRegion));
    assert_eq!(a.pending_count(), 1);
    assert!(a.is_live(&r));
}

#[test]
fn deferred_free_of_dead_region_is_stale() {
    let (_bk, a, _b) = setup();
    let r = a.allocate(100).unwrap();
    a.free(&r).unwrap();
    let (ev, _ctl) = Event::mock();
    assert_eq!(a.free_deferred(&r, ev), Err(AllocatorError::StaleRegion));
    assert_eq!(a.pending_count(), 0);
}

#[test]
fn pending_space_is_not_reused() {
    let (_bk, a, _b) = setup();
    let r = a.allocate(CAP).unwrap();
    let (ev, ctl) = Event::mock();
    a.free_deferred(&r, ev).unwrap();
    assert_eq!(
        a.allocate(10),
        Err(AllocatorError::OutOfMemory {
            arena: ArenaKind::Weights,
            requested: 10,
            free: 0,
        })
    );
    ctl.complete();
    let again = a.allocate(10).unwrap();
    assert_eq!(again.offset, 0);
    assert_eq!(a.pending_count(), 0);
}

#[test]
fn reset_makes_region_wait_again() {
    let (_bk, a, _b) = setup();
    let r1 = a.allocate(10).unwrap();
    let r2 = a.allocate(10).unwrap();
    let (e1, c1) = Event::mock();
    let (e2, c2) = Event::mock();
    a.free_deferred(&r1, e1).unwrap();
    a.free_deferred(&r2, e2).unwrap();
    c1.complete();
    c2.complete();
    c1.reset();
    assert_eq!(a.reap(), 1);
    assert!(a.is_live(&r1));
    assert!(!a.is_live(&r2));
    c1.complete();
    c1.reset();
    assert_eq!(a.reap(), 0);
    assert_eq!(a.pending_count(), 1);
}

#[test]
fn failed_event_keeps_region_pending() {
    let (_bk, a, _b) = setup();
    let r = a.allocate(50).unwrap();
    let (ev, ctl) = Event::mock();
    ctl.fail("boom");
    a.free_deferred(&r, ev).unwrap();
    assert_eq!(a.pending_count(), 1);
    assert_eq!(a.reap(), 0);
    assert_eq!(a.reap(), 0);
    assert!(a.is_live(&r));
    ctl.complete();
    assert_eq!(a.reap(), 1);
    assert!(!a.is_live(&r));
}

#[test]
fn cpu_event_signal() {
    let (_bk, a, _b) = setup();
    let r = a.allocate(50).unwrap();
    let (ev, sig) = Event::cpu();
    a.free_deferred(&r, ev).unwrap();
    assert_eq!(a.reap(), 0);
    sig.signal();
    assert_eq!(a.reap(), 1);
    assert_eq!(a.available(), CAP);
}

#[test]
fn reap_drops_entry_for_invalid_region_without_counting() {
    let (bk, a, _b) = setup();
    let r = a.allocate(50).unwrap();
    let (ev, ctl) = Event::mock();
    a.free_deferred(&r, ev).unwrap();
    bk.free(ArenaKind::Weights, &r).unwrap();
    ctl.complete();
    assert_eq!(a.reap(), 0);
    assert_eq!(a.pending_count(), 0);
    assert_eq!(a.available(), CAP);
}

#[test]
fn allocate_reaps_completed() {
    let (_bk, a, _b) = setup();
    let r = a.allocate(CAP).unwrap();
    let (ev, ctl) = Event::mock();
    a.free_deferred(&r, ev).unwrap();
    ctl.complete();
    assert!(a.allocate(CAP).is_ok());
    assert_eq!(a.pending_count(), 0);
}

#[test]
fn available_does_not_reap() {
    let (_bk, a, _b) = setup();
    let r = a.allocate(100).unwrap();
    let (ev, ctl) = Event::mock();
    a.free_deferred(&r, ev).unwrap();
    ctl.complete();
    assert_eq!(a.available(), CAP - 100);
    assert_eq!(a.pending_count(), 1);
    assert!(a.available() <= a.capacity());
}

#[test]
fn concurrent_deferred_free_of_same_region_succeeds_once() {
    let (_bk, a, _b) = setup();
    let a = Arc::new(a);
    let r = a.allocate(64).unwrap();
    let ok = Arc::new(AtomicUsize::new(0));
    let handles: Vec<_> = (0..8)
        .map(|_| {
            let a = Arc::clone(&a);
            let ok = Arc::clone(&ok);
            std::thread::spawn(move || {
                let (ev, _ctl) = Event::mock();
                match a.free_deferred(&r, ev) {
                    Ok(()) => {
                        ok.fetch_add(1, Ordering::SeqCst);
                    }
                    Err(e) => assert_eq!(e, AllocatorError::StaleRegion),
                }
            })
        })
        .collect();
    for h in handles {
        h.join().unwrap();
    }
    assert_eq!(ok.load(Ordering::SeqCst), 1);
    assert_eq!(a.pending_count(), 1);
}

#[test]
fn concurrent_reap_frees_each_region_once() {
    let (_bk, a, _b) = setup();
    let a = Arc::new(a);
    let (ev, ctl) = Event::mock();
    for _ in 0..32 {
        let r = a.allocate(8).unwrap();
        a.free_deferred(&r, ev.clone()).unwrap();
    }
    assert_eq!(a.pending_count(), 32);
    ctl.complete();
    let handles: Vec<_> = (0..8)
        .map(|_| {
            let a = Arc::clone(&a);
            std::thread::spawn(move || a.reap())
        })
        .collect();
    let total: usize = handles.into_iter().map(|h| h.join().unwrap()).sum();
    assert_eq!(total, 32);
    assert_eq!(a.available(), CAP);
}

#[test]
fn poisoned_lock_is_recovered() {
    let (_bk, a, _b) = setup();
    let a = Arc::new(a);
    let a2 = Arc::clone(&a);
    let _ = std::thread::spawn(move || {
        let _g = a2.pending.lock().unwrap();
        panic!("poison");
    })
    .join();
    assert!(a.pending.is_poisoned());
    let r = a.allocate(10).unwrap();
    let (ev, ctl) = Event::mock();
    ctl.complete();
    a.free_deferred(&r, ev).unwrap();
    assert_eq!(a.pending_count(), 0);
    assert_eq!(a.reap(), 0);
}

#[test]
fn set_construction_and_lookup() {
    let (bk, a, _b) = setup();
    let set = AllocatorSet::new(
        Arc::clone(&bk),
        &[
            (
                Arena {
                    kind: ArenaKind::Weights,
                    capacity: CAP,
                },
                DeviceKind::Gpu,
            ),
            (
                Arena {
                    kind: ArenaKind::Staging,
                    capacity: 512,
                },
                DeviceKind::Cpu,
            ),
        ],
    )
    .unwrap();
    assert_eq!(set.len(), 2);
    assert!(!set.is_empty());
    assert_eq!(set.size_hint(ArenaKind::Weights), Some(CAP));
    assert_eq!(set.size_hint(ArenaKind::Staging), Some(512));
    assert_eq!(set.size_hint(ArenaKind::Workspace), None);
    assert!(set.allocator(ArenaKind::Workspace).is_none());
    let wa = set.allocator(ArenaKind::Weights).unwrap();
    assert_eq!(wa.device_kind(), DeviceKind::Gpu);
    let r = wa.allocate(10).unwrap();
    assert_eq!(set.allocator_for(&r).unwrap().arena(), ArenaKind::Weights);
    // shared bookkeeping: the standalone allocator sees the same liveness
    assert!(a.is_live(&r));
    wa.free(&r).unwrap();
    // liveness is not checked by allocator_for
    assert!(set.allocator_for(&r).is_ok());
}

#[test]
fn set_rejects_duplicates_and_unknown_arena() {
    let (bk, _a, _b) = setup();
    let arena = Arena {
        kind: ArenaKind::Weights,
        capacity: CAP,
    };
    let dup = AllocatorSet::new(
        Arc::clone(&bk),
        &[(arena, DeviceKind::Gpu), (arena, DeviceKind::Cpu)],
    );
    assert_eq!(
        dup.err(),
        Some(AllocatorSetError::DuplicateArena(ArenaKind::Weights))
    );
    let set = AllocatorSet::new(Arc::clone(&bk), &[(arena, DeviceKind::Gpu)]).unwrap();
    let foreign = Region {
        arena: ArenaKind::Staging,
        offset: 0,
        len: 8,
        generation: 0,
    };
    assert_eq!(
        set.allocator_for(&foreign).err(),
        Some(AllocatorError::StaleRegion)
    );
}

#[test]
fn empty_set() {
    let (bk, _a, _b) = setup();
    let set = AllocatorSet::new(bk, &[]).unwrap();
    assert_eq!(set.len(), 0);
    assert!(set.is_empty());
}
