- **Revisionshierarchie:** `resident_subengine.md`, `resident_processor.md`, `resident_precision.md` sind aktueller. Die **Planner/Processor-Zwischenschicht ersetzt** die direkte Task-Verwendung in Subengines (veraltet). `resident_index.md` und `resident_planner.md` ergänzen dies. `resident.md` ist das Basiskonzept, `resident_first.md` sind Modell-Notizen.
- **Fehlende Eingabe:** Die Grundlagen-These `/home/user/xyan/xy.ai.workbench/docs/rag/resident_streaming.md` liegt **nicht** im eingelesenen Ordner, wird aber referenziert → als erstes Paket zu beschaffen/destillieren.
- **Namensinkonsistenz:** `resident_index.md` verweist auf `resident_precision_calibration.md`; die reale Datei heißt `resident_precision.md`.

---

# Metaplan (Plan-Plan) „Resident"-Engine

**Zweck:** Dies ist Stufe 1 (Metaplanung). Sie identifiziert **alle Komponenten und Aspekte**, gruppiert und ordnet sie zu abgeschlossenen **Planungspaketen (PP)**. Stufe 2 (der nachfolgende Lauf) arbeitet diese Pakete **sequentiell** ab, recherchiert je Paket die offenen Punkte und erzeugt daraus den eigentlichen Plan. Jedes Paket ist bewusst so geschnitten, dass es isoliert bearbeitbar ist (eigene Quellen, Abhängigkeiten, zu klärende Aspekte, Ergebnistyp).

**Paketschema:** `Zweck` · `Quellen` · `Abhängt von` · `Zu klären` · `Ergebnis`

**Legende Ordnung:** Gruppen A→H sind grob topologisch sortiert (Fundament → Verträge → Orchestrierung → Subengine → Index → Präzision → konkrete Integration → Ausblick). Querschnitt Q begleitet durchgehend.

---

## Gruppe A — Fundament & Kontext (zuerst)

**A1 · These destillieren**
- Zweck: `resident_streaming.md` beschaffen, Kernaussagen für Streaming/ColBERT/MaxSim extrahieren, Abgleich mit umgesetztem vs. zurückgestelltem Umfang.
- Quellen: `resident_streaming.md` (fehlt im Ordner), `resident.md` (Ausblick).
- Abhängt von: —
- Zu klären: Welche Paper-Aspekte sind *jetzt* umzusetzen, welche nur Ausblick (Ringbuffer, gleitende Indizierung).
- Ergebnis: Referenzzusammenfassung + Umfangsabgrenzung.

**A2 · Projekt- & Modulstruktur**
- Zweck: Crate-/Modul-Layout im Rust-Projekt `/home/user/xyan/xy.ai.workbench/rag`, Modulgrenzen entlang der Schichten (Fassade/Processor/Planner/Subengine/IndexManager/Index/Kalibrierung), Feature-Flags (v. a. `cuda`).
- Quellen: `resident.md` (Stack), `resident_subengine.md` (Invarianten, Ausbaupfad §9).
- Abhängt von: A1.
- Zu klären: Welche Teile hinter `cuda`-Feature-Flag, damit Logik ohne GPU testbar bleibt.
- Ergebnis: Modul-/Crate-Schnitt, Feature-Matrix.

**A3 · Stack- & Versionskompatibilität verifizieren**
- Zweck: Harte Vorabprüfungen der Abhängigkeiten.
- Quellen: `resident.md` (Stack), `resident_subengine.md` §10, `resident_first.md` (Prüfen).
- Abhängt von: A2.
- Zu klären: `cudarc`-Version in `candle` == Projekt-Version; `candle-core` cuda-Feature; `tokenizers`; `parquet`+mmap; `tokio`; `crossbeam`/`parking_lot`; `model.safetensors` vorhanden oder Konvertierung.
- Ergebnis: Verifizierte Abhängigkeitsliste + Risikoliste.

**A4 · Querschnittskonventionen festlegen**
- Zweck: Einheitliche Fehler- (`thiserror`), Logging- (`tracing`) und Test-/Mock-Konventionen für alle Pakete.
- Quellen: alle Dokumente (jeweils Fehler/Logging-Abschnitte).
- Abhängt von: A2.
- Zu klären: Gemeinsame Fehler-Taxonomie-Basis, Span-Granularität (Task-ID/Ressource/Manager-Zyklus), Trait-Abstraktionen für Mocks.
- Ergebnis: Konventionsdokument (speist Q1–Q4).

---

## Gruppe B — Kern-Datentypen & Verträge (bevor Orchestrierung)

**B1 · Queue & Task-Primitive**
- Zweck: Lock-freie einfach verkettete Liste, `Node`, `Task`, `TaskState`, `claim`(CAS Queued→Taken), Fertigstellung, `cancel`, Notify-Topologie, iterativer Drop, kein `unsafe` außer `arc-swap`.
- Quellen: `resident.md` (Queue-Logik), `resident_processor.md` (Queues).
- Abhängt von: A4.
- Zu klären: Payload-`Mutex<Option<Data>>`-Besitzübergang, Unlink-Semantik, shared-Queue (angelegt, leer).
- Ergebnis: Datentyp- & Zustandsautomaten-Spezifikation.

