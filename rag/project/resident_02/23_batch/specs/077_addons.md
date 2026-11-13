# Spec: Addons (build order 77)

## 1. Task, restrictions and boundaries

Implement the Rust crate module `addons`. It has two jobs:
- It derives follow-up tasks from finished tasks, with cloned or changed parameters.
- It enqueues them through the normal queue path.

This is how iteration, cascade and recursion are built outside the core.

**Restrictions**
- No change to the core. Addon tasks are ordinary `Task<D,T,E>` handles with ordinary `TaskParams` and `Extensions`. No flag, type parameter or field marks a task as an addon task. No parent/child link is stored anywhere.
- No queue, scheduler, worker, timeout or retry logic. A follow-up is enqueued only through the injected `Enqueue` trait. Pull-filtering (priority, affinity) stays with the queue.
- No thresholds or decision logic of its own. Whether and how to follow up is decided by caller-supplied closures or policies, based on the facts in a result or `SimilarityReport`.
- No `unsafe`, no global state, no OS-thread blocking. Waiting uses `Task::wait().await`, which is runtime-agnostic.
- Dependencies: `std`, `task_model`, `extensions`, `embedding_task`, `similarity_task`, `similarity_report`, `scopes`.

## 2. Context

- A **Task** (`Task<D,T,E>`) is a clonable handle. It is created by the queue. The result is write-once and `wait()` resolves at Completed or Cancelled.
- An **Extensions** map carries shared extra parameters. Clones share the values, and a later `insert`/`remove` on one map does not affect the other.
- **EmbeddingTask** (`run` yields `TaskResult::Embeddings` when no target is set) and **SimilarityParams** (with `rerank` semantics) describe the work.
- **SimilarityReport** holds the facts (hits, counts, metrics) that steer reranking.
- The application or a resource implementation (index, model) calls these helpers from outside the core:
  - Centroid: a centroid task against the summary structure, then a follow-up over the members of the chosen cells. This uses `cascade` or `follow_ups`.
  - Text query: embedding in Query mode, then similarity with the result vectors, then optional rerank. This uses `text_query`.
- The executor that runs the tasks is external. It runs `EmbeddingTask::run`, `run_similarity` and `build_report`, and completes each task with a `T` that implements `AddonOutput`.

## 3. Interface

