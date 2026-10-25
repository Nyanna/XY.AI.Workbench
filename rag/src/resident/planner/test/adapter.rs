use std::time::{Duration, Instant};
use super::*;
use crate::resident::facade::{
    EmbeddingParams, InferenceParams, SimilarityParams, TaskQueue, TextInput, TextRole,
    VectorInput,
};
use crate::resident::op::{IndexId, OpKind};
use crate::resident::planner::RuleSet;
use crate::resident::processor::{
    Engine, Hub, Processor, Side, Step as EngineStep, Wakeups, WorkerConfig,
};
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
fn run(
    payload: Payload,
    kind: TaskKind,
) -> (Result<Output, TaskError>, Vec<(OpKind, &'static str)>) {
    let planner = Arc::new(Planner::new(RuleSet::from_toml(TOML).unwrap()));
    let offers = EngineOffers::new()
        .with_cpu([
            ResourceKind::Tokenizer,
            ResourceKind::Encoder,
            ResourceKind::Scorer,
            ResourceKind::Index,
        ]);
    let engine = Arc::new(Scripted {
        seen: Mutex::new(vec![]),
    });
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
        seen, vec![(OpKind::Tokenize, "text"), (OpKind::Embed, "tokens"), (OpKind::Pool,
        "f32"), (OpKind::L2Norm, "f32"),]
    );
    match result.unwrap() {
        Output::Embeddings(rows) => assert_eq!(rows, vec![vec![0.5; 2], vec![0.5; 2]]),
        other => panic!("unexpected output {other:?}"),
    }
}
#[test]
fn foreign_extension_does_not_change_the_planned_chain() {
    let plain = Payload::Embedding {
        input: TextInput::new(["a"]),
        params: EmbeddingParams::new(TextRole::Passage),
    };
    let extended = Payload::Embedding {
        input: TextInput::new(["a"]),
        params: EmbeddingParams::new(TextRole::Passage).with_ext(42_u64),
    };
    let (plain_result, plain_seen) = run(plain, TaskKind::Embedding);
    let (ext_result, ext_seen) = run(extended, TaskKind::Embedding);
    assert_eq!(plain_seen, ext_seen);
    assert_eq!(plain_result.unwrap(), ext_result.unwrap());
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
