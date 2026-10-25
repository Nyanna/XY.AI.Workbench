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
    echo: Option<Echo>,
}
type Echo = Box<dyn Fn(&[f32]) -> Vec<f32> + Send + Sync>;
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
            _ => {
                let canned = [0.5f32; 4];
                HostData::F32(
                    match &self.echo {
                        Some(echo) => echo(&canned),
                        None => canned.to_vec(),
                    },
                )
            }
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
    run_with(payload, kind, None)
}
fn run_with(
    payload: Payload,
    kind: TaskKind,
    echo: Option<Echo>,
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
        echo,
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
#[cfg(feature = "cuda")]
mod device {
    use std::sync::Arc;
    use super::*;
    use crate::resident::memory::arena::DeviceMemory;
    use crate::resident::memory::cuda::CudaMemoryManager;
    use crate::resident::memory::{ArenaKind, ArenaSizes};
    use crate::resident::subengine::DeviceHandle;
    fn device_echo() -> Echo {
        let dev = DeviceHandle::cuda(0).unwrap();
        let sizes = ArenaSizes {
            weights: 1 << 20,
            index_segment: 1 << 20,
            workspace: 1 << 20,
            staging: 0,
            param_block: 4096,
            mapped: 0,
        };
        let memory = Arc::new(CudaMemoryManager::new(&dev, sizes, 0, 2).unwrap());
        let stream = Mutex::new(dev.cuda_context().unwrap().new_stream().unwrap());
        Box::new(move |values| {
            let bytes: Vec<u8> = values.iter().flat_map(|v| v.to_le_bytes()).collect();
            let region = memory
                .alloc_in(ArenaKind::IndexSegment, bytes.len() as u64, 256)
                .unwrap();
            let mut back = vec![0u8; bytes.len()];
            let stream = stream.lock();
            unsafe {
                memory.upload(&stream, &bytes, region).unwrap();
                memory.download(&stream, region, &mut back).unwrap();
            }
            stream.synchronize().unwrap();
            memory.free(region).unwrap();
            back.chunks_exact(4)
                .map(|c| f32::from_le_bytes([c[0], c[1], c[2], c[3]]))
                .collect()
        })
    }
    #[test]
    #[ignore = "requires a CUDA device"]
    fn extension_task_runs_through_workers_with_device_round_trips() {
        let payload = |ext: bool| {
            let params = EmbeddingParams::new(TextRole::Passage);
            Payload::Embedding {
                input: TextInput::new(["a"]),
                params: if ext { params.with_ext(42_u64) } else { params },
            }
        };
        let (plain, plain_seen) = run_with(
            payload(false),
            TaskKind::Embedding,
            Some(device_echo()),
        );
        let (ext, ext_seen) = run_with(
            payload(true),
            TaskKind::Embedding,
            Some(device_echo()),
        );
        assert_eq!(plain_seen, ext_seen);
        let (plain, ext) = (plain.unwrap(), ext.unwrap());
        assert_eq!(plain, ext);
        let (_, reference) = run(payload(true), TaskKind::Embedding);
        assert_eq!(ext_seen, reference);
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
