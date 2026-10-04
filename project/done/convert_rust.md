Konvertiere das Projekt in `/home/user/xyan/xy.ai.workbench/rag` von Python zu Rust. Manifest und Konzept sollten dir einen guten Einblick geben. Eine Rust Umgebung ist bereits installiert.

## Konzeptdetails

- Manifest: `/home/user/xyan/xy.ai.workbench/docs/layered_anytime.md`
- Hauptpackage ist "xy.ai.rag"
- Die RAG Engine wird sowohl als Lib eingebunden als sie auch als CLI Utility on demand gestartet wird (kein deamon oder Serverprozess)
- Layer sollen steuerbar sequentiell laufen als auch parallel(performance; Ein Fork/Join Modell) laufen können.
- Dementsprechend wird zwar ein gemeinsames Result Set Objekt notwendig aber die Composition ist Multi Threaded und Variabel. Der erste Layer legt ein Result an, auch wenn es ein Enrichment Layer ist 
	- Das bedeutet die Topologie unterstützt sowohl eine Sequenz in der ein Layer auf ein Result für ein enrichment Observed als auch Layer die vollständig autonom sind als auch Layer die auf ein vollständiges Result Objekt warten für ein Postprocessing.
- Signale laufen über Felder eines Result Entries. Ein Result ist Dabei frei und nicht stark typisiert. Es kann einen Dateinamen beinhalten oder nur eine AST Knoten ID, Zeilennummern oder nur eine Zusammenfassung.
	- Aggregation passiert impliziert, indem Layer auf gemeinsamen Feldern operieren
	- Beispielweise findet eine Grep-artige Textsuche Zeilen in Dateinamen, ein semantischer Layer könnte Zeile aber auf auf einen Textausschnitt ersetzen oder verkleinern während ein Code AST Layer eine Outline und FQND hinzufügt
- Konkrete Layer werden später implementiert
- Weitere Metadaten wie Dateigröße oder Datum werden von weiteren Layern hinzugefügt
- Eine Query besteht aus einem aus einem schwach typisierten Dynamischen Objekt. Es könnte include oder exclude Parameter enthalten oder auch nicht. Eine Suche nach einer Funktion oder einem Text. Die Layer bestimmen wie sie auf welche Felder reagieren.
	- Ein Layer kann durch das Vorhandensein eines Feldes in der Query aktiviert werden oder aber ein Layer inspiziert und analysiert die Query um sich zu aktivieren.
- Ein numerisches Ranking wird nicht durchgeführt aber es gibt eine stabile Sortierung
	- Layer agieren hier für Postprocessing
- 2-Stage Approach wird als Layer umgesetzt. Ein Cache-Layer wartet auf ein Result Set das von bestimmten Layern abgeschlossen wurde, cached das ganze Objekt, reduziert dann den Informationsgehalt und fügt ID's ein.
	- Ein Cache Retriever Layer, reagiert auf das Vorhandensein von ID Parametern und lädt gecachte Einträge per ID in das Result Objekt. Danach können dennoch Layer folgen oder nicht.
- Die ganze RAG Engine ähnelt also einem sich dynamisch aufbauenden und anpassenden DAG
- Wann Layer Hintergrundprozesse aktivieren oder nicht entscheiden sie selbst auf Basis der Query
	- Ein CLI Prozess beendet sich, wenn alle Queries returned wurden und wenn alle Hintergrundaktivitäten aller Layer beendet sind.
	- Ein Layer hat also zwei mögliche Kontrollflüsse 1. einen gekoppelt an einen Channel von Query Objekten, 2. einen globalen Kontrollfluss für Background activities.
	- Layer entscheiden selbst über ihr Verhalten oder Priorisierung beim Eintreffen neuer Queries
- Ein zentraler auf Disk persistierter Index ermöglicht geteilte Change Detection
	- Der Cache ist dabei unspezifisch. Ein Layer kann Einträge für Chunks, Zeilen oder Dateinamen oder Dateipfade anlegen, auch Composite Objekte sind möglich die Pfade, Dateien, Chunks, AST Knoten oder Zeilen korrelieren. Die Unterstützung hierfür ist Sache der Layer
	- Layer können also Resourcen teilen, müssen jedoch nicht
- Ein Layer wird durch eine eindeutige ID identifiziert. Eine Registry verwaltet alle Layer Implementierungen
- Die gemeinsame Persistenz wird unter einem Subfolder ".xyrag" gehalten und liegt im CWD oder wird mittels Argument übergeben
	- Layer erhalten entsprechend ein abstrahiertes Interface für die Verwaltung ihrer Persistenzeinheiten
