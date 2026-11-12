use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

use super::super::super::l020::l010::layout::DType;
use super::super::super::l020::a::model_descriptor::{Normalization, Pooling};
use super::*;

static COUNTER: AtomicU64 = AtomicU64::new(0);

fn temp_path() -> PathBuf {
    let n = COUNTER.fetch_add(1, Ordering::SeqCst);
    let dir = std::env::temp_dir().join(format!("xy_index_test_{}_{}", std::process::id(), n));
    std::fs::create_dir_all(&dir).unwrap();
    dir.join("index.bin")
}

fn header() -> IndexHeader {
    IndexHeader {
        model_id: "m".into(),
        dimension: 2,
        storage_dtype: DType::F32,
        pooling: Pooling::Mean,
        normalization: Normalization::L2,
        producing_profile: None,
    }
}

fn ins(parent: ParentRef) -> Op {
    Op::Insert {
        parent,
        payload: vec![1],
        vectors: vec![0; 8],
    }
}

#[test]
fn vector_bytes_by_dtype() {
    let mut h = header();
    assert_eq!(h.vector_bytes(), 8);
    h.storage_dtype = DType::I64;
    assert_eq!(h.vector_bytes(), 16);
    h.storage_dtype = DType::U8;
    assert_eq!(h.vector_bytes(), 2);
}

#[test]
fn compatibility_ignores_profile() {
    let a = header();
    let mut b = header();
    b.producing_profile = Some("x".into());
    assert!(a.is_compatible(&b));
    b.dimension = 3;
    assert!(!a.is_compatible(&b));
}

#[test]
fn open_is_lazy_and_missing_file_is_empty() {
    let idx = Index::open(temp_path(), header());
    assert_eq!(idx.stamp().unwrap(), 0);
    assert_eq!(idx.len().unwrap(), 0);
    idx.flush().unwrap();
}

#[test]
fn insert_tree_and_stamps() {
    let idx = Index::open(temp_path(), header());
    let a = idx
        .apply(vec![ins(ParentRef::Root), ins(ParentRef::New(0)), ins(ParentRef::New(0))])
        .unwrap();
    assert_eq!(a.stamp, 1);
    assert_eq!(a.inserted, vec![1, 2, 3]);
    assert_eq!(idx.children(Some(1)).unwrap(), vec![2, 3]);
    assert_eq!(idx.structure_stamp().unwrap(), 1);
    idx.apply(vec![Op::Update { id: 2, payload: None, vectors: None }]).unwrap();
    assert_eq!(idx.stamp().unwrap(), 2);
    assert_eq!(idx.structure_stamp().unwrap(), 1);
    assert_eq!(idx.get(2).unwrap().unwrap().stamp, 2);
    assert_eq!(idx.get(3).unwrap().unwrap().stamp, 1);
    assert_eq!(idx.apply(vec![]).unwrap().stamp, 2);
}

#[test]
fn failed_batch_rolls_back() {
    let idx = Index::open(temp_path(), header());
    idx.apply(vec![ins(ParentRef::Root), ins(ParentRef::Root)]).unwrap();
    let rx = idx.subscribe();
    let err = idx.apply(vec![
        ins(ParentRef::Entry(1)),
        Op::Move { id: 1, parent: Some(2), position: Some(0) },
        Op::Update { id: 2, payload: Some(vec![9]), vectors: None },
        Op::Remove { id: 1 },
        Op::Remove { id: 99 },
    ]);
    assert!(matches!(err, Err(IndexError::NotFound(99))));
    assert_eq!(idx.stamp().unwrap(), 1);
    assert_eq!(idx.children(None).unwrap(), vec![1, 2]);
    assert_eq!(idx.get(2).unwrap().unwrap().payload, vec![1]);
    assert!(rx.try_recv().is_err());
    let a = idx.apply(vec![ins(ParentRef::Root)]).unwrap();
    assert_eq!(a.inserted, vec![3]);
}

#[test]
fn invalid_inputs() {
    let idx = Index::open(temp_path(), header());
    assert!(matches!(
        idx.apply(vec![Op::Insert { parent: ParentRef::Root, payload: vec![], vectors: vec![0; 5] }]),
        Err(IndexError::InvalidVector { expected_multiple_of: 8, got: 5 })
    ));
    assert!(matches!(idx.apply(vec![ins(ParentRef::New(0))]), Err(IndexError::InvalidParent(_))));
    assert!(matches!(idx.apply(vec![ins(ParentRef::Entry(7))]), Err(IndexError::InvalidParent(_))));
    idx.apply(vec![ins(ParentRef::Root), ins(ParentRef::New(0))]).unwrap();
    assert!(matches!(
        idx.apply(vec![Op::Move { id: 1, parent: Some(2), position: None }]),
        Err(IndexError::InvalidParent(_))
    ));
    assert!(matches!(
        idx.apply(vec![Op::Move { id: 1, parent: Some(1), position: None }]),
        Err(IndexError::InvalidParent(_))
    ));
}

