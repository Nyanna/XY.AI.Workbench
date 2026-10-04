//! Layer contract of the xy.ai.rag engine.
//!
//! This module is the single, self-sufficient reference for implementing a
//! concrete layer. Read this documentation together with the trait/type
//! docs below before writing a new layer; no other document should be
//! necessary.
//!
//! # Topology, not a pipeline
//!
//! Layers are not stages in a fixed pipeline. Depending on configuration
//! and on what a layer declares via [`Layer::depends_on`]/[`Layer::stage`],
//! the engine composes them as:
//!
//! - a sequence, where later layers enrich/observe a result produced by an
//!   earlier layer,
//! - a fully parallel fork/join set, where independent layers run
//!   concurrently against the same shared [`ResultSet`],
//! - a mix, where some layers wait for individual other layers
//!   (`depends_on`) while others wait for the *whole* result set to be
//!   complete (stage [`LayerStage::Postprocess`]) before post-processing it
//!   (e.g. stable sort, cache reduction).
//!
//! The engine performs no numeric ranking. Ordering is a stable sort
//! applied by post-processing layers over whatever fields they choose.
//!
//! # The shared result object
//!
//! All layers in one query run operate on one shared [`ResultSet`] (see
//! [`crate::core::result`]). It is weakly typed: a `ResultEntry` is a free
//! bag of fields plus a list of signal names. There is no fixed schema -
//! an entry may carry a file path, only an AST node id, line numbers, a
//! text excerpt, or just a summary. The *first* layer that contributes in
//! a given run creates the `ResultSet`, even if that layer's own role is
//! enrichment rather than generation; there is no separate "root" layer
//! type.
//!
//! Aggregation across layers is implicit and happens purely through shared
//! field names: a layer looks up existing entries (e.g. via
//! `ResultSet::find` by `path`) and calls `ResultEntry::merge` to add or
//! overwrite fields and append signals. Example chain: a grep-like
//! text-search layer creates entries with `path`/`line`; a semantic layer
//! finds those entries by `path` and replaces `line` with a trimmed
//! excerpt; a code-AST layer finds them again and adds `outline`/`fqn`
//! fields. No layer needs to know about any other layer's existence - only
//! about the field names it reads and writes.
//!
//! # Query activation
//!
//! A [`Query`] (see [`crate::core::query`]) is likewise a weakly typed,
//! dynamic object (free-form fields, e.g. `include`/`exclude`, a text or
//! function-name search, cache ids, ...). A layer decides for itself
//! whether it participates in a given query, either by
//!
//! - declarative activation: the default [`Layer::applies`] implementation
//!   checks for the presence of a field named after the layer's own `id`
//!   in the query, or
//! - inspecting the query: override `applies` and use `Query::inspect` to
//!   analyze the full field set and decide based on arbitrary logic.
//!
//! A layer must not assume any other field is present; always use
//! `Query::has`/`Query::get` defensively.
//!
//! # Two control flows
//!
//! Every layer has exactly two independent control flows, both defined on
//! this trait:
//!
//! 1. [`Layer::run`] - invoked once per query that the layer declared it
//!    `applies` to, via whatever channel/scheduler the engine uses to
//!    distribute query objects. This is where request-scoped work happens
//!    (generating candidates, enriching existing entries, post-processing).
//! 2. [`Layer::background`] - a global control flow, independent of any
//!    single query, used for activities such as lazy index/cache building.
//!    A layer decides entirely on its own, based on query history or its
//!    own state, whether, when and for how long to run background work;
//!    the engine only signals cancellation via the `cancel`
//!    `CancellationToken` (e.g. on CLI shutdown). The CLI process
//!    terminates only once all queries have returned *and* all layers'
//!    background activity has finished (`background` returned, or
//!    cancellation was honored).
//!
//! # Stage and dependencies as topology hints
//!
//! `stage` and `depends_on` describe a layer's place in the DAG, without
//! the engine imposing a rigid total order:
//!
//! - `Generate` layers may run autonomously (independent of any existing
//!   entries) or be driven by the query, and typically create new
//!   `ResultEntry` objects.
//! - `Enrich` layers observe/extend entries that already exist in the
//!   shared `ResultSet` (produced by some earlier generate/enrich layer in
//!   the same run), typically via `ResultSet::find` + `ResultEntry::merge`.
//! - `Postprocess` layers wait until the result set for the current query
//!   is considered complete (per `depends_on`, or by engine-level topology
//!   configuration) and then operate on the set as a whole, e.g. applying
//!   a stable `ResultSet::sort_by_key`, or reducing/replacing entries.
//!
//! `depends_on` names the ids of other layers whose contribution to the
//! *current query run* must be finished before this layer's `run` is
//! invoked. Layers without dependencies may run fully in parallel with
//! each other (fork), with the engine joining before dependents run.
//!
//! # Persistence contract
//!
//! All shared, disk-backed state lives under a `.xyrag` sub-folder, located
//! in the current working directory or at an explicitly supplied root path.
//! Two kinds of storage
//! are handed to a layer through its context structs:
//!
//! - [`crate::core::persistence::SharedIndex`]: one process-wide, central,
//!   persisted file index (path, content hash, size, mtime, monotonic
//!   sequence number, deletion tombstones) used for change detection that
//!   is shared *across* layers. A layer uses it to detect whether a given
//!   path/content has changed since it last looked, typically by comparing
//!   against a cursor it persisted itself
//!   (`LayerStorage::get_cursor`/`set_cursor`) and then reading
//!   `SharedIndex::changes_since`.
//! - [`crate::core::persistence::LayerStorage`]: a per-layer, abstracted
//!   persistence unit (a simple JSON-valued key/value cache plus a private
//!   directory for sidecar files, e.g. vector files or custom index
//!   files). The key space and the structure of what is stored under it
//!   (an entry for a chunk, a line range, a file, a path, or a composite
//!   object correlating several of these) is entirely up to the layer.
//!   Layers may deliberately share resources by agreeing on a common key
//!   scheme out of band, but nothing forces them to; the engine does not
//!   interpret the key space.
//!
//! A [`LayerContext`] (per-query) and a [`BackgroundContext`] (global) are
//! the only engine-provided handles to this persistence layer and to the
//! current `Query`; a layer implementation should not reach for engine
//! internals beyond what these contexts expose.
//!
//! # Identity and registration
//!
//! Every layer implementation is identified by a stable, unique `id`
//! string. A registry (outside this module, see
//! [`crate::core::registry::LayerRegistry`]) manages the set of known
//! layer implementations by this id; the id is also the storage namespace
//! (`LayerStorage` is keyed by it) and, by default, the query field name
//! that triggers activation. Treat it as a long-lived, versionless
//! identity - if a layer's behavior changes incompatibly, express that
//! through its own persisted metadata/versioning inside `LayerStorage`, not
//! by changing the id.
use std::collections::HashSet;
use std::sync::Arc;
use async_trait::async_trait;
use serde_json::{Map, Value};
use tokio_util::sync::CancellationToken;
use crate::core::executor::CpuExecutor;
use crate::core::persistence::{LayerStorage, SharedIndex};
use crate::core::query::Query;
use crate::core::result::ResultSet;
/// Coarse topology hint for a layer; does not replace explicit `depends_on`.
///
/// Classifies a layer's typical relationship to the shared `ResultSet`
/// within one query run; the engine may use it to decide default
/// scheduling (e.g. run all generate/enrich layers in a fork/join group,
/// then join postprocess layers once that group is done), but explicit
/// `depends_on` entries always take precedence where present.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LayerStage {
    /// Creates new `ResultEntry` objects, either autonomously (independent
    /// of other layers/entries) or driven by matching on the `Query`. The
    /// very first layer to contribute in a run implicitly creates the
    /// `ResultSet` itself, regardless of stage.
    Generate,
    /// Observes and extends entries already present in the shared
    /// `ResultSet` (typically via `ResultSet::find` + `merge`), without
    /// necessarily creating new entries of its own.
    Enrich,
    /// Waits for a (sub-)set of the `ResultSet` to be complete and then
    /// operates on it as a whole, e.g. stable sorting or cache reduction.
    /// Does not imply a numeric ranking - the engine never ranks
    /// numerically, only sorts stably.
    Postprocess,
}
/// Outcome protocol of one layer invocation for one query.
///
/// Returned by [`Layer::run`] so the engine/caller can distinguish "ran and
/// produced nothing" from "did not run" (`skipped`, e.g. because `applies`
/// was false) from "ran but was cancelled/timed out" (`aborted`). There is
/// no numeric score here; `contributions` is a plain count of entries
/// created or merged into, useful for logging and for downstream layers
/// that want to know whether upstream work happened at all.
#[derive(Debug, Clone)]
pub struct LayerStatus {
    pub layer_id: String,
    pub stage: LayerStage,
    pub ran: bool,
    pub skipped: bool,
    pub aborted: bool,
    pub contributions: usize,
    /// Free-form, layer-specific diagnostic detail (e.g. coverage,
    /// timings); not interpreted by the engine.
    pub detail: Map<String, Value>,
}
impl LayerStatus {
    pub fn new(layer_id: impl Into<String>, stage: LayerStage) -> Self {
        Self {
            layer_id: layer_id.into(),
            stage,
            ran: false,
            skipped: false,
            aborted: false,
            contributions: 0,
            detail: Map::new(),
        }
    }
}
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
    /// Shared rayon-backed executor for CPU-bound work; see
    /// [`CpuExecutor::spawn`] to offload work without blocking the async
    /// runtime.
    pub cpu: Arc<CpuExecutor>,
}
/// Global context the engine hands to a layer's `background` call.
///
/// Same storage/index handles as `LayerContext`, but deliberately without a
/// `Query`: background activity is decoupled from any single request and
/// must decide its own scope and priority (e.g. from what previous queries
/// touched, or from `shared_index` changes) without being told what to do.
pub struct BackgroundContext {
    pub storage: Arc<LayerStorage>,
    pub shared_index: Arc<SharedIndex>,
    /// Shared rayon-backed executor for CPU-bound work; see
    /// [`CpuExecutor::spawn`] to offload work without blocking the async
    /// runtime.
    pub cpu: Arc<CpuExecutor>,
}
/// Base trait every concrete RAG layer implements.
///
/// A layer is a fully independent signal producer. It owns its own
/// chunking, model choice, index/cache format and query-matching logic;
/// the only things it shares with other layers are: the field names on
/// `ResultEntry` it reads/writes (implicit aggregation), the persistence
/// primitives exposed through the context structs, and this contract.
///
/// Implementing a new layer means: pick a stable, unique `id`; pick a
/// `stage`/`depends_on` reflecting where it sits relative to other layers
/// that may or may not be present at runtime; override `applies` only if
/// activation needs more than "query has a field named like my id";
/// implement `run` to read/write `ResultEntry` fields and return a
/// `LayerStatus`; optionally implement `background` for lazy building.
/// Nothing else is required, and nothing else should be assumed about the
/// runtime (no central scheduler API, no fixed pipeline order) beyond what
/// is documented here.
#[async_trait]
pub trait Layer: Send + Sync {
    /// Stable, globally unique layer identity. Used for: registry lookup,
    /// `LayerStorage` namespace, and the default query-activation field
    /// name. Never reuse an id for a semantically different layer; version
    /// internally via `LayerStorage` metadata instead.
    fn id(&self) -> &str;
    /// Topology hint; see [`LayerStage`]. Affects default scheduling
    /// relative to other layers, not correctness - correctness must not
    /// depend on stage ordering alone when explicit ordering matters (use
    /// `depends_on` for that).
    fn stage(&self) -> LayerStage {
        LayerStage::Enrich
    }
    /// IDs of other layers whose contribution to the *current query run*
    /// must be complete before this layer's `run` is invoked. Layers
    /// without (mutual) dependencies may be scheduled fully in parallel
    /// (fork); the engine joins dependents after their dependencies
    /// finish. Empty by default for layers that can run at any point
    /// relative to others.
    fn depends_on(&self) -> HashSet<String> {
        HashSet::new()
    }
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
    async fn run(
        &self,
        query: &Query,
        result_set: &ResultSet,
        ctx: &LayerContext,
    ) -> LayerStatus;
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
    async fn background(&self, _ctx: &BackgroundContext, _cancel: CancellationToken) {}
}
