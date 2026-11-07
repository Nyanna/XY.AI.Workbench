Erstelle Umsetzungspläne für Agenten im Verzeichnis `/home/user/xyan/xy.ai.workbench/project/resident/plan` für Gruppe E aus `resident_planplan.md`.

* `resident_index.md` ist nur relevant für die Implementierung eines Vektorindexes und keine Indexdatei.

Projektpläne:
```json
{
  "result" : {
    "results" : [ {
      "path" : "/home/user/xyan/xy.ai.workbench/project/resident",
      "entries" : [ ".:", "resident.md", "resident_first.md", "resident_index.md", "resident_planner.md", "resident_precision.md", "resident_processor.md", "resident_subengine.md", "resident_planplan.md" ]
    } ]
  }
}
Kontext Dateien für Gruppen:
{
  "result" : {
    "results" : [ {
      "path" : "/home/user/xyan/xy.ai.workbench/project/resident/context",
      "entries" : [ ".:", "context_h_q.md", "context_a.md", "context_b.md", "context_c.md", "context_d.md", "context_e.md", "context_f.md", "context_g.md" ]
    } ]
  }
}
```
SystemInit:  id=e2529dd5-743d-4c2f-a9ef-e5e74323a2a2, cwd=/home/user/xyan/xy.ai.workbench/project, model=claude-opus-4-8
Text:
 I'll start by reading the key files to understand the task: the plan-of-plans and the Group E context.
Control Request:
```yaml
id: 7e40-1
toolName: read_file
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/project/resident/resident_planplan.md
  - path: /home/user/xyan/xy.ai.workbench/project/resident/context/context_e.md
  reason: Read the plan-of-plans to understand what Group E entails and read the Group E context file.
```
/answer 7e40-1 allow
ReasoningToken: 220
Text:
 I have the metaplan and Group E context. Now let me read the primary source document `resident_index.md` (the main source for Group E) and the related documents referenced by the dependencies.
Control Request:
```yaml
id: 7e40-2
toolName: read_file
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/project/resident/resident_index.md
  - path: /home/user/xyan/xy.ai.workbench/project/resident/resident_subengine.md
  reason: "Read the primary source for Group E (resident_index.md) and supporting documents (subengine, index-related sections) to build accurate implementation plans."
```
/answer 7e40-2 allow
ReasoningToken: 478
Text:
 Let me read the cross-cutting context (Q packages: errors, logging, tests, feature flags — these feed every E plan) and probe for any existing plan-file convention in the target directory.
