use super::fixture::*;
use super::*;
fn build_with(
    cfg: GpuManagerConfig,
    index: Arc<dyn LazyIndex>,
) -> Result<GpuManager, GpuManagerError> {
    GpuManager::new(
        FakeServices::new(1 << 20) as Arc<dyn GpuServices>,
        index,
        Arc::new(ScopeResolverRegistry::with_defaults()),
        None,
        LoadedQueue::default(),
        cfg,
    )
}
fn invalid(cfg: GpuManagerConfig) -> GpuManagerError {
    build_with(cfg, Arc::new(MissingIndex))
        .err()
        .expect("configuration must be rejected")
}
#[test]
fn zero_dimension_is_rejected() {
    let cfg = GpuManagerConfig {
        dim: 0,
        ..config()
    };
    assert!(matches!(invalid(cfg), GpuManagerError::InvalidConfig(_)));
}
#[test]
fn unsupported_dtype_is_rejected() {
    let cfg = GpuManagerConfig {
        dtype: DType::U8,
        ..config()
    };
    assert!(matches!(invalid(cfg), GpuManagerError::InvalidConfig(_)));
}
#[test]
fn segment_shorter_than_a_slot_is_rejected() {
    let cfg = GpuManagerConfig {
        segment_bytes: SLOT_BYTES - 1,
        ..config()
    };
    assert!(matches!(invalid(cfg), GpuManagerError::InvalidConfig(_)));
}
#[test]
fn oversized_slot_is_rejected() {
    let cfg = GpuManagerConfig {
        class: SlotClass::custom(65_535).unwrap(),
        dim: u32::MAX,
        ..config()
    };
    assert!(matches!(invalid(cfg), GpuManagerError::InvalidConfig(_)));
}
#[test]
fn segment_of_exactly_one_slot_is_accepted() {
    let cfg = GpuManagerConfig {
        segment_bytes: SLOT_BYTES,
        ..config()
    };
    assert!(build_with(cfg, Arc::new(MissingIndex)).is_ok());
}
#[test]
fn construction_neither_loads_nor_allocates() {
    let services = FakeServices::new(1 << 20);
    let manager = GpuManager::new(
            Arc::clone(&services) as Arc<dyn GpuServices>,
            Arc::new(MissingIndex),
            Arc::new(ScopeResolverRegistry::with_defaults()),
            None,
            LoadedQueue::default(),
            config(),
        )
        .unwrap();
    assert_eq!(services.arena().stats().live_segments, 0);
    assert_eq!(manager.segment_count(), 0);
}
#[test]
fn empty_manager_reports_nothing() {
    let f = Fixture::new();
    let m = &f.manager;
    assert!(m.segment_ids().is_empty());
    assert_eq!(
        m.stats(), GpuIndexStats { segments : 0, resident_bytes : 0, pending_updates : 0
        }
    );
    assert!(m.slot_of(EntryId(1)).is_none());
    assert!(m.global_slot_of(EntryId(1)).is_none());
    assert!(m.entry_at_global_slot(0).is_none());
    assert!(! m.signal_pending());
}
#[test]
fn global_slots_and_entries_are_exact_inverses() {
    let f = Fixture::new();
    f.resident(12);
    let mut seen = Vec::new();
    for id in 1..=12u64 {
        let global = f.manager.global_slot_of(EntryId(id)).unwrap();
        assert_eq!(
            f.manager.entry_at_global_slot(global), Some(EntryRef { entry : EntryId(id),
            section : 0 })
        );
        let slot = f.manager.slot_of(EntryId(id)).unwrap();
        assert!(f.manager.segment_ids().contains(& slot.segment));
        seen.push(global);
    }
    seen.sort_unstable();
    seen.dedup();
    assert_eq!(seen.len(), 12);
    assert_eq!(f.manager.entry_at_global_slot(1_000), None);
}
#[test]
fn stats_follow_the_published_segments() {
    let f = Fixture::new();
    f.resident(3);
    let stats = f.manager.stats();
    assert_eq!(stats.segments, 1);
    assert_eq!(stats.pending_updates, 0);
    assert!(stats.resident_bytes >= SEGMENT_BYTES);
    assert_eq!(stats.resident_bytes, f.manager.resident_bytes());
    assert_eq!(f.manager.segment_count(), 1);
    assert_eq!(f.manager.pending_updates(), 0);
}
