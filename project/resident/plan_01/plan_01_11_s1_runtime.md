# Stufe 1 — Runtime-Assembly (CPU-Zusammenbau der Resident-Engine)

Stufe 1 schließt die Lücken des Befundberichts (E26 bis E30 sowie E49 bis E51): Sie baut aus den vorhandenen Bausteinen ein lauffähiges CPU-`Resident` im neuen Engine-Modul `rag/src/resident/runtime/`. Ergebnis ist `ResidentRuntime::start(config)`; das gelieferte `Resident` beantwortet `submit_embedding` und `submit_similarity` auf einem registrierten Index korrekt. Die Stufe ist Engine-Arbeit (Substrat im Sinn der Scope-Charta) und berührt den Layer nicht; sie läuft parallel zu S2 bis S4. Aufwand [W]: 6 bis 9 Personentage; kritischer Pfad des Gesamtplans.

## Eingang

Gate G0 bestanden; D12 bestätigt; R1 bis R4 ausgewertet. Modell `rag/model/multilingual-e5-small` installiert (sonst Modelltests als `ignored`, siehe L1.7).

## Die vier Lücken

| Nr | Lücke | Beleg | Paket |
|---|---|---|---|
| 1 | Keine produktive `Engine`-Implementierung; `Resident::builder()` nur in Tests | E26 | L1.3, L1.4, L1.6 |
| 2 | `IndexId` wird nirgends zu einem Index aufgelöst; `Resident::add_index` legt nur `_index` ab | E27 | L1.1 |
| 3 | `TextRole` erreicht den Tokenizer nicht, die e5-Präfixe fehlen im Op-Pfad | E28 | L1.2 |
| 4 | Der `Index`-Trait bietet keinen Payload-Zugriff; `ScoredHit.source` und `RankedHit.source` bleiben ohne Quelle | E49; `index/mod.rs::Index`, `op/mod.rs::ScoredHit`, `index/parquet_io.rs::ParquetIndex::payload` (inhärent) | L1.5 |

## Pakete

### L1.1 · IndexDirectory und Fassaden-Verdrahtung

**Zweck:** `IndexId` zur Laufzeit zu einem Index auflösen; Fassade und Engine teilen dieselbe Zuordnung.  
**Quellen:** `resident/facade/resident.rs` (`Resident::add_index`, `IndexEntry._index`, `IndexInfo`, `Residency`), `resident/op/requirements.rs` (`Requirements.index`, `IndexId`), `resident/manager/streamed.rs` (Alias `SharedIndex`, `IndexManager::attach_index`, `CpuIndexManager`).  
**Abhängt von:** L0.1 (R3).  
**Zu klären:** Wo lebt das Verzeichnis (Fassade-eigen oder geteilt mit den Engines)? Wann wechselt `Residency` auf `Loaded` und wie werden `leases` gezählt? Wann wird der Index am `CpuIndexManager` angehängt (`attach_index` ist „first one stays“)?  
**Ergebnis:** `IndexDirectory` (`runtime/directory.rs`) und Builder-Methode `ResidentBuilder::index_directory`; `Resident::add_index` schreibt hindurch; `index_info` zeigt den Zustand.

#### Aufgaben

- `IndexDirectory` mit `insert`, `get`, `remove` und einem Zähler für aktive Leases implementieren (kurze kritische Abschnitte, `parking_lot::RwLock`).
- `ResidentBuilder::index_directory(Arc<IndexDirectory>)` ergänzen; ohne Angabe legt `init()` ein privates Verzeichnis an, damit bestehende Tests unverändert laufen.
- `Resident::add_index` trägt den Index zusätzlich in das Verzeichnis ein; das ungenutzte Feld `_index` entfällt oder wird zum Verzeichnis-Handle.
- `IndexInfo` fortschreiben: `Loaded` nach dem ersten erfolgreichen Zugriff einer Op, `leases` während laufender Ops.
- Beim Zugriff einer Op auf eine unbekannte `IndexId` `IndexError::IndexUnavailable` liefern (kein Panic, kein Hängen).
- `Resident::remove_index(&IndexId) -> bool` ergänzen, damit der Layer nach einem Neuaufbau das alte Handle freigeben kann (Rückgabe `false` bei laufenden Leases).

