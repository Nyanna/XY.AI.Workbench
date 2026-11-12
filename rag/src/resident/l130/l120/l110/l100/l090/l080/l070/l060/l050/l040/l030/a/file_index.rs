use std::collections::BTreeMap;
use std::fmt;
use std::fs::{self, File};
use std::io::{self, Read};
use std::os::unix::fs::FileExt;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

use arrow::array::{Array, BinaryArray, BinaryBuilder, UInt64Array};
use arrow::datatypes::{DataType, Field, Schema, SchemaRef};
use arrow::error::ArrowError;
use arrow::record_batch::RecordBatch;
use memmap2::Mmap;
use parking_lot::{Mutex, RwLock};
use parquet::arrow::arrow_reader::{
    ArrowReaderMetadata, ArrowReaderOptions, ParquetRecordBatchReaderBuilder, RowSelection,
    RowSelector,
};
use parquet::arrow::{ArrowWriter, ProjectionMask};
use parquet::basic::Compression;
use parquet::errors::ParquetError;
use parquet::file::metadata::KeyValue;
use parquet::file::properties::{EnabledStatistics, WriterProperties};
use parquet::file::reader::{ChunkReader, Length};
use tokio_util::bytes::Bytes;

use super::super::l020::a::index_contract::{
    AccessMethod, EntryId, Index, IndexError, Payload, PayloadRef, SharedBytes, Stamp,
    StampObserver, StampSignal,
};
use super::super::l020::l010::change_stamp::StampCounter;
use super::super::l020::l010::layout::DType;

const KEY_MODEL_ID: &str = "index.model_id";
const KEY_DIMENSION: &str = "index.dimension";
const KEY_DTYPE: &str = "index.dtype";
const KEY_MAX_STAMP: &str = "index.max_stamp";

const COL_ID: &str = "id";
const COL_STAMP: &str = "stamp";
const COL_VECTOR: &str = "vector";
const COL_META: &str = "meta";

const SCAN_BATCH_ROWS: usize = 65_536;
const WRITE_CHUNK_ROWS: usize = 4_096;
const WRITE_CHUNK_BYTES: usize = 128 * 1024 * 1024;
const GROUP_READ_BATCH_ROWS: usize = 8_192;

/// Identity of the index, stored in the Parquet footer and compared on open.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IndexHeader {
    pub model_id: String,
    pub dimension: usize,
    pub dtype: DType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AccessMode {
    #[default]
    Mmap,
    File,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OpenOptions {
    pub access: AccessMode,
    pub verify: bool,
    pub rows_per_group: i64,
}

impl Default for OpenOptions {
    fn default() -> Self {
        OpenOptions {
            access: AccessMode::Mmap,
            verify: true,
            rows_per_group: 8192,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FileIndexError {
    Io(String),
    HeaderMismatch {
        field: &'static str,
        expected: String,
        found: String,
    },
    Corrupt(String),
}

impl fmt::Display for FileIndexError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FileIndexError::Io(msg) => write!(f, "index file i/o error: {msg}"),
            FileIndexError::HeaderMismatch {
                field,
                expected,
                found,
            } => write!(
                f,
                "index header mismatch in `{field}`: expected `{expected}`, found `{found}`"
            ),
            FileIndexError::Corrupt(msg) => write!(f, "index file is corrupt: {msg}"),
        }
    }
}

impl std::error::Error for FileIndexError {}

fn io_err(e: io::Error) -> FileIndexError {
    FileIndexError::Io(e.to_string())
}

fn parquet_err(e: ParquetError) -> FileIndexError {
    if let ParquetError::External(inner) = &e {
        if let Some(io) = inner.downcast_ref::<io::Error>() {
            return FileIndexError::Io(io.to_string());
        }
    }
    FileIndexError::Corrupt(e.to_string())
}

fn arrow_err(e: ArrowError) -> FileIndexError {
    FileIndexError::Corrupt(e.to_string())
}

fn expected_schema() -> SchemaRef {
    Arc::new(Schema::new(vec![
        Field::new(COL_ID, DataType::UInt64, false),
        Field::new(COL_STAMP, DataType::UInt64, false),
        Field::new(COL_VECTOR, DataType::Binary, false),
        Field::new(COL_META, DataType::Binary, false),
    ]))
}

#[derive(Clone)]
enum Source {
    Mmap(Arc<Mmap>),
    File(Arc<File>, u64),
}

impl Source {
    fn total(&self) -> u64 {
        match self {
            Source::Mmap(m) => m.len() as u64,
            Source::File(_, len) => *len,
        }
    }

    fn read_at(&self, pos: u64, buf: &mut [u8]) -> io::Result<usize> {
        match self {
            Source::Mmap(m) => {
                let total = m.len() as u64;
                if pos >= total {
                    return Ok(0);
                }
                let start = pos as usize;
                let n = buf.len().min(m.len() - start);
                buf[..n].copy_from_slice(&m[start..start + n]);
                Ok(n)
            }
            Source::File(f, len) => {
                if pos >= *len {
                    return Ok(0);
                }
                f.read_at(buf, pos)
            }
        }
    }
}

impl Length for Source {
    fn len(&self) -> u64 {
        self.total()
    }
}

struct SourceRead {
    source: Source,
    pos: u64,
}

impl Read for SourceRead {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let n = self.source.read_at(self.pos, buf)?;
        self.pos += n as u64;
        Ok(n)
    }
}

