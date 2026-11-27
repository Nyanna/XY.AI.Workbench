use super::*;
use std::sync::atomic::AtomicBool;
use std::thread;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::a::slot_layout::SlotClass;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::a::arenas::ArenaKind;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::_1::device_lane_event::{
    DeviceHandle, GpuEventBackend, LaneConfig, LaneKind, StreamBackend,
};
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::_1::layout::DType;
#[test]
fn manifest_types_are_send_sync() {
    fn check<T: Send + Sync>() {}
    check::<Manifest>();
    check::<Lease3>();
    check::<SegmentSpec>();
    check::<ChangeEvent>();
    check::<Subscriber>();
    check::<ManifestStore>();
    check::<ManifestError>();
    check::<SyncError3>();
}
fn region(offset: u64, len: u64, gen: u64) -> RegionHandle {
    RegionHandle::new(ArenaKind::IndexSegments, offset, len, Generation(gen)).unwrap()
}
fn spec(id: u64, offset: u64, len: u64, seg_gen: u64) -> SegmentSpec {
    SegmentSpec::variable(
        SegmentId(id),
        Generation(seg_gen),
        region(offset, len, 0),
        Addr(0x1000 + offset),
        4,
    )
}
fn store() -> ManifestStore {
    ManifestStore::new(StoreConfig::default())
}
fn next_of(store: &ManifestStore, specs: Vec<SegmentSpec>) -> Manifest {
    store.successor(&store.current(), specs).unwrap()
}
fn publish_specs(store: &ManifestStore, specs: Vec<SegmentSpec>) {
    let next = next_of(store, specs);
    drop(store.publish(next).unwrap());
}
#[derive(Debug)]
struct FakeEvent {
    ready: Arc<AtomicBool>,
}
impl GpuEventBackend for FakeEvent {
    fn query(&self) -> Result<bool, String> {
        Ok(self.ready.load(Ordering::SeqCst))
    }
    fn synchronize(&self) -> Result<(), String> {
        Ok(())
    }
}
#[derive(Debug)]
struct FakeStream {
    ready: Arc<AtomicBool>,
}
impl StreamBackend for FakeStream {
    fn record_event(&self) -> Result<Box<dyn GpuEventBackend>, String> {
        Ok(
            Box::new(FakeEvent {
                ready: Arc::clone(&self.ready),
            }),
        )
    }
}
fn copy_lane(ready: &Arc<AtomicBool>) -> Lane {
    let cfg = LaneConfig::preset(LaneKind::CopyH2D, DeviceHandle::Gpu(0));
    Lane::new(
        cfg,
        Some(
            Box::new(FakeStream {
                ready: Arc::clone(ready),
            }),
        ),
    )
}
#[test]
fn empty_manifest() {
    let m = Manifest::empty();
    assert_eq!(m.generation(), 0);
    assert!(m.is_empty());
    assert_eq!(m.len(), 0);
    assert_eq!(m.total_bytes(), 0);
    assert_eq!(m.lease_count(), 0);
    assert!(m.kernel_table().is_empty());
    assert_eq!(store().current().generation(), 0);
}
#[test]
fn display_texts() {
    let e = ManifestError::StaleGeneration {
        current: 3,
        got: 2,
    };
    assert_eq!(e.to_string(), "stale Generation");
    assert!(ManifestError::DuplicateSegment(SegmentId(9)).to_string().contains('9'));
    assert!(SyncError3 { cause : "boom".into() } .to_string().contains("boom"));
}
#[test]
fn successor_generation_and_order() {
    let s = store();
    let base = s.current();
    let m = s.successor(&base, vec![spec(2, 0, 100, 1), spec(1, 100, 50, 1)]).unwrap();
    assert_eq!(m.generation(), 1);
    assert_eq!(base.generation(), 0);
    assert!(base.is_empty());
    let ids: Vec<u64> = m.segments().iter().map(|x| x.id().0).collect();
    assert_eq!(ids, vec![2, 1]);
    assert_eq!(m.total_bytes(), 150);
}
#[test]
fn successor_rejects_duplicates_first_one() {
    let s = store();
    let err = s
        .successor(
            &s.current(),
            vec![
                spec(1, 0, 10, 0), spec(2, 10, 10, 0), spec(2, 20, 10, 0), spec(1, 30,
                10, 0),
            ],
        )
        .unwrap_err();
    assert_eq!(err, ManifestError::DuplicateSegment(SegmentId(2)));
}
#[test]
fn successor_overflow() {
    let s = store();
    let base = Manifest::from_parts(u64::MAX, Vec::new(), Vec::new());
    assert_eq!(
        s.successor(& base, Vec::new()).unwrap_err(), ManifestError::GenerationOverflow
    );
}
#[test]
fn total_bytes_saturates() {
    let s = store();
    let a = SegmentSpec::variable(
        SegmentId(1),
        Generation(0),
        region(0, u64::MAX / 2 + 10, 0),
        Addr(0),
        1,
    );
    let b = SegmentSpec::variable(
        SegmentId(2),
        Generation(0),
        region(0, u64::MAX / 2 + 10, 0),
        Addr(0),
        1,
    );
    let m = s.successor(&s.current(), vec![a, b]).unwrap();
    assert_eq!(m.total_bytes(), u64::MAX);
}
#[test]
fn kernel_table_layout_variable() {
    let s = store();
    let m = next_of(&s, vec![spec(1, 0, 64, 7), spec(2, 64, 64, 9)]);
    let t = m.kernel_table();
    assert_eq!(t.len(), 48);
    assert_eq!(u64::from_le_bytes(t[0..8].try_into().unwrap()), 7);
    assert_eq!(u64::from_le_bytes(t[8..16].try_into().unwrap()), 0x1000);
    assert_eq!(u32::from_le_bytes(t[16..20].try_into().unwrap()), 4);
    assert_eq!(u32::from_le_bytes(t[20..24].try_into().unwrap()), 0);
    assert_eq!(u64::from_le_bytes(t[24..32].try_into().unwrap()), 9);
    assert_eq!(u64::from_le_bytes(t[32..40].try_into().unwrap()), 0x1040);
}
#[test]
fn from_view_matches_descriptor() {
    let view = SegmentView::new(
            region(4096, 2048, 3),
            2,
            SlotClass::C32,
            4,
            DType::F32,
            Generation(5),
        )
        .unwrap();
    let base = Addr(0x8000);
    let sp = SegmentSpec::from_view(SegmentId(3), &view, base).unwrap();
    let d = view.descriptor(base).unwrap();
    assert_eq!(sp.generation(), Generation(5));
    assert_eq!(sp.region(), view.region());
    assert_eq!(sp.start(), d.start);
    assert_eq!(sp.slot_count(), d.slot_count);
    assert_eq!(sp.slot_stride(), d.stride);
    assert_eq!(sp.bytes(), view.total_bytes());
    assert_eq!(sp.bytes(), 1024);
    let s = store();
    let t = next_of(&s, vec![sp]).kernel_table();
    assert_eq!(u64::from_le_bytes(t[0..8].try_into().unwrap()), 5);
    assert_eq!(u64::from_le_bytes(t[8..16].try_into().unwrap()), 0x8000);
    assert_eq!(u32::from_le_bytes(t[16..20].try_into().unwrap()), 2);
    assert_eq!(u32::from_le_bytes(t[20..24].try_into().unwrap()), d.stride);
}
#[test]
fn from_view_address_overflow() {
    let view = SegmentView::new(
            region(0, 1024, 0),
            2,
            SlotClass::C32,
            4,
            DType::F32,
            Generation(0),
        )
        .unwrap();
    assert!(SegmentSpec::from_view(SegmentId(1), & view, Addr(u64::MAX)).is_err());
}
#[test]
fn variable_spec_fields() {
    let sp = SegmentSpec::variable(
        SegmentId(4),
        Generation(2),
        region(8, 77, 1),
        Addr(5),
        3,
    );
    assert_eq!(sp.slot_stride(), 0);
    assert_eq!(sp.bytes(), 77);
    assert_eq!(sp.slot_count(), 3);
    assert_eq!(sp.id(), SegmentId(4));
}
#[test]
fn publish_replaces_and_returns_previous() {
    let s = store();
    let m1 = next_of(&s, vec![spec(1, 0, 10, 0)]);
    let prev = s.publish(m1).unwrap();
    assert_eq!(prev.generation(), 0);
    assert_eq!(s.current().generation(), 1);
    let m2 = next_of(&s, vec![spec(1, 0, 10, 0), spec(2, 10, 10, 0)]);
    let prev = s.publish(m2).unwrap();
    assert_eq!(prev.generation(), 1);
    assert_eq!(s.current().len(), 2);
}
#[test]
fn publish_rejects_stale_and_equal() {
    let s = store();
    publish_specs(&s, vec![spec(1, 0, 10, 0)]);
    let again = Manifest::from_parts(1, Vec::new(), Vec::new());
    assert_eq!(
        s.publish(again).unwrap_err(), ManifestError::StaleGeneration { current : 1, got
        : 1 }
    );
    let older = Manifest::from_parts(0, Vec::new(), Vec::new());
    assert_eq!(
        s.publish(older).unwrap_err(), ManifestError::StaleGeneration { current : 1, got
        : 0 }
    );
    assert_eq!(s.current().len(), 1);
}
#[test]
fn stale_publish_does_not_notify() {
    let s = store();
    publish_specs(&s, vec![spec(1, 0, 10, 0)]);
    let sub = s.subscribe();
    let stale = Manifest::from_parts(1, vec![spec(5, 0, 1, 0)], Vec::new());
    assert!(s.publish(stale).is_err());
    assert!(sub.try_recv().is_none());
}
#[test]
fn publish_allows_generation_gap() {
    let s = store();
    let m = Manifest::from_parts(10, Vec::new(), Vec::new());
    assert!(s.publish(m).is_ok());
    assert_eq!(s.current().generation(), 10);
}
#[test]
fn injected_conflicts_cause_retries_but_succeed() {
    let s = store();
    s.inject_swap_conflicts(3);
    let m = next_of(&s, vec![spec(1, 0, 10, 0)]);
    assert!(s.publish(m).is_ok());
    assert!(s.swap_retries() >= 3);
    assert_eq!(s.current().generation(), 1);
}
#[test]
fn injected_conflicts_stay_correct_against_stale() {
    let s = store();
    publish_specs(&s, vec![spec(1, 0, 10, 0)]);
    s.inject_swap_conflicts(2);
    let stale = Manifest::from_parts(1, Vec::new(), Vec::new());
    assert!(
        matches!(s.publish(stale), Err(ManifestError::StaleGeneration { current : 1, got
        : 1 }))
    );
    let ok = next_of(&s, Vec::new());
    assert!(s.publish(ok).is_ok());
    assert!(s.swap_retries() >= 2);
}
#[test]
fn concurrent_publish_same_generation_one_wins() {
    let s = Arc::new(store());
    let base = s.current();
    let candidates: Vec<Manifest> = (0..8)
        .map(|i| s.successor(&base, vec![spec(i, i * 10, 10, 0)]).unwrap())
        .collect();
    let handles: Vec<_> = candidates
        .into_iter()
        .map(|m| {
            let s = Arc::clone(&s);
            thread::spawn(move || s.publish(m).is_ok())
        })
        .collect();
    let wins = handles.into_iter().map(|h| h.join().unwrap()).filter(|ok| *ok).count();
    assert_eq!(wins, 1);
    assert_eq!(s.current().generation(), 1);
}
#[test]
fn concurrent_readers_see_complete_manifests() {
    let s = Arc::new(store());
    let writer = {
        let s = Arc::clone(&s);
        thread::spawn(move || {
            for i in 0..200u64 {
                let specs = (0..=i % 5).map(|k| spec(k, k * 10, 10, i)).collect();
                let next = s.successor(&s.current(), specs).unwrap();
                drop(s.publish(next));
            }
        })
    };
    let readers: Vec<_> = (0..4)
        .map(|_| {
            let s = Arc::clone(&s);
            thread::spawn(move || {
                for _ in 0..500 {
                    let l = s.lease();
                    let m = l.manifest();
                    assert_eq!(m.kernel_table().len(), 24 * m.len());
                    assert_eq!(l.generation(), m.generation());
                }
            })
        })
        .collect();
    writer.join().unwrap();
    for r in readers {
        r.join().unwrap();
    }
}
#[test]
fn lease_counts_and_pins_manifest() {
    let s = store();
    publish_specs(&s, vec![spec(1, 0, 10, 0)]);
    let l1 = s.lease();
    let l2 = s.lease();
    assert_eq!(l1.generation(), 1);
    assert_eq!(s.current().lease_count(), 2);
    publish_specs(&s, vec![spec(2, 10, 10, 0)]);
    assert_eq!(l1.manifest().len(), 1);
    assert_eq!(l1.manifest().segments() [0].id(), SegmentId(1));
    assert_eq!(l1.manifest().lease_count(), 2);
    assert_eq!(s.current().lease_count(), 0);
    drop(l2);
    assert_eq!(l1.manifest().lease_count(), 1);
    let held = l1.manifest().clone();
    drop(l1);
    assert_eq!(held.lease_count(), 0);
}
#[test]
fn dropped_segment_without_markers_is_reclaimed() {
    let s = store();
    publish_specs(&s, vec![spec(1, 0, 10, 0), spec(2, 10, 10, 0)]);
    assert_eq!(s.retired_count(), 0);
    publish_specs(&s, vec![spec(1, 0, 10, 0)]);
    assert_eq!(s.retired_count(), 1);
    assert_eq!(s.reclaim(), vec![region(10, 10, 0)]);
    assert_eq!(s.retired_count(), 0);
    assert!(s.reclaim().is_empty());
}
#[test]
fn unchanged_segments_share_state_across_manifests() {
    let s = store();
    publish_specs(&s, vec![spec(1, 0, 10, 0)]);
    publish_specs(&s, vec![spec(1, 0, 10, 0), spec(2, 10, 10, 0)]);
    publish_specs(&s, vec![spec(1, 0, 10, 0), spec(2, 10, 10, 0)]);
    assert_eq!(s.retired_count(), 0);
    assert!(s.reclaim().is_empty());
}
#[test]
fn segment_generation_change_keeps_state_region_change_replaces_it() {
    let s = store();
    publish_specs(&s, vec![spec(1, 0, 10, 0)]);
    publish_specs(&s, vec![spec(1, 0, 10, 1)]);
    assert_eq!(s.retired_count(), 0);
    publish_specs(&s, vec![spec(1, 100, 10, 1)]);
    assert_eq!(s.reclaim(), vec![region(0, 10, 0)]);
}
#[test]
fn lease_and_held_previous_keep_segment() {
    let s = store();
    publish_specs(&s, vec![spec(1, 0, 10, 0)]);
    let lease = s.lease();
    let next = next_of(&s, Vec::new());
    let prev = s.publish(next).unwrap();
    drop(prev);
    assert_eq!(s.retired_count(), 0);
    drop(lease);
    assert_eq!(s.retired_count(), 1);
    publish_specs(&s, vec![spec(2, 50, 10, 0)]);
    let next = next_of(&s, Vec::new());
    let prev = s.publish(next).unwrap();
    assert_eq!(s.retired_count(), 1);
    drop(prev);
    assert_eq!(s.retired_count(), 2);
}
#[test]
fn pending_marker_holds_region_until_complete() {
    let s = store();
    publish_specs(&s, vec![spec(1, 0, 10, 0)]);
    let (marker, ctl) = Event::mock();
    let lease = s.lease();
    lease.register_completion(&marker);
    drop(lease);
    publish_specs(&s, Vec::new());
    assert_eq!(s.retired_count(), 1);
    assert!(s.reclaim().is_empty());
    assert_eq!(s.retired_count(), 1);
    ctl.complete();
    assert_eq!(s.reclaim(), vec![region(0, 10, 0)]);
    assert!(s.reclaim().is_empty());
}
#[test]
fn failed_marker_never_releases() {
    let s = store();
    publish_specs(&s, vec![spec(1, 0, 10, 0)]);
    let (marker, ctl) = Event::mock();
    s.lease().register_completion(&marker);
    ctl.fail("device lost");
    publish_specs(&s, Vec::new());
    assert!(s.reclaim().is_empty());
    assert_eq!(s.retired_count(), 1);
}
#[test]
fn completed_marker_is_not_stored() {
    let s = store();
    publish_specs(&s, vec![spec(1, 0, 10, 0)]);
    let (marker, ctl) = Event::mock();
    ctl.complete();
    s.lease().register_completion(&marker);
    ctl.reset();
    publish_specs(&s, Vec::new());
    assert_eq!(s.reclaim().len(), 1);
}
#[test]
fn marker_registered_on_all_segments_and_repeatable() {
    let s = store();
    publish_specs(&s, vec![spec(1, 0, 10, 0), spec(2, 10, 10, 0)]);
    let (marker, ctl) = Event::mock();
    let lease = s.lease();
    lease.register_completion(&marker);
    lease.register_completion(&marker);
    drop(lease);
    publish_specs(&s, Vec::new());
    assert_eq!(s.retired_count(), 2);
    assert!(s.reclaim().is_empty());
    ctl.complete();
    assert_eq!(s.reclaim(), vec![region(0, 10, 0), region(10, 10, 0)]);
}
#[test]
fn reclaim_keeps_blocked_entries_and_orders_by_retirement() {
    let s = store();
    publish_specs(&s, vec![spec(1, 0, 10, 0), spec(2, 10, 10, 0), spec(3, 20, 10, 0)]);
    let (marker, ctl) = Event::mock();
    let lease = s.lease();
    lease.register_completion(&marker);
    drop(lease);
    publish_specs(&s, Vec::new());
    assert!(s.reclaim().is_empty());
    publish_specs(&s, vec![spec(4, 30, 10, 0)]);
    publish_specs(&s, Vec::new());
    assert_eq!(s.reclaim(), vec![region(30, 10, 0)]);
    ctl.complete();
    assert_eq!(
        s.reclaim(), vec![region(0, 10, 0), region(10, 10, 0), region(20, 10, 0)]
    );
}
#[test]
fn many_markers_are_pruned_but_pending_ones_kept() {
    let s = store();
    publish_specs(&s, vec![spec(1, 0, 10, 0)]);
    let (pending, pctl) = Event::mock();
    let lease = s.lease();
    lease.register_completion(&pending);
    for _ in 0..500 {
        let (m, c) = Event::mock();
        lease.register_completion(&m);
        c.complete();
    }
    drop(lease);
    publish_specs(&s, Vec::new());
    assert!(s.reclaim().is_empty());
    pctl.complete();
    assert_eq!(s.reclaim().len(), 1);
}
#[test]
fn concurrent_reclaim_delivers_each_region_once() {
    let s = Arc::new(store());
    let specs: Vec<SegmentSpec> = (0..50).map(|i| spec(i, i * 10, 10, 0)).collect();
    publish_specs(&s, specs);
    publish_specs(&s, Vec::new());
    assert_eq!(s.retired_count(), 50);
    let handles: Vec<_> = (0..4)
        .map(|_| {
            let s = Arc::clone(&s);
            thread::spawn(move || s.reclaim())
        })
        .collect();
    let mut all: Vec<RegionHandle> = handles
        .into_iter()
        .flat_map(|h| h.join().unwrap())
        .collect();
    assert_eq!(all.len(), 50);
    all.sort();
    all.dedup();
    assert_eq!(all.len(), 50);
}
#[test]
fn discarded_unpublished_manifest_returns_new_regions() {
    let s = store();
    publish_specs(&s, vec![spec(1, 0, 10, 0)]);
    let candidate = next_of(&s, vec![spec(1, 0, 10, 0), spec(2, 10, 10, 0)]);
    publish_specs(&s, Vec::new());
    assert!(s.publish(candidate.clone()).is_err());
    drop(candidate);
    let got = s.reclaim();
    assert!(got.contains(& region(10, 10, 0)));
}
#[test]
fn stage_publishes_in_order_when_complete() {
    let s = store();
    let (e1, c1) = Event::mock();
    let (e2, c2) = Event::mock();
    s.stage(next_of(&s, vec![spec(1, 0, 10, 0)]), e1);
    let m2 = Manifest::from_parts(
        2,
        vec![spec(1, 0, 10, 0), spec(2, 10, 10, 0)],
        Vec::new(),
    );
    s.stage(m2, e2);
    assert_eq!(s.staged_count(), 2);
    assert_eq!(s.publish_ready(), 0);
    c2.complete();
    assert_eq!(s.publish_ready(), 0);
    assert_eq!(s.staged_count(), 2);
    assert_eq!(s.current().generation(), 0);
    c1.complete();
    assert_eq!(s.publish_ready(), 2);
    assert_eq!(s.staged_count(), 0);
    assert_eq!(s.current().generation(), 2);
    assert_eq!(s.publish_ready(), 0);
}
#[test]
fn publish_ready_stops_on_failed_event_and_retries() {
    let s = store();
    let (e1, c1) = Event::mock();
    s.stage(next_of(&s, vec![spec(1, 0, 10, 0)]), e1);
    c1.fail("copy failed");
    assert_eq!(s.publish_ready(), 0);
    assert_eq!(s.staged_count(), 1);
    c1.reset();
    c1.complete();
    assert_eq!(s.publish_ready(), 1);
}
#[test]
fn publish_ready_drops_stale_and_continues() {
    let s = store();
    let (e1, _c1) = Event::mock();
    let (e2, _c2) = Event::mock();
    let stale = Manifest::from_parts(0, Vec::new(), Vec::new());
    s.stage(stale, e1.clone());
    s.stage(next_of(&s, vec![spec(1, 0, 10, 0)]), e2.clone());
    let (done, ctl) = Event::mock();
    ctl.complete();
    let s2 = store();
    s2.stage(Manifest::from_parts(0, Vec::new(), Vec::new()), done.clone());
    s2.stage(next_of(&s2, vec![spec(1, 0, 10, 0)]), done);
    assert_eq!(s2.publish_ready(), 1);
    assert_eq!(s2.staged_count(), 0);
    assert_eq!(s2.current().generation(), 1);
    assert_eq!(s.staged_count(), 2);
}
#[test]
fn publish_ready_releases_previous_segments() {
    let s = store();
    publish_specs(&s, vec![spec(1, 0, 10, 0)]);
    let (e, c) = Event::mock();
    c.complete();
    s.stage(next_of(&s, Vec::new()), e);
    assert_eq!(s.publish_ready(), 1);
    assert_eq!(s.reclaim(), vec![region(0, 10, 0)]);
}
#[test]
fn stage_copy_enqueues_records_and_stages() {
    let s = store();
    let ready = Arc::new(AtomicBool::new(false));
    let mut lane = copy_lane(&ready);
    let mut called = false;
    s.stage_copy(
            &mut lane,
            |_| {
                called = true;
                Ok(())
            },
            next_of(&s, vec![spec(1, 0, 10, 0)]),
        )
        .unwrap();
    assert!(called);
    assert_eq!(s.staged_count(), 1);
    assert_eq!(s.publish_ready(), 0);
    ready.store(true, Ordering::SeqCst);
    assert_eq!(s.publish_ready(), 1);
    assert_eq!(s.current().generation(), 1);
}
#[test]
fn stage_copy_enqueue_error_stages_nothing() {
    let s = store();
    let ready = Arc::new(AtomicBool::new(true));
    let mut lane = copy_lane(&ready);
    let err = s
        .stage_copy(&mut lane, |_| Err("no space".to_string()), next_of(&s, Vec::new()))
        .unwrap_err();
    assert_eq!(err.cause, "no space");
    assert_eq!(s.staged_count(), 0);
}
#[test]
fn stage_copy_record_error_stages_nothing() {
    let s = store();
    let cfg = LaneConfig::preset(LaneKind::CopyH2D, DeviceHandle::Cpu);
    let mut lane = Lane::new(cfg, None);
    let err = s.stage_copy(&mut lane, |_| Ok(()), next_of(&s, Vec::new())).unwrap_err();
    assert!(! err.cause.is_empty());
    assert_eq!(s.staged_count(), 0);
}
#[test]
fn notifications_added_changed_unchanged() {
    let s = store();
    let sub = s.subscribe();
    publish_specs(&s, vec![spec(1, 0, 10, 0), spec(2, 10, 10, 0)]);
    let ev: Vec<ChangeEvent> = std::iter::from_fn(|| sub.try_recv()).collect();
    assert_eq!(ev.len(), 2);
    assert_eq!(ev[0].kind, ChangeKind::Added);
    assert_eq!(ev[0].segment_id, SegmentId(1));
    assert_eq!(ev[1].segment_id, SegmentId(2));
    assert!(ev.iter().all(| e | e.manifest_generation == 1));
    publish_specs(&s, vec![spec(1, 0, 10, 0), spec(2, 10, 10, 3), spec(3, 20, 10, 0)]);
    let ev: Vec<ChangeEvent> = std::iter::from_fn(|| sub.try_recv()).collect();
    assert_eq!(
        ev, vec![ChangeEvent { kind : ChangeKind::Changed, segment_id : SegmentId(2),
        segment_generation : Generation(3), manifest_generation : 2, }, ChangeEvent {
        kind : ChangeKind::Added, segment_id : SegmentId(3), segment_generation :
        Generation(0), manifest_generation : 2, },]
    );
    publish_specs(&s, vec![spec(3, 20, 10, 0)]);
    assert!(sub.try_recv().is_none());
}
#[test]
fn notifications_use_actual_predecessor_across_gap() {
    let s = store();
    publish_specs(&s, vec![spec(1, 0, 10, 0)]);
    let sub = s.subscribe();
    let m = Manifest::from_parts(
        9,
        vec![spec(1, 0, 10, 0), spec(2, 10, 10, 0)],
        Vec::new(),
    );
    s.publish(m).unwrap();
    let e = sub.try_recv().unwrap();
    assert_eq!(e.segment_id, SegmentId(2));
    assert_eq!(e.manifest_generation, 9);
    assert!(sub.try_recv().is_none());
}
#[test]
fn subscriber_only_sees_later_publications() {
    let s = store();
    publish_specs(&s, vec![spec(1, 0, 10, 0)]);
    let sub = s.subscribe();
    assert!(sub.try_recv().is_none());
    assert!(sub.recv_timeout(Duration::from_millis(5)).is_none());
    publish_specs(&s, vec![spec(1, 0, 10, 0), spec(2, 10, 10, 0)]);
    assert!(sub.recv_timeout(Duration::from_millis(100)).is_some());
}
#[test]
fn full_buffer_drops_events_only_for_that_subscriber() {
    let s = ManifestStore::new(StoreConfig {
        subscriber_buffer: 2,
    });
    let slow = s.subscribe();
    let specs: Vec<SegmentSpec> = (0..5).map(|i| spec(i, i * 10, 10, 0)).collect();
    let wide = ManifestStore::new(StoreConfig {
        subscriber_buffer: 16,
    });
    let fast = wide.subscribe();
    publish_specs(&s, specs.clone());
    publish_specs(&wide, specs);
    assert_eq!(slow.dropped(), 3);
    assert_eq!(fast.dropped(), 0);
    assert_eq!(std::iter::from_fn(|| slow.try_recv()).count(), 2);
    assert_eq!(std::iter::from_fn(|| fast.try_recv()).count(), 5);
}
#[test]
fn mixed_subscribers_are_independent() {
    let s = ManifestStore::new(StoreConfig {
        subscriber_buffer: 1,
    });
    let a = s.subscribe();
    let b = s.subscribe();
    publish_specs(&s, vec![spec(1, 0, 10, 0), spec(2, 10, 10, 0)]);
    assert_eq!(a.dropped(), 1);
    assert_eq!(b.dropped(), 1);
    assert!(a.try_recv().is_some());
    publish_specs(&s, vec![spec(1, 0, 10, 0), spec(2, 10, 10, 0), spec(3, 20, 10, 0)]);
    assert_eq!(a.dropped(), 1);
    assert_eq!(b.dropped(), 2);
}
#[test]
fn zero_buffer_behaves_like_one() {
    let s = ManifestStore::new(StoreConfig {
        subscriber_buffer: 0,
    });
    let sub = s.subscribe();
    publish_specs(&s, vec![spec(1, 0, 10, 0)]);
    assert!(sub.try_recv().is_some());
}
#[test]
fn dropped_subscriber_is_removed_silently() {
    let s = store();
    let a = s.subscribe();
    let b = s.subscribe();
    drop(a);
    publish_specs(&s, vec![spec(1, 0, 10, 0)]);
    assert!(b.try_recv().is_some());
    assert_eq!(lock(& s.subscribers).len(), 1);
}
#[test]
fn publish_without_subscribers_succeeds() {
    let s = store();
    publish_specs(&s, vec![spec(1, 0, 10, 0)]);
    assert_eq!(s.current().generation(), 1);
}
#[test]
fn poisoned_locks_are_recovered() {
    let s = Arc::new(store());
    let s2 = Arc::clone(&s);
    let _ = thread::spawn(move || {
            let _g = s2.retire.lock().unwrap();
            let _g2 = s2.staged.lock().unwrap();
            panic!("poison");
        })
        .join();
    assert_eq!(s.staged_count(), 0);
    assert_eq!(s.retired_count(), 0);
    publish_specs(&s, vec![spec(1, 0, 10, 0)]);
    publish_specs(&s, Vec::new());
    assert_eq!(s.reclaim().len(), 1);
}