#### Skizze

```rust
pub struct IndexDirectory {
    inner: parking_lot::RwLock<HashMap<IndexId, Arc<dyn Index + Send + Sync>>>,
}

impl IndexDirectory {
    pub fn insert(&self, id: IndexId, index: Arc<dyn Index + Send + Sync>);
    pub fn get(&self, id: &IndexId) -> Option<Arc<dyn Index + Send + Sync>>;
    pub fn remove(&self, id: &IndexId) -> Option<Arc<dyn Index + Send + Sync>>;
}

impl ResidentBuilder {
    pub fn index_directory(mut self, dir: Arc<IndexDirectory>) -> Self;
}
```

#### Abnahme L1.1

- Test `add_index_makes_index_resolvable_by_engine`: nach `add_index` liefert `directory.get(&id)` denselben Index.
- Test `index_info_tracks_residency`: `NotLoaded` vor dem ersten Task, `Loaded` danach.
- Test `unknown_index_id_fails_task_cleanly`: Similarity auf unbekannter ID endet mit `TaskError::Engine(Index(IndexUnavailable))`.
- Bestehende Fassaden-Tests (`facade/test/*`) bleiben ohne Änderung grün.

### L1.2 · Rolle zu Präfix und Pipeline-Konsistenz

**Zweck:** `TextRole` aus `EmbeddingParams` bis zum Tokenize-Op durchreichen, damit `SharedTokenizer::encode(EncodeMode::{Query, Passage}, …)` die e5-Präfixe setzt; Planner-`PipelineConfig` und `ModelSpec` widerspruchsfrei halten.  
**Quellen:** `planner/expand.rs::EmbeddingExpander`, `planner/adapter.rs::initial_input`, `planner/config.rs` (`PipelineConfig`, `Prefixes::for_role`), `tokenize/modes.rs` (`EncodeMode`, `ModeScheme`), `op/mod.rs::OpParams`, `model/mod.rs::ModelSpec::e5_small`, `model/encoder.rs::PooledEncoder`.  
**Abhängt von:** L0.1 (R4).  
**Zu klären:** Rolle als Feld in `OpParams` (Hash und Eq beachten, `OpParams` ist Cache-Schlüssel) oder als Teil des Eingabewerts? Layering: `op` darf `facade::TextRole` nicht importieren, daher `EncodeMode` (tokenize) oder ein eigenes Enum in `op`. Doppelte `Pooling`- und `ScorerKind`-Enums (E29): vereinheitlichen oder Mapping testen. Pool/L2Norm-Ops: Pass-through, wenn `PooledEncoder` bereits gepoolt und normalisiert hat.  
**Ergebnis:** Rolle im Op-Pfad; Konsistenztest zwischen Planner-Pipeline und `ModelSpec`; dokumentierte Festlegung, ob Pool/L2Norm eigenständige Ops oder Pass-through sind.

#### Aufgaben

- `OpParams` um `role: Option<EncodeMode>` erweitern (Konstruktor `OpParams::new` setzt `None`); `EmbeddingExpander` setzt es am Tokenize-Op aus `params.role()` (wegen `#[non_exhaustive]` mit Wildcard-Arm).
- Den Tokenize-Executor (L1.4) über `params.role` den `EncodeMode` wählen lassen; fehlt die Rolle, `Passage` annehmen und `tracing::warn!` ausgeben.
- Die e5-Pipeline im Planner so definieren, dass sie zu `ModelSpec::e5_small` passt: `max_seq_len` 512, `dim` 384, Cosine, und Pool/L2Norm entweder deaktiviert (`pooling = None`, `normalize = false`, weil der Encoder sie ausführt) oder als nachweislich idempotente Ops.
- Die Präfixe werden ausschließlich vom Tokenizer gesetzt (`ModeScheme::Prefix(spec.prefixes)`); `PipelineConfig.prefixes` bleibt leer oder wird gegen die Spec geprüft, damit nichts doppelt präfixiert wird.
- Der Layer präfixiert niemals selbst (siehe D13, Layer-Regel in L2.4).

