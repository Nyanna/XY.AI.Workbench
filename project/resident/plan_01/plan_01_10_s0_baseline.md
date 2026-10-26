# Stufe 0 — Baseline, Verifikation, Entscheidungen, Messbasis

Stufe 0 friert den Ist-Stand ein: Sie verifiziert die Befunde des Berichts (`plan_01_01_evidence.md`) im aktuellen Arbeitsbaum, holt die blockierenden Entscheidungen (`plan_01_02_decisions.md`) ein, schafft die Messbasis und die Testfixtures für alle Folgestufen. Es entsteht kein Produktivcode. Aufwand [W]: etwa ein Personentag plus Messläufe.

## Eingang

Keine fachliche Voraussetzung. Benötigt werden eine Rust-Toolchain, Zugriff auf `/home/user/xyan/xy.ai.workbench/rag` und für die Modellmessungen das installierte Modell `rag/model/multilingual-e5-small` (Gewichte nicht in Git, E33). Fehlt das Modell, laufen M1 bis M3 nicht und werden in L0.5 als offen markiert.

## Pakete

### L0.1 · Arbeitsbaum- und Befundverifikation

**Zweck:** Bestätigen, dass die Befunde des Berichts im aktuellen Arbeitsbaum gelten; Abweichungen den betroffenen Paketen zuordnen.  
**Quellen:** `plan_01_01_evidence.md` (E1 bis E48, K1 bis K6), `git`, `cargo`, Konzeptdokumente `resident*.md`.  
**Abhängt von:** —  
**Zu klären:** Gibt es lokale Änderungen an `rag/src/resident/**`, insbesondere einen produktiven `Engine`-Zweig? Welche Toolchain-Version liegt vor (`File::try_lock`, D16)? Wo liegt `scripts/check_cfg_scatter.sh` (E44)? Sind die Konzeptdokumente (`resident_subengine.md`, `resident_processor.md`, `resident_precision.md` als aktuellste laut Metaplan-Vorspann) verfügbar, um Code und Konzept abzugleichen?  
**Ergebnis:** Tabelle R1 bis R10 mit tatsächlichem Ergebnis (Abschnitt „Ergebnisse“ unten), `[Δ]`-Markierungen im Befundbericht, Liste betroffener Pakete.

#### Aufgaben

- `git status --short` und `git log -5 --oneline` festhalten; ungetrackte Dateien unter `rag/src/resident` und `rag/src/layers` notieren.
- Toolchain festhalten (`rustc --version`, `cargo --version`); Edition 2021 ist gesetzt (E48).
- Basis-Build und Basis-Tests je Feature-Kombination ausführen (Befehle unten), Dauer und Ergebnis notieren; rote Tests vor Planbeginn klären.
- Readiness-Checks R1 bis R10 ausführen und das tatsächliche Ergebnis eintragen.
- Konzeptdokumente beschaffen und deren Aussagen zu Fassade, Index-Vertrag, Scope und Task-Matching gegen E18 bis E40 abgleichen; Abweichungen als `[Δ]` markieren und dem Paket zuordnen.
- Ort von `scripts/check_cfg_scatter.sh` ermitteln und den Aufruf festhalten.

#### Befehle

```bash
cd /home/user/xyan/xy.ai.workbench
git status --short
git log -5 --oneline
cd rag
rustc --version && cargo --version
cargo build
cargo test
cargo test --features mock
cargo test --features candle
cargo test --test api_hygiene
cargo test --test trigram_e2e
grep -rn "impl.*Engine for" src --include=*.rs
grep -rn "Resident::builder()" src --include=*.rs
grep -rn "IndexId" src/resident/processor src/resident/subengine src/resident/planner --include=*.rs
grep -rn "EncodeMode" src/resident --include=*.rs
grep -rn "shared_index\.\(upsert\|mark_deleted\)" src --include=*.rs
grep -rn "\"resident\"" src/layers src/core src/bin --include=*.rs
ls model/multilingual-e5-small/model.safetensors
ls src/layers
```

