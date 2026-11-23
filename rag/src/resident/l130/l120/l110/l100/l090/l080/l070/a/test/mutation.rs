use super::*;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::b::doorbell::{
    Doorbell, DoorbellCell,
};
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::a::memory_access::RegionBackend;
use std::sync::atomic::AtomicBool;
use std::sync::Mutex;
use std::thread;
#[derive(Debug)]
struct MockBackend {
    data: Mutex<Vec<u8>>,
}
impl RegionBackend for MockBackend {
    fn len(&self) -> usize {
        self.data.lock().unwrap().len()
    }
    fn read_raw(&self, offset: usize, dst: &mut [u8]) -> Result<(), AccessError> {
        let d = self.data.lock().unwrap();
        dst.copy_from_slice(&d[offset..offset + dst.len()]);
        Ok(())
    }
    fn write_raw(&self, offset: usize, src: &[u8]) -> Result<(), AccessError> {
        let mut d = self.data.lock().unwrap();
        d[offset..offset + src.len()].copy_from_slice(src);
        Ok(())
    }
}
fn mock_region(len: usize) -> AccessRegion {
    AccessRegion::new(
        Arc::new(MockBackend {
            data: Mutex::new(vec![0; len]),
        }),
    )
}
fn doorbell_with_gap(completed: u64, acked: u64) -> Doorbell {
    let c = DoorbellCell::new_local();
    let a = DoorbellCell::new_local();
    c.test_store(completed, Ordering::Release);
    a.test_store(acked, Ordering::Release);
    Doorbell::new(c, a)
}
#[test]
fn swap_returns_old_and_load_returns_new() {
    let mut db = doorbell_with_gap(3, 0);
    let gap = db.poll().unwrap().unwrap();
    let guard = GapGuard::enter(&gap, KernelMode::Batch);
    let cell = PtrCell::new_local(Level::Root, 0x1000);
    assert_eq!(cell.level(), Level::Root);
    assert_eq!(guard.swap_ptr(& cell, 0x2000), Ok(0x1000));
    assert_eq!(cell.load(), 0x2000);
    assert_eq!(guard.swap_ptr(& cell, 0x3000), Ok(0x2000));
    assert_eq!(guard.gap_range(), (1, 3));
    drop(guard);
    gap.close();
}
#[test]
fn invalid_values_leave_cell_unchanged() {
    let mut db = doorbell_with_gap(1, 0);
    let gap = db.poll().unwrap().unwrap();
    let guard = GapGuard::enter(&gap, KernelMode::Persistent);
    let cell = PtrCell::new_local(Level::Segment, 0x1000);
    assert_eq!(guard.swap_ptr(& cell, 0), Err(MutationError::NullPointer));
    assert_eq!(
        guard.swap_ptr(& cell, 0x1004), Err(MutationError::Misaligned { value : 0x1004 })
    );
    assert_eq!(cell.load(), 0x1000);
    drop(guard);
    gap.close();
}
#[test]
fn guard_does_not_close_gap() {
    let mut db = doorbell_with_gap(2, 0);
    let gap = db.poll().unwrap().unwrap();
    {
        let guard = GapGuard::enter(&gap, KernelMode::Batch);
        let cell = PtrCell::new_local(Level::SegmentTable, 8);
        guard.swap_ptr(&cell, 16).unwrap();
        assert_eq!(guard.mode(), KernelMode::Batch);
    }
    gap.close();
    assert_eq!(db.acked(), 2);
}
#[test]
fn guard_leaves_acked_unchanged_before_close() {
    let acked = DoorbellCell::new_local();
    let completed = DoorbellCell::new_local();
    completed.test_store(5, Ordering::Release);
    let probe = acked.share();
    let mut db = Doorbell::new(completed, acked);
    let gap = db.poll().unwrap().unwrap();
    let guard = GapGuard::enter(&gap, KernelMode::Batch);
    let cell = PtrCell::new_local(Level::OffsetTable, 8);
    guard.swap_ptr(&cell, 16).unwrap();
    assert_eq!(probe.test_load(Ordering::Acquire), 0);
    drop(guard);
    gap.close();
    assert_eq!(probe.test_load(Ordering::Acquire), 5);
}
#[test]
fn reader_sees_only_old_or_new() {
    const OLD: u64 = 0x1000;
    const NEW: u64 = 0x2000;
    let cell = PtrCell::new_local(Level::LookupTable, OLD);
    let reader_cell = cell.share();
    let stop = Arc::new(AtomicBool::new(false));
    let stop_r = Arc::clone(&stop);
    let reader = thread::spawn(move || {
        let mut n = 0u64;
        while !stop_r.load(Ordering::Relaxed) {
            let v = reader_cell.load();
            assert!(v == OLD || v == NEW, "unexpected value {v:#x}");
            n += 1;
        }
        n
    });
    let mut db = doorbell_with_gap(1, 0);
    let gap = db.poll().unwrap().unwrap();
    let guard = GapGuard::enter(&gap, KernelMode::Persistent);
    for i in 0..100_000 {
        let (next, prev) = if i % 2 == 0 { (NEW, OLD) } else { (OLD, NEW) };
        assert_eq!(guard.swap_ptr(& cell, next), Ok(prev));
    }
    stop.store(true, Ordering::Relaxed);
    assert!(reader.join().unwrap() > 0);
    drop(guard);
    gap.close();
}
#[test]
fn release_acquire_publishes_structure() {
    let data: Arc<Vec<AtomicU64>> = Arc::new(
        (0..16).map(|_| AtomicU64::new(0)).collect(),
    );
    let cell = PtrCell::new_local(Level::Segment, 8);
    let reader_cell = cell.share();
    let reader_data = Arc::clone(&data);
    let reader = thread::spawn(move || {
        loop {
            if reader_cell.load() == 0x4000 {
                for slot in reader_data.iter() {
                    assert_eq!(slot.load(Ordering::Relaxed), 0xABCD);
                }
                break;
            }
            std::hint::spin_loop();
        }
    });
    for slot in data.iter() {
        slot.store(0xABCD, Ordering::Relaxed);
    }
    let mut db = doorbell_with_gap(1, 0);
    let gap = db.poll().unwrap().unwrap();
    let guard = GapGuard::enter(&gap, KernelMode::Batch);
    guard.swap_ptr(&cell, 0x4000).unwrap();
    reader.join().unwrap();
    drop(guard);
    gap.close();
}
#[test]
fn concurrent_swaps_each_get_replaced_value() {
    let cell = PtrCell::new_local(Level::Root, 8);
    let mut handles = Vec::new();
    for t in 1..=4u64 {
        let c = cell.share();
        handles
            .push(
                thread::spawn(move || {
                    let mut db = doorbell_with_gap(1, 0);
                    let gap = db.poll().unwrap().unwrap();
                    let guard = GapGuard::enter(&gap, KernelMode::Batch);
                    let mut olds = Vec::new();
                    for i in 0..1000u64 {
                        let new = (t * 100_000 + i + 1) * 8;
                        olds.push((guard.swap_ptr(&c, new).unwrap(), new));
                    }
                    drop(guard);
                    gap.close();
                    olds
                }),
            );
    }
    let mut replaced = Vec::new();
    let mut installed = vec![8u64];
    for h in handles {
        for (old, new) in h.join().unwrap() {
            replaced.push(old);
            installed.push(new);
        }
    }
    let last = cell.load();
    installed.sort_unstable();
    replaced.push(last);
    replaced.sort_unstable();
    assert_eq!(installed, replaced);
}
#[test]
fn write_in_place_effects_and_errors() {
    let mut db = doorbell_with_gap(1, 0);
    let gap = db.poll().unwrap().unwrap();
    let guard = GapGuard::enter(&gap, KernelMode::Batch);
    let region = mock_region(16);
    guard.write_in_place(&region, 4, &[1, 2, 3]).unwrap();
    let mut buf = [0u8; 3];
    region.read(4, &mut buf).unwrap();
    assert_eq!(buf, [1, 2, 3]);
    assert!(
        matches!(guard.write_in_place(& region, 15, & [1, 2]),
        Err(MutationError::Access(AccessError::OutOfBounds { .. })))
    );
    assert_eq!(guard.write_in_place(& region, 1000, & []), Ok(()));
    drop(guard);
    gap.close();
}
#[test]
fn persistent_torn_read_yields_only_old_or_new_elements() {
    const N: usize = 64;
    let region = mock_region(N);
    region.write(0, &[1u8; N]).unwrap();
    let reader_region = region.clone();
    let stop = Arc::new(AtomicBool::new(false));
    let stop_r = Arc::clone(&stop);
    let reader = thread::spawn(move || {
        let mut buf = [0u8; N];
        while !stop_r.load(Ordering::Relaxed) {
            for i in 0..N {
                let mut b = [0u8; 1];
                reader_region.read(i, &mut b).unwrap();
                buf[i] = b[0];
            }
            assert!(buf.iter().all(|& v | v == 1 || v == 2));
        }
    });
    let mut db = doorbell_with_gap(1, 0);
    let gap = db.poll().unwrap().unwrap();
    let guard = GapGuard::enter(&gap, KernelMode::Persistent);
    for i in 0..2000 {
        let v = if i % 2 == 0 { 2u8 } else { 1u8 };
        guard.write_in_place(&region, 0, &[v; N]).unwrap();
    }
    stop.store(true, Ordering::Relaxed);
    reader.join().unwrap();
    drop(guard);
    gap.close();
}
#[test]
fn error_display_and_source() {
    let e = MutationError::Misaligned {
        value: 5,
    };
    assert!(e.to_string().contains("0x5"));
    let a = MutationError::Access(AccessError::Backend("x".into()));
    assert!(std::error::Error::source(& a).is_some());
}
