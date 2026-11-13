use super::*;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::a::region_handle::Generation;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::a::arenas::ArenaKind;
use std::collections::VecDeque;
use std::sync::Condvar;

fn region(offset: u64, len: u64) -> RegionHandle {
    RegionHandle::new(ArenaKind::MappedHost, offset, len, Generation(0)).unwrap()
}

fn spec(queries: u32) -> BatchSpec {
    BatchSpec {
        segments: vec![SegmentDesc {
            base: 0,
            entries: 4,
            layout: SegmentLayout::Fixed { stride: 16 },
        }],
        query_count: queries,
        params: None,
        time_budget: Duration::from_millis(7),
    }
}

fn inputs() -> BatchInputs {
    BatchInputs { regions: Vec::new() }
}

struct Script {
    outcomes: Mutex<VecDeque<Result<LaunchOutcome, String>>>,
    calls: Mutex<Vec<(u32, Vec<u64>, Duration)>>,
    write: u64,
}

impl BatchExecutor for Arc<Script> {
    fn launch(
        &self,
        _id: BatchId,
        spec: &BatchSpec,
        _inputs: &BatchInputs,
        queries_done: u32,
        ranges: &mut [RangeCursor],
    ) -> Result<LaunchOutcome, String> {
        self.calls.lock().unwrap().push((
            queries_done,
            ranges.iter().map(|r| r.region().len()).collect(),
            spec.time_budget,
        ));
        for r in ranges.iter_mut() {
            let _ = r.advance(self.write);
        }
        self.outcomes.lock().unwrap().pop_front().unwrap_or(Ok(LaunchOutcome::Hits { hits: 0 }))
    }
}

fn scripted(write: u64, outcomes: Vec<Result<LaunchOutcome, String>>) -> Arc<Script> {
    Arc::new(Script {
        outcomes: Mutex::new(outcomes.into()),
        calls: Mutex::new(Vec::new()),
        write,
    })
}

#[derive(Debug, Default)]
struct Counter(AtomicU64);

impl ProgressCounter for Counter {
    fn load(&self) -> u64 {
        self.0.load(Ordering::SeqCst)
    }
}

struct PanicExec;

impl BatchExecutor for PanicExec {
    fn launch(
        &self,
        _id: BatchId,
        _spec: &BatchSpec,
        _inputs: &BatchInputs,
        _queries_done: u32,
        _ranges: &mut [RangeCursor],
    ) -> Result<LaunchOutcome, String> {
        panic!("boom");
    }
}

struct Gate {
    entered: (Mutex<bool>, Condvar),
    release: (Mutex<bool>, Condvar),
}

impl BatchExecutor for Arc<Gate> {
    fn launch(
        &self,
        _id: BatchId,
        _spec: &BatchSpec,
        _inputs: &BatchInputs,
        _queries_done: u32,
        _ranges: &mut [RangeCursor],
    ) -> Result<LaunchOutcome, String> {
        *self.entered.0.lock().unwrap() = true;
        self.entered.1.notify_all();
        let mut released = self.release.0.lock().unwrap();
        while !*released {
            released = self.release.1.wait(released).unwrap();
        }
        Ok(LaunchOutcome::Hits { hits: 1 })
    }
}

#[test]
fn fixed_entry_of() {
    let d = SegmentDesc { base: 100, entries: 4, layout: SegmentLayout::Fixed { stride: 10 } };
    assert_eq!(d.entry_of(99, &[]), None);
    assert_eq!(d.entry_of(100, &[]), Some(0));
    assert_eq!(d.entry_of(109, &[]), Some(0));
    assert_eq!(d.entry_of(110, &[]), Some(1));
    assert_eq!(d.entry_of(139, &[]), Some(3));
    assert_eq!(d.entry_of(140, &[]), None);
    assert_eq!(d.entry_of(u64::MAX, &[]), None);
    assert!(d.offsets_valid(&[]));
    let zero = SegmentDesc { layout: SegmentLayout::Fixed { stride: 0 }, ..d };
    assert_eq!(zero.entry_of(100, &[]), None);
    let empty = SegmentDesc { entries: 0, ..d };
    assert_eq!(empty.entry_of(100, &[]), None);
}

