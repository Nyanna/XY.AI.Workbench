# Plan 01 (schlank) — Resident-Layer in xy-ai-rag

Umsetzungsplan für agentische Ausführung. Ersetzt `plan_01_00` bis `plan_01_99`. Pfade relativ zu `/home/user/xyan/xy.ai.workbench/`. Stand 2026-10-27; Ist-Stand gegen den Quelltext geprüft.

## 1. Ziel

Neuer Layer `ResidentLayer` (`id = "resident"`, Stage `Generate`) in `rag/src/layers/resident_layer/`. Er beantwortet eine natürlichsprachliche Anfrage (`resident=...`) über Text- und Quelldateien unterhalb der Suchwurzel. Er liefert pro Datei einen Treffer mit Zeilen und Excerpt im Format der übrigen Layer und pflegt dafür einen persistenten Vektorindex unter `.xyrag/layers/resident/`. Embedding und Similarity kommen aus der Resident-Engine (`rag/src/resident`, e5-small, Cosine, Parquet-Index, CPU). Ohne Modell meldet der Layer `skipped`, andere Layer bleiben unberührt.

Die Engine ist bisher nicht lauffähig zusammengebaut. Deshalb gibt es zwei unabhängige Spuren, die sich in J1 treffen:

- **Spur A** baut die Engine-Runtime (`rag/src/resident/runtime/`).
- **Spur B** baut den Layer gegen ein `FakeBackend`, ohne Modell und ohne Spur A.

## 2. Regeln

- Die Engine ist Substrat. Chunking, Anfrageformulierung, Kandidaten-k und der Zeitpunkt der Index-Persistenz liegen im Layer.
- Der Layer lässt die Gesamtanfrage nie scheitern: kein Panic, kein `unwrap` im Layer-Code, Fehler werden `skipped` oder `aborted` mit `detail.reason`.
- Kein neues `cfg(feature)`. Der Layer wird immer kompiliert; Testdoubles stehen hinter `cfg(any(test, feature = "mock"))`. `cuda` und `candle` bleiben, wo sie sind (`rag/docs/architecture/feature_matrix.md`).
- Fehler mit `thiserror`, `#[non_exhaustive]`, typisierte Felder. Kein `anyhow` in der Bibliothek.
- Persistenz nur über `LayerStorage`. Blockierende Arbeit (Datei lesen, Parquet) über `ctx.cpu.spawn`, nie auf dem Async-Thread.
- Der Layer setzt nie selbst `query: ` oder `passage: `. Die Präfixe setzt der Tokenizer der Engine (A2).
- Zahlenvergleiche in Tests mit Toleranz (f16 gegen f32: 1e-3).
- Bestehende Layer und CLI-Aufrufe bleiben unverändert (`trigram_e2e` bleibt grün).
- Import-Fallen: `core::engine::Engine` (Orchestrator) gegen `resident::processor::Engine` (Trait), beim Import mit `as ResidentEngine` umbenennen. `core::persistence::SharedIndex` gegen `resident::manager::streamed::SharedIndex` (Alias), Alias im Layer ausschreiben. Engine-Typen als `crate::resident::…` importieren.

## 3. Ist-Stand (relevante Fakten)

