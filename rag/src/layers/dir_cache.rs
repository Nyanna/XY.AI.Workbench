//! Generic, mtime-validated in-memory cache of single-directory listings,
//! shared by any layer that needs to walk the filesystem.
//!
//! Control flow is cache-first: a caller asks for the listing of one
//! absolute directory path at a time. The cache answers from memory
//! whenever possible and only touches the filesystem when necessary:
//!
//! - If an entry exists and was checked less than [`STALE_AFTER`] ago,
//!   it is returned as-is (no filesystem access at all).
//! - Otherwise the directory's mtime is checked; if it still matches the
//!   cached mtime, the entry is refreshed ("last checked") and reused.
//! - If the entry is missing or its mtime changed, the directory is
//!   (re-)read synchronously/blocking and the result cached.
//!
//! Recursive traversal of a whole subtree is the caller's
//! responsibility: this cache only ever resolves one directory level
//! per call. Callers walk the tree iteratively (e.g. with an explicit
//! stack), issuing one [`DirCache::list`] call per level.
use std::num::NonZeroUsize;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Mutex, OnceLock};
use std::time::{Duration, Instant, SystemTime};
use glob::Pattern;
use lru::LruCache;
/// Entries are re-checked against the filesystem at most this often;
/// within this window a cache hit is trusted without any I/O.
const STALE_AFTER: Duration = Duration::from_secs(5);
/// Direct children of one directory, split into files and subdirectories.
/// Hidden entries (names starting with `.`) are never listed; no layer
/// built on this cache descends into dot-directories or indexes dot-files.
#[derive(Clone, Default)]
pub struct DirListing {
    pub files: Vec<FileInfo>,
    pub dirs: Vec<String>,
}
/// A listed file's name plus the metadata the cache already paid to read,
/// so callers never need a second `fs::metadata` round-trip just to get
/// the mtime (or size) of a file they obtained via [`DirCache::list`].
#[derive(Clone)]
pub struct FileInfo {
    pub name: String,
    pub mtime_ns: u64,
    pub size: u64,
}
/// Comma-separated-at-the-edge glob patterns that scope which directory
/// entries [`DirCache::list`] returns, without touching what it caches.
///
/// A name/path is kept if it matches `include` (when `include` is
/// non-empty, acting as a whitelist); otherwise it is kept unless it
/// matches `exclude`. Include always wins: an entry matching `include`
/// is never dropped by `exclude`.
#[derive(Clone, Default)]
pub struct PathFilter {
    include: Vec<Pattern>,
    exclude: Vec<Pattern>,
}
impl PathFilter {
    /// Builds a filter from glob pattern lists (already comma-split by
    /// the caller, e.g. the CLI).
    pub fn new(
        include: &[String],
        exclude: &[String],
    ) -> Result<Self, glob::PatternError> {
        let compile = |pats: &[String]| -> Result<Vec<Pattern>, glob::PatternError> {
            pats.iter().map(|p| Pattern::new(p)).collect()
        };
        Ok(Self {
            include: compile(include)?,
            exclude: compile(exclude)?,
        })
    }
    pub fn is_empty(&self) -> bool {
        self.include.is_empty() && self.exclude.is_empty()
    }
    fn matches_any(pats: &[Pattern], name: &str, rel: &str, abs: &str) -> bool {
        pats.iter().any(|p| p.matches(name) || p.matches(rel) || p.matches(abs))
    }
    fn keep(&self, name: &str, rel: &str, abs: &str) -> bool {
        if !self.include.is_empty() {
            return Self::matches_any(&self.include, name, rel, abs);
        }
        !Self::matches_any(&self.exclude, name, rel, abs)
    }
    /// Filters a copy of `listing` whose entries live directly under
    /// `dir`; `rel`/`abs` paths are matched in addition to bare names.
    fn apply(&self, listing: &DirListing, dir: &Path, root: &Path) -> DirListing {
        if self.is_empty() {
            return listing.clone();
        }
        let rel_base = dir.strip_prefix(root).unwrap_or(dir);
        let rel_of = |name: &str| {
            rel_base.join(name).to_string_lossy().replace('\\', "/")
        };
        let abs_of = |name: &str| dir.join(name).to_string_lossy().into_owned();
        DirListing {
            files: listing
                .files
                .iter()
                .filter(|f| self.keep(&f.name, &rel_of(&f.name), &abs_of(&f.name)))
                .cloned()
                .collect(),
            dirs: listing
                .dirs
                .iter()
                .filter(|name| self.keep(name, &rel_of(name), &abs_of(name)))
                .cloned()
                .collect(),
        }
    }
}
/// Process-wide root + filter, installed once from the CLI and applied
/// transparently by every [`DirCache::list`] call. A layer that needs a
/// different scope can bypass this via [`DirCache::list_with_filter`].
struct GlobalFilterConfig {
    root: PathBuf,
    filter: PathFilter,
}
static GLOBAL_FILTER: OnceLock<GlobalFilterConfig> = OnceLock::new();
/// Installs the process-wide include/exclude filter, relative to `root`.
/// Intended to be called once at startup, before any query runs; later
/// calls are ignored.
pub fn configure_global_filter(root: PathBuf, filter: PathFilter) {
    let _ = GLOBAL_FILTER.set(GlobalFilterConfig { root, filter });
}
struct CacheEntry {
    mtime: SystemTime,
    last_checked: Instant,
    listing: DirListing,
}
/// Bounded, mtime-validated in-memory cache of single-directory listings.
pub struct DirCache {
    inner: Mutex<LruCache<PathBuf, CacheEntry>>,
    /// Number of `list()` calls that could not be served from a fresh
    /// cache hit (cumulative since construction).
    misses: AtomicU64,
    /// Total nanoseconds spent re-validating or rebuilding entries
    /// (stat retrieval plus, on a full miss, the directory read).
    update_nanos: AtomicU64,
}
/// Point-in-time snapshot of [`DirCache`]'s cumulative counters; two
/// snapshots can be subtracted via [`DirCacheStats::delta`] to get the
/// activity within a time window (e.g. one query).
#[derive(Clone, Copy, Default)]
pub struct DirCacheStats {
    pub misses: u64,
    pub update_nanos: u64,
}
impl DirCacheStats {
    pub fn delta(&self, start: &DirCacheStats) -> DirCacheStats {
        DirCacheStats {
            misses: self.misses.saturating_sub(start.misses),
            update_nanos: self.update_nanos.saturating_sub(start.update_nanos),
        }
    }
    pub fn update_ms(&self) -> f64 {
        self.update_nanos as f64 / 1_000_000.0
    }
}
impl DirCache {
    pub fn new(capacity: usize) -> Self {
        Self {
            inner: Mutex::new(
                LruCache::new(NonZeroUsize::new(capacity).expect("capacity must be > 0")),
            ),
            misses: AtomicU64::new(0),
            update_nanos: AtomicU64::new(0),
        }
    }
    /// Cumulative miss/timing counters snapshot, see [`DirCacheStats`].
    pub fn stats(&self) -> DirCacheStats {
        DirCacheStats {
            misses: self.misses.load(Ordering::Relaxed),
            update_nanos: self.update_nanos.load(Ordering::Relaxed),
        }
    }
    /// Lists the direct children of the absolute path `dir`.
    ///
    /// Trusts a cache hit outright if it was last checked less than
    /// [`STALE_AFTER`] ago; otherwise validates (and, if needed,
    /// rebuilds) the entry against the filesystem before returning it.
    /// Any path beyond the fresh-hit check counts as a cache miss, and
    /// the time it takes (including stat retrieval) is accumulated.
    pub fn list(&self, dir: &Path) -> std::io::Result<DirListing> {
        let listing = self.list_unfiltered(dir)?;
        Ok(
            match GLOBAL_FILTER.get() {
                Some(cfg) if !cfg.filter.is_empty() => {
                    cfg.filter.apply(&listing, dir, &cfg.root)
                }
                _ => listing,
            },
        )
    }
    /// Like [`list`](Self::list), but applies `filter` (relative to
    /// `root`) instead of the process-wide one, for layers that need to
    /// scope their own traversal independently of the CLI-level filter.
    pub fn list_with_filter(
        &self,
        dir: &Path,
        root: &Path,
        filter: &PathFilter,
    ) -> std::io::Result<DirListing> {
        let listing = self.list_unfiltered(dir)?;
        Ok(if filter.is_empty() { listing } else { filter.apply(&listing, dir, root) })
    }
    /// Lists the direct children of `dir` straight from the cache,
    /// without applying any include/exclude filter.
    fn list_unfiltered(&self, dir: &Path) -> std::io::Result<DirListing> {
        let key = dir.to_path_buf();
        if let Some(listing) = self.fresh_hit(&key) {
            return Ok(listing);
        }
        self.misses.fetch_add(1, Ordering::Relaxed);
        let start = Instant::now();
        let result = (|| {
            let mtime = std::fs::metadata(dir)?.modified()?;
            if let Some(listing) = self.revalidated_hit(&key, mtime) {
                return Ok(listing);
            }
            let listing = read_dir_listing(dir)?;
            self.inner
                .lock()
                .unwrap()
                .put(
                    key,
                    CacheEntry {
                        mtime,
                        last_checked: Instant::now(),
                        listing: listing.clone(),
                    },
                );
            Ok(listing)
        })();
        self.update_nanos
            .fetch_add(start.elapsed().as_nanos() as u64, Ordering::Relaxed);
        result
    }
    /// Returns the cached listing if it was checked recently enough to
    /// be trusted without consulting the filesystem.
    fn fresh_hit(&self, key: &PathBuf) -> Option<DirListing> {
        let mut guard = self.inner.lock().unwrap();
        let entry = guard.get_mut(key)?;
        (entry.last_checked.elapsed() < STALE_AFTER).then(|| entry.listing.clone())
    }
    /// Returns the cached listing if its stored mtime still matches the
    /// filesystem's current mtime, refreshing the last-checked stamp.
    fn revalidated_hit(&self, key: &PathBuf, mtime: SystemTime) -> Option<DirListing> {
        let mut guard = self.inner.lock().unwrap();
        let entry = guard.get_mut(key)?;
        if entry.mtime != mtime {
            return None;
        }
        entry.last_checked = Instant::now();
        Some(entry.listing.clone())
    }
}
fn read_dir_listing(dir: &Path) -> std::io::Result<DirListing> {
    let mut listing = DirListing::default();
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let name = entry.file_name().to_string_lossy().into_owned();
        if name.starts_with('.') {
            continue;
        }
        let meta = entry.metadata()?;
        if meta.is_dir() {
            listing.dirs.push(name);
        } else {
            let mtime_ns = meta
                .modified()
                .ok()
                .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
                .map(|d| d.as_nanos() as u64)
                .unwrap_or(0);
            listing
                .files
                .push(FileInfo {
                    name,
                    mtime_ns,
                    size: meta.len(),
                });
        }
    }
    Ok(listing)
}
