//! Grep-style, regex based file-content search layer.
//!
//! Activates on queries carrying a `query` field (a free-text search
//! string). The string is split on whitespace into tokens; each token is
//! compiled as a regular expression and matched line-by-line against
//! every file below the directory tree rooted at the query's `directory`
//! field, or the document root as fallback (see
//! [`crate::core::query::Query::resolve_search_root`]). Directory
//! listings are served from a small LRU, mtime-validated in-memory cache
//! so repeated searches don't re-read unchanged directories.
//!
//! File content searches run as individual tasks on the shared CPU
//! executor, one per file, subject to an overall time budget: no new
//! search is started once 90% of the budget has elapsed, and any still
//! running once the full budget elapses are abandoned.
use std::collections::{HashMap, VecDeque};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::{Duration, Instant};
use async_trait::async_trait;
use futures::stream::FuturesUnordered;
use futures::StreamExt;
use regex::Regex;
use serde_json::{json, Map, Value};
use crate::core::layer::{Layer, LayerContext, LayerStage, LayerStatus};
use crate::core::query::Query;
use crate::core::result::{ResultEntry, ResultSet};
use crate::layers::dir_cache::DirCache;
/// Max number of directory listings kept in memory at once.
const DIR_CACHE_CAPACITY: usize = 4096;
/// Max number of line matches written back per query.
const MAX_MATCHES: usize = 50;
/// Files larger than this are not searched.
const MAX_FILE_SIZE: u64 = 5 * 1024 * 1024;
/// Overall wall-clock budget for one query's file searches.
const MAX_RUNTIME: Duration = Duration::from_secs(60);
/// Fraction of `MAX_RUNTIME` after which no new file search is started.
const START_CUTOFF_RATIO: f64 = 0.9;
/// Characters of leading/trailing context kept around a match in an
/// excerpt.
const CONTEXT_CHARS: usize = 20;
/// One file discovered below the search root, prior to content search.
struct FileCandidate {
    /// Path relative to the document root, `/`-separated.
    rel_path: String,
    abs_path: PathBuf,
}
fn join_rel(prefix: &str, name: &str) -> String {
    if prefix.is_empty() { name.to_string() } else { format!("{prefix}/{name}") }
}
/// Collects every regular file below `root`, via `cache`.
///
/// Traverses the tree iteratively with an explicit stack of pending
/// directories, issuing one single-level `cache.list` call per
/// directory.
fn collect_files(
    cache: &DirCache,
    root: &Path,
    rel_prefix: &str,
    out: &mut Vec<FileCandidate>,
) {
    let mut pending: Vec<(PathBuf, String)> = vec![
        (root.to_path_buf(), rel_prefix.to_string())
    ];
    while let Some((dir, prefix)) = pending.pop() {
        let listing = match cache.list(&dir) {
            Ok(l) => l,
            Err(_) => continue,
        };
        for f in &listing.files {
            out.push(FileCandidate {
                rel_path: join_rel(&prefix, &f.name),
                abs_path: dir.join(&f.name),
            });
        }
        for name in &listing.dirs {
            let rel = join_rel(&prefix, name);
            pending.push((dir.join(name), rel));
        }
    }
}
/// A compiled search token together with a rough specificity score.
#[derive(Clone)]
struct CompiledToken {
    regex: Regex,
    /// Number of regex meta characters in the source pattern - a rough
    /// specificity signal for sorting (fewer meta characters = more
    /// literal/specific), not used for matching itself.
    specificity: usize,
}
fn regex_specificity(pattern: &str) -> usize {
    pattern
        .chars()
        .filter(|c| {
            matches!(
                c, '.' | '*' | '+' | '?' | '[' | ']' | '{' | '}' | '(' | ')' | '|' | '^'
                | '$' | '\\'
            )
        })
        .count()
}
/// One matching line found in one file.
#[derive(Clone)]
struct LineMatch {
    rel_path: String,
    line_no: usize,
    text: String,
    /// 0 if some token matched the entire line, 1 otherwise - exact
    /// matches sort first.
    rank: usize,
    specificity: usize,
}
/// Builds a combined excerpt for a line given all of its match spans
/// (byte offsets into `line`), keeping `CONTEXT_CHARS` characters of
/// context around each and merging overlapping windows.
fn build_excerpt(line: &str, spans: &[(usize, usize)]) -> String {
    let chars: Vec<char> = line.chars().collect();
    let char_offset = |byte_idx: usize| line[..byte_idx].chars().count();
    let mut windows: Vec<(usize, usize)> = spans
        .iter()
        .map(|&(s, e)| {
            let cs = char_offset(s);
            let ce = char_offset(e);
            (cs.saturating_sub(CONTEXT_CHARS), (ce + CONTEXT_CHARS).min(chars.len()))
        })
        .collect();
    windows.sort_by_key(|w| w.0);
    let mut merged: Vec<(usize, usize)> = Vec::new();
    for w in windows {
        if let Some(last) = merged.last_mut() {
            if w.0 <= last.1 {
                last.1 = last.1.max(w.1);
                continue;
            }
        }
        merged.push(w);
    }
    merged
        .iter()
        .map(|&(s, e)| {
            let mut seg: String = chars[s..e].iter().collect();
            if s > 0 {
                seg = format!("...{seg}");
            }
            if e < chars.len() {
                seg = format!("{seg}...");
            }
            seg
        })
        .collect::<Vec<_>>()
        .join(" ")
}
/// Searches one file for all `tokens`, line by line; CPU-bound, runs on
/// the shared rayon pool.
fn search_file(path: &Path, rel_path: &str, tokens: &[CompiledToken]) -> Vec<LineMatch> {
    let meta = match fs::metadata(path) {
        Ok(m) => m,
        Err(_) => return Vec::new(),
    };
    if !meta.is_file() || meta.len() > MAX_FILE_SIZE {
        return Vec::new();
    }
    let content = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(_) => return Vec::new(),
    };
    let mut hits = Vec::new();
    for (idx, line) in content.lines().enumerate() {
        let mut spans: Vec<(usize, usize)> = Vec::new();
        let mut full_match = false;
        let mut rank = 1usize;
        let mut specificity = usize::MAX;
        for tok in tokens {
            for m in tok.regex.find_iter(line) {
                spans.push((m.start(), m.end()));
                if m.start() == 0 && m.end() == line.len() {
                    full_match = true;
                    rank = 0;
                }
                specificity = specificity.min(tok.specificity);
            }
        }
        if spans.is_empty() {
            continue;
        }
        spans.sort_by_key(|s| s.0);
        let text = if full_match {
            line.to_string()
        } else {
            build_excerpt(line, &spans)
        };
        hits.push(LineMatch {
            rel_path: rel_path.to_string(),
            line_no: idx + 1,
            text,
            rank,
            specificity,
        });
    }
    hits
}
/// Grep-style, regex based file-content search layer.
///
/// See the module documentation for the matching, ranking, time-budget
/// and caching behavior.
pub struct GrepLayer {
    cache: DirCache,
}
impl GrepLayer {
    pub fn new() -> Self {
        Self {
            cache: DirCache::new(DIR_CACHE_CAPACITY),
        }
    }
}
impl Default for GrepLayer {
    fn default() -> Self {
        Self::new()
    }
}
#[async_trait]
impl Layer for GrepLayer {
    fn id(&self) -> &str {
        "grep"
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
        let compiled: Vec<CompiledToken> = tokens
            .iter()
            .filter_map(|t| {
                Regex::new(t)
                    .ok()
                    .map(|regex| CompiledToken {
                        regex,
                        specificity: regex_specificity(t),
                    })
            })
            .collect();
        if compiled.is_empty() {
            status.skipped = true;
            return status;
        }
        let tokens = Arc::new(compiled);
        let (search_root, rel_prefix) = query.resolve_search_root();
        let mut files = Vec::new();
        collect_files(&self.cache, &search_root, &rel_prefix, &mut files);
        let mut queue: VecDeque<FileCandidate> = files.into();
        let start = Instant::now();
        let deadline = start + MAX_RUNTIME;
        let start_cutoff = start
            + Duration::from_secs_f64(MAX_RUNTIME.as_secs_f64() * START_CUTOFF_RATIO);
        let mut pending = FuturesUnordered::new();
        let mut all_hits: Vec<LineMatch> = Vec::new();
        let mut aborted = false;
        loop {
            while Instant::now() < start_cutoff {
                let Some(file) = queue.pop_front() else { break };
                let cpu = ctx.cpu.clone();
                let tokens = Arc::clone(&tokens);
                let rel_path = file.rel_path;
                let abs_path = file.abs_path;
                pending
                    .push(async move {
                        cpu.spawn(move || search_file(&abs_path, &rel_path, &tokens))
                            .await
                            .unwrap_or_default()
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
                maybe_hits = pending.next() => { if let Some(hits) = maybe_hits {
                all_hits.extend(hits); } } _ = tokio::time::sleep(deadline - now) => {
                aborted = true; break; }
            }
        }
        all_hits
            .sort_by(|a, b| {
                a.rank
                    .cmp(&b.rank)
                    .then(a.specificity.cmp(&b.specificity))
                    .then(a.rel_path.cmp(&b.rel_path))
                    .then(a.line_no.cmp(&b.line_no))
            });
        all_hits.truncate(MAX_MATCHES);
        let mut order: Vec<String> = Vec::new();
        let mut grouped: HashMap<String, Vec<(usize, String)>> = HashMap::new();
        for hit in all_hits {
            grouped
                .entry(hit.rel_path.clone())
                .or_insert_with(|| {
                    order.push(hit.rel_path.clone());
                    Vec::new()
                })
                .push((hit.line_no, hit.text));
        }
        for rel in order {
            let mut lines = grouped.remove(&rel).unwrap_or_default();
            lines.sort_by_key(|(ln, _)| *ln);
            let mut lines_obj = Map::new();
            for (ln, text) in lines {
                lines_obj.insert(ln.to_string(), Value::String(text));
            }
            let mut fields = Map::new();
            fields.insert("File".into(), Value::String(rel));
            fields.insert("Lines".into(), Value::Object(lines_obj));
            result_set.add(ResultEntry::new(None, fields));
            status.contributions += 1;
        }
        status.ran = true;
        status.aborted = aborted;
        status.detail.insert("root".into(), json!(search_root.display().to_string()));
        status
    }
}
