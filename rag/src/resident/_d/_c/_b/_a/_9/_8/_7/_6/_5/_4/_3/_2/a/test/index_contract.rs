use super::*;
use std::collections::BTreeMap;
use std::thread;

struct MemIndex {
    state: Mutex<(u64, BTreeMap<u64, (Stamp, Payload)>)>,
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
    fn read(&self, id: EntryId) -> Payload {
        self.state
            .lock()
            .unwrap()
            .1
            .get(&id.0)
            .map(|e| e.1.clone())
            .unwrap_or_default()
    }
    fn write(&self, id: EntryId, p: PayloadRef<'_>) -> Result<Stamp, IndexError> {
        let elem = self.dtype().size_in_bytes();
        if p.vector.len() % elem != 0 {
            return Err(IndexError::InvalidVectorLength { len: p.vector.len(), elem_size: elem });
        }
        let mut g = self.state.lock().unwrap();
        g.0 += 1;
        let stamp = Stamp(g.0);
        let payload = Payload { vector: p.vector.into(), meta: p.meta.into() };
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

#[test]
fn write_read_delete() {
    let idx = MemIndex::new();
    let s1 = idx.write(EntryId(7), pr(&[0; 8], b"m")).unwrap();
    assert!(s1 > Stamp(0));
    assert_eq!(idx.stamp(EntryId(7)), Some(s1));
    assert_eq!(&*idx.read(EntryId(7)).meta, b"m");
    let s2 = idx.delete(EntryId(7)).unwrap().unwrap();
    assert!(s2 > s1);
    assert_eq!(idx.read(EntryId(7)), Payload::default());
    assert_eq!(idx.delete(EntryId(7)).unwrap(), None);
    assert_eq!(idx.subscribe().current(), s2);
}

#[test]
fn invalid_length_leaves_state() {
    let idx = MemIndex::new();
    let err = idx.write(EntryId(1), pr(&[0; 3], b"")).unwrap_err();
    assert_eq!(err, IndexError::InvalidVectorLength { len: 3, elem_size: 4 });
    assert!(idx.stamps().is_empty());
    assert_eq!(idx.subscribe().current(), Stamp(0));
    assert!(idx.write(EntryId(1), pr(&[], b"")).is_ok());
}

#[test]
fn write_many_stops_at_first_error() {
    let idx = MemIndex::new();
    let good = [0u8; 4];
    let bad = [0u8; 5];
    let items = [(EntryId(2), pr(&good, b"")), (EntryId(1), pr(&bad, b"")), (EntryId(3), pr(&good, b""))];
    assert!(idx.write_many(&items).is_err());
    assert_eq!(idx.stamps().len(), 1);
    let ok = idx.write_many(&[(EntryId(5), pr(&good, b"")), (EntryId(4), pr(&good, b""))]).unwrap();
    assert!(ok[0].1 < ok[1].1);
    let ids: Vec<_> = idx.stamps().iter().map(|s| s.0).collect();
    assert_eq!(ids, vec![EntryId(2), EntryId(4), EntryId(5)]);
}

#[test]
fn defaults_are_unsupported() {
    let idx = MemIndex::new();
    assert_eq!(idx.summary(NodeId(1)).unwrap_err(), IndexError::Unsupported(AccessMethod::Centroid));
    assert_eq!(idx.root().unwrap_err(), IndexError::Unsupported(AccessMethod::Subtree));
    assert_eq!(idx.children(NodeId(1)).unwrap_err(), IndexError::Unsupported(AccessMethod::Subtree));
    assert_eq!(idx.cell_entries(CellId(1)).unwrap_err(), IndexError::Unsupported(AccessMethod::Prefix));
    assert_eq!(
        idx.byte_ranges(EntryId(0), EntryId(1)).unwrap_err(),
        IndexError::Unsupported(AccessMethod::Range)
    );
}

#[test]
fn signal_is_monotonic_level() {
    let sig = StampSignal::new(Stamp(0));
    sig.publish(Stamp(5));
    sig.publish(Stamp(3));
    assert_eq!(sig.current(), Stamp(5));
    let mut obs = sig.observer();
    assert_eq!(obs.wait_newer(Some(Duration::from_millis(10))), None);
    sig.publish(Stamp(6));
    sig.publish(Stamp(7));
    assert_eq!(obs.wait_newer(None), Some(Stamp(7)));
    assert_eq!(obs.wait_newer(Some(Duration::from_millis(10))), None);
}

#[test]
fn wait_newer_wakes_across_threads() {
    let idx = Arc::new(MemIndex::new());
    let mut obs = idx.subscribe();
    let writer = {
        let idx = Arc::clone(&idx);
        thread::spawn(move || {
            thread::sleep(Duration::from_millis(20));
            idx.write(EntryId(1), pr(&[0; 4], b"")).unwrap()
        })
    };
    let got = obs.wait_newer(Some(Duration::from_secs(5)));
    assert_eq!(got, Some(writer.join().unwrap()));
}

#[test]
fn concurrent_writes_get_unique_stamps() {
    let idx = Arc::new(MemIndex::new());
    let handles: Vec<_> = (0..4)
        .map(|t| {
            let idx = Arc::clone(&idx);
            thread::spawn(move || {
                (0..50).map(|i| idx.write(EntryId(t * 100 + i), pr(&[], b"")).unwrap()).collect::<Vec<_>>()
            })
        })
        .collect();
    let mut all: Vec<Stamp> = handles.into_iter().flat_map(|h| h.join().unwrap()).collect();
    all.sort();
    all.dedup();
    assert_eq!(all.len(), 200);
    assert_eq!(idx.subscribe().current(), *all.last().unwrap());
}

#[test]
fn error_display_is_descriptive() {
    assert!(IndexError::NotFound.to_string().contains("unknown"));
    assert!(IndexError::Backend("x".into()).to_string().contains('x'));
}