**B2 · Region / Layout / Speicher-Modell**
- Zweck: `Region{arena,offset,len,gen}`, `Layout{shape,strides,dtype}`, eigenes DType-Enum, `MemoryManager`-Schnittstelle, Arena-Konzept (ohne GPU-Details).
- Quellen: `resident_subengine.md` §4.2.
- Abhängt von: A4.
- Zu klären: `gen`-Entwertung nach Evict/Reload, No-Tensor/No-Pointer-Invariante in Signaturen.
- Ergebnis: Speicher-Abstraktionsvertrag.

**B3 · Operation-/Op-Modell**
- Zweck: `Operation` mit `params`+`requires()` (Ressourcenart, Modell, Index, erlaubte Engine), `materialize:bool`, `Value::Host|Device(handle)`, impliziter Graph via Reihenfolge.
- Quellen: `resident_planner.md` (Ausgabe), `resident_subengine.md` (Task≠Op).
- Abhängt von: B1, B2.
- Zu klären: Handle-/Lease-Repräsentation (`Region` mit `gen`), Erweiterungspunkt Affinität in `requires()`.
- Ergebnis: Op-/Value-Vertrag.

**B4 · Index-Trait & -Vertrag**
- Zweck: `Index`-Trait (`stamp/stamps/read/apply/changed/capabilities/children/members/summary/prefetch`), Entry/Stamp-Semantik, `Scope`-Enum, `AccessMethod`, Schreibreihenfolge, Level-Signal `watch<u64>`.
- Quellen: `resident_index.md` (Index-Vertrag, Scope).
- Abhängt von: A4.
- Zu klären: Fallback-Stamp (size+mtime→Hash, Racy), Stamp-Spalte ohne Vektorseiten, Mehrschreiber-Serialisierung.
- Ergebnis: Index-Trait-Spezifikation (eigenständig vom Kern).

**B5 · Kernel-Abstraktion**
- Zweck: `SegmentDesc{base,count,layout}`, `Layout{Fixed{stride}|Variable{offsets}}`, `KernelHit{offset,score}`, Kernel-Trait (Batch-Start/-Ende, Ergebnisse, geschriebene Bereiche, Fortschrittszähler) + Mock.
- Quellen: `resident_index.md` (Kernel-Schnittstelle), `resident_subengine.md` §4.5.
- Abhängt von: B2.
- Zu klären: Adress-/Offset-Agnostik, Rückabbildung-Zuständigkeit (Manager, nicht Kernel).
- Ergebnis: Kernel-Trait + Mock-Vertrag.

**B6 · CapabilitySet & Profile-Typen**
- Zweck: `CapabilitySet` (Achsen/Werte pro Hardware), `Profile{position:f32, config}`, keine festen Profilnamen/-Enums.
- Quellen: `resident_precision.md` (Capability-Listen, Profilbildung).
- Abhängt von: A4.
- Zu klären: Datentyp der Achsen, Erweiterbarkeit ohne Fassaden-/Task-Änderung.
- Ergebnis: Präzisions-Datentypen.

---

## Gruppe C — Orchestrierung

**C1 · Fassade „Resident"**
- Zweck: Einstiegspunkt/Fassade: Config vor Init, Init (min Idle-Threads), Queueing der drei Tasktypen (Embedding/Similarity/Inferenz), erweiterbare Parameterobjekte, Index-Übergabe/-Verwaltung, Ergebnisabholung (Event/Notify), Server/Daemon/CLI.
- Quellen: `resident.md` (Einstieg/Fassade, Kontrollfluss, Klarstellung).
- Abhängt von: B1, B3, B4.
- Zu klären: Parameterobjekt-Design (Query/Passage-Enum, Zukunftserweiterungen), Lazy-Init-Fassade (verbraucht initial keine Ressourcen).
- Ergebnis: Öffentliche API-Spezifikation.

**C2 · Processor (Orchestrierung)**
- Zweck: Task-Queue + 3 Op-Queues (CPU/GPU/shared), Worker-Parkplatz, Worker-Loop-Priorität (GPU-Ergebnis→GPU-füttern→CPU-füttern→expand), GPU-Completion (Callback + `cuEventQuery`), Datenübergabe/Leases, Backpressure/Starved-Hints, Notify-Topologie, Eviction, Abschluss/Fehler.
- Quellen: `resident_processor.md`.
- Abhängt von: B1, B3, C3.
- Zu klären: Callback+`cuEventQuery` ohne Doppelverarbeitung; Worker-Zahl vs. launch-bound GPU-Worker; Starved-Hint-Verfall; Mindestpuffer-Parameter.
- Ergebnis: Orchestrierungs-Spezifikation.

