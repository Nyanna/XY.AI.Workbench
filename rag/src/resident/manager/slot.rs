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
mod tests {
    use super::*;
    use crate::resident::index::memory::InMemoryIndex;
    use crate::resident::kernel::mock::MockKernel;
    use crate::resident::kernel::{BatchDesc, Kernel, KernelOutput, RegionSpan};
    struct Zeros;
    impl RangeSource for Zeros {
        fn read(&self, range: WrittenRange) -> Vec<u8> {
            vec![0; range.len as usize]
        }
    }
    fn id(n: u64) -> EntryId {
        EntryId::new(n)
    }
    fn bound(map: SlotMap, entries: &[Option<u64>]) -> SlotMap {
        for (slot, e) in entries.iter().enumerate() {
            if let Some(n) = e {
                map.bind(
                        slot as u32,
                        EntryRef {
                            entry: id(*n),
                            chunk: 0,
                        },
                    )
                    .unwrap();
            }
        }
        map
    }
    fn fixed_map() -> AddressMap {
        let mut map = AddressMap::new();
        let seg = bound(
            SlotMap::fixed(1000, 16, 4),
            &[Some(1), Some(2), Some(3), Some(4)],
        );
        assert!(map.add_segment(0, seg));
        map
    }
    fn hit(offset: u64) -> KernelHit {
        KernelHit { offset, score: 0.5 }
    }
    #[test]
    fn freshly_written_address_resolves() {
        let index = InMemoryIndex::new();
        let mut map = fixed_map();
        let (mut q, mut t) = (UpdateQueue::new(), ResidencyTracker::new());
        let writes = [
            WrittenRange {
                offset: 1000,
                len: 64,
            },
        ];
        let hits = run_slot(
            &writes,
            &[hit(1020)],
            &Zeros,
            &index,
            &mut map,
            &mut q,
            &mut t,
        );
        assert_eq!(hits, vec![Hit { entry : id(2), score : 0.5, outdated : false }]);
        assert_eq!(t.len(), 4);
        assert_eq!(t.known_stamp(id(2)), index.stamp(id(2)));
    }
    #[test]
    fn removed_entry_is_discarded_and_pending_update_marks_outdated() {
        let index = InMemoryIndex::new();
        let mut map = fixed_map();
        let (mut q, mut t) = (UpdateQueue::new(), ResidencyTracker::new());
        for n in 1..=3 {
            index.apply(id(n), &[0]);
        }
        index.remove(id(1));
        q.push(Update::Unbind { entry: id(3) });
        let hits = [hit(1000), hit(1016), hit(1032)];
        let out = run_slot(&[], &hits, &Zeros, &index, &mut map, &mut q, &mut t);
        assert_eq!(out.len(), 2);
        assert!(! out[0].outdated && out[0].entry == id(2));
        assert!(out[1].outdated && out[1].entry == id(3));
    }
    #[test]
    fn mapping_change_takes_effect_in_the_next_batch() {
        let index = InMemoryIndex::new();
        index.apply(id(3), &[0]);
        let mut map = fixed_map();
        let (mut q, mut t) = (UpdateQueue::new(), ResidencyTracker::new());
        let stamp = index.stamp(id(3)).unwrap();
        let target = SlotRef { segment: 0, slot: 0 };
        t.insert(id(3), SlotRef { segment: 0, slot: 2 }, stamp);
        q.push(Update::Bind {
            entry: id(3),
            slot: target,
            stamp,
        });
        let first = run_slot(
            &[],
            &[hit(1032)],
            &Zeros,
            &index,
            &mut map,
            &mut q,
            &mut t,
        );
        assert_eq!(first[0].entry, id(3));
        assert!(first[0].outdated);
        assert_eq!(map.resolve(1032), None);
        assert_eq!(map.resolve(1000), Some(id(3)));
        let second = run_slot(
            &[],
            &[hit(1000)],
            &Zeros,
            &index,
            &mut map,
            &mut q,
            &mut t,
        );
        assert!(! second[0].outdated);
    }
    #[test]
    fn variable_layout_resolves_at_segment_boundaries() {
        let mut map = AddressMap::new();
        let seg = bound(
            SlotMap::variable(vec![500, 510, 510, 535]),
            &[Some(1), Some(2), Some(3)],
        );
        assert!(map.add_segment(7, seg));
        assert_eq!(map.resolve(500), Some(id(1)));
        assert_eq!(map.resolve(509), Some(id(1)));
        assert_eq!(map.resolve(510), Some(id(3)));
        assert_eq!(map.resolve(534), Some(id(3)));
        assert_eq!(map.resolve(535), None);
        assert_eq!(map.resolve(499), None);
    }
    #[test]
    fn snapshot_is_stable_while_segments_are_appended() {
        let mut map = fixed_map();
        let snapshot = map.clone();
        assert!(
            map.add_segment(1, bound(SlotMap::fixed(2000, 8, 2), & [Some(9), None]))
        );
        assert_eq!(map.resolve(2000), Some(id(9)));
        assert_eq!(snapshot.resolve(2000), None);
        assert!(! map.add_segment(2, SlotMap::fixed(1010, 8, 1)));
        assert!(map.remove_segment(1));
        assert_eq!(map.resolve(2000), None);
    }
    #[test]
    fn unmapped_write_is_retried_in_the_next_slot() {
        let index = InMemoryIndex::new();
        let mut map = AddressMap::new();
        let (mut q, mut t) = (UpdateQueue::new(), ResidencyTracker::new());
        let writes = [
            WrittenRange {
                offset: 1000,
                len: 16,
            },
        ];
        run_slot(&writes, &[], &Zeros, &index, &mut map, &mut q, &mut t);
        assert_eq!(q.deferred_writes(), 1);
        assert!(t.is_empty());
        map = fixed_map();
        run_slot(&[], &[], &Zeros, &index, &mut map, &mut q, &mut t);
        assert_eq!(q.deferred_writes(), 0);
        assert_eq!(t.len(), 1);
    }
    #[test]
    fn kernel_fake_drives_writes_then_hits() {
        let index = InMemoryIndex::new();
        let mut map = fixed_map();
        let (mut q, mut t) = (UpdateQueue::new(), ResidencyTracker::new());
        let writer = MockKernel::embedding(1 << 20);
        let b = writer.begin(&BatchDesc::default()).unwrap();
        writer
            .run(
                b,
                &[
                    RegionSpan {
                        offset: 1000,
                        len: 48,
                    },
                ],
            )
            .unwrap();
        let writes = writer.end(b).unwrap();
        let scorer = MockKernel::scoring();
        let b = scorer
            .begin(
                &BatchDesc {
                    queries: 1,
                    ..Default::default()
                },
            )
            .unwrap();
        let KernelOutput::Hits(hits) = scorer
            .run(
                b,
                &[
                    RegionSpan {
                        offset: 1040,
                        len: 16,
                    },
                ],
            )
            .unwrap() else {
            panic!("scoring kernel returned a partial result");
        };
        scorer.end(b).unwrap();
        let out = run_slot(&writes, &hits, &Zeros, &index, &mut map, &mut q, &mut t);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].entry, id(3));
        assert_eq!(t.len(), 3);
    }
    #[test]
    fn stale_batch_epoch_is_reported_without_side_effects() {
        let index = InMemoryIndex::new();
        let store = MappingStore::new();
        let mut next = store.snapshot().successor();
        assert!(next.add_segment(0, bound(SlotMap::fixed(1000, 16, 1), & [Some(1)])));
        store.publish(next).unwrap();
        let (mut q, mut t) = (UpdateQueue::new(), ResidencyTracker::new());
        let writes = [
            WrittenRange {
                offset: 1000,
                len: 16,
            },
        ];
        let old = Snapshot { root: 0, epoch: 0 };
        let res = run_slot_at(
            &store,
            &old,
            &writes,
            &[],
            &Zeros,
            &index,
            &mut q,
            &mut t,
        );
        assert!(
            matches!(res, Err(ManagerError::StaleMapping { base : 0, published : 1 }))
        );
        assert!(t.is_empty() && index.stamp(id(1)).is_none());
        let now = Snapshot { root: 0, epoch: 1 };
        run_slot_at(&store, &now, &writes, &[], &Zeros, &index, &mut q, &mut t).unwrap();
        assert_eq!(t.len(), 1);
    }
    #[test]
    fn publish_requires_the_next_epoch() {
        let store = MappingStore::new();
        let skipped = store.snapshot().successor().successor();
        assert!(
            matches!(store.publish(skipped), Err(ManagerError::StaleMapping { .. }))
        );
        assert!(store.publish(store.snapshot().successor()).is_ok());
        assert_eq!(store.snapshot().epoch(), 1);
    }
}
