use super::*;

use std::sync::atomic::AtomicBool;
use std::thread;
use std::time::Duration;

use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050 as l050;
use l050::l040::l030::a::bookkeeping::Bookkeeping;
use l050::l040::l030::l020::a::arenas::{ArenaCapacities, ArenaSet, Budget};
use l050::l040::l030::l020::a::memory_access::{AccessError, RegionBackend};
use l050::l040::l030::l020::l010::device_lane_event::DeviceKind;

// ---- Fakes ----

#[derive(Default)]
struct Journal(Mutex<Vec<String>>);

impl Journal {
    fn push(&self, entry: String) {
        lock(&self.0).push(entry);
    }

    fn take(&self) -> Vec<String> {
        std::mem::take(&mut *lock(&self.0))
    }
}

struct FakeSink {
    journal: Arc<Journal>,
    fail: AtomicBool,
    panic_next: AtomicBool,
    written: Mutex<HashMap<u64, Vec<u8>>>,
    regions: Mutex<Vec<Region>>,
}

impl FakeSink {
    fn bytes_at(&self, offset: u64) -> Option<Vec<u8>> {
        lock(&self.written).get(&offset).cloned()
    }

    fn regions(&self) -> Vec<Region> {
        lock(&self.regions).clone()
    }
}

impl TableSink for FakeSink {
    fn write_visible(&self, region: &Region, bytes: &[u8]) -> Result<(), String> {
        if self.panic_next.swap(false, Ordering::SeqCst) {
            panic!("sink panic");
        }
        if self.fail.load(Ordering::SeqCst) {
            return Err("boom".to_string());
        }
        self.journal.push("write".to_string());
        lock(&self.written).insert(region.offset, bytes.to_vec());
        lock(&self.regions).push(*region);
        Ok(())
    }
}

struct FakeRoot {
    inner: HostRootSlot,
    journal: Arc<Journal>,
}

impl RootSlot for FakeRoot {
    fn store_release(&self, value: u64) {
        self.journal.push(format!("store:{value}"));
        self.inner.store_release(value);
    }

    fn load_acquire(&self) -> u64 {
        self.inner.load_acquire()
    }
}

struct Fixture {
    swap: TableSwap,
    sink: Arc<FakeSink>,
    root: Arc<FakeRoot>,
    alloc: Arc<Allocator>,
    journal: Arc<Journal>,
}

fn make_allocator(capacity: u64) -> Arc<Allocator> {
    let mut caps = ArenaCapacities::default();
    caps.set(ArenaKind::IndexSegments, capacity);
    let set = Arc::new(ArenaSet::new(caps));
    let budget = Budget {
        free: u64::MAX / 2,
        total: u64::MAX / 2,
        generation_reserve: 0,
    };
    let arena = set.ensure(ArenaKind::IndexSegments, &budget).unwrap();
    let bookkeeping = Arc::new(Bookkeeping::new(set));
    Arc::new(Allocator::new(arena, DeviceKind::Gpu, bookkeeping))
}

fn fixture(capacity: u64) -> Fixture {
    let journal = Arc::new(Journal::default());
    let sink = Arc::new(FakeSink {
        journal: Arc::clone(&journal),
        fail: AtomicBool::new(false),
        panic_next: AtomicBool::new(false),
        written: Mutex::new(HashMap::new()),
        regions: Mutex::new(Vec::new()),
    });
    let root = Arc::new(FakeRoot {
        inner: HostRootSlot::new(),
        journal: Arc::clone(&journal),
    });
    let alloc = make_allocator(capacity);
    let swap = TableSwap::new(
        Arc::clone(&alloc),
        Arc::clone(&sink) as Arc<dyn TableSink>,
        Arc::clone(&root) as Arc<dyn RootSlot>,
    );
    Fixture {
        swap,
        sink,
        root,
        alloc,
        journal,
    }
}

fn seg(offset: u64) -> Region {
    Region {
        arena: ArenaKind::IndexSegments,
        offset,
        len: 64,
        generation: 1,
    }
}

fn publish_n(f: &Fixture, n: usize) {
    for _ in 0..n {
        let mut table = f.swap.next_table();
        table.add(seg(table.epoch() * 64));
        f.swap.publish(table).unwrap();
    }
}