**C3 · Planner (reine Taskexpansion)**
- Zweck: Zustandslose Expansion Task→Op-Folge, Device-Wahl, `materialize`-Regel, Folgeoperationen + Zähler offener Ops, Expansionsregeln pro Tasktyp.
- Quellen: `resident_planner.md`.
- Abhängt von: B3, B6.
- Zu klären: Konfigurationsformat der Expansionsregeln (Registry pro Modell/Tasktyp); Gewichtung des Starved-Hints.
- Ergebnis: Expansions-Spezifikation + Regel-Registry-Format.

---

## Gruppe D — Subengine-Ressourcen

**D1 · Subengine-Gemeinsames (Lifecycle & Pull)**
- Zweck: Invarianten (§2), Lebenszyklus `Unloaded→Loading→Resident→Leased(n)→Idle→Evicting`+`Failed`, Leases, Lazy-Init (`get_or_try_init`+Semaphore), Aggregation, Pull-Schnittstelle `claim(filter,budget)`, Selbstorganisation per Durchsatz.
- Quellen: `resident_subengine.md` §2,§6,§7; `resident.md` (zwei Subengines).
- Abhängt von: B1, B2, B3.
- Zu klären: Filterprädikate (Priorität/Affinität/Verfügbarkeit), Eviction-Bedingungen (in_flight==0, Leases 0, TTL).
- Ergebnis: Gemeinsamer Subengine-Vertrag.

**D2 · Device & Ausführung (Lane/ExecContext)**
- Zweck: `DeviceHandle`, `ExecContext`/Lane (Stream, Priorität, Events, ParamBlock, Scratch), Lane-Klassen (Query/Index/Copy), CUDA-Init-Kosten.
- Quellen: `resident_subengine.md` §4.1.
- Abhängt von: D1.
- Zu klären: Candle-`CudaDevice` mit eigenem Stream; Primary-Context Device-Pointer in eigenem Kernel lesbar; CPU-Lane als Worker-Gruppe.
- Ergebnis: Ausführungskontext-Spezifikation.

**D3 · GPU-Speicher & Arenen**
- Zweck: Arenen nach Lebensdauer (Weights/Index-Segment/Workspace/Staging/ParamBlock), Budget via `cuMemGetInfo`, stream-geordnete Wiederverwendung, Pinned Staging, dtype pro Device (GPU f16/CPU f32).
- Quellen: `resident_subengine.md` §4.2.
- Abhängt von: B2, D2.
- Zu klären: Fremde Allokation als `CudaStorage` ohne Candle-Free; `cudaMalloc/Free`-Vermeidung im Betrieb.
- Ergebnis: GPU-Speichermanagement-Plan (überlappt E3).

**D4 · Modell-Ressourcen (Encoder)**
- Zweck: `ModelSpec`+Registry, `LoadedEncoder` (Candle-Adapter als einzige Candle-Stelle), `ProjectionHead`, L2-Kernel, `encode_into`/`WriteSlotOp`, Batching per Token-Budget/Bucketing.
- Quellen: `resident_subengine.md` §3,§4.3; `resident.md` (Modelle).
- Abhängt von: D2, D3.
- Zu klären: Gewichtsnamen kompatibel mit candle `BertModel`; ColBERT-Checkpoint Ort/Format (Wechselpfad); kein ColBERT-Modul in candle-transformers.
- Ergebnis: Encoder-Ressourcen-Spezifikation.

**D5 · Tokenizer & Normalizer (CPU, geteilt)**
- Zweck: `tokenizers`-Crate, genau eine Instanz, Query/Doc-Modus (Marker/`[MASK]`/doc_maxlen), Normalizer CPU, Ausgabe `u32`-IDs + Attention-Maske, Längen-Bucketing.
- Quellen: `resident_subengine.md` §3; `resident_first.md` (Tokenizer); `resident.md`.
- Abhängt von: D1.
- Zu klären: GPU-Tokenizer-Adressmapping (laut `resident.md` angedacht, laut `resident_subengine.md` nur Ausblick) → Widerspruch auflösen.
- Ergebnis: Preprocessing-Spezifikation.

**D6 · Kernels & Scorer**
- Zweck: `KernelRegistry` (NVRTC/CUBIN/Candle), Compile-Time- vs. ParamBlock-Parameter, `MaxSimOp` (Kachel-Kette→fused), Top-k-Reduktion/Merge, Work-Slice/Preemption, Query-Batching.
- Quellen: `resident_subengine.md` §4.5.
- Abhängt von: B5, D2, D3.
- Zu klären: MaxSim Kachel-Kette vs. fused Kernel messen; CPU-SIMD-Pfad; Padding-Maskierung.
- Ergebnis: Kernel-/Scorer-Spezifikation.

**D7 · Index-Segmente in der Subengine**
- Zweck: `SegmentView`/Slot/Slot-Klassen, Schreiben/Upsert/Gültigkeitsbit, Mapping `(segment,slot)→EntryRef`, Segment-Basistabelle, bidirektionale Sync.
- Quellen: `resident_subengine.md` §4.4.
- Abhängt von: B4, D3; überlappt stark mit Gruppe E.
- Zu klären: Konsistenzmodell der Index-Sync (Segment-Generationen/Epochen) festlegen.
- Ergebnis: Segment-Repräsentation (Brücke zu E).

