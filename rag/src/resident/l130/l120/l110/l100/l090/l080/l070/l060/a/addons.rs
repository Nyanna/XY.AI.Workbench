//! Derives follow-up tasks from finished tasks and enqueues them through the normal queue path.
use std::fmt;
use std::sync::Arc;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::a::similarity_report::SimilarityReport;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::b::embedding_task::{
    EmbeddingMode, EmbeddingTask, Precision as EmbPrecision,
};
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::a::resource_requirements::ModelId;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::b::similarity_task::{
    Scoring, SimilarityParams,
};
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::b::task_model::{
    Task, TaskError, TaskParams,
};
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::l010::extensions::Extensions;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::l010::scopes_and_entry_sets::{
    EntrySet, Scope,
};
/// Normal enqueue path, implemented by the queue.
pub trait Enqueue<D, T, E>: Send + Sync {
    fn enqueue(
        &self,
        data: D,
        params: TaskParams,
    ) -> Result<Task<D, T, E>, EnqueueError>;
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnqueueError(pub String);
impl fmt::Display for EnqueueError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "enqueue failed: {}", self.0)
    }
}
impl std::error::Error for EnqueueError {}
/// Data of a similarity task: query vectors plus params.
#[derive(Debug, Clone)]
pub struct SimilarityJob {
    pub queries: Vec<Vec<f32>>,
    pub params: SimilarityParams,
}
/// What the addons read from a task output.
pub trait AddonOutput {
    /// Some for an embedding result without target (one vector per input text).
    fn embeddings(&self) -> Option<&[Vec<f32>]>;
    /// Some for a similarity result.
    fn similarity_report(&self) -> Option<&SimilarityReport>;
}
/// A derived task, not yet enqueued.
#[derive(Debug, Clone)]
pub struct FollowUp<D> {
    pub data: D,
    pub params: TaskParams,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stage {
    Embedding,
    Similarity,
    Rerank,
    Parent,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AddonError {
    Enqueue(EnqueueError),
    Cancelled { stage: Stage },
    /// Task completed with `Err(Failed(e))`; `message` is `format!("{:?}", e)`.
    TaskFailed { stage: Stage, message: String },
    /// Result did not have the expected kind.
    UnexpectedOutput { stage: Stage },
    /// Embedding returned no vector.
    NoQueryVectors,
}
impl fmt::Display for AddonError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AddonError::Enqueue(e) => write!(f, "{e}"),
            AddonError::Cancelled { stage } => {
                write!(f, "task cancelled at stage {stage:?}")
            }
            AddonError::TaskFailed { stage, message } => {
                write!(f, "task failed at stage {stage:?}: {message}")
            }
            AddonError::UnexpectedOutput { stage } => {
                write!(f, "unexpected task output at stage {stage:?}")
            }
            AddonError::NoQueryVectors => {
                write!(f, "embedding returned no query vectors")
            }
        }
    }
}
impl std::error::Error for AddonError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            AddonError::Enqueue(e) => Some(e),
            _ => None,
        }
    }
}
impl From<EnqueueError> for AddonError {
    fn from(e: EnqueueError) -> Self {
        AddonError::Enqueue(e)
    }
}
/// Clone of `base` with `f` applied. `base` is unchanged.
pub fn derive_params(base: &TaskParams, f: impl FnOnce(&mut TaskParams)) -> TaskParams {
    let mut params = base.clone();
    f(&mut params);
    params
}
/// Clone of `base` (values shared) with `f` applied to the clone only.
pub fn derive_extensions(
    base: &Extensions,
    f: impl FnOnce(&mut Extensions),
) -> Extensions {
    let mut extensions = base.clone();
    f(&mut extensions);
    extensions
}
/// Enqueues every follow-up in order; stops at the first enqueue error.
pub fn enqueue_all<D, T, E>(
    q: &dyn Enqueue<D, T, E>,
    items: Vec<FollowUp<D>>,
) -> Result<Vec<Task<D, T, E>>, AddonError> {
    let mut handles = Vec::with_capacity(items.len());
    for item in items {
        handles.push(q.enqueue(item.data, item.params)?);
    }
    Ok(handles)
}
/// Waits for `parent`, lets `derive` build follow-ups from its result, then enqueues them.
pub async fn follow_ups<D, T, E, F>(
    q: &dyn Enqueue<D, T, E>,
    parent: &Task<D, T, E>,
    derive: F,
) -> Result<Vec<Task<D, T, E>>, AddonError>
where
    F: FnOnce(Result<&T, TaskError<&E>>) -> Vec<FollowUp<D>>,
{
    let items = derive(parent.wait().await);
    enqueue_all(q, items)
}
/// Enqueues `first`, waits, asks `next` for the following task (None stops), and repeats.
pub async fn cascade<D, T, E, F>(
    q: &dyn Enqueue<D, T, E>,
    first: FollowUp<D>,
    mut next: F,
) -> Result<Vec<Task<D, T, E>>, AddonError>
where
    F: FnMut(Result<&T, TaskError<&E>>) -> Option<FollowUp<D>>,
{
    let mut handles: Vec<Task<D, T, E>> = Vec::new();
    let mut current = first;
    loop {
        let task = q.enqueue(current.data, current.params)?;
        let index = handles.len();
        handles.push(task);
        let following = next(handles[index].wait().await);
        match following {
            Some(item) => current = item,
            None => return Ok(handles),
        }
    }
}
/// Decision based on the facts of the first report. Some(scoring) runs a rerank.
pub type RerankPolicy = dyn Fn(&SimilarityReport) -> Option<Scoring> + Send + Sync;
pub struct TextQuery {
    pub text: String,
    pub model: ModelId,
    pub precision: EmbPrecision,
    pub embed_extensions: Extensions,
    pub search: SimilarityParams,
    pub embed_task_params: TaskParams,
    pub search_task_params: TaskParams,
    pub rerank_task_params: TaskParams,
    pub rerank: Option<Arc<RerankPolicy>>,
}
pub struct TextQueryOutcome<D, T, E> {
    pub embedding: Task<D, T, E>,
    pub similarity: Task<D, T, E>,
    pub rerank: Option<Task<D, T, E>>,
    /// Report of the rerank task if one ran, otherwise of the similarity task.
    pub report: SimilarityReport,
}
async fn settle<'a, D, T, E: fmt::Debug>(
    task: &'a Task<D, T, E>,
    stage: Stage,
) -> Result<&'a T, AddonError> {
    match task.wait().await {
        Ok(output) => Ok(output),
        Err(TaskError::Cancelled) => Err(AddonError::Cancelled { stage }),
        Err(TaskError::Failed(e)) => {
            Err(AddonError::TaskFailed {
                stage,
                message: format!("{:?}", e),
            })
        }
    }
}
async fn settle_report<D, T, E>(
    task: &Task<D, T, E>,
    stage: Stage,
) -> Result<SimilarityReport, AddonError>
where
    T: AddonOutput,
    E: fmt::Debug,
{
    let output = settle(task, stage).await?;
    output
        .similarity_report()
        .cloned()
        .ok_or(AddonError::UnexpectedOutput {
            stage,
        })
}
/// Embedding in Query mode, then similarity over the result vectors, then optional rerank.
pub async fn text_query<D, T, E>(
    q: &dyn Enqueue<D, T, E>,
    query: TextQuery,
) -> Result<TextQueryOutcome<D, T, E>, AddonError>
where
    D: From<EmbeddingTask> + From<SimilarityJob>,
    T: AddonOutput,
    E: fmt::Debug,
{
    let TextQuery {
        text,
        model,
        precision,
        embed_extensions,
        search,
        embed_task_params,
        search_task_params,
        rerank_task_params,
        rerank,
    } = query;
    let embed_job = EmbeddingTask::new(text, model, EmbeddingMode::Query, precision)
        .with_extensions(embed_extensions);
    let embedding = q.enqueue(D::from(embed_job), embed_task_params)?;
    let vectors: Vec<Vec<f32>> = {
        let output = settle(&embedding, Stage::Embedding).await?;
        let vectors = output
            .embeddings()
            .ok_or(AddonError::UnexpectedOutput {
                stage: Stage::Embedding,
            })?;
        if vectors.is_empty() {
            return Err(AddonError::NoQueryVectors);
        }
        vectors.to_vec()
    };
    let (search_queries, rerank_queries) = if rerank.is_some() {
        (vectors.clone(), Some(vectors))
    } else {
        (vectors, None)
    };
    let search_job = SimilarityJob {
        queries: search_queries,
        params: search,
    };
    let similarity = q.enqueue(D::from(search_job), search_task_params)?;
    let mut report = settle_report(&similarity, Stage::Similarity).await?;
    let mut rerank_task = None;
    if let (Some(policy), Some(queries)) = (rerank.as_ref(), rerank_queries) {
        if let Some(scoring) = (policy.as_ref())(&report) {
            let job = rerank_job(queries, &report, scoring);
            let task = q.enqueue(D::from(job), rerank_task_params)?;
            report = settle_report(&task, Stage::Rerank).await?;
            rerank_task = Some(task);
        }
    }
    Ok(TextQueryOutcome {
        embedding,
        similarity,
        rerank: rerank_task,
        report,
    })
}
/// Rerank job from a report: same queries, scope narrowed to the report's hit IDs, given scoring.
pub fn rerank_job(
    queries: Vec<Vec<f32>>,
    report: &SimilarityReport,
    scoring: Scoring,
) -> SimilarityJob {
    let mut params = report.params.clone();
    params.scope = Scope::Set(
        EntrySet::from_ids(report.hits.iter().map(|h| h.entry_id.0)),
    );
    params.scoring = scoring;
    SimilarityJob { queries, params }
}
const _: fn() = || {
    fn assert_traits<T: Send + Sync + 'static>() {}
    assert_traits::<EnqueueError>();
    assert_traits::<SimilarityJob>();
    assert_traits::<Stage>();
    assert_traits::<AddonError>();
    assert_traits::<TextQuery>();
};
#[cfg(test)]
#[path = "test/addons.rs"]
mod tests;
