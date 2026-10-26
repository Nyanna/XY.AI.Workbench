# Stufe 5 — Integration, Fusion, CPU-Budget, Dokumentation, IP1

Stufe 5 bindet den Layer in die Gesamtumgebung ein: CLI-Verhalten und Hilfetext, Zusammenspiel mit grep, trigram, glob und merge, CPU-Budget zwischen Layer-Pool und Engine-Workern, Dokumentation und — als Integrationspunkt IP1 — die Umstellung auf das echte Backend samt Wiederholung der S3/S4-Tests mit dem Modell. Aufwand [W]: 2 bis 3 Personentage plus Messläufe.

## Eingang

Gate G4 (Layer gegen `FakeBackend`) bestanden. Für IP1 (L5.5) zusätzlich Gate G1 (Runtime). L5.1 bis L5.4 können ohne S1 abgeschlossen werden; ohne G1 bleibt L5.5 offen und G5 kann nicht erteilt werden.

## Pakete

### L5.1 · CLI, Registrierung und Hilfetext

**Zweck:** Der Layer ist über das CLI so benutzbar, dass Nutzer Felder und Verhalten entdecken, ohne bestehende Aufrufe zu verändern.  
**Quellen:** `bin/xyrag.rs` (`Cli`, `parse_query`, `build_default_registry`, `main`, `status_to_json`, `numeric_keys_for_lines`; E5, E6), D1.  
**Abhängt von:** L2.1, L4.5.  
**Zu klären:** Ob `after_help` im `Cli`-Attribut oder die Dokumentation reicht; Ausgabeformat ist bekannt (E53): YAML mit `results` und `layers[]` (`layer_id`, `stage`, `ran`, `skipped`, `aborted`, `contributions`, `detail`); Prozessende-Zeit bei aktivem Layer.  
**Ergebnis:** `--help` nennt die Felder `resident`, `residentIndex`, `residentTopK`, `residentMinScore` samt Beispielen; Prozess-Test des Binaries.

#### Aufgaben

- `after_help` (oder `long_about`) in `Cli` um einen kurzen Abschnitt „Semantic search (resident layer)“ mit den Feldern und drei Beispielen ergänzen; keine Verhaltensänderung bestehender Optionen.
- Prüfen, dass `--max-results` (setzt `maxResults`) auch für `resident` wirkt und im Hilfetext als layerübergreifend beschrieben ist.
- Ausgabe prüfen: `layers[]` enthält die Statuszeile `resident` mit `detail`; `Lines`-Schlüssel werden wie bei den anderen Layern numerisch ausgegeben (`numeric_keys_for_lines`).
- Prozess-Test mit `CARGO_BIN_EXE_xyrag`: ohne Modell liefert `resident=x` `skipped / model_unavailable` und das Binary endet innerhalb von zwei Sekunden (Background ist ein No-op).

#### Abnahme L5.1

- Tests `cli_help_lists_resident_fields`, `cli_without_model_reports_skipped_and_exits_fast`, `cli_unrelated_queries_unchanged` (Ausgabe von `query=x` identisch mit der Baseline aus L0.1).

### L5.2 · Fusion mit grep, trigram, glob und merge

**Zweck:** Zeigen, dass die gemeinsame Topologie funktioniert und der Layer keine anderen Layer stört.  
**Quellen:** `core/topology.rs::build_levels` (E3), `layers/merge_layer.rs` (E12), `core/engine.rs` (Parallel und Sequential).  
**Abhängt von:** L4.5.  
**Zu klären:** Reihenfolge der Einträge bei paralleler Ausführung (nicht deterministisch zwischen Layern); ob ein späterer Postprocess-Layer die Reihenfolge herstellt (Ausblick).  
**Ergebnis:** Kompatibilitätstests und dokumentierte Reihenfolge-Semantik.

#### Aufgaben

- Test mit allen Layern aktiv (`query=...`, `resident=...`) in den Modi `parallel` und `sequential`: gleiche Menge von Dateieinträgen, ein Eintrag je Datei nach `merge`.
- `Lines` zweier Layer für dieselbe Zeile: der kürzere Text gewinnt (Regel des `MergeLayer`); das `Resident`-Objekt bleibt erhalten.
- Isolation: schlägt der `resident`-Layer fehl (Fault-Skript), bleiben Ergebnisse und Status der anderen Layer unverändert.
- Topologie: `build_levels` ordnet `resident` auf die Ebene von grep, trigram und glob; `merge` folgt danach.

