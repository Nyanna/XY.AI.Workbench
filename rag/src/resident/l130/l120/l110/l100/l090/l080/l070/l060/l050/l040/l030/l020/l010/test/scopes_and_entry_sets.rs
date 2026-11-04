use super::*;
use std::collections::hash_map::DefaultHasher;
use std::collections::BTreeSet;

fn hash_of(s: &EntrySet) -> u64 {
    let mut h = DefaultHasher::new();
    s.hash(&mut h);
    h.finish()
}

fn is_bitmap(s: &EntrySet) -> bool {
    matches!(s.repr, Repr::Bitmap(_))
}

#[test]
fn scope_names_and_kinds() {
    let cases = [
        (Scope::All, "all"),
        (Scope::Range { start: 5, end: 1 }, "range"),
        (Scope::Set(EntrySet::empty()), "set"),
        (Scope::Subtree { root: 1 }, "subtree"),
        (Scope::Prefix { bytes: vec![] }, "prefix"),
        (Scope::Centroid { count: 3, radius: None }, "centroid"),
    ];
    for (s, n) in cases {
        assert_eq!(s.name(), n);
        assert_eq!(s.kind().name(), n);
    }
    assert_eq!(
        Scope::Set(EntrySet::from_ids([1, 2])),
        Scope::Set(EntrySet::from_ranges([1..3]))
    );
}

#[test]
fn construction_and_len() {
    let s = EntrySet::from_ranges([5..5, 9..3, 10..20, 15..30, 30..31, 0..2]);
    assert_eq!(s.runs(), vec![0..=1, 10..=30]);
    assert_eq!(s.len(), 23);
    let full = EntrySet::from_ranges([0..u64::MAX]).union(&EntrySet::from_ids([u64::MAX]));
    assert_eq!(full.len(), 1u128 << 64);
    assert!(full.contains(0) && full.contains(u64::MAX));
    assert_eq!(full.runs(), vec![0..=u64::MAX]);
    assert!(EntrySet::empty().is_empty());
    assert_eq!(EntrySet::default(), EntrySet::empty());
}

#[test]
fn iter_at_max() {
    let s = EntrySet::from_ids([u64::MAX, u64::MAX - 1, 0, 7, 7]);
    let v: Vec<_> = s.iter().collect();
    assert_eq!(v, vec![0, 7, u64::MAX - 1, u64::MAX]);
}

#[test]
fn bitmap_equals_ranges() {
    let ids: Vec<u64> = (0..2000).map(|i| i * 3).collect();
    let a = EntrySet::from_ids(ids.iter().copied());
    assert!(is_bitmap(&a));
    let b = EntrySet {
        repr: Repr::Runs(ids.iter().map(|&i| (i, i)).collect()),
        len: ids.len() as u128,
    };
    assert_eq!(a, b);
    assert_eq!(hash_of(&a), hash_of(&b));
    assert_eq!(a.iter().collect::<Vec<_>>(), ids);
    assert_eq!(a.runs().len(), ids.len());
    assert!(a.contains(3) && !a.contains(4));
}

#[test]
fn bitmap_runs_cross_boundaries() {
    let mut ranges = vec![60..70, 65530..65545, 131000..131100];
    for i in 0..100u64 {
        ranges.push(1_000_000 + i * 2..1_000_000 + i * 2 + 1);
    }
    let a = EntrySet::from_ranges(ranges.clone());
    let expect: BTreeSet<u64> = ranges.into_iter().flatten().collect();
    assert_eq!(a.iter().collect::<Vec<_>>(), expect.iter().copied().collect::<Vec<_>>());
    assert_eq!(a.len(), expect.len() as u128);
    let runs = a.runs();
    for w in runs.windows(2) {
        assert!(*w[0].end() + 1 < *w[1].start());
    }
}

#[test]
fn set_algebra_matches_reference() {
    let a_ids: Vec<u64> = (0..500).map(|i| i * 5).chain([u64::MAX]).collect();
    let b_ids: Vec<u64> = (0..500).map(|i| i * 7).chain([u64::MAX, 3]).collect();
    let a = EntrySet::from_ids(a_ids.clone());
    let b = EntrySet::from_ids(b_ids.clone());
    let sa: BTreeSet<u64> = a_ids.into_iter().collect();
    let sb: BTreeSet<u64> = b_ids.into_iter().collect();
    let u: Vec<_> = sa.union(&sb).copied().collect();
    let i: Vec<_> = sa.intersection(&sb).copied().collect();
    assert_eq!(a.union(&b).iter().collect::<Vec<_>>(), u);
    assert_eq!(b.union(&a).iter().collect::<Vec<_>>(), u);
    assert_eq!(a.intersection(&b).iter().collect::<Vec<_>>(), i);
    assert!(a.intersection(&EntrySet::empty()).is_empty());
    assert_eq!(a.union(&EntrySet::empty()), a);
}

#[test]
fn constant_memory_for_ranges() {
    let s = EntrySet::from_ranges([0..u64::MAX]);
    match &s.repr {
        Repr::Runs(v) => assert_eq!(v.len(), 1),
        _ => panic!("expected runs"),
    }
    assert_eq!(s.len(), u64::MAX as u128);
}

#[test]
fn thread_safety() {
    fn check<T: Send + Sync + 'static>() {}
    check::<Scope>();
    check::<ScopeKind>();
    check::<EntrySet>();
}
