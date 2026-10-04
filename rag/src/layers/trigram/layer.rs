//! Trigram-gated text search layer (spec part A, sections 2-3).
//!
//! Each query runs a recursive walk below the search root (via
//! [`DirCache`]). Per file, a trigram signature is kept current (re-indexed
//! on mtime/param change, dropped when the file disappears) and a mirrored,
//! normalised copy is written for line-faithful matching. The query's
//! signature gates which files are actually searched (`matches_at_least`);
//! the normalised query is then substring-matched line by line in the mirror,
//! and hit line numbers map straight onto the original file. File searches
//! run as executor tasks under a wall-clock budget (sections 3).
use std::collections::BTreeMap;
use std::collections::VecDeque;
use std::path::{Path, PathBuf};
use std::sync::{Arc, OnceLock, RwLock};
use std::time::{Duration, Instant};
use async_trait::async_trait;
use futures::stream::FuturesUnordered;
use futures::StreamExt;
use serde_json::{json, Map, Value};
use crate::core::layer::{Layer, LayerContext, LayerStage, LayerStatus};
use crate::core::persistence::LayerStorage;
use crate::core::query::Query;
use crate::core::result::{ResultEntry, ResultSet};
use crate::layers::dir_cache::DirCache;
use super::index::{self, IndexEntry};
use super::signature;
use super::trie::{matches_at_least, CellMask, CompactTrie, QueryMask, TrieParams};
use super::vocab::Vocab;
use super::{
    normalize, signature::DEFAULT_PRUNE_MIN_COUNT, signature::DEFAULT_PRUNE_RATIO,
};
const DIR_CACHE_CAPACITY: usize = 4096;
const MAX_FILE_SIZE: u64 = 5 * 1024 * 1024;
const MAX_RUNTIME: Duration = Duration::from_secs(60);
const START_CUTOFF_RATIO: f64 = 0.9;
/// Default fraction of query trigrams a file must contain to be searched.
const DEFAULT_MIN_MATCH_RATIO: f64 = 0.8;
/// Tunable thresholds; defaults follow the spec.
struct Config {
    min_match_ratio: f64,
    prune_ratio: f64,
    prune_min_count: usize,
}
impl Default for Config {
    fn default() -> Self {
        Self {
            min_match_ratio: DEFAULT_MIN_MATCH_RATIO,
            prune_ratio: DEFAULT_PRUNE_RATIO,
            prune_min_count: DEFAULT_PRUNE_MIN_COUNT,
        }
    }
}
/// State shared across queries; built once from the layer's storage.
struct SharedState {
    vocab: RwLock<Vocab>,
    index: RwLock<std::collections::HashMap<String, IndexEntry>>,
    mirror_dir: PathBuf,
    storage: Arc<LayerStorage>,
}
/// A file that passed the signature gate and should be searched.
struct Candidate {
    rel_path: String,
    abs_path: PathBuf,
    mirror_path: PathBuf,
}
fn join_rel(prefix: &str, name: &str) -> String {
    if prefix.is_empty() { name.to_string() } else { format!("{prefix}/{name}") }
}
/// All regular files below `root`, traversed iteratively via `cache`.
fn collect_files(
    cache: &DirCache,
    root: &Path,
    rel_prefix: &str,
    out: &mut Vec<(String, PathBuf)>,
) {
    let mut pending: Vec<(PathBuf, String)> = vec![
        (root.to_path_buf(), rel_prefix.to_string())
    ];
    while let Some((dir, prefix)) = pending.pop() {
        let Ok(listing) = cache.list(&dir) else { continue };
        for name in &listing.files {
            out.push((join_rel(&prefix, name), dir.join(name)));
        }
        for name in &listing.dirs {
            pending.push((dir.join(name), join_rel(&prefix, name)));
        }
    }
}
/// Reads the mirror and original, returning `(line_no, original_text)` for
/// every normalised line that contains `needle` (one entry per line).
fn search_file(mirror: &Path, abs: &Path, needle: &str) -> Vec<(usize, String)> {
    let norm = std::fs::read_to_string(mirror).unwrap_or_default();
    if norm.is_empty() {
        return Vec::new();
    }
    let orig = std::fs::read_to_string(abs).unwrap_or_default();
    let orig_lines: Vec<&str> = orig.split('\n').collect();
    let mut out = Vec::new();
    for (i, nline) in norm.split('\n').enumerate() {
        if !nline.is_empty() && nline.contains(needle) {
            let text = orig_lines.get(i).copied().unwrap_or("").to_string();
            out.push((i + 1, text));
        }
    }
    out
}
/// Trigram-gated text search layer.
pub struct TrigramLayer {
    cache: DirCache,
    params: TrieParams,
    cfg: Config,
    state: OnceLock<SharedState>,
}
impl TrigramLayer {
    pub fn new() -> Self {
        let params = TrieParams::new(16, 0, 4).expect("valid default trie params");
        Self {
            cache: DirCache::new(DIR_CACHE_CAPACITY),
            params,
            cfg: Config::default(),
            state: OnceLock::new(),
        }
    }
    /// Lazily builds the shared state from this run's storage (same storage
    /// across runs). Best-effort: a failed load yields an empty index.
    fn state(&self, ctx: &LayerContext) -> &SharedState {
        self.state
            .get_or_init(|| {
                let storage = ctx.storage.clone();
                let vocab_path = storage
                    .path_for("vocab.json")
                    .unwrap_or_else(|_| PathBuf::from("vocab.json"));
                let mirror_dir = storage
                    .path_for("mirror")
                    .unwrap_or_else(|_| PathBuf::from("mirror"));
                let vocab = Vocab::load_or_new(&vocab_path, self.params);
                let index = index::load_all(&storage, self.params).unwrap_or_default();
                SharedState {
                    vocab: RwLock::new(vocab),
                    index: RwLock::new(index),
                    mirror_dir,
                    storage,
                }
            })
    }
    /// (Re-)indexes one file: normalise, mirror, build + persist signature.
    /// Returns the file's expanded mask, or `None` on an I/O error.
    fn reindex(
        &self,
        state: &SharedState,
        rel: &str,
        abs: &Path,
        mtime: u64,
    ) -> Option<CellMask> {
        let bytes = std::fs::read(abs).ok()?;
        let norm = normalize::normalize_bytes(&bytes).unwrap_or_default();
        let mirror = state.mirror_dir.join(rel);
        if let Some(parent) = mirror.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        let _ = std::fs::write(&mirror, &norm);
        let trigrams = signature::file_trigrams(
            &norm,
            self.cfg.prune_ratio,
            self.cfg.prune_min_count,
        );
        let keys = {
            let mut v = state.vocab.write().unwrap();
            let (keys, changed) = v.intern_all(&trigrams).ok()?;
            if changed {
                let _ = v.persist();
            }
            keys
        };
        let trie = CompactTrie::build(self.params, keys).ok()?;
        let mask = trie.expand();
        let indexed_at = index::now_ns();
        let _ = index::store_entry(&state.storage, rel, mtime, indexed_at, &trie);
        state
            .index
            .write()
            .unwrap()
            .insert(
                rel.to_string(),
                IndexEntry {
                    mtime_ns: mtime,
                    indexed_at,
                    mask: mask.clone(),
                },
            );
        Some(mask)
    }
    /// Removes index entries whose files no longer exist on disk.
    fn prune_deleted(&self, state: &SharedState, document_root: &Path, prefix: &str) {
        let rels: Vec<String> = {
            let idx = state.index.read().unwrap();
            idx.keys()
                .filter(|r| prefix.is_empty() || r.starts_with(prefix))
                .cloned()
                .collect()
        };
        for rel in rels {
            if !document_root.join(&rel).exists() {
                let _ = index::delete_entry(&state.storage, &rel);
                state.index.write().unwrap().remove(&rel);
            }
        }
    }
}
impl Default for TrigramLayer {
    fn default() -> Self {
        Self::new()
    }
}
#[async_trait]
impl Layer for TrigramLayer {
    fn id(&self) -> &str {
        "trigram"
    }
    fn stage(&self) -> LayerStage {
        LayerStage::Generate
    }
    fn applies(&self, query: &Query) -> bool {
        query.has("query")
    }
    async fn run(
        &self,
        query: &Query,
        result_set: &ResultSet,
        ctx: &LayerContext,
    ) -> LayerStatus {
        let mut status = LayerStatus::new(self.id(), self.stage());
        let Some(text) = query.get_str("query") else {
            status.skipped = true;
            return status;
        };
        let query_norm = normalize::normalize(text);
        let needle = query_norm.replace('\n', " ");
        let needle = needle.trim();
        if needle.is_empty() {
            status.skipped = true;
            return status;
        }
        let state = self.state(ctx);
        let vocab_created = state.vocab.read().unwrap().created_at();
        let (search_root, rel_prefix) = query.resolve_search_root();
        let document_root = query.document_root().to_path_buf();
        let mut files = Vec::new();
        collect_files(&self.cache, &search_root, &rel_prefix, &mut files);
        let mut visited: Vec<(String, PathBuf)> = Vec::new();
        for (rel, abs) in files {
            let Ok(meta) = std::fs::metadata(&abs) else { continue };
            if meta.len() > MAX_FILE_SIZE {
                continue;
            }
            let mtime = index::mtime_ns(&meta);
            let stale = {
                let idx = state.index.read().unwrap();
                match idx.get(&rel) {
                    None => true,
                    Some(e) => e.mtime_ns != mtime || e.indexed_at < vocab_created,
                }
            };
            if stale && self.reindex(state, &rel, &abs, mtime).is_none() {
                continue;
            }
            visited.push((rel, abs));
        }
        self.prune_deleted(state, &document_root, &rel_prefix);
        let keys: Vec<u32> = {
            let v = state.vocab.read().unwrap();
            signature::query_trigrams(&query_norm)
                .iter()
                .filter_map(|tg| v.lookup(tg))
                .collect()
        };
        let Ok(qmask) = QueryMask::new(self.params, keys) else {
            status.skipped = true;
            return status;
        };
        let t = (self.cfg.min_match_ratio * qmask.n_keys as f64).ceil() as u32;
        let t_cells = qmask.corrected_threshold(t.max(1));
        let mut candidates: Vec<Candidate> = Vec::new();
        for (rel, abs) in visited {
            let passes = state
                .index
                .read()
                .unwrap()
                .get(&rel)
                .map(|e| matches_at_least(&e.mask, &qmask, t_cells))
                .unwrap_or(false);
            if passes {
                candidates
                    .push(Candidate {
                        mirror_path: state.mirror_dir.join(&rel),
                        abs_path: abs,
                        rel_path: rel,
                    });
            }
        }
        let needle = Arc::new(needle.to_string());
        let mut queue: VecDeque<Candidate> = candidates.into();
        let start = Instant::now();
        let deadline = start + MAX_RUNTIME;
        let start_cutoff = start
            + Duration::from_secs_f64(MAX_RUNTIME.as_secs_f64() * START_CUTOFF_RATIO);
        let mut pending = FuturesUnordered::new();
        let mut results: Vec<(String, Vec<(usize, String)>)> = Vec::new();
        let mut aborted = false;
        loop {
            while Instant::now() < start_cutoff {
                let Some(c) = queue.pop_front() else { break };
                let cpu = ctx.cpu.clone();
                let needle = Arc::clone(&needle);
                let rel = c.rel_path;
                let abs = c.abs_path;
                let mirror = c.mirror_path;
                pending
                    .push(async move {
                        let hits = cpu
                            .spawn(move || search_file(&mirror, &abs, &needle))
                            .await
                            .unwrap_or_default();
                        (rel, hits)
                    });
            }
            if pending.is_empty() {
                break;
            }
            let now = Instant::now();
            if now >= deadline {
                aborted = true;
                break;
            }
            tokio::select! {
                maybe = pending.next() => { if let Some(res) = maybe { results.push(res);
                } } _ = tokio::time::sleep(deadline - now) => { aborted = true; break; }
            }
        }
        results.sort_by(|a, b| a.0.cmp(&b.0));
        for (rel, hits) in results {
            if hits.is_empty() {
                continue;
            }
            let mut lines: BTreeMap<usize, String> = BTreeMap::new();
            for (ln, text) in hits {
                lines.entry(ln).or_insert(text);
            }
            let ordered: Vec<(usize, String)> = lines.into_iter().collect();
            let mut fields = Map::new();
            fields.insert("File".into(), Value::String(rel));
            fields.insert("Lines".into(), json!(ordered));
            result_set.add(ResultEntry::new(None, fields));
            status.contributions += 1;
        }
        status.ran = true;
        status.aborted = aborted;
        status.detail.insert("root".into(), json!(search_root.display().to_string()));
        status
    }
}