impl ChunkReader for Source {
    type T = SourceRead;

    fn get_read(&self, start: u64) -> parquet::errors::Result<SourceRead> {
        if start > self.total() {
            return Err(ParquetError::EOF(format!(
                "read start {start} beyond end {}",
                self.total()
            )));
        }
        Ok(SourceRead {
            source: self.clone(),
            pos: start,
        })
    }

    fn get_bytes(&self, start: u64, length: usize) -> parquet::errors::Result<Bytes> {
        let end = start.checked_add(length as u64).unwrap_or(u64::MAX);
        if end > self.total() {
            return Err(ParquetError::EOF(format!(
                "read {start}..{end} beyond end {}",
                self.total()
            )));
        }
        match self {
            Source::Mmap(m) => Ok(Bytes::copy_from_slice(
                &m[start as usize..start as usize + length],
            )),
            Source::File(f, _) => {
                let mut buf = vec![0u8; length];
                f.read_exact_at(&mut buf, start)
                    .map_err(|e| ParquetError::External(Box::new(e)))?;
                Ok(Bytes::from(buf))
            }
        }
    }
}

fn binary_col<'a>(batch: &'a RecordBatch, name: &str) -> Result<&'a BinaryArray, FileIndexError> {
    batch
        .column_by_name(name)
        .and_then(|c| c.as_any().downcast_ref::<BinaryArray>())
        .ok_or_else(|| FileIndexError::Corrupt(format!("column `{name}` is missing or not binary")))
}

fn u64_col<'a>(batch: &'a RecordBatch, name: &str) -> Result<&'a UInt64Array, FileIndexError> {
    batch
        .column_by_name(name)
        .and_then(|c| c.as_any().downcast_ref::<UInt64Array>())
        .ok_or_else(|| FileIndexError::Corrupt(format!("column `{name}` is missing or not u64")))
}

struct DataFile {
    source: Source,
    meta: ArrowReaderMetadata,
    group_starts: Vec<usize>,
    total_rows: usize,
}

