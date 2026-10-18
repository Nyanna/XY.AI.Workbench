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
mod tests {
    use super::*;

    #[test]
    fn policy() {
        assert_eq!(compute_dtype(DeviceKind::Gpu), DType::F16);
        assert_eq!(compute_dtype(DeviceKind::Cpu), DType::F32);
    }

    #[test]
    fn realloc_bumps_gen_and_stale_is_rejected() {
        let a = BookkeepingArena::new(ArenaId(1), 64);
        let r1 = a.alloc(DeviceKind::Cpu, 16, 8).unwrap();
        a.free(r1);
        let r2 = a.alloc(DeviceKind::Cpu, 16, 8).unwrap();
        assert_eq!(r2.offset, r1.offset);
        assert_eq!(r2.gen, r1.gen + 1);
        assert!(a.try_free(r1).is_err());
        assert!(a.is_live(r2));
        assert!(a.try_free(r2).is_ok());
    }

    #[test]
    fn failpoint_injects_exhaustion_without_consuming_space() {
        let a = BookkeepingArena::new(ArenaId(9001), 64);
        let guard = failpoint::arm("memory.alloc", 9001, 1);
        assert!(matches!(a.alloc(DeviceKind::Cpu, 8, 1), Err(IndexError::ArenaExhausted { .. })));
        assert_eq!(a.available(), 64);
        assert!(a.alloc(DeviceKind::Cpu, 8, 1).is_ok());
        drop(guard);
    }

    proptest::proptest! {
        #[test]
        fn live_regions_never_overlap(ops in proptest::collection::vec((1u64..24, proptest::bool::ANY), 1..80)) {
            let a = BookkeepingArena::new(ArenaId(1), 256);
            let mut live: Vec<Region> = Vec::new();
            for (bytes, free_one) in ops {
                if free_one && !live.is_empty() {
                    let r = live.swap_remove(0);
                    a.free(r);
                } else if let Ok(r) = a.alloc(DeviceKind::Cpu, bytes, 1) {
                    for o in &live {
                        proptest::prop_assert!(r.offset + r.len <= o.offset || o.offset + o.len <= r.offset);
                    }
                    proptest::prop_assert!(r.offset + r.len <= 256);
                    live.push(r);
                }
            }
            let used: u64 = live.iter().map(|r| r.len).sum();
            proptest::prop_assert_eq!(a.available(), 256 - used);
        }
    }

    #[test]
    fn exhaustion() {
        let a = BookkeepingArena::new(ArenaId(0), 32);
        let _r = a.alloc(DeviceKind::Gpu, 24, 1).unwrap();
        assert!(matches!(
            a.alloc(DeviceKind::Gpu, 16, 1),
            Err(IndexError::ArenaExhausted { requested: 16, available: 8 })
        ));
    }
}
