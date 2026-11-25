use std::sync::atomic::Ordering;

use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::a::region_handle::Generation;

use super::super::fixture::{config, MockBackend, SEGMENTS_BASE};
use super::super::vram::VramManager;
use super::*;

struct Rig {
    backend: Arc<MockBackend>,
    shared: Arc<IndexShared>,
    _vram: Arc<VramManager>,
}

fn rig() -> Rig {
    let backend = MockBackend::single();
    let vram = VramManager::new(0, Arc::clone(&backend) as Arc<dyn GpuBackend>, &config());
    let reserved = vram.reserved().unwrap();
    let shared = IndexShared::new(
            Arc::clone(reserved.allocators()),
            backend.table_sink(0),
            backend.root_slot(0),
        )
        .unwrap();
    Rig {
        backend,
        shared: Arc::new(shared),
        _vram: vram,
    }
}

fn services(rig: &Rig, ordinal: u32, label: &str) -> IndexServices {
    let store = Arc::new(MappingStore::new());
    let eviction = Arc::new(
        Eviction::new(
            label,
            Arc::clone(rig.shared.swap()),
            Arc::clone(&store),
            Arc::clone(rig.shared.allocators()),
            TierBudget::default(),
            TierBudget::default(),
        ),
    );
    IndexServices::new(
        ordinal,
        Arc::clone(&rig.backend) as Arc<dyn GpuBackend>,
        Arc::clone(&rig.shared),
        store,
        eviction,
    )
}

fn segment_handle(rig: &Rig) -> RegionHandle {
    rig.shared
        .allocators()
        .shared(ArenaKind::IndexSegments)
        .unwrap()
        .allocate(256)
        .unwrap()
}

#[test]
fn the_shared_structures_use_the_index_segments_allocator() {
    let rig = rig();
    let allocator = rig.shared.allocators().shared(ArenaKind::IndexSegments).unwrap();
    assert_eq!(allocator.capacity(), config().arenas.get(ArenaKind::IndexSegments));
}

#[test]
fn a_set_without_the_index_segments_allocator_is_rejected() {
    let arenas = Arc::new(ArenaSet::new(ArenaCapacities::default()));
    let empty = AllocatorSet::new(Arc::new(Bookkeeping::new(arenas)), &[]).unwrap();
    let backend = MockBackend::single();
    let err = IndexShared::new(Arc::new(empty), backend.table_sink(0), backend.root_slot(0))
        .err()
        .unwrap();
    assert!(matches!(err, GpuError::Memory(AllocatorError::AllocationFailed)), "{err:?}");
}

#[test]
fn services_forward_to_the_backend_with_their_ordinal() {
    let rig = rig();
    let handle = segment_handle(&rig);
    let s0 = services(&rig, 0, "a");
    let s3 = services(&rig, 3, "b");
    assert_eq!(s0.device_base(&handle), Some(Addr(SEGMENTS_BASE + handle.offset())));
    assert_eq!(
        s3.device_base(&handle),
        Some(Addr(SEGMENTS_BASE + 3 * 0x1_0000_0000 + handle.offset()))
    );
    let event = s0.write_visible(&handle, 8, &[1, 2, 3, 4]).unwrap();
    assert!(event.is_complete().unwrap());
    assert_eq!(rig.backend.read_segments(0, handle.offset() + 8, 4), vec![1, 2, 3, 4]);
    assert_eq!(rig.backend.visible_writes.load(Ordering::SeqCst), 1);
    assert!(s0.write_visible(&handle, MIB_BEYOND, &[1]).is_err());
}

const MIB_BEYOND: u64 = 1 << 40;

#[test]
fn the_kernel_table_goes_through_the_backend() {
    let rig = rig();
    let s = services(&rig, 0, "a");
    let mut lane = Lane::new(LaneConfig::preset(LaneKind::Query, DeviceHandle::Gpu(0)), None);
    let first = s.write_kernel_table(&mut lane, &[9, 9]).unwrap();
    let second = s.write_kernel_table(&mut lane, &[7]).unwrap();
    assert_ne!(first, second);
    assert_eq!(*lock(&rig.backend.tables), vec![vec![9, 9], vec![7]]);
}

#[test]
fn indexes_share_the_table_and_the_arena_but_nothing_else() {
    let rig = rig();
    let a = services(&rig, 0, "a");
    let b = services(&rig, 0, "b");
    assert!(Arc::ptr_eq(a.table_swap(), b.table_swap()));
    assert!(std::ptr::eq(a.arena(), b.arena()));
    assert!(!Arc::ptr_eq(a.structure_lock(), b.structure_lock()));
    assert!(!Arc::ptr_eq(a.mapping_store(), b.mapping_store()));
    assert!(!Arc::ptr_eq(a.manifests(), b.manifests()));
    assert!(!Arc::ptr_eq(a.eviction(), b.eviction()));
    assert!(!Arc::ptr_eq(a.updates(), b.updates()));
    assert!(!Arc::ptr_eq(a.residency(), b.residency()));
}

#[test]
fn handles_of_the_arena_are_current_generation_zero() {
    let rig = rig();
    let handle = segment_handle(&rig);
    assert_eq!(handle.generation(), Generation::INITIAL);
}
