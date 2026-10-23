use super::*;
use crate::resident::failpoint;
use crate::resident::index::memory::InMemoryIndex;
use crate::resident::index::{AccessMethod, CellId, EntrySet, NodeId, SummaryRef};
use crate::resident::manager::tracker::SlotRef;

const SLOT: SlotRef = SlotRef { segment: 0, slot: 0 };

fn id(n: u64) -> EntryId {
    EntryId::new(n)
}

/// Index whose reads and stamp scans can inject a concurrent write through failpoints.
struct Probe {
    inner: InMemoryIndex,
    key: u64,
}

impl Index for Probe {
    fn stamp(&self, entry: EntryId) -> Option<Stamp> {
        self.inner.stamp(entry)
    }
    fn stamps(&self) -> Box<dyn Iterator<Item = (EntryId, Stamp)> + '_> {
        let snapshot: Vec<_> = self.inner.stamps().collect();
        if failpoint::hit("sync.stamps", self.key) {
            self.inner.apply(id(1), b"during-scan");
        }
        Box::new(snapshot.into_iter())
    }
    fn read(&self, entry: EntryId) -> EntryData<'_> {
        if failpoint::hit("sync.read", self.key) {
            self.inner.apply(entry, b"newer");
        }
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
}

fn probe(key: u64) -> Probe {
    Probe { inner: InMemoryIndex::new(), key }
}

fn reconcile(ix: &dyn Index, tracker: &mut ResidencyTracker, out: &ScanOutcome) {
    for e in &out.reload {
        let loaded = load_entry(ix, *e).unwrap();
        tracker.insert(*e, SLOT, loaded.stamp);
    }
    for e in &out.removed {
        tracker.remove(*e);
    }
}

#[test]
fn own_write_is_not_reloaded_but_foreign_write_is() {
    let ix = InMemoryIndex::new();
    let mut tracker = ResidencyTracker::new();
    let mut scanner = SyncScanner::new(&ix);
    let own = ix.apply(id(1), b"a");
    tracker.insert(id(1), SLOT, own);
    let out = scanner.scan(&ix, &tracker);
    assert!(out.is_clean());
    assert_eq!(out.current, 1);

    ix.apply(id(1), b"b");
    assert_eq!(scanner.scan(&ix, &tracker).reload, [id(1)]);
}

#[test]
fn two_managers_converge_on_the_index_state() {
    let ix = InMemoryIndex::new();
    let (mut ta, mut tb) = (ResidencyTracker::new(), ResidencyTracker::new());
    let (mut sa, mut sb) = (SyncScanner::new(&ix), SyncScanner::new(&ix));
    ta.insert(id(1), SLOT, ix.apply(id(1), b"seed"));
    tb.insert(id(1), SLOT, ix.stamp(id(1)).unwrap());

    let own = ix.apply(id(1), b"from-a");
    ta.insert(id(1), SLOT, own);
    assert!(sa.scan(&ix, &ta).is_clean());
    let out = sb.scan(&ix, &tb);
    assert_eq!(out.reload, [id(1)]);
    reconcile(&ix, &mut tb, &out);

    assert!(sa.scan(&ix, &ta).is_clean());
    assert!(sb.scan(&ix, &tb).is_clean());
    assert_eq!(ta.known_stamp(id(1)), tb.known_stamp(id(1)));
}

#[test]
fn coalesced_signals_lose_no_change() {
    let ix = InMemoryIndex::new();
    let mut tracker = ResidencyTracker::new();
    for n in 0..3 {
        tracker.insert(id(n), SLOT, ix.apply(id(n), b"v0"));
    }
    let mut scanner = SyncScanner::new(&ix);
    for n in 0..3 {
        ix.apply(id(n), b"v1");
    }
    assert_eq!(scanner.scan(&ix, &tracker).reload, [id(0), id(1), id(2)]);
}

#[test]
fn non_resident_changes_are_ignored() {
    let ix = InMemoryIndex::new();
    let mut tracker = ResidencyTracker::new();
    tracker.insert(id(1), SLOT, ix.apply(id(1), b"a"));
    let mut scanner = SyncScanner::new(&ix);
    ix.apply(id(2), b"elsewhere");
    assert!(scanner.scan(&ix, &tracker).is_clean());
}

#[test]
fn send_during_scan_stays_unseen_and_is_found_next() {
    let ix = probe(6001);
    let mut tracker = ResidencyTracker::new();
    tracker.insert(id(1), SLOT, ix.apply(id(1), b"a"));
    let mut scanner = SyncScanner::new(&ix);
    let _fp = failpoint::arm("sync.stamps", 6001, 1);

    let first = scanner.scan(&ix, &tracker);
    assert!(first.is_clean());
    assert!(scanner.has_pending());
    assert_eq!(scanner.scan(&ix, &tracker).reload, [id(1)]);
    assert!(!scanner.has_pending());
}

