//! Stepwise handover of an operation chain and preparation of its result.
use std::fmt;
use std::sync::{Arc, Mutex, MutexGuard};
use super::materialization::DeviceResult;
use super::super::_5::a::operation_node::{
    ContinuationHandler, HasOperationKind, OperationNode, OutputHandle,
};
use super::super::_5::a::similarity_report::{
    build_report, ReportError, ReportOptions, SimilarityReport as RankedReport,
};
use super::super::_5::_4::a::resource_requirements::HasResourceRequirements;
use super::super::_5::_4::b::embedding_task::EmbeddingTask;
use super::super::_5::_4::b::similarity_task::SimilarityParams;
use super::super::_5::_4::_3::a::result_kinds::{Hit2, TaskResult};
use super::super::_5::_4::b::operation::{DataShape, RamData};
use super::super::_5::_4::_3::b::task_context::{TaskContext, TaskId};
use super::super::_5::_4::_3::_2::a::index_contract::EntryId;
use super::super::_5::_4::_3::_2::b::task_model::Task;
/// Output of one step. `Device` only for steps with `materialize == false`.
#[derive(Debug, Clone)]
pub enum StepOutput {
    Ram(RamData),
    Device(DeviceResult),
}
/// Input of a node: actual data, the output of a planned predecessor, or an inference text pair.
#[derive(Debug, Clone)]
pub enum ChainInput {
    Data(StepOutput),
    Upstream(OutputHandle<StepOutput>),
    TextPair(String, String),
}
pub type ChainNode<O, D, E> = OperationNode<O, ChainInput, StepOutput, D, E>;
/// Builds the operation of one step from that step's actual input and the task context.
pub type StepFactory<O, E> = Arc<
    dyn Fn(&ChainInput, &Arc<TaskContext<E>>) -> O + Send + Sync,
