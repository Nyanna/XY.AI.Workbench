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
        unit(& [1.0, 0.0]), unit(& [1.0, 1.0]), unit(& [0.0, 1.0]), unit(& [- 1.0, 0.0])
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
        .score(&QueryBatch::new(2, vec![a.clone(), b.clone()]).unwrap(), r, &mut lane())
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
        matches!(scorer.score(& multi, r, & mut lane()), Err(KernelError::ShapeMismatch {
        .. }))
    );
    let q = QueryBatch::new(2, vec![vec![1.0, 0.0]]).unwrap();
    let far = SliceRange {
        segment: 0,
        slot_from: 0,
        slot_to: 99,
    };
    assert!(
        matches!(scorer.score(& q, far, & mut lane()), Err(KernelError::SliceOutOfRange {
        .. }))
    );
}
const FULL: SliceRange = SliceRange {
    segment: 0,
    slot_from: 0,
    slot_to: 4,
};
#[test]
fn zero_or_non_finite_query_is_rejected_without_nan_scores() {
    let scorer = cosine_scorer(source(), 2);
    for bad in [vec![0.0, 0.0], vec![f32::NAN, 1.0], vec![f32::INFINITY, 0.0]] {
        let q = QueryBatch::new(2, vec![unit(& [1.0, 0.0]), bad]).unwrap();
        let err = scorer.score(&q, FULL, &mut lane()).unwrap_err();
        assert!(matches!(err, KernelError::EmptyEmbedding { query : 1 }), "{err:?}");
    }
}
#[test]
fn zero_document_scores_zero_in_both_modes() {
    let docs = vec![vec![0.0, 0.0], unit(& [1.0, 0.0])];
    let src = Arc::new(InMemorySlots::new(2, vec![docs]));
    let q = QueryBatch::new(2, vec![unit(& [1.0, 0.0])]).unwrap();
    let range = SliceRange {
        segment: 0,
        slot_from: 0,
        slot_to: 2,
    };
    for scorer in [cosine_scorer(src.clone(), 2), cosine_scorer_exact(src.clone(), 2)] {
        let hits = scorer.score(&q, range, &mut lane()).unwrap();
        assert!(hits[0].iter().all(| h | h.score.is_finite()));
        assert_eq!(hits[0] [1].score, 0.0);
    }
}
#[test]
fn exact_mode_divides_out_document_norm() {
    let docs = vec![vec![3.0, 0.0], vec![0.0, 2.0]];
    let src = Arc::new(InMemorySlots::new(2, vec![docs]));
    let q = QueryBatch::new(2, vec![vec![2.0, 0.0]]).unwrap();
    let range = SliceRange {
        segment: 0,
        slot_from: 0,
        slot_to: 2,
    };
    let plain = cosine_scorer(src.clone(), 2).score(&q, range, &mut lane()).unwrap();
    let exact = cosine_scorer_exact(src, 2).score(&q, range, &mut lane()).unwrap();
    assert!((plain[0] [0].score - 6.0).abs() < 1e-6);
    assert!((exact[0] [0].score - 1.0).abs() < 1e-6);
    assert!(exact[0] [1].score.abs() < 1e-6);
}
fn pseudo_unit(seed: &mut u64, dim: usize) -> Vec<f32> {
    let raw: Vec<f32> = (0..dim)
        .map(|_| {
            *seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            ((*seed >> 33) as f32 / (1u64 << 31) as f32) - 0.5
        })
        .collect();
    unit(&raw)
}
#[test]
fn f16_storage_with_f32_accumulation_stays_within_tolerance() {
    const DIM: usize = 32;
    const N: usize = 64;
    let mut seed = 7u64;
    let docs: Vec<Vec<f32>> = (0..N).map(|_| pseudo_unit(&mut seed, DIM)).collect();
    let stored: Vec<Vec<f32>> = docs
        .iter()
        .map(|d| d.iter().map(|&x| half::f16::from_f32(x).to_f32()).collect())
        .collect();
    let reference = Arc::new(InMemorySlots::new(DIM, vec![docs.clone()]));
    let lossy = Arc::new(InMemorySlots::new(DIM, vec![stored]));
    let q = QueryBatch::new(DIM, vec![pseudo_unit(& mut seed, DIM)]).unwrap();
    let range = SliceRange {
        segment: 0,
        slot_from: 0,
        slot_to: N as u32,
    };
    let want = cosine_scorer(reference, 10).score(&q, range, &mut lane()).unwrap();
    let got = cosine_scorer_exact(lossy, 10).score(&q, range, &mut lane()).unwrap();
    let overlap = got[0]
        .iter()
        .filter(|g| want[0].iter().any(|w| w.offset == g.offset))
        .count();
    assert!(overlap >= 9, "top-k overlap {overlap}/10");
    for g in &got[0] {
        let w = docs[(g.offset / (DIM * 4) as u64) as usize]
            .iter()
            .zip(q.data(0))
            .map(|(a, b)| a * b)
            .sum::<f32>();
        assert!((g.score - w).abs() < 1e-3, "offset {}", g.offset);
    }
}
