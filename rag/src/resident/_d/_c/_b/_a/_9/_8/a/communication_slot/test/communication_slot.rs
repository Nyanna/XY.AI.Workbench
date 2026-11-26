use super::*;
use std::collections::{BTreeSet, HashMap};
use crate::resident::_d::_c::_b::_a::_9::_8::_7::a::manifest::{
    ManifestStore, SegmentSpec, StoreConfig,
};
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::a::segments::{
    SegmentId, SlotRef,
};
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::a::slot_layout::{
    EntryRef, SegmentView, SlotAssignment, SlotClass, SlotStates,
};
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::a::kernel_layout::{
    KernelHit, Slot5,
};
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::a::reference_index::ReferenceIndex;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::a::region_handle::{
    Generation, RegionHandle,
};
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::a::arenas::ArenaKind;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::a::index_contract::{
    AccessMethod, IndexError, Payload2, Stamp, StampObserver,
};
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::_1::layout::DType;
use self::hit_queues::{channel, HitProducer, WriteProducer};
use self::hit_resolution::SegmentBinding;
const SLOT_BYTES: u64 = 512;
const SEG1_BASE: u64 = 0x10000;
const SEG2_BASE: u64 = 0x20000;
const RECLAIMED: usize = 3;
type Log = Arc<Mutex<Vec<String>>>;
fn note(log: &Log, text: impl Into<String>) {
    log.lock().unwrap().push(text.into());
}
fn events(log: &Log) -> Vec<String> {
    log.lock().unwrap().clone()
}
fn read_event(addr: u64) -> String {
    format!("read {:#x} {}", addr, SLOT_BYTES)
}
fn fill_for(addr: Addr) -> u8 {
    (addr.0 % 251) as u8
}
fn entry(n: u64) -> EntryRef {
    EntryRef {
        entry: EntryId(n),
        section: 0,
    }
}
fn seg1(slot: u32) -> u64 {
    SEG1_BASE + u64::from(slot) * SLOT_BYTES
}
struct MockReadback {
    log: Log,
    failing: Mutex<BTreeSet<u64>>,
}
impl DeviceReadback for MockReadback {
    fn read_back(&self, addr: Addr, dst: &mut [u8]) -> Result<(), ReadbackError> {
        note(&self.log, format!("read {:#x} {}", addr.0, dst.len()));
        if self.failing.lock().unwrap().contains(&addr.0) {
            return Err(ReadbackError("boom".into()));
        }
        dst.fill(fill_for(addr));
        Ok(())
    }
}
#[derive(Default)]
struct Dir(HashMap<SegmentId, SegmentBinding>);
impl SegmentDirectory for Dir {
    fn binding(&self, id: SegmentId) -> Option<SegmentBinding> {
        self.0.get(&id).cloned()
    }
}
struct Held(Log);
impl StructureHold for Held {}
impl Drop for Held {
    fn drop(&mut self) {
        note(&self.0, "unlock_structure");
    }
}
struct MockMapping {
    snap: Mutex<MappingSnapshot>,
    log: Log,
}
impl MappingSource for MockMapping {
    fn snapshot(&self) -> MappingSnapshot {
        note(&self.log, "snapshot");
        self.snap.lock().unwrap().clone()
    }
    fn lock_structure(&self) -> Box<dyn StructureHold + '_> {
        note(&self.log, "lock_structure");
        Box::new(Held(Arc::clone(&self.log)))
    }
    fn reclaim_deferred(&self) -> usize {
        note(&self.log, "reclaim");
        RECLAIMED
    }
}
struct MockUpdates {
    pending: BTreeSet<EntryId>,
    log: Log,
}
impl UpdateApplier for MockUpdates {
    fn depth(&self) -> usize {
        self.pending.len()
    }
    fn is_pending(&self, entry: EntryId) -> bool {
        self.pending.contains(&entry)
    }
    fn apply(&mut self, _residency: &ResidencyDirectory) -> UpdateOutcome {
        note(&self.log, "apply");
        let applied = self.pending.len();
        self.pending.clear();
        UpdateOutcome { applied }
    }
}
struct FlakyIndex {
    inner: ReferenceIndex,
    failing: Mutex<BTreeSet<EntryId>>,
}
impl Index for FlakyIndex {
    fn dtype(&self) -> DType {
        self.inner.dtype()
    }
    fn access_methods(&self) -> &[AccessMethod] {
        self.inner.access_methods()
    }
    fn stamp(&self, id: EntryId) -> Option<Stamp> {
        self.inner.stamp(id)
    }
    fn stamps(&self) -> Vec<(EntryId, Stamp)> {
        self.inner.stamps()
    }
    fn read(&self, id: EntryId) -> Payload2 {
        self.inner.read(id)
    }
    fn write(&self, id: EntryId, payload: PayloadRef<'_>) -> Result<Stamp, IndexError> {
        if self.failing.lock().unwrap().contains(&id) {
            return Err(IndexError::Backend("down".into()));
        }
        self.inner.write(id, payload)
    }
    fn delete(&self, id: EntryId) -> Result<Option<Stamp>, IndexError> {
        self.inner.delete(id)
    }
    fn subscribe(&self) -> StampObserver {
        self.inner.subscribe()
    }
}
fn region(len: u64) -> RegionHandle {
    RegionHandle::new(ArenaKind::IndexSegments, 0, len, Generation(0)).unwrap()
}
fn fixed(dir: &mut Dir, id: u64, generation: u64, base: u64, slots: u32) -> SegmentSpec {
    let view = SegmentView::new(
            region(u64::from(slots) * SLOT_BYTES),
            slots,
            SlotClass::C32,
            4,
            DType::F32,
            Generation(generation),
        )
        .unwrap();
    let spec = SegmentSpec::from_view(SegmentId(id), &view, Addr(base)).unwrap();
    dir.0
        .insert(
            SegmentId(id),
            SegmentBinding {
                assignment: Arc::new(SlotAssignment::fixed(&view, Addr(base)).unwrap()),
                states: Arc::new(SlotStates::new(&view)),
            },
        );
    spec
}
fn make_specs(dir: &mut Dir) -> Vec<SegmentSpec> {
    vec![fixed(dir, 1, 1, SEG1_BASE, 4), fixed(dir, 2, 2, SEG2_BASE, 2),]
}
fn chain(specs: Vec<Vec<SegmentSpec>>) -> Vec<Manifest> {
    let store = ManifestStore::new(StoreConfig::default());
    let mut out: Vec<Manifest> = Vec::new();
    for next_specs in specs {
        let next = match out.last() {
            Some(base) => store.successor(base, next_specs),
            None => store.successor(&store.current(), next_specs),
        }
            .unwrap();
        out.push(next);
    }
    out
}
struct Env {
    slot: CommunicationSlot,
    writes: WriteProducer,
    hits: HitProducer,
    dir: Arc<Dir>,
    updates: Arc<Mutex<MockUpdates>>,
    residency: Arc<Mutex<ResidencyDirectory>>,
    readback: Arc<MockReadback>,
    index: Arc<FlakyIndex>,
    manifests: Vec<Arc<Manifest>>,
    log: Log,
}
impl Env {
    fn new(generations: usize) -> Env {
        let log = Log::default();
        let mut dir = Dir::default();
        let specs: Vec<Vec<SegmentSpec>> = (0..generations)
            .map(|_| make_specs(&mut dir))
            .collect();
        let manifests: Vec<Arc<Manifest>> = chain(specs)
            .into_iter()
            .map(Arc::new)
            .collect();
        let last = manifests.len() - 1;
        let snap = MappingSnapshot {
            current: Arc::clone(&manifests[last]),
            previous: last.checked_sub(1).map(|i| Arc::clone(&manifests[i])),
        };
        let (writes, hits, consumer) = channel(16, 16);
        let readback = Arc::new(MockReadback {
            log: Arc::clone(&log),
            failing: Mutex::new(BTreeSet::new()),
        });
        let dir = Arc::new(dir);
        let mapping = Arc::new(MockMapping {
            snap: Mutex::new(snap),
            log: Arc::clone(&log),
        });
        let updates = Arc::new(
            Mutex::new(MockUpdates {
                pending: BTreeSet::new(),
                log: Arc::clone(&log),
            }),
        );
        let residency = Arc::new(Mutex::new(ResidencyDirectory::new()));
        let index = Arc::new(FlakyIndex {
            inner: ReferenceIndex::new(DType::F32),
            failing: Mutex::new(BTreeSet::new()),
        });
        let slot = CommunicationSlot::new(
            consumer,
            readback.clone(),
            dir.clone(),
            mapping.clone(),
            updates.clone(),
            residency.clone(),
        );
        Env {
            slot,
            writes,
            hits,
            dir,
            updates,
            residency,
            readback,
            index,
            manifests,
            log,
        }
    }
    fn attached(generations: usize) -> Env {
        let mut env = Env::new(generations);
        env.slot.attach_index(env.index.clone());
        env
    }
    fn batch(&self) -> Arc<Manifest> {
        Arc::clone(self.manifests.last().unwrap())
    }
    fn bind(&self, seg: u64, slot: u32, entry_id: u64, valid: bool) {
        let binding = &self.dir.0[&SegmentId(seg)];
        binding.assignment.bind(Slot5(slot), entry(entry_id)).unwrap();
        if valid {
            binding.states.set_filled(Slot5(slot), 1).unwrap();
        }
        self.residency
            .lock()
            .unwrap()
            .insert(
                EntryId(entry_id),
                SlotRef {
                    segment: SegmentId(seg),
                    slot: Slot5(slot),
                },
                Stamp(1),
            );
    }
    fn seed(&self, entry_id: u64, meta: &[u8]) {
        let payload = PayloadRef {
            vector: &[0u8; 4],
            meta,
        };
        self.index.inner.write(EntryId(entry_id), payload).unwrap();
    }
    fn push_write(&mut self, entry_id: u64, seg: u64, slot: u32) {
        let w = EmbeddingWrite {
            entry: EntryId(entry_id),
            segment: SegmentId(seg),
            slot: Slot5(slot),
        };
        assert!(self.writes.push(w).is_ok());
    }
    fn push_hit(&mut self, addr: u64, score: f32) {
        let hit = KernelHit {
            addr: Addr(addr),
            score,
        };
        assert!(self.hits.push(hit).is_ok());
    }
    fn pend(&self, entry_id: u64) {
        self.updates.lock().unwrap().pending.insert(EntryId(entry_id));
    }
    fn fail_readback(&self, addr: u64) {
        self.readback.failing.lock().unwrap().insert(addr);
    }
}
#[test]
fn error_display() {
    assert_eq!(CompletionError::NoIndexAttached.to_string(), "no index attached");
    assert_eq!(CompletionError::StaleMapping.to_string(), "stale mapping");
}
#[test]
fn without_index_nothing_happens() {
    let mut env = Env::new(1);
    env.bind(1, 0, 1, true);
    env.push_write(1, 1, 0);
    env.push_hit(seg1(0), 0.5);
    env.pend(1);
    let batch = env.batch();
    assert_eq!(env.slot.try_complete(& batch), Err(CompletionError::NoIndexAttached));
    assert_eq!(env.slot.complete(& batch), Err(CompletionError::NoIndexAttached));
    assert_eq!(env.slot.consumer.write_depth(), 1);
    assert_eq!(env.slot.consumer.hit_depth(), 1);
    assert!(events(& env.log).is_empty());
    env.slot.attach_index(env.index.clone());
    env.slot.detach_index();
    assert_eq!(env.slot.complete(& batch), Err(CompletionError::NoIndexAttached));
}
#[test]
fn writes_are_read_back_and_upserted() {
    let mut env = Env::attached(1);
    env.bind(1, 0, 1, true);
    env.bind(2, 1, 2, true);
    env.seed(1, b"meta");
    env.push_write(1, 1, 0);
    env.push_write(2, 2, 1);
    let batch = env.batch();
    let out = env.slot.complete(&batch).unwrap();
    let r = &out.report;
    assert_eq!((r.writes_applied, r.writes_skipped, r.writes_failed), (2, 0, 0));
    assert_eq!(r.writes_deferred, 0);
    assert_eq!(r.progress_bytes, 2 * SLOT_BYTES);
    assert_eq!(r.write_queue_depth, 2);
    assert!(! r.retried_after_stale_mapping);
    assert_eq!(r.reclaimed, 0);
    let p1 = env.index.read(EntryId(1));
    assert_eq!(p1.vector.len(), SLOT_BYTES as usize);
    assert!(p1.vector.iter().all(|& b | b == fill_for(Addr(seg1(0)))));
    assert_eq!(&* p1.meta, b"meta");
    let p2 = env.index.read(EntryId(2));
    assert!(p2.vector.iter().all(|& b | b == fill_for(Addr(SEG2_BASE + SLOT_BYTES))));
    for id in [1u64, 2] {
        let stamp = env.index.stamp(EntryId(id)).unwrap();
        let known = env.residency.lock().unwrap().get(EntryId(id)).unwrap().stamp;
        assert_eq!(known, stamp);
    }
    assert_eq!(
        events(& env.log), vec!["snapshot".to_string(), read_event(seg1(0)),
        read_event(SEG2_BASE + SLOT_BYTES), "apply".to_string(),]
    );
}
#[test]
fn assignment_decides_the_entry() {
    let mut env = Env::attached(1);
    env.bind(1, 0, 1, true);
    env.push_write(99, 1, 0);
    let batch = env.batch();
    let out = env.slot.complete(&batch).unwrap();
    assert_eq!(out.report.writes_applied, 1);
    assert!(env.index.stamp(EntryId(1)).is_some());
    assert!(env.index.stamp(EntryId(99)).is_none());
}
#[test]
fn rewrite_of_not_resident_entry_is_ignored() {
    let mut env = Env::attached(1);
    env.bind(1, 0, 1, true);
    env.residency.lock().unwrap().remove(EntryId(1));
    env.push_write(1, 1, 0);
    let batch = env.batch();
    let out = env.slot.complete(&batch).unwrap();
    assert_eq!(out.report.writes_applied, 1);
    assert!(env.residency.lock().unwrap().get(EntryId(1)).is_none());
}
#[test]
fn unbound_and_out_of_range_slots_are_skipped() {
    let mut env = Env::attached(1);
    env.bind(1, 0, 1, true);
    env.push_write(5, 1, 3);
    env.push_write(6, 1, 99);
    env.push_write(1, 1, 0);
    let batch = env.batch();
    let out = env.slot.complete(&batch).unwrap();
    let r = &out.report;
    assert_eq!((r.writes_applied, r.writes_skipped, r.writes_deferred), (1, 2, 0));
    assert_eq!(r.progress_bytes, 2 + SLOT_BYTES);
    assert_eq!(events(& env.log).iter().filter(| e | e.starts_with("read")).count(), 1);
}
#[test]
fn unmapped_address_defers_rest_but_hits_and_updates_still_run() {
    let mut env = Env::attached(1);
    env.bind(1, 0, 1, true);
    env.bind(1, 1, 2, true);
    env.seed(1, b"");
    env.push_write(1, 1, 0);
    env.push_write(9, 77, 0);
    env.push_write(2, 1, 1);
    env.push_hit(seg1(0) + 16, 0.25);
    env.pend(2);
    let batch = env.batch();
    let out = env.slot.complete(&batch).unwrap();
    let r = &out.report;
    assert_eq!(r.writes_applied, 1);
    assert_eq!(r.writes_deferred, 2);
    assert_eq!(env.slot.deferred_len(), 2);
    assert_eq!(
        r.defer_reason, Some(DeferReason::UnmappedAddress(EmbeddingWrite { entry :
        EntryId(9), segment : SegmentId(77), slot : Slot5(0), }))
    );
    assert_eq!(out.hits.len(), 1);
    assert_eq!(out.hits[0].entry, entry(1));
    assert_eq!(out.hits[0].score, 0.25);
    assert_eq!(r.updates_applied, 1);
    let out = env.slot.complete(&batch).unwrap();
    assert_eq!(out.report.writes_applied, 0);
    assert_eq!(out.report.writes_deferred, 2);
    assert_eq!(env.slot.deferred_len(), 2);
    assert!(env.index.stamp(EntryId(2)).is_none());
}
#[test]
fn readback_error_defers_and_recovers_in_order() {
    let mut env = Env::attached(1);
    env.bind(1, 0, 1, true);
    env.bind(1, 1, 2, true);
    env.bind(1, 2, 3, true);
    env.fail_readback(seg1(0));
    env.push_write(1, 1, 0);
    env.push_write(2, 1, 1);
    let batch = env.batch();
    let out = env.slot.complete(&batch).unwrap();
    assert_eq!(out.report.writes_applied, 0);
    assert_eq!(out.report.writes_deferred, 2);
    assert_eq!(
        out.report.defer_reason, Some(DeferReason::Readback(ReadbackError("boom"
        .into())))
    );
    assert!(env.index.stamp(EntryId(1)).is_none());
    env.readback.failing.lock().unwrap().clear();
    env.log.lock().unwrap().clear();
    env.push_write(3, 1, 2);
    let out = env.slot.complete(&batch).unwrap();
    assert_eq!(out.report.writes_applied, 3);
    assert_eq!(out.report.writes_deferred, 0);
    assert_eq!(out.report.defer_reason, None);
    assert_eq!(env.slot.deferred_len(), 0);
    let reads: Vec<String> = events(&env.log)
        .into_iter()
        .filter(|e| e.starts_with("read"))
        .collect();
    assert_eq!(
        reads, vec![read_event(seg1(0)), read_event(seg1(1)), read_event(seg1(2))]
    );
}
#[test]
fn failed_index_write_consumes_the_write() {
    let mut env = Env::attached(1);
    env.bind(1, 0, 1, true);
    env.bind(1, 1, 2, true);
    env.index.failing.lock().unwrap().insert(EntryId(1));
    env.push_write(1, 1, 0);
    env.push_write(2, 1, 1);
    let batch = env.batch();
    let out = env.slot.complete(&batch).unwrap();
    let r = &out.report;
    assert_eq!((r.writes_applied, r.writes_failed, r.writes_deferred), (1, 1, 0));
    assert_eq!(r.progress_bytes, 2 * SLOT_BYTES);
    assert_eq!(env.slot.deferred_len(), 0);
    let known = env.residency.lock().unwrap().get(EntryId(1)).unwrap().stamp;
    assert_eq!(known, Stamp(1));
}
#[test]
fn every_write_is_accounted_for() {
    let mut env = Env::attached(1);
    env.bind(1, 0, 1, true);
    env.bind(1, 1, 2, true);
    env.bind(1, 2, 3, true);
    env.index.failing.lock().unwrap().insert(EntryId(2));
    env.push_write(1, 1, 0);
    env.push_write(0, 1, 3);
    env.push_write(2, 1, 1);
    env.push_write(8, 55, 0);
    env.push_write(3, 1, 2);
    let batch = env.batch();
    let r = env.slot.complete(&batch).unwrap().report;
    assert_eq!(r.writes_applied, 1);
    assert_eq!(r.writes_skipped, 1);
    assert_eq!(r.writes_failed, 1);
    assert_eq!(r.writes_deferred, 2);
    assert_eq!(
        r.writes_applied + r.writes_skipped + r.writes_failed + r.writes_deferred, 5
    );
    assert_eq!(env.slot.deferred_len(), r.writes_deferred);
}
#[test]
fn hits_are_filtered_flagged_and_keep_order() {
    let mut env = Env::attached(1);
    env.bind(1, 0, 1, true);
    env.bind(1, 1, 2, true);
    env.bind(1, 2, 3, true);
    env.seed(1, b"");
    env.seed(3, b"");
    env.pend(3);
    env.push_hit(seg1(2) + 16, 0.3);
    env.push_hit(seg1(0) + 16, 0.9);
    env.push_hit(seg1(1) + 16, 0.8);
    env.push_hit(0x90000, 0.7);
    env.push_hit(seg1(2) + 16, 0.1);
    let batch = env.batch();
    let out = env.slot.complete(&batch).unwrap();
    let seen: Vec<(EntryRef, f32, bool)> = out
        .hits
        .iter()
        .map(|h| (h.entry, h.score, h.stale))
        .collect();
    assert_eq!(
        seen, vec![(entry(3), 0.3, true), (entry(1), 0.9, false), (entry(3), 0.1, true)]
    );
    let r = &out.report;
    assert_eq!(r.hits_resolved, 3);
    assert_eq!(r.hits_discarded, 2);
    assert_eq!(r.hits_stale, 2);
    assert_eq!(r.hit_queue_depth, 5);
    assert_eq!(r.update_queue_depth, 1);
    assert_eq!(r.updates_applied, 1);
}
#[test]
fn empty_queues_still_apply_updates() {
    let mut env = Env::attached(1);
    env.pend(4);
    env.pend(5);
    let batch = env.batch();
    let out = env.slot.complete(&batch).unwrap();
    assert!(out.hits.is_empty());
    let expected = SlotReport {
        update_queue_depth: 2,
        updates_applied: 2,
        ..SlotReport::default()
    };
    assert_eq!(out.report, expected);
    assert_eq!(events(& env.log), vec!["snapshot".to_string(), "apply".to_string()]);
}
#[test]
fn stale_mapping_is_reported_without_side_effects() {
    let mut env = Env::attached(2);
    env.bind(1, 0, 1, true);
    env.push_write(1, 1, 0);
    env.push_hit(seg1(0) + 16, 0.5);
    env.pend(1);
    let older = Arc::clone(&env.manifests[0]);
    assert_eq!(env.slot.try_complete(& older), Err(CompletionError::StaleMapping));
    assert_eq!(events(& env.log), vec!["snapshot".to_string()]);
    assert_eq!(env.slot.consumer.write_depth(), 1);
    assert_eq!(env.slot.consumer.hit_depth(), 1);
    assert_eq!(env.updates.lock().unwrap().depth(), 1);
    assert!(env.index.stamp(EntryId(1)).is_none());
}
#[test]
fn complete_repeats_under_structure_lock() {
    let mut env = Env::attached(2);
    env.bind(1, 0, 1, true);
    env.seed(1, b"");
    env.push_write(1, 1, 0);
    env.push_hit(seg1(0) + 16, 0.5);
    let older = Arc::clone(&env.manifests[0]);
    let out = env.slot.complete(&older).unwrap();
    let r = &out.report;
    assert!(r.retried_after_stale_mapping);
    assert_eq!(r.reclaimed, RECLAIMED);
    assert_eq!(r.writes_applied, 1);
    assert_eq!(out.hits.len(), 1);
    assert!(out.hits[0].stale);
    assert_eq!(r.hits_stale, 1);
    assert_eq!(
        events(& env.log), vec!["snapshot".to_string(), "lock_structure".to_string(),
        "snapshot".to_string(), read_event(seg1(0)), "apply".to_string(),
        "unlock_structure".to_string(), "reclaim".to_string(),]
    );
}
#[test]
fn current_batch_is_not_stale_and_does_not_reclaim() {
    let mut env = Env::attached(2);
    env.bind(1, 0, 1, true);
    env.seed(1, b"");
    env.push_hit(seg1(0) + 16, 0.5);
    let batch = env.batch();
    let out = env.slot.complete(&batch).unwrap();
    assert!(! out.report.retried_after_stale_mapping);
    assert_eq!(out.report.reclaimed, 0);
    assert!(! out.hits[0].stale);
    assert!(! events(& env.log).contains(& "lock_structure".to_string()));
}
#[test]
fn poisoned_locks_are_taken_over() {
    let mut env = Env::attached(1);
    env.bind(1, 0, 1, true);
    env.push_write(1, 1, 0);
    let updates = Arc::clone(&env.updates);
    let residency = Arc::clone(&env.residency);
    let _ = std::thread::spawn(move || {
            let _u = updates.lock().unwrap();
            let _r = residency.lock().unwrap();
            panic!("poison");
        })
        .join();
    assert!(env.updates.is_poisoned());
    assert!(env.residency.is_poisoned());
    let batch = env.batch();
    let out = env.slot.complete(&batch).unwrap();
    assert_eq!(out.report.writes_applied, 1);
}
#[test]
fn slot_can_move_between_threads() {
    let mut env = Env::attached(1);
    env.bind(1, 0, 1, true);
    env.push_write(1, 1, 0);
    let batch = env.batch();
    let mut slot = env.slot;
    let out = std::thread::spawn(move || slot.complete(&batch).unwrap()).join().unwrap();
    assert_eq!(out.report.writes_applied, 1);
}