| Fakt | Fundort |
|---|---|
| `Layer`-Trait: `id`, `stage`, `applies` (Default `query.has(id)`), `run(&Query, &ResultSet, &LayerContext) -> LayerStatus`, `background` (Default: leer). Storage je Layer unter `.xyrag/layers/<id>/` | `rag/src/core/layer.rs` |
| `LayerContext{query, storage, shared_index, cpu}`; `CpuExecutor::spawn(f) -> Result<R>` ist awaitable und panic-sicher | `core/layer.rs`, `core/executor.rs` |
| `LayerStorage`: `put`, `get`, `keys`, `path_for`; jedes `put` ist eine eigene SQLite-Transaktion | `core/persistence.rs` |
| Referenz-Layer grep und trigram: Ergebnisfelder `File` und `Lines` (Zeilennummer als String-Schlüssel), `status.detail`; `MergeLayer` faltet per `File` | `layers/grep_layer.rs`, `layers/trigram/layer.rs`, `layers/merge_layer.rs` |
| `DirCache::list` wendet den globalen Include/Exclude-Filter an, listet keine Dot-Einträge, listet Symlinks, FIFOs und Sockets als „Dateien“, betritt keine Verzeichnis-Symlinks; Metadaten können bis `STALE_AFTER` veraltet sein | `layers/dir_cache.rs` |
| CLI: Registrierung in `build_default_registry`; `--max-results` setzt `maxResults`; alle Werte kommen als Strings; Prozessende wartet auf `background` | `rag/src/bin/xyrag.rs` |
| Fassade: `Resident::builder()…init()`, `submit_embedding(TextInput, EmbeddingParams)`, `submit_similarity(VectorInput, SimilarityParams)` liefern `TaskHandle` (`wait_timeout`); `SimilarityReport.hits: Vec<RankedHit{entry, score, source: Option<Payload>}>`. `Resident` droppen joint Worker blockierend. `ResidentBlocking` nicht in Tokio verwenden | `rag/src/resident/facade/*` |
| **Lücke 1:** keine produktive `Engine`-Implementierung (nur `NoEngine`, `TestEngine`, `Scripted`) | `resident/processor/hub.rs`, `facade/resident.rs` |
| **Lücke 2:** `IndexId` wird nirgends aufgelöst, `Resident::add_index` legt nur ab | `facade/resident.rs` |
| **Lücke 3:** `TextRole` erreicht den Tokenizer nicht; die e5-Präfixe setzt erst `SharedTokenizer::encode(EncodeMode, …)` | `op/mod.rs`, `planner/expand.rs`, `tokenize/*` |
| **Lücke 4:** `Index`-Trait ohne Payload-Zugriff; `ParquetIndex::payload` ist inhärent | `resident/index/mod.rs`, `parquet_io.rs` |
| Der Worker-Loop fängt Panics des ganzen Passes, ein bereits beanspruchter Job geht dabei verloren und seine Task hängt | `processor/worker.rs` |
| `ResourceCell::get_or_init` ist `async`, nutzt nur `tokio::sync` und ist ohne Runtime pollbar; `init` darf keine Tokio-I/O oder -Timer nutzen | `subengine/lifecycle.rs` |
| `ModelSpec::e5_small(dir, DType::F16)`: Dim 384, `max_seq_len` 512, Modell unter `rag/model/multilingual-e5-small` (Gewichte nicht in Git). Der Tokenizer kürzt stumm auf 512 Token | `resident/model/mod.rs`, `tokenize/mod.rs` |
| `ParquetIndex::open(path, SchemaHeader, IndexOptions)`: `verify` ist per Default `true` (dekodiert alles); `upsert(EntryId, f16-LE-Bytes, Payload)`, `remove`, `payload`, `flush` (schreibt alle Zeilen, atomarer Rename). `SchemaHeader::from_spec(&spec, revision, tokenizer_sha)`; Abweichung gibt `IndexError::IncompatibleSchema` | `index/parquet_io.rs`, `index/schema.rs` |
| `Scope::{All, Set, Range}` werden aufgelöst, `Subtree`, `Prefix`, `Centroid` scheitern. `EntrySet::from_ranges(IntoIterator<Item = Range<u64>>)` | `index/mod.rs`, `manager/scope.rs` |
| `Payload{target_file, char_start, char_end}`, Einheit der Offsets ist undokumentiert; wir legen Unicode-Skalarwerte fest | `index/payload.rs` |
| Testvorlage `tests/trigram_e2e.rs`; Features `candle`, `mock`; `tests/api_hygiene.rs`; Referenzvektoren `tests/data/reference/multilingual-e5-small.json` | `rag/tests/*` |

## 4. Festlegungen

| Thema | Festlegung |
|---|---|
| Aktivierung | `applies = query.has("resident")`. Felder: `resident` (Text), `residentIndex` (`auto` Default, `update`, `rebuild`, `off`), `directory`, `maxResults` (Default 20, Maximum 200). grep und trigram bleiben auf `query`, bestehende Aufrufe kosten nichts |
| Modul | `rag/src/layers/resident_layer/` (nicht `resident`, wegen `crate::resident`) |
| Backend-Naht | Trait `SemanticBackend` mit `ResidentBackend` (Fassade) und `FakeBackend` (Tests). Der Layer kennt keine Fassadentypen |
| Chunking | zeilenbasiert; Zeichenbudget `target 1000`, `max 1400`, `min 250`; keine Überlappung; Offsets halb offen in Unicode-Skalarwerten des BOM-bereinigten Textes. Passage-Text = Dateipfad, Zeilenumbruch, Chunk-Text; das Payload bezieht sich nur auf den Chunk-Text |
| EntryId | `(file_no << 20) \| chunk_no`; `file_no` wächst monoton und kommt aus dem Katalog; höchstens 1500 Chunks je Datei |
| Index | ein `ParquetIndex` je Dokumentwurzel unter `storage.path_for("index.parquet")`, `verify = false`; Flush alle 2000 Chunks und am Laufende |
| Katalog | ein JSON-Dokument unter dem Schlüssel `catalog` in `LayerStorage` (ein `put` je Commit); wird nach dem Flush geschrieben |
| Abdeckung | bedarfsgetrieben: indexiert wird, was in der aktuellen Anfrage sichtbar ist. Scope = Katalog geschnitten mit sichtbaren und frischen Dateien |
| Wartungsmodi | `auto`: neue und geänderte Dateien nachführen, höchstens 1,5 s oder 32 Chunks, geänderte zuerst. `update`: alles Fehlende, nur durch das Anfragebudget begrenzt. `rebuild`: Katalog und Index zurücksetzen, dann wie `update`. `off`: nur abfragen |
| Veraltete Treffer | Treffer aus Dateien, deren Größe oder mtime vom Katalog abweichen, werden verworfen und gezählt |
| Ergebnisformat | ein `ResultEntry` je Datei; Felder siehe B6 |
| Backend-Lebenszyklus | lazy beim ersten `run` mit `resident` (`tokio::sync::OnceCell`); ein Fehlschlag wird je Prozess gemerkt, außer bei Timeout. Kein Thread-Start ohne `resident`-Feld |
| Konfiguration | Modellverzeichnis aus `XYRAG_RESIDENT_MODEL_DIR`, sonst `<CARGO_MANIFEST_DIR>/model/multilingual-e5-small`, wenn dort `model.safetensors` liegt, sonst `skipped / model_unavailable`. Alles andere sind `const`-Werte (unten) |
| Schreibsperre | Advisory-Lock `index.lock` im Layer-Verzeichnis (`File::try_lock`, sonst `libc::flock`). Ohne Sperre läuft der Prozess read-only: abfragen ja, indexieren nein |
| Threading, Precision | `cpu_workers = 1`, Bibliotheks-Defaults, Fassaden-Default für `precision` |

