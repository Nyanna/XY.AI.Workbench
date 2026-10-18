//! Per-model pipeline data; logic stays in the expanders.

use serde::Deserialize;

use crate::resident::error::PlannerError;
use crate::resident::facade::TextRole;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Pooling {
    #[default]
    Mean,
    Cls,
    /// Token vectors are kept as they are.
    None,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PipelineDtype {
    F16,
    #[default]
    F32,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ScorerKind {
    #[default]
    Cosine,
    MaxSim,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct Prefixes {
    pub query: String,
    pub passage: String,
}

impl Prefixes {
    pub fn for_role(&self, role: TextRole) -> &str {
        match role {
            TextRole::Query => &self.query,
            TextRole::Passage => &self.passage,
        }
    }
}

#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct PipelineConfig {
    pub prefixes: Prefixes,
    pub pooling: Pooling,
    pub normalize: bool,
    pub max_seq_len: u32,
    pub dim: Option<u32>,
    pub dtype: PipelineDtype,
    pub scorer: ScorerKind,
}

impl Default for PipelineConfig {
    fn default() -> Self {
        Self {
            prefixes: Prefixes::default(),
            pooling: Pooling::default(),
            normalize: true,
            max_seq_len: 512,
            dim: None,
            dtype: PipelineDtype::default(),
            scorer: ScorerKind::default(),
        }
    }
}

impl PipelineConfig {
    pub(super) fn validate(&self, model: &str) -> Result<(), PlannerError> {
        let invalid = |reason: &str| PlannerError::InvalidConfig {
            reason: format!("model '{model}': {reason}"),
        };
        if self.max_seq_len == 0 {
            return Err(invalid("max_seq_len must be positive"));
        }
        if self.dim == Some(0) {
            return Err(invalid("dim must be positive"));
        }
        Ok(())
    }
}
