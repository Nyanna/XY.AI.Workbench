# Plan 01 — Befundbericht (Evidenz)

Belegte Ausgangslage für die Anbindung der Resident-Engine als Layer. Jede Zeile nennt Befund, Beleg (Pfad und Symbol relativ zu `/home/user/xyan/xy.ai.workbench/`) und die Folge für den Plan. Legende: **[V]** live im Quelltext/Dokument verifiziert · **[W]** Domänenwissen oder Schätzung · **[?]** offen (Klärung in Stufe 0, Paket L0.1).

## A. Layer-Vertrag und Engine-Kern

| ID | Tag | Befund | Beleg | Folge |
|---|---|---|---|---|
| E1 | V | `Layer`-Trait: `id`, `stage` (Default `Enrich`), `depends_on`, `applies` (Default `query.has(self.id())`), `run(&Query, &ResultSet, &LayerContext) -> LayerStatus`, `background(&BackgroundContext, CancellationToken)` | `rag/src/core/layer.rs::Layer` | Layer-ID ist zugleich Aktivierungsfeld und Storage-Namespace |
| E2 | V | `LayerContext{query, storage, shared_index, cpu}`; `BackgroundContext{storage, shared_index, cpu}` ohne `Query` | `core/layer.rs` | Background kennt die Dokumentwurzel nicht direkt (L3.7) |
| E3 | V | `Engine::run_query` bildet Ebenen via `build_levels` (Standardkanten Generate vor Enrich vor Postprocess); CLI-Default `--mode parallel`; Storage je Layer unter `.xyrag/layers/<id>/` (`cache.db` plus Sidecars) | `core/engine.rs`, `core/topology.rs`, `core/persistence.rs::PersistenceManager::layer_storage` | `resident` (Generate) läuft parallel zu grep/trigram/glob und vor `merge` |
| E4 | V | `start_background` startet jeden Layer-`background`; das CLI ruft `start_background`, dann `run_query`, dann `shutdown(false)`, das auf alle Background-Tasks wartet (kein Cancel) | `core/engine.rs`, `bin/xyrag.rs::main` | Background darf im One-Shot-CLI nicht lange laufen (D9) |
| E5 | V | `build_default_registry` registriert Glob, Grep, Trigram, Merge; Kommentar „Concrete layers are added here later“ | `bin/xyrag.rs::build_default_registry` | Registrierungsstelle für `ResidentLayer` |
| E6 | V | CLI-Felder `key=value`, `--json`, `--root`, `--mode`, `--include`, `--exclude`, `--max-results` (setzt `maxResults`) | `bin/xyrag.rs::Cli`, `parse_query` | gemeinsame Felder `directory`, `maxResults` |
| E7 | V | `run_layer` baut den Kontext und ruft `layer.run`; nur Storage-Fehler werden als `Err` propagiert, Panics in `run` würden die Anfrage treffen | `core/engine.rs::run_layer` | Layer darf nie panicken; Fehler nur über `LayerStatus` |
| E8 | V | `CpuExecutor::spawn(f) -> Result<R>` (rayon, awaitable, panic-sicher als `Err`) | `core/executor.rs` | blockierende Layer-Arbeit dort ausführen |
| E53 | V | CLI-Ausgabe: YAML mit `results` (je `ResultEntry::to_dict()` inklusive `id`) und `layers` (`layer_id`, `stage` in Kleinbuchstaben, `ran`, `skipped`, `aborted`, `contributions`, `detail`); `Lines`-Schlüssel erscheinen als sortierte ganze Zahlen | `bin/xyrag.rs::status_to_json`, `numeric_keys_for_lines`, `core/result.rs::ResultEntry::to_dict` | Beispiele in Plan und Doku folgen diesem Format |

## B. Referenz-Layer

