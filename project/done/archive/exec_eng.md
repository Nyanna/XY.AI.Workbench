Implementiere eine Executor Service basierend auf "rayon". Dieser soll den Layer zur Verfügung stehen und erlaubt maximale CPU Nutzung.

`/home/user/xyan/xy.ai.workbench/rag/src/core/layer.rs` "LayherContext"
SystemInit:  id=30efbf5c-4905-4f48-9959-102efdb2ab4c, cwd=/home/user/xyan/xy.ai.workbench/project, model=claude-sonnet-5
Control Request:
```yaml
id: 8b5f-14
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/rag/src/core/layer.rs
  reason: Inspect LayerContext structure to add rayon-based executor service
```
/answer 8b5f-14 allow
Control Request:
```yaml
id: 8b5f-15
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/core/layer.rs
      nodes:
      - id: 23lmSF|mEOgEf
        signature: use std::collections::HashSet;
      - id: LayerStage
        signature: /// Coarse topology hint for a layer; does not replace explicit `depends_on`. /…
        docstring: Coarse topology hint for a layer; does not replace explicit `depends_on`.
      - id: LayerStatus
        signature: /// Outcome protocol of one layer invocation for one query. /// /// Returned by…
        docstring: Outcome protocol of one layer invocation for one query.
      - id: impl_LayerStatus
        signature: "impl LayerStatus {"
        children:
        - id: impl_LayerStatus.new
          signature: "pub fn new(layer_id: impl Into<String>, stage: LayerStage) -> Self {"
      - id: LayerContext
        signature: /// Per-query context the engine hands to a layer's `run` call. /// /// - `quer…
        docstring: Per-query context the engine hands to a layer's `run` call.
      - id: BackgroundContext
        signature: /// Global context the engine hands to a layer's `background` call. /// /// Sam…
        docstring: Global context the engine hands to a layer's `background` call.
      - id: Layer
        signature: /// Base trait every concrete RAG layer implements. /// /// A layer is a fully …
        docstring: Base trait every concrete RAG layer implements.
        children:
        - id: Layer.id
          signature: "/// Stable, globally unique layer identity. Used for: registry lookup,"
          docstring: "Stable, globally unique layer identity. Used for: registry lookup,"
        - id: Layer.stage
          signature: "/// Topology hint; see [`LayerStage`]. Affects default scheduling /// relative …"
          docstring: "Topology hint; see [`LayerStage`]. Affects default scheduling"
        - id: Layer.depends_on
          signature: /// IDs of other layers whose contribution to the *current query run* /// must …
          docstring: IDs of other layers whose contribution to the *current query run*
        - id: Layer.applies
          signature: /// Decide whether this layer participates in the given query. /// /// Default:…
          docstring: Decide whether this layer participates in the given query.
        - id: Layer.run
          signature: /// Process one query against the shared `ResultSet`.
          docstring: Process one query against the shared `ResultSet`.
        - id: Layer.background
          signature: "/// Optional, self-directed background activity (e.g. lazy index/cache /// buil…"
          docstring: "Optional, self-directed background activity (e.g. lazy index/cache"
```
/answer 8b5f-15 allow
Control Request:
```yaml
id: 8b5f-16
toolName: list
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/core
  - path: /home/user/xyan/xy.ai.workbench/rag/src
  reason: Understand project structure to know where to add executor module
```
/answer 8b5f-16 allow
Control Request:
```yaml
id: 8b5f-17
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src
      entries:
      - ".:"
      - lib.rs
      - "./bin:"
      - xyrag.rs
      - "./core:"
      - engine.rs
      - layer.rs
      - mod.rs
      - persistence.rs
      - query.rs
      - registry.rs
      - result.rs
      - topology.rs
      - "./layers:"
      - mod.rs
```
/answer 8b5f-17 allow
ReasoningToken: 14
Control Request:
```yaml
id: 8b5f-18
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/core/layer.rs
    ids:
    - LayerContext
    - BackgroundContext
    - Layer
  reason: Read LayerContext definition in detail