#### Skizze

```rust
// op/mod.rs
pub struct OpParams {
    pub requires: Requirements,
    pub top_k: Option<u32>,
    pub dim: Option<u32>,
    pub role: Option<EncodeMode>,
}

// planner/expand.rs, EmbeddingExpander
let mut tokenize = OpParams::new(ctx.requires(ResourceKind::Tokenizer, tokenizer));
tokenize.role = Some(match params.role() {
    TextRole::Query => EncodeMode::Query,
    _ => EncodeMode::Passage,
});
chain.push(OpKind::Tokenize, tokenize);
```

#### Abnahme L1.2

- Test `role_selects_prefix`: dieselbe Eingabe ergibt mit `Query` und `Passage` unterschiedliche Token-Sequenzen (`query: ` gegen `passage: `).
- Test `pipeline_matches_spec`: Planner-Pipeline und `ModelSpec` stimmen in Dim, `max_seq_len`, Pooling, Normalisierung und Präfixen überein; kein doppeltes Pooling.
- Test `planner_chain_for_e5_has_no_redundant_ops` (oder Idempotenztest der Pool/L2Norm-Ops).

### L1.3 · Ressourcen: Tokenizer- und Encoder-Zellen

**Zweck:** Lazy-Load (die erste Task zahlt), Einmal-Laden bei Konkurrenz, TTL-Eviction; CPU-Speicherbereiche für Token und Ausgabe.  
**Quellen:** `subengine/lifecycle.rs` (`ResourceCell`, `CellConfig`, `InitLimiter`, `ResourceLease`, `OpGuard`), `subengine/resource.rs` (`Resource`, `ResourceKey`), `tokenize::SharedTokenizer` (implementiert `Resource`), `model::{ModelRegistry, LoadedEncoder, RegionIo}`, `memory/cpu.rs` (`CpuMemoryManager`), `processor/resource.rs` (`ResourceSlot`: Backoff nach Ladefehlern).  
**Abhängt von:** L0.1.  
**Zu klären:** `ResourceCell::get_or_init` ist `async`, `Engine::step` läuft synchron auf Worker-Threads (`futures::executor::block_on` möglich?); `InitLimiter` und `OnceCell` sind Tokio-Sync-Primitive und ohne Runtime pollbar (E55), `init` darf aber weder Tokio-I/O noch -Timer nutzen. TTL-Defaults (One-Shot: unwichtig; Daemon: etwa 10 Minuten). Verhalten nach Ladefehlern (Backoff, wiederholter Fehler wird als `Model`/`Tokenize`-Fehler an die Task gemeldet).  
**Ergebnis:** Struktur `Resources` mit `tokenizer()` und `encoder()` (Leases), geteiltem `CpuMemoryManager` und ausschließlich lazy erzeugten Ressourcen.

#### Aufgaben

- `Resources::new(spec, cfg)`: erzeugt Zellen, keine Ladevorgänge.
- Laden: `SharedTokenizer::from_spec(&spec)` und `ModelRegistry::with_defaults().load(&spec, DeviceKind::Cpu)`; Fehler: `get_or_init` verlangt `SubengineError`; Ladefehler auf `SubengineError::ResourceInit{resource, reason}` abbilden (E55), die typisierte Quelle geht dabei verloren, daher eine Variante mit `#[source]` erwägen.
- Async-Init aus synchronen Workern: Ansatz festlegen (zum Beispiel `futures::executor::block_on`), Deadlock-Freiheit nachweisen (Test mit parallelen ersten Tasks).
- Eviction: `evict_idle(now_ms)` periodisch aus `Engine::step` (nur wenn kein Job claimbar ist) aufrufen; Standard-TTL aus der Konfiguration.
- Encoder-Aufrufe in einem dedizierten rayon-Pool mit `candle_rayon_threads` Threads ausführen (`pool.install`), falls Candle den globalen Pool nutzt (**[?]**, O9); die Thread-Zahl kommt aus `WorkerConfig` bzw. `RuntimeConfig` (D15).
- Zeitquelle als `Clock` injizieren (`resident::mock::Clock`, `FakeClock` in Tests), keine direkten `Instant::now()`-Aufrufe in der Logik.

