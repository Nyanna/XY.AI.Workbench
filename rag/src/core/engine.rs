//! Orchestrator (engine) of the RAG engine.
//!
//! Builds the list of active layers per query, determines their topology
//! and runs them either sequentially or in a fork/join model. Also manages
//! the global control flow for background activity of the layers.
use std::collections::HashMap;
use std::sync::Arc;
use anyhow::Result;
use futures::future::join_all;
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;
use crate::core::executor::CpuExecutor;
use crate::core::layer::{BackgroundContext, Layer, LayerContext, LayerStatus};
use crate::core::persistence::PersistenceManager;
use crate::core::query::Query;
use crate::core::registry::LayerRegistry;
use crate::core::result::ResultSet;
use crate::core::topology::build_levels;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExecutionMode {
    Sequential,
    Parallel,
}
impl ExecutionMode {
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "sequential" => Some(Self::Sequential),
            "parallel" => Some(Self::Parallel),
            _ => None,
        }
    }
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Sequential => "sequential",
            Self::Parallel => "parallel",
        }
    }
}
/// Connects registry, persistence and topology execution.
///
/// Used both as a library (`Engine::run_query`) and instantiated on-demand
/// by the CLI.
pub struct Engine {
    pub registry: LayerRegistry,
    pub persistence: Arc<PersistenceManager>,
    pub mode: ExecutionMode,
    /// Shared rayon pool handed to every layer for CPU-bound work.
    pub cpu: Arc<CpuExecutor>,
    cancel_tokens: HashMap<String, CancellationToken>,
    background_tasks: HashMap<String, JoinHandle<()>>,
}
impl Engine {
    pub fn new(
        registry: LayerRegistry,
        persistence: PersistenceManager,
        mode: ExecutionMode,
    ) -> Self {
        let cpu = Arc::new(
            CpuExecutor::new().expect("failed to initialize CPU executor"),
        );
        Self {
            registry,
            persistence: Arc::new(persistence),
            mode,
            cpu,
            cancel_tokens: HashMap::new(),
            background_tasks: HashMap::new(),
        }
    }
    pub fn active_layers(&self, query: &Query) -> Vec<Arc<dyn Layer>> {
        self.registry.iter().filter(|l| l.applies(query)).cloned().collect()
    }
    /// Runs one query against all activated layers.
    ///
    /// The first executed layer implicitly creates the `ResultSet`, by
    /// writing into the empty instance created here - even if it is itself
    /// only an enrichment layer.
    pub async fn run_query(
        &self,
        query: Query,
    ) -> Result<(ResultSet, Vec<LayerStatus>)> {
        let result_set = ResultSet::new();
        let layers = self.active_layers(&query);
        let levels = build_levels(&layers)?;
        let mut statuses: Vec<LayerStatus> = Vec::new();
        for level in levels {
            if self.mode == ExecutionMode::Parallel && level.len() > 1 {
                let futs = level
                    .iter()
                    .map(|layer| self.run_layer(layer.clone(), &query, &result_set));
                let results = join_all(futs).await;
                for r in results {
                    statuses.push(r?);
                }
            } else {
                for layer in level {
                    statuses.push(self.run_layer(layer, &query, &result_set).await?);
                }
            }
        }
        Ok((result_set, statuses))
    }
    async fn run_layer(
        &self,
        layer: Arc<dyn Layer>,
        query: &Query,
        result_set: &ResultSet,
    ) -> Result<LayerStatus> {
        let storage = self.persistence.layer_storage(layer.id())?;
        let ctx = LayerContext {
            query: query.clone(),
            storage,
            shared_index: self.persistence.shared_index.clone(),
            cpu: self.cpu.clone(),
        };
        Ok(layer.run(query, result_set, &ctx).await)
    }
    /// Starts the background activity of every layer (once, globally).
    ///
    /// Each layer decides for itself, based on its own implementation,
    /// whether it actually becomes active; the default is no activity.
    pub fn start_background(&mut self) {
        for layer in self.registry.all() {
            let id = layer.id().to_string();
            if self.background_tasks.contains_key(&id) {
                continue;
            }
            let cancel = CancellationToken::new();
            self.cancel_tokens.insert(id.clone(), cancel.clone());
            let storage = match self.persistence.layer_storage(&id) {
                Ok(s) => s,
                Err(_) => continue,
            };
            let shared_index = self.persistence.shared_index.clone();
            let cpu = self.cpu.clone();
            let layer_clone = layer.clone();
            let handle = tokio::spawn(async move {
                let ctx = BackgroundContext {
                    storage,
                    shared_index,
                    cpu,
                };
                layer_clone.background(&ctx, cancel).await;
            });
            self.background_tasks.insert(id, handle);
        }
    }
    /// Waits until all background activity has finished.
    ///
    /// A CLI process calls this after all queries have returned, before it
    /// terminates.
    pub async fn wait_background(&mut self) {
        let handles: Vec<_> = self.background_tasks.drain().map(|(_, h)| h).collect();
        for h in handles {
            let _ = h.await;
        }
    }
    pub fn cancel_background(&self) {
        for token in self.cancel_tokens.values() {
            token.cancel();
        }
    }
    pub async fn shutdown(&mut self, cancel: bool) {
        if cancel {
            self.cancel_background();
        }
        self.wait_background().await;
    }
}
