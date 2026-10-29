//! `MemoryManager` contract plus a GPU-free bookkeeping implementation.

use parking_lot::Mutex;

use super::dtype::DType;
use super::freelist::FreeList;
use super::region::{ArenaId, Region};
use crate::resident::error::{IndexError, ManagerError};
use crate::resident::failpoint;

#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DeviceKind {
    Cpu,
    Gpu,
}

pub fn compute_dtype(dev: DeviceKind) -> DType {
    match dev {
        DeviceKind::Gpu => DType::F16,
        DeviceKind::Cpu => DType::F32,
    }
}

pub trait MemoryManager {
    fn alloc(&self, dev: DeviceKind, bytes: u64, align: u64) -> Result<Region, IndexError>;

    /// Stale handles are ignored (logged), never dereferenced.
    fn free(&self, region: Region);
}

struct Slot {
    offset: u64,
    len: u64,
    gen: u32,
    live: bool,
}

impl Slot {
    fn matches(&self, r: &Region) -> bool {
        self.live && self.offset == r.offset && self.len == r.len && self.gen == r.gen
    }
}

struct Inner {
    list: FreeList,
    slots: Vec<Slot>,
}

/// Single-arena allocator tracking offsets and generations only; no device memory.
pub struct BookkeepingArena {
    id: ArenaId,
    inner: Mutex<Inner>,
}

impl BookkeepingArena {
    pub fn new(id: ArenaId, capacity: u64) -> Self {
        Self { id, inner: Mutex::new(Inner { list: FreeList::new(capacity), slots: Vec::new() }) }
    }

    pub fn available(&self) -> u64 {
        self.inner.lock().list.available()
    }

    pub fn is_live(&self, r: Region) -> bool {
        r.arena == self.id && self.inner.lock().slots.iter().any(|s| s.matches(&r))
    }

    pub(crate) fn try_free(&self, region: Region) -> Result<(), ManagerError> {
        let stale = || ManagerError::StaleRegion { arena: region.arena.0, gen: region.gen };
        if region.arena != self.id {
            return Err(stale());
        }
        let mut g = self.inner.lock();
        let slot = g.slots.iter_mut().find(|s| s.matches(&region)).ok_or_else(stale)?;
        slot.live = false;
        g.list.free(region.offset, region.len);
        tracing::debug!(arena = self.id.0, offset = region.offset, bytes = region.len, "free");
        Ok(())
    }
}

impl MemoryManager for BookkeepingArena {
    fn alloc(&self, _dev: DeviceKind, bytes: u64, align: u64) -> Result<Region, IndexError> {
        let mut g = self.inner.lock();
        let available = g.list.available();
        let exhausted = IndexError::ArenaExhausted {
            requested: bytes as usize,
            available: available as usize,
        };
        if failpoint::hit("memory.alloc", u64::from(self.id.0)) {
            return Err(exhausted);
        }
        let offset = g.list.alloc(bytes, align).ok_or(exhausted)?;
        let gen = match g.slots.iter_mut().find(|s| !s.live && s.offset == offset && s.len == bytes) {
            Some(s) => {
                s.gen = s.gen.wrapping_add(1);
                s.live = true;
                s.gen
            }
            None => {
                g.slots.push(Slot { offset, len: bytes, gen: 0, live: true });
                0
            }
        };
        tracing::debug!(arena = self.id.0, offset, bytes, "alloc");
        Ok(Region { arena: self.id, offset, len: bytes, gen })
    }

    fn free(&self, region: Region) {
        if let Err(e) = self.try_free(region) {
            tracing::warn!(error = %e, "ignored stale free");
        }
    }
}

#[cfg(test)]
#[path = "test/manager.rs"]
mod tests;
