# Specification: Schrittweise Übergabe und Ergebnisaufbereitung (build order 78, module `chain_handover`)

## 1. Task, restrictions and boundaries

Implement in Rust a stepwise handover of an operation chain and the preparation of its result.

- `start_chain` builds only the **first** `OperationNode` of the chain and a `Collector`.
- A continuation handler builds each follow-up node from the **actual output** of the previous node.
- The input is prepared per task kind: Embedding, Similarity or Inference.
- The `Collector` takes the overall result **once** and converts it into the result of the task kind.

Out of scope:
- Executing operations, scheduling and enqueueing.
- Calling `TaskContext::register_follow_ups` or `complete_operation`.
- Creating `Task` or `TaskContext`.
- Calling `materialize()`.
- Embedding, scoring and report maths.
- Planning which operations make up the chain.

Restrictions:
- No `unsafe`.
- Nothing blocks, except for a short `Mutex` on the result slot.
- No panics.

## 2. Context

- The Planner supplies the task kind and the chain as a list of step factories. The sub-engine runs a node, calls `set_output` and then `node.advance()`. `advance` calls the continuation handler built here.
- The handler returns the follow-up node, or nothing at the end of the chain. Without a follow-up, `OperationNode::advance` completes the `Task` with `Ok(output.clone())`. The task result type is therefore `RamData`.
- The caller (the queue or sub-engine) owns the returned follow-up nodes. It registers them in the `TaskContext` and enqueues them. This module does neither.
- An executor calls the `Collector` after the task has completed. The task-level result is the `CollectedResult`.
- The operation kind is unrelated to the data conversion. The conversion depends only on the `ChainKind` and the `DataShape` of the stored `RamData`.

## 3. Interface

```rust
use std::sync::Arc;
use embedding_task::EmbeddingTask;
use similarity_task::SimilarityParams;
use similarity_report::{ReportOptions, ReportError, SimilarityReport as RankedReport};
use operation::{RamData, DataShape, EntryId as OpEntryId};
use result_kinds::TaskResult;
use task_context::TaskContext;
use task_model::Task;
use operation_node::{OperationNode, OperationKind};     // OperationNode<O,I,T,D,E>
use resource_requirements::HasResourceRequirements;

/// Input of a node. Embedding/Similarity: `Data`. Inference (first step): `TextPair(query, passage)`.
#[derive(Debug, Clone, PartialEq)]
pub enum ChainInput { Data(RamData), TextPair(String, String) }

/// Node type used by this module: I = ChainInput, T = RamData.
pub type ChainNode<O, D, E> = OperationNode<O, ChainInput, RamData, D, E>;

/// Builds the operation of one step from that step's actual input and the task context
/// (the factory takes id and priority from the context).
pub type StepFactory<O, E> = Arc<dyn Fn(&ChainInput, &Arc<TaskContext<E>>) -> O + Send + Sync>;

/// Only for Similarity with a scored-hit result.
#[derive(Clone)]
pub struct ReportConfig {
    pub options: ReportOptions,
    pub source_link: Arc<dyn Fn(OpEntryId) -> Option<String> + Send + Sync>,
}

/// Task kind with the data needed for preparing input and result.
#[derive(Clone)]
pub enum ChainKind {
    Embedding(EmbeddingTask),
    Similarity { queries: Vec<Vec<f32>>, params: SimilarityParams, report: ReportConfig },
    Inference  { query: String, passage: String },
}

/// Final result. `Report` is the ranked `similarity_report::SimilarityReport`
/// (not `result_kinds::SimilarityReport`, which has a different shape).
#[derive(Debug, Clone, PartialEq)]
pub enum CollectedResult { Task(TaskResult), Report(RankedReport) }

#[derive(Debug, Clone, PartialEq)]
pub enum ResultError {
    /// No result: empty chain, last operation never stored one, or already taken.
    MissingResult,
    /// Stored data does not fit the task kind. `task_kind` is "embedding" | "similarity" | "inference".
    KindMismatch { task_kind: &'static str, found: DataShape },
    /// Embedding: texts == 0 or values % texts != 0.
    EmbeddingSplit { values: usize, texts: usize },
    Report(ReportError),
}
impl std::fmt::Display for ResultError {}   // names task kind, or values and texts, as applicable
impl std::error::Error for ResultError {}
impl ResultError { /// Always true: all result errors are planning configuration errors.
    pub fn is_configuration_error(&self) -> bool; }

/// Cloneable handle on the shared result slot. Clone = same slot. Send + Sync.
#[derive(Clone)]
pub struct Collector { /* Arc<Mutex<Option<RamData>>>, ChainKind, task_id: u64 */ }
impl Collector {
    /// Takes the stored result (once) and converts it per ChainKind.
    pub fn collect(&self) -> Result<CollectedResult, ResultError>;
}

pub struct ChainStart<O, D, E> {
    /// `None` iff the chain is empty.
    pub first: Option<ChainNode<O, D, E>>,
    pub collector: Collector,
}

pub fn start_chain<O, D, E>(
    kind: ChainKind,
    steps: Vec<StepFactory<O, E>>,
    context: Arc<TaskContext<E>>,
    task: Task<D, RamData, E>,
) -> ChainStart<O, D, E>
where O: HasResourceRequirements + OperationKind + Send + Sync + 'static,
      D: Send + 'static,
      E: Send + Sync + 'static;
```

