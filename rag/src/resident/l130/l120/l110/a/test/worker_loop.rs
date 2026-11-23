use super::*;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::a::notification_topology::NotificationTopology;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::l010::device_lane_event::{
    DeviceHandle, Event, Lane, LaneConfig, LaneKind, MockEventControl,
};
use crate::resident::l130::l120::l110::l100::a::planner::OpMeta;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;
use std::thread;
type Log = Arc<Mutex<Vec<String>>>;
fn new_log() -> Log {
    Arc::new(Mutex::new(Vec::new()))
}
fn entries(log: &Log) -> Vec<String> {
    log.lock().unwrap().clone()
}
fn completion() -> Arc<GpuCompletion> {
    let topo = NotificationTopology::new(1, Arc::new(|| {}));
    Arc::new(GpuCompletion::new(topo.notifier(0).unwrap()))
}
fn submit(gc: &GpuCompletion, tag: u64) -> MockEventControl {
    let (event, ctl) = Event::mock();
    let lane = Lane::new(LaneConfig::preset(LaneKind::Query, DeviceHandle::Cpu), None);
    let _cb = gc.submit(lane, event, tag);
    ctl
}
struct ScriptedFeed {
    name: &'static str,
    log: Log,
    script: Vec<IterationOutcome>,
    calls: Arc<AtomicUsize>,
    on_call: Option<Box<dyn FnMut() + Send>>,
}
impl ScriptedFeed {
    fn boxed(
        name: &'static str,
        log: &Log,
        claims: &[usize],
    ) -> (Box<dyn FeedStep>, Arc<AtomicUsize>) {
        let calls = Arc::new(AtomicUsize::new(0));
        let script = claims.iter().rev().map(|&c| outcome(c)).collect();
        let feed = ScriptedFeed {
            name,
            log: Arc::clone(log),
            script,
            calls: Arc::clone(&calls),
            on_call: None,
        };
        (Box::new(feed), calls)
    }
}
fn outcome(claimed: usize) -> IterationOutcome {
    IterationOutcome {
        claimed,
        ..IterationOutcome::default()
    }
}
impl FeedStep for ScriptedFeed {
    fn feed(&mut self, _now: Instant) -> IterationOutcome {
        self.calls.fetch_add(1, Ordering::SeqCst);
        self.log.lock().unwrap().push(format!("feed:{}", self.name));
        if let Some(f) = self.on_call.as_mut() {
            f();
        }
        self.script.pop().unwrap_or_default()
    }
}
struct ScriptedExpand {
    log: Log,
    remaining: usize,
    calls: Arc<AtomicUsize>,
}
impl ExpandStep for ScriptedExpand {
    fn expand_next(&mut self) -> bool {
        self.calls.fetch_add(1, Ordering::SeqCst);
        self.log.lock().unwrap().push("expand".into());
        if self.remaining == 0 {
            return false;
        }
        self.remaining -= 1;
        true
    }
}
struct Fixture {
    worker: WorkerLoop,
    log: Log,
    handled: Arc<Mutex<Vec<(u64, bool)>>>,
    expand_calls: Arc<AtomicUsize>,
}
fn fixture(
    gc: Arc<GpuCompletion>,
    log: &Log,
    gpu: Vec<Box<dyn FeedStep>>,
    cpu: Vec<Box<dyn FeedStep>>,
    tasks: usize,
) -> Fixture {
    let handled = Arc::new(Mutex::new(Vec::new()));
    let h = Arc::clone(&handled);
    let l = Arc::clone(log);
    let expand_calls = Arc::new(AtomicUsize::new(0));
    let expand = ScriptedExpand {
        log: Arc::clone(log),
        remaining: tasks,
        calls: Arc::clone(&expand_calls),
    };
    let worker = WorkerLoop::new(
        gc,
        Box::new(move |f: FinishedLane| {
            l.lock().unwrap().push(format!("result:{}", f.tag));
            h.lock().unwrap().push((f.tag, f.result.is_ok()));
        }),
        gpu,
        cpu,
        Box::new(expand),
    );
    Fixture {
        worker,
        log: Arc::clone(log),
        handled,
        expand_calls,
    }
}
#[test]
fn worker_loop_is_send() {
    fn assert_send<T: Send>() {}
    assert_send::<WorkerLoop>();
    assert_send::<Arc<SubengineLoop<OpMeta, (), NoExec>>>();
    fn assert_feed<T: FeedStep>() {}
    assert_feed::<Arc<SubengineLoop<OpMeta, (), NoExec>>>();
}
struct NoExec;
impl ResourceExec for NoExec {
    type Entry = Claim<OpMeta, (), (), ()>;
    fn has_capacity(&self) -> bool {
        false
    }
    fn execute(
        &self,
        _entry: Self::Entry,
        _lease: crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::a::resource_lifecycle::Lease<
            Self,
        >,
    ) {}
}
#[test]
fn idle_checks_all_stages_once_in_order() {
    let log = new_log();
    let (g, gc_calls) = ScriptedFeed::boxed("g", &log, &[]);
    let (c, cc_calls) = ScriptedFeed::boxed("c", &log, &[]);
    let mut f = fixture(completion(), &log, vec![g], vec![c], 0);
    assert_eq!(f.worker.run_once(Instant::now()), None);
    assert_eq!(entries(& f.log), vec!["feed:g", "feed:c", "expand"]);
    assert_eq!(gc_calls.load(Ordering::SeqCst), 1);
    assert_eq!(cc_calls.load(Ordering::SeqCst), 1);
    assert_eq!(f.expand_calls.load(Ordering::SeqCst), 1);
}
#[test]
fn empty_worker_is_idle() {
    let log = new_log();
    let mut f = fixture(completion(), &log, vec![], vec![], 0);
    assert_eq!(f.worker.run_once(Instant::now()), None);
    assert_eq!(f.worker.run_until_idle(), WorkerReport::default());
    assert!(! WorkerReport::default().progress());
}
#[test]
fn gpu_results_have_highest_priority_and_count_as_one_operation() {
    let log = new_log();
    let gc = completion();
    let a = submit(&gc, 1);
    let b = submit(&gc, 2);
    a.complete();
    b.fail("boom");
    let (g, g_calls) = ScriptedFeed::boxed("g", &log, &[3]);
    let mut f = fixture(Arc::clone(&gc), &log, vec![g], vec![], 1);
    assert_eq!(f.worker.run_once(Instant::now()), Some(Stage::GpuResults));
    assert_eq!(g_calls.load(Ordering::SeqCst), 0);
    assert_eq!(f.expand_calls.load(Ordering::SeqCst), 0);
    assert_eq!(* f.handled.lock().unwrap(), vec![(1, true), (2, false)]);
    assert_eq!(gc.in_flight(), 0);
}
#[test]
fn stage_order_is_results_gpu_cpu_expand() {
    let log = new_log();
    let gc = completion();
    submit(&gc, 7).complete();
    let (g, _) = ScriptedFeed::boxed("g", &log, &[1]);
    let (c, _) = ScriptedFeed::boxed("c", &log, &[2]);
    let mut f = fixture(gc, &log, vec![g], vec![c], 1);
    let mut stages = Vec::new();
    while let Some(s) = f.worker.run_once(Instant::now()) {
        stages.push(s);
    }
    assert_eq!(
        stages, vec![Stage::GpuResults, Stage::GpuFeed, Stage::CpuFeed, Stage::Expand]
    );
}
#[test]
fn higher_stage_is_rechecked_after_every_operation() {
    let log = new_log();
    let (g, _) = ScriptedFeed::boxed("g", &log, &[1, 1, 0]);
    let (c, _) = ScriptedFeed::boxed("c", &log, &[1]);
    let mut f = fixture(completion(), &log, vec![g], vec![c], 0);
    let mut stages = Vec::new();
    while let Some(s) = f.worker.run_once(Instant::now()) {
        stages.push(s);
    }
    assert_eq!(stages, vec![Stage::GpuFeed, Stage::GpuFeed, Stage::CpuFeed]);
    assert_eq!(
        entries(& f.log), vec!["feed:g", "feed:g", "feed:g", "feed:c", "feed:g",
        "feed:c", "expand"]
    );
}
#[test]
fn new_gpu_result_during_feed_preempts_lower_stages() {
    let log = new_log();
    let gc = completion();
    let (event, ctl) = Event::mock();
    let lane = Lane::new(LaneConfig::preset(LaneKind::Query, DeviceHandle::Cpu), None);
    let cb = gc.submit(lane, event, 9);
    let mut ctl = Some(ctl);
    let mut cb = Some(cb);
    let gfeed = ScriptedFeed {
        name: "g",
        log: Arc::clone(&log),
        script: vec![outcome(1)],
        calls: Arc::new(AtomicUsize::new(0)),
        on_call: Some(
            Box::new(move || {
                if let Some(c) = ctl.take() {
                    c.complete();
                }
                if let Some(cb) = cb.take() {
                    cb.fire();
                }
            }),
        ),
    };
    let (c, _) = ScriptedFeed::boxed("c", &log, &[1]);
    let mut f = fixture(Arc::clone(&gc), &log, vec![Box::new(gfeed)], vec![c], 0);
    assert_eq!(f.worker.run_once(Instant::now()), Some(Stage::GpuFeed));
    assert_eq!(f.worker.run_once(Instant::now()), Some(Stage::GpuResults));
    assert_eq!(f.worker.run_once(Instant::now()), Some(Stage::CpuFeed));
    assert_eq!(f.worker.run_once(Instant::now()), None);
}
#[test]
fn feed_stage_stops_after_first_entry_with_progress() {
    let log = new_log();
    let (a, a_calls) = ScriptedFeed::boxed("a", &log, &[0]);
    let (b, b_calls) = ScriptedFeed::boxed("b", &log, &[2]);
    let (c, c_calls) = ScriptedFeed::boxed("c", &log, &[5]);
    let mut f = fixture(completion(), &log, vec![a, b, c], vec![], 0);
    assert_eq!(f.worker.run_once(Instant::now()), Some(Stage::GpuFeed));
    assert_eq!(a_calls.load(Ordering::SeqCst), 1);
    assert_eq!(b_calls.load(Ordering::SeqCst), 1);
    assert_eq!(c_calls.load(Ordering::SeqCst), 0);
    assert_eq!(entries(& f.log), vec!["feed:a", "feed:b"]);
}
#[test]
fn cpu_feed_stage_uses_cpu_feeds_in_order() {
    let log = new_log();
    let (a, _) = ScriptedFeed::boxed("a", &log, &[0]);
    let (b, b_calls) = ScriptedFeed::boxed("b", &log, &[1]);
    let (c, c_calls) = ScriptedFeed::boxed("c", &log, &[1]);
    let mut f = fixture(completion(), &log, vec![], vec![a, b, c], 0);
    assert_eq!(f.worker.run_once(Instant::now()), Some(Stage::CpuFeed));
    assert_eq!(b_calls.load(Ordering::SeqCst), 1);
    assert_eq!(c_calls.load(Ordering::SeqCst), 0);
}
#[test]
fn inits_idle_and_evicted_alone_are_not_progress() {
    let log = new_log();
    let calls = Arc::new(AtomicUsize::new(0));
    let noisy = ScriptedFeed {
        name: "n",
        log: Arc::clone(&log),
        script: vec![
            IterationOutcome { claimed : 0, inits_started : 3, idle : Vec::new(), evicted
            : Vec::new(), }
        ],
        calls: Arc::clone(&calls),
        on_call: None,
    };
    let mut f = fixture(completion(), &log, vec![Box::new(noisy)], vec![], 0);
    assert_eq!(f.worker.run_once(Instant::now()), None);
    assert_eq!(calls.load(Ordering::SeqCst), 1);
}
#[test]
fn failed_expansion_counts_as_consumed_and_loop_terminates() {
    let log = new_log();
    let mut f = fixture(completion(), &log, vec![], vec![], 5);
    let report = f.worker.run_until_idle();
    assert_eq!(
        report, WorkerReport { operations : 5, gpu_results : 0, claimed : 0, expanded :
        5, }
    );
    assert!(report.progress());
    assert_eq!(f.expand_calls.load(Ordering::SeqCst), 6);
}
#[test]
fn report_counts_all_counters() {
    let log = new_log();
    let gc = completion();
    submit(&gc, 1).complete();
    submit(&gc, 2).complete();
    let (g, _) = ScriptedFeed::boxed("g", &log, &[2, 0, 0]);
    let (c, _) = ScriptedFeed::boxed("c", &log, &[3]);
    let mut f = fixture(gc, &log, vec![g], vec![c], 2);
    let report = f.worker.run_until_idle();
    assert_eq!(
        report, WorkerReport { operations : 1 + 1 + 1 + 2, gpu_results : 2, claimed : 5,
        expanded : 2, }
    );
}
#[test]
fn claimed_includes_calls_without_progress_and_with_progress() {
    let log = new_log();
    let (a, _) = ScriptedFeed::boxed("a", &log, &[0, 0]);
    let (b, _) = ScriptedFeed::boxed("b", &log, &[4, 0]);
    let mut f = fixture(completion(), &log, vec![a, b], vec![], 0);
    let report = f.worker.run_until_idle();
    assert_eq!(report.claimed, 4);
    assert_eq!(report.operations, 1);
}
#[test]
fn run_until_idle_ends_only_after_a_full_idle_pass() {
    let log = new_log();
    let (g, _) = ScriptedFeed::boxed("g", &log, &[1, 0]);
    let mut f = fixture(completion(), &log, vec![g], vec![], 0);
    f.worker.run_until_idle();
    assert_eq!(entries(& f.log), vec!["feed:g", "feed:g", "expand"]);
}
#[test]
fn handler_receives_lanes_in_submit_order_and_errors_unchanged() {
    let log = new_log();
    let gc = completion();
    let c0 = submit(&gc, 10);
    let c1 = submit(&gc, 11);
    let c2 = submit(&gc, 12);
    c2.complete();
    c1.fail("bad");
    c0.complete();
    let mut f = fixture(gc, &log, vec![], vec![], 0);
    let report = f.worker.run_until_idle();
    assert_eq!(report.gpu_results, 3);
    assert_eq!(report.operations, 1);
    assert_eq!(* f.handled.lock().unwrap(), vec![(10, true), (11, false), (12, true)]);
}
#[test]
fn pending_lane_is_not_reported_until_finished() {
    let log = new_log();
    let gc = completion();
    let ctl = submit(&gc, 1);
    let mut f = fixture(Arc::clone(&gc), &log, vec![], vec![], 0);
    assert_eq!(f.worker.run_once(Instant::now()), None);
    ctl.complete();
    assert_eq!(f.worker.run_once(Instant::now()), Some(Stage::GpuResults));
    assert_eq!(f.worker.run_once(Instant::now()), None);
}
#[test]
fn shared_completion_hands_each_lane_to_exactly_one_worker() {
    let gc = completion();
    const LANES: u64 = 64;
    for tag in 0..LANES {
        submit(&gc, tag).complete();
    }
    let seen = Arc::new(Mutex::new(Vec::new()));
    let threads: Vec<_> = (0..4)
        .map(|_| {
            let gc = Arc::clone(&gc);
            let seen = Arc::clone(&seen);
            thread::spawn(move || {
                let mut worker = WorkerLoop::new(
                    gc,
                    Box::new(move |f| seen.lock().unwrap().push(f.tag)),
                    vec![],
                    vec![],
                    Box::new(ScriptedExpand {
                        log: new_log(),
                        remaining: 0,
                        calls: Arc::new(AtomicUsize::new(0)),
                    }),
                );
                worker.run_until_idle().gpu_results
            })
        })
        .collect();
    let total: usize = threads.into_iter().map(|t| t.join().unwrap()).sum();
    assert_eq!(total, LANES as usize);
    let mut tags = seen.lock().unwrap().clone();
    tags.sort_unstable();
    assert_eq!(tags, (0..LANES).collect::< Vec < _ >> ());
}
