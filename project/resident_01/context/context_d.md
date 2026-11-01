## Gruppe D: Subengine-Ressourcen, Kontextdatei

**Hinweis zur Methode:** Entsprechend deiner Vorgabe habe ich keine Tools genutzt, also keine Live-Recherche und keine Link-Prüfung. Alles stammt aus Modellwissen. **\[V]** heißt aus Doku/Praxis belastbar bekannt, **\[U]** heißt unsicher und vor Umsetzung per `cargo doc` / Quellcode prüfen. Versionsangaben können veraltet sein.

***

### D1 Lifecycle & Pull

1. **Lazy-Init \[V]:** `tokio::sync::OnceCell::get_or_try_init(|| async {...})`. Bei `Err` bleibt die Zelle uninitialisiert und der nächste Aufruf versucht es erneut. Parallele Aufrufer warten, nur einer initialisiert. Zur Begrenzung paralleler Inits über *verschiedene* Zellen dient ein globaler `Arc<Semaphore>`, dessen Permit innerhalb der Init-Closure gehalten wird. Für Backoff nach Fehlern wird ein `Instant` neben der Zelle gespeichert.
2. **Pull + EWMA \[U]:** Es gibt kein Standard-Crate. Das Muster ist ein gemeinsamer Task-Pool, aus dem jede Engine Batches holt. Die Batchgröße ist proportional zum EWMA-Durchsatz (`alpha≈0.2`), mit Min/Max-Klammern. Verwandte Konzepte sind Guided Self-Scheduling, Work-Stealing (`crossbeam-deque`) und Tokio-Scheduler-Design. Trade-off: Bei starken Durchsatzschwankungen oszilliert die Last, daher EWMA glätten und Batch-Obergrenze setzen.
3. **Claim-Filter \[U]:** Echte lock-free Filter sind selten. Üblich sind getrennte Queues je Priorität/Modell (`crossbeam::queue::SegQueue`, Mutex um `BinaryHeap`) und ein Claim, der Queues in Prioritätsreihenfolge scannt. Verfügbarkeitsprädikate wie „Modell nicht gesperrt” werden als Atomic-Flag vor dem Pop geprüft. Der Skip-Scan unter `parking_lot::Mutex` ist bei dieser Größenordnung (< 10⁴ Tasks) praktisch ausreichend.
4. **Eviction \[V/U]:** Leases sind RAII-Guards (`Drop` dekrementiert `AtomicUsize`). Evictor-Bedingung: `in_flight==0 && leases==0 && last_used+TTL<now`. Sie wird unter Lock **erneut geprüft** (Double-Check), und neue Leases sind während der Eviction gesperrt (Zustand `Evicting`). Als Referenz taugt `moka` (TTI/TTL) oder `Arc::strong_count` bzw. `Weak`.

### D2 Device & Ausführung

5. **Eigener Stream \[U]:** candle-core ≥0.9 nutzt cudarc ≥0.16 mit `CudaContext`/`CudaStream`. Ein `CudaDevice::new_with_stream(ordinal)` existiert meines Wissens, prüfen. Events laufen über cudarc: `stream.record_event(flags)` → `CudaEvent`, `event.synchronize()`, `other_stream.wait(&event)`. Candle-Ops laufen auf dem Stream des Devices. Keine Annahme, dass sie auf dem Default-Stream bleiben.
6. **Pointer/Context \[V/U]:** cudarc retained den **Primary Context**, und candle nutzt cudarc, daher teilen sie ihn, sofern dieselbe cudarc-Version vorliegt (siehe 32). Der Pointer kommt aus `CudaStorage` → `CudaSlice<T>` → `DevicePtr::device_ptr(&stream)`. Die Stream-Synchronisation muss manuell per Event erfolgen.
7. **Init-Kosten \[U]:** Erste Context-Erzeugung typisch 0,2–2 s, bei kalter Treiber-/Modul-Initialisierung auch mehr. Empfehlung: Context beim Start warmlaufen lassen (Dummy-Alloc), `CUDA_MODULE_LOADING=LAZY` setzen, Modell-Init separat messen. Eigene Messung nötig.
8. **Lane/Prioritäten \[U]:** cudarc-safe hat meines Wissens keine Prioritäts-API, daher `sys::cuStreamCreateWithPriority` und `cuCtxGetStreamPriorityRange` nutzen. Turing unterstützt Stream-Prioritäten (Bereich z. B. 0…−5). Prioritäten wirken nur auf Block-Scheduling, nicht als Preemption laufender Blöcke. ExecContext = `{stream, events, pinned staging, ParamBlock (Device-Buffer), scratch-Arena}`.

