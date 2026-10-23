use super::*;
use crate::resident::kernel::cpu_simd::normalize;
use crate::resident::kernel::{score_ranges, InMemorySlots, Scorer};
use crate::resident::subengine::{LaneClass, LaneConfig, RecordingLane};
use crate::resident::memory::{ArenaId, Region};
fn lane() -> RecordingLane {
    let scratch = Region {
        arena: ArenaId(0),
        offset: 0,
        len: 0,
        gen: 0,
    };
    RecordingLane::new(LaneClass::Query, LaneConfig::new(scratch))
}
fn unit(v: &[f32]) -> Vec<f32> {
    let mut v = v.to_vec();
    normalize(&mut v);
    v
}
fn source() -> Arc<InMemorySlots> {
    let docs = vec![
        unit(& [1.0, 0.0]), unit(& [1.0, 1.0]), unit(& [0.0, 1.0]), unit(& [- 1.0,
        0.0])
    ];
    Arc::new(InMemorySlots::new(2, vec![docs.clone(), docs]))
}
#[test]
fn ranks_by_cosine_with_offsets() {
    let scorer = cosine_scorer(source(), 2);
    let q = QueryBatch::new(2, vec![unit(& [1.0, 0.1])]).unwrap();
    let hits = scorer
        .score(
            &q,
            SliceRange {
                segment: 0,
                slot_from: 0,
                slot_to: 4,
            },
            &mut lane(),
        )
        .unwrap();
    assert_eq!(hits[0].len(), 2);
    assert_eq!(hits[0] [0].offset, 0);
    assert_eq!(hits[0] [1].offset, 8);
    assert!(hits[0] [0].score > hits[0] [1].score);
}
#[test]
fn batched_queries_match_individual_runs() {
    let scorer = cosine_scorer(source(), 3);
    let a = unit(&[1.0, 0.0]);
    let b = unit(&[0.0, 1.0]);
    let r = SliceRange {
        segment: 0,
        slot_from: 0,
        slot_to: 4,
    };
    let both = scorer
        .score(
            &QueryBatch::new(2, vec![a.clone(), b.clone()]).unwrap(),
            r,
            &mut lane(),
        )
        .unwrap();
    let only_b = scorer
        .score(&QueryBatch::new(2, vec![b]).unwrap(), r, &mut lane())
        .unwrap();
    assert_eq!(both[1], only_b[0]);
}
#[test]
fn scope_subset_merges_ranges_across_segments() {
    let scorer = cosine_scorer(source(), 10);
    let q = QueryBatch::new(2, vec![unit(& [0.0, 1.0])]).unwrap();
    let ranges = [
        SliceRange {
            segment: 0,
            slot_from: 1,
            slot_to: 3,
        },
        SliceRange {
            segment: 1,
            slot_from: 2,
            slot_to: 3,
        },
    ];
    let hits = score_ranges(&scorer, &q, &ranges, 2, &mut lane()).unwrap();
    assert_eq!(hits[0].len(), 2);
    assert!((hits[0] [0].score - 1.0).abs() < 1e-6);
}
#[test]
fn rejects_multi_token_queries_and_bad_ranges() {
    let scorer = cosine_scorer(source(), 1);
    let multi = QueryBatch::new(2, vec![vec![1.0, 0.0, 0.0, 1.0]]).unwrap();
    let r = SliceRange {
        segment: 0,
        slot_from: 0,
        slot_to: 1,
    };
    assert!(
        matches!(scorer.score(& multi, r, & mut lane()),
        Err(KernelError::ShapeMismatch { .. }))
    );
    let q = QueryBatch::new(2, vec![vec![1.0, 0.0]]).unwrap();
    let far = SliceRange {
        segment: 0,
        slot_from: 0,
        slot_to: 99,
    };
    assert!(
        matches!(scorer.score(& q, far, & mut lane()),
        Err(KernelError::SliceOutOfRange { .. }))
    );
}
