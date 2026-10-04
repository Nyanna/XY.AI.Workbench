//! Postprocessing layer that folds duplicate result entries together.
//!
//! Runs once the result set is assumed complete and merges every
//! `ResultEntry` that shares the same `File` field into a single entry.
//! Fields are merged recursively (nested objects are merged key by key
//! rather than one replacing the other wholesale). The `Lines` field is
//! handled specially: for a line number present on both sides, the
//! shorter text wins, giving excerpted lines precedence over full lines.
//! Entries without a `File` field are left untouched.
use std::collections::BTreeMap;
use async_trait::async_trait;
use serde_json::{json, Map, Value};
use crate::core::layer::{Layer, LayerContext, LayerStage, LayerStatus};
use crate::core::query::Query;
use crate::core::result::ResultSet;
/// Merges the two `Lines` objects of a duplicate pair.
///
/// Both sides are expected in the `{ "line_no": text, ... }` shape produced
/// by content-search layers. For a line number present on both sides, the
/// shorter text is kept (excerpts take precedence over full lines); the
/// result is sorted by line number.
fn merge_lines(existing: Value, incoming: Value) -> Value {
    let mut by_line: BTreeMap<i64, String> = BTreeMap::new();
    for side in [existing, incoming] {
        let Value::Object(entries) = side else { continue };
        for (key, text) in entries {
            let (Ok(line_no), Some(text)) = (key.parse::<i64>(), text.as_str()) else {
                continue;
            };
            by_line
                .entry(line_no)
                .and_modify(|cur| {
                    if text.len() < cur.len() {
                        *cur = text.to_string();
                    }
                })
                .or_insert_with(|| text.to_string());
        }
    }
    Value::Object(
        by_line.into_iter().map(|(n, t)| (n.to_string(), Value::String(t))).collect(),
    )
}
/// Recursively merges `incoming` into `existing` for one field.
///
/// Objects are merged key by key (recursing further); any other pair of
/// values has `incoming` win, except for the `Lines` field, which gets its
/// own line-number-aware merge.
fn merge_value(key: &str, existing: Value, incoming: Value) -> Value {
    if key == "Lines" {
        return merge_lines(existing, incoming);
    }
    match (existing, incoming) {
        (Value::Object(mut a), Value::Object(b)) => {
            for (k, v) in b {
                let merged = match a.remove(&k) {
                    Some(ev) => merge_value(&k, ev, v),
                    None => v,
                };
                a.insert(k, merged);
            }
            Value::Object(a)
        }
        (_, incoming) => incoming,
    }
}
/// Merges `incoming` field by field into `target`.
fn merge_fields(target: &mut Map<String, Value>, incoming: Map<String, Value>) {
    for (k, v) in incoming {
        let merged = match target.remove(&k) {
            Some(existing) => merge_value(&k, existing, v),
            None => v,
        };
        target.insert(k, merged);
    }
}
/// Postprocessing layer merging duplicate `File` entries into one.
///
/// See the module documentation for the merge semantics.
#[derive(Default)]
pub struct MergeLayer;
impl MergeLayer {
    pub fn new() -> Self {
        Self
    }
}
#[async_trait]
impl Layer for MergeLayer {
    fn id(&self) -> &str {
        "merge"
    }
    fn stage(&self) -> LayerStage {
        LayerStage::Postprocess
    }
    /// Always applies: it operates on whatever is already in the result
    /// set, independent of the query's own fields.
    fn applies(&self, _query: &Query) -> bool {
        true
    }
    async fn run(
        &self,
        _query: &Query,
        result_set: &ResultSet,
        _ctx: &LayerContext,
    ) -> LayerStatus {
        let mut status = LayerStatus::new(self.id(), self.stage());
        let entries = result_set.entries();
        let mut order: Vec<String> = Vec::new();
        let mut merged: std::collections::HashMap<String, Map<String, Value>> = std::collections::HashMap::new();
        let mut primary_id: std::collections::HashMap<String, String> = std::collections::HashMap::new();
        let mut duplicate_ids: Vec<String> = Vec::new();
        for entry in &entries {
            let Some(Value::String(file)) = entry.get("File") else {
                continue;
            };
            let mut fields = entry.to_dict();
            fields.remove("id");
            match merged.get_mut(&file) {
                Some(acc) => {
                    merge_fields(acc, fields);
                    duplicate_ids.push(entry.id.clone());
                }
                None => {
                    primary_id.insert(file.clone(), entry.id.clone());
                    merged.insert(file.clone(), fields);
                    order.push(file);
                }
            }
        }
        for file in order {
            let Some(id) = primary_id.remove(&file) else { continue };
            let Some(fields) = merged.remove(&file) else { continue };
            if let Some(primary) = result_set.get(&id) {
                primary.merge(fields);
            }
        }
        for id in &duplicate_ids {
            result_set.remove(id);
        }
        status.contributions = duplicate_ids.len();
        status.ran = true;
        status.detail.insert("merged_matches".into(), json!(duplicate_ids.len()));
        status
    }
}
