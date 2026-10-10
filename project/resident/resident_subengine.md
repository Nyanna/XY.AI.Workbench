# Resident – Ressourcenkonzept der Subengines CPU / GPU

Stand: Konzeptentwurf, abgeleitet aus der Spec `resident_streaming.md` und der Designdiskussion.
Alle Zahlen sind Parameter, keine Designannahmen. Mit **[prüfen]** markierte Punkte sind unverifiziert.

## 1. Geltungsbereich

- Beschreibt, welche **Ressourcen** eine Subengine (CPU-Engine, GPU/CUDA-Engine) besitzt, wie sie **initialisiert, geleast und entladen** werden und wie sie zusammenhängen.
- Nicht Teil: Fassade, Task-Typen, Queue, Batch-Former (eigenes Dokument), Dateihandling und Indexpersistierung (außerhalb der Engine), vorgeschaltete Chunk-Textnormalisierung.
- Task ≠ Op: Task = Nutzeranfrage (Host-Typen, Daten + Parameter). Op = Schritt auf einem Device (Regions + Layouts). Ein Planner expandiert Task → Pipeline aus Ops.

## 2. Invarianten (gelten ab Phase 1)

- Kein `Tensor` und kein Raw-Pointer in öffentlichen Signaturen, nur `Region` + `Layout`.
- Jede GPU-Arbeit läuft über einen `ExecContext`, nie auf dem Default-Stream.
- Jede Berechnung ist ein `Op` mit expliziten Ein-/Ausgabe-Regions.
- Kernels werden nur über die `KernelRegistry` geladen (Candles eigene sind dort nur eine weitere Implementierung).
- Ressourcen werden nur über **Leases** benutzt, nie über rohe Referenzen.
- Der Scheduler kennt weder Candle noch CUDA Graphs.
- Alles, was Speicher, Streams oder Kernels besitzt, gehört der Engine. Candle ist ein Adapter an genau einer Stelle (Schicht 4).

## 3. Gemeinsame Ressourcen (CPU-seitig, von beiden Subengines genutzt)

- **ModelSpec** (aus Config): Architektur-ID, ColBERT-Config, Pfade. Beschreibt eine Pipeline, besitzt nichts.
  - Registry: `"bert" → BertEncoder`, `"maxsim" → MaxSimScorer`. Neue Modelle = Registry-Eintrag.
  - ColBERT-Parameter kommen aus der Checkpoint-Config, nicht hartkodiert: `query_maxlen`, `doc_maxlen`, Marker-Tokens, `dim`, Skiplist.
- **Text-Normalizer** (Tokenizer-zugehörig): Unicode, Kleinschreibung usw., immer CPU, über Parameter konfiguriert.
- **Tokenizer** (`tokenizers`-Crate, CPU-only): modellspezifisch oder geteilt.
  - Query-Modus: Marker + `[MASK]`-Padding auf feste Länge.
  - Dokument-Modus: Marker, Kürzen auf `doc_maxlen`, Padding-Tokens verwerfen.
  - Ausgabe: `u32`-IDs. Mehr braucht die GPU nicht (kein Gather auf CPU, kein Adress-Mapping).
  - GPU-Tokenizer: nicht im Stack, Ausblick.
- **Weights-Quelle**: `safetensors`-Crate, mmap, Zero-Copy auf Rohbytes. Wird pro Engine in deren Arena kopiert.
- **CPU-Threadpool für Vorverarbeitung**: Aus globalen Pool

## 4. Ressourcen pro Subengine

### 4.1 Device & Ausführung
- **DeviceHandle**: Device-Ordinal bzw. CPU. GPU: ein Prozess, ein (Primary-)Context **[prüfen]**, mehrere Streams.
- **ExecContext / Lane**: Stream + Priorität + Events + eigener ParamBlock + Scratch-Region.
  - Lanes z. B.: Query (hohe Priorität, kleine Batches), Index/Upsert (Durchsatz), Copy (H2D/D2H).
  - Übergaben zwischen Lanes nur über Events.
  - CPU-Engine: Lane = Worker-Gruppe im eigenen Pool, gleiche Schnittstelle.
