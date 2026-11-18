use super::*;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::query_batch::{
    Query, RamSource,
};

const SEG: SegmentId = SegmentId(1);

fn fixed(len: u32) -> ChunkSizer {
    ChunkSizer::with_limits(len, len, DEFAULT_TARGET)
}

fn source(slots: usize) -> RamSource {
    let mut src = RamSource::new(1).unwrap();
    src.insert_segment(SEG, (0..slots).map(|_| Some(vec![0.0f32])).collect())
        .unwrap();
    src
}

fn batch(queries: usize) -> QueryBatch {
    let qs = (0..queries)
        .map(|_| Query::new(1, vec![1.0]).unwrap())
        .collect();
    QueryBatch::from_queries(1, qs).unwrap()
}

#[test]
fn sizer_defaults_and_limits() {
    let s = ChunkSizer::new();
    assert_eq!(s.throughput(), None);
    assert_eq!(s.next_len(), DEFAULT_MIN_LEN);
    let s = ChunkSizer::with_limits(0, 0, Duration::ZERO);
    assert_eq!((s.min_len(), s.max_len()), (1, 1));
    let s = ChunkSizer::with_limits(10, 5, DEFAULT_TARGET);
    assert_eq!(s.max_len(), 10);
}

#[test]
fn sizer_observe_and_next_len() {
    let mut s = ChunkSizer::with_limits(1, 1_000_000, Duration::from_millis(2));
    s.observe(0, Duration::from_secs(1));
    assert_eq!(s.throughput(), None);
    s.observe(1000, Duration::from_secs(1));
    assert_eq!(s.throughput(), Some(1000.0));
    assert_eq!(s.next_len(), 2);
    s.observe(2000, Duration::from_secs(1));
    let t = s.throughput().unwrap();
    assert!((t - 1300.0).abs() < 1e-9);
}

#[test]
fn sizer_zero_elapsed_saturates() {
    let mut s = ChunkSizer::with_limits(1, u32::MAX, Duration::from_secs(1000));
    s.observe(u32::MAX, Duration::ZERO);
    assert_eq!(s.next_len(), u32::MAX);
    let mut s = ChunkSizer::new();
    s.observe(1, Duration::from_secs(3600));
    assert_eq!(s.next_len(), DEFAULT_MIN_LEN);
}

#[test]
fn gate_counts_guards() {
    let g = PreemptionGate::new();
    assert!(!g.should_preempt(TaskClass::Indexing));
    let a = g.enter_query();
    let b = g.enter_query();
    assert_eq!(g.waiting_queries(), 2);
    assert!(g.should_preempt(TaskClass::Indexing));
    assert!(!g.should_preempt(TaskClass::Query));
    drop(a);
    assert!(g.should_preempt(TaskClass::Indexing));
    drop(b);
    assert_eq!(g.waiting_queries(), 0);
}

#[test]
fn empty_range_runs_nothing() {
    let g = PreemptionGate::new();
    let r = Workspace::new(5, 5);
    let out = run_chunks::<(), _>(&mut fixed(4), r, Duration::ZERO, TaskClass::Query, &g, |_| {
        panic!("no chunk expected")
    })
    .unwrap();
    assert_eq!(out.done_slots, 0);
    assert_eq!(out.rest, r);
    assert_eq!(out.stop, None);
}

#[test]
fn completes_without_stop() {
    let g = PreemptionGate::new();
    let mut seen = Vec::new();
    let out = run_chunks::<(), _>(
        &mut fixed(4),
        Workspace::new(2, 12),
        Duration::from_secs(60),
        TaskClass::Indexing,
        &g,
        |c| {
            seen.push((c.start(), c.end()));
            Ok(())
        },
    )
    .unwrap();
    assert_eq!(seen, vec![(2, 6), (6, 10), (10, 12)]);
    assert_eq!(out.done_slots, 10);
    assert!(out.rest.is_empty());
    assert_eq!(out.stop, None);
}

#[test]
fn zero_budget_runs_one_chunk() {
    let g = PreemptionGate::new();
    let _q = g.enter_query();
    let out = run_chunks::<(), _>(
        &mut fixed(4),
        Workspace::new(0, 10),
        Duration::ZERO,
        TaskClass::Indexing,
        &g,
        |_| Ok(()),
    )
    .unwrap();
    assert_eq!(out.done_slots, 4);
    assert_eq!(out.rest, Workspace::new(4, 10));
    assert_eq!(out.stop, Some(StopReason::TimeBudget));
}

