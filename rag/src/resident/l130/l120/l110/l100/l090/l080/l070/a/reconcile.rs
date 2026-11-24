use std::collections::BTreeMap;
use std::sync::{Arc, Mutex, MutexGuard, Weak};
use std::time::Duration;
pub use super::super::l060::l050::a::segments::{SegmentId, SlotRef};
use super::super::l060::l050::l040::l030::l020::a::index_contract::{
    EntryId, Index, Payload, Stamp, StampObserver,
};
pub const MAX_LOAD_RETRIES: u32 = 8;
const WAIT_SLICE: Duration = Duration::from_millis(100);
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Resident {
    pub slot: SlotRef,
    pub stamp: Stamp,
}
#[derive(Debug, Default)]
pub struct ResidencyDirectory {
    map: Mutex<BTreeMap<EntryId, Resident>>,
}
impl ResidencyDirectory {
    pub fn new() -> Self {
        Self::default()
    }
    fn lock(&self) -> MutexGuard<'_, BTreeMap<EntryId, Resident>> {
        self.map.lock().unwrap_or_else(|e| e.into_inner())
    }
    /// Inserts or replaces; returns the previous entry.
    pub fn insert(&self, id: EntryId, slot: SlotRef, known: Stamp) -> Option<Resident> {
        let resident = Resident { slot, stamp: known };
        self.lock().insert(id, resident)
    }
    pub fn remove(&self, id: EntryId) -> Option<Resident> {
        self.lock().remove(&id)
    }
    pub fn get(&self, id: EntryId) -> Option<Resident> {
        self.lock().get(&id).copied()
    }
    pub fn stamp(&self, id: EntryId) -> Option<Stamp> {
        self.lock().get(&id).map(|r| r.stamp)
    }
    /// Records the return stamp of an own write as known; the slot stays unchanged.
    /// Returns `false` (no change) if `id` is not resident.
    pub fn set_known_stamp(&self, id: EntryId, known: Stamp) -> bool {
        match self.lock().get_mut(&id) {
            Some(resident) => {
                resident.stamp = known;
                true
            }
            None => false,
        }
    }
    pub fn len(&self) -> usize {
        self.lock().len()
    }
    pub fn is_empty(&self) -> bool {
        self.lock().is_empty()
    }
    /// Snapshot, ascending by entry id.
    pub fn iter(&self) -> Vec<(EntryId, Resident)> {
        self.lock().iter().map(|(id, r)| (*id, *r)).collect()
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoadedEntry {
    pub stamp: Stamp,
    pub payload: Payload,
    pub retries: u32,
}
/// Reads an entry in the order stamp, read, stamp. `None` if the entry is missing.
///
/// The returned stamp is never newer than the returned data.
pub fn load_entry(index: &dyn Index, id: EntryId) -> Option<LoadedEntry> {
    let mut first: Option<Stamp> = None;
    let mut retries = 0;
    loop {
        let before = index.stamp(id)?;
        let first_stamp = *first.get_or_insert(before);
        let payload = index.read(id);
        let after = index.stamp(id);
        if after == Some(before) {
            return Some(LoadedEntry {
                stamp: before,
                payload,
                retries,
            });
        }
        if retries >= MAX_LOAD_RETRIES {
            return Some(LoadedEntry {
                stamp: first_stamp,
                payload,
                retries,
            });
        }
        retries += 1;
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Reconciliation {
    pub removed: Vec<EntryId>,
    pub reload: Vec<EntryId>,
    pub current: usize,
    pub watermark: Stamp,
    pub clean: bool,
}
impl Reconciliation {
    /// Calls `f` for every removed entry in ascending order.
    pub fn for_each_delete<F: FnMut(EntryId)>(&self, mut f: F) {
        for id in &self.removed {
            f(*id);
        }
    }
}
pub struct Reconciler {
    index: Weak<dyn Index>,
    observer: StampObserver,
    seen: Stamp,
}
impl Reconciler {
    pub fn new(index: &Arc<dyn Index>) -> Reconciler {
        let observer = index.subscribe();
        let seen = observer.current();
        Reconciler {
            index: Arc::downgrade(index),
            observer,
            seen,
        }
    }
    fn alive(&self) -> bool {
        self.index.upgrade().is_some()
    }
    /// Blocks until an unseen signal is pending (`true`) or the index is gone (`false`).
    /// Does not acknowledge.
    pub fn wait(&mut self) -> bool {
        let mut probe = self.observer.clone();
        loop {
            if !self.alive() {
                return false;
            }
            if self.observer.current() > self.seen {
                return true;
            }
            if let Some(stamp) = probe.wait_newer(Some(WAIT_SLICE)) {
                if stamp > self.seen {
                    return true;
                }
            }
        }
    }
    pub fn pending(&self) -> bool {
        self.observer.current() > self.seen
    }
    /// Acknowledges the signal, then compares the resident entries with the index.
    /// `None` if the index no longer exists.
    pub fn reconcile(&mut self, dir: &ResidencyDirectory) -> Option<Reconciliation> {
        let index = self.index.upgrade()?;
        if let Some(stamp) = self.observer.wait_newer(Some(Duration::ZERO)) {
            self.seen = self.seen.max(stamp);
        }
        let watermark = self.seen;
        let mut removed = Vec::new();
        let mut reload = Vec::new();
        let mut current = 0;
        for (id, resident) in dir.iter() {
            match index.stamp(id) {
                None => removed.push(id),
                Some(stamp) if stamp != resident.stamp => reload.push(id),
                Some(_) => current += 1,
            }
        }
        let clean = removed.is_empty() && reload.is_empty();
        Some(Reconciliation {
            removed,
            reload,
            current,
            watermark,
            clean,
        })
    }
}
#[cfg(test)]
#[path = "test/reconcile.rs"]
mod tests;
