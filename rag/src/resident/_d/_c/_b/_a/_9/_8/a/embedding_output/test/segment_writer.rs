use super::*;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::a::slot_layout::SlotClass;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::a::region_handle::{
    Generation, RegionHandle,
};
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::a::arenas::ArenaKind;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::a::index_contract::EntryId;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::_1::layout::DType;
use std::sync::Arc;
#[test]
fn segment_writer_is_send_sync() {
    fn check<T: Send + Sync>() {}
    check::<SegmentWriter>();
}
fn region(len: u64) -> RegionHandle {
    RegionHandle::new(ArenaKind::IndexSegments, 4096, len, Generation(1))
        .expect("region")
}
fn view_with(slot_count: u32, class: SlotClass, dim: u32, dtype: DType) -> SegmentView {
    SegmentView::new(region(1 << 22), slot_count, class, dim, dtype, Generation(2))
        .expect("view")
}
fn view(slot_count: u32) -> SegmentView {
    view_with(slot_count, SlotClass::C32, 4, DType::F32)
}
fn writer(slot_count: u32) -> SegmentWriter {
    SegmentWriter::new(view(slot_count), Addr(1 << 20)).expect("writer")
}
fn eref(id: u64) -> EntryRef {
    EntryRef {
        entry: EntryId(id),
        section: 0,
    }
}
fn done() -> Event {
    let (ev, ctl) = Event::mock();
    ctl.complete();
    ev
}
fn insert(w: &SegmentWriter, id: u64, tokens: u32) -> Slot5 {
    let res = w.reserve(eref(id), tokens, None).expect("reserve");
    w.commit(&res, &done()).expect("commit");
    res.slot
}
#[test]
fn new_writer_is_empty() {
    let w = writer(4);
    assert_eq!(w.used_slots(), 0);
    assert_eq!(w.free_slots(), 4);
    assert_eq!(w.pending_count(), 0);
    assert_eq!(w.states().valid_count(), 0);
    assert_eq!(w.view().slot_count(), 4);
}
#[test]
fn from_parts_validates_arguments() {
    let v = view(4);
    let states = Arc::new(SlotStates::new(&v));
    let assignment = Arc::new(SlotAssignment::fixed(&v, Addr(0)).unwrap());
    let err = SegmentWriter::from_parts(v, states.clone(), assignment.clone(), 5, 1)
        .err();
    assert_eq!(
        err, Some(WriteError::Slot(SlotError::SlotOutOfRange { slot : 5, count : 4 }))
    );
    assert!(
        matches!(SegmentWriter::from_parts(v, states.clone(), assignment.clone(), 0, 0),
        Err(WriteError::Slot(_)))
    );
    let other = view(8);
    let other_states = Arc::new(SlotStates::new(&other));
    assert!(
        matches!(SegmentWriter::from_parts(v, other_states, assignment.clone(), 0, 1),
        Err(WriteError::Slot(SlotError::SlotOutOfRange { .. })))
    );
    assert!(SegmentWriter::from_parts(v, states, assignment, 4, 1).is_ok());
}
#[test]
fn from_parts_keeps_existing_state_and_cursor() {
    let v = view(4);
    let states = Arc::new(SlotStates::new(&v));
    let assignment = Arc::new(SlotAssignment::fixed(&v, Addr(0)).unwrap());
    assignment.bind(Slot5(0), eref(1)).unwrap();
    states.set_filled(Slot5(0), 3).unwrap();
    let w = SegmentWriter::from_parts(v, states, assignment, 2, 10).unwrap();
    assert_eq!(w.used_slots(), 2);
    assert_eq!(w.free_slots(), 2);
    assert!(w.states().is_valid(Slot5(0)));
    let res = w.reserve(eref(2), 1, None).unwrap();
    assert_eq!(res.slot, Slot5(2));
    w.commit(&res, &done()).unwrap();
    assert_eq!(w.slot_info(Slot5(2)).unwrap().write_no, 10);
    assert_eq!(w.slot_info(Slot5(0)).unwrap().write_no, 0);
}
#[test]
fn reserve_assigns_ascending_unique_slots_and_stays_invisible() {
    let w = writer(3);
    let a = w.reserve(eref(1), 2, Some(CentroidId(7))).unwrap();
    let b = w.reserve(eref(2), 0, None).unwrap();
    assert_eq!((a.slot, b.slot), (Slot5(0), Slot5(1)));
    assert_eq!(a.tokens, 2);
    assert_eq!(a.centroid, Some(CentroidId(7)));
    let asg = w.assignment();
    assert_eq!(a.addr, asg.device_addr(Slot5(0)).unwrap());
    assert_eq!(a.target, asg.byte_range(Slot5(0)).unwrap());
    assert_eq!(a.target.end - a.target.start, u64::from(w.view().slot_bytes()));
    assert!(! w.states().is_valid(a.slot));
    assert_eq!(asg.lookup(a.slot), None);
    assert_eq!(asg.slot_of_entry(eref(1)), None);
    assert_eq!(w.used_slots(), 2);
    assert_eq!(w.free_slots(), 1);
    assert_eq!(w.pending_count(), 2);
    assert_eq!(w.resolve(a.slot), None);
}
#[test]
fn reserve_error_order_and_no_side_effects() {
    let w = writer(1);
    assert_eq!(
        w.reserve(eref(1), 70_000, Some(CentroidId(u32::MAX))).err(),
        Some(WriteError::InvalidCentroid)
    );
    assert_eq!(
        w.reserve(eref(1), 65_536, None).err(), Some(WriteError::TokensOutOfRange {
        tokens : 65_536 })
    );
    assert_eq!(
        w.reserve(eref(1), 33, None).err(), Some(WriteError::TokensExceedClass { tokens :
        33, max : 32 })
    );
    assert_eq!(w.used_slots(), 0);
    assert_eq!(w.pending_count(), 0);
    w.reserve(eref(1), 32, None).unwrap();
    assert_eq!(
        w.reserve(eref(2), 33, None).err(), Some(WriteError::TokensExceedClass { tokens :
        33, max : 32 })
    );
    assert_eq!(w.reserve(eref(2), 1, None).err(), Some(WriteError::NoFreeSlot));
    assert_eq!(w.used_slots(), 1);
}
#[test]
fn commit_makes_slot_visible_with_metadata() {
    let w = writer(2);
    let res = w.reserve(eref(1), 5, Some(CentroidId(3))).unwrap();
    w.commit(&res, &done()).unwrap();
    assert!(w.states().is_valid(res.slot));
    assert_eq!(w.assignment().lookup(res.slot), Some(eref(1)));
    assert_eq!(w.assignment().slot_of_entry(eref(1)), Some(res.slot));
    assert_eq!(w.resolve(res.slot), Some(eref(1)));
    assert_eq!(
        w.slot_info(res.slot).unwrap(), SlotInfo { valid : true, tokens : 5, write_no :
        1, centroid : Some(CentroidId(3)), }
    );
    assert_eq!(w.pending_count(), 0);
    assert_eq!(
        w.commit(& res, & done()).err(), Some(WriteError::NotReserved { slot : res.slot
        })
    );
}
#[test]
fn commit_not_ready_and_event_error_keep_reservation() {
    let w = writer(2);
    let res = w.reserve(eref(1), 1, None).unwrap();
    let (ev, ctl) = Event::mock();
    assert_eq!(w.commit(& res, & ev).err(), Some(WriteError::NotReady));
    assert!(! w.states().is_valid(res.slot));
    ctl.fail("boom");
    assert!(matches!(w.commit(& res, & ev), Err(WriteError::Event(_))));
    assert_eq!(w.pending_count(), 1);
    ctl.reset();
    ctl.complete();
    w.commit(&res, &ev).unwrap();
    assert!(w.states().is_valid(res.slot));
}
#[test]
fn foreign_reservation_is_rejected() {
    let a = writer(2);
    let b = writer(2);
    let res = a.reserve(eref(1), 1, None).unwrap();
    assert_eq!(b.commit(& res, & done()).err(), Some(WriteError::ForeignReservation));
    assert_eq!(
        b.commit_deferred(& res, done()).err(), Some(WriteError::ForeignReservation)
    );
    assert_eq!(b.pending_count(), 0);
    a.commit(&res, &done()).unwrap();
}
#[test]
fn update_hides_old_slot_but_keeps_it_resolvable() {
    let w = writer(3);
    let old = insert(&w, 1, 4);
    let res = w.reserve(eref(1), 6, None).unwrap();
    assert!(w.states().is_valid(old));
    w.commit(&res, &done()).unwrap();
    assert!(! w.states().is_valid(old));
    assert!(w.states().is_valid(res.slot));
    assert_eq!(w.assignment().slot_of_entry(eref(1)), Some(res.slot));
    assert_eq!(w.assignment().lookup(old), None);
    assert_eq!(w.resolve(old), Some(eref(1)));
    assert_eq!(w.resolve(res.slot), Some(eref(1)));
    let info = w.slot_info(old).unwrap();
    assert!(! info.valid);
    assert_eq!(info.tokens, 0);
    assert_eq!(info.write_no, 1);
    assert_eq!(w.slot_info(res.slot).unwrap().write_no, 2);
    assert_eq!(w.states().valid_count(), 1);
}
#[test]
fn write_numbers_follow_application_order() {
    let w = writer(3);
    let a = w.reserve(eref(1), 1, None).unwrap();
    let b = w.reserve(eref(2), 1, None).unwrap();
    w.commit(&b, &done()).unwrap();
    w.commit(&a, &done()).unwrap();
    assert_eq!(w.slot_info(b.slot).unwrap().write_no, 1);
    assert_eq!(w.slot_info(a.slot).unwrap().write_no, 2);
}
#[test]
fn deferred_commit_needs_drain_and_completed_event() {
    let w = writer(2);
    let res = w.reserve(eref(1), 2, None).unwrap();
    let (ev, ctl) = Event::mock();
    w.commit_deferred(&res, ev).unwrap();
    assert_eq!(w.pending_count(), 1);
    assert_eq!(w.drain(), 0);
    assert!(! w.states().is_valid(res.slot));
    assert_eq!(
        w.commit(& res, & done()).err(), Some(WriteError::NotReserved { slot : res.slot
        })
    );
    assert_eq!(
        w.commit_deferred(& res, done()).err(), Some(WriteError::NotReserved { slot : res
        .slot })
    );
    ctl.complete();
    assert_eq!(w.drain(), 1);
    assert!(w.states().is_valid(res.slot));
    assert_eq!(w.pending_count(), 0);
    assert_eq!(w.drain(), 0);
}
#[test]
fn drain_stops_at_incomplete_head() {
    let w = writer(3);
    let a = w.reserve(eref(1), 1, None).unwrap();
    let b = w.reserve(eref(2), 1, None).unwrap();
    let (ev_a, ctl_a) = Event::mock();
    w.commit_deferred(&a, ev_a).unwrap();
    w.commit_deferred(&b, done()).unwrap();
    assert_eq!(w.drain(), 0);
    assert!(! w.states().is_valid(b.slot));
    ctl_a.complete();
    assert_eq!(w.drain(), 2);
    assert_eq!(w.slot_info(a.slot).unwrap().write_no, 1);
    assert_eq!(w.slot_info(b.slot).unwrap().write_no, 2);
}
#[test]
fn drain_discards_failed_events_and_continues() {
    let w = writer(3);
    let a = w.reserve(eref(1), 1, None).unwrap();
    let b = w.reserve(eref(2), 1, None).unwrap();
    let (ev_a, ctl_a) = Event::mock();
    ctl_a.fail("copy failed");
    w.commit_deferred(&a, ev_a).unwrap();
    w.commit_deferred(&b, done()).unwrap();
    assert_eq!(w.drain(), 1);
    assert!(! w.states().is_valid(a.slot));
    assert!(w.states().is_valid(b.slot));
    assert_eq!(w.slot_info(a.slot).unwrap().write_no, 0);
    assert_eq!(w.slot_info(b.slot).unwrap().write_no, 1);
    assert_eq!(w.pending_count(), 0);
    assert_eq!(
        w.commit(& a, & done()).err(), Some(WriteError::NotReserved { slot : a.slot })
    );
}
#[test]
fn immediate_commit_overtakes_queue() {
    let w = writer(3);
    let a = w.reserve(eref(1), 1, None).unwrap();
    let b = w.reserve(eref(2), 1, None).unwrap();
    let (ev_a, _ctl) = Event::mock();
    w.commit_deferred(&a, ev_a).unwrap();
    w.commit(&b, &done()).unwrap();
    assert!(w.states().is_valid(b.slot));
    assert!(! w.states().is_valid(a.slot));
}
#[test]
fn tombstone_hides_entry_and_is_idempotent() {
    let w = writer(3);
    let slot = insert(&w, 1, 4);
    assert!(w.tombstone(eref(1)));
    assert!(! w.states().is_valid(slot));
    assert_eq!(w.assignment().slot_of_entry(eref(1)), None);
    assert_eq!(w.resolve(slot), Some(eref(1)));
    assert!(! w.tombstone(eref(1)));
    assert!(! w.tombstone(eref(99)));
    let again = w.reserve(eref(1), 2, None).unwrap();
    w.commit(&again, &done()).unwrap();
    assert_eq!(w.assignment().slot_of_entry(eref(1)), Some(again.slot));
    assert_eq!(w.resolve(slot), Some(eref(1)));
}
#[test]
fn tombstone_does_not_touch_open_reservations() {
    let w = writer(3);
    let res = w.reserve(eref(1), 1, None).unwrap();
    assert!(! w.tombstone(eref(1)));
    assert_eq!(w.pending_count(), 1);
    w.commit(&res, &done()).unwrap();
    assert!(w.states().is_valid(res.slot));
    let slot = insert(&w, 2, 1);
    let upd = w.reserve(eref(2), 1, None).unwrap();
    assert!(w.tombstone(eref(2)));
    assert!(! w.states().is_valid(slot));
    w.commit(&upd, &done()).unwrap();
    assert!(w.states().is_valid(upd.slot));
    assert_eq!(w.assignment().slot_of_entry(eref(2)), Some(upd.slot));
}
#[test]
fn slot_info_bounds_and_untouched_slots() {
    let w = writer(2);
    assert!(
        matches!(w.slot_info(Slot5(2)), Err(WriteError::Slot(SlotError::SlotOutOfRange {
        slot : 2, count : 2 })))
    );
    assert_eq!(
        w.slot_info(Slot5(1)).unwrap(), SlotInfo { valid : false, tokens : 0, write_no :
        0, centroid : None, }
    );
    assert_eq!(w.resolve(Slot5(9)), None);
}
#[test]
fn compaction_plan_packs_valid_slots() {
    let w = writer(6);
    for id in 1..=4 {
        let res = w
            .reserve(eref(id), id as u32, Some(CentroidId(id as u32 * 10)))
            .unwrap();
        w.commit(&res, &done()).unwrap();
    }
    w.reserve(eref(5), 1, None).unwrap();
    assert!(w.tombstone(eref(2)));
    let plan = w.compaction_plan();
    assert_eq!(plan.reclaimed_slots, 2);
    assert_eq!(
        plan.moves, vec![SlotMove { from : Slot5(0), to : Slot5(0), entry : eref(1),
        tokens : 1, centroid : Some(CentroidId(10)), }, SlotMove { from : Slot5(2), to :
        Slot5(1), entry : eref(3), tokens : 3, centroid : Some(CentroidId(30)), },
        SlotMove { from : Slot5(3), to : Slot5(2), entry : eref(4), tokens : 4, centroid
        : Some(CentroidId(40)), },]
    );
    assert_eq!(w.states().valid_count(), 3);
}
#[test]
fn rebuild_requires_no_pending_commits() {
    let w = writer(3);
    let res = w.reserve(eref(1), 1, None).unwrap();
    assert!(matches!(w.rebuild(view(3), Addr(0)), Err(WriteError::Busy)));
    let (ev, _ctl) = Event::mock();
    w.commit_deferred(&res, ev).unwrap();
    assert!(matches!(w.rebuild(view(3), Addr(0)), Err(WriteError::Busy)));
}
#[test]
fn rebuild_rejects_mismatching_targets() {
    let w = writer(4);
    insert(&w, 1, 1);
    insert(&w, 2, 1);
    let bad = [
        view_with(4, SlotClass::C64, 4, DType::F32),
        view_with(4, SlotClass::C32, 8, DType::F32),
        view_with(4, SlotClass::C32, 4, DType::F16),
        view(1),
    ];
    for v in bad {
        assert!(matches!(w.rebuild(v, Addr(0)), Err(WriteError::RebuildTarget)));
    }
    assert!(w.rebuild(view(2), Addr(0)).is_ok());
}
#[test]
fn rebuild_creates_reservations_and_leaves_old_writer_untouched() {
    let w = writer(5);
    insert(&w, 1, 1);
    insert(&w, 2, 2);
    insert(&w, 3, 3);
    assert!(w.tombstone(eref(2)));
    let before = w.compaction_plan();
    let (nw, reservations) = w.rebuild(view(4), Addr(1 << 24)).unwrap();
    assert_eq!(w.compaction_plan(), before);
    assert_eq!(w.used_slots(), 3);
    assert_eq!(w.pending_count(), 0);
    assert_eq!(w.resolve(Slot5(1)), Some(eref(2)));
    assert!(w.states().is_valid(Slot5(0)));
    assert_eq!(reservations.len(), 2);
    assert_eq!(reservations[0].0, Slot5(0));
    assert_eq!(reservations[1].0, Slot5(2));
    assert_eq!(reservations[0].1.slot, Slot5(0));
    assert_eq!(reservations[1].1.slot, Slot5(1));
    assert_eq!(reservations[1].1.entry, eref(3));
    assert_eq!(reservations[1].1.tokens, 3);
    assert_eq!(nw.used_slots(), 2);
    assert_eq!(nw.pending_count(), 2);
    assert_eq!(nw.states().valid_count(), 0);
    assert_eq!(nw.resolve(Slot5(0)), None);
    assert_eq!(
        w.commit(& reservations[0].1, & done()).err(),
        Some(WriteError::ForeignReservation)
    );
    for (_, res) in &reservations {
        nw.commit(res, &done()).unwrap();
    }
    assert_eq!(nw.states().valid_count(), 2);
    assert_eq!(nw.assignment().slot_of_entry(eref(3)), Some(Slot5(1)));
    assert_eq!(nw.slot_info(Slot5(0)).unwrap().write_no, 1);
    assert_eq!(nw.slot_info(Slot5(1)).unwrap().write_no, 2);
    assert_eq!(nw.free_slots(), 2);
}
#[test]
fn rebuild_drops_replaced_table_and_keeps_centroids() {
    let w = writer(4);
    let res = w.reserve(eref(1), 1, Some(CentroidId(9))).unwrap();
    w.commit(&res, &done()).unwrap();
    let upd = w.reserve(eref(1), 2, Some(CentroidId(11))).unwrap();
    w.commit(&upd, &done()).unwrap();
    let (nw, reservations) = w.rebuild(view(4), Addr(0)).unwrap();
    assert_eq!(reservations.len(), 1);
    assert_eq!(reservations[0].0, upd.slot);
    assert_eq!(reservations[0].1.centroid, Some(CentroidId(11)));
    nw.commit(&reservations[0].1, &done()).unwrap();
    assert_eq!(nw.resolve(Slot5(1)), None);
    assert_eq!(nw.resolve(Slot5(0)), Some(eref(1)));
    assert_eq!(w.resolve(res.slot), Some(eref(1)));
}
#[test]
fn concurrent_reservations_get_distinct_slots() {
    let w = Arc::new(writer(64));
    let mut handles = Vec::new();
    for t in 0..8u64 {
        let w = Arc::clone(&w);
        handles
            .push(
                std::thread::spawn(move || {
                    (0..8u64)
                        .map(|i| {
                            let res = w.reserve(eref(t * 100 + i), 1, None).unwrap();
                            w.commit(&res, &done()).unwrap();
                            res.slot.0
                        })
                        .collect::<Vec<_>>()
                }),
            );
    }
    let mut slots: Vec<u32> = handles
        .into_iter()
        .flat_map(|h| h.join().unwrap())
        .collect();
    slots.sort_unstable();
    assert_eq!(slots, (0..64).collect::< Vec < _ >> ());
    assert_eq!(w.states().valid_count(), 64);
    assert_eq!(w.free_slots(), 0);
    let mut write_nos: Vec<u64> = (0..64)
        .map(|s| w.slot_info(Slot5(s)).unwrap().write_no)
        .collect();
    write_nos.sort_unstable();
    assert_eq!(write_nos, (1..= 64).collect::< Vec < _ >> ());
}
#[test]
fn readers_see_an_entry_while_it_is_updated() {
    let w = Arc::new(writer(200));
    insert(&w, 1, 1);
    let stop = Arc::new(std::sync::atomic::AtomicBool::new(false));
    let reader = {
        let w = Arc::clone(&w);
        let stop = Arc::clone(&stop);
        std::thread::spawn(move || {
            let asg = w.assignment();
            while !stop.load(Ordering::Relaxed) {
                assert!(
                    asg.slot_of_entry(eref(1)).is_some() || w.states().valid_count() >= 1
                );
            }
        })
    };
    for _ in 0..150 {
        let res = w.reserve(eref(1), 1, None).unwrap();
        w.commit(&res, &done()).unwrap();
        assert_eq!(w.states().valid_count(), 1);
    }
    stop.store(true, Ordering::Relaxed);
    reader.join().unwrap();
}
#[test]
fn poisoned_lock_is_recovered() {
    let w = Arc::new(writer(4));
    let poisoner = {
        let w = Arc::clone(&w);
        std::thread::spawn(move || {
            let _guard = w.state.lock().unwrap();
            panic!("poison");
        })
    };
    assert!(poisoner.join().is_err());
    let slot = insert(&w, 1, 1);
    assert!(w.states().is_valid(slot));
    assert_eq!(w.drain(), 0);
}
#[test]
fn error_display_and_source() {
    use std::error::Error;
    let e = WriteError::NotReserved {
        slot: Slot5(3),
    };
    assert!(e.to_string().contains('3'));
    assert!(e.source().is_none());
    let s = WriteError::from(SlotError::SlotOutOfRange {
        slot: 1,
        count: 1,
    });
    assert!(s.source().is_some());
    assert!(! WriteError::Busy.to_string().is_empty());
}
fn set_expiry(w: &SegmentWriter, ttl: Duration, interval: Duration) {
    let mut st = w.lock();
    st.ttl = ttl;
    st.sweep_interval = interval;
}
#[test]
fn reserve_sweeps_expired_reservations() {
    let w = writer(4);
    set_expiry(&w, Duration::ZERO, Duration::ZERO);
    let stale = w.reserve(eref(1), 1, None).unwrap();
    assert_eq!(w.pending_count(), 1);
    let fresh = w.reserve(eref(2), 1, None).unwrap();
    assert_eq!(w.pending_count(), 1);
    assert_eq!(
        w.commit(& stale, & done()).err(), Some(WriteError::NotReserved { slot : stale
        .slot })
    );
    assert_eq!(w.used_slots(), 2);
    assert_eq!(fresh.slot, Slot5(1));
    assert!(! w.states().is_valid(stale.slot));
    assert_eq!(w.resolve(stale.slot), None);
}
#[test]
fn sweep_keeps_young_reservations_and_respects_interval() {
    let w = writer(4);
    let res = w.reserve(eref(1), 1, None).unwrap();
    w.reserve(eref(2), 1, None).unwrap();
    assert_eq!(w.pending_count(), 2);
    w.commit(&res, &done()).unwrap();
    set_expiry(&w, Duration::ZERO, Duration::from_secs(3600));
    w.reserve(eref(3), 1, None).unwrap();
    assert_eq!(w.pending_count(), 2);
}
#[test]
fn sweep_does_not_touch_queued_commits() {
    let w = writer(4);
    set_expiry(&w, Duration::ZERO, Duration::ZERO);
    let res = w.reserve(eref(1), 1, None).unwrap();
    let (ev, ctl) = Event::mock();
    w.commit_deferred(&res, ev).unwrap();
    w.reserve(eref(2), 1, None).unwrap();
    w.reserve(eref(3), 1, None).unwrap();
    ctl.complete();
    assert_eq!(w.drain(), 1);
    assert!(w.states().is_valid(res.slot));
}
#[test]
fn full_segment_sweeps_so_rebuild_can_proceed() {
    let w = writer(2);
    insert(&w, 1, 1);
    let _lost = w.reserve(eref(2), 1, None).unwrap();
    set_expiry(&w, Duration::ZERO, Duration::ZERO);
    assert_eq!(w.reserve(eref(3), 1, None).err(), Some(WriteError::NoFreeSlot));
    assert_eq!(w.pending_count(), 0);
    let (nw, reservations) = w.rebuild(view(2), Addr(0)).unwrap();
    assert_eq!(reservations.len(), 1);
    assert_eq!(nw.used_slots(), 1);
}
#[test]
fn rebuild_forces_sweep_ignoring_interval() {
    let w = writer(3);
    insert(&w, 1, 1);
    w.reserve(eref(2), 1, None).unwrap();
    set_expiry(&w, Duration::ZERO, Duration::from_secs(3600));
    assert!(w.rebuild(view(3), Addr(0)).is_ok());
}
#[test]
fn live_reservation_survives_default_ttl() {
    let w = writer(2);
    let res = w.reserve(eref(1), 1, None).unwrap();
    w.reserve(eref(2), 1, None).unwrap();
    w.commit(&res, &done()).unwrap();
    assert!(matches!(w.rebuild(view(2), Addr(0)), Err(WriteError::Busy)));
}