SystemInit:  id=527e28d1-2555-420b-935b-6e96da48f2f6, cwd=/home/user/xyan/xy.ai.workbench/project, model=claude-sonnet-5
Control Request:
```yaml
id: 8b5f-1
toolName: read_file
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/docs/layered_anytime.md
  reason: Konzept lesen für Rust Konvertierung
```
/answer 8b5f-1 allow
Control Request:
```yaml
id: 8b5f-2
toolName: list
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag
  reason: Projektstruktur des Python-Projekts einsehen
```
/answer 8b5f-2 allow
Control Request:
```yaml
id: 8b5f-3
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag
      entries:
      - ".:"
      - .gitignore
      - .project
      - xyrag.sh
      - "./.cargo:"
      - config.toml
      - "./.settings:"
      - org.eclipse.core.resources.prefs
      - "./src/xy/ai/rag:"
      - __init__.py
      - __main__.py
      - cli.py
      - "./src/xy/ai/rag/core:"
      - __init__.py
      - engine.py
      - layer.py
      - persistence.py
      - query.py
      - registry.py
      - result.py
      - topology.py
      - "./src/xy/ai/rag/layers:"
      - __init__.py
```
/answer 8b5f-3 allow
ReasoningToken: 21
Control Request:
```yaml
id: 8b5f-4
toolName: read_file
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/.project
  - path: /home/user/xyan/xy.ai.workbench/rag/xyrag.sh
  - path: /home/user/xyan/xy.ai.workbench/rag/.cargo/config.toml
  - path: /home/user/xyan/xy.ai.workbench/rag/.gitignore
  - path: /home/user/xyan/xy.ai.workbench/rag/src/xy/ai/rag/__init__.py
  - path: /home/user/xyan/xy.ai.workbench/rag/src/xy/ai/rag/__main__.py
  - path: /home/user/xyan/xy.ai.workbench/rag/src/xy/ai/rag/cli.py
  - path: /home/user/xyan/xy.ai.workbench/rag/src/xy/ai/rag/core/__init__.py
  - path: /home/user/xyan/xy.ai.workbench/rag/src/xy/ai/rag/core/engine.py
  - path: /home/user/xyan/xy.ai.workbench/rag/src/xy/ai/rag/core/layer.py
  - path: /home/user/xyan/xy.ai.workbench/rag/src/xy/ai/rag/core/persistence.py
  - path: /home/user/xyan/xy.ai.workbench/rag/src/xy/ai/rag/core/query.py
  - path: /home/user/xyan/xy.ai.workbench/rag/src/xy/ai/rag/core/registry.py
  - path: /home/user/xyan/xy.ai.workbench/rag/src/xy/ai/rag/core/result.py
  - path: /home/user/xyan/xy.ai.workbench/rag/src/xy/ai/rag/core/topology.py
  - path: /home/user/xyan/xy.ai.workbench/rag/src/xy/ai/rag/layers/__init__.py
  reason: Vollständigen Python-Code und Projektkonfiguration lesen zur Konvertierung
```
/answer 8b5f-4 allow
ReasoningToken: 49
Control Request:
```yaml
id: 8b5f-5
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench/rag
  script: rm -rf src .project.bak 2>/dev/null; rustc --version; cargo --version; ls -la
  reason: "Alten Python-Quellbaum entfernen, Rust-Toolchain prüfen"
```
/answer 8b5f-5 allow
Control Request:
```yaml
id: 8b5f-6
result:
  structuredContent:
    stdout: |
      rustc 1.99.0 (b940084d7 2026-09-28)
      cargo 1.99.0 (5f94df478 2026-08-27)
      total 32
      drwxrwxr-x  5 user user 4096 Oct  4 08:18 .
      drwxrwxr-x 22 user user 4096 Oct  2 08:25 ..
      drwxrwxr-x  2 user user 4096 Oct  4 08:04 .bin
      drwxrwxr-x  2 user user 4096 Oct  4 08:04 .cargo
      -rw-rw-r--  1 user user   30 Oct  4 08:09 .gitignore
      -rw-rw-r--  1 user user  474 Oct  4 08:08 .project
      drwxrwxr-x  2 user user 4096 Oct  2 08:25 .settings
      -rwxr-xr-x  1 user user 1326 Oct  2 21:35 xyrag.sh
```
/answer 8b5f-6 allow
ReasoningToken: 14185
Text:
 Jetzt erstelle ich die Rust-Projektstruktur.
Control Request:
```yaml
id: 8b5f-7
toolName: ast_create
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/Cargo.toml
    source: |
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
  - path: /home/user/xyan/xy.ai.workbench/rag/src/lib.rs
    source: |
      //! xy.ai.rag - Layered Anytime Retrieval Engine.
      //!
      //! Used both as a library (`xy_ai_rag::core::...`) and as an on-demand CLI
      //! utility (`xyrag`, see `src/bin/xyrag.rs`) - no daemon or server process.

      pub mod core;
      pub mod layers;

      pub const VERSION: &str = env!("CARGO_PKG_VERSION");
  - path: /home/user/xyan/xy.ai.workbench/rag/src/core/mod.rs
    source: |
      //! Core building blocks of the RAG engine: query, result set, layer
      //! contract, engine (orchestrator) and persistence.

      pub mod engine;
      pub mod layer;
      pub mod persistence;
      pub mod query;
      pub mod registry;
      pub mod result;
      pub mod topology;
  - path: /home/user/xyan/xy.ai.workbench/rag/src/layers/mod.rs
    source: |
      //! Concrete layer implementations (grep, BM25, AST, cache, ...).
      //!
      //! Still empty - concrete layers are added here later and registered via
      //! [`crate::core::registry::LayerRegistry`].
  reason: "Rust-Projekt-Grundgerüst anlegen (Cargo.toml, lib.rs, Modulbäume)"
```
/answer 8b5f-7 allow
Text:
 Jetzt die Core-Module: query, result, layer, registry, persistence, topology, engine.
