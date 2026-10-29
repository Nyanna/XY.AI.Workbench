use parking_lot::Mutex;
use super::*;
use crate::resident::index::memory::InMemoryIndex;
use crate::resident::memory::region::ArenaId;
use crate::resident::subengine::{CpuLane, ExecContext, LaneClass, LaneConfig};
fn region(len: u64) -> Region {
    Region {
        arena: ArenaId(0),
        offset: 0,
        len,
        gen: 0,
    }
}
struct HostPort {
    lane: CpuLane,
    memory: Mutex<Vec<u8>>,
    copies: Mutex<usize>,
    fail: bool,
}
impl HostPort {
    fn new(bytes: usize) -> Self {
        Self {
            lane: CpuLane::new(LaneClass::Index, LaneConfig::new(region(1))).unwrap(),
            memory: Mutex::new(vec![0; bytes]),
            copies: Mutex::new(0),
            fail: false,
        }
    }
}
impl UploadPort for HostPort {
    fn upload(&self, src: &[u8], dst: Region) -> Result<EventHandle, MemoryError> {
        if self.fail {
            return Err(MemoryError::TransferFailed {
                dir: TransferDir::HostToDevice,
                reason: "injected".into(),
            });
        }
        let at = dst.offset as usize;
        self.memory.lock()[at..at + src.len()].copy_from_slice(src);
        *self.copies.lock() += 1;
        Ok(self.lane.record_event().unwrap())
    }
}
fn index_of(entries: &[(u64, &[u8])]) -> InMemoryIndex {
    let index = InMemoryIndex::new();
    for (id, data) in entries {
        index.apply(EntryId::new(*id), data);
    }
    index
}
fn gatherer(slot_bytes: usize, slots: usize) -> StagedGather<Vec<u8>> {
    StagedGather::new(StagingRing::new((0..slots).map(|_| vec![0u8; slot_bytes])))
}
fn entry_ids(ids: &[u64]) -> Vec<EntryId> {
    ids.iter().map(|&i| EntryId::new(i)).collect()
}
#[test]
fn packs_aligned_and_uploads_once() {
    let index = index_of(&[(1, &[1; 10]), (2, &[2; 20]), (3, &[3; 5])]);
    let gather = gatherer(64, 1);
    let port = HostPort::new(64);
    let prepared = gather.gather(&index, &entry_ids(&[1, 2, 3]), 16).unwrap();
    assert_eq!(prepared.taken(), 3);
    assert_eq!(prepared.bytes(), 53);
    let ranges: Vec<_> = prepared.placements().iter().map(|p| p.range.clone()).collect();
    assert_eq!(ranges, vec![0..10, 16..36, 48..53]);
    let event = prepared.submit(&port, region(64)).unwrap();
    event.synchronize().unwrap();
    assert_eq!(* port.copies.lock(), 1);
    let mem = port.memory.lock();
    assert_eq!(& mem[0..10], & [1; 10]);
    assert_eq!(& mem[16..36], & [2; 20]);
    assert_eq!(& mem[48..53], & [3; 5]);
}
#[test]
fn oversized_input_is_chunked_by_slot_capacity() {
    let index = index_of(&[(1, &[1; 10]), (2, &[2; 20]), (3, &[3; 5])]);
    let gather = gatherer(32, 1);
    let ids = entry_ids(&[1, 2, 3]);
    let first = gather.gather(&index, &ids, 1).unwrap();
    assert_eq!((first.taken(), first.bytes()), (2, 30));
    let taken = first.taken();
    drop(first);
    let second = gather.gather(&index, &ids[taken..], 1).unwrap();
    assert_eq!((second.taken(), second.bytes()), (1, 5));
    assert_eq!(second.placements() [0].entry, EntryId::new(3));
}
#[test]
fn entry_larger_than_a_slot_fails() {
    let index = index_of(&[(1, &[1; 40])]);
    let gather = gatherer(32, 1);
    let err = gather.gather(&index, &entry_ids(&[1]), 1).err().unwrap();
    assert!(
        matches!(err, ManagerError::ArenaExhausted { requested : 40, available : 32 })
    );
    assert!(gather.gather(& index, & [], 1).is_ok());
}
#[test]
fn removed_entries_are_skipped() {
    let index = index_of(&[(1, &[1; 4]), (3, &[3; 4])]);
    let gather = gatherer(64, 1);
    let prepared = gather.gather(&index, &entry_ids(&[1, 2, 3]), 1).unwrap();
    assert_eq!(prepared.taken(), 3);
    let owners: Vec<_> = prepared.placements().iter().map(|p| p.entry).collect();
    assert_eq!(owners, entry_ids(& [1, 3]));
}
#[test]
fn failed_upload_frees_the_slot() {
    let index = index_of(&[(1, &[1; 4])]);
    let gather = gatherer(64, 1);
    let mut port = HostPort::new(64);
    port.fail = true;
    let prepared = gather.gather(&index, &entry_ids(&[1]), 1).unwrap();
    assert!(prepared.submit(& port, region(64)).is_err());
    assert!(gather.gather(& index, & entry_ids(& [1]), 1).is_ok());
}
#[test]
fn dropped_preparation_frees_the_slot() {
    let index = index_of(&[(1, &[1; 4])]);
    let gather = gatherer(64, 1);
    drop(gather.gather(&index, &entry_ids(&[1]), 1).unwrap());
    assert!(gather.gather(& index, & entry_ids(& [1]), 1).is_ok());
}
#[test]
fn slot_is_reused_after_its_transfer_completes() {
    let index = index_of(&[(1, &[1; 4])]);
    let gather = gatherer(64, 1);
    let port = HostPort::new(64);
    for _ in 0..3 {
        let prepared = gather.gather(&index, &entry_ids(&[1]), 1).unwrap();
        prepared.submit(&port, region(64)).unwrap();
    }
    assert_eq!(* port.copies.lock(), 3);
}
#[cfg(feature = "cuda")]
mod device {
    use std::sync::Arc;
    use std::time::Instant;
    use super::*;
    use crate::resident::memory::arena::DeviceMemory;
    use crate::resident::memory::cuda::CudaMemoryManager;
    use crate::resident::memory::{ArenaKind, ArenaSizes};
    use crate::resident::subengine::{CudaLane, DeviceHandle};
    const ENTRY_BYTES: usize = 1536;
    const SEGMENT_BYTES: usize = 16 << 20;
    fn setup() -> (
        Arc<CudaMemoryManager>,
        CudaUploadPort,
        StagedGather<cudarc::driver::PinnedHostSlice<u8>>,
    ) {
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
        let ring = memory.staging_ring(2, SEGMENT_BYTES).unwrap();
        let port = CudaUploadPort::new(memory.clone(), lane);
        (memory, port, StagedGather::new(ring))
    }
    fn segment_index() -> (InMemoryIndex, Vec<u8>) {
        let index = InMemoryIndex::new();
        let count = SEGMENT_BYTES / ENTRY_BYTES;
        let mut flat = Vec::with_capacity(count * ENTRY_BYTES);
        for id in 0..count {
            let data = vec![(id % 251) as u8; ENTRY_BYTES];
            index.apply(EntryId::new(id as u64), &data);
            flat.extend_from_slice(&data);
        }
        (index, flat)
    }
    #[test]
    #[ignore = "requires a CUDA device"]
    fn gather_reaches_the_device_in_one_copy() {
        let (memory, port, gatherer) = setup();
        let (index, _) = segment_index();
        let ids = entry_ids(&[3, 17, 1000, 4096]);
        let dst = memory
            .alloc_in(ArenaKind::IndexSegment, (ids.len() * ENTRY_BYTES) as u64, 256)
            .unwrap();
        let prepared = gatherer.gather(&index, &ids, ENTRY_BYTES as u64).unwrap();
        assert_eq!(prepared.taken(), ids.len());
        let placements = prepared.placements().to_vec();
        prepared.submit(&port, dst).unwrap().synchronize().unwrap();
        let mut out = vec![0u8; ids.len() * ENTRY_BYTES];
        let dev = DeviceHandle::cuda(0).unwrap();
        let ctx = dev.cuda_context().unwrap();
        let stream = ctx.new_stream().unwrap();
        unsafe { memory.download(&stream, dst, &mut out).unwrap() };
        stream.synchronize().unwrap();
        for (placement, id) in placements.iter().zip([3u64, 17, 1000, 4096]) {
            let at = placement.range.start as usize;
            assert!(out[at..at + ENTRY_BYTES].iter().all(| b | * b == (id % 251) as u8));
        }
    }
    #[test]
    #[ignore = "requires a CUDA device; measurement"]
    fn gather_versus_segment_crossover() {
        let (memory, port, gatherer) = setup();
        let (index, flat) = segment_index();
        let count = SEGMENT_BYTES / ENTRY_BYTES;
        let dst = memory
            .alloc_in(ArenaKind::IndexSegment, SEGMENT_BYTES as u64, 256)
            .unwrap();
        let whole = |port: &CudaUploadPort| {
            let t = Instant::now();
            port.upload(&flat, dst).unwrap().synchronize().unwrap();
            t.elapsed().as_secs_f64()
        };
        let mut segment = (0..5).map(|_| whole(&port)).fold(f64::MAX, f64::min);
        segment = segment.min(whole(&port));
        eprintln!("segment {} MiB: {:.3} ms", SEGMENT_BYTES >> 20, segment * 1e3);
        let mut crossover = None;
        for permille in [5u64, 10, 25, 50, 100, 150, 200, 300, 500, 750, 1000] {
            let wanted = (count as u64 * permille / 1000).max(1) as usize;
            let stride = count / wanted;
            let ids: Vec<EntryId> = (0..wanted)
                .map(|i| EntryId::new((i * stride) as u64))
                .collect();
            let mut best = f64::MAX;
            for _ in 0..5 {
                let t = Instant::now();
                let mut rest = &ids[..];
                while !rest.is_empty() {
                    let prepared = gatherer.gather(&index, rest, 256).unwrap();
                    let taken = prepared.taken();
                    let bytes = prepared.bytes();
                    let region = memory
                        .alloc_in(ArenaKind::IndexSegment, bytes, 256)
                        .unwrap();
                    prepared.submit(&port, region).unwrap().synchronize().unwrap();
                    memory.free(region).unwrap();
                    rest = &rest[taken..];
                }
                best = best.min(t.elapsed().as_secs_f64());
            }
            eprintln!("density {:>4} permille: gather {:.3} ms", permille, best * 1e3);
            if crossover.is_none() && best >= segment {
                crossover = Some(permille);
            }
        }
        eprintln!(
            "gather no longer cheaper than the segment from {:?} permille", crossover
        );
    }
}