Control Request:
```yaml
id: 7e40-3
toolName: read_file
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/project/resident/context/context_h_q.md
  - path: /home/user/xyan/xy.ai.workbench/project/resident/plan/README.md
  - path: /home/user/xyan/xy.ai.workbench/project/resident/plan/plan_d.md
  - path: /home/user/xyan/xy.ai.workbench/project/resident/plan/plan_e.md
  - path: /home/user/xyan/xy.ai.workbench/project/resident/plan/E1.md
  reason: "Read cross-cutting conventions (Q1-Q4) that feed into each E plan, and probe for an existing plan-file naming/format convention in the target directory to stay consistent."
```
/answer 7e40-3 allow
Control Request:
```yaml
id: 7e40-4
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/project/resident/context/context_h_q.md
      checksum: 6207778d550a9f4a7f7a6c7414e5c94646c2b27d10555a37377f56fa15ff7057
      content: |-
        # Kontext: Offene Fragen Gruppe H & Querschnitt Q („Resident"-Engine)

        **Status der Belege:** Die Antworten stammen aus dem Fachwissen (Stand ca. Mitte 2026), es gab keine Live-Abfrage der Doku. Mit **[prüfen]** markierte Punkte sind vor Implementierung gegen die aktuelle Crate-Version oder den CUDA-Guide zu verifizieren. Das betrifft vor allem Crate-Versionen und die API-Abdeckung von `cudarc`/`candle`.

        Zielhardware: GTX 1660 (TU116, sm75, 22 SMs, 6 GB VRAM).

        ---

        ## H1 · GPU-Ringbuffer (persistenter Kernel)

        **1. Persistente Kernel.**
        - Muster: ein Grid, das exakt so groß ist, dass alle Blöcke gleichzeitig resident sind.
        - Gridgröße = `SMs × cudaOccupancyMaxActiveBlocksPerMultiprocessor(kernel, blockSize, smem)`. Auf der 1660 sind das 22 × Blöcke pro SM.
        - Alternative: `cudaLaunchCooperativeKernel`. Es garantiert Co-Residenz und schlägt sonst fehl, statt zu deadlocken. Ein Grid-weiter Barrier über `cooperative_groups::grid_group::sync()` setzt dasselbe voraus. [CUDA Programming Guide, „Cooperative Groups"]
        - Fallstricke:
          - Block-zu-Block-Wartebeziehungen deadlocken, sobald ein Block nicht resident ist.
          - Display-Watchdog (Windows TDR ca. 2 s, Linux mit X11): Kernel, die auf der Display-GPU zu lange laufen, werden abgebrochen. Auf einer Consumer-Karte ist das zentral und ein Grund für Zeitscheiben (Frage 6).
          - Stream-Synchronisation und andere GPU-Arbeit konkurrieren um SM-Ressourcen. Ein Kernel, der alle SMs belegt, blockiert nachfolgende Launches.

        **2. Lock-freier SPSC-Ring Host↔Kernel.**
        - Geeignet ist gemapptes Pinned-Memory: `cudaHostAlloc(..., cudaHostAllocMapped)` plus `cudaHostGetDevicePointer`. Die GPU greift per PCIe auf Host-RAM zu.
        - Unified/Managed Memory eignet sich nur eingeschränkt. Gleichzeitiger CPU/GPU-Zugriff während eines laufenden Kernels (`concurrentManagedAccess`) gibt es nur unter Linux auf Pascal+, nicht unter Windows.
        - Layout: `head` und `tail` auf getrennten Cachelines (mindestens 64, besser 128 B), Slots im Ring, Kapazität eine Zweierpotenz. Der Producer schreibt erst die Nutzdaten und veröffentlicht dann den Index mit Release-Semantik. Der Consumer liest den Index mit Acquire und danach die Daten.
        - Device→Host-Schreibzugriffe sind PCIe-„posted writes". Ihre Reihenfolge gilt erst nach `__threadfence_system()`.

        **3. Sichtbarkeit / Synchronisation ohne Kernel-Ende.**
        - Device-Seite, Release: Daten schreiben, `__threadfence_system()`, dann Index schreiben. Alternativ `cuda::atomic_ref<T, cuda::thread_scope_system>` aus libcu++ mit `store(release)`.
        - Device-Seite, Acquire: `ld.acquire.sys` (siehe 5) oder `cuda::atomic_ref::load(acquire)`.
        - `volatile` allein verhindert nur Compiler-Caching und liefert keine Ordnung.
        - Host-Seite: `std::sync::atomic` mit Acquire/Release. Auf x86 (TSO) reicht das für Host-Seite-Ordnung.
        - Gemapptes Pinned-Memory ist standardmäßig nicht write-combined. Das Flag `cudaHostAllocWriteCombined` nicht setzen, es macht Host-Reads extrem langsam.
        - Quellen: CUDA Programming Guide „Memory Fence Functions" und „Memory Model" (PTX ISA).

        **4. CAS / Exit-Protokoll `CAS(w, r, r | EXIT)`.**
        - Device: `atomicCAS` (device scope), `atomicCAS_system` (system scope, cc ≥ 6.0), `cuda::atomic<..., thread_scope_system>`.
        - Atomics auf gemapptem Host-Speicher von der GPU aus hängen von der Plattform ab (PCIe-Atomics, Attribut `cudaDevAttrHostNativeAtomicSupported`). [prüfen]
        - Empfehlung: Das Exit-CAS **host-seitig** auf dem gemappten Speicher ausführen (normales `AtomicU64::compare_exchange`). Die GPU liest `w`/Flag nur per `ld.acquire.sys`. Das vermeidet GPU-seitige System-Atomics über PCIe und bleibt bei SPSC korrekt, weil Producer und Consumer jeweils nur ihren eigenen Index schreiben.

        **5. `ld.acquire` auf sm75.**
        - Seit PTX ISA 6.0 / sm_70 verfügbar, also auch auf Turing.
        - Syntax: `ld.acquire.sys.u32 %r, [addr];`. Zusätzlich `ld.relaxed.sys`, `st.release.sys`, `fence.acq_rel.sys`, `fence.sc.sys`. Das Scope-Suffix (`.cta`/`.gpu`/`.sys`) bestimmt die Sichtbarkeit. Für Host↔Device ist `.sys` nötig.
        - Ein Acquire-Load garantiert nur die Ordnung relativ zum zugehörigen Release. Daten nach neuen Tabellen/Pointern dürfen erst nach dem Acquire des Indexes gelesen werden. Zusätzlich kann ein Event/Stream-Sync beim Austausch von Device-Tabellen nötig sein.
        - Quelle: PTX ISA, Kapitel „Memory Consistency Model".

        **6. `idle_spin` / `max_run_time`.**
        - Es gibt keine Preemption, mit der der Host einen laufenden Kernel sauber stoppen könnte. Das Abbrechen zerstört den Kontext.
        - Muster: Zeitscheibe im Kernel mit `clock64()` oder `%globaltimer` (ns). Beim Idle `__nanosleep(n)` (sm70+). Stop-Flag und Ring nur an Scheibengrenzen prüfen. Nach `max_run_time` kehrt der Kernel zurück, und der Host startet ihn neu, falls Arbeit ansteht.
        - Die Zeitscheibe sollte deutlich unter dem Watchdog-Limit liegen (z. B. ≤ 100 ms) [Empfehlung].

        **7. `cudarc` / `candle`.**
        - `cudarc` bietet Driver-API (Context, Stream, Module aus PTX/CUBIN, `launch_builder`/`LaunchConfig`), NVRTC und Zugriff auf `result::*` (Raw-Driver-Aufrufe, inkl. Cooperative Launch und Host-Alloc). [prüfen: genaue API der aktuellen Version]
        - `candle` unterstützt eigene Kernel über `CustomOp1/2/3` (`cuda_fwd`) und PTX-Laden über `CudaDevice`. Für langlaufende, persistente Kernel ist `candle` nicht gedacht. Empfehlung: dafür `cudarc` direkt auf einem eigenen Stream nutzen und `candle` nur für Tensor-Ops.
        - Beide teilen sich das Kontext-/Stream-Modell. Rohe Pointer aus candle-Tensoren sind nur gültig, solange der Tensor lebt.

        ---

        ## H2 · Sliding Window

        **8. Etablierte Verfahren.**
        - Überlappende Chunks mit Stride (Standard in RAG-Frameworks).
        - Sliding-Window-Attention in Modellen: Longformer (Beltagy et al., 2020, arXiv 2004.05150), BigBird, Mistral (Rolling Buffer KV-Cache).
        - Late Chunking (Günther et al., Jina, 2024, arXiv 2409.04701): ganzes Dokument in einem Long-Context-Encoder kodieren, danach Mean-Pooling pro Chunk.

        **9. Praktische Umsetzung mit BERT/e5-small (max 512 Tokens).**
        - Fenster = `max_seq_len − Spezialtokens`, Stride 25–50 % Überlappung.
        - Pro Fenster ein Embedding, Aggregation per Mean/Max-Pooling oder getrennte Vektoren pro Fenster mit Dokument-ID. Letzteres ist beim Retrieval üblich (Score = max über Fenster).
        - Das Modell braucht die Präfixe `query: ` / `passage: ` je Fenster.
        - `tokenizers` unterstützt Overflow-Handling: `truncation` mit `stride` und `return_overflowing_tokens` (`encoding.get_overflowing()`).

        **10. GPU-Technik für gleitende Berechnung ohne Neuberechnung.**
        - Bei bidirektionalen Encodern ändert eine Fensterverschiebung die Attention aller Tokens, daher gibt es **keine etablierte Inkrementtechnik**. Wiederverwendbar ist nur der KV-Cache bei kausalen Modellen.
        - Praktikabel ist nur Batching der Fenster und Token-Embedding-Wiederverwendung (Tokenisierung/Embedding-Lookup).

        ---

        ## H3 · PLAID / Reranker

        **11. PLAID.**
        - Paper: Santhanam et al., „PLAID: An Efficient Engine for Late Interaction Retrieval", CIKM 2022 (arXiv 2205.09707). Referenz-Repo: `stanford-futuredata/ColBERT`.
        - Stufen:
          1. Kandidatengenerierung aus den `nprobe` nächsten Zentroiden je Query-Token.
          2. Centroid Pruning (Schwellwert `t_cs`).
          3. Centroid Interaction: Scoring mit Zentroid-Scores ohne Residual-Dekompression, ggf. zweite, kleinere Stufe (`ndocs`).
          4. Residual-Dekompression und volles MaxSim auf den Top-Kandidaten.
        - `next-plaid`: nicht sicher belegt (vermutlich Rust-/Community-Reimplementierung). [prüfen: Repo, Lizenz, Aktivität]

        **12. Centroid-Codes & Residual-Kompression (ColBERTv2).**
        - Paper: Santhanam et al., NAACL 2022 (arXiv 2112.01488).
        - K-Means auf einer Stichprobe der Token-Embeddings ergibt die Zentroid-Tabelle (Größe ca. 2^⌊log2(16·√N)⌋ Zentroide).
        - Pro Token wird der Zentroid-Index (Code) gespeichert. Der Residual (Embedding − Zentroid) wird pro Dimension auf 1 oder 2 Bit quantisiert (`nbits`).
        - Beim Retrieval: Zentroid + dekomprimierter Residual ergibt das Embedding zurück.

        **13. Cross-Encoder `BAAI/bge-reranker-v2-m3`.**
        - Basis: XLM-RoBERTa-artiger Encoder (bge-m3-Backbone, ca. 568 M Parameter).
        - Eingabe: gemeinsame Kodierung `[CLS] query [SEP] doc [SEP]`. Ein Klassifikationskopf liefert einen Logit (optional Sigmoid).
        - Ressourcen: fp32 ca. 2,3 GB, fp16 ca. 1,1 GB. Das passt in 6 GB, aber nicht parallel zu großen Indexen.
        - `safetensors`: Gewichte als `model.safetensors` im HF-Repo. [prüfen]
        - `candle-transformers` enthält XLM-RoBERTa inkl. Sequence-Classification-Variante. [prüfen: aktuelle Version]

        **14. Funnel / Cascade.**
        - Stufe 1: billig (Zentroid-Scores / Binär / niedrige Dimension) → viele Kandidaten.
        - Stufe 2: Pruning (Schwellwert, Top-k).
        - Stufe 3: exaktes MaxSim auf der Spitze.
        - Optional Stufe 4: Cross-Encoder auf Top-n.
        - Steuerparameter: `nprobe`, `t_cs`, `ndocs`, Top-k je Stufe, `nbits`.
        - Die Stufentiefe bestimmt den Recall-/Latenz-Tradeoff und ist pro Query-Klasse konfigurierbar.

        ---

        ## H4 · Graph-Capture

        **15. Funktionsweise.**
        - Ablauf: `cudaStreamBeginCapture(stream, mode)` → Operationen im Stream → `cudaStreamEndCapture` → `cudaGraphInstantiate` → `cudaGraphLaunch` (Replay).
        - Modi: `Global`, `ThreadLocal`, `Relaxed`.
        - Verboten bzw. problematisch während Capture:
          - synchrone Aufrufe: `cudaStreamSynchronize`, `cudaDeviceSynchronize`, `cudaMemcpy` ohne Async
          - Legacy-Default-Stream
          - klassisches `cudaMalloc` (im Global-Modus unzulässig)
          - Event-Queries
        - Erlaubt: Kernel, `cudaMemcpyAsync`, Events (Record/Wait), `cudaMallocAsync`.
        - Quelle: CUDA Programming Guide, „CUDA Graphs".

        **16. Capture über `candle`-Ops auf eigenem Stream.**
        - Machbar nur, wenn candle-Ops keine Allokationen oder Syncs im Capture auslösen. candle allokiert über cudarc. Async-Pool-Allokation (`cuMemAllocAsync`) ist capture-fähig, sonst nicht. [prüfen]
        - Es ist kein offizielles Feature bekannt. Candle-Issues zu CUDA-Graphs/Streams sind zu prüfen. [prüfen: GitHub `huggingface/candle`, Suche „cuda graph"/„stream"]
        - Empfehlung: Warm-up-Lauf vor Capture, feste Buffer, Capture nur für eigene Kernel-Folgen.

        **17. Cache-Schlüssel für Graph-Instanzen.**
        - Schlüssel = (Shape-Bucket, dtype, Kernel-Variante, Gerät/Stream).
        - Feste Ein-/Ausgabe-Regionen. Ändert sich ein Pointer (`gen`), muss die Instanz invalidiert oder per `cudaGraphExecKernelNodeSetParams` / `cudaGraphExecUpdate` aktualisiert werden. Shapes in wenige Buckets quantisieren (z. B. Zweierpotenzen), da jede Instanz VRAM kostet.

        **18. `cudarc` und CUDA-Graphs.**
        - `cudarc` hat Graph-Unterstützung auf Raw-API-Ebene (`result::graph`/`sys`) und ab neueren Versionen ein sicheres Wrapper-API (Capture-Begin/End, Graph-Launch). [prüfen: Version, Beispiele im Repo `chelsea0x3b/cudarc`]

        ---

        ## H5 · Signalkompression (State of the Art)

        **19.–20. Frequenzdomäne für Embeddings.**
        - Etabliert sind **nicht** DCT/FFT, sondern andere Kompressionen:
          - Matryoshka Representation Learning (Kusupati et al., NeurIPS 2022)
          - Product Quantization (Jégou et al., 2011)
          - Binary/Scalar Quantization
          - PCA / Dimensionsreduktion
        - DCT/Wavelet-Ansätze existieren eher für Bild-/Zeitreihen-Hashing. Zu „Semantic Spectrogram" für semantische Suche sind **keine etablierten Referenzen** bekannt. [prüfen: Literatursuche]

        **21. Erweiterbarkeit.**
        - Ja. Ein Vorfilter ist ein Proxy-Score auf einer transformierten Repräsentation, die nur Rangfolge-Treue braucht, gefolgt von exaktem Rerank. Das passt als zusätzliche Scope-/AccessMethod, solange die Oberschicht eine Methoden-Abstraktion hat. Der Recall der Vorfilterstufe muss messbar sein.

        ---

        ## H6 · Bandit / Mutable Weights

        **22. Contextual Bandits als Meta-Router.**
        - Arme = Retrieval-/Cascade-Strategien, Kontext = Query-Features, Reward = Klick/Zufriedenheit/Latenz-adjustierte Qualität.
        - Verfahren: LinUCB (Li et al., WWW 2010), Thompson Sampling, ε-greedy. Praxis-Toolkit: Vowpal Wabbit.
        - Off-Policy-Evaluation (IPS/Doubly Robust) vor dem Ausrollen.

        **23. Online-Adaption von Gewichten.**
        - Ansätze: Online-Fine-Tuning, LoRA-Adapter-Updates, Online-Gradient-Descent auf Heads, kontinuierliches Lernen.
        - Risiken: katastrophales Vergessen, Reward-Hacking, Feedback-Loops/Bias, Nicht-Reproduzierbarkeit.
        - Praktikabel nur für kleine Teile (Router, Score-Gewichte), nicht für das Basismodell.

        **24. Reward-Stream-Pipelines.**
        - Fluss: Inferenz loggt (Kontext, Aktion, Propensity) → Reward-Events über Join-Key → Lernkomponente (getrennter Prozess) → versionierter Snapshot → atomarer Swap im Serving.
        - Inferenz und Update sind strikt getrennt, mit Rollback über Versionierung.

        ---

        ## Q1 · Fehler-Taxonomie

        **25. `thiserror`.**
        - Aktuell 2.x [prüfen].
        - Muster: `#[derive(Error, Debug)]`, `#[error("…{field}")]` für Display, `#[from]` für automatische `From`-Impl plus Source, `#[source]` für Ursachenkette ohne `From`, `#[error(transparent)]` zum Durchreichen.
        - Fehler sind Enum-Varianten mit strukturierten Feldern (keine Strings). `#[non_exhaustive]` auf öffentlichen Enums erlaubt spätere Varianten ohne Breaking Change.

        **26. Geschichtete Fehlertypen.**
        - Empfohlen: ein Enum pro Komponente/Modul (klein, präzise), das Top-Level-Enum aggregiert per `#[from]`.
        - Ein zentrales Mega-Enum koppelt alle Module und zwingt Aufrufer zu Matches über irrelevante Varianten.

        **27. Library vs. Application.**
        - Bibliothek/Engine mit öffentlicher API: `thiserror`, typisierte Fehler, damit Aufrufer matchen können.
        - Anwendung/Binary/Tests: `anyhow` (Kontext per `.context()`), kein Matchen nötig.
        - Die Engine selbst gibt kein `anyhow::Error` in der öffentlichen API zurück.

        **28. Intern vs. extern.**
        - Muster: öffentlicher Fehler als opaker Struct mit privatem `ErrorKind`-Enum (wie `std::io::Error`), plus Accessor `kind()` auf ein `#[non_exhaustive]` öffentliches Kind.
        - Interne Varianten wie `StaleMapping` leben in einem `pub(crate)`-Enum und werden an der API-Grenze per `From`/`map_err` auf öffentliche Varianten abgebildet (oder intern per Retry behandelt).

        ---

        ## Q2 · Logging (`tracing`)

        **29. Spans/strukturiertes Logging.**
        - `#[tracing::instrument(skip_all, fields(task_id = %id))]` auf Funktionen. `skip_all` verhindert ungewolltes Loggen großer Argumente, Felder explizit setzen.
        - Span pro Task/Ressource/Manager-Zyklus mit stabilen Feldnamen. Strukturierte Events: `info!(bytes = n, "msg")`. Hochfrequente Pfade auf `trace`/`debug` halten.

        **30. Propagation über async-/Thread-Grenzen.**
        - Futures: `fut.instrument(span)`, und für gespawnte Tasks `tokio::spawn(fut.instrument(Span::current()))`.
        - Synchrone Abschnitte: `span.in_scope(|| …)`. Einen `Entered`-Guard nie über `.await` halten.
        - `spawn_blocking`/Threads übernehmen den Span nicht automatisch: `let span = Span::current(); spawn_blocking(move || span.in_scope(|| …))`.

        **31. Subscriber mit tokio.**
        - `tracing-subscriber` mit `EnvFilter` (z. B. `RUST_LOG=engine=debug,engine::gpu=trace`), `fmt()` für menschenlesbar, `.json()` (Feature `json`) für Produktion. Pro Target/Modul filterbar.
        - `tracing-appender::non_blocking` für asynchrones Schreiben (Guard am Leben halten). Initialisierung im Binary, nie in der Library.

        **32. Nicht-tokio-Threads / CUDA-Callbacks.**
        - `tracing` ist threadsicher und funktioniert aus beliebigen Threads, solange ein globaler Subscriber gesetzt ist. Der Span muss explizit übergeben werden (siehe 30).
        - CUDA-Host-Callbacks (`cuLaunchHostFunc`): keine CUDA-API-Aufrufe darin, nur kurze Arbeit. Zum Loggen besser Daten per Channel/Atomic an einen Thread geben, der loggt.

        ---

        ## Q3 · Teststrategie

        **33. Mocks.**
        - `mockall` (`#[automock]` auf Traits) eignet sich für Erwartungs-/Aufruf-Verifikation.
        - Für Kernel-/Mess-Mocks mit Zustand und Skripten sind **manuelle Fakes** meist klarer: ein Trait, ein `FakeKernel` mit programmierbaren Ergebnissen, ein `FakeMeasure` mit Sequenz/RNG.

        **34. Nebenläufigkeitstests.**
        - `loom`: erschöpfende Exploration aller Interleavings unter dem C11-Modell (ersetzt `std::sync`/`atomic` durch `loom::sync` über `cfg(loom)`). Geeignet für kleine CAS-Protokolle (Append/Claim/Cancel/Unlink).
        - `shuttle`: randomisierte Schedules, skaliert besser bei größeren Szenarien. Ergänzend `proptest` für Sequenzen.

        **35. GPU-freie Tests.**
        - Backend-Trait (`ComputeBackend`) mit CPU-/Mock-Implementierung. Die gesamte Logik läuft gegen das Trait.
        - Feature `cuda` aktiviert nur die CUDA-Implementierung. Tests laufen standardmäßig ohne.

        **36. Deterministische Sync-/Zustandslogik.**
        - Virtuelle Zeit (`tokio::time::pause`, Feature `test-util`) oder eigener `Clock`-Trait.
        - Schritt-Hooks/Failpoints zum Erzwingen von Interleavings (ABA: gezielt Stamp/Generation zwischen Lesen und CAS ändern).
        - Zustandsbasierte Asserts statt Timing. Seeds fest.

        **37. Simulierte verrauschte Messwerte.**
        - Mock-Messfunktion hinter Trait: Basiswert + Rauschen aus seeded RNG (`rand` mit `StdRng::seed_from_u64`), Varianten: konstant, Ausreißer, Drift.
        - Tests: Abbruch an Obergrenze → `unstable`, Konvergenz bei geringem Rauschen, Verhalten bei Ausreißern. Wiederholt mit mehreren Seeds.

        ---

        ## Q4 · Feature-Flags

        **38. Best Practices.**
        - Features sind additiv (aktivieren nur Code, entfernen nichts). Default **ohne** `cuda`.
        - Optionale Dependencies mit `dep:`-Syntax, Code per `#[cfg(feature = "cuda")]` nur an wenigen Stellen (Modulgrenze).
        - Quelle: Cargo Book „Features".

        **39. `candle-core`-`cuda`.**
        - Das Feature `cuda` von `candle-core` aktiviert `cudarc`, `candle-kernels` und weitere Abhängigkeiten. Das Bauen braucht den CUDA-Toolkit/`nvcc`. [prüfen: aktuelle Cargo.toml]
        - Durchreichen:
          ```toml
          [features]
          default = []
          cuda = ["candle-core/cuda", "dep:cudarc"]
          [dependencies]
          candle-core = "…"
          cudarc = { version = "…", optional = true }
          ```
        - Bei optionalem `candle-core`: Weak-Dependency-Syntax `candle-core?/cuda`.

        **40. No-GPU-Pfad.**
        - Trait-Objekt-/Generics-Austausch statt `cfg`-Streuung: `cfg` nur beim Konstruieren des Backends (eine Factory), alles andere bleibt unabhängig vom Feature.
        - CI: Matrix mit und ohne `cuda`. Der No-GPU-Build muss ohne `nvcc` kompilieren und alle Unit-Tests ausführen. Der `cuda`-Build wird mindestens per `cargo check` geprüft.

        ---

        ## Offene Verifikationen (Zusammenfassung)
        `next-plaid` (Repo/Status), `cudarc` Graph-API und Cooperative-Launch-Wrapper, System-Atomics auf Host-Memory auf sm75 (`HostNativeAtomicSupported`), candle-Graph-Capture-Issues, `candle-transformers` XLM-RoBERTa-Classifier, aktuelle Versionsnummern (`thiserror`, `cudarc`, `candle`).
    errors:
    - path: /home/user/xyan/xy.ai.workbench/project/resident/plan/README.md
      error: File not found.
    - path: /home/user/xyan/xy.ai.workbench/project/resident/plan/plan_d.md
      error: File not found.
    - path: /home/user/xyan/xy.ai.workbench/project/resident/plan/plan_e.md
      error: File not found.
    - path: /home/user/xyan/xy.ai.workbench/project/resident/plan/E1.md
      error: File not found.