**D8 · CPU-vs-GPU-Unterschiede & Ausbaupfad**
- Zweck: Unterschiede (§8) konsolidieren, API-bruchfreier Ausbaupfad (§9, Phasen 1–5) in konkrete Planungsstufen überführen.
- Quellen: `resident_subengine.md` §8,§9.
- Abhängt von: D1–D7.
- Zu klären: Reihenfolge Phase 1 (Candle-Hüllen) → Phase 5 (eigener Forward).
- Ergebnis: Inkrementeller Umsetzungsstufenplan.

---

## Gruppe E — Index-Management & Synchronisation

**E1 · StreamedIndexManager-Grundgerüst**
- Zweck: Rolle als Komponente der CPU/GPU-Engine (nicht Fassade/Index), Konstruktion mit Engine + lazy Index, CPU-Manager trivial (RAM primär) vs. GPU-Manager.
- Quellen: `resident_index.md` (Drei Schichten, Leitprinzipien).
- Abhängt von: B4, D7.
- Zu klären: mmap-Residency über RAM-Größe, `madvise`-Prefetch.
- Ergebnis: Manager-Rollen-Spezifikation.

**E2 · Scope & Resolver**
- Zweck: Scope-Auflösung durch Manager, pluggable `ScopeResolver` pro `AccessMethod`, `UnsupportedScope` (kein stilles Ausweichen), Zentroid-Beispiel, kompakte Entry-Mengen (Ranges/Bitmap).
- Quellen: `resident_index.md` (Scope und Auflösung).
- Abhängt von: B4, E1.
- Zu klären: Resolver-Registrierung, Kompatibilitätsprüfung (Verfahren deklariert UND Resolver vorhanden).
- Ergebnis: Scope-Resolver-Spezifikation.

**E3 · ArenaAllocator (GPU)**
- Zweck: Lazy nach VRAM-Budget, Free-List, Arena-Offsets `u64`, Copy-on-Write-Doppelbedarf eingeplant, Copy-Stream mit Pinned Staging.
- Quellen: `resident_index.md` (Manager-Disziplin/Arena); überlappt D3.
- Abhängt von: D3.
- Zu klären: Budgetplanung für CoW-Spitzen.
- Ergebnis: Arena-Spezifikation (konsolidiert mit D3).

**E4 · Manager-Disziplin**
- Zweck: Nur in Lücken mutieren, atomarer 8-Byte-Pointer-Swap, CoW-Hierarchie (Root→Segmenttabelle→Segment), Pointer-Lesezeitpunkt (Launch-Arg vs. Acquire-Load im Ringmodus), blocking Reads im Worker-Pool.
- Quellen: `resident_index.md` (Manager-Disziplin).
- Abhängt von: E1, E3, B5.
- Zu klären: Acquire-Load `ld.acquire` (sm70+/Turing), Event/Fence-Sichtbarkeit.
- Ergebnis: Mutations-/Swap-Disziplin.

**E5 · Kommunikationsslot (feste Reihenfolge)**
- Zweck: 1) Kernel-Writes lesen→`apply`, 2) Ergebnisse auflösen (Adresse→Entry), 3) Updates einspielen; Ergebnisstatus (entfernt→verworfen, Update anstehend→`outdated`), `Hit{entry,score,outdated}`.
- Quellen: `resident_index.md` (Kommunikationsslot).
- Abhängt von: E4, B4, B5.
- Zu klären: Reihenfolge-Invarianz (Mapping-Stand pro Batch), Indexieren während Suche.
- Ergebnis: Slot-Protokoll-Spezifikation.

**E6 · Synchronisation & Echo-Vermeidung**
- Zweck: Zustandsbasierter Abgleich (nur residente Entries), Signal vor Scan quittieren, Stamp-Lesereihenfolge beim Laden, Echo-Unterdrückung via `apply`-Rückgabe-Stamp, Zwei-Engines-Konvergenz, ABA.
- Quellen: `resident_index.md` (Synchronisation).
- Abhängt von: B4, E5.
- Zu klären: optionale `changed_since(watermark)`-Optimierung, baumartige max-Stamp-Varianten (pro Konsument).
- Ergebnis: Sync-Spezifikation + Testmatrix.

**E7 · Residency & Ladestrategie**
- Zweck: Lazy/scope-getrieben, Ladegranularität (ganze Segmente vs. Gather, Dichteschwelle), Tiering (Summaries langlebig), Aggregation über Scope-Union, Single-Flight, Eviction (VRAM>RAM).
- Quellen: `resident_index.md` (Residency); überlappt E7↔Kalibrierung.
- Abhängt von: E3, E5.
- Zu klären: Dichteschwelle ggf. als Kalibrierungsachse (Verweis F).
- Ergebnis: Lade-/Eviction-Strategie.

