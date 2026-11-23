//! Pure decomposition of a task into an ordered chain of planned operations.
use std::collections::HashMap;
use std::fmt;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::a::resource_requirements::{
    ExecutionEnvironment, IndexId, ModelId, ResourceKind, ResourceRequirements,
};
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::b::embedding_task::{
    EmbeddingMode, EmbeddingTask,
};
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::b::similarity_task::{
    Scoring, SimilarityParams,
};
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::b::operation::{
    OperationDescriptor, OperationKind, OperationParams,
    ResourceRequirements as DescriptorRequirements,
};
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::a::model_descriptor::{
    ModelDescriptor, Normalization, Pooling,
};
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::l010::capability_model::{
    CapabilitySet, Name,
};
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::l010::device_lane_event::DeviceKind;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::l010::layout::DType;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::l010::scopes_and_entry_sets::Scope;
const EXPECTED_TASK_KINDS: &str = "Embedding/Similarity";
const CROSS_ENCODER_AXIS: &str = "cross_encoder";
#[non_exhaustive]
#[derive(Clone)]
pub enum TaskSpec {
    /// `task.target` is ignored; writing into the index depends only on `target_index`.
    Embedding { task: EmbeddingTask, target_index: Option<IndexId> },
    Similarity(SimilaritySpec),
    Other { kind: String },
}
#[derive(Clone, Debug)]
pub struct SimilaritySpec {
    pub params: SimilarityParams,
    pub model: ModelId,
    pub scorer: ScorerChoice,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ScorerChoice {
    Vector(Scoring),
    CrossEncoder(String),
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PoolingMethod {
    Mean,
    Cls,
    None,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PipelineConfig {
    pub query_prefix: String,
    pub passage_prefix: String,
    pub pooling: PoolingMethod,
    pub normalize: bool,
    pub max_seq_len: usize,
    pub dtype: DType,
    pub dimension: usize,
}
impl PipelineConfig {
    pub fn from_descriptor(d: &ModelDescriptor) -> Self {
        Self {
            query_prefix: d.query_prefix.clone(),
            passage_prefix: d.passage_prefix.clone(),
            pooling: match d.pooling {
                Pooling::Mean => PoolingMethod::Mean,
                Pooling::Cls => PoolingMethod::Cls,
            },
            normalize: d.normalization == Normalization::L2,
            max_seq_len: d.max_seq_len,
            dtype: d.storage_dtype,
            dimension: d.dimension,
        }
    }
}
pub type PipelineConfigs = HashMap<String, PipelineConfig>;
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct IdleNotification {
    pub idle_devices: Vec<DeviceKind>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct PlannedOperation {
    pub kind: OperationKind,
    pub descriptor: OperationDescriptor,
    pub resource: ResourceRequirements,
    pub config: OperationConfig,
}
#[derive(Clone, Debug, Default, PartialEq)]
pub struct OperationConfig {
    pub index: Option<IndexId>,
    pub tokenizer: Option<ModelId>,
    pub model: Option<ModelId>,
    pub scorer: Option<String>,
    pub pooling: Option<PoolingMethod>,
    pub prefix: Option<String>,
    pub target_dimension: Option<usize>,
    pub top_k: Option<u32>,
    pub max_seq_len: Option<usize>,
    pub dtype: Option<DType>,
    pub scope: Option<Scope>,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ConfigError {
    ZeroTopK,
}
impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::ZeroTopK => f.write_str("top_k muss größer als 0 sein"),
        }
    }
}
impl std::error::Error for ConfigError {}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ExpandError {
    ForeignTaskKind { expected: String, got: String },
    MissingPipelineConfig { model: String },
    Config(ConfigError),
    ScoringUnavailable { scorer: String },
}
impl fmt::Display for ExpandError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExpandError::ForeignTaskKind { expected, got } => {
                write!(f, "fremde Task-Art: erwartet {expected}, erhalten {got}")
            }
            ExpandError::MissingPipelineConfig { model } => {
                write!(f, "keine Pipeline-Konfiguration für Modell {model}")
            }
            ExpandError::Config(e) => write!(f, "Konfigurationsfehler: {e}"),
            ExpandError::ScoringUnavailable { scorer } => {
                write!(f, "Scoring-Verfahren nicht verfügbar: {scorer}")
            }
        }
    }
}
impl std::error::Error for ExpandError {}
const _: () = {
    const fn assert_shareable<T: Send + Sync + 'static>() {}
    assert_shareable::<TaskSpec>();
    assert_shareable::<SimilaritySpec>();
    assert_shareable::<ScorerChoice>();
    assert_shareable::<PoolingMethod>();
    assert_shareable::<PipelineConfig>();
    assert_shareable::<PipelineConfigs>();
    assert_shareable::<IdleNotification>();
    assert_shareable::<PlannedOperation>();
    assert_shareable::<OperationConfig>();
    assert_shareable::<ConfigError>();
    assert_shareable::<ExpandError>();
};
pub fn expand(
    task: &TaskSpec,
    pipelines: &PipelineConfigs,
    capabilities: &CapabilitySet,
    idle: Option<&IdleNotification>,
) -> Result<Vec<PlannedOperation>, ExpandError> {
    let (label, chain) = match task {
        TaskSpec::Embedding { task, target_index } => {
            (
                "embedding",
                expand_embedding(task, target_index.as_ref(), pipelines, idle)?,
            )
        }
        TaskSpec::Similarity(spec) => expand_similarity(spec, pipelines, capabilities)?,
        TaskSpec::Other { kind } => {
            return Err(ExpandError::ForeignTaskKind {
                expected: EXPECTED_TASK_KINDS.to_string(),
                got: kind.clone(),
            });
        }
    };
    log::debug!("expand: task kind {label}, {} operations", chain.len());
    Ok(chain)
}
fn lookup_pipeline<'a>(
    pipelines: &'a PipelineConfigs,
    model: &str,
) -> Result<&'a PipelineConfig, ExpandError> {
    pipelines
        .get(model)
        .ok_or_else(|| ExpandError::MissingPipelineConfig {
            model: model.to_string(),
        })
}
fn encoder_environment(idle: Option<&IdleNotification>) -> ExecutionEnvironment {
    let Some(idle) = idle else {
        return ExecutionEnvironment::Any;
    };
    let has_cpu = idle.idle_devices.iter().any(|d| *d == DeviceKind::Cpu);
    let has_gpu = idle.idle_devices.iter().any(|d| *d == DeviceKind::Gpu);
    match (has_cpu, has_gpu) {
        (true, false) => ExecutionEnvironment::CpuOnly,
        (false, true) => ExecutionEnvironment::GpuOnly,
        _ => ExecutionEnvironment::Any,
    }
}
fn resource_name(kind: ResourceKind) -> &'static str {
    match kind {
        ResourceKind::Tokenizer => "tokenizer",
        ResourceKind::Normalizer => "normalizer",
        ResourceKind::Encoder => "encoder",
        ResourceKind::Scorer => "scorer",
        ResourceKind::Index => "index",
        ResourceKind::CrossEncoder => "cross_encoder",
    }
}
fn plan(
    kind: OperationKind,
    resource_kind: ResourceKind,
    environment: ExecutionEnvironment,
    model: &ModelId,
    mode: Option<&str>,
    config: OperationConfig,
) -> PlannedOperation {
    let mut resource = ResourceRequirements::new(resource_kind, environment)
        .with_model(model.clone());
    if let Some(index) = &config.index {
        resource = resource.with_index(index.clone());
    }
    let mut params = OperationParams::new(DescriptorRequirements {
        resource: Some(resource_name(resource_kind).to_string()),
        model: Some(model.as_str().to_string()),
        mode: mode.map(str::to_string),
    });
    if let Some(dimension) = config.target_dimension {
        params = params.with_dimension(dimension);
    }
    if let Some(k) = config.top_k {
        params = params.with_top_k(k as usize);
    }
    PlannedOperation {
        kind,
        descriptor: OperationDescriptor::new(kind, params),
        resource,
        config,
    }
}
fn expand_embedding(
    task: &EmbeddingTask,
    target_index: Option<&IndexId>,
    pipelines: &PipelineConfigs,
    idle: Option<&IdleNotification>,
) -> Result<Vec<PlannedOperation>, ExpandError> {
    let pipeline = lookup_pipeline(pipelines, task.model.as_str())?;
    let model = task.model.clone();
    let (prefix, mode) = match task.mode {
        EmbeddingMode::Query => (&pipeline.query_prefix, "query"),
        EmbeddingMode::Passage => (&pipeline.passage_prefix, "passage"),
    };
    let mode = Some(mode);
    let encoder_env = encoder_environment(idle);
    let mut chain = Vec::with_capacity(5);
    chain
        .push(
            plan(
                OperationKind::Tokenization,
                ResourceKind::Tokenizer,
                ExecutionEnvironment::CpuOnly,
                &model,
                mode,
                OperationConfig {
                    tokenizer: Some(model.clone()),
                    prefix: Some(prefix.clone()),
                    max_seq_len: Some(pipeline.max_seq_len),
                    ..OperationConfig::default()
                },
            ),
        );
    chain
        .push(
            plan(
                OperationKind::Embedding,
                ResourceKind::Encoder,
                encoder_env,
                &model,
                mode,
                OperationConfig {
                    model: Some(model.clone()),
                    dtype: Some(pipeline.dtype),
                    target_dimension: Some(pipeline.dimension),
                    ..OperationConfig::default()
                },
            ),
        );
    if pipeline.pooling != PoolingMethod::None {
        chain
            .push(
                plan(
                    OperationKind::Pooling,
                    ResourceKind::Encoder,
                    encoder_env,
                    &model,
                    mode,
                    OperationConfig {
                        pooling: Some(pipeline.pooling),
                        ..OperationConfig::default()
                    },
                ),
            );
    }
    if pipeline.normalize {
        chain
            .push(
                plan(
                    OperationKind::L2Normalization,
                    ResourceKind::Encoder,
                    encoder_env,
                    &model,
                    mode,
                    OperationConfig {
                        model: Some(model.clone()),
                        ..OperationConfig::default()
                    },
                ),
            );
    }
    if let Some(index) = target_index {
        chain
            .push(
                plan(
                    OperationKind::IndexWrite,
                    ResourceKind::Index,
                    ExecutionEnvironment::Any,
                    &model,
                    mode,
                    OperationConfig {
                        index: Some(index.clone()),
                        ..OperationConfig::default()
                    },
                ),
            );
    }
    Ok(chain)
}
fn expand_similarity(
    spec: &SimilaritySpec,
    pipelines: &PipelineConfigs,
    capabilities: &CapabilitySet,
) -> Result<(&'static str, Vec<PlannedOperation>), ExpandError> {
    let pipeline = lookup_pipeline(pipelines, spec.model.as_str())?;
    if spec.params.top_k == 0 {
        return Err(ExpandError::Config(ConfigError::ZeroTopK));
    }
    if let ScorerChoice::CrossEncoder(name) = &spec.scorer {
        if !capabilities.contains(&Name::new(CROSS_ENCODER_AXIS), &Name::new(name)) {
            return Err(ExpandError::ScoringUnavailable {
                scorer: name.clone(),
            });
        }
    }
    let model = &spec.model;
    let index = IndexId::new(spec.params.index.0.to_string());
    let top_k = spec.params.top_k.min(u32::MAX as usize) as u32;
    let scope = spec.params.scope.clone();
    match &spec.scorer {
        ScorerChoice::Vector(scoring) => {
            let chain = vec![
                plan(OperationKind::Similarity, ResourceKind::Scorer,
                ExecutionEnvironment::Any, model, None, OperationConfig { model :
                Some(model.clone()), scorer : Some(format!("{scoring:?}")), index :
                Some(index.clone()), scope : Some(scope), target_dimension :
                Some(pipeline.dimension), dtype : Some(pipeline.dtype),
                ..OperationConfig::default() },), plan(OperationKind::TopK,
                ResourceKind::Scorer, ExecutionEnvironment::Any, model, None,
                OperationConfig { model : Some(model.clone()), index : Some(index), top_k
                : Some(top_k), ..OperationConfig::default() },),
            ];
            Ok(("similarity", chain))
        }
        ScorerChoice::CrossEncoder(name) => {
            let chain = vec![
                plan(OperationKind::Tokenization, ResourceKind::Tokenizer,
                ExecutionEnvironment::CpuOnly, model, None, OperationConfig { tokenizer :
                Some(model.clone()), max_seq_len : Some(pipeline.max_seq_len), scope :
                Some(scope.clone()), index : Some(index.clone()),
                ..OperationConfig::default() },), plan(OperationKind::Inference,
                ResourceKind::CrossEncoder, ExecutionEnvironment::Any, model, None,
                OperationConfig { model : Some(model.clone()), scorer : Some(name
                .clone()), dtype : Some(pipeline.dtype), ..OperationConfig::default()
                },), plan(OperationKind::Similarity, ResourceKind::CrossEncoder,
                ExecutionEnvironment::Any, model, None, OperationConfig { model :
                Some(model.clone()), scorer : Some(name.clone()), index : Some(index),
                scope : Some(scope), top_k : Some(top_k), ..OperationConfig::default()
                },),
            ];
            Ok(("similarity/cross-encoder", chain))
        }
    }
}
#[cfg(test)]
#[path = "test/expander.rs"]
mod tests;
