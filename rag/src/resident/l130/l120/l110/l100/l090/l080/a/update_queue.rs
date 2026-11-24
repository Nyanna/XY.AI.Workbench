use std::collections::BTreeMap;
use std::sync::{Mutex, MutexGuard};
use crate::resident::l130::l120::l110::l100::l090::l080::l070::a::reconcile::{
    ResidencyDirectory, SlotRef,
};
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::a::address_map::AddressMap;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::a::slot_layout::EntryRef;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::a::index_contract::{
    EntryId, Stamp,
};
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UpdateKind {
    /// Detach the entry from `slot` and remove it from the directory.
    Unbind,
    /// Attach the entry to `slot`; a previous binding is released first.
    Bind,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Update {
    pub entry: EntryRef,
    pub slot: SlotRef,
    pub stamp: Stamp,
    pub kind: UpdateKind,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Warning {
    /// The segment of the target slot is not present in the map.
    UnmappedTarget { entry: EntryRef, slot: SlotRef, kind: UpdateKind },
    /// Bind refused (slot out of range or occupied by another entry); not applied.
    BindRejected { entry: EntryRef, slot: SlotRef },
}
/// Pending slot updates, one per entry (the latest wins).
#[derive(Debug, Default)]
pub struct UpdateQueue {
    pending: Mutex<BTreeMap<EntryId, Update>>,
    warnings: Mutex<Vec<Warning>>,
}
impl UpdateQueue {
    pub fn new() -> Self {
        Self::default()
    }
    fn lock_pending(&self) -> MutexGuard<'_, BTreeMap<EntryId, Update>> {
        self.pending.lock().unwrap_or_else(|e| e.into_inner())
    }
    fn lock_warnings(&self) -> MutexGuard<'_, Vec<Warning>> {
        self.warnings.lock().unwrap_or_else(|e| e.into_inner())
    }
    fn push(&self, update: Update) {
        self.lock_pending().insert(update.entry.entry, update);
    }
    /// Replaces any pending update of the same `entry.entry`.
    pub fn push_unbind(&self, entry: EntryRef, slot: SlotRef, stamp: Stamp) {
        self.push(Update {
            entry,
            slot,
            stamp,
            kind: UpdateKind::Unbind,
        });
    }
    /// Replaces any pending update of the same `entry.entry`.
    pub fn push_bind(&self, entry: EntryRef, slot: SlotRef, stamp: Stamp) {
        self.push(Update {
            entry,
            slot,
            stamp,
            kind: UpdateKind::Bind,
        });
    }
    pub fn contains(&self, id: EntryId) -> bool {
        self.lock_pending().contains_key(&id)
    }
    pub fn get(&self, id: EntryId) -> Option<Update> {
        self.lock_pending().get(&id).copied()
    }
    /// Number of pending updates (unbinds and binds).
    pub fn depth(&self) -> usize {
        self.lock_pending().len()
    }
    /// Number of pending binds.
    pub fn deferred_writes(&self) -> usize {
        self.lock_pending().values().filter(|u| u.kind == UpdateKind::Bind).count()
    }
    pub fn is_empty(&self) -> bool {
        self.lock_pending().is_empty()
    }
    /// Applies all pending updates, unbinds first, then binds, each ascending by entry id.
    /// Returns the number of applied updates.
    pub fn apply(&self, dir: &ResidencyDirectory, map: &AddressMap) -> usize {
        let batch = std::mem::take(&mut *self.lock_pending());
        if batch.is_empty() {
            return 0;
        }
        let mut warnings = Vec::new();
        let mut applied = 0;
        for update in batch.values().filter(|u| u.kind == UpdateKind::Unbind) {
            Self::apply_unbind(update, dir, map, &mut warnings);
            applied += 1;
        }
        for update in batch.values().filter(|u| u.kind == UpdateKind::Bind) {
            if Self::apply_bind(update, dir, map, &mut warnings) {
                applied += 1;
            }
        }
        if !warnings.is_empty() {
            self.lock_warnings().extend(warnings);
        }
        applied
    }
    fn apply_unbind(
        update: &Update,
        dir: &ResidencyDirectory,
        map: &AddressMap,
        warnings: &mut Vec<Warning>,
    ) {
        let Update { entry, slot, .. } = *update;
        dir.remove(entry.entry);
        match map.get(slot.segment) {
            None => {
                warnings
                    .push(Warning::UnmappedTarget {
                        entry,
                        slot,
                        kind: UpdateKind::Unbind,
                    })
            }
            Some(segment) => {
                let assignment = segment.assignment();
                if assignment.slot_of_entry(entry) == Some(slot.slot) {
                    let _ = assignment.unbind(slot.slot);
                }
            }
        }
    }
    fn apply_bind(
        update: &Update,
        dir: &ResidencyDirectory,
        map: &AddressMap,
        warnings: &mut Vec<Warning>,
    ) -> bool {
        let Update { entry, slot, stamp, .. } = *update;
        let Some(target) = map.get(slot.segment) else {
            warnings
                .push(Warning::UnmappedTarget {
                    entry,
                    slot,
                    kind: UpdateKind::Bind,
                });
            return false;
        };
        if let Some(old) = dir.get(entry.entry) {
            if let Some(previous) = map.get(old.slot.segment) {
                let assignment = previous.assignment();
                if assignment.slot_of_entry(entry) == Some(old.slot.slot) {
                    let _ = assignment.unbind(old.slot.slot);
                }
            }
        }
        let assignment = target.assignment();
        if let Some(current) = assignment.slot_of_entry(entry) {
            let _ = assignment.unbind(current);
        }
        if assignment.bind(slot.slot, entry).is_err() {
            warnings
                .push(Warning::BindRejected {
                    entry,
                    slot,
                });
            return false;
        }
        dir.insert(entry.entry, slot, stamp);
        true
    }
    /// Returns the warnings collected since the last call and clears them.
    pub fn take_warnings(&self) -> Vec<Warning> {
        std::mem::take(&mut *self.lock_warnings())
    }
}
#[cfg(test)]
#[path = "test/update_queue.rs"]
mod tests;