```
/answer 7e40-4 allow
ReasoningToken: 1845
Text:
 No existing plan files or conventions — the `plan/` directory is empty. I now have all sources (metaplan, Group E research `context_e.md`, the primary `resident_index.md`, `resident_subengine.md` §4.2/§4.4/§4.5/§4.6, and cross‑cutting Q1–Q4 + H1). I'll create one overview file plus one actionable agent plan per package E1–E9.
Control Request:
```yaml
id: 7e40-5
toolName: write
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/project/resident/plan/plan_e_overview.md
    mode: replace
    content: |
      # Umsetzungspläne Gruppe E — Index-Management & Synchronisation (Übersicht)

      > **Zielhardware:** sm_75 (Turing, GTX 1660 / TU116, 22 SMs, 6 GB VRAM).
      > **Projekt:** Rust-Crate unter `/home/user/xyan/xy.ai.workbench/rag`.
      > **Primärquelle:** `resident_index.md`. **Recherche je Paket:** `context/context_e.md`. **Querschnitt:** `context/context_h_q.md` (Q1–Q4).
      > **Hinweis:** `resident_index.md` ist die Spezifikation des **Vektorindex/Index-Managers**, keine Inhaltsübersicht. `resident_index.md` bezeichnet die reale Datei `resident_precision.md` fälschlich als `resident_precision_calibration.md`.

      Diese Übersicht bündelt die gruppenweiten Entscheidungen und verweist auf die neun Einzelpläne `plan_e1.md … plan_e9.md`. Jeder Einzelplan ist isoliert abarbeitbar (eigene Quellen, Voraussetzungen, Artefakte, Tests, Definition of Done).

      ---

      ## 0. Scope-Grenze (verbindlich)

      **In der Engine (Gruppe E baut dies):** `Index`-Trait + `StreamedIndexManager` (RAM ⇄ Index ⇄ VRAM-Spiegel), `ArenaAllocator` (GPU), `ScopeResolver` pro `AccessMethod`, Update-Queue, Residency-Tracker, Kommunikationsslot, zustandsbasierte Synchronisation/Echo-Vermeidung, Indexaufbau-Pfad (Kernel schreibt Segmente), Erweiterungspunkt für Addons.

      **Nicht in der Engine (außerhalb / Oberschicht):** Persistenzformat & -zeitpunkte des Index (gehören dem Index-Objekt), Chunking, Query-Formulierung, Reranking, Kandidaten-`k`, Indexstruktur selbst, Qualität/Recall, Drift-Monitor, approximative Vorfilter (Zentroid/FFT/Kompression verlieren bewusst Randtreffer — Eigenschaft der Oberschicht). Iteration/Kaskade/Rekursion sind **keine** Kernfunktion (nur Addon, E9).

      **Akzeptierte Invarianten:** Veraltete Ergebnisse sind Variablen, kein Fehler. Keine Sperren zugunsten Konsistenz. Stamp-Vergleich ausschließlich mit `!=`, nie mit `<`.

      ---

      ## 1. Konsistenzmodell Index-Sync (gruppenweite Festlegung)

      Das in `resident_subengine.md` §4.4 und im Metaplan (D7/E) als „noch festzulegen" markierte Konsistenzmodell wird hier verbindlich fixiert:

      1. **Source of Truth = Index (RAM-Primärinstanz).** VRAM ist ein abgeleiteter, verwerfbarer Spiegel. Richtung agnostisch RAM ⇄ Index ⇄ VRAM, aber bei Divergenz gewinnt der Index.
      2. **Identität = (EntryId, Stamp).** Stamp ist der Wert eines globalen, monotonen `AtomicU64` zum Write-Zeitpunkt (siehe E6). Vergleich nur mit `!=`. 64 Bit laufen praktisch nie über → kein ABA.
      3. **Zustandsbasierter Abgleich, nicht ereignisbasiert.** Das `watch<u64>`-Level-Signal weckt nur; verglichen werden Stamps **nur residenter Entries**. Verlorene/zusammengefasste Signale sind harmlos.
      4. **Segment-/Tabellen-Mutation per Copy-on-Write + Epoche (RCU-artig).** Große Strukturen werden nie in place geändert; neu anlegen → atomarer 8-Byte-Root-Pointer-Swap → alte Version erst freigeben, wenn kein Reader mehr darauf zeigt. Epochenzähler: Kernel meldet pro Batch die gelesene Epoche; Host gibt frei, sobald alle Reader ≥ neue Epoche (siehe E4).
      5. **Kein Epochen-/Sequenzfeld im Kernel-Ergebnisprotokoll.** Mutation nur in Lücken (Kommunikationsslot) ⇒ jedes Ergebnis sieht den Mapping-Stand seines Batches (siehe E5).
      6. **Schreibreihenfolge im Index:** erst Daten → dann Stamp → dann Signal. **Lesereihenfolge beim Laden:** Stamp → Daten → (bei mmap) Stamp erneut prüfen.
      7. **Last-Writer-Wins** über mehrere Schreiber; `apply` wird serialisiert. Zwei Engines konvergieren über den Stamp-Vergleich auf den Indexstand.

      ---

      ## 2. Konsolidierung D3 ↔ E3 (Arena)

      D3 (Subengine-GPU-Speicher) und E3 (ArenaAllocator) beschreiben dieselbe Allokationsschicht. **Festlegung:** Es gibt **einen** `ArenaAllocator`/`MemoryManager` (aus Gruppe B/D), E3 ist seine **index-/segmentspezifische Nutzung** (Index-Segment-Arena, CoW-Doppelbedarf, Copy-Stream mit Pinned Staging). `plan_e3.md` spezifiziert ausschließlich die Index-Arena-Nutzung und verweist auf den gemeinsamen Allocator; keine zweite Allokator-Implementierung.

      ---

      ## 3. Voraussetzungen aus anderen Gruppen

      | Abhängigkeit | Liefert | Genutzt in |
      |---|---|---|
      | **B4** Index-Trait & -Vertrag | `Index`, `EntryId`, `Stamp`, `EntrySet`, `Scope`, `AccessMethod`, `SummaryRef`, `watch<u64>` | E1, E2, E5, E6, G2 |
      | **B5** Kernel-Abstraktion | `SegmentDesc`, `Layout{Fixed/Variable}`, `KernelHit`, Kernel-Trait + Mock | E4, E5, E8 |
      | **B2 / D2 / D3** Speicher/Device/Arena | `Region{arena,offset,len,gen}`, `MemoryManager`/`ArenaAllocator`, Copy-Stream, Pinned Staging, `cuMemGetInfo`-Budget | E3, E4, E7 |
      | **D7** Index-Segmente in der Subengine | `SegmentView`/Slot/Slot-Klassen, `(segment,slot)→EntryRef` | E1, E5, E8 |
      | **C3** Planner | Op-Expansion, erweiterbares Parameterobjekt | E9 |
      | **Q1–Q4** Querschnitt | Fehler-Taxonomie, `tracing`-Spans, Mocks/No-GPU, `cuda`-Feature-Flag | alle E |

      Fehlt eine Voraussetzung noch, baut der Agent gegen die in B4/B5 vereinbarten Trait-Signaturen und einen Mock.

      ---

      ## 4. Empfohlene Modulstruktur (Rust)

      ```
      index/            # B4: Index-Trait, EntryId, Stamp, EntrySet, Scope, AccessMethod (fremd)
      manager/
        mod.rs         # StreamedIndexManager (E1)
        scope.rs       # ScopeResolver-Registry + Resolver (E2)
        arena.rs       # Index-Segment-Arena-Nutzung (E3, nutzt gemeinsamen Allocator)
        swap.rs        # Manager-Disziplin: CoW, Pointer-Swap, Acquire-Load-Bindung (E4)
        slot.rs        # Kommunikationsslot, feste Reihenfolge (E5)
        sync.rs        # Zustandsabgleich, Echo-Vermeidung, Residency-Tracker (E6)
        residency.rs   # Lade-/Eviction-Strategie, Single-Flight, Gather/Segment (E7)
        build.rs       # Kernel schreibt Segmente, Rückabbildung Adresse→Entry (E8)
      addons/          # Erweiterungspunkt (E9) — nur Konzept, kein Kern
      ```

      Reale CUDA-Pfade (Acquire-Load, Pinned Staging, Arena, Copy-Stream) liegen hinter `#[cfg(feature = "cuda")]`; die gesamte Logik läuft gegen Traits (`ComputeBackend`, Kernel-Trait) und ist ohne GPU testbar (Q4).

      ---

      ## 5. Reihenfolge & Abhängigkeitsgraph

      ```
      E1 (Grundgerüst)
       ├─ E2 (Scope & Resolver)            [braucht E1, B4]
       ├─ E3 (Index-Arena)                 [braucht D3]
       │   └─ E4 (Manager-Disziplin)       [braucht E1, E3, B5]
       │       └─ E5 (Kommunikationsslot)  [braucht E4, B4, B5]
       │           ├─ E6 (Sync/Echo)       [braucht B4, E5]
       │           ├─ E7 (Residency)       [braucht E3, E5]
       │           └─ E8 (Indexaufbau)     [braucht E5, B5]
      E9 (Addons)                          [braucht B1, C3 — unabhängig]
      ```

      Sequenz für den Lauf: **E1 → E2 → E3 → E4 → E5 → E6 → E7 → E8 → E9**.

      **Hochrisiko (zuerst prototypisch absichern):** E4 (Acquire-Load/Fence-Sichtbarkeit Host↔GPU) und E6 (Konsistenzmodell). Siehe die „Offene Verifikationen" der jeweiligen Pläne.

      ---

      ## 6. Gemeinsame Testinfrastruktur (Q3)

      - **`FakeKernel`** (manueller Fake, nicht `mockall`): programmierbare Ergebnisse, geschriebene Bereiche, Fortschrittszähler; implementiert das Kernel-Trait (B5). Treibt E4/E5/E8-Tests ohne GPU.
      - **`FakeIndex`**: `Index`-Trait mit steuerbaren Stamps/`apply`-Rückgaben, programmierbarem `changed()`-Signal; treibt E1/E2/E6.
      - **Nebenläufigkeit:** `loom`/`shuttle` für die CoW-/Epoch-/Slot-Protokolle (E4/E5), `proptest` für Scan-Sequenzen (E6). Virtuelle Zeit (`tokio::time::pause`) für Idle/Eviction (E7). Failpoints zum Erzwingen von Interleavings (Stamp zwischen Stamp- und Datenlesen ändern, E6).
      - Alle Tests laufen **ohne** `cuda`-Feature (No-GPU-Pfad). CUDA-Pfade nur per `cargo check` + Hardware-Smoke-Test.

      ---

      ## 7. Gruppenweite Definition of Done

      - Alle in E1–E8 genannten Tests aus `resident_index.md` („Tests"-Liste) grün im No-GPU-Pfad.
      - Konsistenzmodell (Abschnitt 1) in Code-Doku und Tests nachweisbar verankert.
      - Fehler-Taxonomie (`UnsupportedScope`, `IndexUnavailable`, `ArenaExhausted`, `StaleMapping` intern, `TransferFailed`) vollständig; kein stilles Ausweichen auf andere Verfahren/Scopes.
      - `tracing`-Spans pro Task und Manager-Zyklus; geloggt: Scan-Ergebnis (geladen/veraltet/entfernt), Lade-/Eviction-Entscheidung, verworfene & `outdated` Ergebnisse, Arena-Auslastung.
      - Offene Verifikationen (sm_75-Atomics, Primary-Context-Größe, madvise-Semantik, Gather/Segment-Schwelle) dokumentiert mit Status (verifiziert/offen).
  - path: /home/user/xyan/xy.ai.workbench/project/resident/plan/plan_e1.md
    mode: replace
    content: |
      # Umsetzungsplan E1 · StreamedIndexManager-Grundgerüst

      > **Quellen:** `resident_index.md` (Drei Schichten, Leitprinzipien, Umsetzungsvorgaben). **Recherche:** `context_e.md` §E1 (Punkte 1–4). **Querschnitt:** Q1, Q2, Q4.
      > **Abhängt von:** B4 (Index-Trait), D7 (Segment-Repräsentation). **Ergebnis:** Manager-Rollen-Spezifikation + lauffähiges Grundgerüst.

      ## 1. Ziel & Abgrenzung
      Das Skelett des `StreamedIndexManager` als **Komponente der CPU/GPU-Engine** (nicht Fassade, nicht Index). Konstruktion mit einer Engine-Referenz und einem **lazy aktivierten** Index; der Index bleibt unabhängig nutzbar. Zwei Ausprägungen:
      - **CPU-Manager:** trivial — liest den Index direkt (RAM ist Primärinstanz), fast keine Arbeit. Einzige Aufgabe: Residency bei mmap-Indizes über RAM-Größe (Prefetch via `madvise`, Entladen nach Idle).
      - **GPU-Manager:** vollständiger Streaming-Manager (Fassade/Hülle; die konkrete Logik entsteht in E2–E8).

      Nicht hier: Scope-Auflösung (E2), Arena (E3), Mutationsdisziplin (E4), Slot (E5), Sync (E6), Residency-Strategie (E7), Indexaufbau (E8).

      ## 2. Voraussetzungen
      - B4-Trait `Index` inkl. `EntryId`, `Stamp`, `EntrySet`, `AccessMethod`, `watch<u64>`.
      - D7-Typen `SegmentView`/Slot und Mapping `(segment,slot)→EntryRef` (als Signaturen).
      - Gemeinsame Engine-Handle-Abstraktion (CPU/GPU) aus Gruppe D.

      ## 3. Zu bauende Artefakte
      ```rust
      pub struct StreamedIndexManager<B: ComputeBackend> {
          engine: EngineHandle<B>,
          index: OnceCell<Arc<dyn Index>>,   // lazy aktiviert
          residency: ResidencyTracker,       // Mapping Entry <-> Slot + bekannte Stamps (Platzhalter, Ausbau E6)
          changed_rx: Option<watch::Receiver<u64>>,
      }

      pub trait IndexManager {               // gemeinsame Schnittstelle CPU/GPU
          fn attach_index(&self, index: Arc<dyn Index>);
          fn is_active(&self) -> bool;        // Index geladen?
      }

      pub struct CpuIndexManager { /* direkter Lesezugriff + mmap-Residency */ }
      pub struct GpuIndexManager<B> { /* Streaming-Hülle */ }
      ```
      - `ResidencyTracker` zunächst als Platzhalter (Map `EntryId → (Slot, Stamp)`), voll ausgebaut in E6/E7.

      ## 4. Entscheidungen (aufgelöste „Zu klären"-Punkte)
      **mmap-Residency über RAM-Größe & `madvise`-Prefetch** (context_e.md §E1, `[W]`):
      - **Prefetch:** `memmap2::Mmap::advise(Advice::WillNeed)` (safe) für heiße Bereiche; `Advice::Sequential` bei dichten Segment-Scans, `Advice::Random` bei Gather. Alles nur Hints, kein Garantieversprechen.
      - **Entladen/Kaltstellen:** `MADV_DONTNEED` entfernt nur Seiten aus den Page Tables, **nicht** aus dem Page-Cache (file-backed shared). Für echtes Kaltstellen `MADV_COLD`/`MADV_PAGEOUT` (Linux ≥ 5.4) bzw. `posix_fadvise(POSIX_FADV_DONTNEED)`. `mlock` **nur** für kleine Summary-Strukturen.
      - **Residency-Messung:** `mincore(2)` (via `libc`/`rustix`) liefert pro Page „im Page-Cache resident" — nur Momentaufnahme unter Last.
      - **Soft-Grenze:** Keinen eigenen Cache über mmap bauen — **mit** dem Page-Cache arbeiten. Working-Set-Fenster: `WILLNEED` für heiße, `COLD`/`PAGEOUT` für kalte Bereiche; optional cgroup v2 `memory.high` (Throttling statt OOM).
      - **Blockierende Zugriffe:** Page-Faults in async-Tasks blockieren den Worker-Thread. mmap-Lesen/Prefetch daher in `spawn_blocking` oder dediziertem Thread-Pool; Ergebnis über `oneshot`/Channel. **Fallstricke:** `spawn_blocking`-Tasks sind nicht abbrechbar; Default-Pool (512) kann bei Fault-Stürmen erschöpfen; `&[u8]`-Zugriff in `async`-Code ist ein unsichtbarer Block.

      **CPU- vs. GPU-Manager:** gemeinsame `IndexManager`-Schnittstelle; CPU-Variante delegiert Reads direkt an den Index + mmap-Residency; GPU-Variante hält zusätzlich die Streaming-Zustände (ab E3).

      ## 5. Umsetzungsschritte
      1. `IndexManager`-Trait + `StreamedIndexManager`-Struct mit `OnceCell<Arc<dyn Index>>` (lazy, `get_or_try_init`).
      2. `attach_index` / Lazy-Aktivierung; bei erstem Zugriff `changed()`-Receiver abonnieren.
      3. `CpuIndexManager`: direkter `index.read(entry)` im Blocking-Pool; mmap-Residency-Hooks (`advise`).
      4. `GpuIndexManager`: leere Hülle mit Feldern für Arena/Slot/Tracker (Platzhalter für E3–E8), hinter `#[cfg(feature = "cuda")]` für reale Pfade.
      5. `ResidencyTracker`-Grundstruktur (Map + bekannter Stamp je Entry).

      ## 6. Fehler / Logging / Feature-Flags
      - **Q1:** `IndexUnavailable` (Index noch nicht aktiv / Ladefehler). Modul-Enum `ManagerError`, per `#[from]` ins Top-Level.
      - **Q2:** Span pro Manager-Zyklus (`#[instrument(skip_all, fields(index_id))]`); Events: Index aktiviert, Residency-Hint gesetzt/entfernt.
      - **Q4:** mmap-Pfad feature-frei (CPU); GPU-Hülle hinter `cuda`.

      ## 7. Tests (Q3)
      - Lazy-Aktivierung: Manager verbraucht ohne Index keine Ressourcen; erster Zugriff aktiviert.
      - CPU-Manager liest Entry direkt; blockierender Read läuft im Blocking-Pool (kein async-Worker-Block — über instrumentierten Fake-Clock/Hook prüfen).
      - mmap-Residency: `WillNeed`/`Cold` werden an der richtigen Stelle gesetzt (Fake-`Advise`-Sink zählt Aufrufe).
      - `FakeIndex` ohne echte Datei für alle Logiktests.

      ## 8. Offene Verifikationen
      - Linux-`madvise`-Semantik gegen man7 prüfen; `memmap2`-`Advice`-API gegen aktuelle Crate-Version (context_e.md offener Punkt 3).
      - `MADV_COLD`/`PAGEOUT` erst ab Kernel ≥ 5.4 — Zielkernel bestätigen.

      ## 9. Definition of Done
      - Lazy-Manager konstruierbar, CPU-Variante liest Index korrekt, mmap-Residency-Hooks getestet, GPU-Hülle kompiliert mit und ohne `cuda`. Keine Ressourcenbelegung vor erstem Zugriff.
  - path: /home/user/xyan/xy.ai.workbench/project/resident/plan/plan_e2.md
    mode: replace
    content: |
      # Umsetzungsplan E2 · Scope & Resolver

      > **Quellen:** `resident_index.md` (Scope und Auflösung). **Recherche:** `context_e.md` §E2 (Punkte 5–7). **Querschnitt:** Q1, Q2, Q3.
      > **Abhängt von:** B4 (Index-Trait, `Scope`, `AccessMethod`), E1 (Manager). **Ergebnis:** Scope-Resolver-Spezifikation + Registry.

      ## 1. Ziel & Abgrenzung
      Der **Manager** (nicht der Index) löst den Scope eines Tasks für kompatible Indextypen auf. Pluggable `ScopeResolver` pro `AccessMethod`. **Keine stille Ausweichung** auf `All`: Fehlt Verfahren oder Resolver → `UnsupportedScope`. Ergebnis eines Resolvers ist eine **kompakte Entry-Menge** (Ranges oder Bitmap), keine ID-Liste.

      Nicht hier: das tatsächliche Laden der aufgelösten Mengen (E7), die Geometrie-Berechnung selbst (Kernel/Engine).

      ## 2. Voraussetzungen
      - B4: `enum Scope { All, Range(..), Set(..), Subtree(NodeId), Prefix(..), Centroid{k,radius} }`, `AccessMethod`, `Index::capabilities()`, `Index::summary()`, `Index::children()`, `Index::members()`.
      - E1: Manager-Rahmen.

      ## 3. Zu bauende Artefakte
      ```rust
      pub trait ScopeResolver: Send + Sync {
          fn method(&self) -> AccessMethod;
          // löst einen Scope in eine kompakte Entry-Menge auf; nutzt Index-Struktur, keine Geometrie-Interpretation im Index
          fn resolve(&self, index: &dyn Index, scope: &Scope, ctx: &ResolveCtx) -> Result<EntrySet, ManagerError>;
      }

      pub struct ResolverRegistry { map: HashMap<AccessMethod, Arc<dyn ScopeResolver>> }
      impl ResolverRegistry {
          pub fn register(&mut self, r: Arc<dyn ScopeResolver>);  // Duplikate ablehnen/überschreiben: definieren
          pub fn resolve(&self, index: &dyn Index, scope: &Scope) -> Result<EntrySet, ManagerError>;
      }
      ```
      - `EntrySet` als **Roaring-Bitmap** (siehe Entscheidung) mit Range-Shortcut.
      - `CentroidResolver` als Referenz-Resolver.

      ## 4. Entscheidungen (aufgelöste „Zu klären"-Punkte)
      **Resolver-Registrierung & Kompatibilitätsprüfung** (context_e.md §E2 Punkt 5, `[W]`):
      - **Variante:** `HashMap<AccessMethod, Arc<dyn ScopeResolver>>` mit dyn-Dispatch — Erweiterung ohne Kernänderung, `Send + Sync`, object-safe (keine generischen Trait-Methoden). Enum-Dispatch verworfen (geschlossen); `TypeId`-Any-Registry verworfen (Laufzeit-Downcast-Fehler).
      - **Kompatibel heißt zweifach:** Der Index **deklariert** das Verfahren (`capabilities()` enthält `AccessMethod`) **UND** die Registry hat einen Resolver dafür. Nur dann Auflösung, sonst `UnsupportedScope{method}`. **Kein** stilles `All`.
      - **Registrierungsreihenfolge/Duplikate:** Registrierung bei Manager-Aufbau; Duplikat einer `AccessMethod` ist ein Konfigurationsfehler → Panik beim Aufbau oder `Err` (nicht stilles Überschreiben). Keine Hot-Registration im Betrieb (Lebensdauer-Fallstrick).

      **Kompakte Entry-Menge — Roaring vs. Ranges** (context_e.md §E2 Punkt 7, `[W]`):
      - Standard: **`roaring`-Bitmap** (Container pro 2¹⁶-Block, Array/Bitmap/Run). Run-Container deckt dichte *und* fragmentierte Fälle ab, schnelle Union/Schnitt (für Scope-Aggregation in E7). Reine Range-Listen nur, wenn Scopes **garantiert** zusammenhängend sind.

      **Zentroid-Resolver (Referenz)** (context_e.md §E2 Punkt 6, `[V]` PLAID/ColBERTv2):
      - Manager holt Summary (`summary(Centroid)`) und macht sie resident. Engine/Kernel rechnet `S = C · Qᵀ`, pro Query-Token die `nprobe` nächsten Zentroide; `members(cell)` des Index liefert Kandidaten (PLAID: Zentroid → eindeutige Passage-IDs). Resolver gibt deren Vereinigung als `EntrySet` zurück. `k`/`radius` aus `Scope::Centroid`. Approximation (Randtreffer-Verlust) ist bewusst Eigenschaft der Oberschicht.

      ## 5. Umsetzungsschritte
      1. `ScopeResolver`-Trait + `ResolverRegistry` mit `register`/`resolve` und Doppelprüfung (deklariert ∧ Resolver vorhanden).
      2. `EntrySet` auf `roaring` abstützen; Range-Konstruktor als Shortcut.
      3. Triviale Resolver: `RangeResolver`, `SubtreeResolver` (nutzt `children()`), `PrefixResolver`.
      4. `CentroidResolver`: Summary resident machen (Delegation an E7-Hook), Zellenwahl, `members()`-Union.
      5. `resolve(All)` → vollständige Entry-Menge aus `stamps()`-Domäne (ohne Vektorseiten).

      ## 6. Fehler / Logging / Feature-Flags
      - **Q1:** `UnsupportedScope{method}` (Verfahren nicht deklariert ODER kein Resolver). Keine Ersatzauflösung.
      - **Q2:** Span pro Resolve; Event: gewähltes Verfahren, Größe der aufgelösten Menge.
      - **Q4:** Zentroid-Rechnung auf GPU hinter `cuda`; Resolver-Logik feature-frei testbar.

      ## 7. Tests (Q3)
      - `UnsupportedScope`, wenn Verfahren nicht deklariert.
      - `UnsupportedScope`, wenn deklariert, aber kein Resolver registriert.
      - Range/Subtree/Prefix-Resolver gegen `FakeIndex` liefern erwartete `EntrySet`.
      - Zentroid-Resolver mit Fake-Summary + Fake-`members()`: korrekte Vereinigung, `nprobe`/`k` respektiert.
      - Duplikat-Registrierung wird abgelehnt.
      - Roaring-Union zweier Scopes = erwartete Menge (Vorbereitung E7-Aggregation).

      ## 8. Offene Verifikationen
      - `roaring`-Crate-API/Version prüfen.
      - PLAID-Zählparameter (`nprobe`, Kandidatenzahl) sind Oberschicht-Parameter; hier nur Durchreichung — Schnittstelle gegen G3/H3 abstimmen.

      ## 9. Definition of Done
      - Registry mit Doppelprüfung; mindestens Range/Subtree/Prefix/Centroid-Resolver; `EntrySet` roaring-basiert mit Union; alle genannten Tests grün ohne GPU.
  reason: Create the Group E overview/sequencing plan and the per-package agent implementation plans E1 and E2 in the target plan directory.
```
/answer 7e40-5 allow
ReasoningToken: 4301
Text:
 Overview, E1, E2 written. Now E3, E4, E5 (the hardest: arena + manager discipline + communication slot).
Control Request:
```yaml
id: 7e40-6
toolName: write
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/project/resident/plan/plan_e3.md
    mode: replace
    content: |
      # Umsetzungsplan E3 · ArenaAllocator (GPU, Index-Segmente)

      > **Quellen:** `resident_index.md` (Manager-Disziplin/Arena); **überlappt D3** (`resident_subengine.md` §4.2). **Recherche:** `context_e.md` §E3 (Punkte 8–12). **Querschnitt:** Q1, Q2, Q4.
      > **Abhängt von:** D3 (gemeinsamer Allocator/MemoryManager). **Ergebnis:** Index-Arena-Spezifikation, konsolidiert mit D3.

      ## 1. Ziel & Abgrenzung
      Die **index-/segmentspezifische Nutzung** der gemeinsamen GPU-Arena: lazy Anlage nach VRAM-Budget, Free-List, `u64`-Arena-Offsets, eingeplanter Copy-on-Write-Doppelbedarf, Copy-Stream mit Pinned Staging. **Konsolidierung D3↔E3:** Es gibt genau **einen** `ArenaAllocator`/`MemoryManager` (aus D3). Dieser Plan definiert dessen Nutzung für Index-Segmente; **keine** zweite Allokator-Implementierung.

      Nicht hier: Pointer-Swap-Disziplin & Acquire-Load (E4), Lade-/Eviction-Strategie (E7).

      ## 2. Voraussetzungen
      - D3: `ArenaAllocator` mit `alloc/free/resolve`, `Region{arena,offset,len,gen}`, Budget via `cuMemGetInfo`, stream-geordnete Wiederverwendung, Copy-Stream + Pinned Staging.
      - B2: `Region`-Vertrag (kein Pointer/Tensor in Signaturen).

      ## 3. Zu bauende Artefakte
      ```rust
      pub struct IndexArena<'a> { inner: &'a ArenaAllocator, base: DeviceBase /* u64-Basis */ }
      impl IndexArena<'_> {
          fn alloc_segment(&self, bytes: usize, align: usize) -> Result<Region, ManagerError>; // 256B-align
          fn reserve_writable(&self, cap: usize) -> Result<WritableRegion, ManagerError>;      // für E8-Kernel-Writes
          fn free(&self, r: Region);                                                           // stream-geordnet
          fn budget(&self) -> ArenaBudget;                                                      // live, reserve, cow_headroom
      }
      pub struct CowBudget { live: u64, max_concurrent_cow: u32, largest_segment: u64 }
      ```

      ## 4. Entscheidungen (aufgelöste „Zu klären"-Punkte)
      **VRAM-Budget** (context_e.md §E3 Punkt 9, `[W]`): `cuMemGetInfo` **nach** Context-Erzeugung und **nach** Laden von cuBLAS/cuDNN/Kernel-Modulen aufrufen; Budget = `free − Sicherheitsreserve`. Primary Context + Libraries auf Turing grob **300–600 MB** — auf der Zielkarte (GTX 1660) **messen**. Display-GPU belegt zusätzlich.

      **Arena statt Runtime-Alloc** (context_e.md §E3 Punkt 8, `[V]`): `cudaMalloc`/`cudaFree` synchronisieren über laufende Streams → im Betrieb vermeiden. **Einmal groß** `cudaMalloc` beim ersten Laden, danach nur Suballokation (keine Runtime-Syncs). `cudaMallocAsync`-Pool (ab CUDA 11.2) ist pool-basiert und syncfrei, für eine **feste** Arena aber nicht nötig; Turing-Nutzbarkeit nur über `cudaDevAttrMemoryPoolsSupported` zur Laufzeit prüfen.

      **Suballocator** (context_e.md §E3 Punkt 10, `[W]`): **TLSF** (O(1), geringe Fragmentierung) für variable Segmentgrößen; **Slab** nur falls Segmente feste Größe haben (Slot-Klassen aus D7). Buddy verworfen (bis 50 % interne Fragmentierung). **Alignment:** `cudaMalloc` liefert ≥ 256 B; Suballokationen auf **256 B** alignen (Coalescing/Vektor-Loads ≥ 16 B). Adressierung als **`u64`-Offset relativ zur Arena-Basis** (Pointer = Basis + Offset); 32-Bit nur bei < 4 GiB.

      **CoW-Doppelbedarf** (context_e.md §E3 Punkt 11, `[W]`): Heuristik **Budget ≥ Live-Daten + (max. gleichzeitige CoW-Kopien × größtes Segment)**. Maßnahmen: Segmente **klein** halten (begrenzt Spitzenbedarf), CoW **seriell** statt parallel, alte Kopie erst nach **Epochenende** freigeben (Kopplung an E4). Kein zitierfähiges Standardverfahren — als Heuristik dokumentieren.

      **Pinned Staging / Copy-Stream** (context_e.md §E3 Punkt 12, `[W]`): `cudaHostAlloc` ist teuer (Page-Locking, ms) → **einmalig** Pool anlegen, wiederverwenden. Pinned ermöglicht echt asynchrone H2D/D2H und volle PCIe-Bandbreite. **Separater Copy-Stream** (nicht-blockierend priorisiert) für Überlappung Copy ↔ Compute (Turing hat Copy-Engines). `cudaHostAllocWriteCombined` **nicht** setzen (macht Host-Reads extrem langsam). Pinned-Gesamtmenge begrenzen (OS-Limit/RAM-Druck).

      ## 5. Umsetzungsschritte
      1. `IndexArena` als dünne Hülle um `ArenaAllocator` mit 256-B-Alignment und `u64`-Offset-Adressierung.
      2. Budgetermittlung: `cuMemGetInfo` nach Lib-Load, Reserve-Parameter, `CowBudget`-Berechnung; `ArenaExhausted` bei Unterschreitung.
      3. Lazy-Anlage beim ersten Segment-Load; Free-List; stream-geordnete Freigabe (Event-gebunden, Kopplung D3).
      4. `reserve_writable` für E8 (Cursor-Writes); Pinned-Staging-Pool + Copy-Stream als gemeinsame Ressource referenzieren.
      5. CoW-Kopie seriell, Freigabe der alten Kopie an Epochenende (Callback aus E4).

      ## 6. Fehler / Logging / Feature-Flags
      - **Q1:** `ArenaExhausted` (Budget inkl. CoW-Headroom unterschritten), `TransferFailed` (Copy-Stream).
      - **Q2:** Events: Arena-Anlage (Größe, Budget, Reserve), Suballokation/Free, Arena-Auslastung pro Zyklus, CoW-Spitze.
      - **Q4:** Gesamte CUDA-Mechanik hinter `cuda`; ohne Feature ein **Host-Arena-Mock** (Vec-Backed, gleiche Offset-Semantik) für Logiktests.

      ## 7. Tests (Q3)
      - Suballokation/Free-List: Offsets 256-B-aligned, keine Überlappung, Wiederverwendung nach Free.
      - Budget: `ArenaExhausted`, wenn Live + CoW-Headroom das Budget überschreitet.
      - CoW seriell: zweite CoW wartet, alte Kopie wird erst nach Epochenende frei (Fake-Epoch-Hook, Kopplung E4).
      - Host-Arena-Mock liefert deterministische Offsets für E4/E5/E8-Tests.

      ## 8. Offene Verifikationen
      - Primary-Context-Größe und `cudaDevAttrMemoryPoolsSupported` auf GTX 1660 messen (context_e.md offener Punkt 2).
      - TLSF- vs. Slab-Entscheidung an realen Slot-Klassen (D7) überprüfen.

      ## 9. Definition of Done
      - Eine Arena-Implementierung (konsolidiert mit D3), 256-B-Offset-Suballokation, budgetierte CoW-Heuristik, Pinned-Staging/Copy-Stream referenziert; Host-Mock-Pfad testbar ohne GPU.
  - path: /home/user/xyan/xy.ai.workbench/project/resident/plan/plan_e4.md
    mode: replace
    content: |
      # Umsetzungsplan E4 · Manager-Disziplin (Hochrisiko)

      > **Quellen:** `resident_index.md` (Manager-Disziplin). **Recherche:** `context_e.md` §E4 (Punkte 13–17, **Hochrisiko**), ergänzend `context_h_q.md` H1 (3–5). **Querschnitt:** Q1, Q2, Q3, Q4.
      > **Abhängt von:** E1, E3, B5 (Kernel-Trait). **Ergebnis:** Mutations-/Swap-Disziplin.

      ## 1. Ziel & Abgrenzung
      Die Disziplin, mit der der Manager **nie den Kernel blockiert**: Mutation nur in **Lücken** (Batch-Ende/Kommunikationsslot), **atomarer 8-Byte-Root-Pointer-Swap**, **Copy-on-Write-Hierarchie** (Root → Segmenttabelle → Segment), definierter **Pointer-Lesezeitpunkt** (Launch-Arg vs. Acquire-Load im Ringmodus), **Event/Fence-Sichtbarkeit** Host↔GPU, blocking Reads im Worker-Pool.

      Nicht hier: die feste Slot-Reihenfolge selbst (E5), Residency (E7).

      ## 2. Voraussetzungen
      - E1 Manager-Rahmen; E3 Arena (`u64`-Offsets, CoW-Budget); B5 Kernel-Trait (Batch-Start/-Ende, Fortschrittszähler).
      - Konsistenzmodell aus `plan_e_overview.md` §1 (RCU + Epoche, Stamp-`!=`).

      ## 3. Zu bauende Artefakte
      ```rust
      pub struct RootPointer(AtomicU64);          // 8-Byte, naturally aligned, in pinned+mapped Host-Memory
      pub struct SegmentTable { /* CoW, immutable nach Publish */ }
      pub struct EpochTracker { published: u64, reader_min: AtomicU64 } // Kernel meldet gelesene Epoche je Batch

      pub trait TableSwap {
          fn build_next(&self) -> SegmentTable;            // CoW neue Tabelle
          fn publish(&self, table: SegmentTable);          // (1) Inhalt sichtbar machen, (2) Pointer mit Release swappen
          fn reclaim(&self, epoch: u64);                   // alte Version frei, wenn alle Reader >= epoch
      }
      ```

      ## 4. Entscheidungen (aufgelöste „Zu klären"-Punkte)
      **`ld.acquire`/Fence auf sm_75** (context_e.md §E4 Punkte 13–15, H1 Punkt 5): PTX-Scoped-Memory-Modell mit `ld.acquire`/`st.release`/`fence.sc|acq_rel` ab **sm_70 (Volta)**; Turing (sm_75) gehört dazu. Syntax `ld.acquire.sys.u32 %r, [addr];`. Host↔Device erfordert Scope **`.sys`**. `__threadfence()` ist nur device-scope (`membar.gl` = `fence.sc.gpu` ab sm_70) — **genügt bei Host-Beteiligung nicht**. ABI: Release-Store = `st.release.<scope>`, Acquire-Load = `ld.acquire.<scope>` (empfohlen) bzw. `cuda::atomic_ref<…, thread_scope_system>`.

      **Sichtbarkeits-Reihenfolge vor Pointer-Swap** (context_e.md §E4 Punkt 14, `[W]` aus `[V]` abgeleitet): (1) Tabelle schreiben/kopieren → (2) Abschluss sicherstellen (`cudaStreamSynchronize`/Event bei Copy-Stream; bei gemapptem Host-Memory Release-Store) → (3) Pointer mit **Release** publizieren → (4) Kernel liest Pointer mit **Acquire (.sys)**. Erst danach sind alle vor dem Release geschriebenen Daten sichtbar. Eine `cudaMemcpyAsync` in anderem Stream wird für einen **laufenden** persistenten Kernel erst durch Host-Event/Flag-Publish zu definiertem Zeitpunkt sichtbar (Streamordnung gilt nicht zwischen konkurrierenden Streams).

      **Atomarer 8-Byte-Swap Host↔GPU** (context_e.md §E4 Punkt 15, `[W/?]`): **8-Byte-aligned `AtomicU64`** in **pinned+mapped** Host-Memory (`cudaHostAllocMapped`); Host `store(Release)`, GPU `ld.acquire.sys`. Reine Load/Store-Atomarität für naturally-aligned 8 B ist gegeben. **Single-Writer-Store (nur Host schreibt)** — **kein** GPU-seitiges RMW/CAS über PCIe nötig (PCIe-Atomics plattformabhängig). Unified/Managed mit gleichzeitigem CPU/GPU-Zugriff unter Windows nicht verfügbar → **mapped pinned** verwenden.

      **Lock-free-Muster (Reader=GPU, Writer=Host)** (context_e.md §E4 Punkt 16, `[W]`): **Immutable Segmente + CoW** (RCU-artig): neue Segmenttabelle bauen, Root-Pointer **einmal** swappen; alte Version erst freigeben, wenn kein Reader mehr darauf zeigt. **Epochenzähler:** Kernel meldet pro Batch die gelesene Epoche (Fortschrittszähler); Host gibt frei, sobald **alle Reader ≥ neue Epoche**. **Nie in-place mutieren** (Ausnahme: dokumentierter persistenter In-Place-Update, der halb aktualisierte, aber gültige Werte erlaubt).

      **Pointer-Lesezeitpunkt** (context_e.md §E4 Punkt 17, `[W]`): **Normale Launches:** Pointer als **Launch-Argument** → konsistenter Snapshot pro Launch, Wechsel ab nächstem Launch. **Persistenter Kernel:** Acquire-Load **einmal pro Batch** in ein Register, nicht pro Work-Item mitten im Batch; Mischzustände innerhalb eines Batchs vermeiden; `volatile`/relaxed-Loads ohne Acquire vermeiden (Compiler-Caching).

      ## 5. Umsetzungsschritte
      1. `RootPointer` als `AtomicU64` in pinned+mapped Region (hinter `cuda`); ohne Feature als normaler `AtomicU64` im Host-Mock.
      2. `SegmentTable` immutable nach Publish; `build_next` erzeugt CoW-Kopie (nutzt E3-Arena, serielle CoW).
      3. `publish`: Reihenfolge (1)–(4) strikt; Copy-Stream-Event abwarten, dann Release-Store.
      4. `EpochTracker`: Kernel-Batch meldet gelesene Epoche (B5-Fortschrittszähler); `reclaim(epoch)` gibt alte Tabelle/Segmente frei, wenn `reader_min ≥ epoch`.
      5. Normale Launches: Pointer als Launch-Arg; persistenter Modus (Hook zu H1): Acquire-Load einmal pro Batch.
      6. Blockierende Index-Reads (Page-Faults/I/O) in Blocking-/Worker-Pool; `madvise(WILLNEED)` für Überlappung.

      ## 6. Fehler / Logging / Feature-Flags
      - **Q1:** `StaleMapping` (nur intern; interner Retry), `TransferFailed`, `ArenaExhausted` (CoW-Spitze).
      - **Q2:** Span pro Manager-Zyklus; Events: Publish (Epoche, Tabellengröße), Reclaim (freigegebene Epoche), gelesene Reader-Epoche. CUDA-Host-Callbacks: **keine** CUDA-API/kein Logging darin — Daten per Channel/Atomic an Logging-Thread (H1 Punkt 32/Q2).
      - **Q4:** Acquire-Load/Release-Store/pinned-mapped hinter `cuda`; Host-Mock bildet Release/Acquire mit `std::sync::atomic` nach (x86 TSO) für Logiktests.

      ## 7. Tests (Q3)
      - **CoW-Swap:** altes Segment wird erst frei, wenn kein **vor** dem Swap eingereihter Request es mehr referenziert (Epoch-Reclaim), `FakeKernel` meldet Epochen.
      - Pointer-Lesezeitpunkt: Launch-Arg-Snapshot bleibt konstant während eines Batchs; Swap wirkt ab nächstem Launch.
      - Reihenfolge Publish (1)–(4): Inhalt vor Pointer sichtbar (Host-Mock mit erzwungenem Interleaving / `loom`).
      - Epoch-Reclaim: Freigabe erst bei `reader_min ≥ published`.
      - `loom`/`shuttle` auf dem Swap-/Reclaim-Protokoll (kleines CAS-/Store-Protokoll).

      ## 8. Offene Verifikationen (zuerst, Hochrisiko)
      - **`ld.acquire.sys` und mapped-Atomics** direkt in der PTX-ISA (parallel-thread-execution) bestätigen **und auf der echten GTX 1660 testen** (context_e.md offener Punkt 1).
      - `cudaDevAttrHostNativeAtomicSupported` prüfen (H1 Punkt 4) — bestätigt die Single-Writer-Store-Strategie (kein GPU-CAS über PCIe).
      - Event-/Fence-Sichtbarkeit zwischen Copy-Stream und persistentem Kernel auf Zielhardware messen.

      ## 9. Definition of Done
      - CoW + atomarer Root-Swap + Epoch-Reclaim implementiert und per `loom`/Host-Mock verifiziert; Publish-Reihenfolge erzwungen; Pointer-Lesezeitpunkt beider Modi korrekt; reale Atomics hinter `cuda`, Logik ohne GPU grün.
  - path: /home/user/xyan/xy.ai.workbench/project/resident/plan/plan_e5.md
    mode: replace
    content: |
      # Umsetzungsplan E5 · Kommunikationsslot (feste Reihenfolge)

      > **Quellen:** `resident_index.md` (Kommunikationsslot). **Recherche:** `context_e.md` §E5 (Punkte 18–20). **Querschnitt:** Q1, Q2, Q3.
      > **Abhängt von:** E4 (Mutationsdisziplin), B4 (Index), B5 (Kernel/`KernelHit`). **Ergebnis:** Slot-Protokoll-Spezifikation.

      ## 1. Ziel & Abgrenzung
      Der **Kommunikationsslot** zwischen den Batches mit **fester Reihenfolge**:
      1. **Kernel-Writes lesen** → gemeldete Bereiche den Entries zuordnen → `Index::apply` aufrufen → eigene Stamps eintragen.
      2. **Ergebnisse auflösen** → Adresse → Entry per Mapping.
      3. **Updates einspielen** → Update-Queue anwenden (verändert das Mapping, daher **zuletzt**).

      Weil nur in Lücken mutiert wird, sieht jedes Ergebnis den Mapping-Stand **seines** Batches — **keine** Epoche/Sequenznummer im Kernel-Protokoll. Ergebnisstatus: entfernt → verworfen; Update anstehend → `outdated`. Nach außen `Hit{entry, score, outdated}`.

      Nicht hier: Echo-Vermeidung/Stamp-Abgleich (E6), Indexaufbau-Writes im Detail (E8).

      ## 2. Voraussetzungen
      - E4: Mutation nur in Lücken, Pointer-Swap, Epoch-Reclaim.
      - B5: Kernel meldet geschriebene Bereiche + `KernelHit{offset,score}` am Batch-Ende; `SegmentDesc`/`Layout`.
      - B4: `Index::apply(entry, data) -> Stamp`, `Index::stamp(entry) -> Option<Stamp>`.

      ## 3. Zu bauende Artefakte
      ```rust
      pub struct Hit { pub entry: EntryId, pub score: f32, pub outdated: bool }

      pub struct UpdateQueue { /* ausstehende Mutationen pro Entry */ }

      pub struct AddressMap { /* Adresse(Arena-Offset) -> EntryId */ }
      impl AddressMap {
          fn resolve(&self, off: u64) -> Option<EntryId>; // Fixed: off/stride; Variable: Binärsuche
      }

      pub fn run_slot(
          writes: &[WrittenRange], hits: &[KernelHit],
          index: &dyn Index, map: &mut AddressMap, updates: &mut UpdateQueue,
          tracker: &mut ResidencyTracker,
      ) -> Vec<Hit>;
      ```

      ## 4. Entscheidungen (aufgelöste „Zu klären"-Punkte)
      **Reihenfolge-Invarianz (Mapping-Stand pro Batch)** (`resident_index.md`): Schritt 1 **vor** Schritt 2 garantiert, dass jede vom Kernel gemeldete Adresse auflösbar ist — auch Adressen frisch geschriebener Segmente. Schritt 3 **zuletzt**, weil Updates das Mapping ändern. Der Manager repräsentiert den VRAM → gemeldete Adresse ist aktuell; **keine** Epoche/Sequenz im Protokoll.

      **Doorbell/Mailbox** (context_e.md §E5 Punkt 18, `[W]`): Ring in mapped pinned Memory; Kernel schreibt pro Batch Fortschrittszähler (Release), Host pollt (Acquire) oder wartet auf `cudaEvent`. Host mutiert nur zwischen „Batch-Ende" und „nächster Batch-Start-Quittung". **Nie** auf Kernel-Ende warten.

      **Adresse → Entry** (context_e.md §E5 Punkt 19, `[W]`): Bei **Fixed-Stride** direkt `offset / stride`. Bei **Variable** sortierte Offset-Tabelle (Prefix-Summe der Entry-Größen) + `slice::partition_point`/`binary_search` (O(log n)); alternativ `BTreeMap<u64, EntryId>` mit `range(..=addr).next_back()`.

      **Indexieren während Suche** (context_e.md §E5 Punkt 20, `[W]`): Versionierte Mapping-Tabelle (immutable Snapshot pro Batch, Pointer-Swap an Batch-Grenze aus E4); Writer hängt neue Segmente **append-only** an; Reader sehen den Snapshot des Batch-Starts.

      **Ergebnisstatus beim Auflösen** (`resident_index.md`): ID im Manager vorhanden, im Index entfernt (`stamp(entry)==None`) → **verworfen**. Update für den Entry in der Queue → `outdated=true` (Diagnose, kein Fehler). Sonst normales `Hit`.

      ## 5. Umsetzungsschritte
      1. `AddressMap` mit Fixed- und Variable-Pfad (Prefix-Summen-Tabelle aus Indexstruktur, Build in E8).
      2. `run_slot` in exakt drei Phasen; Phase 1 ruft `Index::apply` und trägt Rückgabe-Stamp in den Tracker (Echo-Vorbereitung E6).
      3. Phase 2: `resolve` + Status (entfernt/outdated/normal) → `Hit`.
      4. Phase 3: `UpdateQueue` anwenden (Mapping ändern), erst nach Phase 2.
      5. Doorbell-Ring + `cudaEvent`-Warten hinter `cuda`; ohne Feature treibt `FakeKernel` die Phasen synchron.

      ## 6. Fehler / Logging / Feature-Flags
      - **Q1:** `StaleMapping` (intern, bei inkonsistentem Snapshot → interner Retry im nächsten Slot); kein nach außen sichtbarer Fehler für `outdated`/verworfen.
      - **Q2:** Events: Anzahl applied Writes, aufgelöste/verworfene/`outdated` Hits, Update-Queue-Tiefe. Span pro Slot-Zyklus.
      - **Q4:** Doorbell/Event hinter `cuda`; Protokolllogik feature-frei.

      ## 7. Tests (Q3)
      - **Reihenfolge:** Ergebnis auf **frisch geschriebene** Adresse löst korrekt auf (Schritt 1 vor 2).
      - **Status:** entfernter Entry wird verworfen; ausstehendes Update markiert `outdated`.
      - Variable-Layout: Binärsuche liefert korrekten Entry an Segmentgrenzen; Fixed: `off/stride`.
      - Update-Last: Mapping-Änderung wirkt erst im nächsten Batch (Phase 3 zuletzt).
      - Indexieren während Suche: Snapshot des Batch-Starts bleibt stabil, während Writer append-only ergänzt.
      - `FakeKernel` liefert programmierte `writes`+`hits`; keine GPU.

      ## 8. Offene Verifikationen
      - Doorbell-Ring-Sichtbarkeit (PCIe posted writes, `__threadfence_system`) auf Zielhardware (H1 Punkt 2/3).

      ## 9. Definition of Done
      - `run_slot` hält die feste Reihenfolge; Adress-Rückabbildung für Fixed/Variable; Ergebnisstatus korrekt; frisch geschriebene Adressen auflösbar; alle Tests grün ohne GPU.
  reason: "Create agent implementation plans E3 (index arena), E4 (manager discipline / high-risk atomics) and E5 (communication slot)."
```
/answer 7e40-6 allow
ReasoningToken: 13
Text:
 Now the final batch: E6 (sync/echo — high risk), E7 (residency), E8 (index build), E9 (addons).
Control Request:
```yaml
id: 7e40-7
toolName: write
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/project/resident/plan/plan_e6.md
    mode: replace
    content: |
      # Umsetzungsplan E6 · Synchronisation & Echo-Vermeidung (Hochrisiko)

      > **Quellen:** `resident_index.md` (Synchronisation und Echo-Vermeidung). **Recherche:** `context_e.md` §E6 (Punkte 21–25, **Hochrisiko für Konsistenzmodell**). **Querschnitt:** Q1, Q2, Q3.
      > **Abhängt von:** B4 (Stamps, `watch<u64>`, `apply`), E5 (Slot, Tracker). **Ergebnis:** Sync-Spezifikation + Testmatrix.

      ## 1. Ziel & Abgrenzung
      Der **zustandsbasierte** Abgleich RAM/Index ↔ VRAM: Signal weckt nur, verglichen werden **Stamps nur residenter Entries**. Enthält: Signal-Quittierung **vor** dem Scan, Stamp-Lesereihenfolge beim Laden, **Echo-Unterdrückung** via `apply`-Rückgabe-Stamp, Zwei-Engines-Konvergenz, ABA-Vermeidung. Umsetzung des Konsistenzmodells aus `plan_e_overview.md` §1.

      Nicht hier: Ladegranularität/Eviction (E7), Indexaufbau (E8).

      ## 2. Voraussetzungen
      - B4: `stamp()`, `stamps()`, `apply() -> Stamp`, `changed() -> watch::Receiver<u64>`; Stamp-Vergleich nur `!=`.
      - E5: `ResidencyTracker` (Mapping Entry ↔ Slot + bekannter Stamp), Slot-Phase 1 trägt `apply`-Stamp ein.

      ## 3. Zu bauende Artefakte
      ```rust
      pub struct ResidencyTracker {
          known: HashMap<EntryId, (Slot, Stamp)>,  // nur residente Entries
      }
      pub struct SyncScanner { rx: watch::Receiver<u64> }
      impl SyncScanner {
          fn scan(&mut self, index: &dyn Index, tr: &mut ResidencyTracker) -> ScanOutcome; // reloads/entfernt
      }
      pub struct Stamps; // globaler monotoner AtomicU64 (Vergabe bei jeder Mutation)
      ```

      ## 4. Entscheidungen (aufgelöste „Zu klären"-Punkte)
      **`watch`-Semantik & Quittierung vor dem Scan** (context_e.md §E6 Punkt 21, `[V]`): `watch` hält nur den **letzten** Wert; mehrere Sends werden zusammengefasst; jeder Receiver verfolgt unabhängig, was er sah. **Vor** dem Scan: `let wm = *rx.borrow_and_update();` (markiert gesehen **und** liest Stand), dann Scan über alles `> wm`. Kommen Sends während des Scans, ist der Wert wieder „unseen" ⇒ nächstes `changed()` kehrt sofort zurück. Kein Verlust, nur ggf. redundanter Scan. **`borrow_and_update()` verwenden**, nicht `borrow()`.

      **Nur residente Entries** (`resident_index.md`): Änderungen an nicht geladenen Entries brauchen keine Behandlung (Laden liest ohnehin aktuellen Stand). Tracker-Zustand skaliert mit dem geladenen Teil, nicht mit der Indexgröße.

      **Stamp-Lesereihenfolge beim Laden** (`resident_index.md`): **Stamp → Daten → (bei mmap) Stamp erneut prüfen**; weicht er ab, wiederholen. Die falsche Reihenfolge (Daten dann Stamp) trägt einen neueren Stamp als bekannt ein, obwohl veraltete Daten geladen wurden — **die einzige unsichere Richtung**. Überzähliger Reload ist harmlos.

      **Echo-Unterdrückung** (`resident_index.md`): Manager trägt den von `apply` **zurückgegebenen** Stamp des eigenen Writes als bekannt ein (Gleichheitsvergleich). Stamp muss aus dem **Rückgabewert** stammen, nicht aus späterem Lesen. Ohne Rückgabewert entsteht höchstens redundanter Upload (sichere Richtung).

      **ABA / Stamps** (context_e.md §E6 Punkt 22, `[W]`): Globaler monotoner `AtomicU64` (`fetch_add`) vergibt Stamps bei jeder Mutation; Identität = (ID, Stamp); Vergleich nur `!=`. 64 Bit läuft praktisch nie über (Generation-Indices/Tagged Pointers, analog `slotmap`). Entfernen+Wiederanlegen derselben ID ergibt anderen Stamp.

      **Seqlock / versioned read bei mmap** (context_e.md §E6 Punkt 23, `[V]` Prinzip): Reads nicht-atomarer mmap-Daten während Fremd-Schreibens sind in Rust formal UB (Data Race) → mmap-Bereich auf **Wortebene atomar** lesen (`AtomicU64`-Wörter oder `ptr::read_volatile` + Acquire-Fence vor dem zweiten Stamp-Read). Writer: Zähler ungerade → schreiben → gerade (+2). Reader wiederholt bei ungerade/Ungleichheit.

      **Optionale Optimierungen (später, kein Vertrag)** (context_e.md §E6 Punkte 24–25, `[W]`): (a) **Baum mit max-Stamp pro Teilbaum** — Konsument besucht nur Teilbäume mit `max_stamp > watermark_c` (sub-linear); Watermark **pro Konsument**; gemeinsame „dirty"-Bitmap verliert Änderungen für den zweiten Konsumenten → **pro-Konsument-Bitmaps oder Stamps**. (b) **`changed_since(watermark)`** — bounded Ring (Stamp,EntryId); liegt `wm ≥ ältester Ringstamp` → Delta aus Ring, sonst Vollscan-Fallback. Analog Log-Offsets pro Consumer-Group (Kafka), LSN.

      ## 5. Umsetzungsschritte
      1. Globaler `Stamps`-Zähler (`AtomicU64::fetch_add`), Identität (ID, Stamp), `!=`-Vergleich überall.
      2. `SyncScanner::scan`: `borrow_and_update` vor Scan; `stamps()` iterieren (ohne Vektorseiten), nur residente vergleichen; Divergenz → Reload planen (an E7 delegiert), `None` → Entry entfernen.
      3. Ladepfad: Stamp→Daten→(mmap)Stamp-Recheck; Seqlock-Wortlesung für mmap.
      4. Echo: Slot-Phase 1 trägt `apply`-Rückgabe-Stamp in Tracker ein; Scanner erkennt ihn als bekannt (kein Reload).
      5. Mehrere Subscriber (GPU-Manager, Persistenz-Timer, CPU-Manager) halten je eigenen Zustand/Receiver.
      6. Optionale `changed_since`/Baum-Varianten als Feature-Erweiterung vorsehen (nicht Pflicht).

      ## 6. Fehler / Logging / Feature-Flags
      - **Q1:** keine neuen öffentlichen Fehler; `StaleMapping` intern bei Recheck-Fehlschlag → Wiederholung.
      - **Q2:** Events: Scan-Ergebnis (geladen/veraltet/entfernt je Zähler), Echo-Treffer (eigener Write erkannt), Recheck-Wiederholung.
      - **Q4:** Logik komplett feature-frei (kern GPU-unabhängig); nur mmap-Seqlock kann realen Speicher brauchen — im Test über `FakeIndex` mit steuerbaren Stamps.

      ## 7. Tests (Q3 — Testmatrix, aus `resident_index.md`)
      - **Echo:** eigener `apply` löst **keinen** Reload aus; fremder Stamp löst Reload aus; zwei Manager auf demselben Entry **konvergieren**.
      - **Verlorenes/zusammengefasstes Signal:** nächster Abgleich findet **alle** Änderungen.
      - **Signal-Quittierung vor dem Scan:** Änderung während des Scans geht **nicht** verloren (`borrow_and_update` vor Scan; Failpoint injiziert Send während Scan).
      - **Stamp-Lesereihenfolge:** Änderung zwischen Stamp- und Datenlesen führt zu Wiederholung (Failpoint zwischen den Reads).
      - **ABA:** Entfernen+Wiederanlegen derselben ID ergibt anderen Stamp; Vergleich `!=` erkennt es.
      - `proptest` auf Scan-Sequenzen; `loom`/`shuttle` auf Echo-/Recheck-Interleavings; feste Seeds.

      ## 8. Offene Verifikationen
      - `tokio::sync::watch` `borrow_and_update`-Verhalten gegen aktuelle Doku (context_e.md Punkt 21 verifiziert `[V]`).
      - Seqlock-Korrektheit im Rust-/mmap-Kontext (Boehm MSPC'12, `atomic_load_per_byte_memcpy`) — Umsetzung mit Wort-Atomics abnicken; Crate `seqlock` Qualität `[?]`.

      ## 9. Definition of Done
      - Zustandsbasierter Scanner mit Quittierung-vor-Scan, korrekter Lade-Lesereihenfolge, Echo-Unterdrückung über `apply`-Rückgabe; globale Stamps `!=`; vollständige Testmatrix oben grün ohne GPU.
  - path: /home/user/xyan/xy.ai.workbench/project/resident/plan/plan_e7.md
    mode: replace
    content: |
      # Umsetzungsplan E7 · Residency & Ladestrategie

      > **Quellen:** `resident_index.md` (Residency und Ladestrategie); **überlappt Kalibrierung** (`resident_precision.md`, Verweis F). **Recherche:** `context_e.md` §E7 (Punkte 26–30). **Querschnitt:** Q1, Q2, Q3.
      > **Abhängt von:** E3 (Arena/Staging), E5 (Slot). **Ergebnis:** Lade-/Eviction-Strategie.

      ## 1. Ziel & Abgrenzung
      Die Manager-Implementierung von **Residency**: lazy/scope-getrieben laden, Ladegranularität (ganze Segmente vs. Gather vs. Zero-Copy), Tiering (Summaries langlebig), Aggregation über Scope-Union, Single-Flight, Eviction (VRAM strenger als RAM). Die **Dichteschwelle** Gather↔Segment ist Manager-Implementierung und kann als Kalibrierungsachse (F) exportiert werden.

      Nicht hier: die Scope-Auflösung selbst (E2), der Stamp-Abgleich (E6).

      ## 2. Voraussetzungen
      - E3: `IndexArena`, Pinned-Staging-Pool, Copy-Stream, CoW-Budget.
      - E5: Slot-Reihenfolge (Laden passiert als Update/Residency-Änderung zwischen Batches).
      - E2: aufgelöste `EntrySet` (roaring) als Ladeeingabe.

      ## 3. Zu bauende Artefakte
      ```rust
      pub enum LoadPlan { Segments(Vec<SegmentId>), Gather(EntrySet /* in Staging */), ZeroCopy(EntrySet) }
      pub struct LoadPolicy { density_threshold: f32 /* Kalibrierungsachse */, segment_bytes: u64 }
      pub struct SingleFlight<K, T> { inflight: Mutex<HashMap<K, Shared<BoxFuture<Result<Arc<T>, Arc<ManagerError>>>>>> }
      pub struct Evictor { /* LRU + TTL + Hysterese, cost-aware */ }
      ```

      ## 4. Entscheidungen (aufgelöste „Zu klären"-Punkte)
      **Gather vs. Segment / Dichteschwelle** (context_e.md §E7 Punkte 26–27, `[W/?]`): Keine allgemeingültige Schwelle. Faustregel: **Segmentladen gewinnt ab hoher Dichte** (grob > ~10–25 % der Segmentbytes `[?]`), weil PCIe pro Transfer Fixkosten (~10 µs) hat und Kontiguität die Bandbreite ausreizt. Kostenmodell `Kosten ≈ Latenz·n_Transfers + Bytes/BW`, **auf der Zielkarte kalibrieren** → `density_threshold` als **Achse in `resident_precision.md`** exportieren. **Gather:** CPU gathert in **pinned Puffer**, dann **ein** `cudaMemcpyAsync` (besser als hunderte kleine Copies). **Zero-Copy** (`cudaHostAllocMapped` + `cudaHostGetDevicePointer`, UVA 64-Bit-Linux) nur für **sehr kleine**, selten gelesene Mengen (Latenz ~µs, Bandbreite < VRAM um Faktor ~20–30 `[?]`).

      **Tiering** (`resident_index.md`): Summary-Strukturen (Zentroide/Präfixtabellen) klein, zuerst und am längsten gehalten; Vektorsegmente warm/evictable; alles andere nicht geladen. Segmentgröße bestimmt der Manager nach VRAM-Budget (keine Index-Eigenschaft).

      **Aggregation** (`resident_index.md`): Tasks gegen denselben Index über die **Vereinigung ihrer Scopes** (roaring-Union aus E2) bündeln und **einmal** laden.

      **Single-Flight** (context_e.md §E7 Punkt 28, `[W]`): Gleichzeitige Ladeanforderungen auf denselben Bereich = **ein** Ladevorgang mit mehreren Wartenden. Umsetzung: `tokio::sync::OnceCell::get_or_try_init` (bei Fehler darf nächster erneut versuchen) oder `futures::future::Shared` (Fehlertyp `Clone`, verteilt Fehler an alle). Muster: `Mutex<HashMap<Key, Shared<BoxFuture<Result<Arc<T>, Arc<E>>>>>>`, Eintrag nach Abschluss entfernen. **Fehler geht an alle Wartenden** (kein stiller Retry-Sturm).

      **Eviction** (context_e.md §E7 Punkt 29, `[W]`): Teure Reloads ⇒ **LRU mit Pinning laufender Batches**, **TTL + Hysterese** (High/Low-Watermark gegen Thrashing), Summary-/Zentroid-Strukturen priorisiert halten, **cost-aware** (GDSF: Wert = Hitwahrscheinlichkeit × Reloadkosten/Größe). Unterschied zu RAM: kein transparentes Paging, Eviction **explizit**, Reload über PCIe, CoW-Spitzen. Idle-Zeiten für VRAM **deutlich länger** als für RAM.

      **Überlappende Entries** (`resident_index.md`): Manager entscheidet zwischen einmaligem Halten mit Mehrfach-Referenz (spart VRAM, komplex) und Duplizieren (einfach, teuer) — Design-Entscheidung, kein Vertrag. Startwahl: **Duplizieren** (einfach), Referenz als Optimierung später.

      ## 5. Umsetzungsschritte
      1. `LoadPolicy` mit `density_threshold` (Default aus Faustregel, überschreibbar durch Kalibrierung F).
      2. `plan_load(EntrySet) -> LoadPlan`: Dichte aus EntrySet vs. Segmentbytes → Segments/Gather/ZeroCopy.
      3. Aggregation: offene Ladeanforderungen gegen denselben Index per Scope-Union zusammenziehen.
      4. `SingleFlight` um den Ladevorgang; Fehlerverteilung an alle Wartenden.
      5. Gather in pinned Puffer + ein `cudaMemcpyAsync`; Segmentladen direkt; Zero-Copy für Mini-Mengen.
      6. `Evictor`: LRU+TTL+Hysterese, cost-aware, Summaries pinnen, laufende Batches pinnen; Trigger bei VRAM-Druck (`cuMemGetInfo`).

      ## 6. Fehler / Logging / Feature-Flags
      - **Q1:** `TransferFailed` (Copy), `ArenaExhausted` (kein Platz trotz Eviction). Single-Flight verteilt Fehler an alle Wartenden.
      - **Q2:** Events: Lade-Entscheidung (Segments/Gather/ZeroCopy, Dichte), Single-Flight-Coalescing (Anzahl Wartende), Eviction-Entscheidung (Opfer, Grund), Arena-Auslastung.
      - **Q4:** Alle CUDA-Copies hinter `cuda`; Entscheidungslogik (Plan/Aggregation/SingleFlight/Eviction) feature-frei testbar über Host-Arena-Mock + virtuelle Zeit.

      ## 7. Tests (Q3)
      - **Gather-vs-Segment** über Dichteschwelle: niedrige Dichte → Gather, hohe → Segments, winzig → ZeroCopy.
      - **Single-Flight:** ein Ladevorgang bei mehreren gleichzeitigen Anforderungen; **Fehlerverteilung an alle** Wartenden; Eintrag danach entfernt.
      - Aggregation: zwei Tasks mit überlappenden Scopes → ein Ladevorgang über die Union.
      - Eviction: TTL + Hysterese kein Thrashing (virtuelle Zeit); Summaries werden nicht evictet; laufender Batch gepinnt.

      ## 8. Offene Verifikationen
      - Gather-vs-Segment-Schwelle **empirisch auf GTX 1660 messen** (context_e.md offener Punkt 4) und als Kalibrierungsachse an F übergeben.
      - `OnceCell::get_or_try_init` vs. `Shared`-Verhalten bei Fehlern gegen aktuelle Crate-Version prüfen.

      ## 9. Definition of Done
      - `plan_load`, Aggregation, Single-Flight, Evictor implementiert; Dichteschwelle als Kalibrierungsachse exportiert; alle Tests grün über Host-Mock ohne GPU.
  - path: /home/user/xyan/xy.ai.workbench/project/resident/plan/plan_e8.md
    mode: replace
    content: |
      # Umsetzungsplan E8 · Kernel schreibt Segmente (Indexaufbau)

      > **Quellen:** `resident_index.md` (Kernel schreibt Segmente, Rückabbildung). **Recherche:** `context_e.md` §E8 (Punkte 31–33). **Querschnitt:** Q1, Q2, Q3.
      > **Abhängt von:** E5 (Slot-Reihenfolge), B5 (Kernel-Trait, `SegmentDesc`/`Layout`). **Ergebnis:** Indexaufbau-Pfad-Spezifikation.

      ## 1. Ziel & Abgrenzung
      Der Pfad, auf dem der **Kernel neue Segmente schreibt**: Der Manager vergibt **vorab** einen beschreibbaren Bereich aus der Arena; der Kernel schreibt per **Cursor** und **allokiert nie**; bei Bereichsmangel **Teilergebnis**; Zuordnung im Slot (Schritt 1 aus E5); **Rückabbildung** Adresse→Entry (Fixed-Arithmetik / Variable-Binärsuche). Getrennte Ringe: **Embedding** liefert neue Segmente, **MaxSim** liefert Hits.

      Nicht hier: der persistente Ringbuffer-Modus selbst (H1, nur Hook), die Slot-Reihenfolge (E5).

      ## 2. Voraussetzungen
      - E5: feste Slot-Reihenfolge (Schritt 1 „Kernel-Writes lesen" vor Schritt 2 „Ergebnisse auflösen").
      - E3: `reserve_writable(cap)`-Bereich, Pinned Staging, Copy-Stream.
      - B5: Kernel-Trait meldet geschriebene Bereiche + Fortschrittszähler am Batch-Ende.

      ## 3. Zu bauende Artefakte
      ```rust
      pub struct WriteRegion { base: u64, cap: u64 }        // vorab vergeben
      pub struct WriteCursor; // GPU-seitig: off = atomicAdd(&cursor, size); off+size>cap => overflow
      pub struct OffsetTable { offsets: Vec<u64> }           // exklusive Prefix-Summe (Variable-Layout)
      impl OffsetTable { fn addr_to_entry(&self, off: u64) -> EntryId; } // partition_point
      pub enum BuildRing { Embedding, MaxSim }
      ```

      ## 4. Entscheidungen (aufgelöste „Zu klären"-Punkte)
      **Output-Arena mit atomarem Cursor** (context_e.md §E8 Punkt 31, `[W]`): Host vergibt Region `(base, cap)`. Kernel: `off = atomicAdd(&cursor, size)`; `if off + size > cap` → **Überlauf-Flag** setzen, **nicht** schreiben (Teilergebnis bleibt gültig, Host vergibt nächsten Bereich und wiederholt). **Pro Warp/Block ein aggregierter `atomicAdd`** statt pro Thread. **Release-Fence/Flag nach dem Schreiben**, bevor der Host liest.

      **Offset-Tabellen / Rückabbildung** (context_e.md §E8 Punkt 32, `[W]`; `resident_index.md`): **Variable** Längen → **exklusive Prefix-Summe** (`Thrust/CUB ExclusiveScan` auf GPU oder CPU) → Offset-Array; Lookup per **Binärsuche** (`partition_point`). **Fixed-Stride** lohnt, wenn Entry-Größen nahezu gleich (geringer Padding-Verlust) — dann `offset / stride`, keine Tabelle. Die Tabelle baut der **Manager** aus der Indexstruktur (E5 `AddressMap`).

      **Getrennte Ringe Embedding vs. MaxSim** (context_e.md §E8 Punkt 33, `[W]`): Zwei unabhängige **SPSC**-Pfade. **Embedding-Ring** liefert neue Segment-Adressen/Mappings, **MaxSim-Ring** liefert Hits. Weil in E5 **Schritt 1 (neue Segment-Adressen in den Snapshot übernehmen) vor Schritt 2 (Hit-Adressen gegen diesen Snapshot auflösen)** liegt, ist jede Hit-Adresse auflösbar — **auch während zeitgleich indexiert wird**.

      **Schreiben in D7-Slots:** Manager vergibt Slot vorab; Encoder/Kernel schreibt direkt; **Gültigkeitsbit erst nach dem Event**. Upsert = neuer Slot, dann Bit des alten löschen (kein In-Place während laufender Kernel). Zuordnung `(segment, slot) → EntryRef` macht der Manager auf der CPU.

      ## 5. Umsetzungsschritte
      1. `reserve_writable(cap)` aus E3; `WriteRegion` an den Kernel übergeben.
      2. Cursor-Protokoll (GPU): aggregierter `atomicAdd`, Überlauf-Flag, Release-Fence vor Host-Read (hinter `cuda`; Host-Mock simuliert Cursor + Überlauf).
      3. Teilergebnis-Handling: bei Überlauf nächste Region vergeben und wiederholen; Teilergebnis behält Gültigkeit.
      4. Im Slot (E5, Schritt 1): gemeldete Bereiche den Entries zuordnen, `OffsetTable`/Fixed bauen, `Index::apply` aufrufen.
      5. Getrennte `BuildRing::Embedding`/`MaxSim`-Pfade; Embedding-Writes vor MaxSim-Auflösung sichtbar machen (Reihenfolge-Garantie aus E5).
      6. D7-Slot-Schreiben: Gültigkeitsbit nach Event; Upsert über neuen Slot.
      7. **H1-Hook:** `BuildRing`-Schnittstelle so lassen, dass getrennte SPSC-Ringe (persistenter Modus) ohne API-Bruch andocken.

      ## 6. Fehler / Logging / Feature-Flags
      - **Q1:** `ArenaExhausted` (kein Folgebereich), `TransferFailed`. Überlauf ist **kein** Fehler (Teilergebnis).
      - **Q2:** Events: vergebene Region (base/cap), Überlauf + Folgereservierung, applied Segmente, Ring-Typ.
      - **Q4:** Cursor/atomicAdd/Fence hinter `cuda`; Offset-Tabellen-Bau und Rückabbildung feature-frei; Host-Mock simuliert Writes+Überlauf.

      ## 7. Tests (Q3)
      - Teilergebnis bei Bereichsmangel: Überlauf-Flag, Host vergibt neuen Bereich, erster Teil bleibt gültig.
      - Rückabbildung: Variable-`OffsetTable` Binärsuche korrekt an Grenzen; Fixed `off/stride`.
      - **Indexieren während Suche:** Embedding-Writes (Schritt 1) sind für MaxSim-Auflösung (Schritt 2) desselben Slots sichtbar → jede Hit-Adresse auflösbar.
      - Getrennte Ringe arbeiten unabhängig (SPSC-Fakes).
      - D7-Upsert: neuer Slot gültig, alter Slot-Bit gelöscht; kein In-Place.

      ## 8. Offene Verifikationen
      - Aggregierter `atomicAdd` pro Warp/Block und `__threadfence`-Sichtbarkeit auf GTX 1660.
      - CUB/Thrust `ExclusiveScan`-Verfügbarkeit im Build (sonst CPU-Prefix-Summe).

      ## 9. Definition of Done
      - Cursor-Write mit Teilergebnis, Offset-Tabellen/Fixed-Rückabbildung, getrennte Embedding/MaxSim-Ringe mit Reihenfolge-Garantie, D7-Upsert; H1-Hook offen; alle Tests grün ohne GPU.
  - path: /home/user/xyan/xy.ai.workbench/project/resident/plan/plan_e9.md
    mode: replace
    content: |
      # Umsetzungsplan E9 · Addons (Iteration / Kaskade / Rekursion)

      > **Quellen:** `resident_index.md` (Addons). **Recherche:** `context_e.md` §E9 (Punkte 34–36). **Querschnitt:** Q1, Q3.
      > **Abhängt von:** B1 (Queue/Task), C3 (Planner, erweiterbares Parameterobjekt). **Ergebnis:** Addon-Konzeptnotiz — **keine Kernänderung**.

      ## 1. Ziel & Abgrenzung
      Iteration, Kaskaden und Rekursion sind **keine Kernfunktion**. Der Kern enthält **keine** Iteration/Kaskade/Rekursion/Eltern-Kind-Beziehung und keine Task-Zwischenzustände. Solche Muster entstehen **ausschließlich** durch ein **Addon**, das Tasks über die **normale Queue** einreiht (kopierte/geänderte Parameter). Ein Addon-Task ist **nicht unterscheidbar** von einem Anwendungs-Task.

      **Auftrag dieses Pakets:** Nur den **Erweiterungspunkt sichern** — das **erweiterbare Parameterobjekt** — und nachweisen, dass Kaskaden ohne Kernänderung möglich sind. **Keine** neue Kernlogik.

      ## 2. Voraussetzungen
      - B1: unveränderte Queue + `Task`-Primitive.
      - C3: Planner expandiert Tasks; Parameterobjekt muss erweiterbar sein.
      - E2: Scope eines Folgetasks kann aus Ergebnissen des Vortasks stammen (z. B. Zellen-Mitglieder).

      ## 3. Zu bauende/zu sichernde Artefakte
      ```rust
      // Erweiterbares Parameterobjekt (der einzige Kernbedarf):
      pub struct TaskParams {
          // feste Felder (Scope, precision, ...)
          ext: Extensions, // typsichere, offene Erweiterungen
      }
      // Erweiterungsmechanismus (Entscheidung unten):
      pub struct Extensions(HashMap<TypeId, Box<dyn Any + Send + Sync>>);
      ```
      - **Kein** Addon-Trait im Kern. Addons leben in Engine/Anwendung/Ressource und nutzen nur die Queue.

      ## 4. Entscheidungen (aufgelöste „Zu klären"-Punkte)
      **Typsichere Erweiterungen** (context_e.md §E9 Punkt 34, `[W]`): **`anymap`/`TypeMap`** (`HashMap<TypeId, Box<dyn Any + Send + Sync>>`) — flexibel, typsicher beim Zugriff, ohne Kernänderung erweiterbar. Für Serialisierung: Registry `name → (de)serialize` oder `serde_json::Value` als Parameter-Fallback. Extension-Traits (statisch) nur, wo zur Compile-Zeit bekannt. Builder nur bei festem Schema.

      **Selbst-schedulende Tasks** (context_e.md §E9 Punkt 35, `[W]`): **Continuation-Passing** — ein Task gibt bei Abschluss Folgetasks an dieselbe Queue zurück (`Vec<Task>`); Work-Stealing (rayon/crossbeam-deque) für Lastverteilung. **Keine** Parent-Child-Struktur im Kern → Korrelation über **ID im Payload**. Pull-Filter (Priorität/Affinität) steuern, welche Engine den Folgetask zieht.

      **Kaskaden-Beispiel (nur Illustration)** (context_e.md §E9 Punkt 36, `[V]` teilw.): Zentroid-Task gegen die Summary-Struktur → Folgetask mit den **Mitgliedern der gewählten Zellen als Scope**. Allgemeines Funnel-Muster (PLAID: Candidate Generation → Centroid Pruning → Centroid Interaction → Dekompression + exaktes MaxSim; BM25 → ColBERT-Rerank) ist so als Addon realisierbar — gehört aber in die **Oberschicht** (siehe H3), nicht in den Engine-Kern.

      ## 5. Umsetzungsschritte
      1. **Verifizieren**, dass `TaskParams` ein erweiterbares `Extensions`-Feld besitzt (ggf. mit C3 abstimmen/ergänzen — dies ist die einzige erlaubte „Änderung", und sie liegt in B/C, nicht im E-Kern).
      2. **Referenz-Addon** (nur als Test/Doku, nicht im Kern): Zentroid→Mitglieder-Kaskade, die Folgetasks mit kopiertem/geändertem `Scope` einreiht.
      3. Nachweisen: Addon-erzeugte Tasks durchlaufen Queue/Planner/Subengine **identisch** zu App-Tasks; keine Sonderbehandlung.
      4. Konzeptnotiz schreiben: Was ist Kern (Queue + Parameterobjekt), was ist Addon (alles Iterative/Kaskadierende), wo liegt die Grenze zur Oberschicht.

      ## 6. Fehler / Logging / Feature-Flags
      - **Q1:** keine neuen Fehler.
      - **Q3-Relevanz:** Korrelation über Payload-ID testbar.
      - Kein Feature-Flag nötig (GPU-unabhängig).

      ## 7. Tests (Q3)
      - Erweiterbares Parameterobjekt: fremdes Extension-Feld wird durch Queue/Planner transportiert, ohne Kernänderung.
      - Referenz-Addon: Folgetask mit abgeleitetem Scope wird regulär eingereiht und ist von einem App-Task nicht unterscheidbar.
      - Continuation-Passing: Task gibt `Vec<Task>` zurück; Korrelation über ID bleibt erhalten; keine Parent-Child-Struktur im Kern.

      ## 8. Offene Verifikationen
      - `anymap`-Crate vs. eigenes `TypeMap`; Serialisierbarkeit nur falls Persistenz von Parametern gewünscht (`typetag`/`erased-serde`).
      - Mit C3 abstimmen, dass das Parameterobjekt `#[non_exhaustive]`/offen bleibt.

      ## 9. Definition of Done
      - Erweiterbares Parameterobjekt nachgewiesen; Referenz-Kaskade als Addon über die normale Queue demonstriert; Konzeptnotiz zur Kern/Addon/Oberschicht-Grenze vorhanden; **keine** Iteration/Kaskade/Rekursion im Kern.
  reason: "Create the final agent implementation plans E6 (synchronisation/echo), E7 (residency/loading), E8 (kernel writes segments) and E9 (addons)."
```
/answer 7e40-7 allow
Result Stats: id=e2529dd5-743d-4c2f-a9ef-e5e74323a2a2, total: 145810, in: 129, out: 44887, reason: 22, read: 300159, write: 100794
ReasoningToken: 22
Fertig. Ich habe im Verzeichnis `/home/user/xyan/xy.ai.workbench/project/resident/plan` zehn Umsetzungspläne für Gruppe E erstellt.

## Erstellte Dateien

| Datei | Inhalt |
|---|---|
| **`plan_e_overview.md`** | Gruppenweite Festlegungen: Scope-Grenze, **verbindliches Konsistenzmodell Index-Sync** (bisher „noch festzulegen"), Konsolidierung D3↔E3 (ein Allocator), Voraussetzungen aus B/C/D/Q, Modulstruktur, Abhängigkeitsgraph & Sequenz, gemeinsame Test-Fakes, Definition of Done |
| **`plan_e1.md`** | StreamedIndexManager-Grundgerüst (CPU trivial / GPU-Hülle, mmap-Residency via `madvise`, Blocking-Pool) |
| **`plan_e2.md`** | Scope & Resolver-Registry (Doppelprüfung → `UnsupportedScope`, roaring-`EntrySet`, Zentroid-Resolver) |
| **`plan_e3.md`** | Index-Arena (TLSF/Slab, 256-B-`u64`-Offsets, VRAM-Budget, CoW-Heuristik, Pinned Staging/Copy-Stream) |
| **`plan_e4.md`** | Manager-Disziplin / **Hochrisiko** (CoW + atomarer Root-Swap, Epoch-Reclaim, `ld.acquire.sys`/Release, Publish-Reihenfolge) |
| **`plan_e5.md`** | Kommunikationsslot mit fester Reihenfolge (Writes → Auflösen → Updates, Adresse→Entry, Doorbell) |
| **`plan_e6.md`** | Synchronisation & Echo-Vermeidung / **Hochrisiko** (`watch`/`borrow_and_update`, Stamp-Lesereihenfolge, Seqlock, ABA, volle Testmatrix) |
| **`plan_e7.md`** | Residency/Ladestrategie (Gather↔Segment-Schwelle als Kalibrierungsachse→F, Single-Flight, Eviction) |
| **`plan_e8.md`** | Kernel schreibt Segmente (Cursor + Teilergebnis, Offset-Tabellen, getrennte Embedding/MaxSim-Ringe, H1-Hook) |
| **`plan_e9.md`** | Addons (nur Erweiterungspunkt sichern, keine Kernänderung) |

## Methodik
Jeder Einzelplan folgt demselben Schema: **Ziel/Abgrenzung · Voraussetzungen · zu bauende Rust-Artefakte · aufgelöste „Zu klären"-Punkte · Umsetzungsschritte · Fehler(Q1)/Logging(Q2)/Feature-Flags(Q4) · Tests(Q3) · offene Verifikationen · Definition of Done**. Die offenen Punkte aus `resident_planplan.md` sind mit der Recherche aus `context_e.md` aufgelöst, inkl. Evidenzgrad (`[V]`/`[W]`/`[?]`) und der hardwarespezifischen Verifikationen (sm_75-Atomics, Primary-Context-Größe, Gather/Segment-Schwelle auf der GTX 1660).

Zwei im Metaplan als höchste Unschärfe markierte Punkte habe ich explizit geschlossen: das **Konsistenzmodell (D7/E)** im Overview §1 und die **Arena-Dopplung D3↔E3** im Overview §2. `resident_index.md` habe ich wie angewiesen als Vektorindex-Spezifikation behandelt (keine Inhaltsdatei) und auf die Namensinkonsistenz `resident_precision(_calibration).md` hingewiesen.

Das Verzeichnis `plan/` war zuvor leer – es gab keine bestehende Namenskonvention, an die ich mich anpassen musste.
