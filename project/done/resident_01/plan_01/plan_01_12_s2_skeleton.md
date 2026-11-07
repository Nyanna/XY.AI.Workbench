# Stufe 2 — Layer-Gerüst, Backend-Naht und Testkit

Stufe 2 legt das Gerüst des Layers an und macht es testbar, ohne dass die Engine lauffähig sein muss: Modul und Registrierung, Konfiguration, Fehler- und Statusmodell, die Backend-Naht mit Produktions-Adapter und `FakeBackend` sowie das `Layer`-Skelett mit allen `skipped`-Pfaden. Aufwand [W]: 3 Personentage. Läuft parallel zu S1 und ist Voraussetzung für S3 und S4.

## Eingang

Gate G0 bestanden. Bestätigt sein müssen D1, D2, D3, D11, D14 und D17. Der Layer-Vertrag (`rag/src/core/layer.rs`) ist maßgeblich: „Read this documentation together with the trait/type docs below before writing a new layer; no other document should be necessary.“

## Zielstruktur

```
rag/src/layers/resident_layer/
  mod.rs        Moduldoku, pub use layer::ResidentLayer
  layer.rs      impl Layer for ResidentLayer, LayerState
  config.rs     ResidentLayerConfig, Auflösung, Validierung
  error.rs      ResidentLayerError, Statusabbildung
  backend.rs    SemanticBackend, BackendFactory, ResidentBackend, BackendError
  fake.rs       FakeBackend, FaultScript            (cfg(any(test, feature = "mock")))
  walk.rs       sichtbare Dateien, Eignung          (S3)
  text.rs       Textzugriff, OffsetMap              (S3)
  chunk.rs      Chunker                             (S3)
  catalog.rs    Katalog, EntryId-Schema             (S3)
  store.rs      IndexStore, Schreibsperre           (S3)
  indexer.rs    Pipeline, Treiber                   (S3)
  search.rs     Query-Pfad                          (S4)
  render.rs     Trefferabbildung                    (S4)
  test/         Unit-Tests (mod.rs, config.rs, ...)
rag/tests/
  resident_layer_e2e.rs      Ende zu Ende mit FakeBackend (cfg(feature = "mock"))
  resident_layer_filter.rs   globaler Include/Exclude-Filter (eigener Prozess)
  resident_layer_model.rs    modellgestützt (ignored ohne Modell)
  data/resident_corpus/, data/resident_gold.json
rag/benches/resident_layer_bench.rs
rag/docs/architecture/resident_layer.md
```

## Pakete

### L2.1 · Modulgerüst und Registrierung

**Zweck:** Der Layer existiert, ist registriert und beeinflusst bestehende Abläufe nicht.  
**Quellen:** `rag/src/layers/mod.rs`, `rag/src/layers/trigram/mod.rs` (Muster Verzeichnis-Layer), `rag/src/bin/xyrag.rs::build_default_registry`, `rag/src/core/registry.rs::LayerRegistry::register`.  
**Abhängt von:** L0.2 (D1, D2, D17).  
**Zu klären:** Position in der Registry (Reihenfolge ist für die Topologie unerheblich, der Layer ist `Generate`); Doku-Kommentar von `layers/mod.rs` aktualisieren.  
**Ergebnis:** `pub mod resident_layer;` in `layers/mod.rs`, `ResidentLayer::new()` in `build_default_registry` vor `MergeLayer`, leere Modulstruktur laut Zielstruktur.

#### Aufgaben

- Verzeichnis `rag/src/layers/resident_layer/` mit `mod.rs` anlegen (`//!`-Moduldoku nach Vorbild `trigram/mod.rs`, `pub use layer::ResidentLayer;`).
- In `layers/mod.rs` `pub mod resident_layer;` ergänzen.
- In `bin/xyrag.rs` nach `TrigramLayer` registrieren: `registry.register(Arc::new(ResidentLayer::new())).expect("failed to register ResidentLayer")`.
- Kein `cfg(feature)` einführen (D17); `use`-Pfade: Engine-Typen als `crate::resident::…`, Layer-Typen als `super::…`.

#### Abnahme L2.1

