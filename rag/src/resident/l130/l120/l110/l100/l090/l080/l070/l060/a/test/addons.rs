use super::*;
use std::sync::Mutex;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::a::similarity_report::ReportHit;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::b::similarity_task::{
    SimilarityHit, SimilarityOutput,
};
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::a::resource_requirements::IndexId;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::b::task_context::TaskId;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::a::index_contract::EntryId;
#[derive(Debug, Clone)]
enum Data {
    Embed(EmbeddingTask),
    Sim(SimilarityJob),
    Num(u32),
}
impl From<EmbeddingTask> for Data {
    fn from(t: EmbeddingTask) -> Self {
        Data::Embed(t)
    }
}
impl From<SimilarityJob> for Data {
    fn from(j: SimilarityJob) -> Self {
        Data::Sim(j)
    }
}
#[derive(Debug, Clone)]
enum Out {
    Vectors(Vec<Vec<f32>>),
    Report(SimilarityReport),
    Num(u32),
}
impl AddonOutput for Out {
    fn embeddings(&self) -> Option<&[Vec<f32>]> {
        match self {
            Out::Vectors(v) => Some(v),
            _ => None,
        }
    }
    fn similarity_report(&self) -> Option<&SimilarityReport> {
        match self {
            Out::Report(r) => Some(r),
            _ => None,
        }
    }
}
enum Reply {
    Done(Out),
    Fail(&'static str),
    Cancel,
}
type Respond = Box<dyn Fn(&Data, usize) -> Reply + Send + Sync>;
struct MockQueue {
    respond: Respond,
    fail_at: Option<usize>,
    log: Mutex<Vec<(Data, TaskParams)>>,
}
impl MockQueue {
    fn new(respond: impl Fn(&Data, usize) -> Reply + Send + Sync + 'static) -> Self {
        MockQueue {
            respond: Box::new(respond),
            fail_at: None,
            log: Mutex::new(Vec::new()),
        }
    }
    fn failing_at(mut self, n: usize) -> Self {
        self.fail_at = Some(n);
        self
    }
    fn logged(&self) -> Vec<(Data, TaskParams)> {
        self.log.lock().unwrap().clone()
    }
}
impl Enqueue<Data, Out, String> for MockQueue {
    fn enqueue(
        &self,
        data: Data,
        params: TaskParams,
    ) -> Result<Task<Data, Out, String>, EnqueueError> {
        let n = {
            let mut log = self.log.lock().unwrap();
            let n = log.len();
            if self.fail_at == Some(n) {
                return Err(EnqueueError("full".to_string()));
            }
            log.push((data.clone(), params.clone()));
            n
        };
        let reply = (self.respond)(&data, n);
        let task = Task::new(data, params);
        match reply {
            Reply::Done(out) => {
                task.claim().unwrap();
                task.complete(Ok(out)).unwrap();
            }
            Reply::Fail(msg) => {
                task.claim().unwrap();
                task.complete(Err(msg.to_string())).unwrap();
            }
            Reply::Cancel => {
                assert!(task.cancel());
            }
        }
        Ok(task)
    }
}
fn num_queue() -> MockQueue {
    MockQueue::new(|d, _| match d {
        Data::Num(n) => Reply::Done(Out::Num(*n)),
        _ => Reply::Cancel,
    })
}
fn num_item(n: u32) -> FollowUp<Data> {
    FollowUp {
        data: Data::Num(n),
        params: TaskParams::default(),
    }
}
fn params_with_priority(priority: i32) -> TaskParams {
    TaskParams {
        priority,
        ..TaskParams::default()
    }
}
fn report_of(ids: &[u64]) -> SimilarityReport {
    SimilarityReport {
        task_id: TaskId(1),
        params: SimilarityParams::new(IndexId::new("7")).with_top_k(3),
        candidate_count: ids.len(),
        dropped_count: 0,
        stale_count: 0,
        hits: ids
            .iter()
            .map(|&id| ReportHit {
                entry_id: EntryId(id),
                score: 1.0,
                stale: false,
                source_link: None,
            })
            .collect(),
        metrics: None,
    }
}
fn text_queue() -> MockQueue {
    MockQueue::new(|d, n| match (d, n) {
        (Data::Embed(_), _) => Reply::Done(Out::Vectors(vec![vec![1.0, 0.0]])),
        (Data::Sim(_), 1) => Reply::Done(Out::Report(report_of(&[3, 1, 2]))),
        (Data::Sim(_), _) => Reply::Done(Out::Report(report_of(&[1, 3]))),
        _ => Reply::Cancel,
    })
}
fn text_query_with(rerank: Option<Arc<RerankPolicy>>) -> TextQuery {
    TextQuery {
        text: "hello".to_string(),
        model: ModelId::new("m"),
        precision: EmbPrecision::F32,
        embed_extensions: Extensions::new(),
        search: SimilarityParams::new(IndexId::new("7")).with_top_k(3),
        embed_task_params: params_with_priority(1),
        search_task_params: params_with_priority(2),
        rerank_task_params: params_with_priority(3),
        rerank,
    }
}
fn assert_send<T: Send>(_: &T) {}
#[test]
fn derive_params_leaves_base_unchanged() {
    let base = TaskParams { priority: 1 };
    let derived = derive_params(&base, |p| p.priority = 5);
    assert_eq!(base.priority, 1);
    assert_eq!(derived.priority, 5);
}
#[test]
fn derive_extensions_shares_values_and_isolates_changes() {
    let mut base = Extensions::new();
    base.insert(7u32);
    let derived = derive_extensions(&base, |e| e.insert("x".to_string()));
    assert!(base.get::< String > ().is_none());
    assert_eq!(derived.get::< String > ().map(String::as_str), Some("x"));
    assert!(
        std::ptr::eq(base.get::< u32 > ().unwrap(), derived.get::< u32 > ().unwrap())
    );
    let removed = derive_extensions(
        &base,
        |e| {
            e.remove::<u32>();
        },
    );
    assert!(removed.get::< u32 > ().is_none());
    assert_eq!(base.get::< u32 > (), Some(& 7));
}
#[test]
fn enqueue_all_keeps_order_and_handles_empty() {
    let q = num_queue();
    assert!(enqueue_all(& q, Vec::new()).unwrap().is_empty());
    let handles = enqueue_all(&q, vec![num_item(1), num_item(2), num_item(3)]).unwrap();
    assert_eq!(handles.len(), 3);
    let seen: Vec<u32> = q
        .logged()
        .iter()
        .map(|(d, _)| match d {
            Data::Num(n) => *n,
            _ => unreachable!(),
        })
        .collect();
    assert_eq!(seen, vec![1, 2, 3]);
    for (i, h) in handles.iter().enumerate() {
        assert!(matches!(h.result(), Some(Ok(Out::Num(n))) if * n == i as u32 + 1));
    }
}
#[test]
fn enqueue_all_stops_at_first_error() {
    let q = num_queue().failing_at(1);
    let err = match enqueue_all(&q, vec![num_item(1), num_item(2), num_item(3)]) {
        Ok(_) => panic!("expected error"),
        Err(e) => e,
    };
    assert_eq!(err, AddonError::Enqueue(EnqueueError("full".to_string())));
    assert_eq!(q.logged().len(), 1);
}
#[tokio::test]
async fn follow_ups_passes_ok_result_to_derive() {
    let q = num_queue();
    let parent = q.enqueue(Data::Num(4), TaskParams::default()).unwrap();
    let handles = follow_ups(
            &q,
            &parent,
            |r| match r {
                Ok(Out::Num(n)) => vec![num_item(* n + 1), num_item(* n + 2)],
                _ => Vec::new(),
            },
        )
        .await
        .unwrap();
    assert_eq!(handles.len(), 2);
    assert_eq!(q.logged().len(), 3);
}
#[tokio::test]
async fn follow_ups_reaches_derive_for_failed_cancelled_and_empty() {
    let q = MockQueue::new(|d, _| match d {
        Data::Num(0) => Reply::Fail("boom"),
        Data::Num(1) => Reply::Cancel,
        Data::Num(n) => Reply::Done(Out::Num(*n)),
        _ => Reply::Cancel,
    });
    let failed = q.enqueue(Data::Num(0), TaskParams::default()).unwrap();
    let handles = follow_ups(
            &q,
            &failed,
            |r| match r {
                Err(TaskError::Failed(e)) => {
                    assert_eq!(e, "boom");
                    vec![num_item(10)]
                }
                _ => Vec::new(),
            },
        )
        .await
        .unwrap();
    assert_eq!(handles.len(), 1);
    let cancelled = q.enqueue(Data::Num(1), TaskParams::default()).unwrap();
    let handles = follow_ups(
            &q,
            &cancelled,
            |r| match r {
                Err(TaskError::Cancelled) => vec![num_item(11)],
                _ => Vec::new(),
            },
        )
        .await
        .unwrap();
    assert_eq!(handles.len(), 1);
    let before = q.logged().len();
    let none = follow_ups(&q, &failed, |_| Vec::new()).await.unwrap();
    assert!(none.is_empty());
    assert_eq!(q.logged().len(), before);
}
#[tokio::test]
async fn cascade_iterates_until_next_returns_none() {
    let q = num_queue();
    let fut = cascade(
        &q,
        num_item(0),
        |r| match r {
            Ok(Out::Num(n)) if *n < 3 => Some(num_item(*n + 1)),
            _ => None,
        },
    );
    assert_send(&fut);
    let handles = fut.await.unwrap();
    assert_eq!(handles.len(), 4);
    for (i, h) in handles.iter().enumerate() {
        assert!(matches!(h.result(), Some(Ok(Out::Num(n))) if * n == i as u32));
    }
}
#[tokio::test]
async fn cascade_stops_immediately_and_reports_enqueue_errors() {
    let q = num_queue();
    let handles = cascade(&q, num_item(0), |_| None).await.unwrap();
    assert_eq!(handles.len(), 1);
    let q = num_queue().failing_at(2);
    let err = match cascade(
            &q,
            num_item(0),
            |r| match r {
                Ok(Out::Num(n)) => Some(num_item(*n + 1)),
                _ => None,
            },
        )
        .await
    {
        Ok(_) => panic!("expected error"),
        Err(e) => e,
    };
    assert_eq!(err, AddonError::Enqueue(EnqueueError("full".to_string())));
    assert_eq!(q.logged().len(), 2);
}
#[tokio::test]
async fn text_query_without_policy_skips_rerank() {
    let q = text_queue();
    let fut = text_query(&q, text_query_with(None));
    assert_send(&fut);
    let outcome = fut.await.unwrap();
    assert!(outcome.rerank.is_none());
    assert_eq!(outcome.report, report_of(& [3, 1, 2]));
    let log = q.logged();
    assert_eq!(log.len(), 2);
    assert_eq!(log[0].1.priority, 1);
    assert_eq!(log[1].1.priority, 2);
    match &log[0].0 {
        Data::Embed(t) => {
            assert_eq!(t.mode, EmbeddingMode::Query);
            assert!(t.target.is_none());
            assert_eq!(t.input.texts(), & ["hello".to_string()]);
        }
        _ => panic!("first task must be the embedding task"),
    }
    match &log[1].0 {
        Data::Sim(j) => assert_eq!(j.queries, vec![vec![1.0, 0.0]]),
        _ => panic!("second task must be the similarity task"),
    }
}
#[tokio::test]
async fn text_query_runs_rerank_when_policy_decides() {
    let q = text_queue();
    let policy: Arc<RerankPolicy> = Arc::new(|r: &SimilarityReport| {
        if r.hits.len() > 2 { Some(Scoring::DotProduct) } else { None }
    });
    let outcome = text_query(&q, text_query_with(Some(policy))).await.unwrap();
    assert!(outcome.rerank.is_some());
    assert_eq!(outcome.report, report_of(& [1, 3]));
    let log = q.logged();
    assert_eq!(log.len(), 3);
    assert_eq!(log[2].1.priority, 3);
    match &log[2].0 {
        Data::Sim(j) => {
            assert_eq!(j.queries, vec![vec![1.0, 0.0]]);
            assert_eq!(j.params.scoring, Scoring::DotProduct);
            assert_eq!(j.params.scope, Scope::Set(EntrySet::from_ids([1, 2, 3])));
        }
        _ => panic!("third task must be the rerank task"),
    }
}
#[tokio::test]
async fn text_query_policy_declining_yields_first_report() {
    let q = text_queue();
    let policy: Arc<RerankPolicy> = Arc::new(|_: &SimilarityReport| None);
    let outcome = text_query(&q, text_query_with(Some(policy))).await.unwrap();
    assert!(outcome.rerank.is_none());
    assert_eq!(outcome.report, report_of(& [3, 1, 2]));
    assert_eq!(q.logged().len(), 2);
}
async fn query_error(q: &MockQueue, rerank: Option<Arc<RerankPolicy>>) -> AddonError {
    match text_query(q, text_query_with(rerank)).await {
        Ok(_) => panic!("expected error"),
        Err(e) => e,
    }
}
#[tokio::test]
async fn text_query_maps_embedding_failures() {
    let q = MockQueue::new(|_, _| Reply::Cancel);
    assert_eq!(
        query_error(& q, None). await, AddonError::Cancelled { stage : Stage::Embedding }
    );
    let q = MockQueue::new(|_, _| Reply::Fail("bad"));
    assert_eq!(
        query_error(& q, None). await, AddonError::TaskFailed { stage : Stage::Embedding,
        message : format!("{:?}", "bad".to_string()), }
    );
    let q = MockQueue::new(|_, _| Reply::Done(Out::Num(1)));
    assert_eq!(
        query_error(& q, None). await, AddonError::UnexpectedOutput { stage :
        Stage::Embedding }
    );
    let q = MockQueue::new(|_, _| Reply::Done(Out::Vectors(Vec::new())));
    assert_eq!(query_error(& q, None). await, AddonError::NoQueryVectors);
    let q = text_queue().failing_at(0);
    assert_eq!(
        query_error(& q, None). await, AddonError::Enqueue(EnqueueError("full"
        .to_string()))
    );
}
#[tokio::test]
async fn text_query_maps_similarity_and_rerank_failures() {
    let q = MockQueue::new(|d, _| match d {
        Data::Embed(_) => Reply::Done(Out::Vectors(vec![vec![1.0]])),
        _ => Reply::Done(Out::Num(1)),
    });
    assert_eq!(
        query_error(& q, None). await, AddonError::UnexpectedOutput { stage :
        Stage::Similarity }
    );
    let q = MockQueue::new(|d, _| match d {
        Data::Embed(_) => Reply::Done(Out::Vectors(vec![vec![1.0]])),
        _ => Reply::Cancel,
    });
    assert_eq!(
        query_error(& q, None). await, AddonError::Cancelled { stage : Stage::Similarity
        }
    );
    let q = MockQueue::new(|d, n| match (d, n) {
        (Data::Embed(_), _) => Reply::Done(Out::Vectors(vec![vec![1.0]])),
        (_, 1) => Reply::Done(Out::Report(report_of(&[1]))),
        _ => Reply::Fail("rr"),
    });
    let policy: Arc<RerankPolicy> = Arc::new(|_: &SimilarityReport| Some(
        Scoring::Cosine,
    ));
    assert_eq!(
        query_error(& q, Some(policy)). await, AddonError::TaskFailed { stage :
        Stage::Rerank, message : format!("{:?}", "rr".to_string()), }
    );
}
#[test]
fn rerank_job_matches_params_rerank() {
    let report = report_of(&[5, 2, 9, 2]);
    let before = report.clone();
    let job = rerank_job(vec![vec![0.5, 0.5]], &report, Scoring::NegEuclidean);
    assert_eq!(report, before);
    assert_eq!(job.queries, vec![vec![0.5, 0.5]]);
    let output = SimilarityOutput {
        per_query: vec![
            report.hits.iter().map(| h | SimilarityHit { entry : h.entry_id, score : h
            .score, stale : h.stale, }).collect()
        ],
    };
    assert_eq!(job.params, report.params.rerank(& output, Scoring::NegEuclidean));
}
#[test]
fn rerank_job_keeps_stale_hits_and_handles_empty_report() {
    let mut report = report_of(&[1, 2]);
    report.hits[0].stale = true;
    let job = rerank_job(Vec::new(), &report, Scoring::Cosine);
    assert_eq!(job.params.scope, Scope::Set(EntrySet::from_ids([1, 2])));
    let empty = report_of(&[]);
    let job = rerank_job(Vec::new(), &empty, Scoring::Cosine);
    assert_eq!(job.params.scope, Scope::Set(EntrySet::empty()));
    assert_eq!(job.params.top_k, empty.params.top_k);
}
#[test]
fn addon_error_display_is_informative() {
    let e = AddonError::TaskFailed {
        stage: Stage::Rerank,
        message: "x".to_string(),
    };
    assert!(e.to_string().contains("Rerank"));
    assert!(AddonError::NoQueryVectors.to_string().contains("no query vectors"));
}
