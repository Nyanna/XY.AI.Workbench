//! Consistency model: entry identity checks against the index, serialized index
//! writes, and copy-on-write table publication with gap-deferred mapping changes.
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::a::table_swap::{
    Epoch, SegmentTable, TableSwap, TableSwapError,
};
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::a::index_contract::{
    EntryId, Index, IndexError, PayloadRef, Stamp,
};
const MAX_PUBLISH_ATTEMPTS: usize = 3;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct EntryIdentity {
    pub id: EntryId,
    pub stamp: Stamp,
}
impl EntryIdentity {
    pub fn new(id: EntryId, stamp: Stamp) -> EntryIdentity {
        EntryIdentity { id, stamp }
    }
    pub fn of(index: &dyn Index, id: EntryId) -> Option<EntryIdentity> {
        index.stamp(id).map(|s| EntryIdentity::new(id, s))
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Verdict {
    Current,
    Stale { winner: EntryIdentity },
    Gone,
}
fn judge2(held: &EntryIdentity, current: Option<Stamp>) -> Verdict {
    match current {
        None => Verdict::Gone,
        Some(s) => {
            let winner = EntryIdentity::new(held.id, s);
            if winner.stamp == held.stamp {
                Verdict::Current
            } else {
                Verdict::Stale { winner }
            }
        }
    }
}
pub fn check(index: &dyn Index, held: &EntryIdentity) -> Verdict {
    judge2(held, index.stamp(held.id))
}
/// One `stamps()` snapshot for all; results follow the order of `held`.
pub fn check_many(index: &dyn Index, held: &[EntryIdentity]) -> Vec<Verdict> {
    if held.is_empty() {
        return Vec::new();
    }
    let snapshot = index.stamps();
    held.iter()
        .map(|h| {
            let current = snapshot
                .binary_search_by_key(&h.id, |(id, _)| *id)
                .ok()
                .map(|at| snapshot[at].1);
            judge2(h, current)
        })
        .collect()
}
pub struct IndexWriter {
    index: Arc<dyn Index>,
    serial: Mutex<()>,
}
impl IndexWriter {
    pub fn new(index: Arc<dyn Index>) -> IndexWriter {
        IndexWriter {
            index,
            serial: Mutex::new(()),
        }
    }
    pub fn write(
        &self,
        id: EntryId,
        payload: PayloadRef<'_>,
    ) -> Result<EntryIdentity, IndexError> {
        let _g = lock(&self.serial);
        let stamp = self.index.write(id, payload)?;
        Ok(EntryIdentity::new(id, stamp))
    }
    pub fn write_many(
        &self,
        items: &[(EntryId, PayloadRef<'_>)],
    ) -> Result<Vec<EntryIdentity>, IndexError> {
        let _g = lock(&self.serial);
        let stamps = self.index.write_many(items)?;
        Ok(stamps.into_iter().map(|(id, stamp)| EntryIdentity::new(id, stamp)).collect())
    }
    pub fn delete(&self, id: EntryId) -> Result<Option<Stamp>, IndexError> {
        let _g = lock(&self.serial);
        self.index.delete(id)
    }
}
/// Runs in the gap after the owning table is published; the argument is that table's epoch.
pub type MappingChange = Box<dyn FnOnce(Epoch) + Send>;
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct GapReport2 {
    pub mappings_applied: usize,
    pub reclaimed: usize,
}
pub struct TablePublisher {
    swap: Arc<TableSwap>,
    apply_lock: Mutex<()>,
    pending: Mutex<VecDeque<(Epoch, MappingChange)>>,
    gap_lock: Mutex<()>,
}
impl TablePublisher {
    pub fn new(swap: Arc<TableSwap>) -> TablePublisher {
        TablePublisher {
            swap,
            apply_lock: Mutex::new(()),
            pending: Mutex::new(VecDeque::new()),
            gap_lock: Mutex::new(()),
        }
    }
    pub fn apply<F: FnMut(&mut SegmentTable)>(
        &self,
        f: F,
    ) -> Result<Epoch, TableSwapError> {
        let _serial = lock(&self.apply_lock);
        self.publish_locked(f)
    }
    pub fn apply_with_mapping<F: FnMut(&mut SegmentTable)>(
        &self,
        f: F,
        mapping: MappingChange,
    ) -> Result<Epoch, TableSwapError> {
        let _serial = lock(&self.apply_lock);
        let epoch = self.publish_locked(f)?;
        lock(&self.pending).push_back((epoch, mapping));
        Ok(epoch)
    }
    fn publish_locked<F: FnMut(&mut SegmentTable)>(
        &self,
        mut f: F,
    ) -> Result<Epoch, TableSwapError> {
        let mut last = None;
        for _ in 0..MAX_PUBLISH_ATTEMPTS {
            let mut table = self.swap.next_table();
            f(&mut table);
            match self.swap.publish(table) {
                Ok(epoch) => return Ok(epoch),
                Err(e @ TableSwapError::StaleMapping { .. }) => last = Some(e),
                Err(e) => return Err(e),
            }
        }
        Err(last.expect("at least one publish attempt"))
    }
    pub fn run_gap(&self) -> GapReport2 {
        let _gap = lock(&self.gap_lock);
        let drained = std::mem::take(&mut *lock(&self.pending));
        let mappings_applied = drained.len();
        for (epoch, mapping) in drained {
            mapping(epoch);
        }
        let reclaimed = self.swap.reclaim(Epoch::MAX);
        GapReport2 {
            mappings_applied,
            reclaimed,
        }
    }
    pub fn pending_mappings(&self) -> usize {
        lock(&self.pending).len()
    }
}
#[cfg(test)]
#[path = "test/consistency.rs"]
mod tests;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::_1::sync_util::lock;
