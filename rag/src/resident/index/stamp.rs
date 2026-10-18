//! Stamp generation and the fallback chain for file-backed indexes:
//! stamp -> size+mtime -> content hash.

use std::sync::atomic::{AtomicU64, Ordering};

use super::Stamp;

/// Conservative racy window (FAT worst case).
pub const RACY_WINDOW_NS: u64 = 2_000_000_000;

/// Monotonic stamp source. Uniqueness only needs `Relaxed`; visibility of the
/// data is established by the writer's lock/release when publishing.
#[derive(Debug)]
pub struct StampCounter(AtomicU64);

impl StampCounter {
    pub const fn new() -> Self {
        Self(AtomicU64::new(1))
    }

    /// Resume after restart: `max(stored, file_max) + 1`.
    pub fn resume(stored_high_water: u64, file_max: u64) -> Self {
        Self(AtomicU64::new(stored_high_water.max(file_max) + 1))
    }

    pub fn next(&self) -> Stamp {
        Stamp::from_raw(self.0.fetch_add(1, Ordering::Relaxed))
    }

    pub fn high_water(&self) -> u64 {
        self.0.load(Ordering::Relaxed) - 1
    }
}

impl Default for StampCounter {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FileStamp {
    pub size: u64,
    pub mtime_ns: u64,
    pub ctime_ns: Option<u64>,
    /// Reference time of the scan that recorded this stamp.
    pub scan_ns: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Verdict {
    Unchanged,
    NeedsHash,
}

impl FileStamp {
    /// Racy: mtime lies within the timestamp resolution before the scan, so a
    /// later write could have left it unchanged.
    pub fn is_racy(&self) -> bool {
        self.mtime_ns.saturating_add(RACY_WINDOW_NS) >= self.scan_ns
    }

    pub fn verdict(&self, current: &FileStamp) -> Verdict {
        let same = self.size == current.size
            && self.mtime_ns == current.mtime_ns
            && self.ctime_ns == current.ctime_ns;
        if same && !self.is_racy() {
            Verdict::Unchanged
        } else {
            tracing::debug!(racy = self.is_racy(), same, "fallback stamp requires content hash");
            Verdict::NeedsHash
        }
    }
}

/// xxh3_64; algorithm and version belong in the index header.
pub fn content_hash(bytes: &[u8]) -> u64 {
    xxhash_rust::xxh3::xxh3_64(bytes)
}
