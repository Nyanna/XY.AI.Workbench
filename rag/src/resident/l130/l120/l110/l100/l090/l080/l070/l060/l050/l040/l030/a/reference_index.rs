use std::collections::BTreeMap;
use std::sync::{Arc, Mutex, RwLock};

use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::a::index_contract::{
    AccessMethod, EntryId, Index, IndexError, Payload, PayloadRef, SharedBytes, Stamp,
    StampObserver, StampSignal,
};
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::l010::change_stamp::StampCounter;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::l010::layout::DType;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::l010::scopes_and_entry_sets::EntrySet;

type Entries = BTreeMap<EntryId, (Stamp, Payload)>;

/// In-memory `Index` implementation that stores bytes unchanged.
pub struct ReferenceIndex {
    dtype: DType,
    entries: RwLock<Entries>,
    writer: Mutex<()>,
    counter: StampCounter,
    signal: Arc<StampSignal>,
}

impl ReferenceIndex {
    pub fn new(dtype: DType) -> Self {
        Self {
            dtype,
            entries: RwLock::new(BTreeMap::new()),
            writer: Mutex::new(()),
            counter: StampCounter::new(),
            signal: StampSignal::new(Stamp(0)),
        }
    }

    pub fn new_shared(dtype: DType) -> Arc<ReferenceIndex> {
        Arc::new(Self::new(dtype))
    }

    pub fn entry_set(&self) -> EntrySet {
        let map = self.read_map();
        EntrySet::from_ids(map.keys().map(|id| id.0))
    }

    pub fn len(&self) -> usize {
        self.read_map().len()
    }

    pub fn is_empty(&self) -> bool {
        self.read_map().is_empty()
    }

    fn read_map(&self) -> std::sync::RwLockReadGuard<'_, Entries> {
        self.entries.read().unwrap_or_else(|e| e.into_inner())
    }

    fn write_map(&self) -> std::sync::RwLockWriteGuard<'_, Entries> {
        self.entries.write().unwrap_or_else(|e| e.into_inner())
    }

    fn next_stamp(&self) -> Result<Stamp, IndexError> {
        self.counter
            .next()
            .map(|s| Stamp(s.as_raw()))
            .map_err(|e| IndexError::Backend(e.to_string()))
    }
}

impl Index for ReferenceIndex {
    fn dtype(&self) -> DType {
        self.dtype
    }

    fn access_methods(&self) -> &[AccessMethod] {
        &[]
    }

    fn stamp(&self, id: EntryId) -> Option<Stamp> {
        self.read_map().get(&id).map(|(stamp, _)| *stamp)
    }

    fn stamps(&self) -> Vec<(EntryId, Stamp)> {
        self.read_map()
            .iter()
            .map(|(id, (stamp, _))| (*id, *stamp))
            .collect()
    }

    fn read(&self, id: EntryId) -> Payload {
        self.read_map()
            .get(&id)
            .map(|(_, payload)| payload.clone())
            .unwrap_or_default()
    }

    fn write(&self, id: EntryId, payload: PayloadRef<'_>) -> Result<Stamp, IndexError> {
        let elem_size = self.dtype.size_in_bytes();
        if payload.vector.len() % elem_size != 0 {
            return Err(IndexError::InvalidVectorLength {
                len: payload.vector.len(),
                elem_size,
            });
        }
        let _writer = self.writer.lock().unwrap_or_else(|e| e.into_inner());
        let stamp = self.next_stamp()?;
        let stored = Payload {
            vector: SharedBytes(Arc::from(payload.vector)),
            meta: SharedBytes(Arc::from(payload.meta)),
        };
        {
            let mut map = self.write_map();
            map.insert(id, (stamp, stored));
        }
        self.signal.publish(stamp);
        Ok(stamp)
    }

    fn delete(&self, id: EntryId) -> Result<Option<Stamp>, IndexError> {
        let _writer = self.writer.lock().unwrap_or_else(|e| e.into_inner());
        if !self.read_map().contains_key(&id) {
            return Ok(None);
        }
        let stamp = self.next_stamp()?;
        {
            let mut map = self.write_map();
            map.remove(&id);
        }
        self.signal.publish(stamp);
        Ok(Some(stamp))
    }

    fn subscribe(&self) -> StampObserver {
        self.signal.observer()
    }
}

#[cfg(test)]
#[path = "test/reference_index.rs"]
mod tests;