**E8 · Kernel schreibt Segmente (Indexaufbau)**
- Zweck: Vorab vergebener beschreibbarer Bereich, Cursor-Schreiben ohne Allokation, Teilergebnis bei Bereichsmangel, Zuordnung im Slot (Schritt 1), Rückabbildung Adresse→Entry (Fixed-Arithmetik/Variable-Binärsuche).
- Quellen: `resident_index.md` (Kernel schreibt Segmente, Rückabbildung).
- Abhängt von: E5, B5.
- Zu klären: Embedding-Ringe (neue Segmente) vs. MaxSim-Ringe (Hits).
- Ergebnis: Indexaufbau-Pfad-Spezifikation.

**E9 · Addons (Iteration/Kaskade/Rekursion)**
- Zweck: Keine Kernfunktion; Iteration/Rekursion nur via Addon über die normale Queue (kopierte/geänderte Params), nicht unterscheidbar von App-Tasks.
- Quellen: `resident_index.md` (Addons).
- Abhängt von: B1, C3.
- Zu klären: Nur Erweiterungspunkt sichern (erweiterbares Parameterobjekt).
- Ergebnis: Addon-Konzeptnotiz (keine Kernänderung).

---

## Gruppe F — Präzision & Kalibrierung

**F1 · CapabilitySet-Erhebung**
- Zweck: Achsen/Werte pro Subengine (GPU: Operanden/Akkumulator/Scoring/Fusion/Batch/Tile; CPU: Operanden/Scoring/Threads/Tile).
- Quellen: `resident_precision.md` (Capability-Listen).
- Abhängt von: B6, D6.
- Ergebnis: Capability-Erhebungsspezifikation.

**F2 · Calibrator**
- Zweck: Aufzählung+Pruning, Mini-Benchmark interleaved, Pareto-Front, Stabilitätslogik (Streuung/adaptiv), harte Obergrenze→`unstable`, GPU ohne Preemption (kurze Läufe), niedrige Priorität, Mess-Trait + Mock.
- Quellen: `resident_precision.md` (Calibrator, Messmethodik).
- Abhängt von: F1.
- Zu klären: Verwerfungskriterien (NaN/Inf, degeneriert, Maximalabweichung), kombinatorische Explosion begrenzen.
- Ergebnis: Kalibrierungsablauf-Spezifikation.

**F3 · Profilbildung**
- Zweck: Profile allein aus Spread der Pareto-Front, normierte Skala, Bogenlängen-Position `p∈[0,1]`, Zusammenfassen redundanter Punkte, Ordnung/Herabstufung.
- Quellen: `resident_precision.md` (Profilbildung).
- Abhängt von: F2.
- Zu klären: Einziger Konfigparameter = Spread-Schwellwert.
- Ergebnis: Profilbildungs-Spezifikation.

**F4 · Task-Matching**
- Zweck: `TaskParams.precision:f32` (0=fast,1=precision), Klemmen, NaN=Fehler, min|t−p|, Tie-Break genauer, pro Engine relativ.
- Quellen: `resident_precision.md` (Matching).
- Abhängt von: F3, B6, C1.
- Ergebnis: Matching-Spezifikation.

**F5 · CalibrationStore / Persistenz**
- Zweck: Initial einmal, pro Modell×Subengine, Fingerprint (GPU/Treiber/CUDA/CPU/Modell-ID/Engine-Version), Zustände `Calibrated|Stale|Missing`, Persistenz inkl. `unstable`/Verwerfungen.
- Quellen: `resident_precision.md` (Persistenz).
- Abhängt von: F3.
- Ergebnis: Persistenz-Spezifikation.

---

## Gruppe G — Konkrete Modell-/Index-Integration

**G1 · e5-small Modell & Pipeline-Config**
- Zweck: Dense Bi-Encoder, Präfixe `query:`/`passage:`, Mean-Pooling+Maske, L2, `max_seq_len` 64–128/512, dtype GPU f16/CPU f32, `BertModel` statt `xlm_roberta`.
- Quellen: `resident_first.md`.
- Abhängt von: D4, D5.
- Zu klären: Gewichtsnamen-Kompatibilität; Mean-Pooling+L2 gegen sentence-transformers-Referenz; f16/f32-Toleranz; Präfix-Behandlung; `model.safetensors` vorhanden/konvertieren.
- Ergebnis: Konkrete Modellkonfiguration + Verifikationschecklist.

**G2 · Index-Schema konkret**
- Zweck: `FixedSizeList<f16,384>` + Zeilen/Offset-ID, Parquet, Schema-Versionierung (Modell-ID, Dim, Pooling, Präfixe, dtype), Payload (Zieldatei+Zeichenoffsets+Checksum), Persistenz außerhalb der Engine.
- Quellen: `resident_first.md` (Index), `resident.md`/`resident_index.md` (Persistenz außerhalb).
- Abhängt von: B4, G1.
- Zu klären: Persistenz-Verantwortung (Index-Objekt besitzt Mutex/Persistenz); Modellwechsel = neue Schema-Version + Re-Embedding.
- Ergebnis: Index-Schema-Spezifikation.

