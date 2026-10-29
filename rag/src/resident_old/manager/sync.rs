//! State-based reconciliation between the index and a resident mirror. The change signal only
//! wakes the scanner; correctness comes from comparing the stamps of resident entries.

use std::collections::HashMap;

use tokio::sync::watch;

use crate::resident::index::{EntryData, EntryId, Index, Stamp};
use crate::resident::manager::slot::{Update, UpdateQueue};
use crate::resident::manager::tracker::ResidencyTracker;

/// Bound on stamp-recheck repetitions per load. Exhaustion falls back to the stamp read
/// first, which is the safe direction (at worst one redundant reload).
const MAX_RECHECKS: u32 = 8;

#[derive(Debug, Default, PartialEq, Eq)]
pub struct ScanOutcome {
    /// Resident entries whose index stamp differs from the known one.
    pub reload: Vec<EntryId>,
    /// Resident entries no longer present in the index.
    pub removed: Vec<EntryId>,
    /// Resident entries already at the index stamp, including echoes of own writes.
    pub current: usize,
    pub watermark: u64,
}

impl ScanOutcome {
    pub fn is_clean(&self) -> bool {
        self.reload.is_empty() && self.removed.is_empty()
    }

    /// Removals are mutations of the mapping and therefore go through the update queue.
    pub fn enqueue_removals(&self, queue: &mut UpdateQueue) {
        for entry in &self.removed {
            queue.push(Update::Unbind { entry: *entry });
        }
    }
}

/// One scanner per subscriber (device manager, persistence timer, host mirror); each owns its
/// receiver and therefore its own view of what it has seen.
pub struct SyncScanner {
    rx: watch::Receiver<u64>,
}

impl SyncScanner {
    pub fn new(index: &dyn Index) -> Self {
        Self { rx: index.changed() }
    }

    /// Resolves once an unseen signal exists; `false` when the index is gone.
    pub async fn changed(&mut self) -> bool {
        self.rx.changed().await.is_ok()
    }

    /// A signal arrived since the last scan acknowledged the channel.
    pub fn has_pending(&self) -> bool {
        self.rx.has_changed().unwrap_or(false)
    }

    pub fn scan(&mut self, index: &dyn Index, tracker: &ResidencyTracker) -> ScanOutcome {
        // Acknowledge before comparing: a send during the scan stays unseen and
        // triggers the next scan instead of being lost.
        let watermark = *self.rx.borrow_and_update();
        let _span = tracing::info_span!("sync_scan", watermark).entered();

        let mut live: HashMap<EntryId, Stamp> = HashMap::with_capacity(tracker.len());
        for (id, stamp) in index.stamps() {
            if tracker.get(id).is_some() {
                live.insert(id, stamp);
            }
        }

        let mut out = ScanOutcome { watermark, ..ScanOutcome::default() };
        for (id, resident) in tracker.iter() {
            match live.get(&id) {
                None => out.removed.push(id),
                Some(stamp) if *stamp != resident.stamp => out.reload.push(id),
                Some(_) => out.current += 1,
            }
        }
        out.reload.sort_unstable();
        out.removed.sort_unstable();
        tracing::debug!(
            reload = out.reload.len(),
            removed = out.removed.len(),
            current = out.current,
            "scan result"
        );
        out
    }
}

pub struct Loaded<'a> {
    /// Stamp read before the data; never newer than the data.
    pub stamp: Stamp,
    pub data: EntryData<'a>,
    pub rechecks: u32,
}

/// Reads stamp, then data, then the stamp again. Registering a stamp read after the data
/// could mark stale data as current; the reverse order only risks a redundant reload.
pub fn load_entry(index: &dyn Index, entry: EntryId) -> Option<Loaded<'_>> {
    let mut rechecks = 0;
    loop {
        let first = index.stamp(entry)?;
        let data = index.read(entry);
        if index.stamp(entry) == Some(first) || rechecks >= MAX_RECHECKS {
            return Some(Loaded { stamp: first, data, rechecks });
        }
        rechecks += 1;
        tracing::debug!(entry = entry.get(), rechecks, "stamp moved during load, repeating");
    }
}

#[cfg(test)]
#[path = "test/sync.rs"]
mod tests;
