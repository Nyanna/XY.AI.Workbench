//! Mapping between index entries and resident slots, with the stamp known per entry.

use std::collections::HashMap;

use crate::resident::index::{EntryId, Stamp};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SlotRef {
    pub segment: u32,
    pub slot: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Resident {
    pub slot: SlotRef,
    pub stamp: Stamp,
}

#[derive(Debug, Default)]
pub struct ResidencyTracker {
    entries: HashMap<EntryId, Resident>,
}

impl ResidencyTracker {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, entry: EntryId, slot: SlotRef, stamp: Stamp) -> Option<Resident> {
        self.entries.insert(entry, Resident { slot, stamp })
    }

    pub fn remove(&mut self, entry: EntryId) -> Option<Resident> {
        self.entries.remove(&entry)
    }

    pub fn get(&self, entry: EntryId) -> Option<Resident> {
        self.entries.get(&entry).copied()
    }

    pub fn known_stamp(&self, entry: EntryId) -> Option<Stamp> {
        self.entries.get(&entry).map(|r| r.stamp)
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = (EntryId, Resident)> + '_ {
        self.entries.iter().map(|(id, r)| (*id, *r))
    }
}

#[cfg(test)]
mod tests {
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
}
