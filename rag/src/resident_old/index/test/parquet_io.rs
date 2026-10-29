use std::sync::atomic::{AtomicU32, Ordering};

use super::*;
use crate::resident::memory::DType;
use crate::resident::model::ModelSpec;

struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        static NEXT: AtomicU32 = AtomicU32::new(0);
        let dir = std::env::temp_dir().join(format!(
            "xyrag-parquet-{}-{}-{tag}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        std::fs::create_dir_all(&dir).unwrap();
        Self(dir)
    }

    fn file(&self) -> PathBuf {
        self.0.join("index.parquet")
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

fn header() -> SchemaHeader {
    let spec = ModelSpec::e5_small(Path::new("/nonexistent"), DType::F16);
    SchemaHeader::from_spec(&spec, "rev-1", "xxh3_64:0000000000000001").unwrap()
}

fn vector(seed: u64) -> Vec<u8> {
    (0..384u64)
        .flat_map(|j| f16::from_f32(((seed * 31 + j * 7) % 1000) as f32 / 1000.0 - 0.5).to_le_bytes())
        .collect()
}

fn id(n: u64) -> EntryId {
    EntryId::new(n)
}

fn open(dir: &TempDir) -> ParquetIndex {
    ParquetIndex::open(dir.file(), header(), IndexOptions::default()).unwrap()
}

fn populated(dir: &TempDir, rows: u64, options: IndexOptions) -> Vec<Stamp> {
    let idx = ParquetIndex::open(dir.file(), header(), options).unwrap();
    let stamps = (0..rows)
        .map(|i| {
            let payload = Payload::new(format!("f{}.txt", i % 7), i * 10, i * 10 + 9);
            idx.upsert(id(i), &vector(i), payload).unwrap()
        })
        .collect();
    assert!(idx.flush().unwrap());
    stamps
}

fn footer_of(path: &Path) -> Vec<parquet::file::metadata::KeyValue> {
    let builder = ParquetRecordBatchReaderBuilder::try_new(File::open(path).unwrap()).unwrap();
    builder.metadata().file_metadata().key_value_metadata().cloned().unwrap_or_default()
}

#[test]
fn roundtrip_is_bit_exact_across_row_groups() {
    let dir = TempDir::new("roundtrip");
    let options = IndexOptions { row_group_rows: 1000, ..IndexOptions::default() };
    let stamps = populated(&dir, 4000, options);

    let builder = ParquetRecordBatchReaderBuilder::try_new(File::open(dir.file()).unwrap()).unwrap();
    assert_eq!(builder.metadata().num_row_groups(), 4);
    assert_eq!(builder.schema().field(COL_VECTOR).data_type(), arrow_schema(384).field(COL_VECTOR).data_type());

    let idx = ParquetIndex::open(dir.file(), header(), options).unwrap();
    for i in 0..4000u64 {
        assert_eq!(idx.read(id(i)).as_bytes(), &vector(i)[..], "vector {i}");
        assert_eq!(idx.stamp(id(i)), Some(stamps[i as usize]));
        assert_eq!(idx.payload(id(i)), Some(Payload::new(format!("f{}.txt", i % 7), i * 10, i * 10 + 9)));
    }
    assert!(idx.read(id(4000)).as_bytes().is_empty());
}

#[test]
fn footer_carries_every_header_key() {
    let dir = TempDir::new("header");
    populated(&dir, 3, IndexOptions::default());
    let kvs = footer_of(&dir.file());
    assert!(kvs.iter().any(|kv| kv.key == "ARROW:schema"));
    let found = SchemaHeader::from_key_values(&kvs).unwrap();
    assert_eq!(found.index.high_water, 3);
    assert_eq!(found, header().with_high_water(3));
    assert_eq!(found.index.model_id, "intfloat/multilingual-e5-small");
    assert_eq!(found.prefix_query, "query: ");
    assert_eq!(found.compute_dtype, "cpu=f32;gpu=f32");
}

#[test]
fn binding_mismatch_is_rejected_with_the_offending_key() {
    let dir = TempDir::new("mismatch");
    populated(&dir, 2, IndexOptions::default());
    let cases: Vec<(&str, Box<dyn Fn(&mut SchemaHeader)>)> = vec![
        ("embedding.dim", Box::new(|h| h.index.dim = 768)),
        ("embedding.model_id", Box::new(|h| h.index.model_id = "other/model".into())),
        ("embedding.model_revision", Box::new(|h| h.model_revision = "rev-2".into())),
        ("embedding.prefix_query", Box::new(|h| h.prefix_query = "q: ".into())),
        ("embedding.prefix_passage", Box::new(|h| h.prefix_passage = String::new())),
        ("embedding.pooling", Box::new(|h| h.pooling = crate::resident::model::Pooling::PerToken)),
        ("embedding.normalize", Box::new(|h| h.normalize = crate::resident::model::Normalize::None)),
        ("embedding.max_seq_len", Box::new(|h| h.max_seq_len = 256)),
        ("tokenizer.sha", Box::new(|h| h.tokenizer_sha = "other".into())),
        ("resident.schema_version", Box::new(|h| h.schema_version += 1)),
    ];
    for (key, mutate) in cases {
        let mut other = header();
        mutate(&mut other);
        match ParquetIndex::open(dir.file(), other, IndexOptions::default()) {
            Err(IndexError::IncompatibleSchema { expected, found }) => {
                assert!(expected.contains(key), "{key}: {expected}");
                assert!(found.contains(key), "{key}: {found}");
            }
            other => panic!("{key}: unexpected {:?}", other.map(|_| ())),
        }
    }
}

#[test]
fn mixed_compute_dtype_is_documented_not_enforced() {
    let dir = TempDir::new("compute");
    populated(&dir, 2, IndexOptions::default());
    let mut other = header();
    other.compute_dtype = "cpu=f32;gpu=f16".into();
    assert!(ParquetIndex::open(dir.file(), other, IndexOptions::default()).is_ok());
}

#[test]
fn missing_header_key_and_foreign_storage_dtype_are_incompatible() {
    let dir = TempDir::new("keys");
    populated(&dir, 1, IndexOptions::default());
    let mut kvs = footer_of(&dir.file());
    kvs.retain(|kv| kv.key != "tokenizer.sha");
    match SchemaHeader::from_key_values(&kvs) {
        Err(IndexError::IncompatibleSchema { found, .. }) => assert!(found.contains("tokenizer.sha=<missing>")),
        other => panic!("unexpected {:?}", other.map(|_| ())),
    }
    let spec = ModelSpec::e5_small(Path::new("/nonexistent"), DType::F32);
    assert!(matches!(
        SchemaHeader::from_spec(&spec, "r", "t"),
        Err(IndexError::IncompatibleSchema { .. })
    ));
}

#[test]
fn corrupted_vector_bytes_fail_the_checksum() {
    let dir = TempDir::new("checksum");
    populated(&dir, 3, IndexOptions::default());
    let mut bytes = std::fs::read(dir.file()).unwrap();
    let needle = vector(1);
    let at = bytes.windows(needle.len()).position(|w| w == &needle[..]).expect("plain vector bytes in file");
    bytes[at + 100] ^= 0x01;
    std::fs::write(dir.file(), &bytes).unwrap();

    let strict = ParquetIndex::open(dir.file(), header(), IndexOptions::default());
    assert!(matches!(strict, Err(IndexError::ChecksumMismatch { entry: 1 })));

    let lazy = IndexOptions { verify: false, ..IndexOptions::default() };
    let idx = ParquetIndex::open(dir.file(), header(), lazy).unwrap();
    assert!(matches!(idx.verify(), Err(IndexError::ChecksumMismatch { entry: 1 })));
}

#[test]
fn stamps_do_not_decode_vectors() {
    let dir = TempDir::new("stamps");
    let expected = populated(&dir, 50, IndexOptions::default());
    let lazy = IndexOptions { verify: false, ..IndexOptions::default() };
    let idx = ParquetIndex::open(dir.file(), header(), lazy).unwrap();

    let seen: Vec<_> = idx.stamps().collect();
    assert_eq!(seen.len(), 50);
    assert!(seen.iter().zip(&expected).all(|((_, s), e)| s == e));
    assert_eq!(idx.stamp(id(7)), Some(expected[7]));
    assert!(idx.base.data.get().is_none());

    assert_eq!(idx.read(id(3)).as_bytes(), &vector(3)[..]);
    assert!(idx.base.data.get().is_some());
}

#[test]
fn mmap_and_file_access_agree() {
    let dir = TempDir::new("access");
    populated(&dir, 120, IndexOptions { row_group_rows: 50, ..IndexOptions::default() });
    let mapped = ParquetIndex::open(dir.file(), header(), IndexOptions { access: Access::Mmap, ..IndexOptions::default() }).unwrap();
    let plain = ParquetIndex::open(dir.file(), header(), IndexOptions { access: Access::File, ..IndexOptions::default() }).unwrap();
    assert_eq!(mapped.stamps().collect::<Vec<_>>(), plain.stamps().collect::<Vec<_>>());
    for i in 0..120 {
        assert_eq!(mapped.read(id(i)).as_bytes(), plain.read(id(i)).as_bytes());
        assert_eq!(mapped.payload(id(i)), plain.payload(id(i)));
    }
}

#[test]
fn rewrite_is_atomic_and_keeps_existing_mappings_valid() {
    let dir = TempDir::new("atomic");
    populated(&dir, 10, IndexOptions::default());
    let lazy = IndexOptions { verify: false, ..IndexOptions::default() };
    let reader = ParquetIndex::open(dir.file(), header(), lazy).unwrap();

    let writer = open(&dir);
    writer.upsert(id(0), &vector(999), Payload::new("new.txt", 1, 2)).unwrap();
    assert!(writer.flush().unwrap());

    let mut tmp = dir.file().into_os_string();
    tmp.push(".tmp");
    assert!(!Path::new(&tmp).exists());
    assert_eq!(reader.read(id(0)).as_bytes(), &vector(0)[..]);
    assert_eq!(open(&dir).read(id(0)).as_bytes(), &vector(999)[..]);
}

#[test]
fn apply_returns_its_own_stamp_signals_and_keeps_the_payload() {
    let dir = TempDir::new("apply");
    populated(&dir, 4, IndexOptions::default());
    let idx = open(&dir);
    let mut rx = idx.changed();
    assert!(!rx.has_changed().unwrap());

    let stamp = idx.apply(id(2), &vector(500));
    assert_eq!(idx.stamp(id(2)), Some(stamp));
    assert!(rx.has_changed().unwrap());
    assert_eq!(*rx.borrow_and_update(), stamp.raw());
    assert_eq!(idx.payload(id(2)), Some(Payload::new("f2.txt", 20, 29)));
    assert_eq!(idx.read(id(2)).as_bytes(), &vector(500)[..]);
    assert!(idx.is_dirty());

    idx.flush().unwrap();
    assert!(!idx.is_dirty());
    let reopened = open(&dir);
    assert_eq!(reopened.payload(id(2)), Some(Payload::new("f2.txt", 20, 29)));
    assert_eq!(reopened.stamp(id(2)), Some(stamp));
}

#[test]
fn stamps_survive_restart_without_reissue() {
    let dir = TempDir::new("resume");
    populated(&dir, 3, IndexOptions::default());
    let idx = open(&dir);
    let removed = idx.remove(id(2)).unwrap();
    idx.flush().unwrap();
    drop(idx);

    let reopened = open(&dir);
    assert_eq!(reopened.stamp(id(2)), None);
    let fresh = reopened.apply(id(2), &vector(2));
    assert!(fresh.raw() > removed.raw());
}

#[test]
fn missing_file_is_empty_and_emptied_index_roundtrips() {
    let dir = TempDir::new("empty");
    let idx = open(&dir);
    assert_eq!(idx.stamps().count(), 0);
    assert!(!idx.is_dirty());
    assert!(!idx.flush().unwrap());
    assert!(!dir.file().exists());

    idx.upsert(id(1), &vector(1), Payload::default()).unwrap();
    idx.remove(id(1)).unwrap();
    assert!(idx.flush().unwrap());
    let reopened = open(&dir);
    assert_eq!(reopened.stamps().count(), 0);
    assert!(reopened.high_water() >= 2);
}

#[test]
fn malformed_vector_is_rejected() {
    let dir = TempDir::new("malformed");
    let idx = open(&dir);
    let r = idx.upsert(id(1), &[0u8; 10], Payload::default());
    assert!(matches!(r, Err(IndexError::IncompatibleSchema { .. })));
    assert_eq!(idx.stamp(id(1)), None);
}

#[test]
#[should_panic(expected = "malformed vector")]
fn apply_panics_on_contract_violation() {
    let dir = TempDir::new("panic");
    open(&dir).apply(id(1), &[0u8; 10]);
}

#[test]
fn checksum_covers_vector_and_payload() {
    let p = Payload::new("a.txt", 1, 2);
    let v = vector(1);
    let base = p.checksum(&v);
    assert_eq!(base, p.checksum(&v));
    assert_ne!(base, Payload::new("b.txt", 1, 2).checksum(&v));
    assert_ne!(base, Payload::new("a.txt", 1, 3).checksum(&v));
    assert_ne!(base, p.checksum(&vector(2)));
}
