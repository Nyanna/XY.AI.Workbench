use std::fmt;
use std::path::{Path, PathBuf};

use super::super::l010::layout::DType;

pub const E5_SMALL_MODEL_ID: &str = "intfloat/multilingual-e5-small";
pub const E5_SMALL_REPO_DIR: &str =
    "/home/user/xyan/xy.ai.workbench/rag/model/multilingual-e5-small";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DeviceClass {
    Cpu,
    Gpu,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Pooling {
    Mean,
    Cls,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Normalization {
    L2,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ComputeDTypes {
    pub cpu: DType,
    pub gpu: DType,
}

impl ComputeDTypes {
    pub const fn for_device(&self, class: DeviceClass) -> DType {
        match class {
            DeviceClass::Cpu => self.cpu,
            DeviceClass::Gpu => self.gpu,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectionLayer {
    pub tensor_name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MultiVectorConfig {
    pub max_query_len: usize,
    pub max_document_len: usize,
    pub query_marker: String,
    pub document_marker: String,
    pub dimension: usize,
    pub skiplist: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModelDescriptor {
    pub model_id: String,
    pub architecture: String,
    pub dimension: usize,
    pub pooling: Pooling,
    pub normalization: Normalization,
    pub query_prefix: String,
    pub passage_prefix: String,
    pub storage_dtype: DType,
    pub compute_dtypes: ComputeDTypes,
    pub max_seq_len: usize,
    pub projection_layers: Vec<ProjectionLayer>,
    pub config_path: PathBuf,
    pub weight_paths: Vec<PathBuf>,
    pub multi_vector: Option<MultiVectorConfig>,
}

fn incompatible(reason: String) -> ModelConfigError {
    ModelConfigError::Incompatible { reason }
}

fn check_prefix(field: &str, value: &str) -> Result<(), ModelConfigError> {
    if value.is_empty() || value.ends_with(' ') {
        Ok(())
    } else {
        Err(incompatible(format!(
            "{field} muss mit einem Leerzeichen enden (war: {value:?})"
        )))
    }
}

impl ModelDescriptor {
    pub fn compute_dtype(&self, class: DeviceClass) -> DType {
        self.compute_dtypes.for_device(class)
    }

    pub fn validate(&self) -> Result<(), ModelConfigError> {
        if self.dimension == 0 {
            return Err(incompatible("dimension muss größer als 0 sein".to_string()));
        }
        if self.max_seq_len == 0 {
            return Err(incompatible("max_seq_len muss größer als 0 sein".to_string()));
        }
        check_prefix("query_prefix", &self.query_prefix)?;
        check_prefix("passage_prefix", &self.passage_prefix)?;
        Ok(())
    }

    pub fn e5_small_default(dir: &Path, storage_dtype: DType) -> ModelDescriptor {
        ModelDescriptor {
            model_id: E5_SMALL_MODEL_ID.to_string(),
            architecture: "bert".to_string(),
            dimension: 384,
            pooling: Pooling::Mean,
            normalization: Normalization::L2,
            query_prefix: "query: ".to_string(),
            passage_prefix: "passage: ".to_string(),
            storage_dtype,
            compute_dtypes: ComputeDTypes {
                cpu: DType::F32,
                gpu: DType::F32,
            },
            max_seq_len: 512,
            projection_layers: Vec::new(),
            config_path: dir.join("config.json"),
            weight_paths: vec![dir.join("model.safetensors")],
            multi_vector: None,
        }
    }

    pub fn e5_small_repo_default(storage_dtype: DType) -> ModelDescriptor {
        Self::e5_small_default(Path::new(E5_SMALL_REPO_DIR), storage_dtype)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModelConfigError {
    Incompatible { reason: String },
}

impl fmt::Display for ModelConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ModelConfigError::Incompatible { reason } => {
                write!(f, "inkompatible Modellkonfiguration: {reason}")
            }
        }
    }
}

impl std::error::Error for ModelConfigError {}

#[cfg(test)]
#[path = "test/model_descriptor.rs"]
mod tests;