#### Skizze

```rust
pub struct Resources {
    spec: ModelSpec,
    tokenizer: ResourceCell<SharedTokenizer>,
    encoder: ResourceCell<LoadedEncoder>,
    limiter: InitLimiter,
    memory: Arc<CpuMemoryManager>,
    clock: Arc<dyn Clock>,
}

impl Resources {
    pub fn tokenizer(&self) -> Result<ResourceLease<SharedTokenizer>, ResidentError>;
    pub fn encoder(&self) -> Result<ResourceLease<LoadedEncoder>, ResidentError>;
    pub fn evict_idle(&self);
}
```

#### Abnahme L1.3

- Test `concurrent_first_tasks_load_resources_once`: 16 parallele erste Tasks lösen genau einen Ladevorgang aus (Zähler in der Encoder-Fabrik).
- Test `load_failure_reports_model_error_and_backs_off`: fehlende Gewichte führen zu `Model`-Fehler, wiederholte Versuche unterliegen dem Backoff.
- Test `idle_ttl_evicts_and_reloads` mit `FakeClock`.

### L1.4 · CpuEngine: Op-Ausführung

**Zweck:** `impl Engine for CpuEngine` — `harvest` und `feed` sind auf der CPU leer (`false`), `step` beansprucht eine Op, führt sie aus und schließt sie ab.  
**Quellen:** `processor/hub.rs` (`Engine`, `Hub::{claim, complete_op, fail_op}`, `Step`), `processor/worker.rs` (`pass`, `run`: Worker fängt Panics des Passes ab), `processor/tests.rs::TestEngine` (Referenzmuster), `op/mod.rs` (`OperationNode`, `Successor`, `Value`, `HostData`, `OpKind`), `planner/adapter.rs` (`Chain::node`, `Step::follow`).  
**Abhängt von:** L1.1, L1.2, L1.3, L1.5.  
**Zu klären:** `accepts`-Filter (CPU-fähige Ops: `requires.engine` ist `Cpu` oder `Either`); Umgang mit `materialize` und Device-Werten (CPU: immer Host); Verhalten bei nicht unterstützten Ops (`Infer`, `IntoHost`, `WriteIndex`: `fail_op` mit `PlannerError::MissingCapability`); Datenvertrag `Similarity` zu `TopK` (E30).  
**Ergebnis:** `CpuEngine` mit Executors je `OpKind`; Task-Ketten Embedding und Similarity laufen durch den Hub bis zum `Output`.

#### Op-Tabelle

| OpKind | Eingabe | Ausgabe | Ausführung | Bausteine |
|---|---|---|---|---|
| Tokenize | `HostData::Text` | `HostData::Tokens` | `SharedTokenizer::encode(mode, texts)`, `mode` aus `params.role` | tokenize |
| Embed | `HostData::Tokens` | `HostData::F32` (flach, n mal dim) | Batching mit `plan_batches` oder `encode_batches`, `Encoder::encode_into` auf CPU-Regionen, Rückordnung über `Batch::indices` | model, memory::cpu |
| Pool | `HostData::F32` | `HostData::F32` | Pass-through, wenn der Encoder bereits gepoolt hat (L1.2) | — |
| L2Norm | `HostData::F32` | `HostData::F32` | idempotent (`kernel::cpu_simd::normalize`) | kernel |
| Similarity | `HostData::F32` (Query-Vektoren flach) plus `requires.index` | `HostData::Scored` (Empfehlung: bereits auf `top_k` begrenzt) | Scope auflösen, Vektoren blockweise lesen, `CosineSlots::exact` | L1.5 |
| TopK | `HostData::Scored` | `HostData::Scored` oder `HostData::Hits` | Pass-through oder Merge über Blöcke; `top_k` aus `params.top_k` | kernel |
| WriteIndex | — | — | in v1 nicht unterstützt (D13): `fail_op` mit eindeutiger Meldung, da keine EntryId-Quelle existiert | — |
| Infer, IntoHost | — | — | in v1 nicht unterstützt (CrossEncoder ist Ausblick) | — |