- `cargo build` ohne und mit `--features candle` ist grün.
- Test `registry_contains_resident_layer`: Die Default-Registry enthält die ID `resident` genau einmal.
- `xyrag query=x` verhält sich unverändert (Layer nicht aktiv).

### L2.2 · Konfiguration

**Zweck:** Alle Stellgrößen an einer Stelle, mit definierter Auflösungsreihenfolge und sicheren Defaults.  
**Quellen:** `planner/config.rs::PipelineConfig` (Muster `serde(default, deny_unknown_fields)`), `core/persistence.rs::LayerStorage::dir`, `toml`-Abhängigkeit (E48).  
**Abhängt von:** L2.1, L0.5 (Budgets), D14.  
**Zu klären:** Name und Ort der Konfigurationsdatei (`<storage.dir>/config.toml`), Umgebungsvariablen-Präfix `XYRAG_RESIDENT_`, Entwicklungs-Fallback für den Modellpfad (`CARGO_MANIFEST_DIR/model/multilingual-e5-small`, nur wenn vorhanden).  
**Ergebnis:** `ResidentLayerConfig` mit Sektionen und Validierung; `ResidentLayerConfig::resolve(query, storage_dir, env)`.

#### Skizze

```rust
#[derive(Clone, Debug, Default, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct ResidentLayerConfig {
    pub model: ModelSection,
    pub runtime: RuntimeSection,
    pub chunk: ChunkSection,
    pub index: IndexSection,
    pub query: QuerySection,
    pub maintenance: MaintenanceSection,
}

impl ResidentLayerConfig {
    pub fn resolve(
        query: &Query,
        storage_dir: &Path,
        env: &dyn Fn(&str) -> Option<String>,
    ) -> Result<Self, ResidentLayerError>;
    pub fn validate(&self) -> Result<(), ResidentLayerError>;
}
```

#### Auflösungsreihenfolge

| Rang | Quelle | Beispiel |
|---|---|---|
| 1 | Query-Felder (nur `resident*`, `directory`, `maxResults`) | `residentTopK=40`, `residentIndex=update` |
| 2 | Umgebung `XYRAG_RESIDENT_*` | `XYRAG_RESIDENT_MODEL_DIR=/opt/models/multilingual-e5-small` |
| 3 | Datei `<storage.dir>/config.toml` | `[chunk] target_chars = 1000` |
| 4 | Defaults (Tabelle unten) | — |
| 5 | Entwicklungs-Fallback Modellpfad | `CARGO_MANIFEST_DIR/model/multilingual-e5-small`, wenn `model.safetensors` existiert |

#### Defaults

