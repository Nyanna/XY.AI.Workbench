//! Pure decomposition of a task into an ordered chain of planned operations.
use std::collections::HashMap;
use std::fmt;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::a::resource_requirements::{
    ExecutionEnvironment, IndexId, ModelId, ResourceKind, ResourceRequirements,
};
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::b::embedding_task::{
    EmbeddingMode, EmbeddingTask,
};
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::b::similarity_task::{
    Scoring, SimilarityParams,
};
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::b::operation::{
    OperationDescriptor, OperationKind, OperationParams,
};
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::a::model_descriptor::{
    ModelDescriptor, Normalization, Pooling,
};
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::_1::capability_model::{
    CapabilitySet, Name,
};
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::_1::device_lane_event::DeviceKind;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::_1::layout::DType;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::_1::scopes_and_entry_sets::Scope;
const EXPECTED_TASK_KINDS2: &str = "Embedding/Similarity";
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
pub type ModelDescriptors = HashMap<String, ModelDescriptor>;
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
    pub pooling: Option<Pooling>,
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
pub enum ExpandError2 {
    ForeignTaskKind { expected: String, got: String },
    UnknownModel { model: String },
    Config(ConfigError),
    ScoringUnavailable { scorer: String },
}
impl fmt::Display for ExpandError2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExpandError2::ForeignTaskKind { expected, got } => {
                write!(f, "fremde Task-Art: erwartet {expected}, erhalten {got}")
            }
            ExpandError2::UnknownModel { model } => {
                write!(f, "unbekanntes Modell: {model}")
            }
            ExpandError2::Config(e) => write!(f, "Konfigurationsfehler: {e}"),
            ExpandError2::ScoringUnavailable { scorer } => {
                write!(f, "Scoring-Verfahren nicht verfügbar: {scorer}")
            }
        }
    }
}
impl std::error::Error for ExpandError2 {}
pub fn expand(
    task: &TaskSpec,
    models: &ModelDescriptors,
    capabilities: &CapabilitySet,
    idle: Option<&IdleNotification>,
) -> Result<Vec<PlannedOperation>, ExpandError2> {
    let (label, chain) = match task {
        TaskSpec::Embedding { task, target_index } => {
            ("embedding", expand_embedding(task, target_index.as_ref(), models, idle)?)
        }
        TaskSpec::Similarity(spec) => expand_similarity(spec, models, capabilities)?,
        TaskSpec::Other { kind } => {
            return Err(ExpandError2::ForeignTaskKind {
                expected: EXPECTED_TASK_KINDS2.to_string(),
                got: kind.clone(),
            });
        }
    };
    log::debug!("expand: task kind {label}, {} operations", chain.len());
    Ok(chain)
}
fn lookup_model<'a>(
    models: &'a ModelDescriptors,
    model: &str,
) -> Result<&'a ModelDescriptor, ExpandError2> {
    models
        .get(model)
        .ok_or_else(|| ExpandError2::UnknownModel {
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
fn plan2(
    kind: OperationKind,
    params: OperationParams,
    config: OperationConfig,
) -> PlannedOperation {
    PlannedOperation::new(OperationDescriptor::new(kind, params), config)
}
fn expand_embedding(
    task: &EmbeddingTask,
    target_index: Option<&IndexId>,
    models: &ModelDescriptors,
    idle: Option<&IdleNotification>,
) -> Result<Vec<PlannedOperation>, ExpandError2> {
    let descriptor = lookup_model(models, task.model.as_str())?;
    let model = &task.model;
    let mode = task.mode;
    let prefix = match mode {
        EmbeddingMode::Query => &descriptor.query_prefix,
        EmbeddingMode::Passage => &descriptor.passage_prefix,
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
            plan2(
                OperationKind::Tokenization,
                params(ResourceKind::Tokenizer, ExecutionEnvironment::CpuOnly, None),
                OperationConfig {
                    prefix: Some(prefix.clone()),
                    max_seq_len: Some(descriptor.max_seq_len),
                    ..OperationConfig::default()
                },
            ),
        );
    chain
        .push(
            plan2(
                OperationKind::Embedding,
                params(ResourceKind::Encoder, encoder_env, None)
                    .with_dimension(descriptor.dimension),
                OperationConfig {
                    dtype: Some(descriptor.storage_dtype),
                    ..OperationConfig::default()
                },
            ),
        );
    if descriptor.pooling != Pooling::PerToken {
        chain
            .push(
                plan2(
                    OperationKind::Pooling,
                    params(ResourceKind::Encoder, encoder_env, None),
                    OperationConfig {
                        pooling: Some(descriptor.pooling),
                        ..OperationConfig::default()
                    },
                ),
            );
    }
    if descriptor.normalization == Normalization::L2 {
        chain
            .push(
                plan2(
                    OperationKind::L2Normalization,
                    params(ResourceKind::Encoder, encoder_env, None),
                    OperationConfig::default(),
                ),
            );
    }
    if let Some(index) = target_index {
        chain
            .push(
                plan2(
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
    models: &ModelDescriptors,
    capabilities: &CapabilitySet,
) -> Result<(&'static str, Vec<PlannedOperation>), ExpandError2> {
    let descriptor = lookup_model(models, spec.model.as_str())?;
    if spec.params.top_k == 0 {
        return Err(ExpandError2::Config(ConfigError::ZeroTopK));
    }
    if let ScorerChoice::CrossEncoder(name) = &spec.scorer {
        if !capabilities.contains(&Name::new(CROSS_ENCODER_AXIS), &Name::new(name)) {
            return Err(ExpandError2::ScoringUnavailable {
                scorer: name.clone(),
            });
        }
    }
    let model = &spec.model;
    let index = spec.params.index.clone();
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
                plan2(OperationKind::Similarity, params(ResourceKind::Scorer,
                ExecutionEnvironment::Any, Some(& index)).with_dimension(descriptor
                .dimension), OperationConfig { scorer : Some(format!("{scoring:?}")),
                scope : Some(scope), dtype : Some(descriptor.storage_dtype),
                ..OperationConfig::default() },), plan2(OperationKind::TopK,
                params(ResourceKind::Scorer, ExecutionEnvironment::Any, Some(& index))
                .with_top_k(top_k), OperationConfig::default(),),
            ];
            Ok(("similarity", chain))
        }
        ScorerChoice::CrossEncoder(name) => {
            let chain = vec![
                plan2(OperationKind::Tokenization, params(ResourceKind::Tokenizer,
                ExecutionEnvironment::CpuOnly, Some(& index),), OperationConfig {
                max_seq_len : Some(descriptor.max_seq_len), scope : Some(scope.clone()),
                ..OperationConfig::default() },), plan2(OperationKind::Inference,
                params(ResourceKind::CrossEncoder, ExecutionEnvironment::Any, None),
                OperationConfig { scorer : Some(name.clone()), dtype : Some(descriptor
                .storage_dtype), ..OperationConfig::default() },),
                plan2(OperationKind::Similarity, params(ResourceKind::CrossEncoder,
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
