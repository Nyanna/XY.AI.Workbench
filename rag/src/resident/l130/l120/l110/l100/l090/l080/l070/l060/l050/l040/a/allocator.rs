//! Per-arena allocator with deferred free and the arena-to-allocator mapping.

use std::collections::HashMap;
use std::fmt;
use std::sync::{Arc, Mutex, MutexGuard};

use log::warn;

use super::super::l030::a::bookkeeping::{AllocError, Bookkeeping, Region};
use super::super::l030::l020::a::arenas::{Arena, ArenaKind};
use super::super::l030::l020::l010::device_lane_event::{DeviceKind, Event};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AllocatorError {
    /// Exhaustion. `free` is the immediately available amount (see `Allocator::available`).
    OutOfMemory {
        arena: ArenaKind,
        requested: u64,
        free: u64,
    },
    /// Any other allocation failure.
    AllocationFailed,
    /// Region invalid, foreign, not live or already pending.
    StaleRegion,
}

impl fmt::Display for AllocatorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AllocatorError::OutOfMemory {
                arena,
                requested,
                free,
            } => write!(
                f,
                "out of memory in arena {arena:?}: requested {requested}, free {free}"
            ),
            AllocatorError::AllocationFailed => f.write_str("Allokation fehlgeschlagen"),
            AllocatorError::StaleRegion => f.write_str("stale Region"),
        }
    }
}

impl std::error::Error for AllocatorError {}

struct Pending {
    region: Region,
    event: Event,
}

fn lock<T>(m: &Mutex<T>) -> MutexGuard<'_, T> {
    m.lock().unwrap_or_else(|poisoned| poisoned.into_inner())
}

/// Thread-safe; `Send + Sync`.
pub struct Allocator {
    arena: ArenaKind,
    capacity: u64,
    device: DeviceKind,
    bookkeeping: Arc<Bookkeeping>,
    pending: Mutex<Vec<Pending>>,
}

impl Allocator {
    pub fn new(arena: Arena, device: DeviceKind, bookkeeping: Arc<Bookkeeping>) -> Allocator {
        Allocator {
            arena: arena.kind,
            capacity: arena.capacity,
            device,
            bookkeeping,
            pending: Mutex::new(Vec::new()),
        }
    }

    pub fn arena(&self) -> ArenaKind {
        self.arena
    }

    pub fn capacity(&self) -> u64 {
        self.capacity
    }

    pub fn device_kind(&self) -> DeviceKind {
        self.device
    }

    /// Reaps completed deferred frees first, then allocates.
    pub fn allocate(&self, len: u64) -> Result<Region, AllocatorError> {
        self.reap();
        self.bookkeeping
            .allocate(self.arena, len)
            .map_err(|e| match e {
                AllocError::OutOfSpace {
                    requested, free, ..
                } => AllocatorError::OutOfMemory {
                    arena: self.arena,
                    requested,
                    free,
                },
                AllocError::ArenaNotCreated { .. } | AllocError::ZeroLength => {
                    AllocatorError::AllocationFailed
                }
            })
    }

    /// Immediate free; only for live, non-pending regions of this arena.
    pub fn free(&self, region: &Region) -> Result<(), AllocatorError> {
        let pending = lock(&self.pending);
        if region.arena != self.arena || pending.iter().any(|p| p.region == *region) {
            return Err(AllocatorError::StaleRegion);
        }
        self.bookkeeping
            .free(self.arena, region)
            .map_err(|_| AllocatorError::StaleRegion)
    }

    /// Marks `region` for release once `event` has completed, then reaps immediately.
    pub fn free_deferred(&self, region: &Region, event: Event) -> Result<(), AllocatorError> {
        let mut pending = lock(&self.pending);
        if region.arena != self.arena
            || !self.bookkeeping.is_live(region)
            || pending.iter().any(|p| p.region == *region)
        {
            return Err(AllocatorError::StaleRegion);
        }
        pending.push(Pending {
            region: *region,
            event,
        });
        self.reap_locked(&mut pending);
        Ok(())
    }

    /// Frees all pending regions whose event has completed; returns how many were freed.
    pub fn reap(&self) -> usize {
        let mut pending = lock(&self.pending);
        self.reap_locked(&mut pending)
    }

    fn reap_locked(&self, pending: &mut Vec<Pending>) -> usize {
        if pending.is_empty() {
            return 0;
        }
        let mut freed = 0;
        pending.retain(|p| match p.event.is_complete() {
            Ok(true) => {
                match self.bookkeeping.free(self.arena, &p.region) {
                    Ok(()) => freed += 1,
                    Err(e) => warn!(
                        "deferred free of invalid region arena={:?} offset={} len={} generation={}: {e}",
                        p.region.arena, p.region.offset, p.region.len, p.region.generation
                    ),
                }
                false
            }
            Ok(false) => true,
            Err(e) => {
                warn!(
                    "event query failed for pending region arena={:?} offset={} len={} generation={}: {e}",
                    p.region.arena, p.region.offset, p.region.len, p.region.generation
                );
                true
            }
        });
        freed
    }

    /// Bytes available right now; pending regions are still live and do not count.
    pub fn available(&self) -> u64 {
        self.capacity
            .saturating_sub(self.bookkeeping.live_bytes(self.arena))
    }

    pub fn pending_count(&self) -> usize {
        lock(&self.pending).len()
    }

    pub fn is_live(&self, region: &Region) -> bool {
        self.bookkeeping.is_live(region)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AllocatorSetError {
    DuplicateArena(ArenaKind),
}

impl fmt::Display for AllocatorSetError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AllocatorSetError::DuplicateArena(kind) => {
                write!(f, "duplicate arena in allocator set: {kind:?}")
            }
        }
    }
}

impl std::error::Error for AllocatorSetError {}

/// Thread-safe; `Send + Sync`. Immutable after construction.
pub struct AllocatorSet {
    allocators: HashMap<ArenaKind, Arc<Allocator>>,
}

impl AllocatorSet {
    pub fn new(
        bookkeeping: Arc<Bookkeeping>,
        arenas: &[(Arena, DeviceKind)],
    ) -> Result<AllocatorSet, AllocatorSetError> {
        let mut allocators = HashMap::with_capacity(arenas.len());
        for (arena, device) in arenas {
            if allocators.contains_key(&arena.kind) {
                return Err(AllocatorSetError::DuplicateArena(arena.kind));
            }
            allocators.insert(
                arena.kind,
                Arc::new(Allocator::new(*arena, *device, Arc::clone(&bookkeeping))),
            );
        }
        Ok(AllocatorSet { allocators })
    }

    pub fn allocator(&self, arena: ArenaKind) -> Option<&Allocator> {
        self.allocators.get(&arena).map(|a| a.as_ref())
    }

    /// Allocator of `region.arena`; liveness of the region is not checked.
    pub fn allocator_for(&self, region: &Region) -> Result<&Allocator, AllocatorError> {
        self.allocator(region.arena)
            .ok_or(AllocatorError::StaleRegion)
    }

    pub fn size_hint(&self, arena: ArenaKind) -> Option<u64> {
        self.allocator(arena).map(Allocator::capacity)
    }

    pub fn len(&self) -> usize {
        self.allocators.len()
    }

    pub fn is_empty(&self) -> bool {
        self.allocators.is_empty()
    }
}

static_assertions::assert_impl_all!(Allocator: Send, Sync);
static_assertions::assert_impl_all!(AllocatorSet: Send, Sync);

#[cfg(test)]
#[path = "test/allocator.rs"]
mod tests;
