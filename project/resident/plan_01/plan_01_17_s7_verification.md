# Stufe 7 — Verifikation, CI und Abnahme

Stufe 7 fügt die Prüfungen aller Stufen zusammen: Testmatrix über alle Ebenen, CI-Matrix, Abnahmekriterien und die Definition of Done. Sie schreibt keine neue Funktion, sondern beweist die vorhandene. Aufwand [W]: 3 Personentage.

## Eingang

Gate G6 bestanden. Die Tests der Stufen S1 bis S6 existieren; die Fixtures aus L0.4 und die Budgets aus L0.5 sind festgeschrieben.

## Pakete

### L7.1 · Testmatrix und Abdeckung

**Zweck:** Jede Anforderung hat mindestens einen automatisierten Nachweis, und jede Testebene hat einen definierten Ort, eine Voraussetzung und einen Lauf.  
**Quellen:** Konventionen Q3 (Testdoubles, Toleranzen, Modellreferenzen), `tests/*`, die benannten Tests der Stufen S1 bis S6.  
**Abhängt von:** G6.  
**Zu klären:** Messung der Testabdeckung (kein Coverage-Werkzeug im Repository bekannt, **[?]**); stattdessen Rückverfolgung „Anforderung zu Test“.  
**Ergebnis:** Tabelle „Testebenen“ und Rückverfolgungsliste (Entscheidungen und Abnahmekriterien zu Tests).

#### Testebenen

| Ebene | Ort | Voraussetzung | Lauf | Stufe |
|---|---|---|---|---|
| Unit Layer | `rag/src/layers/resident_layer/test/*` | keine | `cargo test` | S2 bis S6 |
| Unit Runtime | `rag/src/resident/runtime/test/*` | modellfrei; Modelltests mit Modell | `cargo test`, `cargo test --features candle` | S1 |
| Ende zu Ende mit Fake | `rag/tests/resident_layer_e2e.rs` | Feature `mock` | `cargo test --features mock` | S2 bis S6 |
| Globaler Filter | `rag/tests/resident_layer_filter.rs` | Feature `mock`, eigener Prozess | `cargo test --features mock --test resident_layer_filter` | S4 |
| Runtime-Integration | `rag/tests/resident_runtime.rs` | Feature `candle`, Modell | `cargo test --features candle --test resident_runtime` | S1 |
| Modellgestützt | `rag/tests/resident_layer_model.rs` | Features `candle` und `mock`, Modell | `cargo test --features candle,mock --test resident_layer_model` | S5 |
| CLI und Prozess | `rag/tests/resident_layer_cli.rs` | keine | `cargo test --test resident_layer_cli` | S5 |
| Property | in `chunk`, `text`, `catalog` (proptest) | keine | `cargo test` | S3 |
| Messung und Gates | `#[ignore]`-Tests `gate_resident_*`, Benchmarks | Modell, ruhige Maschine | `cargo test -- --ignored gate_resident`, `cargo bench` | S0, S5, S6 |
| Regression Bestand | `trigram_e2e`, `api_hygiene`, `deps_smoke`, Engine-Tests | keine | `cargo test` | alle |

#### Aufgaben

- Rückverfolgungsliste anlegen: Spalten „Entscheidung oder AC“ und „namentliche Tests“; jede Entscheidung D1 bis D20 und jedes Abnahmekriterium AC1 bis AC16 hat mindestens einen Test.
- Test-Namenskonvention prüfen und vereinheitlichen (beschreibende Verben, kein Bezug auf Interna).
- Flackernde Tests identifizieren: Zeitabhängigkeit über `tokio::time::pause` oder `FakeClock` ersetzen (Q3); drei Wiederholungsläufe der gesamten Layer-Suite ohne Abweichung.
- Modelltests, die ohne Modell nicht laufen können, mit klarer Skip-Meldung versehen (nicht stillschweigend grün): Ausgabe „skipped: model not installed at …“.

#### Abnahme L7.1

- Rückverfolgungsliste vollständig, keine Entscheidung und kein AC ohne Test.
- Drei aufeinanderfolgende Gesamtläufe der Matrix ohne flackernde Tests.

### L7.2 · CI-Matrix

