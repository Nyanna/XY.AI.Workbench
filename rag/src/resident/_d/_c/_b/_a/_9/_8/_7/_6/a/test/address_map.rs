use super::*;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::a::slot_layout::{
    EntryRef, SlotClass,
};
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::a::region_handle::{
    Generation, RegionHandle,
};
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::a::arenas::ArenaKind;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::a::index_contract::EntryId;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::_1::layout::DType;
static_assertions::assert_impl_all!(Entry : Send, Sync);
static_assertions::assert_impl_all!(AddressMap : Send, Sync);
static_assertions::assert_impl_all!(MappingStore : Send, Sync);
static_assertions::assert_impl_all!(SegmentId : Send, Sync);
static_assertions::assert_impl_all!(MapError : Send, Sync);
const SLOT_BYTES: u64 = 512;
fn entry_at(id: u64, offset: u64, slots: u32) -> Entry {
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
    Entry::new(SegmentId(id), view, Addr(offset), states, assignment).unwrap()
}
#[test]
fn entry_range_and_slot_at() {
    let e = entry_at(1, 1000, 3);
    assert_eq!(e.range(), 1000..1000 + 3 * SLOT_BYTES);
    assert_eq!(e.slot_at(999), None);
    assert_eq!(e.slot_at(1000), Some(Slot(0)));
    assert_eq!(e.slot_at(1000 + SLOT_BYTES - 1), Some(Slot(0)));
    assert_eq!(e.slot_at(1000 + SLOT_BYTES), Some(Slot(1)));
    assert_eq!(e.slot_at(1000 + 3 * SLOT_BYTES), None);
    assert_eq!(e.id(), SegmentId(1));
    assert_eq!(e.device_base(), Addr(1000));
}
#[test]
fn entry_range_at_arena_end() {
    let region = RegionHandle::new(
            ArenaKind::IndexSegments,
            u64::MAX - SLOT_BYTES,
            SLOT_BYTES,
            Generation(0),
        )
        .unwrap();
    let view = SegmentView::new(region, 1, SlotClass::C32, 4, DType::F32, Generation(0))
        .unwrap();
    let states = Arc::new(SlotStates::new(&view));
    let assignment = Arc::new(SlotAssignment::fixed(&view, Addr(0)).unwrap());
    let e = Entry::new(SegmentId(1), view, Addr(0), states, assignment).unwrap();
    assert_eq!(e.range().end, u64::MAX);
    assert_eq!(e.slot_at(u64::MAX), None);
}
#[test]
fn add_errors_in_order_and_unchanged() {
    let mut m = AddressMap::new(0);
    assert_eq!(m.add(entry_at(1, 0, 0)), Err(MapError::EmptySegment));
    m.add(entry_at(1, 1024, 2)).unwrap();
    assert_eq!(m.add(entry_at(1, 0, 0)), Err(MapError::EmptySegment));
    assert_eq!(m.add(entry_at(1, 8192, 1)), Err(MapError::DuplicateId(SegmentId(1))));
    assert_eq!(
        m.add(entry_at(2, 1024 + 512, 2)), Err(MapError::Overlap { with : SegmentId(1) })
    );
    assert_eq!(
        m.add(entry_at(2, 512, 2)), Err(MapError::Overlap { with : SegmentId(1) })
    );
    assert_eq!(m.add(entry_at(2, 0, 8)), Err(MapError::Overlap { with : SegmentId(1) }));
    assert_eq!(m.len(), 1);
}
#[test]
fn adjacent_allowed_and_sorted() {
    let mut m = AddressMap::new(0);
    m.add(entry_at(3, 2048, 2)).unwrap();
    m.add(entry_at(1, 0, 2)).unwrap();
    m.add(entry_at(2, 1024, 2)).unwrap();
    let ids: Vec<u64> = m.segments().iter().map(|e| e.id().0).collect();
    assert_eq!(ids, vec![1, 2, 3]);
    assert_eq!(m.len(), 3);
    assert!(! m.is_empty());
}
#[test]
fn lookup_and_get() {
    let mut m = AddressMap::new(0);
    m.add(entry_at(1, 1024, 2)).unwrap();
    m.add(entry_at(2, 4096, 2)).unwrap();
    assert!(m.lookup(1023).is_none());
    assert_eq!(m.lookup(1024).unwrap().id(), SegmentId(1));
    assert_eq!(m.lookup(2047).unwrap().id(), SegmentId(1));
    assert!(m.lookup(2048).is_none());
    assert_eq!(m.lookup(4096 + 1023).unwrap().id(), SegmentId(2));
    assert!(m.lookup(u64::MAX).is_none());
    assert!(m.get(SegmentId(2)).is_some());
    assert!(m.get(SegmentId(9)).is_none());
}
#[test]
fn remove_behaviour() {
    let mut m = AddressMap::new(0);
    m.add(entry_at(1, 0, 2)).unwrap();
    m.add(entry_at(2, 1024, 2)).unwrap();
    assert!(m.remove(SegmentId(1)));
    assert!(! m.remove(SegmentId(1)));
    assert_eq!(m.len(), 1);
    assert!(m.lookup(0).is_none());
}
#[test]
fn copy_on_write_isolation() {
    let mut m = AddressMap::new(0);
    m.add(entry_at(1, 0, 2)).unwrap();
    let snap = m.clone();
    m.add(entry_at(2, 1024, 2)).unwrap();
    assert_eq!(snap.len(), 1);
    assert_eq!(m.len(), 2);
    let mut s2 = m.successor().unwrap();
    s2.remove(SegmentId(1));
    assert_eq!(m.len(), 2);
    assert_eq!(s2.len(), 1);
}
#[test]
fn epoch_rules() {
    let mut m = AddressMap::new(5);
    m.add(entry_at(1, 0, 1)).unwrap();
    assert_eq!(m.epoch(), 5);
    m.remove(SegmentId(1));
    assert_eq!(m.epoch(), 5);
    assert_eq!(m.successor().unwrap().epoch(), 6);
    assert_eq!(
        AddressMap::new(u64::MAX).successor().unwrap_err(), MapError::EpochOverflow
    );
}
#[test]
fn shared_bindings_across_epochs() {
    let mut m = AddressMap::new(0);
    let e = entry_at(1, 0, 2);
    m.add(e.clone()).unwrap();
    let next = m.successor().unwrap();
    let r = EntryRef {
        entry: EntryId(7),
        section: 0,
    };
    e.assignment().bind(Slot(0), r).unwrap();
    let seen = next.get(SegmentId(1)).unwrap().assignment().lookup(Slot(0));
    assert_eq!(seen, Some(r));
    assert!(
        Arc::ptr_eq(m.get(SegmentId(1)).unwrap().states(), next.get(SegmentId(1))
        .unwrap().states())
    );
}
#[test]
fn store_publish_flow() {
    let store = MappingStore::default();
    assert_eq!(store.epoch(), 0);
    assert!(store.current().is_empty());
    let mut n = store.next().unwrap();
    n.add(entry_at(1, 0, 2)).unwrap();
    let old = store.snapshot(0).unwrap();
    assert_eq!(store.publish(n).unwrap(), 1);
    assert_eq!(store.epoch(), 1);
    assert!(old.is_empty());
    assert_eq!(store.snapshot(1).unwrap().len(), 1);
    assert_eq!(
        store.snapshot(0).unwrap_err(), MapError::StaleMapping { base : 0, published : 1
        }
    );
}
#[test]
fn store_publish_rejects_stale() {
    let store = MappingStore::new();
    let a = store.next().unwrap();
    let b = store.next().unwrap();
    store.publish(a).unwrap();
    assert_eq!(
        store.publish(b).unwrap_err(), MapError::StaleMapping { base : 0, published : 1 }
    );
    assert_eq!(
        store.publish(AddressMap::new(5)).unwrap_err(), MapError::StaleMapping { base :
        4, published : 1 }
    );
    assert_eq!(
        store.publish(AddressMap::new(0)).unwrap_err(), MapError::StaleMapping { base :
        0, published : 1 }
    );
    assert_eq!(store.epoch(), 1);
    assert_eq!(
        MapError::StaleMapping { base : 0, published : 1 } .to_string(), "stale Mapping"
    );
}
#[test]
fn store_concurrent_publish_single_winner() {
    let store = Arc::new(MappingStore::new());
    let handles: Vec<_> = (0..8)
        .map(|_| {
            let s = Arc::clone(&store);
            std::thread::spawn(move || {
                let m = s.next().unwrap();
                s.publish(m).is_ok()
            })
        })
        .collect();
    let wins = handles.into_iter().map(|h| h.join().unwrap()).filter(|&w| w).count();
    assert!(wins >= 1);
    assert_eq!(store.epoch(), wins as u64);
}
