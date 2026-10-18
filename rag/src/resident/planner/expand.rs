//! Expansion rules: one task becomes an ordered operation chain.

use crate::resident::error::PlannerError;
use crate::resident::facade::{
    EmbeddingParams, InferenceParams, Payload, SimilarityParams, TaskKind,
};
use crate::resident::op::{
    EngineSelector, ModelId, OpDesc, OpKind, OpParams, Requirements, ResourceKind,
};
use crate::resident::processor::StarvedSnapshot;

use super::config::{PipelineConfig, Pooling};
use super::offers::EngineOffers;

/// Borrowed view of a task's parameters; the data itself never influences the plan.
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub enum TaskSpec<'a> {
    Embedding(&'a EmbeddingParams),
    Similarity(&'a SimilarityParams),
    Inference(&'a InferenceParams),
}

impl<'a> TaskSpec<'a> {
    pub fn from_payload(payload: &'a Payload) -> Self {
        match payload {
            Payload::Embedding { params, .. } => Self::Embedding(params),
            Payload::Similarity { params, .. } => Self::Similarity(params),
            Payload::Inference { params, .. } => Self::Inference(params),
        }
    }

    pub fn kind(&self) -> TaskKind {
        match self {
            Self::Embedding(_) => TaskKind::Embedding,
            Self::Similarity(_) => TaskKind::Similarity,
            Self::Inference(_) => TaskKind::Inference,
        }
    }
}

/// Resource the first operation of a task of this kind needs.
pub fn first_resource(kind: TaskKind) -> ResourceKind {
    match kind {
        TaskKind::Similarity => ResourceKind::Scorer,
        TaskKind::Embedding | TaskKind::Inference => ResourceKind::Tokenizer,
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PlannedOp {
    pub desc: OpDesc,
    /// `false` keeps the result on the device for a GPU successor.
    pub materialize: bool,
}

pub struct ExpandContext<'a> {
    pub starved: StarvedSnapshot,
    pub offers: &'a EngineOffers,
    pub model: &'a ModelId,
    pub pipeline: &'a PipelineConfig,
}

impl ExpandContext<'_> {
    fn engine(&self, kind: ResourceKind) -> Result<EngineSelector, PlannerError> {
        self.offers.choose(kind, self.starved)
    }

    fn requires(&self, kind: ResourceKind, engine: EngineSelector) -> Requirements {
        Requirements::new(kind, engine).with_model(self.model.clone())
    }
}

/// Pure: no state, same inputs give the same operations.
pub trait PlanExpander: Send + Sync {
    fn expand(
        &self,
        task: &TaskSpec<'_>,
        ctx: &ExpandContext<'_>,
    ) -> Result<Vec<PlannedOp>, PlannerError>;
}

#[derive(Default)]
struct Chain(Vec<(OpKind, OpParams)>);

impl Chain {
    fn push(&mut self, kind: OpKind, params: OpParams) {
        self.0.push((kind, params));
    }

    /// A result stays on the device only when a GPU operation consumes it next.
    fn finish(self) -> Vec<PlannedOp> {
        let engines: Vec<EngineSelector> =
            self.0.iter().map(|(_, p)| p.requires.engine).collect();
        self.0
            .into_iter()
            .enumerate()
            .map(|(i, (kind, params))| {
                let gpu_here = engines[i] == EngineSelector::Gpu;
                let gpu_next = engines.get(i + 1) == Some(&EngineSelector::Gpu);
                PlannedOp {
                    desc: OpDesc::new(kind, params),
                    materialize: !(gpu_here && gpu_next),
                }
            })
            .collect()
    }
}

fn mismatch(expected: TaskKind, got: &TaskSpec<'_>) -> PlannerError {
    PlannerError::InvalidConfig {
        reason: format!("expander for {expected:?} received {:?} task", got.kind()),
    }
}

pub(super) struct EmbeddingExpander;

impl PlanExpander for EmbeddingExpander {
    fn expand(
        &self,
        task: &TaskSpec<'_>,
        ctx: &ExpandContext<'_>,
    ) -> Result<Vec<PlannedOp>, PlannerError> {
        let TaskSpec::Embedding(params) = task else {
            return Err(mismatch(TaskKind::Embedding, task));
        };
        let encoder = ctx.engine(ResourceKind::Encoder)?;
        let mut chain = Chain::default();

        let tokenizer = ctx.engine(ResourceKind::Tokenizer)?;
        chain.push(
            OpKind::Tokenize,
            OpParams::new(ctx.requires(ResourceKind::Tokenizer, tokenizer)),
        );
        let mut embed = OpParams::new(ctx.requires(ResourceKind::Encoder, encoder));
        embed.dim = ctx.pipeline.dim;
        chain.push(OpKind::Embed, embed);
        if ctx.pipeline.pooling != Pooling::None {
            chain.push(
                OpKind::Pool,
                OpParams::new(ctx.requires(ResourceKind::Encoder, encoder)),
            );
        }
        if ctx.pipeline.normalize {
            chain.push(
                OpKind::L2Norm,
                OpParams::new(ctx.requires(ResourceKind::Encoder, encoder)),
            );
        }
        if let Some(index) = params.target_index() {
            let engine = ctx.engine(ResourceKind::Index)?;
            chain.push(
                OpKind::WriteIndex,
                OpParams::new(
                    ctx.requires(ResourceKind::Index, engine)
                        .with_index(index.clone()),
                ),
            );
        }
        Ok(chain.finish())
    }
}

pub(super) struct SimilarityExpander;

impl PlanExpander for SimilarityExpander {
    fn expand(
        &self,
        task: &TaskSpec<'_>,
        ctx: &ExpandContext<'_>,
    ) -> Result<Vec<PlannedOp>, PlannerError> {
        let TaskSpec::Similarity(params) = task else {
            return Err(mismatch(TaskKind::Similarity, task));
        };
        if params.get_top_k() == 0 {
            return Err(PlannerError::InvalidConfig {
                reason: "top_k must be positive".into(),
            });
        }
        let scorer = ctx.engine(ResourceKind::Scorer)?;
        let requires = ctx
            .requires(ResourceKind::Scorer, scorer)
            .with_index(params.index().clone());

        let mut score = OpParams::new(requires.clone());
        score.dim = ctx.pipeline.dim;
        let mut top = OpParams::new(requires);
        top.top_k = Some(u32::try_from(params.get_top_k()).unwrap_or(u32::MAX));

        let mut chain = Chain::default();
        chain.push(OpKind::Similarity, score);
        chain.push(OpKind::TopK, top);
        Ok(chain.finish())
    }
}

pub(super) struct InferenceExpander;

impl PlanExpander for InferenceExpander {
    fn expand(
        &self,
        task: &TaskSpec<'_>,
        ctx: &ExpandContext<'_>,
    ) -> Result<Vec<PlannedOp>, PlannerError> {
        if !matches!(task, TaskSpec::Inference(_)) {
            return Err(mismatch(TaskKind::Inference, task));
        }
        let tokenizer = ctx.engine(ResourceKind::Tokenizer)?;
        let cross = ctx.engine(ResourceKind::CrossEncoder)?;
        let mut chain = Chain::default();
        chain.push(
            OpKind::Tokenize,
            OpParams::new(ctx.requires(ResourceKind::Tokenizer, tokenizer)),
        );
        chain.push(
            OpKind::Infer,
            OpParams::new(ctx.requires(ResourceKind::CrossEncoder, cross)),
        );
        Ok(chain.finish())
    }
}
