use super::super::fixture::*;
use super::super::GpuServices;
use super::*;

fn item(id: u64, tokens: usize, seed: f32, placed: Option<SlotRef>) -> MirrorItem {
    MirrorItem { id: EntryId(id), vector: vector(tokens, seed), meta: b"m".to_vec(), placed }
}

fn payload(bytes: &[u8]) -> PayloadRef<'_> {
    PayloadRef { vector: bytes, meta: b"" }
}

#[test]
fn invalid_vectors_fail_only_their_item() {
    let f = Fixture::new();
    let bad = MirrorItem { vector: vec![0u8; 5], ..item(1, 2, 1.0, None) };
    let report = f.manager.write_back(&[bad, item(2, 2, 2.0, None)]).unwrap();
    assert_eq!(
        report.failed,
        vec![(EntryId(1), GpuManagerError::InvalidVector { len: 5, row_bytes: 16 })]
    );
    assert_eq!(report.written, vec![(EntryId(2), f.stamp_of(2))]);
    assert!(f.index.stamp(EntryId(1)).is_none());
}

#[test]
fn resident_entries_get_the_stamp_of_their_write() {
    let f = Fixture::new();
    f.resident(2);
    let report = f.manager.write_back(&[item(1, 2, 70.0, None)]).unwrap();
    let stamp = report.written[0].1;
    assert_eq!(stamp, f.stamp_of(1));
    assert_eq!(lock(f.services.residency()).get(EntryId(1)).unwrap().stamp, stamp);
    assert_eq!(f.manager.pending_updates(), 0);
    let outcome = f.manager.reconcile_iteration().unwrap().unwrap();
    assert!(outcome.reconciliation.clean);
}

#[test]
fn placed_results_become_resident_with_the_stamp_of_their_write() {
    let f = Fixture::new();
    f.resident(2);
    let slot = f.fill_slot(100);
    let report = f.manager.write_back(&[item(100, 1, 3.0, Some(slot))]).unwrap();
    let stamp = report.written[0].1;
    let update = f.services.updates().get(EntryId(100)).unwrap();
    assert_eq!((update.kind, update.slot, update.stamp), (UpdateKind::Bind, slot, stamp));
    assert!(f.manager.slot_of(EntryId(100)).is_none());
    f.apply();
    assert_eq!(f.manager.slot_of(EntryId(100)), Some(slot));
    assert_eq!(lock(f.services.residency()).get(EntryId(100)).unwrap().stamp, stamp);
}

#[test]
fn non_resident_entries_without_a_slot_only_reach_the_index() {
    let f = Fixture::new();
    let report = f.manager.write_back(&[item(50, 2, 1.0, None)]).unwrap();
    assert_eq!(report.written.len(), 1);
    assert!(f.index.stamp(EntryId(50)).is_some());
    assert_eq!(f.manager.pending_updates(), 0);
    assert!(f.manager.slot_of(EntryId(50)).is_none());
}

#[test]
fn a_moved_result_hides_the_slot_it_replaces() {
    let f = Fixture::new();
    f.resident(2);
    let old = f.manager.slot_of(EntryId(1)).unwrap();
    let slot = f.fill_slot(1);
    assert_ne!(slot, old);
    f.manager.write_back(&[item(1, 1, 3.0, Some(slot))]).unwrap();
    f.apply();
    assert_eq!(f.manager.slot_of(EntryId(1)), Some(slot));
    assert_eq!(f.manager.run_gap().slots_hidden, 1);
    assert!(!f.is_valid(old));
    assert!(f.is_valid(slot));
}

#[test]
fn write_back_needs_the_index() {
    let (_services, _queue, manager) = build(Arc::new(MissingIndex));
    let err = manager.write_back(&[item(1, 2, 1.0, None)]).unwrap_err();
    assert!(matches!(err, GpuManagerError::Index(_)));
}

#[test]
fn the_mirrored_index_forwards_reads() {
    let f = Fixture::new();
    let stamp = f.put(1, 2, 1.0);
    let mirrored = f.manager.mirrored_index().unwrap();
    assert_eq!(mirrored.dtype(), DType::F32);
    assert_eq!(mirrored.stamp(EntryId(1)), Some(stamp));
    assert_eq!(mirrored.stamps(), f.index.stamps());
    assert_eq!(mirrored.read(EntryId(1)), f.index.read(EntryId(1)));
    assert_eq!(mirrored.delete(EntryId(1)).unwrap().is_some(), true);
    assert!(f.index.stamp(EntryId(1)).is_none());
}

#[test]
fn gpu_built_entries_become_resident_with_their_own_stamp() {
    let f = Fixture::new();
    f.resident(2);
    let slot = f.fill_slot(100);
    let mirrored = f.manager.mirrored_index().unwrap();
    let stamp = mirrored.write(EntryId(100), payload(&vector(1, 1.0))).unwrap();
    let update = f.services.updates().get(EntryId(100)).unwrap();
    assert_eq!((update.kind, update.slot, update.stamp), (UpdateKind::Bind, slot, stamp));
    f.apply();
    assert_eq!(f.manager.slot_of(EntryId(100)), Some(slot));
    assert_eq!(lock(f.services.residency()).get(EntryId(100)).unwrap().stamp, stamp);
    mirrored.write(EntryId(100), payload(&vector(1, 2.0))).unwrap();
    assert_eq!(f.manager.pending_updates(), 0);
}

#[test]
fn entries_without_a_slot_get_no_bind() {
    let f = Fixture::new();
    f.resident(2);
    let mirrored = f.manager.mirrored_index().unwrap();
    mirrored.write(EntryId(7), payload(&vector(1, 1.0))).unwrap();
    mirrored
        .write_many(&[(EntryId(8), payload(&vector(1, 1.0)))])
        .unwrap();
    assert_eq!(f.manager.pending_updates(), 0);
    assert!(f.index.stamp(EntryId(7)).is_some());
    assert!(f.index.stamp(EntryId(8)).is_some());
}

#[test]
fn write_many_binds_gpu_built_entries() {
    let f = Fixture::new();
    f.resident(2);
    let slot = f.fill_slot(100);
    let mirrored = f.manager.mirrored_index().unwrap();
    let written = mirrored
        .write_many(&[(EntryId(100), payload(&vector(1, 1.0)))])
        .unwrap();
    let update = f.services.updates().get(EntryId(100)).unwrap();
    assert_eq!((update.slot, update.stamp), (slot, written[0].1));
}

#[test]
fn a_busy_residency_mutex_defers_the_bind_to_the_apply() {
    let f = Fixture::new();
    f.resident(2);
    let slot = f.fill_slot(100);
    let mirrored = f.manager.mirrored_index().unwrap();
    let held = lock(f.services.residency());
    let stamp = mirrored.write(EntryId(100), payload(&vector(1, 1.0))).unwrap();
    assert_eq!(f.manager.pending_updates(), 0);
    let mut updates = f.manager.slot_updates();
    assert_eq!(updates.apply(&held).applied, 1);
    let resident = held.get(EntryId(100)).unwrap();
    assert_eq!((resident.slot, resident.stamp), (slot, stamp));
}