impl DataFile {
    fn open_file(file: File, access: AccessMode) -> Result<DataFile, FileIndexError> {
        let len = file.metadata().map_err(io_err)?.len();
        let source = match access {
            AccessMode::Mmap => {
                if len == 0 {
                    return Err(FileIndexError::Corrupt("file is empty".to_string()));
                }
                // SAFETY: the file is not modified by anything else while open; saves replace it by rename.
                let map = unsafe { Mmap::map(&file) }.map_err(io_err)?;
                Source::Mmap(Arc::new(map))
            }
            AccessMode::File => Source::File(Arc::new(file), len),
        };
        let meta =
            ArrowReaderMetadata::load(&source, ArrowReaderOptions::new()).map_err(parquet_err)?;
        let mut group_starts = Vec::new();
        let mut total = 0usize;
        for group in meta.metadata().row_groups() {
            group_starts.push(total);
            total += group.num_rows().max(0) as usize;
        }
        Ok(DataFile {
            source,
            meta,
            group_starts,
            total_rows: total,
        })
    }

    fn footer_value(&self, key: &str) -> Option<String> {
        self.meta
            .metadata()
            .file_metadata()
            .key_value_metadata()
            .and_then(|kv| kv.iter().find(|e| e.key == key))
            .and_then(|e| e.value.clone())
    }

    fn check_header(&self, expected: &IndexHeader) -> Result<(), FileIndexError> {
        let checks = [
            ("model_id", KEY_MODEL_ID, expected.model_id.clone()),
            ("dimension", KEY_DIMENSION, expected.dimension.to_string()),
            ("dtype", KEY_DTYPE, expected.dtype.to_safetensors().to_string()),
        ];
        for (field, key, want) in checks {
            let found = self.footer_value(key);
            if found.as_deref() != Some(want.as_str()) {
                return Err(FileIndexError::HeaderMismatch {
                    field,
                    expected: want,
                    found: found.unwrap_or_else(|| "<missing>".to_string()),
                });
            }
        }
        Ok(())
    }

    fn builder(
        &self,
        columns: &[&str],
    ) -> Result<ParquetRecordBatchReaderBuilder<Source>, FileIndexError> {
        let schema = self.meta.schema();
        let mut indices = Vec::with_capacity(columns.len());
        for name in columns {
            let idx = schema
                .index_of(name)
                .map_err(|_| FileIndexError::Corrupt(format!("column `{name}` is missing")))?;
            indices.push(idx);
        }
        let builder =
            ParquetRecordBatchReaderBuilder::new_with_metadata(self.source.clone(), self.meta.clone());
        let mask = ProjectionMask::roots(builder.parquet_schema(), indices);
        Ok(builder.with_projection(mask))
    }

    fn scan_keys(&self, verify: bool) -> Result<Vec<(u64, u64)>, FileIndexError> {
        let reader = self
            .builder(&[COL_ID, COL_STAMP])?
            .with_batch_size(SCAN_BATCH_ROWS)
            .build()
            .map_err(parquet_err)?;
        let mut out = Vec::with_capacity(self.total_rows);
        let mut prev: Option<u64> = None;
        for batch in reader {
            let batch = batch.map_err(arrow_err)?;
            let ids = u64_col(&batch, COL_ID)?;
            let stamps = u64_col(&batch, COL_STAMP)?;
            for i in 0..batch.num_rows() {
                let id = ids.value(i);
                let stamp = stamps.value(i);
                if verify {
                    if let Some(p) = prev {
                        if id <= p {
                            return Err(FileIndexError::Corrupt(format!(
                                "ids are not strictly ascending at id {id}"
                            )));
                        }
                    }
                    if stamp < 1 {
                        return Err(FileIndexError::Corrupt(format!(
                            "entry {id} has stamp 0"
                        )));
                    }
                }
                prev = Some(id);
                out.push((id, stamp));
            }
        }
        if out.len() != self.total_rows {
            return Err(FileIndexError::Corrupt("row count mismatch".to_string()));
        }
        Ok(out)
    }

