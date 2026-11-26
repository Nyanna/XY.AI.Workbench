use super::super::memory_backend::MAX_ALIGN;
use super::super::super::_4::_3::a::region_handle::Generation;
use super::*;
const KIB: u64 = 1024;
const G: u64 = ALLOC_GRANULE;
fn caps() -> ArenaCapacities {
    let mut c = ArenaCapacities::default();
    c.set(ArenaKind::Weights, 64 * KIB);
    c.set(ArenaKind::IndexSegments, 64 * KIB);
    c.set(ArenaKind::Workspace, 16 * KIB);
    c.set(ArenaKind::ParameterBlock, 4 * KIB);
    c.set(ArenaKind::MappedHost, 16 * KIB);
    c
}
fn backend() -> CpuBackend {
    CpuBackend::new(caps(), 4).unwrap()
}
#[test]
fn new_arenas_are_zeroed() {
    let b = backend();
    let r = b.allocate(ArenaKind::Weights, 1000, 1).unwrap();
    let mut buf = vec![0xAAu8; r.len() as usize];
    b.read(&r, 0, &mut buf).unwrap();
    assert!(buf.iter().all(|& x | x == 0));
}
#[test]
fn allocation_is_rounded_and_aligned() {
    let b = backend();
    let r1 = b.allocate(ArenaKind::Weights, 1, 64).unwrap();
    let r2 = b.allocate(ArenaKind::Weights, G + 1, 32).unwrap();
    let r3 = b.allocate(ArenaKind::Weights, 1, MAX_ALIGN).unwrap();
    assert_eq!(r1.len(), G);
    assert_eq!(r2.len(), 2 * G);
    assert_eq!(r3.len(), G);
    for r in [r1, r2, r3] {
        assert_eq!(r.offset() % G, 0);
    }
    assert_ne!(r1.offset(), r2.offset());
}
#[test]
fn invalid_allocation_arguments() {
    let b = backend();
    let w = ArenaKind::Weights;
    assert_eq!(b.allocate(w, 0, 1), Err(BackendError::AllocationFailed));
    assert_eq!(b.allocate(w, 8, 0), Err(BackendError::AllocationFailed));
    assert_eq!(b.allocate(w, 8, 3), Err(BackendError::AllocationFailed));
    assert_eq!(b.allocate(w, 8, MAX_ALIGN * 2), Err(BackendError::AllocationFailed));
    assert_eq!(b.allocate(w, u64::MAX, 1), Err(BackendError::AllocationFailed));
    assert_eq!(
        b.allocate(ArenaKind::Staging, 8, 1), Err(BackendError::AllocationFailed)
    );
}
#[test]
fn exhaustion_reports_out_of_memory() {
    let b = backend();
    let r = b.allocate(ArenaKind::ParameterBlock, 4 * KIB, 1).unwrap();
    assert_eq!(
        b.allocate(ArenaKind::ParameterBlock, 100, 1), Err(BackendError::OutOfMemory {
        arena : ArenaKind::ParameterBlock, requested : G, free : 0, })
    );
    b.free(&r).unwrap();
    assert!(b.allocate(ArenaKind::ParameterBlock, 100, 1).is_ok());
}
#[test]
fn zero_capacity_arena_is_empty() {
    let mut c = caps();
    c.set(ArenaKind::Workspace, 0);
    let b = CpuBackend::new(c, 1).unwrap();
    assert!(
        matches!(b.allocate(ArenaKind::Workspace, 1, 1), Err(BackendError::OutOfMemory {
        free : 0, .. }))
    );
    assert_eq!(b.available(ArenaKind::Workspace), Ok(0));
}
#[test]
fn write_read_roundtrip_with_offset() {
    let b = backend();
    let r = b.allocate(ArenaKind::IndexSegments, 256, 1).unwrap();
    b.write(&r, 10, &[1, 2, 3, 4]).unwrap();
    let mut out = [0u8; 6];
    b.read(&r, 9, &mut out).unwrap();
    assert_eq!(out, [0, 1, 2, 3, 4, 0]);
}
#[test]
fn out_of_range_access_changes_nothing() {
    let b = backend();
    let r = b.allocate(ArenaKind::Weights, G, 1).unwrap();
    let next = b.allocate(ArenaKind::Weights, G, 1).unwrap();
    b.write(&next, 0, &[7; G as usize]).unwrap();
    assert_eq!(b.write(& r, G - 4, & [1; 8]), Err(BackendError::StaleRegion));
    assert_eq!(b.write(& r, u64::MAX, & [1; 8]), Err(BackendError::StaleRegion));
    assert_eq!(b.write(& r, G + 1, & []), Err(BackendError::StaleRegion));
    let mut out = [0u8; G as usize];
    b.read(&r, 0, &mut out).unwrap();
    assert!(out.iter().all(|& x | x == 0));
    b.read(&next, 0, &mut out).unwrap();
    assert!(out.iter().all(|& x | x == 7));
    let mut big = [0u8; G as usize + 1];
    assert_eq!(b.read(& r, 0, & mut big), Err(BackendError::StaleRegion));
}
#[test]
fn zero_length_access_is_noop() {
    let b = backend();
    let r = b.allocate(ArenaKind::Weights, G, 1).unwrap();
    assert_eq!(b.write(& r, G, & []), Ok(()));
    assert_eq!(b.read(& r, 0, & mut []), Ok(()));
    b.free(&r).unwrap();
    assert_eq!(b.write(& r, 0, & []), Err(BackendError::StaleRegion));
}
#[test]
fn access_to_freed_region_is_stale() {
    let b = backend();
    let r = b.allocate(ArenaKind::Weights, G, 1).unwrap();
    b.free(&r).unwrap();
    let mut out = [0u8; 4];
    assert_eq!(b.read(& r, 0, & mut out), Err(BackendError::StaleRegion));
    assert_eq!(b.write(& r, 0, & out), Err(BackendError::StaleRegion));
    assert_eq!(b.free(& r), Err(BackendError::StaleRegion));
}
#[test]
fn foreign_region_is_stale() {
    let b = backend();
    let other = backend();
    let r = other.allocate(ArenaKind::Weights, G, 1).unwrap();
    assert_eq!(b.free(& r), Err(BackendError::StaleRegion));
    let staging = RegionHandle::new(ArenaKind::Staging, 0, G, Generation(0)).unwrap();
    assert_eq!(b.free(& staging), Err(BackendError::StaleRegion));
    assert_eq!(b.access(& staging).unwrap_err(), BackendError::StaleRegion);
}
#[test]
fn reused_space_keeps_old_bytes() {
    let b = backend();
    let r = b.allocate(ArenaKind::Weights, G, 1).unwrap();
    b.write(&r, 0, &[9; G as usize]).unwrap();
    b.free(&r).unwrap();
    let r2 = b.allocate(ArenaKind::Weights, G, 1).unwrap();
    assert_eq!(r2.offset(), r.offset());
    let mut out = [0u8; G as usize];
    b.read(&r2, 0, &mut out).unwrap();
    assert!(out.iter().all(|& x | x == 9));
}
#[test]
fn deferred_free_waits_for_event() {
    let b = backend();
    let r = b.allocate(ArenaKind::ParameterBlock, 4 * KIB, 1).unwrap();
    let (ev, ctl) = Event::mock();
    b.free_deferred(&r, ev.clone()).unwrap();
    let budget = b.budget().unwrap();
    assert_eq!(budget.free, budget.total - 4 * KIB);
    assert_eq!(b.available(ArenaKind::ParameterBlock), Ok(0));
    let mut out = [0u8; 4];
    assert!(b.read(& r, 0, & mut out).is_ok());
    assert_eq!(b.free(& r), Err(BackendError::StaleRegion));
    assert_eq!(b.free_deferred(& r, ev), Err(BackendError::StaleRegion));
    assert_eq!(b.reap(), 0);
    assert!(
        matches!(b.allocate(ArenaKind::ParameterBlock, 64, 1),
        Err(BackendError::OutOfMemory { .. }))
    );
    ctl.complete();
    assert_eq!(b.reap(), 1);
    assert_eq!(b.read(& r, 0, & mut out), Err(BackendError::StaleRegion));
    assert!(b.allocate(ArenaKind::ParameterBlock, 64, 1).is_ok());
}
#[test]
fn allocate_reaps_completed_deferred_frees() {
    let b = backend();
    let r = b.allocate(ArenaKind::ParameterBlock, 4 * KIB, 1).unwrap();
    let (ev, ctl) = Event::mock();
    b.free_deferred(&r, ev).unwrap();
    ctl.complete();
    assert!(b.allocate(ArenaKind::ParameterBlock, 4 * KIB, 1).is_ok());
}
#[test]
fn copy_within_arena_and_leaves_tail() {
    let b = backend();
    let src = b.allocate(ArenaKind::Weights, G, 1).unwrap();
    let dst = b.allocate(ArenaKind::Weights, 2 * G, 1).unwrap();
    let pattern: Vec<u8> = (0..G as usize).map(|i| i as u8).collect();
    b.write(&src, 0, &pattern).unwrap();
    b.write(&dst, 0, &[0xEE; 2 * G as usize]).unwrap();
    b.copy(&src, &dst).unwrap();
    let mut out = [0u8; 2 * G as usize];
    b.read(&dst, 0, &mut out).unwrap();
    assert_eq!(& out[..G as usize], & pattern[..]);
    assert!(out[G as usize..].iter().all(|& x | x == 0xEE));
}
#[test]
fn copy_to_same_region_is_noop() {
    let b = backend();
    let r = b.allocate(ArenaKind::Weights, G, 1).unwrap();
    let pattern: Vec<u8> = (0..G as usize).map(|i| i as u8).collect();
    b.write(&r, 0, &pattern).unwrap();
    b.copy(&r, &r).unwrap();
    let mut out = [0u8; G as usize];
    b.read(&r, 0, &mut out).unwrap();
    assert_eq!(& out[..], & pattern[..]);
}
#[test]
fn copy_between_arenas() {
    let b = backend();
    let src = b.allocate(ArenaKind::Weights, 100, 1).unwrap();
    let dst = b.allocate(ArenaKind::MappedHost, 100, 1).unwrap();
    let pattern: Vec<u8> = (0..src.len() as usize).map(|i| i as u8).collect();
    b.write(&src, 0, &pattern).unwrap();
    b.copy(&src, &dst).unwrap();
    let mut out = vec![0u8; dst.len() as usize];
    b.read(&dst, 0, &mut out).unwrap();
    assert_eq!(out, pattern);
}
#[test]
fn copy_with_smaller_destination_is_rejected() {
    let b = backend();
    let src = b.allocate(ArenaKind::Weights, 2 * G, 1).unwrap();
    let dst = b.allocate(ArenaKind::Weights, G, 1).unwrap();
    b.write(&src, 0, &[5; 2 * G as usize]).unwrap();
    assert_eq!(b.copy(& src, & dst), Err(BackendError::StaleRegion));
    let mut out = [1u8; G as usize];
    b.read(&dst, 0, &mut out).unwrap();
    assert!(out.iter().all(|& x | x == 0));
}
#[test]
fn copy_with_stale_region_is_rejected() {
    let b = backend();
    let src = b.allocate(ArenaKind::Weights, G, 1).unwrap();
    let dst = b.allocate(ArenaKind::Weights, G, 1).unwrap();
    b.free(&dst).unwrap();
    assert_eq!(b.copy(& src, & dst), Err(BackendError::StaleRegion));
    assert_eq!(b.copy(& dst, & src), Err(BackendError::StaleRegion));
}
#[test]
fn copy_rejects_sub_regions_and_uses_memmove_semantics() {
    let b = backend();
    let big = b.allocate(ArenaKind::Weights, 2 * G, 1).unwrap();
    let src = RegionHandle::new(ArenaKind::Weights, big.offset(), G, big.generation())
        .unwrap();
    let dst = RegionHandle::new(
            ArenaKind::Weights,
            big.offset() + G / 2,
            G,
            big.generation(),
        )
        .unwrap();
    assert_eq!(b.copy(& src, & dst), Err(BackendError::StaleRegion));
    let pattern: Vec<u8> = (0..=255).collect();
    let mut data = RawBuf::zeroed(256).unwrap();
    data.as_mut_slice().copy_from_slice(&pattern);
    data.as_mut_slice().copy_within(0..128, 64);
    assert_eq!(& data.as_slice() [64..192], & pattern[..128]);
}
#[test]
fn buffers_are_aligned_and_zeroed() {
    let raw = RawBuf::zeroed(1000).unwrap();
    assert_eq!(raw.as_slice().as_ptr() as usize % BUFFER_ALIGN, 0);
    assert_eq!(BUFFER_ALIGN as u64, MAX_ALIGN);
    assert!(raw.as_slice().iter().all(|& x | x == 0));
    assert!(RawBuf::zeroed(0).unwrap().as_slice().is_empty());
}
#[test]
fn huge_capacity_fails_without_abort() {
    let mut c = ArenaCapacities::default();
    c.set(ArenaKind::Weights, u64::MAX);
    assert_eq!(
        CpuBackend::new(c, 1).unwrap_err(), BackendError::OutOfMemory { arena :
        ArenaKind::Weights, requested : u64::MAX, free : 0, }
    );
}
#[test]
fn budget_tracks_allocations() {
    let b = backend();
    let total = 64 * KIB + 64 * KIB + 16 * KIB + 4 * KIB + 16 * KIB;
    let before = b.budget().unwrap();
    assert_eq!(before.total, total);
    assert_eq!(before.free, total);
    assert_eq!(before.generation_reserve, caps().generation_reserve(4));
    let r = b.allocate(ArenaKind::Weights, 1000, 1).unwrap();
    let after = b.budget().unwrap();
    assert_eq!(after.free, total - r.len());
    assert!(after.free <= after.total);
    b.free(&r).unwrap();
    assert_eq!(b.budget().unwrap().free, total);
}
#[test]
fn available_tracks_one_arena() {
    let b = backend();
    assert_eq!(b.available(ArenaKind::Weights), Ok(64 * KIB));
    assert_eq!(b.available(ArenaKind::Staging), Ok(0));
    let r = b.allocate(ArenaKind::Weights, 1000, 1).unwrap();
    assert_eq!(b.available(ArenaKind::Weights), Ok(64 * KIB - r.len()));
    assert_eq!(b.available(ArenaKind::IndexSegments), Ok(64 * KIB));
    b.free(&r).unwrap();
    assert_eq!(b.available(ArenaKind::Weights), Ok(64 * KIB));
}
#[test]
fn memory_query_matches_budget() {
    let b = backend();
    let _r = b.allocate(ArenaKind::Workspace, 500, 1).unwrap();
    let info = b.query().unwrap();
    let budget = b.budget().unwrap();
    assert_eq!(info.free, budget.free);
    assert_eq!(info.total, budget.total);
}
#[test]
fn access_shares_bytes_with_backend() {
    let b = backend();
    let r = b.allocate(ArenaKind::Weights, 200, 1).unwrap();
    let acc = b.access(&r).unwrap();
    assert_eq!(acc.len(), r.len() as usize);
    acc.write(8, &[1, 2, 3]).unwrap();
    let mut out = [0u8; 3];
    b.read(&r, 8, &mut out).unwrap();
    assert_eq!(out, [1, 2, 3]);
    b.write(&r, 20, &[4, 5]).unwrap();
    let clone = acc.clone();
    let mut out2 = [0u8; 2];
    clone.read(20, &mut out2).unwrap();
    assert_eq!(out2, [4, 5]);
    assert!(clone.same_region(& acc));
}
#[test]
fn access_region_addresses_its_own_bytes() {
    let b = backend();
    let first = b.allocate(ArenaKind::Weights, G, 1).unwrap();
    let second = b.allocate(ArenaKind::Weights, G, 1).unwrap();
    b.access(&second).unwrap().write(0, &[3; G as usize]).unwrap();
    let mut out = [0u8; G as usize];
    b.read(&first, 0, &mut out).unwrap();
    assert!(out.iter().all(|& x | x == 0));
    assert_eq!(
        b.access(& first).unwrap().write(G as usize - 4, & [1; 8]),
        Err(AccessError::OutOfBounds { offset : G as usize - 4, len : 8, region_len : G
        as usize })
    );
}
#[test]
fn access_reports_stale_after_free() {
    let b = backend();
    let r = b.allocate(ArenaKind::Weights, G, 1).unwrap();
    let acc = b.access(&r).unwrap();
    b.free(&r).unwrap();
    let mut out = [0u8; 4];
    assert_eq!(
        acc.read(0, & mut out), Err(AccessError::Backend("stale Region".to_string()))
    );
    assert_eq!(
        acc.write(0, & out), Err(AccessError::Backend("stale Region".to_string()))
    );
    assert_eq!(b.access(& r).unwrap_err(), BackendError::StaleRegion);
}
#[test]
fn stale_generations_are_rejected() {
    let b = backend();
    let h = b.allocate(ArenaKind::Weights, G, 1).unwrap();
    let big = h.with_generation(Generation(u64::from(u32::MAX) + 1));
    assert_eq!(b.free(& big), Err(BackendError::StaleRegion));
    let wrong = h.with_generation(Generation(h.generation().0 + 1));
    assert_eq!(b.access(& wrong).unwrap_err(), BackendError::StaleRegion);
    b.free(&h).unwrap();
    assert_eq!(b.access(& h).unwrap_err(), BackendError::StaleRegion);
}
#[test]
fn usable_as_trait_object() {
    let b: Arc<dyn MemoryBackend> = Arc::new(backend());
    let r = b.allocate(ArenaKind::Weights, 10, 1).unwrap();
    assert_eq!(r.len(), G);
    assert_eq!(r.offset() % G, 0);
    assert_eq!(b.available(ArenaKind::Weights), Ok(64 * KIB - G));
    let budget = b.budget().unwrap();
    assert_eq!(budget.free, budget.total - G);
    let (ev, ctl) = Event::mock();
    b.free_deferred(&r, ev).unwrap();
    assert_eq!(b.free(& r), Err(BackendError::StaleRegion));
    ctl.complete();
    let again = b.allocate(ArenaKind::Weights, 10, 1).unwrap();
    b.free(&again).unwrap();
    assert_eq!(b.available(ArenaKind::Weights), Ok(64 * KIB));
}
#[test]
fn concurrent_use_from_many_threads() {
    let b = backend();
    std::thread::scope(|s| {
        for t in 0..8u8 {
            let b = &b;
            s.spawn(move || {
                for i in 0..200u32 {
                    let r = b.allocate(ArenaKind::Weights, 100, 8).unwrap();
                    let v = t.wrapping_add(i as u8);
                    b.write(&r, 0, &[v; 100]).unwrap();
                    let mut out = [0u8; 100];
                    b.read(&r, 0, &mut out).unwrap();
                    assert!(out.iter().all(|& x | x == v));
                    let dst = b.allocate(ArenaKind::MappedHost, 100, 1).unwrap();
                    b.copy(&r, &dst).unwrap();
                    b.read(&dst, 0, &mut out).unwrap();
                    assert!(out.iter().all(|& x | x == v));
                    b.free(&r).unwrap();
                    b.free(&dst).unwrap();
                }
            });
        }
    });
    let info = b.budget().unwrap();
    assert_eq!(info.free, info.total);
}
