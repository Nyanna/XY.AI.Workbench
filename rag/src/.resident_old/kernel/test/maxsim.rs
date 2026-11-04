use super::*;
use crate::resident::kernel::InMemorySlots;
use crate::resident::memory::{ArenaId, Region};
use crate::resident::subengine::{LaneClass, LaneConfig, RecordingLane};
fn lane() -> RecordingLane {
    let scratch = Region {
        arena: ArenaId(0),
        offset: 0,
        len: 0,
        gen: 0,
    };
    RecordingLane::new(LaneClass::Query, LaneConfig::new(scratch))
}
fn tokens(n: usize, dim: usize, seed: u32) -> Vec<f32> {
    (0..n * dim)
        .map(|i| {
            (((i as u32).wrapping_mul(2654435761).wrapping_add(seed * 97)) % 1000)
                as f32 / 500.0 - 1.0
        })
        .collect()
}
fn source(dim: usize) -> Arc<InMemorySlots> {
    let docs = vec![
        tokens(5, dim, 1), tokens(12, dim, 2), tokens(1, dim, 3), Vec::new(),
        tokens(8, dim, 4)
    ];
    Arc::new(InMemorySlots::new(dim, vec![docs]))
}
const R: SliceRange = SliceRange {
    segment: 0,
    slot_from: 0,
    slot_to: 5,
};
#[test]
fn cpu_op_ranks_by_maxsim_and_empty_doc_is_last() {
    let dim = 4;
    let op = MaxSimOp::cpu(source(dim), 5);
    let q = QueryBatch::new(dim, vec![tokens(3, dim, 9)]).unwrap();
    let hits = op.score(&q, R, &mut lane()).unwrap();
    assert_eq!(hits[0].len(), 5);
    assert!(hits[0].windows(2).all(| w | w[0].score >= w[1].score));
    let last = hits[0].last().unwrap();
    assert_eq!(last.offset, source(dim).slot_offset(0, 3));
    assert!(last.score.is_finite());
}
#[test]
fn cpu_op_top_k_limits_results() {
    let dim = 4;
    let op = MaxSimOp::cpu(source(dim), 2);
    let q = QueryBatch::new(dim, vec![tokens(2, dim, 5), tokens(4, dim, 6)])
        .unwrap();
    let hits = op.score(&q, R, &mut lane()).unwrap();
    assert!(hits.iter().all(| h | h.len() == 2));
}
#[cfg(feature = "candle")]
mod candle {
    use super::*;
    use crate::resident::memory::DType;
    fn per_slot(op: &dyn SlotScorer, q: &QueryBatch) -> Vec<Vec<f32>> {
        op.scores(q, R).unwrap()
    }
    #[test]
    fn f32_tile_chain_matches_cpu_reference_incl_padding() {
        let dim = 8;
        let src: Arc<dyn SlotSource> = source(dim);
        let q = QueryBatch::new(dim, vec![tokens(4, dim, 7), tokens(2, dim, 8)])
            .unwrap();
        let want = per_slot(&CpuMaxSim::new(src.clone()), &q);
        for tile in [1usize, 3, 4, 64] {
            let chain = candle_chain::CandleMaxSim::new(
                    src.clone(),
                    candle_core::Device::Cpu,
                    DType::F32,
                    tile,
                )
                .unwrap();
            let got = per_slot(&chain, &q);
            for (g, w) in got.iter().flatten().zip(want.iter().flatten()) {
                assert!((g - w).abs() < 1e-3, "tile={tile} got={g} want={w}");
            }
        }
    }
    #[test]
    fn f16_masking_never_produces_nan() {
        let dim = 8;
        let src: Arc<dyn SlotSource> = source(dim);
        let q = QueryBatch::new(dim, vec![tokens(4, dim, 7)]).unwrap();
        let want = per_slot(&CpuMaxSim::new(src.clone()), &q);
        let chain = candle_chain::CandleMaxSim::new(
                src,
                candle_core::Device::Cpu,
                DType::F16,
                4,
            )
            .unwrap();
        let got = per_slot(&chain, &q);
        for (g, w) in got[0].iter().zip(&want[0]) {
            assert!(g.is_finite());
            assert!((g - w).abs() < 0.1 + 0.01 * w.abs(), "got={g} want={w}");
        }
    }
    #[test]
    fn op_wraps_candle_chain() {
        let dim = 8;
        let op = MaxSimOp::candle_cpu(source(dim), 3, DType::F32, 4).unwrap();
        let q = QueryBatch::new(dim, vec![tokens(4, dim, 7)]).unwrap();
        assert_eq!(op.score(& q, R, & mut lane()).unwrap() [0].len(), 3);
    }
}
