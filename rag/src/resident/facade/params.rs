use crate::resident::index::{EntryId, Scope};
use crate::resident::op::{IndexId, ModelId};

const DEFAULT_PRECISION: f32 = 1.0;
const DEFAULT_TOP_K: usize = 10;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
#[non_exhaustive]
pub enum TextRole {
    Query,
    #[default]
    Passage,
}

#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct TextInput {
    texts: Vec<String>,
}

impl TextInput {
    pub fn new(texts: impl IntoIterator<Item = impl Into<String>>) -> Self {
        Self {
            texts: texts.into_iter().map(Into::into).collect(),
        }
    }

    pub fn single(text: impl Into<String>) -> Self {
        Self {
            texts: vec![text.into()],
        }
    }

    pub fn texts(&self) -> &[String] {
        &self.texts
    }
}

#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct VectorInput {
    vectors: Vec<Vec<f32>>,
}

impl VectorInput {
    pub fn new(vectors: Vec<Vec<f32>>) -> Self {
        Self { vectors }
    }

    pub fn vectors(&self) -> &[Vec<f32>] {
        &self.vectors
    }
}

/// Cross-encoder input; kept apart from [`TextRole`] so role and pair scoring stay orthogonal.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct PairInput {
    query: String,
    chunk: String,
}

impl PairInput {
    pub fn new(query: impl Into<String>, chunk: impl Into<String>) -> Self {
        Self {
            query: query.into(),
            chunk: chunk.into(),
        }
    }

    pub fn query(&self) -> &str {
        &self.query
    }

    pub fn chunk(&self) -> &str {
        &self.chunk
    }
}

#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct EmbeddingParams {
    role: TextRole,
    write_index: Option<IndexId>,
    precision: f32,
}

impl EmbeddingParams {
    pub fn new(role: TextRole) -> Self {
        Self {
            role,
            write_index: None,
            precision: DEFAULT_PRECISION,
        }
    }

    pub fn write_index(mut self, index: IndexId) -> Self {
        self.write_index = Some(index);
        self
    }

    pub fn precision(mut self, precision: f32) -> Self {
        self.precision = precision;
        self
    }

    pub fn role(&self) -> TextRole {
        self.role
    }

    pub fn target_index(&self) -> Option<&IndexId> {
        self.write_index.as_ref()
    }

    pub fn get_precision(&self) -> f32 {
        self.precision
    }
}

impl Default for EmbeddingParams {
    fn default() -> Self {
        Self::new(TextRole::default())
    }
}

#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct SimilarityParams {
    index: IndexId,
    top_k: usize,
    scope: Scope,
    precision: f32,
}

impl SimilarityParams {
    pub fn new(index: IndexId) -> Self {
        Self {
            index,
            top_k: DEFAULT_TOP_K,
            scope: Scope::All,
            precision: DEFAULT_PRECISION,
        }
    }

    pub fn top_k(mut self, top_k: usize) -> Self {
        self.top_k = top_k;
        self
    }

    pub fn scope(mut self, scope: Scope) -> Self {
        self.scope = scope;
        self
    }

    pub fn precision(mut self, precision: f32) -> Self {
        self.precision = precision;
        self
    }

    pub fn index(&self) -> &IndexId {
        &self.index
    }

    pub fn get_top_k(&self) -> usize {
        self.top_k
    }

    pub fn get_scope(&self) -> &Scope {
        &self.scope
    }

    pub fn get_precision(&self) -> f32 {
        self.precision
    }
}

#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct InferenceParams {
    model: Option<ModelId>,
    precision: f32,
}

impl InferenceParams {
    pub fn new() -> Self {
        Self {
            model: None,
            precision: DEFAULT_PRECISION,
        }
    }

    pub fn model(mut self, model: ModelId) -> Self {
        self.model = Some(model);
        self
    }

    pub fn precision(mut self, precision: f32) -> Self {
        self.precision = precision;
        self
    }

    pub fn get_model(&self) -> Option<&ModelId> {
        self.model.as_ref()
    }

    pub fn get_precision(&self) -> f32 {
        self.precision
    }
}

impl Default for InferenceParams {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum Output {
    Embeddings(Vec<Vec<f32>>),
    Hits(Vec<(EntryId, f32)>),
    Scores(Vec<f32>),
}
