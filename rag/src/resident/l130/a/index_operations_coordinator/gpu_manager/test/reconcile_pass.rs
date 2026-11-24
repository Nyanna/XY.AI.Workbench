use std::sync::atomic::Ordering;
use std::sync::Arc;
use super::super::fixture::*;
use super::super::GpuServices;
use super::*;
#[test]
fn empty_directory_gives_a_clean_reconciliation() {
    let f = Fixture::new();
    let outcome = f.manager.reconcile_iteration().unwrap().unwrap();
    assert!(outcome.reconciliation.clean);
    assert_eq!(
        (outcome.queued_unbinds, outcome.reloaded, outcome.skipped_pending, outcome
        .failed), (0, 0, 0, 0)
    );
}
#[test]
fn signal_follows_the_index() {
    let f = Fixture::new();
    assert!(! f.manager.signal_pending());
    f.manager.reconcile_iteration().unwrap();
    assert!(! f.manager.signal_pending());
    f.put(1, 2, 1.0);
    assert!(f.manager.signal_pending());
    f.manager.reconcile_iteration().unwrap();
    assert!(! f.manager.signal_pending());
}
#[test]
fn signal_is_pending_while_a_reconcile_runs() {
    let f = Fixture::new();
    f.manager.reconcile_iteration().unwrap();
    let _running = lock(&f.manager.inner.reconciler);
    assert!(f.manager.signal_pending());
}
#[test]
fn deleted_entries_are_unbound_and_hidden_in_the_gap() {
    let f = Fixture::new();
    f.resident(3);
    let old = f.manager.slot_of(EntryId(2)).unwrap();
    f.index.delete(EntryId(2)).unwrap();
    let outcome = f.manager.reconcile_iteration().unwrap().unwrap();
    assert_eq!(outcome.reconciliation.removed, vec![EntryId(2)]);
    assert_eq!(outcome.queued_unbinds, 1);
    assert_eq!(f.manager.pending_updates(), 1);
    assert_eq!(f.manager.slot_of(EntryId(2)), Some(old));
    assert_eq!(f.manager.run_gap().slots_hidden, 0);
    assert!(f.is_valid(old));
    f.apply();
    assert!(f.manager.slot_of(EntryId(2)).is_none());
    assert!(f.is_valid(old));
    assert_eq!(f.manager.run_gap().slots_hidden, 1);
    assert!(! f.is_valid(old));
    assert!(f.manager.slot_of(EntryId(1)).is_some());
}
#[test]
fn changed_entries_move_to_a_new_slot() {
    let f = Fixture::new();
    f.resident(3);
    let old = f.manager.slot_of(EntryId(1)).unwrap();
    let changed = f.put(1, 2, 100.0);
    let outcome = f.manager.reconcile_iteration().unwrap().unwrap();
    assert_eq!(outcome.reconciliation.reload, vec![EntryId(1)]);
    assert_eq!(outcome.reloaded, 1);
    assert_eq!(f.manager.pending_updates(), 1);
    assert!(! f.is_valid(old));
    f.apply();
    let now = f.manager.slot_of(EntryId(1)).unwrap();
    assert_ne!(now, old);
    assert_eq!(f.device_bytes(now), vector(2, 100.0));
    assert_eq!(lock(f.services.residency()).get(EntryId(1)).unwrap().stamp, changed);
    assert_eq!(f.manager.entry_at_global_slot(0), None);
    let again = f.manager.reconcile_iteration().unwrap().unwrap();
    assert!(again.reconciliation.clean);
}
#[test]
fn reload_into_another_segment_hides_the_old_slot_in_the_gap() {
    let f = Fixture::new();
    f.resident(SLOTS_PER_SEGMENT as u64);
    let old = f.manager.slot_of(EntryId(1)).unwrap();
    f.put(1, 2, 100.0);
    let outcome = f.manager.reconcile_iteration().unwrap().unwrap();
    assert_eq!(outcome.reloaded, 1);
    assert_eq!(f.queue.len(), 1);
    f.publish_queued();
    f.apply();
    let now = f.manager.slot_of(EntryId(1)).unwrap();
    assert_eq!(now.segment, SegmentId(1));
    assert!(f.is_valid(old));
    assert_eq!(f.manager.run_gap().slots_hidden, 1);
    assert!(! f.is_valid(old));
    assert_eq!(f.device_bytes(now), vector(2, 100.0));
}
#[test]
fn entries_with_a_pending_update_are_skipped() {
    let f = Fixture::new();
    f.resident(2);
    f.put(1, 2, 50.0);
    assert_eq!(f.manager.reconcile_iteration().unwrap().unwrap().reloaded, 1);
    f.put(1, 2, 60.0);
    let outcome = f.manager.reconcile_iteration().unwrap().unwrap();
    assert_eq!(outcome.skipped_pending, 1);
    assert_eq!(outcome.reloaded, 0);
    assert_eq!(f.manager.pending_updates(), 1);
}
#[test]
fn failed_reloads_leave_the_directory_unchanged() {
    let f = Fixture::new();
    f.resident(2);
    let old = f.manager.slot_of(EntryId(1)).unwrap();
    f.put(1, 2, 50.0);
    f.services.fail_visible.store(true, Ordering::SeqCst);
    let outcome = f.manager.reconcile_iteration().unwrap().unwrap();
    assert_eq!((outcome.failed, outcome.reloaded), (1, 0));
    assert_eq!(f.manager.slot_of(EntryId(1)), Some(old));
    assert_eq!(f.manager.pending_updates(), 0);
    f.services.fail_visible.store(false, Ordering::SeqCst);
    let outcome = f.manager.reconcile_iteration().unwrap().unwrap();
    assert_eq!((outcome.failed, outcome.reloaded), (0, 1));
}
#[test]
fn deleting_a_changed_entry_replaces_its_pending_bind() {
    let f = Fixture::new();
    f.resident(2);
    f.put(1, 2, 50.0);
    f.manager.reconcile_iteration().unwrap();
    f.index.delete(EntryId(1)).unwrap();
    let outcome = f.manager.reconcile_iteration().unwrap().unwrap();
    assert_eq!(outcome.queued_unbinds, 1);
    assert_eq!(f.manager.pending_updates(), 1);
    let update = f.services.updates().get(EntryId(1)).unwrap();
    assert_eq!(update.kind, UpdateKind::Unbind);
}
#[test]
fn the_iteration_never_loads_non_resident_entries() {
    let f = Fixture::new();
    for id in 1..=5 {
        f.put(id, 2, id as f32);
    }
    let before = f.index.stamps();
    let outcome = f.manager.reconcile_iteration().unwrap().unwrap();
    assert!(outcome.reconciliation.clean);
    assert!(f.queue.is_empty());
    assert_eq!(f.index.stamps(), before);
    assert_eq!(f.manager.pending_updates(), 0);
    assert_eq!(f.manager.segment_count(), 0);
}
#[test]
fn vanished_index_ends_the_iteration() {
    let (_services, _queue, manager) = build(Arc::new(FreshIndex));
    assert!(manager.reconcile_iteration().unwrap().is_some());
    assert!(manager.reconcile_iteration().unwrap().is_none());
}
#[test]
fn unavailable_index_is_an_error() {
    let (_services, _queue, manager) = build(Arc::new(MissingIndex));
    let err = manager.reconcile_iteration().unwrap_err();
    assert!(matches!(err, GpuManagerError::Index(_)));
}