| ID | Tag | Befund | Beleg | Folge |
|---|---|---|---|---|
| E9 | V | grep und trigram: `Generate`, `applies = query.has("query")`, Budget `MAX_RUNTIME` 60 s, `START_CUTOFF_RATIO` 0,9, `MAX_FILE_SIZE` 5 MiB, `DIR_CACHE_CAPACITY` 4096, Ergebnis `File` plus `Lines` (Zeilennummer als String-Schlüssel); `status.detail` mit `root`, `dir_cache_misses`, `dir_cache_update_ms`, `traversal_ms`, `files_searched`, `search_ms` (trigram zusätzlich `normalize_mirror_ms`, `signature_ms`, `candidates_after_signature_filter`) | `layers/grep_layer.rs`, `layers/trigram/layer.rs` | Muster für Budget, Detail-Schlüssel, Ergebnisform |
| E10 | V | trigram: lazy `OnceLock<Arc<SharedState>>` aus `ctx.storage`; pro Datei ein Indexeintrag in `LayerStorage` (ein `put` je Datei), Reindex bei mtime-/Parameteränderung, `prune_deleted` | `layers/trigram/layer.rs::TrigramLayer::state`, `trigram/index.rs` | Muster für Lazy-State; `put` je Datei ist für große Korpora teuer (D7) |
| E11 | V | `collect_files` und `join_rel` sind in grep und trigram privat dupliziert | `grep_layer.rs`, `trigram/layer.rs` | neuer Helfer statt dritter Kopie (L3.1) |
| E12 | V | `MergeLayer`: Postprocess, `applies = true`, faltet Einträge mit gleichem `File`; `Lines` mit „kürzerer Text gewinnt“, übrige Objekte rekursiv | `layers/merge_layer.rs` | Ergebnisform `File`/`Lines` ist Pflicht, Zusatzfelder als Objekt |
| E13 | V | Die Core-Doku nennt als Beispiel die Felder `path`/`line`, die ausgelieferten Layer verwenden `File`/`Lines` | `core/layer.rs` (ModuleDoc) gegenüber `layers/*` | Konvention der ausgelieferten Layer ist maßgeblich (D10) |

## C. Persistenz und Dateiauswahl

| ID | Tag | Befund | Beleg | Folge |
|---|---|---|---|---|
| E14 | V | `LayerStorage`: `put`, `get`, `delete`, `keys(prefix)`, `path_for`, `get_cursor`, `set_cursor`; lazy angelegt; jedes `put` ist eine eigene SQLite-Transaktion (WAL) | `core/persistence.rs::LayerStorage` | Katalog gebündelt in Shards ablegen (D7) |
| E15 | V | `SharedIndex` (SQLite `index.db`: path, hash, size, mtime, seq, deleted) mit `upsert`, `mark_deleted`, `get`, `changes_since`; in `rag/src` und `rag/benches` ruft kein Layer `upsert` oder `mark_deleted` auf | `core/persistence.rs`; Quelltextsuche | es gibt keine zentrale Änderungsquelle (D7) |
| E16 | V | `DirCache::list` wendet den prozessweiten Filter (`configure_global_filter`) transparent an; `list_with_filter` umgeht ihn; Dot-Einträge werden nie gelistet; `FileInfo{name, mtime_ns, size}` | `layers/dir_cache.rs` | Sichtbarkeit und Metadaten kommen aus `DirCache` (D8) |
| E17 | V | `PathFilter` exponiert öffentlich nur `new` und `is_empty`; Match-Methoden sind privat | `layers/dir_cache.rs::PathFilter` | Filtersemantik nur über `DirCache` nutzbar |
| E52 | V | `read_dir_listing` nutzt `DirEntry::metadata()` (folgt keinen Symlinks) und überspringt Dot-Namen: Verzeichnisse werden als Verzeichnisse gelistet, alles andere — auch Symlinks auf Dateien oder Verzeichnisse, FIFOs, Sockets, Geräte — erscheint als „Datei“ mit Größe und mtime des Eintrags selbst; Verzeichnis-Symlinks werden nicht traversiert | `layers/dir_cache.rs::read_dir_listing` | Layer prüft den Dateityp vor dem Lesen (L3.2, L6.5) |

## D. Resident-Fassade

