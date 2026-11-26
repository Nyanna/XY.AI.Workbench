use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use super::super::fixture::*;
use super::super::GpuServices;
use super::*;

fn queued(f: &Fixture, count: u64) {
    for id in 1..=count {
        f.put(id, 2, id as f32);
    }
    let list: Vec<u64> = (1..=count).collect();
    f.manager.ensure(&set(&list)).unwrap();
    f.publish_queued();
}

#[test]
fn the_adapter_reports_and_applies_the_queue() {
    let f = Fixture::new();
    queued(&f, 3);
    let mut updates = f.manager.slot_updates();
    assert_eq!(updates.depth(), 3);
    assert!(updates.is_pending(EntryId(1)));
    assert!(!updates.is_pending(EntryId(9)));
    let applied = {
        let directory = lock(f.services.residency());
        updates.apply(&directory)
    };
    assert_eq!(applied, UpdateOutcome { applied: 3 });
    assert_eq!(updates.depth(), 0);
    assert!(f.manager.slot_of(EntryId(3)).is_some());
}

#[test]
fn applying_does_not_change_the_manifest_generation() {
    let f = Fixture::new();
    queued(&f, 3);
    let before = f.services.manifests().current().generation();
    f.apply();
    assert_eq!(f.services.manifests().current().generation(), before);
}

#[test]
fn the_snapshot_holds_the_current_and_the_previous_manifest() {
    let f = Fixture::new();
    let source = f.manager.mapping_source();
    f.resident(3);
    let first = source.snapshot();
    assert_eq!(first.current.generation(), 1);
    assert_eq!(first.previous.as_ref().map(|m| m.generation()), Some(0));
    f.put(100, 2, 1.0);
    f.manager.ensure(&set(&[100])).unwrap();
    f.manager.release_segment(SegmentId(0)).unwrap();
    let second = source.snapshot();
    assert_eq!(second.current.generation(), 2);
    assert_eq!(second.previous.as_ref().map(|m| m.generation()), Some(1));
}

#[test]
fn the_structure_lock_blocks_structure_changes() {
    let f = Fixture::new();
    f.resident(3);
    let source = f.manager.mapping_source();
    let finished = AtomicBool::new(false);
    std::thread::scope(|scope| {
        let guard = source.lock_structure();
        let handle = scope.spawn(|| {
            f.manager.release_segment(SegmentId(0)).unwrap();
            finished.store(true, Ordering::SeqCst);
        });
        std::thread::sleep(Duration::from_millis(150));
        assert!(!finished.load(Ordering::SeqCst));
        drop(guard);
        handle.join().unwrap();
    });
    assert!(finished.load(Ordering::SeqCst));
}

#[test]
fn deferred_regions_are_reclaimed_without_the_structure_lock() {
    let f = Fixture::new();
    f.resident(3);
    let source = f.manager.mapping_source();
    f.manager.release_segment(SegmentId(0)).unwrap();
    assert!(source.reclaim_deferred() >= 1);
    assert_eq!(f.services.arena().stats().live_segments, 0);
    assert_eq!(source.reclaim_deferred(), 0);
}

#[test]
fn the_directory_resolves_published_segments_only() {
    let f = Fixture::new();
    f.resident(3);
    let directory = f.manager.segment_directory();
    let binding = directory.binding(SegmentId(0)).unwrap();
    assert_eq!(
        binding.assignment.lookup(Slot(0)),
        Some(EntryRef { entry: EntryId(1), section: 0 })
    );
    assert!(binding.states.is_valid(Slot(0)));
    assert!(directory.binding(SegmentId(5)).is_none());
    f.manager.release_segment(SegmentId(0)).unwrap();
    assert!(directory.binding(SegmentId(0)).is_none());
}
