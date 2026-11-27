use super::*;
use std::sync::atomic::{AtomicUsize, Ordering};
use super::super::super::_5::_4::a::resource_requirements::{
    ExecutionEnvironment, ModelId, ResourceKind, ResourceRequirements,
};
use super::super::super::_5::_4::b::embedding_task::{EmbeddingMode, Precision};
use super::super::super::_5::_4::a::resource_requirements::IndexId;
use super::super::super::_5::_4::_3::a::region_handle::{Generation, RegionHandle};
use super::super::super::_5::_4::b::operation::{OperationKind, ScoredHitResult};
use super::super::super::_5::_4::_3::_2::a::arenas::ArenaKind;
use super::super::super::_5::_4::_3::_2::a::memory_access::{
    AccessError, AccessRegion, RegionBackend, TensorRef,
};
use super::super::super::_5::_4::_3::_2::b::task_model::TaskParams;
use super::super::super::_5::_4::_3::_2::_1::layout::{DType, Layout};
#[test]
fn chain_types_are_thread_safe() {
    fn check<T: Send + Sync>() {}
    fn check_clone<T: Send + Sync + Clone>() {}
    check_clone::<Collector2>();
    check_clone::<ChainKind>();
    check_clone::<StepOutput>();
    check::<ChainNode<TestOp, String, String>>();
}
#[derive(Debug)]
struct TestOp {
    index: usize,
    requirements: ResourceRequirements,
}
impl TestOp {
    fn new(index: usize) -> Self {
        Self {
            index,
            requirements: ResourceRequirements::new(
                ResourceKind::Encoder,
                ExecutionEnvironment::Any,
            ),
        }
    }
}
impl HasResourceRequirements for TestOp {
    fn resource_requirements(&self) -> &ResourceRequirements {
        &self.requirements
    }
}
impl HasOperationKind for TestOp {
    fn operation_kind(&self) -> OperationKind {
        OperationKind::Embedding
    }
}
type TestTask = Task<String, StepOutput, String>;
type Node = ChainNode<TestOp, String, String>;
type Log = Arc<Mutex<Vec<(usize, ChainInput)>>>;
fn claimed_task() -> TestTask {
    let task = Task::new("data".to_string(), TaskParams::default());
    task.claim().unwrap();
    task
}
fn context(id: u64) -> Arc<TaskContext<String>> {
    TaskContext::new(TaskId(id), 3, 1)
}
fn steps(
    count: usize,
    log: &Log,
    built: &Arc<AtomicUsize>,
) -> Vec<StepFactory<TestOp, String>> {
    (0..count)
        .map(|index| {
            let log = log.clone();
            let built = built.clone();
            let factory: StepFactory<TestOp, String> = Arc::new(move |input, _ctx| {
                built.fetch_add(1, Ordering::SeqCst);
                log.lock().unwrap().push((index, input.clone()));
                TestOp::new(index)
            });
            factory
        })
        .collect()
}
fn embedding_kind(texts: &[&str]) -> ChainKind {
    let texts: Vec<String> = texts.iter().map(|t| t.to_string()).collect();
    ChainKind::Embedding(
        EmbeddingTask::new(
            texts,
            ModelId::new("m"),
            EmbeddingMode::Passage,
            Precision::F32,
        ),
    )
}
fn similarity_kind(queries: Vec<Vec<f32>>, top_k: usize) -> ChainKind {
    ChainKind::Similarity {
        queries,
        params: SimilarityParams::new(IndexId::new("1")).with_top_k(top_k),
        report: ReportConfig {
            options: ReportOptions { temperature: 1.0 },
            source_link: Arc::new(|id| Some(format!("src/{}", id.0))),
        },
    }
}
fn inference_kind() -> ChainKind {
    ChainKind::Inference {
        query: "q".into(),
        passage: "p".into(),
    }
}
fn ram(data: RamData) -> StepOutput {
    StepOutput::Ram(data)
}
fn floats(v: &[f32]) -> StepOutput {
    ram(RamData::Floats(v.to_vec()))
}
fn start(
    kind: ChainKind,
    count: usize,
) -> (ChainStart<TestOp, String, String>, Log, Arc<AtomicUsize>, TestTask) {
    let log: Log = Arc::new(Mutex::new(Vec::new()));
    let built = Arc::new(AtomicUsize::new(0));
    let task = claimed_task();
    let chain = start_chain(kind, steps(count, &log, &built), context(42), task.clone());
    (chain, log, built, task)
}
fn run_chain(first: Node, outputs: &[StepOutput]) -> Vec<Node> {
    let mut nodes = Vec::new();
    let mut current = first;
    for output in outputs {
        current.set_output(output.clone()).unwrap();
        let mut next = current.advance().unwrap();
        nodes.push(current);
        match next.pop() {
            Some(node) => current = node,
            None => return nodes,
        }
    }
    nodes
}
fn data_of(input: &ChainInput) -> &StepOutput {
    match input {
        ChainInput::Data(d) => d,
        other => panic!("unexpected input {other:?}"),
    }
}
fn same_ram(a: &StepOutput, b: &RamData) -> bool {
    matches!(a, StepOutput::Ram(d) if d == b)
}
#[derive(Debug)]
struct Bytes(Mutex<Vec<u8>>);
impl RegionBackend for Bytes {
    fn len(&self) -> usize {
        self.0.lock().unwrap().len()
    }
    fn read_raw(&self, offset: usize, dst: &mut [u8]) -> Result<(), AccessError> {
        dst.copy_from_slice(&self.0.lock().unwrap()[offset..offset + dst.len()]);
        Ok(())
    }
    fn write_raw(&self, offset: usize, src: &[u8]) -> Result<(), AccessError> {
        self.0.lock().unwrap()[offset..offset + src.len()].copy_from_slice(src);
        Ok(())
    }
}
fn device_output() -> StepOutput {
    let region = AccessRegion::new(Arc::new(Bytes(Mutex::new(vec![0; 16]))));
    let handle = RegionHandle::new(ArenaKind::Workspace, 0, 16, Generation(1)).unwrap();
    let tensor = TensorRef::new(region, Layout::contiguous(&[4], DType::F32)).unwrap();
    StepOutput::Device(DeviceResult::new(handle, tensor))
}
fn store(chain: &ChainStart<TestOp, String, String>, output: StepOutput) {
    *chain.collector.slot.lock().unwrap() = Some(output);
}
#[test]
fn empty_chain_has_no_first_node_and_no_result() {
    let (chain, _, built, _) = start(inference_kind(), 0);
    assert!(chain.first.is_none());
    assert_eq!(built.load(Ordering::SeqCst), 0);
    assert_eq!(chain.collector.collect(), Err(ResultError3::MissingResult));
}
#[test]
fn only_the_first_node_is_built_at_start() {
    let (chain, log, built, _) = start(inference_kind(), 3);
    let first = chain.first.unwrap();
    assert_eq!(built.load(Ordering::SeqCst), 1);
    assert_eq!(first.operation().index, 0);
    assert_eq!(first.task_id(), TaskId(42));
    assert!(! first.materialize());
    assert!(first.ends_chain());
    assert!(first.has_continuation());
    assert_eq!(log.lock().unwrap().len(), 1);
}
#[test]
fn single_step_chain_materializes_first_node() {
    let (chain, _, _, _) = start(inference_kind(), 1);
    assert!(chain.first.unwrap().materialize());
}
#[test]
fn embedding_input_is_the_text_list() {
    let (chain, _, _, _) = start(embedding_kind(&["a", "b"]), 2);
    let first = chain.first.unwrap();
    let expected = RamData::TextList(vec!["a".into(), "b".into()]);
    assert!(same_ram(data_of(first.input()), & expected));
}
#[test]
fn embedding_input_may_be_empty() {
    let (chain, _, _, _) = start(embedding_kind(&[]), 1);
    let first = chain.first.unwrap();
    assert!(same_ram(data_of(first.input()), & RamData::TextList(Vec::new())));
}
#[test]
fn similarity_queries_are_concatenated_in_order() {
    let kind = similarity_kind(vec![vec![1.0, 2.0], vec![3.0], vec![]], 5);
    let (chain, _, _, _) = start(kind, 1);
    let first = chain.first.unwrap();
    assert!(same_ram(data_of(first.input()), & RamData::Floats(vec![1.0, 2.0, 3.0])));
}
#[test]
fn similarity_without_queries_gives_empty_floats() {
    let (chain, _, _, _) = start(similarity_kind(Vec::new(), 5), 1);
    let first = chain.first.unwrap();
    assert!(same_ram(data_of(first.input()), & RamData::Floats(Vec::new())));
}
#[test]
fn inference_input_is_the_text_pair_in_order() {
    let (chain, _, _, _) = start(inference_kind(), 1);
    match chain.first.unwrap().input() {
        ChainInput::TextPair(q, p) => assert_eq!((q.as_str(), p.as_str()), ("q", "p")),
        other => panic!("unexpected input {other:?}"),
    }
}
#[test]
fn follow_up_is_built_from_the_actual_output() {
    let (chain, log, built, task) = start(inference_kind(), 3);
    let first = chain.first.unwrap();
    first.set_output(floats(&[7.0, 8.0])).unwrap();
    let next = first.advance().unwrap();
    assert_eq!(next.len(), 1);
    assert_eq!(built.load(Ordering::SeqCst), 2);
    let second = &next[0];
    assert_eq!(second.operation().index, 1);
    assert_eq!(second.task_id(), TaskId(42));
    assert!(! second.materialize());
    assert!(second.has_continuation() && second.ends_chain());
    assert!(same_ram(data_of(second.input()), & RamData::Floats(vec![7.0, 8.0])));
    let recorded = log.lock().unwrap();
    assert_eq!(recorded[1].0, 1);
    assert!(same_ram(data_of(& recorded[1].1), & RamData::Floats(vec![7.0, 8.0])));
    assert!(task.result().is_none());
    assert!(chain.collector.collect().is_err());
}
#[test]
fn last_node_is_materialized_and_stores_the_result() {
    let (chain, _, built, task) = start(inference_kind(), 3);
    let nodes = run_chain(
        chain.first.unwrap(),
        &[floats(&[1.0]), floats(&[2.0]), floats(&[3.0, 4.0])],
    );
    assert_eq!(nodes.len(), 3);
    assert_eq!(built.load(Ordering::SeqCst), 3);
    let flags: Vec<bool> = nodes.iter().map(|n| n.materialize()).collect();
    assert_eq!(flags, [false, false, true]);
    assert!(nodes.iter().all(| n | n.task_id() == TaskId(42)));
    match task.result() {
        Some(Ok(StepOutput::Ram(RamData::Floats(v)))) => assert_eq!(v, & vec![3.0, 4.0]),
        _ => panic!("task not completed with the last output"),
    }
    assert_eq!(
        chain.collector.collect(), Ok(CollectedResult::Task(TaskResult::Scores(vec![3.0,
        4.0])))
    );
}
#[test]
fn non_final_steps_leave_the_slot_empty() {
    let (chain, _, _, _) = start(inference_kind(), 3);
    let first = chain.first.unwrap();
    first.set_output(floats(&[1.0])).unwrap();
    let next = first.advance().unwrap();
    next[0].set_output(floats(&[2.0])).unwrap();
    let _ = next[0].advance().unwrap();
    assert_eq!(chain.collector.collect(), Err(ResultError3::MissingResult));
}
#[test]
fn device_output_is_handed_over_unchanged() {
    let (chain, _, _, _) = start(inference_kind(), 2);
    let first = chain.first.unwrap();
    first.set_output(device_output()).unwrap();
    let next = first.advance().unwrap();
    assert!(matches!(data_of(next[0].input()), StepOutput::Device(_)));
}
#[test]
fn repeated_handler_calls_build_equivalent_nodes() {
    let (chain, _, built, _) = start(inference_kind(), 2);
    let first = chain.first.unwrap();
    first.set_output(floats(&[1.0])).unwrap();
    let a = first.advance().unwrap();
    let b = first.advance().unwrap();
    assert_eq!(built.load(Ordering::SeqCst), 3);
    assert_eq!(a[0].operation().index, b[0].operation().index);
    assert_eq!(a[0].materialize(), b[0].materialize());
    assert!(same_ram(data_of(a[0].input()), & RamData::Floats(vec![1.0])));
    assert!(same_ram(data_of(b[0].input()), & RamData::Floats(vec![1.0])));
}
#[test]
fn filled_slot_stays_unchanged_when_last_handler_runs_again() {
    let (chain, _, _, _) = start(inference_kind(), 2);
    let first = chain.first.unwrap();
    first.set_output(floats(&[0.0])).unwrap();
    let mut a = first.advance().unwrap();
    let mut b = first.advance().unwrap();
    let (a, b) = (
        a.pop().unwrap().with_ends_chain(false),
        b.pop().unwrap().with_ends_chain(false),
    );
    a.set_output(floats(&[1.0])).unwrap();
    b.set_output(floats(&[2.0])).unwrap();
    assert!(a.advance().unwrap().is_empty());
    assert!(b.advance().unwrap().is_empty());
    assert_eq!(
        chain.collector.collect(),
        Ok(CollectedResult::Task(TaskResult::Scores(vec![1.0])))
    );
}
#[test]
fn collect_takes_the_result_once() {
    let (chain, _, _, _) = start(inference_kind(), 1);
    store(&chain, floats(&[0.5]));
    assert!(chain.collector.collect().is_ok());
    assert_eq!(chain.collector.collect(), Err(ResultError3::MissingResult));
}
#[test]
fn clones_share_the_same_slot() {
    let (chain, _, _, _) = start(inference_kind(), 1);
    let other = chain.collector.clone();
    store(&chain, floats(&[0.5]));
    assert!(other.collect().is_ok());
    assert_eq!(chain.collector.collect(), Err(ResultError3::MissingResult));
}
#[test]
fn failed_conversion_still_consumes_the_result() {
    let (chain, _, _, _) = start(inference_kind(), 1);
    store(&chain, ram(RamData::HitList(Vec::new())));
    assert!(chain.collector.collect().is_err());
    assert_eq!(chain.collector.collect(), Err(ResultError3::MissingResult));
}
#[test]
fn concurrent_collect_has_exactly_one_winner() {
    let (chain, _, _, _) = start(inference_kind(), 1);
    store(&chain, floats(&[1.0]));
    let wins = Arc::new(AtomicUsize::new(0));
    let handles: Vec<_> = (0..8)
        .map(|_| {
            let collector = chain.collector.clone();
            let wins = wins.clone();
            std::thread::spawn(move || match collector.collect() {
                Ok(_) => {
                    wins.fetch_add(1, Ordering::SeqCst);
                }
                Err(e) => assert_eq!(e, ResultError3::MissingResult),
            })
        })
        .collect();
    for h in handles {
        h.join().unwrap();
    }
    assert_eq!(wins.load(Ordering::SeqCst), 1);
}
#[test]
fn device_result_is_not_materialized() {
    let (chain, _, _, _) = start(inference_kind(), 1);
    store(&chain, device_output());
    assert_eq!(chain.collector.collect(), Err(ResultError3::NotMaterialized));
}
#[test]
fn embeddings_are_split_in_order() {
    let (chain, _, _, _) = start(embedding_kind(&["a", "b"]), 1);
    store(&chain, floats(&[1.0, 2.0, 3.0, 4.0]));
    assert_eq!(
        chain.collector.collect(),
        Ok(CollectedResult::Task(TaskResult::Embeddings(vec![vec![1.0, 2.0], vec![3.0,
        4.0]])))
    );
}
#[test]
fn embedding_split_rejects_uneven_values() {
    let (chain, _, _, _) = start(embedding_kind(&["a", "b"]), 1);
    store(&chain, floats(&[1.0, 2.0, 3.0]));
    assert_eq!(
        chain.collector.collect(), Err(ResultError3::EmbeddingSplit { values : 3, texts :
        2 })
    );
}
#[test]
fn embedding_split_rejects_zero_texts() {
    let (chain, _, _, _) = start(embedding_kind(&[]), 1);
    store(&chain, floats(&[]));
    assert_eq!(
        chain.collector.collect(), Err(ResultError3::EmbeddingSplit { values : 0, texts :
        0 })
    );
}
#[test]
fn embedding_with_empty_values_gives_empty_vectors() {
    let (chain, _, _, _) = start(embedding_kind(&["a", "b"]), 1);
    store(&chain, floats(&[]));
    assert_eq!(
        chain.collector.collect(),
        Ok(CollectedResult::Task(TaskResult::Embeddings(vec![Vec::new(), Vec::new()])))
    );
}
#[test]
fn embedding_rejects_other_shapes() {
    let (chain, _, _, _) = start(embedding_kind(&["a"]), 1);
    store(&chain, ram(RamData::HitList(Vec::new())));
    assert_eq!(
        chain.collector.collect(), Err(ResultError3::KindMismatch { task_kind :
        "embedding", found : DataShape::HitList })
    );
}
#[test]
fn similarity_hit_list_becomes_hits() {
    let (chain, _, _, _) = start(similarity_kind(vec![vec![1.0]], 5), 1);
    store(&chain, ram(RamData::HitList(vec![(EntryId(7), 0.9), (EntryId(3), 0.1)])));
    let expected = TaskResult::Hits(
        vec![
            Hit2 { id : EntryId(7), score : 0.9, stale : false }, Hit2 { id : EntryId(3),
            score : 0.1, stale : false },
        ],
    );
    assert_eq!(chain.collector.collect(), Ok(CollectedResult::Task(expected)));
}
#[test]
fn similarity_scored_hits_become_a_ranked_report() {
    let (chain, _, _, _) = start(similarity_kind(vec![vec![1.0]], 2), 1);
    let hit = |id, score| Hit2 {
        id: EntryId(id),
        score,
        stale: false,
    };
    let result = ScoredHitResult {
        scored_count: 3,
        dropped_count: 0,
        hits: vec![hit(1, 0.2), hit(2, 0.8), hit(3, 0.5)],
    };
    store(&chain, ram(RamData::ScoredHitResult(result)));
    match chain.collector.collect() {
        Ok(CollectedResult::Report(report)) => {
            assert_eq!(report.task_id, TaskId(42));
            assert_eq!(report.candidate_count, 3);
            let ids: Vec<u64> = report.hits.iter().map(|h| h.entry_id.0).collect();
            assert_eq!(ids, [2, 3]);
            assert_eq!(report.hits[0].source_link.as_deref(), Some("src/2"));
            assert_eq!(report.params.top_k, 2);
        }
        other => panic!("unexpected result {other:?}"),
    }
}
#[test]
fn similarity_rejects_other_shapes() {
    let (chain, _, _, _) = start(similarity_kind(Vec::new(), 1), 1);
    store(&chain, floats(&[1.0]));
    assert_eq!(
        chain.collector.collect(), Err(ResultError3::KindMismatch { task_kind :
        "similarity", found : DataShape::Floats })
    );
}
#[test]
fn inference_floats_become_scores() {
    let (chain, _, _, _) = start(inference_kind(), 1);
    store(&chain, floats(&[0.25, 0.75]));
    assert_eq!(
        chain.collector.collect(), Ok(CollectedResult::Task(TaskResult::Scores(vec![0.25,
        0.75])))
    );
}
#[test]
fn inference_rejects_other_shapes() {
    let (chain, _, _, _) = start(inference_kind(), 1);
    store(&chain, ram(RamData::TextList(Vec::new())));
    assert_eq!(
        chain.collector.collect(), Err(ResultError3::KindMismatch { task_kind :
        "inference", found : DataShape::TextList })
    );
}
#[test]
fn errors_display_and_are_configuration_errors() {
    let errors = [
        ResultError3::MissingResult,
        ResultError3::NotMaterialized,
        ResultError3::KindMismatch {
            task_kind: "inference",
            found: DataShape::HitList,
        },
        ResultError3::EmbeddingSplit {
            values: 5,
            texts: 2,
        },
        ResultError3::Report(ReportError::MissingSearchParams),
    ];
    for e in &errors {
        assert!(e.is_configuration_error());
        assert!(! e.to_string().is_empty());
    }
    assert!(errors[2].to_string().contains("inference"));
    let split = errors[3].to_string();
    assert!(split.contains('5') && split.contains('2'));
}
