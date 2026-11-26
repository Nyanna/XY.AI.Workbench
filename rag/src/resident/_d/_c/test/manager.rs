use super::*;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::a::index_contract::{
    AccessMethod, EntryId, IndexError, Payload2, PayloadRef, Stamp, StampObserver,
    StampSignal,
};
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::_1::layout::DType;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Barrier, Mutex};
use std::thread;
struct FakeIndex {
    signal: Arc<StampSignal>,
}
impl FakeIndex {
    fn arc() -> Arc<dyn Index> {
        Arc::new(FakeIndex {
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
    fn stamp(&self, _id: EntryId) -> Option<Stamp> {
        None
    }
    fn stamps(&self) -> Vec<(EntryId, Stamp)> {
        Vec::new()
    }
    fn read(&self, _id: EntryId) -> Payload2 {
        Payload2::default()
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
/// Reference implementation of the contract: the activation runs without any lock held.
struct TestLazy {
    slot: Mutex<Option<Arc<dyn Index>>>,
    attempts: AtomicUsize,
    failures_left: AtomicUsize,
    gate: Option<Barrier>,
}
impl TestLazy {
    fn new(failures: usize, gate: Option<Barrier>) -> Self {
        TestLazy {
            slot: Mutex::new(None),
            attempts: AtomicUsize::new(0),
            failures_left: AtomicUsize::new(failures),
            gate,
        }
    }
}
impl LazyIndex for TestLazy {
    fn get(&self) -> Result<Arc<dyn Index>, String> {
        if let Some(index) = self.slot.lock().unwrap().as_ref() {
            return Ok(Arc::clone(index));
        }
        self.attempts.fetch_add(1, Ordering::SeqCst);
        if let Some(gate) = &self.gate {
            gate.wait();
        }
        let failing = {
            let mut left = self.failures_left.load(Ordering::SeqCst);
            loop {
                if left == 0 {
                    break false;
                }
                match self
                    .failures_left
                    .compare_exchange(left, left - 1, Ordering::SeqCst, Ordering::SeqCst)
                {
                    Ok(_) => break true,
                    Err(actual) => left = actual,
                }
            }
        };
        if failing {
            return Err("open failed".to_string());
        }
        let fresh = FakeIndex::arc();
        let mut slot = self.slot.lock().unwrap();
        Ok(Arc::clone(slot.get_or_insert(fresh)))
    }
}
#[test]
fn is_send_sync_and_object_safe() {
    fn check<T: Send + Sync + ?Sized>() {}
    check::<dyn LazyIndex>();
    check::<TestLazy>();
}
#[test]
fn same_instance_after_first_success() {
    let lazy = TestLazy::new(0, None);
    let a = lazy.get().unwrap();
    let b = lazy.get().unwrap();
    assert!(Arc::ptr_eq(& a, & b));
    assert_eq!(lazy.attempts.load(Ordering::SeqCst), 1);
}
#[test]
fn failure_is_not_cached() {
    let lazy = TestLazy::new(2, None);
    assert_eq!(lazy.get().err().as_deref(), Some("open failed"));
    assert_eq!(lazy.get().err().as_deref(), Some("open failed"));
    let a = lazy.get().unwrap();
    assert!(Arc::ptr_eq(& a, & lazy.get().unwrap()));
    assert_eq!(lazy.attempts.load(Ordering::SeqCst), 3);
}
#[test]
fn usable_through_trait_object() {
    let lazy: Arc<dyn LazyIndex> = Arc::new(TestLazy::new(0, None));
    let a = lazy.get().unwrap();
    assert!(Arc::ptr_eq(& a, & lazy.get().unwrap()));
}
#[test]
fn concurrent_first_callers_share_one_instance() {
    const N: usize = 8;
    let lazy = Arc::new(TestLazy::new(0, Some(Barrier::new(N))));
    let handles: Vec<_> = (0..N)
        .map(|_| {
            let lazy = Arc::clone(&lazy);
            thread::spawn(move || lazy.get().unwrap())
        })
        .collect();
    let all: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();
    for other in &all[1..] {
        assert!(Arc::ptr_eq(& all[0], other));
    }
    assert!(Arc::ptr_eq(& all[0], & lazy.get().unwrap()));
}