// ---- Segment table ----

#[test]
fn encode_layout_and_roundtrip() {
    let mut table = SegmentTable::new(7);
    table.add(Region {
        arena: ArenaKind::IndexSegments,
        offset: 0x1122_3344_5566_7788,
        len: 4096,
        generation: 9,
    });
    table.add(Region {
        arena: ArenaKind::Weights,
        offset: 8,
        len: 16,
        generation: u32::MAX,
    });
    table.add(seg(512));

    let bytes = table.encode();
    assert_eq!(bytes.len(), 16 + 24 * 3);
    assert_eq!(bytes.len(), table.encoded_len());
    assert_eq!(&bytes[0..8], &7u64.to_le_bytes());
    assert_eq!(&bytes[8..12], &3u32.to_le_bytes());
    assert_eq!(&bytes[12..16], &0u32.to_le_bytes());
    assert_eq!(&bytes[16..20], &1u32.to_le_bytes());
    assert_eq!(&bytes[20..24], &9u32.to_le_bytes());
    assert_eq!(&bytes[24..32], &0x1122_3344_5566_7788u64.to_le_bytes());
    assert_eq!(&bytes[32..40], &4096u64.to_le_bytes());
    assert_eq!(&bytes[40..44], &0u32.to_le_bytes());

    assert_eq!(SegmentTable::decode(&bytes).unwrap(), table);
}

#[test]
fn empty_table_roundtrip() {
    let table = SegmentTable::new(3);
    assert!(table.is_empty());
    let bytes = table.encode();
    assert_eq!(bytes.len(), 16);
    assert_eq!(SegmentTable::decode(&bytes).unwrap(), table);
}

#[test]
fn decode_rejects_malformed_input() {
    let mut table = SegmentTable::new(2);
    table.add(seg(0));
    table.add(seg(64));
    let good = table.encode();

    assert!(matches!(
        SegmentTable::decode(&good[..15]),
        Err(TableSwapError::Decode(_))
    ));
    assert!(matches!(
        SegmentTable::decode(&good[..good.len() - 1]),
        Err(TableSwapError::Decode(_))
    ));
    let mut longer = good.clone();
    longer.push(0);
    assert!(matches!(
        SegmentTable::decode(&longer),
        Err(TableSwapError::Decode(_))
    ));

    let mut reserved = good.clone();
    reserved[12] = 1;
    assert!(matches!(
        SegmentTable::decode(&reserved),
        Err(TableSwapError::Decode(_))
    ));

    let mut unknown_arena = good.clone();
    unknown_arena[16..20].copy_from_slice(&999u32.to_le_bytes());
    assert!(matches!(
        SegmentTable::decode(&unknown_arena),
        Err(TableSwapError::Decode(_))
    ));

    let mut huge_count = good;
    huge_count[8..12].copy_from_slice(&u32::MAX.to_le_bytes());
    assert!(matches!(
        SegmentTable::decode(&huge_count),
        Err(TableSwapError::Decode(_))
    ));
}

#[test]
fn add_keeps_order() {
    let mut table = SegmentTable::new(1);
    table.add(seg(0));
    table.add(seg(64));
    table.add(seg(0));
    assert_eq!(table.len(), 3);
    assert_eq!(table.segments(), &[seg(0), seg(64), seg(0)]);
}

#[test]
fn replace_hits_first_occurrence_in_place() {
    let mut table = SegmentTable::new(1);
    table.add(seg(0));
    table.add(seg(64));
    table.add(seg(0));
    assert!(table.replace(&seg(0), seg(128)));
    assert_eq!(table.segments(), &[seg(128), seg(64), seg(0)]);

    let before = table.clone();
    assert!(!table.replace(&seg(999), seg(1)));
    assert_eq!(table, before);
}

#[test]
fn remove_hits_first_occurrence_and_keeps_order() {
    let mut table = SegmentTable::new(1);
    table.add(seg(0));
    table.add(seg(64));
    table.add(seg(0));
    assert!(table.remove(&seg(0)));
    assert_eq!(table.segments(), &[seg(64), seg(0)]);

    let before = table.clone();
    assert!(!table.remove(&seg(999)));
    assert_eq!(table, before);
}

