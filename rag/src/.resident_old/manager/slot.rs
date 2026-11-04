//! Communication slot between kernel batches. Phases run in a fixed order: kernel writes are
//! registered with the index, results are resolved, and only then is the mapping mutated.
use std::collections::HashMap;
use std::sync::Arc;
use parking_lot::Mutex;
use crate::resident::error::ManagerError;
use crate::resident::index::{EntryId, Index, Stamp};
use crate::resident::kernel::{KernelHit, WrittenRange};
use crate::resident::manager::swap::Snapshot;
use crate::resident::manager::tracker::{ResidencyTracker, SlotRef};
use crate::resident::segment::mapping::{EntryRef, SlotMap};
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Hit {
    pub entry: EntryId,
    pub score: f32,
    pub outdated: bool,
}
#[derive(Debug)]
struct MappedSegment {
    segment: u32,
    map: SlotMap,
    bounds: std::ops::Range<u64>,
}
struct Located {
    slot: SlotRef,
    span: std::ops::Range<u64>,
    entry: Option<EntryId>,
}
/// Arena address -> entry. The segment set is copy-on-write: a clone is the stable snapshot of a
/// batch, later `add_segment`/`remove_segment` calls do not reach it. Slot bindings live in the
/// shared `SlotMap`s and are only changed inside the slot gap.
#[derive(Clone, Debug, Default)]
pub struct AddressMap {
    epoch: u64,
    segments: Arc<Vec<Arc<MappedSegment>>>,
}
impl AddressMap {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn epoch(&self) -> u64 {
        self.epoch
    }
    /// Copy for the next table epoch.
    pub fn successor(&self) -> Self {
        Self {
            epoch: self.epoch + 1,
            segments: self.segments.clone(),
        }
    }
    /// Returns false when the segment is empty, its id is taken or its range overlaps another.
    pub fn add_segment(&mut self, segment: u32, map: SlotMap) -> bool {
        let Some(bounds) = map.bounds().filter(|b| b.start < b.end) else {
            return false;
        };
        if self.segments.iter().any(|s| s.segment == segment) {
            return false;
        }
        let at = self.segments.partition_point(|s| s.bounds.start < bounds.start);
        let before_ok = at == 0 || self.segments[at - 1].bounds.end <= bounds.start;
        let after_ok = self
            .segments
            .get(at)
            .is_none_or(|n| bounds.end <= n.bounds.start);
        if !(before_ok && after_ok) {
            return false;
        }
        let mut next = (*self.segments).clone();
        next.insert(
            at,
            Arc::new(MappedSegment {
                segment,
                map,
                bounds,
            }),
        );
        self.segments = Arc::new(next);
        true
    }
    pub fn remove_segment(&mut self, segment: u32) -> bool {
        if !self.segments.iter().any(|s| s.segment == segment) {
            return false;
        }
        let next = self
            .segments
            .iter()
            .filter(|s| s.segment != segment)
            .cloned()
            .collect();
        self.segments = Arc::new(next);
        true
    }
    pub fn resolve(&self, off: u64) -> Option<EntryId> {
        self.locate(off).and_then(|l| l.entry)
    }
    fn locate(&self, off: u64) -> Option<Located> {
        let idx = self.segments.partition_point(|s| s.bounds.start <= off);
        let seg = self.segments.get(idx.checked_sub(1)?)?;
        if off >= seg.bounds.end {
            return None;
        }
        let slot = seg.map.index_of(off)?;
        Some(Located {
            slot: SlotRef {
                segment: seg.segment,
                slot,
            },
            span: seg.map.slot_span(slot)?,
            entry: seg.map.entry(slot).map(|e| e.entry),
        })
    }
    fn segment(&self, id: u32) -> Option<&MappedSegment> {
        self.segments.iter().find(|s| s.segment == id).map(|s| &**s)
    }
    fn bind(&self, slot: SlotRef, entry: EntryId) -> bool {
        self.segment(slot.segment)
            .is_some_and(|s| s.map.bind(slot.slot, EntryRef { entry, chunk: 0 }).is_ok())
    }
    fn unbind_if(&self, slot: SlotRef, entry: EntryId) {
        if let Some(s) = self.segment(slot.segment) {
            if s.map.entry(slot.slot).is_some_and(|e| e.entry == entry) {
                let _ = s.map.unbind(slot.slot);
            }
        }
    }
}
/// Versioned mapping, advanced together with the segment table epoch of the swap manager.
#[derive(Debug, Default)]
pub struct MappingStore {
    current: Mutex<AddressMap>,
}
impl MappingStore {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn snapshot(&self) -> AddressMap {
        self.current.lock().clone()
    }
    /// The mapping a batch started on `at` must resolve against.
    pub(crate) fn snapshot_at(&self, at: &Snapshot) -> Result<AddressMap, ManagerError> {
        let cur = self.current.lock();
        if cur.epoch != at.epoch {
            return Err(ManagerError::StaleMapping {
                base: at.epoch,
                published: cur.epoch,
            });
        }
        Ok(cur.clone())
    }
    pub(crate) fn publish(&self, next: AddressMap) -> Result<(), ManagerError> {
        let mut cur = self.current.lock();
        if next.epoch != cur.epoch + 1 {
            return Err(ManagerError::StaleMapping {
                base: next.epoch.saturating_sub(1),
                published: cur.epoch,
            });
        }
        *cur = next;
        Ok(())
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Update {
    Unbind { entry: EntryId },
    Bind { entry: EntryId, slot: SlotRef, stamp: Stamp },
}
impl Update {
    fn entry(&self) -> EntryId {
        match *self {
            Update::Unbind { entry } | Update::Bind { entry, .. } => entry,
        }
    }
}
/// Pending mutations, latest per entry; writes that could not be attributed wait for the next slot.
#[derive(Debug, Default)]
pub struct UpdateQueue {
    pending: HashMap<EntryId, Update>,
    retry: Vec<WrittenRange>,
}
impl UpdateQueue {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn push(&mut self, update: Update) {
        self.pending.insert(update.entry(), update);
    }
    pub fn contains(&self, entry: EntryId) -> bool {
        self.pending.contains_key(&entry)
    }
    pub fn depth(&self) -> usize {
        self.pending.len()
    }
    pub fn deferred_writes(&self) -> usize {
        self.retry.len()
    }
    fn apply(&mut self, map: &AddressMap, tracker: &mut ResidencyTracker) -> usize {
        let (unbinds, binds): (Vec<_>, Vec<_>) = self
            .pending
            .drain()
            .map(|(_, u)| u)
            .partition(|u| matches!(u, Update::Unbind { .. }));
        let applied = unbinds.len() + binds.len();
        for u in unbinds.into_iter().chain(binds) {
            match u {
                Update::Unbind { entry } => {
                    if let Some(old) = tracker.remove(entry) {
                        map.unbind_if(old.slot, entry);
                    }
                }
                Update::Bind { entry, slot, stamp } => {
                    if let Some(old) = tracker.get(entry) {
                        map.unbind_if(old.slot, entry);
                    }
                    if map.bind(slot, entry) {
                        tracker.insert(entry, slot, stamp);
                    } else {
                        tracing::warn!(? slot, "bind target not mapped");
                    }
                }
            }
        }
        applied
    }
}
/// Reads back the bytes a kernel wrote into a device range.
pub trait RangeSource {
    fn read(&self, range: WrittenRange) -> Vec<u8>;
}
/// Runs one slot against the mapping of the batch epoch `at`. A mapping that moved on since the
/// batch started is reported as `StaleMapping` without side effects; the caller retries the slot
/// with a fresh snapshot.
#[allow(clippy::too_many_arguments)]
pub(crate) fn run_slot_at(
    store: &MappingStore,
    at: &Snapshot,
    writes: &[WrittenRange],
    hits: &[KernelHit],
    source: &dyn RangeSource,
    index: &dyn Index,
    updates: &mut UpdateQueue,
    tracker: &mut ResidencyTracker,
) -> Result<Vec<Hit>, ManagerError> {
    let mut map = store.snapshot_at(at)?;
    Ok(run_slot(writes, hits, source, index, &mut map, updates, tracker))
}
pub fn run_slot(
    writes: &[WrittenRange],
    hits: &[KernelHit],
    source: &dyn RangeSource,
    index: &dyn Index,
    map: &mut AddressMap,
    updates: &mut UpdateQueue,
    tracker: &mut ResidencyTracker,
) -> Vec<Hit> {
    let _span = tracing::info_span!("slot_cycle", epoch = map.epoch).entered();
    let applied = apply_writes(writes, source, index, map, updates, tracker);
    let mut out = Vec::with_capacity(hits.len());
    let mut discarded = 0usize;
    let mut outdated = 0usize;
    for hit in hits {
        let Some(entry) = map.resolve(hit.offset) else {
            discarded += 1;
            continue;
        };
        if index.stamp(entry).is_none() {
            discarded += 1;
            continue;
        }
        let stale = updates.contains(entry);
        outdated += usize::from(stale);
        out.push(Hit {
            entry,
            score: hit.score,
            outdated: stale,
        });
    }
    let queue_depth = updates.depth();
    let mutated = updates.apply(map, tracker);
    tracing::debug!(
        applied, resolved = out.len(), discarded, outdated, queue_depth, mutated,
        "slot cycle"
    );
    out
}
fn apply_writes(
    writes: &[WrittenRange],
    source: &dyn RangeSource,
    index: &dyn Index,
    map: &AddressMap,
    updates: &mut UpdateQueue,
    tracker: &mut ResidencyTracker,
) -> usize {
    let ranges: Vec<WrittenRange> = std::mem::take(&mut updates.retry)
        .into_iter()
        .chain(writes.iter().copied())
        .collect();
    let mut applied = 0;
    for range in ranges {
        let end = range.offset + range.len;
        let mut at = range.offset;
        while at < end {
            let Some(loc) = map.locate(at) else {
                tracing::debug!(offset = at, "unmapped write deferred");
                updates
                    .retry
                    .push(WrittenRange {
                        offset: at,
                        len: end - at,
                    });
                break;
            };
            if let Some(entry) = loc.entry {
                let bytes = source
                    .read(WrittenRange {
                        offset: loc.span.start,
                        len: loc.span.end - loc.span.start,
                    });
                let stamp = index.apply(entry, &bytes);
                tracker.insert(entry, loc.slot, stamp);
                applied += 1;
            }
            at = loc.span.end.max(at + 1);
        }
    }
    applied
}
#[cfg(test)]
#[path = "test/slot.rs"]
mod tests;
