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
    descriptor: OperationDescriptor,
    config: OperationConfig,
}
impl PlannedOperation {
    pub fn new(descriptor: OperationDescriptor, config: OperationConfig) -> Self {
        Self { descriptor, config }
    }
    pub fn descriptor(&self) -> &OperationDescriptor {
        &self.descriptor
    }
    pub fn config(&self) -> &OperationConfig {
        &self.config
    }
    pub fn kind(&self) -> OperationKind {
        self.descriptor.kind()
    }
    pub fn resource(&self) -> &ResourceRequirements {
        self.descriptor.requirements()
    }
}
#[derive(Clone, Debug, Default, PartialEq)]
pub struct OperationConfig {
    pub scorer: Option<String>,
    pub pooling: Option<PoolingMethod>,
    pub prefix: Option<String>,
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
fn requirements(
    resource_kind: ResourceKind,
    environment: ExecutionEnvironment,
    model: &ModelId,
    index: Option<&IndexId>,
) -> ResourceRequirements {
    let resource = ResourceRequirements::new(resource_kind, environment)
        .with_model(model.clone());
    match index {
        Some(index) => resource.with_index(index.clone()),
        None => resource,
    }
}
fn plan(
    kind: OperationKind,
    params: OperationParams,
    config: OperationConfig,
) -> PlannedOperation {
    PlannedOperation::new(OperationDescriptor::new(kind, params), config)
}
fn expand_embedding(
    task: &EmbeddingTask,
    target_index: Option<&IndexId>,
    pipelines: &PipelineConfigs,
    idle: Option<&IdleNotification>,
) -> Result<Vec<PlannedOperation>, ExpandError> {
    let pipeline = lookup_pipeline(pipelines, task.model.as_str())?;
    let model = &task.model;
    let mode = task.mode;
    let prefix = match mode {
        EmbeddingMode::Query => &pipeline.query_prefix,
        EmbeddingMode::Passage => &pipeline.passage_prefix,
    };
    let encoder_env = encoder_environment(idle);
    let params = |
        resource_kind: ResourceKind,
        environment: ExecutionEnvironment,
        index: Option<&IndexId>|
    {
        OperationParams::new(requirements(resource_kind, environment, model, index))
            .with_mode(mode)
    };
    let mut chain = Vec::with_capacity(5);
    chain
        .push(
            plan(
                OperationKind::Tokenization,
                params(ResourceKind::Tokenizer, ExecutionEnvironment::CpuOnly, None),
                OperationConfig {
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
                params(ResourceKind::Encoder, encoder_env, None)
                    .with_dimension(pipeline.dimension),
                OperationConfig {
                    dtype: Some(pipeline.dtype),
                    ..OperationConfig::default()
                },
            ),
        );
    if pipeline.pooling != PoolingMethod::None {
        chain
            .push(
                plan(
                    OperationKind::Pooling,
                    params(ResourceKind::Encoder, encoder_env, None),
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
                    params(ResourceKind::Encoder, encoder_env, None),
                    OperationConfig::default(),
                ),
            );
    }
    if let Some(index) = target_index {
        chain
            .push(
                plan(
                    OperationKind::IndexWrite,
                    params(ResourceKind::Index, ExecutionEnvironment::Any, Some(index)),
                    OperationConfig::default(),
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
    let top_k = spec.params.top_k.min(u32::MAX as usize);
    let scope = spec.params.scope.clone();
    let params = |
        resource_kind: ResourceKind,
        environment: ExecutionEnvironment,
        index: Option<&IndexId>|
    { OperationParams::new(requirements(resource_kind, environment, model, index)) };
    match &spec.scorer {
        ScorerChoice::Vector(scoring) => {
            let chain = vec![
                plan(OperationKind::Similarity, params(ResourceKind::Scorer,
                ExecutionEnvironment::Any, Some(& index)).with_dimension(pipeline
                .dimension), OperationConfig { scorer : Some(format!("{scoring:?}")),
                scope : Some(scope), dtype : Some(pipeline.dtype),
                ..OperationConfig::default() },), plan(OperationKind::TopK,
                params(ResourceKind::Scorer, ExecutionEnvironment::Any, Some(& index))
                .with_top_k(top_k), OperationConfig::default(),),
            ];
            Ok(("similarity", chain))
        }
        ScorerChoice::CrossEncoder(name) => {
            let chain = vec![
                plan(OperationKind::Tokenization, params(ResourceKind::Tokenizer,
                ExecutionEnvironment::CpuOnly, Some(& index),), OperationConfig {
                max_seq_len : Some(pipeline.max_seq_len), scope : Some(scope.clone()),
                ..OperationConfig::default() },), plan(OperationKind::Inference,
                params(ResourceKind::CrossEncoder, ExecutionEnvironment::Any, None),
                OperationConfig { scorer : Some(name.clone()), dtype : Some(pipeline
                .dtype), ..OperationConfig::default() },),
                plan(OperationKind::Similarity, params(ResourceKind::CrossEncoder,
                ExecutionEnvironment::Any, Some(& index)).with_top_k(top_k),
                OperationConfig { scorer : Some(name.clone()), scope : Some(scope),
                ..OperationConfig::default() },),
            ];
            Ok(("similarity/cross-encoder", chain))
        }
    }
}
#[cfg(test)]
#[path = "test/expander.rs"]
mod tests;