#[test]
fn stale_mapping_display() {
    let e = TableSwapError::StaleMapping {
        expected: 2,
        got: 1,
    };
    assert_eq!(e.to_string(), "stale Mapping");
}

// ---- Publication ----

#[test]
fn initial_state() {
    let f = fixture(1 << 16);
    assert_eq!(f.swap.root_value(), ROOT_SENTINEL);
    assert_eq!(f.swap.published_epoch(), 0);
    assert!(f.swap.current_table().is_none());
    assert_eq!(f.swap.retired_count(), 0);
    let next = f.swap.next_table();
    assert_eq!(next.epoch(), 1);
    assert!(next.is_empty());
    assert_eq!(f.swap.published_epoch(), 0);
}

#[test]
fn first_publish_sets_root_and_content() {
    let f = fixture(1 << 16);
    let mut table = f.swap.next_table();
    table.add(seg(0));
    table.add(seg(64));
    assert_eq!(f.swap.publish(table.clone()), Ok(1));

    let region = f.sink.regions()[0];
    assert_eq!(f.swap.root_value(), region.offset);
    assert_eq!(f.swap.published_epoch(), 1);
    assert_eq!(region.len, table.encoded_len() as u64);
    let stored = f.sink.bytes_at(region.offset).unwrap();
    assert_eq!(SegmentTable::decode(&stored).unwrap(), table);
    assert_eq!(*f.swap.current_table().unwrap(), table);
    assert_eq!(f.swap.retired_count(), 0);
}

#[test]
fn content_is_written_before_root_is_stored() {
    let f = fixture(1 << 16);
    f.journal.take();
    publish_n(&f, 1);
    let offset = f.sink.regions()[0].offset;
    assert_eq!(f.journal.take(), vec!["write".to_string(), format!("store:{offset}")]);
}

#[test]
fn new_writes_sentinel_to_root() {
    let f = fixture(1 << 16);
    assert_eq!(f.journal.take(), vec![format!("store:{ROOT_SENTINEL}")]);
    assert_eq!(f.root.load_acquire(), ROOT_SENTINEL);
}

#[test]
fn epoch_rises_by_one_and_previous_region_is_retired() {
    let f = fixture(1 << 16);
    for expected in 1..=4u64 {
        let mut table = f.swap.next_table();
        table.add(seg(expected));
        assert_eq!(f.swap.publish(table), Ok(expected));
        assert_eq!(f.swap.published_epoch(), expected);
        assert_eq!(f.swap.retired_count(), (expected - 1) as usize);
    }
    let regions = f.sink.regions();
    assert_eq!(f.swap.root_value(), regions[3].offset);
    assert_eq!(f.swap.current_table().unwrap().len(), 4);
}

#[test]
fn next_table_is_independent_copy_with_next_epoch() {
    let f = fixture(1 << 16);
    publish_n(&f, 2);
    let current = f.swap.current_table().unwrap();
    let mut next = f.swap.next_table();
    assert_eq!(next.epoch(), 3);
    assert_eq!(next.segments(), current.segments());
    next.add(seg(4096));
    assert_eq!(current.len(), 2);
    assert_eq!(f.swap.current_table().unwrap().len(), 2);
    assert_eq!(f.swap.published_epoch(), 2);
}

#[test]
fn only_one_of_two_tables_with_same_base_is_published() {
    let f = fixture(1 << 16);
    publish_n(&f, 1);
    let mut a = f.swap.next_table();
    let mut b = f.swap.next_table();
    a.add(seg(1000));
    b.add(seg(2000));
    assert_eq!(f.swap.publish(a), Ok(2));
    let retired = f.swap.retired_count();
    let root = f.swap.root_value();
    assert_eq!(
        f.swap.publish(b),
        Err(TableSwapError::StaleMapping {
            expected: 3,
            got: 2
        })
    );
    assert_eq!(f.swap.published_epoch(), 2);
    assert_eq!(f.swap.retired_count(), retired);
    assert_eq!(f.swap.root_value(), root);
}