#[test]
fn variable_entry_of() {
    let d = SegmentDesc {
        base: 100,
        entries: 4,
        layout: SegmentLayout::Variable { offsets: region(0, 40) },
    };
    let offsets = [0u64, 10, 10, 25, 40];
    assert!(d.offsets_valid(&offsets));
    assert_eq!(d.entry_of(99, &offsets), None);
    assert_eq!(d.entry_of(100, &offsets), Some(0));
    assert_eq!(d.entry_of(109, &offsets), Some(0));
    assert_eq!(d.entry_of(110, &offsets), Some(2));
    assert_eq!(d.entry_of(124, &offsets), Some(2));
    assert_eq!(d.entry_of(125, &offsets), Some(3));
    assert_eq!(d.entry_of(139, &offsets), Some(3));
    assert_eq!(d.entry_of(140, &offsets), None);

    assert!(!d.offsets_valid(&[0, 10, 5, 25, 40]));
    assert_eq!(d.entry_of(100, &[0, 10, 5, 25, 40]), None);
    assert!(!d.offsets_valid(&[0, 10, 20]));
    assert_eq!(d.entry_of(100, &[0, 10, 20]), None);

    let shifted = SegmentDesc { entries: 1, ..d };
    assert_eq!(shifted.entry_of(104, &[5, 10]), None);
    assert_eq!(shifted.entry_of(105, &[5, 10]), Some(0));
    let empty = SegmentDesc { entries: 0, ..d };
    assert_eq!(empty.entry_of(100, &[0]), None);
    let huge = SegmentDesc { entries: u64::MAX, ..d };
    assert!(!huge.offsets_valid(&offsets));
    assert_eq!(huge.entry_of(100, &offsets), None);
}

#[test]
fn cursor_advance() {
    let mut c = RangeCursor::new(region(0, 10));
    assert_eq!((c.used(), c.remaining()), (0, 10));
    assert_eq!(c.advance(4), Ok(0));
    assert_eq!(c.advance(6), Ok(4));
    assert_eq!(c.advance(1), Err(RangeFull { requested: 1, remaining: 0 }));
    assert_eq!(c.advance(0), Ok(10));
    assert_eq!(c.used(), 10);

    let mut c = RangeCursor::new(region(0, 10));
    assert_eq!(c.advance(3), Ok(0));
    assert_eq!(c.advance(u64::MAX), Err(RangeFull { requested: u64::MAX, remaining: 7 }));
    assert_eq!(c.used(), 3);
    assert_eq!(c.region(), region(0, 10));
}

#[test]
fn begin_ids_and_validation() {
    let bi = BatchInterface::new(scripted(0, vec![]));
    assert!(matches!(bi.begin(spec(0), BatchMode::Normal), Err(BatchError::InvalidSpec(_))));
    let a = bi.begin(spec(1), BatchMode::Normal).unwrap();
    let b = bi.begin(spec(1), BatchMode::Normal).unwrap();
    assert!(a < b);
}

#[test]
fn run_without_ranges_needs_ranges() {
    let exec = scripted(0, vec![]);
    let bi = BatchInterface::new(Arc::clone(&exec));
    let id = bi.begin(spec(2), BatchMode::Normal).unwrap();
    assert_eq!(bi.run(id, &inputs()), Err(BatchError::NeedsRanges));
    assert!(exec.calls.lock().unwrap().is_empty());
}

