use super::super::fixture::*;
use super::*;

fn assert_send_sync<T: Send + Sync + 'static>() {}

#[test]
fn error_messages_match_the_contract() {
    assert_eq!(
        GpuManagerError::InvalidConfig("x".into()).to_string(),
        "ungültige GPU-Manager-Konfiguration: x"
    );
    assert_eq!(
        GpuManagerError::Index("down".into()).to_string(),
        "Index nicht verfügbar: down"
    );
    assert_eq!(GpuManagerError::UnknownSegment(7).to_string(), "unknown segment 7");
}

#[test]
fn types_are_thread_safe() {
    assert_send_sync::<GpuManagerError>();
    assert_send_sync::<LoadedQueue>();
    assert_send_sync::<LoadedSegment>();
    assert_send_sync::<EnsureReport>();
}

#[test]
fn queue_keeps_order_and_is_shared_between_clones() {
    let f = Fixture::new();
    for id in 1..=20 {
        f.put(id, 2, id as f32);
    }
    let wanted: Vec<u64> = (1..=20).collect();
    let report = f.manager.ensure(&set(&wanted)).unwrap();
    assert_eq!(report.segments_created, 3);
    let clone = f.queue.clone();
    assert_eq!(clone.len(), 3);
    assert!(!f.queue.is_empty());
    let drained = clone.drain();
    let firsts: Vec<u64> = drained.iter().map(|s| s.bindings[0].entry.entry.0).collect();
    assert_eq!(firsts, vec![1, 9, 17]);
    assert!(f.queue.is_empty());
    assert!(f.queue.drain().is_empty());
}