#### Readiness-Checks

| ID | Prüfung | Erwartung laut Analyse | Konsequenz bei Abweichung |
|---|---|---|---|
| R1 | `impl Engine for` außerhalb von `NoEngine` und Tests | nur `NoEngine`, `TestEngine`, `Scripted` | Treffer: S1 auf Verdrahtungsprüfung (L1.7) reduzieren, D12 anpassen |
| R2 | `Resident::builder()` außerhalb von Tests | nur in `facade/test/*` | Treffer: Zusammenbau-Code prüfen und wiederverwenden (L1.6) |
| R3 | `IndexId`-Auflösung in Processor, Subengine, Planner | keine (nur Typverwendung in Params und Requirements) | Treffer: L1.1 anpassen |
| R4 | `EncodeMode` oder Rollen-Weitergabe im Op-Pfad | `EncodeMode` nur in `tokenize/` und Tests | Treffer: L1.2 entfällt oder verkürzt sich |
| R5 | Befüllung von `SharedIndex` durch Layer oder Kern | keine Treffer | Treffer: D7 neu bewerten |
| R6 | Belegung des Namens `resident` in Layern und CLI | keine Treffer | Konflikt: D1 überarbeiten |
| R7 | Modell installiert (`model.safetensors`) | vorhanden | fehlt: Modelltests als `ignored` planen, Beschaffung klären |
| R8 | `installed_models_cpu_match_reference` mit Feature `candle` | grün | rot: S1 blockiert, Modellpfad klären |
| R9 | `api_hygiene`, `deps_smoke`, `check_cfg_scatter.sh` | grün | rot: Basis vor Planbeginn reparieren |
| R10 | Modul oder Verzeichnis `resident_layer` unter `src/layers` | nicht vorhanden | Konflikt: D2 |

#### Abnahme L0.1

- Tabelle R1 bis R10 ist vollständig ausgefüllt, kein Eintrag steht auf „offen“.
- Jede Abweichung ist im Befundbericht mit `[Δ]` markiert und einem Paket zugeordnet.
- Toolchain-Version, Feature-Testmatrix und Laufzeiten sind festgehalten.

### L0.2 · Entscheidungsfreigabe

**Zweck:** Die Entscheidungen D1 bis D20 bestätigen oder ändern; die blockierenden zuerst.  
**Quellen:** `plan_01_02_decisions.md`.  
**Abhängt von:** L0.1 (R-Ergebnisse beeinflussen D1, D2, D7, D12).  
**Zu klären:** Alle blockierenden Entscheidungen (D1, D2, D4, D5, D8, D9, D12) sowie Abweichungswünsche zu D3, D6, D7, D13.  
**Ergebnis:** Status je Entscheidung in `plan_01_02_decisions.md` auf „bestätigt“ oder „geändert“ (mit Begründung); Folgeänderungen in den betroffenen Paketen nachgezogen.

#### Aufgaben

- Reihenfolge der Freigabe: D12 (bestimmt die Ressourcen), D1 und D2 (Namen), D4 und D5 (Datenformate), D8 und D9 (Verhalten), dann die übrigen.
- Je geänderter Entscheidung die betroffenen Pakete aktualisieren (Spalte „Betrifft“ in den Entscheidungen).
- Nicht blockierende offene Entscheidungen mit Frist bis zum Start der betroffenen Stufe versehen.

#### Abnahme L0.2

- Alle blockierenden Entscheidungen tragen den Status „bestätigt“ oder „geändert“.
- Jede Änderung ist in den Paketen nachgezogen.

### L0.3 · Messbasis

