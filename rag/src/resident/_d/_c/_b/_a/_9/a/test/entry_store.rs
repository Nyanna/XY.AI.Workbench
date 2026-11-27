use super::*;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::path::PathBuf;
use std::sync::Mutex;
use std::time::Duration;
use arrow::array::{ArrayRef, FixedSizeListArray, Float16Array, StringArray, UInt64Array};
use arrow::datatypes::{DataType, Field};
use arrow::record_batch::RecordBatch;
use half::f16;
use parquet::arrow::ArrowWriter;
use parquet::file::metadata::KeyValue;
use parquet::file::properties::WriterProperties;
use crate::resident::_d::_c::_b::_a::_9::_8::a::index_format::{
    column_schema, TokenizerFingerprint, TOKENIZER_FP_ALGORITHM,
};
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::a::index_open::{
    DecodedBatch, FileInfo, FormatDecoder, OpenOptions,
};
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::a::model_descriptor::ModelDescriptor;
#[test]
fn entry_store_is_send_sync() {
    fn check<T: Send + Sync>() {}
    check::<EntryStore>();
}
const DIM: usize = 4;
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
fn vec_of(b: u8) -> Vec<u8> {
    vec![b; DIM * 2]
}
fn file_meta(id: u64) -> Vec<u8> {
    vec![id as u8; 3]
}
fn mix(id: EntryId, vector: &[u8], meta: &[u8]) -> u64 {
    let mut h = id.0 ^ 0xabc;
    for b in vector.iter().chain(meta) {
        h = h.wrapping_mul(31).wrapping_add(u64::from(*b));
    }
    h
}
struct MockChecksums {
    stored: Mutex<Result<Vec<(EntryId, u64)>, String>>,
    fail_for: Mutex<Option<EntryId>>,
}
impl MockChecksums {
    fn new(stored: Vec<(EntryId, u64)>) -> Self {
        MockChecksums {
            stored: Mutex::new(Ok(stored)),
            fail_for: Mutex::new(None),
        }
    }
    fn set(&self, id: u64, value: u64) {
        let mut guard = self.stored.lock().unwrap();
        for pair in guard.as_mut().unwrap().iter_mut() {
            if pair.0 == EntryId(id) {
                pair.1 = value;
            }
        }
    }
    fn remove_stored(&self, id: u64) {
        self.stored.lock().unwrap().as_mut().unwrap().retain(|p| p.0 != EntryId(id));
    }
    fn reverse(&self) {
        self.stored.lock().unwrap().as_mut().unwrap().reverse();
    }
    fn fail_stored(&self, text: &str) {
        *self.stored.lock().unwrap() = Err(text.to_string());
    }
}
impl ChecksumProvider for MockChecksums {
    fn checksum(&self, id: EntryId, vector: &[u8], meta: &[u8]) -> Result<u64, String> {
        if meta == b"bad" || *self.fail_for.lock().unwrap() == Some(id) {
            return Err("rejected".to_string());
        }
        Ok(mix(id, vector, meta))
    }
    fn stored_checksums(&self, _: &Path) -> Result<Vec<(EntryId, u64)>, String> {
        self.stored.lock().unwrap().clone()
    }
}
struct MockDecoder {
    ids: Vec<(u64, u64)>,
    stored_max: Option<u64>,
    short_vector_id: Option<u64>,
    fail_rows: bool,
}
impl MockDecoder {
    fn new(ids: &[(u64, u64)]) -> Self {
        MockDecoder {
            ids: ids.to_vec(),
            stored_max: None,
            short_vector_id: None,
            fail_rows: false,
        }
    }
}
impl FormatDecoder for MockDecoder {
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
    fn read_ids_stamps(
        &self,
        _: &[u8],
        _: &FileInfo,
    ) -> Result<Vec<(u64, u64)>, String> {
        Ok(self.ids.clone())
    }
    fn decode_rows(
        &self,
        _: &[u8],
        _: &FileInfo,
        start: u64,
        end: u64,
    ) -> Result<DecodedBatch, String> {
        if self.fail_rows {
            return Err("boom".into());
        }
        let ids: Vec<u64> = (start..end).map(|r| self.ids[r as usize].0).collect();
        Ok(DecodedBatch {
            vectors: ids
                .iter()
                .map(|&id| {
                    if self.short_vector_id == Some(id) {
                        vec![id as u8; 2].into()
                    } else {
                        vec_of(id as u8).into()
                    }
                })
                .collect(),
            metas: ids.iter().map(|&id| file_meta(id).into()).collect(),
        })
    }
    fn verify(&self, _: &[u8], _: &FileInfo) -> Result<(), String> {
        Ok(())
    }
}
fn temp_file(name: &str, bytes: &[u8]) -> PathBuf {
    let path = std::env::temp_dir()
        .join(format!("entry_store_{}_{}", std::process::id(), name));
    std::fs::write(&path, bytes).unwrap();
    path
}
fn bare() -> (EntryStore, Arc<MockChecksums>) {
    let checks = Arc::new(MockChecksums::new(Vec::new()));
    (EntryStore::new(header(), None, checks.clone()), checks)
}
fn with_file(rows: &[(u64, u64)], name: &str) -> (EntryStore, Arc<MockChecksums>) {
    with_decoder(MockDecoder::new(rows), name)
}
fn with_decoder(decoder: MockDecoder, name: &str) -> (EntryStore, Arc<MockChecksums>) {
    let stored = decoder
        .ids
        .iter()
        .map(|&(id, _)| {
            (EntryId(id), mix(EntryId(id), &vec_of(id as u8), &file_meta(id)))
        })
        .collect();
    let checks = Arc::new(MockChecksums::new(stored));
    let file = LazyIndexFile::new(
        temp_file(name, b"dummy"),
        Arc::new(decoder),
        OpenOptions::default(),
    );
    (EntryStore::new(header(), Some(Arc::new(file)), checks.clone()), checks)
}
fn unavailable() -> EntryStore {
    let file = LazyIndexFile::new(
        "/nonexistent/entry_store/missing",
        Arc::new(MockDecoder::new(&[])),
        OpenOptions::default(),
    );
    EntryStore::new(
        header(),
        Some(Arc::new(file)),
        Arc::new(MockChecksums::new(Vec::new())),
    )
}
fn bytes_of(shared: &SharedBytes) -> Vec<u8> {
    shared.to_vec()
}
#[test]
fn upsert_without_file_issues_increasing_stamps() {
    let (s, _) = bare();
    assert!(! s.is_dirty());
    assert_eq!(s.max_stamp(), None);
    assert_eq!(s.path(), None);
    let a = s.upsert(EntryId(1), &vec_of(1), b"m").unwrap();
    let b = s.upsert(EntryId(2), &vec_of(2), b"").unwrap();
    assert_eq!((a, b), (Stamp(1), Stamp(2)));
    assert!(s.is_dirty());
    assert_eq!(s.change_seq(), 2);
    assert_eq!(s.max_stamp(), Some(Stamp(2)));
    assert_eq!(s.header().max_stamp, 2);
    let p = s.read(EntryId(1));
    assert_eq!(bytes_of(& p.vector), vec_of(1));
    assert_eq!(bytes_of(& p.meta), b"m".to_vec());
    assert_eq!(s.meta(EntryId(2)).map(| m | m.len()), Some(0));
    assert_eq!(s.stamp(EntryId(1)), Some(Stamp(1)));
    assert_eq!(s.stamps(), vec![(EntryId(1), Stamp(1)), (EntryId(2), Stamp(2))]);
}
#[test]
fn index_contract_basics() {
    let (s, _) = bare();
    assert_eq!(s.dtype(), DType::F16);
    assert!(s.access_methods().is_empty());
    assert!(matches!(s.root(), Err(IndexError::Unsupported(AccessMethod::Subtree))));
}
#[test]
fn schema_errors_change_nothing() {
    let (s, _) = bare();
    let mut observer = s.subscribe();
    for len in [0, 2, DIM * 2 - 1, DIM * 2 + 1] {
        assert_eq!(
            s.upsert(EntryId(1), & vec![0u8; len], b""), Err(EntryError::Schema {
            expected : DIM * 2, found : len })
        );
    }
    assert_eq!(s.change_seq(), 0);
    assert!(! s.is_dirty());
    assert_eq!(s.max_stamp(), None);
    assert!(s.stamps().is_empty());
    assert_eq!(observer.wait_newer(Some(Duration::from_millis(20))), None);
}
#[test]
fn checksum_failure_changes_nothing() {
    let (s, _) = bare();
    let mut observer = s.subscribe();
    assert_eq!(
        s.upsert(EntryId(1), & vec_of(1), b"bad"), Err(EntryError::Meta("rejected"
        .into()))
    );
    assert!(! s.is_dirty());
    assert_eq!(s.max_stamp(), None);
    assert_eq!(observer.wait_newer(Some(Duration::from_millis(20))), None);
    assert_eq!(s.upsert(EntryId(1), & vec_of(1), b"ok"), Ok(Stamp(1)));
}
#[test]
fn replace_and_reinsert_get_new_stamps() {
    let (s, _) = bare();
    assert_eq!(s.upsert(EntryId(1), & vec_of(1), b"a"), Ok(Stamp(1)));
    assert_eq!(s.upsert(EntryId(1), & vec_of(2), b"b"), Ok(Stamp(2)));
    assert_eq!(bytes_of(& s.read(EntryId(1)).vector), vec_of(2));
    assert_eq!(s.remove(EntryId(1)), Ok(Some(Stamp(3))));
    assert_eq!(s.upsert(EntryId(1), & vec_of(3), b"c"), Ok(Stamp(4)));
    assert_eq!(s.stamp(EntryId(1)), Some(Stamp(4)));
    assert_eq!(s.change_seq(), 4);
}
#[test]
fn remove_of_missing_entry_changes_nothing() {
    let (s, _) = bare();
    let mut observer = s.subscribe();
    assert_eq!(s.remove(EntryId(7)), Ok(None));
    assert!(! s.is_dirty());
    assert_eq!(s.change_seq(), 0);
    assert_eq!(observer.wait_newer(Some(Duration::from_millis(20))), None);
    s.upsert(EntryId(7), &vec_of(7), b"").unwrap();
    assert_eq!(s.remove(EntryId(7)), Ok(Some(Stamp(2))));
    assert_eq!(s.remove(EntryId(7)), Ok(None));
    assert_eq!(s.read(EntryId(7)), Payload2::default());
    assert_eq!(s.meta(EntryId(7)), None);
    assert_eq!(s.stamp(EntryId(7)), None);
    assert!(s.stamps().is_empty());
    assert_eq!(s.change_seq(), 2);
}
#[test]
fn file_entries_are_overlaid_and_removed() {
    let (s, _) = with_file(&[(1, 5), (3, 7), (5, 9)], "overlay");
    assert!(s.path().is_some());
    assert_eq!(
        s.stamps(), vec![(EntryId(1), Stamp(5)), (EntryId(3), Stamp(7)), (EntryId(5),
        Stamp(9))]
    );
    assert_eq!(bytes_of(& s.read(EntryId(3)).vector), vec_of(3));
    assert_eq!(s.meta(EntryId(3)).map(| m | bytes_of(& m)), Some(file_meta(3)));
    assert_eq!(s.max_stamp(), Some(Stamp(9)));
    assert_eq!(s.upsert(EntryId(2), & vec_of(2), b"n"), Ok(Stamp(10)));
    assert_eq!(s.upsert(EntryId(3), & vec_of(9), b"r"), Ok(Stamp(11)));
    assert_eq!(s.remove(EntryId(5)), Ok(Some(Stamp(12))));
    assert_eq!(s.remove(EntryId(5)), Ok(None));
    assert_eq!(s.remove(EntryId(99)), Ok(None));
    assert_eq!(
        s.stamps(), vec![(EntryId(1), Stamp(5)), (EntryId(2), Stamp(10)), (EntryId(3),
        Stamp(11))]
    );
    assert_eq!(s.stamp(EntryId(5)), None);
    assert_eq!(s.stamp(EntryId(3)), Some(Stamp(11)));
    assert_eq!(s.read(EntryId(5)), Payload2::default());
    assert_eq!(s.meta(EntryId(5)), None);
    assert_eq!(bytes_of(& s.read(EntryId(3)).vector), vec_of(9));
    assert_eq!(s.meta(EntryId(3)).map(| m | bytes_of(& m)), Some(b"r".to_vec()));
    assert_eq!(s.upsert(EntryId(5), & vec_of(5), b""), Ok(Stamp(13)));
    assert_eq!(s.header().max_stamp, 13);
    assert_eq!(s.change_seq(), 4);
}
#[test]
fn counter_resumes_above_stored_max() {
    let mut d = MockDecoder::new(&[(1, 5)]);
    d.stored_max = Some(20);
    let (s, _) = with_decoder(d, "resume");
    assert_eq!(s.max_stamp(), Some(Stamp(20)));
    assert_eq!(s.subscribe().current(), Stamp(20));
    assert_eq!(s.upsert(EntryId(2), & vec_of(2), b""), Ok(Stamp(21)));
}
#[test]
fn write_keeps_existing_meta() {
    let (s, _) = with_file(&[(1, 5)], "write_meta");
    let stamp = s
        .write(
            EntryId(1),
            PayloadRef {
                vector: &vec_of(9),
                meta: b"ignored",
            },
        )
        .unwrap();
    assert_eq!(stamp, Stamp(6));
    let p = s.read(EntryId(1));
    assert_eq!(bytes_of(& p.vector), vec_of(9));
    assert_eq!(bytes_of(& p.meta), file_meta(1));
    s.upsert(EntryId(1), &vec_of(8), b"own").unwrap();
    s.write(
            EntryId(1),
            PayloadRef {
                vector: &vec_of(7),
                meta: b"ignored",
            },
        )
        .unwrap();
    assert_eq!(bytes_of(& s.read(EntryId(1)).meta), b"own".to_vec());
    s.write(
            EntryId(2),
            PayloadRef {
                vector: &vec_of(2),
                meta: b"ignored",
            },
        )
        .unwrap();
    assert!(s.read(EntryId(2)).meta.is_empty());
    assert!(s.delete(EntryId(1)).unwrap().is_some());
    s.write(
            EntryId(1),
            PayloadRef {
                vector: &vec_of(1),
                meta: b"ignored",
            },
        )
        .unwrap();
    assert!(s.read(EntryId(1)).meta.is_empty());
}
#[test]
fn write_with_wrong_length_panics_without_effect() {
    let (s, _) = bare();
    for len in [0, 3, DIM * 2 + 2] {
        let bad = vec![0u8; len];
        let result = catch_unwind(
            AssertUnwindSafe(|| {
                s.write(
                    EntryId(1),
                    PayloadRef {
                        vector: &bad,
                        meta: b"",
                    },
                )
            }),
        );
        assert!(result.is_err());
    }
    assert_eq!(s.change_seq(), 0);
    assert!(s.stamps().is_empty());
    assert_eq!(s.max_stamp(), None);
}
#[test]
fn write_many_and_delete_follow_the_contract() {
    let (s, _) = bare();
    let a = vec_of(1);
    let b = vec_of(2);
    let stamps = s
        .write_many(
            &[
                (
                    EntryId(1),
                    PayloadRef {
                        vector: &a,
                        meta: b"",
                    },
                ),
                (
                    EntryId(2),
                    PayloadRef {
                        vector: &b,
                        meta: b"",
                    },
                ),
            ],
        )
        .unwrap();
    assert_eq!(stamps, vec![(EntryId(1), Stamp(1)), (EntryId(2), Stamp(2))]);
    assert_eq!(s.delete(EntryId(2)), Ok(Some(Stamp(3))));
    assert_eq!(s.delete(EntryId(2)), Ok(None));
}
#[test]
fn unavailable_file_degrades_gracefully() {
    let s = unavailable();
    assert!(
        matches!(s.upsert(EntryId(1), & vec_of(1), b""), Err(EntryError::Unavailable(_)))
    );
    assert!(matches!(s.remove(EntryId(1)), Err(EntryError::Unavailable(_))));
    assert!(
        matches!(s.write(EntryId(1), PayloadRef { vector : & vec_of(1), meta : b"" }),
        Err(IndexError::Backend(_)))
    );
    assert!(matches!(s.delete(EntryId(1)), Err(IndexError::Backend(_))));
    assert_eq!(s.max_stamp(), None);
    assert_eq!(s.subscribe().current(), Stamp(0));
    assert_eq!(s.header().max_stamp, 0);
    assert!(s.stamps().is_empty());
    assert_eq!(s.stamp(EntryId(1)), None);
    assert_eq!(s.read(EntryId(1)), Payload2::default());
    assert_eq!(s.meta(EntryId(1)), None);
    assert!(! s.is_dirty());
    assert!(matches!(s.verify(), Err(VerifyError::Unavailable(_))));
    assert_eq!(s.export(), Ok(None));
}
#[test]
fn schema_error_precedes_unavailable() {
    let s = unavailable();
    assert!(matches!(s.upsert(EntryId(1), & [], b""), Err(EntryError::Schema { .. })));
}
#[test]
fn subscribe_starts_at_max_and_publishes() {
    let (s, _) = with_file(&[(1, 5)], "subscribe");
    let mut observer = s.subscribe();
    assert_eq!(observer.current(), Stamp(5));
    let stamp = s.upsert(EntryId(2), &vec_of(2), b"").unwrap();
    assert_eq!(observer.wait_newer(Some(Duration::from_millis(500))), Some(stamp));
    s.remove(EntryId(2)).unwrap();
    assert_eq!(observer.wait_newer(Some(Duration::from_millis(500))), Some(Stamp(7)));
    assert_eq!(s.subscribe().current(), Stamp(7));
}
#[test]
fn verify_without_file_is_ok() {
    let (s, _) = bare();
    assert_eq!(s.verify(), Ok(()));
}
#[test]
fn verify_accepts_consistent_file_and_ignores_overlay() {
    let (s, _) = with_file(&[(1, 1), (2, 2), (3, 3)], "verify_ok");
    assert_eq!(s.verify(), Ok(()));
    s.upsert(EntryId(2), &vec_of(0), b"changed").unwrap();
    let seq = s.change_seq();
    assert_eq!(s.verify(), Ok(()));
    assert_eq!(s.change_seq(), seq);
    assert_eq!(s.max_stamp(), Some(Stamp(4)));
}
#[test]
fn verify_reports_first_failing_entry_ascending() {
    let (s, checks) = with_file(&[(1, 1), (2, 2), (3, 3)], "verify_mismatch");
    checks.set(3, 1);
    checks.set(2, 2);
    checks.reverse();
    match s.verify() {
        Err(VerifyError::Entry { entry, cause }) => {
            assert_eq!(entry, EntryId(2));
            assert!(cause.contains("checksum mismatch"), "{cause}");
        }
        other => panic!("unexpected {other:?}"),
    }
}
#[test]
fn verify_reports_decode_and_provider_failures() {
    let mut d = MockDecoder::new(&[(1, 1), (2, 2)]);
    d.short_vector_id = Some(2);
    let (s, _) = with_decoder(d, "verify_short");
    match s.verify() {
        Err(VerifyError::Entry { entry, cause }) => {
            assert_eq!(entry, EntryId(2));
            assert!(cause.contains("decode failure"), "{cause}");
        }
        other => panic!("unexpected {other:?}"),
    }
    let (s, checks) = with_file(&[(1, 1), (2, 2)], "verify_provider");
    *checks.fail_for.lock().unwrap() = Some(EntryId(2));
    assert_eq!(
        s.verify(), Err(VerifyError::Entry { entry : EntryId(2), cause : "rejected"
        .into() })
    );
    let (s, checks) = with_file(&[(1, 1)], "verify_stored");
    checks.fail_stored("io");
    assert_eq!(s.verify(), Err(VerifyError::Other("io".into())));
    let mut d = MockDecoder::new(&[(1, 1)]);
    d.fail_rows = true;
    let (s, _) = with_decoder(d, "verify_rows");
    assert!(matches!(s.verify(), Err(VerifyError::Entry { entry : EntryId(1), .. })));
}
#[test]
fn export_is_none_when_clean() {
    let (s, _) = with_file(&[(1, 1)], "export_clean");
    assert_eq!(s.export(), Ok(None));
}
#[test]
fn export_merges_overlay_over_file() {
    let (s, checks) = with_file(&[(1, 5), (2, 6), (3, 7)], "export_merge");
    checks.set(1, 0xdead);
    assert_eq!(s.upsert(EntryId(2), & vec_of(20), b"new"), Ok(Stamp(8)));
    assert_eq!(s.remove(EntryId(3)), Ok(Some(Stamp(9))));
    assert_eq!(s.upsert(EntryId(4), & vec_of(40), b""), Ok(Stamp(10)));
    let exported = s.export().unwrap().unwrap();
    assert_eq!(exported.seq, 3);
    assert_eq!(exported.max_stamp, 10);
    assert_eq!(
        exported.entries, vec![ExportedEntry { id : EntryId(1), stamp : Stamp(5), vector
        : vec_of(1).into(), meta : file_meta(1).into(), checksum : 0xdead, },
        ExportedEntry { id : EntryId(2), stamp : Stamp(8), vector : vec_of(20).into(),
        meta : b"new".to_vec().into(), checksum : mix(EntryId(2), & vec_of(20), b"new"),
        }, ExportedEntry { id : EntryId(4), stamp : Stamp(10), vector : vec_of(40)
        .into(), meta : SharedBytes::default(), checksum : mix(EntryId(4), & vec_of(40),
        b""), },]
    );
}
#[test]
fn export_without_file_is_sorted() {
    let (s, _) = bare();
    for id in [5u64, 2, 9] {
        s.upsert(EntryId(id), &vec_of(id as u8), b"").unwrap();
    }
    s.remove(EntryId(9)).unwrap();
    let exported = s.export().unwrap().unwrap();
    let ids: Vec<u64> = exported.entries.iter().map(|e| e.id.0).collect();
    assert_eq!(ids, vec![2, 5]);
    assert_eq!((exported.seq, exported.max_stamp), (4, 4));
}
#[test]
fn mark_saved_upto_never_lowers() {
    let (s, _) = bare();
    s.upsert(EntryId(1), &vec_of(1), b"").unwrap();
    s.upsert(EntryId(2), &vec_of(2), b"").unwrap();
    let exported = s.export().unwrap().unwrap();
    s.upsert(EntryId(3), &vec_of(3), b"").unwrap();
    s.mark_saved_upto(exported.seq);
    assert!(s.is_dirty());
    s.mark_saved_upto(3);
    assert!(! s.is_dirty());
    assert_eq!(s.export(), Ok(None));
    s.mark_saved_upto(1);
    assert!(! s.is_dirty());
    s.upsert(EntryId(4), &vec_of(4), b"").unwrap();
    assert!(s.is_dirty());
}
#[test]
fn export_errors_change_nothing() {
    let (s, checks) = with_file(&[(1, 1), (2, 2)], "export_errors");
    s.upsert(EntryId(3), &vec_of(3), b"").unwrap();
    checks.fail_stored("io");
    assert!(s.export().is_err());
    assert!(s.is_dirty());
    assert_eq!(s.change_seq(), 1);
    let (s, checks) = with_file(&[(1, 1), (2, 2)], "export_missing");
    s.upsert(EntryId(3), &vec_of(3), b"").unwrap();
    checks.remove_stored(2);
    let err = s.export().unwrap_err();
    assert!(err.0.contains("no stored checksum"), "{err}");
    let mut d = MockDecoder::new(&[(1, 1)]);
    d.short_vector_id = Some(1);
    let (s, _) = with_decoder(d, "export_short");
    s.upsert(EntryId(3), &vec_of(3), b"").unwrap();
    assert!(s.export().is_err());
    let mut d = MockDecoder::new(&[(1, 1)]);
    d.fail_rows = true;
    let (s, _) = with_decoder(d, "export_rows");
    s.upsert(EntryId(3), &vec_of(3), b"").unwrap();
    assert!(s.export().is_err());
    assert!(s.is_dirty());
}
#[test]
fn export_skips_file_when_fully_overlaid() {
    let (s, checks) = with_file(&[(1, 1)], "export_overlaid");
    s.upsert(EntryId(1), &vec_of(5), b"").unwrap();
    checks.fail_stored("io");
    let exported = s.export().unwrap().unwrap();
    assert_eq!(exported.entries.len(), 1);
    assert_eq!(exported.entries[0].stamp, Stamp(2));
}
#[test]
fn concurrent_writers_get_unique_stamps() {
    let (s, _) = bare();
    let s = Arc::new(s);
    let mut handles = Vec::new();
    for t in 0..8u64 {
        let s = Arc::clone(&s);
        handles
            .push(
                std::thread::spawn(move || {
                    let mut stamps = Vec::new();
                    for i in 0..100u64 {
                        let id = EntryId(if i % 2 == 0 { t * 1000 + i } else { 1 });
                        stamps.push(s.upsert(id, &vec_of(t as u8), b"").unwrap().0);
                        let snapshot = s.stamps();
                        assert!(snapshot.windows(2).all(| w | w[0].0 < w[1].0));
                    }
                    stamps
                }),
            );
    }
    let mut all: Vec<u64> = handles
        .into_iter()
        .flat_map(|h| h.join().unwrap())
        .collect();
    all.sort_unstable();
    all.dedup();
    assert_eq!(all.len(), 800);
    assert_eq!(s.max_stamp(), Some(Stamp(800)));
    assert_eq!(s.change_seq(), 800);
}
fn payload_checksums() -> PayloadChecksums {
    PayloadChecksums::new(
        Arc::new(ParquetDecoder::new(header(), Arc::new(compute_checksum_parts))),
    )
}
#[test]
fn payload_checksums_use_the_entry_checksum() {
    let c = payload_checksums();
    let vector = vec_of(3);
    let meta = meta_codec::encode_meta_parts("dir/a.txt", 4, 9);
    assert_eq!(
        c.checksum(EntryId(1), & vector, & meta), Ok(compute_checksum_parts(& vector,
        "dir/a.txt", 4, 9))
    );
    assert_eq!(
        c.checksum(EntryId(1), & vector, b""), Ok(compute_checksum_parts(& vector, "", 0,
        0))
    );
    assert!(c.checksum(EntryId(1), & vector, b"xyz").is_err());
    assert!(c.stored_checksums(Path::new("/nonexistent/entry_store/missing")).is_err());
    let garbage = temp_file("garbage", b"not a parquet file");
    assert!(c.stored_checksums(& garbage).is_err());
}
struct FileRow {
    id: u64,
    stamp: u64,
    vector: Vec<u8>,
    source: (String, u64, u64),
    checksum: Option<u64>,
}
impl FileRow {
    fn new(id: u64, stamp: u64) -> Self {
        FileRow {
            id,
            stamp,
            vector: f16_vector(id as f32),
            source: (format!("a/{id}.txt"), id * 10, id * 10 + 5),
            checksum: None,
        }
    }
    fn meta(&self) -> Vec<u8> {
        meta_codec::encode_meta_parts(&self.source.0, self.source.1, self.source.2)
    }
    fn stored_checksum(&self) -> u64 {
        self.checksum
            .unwrap_or_else(|| {
                compute_checksum_parts(
                    &self.vector,
                    &self.source.0,
                    self.source.1,
                    self.source.2,
                )
            })
    }
}
fn f16_vector(seed: f32) -> Vec<u8> {
    (0..DIM).flat_map(|i| f16::from_f32(seed + i as f32 * 0.5).to_le_bytes()).collect()
}
fn parquet_bytes(rows: &[FileRow], max_stamp: u64) -> Vec<u8> {
    let flat: Vec<f16> = rows
        .iter()
        .flat_map(|r| {
            r.vector
                .chunks(2)
                .map(|c| f16::from_le_bytes([c[0], c[1]]))
                .collect::<Vec<_>>()
        })
        .collect();
    let list = FixedSizeListArray::try_new(
            Arc::new(Field::new("item", DataType::Float16, false)),
            DIM as i32,
            Arc::new(Float16Array::from(flat)),
            None,
        )
        .unwrap();
    let columns: Vec<ArrayRef> = vec![
        Arc::new(UInt64Array::from_iter_values(rows.iter().map(| r | r.id))),
        Arc::new(UInt64Array::from_iter_values(rows.iter().map(| r | r.stamp))),
        Arc::new(list), Arc::new(StringArray::from_iter_values(rows.iter().map(| r | r
        .source.0.as_str()),)), Arc::new(UInt64Array::from_iter_values(rows.iter().map(|
        r | r.source.1))), Arc::new(UInt64Array::from_iter_values(rows.iter().map(| r | r
        .source.2))), Arc::new(UInt64Array::from_iter_values(rows.iter().map(| r | r
        .stored_checksum()),)),
    ];
    let schema = Arc::new(column_schema(DIM).unwrap());
    let kv: Vec<KeyValue> = header()
        .with_max_stamp(max_stamp)
        .to_kv()
        .into_iter()
        .map(|(k, v)| KeyValue::new(k, v))
        .collect();
    let props = WriterProperties::builder().set_key_value_metadata(Some(kv)).build();
    let mut out = Vec::new();
    let mut writer = ArrowWriter::try_new(&mut out, schema.clone(), Some(props))
        .unwrap();
    writer.write(&RecordBatch::try_new(schema, columns).unwrap()).unwrap();
    writer.close().unwrap();
    out
}
fn real_store(rows: &[FileRow], name: &str, verify_on_open: bool) -> EntryStore {
    let path = temp_file(name, &parquet_bytes(rows, 3));
    let decoder = Arc::new(
        ParquetDecoder::new(header(), Arc::new(compute_checksum_parts)),
    );
    let file = LazyIndexFile::new(
        path,
        decoder.clone(),
        OpenOptions {
            verify: verify_on_open,
            ..OpenOptions::default()
        },
    );
    EntryStore::new(
        header(),
        Some(Arc::new(file)),
        Arc::new(PayloadChecksums::new(decoder)),
    )
}
#[test]
fn real_file_round_trip() {
    let rows: Vec<FileRow> = (1..=3).map(|i| FileRow::new(i, i)).collect();
    let s = real_store(&rows, "real_ok", true);
    assert_eq!(s.verify(), Ok(()));
    assert_eq!(s.max_stamp(), Some(Stamp(3)));
    assert_eq!(s.meta(EntryId(1)).map(| m | bytes_of(& m)), Some(rows[0].meta()));
    let vector = f16_vector(50.0);
    let meta = meta_codec::encode_meta_parts("x.txt", 0, 5);
    assert_eq!(s.upsert(EntryId(2), & vector, & meta), Ok(Stamp(4)));
    assert_eq!(s.remove(EntryId(3)), Ok(Some(Stamp(5))));
    assert_eq!(s.verify(), Ok(()));
    let exported = s.export().unwrap().unwrap();
    assert_eq!(exported.max_stamp, 5);
    assert_eq!(exported.entries.len(), 2);
    assert_eq!(exported.entries[0].id, EntryId(1));
    assert_eq!(exported.entries[0].checksum, rows[0].stored_checksum());
    assert_eq!(bytes_of(& exported.entries[0].vector), rows[0].vector);
    assert_eq!(exported.entries[1].stamp, Stamp(4));
    assert_eq!(
        exported.entries[1].checksum, compute_checksum_parts(& vector, "x.txt", 0, 5)
    );
}
#[test]
fn real_file_with_damaged_checksum_stays_detectable() {
    let mut rows: Vec<FileRow> = (1..=3).map(|i| FileRow::new(i, i)).collect();
    rows[1].checksum = Some(1234);
    let s = real_store(&rows, "real_bad", false);
    match s.verify() {
        Err(VerifyError::Entry { entry, cause }) => {
            assert_eq!(entry, EntryId(2));
            assert!(cause.contains("checksum mismatch"), "{cause}");
        }
        other => panic!("unexpected {other:?}"),
    }
    s.upsert(EntryId(9), &f16_vector(1.0), b"").unwrap();
    let exported = s.export().unwrap().unwrap();
    let damaged = exported.entries.iter().find(|e| e.id == EntryId(2)).unwrap();
    assert_eq!(damaged.checksum, 1234);
}