#### Aufgaben

- `CpuEngine::step`: `hub.claim(Side::Cpu, &accepts)`, bei `None` `Step::Idle(None)`; sonst Ausführung, Folge-Ops über `job.node.successor.follow(output, &job.node.task)` ermitteln, `hub.complete_op(job, followups)`; bei Fehler `hub.fail_op(job, err)`.
- Panics der Op-Ausführung selbst abfangen (`catch_unwind`) und die Task über `fail_op` beenden: Der Worker-Loop schützt nur den Pass (E51), der bereits beanspruchte Job ginge sonst verloren und die Task hängt.
- Cooperative Cancellation: vor jeder Op `job.run.is_cancelled()` prüfen und mit `job.run.skip()` abbrechen.
- Den Datenvertrag Similarity zu TopK festlegen und in `op/mod.rs` dokumentieren (E30).
- Tracing: `resource_span(task_id, kind)` je Op, Feldnamen laut Konventionen Q2 (`task_id`, `op`, `resource_kind`, `index_id`).

#### Skizze

```rust
pub struct CpuEngine {
    resources: Arc<Resources>,
    scorer: Arc<ScorerBridge>,
    directory: Arc<IndexDirectory>,
}

impl Engine for CpuEngine {
    fn step(&self, hub: &Hub) -> Step {
        let accepts = |p: &OpParams| self.can_run(p);
        let Some(job) = hub.claim(Side::Cpu, &accepts) else {
            return Step::Idle(None);
        };
        let outcome = catch_unwind(AssertUnwindSafe(|| self.execute(&job)));
        match outcome {
            Ok(Ok(followups)) => hub.complete_op(job, followups),
            Ok(Err(err)) => hub.fail_op(job, err),
            Err(_) => hub.fail_op(job, ResidentError::from(OrchestrationError::OpPanicked { task })),
        }
        Step::Worked
    }
}
```

#### Abnahme L1.4

- Test `embedding_chain_runs_to_output`: Tokenize bis L2Norm liefern `Output::Embeddings` mit korrekter Anzahl und Dimension.
- Test `similarity_chain_runs_to_output`: Similarity und TopK liefern `Output::Similarity` mit `SimilarityReport`.
- Test `unsupported_ops_fail_the_task_not_the_engine`.
- Test `op_panic_fails_the_task_instead_of_hanging` (Fault-Injection im Executor).
- Test `cancelled_task_skips_remaining_ops`.
- Neue Variante `OrchestrationError::OpPanicked { task }` (E56, `non_exhaustive`) ist ergänzt und getestet.

### L1.5 · Scorer-Brücke: Index, Scope und Payload

