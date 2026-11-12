use super::*;
use std::sync::atomic::AtomicUsize;

static DIR_SEQ: AtomicUsize = AtomicUsize::new(0);

fn temp_dir() -> PathBuf {
    let n = DIR_SEQ.fetch_add(1, Ordering::SeqCst);
    let dir = std::env::temp_dir().join(format!("file_index_test_{}_{n}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}

fn header() -> IndexHeader {
    IndexHeader {
        model_id: "m".to_string(),
        dimension: 2,
        dtype: DType::F32,
    }
}

fn opts(access: AccessMode) -> OpenOptions {
    OpenOptions {
        access,
        ..OpenOptions::default()
    }
}

fn vec8(x: u8) -> Vec<u8> {
    vec![x; 8]
}

fn put(idx: &FileIndex, id: u64, x: u8) -> Stamp {
    idx.write(
        EntryId(id),
        PayloadRef {
            vector: &vec8(x),
            meta: &[x, x],
        },
    )
    .unwrap()
}

#[test]
fn missing_file_gives_empty_index_and_no_file() {
    let dir = temp_dir();
    let path = dir.join("a.parquet");
    let idx = FileIndex::open(&path, header(), OpenOptions::default()).unwrap();
    assert!(idx.stamps().is_empty());
    assert_eq!(idx.subscribe().current(), Stamp(0));
    assert!(!path.exists());
    assert!(idx.access_methods().is_empty());
    assert_eq!(idx.dtype(), DType::F32);
}

#[test]
fn save_and_reopen_roundtrip_both_modes() {
    for mode in [AccessMode::Mmap, AccessMode::File] {
        let dir = temp_dir();
        let path = dir.join("a.parquet");
        let idx = FileIndex::open(
            &path,
            header(),
            OpenOptions {
                rows_per_group: 2,
                ..opts(mode)
            },
        )
        .unwrap();
        for i in 1..=5u64 {
            put(&idx, i, i as u8);
        }
        let del = idx.delete(EntryId(3)).unwrap().unwrap();
        assert_eq!(idx.delete(EntryId(99)).unwrap(), None);
        idx.save().unwrap();
        assert_eq!(idx.read(EntryId(4)).vector.to_vec(), vec8(4));

        let again = FileIndex::open(&path, header(), opts(mode)).unwrap();
        assert_eq!(again.stamps(), idx.stamps());
        assert_eq!(again.read(EntryId(5)).meta.to_vec(), vec![5, 5]);
        assert_eq!(again.read(EntryId(3)), Payload::default());
        assert_eq!(again.subscribe().current(), del);
        let next = put(&again, 1, 9);
        assert!(next > del);
        assert_eq!(again.read(EntryId(1)).vector.to_vec(), vec8(9));
    }
}

#[test]
fn row_groups_follow_option() {
    let dir = temp_dir();
    let path = dir.join("a.parquet");
    let idx = FileIndex::open(
        &path,
        header(),
        OpenOptions {
            rows_per_group: 0,
            ..OpenOptions::default()
        },
    )
    .unwrap();
    for i in 1..=3u64 {
        put(&idx, i, 1);
    }
    idx.save().unwrap();
    let data = DataFile::open_file(File::open(&path).unwrap(), AccessMode::File).unwrap();
    assert_eq!(data.meta.metadata().num_row_groups(), 3);
}

#[test]
fn empty_save_writes_valid_file() {
    let dir = temp_dir();
    let path = dir.join("a.parquet");
    let idx = FileIndex::open(&path, header(), OpenOptions::default()).unwrap();
    idx.save().unwrap();
    let again = FileIndex::open(&path, header(), OpenOptions::default()).unwrap();
    assert!(again.stamps().is_empty());
}

#[test]
fn header_mismatch_is_reported() {
    let dir = temp_dir();
    let path = dir.join("a.parquet");
    let idx = FileIndex::open(&path, header(), OpenOptions::default()).unwrap();
    idx.save().unwrap();
    let mut h = header();
    h.dimension = 3;
    let err = FileIndex::open(&path, h, OpenOptions::default()).err().unwrap();
    assert!(matches!(
        err,
        FileIndexError::HeaderMismatch {
            field: "dimension",
            ..
        }
    ));
    let mut h = header();
    h.model_id = "x".to_string();
    assert!(matches!(
        FileIndex::open(&path, h, OpenOptions::default()).err().unwrap(),
        FileIndexError::HeaderMismatch {
            field: "model_id",
            ..
        }
    ));
    let mut h = header();
    h.dtype = DType::F16;
    assert!(matches!(
        FileIndex::open(&path, h, OpenOptions::default()).err().unwrap(),
        FileIndexError::HeaderMismatch { field: "dtype", .. }
    ));
}

#[test]
fn garbage_file_is_corrupt() {
    let dir = temp_dir();
    let path = dir.join("a.parquet");
    fs::write(&path, b"not parquet at all").unwrap();
    for mode in [AccessMode::Mmap, AccessMode::File] {
        let err = FileIndex::open(&path, header(), opts(mode)).err().unwrap();
        assert!(matches!(err, FileIndexError::Corrupt(_)));
    }
}

#[test]
fn invalid_vector_length_changes_nothing() {
    let dir = temp_dir();
    let idx = FileIndex::open(dir.join("a.parquet"), header(), OpenOptions::default()).unwrap();
    let err = idx
        .write(
            EntryId(1),
            PayloadRef {
                vector: &[1, 2, 3],
                meta: &[],
            },
        )
        .unwrap_err();
    assert_eq!(
        err,
        IndexError::InvalidVectorLength {
            len: 3,
            elem_size: 4
        }
    );
    assert!(idx.stamps().is_empty());
    assert_eq!(idx.subscribe().current(), Stamp(0));
    assert_eq!(put(&idx, 1, 1), Stamp(1));
}

#[test]
fn save_without_parent_directory_fails_and_keeps_state() {
    let dir = temp_dir();
    let path = dir.join("missing").join("a.parquet");
    let idx = FileIndex::open(&path, header(), OpenOptions::default()).unwrap();
    put(&idx, 1, 1);
    assert!(matches!(idx.save(), Err(IndexError::Backend(_))));
    assert_eq!(idx.read(EntryId(1)).vector.to_vec(), vec8(1));
}

#[test]
fn replaced_entries_survive_save_and_old_payload_stays_valid() {
    let dir = temp_dir();
    let path = dir.join("a.parquet");
    let idx = FileIndex::open(&path, header(), OpenOptions::default()).unwrap();
    put(&idx, 1, 1);
    idx.save().unwrap();
    let old = idx.read(EntryId(1));
    put(&idx, 1, 2);
    idx.save().unwrap();
    assert_eq!(old.vector.to_vec(), vec8(1));
    assert_eq!(idx.read(EntryId(1)).vector.to_vec(), vec8(2));
    let leftovers = fs::read_dir(&dir).unwrap().count();
    assert_eq!(leftovers, 1);
}

#[test]
fn concurrent_writes_and_saves_lose_nothing() {
    let dir = temp_dir();
    let path = dir.join("a.parquet");
    let idx = Arc::new(FileIndex::open(&path, header(), OpenOptions::default()).unwrap());
    let mut handles = Vec::new();
    for t in 0..4u64 {
        let idx = idx.clone();
        handles.push(std::thread::spawn(move || {
            for i in 0..50u64 {
                put(&idx, t * 100 + i, (i % 200) as u8);
                if i % 10 == 0 {
                    idx.save().unwrap();
                }
            }
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
    idx.save().unwrap();
    let again = FileIndex::open(&path, header(), OpenOptions::default()).unwrap();
    assert_eq!(again.stamps().len(), 200);
    assert_eq!(again.stamps(), idx.stamps());
}

#[test]
fn verify_off_skips_data_checks() {
    let dir = temp_dir();
    let path = dir.join("a.parquet");
    let idx = FileIndex::open(&path, header(), OpenOptions::default()).unwrap();
    put(&idx, 1, 1);
    idx.save().unwrap();
    let mut h = header();
    h.dtype = DType::F32;
    let fast = FileIndex::open(
        &path,
        h,
        OpenOptions {
            verify: false,
            ..OpenOptions::default()
        },
    )
    .unwrap();
    assert_eq!(fast.stamps().len(), 1);
}
