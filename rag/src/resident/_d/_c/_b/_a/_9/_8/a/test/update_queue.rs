use super::*;
use std::sync::Arc;
use std::thread;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::a::segments::SegmentId;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::a::slot_layout::{
    SegmentView, SlotAssignment, SlotClass, SlotStates,
};
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::a::kernel_layout::{
    Addr, Slot5,
};
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::a::region_handle::{
    Generation, RegionHandle,
};
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::a::arenas::ArenaKind;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::_1::layout::DType;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::a::address_map::Entry6;
static_assertions::assert_impl_all!(UpdateQueue : Send, Sync);
const SLOT_BYTES: u64 = 512;
fn segment(id: u64, offset: u64, slots: u32) -> Entry6 {
    let region = RegionHandle::new(
            ArenaKind::IndexSegments,
            offset,
            u64::from(slots) * SLOT_BYTES,
            Generation(1),
        )
        .unwrap();
    let view = SegmentView::new(
            region,
            slots,
            SlotClass::C32,
            4,
            DType::F32,
            Generation(1),
        )
        .unwrap();
    let states = Arc::new(SlotStates::new(&view));
    let assignment = Arc::new(SlotAssignment::fixed(&view, Addr(offset)).unwrap());
    Entry6::new(SegmentId(id), view, Addr(offset), states, assignment).unwrap()
}
fn map_with(segments: &[(u64, u64, u32)]) -> AddressMap {
    let mut map = AddressMap::new(0);
    for &(id, offset, slots) in segments {
        map.add(segment(id, offset, slots)).unwrap();
    }
    map
}
fn eref(id: u64) -> EntryRef {
    EntryRef {
        entry: EntryId(id),
        section: 0,
    }
}
fn sref(segment: u64, slot: u32) -> SlotRef {
    SlotRef {
        segment: SegmentId(segment),
        slot: Slot5(slot),
    }
}
fn st(raw: u64) -> Stamp {
    Stamp(raw)
}
fn assignment_of(map: &AddressMap, segment: u64) -> Arc<SlotAssignment> {
    Arc::clone(map.get(SegmentId(segment)).unwrap().assignment())
}
#[test]
fn empty_queue() {
    let q = UpdateQueue::new();
    let dir = ResidencyDirectory::new();
    let map = map_with(&[(1, 0, 4)]);
    assert!(q.is_empty());
    assert_eq!(q.depth(), 0);
    assert_eq!(q.deferred_writes(), 0);
    assert_eq!(q.apply(& dir, & map), 0);
    assert!(q.take_warnings().is_empty());
}
#[test]
fn latest_update_replaces_previous() {
    let q = UpdateQueue::new();
    q.push_unbind(eref(1), sref(1, 0), st(1));
    q.push_bind(eref(1), sref(1, 2), st(2));
    assert_eq!(q.depth(), 1);
    assert_eq!(q.deferred_writes(), 1);
    let u = q.get(EntryId(1)).unwrap();
    assert_eq!(u.kind, UpdateKind::Bind);
    assert_eq!(u.slot, sref(1, 2));
    assert_eq!(u.stamp, st(2));
    q.push_unbind(eref(1), sref(1, 2), st(3));
    assert_eq!(q.deferred_writes(), 0);
    assert_eq!(q.get(EntryId(1)).unwrap().kind, UpdateKind::Unbind);
    assert!(q.contains(EntryId(1)));
    assert!(! q.contains(EntryId(2)));
    assert_eq!(q.get(EntryId(2)), None);
}
#[test]
fn push_does_not_touch_state() {
    let q = UpdateQueue::new();
    let dir = ResidencyDirectory::new();
    let map = map_with(&[(1, 0, 4)]);
    q.push_bind(eref(1), sref(1, 0), st(5));
    assert!(dir.is_empty());
    assert_eq!(assignment_of(& map, 1).lookup(Slot5(0)), None);
}
#[test]
fn bind_binds_and_records_stamp() {
    let q = UpdateQueue::new();
    let dir = ResidencyDirectory::new();
    let map = map_with(&[(1, 0, 4)]);
    q.push_bind(eref(1), sref(1, 2), st(7));
    assert_eq!(q.apply(& dir, & map), 1);
    assert!(q.is_empty());
    assert_eq!(assignment_of(& map, 1).lookup(Slot5(2)), Some(eref(1)));
    let r = dir.get(EntryId(1)).unwrap();
    assert_eq!(r.slot, sref(1, 2));
    assert_eq!(r.stamp, st(7));
    assert!(q.take_warnings().is_empty());
}
#[test]
fn bind_does_not_change_valid_bits() {
    let q = UpdateQueue::new();
    let dir = ResidencyDirectory::new();
    let map = map_with(&[(1, 0, 4)]);
    let states = Arc::clone(map.get(SegmentId(1)).unwrap().states());
    states.set_filled(Slot5(1), 3).unwrap();
    q.push_bind(eref(1), sref(1, 1), st(1));
    q.apply(&dir, &map);
    assert!(states.is_valid(Slot5(1)));
    assert!(! states.is_valid(Slot5(0)));
    q.push_unbind(eref(1), sref(1, 1), st(2));
    q.apply(&dir, &map);
    assert!(states.is_valid(Slot5(1)));
}
#[test]
fn unbind_removes_directory_and_binding() {
    let q = UpdateQueue::new();
    let dir = ResidencyDirectory::new();
    let map = map_with(&[(1, 0, 4)]);
    q.push_bind(eref(1), sref(1, 0), st(1));
    q.apply(&dir, &map);
    q.push_unbind(eref(1), sref(1, 0), st(2));
    assert_eq!(q.apply(& dir, & map), 1);
    assert_eq!(dir.get(EntryId(1)), None);
    assert_eq!(assignment_of(& map, 1).lookup(Slot5(0)), None);
    assert!(q.take_warnings().is_empty());
}
#[test]
fn unbind_keeps_binding_of_other_entry() {
    let q = UpdateQueue::new();
    let dir = ResidencyDirectory::new();
    let map = map_with(&[(1, 0, 4)]);
    let a = assignment_of(&map, 1);
    a.bind(Slot5(0), eref(2)).unwrap();
    dir.insert(EntryId(1), sref(1, 0), Stamp(1));
    q.push_unbind(eref(1), sref(1, 0), st(2));
    assert_eq!(q.apply(& dir, & map), 1);
    assert_eq!(dir.get(EntryId(1)), None);
    assert_eq!(a.lookup(Slot5(0)), Some(eref(2)));
    assert!(q.take_warnings().is_empty());
}
#[test]
fn unbind_unmapped_segment_warns_but_counts_and_removes() {
    let q = UpdateQueue::new();
    let dir = ResidencyDirectory::new();
    let map = map_with(&[(1, 0, 4)]);
    dir.insert(EntryId(1), sref(9, 0), Stamp(1));
    q.push_unbind(eref(1), sref(9, 0), st(2));
    assert_eq!(q.apply(& dir, & map), 1);
    assert_eq!(dir.get(EntryId(1)), None);
    assert_eq!(
        q.take_warnings(), vec![Warning::UnmappedTarget { entry : eref(1), slot : sref(9,
        0), kind : UpdateKind::Unbind, }]
    );
    assert!(q.take_warnings().is_empty());
}
#[test]
fn bind_unmapped_segment_changes_nothing() {
    let q = UpdateQueue::new();
    let dir = ResidencyDirectory::new();
    let map = map_with(&[(1, 0, 4)]);
    dir.insert(EntryId(1), sref(1, 0), Stamp(1));
    assignment_of(&map, 1).bind(Slot5(0), eref(1)).unwrap();
    q.push_bind(eref(1), sref(9, 0), st(2));
    assert_eq!(q.apply(& dir, & map), 0);
    assert_eq!(dir.get(EntryId(1)).unwrap().slot, sref(1, 0));
    assert_eq!(assignment_of(& map, 1).lookup(Slot5(0)), Some(eref(1)));
    assert_eq!(
        q.take_warnings(), vec![Warning::UnmappedTarget { entry : eref(1), slot : sref(9,
        0), kind : UpdateKind::Bind, }]
    );
}
#[test]
fn bind_moves_entry_within_segment() {
    let q = UpdateQueue::new();
    let dir = ResidencyDirectory::new();
    let map = map_with(&[(1, 0, 4)]);
    q.push_bind(eref(1), sref(1, 0), st(1));
    q.apply(&dir, &map);
    q.push_bind(eref(1), sref(1, 3), st(2));
    assert_eq!(q.apply(& dir, & map), 1);
    let a = assignment_of(&map, 1);
    assert_eq!(a.lookup(Slot5(0)), None);
    assert_eq!(a.lookup(Slot5(3)), Some(eref(1)));
    assert_eq!(dir.get(EntryId(1)).unwrap().slot, sref(1, 3));
    assert_eq!(dir.get(EntryId(1)).unwrap().stamp, st(2));
}
#[test]
fn bind_moves_entry_across_segments() {
    let q = UpdateQueue::new();
    let dir = ResidencyDirectory::new();
    let map = map_with(&[(1, 0, 4), (2, 10_000, 4)]);
    q.push_bind(eref(1), sref(1, 0), st(1));
    q.apply(&dir, &map);
    q.push_bind(eref(1), sref(2, 1), st(2));
    assert_eq!(q.apply(& dir, & map), 1);
    assert_eq!(assignment_of(& map, 1).lookup(Slot5(0)), None);
    assert_eq!(assignment_of(& map, 2).lookup(Slot5(1)), Some(eref(1)));
    assert_eq!(dir.get(EntryId(1)).unwrap().slot, sref(2, 1));
}
#[test]
fn bind_same_slot_rebinds() {
    let q = UpdateQueue::new();
    let dir = ResidencyDirectory::new();
    let map = map_with(&[(1, 0, 4)]);
    q.push_bind(eref(1), sref(1, 1), st(1));
    q.apply(&dir, &map);
    q.push_bind(eref(1), sref(1, 1), st(9));
    assert_eq!(q.apply(& dir, & map), 1);
    assert_eq!(assignment_of(& map, 1).lookup(Slot5(1)), Some(eref(1)));
    assert_eq!(dir.get(EntryId(1)).unwrap().stamp, st(9));
    assert!(q.take_warnings().is_empty());
}
#[test]
fn bind_occupied_is_rejected_and_old_binding_stays_released() {
    let q = UpdateQueue::new();
    let dir = ResidencyDirectory::new();
    let map = map_with(&[(1, 0, 4)]);
    let a = assignment_of(&map, 1);
    q.push_bind(eref(1), sref(1, 0), st(1));
    q.apply(&dir, &map);
    a.bind(Slot5(2), eref(2)).unwrap();
    q.push_bind(eref(1), sref(1, 2), st(2));
    assert_eq!(q.apply(& dir, & map), 0);
    assert_eq!(a.lookup(Slot5(0)), None);
    assert_eq!(a.lookup(Slot5(2)), Some(eref(2)));
    let r = dir.get(EntryId(1)).unwrap();
    assert_eq!(r.slot, sref(1, 0));
    assert_eq!(r.stamp, st(1));
    assert_eq!(
        q.take_warnings(), vec![Warning::BindRejected { entry : eref(1), slot : sref(1,
        2), }]
    );
}
#[test]
fn bind_out_of_range_is_rejected() {
    let q = UpdateQueue::new();
    let dir = ResidencyDirectory::new();
    let map = map_with(&[(1, 0, 4)]);
    q.push_bind(eref(1), sref(1, 4), st(1));
    assert_eq!(q.apply(& dir, & map), 0);
    assert_eq!(dir.get(EntryId(1)), None);
    assert_eq!(
        q.take_warnings(), vec![Warning::BindRejected { entry : eref(1), slot : sref(1,
        4), }]
    );
}
#[test]
fn unbinds_run_before_binds() {
    let q = UpdateQueue::new();
    let dir = ResidencyDirectory::new();
    let map = map_with(&[(1, 0, 4)]);
    let a = assignment_of(&map, 1);
    q.push_bind(eref(5), sref(1, 0), st(1));
    q.apply(&dir, &map);
    q.push_bind(eref(1), sref(1, 0), st(2));
    q.push_unbind(eref(5), sref(1, 0), st(3));
    assert_eq!(q.depth(), 2);
    assert_eq!(q.apply(& dir, & map), 2);
    assert_eq!(a.lookup(Slot5(0)), Some(eref(1)));
    assert_eq!(dir.get(EntryId(5)), None);
    assert_eq!(dir.get(EntryId(1)).unwrap().slot, sref(1, 0));
    assert!(q.take_warnings().is_empty());
}
#[test]
fn binds_apply_in_ascending_entry_order() {
    let q = UpdateQueue::new();
    let dir = ResidencyDirectory::new();
    let map = map_with(&[(1, 0, 4)]);
    q.push_bind(eref(9), sref(1, 0), st(1));
    q.push_bind(eref(3), sref(1, 0), st(2));
    assert_eq!(q.apply(& dir, & map), 1);
    assert_eq!(assignment_of(& map, 1).lookup(Slot5(0)), Some(eref(3)));
    assert_eq!(
        q.take_warnings(), vec![Warning::BindRejected { entry : eref(9), slot : sref(1,
        0), }]
    );
}
#[test]
fn concurrent_push_and_apply_apply_each_update_once() {
    let q = Arc::new(UpdateQueue::new());
    let dir = Arc::new(ResidencyDirectory::new());
    let map = Arc::new(map_with(&[(1, 0, 64)]));
    let mut handles = Vec::new();
    for t in 0..4u64 {
        let q = Arc::clone(&q);
        handles
            .push(
                thread::spawn(move || {
                    for i in 0..16u64 {
                        let id = t * 16 + i;
                        q.push_bind(eref(id), sref(1, id as u32), st(id));
                    }
                }),
            );
    }
    let appliers: Vec<_> = (0..2)
        .map(|_| {
            let (q, dir, map) = (Arc::clone(&q), Arc::clone(&dir), Arc::clone(&map));
            thread::spawn(move || {
                let mut total = 0;
                for _ in 0..50 {
                    total += q.apply(&dir, &map);
                }
                total
            })
        })
        .collect();
    for h in handles {
        h.join().unwrap();
    }
    let mut total: usize = appliers.into_iter().map(|h| h.join().unwrap()).sum();
    total += q.apply(&dir, &map);
    assert_eq!(total, 64);
    assert_eq!(dir.len(), 64);
    assert!(q.is_empty());
    assert!(q.take_warnings().is_empty());
}