Konstanten (vorläufig, ohne Messung gewählt, an einer Stelle als `const` definiert):

| Konstante | Wert |
|---|---|
| Chunk `target` / `max` / `min` | 1000 / 1400 / 250 Zeichen |
| Datei-Obergrenze | 1 MiB |
| Chunks je Datei | 1500 |
| Embedding-Batch | 16 Chunks |
| Flush | alle 2000 Chunks |
| Auto-Budget | 1,5 s oder 32 Chunks |
| Anfragebudget gesamt | 30 s (inklusive Kaltstart); `WorkerConfig`-Task-Timeout 60 s |
| `top_k` | `clamp(3 * maxResults, maxResults, 200)` |
| Anfragetext | höchstens 2000 Zeichen, sonst gekürzt |
| Ergebnis | höchstens 3 Chunks je Datei, Excerpt höchstens 200 Zeichen |
| Nie indexiert | Endungen `png jpg jpeg gif pdf zip gz tar so dll exe parquet safetensors sqlite db pem key p12`, Namen `id_rsa*` |

## 5. Spur A — Engine-Runtime

Neues Modul `rag/src/resident/runtime/`. Das ist Engine-Arbeit und berührt den Layer nicht.

**A1 · IndexDirectory.** `IndexDirectory` (`RwLock<HashMap<IndexId, Arc<dyn Index + Send + Sync>>>` mit `insert`, `get`, `remove`). `ResidentBuilder::index_directory(...)`; ohne Angabe legt `init()` ein privates Verzeichnis an, bestehende Tests bleiben unverändert. `Resident::add_index` schreibt hindurch; neu `Resident::remove_index(&IndexId) -> bool`. Zugriff auf eine unbekannte `IndexId` liefert `IndexError::IndexUnavailable`, nie Panic oder Hänger.

**A2 · Rolle zu Präfix.** `OpParams.role: Option<EncodeMode>`. `EmbeddingExpander` setzt es am Tokenize-Op aus `params.role()` (`Query` zu `EncodeMode::Query`, sonst `Passage`; `TextRole` ist `#[non_exhaustive]`). Der Tokenize-Executor wählt damit den Modus. Die Planner-Pipeline muss zu `ModelSpec::e5_small` passen (Dim, `max_seq_len`, Cosine). `PooledEncoder` poolt und normalisiert bereits, also Pool und L2Norm als Pass-through oder idempotent behandeln. Es gibt zwei `Pooling`- und zwei `ScorerKind`-Enums (`planner::config`, `model`, `facade::params`), nur Konsistenz prüfen, nicht vereinheitlichen. Präfixe nie doppelt setzen.

**A3 · Scorer-Brücke.** Default-Methode `Index::payload(&self, EntryId) -> Option<Payload> { None }` am Trait, in `ParquetIndex` implementiert. `ScorerBridge::score(index, scope, query, top_k) -> ScoredHits`: Scope auflösen (`All`, `Range`, `Set`), Kandidaten aufsteigend in Blöcke zu höchstens 8192 Einträgen, f16 zu f32 dekodieren, `CosineSlots::exact` mit `TopKScorer`, Gleichstand zugunsten der kleineren `EntryId`, `ScoredHit.source` füllen. Entfernte oder leere Einträge zählen als `discarded`. Die Vektoren nicht komplett materialisieren.

**A4 · CpuEngine.** `impl ResidentEngine for CpuEngine`: `harvest` und `feed` liefern `false`, `step` beansprucht eine Op (`hub.claim(Side::Cpu, …)`), führt sie aus und ruft `hub.complete_op` oder `hub.fail_op`.

| Op | Ausführung |
|---|---|
| Tokenize | `SharedTokenizer::encode(mode, texts)` |
| Embed | `plan_batches` oder `encode_batches`, `Encoder::encode_into` auf CPU-Regionen, Ergebnis in Eingabereihenfolge (flach, n mal dim) |
| Pool, L2Norm | Pass-through oder idempotent (A2) |
| Similarity, TopK | A3; Datenvertrag zwischen beiden festlegen und in `op/mod.rs` dokumentieren |
| WriteIndex, Infer, IntoHost | `fail_op` mit eindeutiger Meldung (nicht unterstützt) |