```
/answer 8b5f-18 allow
Control Request:
```yaml
id: 8b5f-19
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/core/layer.rs
      nodes:
      - id: LayerContext
        type: ItemStruct
        code: |
          /// Per-query context the engine hands to a layer's `run` call.
          ///
          /// - `query`: the current `Query` object (also passed separately to `run`
          ///   for convenience).
          /// - `storage`: this layer's own `LayerStorage` - the only place a layer
          ///   should persist state; its key space and content structure are
          ///   entirely private to the layer.
          /// - `shared_index`: the process-wide `SharedIndex` for cross-layer change
          ///   detection (path/hash/size/mtime/sequence number).
          pub struct LayerContext {
              pub query: Query,
              pub storage: Arc<LayerStorage>,
              pub shared_index: Arc<SharedIndex>,
          }
      - id: BackgroundContext
        type: ItemStruct
        code: |
          /// Global context the engine hands to a layer's `background` call.
          ///
          /// Same storage/index handles as `LayerContext`, but deliberately without a
          /// `Query`: background activity is decoupled from any single request and
          /// must decide its own scope and priority (e.g. from what previous queries
          /// touched, or from `shared_index` changes) without being told what to do.
          pub struct BackgroundContext {
              pub storage: Arc<LayerStorage>,
              pub shared_index: Arc<SharedIndex>,
          }
      - id: Layer
        type: ItemTrait
        signature: /// Base trait every concrete RAG layer implements. /// /// A layer is a fully …
        docstring: Base trait every concrete RAG layer implements.
        children:
        - id: Layer.id
          type: TraitItemFn
          code: |2
                /// Stable, globally unique layer identity. Used for: registry lookup,
                /// `LayerStorage` namespace, and the default query-activation field
                /// name. Never reuse an id for a semantically different layer; version
                /// internally via `LayerStorage` metadata instead.
                fn id(&self) -> &str;
        - id: Layer.stage
          type: TraitItemFn
          code: |2
                /// Topology hint; see [`LayerStage`]. Affects default scheduling
                /// relative to other layers, not correctness - correctness must not
                /// depend on stage ordering alone when explicit ordering matters (use
                /// `depends_on` for that).
                fn stage(&self) -> LayerStage {
                    LayerStage::Enrich
                }
        - id: Layer.depends_on
          type: TraitItemFn
          code: |2
                /// IDs of other layers whose contribution to the *current query run*
                /// must be complete before this layer's `run` is invoked. Layers
                /// without (mutual) dependencies may be scheduled fully in parallel
                /// (fork); the engine joins dependents after their dependencies
                /// finish. Empty by default for layers that can run at any point
                /// relative to others.
                fn depends_on(&self) -> HashSet<String> {
                    HashSet::new()
                }
        - id: Layer.applies
          type: TraitItemFn
          code: |2
                /// Decide whether this layer participates in the given query.
                ///
                /// Default: activate when the query carries a field named exactly like
                /// `self.id()` (`query.has(self.id())`) - the declarative activation
                /// path. Override to inspect the full query (`query.inspect()`) and
                /// decide based on arbitrary combinations of fields instead (the
                /// analytical activation path), e.g. a layer that only reacts when
                /// both a `text` and a `language` field are present, or that activates
                /// on any cache-id-shaped field.
                ///
                /// Called by the engine before `run`; must be cheap and side-effect
                /// free, and must not assume any field other than those it explicitly
                /// checks is present.
                fn applies(&self, query: &Query) -> bool {
                    query.has(self.id())
                }
        - id: Layer.run
          type: TraitItemFn
          code: |2
                /// Process one query against the shared `ResultSet`.
                ///
                /// Invoked once per query for which `applies` returned true. Must
                /// implement exactly one of the three roles implied by `stage`:
                ///
                /// - Generate: create new `ResultEntry` objects (`ResultSet::add`) from
                ///   the query and/or from this layer's own data source, either
                ///   autonomously or triggered by matching query fields. If this is
                ///   the first layer to contribute in the run, it is responsible for
                ///   the `ResultSet` coming into existence (the engine still owns and
                ///   passes the instance; the layer simply populates it).
                /// - Enrich: look up existing entries via shared fields
                ///   (`ResultSet::find`/`ResultSet::get`) and extend them via
                ///   `ResultEntry::merge` (adding/overwriting fields, appending signal
                ///   names). Must tolerate zero matching entries (e.g. because an
                ///   expected upstream layer did not run or is absent from this
                ///   topology) and should then report `skipped`/`contributions = 0`
                ///   rather than fail.
                /// - Postprocess: operate on the (assumed complete, per `depends_on`)
                ///   result set as a whole, e.g. `ResultSet::sort_by_key` with a
                ///   caller-chosen key for stable ordering, or reduce/replace entries
                ///   (e.g. the cache-reduction half of the two-stage pattern).
                ///
                /// Must be cooperative with cancellation/timeboxing performed by the
                /// caller (accept being cancelled between awaited points) and must
                /// reflect what actually happened in the returned `LayerStatus`
                /// (`ran`/`skipped`/`aborted`/`contributions`), since that is the only
                /// outcome signal available to the caller - there is no numeric score
                /// to fall back on.
                ///
                /// Must not assume any particular set or order of other layers is
                /// present beyond what it declared via `depends_on`; all other
                /// coordination happens implicitly through shared `ResultEntry` field
                /// names.
                async fn run(&self, query: &Query, result_set: &ResultSet, ctx: &LayerContext) -> LayerStatus;
        - id: Layer.background
          type: TraitItemFn
          code: |2
                /// Optional, self-directed background activity (e.g. lazy index/cache
                /// build).
                ///
                /// Default: no background activity. A layer that overrides this is
                /// entirely responsible for deciding, on its own, whether and when to
                /// run (e.g. based on what recent queries touched, or on changes seen
                /// through `ctx.shared_index`), how to prioritize its work, and how to
                /// make it resumable/abortable. There is no query object here - this
                /// flow is global and decoupled from any single request.
                ///
                /// Must check `cancel` frequently (between small units of work) and
                /// return promptly once it is cancelled: the hosting CLI process will
                /// not terminate until every layer's `background` call has returned,
                /// so unresponsive cancellation handling stalls process shutdown.
                /// Persist progress incrementally via `ctx.storage` so an aborted run
                /// loses at most the current unit of work and leaves previously
                /// written data usable.
                async fn background(&self, _ctx: &BackgroundContext, _cancel: CancellationToken) {
                    // Default: no background activity.
                }
