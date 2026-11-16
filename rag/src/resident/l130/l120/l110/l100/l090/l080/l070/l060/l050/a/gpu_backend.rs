//! GPU memory backend: contiguous zeroed VRAM arenas, one mapped host block and asynchronous transfers on a dedicated copy stream.

use std::ffi::c_void;
use std::fmt;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::ptr::{self, NonNull};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, MutexGuard, RwLock, RwLockReadGuard, RwLockWriteGuard};

use candle_core::cuda_backend::CudaDevice;
use cudarc::driver::result as drv;
use cudarc::driver::{sys, CudaContext};
use log::{debug, info, warn};

use super::super::l040::a::allocator::AllocatorSet;
use super::super::l040::a::library_adapter::{wrap_region, DeviceBlock, LibTensor};
use super::super::l040::l030::a::bookkeeping::{Bookkeeping, Region as BRegion};
use super::super::l040::l030::a::region_handle::{Generation, RegionHandle};
use super::super::l040::l030::l020::a::arenas::{
    fit_check, Arena, ArenaCapacities, ArenaKind, ArenaSet, Budget, MemInfo, MemoryQuery, MIB,
};
use super::super::l040::l030::l020::a::memory_access::AccessError;
use super::super::l040::l030::l020::l010::device_lane_event::{
    DeviceHandle, DeviceKind, Event, GpuEventBackend, Lane, LaneConfig, LaneKind, StreamBackend,
};
use super::super::l040::l030::l020::l010::layout::Layout;
use super::memory_backend::{rounded_len, BackendError, MemoryBackend, TransferDirection};

const SLOTS: usize = ArenaKind::ALL.len();

fn lock<T>(m: &Mutex<T>) -> MutexGuard<'_, T> {
    m.lock().unwrap_or_else(|poisoned| poisoned.into_inner())
}

fn read<T>(l: &RwLock<T>) -> RwLockReadGuard<'_, T> {
    l.read().unwrap_or_else(|poisoned| poisoned.into_inner())
}

fn write<T>(l: &RwLock<T>) -> RwLockWriteGuard<'_, T> {
    l.write().unwrap_or_else(|poisoned| poisoned.into_inner())
}

fn text(e: drv::DriverError) -> String {
    e.to_string()
}

fn unavailable(reason: impl Into<String>) -> BackendError {
    BackendError::DeviceMemoryUnavailable(reason.into())
}

fn slot_of(kind: ArenaKind) -> usize {
    kind.id() as usize
}

/// Converts a handle to the bookkeeping region it denotes; a generation beyond `u32` is stale.
fn to_region(handle: &RegionHandle) -> Result<BRegion, BackendError> {
    let generation =
        u32::try_from(handle.generation().0).map_err(|_| BackendError::StaleRegion)?;
    Ok(BRegion {
        arena: handle.arena(),
        offset: handle.offset(),
        len: handle.len(),
        generation,
    })
}

/// Pinned, zeroed host buffer.
pub struct PinnedBuffer {
    ptr: NonNull<u8>,
    len: usize,
    ctx: Option<Arc<CudaContext>>,
}

// SAFETY: the buffer owns its allocation exclusively; access follows Rust borrow rules.
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
        ctx.bind_to_thread().map_err(text)?;
        let raw = unsafe { drv::malloc_host(len, sys::CU_MEMHOSTALLOC_PORTABLE) }.map_err(text)?;
        let ptr = NonNull::new(raw.cast::<u8>()).ok_or_else(|| "null host allocation".to_string())?;
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
        f.debug_struct("PinnedBuffer")
            .field("len", &self.len)
            .finish()
    }
}

/// Ring of pinned, zeroed buffers.
pub struct StagingRing {
    buffers: Vec<PinnedBuffer>,
    cursor: AtomicUsize,
}

impl StagingRing {
    fn from_buffers(buffers: Vec<PinnedBuffer>) -> StagingRing {
        StagingRing {
            buffers,
            cursor: AtomicUsize::new(0),
        }
    }

    pub fn len(&self) -> usize {
        self.buffers.len()
    }

