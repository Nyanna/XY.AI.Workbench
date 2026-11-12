use super::*;
use std::collections::hash_map::DefaultHasher;

fn h<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

#[test]
fn kind_and_names() {
    let cases = [
        (TaskResult::Embeddings(vec![]), ResultKind::Embeddings, "embeddings"),
        (TaskResult::EntryRefs(vec![]), ResultKind::EntryRefs, "entry_refs"),
        (TaskResult::Hits(vec![]), ResultKind::Hits, "hits"),
        (
            TaskResult::SimilarityReport(SimilarityReport::default()),
            ResultKind::SimilarityReport,
            "similarity_report",
        ),
        (TaskResult::Scores(vec![]), ResultKind::Scores, "scores"),
    ];
    for (r, k, n) in cases {
        assert_eq!(r.kind(), k);
        assert_eq!(k.name(), n);
    }
}

#[test]
fn accessors_match_only_own_variant() {
    let r = TaskResult::Scores(vec![1.0]);
    assert_eq!(r.as_scores(), Some(&[1.0f32][..]));
    assert!(r.as_embeddings().is_none());
    assert!(r.as_entry_refs().is_none());
    assert!(r.as_hits().is_none());
    assert!(r.as_report().is_none());
    let r = TaskResult::EntryRefs(vec![EntryId(1)]);
    assert_eq!(r.as_entry_refs(), Some(&[EntryId(1)][..]));
    assert!(r.as_scores().is_none());
}

#[test]
fn empty_lists_of_different_kinds_differ() {
    assert_ne!(TaskResult::Embeddings(vec![]), TaskResult::Scores(vec![]));
    assert_eq!(TaskResult::Hits(vec![]), TaskResult::Hits(vec![]));
}

#[test]
fn float_equality_is_bitwise() {
    let nan = f32::NAN;
    assert_eq!(TaskResult::Scores(vec![nan]), TaskResult::Scores(vec![nan]));
    assert_ne!(TaskResult::Scores(vec![0.0]), TaskResult::Scores(vec![-0.0]));
    assert_eq!(
        h(&TaskResult::Scores(vec![nan])),
        h(&TaskResult::Scores(vec![nan]))
    );
}

#[test]
fn order_and_length_sensitive() {
    assert_ne!(
        TaskResult::Scores(vec![1.0, 2.0]),
        TaskResult::Scores(vec![2.0, 1.0])
    );
    assert_ne!(
        TaskResult::Embeddings(vec![vec![1.0], vec![2.0]]),
        TaskResult::Embeddings(vec![vec![1.0, 2.0]])
    );
    assert_ne!(
        TaskResult::Embeddings(vec![vec![1.0]]),
        TaskResult::Embeddings(vec![vec![1.0], vec![]])
    );
}

#[test]
fn hits_and_reports_compare_and_hash() {
    let hit = Hit { id: EntryId(3), score: 0.5 };
    let a = TaskResult::Hits(vec![hit]);
    let b = a.clone();
    assert_eq!(a, b);
    assert_eq!(h(&a), h(&b));
    assert_ne!(a, TaskResult::Hits(vec![Hit { id: EntryId(3), score: 0.6 }]));

    let p = SimilarityPair { a: EntryId(1), b: EntryId(2), score: 0.9 };
    let r1 = TaskResult::SimilarityReport(SimilarityReport { pairs: vec![p] });
    let r2 = r1.clone();
    assert_eq!(r1, r2);
    assert_eq!(h(&r1), h(&r2));
    assert_ne!(r1, TaskResult::SimilarityReport(SimilarityReport::default()));
}