#[test]
fn wrong_epochs_are_rejected_without_side_effects() {
    let f = fixture(1 << 16);
    let available = f.alloc.available();
    f.journal.take();
    for epoch in [0u64, 2, 99] {
        assert_eq!(
            f.swap.publish(SegmentTable::new(epoch)),
            Err(TableSwapError::StaleMapping {
                expected: 1,
                got: epoch
            })
        );
    }
    assert_eq!(f.alloc.available(), available);
    assert!(f.journal.take().is_empty());
    assert_eq!(f.swap.root_value(), ROOT_SENTINEL);
}

#[test]
fn write_failure_frees_region_and_keeps_state() {
    let f = fixture(1 << 16);
    publish_n(&f, 1);
    let available = f.alloc.available();
    let root = f.swap.root_value();
    f.journal.take();
    f.sink.fail.store(true, Ordering::SeqCst);

    let mut table = f.swap.next_table();
    table.add(seg(5));
    assert_eq!(
        f.swap.publish(table),
        Err(TableSwapError::Write("boom".to_string()))
    );
    assert_eq!(f.alloc.available(), available);
    assert_eq!(f.swap.root_value(), root);
    assert_eq!(f.swap.published_epoch(), 1);
    assert_eq!(f.swap.retired_count(), 0);
    assert!(f.journal.take().is_empty());

    f.sink.fail.store(false, Ordering::SeqCst);
    let mut table = f.swap.next_table();
    table.add(seg(5));
    assert_eq!(f.swap.publish(table), Ok(2));
}

#[test]
fn allocation_failure_keeps_state() {
    let f = fixture(16);
    let mut table = f.swap.next_table();
    table.add(seg(0));
    let err = f.swap.publish(table).unwrap_err();
    assert!(matches!(
        err,
        TableSwapError::Allocation(AllocatorError::OutOfMemory { .. })
    ));
    assert_eq!(f.swap.published_epoch(), 0);
    assert_eq!(f.swap.root_value(), ROOT_SENTINEL);
    assert_eq!(f.swap.retired_count(), 0);
    assert!(f.sink.regions().is_empty());
}

#[test]
fn publish_survives_poisoned_locks() {
    let f = fixture(1 << 16);
    f.sink.panic_next.store(true, Ordering::SeqCst);
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        f.swap.publish(f.swap.next_table())
    }));
    assert!(result.is_err());
    assert_eq!(f.swap.published_epoch(), 0);
    assert_eq!(f.swap.publish(f.swap.next_table()), Ok(1));
}

// ---- Readers and epochs ----

#[test]
fn registered_reader_starts_quiescent_at_published_epoch() {
    let f = fixture(1 << 16);
    publish_n(&f, 2);
    let id = f.swap.register_reader();
    assert_eq!(f.swap.reader_epoch(id), Ok(2));
    assert_eq!(f.swap.min_reader_epoch(), 2);
    let other = f.swap.register_reader();
    assert_ne!(id, other);
}

#[test]
fn min_reader_epoch_without_readers_is_published() {
    let f = fixture(1 << 16);
    assert_eq!(f.swap.min_reader_epoch(), 0);
    publish_n(&f, 3);
    assert_eq!(f.swap.min_reader_epoch(), 3);
}

#[test]
fn report_epoch_is_monotonic_and_bounded() {
    let f = fixture(1 << 16);
    let id = f.swap.register_reader();
    publish_n(&f, 3);
    f.swap.set_quiescent(id, false).unwrap();
    assert_eq!(f.swap.min_reader_epoch(), 0);

    assert_eq!(f.swap.report_epoch(id, 2), Ok(()));
    assert_eq!(f.swap.reader_epoch(id), Ok(2));
    assert_eq!(f.swap.min_reader_epoch(), 2);
    assert_eq!(f.swap.report_epoch(id, 1), Ok(()));
    assert_eq!(f.swap.reader_epoch(id), Ok(2));
    assert_eq!(
        f.swap.report_epoch(id, 4),
        Err(TableSwapError::EpochAhead {
            reported: 4,
            published: 3
        })
    );
    assert_eq!(f.swap.reader_epoch(id), Ok(2));
}

