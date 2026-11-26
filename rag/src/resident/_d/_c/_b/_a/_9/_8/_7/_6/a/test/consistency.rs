use super::*;
use std::collections::{BTreeMap, HashMap};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::a::table_swap::{
    HostRootSlot, ReaderId, RootSlot, TableSink,
};
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::a::allocator::Allocator;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::a::bookkeeping::Bookkeeping;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::a::region_handle::{
    Generation, RegionHandle,
};
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::a::arenas::{
    ArenaCapacities, ArenaKind, ArenaSet, Budget,
};
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::a::index_contract::{
    Payload2, StampObserver, StampSignal, AccessMethod,
};
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::_1::device_lane_event::DeviceKind;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::_1::layout::DType;
struct MemIndex {
    state: Mutex<(u64, BTreeMap<u64, (Stamp, Payload2)>)>,
    signal: Arc<StampSignal>,
}
impl MemIndex {
    fn new() -> Self {
        MemIndex {
            state: Mutex::new((0, BTreeMap::new())),
            signal: StampSignal::new(Stamp(0)),
        }
    }
}
impl Index for MemIndex {
    fn dtype(&self) -> DType {
        DType::F32
    }
    fn access_methods(&self) -> &[AccessMethod] {
        &[]
    }
    fn stamp(&self, id: EntryId) -> Option<Stamp> {
        self.state.lock().unwrap().1.get(&id.0).map(|e| e.0)
    }
    fn stamps(&self) -> Vec<(EntryId, Stamp)> {
        let g = self.state.lock().unwrap();
        g.1.iter().map(|(k, v)| (EntryId(*k), v.0)).collect()
    }
    fn read(&self, id: EntryId) -> Payload2 {
        self.state.lock().unwrap().1.get(&id.0).map(|e| e.1.clone()).unwrap_or_default()
    }
    fn write(&self, id: EntryId, p: PayloadRef<'_>) -> Result<Stamp, IndexError> {
        let elem = self.dtype().size_in_bytes();
        if p.vector.len() % elem != 0 {
            return Err(IndexError::InvalidVectorLength {
                len: p.vector.len(),
                elem_size: elem,
            });
        }
        let mut g = self.state.lock().unwrap();
        g.0 += 1;
        let stamp = Stamp(g.0);
        let payload = Payload2 {
            vector: p.vector.into(),
            meta: p.meta.into(),
        };
        g.1.insert(id.0, (stamp, payload));
        self.signal.publish(stamp);
        Ok(stamp)
    }
    fn delete(&self, id: EntryId) -> Result<Option<Stamp>, IndexError> {
        let mut g = self.state.lock().unwrap();
        if g.1.remove(&id.0).is_none() {
            return Ok(None);
        }
        g.0 += 1;
        let stamp = Stamp(g.0);
        self.signal.publish(stamp);
        Ok(Some(stamp))
    }
    fn subscribe(&self) -> StampObserver {
        self.signal.observer()
    }
}
fn pr<'a>(v: &'a [u8], m: &'a [u8]) -> PayloadRef<'a> {
    PayloadRef { vector: v, meta: m }
}
const V: [u8; 4] = [0; 4];
struct FlagSink {
    fail: AtomicBool,
}
impl TableSink for FlagSink {
    fn write_visible(
        &self,
        _region: &RegionHandle,
        _bytes: &[u8],
    ) -> Result<(), String> {
        if self.fail.load(Ordering::SeqCst) { Err("boom".to_string()) } else { Ok(()) }
    }
}
fn make_allocator(capacity: u64) -> Arc<Allocator> {
    let mut caps = ArenaCapacities::default();
    caps.set(ArenaKind::IndexSegments, capacity);
    let set = Arc::new(ArenaSet::new(caps));
    let budget = Budget {
        free: u64::MAX / 2,
        total: u64::MAX / 2,
        generation_reserve: 0,
    };
    let arena = set.ensure(ArenaKind::IndexSegments, &budget).unwrap();
    let bookkeeping = Arc::new(Bookkeeping::new(set));
    Arc::new(Allocator::new(arena, DeviceKind::Gpu, bookkeeping))
}
fn seg(offset: u64) -> RegionHandle {
    RegionHandle::new(ArenaKind::IndexSegments, offset, 64, Generation(1)).unwrap()
}
fn publisher() -> (TablePublisher, Arc<TableSwap>, Arc<FlagSink>) {
    let sink = Arc::new(FlagSink {
        fail: AtomicBool::new(false),
    });
    let swap = Arc::new(
        TableSwap::new(
            make_allocator(1 << 20),
            Arc::clone(&sink) as Arc<dyn TableSink>,
            Arc::new(HostRootSlot::new()) as Arc<dyn RootSlot>,
        ),
    );
    (TablePublisher::new(Arc::clone(&swap)), swap, sink)
}
fn ident(id: u64, raw: u64) -> EntryIdentity {
    EntryIdentity::new(EntryId(id), Stamp(raw))
}
#[test]
fn identity_of_reads_index_stamp() {
    let ix = MemIndex::new();
    assert_eq!(EntryIdentity::of(& ix, EntryId(1)), None);
    let s = ix.write(EntryId(1), pr(&V, b"")).unwrap();
    assert_eq!(
        EntryIdentity::of(& ix, EntryId(1)), Some(EntryIdentity::new(EntryId(1), s))
    );
}
#[test]
fn check_current_stale_gone() {
    let ix = MemIndex::new();
    let s = ix.write(EntryId(1), pr(&V, b"")).unwrap();
    let held = EntryIdentity::new(EntryId(1), s);
    assert_eq!(check(& ix, & held), Verdict::Current);
    let s2 = ix.write(EntryId(1), pr(&V, b"x")).unwrap();
    assert_eq!(
        check(& ix, & held), Verdict::Stale { winner : EntryIdentity::new(EntryId(1), s2)
        }
    );
    assert_eq!(check(& ix, & ident(9, 1)), Verdict::Gone);
}
#[test]
fn stale_when_held_stamp_is_numerically_larger() {
    let ix = MemIndex::new();
    let s = ix.write(EntryId(1), pr(&V, b"")).unwrap();
    let held = ident(1, s.0 + 100);
    assert_eq!(
        check(& ix, & held), Verdict::Stale { winner : EntryIdentity::new(EntryId(1), s)
        }
    );
}
#[test]
fn deleted_and_recreated_is_stale() {
    let ix = MemIndex::new();
    let s = ix.write(EntryId(1), pr(&V, b"")).unwrap();
    let held = EntryIdentity::new(EntryId(1), s);
    ix.delete(EntryId(1)).unwrap();
    assert_eq!(check(& ix, & held), Verdict::Gone);
    ix.write(EntryId(1), pr(&V, b"")).unwrap();
    assert!(matches!(check(& ix, & held), Verdict::Stale { .. }));
}
#[test]
fn check_does_not_modify_index() {
    let ix = MemIndex::new();
    ix.write(EntryId(1), pr(&V, b"")).unwrap();
    let before = ix.stamps();
    check(&ix, &ident(1, 999));
    assert_eq!(ix.stamps(), before);
}
#[test]
fn check_many_order_duplicates_and_empty() {
    let ix = MemIndex::new();
    let a = ix.write(EntryId(1), pr(&V, b"")).unwrap();
    let b = ix.write(EntryId(2), pr(&V, b"")).unwrap();
    assert!(check_many(& ix, & []).is_empty());
    let held = [
        EntryIdentity::new(EntryId(2), b),
        ident(7, 1),
        EntryIdentity::new(EntryId(1), a),
        ident(1, 999),
        EntryIdentity::new(EntryId(1), a),
    ];
    let v = check_many(&ix, &held);
    assert_eq!(v.len(), 5);
    assert_eq!(v[0], Verdict::Current);
    assert_eq!(v[1], Verdict::Gone);
    assert_eq!(v[2], Verdict::Current);
    assert_eq!(v[3], Verdict::Stale { winner : EntryIdentity::new(EntryId(1), a) });
    assert_eq!(v[4], Verdict::Current);
    for (h, verdict) in held.iter().zip(&v) {
        assert_eq!(check(& ix, h), * verdict);
    }
}
#[test]
fn writer_returns_identity_of_this_write() {
    let ix = Arc::new(MemIndex::new());
    let w = IndexWriter::new(Arc::clone(&ix) as Arc<dyn Index>);
    let a = w.write(EntryId(1), pr(&V, b"")).unwrap();
    let b = w.write(EntryId(1), pr(&V, b"")).unwrap();
    assert_ne!(a.stamp, b.stamp);
    assert_eq!(EntryIdentity::of(&* ix, EntryId(1)), Some(b));
    assert_eq!(check(&* ix, & a), Verdict::Stale { winner : b });
}
#[test]
fn writer_passes_errors_through_and_keeps_state() {
    let ix = Arc::new(MemIndex::new());
    let w = IndexWriter::new(Arc::clone(&ix) as Arc<dyn Index>);
    let err = w.write(EntryId(1), pr(&[0; 3], b"")).unwrap_err();
    assert_eq!(err, IndexError::InvalidVectorLength { len : 3, elem_size : 4 });
    assert!(ix.stamps().is_empty());
}
#[test]
fn writer_write_many_in_input_order() {
    let ix = Arc::new(MemIndex::new());
    let w = IndexWriter::new(Arc::clone(&ix) as Arc<dyn Index>);
    let items = [(EntryId(5), pr(&V, b"")), (EntryId(2), pr(&V, b""))];
    let ids = w.write_many(&items).unwrap();
    assert_eq!(ids.len(), 2);
    assert_eq!(ids[0].id, EntryId(5));
    assert_eq!(ids[1].id, EntryId(2));
    for i in &ids {
        assert_eq!(check(&* ix, i), Verdict::Current);
    }
}
#[test]
fn writer_delete() {
    let ix = Arc::new(MemIndex::new());
    let w = IndexWriter::new(Arc::clone(&ix) as Arc<dyn Index>);
    assert_eq!(w.delete(EntryId(1)).unwrap(), None);
    w.write(EntryId(1), pr(&V, b"")).unwrap();
    assert!(w.delete(EntryId(1)).unwrap().is_some());
    assert_eq!(ix.stamp(EntryId(1)), None);
}
#[test]
fn writer_survives_poisoned_lock() {
    let ix = Arc::new(MemIndex::new());
    let w = Arc::new(IndexWriter::new(Arc::clone(&ix) as Arc<dyn Index>));
    let w2 = Arc::clone(&w);
    let _ = thread::spawn(move || {
            let _g = w2.serial.lock().unwrap();
            panic!("poison");
        })
        .join();
    assert!(w.write(EntryId(1), pr(& V, b"")).is_ok());
}
#[test]
fn concurrent_writers_end_in_consistent_state() {
    let ix = Arc::new(MemIndex::new());
    let w = Arc::new(IndexWriter::new(Arc::clone(&ix) as Arc<dyn Index>));
    let handles: Vec<_> = (0..4)
        .map(|_| {
            let w = Arc::clone(&w);
            thread::spawn(move || {
                (0..50)
                    .map(|_| w.write(EntryId(1), pr(&V, b"")).unwrap())
                    .collect::<Vec<_>>()
            })
        })
        .collect();
    let mut all = Vec::new();
    for h in handles {
        all.extend(h.join().unwrap());
    }
    let mut raws: Vec<u64> = all.iter().map(|i| i.stamp.0).collect();
    raws.sort_unstable();
    raws.dedup();
    assert_eq!(raws.len(), 200);
    let current = all.iter().filter(|i| check(&*ix, i) == Verdict::Current).count();
    assert_eq!(current, 1);
}
#[test]
fn apply_publishes_copy_on_write() {
    let (p, swap, _) = publisher();
    assert_eq!(p.apply(| t | t.add(seg(0))).unwrap(), 1);
    let first = swap.current_table().unwrap();
    assert_eq!(p.apply(| t | t.add(seg(64))).unwrap(), 2);
    assert_eq!(swap.published_epoch(), 2);
    assert_eq!(first.len(), 1);
    assert_eq!(swap.current_table().unwrap().len(), 2);
}
#[test]
fn apply_retries_stale_mapping_up_to_three_times() {
    let (p, swap, _) = publisher();
    let calls = Mutex::new(0usize);
    let epoch = p
        .apply(|t| {
            let mut n = calls.lock().unwrap();
            *n += 1;
            if *n == 1 {
                swap.publish(swap.next_table()).unwrap();
            }
            t.add(seg(0));
        })
        .unwrap();
    assert_eq!(* calls.lock().unwrap(), 2);
    assert_eq!(epoch, 2);
    let calls = Mutex::new(0usize);
    let err = p
        .apply(|_| {
            *calls.lock().unwrap() += 1;
            swap.publish(swap.next_table()).unwrap();
        })
        .unwrap_err();
    assert!(matches!(err, TableSwapError::StaleMapping { .. }));
    assert_eq!(* calls.lock().unwrap(), 3);
}
#[test]
fn failed_apply_does_not_queue_mapping_or_retry_other_errors() {
    let (p, swap, sink) = publisher();
    sink.fail.store(true, Ordering::SeqCst);
    let calls = Mutex::new(0usize);
    let err = p
        .apply_with_mapping(
            |_| *calls.lock().unwrap() += 1,
            Box::new(|_| panic!("must not run")),
        )
        .unwrap_err();
    assert!(matches!(err, TableSwapError::Write(_)));
    assert_eq!(* calls.lock().unwrap(), 1);
    assert_eq!(p.pending_mappings(), 0);
    assert_eq!(swap.published_epoch(), 0);
}
#[test]
fn mappings_run_only_in_gap_in_epoch_order() {
    let (p, _, _) = publisher();
    let log: Arc<Mutex<Vec<Epoch>>> = Arc::default();
    for i in 0..3u64 {
        let log = Arc::clone(&log);
        p.apply_with_mapping(
                |t| t.add(seg(i * 64)),
                Box::new(move |e| log.lock().unwrap().push(e)),
            )
            .unwrap();
    }
    assert!(log.lock().unwrap().is_empty());
    assert_eq!(p.pending_mappings(), 3);
    let r = p.run_gap();
    assert_eq!(r.mappings_applied, 3);
    assert_eq!(* log.lock().unwrap(), vec![1, 2, 3]);
    assert_eq!(p.pending_mappings(), 0);
    assert_eq!(p.run_gap(), GapReport2::default());
}
#[test]
fn mapping_sees_published_table() {
    let (p, swap, _) = publisher();
    let seen = Arc::new(Mutex::new(None));
    let (s2, sw2) = (Arc::clone(&seen), Arc::clone(&swap));
    p.apply_with_mapping(
            |t| t.add(seg(0)),
            Box::new(move |e| *s2.lock().unwrap() = Some((e, sw2.published_epoch()))),
        )
        .unwrap();
    p.run_gap();
    let (e, published) = seen.lock().unwrap().unwrap();
    assert!(published >= e);
}
#[test]
fn gap_reclaims_only_after_readers_advance() {
    let (p, swap, _) = publisher();
    p.apply(|t| t.add(seg(0))).unwrap();
    let reader: ReaderId = swap.register_reader();
    let guard = swap.acquire(reader).unwrap();
    p.apply(|t| t.add(seg(64))).unwrap();
    assert_eq!(p.run_gap().reclaimed, 0);
    drop(guard);
    swap.report_epoch(reader, 2).unwrap();
    assert_eq!(p.run_gap().reclaimed, 1);
    assert_eq!(p.run_gap().reclaimed, 0);
}
#[test]
fn mapping_panic_drops_queue_and_locks_recover() {
    let (p, _, _) = publisher();
    let p = Arc::new(p);
    p.apply_with_mapping(|t| t.add(seg(0)), Box::new(|_| panic!("mapping"))).unwrap();
    p.apply_with_mapping(|t| t.add(seg(64)), Box::new(|_| {})).unwrap();
    let p2 = Arc::clone(&p);
    assert!(thread::spawn(move || p2.run_gap()).join().is_err());
    assert_eq!(p.pending_mappings(), 0);
    assert_eq!(p.run_gap(), GapReport2 { mappings_applied : 0, reclaimed : 1 });
    assert_eq!(p.run_gap(), GapReport2::default());
    assert!(p.apply(| t | t.add(seg(128))).is_ok());
}
#[test]
fn parallel_apply_and_gap_apply_each_mapping_once() {
    let (p, swap, _) = publisher();
    let p = Arc::new(p);
    let counts: Arc<Mutex<HashMap<Epoch, usize>>> = Arc::default();
    let appliers: Vec<_> = (0..4)
        .map(|t| {
            let (p, counts) = (Arc::clone(&p), Arc::clone(&counts));
            thread::spawn(move || {
                for i in 0..25u64 {
                    let counts = Arc::clone(&counts);
                    p.apply_with_mapping(
                            |tb| tb.add(seg((t * 25 + i) * 64)),
                            Box::new(move |e| {
                                *counts.lock().unwrap().entry(e).or_default() += 1;
                            }),
                        )
                        .unwrap();
                }
            })
        })
        .collect();
    let gappers: Vec<_> = (0..2)
        .map(|_| {
            let p = Arc::clone(&p);
            thread::spawn(move || {
                for _ in 0..50 {
                    p.run_gap();
                }
            })
        })
        .collect();
    for h in appliers.into_iter().chain(gappers) {
        h.join().unwrap();
    }
    p.run_gap();
    let counts = counts.lock().unwrap();
    assert_eq!(swap.published_epoch(), 100);
    assert_eq!(counts.len(), 100);
    assert!(counts.values().all(|& n | n == 1));
}
