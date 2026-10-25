use std::sync::atomic::{AtomicBool, Ordering};
use parking_lot::Mutex;
use super::*;
use crate::resident::backend::ComputeBackend;
use crate::resident::index::memory::InMemoryIndex;
use crate::resident::index::Index;
use crate::resident::manager::flat_index::FlatIndex;
use crate::resident::kernel::WrittenRange;
use crate::resident::manager::evict::EvictPolicy;
use crate::resident::manager::residency::{LoadPolicy, ResidencyLoader, UniformGeometry};
use crate::resident::manager::scope::ResolverRegistry;
use crate::resident::manager::slot::{MappingStore, RangeSource};
use crate::resident::manager::streamed::{
    GpuIndexManager, IndexManager, StreamedIndexManager,
};
use crate::resident::manager::swap::{SegmentTable, SwapManager, TableSink, TableSwap};
use crate::resident::memory::region::ArenaId;
use crate::resident::memory::{ArenaAllocator, ArenaKind, DeviceKind, StagingRing};
use crate::resident::subengine::{CpuLane, ExecContext, LaneClass, LaneConfig};
use crate::resident::subengine::EventHandle;
const ENTRY_BYTES: usize = 100;
const ARENA_BYTES: u64 = 1 << 20;
struct HostPort {
    lane: CpuLane,
    memory: Mutex<Vec<u8>>,
    fail: AtomicBool,
}
impl UploadPort for HostPort {
    fn upload(&self, src: &[u8], dst: Region) -> Result<EventHandle, MemoryError> {
        if self.fail.load(Ordering::SeqCst) {
            return Err(MemoryError::TransferFailed {
                dir: TransferDir::HostToDevice,
                reason: "injected".into(),
            });
        }
        let at = dst.offset as usize;
        self.memory.lock()[at..at + src.len()].copy_from_slice(src);
        Ok(self.lane.record_event().unwrap())
    }
}
struct ArenaSink<'a>(&'a IndexArena<'static>);
impl TableSink for ArenaSink<'_> {
    fn write(&self, table: &SegmentTable) -> Result<Region, ManagerError> {
        self.0.alloc_segment(table.encoded_len().max(256), 256)
    }
}
struct NullBackend;
impl ComputeBackend for NullBackend {
    fn run_batch(&self, input: &[f32]) -> Vec<f32> {
        input.to_vec()
    }
}
struct NoSource;
impl RangeSource for NoSource {
    fn read(&self, _: WrittenRange) -> Vec<u8> {
        Vec::new()
    }
}
struct Fixture {
    index: Arc<InMemoryIndex>,
    alloc: Arc<ArenaAllocator>,
    arena: Arc<IndexArena<'static>>,
    port: Arc<HostPort>,
    queue: Arc<LoadedQueue>,
    transfer: Arc<GatherTransfer<Vec<u8>>>,
}
fn id(n: u64) -> EntryId {
    EntryId::new(n)
}
fn geometry() -> UniformGeometry {
    UniformGeometry {
        entries_per_segment: 10,
        entry_bytes: ENTRY_BYTES as u64,
    }
}
fn fixture(slot_bytes: usize, entries: u64) -> Fixture {
    let index = Arc::new(InMemoryIndex::new());
    for n in 0..entries {
        index.apply(id(n), &[n as u8; ENTRY_BYTES]);
    }
    let alloc = Arc::new(
        ArenaAllocator::new(ArenaKind::IndexSegment, DeviceKind::Cpu, ARENA_BYTES),
    );
    let arena = Arc::new(IndexArena::shared(alloc.clone(), 0));
    let scratch = Region {
        arena: ArenaId(0),
        offset: 0,
        len: 1,
        gen: 0,
    };
    let port = Arc::new(HostPort {
        lane: CpuLane::new(LaneClass::Index, LaneConfig::new(scratch)).unwrap(),
        memory: Mutex::new(vec![0; ARENA_BYTES as usize]),
        fail: AtomicBool::new(false),
    });
    let queue = Arc::new(LoadedQueue::new());
    let gather = StagedGather::new(
        StagingRing::new((0..2).map(|_| vec![0u8; slot_bytes])),
    );
    let transfer = Arc::new(
        GatherTransfer::new(
            index.clone(),
            Arc::new(geometry()),
            gather,
            port.clone(),
            arena.clone(),
            queue.clone(),
            256,
        ),
    );
    Fixture {
        index,
        alloc,
        arena,
        port,
        queue,
        transfer,
    }
}
#[test]
fn gather_uploads_entries_and_reports_the_layout() {
    let f = fixture(4096, 30);
    let bytes = f.transfer.load_gather(&EntrySet::from_ids([id(2), id(5)])).unwrap();
    let mut loaded = f.queue.drain();
    assert_eq!(loaded.len(), 1);
    let seg = loaded.pop().unwrap();
    assert!(bytes > 0);
    assert_eq!(seg.map.bounds(), Some(seg.region.offset..seg.region.offset + bytes));
    assert_eq!(
        seg.binds, vec![(id(2), 0, f.index.stamp(id(2)).unwrap()), (id(5), 1, f.index
        .stamp(id(5)).unwrap()),]
    );
    let memory = f.port.memory.lock();
    for (slot, byte) in [(0u32, 2u8), (1, 5)] {
        let at = seg.map.slot_span(slot).unwrap().start as usize;
        assert!(memory[at..at + ENTRY_BYTES].iter().all(| b | * b == byte));
    }
}
#[test]
fn segments_load_the_entries_of_their_geometry_range() {
    let f = fixture(4096, 30);
    f.transfer.load_segments(&[1]).unwrap();
    let seg = f.queue.drain().pop().unwrap();
    let entries: Vec<_> = seg.binds.iter().map(|b| b.0).collect();
    assert_eq!(entries, (10..20).map(id).collect::< Vec < _ >> ());
}
#[test]
fn removed_entries_get_no_placement() {
    let f = fixture(4096, 30);
    f.transfer.load_gather(&EntrySet::from_ids([id(3), id(99)])).unwrap();
    let seg = f.queue.drain().pop().unwrap();
    assert_eq!(seg.binds.len(), 1);
    assert_eq!(seg.binds[0].0, id(3));
}
#[test]
fn oversized_sets_split_by_staging_capacity() {
    let f = fixture(1024, 30);
    f.transfer.load_gather(&EntrySet::from_ranges([0..10])).unwrap();
    let sizes: Vec<_> = f.queue.drain().iter().map(|s| s.binds.len()).collect();
    assert_eq!(sizes, [4, 4, 2]);
}
#[test]
fn failed_upload_returns_the_region_and_the_staging_slot() {
    let f = fixture(4096, 30);
    let set = EntrySet::from_ids([id(1), id(2)]);
    f.port.fail.store(true, Ordering::SeqCst);
    for _ in 0..3 {
        assert!(matches!(f.transfer.load_gather(& set), Err(ManagerError::Memory(_))));
    }
    assert!(f.queue.is_empty());
    assert_eq!(f.alloc.available(), f.alloc.capacity());
    f.port.fail.store(false, Ordering::SeqCst);
    assert!(f.transfer.load_gather(& set).is_ok());
}
#[test]
fn zero_copy_falls_back_to_the_copy_path() {
    let f = fixture(4096, 30);
    f.transfer.map_zero_copy(&EntrySet::from_ids([id(4)])).unwrap();
    assert_eq!(f.queue.len(), 1);
}
struct HostMapped {
    alloc: ArenaAllocator,
    memory: Mutex<Vec<u8>>,
}
impl HostMapped {
    fn new(bytes: u64) -> Arc<Self> {
        Arc::new(Self {
            alloc: ArenaAllocator::new(ArenaKind::Mapped, DeviceKind::Cpu, bytes),
            memory: Mutex::new(vec![0; bytes as usize]),
        })
    }
}
impl MappedHost for HostMapped {
    fn map_alloc(&self, bytes: u64, align: u64) -> Result<Region, MemoryError> {
        self.alloc.alloc(bytes, align)
    }
    fn map_write(
        &self,
        region: Region,
        offset: u64,
        src: &[u8],
    ) -> Result<(), MemoryError> {
        let at = (region.offset + offset) as usize;
        self.memory.lock()[at..at + src.len()].copy_from_slice(src);
        Ok(())
    }
    fn map_free(&self, region: Region) -> Result<(), MemoryError> {
        self.alloc.free(region)
    }
}
fn mapped_transfer(
    f: &Fixture,
    mapped: Arc<HostMapped>,
    slot_bytes: usize,
) -> GatherTransfer<Vec<u8>> {
    GatherTransfer::new(
            f.index.clone(),
            Arc::new(geometry()),
            StagedGather::new(StagingRing::new((0..2).map(|_| vec![0u8; slot_bytes]))),
            f.port.clone(),
            f.arena.clone(),
            f.queue.clone(),
            256,
        )
        .with_mapped(mapped)
}
#[test]
fn zero_copy_writes_in_place_without_staging_or_upload() {
    let f = fixture(4096, 30);
    let mapped = HostMapped::new(1 << 16);
    let transfer = mapped_transfer(&f, mapped.clone(), 4096);
    let moved = transfer.map_zero_copy(&EntrySet::from_ids([id(4), id(9)])).unwrap();
    assert_eq!(moved, 256 + ENTRY_BYTES as u64);
    let loaded = f.queue.drain();
    assert_eq!(loaded.len(), 1);
    assert_eq!(loaded[0].region.arena, ArenaKind::Mapped.id());
    assert_eq!(loaded[0].binds.len(), 2);
    let base = loaded[0].region.offset as usize;
    let memory = mapped.memory.lock();
    assert!(memory[base..base + ENTRY_BYTES].iter().all(| b | * b == 4));
    assert!(memory[base + 256..base + 256 + ENTRY_BYTES].iter().all(| b | * b == 9));
    assert!(
        f.port.memory.lock().iter().all(| b | * b == 0),
        "nothing went through the copy port"
    );
}
#[test]
fn zero_copy_falls_back_when_the_mapped_arena_is_full() {
    let f = fixture(4096, 30);
    let mapped = HostMapped::new(64);
    let transfer = mapped_transfer(&f, mapped, 4096);
    transfer.map_zero_copy(&EntrySet::from_ids([id(4)])).unwrap();
    let loaded = f.queue.drain();
    assert_eq!(loaded.len(), 1);
    assert_ne!(loaded[0].region.arena, ArenaKind::Mapped.id());
}
#[test]
fn zero_copy_skips_removed_entries() {
    let f = fixture(4096, 3);
    let mapped = HostMapped::new(1 << 12);
    let transfer = mapped_transfer(&f, mapped, 4096);
    let moved = transfer.map_zero_copy(&EntrySet::from_ids([id(50)])).unwrap();
    assert_eq!(moved, 0);
    assert!(f.queue.is_empty());
}
const FLAT_ENTRY: usize = 256;
#[derive(Default)]
struct Hints(Mutex<Vec<(Range<usize>, Hint)>>);
impl AdviceSink for Hints {
    fn advise(&self, range: Range<usize>, hint: Hint) -> std::io::Result<()> {
        self.0.lock().push((range, hint));
        Ok(())
    }
}
struct Flat {
    transfer: GatherTransfer<Vec<u8>>,
    queue: Arc<LoadedQueue>,
    port: Arc<HostPort>,
    hints: Arc<Hints>,
}
fn flat_transfer(index: Arc<FlatIndex>, staging_bytes: usize) -> Flat {
    let alloc = Arc::new(
        ArenaAllocator::new(ArenaKind::IndexSegment, DeviceKind::Cpu, ARENA_BYTES),
    );
    let arena = Arc::new(IndexArena::shared(alloc, 0));
    let scratch = Region {
        arena: ArenaId(0),
        offset: 0,
        len: 1,
        gen: 0,
    };
    let port = Arc::new(HostPort {
        lane: CpuLane::new(LaneClass::Index, LaneConfig::new(scratch)).unwrap(),
        memory: Mutex::new(vec![0; ARENA_BYTES as usize]),
        fail: AtomicBool::new(false),
    });
    let queue = Arc::new(LoadedQueue::new());
    let hints = Arc::new(Hints::default());
    let gather = StagedGather::new(
        StagingRing::new((0..2).map(|_| vec![0u8; staging_bytes])),
    );
    let geometry = UniformGeometry {
        entries_per_segment: 10,
        entry_bytes: FLAT_ENTRY as u64,
    };
    let transfer = GatherTransfer::new(
            index,
            Arc::new(geometry),
            gather,
            port.clone(),
            arena,
            queue.clone(),
            256,
        )
        .with_advice(hints.clone());
    Flat {
        transfer,
        queue,
        port,
        hints,
    }
}
#[test]
fn backed_segments_go_to_the_device_in_one_direct_copy() {
    let flat = flat_transfer(FlatIndex::new(30, FLAT_ENTRY, 0), 16);
    let moved = flat.transfer.load_segments(&[1]).unwrap();
    assert_eq!(moved, 10 * FLAT_ENTRY as u64);
    let seg = flat.queue.drain().pop().unwrap();
    let entries: Vec<_> = seg.binds.iter().map(|b| b.0).collect();
    assert_eq!(entries, (10..20).map(id).collect::< Vec < _ >> ());
    let at = seg.region.offset as usize;
    let memory = flat.port.memory.lock();
    assert_eq!(memory[at], 11);
    assert_eq!(memory[at + 9 * FLAT_ENTRY], 20);
    assert_eq!(
        flat.hints.0.lock().as_slice(), & [(10 * FLAT_ENTRY..20 * FLAT_ENTRY,
        Hint::Sequential)]
    );
}
#[test]
fn backed_segments_with_gaps_fall_back_to_the_gather() {
    let flat = flat_transfer(FlatIndex::new(30, FLAT_ENTRY, FLAT_ENTRY), 4096);
    flat.transfer.load_segments(&[1]).unwrap();
    let seg = flat.queue.drain().pop().unwrap();
    assert_eq!(seg.binds.len(), 10);
    let hints = flat.hints.0.lock();
    assert_eq!(hints.len(), 10);
    assert!(hints.iter().all(| (_, h) | * h == Hint::Random));
}
#[test]
fn unbacked_indexes_never_produce_hints() {
    let f = fixture(4096, 30);
    let hints = Arc::new(Hints::default());
    let transfer = Arc::new(
        GatherTransfer::new(
                f.index.clone(),
                Arc::new(geometry()),
                StagedGather::new(StagingRing::new((0..2).map(|_| vec![0u8; 4096]))),
                f.port.clone(),
                f.arena.clone(),
                f.queue.clone(),
                256,
            )
            .with_advice(hints.clone()),
    );
    transfer.load_segments(&[0]).unwrap();
    assert!(hints.0.lock().is_empty());
}
#[test]
fn published_segment_resolves_kernel_addresses_after_the_next_slot() {
    let f = fixture(4096, 30);
    f.transfer.load_gather(&EntrySet::from_ids([id(2), id(5)])).unwrap();
    let loaded = f.queue.drain().pop().unwrap();
    let span = loaded.map.slot_span(1).unwrap();
    let loader = ResidencyLoader::new(
        LoadPolicy::default(),
        Arc::new(geometry()),
        f.transfer.clone(),
    );
    let core = StreamedIndexManager::new(Arc::new(NullBackend), "t");
    let mgr = GpuIndexManager::new(
        core,
        ResolverRegistry::with_defaults(),
        loader,
        EvictPolicy::vram(),
    );
    mgr.attach_index(f.index.clone());
    let swap = SwapManager::new(&f.arena, Box::new(ArenaSink(&f.arena)), 1);
    let store = MappingStore::new();
    assert_eq!(mgr.publish_loaded(loaded, & swap, & store, &* f.arena).unwrap(), 0);
    assert_eq!(swap.current().segments().len(), 1);
    assert_eq!(mgr.pending_updates(), 2);
    assert_eq!(store.snapshot().resolve(span.start), None);
    mgr.complete_slot(&store, &swap.snapshot(), &[], &[], &NoSource).unwrap();
    assert_eq!(mgr.pending_updates(), 0);
    assert_eq!(mgr.core().resident_len(), 2);
    assert_eq!(store.snapshot().resolve(span.start), Some(id(5)));
}
struct CountingRegions<'a> {
    arena: &'a IndexArena<'static>,
    freed: std::sync::atomic::AtomicUsize,
}
impl RegionSource for CountingRegions<'_> {
    fn alloc_segment(&self, bytes: u64, align: u64) -> Result<Region, ManagerError> {
        self.arena.alloc_segment(bytes, align)
    }
    fn free_segment(&self, region: Region) -> Result<(), ManagerError> {
        self.freed.fetch_add(1, Ordering::SeqCst);
        self.arena.free_segment(region)
    }
}
struct FailingSwap<'a, 'b>(&'a SwapManager<'a, 'b>);
impl TableSwap for FailingSwap<'_, '_> {
    fn build_next(&self) -> SegmentTable {
        self.0.build_next()
    }
    fn publish(&self, _: SegmentTable) -> Result<u64, ManagerError> {
        Err(
            ManagerError::from(crate::resident::error::IndexError::TransferFailed {
                reason: "injected".into(),
            }),
        )
    }
    fn reclaim(&self, upto: u64) -> usize {
        self.0.reclaim(upto)
    }
}
fn manager_for(f: &Fixture) -> GpuIndexManager<NullBackend> {
    let loader = ResidencyLoader::new(
        LoadPolicy::default(),
        Arc::new(geometry()),
        f.transfer.clone(),
    );
    let mgr = GpuIndexManager::new(
        StreamedIndexManager::new(Arc::new(NullBackend), "t"),
        ResolverRegistry::with_defaults(),
        loader,
        EvictPolicy::vram(),
    );
    mgr.attach_index(f.index.clone());
    mgr
}
#[test]
fn overlapping_segment_is_rejected_and_its_region_is_returned() {
    let f = fixture(4096, 30);
    f.transfer.load_gather(&EntrySet::from_ids([id(2)])).unwrap();
    let first = f.queue.drain().pop().unwrap();
    let start = first.map.slot_span(0).unwrap().start;
    let mgr = manager_for(&f);
    let swap = SwapManager::new(&f.arena, Box::new(ArenaSink(&f.arena)), 1);
    let store = MappingStore::new();
    let regions = CountingRegions {
        arena: &f.arena,
        freed: Default::default(),
    };
    mgr.publish_loaded(first, &swap, &store, &regions).unwrap();
    let pending = mgr.pending_updates();
    let clash = LoadedSegment {
        region: regions.alloc_segment(256, 256).unwrap(),
        map: SlotMap::fixed(start, 16, 1),
        binds: vec![(id(9), 0, crate ::resident::index::Stamp::from_raw(1))],
    };
    assert!(mgr.publish_loaded(clash, & swap, & store, & regions).is_err());
    assert_eq!(regions.freed.load(Ordering::SeqCst), 1);
    assert_eq!(swap.current().segments().len(), 1);
    assert_eq!(mgr.pending_updates(), pending);
}
#[test]
fn failed_swap_publish_returns_the_region_and_queues_nothing() {
    let f = fixture(4096, 30);
    f.transfer.load_gather(&EntrySet::from_ids([id(2)])).unwrap();
    let loaded = f.queue.drain().pop().unwrap();
    let mgr = manager_for(&f);
    let swap = SwapManager::new(&f.arena, Box::new(ArenaSink(&f.arena)), 1);
    let store = MappingStore::new();
    let regions = CountingRegions {
        arena: &f.arena,
        freed: Default::default(),
    };
    let res = mgr.publish_loaded(loaded, &FailingSwap(&swap), &store, &regions);
    assert!(res.is_err());
    assert_eq!(regions.freed.load(Ordering::SeqCst), 1);
    assert_eq!(swap.current().segments().len(), 0);
    assert_eq!(mgr.pending_updates(), 0);
    assert_eq!(store.snapshot().epoch(), 0);
}
#[cfg(feature = "cuda")]
mod device {
    use super::*;
    use crate::resident::manager::upload::CudaUploadPort;
    use crate::resident::memory::cuda::CudaMemoryManager;
    use crate::resident::memory::arena::DeviceMemory;
    use crate::resident::memory::ArenaSizes;
    use crate::resident::subengine::{CudaLane, DeviceHandle};
    const BYTES: usize = 1536;
    struct MemRegions(Arc<CudaMemoryManager>);
    impl RegionSource for MemRegions {
        fn alloc_segment(&self, bytes: u64, align: u64) -> Result<Region, ManagerError> {
            Ok(self.0.alloc_in(ArenaKind::IndexSegment, bytes, align)?)
        }
        fn free_segment(&self, region: Region) -> Result<(), ManagerError> {
            Ok(self.0.free(region)?)
        }
    }
    #[test]
    #[ignore = "requires a CUDA device"]
    fn transfer_reaches_the_device_and_reads_back() {
        let dev = DeviceHandle::cuda(0).unwrap();
        let sizes = ArenaSizes {
            weights: 1 << 20,
            index_segment: 40 << 20,
            workspace: 1 << 20,
            staging: 0,
            param_block: 4096,
            mapped: 0,
        };
        let memory = Arc::new(CudaMemoryManager::new(&dev, sizes, 0, 2).unwrap());
        let scratch = memory.alloc_in(ArenaKind::ParamBlock, 256, 8).unwrap();
        let ctx = dev.cuda_context().unwrap();
        let lane = CudaLane::new(&ctx, LaneClass::Index, LaneConfig::new(scratch))
            .unwrap();
        let gather = StagedGather::new(memory.staging_ring(2, 16 << 10).unwrap());
        let port = Arc::new(CudaUploadPort::new(memory.clone(), lane));
        let index = Arc::new(InMemoryIndex::new());
        for n in 0..64u64 {
            index.apply(id(n), &vec![(n % 251) as u8; BYTES]);
        }
        let queue = Arc::new(LoadedQueue::new());
        let transfer = GatherTransfer::new(
            index.clone(),
            Arc::new(UniformGeometry {
                entries_per_segment: 32,
                entry_bytes: BYTES as u64,
            }),
            gather,
            port,
            Arc::new(MemRegions(memory.clone())),
            queue.clone(),
            256,
        );
        transfer.load_gather(&EntrySet::from_ids([id(3), id(17), id(40)])).unwrap();
        transfer.load_segments(&[1]).unwrap();
        let stream = ctx.new_stream().unwrap();
        let mut checked = 0;
        for seg in queue.drain() {
            let mut out = vec![0u8; seg.region.len as usize];
            unsafe { memory.download(&stream, seg.region, &mut out).unwrap() };
            stream.synchronize().unwrap();
            for (entry, slot, _) in &seg.binds {
                let at = (seg.map.slot_span(*slot).unwrap().start - seg.region.offset)
                    as usize;
                let expect = (entry.get() % 251) as u8;
                assert!(out[at..at + BYTES].iter().all(| b | * b == expect));
                checked += 1;
            }
            memory.free(seg.region).unwrap();
        }
        assert!(checked >= 3 + 32);
    }
    #[test]
    #[ignore = "requires a CUDA device"]
    fn zero_copy_segment_is_readable_through_the_device_address() {
        let dev = DeviceHandle::cuda(0).unwrap();
        let sizes = ArenaSizes {
            weights: 1 << 20,
            index_segment: 4 << 20,
            workspace: 1 << 20,
            staging: 0,
            param_block: 4096,
            mapped: 1 << 20,
        };
        let memory = Arc::new(CudaMemoryManager::new(&dev, sizes, 0, 2).unwrap());
        let scratch = memory.alloc_in(ArenaKind::ParamBlock, 256, 8).unwrap();
        let ctx = dev.cuda_context().unwrap();
        let lane = CudaLane::new(&ctx, LaneClass::Index, LaneConfig::new(scratch))
            .unwrap();
        let gather = StagedGather::new(memory.staging_ring(2, 16 << 10).unwrap());
        let port = Arc::new(CudaUploadPort::new(memory.clone(), lane));
        let index = Arc::new(InMemoryIndex::new());
        for n in 0..8u64 {
            index.apply(id(n), &vec![(n + 1) as u8; BYTES]);
        }
        let queue = Arc::new(LoadedQueue::new());
        let transfer = GatherTransfer::new(
                index,
                Arc::new(UniformGeometry {
                    entries_per_segment: 8,
                    entry_bytes: BYTES as u64,
                }),
                gather,
                port,
                Arc::new(MemRegions(memory.clone())),
                queue.clone(),
                256,
            )
            .with_mapped(memory.clone());
        transfer.map_zero_copy(&EntrySet::from_ids([id(2), id(5)])).unwrap();
        let segments = queue.drain();
        assert_eq!(segments.len(), 1);
        let seg = &segments[0];
        assert_eq!(seg.region.arena, ArenaKind::Mapped.id());
        let device = memory.mapped_device_ptr(seg.region).unwrap();
        ctx.bind_to_thread().unwrap();
        for (entry, slot, _) in &seg.binds {
            let at = seg.map.slot_span(*slot).unwrap().start - seg.region.offset;
            let mut out = vec![0u8; BYTES];
            unsafe {
                cudarc::driver::result::memcpy_dtoh_sync(&mut out, device + at).unwrap();
            }
            let expect = (entry.get() + 1) as u8;
            assert!(
                out.iter().all(| b | * b == expect),
                "entry {entry:?} differs on the device"
            );
        }
        memory.free(seg.region).unwrap();
    }
}
