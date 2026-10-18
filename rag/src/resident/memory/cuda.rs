//! CUDA device arenas: one backing allocation per device-resident class, pinned staging, and
//! region-addressed copies. Device pointers never leave this module.
use std::ffi::c_void;
use std::sync::Arc;
use cudarc::driver::{
    result, sys, CudaContext, CudaSlice, CudaStream, DevicePtr, DevicePtrMut,
    DriverError, PinnedHostSlice,
};
use parking_lot::{Mutex, MutexGuard};
use super::arena::{ArenaSet, DeviceMemory};
use super::budget::{check_fit, ArenaKind, ArenaSizes, Budget, BudgetProbe};
use super::error::{MemoryError, TransferDir};
use super::manager::DeviceKind;
use super::region::Region;
use super::staging::StagingRing;
use crate::resident::subengine::{DeviceHandle, EventHandle};
pub type PinnedStaging = StagingRing<PinnedHostSlice<u8>>;
fn unavailable(e: DriverError) -> MemoryError {
    MemoryError::Unavailable {
        reason: e.to_string(),
    }
}
fn transfer(dir: TransferDir) -> impl Fn(DriverError) -> MemoryError {
    move |e| MemoryError::TransferFailed {
        dir,
        reason: e.to_string(),
    }
}
pub struct CudaProbe {
    ctx: Arc<CudaContext>,
}
impl BudgetProbe for CudaProbe {
    fn probe(&self) -> Result<(u64, u64), MemoryError> {
        let (free, total) = self
            .ctx
            .mem_get_info()
            .map_err(|e| MemoryError::BudgetProbe {
                reason: e.to_string(),
            })?;
        Ok((free as u64, total as u64))
    }
}
/// Keeps freed stream-ordered allocations in the default pool instead of returning them to the
/// driver, so steady-state operation does no driver-level allocation.
fn retain_pool_memory(ctx: &Arc<CudaContext>) -> Result<(), MemoryError> {
    ctx.bind_to_thread().map_err(unavailable)?;
    let mut threshold = u64::MAX;
    unsafe {
        let pool = result::device::get_default_mem_pool(ctx.cu_device())
            .map_err(unavailable)?;
        result::mem_pool::set_attribute(
                pool,
                sys::CUmemPool_attribute::CU_MEMPOOL_ATTR_RELEASE_THRESHOLD,
                &mut threshold as *mut u64 as *mut c_void,
            )
            .map_err(unavailable)
    }
}
pub struct CudaMemoryManager {
    ctx: Arc<CudaContext>,
    set: ArenaSet,
    backing: Vec<Mutex<Option<CudaSlice<u8>>>>,
    probe: CudaProbe,
    generations: u32,
}
impl CudaMemoryManager {
    /// Allocates the device arenas up front. `headroom` stays free for the driver context and
    /// other processes.
    pub fn new(
        device: &DeviceHandle,
        sizes: ArenaSizes,
        headroom: u64,
        generations: u32,
    ) -> Result<Self, MemoryError> {
        let ctx = device
            .cuda_context()
            .ok_or_else(|| MemoryError::Unavailable {
                reason: "not a CUDA device".into(),
            })?
            .clone();
        let probe = CudaProbe { ctx: ctx.clone() };
        let (free, total) = probe.probe()?;
        check_fit(&sizes, free, headroom)?;
        retain_pool_memory(&ctx)?;
        let stream = ctx.new_stream().map_err(unavailable)?;
        let mut backing = Vec::with_capacity(ArenaKind::ALL.len());
        for kind in ArenaKind::ALL {
            let bytes = if kind == ArenaKind::Staging {
                0
            } else {
                sizes.get(kind) as usize
            };
            let slice = (bytes > 0)
                .then(|| stream.alloc_zeros::<u8>(bytes))
                .transpose()
                .map_err(unavailable)?;
            backing.push(Mutex::new(slice));
        }
        stream.synchronize().map_err(unavailable)?;
        let staged_off = ArenaSizes { staging: 0, ..sizes };
        tracing::info!(
            free_mib = free >> 20, total_mib = total >> 20, device_mib = sizes
            .device_total() >> 20, "cuda arenas allocated"
        );
        Ok(Self {
            ctx,
            set: ArenaSet::new(DeviceKind::Gpu, staged_off),
            backing,
            probe,
            generations,
        })
    }
    pub fn arenas(&self) -> &ArenaSet {
        &self.set
    }
    /// Ring of page-locked host buffers for host-to-device uploads.
    pub fn staging_ring(
        &self,
        slots: usize,
        slot_bytes: usize,
    ) -> Result<PinnedStaging, MemoryError> {
        let mut payloads = Vec::with_capacity(slots);
        for _ in 0..slots {
            let mut pinned = unsafe { self.ctx.alloc_pinned::<u8>(slot_bytes) }
                .map_err(unavailable)?;
            pinned.as_mut_slice().map_err(unavailable)?.fill(0);
            payloads.push(pinned);
        }
        Ok(StagingRing::new(payloads))
    }
    /// Device-to-device copy between two live regions of different arena classes, ordered on
    /// `stream`. Used to move results between engine arenas and library-owned allocations.
    pub(crate) fn copy_d2d(
        &self,
        stream: &Arc<CudaStream>,
        src: Region,
        dst: Region,
    ) -> Result<(), MemoryError> {
        let (si, di) = (self.slot(src)?, self.slot(dst)?);
        let bytes = src.len;
        self.check_live(src, bytes)?;
        self.check_live(dst, bytes)?;
        if si == di {
            return Err(MemoryError::TransferFailed {
                dir: TransferDir::DeviceToDevice,
                reason: "source and destination share an arena".into(),
            });
        }
        let (first, second) = if si < di { (si, di) } else { (di, si) };
        let g1 = self.backing[first].lock();
        let g2 = self.backing[second].lock();
        let (sg, mut dg) = if si < di { (g1, g2) } else { (g2, g1) };
        let (Some(s), Some(d)) = (sg.as_ref(), dg.as_mut()) else {
            return Err(MemoryError::StaleRegion {
                arena: src.arena.0,
                gen: src.gen,
            });
        };
        let sv = s.slice(src.offset as usize..(src.offset + bytes) as usize);
        let mut dv = d.slice_mut(dst.offset as usize..(dst.offset + bytes) as usize);
        stream.memcpy_dtod(&sv, &mut dv).map_err(transfer(TransferDir::DeviceToDevice))
    }
    /// Asynchronous upload from pinned memory.
    ///
    /// # Safety
    /// `src` must be page-locked and stay untouched until an event recorded on `stream` after
    /// this call has completed.
    pub(crate) unsafe fn upload(
        &self,
        stream: &Arc<CudaStream>,
        src: &[u8],
        dst: Region,
    ) -> Result<(), MemoryError> {
        let slot = self.slot(dst)?;
        self.check_live(dst, src.len() as u64)?;
        let mut guard = self.backing[slot].lock();
        let slice = guard
            .as_mut()
            .ok_or(MemoryError::StaleRegion {
                arena: dst.arena.0,
                gen: dst.gen,
            })?;
        let mut view = slice
            .slice_mut(dst.offset as usize..dst.offset as usize + src.len());
        let (ptr, _sync) = view.device_ptr_mut(stream);
        result::memcpy_htod_async(ptr, src, stream.cu_stream())
            .map_err(transfer(TransferDir::HostToDevice))
    }
    /// Asynchronous download into pinned memory.
    ///
    /// # Safety
    /// `dst` must be page-locked and must not be read until an event recorded on `stream` after
    /// this call has completed.
    pub(crate) unsafe fn download(
        &self,
        stream: &Arc<CudaStream>,
        src: Region,
        dst: &mut [u8],
    ) -> Result<(), MemoryError> {
        let slot = self.slot(src)?;
        self.check_live(src, dst.len() as u64)?;
        let guard: MutexGuard<'_, Option<CudaSlice<u8>>> = self.backing[slot].lock();
        let slice = guard
            .as_ref()
            .ok_or(MemoryError::StaleRegion {
                arena: src.arena.0,
                gen: src.gen,
            })?;
        let view = slice.slice(src.offset as usize..src.offset as usize + dst.len());
        let (ptr, _sync) = view.device_ptr(stream);
        result::memcpy_dtoh_async(dst, ptr, stream.cu_stream())
            .map_err(transfer(TransferDir::DeviceToHost))
    }
    fn slot(&self, region: Region) -> Result<usize, MemoryError> {
        Ok(self.set.for_region(region)?.kind().index())
    }
    fn check_live(&self, region: Region, bytes: u64) -> Result<(), MemoryError> {
        if !self.set.for_region(region)?.is_live(region) || bytes > region.len {
            return Err(MemoryError::StaleRegion {
                arena: region.arena.0,
                gen: region.gen,
            });
        }
        Ok(())
    }
}
impl DeviceMemory for CudaMemoryManager {
    fn alloc_in(
        &self,
        kind: ArenaKind,
        bytes: u64,
        align: u64,
    ) -> Result<Region, MemoryError> {
        self.set.get(kind).alloc(bytes, align)
    }
    fn free(&self, region: Region) -> Result<(), MemoryError> {
        self.set.for_region(region)?.free(region)
    }
    fn release_after(
        &self,
        region: Region,
        event: EventHandle,
    ) -> Result<(), MemoryError> {
        self.set.for_region(region)?.release_after(region, event)
    }
    fn budget(&self) -> Result<Budget, MemoryError> {
        let (free, total) = self.probe.probe()?;
        let budget = Budget {
            free,
            total,
            reserved_for_generations: self
                .set
                .sizes()
                .generation_reserve(self.generations),
        };
        tracing::debug!(
            free, total, reserved = budget.reserved_for_generations, "vram budget"
        );
        Ok(budget)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::resident::subengine::{LaneClass, LaneConfig};
    fn manager() -> (DeviceHandle, CudaMemoryManager) {
        let dev = DeviceHandle::cuda(0).unwrap();
        let sizes = ArenaSizes {
            weights: 1 << 20,
            index_segment: 2 << 20,
            workspace: 1 << 20,
            staging: 0,
            param_block: 4096,
        };
        let m = CudaMemoryManager::new(&dev, sizes, 0, 2).unwrap();
        (dev, m)
    }
    #[test]
    #[ignore = "requires a CUDA device"]
    fn upload_copy_download_roundtrip() {
        let (dev, m) = manager();
        let scratch = m.alloc_in(ArenaKind::ParamBlock, 256, 8).unwrap();
        let lane = dev.open_lane(LaneClass::Index, LaneConfig::new(scratch)).unwrap();
        let ctx = dev.cuda_context().unwrap();
        let stream = ctx.new_stream().unwrap();
        let ring = m.staging_ring(2, 64).unwrap();
        let a = m.alloc_in(ArenaKind::Workspace, 64, 16).unwrap();
        let b = m.alloc_in(ArenaKind::IndexSegment, 64, 16).unwrap();
        let slot = ring.acquire().unwrap();
        ring.with_slot(
                slot,
                |p| p.as_mut_slice().unwrap()[..64].copy_from_slice(&[7u8; 64]),
            )
            .unwrap();
        let host: Vec<u8> = ring
            .with_slot(slot, |p| p.as_slice().unwrap()[..64].to_vec())
            .unwrap();
        unsafe { m.upload(&stream, &host, a).unwrap() };
        m.copy_d2d(&stream, a, b).unwrap();
        let mut out = vec![0u8; 64];
        unsafe { m.download(&stream, b, &mut out).unwrap() };
        stream.synchronize().unwrap();
        assert_eq!(out, vec![7u8; 64]);
        assert!(m.copy_d2d(& stream, a, a).is_err());
        assert!(m.budget().unwrap().total > 0);
        drop(lane);
    }
    #[test]
    fn oversized_plan_is_rejected_before_allocating() {
        let Ok(dev) = DeviceHandle::cuda(0) else { return };
        let sizes = ArenaSizes {
            weights: u64::MAX / 2,
            ..Default::default()
        };
        assert!(
            matches!(CudaMemoryManager::new(& dev, sizes, 0, 2),
            Err(MemoryError::BudgetExceeded { .. }))
        );
    }
}
