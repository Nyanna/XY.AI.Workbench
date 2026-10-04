//! Shared, disk-persisted infrastructure under `.xyrag`.
//!
//! Contains the central shared index (change detection across layer
//! boundaries) and an abstract interface through which every layer manages
//! its own persistence units (cache entries for chunks, lines, files,
//! composite objects, ...), without the engine prescribing the structure
//! of these units.
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use anyhow::Result;
use rusqlite::{params, Connection, OptionalExtension};
use serde_json::Value;
pub const PERSISTENCE_DIRNAME: &str = ".xyrag";
/// Resolves the RAG root directory: explicit argument, or CWD.
pub fn resolve_root(root: Option<&Path>) -> Result<PathBuf> {
    let base = match root {
        Some(p) => p.to_path_buf(),
        None => std::env::current_dir()?,
    };
    std::fs::create_dir_all(&base)?;
    Ok(base)
}
/// Derives the persistence storage directory (`<root>/.xyrag`) from the RAG root.
pub fn resolve_storagedir(root: &Path) -> Result<PathBuf> {
    let storagedir = root.join(PERSISTENCE_DIRNAME);
    std::fs::create_dir_all(&storagedir)?;
    Ok(storagedir)
}
/// One row of the central file index.
#[derive(Debug, Clone)]
pub struct FileRecord {
    pub path: String,
    pub hash: Option<String>,
    pub size: Option<i64>,
    pub mtime: Option<f64>,
    pub seq: i64,
    pub deleted: bool,
}
/// Central, shared index for change detection across all layers.
///
/// Minimal base: path, hash, size, mtime and a monotonic sequence number
/// per entry; deletions as tombstones. Concrete layers use this as the
/// shared source of "has something changed".
pub struct SharedIndex {
    conn: Mutex<Connection>,
}
fn row_to_record(row: &rusqlite::Row<'_>) -> rusqlite::Result<FileRecord> {
    Ok(FileRecord {
        path: row.get(0)?,
        hash: row.get(1)?,
        size: row.get(2)?,
        mtime: row.get(3)?,
        seq: row.get(4)?,
        deleted: row.get::<_, i64>(5)? != 0,
    })
}
impl SharedIndex {
    pub fn open(db_path: &Path) -> Result<Self> {
        let conn = Connection::open(db_path)?;
        conn.execute_batch(
            "PRAGMA journal_mode=WAL;
             CREATE TABLE IF NOT EXISTS file_index (
                 path TEXT PRIMARY KEY,
                 hash TEXT,
                 size INTEGER,
                 mtime REAL,
                 seq INTEGER,
                 deleted INTEGER NOT NULL DEFAULT 0
             );
             CREATE TABLE IF NOT EXISTS seq_counter (name TEXT PRIMARY KEY, value INTEGER);",
        )?;
        Ok(Self { conn: Mutex::new(conn) })
    }
    fn next_seq(conn: &Connection) -> rusqlite::Result<i64> {
        conn.execute(
            "INSERT INTO seq_counter(name, value) VALUES ('global', 1)
             ON CONFLICT(name) DO UPDATE SET value = value + 1",
            [],
        )?;
        conn.query_row(
            "SELECT value FROM seq_counter WHERE name = 'global'",
            [],
            |r| r.get(0),
        )
    }
    /// Creates/updates a file entry, returns the new sequence number.
    pub fn upsert(
        &self,
        path: &str,
        content_hash: &str,
        size: i64,
        mtime: f64,
    ) -> Result<i64> {
        let conn = self.conn.lock().unwrap();
        let seq = Self::next_seq(&conn)?;
        conn.execute(
            "INSERT INTO file_index(path, hash, size, mtime, seq, deleted)
             VALUES (?1, ?2, ?3, ?4, ?5, 0)
             ON CONFLICT(path) DO UPDATE SET hash=?2, size=?3, mtime=?4, seq=?5, deleted=0",
            params![path, content_hash, size, mtime, seq],
        )?;
        Ok(seq)
    }
    pub fn mark_deleted(&self, path: &str) -> Result<i64> {
        let conn = self.conn.lock().unwrap();
        let seq = Self::next_seq(&conn)?;
        conn.execute(
            "INSERT INTO file_index(path, hash, size, mtime, seq, deleted)
             VALUES (?1, NULL, NULL, NULL, ?2, 1)
             ON CONFLICT(path) DO UPDATE SET deleted=1, seq=?2",
            params![path, seq],
        )?;
        Ok(seq)
    }
    pub fn get(&self, path: &str) -> Result<Option<FileRecord>> {
        let conn = self.conn.lock().unwrap();
        let rec = conn
            .query_row(
                "SELECT path, hash, size, mtime, seq, deleted FROM file_index WHERE path = ?1",
                params![path],
                row_to_record,
            )
            .optional()?;
        Ok(rec)
    }
    /// Returns all changes since `seq` - basis for the per-layer cursor.
    pub fn changes_since(&self, seq: i64) -> Result<Vec<FileRecord>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn
            .prepare(
                "SELECT path, hash, size, mtime, seq, deleted FROM file_index
             WHERE seq > ?1 ORDER BY seq ASC",
            )?;
        let rows = stmt.query_map(params![seq], row_to_record)?;
        Ok(rows.collect::<rusqlite::Result<Vec<_>>>()?)
    }
}
/// Abstracted persistence interface for a single layer.
///
/// Unspecific key/value cache (JSON-serialized) per layer, plus its own
/// file-system namespace for sidecars (vectors, index files, ...). Layers
/// decide for themselves what structure their ids have (chunk id, line
/// range, file path, composite key) and whether/how they share resources
/// with other layers (e.g. via the same key scheme).
///
/// Lazily backed: `open` only records the paths, it does not touch the
/// filesystem. The sidecar directory and the `cache.db` SQLite file are
/// created on first actual access (`put`/`get`/`delete`/`keys`/
/// `get_cursor`/`set_cursor`/`path_for`) - a layer that is registered but
/// never uses its storage for a given run leaves no trace on disk.
pub struct LayerStorage {
    pub layer_id: String,
    pub dir: PathBuf,
    db_path: PathBuf,
    conn: Mutex<Option<Connection>>,
}
impl LayerStorage {
    /// Records the paths only; performs no filesystem access.
    pub fn open(layer_id: &str, db_path: &Path, dir_path: &Path) -> Result<Self> {
        Ok(Self {
            layer_id: layer_id.to_string(),
            dir: dir_path.to_path_buf(),
            db_path: db_path.to_path_buf(),
            conn: Mutex::new(None),
        })
    }
    /// Returns the open connection, creating the sidecar directory and
    /// opening/initializing `cache.db` on the very first call.
    fn with_conn<T>(&self, f: impl FnOnce(&Connection) -> Result<T>) -> Result<T> {
        let mut guard = self.conn.lock().unwrap();
        if guard.is_none() {
            std::fs::create_dir_all(&self.dir)?;
            let conn = Connection::open(&self.db_path)?;
            conn.execute_batch(
                "PRAGMA journal_mode=WAL;
                 CREATE TABLE IF NOT EXISTS cache (key TEXT PRIMARY KEY, value TEXT);
                 CREATE TABLE IF NOT EXISTS layer_meta (name TEXT PRIMARY KEY, value TEXT);",
            )?;
            *guard = Some(conn);
        }
        f(guard.as_ref().unwrap())
    }
    pub fn put(&self, key: &str, value: &Value) -> Result<()> {
        let json = serde_json::to_string(value)?;
        self.with_conn(|conn| {
            conn.execute(
                "INSERT INTO cache(key, value) VALUES (?1, ?2)
                 ON CONFLICT(key) DO UPDATE SET value=?2",
                params![key, json],
            )?;
            Ok(())
        })
    }
    pub fn get(&self, key: &str) -> Result<Option<Value>> {
        self.with_conn(|conn| {
            let raw: Option<String> = conn
                .query_row(
                    "SELECT value FROM cache WHERE key = ?1",
                    params![key],
                    |r| { r.get(0) },
                )
                .optional()?;
            Ok(
                match raw {
                    Some(s) => Some(serde_json::from_str(&s)?),
                    None => None,
                },
            )
        })
    }
    pub fn delete(&self, key: &str) -> Result<()> {
        self.with_conn(|conn| {
            conn.execute("DELETE FROM cache WHERE key = ?1", params![key])?;
            Ok(())
        })
    }
    pub fn keys(&self, prefix: &str) -> Result<Vec<String>> {
        self.with_conn(|conn| {
            let pattern = format!("{}%", prefix);
            let mut stmt = conn
                .prepare("SELECT key FROM cache WHERE key LIKE ?1 ORDER BY key")?;
            let rows = stmt.query_map(params![pattern], |r| r.get(0))?;
            Ok(rows.collect::<rusqlite::Result<Vec<String>>>()?)
        })
    }
    /// Path for layer-owned sidecar files (e.g. vector sidecars). Creates
    /// the sidecar directory on first call.
    pub fn path_for(&self, name: &str) -> Result<PathBuf> {
        std::fs::create_dir_all(&self.dir)?;
        Ok(self.dir.join(name))
    }
    pub fn get_cursor(&self) -> Result<i64> {
        self.with_conn(|conn| {
            let raw: Option<String> = conn
                .query_row(
                    "SELECT value FROM layer_meta WHERE name = 'cursor'",
                    [],
                    |r| r.get(0),
                )
                .optional()?;
            Ok(raw.and_then(|s| s.parse().ok()).unwrap_or(0))
        })
    }
    pub fn set_cursor(&self, seq: i64) -> Result<()> {
        self.with_conn(|conn| {
            conn.execute(
                "INSERT INTO layer_meta(name, value) VALUES ('cursor', ?1)
                 ON CONFLICT(name) DO UPDATE SET value=?1",
                params![seq.to_string()],
            )?;
            Ok(())
        })
    }
}
/// Manages the RAG root directory's `.xyrag` storage and hands out their storage to layers.
pub struct PersistenceManager {
    pub root: PathBuf,
    pub storagedir: PathBuf,
    pub shared_index: Arc<SharedIndex>,
    layer_storages: Mutex<HashMap<String, Arc<LayerStorage>>>,
}
impl PersistenceManager {
    pub fn new(root: Option<&Path>) -> Result<Self> {
        let root = resolve_root(root)?;
        let storagedir = resolve_storagedir(&root)?;
        let shared_index = Arc::new(SharedIndex::open(&storagedir.join("index.db"))?);
        Ok(Self {
            root,
            storagedir,
            shared_index,
            layer_storages: Mutex::new(HashMap::new()),
        })
    }
    pub fn layer_storage(&self, layer_id: &str) -> Result<Arc<LayerStorage>> {
        let mut map = self.layer_storages.lock().unwrap();
        if let Some(s) = map.get(layer_id) {
            return Ok(s.clone());
        }
        let layer_dir = self.storagedir.join("layers").join(layer_id);
        let storage = Arc::new(
            LayerStorage::open(layer_id, &layer_dir.join("cache.db"), &layer_dir)?,
        );
        map.insert(layer_id.to_string(), storage.clone());
        Ok(storage)
    }
}