    pub fn is_empty(&self) -> bool {
        self.buffers.is_empty()
    }

    pub fn buffer_size(&self) -> usize {
        self.buffers.first().map_or(0, PinnedBuffer::len)
    }

    pub fn get(&self, i: usize) -> Option<&PinnedBuffer> {
        self.buffers.get(i)
    }

    pub fn get_mut(&mut self, i: usize) -> Option<&mut PinnedBuffer> {
        self.buffers.get_mut(i)
    }

    /// Round-robin index in `0..len`; `None` for an empty ring.
    pub fn next_index(&self) -> Option<usize> {
        if self.buffers.is_empty() {
            return None;
        }
        Some(self.cursor.fetch_add(1, Ordering::Relaxed) % self.buffers.len())
    }
}

impl fmt::Debug for StagingRing {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("StagingRing")
            .field("count", &self.len())
            .field("buffer_size", &self.buffer_size())
            .finish()
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

struct StreamOwner {
    ctx: Arc<CudaContext>,
    raw: sys::CUstream,
}

// SAFETY: a CUDA stream handle may be used from any thread; submissions are serialized by the backend.
unsafe impl Send for StreamOwner {}
unsafe impl Sync for StreamOwner {}

impl fmt::Debug for StreamOwner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "StreamOwner({:p})", self.raw)
    }
}

impl StreamOwner {
    fn create(ctx: &Arc<CudaContext>) -> Result<Arc<StreamOwner>, String> {
        ctx.bind_to_thread().map_err(text)?;
        let raw = drv::stream::create(drv::stream::StreamKind::NonBlocking).map_err(text)?;
        Ok(Arc::new(StreamOwner {
            ctx: Arc::clone(ctx),
            raw,
        }))
    }

    fn synchronize(&self) -> Result<(), String> {
        self.ctx.bind_to_thread().map_err(text)?;
        unsafe { drv::stream::synchronize(self.raw) }.map_err(text)
    }
}

impl Drop for StreamOwner {
    fn drop(&mut self) {
        if self.ctx.bind_to_thread().is_ok() {
            let _ = unsafe { drv::stream::destroy(self.raw) };
        }
    }
}

#[derive(Debug)]
struct CopyStreamBackend {
    owner: Arc<StreamOwner>,
}

impl StreamBackend for CopyStreamBackend {
    fn record_event(&self) -> Result<Box<dyn GpuEventBackend>, String> {
        let ctx = &self.owner.ctx;
        ctx.bind_to_thread().map_err(text)?;
        let raw = drv::event::create(sys::CUevent_flags::CU_EVENT_DISABLE_TIMING).map_err(text)?;
        let event = CuEvent {
            ctx: Arc::clone(ctx),
            raw,
        };
        unsafe { drv::event::record(event.raw, self.owner.raw) }.map_err(text)?;
        Ok(Box::new(event))
    }
}

#[derive(Debug)]
struct CuEvent {
    ctx: Arc<CudaContext>,
    raw: sys::CUevent,
}

// SAFETY: CUDA events may be queried and waited on from any thread.
unsafe impl Send for CuEvent {}
unsafe impl Sync for CuEvent {}

impl GpuEventBackend for CuEvent {
    fn query(&self) -> Result<bool, String> {
        self.ctx.bind_to_thread().map_err(text)?;
        match unsafe { drv::event::query(self.raw) } {
            Ok(()) => Ok(true),
            Err(e) if e.0 == sys::CUresult::CUDA_ERROR_NOT_READY => Ok(false),
            Err(e) => Err(text(e)),
        }
    }

    fn synchronize(&self) -> Result<(), String> {
        self.ctx.bind_to_thread().map_err(text)?;
        unsafe { drv::event::synchronize(self.raw) }.map_err(text)
    }
}

impl Drop for CuEvent {
    fn drop(&mut self) {
        if self.ctx.bind_to_thread().is_ok() {
            let _ = unsafe { drv::event::destroy(self.raw) };
        }
    }
}

struct DriverQuery {
    ctx: Arc<CudaContext>,
}

