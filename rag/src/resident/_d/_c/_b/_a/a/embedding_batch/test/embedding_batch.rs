use super::*;
use std::collections::VecDeque;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::a::segment_arena::SegmentArenaConfig;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::a::kernel_batch::{
    BatchMode, BatchSpec, RangeCursor, SegmentDesc, SegmentLayout,
};
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::a::bookkeeping::Bookkeeping;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::a::region_handle::Generation;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::a::arenas::{
    ArenaCapacities, ArenaKind, ArenaSet, Budget,
};
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::_1::device_lane_event::DeviceKind;
fn arena(capacity: u64) -> SegmentArena<Arc<Allocator>> {
    let mut caps = ArenaCapacities::default();
    caps.set(ArenaKind::IndexSegments, capacity);
    let set = Arc::new(ArenaSet::new(caps));
    let budget = Budget {
        free: u64::MAX / 2,
        total: u64::MAX / 2,
        generation_reserve: 0,
    };
    let seg = set.ensure(ArenaKind::IndexSegments, &budget).unwrap();
    let bk = Arc::new(Bookkeeping::new(set));
    let alloc = Arc::new(Allocator::new(seg, DeviceKind::Gpu, bk));
    let config = SegmentArenaConfig {
        reserve_bp: 0,
        ..SegmentArenaConfig::default()
    };
    SegmentArena::new(alloc, config).unwrap()
}
fn input_region(len: u64) -> RegionHandle {
    RegionHandle::new(ArenaKind::MappedHost, 4096, len, Generation(0)).unwrap()
}
fn spec() -> BatchSpec {
    BatchSpec {
        segments: vec![
            SegmentDesc { base : 0, entries : 4, layout : SegmentLayout::Fixed { stride :
            16 } }
        ],
        query_count: 8,
        params: None,
        time_budget: Duration::from_millis(5),
    }
}
fn config(range_len: u64, max_ranges: usize) -> ExecConfig {
    ExecConfig {
        range_len,
        max_ranges,
    }
}
#[derive(Default)]
struct Script {
    outcomes: Mutex<VecDeque<Result<LaunchOutcome, String>>>,
    inputs: Mutex<Vec<RegionHandle>>,
}
impl BatchExecutor for Arc<Script> {
    fn launch(
        &self,
        _id: BatchId,
        _spec: &BatchSpec,
        inputs: &BatchInputs,
        _queries_done: u32,
        ranges: &mut [RangeCursor],
    ) -> Result<LaunchOutcome, String> {
        self.inputs.lock().unwrap().push(inputs.regions[0]);
        for r in ranges.iter_mut() {
            let _ = r.advance(64);
        }
        self.outcomes
            .lock()
            .unwrap()
            .pop_front()
            .unwrap_or(Ok(LaunchOutcome::Hits { hits: 0 }))
    }
}
fn scripted(outcomes: Vec<Result<LaunchOutcome, String>>) -> Arc<Script> {
    Arc::new(Script {
        outcomes: Mutex::new(outcomes.into()),
        inputs: Mutex::new(Vec::new()),
    })
}
fn partial() -> Result<LaunchOutcome, String> {
    Ok(LaunchOutcome::Partial {
        queries_done: 1,
        hits: 0,
    })
}
fn hits(n: u64) -> Result<LaunchOutcome, String> {
    Ok(LaunchOutcome::Hits { hits: n })
}
struct Fixed(Vec<Result<u64, String>>, Mutex<usize>);
fn fixed(values: Vec<Result<u64, String>>) -> Fixed {
    Fixed(values, Mutex::new(0))
}
impl StepProgress for Fixed {
    fn write_result(&self, range: RegionHandle) -> Result<WriteResult, String> {
        let mut at = self.1.lock().unwrap();
        let v = self.0.get(*at).cloned().unwrap_or(Ok(0));
        *at += 1;
        v.map(|len| WriteResult::Complete {
            region: range,
            len,
        })
    }
}
fn begun(script: &Arc<Script>) -> (BatchInterface<Arc<Script>>, BatchId) {
    let batch = BatchInterface::new(Arc::clone(script));
    let id = batch.begin(spec(), BatchMode::Normal).unwrap();
    (batch, id)
}
fn assert_ended(batch: &BatchInterface<Arc<Script>>, id: BatchId) {
    assert_eq!(batch.end(id), Err(BatchError::UnknownBatch(id)));
}
#[test]
fn hits_in_first_step() {
    let arena = arena(1 << 20);
    let script = scripted(vec![hits(5)]);
    let (batch, id) = begun(&script);
    let ok = execute_embedding_batch(
            &batch,
            &arena,
            &fixed(vec![]),
            id,
            BatchInput {
                region: input_region(1000),
            },
            &config(1000, 4),
        )
        .unwrap();
    assert_eq!(ok.partials, 0);
    assert_eq!(ok.hits, 5);
    assert_eq!(ok.regions.len(), 1);
    assert_eq!(ok.ranges.len(), 1);
    assert_eq!(ok.ranges[0].used, 64);
    assert_eq!(ok.ranges[0].region.offset(), ok.regions[0].offset);
    assert_eq!(ok.ranges[0].region.len(), ok.regions[0].len);
    assert_eq!(ok.regions[0].len % 256, 0);
    assert_eq!(arena.stats().live_segments, 1);
    assert_ended(&batch, id);
}
#[test]
fn partials_resume_behind_processed_bytes() {
    let arena = arena(1 << 20);
    let script = scripted(vec![partial(), partial(), hits(9)]);
    let (batch, id) = begun(&script);
    let ok = execute_embedding_batch(
            &batch,
            &arena,
            &fixed(vec![Ok(100), Ok(50)]),
            id,
            BatchInput {
                region: input_region(1000),
            },
            &config(512, 4),
        )
        .unwrap();
    assert_eq!(ok.partials, 2);
    assert_eq!(ok.hits, 9);
    assert_eq!(ok.regions.len(), 3);
    assert_eq!(ok.ranges.len(), 3);
    let seen = script.inputs.lock().unwrap().clone();
    assert_eq!(seen.len(), 3);
    assert_eq!((seen[0].offset(), seen[0].len()), (4096, 1000));
    assert_eq!((seen[1].offset(), seen[1].len()), (4196, 900));
    assert_eq!((seen[2].offset(), seen[2].len()), (4246, 850));
    assert_eq!(seen[2].arena(), ArenaKind::MappedHost);
    for (r, s) in ok.ranges.iter().zip(&ok.regions) {
        assert_eq!(r.region.offset(), s.offset);
    }
    assert_eq!(arena.stats().live_segments, 3);
    assert_ended(&batch, id);
}
#[test]
fn max_ranges_exhausted_releases_everything() {
    let arena = arena(1 << 20);
    let script = scripted(vec![partial(), partial(), partial()]);
    let (batch, id) = begun(&script);
    let err = execute_embedding_batch(
            &batch,
            &arena,
            &fixed(vec![Ok(1), Ok(1), Ok(1)]),
            id,
            BatchInput {
                region: input_region(1000),
            },
            &config(512, 2),
        )
        .unwrap_err();
    assert_eq!(err, ExecError::ArenaExhausted { max_ranges : 2 });
    assert_eq!(err.to_string(), "Arena erschöpft");
    assert_eq!(script.inputs.lock().unwrap().len(), 2);
    assert_eq!(arena.stats().live_segments, 0);
    assert_ended(&batch, id);
}
#[test]
fn invalid_config_ends_without_reserving() {
    for cfg in [config(0, 4), config(512, 0)] {
        let arena = arena(1 << 20);
        let script = scripted(vec![]);
        let (batch, id) = begun(&script);
        let err = execute_embedding_batch(
                &batch,
                &arena,
                &fixed(vec![]),
                id,
                BatchInput {
                    region: input_region(10),
                },
                &cfg,
            )
            .unwrap_err();
        assert_eq!(err, ExecError::InvalidConfig);
        assert_eq!(arena.stats().live_segments, 0);
        assert!(script.inputs.lock().unwrap().is_empty());
        assert_ended(&batch, id);
    }
}
#[test]
fn executor_error_is_reported_first() {
    let arena = arena(1 << 20);
    let script = scripted(vec![partial(), Err("boom".to_string())]);
    let (batch, id) = begun(&script);
    let err = execute_embedding_batch(
            &batch,
            &arena,
            &fixed(vec![Ok(10)]),
            id,
            BatchInput {
                region: input_region(100),
            },
            &config(512, 4),
        )
        .unwrap_err();
    assert_eq!(err, ExecError::Batch(BatchError::Executor("boom".to_string())));
    assert_eq!(arena.stats().live_segments, 0);
    assert_ended(&batch, id);
}
#[test]
fn progress_errors() {
    let arena = arena(1 << 20);
    let script = scripted(vec![partial()]);
    let (batch, id) = begun(&script);
    let err = execute_embedding_batch(
            &batch,
            &arena,
            &fixed(vec![Err("no result".to_string())]),
            id,
            BatchInput {
                region: input_region(100),
            },
            &config(512, 4),
        )
        .unwrap_err();
    assert_eq!(err, ExecError::Progress("no result".to_string()));
    assert_eq!(arena.stats().live_segments, 0);
    assert_ended(&batch, id);
    let script = scripted(vec![partial()]);
    let (batch, id) = begun(&script);
    let err = execute_embedding_batch(
            &batch,
            &arena,
            &fixed(vec![Ok(101)]),
            id,
            BatchInput {
                region: input_region(100),
            },
            &config(512, 4),
        )
        .unwrap_err();
    assert_eq!(err, ExecError::ProgressOutOfRange { processed : 101, input_len : 100 });
    assert_eq!(arena.stats().live_segments, 0);
    assert_ended(&batch, id);
}
#[test]
fn processed_equal_to_input_gives_empty_input() {
    let arena = arena(1 << 20);
    let script = scripted(vec![partial(), hits(1)]);
    let (batch, id) = begun(&script);
    let ok = execute_embedding_batch(
            &batch,
            &arena,
            &fixed(vec![Ok(100)]),
            id,
            BatchInput {
                region: input_region(100),
            },
            &config(512, 4),
        )
        .unwrap();
    assert_eq!(ok.partials, 1);
    let seen = script.inputs.lock().unwrap().clone();
    assert_eq!((seen[1].offset(), seen[1].len()), (4196, 0));
}
#[test]
fn arena_failure_releases_earlier_segments() {
    let arena = arena(4096);
    let script = scripted(vec![partial(), partial(), partial(), partial(), partial()]);
    let (batch, id) = begun(&script);
    let err = execute_embedding_batch(
            &batch,
            &arena,
            &fixed(vec![Ok(0); 8]),
            id,
            BatchInput {
                region: input_region(100),
            },
            &config(1024, 16),
        )
        .unwrap_err();
    match &err {
        ExecError::Arena(SegmentArenaError::Exhausted { .. }) => {}
        other => panic!("unexpected {other:?}"),
    }
    assert!(err.to_string().starts_with("Arena erschöpft"));
    assert_eq!(arena.stats().live_segments, 0);
    assert_ended(&batch, id);
}
#[test]
fn unknown_batch_reports_first_error() {
    let arena = arena(1 << 20);
    let script = scripted(vec![]);
    let (batch, id) = begun(&script);
    let bogus = BatchId(id.0 + 100);
    let err = execute_embedding_batch(
            &batch,
            &arena,
            &fixed(vec![]),
            bogus,
            BatchInput {
                region: input_region(10),
            },
            &config(512, 2),
        )
        .unwrap_err();
    assert_eq!(err, ExecError::Batch(BatchError::UnknownBatch(bogus)));
    assert_eq!(arena.stats().live_segments, 0);
    assert!(batch.end(id).is_ok());
}
#[test]
fn release_failure_does_not_change_the_error() {
    let arena = arena(1 << 20);
    let script = scripted(vec![Err("boom".to_string())]);
    let (batch, id) = begun(&script);
    let err = execute_embedding_batch(
            &batch,
            &arena,
            &fixed(vec![]),
            id,
            BatchInput {
                region: input_region(10),
            },
            &config(512, 2),
        )
        .unwrap_err();
    assert_eq!(err, ExecError::Batch(BatchError::Executor("boom".to_string())));
    let seg = arena.allocate(512).unwrap();
    assert!(arena.release(& seg, Readers::None).is_ok());
    release_all(&arena, &[seg]);
}
#[test]
fn different_ids_run_in_parallel_threads() {
    let arena = Arc::new(arena(1 << 22));
    let script = Arc::new(Script::default());
    let batch = Arc::new(BatchInterface::new(Arc::clone(&script)));
    let mut workers = Vec::new();
    for _ in 0..4 {
        let id = batch.begin(spec(), BatchMode::Normal).unwrap();
        let (arena, batch) = (Arc::clone(&arena), Arc::clone(&batch));
        workers
            .push(
                thread::spawn(move || {
                    execute_embedding_batch(
                        &batch,
                        &arena,
                        &fixed(vec![]),
                        id,
                        BatchInput {
                            region: input_region(64),
                        },
                        &config(512, 2),
                    )
                }),
            );
    }
    for w in workers {
        let ok = w.join().unwrap().unwrap();
        assert_eq!(ok.regions.len(), 1);
    }
    assert_eq!(arena.stats().live_segments, 4);
}
struct Gate {
    entered: Mutex<Sender<()>>,
    resume: Mutex<Receiver<()>>,
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
        self.entered.lock().unwrap().send(()).unwrap();
        self.resume.lock().unwrap().recv().unwrap();
        Ok(LaunchOutcome::Hits { hits: 3 })
    }
}
#[test]
fn same_id_concurrently_is_busy() {
    let arena = Arc::new(arena(1 << 20));
    let (entered_tx, entered_rx) = channel();
    let (resume_tx, resume_rx) = channel();
    let gate = Arc::new(Gate {
        entered: Mutex::new(entered_tx),
        resume: Mutex::new(resume_rx),
    });
    let batch = Arc::new(BatchInterface::new(gate));
    let id = batch.begin(spec(), BatchMode::Normal).unwrap();
    let first = {
        let (arena, batch) = (Arc::clone(&arena), Arc::clone(&batch));
        thread::spawn(move || {
            execute_embedding_batch(
                &batch,
                &arena,
                &fixed(vec![]),
                id,
                BatchInput {
                    region: input_region(64),
                },
                &config(512, 2),
            )
        })
    };
    entered_rx.recv().unwrap();
    let err = execute_embedding_batch(
            &batch,
            &arena,
            &fixed(vec![]),
            id,
            BatchInput {
                region: input_region(64),
            },
            &config(512, 2),
        )
        .unwrap_err();
    assert_eq!(err, ExecError::Batch(BatchError::Busy));
    resume_tx.send(()).unwrap();
    let ok = first.join().unwrap().unwrap();
    assert_eq!(ok.hits, 3);
    assert_eq!(arena.stats().live_segments, 1);
}
#[test]
fn error_display_and_traits() {
    assert_eq!(
        ExecError::ArenaExhausted { max_ranges : 3 } .to_string(), "Arena erschöpft"
    );
    let exhausted = SegmentArenaError::Exhausted {
        requested: 1,
        available: 0,
    };
    assert_eq!(ExecError::Arena(exhausted.clone()).to_string(), exhausted.to_string());
    assert!(! ExecError::InvalidConfig.to_string().is_empty());
    let e: &dyn std::error::Error = &ExecError::Progress("x".to_string());
    assert!(e.to_string().contains('x'));
}
