//! These tests run on the real card. Without a CUDA device they fail instead of skipping.
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc;
use std::time::{Duration, Instant};
use super::super::gpu_subengine::fixture::{config, shared, MIB};
use super::super::gpu_subengine::{
    GpuConfig, GpuDeviceManager, GpuDeviceManagerConfig, GpuError, GpuSubengine,
};
use super::*;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::_1::capability_model::{axes, values};
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::_1::device_lane_event::{DeviceHandle, LaneConfig, LaneKind};
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::_1::layout::DType;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::_1::queue::Queue;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::a::index_contract::{EntryId, Index, PayloadRef};
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::a::reference_index::ReferenceIndex;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::a::region_handle::Generation;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::a::weight_source::WeightFile;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::a::resource_requirements::{
    ExecutionEnvironment, HasResourceRequirements, IndexId, ResourceKind,
    ResourceRequirements,
};
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::a::slot_layout::SlotClass;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::a::resource_lifecycle::{
    InitRequest, LifecycleConfig, Lease2, MemoryClass, ResourceLoader, ResourceRegistry,
};
use crate::resident::_d::_c::_b::_a::_9::_8::_7::a::notification_topology::NotificationTopology;
use crate::resident::_d::_c::_b::_a::_9::a::gpu_completion::GpuCompletion;
use crate::resident::_d::_c::_b::_a::_9::a::lane_batching::{
    BatchKey, BatchLimit, BatchMeta, BatchPayload, BatchRunner, ExecBatch, LaneResource,
    LaneResourceConfig, Launch,
};
use crate::resident::_d::_c::_b::_a::_9::a::manager_lifecycle::IndexManager;
use crate::resident::_d::_c::_b::_a::_9::a::residency_control::IndexSpec;
use crate::resident::_d::_c::_b::_a::_9::_8::subengine_loop::ResourceExec;
use crate::resident::_d::_c::_b::_a::a::planner::Subengine;
static COUNTER: AtomicUsize = AtomicUsize::new(0);
fn real() -> Arc<CudaBackend> {
    let backend = Arc::new(CudaBackend::new());
    let devices = backend.devices().expect("CUDA driver is available");
    assert!(! devices.is_empty(), "no CUDA device found");
    backend
}
fn retained() -> (Arc<CudaBackend>, GpuDeviceInfo) {
    let backend = real();
    let info = backend.devices().unwrap().remove(0);
    backend.retain_primary_context(info.ordinal).unwrap();
    (backend, info)
}
fn arena(kind: ArenaKind, capacity: u64) -> Arena {
    Arena { kind, capacity }
}
fn as_dyn(backend: &Arc<CudaBackend>) -> Arc<dyn GpuBackend3> {
    Arc::clone(backend) as Arc<dyn GpuBackend3>
}
#[test]
fn the_real_card_is_enumerated() {
    let backend = real();
    let devices = backend.devices().unwrap();
    let card = &devices[0];
    assert_eq!(card.ordinal, 0);
    assert!(! card.name.is_empty());
    assert!(card.compute_capability.major >= 5, "{card:?}");
    assert!(card.multiprocessors > 0);
    assert!(card.total_memory > 1024 * MIB);
}
#[test]
fn the_primary_context_is_retained_idempotently() {
    let backend = real();
    let info = backend.devices().unwrap().remove(0);
    assert!(backend.probe(0).compute_capability().is_none());
    assert!(backend.create_stream(0, Priority::Normal).is_err());
    assert!(backend.memory_query(0).query().is_err());
    backend.retain_primary_context(0).unwrap();
    backend.retain_primary_context(0).unwrap();
    let probe = backend.probe(0);
    assert_eq!(probe.compute_capability(), Some(info.compute_capability));
    assert_eq!(probe.device_name(), Some(info.name));
    assert!(backend.retain_primary_context(99).is_err());
}
#[test]
fn free_memory_is_queried_from_the_driver() {
    let (backend, info) = retained();
    let mem = backend.memory_query(0).query().unwrap();
    assert!(mem.free > 0 && mem.free <= mem.total);
    assert_eq!(mem.total, info.total_memory);
}
#[test]
fn streams_record_events_that_complete() {
    let (backend, _) = retained();
    for priority in [Priority::High, Priority::Normal, Priority::Low] {
        let stream = backend.create_stream(0, priority).unwrap();
        let lane = Lane::new(
            LaneConfig::preset(LaneKind::Query, DeviceHandle::Gpu(0)),
            Some(stream),
        );
        let event = lane.record_event().unwrap();
        event.wait().unwrap();
        assert!(event.is_complete().unwrap());
        assert_eq!(event.device(), DeviceHandle::Gpu(0));
    }
}
#[test]
fn the_stream_calls_back_when_its_work_is_done() {
    let (backend, _) = retained();
    let stream = backend.create_stream(0, Priority::High).unwrap();
    let (tx, rx) = mpsc::channel();
    let registered = stream
        .notify_on_complete(
            Box::new(move || {
                let _ = tx.send(std::thread::current().id());
            }),
        )
        .unwrap();
    assert!(registered);
    let ran_on = rx.recv_timeout(Duration::from_secs(10)).expect("callback did not run");
    assert_ne!(ran_on, std::thread::current().id());
}
#[test]
fn device_arenas_are_real_zeroed_vram() {
    let (backend, _) = retained();
    let before = backend.memory_query(0).query().unwrap().free;
    let region = backend.reserve_arena(0, arena(ArenaKind::Weights, 256 * MIB)).unwrap();
    let during = backend.memory_query(0).query().unwrap().free;
    assert!(during + 200 * MIB <= before, "before {before}, during {during}");
    assert_eq!(region.len() as u64, 256 * MIB);
    let mut probe = vec![0xAAu8; 4096];
    region.read(1000, &mut probe).unwrap();
    assert!(probe.iter().all(| b | * b == 0));
    let data: Vec<u8> = (0..5000u32).map(|i| (i % 251) as u8).collect();
    region.write(12_345, &data).unwrap();
    let mut back = vec![0u8; data.len()];
    region.read(12_345, &mut back).unwrap();
    assert_eq!(back, data);
    let last = region.len();
    assert!(region.write(last - 1, & [1, 2]).is_err());
    assert!(region.read(last, & mut [0u8; 1]).is_err());
    drop(region);
    let after = backend.memory_query(0).query().unwrap().free;
    assert!(after + 100 * MIB >= before, "before {before}, after {after}");
}
#[test]
fn host_arenas_are_pinned_memory() {
    let (backend, _) = retained();
    for kind in [ArenaKind::Staging, ArenaKind::MappedHost] {
        let region = backend.reserve_arena(0, arena(kind, MIB)).unwrap();
        let mut zero = [1u8; 16];
        region.read(0, &mut zero).unwrap();
        assert_eq!(zero, [0u8; 16]);
        region.write(100, b"pinned").unwrap();
        let mut back = [0u8; 6];
        region.read(100, &mut back).unwrap();
        assert_eq!(& back, b"pinned");
        assert!(region.write(MIB as usize - 2, & [1, 2, 3]).is_err());
    }
    let empty = backend.reserve_arena(0, arena(ArenaKind::Staging, 0)).unwrap();
    assert!(empty.is_empty());
    let empty = backend.reserve_arena(0, arena(ArenaKind::Workspace, 0)).unwrap();
    assert!(empty.is_empty());
}
#[test]
fn a_write_becomes_visible_and_its_event_completes() {
    let (backend, _) = retained();
    let region = backend.reserve_arena(0, arena(ArenaKind::IndexSegments, MIB)).unwrap();
    let handle = RegionHandle::new(
            ArenaKind::IndexSegments,
            256,
            1024,
            Generation::INITIAL,
        )
        .unwrap();
    let bytes: Vec<u8> = (0..200u32).map(|i| i as u8).collect();
    let event = backend.write_visible(0, &handle, 16, &bytes).unwrap();
    event.wait().unwrap();
    assert!(event.is_complete().unwrap());
    let mut back = vec![0u8; bytes.len()];
    region.read(256 + 16, &mut back).unwrap();
    assert_eq!(back, bytes);
    assert!(backend.write_visible(0, & handle, 1000, & [0u8; 100]).is_err());
    let other = RegionHandle::new(ArenaKind::Weights, 0, 64, Generation::INITIAL)
        .unwrap();
    assert!(backend.write_visible(0, & other, 0, & [1]).is_err());
    let empty = backend.write_visible(0, &handle, 0, &[]).unwrap();
    empty.wait().unwrap();
}
#[test]
fn device_addresses_follow_the_region_offsets() {
    let (backend, _) = retained();
    assert!(
        backend.device_base(0, & RegionHandle::new(ArenaKind::IndexSegments, 0, 16,
        Generation::INITIAL).unwrap()).is_none()
    );
    let _region = backend
        .reserve_arena(0, arena(ArenaKind::IndexSegments, MIB))
        .unwrap();
    let at = |offset: u64| {
        let handle = RegionHandle::new(
                ArenaKind::IndexSegments,
                offset,
                256,
                Generation::INITIAL,
            )
            .unwrap();
        backend.device_base(0, &handle).unwrap().0
    };
    let base = at(0);
    assert_ne!(base, 0);
    assert_eq!(at(4096), base + 4096);
    let beyond = RegionHandle::new(
            ArenaKind::IndexSegments,
            MIB,
            256,
            Generation::INITIAL,
        )
        .unwrap();
    assert!(backend.device_base(0, & beyond).is_none());
    let other = RegionHandle::new(ArenaKind::Weights, 0, 64, Generation::INITIAL)
        .unwrap();
    assert!(backend.device_base(0, & other).is_none());
}
#[test]
fn kernel_tables_alternate_between_two_halves() {
    let (backend, _) = retained();
    let region = backend
        .reserve_arena(0, arena(ArenaKind::ParameterBlock, 4096))
        .unwrap();
    let mut lane = Lane::new(
        LaneConfig::preset(LaneKind::Update, DeviceHandle::Gpu(0)),
        None,
    );
    let a = vec![1u8; 100];
    let b = vec![2u8; 300];
    let first = backend.write_kernel_table(0, &mut lane, &a).unwrap();
    let second = backend.write_kernel_table(0, &mut lane, &b).unwrap();
    assert_eq!(second - first, 2048);
    let mut back = vec![0u8; 300];
    region.read(0, &mut back[..100]).unwrap();
    assert_eq!(& back[..100], & a[..]);
    region.read(2048, &mut back).unwrap();
    assert_eq!(back, b);
    let third = backend.write_kernel_table(0, &mut lane, &[3u8; 8]).unwrap();
    assert_eq!(third, first);
    let mut again = [0u8; 8];
    region.read(0, &mut again).unwrap();
    assert_eq!(again, [3u8; 8]);
    assert!(backend.write_kernel_table(0, & mut lane, & vec![0u8; 2049]).is_err());
}
#[test]
fn the_table_sink_writes_into_the_segment_arena() {
    let (backend, _) = retained();
    let region = backend.reserve_arena(0, arena(ArenaKind::IndexSegments, MIB)).unwrap();
    let sink = backend.table_sink(0);
    let handle = RegionHandle::new(
            ArenaKind::IndexSegments,
            512,
            64,
            Generation::INITIAL,
        )
        .unwrap();
    sink.write_visible(&handle, &[7u8; 64]).unwrap();
    let mut back = [0u8; 64];
    region.read(512, &mut back).unwrap();
    assert_eq!(back, [7u8; 64]);
    let beyond = RegionHandle::new(
            ArenaKind::IndexSegments,
            MIB - 8,
            8,
            Generation::INITIAL,
        )
        .unwrap();
    assert!(sink.write_visible(& beyond, & [0u8; 16]).is_err());
}
#[test]
fn the_root_slot_is_device_mapped_host_memory() {
    let (backend, _) = retained();
    let first = backend.root_slot(0);
    let second = backend.root_slot(0);
    first.store_release(0xDEAD_BEEF);
    assert_eq!(second.load_acquire(), 0xDEAD_BEEF);
    assert_ne!(backend.root_device_address(0).unwrap(), 0);
    assert!(backend.root_device_address(5).is_none());
}
#[test]
fn an_unretained_device_fails_every_write() {
    let backend = real();
    let handle = RegionHandle::new(ArenaKind::IndexSegments, 0, 8, Generation::INITIAL)
        .unwrap();
    assert!(backend.table_sink(0).write_visible(& handle, & [1]).is_err());
    assert!(backend.write_visible(0, & handle, 0, & [1]).is_err());
    assert!(backend.reserve_arena(0, arena(ArenaKind::Weights, 16)).is_err());
}
fn real_manager() -> (Arc<CudaBackend>, GpuDeviceManager) {
    let backend = real();
    let manager = GpuDeviceManager::new(
            as_dyn(&backend),
            shared(),
            GpuDeviceManagerConfig {
                devices: None,
                gpu: config(),
            },
        )
        .unwrap();
    (backend, manager)
}
fn real_subengine() -> (Arc<CudaBackend>, Arc<GpuSubengine>) {
    let (backend, manager) = real_manager();
    let sub = manager.subengine(0).expect("device 0 is usable");
    (backend, sub)
}
#[test]
fn the_device_manager_builds_a_subengine_for_the_real_card() {
    let (_backend, manager) = real_manager();
    assert!(! manager.devices().is_empty());
    let sub = manager.subengine(0).unwrap();
    assert_eq!(sub.name(), "gpu:0");
    assert_eq!(sub.compute_units(), manager.devices() [0].multiprocessors);
    let caps = &sub.detected().capabilities;
    assert!(caps.contains(& axes::operands(), & values::fp32()));
    let cc = manager.devices()[0].compute_capability;
    let has_half = (cc.major, cc.minor) >= (5, 3);
    assert_eq!(caps.contains(& axes::operands(), & values::fp16()), has_half);
    assert_eq!(sub.compute_dtype(), if has_half { DType::F16 } else { DType::F32 });
    assert!(! sub.vram().arenas_ready());
    assert!(! sub.core().is_active());
}
#[test]
fn activation_reserves_real_arenas_and_creates_five_real_lanes() {
    let (backend, sub) = real_subengine();
    let probe = RegionHandle::new(ArenaKind::IndexSegments, 0, 16, Generation::INITIAL)
        .unwrap();
    assert!(backend.device_base(0, & probe).is_none());
    sub.activate().unwrap();
    sub.activate().unwrap();
    assert!(sub.core().is_active());
    let caps = config().arenas;
    for kind in ArenaKind::ALL {
        let region = sub.vram().arena_region(kind).unwrap();
        assert_eq!(region.len() as u64, caps.get(kind), "{kind:?}");
    }
    assert_ne!(backend.device_base(0, & probe).unwrap().0, 0);
    for kind in [
        LaneKind::Query,
        LaneKind::Insert,
        LaneKind::Update,
        LaneKind::CopyH2D,
        LaneKind::CopyD2H,
    ] {
        let event = sub
            .core()
            .with_lane(kind, |lane| lane.record_event().unwrap())
            .unwrap();
        event.wait().unwrap();
        assert_eq!(event.device(), DeviceHandle::Gpu(0));
    }
}
#[test]
fn an_oversized_configuration_is_rejected_by_the_real_budget() {
    let backend = real();
    backend.retain_primary_context(0).unwrap();
    let info = backend.devices().unwrap().remove(0);
    let mut cfg = config();
    cfg.arenas.set(ArenaKind::Weights, info.total_memory + MIB);
    let sub = GpuSubengine::new(info, as_dyn(&backend), shared(), cfg).unwrap();
    let err = sub.activate().unwrap_err();
    assert!(err.contains("Budget"), "{err}");
    assert!(! sub.vram().arenas_ready());
}
fn write_weights(path: &std::path::Path) {
    let tensors: [(&str, usize); 3] = [("a", 100), ("b", 10), ("c", 64)];
    let mut header = String::from("{");
    let mut data: Vec<u8> = Vec::new();
    for (i, (name, count)) in tensors.iter().enumerate() {
        let begin = data.len();
        for _ in 0..*count {
            data.extend_from_slice(&((i + 1) as f32).to_le_bytes());
        }
        if i > 0 {
            header.push(',');
        }
        header
            .push_str(
                &format!(
                    "\"{name}\":{{\"dtype\":\"F32\",\"shape\":[{count}],\"data_offsets\":[{begin},{}]}}",
                    data.len()
                ),
            );
    }
    header.push('}');
    while header.len() % 8 != 0 {
        header.push(' ');
    }
    let mut file = Vec::new();
    file.extend_from_slice(&(header.len() as u64).to_le_bytes());
    file.extend_from_slice(header.as_bytes());
    file.extend_from_slice(&data);
    std::fs::write(path, file).unwrap();
}
#[test]
fn weights_are_copied_into_vram_and_freed_with_the_set() {
    let n = COUNTER.fetch_add(1, Ordering::SeqCst);
    let path = std::env::temp_dir()
        .join(format!("cuda_backend_{}_{n}.safetensors", std::process::id()));
    write_weights(&path);
    let file = WeightFile::open(&path).unwrap();
    let (_backend, sub) = real_subengine();
    let set = sub.vram().stage_weights(&file, &|_| true).unwrap();
    assert_eq!(set.bytes(), 1024);
    assert_eq!(sub.vram().weights_used(), 1024);
    let region = sub.vram().arena_region(ArenaKind::Weights).unwrap();
    for (name, expected) in [("a", 1.0f32), ("b", 2.0), ("c", 3.0)] {
        let tensor = set.tensors().get(name).unwrap();
        let mut first = [0u8; 4];
        region.read(tensor.offset_bytes(), &mut first).unwrap();
        assert_eq!(f32::from_le_bytes(first), expected, "{name}");
        assert_eq!(tensor.offset_bytes() % 256, 0);
    }
    drop(set);
    assert_eq!(sub.vram().weights_used(), 0);
    let _ = std::fs::remove_file(path);
}
fn index_spec() -> IndexSpec {
    IndexSpec {
        class: SlotClass::custom(2).unwrap(),
        dim: 4,
        dtype: DType::F32,
        segment_bytes: 1024,
    }
}
fn vector(id: u64) -> Vec<u8> {
    (0..8).flat_map(|i| (id as f32 * 10.0 + i as f32).to_le_bytes()).collect()
}
#[test]
fn index_entries_arrive_in_the_vram_of_the_card() {
    let (_backend, sub) = real_subengine();
    sub.activate().unwrap();
    let id = IndexId::new("docs");
    let manager = Arc::new(IndexManager::new(id.clone()));
    let index = ReferenceIndex::new_shared(DType::F32);
    assert!(manager.attach(Arc::clone(& index) as Arc < dyn Index >));
    let memory = sub.attach_index(id.clone(), manager, index_spec()).unwrap();
    let ids: Vec<EntryId> = (1..=6).map(EntryId).collect();
    for entry in &ids {
        let bytes = vector(entry.0);
        index
            .write(
                *entry,
                PayloadRef {
                    vector: &bytes,
                    meta: b"m",
                },
            )
            .unwrap();
    }
    let report = sub.ensure_entries(&id, &ids).unwrap();
    assert_eq!(report.loaded, ids);
    assert!(report.failed.is_empty());
    assert!(report.bytes_moved > 0);
    let region = sub.vram().arena_region(ArenaKind::IndexSegments).unwrap();
    let mut arena = vec![0u8; region.len()];
    region.read(0, &mut arena).unwrap();
    for entry in &ids {
        let bytes = vector(entry.0);
        assert!(
            arena.windows(bytes.len()).any(| w | w == bytes.as_slice()), "entry {}",
            entry.0
        );
    }
    assert!(memory.gpu.segment_count() >= 1);
}
#[test]
fn the_kernel_table_of_an_index_is_written_to_vram() {
    let (_backend, sub) = real_subengine();
    sub.activate().unwrap();
    let id = IndexId::new("docs");
    let manager = Arc::new(IndexManager::new(id.clone()));
    let index = ReferenceIndex::new_shared(DType::F32);
    assert!(manager.attach(Arc::clone(& index) as Arc < dyn Index >));
    let memory = sub.attach_index(id.clone(), manager, index_spec()).unwrap();
    for entry in 1..=3u64 {
        let bytes = vector(entry);
        index
            .write(
                EntryId(entry),
                PayloadRef {
                    vector: &bytes,
                    meta: b"m",
                },
            )
            .unwrap();
    }
    sub.ensure_entries(&id, &[EntryId(1), EntryId(2), EntryId(3)]).unwrap();
    let table = sub
        .core()
        .with_lane(LaneKind::Update, |lane| memory.gpu.sync_to_vram(lane))
        .unwrap()
        .unwrap();
    assert!(table.segment_count >= 1);
    assert_ne!(table.segments_addr, 0);
    let region = sub.vram().arena_region(ArenaKind::ParameterBlock).unwrap();
    let mut head = vec![0u8; 32];
    region.read(0, &mut head).unwrap();
    assert!(head.iter().any(| b | * b != 0), "the table is empty");
}
struct Job;
impl BatchPayload for Job {
    fn batch_key(&self) -> Option<BatchKey> {
        None
    }
    fn sequences(&self) -> &[Vec<u32>] {
        &[]
    }
}
struct Meta {
    requirements: ResourceRequirements,
}
impl BatchMeta for Meta {
    fn batch_key(&self) -> Option<BatchKey> {
        None
    }
    fn seq_lengths(&self) -> &[usize] {
        &[]
    }
}
impl HasResourceRequirements for Meta {
    fn resource_requirements(&self) -> &ResourceRequirements {
        &self.requirements
    }
}
#[derive(Default)]
struct Recorder {
    runs: AtomicUsize,
    collects: AtomicUsize,
    streams: AtomicUsize,
}
impl BatchRunner<Job> for Recorder {
    fn run(&self, lane: &mut Lane, _batch: &mut ExecBatch<Job>) -> Launch {
        self.runs.fetch_add(1, Ordering::SeqCst);
        if lane.stream().is_some() {
            self.streams.fetch_add(1, Ordering::SeqCst);
        }
        Launch::Pending
    }
    fn collect(
        &self,
        _lane: &mut Lane,
        _batch: ExecBatch<Job>,
        result: Result<(), crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::_1::device_lane_event::EventError>,
    ) {
        assert!(result.is_ok());
        self.collects.fetch_add(1, Ordering::SeqCst);
    }
}
type Res = LaneResource<Meta, Job>;
struct OnceLoader(Mutex<Option<Arc<Res>>>);
impl ResourceLoader<Res> for OnceLoader {
    fn load(&self) -> Result<Res, String> {
        let arc = lock(&self.0).take().ok_or("loaded twice")?;
        Arc::try_unwrap(arc).map_err(|_| "resource is shared".to_string())
    }
}
#[test]
fn a_batch_runs_on_real_streams_and_finishes_through_the_stream_callback() {
    let (_backend, sub) = real_subengine();
    let woken = Arc::new(AtomicUsize::new(0));
    let w = Arc::clone(&woken);
    let topology = NotificationTopology::new(
        1,
        Arc::new(move || {
            w.fetch_add(1, Ordering::SeqCst);
        }),
    );
    let notifier = topology.notifier(0).unwrap();
    let completion = Arc::new(GpuCompletion::new(notifier.clone()));
    let own: Arc<Queue<Meta, Job, (), ()>> = Arc::new(Queue::new());
    let runner = Arc::new(Recorder::default());
    let resource = sub
        .build_lane_resource(
            LaneResourceConfig {
                own: Arc::clone(&own),
                shared: None,
                filter: Arc::new(|_: &Meta| true),
                completion: Some(Arc::clone(&completion)),
                limit: BatchLimit::Tasks(4),
                notifier,
                kind: ResourceKind::Scorer,
                device: DeviceHandle::Gpu(0),
            },
            Arc::clone(&runner) as Arc<dyn BatchRunner<Job>>,
        )
        .unwrap();
    assert_eq!(resource.lane_count(), 2);
    let registry = Arc::new(
        ResourceRegistry::new(
            LifecycleConfig::new(
                    Duration::from_secs(60),
                    Duration::from_secs(120),
                    Duration::from_secs(5),
                )
                .unwrap(),
        ),
    );
    registry
        .register(
            "lanes".to_string(),
            MemoryClass::Ram,
            Arc::new(OnceLoader(Mutex::new(Some(resource)))),
        )
        .unwrap();
    let now = Instant::now();
    let InitRequest::Started(ticket) = registry.request_init("lanes", 1, now) else {
        panic!("load not started");
    };
    ticket.run(now).unwrap();
    let lease = |registry: &ResourceRegistry<Res>| -> Lease2<Res> {
        match registry.request_init("lanes", 2, Instant::now()) {
            InitRequest::Ready(lease) => lease,
            _ => panic!("not ready"),
        }
    };
    let meta = Meta {
        requirements: ResourceRequirements::new(
            ResourceKind::Scorer,
            ExecutionEnvironment::Any,
        ),
    };
    let task = own.push(meta, Job);
    let claim = own.take().expect("queue is empty");
    let held = lease(&registry);
    let resource = Arc::clone(held.handle());
    resource.execute(claim, held);
    assert_eq!(runner.runs.load(Ordering::SeqCst), 1);
    assert_eq!(runner.streams.load(Ordering::SeqCst), 1);
    let deadline = Instant::now() + Duration::from_secs(10);
    let finished = loop {
        let mut done = completion.poll();
        if let Some(lane) = done.pop() {
            break lane;
        }
        assert!(Instant::now() < deadline, "the lane did not finish");
        std::thread::sleep(Duration::from_millis(5));
    };
    assert!(resource.on_finished(finished).is_ok());
    assert_eq!(runner.collects.load(Ordering::SeqCst), 1);
    assert_eq!(resource.idle_lanes(), 2);
    assert!(woken.load(Ordering::SeqCst) >= 1);
    assert_eq!(task.state(), crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::_1::queue::State2::Succeeded);
}
#[test]
fn lane_resources_get_streams_of_their_own() {
    let (_backend, sub) = real_subengine();
    let topology = NotificationTopology::new(1, Arc::new(|| {}));
    let notifier = topology.notifier(0).unwrap();
    let build = |device: DeviceHandle| {
        sub.build_lane_resource(
            LaneResourceConfig {
                own: Arc::new(Queue::<Meta, Job, (), ()>::new()),
                shared: None,
                filter: Arc::new(|_: &Meta| true),
                completion: Some(Arc::new(GpuCompletion::new(notifier.clone()))),
                limit: BatchLimit::Tasks(1),
                notifier: notifier.clone(),
                kind: ResourceKind::Scorer,
                device,
            },
            Arc::new(Recorder::default()) as Arc<dyn BatchRunner<Job>>,
        )
    };
    assert!(matches!(build(DeviceHandle::Cpu), Err(GpuError::InvalidConfig(_))));
    let resource = build(DeviceHandle::Gpu(0)).unwrap();
    assert_eq!(resource.lane_count(), GpuConfig::default().worker_lanes);
    assert!(! sub.core().is_active());
}
