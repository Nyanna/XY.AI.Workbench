//! Atomic persistence of a consistent index snapshot as a Parquet index file.
use std::fs::{self, File, OpenOptions};
use std::io;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, PoisonError};
use arrow::array::{ArrayRef, FixedSizeListArray, Float16Array, StringArray, UInt64Array};
use arrow::datatypes::{DataType, Schema, SchemaRef};
use arrow::record_batch::RecordBatch;
use half::f16;
use parquet::arrow::{ArrowSchemaConverter, ArrowWriter};
use parquet::basic::Compression;
use parquet::file::metadata::KeyValue;
use parquet::file::properties::{EnabledStatistics, WriterProperties};
use parquet::schema::types::ColumnPath;
use tracing::{debug, error, info, warn};
use super::super::entry_store::{EntryStore, Exported};
use super::entry_payload::{EntryData, SourceRef};
use crate::resident::_d::_c::_b::_a::_9::_8::a::index_format::{
    IndexHeader, COL_VECTOR,
};
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::a::index_contract::{
    EntryId, Stamp,
};
const BATCH_ROWS2: usize = 4096;
/// Opaque marker of the state a snapshot represents.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SnapshotToken(pub u64);
#[derive(Debug, Clone)]
pub struct SnapshotEntry {
    pub id: EntryId,
    pub stamp: Stamp,
    pub data: EntryData,
}
/// Consistent snapshot; `entries` is ascending by `id` without duplicates.
#[derive(Debug, Clone)]
pub struct Snapshot {
    pub token: SnapshotToken,
    pub max_stamp: u64,
    pub entries: Vec<SnapshotEntry>,
}
pub trait SnapshotSource: Send + Sync {
    /// `None` when nothing changed. Does not alter the dirty state.
    fn snapshot(&self) -> Option<Snapshot>;
    /// Called only after a successful write; clears dirty state up to `token`.
    fn mark_saved(&self, token: SnapshotToken);
}
#[derive(Debug, Clone)]
pub struct SaveConfig {
    pub path: PathBuf,
    pub header: IndexHeader,
    pub row_group_rows: usize,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SaveOutcome {
    Unchanged,
    Written { rows: u64, row_groups: usize },
}
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum SaveError {
    #[error("Index nicht verfügbar: {path}: {cause}")]
    Unavailable { path: PathBuf, cause: String },
}
pub struct IndexSaver {
    config: SaveConfig,
    source: Arc<dyn SnapshotSource>,
    lock: Mutex<()>,
}
impl IndexSaver {
    pub fn new(config: SaveConfig, source: Arc<dyn SnapshotSource>) -> IndexSaver {
        IndexSaver {
            config,
            source,
            lock: Mutex::new(()),
        }
    }
    pub fn path(&self) -> &Path {
        &self.config.path
    }
    /// Blocking. Concurrent calls are serialized.
    pub fn save(&self) -> Result<SaveOutcome, SaveError> {
        self.save_with(&sync_directory)
    }
    fn save_with(
        &self,
        sync_dir: &dyn Fn(&Path) -> io::Result<()>,
    ) -> Result<SaveOutcome, SaveError> {
        let _serial = self.lock.lock().unwrap_or_else(PoisonError::into_inner);
        let Some(snapshot) = self.source.snapshot() else {
            debug!(
                path = % self.config.path.display(), "index unchanged, nothing written"
            );
            return Ok(SaveOutcome::Unchanged);
        };
        match self.persist(&snapshot, sync_dir) {
            Ok(outcome) => {
                self.source.mark_saved(snapshot.token);
                if let SaveOutcome::Written { rows, row_groups } = outcome {
                    let header = &self.config.header;
                    info!(
                        path = % self.config.path.display(), schema_version = header
                        .schema_version, model_id = % header.model_id, model_revision = %
                        header.model_revision, rows, row_groups, "index saved"
                    );
                }
                Ok(outcome)
            }
            Err(cause) => {
                error!(
                    path = % self.config.path.display(), % cause, "index save failed"
                );
                Err(SaveError::Unavailable {
                    path: self.config.path.clone(),
                    cause,
                })
            }
        }
    }
    fn persist(
        &self,
        snapshot: &Snapshot,
        sync_dir: &dyn Fn(&Path) -> io::Result<()>,
    ) -> Result<SaveOutcome, String> {
        let config = &self.config;
        if config.row_group_rows == 0 {
            return Err("row_group_rows must be at least 1".to_owned());
        }
        let schema: SchemaRef = Arc::new(
            config.header.column_schema().map_err(|e| e.to_string())?,
        );
        validate3(snapshot, &config.header)?;
        let props = writer_properties(config, snapshot, &schema)?;
        let tmp = temp_path(&config.path)?;
        let (rows, row_groups) = match write_temp(
            &tmp,
            &schema,
            props,
            config,
            snapshot,
        ) {
            Ok(done) => done,
            Err(cause) => {
                remove_temp(&tmp);
                return Err(cause);
            }
        };
        if let Err(e) = fs::rename(&tmp, &config.path) {
            remove_temp(&tmp);
            return Err(format!("rename failed: {e}"));
        }
        sync_dir(parent_dir(&config.path))
            .map_err(|e| format!("directory fsync failed: {e}"))?;
        Ok(SaveOutcome::Written {
            rows,
            row_groups,
        })
    }
}
fn parent_dir(path: &Path) -> &Path {
    match path.parent() {
        Some(dir) if !dir.as_os_str().is_empty() => dir,
        _ => Path::new("."),
    }
}
fn sync_directory(dir: &Path) -> io::Result<()> {
    File::open(dir)?.sync_all()
}
fn temp_path(path: &Path) -> Result<PathBuf, String> {
    let name = path
        .file_name()
        .ok_or_else(|| "target path has no file name".to_owned())?;
    let mut tmp_name = name.to_os_string();
    tmp_name.push(".tmp");
    Ok(path.with_file_name(tmp_name))
}
fn remove_temp(tmp: &Path) {
    match fs::remove_file(tmp) {
        Ok(()) => {}
        Err(e) if e.kind() == io::ErrorKind::NotFound => {}
        Err(e) => {
            warn!(path = % tmp.display(), error = % e, "cannot remove temporary file")
        }
    }
}
fn validate3(snapshot: &Snapshot, header: &IndexHeader) -> Result<(), String> {
    let expected = header.vector_bytes();
    let mut previous: Option<EntryId> = None;
    for entry in &snapshot.entries {
        if let Some(prev) = previous {
            if entry.id <= prev {
                return Err(
                    format!(
                        "entries not strictly ascending: id {} follows id {}", entry.id
                        .0, prev.0
                    ),
                );
            }
        }
        previous = Some(entry.id);
        let stamp = entry.stamp.0;
        if stamp == 0 || stamp > snapshot.max_stamp {
            return Err(
                format!(
                    "entry {}: stamp {} outside 1..={}", entry.id.0, stamp, snapshot
                    .max_stamp
                ),
            );
        }
        let len = entry.data.vector.len();
        if len != expected {
            return Err(
                format!(
                    "entry {}: vector has {} bytes, expected {}", entry.id.0, len,
                    expected
                ),
            );
        }
    }
    Ok(())
}
fn vector_leaf_path(schema: &Schema) -> Result<ColumnPath, String> {
    let descriptor = ArrowSchemaConverter::new()
        .convert(schema)
        .map_err(|e| format!("cannot derive parquet schema: {e}"))?;
    descriptor
        .columns()
        .iter()
        .find(|c| c.path().parts().first().map(String::as_str) == Some(COL_VECTOR))
        .map(|c| c.path().clone())
        .ok_or_else(|| format!("column {COL_VECTOR} missing in parquet schema"))
}
fn writer_properties(
    config: &SaveConfig,
    snapshot: &Snapshot,
    schema: &Schema,
) -> Result<WriterProperties, String> {
    let kv: Vec<KeyValue> = config
        .header
        .with_max_stamp(snapshot.max_stamp)
        .to_kv()
        .into_iter()
        .map(|(key, value)| KeyValue::new(key, value))
        .collect();
    Ok(
        WriterProperties::builder()
            .set_compression(Compression::UNCOMPRESSED)
            .set_statistics_enabled(EnabledStatistics::Page)
            .set_column_dictionary_enabled(vector_leaf_path(schema)?, false)
            .set_max_row_group_row_count(Some(config.row_group_rows))
            .set_max_row_group_bytes(None)
            .set_key_value_metadata(Some(kv))
            .build(),
    )
}
fn write_temp(
    tmp: &Path,
    schema: &SchemaRef,
    props: WriterProperties,
    config: &SaveConfig,
    snapshot: &Snapshot,
) -> Result<(u64, usize), String> {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(tmp)
        .map_err(|e| format!("cannot create temporary file {}: {e}", tmp.display()))?;
    let mut writer = ArrowWriter::try_new(&file, schema.clone(), Some(props))
        .map_err(|e| format!("cannot start parquet writer: {e}"))?;
    let batch_rows = config.row_group_rows.min(BATCH_ROWS2);
    for chunk in snapshot.entries.chunks(batch_rows) {
        let batch = build_batch(schema, chunk, config.header.dimension)?;
        writer.write(&batch).map_err(|e| format!("cannot write rows: {e}"))?;
    }
    let metadata = writer
        .close()
        .map_err(|e| format!("cannot finish parquet file: {e}"))?;
    file.sync_all().map_err(|e| format!("fsync failed: {e}"))?;
    Ok((snapshot.entries.len() as u64, metadata.num_row_groups()))
}
fn build_batch(
    schema: &SchemaRef,
    chunk: &[SnapshotEntry],
    dimension: usize,
) -> Result<RecordBatch, String> {
    let item = match schema.field_with_name(COL_VECTOR).map(|f| f.data_type()) {
        Ok(DataType::FixedSizeList(item, _)) => item.clone(),
        _ => return Err(format!("column {COL_VECTOR} is not a fixed size list")),
    };
    let size = i32::try_from(dimension)
        .map_err(|_| format!("dimension {dimension} too large"))?;
    let mut values: Vec<f16> = Vec::with_capacity(chunk.len() * dimension);
    for entry in chunk {
        values
            .extend(
                entry
                    .data
                    .vector
                    .chunks_exact(2)
                    .map(|b| f16::from_le_bytes([b[0], b[1]])),
            );
    }
    let vectors = FixedSizeListArray::try_new(
            item,
            size,
            Arc::new(Float16Array::from(values)),
            None,
        )
        .map_err(|e| format!("cannot build vector column: {e}"))?;
    let columns: Vec<ArrayRef> = vec![
        Arc::new(UInt64Array::from_iter_values(chunk.iter().map(| e | e.id.0))),
        Arc::new(UInt64Array::from_iter_values(chunk.iter().map(| e | e.stamp.0))),
        Arc::new(vectors), Arc::new(StringArray::from_iter_values(chunk.iter().map(| e |
        e.data.source.target_file.as_str()),)),
        Arc::new(UInt64Array::from_iter_values(chunk.iter().map(| e | e.data.source
        .char_start))), Arc::new(UInt64Array::from_iter_values(chunk.iter().map(| e | e
        .data.source.char_end))), Arc::new(UInt64Array::from_iter_values(chunk.iter()
        .map(| e | e.data.checksum()))),
    ];
    RecordBatch::try_new(schema.clone(), columns)
        .map_err(|e| format!("cannot build batch: {e}"))
}
/// Snapshot source backed by an `EntryStore`.
pub struct StoreSource {
    store: Arc<EntryStore>,
}
impl StoreSource {
    pub fn new(store: Arc<EntryStore>) -> Self {
        StoreSource { store }
    }
}
fn into_snapshot(exported: Exported) -> Result<Snapshot, String> {
    let mut entries = Vec::with_capacity(exported.entries.len());
    for e in exported.entries {
        let source = SourceRef::from_meta(&e.meta)
            .map_err(|err| format!("entry {}: {err}", e.id.0))?;
        entries
            .push(SnapshotEntry {
                id: e.id,
                stamp: e.stamp,
                data: EntryData::from_stored(e.vector, source, e.checksum),
            });
    }
    Ok(Snapshot {
        token: SnapshotToken(exported.seq),
        max_stamp: exported.max_stamp,
        entries,
    })
}
impl SnapshotSource for StoreSource {
    fn snapshot(&self) -> Option<Snapshot> {
        match self.store.export() {
            Ok(None) => None,
            Ok(Some(exported)) => {
                match into_snapshot(exported) {
                    Ok(snapshot) => Some(snapshot),
                    Err(e) => {
                        error!(error = % e, "cannot build snapshot from entry store");
                        None
                    }
                }
            }
            Err(e) => {
                error!(error = % e, "cannot export entry store");
                None
            }
        }
    }
    fn mark_saved(&self, token: SnapshotToken) {
        self.store.mark_saved_upto(token.0);
    }
}
#[cfg(test)]
#[path = "test/index_save.rs"]
mod tests;
