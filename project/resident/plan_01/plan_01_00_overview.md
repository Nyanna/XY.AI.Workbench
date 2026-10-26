# Plan 01 — Resident-Engine als Layer `resident` in xy-ai-rag (Masterplan)

Mehrstufiger Umsetzungsplan zur Anbindung der Resident-Engine (`rag/src/resident`) an die Layer-Topologie der Retrieval-Engine `xy-ai-rag`, analog zu `GrepLayer` (`rag/src/layers/grep_layer.rs`) und `TrigramLayer` (`rag/src/layers/trigram/layer.rs`). Stand der Analyse: 2026-10-26. Pfade sind relativ zu `/home/user/xyan/xy.ai.workbench/`.

Dieses Dokument ist der Einstieg: Ziel, Leitplanken, Stufen, Gates, Paketverzeichnis, Risiken, Fortschritt. Belege stehen in `plan_01_01_evidence.md`, Entscheidungen in `plan_01_02_decisions.md`, die Umsetzung je Stufe in `plan_01_10_s0_baseline.md` bis `plan_01_17_s7_verification.md`, der Ausblick in `plan_01_99_outlook.md`.

## Lesehinweis

**Belege:** **[V]** im Quelltext oder in der Dokumentation live verifiziert (Pfad und Symbol genannt) · **[W]** Domänenwissen oder Schätzung, nicht verifiziert · **[?]** offen, Klärung in Stufe 0. Die Legende entspricht `rag/docs/architecture/dependency_verification.md`.  
**IDs:** Stufen `S0` bis `S7` · Pakete `L<Stufe>.<n>` · Entscheidungen `D1` bis `D20` · Leitplanken `LP1` bis `LP13` · Risiken `R1` bis `R14` · Gates `G0` bis `G7` · Befunde `E*`, `K*`, `W*` · Budgets `B1` bis `B12` · Abnahmekriterien `AC1` bis `AC16` · Offene Punkte `O1` bis `O11`.  
**Paketschema** wie in `project/resident/resident_planplan.md`: `Zweck`, `Quellen`, `Abhängt von`, `Zu klären`, `Ergebnis`. Jedes Paket in den Stufendateien trägt diese fünf Felder in einem Absatz.  
**Format für Folgeläufe:** Listen und Tabellen stehen bewusst in eigenen Abschnitten ohne Fließtext, damit `ast_read` sie vollständig liefert; Abschnitte, die Fließtext und eine Liste mischen, zeigen die Liste in den AST-Werkzeugen nicht. Beim Ändern dieses Prinzip beibehalten.

## 1. Auftrag und Ergebnisbild

Die Resident-Engine ist ein warmgehaltenes Substrat für Embedding und Ähnlichkeitssuche (e5-small, Cosine, Parquet-Index). Plan 01 beschreibt, wie sie als eigener Layer `resident` in die Engine `xy-ai-rag` eingebunden wird, mit allem, was dazu gehört: Layer-Code, Index-Pflege, Anfragepfad, Konfiguration, Fehler- und Statusmodell, Tests, CI und Dokumentation. Gemäß Scope-Charta liefert die Engine Embedding, Similarity und das Index-Substrat; der Layer als Suchschicht besitzt Chunking, Anfrageformulierung, Kandidaten-k und Rerank-Politik, Vollständigkeitsaussagen und den Zeitpunkt der Index-Persistenz.

**Ergebnisbild:** Ein neuer Layer `ResidentLayer` (ID `resident`, Stage `Generate`) in `rag/src/layers/resident_layer/`. Er beantwortet natürlichsprachliche Anfragen über Text- und Quelldateien unterhalb der Suchwurzel mit Treffern auf Chunk-Ebene, hält dafür einen persistenten, modellgebundenen Vektorindex unter `.xyrag/layers/resident/` aktuell, spricht die Engine über die Fassade `Resident` an und liefert Ergebnisse im Format der übrigen Layer (`File`, `Lines`) plus einem Objekt `Resident`. Ohne Modell oder Backend degradiert er kontrolliert zu `skipped`, ohne die Anfrage der anderen Layer zu stören.

### Beispiel (schematisch)

```
xyrag resident="wie wird der Index mit dem Speicher synchronisiert" directory=rag/src maxResults=10
xyrag resident="..." residentIndex=update      # Index aufbauen oder aktualisieren (explizit, abbrechbar)
xyrag resident="..." residentIndex=status      # nur Abdeckung und Zustand melden
```

