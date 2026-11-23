use super::*;
use l040::l030::l020::l010::scopes_and_entry_sets::RawEntryId;
use std::collections::HashSet;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::mpsc;
use std::thread;
const ENTRIES: [u64; 3] = [1, 2, 3];
fn id(name: &str) -> IndexId {
    IndexId::new(name)
}
fn spec() -> IndexSpec {
    IndexSpec {
        class: SlotClass::C32,
        dim: 4,
        dtype: DType::F32,
        segment_bytes: 4096,
    }
}
fn manager(name: &str) -> Arc<IndexManager> {
    Arc::new(IndexManager::new(IndexId::new(name)))
}
#[derive(Default)]
struct MockBackend {
    ensure_calls: AtomicUsize,
    reconcile_calls: AtomicUsize,
    shutdown_calls: AtomicUsize,
    gap_units: AtomicUsize,
    shutdown_count: AtomicUsize,
    resident: Mutex<HashSet<u64>>,
    pending: AtomicBool,
    fail_ensure: AtomicBool,
    fail_shutdown: AtomicBool,
    panic_in_ensure: AtomicBool,
    reconcile_result: Mutex<Option<Result<Option<ReconcileSummary>, String>>>,
    block: Mutex<Option<(mpsc::Sender<()>, mpsc::Receiver<()>)>>,
}
impl ResidencyBackend for MockBackend {
    fn ensure(&self, _scope: &Scope) -> Result<EnsureSummary, String> {
        self.ensure_calls.fetch_add(1, Ordering::SeqCst);
        if self.panic_in_ensure.swap(false, Ordering::SeqCst) {
            panic!("backend ensure panic");
        }
        if let Some((entered, release)) = lock(&self.block).take() {
            entered.send(()).unwrap();
            release.recv().unwrap();
        }
        if self.fail_ensure.load(Ordering::SeqCst) {
            return Err("ensure failed".to_string());
        }
        let mut resident = lock(&self.resident);
        let mut summary = EnsureSummary::default();
        for e in ENTRIES {
            if resident.insert(e) {
                summary.loaded += 1;
            } else {
                summary.already_resident += 1;
            }
        }
        Ok(summary)
    }
    fn reconcile(&self) -> Result<Option<ReconcileSummary>, String> {
        self.reconcile_calls.fetch_add(1, Ordering::SeqCst);
        lock(&self.reconcile_result)
            .clone()
            .unwrap_or(Ok(Some(ReconcileSummary::default())))
    }
    fn pending(&self) -> bool {
        self.pending.load(Ordering::SeqCst)
    }
    fn is_resident(&self, entry: EntryId) -> bool {
        lock(&self.resident).contains(&entry.0)
    }
    fn run_gap(&self) -> usize {
        self.gap_units.load(Ordering::SeqCst)
    }
    fn shutdown(&self) -> Result<usize, String> {
        self.shutdown_calls.fetch_add(1, Ordering::SeqCst);
        if self.fail_shutdown.load(Ordering::SeqCst) {
            return Err("shutdown failed".to_string());
        }
        Ok(self.shutdown_count.load(Ordering::SeqCst))
    }
}
#[derive(Default)]
struct MockFactory {
    backends: Mutex<HashMap<IndexId, Arc<MockBackend>>>,
    create_calls: AtomicUsize,
    fail_remaining: AtomicUsize,
    last_spec: Mutex<Option<IndexSpec>>,
}
impl MockFactory {
    fn backend(&self, id: &IndexId) -> Arc<MockBackend> {
        Arc::clone(lock(&self.backends).entry(id.clone()).or_default())
    }
}
impl BackendFactory for MockFactory {
    fn create(
        &self,
        id: &IndexId,
        spec: &IndexSpec,
        _index: Arc<IndexManager>,
    ) -> Result<Arc<dyn ResidencyBackend>, String> {
        self.create_calls.fetch_add(1, Ordering::SeqCst);
        *lock(&self.last_spec) = Some(*spec);
        let failing = self.fail_remaining.load(Ordering::SeqCst) > 0;
        if failing {
            self.fail_remaining.fetch_sub(1, Ordering::SeqCst);
            return Err("factory failed".to_string());
        }
        let backend: Arc<dyn ResidencyBackend> = self.backend(id);
        Ok(backend)
    }
}
fn setup() -> (Arc<MockFactory>, ResidencyController) {
    let factory = Arc::new(MockFactory::default());
    let controller = ResidencyController::new(factory.clone());
    (factory, controller)
}
fn registered(names: &[&str]) -> (Arc<MockFactory>, ResidencyController) {
    let (factory, controller) = setup();
    for name in names {
        controller.register(id(name), manager(name), spec()).unwrap();
    }
    (factory, controller)
}
#[test]
fn register_is_lazy() {
    let (factory, controller) = setup();
    let mgr = manager("a");
    controller.register(id("a"), Arc::clone(&mgr), spec()).unwrap();
    assert_eq!(factory.create_calls.load(Ordering::SeqCst), 0);
    assert_eq!(controller.registered_count(), 1);
    assert_eq!(controller.active_count(), 0);
    assert!(! controller.is_active(& id("a")));
    assert!(! controller.is_resident(& id("a"), EntryId(1)));
    let status = mgr.status();
    assert!(! status.attached);
    assert!(! status.active);
}
#[test]
fn register_rejects_invalid_spec() {
    let (_, controller) = setup();
    let zero_dim = IndexSpec { dim: 0, ..spec() };
    let small = IndexSpec {
        segment_bytes: 511,
        ..spec()
    };
    let overflow = IndexSpec {
        dim: u32::MAX,
        ..spec()
    };
    for bad in [zero_dim, small, overflow] {
        assert_eq!(
            controller.register(id("a"), manager("a"), bad),
            Err(ResidencyError::InvalidSpec)
        );
    }
    assert_eq!(controller.registered_count(), 0);
    let exact = IndexSpec {
        segment_bytes: 512,
        ..spec()
    };
    controller.register(id("a"), manager("a"), exact).unwrap();
}
#[test]
fn register_rejects_duplicate() {
    let (_, controller) = registered(&["a"]);
    assert_eq!(
        controller.register(id("a"), manager("a"), spec()),
        Err(ResidencyError::AlreadyRegistered(id("a")))
    );
    assert_eq!(controller.registered_count(), 1);
}
#[test]
fn unknown_index_errors() {
    let (_, controller) = setup();
    let unknown = ResidencyError::UnknownIndex(id("x"));
    assert_eq!(controller.ensure(& id("x"), & Scope::All), Err(unknown.clone()));
    assert_eq!(controller.reconcile(& id("x")), Err(unknown.clone()));
    assert_eq!(controller.pending(& id("x")), Err(unknown.clone()));
    assert_eq!(controller.unregister(& id("x")), Err(unknown));
    assert!(! controller.is_resident(& id("x"), EntryId(1)));
    assert!(! controller.is_active(& id("x")));
}
#[test]
fn ensure_creates_backend_once_and_loads() {
    let (factory, controller) = registered(&["a"]);
    let first = controller.ensure(&id("a"), &Scope::All).unwrap();
    assert_eq!(first.loaded, 3);
    let second = controller.ensure(&id("a"), &Scope::All).unwrap();
    assert_eq!(second.loaded, 0);
    assert_eq!(second.already_resident, 3);
    assert_eq!(factory.create_calls.load(Ordering::SeqCst), 1);
    assert_eq!(* lock(& factory.last_spec), Some(spec()));
    assert!(controller.is_active(& id("a")));
    assert_eq!(controller.active_count(), 1);
    assert!(controller.is_resident(& id("a"), EntryId(2)));
    assert!(! controller.is_resident(& id("a"), EntryId(9)));
}
#[test]
fn ensure_accepts_other_scopes() {
    let (_, controller) = registered(&["a"]);
    let scope = Scope::Range {
        start: 1 as RawEntryId,
        end: 3 as RawEntryId,
    };
    controller.ensure(&id("a"), &scope).unwrap();
}
#[test]
fn concurrent_ensure_creates_one_backend() {
    let (factory, controller) = registered(&["a"]);
    let controller = Arc::new(controller);
    let handles: Vec<_> = (0..8)
        .map(|_| {
            let c = Arc::clone(&controller);
            thread::spawn(move || c.ensure(&id("a"), &Scope::All).unwrap().loaded)
        })
        .collect();
    let loaded: u64 = handles.into_iter().map(|h| h.join().unwrap()).sum();
    assert_eq!(loaded, 3);
    assert_eq!(factory.create_calls.load(Ordering::SeqCst), 1);
}
#[test]
fn factory_failure_is_not_cached() {
    let (factory, controller) = registered(&["a"]);
    factory.fail_remaining.store(1, Ordering::SeqCst);
    assert_eq!(
        controller.ensure(& id("a"), & Scope::All),
        Err(ResidencyError::Backend("factory failed".to_string()))
    );
    assert!(! controller.is_active(& id("a")));
    assert_eq!(controller.active_count(), 0);
    assert_eq!(controller.ensure(& id("a"), & Scope::All).unwrap().loaded, 3);
    assert_eq!(factory.create_calls.load(Ordering::SeqCst), 2);
}
#[test]
fn backend_ensure_failure_maps_to_backend_error() {
    let (factory, controller) = registered(&["a"]);
    factory.backend(&id("a")).fail_ensure.store(true, Ordering::SeqCst);
    assert_eq!(
        controller.ensure(& id("a"), & Scope::All),
        Err(ResidencyError::Backend("ensure failed".to_string()))
    );
    assert!(controller.is_active(& id("a")));
}
#[test]
fn reconcile_without_backend_does_not_activate() {
    let (factory, controller) = registered(&["a"]);
    assert_eq!(controller.reconcile(& id("a")), Ok(ReconcileSummary::default()));
    assert_eq!(factory.create_calls.load(Ordering::SeqCst), 0);
    assert!(! controller.is_active(& id("a")));
}
#[test]
fn reconcile_forwards_to_active_backend() {
    let (factory, controller) = registered(&["a"]);
    controller.ensure(&id("a"), &Scope::All).unwrap();
    let expected = ReconcileSummary {
        removed: 1,
        reloaded: 2,
        current: 5,
    };
    let backend = factory.backend(&id("a"));
    *lock(&backend.reconcile_result) = Some(Ok(Some(expected)));
    assert_eq!(controller.reconcile(& id("a")), Ok(expected));
    *lock(&backend.reconcile_result) = Some(Ok(None));
    assert_eq!(controller.reconcile(& id("a")), Ok(ReconcileSummary::default()));
    *lock(&backend.reconcile_result) = Some(Err("boom".to_string()));
    assert_eq!(
        controller.reconcile(& id("a")), Err(ResidencyError::Backend("boom".to_string()))
    );
    assert_eq!(backend.reconcile_calls.load(Ordering::SeqCst), 3);
}
#[test]
fn pending_reflects_backend() {
    let (factory, controller) = registered(&["a"]);
    assert_eq!(controller.pending(& id("a")), Ok(false));
    assert_eq!(factory.create_calls.load(Ordering::SeqCst), 0);
    controller.ensure(&id("a"), &Scope::All).unwrap();
    assert_eq!(controller.pending(& id("a")), Ok(false));
    factory.backend(&id("a")).pending.store(true, Ordering::SeqCst);
    assert_eq!(controller.pending(& id("a")), Ok(true));
}
#[test]
fn run_gap_sums_active_backends_only() {
    let (factory, controller) = registered(&["a", "b", "c"]);
    assert_eq!(controller.run_gap(), 0);
    factory.backend(&id("a")).gap_units.store(3, Ordering::SeqCst);
    factory.backend(&id("b")).gap_units.store(4, Ordering::SeqCst);
    factory.backend(&id("c")).gap_units.store(100, Ordering::SeqCst);
    controller.ensure(&id("a"), &Scope::All).unwrap();
    controller.ensure(&id("b"), &Scope::All).unwrap();
    assert_eq!(controller.run_gap(), 7);
    assert_eq!(controller.run_gap(), 7);
    assert_eq!(controller.active_count(), 2);
    assert_eq!(controller.registered_count(), 3);
}
#[test]
fn unregister_without_backend_returns_zero() {
    let (factory, controller) = registered(&["a"]);
    assert_eq!(controller.unregister(& id("a")), Ok(0));
    assert_eq!(controller.registered_count(), 0);
    assert_eq!(factory.create_calls.load(Ordering::SeqCst), 0);
}
#[test]
fn unregister_shuts_backend_down() {
    let (factory, controller) = registered(&["a"]);
    controller.ensure(&id("a"), &Scope::All).unwrap();
    let backend = factory.backend(&id("a"));
    backend.shutdown_count.store(5, Ordering::SeqCst);
    assert_eq!(controller.unregister(& id("a")), Ok(5));
    assert_eq!(backend.shutdown_calls.load(Ordering::SeqCst), 1);
    assert_eq!(controller.registered_count(), 0);
    assert_eq!(controller.active_count(), 0);
    assert_eq!(
        controller.ensure(& id("a"), & Scope::All),
        Err(ResidencyError::UnknownIndex(id("a")))
    );
    controller.register(id("a"), manager("a"), spec()).unwrap();
}
#[test]
fn unregister_failure_keeps_registration() {
    let (factory, controller) = registered(&["a"]);
    controller.ensure(&id("a"), &Scope::All).unwrap();
    let backend = factory.backend(&id("a"));
    backend.fail_shutdown.store(true, Ordering::SeqCst);
    assert_eq!(
        controller.unregister(& id("a")), Err(ResidencyError::Backend("shutdown failed"
        .to_string()))
    );
    assert_eq!(controller.registered_count(), 1);
    assert!(controller.is_active(& id("a")));
    backend.fail_shutdown.store(false, Ordering::SeqCst);
    assert_eq!(controller.unregister(& id("a")), Ok(0));
    assert_eq!(controller.registered_count(), 0);
}
#[test]
fn pending_and_run_gap_do_not_wait_for_ensure() {
    let (factory, controller) = registered(&["a"]);
    let controller = Arc::new(controller);
    let (entered_tx, entered_rx) = mpsc::channel();
    let (release_tx, release_rx) = mpsc::channel();
    let backend = factory.backend(&id("a"));
    backend.gap_units.store(2, Ordering::SeqCst);
    *lock(&backend.block) = Some((entered_tx, release_rx));
    let worker = {
        let c = Arc::clone(&controller);
        thread::spawn(move || c.ensure(&id("a"), &Scope::All))
    };
    entered_rx.recv().unwrap();
    assert_eq!(controller.pending(& id("a")), Ok(false));
    assert_eq!(controller.run_gap(), 2);
    assert!(! controller.is_resident(& id("a"), EntryId(1)));
    release_tx.send(()).unwrap();
    assert_eq!(worker.join().unwrap().unwrap().loaded, 3);
}
#[test]
fn different_indexes_run_in_parallel() {
    let (factory, controller) = registered(&["a", "b"]);
    let controller = Arc::new(controller);
    let (entered_tx, entered_rx) = mpsc::channel();
    let (release_tx, release_rx) = mpsc::channel();
    *lock(&factory.backend(&id("a")).block) = Some((entered_tx, release_rx));
    let worker = {
        let c = Arc::clone(&controller);
        thread::spawn(move || c.ensure(&id("a"), &Scope::All))
    };
    entered_rx.recv().unwrap();
    assert_eq!(controller.ensure(& id("b"), & Scope::All).unwrap().loaded, 3);
    release_tx.send(()).unwrap();
    worker.join().unwrap().unwrap();
}
#[test]
fn poisoned_index_lock_is_recovered() {
    let (factory, controller) = registered(&["a"]);
    factory.backend(&id("a")).panic_in_ensure.store(true, Ordering::SeqCst);
    let result = catch_unwind(
        AssertUnwindSafe(|| {
            let _ = controller.ensure(&id("a"), &Scope::All);
        }),
    );
    assert!(result.is_err());
    assert_eq!(controller.ensure(& id("a"), & Scope::All).unwrap().loaded, 3);
    assert!(controller.reconcile(& id("a")).is_ok());
    assert_eq!(controller.unregister(& id("a")), Ok(0));
}
#[test]
fn error_display_is_informative() {
    assert_eq!(ResidencyError::UnknownIndex(id("x")).to_string(), "unknown index: x");
    assert_eq!(ResidencyError::Backend("m".into()).to_string(), "backend error: m");
    let _: &dyn std::error::Error = &ResidencyError::InvalidSpec;
}
