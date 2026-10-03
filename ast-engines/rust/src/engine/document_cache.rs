//! Timestamp/content-hash validated cache of parsed `syn::File`s, keyed by absolute
//! path -- mirrors Java's `DocumentCache`: a cheap mtime+size check first, a SHA-256
//! content check only on mismatch, and a real re-parse only if the file's content
//! actually changed outside this process. Every mutating operation writes the
//! pretty-printed source back to disk and refreshes the cache from that exact text,
//! so reported line numbers always match what's on disk.
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use sha2::{Digest, Sha256};
use crate::engine::ast_engine_exception::{AstEngineException, AstResult, Kind};
use crate::engine::rust_ast_engine;
/// `file` plus the exact text it was parsed from.
pub struct Entry {
    pub file: syn::File,
    pub source: String,
}
struct CacheEntry {
    mtime_millis: u128,
    size: u64,
    content_hash: String,
    file: syn::File,
    source: String,
}
pub struct DocumentCache {
    entries: Mutex<HashMap<String, CacheEntry>>,
}
impl DocumentCache {
    pub fn new() -> Self {
        Self {
            entries: Mutex::new(HashMap::new()),
        }
    }
    pub fn get(&self, path: &Path) -> AstResult<Entry> {
        let key = path.to_string_lossy().into_owned();
        let meta = std::fs::metadata(path)
            .map_err(|e| AstEngineException::new(
                Kind::Syntax,
                format!("cannot read {}: {}", key, e),
            ))?;
        let mtime_millis = meta
            .modified()
            .ok()
            .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
            .map(|d| d.as_millis())
            .unwrap_or(0);
        let size = meta.len();
        let mut entries = self.entries.lock().unwrap();
        if let Some(entry) = entries.get(&key) {
            if entry.mtime_millis == mtime_millis && entry.size == size {
                return Ok(Entry {
                    file: entry.file.clone(),
                    source: entry.source.clone(),
                });
            }
        }
        let source = std::fs::read_to_string(path)
            .map_err(|e| AstEngineException::new(
                Kind::Syntax,
                format!("cannot read {}: {}", key, e),
            ))?;
        let digest = sha256(&source);
        if let Some(entry) = entries.get(&key) {
            if entry.content_hash == digest {
                let file = entry.file.clone();
                let src = entry.source.clone();
                entries
                    .insert(
                        key,
                        CacheEntry {
                            mtime_millis,
                            size,
                            content_hash: digest,
                            file: file.clone(),
                            source: src.clone(),
                        },
                    );
                return Ok(Entry { file, source: src });
            }
        }
        let file = rust_ast_engine::parse_file(&source)?;
        entries
            .insert(
                key,
                CacheEntry {
                    mtime_millis,
                    size,
                    content_hash: digest,
                    file: file.clone(),
                    source: source.clone(),
                },
            );
        Ok(Entry { file, source })
    }
    /// Serialises `file`, writes it to `path` and refreshes the cache entry from that text.
    pub fn save(&self, path: &Path, file: syn::File) -> AstResult<String> {
        let source = rust_ast_engine::print_file(&file);
        std::fs::write(path, &source)
            .map_err(|e| AstEngineException::new(
                Kind::Syntax,
                format!("cannot write {}: {}", path.display(), e),
            ))?;
        let normalized = rust_ast_engine::parse_file(&source)?;
        let meta = std::fs::metadata(path)
            .map_err(|e| AstEngineException::new(
                Kind::Syntax,
                format!("cannot read {}: {}", path.display(), e),
            ))?;
        let mtime_millis = meta
            .modified()
            .ok()
            .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
            .map(|d| d.as_millis())
            .unwrap_or(0);
        let digest = sha256(&source);
        let key = path.to_string_lossy().into_owned();
        self.entries
            .lock()
            .unwrap()
            .insert(
                key,
                CacheEntry {
                    mtime_millis,
                    size: meta.len(),
                    content_hash: digest,
                    file: normalized,
                    source: source.clone(),
                },
            );
        Ok(source)
    }
    pub fn invalidate(&self, path: &Path) {
        self.entries.lock().unwrap().remove(&path.to_string_lossy().into_owned());
    }
}
fn sha256(text: &str) -> String {
    let digest = Sha256::digest(text.as_bytes());
    digest.iter().map(|b| format!("{:02x}", b)).collect()
}
#[allow(dead_code)]
fn _unused(_: PathBuf) {}