- **CUDA-Init-Kosten**: Erste Context-Erzeugung kann Sekunden dauern. Lazy-Init muss das einplanen.

### 4.2 Speicher
- **MemoryManager**: `alloc(dev, bytes, align) → Region`, `free`, `resolve` (nur intern).
- **Region** = Handle `{arena, offset, len, gen}`, kein Pointer. `gen` entwertet Handles nach Evict/Reload.
- **Layout** = `{shape, strides, dtype}` mit eigenem DType-Enum.
- **Arenen** (getrennt nach Lebensdauer):
  - Weights-Arena: langlebig, nur bei Entladung frei.
  - Index-Segment-Arena: gestreamt, evictable.
  - Workspace/Scratch pro Lane: kurzlebig.
  - Staging: Pinned Host Memory für H2D/D2H (GPU), bei CPU entfällt es.
  - ParamBlock pro Lane: kleine feste Region für dynamische Kernel-Parameter.
- **Budget**: VRAM zur Laufzeit per `cuMemGetInfo` abfragen, nicht feste 6 GB annehmen (Context-Overhead, andere Prozesse, Compositor).
- **Stream-geordnete Wiederverwendung**: Region wird erst frei/überschrieben, wenn alle Streams, die sie nutzen, ihr Event erreicht haben. Gehört in den MemoryManager, nicht in die Ops.
- **dtype pro Device**: GPU f16, CPU f32. Index-dtype pro Index fest. Gemischte CPU/GPU-Indexierung nur mit definierter Toleranz.

### 4.3 Modell
- **LoadedEncoder** = Architektur (Code) + Gewichte (Daten) auf einem Device. Ein Objekt, entladbar.
  - Phase 1: Candle-Backbone (`bert`, `modernbert`, ...) über den Adapter.
  - Es gibt kein ColBERT-Modul in `candle-transformers`; Backbone + eigener Kopf.
- **ProjectionHead**: `Linear` (z. B. 768→128), Gewichte aus dem Checkpoint (Dense-Modul, Ort je Checkpoint **[prüfen]**).
- **L2-Normierung**: eigener Kernel am Ende des Encoders (einmal beim Schreiben, nicht im Scorer). Query und Dokument gleich behandelt.
- **Ausgabe**: `encode_into(tokens: Region, out: Region)`. Phase 1: Candle rechnet, ein dünner Kernel (`WriteSlotOp`) liest den Device-Pointer des Candle-Tensors und schreibt normiert + dtype-konvertiert in den Slot. Tensor bleibt bis zum Event am Ende des Schreibens am Leben.
- **Batching**: Token-Budget statt Task-Anzahl, Längen-Bucketing, Padding auf gemeinsame `seq_len`.

### 4.4 Index-Segmente
- Überlappt mit `resident_index.md`
- **SegmentView**: Basisadresse (als Region), Slot-Zahl, Stride, dtype, Generation.
- **Slot** = feste Größe `T_max × dim × dtype`. Slot-Klassen (z. B. 32/64/128 Tokens) pro Segment bei variablen Längen, damit der Kernel lookup-frei bleibt.
- **Pro Slot**: gültige Token-Zahl (Padding wird mit `-inf` maskiert, nicht mit 0) und Gültigkeitsbit (Tombstones, noch nicht befüllt).
- **Optionales Feld pro Token**: Centroid-ID/Code (für späteren Funnel, vom Kernel ignoriert, wenn nicht genutzt).
- **Schreiben**: Manager vergibt den Slot vorab, Encoder schreibt direkt dorthin, Gültigkeitsbit erst nach dem Event. Upsert = neuer Slot, dann Bit des alten löschen. Kein In-Place während laufender Kernels.
- **Kernel kennt nur Slots**: Ergebnis `(segment_gen, slot, score)`. Zuordnung `(segment, slot) → EntryRef` (Chunk/Zeile/Bereich/Datei) macht der Manager auf der CPU. Treffer aus inzwischen entladenen Segmenten werden verworfen oder remapped.
- **Segment-Basisadressen** gehen als kleine Tabelle über Segmente (nicht über Einträge) an den Kernel.
- **Bidirektionale Sync** (Index-Objekt ↔ RAM/VRAM): Änderungen laufen über Segment-Ereignisse („Segment hinzugefügt/geändert“), die Subengine hängt neue Segmente per Async-Copy an. Konsistenzmodell (Source of Truth, Versionierung) noch festzulegen.

