//! Stamp generation and the fallback chain for file-backed indexes:
//! stamp -> size+mtime -> content hash.

use std::sync::atomic::{AtomicU64, Ordering};

use super::Stamp;

/// Conservative racy window (FAT worst case).
// TODO: confirm window against the target filesystems and decide xxh3_64 vs xxh3_128.
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counter_is_strictly_monotonic_and_unique_across_threads() {
        let c = std::sync::Arc::new(StampCounter::new());
        let hs: Vec<_> = (0..4)
            .map(|_| {
                let c = c.clone();
                std::thread::spawn(move || (0..500).map(|_| c.next().raw()).collect::<Vec<_>>())
            })
            .collect();
        let mut all = Vec::new();
        for h in hs {
            let v = h.join().unwrap();
            assert!(v.windows(2).all(|w| w[0] < w[1]));
            all.extend(v);
        }
        all.sort_unstable();
        all.dedup();
        assert_eq!(all.len(), 2000);
        assert_eq!(c.high_water(), 2000);
    }

    #[test]
    fn resume_continues_above_both_marks() {
        assert_eq!(StampCounter::resume(10, 7).next().raw(), 11);
        assert_eq!(StampCounter::resume(3, 40).next().raw(), 41);
        assert_eq!(StampCounter::resume(5, 5).high_water(), 5);
    }

    fn fs(size: u64, mtime_ns: u64, scan_ns: u64) -> FileStamp {
        FileStamp { size, mtime_ns, ctime_ns: Some(mtime_ns), scan_ns }
    }

    #[test]
    fn settled_identical_stamp_is_unchanged() {
        let a = fs(10, 1_000, 1_000 + RACY_WINDOW_NS + 1);
        assert!(!a.is_racy());
        assert_eq!(a.verdict(&a), Verdict::Unchanged);
    }

    #[test]
    fn racy_stamp_always_needs_hash() {
        let a = fs(10, 1_000, 1_000 + RACY_WINDOW_NS);
        assert!(a.is_racy());
        assert_eq!(a.verdict(&a), Verdict::NeedsHash);
    }

    #[test]
    fn any_metadata_difference_needs_hash() {
        let a = fs(10, 1_000, 10 * RACY_WINDOW_NS);
        for b in [
            FileStamp { size: 11, ..a },
            FileStamp { mtime_ns: 2_000, ..a },
            FileStamp { ctime_ns: None, ..a },
        ] {
            assert_eq!(a.verdict(&b), Verdict::NeedsHash);
        }
    }

    #[test]
    fn content_hash_is_stable_and_discriminating() {
        assert_eq!(content_hash(b"abc"), content_hash(b"abc"));
        assert_ne!(content_hash(b"abc"), content_hash(b"abd"));
    }
}