#[test]
fn quiescent_reader_counts_as_published() {
    let f = fixture(1 << 16);
    let id = f.swap.register_reader();
    publish_n(&f, 3);
    assert_eq!(f.swap.reader_epoch(id), Ok(0));
    assert_eq!(f.swap.min_reader_epoch(), 3);
    f.swap.set_quiescent(id, false).unwrap();
    assert_eq!(f.swap.min_reader_epoch(), 0);
    f.swap.set_quiescent(id, true).unwrap();
    assert_eq!(f.swap.min_reader_epoch(), 3);
}

#[test]
fn unknown_reader_is_reported() {
    let f = fixture(1 << 16);
    publish_n(&f, 1);
    let ghost = ReaderId(4242);
    assert_eq!(
        f.swap.report_epoch(ghost, 1),
        Err(TableSwapError::UnknownReader(ghost))
    );
    assert_eq!(
        f.swap.set_quiescent(ghost, true),
        Err(TableSwapError::UnknownReader(ghost))
    );
    assert_eq!(
        f.swap.reader_epoch(ghost),
        Err(TableSwapError::UnknownReader(ghost))
    );
    assert_eq!(
        f.swap.unregister_reader(ghost),
        Err(TableSwapError::UnknownReader(ghost))
    );
    assert!(matches!(
        f.swap.acquire(ghost),
        Err(TableSwapError::UnknownReader(_))
    ));
}

#[test]
fn unregistered_reader_no_longer_holds_back_min() {
    let f = fixture(1 << 16);
    let id = f.swap.register_reader();
    f.swap.set_quiescent(id, false).unwrap();
    publish_n(&f, 2);
    assert_eq!(f.swap.min_reader_epoch(), 0);
    assert_eq!(f.swap.unregister_reader(id), Ok(()));
    assert_eq!(f.swap.min_reader_epoch(), 2);
    assert_eq!(
        f.swap.unregister_reader(id),
        Err(TableSwapError::UnknownReader(id))
    );
}

#[test]
fn acquire_without_table_fails() {
    let f = fixture(1 << 16);
    let id = f.swap.register_reader();
    assert!(matches!(f.swap.acquire(id), Err(TableSwapError::NoTable)));
}

#[test]
fn acquire_pins_root_epoch_and_table() {
    let f = fixture(1 << 16);
    let id = f.swap.register_reader();
    publish_n(&f, 3);
    assert_eq!(f.swap.reader_epoch(id), Ok(0));

    let guard = f.swap.acquire(id).unwrap();
    assert_eq!(guard.epoch(), 3);
    assert_eq!(guard.root(), f.swap.root_value());
    assert_eq!(guard.table().len(), 3);
    assert_eq!(f.swap.reader_epoch(id), Ok(3));
    assert_eq!(f.swap.min_reader_epoch(), 3);

    publish_n(&f, 1);
    assert_eq!(guard.epoch(), 3);
    assert_eq!(guard.table().len(), 3);
    assert_eq!(f.swap.min_reader_epoch(), 3);
    assert_ne!(guard.root(), f.swap.root_value());
}

#[test]
fn guard_drop_reports_epoch_and_ignores_unknown_reader() {
    let f = fixture(1 << 16);
    let id = f.swap.register_reader();
    publish_n(&f, 2);
    let guard = f.swap.acquire(id).unwrap();
    f.swap.unregister_reader(id).unwrap();
    drop(guard);
    assert_eq!(
        f.swap.reader_epoch(id),
        Err(TableSwapError::UnknownReader(id))
    );
}

// ---- Retire and reclaim ----

#[test]
fn guard_keeps_its_table_unreclaimable() {
    let f = fixture(1 << 16);
    let id = f.swap.register_reader();
    publish_n(&f, 1);
    let guard = f.swap.acquire(id).unwrap();
    publish_n(&f, 1);
    assert_eq!(f.swap.retired_count(), 1);
    assert_eq!(f.swap.reclaim(u64::MAX), 0);

    drop(guard);
    assert_eq!(f.swap.reader_epoch(id), Ok(1));
    assert_eq!(f.swap.reclaim(u64::MAX), 0);

    f.swap.report_epoch(id, 2).unwrap();
    assert_eq!(f.swap.reclaim(u64::MAX), 1);
    assert_eq!(f.swap.retired_count(), 0);
}

