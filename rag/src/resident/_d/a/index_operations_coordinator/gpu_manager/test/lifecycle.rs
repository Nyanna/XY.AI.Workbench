use std::sync::atomic::Ordering;
use std::time::Duration;

use crate::resident::_d::a::index_operations_coordinator::eviction::EvictionPolicy;

use super::super::fixture::*;
use super::super::GpuServices;
use super::*;

fn ids(list: &[EntryId]) -> Vec<u64> {
    list.iter().map(|id| id.0).collect()
}

fn wanted(count: u64) -> EntrySet {
    let list: Vec<u64> = (1..=count).collect();
    set(&list)
}

#[test]
fn publish_assigns_ascending_ids_and_queues_the_binds() {
    let f = Fixture::new();
    for id in 1..=12 {
        f.put(id, 2, id as f32);
    }
    f.manager.ensure(&wanted(12)).unwrap();
    let published = f.publish_queued();
    assert_eq!(published, vec![SegmentId(0), SegmentId(1)]);
    assert_eq!(f.manager.segment_ids(), published);
    assert_eq!(f.manager.pending_updates(), 12);
    assert_eq!(f.apply(), 12);
    assert_eq!(f.manager.pending_updates(), 0);
    let services = &f.services;
    assert_eq!(services.manifests().current().generation(), 2);
    assert_eq!(services.manifests().current().len(), 2);
    assert_eq!(services.mapping_store().current().len(), 2);
    let table = services.table_swap().current_table().unwrap();
    assert_eq!(table.len(), 2);
    let bytes: u64 = table.segments().iter().map(|r| r.len()).sum();
    assert_eq!(f.manager.resident_bytes(), bytes);
    assert!(services.eviction().contains(EvictKey(0)));
    assert!(services.eviction().contains(EvictKey(1)));
}

#[test]
fn failed_publish_releases_the_segment_and_consumes_no_id() {
    let f = Fixture::new();
    for id in 1..=3 {
        f.put(id, 2, id as f32);
    }
    f.manager.ensure(&wanted(3)).unwrap();
    let seg = f.queue.drain().pop().unwrap();
    f.services.sink.fail.store(true, Ordering::SeqCst);
    let err = f.manager.publish_segment(seg).unwrap_err();
    assert!(matches!(err, GpuManagerError::Publish(TableSwapError::Write(_))));
    assert_eq!(f.services.arena().stats().live_segments, 0);
    assert_eq!(f.manager.segment_count(), 0);
    assert_eq!(f.manager.pending_updates(), 0);
    f.services.sink.fail.store(false, Ordering::SeqCst);
    let report = f.manager.ensure(&wanted(3)).unwrap();
    assert_eq!(report.loaded.len(), 3);
    assert_eq!(f.publish_queued(), vec![SegmentId(0)]);
}

#[test]
fn unknown_segments_cannot_be_released() {
    let f = Fixture::new();
    let err = f.manager.release_segment(SegmentId(7)).unwrap_err();
    assert_eq!(err, GpuManagerError::UnknownSegment(7));
}

#[test]
fn release_removes_the_segment_everywhere() {
    let f = Fixture::new();
    f.resident(3);
    let region = f.services.table_swap().current_table().unwrap().segments()[0];
    let released = f.manager.release_segment(SegmentId(0)).unwrap();
    assert_eq!(released.id, SegmentId(0));
    assert_eq!(released.region, region);
    assert_eq!(ids(&released.cleared), vec![1, 2, 3]);
    assert_eq!(f.manager.segment_count(), 0);
    assert!(f.manager.slot_of(EntryId(1)).is_none());
    assert!(f.services.mapping_store().current().get(SegmentId(0)).is_none());
    assert!(f.services.table_swap().current_table().unwrap().is_empty());
    assert_eq!(f.services.manifests().current().len(), 0);
    assert_eq!(f.services.manifests().current().generation(), 2);
    assert!(!f.services.eviction().contains(EvictKey(0)));
    assert_eq!(f.manager.resident_bytes(), 0);
    let gap = f.manager.run_gap();
    assert!(gap.reclaimed >= 1);
    assert_eq!(f.services.arena().stats().live_segments, 0);
}

#[test]
fn queued_updates_of_a_released_segment_fail_harmlessly() {
    let f = Fixture::new();
    for id in 1..=3 {
        f.put(id, 2, id as f32);
    }
    f.manager.ensure(&wanted(3)).unwrap();
    f.publish_queued();
    f.manager.release_segment(SegmentId(0)).unwrap();
    assert_eq!(f.apply(), 0);
    assert!(f.manager.slot_of(EntryId(1)).is_none());
    assert_eq!(f.manager.pending_updates(), 0);
}

#[test]
fn global_slots_of_released_segments_are_never_reused() {
    let f = Fixture::new();
    f.resident(SLOTS_PER_SEGMENT as u64);
    f.manager.release_segment(SegmentId(0)).unwrap();
    f.put(100, 2, 1.0);
    f.manager.ensure(&set(&[100])).unwrap();
    f.publish_queued();
    f.apply();
    assert_eq!(f.manager.global_slot_of(EntryId(100)), Some(SLOTS_PER_SEGMENT));
    assert_eq!(f.manager.segment_ids(), vec![SegmentId(1)]);
}

#[test]
fn idle_segments_are_evicted_and_cleaned_up() {
    let f = Fixture::new();
    f.resident(3);
    f.services
        .eviction()
        .set_policy(
            Tier2::Vram,
            EvictionPolicy { high: 0.9, low: 0.75, idle: Duration::from_secs(1) },
        );
    let quiet = f.manager.evict_step(Instant::now());
    assert!(quiet.decisions.is_empty());
    assert_eq!(f.manager.segment_count(), 1);
    let outcome = f.manager.evict_step(Instant::now() + Duration::from_secs(60));
    assert_eq!(outcome.decisions.len(), 1);
    assert_eq!(f.manager.segment_count(), 0);
    assert!(f.manager.slot_of(EntryId(1)).is_none());
    assert_eq!(f.services.manifests().current().len(), 0);
    assert!(f.services.table_swap().current_table().unwrap().is_empty());
    assert!(f.services.mapping_store().current().is_empty());
    f.manager.run_gap();
    assert_eq!(f.services.arena().stats().live_segments, 0);
}

#[test]
fn make_room_evicts_until_the_need_fits() {
    let f = Fixture::new();
    f.resident(3);
    let err = f.manager.make_room(u64::MAX / 2).unwrap_err();
    assert!(matches!(err, EvictionError::Exhausted { .. }));
    assert_eq!(f.manager.segment_count(), 1);
    let outcome = f.manager.make_room(1 << 20).unwrap();
    assert_eq!(outcome.decisions.len(), 1);
    assert_eq!(f.manager.segment_count(), 0);
    assert_eq!(f.services.manifests().current().len(), 0);
    assert!(f.manager.slot_of(EntryId(2)).is_none());
}

#[test]
fn make_room_without_need_changes_nothing() {
    let f = Fixture::new();
    f.resident(3);
    let outcome = f.manager.make_room(1).unwrap();
    assert!(outcome.decisions.is_empty());
    assert_eq!(f.manager.segment_count(), 1);
}
