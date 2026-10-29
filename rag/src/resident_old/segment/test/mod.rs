use super::mock::ManualFence;
use super::testutil::*;
use super::*;

#[test]
fn stride_follows_class_dim_and_dtype() {
    assert_eq!(SlotClass::T32.stride(384, DType::F16).unwrap(), 32 * 384 * 2);
    assert_eq!(SlotClass::T128.stride(384, DType::F32).unwrap(), 128 * 384 * 4);
    assert!(matches!(
        SlotClass::Custom(u16::MAX).stride(usize::MAX / 2, DType::F32),
        Err(SegmentError::GeometryOverflow)
    ));
}

#[test]
fn view_checks_capacity_and_addresses_slots() {
    let small = region(0, 100);
    assert!(matches!(
        SegmentView::new(small, 4, SlotClass::T32, 4, DType::F32, 1),
        Err(SegmentError::RegionTooSmall { .. })
    ));
    let view = SegmentView::new(region(4096, 1 << 20), 4, SlotClass::T32, 4, DType::F32, 1).unwrap();
    let r = view.slot_region(2).unwrap();
    assert_eq!(r.offset, 4096 + 2 * u64::from(view.stride));
    assert_eq!(r.len, u64::from(view.stride));
    assert!(view.slot_region(4).is_err());
    assert_eq!(view.desc().count, 4);
}

#[test]
fn table_encodes_one_record_per_segment() {
    let store = SegmentStore::new();
    let m = Manifest::new(1, vec![segment(&store, 1, 10, 0, 2), segment(&store, 2, 11, 1 << 16, 3)]);
    let table = m.table();
    assert_eq!(table.len(), 2);
    let bytes = table.encode();
    assert_eq!(bytes.len(), 48);
    assert_eq!(u64::from_le_bytes(bytes[24..32].try_into().unwrap()), 11);
    assert_eq!(u32::from_le_bytes(bytes[40..44].try_into().unwrap()), 3);
}

#[test]
fn publish_is_monotonic() {
    let store = SegmentStore::new();
    let m1 = store.current().successor(vec![]);
    store.publish(m1).unwrap();
    let stale = Manifest::new(1, vec![]);
    assert!(matches!(store.publish(stale), Err(SegmentError::StaleGeneration { gen: 1 })));
    assert_eq!(store.current().generation(), 1);
}

#[test]
fn lost_cas_is_retried() {
    let store = SegmentStore::new();
    let _armed = failpoint::arm("segment.publish_cas", 1, 3);
    store.publish(Manifest::new(1, vec![])).unwrap();
    assert_eq!(store.current().generation(), 1);
}

#[test]
fn concurrent_publishers_end_on_highest_generation() {
    let store = Arc::new(SegmentStore::new());
    let handles: Vec<_> = (1..=8u64)
        .map(|g| {
            let store = Arc::clone(&store);
            std::thread::spawn(move || {
                let _ = store.publish(Manifest::new(g, vec![]));
            })
        })
        .collect();
    for h in handles {
        h.join().unwrap();
    }
    assert_eq!(store.current().generation(), 8);
}

#[test]
fn slab_is_reclaimed_only_after_leases_and_fences() {
    let store = SegmentStore::new();
    let seg = segment(&store, 1, 10, 0, 2);
    let base = seg.view().base;
    store.publish(Manifest::new(1, vec![seg])).unwrap();

    let lease = store.pin();
    assert_eq!(lease.manifest().pins(), 1);
    let fence = ManualFence::new();
    lease.track(fence.clone());

    store.publish(Manifest::new(2, vec![])).unwrap();
    assert!(store.reclaim().is_empty(), "lease still held");

    let manifest_one = Arc::clone(&lease.manifest);
    drop(lease);
    assert_eq!(manifest_one.pins(), 0);
    drop(manifest_one);
    assert!(store.reclaim().is_empty(), "kernel work still in flight");

    fence.complete();
    assert_eq!(store.reclaim(), vec![base]);
    assert!(store.reclaim().is_empty());
}

#[test]
fn shared_segment_survives_generation_swap() {
    let store = SegmentStore::new();
    let kept = segment(&store, 1, 10, 0, 2);
    let dropped = segment(&store, 2, 11, 1 << 16, 2);
    let dropped_base = dropped.view().base;
    store.publish(Manifest::new(1, vec![kept.clone(), dropped])).unwrap();
    store.publish(Manifest::new(2, vec![kept])).unwrap();
    assert_eq!(store.reclaim(), vec![dropped_base]);
    assert_eq!(store.current().segments().len(), 1);
}

#[test]
fn cow_budget_counts_two_generations() {
    let store = SegmentStore::new();
    let seg = segment(&store, 1, 10, 0, 2);
    let bytes = seg.view().bytes();
    store.publish(Manifest::new(1, vec![seg])).unwrap();
    assert_eq!(store.cow_budget(bytes), 2 * bytes);
}
