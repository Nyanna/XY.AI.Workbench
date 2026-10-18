use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};

use crate::resident::op::ResourceKind;

const KINDS: usize = 6;

fn slot(kind: ResourceKind) -> usize {
    match kind {
        ResourceKind::Tokenizer => 0,
        ResourceKind::Normalizer => 1,
        ResourceKind::Encoder => 2,
        ResourceKind::Scorer => 3,
        ResourceKind::Index => 4,
        ResourceKind::CrossEncoder => 5,
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct StarvedSnapshot(u32);

impl StarvedSnapshot {
    pub fn contains(self, kind: ResourceKind) -> bool {
        self.0 & (1 << slot(kind)) != 0
    }

    pub fn is_empty(self) -> bool {
        self.0 == 0
    }
}

/// Decaying per-kind demand hints; a hint is cleared by a pick or by occupancy and
/// expires after `ttl`.
pub struct StarvedSet {
    marks: [AtomicU64; KINDS],
    ttl_ms: u64,
    origin: Instant,
}

impl StarvedSet {
    pub fn new(ttl: Duration) -> Self {
        Self {
            marks: std::array::from_fn(|_| AtomicU64::new(0)),
            ttl_ms: ttl.as_millis() as u64,
            origin: Instant::now(),
        }
    }

    fn now_ms(&self) -> u64 {
        self.origin.elapsed().as_millis() as u64
    }

    pub fn mark(&self, kind: ResourceKind) {
        self.mark_at(kind, self.now_ms());
    }

    pub fn clear(&self, kind: ResourceKind) {
        self.marks[slot(kind)].store(0, Ordering::Release);
    }

    pub fn clear_all(&self, snapshot: StarvedSnapshot) {
        for (i, mark) in self.marks.iter().enumerate() {
            if snapshot.0 & (1 << i) != 0 {
                mark.store(0, Ordering::Release);
            }
        }
    }

    pub fn snapshot(&self) -> StarvedSnapshot {
        self.snapshot_at(self.now_ms())
    }

    pub(crate) fn mark_at(&self, kind: ResourceKind, now_ms: u64) {
        self.marks[slot(kind)].store(now_ms + 1, Ordering::Release);
    }

    pub(crate) fn snapshot_at(&self, now_ms: u64) -> StarvedSnapshot {
        let mut bits = 0;
        for (i, mark) in self.marks.iter().enumerate() {
            let m = mark.load(Ordering::Acquire);
            if m != 0 && now_ms.saturating_sub(m - 1) <= self.ttl_ms {
                bits |= 1 << i;
            }
        }
        StarvedSnapshot(bits)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn set() -> StarvedSet {
        StarvedSet::new(Duration::from_millis(100))
    }

    #[test]
    fn mark_is_visible_until_ttl() {
        let s = set();
        s.mark_at(ResourceKind::Encoder, 10);
        assert!(s.snapshot_at(50).contains(ResourceKind::Encoder));
        assert!(!s.snapshot_at(50).contains(ResourceKind::Scorer));
        assert!(s.snapshot_at(111).is_empty());
    }

    #[test]
    fn pick_clears_only_its_kind() {
        let s = set();
        s.mark_at(ResourceKind::Encoder, 0);
        s.mark_at(ResourceKind::Scorer, 0);
        s.clear(ResourceKind::Encoder);
        let snap = s.snapshot_at(1);
        assert!(!snap.contains(ResourceKind::Encoder));
        assert!(snap.contains(ResourceKind::Scorer));
    }

    #[test]
    fn remark_extends_lifetime() {
        let s = set();
        s.mark_at(ResourceKind::Index, 0);
        s.mark_at(ResourceKind::Index, 90);
        assert!(s.snapshot_at(150).contains(ResourceKind::Index));
    }

    #[test]
    fn clear_all_consumes_snapshot() {
        let s = set();
        s.mark_at(ResourceKind::Encoder, 0);
        s.mark_at(ResourceKind::Tokenizer, 0);
        let snap = s.snapshot_at(1);
        s.clear_all(snap);
        assert!(s.snapshot_at(1).is_empty());
    }
}
