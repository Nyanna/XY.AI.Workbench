//! CUDA binding of `GpuBackend` on the driver API: primary contexts, prioritised streams,
//! arena memory and host-to-device copies on one copy stream per device.
use std::any::Any;
use std::collections::HashMap;
use std::ffi::c_void;
use std::fmt;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::ptr::{self, NonNull};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex, Weak};
use cudarc::driver::result as drv;
use cudarc::driver::{sys, CudaContext};
use super::gpu_subengine::{GpuBackend, GpuDeviceInfo};
use crate::resident::l130::l120::l110::l100::l090 as l090;
use l090::l080 as l080;
use l080::l070 as l070;
use l070::l060 as l060;
use l060::l050 as l050;
use l050::l040 as l040;
use l040::l030 as l030;
use l030::l020 as l020;
use l020::l010 as l010;
use l080::a::capability_detection::GpuProbe;
use l050::a::table_swap::{RootSlot, TableSink};
use l030::a::kernel_layout::Addr;
use l030::a::region_handle::RegionHandle;
use l020::a::arenas::{Arena, ArenaKind, MemInfo, MemoryQuery};
use l020::a::memory_access::{AccessError, AccessRegion, RegionBackend};
use l010::device_lane_event::{Event, GpuEventBackend, Lane, Priority, StreamBackend};
use l010::sync_util::lock;
use l010::target_env::ComputeCapability;
const ROOT_BYTES: usize = 64;
const TABLE_ALIGN: usize = 256;
fn text(e: drv::DriverError) -> String {
    e.to_string()
}
fn ensure_driver() -> Result<(), String> {
    match catch_unwind(AssertUnwindSafe(drv::init)) {
        Ok(Ok(())) => Ok(()),
        Ok(Err(e)) => Err(text(e)),
        Err(_) => Err("CUDA driver could not be loaded".to_string()),
    }
}
struct StreamOwner {
    ctx: Arc<CudaContext>,
    raw: sys::CUstream,
}
unsafe impl Send for StreamOwner {}
unsafe impl Sync for StreamOwner {}
impl fmt::Debug for StreamOwner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("StreamOwner").field("raw", &self.raw).finish()
    }
}
impl StreamOwner {
    fn create(
        ctx: &Arc<CudaContext>,
        priority: Priority,
    ) -> Result<Arc<StreamOwner>, String> {
        ctx.bind_to_thread().map_err(text)?;
        let (mut least, mut greatest) = (0i32, 0i32);
        unsafe { sys::cuCtxGetStreamPriorityRange(&mut least, &mut greatest) }
            .result()
            .map_err(text)?;
        let level = match priority {
            Priority::High => greatest,
            Priority::Normal | Priority::Low => least,
        };
        let mut raw: sys::CUstream = ptr::null_mut();
        unsafe {
            sys::cuStreamCreateWithPriority(
                &mut raw,
                sys::CUstream_flags::CU_STREAM_NON_BLOCKING as u32,
                level,
            )
        }
            .result()
            .map_err(text)?;
        Ok(
            Arc::new(StreamOwner {
                ctx: Arc::clone(ctx),
                raw,
            }),
        )
    }
    fn synchronize(&self) -> Result<(), String> {
        self.ctx.bind_to_thread().map_err(text)?;
        unsafe { drv::stream::synchronize(self.raw) }.map_err(text)
    }
    fn record(self: &Arc<Self>) -> Result<CuEvent, String> {
        self.ctx.bind_to_thread().map_err(text)?;
        let raw = drv::event::create(sys::CUevent_flags::CU_EVENT_DISABLE_TIMING)
            .map_err(text)?;
        let event = CuEvent {
            ctx: Arc::clone(&self.ctx),
            raw,
        };
        unsafe { drv::event::record(event.raw, self.raw) }.map_err(text)?;
        Ok(event)
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
struct CudaStream {
    owner: Arc<StreamOwner>,
}
unsafe extern "C" fn run_callback(arg: *mut c_void) {
    let callback = *Box::from_raw(arg.cast::<Box<dyn FnOnce() + Send>>());
    let _ = catch_unwind(AssertUnwindSafe(callback));
}
impl StreamBackend for CudaStream {
    fn record_event(&self) -> Result<Box<dyn GpuEventBackend>, String> {
        Ok(Box::new(self.owner.record()?))
    }
    fn notify_on_complete(&self, f: Box<dyn FnOnce() + Send>) -> Result<bool, String> {
        self.owner.ctx.bind_to_thread().map_err(text)?;
        let arg = Box::into_raw(Box::new(f)).cast::<c_void>();
        match unsafe {
            drv::stream::launch_host_function(self.owner.raw, run_callback, arg)
        } {
            Ok(()) => Ok(true),
            Err(e) => {
                drop(unsafe { Box::from_raw(arg.cast::<Box<dyn FnOnce() + Send>>()) });
                Err(text(e))
            }
        }
    }
    fn as_any(&self) -> Option<&dyn Any> {
        Some(self)
    }
}
/// Context and raw handle of a stream created by this backend; `None` for any other stream.
pub(super) fn stream_of(
    stream: &dyn StreamBackend,
) -> Option<(Arc<CudaContext>, sys::CUstream)> {
    let cuda = stream.as_any()?.downcast_ref::<CudaStream>()?;
    Some((Arc::clone(&cuda.owner.ctx), cuda.owner.raw))
}
#[derive(Debug)]
struct CuEvent {
    ctx: Arc<CudaContext>,
    raw: sys::CUevent,
}
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
/// Eight bytes of device-mapped pinned host memory that the device reads with acquire.
struct MappedRoot {
    ctx: Arc<CudaContext>,
    host: NonNull<u8>,
    dev: u64,
}
unsafe impl Send for MappedRoot {}
unsafe impl Sync for MappedRoot {}
impl MappedRoot {
    fn create(ctx: &Arc<CudaContext>) -> Result<MappedRoot, String> {
        ctx.bind_to_thread().map_err(text)?;
        let raw = unsafe { drv::malloc_host(ROOT_BYTES, sys::CU_MEMHOSTALLOC_DEVICEMAP) }
            .map_err(text)?;
        let mut dev: sys::CUdeviceptr = 0;
        if let Err(e) = unsafe { sys::cuMemHostGetDevicePointer_v2(&mut dev, raw, 0) }
            .result()
        {
            let _ = unsafe { drv::free_host(raw) };
            return Err(text(e));
        }
        let host = NonNull::new(raw.cast::<u8>()).ok_or("null host allocation")?;
        unsafe { ptr::write_bytes(host.as_ptr(), 0, ROOT_BYTES) };
        Ok(MappedRoot {
            ctx: Arc::clone(ctx),
            host,
            dev,
        })
    }
    fn cell(&self) -> &AtomicU64 {
        unsafe { &*self.host.as_ptr().cast::<AtomicU64>() }
    }
}
impl RootSlot for MappedRoot {
    fn store_release(&self, value: u64) {
        self.cell().store(value, Ordering::Release);
    }
    fn load_acquire(&self) -> u64 {
        self.cell().load(Ordering::Acquire)
    }
}
impl Drop for MappedRoot {
    fn drop(&mut self) {
        if self.ctx.bind_to_thread().is_ok() {
            let _ = unsafe { drv::free_host(self.host.as_ptr().cast::<c_void>()) };
        }
    }
}
/// Everything the backend keeps per retained device.
struct DeviceState {
    ctx: Arc<CudaContext>,
    copy: Arc<StreamOwner>,
    gate: Mutex<()>,
    root: Arc<MappedRoot>,
    next_table: Mutex<usize>,
}
impl DeviceState {
    fn copy_in(&self, dst: u64, bytes: &[u8]) -> Result<(), String> {
        if bytes.is_empty() {
            return Ok(());
        }
        let _gate = lock(&self.gate);
        self.ctx.bind_to_thread().map_err(text)?;
        unsafe { drv::memcpy_htod_async(dst, bytes, self.copy.raw) }.map_err(text)?;
        self.copy.synchronize()
    }
    fn copy_in_visible(&self, dst: u64, bytes: &[u8]) -> Result<CuEvent, String> {
        let _gate = lock(&self.gate);
        self.ctx.bind_to_thread().map_err(text)?;
        if !bytes.is_empty() {
            unsafe { drv::memcpy_htod_async(dst, bytes, self.copy.raw) }.map_err(text)?;
        }
        self.copy.record()
    }
    fn copy_out(&self, src: u64, out: &mut [u8]) -> Result<(), String> {
        if out.is_empty() {
            return Ok(());
        }
        let _gate = lock(&self.gate);
        self.ctx.bind_to_thread().map_err(text)?;
        unsafe { drv::memcpy_dtoh_async(out, src, self.copy.raw) }.map_err(text)?;
        self.copy.synchronize()
    }
    fn zero(&self, dst: u64, len: usize) -> Result<(), String> {
        let _gate = lock(&self.gate);
        self.ctx.bind_to_thread().map_err(text)?;
        unsafe { drv::memset_d8_async(dst, 0, len, self.copy.raw) }.map_err(text)?;
        self.copy.synchronize()
    }
}
fn out_of_bounds(offset: usize, len: usize, region_len: usize) -> AccessError {
    AccessError::OutOfBounds {
        offset,
        len,
        region_len,
    }
}
/// A device arena in VRAM. Dropping it frees the memory.
struct DeviceRegion {
    state: Arc<DeviceState>,
    dev: u64,
    len: usize,
}
impl fmt::Debug for DeviceRegion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DeviceRegion")
            .field("dev", &self.dev)
            .field("len", &self.len)
            .finish()
    }
}
impl RegionBackend for DeviceRegion {
    fn len(&self) -> usize {
        self.len
    }
    fn read_raw(&self, offset: usize, dst: &mut [u8]) -> Result<(), AccessError> {
        if offset.checked_add(dst.len()).is_none_or(|end| end > self.len) {
            return Err(out_of_bounds(offset, dst.len(), self.len));
        }
        self.state.copy_out(self.dev + offset as u64, dst).map_err(AccessError::Backend)
    }
    fn write_raw(&self, offset: usize, src: &[u8]) -> Result<(), AccessError> {
        if offset.checked_add(src.len()).is_none_or(|end| end > self.len) {
            return Err(out_of_bounds(offset, src.len(), self.len));
        }
        self.state.copy_in(self.dev + offset as u64, src).map_err(AccessError::Backend)
    }
}
impl Drop for DeviceRegion {
    fn drop(&mut self) {
        if self.len > 0 && self.state.ctx.bind_to_thread().is_ok() {
            let _ = unsafe { drv::free_sync(self.dev) };
        }
    }
}
/// A pinned host arena. Dropping it frees the memory.
struct HostRegion {
    ctx: Arc<CudaContext>,
    ptr: Option<NonNull<u8>>,
    len: usize,
    access: Mutex<()>,
}
unsafe impl Send for HostRegion {}
unsafe impl Sync for HostRegion {}
impl fmt::Debug for HostRegion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("HostRegion").field("len", &self.len).finish()
    }
}
impl RegionBackend for HostRegion {
    fn len(&self) -> usize {
        self.len
    }
    fn read_raw(&self, offset: usize, dst: &mut [u8]) -> Result<(), AccessError> {
        let Some(base) = self.ptr else {
            return Err(out_of_bounds(offset, dst.len(), 0));
        };
        if offset.checked_add(dst.len()).is_none_or(|end| end > self.len) {
            return Err(out_of_bounds(offset, dst.len(), self.len));
        }
        let _guard = lock(&self.access);
        unsafe {
            ptr::copy_nonoverlapping(
                base.as_ptr().add(offset),
                dst.as_mut_ptr(),
                dst.len(),
            )
        };
        Ok(())
    }
    fn write_raw(&self, offset: usize, src: &[u8]) -> Result<(), AccessError> {
        let Some(base) = self.ptr else {
            return Err(out_of_bounds(offset, src.len(), 0));
        };
        if offset.checked_add(src.len()).is_none_or(|end| end > self.len) {
            return Err(out_of_bounds(offset, src.len(), self.len));
        }
        let _guard = lock(&self.access);
        unsafe {
            ptr::copy_nonoverlapping(src.as_ptr(), base.as_ptr().add(offset), src.len())
        };
        Ok(())
    }
}
impl Drop for HostRegion {
    fn drop(&mut self) {
        if let Some(ptr) = self.ptr {
            if self.ctx.bind_to_thread().is_ok() {
                let _ = unsafe { drv::free_host(ptr.as_ptr().cast::<c_void>()) };
            }
        }
    }
}
type DeviceRegions = Arc<Mutex<HashMap<(u32, ArenaKind), Weak<DeviceRegion>>>>;
struct DeviceProbe {
    capability: Option<ComputeCapability>,
    name: Option<String>,
}
impl GpuProbe for DeviceProbe {
    fn compute_capability(&self) -> Option<ComputeCapability> {
        self.capability
    }
    fn device_name(&self) -> Option<String> {
        self.name.clone()
    }
}
struct DriverQuery {
    state: Option<Arc<DeviceState>>,
}
impl MemoryQuery for DriverQuery {
    fn query(&self) -> Result<MemInfo, String> {
        let state = self.state.as_ref().ok_or("the primary context is not retained")?;
        state.ctx.bind_to_thread().map_err(text)?;
        let (free, total) = drv::mem_get_info().map_err(text)?;
        Ok(MemInfo {
            free: free as u64,
            total: total as u64,
        })
    }
}
/// Writes into the `IndexSegments` arena and returns once the bytes are visible to the device.
struct SegmentSink {
    ordinal: u32,
    state: Arc<DeviceState>,
    regions: DeviceRegions,
}
impl TableSink for SegmentSink {
    fn write_visible(&self, region: &RegionHandle, bytes: &[u8]) -> Result<(), String> {
        let arena = device_region(
            &self.regions,
            self.ordinal,
            ArenaKind::IndexSegments,
        )?;
        let end = region.offset().checked_add(bytes.len() as u64);
        if end.is_none_or(|e| e > arena.len as u64) {
            return Err("table write beyond the IndexSegments arena".to_string());
        }
        self.state.copy_in(arena.dev + region.offset(), bytes)
    }
}
fn device_region(
    regions: &DeviceRegions,
    ordinal: u32,
    kind: ArenaKind,
) -> Result<Arc<DeviceRegion>, String> {
    lock(regions)
        .get(&(ordinal, kind))
        .and_then(Weak::upgrade)
        .ok_or_else(|| format!("the {kind:?} arena of device {ordinal} is not reserved"))
}
/// CUDA implementation of `GpuBackend`. Send + Sync.
pub struct CudaBackend {
    devices: Mutex<HashMap<u32, Arc<DeviceState>>>,
    regions: DeviceRegions,
}
static_assertions::assert_impl_all!(CudaBackend : Send, Sync);
impl Default for CudaBackend {
    fn default() -> Self {
        CudaBackend::new()
    }
}
impl CudaBackend {
    /// Touches no device; the driver is initialised by `devices`.
    pub fn new() -> CudaBackend {
        CudaBackend {
            devices: Mutex::new(HashMap::new()),
            regions: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    fn state(&self, ordinal: u32) -> Result<Arc<DeviceState>, String> {
        lock(&self.devices)
            .get(&ordinal)
            .cloned()
            .ok_or_else(|| {
                format!("the primary context of device {ordinal} is not retained")
            })
    }
    /// Device address of the root slot of the device; the kernel reads the table root there.
    pub fn root_device_address(&self, ordinal: u32) -> Option<u64> {
        self.state(ordinal).ok().map(|s| s.root.dev)
    }
    fn attribute(
        dev: sys::CUdevice,
        attribute: sys::CUdevice_attribute,
    ) -> Result<u32, String> {
        let value = unsafe { drv::device::get_attribute(dev, attribute) }.map_err(text)?;
        Ok(u32::try_from(value).unwrap_or(0))
    }
    fn describe(ordinal: u32) -> Result<(GpuDeviceInfo, sys::CUdevice), String> {
        let dev = drv::device::get(ordinal as i32).map_err(text)?;
        let capability = ComputeCapability {
            major: Self::attribute(
                dev,
                sys::CUdevice_attribute::CU_DEVICE_ATTRIBUTE_COMPUTE_CAPABILITY_MAJOR,
            )?,
            minor: Self::attribute(
                dev,
                sys::CUdevice_attribute::CU_DEVICE_ATTRIBUTE_COMPUTE_CAPABILITY_MINOR,
            )?,
        };
        let info = GpuDeviceInfo {
            ordinal,
            name: drv::device::get_name(dev).map_err(text)?,
            compute_capability: capability,
            multiprocessors: Self::attribute(
                dev,
                sys::CUdevice_attribute::CU_DEVICE_ATTRIBUTE_MULTIPROCESSOR_COUNT,
            )?,
            total_memory: unsafe { drv::device::total_mem(dev) }.map_err(text)? as u64,
        };
        Ok((info, dev))
    }
}
impl GpuBackend for CudaBackend {
    fn devices(&self) -> Result<Vec<GpuDeviceInfo>, String> {
        ensure_driver()?;
        let count = drv::device::get_count().map_err(text)?;
        (0..count.max(0) as u32)
            .map(|ordinal| Self::describe(ordinal).map(|(info, _)| info))
            .collect()
    }
    fn retain_primary_context(&self, ordinal: u32) -> Result<(), String> {
        ensure_driver()?;
        if lock(&self.devices).contains_key(&ordinal) {
            return Ok(());
        }
        let ctx = catch_unwind(AssertUnwindSafe(|| CudaContext::new(ordinal as usize)))
            .map_err(|_| "CUDA driver could not be loaded".to_string())?
            .map_err(text)?;
        let copy = StreamOwner::create(&ctx, Priority::Normal)?;
        let root = Arc::new(MappedRoot::create(&ctx)?);
        let state = Arc::new(DeviceState {
            ctx,
            copy,
            gate: Mutex::new(()),
            root,
            next_table: Mutex::new(0),
        });
        lock(&self.devices).entry(ordinal).or_insert(state);
        Ok(())
    }
    fn probe(&self, ordinal: u32) -> Arc<dyn GpuProbe> {
        let retained = lock(&self.devices).contains_key(&ordinal);
        let described = if retained { Self::describe(ordinal).ok() } else { None };
        Arc::new(DeviceProbe {
            capability: described.as_ref().map(|(info, _)| info.compute_capability),
            name: described.map(|(info, _)| info.name),
        })
    }
    fn create_stream(
        &self,
        ordinal: u32,
        priority: Priority,
    ) -> Result<Box<dyn StreamBackend>, String> {
        let state = self.state(ordinal)?;
        let owner = StreamOwner::create(&state.ctx, priority)?;
        Ok(Box::new(CudaStream { owner }))
    }
    fn memory_query(&self, ordinal: u32) -> Arc<dyn MemoryQuery> {
        Arc::new(DriverQuery {
            state: self.state(ordinal).ok(),
        })
    }
    fn reserve_arena(&self, ordinal: u32, arena: Arena) -> Result<AccessRegion, String> {
        let state = self.state(ordinal)?;
        let len = usize::try_from(arena.capacity)
            .map_err(|_| "arena exceeds the address space".to_string())?;
        state.ctx.bind_to_thread().map_err(text)?;
        if !arena.kind.is_device() {
            let ptr = if len == 0 {
                None
            } else {
                let raw = unsafe { drv::malloc_host(len, sys::CU_MEMHOSTALLOC_PORTABLE) }
                    .map_err(text)?;
                let ptr = NonNull::new(raw.cast::<u8>()).ok_or("null host allocation")?;
                unsafe { ptr::write_bytes(ptr.as_ptr(), 0, len) };
                Some(ptr)
            };
            let region = HostRegion {
                ctx: Arc::clone(&state.ctx),
                ptr,
                len,
                access: Mutex::new(()),
            };
            return Ok(AccessRegion::new(Arc::new(region)));
        }
        let dev = if len == 0 {
            0
        } else {
            unsafe { drv::malloc_sync(len) }.map_err(text)?
        };
        let region = Arc::new(DeviceRegion {
            state: Arc::clone(&state),
            dev,
            len,
        });
        if len > 0 {
            state.zero(dev, len)?;
        }
        lock(&self.regions).insert((ordinal, arena.kind), Arc::downgrade(&region));
        Ok(AccessRegion::new(region))
    }
    fn device_base(&self, ordinal: u32, handle: &RegionHandle) -> Option<Addr> {
        if handle.arena() != ArenaKind::IndexSegments {
            return None;
        }
        let arena = device_region(&self.regions, ordinal, ArenaKind::IndexSegments)
            .ok()?;
        (handle.end() <= arena.len as u64).then(|| Addr(arena.dev + handle.offset()))
    }
    fn write_visible(
        &self,
        ordinal: u32,
        handle: &RegionHandle,
        offset: u64,
        bytes: &[u8],
    ) -> Result<Event, String> {
        let state = self.state(ordinal)?;
        let arena = device_region(&self.regions, ordinal, ArenaKind::IndexSegments)?;
        let start = handle.offset().checked_add(offset);
        let end = start.and_then(|s| s.checked_add(bytes.len() as u64));
        if handle.arena() != ArenaKind::IndexSegments
            || offset.checked_add(bytes.len() as u64).is_none_or(|e| e > handle.len())
            || end.is_none_or(|e| e > arena.len as u64)
        {
            return Err("write beyond the region".to_string());
        }
        let event = state.copy_in_visible(arena.dev + start.unwrap_or(0), bytes)?;
        Ok(Event::gpu(ordinal, Box::new(event)))
    }
    /// The table is written completely before the call returns, so it is ordered before any
    /// later work of `lane`. Two halves of the ParameterBlock arena alternate: the previous
    /// table stays intact until the next one is written.
    fn write_kernel_table(
        &self,
        ordinal: u32,
        _lane: &mut Lane,
        table: &[u8],
    ) -> Result<u64, String> {
        let state = self.state(ordinal)?;
        let arena = device_region(&self.regions, ordinal, ArenaKind::ParameterBlock)?;
        let half = (arena.len / 2) / TABLE_ALIGN * TABLE_ALIGN;
        if table.len() > half {
            return Err(
                format!(
                    "kernel table of {} bytes exceeds the {half} bytes of a ParameterBlock half",
                    table.len()
                ),
            );
        }
        let mut next = lock(&state.next_table);
        let dst = arena.dev + (*next * half) as u64;
        state.copy_in(dst, table)?;
        *next ^= 1;
        Ok(dst)
    }
    fn table_sink(&self, ordinal: u32) -> Arc<dyn TableSink> {
        match self.state(ordinal) {
            Ok(state) => {
                Arc::new(SegmentSink {
                    ordinal,
                    state,
                    regions: Arc::clone(&self.regions),
                })
            }
            Err(reason) => Arc::new(FailedSink(reason)),
        }
    }
    fn root_slot(&self, ordinal: u32) -> Arc<dyn RootSlot> {
        match self.state(ordinal) {
            Ok(state) => Arc::clone(&state.root) as Arc<dyn RootSlot>,
            Err(_) => Arc::new(l050::a::table_swap::HostRootSlot::new()),
        }
    }
}
/// Stands in for a device that is not retained; every write fails.
struct FailedSink(String);
impl TableSink for FailedSink {
    fn write_visible(
        &self,
        _region: &RegionHandle,
        _bytes: &[u8],
    ) -> Result<(), String> {
        Err(self.0.clone())
    }
}
#[cfg(test)]
#[path = "test/cuda_backend.rs"]
mod tests;