Control Request:
```yaml
id: 8b5f-8
toolName: ast_create
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/core/query.rs
    source: |
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
  - path: /home/user/xyan/xy.ai.workbench/rag/src/core/result.rs
    source: |
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
          signals: Vec<String>,
      }

      /// A single, weakly typed entry in the result set.
      #[derive(Debug)]
      pub struct ResultEntry {
          pub id: String,
          inner: Mutex<EntryInner>,
      }

      impl ResultEntry {
          pub fn new(
              entry_id: Option<String>,
              fields: Map<String, Value>,
              signals: Vec<String>,
          ) -> Arc<Self> {
              Arc::new(Self {
                  id: entry_id.unwrap_or_else(next_entry_id),
                  inner: Mutex::new(EntryInner { fields, signals }),
              })
          }

          /// Extends/overwrites fields and appends signals (thread-safe).
          ///
          /// An enrichment layer calls this on an already existing entry, e.g. to
          /// replace "line" with a text excerpt or to add an AST outline/FQN.
          pub fn merge(&self, fields: Option<Map<String, Value>>, signals: Option<Vec<String>>) {
              let mut guard = self.inner.lock().unwrap();
              if let Some(f) = fields {
                  for (k, v) in f {
                      guard.fields.insert(k, v);
                  }
              }
              for s in signals.into_iter().flatten() {
                  if !guard.signals.contains(&s) {
                      guard.signals.push(s);
                  }
              }
          }

          pub fn has(&self, field: &str) -> bool {
              let guard = self.inner.lock().unwrap();
              matches!(guard.fields.get(field), Some(v) if !v.is_null())
          }

          pub fn get(&self, field: &str) -> Option<Value> {
              let guard = self.inner.lock().unwrap();
              guard.fields.get(field).cloned()
          }

          pub fn signals(&self) -> Vec<String> {
              self.inner.lock().unwrap().signals.clone()
          }

          pub fn to_dict(&self) -> Map<String, Value> {
              let guard = self.inner.lock().unwrap();
              let mut out = Map::new();
              out.insert("id".into(), Value::String(self.id.clone()));
              out.insert(
                  "signals".into(),
                  Value::Array(guard.signals.iter().cloned().map(Value::String).collect()),
              );
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
      /// entries (generate) or enrich existing ones, by operating on shared
      /// fields.
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
          /// them with further fields/signals.
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
  reason: Query- und ResultSet-Module des Rust-Kerns anlegen
```
/answer 8b5f-8 allow
Text:
 Jetzt der Layer-Contract mit ausführlicher Dokumentation (das Herzstück des Konzepts):
