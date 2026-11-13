use std::fmt;
use std::sync::Arc;

use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::a::result_kinds::TaskResult;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::a::index_contract::EntryId;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::l010::extensions::Extensions;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EmbeddingMode {
    Query,
    Passage,
}

#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Precision {
    F32,
    F16,
    Bf16,
    Int8,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ModelId(pub String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EmbeddingInput {
    One(String),
    Many(Vec<String>),
}

impl EmbeddingInput {
    pub fn texts(&self) -> &[String] {
        match self {
            EmbeddingInput::One(s) => std::slice::from_ref(s),
            EmbeddingInput::Many(v) => v.as_slice(),
        }
    }

    pub fn len(&self) -> usize {
        self.texts().len()
    }

    pub fn is_empty(&self) -> bool {
        self.texts().is_empty()
    }
}

impl From<String> for EmbeddingInput {
    fn from(s: String) -> Self {
        EmbeddingInput::One(s)
    }
}

impl From<&str> for EmbeddingInput {
    fn from(s: &str) -> Self {
        EmbeddingInput::One(s.to_string())
    }
}

impl From<Vec<String>> for EmbeddingInput {
    fn from(v: Vec<String>) -> Self {
        EmbeddingInput::Many(v)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EntryMetadata {
    pub text: String,
    pub model: ModelId,
    pub mode: EmbeddingMode,
    pub precision: Precision,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BackendError(pub String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TargetError(pub String);

pub trait EmbeddingTarget: Send + Sync {
    fn write(
        &self,
        vectors: Vec<Vec<f32>>,
        metadata: Vec<EntryMetadata>,
        extensions: &Extensions,
    ) -> Result<Vec<EntryId>, TargetError>;
}

pub trait EmbeddingBackend: Send + Sync {
    fn embed(
        &self,
        model: &ModelId,
        mode: EmbeddingMode,
        precision: Precision,
        texts: &[String],
        extensions: &Extensions,
    ) -> Result<Vec<Vec<f32>>, BackendError>;
}

#[derive(Clone)]
pub struct EmbeddingTask {
    pub input: EmbeddingInput,
    pub model: ModelId,
    pub mode: EmbeddingMode,
    pub precision: Precision,
    pub target: Option<Arc<dyn EmbeddingTarget>>,
    pub extensions: Extensions,
}

impl EmbeddingTask {
    pub fn new(
        input: impl Into<EmbeddingInput>,
        model: ModelId,
        mode: EmbeddingMode,
        precision: Precision,
    ) -> Self {
        Self {
            input: input.into(),
            model,
            mode,
            precision,
            target: None,
            extensions: Extensions::new(),
        }
    }

    pub fn with_target(mut self, target: Arc<dyn EmbeddingTarget>) -> Self {
        self.target = Some(target);
        self
    }

    pub fn with_extensions(mut self, extensions: Extensions) -> Self {
        self.extensions = extensions;
        self
    }

    pub fn run(&self, backend: &dyn EmbeddingBackend) -> Result<TaskResult, EmbeddingError> {
        let texts = self.input.texts();
        if texts.is_empty() {
            return Ok(match self.target {
                Some(_) => TaskResult::EntryRefs(Vec::new()),
                None => TaskResult::Embeddings(Vec::new()),
            });
        }

        let vectors = backend
            .embed(&self.model, self.mode, self.precision, texts, &self.extensions)
            .map_err(EmbeddingError::Backend)?;
        if vectors.len() != texts.len() {
            return Err(EmbeddingError::CountMismatch {
                expected: texts.len(),
                got: vectors.len(),
            });
        }

        let Some(target) = &self.target else {
            return Ok(TaskResult::Embeddings(vectors));
        };

        let expected = vectors.len();
        let metadata = texts
            .iter()
            .map(|t| EntryMetadata {
                text: t.clone(),
                model: self.model.clone(),
                mode: self.mode,
                precision: self.precision,
            })
            .collect();
        let ids = target
            .write(vectors, metadata, &self.extensions)
            .map_err(EmbeddingError::Target)?;
        if ids.len() != expected {
            return Err(EmbeddingError::CountMismatch {
                expected,
                got: ids.len(),
            });
        }
        Ok(TaskResult::EntryRefs(ids))
    }
}

impl fmt::Debug for EmbeddingTask {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("EmbeddingTask")
            .field("input", &self.input)
            .field("model", &self.model)
            .field("mode", &self.mode)
            .field("precision", &self.precision)
            .field("target", &self.target.is_some())
            .field("extensions", &self.extensions)
            .finish()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EmbeddingError {
    Backend(BackendError),
    Target(TargetError),
    CountMismatch { expected: usize, got: usize },
}

impl fmt::Display for EmbeddingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EmbeddingError::Backend(e) => write!(f, "embedding backend failed: {}", e.0),
            EmbeddingError::Target(e) => write!(f, "embedding target failed: {}", e.0),
            EmbeddingError::CountMismatch { expected, got } => {
                write!(f, "count mismatch: expected {expected}, got {got}")
            }
        }
    }
}

impl std::error::Error for EmbeddingError {}

const _: fn() = || {
    fn assert_traits<T: Send + Sync + 'static>() {}
    assert_traits::<EmbeddingMode>();
    assert_traits::<Precision>();
    assert_traits::<ModelId>();
    assert_traits::<EmbeddingInput>();
    assert_traits::<EntryMetadata>();
    assert_traits::<BackendError>();
    assert_traits::<TargetError>();
    assert_traits::<EmbeddingTask>();
    assert_traits::<EmbeddingError>();
};

#[cfg(test)]
#[path = "test/embedding_task.rs"]
mod tests;
