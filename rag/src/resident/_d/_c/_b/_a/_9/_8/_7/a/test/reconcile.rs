use super::*;
use std::collections::VecDeque;
use std::sync::atomic::{AtomicU8, Ordering};
use std::thread;
use std::time::Instant;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::a::segments::SegmentId;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::a::kernel_layout::Slot;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::a::index_contract::{
    AccessMethod, IndexError, PayloadRef, StampSignal,
};
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::_1::layout::DType;
struct MemIndex {
    state: Mutex<(u64, BTreeMap<u64, (Stamp, Payload)>)>,
    signal: Arc<StampSignal>,
}
impl MemIndex {
    fn new() -> Arc<MemIndex> {
        Arc::new(MemIndex {
            state: Mutex::new((0, BTreeMap::new())),
            signal: StampSignal::new(Stamp(0)),
        })
    }
    fn put(&self, id: u64, tag: u8) -> Stamp {
        let stamp = {
            let mut st = self.state.lock().unwrap();
            st.0 += 1;
            let stamp = Stamp(st.0);
            let payload = Payload {
                vector: vec![tag].into(),
                meta: Vec::new().into(),
            };
            st.1.insert(id, (stamp, payload));
            stamp
        };
        self.signal.publish(stamp);
        stamp
    }
    fn del(&self, id: u64) -> Option<Stamp> {
        let stamp = {
            let mut st = self.state.lock().unwrap();
            st.1.remove(&id)?;
            st.0 += 1;
            Stamp(st.0)
        };
        self.signal.publish(stamp);
        Some(stamp)
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
        let st = self.state.lock().unwrap();
        st.1.iter().map(|(k, v)| (EntryId(*k), v.0)).collect()
    }
    fn read(&self, id: EntryId) -> Payload {
        let st = self.state.lock().unwrap();
        st.1.get(&id.0).map(|e| e.1.clone()).unwrap_or_default()
    }
    fn write(&self, id: EntryId, payload: PayloadRef<'_>) -> Result<Stamp, IndexError> {
        Ok(self.put(id.0, payload.vector.first().copied().unwrap_or(0)))
    }
    fn delete(&self, id: EntryId) -> Result<Option<Stamp>, IndexError> {
        Ok(self.del(id.0))
    }
    fn subscribe(&self) -> StampObserver {
        self.signal.observer()
    }
}
struct Scripted {
    script: Mutex<(VecDeque<Option<Stamp>>, Option<Stamp>)>,
    log: Mutex<Vec<&'static str>>,
    reads: AtomicU8,
    signal: Arc<StampSignal>,
}
impl Scripted {
    fn new(script: &[Option<u64>]) -> Scripted {
        Scripted {
            script: Mutex::new((script.iter().map(|s| s.map(Stamp)).collect(), None)),
            log: Mutex::new(Vec::new()),
            reads: AtomicU8::new(0),
            signal: StampSignal::new(Stamp(0)),
        }
    }
    fn log(&self) -> Vec<&'static str> {
        self.log.lock().unwrap().clone()
    }
}
impl Index for Scripted {
    fn dtype(&self) -> DType {
        DType::F32
    }
    fn access_methods(&self) -> &[AccessMethod] {
        &[]
    }
    fn stamp(&self, _id: EntryId) -> Option<Stamp> {
        self.log.lock().unwrap().push("stamp");
        let mut s = self.script.lock().unwrap();
        if let Some(next) = s.0.pop_front() {
            s.1 = next;
        }
        s.1
    }
    fn stamps(&self) -> Vec<(EntryId, Stamp)> {
        Vec::new()
    }
    fn read(&self, _id: EntryId) -> Payload {
        self.log.lock().unwrap().push("read");
        let n = self.reads.fetch_add(1, Ordering::SeqCst) + 1;
        Payload {
            vector: vec![n].into(),
            meta: Vec::new().into(),
        }
    }
    fn write(
        &self,
        _id: EntryId,
        _payload: PayloadRef<'_>,
    ) -> Result<Stamp, IndexError> {
        Err(IndexError::NotFound)
    }
    fn delete(&self, _id: EntryId) -> Result<Option<Stamp>, IndexError> {
        Ok(None)
    }
    fn subscribe(&self) -> StampObserver {
        self.signal.observer()
    }
}
fn setup() -> (Arc<MemIndex>, Arc<dyn Index>) {
    let mem = MemIndex::new();
    let idx: Arc<dyn Index> = mem.clone();
    (mem, idx)
}
fn slot(n: u32) -> SlotRef {
    SlotRef {
        segment: SegmentId(0),
        slot: Slot(n),
    }
}
fn id(n: u64) -> EntryId {
    EntryId(n)
}
fn assert_send_sync<T: Send + Sync>() {}
#[test]
fn types_are_send_sync() {
    assert_send_sync::<ResidencyDirectory>();
    assert_send_sync::<Reconciler>();
    assert_send_sync::<Reconciliation>();
    assert_send_sync::<LoadedEntry>();
}
#[test]
fn directory_insert_replace_remove() {
    let dir = ResidencyDirectory::new();
    assert!(dir.is_empty());
    assert_eq!(dir.insert(id(1), slot(0), Stamp(5)), None);
    let prev = dir.insert(id(1), slot(3), Stamp(7)).unwrap();
    assert_eq!(prev.slot, slot(0));
    assert_eq!(prev.stamp, Stamp(5));
    assert_eq!(dir.len(), 1);
    assert_eq!(dir.get(id(1)).unwrap().slot, slot(3));
    assert_eq!(dir.stamp(id(1)), Some(Stamp(7)));
    assert_eq!(dir.remove(id(1)).unwrap().slot, slot(3));
    assert_eq!(dir.remove(id(1)), None);
    assert_eq!(dir.get(id(1)), None);
    assert_eq!(dir.stamp(id(1)), None);
    assert!(dir.is_empty());
}
#[test]
fn directory_set_known_stamp_keeps_slot() {
    let dir = ResidencyDirectory::new();
    assert!(! dir.set_known_stamp(id(1), Stamp(9)));
    assert!(dir.is_empty());
    dir.insert(id(1), slot(4), Stamp(1));
    assert!(dir.set_known_stamp(id(1), Stamp(9)));
    let r = dir.get(id(1)).unwrap();
    assert_eq!(r.slot, slot(4));
    assert_eq!(r.stamp, Stamp(9));
}
#[test]
fn directory_iter_is_sorted_snapshot() {
    let dir = ResidencyDirectory::new();
    for n in [5u64, 1, 9, 3] {
        dir.insert(id(n), slot(n as u32), Stamp(n));
    }
    let snap = dir.iter();
    dir.remove(id(5));
    dir.insert(id(7), slot(7), Stamp(7));
    let ids: Vec<u64> = snap.iter().map(|(i, _)| i.0).collect();
    assert_eq!(ids, vec![1, 3, 5, 9]);
    assert_eq!(dir.len(), 4);
}
#[test]
fn directory_survives_poisoned_lock() {
    let dir = Arc::new(ResidencyDirectory::new());
    let d2 = dir.clone();
    let _ = thread::spawn(move || {
            let _guard = d2.map.lock().unwrap();
            panic!("poison");
        })
        .join();
    assert!(dir.map.is_poisoned());
    assert_eq!(dir.insert(id(1), slot(0), Stamp(1)), None);
    assert_eq!(dir.len(), 1);
}
#[test]
fn reconcile_empty_directory_is_clean() {
    let (mem, idx) = setup();
    mem.put(1, 1);
    let mut r = Reconciler::new(&idx);
    let dir = ResidencyDirectory::new();
    let rec = r.reconcile(&dir).unwrap();
    assert!(rec.clean);
    assert!(rec.removed.is_empty() && rec.reload.is_empty());
    assert_eq!(rec.current, 0);
}
#[test]
fn reconcile_classifies_entries() {
    let (mem, idx) = setup();
    let dir = ResidencyDirectory::new();
    for n in 1..=5u64 {
        let s = mem.put(n, 1);
        dir.insert(id(n), slot(n as u32), s);
    }
    mem.put(100, 1);
    let mut r = Reconciler::new(&idx);
    mem.del(2);
    mem.put(4, 2);
    mem.del(5);
    mem.put(5, 2);
    let before = dir.iter();
    let rec = r.reconcile(&dir).unwrap();
    assert_eq!(rec.removed, vec![id(2)]);
    assert_eq!(rec.reload, vec![id(4), id(5)]);
    assert_eq!(rec.current, 2);
    assert!(! rec.clean);
    assert_eq!(rec.removed.len() + rec.reload.len() + rec.current, before.len());
    assert_eq!(dir.iter(), before);
    let mut deleted = Vec::new();
    rec.for_each_delete(|e| deleted.push(e));
    assert_eq!(deleted, rec.removed);
}
#[test]
fn reconcile_stale_stamp_in_index_reloads() {
    let (mem, idx) = setup();
    mem.put(1, 1);
    let dir = ResidencyDirectory::new();
    dir.insert(id(1), slot(0), Stamp(1000));
    let mut r = Reconciler::new(&idx);
    let rec = r.reconcile(&dir).unwrap();
    assert_eq!(rec.reload, vec![id(1)]);
}
#[test]
fn reconcile_echo_suppression() {
    let (mem, idx) = setup();
    let dir = ResidencyDirectory::new();
    let mut r = Reconciler::new(&idx);
    let s = idx
        .write(
            id(1),
            PayloadRef {
                vector: &[1],
                meta: &[],
            },
        )
        .unwrap();
    dir.insert(id(1), slot(0), s);
    let s2 = idx
        .write(
            id(1),
            PayloadRef {
                vector: &[2],
                meta: &[],
            },
        )
        .unwrap();
    assert!(dir.set_known_stamp(id(1), s2));
    let rec = r.reconcile(&dir).unwrap();
    assert!(rec.clean);
    assert_eq!(rec.current, 1);
    mem.put(1, 3);
    let rec = r.reconcile(&dir).unwrap();
    assert_eq!(rec.reload, vec![id(1)]);
}
#[test]
fn reconcile_ordering_and_disjointness() {
    let (mem, idx) = setup();
    let dir = ResidencyDirectory::new();
    for n in [40u64, 7, 23, 3, 15, 31, 11, 2, 19, 27] {
        let s = mem.put(n, 1);
        dir.insert(id(n), slot(n as u32), s);
    }
    let mut r = Reconciler::new(&idx);
    for n in [40u64, 3, 19] {
        mem.del(n);
    }
    for n in [31u64, 2, 11] {
        mem.put(n, 2);
    }
    let rec = r.reconcile(&dir).unwrap();
    assert!(rec.removed.windows(2).all(| w | w[0] < w[1]));
    assert!(rec.reload.windows(2).all(| w | w[0] < w[1]));
    assert!(rec.removed.iter().all(| e | ! rec.reload.contains(e)));
    assert_eq!(rec.removed.len() + rec.reload.len() + rec.current, 10);
}
#[test]
fn reconcile_acknowledges_signal() {
    let (mem, idx) = setup();
    mem.put(1, 1);
    let mut r = Reconciler::new(&idx);
    assert!(! r.pending());
    let s = mem.put(2, 1);
    assert!(r.pending());
    let dir = ResidencyDirectory::new();
    let rec = r.reconcile(&dir).unwrap();
    assert_eq!(rec.watermark, s);
    assert!(! r.pending());
    let rec = r.reconcile(&dir).unwrap();
    assert_eq!(rec.watermark, s);
    let s3 = mem.put(3, 1);
    assert!(r.pending());
    assert_eq!(r.reconcile(& dir).unwrap().watermark, s3);
}
#[test]
fn wait_does_not_acknowledge() {
    let (mem, idx) = setup();
    let mut r = Reconciler::new(&idx);
    mem.put(1, 1);
    assert!(r.wait());
    assert!(r.wait());
    assert!(r.pending());
    r.reconcile(&ResidencyDirectory::new()).unwrap();
    assert!(! r.pending());
}
#[test]
fn wait_wakes_on_publish() {
    let (mem, idx) = setup();
    let mut r = Reconciler::new(&idx);
    let writer = mem.clone();
    let h = thread::spawn(move || {
        thread::sleep(Duration::from_millis(50));
        writer.put(1, 1);
    });
    assert!(r.wait());
    h.join().unwrap();
}
#[test]
fn wait_ends_when_index_dropped() {
    let (mem, idx) = setup();
    let mut r = Reconciler::new(&idx);
    let h = thread::spawn(move || r.wait());
    thread::sleep(Duration::from_millis(50));
    drop(idx);
    drop(mem);
    let start = Instant::now();
    assert!(! h.join().unwrap());
    assert!(start.elapsed() < Duration::from_secs(2));
}
#[test]
fn reconcile_none_when_index_dropped() {
    let (mem, idx) = setup();
    let mut r = Reconciler::new(&idx);
    drop(idx);
    drop(mem);
    assert!(r.reconcile(& ResidencyDirectory::new()).is_none());
    assert!(! r.wait());
}
#[test]
fn reconciler_holds_index_weakly() {
    let (_mem, idx) = setup();
    let strong = Arc::strong_count(&idx);
    let _r = Reconciler::new(&idx);
    assert_eq!(Arc::strong_count(& idx), strong);
    assert_eq!(Arc::weak_count(& idx), 1);
}
#[test]
fn load_entry_stable() {
    let ix = Scripted::new(&[Some(5), Some(5)]);
    let got = load_entry(&ix, id(1)).unwrap();
    assert_eq!(got.stamp, Stamp(5));
    assert_eq!(got.retries, 0);
    assert_eq!(got.payload.vector.to_vec(), vec![1]);
    assert_eq!(ix.log(), vec!["stamp", "read", "stamp"]);
}
#[test]
fn load_entry_retries_once() {
    let ix = Scripted::new(&[Some(5), Some(6), Some(6), Some(6)]);
    let got = load_entry(&ix, id(1)).unwrap();
    assert_eq!(got.stamp, Stamp(6));
    assert_eq!(got.retries, 1);
    assert_eq!(got.payload.vector.to_vec(), vec![2]);
}
#[test]
fn load_entry_gives_up_with_first_stamp() {
    let script: Vec<Option<u64>> = (0..18).map(|i| Some(10 + i)).collect();
    let ix = Scripted::new(&script);
    let got = load_entry(&ix, id(1)).unwrap();
    assert_eq!(got.stamp, Stamp(10));
    assert_eq!(got.retries, MAX_LOAD_RETRIES);
    assert_eq!(got.payload.vector.to_vec(), vec![9]);
    let log = ix.log();
    assert_eq!(log.iter().filter(| l | ** l == "read").count(), 9);
    assert_eq!(log.iter().filter(| l | ** l == "stamp").count(), 18);
}
#[test]
fn load_entry_missing() {
    let ix = Scripted::new(&[None]);
    assert_eq!(load_entry(& ix, id(1)), None);
    assert_eq!(ix.log(), vec!["stamp"]);
}
#[test]
fn load_entry_missing_during_retry() {
    let ix = Scripted::new(&[Some(5), Some(6), None]);
    assert_eq!(load_entry(& ix, id(1)), None);
    let ix = Scripted::new(&[Some(5), None, None]);
    assert_eq!(load_entry(& ix, id(1)), None);
}
#[test]
fn load_entry_real_index() {
    let (mem, idx) = setup();
    let s = mem.put(3, 42);
    let got = load_entry(idx.as_ref(), id(3)).unwrap();
    assert_eq!(got.stamp, s);
    assert_eq!(got.payload.vector.to_vec(), vec![42]);
    assert_eq!(load_entry(idx.as_ref(), id(4)), None);
}
#[test]
fn directory_accepts_slot_refs() {
    use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::a::segments as store;
    let from_store: store::SlotRef = store::SlotRef {
        segment: store::SegmentId(3),
        slot: Slot(4),
    };
    let dir = ResidencyDirectory::new();
    dir.insert(id(1), from_store, Stamp(1));
    assert_eq!(dir.get(id(1)).unwrap().slot, from_store);
}