| ID | Tag | Befund | Beleg | Folge |
|---|---|---|---|---|
| E18 | V | `Resident::builder()` mit `model(ModelId)`, `worker_hint(WorkerConfig)`, `min_op_buffer`, `expander(Arc<dyn Expander>)`, `engines(cpu, Option<gpu>)`, `init()`; Defaults `Unplanned` (Tasks scheitern) und `NoEngine`; Ressourcen entstehen erst durch die erste Task | `resident/facade/resident.rs` | Zusammenbau fehlt (E26) |
| E19 | V | `submit_embedding(TextInput, EmbeddingParams)`, `submit_similarity(VectorInput, SimilarityParams)`, `submit_inference(PairInput, InferenceParams)` liefern `TaskHandle` (`IntoFuture`, `wait_timeout`, Drop storniert nicht begonnene Tasks); `Output::{Embeddings, Hits, Similarity, Scores}` | `facade/resident.rs`, `facade/handle.rs`, `facade/params.rs` | Query-Pfad: zwei Tasks (Query-Embedding, Similarity) |
| E20 | V | `SimilarityParams{index, top_k, scope, scorer, precision}`; `ScorerKind::CrossEncoder` führt zu `ScorerUnavailable`; `top_k = 0` ist ungültig | `facade/params.rs`, `planner/expand.rs::SimilarityExpander` | Rerank ist Ausblick |
| E21 | V | `SimilarityReport{task_id, candidates, discarded, outdated, hits: Vec<RankedHit{entry, score, outdated, source: Option<Payload>}>}` plus Statistik (`s1`, `sk`, `mean`, `std`, `gap_top2`, `gap_to_k`, `z_top1`, `softmax`, `top1_mass`, `entropy`, `elbow`); „the engine decides nothing, thresholds stay outside“ | `facade/report.rs` | Schwellen- und Cut-Politik liegt im Layer (D19) |
| E22 | V | `TaskError::{Cancelled, Timeout, InvalidPrecision, Engine(Arc<ResidentError>)}`; `ResidentError` bündelt Index, Precision, Planner, Config, Capability, Orchestration, Subengine, Device, Memory, Model, Tokenize, Kernel, Segment; `IsRetryable` | `facade/error.rs`, `resident/error.rs` | Fehlerabbildung auf `LayerStatus` (L2.3) |
| E23 | V | `ResidentBlocking` besitzt eine eigene Runtime und darf nicht innerhalb einer Tokio-Runtime benutzt werden | `facade/blocking.rs` | im Layer nur die async-Handles verwenden |
| E24 | V | `facade/mod.rs` re-exportiert `Resident`, `ResidentBuilder`, `ResidentConfig`, `IndexInfo`, `Residency`, `TaskHandle`, `TaskError`, `Output`, `TextInput`, `VectorInput`, `EmbeddingParams`, `SimilarityParams`, `TextRole`, `ScorerKind`, `RankedHit`, `SimilarityReport`, `WorkerConfig`, `Extensions` | `facade/mod.rs` | Importe über `crate::resident::facade` |

## E. Planner, Processor und die Engine-Lücke