| Schlüssel | Default [W] | Bedeutung |
|---|---|---|
| `model.dir` | keiner | Modellverzeichnis; ohne Wert und ohne Fallback meldet der Layer `skipped` (`model_unavailable`) |
| `runtime.cpu_workers` | 1 | Worker der Fassade (D15) |
| `runtime.task_timeout_ms` | 60000 | `WorkerConfig::with_task_timeout` |
| `runtime.idle_ttl_s` | 600 | TTL für Tokenizer und Encoder |
| `runtime.candle_threads` | modusabhängig | Override für die Encoder-Threads (D15, L5.3) |
| `chunk.target_chars` | 1000 | Zielgröße eines Chunks (D4) |
| `chunk.max_chars` | 1400 | harte Obergrenze |
| `chunk.min_chars` | 250 | Untergrenze, außer am Dateiende |
| `chunk.overlap_chars` | 150 | Überlappung zum Vorgänger |
| `chunk.context_header` | `path` | Kopfzeile im Passage-Text: `path` oder `none` |
| `index.max_file_bytes` | 1048576 | größere Dateien werden übersprungen |
| `index.max_chunks_per_file` | 1500 | Cap je Datei (D5: höchstens 2^20) |
| `index.max_chunks_total` | 500000 | Gesamt-Cap |
| `index.max_files_per_run` | 20000 | Obergrenze je Lauf, Rest in der nächsten Runde |
| `index.verify_on_open` | false | Prüfsummen beim Öffnen, nur in Wartungsmodi (K5) |
| `index.deny_globs` | Schlüssel- und Datenbankdateien | Namensmuster, die nie indexiert werden (L3.1) |
| `index.skip_extensions` | Bilder, Archive, Binärformate | Endungen, die nie indexiert werden (L3.1) |
| `index.flush_every_chunks` | 2000 | Flush-Politik (D6) |
| `index.flush_every_ms` | 30000 | Flush-Politik (D6) |
| `index.row_group_rows` | 8192 | Parquet-Row-Group |
| `query.default_max_results` | 20 | wenn `maxResults` fehlt |
| `query.top_k_factor` | 3 | `top_k = clamp(max_results * factor, max_results, 200)` |
| `query.min_score` | keiner | optional (D19) |
| `query.keep_stale` | false | D20 |
| `query.excerpt_lines` | 1 | Zeilen je Treffer-Excerpt |
| `query.excerpt_max_chars` | 200 | Zeilenlänge mit Mittenkürzung |
| `query.max_chunks_per_file` | 3 | Chunk-Treffer je Datei (L4.4) |
| `query.budget_ms` | 30000 | Gesamtbudget je Anfrage |
| `query.embed_budget_ms` | 20000 | inklusive Kaltstart |
| `query.search_budget_ms` | 5000 | Similarity-Task |
| `maintenance.auto_budget_ms` | 1500 | Modus `auto` (D9) |
| `maintenance.auto_max_chunks` | 32 | Modus `auto` |
| `maintenance.background_enabled` | false | D9 |
| `maintenance.background_interval_s` | 60 | Takt des Background-Laufs |
| `maintenance.background_budget_ms` | 5000 | Zeitscheibe je Takt |
| `precision` | keiner | optional, D18 |

#### Aufgaben

- Strukturen mit `serde(default, deny_unknown_fields)`; Validierung der Beziehungen (`min < target < max`, `overlap < min`, `max_chunks_per_file <= 2^20`, `flush_*` positiv, Budgets positiv, `top_k_factor >= 1`).
- `resolve` liest die Datei nur, wenn sie existiert; Parse-Fehler sind `ResidentLayerError::Config` mit Pfad und Zeile (`toml::de::Error`).
- Umgebung und Query-Felder als schmale Overrides (Whitelist der Schlüssel); unbekannte `resident*`-Felder werden ignoriert und in `detail.ignored_fields` gezählt.
- Funktions-Parameter statt globaler Zugriffe: `env` als Closure, damit Tests ohne `std::env::set_var` auskommen.

#### Abnahme L2.2

- Tests `defaults_validate`, `file_overrides_defaults`, `env_overrides_file`, `query_overrides_env`, `unknown_key_is_config_error`, `invalid_relations_are_rejected`, `dev_fallback_only_when_model_present`.
- Ungültige Konfiguration führt im Layer zu `skipped` mit `reason = config_invalid` (siehe L2.3), nie zu einem Fehler der Gesamtanfrage.

### L2.3 · Fehler- und Statusmodell

**Zweck:** Ein einheitliches Fehlermodell und stabile `status.detail`-Schlüssel; Abbildung der Engine-Fehler auf `LayerStatus`.  
**Quellen:** Konventionen Q1 (thiserror, `non_exhaustive`, typed fields, `IsRetryable`), `resident/error.rs`, `facade/error.rs::TaskError`, `core/layer.rs::LayerStatus`, Muster `status.detail` in grep und trigram (E9).  
**Abhängt von:** L2.1.  
**Zu klären:** Welche Fehlerklasse ist „skipped“ (nicht gelaufen) und welche „aborted“ (gelaufen, aber unterbrochen)? Welche Detail-Schlüssel gelten als stabile Schnittstelle?  
**Ergebnis:** `ResidentLayerError`, `BackendError`, `fn apply_error(status, err)`, Schlüsseltabelle (unten) und Grundtabelle (Fehler zu Status).

#### Skizze