Control Request:
```yaml
id: 8b5f-9
toolName: ast_create
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/core/layer.rs
    source: |
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
      //! in the current working directory or at an explicitly supplied root path
      //! (see [`crate::core::persistence::resolve_root`]). Two kinds of storage
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
          async fn run(&self, query: &Query, result_set: &ResultSet, ctx: &LayerContext) -> LayerStatus;

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
      }
  reason: Layer-Contract (Kernkonzept) als Rust-Trait mit vollständiger Dokumentation anlegen
```
/answer 8b5f-9 allow
Control Request:
```yaml
id: 8b5f-a
toolName: ast_create
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/core/registry.rs
    source: |
      //! Registry for layer implementations.

      use std::sync::Arc;

      use anyhow::{bail, Result};

      use crate::core::layer::Layer;

      /// Manages all available layer instances, addressed by their id.
      #[derive(Default)]
      pub struct LayerRegistry {
          layers: Vec<Arc<dyn Layer>>,
      }

      impl LayerRegistry {
          pub fn new() -> Self {
              Self { layers: Vec::new() }
          }

          pub fn register(&mut self, layer: Arc<dyn Layer>) -> Result<()> {
              if layer.id().is_empty() {
                  bail!("Layer benötigt eine eindeutige, nicht-leere id");
              }
              if self.layers.iter().any(|l| l.id() == layer.id()) {
                  bail!("Layer-ID bereits registriert: {}", layer.id());
              }
              self.layers.push(layer);
              Ok(())
          }

          pub fn unregister(&mut self, layer_id: &str) {
              self.layers.retain(|l| l.id() != layer_id);
          }

          pub fn get(&self, layer_id: &str) -> Option<Arc<dyn Layer>> {
              self.layers.iter().find(|l| l.id() == layer_id).cloned()
          }

          pub fn all(&self) -> Vec<Arc<dyn Layer>> {
              self.layers.clone()
          }

          pub fn iter(&self) -> impl Iterator<Item = &Arc<dyn Layer>> {
              self.layers.iter()
          }

          pub fn len(&self) -> usize {
              self.layers.len()
          }

          pub fn is_empty(&self) -> bool {
              self.layers.is_empty()
          }

          pub fn contains(&self, layer_id: &str) -> bool {
              self.layers.iter().any(|l| l.id() == layer_id)
          }
      }
  - path: /home/user/xyan/xy.ai.workbench/rag/src/core/topology.rs
    source: |
      //! Dynamic topology (DAG) over the active layers of a query.
      //!
      //! Supports both purely sequential execution and a fork/join model for
      //! parallel execution of independent layers. The topology is derived anew
      //! per query from the active layers (`Layer::applies`) and their explicit
      //! dependencies (`Layer::depends_on`), complemented by the coarse stage
      //! ordering (Generate before Enrich before Postprocess) as a default edge
      //! when no explicit dependencies exist.

      use std::collections::{HashMap, HashSet};
      use std::fmt;
      use std::sync::Arc;

      use crate::core::layer::{Layer, LayerStage};

      #[derive(Debug)]
      pub struct CyclicDependencyError(pub String);

      impl fmt::Display for CyclicDependencyError {
          fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
              write!(f, "Zyklische Abhängigkeiten unter Layern: {}", self.0)
          }
      }

      impl std::error::Error for CyclicDependencyError {}

      fn stage_order(stage: LayerStage) -> u8 {
          match stage {
              LayerStage::Generate => 0,
              LayerStage::Enrich => 1,
              LayerStage::Postprocess => 2,
          }
      }

      /// Topological sort into levels ("fork/join" groups).
      ///
      /// Each level contains layers that can run in parallel, because all their
      /// dependencies are already satisfied by previous levels. For sequential
      /// execution the levels are simply processed one after another, and within
      /// each level one layer at a time.
      pub fn build_levels(
          layers: &[Arc<dyn Layer>],
      ) -> Result<Vec<Vec<Arc<dyn Layer>>>, CyclicDependencyError> {
          let by_id: HashMap<String, Arc<dyn Layer>> = layers
              .iter()
              .map(|l| (l.id().to_string(), l.clone()))
              .collect();

          let mut deps: HashMap<String, HashSet<String>> = HashMap::new();
          for layer in layers {
              let mut d: HashSet<String> = layer
                  .depends_on()
                  .into_iter()
                  .filter(|dep| by_id.contains_key(dep))
                  .collect();
              for other in layers {
                  if other.id() != layer.id() && stage_order(other.stage()) < stage_order(layer.stage())
                  {
                      d.insert(other.id().to_string());
                  }
              }
              deps.insert(layer.id().to_string(), d);
          }

          let mut remaining = deps;
          let mut done: HashSet<String> = HashSet::new();
          let mut levels: Vec<Vec<Arc<dyn Layer>>> = Vec::new();

          while !remaining.is_empty() {
              let ready: Vec<String> = remaining
                  .iter()
                  .filter(|(_, d)| d.is_subset(&done))
                  .map(|(k, _)| k.clone())
                  .collect();
              if ready.is_empty() {
                  let mut names: Vec<String> = remaining.keys().cloned().collect();
                  names.sort();
                  return Err(CyclicDependencyError(format!("{:?}", names)));
              }
              levels.push(ready.iter().map(|id| by_id[id].clone()).collect());
              for id in &ready {
                  done.insert(id.clone());
                  remaining.remove(id);
              }
          }

          Ok(levels)
      }
  - path: /home/user/xyan/xy.ai.workbench/rag/src/core/persistence.rs
    source: |
      //! Shared, disk-persisted infrastructure under `.xyrag`.
      //!
      //! Contains the central shared index (change detection across layer
      //! boundaries) and an abstract interface through which every layer manages
      //! its own persistence units (cache entries for chunks, lines, files,
      //! composite objects, ...), without the engine prescribing the structure
      //! of these units.

      use std::collections::HashMap;
      use std::path::{Path, PathBuf};
      use std::sync::{Arc, Mutex};

      use anyhow::Result;
      use rusqlite::{params, Connection, OptionalExtension};
      use serde_json::Value;

      pub const PERSISTENCE_DIRNAME: &str = ".xyrag";

      /// Resolves the persistence root: explicit argument, or CWD/.xyrag.
      pub fn resolve_root(root: Option<&Path>) -> Result<PathBuf> {
          let base = match root {
              Some(p) => p.to_path_buf(),
              None => std::env::current_dir()?.join(PERSISTENCE_DIRNAME),
          };
          std::fs::create_dir_all(&base)?;
          Ok(base)
      }

      /// One row of the central file index.
      #[derive(Debug, Clone)]
      pub struct FileRecord {
          pub path: String,
          pub hash: Option<String>,
          pub size: Option<i64>,
          pub mtime: Option<f64>,
          pub seq: i64,
          pub deleted: bool,
      }

      /// Central, shared index for change detection across all layers.
      ///
      /// Minimal base: path, hash, size, mtime and a monotonic sequence number
      /// per entry; deletions as tombstones. Concrete layers use this as the
      /// shared source of "has something changed".
      pub struct SharedIndex {
          conn: Mutex<Connection>,
      }

      fn row_to_record(row: &rusqlite::Row<'_>) -> rusqlite::Result<FileRecord> {
          Ok(FileRecord {
              path: row.get(0)?,
              hash: row.get(1)?,
              size: row.get(2)?,
              mtime: row.get(3)?,
              seq: row.get(4)?,
              deleted: row.get::<_, i64>(5)? != 0,
          })
      }

      impl SharedIndex {
          pub fn open(db_path: &Path) -> Result<Self> {
              let conn = Connection::open(db_path)?;
              conn.execute_batch(
                  "PRAGMA journal_mode=WAL;
                   CREATE TABLE IF NOT EXISTS file_index (
                       path TEXT PRIMARY KEY,
                       hash TEXT,
                       size INTEGER,
                       mtime REAL,
                       seq INTEGER,
                       deleted INTEGER NOT NULL DEFAULT 0
                   );
                   CREATE TABLE IF NOT EXISTS seq_counter (name TEXT PRIMARY KEY, value INTEGER);",
              )?;
              Ok(Self {
                  conn: Mutex::new(conn),
              })
          }

          fn next_seq(conn: &Connection) -> rusqlite::Result<i64> {
              conn.execute(
                  "INSERT INTO seq_counter(name, value) VALUES ('global', 1)
                   ON CONFLICT(name) DO UPDATE SET value = value + 1",
                  [],
              )?;
              conn.query_row(
                  "SELECT value FROM seq_counter WHERE name = 'global'",
                  [],
                  |r| r.get(0),
              )
          }

          /// Creates/updates a file entry, returns the new sequence number.
          pub fn upsert(&self, path: &str, content_hash: &str, size: i64, mtime: f64) -> Result<i64> {
              let conn = self.conn.lock().unwrap();
              let seq = Self::next_seq(&conn)?;
              conn.execute(
                  "INSERT INTO file_index(path, hash, size, mtime, seq, deleted)
                   VALUES (?1, ?2, ?3, ?4, ?5, 0)
                   ON CONFLICT(path) DO UPDATE SET hash=?2, size=?3, mtime=?4, seq=?5, deleted=0",
                  params![path, content_hash, size, mtime, seq],
              )?;
              Ok(seq)
          }

          pub fn mark_deleted(&self, path: &str) -> Result<i64> {
              let conn = self.conn.lock().unwrap();
              let seq = Self::next_seq(&conn)?;
              conn.execute(
                  "INSERT INTO file_index(path, hash, size, mtime, seq, deleted)
                   VALUES (?1, NULL, NULL, NULL, ?2, 1)
                   ON CONFLICT(path) DO UPDATE SET deleted=1, seq=?2",
                  params![path, seq],
              )?;
              Ok(seq)
          }

          pub fn get(&self, path: &str) -> Result<Option<FileRecord>> {
              let conn = self.conn.lock().unwrap();
              let rec = conn
                  .query_row(
                      "SELECT path, hash, size, mtime, seq, deleted FROM file_index WHERE path = ?1",
                      params![path],
                      row_to_record,
                  )
                  .optional()?;
              Ok(rec)
          }

          /// Returns all changes since `seq` - basis for the per-layer cursor.
          pub fn changes_since(&self, seq: i64) -> Result<Vec<FileRecord>> {
              let conn = self.conn.lock().unwrap();
              let mut stmt = conn.prepare(
                  "SELECT path, hash, size, mtime, seq, deleted FROM file_index
                   WHERE seq > ?1 ORDER BY seq ASC",
              )?;
              let rows = stmt.query_map(params![seq], row_to_record)?;
              Ok(rows.collect::<rusqlite::Result<Vec<_>>>()?)
          }
      }

      /// Abstracted persistence interface for a single layer.
      ///
      /// Unspecific key/value cache (JSON-serialized) per layer, plus its own
      /// file-system namespace for sidecars (vectors, index files, ...). Layers
      /// decide for themselves what structure their ids have (chunk id, line
      /// range, file path, composite key) and whether/how they share resources
      /// with other layers (e.g. via the same key scheme).
      pub struct LayerStorage {
          pub layer_id: String,
          pub dir: PathBuf,
          conn: Mutex<Connection>,
      }

      impl LayerStorage {
          pub fn open(layer_id: &str, db_path: &Path, dir_path: &Path) -> Result<Self> {
              std::fs::create_dir_all(dir_path)?;
              let conn = Connection::open(db_path)?;
              conn.execute_batch(
                  "PRAGMA journal_mode=WAL;
                   CREATE TABLE IF NOT EXISTS cache (key TEXT PRIMARY KEY, value TEXT);
                   CREATE TABLE IF NOT EXISTS layer_meta (name TEXT PRIMARY KEY, value TEXT);",
              )?;
              Ok(Self {
                  layer_id: layer_id.to_string(),
                  dir: dir_path.to_path_buf(),
                  conn: Mutex::new(conn),
              })
          }

          pub fn put(&self, key: &str, value: &Value) -> Result<()> {
              let json = serde_json::to_string(value)?;
              let conn = self.conn.lock().unwrap();
              conn.execute(
                  "INSERT INTO cache(key, value) VALUES (?1, ?2)
                   ON CONFLICT(key) DO UPDATE SET value=?2",
                  params![key, json],
              )?;
              Ok(())
          }

          pub fn get(&self, key: &str) -> Result<Option<Value>> {
              let conn = self.conn.lock().unwrap();
              let raw: Option<String> = conn
                  .query_row("SELECT value FROM cache WHERE key = ?1", params![key], |r| {
                      r.get(0)
                  })
                  .optional()?;
              Ok(match raw {
                  Some(s) => Some(serde_json::from_str(&s)?),
                  None => None,
              })
          }

          pub fn delete(&self, key: &str) -> Result<()> {
              let conn = self.conn.lock().unwrap();
              conn.execute("DELETE FROM cache WHERE key = ?1", params![key])?;
              Ok(())
          }

          pub fn keys(&self, prefix: &str) -> Result<Vec<String>> {
              let conn = self.conn.lock().unwrap();
              let pattern = format!("{}%", prefix);
              let mut stmt = conn.prepare("SELECT key FROM cache WHERE key LIKE ?1 ORDER BY key")?;
              let rows = stmt.query_map(params![pattern], |r| r.get(0))?;
              Ok(rows.collect::<rusqlite::Result<Vec<String>>>()?)
          }

          /// Path for layer-owned sidecar files (e.g. vector sidecars).
          pub fn path_for(&self, name: &str) -> PathBuf {
              self.dir.join(name)
          }

          // ---- Per-layer cursor over the SharedIndex (change detection) ----

          pub fn get_cursor(&self) -> Result<i64> {
              let conn = self.conn.lock().unwrap();
              let raw: Option<String> = conn
                  .query_row(
                      "SELECT value FROM layer_meta WHERE name = 'cursor'",
                      [],
                      |r| r.get(0),
                  )
                  .optional()?;
              Ok(raw.and_then(|s| s.parse().ok()).unwrap_or(0))
          }

          pub fn set_cursor(&self, seq: i64) -> Result<()> {
              let conn = self.conn.lock().unwrap();
              conn.execute(
                  "INSERT INTO layer_meta(name, value) VALUES ('cursor', ?1)
                   ON CONFLICT(name) DO UPDATE SET value=?1",
                  params![seq.to_string()],
              )?;
              Ok(())
          }
      }

      /// Manages the `.xyrag` directory and hands out their storage to layers.
      pub struct PersistenceManager {
          pub root: PathBuf,
          pub shared_index: Arc<SharedIndex>,
          layer_storages: Mutex<HashMap<String, Arc<LayerStorage>>>,
      }

      impl PersistenceManager {
          pub fn new(root: Option<&Path>) -> Result<Self> {
              let root = resolve_root(root)?;
              let shared_index = Arc::new(SharedIndex::open(&root.join("index.db"))?);
              Ok(Self {
                  root,
                  shared_index,
                  layer_storages: Mutex::new(HashMap::new()),
              })
          }

          pub fn layer_storage(&self, layer_id: &str) -> Result<Arc<LayerStorage>> {
              let mut map = self.layer_storages.lock().unwrap();
              if let Some(s) = map.get(layer_id) {
                  return Ok(s.clone());
              }
              let layer_dir = self.root.join("layers").join(layer_id);
              let storage = Arc::new(LayerStorage::open(
                  layer_id,
                  &layer_dir.join("cache.db"),
                  &layer_dir,
              )?);
              map.insert(layer_id.to_string(), storage.clone());
              Ok(storage)
          }
      }
  reason: "Registry-, Topology- und Persistence-Module anlegen"
```
/answer 8b5f-a allow
Control Request:
```yaml
id: 8b5f-b
toolName: ast_create
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/core/engine.rs
    source: |
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
          cancel_tokens: HashMap<String, CancellationToken>,
          background_tasks: HashMap<String, JoinHandle<()>>,
      }

      impl Engine {
          pub fn new(registry: LayerRegistry, persistence: PersistenceManager, mode: ExecutionMode) -> Self {
              Self {
                  registry,
                  persistence: Arc::new(persistence),
                  mode,
                  cancel_tokens: HashMap::new(),
                  background_tasks: HashMap::new(),
              }
          }

          // ---- Query-channel control flow ------------------------------------

          pub fn active_layers(&self, query: &Query) -> Vec<Arc<dyn Layer>> {
              self.registry
                  .iter()
                  .filter(|l| l.applies(query))
                  .cloned()
                  .collect()
          }

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

          // ---- Global background control flow --------------------------------

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
  - path: /home/user/xyan/xy.ai.workbench/rag/src/bin/xyrag.rs
    source: |
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

      #[derive(Parser, Debug)]
      #[command(name = "xyrag", about = "xy.ai.rag - Layered Anytime Retrieval Engine")]
      struct Cli {
          /// Query fields as key=value
          query: Vec<String>,

          /// Query as a JSON object
          #[arg(long = "json")]
          json_query: Option<String>,

          /// Persistence root (default: ./.xyrag)
          #[arg(long)]
          root: Option<String>,

          /// Execution model of the layer topology
          #[arg(long, default_value = "parallel")]
          mode: String,
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
                  .ok_or_else(|| anyhow!("Ungültiges Query-Feld (erwartet key=value): {item}"))?;
              fields.insert(key.to_string(), Value::String(value.to_string()));
          }
          Ok(Query::from_fields(fields))
      }

      /// Builds the registry with all known layer implementations.
      ///
      /// Concrete layers are added here later.
      fn build_default_registry() -> LayerRegistry {
          LayerRegistry::new()
      }

      fn status_to_json(s: &LayerStatus) -> Value {
          let stage = match s.stage {
              xy_ai_rag::core::layer::LayerStage::Generate => "generate",
              xy_ai_rag::core::layer::LayerStage::Enrich => "enrich",
              xy_ai_rag::core::layer::LayerStage::Postprocess => "postprocess",
          };
          json!({
              "layer_id": s.layer_id,
              "stage": stage,
              "ran": s.ran,
              "skipped": s.skipped,
              "aborted": s.aborted,
              "contributions": s.contributions,
              "detail": s.detail,
          })
      }

      #[tokio::main]
      async fn main() -> Result<()> {
          let cli = Cli::parse();
          let mode = ExecutionMode::parse(&cli.mode)
              .ok_or_else(|| anyhow!("Ungültiger Modus: {}", cli.mode))?;
          let query = parse_query(&cli.query, cli.json_query.as_deref())?;

          let registry = build_default_registry();
          let persistence = PersistenceManager::new(cli.root.as_deref().map(Path::new))?;
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
          let output = json!({ "results": results, "layers": layers });
          println!("{}", serde_json::to_string_pretty(&output)?);

          Ok(())
      }
  reason: Engine-Orchestrator und CLI-Binary anlegen
```
/answer 8b5f-b allow
Control Request:
```yaml
id: 8b5f-c
toolName: ast_create
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/xyrag.sh
    overwrite: true
    source: |
      #!/usr/bin/env bash
      # On-demand CLI start of the xy.ai.rag engine.
      #
      # Usage: xyrag.sh [--root PATH] <query...>
      #   --root PATH   Persistence root (.xyrag); default: current working directory (CWD).
      #   <query...>    Free text, packed as field "query" into the query object
      #                 and sent through the layer pipeline.
      set -euo pipefail

      SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

      root=""
      query_args=()

      usage() {
        echo "Usage: $(basename "$0") [--root PATH] <query...>" >&2
        exit 1
      }

      while [[ $# -gt 0 ]]; do
        case "$1" in
          --root)
            [[ $# -ge 2 ]] || usage
            root="$2"
            shift 2
            ;;
          --root=*)
            root="${1#--root=}"
            shift
            ;;
          -h|--help)
            usage
            ;;
          *)
            query_args+=("$1")
            shift
            ;;
        esac
      done

      [[ ${#query_args[@]} -gt 0 ]] || usage

      query_text="${query_args[*]}"

      cmd=(cargo run --quiet --manifest-path "${SCRIPT_DIR}/Cargo.toml" --bin xyrag -- "query=${query_text}")
      if [[ -n "$root" ]]; then
        cmd+=(--root "$root")
      fi

      exec "${cmd[@]}"
  reason: CLI-Startskript auf Cargo/Rust umstellen
```
/answer 8b5f-c allow
Control Request:
```yaml
id: 8b5f-d
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench/rag
  script: chmod +x xyrag.sh; cargo build 2>&1 | tail -100
  reason: Rust-Projekt bauen und Compilerfehler prüfen
```
/answer 8b5f-d allow
Control Request:
```yaml
id: 8b5f-e
result:
  structuredContent:
    stdout: |2
          Updating crates.io index
           Locking 65 packages to highest compatible versions
            Adding rusqlite v0.31.0 (available: v0.40.2)
       Downloading crates ...
        Downloaded clap_lex v1.1.1
        Downloaded clap_derive v4.6.7
        Downloaded clap_builder v4.6.7
        Downloaded hashlink v0.9.1
        Downloaded find-msvc-tools v0.1.14
        Downloaded smallvec v1.16.2
        Downloaded clap v4.6.7
        Downloaded unicode-ident v1.0.26
        Downloaded cc v1.6.0
        Downloaded rusqlite v0.31.0
        Downloaded zerocopy v0.8.59
        Downloaded syn v3.0.6
        Downloaded tokio v1.53.2
        Downloaded libsqlite3-sys v0.28.0
         Compiling proc-macro2 v1.0.107
         Compiling quote v1.0.47
         Compiling unicode-ident v1.0.26
         Compiling zerocopy v0.8.59
         Compiling version_check v0.9.5
         Compiling futures-sink v0.3.34
         Compiling futures-core v0.3.34
         Compiling find-msvc-tools v0.1.14
         Compiling pin-project-lite v0.2.17
         Compiling shlex v2.0.1
         Compiling serde_core v1.0.229
         Compiling vcpkg v0.2.15
         Compiling pkg-config v0.3.34
         Compiling cfg-if v1.0.5
         Compiling utf8parse v0.2.2
         Compiling memchr v2.8.3
         Compiling once_cell v1.21.4
         Compiling anstyle-parse v1.0.0
         Compiling colorchoice v1.0.5
         Compiling futures-channel v0.3.34
         Compiling anstyle v1.0.14
         Compiling anstyle-query v1.1.5
         Compiling futures-task v0.3.34
         Compiling slab v0.4.12
         Compiling cc v1.6.0
         Compiling zmij v1.0.23
         Compiling futures-io v0.3.34
         Compiling is_terminal_polyfill v1.70.2
         Compiling serde_json v1.0.151
         Compiling heck v0.5.0
         Compiling ahash v0.8.12
         Compiling anstream v1.0.0
         Compiling strsim v0.11.1
         Compiling serde v1.0.229
         Compiling clap_lex v1.1.1
         Compiling anyhow v1.0.104
         Compiling fallible-iterator v0.3.0
         Compiling smallvec v1.16.2
         Compiling itoa v1.0.18
         Compiling clap_builder v4.6.7
         Compiling bitflags v2.13.2
         Compiling bytes v1.12.1
         Compiling fallible-streaming-iterator v0.1.9
         Compiling syn v3.0.6
         Compiling libsqlite3-sys v0.28.0
         Compiling futures-macro v0.3.34
         Compiling tokio-macros v2.7.2
         Compiling clap_derive v4.6.7
         Compiling serde_derive v1.0.229
         Compiling async-trait v0.1.92
         Compiling tokio v1.53.2
         Compiling hashbrown v0.14.5
         Compiling futures-util v0.3.34
         Compiling hashlink v0.9.1
         Compiling clap v4.6.7
         Compiling tokio-util v0.7.19
         Compiling futures-executor v0.3.34
         Compiling futures v0.3.34
         Compiling rusqlite v0.31.0
         Compiling xy-ai-rag v0.1.0 (/home/user/xyan/xy.ai.workbench/rag)
          Finished `dev` profile [unoptimized + debuginfo] target(s) in 12.65s
```
/answer 8b5f-e allow
Control Request:
```yaml
id: 8b5f-f
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench/rag
  script: |-
    rm -rf /tmp/xyrag-test && mkdir /tmp/xyrag-test && cd /tmp/xyrag-test && /home/user/xyan/xy.ai.workbench/rag/xyrag.sh "hallo welt" && ls -la .xyrag && cat .xyrag/index.db >/dev/null && echo OK
    cd /home/user/xyan/xy.ai.workbench/rag && cargo clippy --quiet 2>&1 | tail -60
  reason: CLI-Ausführung testen und Lints prüfen
```
/answer 8b5f-f allow
Text:
 Jetzt Registry, Persistence und Topology:
Control Request:
```yaml
id: 8b5f-10
result:
  structuredContent:
    stdout: |+
      {
        "layers": [],
        "results": []
      }
      total 28
      drwxrwxr-x 2 user user  4096 Oct  4 08:22 .
      drwxrwxr-x 3 user user  4096 Oct  4 08:22 ..
      -rw-r--r-- 1 user user 20480 Oct  4 08:22 index.db
      OK
      warning: doc list item without indentation
        --> src/core/result.rs:96:5
         |
      96 | /// entries (generate) or enrich existing ones, by operating on shared
         |     ^
         |
         = help: if this is supposed to be its own paragraph, add a blank line
         = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#doc_lazy_continuation
         = note: `#[warn(clippy::doc_lazy_continuation)]` on by default
      help: indent this line
         |
      96 | ///   entries (generate) or enrich existing ones, by operating on shared
         |     ++

      warning: doc list item without indentation
        --> src/core/result.rs:97:5
         |
      97 | /// fields.
         |     ^
         |
         = help: if this is supposed to be its own paragraph, add a blank line
         = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#doc_lazy_continuation
      help: indent this line
         |
      97 | ///   fields.
         |     ++
```
/answer 8b5f-10 allow
Control Request:
```yaml
id: 8b5f-11
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/core/result.rs
    old_text: |-
      /// entries (generate) or enrich existing ones, by operating on shared
      /// fields.
    new_text: |-
      ///   entries (generate) or enrich existing ones, by operating on shared
      ///   fields.
  reason: Clippy-Doc-Lint zur Einrückung des Listenelements beheben