#[test]
fn quiescent_reader_does_not_block_reclaim() {
    let f = fixture(1 << 16);
    let id = f.swap.register_reader();
    publish_n(&f, 1);
    let guard = f.swap.acquire(id).unwrap();
    publish_n(&f, 1);
    drop(guard);
    f.swap.set_quiescent(id, true).unwrap();
    assert_eq!(f.swap.reclaim(u64::MAX), 1);
}

#[test]
fn reclaim_respects_up_to_and_is_idempotent() {
    let f = fixture(1 << 16);
    publish_n(&f, 3);
    let regions = f.sink.regions();
    assert_eq!(f.swap.retired_count(), 2);

    assert_eq!(f.swap.reclaim(1), 0);
    assert_eq!(f.swap.reclaim(2), 1);
    assert_eq!(f.swap.reclaim(2), 0);
    assert!(!f.alloc.is_live(&regions[0]));
    assert!(f.alloc.is_live(&regions[1]));

    assert_eq!(f.swap.reclaim(u64::MAX), 1);
    assert_eq!(f.swap.reclaim(u64::MAX), 0);
    assert_eq!(f.swap.retired_count(), 0);
    assert!(!f.alloc.is_live(&regions[1]));
    assert!(f.alloc.is_live(&regions[2]));
}

#[test]
fn reclaim_drops_stale_entries_without_counting_them() {
    let f = fixture(1 << 16);
    publish_n(&f, 2);
    let first = f.sink.regions()[0];
    f.alloc.free(&first).unwrap();
    assert_eq!(f.swap.retired_count(), 1);
    assert_eq!(f.swap.reclaim(u64::MAX), 0);
    assert_eq!(f.swap.retired_count(), 0);
}

#[test]
fn reclaim_without_entries_returns_zero() {
    let f = fixture(1 << 16);
    assert_eq!(f.swap.reclaim(u64::MAX), 0);
    publish_n(&f, 1);
    assert_eq!(f.swap.reclaim(u64::MAX), 0);
    assert!(f.alloc.is_live(&f.sink.regions()[0]));
}

// ---- Concurrency ----

#[test]
fn parallel_reclaim_frees_each_entry_once() {
    let f = Arc::new(fixture(1 << 20));
    publish_n(&f, 21);
    assert_eq!(f.swap.retired_count(), 20);
    let handles: Vec<_> = (0..8)
        .map(|_| {
            let f = Arc::clone(&f);
            thread::spawn(move || f.swap.reclaim(u64::MAX))
        })
        .collect();
    let total: usize = handles.into_iter().map(|h| h.join().unwrap()).sum();
    assert_eq!(total, 20);
    assert_eq!(f.swap.retired_count(), 0);
}

#[test]
fn parallel_publish_has_one_winner_per_epoch() {
    let f = Arc::new(fixture(1 << 20));
    let handles: Vec<_> = (0..8)
        .map(|_| {
            let f = Arc::clone(&f);
            thread::spawn(move || {
                let mut wins = 0u64;
                for _ in 0..25 {
                    let mut table = f.swap.next_table();
                    table.add(seg(table.epoch()));
                    match f.swap.publish(table) {
                        Ok(_) => wins += 1,
                        Err(TableSwapError::StaleMapping { .. }) => {}
                        Err(e) => panic!("unexpected error: {e}"),
                    }
                }
                wins
            })
        })
        .collect();
    let wins: u64 = handles.into_iter().map(|h| h.join().unwrap()).sum();
    assert!(wins > 0);
    assert_eq!(f.swap.published_epoch(), wins);
    assert_eq!(f.swap.retired_count() as u64, wins - 1);
    assert_eq!(f.swap.current_table().unwrap().len() as u64, wins);
}

