use std::sync::atomic::Ordering;
use std::sync::Arc;
use super::super::fixture::*;
use super::super::GpuServices;
use super::*;
fn ids(list: &[EntryId]) -> Vec<u64> {
    list.iter().map(|id| id.0).collect()
}
fn all(count: u64) -> EntrySet {
    let wanted: Vec<u64> = (1..=count).collect();
    set(&wanted)
}
#[test]
fn entries_are_placed_in_ascending_slots_and_reported() {
    let f = Fixture::new();
    for id in [3u64, 1, 2] {
        f.put(id, 2, id as f32);
    }
    let report = f.manager.ensure(&set(&[1, 2, 3])).unwrap();
    assert_eq!(ids(& report.loaded), vec![1, 2, 3]);
    assert_eq!(report.bytes_moved, 3 * SLOT_BYTES);
    assert_eq!(report.segments_created, 1);
    assert!(report.failed.is_empty());
    assert!(report.absent_in_index.is_empty());
    assert!(report.already_resident.is_empty());
    let queued = f.queue.drain();
    assert_eq!(queued.len(), 1);
    let seg = &queued[0];
    let placed: Vec<(u64, u32)> = seg
        .bindings
        .iter()
        .map(|b| (b.entry.entry.0, b.slot.0))
        .collect();
    assert_eq!(placed, vec![(1, 0), (2, 1), (3, 2)]);
    for binding in &seg.bindings {
        assert_eq!(binding.stamp, f.stamp_of(binding.entry.entry.0));
        assert_eq!(binding.entry.section, 0);
    }
    let addr = seg.assignment.device_addr(Slot(1)).unwrap();
    assert_eq!(f.services.read_device(addr.0, SLOT_BYTES as usize), vector(2, 2.0));
    assert!(seg.writer.is_some());
}
#[test]
fn short_entries_are_padded_with_the_mask_value() {
    let f = Fixture::new();
    f.put(1, 1, 5.0);
    f.manager.ensure(&set(&[1])).unwrap();
    let seg = f.queue.drain().pop().unwrap();
    let addr = seg.assignment.device_addr(Slot(0)).unwrap();
    let mut expected = vector(1, 5.0);
    for _ in 0..DIM {
        expected.extend_from_slice(&(-10_000.0f32).to_le_bytes());
    }
    assert_eq!(f.services.read_device(addr.0, SLOT_BYTES as usize), expected);
    assert_eq!(seg.states.meta(Slot(0)).unwrap().tokens, 1);
}
#[test]
fn ensure_does_not_publish() {
    let f = Fixture::new();
    f.put(1, 2, 1.0);
    f.manager.ensure(&set(&[1])).unwrap();
    assert_eq!(f.manager.segment_count(), 0);
    assert!(f.manager.slot_of(EntryId(1)).is_none());
    assert_eq!(f.manager.pending_updates(), 0);
    assert!(f.services.mapping_store().current().is_empty());
    assert!(f.services.table_swap().current_table().is_none());
}
#[test]
fn entries_waiting_for_publication_count_as_present() {
    let f = Fixture::new();
    for id in 1..=3 {
        f.put(id, 2, id as f32);
    }
    f.manager.ensure(&set(&[1, 2])).unwrap();
    let second = f.manager.ensure(&set(&[1, 2, 3])).unwrap();
    assert_eq!(ids(& second.loaded), vec![3]);
    assert_eq!(ids(& second.already_resident), vec![1, 2]);
    assert_eq!(second.segments_created, 1);
    assert_eq!(f.queue.len(), 2);
}
#[test]
fn resident_entries_are_not_loaded_again() {
    let f = Fixture::new();
    f.resident(3);
    let report = f.manager.ensure(&all(3)).unwrap();
    assert!(report.loaded.is_empty());
    assert_eq!(ids(& report.already_resident), vec![1, 2, 3]);
    assert_eq!(report.segments_created, 0);
    assert_eq!(report.bytes_moved, 0);
    assert!(f.queue.is_empty());
}
#[test]
fn entries_with_a_pending_bind_count_as_present() {
    let f = Fixture::new();
    for id in 1..=3 {
        f.put(id, 2, id as f32);
    }
    f.manager.ensure(&all(3)).unwrap();
    f.publish_queued();
    assert_eq!(f.manager.pending_updates(), 3);
    let report = f.manager.ensure(&all(3)).unwrap();
    assert!(report.loaded.is_empty());
    assert_eq!(ids(& report.already_resident), vec![1, 2, 3]);
}
#[test]
fn stale_resident_entries_are_left_to_reconcile() {
    let f = Fixture::new();
    f.resident(2);
    f.put(1, 2, 99.0);
    let report = f.manager.ensure(&all(2)).unwrap();
    assert!(report.loaded.is_empty());
    assert_eq!(ids(& report.already_resident), vec![1, 2]);
}
#[test]
fn entries_missing_in_the_index_are_reported() {
    let f = Fixture::new();
    f.put(1, 2, 1.0);
    let report = f.manager.ensure(&set(&[1, 99])).unwrap();
    assert_eq!(ids(& report.loaded), vec![1]);
    assert_eq!(ids(& report.absent_in_index), vec![99]);
}
#[test]
fn bad_vectors_fail_only_their_entry() {
    let f = Fixture::new();
    f.put(5, 3, 1.0);
    f.index
        .write(
            EntryId(6),
            PayloadRef {
                vector: &[0u8; 8],
                meta: b"",
            },
        )
        .unwrap();
    f.put(7, 2, 7.0);
    let report = f.manager.ensure(&set(&[5, 6, 7])).unwrap();
    assert_eq!(
        report.failed, vec![(EntryId(5), GpuManagerError::TooManyTokens { tokens : 3, max
        : MAX_TOKENS }), (EntryId(6), GpuManagerError::InvalidVector { len : 8, row_bytes
        : 16 }),]
    );
    assert_eq!(ids(& report.loaded), vec![7]);
}
#[test]
fn index_dtype_must_match_the_configuration() {
    let index = reference_index(DType::F16);
    let (_services, _queue, manager) = build(
        Arc::new(FixedIndex(index as Arc<dyn Index>)),
    );
    let err = manager.ensure(&set(&[1])).unwrap_err();
    assert_eq!(
        err, GpuManagerError::DTypeMismatch { index : DType::F16, configured : DType::F32
        }
    );
}
#[test]
fn unavailable_index_changes_nothing() {
    let (services, queue, manager) = build(Arc::new(MissingIndex));
    let err = manager.ensure(&set(&[1])).unwrap_err();
    assert_eq!(err, GpuManagerError::Index("index is not available".to_string()));
    assert!(queue.is_empty());
    assert_eq!(services.arena().stats().live_segments, 0);
}
#[test]
fn exhausted_arena_reports_the_filled_segments_first() {
    let f = Fixture::with_capacity(2048);
    for id in 1..=40 {
        f.put(id, 2, id as f32);
    }
    let err = f.manager.ensure(&all(40)).unwrap_err();
    assert!(matches!(err, GpuManagerError::Arena(SegmentArenaError::Exhausted { .. })));
    let placed: usize = f.queue.drain().iter().map(|s| s.bindings.len()).sum();
    assert!(placed > 0 && placed < 40);
    let again = f.manager.ensure(&all(40)).unwrap_err();
    assert!(
        matches!(again, GpuManagerError::Arena(SegmentArenaError::Exhausted { .. }))
    );
    assert!(f.queue.is_empty());
}
#[test]
fn failed_device_writes_leave_the_slot_unused_and_the_entry_loadable() {
    let f = Fixture::new();
    for id in 1..=3 {
        f.put(id, 2, id as f32);
    }
    f.services.fail_visible.store(true, Ordering::SeqCst);
    let report = f.manager.ensure(&all(3)).unwrap();
    assert!(report.loaded.is_empty());
    assert_eq!(report.failed.len(), 3);
    assert!(
        report.failed.iter().all(| (_, e) | matches!(e, GpuManagerError::Memory(_)))
    );
    assert_eq!(report.segments_created, 0);
    assert!(f.queue.is_empty());
    f.services.fail_visible.store(false, Ordering::SeqCst);
    let report = f.manager.ensure(&all(3)).unwrap();
    assert_eq!(ids(& report.loaded), vec![1, 2, 3]);
    assert_eq!(report.segments_created, 1);
    assert_eq!(f.services.arena().stats().live_segments, 1);
    let seg = f.queue.drain().pop().unwrap();
    let slots: Vec<u32> = seg.bindings.iter().map(|b| b.slot.0).collect();
    assert_eq!(slots, vec![3, 4, 5]);
}
#[test]
fn segment_without_device_address_is_released_again() {
    let f = Fixture::new();
    f.put(1, 2, 1.0);
    f.services.fail_base.store(true, Ordering::SeqCst);
    let report = f.manager.ensure(&set(&[1])).unwrap();
    assert!(matches!(report.failed[0].1, GpuManagerError::Memory(_)));
    assert_eq!(f.services.arena().stats().live_segments, 0);
}
#[test]
fn concurrent_calls_place_an_entry_once() {
    let f = Fixture::new();
    for id in 1..=16 {
        f.put(id, 2, id as f32);
    }
    let wanted = all(16);
    let (a, b) = std::thread::scope(|scope| {
        let first = scope.spawn(|| f.manager.ensure(&wanted).unwrap());
        let second = scope.spawn(|| f.manager.ensure(&wanted).unwrap());
        (first.join().unwrap(), second.join().unwrap())
    });
    assert_eq!(a.loaded.len() + b.loaded.len(), 16);
    let placed: usize = f.queue.drain().iter().map(|s| s.bindings.len()).sum();
    assert_eq!(placed, 16);
}
#[test]
fn scopes_resolve_through_the_registry() {
    let f = Fixture::new();
    f.put(1, 2, 1.0);
    f.put(2, 2, 2.0);
    let resolved = f.manager.resolve_scope(&Scope::Set(set(&[4, 5]))).unwrap();
    assert_eq!(resolved.len(), 2);
    let everything = f.manager.resolve_scope(&Scope::All).unwrap();
    assert_eq!(everything.len(), 2);
    let err = f.manager.resolve_scope(&Scope::Range { start: 1, end: 3 }).unwrap_err();
    assert!(matches!(err, GpuManagerError::Resolve(ResolveError::Unsupported { .. })));
}
#[test]
fn scope_resolution_needs_the_index() {
    let (_services, _queue, manager) = build(Arc::new(MissingIndex));
    let err = manager.resolve_scope(&Scope::All).unwrap_err();
    assert!(matches!(err, GpuManagerError::Index(_)));
}
