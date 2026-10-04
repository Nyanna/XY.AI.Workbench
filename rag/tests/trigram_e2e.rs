//! End-to-end tests for the trigram layer (spec part A, "Tests").

use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use serde_json::{json, Value};

use xy_ai_rag::core::engine::{Engine, ExecutionMode};
use xy_ai_rag::core::persistence::PersistenceManager;
use xy_ai_rag::core::query::Query;
use xy_ai_rag::core::registry::LayerRegistry;
use xy_ai_rag::layers::trigram::TrigramLayer;

struct TmpDirs {
    content: PathBuf,
    rag_root: PathBuf,
}

impl TmpDirs {
    fn new(tag: &str) -> Self {
        let nanos = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
        let base = std::env::temp_dir().join(format!("trigram_e2e_{tag}_{nanos}"));
        let content = base.join("content");
        let rag_root = base.join("rag");
        fs::create_dir_all(&content).unwrap();
        fs::create_dir_all(&rag_root).unwrap();
        Self { content, rag_root }
    }
}

impl Drop for TmpDirs {
    fn drop(&mut self) {
        if let Some(parent) = self.content.parent() {
            let _ = fs::remove_dir_all(parent);
        }
    }
}

fn engine_for(rag_root: &std::path::Path) -> Engine {
    let mut registry = LayerRegistry::new();
    registry.register(std::sync::Arc::new(TrigramLayer::new())).unwrap();
    let persistence = PersistenceManager::new(Some(rag_root)).unwrap();
    Engine::new(registry, persistence, ExecutionMode::Sequential)
}

fn query_for(content: &std::path::Path, text: &str) -> Query {
    let mut q = Query::new();
    q.set_document_root(content.to_path_buf());
    q.set("query", json!(text));
    q
}

/// Lines of the result entry for `file`, as `(line_no, text)` pairs.
fn lines_of(result: &xy_ai_rag::core::result::ResultSet, file: &str) -> Vec<(u64, String)> {
    for entry in result.entries() {
        let dict = entry.to_dict();
        if dict.get("File").and_then(Value::as_str) == Some(file) {
            let arr = dict.get("Lines").and_then(Value::as_array).unwrap();
            return arr
                .iter()
                .map(|pair| {
                    let p = pair.as_array().unwrap();
                    (p[0].as_u64().unwrap(), p[1].as_str().unwrap().to_string())
                })
                .collect();
        }
    }
    Vec::new()
}

#[tokio::test]
async fn multiple_hits_per_line_collapse_and_lines_are_sorted() {
    let dirs = TmpDirs::new("hits");
    fs::write(
        dirs.content.join("a.txt"),
        "alpha userName beta userName\nno match here\nuser name again\n",
    )
    .unwrap();

    let engine = engine_for(&dirs.rag_root);
    let (result, statuses) = engine.run_query(query_for(&dirs.content, "userName")).await.unwrap();

    assert!(statuses.iter().any(|s| s.layer_id == "trigram" && s.ran));
    let lines = lines_of(&result, "a.txt");
    // Two matching lines (1 and 3); the double hit on line 1 collapses to one.
    assert_eq!(lines.len(), 2);
    assert_eq!(lines[0].0, 1);
    assert_eq!(lines[1].0, 3);
    // Original (not normalised) text is returned.
    assert_eq!(lines[0].1, "alpha userName beta userName");
}

#[tokio::test]
async fn reindexes_on_mtime_change() {
    let dirs = TmpDirs::new("mtime");
    let path = dirs.content.join("b.txt");
    fs::write(&path, "something unrelated here\n").unwrap();

    let engine = engine_for(&dirs.rag_root);
    let (r1, _) = engine.run_query(query_for(&dirs.content, "treasure")).await.unwrap();
    assert!(lines_of(&r1, "b.txt").is_empty());

    // Rewrite with the search term; mtime changes -> re-index -> match.
    std::thread::sleep(std::time::Duration::from_millis(10));
    fs::write(&path, "the hidden treasure map\n").unwrap();
    let (r2, _) = engine.run_query(query_for(&dirs.content, "treasure")).await.unwrap();
    assert_eq!(lines_of(&r2, "b.txt").len(), 1);
}

#[tokio::test]
async fn large_files_are_skipped() {
    let dirs = TmpDirs::new("large");
    let mut big = String::from("treasure\n");
    big.push_str(&"x".repeat(6 * 1024 * 1024));
    fs::write(dirs.content.join("big.txt"), big).unwrap();

    let engine = engine_for(&dirs.rag_root);
    let (result, _) = engine.run_query(query_for(&dirs.content, "treasure")).await.unwrap();
    assert!(lines_of(&result, "big.txt").is_empty());
}

#[tokio::test]
async fn deleted_file_entry_is_dropped() {
    let dirs = TmpDirs::new("deleted");
    let path = dirs.content.join("c.txt");
    fs::write(&path, "the hidden treasure map\n").unwrap();

    let engine = engine_for(&dirs.rag_root);
    let (r1, _) = engine.run_query(query_for(&dirs.content, "treasure")).await.unwrap();
    assert_eq!(lines_of(&r1, "c.txt").len(), 1);

    fs::remove_file(&path).unwrap();
    let (r2, _) = engine.run_query(query_for(&dirs.content, "treasure")).await.unwrap();
    assert!(lines_of(&r2, "c.txt").is_empty());
}
