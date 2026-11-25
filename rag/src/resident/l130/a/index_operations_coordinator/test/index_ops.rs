use super::*;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;
use super::super::gpu_manager::fixture::{config, set, vector, FakeServices};
use super::super::gpu_manager::scope_resolution::ScopeResolverRegistry;
use super::super::gpu_manager::GpuServices;
use l020::a::index_contract::PayloadRef;
use l090::l080::a::communication_slot::UpdateApplier;
use l020::l010::layout::DType;
use l020::l010::scopes_and_entry_sets::EntrySet;
use l060::l050::l040::a::resource_requirements::IndexId;
use l060::l050::l040::l030::a::reference_index::ReferenceIndex;
const MIB: u64 = 1 << 20;
struct Rig {
    manager: Arc<IndexManager>,
    index: Arc<ReferenceIndex>,
    services: Arc<FakeServices>,
    gpu: Arc<GpuManager>,
    queue: LoadedQueue,
    ops: IndexOperations,
}
fn assemble(manager: Arc<IndexManager>, capacity: u64) -> Rig {
    let services = FakeServices::new(capacity);
    let index = ReferenceIndex::new_shared(DType::F32);
    let queue = LoadedQueue::default();
    let gpu = Arc::new(
        GpuManager::new(
                Arc::clone(&services) as Arc<dyn GpuServices>,
                Arc::new(ManagedIndex::new(Arc::clone(&manager))),
                Arc::new(ScopeResolverRegistry::with_defaults()),
                None,
                queue.clone(),
                config(),
            )
            .unwrap(),
    );
    let ops = IndexOperations::new(
        Arc::clone(&manager),
        Arc::clone(&gpu),
        queue.clone(),
    );
    Rig {
        manager,
        index,
        services,
        gpu,
        queue,
        ops,
    }
}
fn rig(capacity: u64) -> Rig {
    let r = assemble(Arc::new(IndexManager::new(IndexId::new("test"))), capacity);
    assert!(r.manager.attach(Arc::clone(& r.index) as Arc < dyn Index >));
    r
}
impl Rig {
    fn put(&self, ids: std::ops::RangeInclusive<u64>) {
        for id in ids {
            self.index
                .write(
                    EntryId(id),
                    PayloadRef {
                        vector: &vector(2, id as f32),
                        meta: b"meta",
                    },
                )
                .unwrap();
        }
    }
    fn scope(&self, ids: &[u64]) -> Scope {
        Scope::Set(set(ids))
    }
    fn range(&self, ids: std::ops::RangeInclusive<u64>) -> EntrySet {
        set(&ids.collect::<Vec<_>>())
    }
    fn apply(&self) -> usize {
        let mut updates = self.gpu.slot_updates();
        let directory = lock(self.services.residency());
        updates.apply(&directory).applied
    }
}
#[test]
fn types_are_send_and_sync() {
    fn check<T: Send + Sync + 'static>() {}
    check::<IndexOpError>();
    check::<ManagedIndex>();
    check::<IndexOperations>();
}
#[test]
fn exhaustion_errors_keep_their_numbers() {
    let arena = GpuManagerError::Arena(SegmentArenaError::Exhausted {
        requested: 9,
        available: 3,
    });
    assert_eq!(
        IndexOpError::from(arena), IndexOpError::ArenaExhausted { requested : 9,
        available : 3 }
    );
    assert_eq!(
        IndexOpError::from(EvictionError::Exhausted { requested : 5, available : 1 }),
        IndexOpError::ArenaExhausted { requested : 5, available : 1 }
    );
    assert_eq!(
        IndexOpError::ArenaExhausted { requested : 1, available : 0 } .to_string(),
        "Arena erschöpft"
    );
}
#[test]
fn other_errors_become_transfer_with_the_cause_text() {
    let e = GpuManagerError::UnknownSegment(4);
    assert_eq!(IndexOpError::from(e.clone()), IndexOpError::Transfer(e.to_string()));
    let e = GpuManagerError::Arena(SegmentArenaError::StaleRegion);
    assert_eq!(IndexOpError::from(e.clone()), IndexOpError::Transfer(e.to_string()));
    let e = IndexError::NotFound;
    assert_eq!(IndexOpError::from(e.clone()), IndexOpError::Transfer(e.to_string()));
    let e = ManagerError::IndexUnavailable {
        index_id: IndexId::new("x"),
    };
    assert_eq!(IndexOpError::from(e.clone()), IndexOpError::Transfer(e.to_string()));
}
#[test]
fn managed_index_follows_the_manager() {
    let manager = Arc::new(IndexManager::new(IndexId::new("test")));
    let lazy = ManagedIndex::new(Arc::clone(&manager));
    assert!(lazy.get().is_err());
    let index = ReferenceIndex::new_shared(DType::F32);
    manager.attach(Arc::clone(&index) as Arc<dyn Index>);
    let got = lazy.get().unwrap();
    assert_eq!(Arc::as_ptr(& got) as * const u8, Arc::as_ptr(& index) as * const u8);
    let again = lazy.get().unwrap();
    assert_eq!(Arc::as_ptr(& got) as * const u8, Arc::as_ptr(& again) as * const u8);
}
#[test]
fn publish_pending_of_an_empty_queue_is_zero() {
    let r = rig(MIB);
    assert_eq!(r.ops.publish_pending(), Ok(0));
}
#[test]
fn publish_pending_publishes_in_queue_order_once() {
    let r = rig(MIB);
    r.put(1..=20);
    r.gpu.ensure(&r.range(1..=20)).unwrap();
    assert_eq!(r.queue.len(), 3);
    assert_eq!(r.ops.publish_pending(), Ok(3));
    assert!(r.queue.is_empty());
    let ids: Vec<u64> = r.gpu.segment_ids().iter().map(|id| id.0).collect();
    assert_eq!(ids.len(), 3);
    assert!(ids.windows(2).all(| w | w[0] < w[1]));
    assert_eq!(r.ops.publish_pending(), Ok(0));
    assert_eq!(r.gpu.segment_count(), 3);
}
#[test]
fn failing_publications_are_all_tried_and_return_the_first_error() {
    let r = rig(MIB);
    r.put(1..=20);
    r.gpu.ensure(&r.range(1..=20)).unwrap();
    r.services.sink.fail.store(true, Ordering::SeqCst);
    let err = r.ops.publish_pending().unwrap_err();
    assert!(matches!(err, IndexOpError::Transfer(_)));
    assert!(r.queue.is_empty());
    assert_eq!(r.gpu.segment_count(), 0);
    r.services.sink.fail.store(false, Ordering::SeqCst);
    r.gpu.ensure(&r.range(1..=20)).unwrap();
    assert_eq!(r.ops.publish_pending(), Ok(3));
}
#[test]
fn load_publishes_the_reported_segments_on_success() {
    let r = rig(MIB);
    r.put(1..=3);
    let report = r.ops.load(|_, _| Ok(r.gpu.ensure(&r.range(1..=3))?)).unwrap();
    assert_eq!(report.loaded.len(), 3);
    assert!(r.queue.is_empty());
    assert_eq!(r.gpu.segment_count(), 1);
}
#[test]
fn load_publishes_even_when_the_operation_fails() {
    let r = rig(MIB);
    r.put(1..=3);
    let calls = AtomicUsize::new(0);
    let err = r
        .ops
        .load(|_, _| -> Result<(), IndexOpError> {
            calls.fetch_add(1, Ordering::SeqCst);
            r.gpu.ensure(&r.range(1..=3))?;
            Err(IndexOpError::Transfer("boom".into()))
        })
        .unwrap_err();
    assert_eq!(err, IndexOpError::Transfer("boom".into()));
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    assert!(r.queue.is_empty());
    assert_eq!(r.gpu.segment_count(), 1);
}
#[test]
fn publish_error_after_a_successful_operation_is_returned() {
    let r = rig(MIB);
    r.put(1..=3);
    r.services.sink.fail.store(true, Ordering::SeqCst);
    let err = r.ops.load(|_, _| Ok(r.gpu.ensure(&r.range(1..=3))?)).unwrap_err();
    assert!(matches!(err, IndexOpError::Transfer(_)));
    assert!(r.queue.is_empty());
}
#[test]
fn publish_error_does_not_hide_the_operation_error() {
    let r = rig(MIB);
    r.put(1..=3);
    r.services.sink.fail.store(true, Ordering::SeqCst);
    let err = r
        .ops
        .load(|_, _| -> Result<(), IndexOpError> {
            r.gpu.ensure(&r.range(1..=3))?;
            Err(IndexOpError::Transfer("op failed".into()))
        })
        .unwrap_err();
    assert_eq!(err, IndexOpError::Transfer("op failed".into()));
}
#[test]
fn load_without_an_index_does_not_call_the_operation() {
    let r = assemble(Arc::new(IndexManager::new(IndexId::new("none"))), MIB);
    let calls = AtomicUsize::new(0);
    let err = r
        .ops
        .load(|_, _| {
            calls.fetch_add(1, Ordering::SeqCst);
            Ok(())
        })
        .unwrap_err();
    assert!(matches!(err, IndexOpError::Transfer(_)));
    assert_eq!(calls.load(Ordering::SeqCst), 0);
    let err = r.ops.reconcile_iteration(|_, _| Ok(())).unwrap_err();
    assert!(matches!(err, IndexOpError::Transfer(_)));
}
#[test]
fn exhaustion_makes_room_and_retries_once() {
    let r = rig(MIB);
    r.put(1..=3);
    r.ops.ensure(&r.scope(&[1, 2, 3])).unwrap();
    assert_eq!(r.gpu.segment_count(), 1);
    let calls = AtomicUsize::new(0);
    let out = r
        .ops
        .load(|_, _| {
            if calls.fetch_add(1, Ordering::SeqCst) == 0 {
                Err(IndexOpError::ArenaExhausted {
                    requested: 1 << 20,
                    available: 0,
                })
            } else {
                Ok(7)
            }
        });
    assert_eq!(out, Ok(7));
    assert_eq!(calls.load(Ordering::SeqCst), 2);
    assert_eq!(r.gpu.segment_count(), 0);
}
#[test]
fn second_exhaustion_is_returned_without_a_third_attempt() {
    let r = rig(MIB);
    r.put(1..=3);
    r.ops.ensure(&r.scope(&[1, 2, 3])).unwrap();
    let calls = AtomicUsize::new(0);
    let out: Result<(), _> = r
        .ops
        .load(|_, _| {
            calls.fetch_add(1, Ordering::SeqCst);
            Err(IndexOpError::ArenaExhausted {
                requested: 1 << 20,
                available: 0,
            })
        });
    assert_eq!(
        out, Err(IndexOpError::ArenaExhausted { requested : 1 << 20, available : 0 })
    );
    assert_eq!(calls.load(Ordering::SeqCst), 2);
}
#[test]
fn unsatisfiable_need_is_not_retried() {
    let r = rig(MIB);
    r.put(1..=3);
    r.ops.ensure(&r.scope(&[1, 2, 3])).unwrap();
    let calls = AtomicUsize::new(0);
    let out: Result<(), _> = r
        .ops
        .load(|_, _| {
            calls.fetch_add(1, Ordering::SeqCst);
            Err(IndexOpError::ArenaExhausted {
                requested: u64::MAX / 2,
                available: 0,
            })
        });
    assert!(matches!(out, Err(IndexOpError::ArenaExhausted { .. })));
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    assert_eq!(r.gpu.segment_count(), 1);
}
#[test]
fn pending_segments_are_published_before_room_is_made() {
    let r = rig(MIB);
    r.put(1..=3);
    let calls = AtomicUsize::new(0);
    let out = r
        .ops
        .load(|_, _| {
            if calls.fetch_add(1, Ordering::SeqCst) == 0 {
                r.gpu.ensure(&r.range(1..=3))?;
                Err(IndexOpError::ArenaExhausted {
                    requested: 1 << 20,
                    available: 0,
                })
            } else {
                Ok(())
            }
        });
    assert_eq!(out, Ok(()));
    assert!(r.queue.is_empty());
    assert_eq!(r.gpu.segment_count(), 0);
}
#[test]
fn reconcile_iteration_publishes_and_returns_the_value() {
    let r = rig(MIB);
    r.put(1..=3);
    let out = r
        .ops
        .reconcile_iteration(|_, queue| {
            assert!(queue.is_empty());
            r.gpu.ensure(&r.range(1..=3))?;
            Ok(5)
        });
    assert_eq!(out, Ok(5));
    assert!(r.queue.is_empty());
    assert_eq!(r.gpu.segment_count(), 1);
}
#[test]
fn reconcile_iteration_returns_the_operation_error_and_does_not_retry() {
    let r = rig(MIB);
    r.put(1..=3);
    r.services.sink.fail.store(true, Ordering::SeqCst);
    let out: Result<(), _> = r
        .ops
        .reconcile_iteration(|_, _| {
            r.gpu.ensure(&r.range(1..=3))?;
            Err(IndexOpError::ArenaExhausted {
                requested: 1,
                available: 0,
            })
        });
    assert_eq!(out, Err(IndexOpError::ArenaExhausted { requested : 1, available : 0 }));
    assert!(r.queue.is_empty());
}
#[test]
fn reconcile_iteration_returns_the_publish_error() {
    let r = rig(MIB);
    r.put(1..=3);
    r.services.sink.fail.store(true, Ordering::SeqCst);
    let out = r.ops.reconcile_iteration(|_, _| Ok(r.gpu.ensure(&r.range(1..=3))?));
    assert!(matches!(out, Err(IndexOpError::Transfer(_))));
    assert!(r.queue.is_empty());
}
#[test]
fn backend_ensure_reports_the_list_lengths() {
    let r = rig(MIB);
    r.put(1..=3);
    let first = ResidencyBackend::ensure(&r.ops, &r.scope(&[1, 2, 3, 99])).unwrap();
    assert_eq!(
        first, EnsureSummary { loaded : 3, already_resident : 0, missing : 1, failed : 0
        }
    );
    let second = ResidencyBackend::ensure(&r.ops, &r.scope(&[1, 2, 3, 99])).unwrap();
    assert_eq!(
        second, EnsureSummary { loaded : 0, already_resident : 3, missing : 1, failed : 0
        }
    );
    assert!(r.queue.is_empty());
}
#[test]
fn backend_ensure_errors_become_text() {
    let r = assemble(Arc::new(IndexManager::new(IndexId::new("none"))), MIB);
    let err = ResidencyBackend::ensure(&r.ops, &r.scope(&[1])).unwrap_err();
    assert!(! err.is_empty());
}
#[test]
fn backend_residency_follows_the_applied_bindings() {
    let r = rig(MIB);
    r.put(1..=3);
    ResidencyBackend::ensure(&r.ops, &r.scope(&[1, 2, 3])).unwrap();
    assert!(! r.ops.is_resident(EntryId(1)));
    assert_eq!(r.apply(), 3);
    assert!(r.ops.is_resident(EntryId(1)));
    assert!(! r.ops.is_resident(EntryId(99)));
}
#[test]
fn backend_reconcile_reports_a_summary() {
    let r = rig(MIB);
    r.put(1..=3);
    ResidencyBackend::ensure(&r.ops, &r.scope(&[1, 2, 3])).unwrap();
    r.apply();
    let summary = ResidencyBackend::reconcile(&r.ops).unwrap().unwrap();
    assert_eq!(summary.removed, 0);
    assert_eq!(summary.reloaded, 0);
}
#[test]
fn backend_run_gap_publishes_first() {
    let r = rig(MIB);
    r.put(1..=3);
    r.gpu.ensure(&r.range(1..=3)).unwrap();
    assert_eq!(r.queue.len(), 1);
    assert!(r.ops.run_gap() >= 1);
    assert!(r.queue.is_empty());
    assert_eq!(r.gpu.segment_count(), 1);
}
#[test]
fn backend_shutdown_releases_every_segment() {
    let r = rig(MIB);
    r.put(1..=26);
    ResidencyBackend::ensure(&r.ops, &r.scope(&(1..=24).collect::<Vec<_>>())).unwrap();
    r.gpu.ensure(&r.range(25..=26)).unwrap();
    assert!(! r.queue.is_empty());
    let expected = r.gpu.segment_count() + r.queue.len();
    assert!(r.gpu.segment_count() >= 1);
    assert_eq!(r.ops.shutdown(), Ok(expected));
    assert_eq!(r.gpu.segment_count(), 0);
    assert!(r.queue.is_empty());
    assert_eq!(r.ops.shutdown(), Ok(0));
}
#[test]
fn concurrent_loads_publish_every_segment_once() {
    let r = Arc::new(rig(MIB));
    r.put(1..=32);
    let handles: Vec<_> = (0..4u64)
        .map(|i| {
            let r = Arc::clone(&r);
            thread::spawn(move || {
                let ids: Vec<u64> = (i * 8 + 1..=i * 8 + 8).collect();
                ResidencyBackend::ensure(&r.ops, &r.scope(&ids)).unwrap();
                r.ops.publish_pending().unwrap();
            })
        })
        .collect();
    for handle in handles {
        handle.join().unwrap();
    }
    assert!(r.queue.is_empty());
    r.apply();
    for id in 1..=32 {
        assert!(r.ops.is_resident(EntryId(id)), "entry {id}");
    }
    let ids: Vec<u64> = r.gpu.segment_ids().iter().map(|id| id.0).collect();
    assert!(ids.windows(2).all(| w | w[0] < w[1]));
}
