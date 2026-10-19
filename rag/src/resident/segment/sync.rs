//! Segment change events and publication ordered behind the copy that fills the segment.

use std::collections::VecDeque;
use std::sync::Arc;

use parking_lot::Mutex;
use tokio::sync::broadcast;

use super::error::SegmentError;
use super::{Fence, Manifest, SegmentStore};
use crate::resident::subengine::{ExecContext, LaneJob};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SegmentEventKind {
    Added,
    Changed,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SegmentEvent {
    pub kind: SegmentEventKind,
    pub segment_id: u64,
    pub segment_gen: u64,
    pub manifest_gen: u64,
}

#[derive(Clone)]
pub struct SegmentBus {
    tx: broadcast::Sender<SegmentEvent>,
}

impl SegmentBus {
    pub fn new(capacity: usize) -> Self {
        Self { tx: broadcast::channel(capacity).0 }
    }

    pub fn subscribe(&self) -> broadcast::Receiver<SegmentEvent> {
        self.tx.subscribe()
    }

    pub fn emit(&self, event: SegmentEvent) {
        tracing::debug!(?event, "segment event");
        let _ = self.tx.send(event);
    }
}

struct Staged {
    manifest: Manifest,
    ready: Arc<dyn Fence>,
}

/// Publishes staged manifests in order, each only after its copy event has completed.
pub struct Publisher {
    store: Arc<SegmentStore>,
    bus: SegmentBus,
    staged: Mutex<VecDeque<Staged>>,
}

impl Publisher {
    pub fn new(store: Arc<SegmentStore>, bus: SegmentBus) -> Self {
        Self { store, bus, staged: Mutex::new(VecDeque::new()) }
    }

    pub fn pending(&self) -> usize {
        self.staged.lock().len()
    }

    pub fn stage(&self, manifest: Manifest, ready: Arc<dyn Fence>) {
        self.staged.lock().push_back(Staged { manifest, ready });
    }

    /// Enqueues `copy` on the copy lane and stages `manifest` behind its completion.
    pub fn stage_copy(
        &self,
        lane: &dyn ExecContext,
        copy: LaneJob,
        manifest: Manifest,
    ) -> Result<(), SegmentError> {
        lane.submit(copy).map_err(|source| SegmentError::SyncFailed { source })?;
        let done = lane.record_event().map_err(|source| SegmentError::SyncFailed { source })?;
        self.stage(manifest, Arc::new(done));
        Ok(())
    }

    /// Publishes every staged manifest whose copy has completed; returns how many.
    pub fn poll(&self) -> Result<usize, SegmentError> {
        let mut published = 0;
        loop {
            let next = {
                let mut q = self.staged.lock();
                match q.front() {
                    Some(s) if s.ready.is_complete() => q.pop_front(),
                    _ => None,
                }
            };
            let Some(staged) = next else {
                return Ok(published);
            };
            let manifest_gen = staged.manifest.generation();
            let events = self.diff(&self.store.current(), &staged.manifest);
            self.store.publish(staged.manifest)?;
            for (kind, segment_id, segment_gen) in events {
                self.bus.emit(SegmentEvent { kind, segment_id, segment_gen, manifest_gen });
            }
            published += 1;
        }
    }

    fn diff(&self, prev: &Manifest, next: &Manifest) -> Vec<(SegmentEventKind, u64, u64)> {
        next.segments()
            .iter()
            .filter_map(|s| {
                match prev.segments().iter().find(|p| p.id() == s.id()) {
                    None => Some((SegmentEventKind::Added, s.id(), s.view().gen)),
                    Some(p) if p.view().gen != s.view().gen => {
                        Some((SegmentEventKind::Changed, s.id(), s.view().gen))
                    }
                    Some(_) => None,
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::super::mock::ManualFence;
    use super::super::testutil::{region, segment};
    use super::*;
    use crate::resident::subengine::{LaneClass, LaneConfig, RecordingLane};

    fn setup() -> (Arc<SegmentStore>, Publisher, broadcast::Receiver<SegmentEvent>) {
        let store = Arc::new(SegmentStore::new());
        let bus = SegmentBus::new(16);
        let rx = bus.subscribe();
        (Arc::clone(&store), Publisher::new(store, bus), rx)
    }

    #[test]
    fn publication_waits_for_copy_completion() {
        let (store, publisher, mut rx) = setup();
        let seg = segment(&store, 1, 10, 0, 2);
        let fence = ManualFence::new();
        publisher.stage(Manifest::new(1, vec![seg]), fence.clone());

        assert_eq!(publisher.poll().unwrap(), 0);
        assert_eq!(store.current().generation(), 0);
        assert!(rx.try_recv().is_err());

        fence.complete();
        assert_eq!(publisher.poll().unwrap(), 1);
        assert_eq!(store.current().generation(), 1);
        let ev = rx.try_recv().unwrap();
        assert_eq!(
            ev,
            SegmentEvent { kind: SegmentEventKind::Added, segment_id: 1, segment_gen: 10, manifest_gen: 1 }
        );
    }

    #[test]
    fn rebuilt_segment_reports_changed_with_new_generation() {
        let (store, publisher, mut rx) = setup();
        let done = ManualFence::new();
        done.complete();
        publisher.stage(Manifest::new(1, vec![segment(&store, 1, 10, 0, 2)]), done.clone());
        publisher.stage(Manifest::new(2, vec![segment(&store, 1, 11, 1 << 16, 2)]), done);
        assert_eq!(publisher.poll().unwrap(), 2);
        rx.try_recv().unwrap();
        let ev = rx.try_recv().unwrap();
        assert_eq!((ev.kind, ev.segment_gen, ev.manifest_gen), (SegmentEventKind::Changed, 11, 2));
    }

    #[test]
    fn staged_manifests_publish_in_order() {
        let (store, publisher, _rx) = setup();
        let (slow, fast) = (ManualFence::new(), ManualFence::new());
        publisher.stage(Manifest::new(1, vec![]), slow.clone());
        publisher.stage(Manifest::new(2, vec![]), fast.clone());
        fast.complete();
        assert_eq!(publisher.poll().unwrap(), 0);
        slow.complete();
        assert_eq!(publisher.poll().unwrap(), 2);
        assert_eq!(store.current().generation(), 2);
    }

    #[test]
    fn stale_stage_is_rejected_at_publication() {
        let (store, publisher, _rx) = setup();
        store.publish(Manifest::new(5, vec![])).unwrap();
        let done = ManualFence::new();
        done.complete();
        publisher.stage(Manifest::new(3, vec![]), done);
        assert!(matches!(publisher.poll(), Err(SegmentError::StaleGeneration { gen: 3 })));
        assert_eq!(publisher.pending(), 0);
        assert_eq!(store.current().generation(), 5);
    }

    #[test]
    fn copy_is_submitted_before_the_event_is_recorded() {
        let (store, publisher, _rx) = setup();
        let lane = RecordingLane::new(LaneClass::Copy, LaneConfig::new(region(0, 4096)));
        publisher
            .stage_copy(&lane, Box::new(|| Ok(())), Manifest::new(1, vec![segment(&store, 1, 10, 0, 1)]))
            .unwrap();
        assert_eq!(lane.calls(), vec!["submit", "record"]);
        assert_eq!(publisher.poll().unwrap(), 1);
    }
}