    fn verify_vectors(&self, elem_size: usize) -> Result<(), FileIndexError> {
        let reader = self
            .builder(&[COL_VECTOR])?
            .with_batch_size(GROUP_READ_BATCH_ROWS)
            .build()
            .map_err(parquet_err)?;
        for batch in reader {
            let batch = batch.map_err(arrow_err)?;
            let vectors = binary_col(&batch, COL_VECTOR)?;
            for i in 0..batch.num_rows() {
                let len = vectors.value_length(i) as usize;
                if len % elem_size != 0 {
                    return Err(FileIndexError::Corrupt(format!(
                        "vector length {len} is not a multiple of {elem_size}"
                    )));
                }
            }
        }
        Ok(())
    }

    fn group_of(&self, row: usize) -> Result<usize, FileIndexError> {
        if row >= self.total_rows {
            return Err(FileIndexError::Corrupt(format!("row {row} out of range")));
        }
        Ok(self.group_starts.partition_point(|&s| s <= row) - 1)
    }

    fn read_row(&self, row: usize) -> Result<Payload, FileIndexError> {
        let group = self.group_of(row)?;
        let offset = row - self.group_starts[group];
        let mut selectors = Vec::with_capacity(2);
        if offset > 0 {
            selectors.push(RowSelector::skip(offset));
        }
        selectors.push(RowSelector::select(1));
        let mut reader = self
            .builder(&[COL_VECTOR, COL_META])?
            .with_row_groups(vec![group])
            .with_row_selection(RowSelection::from(selectors))
            .with_batch_size(1)
            .build()
            .map_err(parquet_err)?;
        let batch = reader
            .next()
            .ok_or_else(|| FileIndexError::Corrupt("row not found".to_string()))?
            .map_err(arrow_err)?;
        if batch.num_rows() < 1 {
            return Err(FileIndexError::Corrupt("row not found".to_string()));
        }
        let vectors = binary_col(&batch, COL_VECTOR)?;
        let metas = binary_col(&batch, COL_META)?;
        Ok(Payload {
            vector: SharedBytes::from(vectors.value(0)),
            meta: SharedBytes::from(metas.value(0)),
        })
    }

    fn load_group(&self, group: usize) -> Result<Vec<RecordBatch>, FileIndexError> {
        let reader = self
            .builder(&[COL_VECTOR, COL_META])?
            .with_row_groups(vec![group])
            .with_batch_size(GROUP_READ_BATCH_ROWS)
            .build()
            .map_err(parquet_err)?;
        let mut batches = Vec::new();
        for batch in reader {
            batches.push(batch.map_err(arrow_err)?);
        }
        Ok(batches)
    }
}

struct GroupFetcher<'a> {
    file: Option<&'a DataFile>,
    cached: Option<(usize, Vec<RecordBatch>)>,
}

impl<'a> GroupFetcher<'a> {
    fn append(
        &mut self,
        row: usize,
        vectors: &mut BinaryBuilder,
        metas: &mut BinaryBuilder,
    ) -> Result<usize, FileIndexError> {
        let file = self
            .file
            .ok_or_else(|| FileIndexError::Corrupt("entry refers to a missing file".to_string()))?;
        let group = file.group_of(row)?;
        let needs_load = !matches!(&self.cached, Some((g, _)) if *g == group);
        if needs_load {
            self.cached = Some((group, file.load_group(group)?));
        }
        let mut offset = row - file.group_starts[group];
        if let Some((_, batches)) = &self.cached {
            for batch in batches {
                if offset < batch.num_rows() {
                    let v = binary_col(batch, COL_VECTOR)?.value(offset);
                    let m = binary_col(batch, COL_META)?.value(offset);
                    vectors.append_value(v);
                    metas.append_value(m);
                    return Ok(v.len() + m.len());
                }
                offset -= batch.num_rows();
            }
        }
        Err(FileIndexError::Corrupt("row not found in group".to_string()))
    }
}

#[derive(Clone)]
enum Loc {
    File(usize),
    Mem { vector: SharedBytes, meta: SharedBytes },
}

struct Entry {
    stamp: u64,
    loc: Loc,
}

