//! Index trait, Scope, AccessMethod, Stamp.
//!
//! The contract is independent of the core: an index reports changes and
//! delivers structure and stamps; it knows no geometry and resolves no scope.
//!
//! Write order is binding: data first, then stamp, then signal.
//! `read` may block on page faults and disk I/O; callers use a blocking pool.
pub mod memory;
pub mod parquet_io;
pub mod payload;
pub mod schema;
pub mod stamp;
use std::ops::Range;
use std::sync::Arc;
use roaring::RoaringTreemap;
use tokio::sync::watch;
pub use parquet_io::{Access, IndexOptions, ParquetIndex};
pub use payload::Payload;
pub use schema::{tokenizer_fingerprint, SchemaHeader, SCHEMA_VERSION};
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
/// Compact set of entry ids backed by a Roaring treemap (array, bitmap and run containers).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct EntrySet {
    bits: RoaringTreemap,
}
impl std::hash::Hash for EntrySet {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.bits.len().hash(state);
        self.bits.min().hash(state);
        self.bits.max().hash(state);
    }
}
impl EntrySet {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn from_ranges(ranges: impl IntoIterator<Item = Range<u64>>) -> Self {
        let mut bits = RoaringTreemap::new();
        for r in ranges {
            if r.start < r.end {
                bits.insert_range(r);
            }
        }
        Self { bits }
    }
    pub fn from_ids(ids: impl IntoIterator<Item = EntryId>) -> Self {
        Self {
            bits: ids.into_iter().map(|id| id.0).collect(),
        }
    }
    /// Maximal contiguous runs in ascending order; walks every id, so prefer `iter` or `len` on
    /// large sets.
    pub fn ranges(&self) -> Vec<Range<u64>> {
        let mut out: Vec<Range<u64>> = Vec::new();
        for id in self.bits.iter() {
            match out.last_mut() {
                Some(last) if last.end == id => last.end = id + 1,
                _ => out.push(id..id + 1),
            }
        }
        out
    }
    pub fn is_empty(&self) -> bool {
        self.bits.is_empty()
    }
    pub fn len(&self) -> u64 {
        self.bits.len()
    }
    pub fn contains(&self, id: EntryId) -> bool {
        self.bits.contains(id.0)
    }
    pub fn iter(&self) -> impl Iterator<Item = EntryId> + '_ {
        self.bits.iter().map(EntryId)
    }
    pub fn union(&self, other: &Self) -> Self {
        Self {
            bits: &self.bits | &other.bits,
        }
    }
    pub fn intersection(&self, other: &Self) -> Self {
        Self {
            bits: &self.bits & &other.bits,
        }
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
    /// Byte range of `entry` inside contiguous backing storage (e.g. a mapped file).
    /// `None` when the index has no such storage or the entry is missing.
    fn extent(&self, _entry: EntryId) -> Option<Range<usize>> {
        None
    }
    /// Bytes of a range obtained from [`extent`](Index::extent), or of a run of adjacent
    /// extents. `None` when the range is not backed. Never copies.
    fn backing(&self, _range: Range<usize>) -> Option<&[u8]> {
        None
    }
}
