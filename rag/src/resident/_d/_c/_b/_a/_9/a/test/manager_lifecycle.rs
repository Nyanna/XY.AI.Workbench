use super::*;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::a::index_contract::{
    AccessMethod, EntryId, IndexError, Payload, PayloadRef, Stamp, StampObserver,
    StampSignal,
};
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::_1::layout::DType;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

struct FakeIndex {
    entries: usize,
    signal: Arc<StampSignal>,
}

impl FakeIndex {
    fn arc(entries: usize) -> Arc<dyn Index> {
        Arc::new(FakeIndex {
            entries,
            signal: StampSignal::new(Stamp(0)),
        })
    }
}

impl Index for FakeIndex {
    fn dtype(&self) -> DType {
        DType::F32
    }

    fn access_methods(&self) -> &[AccessMethod] {
        &[]
    }

    fn stamp(&self, id: EntryId) -> Option<Stamp> {
        (id.0 < self.entries as u64).then_some(Stamp(id.0 + 1))
    }

    fn stamps(&self) -> Vec<(EntryId, Stamp)> {
        (0..self.entries as u64).map(|i| (EntryId(i), Stamp(i + 1))).collect()
    }

    fn read(&self, _id: EntryId) -> Payload {
        Payload::default()
    }

    fn write(
        &self,
        _id: EntryId,
        _payload: PayloadRef<'_>,
    ) -> Result<Stamp, IndexError> {
        Ok(Stamp(0))
    }

    fn delete(&self, _id: EntryId) -> Result<Option<Stamp>, IndexError> {
        Ok(None)
    }

    fn subscribe(&self) -> StampObserver {
        self.signal.observer()
    }
}

fn manager() -> IndexManager {
    IndexManager::with_observer(IndexId::new("idx"), Arc::new(NoCycleObserver))
}

struct RecordingObserver(Arc<Mutex<Vec<String>>>);

struct RecordingGuard(Arc<Mutex<Vec<String>>>, String);

impl Drop for RecordingGuard {
    fn drop(&mut self) {
        self.0.lock().unwrap().push(format!("exit {}", self.1));
    }
}

impl CycleObserver for RecordingObserver {
    fn enter(&self, index_id: &IndexId, cycle: &str) -> Box<dyn CycleGuard> {
        let label = format!("{index_id} {cycle}");
        self.0.lock().unwrap().push(format!("enter {label}"));
        Box::new(RecordingGuard(Arc::clone(&self.0), label))
    }
}

#[test]
fn initial_state() {
    let m = manager();
    let s = m.status();
    assert!(! s.attached && ! s.active);
    assert_eq!(s.resident_entries, 0);
    assert_eq!(s.index_id.to_string(), "idx");
}

#[test]
fn attach_once() {
    let m = manager();
    let first = FakeIndex::arc(3);
    assert!(m.attach(Arc::clone(& first)));
    assert!(! m.attach(Arc::clone(& first)));
    assert!(! m.attach(FakeIndex::arc(5)));
    assert_eq!(m.status().resident_entries, 3);
    assert!(Arc::ptr_eq(& m.index().unwrap(), & first));
}

#[test]
fn concurrent_attach_single_winner() {
    let m = Arc::new(manager());
    let wins = Arc::new(AtomicUsize::new(0));
    let handles: Vec<_> = (0..8)
        .map(|_| {
            let (m, wins) = (Arc::clone(&m), Arc::clone(&wins));
            thread::spawn(move || {
                if m.attach(FakeIndex::arc(1)) {
                    wins.fetch_add(1, Ordering::SeqCst);
                }
            })
        })
        .collect();
    for h in handles {
        h.join().unwrap();
    }
    assert_eq!(wins.load(Ordering::SeqCst), 1);
}

#[test]
fn unavailable_without_index() {
    let m = manager();
    let err = ManagerError::IndexUnavailable {
        index_id: m.index_id().clone(),
    };
    assert_eq!(m.index().err(), Some(err.clone()));
    let mut called = false;
    assert_eq!(m.activate(| _ | { called = true; Ok(()) }), Err(err.clone()));
    assert_eq!(m.run_cycle("c", | _ | { called = true; }), Err(err));
    assert!(! called);
}