### Ergebnisform (schematisch, Ausgabeformat des CLI [V], E53)

```yaml
results:
  - File: rag/src/resident/manager/streamed.rs
    Lines:
      44: 'pub trait IndexManager: Send + Sync {'
    Resident:
      Score: 0.8421
      Rank: 1
      Chunks:
        - {Start: 40, End: 63, Score: 0.8421}
layers:
  - layer_id: resident
    stage: generate
    ran: true
    skipped: false
    aborted: false
    contributions: 7
    detail: {backend: resident-cpu, index_mode: auto, coverage: {visible: 412, indexed: 398, stale: 3}}
```

### Nicht-Ziele

- Keine Retrieval-Logik in der Engine (Charta §4).
- Keine GPU-Pfade, Ringbuffer, Reranker, Funnel, MaxSim oder ColBERT in v1; die Haken bleiben offen (`plan_01_99_outlook.md`).
- Kein Daemon- oder Server-Modus, kein Dateiwächter.
- Keine Änderung an `GrepLayer`, `TrigramLayer`, `GlobLayer`, `MergeLayer` (kein Regressionsrisiko).
- Kein numerisches Ranking im Core, keine Hybrid-Fusion in v1.
- Keine Kalibrierung und keine Precision-Profile im Layer (Fassaden-Default).

## 2. Eingangsbasis und Einschränkung

Ausgewertet wurden die Metaplanung `project/resident/resident_planplan.md` (Gruppen A bis H, Querschnitt Q, Abgleich These und Plan, Auswirkungen), die Architekturdokumente `rag/docs/architecture/` (Scope-Charta, Konventionen, Feature-Matrix, Phasenplan, Addons, Abhängigkeitsverifikation, GPU-Speicher), der Layer-Vertrag und Kern (`rag/src/core/*`), die Referenz-Layer samt `DirCache` und `MergeLayer`, das CLI `bin/xyrag.rs` sowie die Resident-Engine im Quelltext (Fassade, Index, Planner, Processor, Op, Modell, Tokenizer, Kernel, Manager, Segment, Subengine, Fehler, Telemetrie, Mock, Backend).

**Einschränkung:** Die Paketdetails der Metaplanung (Zweck, Quellen, Abhängt von, Zu klären, Ergebnis je Paket) liegen im Markdown als Listen und Tabellen unter den Paket-Titeln und waren mit den im Planungslauf verfügbaren AST-Werkzeugen nicht lesbar. Maßgeblich für die Abgrenzung sind deshalb die Scope-Charta (sie reproduziert die Abgleich-Tabelle der Metaplanung) und der Quelltext. Die Konzeptdokumente `resident.md`, `resident_subengine.md`, `resident_processor.md`, `resident_precision.md`, `resident_index.md`, `resident_planner.md`, `resident_first.md` und die These `resident_streaming.md` wurden in diesem Lauf nicht gelesen; Paket L0.1 gleicht sie gegen den Code ab (Risiko R14).

## 3. Zentrale Befunde

| Nr | Befund | Konsequenz für den Plan |
|---|---|---|
| 1 | Die Engine besitzt alle Bausteine (Modell, Tokenizer, Index, Kernel, Manager, Planner, Processor, Fassade), aber keine produktive `Engine`-Implementierung; `Resident::builder()` wird nur in Tests aufgerufen (E26) | Stufe 1 (Runtime-Assembly) ist Voraussetzung für den Echtbetrieb und der kritische Pfad; die Layer-Entwicklung läuft parallel gegen `FakeBackend` (D3, D12) |
| 2 | `Resident::add_index` legt Indizes nur ab; `IndexId` wird nirgends aufgelöst (E27) | `IndexDirectory` und Fassaden-Verdrahtung (L1.1) |
| 3 | `TextRole` erreicht den Tokenizer nicht, die e5-Präfixe `query: ` und `passage: ` fehlen im Op-Pfad (E28) | Rolle im Op-Pfad (L1.2); der Layer präfixiert nie selbst |
| 4 | Der `Index`-Trait hat keinen Payload-Zugriff; `ScoredHit.source` bleibt ohne Quelle (E49, L1.5) | Trait-Ergänzung als Default-Methode, Rückfall über `ParquetIndex::payload` im Layer |
| 5 | Das CLI ist on-demand, `shutdown(false)` wartet auf `background` (E4); „kept warm“ gilt nur innerhalb eines Prozesses | Kaltstartpfad optimieren, Background standardmäßig aus (D9, D11) |
| 6 | `query` gehört grep und trigram (E9) | eigenes Aktivierungsfeld `resident` (D1) |
| 7 | Kein Layer befüllt den zentralen `SharedIndex` (E15) | eigener Katalog mit `FileStamp` und Hash (D7) |
| 8 | `ParquetIndex`: `verify` per Default an, `flush` schreibt alle Zeilen, erster Zugriff dekodiert die ganze Datei (E36, K5) | Optionen und Flush-Politik (D6), Messung M4 |
| 9 | Die Erstindexierung ist rechenintensiv (Größenordnung Minuten pro MB Quelltext auf der CPU, W2) | Indexierung als budgetiertes, wiederaufnehmbares Wartungsverfahren; `auto` nur klein (D9) |
| 10 | Der globale Include/Exclude-Filter wirkt nur über `DirCache` (E16, E17) | bedarfsgetriebene Abdeckung, Scope gleich Katalog geschnitten mit sichtbaren Dateien (D8) |