### D3 GPU-Speicher

9./33. **Fremde Allokation in candle \[U, kritisch]:** `CudaStorage::wrap_cuda_slice(CudaSlice<T>, CudaDevice)` existiert. Der Haken: `CudaSlice` ist **owning** und gibt beim Drop frei. Mögliche Wege sind `stream.upgrade_device_ptr` (Slice aus Rohpointer) kombiniert mit `CudaSlice::leak()`, was aber nicht in einen Tensor-Drop-Pfad passt. Ein Sub-Range einer Arena ergibt nur `CudaView` (borrowed), das candle nicht als Storage nimmt. **Empfehlung:** Arena-Speicher nur für eigene Kernel nutzen, candle-Tensoren über candles eigene Allokation (stream-ordered Pool) laufen lassen und Daten per D2D kopieren. Prototyp vor Festlegung.\
10\. **Kein malloc im Betrieb \[V/U]:** cudarc ≥0.12 nutzt `cuMemAllocAsync`/`cuMemFreeAsync`, wenn das Device Memory Pools unterstützt (Turing: ja). Mit `cuMemPoolSetAttribute(RELEASE_THRESHOLD=u64::MAX)` behält der Pool seinen Speicher. Daneben eine Bump-Arena (ein großer Alloc, Offsets) für eigene Scratch-Buffer.\
11\. **Pinned Staging \[U]:** `ctx.alloc_pinned::<T>(n)` → `PinnedHostSlice`, `stream.memcpy_htod` bzw. `memcpy_dtoh` async. Ring von Regionen, jede mit einem Event. Eine Region wird erst nach `event.synchronize()` bzw. Completion-Query wiederverwendet.\
12\. **VRAM \[V/U]:** `cudarc::driver::result::mem_get_info()` → `(free, total)`. Realistisch auf 6 GB: CUDA-Context ca. 250–400 MB, Desktop/Compositor 100–500 MB, nutzbar ca. 5–5,5 GB. Vor Ort messen.\
13\. **dtype \[V/U]:** GPU `DType::F16` (Turing hat kein natives BF16, daher BF16 meiden), CPU f32. Candle-BERT läuft in F16 über `VarBuilder::from_mmaped_safetensors(&[..], DType::F16, &dev)`. Die Attention-Maske muss f16-sicher sein, also −1e4 statt −inf in Softmax-Pfaden. Erwartete Cosinus-Abweichung f16 vs f32: \~1e-3 oder besser.

### D4 Modell-Ressourcen

14. **Namen \[U]:** e5-small speichert BERT-Schlüssel ohne `bert.`-Präfix (`embeddings.*`, `encoder.layer.N.*`). `candle_transformers::models::bert::BertModel::load` hat einen Fallback über `config.model_type`, daher meist kein Mapping nötig. Prüfen: `config.json` (`model_type: "bert"`, `vocab_size≈250037`, `hidden_size 384`, 12 Layer), Pooler-Gewichte (werden nicht benötigt).
15. **safetensors \[V/U]:** Das HF-Repo enthält `model.safetensors` (zusätzlich ONNX/`pytorch_model.bin`). Falls nötig: `transformers` `save_pretrained(safe_serialization=True)` oder Candle `VarBuilder::from_pth`.
16. **ColBERT \[V/U]:** candle-transformers hat kein ColBERT-Textmodul (es gibt `colpali` für Vision). Praxis: Backbone (`bert`/`xlm_roberta`/`jina_bert`) plus eigener `candle_nn::Linear` aus `VarBuilder`, danach L2-Norm pro Token.
17. **Projektionskopf \[U]:** `jina-colbert-v2` hat den Kopf als `linear.weight` (1024→128, ohne Bias) im Hauptcheckpoint. `answerai-colbert-small-v1` hat einen separaten `1_Dense/` (PyLate/ST-Format) mit `linear.weight` (384→96, nicht 128). Tensor-Namen per `safetensors`-Header vorab lesen.
18. **Batching \[V]:** Sortieren nach Länge, Buckets (z. B. 32/64/128/256/512), Batch bis `batch*max_len ≤ Token-Budget`, Padding mit `pad_token_id` (XLM-R: 1), Maske 0/1, Zusammenbau per `Tensor::stack`/`from_vec`. Dasselbe Prinzip nutzt text-embeddings-inference.

