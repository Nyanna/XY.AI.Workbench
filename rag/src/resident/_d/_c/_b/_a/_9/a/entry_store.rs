//! In-memory entry management over a lazily opened index file.
use std::collections::{BTreeMap, HashMap};
use std::path::Path;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, OnceLock, RwLock, RwLockReadGuard, RwLockWriteGuard};
use tracing::{error, info};
use super::persistent_save::entry_payload::compute_checksum_parts;
use crate::resident::_d::_c::_b::_a::_9::_8::a::index_format::IndexHeader;
use crate::resident::_d::_c::_b::_a::_9::_8::a::meta_codec;
use crate::resident::_d::_c::_b::_a::_9::_8::a::parquet_decoder::ParquetDecoder;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::a::index_open::{
    LazyIndexFile, OpenError,
};
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::a::index_contract::{
    AccessMethod, EntryId, Index, IndexError, Payload2, PayloadRef, SharedBytes, Stamp,
    StampObserver, StampSignal,
};
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::_1::change_stamp::{
    StampCounter, StampError,
};
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::_1::layout::DType;
pub trait ChecksumProvider: Send + Sync {
    fn checksum(&self, id: EntryId, vector: &[u8], meta: &[u8]) -> Result<u64, String>;
    fn stored_checksums(&self, path: &Path) -> Result<Vec<(EntryId, u64)>, String>;
}
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum EntryError {
    #[error("Schema-Fehler: erwartet {expected} Bytes, gefunden {found}")]
    Schema { expected: usize, found: usize },
    #[error("invalid payload: {0}")]
    Meta(String),
    #[error(transparent)]
    Unavailable(#[from] OpenError),
    #[error(transparent)]
    Stamp(#[from] StampError),
}
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum VerifyError {
    #[error("Verifikation fehlgeschlagen bei Entry {entry:?}: {cause}")]
    Entry { entry: EntryId, cause: String },
    #[error(transparent)]
    Unavailable(#[from] OpenError),
    #[error("Verifikation fehlgeschlagen: {0}")]
    Other(String),
}
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("export failed: {0}")]
pub struct ExportError(pub String);
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExportedEntry {
    pub id: EntryId,
    pub stamp: Stamp,
    pub vector: SharedBytes,
    pub meta: SharedBytes,
    pub checksum: u64,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Exported {
    pub seq: u64,
    pub max_stamp: u64,
    pub entries: Vec<ExportedEntry>,
}
#[derive(Clone)]
struct Live {
    vector: SharedBytes,
    meta: SharedBytes,
    stamp: Stamp,
    checksum: u64,
}
#[derive(Clone)]
enum Slot9 {
    Live(Live),
    Tombstone,
}
struct Runtime {
    counter: Arc<StampCounter>,
    signal: Arc<StampSignal>,
}
enum MetaSource<'a> {
    Given(&'a [u8]),
    Keep,
}
pub struct EntryStore {
    header: IndexHeader,
    file: Option<Arc<LazyIndexFile>>,
    checksum: Arc<dyn ChecksumProvider>,
    overlay: RwLock<BTreeMap<EntryId, Slot9>>,
    change_seq: AtomicU64,
    saved_seq: AtomicU64,
    runtime: OnceLock<Result<Runtime, OpenError>>,
}
impl EntryStore {
    pub fn new(
        header: IndexHeader,
        file: Option<Arc<LazyIndexFile>>,
        checksum: Arc<dyn ChecksumProvider>,
    ) -> Self {
        EntryStore {
            header,
            file,
            checksum,
            overlay: RwLock::new(BTreeMap::new()),
            change_seq: AtomicU64::new(0),
            saved_seq: AtomicU64::new(0),
            runtime: OnceLock::new(),
        }
    }
    pub fn upsert(
        &self,
        id: EntryId,
        vector: &[u8],
        meta: &[u8],
    ) -> Result<Stamp, EntryError> {
        self.upsert_with(id, vector, MetaSource::Given(meta))
    }
    pub fn remove(&self, id: EntryId) -> Result<Option<Stamp>, EntryError> {
        let rt = self.runtime()?;
        let mut overlay = self.overlay_write();
        let in_file = match &self.file {
            Some(file) => file.stamp(id)?.is_some(),
            None => false,
        };
        let exists = match overlay.get(&id) {
            Some(Slot9::Live(_)) => true,
            Some(Slot9::Tombstone) => false,
            None => in_file,
        };
        if !exists {
            return Ok(None);
        }
        let stamp = rt.counter.next()?;
        if in_file {
            overlay.insert(id, Slot9::Tombstone);
        } else {
            overlay.remove(&id);
        }
        self.change_seq.fetch_add(1, Ordering::SeqCst);
        rt.signal.publish(stamp);
        info!(entry = id.0, stamp = stamp.0, "entry removed");
        Ok(Some(stamp))
    }
    pub fn meta(&self, id: EntryId) -> Option<SharedBytes> {
        match self.overlay_lookup(id) {
            Some(Slot9::Live(live)) => Some(live.meta),
            Some(Slot9::Tombstone) => None,
            None => self.file.as_ref().and_then(|file| file.read_meta(id)),
        }
    }
    pub fn path(&self) -> Option<&Path> {
        self.file.as_ref().map(|file| file.path())
    }
    pub fn header(&self) -> IndexHeader {
        self.header.with_max_stamp(self.max_stamp().map_or(0, |s| s.0))
    }
    pub fn max_stamp(&self) -> Option<Stamp> {
        let rt = self.runtime().ok()?;
        rt.counter.max()
    }
    pub fn is_dirty(&self) -> bool {
        self.change_seq.load(Ordering::SeqCst) > self.saved_seq.load(Ordering::SeqCst)
    }
    pub fn change_seq(&self) -> u64 {
        self.change_seq.load(Ordering::SeqCst)
    }
    pub fn mark_saved_upto(&self, seq: u64) {
        self.saved_seq.fetch_max(seq, Ordering::SeqCst);
    }
    pub fn verify(&self) -> Result<(), VerifyError> {
        let Some(file) = &self.file else {
            return Ok(());
        };
        file.ensure_open()?;
        let mut stored = self
            .checksum
            .stored_checksums(file.path())
            .map_err(VerifyError::Other)?;
        stored.sort_by_key(|&(id, _)| id);
        let expected_len = self.header.vector_bytes();
        for (id, stored_checksum) in stored {
            let fail = |cause: String| VerifyError::Entry {
                entry: id,
                cause,
            };
            let payload = file.read(id);
            if payload.vector.len() != expected_len {
                return Err(
                    fail(
                        format!(
                            "decode failure: vector has {} bytes, expected {}", payload
                            .vector.len(), expected_len
                        ),
                    ),
                );
            }
            let computed = self
                .checksum
                .checksum(id, &payload.vector, &payload.meta)
                .map_err(fail)?;
            if computed != stored_checksum {
                return Err(
                    fail(
                        format!(
                            "checksum mismatch: stored {stored_checksum:016x}, computed {computed:016x}"
                        ),
                    ),
                );
            }
        }
        Ok(())
    }
    pub fn export(&self) -> Result<Option<Exported>, ExportError> {
        if !self.is_dirty() {
            return Ok(None);
        }
        let rt = self.runtime().map_err(|e| ExportError(e.to_string()))?;
        let (seq, max_stamp, overlay) = {
            let guard = self.overlay_read();
            let seq = self.change_seq.load(Ordering::SeqCst);
            if seq <= self.saved_seq.load(Ordering::SeqCst) {
                return Ok(None);
            }
            (seq, rt.counter.max_raw().unwrap_or(0), guard.clone())
        };
        let mut entries = Vec::new();
        if let Some(file) = &self.file {
            self.export_file_rows(file, &overlay, &mut entries)?;
        }
        for (id, slot) in overlay {
            if let Slot9::Live(live) = slot {
                entries
                    .push(ExportedEntry {
                        id,
                        stamp: live.stamp,
                        vector: live.vector,
                        meta: live.meta,
                        checksum: live.checksum,
                    });
            }
        }
        entries.sort_by_key(|e| e.id);
        Ok(
            Some(Exported {
                seq,
                max_stamp,
                entries,
            }),
        )
    }
    fn export_file_rows(
        &self,
        file: &LazyIndexFile,
        overlay: &BTreeMap<EntryId, Slot9>,
        out: &mut Vec<ExportedEntry>,
    ) -> Result<(), ExportError> {
        let rows: Vec<(EntryId, Stamp)> = file
            .stamps()
            .map_err(|e| ExportError(e.to_string()))?
            .into_iter()
            .filter(|(id, _)| !overlay.contains_key(id))
            .collect();
        if rows.is_empty() {
            return Ok(());
        }
        let stored: HashMap<EntryId, u64> = self
            .checksum
            .stored_checksums(file.path())
            .map_err(ExportError)?
            .into_iter()
            .collect();
        let expected_len = self.header.vector_bytes();
        out.reserve(rows.len());
        for (id, stamp) in rows {
            let vector = file.read_vector(id);
            if vector.is_empty() || vector.len() != expected_len {
                return Err(
                    ExportError(
                        format!(
                            "entry {}: decode failure, vector has {} bytes, expected {}",
                            id.0, vector.len(), expected_len
                        ),
                    ),
                );
            }
            let Some(meta) = file.read_meta(id) else {
                return Err(
                    ExportError(format!("entry {}: decode failure, no payload", id.0)),
                );
            };
            let Some(&checksum) = stored.get(&id) else {
                return Err(ExportError(format!("entry {}: no stored checksum", id.0)));
            };
            out.push(ExportedEntry {
                id,
                stamp,
                vector,
                meta,
                checksum,
            });
        }
        Ok(())
    }
    fn upsert_with(
        &self,
        id: EntryId,
        vector: &[u8],
        meta: MetaSource<'_>,
    ) -> Result<Stamp, EntryError> {
        let expected = self.header.vector_bytes();
        if vector.is_empty() || vector.len() != expected {
            return Err(EntryError::Schema {
                expected,
                found: vector.len(),
            });
        }
        let rt = self.runtime()?;
        let mut overlay = self.overlay_write();
        let meta = match meta {
            MetaSource::Given(bytes) => SharedBytes::from(bytes),
            MetaSource::Keep => self.kept_meta(&overlay, id),
        };
        let checksum = self
            .checksum
            .checksum(id, vector, &meta)
            .map_err(EntryError::Meta)?;
        let stamp = rt.counter.next()?;
        let meta_len = meta.len();
        overlay
            .insert(
                id,
                Slot9::Live(Live {
                    vector: SharedBytes::from(vector),
                    meta,
                    stamp,
                    checksum,
                }),
            );
        self.change_seq.fetch_add(1, Ordering::SeqCst);
        rt.signal.publish(stamp);
        info!(
            entry = id.0, stamp = stamp.0, vector_len = vector.len(), meta_len,
            "entry upserted"
        );
        Ok(stamp)
    }
    fn kept_meta(&self, overlay: &BTreeMap<EntryId, Slot9>, id: EntryId) -> SharedBytes {
        match overlay.get(&id) {
            Some(Slot9::Live(live)) => live.meta.clone(),
            Some(Slot9::Tombstone) => SharedBytes::default(),
            None => {
                self.file
                    .as_ref()
                    .and_then(|file| file.read_meta(id))
                    .unwrap_or_default()
            }
        }
    }
    fn runtime(&self) -> Result<&Runtime, OpenError> {
        self.runtime.get_or_init(|| self.open_runtime()).as_ref().map_err(|e| e.clone())
    }
    fn open_runtime(&self) -> Result<Runtime, OpenError> {
        let counter = match &self.file {
            Some(file) => file.stamp_counter()?,
            None => Arc::new(StampCounter::new()),
        };
        let signal = StampSignal::new(Stamp(counter.max_raw().unwrap_or(0)));
        Ok(Runtime { counter, signal })
    }
    fn overlay_read(&self) -> RwLockReadGuard<'_, BTreeMap<EntryId, Slot9>> {
        self.overlay.read().unwrap_or_else(|e| e.into_inner())
    }
    fn overlay_write(&self) -> RwLockWriteGuard<'_, BTreeMap<EntryId, Slot9>> {
        self.overlay.write().unwrap_or_else(|e| e.into_inner())
    }
    fn overlay_lookup(&self, id: EntryId) -> Option<Slot9> {
        self.overlay_read().get(&id).cloned()
    }
    fn file_stamps(&self) -> Vec<(EntryId, Stamp)> {
        let Some(file) = &self.file else {
            return Vec::new();
        };
        file.stamps()
            .unwrap_or_else(|err| {
                error!(
                    path = % file.path().display(), cause = % err,
                    "stamps of unavailable index file"
                );
                Vec::new()
            })
    }
}
fn merge_stamps(
    file: Vec<(EntryId, Stamp)>,
    overlay: Vec<(EntryId, Option<Stamp>)>,
) -> Vec<(EntryId, Stamp)> {
    let mut out = Vec::with_capacity(file.len() + overlay.len());
    let mut file = file.into_iter().peekable();
    let mut overlay = overlay.into_iter().peekable();
    loop {
        let take_file = match (file.peek(), overlay.peek()) {
            (None, None) => break,
            (Some(_), None) => true,
            (None, Some(_)) => false,
            (Some(&(file_id, _)), Some(&(overlay_id, _))) => {
                if file_id == overlay_id {
                    file.next();
                }
                file_id < overlay_id
            }
        };
        if take_file {
            out.extend(file.next());
        } else if let Some((id, Some(stamp))) = overlay.next() {
            out.push((id, stamp));
        }
    }
    out
}
impl Index for EntryStore {
    fn dtype(&self) -> DType {
        DType::F16
    }
    fn access_methods(&self) -> &[AccessMethod] {
        &[]
    }
    fn stamp(&self, id: EntryId) -> Option<Stamp> {
        match self.overlay_lookup(id) {
            Some(Slot9::Live(live)) => Some(live.stamp),
            Some(Slot9::Tombstone) => None,
            None => {
                let file = self.file.as_ref()?;
                file.stamp(id)
                    .unwrap_or_else(|err| {
                        error!(
                            path = % file.path().display(), cause = % err,
                            "stamp of unavailable index file"
                        );
                        None
                    })
            }
        }
    }
    fn stamps(&self) -> Vec<(EntryId, Stamp)> {
        let file = self.file_stamps();
        let overlay: Vec<(EntryId, Option<Stamp>)> = self
            .overlay_read()
            .iter()
            .map(|(id, slot)| match slot {
                Slot9::Live(live) => (*id, Some(live.stamp)),
                Slot9::Tombstone => (*id, None),
            })
            .collect();
        merge_stamps(file, overlay)
    }
    fn read(&self, id: EntryId) -> Payload2 {
        match self.overlay_lookup(id) {
            Some(Slot9::Live(live)) => {
                Payload2 {
                    vector: live.vector,
                    meta: live.meta,
                }
            }
            Some(Slot9::Tombstone) => Payload2::default(),
            None => self.file.as_ref().map(|file| file.read(id)).unwrap_or_default(),
        }
    }
    fn write(&self, id: EntryId, payload: PayloadRef<'_>) -> Result<Stamp, IndexError> {
        let expected = self.header.vector_bytes();
        assert_eq!(
            payload.vector.len(), expected,
            "vector length does not match the index dimension"
        );
        self.upsert_with(id, payload.vector, MetaSource::Keep)
            .map_err(|e| IndexError::Backend(e.to_string()))
    }
    fn delete(&self, id: EntryId) -> Result<Option<Stamp>, IndexError> {
        self.remove(id).map_err(|e| IndexError::Backend(e.to_string()))
    }
    fn subscribe(&self) -> StampObserver {
        match self.runtime() {
            Ok(rt) => rt.signal.observer(),
            Err(_) => StampSignal::new(Stamp(0)).observer(),
        }
    }
}
pub struct PayloadChecksums {
    decoder: Arc<ParquetDecoder>,
}
impl PayloadChecksums {
    pub fn new(decoder: Arc<ParquetDecoder>) -> Self {
        PayloadChecksums { decoder }
    }
}
impl ChecksumProvider for PayloadChecksums {
    fn checksum(&self, _id: EntryId, vector: &[u8], meta: &[u8]) -> Result<u64, String> {
        let fields = meta_codec::decode_meta(meta).map_err(|e| e.to_string())?;
        Ok(
            compute_checksum_parts(
                vector,
                &fields.target_file,
                fields.char_start,
                fields.char_end,
            ),
        )
    }
    fn stored_checksums(&self, path: &Path) -> Result<Vec<(EntryId, u64)>, String> {
        let bytes = std::fs::read(path).map_err(|e| format!("{}: {e}", path.display()))?;
        self.decoder
            .read_stored_checksums(&bytes)
            .map(|pairs| pairs.into_iter().map(|(id, sum)| (EntryId(id), sum)).collect())
            .map_err(|e| e.to_string())
    }
}
#[cfg(test)]
#[path = "test/entry_store.rs"]
mod tests;