#[test]
fn move_and_remove_events() {
    let idx = Index::open(temp_path(), header());
    idx.apply(vec![
        ins(ParentRef::Root),
        ins(ParentRef::Root),
        ins(ParentRef::New(0)),
        ins(ParentRef::New(2)),
    ])
    .unwrap();
    let rx = idx.subscribe();
    idx.apply(vec![Op::Move { id: 2, parent: Some(1), position: Some(0) }]).unwrap();
    assert_eq!(idx.children(Some(1)).unwrap(), vec![2, 3]);
    idx.apply(vec![Op::Move { id: 2, parent: Some(1), position: Some(50) }]).unwrap();
    assert_eq!(idx.children(Some(1)).unwrap(), vec![3, 2]);
    idx.apply(vec![Op::Remove { id: 1 }]).unwrap();
    assert_eq!(idx.len().unwrap(), 0);
    assert!(idx.get(3).unwrap().is_none());
    let got: Vec<Change> = rx.try_iter().collect();
    let removed: Vec<EntryId> = got
        .iter()
        .filter(|c| c.kind == ChangeKind::Removed)
        .map(|c| c.id)
        .collect();
    assert_eq!(removed, vec![1, 3, 4, 2]);
    assert_eq!(got.len(), 6);
    let cs = idx.changes_since(1).unwrap();
    assert_eq!(cs.to, 4);
    assert_eq!(cs.changes.len(), 6);
    assert!(idx.changes_since(4).unwrap().changes.is_empty());
}

#[test]
fn flush_and_reopen_roundtrip() {
    let path = temp_path();
    let idx = Index::open(path.clone(), header());
    idx.apply(vec![
        Op::Insert { parent: ParentRef::Root, payload: vec![1, 2], vectors: vec![7; 16] },
        ins(ParentRef::New(0)),
        ins(ParentRef::Root),
    ])
    .unwrap();
    idx.apply(vec![Op::Update { id: 3, payload: Some(vec![5]), vectors: None }]).unwrap();
    idx.flush().unwrap();
    let again = Index::open(path, header());
    assert_eq!(again.stamp().unwrap(), 2);
    assert_eq!(again.structure_stamp().unwrap(), 1);
    assert_eq!(again.children(None).unwrap(), vec![1, 3]);
    assert_eq!(again.get(1).unwrap(), idx.get(1).unwrap());
    assert_eq!(again.get(3).unwrap(), idx.get(3).unwrap());
    assert_eq!(again.apply(vec![ins(ParentRef::Root)]).unwrap().inserted, vec![4]);
    assert!(matches!(
        again.changes_since(0),
        Err(IndexError::HistoryTruncated { oldest: 2 })
    ));
}

#[test]
fn mismatch_and_corrupt() {
    let path = temp_path();
    let idx = Index::open(path.clone(), header());
    idx.apply(vec![ins(ParentRef::Root)]).unwrap();
    idx.flush().unwrap();
    let mut other = header();
    other.dimension = 4;
    let m = Index::open(path.clone(), other);
    assert!(matches!(m.len(), Err(IndexError::ModelMismatch { .. })));
    assert!(matches!(m.apply(vec![]), Err(IndexError::ModelMismatch { .. })));
    let mut profiled = header();
    profiled.producing_profile = Some("p".into());
    assert!(Index::open(path.clone(), profiled).len().is_ok());

    let mut bytes = std::fs::read(&path).unwrap();
    let mid = bytes.len() / 2;
    bytes[mid] ^= 0xff;
    std::fs::write(&path, &bytes).unwrap();
    assert!(matches!(Index::open(path.clone(), header()).len(), Err(IndexError::Corrupt(_))));
    std::fs::write(&path, b"nonsense").unwrap();
    assert!(matches!(Index::open(path, header()).len(), Err(IndexError::Corrupt(_))));
}

#[test]
fn dropped_subscriber_is_removed() {
    let idx = Index::open(temp_path(), header());
    drop(idx.subscribe());
    let rx = idx.subscribe();
    idx.apply(vec![ins(ParentRef::Root)]).unwrap();
    assert_eq!(rx.try_recv().unwrap().kind, ChangeKind::Inserted);
}

#[test]
fn read_view_and_send_sync() {
    fn assert_traits<T: Send + Sync + 'static>() {}
    assert_traits::<Index>();
    let idx = Index::open(temp_path(), header());
    idx.apply(vec![ins(ParentRef::Root), ins(ParentRef::New(0))]).unwrap();
    let ids = idx
        .read(|v| {
            assert_eq!(v.parent(2), Some(Some(1)));
            assert_eq!(v.parent(9), None);
            assert_eq!(v.children(Some(1)), &[2]);
            v.iter().collect::<Vec<_>>()
        })
        .unwrap();
    assert_eq!(ids, vec![1, 2]);
}
