//! CLI entry point of the xy.ai.rag engine (on-demand, no daemon).
use std::path::Path;
use anyhow::{anyhow, Result};
use clap::Parser;
use serde_json::{json, Map, Value};
use xy_ai_rag::core::engine::{Engine, ExecutionMode};
use xy_ai_rag::core::layer::LayerStatus;
use xy_ai_rag::core::persistence::PersistenceManager;
use xy_ai_rag::core::query::Query;
use xy_ai_rag::core::registry::LayerRegistry;
use xy_ai_rag::layers::dir_cache::{configure_global_filter, PathFilter};
#[derive(Parser, Debug)]
#[command(name = "xyrag", about = "xy.ai.rag - Layered Anytime Retrieval Engine")]
struct Cli {
    /// Query fields as key=value
    query: Vec<String>,
    /// Query as a JSON object
    #[arg(long = "json")]
    json_query: Option<String>,
    /// RAG root directory; storage lives at <root>/.xyrag (default: CWD)
    #[arg(long)]
    root: Option<String>,
    /// Execution model of the layer topology
    #[arg(long, default_value = "parallel")]
    mode: String,
    /// Comma-separated glob patterns; only matching names/paths are
    /// returned by searches (wins over --exclude on conflicts)
    #[arg(long, value_delimiter = ',')]
    include: Vec<String>,
    /// Comma-separated glob patterns; matching names/paths are dropped
    /// from searches, unless also matched by --include
    #[arg(long, value_delimiter = ',')]
    exclude: Vec<String>,
    /// Overrides the per-layer result limit (default: 50 per layer)
    #[arg(long = "max-results")]
    max_results: Option<u64>,
}
fn parse_query(args: &[String], json_query: Option<&str>) -> Result<Query> {
    let mut fields = Map::new();
    if let Some(j) = json_query {
        if let Value::Object(m) = serde_json::from_str(j)? {
            fields.extend(m);
        }
    }
    for item in args {
        let (key, value) = item
            .split_once('=')
            .ok_or_else(|| {
                anyhow!("Invalid Query-Field (exspected key=value): {item}")
            })?;
        fields.insert(key.to_string(), Value::String(value.to_string()));
    }
    Ok(Query::from_fields(fields))
}
/// Builds the registry with all known layer implementations.
///
/// Concrete layers are added here later.
fn build_default_registry() -> LayerRegistry {
    let mut registry = LayerRegistry::new();
    registry
        .register(std::sync::Arc::new(xy_ai_rag::layers::glob_layer::GlobLayer::new()))
        .expect("failed to register GlobLayer");
    registry
        .register(std::sync::Arc::new(xy_ai_rag::layers::grep_layer::GrepLayer::new()))
        .expect("failed to register GrepLayer");
    registry
        .register(std::sync::Arc::new(xy_ai_rag::layers::trigram::TrigramLayer::new()))
        .expect("failed to register TrigramLayer");
    registry
        .register(std::sync::Arc::new(xy_ai_rag::layers::merge_layer::MergeLayer::new()))
        .expect("failed to register MergeLayer");
    registry
}
fn status_to_json(s: &LayerStatus) -> Value {
    let stage = match s.stage {
        xy_ai_rag::core::layer::LayerStage::Generate => "generate",
        xy_ai_rag::core::layer::LayerStage::Enrich => "enrich",
        xy_ai_rag::core::layer::LayerStage::Postprocess => "postprocess",
    };
    json!(
        { "layer_id" : s.layer_id, "stage" : stage, "ran" : s.ran, "skipped" : s.skipped,
        "aborted" : s.aborted, "contributions" : s.contributions, "detail" : s.detail, }
    )
}
/// Recursively rewrites `Lines` objects so their (numeric) string keys
/// become real YAML integers, yielding `1: text` instead of `'1': text`.
fn numeric_keys_for_lines(value: &mut serde_yaml::Value) {
    match value {
        serde_yaml::Value::Sequence(items) => {
            for item in items {
                numeric_keys_for_lines(item);
            }
        }
        serde_yaml::Value::Mapping(map) => {
            for (key, val) in map.iter_mut() {
                if key.as_str() == Some("Lines") {
                    if let serde_yaml::Value::Mapping(lines) = val {
                        let mut entries: Vec<
                            (i64, serde_yaml::Value, serde_yaml::Value),
                        > = lines
                            .iter()
                            .filter_map(|(k, v)| {
                                let n = k.as_str()?.parse::<i64>().ok()?;
                                Some((n, serde_yaml::Value::from(n), v.clone()))
                            })
                            .collect();
                        entries.sort_by_key(|(n, _, _)| *n);
                        *lines = entries.into_iter().map(|(_, k, v)| (k, v)).collect();
                    }
                } else {
                    numeric_keys_for_lines(val);
                }
            }
        }
        _ => {}
    }
}
#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let mode = ExecutionMode::parse(&cli.mode)
        .ok_or_else(|| anyhow!("Invalid mode: {}", cli.mode))?;
    let mut query = parse_query(&cli.query, cli.json_query.as_deref())?;
    if let Some(max_results) = cli.max_results {
        query.set("maxResults", json!(max_results));
    }
    let registry = build_default_registry();
    let persistence = PersistenceManager::new(cli.root.as_deref().map(Path::new))?;
    query.set_document_root(persistence.root.clone());
    let filter = PathFilter::new(&cli.include, &cli.exclude)
        .map_err(|e| anyhow!("Invalid --include/--exclude pattern: {}", e))?;
    configure_global_filter(persistence.root.clone(), filter);
    let mut engine = Engine::new(registry, persistence, mode);
    engine.start_background();
    let (result_set, statuses) = engine.run_query(query).await?;
    engine.shutdown(false).await;
    let results: Vec<Value> = result_set
        .entries()
        .iter()
        .map(|e| Value::Object(e.to_dict()))
        .collect();
    let layers: Vec<Value> = statuses.iter().map(status_to_json).collect();
    let output = json!({ "results" : results, "layers" : layers });
    let mut yaml_value = serde_yaml::to_value(&output)?;
    numeric_keys_for_lines(&mut yaml_value);
    println!("{}", serde_yaml::to_string(& yaml_value) ?.trim_end());
    Ok(())
}