### 4.5 Kernels & Scorer
- **KernelRegistry**: `(name, dtype, sm_arch) → geladene Funktion`. Quellen: NVRTC-Laufzeitkompilierung mit `--gpu-architecture=sm_XY`, CUBIN-Fallback bei zu altem Treiber, Candle-Ops als weitere Implementierung.
- **Konstanten vs. Parameter**:
  - Compile-Time (`-D`, Varianten als eigene Funktionen): dtype, `dim`, Tile-Größe.
  - Dynamisch über ParamBlock (feste Region): Slot-Indizes, Zähler, Segmenttabelle, Query-Länge, Head/Tail.
- **MaxSimOp**: `SliceRange{segment, slot_from, slot_to}` + Query-Batch → Scores je Slot.
  - Phase 1: Kachel-Kette in Candle-Ops (matmul → Maske → max, laufendes Maximum über Token-Kacheln, Summe über Query-Tokens). Zwischenmatrix durch Kachelgröße begrenzt.
  - Später: fused Kernel (NVRTC), Zwischenmatrix nie im VRAM. Austausch hinter demselben Op.
  - CPU: Schleife pro Slot mit laufendem Maximum je Query-Token, SIMD, keine Matrix über Slots.
  - Padding: Dokumentseite maskiert, Query-`[MASK]`-Tokens zählen mit.
- **Top-k**: pro Slice lokal reduzieren, Merge auf der CPU.
- **Work-Slice**: Scheduling-Einheit mit Zeitbudget (Parameter). Größe adaptiv aus gemessenem Durchsatz. Preemption nur an Slice-Grenzen. Query-Tasks können Indexing-Slices überholen.
- **Query-Batching**: Ein Tile wird gegen alle wartenden Queries gescort, um Lesekosten zu amortisieren.
- **Funnel (optional, später)**: Stufe 1 = billiger Filter auf Centroid-Codes, Stufe 2 = exaktes MaxSim nur auf der Spitze. Referenz: PLAID / next-plaid. Anfangs Brute-Force über Slices.

### 4.6 Ausführungs-Optimierungen (optional, abschaltbar)
- **Graph-Decorator** auf dem Launcher: Modus `Direct` oder `Capture`. Cache-Schlüssel `(Op, Shape-Bucket, Regions)`. Hält Handles, keine Besitzrechte. Bei `gen`-Änderung verwerfen. Ein-/Ausgaben liegen in festen Ring-Slots des Managers. Candle-Capture auf eigenem Stream vorher testen.
- **Persistent-Kernel / Ringbuffer** (Ausblick): Zustand (Head, Tail, Stop-Flag, Zähler) in eigener Region, Kernel läuft zeitscheibenweise und wird vom Host neu gestartet. Grid ≤ SMs × Occupancy (alle Blöcke gleichzeitig resident). Stop-Flag nur am Ende der Zeitscheibe prüfen. Sinnvoll für den Scorer, nicht für den Encoder.

## 5. Abhängigkeiten

```
ModelSpec ─┬─ Tokenizer/Normalizer (CPU, geteilt)
           ├─ LoadedEncoder ── Weights (Region, Weights-Arena)
           │      └─ ProjectionHead + L2-Norm
           └─ Scorer ── KernelRegistry ── geladenes Modul
IndexSegments (Region, Index-Arena) ── ParamBlock ── Lane/ExecContext
Lane ── Stream, Events, Scratch, ParamBlock
alle ── MemoryManager ── Budget (cuMemGetInfo)
```

## 6. Lebenszyklus