>;
/// Report settings for a Similarity task with a scored-hit result.
#[derive(Clone)]
pub struct ReportConfig {
    pub options: ReportOptions,
    pub source_link: Arc<dyn Fn(EntryId) -> Option<String> + Send + Sync>,
}
/// Task kind with the data needed for preparing input and result.
#[derive(Clone)]
pub enum ChainKind {
    Embedding(EmbeddingTask),
    Similarity {
        queries: Vec<Vec<f32>>,
        params: SimilarityParams,
        report: ReportConfig,
    },
    Inference { query: String, passage: String },
}
impl ChainKind {
    fn name(&self) -> &'static str {
        match self {
            ChainKind::Embedding(_) => "embedding",
            ChainKind::Similarity { .. } => "similarity",
            ChainKind::Inference { .. } => "inference",
        }
    }
}
/// Final result of a chain.
#[derive(Debug, Clone, PartialEq)]
pub enum CollectedResult {
    Task(TaskResult),
    Report(RankedReport),
}
#[derive(Debug, Clone, PartialEq)]
pub enum ResultError3 {
    MissingResult,
    NotMaterialized,
    KindMismatch { task_kind: &'static str, found: DataShape },
    EmbeddingSplit { values: usize, texts: usize },
    Report(ReportError),
}
impl fmt::Display for ResultError3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ResultError3::MissingResult => write!(f, "no result available"),
            ResultError3::NotMaterialized => write!(f, "result is still on the device"),
            ResultError3::KindMismatch { task_kind, found } => {
                write!(f, "{task_kind} task cannot convert {found:?} data")
            }
            ResultError3::EmbeddingSplit { values, texts } => {
                write!(f, "cannot split {values} values into {texts} embeddings")
            }
            ResultError3::Report(e) => write!(f, "report failed: {e:?}"),
        }
    }
}
impl std::error::Error for ResultError3 {}
impl ResultError3 {
    /// All result errors are planning configuration errors; retrying is not meaningful.
    pub fn is_configuration_error(&self) -> bool {
        true
    }
}
type Slot6 = Arc<Mutex<Option<StepOutput>>>;
fn lock_slot(slot: &Mutex<Option<StepOutput>>) -> MutexGuard<'_, Option<StepOutput>> {
    slot.lock().unwrap_or_else(|p| p.into_inner())
}
/// Cloneable handle on the shared result slot. Clones share the same slot.
#[derive(Clone)]
pub struct Collector2 {
    slot: Slot6,
    kind: ChainKind,
    task_id: TaskId,
    texts: usize,
}
impl Collector2 {
    /// Takes the stored result (once) and converts it per `ChainKind`.
    pub fn collect(&self) -> Result<CollectedResult, ResultError3> {
        let stored = lock_slot(&self.slot).take().ok_or(ResultError3::MissingResult)?;
        let data = match stored {
            StepOutput::Ram(data) => data,
            StepOutput::Device(_) => return Err(ResultError3::NotMaterialized),
        };
        match &self.kind {
            ChainKind::Embedding(_) => self.collect_embeddings(data),
            ChainKind::Similarity { params, report, .. } => {
                match data {
                    RamData::HitList(hits) => {
                        Ok(
                            CollectedResult::Task(
                                TaskResult::Hits(
                                    hits
                                        .into_iter()
                                        .map(|(id, score)| Hit2 { id, score, stale: false })
                                        .collect(),
                                ),
                            ),
                        )
                    }
                    RamData::ScoredHitResult(result) => {
                        build_report(
                                self.task_id,
                                Some(params),
                                &result,
                                report.options,
                                &*report.source_link,
                            )
                            .map(CollectedResult::Report)
                            .map_err(ResultError3::Report)
                    }
                    other => Err(self.mismatch(&other)),
                }
            }
            ChainKind::Inference { .. } => {
                match data {
                    RamData::Floats(scores) => {
                        Ok(CollectedResult::Task(TaskResult::Scores(scores)))
                    }
                    other => Err(self.mismatch(&other)),
                }
            }
        }
    }
    fn collect_embeddings(&self, data: RamData) -> Result<CollectedResult, ResultError3> {
        let values = match data {
            RamData::Floats(values) => values,
            other => return Err(self.mismatch(&other)),
        };
        let texts = self.texts;
        if texts == 0 || values.len() % texts != 0 {
            return Err(ResultError3::EmbeddingSplit {
                values: values.len(),
                texts,
            });
        }
        let dim = values.len() / texts;
        let vectors = (0..texts)
            .map(|i| values[i * dim..(i + 1) * dim].to_vec())
            .collect();
        Ok(CollectedResult::Task(TaskResult::Embeddings(vectors)))
    }
    fn mismatch(&self, data: &RamData) -> ResultError3 {
        ResultError3::KindMismatch {
            task_kind: self.kind.name(),
            found: data.shape(),
        }
    }
}
pub struct ChainStart<O, D, E> {
    /// `None` iff the chain is empty.
    pub first: Option<ChainNode<O, D, E>>,
    pub collector: Collector2,
}
struct ChainPlan<O, E> {
    steps: Vec<StepFactory<O, E>>,
    context: Arc<TaskContext<E>>,
    slot: Slot6,
}
fn build_node<O, D, E>(
    plan: &Arc<ChainPlan<O, E>>,
    index: usize,
    input: ChainInput,
    task: Task<D, StepOutput, E>,
) -> Option<ChainNode<O, D, E>>
where
    O: HasResourceRequirements + HasOperationKind + Send + Sync + 'static,
    D: Send + 'static,
    E: Send + Sync + 'static,
{
    let factory = plan.steps.get(index)?;
    let operation = factory(&input, &plan.context);
    let materialize = index + 1 == plan.steps.len();
    let node = OperationNode::new(
        plan.context.id(),
        operation,
        input,
        materialize,
        task,
    );
    Some(node.with_continuation(make_handler(plan.clone(), index)))
}
fn make_handler<O, D, E>(
    plan: Arc<ChainPlan<O, E>>,
    index: usize,
) -> ContinuationHandler<O, ChainInput, StepOutput, D, E>
where
    O: HasResourceRequirements + HasOperationKind + Send + Sync + 'static,
    D: Send + 'static,
    E: Send + Sync + 'static,
{
    Arc::new(move |output: &StepOutput, task: &Task<D, StepOutput, E>| {
        if index + 1 < plan.steps.len() {
            let input = ChainInput::Data(output.clone());
            build_node(&plan, index + 1, input, task.clone()).into_iter().collect()
        } else {
            lock_slot(&plan.slot).get_or_insert_with(|| output.clone());
            Vec::new()
        }
    })
}
/// Builds only the first node of the chain and the collector for its result.
pub fn start_chain<O, D, E>(
    kind: ChainKind,
    steps: Vec<StepFactory<O, E>>,
    context: Arc<TaskContext<E>>,
    task: Task<D, StepOutput, E>,
) -> ChainStart<O, D, E>
where
    O: HasResourceRequirements + HasOperationKind + Send + Sync + 'static,
    D: Send + 'static,
    E: Send + Sync + 'static,
{
    let (input0, texts) = match &kind {
        ChainKind::Embedding(embedding) => {
            let texts = embedding.input.texts().to_vec();
            let count = texts.len();
            (ChainInput::Data(StepOutput::Ram(RamData::TextList(texts))), count)
        }
        ChainKind::Similarity { queries, .. } => {
            let flat: Vec<f32> = queries.iter().flatten().copied().collect();
            (ChainInput::Data(StepOutput::Ram(RamData::Floats(flat))), 0)
        }
        ChainKind::Inference { query, passage } => {
            (ChainInput::TextPair(query.clone(), passage.clone()), 0)
        }
    };
    let slot: Slot6 = Arc::new(Mutex::new(None));
    let collector = Collector2 {
        slot: slot.clone(),
        kind,
        task_id: context.id(),
        texts,
    };
    let plan = Arc::new(ChainPlan { steps, context, slot });
    let first = build_node(&plan, 0, input0, task);
    ChainStart { first, collector }
}
#[cfg(test)]
#[path = "test/chain_handover.rs"]
mod tests;