**Zweck:** Alle Testebenen laufen automatisch dort, wo ihre Voraussetzungen erfüllt sind, und die Konventionsprüfungen sind Teil der Pipeline.  
**Quellen:** `rag/docs/architecture/feature_matrix.md` (CI definiert in `.github/workflows/rag.yml` im Repository-Wurzelverzeichnis; E44), `rag/model/README.md` (der Referenztest scheitert, wenn kein Modell installiert ist, E33).  
**Abhängt von:** L7.1.  
**Zu klären:** **[?]** bestehende Jobs in `.github/workflows/rag.yml` (in L0.1 lesen und erweitern statt neu schreiben); Beschaffung des Modells in der CI (Cache-Artefakt, selbst gehosteter Runner oder manueller Job); GPU-Job (`cuda`) bleibt Ausblick.  
**Ergebnis:** Erweiterte Workflow-Definition und aktualisiertes `feature_matrix.md`.

#### Jobs

| Job | Befehle | Inhalt |
|---|---|---|
| `default` | `cargo build`, `cargo test`, `scripts/check_cfg_scatter.sh` | Unit Layer, Property, CLI, Regression, Konventionsprüfung |
| `mock` | `cargo test --features mock` | Ende zu Ende mit Fake, Filtertest |
| `candle-model` | `cargo test --features candle,mock` mit installiertem Modell | Runtime, Modellgestützt, Referenz |
| `bench-smoke` | `cargo bench --bench resident_layer_bench -- --test` | Benchmarks bauen und kurz laufen |
| `lint` | `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings` (falls im Repository üblich, **[?]**) | Stil |
| `gates` (manuell oder nightly) | `cargo test --features candle -- --ignored gate_resident` | Messwerte M1 bis M6, Contention |
| `gpu` (Ausblick) | `cargo test --features cuda -- --ignored` | nicht Teil von Plan 01 |

#### Aufgaben

- Bestehende Workflows lesen und die Jobs `mock`, `candle-model`, `bench-smoke` ergänzen; das Modell als Cache-Artefakt oder Runner-Ressource bereitstellen und den Pfad (`XYRAG_RESIDENT_MODEL_DIR` oder Fallback `rag/model/multilingual-e5-small`) setzen.
- Sicherstellen, dass `default` ohne Modell grün ist: modellabhängige Tests der neuen Suiten dürfen nicht scheitern, sondern melden „skipped“. Das bestehende Verhalten des Referenztests (scheitert ohne Modell) bleibt unverändert und ist ein bekannter Unterschied.
- `feature_matrix.md` um die neuen Jobs ergänzen.

#### Abnahme L7.2

- Alle Jobs der Tabelle laufen in der CI oder sind mit Begründung als manuell markiert; `default` ist ohne Modell grün.

### L7.3 · Abnahmekriterien

**Zweck:** Eine abschließende Liste messbarer Kriterien, an der „fertig“ entschieden wird.  
**Quellen:** Gates G0 bis G6, Budgets B1 bis B12, Entscheidungen D1 bis D20, Leitplanken LP1 bis LP13.  
**Abhängt von:** L7.1, L7.2.  
**Zu klären:** Endgültige Schwellen B11 und B12 nach IP1; Umgang mit bewusst akzeptierten Abweichungen (immer mit Entscheidung und Datum).  
**Ergebnis:** Tabelle AC1 bis AC16 mit Nachweis und Status.

#### Abnahmekriterien