```
/answer 8b5f-19 allow
Control Request:
```yaml
id: 8b5f-1a
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/rag/src/core/engine.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/core/mod.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/lib.rs
  reason: Understand engine and module structure to integrate executor
```
/answer 8b5f-1a allow
Control Request:
```yaml
id: 8b5f-1b
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/core/engine.rs
      nodes:
      - id: jQwv50|sQUvVv
        signature: use std::collections::HashMap;
      - id: ExecutionMode
        signature: "#[derive(Debug, Clone, Copy, PartialEq, Eq)] pub enum ExecutionMode {"
      - id: impl_ExecutionMode
        signature: "impl ExecutionMode {"
        children:
        - id: impl_ExecutionMode.parse
          signature: "pub fn parse(s: &str) -> Option<Self> {"
        - id: impl_ExecutionMode.as_str
          signature: "pub fn as_str(&self) -> &'static str {"
      - id: Engine
        signature: "/// Connects registry, persistence and topology execution. /// /// Used both as…"
        docstring: "Connects registry, persistence and topology execution."
      - id: impl_Engine
        signature: "impl Engine {"
        children:
        - id: impl_Engine.new
          signature: "pub fn new(     registry: LayerRegistry,     persistence: PersistenceManager,  …"
        - id: impl_Engine.active_layers
          signature: "pub fn active_layers(&self, query: &Query) -> Vec<Arc<dyn Layer>> {"
        - id: impl_Engine.run_query
          signature: /// Runs one query against all activated layers. /// /// The first executed lay…
          docstring: Runs one query against all activated layers.
        - id: impl_Engine.run_layer
          signature: "async fn run_layer(     &self,     layer: Arc<dyn Layer>,     query: &Query,   …"
        - id: impl_Engine.start_background
          signature: "/// Starts the background activity of every layer (once, globally). /// /// Eac…"
          docstring: "Starts the background activity of every layer (once, globally)."
        - id: impl_Engine.wait_background
          signature: /// Waits until all background activity has finished. /// /// A CLI process cal…
          docstring: Waits until all background activity has finished.
        - id: impl_Engine.cancel_background
          signature: "pub fn cancel_background(&self) {"
        - id: impl_Engine.shutdown
          signature: "pub async fn shutdown(&mut self, cancel: bool) {"
    - path: /home/user/xyan/xy.ai.workbench/rag/src/core/mod.rs
      nodes:
      - id: engine
        signature: pub mod engine;
      - id: layer
        signature: pub mod layer;
      - id: persistence
        signature: pub mod persistence;
      - id: query
        signature: pub mod query;
      - id: registry
        signature: pub mod registry;
      - id: result
        signature: pub mod result;
      - id: topology
        signature: pub mod topology;
    - path: /home/user/xyan/xy.ai.workbench/rag/src/lib.rs
      nodes:
      - id: core
        signature: pub mod core;
      - id: layers
        signature: pub mod layers;
      - id: xLpY3U|tx8ZzX
        signature: "pub const VERSION: &str = env!(\"CARGO_PKG_VERSION\");"
```
/answer 8b5f-1b allow
Control Request:
```yaml
id: 8b5f-1c
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench/rag
  script: cat Cargo.toml
  reason: Check dependencies for rayon