#[test]
fn normal_hits_flow() {
    let exec = scripted(4, vec![Ok(LaunchOutcome::Hits { hits: 9 })]);
    let bi = BatchInterface::new(Arc::clone(&exec));
    let id = bi.begin(spec(3), BatchMode::Normal).unwrap();
    let (r1, r2) = (region(0, 16), region(16, 16));
    bi.assign_ranges(id, &[r1, r2]).unwrap();
    let event = bi.event(id).unwrap();
    assert_eq!(event.is_complete(), Ok(false));
    assert_eq!(bi.progress(id), Ok(0));

    assert_eq!(bi.run(id, &inputs()), Ok(LaunchOutcome::Hits { hits: 9 }));
    assert_eq!(event.is_complete(), Ok(true));
    assert_eq!(bi.progress(id), Ok(3));
    assert_eq!(bi.run(id, &inputs()), Err(BatchError::AlreadyFinished));
    assert_eq!(bi.assign_ranges(id, &[r1]), Err(BatchError::AlreadyFinished));
    assert_eq!(
        exec.calls.lock().unwrap().as_slice(),
        &[(0, vec![16, 16], Duration::from_millis(7))]
    );

    let used = bi.end(id).unwrap();
    assert_eq!(
        used,
        vec![UsedRange { region: r1, used: 4 }, UsedRange { region: r2, used: 4 }]
    );
    assert_eq!(bi.end(id), Err(BatchError::UnknownBatch(id)));
    assert_eq!(bi.progress(id), Err(BatchError::UnknownBatch(id)));
    assert!(matches!(bi.event(id), Err(BatchError::UnknownBatch(_))));
    assert_eq!(bi.run(id, &inputs()), Err(BatchError::UnknownBatch(id)));
    assert_eq!(bi.assign_ranges(id, &[]), Err(BatchError::UnknownBatch(id)));
}

#[test]
fn partial_then_resume() {
    let exec = scripted(
        8,
        vec![
            Ok(LaunchOutcome::Partial { queries_done: 3, hits: 5 }),
            Ok(LaunchOutcome::Hits { hits: 2 }),
        ],
    );
    let bi = BatchInterface::new(Arc::clone(&exec));
    let id = bi.begin(spec(10), BatchMode::Normal).unwrap();
    let (r1, r2) = (region(0, 8), region(8, 20));
    bi.assign_ranges(id, &[r1]).unwrap();
    let event = bi.event(id).unwrap();

    assert_eq!(bi.run(id, &inputs()), Ok(LaunchOutcome::Partial { queries_done: 3, hits: 5 }));
    assert_eq!(event.is_complete(), Ok(false));
    assert_eq!(bi.progress(id), Ok(3));
    assert_eq!(bi.run(id, &inputs()), Err(BatchError::NeedsRanges));

    bi.assign_ranges(id, &[r2]).unwrap();
    assert_eq!(bi.run(id, &inputs()), Ok(LaunchOutcome::Hits { hits: 2 }));
    assert_eq!(event.is_complete(), Ok(true));
    assert_eq!(bi.progress(id), Ok(10));
    assert_eq!(
        exec.calls.lock().unwrap().as_slice(),
        &[
            (0, vec![8], Duration::from_millis(7)),
            (3, vec![20], Duration::from_millis(7)),
        ]
    );
    assert_eq!(
        bi.end(id).unwrap(),
        vec![UsedRange { region: r1, used: 8 }, UsedRange { region: r2, used: 8 }]
    );
}

#[test]
fn executor_error_keeps_state() {
    let exec = scripted(4, vec![Err("boom".to_string())]);
    let bi = BatchInterface::new(Arc::clone(&exec));
    let id = bi.begin(spec(2), BatchMode::Normal).unwrap();
    let r = region(0, 16);
    bi.assign_ranges(id, &[r]).unwrap();
    let event = bi.event(id).unwrap();

    assert_eq!(bi.run(id, &inputs()), Err(BatchError::Executor("boom".to_string())));
    assert_eq!(event.is_complete(), Ok(false));
    assert_eq!(bi.progress(id), Ok(0));
    assert_eq!(bi.end(id).unwrap(), vec![UsedRange { region: r, used: 0 }]);
}

#[test]
fn executor_panic_becomes_error() {
    let bi = BatchInterface::new(PanicExec);
    let id = bi.begin(spec(2), BatchMode::Normal).unwrap();
    bi.assign_ranges(id, &[region(0, 16)]).unwrap();
    assert!(matches!(bi.run(id, &inputs()), Err(BatchError::Executor(_))));
    assert!(matches!(bi.run(id, &inputs()), Err(BatchError::Executor(_))));
    assert!(bi.end(id).is_ok());
}