| ID | Kriterium | Nachweis | Bezug |
|---|---|---|---|
| AC1 | Der Layer aktiviert sich nur mit dem Feld `resident`; bestehende Aufrufe sind unverändert | `applies_only_with_resident_field`, `cli_unrelated_queries_unchanged` | D1 |
| AC2 | Ohne Modell oder Backend meldet er `skipped` mit Grund und startet keine Threads | `missing_model_is_skipped_with_reason`, `factory_is_not_called_without_resident_field` | LP3, D11, D17 |
| AC3 | Erstaufbau und Inkremente (neu, geändert, gelöscht, umbenannt, berührt) sind korrekt und idempotent | Tests aus L3.8 | D5, D7, D8 |
| AC4 | Abbruch, Absturz und zweiter Prozess führen nicht zu inkonsistentem Index | `crash_between_flush_and_catalog_commit_recovers`, `second_writer_runs_read_only`, `cancel_mid_run_keeps_last_flush_consistent` | D6, D16 |
| AC5 | Ergebnis `File`, `Lines`, `Resident` ist stabil, `MergeLayer`-kompatibel und zeilengenau (Unicode, CRLF) | L4.4, L4.5, L5.2 | D10 |
| AC6 | `directory`, globaler Filter und veraltete Dateien wirken korrekt auf den Scope | L4.2, `resident_layer_filter` | D8, D20 |
| AC7 | Timeout, Engine-Fehler und Cancel werden in `LayerStatus` abgebildet; andere Layer bleiben unberührt | L4.3, `resident_failure_does_not_affect_other_layers` | LP3 |
| AC8 | Degradationsmatrix 13 von 13 getestet | L6.1 | LP3 |
| AC9 | Limits wirken und sind im Status sichtbar | L6.2 | — |
| AC10 | Versionierung und Migration wie in der Matrix, ältere Version zerstört keinen neueren Katalog | L6.3 | LP12 |
| AC11 | Pfadvalidierung, Dateityp-Prüfung, Denylist | L6.5 | — |
| AC12 | Leistung: B1 bis B9 erfüllt oder mit Entscheidung abgewichen | L0.3, L5.5, L6.6 | — |
| AC13 | Qualität: B10 (Fake), B11 und B12 (e5) erreicht oder mit Entscheidung abgewichen | L4.6, L5.5 | D4, D19 |
| AC14 | Engine: Referenz-Cosinus mindestens 0,9999, Top-k gleich Brute Force, Rolle zu Präfix | G1, L1.7 | D12, D13 |
| AC15 | Konventionen: thiserror und `non_exhaustive`, tracing-Felder, kein neues `cfg(feature)`, `api_hygiene`, `fmt` und `clippy` | CI-Jobs `default`, `lint` | LP8 bis LP11 |
| AC16 | Dokumentation vollständig, Verweise in Charta, Konventionen und Feature-Matrix gesetzt | L5.4 | — |

#### Abnahme L7.3

- Jede Zeile trägt Status (erfüllt, abgewichen mit Entscheidung, offen) und Datum.

### L7.4 · Definition of Done und Übergabe

**Zweck:** Formaler Abschluss mit nachvollziehbarer Übergabe.  
**Quellen:** Alle Gates, `plan_01_00_overview.md` (Fortschrittstabelle), Repository-Konventionen.  
**Abhängt von:** L7.3.  
**Zu klären:** Ort von Changelog oder TODO nach Repository-Konvention (**[?]**; `rag/TODO.md` wurde in der Analyse nicht gelesen).  
**Ergebnis:** Abgehakte Liste und Übergabevermerk.

#### Checkliste

- Gates G0 bis G7 erteilt, Fortschrittstabelle im Masterplan aktualisiert.
- Entscheidungen D1 bis D20 bestätigt oder geändert, Abweichungen dokumentiert.
- Alle CI-Jobs der Matrix grün (oder manuell mit Protokoll).
- Keine `todo!()`, `unimplemented!()`, `dbg!()` und keine ungenutzten Test-Nahten außerhalb von `cfg(any(test, feature = "mock"))` im Layer-Code.
- Messwerte (M1 bis M6, Benchmarks, Contention) und Budgets in der Doku; Beispiele geprüft.
- Offene Punkte O1 bis O11 erledigt oder in Folgepläne überführt.
- Übergabevermerk (Zusammenfassung, bekannte Grenzen, Folgepläne) im Changelog oder TODO nach Repository-Konvention.
- Folgepläne angelegt, soweit ausgelöst: Sharding (D6), GPU-Pfad, Rerank, Daemon-Modus, SharedIndex-Übernahme (siehe `plan_01_99_outlook.md`).

## Gate G7

- AC1 bis AC16 erfüllt oder mit bestätigter Abweichung abgeschlossen.
- Testmatrix und CI-Matrix vollständig, dreifacher Lauf ohne flackernde Tests.
- Definition of Done vollständig abgehakt.

## Risiken und Stolpersteine

- Ohne Modell in der CI bleiben die wichtigsten Qualitätsnachweise manuell; das Risiko ist gering, wenn der Job `candle-model` verbindlich wird, hoch, wenn er optional bleibt. Als Entscheidung festhalten.
- Qualitätsschwellen (B11, B12) sind empirisch; zu strenge Schwellen erzeugen flackernde Tests, zu lasche entwerten sie. Schwellen mit Reserve gegen die gemessene Basislinie festlegen und im Dokument begründen.
- Der Wartungsaufwand für den Fixture-Korpus ist nicht null: Änderungen am Chunker verändern erwartete Zeilenbereiche; Golden-Dateien daher mit Regenerierungsskript und Review-Hinweis pflegen.
