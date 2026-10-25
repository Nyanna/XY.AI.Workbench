//! Parquet-backed `Index`: stamps are scanned without decoding vectors, vectors and
//! payload are decoded on first use, and every write replaces the file atomically.
use std::collections::BTreeMap;
use std::fmt::Display;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::sync::{Arc, OnceLock};
use arrow::array::{
    ArrayRef, AsArray, FixedSizeListArray, Float16Array, StringArray, UInt64Array,
};
use arrow::datatypes::{DataType, SchemaRef, UInt64Type};
use arrow::record_batch::RecordBatch;
use half::f16;
use memmap2::Mmap;
use parking_lot::Mutex;
use parquet::arrow::arrow_reader::{
    ArrowReaderMetadata, ArrowReaderOptions, ParquetRecordBatchReaderBuilder,
};
use parquet::arrow::{ArrowWriter, ProjectionMask};
use parquet::basic::Compression;
use parquet::file::properties::{EnabledStatistics, WriterProperties};
use parquet::file::reader::{ChunkReader, Length};
use parquet::schema::types::ColumnPath;
use tokio::sync::watch;
use tokio_util::bytes::Bytes;
use super::payload::Payload;
use super::schema::{
    arrow_schema, check_fields, SchemaHeader, COL_CHAR_END, COL_CHAR_START, COL_CHECKSUM,
    COL_ENTRY_ID, COL_STAMP, COL_TARGET_FILE, COL_VECTOR, NAME_VECTOR,
};
use super::{
    AccessMethod, CellId, EntryData, EntryId, EntrySet, Index, NodeId, Stamp,
    StampCounter, SummaryRef,
};
use crate::resident::error::IndexError;
const DEFAULT_ROW_GROUP_ROWS: usize = 8192;
const READ_BATCH_ROWS: usize = 4096;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Access {
    Mmap,
    File,
}
impl Access {
    pub fn as_str(self) -> &'static str {
        match self {
            Access::Mmap => "mmap",
            Access::File => "file",
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct IndexOptions {
    pub access: Access,
    /// Decode everything and check all checksums while opening.
    pub verify: bool,
    pub row_group_rows: usize,
}
impl Default for IndexOptions {
    fn default() -> Self {
        Self {
            access: Access::Mmap,
            verify: true,
            row_group_rows: DEFAULT_ROW_GROUP_ROWS,
        }
    }
}
struct Mapped(Arc<Mmap>);
impl AsRef<[u8]> for Mapped {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}
#[derive(Clone)]
enum Source {
    Mapped(Bytes),
    File(Arc<File>),
}
impl Source {
    fn open(path: &Path, access: Access) -> std::io::Result<Self> {
        let file = File::open(path)?;
        match access {
            Access::File => Ok(Source::File(Arc::new(file))),
            Access::Mmap => {
                let map = unsafe { Mmap::map(&file)? };
                #[cfg(unix)]
                let _ = map.advise(memmap2::Advice::Random);
                Ok(Source::Mapped(Bytes::from_owner(Mapped(Arc::new(map)))))
            }
        }
    }
}
impl Length for Source {
    fn len(&self) -> u64 {
        match self {
            Source::Mapped(b) => b.len() as u64,
            Source::File(f) => Length::len(f.as_ref()),
        }
    }
}
impl ChunkReader for Source {
    type T = Box<dyn Read + Send>;
    fn get_read(&self, start: u64) -> parquet::errors::Result<Self::T> {
        Ok(
            match self {
                Source::Mapped(b) => Box::new(b.get_read(start)?),
                Source::File(f) => Box::new(f.get_read(start)?),
            },
        )
    }
    fn get_bytes(&self, start: u64, length: usize) -> parquet::errors::Result<Bytes> {
        match self {
            Source::Mapped(b) => b.get_bytes(start, length),
            Source::File(f) => f.get_bytes(start, length),
        }
    }
}
fn unavailable<E: Display>(path: &Path) -> impl Fn(E) -> IndexError + '_ {
    move |e| IndexError::IndexUnavailable {
        scope: format!("{}: {e}", path.display()),
    }
}
fn fail(path: &Path, e: impl Display) -> IndexError {
    IndexError::IndexUnavailable {
        scope: format!("{}: {e}", path.display()),
    }
}
/// Fully decoded content of the file the index was opened from.
struct BaseData {
    entry_ids: Vec<u64>,
    vectors: Vec<u8>,
    payloads: Vec<Payload>,
    checksums: Vec<u64>,
}
struct Base {
    path: PathBuf,
    dim: usize,
    backing: Option<(Source, ArrowReaderMetadata)>,
    data: OnceLock<Result<BaseData, String>>,
}
impl Base {
    fn data(&self) -> Result<&BaseData, IndexError> {
        let loaded = self
            .data
            .get_or_init(|| match &self.backing {
                Some((source, meta)) => {
                    load_base(source, meta, self.dim, &self.path)
                        .map_err(|e| e.to_string())
                }
                None => Err("no backing file".to_string()),
            });
        loaded
            .as_ref()
            .map_err(|reason| IndexError::IndexUnavailable {
                scope: format!("{}: {reason}", self.path.display()),
            })
    }
}
fn scan_stamps(
    source: &Source,
    meta: &ArrowReaderMetadata,
    path: &Path,
) -> Result<(BTreeMap<EntryId, Row>, u64), IndexError> {
    let builder = ParquetRecordBatchReaderBuilder::new_with_metadata(
        source.clone(),
        meta.clone(),
    );
    let mask = ProjectionMask::roots(
        builder.parquet_schema(),
        [COL_ENTRY_ID, COL_STAMP],
    );
    let reader = builder.with_projection(mask).build().map_err(unavailable(path))?;
    let mut rows = BTreeMap::new();
    let (mut position, mut max_stamp) = (0u32, 0u64);
    for batch in reader {
        let batch = batch.map_err(unavailable(path))?;
        let ids = batch.column(0).as_primitive::<UInt64Type>();
        let stamps = batch.column(1).as_primitive::<UInt64Type>();
        for (id, stamp) in ids.values().iter().zip(stamps.values().iter()) {
            max_stamp = max_stamp.max(*stamp);
            let row = Row {
                stamp: Stamp::from_raw(*stamp),
                slot: Slot::Base(position),
            };
            if rows.insert(EntryId::new(*id), row).is_some() {
                return Err(IndexError::IndexUnavailable {
                    scope: format!("{}: duplicate entry {id}", path.display()),
                });
            }
            position += 1;
        }
    }
    Ok((rows, max_stamp))
}
fn load_base(
    source: &Source,
    meta: &ArrowReaderMetadata,
    dim: usize,
    path: &Path,
) -> Result<BaseData, IndexError> {
    let total = meta.metadata().file_metadata().num_rows().max(0) as usize;
    let reader = ParquetRecordBatchReaderBuilder::new_with_metadata(
            source.clone(),
            meta.clone(),
        )
        .with_batch_size(READ_BATCH_ROWS)
        .build()
        .map_err(unavailable(path))?;
    let mut data = BaseData {
        entry_ids: Vec::with_capacity(total),
        vectors: Vec::with_capacity(total * dim * 2),
        payloads: Vec::with_capacity(total),
        checksums: Vec::with_capacity(total),
    };
    for batch in reader {
        let batch = batch.map_err(unavailable(path))?;
        let n = batch.num_rows();
        data.entry_ids
            .extend_from_slice(
                batch.column(COL_ENTRY_ID).as_primitive::<UInt64Type>().values(),
            );
        data.checksums
            .extend_from_slice(
                batch.column(COL_CHECKSUM).as_primitive::<UInt64Type>().values(),
            );
        let list = batch.column(COL_VECTOR).as_fixed_size_list();
        let values = list.values().as_primitive::<arrow::datatypes::Float16Type>();
        let start = list.value_offset(0) as usize;
        for v in &values.values()[start..start + n * dim] {
            data.vectors.extend_from_slice(&v.to_le_bytes());
        }
        let files = batch.column(COL_TARGET_FILE).as_string::<i32>();
        let starts = batch.column(COL_CHAR_START).as_primitive::<UInt64Type>();
        let ends = batch.column(COL_CHAR_END).as_primitive::<UInt64Type>();
        for i in 0..n {
            data.payloads
                .push(Payload::new(files.value(i), starts.value(i), ends.value(i)));
        }
    }
    Ok(data)
}
struct MemRow {
    vector: Arc<[u8]>,
    payload: Payload,
    checksum: u64,
}
#[derive(Clone)]
enum Slot {
    Base(u32),
    Mem(Arc<MemRow>),
}
#[derive(Clone)]
struct Row {
    stamp: Stamp,
    slot: Slot,
}
struct Inner {
    rows: BTreeMap<EntryId, Row>,
    revision: u64,
    flushed: u64,
}
/// Index whose persistence is owned by the index itself; `flush` publishes changes to disk.
pub struct ParquetIndex {
    path: PathBuf,
    header: SchemaHeader,
    options: IndexOptions,
    base: Base,
    inner: Mutex<Inner>,
    counter: StampCounter,
    tx: watch::Sender<u64>,
    flush_lock: Mutex<()>,
}
impl ParquetIndex {
    /// A missing file yields an empty index; the first `flush` creates it.
    pub fn open(
        path: impl Into<PathBuf>,
        expected: SchemaHeader,
        options: IndexOptions,
    ) -> Result<Self, IndexError> {
        let path = path.into();
        let span = tracing::info_span!(
            "index.open", path = % path.display(), schema_version = expected
            .schema_version, model_id = % expected.index.model_id, model_revision = %
            expected.model_revision, access = options.access.as_str(), row_groups =
            tracing::field::Empty, checksum_ok = tracing::field::Empty,
        );
        let _guard = span.enter();
        if !path.exists() {
            let base = Base {
                path: path.clone(),
                dim: expected.dim(),
                backing: None,
                data: OnceLock::new(),
            };
            return Ok(
                Self::assemble(
                    path,
                    expected,
                    options,
                    base,
                    BTreeMap::new(),
                    StampCounter::new(),
                ),
            );
        }
        let source = Source::open(&path, options.access).map_err(unavailable(&path))?;
        let meta = ArrowReaderMetadata::load(&source, ArrowReaderOptions::new())
            .map_err(unavailable(&path))?;
        span.record("row_groups", meta.metadata().num_row_groups());
        let kvs = meta
            .metadata()
            .file_metadata()
            .key_value_metadata()
            .map(Vec::as_slice)
            .unwrap_or_default();
        let found = SchemaHeader::from_key_values(kvs)?;
        found.check_against(&expected)?;
        check_fields(meta.schema(), &arrow_schema(expected.dim()))?;
        let (rows, file_max) = scan_stamps(&source, &meta, &path)?;
        let counter = StampCounter::resume(found.index.high_water, file_max);
        let base = Base {
            path: path.clone(),
            dim: expected.dim(),
            backing: Some((source, meta)),
            data: OnceLock::new(),
        };
        let index = Self::assemble(path, expected, options, base, rows, counter);
        if options.verify {
            let verified = index.verify();
            span.record("checksum_ok", verified.is_ok());
            verified?;
        }
        Ok(index)
    }
    fn assemble(
        path: PathBuf,
        header: SchemaHeader,
        options: IndexOptions,
        base: Base,
        rows: BTreeMap<EntryId, Row>,
        counter: StampCounter,
    ) -> Self {
        let (tx, _) = watch::channel(counter.high_water());
        Self {
            path,
            header,
            options,
            base,
            inner: Mutex::new(Inner {
                rows,
                revision: 0,
                flushed: 0,
            }),
            counter,
            tx,
            flush_lock: Mutex::new(()),
        }
    }
    pub fn path(&self) -> &Path {
        &self.path
    }
    pub fn header(&self) -> &SchemaHeader {
        &self.header
    }
    pub fn high_water(&self) -> u64 {
        self.counter.high_water()
    }
    pub fn is_dirty(&self) -> bool {
        let g = self.inner.lock();
        g.revision != g.flushed
    }
    /// Decodes the file and compares every stored checksum.
    pub fn verify(&self) -> Result<(), IndexError> {
        if self.base.backing.is_none() {
            return Ok(());
        }
        let data = self.base.data()?;
        let vb = self.header.vector_bytes();
        for (i, entry) in data.entry_ids.iter().enumerate() {
            let vector = &data.vectors[i * vb..(i + 1) * vb];
            if data.payloads[i].checksum(vector) != data.checksums[i] {
                return Err(IndexError::ChecksumMismatch {
                    entry: *entry,
                });
            }
        }
        Ok(())
    }
    /// Stores a vector (f16 little-endian bytes) together with its payload.
    pub fn upsert(
        &self,
        entry: EntryId,
        vector: &[u8],
        payload: Payload,
    ) -> Result<Stamp, IndexError> {
        let expected = self.header.vector_bytes();
        if vector.len() != expected {
            return Err(IndexError::IncompatibleSchema {
                expected: format!("vector of {expected} bytes"),
                found: format!("vector of {} bytes", vector.len()),
            });
        }
        let checksum = payload.checksum(vector);
        let row = Arc::new(MemRow {
            vector: Arc::from(vector),
            payload,
            checksum,
        });
        let mut g = self.inner.lock();
        let stamp = self.counter.next();
        g.rows.insert(entry, Row { stamp, slot: Slot::Mem(row) });
        g.revision += 1;
        self.tx.send_replace(stamp.raw());
        drop(g);
        tracing::debug!(entry = entry.get(), stamp = stamp.raw(), "upsert");
        Ok(stamp)
    }
    /// Removal consumes a stamp so that re-adding the same id yields a new one.
    pub fn remove(&self, entry: EntryId) -> Option<Stamp> {
        let mut g = self.inner.lock();
        g.rows.remove(&entry)?;
        g.revision += 1;
        let stamp = self.counter.next();
        self.tx.send_replace(stamp.raw());
        Some(stamp)
    }
    pub fn payload(&self, entry: EntryId) -> Option<Payload> {
        let slot = self.inner.lock().rows.get(&entry)?.slot.clone();
        match slot {
            Slot::Mem(m) => Some(m.payload.clone()),
            Slot::Base(i) => {
                match self.base.data() {
                    Ok(d) => Some(d.payloads[i as usize].clone()),
                    Err(e) => {
                        tracing::error!(error = % e, "payload unavailable");
                        None
                    }
                }
            }
        }
    }
    /// Writes all rows to a temporary file and renames it over the index file.
    /// Returns `false` when nothing changed since the last write.
    pub fn flush(&self) -> Result<bool, IndexError> {
        let _serial = self.flush_lock.lock();
        let (rows, high_water, revision) = {
            let g = self.inner.lock();
            if g.revision == g.flushed {
                return Ok(false);
            }
            let rows: Vec<(EntryId, Row)> = g
                .rows
                .iter()
                .map(|(id, r)| (*id, r.clone()))
                .collect();
            (rows, self.counter.high_water(), g.revision)
        };
        let row_group_rows = self.options.row_group_rows.max(1);
        let span = tracing::info_span!(
            "index.write", path = % self.path.display(), schema_version = self.header
            .schema_version, model_id = % self.header.index.model_id, model_revision = %
            self.header.model_revision, rows = rows.len(), row_groups = rows.len()
            .div_ceil(row_group_rows),
        );
        let _guard = span.enter();
        let needs_base = rows.iter().any(|(_, r)| matches!(r.slot, Slot::Base(_)));
        let base = if needs_base { Some(self.base.data()?) } else { None };
        let header = self.header.with_high_water(high_water);
        write_file(&self.path, &header, &rows, base, row_group_rows)?;
        self.inner.lock().flushed = revision;
        Ok(true)
    }
}
fn write_file(
    path: &Path,
    header: &SchemaHeader,
    rows: &[(EntryId, Row)],
    base: Option<&BaseData>,
    row_group_rows: usize,
) -> Result<(), IndexError> {
    let mut tmp_name = path.as_os_str().to_owned();
    tmp_name.push(".tmp");
    let tmp = PathBuf::from(tmp_name);
    let result = write_tmp(&tmp, header, rows, base, row_group_rows, path)
        .and_then(|()| std::fs::rename(&tmp, path).map_err(unavailable(path)))
        .and_then(|()| sync_parent(path));
    if result.is_err() {
        let _ = std::fs::remove_file(&tmp);
    }
    result
}
fn sync_parent(path: &Path) -> Result<(), IndexError> {
    match path.parent().filter(|p| !p.as_os_str().is_empty()) {
        Some(dir) => {
            File::open(dir).and_then(|d| d.sync_all()).map_err(unavailable(path))
        }
        None => Ok(()),
    }
}
fn write_tmp(
    tmp: &Path,
    header: &SchemaHeader,
    rows: &[(EntryId, Row)],
    base: Option<&BaseData>,
    row_group_rows: usize,
    path: &Path,
) -> Result<(), IndexError> {
    let schema = arrow_schema(header.dim());
    let props = WriterProperties::builder()
        .set_key_value_metadata(Some(header.to_key_values()))
        .set_compression(Compression::UNCOMPRESSED)
        .set_max_row_group_row_count(Some(row_group_rows))
        .set_statistics_enabled(EnabledStatistics::Page)
        .set_column_dictionary_enabled(ColumnPath::from(NAME_VECTOR), false)
        .build();
    let file = File::create(tmp).map_err(|e| fail(path, e))?;
    let mut writer = ArrowWriter::try_new(&file, schema.clone(), Some(props))
        .map_err(|e| fail(path, e))?;
    for chunk in rows.chunks(row_group_rows) {
        let batch = build_batch(&schema, header, chunk, base, path)?;
        writer.write(&batch).map_err(|e| fail(path, e))?;
        writer.flush().map_err(|e| fail(path, e))?;
    }
    writer.close().map_err(|e| fail(path, e))?;
    file.sync_all().map_err(|e| fail(path, e))
}
fn build_batch(
    schema: &SchemaRef,
    header: &SchemaHeader,
    chunk: &[(EntryId, Row)],
    base: Option<&BaseData>,
    path: &Path,
) -> Result<RecordBatch, IndexError> {
    let (dim, vb) = (header.dim(), header.vector_bytes());
    let n = chunk.len();
    let (mut ids, mut stamps) = (Vec::with_capacity(n), Vec::with_capacity(n));
    let (mut starts, mut ends, mut sums) = (
        Vec::with_capacity(n),
        Vec::with_capacity(n),
        Vec::with_capacity(n),
    );
    let mut files: Vec<&str> = Vec::with_capacity(n);
    let mut values: Vec<f16> = Vec::with_capacity(n * dim);
    for (id, row) in chunk {
        let (vector, payload, checksum): (&[u8], &Payload, u64) = match &row.slot {
            Slot::Mem(m) => (&m.vector, &m.payload, m.checksum),
            Slot::Base(i) => {
                let b = base.ok_or_else(|| fail(path, "base data missing"))?;
                let i = *i as usize;
                (&b.vectors[i * vb..(i + 1) * vb], &b.payloads[i], b.checksums[i])
            }
        };
        ids.push(id.get());
        stamps.push(row.stamp.raw());
        files.push(&payload.target_file);
        starts.push(payload.char_start);
        ends.push(payload.char_end);
        sums.push(checksum);
        values.extend(vector.chunks_exact(2).map(|c| f16::from_le_bytes([c[0], c[1]])));
    }
    let DataType::FixedSizeList(item, _) = schema.field(COL_VECTOR).data_type() else {
        return Err(fail(path, "vector column is not a fixed size list"));
    };
    let vectors = FixedSizeListArray::try_new(
            item.clone(),
            dim as i32,
            Arc::new(Float16Array::from(values)) as ArrayRef,
            None,
        )
        .map_err(|e| fail(path, e))?;
    let columns: Vec<ArrayRef> = vec![
        Arc::new(UInt64Array::from(ids)), Arc::new(UInt64Array::from(stamps)),
        Arc::new(vectors), Arc::new(StringArray::from(files)),
        Arc::new(UInt64Array::from(starts)), Arc::new(UInt64Array::from(ends)),
        Arc::new(UInt64Array::from(sums)),
    ];
    RecordBatch::try_new(schema.clone(), columns).map_err(|e| fail(path, e))
}
impl Index for ParquetIndex {
    fn stamp(&self, entry: EntryId) -> Option<Stamp> {
        self.inner.lock().rows.get(&entry).map(|r| r.stamp)
    }
    fn stamps(&self) -> Box<dyn Iterator<Item = (EntryId, Stamp)> + '_> {
        let snapshot: Vec<_> = self
            .inner
            .lock()
            .rows
            .iter()
            .map(|(id, r)| (*id, r.stamp))
            .collect();
        Box::new(snapshot.into_iter())
    }
    fn read(&self, entry: EntryId) -> EntryData<'_> {
        let slot = match self.inner.lock().rows.get(&entry) {
            Some(row) => row.slot.clone(),
            None => return EntryData::Borrowed(&[]),
        };
        match slot {
            Slot::Mem(m) => EntryData::Shared(Arc::clone(&m.vector)),
            Slot::Base(i) => {
                match self.base.data() {
                    Ok(d) => {
                        let vb = self.header.vector_bytes();
                        let i = i as usize;
                        EntryData::Shared(Arc::from(&d.vectors[i * vb..(i + 1) * vb]))
                    }
                    Err(e) => {
                        tracing::error!(
                            error = % e, entry = entry.get(), "vector unavailable"
                        );
                        EntryData::Borrowed(&[])
                    }
                }
            }
        }
    }
    /// `data` must be exactly one vector; the payload of an existing entry is kept.
    fn apply(&self, entry: EntryId, data: &[u8]) -> Stamp {
        let payload = self.payload(entry).unwrap_or_default();
        match self.upsert(entry, data, payload) {
            Ok(stamp) => stamp,
            Err(e) => panic!("apply with malformed vector: {e}"),
        }
    }
    fn changed(&self) -> watch::Receiver<u64> {
        self.tx.subscribe()
    }
    fn capabilities(&self) -> &[AccessMethod] {
        &[AccessMethod::Range]
    }
    fn children(&self, _node: NodeId) -> EntrySet {
        EntrySet::new()
    }
    fn members(&self, cell: CellId) -> EntrySet {
        if cell.0 != 0 {
            return EntrySet::new();
        }
        EntrySet::from_ids(self.inner.lock().rows.keys().copied())
    }
    fn summary(&self, _kind: AccessMethod) -> Option<SummaryRef> {
        None
    }
    fn prefetch(&self, _set: &EntrySet) {}
}
#[cfg(test)]
#[path = "test/parquet_io.rs"]
mod tests;
