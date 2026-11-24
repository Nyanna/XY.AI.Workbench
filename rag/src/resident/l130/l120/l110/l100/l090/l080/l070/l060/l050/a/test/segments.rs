use super::*;
use std::collections::HashSet;

static_assertions::assert_impl_all!(SegmentId: Send, Sync);
static_assertions::assert_impl_all!(SlotRef: Send, Sync);

#[test]
fn segment_ids_order_by_value() {
    assert!(SegmentId(1) < SegmentId(2));
    assert_eq!(SegmentId(3), SegmentId(3));
}

#[test]
fn slot_refs_order_by_segment_then_slot() {
    let a = SlotRef { segment: SegmentId(1), slot: Slot(9) };
    let b = SlotRef { segment: SegmentId(2), slot: Slot(0) };
    let c = SlotRef { segment: SegmentId(2), slot: Slot(1) };
    assert!(a < b);
    assert!(b < c);
}

#[test]
fn slot_refs_are_hashable() {
    let set: HashSet<SlotRef> = [
        SlotRef { segment: SegmentId(1), slot: Slot(1) },
        SlotRef { segment: SegmentId(1), slot: Slot(1) },
    ]
    .into_iter()
    .collect();
    assert_eq!(set.len(), 1);
}
