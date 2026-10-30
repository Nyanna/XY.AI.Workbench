Erzeuge aus dem folgenden Korpus ein hierarchisches Komponentenmodell, das prinzipiell alle Anforderungen aufnehmen kann.

- YAML Output, eine flache Liste; Felder: Name, Parent, Responsibility(kompakte textuelle Beschreibung), Interfaces(kompakte textuelle Beschreibung)
- Zusätzlich eine kurze Darstellung der Hierarchie mittels Einrückung

Material:
[include](/home/user/xyan/xy.ai.workbench/project/resident2/runs/plan7.md)
Result Stats: id=6ebaf073-79b9-4507-a0fe-1eebc5f46aba, total: 98741, in: 65221, out: 33520, reason: 8171, read: 0, write: 0, start: 1793387035065, end: 1793387240502, time: 204806, total: 204806, io: 323, control: 631, sent: 1, recv: 4365
SystemInit:  id=6ebaf073-79b9-4507-a0fe-1eebc5f46aba, cwd=/home/user/xyan/xy.ai.workbench/project, model=claude-sonnet-5-5
ReasoningToken: 8171
# Komponentenmodell Resident

## Hierarchie

```
Resident
├─ Host
├─ Engine
│  ├─ EngineBuilder
│  ├─ IndexRegistry
│  ├─ ShutdownCoordinator
│  ├─ Processor
│  │  ├─ TaskQueue
│  │  ├─ OperationQueues
│  │  ├─ WorkerPool
│  │  ├─ WorkerLoop
│  │  ├─ NotificationHub
│  │  ├─ IdleNotificationSet
│  │  └─ GpuCompletionHandler
│  ├─ Planner
│  │  ├─ CapabilityLedger
│  │  ├─ ModelSelector
│  │  ├─ Expander
│  │  ├─ ExpansionRules
│  │  ├─ MaterializationPolicy
│  │  ├─ ResultCollector
│  │  └─ Operation
│  │     ├─ OperationKind
│  │     ├─ OperationParams
│  │     ├─ ResourceRequirements
│  │     ├─ OperationNode
│  │     ├─ ContinuationHandler
│  │     ├─ RamDataForms
│  │     └─ DevicePin
│  ├─ Task
│  │  ├─ TaskContext
│  │  ├─ TaskHandle
│  │  ├─ ParamExtensions
│  │  ├─ EmbeddingTask
│  │  ├─ SimilarityTask
│  │  ├─ TaskResult
│  │  └─ SimilarityReport
│  └─ Subengine
│     ├─ SubengineCommon
│     │  ├─ SubengineContract
│     │  ├─ PullLoop
│     │  ├─ ResourceRegistry
│     │  ├─ LaneBatcher
│     │  ├─ BatchComputeBackend
│     │  ├─ Model
│     │  │  ├─ ModelDescriptor
│     │  │  ├─ ModelRegistry
│     │  │  ├─ LoadedEncoder
│     │  │  ├─ TransformerBackbone
│     │  │  ├─ ProjectionLayer
│     │  │  ├─ PoolingNormalization
│     │  │  ├─ EmbeddingWriter
│     │  │  ├─ EncoderInterface
│     │  │  └─ WeightSource
│     │  ├─ IndexManagerCore
│     │  │  ├─ ScopeResolver
│     │  │  ├─ Reconciler
│     │  │  ├─ ResidencyDirectory
│     │  │  ├─ EvictionPolicy
│     │  │  └─ ConsistentSnapshot
│     │  └─ Scoring
│     │     ├─ QueryBatch
│     │     ├─ DenseScorer
│     │     ├─ MaxSimOperation
│     │     ├─ TopKSelector
│     │     └─ ReferenceComparator
│     ├─ CpuSubengine
│     │  ├─ CpuDevice
│     │  ├─ CpuMemoryBackend
│     │  ├─ CpuKernelRegistry
│     │  ├─ CpuScorer
│     │  ├─ Tokenizer
│     │  │  ├─ TextNormalizer
│     │  │  └─ BatchBuilder
│     │  └─ CpuIndexManager
│     │     ├─ PageCacheAdvisor
│     │     └─ BlockingReader
│     └─ GpuSubengine
│        ├─ GpuDevice
│        ├─ VramManager
│        │  ├─ GpuMemoryBackend
│        │  ├─ StagingRing
│        │  └─ TensorRegionWrapper
│        ├─ GpuKernelRegistry
│        ├─ GpuKernels
│        ├─ LaunchModes
│        ├─ WorkChunker
│        └─ GpuIndexManager
│           ├─ SegmentStore
│           ├─ SegmentArena
│           ├─ ManifestPublication
│           ├─ TableSwap
│           ├─ DoorbellSync
│           ├─ CommunicationSlot
│           ├─ AddressMapping
│           ├─ UpdateQueue
│           ├─ ResidencyLoader
│           ├─ DeviceTransfer
│           ├─ IndexBuildWriter
│           └─ IndexOpsCoordinator
├─ Common
│  ├─ Queue
│  │  ├─ QueueNode
│  │  └─ QueueIterator
│  ├─ Devices
│  │  ├─ Device
│  │  ├─ Lane
│  │  └─ Event
│  ├─ Memory
│  │  ├─ RegionLayout
│  │  ├─ Arenas
│  │  ├─ OffsetAllocator
│  │  ├─ BookkeepingAllocator
│  │  ├─ DeferredFreeAllocator
│  │  ├─ MemoryBudget
│  │  └─ MemoryBackend
│  ├─ Index
│  │  ├─ IndexContract
│  │  ├─ ChangeStamp
│  │  ├─ EntrySetScope
│  │  ├─ ReferenceIndex
│  │  ├─ FileIndex
│  │  ├─ IndexHeader
│  │  └─ ChecksumChangeDetection
│  ├─ Precision
│  │  ├─ CapabilityModel
│  │  ├─ CapabilityDetection
│  │  ├─ BenchmarkWorkload
│  │  ├─ Measurer
│  │  ├─ Calibrator
│  │  ├─ ParetoProfiles
│  │  ├─ ProfileSelector
│  │  ├─ CalibrationStore
│  │  └─ BackgroundPool
│  ├─ Adapters
│  │  ├─ CandleAdapter
│  │  ├─ SafetensorsAdapter
│  │  ├─ ParquetArrowAdapter
│  │  └─ CudaAdapter
│  └─ Diagnostics
└─ Addon
```

## YAML (flache Liste)