**G3 · Similarity/Scorer-Konfiguration**
- Zweck: `Cosine` (Standardpfad), Scope-Subset, `top_k`, Entscheidungsgrundlagen (Anzahl, Score-Abstand, Streuung, Task-ID) für die äußere Schicht.
- Quellen: `resident_first.md` (Similarity).
- Abhängt von: G1, G2, C3.
- Zu klären: Rerank nur als Similarity mit anderem Scorer auf Subset (kein eigener Tasktyp).
- Ergebnis: Similarity-Konfiguration.

---

## Gruppe H — Ausblick / zurückgestellt (nur umreißen, Erweiterungspunkte sichern)

**H1 · GPU-Ringbuffer-Modus** — SPSC-Ring (`w`/`r`, CAS nur für Exit), getrennte Ringe Embedding/MaxSim, persistenter Kernel (`idle_spin`/`max_run_time`), blockierte Segmente. Quellen: `resident.md` (Ausblick), `resident_index.md` (Ringbuffer-Modus), `resident_subengine.md` §4.6.
**H2 · Gleitende Indizierung** — großer Chunk-Input, gleitender Kernel. Quellen: `resident.md`, `resident_planner.md`.
**H3 · Funnel/PLAID & Cross-Encoder/Reranker** — Centroid-Codes/Funnel, Reranker als eigener Inferenz-Tasktyp (lazy, TTL). Quellen: `resident_subengine.md` §11, `resident_first.md`.
**H4 · Graph-Capture & Persistent-Scorer** — Graph-Decorator (`Direct`/`Capture`), Persistent-Kernel-Ring. Quellen: `resident_subengine.md` §4.6.

> Für alle H: In Stufe 2 nur **Erweiterungspunkte** (Parameterobjekt, `requires()`-Affinität, Op-Austausch hinter gleichem Trait) sicherstellen, keine Detailplanung.

---

## Querschnitt Q (begleitet alle Gruppen)

- **Q1 · Fehler-Taxonomie** (`thiserror`): konsolidiert u. a. `UnsupportedScope`, `IndexUnavailable`, `ArenaExhausted`, `StaleMapping`(intern), `TransferFailed`, `Incompatible`, `CalibrationUnstable`, `NoActiveProfile`, `InvalidPrecision`, Planner-/Config-/Capability-Fehler.
- **Q2 · Logging** (`tracing`): Span pro Task-ID/Ressource/Manager-Zyklus; geloggte Ereignisse pro Gruppe.
- **Q3 · Teststrategie & Mocks:** Kernel-Mock (B5), Mess-Mock (F2), No-GPU-Pfad; Testmatrizen je Paket (insb. E5/E6 Sync-Tests, F2–F5 Kalibrierung, B1 Queue-Semantik).
- **Q4 · Feature-Flags:** reale CUDA-Pfade hinter Flag, Logik ohne GPU testbar (A2).

---

## Empfohlene Sequenz für den nächsten Lauf (Stufe 2)

```
A1 → A2 → A3 → A4
→ B1 → B2 → B3 → B4 → B5 → B6
→ C1 → C3 → C2
→ D1 → D2 → D3 → D4 → D5 → D6 → D7 → D8
→ E1 → E2 → E3 → E4 → E5 → E6 → E7 → E8 → E9
→ F1 → F2 → F3 → F4 → F5
→ G1 → G2 → G3
→ H1..H4 (nur umreißen)
Q1..Q4: nach A4 initial fixieren, je Gruppe fortschreiben.
```

**Für die zweite Metaplanung (Schärfung) markiert, höchste Unschärfe/Risiko:**
1. **A1** — fehlende These `resident_streaming.md` (harte Lücke, blockiert Umfangsabgrenzung, erledigt siehe unten).
2. **D5 vs. resident.md** — Widerspruch GPU-Tokenizer/Adressmapping (im Hauptkonzept angedacht, in Subengine nur Ausblick).
3. **D7/E (Konsistenzmodell Index-Sync)** — explizit „noch festzulegen" (Segment-Generationen/Epochen).
4. **D3↔E3** — doppelte Arena-/Speicherbeschreibung, muss konsolidiert werden (Dopplungsgefahr).
5. **C3** — Konfigurationsformat der Expansionsregeln noch offen.
6. **G1** — mehrere unverifizierte Modell-Checks (`[prüfen]`), direkt implementierungskritisch.

---

Die These ist eingelesen. Hier der **Abgleich These ↔ Plan** (These = Idee/Vision, Plan = umzusetzender Teil), inklusive der daraus folgenden Schärfung des Metaplans.

---

## Kernbefund

Der Plan setzt die **Fundamentschicht** der These um — Disaggregation (Tokenizer/Weights/Kernel), Residency, modell-entkoppelte MaxSim-Geometrie und ein streambares Index-Substrat — und baut daraus eine **produktionsreife, nebenläufige Engine**. Dabei gibt es drei systematische Verschiebungen gegenüber der These:

