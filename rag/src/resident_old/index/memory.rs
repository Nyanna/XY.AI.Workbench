//! Reference in-memory index; builds without `cuda`/`candle`.

use std::collections::BTreeMap;
use std::sync::Arc;

use parking_lot::Mutex;
use tokio::sync::watch;

use super::{
    AccessMethod, CellId, EntryData, EntryId, EntrySet, Index, NodeId, Stamp, StampCounter,
    SummaryRef,
};

struct Inner {
    entries: BTreeMap<EntryId, (Stamp, Arc<[u8]>)>,
}

pub struct InMemoryIndex {
    inner: Mutex<Inner>,
    counter: StampCounter,
    tx: watch::Sender<u64>,
}

impl InMemoryIndex {
    pub fn new() -> Self {
        Self::with_counter(StampCounter::new())
    }

    pub fn with_counter(counter: StampCounter) -> Self {
        let (tx, _) = watch::channel(counter.high_water());
        Self {
            inner: Mutex::new(Inner { entries: BTreeMap::new() }),
            counter,
            tx,
        }
    }

    pub fn high_water(&self) -> u64 {
        self.counter.high_water()
    }

    /// Removal consumes a stamp so that re-adding the same id yields a new one.
    pub fn remove(&self, entry: EntryId) -> Option<Stamp> {
        let mut g = self.inner.lock();
        g.entries.remove(&entry)?;
        let stamp = self.counter.next();
        self.tx.send_replace(stamp.raw());
        Some(stamp)
    }
}

impl Default for InMemoryIndex {
    fn default() -> Self {
        Self::new()
    }
}

impl Index for InMemoryIndex {
    fn stamp(&self, entry: EntryId) -> Option<Stamp> {
        self.inner.lock().entries.get(&entry).map(|(s, _)| *s)
    }

    fn stamps(&self) -> Box<dyn Iterator<Item = (EntryId, Stamp)> + '_> {
        let snapshot: Vec<_> = self.inner.lock().entries.iter().map(|(id, (s, _))| (*id, *s)).collect();
        Box::new(snapshot.into_iter())
    }

    fn read(&self, entry: EntryId) -> EntryData<'_> {
        match self.inner.lock().entries.get(&entry) {
            Some((_, data)) => EntryData::Shared(Arc::clone(data)),
            None => EntryData::Borrowed(&[]),
        }
    }

    fn apply(&self, entry: EntryId, data: &[u8]) -> Stamp {
        let mut g = self.inner.lock();
        // Stamp is drawn inside the write lock, immediately before publication.
        let stamp = self.counter.next();
        g.entries.insert(entry, (stamp, Arc::from(data)));
        self.tx.send_replace(stamp.raw());
        drop(g);
        tracing::debug!(entry = entry.get(), stamp = stamp.raw(), "apply");
        stamp
    }

    fn changed(&self) -> watch::Receiver<u64> {
        self.tx.subscribe()
    }

    fn capabilities(&self) -> &[AccessMethod] {
        &[AccessMethod::Range]
    }

    fn children(&self, _node: NodeId) -> EntrySet {
        EntrySet::new()
    }

    fn members(&self, cell: CellId) -> EntrySet {
        if cell.0 != 0 {
            return EntrySet::new();
        }
        EntrySet::from_ids(self.inner.lock().entries.keys().copied())
    }

    fn summary(&self, _kind: AccessMethod) -> Option<SummaryRef> {
        None
    }

    fn prefetch(&self, _set: &EntrySet) {}
}

#[cfg(test)]
#[path = "test/memory.rs"]
mod tests;