- Tokenizer und Encoder liegen in `ResourceCell`s und werden lazy geladen (`SharedTokenizer::from_spec`, `ModelRegistry::with_defaults().load(&spec, DeviceKind::Cpu)`). Die async-`get_or_init`-Brücke aus dem synchronen Worker (`futures::executor::block_on`) zuerst mit einem Prototyp-Test absichern: 16 parallele erste Tasks laden genau einmal, kein Deadlock.
- Op-Panics mit `catch_unwind` selbst fangen und `fail_op` aufrufen. Dafür neue Variante `OrchestrationError::OpPanicked { task }`.
- Vor jeder Op `job.run.is_cancelled()` prüfen.
- Kein TTL-Eviction, keine Backoff-Eigenbauten. Verwendet werden die vorhandenen Zellen.

**A5 · ResidentRuntime.** `ResidentRuntime::start(RuntimeConfig{model_dir, workers: WorkerConfig, task_timeout}) -> Result<Self, ResidentError>` baut `ModelSpec`, `RuleSet`, `EngineOffers` (CPU: Tokenizer, Encoder, Scorer, Index), `PlannerExpander` und `Resident::builder()…init()`. `start` lädt weder Tokenizer noch Gewichte. Zugriff über `resident()`, `spec()`, `fingerprint()` und `register_index(Arc<dyn Index + Send + Sync>) -> IndexId`. `ModelFingerprint{model_id, revision, tokenizer_sha}`: `revision` aus Hash von `config.json` plus Größe von `model.safetensors`, `tokenizer_sha` aus `tokenizer_fingerprint(tokenizer.json)`. `shutdown(self)` dokumentieren: nicht auf dem Async-Thread droppen, solange Tasks laufen.

**A6 · Tests und Gate A.** Ohne Modell:

- `role_selects_prefix`
- `add_index_makes_index_resolvable`, `unknown_index_id_fails_task_cleanly`
- `scorer_matches_bruteforce` (Range und Set, Toleranz 1e-3), `scorer_ties_prefer_lower_entry_id`, `payload_is_filled_for_parquet_index`
- `op_panic_fails_the_task_instead_of_hanging`, `unsupported_ops_fail_the_task`
- `concurrent_first_tasks_load_resources_once`
- `start_without_model_files_fails_cleanly`, `start_loads_nothing`

Mit Modell (Feature `candle`, ohne Modell Ausgabe `skipped: model not installed at …`):

- `embedding_matches_reference_on_cpu`: Cosinus mindestens 0,9999 gegen `tests/data/reference/multilingual-e5-small.json`
- `query_and_passage_roles_differ`
- `similarity_matches_bruteforce_on_parquet_index`

**Gate A:** alle Tests grün; `cargo test --test api_hygiene` grün; `scripts/check_cfg_scatter.sh` grün (Ort im Repo ermitteln); kein Candle-Typ in öffentlichen Signaturen; die bestehenden Fassaden-Tests laufen unverändert durch.

## 6. Spur B — Layer

Dateien unter `rag/src/layers/resident_layer/`: `mod.rs`, `layer.rs`, `error.rs`, `backend.rs`, `fake.rs`, `walk.rs`, `text.rs`, `chunk.rs`, `catalog.rs`, `store.rs`, `indexer.rs`, `search.rs`, `test/`.

**B1 · Gerüst.**

- `pub mod resident_layer;` in `layers/mod.rs`.
- `registry.register(Arc::new(ResidentLayer::new()))` in `build_default_registry` vor `MergeLayer`.
- `ResidentLayer::new()` und `with_factory(Arc<dyn BackendFactory>)` für Tests.
- `ResidentLayer` hält `Arc<DirCache>` (Kapazität 4096 wie grep), `OnceCell` für den State und den Failure-Cache.
- `background` nicht implementieren (Default).
- `ResidentLayerError` nach dieser Tabelle auf `LayerStatus` abbilden. `detail` enthält mindestens `reason`; `error` (Text) bei Fehlern, `hint` als Handlungshinweis.

| Ursache | Status | `detail.reason` |
|---|---|---|
| Feld `resident` leer | `skipped` | `no_query` |
| ungültiges Feld (`residentIndex`, `maxResults`) | `skipped` | `invalid_field` |
| Modell fehlt | `skipped` | `model_unavailable` |
| Backend-Start scheitert (zum Beispiel ohne `candle`) | `skipped` | `backend_unavailable` |
| Backend-Fehler zur Laufzeit | `skipped` | `engine_error` |
| Timeout oder Budget überschritten | `aborted` | `timeout` |
| Cancel | `aborted` | `cancelled` |
| Index gesperrt (zweiter Schreiber) | `ran` | `index_locked` |
| Index wegen Schema oder Beschädigung neu aufgebaut | `ran` | `index_rebuilt` |
| interner Fehler (Join, Katalog, I/O) | `skipped` | `internal` |

**B2 · Backend-Naht und FakeBackend.**