**Zweck:** Vektoren eines `Index` blockweise als f32 bereitstellen, den Scope zu Kandidaten auflösen, Cosine-Top-k berechnen und Treffer mit Quellverweis (`Payload`) versehen.  
**Quellen:** `kernel/mod.rs` (`SlotSource`, `SlotScorer`, `Scorer`, `QueryBatch`, `SliceRange`, `TopKScorer`, `score_ranges`), `kernel/cosine.rs` (`CosineSlots::exact`, `cosine_scorer_exact`), `manager/scope.rs` (`ResolverRegistry`, `ResolveCtx`, `ScopeResolver`), `manager/streamed.rs` (`CpuIndexManager::{read, read_entries}`), `index/mod.rs`, `index/parquet_io.rs`, `op/mod.rs` (`ScoredHit`, `ScoredHits`).  
**Abhängt von:** L1.1.  
**Zu klären:** (1) Abbildung auf die Kernel-Abstraktion: Segmente und Slots; ein CPU-„Segment“ wäre ein Block von höchstens 8192 Einträgen. (2) Speicher: alle Vektoren als f32 zu materialisieren kostet bei 100k Einträgen etwa 154 MB, deshalb blockweise mit begrenztem Cache. (3) `outdated` ist auf der CPU immer `false` (RAM ist primär). (4) Payload-Zugriff: Der `Index`-Trait hat keine Payload-Methode. Optionen: (a) Default-Methode `fn payload(&self, entry: EntryId) -> Option<Payload> { None }` am Trait, in `ParquetIndex` implementiert; (b) separater Trait `PayloadSource` im `IndexDirectory`; (c) nur der Layer löst Payloads über den konkreten `ParquetIndex` auf (robuster Rückfall, siehe L4.4). (5) `exact` statt reinem Skalarprodukt, weil f16-Speicher die Norm driften lässt (E45).  
**Ergebnis:** `ScorerBridge::score(index, scope, query, top_k) -> ScoredHits`, getestet gegen eine Brute-Force-Referenz; Payload-Option (a) umgesetzt, (c) als Rückfall im Layer vorgesehen.

#### Aufgaben

- Scope auflösen: `ResolverRegistry::with_defaults()`; für `Scope::All`, `Range` und `Set` die direkte Auflösung nutzen, nicht unterstützte Kombinationen laut Charta mit `UnsupportedScope` scheitern lassen (`ParquetIndex::capabilities()` ist `[Range]`; `All` und `Set` lösen direkt auf, E40).
- Blöcke bilden: Kandidaten (aufsteigend nach `EntryId`) in Blöcke teilen; je Block `Index::read` für jeden Eintrag, f16 zu f32 dekodieren (`half`), in einen wiederverwendeten Puffer schreiben.
- Scoring: `CosineSlots::exact` plus `TopKScorer`; Ties bevorzugen die kleinere EntryId (so spezifiziert der Report: „best first; ties prefer the lower entry id“).
- Payload: Option (a) umsetzen und `ScoredHit.source` füllen; die Trait-Ergänzung ist rückwärtskompatibel (Default `None`).
- Konkurrenz: Einträge, die während des Scans entfernt werden, zählen als `discarded`; leere `EntryData` gelten als entfernt.
- Speicherobergrenze: Blockgröße und Gesamtpuffer konfigurierbar; Standard so, dass der Zusatzbedarf unter 64 MB bleibt.

#### Skizze

```rust
pub struct ScorerBridge {
    block_entries: usize,
    resolvers: ResolverRegistry,
}

impl ScorerBridge {
    pub fn score(
        &self,
        index: &(dyn Index + Send + Sync),
        scope: &Scope,
        query: &[f32],
        top_k: usize,
    ) -> Result<ScoredHits, IndexError>;
}

// index/mod.rs: backward compatible default method
pub trait Index {
    // ...
    fn payload(&self, _entry: EntryId) -> Option<Payload> {
        None
    }
}
```

#### Abnahme L1.5

- Test `scorer_matches_bruteforce_for_all_range_set`: identische Treffermenge wie eine naive Referenz, Rangabweichungen nur bei Score-Differenz unter 1e-3 (E45).
- Test `scorer_ties_prefer_lower_entry_id`.
- Test `scorer_blocks_bound_memory`: Spitzenspeicher bleibt bei 100k synthetischen Einträgen unter der Grenze.
- Test `removed_entries_are_discarded_not_failed`.
- Test `unsupported_scope_fails_loudly` mit `Scope::Subtree`.
- Test `payload_is_filled_for_parquet_index`.

### L1.6 · ResidentRuntime (Zusammenbau)

