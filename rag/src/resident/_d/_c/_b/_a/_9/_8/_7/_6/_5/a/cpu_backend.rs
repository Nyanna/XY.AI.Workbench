//! CPU memory backend: every arena is one zeroed, contiguous RAM block with checked region access.
use std::alloc::{alloc_zeroed, dealloc, Layout};
use std::collections::HashMap;
use std::fmt;
use std::ptr::NonNull;
use std::sync::{Arc, Mutex};
use super::super::_4::a::allocator::AllocatorSet;
use super::super::_4::_3::a::bookkeeping::Bookkeeping;
use super::super::_4::_3::a::region_handle::RegionHandle;
use super::super::_4::_3::_2::a::arenas::{
    ArenaCapacities, ArenaKind, ArenaSet, Budget, MemInfo, MemoryQuery,
};
use super::super::_4::_3::_2::a::memory_access::{
    AccessError, AccessRegion, RegionBackend,
};
use super::super::_4::_3::_2::_1::device_lane_event::{DeviceKind, Event};
use super::memory_backend::{rounded_len, BackendError2, MemoryBackend, ALLOC_GRANULE};
const BUFFER_ALIGN: usize = ALLOC_GRANULE as usize;
/// Zeroed, granule-aligned heap block. Never exposes its address outside this module.
struct RawBuf {
    ptr: NonNull<u8>,
    len: usize,
}
unsafe impl Send for RawBuf {}
impl RawBuf {
    /// `None` if the memory cannot be obtained; never aborts.
    fn zeroed(len: usize) -> Option<RawBuf> {
        if len == 0 {
            return Some(RawBuf {
                ptr: NonNull::dangling(),
                len: 0,
            });
        }
        let layout = Layout::from_size_align(len, BUFFER_ALIGN).ok()?;
        let ptr = NonNull::new(unsafe { alloc_zeroed(layout) })?;
        Some(RawBuf { ptr, len })
    }
    fn as_slice(&self) -> &[u8] {
        if self.len == 0 {
            return &[];
        }
        unsafe { std::slice::from_raw_parts(self.ptr.as_ptr(), self.len) }
    }
    fn as_mut_slice(&mut self) -> &mut [u8] {
        if self.len == 0 {
            return &mut [];
        }
        unsafe { std::slice::from_raw_parts_mut(self.ptr.as_ptr(), self.len) }
    }
}
impl Drop for RawBuf {
    fn drop(&mut self) {
        if self.len == 0 {
            return;
        }
        if let Ok(layout) = Layout::from_size_align(self.len, BUFFER_ALIGN) {
            unsafe { dealloc(self.ptr.as_ptr(), layout) };
        }
    }
}
struct ArenaBuf {
    capacity: u64,
    data: Mutex<RawBuf>,
}
struct Inner5 {
    bookkeeping: Arc<Bookkeeping>,
    allocators: AllocatorSet,
    buffers: HashMap<ArenaKind, ArenaBuf>,
    generation_reserve: u64,
}
impl Inner5 {
    /// Returns the handle if it denotes a live region.
    fn resolve(&self, handle: &RegionHandle) -> Result<RegionHandle, BackendError2> {
        if !self.bookkeeping.is_live(handle) {
            return Err(BackendError2::StaleRegion);
        }
        Ok(*handle)
    }
    /// Validates a `[offset, offset + len)` access of `region` and returns its buffer and absolute start.
    fn locate(
        &self,
        region: &RegionHandle,
        offset: u64,
        len: u64,
    ) -> Result<(&ArenaBuf, usize), BackendError2> {
        if !self.bookkeeping.is_live(region) {
            return Err(BackendError2::StaleRegion);
        }
        let end = offset.checked_add(len).ok_or(BackendError2::StaleRegion)?;
        if end > region.len() {
            return Err(BackendError2::StaleRegion);
        }
        let buf = self.buffers.get(&region.arena()).ok_or(BackendError2::StaleRegion)?;
        let start = region
            .offset()
            .checked_add(offset)
            .ok_or(BackendError2::StaleRegion)?;
        let abs_end = start.checked_add(len).ok_or(BackendError2::StaleRegion)?;
        if abs_end > buf.capacity {
            return Err(BackendError2::StaleRegion);
        }
        let start = usize::try_from(start).map_err(|_| BackendError2::StaleRegion)?;
        Ok((buf, start))
    }
    fn write(
        &self,
        region: &RegionHandle,
        offset: u64,
        src: &[u8],
    ) -> Result<(), BackendError2> {
        let (buf, start) = self.locate(region, offset, src.len() as u64)?;
        if src.is_empty() {
            return Ok(());
        }
        let mut data = lock(&buf.data);
        data.as_mut_slice()[start..start + src.len()].copy_from_slice(src);
        Ok(())
    }
    fn read(
        &self,
        region: &RegionHandle,
        offset: u64,
        dst: &mut [u8],
    ) -> Result<(), BackendError2> {
        let (buf, start) = self.locate(region, offset, dst.len() as u64)?;
        if dst.is_empty() {
            return Ok(());
        }
        let data = lock(&buf.data);
        dst.copy_from_slice(&data.as_slice()[start..start + dst.len()]);
        Ok(())
    }
    fn copy(&self, src: &RegionHandle, dst: &RegionHandle) -> Result<(), BackendError2> {
        let (src_buf, src_start) = self.locate(src, 0, src.len())?;
        let (dst_buf, dst_start) = self.locate(dst, 0, src.len())?;
        let n = usize::try_from(src.len()).map_err(|_| BackendError2::StaleRegion)?;
        if n == 0 {
            return Ok(());
        }
        if src.arena() == dst.arena() {
            let mut data = lock(&dst_buf.data);
            data.as_mut_slice().copy_within(src_start..src_start + n, dst_start);
            return Ok(());
        }
        let mut staging: Vec<u8> = Vec::new();
        staging.try_reserve_exact(n).map_err(|_| BackendError2::AllocationFailed)?;
        {
            let data = lock(&src_buf.data);
            staging.extend_from_slice(&data.as_slice()[src_start..src_start + n]);
        }
        let mut data = lock(&dst_buf.data);
        data.as_mut_slice()[dst_start..dst_start + n].copy_from_slice(&staging);
        Ok(())
    }
    fn sums(&self) -> MemInfo {
        let mut free = 0u64;
        let mut total = 0u64;
        for (kind, buf) in &self.buffers {
            if let Some(a) = self.allocators.allocator(*kind) {
                free = free.saturating_add(a.available());
            }
            total = total.saturating_add(buf.capacity);
        }
        MemInfo { free, total }
    }
}
/// Thread-safe; `Send + Sync`.
pub struct CpuBackend {
    inner: Arc<Inner5>,
}
impl CpuBackend {
    /// Creates one zeroed RAM block per arena except `Staging`. Arenas with capacity 0 are empty.
    pub fn new(
        caps: ArenaCapacities,
        generations: u32,
    ) -> Result<CpuBackend, BackendError2> {
        let arenas = Arc::new(ArenaSet::new(caps));
        let unrestricted = Budget {
            free: u64::MAX,
            total: u64::MAX,
            generation_reserve: 0,
        };
        let mut buffers = HashMap::new();
        let mut specs = Vec::new();
        for kind in ArenaKind::ALL {
            if kind == ArenaKind::Staging {
                continue;
            }
            let arena = arenas
                .ensure(kind, &unrestricted)
                .map_err(|_| BackendError2::AllocationFailed)?;
            let exhausted = BackendError2::OutOfMemory {
                arena: kind,
                requested: arena.capacity,
                free: 0,
            };
            let len = usize::try_from(arena.capacity).map_err(|_| exhausted.clone())?;
            let raw = RawBuf::zeroed(len).ok_or(exhausted)?;
            buffers
                .insert(
                    kind,
                    ArenaBuf {
                        capacity: arena.capacity,
                        data: Mutex::new(raw),
                    },
                );
            specs.push((arena, DeviceKind::Cpu));
        }
        let bookkeeping = Arc::new(Bookkeeping::new(Arc::clone(&arenas)));
        let allocators = AllocatorSet::new(Arc::clone(&bookkeeping), &specs)
            .map_err(|_| BackendError2::AllocationFailed)?;
        Ok(CpuBackend {
            inner: Arc::new(Inner5 {
                bookkeeping,
                allocators,
                buffers,
                generation_reserve: caps.generation_reserve(generations),
            }),
        })
    }
    /// Uniform byte access to `region`; `StaleRegion` if the region is not live.
    pub fn access(&self, region: &RegionHandle) -> Result<AccessRegion, BackendError2> {
        let region = self.inner.resolve(region)?;
        let len = usize::try_from(region.len()).map_err(|_| BackendError2::StaleRegion)?;
        Ok(
            AccessRegion::new(
                Arc::new(CpuRegionAccess {
                    inner: Arc::clone(&self.inner),
                    region,
                    len,
                }),
            ),
        )
    }
    /// Writes `src` at `offset` inside `region`; any violated bound or liveness check is `StaleRegion`.
    pub fn write(
        &self,
        region: &RegionHandle,
        offset: u64,
        src: &[u8],
    ) -> Result<(), BackendError2> {
        let region = self.inner.resolve(region)?;
        self.inner.write(&region, offset, src)
    }
    /// Reads `dst.len()` bytes at `offset` inside `region`; any violated bound or liveness check is `StaleRegion`.
    pub fn read(
        &self,
        region: &RegionHandle,
        offset: u64,
        dst: &mut [u8],
    ) -> Result<(), BackendError2> {
        let region = self.inner.resolve(region)?;
        self.inner.read(&region, offset, dst)
    }
    /// Copies `src.len()` bytes from the start of `src` to the start of `dst` with memmove semantics.
    pub fn copy(
        &self,
        src: &RegionHandle,
        dst: &RegionHandle,
    ) -> Result<(), BackendError2> {
        let src = self.inner.resolve(src)?;
        let dst = self.inner.resolve(dst)?;
        self.inner.copy(&src, &dst)
    }
    /// Collects completed deferred frees in all arenas; returns how many regions were released.
    pub fn reap(&self) -> usize {
        self.inner
            .buffers
            .keys()
            .filter_map(|kind| self.inner.allocators.allocator(*kind))
            .map(|a| a.reap())
            .sum()
    }
}
impl fmt::Debug for CpuBackend {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let info = self.inner.sums();
        f.debug_struct("CpuBackend")
            .field("free", &info.free)
            .field("total", &info.total)
            .finish()
    }
}
impl MemoryBackend for CpuBackend {
    fn allocate(
        &self,
        arena: ArenaKind,
        size: u64,
        align: u64,
    ) -> Result<RegionHandle, BackendError2> {
        let len = rounded_len(size, align)?;
        let allocator = self
            .inner
            .allocators
            .allocator(arena)
            .ok_or(BackendError2::AllocationFailed)?;
        let region = allocator.allocate(len)?;
        debug_assert_eq!(region.offset() % align, 0);
        Ok(region)
    }
    fn free(&self, region: &RegionHandle) -> Result<(), BackendError2> {
        let region = self.inner.resolve(region)?;
        self.inner.allocators.allocator_for(&region)?.free(&region)?;
        Ok(())
    }
    fn free_deferred(
        &self,
        region: &RegionHandle,
        event: Event,
    ) -> Result<(), BackendError2> {
        let region = self.inner.resolve(region)?;
        self.inner.allocators.allocator_for(&region)?.free_deferred(&region, event)?;
        Ok(())
    }
    fn budget(&self) -> Result<Budget, BackendError2> {
        let info = self.inner.sums();
        Ok(Budget {
            free: info.free,
            total: info.total,
            generation_reserve: self.inner.generation_reserve,
        })
    }
    fn available(&self, arena: ArenaKind) -> Result<u64, BackendError2> {
        Ok(self.inner.allocators.allocator(arena).map_or(0, |a| a.available()))
    }
}
impl MemoryQuery for CpuBackend {
    fn query(&self) -> Result<MemInfo, String> {
        Ok(self.inner.sums())
    }
}
struct CpuRegionAccess {
    inner: Arc<Inner5>,
    region: RegionHandle,
    len: usize,
}
impl fmt::Debug for CpuRegionAccess {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CpuRegionAccess").field("region", &self.region).finish()
    }
}
impl RegionBackend for CpuRegionAccess {
    fn len(&self) -> usize {
        self.len
    }
    fn read_raw(&self, offset: usize, dst: &mut [u8]) -> Result<(), AccessError> {
        self.inner
            .read(&self.region, offset as u64, dst)
            .map_err(|e| AccessError::Backend(e.to_string()))
    }
    fn write_raw(&self, offset: usize, src: &[u8]) -> Result<(), AccessError> {
        self.inner
            .write(&self.region, offset as u64, src)
            .map_err(|e| AccessError::Backend(e.to_string()))
    }
}
#[cfg(test)]
#[path = "test/cpu_backend.rs"]
mod tests;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::_1::sync_util::lock;
