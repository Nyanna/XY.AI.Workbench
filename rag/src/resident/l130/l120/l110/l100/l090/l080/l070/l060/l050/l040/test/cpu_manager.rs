use super::super::l030::l020::a::index_contract::{
    IndexError, PayloadRef, SharedBytes, Stamp, StampObserver, StampSignal,
};
use super::super::l030::l020::l010::layout::DType;
use super::*;
static_assertions::assert_impl_all!(CpuManager : Send, Sync);
type Call = (u64, u64, Advice);
#[derive(Default)]
struct RecordingAdvisor {
    calls: Mutex<Vec<Call>>,
    fail: bool,
}
impl RecordingAdvisor {
    fn calls(&self) -> Vec<Call> {
        self.calls.lock().unwrap().clone()
    }
}
impl MemoryAdvisor for RecordingAdvisor {
    fn advise(&self, offset: u64, len: u64, advice: Advice) -> Result<(), String> {
        self.calls.lock().unwrap().push((offset, len, advice));
        if self.fail { Err("rejected".to_string()) } else { Ok(()) }
    }
}
struct MockIndex {
    entries: HashMap<u64, (Vec<u8>, Vec<u8>)>,
    methods: Vec<AccessMethod>,
    ranges: Result<Vec<ByteRange>, IndexError>,
    panic_on: Option<u64>,
    reads: Mutex<Vec<u64>>,
    range_calls: Mutex<Vec<(u64, u64)>>,
    signal: Arc<StampSignal>,
}
impl MockIndex {
    fn new(ids: &[u64]) -> MockIndex {
        MockIndex {
            entries: ids
                .iter()
                .map(|&i| (i, (vec![i as u8; 4], vec![i as u8 + 100])))
                .collect(),
            methods: Vec::new(),
            ranges: Err(IndexError::Unsupported(AccessMethod::Range)),
            panic_on: None,
            reads: Mutex::new(Vec::new()),
            range_calls: Mutex::new(Vec::new()),
            signal: StampSignal::new(Stamp(0)),
        }
    }
    fn with_ranges(mut self, ranges: Vec<ByteRange>) -> MockIndex {
        self.methods = vec![AccessMethod::Range];
        self.ranges = Ok(ranges);
        self
    }
}
impl Index for MockIndex {
    fn dtype(&self) -> DType {
        DType::F32
    }
    fn access_methods(&self) -> &[AccessMethod] {
        &self.methods
    }
    fn stamp(&self, _id: EntryId) -> Option<Stamp> {
        None
    }
    fn stamps(&self) -> Vec<(EntryId, Stamp)> {
        Vec::new()
    }
    fn read(&self, id: EntryId) -> Payload {
        if self.panic_on == Some(id.0) {
            panic!("read failure");
        }
        self.reads.lock().unwrap().push(id.0);
        match self.entries.get(&id.0) {
            Some((v, m)) => {
                Payload {
                    vector: SharedBytes::from(v.clone()),
                    meta: SharedBytes::from(m.clone()),
                }
            }
            None => Payload::default(),
        }
    }
    fn write(
        &self,
        _id: EntryId,
        _payload: PayloadRef<'_>,
    ) -> Result<Stamp, IndexError> {
        Err(IndexError::Backend("read only".to_string()))
    }
    fn delete(&self, _id: EntryId) -> Result<Option<Stamp>, IndexError> {
        Err(IndexError::Backend("read only".to_string()))
    }
    fn subscribe(&self) -> StampObserver {
        self.signal.observer()
    }
    fn byte_ranges(
        &self,
        first: EntryId,
        last: EntryId,
    ) -> Result<Vec<ByteRange>, IndexError> {
        self.range_calls.lock().unwrap().push((first.0, last.0));
        self.ranges.clone()
    }
}
fn br(first: u64, count: u64, offset: u64, len: u64) -> ByteRange {
    ByteRange {
        first: EntryId(first),
        count,
        offset,
        len,
    }
}
fn lazy(index: Option<Arc<MockIndex>>) -> LazyIndex {
    let slot: LazyIndex = Arc::new(OnceLock::new());
    if let Some(index) = index {
        let dynamic: Arc<dyn Index> = index;
        assert!(slot.set(dynamic).is_ok());
    }
    slot
}
fn manager(
    index: Arc<MockIndex>,
    advisor: &Arc<RecordingAdvisor>,
    idle_after: Duration,
) -> CpuManager {
    let advisor: Arc<dyn MemoryAdvisor> = advisor.clone();
    CpuManager::new(lazy(Some(index)), Some(advisor), idle_after)
}
fn owned(i: u64) -> OwnedPayload {
    OwnedPayload {
        vector: vec![i as u8; 4],
        meta: vec![i as u8 + 100],
    }
}
fn err(text: &str) -> TransferError {
    TransferError(text.to_string())
}
#[test]
fn transfer_error_displays_its_text() {
    assert_eq!(err("x y").to_string(), "x y");
}
#[test]
fn plan_without_ranges_is_none() {
    assert_eq!(plan_hints(& []), HintPlan::None);
    assert_eq!(plan_hints(& [None, Some((5, 0))]), HintPlan::None);
}
#[test]
fn plan_adjacent_ranges_is_dense_over_span() {
    let plan = plan_hints(&[Some((0, 10)), Some((10, 10))]);
    assert_eq!(plan, HintPlan::Dense((0, 20)));
}
#[test]
fn plan_half_coverage_is_dense() {
    let plan = plan_hints(&[Some((0, 5)), Some((15, 5))]);
    assert_eq!(plan, HintPlan::Dense((0, 20)));
}
#[test]
fn plan_below_half_coverage_is_scattered() {
    let plan = plan_hints(&[Some((1000, 10)), None, Some((0, 10)), Some((1000, 10))]);
    assert_eq!(plan, HintPlan::Scattered(vec![(1000, 10), (0, 10)]));
}
#[test]
fn plan_counts_overlaps_once() {
    let plan = plan_hints(
        &[Some((0, 10)), Some((5, 10)), Some((0, 10)), Some((90, 10))],
    );
    assert_eq!(plan, HintPlan::Scattered(vec![(0, 10), (5, 10), (90, 10)]));
    let plan = plan_hints(
        &[Some((0, 10)), Some((5, 10)), Some((0, 10)), Some((20, 10))],
    );
    assert_eq!(plan, HintPlan::Dense((0, 30)));
}
#[test]
fn plan_single_range_is_dense() {
    assert_eq!(plan_hints(& [Some((7, 3))]), HintPlan::Dense((7, 3)));
}
#[test]
fn read_direct_without_index_is_none() {
    let m = CpuManager::new(lazy(None), None, Duration::ZERO);
    assert!(m.read_direct(EntryId(1)).is_none());
}
#[test]
fn read_direct_reads_on_calling_thread() {
    let index = Arc::new(MockIndex::new(&[1]));
    let m = CpuManager::new(lazy(Some(index.clone())), None, Duration::ZERO);
    let p = m.read_direct(EntryId(1)).unwrap();
    assert_eq!(&* p.vector, & [1u8; 4] [..]);
    assert!(m.read_direct(EntryId(9)).unwrap().vector.is_empty());
    assert_eq!(* index.reads.lock().unwrap(), vec![1, 9]);
}
#[test]
fn later_filled_slot_becomes_effective() {
    let slot = lazy(None);
    let m = CpuManager::new(slot.clone(), None, Duration::ZERO);
    assert!(m.read_direct(EntryId(1)).is_none());
    let index: Arc<dyn Index> = Arc::new(MockIndex::new(&[1]));
    assert!(slot.set(index).is_ok());
    assert!(m.read_direct(EntryId(1)).is_some());
}
#[tokio::test]
async fn read_entry_copies_payload() {
    let m = CpuManager::new(
        lazy(Some(Arc::new(MockIndex::new(&[3])))),
        None,
        Duration::ZERO,
    );
    assert_eq!(m.read_entry(EntryId(3), None). await, Ok(owned(3)));
}
#[tokio::test]
async fn read_entry_missing_is_empty_not_error() {
    let m = CpuManager::new(
        lazy(Some(Arc::new(MockIndex::new(&[])))),
        None,
        Duration::ZERO,
    );
    assert_eq!(m.read_entry(EntryId(3), None). await, Ok(OwnedPayload::default()));
}
#[tokio::test]
async fn read_entry_hints_will_need_and_tracks() {
    let advisor = Arc::new(RecordingAdvisor::default());
    let m = manager(Arc::new(MockIndex::new(&[3])), &advisor, Duration::ZERO);
    m.read_entry(EntryId(3), Some(br(3, 1, 64, 32))).await.unwrap();
    assert_eq!(advisor.calls(), vec![(64, 32, Advice::WillNeed)]);
    assert_eq!(m.mark_idle_cold(Instant::now()), 1);
    assert_eq!(advisor.calls() [1], (64, 32, Advice::Cold));
}
#[tokio::test]
async fn read_entry_ignores_empty_hint() {
    let advisor = Arc::new(RecordingAdvisor::default());
    let m = manager(Arc::new(MockIndex::new(&[3])), &advisor, Duration::ZERO);
    m.read_entry(EntryId(3), Some(br(3, 1, 64, 0))).await.unwrap();
    assert!(advisor.calls().is_empty());
    assert_eq!(m.mark_idle_cold(Instant::now()), 0);
}
#[tokio::test]
async fn read_entry_not_attached_fails() {
    let m = CpuManager::new(lazy(None), None, Duration::ZERO);
    assert_eq!(
        m.read_entry(EntryId(1), None). await, Err(err("Index nicht angehängt"))
    );
}
#[tokio::test]
async fn read_entry_panicking_task_reports_abort() {
    let mut index = MockIndex::new(&[1]);
    index.panic_on = Some(1);
    let m = CpuManager::new(lazy(Some(Arc::new(index))), None, Duration::ZERO);
    assert_eq!(m.read_entry(EntryId(1), None). await, Err(err("Lese-Task abgebrochen")));
}
#[tokio::test]
async fn read_entry_survives_advisor_failure() {
    let advisor = Arc::new(RecordingAdvisor {
        fail: true,
        ..Default::default()
    });
    let m = manager(Arc::new(MockIndex::new(&[3])), &advisor, Duration::ZERO);
    assert_eq!(m.read_entry(EntryId(3), Some(br(3, 1, 0, 8))). await, Ok(owned(3)));
}
#[tokio::test]
async fn read_entries_empty_is_ok_even_when_not_attached() {
    let advisor = Arc::new(RecordingAdvisor::default());
    let advisor_dyn: Arc<dyn MemoryAdvisor> = advisor.clone();
    let m = CpuManager::new(lazy(None), Some(advisor_dyn), Duration::ZERO);
    assert_eq!(m.read_entries(& []). await, Ok(vec![]));
    assert_eq!(m.read_entries_auto(& []). await, Ok(vec![]));
    assert!(advisor.calls().is_empty());
}
#[tokio::test]
async fn read_entries_not_attached_fails() {
    let m = CpuManager::new(lazy(None), None, Duration::ZERO);
    assert_eq!(
        m.read_entries(& [(EntryId(1), None)]). await, Err(err("Index nicht angehängt"))
    );
    assert_eq!(
        m.read_entries_auto(& [EntryId(1)]). await, Err(err("Index nicht angehängt"))
    );
}
#[tokio::test]
async fn read_entries_keeps_order_duplicates_and_missing() {
    let index = Arc::new(MockIndex::new(&[1, 2, 3]));
    let m = CpuManager::new(lazy(Some(index.clone())), None, Duration::ZERO);
    let ids = [3u64, 1, 9, 3, 2];
    let entries: Vec<_> = ids.iter().map(|&i| (EntryId(i), None)).collect();
    let out = m.read_entries(&entries).await.unwrap();
    assert_eq!(
        out, vec![owned(3), owned(1), OwnedPayload::default(), owned(3), owned(2)]
    );
    assert_eq!(* index.reads.lock().unwrap(), vec![3, 1, 9, 3, 2]);
}
#[tokio::test]
async fn read_entries_dense_pattern_hints_span_sequentially() {
    let advisor = Arc::new(RecordingAdvisor::default());
    let m = manager(Arc::new(MockIndex::new(&[1, 2])), &advisor, Duration::ZERO);
    let entries = [
        (EntryId(1), Some(br(1, 1, 0, 10))),
        (EntryId(2), Some(br(2, 1, 10, 10))),
        (EntryId(1), Some(br(1, 1, 0, 10))),
    ];
    m.read_entries(&entries).await.unwrap();
    assert_eq!(advisor.calls(), vec![(0, 20, Advice::Sequential)]);
    assert_eq!(m.mark_idle_cold(Instant::now()), 1);
}
#[tokio::test]
async fn read_entries_scattered_pattern_hints_each_range_randomly() {
    let advisor = Arc::new(RecordingAdvisor::default());
    let m = manager(Arc::new(MockIndex::new(&[1, 2])), &advisor, Duration::ZERO);
    let entries = [
        (EntryId(1), Some(br(1, 1, 0, 10))),
        (EntryId(2), Some(br(2, 1, 1000, 10))),
        (EntryId(3), None),
        (EntryId(2), Some(br(2, 1, 1000, 10))),
    ];
    let out = m.read_entries(&entries).await.unwrap();
    assert_eq!(out.len(), 4);
    assert_eq!(
        advisor.calls(), vec![(0, 10, Advice::Random), (1000, 10, Advice::Random)]
    );
    assert_eq!(m.mark_idle_cold(Instant::now()), 2);
}
#[tokio::test]
async fn read_entries_without_ranges_sets_no_hint() {
    let advisor = Arc::new(RecordingAdvisor::default());
    let m = manager(Arc::new(MockIndex::new(&[1])), &advisor, Duration::ZERO);
    m.read_entries(&[(EntryId(1), None), (EntryId(2), Some(br(2, 1, 5, 0)))])
        .await
        .unwrap();
    assert!(advisor.calls().is_empty());
}
#[tokio::test]
async fn read_entries_panicking_task_reports_abort() {
    let mut index = MockIndex::new(&[1, 2]);
    index.panic_on = Some(2);
    let m = CpuManager::new(lazy(Some(Arc::new(index))), None, Duration::ZERO);
    assert_eq!(
        m.read_entries(& [(EntryId(1), None), (EntryId(2), None)]). await,
        Err(err("Lese-Task abgebrochen"))
    );
}
#[tokio::test]
async fn auto_maps_entries_to_byte_ranges() {
    let advisor = Arc::new(RecordingAdvisor::default());
    let index = Arc::new(
        MockIndex::new(&[10, 11, 12, 20])
            .with_ranges(vec![br(10, 3, 0, 30), br(20, 1, 1000, 10),]),
    );
    let m = manager(index.clone(), &advisor, Duration::ZERO);
    let out = m
        .read_entries_auto(&[EntryId(20), EntryId(11), EntryId(10), EntryId(15)])
        .await
        .unwrap();
    assert_eq!(out, vec![owned(20), owned(11), owned(10), OwnedPayload::default()]);
    assert_eq!(* index.range_calls.lock().unwrap(), vec![(10, 20)]);
    assert_eq!(
        advisor.calls(), vec![(1000, 10, Advice::Random), (0, 30, Advice::Random)]
    );
}
#[tokio::test]
async fn auto_dense_range_hints_whole_span() {
    let advisor = Arc::new(RecordingAdvisor::default());
    let index = Arc::new(
        MockIndex::new(&[1, 2]).with_ranges(vec![br(1, 1, 0, 10), br(2, 1, 10, 10)]),
    );
    let m = manager(index, &advisor, Duration::ZERO);
    m.read_entries_auto(&[EntryId(1), EntryId(2)]).await.unwrap();
    assert_eq!(advisor.calls(), vec![(0, 20, Advice::Sequential)]);
}
#[tokio::test]
async fn auto_without_range_method_reads_without_hint() {
    let advisor = Arc::new(RecordingAdvisor::default());
    let mut index = MockIndex::new(&[1]).with_ranges(vec![br(1, 1, 0, 10)]);
    index.methods = Vec::new();
    let index = Arc::new(index);
    let m = manager(index.clone(), &advisor, Duration::ZERO);
    assert_eq!(m.read_entries_auto(& [EntryId(1)]). await, Ok(vec![owned(1)]));
    assert!(index.range_calls.lock().unwrap().is_empty());
    assert!(advisor.calls().is_empty());
}
#[tokio::test]
async fn auto_range_lookup_error_reads_without_hint() {
    let advisor = Arc::new(RecordingAdvisor::default());
    let mut index = MockIndex::new(&[1]);
    index.methods = vec![AccessMethod::Range];
    index.ranges = Err(IndexError::Backend("boom".to_string()));
    let m = manager(Arc::new(index), &advisor, Duration::ZERO);
    assert_eq!(m.read_entries_auto(& [EntryId(1)]). await, Ok(vec![owned(1)]));
    assert!(advisor.calls().is_empty());
}
#[tokio::test]
async fn auto_range_containment_is_exclusive_at_end() {
    let advisor = Arc::new(RecordingAdvisor::default());
    let index = Arc::new(MockIndex::new(&[1, 2]).with_ranges(vec![br(1, 1, 0, 10)]));
    let m = manager(index, &advisor, Duration::ZERO);
    m.read_entries_auto(&[EntryId(1), EntryId(2)]).await.unwrap();
    assert_eq!(advisor.calls(), vec![(0, 10, Advice::Sequential)]);
}
#[test]
fn secure_residency_hints_and_ignores_empty() {
    let advisor = Arc::new(RecordingAdvisor::default());
    let m = manager(Arc::new(MockIndex::new(&[])), &advisor, Duration::ZERO);
    m.secure_residency(br(0, 1, 8, 16));
    m.secure_residency(br(0, 0, 8, 0));
    assert_eq!(advisor.calls(), vec![(8, 16, Advice::WillNeed)]);
}
#[test]
fn mark_idle_cold_respects_threshold() {
    let advisor = Arc::new(RecordingAdvisor::default());
    let m = manager(Arc::new(MockIndex::new(&[])), &advisor, Duration::from_secs(10));
    m.secure_residency(br(0, 1, 0, 8));
    assert_eq!(m.mark_idle_cold(Instant::now()), 0);
    assert_eq!(m.mark_idle_cold(Instant::now() + Duration::from_secs(9)), 0);
    let later = Instant::now() + Duration::from_secs(10);
    assert_eq!(m.mark_idle_cold(later), 1);
    assert_eq!(advisor.calls().last(), Some(& (0, 8, Advice::Cold)));
    assert_eq!(m.mark_idle_cold(later), 0);
}
#[test]
fn mark_idle_cold_refreshes_on_reuse() {
    let advisor = Arc::new(RecordingAdvisor::default());
    let m = manager(Arc::new(MockIndex::new(&[])), &advisor, Duration::from_secs(10));
    m.secure_residency(br(0, 1, 0, 8));
    std::thread::sleep(Duration::from_millis(20));
    let first_use = Instant::now();
    m.secure_residency(br(0, 1, 0, 8));
    assert_eq!(m.mark_idle_cold(first_use + Duration::from_secs(10)), 0);
}
#[test]
fn mark_idle_cold_drops_range_even_when_advisor_rejects() {
    let advisor = Arc::new(RecordingAdvisor {
        fail: true,
        ..Default::default()
    });
    let m = manager(Arc::new(MockIndex::new(&[])), &advisor, Duration::ZERO);
    m.secure_residency(br(0, 1, 0, 8));
    m.secure_residency(br(0, 1, 100, 8));
    assert_eq!(m.mark_idle_cold(Instant::now()), 2);
    assert_eq!(m.mark_idle_cold(Instant::now()), 0);
}
#[test]
fn without_advisor_nothing_happens() {
    let m = CpuManager::new(lazy(None), None, Duration::ZERO);
    m.secure_residency(br(0, 1, 0, 8));
    assert_eq!(m.mark_idle_cold(Instant::now()), 0);
}
#[tokio::test]
async fn concurrent_reads_are_supported() {
    let ids: Vec<u64> = (0..16).collect();
    let advisor = Arc::new(RecordingAdvisor::default());
    let m = Arc::new(manager(Arc::new(MockIndex::new(&ids)), &advisor, Duration::ZERO));
    let mut tasks = Vec::new();
    for i in 0..16u64 {
        let m = m.clone();
        tasks
            .push(
                tokio::spawn(async move {
                    let a = m
                        .read_entry(EntryId(i), Some(br(i, 1, i * 8, 8)))
                        .await
                        .unwrap();
                    let b = m.read_entries_auto(&[EntryId(i)]).await.unwrap();
                    (a, b)
                }),
            );
    }
    for (i, task) in tasks.into_iter().enumerate() {
        let (a, b) = task.await.unwrap();
        assert_eq!(a, owned(i as u64));
        assert_eq!(b, vec![owned(i as u64)]);
    }
    m.mark_idle_cold(Instant::now());
}
use super::super::l030::a::file_index::{FileIndex, IndexHeader, OpenOptions};
fn file_index_with(ids: &[u64]) -> Arc<FileIndex> {
    let path = std::env::temp_dir()
        .join(format!("cpu_manager_absent_{}.parquet", std::process::id()));
    let header = IndexHeader {
        model_id: "m".to_string(),
        dimension: 2,
        dtype: DType::F32,
    };
    let index = Arc::new(FileIndex::open(path, header, OpenOptions::default()).unwrap());
    for &i in ids {
        let vector = [i as u8; 8];
        let meta = [i as u8 + 100];
        index
            .write(
                EntryId(i),
                PayloadRef {
                    vector: &vector,
                    meta: &meta,
                },
            )
            .unwrap();
    }
    index
}
fn attached(index: Arc<FileIndex>) -> LazyIndex {
    let slot: LazyIndex = Arc::new(OnceLock::new());
    let dynamic: Arc<dyn Index> = index;
    assert!(slot.set(dynamic).is_ok());
    slot
}
#[tokio::test]
async fn file_index_reads_without_hints_in_auto_mode() {
    let advisor = Arc::new(RecordingAdvisor::default());
    let advisor_dyn: Arc<dyn MemoryAdvisor> = advisor.clone();
    let m = CpuManager::new(
        attached(file_index_with(&[1, 2])),
        Some(advisor_dyn),
        Duration::ZERO,
    );
    let out = m.read_entries_auto(&[EntryId(2), EntryId(7), EntryId(1)]).await.unwrap();
    assert_eq!(out.len(), 3);
    assert_eq!(out[0].vector, vec![2u8; 8]);
    assert_eq!(out[0].meta, vec![102u8]);
    assert_eq!(out[1], OwnedPayload::default());
    assert_eq!(out[2].vector, vec![1u8; 8]);
    assert!(advisor.calls().is_empty());
}
#[tokio::test]
async fn file_index_serves_single_and_direct_reads() {
    let m = CpuManager::new(attached(file_index_with(&[4])), None, Duration::ZERO);
    let single = m.read_entry(EntryId(4), None).await.unwrap();
    assert_eq!(single.vector, vec![4u8; 8]);
    assert_eq!(&* m.read_direct(EntryId(4)).unwrap().meta, & [104u8] [..]);
    assert_eq!(m.read_entry(EntryId(5), None). await, Ok(OwnedPayload::default()));
}