## 4. Leitplanken

| ID | Leitplanke | Quelle |
|---|---|---|
| LP1 | Die Engine ist Substrat. Chunking, Query-Formulierung, Kandidaten-k und Rerank-Politik, Vollständigkeitsaussagen und der Zeitpunkt der Index-Persistenz liegen im Layer | Charta §1, §3, §4 |
| LP2 | Layer-Vertrag: stabile `id` (zugleich Storage-Namespace und Aktivierungsfeld), billiges `applies`, Ausgang nur über `LayerStatus`, `background` reagiert zügig auf Cancel | `core/layer.rs` |
| LP3 | Der Layer lässt die Gesamtanfrage nie scheitern: Fehler werden zu `skipped` oder `aborted` mit `detail`, keine Panics | `core/layer.rs`, `core/engine.rs` (E7) |
| LP4 | Das CLI ist on-demand ohne Daemon; das Prozessende wartet auf alle `background`-Tasks | `core/engine.rs`, `bin/xyrag.rs` (E4) |
| LP5 | Persistenz nur über `LayerStorage` (Key/Value und `path_for`-Sidecars) unter `.xyrag/layers/resident/` | `core/persistence.rs` |
| LP6 | Dateiauswahl über `DirCache` mit globalem Include/Exclude, keine Dot-Einträge | `layers/dir_cache.rs` (E16) |
| LP7 | Ergebnisfelder `File` und `Lines`, kompatibel mit `MergeLayer`; der Core rankt nicht numerisch | `layers/merge_layer.rs`, `core/layer.rs` (E12, E13) |
| LP8 | Fehler mit `thiserror`, `non_exhaustive`, typisierte Felder, `IsRetryable`; `anyhow` nur in Binaries | Konventionen Q1 |
| LP9 | Kein neues `cfg(feature)`; `cuda` nur in `backend::create_backend`, `candle` nur in `memory::dtype`; keine Candle-Typen und Zeiger in öffentlichen Signaturen | `feature_matrix.md`, `tests/api_hygiene.rs` |
| LP10 | Logging mit `tracing` und festen Feldnamen; Subscriber nur im Binary; kein `span.enter()` über `await` | Konventionen Q2 |
| LP11 | Testdoubles nur unter `cfg(any(test, feature = "mock"))`; Zahlenvergleiche mit Toleranz (f16 gegen f32: 1e-3 bis 1e-4) | Konventionen Q3, `phase_plan.md` |
| LP12 | Der Index ist modellgebunden (`SchemaHeader`: Modell, Revision, Tokenizer-SHA, Präfixe, Pooling, Normalisierung, Dtype, `max_seq_len`); Abweichung führt zu kontrolliertem Neuaufbau | `index/schema.rs` (E38) |
| LP13 | Phase 1 ist dichtes e5-small mit Cosine auf der CPU; GPU, Ringbuffer, Rerank, Funnel sind Ausblick und werden nicht verbaut | Charta §5, §6; Phasenplan |

## 5. Architektur