```rust
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum ResidentLayerError {
    #[error("invalid configuration: {reason}")]
    Config { reason: String },
    #[error("backend unavailable: {reason}")]
    BackendUnavailable { reason: String },
    #[error(transparent)]
    Backend(#[from] BackendError),
    #[error(transparent)]
    Index(#[from] crate::resident::error::IndexError),
    #[error("i/o failure at {path}: {source}")]
    Io { path: PathBuf, source: std::io::Error },
    #[error("catalog inconsistent: {reason}")]
    Catalog { reason: String },
    #[error("index is locked by another writer")]
    Locked,
    #[error("budget exhausted in {phase}")]
    Budget { phase: &'static str },
}

pub fn apply_error(status: &mut LayerStatus, err: &ResidentLayerError);
```

#### Detail-Schlüssel

| Schlüssel | Typ | Bedeutung |
|---|---|---|
| `schema` | Zahl | Version des Detail-Schemas (1) |
| `backend` | Text | `resident-cpu` oder `fake` |
| `model` | Text | Modell-ID |
| `root` | Text | Suchwurzel |
| `index_mode` | Text | `auto`, `update`, `rebuild`, `status`, `off`, `read_only` |
| `reason` | Text | Grund bei `skipped` oder `aborted` (Grundtabelle) |
| `error` | Objekt | `{kind, retryable, message}` |
| `coverage` | Objekt | `{visible, indexed, stale, excluded_stale}` |
| `index` | Objekt | `{entries, files, rebuilt, flushed, files_new, files_changed, files_removed, chunks_written, remaining_files, elapsed_ms}` |
| `search` | Objekt | `{scope, top_k, candidates, discarded, outdated, hits, dropped_stale, dropped_invisible, s1, sk, gap_top2, z_top1}` |
| `timings_ms` | Objekt | `{walk, plan, index, embed, search, map, total}` |
| `dir_cache_misses`, `dir_cache_update_ms`, `traversal_ms` | Zahl | wie bei grep und trigram |
| `hint` | Text | Handlungshinweis, zum Beispiel leerer Index |
| `ignored_fields` | Zahl | ignorierte `resident*`-Felder |

#### Grundtabelle

| Ursache | Status | `detail.reason` |
|---|---|---|
| Feld `resident` fehlt oder ist leer | `skipped` | `no_query` |
| Konfiguration ungültig | `skipped` | `config_invalid` |
| Modell fehlt oder unlesbar | `skipped` | `model_unavailable` |
| Backend nicht verfügbar (zum Beispiel ohne `candle`) | `skipped` | `backend_unavailable` |
| `TaskError::Timeout` oder Budget überschritten | `aborted` | `timeout` |
| `TaskError::Cancelled` | `aborted` | `cancelled` |
| Engine-Fehler (Device, Memory, Model, Kernel …) | `skipped` (nicht gelaufen) oder `aborted` (während des Laufs) | `engine_error` mit `error.kind` und `error.retryable` |
| Index inkompatibel und Neuaufbau nötig | `ran`, `index.rebuilt = true` | `index_rebuilt` |
| Index gesperrt (zweiter Schreiber) | `ran`, `index_mode = read_only` | `index_locked` |
| Interner Fehler (Join-Fehler, Katalog) | `skipped` | `internal` |
| Skelett vor S4 | `skipped` | `not_implemented` |

#### Aufgaben

- `BackendError` und `From<TaskError>`: `Timeout` zu `Timeout`, `Cancelled` zu `Cancelled`, `InvalidPrecision` und `Engine(e)` zu `Engine{kind, retryable, message}`; `kind` aus der `ResidentError`-Variante, `retryable` aus `IsRetryable`; wegen `non_exhaustive` mit Auffangzweig.
- `apply_error` setzt `skipped` oder `aborted`, `detail.reason`, `detail.error` und loggt mit `tracing::warn!` (Felder laut Q2).
- Keine `anyhow`-Nutzung, keine Strings für Kontext in Fehlertexten statt typisierter Felder (Q1).
- Alle Fehlertypen sind `Send + Sync + 'static` (Compile-Zeit-Assertion im Test).

#### Abnahme L2.3

