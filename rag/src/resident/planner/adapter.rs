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
mod tests {
    use std::time::{Duration, Instant};

    use super::*;
    use crate::resident::facade::{
        EmbeddingParams, InferenceParams, SimilarityParams, TaskQueue, TextInput, TextRole,
        VectorInput,
    };
    use crate::resident::op::{IndexId, OpKind};
    use crate::resident::planner::RuleSet;
    use crate::resident::processor::{Engine, Hub, Processor, Side, Step as EngineStep, Wakeups, WorkerConfig};
    use crate::resident::queue::{Queue, TaskState};

    const TOML: &str = r#"
        default_model = "e5-small"

        [models.e5-small]
        pooling = "mean"
        normalize = true
        max_seq_len = 512
        dtype = "f16"

        [models.e5-small.prefixes]
        query = "query: "
        passage = "passage: "
    "#;

    /// Executes every operation with canned output and records what each one received.
    struct Scripted {
        seen: Mutex<Vec<(OpKind, &'static str)>>,
    }

    fn shape(value: &Value) -> &'static str {
        match value {
            Value::Host(HostData::Text(_)) => "text",
            Value::Host(HostData::Tokens(_)) => "tokens",
            Value::Host(HostData::F32(_)) => "f32",
            Value::Host(HostData::Hits(_)) => "hits",
            _ => "other",
        }
    }

    impl Engine for Scripted {
        fn step(&self, hub: &Hub) -> EngineStep {
            let Some(job) = hub.claim(Side::Cpu, &|_| true) else {
                return EngineStep::Idle(None);
            };
            let kind = job.node.op.kind();
            self.seen.lock().push((kind, shape(&job.node.input)));
            let data = match kind {
                OpKind::Tokenize => HostData::Tokens(vec![vec![1]]),
                OpKind::TopK => HostData::Hits(vec![(7, 0.9)]),
                _ => HostData::F32(vec![0.5; 4]),
            };
            let followups = match &job.node.successor {
                Some(next) => next.follow(Value::Host(data), &job.node.task),
                None => Vec::new(),
            };
            hub.complete_op(job, followups);
            EngineStep::Worked
        }
    }

    fn run(payload: Payload, kind: TaskKind) -> (Result<Output, TaskError>, Vec<(OpKind, &'static str)>) {
        let planner = Arc::new(Planner::new(RuleSet::from_toml(TOML).unwrap()));
        let offers = EngineOffers::new().with_cpu([
            ResourceKind::Tokenizer,
            ResourceKind::Encoder,
            ResourceKind::Scorer,
            ResourceKind::Index,
        ]);
        let engine = Arc::new(Scripted { seen: Mutex::new(vec![]) });
        let wakeups = Wakeups::new();
        let tasks: Arc<TaskQueue> = Arc::new(Queue::new(wakeups.relays()));
        let _processor = Processor::start(
            WorkerConfig::new(1, 0),
            tasks.clone(),
            wakeups,
            Arc::new(PlannerExpander::new(planner, offers)),
            engine.clone(),
            None,
        )
        .unwrap();

        let state = Arc::new(TaskState::new());
        tasks.push(TaskMeta { id: 1, kind }, state.clone(), payload);
        let deadline = Instant::now() + Duration::from_secs(5);
        while state.result().is_none() {
            assert!(Instant::now() < deadline, "task did not complete");
            std::thread::sleep(Duration::from_millis(1));
        }
        let result = state.result().unwrap().clone();
        let seen = engine.seen.lock().clone();
        (result, seen)
    }

    #[test]
    fn embedding_chain_passes_each_output_to_the_next_op() {
        let payload = Payload::Embedding {
            input: TextInput::new(["a", "b"]),
            params: EmbeddingParams::new(TextRole::Passage),
        };
        let (result, seen) = run(payload, TaskKind::Embedding);
        assert_eq!(
            seen,
            vec![
                (OpKind::Tokenize, "text"),
                (OpKind::Embed, "tokens"),
                (OpKind::Pool, "f32"),
                (OpKind::L2Norm, "f32"),
            ]
        );
        match result.unwrap() {
            Output::Embeddings(rows) => assert_eq!(rows, vec![vec![0.5; 2], vec![0.5; 2]]),
            other => panic!("unexpected output {other:?}"),
        }
    }

    #[test]
    fn similarity_returns_hits() {
        let payload = Payload::Similarity {
            input: VectorInput::new(vec![vec![0.1, 0.2]]),
            params: SimilarityParams::new(IndexId("i".into())),
        };
        let (result, seen) = run(payload, TaskKind::Similarity);
        assert_eq!(seen[0], (OpKind::Similarity, "f32"));
        match result.unwrap() {
            Output::Hits(hits) => assert_eq!(hits, vec![(EntryId::new(7), 0.9)]),
            other => panic!("unexpected output {other:?}"),
        }
    }

    #[test]
    fn planner_error_fails_the_task() {
        let payload = Payload::Inference {
            input: crate::resident::facade::PairInput::new("q", "c"),
            params: InferenceParams::new().model(crate::resident::op::ModelId("nope".into())),
        };
        let (result, seen) = run(payload, TaskKind::Inference);
        assert!(result.is_err());
        assert!(seen.is_empty());
    }

    #[test]
    fn missing_result_is_an_error() {
        let err = into_output(TaskKind::Similarity, 1, None).unwrap_err();
        assert!(matches!(err, TaskError::Engine(_)));
    }
}