**Zweck:** Messwerte für Budgets, Chunk-Größen und Flush-Politik gewinnen, bevor Parameter festgeschrieben werden.  
**Quellen:** `rag/src/resident/model/test/candle_adapter.rs` (`embed_with` und `gate_encoder_dtype_front` als Vorlage für Mess-Gates), `resident/tokenize/mod.rs`, `resident/index/parquet_io.rs`, `core/persistence.rs`, `layers/dir_cache.rs`.  
**Abhängt von:** L0.1.  
**Zu klären:** Messmaschine (Kerne, RAM, Datenträger), Seitencache-Zustand (kalt oder warm), Thread-Zahl (`candle_rayon_threads`).  
**Ergebnis:** Messtabelle M1 bis M6 (Abschnitt „Messwerte“ unten), abgeleitete Budgets (L0.5), Mess-Gates als `#[ignore]`-Tests `gate_resident_*` unter `rag/src/layers/resident_layer/test/gates.rs` (Vorbild `gate_encoder_dtype_front`).

#### Messungen

| ID | Messgröße | Methode | Ausgabe |
|---|---|---|---|
| M1 | Modell-Kaltstart | `ModelRegistry::with_defaults().load(&spec, DeviceKind::Cpu)` plus `SharedTokenizer::from_spec`, je 5 Läufe mit kaltem und warmem Seitencache | Millisekunden |
| M2 | Embedding-Durchsatz | Texte mit 128, 256, 384, 512 Token; Batch 1, 8, 16, 32; `candle_rayon_threads` 1, 2, 4, alle | Chunks je Sekunde, ms je Batch |
| M3 | Token je Zeichen | `SharedTokenizer::encode(EncodeMode::Passage, …)` über Repo-Korpus (rs, md, py, toml) und Fixtures | p50, p95, p99; Anteil über 512 Token bei Chunk-Größen 800, 1000, 1400, 1800 |
| M4 | Parquet-Skalierung | synthetische Zeilen 10k, 50k, 100k (Dim 384, f16): `upsert`-Rate, `flush`-Dauer, `open` mit und ohne `verify`, Brute-Force-Cosine je Anfrage | Zeiten, Dateigröße |
| M5 | LayerStorage-Schreibrate | 10k einzelne `put` gegen 256 Shard-`put` | Sekunden |
| M6 | Walk-Dauer | `DirCache`-Walk der Repo-Wurzel, kalt und warm | ms, Dateizahl |

#### Aufgaben

- Die sechs Messungen als `#[ignore]`-Tests mit stabilen Namen `gate_resident_m1` bis `gate_resident_m6` anlegen; Ausgabe als eine Zeile je Messpunkt (`name=value unit`) auf stderr.
- M1 bis M3 benutzen nur vorhandene Bausteine (Registry, Tokenizer, Encoder), nicht die Fassade; M2 folgt dem Muster von `embed_with` im Candle-Adapter-Test.
- M4 verwendet `ParquetIndex` direkt mit `SchemaHeader::from_spec(&ModelSpec::e5_small(Path::new("/nonexistent"), DType::F16), "gate", "gate")`.
- Messläufe auf der Zielmaschine ausführen, Ergebnisse in den Abschnitt „Messwerte“ eintragen, Maschine und Datum nennen.

#### Abnahme L0.3

- Alle sechs Gates laufen reproduzierbar (Streuung unter 15 Prozent bei drei Wiederholungen).
- Messwerte stehen in diesem Dokument; Schätzungen W1 bis W5 des Berichts sind bestätigt oder korrigiert.

### L0.4 · Fixture-Korpus und Gold-Set

**Zweck:** Gemeinsame Testdaten für S2 bis S7: ein kleiner mehrsprachiger Korpus und Anfragen mit erwarteten Treffern.  
**Quellen:** `rag/tests/trigram_e2e.rs` (Muster `TmpDirs`, `engine_for`, `query_for`), `rag/tests/data/reference/multilingual-e5-small.json`.  
**Abhängt von:** L0.2 (D4 und D8 beeinflussen die Randfälle).  
**Zu klären:** Sprachmix, Umfang, Inhalte selbst verfasst (Lizenz), getrennte Gold-Sets für den Fake (lexikalisch) und das echte Modell (semantisch).  
**Ergebnis:** `rag/tests/data/resident_corpus/` (etwa 40 Dateien, unter 300 KB), `rag/tests/data/resident_gold.json`, Hilfsfunktion `TmpCorpus::with_edge_files()` für zur Laufzeit erzeugte Sonderfälle.