- Tabellengetriebener Test `error_mapping_matches_table`: je Zeile der Grundtabelle ein Fall.
- Test `error_types_are_send_sync`.
- Test `detail_schema_keys_are_stable`: Golden-Vergleich der Schlüsselmenge.

### L2.4 · Backend-Naht

**Zweck:** Die gesamte Layer-Logik spricht nur mit einem schmalen Trait; die Fassade bleibt in einem Adapter gekapselt.  
**Quellen:** `facade/*` (E18 bis E24), Konventionen Q3, Charta (Layer besitzt Chunking und Persistenz-Zeitpunkt).  
**Abhängt von:** L2.3.  
**Zu klären:** Welche Engine-Typen dürfen im Trait auftauchen (EntryId, Scope, Payload und Index sind gemeinsames Vokabular des Substrats; `TextRole`, `Output`, `TaskError` bleiben im Adapter)? Wie wird ein fehlgeschlagener Backend-Start je Prozess gemerkt?  
**Ergebnis:** `SemanticBackend`, `BackendFactory`, `ResidentBackend` (nutzt `ResidentRuntime` aus S1) und die Regel „der Layer präfixiert nie selbst“.

#### Skizze

```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Role { Query, Passage }

pub struct BackendInfo {
    pub kind: &'static str,
    pub model_id: String,
    pub dim: usize,
    pub spec: ModelSpec,
    pub revision: String,
    pub tokenizer_sha: String,
}

pub struct IndexHandle(u64);

pub struct BackendHit {
    pub entry: EntryId,
    pub score: f32,
    pub source: Option<Payload>,
    pub outdated: bool,
}

pub struct SearchOutcome {
    pub hits: Vec<BackendHit>,
    pub candidates: u64,
    pub discarded: usize,
    pub outdated: usize,
    pub stats: ScoreStats,
}

#[async_trait]
pub trait SemanticBackend: Send + Sync {
    fn info(&self) -> &BackendInfo;
    fn attach_index(&self, index: Arc<dyn Index + Send + Sync>) -> IndexHandle;
    fn detach_index(&self, handle: IndexHandle);
    async fn embed(
        &self,
        texts: Vec<String>,
        role: Role,
        deadline: Instant,
    ) -> Result<Vec<Vec<f32>>, BackendError>;
    async fn search(
        &self,
        index: &IndexHandle,
        query: Vec<f32>,
        scope: Scope,
        top_k: usize,
        deadline: Instant,
    ) -> Result<SearchOutcome, BackendError>;
}

#[async_trait]
pub trait BackendFactory: Send + Sync {
    async fn create(&self, cfg: &ResidentLayerConfig) -> Result<Arc<dyn SemanticBackend>, ResidentLayerError>;
}
```

#### Aufgaben

- `ResidentBackend::embed`: `Role` zu `TextRole`, `submit_embedding(TextInput::new(texts), EmbeddingParams::new(role))`, `handle.wait_timeout(remaining)`, Ergebnis nur als `Output::Embeddings` akzeptieren; jede andere Ausgabe ist `BackendError::Engine{kind: "protocol"}`.
- `ResidentBackend::search`: `submit_similarity(VectorInput::new(vec![query]), SimilarityParams::new(id).top_k(k).scope(scope))`, Ergebnis als `Output::Similarity(report)`; `RankedHit.source` durchreichen, Statistik (`s1`, `sk`, `mean`, `std`, `gap_top2`, `z_top1`) in `ScoreStats` übernehmen.
- Validierung der Embeddings im Adapter: Anzahl gleich Eingabeanzahl, Länge gleich `dim`, alle Werte endlich; sonst `BackendError::InvalidEmbedding`.
- `ResidentBackendFactory::create` baut `RuntimeConfig` aus der Layer-Konfiguration, startet `ResidentRuntime` in `spawn_blocking` (Start ist billig, aber `Drop` und `init()` sind blockierend) und liefert `BackendUnavailable` bei fehlendem Modell oder fehlenden Engine-Bausteinen.
- Regel: Der Layer setzt nie `query: ` oder `passage: ` selbst; die Präfixe gehören dem Tokenizer (L1.2) und sind per `SchemaHeader` an den Index gebunden.
- Compile-Zeit-Prüfung, dass alle Futures `Send` sind (`fn assert_send<T: Send>(_: T)` im Test).