```rust
pub enum Role { Query, Passage }

pub struct BackendInfo { pub kind: &'static str, pub model_id: String, pub dim: usize,
                         pub spec: ModelSpec, pub revision: String, pub tokenizer_sha: String }
pub struct BackendHit { pub entry: EntryId, pub score: f32, pub source: Option<Payload> }
pub struct SearchOutcome { pub hits: Vec<BackendHit>, pub candidates: u64 }

#[non_exhaustive]
pub enum BackendError { Timeout, Cancelled, InvalidEmbedding, Engine { kind: String, retryable: bool, message: String } }

#[async_trait]
pub trait SemanticBackend: Send + Sync {
    fn info(&self) -> &BackendInfo;
    fn attach_index(&self, index: Arc<dyn Index + Send + Sync>) -> IndexHandle;
    fn detach_index(&self, handle: IndexHandle);
    async fn embed(&self, texts: Vec<String>, role: Role, deadline: Instant) -> Result<Vec<Vec<f32>>, BackendError>;
    async fn search(&self, index: &IndexHandle, query: Vec<f32>, scope: Scope, top_k: usize, deadline: Instant)
        -> Result<SearchOutcome, BackendError>;
}

#[async_trait]
pub trait BackendFactory: Send + Sync {
    async fn create(&self, model_dir: &Path) -> Result<Arc<dyn SemanticBackend>, ResidentLayerError>;
}
```

- `ResidentBackend` (Adapter auf `ResidentRuntime`): `Role` zu `TextRole`; `embed` über `submit_embedding` und `wait_timeout(remaining)`, `search` über `submit_similarity` mit `SimilarityParams::new(id).top_k(k).scope(scope)`; `RankedHit.source` durchreichen. `TaskError` abbilden (`Timeout` zu `Timeout`, `Cancelled` zu `Cancelled`, `Engine(e)` und `InvalidPrecision` zu `Engine{kind, retryable (aus IsRetryable), message}`, wegen `non_exhaustive` mit Auffangzweig). Jede Embedding-Zeile prüfen: Anzahl gleich Eingabe, Länge gleich `dim`, endlich, sonst `InvalidEmbedding`. `ResidentBackendFactory` startet die Runtime in `spawn_blocking`.
- `FakeBackend` (`cfg(any(test, feature = "mock"))`): Hashing-Trick-Embedding (Kleinbuchstaben, Zerlegung an Nicht-Alphanumerika, `xxh3` auf Index und Vorzeichen, Gewicht `1 + ln(tf)`, L2-normalisiert); `search` per Brute Force über `Index::read` mit Scope; Zähler `embed_calls`, `embedded_texts`, `search_calls`; wahlweise ein vorgegebener `BackendError` oder künstliche Latenz (`tokio::time::pause`). Dazu `FakeBackendFactory`.
- Tests: `backend_futures_are_send`, `embed_rejects_wrong_dimension_and_nan`, `task_errors_map_to_backend_errors`, `fake_embedding_is_deterministic_and_normalised`, `fake_search_ranks_lexical_overlap_first`.

**B3 · Dateien, Text, Offsets.**

- `walk::visible_files(cache, root, rel_prefix) -> Vec<VisibleFile{rel, abs, size, mtime_ns}>`: iterativer Walk über `DirCache::list` (globaler Filter wirkt dadurch), `rel` relativ zur Dokumentwurzel mit `/`, nach `rel` sortiert. Keine dritte Kopie von `collect_files`; neuer Helfer, grep und trigram bleiben unangetastet.
- `walk::eligible`: nur nach Metadaten (Größe 0, über 1 MiB, Endung oder Name auf der Denylist).
- `text::read_text(path, max_bytes) -> Result<TextFile{text, content_hash}, ReadError>`: nur reguläre Dateien (`symlink_metadata`; Symlinks, FIFOs, Sockets, Geräte ablehnen), Lesen mit `take(max_bytes + 1)`, NUL im Inhalt bedeutet `Binary`, strenges UTF-8, führendes U+FEFF entfernen, Hash `xxh3-64` über die Rohbytes.
- `OffsetMap::new(text)` mit `line_of_char(c) -> u32` (1-basiert), `byte_of_char(c)` und `slice(text, start, end)`; Zeilenanfänge einmal berechnen, Abfragen per Binary Search. CRLF bleibt im Text, Zeilen enden an `\n`.
- Tests: `offsets_roundtrip_multibyte` (Umlaute, CJK, Emoji, CRLF, BOM, letzte Zeile ohne Zeilenende), `binary_and_invalid_utf8_are_rejected`, `fifo_and_symlink_are_not_read`, `walk_lists_exactly_the_visible_files` (keine Dot-Einträge, korrekte Präfixe bei `directory`, Verzeichnis-Symlink-Schleife terminiert), `payload_offsets_reconstruct_source` (Payload in `ParquetIndex` schreiben, lesen, mit `slice` den Originaltext rekonstruieren).

**B4 · Chunker.**

`Chunker::chunk(text, &OffsetMap) -> Vec<Chunk{char_start, char_end, line_start, line_end, text}>` und `passage_text(rel, &Chunk)`.