#[test]
fn pinned_content_matches_epoch_under_concurrent_publish_and_reclaim() {
    let f = Arc::new(fixture(1 << 20));
    let stop = Arc::new(AtomicBool::new(false));
    let reader = {
        let f = Arc::clone(&f);
        let stop = Arc::clone(&stop);
        thread::spawn(move || {
            let id = f.swap.register_reader();
            let mut checked = 0u64;
            while !stop.load(Ordering::Acquire) {
                match f.swap.acquire(id) {
                    Err(TableSwapError::NoTable) => thread::yield_now(),
                    Err(e) => panic!("unexpected error: {e}"),
                    Ok(guard) => {
                        let bytes = f
                            .sink
                            .bytes_at(guard.root())
                            .expect("root must point at a written table");
                        let decoded = SegmentTable::decode(&bytes).unwrap();
                        assert_eq!(&decoded, guard.table());
                        checked += 1;
                    }
                }
            }
            checked
        })
    };
    for _ in 0..300 {
        let mut table = f.swap.next_table();
        table.add(seg(table.epoch()));
        f.swap.publish(table).unwrap();
        f.swap.reclaim(u64::MAX);
    }
    stop.store(true, Ordering::Release);
    assert!(reader.join().unwrap() > 0);
}

// ---- AccessSink ----

#[derive(Debug)]
struct VecBackend {
    data: Mutex<Vec<u8>>,
}

impl RegionBackend for VecBackend {
    fn len(&self) -> usize {
        lock(&self.data).len()
    }

    fn read_raw(&self, offset: usize, dst: &mut [u8]) -> Result<(), AccessError> {
        dst.copy_from_slice(&lock(&self.data)[offset..offset + dst.len()]);
        Ok(())
    }

    fn write_raw(&self, offset: usize, src: &[u8]) -> Result<(), AccessError> {
        lock(&self.data)[offset..offset + src.len()].copy_from_slice(src);
        Ok(())
    }
}

fn backend(size: usize) -> Arc<VecBackend> {
    Arc::new(VecBackend {
        data: Mutex::new(vec![0; size]),
    })
}

fn access_sink(
    backend: &Arc<VecBackend>,
    after_write: Box<dyn Fn() -> Result<Option<Event>, String> + Send + Sync>,
) -> AccessSink {
    let backend = Arc::clone(backend);
    AccessSink::new(
        Box::new(move |_| Some(AccessRegion::new(Arc::clone(&backend) as Arc<dyn RegionBackend>))),
        after_write,
    )
}

#[test]
fn access_sink_writes_and_waits_for_event() {
    let store = backend(64);
    let (event, control) = Event::mock();
    control.complete();
    let sink = access_sink(&store, Box::new(move || Ok(Some(event.clone()))));
    sink.write_visible(&seg(0), &[1, 2, 3, 4]).unwrap();
    assert_eq!(&lock(&store.data)[..5], &[1, 2, 3, 4, 0]);
}

#[test]
fn access_sink_blocks_until_event_completes() {
    let store = backend(16);
    let (event, control) = Event::mock();
    let completed = Arc::new(AtomicBool::new(false));
    let completer = {
        let completed = Arc::clone(&completed);
        thread::spawn(move || {
            thread::sleep(Duration::from_millis(50));
            completed.store(true, Ordering::SeqCst);
            control.complete();
        })
    };
    let sink = access_sink(&store, Box::new(move || Ok(Some(event.clone()))));
    sink.write_visible(&seg(0), &[9; 8]).unwrap();
    assert!(completed.load(Ordering::SeqCst));
    completer.join().unwrap();
}

#[test]
fn access_sink_without_event_only_runs_flush() {
    let store = backend(16);
    let flushed = Arc::new(AtomicBool::new(false));
    let flag = Arc::clone(&flushed);
    let sink = access_sink(
        &store,
        Box::new(move || {
            flag.store(true, Ordering::SeqCst);
            Ok(None)
        }),
    );
    sink.write_visible(&seg(0), &[7; 4]).unwrap();
    assert!(flushed.load(Ordering::SeqCst));
    assert_eq!(&lock(&store.data)[..4], &[7; 4]);
}