#[tokio::test]
async fn changed_returns_immediately_for_an_unseen_signal() {
    let ix = InMemoryIndex::new();
    let tracker = ResidencyTracker::new();
    let mut scanner = SyncScanner::new(&ix);
    ix.apply(id(1), b"a");
    assert!(scanner.changed().await);
    scanner.scan(&ix, &tracker);
    assert!(!scanner.has_pending());
}

#[test]
fn write_between_stamp_and_data_read_repeats_the_load() {
    let ix = probe(6002);
    ix.apply(id(1), b"old");
    let _fp = failpoint::arm("sync.read", 6002, 1);

    let loaded = load_entry(&ix, id(1)).unwrap();
    assert_eq!(loaded.rechecks, 1);
    assert_eq!(loaded.data.as_bytes(), b"newer");
    assert_eq!(Some(loaded.stamp), ix.stamp(id(1)));
}

#[test]
fn exhausted_rechecks_register_the_older_stamp() {
    let ix = probe(6003);
    ix.apply(id(1), b"old");
    let _fp = failpoint::arm("sync.read", 6003, u32::MAX);

    let loaded = load_entry(&ix, id(1)).unwrap();
    assert_eq!(loaded.rechecks, MAX_RECHECKS);
    let mut tracker = ResidencyTracker::new();
    tracker.insert(id(1), SLOT, loaded.stamp);
    let mut scanner = SyncScanner::new(&ix);
    assert_eq!(scanner.scan(&ix, &tracker).reload, [id(1)]);
}

#[test]
fn load_of_a_removed_entry_is_none() {
    let ix = InMemoryIndex::new();
    assert!(load_entry(&ix, id(7)).is_none());
}

#[test]
fn remove_and_recreate_yields_a_different_stamp() {
    let ix = InMemoryIndex::new();
    let old = ix.apply(id(1), b"a");
    let mut tracker = ResidencyTracker::new();
    tracker.insert(id(1), SLOT, old);
    let mut scanner = SyncScanner::new(&ix);
    ix.remove(id(1)).unwrap();
    ix.apply(id(1), b"a");
    assert_ne!(ix.stamp(id(1)).unwrap(), old);
    assert_eq!(scanner.scan(&ix, &tracker).reload, [id(1)]);
}

#[test]
fn removed_entries_are_reported_and_queued_for_unbinding() {
    let ix = InMemoryIndex::new();
    let mut tracker = ResidencyTracker::new();
    tracker.insert(id(1), SLOT, ix.apply(id(1), b"a"));
    let mut scanner = SyncScanner::new(&ix);
    ix.remove(id(1));
    let out = scanner.scan(&ix, &tracker);
    assert_eq!(out.removed, [id(1)]);

    let mut queue = UpdateQueue::new();
    out.enqueue_removals(&mut queue);
    assert!(queue.contains(id(1)));
}

#[test]
fn scan_sequences_converge_to_the_index_state() {
    use proptest::prelude::*;
    use proptest::test_runner::{Config, RngAlgorithm, TestRng, TestRunner};

    let config = Config { cases: 64, failure_persistence: None, ..Config::default() };
    let rng = TestRng::from_seed(RngAlgorithm::ChaCha, &[7u8; 32]);
    let mut runner = TestRunner::new_with_rng(config, rng);
    let ops = proptest::collection::vec((0u8..4, 0u8..4), 1..40);
    runner
        .run(&ops, |ops| {
            let ix = InMemoryIndex::new();
            let mut tracker = ResidencyTracker::new();
            let mut scanner = SyncScanner::new(&ix);
            for (op, n) in ops {
                let e = id(u64::from(n));
                match op {
                    0 => {
                        ix.apply(e, b"x");
                    }
                    1 => {
                        ix.remove(e);
                    }
                    2 => {
                        if let Some(l) = load_entry(&ix, e) {
                            tracker.insert(e, SLOT, l.stamp);
                        }
                    }
                    _ => {
                        let out = scanner.scan(&ix, &tracker);
                        reconcile(&ix, &mut tracker, &out);
                    }
                }
            }
            let out = scanner.scan(&ix, &tracker);
            reconcile(&ix, &mut tracker, &out);
            let again = scanner.scan(&ix, &tracker);
            prop_assert!(again.is_clean());
            for (e, r) in tracker.iter() {
                prop_assert_eq!(ix.stamp(e), Some(r.stamp));
            }
            Ok(())
        })
        .unwrap();
}
