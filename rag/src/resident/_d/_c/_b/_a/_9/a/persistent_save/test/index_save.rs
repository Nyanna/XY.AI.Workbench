use super::*;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::time::Duration;
use parquet::file::reader::{FileReader, SerializedFileReader};
use super::super::entry_payload::compute_checksum_parts;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::a::index_contract::Index;
use crate::resident::_d::_c::_b::_a::_9::a::entry_store::PayloadChecksums;
use crate::resident::_d::_c::_b::_a::_9::_8::a::index_format::{
    TokenizerFingerprint, TOKENIZER_FP_ALGORITHM,
};
use crate::resident::_d::_c::_b::_a::_9::_8::a::meta_codec;
use crate::resident::_d::_c::_b::_a::_9::_8::a::parquet_decoder::ParquetDecoder;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::a::index_open::{
    LazyIndexFile, OpenOptions as ReadOptions,
};
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::a::model_descriptor::ModelDescriptor;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::_1::layout::DType;
const DIM: usize = 4;
#[test]
fn save_types_are_send_sync() {
    fn check<T: Send + Sync>() {}
    check::<IndexSaver>();
    check::<SaveOutcome>();
    check::<SaveError>();
}
fn assert_static<T: Send + Sync + 'static>() {}
fn header() -> IndexHeader {
    let mut h = IndexHeader::from_descriptor(
            &ModelDescriptor::e5_small_repo_default(DType::F16),
            "rev-1",
            TokenizerFingerprint::new(TOKENIZER_FP_ALGORITHM, 0x2a),
            "prep",
        )
        .unwrap();
    h.dimension = DIM;
    h
}
struct TempDir(PathBuf);
impl TempDir {
    fn new(name: &str) -> TempDir {
        let dir = std::env::temp_dir()
            .join(format!("index_save_{}_{}", std::process::id(), name));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        TempDir(dir)
    }
    fn file(&self, name: &str) -> PathBuf {
        self.0.join(name)
    }
}
impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}
fn vector_bytes(seed: u64) -> Vec<u8> {
    (0..DIM)
        .flat_map(|i| f16::from_f32(seed as f32 + i as f32 * 0.5).to_le_bytes())
        .collect()
}
fn entry(id: u64, stamp: u64) -> SnapshotEntry {
    SnapshotEntry {
        id: EntryId(id),
        stamp: Stamp(stamp),
        data: EntryData::new(
            vector_bytes(id).into(),
            SourceRef::new(format!("f/{id}.txt"), id * 10, id * 10 + 5),
        ),
    }
}
fn snapshot(ids: impl IntoIterator<Item = u64>, token: u64) -> Snapshot {
    let entries: Vec<SnapshotEntry> = ids.into_iter().map(|id| entry(id, id)).collect();
    let max_stamp = entries.iter().map(|e| e.stamp.0).max().unwrap_or(0);
    Snapshot {
        token: SnapshotToken(token),
        max_stamp,
        entries,
    }
}
#[derive(Default)]
struct MockSource {
    current: Mutex<Option<Snapshot>>,
    saved: Mutex<Vec<u64>>,
    taken: AtomicUsize,
    active: AtomicUsize,
    max_active: AtomicUsize,
    delay: Duration,
    panic_once: AtomicBool,
}
impl MockSource {
    fn with(snapshot: Option<Snapshot>) -> Arc<MockSource> {
        Arc::new(MockSource {
            current: Mutex::new(snapshot),
            ..MockSource::default()
        })
    }
    fn saved(&self) -> Vec<u64> {
        self.saved.lock().unwrap().clone()
    }
}
impl SnapshotSource for MockSource {
    fn snapshot(&self) -> Option<Snapshot> {
        let now = self.active.fetch_add(1, Ordering::SeqCst) + 1;
        self.max_active.fetch_max(now, Ordering::SeqCst);
        self.taken.fetch_add(1, Ordering::SeqCst);
        if !self.delay.is_zero() {
            std::thread::sleep(self.delay);
        }
        let result = if self.panic_once.swap(false, Ordering::SeqCst) {
            self.active.fetch_sub(1, Ordering::SeqCst);
            panic!("snapshot failure");
        } else {
            self.current.lock().unwrap().clone()
        };
        self.active.fetch_sub(1, Ordering::SeqCst);
        result
    }
    fn mark_saved(&self, token: SnapshotToken) {
        self.saved.lock().unwrap().push(token.0);
        let mut current = self.current.lock().unwrap();
        if current.as_ref().is_some_and(|s| s.token == token) {
            *current = None;
        }
    }
}
fn saver(dir: &TempDir, source: Arc<MockSource>, row_group_rows: usize) -> IndexSaver {
    IndexSaver::new(
        SaveConfig {
            path: dir.file("index.parquet"),
            header: header(),
            row_group_rows,
        },
        source,
    )
}
fn tmp_of(dir: &TempDir) -> PathBuf {
    dir.file("index.parquet.tmp")
}
fn reader() -> (Arc<ParquetDecoder>, ReadOptions) {
    let decoder = Arc::new(
        ParquetDecoder::new(header(), Arc::new(compute_checksum_parts)),
    );
    let options = ReadOptions {
        verify: true,
        ..ReadOptions::default()
    };
    (decoder, options)
}
fn open(path: &Path) -> LazyIndexFile {
    let (decoder, options) = reader();
    LazyIndexFile::new(path, decoder, options)
}
fn assert_reads_back(path: &Path, expected: &Snapshot) {
    let file = open(path);
    file.ensure_open().unwrap();
    let stamps = file.stamps().unwrap();
    assert_eq!(stamps.len(), expected.entries.len());
    for (entry, (id, stamp)) in expected.entries.iter().zip(stamps) {
        assert_eq!(id, entry.id);
        assert_eq!(stamp.0, entry.stamp.0);
        assert_eq!(& file.read_vector(id) [..], & entry.data.vector[..]);
        let meta = meta_codec::encode_meta_parts(
            &entry.data.source.target_file,
            entry.data.source.char_start,
            entry.data.source.char_end,
        );
        assert_eq!(& file.read_meta(id).unwrap() [..], & meta[..]);
    }
}
fn unavailable_cause(result: Result<SaveOutcome, SaveError>, path: &Path) -> String {
    match result {
        Err(SaveError::Unavailable { path: p, cause }) => {
            assert_eq!(p, path);
            cause
        }
        other => panic!("expected Unavailable, got {other:?}"),
    }
}
#[test]
fn types_are_send_sync_static() {
    assert_static::<IndexSaver>();
    assert_static::<SaveOutcome>();
    assert_static::<SaveError>();
}
#[test]
fn unchanged_writes_nothing() {
    let dir = TempDir::new("unchanged");
    let source = MockSource::with(None);
    let saver = saver(&dir, source.clone(), 8);
    assert_eq!(saver.save(), Ok(SaveOutcome::Unchanged));
    assert!(! saver.path().exists());
    assert!(! tmp_of(& dir).exists());
    assert!(source.saved().is_empty());
}
#[test]
fn writes_snapshot_and_marks_saved_once() {
    let dir = TempDir::new("written");
    let snap = snapshot([2, 5, 9], 7);
    let source = MockSource::with(Some(snap.clone()));
    let saver = saver(&dir, source.clone(), 8);
    assert_eq!(saver.save(), Ok(SaveOutcome::Written { rows : 3, row_groups : 1 }));
    assert_eq!(source.saved(), vec![7]);
    assert!(! tmp_of(& dir).exists());
    assert_reads_back(saver.path(), &snap);
    assert_eq!(saver.save(), Ok(SaveOutcome::Unchanged));
    assert_eq!(source.saved(), vec![7]);
}
#[test]
fn stored_checksum_is_kept_unchanged() {
    let dir = TempDir::new("checksum");
    let mut snap = snapshot([1], 1);
    snap.entries[0].data = EntryData::from_stored(
        snap.entries[0].data.vector.clone(),
        snap.entries[0].data.source.clone(),
        0xdead_beef,
    );
    let source = MockSource::with(Some(snap));
    let saver = saver(&dir, source, 8);
    saver.save().unwrap();
    let file = fs::File::open(saver.path()).unwrap();
    let builder = parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder::try_new(
            file,
        )
        .unwrap();
    let batch = builder.build().unwrap().next().unwrap().unwrap();
    let column = batch
        .column_by_name("checksum")
        .unwrap()
        .as_any()
        .downcast_ref::<UInt64Array>()
        .unwrap();
    assert_eq!(column.value(0), 0xdead_beef);
}
#[test]
fn empty_snapshot_gives_header_only_file() {
    let dir = TempDir::new("empty");
    let snap = Snapshot {
        token: SnapshotToken(3),
        max_stamp: 0,
        entries: Vec::new(),
    };
    let source = MockSource::with(Some(snap.clone()));
    let saver = saver(&dir, source.clone(), 8);
    assert_eq!(saver.save(), Ok(SaveOutcome::Written { rows : 0, row_groups : 0 }));
    assert_eq!(source.saved(), vec![3]);
    let reader = SerializedFileReader::new(fs::File::open(saver.path()).unwrap())
        .unwrap();
    assert_eq!(reader.metadata().num_row_groups(), 0);
    assert_reads_back(saver.path(), &snap);
}
#[test]
fn row_groups_are_full_except_the_last() {
    let dir = TempDir::new("groups");
    let snap = snapshot(1..=10, 1);
    let source = MockSource::with(Some(snap.clone()));
    let saver = saver(&dir, source, 4);
    assert_eq!(saver.save(), Ok(SaveOutcome::Written { rows : 10, row_groups : 3 }));
    let reader = SerializedFileReader::new(fs::File::open(saver.path()).unwrap())
        .unwrap();
    let rows: Vec<i64> = (0..3)
        .map(|i| reader.metadata().row_group(i).num_rows())
        .collect();
    assert_eq!(rows, vec![4, 4, 2]);
    assert_reads_back(saver.path(), &snap);
}
#[test]
fn row_groups_span_internal_batches() {
    let dir = TempDir::new("large");
    let snap = snapshot(1..=5000, 1);
    let source = MockSource::with(Some(snap.clone()));
    let saver = saver(&dir, source, 4500);
    assert_eq!(saver.save(), Ok(SaveOutcome::Written { rows : 5000, row_groups : 2 }));
    let reader = SerializedFileReader::new(fs::File::open(saver.path()).unwrap())
        .unwrap();
    assert_eq!(reader.metadata().row_group(0).num_rows(), 4500);
    assert_eq!(reader.metadata().row_group(1).num_rows(), 500);
    assert_reads_back(saver.path(), &snap);
}
#[test]
fn writer_properties_are_applied() {
    let dir = TempDir::new("props");
    let snap = snapshot(1..=6, 1);
    let source = MockSource::with(Some(snap.clone()));
    let saver = saver(&dir, source, 4);
    saver.save().unwrap();
    let reader = SerializedFileReader::new(fs::File::open(saver.path()).unwrap())
        .unwrap();
    let metadata = reader.metadata();
    let expected_kv = header().with_max_stamp(snap.max_stamp).to_kv();
    assert_eq!(expected_kv.len(), 16);
    let found = metadata.file_metadata().key_value_metadata().unwrap();
    for (key, value) in &expected_kv {
        let pair = found.iter().find(|kv| &kv.key == key).unwrap();
        assert_eq!(pair.value.as_deref(), Some(value.as_str()));
    }
    let mut seen_vector_leaf = false;
    for group in metadata.row_groups() {
        for column in group.columns() {
            assert_eq!(column.compression(), Compression::UNCOMPRESSED);
            assert!(column.column_index_offset().is_some());
            let path = column.column_path();
            if path.parts().first().map(String::as_str) == Some(COL_VECTOR) {
                seen_vector_leaf = true;
                assert!(column.dictionary_page_offset().is_none());
            }
            if path.string() == "target_file" {
                assert!(column.dictionary_page_offset().is_some());
            }
        }
    }
    assert!(seen_vector_leaf);
    assert_eq!(
        vector_leaf_path(& header().column_schema().unwrap()).unwrap().string(),
        "vector.list.item"
    );
}
#[test]
fn invalid_snapshots_leave_target_untouched() {
    let mut wrong_length = snapshot([1, 2], 1);
    wrong_length.entries[1].data = EntryData::new(
        vec![0u8; 3].into(),
        SourceRef::new("f", 0, 1),
    );
    let mut unsorted = snapshot([1, 2], 2);
    unsorted.entries.swap(0, 1);
    let mut duplicate = snapshot([1, 2], 3);
    duplicate.entries[1].id = EntryId(1);
    let mut stamp_zero = snapshot([1, 2], 4);
    stamp_zero.entries[0].stamp = Stamp(0);
    let mut stamp_above_max = snapshot([1, 2], 5);
    stamp_above_max.max_stamp = 1;
    for (i, snap) in [wrong_length, unsorted, duplicate, stamp_zero, stamp_above_max]
        .into_iter()
        .enumerate()
    {
        let dir = TempDir::new(&format!("invalid_{i}"));
        fs::write(dir.file("index.parquet"), b"OLD").unwrap();
        let source = MockSource::with(Some(snap));
        let saver = saver(&dir, source.clone(), 8);
        let cause = unavailable_cause(saver.save(), saver.path());
        assert!(! cause.is_empty());
        assert_eq!(fs::read(saver.path()).unwrap(), b"OLD");
        assert!(! tmp_of(& dir).exists());
        assert!(source.saved().is_empty());
    }
}
#[test]
fn invalid_configuration_is_reported() {
    let dir = TempDir::new("config");
    let source = MockSource::with(Some(snapshot([1], 1)));
    let zero_rows = saver(&dir, source.clone(), 0);
    unavailable_cause(zero_rows.save(), zero_rows.path());
    let mut bad_header = header();
    bad_header.dimension = 0;
    let zero_dim = IndexSaver::new(
        SaveConfig {
            path: dir.file("index.parquet"),
            header: bad_header,
            row_group_rows: 8,
        },
        source.clone(),
    );
    unavailable_cause(zero_dim.save(), zero_dim.path());
    assert!(source.saved().is_empty());
    assert!(! dir.file("index.parquet").exists());
}
#[test]
fn missing_parent_directory_is_unavailable() {
    let dir = TempDir::new("no_parent");
    let source = MockSource::with(Some(snapshot([1], 1)));
    let saver = IndexSaver::new(
        SaveConfig {
            path: dir.file("missing/index.parquet"),
            header: header(),
            row_group_rows: 8,
        },
        source.clone(),
    );
    unavailable_cause(saver.save(), saver.path());
    assert!(source.saved().is_empty());
}
#[test]
fn rename_failure_removes_temp_and_keeps_target() {
    let dir = TempDir::new("rename");
    let target = dir.file("index.parquet");
    fs::create_dir(&target).unwrap();
    fs::write(target.join("keep"), b"x").unwrap();
    let source = MockSource::with(Some(snapshot([1], 1)));
    let saver = saver(&dir, source.clone(), 8);
    let cause = unavailable_cause(saver.save(), saver.path());
    assert!(cause.contains("rename"), "{cause}");
    assert!(! tmp_of(& dir).exists());
    assert!(target.join("keep").exists());
    assert!(source.saved().is_empty());
}
#[test]
fn directory_sync_failure_keeps_new_file_but_reports_error() {
    let dir = TempDir::new("dirsync");
    let snap = snapshot([1, 2], 4);
    let source = MockSource::with(Some(snap.clone()));
    let saver = saver(&dir, source.clone(), 8);
    let result = saver.save_with(&|_| Err(io::Error::other("sync refused")));
    let cause = unavailable_cause(result, saver.path());
    assert!(cause.contains("directory fsync"), "{cause}");
    assert!(source.saved().is_empty());
    assert!(! tmp_of(& dir).exists());
    assert_reads_back(saver.path(), &snap);
    assert_eq!(saver.save(), Ok(SaveOutcome::Written { rows : 2, row_groups : 1 }));
    assert_eq!(source.saved(), vec![4]);
}
#[test]
fn stale_temp_file_is_overwritten() {
    let dir = TempDir::new("stale");
    fs::write(tmp_of(&dir), vec![0xAB; 100_000]).unwrap();
    let snap = snapshot([1, 2, 3], 1);
    let source = MockSource::with(Some(snap.clone()));
    let saver = saver(&dir, source, 8);
    saver.save().unwrap();
    assert!(! tmp_of(& dir).exists());
    assert_reads_back(saver.path(), &snap);
}
#[test]
fn existing_target_is_replaced() {
    let dir = TempDir::new("replace");
    fs::write(dir.file("index.parquet"), b"OLD").unwrap();
    let snap = snapshot([4, 8], 2);
    let source = MockSource::with(Some(snap.clone()));
    let saver = saver(&dir, source, 8);
    saver.save().unwrap();
    assert_reads_back(saver.path(), &snap);
}
#[test]
fn concurrent_saves_are_serialized() {
    let dir = TempDir::new("concurrent");
    let source = Arc::new(MockSource {
        current: Mutex::new(Some(snapshot([1, 2, 3], 9))),
        delay: Duration::from_millis(30),
        ..MockSource::default()
    });
    let saver = Arc::new(saver(&dir, source.clone(), 8));
    let handles: Vec<_> = (0..4)
        .map(|_| {
            let saver = saver.clone();
            std::thread::spawn(move || saver.save().unwrap())
        })
        .collect();
    let outcomes: Vec<SaveOutcome> = handles
        .into_iter()
        .map(|h| h.join().unwrap())
        .collect();
    let written = outcomes
        .iter()
        .filter(|o| matches!(o, SaveOutcome::Written { .. }))
        .count();
    assert_eq!(written, 1);
    assert_eq!(source.max_active.load(Ordering::SeqCst), 1);
    assert_eq!(source.taken.load(Ordering::SeqCst), 4);
    assert_eq!(source.saved(), vec![9]);
}
#[test]
fn poisoned_lock_is_reused() {
    let dir = TempDir::new("poison");
    let source = Arc::new(MockSource {
        current: Mutex::new(Some(snapshot([1], 1))),
        panic_once: AtomicBool::new(true),
        ..MockSource::default()
    });
    let saver = saver(&dir, source.clone(), 8);
    assert!(catch_unwind(AssertUnwindSafe(|| saver.save())).is_err());
    assert_eq!(saver.save(), Ok(SaveOutcome::Written { rows : 1, row_groups : 1 }));
}
fn new_store() -> Arc<EntryStore> {
    let decoder = Arc::new(
        ParquetDecoder::new(header(), Arc::new(compute_checksum_parts)),
    );
    Arc::new(EntryStore::new(header(), None, Arc::new(PayloadChecksums::new(decoder))))
}
fn upsert(store: &EntryStore, id: u64) {
    let meta = meta_codec::encode_meta_parts(
        &format!("f/{id}.txt"),
        id * 10,
        id * 10 + 5,
    );
    store.upsert(EntryId(id), &vector_bytes(id), &meta).unwrap();
}
#[test]
fn store_source_round_trip() {
    let dir = TempDir::new("store");
    let store = new_store();
    let saver = IndexSaver::new(
        SaveConfig {
            path: dir.file("index.parquet"),
            header: header(),
            row_group_rows: 2,
        },
        Arc::new(StoreSource::new(store.clone())),
    );
    assert_eq!(saver.save(), Ok(SaveOutcome::Unchanged));
    for id in [3, 1, 2] {
        upsert(&store, id);
    }
    assert_eq!(saver.save(), Ok(SaveOutcome::Written { rows : 3, row_groups : 2 }));
    assert!(! store.is_dirty());
    assert_eq!(saver.save(), Ok(SaveOutcome::Unchanged));
    let file = open(saver.path());
    file.ensure_open().unwrap();
    let stamps = file.stamps().unwrap();
    let ids: Vec<u64> = stamps.iter().map(|(id, _)| id.0).collect();
    assert_eq!(ids, vec![1, 2, 3]);
    for (id, stamp) in stamps {
        assert_eq!(Some(stamp), Index::stamp(&* store, id));
    }
}
#[test]
fn store_source_token_keeps_later_changes_dirty() {
    let store = new_store();
    let source = StoreSource::new(store.clone());
    assert!(source.snapshot().is_none());
    upsert(&store, 1);
    let first = source.snapshot().unwrap();
    assert_eq!(first.entries.len(), 1);
    upsert(&store, 2);
    source.mark_saved(first.token);
    assert!(store.is_dirty());
    let second = source.snapshot().unwrap();
    assert_eq!(second.entries.len(), 2);
    assert!(second.token.0 > first.token.0);
    assert_eq!(second.max_stamp, 2);
    source.mark_saved(second.token);
    assert!(! store.is_dirty());
    assert!(source.snapshot().is_none());
}
