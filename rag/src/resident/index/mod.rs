//! Index trait, Scope, AccessMethod, Stamp.
//!
//! The contract is independent of the core: an index reports changes and
//! delivers structure and stamps; it knows no geometry and resolves no scope.
//!
//! Write order is binding: data first, then stamp, then signal.
//! `read` may block on page faults and disk I/O; callers use a blocking pool.
pub mod memory;
pub mod stamp;
use std::ops::Range;
use std::sync::Arc;
use tokio::sync::watch;
pub use stamp::{content_hash, FileStamp, StampCounter, Verdict, RACY_WINDOW_NS};
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EntryId(u64);
impl EntryId {
    pub const fn new(raw: u64) -> Self {
        Self(raw)
    }
    pub const fn get(self) -> u64 {
        self.0
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct NodeId(pub u64);
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CellId(pub u64);
/// Opaque change marker. Compare with `!=` only; ordering is deliberately absent.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Stamp(u64);
impl Stamp {
    pub(crate) const fn from_raw(raw: u64) -> Self {
        Self(raw)
    }
    /// Raw value, e.g. as `watch` watermark or for persistence.
    pub const fn raw(self) -> u64 {
        self.0
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum AccessMethod {
    Centroid,
    Prefix,
    Subtree,
    Range,
}
#[derive(Clone, Debug, PartialEq)]
pub enum Scope {
    All,
    Range(Range<u64>),
    Set(EntrySet),
    Subtree(NodeId),
    Prefix(Vec<u8>),
    Centroid { k: u32, radius: Option<f32> },
}
/// Compact set of entry ids stored as sorted, merged, half-open ranges.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct EntrySet {
    ranges: Vec<Range<u64>>,
}
impl EntrySet {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn from_ranges(ranges: impl IntoIterator<Item = Range<u64>>) -> Self {
        let mut v: Vec<Range<u64>> = ranges
            .into_iter()
            .filter(|r| r.start < r.end)
            .collect();
        v.sort_by_key(|r| r.start);
        let mut merged: Vec<Range<u64>> = Vec::with_capacity(v.len());
        for r in v {
            match merged.last_mut() {
                Some(last) if r.start <= last.end => last.end = last.end.max(r.end),
                _ => merged.push(r),
            }
        }
        Self { ranges: merged }
    }
    pub fn from_ids(ids: impl IntoIterator<Item = EntryId>) -> Self {
        Self::from_ranges(ids.into_iter().map(|id| id.0..id.0.saturating_add(1)))
    }
    pub fn ranges(&self) -> &[Range<u64>] {
        &self.ranges
    }
    pub fn is_empty(&self) -> bool {
        self.ranges.is_empty()
    }
    pub fn len(&self) -> u64 {
        self.ranges.iter().map(|r| r.end - r.start).sum()
    }
    pub fn contains(&self, id: EntryId) -> bool {
        let i = self.ranges.partition_point(|r| r.end <= id.0);
        self.ranges.get(i).is_some_and(|r| r.start <= id.0)
    }
    pub fn iter(&self) -> impl Iterator<Item = EntryId> + '_ {
        self.ranges.iter().flat_map(|r| r.clone().map(EntryId))
    }
    pub fn union(&self, other: &Self) -> Self {
        Self::from_ranges(self.ranges.iter().chain(&other.ranges).cloned())
    }
    pub fn intersection(&self, other: &Self) -> Self {
        let (mut a, mut b) = (
            self.ranges.iter().peekable(),
            other.ranges.iter().peekable(),
        );
        let mut out = Vec::new();
        while let (Some(x), Some(y)) = (a.peek(), b.peek()) {
            let (start, end) = (x.start.max(y.start), x.end.min(y.end));
            if start < end {
                out.push(start..end);
            }
            if x.end <= y.end {
                a.next();
            } else {
                b.next();
            }
        }
        Self { ranges: out }
    }
}
/// Variable-length vector region of one entry; never a copy of backing storage.
#[derive(Clone, Debug)]
pub enum EntryData<'a> {
    Borrowed(&'a [u8]),
    Shared(Arc<[u8]>),
}
impl EntryData<'_> {
    pub fn as_bytes(&self) -> &[u8] {
        match self {
            Self::Borrowed(b) => b,
            Self::Shared(a) => a,
        }
    }
}
/// Summary structure such as a centroid table, exposed as vector data.
#[derive(Clone, Debug)]
pub struct SummaryRef(pub Arc<[u8]>);
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum HashAlgo {
    Xxh3_64,
    Xxh3_128,
    Blake3,
}
/// Binding between index and model; persistence format is owned by the index.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IndexHeader {
    pub model_id: String,
    pub dim: u32,
    pub high_water: u64,
    pub hash_algo: HashAlgo,
    pub hash_version: u32,
}
/// Object-safe: engines hold indexes as `Arc<dyn Index + Send + Sync>`.
pub trait Index {
    /// `None` = entry removed.
    fn stamp(&self, entry: EntryId) -> Option<Stamp>;
    /// Must not touch vector data.
    fn stamps(&self) -> Box<dyn Iterator<Item = (EntryId, Stamp)> + '_>;
    /// Missing entries yield empty data. May block on I/O.
    fn read(&self, entry: EntryId) -> EntryData<'_>;
    /// Serializes writers (last writer wins in dequeue order). Returns the stamp
    /// of this very write; callers register it as known to suppress their echo.
    fn apply(&self, entry: EntryId, data: &[u8]) -> Stamp;
    /// Level signal carrying the latest stamp watermark. Consume with
    /// `borrow_and_update()`; coalesced signals are harmless.
    fn changed(&self) -> watch::Receiver<u64>;
    fn capabilities(&self) -> &[AccessMethod];
    fn children(&self, node: NodeId) -> EntrySet;
    fn members(&self, cell: CellId) -> EntrySet;
    fn summary(&self, kind: AccessMethod) -> Option<SummaryRef>;
    /// Hint only; may be a no-op.
    fn prefetch(&self, set: &EntrySet);
}