### D5 Tokenizer — Widerspruch aufgelöst

19. **GPU-Tokenizer/ID→Adresse \[V-Argumentation]:** Nicht empfehlenswert. Tokenisierung ist String-Verarbeitung und läuft auf der CPU. Der Embedding-Lookup ist ein Gather von `seq×384` Werten, bei `index_select` auf IDs unmessbar klein gegenüber 12 Transformer-Layern. Ein eigener Gather-Kernel mit CPU-aufgelösten VRAM-Adressen spart nichts, bricht die candle-Kompatibilität und braucht Adresspflege bei Modell-Eviction. Präzedenzfälle für GPU-Tokenisierung (RAPIDS/cuDF `nvtext`) betreffen String-Tokenisierung, nicht ID→Adresse. **Entscheidung: `resident_subengine.md` („nur Ausblick”) gewinnt.** Pipeline: CPU-Tokenizer → `u32`-IDs → H2D → `index_select`.
20. **tokenizers \[V]:** `Encoding::get_ids() -> &[u32]`, `get_attention_mask() -> &[u32]`, `get_type_ids()`. Die Normalisierung liegt **im Tokenizer** (`tokenizer.json`: Normalizer → Pre-Tokenizer → Model → Post-Processor), also nicht separat nachbauen. Batch: `encode_batch` plus `with_padding`/`with_truncation`.
21. **Modi \[U]:** e5: `"query: "` bzw. `"passage: "` als Klartext vor der Eingabe, Special Tokens `<s>…</s>` fügt der Tokenizer hinzu. ColBERT: Marker (`[Q]`/`[D]` bzw. Prefix-Tokens je Checkpoint), Query-Padding mit `[MASK]` bis `query_maxlen` (32), Dokument-Kürzung auf `doc_maxlen`. Das ist Anwendungslogik nach `encode` und keine Tokenizer-Funktion, je Checkpoint-Doku prüfen.

### D6 Kernels & Scorer

22. **MaxSim \[U]:** Candle-Kette (`matmul → masked_fill → max → sum`) erzeugt eine Sim-Matrix je Dokument (z. B. 32×128 f16 = 8 KB). Das ist speichergebunden, aber cuBLAS/Tensor-Cores sind effizient. Der Fused-Kernel lohnt, wenn das Profil Sim-Matrix-Traffic oder Launch-Overhead zeigt (viele Kandidaten pro Query). Start mit der Kette, danach messen.
23. **NVRTC \[V/U]:** `cudarc::nvrtc::compile_ptx_with_opts(src, CompileOptions{ arch: Some("sm_75"), .. })`, dann `ctx.load_module(ptx)` und `get_func`. Fallback bei altem Treiber: PTX (`compute_75`) statt CUBIN. `KernelRegistry: HashMap<(&'static str, DType, u32 /*sm*/), CudaFunction>`. Candles eigene Kernel werden zur Build-Zeit mit `CUDA_COMPUTE_CAP=75` gebaut.
24. **CPU-SIMD \[U]:** Schleife über Query-Tokens, je Token ein laufendes Maximum über Doc-Tokens, `std::arch` (AVX2/FMA), `wide`, `pulp` oder Autovektorisierung über `chunks_exact(8)`; `half` für f16. Keine Slot-übergreifende Matrix.
25. **Padding \[V/U]:** Originalcode (ColBERT) maskiert Dokument-Padding vor dem Max mit stark negativem Wert (≈ −9999), Query-`[MASK]`-Tokens zählen mit (Query-Augmentation). In f16 −1e4 verwenden statt −inf (NaN-Risiko bei 0·−inf). PLAID teilt dasselbe Scoring.
26. **Slices \[U]:** Kernel in begrenzte Launches (\~1–5 ms) teilen, Deadline zwischen Launches prüfen, Event je Slice. Preemption nur an Grenzen. Hoch priorisierte Lane gewinnt nur beim Block-Scheduling. Literatur: Clockwork (OSDI’20), PipeSwitch (OSDI’20), REEF (OSDI’22).