#[test]
fn persistent_progress_is_monotonic_and_clamped() {
    let bi = BatchInterface::new(scripted(0, vec![]));
    let counter = Arc::new(Counter::default());
    let id = bi
        .begin(spec(10), BatchMode::Persistent { counter: counter.clone() })
        .unwrap();
    assert!(matches!(bi.event(id), Err(BatchError::InvalidSpec(_))));
    assert_eq!(bi.progress(id), Ok(0));
    counter.0.store(5, Ordering::SeqCst);
    assert_eq!(bi.progress(id), Ok(5));
    counter.0.store(2, Ordering::SeqCst);
    assert_eq!(bi.progress(id), Ok(5));
    counter.0.store(100, Ordering::SeqCst);
    assert_eq!(bi.progress(id), Ok(10));
}

#[test]
fn persistent_run_finishes_without_event() {
    let exec = scripted(1, vec![Ok(LaunchOutcome::Hits { hits: 1 })]);
    let bi = BatchInterface::new(Arc::clone(&exec));
    let counter = Arc::new(Counter::default());
    let id = bi.begin(spec(4), BatchMode::Persistent { counter }).unwrap();
    bi.assign_ranges(id, &[region(0, 4)]).unwrap();
    assert_eq!(bi.run(id, &inputs()), Ok(LaunchOutcome::Hits { hits: 1 }));
    assert_eq!(bi.progress(id), Ok(4));
}

#[test]
fn concurrent_run_is_busy() {
    let gate = Arc::new(Gate {
        entered: (Mutex::new(false), Condvar::new()),
        release: (Mutex::new(false), Condvar::new()),
    });
    let bi = Arc::new(BatchInterface::new(Arc::clone(&gate)));
    let id = bi.begin(spec(2), BatchMode::Normal).unwrap();
    let other = bi.begin(spec(2), BatchMode::Normal).unwrap();
    bi.assign_ranges(id, &[region(0, 8)]).unwrap();

    let worker = {
        let bi = Arc::clone(&bi);
        std::thread::spawn(move || bi.run(id, &inputs()))
    };
    {
        let mut entered = gate.entered.0.lock().unwrap();
        while !*entered {
            entered = gate.entered.1.wait(entered).unwrap();
        }
    }

    assert_eq!(bi.run(id, &inputs()), Err(BatchError::Busy));
    assert_eq!(bi.assign_ranges(id, &[region(8, 8)]), Err(BatchError::Busy));
    assert_eq!(bi.end(id), Err(BatchError::Busy));
    assert_eq!(bi.progress(id), Ok(0));
    assert_eq!(bi.event(id).unwrap().is_complete(), Ok(false));
    assert_eq!(bi.progress(other), Ok(0));
    assert!(bi.end(other).is_ok());

    *gate.release.0.lock().unwrap() = true;
    gate.release.1.notify_all();
    assert_eq!(worker.join().unwrap(), Ok(LaunchOutcome::Hits { hits: 1 }));
    assert_eq!(bi.progress(id), Ok(2));
    assert_eq!(bi.end(id).unwrap().len(), 1);
}

#[test]
fn end_without_finishing_reports_unused() {
    let bi = BatchInterface::new(scripted(0, vec![]));
    let id = bi.begin(spec(2), BatchMode::Normal).unwrap();
    let (r1, r2) = (region(0, 4), region(4, 4));
    bi.assign_ranges(id, &[r1, r2]).unwrap();
    assert_eq!(
        bi.end(id).unwrap(),
        vec![UsedRange { region: r1, used: 0 }, UsedRange { region: r2, used: 0 }]
    );
}

#[test]
fn interface_is_send_sync() {
    fn check<T: Send + Sync>() {}
    check::<BatchInterface<Arc<Script>>>();
    check::<BatchMode>();
}
