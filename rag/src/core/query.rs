//! Weakly typed, dynamic query object of the RAG engine.

use serde_json::{Map, Value};

/// Dynamic, weakly typed query object.
///
/// Layers decide for themselves which fields they react to: either by
/// presence of a field (`query.has("include")`) or by inspecting the whole
/// object (`query.inspect()`).
#[derive(Debug, Clone, Default)]
pub struct Query {
    fields: Map<String, Value>,
}

impl Query {
    pub fn new() -> Self {
        Self { fields: Map::new() }
    }

    pub fn from_fields(fields: Map<String, Value>) -> Self {
        Self { fields }
    }

    pub fn has(&self, field: &str) -> bool {
        matches!(self.fields.get(field), Some(v) if !v.is_null())
    }

    pub fn get(&self, field: &str) -> Option<&Value> {
        self.fields.get(field)
    }

    pub fn get_str(&self, field: &str) -> Option<&str> {
        self.fields.get(field).and_then(Value::as_str)
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
        Query { fields: merged }
    }

    pub fn set(&mut self, key: impl Into<String>, value: Value) {
        self.fields.insert(key.into(), value);
    }

    pub fn fields(&self) -> &Map<String, Value> {
        &self.fields
    }
}
