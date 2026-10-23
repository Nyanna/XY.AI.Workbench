use super::*;
fn eref(n: u64) -> EntryRef {
    EntryRef {
        entry: EntryId::new(n),
        chunk: 0,
    }
}
#[test]
fn fixed_addressing_is_arithmetic() {
    let map = SlotMap::fixed(1000, 100, 4);
    assert_eq!(map.index_of(1000), Some(0));
    assert_eq!(map.index_of(1199), Some(1));
    assert_eq!(map.index_of(1300), Some(3));
    assert_eq!(map.index_of(1400), None);
    assert_eq!(map.index_of(999), None);
}
#[test]
fn variable_addressing_searches_offsets() {
    let map = SlotMap::variable(vec![0, 10, 10, 35, 40]);
    assert_eq!(map.len(), 4);
    assert_eq!(map.index_of(0), Some(0));
    assert_eq!(map.index_of(9), Some(0));
    assert_eq!(map.index_of(10), Some(2));
    assert_eq!(map.index_of(39), Some(3));
    assert_eq!(map.index_of(40), None);
}
#[test]
fn bind_unbind_and_bounds() {
    let map = SlotMap::fixed(0, 8, 2);
    map.bind(1, eref(7)).unwrap();
    assert_eq!(map.entry(1), Some(eref(7)));
    map.unbind(1).unwrap();
    assert_eq!(map.entry(1), None);
    assert!(
        matches!(map.bind(2, eref(1)), Err(SegmentError::SlotOutOfRange { slot : 2,
        slots : 2 }))
    );
}
fn committed(seg: &std::sync::Arc<Segment>, entry: u64) -> u32 {
    let done = crate::resident::segment::mock::ManualFence::new();
    done.complete();
    let w = seg.begin_write(eref(entry), 1, None).unwrap();
    let slot = w.slot();
    seg.commit(w, &*done).unwrap();
    slot
}
#[test]
fn resolve_reports_stale_generation() {
    let store = crate::resident::segment::SegmentStore::new();
    let seg = crate::resident::segment::testutil::segment(&store, 1, 10, 0, 2);
    let slot = committed(&seg, 5);
    let m = Manifest::new(1, vec![seg]);
    let hit = SegmentHit {
        segment_gen: 10,
        slot,
        score: 1.0,
    };
    assert_eq!(m.resolve(& hit).unwrap(), eref(5));
    let stale = SegmentHit {
        segment_gen: 99,
        ..hit
    };
    assert!(
        matches!(m.resolve(& stale), Err(SegmentError::StaleGeneration { gen : 99 }))
    );
    let empty = SegmentHit { slot: 1, ..hit };
    assert!(
        matches!(m.resolve(& empty), Err(SegmentError::MappingMissing { slot : 1, ..
        }))
    );
}
#[test]
fn hits_from_unloaded_segments_are_remapped_or_discarded() {
    let store = crate::resident::segment::SegmentStore::new();
    let old_seg = crate::resident::segment::testutil::segment(&store, 1, 10, 0, 4);
    let kept = committed(&old_seg, 5);
    let gone = committed(&old_seg, 6);
    let old = Manifest::new(1, vec![old_seg]);
    let new_seg = crate::resident::segment::testutil::segment(
        &store,
        1,
        11,
        1 << 16,
        4,
    );
    let moved = committed(&new_seg, 5);
    let new = Manifest::new(2, vec![new_seg]);
    let hit = |slot| SegmentHit {
        segment_gen: 10,
        slot,
        score: 0.5,
    };
    match new.remap(&old, hit(kept)) {
        Resolution::Remapped { entry, hit } => {
            assert_eq!(entry, eref(5));
            assert_eq!((hit.segment_gen, hit.slot, hit.score), (11, moved, 0.5));
        }
        other => panic!("unexpected {other:?}"),
    }
    assert_eq!(new.remap(& old, hit(gone)), Resolution::Discarded);
}
#[test]
fn tombstoned_hit_follows_the_entry_to_its_new_slot() {
    let store = crate::resident::segment::SegmentStore::new();
    let seg = crate::resident::segment::testutil::segment(&store, 1, 10, 0, 4);
    let first = committed(&seg, 5);
    let second = committed(&seg, 5);
    let m = Manifest::new(1, vec![seg]);
    let stale = SegmentHit {
        segment_gen: 10,
        slot: first,
        score: 1.0,
    };
    match m.remap(&m, stale) {
        Resolution::Remapped { hit, .. } => assert_eq!(hit.slot, second),
        other => panic!("unexpected {other:?}"),
    }
    let live = SegmentHit {
        slot: second,
        ..stale
    };
    assert_eq!(m.remap(& m, live), Resolution::Entry(eref(5)));
}
