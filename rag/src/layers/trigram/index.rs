//! Persistent per-file trigram index (spec part A, section 7).
//!
//! One entry per file stores its mtime and signature (a `CompactTrie`
//! payload under shared `TrieParams`). Entries live in the layer's key/value
//! storage under an `idx:` prefix; each is expanded to a `CellMask` in memory
//! for matching. Staleness is driven by mtime and by the vocabulary creation
//! timestamp (decision 9.6).

use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::core::persistence::LayerStorage;

use super::trie::{CellMask, CompactTrie, TrieParams};

const KEY_PREFIX: &str = "idx:";

/// Modification time of a file as nanoseconds since the epoch.
pub fn mtime_ns(meta: &std::fs::Metadata) -> u64 {
    meta.modified()
        .ok()
        .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
        .map(|d| d.as_nanos() as u64)
        .unwrap_or(0)
}

/// In-memory index entry: expanded mask plus the freshness timestamps.
pub struct IndexEntry {
    pub mtime_ns: u64,
    pub indexed_at: u64,
    pub mask: CellMask,
}

#[derive(Serialize, Deserialize)]
struct EntryFile {
    mtime: u64,
    indexed_at: u64,
    bits: u32,
    payload: Vec<u8>,
}

fn key_for(rel: &str) -> String {
    format!("{KEY_PREFIX}{rel}")
}

/// Loads every stored entry, expanding each signature to a `CellMask`.
/// Entries that fail to parse are skipped (they will be re-indexed on visit).
pub fn load_all(
    storage: &LayerStorage,
    params: TrieParams,
) -> Result<HashMap<String, IndexEntry>> {
    let mut out = HashMap::new();
    for key in storage.keys(KEY_PREFIX)? {
        let Some(value) = storage.get(&key)? else { continue };
        let Ok(ef) = serde_json::from_value::<EntryFile>(value) else { continue };
        let Ok(trie) = CompactTrie::from_payload(params, ef.payload, ef.bits) else {
            continue;
        };
        let rel = key[KEY_PREFIX.len()..].to_string();
        out.insert(
            rel,
            IndexEntry { mtime_ns: ef.mtime, indexed_at: ef.indexed_at, mask: trie.expand() },
        );
    }
    Ok(out)
}

/// Persists one entry's signature payload.
pub fn store_entry(
    storage: &LayerStorage,
    rel: &str,
    mtime_ns: u64,
    indexed_at: u64,
    trie: &CompactTrie,
) -> Result<()> {
    let (payload, bits) = trie.to_payload();
    let value = json!({
        "mtime": mtime_ns,
        "indexed_at": indexed_at,
        "bits": bits,
        "payload": payload,
    });
    storage.put(&key_for(rel), &value)
}

/// Removes one entry (file deleted from disk).
pub fn delete_entry(storage: &LayerStorage, rel: &str) -> Result<()> {
    storage.delete(&key_for(rel))
}

/// Nanoseconds since the epoch, for `indexed_at` stamps.
pub fn now_ns() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_nanos() as u64).unwrap_or(0)
}
