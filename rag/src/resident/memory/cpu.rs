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
mod tests {
    use super::*;

    fn mgr() -> CpuMemoryManager {
        CpuMemoryManager::new(
            ArenaSizes { weights: 64, index_segment: 128, workspace: 64, staging: 0, param_block: 16 },
            2,
        )
    }

    #[test]
    fn write_read_roundtrip_and_stale_rejection() {
        let m = mgr();
        let r = m.alloc_in(ArenaKind::Workspace, 8, 8).unwrap();
        m.write(r, &[1, 2, 3, 4]).unwrap();
        let mut out = [0u8; 4];
        m.read(r, &mut out).unwrap();
        assert_eq!(out, [1, 2, 3, 4]);
        m.free(r).unwrap();
        assert!(m.read(r, &mut out).is_err());
        assert!(m.write(r, &[0]).is_err());
    }

    #[test]
    fn oversized_access_is_rejected() {
        let m = mgr();
        let r = m.alloc_in(ArenaKind::Workspace, 4, 4).unwrap();
        assert!(m.write(r, &[0; 5]).is_err());
    }

    #[test]
    fn copy_across_and_within_arenas() {
        let m = mgr();
        let a = m.alloc_in(ArenaKind::Weights, 4, 4).unwrap();
        let b = m.alloc_in(ArenaKind::IndexSegment, 4, 4).unwrap();
        let c = m.alloc_in(ArenaKind::Weights, 4, 4).unwrap();
        m.write(a, &[9, 8, 7, 6]).unwrap();
        m.copy(a, b).unwrap();
        m.copy(a, c).unwrap();
        let mut out = [0u8; 4];
        m.read(b, &mut out).unwrap();
        assert_eq!(out, [9, 8, 7, 6]);
        m.read(c, &mut out).unwrap();
        assert_eq!(out, [9, 8, 7, 6]);
    }

    #[test]
    fn budget_reserves_the_second_generation() {
        let m = mgr();
        let before = m.budget().unwrap();
        assert_eq!(before.total, 272);
        assert_eq!(before.reserved_for_generations, 64);
        m.alloc_in(ArenaKind::IndexSegment, 32, 8).unwrap();
        assert_eq!(m.budget().unwrap().free, 240);
    }
}