1. **Mechanismus-Reinterpretation:** Das zentrale Nebenläufigkeitsmodell der These (SPSC-Ringbuffer, §3) wird in Phase 1 durch ein **Pull-basiertes Lock-free-Task-Queue-Modell + Processor/Planner** ersetzt. Residency bleibt das Ziel, der Ringbuffer wird zur *späteren* Spezialisierung (Persistent-Kernel-Modus).
2. **Scope-Ausklammerung:** Die „Intelligenz"-/Qualitätsschicht der These (Chunking, Frequenz-/Wavelet-Kompression, Cascade-Policy, Retrieval-Qualität) wird **bewusst aus der Engine herausgezogen** und der äußeren Suchschicht zugewiesen („Nicht Teil der Engine"). Die Engine ist Substrat, nicht Retrieval-Algorithmus.
3. **Zwei Thesen-Ideen fehlen vollständig und ohne Hook:** die signaltheoretische Kompression (§6/§6.1) und die mutable-weights-Online-Adaption per Contextual Bandit (§9/§9.1) — die spekulativsten Teile.

Außerdem: Das konkrete Modell ist **dense e5-small (Cosine)**, **nicht** ColBERT/MaxSim. MaxSim existiert nur als Kernel-Infrastruktur/Wechselpfad.

---

## Abgleich-Tabelle

Legende Status: **K**=Kern/umgesetzt · **I**=Infrastruktur/Hook vorbereitet · **A**=Ausblick/später in der Engine · **E**=bewusst außerhalb der Engine · **—**=nicht adressiert

| These (§) | Idee | Status | Beleg im Plan | Implikation |
|---|---|---|---|---|
| §2 | Disaggregation Tokenizer/Weights/Kernel | **K** | `resident.md` Stack; `subengine` §3; `KernelRegistry` §4.5 | Rückgrat, voll deckungsgleich |
| §3 | Residency (keine Per-Request-Last, Lazy+TTL) | **K** | `resident.md` Kontrollfluss; `subengine` §6 Lifecycle | Ziel erreicht |
| §3 | **Ringbuffer-Producer/Consumer-Compute** | **A** | `resident.md` Ausblick; `index` „Ringbuffer-Modus (geplant)"; `subengine` §4.6 | **In Phase 1 ersetzt durch Pull-Task-Queue** |
| §3 | Mehrere Modelle resident | **K** (eingeschr.) | fixer Satz vor Init, Modell-ID-geschlüsselt, lazy | dynamisches Hinzufügen nicht vorgesehen |
| §3 | Batching optional / Streaming | teils **K** | Batching bleibt reale Mechanik (Token-Budget) | Streaming-Äquivalenz nur als Ausblick |
| §4 | MaxSim = Geometrie, modell-entkoppelt | **K** (Prinzip) | `precision` „nicht gekoppelt"; `index` „Kernel kennt nur Segmente"; `MaxSimOp` | Prinzip verankert |
| §4 | ColBERT-Multi-Vektor-Modell | **A** | `resident_first` Wechselpfad; „kein ColBERT-Modul in candle-transformers" | zunächst dense e5 |
| §4 | Position statt Relation (Index speichert Positionen) | **K** | Index speichert Entries/Vektoren; Relation = äußere Schicht | konsistent |
| §5 | Sliding-Window statt Chunking | **A** | `resident.md`/`planner` „gleitende Indizierung (Ausblick)" | nur Datenmodell vorbereitet |
| §5 | Index-Datenmodell für Multi-Vektor/Fenster | **I** | `index` Entry variabler Länge; `subengine` Slot-Klassen | Hook vorhanden |
| §5 | Chunking selbst | **E** | `index`/`precision` „Nicht Teil … Suchschicht" | außerhalb |
| §6 | Frequenz-Kompression (JPEG/DCT) | **E / —** | nur als Oberschicht-Vorfilter erwähnt (`FFT/Kompressionsdomäne` als Scope) | Engine implementiert es nicht |
| §6.1 | Zwei Achsen / Semantic Spectrogram / Wavelet | **—** | — | gar nicht adressiert |
| §7 | Cascaded Refinement (fraktal) | **I / A** | `index` „Addons (Iteration/Kaskade/Rekursion)"; Funnel „später"; Scope-Resolver/Centroid | als Addon-Muster ermöglicht, nicht Kern |
| §9 | Parameter/Aktivierung-Dualität, **mutable weights** | **—** | Plan: Weights immutabel (Weights-Arena langlebig); nur **Index** bidirektional mutabel | Divergenz |
| §9 | Online-Adaption (Bandit, Reward-Stream) | **—** | — | gar nicht adressiert |
| §9.1 | Bandit als Meta-Router über Cascade | **—** | — | gar nicht adressiert |
| §10 | Drift-Monitor / Re-Embed-Trigger | **I / E** | Stamps/Change-Notify stützen Re-Embed; Monitor selbst außen | Hook über `changed()`/Stamps |
| §10 | Hierarchischer Index (coarse/fine, Frequenzbänder) | **I** | `index` `children`/`subtree`/`summary` generisch; Frequenzbänder nicht | Struktur generisch offen |
| §10 | Bridge zu pylate (Referenz/Validierung) | teils **—** | `resident_first`: Referenzvergleich mit sentence-transformers (nicht pylate) | andere Referenz |
| §2/§10 | Stack (safetensors/tokenizers/candle/cudarc/Parquet) | **K** | voll + tokio/crossbeam/mmap | deckungsgleich |

