//! Hub adapter: plans a task once and hands the chain over one operation at a time.

use std::sync::Arc;

use parking_lot::Mutex;

use crate::resident::error::{PlannerError, ResidentError};
use crate::resident::facade::{Output, Payload, TaskError, TaskKind, TaskMeta};
use crate::resident::index::EntryId;
use crate::resident::op::{HostData, OperationNode, ResourceKind, Successor, TaskCtx, Value};
use crate::resident::processor::{Collect, Expander, Expansion, StarvedSnapshot};

use super::expand::{first_resource, PlannedOp, TaskSpec};
use super::{EngineOffers, Planner};

pub struct PlannerExpander {
    planner: Arc<Planner>,
    offers: EngineOffers,
}

impl PlannerExpander {
    pub fn new(planner: Arc<Planner>, offers: EngineOffers) -> Self {
        Self { planner, offers }
    }
}

/// Planned operations plus the host result of the last one.
struct Chain {
    ops: Vec<PlannedOp>,
    result: Mutex<Option<HostData>>,
}

struct Step {
    chain: Arc<Chain>,
    index: usize,
}

impl Chain {
    fn node(self: &Arc<Self>, index: usize, input: Value, task: &Arc<TaskCtx>) -> Arc<OperationNode> {
        let planned = &self.ops[index];
        let node = OperationNode::new(
            Arc::new(planned.desc.clone()),
            input,
            planned.materialize,
            task.clone(),
        );
        Arc::new(node.with_successor(Arc::new(Step { chain: self.clone(), index })))
    }
}

impl Successor for Step {
    fn follow(&self, output: Value, task: &Arc<TaskCtx>) -> Vec<Arc<OperationNode>> {
        let next = self.index + 1;
        if next < self.chain.ops.len() {
            return vec![self.chain.node(next, output, task)];
        }
        if let Value::Host(data) = output {
            *self.chain.result.lock() = Some(data);
        }
        Vec::new()
    }
}

fn invalid(reason: impl Into<String>) -> TaskError {
    ResidentError::from(PlannerError::InvalidConfig { reason: reason.into() }).into()
}

/// `items` is the number of texts an embedding task carries.
fn into_output(kind: TaskKind, items: usize, data: Option<HostData>) -> Result<Output, TaskError> {
    match (kind, data) {
        (TaskKind::Embedding, Some(HostData::F32(flat))) => {
            if items == 0 || flat.len() % items != 0 {
                return Err(invalid(format!("{} values for {items} texts", flat.len())));
            }
            let dim = flat.len() / items;
            Ok(Output::Embeddings(flat.chunks(dim.max(1)).map(<[f32]>::to_vec).collect()))
        }
        (TaskKind::Similarity, Some(HostData::Hits(hits))) => Ok(Output::Hits(
            hits.into_iter().map(|(id, score)| (EntryId::new(id), score)).collect(),
        )),
        (TaskKind::Inference, Some(HostData::F32(scores))) => Ok(Output::Scores(scores)),
        (kind, Some(_)) => Err(invalid(format!("unexpected result type for {kind:?} task"))),
        (kind, None) => Err(invalid(format!("{kind:?} chain produced no host result"))),
    }
}

fn initial_input(payload: &Payload) -> (Value, usize) {
    match payload {
        Payload::Embedding { input, .. } => {
            (Value::Host(HostData::Text(input.texts().to_vec())), input.texts().len())
        }
        Payload::Similarity { input, .. } => {
            let flat = input.vectors().iter().flatten().copied().collect();
            (Value::Host(HostData::F32(flat)), 1)
        }
        Payload::Inference { input, .. } => {
            let pair = vec![input.query().to_owned(), input.chunk().to_owned()];
            (Value::Host(HostData::Text(pair)), 1)
        }
    }
}

impl Expander for PlannerExpander {
    fn first_resource(&self, meta: &TaskMeta) -> Option<ResourceKind> {
        Some(first_resource(meta.kind))
    }

    fn expand(&self, meta: &TaskMeta, payload: Payload, ctx: &Arc<TaskCtx>) -> Result<Expansion, ResidentError> {
        self.expand_hinted(meta, payload, ctx, StarvedSnapshot::default())
    }

    /// Only the first operation is enqueued; each finished operation yields its successor
    /// through [`Successor::follow`], so inputs are always the real predecessor output.
    fn expand_hinted(
        &self,
        meta: &TaskMeta,
        payload: Payload,
        ctx: &Arc<TaskCtx>,
        hint: StarvedSnapshot,
    ) -> Result<Expansion, ResidentError> {
        let ops = self
            .planner
            .expand(&TaskSpec::from_payload(&payload), hint, &self.offers)?;
        let (input, items) = initial_input(&payload);
        let chain = Arc::new(Chain { ops, result: Mutex::new(None) });

        let first = if chain.ops.is_empty() { Vec::new() } else { vec![chain.node(0, input, ctx)] };
        let kind = meta.kind;
        let collect: Collect = Box::new({
            let chain = chain.clone();
            move || into_output(kind, items, chain.result.lock().take())
        });
        Ok(Expansion { ops: first, collect })
    }
}

#[cfg(test)]
#[path = "test/adapter.rs"]
mod tests;
