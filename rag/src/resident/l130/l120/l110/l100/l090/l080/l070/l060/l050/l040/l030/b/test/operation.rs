use super::*;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::l010::region_pinning::PinCount;
use std::collections::HashSet;
use std::thread;
fn reqs() -> ResourceRequirements {
    ResourceRequirements {
        resource: Some("gpu".into()),
        model: Some("m".into()),
        mode: None,
    }
}
#[test]
fn params_defaults_and_builders() {
    let p = OperationParams::new(reqs());
    assert_eq!(p.top_k(), None);
    assert_eq!(p.dimension(), None);
    assert_ne!(p, p.clone().with_top_k(3));
    assert_ne!(p.clone().with_top_k(3), p.clone().with_top_k(4));
    assert_eq!(p.clone().with_dimension(8).dimension(), Some(8));
    assert_eq!(p.requirements(), & reqs());
}
#[test]
fn descriptor_is_key() {
    let a = OperationDescriptor::new(OperationKind::TopK, OperationParams::new(reqs()));
    let b = OperationDescriptor::new(OperationKind::TopK, OperationParams::new(reqs()));
    let c = OperationDescriptor::new(
        OperationKind::Pooling,
        OperationParams::new(reqs()),
    );
    assert_eq!(a, b);
    assert_ne!(a, c);
    let mut set = HashSet::new();
    set.insert(a.clone());
    assert!(set.contains(& b));
    assert!(! set.contains(& c));
    assert_eq!(a.requirements(), a.params().requirements());
}
#[test]
fn slot_set_once_and_shape_checked() {
    let s = RamSlot::empty(DataShape::Floats);
    assert!(s.get().is_none());
    assert!(s.set(RamData::TextList(vec![])).is_err());
    assert!(s.set(RamData::Floats(vec![1.0])).is_ok());
    let back = s.set(RamData::Floats(vec![2.0])).unwrap_err();
    assert_eq!(back, RamData::Floats(vec![2.0]));
    assert_eq!(s.get(), Some(& RamData::Floats(vec![1.0])));
    let c = s.clone();
    assert_eq!(c.get(), s.get());
}
#[test]
fn slot_concurrent_single_winner() {
    let s = RamSlot::empty(DataShape::Floats);
    let wins: usize = (0..8)
        .map(|i| {
            let s = s.clone();
            thread::spawn(move || s.set(RamData::Floats(vec![i as f32])).is_ok())
        })
        .collect::<Vec<_>>()
        .into_iter()
        .map(|h| h.join().unwrap() as usize)
        .sum();
    assert_eq!(wins, 1);
}
#[test]
fn filled_and_shapes() {
    let d = RamData::HitList(vec![(EntryId(1), 0.5)]);
    let s = RamSlot::filled(d.clone());
    assert_eq!(s.shape(), DataShape::HitList);
    assert_eq!(s.get(), Some(& d));
    let r = RamData::ScoredHitResult(ScoredHitResult {
        scored_count: 1,
        dropped_count: 0,
        hits: vec![],
    });
    assert_eq!(r.shape(), DataShape::ScoredHitResult);
}
#[test]
fn device_region_keeps_pin_alive() {
    let count = PinCount::default();
    let pin = Arc::new(Pin::new(7u32, &count));
    let io: IoRegion<u32> = IoRegion::Device {
        pin,
        shape: DataShape::Floats,
    };
    let io2 = io.clone();
    assert_eq!(io.shape(), DataShape::Floats);
    assert_eq!(count.count(), 1);
    drop(io);
    assert_eq!(count.count(), 1);
    drop(io2);
    assert!(count.is_unpinned());
}
#[test]
fn operation_accessors() {
    let desc = OperationDescriptor::new(
        OperationKind::Embedding,
        OperationParams::new(reqs()).with_dimension(4),
    );
    let op: Operation<u32> = Operation::new(
        TaskId(9),
        -2,
        desc.clone(),
        IoRegion::Ram(RamSlot::empty(DataShape::TextList)),
        IoRegion::Ram(RamSlot::empty(DataShape::Floats)),
    );
    assert_eq!(op.task_id(), TaskId(9));
    assert_eq!(op.priority(), - 2);
    assert_eq!(op.descriptor(), & desc);
    assert_eq!(op.kind(), OperationKind::Embedding);
    assert_eq!(op.params().dimension(), Some(4));
    assert_eq!(op.input().shape(), DataShape::TextList);
    assert_eq!(op.output().shape(), DataShape::Floats);
}
#[test]
fn send_sync() {
    fn check<T: Send + Sync>() {}
    check::<Operation<u32>>();
    check::<RamSlot>();
    check::<OperationDescriptor>();
}
