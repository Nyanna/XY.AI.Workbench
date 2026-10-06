//! Weakly typed, dynamic query object of the RAG engine.
use std::path::{Path, PathBuf};
use serde_json::{Map, Value};
fn default_document_root() -> PathBuf {
    std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
}
/// Dynamic, weakly typed query object.
///
/// Layers decide for themselves which fields they react to: either by
/// presence of a field (`query.has("include")`) or by inspecting the whole
/// object (`query.inspect()`).
#[derive(Debug, Clone)]
pub struct Query {
    fields: Map<String, Value>,
    /// Fallback for the `directory` field; see [`Query::directory`].
    document_root: PathBuf,
}
impl Default for Query {
    fn default() -> Self {
        Self::new()
    }
}
impl Query {
    pub fn new() -> Self {
        Self {
            fields: Map::new(),
            document_root: default_document_root(),
        }
    }
    pub fn from_fields(fields: Map<String, Value>) -> Self {
        Self {
            fields,
            document_root: default_document_root(),
        }
    }
    /// Overrides the document root (fallback for `directory`), e.g. with
    /// the RAG root resolved by the engine's `PersistenceManager`.
    pub fn set_document_root(&mut self, root: impl Into<PathBuf>) {
        self.document_root = root.into();
    }
    pub fn document_root(&self) -> &Path {
        &self.document_root
    }
    /// Resolves the directory a layer should operate on: the `directory`
    /// field if present and non-null, otherwise the document root.
    pub fn directory(&self) -> PathBuf {
        self.resolve_search_root().0
    }
    /// Resolves the actual filesystem directory a layer should search,
    /// plus the relative prefix (document-root-relative, `/`-separated)
    /// every path discovered below it must be prepended with, so results
    /// always carry the full path relative to the document root - not
    /// just relative to the (optional) `directory` field.
    pub fn resolve_search_root(&self) -> (PathBuf, String) {
        let document_root = self.document_root().to_path_buf();
        match self.get_str("directory") {
            None => (document_root, String::new()),
            Some(dir) => {
                let dir_path = PathBuf::from(dir);
                let search_root = if dir_path.is_absolute() {
                    dir_path.clone()
                } else {
                    document_root.join(&dir_path)
                };
                let prefix = match search_root.strip_prefix(&document_root) {
                    Ok(rel) => {
                        rel.to_string_lossy().replace(std::path::MAIN_SEPARATOR, "/")
                    }
                    Err(_) => dir.trim_matches('/').to_string(),
                };
                (search_root, prefix)
            }
        }
    }
    pub fn has(&self, field: &str) -> bool {
        matches!(self.fields.get(field), Some(v) if ! v.is_null())
    }
    pub fn get(&self, field: &str) -> Option<&Value> {
        self.fields.get(field)
    }
    pub fn get_str(&self, field: &str) -> Option<&str> {
        self.fields.get(field).and_then(Value::as_str)
    }
    /// Resolves a numeric field as `usize`, e.g. `maxResults`, accepting
    /// both JSON numbers and numeric strings (as passed via the CLI).
    pub fn get_usize(&self, field: &str) -> Option<usize> {
        match self.fields.get(field)? {
            Value::Number(n) => n.as_u64().map(|v| v as usize),
            Value::String(s) => s.parse::<usize>().ok(),
            _ => None,
        }
    }
    /// Full copy of the fields for free analysis by layers.
    pub fn inspect(&self) -> Map<String, Value> {
        self.fields.clone()
    }
    pub fn with_fields(&self, overrides: Map<String, Value>) -> Query {
        let mut merged = self.fields.clone();
        for (k, v) in overrides {
            merged.insert(k, v);
        }
        Query {
            fields: merged,
            document_root: self.document_root.clone(),
        }
    }
    pub fn set(&mut self, key: impl Into<String>, value: Value) {
        self.fields.insert(key.into(), value);
    }
    pub fn fields(&self) -> &Map<String, Value> {
        &self.fields
    }
}