```
xyrag (CLI)  -->  Engine::run_query  -->  ResidentLayer  (applies: Feld "resident")
                                            |
                       +--------------------+--------------------------------------------+
                       |  LayerState (lazy, OnceCell)                                    |
                       |  Config  Catalog  Chunker  IndexStore(ParquetIndex)  Indexer    |
                       |  SemanticBackend (Trait)                                        |
                       +--------+------------------------------------------+-----------+
                                | Produktion                                | Tests
                                v                                           v
                        ResidentBackend  (Adapter)                    FakeBackend (mock)
                                |
                                v   Resident::submit_embedding / submit_similarity
                        ResidentRuntime  (Stufe 1)
                                |
                  Processor / Hub  --  CpuEngine  --  Tokenizer, Encoder (candle), Scorer (Cosine)
                                |
                          IndexDirectory  -->  ParquetIndex  (geteilt mit dem Layer)
```

Indexierung: sichtbare Dateien, Katalog-Plan, Chunking, Embedding (Rolle Passage), `ParquetIndex::upsert`, Flush, Katalog-Commit. Anfrage: Query-Embedding (Rolle Query), Similarity mit Scope, Abbildung der Payloads auf Dateien und Zeilen, `ResultEntry`. Details in den Stufen S3 und S4.

## 6. Stufenplan

| Stufe | Titel | Inhalt | Pakete | Gate | Aufwand [W] | Abhängt von |
|---|---|---|---|---|---|---|
| S0 | Baseline | Befunde verifizieren, Entscheidungen einholen, Messbasis, Fixtures, Budgets | L0.1 bis L0.5 | G0 | 1 PT plus Messläufe | — |
| S1 | Runtime-Assembly | CPU-Zusammenbau der Engine: IndexDirectory, Rolle zu Präfix, Ressourcen, CpuEngine, Scorer-Brücke, ResidentRuntime | L1.1 bis L1.7 | G1 | 6 bis 9 PT | S0 |
| S2 | Gerüst | Modul, Registrierung, Konfiguration, Fehler- und Statusmodell, Backend-Naht, FakeBackend, Skelett | L2.1 bis L2.6 | G2 | 3 PT | S0 |
| S3 | Indexierung | Walk, Offsets, Chunker, Katalog, IndexStore, Embedding-Pipeline, Treiber | L3.1 bis L3.8 | G3 | 6 PT | S2 |
| S4 | Query-Pfad | Optionen, Scope, Embedding und Similarity, Trefferabbildung, Ergebnisse | L4.1 bis L4.6 | G4 | 4 PT | S3 |
| S5 | Integration | CLI, Fusion, CPU-Budget, Doku, IP1 mit echtem Backend | L5.1 bis L5.5 | G5 | 2 bis 3 PT | S4, für IP1 auch S1 |
| S6 | Härtung | Degradation, Limits, Migration, Telemetrie, Sicherheit, Performance | L6.1 bis L6.6 | G6 | 4 PT | S5 |
| S7 | Verifikation | Testmatrix, CI, Abnahmekriterien, Definition of Done | L7.1 bis L7.4 | G7 | 3 PT | S6 |

Summe etwa 29 bis 32 Personentage [W]; der Hauptunsicherheitsfaktor ist S1 (R1).

### Reihenfolge und Parallelisierung

```
S0 --+--> S1 -----------------------------+
     |                                      v
     +--> S2 --> S3 --> S4 --> (G4) --> IP1 (in S5, benötigt G1 und G4) --> S5 --> S6 --> S7
```

S1 und der Strang S2 bis S4 sind unabhängig, weil der Layer nur die Backend-Naht kennt (D3). Meilensteine: **M-A** Layer funktioniert gegen den Fake (G4, nach etwa 14 PT), **M-B** Echtbetrieb mit Modell (G5 mit IP1), **M-C** betriebsreif (G6), **M-D** abgenommen (G7).

## 7. Gates

