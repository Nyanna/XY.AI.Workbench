use super::*;
use crate::resident::index::memory::InMemoryIndex;
use crate::resident::kernel::mock::MockKernel;
use crate::resident::kernel::{BatchDesc, Kernel, KernelOutput, RegionSpan};
struct Zeros;
impl RangeSource for Zeros {
    fn read(&self, range: WrittenRange) -> Vec<u8> {
        vec![0; range.len as usize]
    }
}
fn id(n: u64) -> EntryId {
    EntryId::new(n)
}
fn bound(map: SlotMap, entries: &[Option<u64>]) -> SlotMap {
    for (slot, e) in entries.iter().enumerate() {
        if let Some(n) = e {
            map.bind(
                    slot as u32,
                    EntryRef {
                        entry: id(*n),
                        chunk: 0,
                    },
                )
                .unwrap();
        }
    }
    map
}
fn fixed_map() -> AddressMap {
    let mut map = AddressMap::new();
    let seg = bound(
        SlotMap::fixed(1000, 16, 4),
        &[Some(1), Some(2), Some(3), Some(4)],
    );
    assert!(map.add_segment(0, seg));
    map
}
fn hit(offset: u64) -> KernelHit {
    KernelHit { offset, score: 0.5 }
}
#[test]
fn freshly_written_address_resolves() {
    let index = InMemoryIndex::new();
    let mut map = fixed_map();
    let (mut q, mut t) = (UpdateQueue::new(), ResidencyTracker::new());
    let writes = [
        WrittenRange {
            offset: 1000,
            len: 64,
        },
    ];
    let hits = run_slot(
        &writes,
        &[hit(1020)],
        &Zeros,
        &index,
        &mut map,
        &mut q,
        &mut t,
    );
    assert_eq!(hits, vec![Hit { entry : id(2), score : 0.5, outdated : false }]);
    assert_eq!(t.len(), 4);
    assert_eq!(t.known_stamp(id(2)), index.stamp(id(2)));
}
#[test]
fn removed_entry_is_discarded_and_pending_update_marks_outdated() {
    let index = InMemoryIndex::new();
    let mut map = fixed_map();
    let (mut q, mut t) = (UpdateQueue::new(), ResidencyTracker::new());
    for n in 1..=3 {
        index.apply(id(n), &[0]);
    }
    index.remove(id(1));
    q.push(Update::Unbind { entry: id(3) });
    let hits = [hit(1000), hit(1016), hit(1032)];
    let out = run_slot(&[], &hits, &Zeros, &index, &mut map, &mut q, &mut t);
    assert_eq!(out.len(), 2);
    assert!(! out[0].outdated && out[0].entry == id(2));
    assert!(out[1].outdated && out[1].entry == id(3));
}
#[test]
fn mapping_change_takes_effect_in_the_next_batch() {
    let index = InMemoryIndex::new();
    index.apply(id(3), &[0]);
    let mut map = fixed_map();
    let (mut q, mut t) = (UpdateQueue::new(), ResidencyTracker::new());
    let stamp = index.stamp(id(3)).unwrap();
    let target = SlotRef { segment: 0, slot: 0 };
    t.insert(id(3), SlotRef { segment: 0, slot: 2 }, stamp);
    q.push(Update::Bind {
        entry: id(3),
        slot: target,
        stamp,
    });
    let first = run_slot(
        &[],
        &[hit(1032)],
        &Zeros,
        &index,
        &mut map,
        &mut q,
        &mut t,
    );
    assert_eq!(first[0].entry, id(3));
    assert!(first[0].outdated);
    assert_eq!(map.resolve(1032), None);
    assert_eq!(map.resolve(1000), Some(id(3)));
    let second = run_slot(
        &[],
        &[hit(1000)],
        &Zeros,
        &index,
        &mut map,
        &mut q,
        &mut t,
    );
    assert!(! second[0].outdated);
}
#[test]
fn variable_layout_resolves_at_segment_boundaries() {
    let mut map = AddressMap::new();
    let seg = bound(
        SlotMap::variable(vec![500, 510, 510, 535]),
        &[Some(1), Some(2), Some(3)],
    );
    assert!(map.add_segment(7, seg));
    assert_eq!(map.resolve(500), Some(id(1)));
    assert_eq!(map.resolve(509), Some(id(1)));
    assert_eq!(map.resolve(510), Some(id(3)));
    assert_eq!(map.resolve(534), Some(id(3)));
    assert_eq!(map.resolve(535), None);
    assert_eq!(map.resolve(499), None);
}
#[test]
fn snapshot_is_stable_while_segments_are_appended() {
    let mut map = fixed_map();
    let snapshot = map.clone();
    assert!(
        map.add_segment(1, bound(SlotMap::fixed(2000, 8, 2), & [Some(9), None]))
    );
    assert_eq!(map.resolve(2000), Some(id(9)));
    assert_eq!(snapshot.resolve(2000), None);
    assert!(! map.add_segment(2, SlotMap::fixed(1010, 8, 1)));
    assert!(map.remove_segment(1));
    assert_eq!(map.resolve(2000), None);
}
#[test]
fn unmapped_write_is_retried_in_the_next_slot() {
    let index = InMemoryIndex::new();
    let mut map = AddressMap::new();
    let (mut q, mut t) = (UpdateQueue::new(), ResidencyTracker::new());
    let writes = [
        WrittenRange {
            offset: 1000,
            len: 16,
        },
    ];
    run_slot(&writes, &[], &Zeros, &index, &mut map, &mut q, &mut t);
    assert_eq!(q.deferred_writes(), 1);
    assert!(t.is_empty());
    map = fixed_map();
    run_slot(&[], &[], &Zeros, &index, &mut map, &mut q, &mut t);
    assert_eq!(q.deferred_writes(), 0);
    assert_eq!(t.len(), 1);
}
#[test]
fn kernel_fake_drives_writes_then_hits() {
    let index = InMemoryIndex::new();
    let mut map = fixed_map();
    let (mut q, mut t) = (UpdateQueue::new(), ResidencyTracker::new());
    let writer = MockKernel::embedding(1 << 20);
    let b = writer.begin(&BatchDesc::default()).unwrap();
    writer
        .run(
            b,
            &[
                RegionSpan {
                    offset: 1000,
                    len: 48,
                },
            ],
        )
        .unwrap();
    let writes = writer.end(b).unwrap();
    let scorer = MockKernel::scoring();
    let b = scorer
        .begin(
            &BatchDesc {
                queries: 1,
                ..Default::default()
            },
        )
        .unwrap();
    let KernelOutput::Hits(hits) = scorer
        .run(
            b,
            &[
                RegionSpan {
                    offset: 1040,
                    len: 16,
                },
            ],
        )
        .unwrap() else {
        panic!("scoring kernel returned a partial result");
    };
    scorer.end(b).unwrap();
    let out = run_slot(&writes, &hits, &Zeros, &index, &mut map, &mut q, &mut t);
    assert_eq!(out.len(), 1);
    assert_eq!(out[0].entry, id(3));
    assert_eq!(t.len(), 3);
}
#[test]
fn stale_batch_epoch_is_reported_without_side_effects() {
    let index = InMemoryIndex::new();
    let store = MappingStore::new();
    let mut next = store.snapshot().successor();
    assert!(next.add_segment(0, bound(SlotMap::fixed(1000, 16, 1), & [Some(1)])));
    store.publish(next).unwrap();
    let (mut q, mut t) = (UpdateQueue::new(), ResidencyTracker::new());
    let writes = [
        WrittenRange {
            offset: 1000,
            len: 16,
        },
    ];
    let old = Snapshot { root: 0, epoch: 0 };
    let res = run_slot_at(
        &store,
        &old,
        &writes,
        &[],
        &Zeros,
        &index,
        &mut q,
        &mut t,
    );
    assert!(
        matches!(res, Err(ManagerError::StaleMapping { base : 0, published : 1 }))
    );
    assert!(t.is_empty() && index.stamp(id(1)).is_none());
    let now = Snapshot { root: 0, epoch: 1 };
    run_slot_at(&store, &now, &writes, &[], &Zeros, &index, &mut q, &mut t).unwrap();
    assert_eq!(t.len(), 1);
}
#[test]
fn publish_requires_the_next_epoch() {
    let store = MappingStore::new();
    let skipped = store.snapshot().successor().successor();
    assert!(
        matches!(store.publish(skipped), Err(ManagerError::StaleMapping { .. }))
    );
    assert!(store.publish(store.snapshot().successor()).is_ok());
    assert_eq!(store.snapshot().epoch(), 1);
}