```rust
use std::sync::Arc;
use task_model::{Task, TaskParams, TaskError};
use extensions::Extensions;
use embedding_task::{EmbeddingTask, EmbeddingMode, EmbeddingInput, ModelId, Precision as EmbPrecision};
use similarity_task::{SimilarityParams, Scoring};
use similarity_report::SimilarityReport;
use scopes::{Scope, EntrySet};

/// Normal enqueue path, implemented by the queue. Creates the task and returns a handle.
pub trait Enqueue<D, T, E>: Send + Sync {
    fn enqueue(&self, data: D, params: TaskParams) -> Result<Task<D, T, E>, EnqueueError>;
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnqueueError(pub String);

/// Data of a similarity task: query vectors plus params.
#[derive(Debug, Clone)]
pub struct SimilarityJob { pub queries: Vec<Vec<f32>>, pub params: SimilarityParams }

/// Task data type D must be able to carry the two job kinds.
/// (bounds: `D: From<EmbeddingTask> + From<SimilarityJob>`)

/// Task output type T must expose what the addons read.
pub trait AddonOutput {
    /// Some for an embedding result without target (one vector per input text).
    fn embeddings(&self) -> Option<&[Vec<f32>]>;
    /// Some for a similarity result.
    fn similarity_report(&self) -> Option<&SimilarityReport>;
}

/// A derived task, not yet enqueued.
#[derive(Debug, Clone)]
pub struct FollowUp<D> { pub data: D, pub params: TaskParams }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stage { Embedding, Similarity, Rerank, Parent }

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AddonError {
    Enqueue(EnqueueError),
    Cancelled { stage: Stage },
    /// Task completed with Err(Failed(e)); `message` = format!("{:?}", e).
    TaskFailed { stage: Stage, message: String },
    /// Result did not have the expected kind (e.g. embedding task had a target, or no report).
    UnexpectedOutput { stage: Stage },
    /// Embedding returned no vector (empty input).
    NoQueryVectors,
}
impl std::fmt::Display for AddonError {}
impl std::error::Error for AddonError {}

// ---- parameter derivation (pure) ----

/// Clone of `base` with `f` applied. `base` is unchanged.
pub fn derive_params(base: &TaskParams, f: impl FnOnce(&mut TaskParams)) -> TaskParams;
/// Clone of `base` (values shared) with `f` applied to the clone only.
pub fn derive_extensions(base: &Extensions, f: impl FnOnce(&mut Extensions)) -> Extensions;

// ---- generic cascade ----

/// Enqueues every follow-up in order; returns handles in the same order.
/// Stops at the first enqueue error (already enqueued tasks stay enqueued).
pub fn enqueue_all<D, T, E>(q: &dyn Enqueue<D, T, E>, items: Vec<FollowUp<D>>)
    -> Result<Vec<Task<D, T, E>>, AddonError>;

/// Waits for `parent`, lets `derive` build follow-ups from its result
/// (Ok(&T) / Failed(&E) / Cancelled all reach `derive`), then enqueues them.
pub async fn follow_ups<D, T, E, F>(q: &dyn Enqueue<D, T, E>, parent: &Task<D, T, E>, derive: F)
    -> Result<Vec<Task<D, T, E>>, AddonError>
where F: FnOnce(Result<&T, TaskError<&E>>) -> Vec<FollowUp<D>>;

/// Iteration/recursion: enqueue `first`, wait, ask `next` for the following task
/// (None = stop), repeat. Returns all handles in order. `next` alone bounds the depth.
pub async fn cascade<D, T, E, F>(q: &dyn Enqueue<D, T, E>, first: FollowUp<D>, next: F)
    -> Result<Vec<Task<D, T, E>>, AddonError>
where F: FnMut(Result<&T, TaskError<&E>>) -> Option<FollowUp<D>>;

// ---- text query ----

/// Decision based on the facts of the first report. Some(scoring) = run a rerank.
pub type RerankPolicy = dyn Fn(&SimilarityReport) -> Option<Scoring> + Send + Sync;

pub struct TextQuery {
    pub text: String,
    pub model: ModelId,
    pub precision: EmbPrecision,
    /// Extensions for the embedding task.
    pub embed_extensions: Extensions,
    /// Search params for the similarity task (index, top_k, scope, scoring, precision, extensions).
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

pub async fn text_query<D, T, E>(q: &dyn Enqueue<D, T, E>, query: TextQuery)
    -> Result<TextQueryOutcome<D, T, E>, AddonError>
where D: From<EmbeddingTask> + From<SimilarityJob>, T: AddonOutput, E: std::fmt::Debug;

/// Rerank job from a report: same queries, params = report.params.clone() with
/// scope = Scope::Set(EntrySet::from_ids(report.hits' entry_id.0)) and scoring = `scoring`.
pub fn rerank_job(queries: Vec<Vec<f32>>, report: &SimilarityReport, scoring: Scoring) -> SimilarityJob;
```

`Send + Sync + 'static` holds for all non-generic types (compile-time assertion). The futures are `Send` if their inputs are.

## 4. Behavior and invariants

**Derivation**
- `derive_params` and `derive_extensions` never mutate their input. The result equals the input except for the change made by `f`. Extension values are shared, never copied (T need not be `Clone`).
- Follow-up tasks go only through `Enqueue::enqueue`. No other path exists. They have the same type as application tasks and carry only `TaskParams` and the data. All additional parameters travel in the `Extensions` inside the data, for example `EmbeddingTask.extensions` and `SimilarityParams.extensions`.

**enqueue_all / follow_ups / cascade**
- `enqueue_all` enqueues in input order and returns handles in that order. An empty list returns `Ok(vec![])`. On the first `Err(e)` it returns `AddonError::Enqueue(e)` and does not enqueue the rest.
- `follow_ups` calls `parent.wait().await`, then `derive` exactly once with that result, then `enqueue_all`. If `derive` returns an empty list, no task is enqueued and the result is `Ok(vec![])`. A failed or cancelled parent is not an error here, and `derive` decides.
- `cascade` enqueues `first` and waits for it. It calls `next` with its result. On `Some(f)` it enqueues `f` and repeats. On `None` it returns all handles (including `first`). `cascade` itself imposes no depth limit.
- Dropping a returned future at any await point is safe. Tasks already enqueued stay enqueued and are not cancelled.
- No handle is retained after return, and there is no shared state between calls. Concurrent calls are independent.