---

## Zentrale Divergenz (explizit festhalten)

**These §3:** „The answer is not classical multithreading but a producer–consumer architecture over ring buffers." — Das ist die *zentrale Antwort* der These auf das Nebenläufigkeitsproblem.

**Plan:** Die zentrale Antwort ist stattdessen die **Lock-free-Pull-Queue + Processor/Planner/Subengine** (`resident.md` Queue-Logik, `resident_processor.md`, `resident_planner.md`). Der Ringbuffer bleibt als **optionaler Persistent-Kernel-Modus für den Scorer** erhalten (`subengine` §4.6: „Sinnvoll für den Scorer, nicht für den Encoder").

→ Das Ziel (Residency, kein Setup-Overhead) ist identisch; der Weg ist für Phase 1 bewusst ein anderer. Das ist **keine Inkonsistenz**, sondern eine Umsetzungsentscheidung — aber sie muss im Plan explizit als solche benannt werden, weil sonst der Eindruck entsteht, §3 der These sei 1:1 umgesetzt.

---

## Auswirkungen auf den Metaplan

**A1 ist aufgelöst.** Das Paket „These destillieren" entfällt als Lücke; der Abgleich oben ist sein Ergebnis. Die Umfangsabgrenzung steht damit fest.

**Neues Paket (ersetzt A1): A1′ · Scope-Grenze Engine ↔ Suchschicht fixieren**
- Zweck: Verbindliche Grenze dokumentieren — was die Engine liefert (resident Compute, Index-Substrat, MaxSim-Geometrie, Streaming-Sync) vs. was außen bleibt (Chunking, Kompression/Signaltheorie, Cascade-Policy, Qualität, Drift-Monitor, Bandit).
- Quellen: Abgleich oben; `index`/`precision` „Nicht Teil"-Abschnitte.
- Ergebnis: Scope-Charta. **Blockiert sonst C1 (Fassaden-API) und E2 (Scope-Resolver).**

**Gruppe H neu gemappt an Thesen-Sektionen** (Status in Klammern):
- **H1 · GPU-Ringbuffer-Modus** ← These §3 (**A**): die eigentliche „zentrale Antwort" der These; in Phase 1 durch Pull-Queue ersetzt. Planungsauftrag: nur Erweiterungspunkte sichern (SPSC-Ring, getrennte Ringe Embedding/MaxSim, Acquire-Load-Pointer in E4).
- **H2 · Gleitende Indizierung** ← These §5 (**A/I**): Datenmodell (Entry variabler Länge, Slot-Klassen) ist bereits der Hook; Kernel/Pipeline später.
- **H3 · Funnel/PLAID + Cross-Encoder** ← These §7 (**I/A**): über Addons + Centroid-Scope-Resolver realisierbar.
- **H4 · Graph-Capture/Persistent-Scorer** ← These §3/§9 (**A**).
- **H5 (neu) · Signalkompression (FFT/Wavelet, Semantic Spectrogram)** ← These §6/§6.1 (**E/—**): **nicht Engine-Scope.** Planungsauftrag: lediglich prüfen, dass `AccessMethod`/`Scope`/`summary()` eine spätere „Frequenz-/Kompressionsdomäne" als *Vorfilter der Oberschicht* nicht ausschließen. Kein Engine-Workstream.
- **H6 (neu) · Mutable Weights / Online-Bandit** ← These §9/§9.1 (**—**): **nicht Engine-Scope.** Divergenz dokumentieren (Plan: Weights immutabel, nur Index bidirektional). Falls je gewünscht: eigener neuer Workstream (zusätzliche Reward-Queue, mutable Weights-Arena) — in Phase 1 nur „nicht verbauen".

**Aktualisierte Risiko-/Schärfungsliste** (Stand nach Abgleich):
1. ~~A1 fehlende These~~ → **erledigt.**
2. **A1′ Scope-Grenze** — neu, höchste Priorität (blockiert API-Design).
3. **D5-Widerspruch GPU-Tokenizer** — bleibt offen (Haupt- vs. Subengine-Doku).
4. **D7/E Konsistenzmodell Index-Sync** — bleibt offen.
5. **Ringbuffer-Reinterpretation (H1)** — als Umsetzungsentscheidung dokumentieren, damit die These-Treue nachvollziehbar ist.
6. **G1 e5 vs. These-ColBERT** — klarstellen: dense Standardpfad jetzt, MaxSim/ColBERT als Infrastruktur/Wechselpfad.

---

Der Metaplan ist damit vollständig fundiert: alle Komponenten identifiziert, gruppiert, geordnet, und der Umfang gegenüber der These sauber abgegrenzt (Kern / Hook / Ausblick / außerhalb / nicht adressiert).