## 4. Behavior and invariants

**Start**
- `steps` empty: `first == None`. `collect()` returns `Err(MissingResult)`.
- Otherwise `first` is the node of `steps[0]` only. No other node is built yet.
- `first` is built as `OperationNode::new(context.id(), steps[0](&input0, &context), input0, materialize, task.clone())`.
- `materialize` is `true` only for the last step of the chain, otherwise `false`.
- `input0` is prepared per kind:
  - **Embedding:** `ChainInput::Data(RamData::TextList(task.input.texts().to_vec()))`. The text count is stored in the `Collector`. An empty list is allowed here.
  - **Similarity:** `Data(RamData::Floats(..))`. All query vectors are concatenated in query order into one flat sequence. Dimensions are not checked. No queries gives an empty `Floats`.
  - **Inference:** `TextPair(query, passage)`, always in this order.
- `first` has a continuation handler attached (`set_continuation`).

**Continuation handler** (for step k, called with `(&output, &task)`)
- If k is not the last step:
  - Let `input = ChainInput::Data(output.clone())`. This is the actual output of the previous node and is never recomputed or replaced.
  - The handler returns exactly one node: `new(context.id(), steps[k+1](&input, &context), input, materialize = (k+1 is last), task.clone())`.
  - That node has the handler for step k+1 attached.
  - The node carries the same `Task` handle (clone) and the same task ID as all other nodes of the chain.
- If k is the last step:
  - The handler stores `output.clone()` in the result slot. If the slot is already filled, it stays unchanged.
  - It returns an empty `Vec`. `OperationNode::advance` then completes the task with `Ok(output)`.
- Handlers are `Send + Sync`. They may be called repeatedly and concurrently. A repeated call builds an equivalent new node. This module adds no further state.
- Non-final operations leave the slot empty.

**Collector**
- `collect` first **takes** the slot content (`Option::take` under the lock). Afterwards the slot is empty, so the result is available once.
  - A second call, or a call with an empty slot, returns `Err(MissingResult)`.
  - The result stays consumed even when the conversion fails.
- Conversion per kind:
  - **Embedding** accepts only `Floats(v)`. Let `n` be the stored text count.
    - If `n == 0` or `v.len() % n != 0`, return `Err(EmbeddingSplit { values: v.len(), texts: n })`.
    - Otherwise split into `n` vectors of length `v.len()/n`, in order. Result: `Task(TaskResult::Embeddings(..))`.
  - **Similarity**:
    - `HitList(h)` gives `Task(TaskResult::Hits(..))` with `Hit { id: index_contract::EntryId(e.0), score }` per pair, in order.
    - `ScoredHitResult(r)` gives `Report(build_report(TaskId(context id), Some(&params), &r, report.options, &*report.source_link)?)`. A `ReportError` becomes `Report(e)`.
  - **Inference** accepts only `Floats(v)`. Result: `Task(TaskResult::Scores(v))`.
  - Any other `RamData` returns `Err(KindMismatch { task_kind, found: data.shape() })`.
- Concurrent `collect` calls: exactly one gets the data, the others get `MissingResult`.

**Errors**
- Errors are only returned, never panicked. They are configuration errors of the planning, so no retry is meaningful.

## 5. Relied-on elements

- `OperationNode<O,I,T,D,E>`:
  - `new(task_id: u64, operation: O, input: I, materialize: bool, task: Task<D,T,E>)`, requires `O: HasResourceRequirements + OperationKind`.
  - `set_continuation(&mut self, ContinuationHandler)` and `with_continuation(self, ..) -> Self`.
  - `ContinuationHandler = Arc<dyn Fn(&T, &Task<D,T,E>) -> Vec<OperationNode<..>> + Send + Sync>`.
  - `advance()`: with no handler or an empty result, it completes the task with `Ok(output.clone())`. `T: Clone` is required.
- `TaskContext<E>`: `id() -> u64`, `priority() -> i32`. It is used as `Arc<TaskContext<E>>`.
- `Task<D,T,E>`: `Clone`, a handle on the same task.
- `operation`:
  - `RamData::{TextList(Vec<String>), Floats(Vec<f32>), HitList(Vec<(EntryId, f32)>), ScoredHitResult(ScoredHitResult), ..}` is `#[non_exhaustive]` and `Clone`. `RamData::shape() -> DataShape`.
  - `EntryId(pub u64)` and `TaskId(pub u64)`.
- `materialization`: `ToHost` corresponds to `materialize == true`, `KeepOnDevice` to `false`. This module only sets the bool.
- `EmbeddingTask.input.texts() -> &[String]`.
- `SimilarityParams` is `Clone`. It has `top_k`, and `build_report` clones it.
- `result_kinds`:
  - `TaskResult::{Embeddings(Vec<Vec<f32>>), Hits(Vec<Hit>), Scores(Vec<f32>), ..}`.
  - `Hit { id: index_contract::EntryId, score: f32 }`.
- `similarity_report::build_report(task_id: TaskId, params: Option<&SimilarityParams>, input: &ScoredHitResult, options: ReportOptions, source_link: &dyn Fn(EntryId) -> Option<String>) -> Result<SimilarityReport, ReportError>`.