```
/answer 8b5f-1c allow
Control Request:
```yaml
id: 8b5f-1d
result:
  structuredContent:
    stdout: |
      [package]
      name = "xy-ai-rag"
      version = "0.1.0"
      edition = "2021"
      description = "xy.ai.rag - Layered Anytime Retrieval Engine"

      [lib]
      name = "xy_ai_rag"
      path = "src/lib.rs"

      [[bin]]
      name = "xyrag"
      path = "src/bin/xyrag.rs"

      [dependencies]
      tokio = { version = "1", features = ["rt-multi-thread", "macros", "sync", "time"] }
      tokio-util = "0.7"
      async-trait = "0.1"
      futures = "0.3"
      serde = { version = "1", features = ["derive"] }
      serde_json = "1"
      rusqlite = { version = "0.31", features = ["bundled"] }
      clap = { version = "4", features = ["derive"] }
      anyhow = "1"
```
/answer 8b5f-1d allow
ReasoningToken: 33
Control Request:
```yaml
id: 8b5f-1e
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/core/engine.rs
    ids:
    - Engine
    - impl_Engine
  reason: Read full Engine implementation to integrate rayon executor
```
/answer 8b5f-1e allow
Control Request:
```yaml
id: 8b5f-1f
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/core/engine.rs
      nodes:
      - id: Engine
        type: ItemStruct
        code: |
          /// Connects registry, persistence and topology execution.
          ///
          /// Used both as a library (`Engine::run_query`) and instantiated on-demand
          /// by the CLI.
          pub struct Engine {
              pub registry: LayerRegistry,
              pub persistence: Arc<PersistenceManager>,
              pub mode: ExecutionMode,
              cancel_tokens: HashMap<String, CancellationToken>,
              background_tasks: HashMap<String, JoinHandle<()>>,
          }
      - id: impl_Engine
        type: ItemImpl
        signature: "impl Engine {"
        children:
        - id: impl_Engine.new
          type: ImplItemFn
          code: |2
                pub fn new(registry: LayerRegistry, persistence: PersistenceManager, mode: ExecutionMode) -> Self {
                    Self {
                        registry,
                        persistence: Arc::new(persistence),
                        mode,
                        cancel_tokens: HashMap::new(),
                        background_tasks: HashMap::new(),
                    }
                }
        - id: impl_Engine.active_layers
          type: ImplItemFn
          code: |2
                pub fn active_layers(&self, query: &Query) -> Vec<Arc<dyn Layer>> {
                    self.registry
                        .iter()
                        .filter(|l| l.applies(query))
                        .cloned()
                        .collect()
                }
        - id: impl_Engine.run_query
          type: ImplItemFn
          code: |2
                /// Runs one query against all activated layers.
                ///
                /// The first executed layer implicitly creates the `ResultSet`, by
                /// writing into the empty instance created here - even if it is itself
                /// only an enrichment layer.
                pub async fn run_query(&self, query: Query) -> Result<(ResultSet, Vec<LayerStatus>)> {
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
        - id: impl_Engine.run_layer
          type: ImplItemFn
          code: |2
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
                    };
                    Ok(layer.run(query, result_set, &ctx).await)
                }
        - id: impl_Engine.start_background
          type: ImplItemFn
          code: |2
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
                        let layer_clone = layer.clone();
                        let handle = tokio::spawn(async move {
                            let ctx = BackgroundContext {
                                storage,
                                shared_index,
                            };
                            layer_clone.background(&ctx, cancel).await;
                        });
                        self.background_tasks.insert(id, handle);
                    }
                }
        - id: impl_Engine.wait_background
          type: ImplItemFn
          code: |2
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
        - id: impl_Engine.cancel_background
          type: ImplItemFn
          code: |2
                pub fn cancel_background(&self) {
                    for token in self.cancel_tokens.values() {
                        token.cancel();
                    }
                }
        - id: impl_Engine.shutdown
          type: ImplItemFn
          code: |2
                pub async fn shutdown(&mut self, cancel: bool) {
                    if cancel {
                        self.cancel_background();
                    }
                    self.wait_background().await;
                }
```
/answer 8b5f-1f allow
Control Request:
```yaml
id: 8b5f-20
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/rag/src/core/topology.rs
  reason: Check topology execution to understand concurrency model
```
/answer 8b5f-20 allow
Control Request:
```yaml
id: 8b5f-21
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/core/topology.rs
      nodes:
      - id: tfOe3j|hmVGxC
        signature: "use std::collections::{HashMap, HashSet};"
      - id: CyclicDependencyError
        signature: "#[derive(Debug)]"
      - id: impl_fmt__Display_for_CyclicDependencyError
        signature: "impl fmt::Display for CyclicDependencyError {"
        children:
        - id: impl_fmt__Display_for_CyclicDependencyError.fmt
          signature: "fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {"
      - id: impl_std__error__Error_for_CyclicDependencyError
        signature: "impl std::error::Error for CyclicDependencyError {"
      - id: stage_order
        signature: "fn stage_order(stage: LayerStage) -> u8 {"
      - id: build_levels
        signature: /// Topological sort into levels ("fork/join" groups). /// /// Each level conta…
        docstring: Topological sort into levels ("fork/join" groups).