#### Korpusklassen

| Klasse | Anzahl | Zweck |
|---|---|---|
| Markdown Deutsch | 8 | verschiedene Themen (Küche, Verkehr, Recht, Medizin, Garten, Musik, Sport, Technik) für semantische Treffer |
| Markdown Englisch | 8 | dieselben Themen, Grundlage für crosslinguale Anfragen |
| Quelltext Rust | 6 | Code-Chunks, Signaturen als Excerpt |
| Quelltext Python | 6 | wie Rust |
| Textdateien mit Beinahe-Duplikaten | 4 | Rangfolge und Gruppierung je Datei |
| Sonderfälle eingecheckt | 6 | leere Datei, nur Whitespace, eine sehr lange Zeile (minifiziert), CRLF-Datei, UTF-8 mit BOM, Datei mit Emoji und CJK (Offsets) |
| Zur Laufzeit erzeugt | — | Binärdatei mit NUL, ungültiges UTF-8, Datei über `max_file_bytes`, Dot-Datei, Verzeichnis mit Dot-Namen |

#### Gold-Set-Schema

```json
{
  "version": 1,
  "queries": [
    { "id": "q01", "text": "...", "kind": "lexical", "expect": ["docs/de/espresso.md"], "within_rank": 3 },
    { "id": "q02", "text": "...", "kind": "semantic", "expect": ["docs/en/queues.md"], "within_rank": 5 },
    { "id": "q03", "text": "...", "kind": "crosslingual", "expect": ["docs/de/espresso.md"], "within_rank": 5 }
  ]
}
```

#### Aufgaben

- Korpus mit klar getrennten Themen schreiben; mindestens 12 lexikalische, 12 semantische und 6 crosslinguale Anfragen im Gold-Set.
- `lexical` enthält Schlüsselwörter des Zieldokuments (geeignet für den hashbasierten `FakeBackend`), `semantic` paraphrasiert ohne diese Wörter, `crosslingual` stellt die Frage in der anderen Sprache.
- `TmpCorpus` im Stil von `TmpDirs` (temporäres Verzeichnis, Aufräumen im Drop) mit Methoden zum Kopieren des Korpus und zum Erzeugen der Sonderfälle.
- Eine Prüfsummenliste `resident_corpus.sha` ablegen, damit Änderungen am Korpus Tests bewusst verändern.

#### Abnahme L0.4

- Korpus unter 300 KB, alle Dateien selbst verfasst (Vermerk im Verzeichnis).
- Jede Gold-Anfrage nennt mindestens ein erwartetes Dokument; kein Dokument ist Erwartung für mehr als vier Anfragen.
- Ein Selbsttest lädt Korpus und Gold-Set und prüft, dass alle erwarteten Pfade existieren.

### L0.5 · Budgets und Schwellen

**Zweck:** Zielwerte festschreiben, gegen die die Gates G4 bis G7 messen.  
**Quellen:** Messwerte M1 bis M6, Berichtsabschnitt J (W1 bis W5).  
**Abhängt von:** L0.3.  
**Zu klären:** Referenzhardware, Korpusgröße der Zielanwendung (Dateien, MB), akzeptable Kaltstartzeit im CLI.  
**Ergebnis:** Tabelle „Budgets“ mit endgültigen Werten, je mit Messquelle; vorläufige Werte bleiben mit [W] markiert, bis M1 bis M6 vorliegen.

#### Budgets