- **Zustände**: `Unloaded → Loading → Resident → Leased(n) → Idle(seit t) → Evicting → Unloaded`, zusätzlich `Failed`.
- **Lazy-Init**: erst bei erster Verwendung, `get_or_try_init` (bei Fehler erneut versuchbar), ergänzt um Limit für parallele Initialisierungen (Semaphore).
- **Lease**: Zählt aktive Nutzer. Entladen erst bei 0 und nach Ablauf der Idle-Zeit (Hysterese).
- **Entladezeiten**: VRAM-Ressourcen (Modell, Index, Kernels) deutlich länger als RAM/CPU.
- **Aggregation**: Beim Laden einer Ressource werden wartende Tasks auf demselben Modell/Index zusammengezogen.
- **Fehler**: Ressource → `Failed`, strukturierte Fehler (`thiserror`), Logging mit `tracing` (Span pro Task-ID und Ressource). GPU-Initfehler → Fallback auf CPU-Engine, GPU als nicht verfügbar markiert (wiederholbar).
- **Entladen eines laufenden Kernels**: nur über Stop-Flag/Slice-Grenze, nie hart.

## 7. Zusammenspiel mit dem Scheduler (Pull)

- Subengine meldet freie Kapazität und holt per `claim(filter, budget)` fertige Batches.
- Zuteilung selbstorganisierend: gleitender Mittelwert des Durchsatzes, schnellere Engine holt mehr.
- Priorisierung und Filter: Tasktyp, Index-Affinität, Prioritätsklasse. GPU kann auf Indexierung priorisiert sein, Starvation-Schutz und Query-Überholen bleiben.
- Batch-Größe: Token-Budget, Längen-Bucket, Deadline pro Klasse. Obergrenze der Batchdauer begrenzt die Query-Latenz.
- Ergebnis: Oneshot pro Task plus Event/Notification. Batch merkt sich Zeilen-Offsets pro Task.

## 8. Unterschiede CPU- vs. GPU-Subengine

- **CPU**: mehr Speicher, alles nutzt CPU. Scorer selbst implementiert (SIMD), Encoder über Candle-CPU-Backend. Index per mmap lesbar. Kein Staging, keine Streams.
- **GPU**: begrenzter VRAM, Streams, Staging, Transfers. Tokenizer/Normalizer belegen CPU-Ressourcen (Bottleneck beachten). Batching ist notwendig, um Index-Segmente und Kernels effizient zu streamen und zu swappen.
- **Gemeinsam**: dieselben Traits (`Op`, `Region`, `ExecContext`), dieselbe Lease-Logik, dieselben Task-Pipelines.

## 9. Ausbaupfad ohne API-Bruch

1. Region/Op/ExecContext als dünne Hüllen um Candle. MaxSim als Kachel-Kette. Engine-Gerüst, Lazy-Init, Leases.
2. Eigene Streams, eigener MaxSim-Kernel (NVRTC), Slice-Scheduling.
3. Eigene Arena für Gewichte und Workspace, `encode_into` ohne Copy.
4. Graph-Decorator, Persistent-Scorer mit Ring.
5. Optional: eigener Encoder-Forward (cuBLAS + wenige Kernels) hinter demselben `Op`.

## 10. Vor dem Entwurf prüfen (Prototyp-Tests)

- [ ] `cudarc`-Version in Candle = Version im Projekt (sonst inkompatible Typen).
- [ ] Candle-`CudaDevice` mit eigenem Stream, Events auf Candles Stream aufzeichnen/abwarten.
- [ ] Primary Context: Device-Pointer eines Candle-Tensors in eigenem Kernel lesbar.
- [ ] Fremde Allokation als `CudaStorage` einbinden, ohne dass Candle sie freigibt.
- [ ] CUDA-Graph-Capture über Candle-Ops auf eigenem Stream.
- [ ] ColBERT-Checkpoint: Ort/Format von Projektionskopf und Config, safetensors vorhanden oder Konvertierung nötig.
- [ ] MaxSim-Kachel-Kette vs. fused Kernel messen (Bandbreite, Latenz) mit realen Workloads.
- [ ] Konsistenzmodell der Index-Synchronisation festlegen (Segment-Generationen, Epochen).

## 11. Offene Entscheidungen

- Reranker/Cross-Encoder als eigener Tasktyp (Inferenz), eigene Gewichte, Forward pro Paar.
- Funnel: wann einführen, Centroid-Training und -Verwaltung.
- Persistenz-Verantwortung: Index-Objekt besitzt Persistenz, Engine kennt nur ein `Index`-Trait mit Änderungsereignissen.