```
/answer 8b5f-21 allow
ReasoningToken: 57
Control Request:
```yaml
id: 8b5f-22
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/rag/src/core/layer.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/core/engine.rs
  node_type: Use
  reason: Check full use/import statements in layer.rs and engine.rs
```
/answer 8b5f-22 allow
Control Request:
```yaml
id: 8b5f-23
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/core/layer.rs
      nodes: []
    - path: /home/user/xyan/xy.ai.workbench/rag/src/core/engine.rs
      nodes: []
```
/answer 8b5f-23 allow
ReasoningToken: 1206
Control Request:
```yaml
id: 8b5f-24
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench/rag
  script: "sed -n '1,20p' src/core/layer.rs; echo ---; sed -n '1,25p' src/core/engine.rs"
  reason: See top imports of both files
```
/answer 8b5f-24 deny nutze ast_outline
Control Request:
```yaml
id: 8b5f-25
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/core/layer.rs
    ids:
    - 23lmSF|mEOgEf
  - path: /home/user/xyan/xy.ai.workbench/rag/src/core/engine.rs
    ids:
    - jQwv50|sQUvVv
  reason: Read import blocks of layer.rs and engine.rs
```
/answer 8b5f-25 allow
Control Request:
```yaml
id: 8b5f-26
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/core/layer.rs
      nodes:
      - id: 23lmSF|mEOgEf
        type: ImportGroup
        code: |
          use std::collections::HashSet;
          use std::sync::Arc;

          use async_trait::async_trait;
          use serde_json::{Map, Value};
          use tokio_util::sync::CancellationToken;

          use crate::core::persistence::{LayerStorage, SharedIndex};
          use crate::core::query::Query;
          use crate::core::result::ResultSet;
    - path: /home/user/xyan/xy.ai.workbench/rag/src/core/engine.rs
      nodes:
      - id: jQwv50|sQUvVv
        type: ImportGroup
        code: |
          use std::collections::HashMap;
          use std::sync::Arc;

          use anyhow::Result;
          use futures::future::join_all;
          use tokio::task::JoinHandle;
          use tokio_util::sync::CancellationToken;

          use crate::core::layer::{BackgroundContext, Layer, LayerContext, LayerStatus};
          use crate::core::persistence::PersistenceManager;
          use crate::core::query::Query;
          use crate::core::registry::LayerRegistry;
          use crate::core::result::ResultSet;
          use crate::core::topology::build_levels;
```
/answer 8b5f-26 allow
Text:
 Now implementing. First add rayon dependency:
Control Request:
```yaml
id: 8b5f-27
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/Cargo.toml
    old_text: anyhow = "1"
    new_text: |-
      anyhow = "1"
      rayon = "1"
  reason: Add rayon dependency for CPU executor
```
/answer 8b5f-27 allow
Control Request:
```yaml
id: 8b5f-28
toolName: ast_create
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/core/executor.rs
    source: |
      //! Rayon-backed CPU executor shared by all layers.
      //!
      //! `tokio` in this crate schedules I/O-bound/async work; CPU-bound work
      //! (parsing, embedding math, hashing, etc.) must not run directly on a
      //! tokio worker thread, as it would block other queries' progress.
      //! `CpuExecutor` bridges the two: it owns a dedicated rayon thread pool
      //! sized to all available CPU cores (rayon's default) and exposes an
      //! `async fn spawn` that offloads a closure onto that pool and resolves
      //! once it completes, without occupying a tokio worker while the CPU work
      //! runs.

      use std::sync::Arc;

      use anyhow::{anyhow, Result};
      use rayon::{ThreadPool, ThreadPoolBuilder};

      /// Shared handle to a process-wide rayon thread pool.
      ///
      /// One instance is created by the `Engine` and handed to every layer via
      /// `LayerContext`/`BackgroundContext`, so all layers compete for and share
      /// the same bounded set of CPU threads rather than each spinning up its
      /// own pool.
      pub struct CpuExecutor {
          pool: ThreadPool,
      }

      impl CpuExecutor {
          /// Builds a pool using rayon's default sizing, i.e. one worker thread
          /// per available CPU core - maximal CPU utilization for offloaded
          /// work without oversubscription.
          pub fn new() -> Result<Self> {
              let pool = ThreadPoolBuilder::new()
                  .thread_name(|i| format!("xy-rag-cpu-{i}"))
                  .build()
                  .map_err(|e| anyhow!("failed to build rayon thread pool: {e}"))?;
              Ok(Self { pool })
          }

          /// Number of worker threads backing this pool.
          pub fn num_threads(&self) -> usize {
              self.pool.current_num_threads()
          }

          /// Runs `f` on the rayon pool and awaits its result without blocking
          /// the calling tokio worker thread.
          ///
          /// Panics inside `f` are propagated as an `Err` rather than poisoning
          /// the pool or the caller's task.
          pub async fn spawn<F, R>(&self, f: F) -> Result<R>
          where
              F: FnOnce() -> R + Send + 'static,
              R: Send + 'static,
          {
              let (tx, rx) = tokio::sync::oneshot::channel();
              self.pool.spawn(move || {
                  let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(f));
                  let _ = tx.send(result);
              });
              match rx.await {
                  Ok(Ok(r)) => Ok(r),
                  Ok(Err(panic)) => {
                      let msg = panic
                          .downcast_ref::<&str>()
                          .map(|s| s.to_string())
                          .or_else(|| panic.downcast_ref::<String>().cloned())
                          .unwrap_or_else(|| "CPU task panicked".to_string());
                      Err(anyhow!("rayon task panicked: {msg}"))
                  }
                  Err(_) => Err(anyhow!("rayon task dropped sender without producing a result")),
              }
          }

          /// Runs a rayon `par_iter`/scope-style closure with the pool installed
          /// as the current thread's rayon context, blocking the calling thread
          /// until it completes.
          ///
          /// Intended for use from inside a `CpuExecutor::spawn` closure (i.e.
          /// already off the tokio runtime) when a layer wants fine-grained data
          /// parallelism (e.g. `rayon::prelude::*` iterator methods) rather than
          /// a single offloaded closure.
          pub fn install<F, R>(&self, f: F) -> R
          where
              F: FnOnce() -> R + Send,
              R: Send,
          {
              self.pool.install(f)
          }
      }

      pub type SharedCpuExecutor = Arc<CpuExecutor>;
  reason: Create the rayon-based CPU executor module