#[test]
fn preemption_stops_indexing_only() {
    let g = PreemptionGate::new();
    let _q = g.enter_query();
    let out = run_chunks::<(), _>(
        &mut fixed(4),
        Workspace::new(0, 10),
        Duration::from_secs(60),
        TaskClass::Indexing,
        &g,
        |_| Ok(()),
    )
    .unwrap();
    assert_eq!(out.stop, Some(StopReason::Preemption));
    assert_eq!(out.done_slots, 4);
    assert_eq!(out.rest, Workspace::new(4, 10));

    let out = run_chunks::<(), _>(
        &mut fixed(4),
        Workspace::new(0, 10),
        Duration::from_secs(60),
        TaskClass::Query,
        &g,
        |_| Ok(()),
    )
    .unwrap();
    assert_eq!(out.stop, None);
    assert_eq!(out.done_slots, 10);
}

#[test]
fn last_chunk_is_never_stopped() {
    let g = PreemptionGate::new();
    let _q = g.enter_query();
    let out = run_chunks::<(), _>(
        &mut fixed(4),
        Workspace::new(0, 4),
        Duration::ZERO,
        TaskClass::Indexing,
        &g,
        |_| Ok(()),
    )
    .unwrap();
    assert_eq!(out.stop, None);
    assert!(out.rest.is_empty());
}

#[test]
fn chunk_error_is_immediate_and_not_observed() {
    let g = PreemptionGate::new();
    let mut sizer = fixed(4);
    let mut calls = 0;
    let r = run_chunks(
        &mut sizer,
        Workspace::new(0, 10),
        Duration::from_secs(60),
        TaskClass::Query,
        &g,
        |_| {
            calls += 1;
            if calls == 2 {
                Err("boom")
            } else {
                Ok(())
            }
        },
    );
    assert_eq!(r.unwrap_err(), "boom");
    assert_eq!(calls, 2);
    let mut fresh = fixed(4);
    fresh.observe(4, Duration::from_millis(1));
    assert!(sizer.throughput().is_some());
}

#[derive(Debug)]
struct Boom;

impl fmt::Display for Boom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "boom")
    }
}

impl std::error::Error for Boom {}

struct Ramp {
    fail_at: Option<u32>,
    queries: usize,
}

impl TileScorer for Ramp {
    type Error = Boom;

    fn score_tile(
        &self,
        _batch: &QueryBatch,
        _segment: SegmentId,
        tile: Workspace,
    ) -> Result<Vec<f32>, Boom> {
        if self.fail_at == Some(tile.start()) {
            return Err(Boom);
        }
        let mut out = Vec::new();
        for q in 0..self.queries {
            for i in 0..tile.len() {
                out.push((tile.start() + i) as f32 + q as f32 * 0.5);
            }
        }
        Ok(out)
    }
}

#[test]
fn score_range_accumulates_all_chunks() {
    let src = source(10);
    let b = batch(2);
    let g = PreemptionGate::new();
    let mut rs = RangeScorer::new(2, 3);
    let out = score_range(
        &mut fixed(3),
        &Ramp { fail_at: None, queries: 2 },
        &src,
        &b,
        SEG,
        Workspace::new(0, 10),
        Duration::from_secs(60),
        TaskClass::Query,
        &g,
        &mut rs,
    )
    .unwrap();
    assert_eq!(out.done_slots, 10);
    let res = rs.into_results();
    assert_eq!(res.len(), 2);
    assert_eq!(res[0].len(), 3);
    assert_eq!(res[0][0].score, 9.0);
    assert_eq!(res[1][0].score, 9.5);
}

#[test]
fn score_error_stops_with_prior_chunks_kept() {
    let src = source(10);
    let b = batch(1);
    let g = PreemptionGate::new();
    let mut rs = RangeScorer::new(1, 20);
    let r = score_range(
        &mut fixed(3),
        &Ramp { fail_at: Some(6), queries: 1 },
        &src,
        &b,
        SEG,
        Workspace::new(0, 10),
        Duration::from_secs(60),
        TaskClass::Query,
        &g,
        &mut rs,
    );
    assert!(matches!(r, Err(ChunkError::Score(_))));
    assert_eq!(rs.into_results()[0].len(), 6);
}

#[test]
fn topk_errors_are_mapped() {
    let src = source(4);
    let b = batch(1);
    let g = PreemptionGate::new();
    let mut rs = RangeScorer::new(2, 3);
    let r = score_range(
        &mut fixed(2),
        &Ramp { fail_at: None, queries: 1 },
        &src,
        &b,
        SEG,
        Workspace::new(0, 4),
        Duration::from_secs(60),
        TaskClass::Query,
        &g,
        &mut rs,
    );
    assert!(matches!(r, Err(ChunkError::TopK(_))));

    let mut rs = RangeScorer::new(1, 3);
    let r = score_range(
        &mut fixed(2),
        &Ramp { fail_at: None, queries: 1 },
        &src,
        &b,
        SEG,
        Workspace::new(0, 9),
        Duration::from_secs(60),
        TaskClass::Query,
        &g,
        &mut rs,
    );
    let e = r.unwrap_err();
    assert!(matches!(e, ChunkError::TopK(_)));
    assert!(!e.to_string().is_empty());
}