```yaml
components:
  - Name: Resident
    Parent: null
    Responsibility: "Wurzel; dauerhaft laufende Engine für Embedding und Similarity. Nach Init bereit, aber ohne Ressourcenverbrauch (Lazy)."
    Interfaces: "Gestartet von RAG-Anwendung (Server, Daemon, CLI); delegiert nach außen nur über Engine."

  - Name: Host
    Parent: Resident
    Responsibility: "Einstiegspunkte (Server-Prozess, Daemon, CLI), die Engine instanziieren, konfigurieren, starten und beenden."
    Interfaces: "nutzt Engine-Fassade; bietet Prozess-/CLI-/Server-Schnittstelle an die RAG-Anwendung."

  - Name: Engine
    Parent: Resident
    Responsibility: "Fassade nach außen; konfiguriert Planner mit Subengines, Modellen, Indizes; Modell-/Device-Ressourcen entstehen erst durch den ersten Task."
    Interfaces: "Builder->start; enqueue_embedding/enqueue_similarity -> TaskHandle; register_index; shutdown."

  - Name: EngineBuilder
    Parent: Engine
    Responsibility: "Sammelt Konfiguration (Modell verpflichtend, Subengines, Indizes); wirksam erst bei explizitem Start; fehlendes Modell -> Konfigurationsfehler mit Feldnamen."
    Interfaces: "Setter je Konfigurationsfeld; start() -> Engine | Konfigurationsfehler."

  - Name: IndexRegistry
    Parent: Engine
    Responsibility: "Registriert beliebig viele durchsuchbare Indizes mit eindeutiger ID; Registrierung lädt nicht (Zustand 'nicht geladen')."
    Interfaces: "register(index) -> IndexId; lookup(IndexId); Zugriff für Planner/Manager."

  - Name: ShutdownCoordinator
    Parent: Engine
    Responsibility: "Geordnetes Shutdown (auch bei Drop): Worker anhalten und joinen, nicht übernommene Tasks abbrechen (Fehler 'cancelled')."
    Interfaces: "shutdown() -> Erfolgsmeldung; Drop-Hook."

  - Name: Processor
    Parent: Engine
    Responsibility: "Orchestrierung: besitzt Task-Queue, Operations-Queues, Worker-Pool, Notifications; ruft Planner für Expansion; Backpressure ergibt sich aus blockierenden Workern."
    Interfaces: "nimmt Tasks an; stellt Queues/Notifications den Subengines bereit; ruft Planner.expand."

  - Name: TaskQueue
    Parent: Processor
    Responsibility: "Queue-Instanz für eingehende Tasks (eigenes Set an Notification-Channels)."
    Interfaces: "Queue-API (enqueue, iterate, claim, cancel)."

  - Name: OperationQueues
    Parent: Processor
    Responsibility: "Drei Queue-Instanzen (CPU, GPU, gemeinsam) für Operationen; Subengine prüft erst eigene, dann gemeinsame Queue; keine Head-of-Line-Blockade."
    Interfaces: "Queue-API; Producer: Planner/Continuations; Consumer: PullLoop der Subengines."

  - Name: WorkerPool
    Parent: Processor
    Responsibility: "Interner Executor: Mindestzahl idle Threads (Worker = CPU-Cores minus 1, kein Pinning); Parkplatz der Worker, Wakeup per Notification."
    Interfaces: "start(min_threads); park/wake; Worker stellen CPU-Lanes."

  - Name: WorkerLoop
    Parent: Processor
    Responsibility: "Prioritätsfolge nach jeder Operation: GPU-Ergebnisse/Mirror-Back, GPU füttern, CPU füttern/ausführen, neue Tasks expandieren."
    Interfaces: "nutzt Subengines, Queues, GpuCompletionHandler, Planner."

  - Name: NotificationHub
    Parent: Processor
    Responsibility: "Zentrale Notification: weckt Worker und Subengine-Loops bei jeder Zustandsänderung (Append, Ergebnis, Init-Ende, Eviction-Timer)."
    Interfaces: "notify(); wait(); Channels je Subengine."

  - Name: IdleNotificationSet
    Parent: Processor
    Responsibility: "Set (keine Queue) je Subengine-Art; Iteration ohne Treffer bei freier Ressource erzeugt Idle-Notification (Ressourcenart, Parameter); verfällt bei Treffer/Belegung; steuert bevorzugte Expansion."
    Interfaces: "post(kind, params); take/peek je Subengine-Art; Input für Expander."

  - Name: GpuCompletionHandler
    Parent: Processor
    Responsibility: "Verarbeitet GPU-Completions: Stream-Callback legt nur Eintrag ab und notifiziert; zusätzlich Event-Query mit atomarem Lane-Swap gegen verpasste Callbacks."
    Interfaces: "Callback-Eingang (ohne GPU-API-Aufruf); poll_events(); Ergebnisweitergabe an Task/Index."

  - Name: Planner
    Parent: Engine
    Responsibility: "Nimmt Tasks an, lässt sie expandieren, ordnet Operationen passenden Queues zu (gemeinsame bevorzugt); aktiviert Subengines blockierend (nur Aktivierung); Fehler 'fehlende Planungs-Capability'."
    Interfaces: "plan(task) -> erste Operation + Collector; Zuordnung an OperationQueues; Zugriff auf Modelle, Subengine-Angebote, Idle-Notification."

  - Name: CapabilityLedger
    Parent: Planner
    Responsibility: "Verwaltet je Subengine die angebotenen Ressourcenarten (schrittweise erweiterbar); unterscheidet verfügbar (registriert) von geladen."
    Interfaces: "register_offer(subengine, kind); compatible(requirements) -> Queues; first_resource_kind(task_kind)."

  - Name: ModelSelector
    Parent: Planner
    Responsibility: "Wählt Default-Modell bzw. aus Index abgeleitetes Modell; 'unbekanntes Modell' bei fehlender Konfiguration oder fehlendem Expander."
    Interfaces: "select(task) -> ModelDescriptor | Fehler."

  - Name: Expander
    Parent: Planner
    Responsibility: "Reine, zustandslose Funktion: zerlegt Task genau einmal in geordnete Operationskette; übersetzt Parameter in Operationskonfiguration; lehnt fremde Task-Arten ab; loggt Art und Anzahl."
    Interfaces: "expand(task, pipeline_config, capabilities, idle_notification) -> Kette von Operationen | Fehler."

  - Name: ExpansionRules
    Parent: Planner
    Responsibility: "Regeln: Embedding (Tokenisierung, Embedding, Pooling, L2, Index-Schreiben), Similarity (Berechnung, Top-k; Top-k 0 abgelehnt), Cross-Encoder-Variante; Input-/Ergebnisaufbereitung."
    Interfaces: "Regelsatz je Modell und Task-Art; Pipeline-Konfiguration (Präfixe, Pooling, Normalisierung, max. Länge, Datentyp)."

  - Name: MaterializationPolicy
    Parent: Planner
    Responsibility: "Setzt Materialization-Flag je Operation: Ergebnis bleibt nur im VRAM, wenn diese und die Folgeoperation explizit auf der GPU laufen; sonst Rückholung ins RAM."
    Interfaces: "decide(op, next_op) -> Flag; Ausführung durch Operation."

  - Name: ResultCollector
    Parent: Planner
    Responsibility: "Schrittweise Übergabe (nur erste Operation + Collector-Funktion) und Ergebnisaufbereitung: Embedding-Splitting, Trefferliste, Report, Scores; take-Semantik; Fehler bei Typ-/Wertemismatch."
    Interfaces: "collect(final_output) -> TaskResult | Fehler (genau einmal entnehmbar)."

  - Name: Operation
    Parent: Planner
    Responsibility: "Berechnungseinheit eines Tasks mit Art, Parametern, Requirements; explizite Input-/Output-Regionen (RAM-Daten oder Device-Pin, nie Tensoren/Pointer); thread-sicher nutzbar."
    Interfaces: "kind(); params(); requirements(); descriptor() (vergleichbar, Key); Task-ID, Priorität."

  - Name: OperationKind
    Parent: Operation
    Responsibility: "Erweiterbare Operationsarten: Tokenisierung, Embedding, Pooling, L2, Similarity, Top-k, Index-Schreiben, Inferenz, Device->RAM-Übertragung."
    Interfaces: "Enum/Registry; Descriptor (Art + Parameter)."

  - Name: OperationParams
    Parent: Operation
    Responsibility: "Immutable Parameter: Resource-Requirements, optional Top-k und Dimension; vergleichbar, als Cache-/Graph-Key und für Pull-Filter vor dem Claim."
    Interfaces: "Eq/Hash; Filter-Zugriff."

  - Name: ResourceRequirements
    Parent: Operation
    Responsibility: "Ressourcenart (Tokenizer, Normalizer, Encoder, Scorer, Index, Cross-Encoder), Modell-/Index-ID, Execution-Umgebung (CPU-only, GPU-only, any), Capabilities, optionaler Placement-Hint (nicht Teil der Erfüllung)."
    Interfaces: "satisfied_by(device) (Art + Capabilities); Eq/Hash; schrittweise Ergänzung."

  - Name: OperationNode
    Parent: Operation
    Responsibility: "Bündelt Operation, Input, einmal setzbaren Output, Materialization-Flag, Task-Bezug, optionalen Continuation-Handler; loggt Anlage."
    Interfaces: "set_output (einmal); attach_continuation; Zugriff auf TaskContext."

  - Name: ContinuationHandler
    Parent: Operation
    Responsibility: "Erzeugt aus Output und Task-Kontext null oder mehr Folgeknoten; ohne Folgeknoten wird Output Task-Ergebnis; nebenläufig nutzbar."
    Interfaces: "on_complete(output, ctx) -> [OperationNode]."

  - Name: RamDataForms
    Parent: Operation
    Responsibility: "Erweiterbare RAM-Datenformen: Textliste, Tokenfolgen, Float-Folge, Trefferliste (Entry-ID, Score), gescortes Trefferergebnis (Zwischenergebnis mit Zählern)."
    Interfaces: "Typen als Input/Output der Operationen."

  - Name: DevicePin
    Parent: Operation
    Responsibility: "RAII-Zugriff auf Device-Region; Pin-Count verhindert Eviction; optionaler Rückmeldekanal (Fehler bei fehlendem Empfänger unkritisch); auch als Device-Handle verfügbar."
    Interfaces: "pin(region) -> Pin; count(); is_unpinned(); drop verringert Zähler."

  - Name: Task
    Parent: Engine
    Responsibility: "Kapselt Daten, Parameter, Ergebnis, Zustand (enqueued, claimed, completed, cancelled) und Operationen; Priorität/Ressource/Modell/Modus sind Metadaten, der Queue unbekannt."
    Interfaces: "Parameterobjekt, Ergebnis genau einmal setzbar (Output | Fehler), Notification für Wartende."

  - Name: TaskContext
    Parent: Task
    Responsibility: "Geteilter Kontext je Task: ID, Priorität, Zähler offener Operationen, Cancel-Flag, erster Fehler; Folgeoperationen werden vor Abschluss der erzeugenden registriert; erster Fehler cancelt den Task."
    Interfaces: "register_followup; complete_op -> war letzte?; fail(err) -> war erster?; Debug-Check bei Abschluss ohne offene Operation."

  - Name: TaskHandle
    Parent: Task
    Responsibility: "Aufrufer-Handle: Polling, Warten (unbegrenzt/Timeout, Registrierung vor Zustandslesen), Cancel bei Drop falls nicht geclaimt; Timeout liefert Fehler 'timeout'."
    Interfaces: "poll() -> Ergebnis | noch keins | cancelled; wait(); wait_timeout(d); drop-Cancel."

  - Name: ParamExtensions
    Parent: Task
    Responsibility: "Typindizierte Extension-Map im Parameterobjekt (ein Wert je Typ, Send+Sync), geteilt beim Klonen; für Addons."
    Interfaces: "set<T>, get<T>, remove<T> -> war vorhanden, len, is_empty."

  - Name: EmbeddingTask
    Parent: Task
    Responsibility: "Text(e) -> Embeddings; Parameter: Modell, Modus (Query/Passage), Precision, optionaler Ziel-Index; kann direkt in Index schreiben und liefert dann Entry-Referenzen."
    Interfaces: "Input: Text | Textliste; Ergebnis: Float-Vektoren je Text oder Entry-IDs."

  - Name: SimilarityTask
    Parent: Task
    Responsibility: "Ähnlichkeitssuche für Query-Vektoren; Parameter: Index-ID, Top-k (Default 10), Scope (Default alle), Scoring (Default Cosine), Precision; Reranking als zweite Similarity mit verengtem Scope."
    Interfaces: "Input: Query-Vektoren; Ergebnis: Treffer (Entry, Score, stale) bzw. Report."

  - Name: TaskResult
    Parent: Task
    Responsibility: "Erweiterbare Ergebnisarten: Embeddings, Entry-Referenzen, Trefferliste, Similarity-Report, Score-Liste (Cross-Encoder); vergleichbar."
    Interfaces: "Enum mit Eq; genau eine Art je Task."

  - Name: SimilarityReport
    Parent: Task
    Responsibility: "Liefert Fakten statt Entscheidungen: Zähler (Kandidaten, verworfen, stale), sortierte Rangliste mit Source-Link, Kennzahlen (Gap, Spread, z-Score, Softmax, Entropie, Knee-Point u.a.); keine Thresholds."
    Interfaces: "build(scored_hits, params, source_lookup) -> Report | Fehler bei fehlenden Suchparametern; Kennzahlen optional (nicht vorhanden bei zu wenigen Treffern)."

  - Name: Subengine
    Parent: Engine
    Responsibility: "Ressourcen-, Speicher- und Ausführungsverwaltung je Hardwareart; besitzt Speicher, Streams, Kernel; Pull-Prinzip; Schnittstelle für Planner/Processor."
    Interfaces: "capabilities(); activate(); registriert Ressourcen/Lanes in Queues; load report (Art, Device, Kapazität, Durchsatz)."

  - Name: SubengineCommon
    Parent: Subengine
    Responsibility: "Gemeinsame, CPU-seitig liegende Bausteine beider Subengines: Contract, Pull-Loop, Ressourcenregister, Batching, Modell, Manager-Kern, Scoring."
    Interfaces: "Wiederverwendung durch CpuSubengine und GpuSubengine."

  - Name: SubengineContract
    Parent: SubengineCommon
    Responsibility: "Gemeinsame Regeln/Invarianten: Capability-Set, Lazy-Ressourcen, Kalibrierung und Precision-Profile, hardwareoptimale Pfade, keine Tensoren/Raw-Pointer in Signaturen, GPU-Arbeit nur über Lanes, Kernel nur über Register; Auslastungsmeldung (geglätteter Durchsatz)."
    Interfaces: "capability_set(); throughput(); kind(); device(); capacity()."

  - Name: PullLoop
    Parent: SubengineCommon
    Responsibility: "Loop je Subengine (Wartebedingung: Notification) mit Iteration über Queue in Altersreihenfolge: Filter, Ressourceneintrag, Lazy-Init-Anstoß (uninitialized->loading), Claim und Ausführung; Eviction ungenutzter Ressourcen am Ende."
    Interfaces: "nutzt OperationQueues, ResourceRegistry, NotificationHub, IdleNotificationSet."

  - Name: ResourceRegistry
    Parent: SubengineCommon
    Responsibility: "Verwaltet Ressourcen (Tokenizer, Encoder, Scorer, Index-Segmente, Kernel) mit Zuständen uninitialized, loading, ready, backoff; TTL-Eviction (VRAM deutlich länger als RAM); Fehler -> Batch-Fehler + Backoff."
    Interfaces: "lookup(requirements); try_init; try_acquire (Kapazität); evict_idle; Key z.B. Architektur-ID + Device-Klasse."

  - Name: LaneBatcher
    Parent: SubengineCommon
    Responsibility: "Lanes ziehen Operationen und bilden Batches (vorwärts durch die Liste, alles Passende aufnehmen); Packing, Length-Bucketing, Streams sind Ressourcenlogik; Token-Budget/Deadline statt Task-Anzahl."
    Interfaces: "form_batch(start_op, filter, limits) -> Batch; Ergebnis löst Subengine-Notification aus."

  - Name: BatchComputeBackend
    Parent: SubengineCommon
    Responsibility: "Austauschbare Schnittstelle Float-Batch -> Float-Batch; CPU-Implementierung als Default/Fallback, GPU-Implementierung als einzige Anbindung der GPU-Library (Anlegen scheitert ohne GPU -> Fallback)."
    Interfaces: "trait compute(batch) -> batch; create_gpu() -> Fehler ohne kompatible GPU."

  - Name: Model
    Parent: SubengineCommon
    Responsibility: "Gemeinsame Modell-Ressource (Host): beschreibt Pipeline, besitzt nichts; Register verknüpft Architektur-IDs mit Encoder-Factories und Scoring-Arten mit Scorern; neue Modelle = neuer Registereintrag."
    Interfaces: "descriptor(); load(device_class) -> LoadedEncoder; Multi-Vector-Parameter aus Checkpoint-Konfiguration."

  - Name: ModelDescriptor
    Parent: Model
    Responsibility: "Ressourcenlose Beschreibung: IDs, Dimension, Pooling, Normalisierung, Präfixe, Storage-/Compute-Datentypen je Device-Klasse, max. Länge, Projection, Pfade, Multi-Vector; Default für multilingual-e5-small; Validierung (verhindert stille Qualitätsverluste)."
    Interfaces: "default_e5_small(dir, dtype); validate() -> 'inkompatible Modellkonfiguration'."

  - Name: ModelRegistry
    Parent: Model
    Responsibility: "Architektur-ID -> Encoder-Factory (Neuregistrierung ersetzt; 'bert' nur mit Feature-Flag); Laden: validieren, Factory suchen, Encoder bauen; 'nicht unterstützte Architektur'."
    Interfaces: "register(id, factory); with_builtin(); load(descriptor, device_class)."

  - Name: LoadedEncoder
    Parent: Model
    Responsibility: "Geladener Encoder (Architektur + Gewichte auf Device) bündelt Beschreibung, Device-Klasse und Encoder; verwaltbare Ressource (Key, Init-Kosten, Freigeben entlädt)."
    Interfaces: "descriptor(), device_class(), encoder(); ManagedResource (key, cost, release)."

  - Name: TransformerBackbone
    Parent: Model
    Responsibility: "Hauptmodell (BERT; ColBERT-artig mit Backbone + Projection-Head): Config-Validierung, Gewichtsladen per mmap, Forward-Pass; Austausch nur als einfache Zahlenlisten."
    Interfaces: "forward(token_ids, mask) -> flache FP32-Hidden-States; output_dim(); Fehler: inkompatibel, Gewichtsname passt nicht, Encoding fehlgeschlagen."

  - Name: ProjectionLayer
    Parent: Model
    Responsibility: "Optionale lineare Abbildung je Token-Vektor (z.B. 768->128) für Late Interaction; Weight/Bias aus safetensors, Shape-/Typprüfung."
    Interfaces: "load(source, tensor_name); apply(vectors); input_dim(), output_dim()."

  - Name: PoolingNormalization
    Parent: Model
    Responsibility: "Masked Mean oder kein Pooling; L2-Normalisierung (Epsilon-Clamp, Zählung degenerierter Vektoren), einmal am Encoder-Ende; Query und Dokument gleich behandelt."
    Interfaces: "pool(hidden, mask); l2(vectors) -> (vectors, degenerate_count)."

  - Name: EmbeddingWriter
    Parent: Model
    Responsibility: "Schreibt Ergebnisvektoren normalisiert und datentypkonvertiert lückenlos in Output-Region (FP32/FP16/BF16 LE); Warnung bei Nullvektoren; GPU: dünner Write-Kernel in vergebenen Slot."
    Interfaces: "write(vectors, output_region, dtype) -> Erfolg | Memory-Error."

  - Name: EncoderInterface
    Parent: Model
    Responsibility: "Asynchrone Encoder-Schnittstelle: prüft Input-/Output-Region und Layout vorab, läuft auf übergebener Lane, liefert Completion-Event; Ausführungsfehler im Event hinterlegt."
    Interfaces: "encode(input_region, output_region, lane) -> Event; output_dim(); storage_dtype()."

  - Name: WeightSource
    Parent: Model
    Responsibility: "Gewichte aus safetensors per mmap/zero-copy; Kopie in die Arena der jeweiligen Subengine."
    Interfaces: "tensor(name) -> Rohbytes + Typ; über SafetensorsAdapter."

  - Name: IndexManagerCore
    Parent: SubengineCommon
    Responsibility: "Gemeinsames Manager-Verhalten je Subengine: Lazy Attach, Once-Aktivierung, Entry->Segment/Slot-Zuordnung, Residency, Updates; Index bleibt Source of Truth."
    Interfaces: "attach(index) -> bool; activate(init); is_active(); resident_count(); Fehler 'Index nicht verfügbar'."

  - Name: ScopeResolver
    Parent: IndexManagerCore
    Responsibility: "Löst Scope in kompakte Entry-Menge auf (all, set direkt; range, subtree, prefix, centroid über registrierte Resolver); kein stilles Ausweichen auf 'alle'; 'Scope nicht unterstützt'."
    Interfaces: "register_resolver(method) (Duplikat abgewiesen); resolve(scope, ctx) -> EntrySet."

  - Name: Reconciler
    Parent: IndexManagerCore
    Responsibility: "Zustandsbasierter Abgleich je Subscriber (GPU-Manager, Persistenz, CPU-Manager): Signal quittieren, residente Entries per Stamp vergleichen, Entfernte/Geänderte ermitteln; Echo-Suppression; Laden Stamp-Daten-Stamp mit max. 8 Retries."
    Interfaces: "wait_signal(); reconcile() -> {removed, stale, current, watermark, clean}; has_pending()."

  - Name: ResidencyDirectory
    Parent: IndexManagerCore
    Responsibility: "Verzeichnis je residentem Entry: Slot (Segment, Slot-Nr.) und bekannter Stamp."
    Interfaces: "insert (liefert Vorgänger), remove, get, stamp_of, len, iter."

  - Name: EvictionPolicy
    Parent: IndexManagerCore
    Responsibility: "Explizite Eviction von VRAM-/RAM-Mirrors: Idle-Timeout, High/Low-Watermark mit Hysterese, GreedyDual-Size-Frequency-Priorität, Tiers (Summary/Pinned nie Victim); Gründe Idle, Pressure, Space-Need."
    Interfaces: "track(key,size,tier,cost); touch; remove; tick(); make_space(bytes) | 'Arena erschöpft'; Logging je Entscheidung."

  - Name: ConsistentSnapshot
    Parent: IndexManagerCore
    Responsibility: "Seqlock-artiges, wortweise atomares Lesen gleichzeitig veränderbarer Daten; vollständige Kopie plus Retry-Zähler; Writer schließen sich aus."
    Interfaces: "read() -> (Kopie, retries); write(region)."

  - Name: Scoring
    Parent: SubengineCommon
    Responsibility: "Gemeinsame Scoring-Schicht: Query-Batches, Dichtes Cosine/Dot-Scoring, Max-Similarity (Late Interaction), Top-k, Referenzvergleich; Datentypen je Device-Art (CPU FP32, GPU FP32/FP16)."
    Interfaces: "score(query_batch, slot_range, source) -> Top-k je Query; Fehler: Shape-Error, leeres Embedding."

  - Name: QueryBatch
    Parent: Scoring
    Responsibility: "Bündelt Queries gleicher Dimension (Token x Dimension), prüft Shape; Dokumentvektoren über Source-Schnittstelle (RAM-Source mit Slot-Offsets); Workspace = halboffener Slot-Bereich."
    Interfaces: "QueryBatch::new(...); Source: dimension, slots_per_segment, slot_offset, vectors(slot)."

  - Name: DenseScorer
    Parent: Scoring
    Responsibility: "Cosine als Dot-Product (Einheitsvektoren) oder exakt mit Normprodukt; ein Token je Query; Score je Slot in Slot-Reihenfolge; Prüfung auf leeres Embedding."
    Interfaces: "score_dense(queries, workspace) -> Scores; Top-k-Variante."

  - Name: MaxSimOperation
    Parent: Scoring
    Responsibility: "Einheitliche Max-Similarity-Operation mit austauschbarer Implementierung: Tile-Kette (GEMM, Mask, Max, Summe) auf CPU/GPU in wählbarer Precision oder fused Kernel; Mask-Tokens der Query zählen mit."
    Interfaces: "maxsim(queries, slot_range, precision, tile) -> Top-k je Query; Fehler 'nicht verfügbar', Launch-Error."

  - Name: TopKSelector
    Parent: Scoring
    Responsibility: "Begrenzte Top-k-Menge (höherer Score, bei Gleichstand niedrigerer Offset; nicht numerische verworfen); lokale Reduktion je Workspace, Merge auf der CPU; Range-Scorer über mehrere Bereiche."
    Interfaces: "push(offset,score); extend; into_sorted(); merge(partials)."

  - Name: ReferenceComparator
    Parent: Scoring
    Responsibility: "Vergleicht Scorer gegen Referenz-Scorer über dieselben Workspaces mit Toleranz (1e-5 FP32, 1e-3 FP16); Abweichungen nur als Warnung, nie Abbruch."
    Interfaces: "compare(candidate, reference, workspaces) -> Abweichungsliste (Trefferanzahl, Wert, Rang)."

  - Name: CpuSubengine
    Parent: Subengine
    Responsibility: "Rechnet auf CPU (potenziell mehrere Sockets): Cores mit Lanes (Worker-Threads), RAM-Speicher, mehr Platz, Default FP32, keine Streams/Staging; Kalibrierung und Kernel auf Low-Priority-Threads getrennt vom Async-Runtime."
    Interfaces: "SubengineContract; registriert Tokenizer, Normalizer, Encoder (CPU-Backend), Scorer (SIMD), Index-Ressourcen."

  - Name: CpuDevice
    Parent: CpuSubengine
    Responsibility: "CPU-Device mit Cores als Ressourcen und Lanes (Threads des Processors); Feature-Erkennung (AVX2, F16C, FMA, VNNI)."
    Interfaces: "Device-Handle CPU; Kapazität, Durchsatz; Features."

  - Name: CpuMemoryBackend
    Parent: CpuSubengine
    Responsibility: "RAM-Backend: je Arena genullter Bereich; Lesen/Schreiben/Kopieren auf Regionsbasis (memmove, Liveness-Check); Budget aus Arena-Summen; lädt Modelle/Indizes ins RAM bzw. streamt."
    Interfaces: "MemoryBackend (alloc, free, free_after_event, budget, read, write, copy)."

  - Name: CpuKernelRegistry
    Parent: CpuSubengine
    Responsibility: "Eigene CPU-Kernel mit eigenem Register (Dot-Product mit 8 Teilsummen, MaxSim, L2, Slot-Scan)."
    Interfaces: "kernel(name, dtype, variant) -> Funktion."

  - Name: CpuScorer
    Parent: CpuSubengine
    Responsibility: "Selbst implementierter SIMD-Scorer; slotweiser Scan gegen alle Queries mit laufendem Maximum."
    Interfaces: "Scoring-Schnittstelle; nur FP32."

  - Name: Tokenizer
    Parent: CpuSubengine
    Responsibility: "CPU-Ressource: lädt Tokenizer (Pad-Token-Bestimmung, Trunkierung, Padding aus), Modi Query/Passage, Präfix- und Multi-Vector-Schema, Token-IDs als u32, Fingerprint."
    Interfaces: "load(model_desc | config); encode(texts, mode) -> Tokenfolgen; encode_batched(...); pad_token(); Fehler: Load-Error, Encoding-Error, 'keine Eingabetexte'."

  - Name: TextNormalizer
    Parent: Tokenizer
    Responsibility: "Austauschbarer, nebenläufiger Text-Normalizer (Unicode, Lowercase usw.), läuft vor Präfix und Tokenisierung, ergänzt nur; Identity mitgeliefert."
    Interfaces: "normalize(text) -> text; Konfiguration über Parameter."

  - Name: BatchBuilder
    Parent: Tokenizer
    Responsibility: "Bildet aus Tokenfolgen padded Batches: Length-Buckets (32..512), Token-Budget, Sortierung nach Länge, Attention-Mask, Positionsmapping, binäre Serialisierung (2 x Zeilen x Länge, u32 LE)."
    Interfaces: "build(sequences, config{budget,max_len,pad_id}) -> [Batch{rows,positions,ids,mask}]; serialize()."

  - Name: CpuIndexManager
    Parent: CpuSubengine
    Responsibility: "Manager für RAM-Primärindex: Residency dateigestützter Daten im Page-Cache (mmap > RAM), Unload nach Leerlauf; Lesen direkt am Index."
    Interfaces: "IndexManagerCore; read(entry), read_many(entries, Zugriffsmuster); mark_cold(); Fehler Transfer-Error 'Lese-Task abgebrochen'."

  - Name: PageCacheAdvisor
    Parent: CpuIndexManager
    Responsibility: "Page-Cache-Hinweise (will-need, sequential, random, cold) je Byte-Bereich über austauschbares Trait; Bereichsprüfung, Page-Alignment bei cold, unverbindlich."
    Interfaces: "advise(range, advice) -> ok | Fehler; no-op auf nicht unterstützten Plattformen."

  - Name: BlockingReader
    Parent: CpuIndexManager
    Responsibility: "Blockierende Lesezugriffe (Page-Faults, Disk-I/O) im Blocking-Pool statt im Async-Runtime; Prefetch überlappt I/O und Kernel; Panics/Abbruch als Fehler."
    Interfaces: "read_with_hint(range, advice) -> Bytes; prefetch(range)."

  - Name: GpuSubengine
    Parent: Subengine
    Responsibility: "Rechnet auf CUDA-GPU (potenziell mehrere): Compute Units/SMs mit Lanes, begrenztes VRAM, Streams/Staging/Transfers, Batching notwendig, Default FP16; JIT-Kernel für vorhandene Architektur."
    Interfaces: "SubengineContract; fällt ohne kompatible GPU aus (CPU-Fallback)."

  - Name: GpuDevice
    Parent: GpuSubengine
    Responsibility: "Ein Prozess, ein Primary Context, mehrere Streams; Device-Ordinal, Compute Capability, Treiber-Info; Tensor-Core-Erkennung (nur Log)."
    Interfaces: "Device-Handle; create_lane(class, priority); query memory."

  - Name: VramManager
    Parent: GpuSubengine
    Responsibility: "Lädt/streamt Modelle und Indizes ins VRAM, kennt verfügbares VRAM, tauscht Modelle/Kernel/Ressourcen aus/ein, verwaltet Arenen (Weights, Segmente, Workspace, Parameter-Block), Brücke RAM-Index <-> VRAM-Mirror."
    Interfaces: "MemoryBackend; load/evict model/kernel/segment; Fit-Check mit Headroom; Fehler Arena erschöpft/Device-Speicher nicht verfügbar."

  - Name: GpuMemoryBackend
    Parent: VramManager
    Responsibility: "CUDA-Backend: je Arena ein genullter Block, stream-ordered Allokation (Default-Pool), D2D/H2D/D2H-Transfers, Mapped Host-Memory; keine Allokation im Dauerbetrieb."
    Interfaces: "MemoryBackend; copy_h2d/d2h/d2d (async, Event); mapped_ptr; Fehler Transfer-Error."

  - Name: StagingRing
    Parent: VramManager
    Responsibility: "Ring aus Pinned Staging-Slots (frei, belegt, in Übertragung mit Event); Acquire reihum, blockierend/nicht blockierend; Wiederverwendung erst nach Event."
    Interfaces: "acquire(blocking) -> Slot | 'Staging erschöpft'; begin_transfer(event); release; in_flight()."

  - Name: TensorRegionWrapper
    Parent: VramManager
    Responsibility: "Hüllt Device-Teilbereich als Tensor der Tensor-Library (ohne Ownership, nicht freigegeben) für Library-Operationen; nur am Adapter-Rand."
    Interfaces: "wrap(region) -> Tensor (intern via CandleAdapter)."

  - Name: GpuKernelRegistry
    Parent: GpuSubengine
    Responsibility: "Register für Kernel: Name+Datentyp+Architekturstufe+Variante -> Funktion; NVRTC-Kompilierung (PTX, Fallback SASS/Maschinencode), Lazy und ohne Doppelkompilierung; Slots für Address-Mapping."
    Interfaces: "get(name, dtype, arch, variant) -> Funktion; clear(); count(); Fehler: Compile-/Module-Load-Error, 'nicht verfügbar'."

  - Name: GpuKernels
    Parent: GpuSubengine
    Responsibility: "Kernel für dichtes Dot-Product und fused Max-Similarity (FP32/FP16-Dokumente, FP32-Queries); Dimension kompilierzeitfest; Parameter-Block dynamisch; Embedding-Write-Kernel."
    Interfaces: "launch(lane, args, parameter_block); nur Segmente/Adressen/Offsets, keine Entry-Identität."

  - Name: LaunchModes
    Parent: GpuSubengine
    Responsibility: "Launch-Modus getrennt vom Scorer: direkter Launch verfügbar; Graph-Capture und Persistent-Kernel als 'nicht verfügbar' abgelehnt; Launch-Key (Shape-Klasse, Regionen mit Generation); optionales Warm-up."
    Interfaces: "launch(work_item, lane) -> Kernel-Error | ok; mode capability."

  - Name: WorkChunker
    Parent: GpuSubengine
    Responsibility: "Planungseinheit mit Zeitbudget (Ziel ca. 2 ms, EMA-Durchsatz); Preemption nur an Chunk-Grenzen (Query überholt Indexing); Abbruchgründe Zeitbudget/Preemption."
    Interfaces: "next_chunk_len(); observe(slots,duration); throughput(); run(chunks) -> {done, rest, reason}."

  - Name: GpuIndexManager
    Parent: GpuSubengine
    Responsibility: "Manager für den VRAM-Mirror des Index: Zuordnung Entry<->Segment/Slot, Residency, Update-Queue, Upload/Eviction, Rückabbildung Adresse->Entry; mutiert nur in Gaps, blockiert Kernel nie."
    Interfaces: "IndexManagerCore; ensure(entry_set); reconcile_iteration(); publish_segment; release_segment; Abfragen: Segmentzahl, residente Bytes, ausstehende Updates."

  - Name: SegmentStore
    Parent: GpuIndexManager
    Responsibility: "Segmente und Slots: feste Slot-Klassen (32/64/128..65535 Token), Segment-View, Slot-Encoding mit Padding, Valid-Bits/Tombstones, Slot->Entry-Zuordnung, Append-only mit Compaction, Reserve/Commit/Drain, Treffer-Umrechnung."
    Interfaces: "reserve_slot/commit/drain; bind/unbind/lookup; valid-bit-snapshot (u64-Wörter); resolve_hit(addr,score) -> Entry-Ref | stale/Fehler."

  - Name: SegmentArena
    Parent: GpuIndexManager
    Responsibility: "Speicherverwaltung für Index-Segmente: 256-Byte-Alignment, Reserve, Replacement-Headroom, Budgets (overflow-sicher), Replacement durch Kopie (max. 1 gleichzeitig), Bump-Segmente, Retire/Reclaim nach Reader-Epoche."
    Interfaces: "alloc(size) | 'Arena erschöpft'; begin/finish/discard_replacement; free(now|after_event); reclaim(min_reader_epoch); utilization()."

  - Name: ManifestPublication
    Parent: GpuIndexManager
    Responsibility: "Manifest = Snapshot der Segmente mit Generation; lock-freier Zugriff, atomares Ersetzen (nur steigende Generation), Leases für Reader, Reclaim, Segmenttabelle (24 Byte/Segment), Event-verzögerte Publikation, Change-Events (added/changed)."
    Interfaces: "current() -> Manifest; publish(m) -> vorheriges | 'stale Generation'; lease(); defer_until(event); poll_publish(); subscribe()."

  - Name: TableSwap
    Parent: GpuIndexManager
    Responsibility: "Copy-on-Write der Segmenttabelle und Root-Pointer-Swap (Release/Acquire, 8 Byte), Epoch-Tracking der Reader, Retire/Reclaim, Sichtbarkeitsgarantie vor Root-Swap, Guard im Persistent-Modus."
    Interfaces: "publish_table(next) -> Epoche | 'stale Mapping'; min_reader_epoch(); reader report/quiescent; reclaim(limit)."

  - Name: DoorbellSync
    Parent: GpuIndexManager
    Responsibility: "Batch-Synchronisation: Device publiziert completed (Release), Host acked (Acquire); Host verändert nur zwischen Batch-Ende und Ack (Gap); Host wartet nur auf Batch-Event, nie auf den Kernel."
    Interfaces: "poll_gap() -> Option<Gap>; wait_gap(); close_gap(ack) (offene Gap = Programmierfehler)."

  - Name: CommunicationSlot
    Parent: GpuIndexManager
    Responsibility: "Feste Reihenfolge in der Gap: Kernel-Writes lesen und am Index upserten, Ergebnisse auflösen (verwerfen/stale/unverändert), Updates einspielen; Wiederholung bei 'stale Mapping'."
    Interfaces: "run(batch) -> aufgelöste Treffer (mit stale-Flag); Trait für Device-Rücklesen; Logging der Zähler."

  - Name: AddressMapping
    Parent: GpuIndexManager
    Responsibility: "Address-Map (Arena-Adresse -> Entry, Copy-on-Write, Epoche, Überlappungsprüfung) und Mapping-Store (versionierte Snapshots, Publikation nur aktuell+1)."
    Interfaces: "add/remove segment; snapshot(epoch) | 'stale Mapping'; publish(map); resolve(addr) -> Entry."

  - Name: UpdateQueue
    Parent: GpuIndexManager
    Responsibility: "Bind-/Unbind-Updates mit Stamp, je Entry nur das jüngste; Anwendung in Gap (erst Deletes, dann Binds); wirkt auf Residency und Slot-Bindings."
    Interfaces: "enqueue(update); contains; depth; apply() -> Anzahl."

  - Name: ResidencyLoader
    Parent: GpuIndexManager
    Responsibility: "Lazy, scope-getriebene Ladestrategie: Geometrie, Policy (Dichteschwelle, Segmentgröße nach Budget), Pläne (Segmente, Gather, Zero-Copy), Aggregation über Scope-Union, Single-Flight, Tiering."
    Interfaces: "plan(entry_set, access) -> Pläne; execute (Abbruch bei erstem Fehler); load_once(key) (Joiner geteilt)."

  - Name: DeviceTransfer
    Parent: GpuIndexManager
    Responsibility: "Transfer auf das Device: direktes Segmentkopieren, Gather-Pfad über Staging-Slots, Direktes Mapping (Zero-Copy) über Mapped Host-Memory; Stamp vor den Daten erfassen; Copy-Lane."
    Interfaces: "upload_segment(range) -> LoadedSegment{region,bindings}; Sink für geladene Segmente; Transfer-Error H2D."

  - Name: IndexBuildWriter
    Parent: GpuIndexManager
    Responsibility: "Index-Aufbau durch Kernel: vorab vergebene Schreibbereiche, Write-Pointer-Modell (Overflow, Partial Result), Warp-Claims, Embedding-Batch-Ausführung (Resume), Adressierung gebauter Segmente (fest/variabel), SPSC-Queues für Writes und Treffer."
    Interfaces: "run_embedding_batch(...) -> Bereiche, Regionen, Teilantworten | 'Arena erschöpft'; Kernel-Batch-Lifecycle: begin, run, allocate, progress, end."

  - Name: IndexOpsCoordinator
    Parent: GpuIndexManager
    Responsibility: "Koordiniert je Index-Operation Manager, Table-Swap, Mapping-Store und Segment-Queue: serialisierte strukturelle Änderungen, Publish geladener Segmente, Retry bei 'Arena erschöpft', Fehlerabbildung auf Index-Fehler."
    Interfaces: "publish_pending() -> erster Fehler | Anzahl; ensure/reconcile mit Auto-Publish."

  - Name: Common
    Parent: Resident
    Responsibility: "Gemeinsam genutzte, hardwareunabhängige Bausteine (Queue, Devices, Memory, Index, Precision, Adapter, Diagnostics); ohne Kopplung an Subengines, Kernel, Modelle."
    Interfaces: "von allen Komponenten genutzt."

  - Name: Queue
    Parent: Common
    Responsibility: "Lock-free MPMC-Queue (einfach verkettete Liste mit Sentinel, CAS-Append/Unlink); Altersreihenfolge; Metadaten opak; kein Head-of-Line-Blocking; 4 Instanzen (Task, CPU, GPU, gemeinsam); Reference-Counting, iterative Freigabe, kein unsafe außer atomarem Pointer-Swap."
    Interfaces: "enqueue(meta,payload) -> Entry-Handle; iter_all/iter_active; claim; cancel; Notification-Channels pro Subengine."

  - Name: QueueNode
    Parent: Queue
    Responsibility: "Knoten mit immutablen Metadaten, gemeinsamem Zustandsobjekt (atomarer Zustand, einmal setzbares Ergebnis, Notification), Payload-Fach (Ownership-Transfer beim Claim), Next-Pointer; Zustände enqueued/claimed/completed/cancelled."
    Interfaces: "claim() -> Payload | None; complete(result) (einmal); cancel() -> bool; state(); async wait."

  - Name: QueueIterator
    Parent: Queue
    Responsibility: "Nicht blockierende Iteration (alle / nur aktive) in Enqueue-Reihenfolge, bleibt bei parallelem Enqueue/Claim/Completion gültig; hält Knotenreferenzen."
    Interfaces: "Iterator<Item = Entry>; Sentinel nie sichtbar."

  - Name: Devices
    Parent: Common
    Responsibility: "Abstraktion von Device-Arten (CPU, GPU, erweiterbar), Lanes (Execution-Contexts) und Completion-Events."
    Interfaces: "Device-Handle, Lane, Event."

  - Name: Device
    Parent: Devices
    Responsibility: "Device-Handle (Ordinal bzw. CPU) und Device-Klasse."
    Interfaces: "kind(); ordinal(); Vergleich/Key."

  - Name: Lane
    Parent: Devices
    Responsibility: "Execution-Context: Stream, Priorität, Events, eigener Parameter-Block und Workspace (z.B. Query, Insert/Update, H2D, D2H); Übergaben zwischen Lanes nur über die Queue."
    Interfaces: "submit(work) -> Event; priority; parameter_block; workspace."

  - Name: Event
    Parent: Devices
    Responsibility: "Opaker Completion-Marker (GPU-Event bzw. CPU-Event, dauerhaft abgeschlossen nach Signal); Mock-Event für Tests."
    Interfaces: "is_complete() (non-blocking); wait() (blocking, Event-Error); Display mit Status."

  - Name: Memory
    Parent: Common
    Responsibility: "Gemeinsame Speicherverwaltung: Arenen, Regionen/Layouts, Allocators, Budget, Backend-Interface; Alloc/Free im Betrieb vermeiden, stream-ordered Reuse."
    Interfaces: "MemoryBackend, Allocator-Set, Budget."

  - Name: RegionLayout
    Parent: Memory
    Responsibility: "Pointerfreie Region (Arena-ID, Offset, Länge, Generation), Layout (Shape, Strides, Elementtyp), eigene Elementtypen (FP32/FP16/BF16/u8/u32/i64), Views (transpose, narrow), Mask-Wert -10000."
    Interfaces: "Region/Layout-Typen (Copy, Eq, Hash); contiguous(); elem_count(); byte_len(); Typkonvertierung an Library-Rändern."

  - Name: Arenas
    Parent: Memory
    Responsibility: "Sechs Arenen nach Lebensdauer (Weights, Index-Segmente, Workspace, Staging, Parameter-Block, Mapped Host-Memory) mit stabiler ID und konfigurierbarer Kapazität; Lazy-Anlage nach Budget; 64-Bit-Offsets."
    Interfaces: "Arena::from_id (sonst 'keine Arena'); all(); capacity(); device_demand() (saturierend)."

  - Name: OffsetAllocator
    Parent: Memory
    Responsibility: "Offset-Allocator je Arena ohne eigenen Speicher: First-Fit aufsteigend, Coalescing, Alignment, overflow-sicher."
    Interfaces: "alloc(len, align) -> Offset | passt nicht; free(offset,len); capacity(); free_bytes()."

  - Name: BookkeepingAllocator
    Parent: Memory
    Responsibility: "Thread-safe Verfolgung von Offsets und Generationen (stale Handles ignoriert), Liveness-Prüfung, Fault Injection, Debug-Logging."
    Interfaces: "allocate -> Region; free(region) | stale; is_live(region); inject_failure(arena)."

  - Name: DeferredFreeAllocator
    Parent: Memory
    Responsibility: "Allocator je Arena mit Deferred Free: Region kehrt erst nach Event-Abschluss zurück (Reaping vor jeder Allokation); Allocator-Set je Arena genau einer."
    Interfaces: "alloc; free_now; free_after(region, event); reap() -> Anzahl; pending_count()."

  - Name: MemoryBudget
    Parent: Memory
    Responsibility: "Budget aus freiem/gesamtem Speicher und Generationsreserve, Headroom (512 MiB), Fit-Check, Copy-on-Write-Doppelbedarf; Abfrage austauschbar."
    Interfaces: "budget() | 'Speicherabfrage fehlgeschlagen'; fit_check(plan) -> ok | Fehler (angefordert/nutzbar)."

  - Name: MemoryBackend
    Parent: Memory
    Responsibility: "Nebenläufige Backend-Schnittstelle: Allokation in Arena (Größe, Alignment), Freigabe, Freigabe nach Event, Budget; Auflösen Region->Adresse nur hier."
    Interfaces: "alloc, free, free_after_event, budget, read/write (Bytes einer Region)."

  - Name: Index
    Parent: Common
    Responsibility: "Source of Truth für Vektoren: kapselt beliebige Struktur (Datei, mmap, Chunks, Bäume), kennt weder Subengine noch Kernel noch Scopes; RAM = persistente Instanz, VRAM nur Mirror."
    Interfaces: "Index-Contract; Change-Notification; deklarierte Zugriffsmethoden (Centroid, Prefix, Subtree, Range)."

  - Name: IndexContract
    Parent: Index
    Responsibility: "Allgemeiner Vertrag: Stamp je Entry, Snapshot aller Stamps, Lesen (Vektorbereich), Schreiben (Last-Writer-Wins; Reihenfolge Daten, Stamp, Signal), Level-Signal mit Max-Stamp, Struktur-Zugriff (Summary, Kinder, Byte-Bereiche)."
    Interfaces: "stamp(id); all_stamps(); read(id); write(id,vector)->Stamp; remove; subscribe_changes(); supported_access()."

  - Name: ChangeStamp
    Parent: Index
    Responsibility: "Opaker Veränderungsmarker, eindeutig je Write/Delete (auch Remove verbraucht einen), Vergabe im exklusiven Write-Abschnitt; Fortsetzung nach Wiederaufnahme."
    Interfaces: "next(); max_stamp(); raw(); nur Gleichheitsvergleich."

  - Name: EntrySetScope
    Parent: Index
    Responsibility: "Scope-Arten (all, range, set, subtree, prefix, centroid; erweiterbar) und kompakte Entry-Mengen (Ranges/Bitmap, 64-Bit-IDs) mit Mengenoperationen."
    Interfaces: "Scope::name(); EntrySet: from_ranges/ids, contains, len, iter, union, intersect, eq, hash, runs()."

  - Name: ReferenceIndex
    Parent: Index
    Responsibility: "Referenz-Index im RAM ohne GPU und ML-Libraries (Tests, Baseline)."
    Interfaces: "IndexContract."

  - Name: FileIndex
    Parent: Index
    Responsibility: "Dateigestützter Parquet/Arrow-Index: Open (mmap/File, Header-/Schema-Prüfung, Verifikation), Lazy-Dekodierung in 4096er-Batches (FP16 LE), Upsert/Remove, atomarer Save (Temp+fsync+rename), Row-Groups, Nutzdaten, Verifikation."
    Interfaces: "open(path, expected_header, options); upsert(id,vector,payload); remove; save() -> geschrieben | keine Änderung; verify(); Fehler 'Index nicht verfügbar'/Schema-Inkompatibilität."

  - Name: IndexHeader
    Parent: Index
    Responsibility: "Header im Datei-Footer und einzige harte Bindung Index<->Modell: Schema-Version, Max-Stamp, Hash-Algorithmus/-Version, Model-ID/-Revision, Dimension, Pooling, Normalisierung, Präfixe, Storage-/Compute-Datentyp, max. Länge, Tokenizer-Fingerprint."
    Interfaces: "from_descriptor(model); matches(expected) (sammelt Abweichungen); to/from key-value pairs."

  - Name: ChecksumChangeDetection
    Parent: Index
    Responsibility: "64-Bit-Checksum je Entry (Länge-präfixierte Felder) und Erkennung von Quelldateiänderungen über Fallback-Kette (Stamp, Größe+Zeit mit Racy-Check, Content-Hash)."
    Interfaces: "checksum(entry); check_file(record) -> unverändert | Content-Prüfung nötig."

  - Name: Precision
    Parent: Common
    Responsibility: "Capabilities, Kalibrierung und Precision-Profile; Precision-Wunsch 0..1 ist keine Zusicherung; Kalibrierung blockiert den Nutzer nie; kein Rounding-Vertrag zwischen CPU und GPU."
    Interfaces: "profile_for(subengine, model, wish) -> Konfiguration."

  - Name: CapabilityModel
    Parent: Precision
    Responsibility: "Erweiterbare Achsen/Werte (Strings), Capability-Sets (Add, Containment, Erfüllung von Requirements, reihenfolgeunabhängig), vordefinierte Achsen und Referenz-Sets."
    Interfaces: "CapabilitySet::add/contains/satisfies; Eq/Hash/Serialize."

  - Name: CapabilityDetection
    Parent: Precision
    Responsibility: "Ermittelt CPU-/GPU-Capabilities (F16C, AVX2, VNNI; FP16 ab CC 5.3, DP4A ab 6.1), numerische Achsen, Reduktion auf vorhandene Kernel; Capability-Error ohne GPU."
    Interfaces: "detect(subengine, kernel_catalog) -> CapabilitySet | Capability-Error."

  - Name: BenchmarkWorkload
    Parent: Precision
    Responsibility: "Reproduzierbare synthetische Workload (Seed) und Abweichungsmaße (Cosine-Deviation, relativer Fehler, Top-k-Overlap, Validity-Zustände); Referenz FP32 auf CPU."
    Interfaces: "generate(seed, docs, dim); deviation(candidate, reference) -> Maß + Validity."

  - Name: Measurer
    Parent: Precision
    Responsibility: "Austauschbares Mess-Interface (Abweichung, Zeit-Probe, Unterbrechung durch produktive Aufgaben); CPU- und GPU-Messmittel, Clock austauschbar."
    Interfaces: "measure_deviation(combo); probe_time(combo) -> Zeit | unterbrochen."

  - Name: Calibrator
    Parent: Precision
    Responsibility: "Kalibrierungslauf je Subengine+Modell: Kombinationen aufzählen, Achsen-Pruning, interleaved Messung gegen Anchor, Stabilitäts-/Dominanz-Entscheidung, Budgets, Report, Pareto-Front."
    Interfaces: "calibrate(target, capability_set, config) -> Report; Fehler Capability-Error."

  - Name: ParetoProfiles
    Parent: Precision
    Responsibility: "Pareto-Front über Zeit und Abweichung, Positionen (normierte Bogenlänge), Profile ableiten (Merge-Threshold 0,1); Profil = Position + Wert je Achse."
    Interfaces: "front(points); positions(front); derive_profiles(front, threshold) -> [Profile]."

  - Name: ProfileSelector
    Parent: Precision
    Responsibility: "Wählt Profil nach Precision-Wunsch (Clamp, NaN ungültig; Gleichstand -> präziseres); Latch 'kein aktives Profil' je Modell/Subengine, thread-safe."
    Interfaces: "select(wish) -> Profile | 'ungültige Precision' | 'kein aktives Profil'; unlatch() -> bool."

  - Name: CalibrationStore
    Parent: Precision
    Responsibility: "Persistenz der Ergebnisse je Modell/Subengine mit Fingerprint (Schema, Hardware, Treiber, CPU-Features, Model, Engine-Version), Zustände fehlend/kalibriert/veraltet, atomares Save, Hintergrund-Neukalibrierung."
    Interfaces: "load(); save() (Temp+fsync+rename); resolve_at_start() (scheitert nie); fingerprint()."

  - Name: BackgroundPool
    Parent: Precision
    Responsibility: "Low-Priority-Thread-Pools (nice) für Kalibrierung/Kernel; Futures, Panic-sicher, Drain beim Shutdown."
    Interfaces: "submit(job) -> Future; poll/wait; shutdown(drain)."

  - Name: Adapters
    Parent: Common
    Responsibility: "Einzige Stellen für externe Libraries; Typen externer Libraries werden nur an diesen Rändern konvertiert."
    Interfaces: "Candle, safetensors, Parquet/Arrow, CUDA/NVRTC."

  - Name: CandleAdapter
    Parent: Adapters
    Responsibility: "Anbindung der Tensor-/NN-Library Candle (CPU- und CUDA-Backend, Datentyp- und Tensor-Konvertierung, Region-Wrapping); keine Candle-Typen in öffentlichen Signaturen."
    Interfaces: "intern: forward, gemm, convert dtype, wrap region."

  - Name: SafetensorsAdapter
    Parent: Adapters
    Responsibility: "Lesen von safetensors (mmap, zero-copy), Elementtyp-Übernahme ('nicht unterstützter Datentyp')."
    Interfaces: "open(path|bytes); tensor(name) -> Rohbytes, Shape, Typ."

  - Name: ParquetArrowAdapter
    Parent: Adapters
    Responsibility: "Lesen/Schreiben des Parquet-Spaltenformats mit Arrow-Schema (Row-Groups, Footer-Key-Values, Statistiken)."
    Interfaces: "read_columns(projection); write_snapshot(rows, row_group_size, footer_kv)."

  - Name: CudaAdapter
    Parent: Adapters
    Responsibility: "CUDA-Driver-/NVRTC-Anbindung: Context, Streams, Events, Speicher, Kernel-Kompilierung/-Laden, Gerätedaten (Name, CC, Treiber)."
    Interfaces: "intern: stream/event/memcpy/launch/compile; Fehler als Device-/Event-/Launch-/Compile-Error."

  - Name: Diagnostics
    Parent: Common
    Responsibility: "Querschnitt: strukturiertes Logging/Tracing (Debug/Trace-Level, Spans), Fehlerkategorien, Debug-Assertions für Programmierfehler, Fault Injection, Test-Mocks."
    Interfaces: "log/span-API; Fehlertypen (Konfigurations-, Load-, Transfer-, Memory-, Device-, Index-Fehler)."

  - Name: Addon
    Parent: Resident
    Responsibility: "Erweiterungspunkt für Iteration, Kaskade und Rekursion (in Subengine, Anwendung, Index oder Modell): enqueued Folge-Tasks über den normalen Weg mit geklonten/geänderten Parametern, z.B. Centroid->Mitglieder, Text->Embedding->Similarity, Reranking."
    Interfaces: "nutzt Engine.enqueue_*, ParamExtensions, Pull-Filter; Addon-Tasks von Anwendungs-Tasks nicht unterscheidbar."
```

## Anmerkungen zur Modellierung

- **Aufnahmefähigkeit:** Neue Modelle, Indexarten, Scope-Arten, Operationsarten, Ergebnisarten, Capability-Achsen und Launch-Modi sind jeweils Erweiterungen innerhalb eines bestehenden Knotens (Register oder Enum). Fassade und Task-Schnittstelle ändern sich dadurch nicht. Verhalten wie Iteration und Rekursion gehört ausschließlich in `Addon`.
- **Index:** Der Index liegt unter `Common`, weil er weder Subengine noch Kernel kennt. Die Manager liegen als Subengine-Komponenten unter `CpuSubengine` und `GpuSubengine`, ihr gemeinsamer Kern unter `SubengineCommon`.
- **Tokenizer/Normalizer:** Sie sind CPU-Ressourcen und gehören deshalb zu `CpuSubengine`. `Model` bleibt unter `SubengineCommon` als gemeinsame Host-Ressource.
- **Kalibrierung:** Das Framework liegt in `Common/Precision`. Die Subengines liefern nur Capabilities, Messmittel und Profile.
