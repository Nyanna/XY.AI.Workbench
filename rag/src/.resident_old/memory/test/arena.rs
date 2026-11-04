use super::testing::Gate;
use super::*;

fn alloc_for(cap: u64) -> ArenaAllocator {
    ArenaAllocator::new(ArenaKind::Workspace, DeviceKind::Cpu, cap)
}

#[test]
fn exhaustion_reports_arena_and_free_bytes() {
    let a = alloc_for(64);
    let _r = a.alloc(48, 8).unwrap();
    match a.alloc(32, 8) {
        Err(MemoryError::ArenaExhausted { arena, requested: 32, free: 16 }) => {
            assert_eq!(arena, ArenaKind::Workspace)
        }
        other => panic!("unexpected {other:?}"),
    }
}

#[test]
fn regions_carry_arena_identity_and_stale_free_is_rejected() {
    let a = alloc_for(64);
    let r = a.alloc(16, 8).unwrap();
    assert_eq!(r.arena, ArenaKind::Workspace.id());
    a.free(r).unwrap();
    assert!(matches!(a.free(r), Err(MemoryError::StaleRegion { .. })));
    let r2 = a.alloc(16, 8).unwrap();
    assert_eq!(r2.offset, r.offset);
    assert_ne!(r2.gen, r.gen);
    assert!(matches!(a.free(r), Err(MemoryError::StaleRegion { .. })));
}

#[test]
fn release_after_defers_reuse_until_event_completes() {
    let a = alloc_for(32);
    let r = a.alloc(32, 8).unwrap();
    let gate = Gate::closed();
    a.release_after(r, gate.event.clone()).unwrap();
    assert_eq!(a.pending(), 1);
    assert!(matches!(a.alloc(8, 8), Err(MemoryError::ArenaExhausted { .. })));
    gate.open();
    let again = a.alloc(8, 8).unwrap();
    assert_eq!(again.offset, 0);
    assert_eq!(a.pending(), 0);
}

#[test]
fn pending_region_cannot_be_freed_or_released_twice() {
    let a = alloc_for(32);
    let r = a.alloc(16, 8).unwrap();
    let gate = Gate::closed();
    a.release_after(r, gate.event.clone()).unwrap();
    assert!(matches!(a.free(r), Err(MemoryError::StaleRegion { .. })));
    assert!(a.release_after(r, gate.event.clone()).is_err());
    gate.open();
}

#[test]
fn set_routes_regions_to_their_class() {
    let set = ArenaSet::new(
        DeviceKind::Cpu,
        ArenaSizes { weights: 64, index_segment: 128, ..Default::default() },
    );
    let w = set.get(ArenaKind::Weights).alloc(8, 8).unwrap();
    assert_eq!(set.for_region(w).unwrap().kind(), ArenaKind::Weights);
    assert!(matches!(
        set.get(ArenaKind::Staging).alloc(1, 1),
        Err(MemoryError::ArenaExhausted { .. })
    ));
}
