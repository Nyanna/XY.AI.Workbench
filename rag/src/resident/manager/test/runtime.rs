use std::sync::atomic::AtomicBool;
use std::time::Duration;
use parking_lot::Mutex;
use super::*;
use crate::resident::backend::ComputeBackend;
use crate::resident::error::ManagerError;
use crate::resident::index::memory::InMemoryIndex;
use crate::resident::index::{EntryId, Index};
use crate::resident::manager::evict::EvictPolicy;
use crate::resident::manager::residency::{LoadPolicy, ResidencyLoader, UniformGeometry};
use crate::resident::manager::scope::ResolverRegistry;
use crate::resident::manager::slot::RangeSource;
use crate::resident::manager::streamed::{IndexManager, StreamedIndexManager};
use crate::resident::manager::swap::{SegmentTable, TableSink};
use crate::resident::manager::transfer::GatherTransfer;
use crate::resident::manager::upload::{StagedGather, UploadPort};
use crate::resident::memory::error::{MemoryError, TransferDir};
use crate::resident::memory::region::ArenaId;
use crate::resident::memory::{
    ArenaAllocator, ArenaKind, DeviceKind, Region, StagingRing,
};
use crate::resident::subengine::{
    CpuLane, EventHandle, ExecContext, LaneClass, LaneConfig,
};
const ENTRY_BYTES: usize = 100;
struct HostPort {
    lane: CpuLane,
    memory: Mutex<Vec<u8>>,
    fail: AtomicBool,
}
impl UploadPort for HostPort {
    fn upload(&self, src: &[u8], dst: Region) -> Result<EventHandle, MemoryError> {
        if self.fail.load(std::sync::atomic::Ordering::SeqCst) {
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
    fn read(&self, _: crate::resident::kernel::WrittenRange) -> Vec<u8> {
        Vec::new()
    }
}
struct Rig {
    index: Arc<InMemoryIndex>,
    arena: Arc<IndexArena<'static>>,
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
fn rig(arena_bytes: u64) -> Rig {
    let index = Arc::new(InMemoryIndex::new());
    for n in 0..60 {
        index.apply(id(n), &[n as u8; ENTRY_BYTES]);
    }
    let alloc = Arc::new(
        ArenaAllocator::new(ArenaKind::IndexSegment, DeviceKind::Cpu, arena_bytes),
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
        memory: Mutex::new(vec![0; arena_bytes as usize]),
        fail: AtomicBool::new(false),
    });
    let queue = Arc::new(LoadedQueue::new());
    let gather = StagedGather::new(StagingRing::new((0..2).map(|_| vec![0u8; 8192])));
    let transfer = Arc::new(
        GatherTransfer::new(
            index.clone(),
            Arc::new(geometry()),
            gather,
            port,
            arena.clone(),
            queue.clone(),
            256,
        ),
    );
    Rig {
        index,
        arena,
        queue,
        transfer,
    }
}
fn runtime(rig: &Rig, evict: EvictPolicy) -> IndexRuntime<'_, 'static, NullBackend> {
    let loader = ResidencyLoader::new(
        LoadPolicy::default(),
        Arc::new(geometry()),
        rig.transfer.clone(),
    );
    let manager = GpuIndexManager::new(
        StreamedIndexManager::new(Arc::new(NullBackend), "t"),
        ResolverRegistry::with_defaults(),
        loader,
        evict,
    );
    manager.attach_index(rig.index.clone());
    let swap = SwapManager::new(&rig.arena, Box::new(ArenaSink(&rig.arena)), 0);
    IndexRuntime::new(manager, &rig.arena, swap, rig.queue.clone())
}
fn set(ids: impl IntoIterator<Item = u64>) -> EntrySet {
    EntrySet::from_ids(ids.into_iter().map(id))
}
fn short_ttl() -> EvictPolicy {
    EvictPolicy::new(0.9, 0.5, Duration::from_secs(1))
}
#[tokio::test]
async fn ensure_publishes_the_loaded_segment_without_further_calls() {
    let rig = rig(1 << 20);
    let rt = runtime(&rig, short_ttl());
    rt.ensure(set([2, 5]), Access::Regular).await.unwrap();
    assert_eq!(rt.swap().current().segments().len(), 1);
    assert_eq!(rt.manager().segment_count(), 1);
    assert_eq!(rt.manager().pending_updates(), 10);
    assert!(rig.queue.is_empty());
}
#[tokio::test]
async fn slot_completion_binds_the_published_entries() {
    let rig = rig(1 << 20);
    let rt = runtime(&rig, short_ttl());
    rt.ensure(set([2, 5]), Access::Regular).await.unwrap();
    rt.complete_slot(&rt.swap().snapshot(), &[], &[], &NoSource).unwrap();
    assert_eq!(rt.manager().pending_updates(), 0);
    assert_eq!(rt.manager().core().resident_len(), 10);
}
#[tokio::test]
async fn cycle_publishes_reloaded_entries() {
    let rig = rig(1 << 20);
    let rt = runtime(&rig, short_ttl());
    rt.cycle().await.unwrap();
    rt.ensure(set([3]), Access::Regular).await.unwrap();
    rt.complete_slot(&rt.swap().snapshot(), &[], &[], &NoSource).unwrap();
    rig.index.apply(id(3), &[9; ENTRY_BYTES]);
    let outcome = rt.cycle().await.unwrap();
    assert!(! outcome.is_clean());
    assert_eq!(rt.manager().segment_count(), 2);
}
#[tokio::test]
async fn evictor_accounts_every_segment_separately() {
    let rig = rig(1 << 20);
    let rt = runtime(&rig, short_ttl());
    rt.ensure(set([1, 2]), Access::Regular).await.unwrap();
    let first = rt.manager().resident_bytes();
    rt.ensure(set([11, 12]), Access::Regular).await.unwrap();
    assert_eq!(rt.manager().segment_count(), 2);
    assert_eq!(rt.manager().resident_bytes(), 2 * first);
}
#[tokio::test]
async fn idle_segments_leave_table_mapping_and_residency() {
    let rig = rig(1 << 20);
    let rt = runtime(&rig, short_ttl());
    rt.ensure(set([1, 2]), Access::Regular).await.unwrap();
    rt.complete_slot(&rt.swap().snapshot(), &[], &[], &NoSource).unwrap();
    assert_eq!(rt.manager().core().resident_len(), 10);
    let evicted = rt.evict(Instant::now() + Duration::from_secs(60)).unwrap();
    assert_eq!(evicted.len(), 1);
    assert_eq!(rt.swap().current().segments().len(), 0);
    assert_eq!(rt.manager().segment_count(), 0);
    assert_eq!(rt.manager().resident_bytes(), 0);
    assert_eq!(rt.manager().core().resident_len(), 0);
    assert!(rt.reclaim() >= 1);
}
#[tokio::test]
async fn exhausted_arena_evicts_and_retries_once() {
    let rig = rig(12288);
    let rt = runtime(&rig, short_ttl());
    rt.ensure(set(0..20), Access::Regular).await.unwrap();
    rt.complete_slot(&rt.swap().snapshot(), &[], &[], &NoSource).unwrap();
    rt.ensure(set(30..50), Access::Regular).await.unwrap();
    assert_eq!(rt.manager().segment_count(), 1);
    assert_eq!(rt.swap().current().segments().len(), 1);
}
#[test]
fn publish_evict_and_slot_completion_race_without_leaking_regions() {
    const ROUNDS: u64 = 300;
    let rig = rig(1 << 20);
    let rt = runtime(&rig, short_ttl());
    let warm_up = tokio::runtime::Builder::new_current_thread().build().unwrap();
    warm_up.block_on(rt.ensure(set(0..10), Access::Regular)).unwrap();
    rt.evict(Instant::now() + Duration::from_secs(60)).unwrap();
    rt.reclaim();
    let baseline = rig.arena.budget().live;
    let done = AtomicBool::new(false);
    std::thread::scope(|s| {
        s.spawn(|| {
            let tokio = tokio::runtime::Builder::new_current_thread().build().unwrap();
            for round in 0..ROUNDS {
                let base = round % 40;
                tokio
                    .block_on(rt.ensure(set(base..base + 10), Access::Regular))
                    .unwrap();
                rt.evict(Instant::now() + Duration::from_secs(60)).unwrap();
            }
            done.store(true, std::sync::atomic::Ordering::Release);
        });
        s.spawn(|| {
            while !done.load(std::sync::atomic::Ordering::Acquire) {
                rt.complete_slot(&rt.swap().snapshot(), &[], &[], &NoSource).unwrap();
            }
        });
    });
    rt.evict(Instant::now() + Duration::from_secs(60)).unwrap();
    rt.reclaim();
    let budget = rig.arena.budget();
    assert_eq!(rt.manager().segment_count(), 0);
    assert_eq!(rt.swap().current().segments().len(), 0);
    assert_eq!((budget.live, budget.retired), (baseline, 0));
}
