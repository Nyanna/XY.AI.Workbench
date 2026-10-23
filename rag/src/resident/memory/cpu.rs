//! Host-RAM arenas for the CPU path; no streams, no staging.

use parking_lot::Mutex;

use super::arena::{ArenaSet, DeviceMemory};
use super::budget::{ArenaKind, ArenaSizes, Budget};
use super::error::MemoryError;
use super::manager::DeviceKind;
use super::region::Region;
use crate::resident::subengine::EventHandle;

pub struct CpuMemoryManager {
    set: ArenaSet,
    storage: Vec<Mutex<Vec<u8>>>,
    generations: u32,
}

impl CpuMemoryManager {
    pub fn new(sizes: ArenaSizes, generations: u32) -> Self {
        let storage = ArenaKind::ALL.iter().map(|k| Mutex::new(vec![0u8; sizes.get(*k) as usize])).collect();
        Self { set: ArenaSet::new(DeviceKind::Cpu, sizes), storage, generations }
    }

    pub fn arenas(&self) -> &ArenaSet {
        &self.set
    }

    pub fn write(&self, region: Region, data: &[u8]) -> Result<(), MemoryError> {
        let range = self.range(region, data.len())?;
        self.storage[self.slot(region)?].lock()[range].copy_from_slice(data);
        Ok(())
    }

    pub fn read(&self, region: Region, out: &mut [u8]) -> Result<(), MemoryError> {
        let range = self.range(region, out.len())?;
        out.copy_from_slice(&self.storage[self.slot(region)?].lock()[range]);
        Ok(())
    }

    pub fn copy(&self, src: Region, dst: Region) -> Result<(), MemoryError> {
        let len = src.len as usize;
        let from = self.range(src, len)?;
        let to = self.range(dst, len)?;
        let (si, di) = (self.slot(src)?, self.slot(dst)?);
        if si == di {
            self.storage[si].lock().copy_within(from, to.start);
        } else {
            let tmp = self.storage[si].lock()[from].to_vec();
            self.storage[di].lock()[to].copy_from_slice(&tmp);
        }
        Ok(())
    }

    fn slot(&self, region: Region) -> Result<usize, MemoryError> {
        let arena = self.set.for_region(region)?;
        Ok(arena.kind().index())
    }

    fn range(&self, region: Region, len: usize) -> Result<std::ops::Range<usize>, MemoryError> {
        let arena = self.set.for_region(region)?;
        if !arena.is_live(region) || len as u64 > region.len {
            return Err(MemoryError::StaleRegion { arena: region.arena.0, gen: region.gen });
        }
        let start = region.offset as usize;
        Ok(start..start + len)
    }
}

impl DeviceMemory for CpuMemoryManager {
    fn alloc_in(&self, kind: ArenaKind, bytes: u64, align: u64) -> Result<Region, MemoryError> {
        self.set.get(kind).alloc(bytes, align)
    }

    fn free(&self, region: Region) -> Result<(), MemoryError> {
        self.set.for_region(region)?.free(region)
    }

    fn release_after(&self, region: Region, event: EventHandle) -> Result<(), MemoryError> {
        self.set.for_region(region)?.release_after(region, event)
    }

    fn budget(&self) -> Result<Budget, MemoryError> {
        let sizes = self.set.sizes();
        let free = ArenaKind::ALL.iter().map(|k| self.set.get(*k).available()).sum();
        let total = ArenaKind::ALL.iter().map(|k| sizes.get(*k)).sum();
        Ok(Budget { free, total, reserved_for_generations: sizes.generation_reserve(self.generations) })
    }
}

#[cfg(test)]
#[path = "test/cpu.rs"]
mod tests;