```
/answer 8b5f-28 allow
Control Request:
```yaml
id: 8b5f-29
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/core/mod.rs
    old_text: pub mod engine;
    new_text: |-
      pub mod engine;
      pub mod executor;
  reason: Register executor module
```
/answer 8b5f-29 allow
Control Request:
```yaml
id: 8b5f-2a
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/core/layer.rs
    old_text: |-
      use crate::core::persistence::{LayerStorage, SharedIndex};
      use crate::core::query::Query;
    new_text: |-
      use crate::core::executor::CpuExecutor;
      use crate::core::persistence::{LayerStorage, SharedIndex};
      use crate::core::query::Query;
  reason: Import CpuExecutor into layer.rs
```
/answer 8b5f-2a allow
Control Request:
```yaml
id: 8b5f-2b
toolName: ast_edit_marks
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/core/layer.rs
    start_marker: "pub struct LayerContext {"
    end_marker: |-
      pub shared_index: Arc<SharedIndex>,
      }

      /// Global context
    content: |-
      pub struct LayerContext {
          pub query: Query,
          pub storage: Arc<LayerStorage>,
          pub shared_index: Arc<SharedIndex>,
          /// Shared rayon-backed executor for CPU-bound work; see
          /// [`CpuExecutor::spawn`] to offload work without blocking the async
          /// runtime.
          pub cpu: Arc<CpuExecutor>,
      }

      /// Global context
  reason: Add cpu field to LayerContext
```
/answer 8b5f-2b allow
Control Request:
```yaml
id: 8b5f-2c
result:
  structuredContent:
    errors:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/core/layer.rs
      error: No node matched the given text; a node selector (id) is required.
```
/answer 8b5f-2c allow
Control Request:
```yaml
id: 8b5f-2d
toolName: ast_edit_marks
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/core/layer.rs
    start_marker: "pub struct BackgroundContext {"
    end_marker: |-
      pub shared_index: Arc<SharedIndex>,
      }
    content: |-
      pub struct BackgroundContext {
          pub storage: Arc<LayerStorage>,
          pub shared_index: Arc<SharedIndex>,
          /// Shared rayon-backed executor for CPU-bound work; see
          /// [`CpuExecutor::spawn`] to offload work without blocking the async
          /// runtime.
          pub cpu: Arc<CpuExecutor>,
      }
  reason: Add cpu field to BackgroundContext
```
/answer 8b5f-2d allow
Control Request:
```yaml
id: 8b5f-2e
toolName: ast_edit_marks
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/core/layer.rs
    id: LayerContext
    start_marker: "pub struct LayerContext {"
    end_marker: |-
      pub shared_index: Arc<SharedIndex>,
      }
    content: |-
      pub struct LayerContext {
          pub query: Query,
          pub storage: Arc<LayerStorage>,
          pub shared_index: Arc<SharedIndex>,
          /// Shared rayon-backed executor for CPU-bound work; see
          /// [`CpuExecutor::spawn`] to offload work without blocking the async
          /// runtime.
          pub cpu: Arc<CpuExecutor>,
      }
  reason: Add cpu field to LayerContext
