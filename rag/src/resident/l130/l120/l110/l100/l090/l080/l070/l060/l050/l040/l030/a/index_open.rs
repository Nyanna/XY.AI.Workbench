use std::fs::File;
use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::sync::{Arc, OnceLock};

use tracing::{error, info, warn};

use super::super::l020::a::index_contract::{EntryId, Payload, SharedBytes, Stamp};
use super::super::l020::l010::change_stamp::StampCounter;
use super::super::l020::l010::page_advice::{advise, Advice, ByteRange, MmapAdvisor};

pub const BATCH_ROWS: usize = 4096;

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum OpenError {
    #[error("Index nicht verfügbar: {path}: {cause}")]
    Unavailable { path: PathBuf, cause: String },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccessKind {
    Mmap,
    FileRead,
}

#[derive(Debug, Clone)]
pub struct OpenOptions {
    pub use_mmap: bool,
    pub verify: bool,
}

impl Default for OpenOptions {
    fn default() -> Self {
        OpenOptions {
            use_mmap: true,
            verify: true,
        }
    }
}

/// File information supplied by the format decoder, without vector data.
#[derive(Debug, Clone)]
pub struct FileInfo {
    pub schema_version: u32,
    pub model_id: String,
    pub model_revision: String,
    pub row_groups: usize,
    pub rows: u64,
    pub stored_max_stamp: Option<u64>,
}

/// Decoded rows of one batch; row index is relative to the batch start.
pub struct DecodedBatch {
    pub vectors: Vec<SharedBytes>,
    pub metas: Vec<SharedBytes>,
}

pub trait FormatDecoder: Send + Sync {
    fn read_info(&self, bytes: &[u8]) -> Result<FileInfo, String>;
    fn read_ids_stamps(&self, bytes: &[u8], info: &FileInfo) -> Result<Vec<(u64, u64)>, String>;
    fn decode_rows(
        &self,
        bytes: &[u8],
        info: &FileInfo,
        start: u64,
        end: u64,
    ) -> Result<DecodedBatch, String>;
    fn verify(&self, bytes: &[u8], info: &FileInfo) -> Result<(), String>;
}

enum Backing {
    Mapped(Arc<memmap2::Mmap>),
    Heap(Vec<u8>),
}

impl Backing {
    fn bytes(&self) -> &[u8] {
        match self {
            Backing::Mapped(map) => map.deref(),
            Backing::Heap(data) => data.as_slice(),
        }
    }
}

struct Entry {
    id: EntryId,
    stamp: Stamp,
    row: u64,
}

struct Loaded {
    info: FileInfo,
    access: AccessKind,
    backing: Backing,
    entries: Vec<Entry>,
    counter: Arc<StampCounter>,
    batches: Vec<OnceLock<Result<DecodedBatch, String>>>,
}

/// Lazily opened index file. IDs and stamps load on first access, vectors and payloads per batch.
pub struct LazyIndexFile {
    path: PathBuf,
    decoder: Arc<dyn FormatDecoder>,
    options: OpenOptions,
    state: OnceLock<Result<Loaded, OpenError>>,
}

enum RowLookup<'a> {
    Unknown,
    Unavailable,
    Failed,
    Found(&'a DecodedBatch, usize),
}

impl LazyIndexFile {
    pub fn new(
        path: impl Into<PathBuf>,
        decoder: Arc<dyn FormatDecoder>,
        options: OpenOptions,
    ) -> Self {
        LazyIndexFile {
            path: path.into(),
            decoder,
            options,
            state: OnceLock::new(),
        }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn ensure_open(&self) -> Result<(), OpenError> {
        self.loaded().map(|_| ())
    }

    pub fn info(&self) -> Result<&FileInfo, OpenError> {
        self.loaded().map(|l| &l.info)
    }

    pub fn access_kind(&self) -> Result<AccessKind, OpenError> {
        self.loaded().map(|l| l.access)
    }

    pub fn stamp_counter(&self) -> Result<Arc<StampCounter>, OpenError> {
        self.loaded().map(|l| Arc::clone(&l.counter))
    }

    pub fn stamps(&self) -> Result<Vec<(EntryId, Stamp)>, OpenError> {
        self.loaded()
            .map(|l| l.entries.iter().map(|e| (e.id, e.stamp)).collect())
    }

    pub fn stamp(&self, id: EntryId) -> Result<Option<Stamp>, OpenError> {
        self.loaded()
            .map(|l| Self::find(l, id).map(|i| l.entries[i].stamp))
    }

    pub fn read_vector(&self, id: EntryId) -> SharedBytes {
        match self.lookup(id) {
            RowLookup::Found(batch, i) => batch.vectors.get(i).cloned().unwrap_or_default(),
            _ => SharedBytes::default(),
        }
    }

    pub fn read_meta(&self, id: EntryId) -> Option<SharedBytes> {
        match self.lookup(id) {
            RowLookup::Found(batch, i) => Some(batch.metas.get(i).cloned().unwrap_or_default()),
            _ => None,
        }
    }

    pub fn read(&self, id: EntryId) -> Payload {
        match self.lookup(id) {
            RowLookup::Found(batch, i) => Payload {
                vector: batch.vectors.get(i).cloned().unwrap_or_default(),
                meta: batch.metas.get(i).cloned().unwrap_or_default(),
            },
            _ => Payload::default(),
        }
    }

    fn loaded(&self) -> Result<&Loaded, OpenError> {
        match self.state.get_or_init(|| self.open()) {
            Ok(loaded) => Ok(loaded),
            Err(err) => Err(err.clone()),
        }
    }

    fn find(loaded: &Loaded, id: EntryId) -> Option<usize> {
        loaded.entries.binary_search_by(|e| e.id.cmp(&id)).ok()
    }

    fn lookup(&self, id: EntryId) -> RowLookup<'_> {
        let loaded = match self.loaded() {
            Ok(loaded) => loaded,
            Err(err) => {
                error!(path = %self.path.display(), cause = %err, "read on unavailable index");
                return RowLookup::Unavailable;
            }
        };
        let Some(pos) = Self::find(loaded, id) else {
            return RowLookup::Unknown;
        };
        let row = loaded.entries[pos].row;
        let batch_no = (row / BATCH_ROWS as u64) as usize;
        let start = batch_no as u64 * BATCH_ROWS as u64;
        let end = (start + BATCH_ROWS as u64).min(loaded.info.rows);
        let result = loaded.batches[batch_no].get_or_init(|| {
            self.decoder
                .decode_rows(loaded.backing.bytes(), &loaded.info, start, end)
        });
        match result {
            Ok(batch) => RowLookup::Found(batch, (row - start) as usize),
            Err(cause) => {
                error!(
                    path = %self.path.display(),
                    batch = batch_no,
                    cause = %cause,
                    "batch decoding failed"
                );
                RowLookup::Failed
            }
        }
    }

    fn fail(&self, cause: impl Into<String>) -> OpenError {
        let cause = cause.into();
        error!(path = %self.path.display(), cause = %cause, "index open failed");
        OpenError::Unavailable {
            path: self.path.clone(),
            cause,
        }
    }

    fn load_backing(&self) -> Result<(Backing, AccessKind), OpenError> {
        if self.options.use_mmap {
            match File::open(&self.path).and_then(|f| {
                // SAFETY: the mapping is read-only; index files are replaced, not edited in place.
                unsafe { memmap2::Mmap::map(&f) }
            }) {
                Ok(map) => {
                    let map = Arc::new(map);
                    let len = map.len() as u64;
                    let advisor = MmapAdvisor::new(Arc::clone(&map));
                    if let Err(err) =
                        advise(&advisor, Advice::Random, ByteRange { start: 0, end: len })
                    {
                        warn!(path = %self.path.display(), error = %err, "page advice failed");
                    }
                    return Ok((Backing::Mapped(map), AccessKind::Mmap));
                }
                Err(err) => {
                    warn!(
                        path = %self.path.display(),
                        error = %err,
                        "mmap failed, falling back to file read"
                    );
                }
            }
        }
        match std::fs::read(&self.path) {
            Ok(data) => Ok((Backing::Heap(data), AccessKind::FileRead)),
            Err(err) => Err(self.fail(format!("read failed: {err}"))),
        }
    }

    fn open(&self) -> Result<Loaded, OpenError> {
        let (backing, access) = self.load_backing()?;
        let bytes = backing.bytes();

        let info = self.decoder.read_info(bytes).map_err(|c| self.fail(c))?;
        let pairs = self
            .decoder
            .read_ids_stamps(bytes, &info)
            .map_err(|c| self.fail(c))?;
        if pairs.len() as u64 != info.rows {
            return Err(self.fail(format!(
                "row count mismatch: header {} vs. {} ids",
                info.rows,
                pairs.len()
            )));
        }

        let highest = pairs.iter().map(|&(_, s)| s).max();
        let mut entries: Vec<Entry> = pairs
            .iter()
            .enumerate()
            .map(|(row, &(id, stamp))| Entry {
                id: EntryId(id),
                stamp: Stamp(stamp),
                row: row as u64,
            })
            .collect();
        entries.sort_by_key(|e| e.id);
        if let Some(w) = entries.windows(2).find(|w| w[0].id == w[1].id) {
            return Err(self.fail(format!("duplicate id {}", w[0].id.0)));
        }

        let counter = Arc::new(StampCounter::resume(info.stored_max_stamp, highest));

        if self.options.verify {
            self.decoder
                .verify(bytes, &info)
                .map_err(|c| self.fail(c))?;
        }

        let batch_count = (info.rows as usize).div_ceil(BATCH_ROWS);
        let batches = (0..batch_count).map(|_| OnceLock::new()).collect();

        info!(
            path = %self.path.display(),
            schema_version = info.schema_version,
            model_id = %info.model_id,
            model_revision = %info.model_revision,
            access = ?access,
            row_groups = info.row_groups,
            checksum = if self.options.verify { "ok" } else { "übersprungen" },
            "index opened"
        );

        Ok(Loaded {
            info,
            access,
            backing,
            entries,
            counter,
            batches,
        })
    }
}

#[cfg(test)]
#[path = "test/index_open.rs"]
mod tests;
