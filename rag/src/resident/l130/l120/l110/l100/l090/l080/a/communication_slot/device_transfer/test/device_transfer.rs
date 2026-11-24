use super::*;
use std::cell::Cell;
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::a::region_handle::Generation;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::a::arenas::ArenaCapacities;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::a::index_contract::{
    Payload, PayloadRef, StampObserver, StampSignal,
};
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::l010::device_lane_event::DeviceHandle;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::l010::layout::DType;
fn id(n: u64) -> EntryId {
    EntryId(n)
}
fn ids(range: std::ops::Range<u64>) -> Vec<EntryId> {
    range.map(EntryId).collect()
}
fn region(offset: u64, len: u64) -> RegionHandle {
    RegionHandle::new(ArenaKind::IndexSegments, offset, len, Generation(0)).unwrap()
}
fn range(first: u64, count: u64, offset: u64, len: u64) -> IndexByteRange {
    IndexByteRange {
        first: id(first),
        count,
        offset,
        len,
    }
}
fn assert_send_sync<T: Send + Sync>() {}
#[test]
fn types_are_send_sync() {
    assert_send_sync::<DeviceTransfer>();
    assert_send_sync::<LoadedSegment>();
    assert_send_sync::<DeviceTransferError>();
    assert_send_sync::<TransferConfig>();
    assert_send_sync::<Arc<dyn IndexArenaSource>>();
    assert_send_sync::<Arc<dyn SegmentSink>>();
}
#[test]
fn error_display_texts() {
    assert_eq!(
        DeviceTransferError::GatherNoProgress.to_string(), "Gather ohne Fortschritt"
    );
    assert_eq!(DeviceTransferError::ArenaExhausted.to_string(), "Arena erschöpft");
    assert_eq!(
        DeviceTransferError::InvalidConfig.to_string(), "invalid transfer config"
    );
    assert_eq!(
        DeviceTransferError::Sink(SinkError("full".into())).to_string(),
        "sink rejected segment: full"
    );
    assert_eq!(
        DeviceTransferError::Backend(BackendError::StaleRegion).to_string(),
        BackendError::StaleRegion.to_string()
    );
    assert_eq!(
        DeviceTransferError::Staging(StagingError::Exhausted).to_string(),
        StagingError::Exhausted.to_string()
    );
    let transfer = TransferError {
        direction: TransferDirection::H2D,
        cause: EventError {
            device: DeviceHandle::Gpu(0),
            cause: "cancelled".into(),
        },
    };
    assert_eq!(
        DeviceTransferError::Transfer(transfer.clone()).to_string(), transfer.to_string()
    );
}
#[test]
fn error_sources() {
    use std::error::Error;
    assert!(DeviceTransferError::GatherNoProgress.source().is_none());
    assert!(DeviceTransferError::Sink(SinkError(String::new())).source().is_some());
    assert!(DeviceTransferError::Staging(StagingError::InvalidSlot).source().is_some());
}
#[test]
fn default_config_is_valid() {
    let config = TransferConfig::default();
    assert_eq!(config.align, 16);
    assert_eq!(check_config(& config), Ok(()));
}
#[test]
fn config_validation() {
    for align in [1u64, 2, 4, 8, 16, 32, 64, 128, 256] {
        assert_eq!(check_config(& TransferConfig { align }), Ok(()), "{align}");
    }
    for align in [0u64, 3, 6, 12, 24, 255, 257, 384, 512, 1024] {
        assert_eq!(
            check_config(& TransferConfig { align }),
            Err(DeviceTransferError::InvalidConfig), "{align}"
        );
    }
}
#[test]
fn align_up_and_place() {
    assert_eq!(align_up(0, 16), Some(0));
    assert_eq!(align_up(1, 16), Some(16));
    assert_eq!(align_up(16, 16), Some(16));
    assert_eq!(align_up(17, 1), Some(17));
    assert_eq!(align_up(u64::MAX, 16), None);
    assert_eq!(place(0, 64, 16, 64), Some(0));
    assert_eq!(place(0, 65, 16, 64), None);
    assert_eq!(place(10, 48, 16, 64), Some(16));
    assert_eq!(place(10, 49, 16, 64), None);
    assert_eq!(place(60, 0, 16, 64), Some(64));
    assert_eq!(place(65, 0, 16, 64), None);
    assert_eq!(place(0, u64::MAX, 16, 64), None);
}
#[test]
fn normalize_sorts_and_dedups() {
    assert_eq!(
        normalize(& [id(5), id(1), id(5), id(3), id(1)]), vec![id(1), id(3), id(5)]
    );
    assert!(normalize(& []).is_empty());
}
#[test]
fn arena_bytes_len() {
    let bytes = [1u8, 2, 3];
    let ordinary = ArenaBytes::Ordinary(&bytes);
    assert_eq!(ordinary.len(), 3);
    assert!(! ordinary.is_empty());
    assert!(ArenaBytes::Ordinary(& []).is_empty());
}
#[test]
fn direct_plan_accepts_dense_aligned_range() {
    let entries = ids(4..12);
    let plan = plan_direct(&[range(4, 8, 256, 8 * 32)], &entries, 16).unwrap();
    assert_eq!(plan.stride, 32);
    assert_eq!(plan.range.offset, 256);
}
#[test]
fn direct_plan_rejections() {
    let entries = ids(4..12);
    let good = range(4, 8, 256, 256);
    assert!(plan_direct(& [good], & entries, 16).is_some());
    assert!(plan_direct(& [], & entries, 16).is_none());
    assert!(plan_direct(& [good, good], & entries, 16).is_none());
    assert!(plan_direct(& [range(5, 8, 256, 256)], & entries, 16).is_none());
    assert!(plan_direct(& [range(4, 9, 256, 288)], & entries, 16).is_none());
    assert!(plan_direct(& [range(4, 7, 256, 224)], & entries, 16).is_none());
    assert!(plan_direct(& [range(4, 8, 256, 0)], & entries, 16).is_none());
    assert!(plan_direct(& [range(4, 0, 256, 256)], & ids(4..4), 16).is_none());
    assert!(plan_direct(& [range(4, 8, 256, 255)], & entries, 16).is_none());
    assert!(plan_direct(& [range(4, 8, 250, 256)], & entries, 16).is_none());
    assert!(plan_direct(& [range(4, 8, 256, 8 * 24)], & entries, 16).is_none());
    assert!(plan_direct(& [range(4, 8, 250, 8 * 24)], & entries, 2).is_some());
    assert!(plan_direct(& [good], & [], 16).is_none());
}
#[test]
fn segment_bounds_and_bindings() {
    let region = region(512, 256);
    let entries = vec![id(2), id(3), id(4)];
    let stamps = vec![Some(Stamp(7)), None, Some(Stamp(9))];
    let segment = build_segment(region, entries.clone(), &stamps, &[0, 32, 64], 96);
    assert_eq!(segment.region, region);
    assert_eq!(segment.entries, entries);
    assert_eq!(segment.bounds, vec![512, 544, 576, 608]);
    assert_eq!(segment.bounds.len(), segment.entries.len() + 1);
    assert_eq!(segment.bounds[0], region.offset());
    assert_eq!(
        segment.bindings, vec![Binding { entry : id(2), slot : 0, stamp : Stamp(7) },
        Binding { entry : id(4), slot : 2, stamp : Stamp(9) },]
    );
}
#[test]
fn segment_with_equal_bounds_for_empty_slot() {
    let stamps = vec![Some(Stamp(1)), Some(Stamp(2))];
    let segment = build_segment(
        region(256, 256),
        vec![id(0), id(1)],
        &stamps,
        &[0, 16],
        16,
    );
    assert_eq!(segment.bounds, vec![256, 272, 272]);
    assert_eq!(segment.bindings.len(), 2);
}
struct Vectors {
    data: HashMap<u64, Vec<u8>>,
    reads: Cell<usize>,
}
impl Vectors {
    fn new(lens: &[(u64, usize)]) -> Vectors {
        Vectors {
            data: lens.iter().map(|&(id, len)| (id, vec![id as u8 + 1; len])).collect(),
            reads: Cell::new(0),
        }
    }
    fn read(&self, entry: EntryId) -> SharedBytes {
        self.reads.set(self.reads.get() + 1);
        SharedBytes::from(self.data[&entry.0].clone())
    }
}
fn items(ids: &[u64], removed: &[u64]) -> Vec<(EntryId, Option<Stamp>)> {
    ids.iter()
        .map(|&n| (id(n), (!removed.contains(&n)).then_some(Stamp(n + 1))))
        .collect()
}
fn pack(
    ring: &StagingRing,
    slot: SlotId,
    items: &[(EntryId, Option<Stamp>)],
    pending: &mut Option<SharedBytes>,
    vectors: &Vectors,
    align: u64,
) -> Result<PackedRun, PackError> {
    pack_run(
        items,
        pending,
        ring.slot_size() as u64,
        align,
        |entry| vectors.read(entry),
        |from, to, bytes| write_slot(ring, slot, from, to, bytes),
    )
}
fn slot_bytes(ring: &StagingRing, slot: SlotId) -> Vec<u8> {
    ring.with_payload(slot, |p| p.to_vec()).unwrap()
}
#[test]
fn pack_fills_slot_in_order_and_keeps_overflow_payload() {
    let ring = StagingRing::new(1, 64).unwrap();
    let slot = ring.acquire().unwrap();
    let vectors = Vectors::new(&[(0, 10), (1, 20), (2, 30)]);
    let all = items(&[0, 1, 2], &[]);
    let mut pending = None;
    let run = pack(&ring, slot, &all, &mut pending, &vectors, 16).unwrap();
    assert_eq!(run.entries, vec![id(0), id(1)]);
    assert_eq!(run.offsets, vec![0, 16]);
    assert_eq!(run.end, 36);
    assert_eq!(run.removed, 0);
    assert_eq!(run.done(), 2);
    assert_eq!(vectors.reads.get(), 3);
    assert_eq!(pending.as_ref().map(| p | p.len()), Some(30));
    let bytes = slot_bytes(&ring, slot);
    assert!(bytes[..10].iter().all(|& b | b == 1));
    assert!(bytes[10..16].iter().all(|& b | b == 0));
    assert!(bytes[16..36].iter().all(|& b | b == 2));
    ring.release(slot).unwrap();
    let slot = ring.acquire().unwrap();
    let run = pack(&ring, slot, &all[2..], &mut pending, &vectors, 16).unwrap();
    assert_eq!(run.entries, vec![id(2)]);
    assert_eq!(run.offsets, vec![0]);
    assert_eq!(run.end, 30);
    assert_eq!(vectors.reads.get(), 3, "the kept payload is not read again");
    assert!(pending.is_none());
}
#[test]
fn pack_zero_fills_padding_of_reused_slot() {
    let ring = StagingRing::new(1, 64).unwrap();
    let slot = ring.acquire().unwrap();
    ring.with_payload(slot, |p| p.fill(0xFF)).unwrap();
    let vectors = Vectors::new(&[(0, 5), (1, 5)]);
    let run = pack(&ring, slot, &items(&[0, 1], &[]), &mut None, &vectors, 16).unwrap();
    assert_eq!(run.offsets, vec![0, 16]);
    let bytes = slot_bytes(&ring, slot);
    assert!(bytes[5..16].iter().all(|& b | b == 0));
    assert!(bytes[21..].iter().all(|& b | b == 0xFF));
}
#[test]
fn pack_skips_removed_entries_without_space() {
    let ring = StagingRing::new(1, 64).unwrap();
    let slot = ring.acquire().unwrap();
    let vectors = Vectors::new(&[(1, 16), (3, 16)]);
    let run = pack(&ring, slot, &items(&[0, 1, 2, 3], &[0, 2]), &mut None, &vectors, 16)
        .unwrap();
    assert_eq!(run.entries, vec![id(1), id(3)]);
    assert_eq!(run.offsets, vec![0, 16]);
    assert_eq!(run.removed, 2);
    assert_eq!(run.done(), 4);
    assert_eq!(run.end, 32);
    assert_eq!(vectors.reads.get(), 2, "removed entries are never read");
}
#[test]
fn pack_only_removed_entries_has_no_data() {
    let ring = StagingRing::new(1, 64).unwrap();
    let slot = ring.acquire().unwrap();
    let vectors = Vectors::new(&[]);
    let run = pack(&ring, slot, &items(&[4, 5], &[4, 5]), &mut None, &vectors, 16)
        .unwrap();
    assert!(run.entries.is_empty());
    assert_eq!(run.removed, 2);
    assert_eq!(run.end, 0);
    assert_eq!(run.done(), 2);
    assert_eq!(vectors.reads.get(), 0);
}
#[test]
fn pack_empty_vector_takes_a_slot_but_no_space() {
    let ring = StagingRing::new(1, 64).unwrap();
    let slot = ring.acquire().unwrap();
    let vectors = Vectors::new(&[(0, 8), (1, 0), (2, 8)]);
    let run = pack(&ring, slot, &items(&[0, 1, 2], &[]), &mut None, &vectors, 16)
        .unwrap();
    assert_eq!(run.entries, vec![id(0), id(1), id(2)]);
    assert_eq!(run.offsets, vec![0, 16, 16]);
    assert_eq!(run.end, 24);
    let segment = build_segment(
        region(256, 256),
        run.entries,
        &run.stamps,
        &run.offsets,
        run.end,
    );
    assert_eq!(segment.bounds, vec![256, 272, 272, 280]);
    assert_eq!(segment.bindings.len(), 3);
}
#[test]
fn pack_only_empty_vectors_has_zero_length() {
    let ring = StagingRing::new(1, 64).unwrap();
    let slot = ring.acquire().unwrap();
    let vectors = Vectors::new(&[(0, 0), (1, 0)]);
    let run = pack(&ring, slot, &items(&[0, 1], &[]), &mut None, &vectors, 16).unwrap();
    assert_eq!(run.entries.len(), 2);
    assert_eq!(run.end, 0);
    assert_eq!(run.done(), 2);
}
#[test]
fn pack_first_entry_too_large_is_exhausted() {
    let ring = StagingRing::new(1, 64).unwrap();
    let slot = ring.acquire().unwrap();
    let vectors = Vectors::new(&[(1, 65)]);
    let result = pack(&ring, slot, &items(&[0, 1], &[0]), &mut None, &vectors, 16);
    assert!(matches!(result, Err(PackError::Exhausted)));
}
#[test]
fn pack_entry_exactly_slot_size_fits() {
    let ring = StagingRing::new(1, 64).unwrap();
    let slot = ring.acquire().unwrap();
    let vectors = Vectors::new(&[(0, 64), (1, 1)]);
    let mut pending = None;
    let run = pack(&ring, slot, &items(&[0, 1], &[]), &mut pending, &vectors, 16)
        .unwrap();
    assert_eq!(run.entries, vec![id(0)]);
    assert_eq!(run.end, 64);
    assert!(pending.is_some());
}
#[test]
fn pack_removed_entry_behind_overflow_is_left_for_next_run() {
    let ring = StagingRing::new(1, 32).unwrap();
    let slot = ring.acquire().unwrap();
    let vectors = Vectors::new(&[(0, 20), (1, 20), (3, 4)]);
    let all = items(&[0, 1, 2, 3], &[2]);
    let mut pending = None;
    let run = pack(&ring, slot, &all, &mut pending, &vectors, 16).unwrap();
    assert_eq!((run.entries.len(), run.removed, run.done()), (1, 0, 1));
    ring.release(slot).unwrap();
    let slot = ring.acquire().unwrap();
    let run = pack(&ring, slot, &all[1..], &mut pending, &vectors, 16).unwrap();
    assert_eq!(run.entries, vec![id(1)]);
    assert_eq!(run.removed, 1);
    assert_eq!(run.done(), 2);
    ring.release(slot).unwrap();
    let slot = ring.acquire().unwrap();
    let run = pack(&ring, slot, &all[3..], &mut pending, &vectors, 16).unwrap();
    assert_eq!(run.entries, vec![id(3)]);
    assert_eq!(run.removed, 0);
}
#[test]
fn pack_alignment_one_is_dense() {
    let ring = StagingRing::new(1, 64).unwrap();
    let slot = ring.acquire().unwrap();
    let vectors = Vectors::new(&[(0, 3), (1, 5)]);
    let run = pack(&ring, slot, &items(&[0, 1], &[]), &mut None, &vectors, 1).unwrap();
    assert_eq!(run.offsets, vec![0, 3]);
    assert_eq!(run.end, 8);
}
#[test]
fn lease_releases_slot_unless_handed_over() {
    let ring = StagingRing::new(2, 16).unwrap();
    let slot = ring.acquire().unwrap();
    drop(SlotLease {
        ring: &ring,
        slot,
        armed: true,
    });
    assert_eq!(ring.slot_state(slot), Some(staging_ring::SlotState::Free));
    let slot = ring.acquire().unwrap();
    let mut lease = SlotLease {
        ring: &ring,
        slot,
        armed: true,
    };
    lease.hand_over();
    drop(lease);
    assert_eq!(ring.slot_state(slot), Some(staging_ring::SlotState::Occupied));
}
const TEST_MIB: u64 = 1024 * 1024;
fn backend() -> Option<Arc<GpuBackend>> {
    let mut caps = ArenaCapacities::default();
    caps.set(ArenaKind::Weights, 8 * TEST_MIB);
    caps.set(ArenaKind::IndexSegments, 8 * TEST_MIB);
    caps.set(ArenaKind::Workspace, 8 * TEST_MIB);
    caps.set(ArenaKind::Staging, 4 * TEST_MIB);
    caps.set(ArenaKind::ParameterBlock, TEST_MIB);
    caps.set(ArenaKind::MappedHost, 2 * TEST_MIB);
    match GpuBackend::new(DeviceHandle::Gpu(0), caps, TEST_MIB) {
        Ok(b) => Some(Arc::new(b)),
        Err(BackendError::DeviceMemoryUnavailable(reason)) => {
            eprintln!("skipped, no device: {reason}");
            None
        }
        Err(e) => panic!("unexpected error: {e}"),
    }
}
/// In-memory index over a contiguous blob; `None` marks a removed entry.
struct MemSource {
    vectors: Vec<Option<Vec<u8>>>,
    blob: Vec<u8>,
    starts: Vec<usize>,
    ranged: bool,
    signal: Arc<StampSignal>,
}
impl MemSource {
    fn new(vectors: Vec<Option<Vec<u8>>>, ranged: bool) -> Arc<MemSource> {
        let mut blob = Vec::new();
        let mut starts = Vec::new();
        for v in &vectors {
            starts.push(blob.len());
            blob.extend_from_slice(v.as_deref().unwrap_or(&[]));
        }
        Arc::new(MemSource {
            vectors,
            blob,
            starts,
            ranged,
            signal: StampSignal::new(Stamp(0)),
        })
    }
    fn uniform(count: usize, len: usize, ranged: bool) -> Arc<MemSource> {
        let vectors = (0..count).map(|i| Some(vec![i as u8 + 1; len])).collect();
        MemSource::new(vectors, ranged)
    }
}
impl Index for MemSource {
    fn dtype(&self) -> DType {
        DType::U8
    }
    fn access_methods(&self) -> &[AccessMethod] {
        if self.ranged { &[AccessMethod::Range] } else { &[] }
    }
    fn stamp(&self, entry: EntryId) -> Option<Stamp> {
        self.vectors.get(entry.0 as usize)?.as_ref().map(|_| Stamp(entry.0 + 1))
    }
    fn stamps(&self) -> Vec<(EntryId, Stamp)> {
        (0..self.vectors.len() as u64)
            .filter_map(|i| self.stamp(id(i)).map(|s| (id(i), s)))
            .collect()
    }
    fn read(&self, entry: EntryId) -> Payload {
        match self.vectors.get(entry.0 as usize) {
            Some(Some(v)) => {
                Payload {
                    vector: SharedBytes::from(v.clone()),
                    meta: SharedBytes::default(),
                }
            }
            _ => Payload::default(),
        }
    }
    fn write(&self, _: EntryId, _: PayloadRef<'_>) -> Result<Stamp, IndexError> {
        Err(IndexError::Backend("read only".into()))
    }
    fn delete(&self, _: EntryId) -> Result<Option<Stamp>, IndexError> {
        Err(IndexError::Backend("read only".into()))
    }
    fn subscribe(&self) -> StampObserver {
        self.signal.observer()
    }
    fn byte_ranges(
        &self,
        first: EntryId,
        last: EntryId,
    ) -> Result<Vec<IndexByteRange>, IndexError> {
        if !self.ranged {
            return Err(IndexError::Unsupported(AccessMethod::Range));
        }
        let (a, b) = (first.0 as usize, last.0 as usize);
        let end = self.starts[b] + self.vectors[b].as_ref().map_or(0, Vec::len);
        Ok(
            vec![
                IndexByteRange { first, count : (b - a + 1) as u64, offset : self
                .starts[a] as u64, len : (end - self.starts[a]) as u64, }
            ],
        )
    }
}
impl IndexArenaSource for MemSource {
    fn index(&self) -> &dyn Index {
        self
    }
    fn range_bytes(&self, range: &IndexByteRange) -> Result<ArenaBytes<'_>, IndexError> {
        let start = range.offset as usize;
        Ok(ArenaBytes::Ordinary(&self.blob[start..start + range.len as usize]))
    }
    fn advice_target(&self) -> Option<Arc<dyn AdviceTarget>> {
        None
    }
}
#[derive(Default)]
struct CollectSink {
    segments: Mutex<Vec<LoadedSegment>>,
    fail: AtomicBool,
}
impl SegmentSink for CollectSink {
    fn accept(&self, segment: LoadedSegment) -> Result<(), SinkError> {
        if self.fail.load(Ordering::SeqCst) {
            return Err(SinkError("refused".into()));
        }
        self.segments.lock().unwrap().push(segment);
        Ok(())
    }
}
struct Fixture {
    backend: Arc<GpuBackend>,
    sink: Arc<CollectSink>,
    transfer: Arc<DeviceTransfer>,
}
fn fixture(source: Arc<MemSource>, slots: usize, slot_size: usize) -> Option<Fixture> {
    let backend = backend()?;
    let sink = Arc::new(CollectSink::default());
    let ring = Arc::new(StagingRing::pinned(&backend, slots, slot_size).unwrap());
    let transfer = Arc::new(
        DeviceTransfer::new(
                source,
                Arc::clone(&backend),
                ring,
                sink.clone(),
                TransferConfig::default(),
            )
            .unwrap(),
    );
    Some(Fixture { backend, sink, transfer })
}
#[test]
fn pinned_ring_slots_hold_zeroed_writable_payload() {
    let Some(b) = backend() else { return };
    let ring = StagingRing::pinned(&b, 2, 64).unwrap();
    assert_eq!((ring.slot_count(), ring.slot_size()), (2, 64));
    let slot = ring.acquire().unwrap();
    assert!(ring.with_payload(slot, | p | p.iter().all(|& v | v == 0)).unwrap());
    ring.with_payload(slot, |p| p[..4].copy_from_slice(&[1, 2, 3, 4])).unwrap();
    assert_eq!(
        ring.with_payload(slot, | p | p[..4].to_vec()).unwrap(), vec![1, 2, 3, 4]
    );
    assert_eq!(StagingRing::pinned(& b, 0, 8).err(), Some(StagingError::InvalidConfig));
}
fn device_bytes(backend: &GpuBackend, segment: &LoadedSegment) -> Vec<u8> {
    let buffer = backend.allocate_pinned(segment.region.len() as usize).unwrap();
    backend.download(&segment.region, &buffer).unwrap().wait().unwrap();
    buffer.as_slice().to_vec()
}
fn check_contents(f: &Fixture, source: &MemSource, segment: &LoadedSegment) {
    let data = device_bytes(&f.backend, segment);
    let base = segment.region.offset();
    for (slot, entry) in segment.entries.iter().enumerate() {
        let expected = source.vectors[entry.0 as usize].as_ref().unwrap();
        let start = (segment.bounds[slot] - base) as usize;
        let end = (segment.bounds[slot + 1] - base) as usize;
        assert!(end - start >= expected.len());
        assert_eq!(& data[start..start + expected.len()], & expected[..]);
    }
}
fn free_all(f: &Fixture) {
    for segment in f.sink.segments.lock().unwrap().drain(..) {
        f.backend.free(&segment.region).unwrap();
    }
}
#[test]
fn new_rejects_invalid_alignment() {
    let Some(backend) = backend() else { return };
    let source = MemSource::uniform(1, 16, false);
    for align in [0u64, 3, 512] {
        let result = DeviceTransfer::new(
            source.clone(),
            Arc::clone(&backend),
            Arc::new(StagingRing::new(1, 16).unwrap()),
            Arc::new(CollectSink::default()),
            TransferConfig { align },
        );
        assert_eq!(result.err(), Some(DeviceTransferError::InvalidConfig));
    }
}
#[test]
fn empty_list_does_nothing() {
    let Some(f) = fixture(MemSource::uniform(4, 32, true), 2, 64) else { return };
    assert_eq!(f.transfer.load_segment_blocking(& []), Ok(TransferReport::default()));
    assert_eq!(f.transfer.load_mapped_blocking(& []), Ok(TransferReport::default()));
    assert!(f.sink.segments.lock().unwrap().is_empty());
}
#[test]
fn direct_copy_of_dense_range() {
    let source = MemSource::uniform(8, 32, true);
    let Some(f) = fixture(source.clone(), 2, 64) else { return };
    let unsorted: Vec<EntryId> = [3u64, 1, 0, 2, 7, 6, 5, 4, 3]
        .iter()
        .map(|&n| id(n))
        .collect();
    let report = f.transfer.load_segment_blocking(&unsorted).unwrap();
    assert_eq!(report, TransferReport { bytes : 256, segments : 1, entries_done : 8 });
    let segments = f.sink.segments.lock().unwrap().clone();
    assert_eq!(segments.len(), 1);
    let segment = &segments[0];
    assert_eq!(segment.region.arena(), ArenaKind::IndexSegments);
    assert_eq!(segment.entries, ids(0..8));
    let base = segment.region.offset();
    let expected: Vec<u64> = (0..=8).map(|i| base + i * 32).collect();
    assert_eq!(segment.bounds, expected);
    assert_eq!(segment.bindings.len(), 8);
    assert_eq!(segment.bindings[5].stamp, Stamp(6));
    check_contents(&f, &source, segment);
    free_all(&f);
}
#[test]
fn gather_when_no_range_access() {
    let source = MemSource::uniform(8, 32, false);
    let Some(f) = fixture(source.clone(), 2, 64) else { return };
    let report = f.transfer.load_segment_blocking(&ids(0..8)).unwrap();
    assert_eq!(report, TransferReport { bytes : 256, segments : 4, entries_done : 8 });
    let segments = f.sink.segments.lock().unwrap().clone();
    assert_eq!(segments.len(), 4);
    for (i, segment) in segments.iter().enumerate() {
        assert_eq!(segment.entries, ids(2 * i as u64..2 * i as u64 + 2));
        assert_eq!(segment.bounds.len(), 3);
        assert_eq!(segment.bounds[1] - segment.bounds[0], 32);
        assert_eq!(segment.bounds[2] - segment.bounds[1], 32);
        check_contents(&f, &source, segment);
    }
    free_all(&f);
}
#[test]
fn gather_skips_removed_entries() {
    let mut vectors: Vec<Option<Vec<u8>>> = (0..8)
        .map(|i| Some(vec![i as u8 + 1; 32]))
        .collect();
    vectors[3] = None;
    let source = MemSource::new(vectors, false);
    let Some(f) = fixture(source.clone(), 2, 64) else { return };
    let report = f.transfer.load_segment_blocking(&ids(0..8)).unwrap();
    assert_eq!(report.entries_done, 8);
    assert_eq!(report.bytes, 7 * 32);
    let segments = f.sink.segments.lock().unwrap().clone();
    let all: Vec<EntryId> = segments.iter().flat_map(|s| s.entries.clone()).collect();
    assert_eq!(all, vec![id(0), id(1), id(2), id(4), id(5), id(6), id(7)]);
    for segment in &segments {
        assert_eq!(segment.bindings.len(), segment.entries.len());
        check_contents(&f, &source, segment);
    }
    free_all(&f);
}
#[test]
fn gather_with_oversized_entry_is_exhausted() {
    let source = MemSource::uniform(2, 32, false);
    let Some(f) = fixture(source, 1, 16) else { return };
    assert_eq!(
        f.transfer.load_segment_blocking(& ids(0..2)),
        Err(DeviceTransferError::ArenaExhausted)
    );
    assert!(f.sink.segments.lock().unwrap().is_empty());
    assert!(f.transfer.ring.acquire().is_ok(), "slot was released");
}
#[test]
fn only_removed_entries_produce_no_segment() {
    let source = MemSource::new(vec![None, None, None], false);
    let Some(f) = fixture(source, 1, 64) else { return };
    let report = f.transfer.load_segment_blocking(&ids(0..3)).unwrap();
    assert_eq!(report, TransferReport { bytes : 0, segments : 0, entries_done : 3 });
    assert!(f.sink.segments.lock().unwrap().is_empty());
    assert!(f.transfer.ring.acquire().is_ok());
}
#[test]
fn sink_error_frees_region() {
    let source = MemSource::uniform(4, 32, true);
    let Some(f) = fixture(source, 2, 64) else { return };
    let before = f.backend.available(ArenaKind::IndexSegments).unwrap();
    f.sink.fail.store(true, Ordering::SeqCst);
    assert_eq!(
        f.transfer.load_segment_blocking(& ids(0..4)),
        Err(DeviceTransferError::Sink(SinkError("refused".into())))
    );
    assert_eq!(f.backend.available(ArenaKind::IndexSegments).unwrap(), before);
}
#[test]
fn mapped_load() {
    let source = MemSource::uniform(3, 20, false);
    let Some(f) = fixture(source.clone(), 2, 64) else { return };
    if f.backend.mapped_device_addr().is_none() {
        return;
    }
    let report = f.transfer.load_mapped_blocking(&ids(0..3)).unwrap();
    assert_eq!(report, TransferReport { bytes : 84, segments : 1, entries_done : 3 });
    let segments = f.sink.segments.lock().unwrap().clone();
    assert_eq!(segments.len(), 1);
    let segment = &segments[0];
    assert_eq!(segment.region.arena(), ArenaKind::MappedHost);
    let base = segment.region.offset();
    assert_eq!(segment.bounds, vec![base, base + 32, base + 64, base + 84]);
    for (slot, entry) in segment.entries.iter().enumerate() {
        let mut buf = vec![0u8; 20];
        f.backend
            .mapped_read(
                &segment.region,
                (segment.bounds[slot] - base) as usize,
                &mut buf,
            )
            .unwrap();
        assert_eq!(buf, source.vectors[entry.0 as usize].clone().unwrap());
    }
    free_all(&f);
}
#[test]
fn mapped_without_data_falls_back_to_gather() {
    let source = MemSource::new(vec![None, None], false);
    let Some(f) = fixture(source, 1, 64) else { return };
    let report = f.transfer.load_mapped_blocking(&ids(0..2)).unwrap();
    assert_eq!(report, TransferReport { bytes : 0, segments : 0, entries_done : 2 });
}
#[tokio::test]
async fn async_variants_run_in_blocking_pool() {
    let source = MemSource::uniform(4, 32, true);
    let Some(f) = fixture(source, 2, 64) else { return };
    let report = f.transfer.load_segment(ids(0..4)).await.unwrap();
    assert_eq!(report.segments, 1);
    assert_eq!(report.entries_done, 4);
    free_all(&f);
    if f.backend.mapped_device_addr().is_some() {
        let report = f.transfer.load_mapped(ids(0..2)).await.unwrap();
        assert_eq!(report.segments, 1);
        free_all(&f);
    }
}
#[tokio::test]
async fn concurrent_loads_get_distinct_slots() {
    let source = MemSource::uniform(16, 32, false);
    let Some(f) = fixture(source.clone(), 3, 64) else { return };
    let a = f.transfer.load_segment(ids(0..8));
    let b = f.transfer.load_segment(ids(8..16));
    let (a, b) = tokio::join!(a, b);
    assert_eq!(a.unwrap().entries_done, 8);
    assert_eq!(b.unwrap().entries_done, 8);
    let segments = f.sink.segments.lock().unwrap().clone();
    let mut all: Vec<EntryId> = segments
        .iter()
        .flat_map(|s| s.entries.clone())
        .collect();
    all.sort();
    assert_eq!(all, ids(0..16));
    for segment in &segments {
        check_contents(&f, &source, segment);
    }
    free_all(&f);
}
#[tokio::test]
async fn more_concurrent_loads_than_slots_all_succeed() {
    let source = MemSource::uniform(24, 32, false);
    let Some(f) = fixture(source.clone(), 1, 64) else { return };
    let loads: Vec<_> = (0..6u64)
        .map(|i| f.transfer.load_segment(ids(4 * i..4 * i + 4)))
        .collect();
    for result in futures::future::join_all(loads).await {
        assert_eq!(result.unwrap().entries_done, 4);
    }
    let segments = f.sink.segments.lock().unwrap().clone();
    let mut all: Vec<EntryId> = segments
        .iter()
        .flat_map(|s| s.entries.clone())
        .collect();
    all.sort();
    assert_eq!(all, ids(0..24));
    for segment in &segments {
        check_contents(&f, &source, segment);
    }
    free_all(&f);
}
