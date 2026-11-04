//! Rule set: pipeline data plus the explicit expander registry.
use std::collections::{BTreeMap, HashMap};
use serde::Deserialize;
use crate::resident::error::PlannerError;
use crate::resident::facade::TaskKind;
use crate::resident::op::ModelId;
use crate::resident::processor::StarvedSnapshot;
use super::config::PipelineConfig;
use super::expand::{
    EmbeddingExpander, ExpandContext, InferenceExpander, PlanExpander, PlannedOp,
    SimilarityExpander, TaskSpec,
};
use super::offers::EngineOffers;
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawRules {
    default_model: String,
    models: BTreeMap<String, PipelineConfig>,
}
pub struct RuleSet {
    default_model: ModelId,
    pipelines: HashMap<ModelId, PipelineConfig>,
    expanders: HashMap<(ModelId, TaskKind), Box<dyn PlanExpander>>,
}
impl RuleSet {
    /// Registers the standard expanders for every model and task kind.
    pub fn new(
        default_model: ModelId,
        pipelines: HashMap<ModelId, PipelineConfig>,
    ) -> Result<Self, PlannerError> {
        if !pipelines.contains_key(&default_model) {
            return Err(PlannerError::UnknownModel {
                model: default_model.0,
            });
        }
        for (model, pipeline) in &pipelines {
            pipeline.validate(&model.0)?;
        }
        let mut expanders: HashMap<(ModelId, TaskKind), Box<dyn PlanExpander>> = HashMap::new();
        for model in pipelines.keys() {
            expanders
                .insert(
                    (model.clone(), TaskKind::Embedding),
                    Box::new(EmbeddingExpander),
                );
            expanders
                .insert(
                    (model.clone(), TaskKind::Similarity),
                    Box::new(SimilarityExpander),
                );
            expanders
                .insert(
                    (model.clone(), TaskKind::Inference),
                    Box::new(InferenceExpander),
                );
        }
        Ok(Self {
            default_model,
            pipelines,
            expanders,
        })
    }
    pub fn from_toml(src: &str) -> Result<Self, PlannerError> {
        let raw: RawRules = toml::from_str(src)
            .map_err(|e| PlannerError::InvalidConfig {
                reason: e.to_string(),
            })?;
        let pipelines = raw
            .models
            .into_iter()
            .map(|(name, cfg)| (ModelId(name), cfg))
            .collect();
        Self::new(ModelId(raw.default_model), pipelines)
    }
    /// Replaces the expander for a configured model.
    pub fn register(
        &mut self,
        model: ModelId,
        kind: TaskKind,
        expander: Box<dyn PlanExpander>,
    ) -> Result<(), PlannerError> {
        if !self.pipelines.contains_key(&model) {
            return Err(PlannerError::UnknownModel {
                model: model.0,
            });
        }
        self.expanders.insert((model, kind), expander);
        Ok(())
    }
    pub fn default_model(&self) -> &ModelId {
        &self.default_model
    }
    pub fn pipeline(&self, model: &ModelId) -> Option<&PipelineConfig> {
        self.pipelines.get(model)
    }
    pub(super) fn expand(
        &self,
        task: &TaskSpec<'_>,
        starved: StarvedSnapshot,
        offers: &EngineOffers,
    ) -> Result<Vec<PlannedOp>, PlannerError> {
        let model = match task {
            TaskSpec::Inference(p) => p.get_model().unwrap_or(&self.default_model),
            _ => &self.default_model,
        };
        let unknown = || PlannerError::UnknownModel {
            model: model.0.clone(),
        };
        let pipeline = self.pipelines.get(model).ok_or_else(unknown)?;
        let expander = self
            .expanders
            .get(&(model.clone(), task.kind()))
            .ok_or_else(unknown)?;
        expander
            .expand(
                task,
                &ExpandContext {
                    starved,
                    offers,
                    model,
                    pipeline,
                },
            )
    }
}
