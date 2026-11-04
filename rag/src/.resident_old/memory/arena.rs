//! Per-class arena allocator with stream-ordered release, shared by every memory backend.

use parking_lot::Mutex;

use super::budget::{ArenaKind, ArenaSizes, Budget};
use super::error::MemoryError;
use super::manager::{BookkeepingArena, DeviceKind, MemoryManager};
use super::region::Region;
use crate::resident::error::IndexError;
use crate::resident::subengine::EventHandle;

/// Offset allocator for one arena class. A region released with `release_after` returns to the
/// free list only once its event has completed, so in-flight work never sees reused memory.
pub struct ArenaAllocator {
    kind: ArenaKind,
    dev: DeviceKind,
    capacity: u64,
    arena: BookkeepingArena,
    pending: Mutex<Vec<(Region, EventHandle)>>,
}

impl ArenaAllocator {
    pub fn new(kind: ArenaKind, dev: DeviceKind, capacity: u64) -> Self {
        Self {
            kind,
            dev,
            capacity,
            arena: BookkeepingArena::new(kind.id(), capacity),
            pending: Mutex::new(Vec::new()),
        }
    }

    pub fn kind(&self) -> ArenaKind {
        self.kind
    }

    pub fn capacity(&self) -> u64 {
        self.capacity
    }

    /// Immediately reusable bytes; excludes regions still waiting for their event.
    pub fn available(&self) -> u64 {
        self.arena.available()
    }

    pub fn pending(&self) -> usize {
        self.pending.lock().len()
    }

    pub fn is_live(&self, region: Region) -> bool {
        self.arena.is_live(region)
    }

    #[tracing::instrument(level = "trace", skip(self), fields(arena = ?self.kind))]
    pub fn alloc(&self, bytes: u64, align: u64) -> Result<Region, MemoryError> {
        self.reclaim();
        self.arena.alloc(self.dev, bytes, align).map_err(|e| match e {
            IndexError::ArenaExhausted { requested, available } => MemoryError::ArenaExhausted {
                arena: self.kind,
                requested: requested as u64,
                free: available as u64,
            },
            other => MemoryError::AllocFailed { arena: self.kind, reason: other.to_string() },
        })
    }

    pub fn free(&self, region: Region) -> Result<(), MemoryError> {
        if self.is_pending(region) {
            return Err(stale(region));
        }
        self.arena.try_free(region).map_err(|_| stale(region))
    }

    #[tracing::instrument(level = "trace", skip_all, fields(arena = ?self.kind, offset = region.offset, len = region.len, gen = region.gen))]
    pub fn release_after(&self, region: Region, event: EventHandle) -> Result<(), MemoryError> {
        if !self.arena.is_live(region) || self.is_pending(region) {
            return Err(stale(region));
        }
        self.pending.lock().push((region, event));
        self.reclaim();
        Ok(())
    }

    /// Frees every pending region whose event has completed; returns how many.
    pub fn reclaim(&self) -> usize {
        let done: Vec<Region> = {
            let mut pending = self.pending.lock();
            let (done, waiting): (Vec<_>, Vec<_>) =
                pending.drain(..).partition(|(_, e)| e.is_complete());
            *pending = waiting;
            done.into_iter().map(|(r, _)| r).collect()
        };
        for r in &done {
            if let Err(e) = self.arena.try_free(*r) {
                tracing::warn!(error = %e, "pending release found a stale region");
            }
        }
        done.len()
    }

    fn is_pending(&self, region: Region) -> bool {
        self.pending.lock().iter().any(|(r, _)| *r == region)
    }
}

fn stale(region: Region) -> MemoryError {
    MemoryError::StaleRegion { arena: region.arena.0, gen: region.gen }
}

impl MemoryManager for ArenaAllocator {
    fn alloc(&self, _dev: DeviceKind, bytes: u64, align: u64) -> Result<Region, IndexError> {
        ArenaAllocator::alloc(self, bytes, align).map_err(|e| match e {
            MemoryError::ArenaExhausted { requested, free, .. } => IndexError::ArenaExhausted {
                requested: requested as usize,
                available: free as usize,
            },
            other => IndexError::TransferFailed { reason: other.to_string() },
        })
    }

    fn free(&self, region: Region) {
        if let Err(e) = ArenaAllocator::free(self, region) {
            tracing::warn!(error = %e, "ignored stale free");
        }
    }
}

/// One allocator per arena class.
pub struct ArenaSet {
    arenas: Vec<ArenaAllocator>,
    sizes: ArenaSizes,
}

impl ArenaSet {
    pub fn new(dev: DeviceKind, sizes: ArenaSizes) -> Self {
        let arenas = ArenaKind::ALL
            .iter()
            .map(|k| ArenaAllocator::new(*k, dev, sizes.get(*k)))
            .collect();
        Self { arenas, sizes }
    }

    pub fn sizes(&self) -> &ArenaSizes {
        &self.sizes
    }

    pub fn get(&self, kind: ArenaKind) -> &ArenaAllocator {
        &self.arenas[kind.index()]
    }

    pub fn for_region(&self, region: Region) -> Result<&ArenaAllocator, MemoryError> {
        ArenaKind::from_id(region.arena).map(|k| self.get(k)).ok_or_else(|| stale(region))
    }
}

/// Arena-class aware allocation on one device.
pub trait DeviceMemory: Send + Sync {
    fn alloc_in(&self, kind: ArenaKind, bytes: u64, align: u64) -> Result<Region, MemoryError>;
    fn free(&self, region: Region) -> Result<(), MemoryError>;
    fn release_after(&self, region: Region, event: EventHandle) -> Result<(), MemoryError>;
    fn budget(&self) -> Result<Budget, MemoryError>;
}

#[cfg(test)]
pub(crate) mod testing {
    use std::sync::mpsc::{channel, Sender};

    use crate::resident::memory::{ArenaId, Region};
    use crate::resident::subengine::{CpuLane, EventHandle, ExecContext, LaneClass, LaneConfig};

    /// An event that stays incomplete until `open` is called.
    pub struct Gate {
        // `tx` must drop before `lane`: the lane's drop waits for the blocked job.
        tx: Sender<()>,
        lane: CpuLane,
        pub event: EventHandle,
    }

    impl Gate {
        pub fn closed() -> Self {
            let cfg = LaneConfig::new(Region { arena: ArenaId(0), offset: 0, len: 1, gen: 0 });
            let lane = CpuLane::new(LaneClass::Index, cfg).unwrap();
            let (tx, rx) = channel::<()>();
            lane.submit(Box::new(move || {
                let _ = rx.recv();
                Ok(())
            }))
            .unwrap();
            let event = lane.record_event().unwrap();
            Self { tx, lane, event }
        }

        pub fn open(self) {
            self.tx.send(()).unwrap();
            self.event.synchronize().unwrap();
            drop(self.lane);
        }
    }
}

#[cfg(test)]
#[path = "test/arena.rs"]
mod tests;
