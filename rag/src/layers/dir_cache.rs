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
use std::sync::Mutex;
use std::time::{Duration, Instant, SystemTime};
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
struct CacheEntry {
    mtime: SystemTime,
    last_checked: Instant,
    listing: DirListing,
}
/// Bounded, mtime-validated in-memory cache of single-directory listings.
pub struct DirCache {
    inner: Mutex<LruCache<PathBuf, CacheEntry>>,
}
impl DirCache {
    pub fn new(capacity: usize) -> Self {
        Self {
            inner: Mutex::new(
                LruCache::new(NonZeroUsize::new(capacity).expect("capacity must be > 0")),
            ),
        }
    }
    /// Lists the direct children of the absolute path `dir`.
    ///
    /// Trusts a cache hit outright if it was last checked less than
    /// [`STALE_AFTER`] ago; otherwise validates (and, if needed,
    /// rebuilds) the entry against the filesystem before returning it.
    pub fn list(&self, dir: &Path) -> std::io::Result<DirListing> {
        let key = dir.to_path_buf();
        if let Some(listing) = self.fresh_hit(&key) {
            return Ok(listing);
        }
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
