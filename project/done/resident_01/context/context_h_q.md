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