//! Shared test fixture: a mock CUDA backend over host memory.
use std::collections::{HashMap, HashSet};
use std::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, Weak};
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::a::library_adapter::library_kernel_source;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::a::arenas::MemInfo;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::a::memory_access::{
    AccessError, RegionBackend,
};
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::a::table_swap::HostRootSlot;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::a::kernel_register::{
    CodeImage, CodeKind, CompileRequest, CompilerError, KernelCompiler, KernelFunction,
    LoadFailure, ModuleLoader,
};
use super::backend::{GpuBackend3, GpuDeviceInfo};
use super::config::{GpuConfig, GpuShared};
use super::deps::*;
use super::subengine::GpuSubengine;
pub const MIB: u64 = 1 << 20;
pub const SEGMENTS_BASE: u64 = 0x7000_0000;
#[derive(Debug)]
struct MemRegion {
    bytes: Mutex<Vec<u8>>,
    live: Arc<AtomicUsize>,
}
impl RegionBackend for MemRegion {
    fn len(&self) -> usize {
        lock(&self.bytes).len()
    }
    fn read_raw(&self, offset: usize, dst: &mut [u8]) -> Result<(), AccessError> {
        dst.copy_from_slice(&lock(&self.bytes)[offset..offset + dst.len()]);
        Ok(())
    }
    fn write_raw(&self, offset: usize, src: &[u8]) -> Result<(), AccessError> {
        lock(&self.bytes)[offset..offset + src.len()].copy_from_slice(src);
        Ok(())
    }
}
impl Drop for MemRegion {
    fn drop(&mut self) {
        self.live.fetch_sub(1, Ordering::SeqCst);
    }
}
struct MockProbe {
    capability: Option<ComputeCapability>,
}
impl GpuProbe for MockProbe {
    fn compute_capability(&self) -> Option<ComputeCapability> {
        self.capability
    }
    fn device_name(&self) -> Option<String> {
        Some("mock gpu".to_string())
    }
}
#[derive(Debug)]
struct MockStream;
impl StreamBackend for MockStream {
    fn record_event(
        &self,
    ) -> Result<
        Box<
            dyn crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::_1::device_lane_event::GpuEventBackend,
        >,
        String,
    > {
        Err("mock stream records no events".to_string())
    }
}
struct MockQuery {
    free: Arc<AtomicU64>,
    total: u64,
}
impl MemoryQuery for MockQuery {
    fn query(&self) -> Result<MemInfo, String> {
        Ok(MemInfo {
            free: self.free.load(Ordering::SeqCst),
            total: self.total,
        })
    }
}
struct NullSink;
impl TableSink for NullSink {
    fn write_visible(
        &self,
        _region: &RegionHandle,
        _bytes: &[u8],
    ) -> Result<(), String> {
        Ok(())
    }
}
pub fn device(ordinal: u32) -> GpuDeviceInfo {
    GpuDeviceInfo {
        ordinal,
        name: format!("mock gpu {ordinal}"),
        compute_capability: ComputeCapability {
            major: 7,
            minor: 5,
        },
        multiprocessors: 22,
        total_memory: 6 * 1024 * MIB,
    }
}
/// Backend over host memory that records every call.
pub struct MockBackend {
    pub devices: Mutex<Vec<GpuDeviceInfo>>,
    pub fail_devices: AtomicBool,
    pub retained: Mutex<Vec<u32>>,
    pub fail_retain: Mutex<HashSet<u32>>,
    pub no_context: Mutex<HashSet<u32>>,
    pub compute_capability: Mutex<ComputeCapability>,
    pub streams: Mutex<Vec<(u32, Priority)>>,
    pub streams_before_failure: AtomicUsize,
    pub free: Arc<AtomicU64>,
    pub reserved: Mutex<Vec<(u32, ArenaKind)>>,
    pub fail_reserve: Mutex<Option<ArenaKind>>,
    /// The named arena gets a region of 16 bytes only.
    pub shrink: Mutex<Option<ArenaKind>>,
    pub live: Arc<AtomicUsize>,
    pub tables: Mutex<Vec<Vec<u8>>>,
    pub visible_writes: AtomicUsize,
    memory: Mutex<HashMap<(u32, ArenaKind), Weak<MemRegion>>>,
}
impl MockBackend {
    pub fn new(ordinals: &[u32]) -> Arc<MockBackend> {
        Arc::new(MockBackend {
            devices: Mutex::new(ordinals.iter().map(|o| device(*o)).collect()),
            fail_devices: AtomicBool::new(false),
            retained: Mutex::new(Vec::new()),
            fail_retain: Mutex::new(HashSet::new()),
            no_context: Mutex::new(HashSet::new()),
            compute_capability: Mutex::new(ComputeCapability {
                major: 7,
                minor: 5,
            }),
            streams: Mutex::new(Vec::new()),
            streams_before_failure: AtomicUsize::new(usize::MAX),
            free: Arc::new(AtomicU64::new(64 * MIB)),
            reserved: Mutex::new(Vec::new()),
            fail_reserve: Mutex::new(None),
            shrink: Mutex::new(None),
            live: Arc::new(AtomicUsize::new(0)),
            tables: Mutex::new(Vec::new()),
            visible_writes: AtomicUsize::new(0),
            memory: Mutex::new(HashMap::new()),
        })
    }
    pub fn single() -> Arc<MockBackend> {
        MockBackend::new(&[0])
    }
    pub fn retain_count(&self, ordinal: u32) -> usize {
        lock(&self.retained).iter().filter(|o| **o == ordinal).count()
    }
    pub fn reserve_count(&self, kind: ArenaKind) -> usize {
        lock(&self.reserved).iter().filter(|(_, k)| *k == kind).count()
    }
    pub fn read_segments(&self, ordinal: u32, offset: u64, len: usize) -> Vec<u8> {
        let region = lock(&self.memory)
            .get(&(ordinal, ArenaKind::IndexSegments))
            .and_then(Weak::upgrade)
            .expect("the IndexSegments arena is reserved");
        let mut out = vec![0u8; len];
        region.read_raw(offset as usize, &mut out).unwrap();
        out
    }
}
impl GpuBackend3 for MockBackend {
    fn devices(&self) -> Result<Vec<GpuDeviceInfo>, String> {
        if self.fail_devices.load(Ordering::SeqCst) {
            return Err("enumeration failed".to_string());
        }
        Ok(lock(&self.devices).clone())
    }
    fn retain_primary_context(&self, ordinal: u32) -> Result<(), String> {
        lock(&self.retained).push(ordinal);
        if lock(&self.fail_retain).contains(&ordinal) {
            return Err(format!("context of device {ordinal} is busy"));
        }
        Ok(())
    }
    fn probe(&self, ordinal: u32) -> Arc<dyn GpuProbe> {
        let capability = if lock(&self.no_context).contains(&ordinal) {
            None
        } else {
            Some(*lock(&self.compute_capability))
        };
        Arc::new(MockProbe { capability })
    }
    fn create_stream(
        &self,
        ordinal: u32,
        priority: Priority,
    ) -> Result<Box<dyn StreamBackend>, String> {
        let mut streams = lock(&self.streams);
        if streams.len() >= self.streams_before_failure.load(Ordering::SeqCst) {
            return Err("stream limit reached".to_string());
        }
        streams.push((ordinal, priority));
        Ok(Box::new(MockStream))
    }
    fn memory_query(&self, _ordinal: u32) -> Arc<dyn MemoryQuery> {
        Arc::new(MockQuery {
            free: Arc::clone(&self.free),
            total: 6 * 1024 * MIB,
        })
    }
    fn reserve_arena(&self, ordinal: u32, arena: Arena) -> Result<AccessRegion, String> {
        if *lock(&self.fail_reserve) == Some(arena.kind) {
            return Err(format!("out of memory for {:?}", arena.kind));
        }
        lock(&self.reserved).push((ordinal, arena.kind));
        self.live.fetch_add(1, Ordering::SeqCst);
        let len = if *lock(&self.shrink) == Some(arena.kind) {
            16
        } else {
            arena.capacity as usize
        };
        let region = Arc::new(MemRegion {
            bytes: Mutex::new(vec![0u8; len]),
            live: Arc::clone(&self.live),
        });
        lock(&self.memory).insert((ordinal, arena.kind), Arc::downgrade(&region));
        Ok(AccessRegion::new(region))
    }
    fn device_base(&self, ordinal: u32, handle: &RegionHandle) -> Option<Addr> {
        Some(Addr(SEGMENTS_BASE + u64::from(ordinal) * 0x1_0000_0000 + handle.offset()))
    }
    fn write_visible(
        &self,
        ordinal: u32,
        handle: &RegionHandle,
        offset: u64,
        bytes: &[u8],
    ) -> Result<Event, String> {
        let region = lock(&self.memory)
            .get(&(ordinal, ArenaKind::IndexSegments))
            .and_then(Weak::upgrade)
            .ok_or_else(|| "the IndexSegments arena is not reserved".to_string())?;
        let start = (handle.offset() + offset) as usize;
        if start + bytes.len() > region.len() {
            return Err("write beyond the arena".to_string());
        }
        region.write_raw(start, bytes).map_err(|e| e.to_string())?;
        self.visible_writes.fetch_add(1, Ordering::SeqCst);
        let (event, signal) = Event::cpu();
        signal.signal();
        Ok(event)
    }
    fn write_kernel_table(
        &self,
        _ordinal: u32,
        _lane: &mut Lane,
        table: &[u8],
    ) -> Result<u64, String> {
        let mut tables = lock(&self.tables);
        tables.push(table.to_vec());
        Ok(0x2000_0000 + tables.len() as u64 * 0x1000)
    }
    fn table_sink(&self, _ordinal: u32) -> Arc<dyn TableSink> {
        Arc::new(NullSink)
    }
    fn root_slot(&self, _ordinal: u32) -> Arc<dyn RootSlot> {
        Arc::new(HostRootSlot::new())
    }
}
struct StubCompiler;
impl KernelCompiler for StubCompiler {
    fn compile(&self, _req: &CompileRequest) -> Result<CodeImage, CompilerError> {
        Ok(CodeImage {
            kind: CodeKind::Ptx,
            bytes: vec![0],
        })
    }
}
#[derive(Debug)]
struct StubFunction;
impl KernelFunction for StubFunction {
    fn symbol(&self) -> &str {
        "stub"
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
struct StubLoader;
impl ModuleLoader for StubLoader {
    fn device_capability(&self) -> Option<ComputeCapability> {
        None
    }
    fn load(
        &self,
        _image: &CodeImage,
        _symbol: &str,
    ) -> Result<Arc<dyn KernelFunction>, LoadFailure> {
        Ok(Arc::new(StubFunction))
    }
}
pub fn shared_with(registry: ModelRegistry) -> GpuShared {
    GpuShared {
        kernels: Arc::new(
            KernelRegister::new(
                DeviceHandle::Gpu(0),
                Arc::new(StubCompiler),
                Arc::new(StubLoader),
                library_kernel_source(),
            ),
        ),
        registry: Arc::new(registry),
        registrations: Arc::new(QueueRegistrations::new()),
        resolver: Arc::new(ScopeResolverRegistry::with_defaults()),
        cell_selector: None,
    }
}
pub fn shared() -> GpuShared {
    shared_with(ModelRegistry::with_builtin())
}
/// Small arenas that fit the mock device.
pub fn config() -> GpuConfig {
    let mut cfg = GpuConfig::default();
    cfg.arenas.set(ArenaKind::Weights, MIB);
    cfg.arenas.set(ArenaKind::IndexSegments, 2 * MIB);
    cfg.arenas.set(ArenaKind::Workspace, 64 * 1024);
    cfg.arenas.set(ArenaKind::Staging, 64 * 1024);
    cfg.arenas.set(ArenaKind::ParameterBlock, 64 * 1024);
    cfg.arenas.set(ArenaKind::MappedHost, 64 * 1024);
    cfg.headroom = 0;
    cfg
}
pub fn subengine_with(backend: &Arc<MockBackend>, cfg: GpuConfig) -> Arc<GpuSubengine> {
    GpuSubengine::new(
        device(0),
        Arc::clone(backend) as Arc<dyn GpuBackend3>,
        shared(),
        cfg,
        )
        .unwrap()
}
pub fn subengine(backend: &Arc<MockBackend>) -> Arc<GpuSubengine> {
    subengine_with(backend, config())
}