### D7 Index-Segmente

27. **Konsistenz \[V-Konzept, Entscheidung offen]:** Empfohlen: **immutable Segmente + Generations-Manifest** (Lucene/Tantivy-Prinzip). Der Host baut ein neues Segment bzw. Manifest, publiziert es per `arc-swap`, Leser pinnen die Generation per Lease. Ein altes Segment wird erst freigegeben, wenn Leases == 0 **und** alle Stream-Events der Kernel abgeschlossen sind. Alternativen: Epoch-Reclamation (`crossbeam-epoch`), Double-Buffering. Trade-off: Copy-on-Write kostet VRAM temporär, daher VRAM-Budget für 2 Generationen einplanen.
28. **Slot-Klassen \[U]:** Adresse = `base + slot*stride(class)`, getrennte Längen-Arrays und Valid-Bitmap, Gültigkeit/Generationsnummer je Slot. Upsert: neuen Slot schreiben → stream-geordnet Valid-Bit setzen → altes Bit löschen (Tombstone), kein In-Place während laufender Kernels. Kompaktierung durch Segment-Rebuild. Vergleichbar: vLLM-PagedAttention-Blöcke.
29. **Bidirektionale Sync \[U]:** Segment-Events über `tokio::sync::mpsc`/`broadcast`, Kopie auf der Copy-Lane mit Pinned-Staging, Publikation erst nach Event-Completion, Version/Generation im Event.

### D8 Ausbaupfad & CPU/GPU

30. **Ausbaupfad \[U]:** Fallstricke: candle-Ops nutzen eigenen Allokator/Stream, sodass Event-Ordnung zwischen Candle- und eigenen Kerneln explizit sein muss. Numerik: cuBLAS-Akkumulation (f16 vs f32) unterscheidet sich von candle-Ops, daher Golden-Tests je Phase. Traits (`Op`, `Region`, `ExecContext`) nicht an `candle_core::Tensor` leaken, sonst bricht die API beim Ersetzen.
31. **CPU vs GPU \[U]:** f32 (CPU) vs f16 (GPU): Cosinus-Differenz typ. 1e-3…1e-4, Ranking-Unterschiede nur bei Beinahe-Gleichstand. CPU-f16 in candle ist langsam, daher CPU f32. Gemischte Indizes nur mit Toleranz (kein exakter Gleichheitstest), idealerweise dtype pro Index festlegen.

### Vorab-Verifikation

32. **Version-Alignment \[V/U]:** Maßgeblich ist die `cudarc`-Version, die candle-core zieht (0.9.x → cudarc 0.16.x, später evtl. 0.17+, Zuordnung prüfen). Im Projekt **dieselbe Version pinnen** oder besser `candle_core::cuda_backend::cudarc` (Re-Export) nutzen. Kontrolle: `cargo tree -i cudarc` und `cargo tree -d`.
33. **Pooling \[V/U]:** Mean-Pooling über `attention_mask`, danach L2 (wie sentence-transformers für e5). Erwartung f32 gegen Referenz: Cosinus ≥ 0,9999 (Differenz \~1e-5…1e-6), f16 ≈ 1e-3. Eigene Referenzwerte aus Python (`sentence-transformers`) erzeugen und als Test-Fixtures ablegen.
34. **Präfix \[V/U]:** Die Model Card verlangt `query: `/`passage: `und warnt vor Qualitätsverlust ohne Präfix. Eine belastbare Zahl dafür kenne ich nicht. Für symmetrische Aufgaben `query: `auf beiden Seiten.

***

**Prioritäts-Zusammenfassung:** Frage 19 ist entschieden (CPU-Tokenizer, `index_select`). Frage 27 hat eine Empfehlung (immutable Segmente + Generationen), die Festlegung bleibt bei dir. Frage 9/33 ist das größte technische Risiko, ein Prototyp mit `wrap_cuda_slice` + Drop-Verhalten ist zwingend. Frage 32 ist per `cargo tree` in Minuten klärbar.