| ID | Tag | Befund | Beleg | Folge |
|---|---|---|---|---|
| E25 | V | Task-Ketten: Embedding = Tokenize, Embed, optional Pool, optional L2Norm, optional WriteIndex; Similarity = Similarity, TopK; Inference = Tokenize, Infer | `planner/expand.rs` | Op-Menge, die eine CPU-Engine beherrschen muss |
| E26 | V | `Engine`-Trait (`harvest`, `feed`, `step`); Worker-Pass harvest, feed, step, expand; die einzigen Implementierungen sind `NoEngine` (Platzhalter), `TestEngine` und `Scripted` (Tests); `Resident::builder()` wird nur in Tests aufgerufen | `processor/hub.rs`, `processor/worker.rs`, `processor/tests.rs`, `facade/resident.rs`; Quelltextsuche | **Lücke 1:** keine produktive Engine, kein lauffähiges `Resident` |
| E27 | V | `Resident.indexes: HashMap<IndexId, IndexEntry{info, _index}>`; `IndexInfo{residency: NotLoaded, leases: 0}` wird nirgends fortgeschrieben; Processor und Planner lösen `IndexId` nicht auf | `facade/resident.rs`; Quelltextsuche nach `IndexId` | **Lücke 2:** Verdrahtung `IndexId` zu Index fehlt |
| E28 | V | `TextRole` erreicht den Op-Pfad nicht: `OpParams{requires, top_k, dim}` ohne Rolle, `EmbeddingExpander` reicht sie nicht durch, `initial_input` übergibt Rohtexte; die Präfixe setzt erst `SharedTokenizer::encode(EncodeMode, …)` | `op/mod.rs`, `planner/expand.rs`, `planner/adapter.rs`, `tokenize/mod.rs`, `tokenize/modes.rs` | **Lücke 3:** Rolle zu Präfix fehlt (Qualität von e5) |
| E29 | V | Zwei `Pooling`-Enums (`planner::config` und `model`) und zwei `ScorerKind`-Enums (`planner::config` und `facade::params`); `PooledEncoder` poolt und normalisiert selbst, der Planner fügt zusätzlich Pool/L2Norm-Ops ein | `planner/config.rs`, `model/mod.rs`, `model/encoder.rs`, `planner/expand.rs` | Pipeline-Konsistenz prüfen (L1.2) |
| E30 | ? | Datenvertrag zwischen `Similarity`- und `TopK`-Op ist nicht festgelegt (`HostData::F32`, `Hits`, `Scored`) | `op/mod.rs::HostData`, `planner/adapter.rs::into_output` | in L1.4 festlegen und dokumentieren |
| E49 | V | Der `Index`-Trait hat keine Payload-Methode; `ScoredHit.source` und `RankedHit.source` müssen vom Op-Executor befüllt werden, `ParquetIndex::payload` ist inhärent | `resident/index/mod.rs::Index`, `resident/op/mod.rs::ScoredHit`, `resident/facade/report.rs::RankedHit`, `resident/index/parquet_io.rs` | **Lücke 4:** Default-Methode am Trait (L1.5), Rückfall im Layer (L4.4) |
| E50 | V | `WorkerConfig{cpu_workers, gpu_workers, min_op_buffer, candle_rayon_threads, cpu_nice, starved_ttl, task_timeout}`; Default `new(1, 0)` mit `min_op_buffer` 2, `cpu_nice` 5, `starved_ttl` 200 ms, `task_timeout` None; `cpu_nice` wird nur angewandt, wenn GPU-Worker existieren; ohne `task_timeout` können Tasks unbegrenzt laufen; `candle_rayon_threads` wird im Quelltext nirgends ausgewertet (nur gespeichert) | `resident/processor/config.rs`, `resident/processor/worker.rs::Processor::start` | Layer setzt Timeouts (L1.6, L2.2); Threading (D15, L5.3) |
| E51 | V | Der Worker-Loop fängt Panics des gesamten Passes ab (`catch_unwind`); ein dabei bereits beanspruchter Job geht verloren, seine Task bliebe offen | `resident/processor/worker.rs::run`, `pass` | CpuEngine fängt Op-Panics selbst ab und ruft `fail_op` (L1.4) |
| E56 | V | `OrchestrationError::{Cancelled, Backoff, EngineUnavailable, CompletionLost, OpFailed{task, source}, Timeout, Shutdown}` (non_exhaustive); eine Variante für Op-Panics fehlt | `resident/processor/error.rs` | Variante `OpPanicked{task}` ergänzen (L1.4) |

## F. Modell, Tokenizer, Kernel, Manager

| ID | Tag | Befund | Beleg | Folge |
|---|---|---|---|---|
| E31 | V | `ModelSpec::e5_small(dir, dtype)`: `intfloat/multilingual-e5-small`, Dim 384, MeanMasked, L2, Präfixe `query: ` und `passage: `, `max_seq_len` 512, Compute-Dtype CPU und GPU f32; `SchemaHeader::from_spec` verlangt Speicher-Dtype F16 | `resident/model/mod.rs`, `resident/index/schema.rs` | Spec mit `DType::F16` für den Index |
| E32 | V | `SharedTokenizer::from_spec` lädt `tokenizer.json` neben `config.json` und kürzt auf `max_seq_len`; die Zahl gekürzter Eingaben erscheint nur im Tracing-Span | `resident/tokenize/mod.rs` | Chunker muss Kürzung vermeiden (L3.3) |
| E33 | V | `ModelRegistry::with_defaults()`, `CandleBert`, `bert_factory`; Referenztests `installed_models_cpu_match_reference`; Modell liegt unter `rag/model/multilingual-e5-small` (Gewichte nicht in Git) | `resident/model/*`, `model/test/candle_adapter.rs`, `rag/model/README.md` | Modelltests nur bei installiertem Modell |
| E34 | V | CPU-Scoring: `CosineSlots::{new, exact}` über `SlotSource`, `TopKScorer`, `score_ranges`; `CpuIndexManager` (RAM primär, Lesen direkt am Index) | `resident/kernel/{mod,cosine}.rs`, `resident/manager/streamed.rs` | Bausteine für L1.5 sind vorhanden |
| E55 | V | `ResourceCell::get_or_init(limiter, now_ms, init)` ist async, nutzt nur `tokio::sync` (`Semaphore`, `OnceCell`) und verlangt `init: FnOnce() -> Future<Output = Result<R, SubengineError>>`; `SubengineError::ResourceInit{resource, reason}` für Ladefehler; Backoff und `evict_idle(now_ms)` sind eingebaut (`CellConfig{ttl_ms, backoff_ms}`) | `resident/subengine/lifecycle.rs`, `resident/subengine/error.rs` | Async-Init aus Worker-Threads machbar, `init` ohne Tokio-I/O (L1.3) |

