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