#### Abnahme L5.2

- Tests `merge_combines_grep_and_resident_for_same_file`, `parallel_and_sequential_produce_same_entry_set`, `resident_failure_does_not_affect_other_layers`, `resident_is_scheduled_with_other_generate_layers`.

### L5.3 · Threading und CPU-Budget

**Zweck:** Überbuchung der Kerne vermeiden und die Wahl der Thread-Zahlen belegen.  
**Quellen:** `core/executor.rs::CpuExecutor` (eigener rayon-Pool, E8), `processor/config.rs::WorkerConfig` (`candle_rayon_threads`, `cpu_nice`), `processor/worker.rs::Processor::start` (Nice nur mit GPU-Workern, E50), D15.  
**Abhängt von:** L1.6 (falls verfügbar), L4.5.  
**Zu klären:** **[?]** nutzt Candle den globalen rayon-Pool oder einen eigenen (Wirkung von `candle_rayon_threads`); Startwert abhängig vom Modus der ersten Anfrage; Verhalten bei Parallelbetrieb mit grep und trigram auf kleinen Rechnern.  
**Ergebnis:** Dokumentierte Standardwerte und ein Mess-Gate `gate_resident_contention`.

#### Aufgaben

- Startwert festlegen: `cpu_workers = 1`; `candle_rayon_threads = max(1, ncpu - 1)` wenn die erste Anfrage `update` oder `rebuild` ist, sonst `max(1, ncpu / 2)`; Override über `runtime.candle_threads` in der Konfiguration.
- Mess-Gate (`#[ignore]`, mit Modell): Szenario A nur `resident`, Szenario B `query` und `resident` gemeinsam im Modus `parallel`, Szenario C dasselbe im Modus `sequential`; erfasst Wandzeit und CPU-Auslastung.
- Akzeptanz: Szenario B benötigt höchstens das 1,3-fache der längsten Einzelzeit aus A und der grep-Zeit; sonst Standardwerte senken oder in der Doku die Sequenzielle Ausführung empfehlen.
- Hinweis in der Doku: Die Aktivierung per Feld isoliert die Layer weitgehend, weil grep und trigram `query` benötigen (D1).

#### Abnahme L5.3

- Messwerte und gewählte Standardwerte stehen in `docs/architecture/resident_layer.md`; die Entscheidung D15 ist bestätigt oder angepasst.

### L5.4 · Dokumentation

**Zweck:** Betrieb und Entwicklung des Layers dauerhaft beschreiben, im Stil der vorhandenen Architekturdokumente.  
**Quellen:** `rag/docs/architecture/*.md` (Muster: Legende [V]/[W]/[?], kurze Abschnitte), `scope_charta.md` (§7 Referenzen), `conventions.md` (Q3), `feature_matrix.md` (CI).  
**Abhängt von:** L5.1 bis L5.3.  
**Zu klären:** Welche Beispiele (Ausgabeformat) erst nach Prüfung von `status_to_json` verbindlich sind.  
**Ergebnis:** `rag/docs/architecture/resident_layer.md` und kleine Ergänzungen in drei bestehenden Dokumenten.

#### Gliederung

- Zweck, Abgrenzung zur Engine (Verweis auf die Charta) und zu grep und trigram.
- Aktivierung, Felder, Ausgabeformat (`File`, `Lines`, `Resident`), Beispiele.
- Index-Lebenszyklus: Erstaufbau, `auto`, `update`, `rebuild`, `status`, Dauer und Ressourcenbedarf (Messwerte aus L0.3).
- Konfiguration: Auflösungsreihenfolge, Schlüsseltabelle mit Defaults, Modellpfad.
- Statusfelder und Fehlerbilder (Grundtabelle aus L2.3) mit Handlungshinweisen.
- Betrieb: Parallelstart mehrerer Prozesse (Schreibsperre), Plattenbedarf, Neuaufbau bei Modellwechsel, Datenschutz (nur lokale Dateien, keine Netzwerkzugriffe).
- Entwicklung: Backend-Naht, `FakeBackend`, Testmatrix, Messungs-Gates.
- Grenzen und Ausblick (Verweis auf `plan_01_99_outlook.md`).

#### Abnahme L5.4