**text_query**
1. Builds `EmbeddingTask::new(text, model, EmbeddingMode::Query, precision)` with no target and with `embed_extensions`. It enqueues it with `embed_task_params`, then awaits it.
2. `Cancelled` becomes `AddonError::Cancelled{Embedding}`. `Failed(e)` becomes `TaskFailed{Embedding, ..}`. A result where `embeddings()` is `None` becomes `UnexpectedOutput{Embedding}`. An empty vector list becomes `NoQueryVectors`.
3. It enqueues `SimilarityJob{queries: vectors, params: search}` with `search_task_params`, then awaits it. Errors map as in step 2 with stage `Similarity`. A result where `similarity_report()` is `None` becomes `UnexpectedOutput{Similarity}`.
4. If `rerank` is `Some` and the policy returns `Some(scoring)` for that report, it enqueues `rerank_job(vectors, &report, scoring)` with `rerank_task_params` and awaits it. Errors map with stage `Rerank`. The rerank report becomes `outcome.report`. If the policy returns `None` or `rerank` is `None`, no rerank task exists (`rerank == None`) and `outcome.report` is the first report.
5. The policy sees only the report, so it is steered by facts and not by the addon.
- The embedding task is a separate task and the similarity task depends on it only through the enqueue order. Priority and affinity are the caller's `TaskParams`. This module does not alter them.

**rerank_job**
- It does not mutate the report. `top_k`, index, precision and extensions are taken unchanged from `report.params`. Stale hits remain in the scope. An empty report gives an empty `EntrySet`.
- The result is equivalent to `SimilarityParams::rerank` for a `SimilarityOutput` with the same hit IDs.

**Errors**
- No panics from this module. All failures map to `AddonError`. Task `Cancelled` or `Failed` in `follow_ups` and `cascade` is passed to the caller's closure and is not an error. In `text_query` it is an error.

## 5. Relied-on elements

- `task_model`:
  - `Task::new` is not used.
  - `Task<D,T,E>: Clone`, with clones sharing state.
  - `Task::wait(&self) -> Result<&T, TaskError<&E>>` (async).
  - `TaskError<E> { Cancelled, Failed(E) }`.
  - `TaskParams { priority: i32, resource: Option<String>, model: Option<String>, mode: Option<String> }`, which is `Clone + Default`. It is immutable after task creation, so changes are made on a derived copy.
  - `Task` is `Send + Sync` if `D: Send, T: Send + Sync, E: Send + Sync`.
- `extensions::Extensions`: `Clone + Default + Send + Sync`, with `insert`, `get`, `remove` (`&mut self`) and shared values on clone.
- `embedding_task`:
  - `EmbeddingTask::new(input: impl Into<EmbeddingInput>, model: ModelId, mode: EmbeddingMode, precision: Precision)` gives target `None` and empty extensions.
  - `.with_extensions(Extensions)`.
  - `EmbeddingMode::{Query, Passage}`.
  - `ModelId(pub String)`.
  - Without a target the result is `TaskResult::Embeddings(Vec<Vec<f32>>)`, one vector per text in order. With a target it is `EntryRefs`.
- `similarity_task`:
  - `SimilarityParams` (`Debug + Clone`) has the public fields `index, top_k, scope, scoring, precision, extensions`.
  - `Scoring` is `Copy`.
  - `Scope::Set(EntrySet)`.
  - `EntrySet::from_ids<I: IntoIterator<Item = u64>>`.
  - `SimilarityParams::rerank` semantics: same index, precision and extensions, scope narrowed to the hit IDs, given scoring, same `top_k`.
- `similarity_report::SimilarityReport`:
  - Fields `task_id`, `params: SimilarityParams`, `candidate_count`, `dropped_count`, `stale_count`, `hits: Vec<ReportHit>`, `metrics: Option<Metrics>`.
  - `ReportHit { entry_id: operation::EntryId(pub u64), score, stale, source_link }`.
  - `Metrics` (gap, spread, z_score, softmax, knee) are facts only, and the addon applies no thresholds.