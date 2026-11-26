//! Shared test fixture: real shared structures on top of a fake device memory.
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use super::deps::*;
use super::{GpuManager, GpuManagerConfig, GpuServices, LoadedQueue, StructureLock};
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::a::segment_arena::SegmentArenaConfig;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::a::table_swap::{
    HostRootSlot, RootSlot, TableSink,
};
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::a::allocator::AllocatorSet;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::a::bookkeeping::Bookkeeping;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::a::reference_index::ReferenceIndex;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::a::arenas::{
    ArenaCapacities, ArenaSet, Budget, MIB,
};
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::_1::device_lane_event::{
    DeviceHandle, DeviceKind, LaneConfig, LaneKind,
};
use crate::resident::_d::_c::_b::_a::_9::_8::_7::a::manifest::StoreConfig;
use super::super::eviction::TierBudget;
pub const DIM: u32 = 4;
pub const MAX_TOKENS: u32 = 2;
pub const SLOT_BYTES: u64 = (MAX_TOKENS * DIM * 4) as u64;
pub const SEGMENT_BYTES: u64 = 256;
pub const SLOTS_PER_SEGMENT: u32 = (SEGMENT_BYTES / SLOT_BYTES) as u32;
pub const DEVICE_ORIGIN: u64 = 0x1_0000_0000;
pub struct SwitchSink {
    pub fail: AtomicBool,
}
impl TableSink for SwitchSink {
    fn write_visible(
        &self,
        _region: &RegionHandle,
        _bytes: &[u8],
    ) -> Result<(), String> {
        if self.fail.load(Ordering::SeqCst) {
            return Err("table sink down".to_string());
        }
        Ok(())
    }
}
pub struct FakeServices {
    arena: SegmentArena<Arc<Allocator>>,
    swap: Arc<TableSwap>,
    store: Arc<MappingStore>,
    manifests: Arc<ManifestStore>,
    eviction: Arc<Eviction>,
    updates: Arc<UpdateQueue>,
    residency: Arc<Mutex<ResidencyDirectory>>,
    structure: Arc<StructureLock>,
    pub sink: Arc<SwitchSink>,
    pub memory: Mutex<Vec<u8>>,
    pub tables: Mutex<Vec<Vec<u8>>>,
    pub fail_base: AtomicBool,
    pub fail_visible: AtomicBool,
    pub fail_kernel_table: AtomicBool,
}
fn budget() -> Budget {
    Budget {
        free: u64::MAX / 2,
        total: u64::MAX / 2,
        generation_reserve: 0,
    }
}
fn allocator_set(capacity: u64) -> Arc<AllocatorSet> {
    let mut caps = ArenaCapacities::default();
    caps.set(ArenaKind::IndexSegments, capacity);
    let set = Arc::new(ArenaSet::new(caps));
    let arena = set.ensure(ArenaKind::IndexSegments, &budget()).unwrap();
    let bookkeeping = Arc::new(Bookkeeping::new(set));
    Arc::new(AllocatorSet::new(bookkeeping, &[(arena, DeviceKind::Gpu)]).unwrap())
}
impl FakeServices {
    pub fn new(capacity: u64) -> Arc<FakeServices> {
        let allocators = allocator_set(capacity);
        let table_allocators = allocator_set(8 * MIB);
        let sink = Arc::new(SwitchSink {
            fail: AtomicBool::new(false),
        });
        let swap = Arc::new(
            TableSwap::new(
                table_allocators.shared(ArenaKind::IndexSegments).unwrap(),
                Arc::clone(&sink) as Arc<dyn TableSink>,
                Arc::new(HostRootSlot::new()) as Arc<dyn RootSlot>,
            ),
        );
        let store = Arc::new(MappingStore::new());
        let eviction = Arc::new(
            Eviction::new(
                "index",
                Arc::clone(&swap),
                Arc::clone(&store),
                Arc::clone(&allocators),
                TierBudget::new(capacity, 0, 0),
                TierBudget::new(capacity, 0, 0),
            ),
        );
        let arena = SegmentArena::new(
                allocators.shared(ArenaKind::IndexSegments).unwrap(),
                SegmentArenaConfig::default(),
            )
            .unwrap();
        Arc::new(FakeServices {
            arena,
            swap,
            store,
            manifests: Arc::new(ManifestStore::new(StoreConfig::default())),
            eviction,
            updates: Arc::new(UpdateQueue::new()),
            residency: Arc::new(Mutex::new(ResidencyDirectory::new())),
            structure: Arc::new(StructureLock::new()),
            sink,
            memory: Mutex::new(vec![0u8; capacity as usize]),
            tables: Mutex::new(Vec::new()),
            fail_base: AtomicBool::new(false),
            fail_visible: AtomicBool::new(false),
            fail_kernel_table: AtomicBool::new(false),
        })
    }
    /// Bytes of the device memory at an absolute device address.
    pub fn read_device(&self, addr: u64, len: usize) -> Vec<u8> {
        let start = (addr - DEVICE_ORIGIN) as usize;
        lock(&self.memory)[start..start + len].to_vec()
    }
}
impl GpuServices for FakeServices {
    fn arena(&self) -> &SegmentArena<Arc<Allocator>> {
        &self.arena
    }
    fn table_swap(&self) -> &Arc<TableSwap> {
        &self.swap
    }
    fn mapping_store(&self) -> &Arc<MappingStore> {
        &self.store
    }
    fn manifests(&self) -> &Arc<ManifestStore> {
        &self.manifests
    }
    fn eviction(&self) -> &Arc<Eviction> {
        &self.eviction
    }
    fn updates(&self) -> &Arc<UpdateQueue> {
        &self.updates
    }
    fn residency(&self) -> &Arc<Mutex<ResidencyDirectory>> {
        &self.residency
    }
    fn structure_lock(&self) -> &Arc<StructureLock> {
        &self.structure
    }
    fn device_base(&self, handle: &RegionHandle) -> Option<Addr> {
        if self.fail_base.load(Ordering::SeqCst) {
            return None;
        }
        Some(Addr(DEVICE_ORIGIN + handle.offset()))
    }
    fn write_visible(
        &self,
        handle: &RegionHandle,
        offset: u64,
        bytes: &[u8],
    ) -> Result<Event, String> {
        if self.fail_visible.load(Ordering::SeqCst) {
            return Err("device write failed".to_string());
        }
        let start = (handle.offset() + offset) as usize;
        let mut memory = lock(&self.memory);
        let end = start + bytes.len();
        if end > memory.len() {
            return Err("write beyond the device memory".to_string());
        }
        memory[start..end].copy_from_slice(bytes);
        let (event, signal) = Event::cpu();
        signal.signal();
        Ok(event)
    }
    fn write_kernel_table(&self, _lane: &mut Lane, table: &[u8]) -> Result<u64, String> {
        if self.fail_kernel_table.load(Ordering::SeqCst) {
            return Err("kernel table write failed".to_string());
        }
        let mut tables = lock(&self.tables);
        tables.push(table.to_vec());
        Ok(0x2000_0000 + tables.len() as u64 * 0x1000)
    }
}
pub struct FixedIndex(pub Arc<dyn Index>);
impl LazyIndex for FixedIndex {
    fn get(&self) -> Result<Arc<dyn Index>, String> {
        Ok(Arc::clone(&self.0))
    }
}
pub struct MissingIndex;
impl LazyIndex for MissingIndex {
    fn get(&self) -> Result<Arc<dyn Index>, String> {
        Err("index is not available".to_string())
    }
}
/// Hands out a new index on every call; earlier ones are dropped at once.
pub struct FreshIndex;
impl LazyIndex for FreshIndex {
    fn get(&self) -> Result<Arc<dyn Index>, String> {
        Ok(ReferenceIndex::new_shared(DType::F32) as Arc<dyn Index>)
    }
}
pub fn reference_index(dtype: DType) -> Arc<ReferenceIndex> {
    ReferenceIndex::new_shared(dtype)
}
/// A manager over fresh services and the given index source.
pub fn build(index: Arc<dyn LazyIndex>) -> (Arc<FakeServices>, LoadedQueue, GpuManager) {
    let services = FakeServices::new(MIB);
    let queue = LoadedQueue::default();
    let manager = GpuManager::new(
            Arc::clone(&services) as Arc<dyn GpuServices>,
            index,
            Arc::new(ScopeResolverRegistry::with_defaults()),
            None,
            queue.clone(),
            config(),
        )
        .unwrap();
    (services, queue, manager)
}
pub fn config() -> GpuManagerConfig {
    GpuManagerConfig {
        class: SlotClass::custom(MAX_TOKENS).unwrap(),
        dim: DIM,
        dtype: DType::F32,
        segment_bytes: SEGMENT_BYTES,
    }
}
pub fn lane() -> Lane {
    Lane::new(LaneConfig::preset(LaneKind::Query, DeviceHandle::Cpu), None)
}
/// `tokens` rows of `DIM` values `seed, seed + 1, ...` as little-endian f32.
pub fn vector(tokens: usize, seed: f32) -> Vec<u8> {
    (0..tokens * DIM as usize).flat_map(|i| (seed + i as f32).to_le_bytes()).collect()
}
pub fn set(ids: &[u64]) -> EntrySet {
    EntrySet::from_ids(ids.iter().copied())
}
pub struct Fixture {
    pub services: Arc<FakeServices>,
    pub index: Arc<ReferenceIndex>,
    pub queue: LoadedQueue,
    pub manager: GpuManager,
}
impl Fixture {
    pub fn new() -> Fixture {
        Fixture::with_capacity(MIB)
    }
    pub fn with_capacity(capacity: u64) -> Fixture {
        let services = FakeServices::new(capacity);
        let index = ReferenceIndex::new_shared(DType::F32);
        let queue = LoadedQueue::default();
        let manager = GpuManager::new(
                Arc::clone(&services) as Arc<dyn GpuServices>,
                Arc::new(FixedIndex(Arc::clone(&index) as Arc<dyn Index>)),
                Arc::new(ScopeResolverRegistry::with_defaults()),
                None,
                queue.clone(),
                config(),
            )
            .unwrap();
        Fixture {
            services,
            index,
            queue,
            manager,
        }
    }
    /// Writes an entry of `tokens` rows to the index and returns its stamp.
    pub fn put(&self, id: u64, tokens: usize, seed: f32) -> Stamp {
        self.index
            .write(
                EntryId(id),
                PayloadRef {
                    vector: &vector(tokens, seed),
                    meta: b"meta",
                },
            )
            .unwrap()
    }
    /// Applies the pending updates like the Communication-Slot does.
    pub fn apply(&self) -> usize {
        let mut updates = self.manager.slot_updates();
        let directory = lock(self.services.residency());
        updates.apply(&directory).applied
    }
    /// Publishes everything the queue holds, in order.
    pub fn publish_queued(&self) -> Vec<SegmentId> {
        self.queue
            .drain()
            .into_iter()
            .map(|seg| self.manager.publish_segment(seg).unwrap())
            .collect()
    }
    /// Entries 1..=count are put into the index, loaded, published and bound.
    pub fn resident(&self, count: u64) {
        for id in 1..=count {
            self.put(id, 2, id as f32);
        }
        let ids: Vec<u64> = (1..=count).collect();
        self.manager.ensure(&set(&ids)).unwrap();
        self.publish_queued();
        self.apply();
    }
}
impl Fixture {
    pub fn is_valid(&self, slot: SlotRef) -> bool {
        lock(&self.manager.inner.state)
            .records
            .iter()
            .find(|r| r.id == slot.segment)
            .is_some_and(|r| r.states.is_valid(slot.slot))
    }
    /// Content of a published slot as the device sees it.
    pub fn device_bytes(&self, slot: SlotRef) -> Vec<u8> {
        let addr = {
            let state = lock(&self.manager.inner.state);
            let record = state.records.iter().find(|r| r.id == slot.segment).unwrap();
            record.assignment.device_addr(slot.slot).unwrap()
        };
        self.services.read_device(addr.0, SLOT_BYTES as usize)
    }
    /// Commits a slot for `id` in the first published segment, like the kernel does.
    pub fn fill_slot(&self, id: u64) -> SlotRef {
        let state = lock(&self.manager.inner.state);
        let record = state.records.first().unwrap();
        let writer = record.writer.as_ref().unwrap();
        let entry = EntryRef {
            entry: EntryId(id),
            section: 0,
        };
        let reservation = writer.reserve(entry, 1, None).unwrap();
        let (event, signal) = Event::cpu();
        signal.signal();
        writer.commit(&reservation, &event).unwrap();
        SlotRef {
            segment: record.id,
            slot: reservation.slot,
        }
    }
    pub fn stamp_of(&self, id: u64) -> Stamp {
        self.index.stamp(EntryId(id)).unwrap()
    }
}