| Gate | Kriterien |
|---|---|
| G0 | R1 bis R10 ausgefüllt; blockierende Entscheidungen bestätigt; Messwerte M1 bis M6 und Budgets B1 bis B12 festgeschrieben; Fixtures und Gold-Set eingecheckt; Basis-Tests in allen Feature-Kombinationen grün |
| G1 | `ResidentRuntime::start` liefert ein lauffähiges CPU-`Resident`; Referenz-Cosinus mindestens 0,9999; Similarity gleich Brute Force (Toleranz 1e-3); Rolle zu Präfix getestet; kein Hängen bei Cancel, Timeout, Panic, Shutdown; `api_hygiene` und cfg-Prüfung grün |
| G2 | Layer registriert; `applies`, `skipped`-Pfade, Konfiguration, Fehler- und Statusmodell getestet; Backend-Naht, FakeBackend, Testkit vorhanden; `cargo test --features mock` grün; Bestand unverändert |
| G3 | Erstaufbau und Inkremente korrekt und idempotent; Crash- und Abbruchfestigkeit; Schemawechsel führt zu kontrolliertem Neuaufbau; zweiter Schreiber läuft read-only; B4 und B6 erfüllt |
| G4 | Query-Pfad Ende zu Ende mit Fake; Scope, Filter, Staleness, Budgets, Fehlerpfade getestet; Ergebnisformat stabil und `MergeLayer`-kompatibel; B10 erfüllt |
| G5 | Fusion und Isolation nachgewiesen; CPU-Budget gemessen und dokumentiert; Doku vollständig; IP1 bestanden (B11, B12 oder bestätigte Abweichung); bestehende CLI-Aufrufe unverändert |
| G6 | Degradationsmatrix 13 von 13 getestet; Limits wirksam; Versionierung getestet; Telemetrie und Status vorhanden; Sicherheitstests grün; Benchmarks innerhalb der Budgets |
| G7 | AC1 bis AC16 erfüllt oder mit bestätigter Abweichung; Testmatrix und CI-Matrix vollständig, dreifacher Lauf ohne flackernde Tests; Definition of Done abgehakt |

## 8. Paketverzeichnis

Das vollständige Paketschema (Zweck, Quellen, Abhängt von, Zu klären, Ergebnis) steht je Paket in der genannten Stufendatei.

### Stufe 0 — `plan_01_10_s0_baseline.md`

| ID | Titel | Abhängt von | Ergebnis |
|---|---|---|---|
| L0.1 | Arbeitsbaum- und Befundverifikation | — | Tabelle R1 bis R10, `[Δ]`-Markierungen |
| L0.2 | Entscheidungsfreigabe | L0.1 | D1 bis D20 bestätigt oder geändert |
| L0.3 | Messbasis | L0.1 | Messwerte M1 bis M6, Mess-Gates |
| L0.4 | Fixture-Korpus und Gold-Set | L0.2 | Korpus, Gold-Set, `TmpCorpus` |
| L0.5 | Budgets und Schwellen | L0.3 | Budgets B1 bis B12 |

### Stufe 1 — `plan_01_11_s1_runtime.md`

| ID | Titel | Abhängt von | Ergebnis |
|---|---|---|---|
| L1.1 | IndexDirectory und Fassaden-Verdrahtung | L0.1 | `IndexId` wird aufgelöst |
| L1.2 | Rolle zu Präfix und Pipeline-Konsistenz | L0.1 | `TextRole` bis zum Tokenizer, konsistente Pipeline |
| L1.3 | Ressourcen: Tokenizer- und Encoder-Zellen | L0.1 | lazy Laden, TTL, Backoff |
| L1.4 | CpuEngine: Op-Ausführung | L1.1, L1.2, L1.3, L1.5 | `impl Engine for CpuEngine` |
| L1.5 | Scorer-Brücke: Index, Scope und Payload | L1.1 | blockweises Cosine-Top-k mit Payload |
| L1.6 | ResidentRuntime (Zusammenbau) | L1.1 bis L1.5 | `ResidentRuntime::start` |
| L1.7 | Abnahme-Tests der Runtime | L1.1 bis L1.6 | Referenz- und Brute-Force-Tests |

### Stufe 2 — `plan_01_12_s2_skeleton.md`

| ID | Titel | Abhängt von | Ergebnis |
|---|---|---|---|
| L2.1 | Modulgerüst und Registrierung | L0.2 | Modul, Registry-Eintrag |
| L2.2 | Konfiguration | L2.1, L0.5 | `ResidentLayerConfig` |
| L2.3 | Fehler- und Statusmodell | L2.1 | `ResidentLayerError`, Detail-Schema, Grundtabelle |
| L2.4 | Backend-Naht | L2.3 | `SemanticBackend`, `ResidentBackend` |
| L2.5 | FakeBackend und Testkit | L2.4, L0.4 | `FakeBackend`, `TmpCorpus` |
| L2.6 | Layer-Skelett | L2.2 bis L2.5 | lauffähiger Layer mit `skipped`-Pfaden |

### Stufe 3 — `plan_01_13_s3_indexing.md`