```
/answer 8b5f-2e allow
Control Request:
```yaml
id: 8b5f-2f
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/core/engine.rs
    old_text: "use crate::core::layer::{BackgroundContext, Layer, LayerContext, LayerStatus};"
    new_text: |-
      use crate::core::executor::CpuExecutor;
      use crate::core::layer::{BackgroundContext, Layer, LayerContext, LayerStatus};
  - path: /home/user/xyan/xy.ai.workbench/rag/src/core/engine.rs
    old_text: |2-
          pub mode: ExecutionMode,
          cancel_tokens: HashMap<String, CancellationToken>,
    new_text: |2-
          pub mode: ExecutionMode,
          /// Shared rayon pool handed to every layer for CPU-bound work.
          pub cpu: Arc<CpuExecutor>,
          cancel_tokens: HashMap<String, CancellationToken>,
  - path: /home/user/xyan/xy.ai.workbench/rag/src/core/engine.rs
    old_text: |2-
          pub fn new(registry: LayerRegistry, persistence: PersistenceManager, mode: ExecutionMode) -> Self {
              Self {
                  registry,
                  persistence: Arc::new(persistence),
                  mode,
                  cancel_tokens: HashMap::new(),
                  background_tasks: HashMap::new(),
              }
          }
    new_text: |2-
          pub fn new(registry: LayerRegistry, persistence: PersistenceManager, mode: ExecutionMode) -> Self {
              let cpu = Arc::new(CpuExecutor::new().expect("failed to initialize CPU executor"));
              Self {
                  registry,
                  persistence: Arc::new(persistence),
                  mode,
                  cpu,
                  cancel_tokens: HashMap::new(),
                  background_tasks: HashMap::new(),
              }
          }
  - path: /home/user/xyan/xy.ai.workbench/rag/src/core/engine.rs
    old_text: |2-
                  query: query.clone(),
                  storage,
                  shared_index: self.persistence.shared_index.clone(),
              };
              Ok(layer.run(query, result_set, &ctx).await)
    new_text: |2-
                  query: query.clone(),
                  storage,
                  shared_index: self.persistence.shared_index.clone(),
                  cpu: self.cpu.clone(),
              };
              Ok(layer.run(query, result_set, &ctx).await)
  - path: /home/user/xyan/xy.ai.workbench/rag/src/core/engine.rs
    old_text: |2-
                  let shared_index = self.persistence.shared_index.clone();
                  let layer_clone = layer.clone();
                  let handle = tokio::spawn(async move {
                      let ctx = BackgroundContext {
                          storage,
                          shared_index,
                      };
    new_text: |2-
                  let shared_index = self.persistence.shared_index.clone();
                  let cpu = self.cpu.clone();
                  let layer_clone = layer.clone();
                  let handle = tokio::spawn(async move {
                      let ctx = BackgroundContext {
                          storage,
                          shared_index,
                          cpu,
                      };
  reason: Wire CpuExecutor into Engine and its contexts
```
/answer 8b5f-2f allow
Control Request:
```yaml
id: 8b5f-30
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/core/engine.rs
      result: success
      new_id: jQwv50|sQUvVv
    - path: /home/user/xyan/xy.ai.workbench/rag/src/core/engine.rs
      result: success
      new_id: Engine
    - path: /home/user/xyan/xy.ai.workbench/rag/src/core/engine.rs
      result: success
      new_id: impl_Engine.run_layer
    - path: /home/user/xyan/xy.ai.workbench/rag/src/core/engine.rs
      result: success
      new_id: impl_Engine.start_background
    errors:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/core/engine.rs
      error: No node matched the given text; a node selector (id) is required.
```
/answer 8b5f-30 allow
Text:
 Now register module and wire it into layer.rs contexts and engine.rs:
Control Request:
```yaml
id: 8b5f-31
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/core/engine.rs
    id: impl_Engine.new
    old_text: |-
      pub fn new(registry: LayerRegistry, persistence: PersistenceManager, mode: ExecutionMode) -> Self {
              Self {
                  registry,
                  persistence: Arc::new(persistence),
                  mode,
                  cancel_tokens: HashMap::new(),
                  background_tasks: HashMap::new(),
              }
          }
    new_text: |-
      pub fn new(registry: LayerRegistry, persistence: PersistenceManager, mode: ExecutionMode) -> Self {
              let cpu = Arc::new(CpuExecutor::new().expect("failed to initialize CPU executor"));
              Self {
                  registry,
                  persistence: Arc::new(persistence),
                  mode,
                  cpu,
                  cancel_tokens: HashMap::new(),
                  background_tasks: HashMap::new(),
              }
          }
  reason: Add cpu executor construction to Engine::new
```
/answer 8b5f-31 allow
Control Request:
```yaml
id: 8b5f-32
result:
  structuredContent:
    errors:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/core/engine.rs
      error: Text not found in node.
      id: impl_Engine.new
      reason: not_found
      next_step: reread_node
```
/answer 8b5f-32 allow
Control Request:
```yaml
id: 8b5f-33
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/core/engine.rs
    ids:
    - impl_Engine.new
  reason: Recheck current content of Engine::new after previous edits
```
/answer 8b5f-33 allow
Control Request:
```yaml
id: 8b5f-34
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/core/engine.rs
      nodes:
      - id: impl_Engine.new
        type: ImplItemFn
        code: |2
              pub fn new(
                  registry: LayerRegistry,
                  persistence: PersistenceManager,
                  mode: ExecutionMode,
              ) -> Self {
                  Self {
                      registry,
                      persistence: Arc::new(persistence),
                      mode,
                      cancel_tokens: HashMap::new(),
                      background_tasks: HashMap::new(),
                  }
              }
```
/answer 8b5f-34 allow
Control Request:
```yaml
id: 8b5f-35
toolName: ast_replace
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/core/engine.rs
    id: impl_Engine.new
    source: |2
          pub fn new(
              registry: LayerRegistry,
              persistence: PersistenceManager,
              mode: ExecutionMode,
          ) -> Self {
              let cpu = Arc::new(CpuExecutor::new().expect("failed to initialize CPU executor"));
              Self {
                  registry,
                  persistence: Arc::new(persistence),
                  mode,
                  cpu,
                  cancel_tokens: HashMap::new(),
                  background_tasks: HashMap::new(),
              }
          }
  reason: Initialize CpuExecutor in Engine::new
```
/answer 8b5f-35 allow
Text:
 Need id for LayerContext marker edit:
Control Request:
```yaml
id: 8b5f-36
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench/rag
  script: cargo build 2>&1 | tail -80
  reason: Build the crate to verify the rayon executor integration compiles
```
/answer 8b5f-36 allow
Text:
 Now update engine.rs: add `cpu` field to Engine, construct it in `new()`, and pass into contexts.
Control Request:
```yaml
id: 8b5f-37
result:
  structuredContent:
    stdout: |2
          Updating crates.io index
           Locking 6 packages to highest compatible versions
            Adding crossbeam-deque v0.8.8
            Adding crossbeam-epoch v0.9.21
            Adding crossbeam-utils v0.8.23
            Adding either v1.18.0
            Adding rayon v1.12.0
            Adding rayon-core v1.13.0
         Compiling crossbeam-utils v0.8.23
         Compiling crossbeam-epoch v0.9.21
         Compiling crossbeam-deque v0.8.8
         Compiling rayon-core v1.13.0
         Compiling either v1.18.0
         Compiling rayon v1.12.0
         Compiling xy-ai-rag v0.1.0 (/home/user/xyan/xy.ai.workbench/rag)
          Finished `dev` profile [unoptimized + debuginfo] target(s) in 4.04s
```
/answer 8b5f-37 allow
Result Stats: id=30efbf5c-4905-4f48-9959-102efdb2ab4c, total: 34679, in: 163, out: 9311, reason: 145, read: 268648, write: 25205
ReasoningToken: 104
ReasoningToken: 41
Text:
 Formatting differs (reformatted by tool). Use ast_replace instead.
Text:
 Now validate everything compiles.
Builds cleanly. Summary of the implementation:

- **`src/core/executor.rs`** (new): `CpuExecutor` wraps a dedicated `rayon::ThreadPool` (default-sized to all CPU cores). `CpuExecutor::spawn` offloads a closure onto the pool and bridges the result back via a `tokio::sync::oneshot` channel without blocking the async runtime; panics are caught and turned into `Err`. `CpuExecutor::install` exposes `rayon::ThreadPool::install` for layers that want to use `rayon::prelude::*` data-parallel iterators once already off the tokio runtime.
- **`core/mod.rs`**: registers the new `executor` module.
- **`core/layer.rs`**: `LayerContext` and `BackgroundContext` both gained a `pub cpu: Arc<CpuExecutor>` field, so every layer's `run`/`background` call has access to the shared pool.
- **`core/engine.rs`**: `Engine` now owns one `Arc<CpuExecutor>` (created once in `Engine::new`) and passes it into both `run_layer`'s `LayerContext` and `start_background`'s `BackgroundContext`, so all layers share the same bounded CPU pool.
- **`Cargo.toml`**: added `rayon = "1"`.

Layers use it like `ctx.cpu.spawn(move || heavy_cpu_work(...)).await?` or `ctx.cpu.install(|| data.par_iter().map(...).collect())`.
