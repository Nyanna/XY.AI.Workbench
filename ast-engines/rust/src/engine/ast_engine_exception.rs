//! A user-facing error, path-free, raised by the engine; carries the HTTP status kind.
//! Mirrors `xy.ai.mcpc.ast.engine.AstEngineException` (Java).

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    NotFound,
    Conflict,
    Syntax,
}

#[derive(Debug, Clone)]
pub struct AstEngineException {
    kind: Kind,
    message: String,
    candidates: Vec<String>,
}

impl AstEngineException {
    pub fn new(kind: Kind, message: impl Into<String>) -> Self {
        Self { kind, message: message.into(), candidates: Vec::new() }
    }

    pub fn with_candidates(kind: Kind, message: impl Into<String>, candidates: Vec<String>) -> Self {
        Self { kind, message: message.into(), candidates }
    }

    pub fn kind(&self) -> Kind {
        self.kind
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn candidates(&self) -> &[String] {
        &self.candidates
    }
}

impl std::fmt::Display for AstEngineException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for AstEngineException {}

pub type AstResult<T> = Result<T, AstEngineException>;