- Dokument vorhanden, Legende [V]/[W]/[?] wird verwendet, jede Zahl nennt ihre Quelle (Messung oder Konfiguration).
- `scope_charta.md` §7 verweist auf das neue Dokument; `feature_matrix.md` enthält die zusätzlichen CI-Jobs (L7.2); `conventions.md` Q3 nennt `FakeBackend`.

### L5.5 · Integrationspunkt IP1: echtes Backend

**Zweck:** Nachweis, dass der Layer mit dem echten CPU-Backend die S3/S4-Eigenschaften behält und die Qualitätsziele erreicht.  
**Quellen:** G1 (Runtime), Gold-Set (L0.4), Budgets B1, B2, B7, B11, B12 (L0.5), `tests/resident_layer_model.rs`.  
**Abhängt von:** G1, G4.  
**Zu klären:** Welche Szenarien doppelt gefahren werden (Fake und echt) und welche nur echt; Umgang mit nicht deterministischen Zeitwerten; Entscheidung über `context_header` und Chunkgrößen anhand der Gold-Set-Auswertung.  
**Ergebnis:** Modellgestützte Tests grün (oder dokumentierte Abweichung mit Entscheidung), Messwerte für Kaltstart und warme Anfrage, angepasste Defaults.

#### Aufgaben

- Tests nach Backend parametrisieren: `RESIDENT_TEST_BACKEND=fake` (Default) oder `resident` (Modell erforderlich); Szenarien Erstaufbau, inkrementelle Änderung, Löschen, Schemawechsel, Cancel/Timeout, Zwei-Prozess-Lauf.
- Gold-Set mit echtem Modell auswerten: Recall@5 getrennt nach `semantic` und `crosslingual` gegen B11 und B12; Fehlschläge einzeln analysieren (Chunking, Präfix, Kopfzeile).
- A/B-Läufe: `context_header` gleich `path` gegen `none`; `target_chars` 800, 1000, 1400; Ergebnis in D4 und L0.5 zurückschreiben.
- Messung Kaltstart (B1), warme Anfrage (B2), Zusatzspeicher (B7), Indexgröße (B8); Auto-Budget (B3) an der gemessenen Durchsatzrate ausrichten.
- Sanitätstest `prefixes_matter`: Mit entfernten Präfixen (Test-Spec) sinkt der Recall messbar, dient als Wachposten gegen eine kaputte Rollenweitergabe.

#### Abnahme L5.5

- `cargo test --features candle --test resident_layer_model` grün (mit Modell).
- B1, B2, B7, B8 erfüllt oder bewusst angepasst; B11 und B12 erreicht oder mit Entscheidung dokumentiert.
- Defaults in `plan_01_02_decisions.md` und `docs/architecture/resident_layer.md` entsprechen den Messungen.

## Gate G5

- Kompatibilität mit `MergeLayer`, Parallel- und Sequenziell-Modus nachgewiesen; Isolation bei Layer-Fehlern nachgewiesen.
- CPU-Budget gemessen, Standardwerte festgelegt und dokumentiert.
- Dokumentation vollständig (L5.4).
- IP1 bestanden: echtes Backend erfüllt die S3/S4-Tests und die Qualitätsziele B11 und B12 (oder dokumentierte, bestätigte Abweichung).
- CLI-Verhalten bestehender Aufrufe unverändert (Baseline aus L0.1).

## Risiken und Stolpersteine

- Die Qualitätsziele B11 und B12 sind Schätzungen ohne Messbasis; sie dürfen nach der ersten Auswertung angepasst werden, aber nur mit dokumentierter Begründung und Bestätigung (nicht stillschweigend herunterstufen).
- Ein A/B-Lauf auf einem Korpus von 40 Dateien ist statistisch dünn; Ergebnisse als Hinweise, nicht als Beweis behandeln und bei Gelegenheit am echten Repository gegenprüfen.
- Die Reihenfolge der Einträge zwischen Layern ist im Parallelmodus nicht deterministisch; Tests dürfen nur Mengen vergleichen, keine Reihenfolgen über Layergrenzen.
- Ohne installiertes Modell in der CI bleiben L5.5 und die Mess-Gates ungeprüft; die CI-Matrix (L7.2) muss einen Job mit Modell vorsehen oder die Prüfung ausdrücklich als manuellen Schritt markieren.