| ID | Budget | Vorläufiger Wert [W] | Quelle |
|---|---|---|---|
| B1 | Kaltstart bis zum ersten Query-Embedding (CPU) | höchstens 4 s | M1 |
| B2 | Warme Anfrage Ende zu Ende bei 50k Chunks | höchstens 250 ms | M2, M4 |
| B3 | Auto-Index-Budget je Anfrage | höchstens 1,5 s oder 32 Chunks | M2 |
| B4 | Indexdurchsatz ohne Modell (FakeBackend) | mindestens 3000 Chunks/s | L3.8 |
| B5 | Flush bei 50k Chunks | höchstens 2 s, sonst Sharding (D6) | M4 |
| B6 | Katalog-Commit bei 10k Dateien | höchstens 500 ms | M5 |
| B7 | Zusätzlicher Speicher im Query-Pfad bei 100k Chunks | höchstens 300 MB RSS | M4 |
| B8 | Indexgröße je Chunk | höchstens 1,2 KB | M4 |
| B9 | Cancel-Latenz von Update und Background | höchstens 1 s | L3.7 |
| B10 | Recall@5 Gold-Set lexikalisch (FakeBackend) | mindestens 0,9 | L4.6 |
| B11 | Recall@5 Gold-Set semantisch (e5) | mindestens 0,7 (vorläufig) | L5.5 |
| B12 | Recall@5 Gold-Set crosslingual (e5) | mindestens 0,6 (vorläufig) | L5.5 |

#### Abnahme L0.5

- Jedes Budget nennt Messmethode und Quelle.
- Die Parameter in `plan_01_02_decisions.md` (D4 Chunkgrößen, D6 Flush, D9 Auto-Budget) sind mit den Messwerten abgeglichen oder bewusst beibehalten.

## Ergebnisse (auszufüllen)

### Ergebnisse R1 bis R10

| ID | Ergebnis | Datum | Konsequenz |
|---|---|---|---|
| R1 | offen | | |
| R2 | offen | | |
| R3 | offen | | |
| R4 | offen | | |
| R5 | offen | | |
| R6 | offen | | |
| R7 | offen | | |
| R8 | offen | | |
| R9 | offen | | |
| R10 | offen | | |

### Messwerte M1 bis M6

| ID | Wert | Maschine | Datum |
|---|---|---|---|
| M1 | offen | | |
| M2 | offen | | |
| M3 | offen | | |
| M4 | offen | | |
| M5 | offen | | |
| M6 | offen | | |

## Gate G0

- R1 bis R10 sind ausgefüllt, Abweichungen den Paketen zugeordnet.
- Blockierende Entscheidungen (D1, D2, D4, D5, D8, D9, D12) sind bestätigt oder geändert.
- Messwerte M1 bis M6 liegen vor, Budgets B1 bis B12 sind festgeschrieben.
- Fixture-Korpus und Gold-Set sind eingecheckt, der Selbsttest ist grün.
- `cargo test` (Default), `cargo test --features mock` und `cargo test --features candle` sind als Basislinie dokumentiert und grün.

## Risiken und Stolpersteine

- Ohne installiertes Modell lassen sich M1 bis M3 und alle Modelltests nicht fahren; dann S1 und IP1 mit `FakeEncoder`-Variante vorbereiten und Modelltests bis zur Beschaffung offen lassen.
- Mess-Gates dürfen nicht Teil der Standard-Testläufe sein (`#[ignore]`), sonst verlängert sich CI erheblich.
- Der Seitencache verfälscht M1 und M4; kalte Messungen brauchen einen dokumentierten Weg (zum Beispiel `sync` und `echo 3 > /proc/sys/vm/drop_caches` mit Rechten) oder werden als „warm“ gekennzeichnet.
- Der Gold-Set darf nicht auf den FakeBackend zugeschnitten werden, ohne die semantischen Anfragen getrennt zu halten, sonst entsteht eine Scheingenauigkeit.