#[test]
fn activate_once() {
    let m = manager();
    m.attach(FakeIndex::arc(2));
    let calls = AtomicUsize::new(0);
    for _ in 0..3 {
        m.activate(|_| {
                calls.fetch_add(1, Ordering::SeqCst);
                Ok(())
            })
            .unwrap();
    }
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    assert!(m.status().active);
}

#[test]
fn failure_then_retry() {
    let m = manager();
    m.attach(FakeIndex::arc(0));
    let err = m.activate(|_| Err("boom".to_string())).unwrap_err();
    assert!(
        matches!(& err, ManagerError::InitFailed { reason, .. } if reason == "boom")
    );
    assert!(err.to_string().contains("idx"));
    assert!(! m.status().active);
    let err = m.activate(|_| panic!("x")).unwrap_err();
    assert!(
        matches!(& err, ManagerError::InitFailed { reason, .. } if reason ==
        "init panicked")
    );
    assert!(! m.status().active);
    m.activate(|_| Ok(())).unwrap();
    assert!(m.status().active);
}

#[test]
fn waiters_share_result_and_status_does_not_block() {
    let m = Arc::new(manager());
    m.attach(FakeIndex::arc(4));
    let (started_tx, started_rx) = mpsc::channel();
    let (release_tx, release_rx) = mpsc::channel::<()>();
    let init_m = Arc::clone(&m);
    let initializer = thread::spawn(move || {
        init_m
            .activate(|_| {
                started_tx.send(()).unwrap();
                release_rx.recv().unwrap();
                Err("late failure".to_string())
            })
    });
    started_rx.recv().unwrap();
    let waiter_m = Arc::clone(&m);
    let waiter = thread::spawn(move || {
        waiter_m.activate(|_| panic!("waiter init must not run"))
    });
    thread::sleep(Duration::from_millis(50));
    let s = m.status();
    assert!(! s.active);
    assert_eq!(s.resident_entries, 4);
    assert!(m.index().is_ok());
    release_tx.send(()).unwrap();
    let a = initializer.join().unwrap();
    let b = waiter.join().unwrap();
    assert_eq!(a, b);
    assert!(matches!(a, Err(ManagerError::InitFailed { .. })));
}

#[test]
fn run_cycle_returns_value_without_activation() {
    let m = manager();
    m.attach(FakeIndex::arc(2));
    let n = m.run_cycle("sweep", |i| i.stamps().len()).unwrap();
    assert_eq!(n, 2);
}

#[test]
fn manager_is_send_sync() {
    fn check<T: Send + Sync>() {}
    check::<IndexManager>();
}

#[test]
fn cycle_body_runs_between_observer_enter_and_exit() {
    let events = Arc::new(Mutex::new(Vec::new()));
    let m = IndexManager::with_observer(
        IndexId::new("idx"),
        Arc::new(RecordingObserver(Arc::clone(&events))),
    );
    m.attach(FakeIndex::arc(1));
    m.run_cycle("sweep", |_| events.lock().unwrap().push("body".to_string())).unwrap();
    events.lock().unwrap().push("after".to_string());
    assert_eq!(
        * events.lock().unwrap(), vec!["enter idx sweep", "body", "exit idx sweep",
        "after"]
    );
}

#[test]
fn cycle_span_closes_when_closure_panics() {
    let events = Arc::new(Mutex::new(Vec::new()));
    let m = IndexManager::with_observer(
        IndexId::new("idx"),
        Arc::new(RecordingObserver(Arc::clone(&events))),
    );
    m.attach(FakeIndex::arc(1));
    let result = catch_unwind(
        AssertUnwindSafe(|| {
            let _ = m.run_cycle("boom", |_| -> () { panic!("cycle failure") });
        }),
    );
    assert!(result.is_err());
    assert_eq!(* events.lock().unwrap(), vec!["enter idx boom", "exit idx boom"]);
}

#[test]
fn attach_after_activation_is_rejected_and_keeps_index() {
    let m = manager();
    let first = FakeIndex::arc(2);
    m.attach(Arc::clone(&first));
    m.activate(|_| Ok(())).unwrap();
    assert!(! m.attach(Arc::clone(& first)));
    assert!(Arc::ptr_eq(& m.index().unwrap(), & first));
    assert!(m.status().active);
}
