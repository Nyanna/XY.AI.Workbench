use super::*;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::thread;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050 as l050;
use l050::a::table_swap::{HostRootSlot, RootSlot, TableSink};
use l050::l040::a::allocator::Allocator;
use l050::l040::l030::a::bookkeeping::Bookkeeping;
use l050::l040::l030::a::region_handle::Generation;
use l050::l040::l030::l020::a::arenas::{ArenaCapacities, ArenaKind, ArenaSet, Budget};
use l050::l040::l030::l020::l010::device_lane_event::{DeviceKind, Event};
use static_assertions::assert_impl_all;
use std::sync::mpsc;
use std::time::Duration;
struct NullSink;
impl TableSink for NullSink {
    fn write_visible(
        &self,
        _region: &RegionHandle,
        _bytes: &[u8],
    ) -> Result<(), String> {
        Ok(())
    }
}
fn make_swap() -> Arc<TableSwap> {
    let mut caps = ArenaCapacities::default();
    caps.set(ArenaKind::IndexSegments, 1 << 16);
    let set = Arc::new(ArenaSet::new(caps));
    let budget = Budget {
        free: u64::MAX / 2,
        total: u64::MAX / 2,
        generation_reserve: 0,
    };
    let arena = set.ensure(ArenaKind::IndexSegments, &budget).unwrap();
    let bookkeeping = Arc::new(Bookkeeping::new(set));
    let alloc = Arc::new(Allocator::new(arena, DeviceKind::Gpu, bookkeeping));
    Arc::new(
        TableSwap::new(
            alloc,
            Arc::new(NullSink) as Arc<dyn TableSink>,
            Arc::new(HostRootSlot::new()) as Arc<dyn RootSlot>,
        ),
    )
}
fn publish(swap: &TableSwap) -> Epoch {
    let mut table = swap.next_table();
    table.add(handle(0, 1));
    swap.publish(table).unwrap()
}
struct Setup {
    swap: Arc<TableSwap>,
    reader: ReaderId,
    launcher: Launcher,
}
fn setup_with(warm: Arc<dyn WarmUp>) -> Setup {
    let swap = make_swap();
    let reader = swap.register_reader();
    let launcher = Launcher::new(Arc::clone(&swap), reader, warm);
    Setup { swap, reader, launcher }
}
fn setup() -> Setup {
    setup_with(Arc::new(NoWarmUp))
}
fn handle(offset: u64, generation: u64) -> RegionHandle {
    RegionHandle::new(ArenaKind::IndexSegments, offset, 64, Generation(generation))
        .unwrap()
}
fn key_of(op: &str) -> LaunchKey {
    LaunchKey::new(
        op,
        100,
        &[handle(0, 1)],
        DType::F32,
        Variant::default(),
        LaneKind::Query,
    )
}
#[derive(Default)]
struct RecordingLane {
    calls: Mutex<Vec<(usize, LaunchArgs)>>,
    fail: Option<String>,
    event: Option<Event>,
}
impl LaunchLane<Vec<u8>> for RecordingLane {
    fn launch(&self, item: &Vec<u8>, args: &LaunchArgs) -> Result<(), String> {
        lock(&self.calls).push((item as *const Vec<u8> as usize, *args));
        match &self.fail {
            Some(text) => Err(text.clone()),
            None => Ok(()),
        }
    }
}
impl RecordingLane {
    fn count(&self) -> usize {
        lock(&self.calls).len()
    }
}
#[derive(Default)]
struct CountingWarmUp {
    calls: AtomicUsize,
    fail: AtomicBool,
}
impl WarmUp for CountingWarmUp {
    fn warm_up(&self, _key: &LaunchKey) -> Result<(), String> {
        self.calls.fetch_add(1, Ordering::SeqCst);
        if self.fail.load(Ordering::SeqCst) {
            Err("warm boom".to_string())
        } else {
            Ok(())
        }
    }
}
#[test]
fn launcher_is_send_and_sync() {
    assert_impl_all!(Launcher : Send, Sync);
    assert_impl_all!(LaunchError : Send, Sync, std::error::Error);
}
#[test]
fn shape_class_small_values() {
    assert_eq!(ShapeClass::of(0).get(), 1);
    assert_eq!(ShapeClass::of(1).get(), 1);
    assert_eq!(ShapeClass::of(2).get(), 2);
    assert_eq!(ShapeClass::of(3).get(), 4);
    assert_eq!(ShapeClass::of(4).get(), 4);
    assert_eq!(ShapeClass::of(5).get(), 8);
}
#[test]
fn shape_class_is_smallest_power_of_two_not_below_extent() {
    for n in 1u64..=4096 {
        let class = u64::from(ShapeClass::of(n).get());
        assert!(class.is_power_of_two());
        assert!(class >= n);
        assert!(class / 2 < n);
    }
}
#[test]
fn shape_class_upper_bound_saturates() {
    let max = 1u64 << 31;
    assert_eq!(ShapeClass::of(max - 1), ShapeClass::MAX);
    assert_eq!(ShapeClass::of(max), ShapeClass::MAX);
    assert_eq!(ShapeClass::of(max + 1), ShapeClass::MAX);
    assert_eq!(ShapeClass::of(u64::MAX), ShapeClass::MAX);
    assert_eq!(ShapeClass::MAX.get(), 1 << 31);
}
#[test]
fn shape_class_orders_by_size() {
    assert!(ShapeClass::of(3) < ShapeClass::of(5));
}
#[test]
fn equal_inputs_give_equal_keys() {
    assert_eq!(key_of("score"), key_of("score"));
    let mut set = HashSet::new();
    set.insert(key_of("score"));
    assert!(set.contains(& key_of("score")));
}
#[test]
fn key_uses_shape_class_of_extent() {
    let a = LaunchKey::new(
        "op",
        5,
        &[],
        DType::F32,
        Variant::default(),
        LaneKind::Query,
    );
    let b = LaunchKey::new(
        "op",
        8,
        &[],
        DType::F32,
        Variant::default(),
        LaneKind::Query,
    );
    let c = LaunchKey::new(
        "op",
        9,
        &[],
        DType::F32,
        Variant::default(),
        LaneKind::Query,
    );
    assert_eq!(a, b);
    assert_ne!(b, c);
    assert_eq!(a.shape.get(), 8);
}
#[test]
fn different_generation_gives_different_key() {
    let new = |generation| {
        LaunchKey::new(
            "op",
            10,
            &[handle(0, generation)],
            DType::F32,
            Variant::default(),
            LaneKind::Query,
        )
    };
    assert_ne!(new(1), new(2));
    assert_eq!(new(1), new(1));
}
#[test]
fn region_order_is_significant() {
    let new = |regions: &[RegionHandle]| {
        LaunchKey::new(
            "op",
            10,
            regions,
            DType::F32,
            Variant::default(),
            LaneKind::Query,
        )
    };
    let a = handle(0, 1);
    let b = handle(64, 1);
    assert_ne!(new(& [a, b]), new(& [b, a]));
    assert_eq!(new(& [a, b]).regions, vec![a, b]);
}
#[test]
fn every_key_field_counts() {
    let base = key_of("op");
    let mut other = base.clone();
    other.operation = "other".to_string();
    assert_ne!(base, other);
    let mut other = base.clone();
    other.dtype = DType::F16;
    assert_ne!(base, other);
    let mut other = base.clone();
    other.variant = Variant {
        dim: Some(384),
        tile: None,
    };
    assert_ne!(base, other);
    let mut other = base.clone();
    other.lane_class = LaneKind::Insert;
    assert_ne!(base, other);
}
#[test]
fn graph_capture_is_rejected_without_side_effects() {
    let warm = Arc::new(CountingWarmUp::default());
    let s = setup_with(Arc::clone(&warm) as Arc<dyn WarmUp>);
    let lane = RecordingLane::default();
    let key = key_of("op");
    let err = s
        .launcher
        .launch(LaunchMode::GraphCapture, &key, &lane, &vec![1u8])
        .unwrap_err();
    assert_eq!(
        err, LaunchError::NotAvailable { mode : LaunchMode::GraphCapture, reason :
        "Graph-Capture ist nicht aktiviert".to_string(), }
    );
    assert_eq!(err.to_string(), "nicht verfügbar: Graph-Capture ist nicht aktiviert");
    assert_eq!(lane.count(), 0);
    assert_eq!(warm.calls.load(Ordering::SeqCst), 0);
    assert!(! s.launcher.is_warm(& key));
}
#[test]
fn persistent_is_rejected_without_side_effects() {
    let warm = Arc::new(CountingWarmUp::default());
    let s = setup_with(Arc::clone(&warm) as Arc<dyn WarmUp>);
    let lane = RecordingLane::default();
    let key = key_of("op");
    let err = s
        .launcher
        .launch(LaunchMode::Persistent, &key, &lane, &vec![1u8])
        .unwrap_err();
    assert_eq!(
        err, LaunchError::NotAvailable { mode : LaunchMode::Persistent, reason :
        "Persistent-Modus ist nicht aktiviert".to_string(), }
    );
    assert_eq!(
        err.to_string(), "nicht verfügbar: Persistent-Modus ist nicht aktiviert"
    );
    assert_eq!(lane.count(), 0);
    assert_eq!(warm.calls.load(Ordering::SeqCst), 0);
    assert!(! s.launcher.is_warm(& key));
}
#[test]
fn rejection_does_not_touch_unknown_reader() {
    let swap = make_swap();
    publish(&swap);
    let launcher = Launcher::new(swap, ReaderId(999), Arc::new(NoWarmUp));
    let lane = RecordingLane::default();
    let result = launcher.launch(LaunchMode::Persistent, &key_of("op"), &lane, &vec![]);
    assert!(matches!(result, Err(LaunchError::NotAvailable { .. })));
}
#[test]
fn first_launch_is_not_productive_then_productive() {
    let s = setup();
    let lane = RecordingLane::default();
    let key = key_of("op");
    let item = vec![1u8, 2];
    assert!(! s.launcher.is_warm(& key));
    let first = s.launcher.launch(LaunchMode::Direct, &key, &lane, &item).unwrap();
    assert!(! first.productive);
    assert!(s.launcher.is_warm(& key));
    let second = s.launcher.launch(LaunchMode::Direct, &key, &lane, &item).unwrap();
    assert!(second.productive);
    assert_eq!(lane.count(), 2);
}
#[test]
fn warm_up_runs_exactly_once_per_key() {
    let warm = Arc::new(CountingWarmUp::default());
    let s = setup_with(Arc::clone(&warm) as Arc<dyn WarmUp>);
    let lane = RecordingLane::default();
    let key = key_of("op");
    for _ in 0..3 {
        s.launcher.launch(LaunchMode::Direct, &key, &lane, &vec![]).unwrap();
    }
    assert_eq!(warm.calls.load(Ordering::SeqCst), 1);
    s.launcher.launch(LaunchMode::Direct, &key_of("other"), &lane, &vec![]).unwrap();
    assert_eq!(warm.calls.load(Ordering::SeqCst), 2);
}
#[test]
fn new_generation_key_warms_up_again() {
    let s = setup();
    let lane = RecordingLane::default();
    let key = |generation| {
        LaunchKey::new(
            "op",
            10,
            &[handle(0, generation)],
            DType::F32,
            Variant::default(),
            LaneKind::Query,
        )
    };
    assert!(
        ! s.launcher.launch(LaunchMode::Direct, & key(1), & lane, & vec![]).unwrap()
        .productive
    );
    assert!(
        ! s.launcher.launch(LaunchMode::Direct, & key(2), & lane, & vec![]).unwrap()
        .productive
    );
    assert!(
        s.launcher.launch(LaunchMode::Direct, & key(2), & lane, & vec![]).unwrap()
        .productive
    );
}
#[test]
fn failing_warm_up_removes_key_and_skips_lane() {
    let warm = Arc::new(CountingWarmUp::default());
    warm.fail.store(true, Ordering::SeqCst);
    let s = setup_with(Arc::clone(&warm) as Arc<dyn WarmUp>);
    let lane = RecordingLane::default();
    let key = key_of("op");
    let err = s.launcher.launch(LaunchMode::Direct, &key, &lane, &vec![]).unwrap_err();
    assert_eq!(err, LaunchError::WarmUp("warm boom".to_string()));
    assert_eq!(lane.count(), 0);
    assert!(! s.launcher.is_warm(& key));
    warm.fail.store(false, Ordering::SeqCst);
    let report = s.launcher.launch(LaunchMode::Direct, &key, &lane, &vec![]).unwrap();
    assert!(! report.productive);
    assert_eq!(warm.calls.load(Ordering::SeqCst), 2);
    assert_eq!(lane.count(), 1);
}
struct LockProbe {
    launcher: Arc<Mutex<Option<Arc<Launcher>>>>,
    observed_warm: AtomicBool,
    reached: AtomicBool,
}
impl WarmUp for LockProbe {
    fn warm_up(&self, key: &LaunchKey) -> Result<(), String> {
        let launcher = lock(&self.launcher).clone().unwrap();
        self.observed_warm.store(launcher.is_warm(key), Ordering::SeqCst);
        self.reached.store(true, Ordering::SeqCst);
        Ok(())
    }
}
#[test]
fn warm_up_runs_without_held_lock() {
    let swap = make_swap();
    let reader = swap.register_reader();
    let cell = Arc::new(Mutex::new(None));
    let probe = Arc::new(LockProbe {
        launcher: Arc::clone(&cell),
        observed_warm: AtomicBool::new(false),
        reached: AtomicBool::new(false),
    });
    let launcher = Arc::new(
        Launcher::new(swap, reader, Arc::clone(&probe) as Arc<dyn WarmUp>),
    );
    *lock(&cell) = Some(Arc::clone(&launcher));
    let lane = RecordingLane::default();
    let key = key_of("op");
    launcher.launch(LaunchMode::Direct, &key, &lane, &vec![]).unwrap();
    assert!(probe.reached.load(Ordering::SeqCst));
    assert!(! probe.observed_warm.load(Ordering::SeqCst));
    assert!(launcher.is_warm(& key));
    *lock(&cell) = None;
}
#[test]
fn concurrent_first_launches_have_exactly_one_non_productive() {
    let warm = Arc::new(CountingWarmUp::default());
    let s = setup_with(Arc::clone(&warm) as Arc<dyn WarmUp>);
    publish(&s.swap);
    let launcher = Arc::new(s.launcher);
    let lane = Arc::new(RecordingLane::default());
    let key = key_of("op");
    let threads: Vec<_> = (0..8)
        .map(|_| {
            let launcher = Arc::clone(&launcher);
            let lane = Arc::clone(&lane);
            let key = key.clone();
            thread::spawn(move || {
                launcher
                    .launch(LaunchMode::Direct, &key, lane.as_ref(), &vec![0u8])
                    .unwrap()
                    .productive
            })
        })
        .collect();
    let flags: Vec<bool> = threads.into_iter().map(|t| t.join().unwrap()).collect();
    assert_eq!(flags.iter().filter(| p | !** p).count(), 1);
    assert_eq!(warm.calls.load(Ordering::SeqCst), 1);
    assert_eq!(lane.count(), 8);
}
#[test]
fn no_table_yields_sentinel_args() {
    let s = setup();
    let lane = RecordingLane::default();
    let report = s
        .launcher
        .launch(LaunchMode::Direct, &key_of("op"), &lane, &vec![])
        .unwrap();
    assert_eq!(report.args, LaunchArgs { root : ROOT_SENTINEL, epoch : 0 });
    assert_eq!(s.launcher.current_args().unwrap(), report.args);
}
#[test]
fn args_follow_publish_from_next_launch() {
    let s = setup();
    let lane = RecordingLane::default();
    let key = key_of("op");
    let item = vec![];
    let before = s.launcher.launch(LaunchMode::Direct, &key, &lane, &item).unwrap();
    assert_eq!(before.args.root, ROOT_SENTINEL);
    let epoch = publish(&s.swap);
    let after = s.launcher.launch(LaunchMode::Direct, &key, &lane, &item).unwrap();
    assert_eq!(after.args.epoch, epoch);
    assert_eq!(after.args.root, s.swap.root_value());
    assert_ne!(after.args.root, ROOT_SENTINEL);
    let epoch2 = publish(&s.swap);
    let latest = s.launcher.launch(LaunchMode::Direct, &key, &lane, &item).unwrap();
    assert_eq!(latest.args.epoch, epoch2);
    assert_eq!(latest.args.root, s.swap.root_value());
    let calls = lock(&lane.calls);
    assert_eq!(calls[0].1, before.args);
    assert_eq!(calls[1].1, after.args);
    assert_eq!(calls[2].1, latest.args);
}
#[test]
fn lane_receives_same_item_and_reported_args() {
    let s = setup();
    publish(&s.swap);
    let lane = RecordingLane::default();
    let item = vec![7u8; 4];
    let report = s
        .launcher
        .launch(LaunchMode::Direct, &key_of("op"), &lane, &item)
        .unwrap();
    let calls = lock(&lane.calls);
    assert_eq!(calls.len(), 1);
    assert_eq!(calls[0].0, & item as * const Vec < u8 > as usize);
    assert_eq!(calls[0].1, report.args);
}
#[test]
fn lane_error_becomes_kernel_error_without_retry() {
    let s = setup();
    let lane = RecordingLane {
        fail: Some("lane exploded".to_string()),
        ..RecordingLane::default()
    };
    let err = s
        .launcher
        .launch(LaunchMode::Direct, &key_of("op"), &lane, &vec![])
        .unwrap_err();
    assert_eq!(err, LaunchError::Kernel("lane exploded".to_string()));
    assert!(err.to_string().contains("lane exploded"));
    assert_eq!(lane.count(), 1);
}
#[test]
fn unknown_reader_is_table_error_and_lane_is_skipped() {
    let swap = make_swap();
    publish(&swap);
    let launcher = Launcher::new(swap, ReaderId(4242), Arc::new(NoWarmUp));
    let lane = RecordingLane::default();
    let err = launcher
        .launch(LaunchMode::Direct, &key_of("op"), &lane, &vec![])
        .unwrap_err();
    assert_eq!(err, LaunchError::Table(TableSwapError::UnknownReader(ReaderId(4242))));
    assert_eq!(lane.count(), 0);
    assert_eq!(
        launcher.current_args(),
        Err(LaunchError::Table(TableSwapError::UnknownReader(ReaderId(4242))))
    );
}
#[test]
fn launcher_reports_epoch_but_leaves_quiescence_to_caller() {
    let s = setup();
    let epoch = publish(&s.swap);
    assert_eq!(s.swap.reader_epoch(s.reader), Ok(0));
    let lane = RecordingLane::default();
    s.launcher.launch(LaunchMode::Direct, &key_of("op"), &lane, &vec![]).unwrap();
    assert_eq!(s.swap.reader_epoch(s.reader), Ok(epoch));
    s.swap.set_quiescent(s.reader, true).unwrap();
    assert_eq!(s.swap.retired_count(), 0);
}
struct GuardProbe {
    swap: Arc<TableSwap>,
    seen_min: Mutex<Vec<Epoch>>,
}
impl LaunchLane<()> for GuardProbe {
    fn launch(&self, _item: &(), _args: &LaunchArgs) -> Result<(), String> {
        lock(&self.seen_min).push(self.swap.min_reader_epoch());
        Ok(())
    }
}
#[test]
fn guard_is_held_during_lane_call() {
    let s = setup();
    let first = publish(&s.swap);
    let probe = GuardProbe {
        swap: Arc::clone(&s.swap),
        seen_min: Mutex::new(Vec::new()),
    };
    s.launcher.launch(LaunchMode::Direct, &key_of("op"), &probe, &()).unwrap();
    publish(&s.swap);
    assert_eq!(lock(& probe.seen_min) [0], first);
}
#[test]
fn lane_may_hold_an_event_for_completion_tracking() {
    let (event, control) = Event::mock();
    let lane = RecordingLane {
        event: Some(event),
        ..RecordingLane::default()
    };
    let s = setup();
    s.launcher.launch(LaunchMode::Direct, &key_of("op"), &lane, &vec![]).unwrap();
    control.complete();
    assert_eq!(lane.event.as_ref().unwrap().is_complete(), Ok(true));
}
#[test]
fn poisoned_seen_lock_is_taken_over() {
    let s = setup();
    let launcher = Arc::new(s.launcher);
    let poisoner = Arc::clone(&launcher);
    let _ = thread::spawn(move || {
            let _guard = poisoner.seen.lock().unwrap();
            panic!("poison");
        })
        .join();
    assert!(launcher.seen.is_poisoned());
    let lane = RecordingLane::default();
    let key = key_of("op");
    let report = launcher.launch(LaunchMode::Direct, &key, &lane, &vec![]).unwrap();
    assert!(! report.productive);
    assert!(launcher.is_warm(& key));
}
#[test]
fn parallel_launches_over_different_keys() {
    let s = setup();
    publish(&s.swap);
    let launcher = Arc::new(s.launcher);
    let lane = Arc::new(RecordingLane::default());
    let threads: Vec<_> = (0..8)
        .map(|i| {
            let launcher = Arc::clone(&launcher);
            let lane = Arc::clone(&lane);
            thread::spawn(move || {
                let key = key_of(&format!("op{i}"));
                let first = launcher
                    .launch(LaunchMode::Direct, &key, lane.as_ref(), &vec![])
                    .unwrap();
                let second = launcher
                    .launch(LaunchMode::Direct, &key, lane.as_ref(), &vec![])
                    .unwrap();
                (first.productive, second.productive)
            })
        })
        .collect();
    for t in threads {
        assert_eq!(t.join().unwrap(), (false, true));
    }
    assert_eq!(lane.count(), 16);
}
#[test]
fn no_warm_up_still_reports_first_launch_as_non_productive() {
    let s = setup();
    let lane = RecordingLane::default();
    let key = key_of("op");
    let report = s.launcher.launch(LaunchMode::Direct, &key, &lane, &vec![]).unwrap();
    assert!(! report.productive);
}
#[test]
fn launch_report_is_copy_and_comparable() {
    let args = LaunchArgs { root: 1, epoch: 2 };
    let report = LaunchReport {
        args,
        productive: true,
    };
    let copy = report;
    assert_eq!(report, copy);
}
enum FirstCall {
    Succeed,
    Fail,
    Panic,
}
/// The first call announces itself, waits for the gate, then behaves as configured.
struct GatedWarmUp {
    calls: AtomicUsize,
    first: FirstCall,
    entered: Mutex<mpsc::Sender<()>>,
    gate: Mutex<mpsc::Receiver<()>>,
}
impl WarmUp for GatedWarmUp {
    fn warm_up(&self, _key: &LaunchKey) -> Result<(), String> {
        if self.calls.fetch_add(1, Ordering::SeqCst) > 0 {
            return Ok(());
        }
        lock(&self.entered).send(()).unwrap();
        lock(&self.gate).recv().unwrap();
        match self.first {
            FirstCall::Succeed => Ok(()),
            FirstCall::Fail => Err("gated boom".to_string()),
            FirstCall::Panic => panic!("warm-up panic"),
        }
    }
}
struct Gated {
    warm: Arc<GatedWarmUp>,
    entered: mpsc::Receiver<()>,
    open: mpsc::Sender<()>,
    launcher: Arc<Launcher>,
    lane: Arc<RecordingLane>,
}
fn gated(first: FirstCall) -> Gated {
    let (entered_tx, entered) = mpsc::channel();
    let (open, gate_rx) = mpsc::channel();
    let warm = Arc::new(GatedWarmUp {
        calls: AtomicUsize::new(0),
        first,
        entered: Mutex::new(entered_tx),
        gate: Mutex::new(gate_rx),
    });
    let s = setup_with(Arc::clone(&warm) as Arc<dyn WarmUp>);
    publish(&s.swap);
    Gated {
        warm,
        entered,
        open,
        launcher: Arc::new(s.launcher),
        lane: Arc::new(RecordingLane::default()),
    }
}
fn spawn_launch(
    g: &Gated,
    key: &LaunchKey,
) -> thread::JoinHandle<Result<LaunchReport, LaunchError>> {
    let launcher = Arc::clone(&g.launcher);
    let lane = Arc::clone(&g.lane);
    let key = key.clone();
    thread::spawn(move || {
        launcher.launch(LaunchMode::Direct, &key, lane.as_ref(), &vec![0u8])
    })
}
const SETTLE: Duration = Duration::from_millis(100);
#[test]
fn same_key_launches_wait_for_running_warm_up() {
    let g = gated(FirstCall::Succeed);
    let key = key_of("op");
    let first = spawn_launch(&g, &key);
    g.entered.recv().unwrap();
    let second = spawn_launch(&g, &key);
    thread::sleep(SETTLE);
    assert!(! second.is_finished());
    assert_eq!(g.lane.count(), 0);
    assert!(! g.launcher.is_warm(& key));
    g.open.send(()).unwrap();
    assert!(! first.join().unwrap().unwrap().productive);
    assert!(second.join().unwrap().unwrap().productive);
    assert_eq!(g.lane.count(), 2);
    assert_eq!(g.warm.calls.load(Ordering::SeqCst), 1);
}
#[test]
fn other_keys_are_not_blocked_by_running_warm_up() {
    let g = gated(FirstCall::Succeed);
    let slow = key_of("slow");
    let first = spawn_launch(&g, &slow);
    g.entered.recv().unwrap();
    let other = key_of("other");
    let report = g
        .launcher
        .launch(LaunchMode::Direct, &other, g.lane.as_ref(), &vec![0u8])
        .unwrap();
    assert!(! report.productive);
    assert_eq!(g.lane.count(), 1);
    g.open.send(()).unwrap();
    first.join().unwrap().unwrap();
}
#[test]
fn failed_warm_up_hands_over_to_waiting_launch() {
    let g = gated(FirstCall::Fail);
    let key = key_of("op");
    let first = spawn_launch(&g, &key);
    g.entered.recv().unwrap();
    let second = spawn_launch(&g, &key);
    thread::sleep(SETTLE);
    assert!(! second.is_finished());
    g.open.send(()).unwrap();
    assert_eq!(
        first.join().unwrap().unwrap_err(), LaunchError::WarmUp("gated boom".to_string())
    );
    assert!(! second.join().unwrap().unwrap().productive);
    assert_eq!(g.warm.calls.load(Ordering::SeqCst), 2);
    assert_eq!(g.lane.count(), 1);
    assert!(g.launcher.is_warm(& key));
}
#[test]
fn panicking_warm_up_releases_waiting_launches() {
    let g = gated(FirstCall::Panic);
    let key = key_of("op");
    let first = spawn_launch(&g, &key);
    g.entered.recv().unwrap();
    let second = spawn_launch(&g, &key);
    thread::sleep(SETTLE);
    assert!(! second.is_finished());
    g.open.send(()).unwrap();
    assert!(first.join().is_err());
    assert!(! second.join().unwrap().unwrap().productive);
    assert!(g.launcher.is_warm(& key));
}