| ID | Titel | Abhängt von | Ergebnis |
|---|---|---|---|
| L3.1 | Dateiermittlung und Eignung | L2.1 | `visible_files`, `eligible` |
| L3.2 | Textzugriff und Offsets | L3.1 | `read_text`, `OffsetMap` |
| L3.3 | Chunker | L3.2, L0.3 | `LineAwareChunker` |
| L3.4 | Katalog und EntryId-Schema | L3.2, L3.3 | `Catalog`, `entry_id` |
| L3.5 | IndexStore | L3.4, L2.4 | `IndexStore` mit Sperre und Neuaufbau |
| L3.6 | Embedding-Pipeline | L2.4, L3.3, L3.5 | `EmbedPipeline` |
| L3.7 | Indexierungstreiber und Wartungsmodi | L3.1 bis L3.6 | `maintain`, Background |
| L3.8 | Tests und Messung | L3.1 bis L3.7, L2.5 | Suite, B4, B6 |

### Stufe 4 — `plan_01_14_s4_query.md`

| ID | Titel | Abhängt von | Ergebnis |
|---|---|---|---|
| L4.1 | Query-Optionen und Validierung | L2.2, L2.6 | `QueryOptions` |
| L4.2 | Sichtbarkeit und Scope | L3.1, L3.4, L4.1 | `ScopePlan` |
| L4.3 | Embedding und Similarity | L2.4, L4.2 | Budgets, Fehlerabbildung |
| L4.4 | Trefferabbildung | L3.2, L4.3 | `FileHit` mit Zeilen und Excerpt |
| L4.5 | Ergebnisse und Status | L4.4 | `ResultEntry`, Status |
| L4.6 | Tests der Stufe | L4.1 bis L4.5 | E2E-Suite, B10 |

### Stufe 5 — `plan_01_15_s5_integration.md`

| ID | Titel | Abhängt von | Ergebnis |
|---|---|---|---|
| L5.1 | CLI, Registrierung und Hilfetext | L2.1, L4.5 | Hilfetext, Prozess-Test |
| L5.2 | Fusion mit grep, trigram, glob und merge | L4.5 | Kompatibilitäts- und Isolationstests |
| L5.3 | Threading und CPU-Budget | L1.6, L4.5 | Standardwerte, Mess-Gate |
| L5.4 | Dokumentation | L5.1 bis L5.3 | `resident_layer.md` |
| L5.5 | Integrationspunkt IP1 | G1, G4 | Echtbackend-Tests, B11, B12 |

### Stufe 6 — `plan_01_16_s6_hardening.md`

| ID | Titel | Abhängt von | Ergebnis |
|---|---|---|---|
| L6.1 | Degradationsmatrix und Fehlerpfade | G5 | 13 Tests `degrade_*` |
| L6.2 | Ressourcenlimits | L3.x, L4.x | Limits mit Statusanzeige |
| L6.3 | Migration und Versionierung | L3.4, L3.5 | Versionsmatrix, `migrate`-Haken |
| L6.4 | Telemetrie und Diagnose | G5 | Spans, Ereignisse, `status`-Ausgabe |
| L6.5 | Sicherheit und Robustheit des Dateizugriffs | L3.1, L3.2, L4.4 | Pfadvalidierung, Dateityp-Prüfung |
| L6.6 | Leistungsmessung und Tuning | G5 | Benchmarks, Sharding-Entscheidung |

### Stufe 7 — `plan_01_17_s7_verification.md`

| ID | Titel | Abhängt von | Ergebnis |
|---|---|---|---|
| L7.1 | Testmatrix und Abdeckung | G6 | Rückverfolgungsliste |
| L7.2 | CI-Matrix | L7.1 | erweiterte Workflows |
| L7.3 | Abnahmekriterien | L7.1, L7.2 | AC1 bis AC16 |
| L7.4 | Definition of Done und Übergabe | L7.3 | Checkliste, Übergabevermerk |

## 9. Rückverfolgbarkeit zur Metaplanung