impl MemoryQuery for DriverQuery {
    fn query(&self) -> Result<MemInfo, String> {
        self.ctx.bind_to_thread().map_err(text)?;
        let (free, total) = drv::mem_get_info().map_err(text)?;
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

// SAFETY: a block is a plain address range; every access goes through the arena lock.
unsafe impl Send for Block {}
unsafe impl Sync for Block {}

fn alloc_device(stream: sys::CUstream, capacity: u64) -> Result<Block, String> {
    let len = usize::try_from(capacity).map_err(|_| "arena exceeds address space".to_string())?;
    let dev = unsafe { drv::malloc_async(stream, len) }.map_err(text)?;
    if let Err(e) = unsafe { drv::memset_d8_async(dev, 0, len, stream) } {
        let _ = unsafe { drv::free_async(dev, stream) };
        return Err(text(e));
    }
    Ok(Block {
        dev,
        host: ptr::null_mut(),
        len: capacity,
    })
}

fn alloc_mapped(capacity: u64) -> Result<Block, String> {
    let len = usize::try_from(capacity).map_err(|_| "arena exceeds address space".to_string())?;
    let raw = unsafe { drv::malloc_host(len, sys::CU_MEMHOSTALLOC_DEVICEMAP) }.map_err(text)?;
    let mut dev: sys::CUdeviceptr = 0;
    if let Err(e) = unsafe { sys::cuMemHostGetDevicePointer_v2(&mut dev, raw, 0) }.result() {
        let _ = unsafe { drv::free_host(raw) };
        return Err(text(e));
    }
    let host = raw.cast::<u8>();
    unsafe { ptr::write_bytes(host, 0, len) };
    Ok(Block {
        dev,
        host,
        len: capacity,
    })
}

fn release_block(stream: sys::CUstream, block: &Block) -> Result<(), String> {
    if block.host.is_null() {
        unsafe { drv::free_async(block.dev, stream) }.map_err(text)
    } else {
        unsafe { drv::free_host(block.host.cast::<c_void>()) }.map_err(text)
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

fn finish_setup(ctx: &Arc<CudaContext>, owner: &StreamOwner) -> Result<(), String> {
    let pool = unsafe { drv::device::get_default_mem_pool(ctx.cu_device()) }.map_err(text)?;
    let mut threshold = u64::MAX;
    unsafe {
        drv::mem_pool::set_attribute(
            pool,
            sys::CUmemPool_attribute::CU_MEMPOOL_ATTR_RELEASE_THRESHOLD,
            (&mut threshold as *mut u64).cast::<c_void>(),
        )
    }
    .map_err(text)?;
    owner.synchronize()?;
    ctx.synchronize().map_err(text)
}

fn rollback(ctx: &Arc<CudaContext>, owner: &StreamOwner, made: &[(usize, Block)]) {
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
    owner: &StreamOwner,
    arenas: &[Arena],
) -> Result<Vec<(usize, Block)>, String> {
    ctx.bind_to_thread().map_err(text)?;
    let mut made = Vec::new();
    let outcome = fill_blocks(owner.raw, arenas, &mut made).and_then(|()| finish_setup(ctx, owner));
    match outcome {
        Ok(()) => Ok(made),
        Err(e) => {
            rollback(ctx, owner, &made);
            Err(e)
        }
    }
}

fn open_context(ordinal: u32) -> Result<Arc<CudaContext>, BackendError> {
    match catch_unwind(AssertUnwindSafe(|| CudaContext::new(ordinal as usize))) {
        Ok(Ok(ctx)) => Ok(ctx),
        Ok(Err(e)) => Err(unavailable(text(e))),
        Err(_) => Err(unavailable("CUDA driver could not be loaded")),
    }
}

struct CopyStream {
    owner: Arc<StreamOwner>,
    lane: Lane,
    gate: Mutex<()>,
}

fn mib(bytes: u64) -> u64 {
    bytes / MIB
}

/// Memory backend for one Nvidia device. `Send + Sync`.
pub struct GpuBackend {
    device: DeviceHandle,
    ordinal: usize,
    ctx: Arc<CudaContext>,
    query: DriverQuery,
    arenas: Arc<ArenaSet>,
    allocators: AllocatorSet,
    blocks: [RwLock<Option<Block>>; SLOTS],
    copy: CopyStream,
    candle: Mutex<Option<CudaDevice>>,
}

impl GpuBackend {
    pub fn new(
        device: DeviceHandle,
        caps: ArenaCapacities,
        headroom: u64,
    ) -> Result<GpuBackend, BackendError> {
        let ordinal = match device {
            DeviceHandle::Gpu(ordinal) => ordinal,
            _ => return Err(unavailable("device is not a GPU")),
        };
        let ctx = open_context(ordinal)?;
        let query = DriverQuery {
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
        let specs: Vec<(Arena, DeviceKind)> =
            created.iter().map(|a| (*a, DeviceKind::Gpu)).collect();
        let allocators = AllocatorSet::new(bookkeeping, &specs)
            .map_err(|_| BackendError::AllocationFailed)?;

        let owner = StreamOwner::create(&ctx).map_err(unavailable)?;
        let built = setup_blocks(&ctx, &owner, &created).map_err(unavailable)?;
        let mut by_slot: [Option<Block>; SLOTS] = std::array::from_fn(|_| None);
        for (slot, block) in built {
            by_slot[slot] = Some(block);
        }

        let lane = Lane::new(
            LaneConfig::preset(LaneKind::CopyH2D, device),
            Some(Box::new(CopyStreamBackend {
                owner: Arc::clone(&owner),
            })),
        );
        info!(
            "gpu backend {}: free {} MiB, total {} MiB, weights {} MiB, index {} MiB, workspace {} MiB, staging {} MiB, parameters {} MiB, mapped host {} MiB",
            ordinal,
            mib(budget.free),
            mib(budget.total),
            mib(created[slot_of(ArenaKind::Weights)].capacity),
            mib(created[slot_of(ArenaKind::IndexSegments)].capacity),
            mib(created[slot_of(ArenaKind::Workspace)].capacity),
            mib(created[slot_of(ArenaKind::Staging)].capacity),
            mib(created[slot_of(ArenaKind::ParameterBlock)].capacity),
            mib(created[slot_of(ArenaKind::MappedHost)].capacity),
        );
        Ok(GpuBackend {
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

    fn handle_for(&self, region: &BRegion) -> Result<RegionHandle, BackendError> {
        let arena = self
            .arenas
            .get(region.arena)
            .ok_or(BackendError::StaleRegion)?;
        RegionHandle::new_in(
            &arena,
            region.offset,
            region.len,
            Generation(u64::from(region.generation)),
        )
        .map_err(|_| BackendError::StaleRegion)
    }

    /// Validates liveness and bounds of `handle` against the arena block in `slot`.
    fn resolve<'a>(
        &self,
        slot: &'a Option<Block>,
        handle: &RegionHandle,
    ) -> Result<(BRegion, &'a Block, u64), BackendError> {
        let region = to_region(handle)?;
        let allocator = self
            .allocators
            .allocator_for(&region)
            .map_err(|_| BackendError::StaleRegion)?;
        if !allocator.is_live(&region) {
            return Err(BackendError::StaleRegion);
        }
        let block = slot.as_ref().ok_or(BackendError::StaleRegion)?;
        let end = region
            .offset
            .checked_add(region.len)
            .ok_or(BackendError::StaleRegion)?;
        if end > block.len {
            return Err(BackendError::StaleRegion);
        }
        let addr = block
            .dev
            .checked_add(region.offset)
            .ok_or(BackendError::StaleRegion)?;
        Ok((region, block, addr))
    }

    /// Enqueues `op` on the copy stream and records the covering event under one lock.
    fn enqueue<F>(
        &self,
        direction: TransferDirection,
        active: bool,
        op: F,
    ) -> Result<Event, BackendError>
    where
        F: FnOnce(sys::CUstream) -> Result<(), drv::DriverError>,
    {
        let fail = |reason: String| BackendError::Transfer { direction, reason };
        let _gate = lock(&self.copy.gate);
        self.ctx.bind_to_thread().map_err(|e| fail(text(e)))?;
        if active {
            op(self.copy.owner.raw).map_err(|e| fail(text(e)))?;
        }
        self.copy
            .lane
            .record_event()
            .map_err(|e| fail(e.to_string()))
    }

    pub fn copy_d2d(
        &self,
        src: &RegionHandle,
        dst: &RegionHandle,
        len: u64,
    ) -> Result<Event, BackendError> {
        let direction = TransferDirection::D2D;
        if src.arena() == dst.arena() {
            return Err(BackendError::Transfer {
                direction,
                reason: "same arena".to_string(),
            });
        }
        let (src_slot, dst_slot) = (slot_of(src.arena()), slot_of(dst.arena()));
        let (src_guard, dst_guard);
        if src_slot < dst_slot {
            src_guard = read(&self.blocks[src_slot]);
            dst_guard = read(&self.blocks[dst_slot]);
        } else {
            dst_guard = read(&self.blocks[dst_slot]);
            src_guard = read(&self.blocks[src_slot]);
        }
        let (_, _, src_addr) = self.resolve(&src_guard, src)?;
        let (_, _, dst_addr) = self.resolve(&dst_guard, dst)?;
        if len > src.len() || len > dst.len() {
            return Err(BackendError::Transfer {
                direction,
                reason: "size mismatch".to_string(),
            });
        }
        self.enqueue(direction, len > 0, |stream| unsafe {
            drv::memcpy_dtod_async(dst_addr, src_addr, len as usize, stream)
        })
    }

    pub fn upload(&self, dst: &RegionHandle, src: HostSource<'_>) -> Result<Event, BackendError> {
        let direction = TransferDirection::H2D;
        let guard = read(&self.blocks[slot_of(dst.arena())]);
        let (_, _, addr) = self.resolve(&guard, dst)?;
        let data = src.as_slice();
        if data.len() as u64 > dst.len() {
            return Err(BackendError::Transfer {
                direction,
                reason: "size mismatch".to_string(),
            });
        }
        self.enqueue(direction, !data.is_empty(), |stream| unsafe {
            drv::memcpy_htod_async(addr, data, stream)
        })
    }

    pub fn download(&self, src: &RegionHandle, dst: &PinnedBuffer) -> Result<Event, BackendError> {
        let direction = TransferDirection::D2H;
        let guard = read(&self.blocks[slot_of(src.arena())]);
        let (_, _, addr) = self.resolve(&guard, src)?;
        if dst.len() as u64 > src.len() {
            return Err(BackendError::Transfer {
                direction,
                reason: "size mismatch".to_string(),
            });
        }
        let (host, len) = (dst.ptr.as_ptr().cast::<c_void>(), dst.len());
        self.enqueue(direction, len > 0, |stream| unsafe {
            sys::cuMemcpyDtoHAsync_v2(host, addr, len, stream).result()
        })
    }

    pub fn create_staging_ring(
        &self,
        count: usize,
        buffer_size: usize,
    ) -> Result<StagingRing, BackendError> {
        count
            .checked_mul(buffer_size)
            .ok_or_else(|| unavailable("staging ring size overflows"))?;
        let mut buffers = Vec::new();
        for _ in 0..count {
            buffers.push(PinnedBuffer::allocate(&self.ctx, buffer_size).map_err(unavailable)?);
        }
        Ok(StagingRing::from_buffers(buffers))
    }

    /// Locks the mapped block and validates an access; returns the guard and the host address.
    fn mapped_span(
        &self,
        region: &RegionHandle,
        offset: usize,
        len: usize,
    ) -> Result<(RwLockReadGuard<'_, Option<Block>>, *mut u8), BackendError> {
        if region.arena() != ArenaKind::MappedHost {
            return Err(BackendError::ArenaMismatch {
                expected: ArenaKind::MappedHost,
                found: region.arena(),
            });
        }
        let guard = read(&self.blocks[slot_of(ArenaKind::MappedHost)]);
        let ptr = {
            let (_, block, _) = self.resolve(&guard, region)?;
            let region_len = usize::try_from(region.len()).unwrap_or(usize::MAX);
            let out_of_bounds = || {
                BackendError::Access(AccessError::OutOfBounds {
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
    ) -> Result<(), BackendError> {
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
    ) -> Result<(), BackendError> {
        let (_guard, dst) = self.mapped_span(region, offset, src.len())?;
        if !src.is_empty() {
            unsafe { ptr::copy_nonoverlapping(src.as_ptr(), dst, src.len()) };
        }
        Ok(())
    }

    /// Device address of the start of the mapped host block.
    pub fn mapped_device_addr(&self) -> Option<u64> {
        read(&self.blocks[slot_of(ArenaKind::MappedHost)])
            .as_ref()
            .map(|b| b.dev)
    }

    fn candle_device(&self) -> Result<CudaDevice, BackendError> {
        let mut cached = lock(&self.candle);
        if let Some(device) = cached.as_ref() {
            return Ok(device.clone());
        }
        let ordinal = self.ordinal;
        let device = match catch_unwind(AssertUnwindSafe(|| CudaDevice::new_with_stream(ordinal))) {
            Ok(Ok(device)) => device,
            Ok(Err(e)) => return Err(unavailable(e.to_string())),
            Err(_) => return Err(unavailable("library device could not be created")),
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
    ) -> Result<LibTensor, BackendError> {
        let guard = read(&self.blocks[slot_of(region.arena())]);
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
        let mut guards: Vec<RwLockWriteGuard<'_, Option<Block>>> =
            self.blocks.iter().map(write).collect();
        let _gate = lock(&self.copy.gate);
        let blocks: Vec<Block> = guards.iter_mut().filter_map(|g| g.take()).collect();
        if blocks.is_empty() {
            return;
        }
        if self.ctx.bind_to_thread().is_err() {
            warn!("gpu backend {}: context not bindable, blocks are leaked", self.ordinal);
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

impl fmt::Debug for GpuBackend {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("GpuBackend")
            .field("device", &self.device)
            .field("capacities", &self.capacities())
            .finish()
    }
}

impl Drop for GpuBackend {
    fn drop(&mut self) {
        self.shutdown();
    }
}

impl MemoryBackend for GpuBackend {
    fn allocate(
        &self,
        arena: ArenaKind,
        size: u64,
        align: u64,
    ) -> Result<RegionHandle, BackendError> {
        let len = rounded_len(size, align)?;
        let allocator = self
            .allocators
            .allocator(arena)
            .ok_or(BackendError::AllocationFailed)?;
        let region = allocator.allocate(len)?;
        debug_assert_eq!(region.offset % align, 0);
        match self.handle_for(&region) {
            Ok(handle) => Ok(handle),
            Err(e) => {
                let _ = allocator.free(&region);
                Err(e)
            }
        }
    }

    fn free(&self, region: &RegionHandle) -> Result<(), BackendError> {
        let region = to_region(region)?;
        self.allocators.allocator_for(&region)?.free(&region)?;
        Ok(())
    }

    fn free_deferred(&self, region: &RegionHandle, event: Event) -> Result<(), BackendError> {
        let region = to_region(region)?;
        self.allocators
            .allocator_for(&region)?
            .free_deferred(&region, event)?;
        Ok(())
    }

    fn budget(&self) -> Result<Budget, BackendError> {
        let budget = Budget::from_query(&self.query, 0)?;
        debug!(
            "gpu backend {}: budget free {} MiB, total {} MiB",
            self.ordinal,
            mib(budget.free),
            mib(budget.total)
        );
        Ok(budget)
    }

    fn available(&self, arena: ArenaKind) -> Result<u64, BackendError> {
        if arena == ArenaKind::Staging || read(&self.blocks[slot_of(arena)]).is_none() {
            return Ok(0);
        }
        Ok(self
            .allocators
            .allocator(arena)
            .map_or(0, |a| a.available()))
    }
}

#[cfg(test)]
#[path = "test/gpu_backend.rs"]
mod tests;
