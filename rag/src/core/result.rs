//! Shared, weakly typed result object of the RAG engine.
//!
//! A `ResultSet` consists of `ResultEntry` objects. Fields are free form -
//! a layer may store a file name, an AST node id, line numbers, or just a
//! summary. Aggregation across layers happens implicitly: layers read and
//! write on shared field names (e.g. "path", "line", "span").
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use serde_json::{Map, Value};
static ID_COUNTER: AtomicU64 = AtomicU64::new(1);
fn next_entry_id() -> String {
    format!("e{}", ID_COUNTER.fetch_add(1, Ordering::SeqCst))
}
#[derive(Debug, Default)]
struct EntryInner {
    fields: Map<String, Value>,
}
/// A single, weakly typed entry in the result set.
#[derive(Debug)]
pub struct ResultEntry {
    pub id: String,
    inner: Mutex<EntryInner>,
}
impl ResultEntry {
    pub fn new(entry_id: Option<String>, fields: Map<String, Value>) -> Arc<Self> {
        Arc::new(Self {
            id: entry_id.unwrap_or_else(next_entry_id),
            inner: Mutex::new(EntryInner { fields }),
        })
    }
    /// Extends/overwrites fields (thread-safe).
    ///
    /// An enrichment layer calls this on an already existing entry, e.g. to
    /// replace "line" with a text excerpt or to add an AST outline/FQN.
    pub fn merge(&self, fields: Map<String, Value>) {
        let mut guard = self.inner.lock().unwrap();
        for (k, v) in fields {
            guard.fields.insert(k, v);
        }
    }
    pub fn has(&self, field: &str) -> bool {
        let guard = self.inner.lock().unwrap();
        matches!(guard.fields.get(field), Some(v) if ! v.is_null())
    }
    pub fn get(&self, field: &str) -> Option<Value> {
        let guard = self.inner.lock().unwrap();
        guard.fields.get(field).cloned()
    }
    pub fn to_dict(&self) -> Map<String, Value> {
        let guard = self.inner.lock().unwrap();
        let mut out = Map::new();
        out.insert("id".into(), Value::String(self.id.clone()));
        for (k, v) in guard.fields.iter() {
            out.insert(k.clone(), v.clone());
        }
        out
    }
}
/// Thread-safe, shared collection of `ResultEntry` objects.
///
/// The first layer in the topology - even if it is a pure enrichment layer
/// - implicitly creates the result set. Subsequent layers either create new
///   entries (generate) or enrich existing ones, by operating on shared
///   fields.
pub struct ResultSet {
    entries: Mutex<HashMap<String, Arc<ResultEntry>>>,
    order: Mutex<Vec<String>>,
}
impl ResultSet {
    pub fn new() -> Self {
        Self {
            entries: Mutex::new(HashMap::new()),
            order: Mutex::new(Vec::new()),
        }
    }
    pub fn add(&self, entry: Arc<ResultEntry>) -> Arc<ResultEntry> {
        let mut entries = self.entries.lock().unwrap();
        let mut order = self.order.lock().unwrap();
        if !entries.contains_key(&entry.id) {
            order.push(entry.id.clone());
        }
        entries.insert(entry.id.clone(), entry.clone());
        entry
    }
    pub fn get(&self, entry_id: &str) -> Option<Arc<ResultEntry>> {
        self.entries.lock().unwrap().get(entry_id).cloned()
    }
    pub fn entries(&self) -> Vec<Arc<ResultEntry>> {
        let order = self.order.lock().unwrap();
        let entries = self.entries.lock().unwrap();
        order.iter().map(|id| entries[id].clone()).collect()
    }
    /// Finds entries whose fields match all given criteria.
    ///
    /// The implicit aggregation of layers is based on this mechanism: a
    /// layer looks up e.g. all entries with a given "path" and enriches
    /// them with further fields.
    pub fn find(&self, criteria: &[(&str, Value)]) -> Vec<Arc<ResultEntry>> {
        self.entries()
            .into_iter()
            .filter(|e| criteria.iter().all(|(k, v)| e.get(k).as_ref() == Some(v)))
            .collect()
    }
    /// Stable sort (no numeric ranking) for post-processing layers.
    pub fn sort_by_key<K: Ord>(&self, mut key: impl FnMut(&ResultEntry) -> K) {
        let entries = self.entries.lock().unwrap();
        let mut order = self.order.lock().unwrap();
        order.sort_by_key(|id| key(&entries[id]));
    }
    pub fn len(&self) -> usize {
        self.order.lock().unwrap().len()
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
impl Default for ResultSet {
    fn default() -> Self {
        Self::new()
    }
}