#### Abnahme L2.4

- Test `backend_futures_are_send`.
- Test `embed_rejects_wrong_dimension_and_nan` (FakeBackend mit Fehler-Skript).
- Test `task_errors_map_to_backend_errors` (tabellengetrieben).
- Test `factory_reports_unavailable_without_model`.

### L2.5 · FakeBackend und Testkit

**Zweck:** Deterministisches Backend ohne Modell, mit dem Chunking, Katalog, Scope, Abbildung, Budgets und Fehlerpfade des Layers vollständig testbar sind.  
**Quellen:** Konventionen Q3 (Fakes `Fake*`, Sichtbarkeit `cfg(any(test, feature = "mock"))`), `resident/mock.rs` (`FakeClock`, `FakeMeasure`), `resident/index/memory.rs`, `tests/trigram_e2e.rs`.  
**Abhängt von:** L2.4, L0.4 (Korpus).  
**Zu klären:** Wie realistisch muss die Fake-Semantik sein? (Ausreichend: lexikalische Überlappung über Hashing-Trick, damit lexikalische Gold-Anfragen gelingen.)  
**Ergebnis:** `FakeBackend` mit Fehler-Skript und Zählern, `TmpCorpus`, `engine_with_fake()`, Hilfsfunktionen `query_for`, `entries_of`.

#### Aufgaben

- Embedding: Text in Kleinbuchstaben, Zerlegung an Nicht-Alphanumerika, jedes Token per `xxh3` auf einen Index in `0..dim` und ein Vorzeichen abbilden, Gewicht `1 + ln(tf)`, danach L2-Normalisierung; leerer Text ergibt den Nullvektor (der Adapter lehnt das wie das echte Backend ab).
- Suche: Kandidaten aus dem angehängten Index per `Scope` auflösen (`All`, `Range`, `Set`), Vektoren über `Index::read` (f16) lesen, Cosine, stabile Sortierung, Top-k; Payload über den konkreten Index oder `Index::payload`.
- Fehler-Skript: `FaultScript::new().timeout_on_embed(n).engine_error_on_search(kind, retryable).cancel_on_embed(n).latency(Duration)` (virtuelle Zeit mit `tokio::time::pause`).
- Zähler: `embed_calls`, `embedded_texts`, `search_calls`, damit Tests Wiederverwendung (kein Re-Embedding unveränderter Dateien) belegen können.
- `FakeBackendFactory` für `ResidentLayer::with_factory`.
- Sichtbarkeit: `#[cfg(any(test, feature = "mock"))]`; Integrationstests tragen `#![cfg(feature = "mock")]`.

#### Abnahme L2.5

- Test `fake_embedding_is_deterministic_and_normalised`.
- Test `fake_similarity_ranks_lexical_overlap_first`.
- Test `fault_script_triggers_exactly_once_per_entry`.
- `cargo test --features mock` ist grün.

### L2.6 · Layer-Skelett

**Zweck:** `impl Layer for ResidentLayer` mit allen Randpfaden, jedoch noch ohne Suchlogik.  
**Quellen:** `core/layer.rs`, `grep_layer.rs::run` (Muster Statusbehandlung), `trigram/layer.rs::TrigramLayer::state` (Muster Lazy-State), Konventionen Q1/Q2.  
**Abhängt von:** L2.2 bis L2.5.  
**Zu klären:** Failure-Caching je Prozess (D11); Verhalten von `background` ohne Konfiguration.  
**Ergebnis:** Lauffähiger Layer; bis S4 meldet er nach erfolgreicher Validierung `skipped` mit `reason = not_implemented`.

#### Skizze

