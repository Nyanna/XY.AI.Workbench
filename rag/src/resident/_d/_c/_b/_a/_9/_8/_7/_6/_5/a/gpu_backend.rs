//! GPU memory backend: contiguous zeroed VRAM arenas, one mapped host block and asynchronous transfers on a dedicated copy stream.
use std::ffi::c_void;
use std::fmt;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::ptr::{self, NonNull};
use std::sync::{Arc, Mutex, RwLock, RwLockReadGuard, RwLockWriteGuard};
use candle_core::cuda_backend::CudaDevice;
use cudarc::driver::result as drv;
use cudarc::driver::{sys, CudaContext};
use log::{debug, info, warn};
use super::super::_4::a::allocator::AllocatorSet;
use super::super::_4::a::library_adapter::{wrap_region, DeviceBlock, LibTensor};
use super::super::_4::_3::a::bookkeeping::Bookkeeping;
use super::super::_4::_3::a::region_handle::RegionHandle;
use super::super::_4::_3::_2::a::arenas::{
    fit_check, Arena, ArenaCapacities, ArenaKind, ArenaSet, Budget, MemInfo, MemoryQuery,
    MIB,
};
use super::super::_4::_3::_2::a::memory_access::AccessError;
use super::super::_4::_3::_2::_1::device_lane_event::{
    DeviceHandle, DeviceKind, Event, GpuEventBackend, Lane, LaneConfig, LaneKind,
    StreamBackend,
};
use super::super::_4::_3::_2::_1::layout::Layout;
use super::memory_backend::{rounded_len, BackendError2, MemoryBackend, TransferDirection};
const SLOTS: usize = ArenaKind::ALL.len();
fn read2<T>(l: &RwLock<T>) -> RwLockReadGuard<'_, T> {
    l.read().unwrap_or_else(|poisoned| poisoned.into_inner())
}
fn write2<T>(l: &RwLock<T>) -> RwLockWriteGuard<'_, T> {
    l.write().unwrap_or_else(|poisoned| poisoned.into_inner())
}
fn text2(e: drv::DriverError) -> String {
    e.to_string()
}
fn unavailable2(reason: impl Into<String>) -> BackendError2 {
    BackendError2::DeviceMemoryUnavailable(reason.into())
}
fn slot_of(kind: ArenaKind) -> usize {
    kind.id() as usize
}
/// Pinned, zeroed host buffer.
pub struct PinnedBuffer {
    ptr: NonNull<u8>,
    len: usize,
    ctx: Option<Arc<CudaContext>>,
}
unsafe impl Send for PinnedBuffer {}
unsafe impl Sync for PinnedBuffer {}
impl PinnedBuffer {
    fn empty() -> PinnedBuffer {
        PinnedBuffer {
            ptr: NonNull::dangling(),
            len: 0,
            ctx: None,
        }
    }
    fn allocate(ctx: &Arc<CudaContext>, len: usize) -> Result<PinnedBuffer, String> {
        if len == 0 {
            return Ok(PinnedBuffer::empty());
        }
        ctx.bind_to_thread().map_err(text2)?;
        let raw = unsafe { drv::malloc_host(len, sys::CU_MEMHOSTALLOC_PORTABLE) }
            .map_err(text2)?;
        let ptr = NonNull::new(raw.cast::<u8>())
            .ok_or_else(|| "null host allocation".to_string())?;
        unsafe { ptr::write_bytes(ptr.as_ptr(), 0, len) };
        Ok(PinnedBuffer {
            ptr,
            len,
            ctx: Some(Arc::clone(ctx)),
        })
    }
    pub fn len(&self) -> usize {
        self.len
    }
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
    pub fn as_slice(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.ptr.as_ptr(), self.len) }
    }
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        unsafe { std::slice::from_raw_parts_mut(self.ptr.as_ptr(), self.len) }
    }
}
impl Drop for PinnedBuffer {
    fn drop(&mut self) {
        if self.len == 0 {
            return;
        }
        if let Some(ctx) = &self.ctx {
            if ctx.bind_to_thread().is_ok() {
                let _ = unsafe { drv::free_host(self.ptr.as_ptr().cast::<c_void>()) };
            }
        }
    }
}
impl fmt::Debug for PinnedBuffer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PinnedBuffer").field("len", &self.len).finish()
    }
}
/// Host-side source of an upload.
#[derive(Clone, Copy, Debug)]
pub enum HostSource<'a> {
    Pageable(&'a [u8]),
    Pinned(&'a PinnedBuffer),
}
impl HostSource<'_> {
    fn as_slice(&self) -> &[u8] {
        match self {
            HostSource::Pageable(s) => s,
            HostSource::Pinned(p) => p.as_slice(),
        }
    }
}
struct StreamOwner2 {
    ctx: Arc<CudaContext>,
    raw: sys::CUstream,
}
unsafe impl Send for StreamOwner2 {}
unsafe impl Sync for StreamOwner2 {}
impl fmt::Debug for StreamOwner2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "StreamOwner({:p})", self.raw)
    }
}
impl StreamOwner2 {
    fn create(ctx: &Arc<CudaContext>) -> Result<Arc<StreamOwner2>, String> {
        ctx.bind_to_thread().map_err(text2)?;
        let raw = drv::stream::create(drv::stream::StreamKind::NonBlocking)
            .map_err(text2)?;
        Ok(
            Arc::new(StreamOwner2 {
                ctx: Arc::clone(ctx),
                raw,
            }),
        )
    }
    fn synchronize(&self) -> Result<(), String> {
        self.ctx.bind_to_thread().map_err(text2)?;
        unsafe { drv::stream::synchronize(self.raw) }.map_err(text2)
    }
}
impl Drop for StreamOwner2 {
    fn drop(&mut self) {
        if self.ctx.bind_to_thread().is_ok() {
            let _ = unsafe { drv::stream::destroy(self.raw) };
        }
    }
}
#[derive(Debug)]
struct CopyStreamBackend {
    owner: Arc<StreamOwner2>,
}
impl StreamBackend for CopyStreamBackend {
    fn record_event(&self) -> Result<Box<dyn GpuEventBackend>, String> {
        let ctx = &self.owner.ctx;
        ctx.bind_to_thread().map_err(text2)?;
        let raw = drv::event::create(sys::CUevent_flags::CU_EVENT_DISABLE_TIMING)
            .map_err(text2)?;
        let event = CuEvent2 {
            ctx: Arc::clone(ctx),
            raw,
        };
        unsafe { drv::event::record(event.raw, self.owner.raw) }.map_err(text2)?;
        Ok(Box::new(event))
    }
}
#[derive(Debug)]
struct CuEvent2 {
    ctx: Arc<CudaContext>,
    raw: sys::CUevent,
}
unsafe impl Send for CuEvent2 {}
unsafe impl Sync for CuEvent2 {}
impl GpuEventBackend for CuEvent2 {
    fn query(&self) -> Result<bool, String> {
        self.ctx.bind_to_thread().map_err(text2)?;
        match unsafe { drv::event::query(self.raw) } {
            Ok(()) => Ok(true),
            Err(e) if e.0 == sys::CUresult::CUDA_ERROR_NOT_READY => Ok(false),
            Err(e) => Err(text2(e)),
        }
    }
    fn synchronize(&self) -> Result<(), String> {
        self.ctx.bind_to_thread().map_err(text2)?;
        unsafe { drv::event::synchronize(self.raw) }.map_err(text2)
    }
}
impl Drop for CuEvent2 {
    fn drop(&mut self) {
        if self.ctx.bind_to_thread().is_ok() {
            let _ = unsafe { drv::event::destroy(self.raw) };
        }
    }
}
struct DriverQuery2 {
    ctx: Arc<CudaContext>,
}
impl MemoryQuery for DriverQuery2 {
    fn query(&self) -> Result<MemInfo, String> {
        self.ctx.bind_to_thread().map_err(text2)?;
        let (free, total) = drv::mem_get_info().map_err(text2)?;
        Ok(MemInfo {
            free: free as u64,
            total: total as u64,
        })
    }
}
/// One arena block; `host` is null for device blocks.
struct Block {
    dev: u64,
    host: *mut u8,
    len: u64,
}
unsafe impl Send for Block {}
unsafe impl Sync for Block {}
fn alloc_device(stream: sys::CUstream, capacity: u64) -> Result<Block, String> {
    let len = usize::try_from(capacity)
        .map_err(|_| "arena exceeds address space".to_string())?;
    let dev = unsafe { drv::malloc_async(stream, len) }.map_err(text2)?;
    if let Err(e) = unsafe { drv::memset_d8_async(dev, 0, len, stream) } {
        let _ = unsafe { drv::free_async(dev, stream) };
        return Err(text2(e));
    }
    Ok(Block {
        dev,
        host: ptr::null_mut(),
        len: capacity,
    })
}
fn alloc_mapped(capacity: u64) -> Result<Block, String> {
    let len = usize::try_from(capacity)
        .map_err(|_| "arena exceeds address space".to_string())?;
    let raw = unsafe { drv::malloc_host(len, sys::CU_MEMHOSTALLOC_DEVICEMAP) }
        .map_err(text2)?;
    let mut dev: sys::CUdeviceptr = 0;
    if let Err(e) = unsafe { sys::cuMemHostGetDevicePointer_v2(&mut dev, raw, 0) }
        .result()
    {
        let _ = unsafe { drv::free_host(raw) };
        return Err(text2(e));
    }
    let host = raw.cast::<u8>();
    unsafe { ptr::write_bytes(host, 0, len) };
    Ok(Block { dev, host, len: capacity })
}
fn release_block(stream: sys::CUstream, block: &Block) -> Result<(), String> {
    if block.host.is_null() {
        unsafe { drv::free_async(block.dev, stream) }.map_err(text2)
    } else {
        unsafe { drv::free_host(block.host.cast::<c_void>()) }.map_err(text2)
    }
}
fn fill_blocks(
    stream: sys::CUstream,
    arenas: &[Arena],
    made: &mut Vec<(usize, Block)>,
) -> Result<(), String> {
    for arena in arenas {
        if arena.capacity == 0 {
            continue;
        }
        let block = if arena.kind.is_device() {
            alloc_device(stream, arena.capacity)?
        } else if arena.kind == ArenaKind::MappedHost {
            alloc_mapped(arena.capacity)?
        } else {
            continue;
        };
        made.push((slot_of(arena.kind), block));
    }
    Ok(())
}
fn finish_setup(ctx: &Arc<CudaContext>, owner: &StreamOwner2) -> Result<(), String> {
    let pool = unsafe { drv::device::get_default_mem_pool(ctx.cu_device()) }
        .map_err(text2)?;
    let mut threshold = u64::MAX;
    unsafe {
        drv::mem_pool::set_attribute(
            pool,
            sys::CUmemPool_attribute::CU_MEMPOOL_ATTR_RELEASE_THRESHOLD,
            (&mut threshold as *mut u64).cast::<c_void>(),
        )
    }
        .map_err(text2)?;
    owner.synchronize()?;
    ctx.synchronize().map_err(text2)
}
fn rollback(ctx: &Arc<CudaContext>, owner: &StreamOwner2, made: &[(usize, Block)]) {
    if ctx.bind_to_thread().is_err() {
        return;
    }
    let _ = owner.synchronize();
    for (_, block) in made {
        let _ = release_block(owner.raw, block);
    }
    let _ = owner.synchronize();
}
fn setup_blocks(
    ctx: &Arc<CudaContext>,
    owner: &StreamOwner2,
    arenas: &[Arena],
) -> Result<Vec<(usize, Block)>, String> {
    ctx.bind_to_thread().map_err(text2)?;
    let mut made = Vec::new();
    let outcome = fill_blocks(owner.raw, arenas, &mut made)
        .and_then(|()| finish_setup(ctx, owner));
    match outcome {
        Ok(()) => Ok(made),
        Err(e) => {
            rollback(ctx, owner, &made);
            Err(e)
        }
    }
}
fn open_context(ordinal: u32) -> Result<Arc<CudaContext>, BackendError2> {
    match catch_unwind(AssertUnwindSafe(|| CudaContext::new(ordinal as usize))) {
        Ok(Ok(ctx)) => Ok(ctx),
        Ok(Err(e)) => Err(unavailable2(text2(e))),
        Err(_) => Err(unavailable2("CUDA driver could not be loaded")),
    }
}
struct CopyStream {
    owner: Arc<StreamOwner2>,
    lane: Lane,
    gate: Mutex<()>,
}
fn mib(bytes: u64) -> u64 {
    bytes / MIB
}
/// Memory backend for one Nvidia device. `Send + Sync`.
pub struct GpuBackend2 {
    device: DeviceHandle,
    ordinal: usize,
    ctx: Arc<CudaContext>,
    query: DriverQuery2,
    arenas: Arc<ArenaSet>,
    allocators: AllocatorSet,
    blocks: [RwLock<Option<Block>>; SLOTS],
    copy: CopyStream,
    candle: Mutex<Option<CudaDevice>>,
}
impl GpuBackend2 {
    pub fn new(
        device: DeviceHandle,
        caps: ArenaCapacities,
        headroom: u64,
    ) -> Result<GpuBackend2, BackendError2> {
        let ordinal = match device {
            DeviceHandle::Gpu(ordinal) => ordinal,
            _ => return Err(unavailable2("device is not a GPU")),
        };
        let ctx = open_context(ordinal)?;
        let query = DriverQuery2 {
            ctx: Arc::clone(&ctx),
        };
        let budget = Budget::from_query(&query, 0)?;
        fit_check(&caps, budget.free, headroom)?;
        let arenas = Arc::new(ArenaSet::new(caps));
        arenas.set_capacity(ArenaKind::Staging, 0);
        let mut created = Vec::with_capacity(SLOTS);
        for kind in ArenaKind::ALL {
            created.push(arenas.ensure(kind, &budget)?);
        }
        let bookkeeping = Arc::new(Bookkeeping::new(Arc::clone(&arenas)));
        let specs: Vec<(Arena, DeviceKind)> = created
            .iter()
            .map(|a| (*a, DeviceKind::Gpu))
            .collect();
        let allocators = AllocatorSet::new(bookkeeping, &specs)
            .map_err(|_| BackendError2::AllocationFailed)?;
        let owner = StreamOwner2::create(&ctx).map_err(unavailable2)?;
        let built = setup_blocks(&ctx, &owner, &created).map_err(unavailable2)?;
        let mut by_slot: [Option<Block>; SLOTS] = std::array::from_fn(|_| None);
        for (slot, block) in built {
            by_slot[slot] = Some(block);
        }
        let lane = Lane::new(
            LaneConfig::preset(LaneKind::CopyH2D, device),
            Some(
                Box::new(CopyStreamBackend {
                    owner: Arc::clone(&owner),
                }),
            ),
        );
        info!(
            "gpu backend {}: free {} MiB, total {} MiB, weights {} MiB, index {} MiB, workspace {} MiB, staging {} MiB, parameters {} MiB, mapped host {} MiB",
            ordinal, mib(budget.free), mib(budget.total),
            mib(created[slot_of(ArenaKind::Weights)].capacity),
            mib(created[slot_of(ArenaKind::IndexSegments)].capacity),
            mib(created[slot_of(ArenaKind::Workspace)].capacity),
            mib(created[slot_of(ArenaKind::Staging)].capacity),
            mib(created[slot_of(ArenaKind::ParameterBlock)].capacity),
            mib(created[slot_of(ArenaKind::MappedHost)].capacity),
        );
        Ok(GpuBackend2 {
            device,
            ordinal: ordinal as usize,
            ctx,
            query,
            arenas,
            allocators,
            blocks: by_slot.map(RwLock::new),
            copy: CopyStream {
                owner,
                lane,
                gate: Mutex::new(()),
            },
            candle: Mutex::new(None),
        })
    }
    pub fn device(&self) -> DeviceHandle {
        self.device
    }
    pub fn capacities(&self) -> ArenaCapacities {
        self.arenas.capacities()
    }
    /// Validates liveness and bounds of `handle` against the arena block in `slot`.
    fn resolve<'a>(
        &self,
        slot: &'a Option<Block>,
        handle: &RegionHandle,
    ) -> Result<(RegionHandle, &'a Block, u64), BackendError2> {
        let allocator = self
            .allocators
            .allocator_for(handle)
            .map_err(|_| BackendError2::StaleRegion)?;
        if !allocator.is_live(handle) {
            return Err(BackendError2::StaleRegion);
        }
        let block = slot.as_ref().ok_or(BackendError2::StaleRegion)?;
        let end = handle
            .offset()
            .checked_add(handle.len())
            .ok_or(BackendError2::StaleRegion)?;
        if end > block.len {
            return Err(BackendError2::StaleRegion);
        }
        let addr = block
            .dev
            .checked_add(handle.offset())
            .ok_or(BackendError2::StaleRegion)?;
        Ok((*handle, block, addr))
    }
    /// Enqueues `op` on the copy stream and records the covering event under one lock.
    fn enqueue<F>(
        &self,
        direction: TransferDirection,
        active: bool,
        op: F,
    ) -> Result<Event, BackendError2>
    where
        F: FnOnce(sys::CUstream) -> Result<(), drv::DriverError>,
    {
        let fail = |reason: String| BackendError2::Transfer {
            direction,
            reason,
        };
        let _gate = lock(&self.copy.gate);
        self.ctx.bind_to_thread().map_err(|e| fail(text2(e)))?;
        if active {
            op(self.copy.owner.raw).map_err(|e| fail(text2(e)))?;
        }
        self.copy.lane.record_event().map_err(|e| fail(e.to_string()))
    }
    pub fn copy_d2d(
        &self,
        src: &RegionHandle,
        dst: &RegionHandle,
        len: u64,
    ) -> Result<Event, BackendError2> {
        let direction = TransferDirection::D2D;
        if src.arena() == dst.arena() {
            return Err(BackendError2::Transfer {
                direction,
                reason: "same arena".to_string(),
            });
        }
        let (src_slot, dst_slot) = (slot_of(src.arena()), slot_of(dst.arena()));
        let (src_guard, dst_guard);
        if src_slot < dst_slot {
            src_guard = read2(&self.blocks[src_slot]);
            dst_guard = read2(&self.blocks[dst_slot]);
        } else {
            dst_guard = read2(&self.blocks[dst_slot]);
            src_guard = read2(&self.blocks[src_slot]);
        }
        let (_, _, src_addr) = self.resolve(&src_guard, src)?;
        let (_, _, dst_addr) = self.resolve(&dst_guard, dst)?;
        if len > src.len() || len > dst.len() {
            return Err(BackendError2::Transfer {
                direction,
                reason: "size mismatch".to_string(),
            });
        }
        self.enqueue(
            direction,
            len > 0,
            |stream| unsafe {
                drv::memcpy_dtod_async(dst_addr, src_addr, len as usize, stream)
            },
        )
    }
    pub fn upload(
        &self,
        dst: &RegionHandle,
        src: HostSource<'_>,
    ) -> Result<Event, BackendError2> {
        let direction = TransferDirection::H2D;
        let guard = read2(&self.blocks[slot_of(dst.arena())]);
        let (_, _, addr) = self.resolve(&guard, dst)?;
        let data = src.as_slice();
        if data.len() as u64 > dst.len() {
            return Err(BackendError2::Transfer {
                direction,
                reason: "size mismatch".to_string(),
            });
        }
        self.enqueue(
            direction,
            !data.is_empty(),
            |stream| unsafe { drv::memcpy_htod_async(addr, data, stream) },
        )
    }
    pub fn download(
        &self,
        src: &RegionHandle,
        dst: &PinnedBuffer,
    ) -> Result<Event, BackendError2> {
        let direction = TransferDirection::D2H;
        let guard = read2(&self.blocks[slot_of(src.arena())]);
        let (_, _, addr) = self.resolve(&guard, src)?;
        if dst.len() as u64 > src.len() {
            return Err(BackendError2::Transfer {
                direction,
                reason: "size mismatch".to_string(),
            });
        }
        let (host, len) = (dst.ptr.as_ptr().cast::<c_void>(), dst.len());
        self.enqueue(
            direction,
            len > 0,
            |stream| unsafe {
                sys::cuMemcpyDtoHAsync_v2(host, addr, len, stream).result()
            },
        )
    }
    pub fn allocate_pinned(&self, len: usize) -> Result<PinnedBuffer, BackendError2> {
        PinnedBuffer::allocate(&self.ctx, len).map_err(unavailable2)
    }
    /// Locks the mapped block and validates an access; returns the guard and the host address.
    fn mapped_span(
        &self,
        region: &RegionHandle,
        offset: usize,
        len: usize,
    ) -> Result<(RwLockReadGuard<'_, Option<Block>>, *mut u8), BackendError2> {
        if region.arena() != ArenaKind::MappedHost {
            return Err(BackendError2::ArenaMismatch {
                expected: ArenaKind::MappedHost,
                found: region.arena(),
            });
        }
        let guard = read2(&self.blocks[slot_of(ArenaKind::MappedHost)]);
        let ptr = {
            let (_, block, _) = self.resolve(&guard, region)?;
            let region_len = usize::try_from(region.len()).unwrap_or(usize::MAX);
            let out_of_bounds = || {
                BackendError2::Access(AccessError::OutOfBounds {
                    offset,
                    len,
                    region_len,
                })
            };
            let rel = u64::try_from(offset).map_err(|_| out_of_bounds())?;
            let count = u64::try_from(len).map_err(|_| out_of_bounds())?;
            let rel_end = rel.checked_add(count).ok_or_else(out_of_bounds)?;
            if rel_end > region.len() {
                return Err(out_of_bounds());
            }
            let start = region.offset().checked_add(rel).ok_or_else(out_of_bounds)?;
            let end = start.checked_add(count).ok_or_else(out_of_bounds)?;
            if end > block.len || block.host.is_null() {
                return Err(out_of_bounds());
            }
            let start = usize::try_from(start).map_err(|_| out_of_bounds())?;
            unsafe { block.host.add(start) }
        };
        Ok((guard, ptr))
    }
    pub fn mapped_read(
        &self,
        region: &RegionHandle,
        offset: usize,
        dst: &mut [u8],
    ) -> Result<(), BackendError2> {
        let (_guard, src) = self.mapped_span(region, offset, dst.len())?;
        if !dst.is_empty() {
            unsafe { ptr::copy_nonoverlapping(src, dst.as_mut_ptr(), dst.len()) };
        }
        Ok(())
    }
    pub fn mapped_write(
        &self,
        region: &RegionHandle,
        offset: usize,
        src: &[u8],
    ) -> Result<(), BackendError2> {
        let (_guard, dst) = self.mapped_span(region, offset, src.len())?;
        if !src.is_empty() {
            unsafe { ptr::copy_nonoverlapping(src.as_ptr(), dst, src.len()) };
        }
        Ok(())
    }
    /// Device address of the start of the mapped host block.
    pub fn mapped_device_addr(&self) -> Option<u64> {
        read2(&self.blocks[slot_of(ArenaKind::MappedHost)]).as_ref().map(|b| b.dev)
    }
    fn candle_device(&self) -> Result<CudaDevice, BackendError2> {
        let mut cached = lock(&self.candle);
        if let Some(device) = cached.as_ref() {
            return Ok(device.clone());
        }
        let ordinal = self.ordinal;
        let device = match catch_unwind(
            AssertUnwindSafe(|| CudaDevice::new_with_stream(ordinal)),
        ) {
            Ok(Ok(device)) => device,
            Ok(Err(e)) => return Err(unavailable2(e.to_string())),
            Err(_) => return Err(unavailable2("library device could not be created")),
        };
        *cached = Some(device.clone());
        Ok(device)
    }
    /// Wraps a live region as a library tensor.
    ///
    /// # Safety
    /// The tensor and all its views must be gone before `shutdown` or drop of the backend.
    pub unsafe fn wrap_tensor(
        &self,
        region: &RegionHandle,
        layout: &Layout,
    ) -> Result<LibTensor, BackendError2> {
        let guard = read2(&self.blocks[slot_of(region.arena())]);
        let (base_addr, len_bytes) = {
            let (_, block, _) = self.resolve(&guard, region)?;
            (block.dev, block.len)
        };
        let block = DeviceBlock {
            device: self.candle_device()?,
            base_addr,
            len_bytes,
        };
        Ok(wrap_region(&block, region, layout)?)
    }
    /// Idempotent; also called by `Drop`.
    pub fn shutdown(&self) {
        let mut guards: Vec<RwLockWriteGuard<'_, Option<Block>>> = self
            .blocks
            .iter()
            .map(write2)
            .collect();
        let _gate = lock(&self.copy.gate);
        let blocks: Vec<Block> = guards.iter_mut().filter_map(|g| g.take()).collect();
        if blocks.is_empty() {
            return;
        }
        if self.ctx.bind_to_thread().is_err() {
            warn!(
                "gpu backend {}: context not bindable, blocks are leaked", self.ordinal
            );
            return;
        }
        if let Err(e) = self.copy.owner.synchronize() {
            warn!("gpu backend {}: copy stream synchronize failed: {e}", self.ordinal);
        }
        for block in &blocks {
            if let Err(e) = release_block(self.copy.owner.raw, block) {
                warn!("gpu backend {}: block release failed: {e}", self.ordinal);
            }
        }
        if let Err(e) = self.copy.owner.synchronize() {
            warn!("gpu backend {}: copy stream synchronize failed: {e}", self.ordinal);
        }
        info!("gpu backend {}: shut down", self.ordinal);
    }
}
impl fmt::Debug for GpuBackend2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("GpuBackend")
            .field("device", &self.device)
            .field("capacities", &self.capacities())
            .finish()
    }
}
impl Drop for GpuBackend2 {
    fn drop(&mut self) {
        self.shutdown();
    }
}
impl MemoryBackend for GpuBackend2 {
    fn allocate(
        &self,
        arena: ArenaKind,
        size: u64,
        align: u64,
    ) -> Result<RegionHandle, BackendError2> {
        let len = rounded_len(size, align)?;
        let allocator = self
            .allocators
            .allocator(arena)
            .ok_or(BackendError2::AllocationFailed)?;
        let region = allocator.allocate(len)?;
        debug_assert_eq!(region.offset() % align, 0);
        Ok(region)
    }
    fn free(&self, region: &RegionHandle) -> Result<(), BackendError2> {
        self.allocators.allocator_for(region)?.free(region)?;
        Ok(())
    }
    fn free_deferred(
        &self,
        region: &RegionHandle,
        event: Event,
    ) -> Result<(), BackendError2> {
        self.allocators.allocator_for(region)?.free_deferred(region, event)?;
        Ok(())
    }
    fn budget(&self) -> Result<Budget, BackendError2> {
        let budget = Budget::from_query(&self.query, 0)?;
        debug!(
            "gpu backend {}: budget free {} MiB, total {} MiB", self.ordinal, mib(budget
            .free), mib(budget.total)
        );
        Ok(budget)
    }
    fn available(&self, arena: ArenaKind) -> Result<u64, BackendError2> {
        if arena == ArenaKind::Staging || read2(&self.blocks[slot_of(arena)]).is_none() {
            return Ok(0);
        }
        Ok(self.allocators.allocator(arena).map_or(0, |a| a.available()))
    }
}
#[cfg(test)]
#[path = "test/gpu_backend.rs"]
mod tests;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::_1::sync_util::lock;
