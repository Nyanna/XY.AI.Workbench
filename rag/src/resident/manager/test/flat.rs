//! Index with contiguous backing storage, like a mapped file.
use std::collections::HashMap;
use std::ops::Range;
use std::sync::Arc;
use tokio::sync::watch;
use crate::resident::index::memory::InMemoryIndex;
use crate::resident::index::{
    AccessMethod, CellId, EntryData, EntryId, EntrySet, Index, NodeId, Stamp, SummaryRef,
};

pub(crate) struct FlatIndex {
    inner: InMemoryIndex,
    flat: Vec<u8>,
    extents: HashMap<EntryId, Range<usize>>,
}

impl FlatIndex {
    /// `entries` of `size` bytes each, `gap` zero bytes between neighbours; entry `n` holds
    /// the byte `n + 1`.
    pub(crate) fn new(entries: u64, size: usize, gap: usize) -> Arc<Self> {
        let inner = InMemoryIndex::new();
        let mut flat = Vec::new();
        let mut extents = HashMap::new();
        for n in 0..entries {
            let data = vec![n as u8 + 1; size];
            let id = EntryId::new(n);
            extents.insert(id, flat.len()..flat.len() + size);
            flat.extend_from_slice(&data);
            flat.extend(std::iter::repeat(0u8).take(gap));
            inner.apply(id, &data);
        }
        Arc::new(Self { inner, flat, extents })
    }
}

impl Index for FlatIndex {
    fn stamp(&self, entry: EntryId) -> Option<Stamp> {
        self.inner.stamp(entry)
    }
    fn stamps(&self) -> Box<dyn Iterator<Item = (EntryId, Stamp)> + '_> {
        self.inner.stamps()
    }
    fn read(&self, entry: EntryId) -> EntryData<'_> {
        self.inner.read(entry)
    }
    fn apply(&self, entry: EntryId, data: &[u8]) -> Stamp {
        self.inner.apply(entry, data)
    }
    fn changed(&self) -> watch::Receiver<u64> {
        self.inner.changed()
    }
    fn capabilities(&self) -> &[AccessMethod] {
        self.inner.capabilities()
    }
    fn children(&self, node: NodeId) -> EntrySet {
        self.inner.children(node)
    }
    fn members(&self, cell: CellId) -> EntrySet {
        self.inner.members(cell)
    }
    fn summary(&self, kind: AccessMethod) -> Option<SummaryRef> {
        self.inner.summary(kind)
    }
    fn prefetch(&self, set: &EntrySet) {
        self.inner.prefetch(set)
    }
    fn extent(&self, entry: EntryId) -> Option<Range<usize>> {
        self.extents.get(&entry).cloned()
    }
    fn backing(&self, range: Range<usize>) -> Option<&[u8]> {
        self.flat.get(range)
    }
}
