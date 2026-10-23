use super::*;

fn id(n: u64) -> EntryId {
    EntryId::new(n)
}

#[test]
fn apply_returns_the_stamp_later_reported() {
    let ix = InMemoryIndex::new();
    let s = ix.apply(id(1), b"a");
    assert_eq!(ix.stamp(id(1)), Some(s));
    assert_eq!(ix.read(id(1)).as_bytes(), b"a");
}

#[test]
fn rewrite_changes_stamp() {
    let ix = InMemoryIndex::new();
    let s1 = ix.apply(id(1), b"a");
    let s2 = ix.apply(id(1), b"a");
    assert_ne!(s1, s2);
}

#[test]
fn remove_then_readd_yields_new_stamp_no_aba() {
    let ix = InMemoryIndex::new();
    let s1 = ix.apply(id(1), b"a");
    let removed = ix.remove(id(1)).unwrap();
    assert_ne!(removed, s1);
    assert_eq!(ix.stamp(id(1)), None);
    assert!(ix.read(id(1)).as_bytes().is_empty());
    let s2 = ix.apply(id(1), b"a");
    assert_ne!(s2, s1);
    assert_ne!(s2, removed);
    assert!(ix.remove(id(9)).is_none());
}

#[test]
fn watch_carries_high_water_and_echo_is_recognisable() {
    let ix = InMemoryIndex::new();
    let mut rx = ix.changed();
    let own = ix.apply(id(1), b"x");
    assert!(rx.has_changed().unwrap());
    assert_eq!(*rx.borrow_and_update(), own.raw());
    assert!(!rx.has_changed().unwrap());
    let foreign = ix.apply(id(2), b"y");
    assert!(rx.has_changed().unwrap());
    assert_eq!(*rx.borrow_and_update(), foreign.raw());
    assert_ne!(foreign, own);
    assert_eq!(ix.high_water(), foreign.raw());
}

#[test]
fn coalesced_signals_still_expose_all_stamps() {
    let ix = InMemoryIndex::new();
    let mut rx = ix.changed();
    ix.apply(id(1), b"a");
    ix.apply(id(2), b"b");
    let mark = *rx.borrow_and_update();
    assert_eq!(mark, ix.high_water());
    assert_eq!(ix.stamps().count(), 2);
}

#[test]
fn resumed_counter_never_reissues_old_stamps() {
    let ix = InMemoryIndex::with_counter(StampCounter::resume(100, 90));
    assert_eq!(ix.high_water(), 100);
    assert_eq!(ix.apply(id(1), b"a").raw(), 101);
}

#[test]
fn members_and_stamps_snapshot() {
    let ix = InMemoryIndex::new();
    ix.apply(id(3), b"c");
    ix.apply(id(1), b"a");
    ix.apply(id(2), b"b");
    let m = ix.members(CellId(0));
    assert_eq!(m.ranges(), &[1..4]);
    assert!(ix.members(CellId(1)).is_empty());
    let ids: Vec<u64> = ix.stamps().map(|(e, _)| e.get()).collect();
    assert_eq!(ids, vec![1, 2, 3]);
}

#[test]
fn entry_set_merges_and_contains() {
    let s = EntrySet::from_ranges([5..8, 1..3, 3..4, 7..10, 12..12]);
    assert_eq!(s.ranges(), &[1..4, 5..10]);
    assert_eq!(s.len(), 8);
    assert!(s.contains(id(9)) && !s.contains(id(4)) && !s.contains(id(10)));
}
