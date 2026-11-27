use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;
use super::super::fixture::{config, MockBackend, MIB};
use super::*;
#[test]
fn vram_types_are_send_sync() {
    fn check<T: Send + Sync>() {}
    check::<WeightSet>();
    check::<VramManager>();
}
static COUNTER: AtomicUsize = AtomicUsize::new(0);
struct TestDir(PathBuf);
impl TestDir {
    fn new() -> TestDir {
        let n = COUNTER.fetch_add(1, Ordering::SeqCst);
        let path = std::env::temp_dir()
            .join(format!("gpu_vram_{}_{n}", std::process::id()));
        std::fs::create_dir_all(&path).unwrap();
        TestDir(path)
    }
}
impl Drop for TestDir {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}
/// Writes a safetensors file; tensor `i` holds `count` f32 values of `i + 1`.
fn write_weights(path: &Path, tensors: &[(&str, usize)]) {
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
/// `a`: 400 bytes at 0, `b`: 40 bytes at 512, `c`: 256 bytes at 768; 1024 bytes in all.
fn small_file(dir: &TestDir) -> WeightFile {
    let path = dir.0.join("small.safetensors");
    write_weights(&path, &[("a", 100), ("b", 10), ("c", 64)]);
    WeightFile::open(&path).unwrap()
}
fn manager(backend: &Arc<MockBackend>) -> Arc<VramManager> {
    VramManager::new(0, Arc::clone(backend) as Arc<dyn GpuBackend3>, &config())
}
fn all(_: &str) -> bool {
    true
}
fn offsets(set: &WeightSet) -> Vec<(String, usize)> {
    set.tensors()
        .tensors
        .iter()
        .map(|(name, t)| (name.clone(), t.offset_bytes()))
        .collect()
}
#[test]
fn arenas_are_reserved_once_each() {
    let backend = MockBackend::single();
    let vram = manager(&backend);
    assert!(! vram.arenas_ready());
    assert!(vram.arena_region(ArenaKind::Weights).is_none());
    vram.ensure_arenas().unwrap();
    assert!(vram.arenas_ready());
    let order: Vec<ArenaKind> = lock(&backend.reserved)
        .iter()
        .map(|(_, k)| *k)
        .collect();
    assert_eq!(order, ArenaKind::ALL.to_vec());
    assert_eq!(backend.live.load(Ordering::SeqCst), 6);
    vram.ensure_arenas().unwrap();
    assert_eq!(lock(& backend.reserved).len(), 6);
    let caps = config().arenas;
    for kind in ArenaKind::ALL {
        let region = vram.arena_region(kind).unwrap();
        assert_eq!(region.len() as u64, caps.get(kind), "{kind:?}");
    }
    assert_eq!(vram.ordinal(), 0);
}
#[test]
fn arenas_are_released_with_the_manager() {
    let backend = MockBackend::single();
    let vram = manager(&backend);
    vram.ensure_arenas().unwrap();
    assert_eq!(backend.live.load(Ordering::SeqCst), 6);
    drop(vram);
    assert_eq!(backend.live.load(Ordering::SeqCst), 0);
}
#[test]
fn a_budget_failure_reserves_nothing_and_is_retried() {
    let backend = MockBackend::single();
    let vram = manager(&backend);
    backend.free.store(MIB, Ordering::SeqCst);
    let err = vram.ensure_arenas().unwrap_err();
    assert!(matches!(err, GpuError::Budget(BudgetError::DoesNotFit { .. })), "{err:?}");
    assert!(lock(& backend.reserved).is_empty());
    assert!(! vram.arenas_ready());
    backend.free.store(64 * MIB, Ordering::SeqCst);
    vram.ensure_arenas().unwrap();
    assert!(vram.arenas_ready());
}
#[test]
fn the_headroom_is_part_of_the_fit_check() {
    let backend = MockBackend::single();
    let mut cfg = config();
    cfg.headroom = 62 * MIB;
    let vram = VramManager::new(0, Arc::clone(&backend) as Arc<dyn GpuBackend3>, &cfg);
    backend.free.store(64 * MIB, Ordering::SeqCst);
    assert!(matches!(vram.ensure_arenas(), Err(GpuError::Budget(_))));
    assert!(lock(& backend.reserved).is_empty());
}
#[test]
fn a_backend_failure_releases_the_reserved_arenas_and_is_not_cached() {
    let backend = MockBackend::single();
    let vram = manager(&backend);
    *lock(&backend.fail_reserve) = Some(ArenaKind::Staging);
    let err = vram.ensure_arenas().unwrap_err();
    assert!(matches!(err, GpuError::Backend(_)), "{err:?}");
    assert_eq!(backend.live.load(Ordering::SeqCst), 0);
    assert!(! vram.arenas_ready());
    assert!(vram.arena_region(ArenaKind::Weights).is_none());
    *lock(&backend.fail_reserve) = None;
    vram.ensure_arenas().unwrap();
    assert_eq!(backend.live.load(Ordering::SeqCst), 6);
}
#[test]
fn concurrent_calls_reserve_every_arena_exactly_once() {
    let backend = MockBackend::single();
    let vram = manager(&backend);
    thread::scope(|scope| {
        for _ in 0..8 {
            scope.spawn(|| vram.ensure_arenas().unwrap());
        }
    });
    assert_eq!(lock(& backend.reserved).len(), 6);
    for kind in ArenaKind::ALL {
        assert_eq!(backend.reserve_count(kind), 1, "{kind:?}");
    }
}
#[test]
fn concurrent_calls_see_a_failure_and_a_later_retry_succeeds() {
    let backend = MockBackend::single();
    let vram = manager(&backend);
    *lock(&backend.fail_reserve) = Some(ArenaKind::MappedHost);
    thread::scope(|scope| {
        for _ in 0..4 {
            scope.spawn(|| assert!(vram.ensure_arenas().is_err()));
        }
    });
    assert_eq!(backend.live.load(Ordering::SeqCst), 0);
    *lock(&backend.fail_reserve) = None;
    vram.ensure_arenas().unwrap();
}
#[test]
fn weights_are_copied_to_aligned_offsets() {
    let dir = TestDir::new();
    let file = small_file(&dir);
    let backend = MockBackend::single();
    let vram = manager(&backend);
    assert_eq!(vram.weights_used(), 0);
    assert_eq!(vram.weights_capacity(), MIB);
    let set = vram.stage_weights(&file, &all).unwrap();
    assert!(vram.arenas_ready());
    assert_eq!(set.bytes(), 1024);
    assert_eq!(vram.weights_used(), 1024);
    assert_eq!(
        offsets(& set), vec![("a".to_string(), 0), ("b".to_string(), 512), ("c"
        .to_string(), 768)]
    );
    let region = vram.arena_region(ArenaKind::Weights).unwrap();
    let mut b = [0u8; 4];
    region.read(512, &mut b).unwrap();
    assert_eq!(f32::from_le_bytes(b), 2.0);
    region.read(768, &mut b).unwrap();
    assert_eq!(f32::from_le_bytes(b), 3.0);
    region.read(396, &mut b).unwrap();
    assert_eq!(f32::from_le_bytes(b), 1.0);
}
#[test]
fn the_extent_is_rounded_up_to_the_alignment() {
    let dir = TestDir::new();
    let file = small_file(&dir);
    let backend = MockBackend::single();
    let vram = manager(&backend);
    let set = vram.stage_weights(&file, &|n| n != "c").unwrap();
    assert_eq!(set.bytes(), 768);
    assert_eq!(vram.weights_used(), 768);
}
#[test]
fn later_sets_follow_the_earlier_ones_and_stay_aligned() {
    let dir = TestDir::new();
    let file = small_file(&dir);
    let backend = MockBackend::single();
    let vram = manager(&backend);
    let first = vram.stage_weights(&file, &all).unwrap();
    let second = vram.stage_weights(&file, &|n| n != "c").unwrap();
    assert_eq!(offsets(& first) [0].1, 0);
    assert_eq!(
        offsets(& second), vec![("a".to_string(), 1024), ("b".to_string(), 1536)]
    );
    assert_eq!(vram.weights_used(), 1024 + 768);
    assert!(vram.weights_used() <= vram.weights_capacity());
}
#[test]
fn dropping_the_last_arc_frees_the_extent_for_reuse() {
    let dir = TestDir::new();
    let file = small_file(&dir);
    let backend = MockBackend::single();
    let vram = manager(&backend);
    let first = vram.stage_weights(&file, &all).unwrap();
    let keep = Arc::clone(&first);
    drop(first);
    assert_eq!(vram.weights_used(), 1024);
    drop(keep);
    assert_eq!(vram.weights_used(), 0);
    let again = vram.stage_weights(&file, &all).unwrap();
    assert_eq!(offsets(& again) [0].1, 0);
    assert_eq!(vram.weights_used(), 1024);
}
#[test]
fn an_empty_selection_allocates_nothing() {
    let dir = TestDir::new();
    let file = small_file(&dir);
    let backend = MockBackend::single();
    let vram = manager(&backend);
    let set = vram.stage_weights(&file, &|_| false).unwrap();
    assert_eq!(set.bytes(), 0);
    assert!(set.tensors().tensors.is_empty());
    assert_eq!(vram.weights_used(), 0);
    drop(set);
    assert_eq!(vram.weights_used(), 0);
}
#[test]
fn an_exhausted_weights_arena_reports_out_of_memory_and_keeps_the_state() {
    let dir = TestDir::new();
    let big = dir.0.join("big.safetensors");
    write_weights(&big, &[("huge", (MIB as usize) / 4 + 1)]);
    let big = WeightFile::open(&big).unwrap();
    let file = small_file(&dir);
    let backend = MockBackend::single();
    let vram = manager(&backend);
    let kept = vram.stage_weights(&file, &all).unwrap();
    let err = vram.stage_weights(&big, &all).err().unwrap();
    assert!(
        matches!(err, GpuError::Memory(AllocatorError::OutOfMemory { .. })), "{err:?}"
    );
    assert_eq!(vram.weights_used(), 1024);
    assert_eq!(offsets(& kept) [0].1, 0);
    let more = vram.stage_weights(&file, &all).unwrap();
    assert_eq!(offsets(& more) [0].1, 1024);
}
#[test]
fn a_copy_error_frees_the_extent_again() {
    let dir = TestDir::new();
    let file = small_file(&dir);
    let backend = MockBackend::single();
    *lock(&backend.shrink) = Some(ArenaKind::Weights);
    let vram = manager(&backend);
    let err = vram.stage_weights(&file, &all).err().unwrap();
    assert!(
        matches!(err, GpuError::Weights(WeightError::CapacityExceeded { .. })), "{err:?}"
    );
    assert_eq!(vram.weights_used(), 0);
}
#[test]
fn a_failing_reservation_is_reported_by_stage_weights() {
    let dir = TestDir::new();
    let file = small_file(&dir);
    let backend = MockBackend::single();
    *lock(&backend.fail_reserve) = Some(ArenaKind::Weights);
    let vram = manager(&backend);
    assert!(matches!(vram.stage_weights(& file, & all), Err(GpuError::Backend(_))));
    assert_eq!(vram.weights_used(), 0);
}
#[test]
fn concurrent_sets_get_disjoint_extents() {
    let dir = TestDir::new();
    let file = small_file(&dir);
    let backend = MockBackend::single();
    let vram = manager(&backend);
    let sets: Vec<Arc<WeightSet>> = thread::scope(|scope| {
        let handles: Vec<_> = (0..8)
            .map(|_| scope.spawn(|| vram.stage_weights(&file, &|n| n != "c").unwrap()))
            .collect();
        handles.into_iter().map(|h| h.join().unwrap()).collect()
    });
    let mut starts: Vec<usize> = sets.iter().map(|s| offsets(s)[0].1).collect();
    starts.sort();
    for (i, start) in starts.iter().enumerate() {
        assert_eq!(* start, i * 768);
    }
    assert_eq!(vram.weights_used(), 8 * 768);
    assert!(vram.weights_used() <= vram.weights_capacity());
    drop(sets);
    assert_eq!(vram.weights_used(), 0);
}
#[test]
fn a_set_keeps_the_arena_reserved_while_it_lives() {
    let dir = TestDir::new();
    let file = small_file(&dir);
    let backend = MockBackend::single();
    let vram = manager(&backend);
    let set = vram.stage_weights(&file, &all).unwrap();
    drop(vram);
    assert!(backend.live.load(Ordering::SeqCst) > 0);
    drop(set);
    assert_eq!(backend.live.load(Ordering::SeqCst), 0);
}