```rust
pub struct ResidentLayer {
    cache: Arc<DirCache>,
    factory: Arc<dyn BackendFactory>,
    state: tokio::sync::OnceCell<Arc<LayerState>>,
    failure: parking_lot::Mutex<Option<Arc<ResidentLayerError>>>,
}

impl ResidentLayer {
    pub fn new() -> Self;
    pub fn with_factory(factory: Arc<dyn BackendFactory>) -> Self;
}

#[async_trait]
impl Layer for ResidentLayer {
    fn id(&self) -> &str { "resident" }
    fn stage(&self) -> LayerStage { LayerStage::Generate }
    fn applies(&self, query: &Query) -> bool { query.has("resident") }

    async fn run(&self, query: &Query, result_set: &ResultSet, ctx: &LayerContext) -> LayerStatus {
        let mut status = LayerStatus::new(self.id(), self.stage());
        let Some(text) = query.get_str("resident").map(str::trim).filter(|t| !t.is_empty()) else {
            status.skipped = true;
            status.detail.insert("reason".into(), json!("no_query"));
            return status;
        };
        // resolve config, obtain LayerState lazily, then (S4) search
        status
    }

    async fn background(&self, ctx: &BackgroundContext, cancel: CancellationToken) {
        // returns immediately unless maintenance.background_enabled (S3)
    }
}

struct LayerState {
    cfg: ResidentLayerConfig,
    backend: Arc<dyn SemanticBackend>,
    storage: Arc<LayerStorage>,
    cpu: Arc<CpuExecutor>,
}
```

#### Aufgaben

- `new()` erzeugt `DirCache::new(4096)` (wie grep und trigram) und die Standard-Factory.
- `state(&self, ...)` initialisiert mit `OnceCell::get_or_try_init`; nicht transiente Fehler (Konfiguration, Modell fehlt) werden in `failure` gemerkt und ohne erneuten Versuch gemeldet, `Timeout` nicht (D11).
- `run`: Pfade `no_query`, `config_invalid`, `model_unavailable`, `backend_unavailable`, danach `not_implemented`; `status.detail` enthält `schema`, `backend`, `root`, `timings_ms.total`.
- Kein Thread-Start und kein Modellzugriff, solange kein Aufruf mit `resident` erfolgt (Zähler in der Test-Factory).
- `background`: gibt sofort zurück; reagiert auf `cancel` (Grundlage für S3).
- Kein Panic-Pfad; `unwrap` nur in Tests.

#### Abnahme L2.6

- Tests `id_and_stage_are_stable`, `applies_only_with_resident_field`, `empty_text_is_skipped`, `missing_model_is_skipped_with_reason`, `invalid_config_is_skipped_with_reason`, `factory_is_not_called_without_resident_field`, `factory_failure_is_cached_per_process`, `timeout_is_not_cached`, `layer_futures_are_send`, `background_returns_immediately_by_default`.
- End-to-End-Rauchtest mit `Engine` und Default-Registry: `resident=x` ohne Modell liefert eine Statuszeile `resident` mit `skipped=true`, die anderen Layer laufen unberührt.

## Gate G2

- Layer registriert; `applies`, `skipped`-Pfade und Statusabbildung sind durch Tests belegt.
- Konfiguration mit Auflösungsreihenfolge und Validierung vorhanden.
- `SemanticBackend`, `ResidentBackend` (gegen S1 oder als Kompiliergerüst), `FakeBackend` und Testkit vorhanden.
- `cargo test`, `cargo test --features mock`, `cargo test --test api_hygiene` grün; `scripts/check_cfg_scatter.sh` grün.
- Keine Änderung am Verhalten von grep, trigram, glob, merge (Regression: `trigram_e2e` grün).

## Risiken und Stolpersteine

- `OnceCell::get_or_try_init` plus separater Failure-Cache ist fehleranfällig; die Reihenfolge der Prüfungen (Failure-Cache zuerst) per Test festschreiben.
- `async_trait` verlangt `Send`-Futures: `TaskHandle` und alle gehaltenen Locks müssen `Send` sein; kein `parking_lot`-Guard über ein `await` halten.
- `FakeBackend` darf nie in Produktionspfaden erreichbar sein; die Factory-Auswahl erfolgt ausschließlich in `new()` (Produktion) und `with_factory` (Tests).
- Der Detail-Schema-Golden-Test verhindert versehentliche Schlüsseländerungen, sollte aber bei bewusster Erweiterung nur eine Datei berühren.