**Zweck:** Ein Aufruf liefert ein fertig verdrahtetes CPU-`Resident` samt Modellbeschreibung und Index-Fingerabdruck.  
**Quellen:** `facade/resident.rs` (`ResidentBuilder`), `planner/{mod,rules,offers,adapter}.rs` (`Planner`, `RuleSet::new`, `EngineOffers`, `PlannerExpander`), `processor/config.rs` (`WorkerConfig`), `model/mod.rs`, `index/schema.rs` (`tokenizer_fingerprint`, `SchemaHeader`).  
**Abhängt von:** L1.1 bis L1.5.  
**Zu klären:** `ModelId` (Wert `spec.model_id`, „intfloat/multilingual-e5-small“); Defaults `WorkerConfig::new(1, 0)`; `candle_rayon_threads` (D15); Fingerabdruck der Modellrevision (Hash von `config.json` plus Größe von `model.safetensors`) und `tokenizer_fingerprint(tokenizer.json)`; Fehler bei `init()` (`ConfigError`).  
**Ergebnis:** `ResidentRuntime::start(RuntimeConfig)`, Zugriff auf `Resident`, `ModelSpec`, `ModelFingerprint`, `register_index`.

#### Aufgaben

- `RuntimeConfig` (Modellverzeichnis, `WorkerConfig`, Task-Timeout, TTLs) mit Validierung (`ModelSpec::validate`, vorhandene Dateien, `max_seq_len` im Positionsbereich des Modells).
- Planner und Expander zusammensetzen: `RuleSet::new(default_model, pipelines)` mit der e5-Pipeline aus L1.2, `EngineOffers::new().with_cpu([Tokenizer, Encoder, Scorer, Index])`, `PlannerExpander::new`.
- `Resident::builder().model(...).worker_hint(...).expander(...).engines(cpu, None).index_directory(...).init()`.
- `ModelFingerprint { model_id, revision, tokenizer_sha }` berechnen (liest `config.json` und `tokenizer.json`, kein Laden der Gewichte).
- `register_index(Arc<dyn Index + Send + Sync>) -> IndexId` kapselt `add_index`.
- Lebenszyklus: `Drop` joint Worker (K4); `ResidentRuntime` bietet `shutdown(self)` und dokumentiert, dass es nicht auf dem Async-Thread bei laufenden Tasks gedroppt werden soll.

#### Skizze

```rust
pub struct RuntimeConfig {
    pub model_dir: PathBuf,
    pub workers: WorkerConfig,
    pub task_timeout: Option<Duration>,
    pub idle_ttl: Duration,
}

pub struct ModelFingerprint {
    pub model_id: String,
    pub revision: String,
    pub tokenizer_sha: String,
}

pub struct ResidentRuntime {
    resident: Resident,
    directory: Arc<IndexDirectory>,
    spec: ModelSpec,
    fingerprint: ModelFingerprint,
}

impl ResidentRuntime {
    pub fn start(cfg: RuntimeConfig) -> Result<Self, ResidentError>;
    pub fn resident(&self) -> &Resident;
    pub fn spec(&self) -> &ModelSpec;
    pub fn fingerprint(&self) -> &ModelFingerprint;
    pub fn register_index(&self, index: Arc<dyn Index + Send + Sync>) -> IndexId;
}
```

#### Abnahme L1.6

- Test `start_without_model_files_fails_with_config_or_model_error` (kein Panic).
- Test `start_is_cheap`: `start` lädt weder Tokenizer noch Gewichte (Ladezähler gleich null).
- Test `fingerprint_is_stable_and_changes_with_inputs`.
- Test `runtime_drop_joins_workers`.

### L1.7 · Abnahme-Tests der Runtime

**Zweck:** Die Runtime gegen unabhängige Referenzen prüfen, bevor der Layer sie nutzt.  
**Quellen:** `tests/data/reference/multilingual-e5-small.json`, `resident/model/test/candle_adapter.rs` (`assert_matches_reference`, `installed_models`), `resident/kernel/golden` (`Tolerance`, `check`), Konventionen Q3.  
**Abhängt von:** L1.1 bis L1.6.  
**Zu klären:** Welche Rolle/Präfixe erzeugt die Referenzdatei (so wählen, wie `assert_matches_reference` es tut)? Ab welcher Dauer gilt ein Modelltest als zu lang für den Default-Lauf?  
**Ergebnis:** Test-Suite `resident/runtime/test/*` plus Integrationstest `tests/resident_runtime.rs`.