## G. Index-Substrat

| ID | Tag | Befund | Beleg | Folge |
|---|---|---|---|---|
| E35 | V | `Index`-Trait (`stamp`, `stamps`, `read`, `apply`, `changed`, `capabilities`, `children`, `members`, `summary`, `prefetch`, `extent`, `backing`); Schreibreihenfolge „Daten, dann Stamp, dann Signal“; `read` darf blockieren | `resident/index/mod.rs` | Layer ruft Index-Methoden nicht auf dem Async-Thread auf |
| E36 | V | `ParquetIndex::open(path, SchemaHeader, IndexOptions{access: Mmap, verify: true, row_group_rows: 8192})`, `upsert(EntryId, f16-LE-Bytes, Payload)`, `remove`, `payload`, `flush` (Temp-Datei plus Rename, schreibt alle Zeilen, `false` wenn unverändert), `verify`, `is_dirty` | `resident/index/parquet_io.rs` | `verify=false` im Query-Pfad; Flush-Politik (D6) |
| E37 | V | `Payload{target_file, char_start, char_end}` mit xxh3-Checksumme; die Einheit der `char_*`-Offsets ist nicht dokumentiert | `resident/index/payload.rs` | **[?]** Einheit festlegen (D4, L3.2) |
| E38 | V | `SchemaHeader` bindet Modell-ID, Revision, Dim, Pooling, Normalisierung, Präfixe, Speicher-Dtype, `max_seq_len`, Tokenizer-SHA und Hash-Algorithmus; Abweichung führt zu `IndexError::IncompatibleSchema` | `resident/index/schema.rs` | Neuaufbau-Pfad (L3.5, L6.3) |
| E39 | V | `FileStamp{size, mtime_ns, ctime_ns, scan_ns}` und `Verdict::{Unchanged, NeedsHash}` (racy-sicher), `content_hash` (xxh3-64) | `resident/index/stamp.rs` | für den Katalog wiederverwendbar (L3.4) |
| E40 | V | `Scope::{All, Range, Set(EntrySet), Subtree, Prefix, Centroid}` (non_exhaustive), `EntrySet` auf Roaring-Basis | `resident/index/mod.rs`, `resident/manager/scope.rs` | Verzeichnisfilter als `Scope::Set` (L4.2); `ParquetIndex::capabilities()` ist `[Range]`; `All` und `Set` werden direkt aufgelöst (`resolve_direct`), `Subtree`, `Prefix`, `Centroid` scheitern mit `UnsupportedScope` |
| E41 | V | `InMemoryIndex` als Referenzindex, baut ohne `cuda`/`candle` | `resident/index/memory.rs` | Tests ohne Parquet möglich |

## H. Dokumentation, Konventionen, Tests

