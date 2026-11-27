pub mod device_transfer;
pub mod hit_queues;
pub mod hit_resolution;
use std::fmt;
use std::sync::{Arc, Mutex};
use log::{debug, warn};
use self::hit_queues::{EmbeddingWrite, QueueConsumer};
use self::hit_resolution::{resolve_hits, ResolvedHit, SegmentDirectory};
use crate::resident::_d::_c::_b::_a::_9::_8::_7::a::manifest::Manifest;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::a::reconcile::ResidencyDirectory;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::a::kernel_layout::Addr;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::a::index_contract::{
    EntryId, Index, PayloadRef,
};
const LOG_TARGET3: &str = "communication_slot";
/// Replaceable device readback. Reads `dst.len()` bytes starting at the device address.
pub trait DeviceReadback: Send + Sync {
    fn read_back(&self, addr: Addr, dst: &mut [u8]) -> Result<(), ReadbackError>;
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReadbackError(pub String);
impl fmt::Display for ReadbackError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "readback failed: {}", self.0)
    }
}
impl std::error::Error for ReadbackError {}
/// Mapping state: current manifest plus the one before it.
#[derive(Clone)]
pub struct MappingSnapshot {
    pub current: Arc<Manifest>,
    pub previous: Option<Arc<Manifest>>,
}
/// Marker. Holding the guard blocks mapping and structure changes; dropping releases it.
pub trait StructureHold {}
/// Implemented by the manager.
pub trait MappingSource: Send + Sync {
    fn snapshot(&self) -> MappingSnapshot;
    fn lock_structure(&self) -> Box<dyn StructureHold + '_>;
    /// Reclaims deferred-freed memory whose work is complete; returns the number of regions.
    fn reclaim_deferred(&self) -> usize;
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct UpdateOutcome {
    pub applied: usize,
}
/// Implemented by the manager; always accessed under its mutex.
pub trait UpdateApplier: Send {
    fn depth(&self) -> usize;
    /// True if a not-yet-applied update exists for this entry.
    fn is_pending(&self, entry: EntryId) -> bool;
    /// Applies all pending updates and thereby changes the mapping. The caller holds the
    /// residency lock.
    fn apply(&mut self, residency: &ResidencyDirectory) -> UpdateOutcome;
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompletionError {
    /// No index attached; no side effects.
    NoIndexAttached,
    /// The mapping has moved past the batch epoch; only returned by `try_complete`,
    /// without side effects.
    StaleMapping,
}
impl fmt::Display for CompletionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CompletionError::NoIndexAttached => f.write_str("no index attached"),
            CompletionError::StaleMapping => f.write_str("stale mapping"),
        }
    }
}
impl std::error::Error for CompletionError {}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeferReason {
    UnmappedAddress(EmbeddingWrite),
    Readback(ReadbackError),
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SlotReport {
    pub writes_applied: usize,
    pub writes_skipped: usize,
    pub writes_failed: usize,
    pub writes_deferred: usize,
    pub progress_bytes: u64,
    pub defer_reason: Option<DeferReason>,
    pub hits_resolved: usize,
    pub hits_discarded: usize,
    pub hits_stale: usize,
    pub write_queue_depth: usize,
    pub hit_queue_depth: usize,
    pub update_queue_depth: usize,
    pub updates_applied: usize,
    pub retried_after_stale_mapping: bool,
    pub reclaimed: usize,
}
#[derive(Debug, Clone, PartialEq)]
pub struct SlotOutput {
    pub hits: Vec<ResolvedHit>,
    pub report: SlotReport,
}
enum WriteStep {
    Applied(usize),
    Skipped,
    Failed(usize),
    Unmapped,
    Readback(ReadbackError),
}
fn process_write(
    w: &EmbeddingWrite,
    current: &Manifest,
    dir: &dyn SegmentDirectory,
    readback: &dyn DeviceReadback,
    index: &dyn Index,
    residency: &ResidencyDirectory,
    buf: &mut Vec<u8>,
) -> WriteStep {
    let Some(spec) = current.segments().iter().find(|s| s.id() == w.segment) else {
        return WriteStep::Unmapped;
    };
    let Some(binding) = dir.binding(spec.id()) else {
        return WriteStep::Unmapped;
    };
    let Some(entry_ref) = binding.assignment.lookup(w.slot) else {
        return WriteStep::Skipped;
    };
    let (Ok(addr), Ok(range)) = (
        binding.assignment.device_addr(w.slot),
        binding.assignment.byte_range(w.slot),
    ) else {
        return WriteStep::Unmapped;
    };
    let Some(len) = range
        .end
        .checked_sub(range.start)
        .and_then(|l| usize::try_from(l).ok()) else {
        return WriteStep::Unmapped;
    };
    if len == 0 {
        return WriteStep::Skipped;
    }
    let entry = entry_ref.entry;
    if entry != w.entry {
        debug!(
            target : LOG_TARGET3,
            "write entry {:?} differs from assignment {:?}; assignment wins", w.entry,
            entry
        );
    }
    buf.clear();
    buf.resize(len, 0);
    if let Err(e) = readback.read_back(addr, &mut buf[..len]) {
        return WriteStep::Readback(e);
    }
    let current_payload = index.read(entry);
    let payload = PayloadRef {
        vector: &buf[..len],
        meta: &current_payload.meta,
    };
    match index.write(entry, payload) {
        Ok(stamp) => {
            residency.set_known_stamp(entry, stamp);
            WriteStep::Applied(len)
        }
        Err(e) => {
            warn!(
                target : LOG_TARGET3, "index write failed entry={:?} slot={:?}: {}",
                entry, w.slot, e
            );
            WriteStep::Failed(len)
        }
    }
}
/// Single-owner completion state: deferred writes and the queue consumer.
pub struct CommunicationSlot {
    consumer: QueueConsumer,
    readback: Arc<dyn DeviceReadback>,
    dir: Arc<dyn SegmentDirectory>,
    mapping: Arc<dyn MappingSource>,
    updates: Arc<Mutex<dyn UpdateApplier>>,
    residency: Arc<Mutex<ResidencyDirectory>>,
    deferred: Vec<EmbeddingWrite>,
    index: Option<Arc<dyn Index>>,
}
impl CommunicationSlot {
    pub fn new(
        consumer: QueueConsumer,
        readback: Arc<dyn DeviceReadback>,
        dir: Arc<dyn SegmentDirectory>,
        mapping: Arc<dyn MappingSource>,
        updates: Arc<Mutex<dyn UpdateApplier>>,
        residency: Arc<Mutex<ResidencyDirectory>>,
    ) -> CommunicationSlot {
        CommunicationSlot {
            consumer,
            readback,
            dir,
            mapping,
            updates,
            residency,
            deferred: Vec::new(),
            index: None,
        }
    }
    pub fn attach_index(&mut self, index: Arc<dyn Index>) {
        self.index = Some(index);
    }
    pub fn detach_index(&mut self) {
        self.index = None;
    }
    /// Writes held over for the next slot.
    pub fn deferred_len(&self) -> usize {
        self.deferred.len()
    }
    /// Full completion: attempt against the batch epoch; on stale mapping repeat under the
    /// Structure-Lock.
    pub fn complete(&mut self, batch: &Manifest) -> Result<SlotOutput, CompletionError> {
        match self.try_complete(batch) {
            Err(CompletionError::StaleMapping) => {}
            other => return other,
        }
        warn!(target : LOG_TARGET3, "stale Mapping");
        let index = self.index.clone().ok_or(CompletionError::NoIndexAttached)?;
        let mapping = Arc::clone(&self.mapping);
        let updates = Arc::clone(&self.updates);
        let residency = Arc::clone(&self.residency);
        let structure = mapping.lock_structure();
        let mut updates_guard = lock(&updates);
        let residency_guard = lock(&residency);
        let snap = mapping.snapshot();
        let mut out = self
            .run_pass(batch, &snap, &*index, &mut *updates_guard, &residency_guard);
        drop(residency_guard);
        drop(updates_guard);
        drop(structure);
        out.report.reclaimed = mapping.reclaim_deferred();
        out.report.retried_after_stale_mapping = true;
        Ok(out)
    }
    /// Single attempt against the batch epoch; `Err(StaleMapping)` without any side effect.
    pub fn try_complete(
        &mut self,
        batch: &Manifest,
    ) -> Result<SlotOutput, CompletionError> {
        let index = self.index.clone().ok_or(CompletionError::NoIndexAttached)?;
        let mapping = Arc::clone(&self.mapping);
        let updates = Arc::clone(&self.updates);
        let residency = Arc::clone(&self.residency);
        let mut updates_guard = lock(&updates);
        let residency_guard = lock(&residency);
        let snap = mapping.snapshot();
        if snap.current.generation() != batch.generation() {
            return Err(CompletionError::StaleMapping);
        }
        Ok(self.run_pass(batch, &snap, &*index, &mut *updates_guard, &residency_guard))
    }
    fn run_pass(
        &mut self,
        batch: &Manifest,
        snap: &MappingSnapshot,
        index: &dyn Index,
        updates: &mut dyn UpdateApplier,
        residency: &ResidencyDirectory,
    ) -> SlotOutput {
        let mut report = SlotReport {
            write_queue_depth: self.consumer.write_depth(),
            hit_queue_depth: self.consumer.hit_depth(),
            ..SlotReport::default()
        };
        let drained = self.consumer.drain();
        let mut work = std::mem::take(&mut self.deferred);
        work.extend(drained.writes);
        let mut buf: Vec<u8> = Vec::new();
        let mut remaining: Vec<EmbeddingWrite> = Vec::new();
        for (i, w) in work.iter().enumerate() {
            let step = process_write(
                w,
                &snap.current,
                &*self.dir,
                &*self.readback,
                index,
                residency,
                &mut buf,
            );
            match step {
                WriteStep::Applied(len) => {
                    report.writes_applied += 1;
                    report.progress_bytes += len.max(1) as u64;
                }
                WriteStep::Skipped => {
                    report.writes_skipped += 1;
                    report.progress_bytes += 1;
                }
                WriteStep::Failed(len) => {
                    report.writes_failed += 1;
                    report.progress_bytes += len.max(1) as u64;
                }
                WriteStep::Unmapped => {
                    report.defer_reason = Some(DeferReason::UnmappedAddress(*w));
                    remaining.extend_from_slice(&work[i..]);
                    break;
                }
                WriteStep::Readback(e) => {
                    report.defer_reason = Some(DeferReason::Readback(e));
                    remaining.extend_from_slice(&work[i..]);
                    break;
                }
            }
        }
        report.writes_deferred = remaining.len();
        self.deferred = remaining;
        let resolution = resolve_hits(
            batch,
            &snap.current,
            snap.previous.as_deref(),
            &*self.dir,
            &drained.hits,
        );
        report.hits_discarded = resolution.discarded;
        let mut hits: Vec<ResolvedHit> = Vec::with_capacity(resolution.hits.len());
        for mut hit in resolution.hits {
            if index.stamp(hit.entry.entry).is_none() {
                report.hits_discarded += 1;
                continue;
            }
            if updates.is_pending(hit.entry.entry) {
                hit.stale = true;
            }
            hits.push(hit);
        }
        report.hits_resolved = hits.len();
        report.hits_stale = hits.iter().filter(|h| h.stale).count();
        report.update_queue_depth = updates.depth();
        report.updates_applied = updates.apply(residency).applied;
        debug!(
            target : LOG_TARGET3,
            "pass writes applied={} skipped={} failed={} deferred={} hits resolved={} discarded={} stale={} \
             depths write={} hit={} update={} updates_applied={}",
            report.writes_applied, report.writes_skipped, report.writes_failed, report
            .writes_deferred, report.hits_resolved, report.hits_discarded, report
            .hits_stale, report.write_queue_depth, report.hit_queue_depth, report
            .update_queue_depth, report.updates_applied
        );
        SlotOutput { hits, report }
    }
}
#[cfg(test)]
#[path = "test/communication_slot.rs"]
mod tests;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::_1::sync_util::lock;