#### Tests

- `embedding_matches_reference_on_cpu` (Modell erforderlich): Cosinus gegen die Referenz mindestens 0,9999 für alle Referenztexte, Ausführung über die Fassade.
- `query_and_passage_roles_differ` (Modell erforderlich) und `role_selects_prefix` (ohne Modell).
- `similarity_matches_bruteforce_on_inmemory_index`, `similarity_matches_bruteforce_on_parquet_index`, `parquet_roundtrip_via_runtime` (ohne Modell: eigene Vektoren als `VectorInput`).
- `scope_all_range_set_and_unsupported`.
- `concurrent_first_tasks_load_resources_once`, `load_failure_backs_off`.
- `task_timeout_fails_running_task`, `dropped_handle_cancels_queued_task`.
- `op_panic_fails_the_task_instead_of_hanging`.
- `shutdown_joins_workers_within_timeout`.
- `embedding_batch_order_is_preserved` (Texte stark unterschiedlicher Länge).
- `empty_input_is_an_error_not_a_panic`, `oversized_input_is_truncated_and_stays_normalised`.
- `api_hygiene` und `deps_smoke` bleiben grün.

#### Aufruf

```bash
cd /home/user/xyan/xy.ai.workbench/rag
cargo test --features candle resident::runtime
cargo test --features candle --test resident_runtime
cargo test --features candle,mock resident::runtime
```

## Gate G1

- `ResidentRuntime::start` liefert ein lauffähiges CPU-`Resident`; Embedding und Similarity laufen über die Fassade Ende zu Ende.
- Referenz-Cosinus mindestens 0,9999 gegen `multilingual-e5-small.json` (Modelltests; ohne Modell sind sie `ignored` und die modellfreien Tests grün).
- Similarity-Top-k stimmt mit der Brute-Force-Referenz überein (Toleranz 1e-3 wegen f16).
- Rolle zu Präfix ist getestet; Pipeline und `ModelSpec` sind konsistent.
- Weder Hänger noch Datenverlust bei Cancel, Timeout, Panic und Shutdown.
- Keine neuen `cfg(feature)` außerhalb der erlaubten Stellen, `api_hygiene` grün, kein Candle-Typ in öffentlichen Signaturen.

## Risiken und Stolpersteine

- `ResourceCell::get_or_init` ist async, die Worker sind synchrone Threads: Falsche Brücke führt zu Deadlocks oder Runtime-Abhängigkeit. Früh mit einem Prototyp-Test absichern.
- Candle-CPU nutzt rayon; zusätzliche Worker und der Layer-Pool können die Kerne überbuchen (D15, R4 im Masterplan). `candle_rayon_threads` wird im Quelltext bisher nirgends ausgewertet (E50); ob Candle den globalen Pool nutzt, ist **[?]**.
- Block-Dekodierung f16 zu f32 ist der Heißpfad der Anfrage; ohne SIMD-freundliche Schleifen (`half`-Batch-Konvertierung) wird die Latenz dominiert. In M4 messen.
- `ParquetIndex::read` dekodiert beim ersten Zugriff die gesamte Datei (`Base::data`); der erste Query-Zugriff trägt diese Kosten. Im Budget B1/B2 berücksichtigen.
- `Resident` droppen blockiert (Worker-Join); niemals auf dem Async-Thread bei laufenden Tasks.
- Änderungen am `Index`-Trait und an `OpParams` berühren Engine-Tests und eventuell den GPU-Pfad; GPU-CI-Job (`cuda`) vorab auf Kompatibilität prüfen (Trait-Erweiterung ist rückwärtskompatibel, `OpParams`-Feld nicht für Struct-Literale).