| Metaplan | Bezug | Plan-01-Pakete |
|---|---|---|
| A1′ Scope-Grenze Engine und Suchschicht | `scope_charta.md` | LP1, L2.4, L3.3, L4.x, D13 |
| A2 Projekt- und Modulstruktur | neues Modul `resident/runtime`, `layers/resident_layer` | L1.6, L2.1 |
| A3 Stack- und Versionskompatibilität | Toolchain, Abhängigkeiten | L0.1, E48 |
| A4 und Q1 Fehler-Taxonomie | thiserror, `IsRetryable` | L2.3, L6.1 |
| Q2 Logging | tracing, Feldnamen | L6.4 |
| Q3 Teststrategie und Mocks | Fakes, Toleranzen | L2.5, L7.1 |
| Q4 Feature-Flags | kein `cfg`-Streuen | LP9, D17, L7.2 |
| B4 Index-Trait und -Vertrag | `Index`, `ParquetIndex`, Payload | L1.5, L3.5 |
| B6, F Capability und Profile | Precision-Default | D18, A15 |
| C1 Fassade `Resident` | Builder, `submit_*` | L1.6, L2.4 |
| C2 Processor | `Engine`, `Hub` | L1.4 |
| C3 Planner | `PlannerExpander`, Ketten | L1.2, L1.6 |
| D1, D2 Lifecycle, Lane und Exec | CPU-Lane, Ressourcenzellen | L1.3, L1.4 |
| D4 Modell-Ressourcen (Encoder) | `LoadedEncoder`, Registry | L1.3 |
| D5 Tokenizer und Normalizer | `SharedTokenizer` | L1.2, L1.3 |
| D6 Kernels und Scorer | `CosineSlots`, `TopKScorer` | L1.5 |
| D7, D8 Index-Segmente, CPU und GPU | CPU-Pfad zuerst | L1.5, A1 |
| E1, E2 Manager und Scope | `CpuIndexManager`, `ResolverRegistry` | L1.5, L4.2 |
| E7 Residency und Ladestrategie | Lazy und TTL | L1.3, A1 |
| E9 Addons | Kaskaden gehören in die Suchschicht | A3, A4, A6 |
| G1 e5-small | `ModelSpec::e5_small` | L1.6, L0.3 |
| G2 Index-Schema | `SchemaHeader`, Parquet | L3.5 |
| G3 Similarity und Scorer-Konfiguration | Cosine, Top-k | L1.5, L4.3 |
| H1 bis H4 Ausblick | Ringbuffer, gleitende Indizierung, Funnel und Reranker, Graph-Capture | `plan_01_99_outlook.md` |

## 10. Risiken

| ID | Risiko | Wahrscheinlichkeit | Auswirkung | Gegenmaßnahme | Paket |
|---|---|---|---|---|---|
| R1 | Die Engine-Lücke (S1) ist größer als geschätzt und bildet den kritischen Pfad | hoch | hoch | `FakeBackend` entkoppelt S2 bis S4; früher Prototyp des Async-Init; Verdrahtungsprüfung R1 bis R4; Zeitpuffer | L0.1, S1 |
| R2 | One-Shot-CLI trägt je Anfrage Kaltstartkosten (Modell, Tokenizer, Index-Dekodierung, 1 bis 4 s [W]) | hoch | mittel | lazy Start, `verify=false`, Budget B1, Hinweis in der Doku, Daemon als Ausblick | L2.6, L4.3, A2 |
| R3 | Parquet-Eigenschaften: Flush O(N), `verify` per Default, Volldekodierung beim ersten Zugriff | mittel | mittel | Flush-Politik, `verify=false`, Messung M4, Sharding-Auslöser | L3.5, L6.6 |
| R4 | CPU-Überbuchung durch drei Pools (`CpuExecutor`, Resident-Worker, Candle/rayon) | mittel | mittel | Startwerte D15, Messung, sequenzieller Modus | L5.3 |
| R5 | Retrieval-Qualität bei Code und kurzen Anfragen unter Erwartung (multilingual-e5-small) | mittel | mittel | Gold-Set mit ehrlichen Schwellen, Chunking-A/B, Kombination mit grep über `MergeLayer`, Ausblick A3 und A7 | L5.5 |
| R6 | Stille Kürzung auf 512 Token | mittel | mittel | konservative Chunkgrößen, Kalibrierung M3, Test `chunks_stay_below_512_tokens` | L3.3 |
| R7 | Index- und Modell-Drift (Präfix, Tokenizer, Revision) | mittel | hoch | `SchemaHeader`, Fingerabdruck, Neuaufbau mit Quarantäne, Versionsmatrix | L3.5, L6.3 |
| R8 | Konkurrierende Prozesse beschädigen oder überschreiben den Index | mittel | hoch | Advisory-Lock, atomarer Austausch, read-only-Rückfall | L3.5 |
| R9 | Erstindexierung dauert Minuten pro MB auf der CPU | hoch | mittel | explizites `update`, Auto-Budget, Fortschrittsmeldung, Priorisierung, Messung M2 | L3.7, L0.3 |
| R10 | Offsets (Unicode, CRLF, BOM) führen zu falschen Zeilen | mittel | mittel | `OffsetMap`, Property-Tests, Fixtures mit Mehrbyte-Zeichen | L3.2, L4.4 |
| R11 | Plattenbedarf und Wachstum des Index | niedrig | mittel | Limits, Cap, Statusanzeige, Bereinigung der Quarantäne | L6.2 |
| R12 | Tests ohne Modell sind nicht aussagekräftig, CI ohne Modell | mittel | mittel | `FakeBackend`, Job `candle-model`, klare Skip-Meldungen | L7.2 |
| R13 | Verstöße gegen Konventionen (`cfg`-Streuung, API-Hygiene) | niedrig | mittel | Prüfungen in der CI, D17 | L7.2 |
| R14 | Dokumentationsdrift: Metaplan-Paketdetails und `resident_*.md` nicht gelesen | mittel | mittel | Abgleich in L0.1, Annahmen als [?] markiert, Charta als Anker | L0.1 |

