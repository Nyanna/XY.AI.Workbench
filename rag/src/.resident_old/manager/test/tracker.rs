use super::*;

#[test]
fn insert_replace_remove() {
    let mut t = ResidencyTracker::new();
    let id = EntryId::new(7);
    let slot = SlotRef { segment: 1, slot: 2 };
    assert!(t.is_empty());
    assert!(t.insert(id, slot, Stamp::from_raw(1)).is_none());
    assert_eq!(t.known_stamp(id), Some(Stamp::from_raw(1)));
    let old = t.insert(id, slot, Stamp::from_raw(5)).unwrap();
    assert_eq!(old.stamp, Stamp::from_raw(1));
    assert_eq!(t.len(), 1);
    assert_eq!(t.get(id).unwrap().slot, slot);
    assert!(t.remove(id).is_some());
    assert!(t.known_stamp(id).is_none());
}
