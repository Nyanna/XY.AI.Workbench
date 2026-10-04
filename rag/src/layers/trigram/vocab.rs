//! Lazy, persistent trigram vocabulary (spec part A, section 5).
//!
//! Maps trigram strings to random, collision-free `u32` IDs in
//! `[0, 2^key_bits)` so trigrams spread evenly through the key space. IDs are
//! assigned lazily on first sight of a trigram (file side only). The file on
//! disk is the source of truth: once it exists it is loaded, never re-rolled.
//! It also records a creation timestamp used to detect `TrieParams` changes
//! (decision 9.6).
use std::collections::{HashMap, HashSet};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};
use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};
use super::trie::TrieParams;
fn now_ns() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos() as u64)
        .unwrap_or(0)
}
/// SplitMix64 step - a tiny, dependency-free PRNG for ID assignment.
fn split_mix64(state: &mut u64) -> u64 {
    *state = state.wrapping_add(0x9E37_79B9_7F4A_7C15);
    let mut z = *state;
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}
#[derive(Serialize, Deserialize)]
struct VocabFile {
    key_bits: u8,
    quant_bits: u8,
    root_bits: u8,
    created_at: u64,
    rng_state: u64,
    entries: Vec<(String, u32)>,
}
/// In-memory vocabulary; wrap in an `RwLock` for concurrent access.
pub struct Vocab {
    params: TrieParams,
    created_at: u64,
    rng_state: u64,
    map: HashMap<String, u32>,
    used: HashSet<u32>,
    path: PathBuf,
}
impl Vocab {
    /// Loads the vocabulary from `path`, or creates a fresh one if the file is
    /// absent or was built with different parameters (which invalidates it).
    pub fn load_or_new(path: &Path, params: TrieParams) -> Self {
        if let Some(v) = Self::try_load(path, params) {
            return v;
        }
        Self {
            params,
            created_at: now_ns(),
            rng_state: now_ns() ^ 0x243F_6A88_85A3_08D3,
            map: HashMap::new(),
            used: HashSet::new(),
            path: path.to_path_buf(),
        }
    }
    fn try_load(path: &Path, params: TrieParams) -> Option<Self> {
        let bytes = std::fs::read(path).ok()?;
        let vf: VocabFile = serde_json::from_slice(&bytes).ok()?;
        if vf.key_bits != params.key_bits || vf.quant_bits != params.quant_bits
            || vf.root_bits != params.root_bits
        {
            return None;
        }
        let mut map = HashMap::with_capacity(vf.entries.len());
        let mut used = HashSet::with_capacity(vf.entries.len());
        for (tg, id) in vf.entries {
            used.insert(id);
            map.insert(tg, id);
        }
        Some(Self {
            params,
            created_at: vf.created_at,
            rng_state: vf.rng_state,
            map,
            used,
            path: path.to_path_buf(),
        })
    }
    /// Creation timestamp; signatures older than this are stale (decision 9.6).
    pub fn created_at(&self) -> u64 {
        self.created_at
    }
    /// Looks up an existing ID without assigning one (query side).
    pub fn lookup(&self, trigram: &str) -> Option<u32> {
        self.map.get(trigram).copied()
    }
    /// Interns trigrams, assigning IDs to unseen ones. Returns the keys and
    /// whether any new ID was created (callers then persist before writing
    /// index entries that reference the new IDs).
    pub fn intern_all(&mut self, trigrams: &[String]) -> Result<(Vec<u32>, bool)> {
        let mut keys = Vec::with_capacity(trigrams.len());
        let mut changed = false;
        for tg in trigrams {
            if let Some(id) = self.map.get(tg) {
                keys.push(*id);
                continue;
            }
            let id = self.allocate_id()?;
            self.used.insert(id);
            self.map.insert(tg.clone(), id);
            keys.push(id);
            changed = true;
        }
        Ok((keys, changed))
    }
    /// Picks a random free ID via probing, erroring if the space is exhausted.
    fn allocate_id(&mut self) -> Result<u32> {
        let capacity = 1u64 << self.params.key_bits;
        if self.used.len() as u64 >= capacity {
            bail!("trigram ID space exhausted ({} keys)", capacity);
        }
        let start = (split_mix64(&mut self.rng_state) % capacity) as u32;
        let cap = capacity as u32;
        let mut id = start;
        loop {
            if !self.used.contains(&id) {
                return Ok(id);
            }
            id = (id + 1) % cap;
        }
    }
    /// Writes the vocabulary atomically (temp file + rename).
    pub fn persist(&self) -> Result<()> {
        let mut entries: Vec<(String, u32)> = self
            .map
            .iter()
            .map(|(k, v)| (k.clone(), *v))
            .collect();
        entries.sort_by_key(|a| a.1);
        let vf = VocabFile {
            key_bits: self.params.key_bits,
            quant_bits: self.params.quant_bits,
            root_bits: self.params.root_bits,
            created_at: self.created_at,
            rng_state: self.rng_state,
            entries,
        };
        let bytes = serde_json::to_vec(&vf)?;
        if let Some(parent) = self.path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let tmp = self.path.with_extension("tmp");
        {
            let mut f = std::fs::File::create(&tmp)
                .with_context(|| format!("creating {}", tmp.display()))?;
            f.write_all(&bytes)?;
            f.sync_all()?;
        }
        std::fs::rename(&tmp, &self.path)?;
        Ok(())
    }
}