## 11. Offene Punkte

- O1 — Einheit von `Payload.char_start` und `char_end` (E37); Empfehlung Unicode-Skalarwerte, in L3.2 per Test festschreiben.
- O2 — geklärt: `ParquetIndex::capabilities()` ist `[Range]`, `Scope::Set` und `Scope::All` werden direkt aufgelöst (E40).
- O3 — Datenvertrag zwischen `Similarity`- und `TopK`-Op (E30); in L1.4 festlegen.
- O4 — weitgehend geklärt: `ResourceCell` und `InitLimiter` nutzen nur `tokio::sync` und sind ohne Runtime pollbar (E55); `init` darf keine Tokio-I/O oder -Timer verwenden; ein Prototyp in L1.3 bestätigt es.
- O5 — geklärt: Symlinks, FIFOs, Sockets und Geräte erscheinen im Listing als „Dateien“, Verzeichnis-Symlinks werden nicht traversiert (E52); Dateityp-Prüfung vor dem Lesen in L3.2 und L6.5.
- O6 — Ort und Inhalt von `scripts/check_cfg_scatter.sh` (E44); in L0.1.
- O7 — Rust-Toolchain-Version (für `File::try_lock`, D16); in L0.1.
- O8 — Metaplan-Paketdetails und Konzeptdokumente `resident_*.md` (R14); Abgleich in L0.1.
- O9 — `candle_rayon_threads` wird im Quelltext nirgends ausgewertet (E50); Anwendung über einen dedizierten Pool in L1.3, Wirkung auf den globalen rayon-Pool in L1.3 und L5.3 zu prüfen.
- O10 — geklärt (E53): `layers[]` mit `layer_id`, `stage`, `ran`, `skipped`, `aborted`, `contributions`, `detail`; `Lines`-Schlüssel als sortierte ganze Zahlen.
- O11 — Bestehende CI-Jobs in `.github/workflows/rag.yml` und Bezug von Modellgewichten in der CI; in L0.1 und L7.2.

## 12. Fortschritt

Diese Tabelle wird von jedem Folgelauf aktualisiert (Status: offen, in Arbeit, bestanden, abgewichen).

| Stufe | Status | Gate | Datum | Notiz |
|---|---|---|---|---|
| S0 | offen | G0 | | |
| S1 | offen | G1 | | |
| S2 | offen | G2 | | |
| S3 | offen | G3 | | |
| S4 | offen | G4 | | |
| S5 | offen | G5 | | |
| S6 | offen | G6 | | |
| S7 | offen | G7 | | |

## 13. Arbeitsweise für Folgeläufe

Ein Lauf bearbeitet genau eine Stufe in der Reihenfolge der Abhängigkeiten (S1 und der Strang S2 bis S4 dürfen parallel laufen). Er liest `plan_01_00_overview.md`, die Stufendatei und die dort genannten Quellen, setzt die Pakete der Stufe in der angegebenen Reihenfolge um, führt die genannten Checks aus und trägt das Ergebnis in die Fortschrittstabelle ein. Abweichungen vom Plan werden nicht stillschweigend eingebaut: Sie erscheinen als `[Δ]` im Befundbericht oder als geänderte Entscheidung im Register, mit Begründung. Code und Kommentare sind englisch, Kommentare nur dort, wo das Warum nicht aus dem Code hervorgeht; Plantexte bleiben deutsch.
