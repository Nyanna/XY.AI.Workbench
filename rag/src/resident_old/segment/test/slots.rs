use super::super::mock::ManualFence;
use super::super::testutil::segment;
use super::super::SegmentStore;
use super::*;

fn eref(n: u64) -> EntryRef {
    EntryRef { entry: EntryId::new(n), chunk: 0 }
}

#[test]
fn bitmap_set_clear_count() {
    let b = ValidBitmap::new(130);
    b.set(0);
    b.set(64);
    b.set(129);
    assert_eq!(b.count(), 3);
    assert!(b.get(64) && !b.get(65));
    b.clear(64);
    assert!(!b.get(64));
    b.set(500);
    assert!(!b.get(500));
    assert_eq!(b.snapshot().len(), 3);
}

#[test]
fn slot_is_invisible_until_its_transfer_completes() {
    let store = SegmentStore::new();
    let seg = segment(&store, 1, 10, 0, 4);
    let write = seg.begin_write(eref(1), 5, Some(9)).unwrap();
    let slot = write.slot();
    let fence = ManualFence::new();
    seg.commit_after(write, fence.clone());

    assert_eq!(seg.settle(), 0);
    assert!(!seg.is_valid(slot));
    fence.complete();
    assert_eq!(seg.settle(), 1);
    assert!(seg.is_valid(slot));
    let meta = seg.meta(slot).unwrap();
    assert_eq!((meta.token_count, meta.centroid), (5, Some(9)));
}

#[test]
fn commit_rejects_incomplete_transfer() {
    let store = SegmentStore::new();
    let seg = segment(&store, 1, 10, 0, 2);
    let write = seg.begin_write(eref(1), 1, None).unwrap();
    let fence = ManualFence::new();
    assert!(matches!(seg.commit(write, &*fence), Err(SegmentError::NotReady)));
    assert_eq!(seg.valid_count(), 0);
}

#[test]
fn upsert_writes_a_new_slot_then_tombstones_the_old_one() {
    let store = SegmentStore::new();
    let seg = segment(&store, 1, 10, 0, 4);
    let done = ManualFence::new();
    done.complete();

    let first = seg.begin_write(eref(7), 3, None).unwrap();
    let first_region = first.region();
    seg.commit(first, &*done).unwrap();
    let before = seg.valid_words();

    let second = seg.begin_write(eref(7), 4, None).unwrap();
    assert_ne!(second.region(), first_region, "live slot must not be rewritten");
    assert_eq!(seg.valid_words(), before, "reserved slot stays invisible");
    let new_slot = second.slot();
    seg.commit(second, &*done).unwrap();

    assert!(!seg.is_valid(0) && seg.is_valid(new_slot));
    assert_eq!(seg.slot_of(EntryId::new(7)), Some(new_slot));
    assert_eq!(seg.valid_count(), 1);
    assert!(!seg.meta(0).unwrap().valid);
}

#[test]
fn tombstone_hides_entry() {
    let store = SegmentStore::new();
    let seg = segment(&store, 1, 10, 0, 2);
    let done = ManualFence::new();
    done.complete();
    let w = seg.begin_write(eref(1), 1, None).unwrap();
    seg.commit(w, &*done).unwrap();
    assert!(seg.tombstone(EntryId::new(1)));
    assert!(!seg.tombstone(EntryId::new(1)));
    assert_eq!(seg.valid_count(), 0);
}

#[test]
fn pending_commits_apply_in_submission_order() {
    let store = SegmentStore::new();
    let seg = segment(&store, 1, 10, 0, 4);
    let (slow, fast) = (ManualFence::new(), ManualFence::new());
    let a = seg.begin_write(eref(1), 1, None).unwrap();
    let b = seg.begin_write(eref(1), 2, None).unwrap();
    let (sa, sb) = (a.slot(), b.slot());
    seg.commit_after(a, slow.clone());
    seg.commit_after(b, fast.clone());
    fast.complete();
    assert_eq!(seg.settle(), 0);
    slow.complete();
    assert_eq!(seg.settle(), 2);
    assert!(!seg.is_valid(sa) && seg.is_valid(sb));
}

#[test]
fn segment_fills_up_and_checks_token_capacity() {
    let store = SegmentStore::new();
    let seg = segment(&store, 1, 10, 0, 1);
    assert!(matches!(
        seg.begin_write(eref(1), 33, None),
        Err(SegmentError::TokenOverflow { tokens: 33, max: 32 })
    ));
    seg.begin_write(eref(1), 1, None).unwrap();
    assert!(matches!(
        seg.begin_write(eref(2), 1, None),
        Err(SegmentError::SegmentFull { slots: 1 })
    ));
}

#[test]
fn encoded_slot_pads_with_finite_mask() {
    let bytes = encode_slot(SlotClass::Custom(4), 2, DType::F16, 1, &[1.0, 2.0]).unwrap();
    assert_eq!(bytes.len(), 4 * 2 * 2);
    let at = |i: usize| half::f16::from_le_bytes([bytes[2 * i], bytes[2 * i + 1]]).to_f32();
    assert_eq!((at(0), at(1)), (1.0, 2.0));
    assert_eq!(at(2), F16_MASK_VALUE);
    assert!(at(7).is_finite());

    let f32s = encode_slot(SlotClass::Custom(2), 1, DType::F32, 1, &[0.5]).unwrap();
    assert_eq!(f32s.len(), 8);
    assert!(matches!(
        encode_slot(SlotClass::T32, 2, DType::F32, 1, &[1.0]),
        Err(SegmentError::ShapeMismatch { expected: 2, got: 1 })
    ));
    assert!(matches!(
        encode_slot(SlotClass::T32, 1, DType::U8, 0, &[]),
        Err(SegmentError::UnsupportedDType { .. })
    ));
}
