use super::*;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::a::index_contract::{ByteRange, CellId, NodeId};
use std::thread;
use std::time::Duration;

fn index() -> ReferenceIndex {
    ReferenceIndex::new(DType::F32)
}

fn pr<'a>(v: &'a [u8], m: &'a [u8]) -> PayloadRef<'a> {
    PayloadRef { vector: v, meta: m }
}

#[test]
fn send_sync() {
    fn check<T: Send + Sync>() {}
    check::<ReferenceIndex>();
}

#[test]
fn write_read_replace() {
    let ix = index();
    assert!(ix.is_empty());
    let s1 = ix.write(EntryId(1), pr(&[1; 8], b"a")).unwrap();
    assert!(s1 > Stamp(0));
    let p = ix.read(EntryId(1));
    assert_eq!(&*p.vector, &[1u8; 8][..]);
    assert_eq!(&*p.meta, b"a");
    let s2 = ix.write(EntryId(1), pr(&[], b"b")).unwrap();
    assert!(s2 > s1);
    assert_eq!(ix.len(), 1);
    assert_eq!(ix.stamp(EntryId(1)), Some(s2));
    assert!(ix.read(EntryId(1)).vector.is_empty());
    assert_eq!(ix.read(EntryId(9)), Payload::default());
    assert_eq!(p.vector.len(), 8);
}

#[test]
fn delete_present_and_absent() {
    let ix = index();
    let s = ix.write(EntryId(1), pr(&[], &[])).unwrap();
    let before = ix.subscribe().current();
    assert_eq!(ix.delete(EntryId(7)).unwrap(), None);
    assert_eq!(ix.subscribe().current(), before);
    let d = ix.delete(EntryId(1)).unwrap().unwrap();
    assert!(d > s);
    assert_eq!(ix.subscribe().current(), d);
    assert!(ix.stamp(EntryId(1)).is_none());
    assert!(ix.is_empty());
}

#[test]
fn invalid_vector_length_keeps_signal() {
    let ix = index();
    let err = ix.write(EntryId(1), pr(&[0; 5], &[])).unwrap_err();
    assert_eq!(err, IndexError::InvalidVectorLength { len: 5, elem_size: 4 });
    assert_eq!(ix.subscribe().current(), Stamp(0));
    assert!(ix.is_empty());
}

#[test]
fn parallel_writers_strictly_monotonic() {
    let ix = Arc::new(index());
    let handles: Vec<_> = (0..4u64)
        .map(|t| {
            let ix = ix.clone();
            thread::spawn(move || {
                (0..100u64)
                    .map(|i| ix.write(EntryId(t * 1000 + i % 10), pr(&[0; 4], &[])).unwrap())
                    .collect::<Vec<_>>()
            })
        })
        .collect();
    let mut all = Vec::new();
    for h in handles {
        let v = h.join().unwrap();
        assert!(v.windows(2).all(|w| w[0] < w[1]));
        all.extend(v);
    }
    all.sort();
    all.dedup();
    assert_eq!(all.len(), 400);
    assert_eq!(ix.subscribe().current(), *all.last().unwrap());
}

#[test]
fn stamps_sorted_and_entry_set_matches() {
    let ix = index();
    for id in [5u64, 1, 3, 3] {
        ix.write(EntryId(id), pr(&[], &[])).unwrap();
    }
    let ids: Vec<u64> = ix.stamps().iter().map(|(i, _)| i.0).collect();
    assert_eq!(ids, vec![1, 3, 5]);
    let set: Vec<u64> = ix.entry_set().iter().collect();
    assert_eq!(set, ids);
}

#[test]
fn wait_newer_wakes_after_write() {
    let ix = Arc::new(index());
    let mut obs = ix.subscribe();
    let w = ix.clone();
    let h = thread::spawn(move || {
        thread::sleep(Duration::from_millis(50));
        w.write(EntryId(1), pr(&[], &[])).unwrap()
    });
    let got = obs.wait_newer(Some(Duration::from_secs(5)));
    assert_eq!(got, Some(h.join().unwrap()));
}

#[test]
fn optional_methods_unsupported() {
    let ix = index();
    assert!(ix.access_methods().is_empty());
    assert_eq!(ix.summary(NodeId(0)).unwrap_err(), IndexError::Unsupported(AccessMethod::Centroid));
    assert_eq!(ix.children(NodeId(0)).unwrap_err(), IndexError::Unsupported(AccessMethod::Subtree));
    assert_eq!(ix.root().unwrap_err(), IndexError::Unsupported(AccessMethod::Subtree));
    assert_eq!(ix.cell_entries(CellId(0)).unwrap_err(), IndexError::Unsupported(AccessMethod::Prefix));
    let r: Result<Vec<ByteRange>, _> = ix.byte_ranges(EntryId(0), EntryId(1));
    assert_eq!(r.unwrap_err(), IndexError::Unsupported(AccessMethod::Range));
}
