use std::sync::atomic::Ordering;
use super::super::fixture::*;
use super::*;
fn u64_at(bytes: &[u8], at: usize) -> u64 {
    u64::from_ne_bytes(bytes[at..at + 8].try_into().unwrap())
}
fn u32_at(bytes: &[u8], at: usize) -> u32 {
    u32::from_ne_bytes(bytes[at..at + 4].try_into().unwrap())
}
#[test]
fn empty_manager_writes_nothing() {
    let f = Fixture::new();
    let mut lane = lane();
    let table = f.manager.sync_to_vram(&mut lane).unwrap();
    assert_eq!(table, VramTable { segments_addr : 0, segment_count : 0, version : 0 });
    assert!(lock(& f.services.tables).is_empty());
}
#[test]
fn table_holds_one_record_per_segment() {
    let f = Fixture::new();
    f.resident(12);
    let mut lane = lane();
    let table = f.manager.sync_to_vram(&mut lane).unwrap();
    assert_eq!(table.segment_count, 2);
    assert_eq!(table.version, 2);
    assert_eq!(table.segments_addr, 0x2000_0000 + 0x1000);
    let tables = lock(&f.services.tables);
    assert_eq!(tables.len(), 1);
    let bytes = &tables[0];
    assert_eq!(bytes.len(), 80);
    let base = lock(&f.manager.inner.state).records[0].device_base.0;
    assert_eq!(u64_at(bytes, 0), base);
    assert_eq!(u64_at(bytes, 8), 0);
    assert_eq!(u64_at(bytes, 16), SLOT_BYTES);
    assert_eq!(u32_at(bytes, 24), 0);
    assert_eq!(u32_at(bytes, 28), SLOTS_PER_SEGMENT);
    assert_eq!(u32_at(bytes, 32), MAX_TOKENS);
    assert_eq!(u32_at(bytes, 36), 0);
    assert_eq!(u32_at(bytes, 40 + 24), SLOTS_PER_SEGMENT);
    assert_eq!(u32_at(bytes, 40 + 28), SLOTS_PER_SEGMENT);
    assert_ne!(u64_at(bytes, 40), base);
}
#[test]
fn unchanged_state_is_not_written_again() {
    let f = Fixture::new();
    f.resident(3);
    let mut lane = lane();
    let first = f.manager.sync_to_vram(&mut lane).unwrap();
    let second = f.manager.sync_to_vram(&mut lane).unwrap();
    assert_eq!(first, second);
    assert_eq!(lock(& f.services.tables).len(), 1);
    f.put(50, 2, 1.0);
    f.manager.ensure(&set(&[50])).unwrap();
    f.apply();
    let third = f.manager.sync_to_vram(&mut lane).unwrap();
    assert_eq!(third, first);
    assert_eq!(lock(& f.services.tables).len(), 1);
}
#[test]
fn a_changed_segment_set_is_written() {
    let f = Fixture::new();
    f.resident(3);
    let mut lane = lane();
    let first = f.manager.sync_to_vram(&mut lane).unwrap();
    f.manager.release_segment(SegmentId(0)).unwrap();
    let second = f.manager.sync_to_vram(&mut lane).unwrap();
    assert!(second.version > first.version);
    assert_eq!(second.segment_count, 0);
    assert_eq!(second.segments_addr, 0);
}
#[test]
fn failed_write_keeps_the_cache_and_is_retried() {
    let f = Fixture::new();
    f.resident(3);
    let mut lane = lane();
    f.services.fail_kernel_table.store(true, Ordering::SeqCst);
    let err = f.manager.sync_to_vram(&mut lane).unwrap_err();
    assert_eq!(err, GpuManagerError::VramSync("kernel table write failed".to_string()));
    f.services.fail_kernel_table.store(false, Ordering::SeqCst);
    let table = f.manager.sync_to_vram(&mut lane).unwrap();
    assert_eq!(table.segment_count, 1);
    assert_eq!(lock(& f.services.tables).len(), 1);
}
