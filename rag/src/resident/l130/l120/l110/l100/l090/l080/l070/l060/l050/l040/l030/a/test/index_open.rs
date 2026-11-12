use super::*;
use std::sync::atomic::{AtomicUsize, Ordering};

static_assertions::assert_impl_all!(LazyIndexFile: Send, Sync);

struct Mock {
    ids: Vec<(u64, u64)>,
    decodes: AtomicUsize,
    fail_batch_start: Option<u64>,
    verify_err: Option<String>,
    stored_max: Option<u64>,
}

impl Mock {
    fn new(ids: Vec<(u64, u64)>) -> Self {
        Mock {
            ids,
            decodes: AtomicUsize::new(0),
            fail_batch_start: None,
            verify_err: None,
            stored_max: None,
        }
    }
}

impl FormatDecoder for Mock {
    fn read_info(&self, _: &[u8]) -> Result<FileInfo, String> {
        Ok(FileInfo {
            schema_version: 1,
            model_id: "m".into(),
            model_revision: "r".into(),
            row_groups: 1,
            rows: self.ids.len() as u64,
            stored_max_stamp: self.stored_max,
        })
    }
    fn read_ids_stamps(&self, _: &[u8], _: &FileInfo) -> Result<Vec<(u64, u64)>, String> {
        Ok(self.ids.clone())
    }
    fn decode_rows(
        &self,
        _: &[u8],
        _: &FileInfo,
        start: u64,
        end: u64,
    ) -> Result<DecodedBatch, String> {
        self.decodes.fetch_add(1, Ordering::SeqCst);
        if self.fail_batch_start == Some(start) {
            return Err("bad batch".into());
        }
        let rows = start..end;
        Ok(DecodedBatch {
            vectors: rows.clone().map(|r| vec![r as u8; 2].into()).collect(),
            metas: rows.map(|r| vec![r as u8].into()).collect(),
        })
    }
    fn verify(&self, _: &[u8], _: &FileInfo) -> Result<(), String> {
        match &self.verify_err {
            Some(e) => Err(e.clone()),
            None => Ok(()),
        }
    }
}

fn temp_file(name: &str) -> PathBuf {
    let path = std::env::temp_dir().join(format!("index_open_{}_{}", std::process::id(), name));
    std::fs::write(&path, b"dummy-bytes").unwrap();
    path
}

fn open(mock: Mock, name: &str, options: OpenOptions) -> (LazyIndexFile, Arc<Mock>) {
    let mock = Arc::new(mock);
    let file = LazyIndexFile::new(temp_file(name), mock.clone(), options);
    (file, mock)
}

#[test]
fn stamps_sorted_without_decoding() {
    let (f, m) = open(
        Mock::new(vec![(9, 3), (2, 7), (5, 1)]),
        "sorted",
        OpenOptions::default(),
    );
    let s = f.stamps().unwrap();
    assert_eq!(
        s,
        vec![
            (EntryId(2), Stamp(7)),
            (EntryId(5), Stamp(1)),
            (EntryId(9), Stamp(3))
        ]
    );
    assert_eq!(f.stamp(EntryId(9)).unwrap(), Some(Stamp(3)));
    assert_eq!(f.stamp(EntryId(4)).unwrap(), None);
    assert_eq!(m.decodes.load(Ordering::SeqCst), 0);
    assert_eq!(f.access_kind().unwrap(), AccessKind::Mmap);
    assert_eq!(f.stamp_counter().unwrap().max_raw(), Some(7));
}

#[test]
fn duplicate_ids_rejected_and_remembered() {
    let (f, _) = open(
        Mock::new(vec![(1, 1), (1, 2)]),
        "dup",
        OpenOptions::default(),
    );
    let e = f.ensure_open().unwrap_err();
    assert!(matches!(e, OpenError::Unavailable { .. }));
    assert_eq!(f.stamps().unwrap_err(), e);
    assert!(f.read(EntryId(1)).vector.is_empty());
}

#[test]
fn missing_file_is_unavailable() {
    let f = LazyIndexFile::new(
        "/nonexistent/dir/file",
        Arc::new(Mock::new(vec![])),
        OpenOptions::default(),
    );
    assert!(f.ensure_open().is_err());
}

#[test]
fn verify_failure_and_skip() {
    let mut m = Mock::new(vec![(1, 1)]);
    m.verify_err = Some("checksum".into());
    let (f, _) = open(m, "verify", OpenOptions::default());
    assert!(f.ensure_open().is_err());

    let mut m = Mock::new(vec![(1, 1)]);
    m.verify_err = Some("checksum".into());
    let (f, _) = open(
        m,
        "noverify",
        OpenOptions {
            use_mmap: false,
            verify: false,
        },
    );
    assert!(f.ensure_open().is_ok());
    assert_eq!(f.access_kind().unwrap(), AccessKind::FileRead);
}

#[test]
fn batches_decode_once() {
    let ids = (0..(BATCH_ROWS as u64 + 10)).map(|i| (i, i + 1)).collect();
    let (f, m) = open(Mock::new(ids), "batches", OpenOptions::default());
    assert_eq!(&*f.read_vector(EntryId(3)), &[3, 3]);
    assert_eq!(&*f.read_meta(EntryId(4)).unwrap(), &[4]);
    assert_eq!(m.decodes.load(Ordering::SeqCst), 1);
    let p = f.read(EntryId(BATCH_ROWS as u64 + 1));
    assert_eq!(p.meta.len(), 1);
    assert_eq!(m.decodes.load(Ordering::SeqCst), 2);
}

#[test]
fn unknown_id_does_not_decode() {
    let (f, m) = open(Mock::new(vec![(1, 1)]), "unknown", OpenOptions::default());
    assert!(f.read_vector(EntryId(7)).is_empty());
    assert!(f.read_meta(EntryId(7)).is_none());
    assert_eq!(f.read(EntryId(7)), Payload::default());
    assert_eq!(m.decodes.load(Ordering::SeqCst), 0);
}

#[test]
fn failed_batch_is_remembered_others_usable() {
    let ids = (0..(BATCH_ROWS as u64 * 2)).map(|i| (i, i + 1)).collect();
    let mut m = Mock::new(ids);
    m.fail_batch_start = Some(0);
    let (f, m) = open(m, "failbatch", OpenOptions::default());
    assert!(f.read_meta(EntryId(1)).is_none());
    assert!(f.read_vector(EntryId(1)).is_empty());
    assert_eq!(f.read(EntryId(1)), Payload::default());
    assert_eq!(m.decodes.load(Ordering::SeqCst), 1);
    assert!(f.read_meta(EntryId(BATCH_ROWS as u64)).is_some());
}

#[test]
fn concurrent_first_access_decodes_once() {
    let ids = (0..100u64).map(|i| (i, i + 1)).collect();
    let (f, m) = open(Mock::new(ids), "concurrent", OpenOptions::default());
    let f = Arc::new(f);
    let handles: Vec<_> = (0..8)
        .map(|_| {
            let f = f.clone();
            std::thread::spawn(move || f.read(EntryId(5)))
        })
        .collect();
    for h in handles {
        assert_eq!(h.join().unwrap().meta.len(), 1);
    }
    assert_eq!(m.decodes.load(Ordering::SeqCst), 1);
}

#[test]
fn counter_resumes_from_stored_max() {
    let mut m = Mock::new(vec![(1, 5)]);
    m.stored_max = Some(20);
    let (f, _) = open(m, "resume", OpenOptions::default());
    assert_eq!(f.stamp_counter().unwrap().next().unwrap().as_raw(), 21);
}