struct State {
    entries: BTreeMap<u64, Entry>,
    file: Option<Arc<DataFile>>,
}

/// Index backed by a single Parquet file; changes live in memory until `save`.
pub struct FileIndex {
    path: PathBuf,
    header: IndexHeader,
    access: AccessMode,
    rows_per_group: usize,
    state: RwLock<State>,
    counter: StampCounter,
    signal: Arc<StampSignal>,
    save_lock: Mutex<()>,
    tmp_seq: AtomicU64,
}

impl FileIndex {
    pub fn open(
        path: impl AsRef<Path>,
        expected: IndexHeader,
        options: OpenOptions,
    ) -> Result<FileIndex, FileIndexError> {
        let path = path.as_ref().to_path_buf();
        let rows_per_group = options.rows_per_group.max(1) as usize;
        let mut entries = BTreeMap::new();
        let mut file = None;
        let mut stored_max = None;

        match File::open(&path) {
            Ok(handle) => {
                let data = DataFile::open_file(handle, options.access)?;
                data.check_header(&expected)?;
                if options.verify {
                    let want = expected_schema();
                    if data.meta.schema().fields() != want.fields() {
                        return Err(FileIndexError::Corrupt("schema mismatch".to_string()));
                    }
                }
                let keys = data.scan_keys(options.verify)?;
                if options.verify {
                    data.verify_vectors(expected.dtype.size_in_bytes())?;
                }
                let raw = data.footer_value(KEY_MAX_STAMP).ok_or_else(|| {
                    FileIndexError::Corrupt(format!("missing footer key `{KEY_MAX_STAMP}`"))
                })?;
                stored_max = Some(raw.parse::<u64>().map_err(|_| {
                    FileIndexError::Corrupt(format!("invalid `{KEY_MAX_STAMP}`: {raw}"))
                })?);
                for (row, (id, stamp)) in keys.into_iter().enumerate() {
                    entries.insert(
                        id,
                        Entry {
                            stamp,
                            loc: Loc::File(row),
                        },
                    );
                }
                file = Some(Arc::new(data));
            }
            Err(e) if e.kind() == io::ErrorKind::NotFound => {}
            Err(e) => return Err(io_err(e)),
        }

        let highest = entries.values().map(|e| e.stamp).max();
        let counter = StampCounter::resume(stored_max, highest);
        let signal = StampSignal::new(Stamp(counter.max_raw().unwrap_or(0)));
        Ok(FileIndex {
            path,
            header: expected,
            access: options.access,
            rows_per_group,
            state: RwLock::new(State { entries, file }),
            counter,
            signal,
            save_lock: Mutex::new(()),
            tmp_seq: AtomicU64::new(0),
        })
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn header(&self) -> &IndexHeader {
        &self.header
    }

    /// Atomically persists the current state.
    pub fn save(&self) -> Result<(), IndexError> {
        let _serial = self.save_lock.lock();
        let (snapshot, old, max_raw) = {
            let st = self.state.read();
            let rows: Vec<(u64, u64, Loc)> = st
                .entries
                .iter()
                .map(|(id, e)| (*id, e.stamp, e.loc.clone()))
                .collect();
            (rows, st.file.clone(), self.counter.max_raw())
        };

        let tmp = self.temp_path();
        let written = self.write_replace(&tmp, &snapshot, old.as_deref(), max_raw);
        let dir_sync = match written {
            Ok(dir_sync) => dir_sync,
            Err(msg) => {
                let _ = fs::remove_file(&tmp);
                return Err(IndexError::Backend(msg));
            }
        };

        let opened = File::open(&self.path)
            .map_err(io_err)
            .and_then(|f| DataFile::open_file(f, self.access));
        match opened {
            Ok(new_file) => {
                let mut st = self.state.write();
                for (row, (id, stamp, _)) in snapshot.iter().enumerate() {
                    if let Some(entry) = st.entries.get_mut(id) {
                        if entry.stamp == *stamp {
                            entry.loc = Loc::File(row);
                        }
                    }
                }
                st.file = Some(Arc::new(new_file));
            }
            Err(e) => return Err(IndexError::Backend(format!("reopen after save failed: {e}"))),
        }
        dir_sync.map_err(IndexError::Backend)
    }

    fn temp_path(&self) -> PathBuf {
        let name = self
            .path
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_else(|| "index".to_string());
        let seq = self.tmp_seq.fetch_add(1, Ordering::Relaxed);
        self.path.with_file_name(format!(
            ".{name}.{}.{seq}.tmp",
            std::process::id()
        ))
    }

    /// Writes the temporary file and renames it over the target. The inner
    /// result reports a failed directory sync after the rename succeeded.
    fn write_replace(
        &self,
        tmp: &Path,
        rows: &[(u64, u64, Loc)],
        old: Option<&DataFile>,
        max_raw: Option<u64>,
    ) -> Result<Result<(), String>, String> {
        let mut file = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(tmp)
            .map_err(|e| format!("cannot create temporary file: {e}"))?;
        self.write_rows(&mut file, rows, old, max_raw)?;
        file.sync_all().map_err(|e| format!("fsync failed: {e}"))?;
        drop(file);
        fs::rename(tmp, &self.path).map_err(|e| format!("rename failed: {e}"))?;
        let dir = match self.path.parent() {
            Some(p) if !p.as_os_str().is_empty() => p.to_path_buf(),
            _ => PathBuf::from("."),
        };
        Ok(File::open(&dir)
            .and_then(|d| d.sync_all())
            .map_err(|e| format!("directory fsync failed: {e}")))
    }

    fn write_rows(
        &self,
        file: &mut File,
        rows: &[(u64, u64, Loc)],
        old: Option<&DataFile>,
        max_raw: Option<u64>,
    ) -> Result<(), String> {
        let schema = expected_schema();
        let kv = vec![
            KeyValue::new(KEY_MODEL_ID.to_string(), self.header.model_id.clone()),
            KeyValue::new(KEY_DIMENSION.to_string(), self.header.dimension.to_string()),
            KeyValue::new(
                KEY_DTYPE.to_string(),
                self.header.dtype.to_safetensors().to_string(),
            ),
            KeyValue::new(KEY_MAX_STAMP.to_string(), max_raw.unwrap_or(0).to_string()),
        ];
        let props = WriterProperties::builder()
            .set_compression(Compression::UNCOMPRESSED)
            .set_dictionary_enabled(false)
            .set_statistics_enabled(EnabledStatistics::None)
            .set_max_row_group_row_count(Some(self.rows_per_group))
            .set_max_row_group_bytes(None)
            .set_key_value_metadata(Some(kv))
            .build();
        let mut writer = ArrowWriter::try_new(&mut *file, schema.clone(), Some(props))
            .map_err(|e| format!("cannot start writer: {e}"))?;

        let chunk_rows = self.rows_per_group.min(WRITE_CHUNK_ROWS);
        let mut fetcher = GroupFetcher {
            file: old,
            cached: None,
        };
        let mut ids: Vec<u64> = Vec::new();
        let mut stamps: Vec<u64> = Vec::new();
        let mut vectors = BinaryBuilder::new();
        let mut metas = BinaryBuilder::new();
        let mut bytes = 0usize;

        for (id, stamp, loc) in rows {
            match loc {
                Loc::Mem { vector, meta } => {
                    vectors.append_value(&vector[..]);
                    metas.append_value(&meta[..]);
                    bytes += vector.len() + meta.len();
                }
                Loc::File(row) => {
                    bytes += fetcher
                        .append(*row, &mut vectors, &mut metas)
                        .map_err(|e| e.to_string())?;
                }
            }
            ids.push(*id);
            stamps.push(*stamp);
            if ids.len() >= chunk_rows || bytes >= WRITE_CHUNK_BYTES {
                flush_chunk(&mut writer, &schema, &mut ids, &mut stamps, &mut vectors, &mut metas)?;
                bytes = 0;
            }
        }
        if !ids.is_empty() {
            flush_chunk(&mut writer, &schema, &mut ids, &mut stamps, &mut vectors, &mut metas)?;
        }
        writer
            .close()
            .map_err(|e| format!("cannot finish file: {e}"))?;
        Ok(())
    }
}

fn flush_chunk(
    writer: &mut ArrowWriter<&mut File>,
    schema: &SchemaRef,
    ids: &mut Vec<u64>,
    stamps: &mut Vec<u64>,
    vectors: &mut BinaryBuilder,
    metas: &mut BinaryBuilder,
) -> Result<(), String> {
    let batch = RecordBatch::try_new(
        schema.clone(),
        vec![
            Arc::new(UInt64Array::from(std::mem::take(ids))),
            Arc::new(UInt64Array::from(std::mem::take(stamps))),
            Arc::new(vectors.finish()),
            Arc::new(metas.finish()),
        ],
    )
    .map_err(|e| format!("cannot build batch: {e}"))?;
    writer
        .write(&batch)
        .map_err(|e| format!("cannot write batch: {e}"))
}

impl Index for FileIndex {
    fn dtype(&self) -> DType {
        self.header.dtype
    }