| ID | Tag | Befund | Beleg | Folge |
|---|---|---|---|---|
| E42 | V | Scope-Charta: außerhalb der Engine liegen Chunking, Query-Formulierung, Kandidaten-k und Rerank-Politik, Vollständigkeitsgarantien, approximative Prefilter, Index-Persistenzformat und -zeitpunkt; die Engine ist „Substrat, kein Retrieval-Algorithmus“ | `rag/docs/architecture/scope_charta.md` §1 bis §4 | verbindliche Verantwortungsteilung (LP1) |
| E43 | V | Konventionen Q1 bis Q4: thiserror je Schicht, `non_exhaustive`, `IsRetryable`; tracing mit festen Feldnamen; Testdoubles unter `cfg(any(test, feature = "mock"))`; keine `anyhow` in Bibliotheken | `rag/docs/architecture/conventions.md` | LP8, LP10, LP11 |
| E44 | V | `cfg(feature = "cuda")` nur in `backend::create_backend`; `candle`-Gating nur in `memory::dtype`; `scripts/check_cfg_scatter.sh` erzwingt das | `rag/docs/architecture/feature_matrix.md` | **[?]** Ort des Skripts; kein neues `cfg(feature)` (LP9) |
| E45 | V | f32- und f16-Cosine unterscheiden sich um 1e-3 bis 1e-4; Vergleiche immer mit Toleranz (`kernel::golden::Tolerance`) | `rag/docs/architecture/phase_plan.md` | Toleranzen in Tests (LP11) |
| E46 | V | Addons/Kaskaden: Strategie (Funnel, Kandidatenzahlen) gehört in die Suchschicht; Referenz-Kaskade in `facade/test/addon.rs` | `rag/docs/architecture/addons.md` | Ausblick (Rerank/Funnel) |
| E47 | V | Testinfrastruktur: `tests/trigram_e2e.rs` (TmpDirs, engine_for, query_for, lines_of), `tests/api_hygiene.rs`, `tests/deps_smoke.rs`, `tests/data/reference/multilingual-e5-small.json`, `benches/trie_bench.rs`; Features `candle`, `cuda`, `mock`, `json`; Dev-Abhängigkeiten `proptest`, `criterion`, `mockall`, `loom`, `shuttle` | `rag/tests/*`, `rag/benches/*`, `rag/Cargo.toml` | Vorlagen für S2 bis S7 |
| E48 | V | Benötigte Abhängigkeiten sind bereits vorhanden: `half`, `memmap2`, `xxhash-rust`, `parquet`, `arrow`, `roaring`, `tokenizers`, `safetensors`, `toml`, `thiserror`, `tracing`, `tokio` (rt-multi-thread, macros, sync, time), `async-trait`, `libc` (Linux), `serde`, `serde_json` | `rag/Cargo.toml` | keine neuen Laufzeit-Abhängigkeiten geplant |
| E54 | V | `resident::failpoint`: benannte Fehlerinjektionspunkte `hit(name, key) -> bool` und `arm(name, key, times) -> Armed` (Drop entschärft); ohne `test`/`mock` konstant `false` | `resident/failpoint.rs` | Test-Nähte des Layers nutzen diese Facility (L3.8, L6.1) |

## I. Namens- und Semantikfallen

| ID | Tag | Befund | Beleg | Folge |
|---|---|---|---|---|
| K1 | V | `crate::resident` (Engine) und das neue Layer-Modul tragen ähnliche Namen | `lib.rs`, `layers/mod.rs` | Layer-Modul heißt `layers::resident_layer` (D2) |
| K2 | V | `core::engine::Engine` (RAG-Orchestrator) und `resident::processor::Engine` (Trait) | `core/engine.rs`, `processor/hub.rs` | beim Import umbenennen (`as ResidentEngine`) |
| K3 | V | `core::persistence::SharedIndex` (SQLite-Dateiindex) und `resident::manager::streamed::SharedIndex` (Alias auf `Arc<dyn Index + Send + Sync>`) | `core/persistence.rs`, `manager/streamed.rs` | Aliase im Layer-Code ausschreiben |
| K4 | V | `Resident::shutdown(self)` ist ein `drop`; `Drop` joint die Worker blockierend | `facade/resident.rs` | nie auf dem Async-Thread droppen, wenn Tasks laufen |
| K5 | V | `IndexOptions::default().verify == true` dekodiert und prüft beim Öffnen die ganze Datei | `index/parquet_io.rs` | im Query-Pfad `verify=false` (D6) |
| K6 | V | Die Metaplanung verweist auf `resident_precision_calibration.md`, die reale Datei heißt `resident_precision.md` | `project/resident/resident_planplan.md` (Vorspann) | nur Hinweis |
