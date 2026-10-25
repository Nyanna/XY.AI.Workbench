use std::io;
use std::sync::atomic::{AtomicU32, Ordering};
use super::*;
use crate::resident::index::memory::InMemoryIndex;
use crate::resident::manager::residency::{
    LoadPolicy, LoadTransfer, SegmentId, UniformGeometry,
};
use crate::resident::manager::tracker::SlotRef;
const SLOT: SlotRef = SlotRef { segment: 0, slot: 0 };
struct NullBackend;
impl ComputeBackend for NullBackend {
    fn run_batch(&self, input: &[f32]) -> Vec<f32> {
        input.to_vec()
    }
}
#[derive(Default)]
struct Hints(Mutex<Vec<(Range<usize>, Hint)>>);
impl AdviceSink for Hints {
    fn advise(&self, range: Range<usize>, hint: Hint) -> io::Result<()> {
        self.0.lock().push((range, hint));
        Ok(())
    }
}
#[derive(Default)]
struct Transfers {
    gathers: AtomicU32,
}
impl LoadTransfer for Transfers {
    fn load_segments(&self, segments: &[SegmentId]) -> Result<u64, ManagerError> {
        Ok(segments.len() as u64 * 1000)
    }
    fn load_gather(&self, set: &EntrySet) -> Result<u64, ManagerError> {
        self.gathers.fetch_add(1, Ordering::SeqCst);
        Ok(set.len() * 1024)
    }
    fn map_zero_copy(&self, set: &EntrySet) -> Result<u64, ManagerError> {
        Ok(set.len() * 1024)
    }
}
fn id(n: u64) -> EntryId {
    EntryId::new(n)
}
fn core() -> StreamedIndexManager<NullBackend> {
    StreamedIndexManager::new(Arc::new(NullBackend), "test")
}
fn gpu(transfers: &Arc<Transfers>) -> GpuIndexManager<NullBackend> {
    let geometry = UniformGeometry {
        entries_per_segment: 100,
        entry_bytes: 1024,
    };
    let loader = ResidencyLoader::new(
        LoadPolicy::default(),
        Arc::new(geometry),
        transfers.clone(),
    );
    GpuIndexManager::new(
        core(),
        ResolverRegistry::with_defaults(),
        loader,
        EvictPolicy::vram(),
    )
}
#[test]
fn manager_holds_nothing_until_an_index_is_attached() {
    let m = core();
    assert!(! m.is_active());
    assert!(matches!(m.index(), Err(IndexError::IndexUnavailable { .. })));
    assert_eq!(m.resident_len(), 0);
    assert!(m.attach_index(Arc::new(InMemoryIndex::new())));
    assert!(m.is_active());
    assert!(m.index().is_ok());
    assert!(! m.attach_index(Arc::new(InMemoryIndex::new())));
}
#[tokio::test]
async fn activation_runs_once_and_a_failure_leaves_the_manager_inactive() {
    let m = core();
    let failed = m
        .activate(|| async {
            Err(IndexError::TransferFailed {
                reason: "open".into(),
            })
        })
        .await;
    assert!(matches!(failed, Err(IndexError::TransferFailed { .. })));
    assert!(! m.is_active());
    let runs = AtomicU32::new(0);
    for _ in 0..2 {
        m.activate(|| async {
                runs.fetch_add(1, Ordering::SeqCst);
                Ok(Arc::new(InMemoryIndex::new()) as SharedIndex)
            })
            .await
            .unwrap();
    }
    assert_eq!(runs.load(Ordering::SeqCst), 1);
    assert!(m.is_active());
}
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn cpu_manager_reads_the_entry_and_hints_the_page_cache() {
    let index = Arc::new(InMemoryIndex::new());
    index.apply(id(1), b"abc");
    let hints = Arc::new(Hints::default());
    let m = CpuIndexManager::new(core(), Handle::current(), Some(hints.clone()));
    assert!(
        matches!(m.read(id(1), 0..3). await, Err(IndexError::IndexUnavailable { .. }))
    );
    m.attach_index(index);
    assert_eq!(m.read(id(1), 0..3). await .unwrap(), b"abc");
    assert!(m.read(id(9), 3..6). await .unwrap().is_empty());
    m.cool(0..3);
    assert_eq!(
        * hints.0.lock(), vec![(0..3, Hint::WillNeed), (3..6, Hint::WillNeed), (0..3,
        Hint::Cold)]
    );
}
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn dense_sets_hint_sequential_and_scattered_sets_hint_random() {
    let index = Arc::new(InMemoryIndex::new());
    index.apply(id(1), b"a");
    index.apply(id(2), b"b");
    let hints = Arc::new(Hints::default());
    let m = CpuIndexManager::new(core(), Handle::current(), Some(hints.clone()));
    m.attach_index(index);
    let items = vec![(id(1), 0..10), (id(2), 40..50)];
    let dense = m.read_set(items.clone(), ReadPattern::Dense).await.unwrap();
    assert_eq!(dense, vec![b"a".to_vec(), b"b".to_vec()]);
    assert_eq!(* hints.0.lock(), vec![(0..50, Hint::Sequential)]);
    hints.0.lock().clear();
    m.read_set(items, ReadPattern::Scattered).await.unwrap();
    assert_eq!(* hints.0.lock(), vec![(0..10, Hint::Random), (40..50, Hint::Random)]);
    hints.0.lock().clear();
    assert!(m.read_set(Vec::new(), ReadPattern::Dense). await .unwrap().is_empty());
    assert!(hints.0.lock().is_empty());
}
#[tokio::test]
async fn cycle_without_an_index_is_unavailable() {
    let m = gpu(&Arc::default());
    assert!(matches!(m.cycle(). await, Err(IndexError::IndexUnavailable { .. })));
    assert!(
        matches!(m.resolve(& Scope::All, & ResolveCtx::default()),
        Err(IndexError::IndexUnavailable { .. }))
    );
}
#[tokio::test]
async fn cycle_reloads_foreign_changes_and_queues_removals() {
    let transfers = Arc::new(Transfers::default());
    let m = gpu(&transfers);
    let index = Arc::new(InMemoryIndex::new());
    m.attach_index(index.clone());
    let first = index.apply(id(1), b"a");
    let second = index.apply(id(2), b"b");
    {
        let mut residency = m.core.residency.lock();
        residency.insert(id(1), SLOT, first);
        residency.insert(id(2), SlotRef { segment: 0, slot: 1 }, second);
    }
    assert!(m.cycle(). await .unwrap().is_clean());
    assert_eq!(transfers.gathers.load(Ordering::SeqCst), 0);
    index.apply(id(1), b"changed");
    index.remove(id(2));
    let out = m.cycle().await.unwrap();
    assert_eq!(out.reload, [id(1)]);
    assert_eq!(out.removed, [id(2)]);
    assert_eq!(transfers.gathers.load(Ordering::SeqCst), 1);
    assert_eq!(m.pending_updates(), 1);
}
#[tokio::test]
async fn loaded_bytes_feed_the_evictor() {
    let m = gpu(&Arc::default());
    m.attach_index(Arc::new(InMemoryIndex::new()));
    let set = EntrySet::from_ids([id(1), id(2)]);
    let outcome = m.ensure(set, Access::Regular).await.unwrap();
    assert_eq!(outcome.bytes, 2048);
    assert_eq!(m.evictor.lock().tracked_bytes(), 2048);
}