    fn access_methods(&self) -> &[AccessMethod] {
        &[]
    }

    fn stamp(&self, id: EntryId) -> Option<Stamp> {
        self.state.read().entries.get(&id.0).map(|e| Stamp(e.stamp))
    }

    fn stamps(&self) -> Vec<(EntryId, Stamp)> {
        self.state
            .read()
            .entries
            .iter()
            .map(|(id, e)| (EntryId(*id), Stamp(e.stamp)))
            .collect()
    }

    fn read(&self, id: EntryId) -> Payload {
        let (file, row) = {
            let st = self.state.read();
            match st.entries.get(&id.0) {
                None => return Payload::default(),
                Some(entry) => match &entry.loc {
                    Loc::Mem { vector, meta } => {
                        return Payload {
                            vector: vector.clone(),
                            meta: meta.clone(),
                        }
                    }
                    Loc::File(row) => match &st.file {
                        Some(f) => (f.clone(), *row),
                        None => return Payload::default(),
                    },
                },
            }
        };
        file.read_row(row).unwrap_or_default()
    }

    fn write(&self, id: EntryId, payload: PayloadRef<'_>) -> Result<Stamp, IndexError> {
        let elem_size = self.header.dtype.size_in_bytes();
        if payload.vector.len() % elem_size != 0 {
            return Err(IndexError::InvalidVectorLength {
                len: payload.vector.len(),
                elem_size,
            });
        }
        let vector = SharedBytes::from(payload.vector);
        let meta = SharedBytes::from(payload.meta);
        let mut st = self.state.write();
        let next = self
            .counter
            .next()
            .map_err(|e| IndexError::Backend(e.to_string()))?;
        let stamp = Stamp(next.as_raw());
        st.entries.insert(
            id.0,
            Entry {
                stamp: stamp.0,
                loc: Loc::Mem { vector, meta },
            },
        );
        self.signal.publish(stamp);
        Ok(stamp)
    }

    fn delete(&self, id: EntryId) -> Result<Option<Stamp>, IndexError> {
        let mut st = self.state.write();
        if !st.entries.contains_key(&id.0) {
            return Ok(None);
        }
        let next = self
            .counter
            .next()
            .map_err(|e| IndexError::Backend(e.to_string()))?;
        let stamp = Stamp(next.as_raw());
        st.entries.remove(&id.0);
        self.signal.publish(stamp);
        Ok(Some(stamp))
    }

    fn subscribe(&self) -> StampObserver {
        self.signal.observer()
    }
}

#[cfg(test)]
#[path = "test/file_index.rs"]
mod tests;
