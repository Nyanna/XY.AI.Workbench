//! Glob-pattern based directory/file search layer.
//!
//! Activates on queries carrying a `query` field (a free-text search
//! string). The string is split on whitespace into tokens; each token is
//! treated as a glob pattern and matched recursively against the
//! directory tree rooted at the query's `directory` field, or the
//! document root as fallback (see [`Query::directory`]). Directory
//! listings are served from a small LRU, mtime-validated in-memory cache
//! so repeated sub-globs/queries don't re-read unchanged directories.
use std::path::{Path, PathBuf};
use async_trait::async_trait;
use glob::Pattern;
use serde_json::{json, Map, Value};
use crate::core::layer::{Layer, LayerContext, LayerStage, LayerStatus};
use crate::core::query::Query;
use crate::core::result::{ResultEntry, ResultSet};
use crate::layers::dir_cache::DirCache;
/// Max number of directory listings kept in memory at once.
const DIR_CACHE_CAPACITY: usize = 4096;
/// Max number of matches written back per query.
const MAX_MATCHES: usize = 50;
/// One candidate found below the search root, prior to glob matching.
#[derive(Clone)]
struct Candidate {
    /// Path relative to the search root, `/`-separated.
    rel_path: String,
    is_dir: bool,
}
fn join_rel(prefix: &str, name: &str) -> String {
    if prefix.is_empty() { name.to_string() } else { format!("{prefix}/{name}") }
}
/// Collects every file/directory below `root`, via `cache`.
///
/// Traverses the tree iteratively with an explicit stack of pending
/// directories, issuing one single-level `cache.list` call per
/// directory - the recursive descent lives entirely here, not in the
/// cache.
fn walk(cache: &DirCache, root: &Path, rel_prefix: &str, out: &mut Vec<Candidate>) {
    let mut pending: Vec<(PathBuf, String)> = vec![
        (root.to_path_buf(), rel_prefix.to_string())
    ];
    while let Some((dir, prefix)) = pending.pop() {
        let listing = match cache.list(&dir) {
            Ok(l) => l,
            Err(_) => continue,
        };
        for name in &listing.files {
            out.push(Candidate {
                rel_path: join_rel(&prefix, name),
                is_dir: false,
            });
        }
        for name in &listing.dirs {
            let rel = join_rel(&prefix, name);
            out.push(Candidate {
                rel_path: rel.clone(),
                is_dir: true,
            });
            pending.push((dir.join(name), rel));
        }
    }
}
/// Number of glob meta characters in a pattern - a rough specificity
/// signal for sorting, not used for matching itself.
fn wildcard_count(pattern: &str) -> usize {
    pattern.chars().filter(|c| matches!(c, '*' | '?' | '[' | ']')).count()
}
/// Resolves the actual filesystem directory to walk, plus the relative
/// prefix (document-root-relative, `/`-separated) every discovered path
/// must be prepended with, so `File`/`Directory` always carry the full
/// path relative to the document root - not just relative to the
/// (optional) search `directory`.
fn resolve_search_root(query: &Query) -> (PathBuf, String) {
    let document_root = query.document_root().to_path_buf();
    match query.get_str("directory") {
        None => (document_root, String::new()),
        Some(dir) => {
            let dir_path = PathBuf::from(dir);
            let search_root = if dir_path.is_absolute() {
                dir_path.clone()
            } else {
                document_root.join(&dir_path)
            };
            let prefix = match search_root.strip_prefix(&document_root) {
                Ok(rel) => rel.to_string_lossy().replace(std::path::MAIN_SEPARATOR, "/"),
                Err(_) => dir.trim_matches('/').to_string(),
            };
            (search_root, prefix)
        }
    }
}
/// Glob-pattern based file/directory search layer.
///
/// See the module documentation for the matching, ranking and caching
/// behavior.
pub struct GlobLayer {
    cache: DirCache,
}
impl GlobLayer {
    pub fn new() -> Self {
        Self {
            cache: DirCache::new(DIR_CACHE_CAPACITY),
        }
    }
}
impl Default for GlobLayer {
    fn default() -> Self {
        Self::new()
    }
}
#[async_trait]
impl Layer for GlobLayer {
    fn id(&self) -> &str {
        "glob"
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
        _ctx: &LayerContext,
    ) -> LayerStatus {
        let mut status = LayerStatus::new(self.id(), self.stage());
        let text = match query.get_str("query") {
            Some(t) => t,
            None => {
                status.skipped = true;
                return status;
            }
        };
        let tokens: Vec<&str> = text.split_whitespace().collect();
        if tokens.is_empty() {
            status.skipped = true;
            return status;
        }
        let (search_root, rel_prefix) = resolve_search_root(query);
        let mut candidates = Vec::new();
        walk(&self.cache, &search_root, &rel_prefix, &mut candidates);
        let mut hits: Vec<(usize, usize, Candidate)> = Vec::new();
        for token in &tokens {
            let pattern = match Pattern::new(token) {
                Ok(p) => p,
                Err(_) => continue,
            };
            let wc = wildcard_count(token);
            for cand in &candidates {
                if pattern.matches(&cand.rel_path) {
                    let rank = if cand.rel_path == *token { 0 } else { 1 };
                    hits.push((rank, wc, cand.clone()));
                }
            }
        }
        hits.sort_by(|a, b| {
            a.0.cmp(&b.0).then(a.1.cmp(&b.1)).then(a.2.rel_path.cmp(&b.2.rel_path))
        });
        let mut seen = std::collections::HashSet::new();
        hits.retain(|(_, _, cand)| seen.insert(cand.rel_path.clone()));
        hits.truncate(MAX_MATCHES);
        for (_, _, cand) in &hits {
            let mut fields = Map::new();
            if cand.is_dir {
                fields.insert("Directory".into(), Value::String(cand.rel_path.clone()));
            } else {
                fields.insert("File".into(), Value::String(cand.rel_path.clone()));
            }
            result_set.add(ResultEntry::new(None, fields));
            status.contributions += 1;
        }
        status.ran = true;
        status.detail.insert("root".into(), json!(search_root.display().to_string()));
        status
    }
}