#[test]
fn access_sink_reports_failures() {
    let store = backend(4);

    let unresolved = AccessSink::new(Box::new(|_| None), Box::new(|| Ok(None)));
    assert!(unresolved.write_visible(&seg(0), &[1]).is_err());

    let too_long = access_sink(&store, Box::new(|| Ok(None)));
    assert!(too_long.write_visible(&seg(0), &[0; 8]).is_err());

    let after_fail = access_sink(&store, Box::new(|| Err("flush failed".to_string())));
    assert_eq!(
        after_fail.write_visible(&seg(0), &[1]),
        Err("flush failed".to_string())
    );

    let (event, control) = Event::mock();
    control.fail("copy failed");
    let failed_event = access_sink(&store, Box::new(move || Ok(Some(event.clone()))));
    assert!(failed_event.write_visible(&seg(0), &[1]).is_err());
}

#[test]
fn access_sink_publishes_through_table_swap() {
    let store = backend(4096);
    let sink = access_sink(&store, Box::new(|| Ok(None)));
    let swap = TableSwap::new(
        make_allocator(4096),
        Arc::new(sink),
        Arc::new(HostRootSlot::new()),
    );
    let mut table = swap.next_table();
    table.add(seg(0));
    swap.publish(table.clone()).unwrap();
    let bytes = lock(&store.data)[..table.encoded_len()].to_vec();
    assert_eq!(SegmentTable::decode(&bytes).unwrap(), table);
}

// ---- Device protocol emulation (feature gpu-tests) ----

#[cfg(feature = "gpu-tests")]
mod device_protocol {
    use super::*;
    use std::sync::atomic::fence;

    struct SharedMemorySink {
        words: Arc<Vec<AtomicU64>>,
    }

    impl TableSink for SharedMemorySink {
        fn write_visible(&self, region: &Region, bytes: &[u8]) -> Result<(), String> {
            let base = (region.offset / 8) as usize;
            for (i, chunk) in bytes.chunks(8).enumerate() {
                let mut word = [0u8; 8];
                word[..chunk.len()].copy_from_slice(chunk);
                self.words[base + i].store(u64::from_le_bytes(word), Ordering::Relaxed);
            }
            fence(Ordering::Release);
            Ok(())
        }
    }

    #[test]
    #[ignore]
    fn every_observed_root_points_at_a_complete_table() {
        const ARENA_BYTES: u64 = 8 << 20;
        let words: Arc<Vec<AtomicU64>> =
            Arc::new((0..ARENA_BYTES / 8).map(|_| AtomicU64::new(0)).collect());
        let root = Arc::new(HostRootSlot::new());
        let swap = Arc::new(TableSwap::new(
            make_allocator(ARENA_BYTES),
            Arc::new(SharedMemorySink {
                words: Arc::clone(&words),
            }),
            Arc::clone(&root) as Arc<dyn RootSlot>,
        ));
        let stop = Arc::new(AtomicBool::new(false));

        let reader = {
            let words = Arc::clone(&words);
            let root = Arc::clone(&root);
            let stop = Arc::clone(&stop);
            thread::spawn(move || {
                let mut last_epoch = 0u64;
                let mut observed = 0u64;
                while !stop.load(Ordering::Acquire) {
                    let offset = root.load_acquire();
                    if offset == ROOT_SENTINEL {
                        continue;
                    }
                    let base = (offset / 8) as usize;
                    let count = (words[base + 1].load(Ordering::Relaxed) & 0xffff_ffff) as usize;
                    let total_words = (TABLE_HEADER_BYTES + SEGMENT_BYTES * count) / 8;
                    let mut bytes = Vec::with_capacity(total_words * 8);
                    for w in &words[base..base + total_words] {
                        bytes.extend_from_slice(&w.load(Ordering::Relaxed).to_le_bytes());
                    }
                    let table = SegmentTable::decode(&bytes).expect("complete table");
                    assert_eq!(table.len() as u64, table.epoch());
                    assert!(table.epoch() >= last_epoch);
                    last_epoch = table.epoch();
                    observed += 1;
                }
                observed
            })
        };

        for _ in 0..300 {
            let mut table = swap.next_table();
            table.add(seg(table.epoch()));
            swap.publish(table).unwrap();
        }
        stop.store(true, Ordering::Release);
        assert!(reader.join().unwrap() > 0);
    }
}