- Zeilenweise sammeln bis `target`. Danach bevorzugt an der letzten Leerzeile im Fenster `[min, max]` brechen, sonst am nächsten Zeilenende.
- Ein Chunk überschreitet `max` nie. Eine einzelne Zeile über `max` wird an einer Zeichengrenze geteilt, bevorzugt am letzten Leerzeichen.
- Whitespace-only-Chunks entfallen. Die Zahl der Chunks wird bei 1500 gekappt (`truncated`).
- Tests (ein `proptest` genügt): `chunks_are_ordered_bounded_and_slice_exact` (geordnet, nicht leer, höchstens `max`, `text` entspricht dem Quellbereich, nie Panic bei Mehrbyte-Zeichen, jede Nicht-Whitespace-Stelle wird überdeckt, deterministisch), `very_long_line_is_split`, `cap_marks_truncated`.
- Tokenzahl: Der Tokenizer kürzt stumm auf 512. Deshalb prüft J1 (`chunks_stay_below_512_tokens`) mit dem echten Tokenizer, dass kein Chunk darüber liegt. Bei Überschreitung `max` senken.

**B5 · Katalog, IndexStore, Indexierung.**

Katalog (ein JSON-Dokument, `serde`):

```rust
pub struct Catalog { next_file_no: u64, files: BTreeMap<String /*rel*/, FileRecord> }
pub struct FileRecord { file_no: u64, size: u64, mtime_ns: u64, content_hash: u64, chunks: u32 }
```

- `entry_id(file_no, chunk_no) -> EntryId` und `file_of(EntryId) -> (u64, u32)`; eine Datei belegt den Bereich `[file_no << 20, (file_no << 20) + chunks)`.
- `Catalog::plan(visible) -> Plan{new, changed, touched, unchanged, removed}`: ohne Eintrag `new`; andere Größe `changed`; gleiche Größe und gleicher mtime `unchanged`; gleiche Größe und anderer mtime: lesen und Hash vergleichen (gleich `touched`, nur mtime aktualisieren; anders `changed`). Katalogdateien, die nicht sichtbar sind: existiert die Datei noch (`symlink_metadata`), bleibt sie unberührt und liegt nur außerhalb des Scopes (Filter), sonst `removed`.
- `Catalog::commit(storage)` schreibt das Dokument mit einem `put`.

`IndexStore`:

- `open`: Pfad `storage.path_for("index.parquet")`, `SchemaHeader::from_spec(&info.spec, &info.revision, &info.tokenizer_sha)` mit `DType::F16`, `IndexOptions{verify: false, …}`.
- `IncompatibleSchema`, Prüfsummen- oder Parquet-Lesefehler: Indexdatei löschen, Katalog zurücksetzen, `index_rebuilt` melden. Keine Quarantäne, keine Migration.
- Konsistenzprüfung beim Öffnen: Summe der Katalog-Chunks gegen die Zahl der Index-Einträge (`Index::stamps().count()`). Bei Abweichung (Absturz zwischen Flush und Katalog-Commit) Index und Katalog zurücksetzen.
- `replace_file(file_no, old_chunks, new)`: `upsert(entry_id(file_no, i), f16-Bytes, Payload::new(rel, char_start, char_end))`, überschüssige alte Chunk-Nummern per `remove` entfernen. `remove_file`, `payload(entry)`, `flush`.
- Schreibsperre `index.lock` (nicht blockierend). Ohne Sperre überspringt der Layer jede Wartung (`index_locked`).

Indexierung `maintain(visible, mode, budget, cancel) -> IndexReport`:

- Arbeitsliste: erst `changed`, dann `new`, je nach Pfad sortiert. `removed` entfernen. `touched` nur im Katalog nachziehen.
- Je Datei: `read_text`, `OffsetMap`, Chunker, `passage_text`. Alle Chunks eines Laufs in Batches zu 16: `backend.embed(texts, Role::Passage, deadline)`, anschließend f16-Bytes (`half::f16::from_f32(..).to_le_bytes()`). Eine Datei wird erst geschrieben (`replace_file`), wenn alle ihre Vektoren vorliegen; nie teilweise.
- Zwischen Batches `cancel` und Deadline prüfen. Fehlgeschlagene Dateien (ungültiger Vektor, nicht lesbar, geändert während des Laufs) bekommen keinen Katalogeintrag und werden im nächsten Lauf erneut versucht. Engine-Fehler (`Engine`, `Timeout`, `Cancelled`) brechen den Lauf ab; der letzte Flush bleibt gültig.
- Flush alle 2000 Chunks und am Ende, direkt danach `catalog.commit`. Nie dateiweise flushen (Flush schreibt alle Zeilen) und nie dateiweise in `LayerStorage` schreiben.
- `auto` stoppt bei Budget ohne Fehler und meldet `remaining_files` und `hint`. `rebuild` setzt Katalog und Index zurück (altes Handle per `detach_index` freigeben), dann wie `update`. `off` tut nichts.
- Höchstens ein Indexierer je Prozess (`tokio::sync::Mutex` im State).
- Tests (Unit): `plan_classifies_new_changed_touched_unchanged_removed`, `filtered_out_file_is_not_removed`, `schema_mismatch_rebuilds`, `count_mismatch_resets`, `replace_file_removes_surplus_chunks`, `second_writer_gets_no_lock`, `file_is_written_only_when_all_vectors_arrived`, `cancel_between_batches_leaves_no_partial_file`.