```
/answer 8b5f-11 allow
Control Request:
```yaml
id: 8b5f-12
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench/rag
  script: cargo clippy --quiet 2>&1 | tail -40
  reason: Lint erneut prüfen nach Korrektur
```
/answer 8b5f-12 allow
Control Request:
```yaml
id: 8b5f-13
result:
  structuredContent:
    result: success
```
/answer 8b5f-13 allow
Result Stats: id=527e28d1-2555-420b-935b-6e96da48f2f6, total: 120271, in: 159, out: 39740, reason: 0, read: 764902, write: 80372
Text:
 Jetzt die Engine (Orchestrator) und die CLI:
Text:
 Jetzt xyrag.sh anpassen und build/test durchführen.
Text:
 Build erfolgreich. Jetzt kurzer Funktionstest der CLI:
Text:
 CLI funktioniert, nur Doc-Lint-Hinweise (keine Fehler). Behebe die Einrückung schnell:
Clippy ist clean, Build und CLI-Run funktionieren. Zusammenfassung der Konvertierung:

- `Cargo.toml` (Package `xy-ai-rag`, Lib `xy_ai_rag`, Bin `xyrag`)
- `src/lib.rs`, `src/core/mod.rs`, `src/layers/mod.rs`
- `src/core/query.rs` – dynamisches `Query` über `serde_json::Map`
- `src/core/result.rs` – `ResultEntry`/`ResultSet`, thread-sicher via `Mutex`
- `src/core/layer.rs` – `Layer`-Trait (async-trait), `LayerStage`, `LayerStatus`, `LayerContext`/`BackgroundContext`, vollständig dokumentiert
- `src/core/registry.rs` – `LayerRegistry`
- `src/core/topology.rs` – Fork/Join-Level-Aufbau (`build_levels`) mit Zyklenerkennung
- `src/core/persistence.rs` – `SharedIndex`/`LayerStorage`/`PersistenceManager` (rusqlite, `.xyrag`)
- `src/core/engine.rs` – `Engine`/`ExecutionMode` (sequentiell/parallel via `tokio`/`futures::join_all`), globaler Background-Kontrollfluss mit `CancellationToken`
- `src/bin/xyrag.rs` – CLI-Einstieg (clap), kein Daemon
- `xyrag.sh` auf `cargo run` umgestellt

Python-Quellbaum entfernt; `cargo build` und `cargo clippy` laufen fehlerfrei, CLI-Smoke-Test erzeugt `.xyrag/index.db` und liefert erwartetes leeres JSON-Ergebnis (noch keine konkreten Layer registriert, wie im Python-Original).
