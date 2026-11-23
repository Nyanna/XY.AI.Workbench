use super::*;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use super::super::super::l040::l030::a::region_handle::Generation;
use super::super::super::l040::l030::l020::a::memory_access::RegionBackend;
static_assertions::assert_impl_all!(SegmentStore : Send, Sync);
static_assertions::assert_impl_all!(SlotError : Send, Sync);
#[derive(Debug)]
struct MemBackend {
    bytes: Mutex<Vec<u8>>,
    fail_writes: Arc<AtomicBool>,
}
impl RegionBackend for MemBackend {
    fn len(&self) -> usize {
        self.bytes.lock().unwrap().len()
    }
    fn read_raw(&self, offset: usize, dst: &mut [u8]) -> Result<(), AccessError> {
        dst.copy_from_slice(&self.bytes.lock().unwrap()[offset..offset + dst.len()]);
        Ok(())
    }
    fn write_raw(&self, offset: usize, src: &[u8]) -> Result<(), AccessError> {
        if self.fail_writes.load(Ordering::SeqCst) {
            return Err(AccessError::Backend("boom".into()));
        }
        self.bytes.lock().unwrap()[offset..offset + src.len()].copy_from_slice(src);
        Ok(())
    }
}
fn mem_with(len: usize, fail: &Arc<AtomicBool>) -> AccessRegion {
    AccessRegion::new(
        Arc::new(MemBackend {
            bytes: Mutex::new(vec![0xEE; len]),
            fail_writes: fail.clone(),
        }),
    )
}
fn mem(len: usize) -> AccessRegion {
    mem_with(len, &Arc::new(AtomicBool::new(false)))
}
fn handle_at(arena: ArenaKind, offset: u64, len: u64) -> RegionHandle {
    RegionHandle::new(arena, offset, len, Generation(0)).unwrap()
}
fn seg_handle(offset: u64, len: u64) -> RegionHandle {
    handle_at(ArenaKind::IndexSegments, offset, len)
}
fn store_with(slot_size: u64, len: u64) -> (SegmentStore, SegmentId) {
    let store = SegmentStore::new(slot_size).unwrap();
    let id = store.add_segment(seg_handle(0, len), mem(len as usize)).unwrap();
    (store, id)
}
#[test]
fn zero_slot_size_is_rejected() {
    assert_eq!(SegmentStore::new(0).err(), Some(SlotError::InvalidSlotSize));
    assert_eq!(SegmentStore::new(8).unwrap().slot_size(), 8);
}
#[test]
fn add_segment_validation_order() {
    let store = SegmentStore::new(16).unwrap();
    let wrong = handle_at(ArenaKind::Weights, 0, 8);
    assert_eq!(
        store.add_segment(wrong, mem(1)), Err(SlotError::WrongArena(ArenaKind::Weights))
    );
    assert_eq!(
        store.add_segment(seg_handle(0, 15), mem(1)), Err(SlotError::SegmentTooSmall {
        len : 15, slot_size : 16 })
    );
    assert_eq!(
        store.add_segment(seg_handle(0, 32), mem(31)), Err(SlotError::BackingMismatch {
        handle_len : 32, mem_len : 31 })
    );
    assert_eq!(store.stats().segments, 0);
}
#[test]
fn duplicate_region_is_rejected_but_other_fields_differ() {
    let store = SegmentStore::new(8).unwrap();
    let h = seg_handle(0, 32);
    store.add_segment(h, mem(32)).unwrap();
    assert_eq!(store.add_segment(h, mem(32)), Err(SlotError::DuplicateRegion(h)));
    let other_gen = RegionHandle::new(ArenaKind::IndexSegments, 0, 32, Generation(1))
        .unwrap();
    assert!(store.add_segment(other_gen, mem(32)).is_ok());
    assert!(store.add_segment(seg_handle(32, 32), mem(32)).is_ok());
}
#[test]
fn slot_count_rounds_down_and_ids_ascend() {
    let store = SegmentStore::new(10).unwrap();
    let a = store.add_segment(seg_handle(0, 35), mem(35)).unwrap();
    let b = store.add_segment(seg_handle(100, 10), mem(10)).unwrap();
    assert!(a < b);
    assert_eq!(store.slot_count(a), Some(3));
    assert_eq!(store.slot_count(b), Some(1));
    assert_eq!(store.slot_count(SegmentId(99)), None);
    assert_eq!(store.segment_ids(), vec![a, b]);
    assert_eq!(store.segment_handle(a), Some(seg_handle(0, 35)));
    assert_eq!(store.segment_handle(SegmentId(99)), None);
}
#[test]
fn segment_ids_are_never_reused() {
    let store = SegmentStore::new(8).unwrap();
    let h = seg_handle(0, 8);
    let a = store.add_segment(h, mem(8)).unwrap();
    store.remove_segment(a).unwrap();
    let b = store.add_segment(h, mem(8)).unwrap();
    assert!(b > a);
}
#[test]
fn put_picks_lowest_free_slot_across_segments() {
    let store = SegmentStore::new(4).unwrap();
    let a = store.add_segment(seg_handle(0, 8), mem(8)).unwrap();
    let b = store.add_segment(seg_handle(8, 8), mem(8)).unwrap();
    let refs: Vec<SlotRef> = (0..4)
        .map(|i| store.put(EntryId(i), b"x").unwrap())
        .collect();
    assert_eq!(
        refs, vec![SlotRef { segment : a, slot : Slot(0) }, SlotRef { segment : a, slot :
        Slot(1) }, SlotRef { segment : b, slot : Slot(0) }, SlotRef { segment : b, slot :
        Slot(1) },]
    );
    assert_eq!(store.remove(EntryId(1)), Some(SlotRef { segment : a, slot : Slot(1) }));
    assert_eq!(
        store.put(EntryId(9), b"y").unwrap(), SlotRef { segment : a, slot : Slot(1) }
    );
    assert_eq!(store.put(EntryId(10), b"z"), Err(SlotError::NoFreeSlot));
    assert_eq!(store.slot_of(EntryId(10)), None);
    assert_eq!(
        store.stats(), SegmentStats { segments : 2, slots_total : 4, slots_used : 4 }
    );
}
#[test]
fn put_overwrites_same_slot_and_pads_with_zero() {
    let (store, _) = store_with(8, 16);
    let first = store.put(EntryId(1), &[1, 2, 3, 4, 5, 6, 7, 8]).unwrap();
    let second = store.put(EntryId(1), &[9, 9]).unwrap();
    assert_eq!(first, second);
    let mut buf = [0xAAu8; 8];
    store.read(EntryId(1), &mut buf).unwrap();
    assert_eq!(buf, [9, 9, 0, 0, 0, 0, 0, 0]);
    assert_eq!(store.stats().slots_used, 1);
}
#[test]
fn empty_payload_zeroes_slot() {
    let (store, _) = store_with(4, 8);
    store.put(EntryId(1), &[]).unwrap();
    let mut buf = [0xAAu8; 4];
    store.read(EntryId(1), &mut buf).unwrap();
    assert_eq!(buf, [0; 4]);
}
#[test]
fn payload_too_large_changes_nothing() {
    let (store, _) = store_with(4, 8);
    assert_eq!(
        store.put(EntryId(1), & [0; 5]), Err(SlotError::PayloadTooLarge { len : 5,
        slot_size : 4 })
    );
    assert_eq!(store.slot_of(EntryId(1)), None);
    assert_eq!(store.stats().slots_used, 0);
}
#[test]
fn slots_do_not_overlap() {
    let (store, _) = store_with(4, 12);
    for i in 0..3u8 {
        store.put(EntryId(i as u64), &[i + 1; 4]).unwrap();
    }
    for i in 0..3u8 {
        let mut buf = [0u8; 4];
        store.read(EntryId(i as u64), &mut buf).unwrap();
        assert_eq!(buf, [i + 1; 4]);
    }
}
#[test]
fn trailing_remainder_is_unused() {
    let (store, id) = store_with(4, 10);
    assert_eq!(store.slot_count(id), Some(2));
    store.put(EntryId(1), b"a").unwrap();
    store.put(EntryId(2), b"b").unwrap();
    assert_eq!(store.put(EntryId(3), b"c"), Err(SlotError::NoFreeSlot));
}
#[test]
fn read_errors_and_partial_reads() {
    let (store, _) = store_with(4, 8);
    assert_eq!(
        store.read(EntryId(1), & mut [0u8; 1]), Err(SlotError::UnknownEntry(EntryId(1)))
    );
    store.put(EntryId(1), &[1, 2, 3, 4]).unwrap();
    assert_eq!(
        store.read(EntryId(1), & mut [0u8; 5]), Err(SlotError::OutOfBounds { len : 5,
        slot_size : 4 })
    );
    let mut two = [0u8; 2];
    store.read(EntryId(1), &mut two).unwrap();
    assert_eq!(two, [1, 2]);
    store.read(EntryId(1), &mut []).unwrap();
}
#[test]
fn remove_keeps_bytes_until_next_put() {
    let (store, id) = store_with(4, 8);
    let slot = store.put(EntryId(1), &[7; 4]).unwrap();
    assert_eq!(store.remove(EntryId(1)), Some(slot));
    assert_eq!(store.remove(EntryId(1)), None);
    assert_eq!(store.entry_at(slot), None);
    assert_eq!(
        store.read(EntryId(1), & mut [0u8; 1]), Err(SlotError::UnknownEntry(EntryId(1)))
    );
    let again = store.put(EntryId(2), &[1]).unwrap();
    assert_eq!(again, slot);
    assert_eq!(store.entries_in(id).unwrap(), vec![(Slot(0), EntryId(2))]);
}
#[test]
fn mapping_lookup_and_entries_in_order() {
    let (store, id) = store_with(4, 16);
    store.put(EntryId(30), b"a").unwrap();
    store.put(EntryId(10), b"b").unwrap();
    store.put(EntryId(20), b"c").unwrap();
    store.remove(EntryId(10));
    assert_eq!(
        store.entries_in(id).unwrap(), vec![(Slot(0), EntryId(30)), (Slot(2),
        EntryId(20))]
    );
    assert_eq!(
        store.slot_of(EntryId(20)), Some(SlotRef { segment : id, slot : Slot(2) })
    );
    assert_eq!(
        store.entry_at(SlotRef { segment : id, slot : Slot(2) }), Some(EntryId(20))
    );
    assert_eq!(store.entry_at(SlotRef { segment : id, slot : Slot(1) }), None);
    assert_eq!(store.entry_at(SlotRef { segment : SegmentId(7), slot : Slot(0) }), None);
    assert_eq!(
        store.entries_in(SegmentId(7)), Err(SlotError::UnknownSegment(SegmentId(7)))
    );
}
#[test]
fn remove_segment_returns_sorted_entries_and_drops_mapping() {
    let store = SegmentStore::new(4).unwrap();
    let h1 = seg_handle(0, 8);
    let a = store.add_segment(h1, mem(8)).unwrap();
    let b = store.add_segment(seg_handle(8, 8), mem(8)).unwrap();
    store.put(EntryId(5), b"a").unwrap();
    store.put(EntryId(2), b"b").unwrap();
    store.put(EntryId(9), b"c").unwrap();
    let removed = store.remove_segment(a).unwrap();
    assert_eq!(
        removed, RemovedSegment { id : a, handle : h1, entries : vec![EntryId(2),
        EntryId(5)] }
    );
    assert_eq!(store.slot_of(EntryId(5)), None);
    assert_eq!(store.slot_of(EntryId(2)), None);
    assert_eq!(
        store.read(EntryId(2), & mut [0u8; 1]), Err(SlotError::UnknownEntry(EntryId(2)))
    );
    assert_eq!(store.entry_at(SlotRef { segment : a, slot : Slot(0) }), None);
    assert_eq!(store.slot_of(EntryId(9)), Some(SlotRef { segment : b, slot : Slot(0) }));
    assert_eq!(store.remove_segment(a), Err(SlotError::UnknownSegment(a)));
    assert_eq!(store.segment_ids(), vec![b]);
    assert!(store.add_segment(h1, mem(8)).is_ok());
}
#[test]
fn backend_failure_leaves_mapping_untouched() {
    let fail = Arc::new(AtomicBool::new(false));
    let store = SegmentStore::new(4).unwrap();
    let id = store.add_segment(seg_handle(0, 8), mem_with(8, &fail)).unwrap();
    store.put(EntryId(1), &[1, 1, 1, 1]).unwrap();
    fail.store(true, Ordering::SeqCst);
    assert!(matches!(store.put(EntryId(2), b"x"), Err(SlotError::Access(_))));
    assert_eq!(store.slot_of(EntryId(2)), None);
    assert!(matches!(store.put(EntryId(1), & [2; 4]), Err(SlotError::Access(_))));
    assert_eq!(
        store.slot_of(EntryId(1)), Some(SlotRef { segment : id, slot : Slot(0) })
    );
    fail.store(false, Ordering::SeqCst);
    let mut buf = [0u8; 4];
    store.read(EntryId(1), &mut buf).unwrap();
    assert_eq!(buf, [1; 4]);
    assert_eq!(store.put(EntryId(2), b"x").unwrap().slot, Slot(1));
}
#[test]
fn segment_desc_and_entry_of_addr() {
    let (store, id) = store_with(16, 64);
    let desc = store.segment_desc(id, 1000).unwrap();
    assert_eq!(
        desc, SegmentDesc { base : 1000, entries : 4, layout : SegmentLayout::Fixed {
        stride : 16 } }
    );
    assert_eq!(
        store.segment_desc(SegmentId(5), 0), Err(SlotError::UnknownSegment(SegmentId(5)))
    );
    store.put(EntryId(7), b"a").unwrap();
    store.put(EntryId(8), b"b").unwrap();
    assert_eq!(store.entry_of_addr(id, 1000, 1000), Some(EntryId(7)));
    assert_eq!(store.entry_of_addr(id, 1000, 1015), Some(EntryId(7)));
    assert_eq!(store.entry_of_addr(id, 1000, 1016), Some(EntryId(8)));
    assert_eq!(store.entry_of_addr(id, 1000, 1032), None);
    assert_eq!(store.entry_of_addr(id, 1000, 999), None);
    assert_eq!(store.entry_of_addr(id, 1000, 1064), None);
    assert_eq!(store.entry_of_addr(SegmentId(9), 1000, 1000), None);
    store.remove(EntryId(7));
    assert_eq!(store.entry_of_addr(id, 1000, 1000), None);
}
#[test]
fn entry_of_addr_is_overflow_safe() {
    let (store, id) = store_with(16, 32);
    store.put(EntryId(1), b"a").unwrap();
    assert_eq!(store.entry_of_addr(id, u64::MAX - 8, u64::MAX), Some(EntryId(1)));
    assert_eq!(store.entry_of_addr(id, u64::MAX - 8, 0), None);
    assert_eq!(store.entry_of_addr(id, u64::MAX - 8, u64::MAX - 8), Some(EntryId(1)));
}
#[test]
fn concurrent_puts_get_distinct_slots() {
    let (store, id) = store_with(8, 8 * 64);
    let store = Arc::new(store);
    let handles: Vec<_> = (0..4u64)
        .map(|t| {
            let store = store.clone();
            thread::spawn(move || {
                (0..16u64)
                    .map(|i| store.put(EntryId(t * 100 + i), &[t as u8; 8]).unwrap())
                    .collect::<Vec<_>>()
            })
        })
        .collect();
    let mut all: Vec<SlotRef> = handles
        .into_iter()
        .flat_map(|h| h.join().unwrap())
        .collect();
    all.sort();
    all.dedup();
    assert_eq!(all.len(), 64);
    assert_eq!(store.stats().slots_used, 64);
    assert_eq!(store.entries_in(id).unwrap().len(), 64);
    assert_eq!(store.put(EntryId(999), b"x"), Err(SlotError::NoFreeSlot));
}
#[test]
fn concurrent_puts_for_same_entry_do_not_mix() {
    let (store, _) = store_with(8, 16);
    let store = Arc::new(store);
    let writers: Vec<_> = [0xA1u8, 0xB2]
        .into_iter()
        .map(|byte| {
            let store = store.clone();
            thread::spawn(move || {
                for _ in 0..200 {
                    store.put(EntryId(1), &[byte; 8]).unwrap();
                }
            })
        })
        .collect();
    for w in writers {
        w.join().unwrap();
    }
    let mut buf = [0u8; 8];
    store.read(EntryId(1), &mut buf).unwrap();
    assert!(buf == [0xA1; 8] || buf == [0xB2; 8]);
    assert_eq!(store.stats().slots_used, 1);
}
#[test]
fn error_display_is_non_empty() {
    let errors = [
        SlotError::InvalidSlotSize,
        SlotError::WrongArena(ArenaKind::Weights),
        SlotError::SegmentTooSmall {
            len: 1,
            slot_size: 2,
        },
        SlotError::BackingMismatch {
            handle_len: 1,
            mem_len: 2,
        },
        SlotError::UnknownSegment(SegmentId(1)),
        SlotError::DuplicateRegion(seg_handle(0, 1)),
        SlotError::NoFreeSlot,
        SlotError::PayloadTooLarge {
            len: 3,
            slot_size: 2,
        },
        SlotError::UnknownEntry(EntryId(1)),
        SlotError::OutOfBounds {
            len: 3,
            slot_size: 2,
        },
        SlotError::Access(AccessError::Backend("x".into())),
    ];
    for e in errors {
        assert!(! e.to_string().is_empty());
    }
}
