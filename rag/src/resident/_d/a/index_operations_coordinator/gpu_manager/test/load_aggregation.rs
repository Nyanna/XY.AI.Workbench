use super::*;
use futures::executor::block_on;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::a::index_contract::{
    AccessMethod, CellId, IndexError, NodeId, Payload, PayloadRef, Stamp, StampObserver,
    StampSignal,
};
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::_1::layout::DType;
use std::collections::BTreeMap;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::atomic::AtomicUsize;
use std::sync::Barrier;
use std::time::{Duration, Instant};
struct FakeIndex {
    entries: BTreeMap<u64, u64>,
    signal: Arc<StampSignal>,
    reads: Mutex<Vec<u64>>,
    touches: AtomicUsize,
    rendezvous: Option<Arc<AtomicUsize>>,
    met: Mutex<bool>,
}
impl FakeIndex {
    fn new(ids: &[u64]) -> Self {
        Self {
            entries: ids.iter().map(|&i| (i, i + 100)).collect(),
            signal: StampSignal::new(Stamp(1)),
            reads: Mutex::new(Vec::new()),
            touches: AtomicUsize::new(0),
            rendezvous: None,
            met: Mutex::new(false),
        }
    }
    fn with_rendezvous(mut self, counter: Arc<AtomicUsize>) -> Self {
        self.rendezvous = Some(counter);
        self
    }
    fn reads(&self) -> Vec<u64> {
        self.reads.lock().unwrap().clone()
    }
    fn touches(&self) -> usize {
        self.touches.load(Ordering::SeqCst)
    }
    fn rendezvous_met(&self) -> bool {
        *self.met.lock().unwrap()
    }
    fn meet(&self) {
        let Some(counter) = &self.rendezvous else {
            return;
        };
        counter.fetch_add(1, Ordering::SeqCst);
        let deadline = Instant::now() + Duration::from_secs(5);
        while counter.load(Ordering::SeqCst) < 2 {
            if Instant::now() > deadline {
                return;
            }
            std::thread::yield_now();
        }
        *self.met.lock().unwrap() = true;
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
        self.touches.fetch_add(1, Ordering::SeqCst);
        self.entries.get(&id.0).map(|&s| Stamp(s))
    }
    fn stamps(&self) -> Vec<(EntryId, Stamp)> {
        self.touches.fetch_add(1, Ordering::SeqCst);
        self.entries.iter().map(|(&i, &s)| (EntryId(i), Stamp(s))).collect()
    }
    fn read(&self, id: EntryId) -> Payload {
        self.touches.fetch_add(1, Ordering::SeqCst);
        self.meet();
        self.reads.lock().unwrap().push(id.0);
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
    fn children(&self, _node: NodeId) -> Result<Vec<NodeId>, IndexError> {
        Err(IndexError::NotFound)
    }
    fn cell_entries(&self, _cell: CellId) -> Result<Vec<EntryId>, IndexError> {
        Err(IndexError::NotFound)
    }
}
fn set(ids: &[u64]) -> Scope {
    Scope::Set(EntrySet::from_ids(ids.iter().copied()))
}
fn request(index: u32, scope: Scope) -> LoadRequest<u32> {
    LoadRequest { index, scope }
}
fn registry() -> ScopeResolverRegistry {
    ScopeResolverRegistry::with_defaults()
}
fn loaded_ids(loaded: &Loaded) -> Vec<u64> {
    loaded.iter().map(|(id, _)| id.0).collect()
}
fn run(
    requests: Vec<LoadRequest<u32>>,
    indexes: &HashMap<u32, Arc<FakeIndex>>,
) -> Vec<IndexLoad<u32>> {
    let lookup = |key: &u32| indexes.get(key).map(|i| Arc::clone(i) as Arc<dyn Index>);
    load_aggregated(requests, &registry(), &lookup, None)
}
fn indexes(list: Vec<(u32, FakeIndex)>) -> HashMap<u32, Arc<FakeIndex>> {
    list.into_iter().map(|(k, i)| (k, Arc::new(i))).collect()
}
#[test]
fn types_are_send_and_sync() {
    fn sync_send<T: Send + Sync>() {}
    fn send<T: Send>() {}
    sync_send::<Collector<u32>>();
    sync_send::<SingleFlight<String, u32, String>>();
    sync_send::<Flight<u32, String>>();
    send::<FlightCompleter<String, u32, String>>();
}
#[test]
fn collector_counts_and_drains_in_order() {
    let c = Collector::new();
    assert!(c.is_empty());
    assert_eq!(c.len(), 0);
    c.push(request(2, set(&[1])));
    c.push(request(1, set(&[2])));
    c.push(request(2, set(&[1])));
    assert_eq!(c.len(), 3);
    assert!(! c.is_empty());
    let drained = c.drain();
    assert_eq!(
        drained, vec![request(2, set(& [1])), request(1, set(& [2])), request(2, set(&
        [1]))]
    );
    assert!(c.is_empty());
    assert!(c.drain().is_empty());
}
#[test]
fn collector_default_is_empty() {
    let c: Collector<u32> = Collector::default();
    assert_eq!(c.len(), 0);
}
#[test]
fn collector_concurrent_pushes_are_neither_lost_nor_duplicated() {
    let c = Collector::new();
    std::thread::scope(|s| {
        for t in 0..8u32 {
            let c = &c;
            s.spawn(move || {
                for n in 0..100u64 {
                    c.push(request(t, set(&[n])));
                }
            });
        }
    });
    assert_eq!(c.len(), 800);
    let drained = c.drain();
    assert_eq!(drained.len(), 800);
    for t in 0..8u32 {
        let seq: Vec<u64> = drained
            .iter()
            .filter(|r| r.index == t)
            .map(|r| match &r.scope {
                Scope::Set(s) => s.iter().next().unwrap(),
                _ => unreachable!(),
            })
            .collect();
        assert_eq!(seq, (0..100).collect::< Vec < u64 >> ());
    }
}
#[test]
fn aggregated_empty_input_yields_empty() {
    let out = run(Vec::new(), &HashMap::new());
    assert!(out.is_empty());
}
#[test]
fn aggregated_unions_scopes_and_loads_each_entry_once() {
    let idx = indexes(vec![(1, FakeIndex::new(& [1, 2, 3, 4, 9]))]);
    let out = run(vec![request(1, set(& [3, 1, 2])), request(1, set(& [3, 4]))], &idx);
    assert_eq!(out.len(), 1);
    let loaded = out[0].result.as_ref().unwrap();
    assert_eq!(loaded_ids(loaded), vec![1, 2, 3, 4]);
    assert!(loaded.iter().all(| (_, e) | e.is_some()));
    let mut reads = idx[&1].reads();
    reads.sort_unstable();
    assert_eq!(reads, vec![1, 2, 3, 4]);
}
#[test]
fn aggregated_marks_missing_entries_as_none() {
    let idx = indexes(vec![(1, FakeIndex::new(& [2]))]);
    let out = run(vec![request(1, set(& [5, 2]))], &idx);
    let loaded = out[0].result.as_ref().unwrap();
    assert_eq!(loaded_ids(loaded), vec![2, 5]);
    assert!(loaded[0].1.is_some());
    assert!(loaded[1].1.is_none());
}
#[test]
fn aggregated_all_scope_loads_every_entry() {
    let idx = indexes(vec![(1, FakeIndex::new(& [7, 3, 5]))]);
    let out = run(vec![request(1, Scope::All)], &idx);
    assert_eq!(loaded_ids(out[0].result.as_ref().unwrap()), vec![3, 5, 7]);
}
#[test]
fn aggregated_orders_results_by_first_appearance() {
    let idx = indexes(
        vec![
            (1, FakeIndex::new(& [1])), (2, FakeIndex::new(& [2])), (3, FakeIndex::new(&
            [3])),
        ],
    );
    let out = run(
        vec![
            request(3, set(& [3])), request(1, set(& [1])), request(3, set(& [3])),
            request(2, set(& [2])),
        ],
        &idx,
    );
    let keys: Vec<u32> = out.iter().map(|l| l.index).collect();
    assert_eq!(keys, vec![3, 1, 2]);
}
#[test]
fn aggregated_calls_lookup_once_per_index() {
    let idx = indexes(vec![(1, FakeIndex::new(& [1, 2])), (2, FakeIndex::new(& [1]))]);
    let calls = Mutex::new(Vec::new());
    let lookup = |key: &u32| {
        calls.lock().unwrap().push(*key);
        idx.get(key).map(|i| Arc::clone(i) as Arc<dyn Index>)
    };
    let requests = vec![
        request(1, set(& [1])), request(2, set(& [1])), request(1, set(& [2])),
        request(1, Scope::All),
    ];
    let out = load_aggregated(requests, &registry(), &lookup, None);
    assert_eq!(out.len(), 2);
    let mut calls = calls.into_inner().unwrap();
    calls.sort_unstable();
    assert_eq!(calls, vec![1, 2]);
}
#[test]
fn aggregated_unavailable_index_does_not_affect_others() {
    let idx = indexes(vec![(2, FakeIndex::new(& [1]))]);
    let out = run(vec![request(1, set(& [1])), request(2, set(& [1]))], &idx);
    assert_eq!(out[0].index, 1);
    assert_eq!(out[0].result.as_ref().unwrap_err(), & ResolveError::IndexUnavailable);
    assert_eq!(loaded_ids(out[1].result.as_ref().unwrap()), vec![1]);
}
#[test]
fn aggregated_resolve_failure_loads_nothing_for_that_index() {
    let idx = indexes(vec![(1, FakeIndex::new(& [1, 2])), (2, FakeIndex::new(& [1]))]);
    let out = run(
        vec![
            request(1, set(& [1])), request(1, Scope::Subtree { root : 1 }), request(2,
            set(& [1])),
        ],
        &idx,
    );
    assert!(matches!(out[0].result, Err(ResolveError::Unsupported { .. })));
    assert_eq!(idx[& 1].touches(), 0);
    assert_eq!(loaded_ids(out[1].result.as_ref().unwrap()), vec![1]);
}
#[test]
fn aggregated_empty_union_does_not_touch_index() {
    let idx = indexes(vec![(1, FakeIndex::new(& [1, 2]))]);
    let out = run(
        vec![
            request(1, Scope::Set(EntrySet::empty())), request(1,
            Scope::Set(EntrySet::empty())),
        ],
        &idx,
    );
    assert_eq!(out[0].result, Ok(Vec::new()));
    assert_eq!(idx[& 1].touches(), 0);
}
#[test]
fn aggregated_runs_indexes_concurrently() {
    let counter = Arc::new(AtomicUsize::new(0));
    let idx = indexes(
        vec![
            (1, FakeIndex::new(& [1]).with_rendezvous(Arc::clone(& counter))), (2,
            FakeIndex::new(& [1]).with_rendezvous(Arc::clone(& counter))),
        ],
    );
    let out = run(vec![request(1, set(& [1])), request(2, set(& [1]))], &idx);
    assert_eq!(out.len(), 2);
    assert!(idx[& 1].rendezvous_met());
    assert!(idx[& 2].rendezvous_met());
}
#[test]
fn aggregated_propagates_thread_panics() {
    let lookup = |key: &u32| -> Option<Arc<dyn Index>> {
        if *key == 2 {
            panic!("lookup failed");
        }
        Some(Arc::new(FakeIndex::new(&[1])))
    };
    let outcome = catch_unwind(
        AssertUnwindSafe(|| {
            load_aggregated(
                vec![request(1, set(& [1])), request(2, set(& [1]))],
                &registry(),
                &lookup,
                None,
            )
        }),
    );
    assert!(outcome.is_err());
}
#[derive(Debug, PartialEq)]
struct NoClone(String);
type Flights = SingleFlight<String, u32, NoClone>;
type Slot = Mutex<Option<FlightCompleter<String, u32, NoClone>>>;
fn park(slot: &Slot) -> impl FnOnce(FlightCompleter<String, u32, NoClone>) + '_ {
    move |completer| *slot.lock().unwrap() = Some(completer)
}
fn key() -> String {
    "k".to_string()
}
#[test]
fn starter_calls_start_once_and_joiner_never() {
    let sf = Flights::new();
    let slot: Slot = Mutex::new(None);
    let starts = AtomicUsize::new(0);
    let a = sf
        .join_or_start(
            key(),
            |c| {
                starts.fetch_add(1, Ordering::SeqCst);
                *slot.lock().unwrap() = Some(c);
            },
        );
    let b = sf
        .join_or_start(
            key(),
            |_| {
                starts.fetch_add(1, Ordering::SeqCst);
            },
        );
    assert_eq!(a.role(), FlightRole::Starter);
    assert_eq!(b.role(), FlightRole::Joiner);
    assert_eq!(starts.load(Ordering::SeqCst), 1);
    assert_eq!(sf.in_flight(), 1);
    assert!(sf.is_in_flight(& key()));
    assert_eq!(sf.keys_in_flight(), vec![key()]);
    assert!(a.task().result().is_none());
    slot.lock().unwrap().take().unwrap().finish(Ok(7));
}
#[test]
fn all_handles_share_the_result() {
    let sf = Flights::new();
    let slot: Slot = Mutex::new(None);
    let a = sf.join_or_start(key(), park(&slot));
    let b = sf.join_or_start(key(), |_| unreachable!());
    let c = b.clone();
    slot.lock().unwrap().take().unwrap().finish(Ok(7));
    assert_eq!(block_on(a.wait()), Ok(& 7));
    assert_eq!(block_on(b.wait()), Ok(& 7));
    assert_eq!(block_on(c.wait()), Ok(& 7));
    assert_eq!(c.role(), FlightRole::Joiner);
    assert!(std::ptr::eq(block_on(a.wait()).unwrap(), block_on(b.wait()).unwrap()));
}
#[test]
fn failure_is_shared_without_cloning() {
    let sf = Flights::new();
    let slot: Slot = Mutex::new(None);
    let a = sf.join_or_start(key(), park(&slot));
    let b = sf.join_or_start(key(), |_| unreachable!());
    slot.lock().unwrap().take().unwrap().finish(Err(NoClone("boom".into())));
    let expected = NoClone("boom".into());
    assert_eq!(block_on(a.wait()), Err(TaskError::Failed(& expected)));
    assert_eq!(block_on(b.wait()), Err(TaskError::Failed(& expected)));
    let (ea, eb) = match (block_on(a.wait()), block_on(b.wait())) {
        (Err(TaskError::Failed(x)), Err(TaskError::Failed(y))) => (x, y),
        _ => unreachable!(),
    };
    assert!(std::ptr::eq(ea, eb));
}
#[test]
fn entry_is_removed_before_waiters_see_the_result() {
    let sf = Flights::new();
    let slot: Slot = Mutex::new(None);
    let a = sf.join_or_start(key(), park(&slot));
    slot.lock().unwrap().take().unwrap().finish(Ok(1));
    assert_eq!(sf.in_flight(), 0);
    assert!(! sf.is_in_flight(& key()));
    assert!(a.task().result().is_some());
}
#[test]
fn new_caller_after_finish_starts_a_new_flight_without_caching() {
    let sf = Flights::new();
    let slot: Slot = Mutex::new(None);
    let first = sf.join_or_start(key(), park(&slot));
    slot.lock().unwrap().take().unwrap().finish(Err(NoClone("x".into())));
    let second = sf.join_or_start(key(), park(&slot));
    assert_eq!(second.role(), FlightRole::Starter);
    assert!(second.task().result().is_none());
    slot.lock().unwrap().take().unwrap().finish(Ok(2));
    assert_eq!(block_on(second.wait()), Ok(& 2));
    assert!(matches!(block_on(first.wait()), Err(TaskError::Failed(_))));
}
#[test]
fn different_keys_are_independent() {
    let sf = Flights::new();
    let a = sf.join_or_start("a".to_string(), |c| std::mem::forget(c));
    let b = sf.join_or_start("b".to_string(), |c| std::mem::forget(c));
    assert_eq!(a.role(), FlightRole::Starter);
    assert_eq!(b.role(), FlightRole::Starter);
    let mut keys = sf.keys_in_flight();
    keys.sort();
    assert_eq!(keys, vec!["a".to_string(), "b".to_string()]);
}
#[test]
fn dropping_the_completer_cancels_and_removes_the_entry() {
    let sf = Flights::new();
    let a = sf.join_or_start(key(), |c| drop(c));
    let slot: Slot = Mutex::new(None);
    assert_eq!(sf.in_flight(), 0);
    assert_eq!(block_on(a.wait()), Err(TaskError::Cancelled));
    let b = sf.join_or_start(key(), park(&slot));
    assert_eq!(b.role(), FlightRole::Starter);
    slot.lock().unwrap().take().unwrap().finish(Ok(3));
    assert_eq!(block_on(b.wait()), Ok(& 3));
}
#[test]
fn dropping_a_parked_completer_cancels_joiners() {
    let sf = Flights::new();
    let slot: Slot = Mutex::new(None);
    let _a = sf.join_or_start(key(), park(&slot));
    let b = sf.join_or_start(key(), |_| unreachable!());
    drop(slot.lock().unwrap().take());
    assert_eq!(block_on(b.wait()), Err(TaskError::Cancelled));
    assert_eq!(sf.in_flight(), 0);
}
#[test]
fn panic_in_start_propagates_and_cancels() {
    let sf = Flights::new();
    let outcome = catch_unwind(
        AssertUnwindSafe(|| {
            sf.join_or_start(
                key(),
                |c| {
                    let _c = c;
                    panic!("start failed");
                },
            )
        }),
    );
    assert!(outcome.is_err());
    assert_eq!(sf.in_flight(), 0);
    let slot: Slot = Mutex::new(None);
    let again = sf.join_or_start(key(), park(&slot));
    assert_eq!(again.role(), FlightRole::Starter);
    slot.lock().unwrap().take().unwrap().finish(Ok(1));
}
#[test]
fn stale_id_does_not_remove_a_newer_entry() {
    let sf = Flights::new();
    let slot: Slot = Mutex::new(None);
    let _a = sf.join_or_start(key(), park(&slot));
    sf.shared.remove_if_current(&key(), 9_999);
    assert!(sf.is_in_flight(& key()));
    slot.lock().unwrap().take().unwrap().finish(Ok(1));
    assert!(! sf.is_in_flight(& key()));
}
#[test]
fn flight_ids_are_monotonic() {
    let sf = Flights::new();
    let slot: Slot = Mutex::new(None);
    let mut last = None;
    for _ in 0..3 {
        let _f = sf.join_or_start(key(), park(&slot));
        let id = slot.lock().unwrap().as_ref().unwrap().id;
        if let Some(prev) = last {
            assert!(id > prev);
        }
        last = Some(id);
        slot.lock().unwrap().take().unwrap().finish(Ok(0));
    }
}
#[test]
fn externally_cancelled_task_does_not_break_finish() {
    let sf = Flights::new();
    let slot: Slot = Mutex::new(None);
    let a = sf.join_or_start(key(), park(&slot));
    assert!(a.task().cancel());
    slot.lock().unwrap().take().unwrap().finish(Ok(1));
    assert_eq!(block_on(a.wait()), Err(TaskError::Cancelled));
    assert_eq!(sf.in_flight(), 0);
}
#[test]
fn clones_of_single_flight_share_state() {
    let sf = Flights::default();
    let other = sf.clone();
    let slot: Slot = Mutex::new(None);
    let _a = sf.join_or_start(key(), park(&slot));
    let b = other.join_or_start(key(), |_| unreachable!());
    assert_eq!(b.role(), FlightRole::Joiner);
    assert_eq!(other.in_flight(), 1);
    slot.lock().unwrap().take().unwrap().finish(Ok(1));
}
#[test]
fn waiter_blocked_before_finish_is_woken() {
    let sf = Flights::new();
    let slot: Slot = Mutex::new(None);
    let a = sf.join_or_start(key(), park(&slot));
    let b = sf.join_or_start(key(), |_| unreachable!());
    std::thread::scope(|s| {
        let waiter = s.spawn(|| block_on(b.wait()).map(|v| *v).map_err(|_| ()));
        std::thread::sleep(Duration::from_millis(50));
        slot.lock().unwrap().take().unwrap().finish(Ok(42));
        assert_eq!(waiter.join().unwrap(), Ok(42));
    });
    assert_eq!(block_on(a.wait()), Ok(& 42));
}
#[test]
fn dropping_a_wait_future_does_not_affect_the_flight() {
    let sf = Flights::new();
    let slot: Slot = Mutex::new(None);
    let a = sf.join_or_start(key(), park(&slot));
    {
        let fut = a.wait();
        drop(fut);
    }
    assert!(sf.is_in_flight(& key()));
    slot.lock().unwrap().take().unwrap().finish(Ok(5));
    assert_eq!(block_on(a.wait()), Ok(& 5));
}
#[test]
fn concurrent_callers_produce_exactly_one_starter() {
    const THREADS: usize = 16;
    let sf = Flights::new();
    let slot: Slot = Mutex::new(None);
    let starts = AtomicUsize::new(0);
    let barrier = Barrier::new(THREADS);
    let flights: Vec<Flight<u32, NoClone>> = std::thread::scope(|s| {
        let handles: Vec<_> = (0..THREADS)
            .map(|_| {
                s.spawn(|| {
                    barrier.wait();
                    sf.join_or_start(
                        key(),
                        |c| {
                            starts.fetch_add(1, Ordering::SeqCst);
                            *slot.lock().unwrap() = Some(c);
                        },
                    )
                })
            })
            .collect();
        handles.into_iter().map(|h| h.join().unwrap()).collect()
    });
    assert_eq!(starts.load(Ordering::SeqCst), 1);
    let starters = flights.iter().filter(|f| f.role() == FlightRole::Starter).count();
    assert_eq!(starters, 1);
    slot.lock().unwrap().take().unwrap().finish(Ok(9));
    for f in &flights {
        assert_eq!(block_on(f.wait()), Ok(& 9));
    }
    assert_eq!(sf.in_flight(), 0);
}