**B6 · Query-Pfad.** `run` in dieser Reihenfolge:

1. Felder lesen: Text trimmen, Whitespace normalisieren, auf 2000 Zeichen kürzen. `maxResults` und `residentIndex` aus Strings parsen (CLI), ungültig führt zu `skipped / invalid_field`. `directory` über `query.resolve_search_root` auflösen (Suchwurzel und `rel_prefix`).
2. Modellverzeichnis auflösen, Backend lazy erzeugen (B1, Failure-Cache zuerst prüfen), Index öffnen.
3. `walk::visible_files`, danach `maintain(mode)`.
4. Scope bilden: sichtbare Dateien mit Katalogeintrag und gleicher Größe und mtime. Abweichende zählen als `stale` und liegen nicht im Scope. Scope = `Scope::Set(EntrySet::from_ranges(...))` mit einem Bereich je Datei. Ist der Scope leer, kein Embedding und keine Suche; Status `ran`, `contributions = 0`, `hint` (Index leer oder keine passende Abdeckung, `residentIndex=update` ausführen).
5. Anfrage einbetten: `backend.embed(vec![text], Role::Query, deadline)`, genau ein Vektor.
6. `backend.search(handle, qvec, scope, top_k, deadline)`.
7. Treffer abbilden: `(file_no, chunk_no) = file_of(entry)`, `rel` über die Umkehrung des Katalogs (das `target_file` des Index wird nicht benutzt); Payload aus `hit.source`, sonst `store.payload(entry)`, ohne Payload verwerfen. Pro Trefferdatei frisch `fs::metadata` (nicht den `DirCache`) mit dem Katalog vergleichen, bei Abweichung verwerfen (`dropped_stale`). Datei einmal mit `read_text` lesen und `OffsetMap` bilden; liegt `char_end` hinter dem Textende, verwerfen. Zeilenbereich `line_of_char(char_start)` bis `line_of_char(char_end - 1)`. Excerpt ist die erste nicht leere Zeile des Chunks, rechts getrimmt, Tabs zu einem Leerzeichen, auf 200 Zeichen mit Mittenkürzung begrenzt. Das Lesen läuft gebündelt in einem `ctx.cpu.spawn`-Block.
8. Je Datei höchstens 3 Chunks (nach Wertung), insgesamt höchstens `maxResults` Chunk-Treffer, Dateien nach bestem Score (Gleichstand: Pfad).
9. Einträge schreiben, Status füllen.

Ergebnisformat (ein `ResultEntry::new(None, fields)` je Datei; `File` relativ zur Dokumentwurzel mit `/` wie bei grep; Scores auf vier Nachkommastellen):

```json
{
  "File": "docs/de/espresso.md",
  "Lines": { "12": "## Mahlgrad und Brühzeit" },
  "Resident": { "Score": 0.8421, "Chunks": [ { "Start": 12, "End": 31, "Score": 0.8421 } ] }
}
```

`Lines` enthält je Chunk die Nummer der Excerpt-Zeile und deren Text. Status: `ran = true`, `contributions` gleich Anzahl der Dateieinträge. `detail`: `root`, `coverage{visible, indexed, stale}`, `index{files_new, files_changed, files_removed, chunks_written, remaining_files}`, `search{candidates, hits, dropped_stale}`, `hint`. Leere Ergebnisse sind regulär.

**B7 · Tests Ende zu Ende (mit `FakeBackend`).**

Datei `rag/tests/resident_layer_e2e.rs` mit `#![cfg(feature = "mock")]`, Vorlage `tests/trigram_e2e.rs`. Der Hilfstyp `TmpCorpus` erzeugt die Dateien zur Laufzeit in einem temporären Verzeichnis. Es gibt weder einen eingecheckten Korpus noch ein Gold-Set. Inhalt: etwa 8 Dateien (zwei Markdown deutsch, zwei englisch, eine Rust-, eine Python-Datei, thematisch klar getrennt) plus Sonderfälle: leer, nur Whitespace, CRLF, BOM, Emoji und CJK, Binärdatei mit NUL, ungültiges UTF-8, Datei über 1 MiB, Dot-Datei.

