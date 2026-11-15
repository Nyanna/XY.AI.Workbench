use super::*;
use std::path::Path;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::b::embedding_task::{
    ModelId as TaskModelId, Precision as EmbeddingPrecision,
};
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::b::similarity_task::IndexId as SimilarityIndexId;
const MODEL: &str = "e5";
fn pipeline() -> PipelineConfig {
    PipelineConfig {
        query_prefix: "query: ".to_string(),
        passage_prefix: "passage: ".to_string(),
        pooling: PoolingMethod::Mean,
        normalize: true,
        max_seq_len: 512,
        dtype: DType::F16,
        dimension: 384,
    }
}
fn pipelines_with(config: PipelineConfig) -> PipelineConfigs {
    let mut map = PipelineConfigs::new();
    map.insert(MODEL.to_string(), config);
    map
}
fn pipelines() -> PipelineConfigs {
    pipelines_with(pipeline())
}
fn embedding(mode: EmbeddingMode, target: Option<&str>) -> TaskSpec {
    TaskSpec::Embedding {
        task: EmbeddingTask::new(
            "hello".to_string(),
            TaskModelId(MODEL.to_string()),
            mode,
            EmbeddingPrecision::F32,
        ),
        target_index: target.map(IndexId::new),
    }
}
fn similarity(scorer: ScorerChoice, top_k: usize) -> TaskSpec {
    TaskSpec::Similarity(SimilaritySpec {
        params: SimilarityParams::new(SimilarityIndexId(42))
            .with_top_k(top_k)
            .with_scope(Scope::Subtree { root: 7 }),
        model: ModelId::new(MODEL),
        scorer,
    })
}
fn cross_encoder_caps() -> CapabilitySet {
    CapabilitySet::new().with(CROSS_ENCODER_AXIS, "minilm")
}
fn kinds(ops: &[PlannedOperation]) -> Vec<OperationKind> {
    ops.iter().map(|o| o.kind).collect()
}
fn idle(devices: &[DeviceKind]) -> IdleNotification {
    IdleNotification {
        idle_devices: devices.to_vec(),
    }
}
fn run_embedding(
    spec: &TaskSpec,
    idle: Option<&IdleNotification>,
) -> Vec<PlannedOperation> {
    expand(spec, &pipelines(), &CapabilitySet::new(), idle).unwrap()
}
#[test]
fn from_descriptor_maps_fields() {
    let mut d = ModelDescriptor::e5_small_default(Path::new("/tmp/model"), DType::F16);
    let cfg = PipelineConfig::from_descriptor(&d);
    assert_eq!(cfg.query_prefix, "query: ");
    assert_eq!(cfg.passage_prefix, "passage: ");
    assert_eq!(cfg.pooling, PoolingMethod::Mean);
    assert!(cfg.normalize);
    assert_eq!(cfg.max_seq_len, 512);
    assert_eq!(cfg.dtype, DType::F16);
    assert_eq!(cfg.dimension, 384);
    d.pooling = Pooling::Cls;
    d.normalization = Normalization::None;
    d.storage_dtype = DType::F32;
    let cfg = PipelineConfig::from_descriptor(&d);
    assert_eq!(cfg.pooling, PoolingMethod::Cls);
    assert!(! cfg.normalize);
    assert_eq!(cfg.dtype, DType::F32);
}
#[test]
fn embedding_full_chain_order_and_configs() {
    let ops = run_embedding(&embedding(EmbeddingMode::Query, Some("idx")), None);
    assert_eq!(
        kinds(& ops), vec![OperationKind::Tokenization, OperationKind::Embedding,
        OperationKind::Pooling, OperationKind::L2Normalization,
        OperationKind::IndexWrite,]
    );
    let model = ModelId::new(MODEL);
    let tok = &ops[0];
    assert_eq!(tok.resource.kind(), ResourceKind::Tokenizer);
    assert_eq!(tok.resource.environment(), ExecutionEnvironment::CpuOnly);
    assert_eq!(tok.config.tokenizer, Some(model.clone()));
    assert_eq!(tok.config.prefix.as_deref(), Some("query: "));
    assert_eq!(tok.config.max_seq_len, Some(512));
    let emb = &ops[1];
    assert_eq!(emb.resource.kind(), ResourceKind::Encoder);
    assert_eq!(emb.config.model, Some(model.clone()));
    assert_eq!(emb.config.dtype, Some(DType::F16));
    assert_eq!(emb.config.target_dimension, Some(384));
    assert_eq!(emb.descriptor.params().dimension(), Some(384));
    assert_eq!(ops[2].config.pooling, Some(PoolingMethod::Mean));
    assert_eq!(ops[3].config.model, Some(model.clone()));
    let write = &ops[4];
    assert_eq!(write.resource.kind(), ResourceKind::Index);
    assert_eq!(write.resource.environment(), ExecutionEnvironment::Any);
    assert_eq!(write.config.index, Some(IndexId::new("idx")));
    assert_eq!(write.resource.index(), Some(& IndexId::new("idx")));
}
#[test]
fn embedding_resource_has_model_but_no_affinity_or_capabilities() {
    let ops = run_embedding(&embedding(EmbeddingMode::Query, None), None);
    for op in &ops {
        assert_eq!(op.resource.model(), Some(& ModelId::new(MODEL)));
        assert!(op.resource.affinity().is_none());
        assert!(op.resource.capabilities().is_empty());
        assert!(op.resource.index().is_none());
    }
}
#[test]
fn embedding_descriptor_resource_names_and_mode() {
    let ops = run_embedding(&embedding(EmbeddingMode::Passage, Some("idx")), None);
    let names: Vec<_> = ops
        .iter()
        .map(|o| o.descriptor.requirements().resource.clone().unwrap())
        .collect();
    assert_eq!(names, ["tokenizer", "encoder", "encoder", "encoder", "index"]);
    for op in &ops {
        let req = op.descriptor.requirements();
        assert_eq!(req.model.as_deref(), Some(MODEL));
        assert_eq!(req.mode.as_deref(), Some("passage"));
        assert_eq!(op.descriptor.kind(), op.kind);
        assert_eq!(op.descriptor.params().top_k(), None);
    }
    assert_eq!(ops[0].config.prefix.as_deref(), Some("passage: "));
}
#[test]
fn embedding_mode_query_in_descriptor() {
    let ops = run_embedding(&embedding(EmbeddingMode::Query, None), None);
    assert!(
        ops.iter().all(| o | o.descriptor.requirements().mode.as_deref() ==
        Some("query"))
    );
}
#[test]
fn embedding_without_pooling_keeps_token_vectors() {
    let mut cfg = pipeline();
    cfg.pooling = PoolingMethod::None;
    let ops = expand(
            &embedding(EmbeddingMode::Query, Some("idx")),
            &pipelines_with(cfg),
            &CapabilitySet::new(),
            None,
        )
        .unwrap();
    assert_eq!(
        kinds(& ops), vec![OperationKind::Tokenization, OperationKind::Embedding,
        OperationKind::L2Normalization, OperationKind::IndexWrite,]
    );
}
#[test]
fn embedding_without_normalization_skips_l2() {
    let mut cfg = pipeline();
    cfg.normalize = false;
    let ops = expand(
            &embedding(EmbeddingMode::Query, Some("idx")),
            &pipelines_with(cfg),
            &CapabilitySet::new(),
            None,
        )
        .unwrap();
    assert_eq!(
        kinds(& ops), vec![OperationKind::Tokenization, OperationKind::Embedding,
        OperationKind::Pooling, OperationKind::IndexWrite,]
    );
}
#[test]
fn embedding_minimal_chain_has_two_operations() {
    let mut cfg = pipeline();
    cfg.pooling = PoolingMethod::None;
    cfg.normalize = false;
    let ops = expand(
            &embedding(EmbeddingMode::Query, None),
            &pipelines_with(cfg),
            &CapabilitySet::new(),
            None,
        )
        .unwrap();
    assert_eq!(
        kinds(& ops), vec![OperationKind::Tokenization, OperationKind::Embedding]
    );
}
#[test]
fn embedding_without_target_index_has_no_write() {
    let ops = run_embedding(&embedding(EmbeddingMode::Query, None), None);
    assert!(! kinds(& ops).contains(& OperationKind::IndexWrite));
    assert_eq!(ops.len(), 4);
}
#[test]
fn embedding_unknown_model_id_conversion_is_verbatim() {
    let mut map = PipelineConfigs::new();
    map.insert("org/Model-1".to_string(), pipeline());
    let spec = TaskSpec::Embedding {
        task: EmbeddingTask::new(
            "hello".to_string(),
            TaskModelId("org/Model-1".to_string()),
            EmbeddingMode::Query,
            EmbeddingPrecision::F32,
        ),
        target_index: None,
    };
    let ops = expand(&spec, &map, &CapabilitySet::new(), None).unwrap();
    assert!(
        ops.iter().all(| o | o.resource.model().map(ModelId::as_str) ==
        Some("org/Model-1"))
    );
}
#[test]
fn encoder_environment_follows_idle_devices() {
    let spec = embedding(EmbeddingMode::Query, Some("idx"));
    let cases: Vec<(Option<IdleNotification>, ExecutionEnvironment)> = vec![
        (None, ExecutionEnvironment::Any), (Some(idle(& [])), ExecutionEnvironment::Any),
        (Some(idle(& [DeviceKind::Gpu])), ExecutionEnvironment::GpuOnly), (Some(idle(&
        [DeviceKind::Gpu, DeviceKind::Gpu])), ExecutionEnvironment::GpuOnly,),
        (Some(idle(& [DeviceKind::Cpu])), ExecutionEnvironment::CpuOnly), (Some(idle(&
        [DeviceKind::Cpu, DeviceKind::Gpu])), ExecutionEnvironment::Any,),
    ];
    for (notification, expected) in cases {
        let ops = run_embedding(&spec, notification.as_ref());
        for op in &ops {
            let env = op.resource.environment();
            match op.kind {
                OperationKind::Embedding
                | OperationKind::Pooling
                | OperationKind::L2Normalization => assert_eq!(env, expected),
                OperationKind::Tokenization => {
                    assert_eq!(env, ExecutionEnvironment::CpuOnly)
                }
                OperationKind::IndexWrite => assert_eq!(env, ExecutionEnvironment::Any),
                _ => unreachable!(),
            }
        }
    }
}
#[test]
fn vector_similarity_chain() {
    let spec = similarity(ScorerChoice::Vector(Scoring::DotProduct), 5);
    let ops = expand(&spec, &pipelines(), &CapabilitySet::new(), None).unwrap();
    assert_eq!(kinds(& ops), vec![OperationKind::Similarity, OperationKind::TopK]);
    let index = IndexId::new("42");
    let model = ModelId::new(MODEL);
    let sim = &ops[0];
    assert_eq!(sim.resource.kind(), ResourceKind::Scorer);
    assert_eq!(sim.resource.environment(), ExecutionEnvironment::Any);
    assert_eq!(sim.resource.model(), Some(& model));
    assert_eq!(sim.resource.index(), Some(& index));
    assert_eq!(sim.config.model, Some(model.clone()));
    assert_eq!(sim.config.scorer.as_deref(), Some("DotProduct"));
    assert_eq!(sim.config.index, Some(index.clone()));
    assert_eq!(sim.config.scope, Some(Scope::Subtree { root : 7 }));
    assert_eq!(sim.config.target_dimension, Some(384));
    assert_eq!(sim.config.dtype, Some(DType::F16));
    assert_eq!(sim.descriptor.params().dimension(), Some(384));
    assert_eq!(sim.descriptor.params().top_k(), None);
    let top = &ops[1];
    assert_eq!(top.resource.kind(), ResourceKind::Scorer);
    assert_eq!(top.resource.index(), Some(& index));
    assert_eq!(top.config.top_k, Some(5));
    assert_eq!(top.config.index, Some(index));
    assert_eq!(top.descriptor.params().top_k(), Some(5));
    assert_eq!(top.descriptor.params().dimension(), None);
    for op in &ops {
        let req = op.descriptor.requirements();
        assert_eq!(req.resource.as_deref(), Some("scorer"));
        assert_eq!(req.model.as_deref(), Some(MODEL));
        assert_eq!(req.mode, None);
    }
}
#[test]
fn vector_similarity_scorer_name_is_debug_name() {
    for (scoring, name) in [
        (Scoring::Cosine, "Cosine"),
        (Scoring::DotProduct, "DotProduct"),
        (Scoring::NegEuclidean, "NegEuclidean"),
    ] {
        let ops = expand(
                &similarity(ScorerChoice::Vector(scoring), 3),
                &pipelines(),
                &CapabilitySet::new(),
                None,
            )
            .unwrap();
        assert_eq!(ops[0].config.scorer.as_deref(), Some(name));
    }
}
#[test]
fn top_k_is_clamped_to_u32() {
    let spec = similarity(ScorerChoice::Vector(Scoring::Cosine), usize::MAX);
    let ops = expand(&spec, &pipelines(), &CapabilitySet::new(), None).unwrap();
    assert_eq!(ops[1].config.top_k, Some(u32::MAX));
    assert_eq!(ops[1].descriptor.params().top_k(), Some(u32::MAX as usize));
}
#[test]
fn similarity_ignores_idle_notification() {
    let spec = similarity(ScorerChoice::Vector(Scoring::Cosine), 3);
    let gpu = idle(&[DeviceKind::Gpu]);
    let ops = expand(&spec, &pipelines(), &CapabilitySet::new(), Some(&gpu)).unwrap();
    assert!(ops.iter().all(| o | o.resource.environment() == ExecutionEnvironment::Any));
}
#[test]
fn cross_encoder_chain() {
    let spec = similarity(ScorerChoice::CrossEncoder("minilm".to_string()), 4);
    let ops = expand(&spec, &pipelines(), &cross_encoder_caps(), None).unwrap();
    assert_eq!(
        kinds(& ops), vec![OperationKind::Tokenization, OperationKind::Inference,
        OperationKind::Similarity,]
    );
    let index = IndexId::new("42");
    let model = ModelId::new(MODEL);
    let tok = &ops[0];
    assert_eq!(tok.resource.kind(), ResourceKind::Tokenizer);
    assert_eq!(tok.resource.environment(), ExecutionEnvironment::CpuOnly);
    assert_eq!(tok.resource.index(), Some(& index));
    assert_eq!(tok.config.tokenizer, Some(model.clone()));
    assert_eq!(tok.config.max_seq_len, Some(512));
    assert_eq!(tok.config.scope, Some(Scope::Subtree { root : 7 }));
    assert_eq!(tok.config.index, Some(index.clone()));
    let inf = &ops[1];
    assert_eq!(inf.resource.kind(), ResourceKind::CrossEncoder);
    assert_eq!(inf.resource.environment(), ExecutionEnvironment::Any);
    assert!(inf.resource.index().is_none());
    assert_eq!(inf.config.model, Some(model.clone()));
    assert_eq!(inf.config.scorer.as_deref(), Some("minilm"));
    assert_eq!(inf.config.dtype, Some(DType::F16));
    let sim = &ops[2];
    assert_eq!(sim.resource.kind(), ResourceKind::CrossEncoder);
    assert_eq!(sim.resource.index(), Some(& index));
    assert_eq!(sim.config.scorer.as_deref(), Some("minilm"));
    assert_eq!(sim.config.index, Some(index));
    assert_eq!(sim.config.scope, Some(Scope::Subtree { root : 7 }));
    assert_eq!(sim.config.top_k, Some(4));
    assert_eq!(sim.descriptor.params().top_k(), Some(4));
    let names: Vec<_> = ops
        .iter()
        .map(|o| o.descriptor.requirements().resource.clone().unwrap())
        .collect();
    assert_eq!(names, ["tokenizer", "cross_encoder", "cross_encoder"]);
    assert!(ops.iter().all(| o | o.descriptor.requirements().mode.is_none()));
}
#[test]
fn cross_encoder_unavailable() {
    let spec = similarity(ScorerChoice::CrossEncoder("other".to_string()), 4);
    let err = expand(&spec, &pipelines(), &cross_encoder_caps(), None).unwrap_err();
    assert_eq!(err, ExpandError::ScoringUnavailable { scorer : "other".to_string() });
    assert_eq!(err.to_string(), "Scoring-Verfahren nicht verfügbar: other");
    let err = expand(&spec, &pipelines(), &CapabilitySet::new(), None).unwrap_err();
    assert!(matches!(err, ExpandError::ScoringUnavailable { .. }));
}
#[test]
fn cross_encoder_requires_the_dedicated_axis() {
    let caps = CapabilitySet::new().with("scoring", "minilm");
    let spec = similarity(ScorerChoice::CrossEncoder("minilm".to_string()), 4);
    assert!(expand(& spec, & pipelines(), & caps, None).is_err());
}
#[test]
fn zero_top_k_is_rejected_for_both_scorers() {
    let vector = similarity(ScorerChoice::Vector(Scoring::Cosine), 0);
    let cross = similarity(ScorerChoice::CrossEncoder("minilm".to_string()), 0);
    for spec in [vector, cross] {
        let err = expand(&spec, &pipelines(), &cross_encoder_caps(), None).unwrap_err();
        assert_eq!(err, ExpandError::Config(ConfigError::ZeroTopK));
        assert_eq!(
            err.to_string(), "Konfigurationsfehler: top_k muss größer als 0 sein"
        );
    }
}
#[test]
fn missing_pipeline_config_for_embedding_and_similarity() {
    let empty = PipelineConfigs::new();
    let err = expand(
            &embedding(EmbeddingMode::Query, None),
            &empty,
            &CapabilitySet::new(),
            None,
        )
        .unwrap_err();
    assert_eq!(err, ExpandError::MissingPipelineConfig { model : MODEL.to_string() });
    assert_eq!(err.to_string(), "keine Pipeline-Konfiguration für Modell e5");
    let err = expand(
            &similarity(ScorerChoice::Vector(Scoring::Cosine), 3),
            &empty,
            &CapabilitySet::new(),
            None,
        )
        .unwrap_err();
    assert!(matches!(err, ExpandError::MissingPipelineConfig { .. }));
}
#[test]
fn foreign_task_kind() {
    let spec = TaskSpec::Other {
        kind: "Rerank".to_string(),
    };
    let err = expand(&spec, &pipelines(), &CapabilitySet::new(), None).unwrap_err();
    assert_eq!(
        err, ExpandError::ForeignTaskKind { expected : "Embedding/Similarity"
        .to_string(), got : "Rerank".to_string() }
    );
    assert_eq!(
        err.to_string(),
        "fremde Task-Art: erwartet Embedding/Similarity, erhalten Rerank"
    );
}
#[test]
fn check_order_kind_then_pipeline_then_top_k_then_scorer() {
    let empty = PipelineConfigs::new();
    let foreign = TaskSpec::Other {
        kind: "x".to_string(),
    };
    assert!(
        matches!(expand(& foreign, & empty, & CapabilitySet::new(), None),
        Err(ExpandError::ForeignTaskKind { .. }))
    );
    let bad = similarity(ScorerChoice::CrossEncoder("nope".to_string()), 0);
    assert!(
        matches!(expand(& bad, & empty, & CapabilitySet::new(), None),
        Err(ExpandError::MissingPipelineConfig { .. }))
    );
    assert_eq!(
        expand(& bad, & pipelines(), & CapabilitySet::new(), None).unwrap_err(),
        ExpandError::Config(ConfigError::ZeroTopK)
    );
}
#[test]
fn expand_is_pure() {
    let spec = embedding(EmbeddingMode::Query, Some("idx"));
    let gpu = idle(&[DeviceKind::Gpu]);
    let a = run_embedding(&spec, Some(&gpu));
    let b = run_embedding(&spec, Some(&gpu));
    assert_eq!(a, b);
    let sim = similarity(ScorerChoice::CrossEncoder("minilm".to_string()), 9);
    let a = expand(&sim, &pipelines(), &cross_encoder_caps(), None).unwrap();
    let b = expand(&sim, &pipelines(), &cross_encoder_caps(), None).unwrap();
    assert_eq!(a, b);
}
#[test]
fn descriptors_are_usable_as_keys() {
    use std::collections::HashSet;
    let ops = run_embedding(&embedding(EmbeddingMode::Query, Some("idx")), None);
    let set: HashSet<_> = ops.iter().map(|o| o.descriptor.clone()).collect();
    assert_eq!(set.len(), ops.len());
}
#[test]
fn concurrent_calls_agree() {
    let spec = std::sync::Arc::new(embedding(EmbeddingMode::Passage, Some("idx")));
    let pipelines = std::sync::Arc::new(pipelines());
    let caps = std::sync::Arc::new(CapabilitySet::new());
    let expected = expand(&spec, &pipelines, &caps, None).unwrap();
    let handles: Vec<_> = (0..4)
        .map(|_| {
            let (spec, pipelines, caps) = (
                spec.clone(),
                pipelines.clone(),
                caps.clone(),
            );
            std::thread::spawn(move || expand(&spec, &pipelines, &caps, None).unwrap())
        })
        .collect();
    for h in handles {
        assert_eq!(h.join().unwrap(), expected);
    }
}
#[test]
fn error_types_implement_error() {
    fn assert_error<E: std::error::Error>() {}
    assert_error::<ExpandError>();
    assert_error::<ConfigError>();
}