- `initial_build_then_query_finds_lexical_match_with_correct_lines` (inklusive Umlaute, CJK, Emoji, CRLF)
- `second_run_does_not_reembed` (`embedded_texts` bleibt gleich)
- `touch_without_change_does_not_reembed`
- `modified_file_replaces_its_chunks_only`, `deleted_file_is_not_returned`, `renamed_file_is_delete_plus_add`
- `auto_stops_at_budget_and_update_completes`
- `directory_field_limits_results`
- `stale_hit_is_dropped`
- `skipped_files_are_not_indexed` (zu groß, binär, kein UTF-8, Denylist)
- `crash_between_flush_and_catalog_commit_recovers` (Index per `IndexStore` schreiben und flushen, Katalog nicht committen, neu öffnen, Reset und Neuaufbau prüfen)
- `schema_mismatch_rebuilds_and_reports_it`
- `second_writer_runs_read_only`
- `empty_index_reports_hint`, `max_results_is_respected`
- `backend_timeout_is_aborted_and_other_layers_unaffected`: alle Layer aktiv mit `query=…` und `resident=…`, parallel und sequenziell, `FakeBackend` mit Fehler; grep, trigram, glob liefern unverändert, nach `merge` genau ein Eintrag je Datei
- `missing_model_is_skipped_with_reason` (Default-Registry, Engine, ohne Modell, die anderen Layer laufen)

Globaler Filter in `rag/tests/resident_layer_filter.rs` (eigene Datei, denn `configure_global_filter` gilt einmalig je Prozess): `global_exclude_removes_files_from_scope`.

Dazu Unit-Tests aus B1 bis B5 sowie `applies_only_with_resident_field` und `factory_is_not_called_without_resident_field`.

**Gate B:** `cargo test`, `cargo test --features mock`, `cargo test --test api_hygiene`, `cargo test --test trigram_e2e` und `scripts/check_cfg_scatter.sh` sind grün.

## 7. Zusammenführung

Voraussetzung: Gate A und Gate B.

**J1 · Echtes Backend.** `ResidentBackendFactory` gegen `ResidentRuntime`. `rag/tests/resident_layer_model.rs` (Features `candle,mock`, ohne Modell Ausgabe `skipped: model not installed at …`, nicht stillschweigend grün):

- `semantic_and_crosslingual_queries_find_the_target_file`: Korpus aus B7 indexieren, je eine Paraphrase ohne Schlüsselwörter und eine Frage in der anderen Sprache, das Zieldokument liegt in den ersten 5 Treffern.
- `chunks_stay_below_512_tokens`: mit dem echten Tokenizer über `rag/src/**/*.rs` und `*.md`; Maximum höchstens 512 Token, sonst `max` im Chunker senken.
- `prefixes_matter`: ohne Präfixe (Test-Spec) schlechterer Treffer als mit, Wachposten gegen eine kaputte Rollenweitergabe.

Bei Fehlschlag zuerst Chunking, Präfix und Passage-Kopfzeile prüfen.

**J2 · CLI.**

- `after_help` in `Cli` (`bin/xyrag.rs`) um einen kurzen Abschnitt „Semantic search (resident layer)“ mit den vier Feldern und zwei Beispielen ergänzen. Moduldoku in `resident_layer/mod.rs`: Felder, Modi, Ergebnisformat, Modellpfad, Hinweis auf Kaltstart bei jedem CLI-Aufruf und auf den Erstaufbau per `residentIndex=update`.
- Prozess-Test mit `CARGO_BIN_EXE_xyrag` in `rag/tests/resident_layer_cli.rs`: `resident=x` ohne Modell gibt `skipped / model_unavailable`, das Binary endet innerhalb von 2 s; `query=x` verhält sich unverändert.

## 8. Fertig, wenn

- Gate A, Gate B, J1 und J2 grün; die Modelltests laufen mindestens einmal auf einer Maschine mit installiertem Modell.
- Im Layer-Code und in `resident/runtime` gibt es kein `todo!()`, `unimplemented!()`, `dbg!()`, keinen `unwrap` außerhalb von Tests.
- Alle Befehle aus Abschnitt 9 grün.

## 9. Befehle

```bash
cd /home/user/xyan/xy.ai.workbench/rag
cargo test                                   # Baseline vor dem Start und danach
cargo test --features mock
cargo test --features candle,mock           # Modelltests (Modell erforderlich)
cargo test --test api_hygiene --test trigram_e2e
scripts/check_cfg_scatter.sh                # Ort im Repo ermitteln
```

Vor dem Start: Baseline `cargo test` und `cargo test --features mock` ausführen. Prüfen, ob `rag/model/multilingual-e5-small/model.safetensors` vorhanden ist (sonst laufen die Modelltests nicht) und welche Rust-Toolchain installiert ist (`File::try_lock` braucht eine aktuelle; sonst `libc::flock`).

## 10. Bewusst nicht Teil des Plans

Background-Indexierung, Daemon, Dateiwächter, GPU-Pfad, Rerank, Funnel, Hybrid-Fusion, `SharedIndex`-Nutzung, Index-Sharding, Kalibrierung und Precision-Profile, Konfigurationsdatei (TOML), Statusmodus, Quarantäne und Migrationsschema, Telemetrie-Spans, Benchmarks, Messgates und Zahlenbudgets, Recall-Schwellen und Gold-Set, CI-Matrix, Abnahmekatalog, Architekturdokument. Die Konstanten aus Abschnitt 4 sind gewählt, nicht gemessen. Falls etwas davon später gebraucht wird, ist es ein eigener Plan.
