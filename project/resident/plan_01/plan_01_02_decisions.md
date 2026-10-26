# Plan 01 — Entscheidungsregister

Alle Entscheidungen, die den Zuschnitt des Layers festlegen. Jede trägt eine Empfehlung mit Begründung aus dem Befundbericht (`plan_01_01_evidence.md`, IDs `E*`/`K*`/`W*`). Der Status ist durchgehend **Vorschlag**; L0.2 (Stufe 0) holt die Bestätigung des Auftraggebers ein. Blockierend heißt: die genannte Stufe darf ohne Bestätigung nicht beginnen.

## Übersicht

| ID | Thema | Empfehlung (kurz) | Blockiert | Status |
|---|---|---|---|---|
| D1 | Layer-ID und Aktivierung | `id = resident`, Aktivierungsfeld `resident`, Begleitfelder `residentIndex`, `residentTopK`, `residentMinScore` | S2 | Vorschlag |
| D2 | Modulstandort | `rag/src/layers/resident_layer/` | S2 | Vorschlag |
| D3 | Backend-Naht | Trait `SemanticBackend` mit `ResidentBackend` und `FakeBackend` | S2 | Vorschlag |
| D4 | Chunking und Offsets | zeilenbasiert, Zeichenbudget 1000/1400, Offsets in Unicode-Skalarwerten, Pfad-Kopfzeile | S3 | Vorschlag |
| D5 | EntryId-Schema | `(file_no << 20) \| chunk_no`, `file_no` monoton und nie wiederverwendet | S3 | Vorschlag |
| D6 | Index-Topologie | ein `ParquetIndex` je Dokumentwurzel, `verify=false` im Query-Pfad, gebündeltes Flush | S3 | Vorschlag |
| D7 | Katalog und Änderungserkennung | eigener Katalog (256 JSON-Shards in `LayerStorage`), `FileStamp` plus Hash; `SharedIndex` nicht genutzt | S3 | Vorschlag |
| D8 | Dateiabdeckung und Filter | bedarfsgetrieben; Scope = Katalog geschnitten mit sichtbaren Dateien | S3, S4 | Vorschlag |
| D9 | Indexierungstrigger | Modi `auto`, `update`, `rebuild`, `status`, `off`; Background im CLI aus | S3 | Vorschlag |
| D10 | Ergebnisformat | `File`, `Lines` plus Objekt `Resident`; ein Eintrag je Datei | S4 | Vorschlag |
| D11 | Runtime-Lebenszyklus | lazy `OnceCell`, geteilt zwischen `run` und `background` | S2 | Vorschlag |
| D12 | Umfang Stufe 1 | Variante A: Engine-Zusammenbau im Rahmen von Plan 01 | Ressourcenplanung | Vorschlag |
| D13 | Schreib- und Scoring-Pfad | Layer schreibt per `ParquetIndex::upsert`, Query per `Similarity`-Task | S3, S4 | Vorschlag |
| D14 | Konfiguration | TOML plus Umgebung plus Query-Felder; Modellpfad ohne festen Default | S2 | Vorschlag |
| D15 | Threading | `cpu_workers = 1`, Candle-Threads je nach Modus | S5 | Vorschlag |
| D16 | Schreibsperre | Advisory-Lock `index.lock`, Leser ohne Sperre | S3 | Vorschlag |
| D17 | Feature-Gating | kein `cfg(feature)` im Layer; Laufzeit-Verfügbarkeit | S2 | Vorschlag |
| D18 | Precision | Fassaden-Default durchreichen, keine Kalibrierung im Layer | S4 | Vorschlag |
| D19 | Schwellen und Ranking | Top-k plus optional `residentMinScore`; Statistik nur in `detail` | S4 | Vorschlag |
| D20 | Veraltete Treffer | verwerfen und zählen; per `keep_stale` markieren | S4 | Vorschlag |

## Entscheidungen

### D1 · Layer-ID und Aktivierung

**Kontext:** Der Layer-Vertrag koppelt `id` an Storage-Namespace und Standard-Aktivierungsfeld (E1). grep und trigram aktivieren sich über `query` (E9); ein dritter Layer auf `query` würde jede Textsuche zusätzlich Modell und Index laden lassen.  
**Optionen:** (a) `id = "resident"`, Aktivierung über das Feld `resident` (Anfragetext); (b) `id = "semantic"`; (c) Aktivierung über `query` plus Flag.  
**Empfehlung:** (a). Begleitfelder `residentIndex` (`auto`, `update`, `rebuild`, `status`, `off`), `residentTopK`, `residentMinScore`; gemeinsam genutzt `directory` und `maxResults`.  
**Konsequenz:** Bestehende `query`-Aufrufe bleiben unverändert und kostenfrei; semantische Suche ist ausdrücklich angefordert.  
**Betrifft:** L2.1, L2.6, L4.1, L5.1.  
**Status:** Vorschlag, Bestätigung ausstehend.

### D2 · Modulstandort und Namensgebung

**Kontext:** `crate::resident` ist die Engine (K1); trigram nutzt ein Verzeichnis mit `layer.rs`, die Einzeldatei-Layer tragen das Suffix `_layer`.  
**Optionen:** (a) `rag/src/layers/resident_layer/` mit `mod.rs`, `layer.rs` und weiteren Dateien; (b) `layers/resident/`.  
**Empfehlung:** (a). Vermeidet die Verwechslung mit `crate::resident`, folgt `grep_layer`, `glob_layer`, `merge_layer`; Re-Export `pub use layer::ResidentLayer`.  
**Konsequenz:** Engine-Typen immer als `crate::resident::…` importieren, Layer-interne Typen als `super::…`.  
**Betrifft:** L2.1.  
**Status:** Vorschlag, Bestätigung ausstehend.

### D3 · Backend-Naht und Fake

**Kontext:** Die Engine ist ohne produktive `Engine` nicht lauffähig (E26); die Konventionen empfehlen schmale Traits und handgeschriebene Fakes (E43).  
**Optionen:** (a) eigener Trait `SemanticBackend` mit `ResidentBackend` (Fassade) und `FakeBackend`; (b) Fassade direkt verwenden, Tests mit `TestEngine`.  
**Empfehlung:** (a). Entkoppelt S2 bis S4 von S1, macht die Layer-Logik ohne Modell testbar, hält Engine-Typen aus der Layer-Logik heraus.  
**Konsequenz:** Ein zusätzlicher Trait mit wenigen Methoden; `FakeBackend` unter `cfg(any(test, feature = "mock"))`.  
**Betrifft:** L2.4, L2.5.  
**Status:** Vorschlag.

### D4 · Chunking und Offsets

**Kontext:** Chunking liegt in der Suchschicht (E42). `Payload` trägt `char_start` und `char_end` ohne dokumentierte Einheit (E37). Der Tokenizer kürzt stumm auf `max_seq_len` 512 (E32).  
**Optionen:** Einheit (a) Unicode-Skalarwerte, (b) Bytes. Strategie (a) zeilenbasiert mit Zeichenbudget, (b) tokenbasiert über `SharedTokenizer`.  
**Empfehlung:** Einheit Skalarwerte (entspricht dem Feldnamen), gekapselt in `OffsetMap`. Strategie zeilenbasiert mit Zeichenbudget `target 1000`, `max 1400`, `min 250`, `overlap 150` (vorläufig [W], Kalibrierung durch M3), Kopfzeile mit Dateipfad im Passage-Text (`context_header = "path"`); Offsets beziehen sich auf den Dateitext ohne Kopfzeile.  
**Konsequenz:** Der Chunker ist deterministisch und versioniert (`CHUNKER_VERSION`); Tokenzählung nur in Tests und Kalibrierung.  
**Betrifft:** L3.2, L3.3.  
**Status:** Vorschlag, Bestätigung ausstehend.

### D5 · EntryId-Schema

**Kontext:** `EntryId(u64)` wird frei vergeben (E35); Scopes arbeiten auf Range und Set (E40).  
**Optionen:** (a) `(file_no << 20) | chunk_no` mit persistenter, monoton wachsender `file_no`; (b) Hash des Pfads; (c) fortlaufende Nummer ohne Dateibezug.  
**Empfehlung:** (a). Jede Datei belegt einen zusammenhängenden Bereich, Ersetzen oder Löschen einer Datei ist ein Bereichsvorgang; `file_no` wird in Blöcken à 256 reserviert und vor Gebrauch persistiert, nie wiederverwendet.  
**Konsequenz:** Höchstens 2^20 Chunks je Datei (Cap `max_chunks_per_file`), 2^44 Dateien; Verzeichnisscope als `Scope::Set` aus Bereichen.  
**Betrifft:** L3.4, L4.2.  
**Status:** Vorschlag, Bestätigung ausstehend.

### D6 · Index-Topologie und Flush-Politik

**Kontext:** `ParquetIndex::flush` schreibt alle Zeilen (E36, W4); `verify` ist per Default an (K5).  
**Optionen:** (a) ein Index je Dokumentwurzel, `verify=false` im Query-Pfad, gebündeltes Flush (alle 2000 Chunks oder 30 s oder am Laufende); (b) Sharding in mehrere Parquet-Dateien.  
**Empfehlung:** (a). Sharding erst, wenn M4 zeigt, dass ein Flush länger als 2 s dauert oder der Index 50k Chunks überschreitet (Ausblick).  
**Konsequenz:** Das Absturzfenster beträgt höchstens ein Flush-Intervall; der Katalog-Commit folgt dem Flush (D7).  
**Betrifft:** L3.5, L6.6.  
**Status:** Vorschlag.

### D7 · Änderungserkennung und Katalog

**Kontext:** `SharedIndex` wird von keinem Layer befüllt (E15); `LayerStorage::put` ist je Aufruf eine Transaktion (E14); `FileStamp` und `content_hash` existieren (E39).  
**Optionen:** (a) eigener Katalog im Layer (256 JSON-Shards in `LayerStorage`) mit `FileStamp`-Entscheid und Hash-Fallback; (b) `SharedIndex` plus Cursor nutzen und selbst befüllen; (c) ein `put` je Datei wie trigram.  
**Empfehlung:** (a). Übernahme von `SharedIndex` als Änderungsquelle im Ausblick, sobald eine zentrale Komponente ihn befüllt.  
**Konsequenz:** Eigener Walk je Anfrage (über `DirCache`, gecacht); Commit-Reihenfolge Daten, Flush, Katalog.  
**Betrifft:** L3.4, L3.7.  
**Status:** Vorschlag.

### D8 · Dateiabdeckung und Filter

**Kontext:** Der globale Include/Exclude-Filter gilt prozessweit je CLI-Aufruf und wird von `DirCache::list` angewandt (E16); `PathFilter` ist nur darüber nutzbar (E17).  
**Optionen:** (a) bedarfsgetriebener Aufbau: indexiert werden die in der aktuellen Anfrage sichtbaren Dateien; Anfrage-Scope ist der Katalog geschnitten mit den sichtbaren Dateien; (b) immer alles Zulässige indexieren (ungefilterter Walk) und beim Abfragen filtern.  
**Empfehlung:** (a). Entspricht dem trigram-Verhalten (Indexierung folgt dem Walk) und vermeidet eine teure Vollindexierung wider den Filter; `residentIndex=update` ohne Filter erzeugt Vollabdeckung; `detail.coverage` meldet sichtbar, indexiert, veraltet.  
**Konsequenz:** Ergebnisse sind je Anfrage konsistent zum Filter; die Abdeckung wächst mit der Nutzung.  
**Betrifft:** L3.1, L4.2.  
**Status:** Vorschlag, Bestätigung ausstehend.

### D9 · Indexierungstrigger und Background

**Kontext:** Die Erstindexierung ist rechenintensiv (W2); `shutdown(false)` wartet auf `background` (E4); die Engine ist auf „Anytime“-Verhalten ausgelegt.  
**Optionen:** (a) Modi `auto` (klein, begrenzt), `update`, `rebuild`, `status`, `off`, Background im CLI standardmäßig aus; (b) vollständig lazy im Query-Pfad; (c) Background immer an.  
**Empfehlung:** (a). `auto` höchstens 1,5 s und 32 Chunks, Priorität Dateien unter der Suchwurzel, zuletzt geändert, klein; `update` wiederaufnehmbar und abbrechbar mit Fortschritt in `status.detail`; `background` nur per Konfiguration (Bibliotheks- und Daemon-Betrieb).  
**Konsequenz:** Der Erstaufbau ist ein ausdrücklicher Schritt; Anfragen bleiben vorhersagbar schnell.  
**Betrifft:** L3.7, L4.1.  
**Status:** Vorschlag, Bestätigung ausstehend.

### D10 · Ergebnisformat

**Kontext:** `MergeLayer` fasst per `File` zusammen (E12); die Core-Doku nennt `path` und `line` (E13).  
**Optionen:** (a) `File`, `Lines` (Zeile als String-Schlüssel) plus Objekt `Resident` mit `Score`, `Rank`, `Chunks[{Start, End, Score}]`, ein Eintrag je Datei; (b) ein Eintrag je Chunk.  
**Empfehlung:** (a). Excerpt ist die erste nicht-leere Zeile des Chunks (höchstens 200 Zeichen, Mitte gekürzt), Anzahl über `excerpt_lines`.  
**Konsequenz:** MergeLayer-kompatibel; Scores auf vier Nachkommastellen gerundet.  
**Betrifft:** L4.4, L4.5, L5.2.  
**Status:** Vorschlag.

### D11 · Runtime-Lebenszyklus

**Kontext:** One-Shot-CLI (E4); `Resident::init()` startet Worker-Threads, Ressourcen entstehen lazy (E18).  
**Optionen:** (a) lazy `tokio::sync::OnceCell` im Layer, geteilt zwischen `run` und `background`, Drop beim Prozessende; (b) Start bei der Registrierung.  
**Empfehlung:** (a). Kein Thread-Start für Anfragen ohne `resident`; ein Fehlschlag der Initialisierung wird je Prozess gemerkt (kein Retry-Sturm), außer bei Timeout.  
**Konsequenz:** Der One-Shot-Aufruf trägt die Kaltstartkosten (W1); Bibliotheks- und Daemon-Betrieb profitieren automatisch.  
**Betrifft:** L2.4, L2.6.  
**Status:** Vorschlag.

### D12 · Umfang von Stufe 1 (Engine-Zusammenbau)

**Kontext:** E26 bis E29 zeigen die fehlende Verdrahtung (Engine, `IndexId`-Auflösung, Rolle zu Präfix).  
**Optionen:** (A) Stufe 1 wird im Rahmen von Plan 01 umgesetzt (neues Modul `rag/src/resident/runtime/`); (B) der Engine-Workstream liefert vorab, Plan 01 startet bei S2 und behandelt S1 als externe Abhängigkeit.  
**Empfehlung:** (A), abgesichert durch die Prüfungen R1 bis R4 in L0.1. Existiert lokal bereits ein produktiver Engine-Zweig, schrumpft S1 auf die Abnahme L1.7.  
**Konsequenz:** S1 ist der kritische Pfad (Aufwand L); S2 bis S4 laufen parallel gegen `FakeBackend`.  
**Betrifft:** S1, IP1.  
**Status:** Vorschlag, Bestätigung ausstehend.

### D13 · Schreib- und Scoring-Pfad

**Kontext:** Der Planner kennt `WriteIndex`, der Op fehlt aber eine EntryId-Quelle (E25, E28); laut Charta liegt der Persistenz-Zeitpunkt außerhalb der Engine (E42).  
**Optionen:** (a) Der Layer schreibt über `ParquetIndex::upsert` (Embeddings über die Fassade ohne `write_index`), die Anfrage läuft über einen `Similarity`-Task; (b) `write_index` mit Engine-Op; (c) Scoring im Layer (Brute Force).  
**Empfehlung:** (a). Ein Brute-Force-Scorer existiert nur als Test-Referenz (Golden).  
**Konsequenz:** Der Layer besitzt die Abbildung Chunk zu EntryId und den Flush-Zeitpunkt; die Engine bleibt Substrat.  
**Betrifft:** L1.5, L3.6, L4.3.  
**Status:** Vorschlag.

### D14 · Konfiguration

**Kontext:** Der Modellpfad ist nicht festgelegt; Tests nutzen `CARGO_MANIFEST_DIR/model/…` (E33).  
**Optionen:** (a) TOML in `.xyrag/layers/resident/config.toml` plus `XYRAG_RESIDENT_*` plus Query-Felder; (b) nur Umgebung.  
**Empfehlung:** (a), Rangfolge Query-Felder, Umgebung, Datei, Defaults. Kein Default für den Modellpfad außer dem Entwicklungs-Fallback `CARGO_MANIFEST_DIR/model/multilingual-e5-small`, sofern vorhanden. Ungültige Konfiguration führt zu `skipped` mit Grund, nie zum Abbruch der Anfrage.  
**Konsequenz:** `toml` und `serde` sind vorhanden; `deny_unknown_fields`.  
**Betrifft:** L2.2.  
**Status:** Vorschlag.

### D15 · Threading und CPU-Budget

**Kontext:** `Engine.cpu` ist ein eigener rayon-Pool (E8); `Resident` startet eigene Worker, Candle nutzt rayon; `WorkerConfig` kennt `candle_rayon_threads` und `cpu_nice` (letzteres wirkt nur bei GPU-Workern).  
**Optionen:** (a) `cpu_workers = 1`, `candle_rayon_threads = max(1, ncpu - 1)` bei `update` und `rebuild`, `max(1, ncpu / 2)` bei `auto` und Anfragen; (b) Standardwerte.  
**Empfehlung:** (a); die Wirksamkeit wird in S5 gemessen (**[?]** ob Candle den globalen oder einen eigenen Pool nutzt).  
**Konsequenz:** Die Aktivierung per Feld isoliert die Layer meist voneinander, weil grep und trigram `query` benötigen.  
**Betrifft:** L1.6, L5.3.  
**Status:** Vorschlag.

### D16 · Schreibsperre

**Kontext:** Mehrere `xyrag`-Prozesse können dieselbe Wurzel bedienen; `ParquetIndex::flush` ersetzt die Datei atomar, kennt aber keinen Schreiberschutz.  
**Optionen:** (a) Advisory-Lock `index.lock` im Layer-Verzeichnis (`File::try_lock` oder `libc::flock`), Leser ohne Sperre; (b) keine Sperre.  
**Empfehlung:** (a). Ein Prozess ohne Sperre läuft read-only (Anfrage ja, Indexieren nein) und meldet `index_mode = read_only`.  
**Konsequenz:** Kein verlorenes Update bei Parallelstart; die Toolchain-Version ist zu prüfen (**[?]**, L0.1).  
**Betrifft:** L3.5.  
**Status:** Vorschlag.

### D17 · Feature-Gating

**Kontext:** `cfg(feature)` ist nur an dokumentierten Stellen erlaubt (E44).  
**Optionen:** (a) kein `cfg(feature)` im Layer, Verfügbarkeit zur Laufzeit (Modell, Registry), Testdoubles hinter `mock`; (b) eigenes Feature `resident-layer`.  
**Empfehlung:** (a).  
**Konsequenz:** Der Layer wird immer kompiliert und registriert; ohne `candle` oder Modell meldet er `skipped` mit Grund; `scripts/check_cfg_scatter.sh` bleibt grün.  
**Betrifft:** L2.1, L7.2.  
**Status:** Vorschlag.

### D18 · Precision und Kalibrierung

**Kontext:** `EmbeddingParams` und `SimilarityParams` tragen `precision` (Default 1.0, NaN wird abgelehnt) (E19); die Kalibrierung ist Engine-Sache (Gruppe F der Metaplanung).  
**Empfehlung:** Der Layer reicht den Fassaden-Default durch, optional über den Konfigwert `precision`; keine Kalibrierung im Layer.  
**Betrifft:** L2.2, L4.3.  
**Status:** Vorschlag.

### D19 · Schwellen und Ranking

**Kontext:** Der Core rankt nicht numerisch (E1); der Report liefert Statistik, Schwellen bleiben außen (E21); e5-Scores liegen in einem engen Band.  
**Optionen:** (a) Top-k plus optionales `residentMinScore`, Statistik nur in `detail`; (b) automatisches Abschneiden (Elbow, `z_top1`).  
**Empfehlung:** (a). Elbow- oder Gap-basiertes Abschneiden folgt als Option nach Auswertung des Gold-Sets (Ausblick).  
**Betrifft:** L4.3, L4.5.  
**Status:** Vorschlag.

### D20 · Umgang mit veralteten Treffern

**Kontext:** Treffer verweisen auf Zeichenbereiche einer früheren Dateiversion; `RankedHit.outdated` betrifft Index-Stamps, nicht die Quelldatei (E21).  
**Optionen:** (a) Treffer aus Dateien mit geänderter Größe oder mtime verwerfen und als `stale_dropped` zählen (die Datei wird für die nächste Runde vorgemerkt); (b) behalten und markieren (`Stale: true`).  
**Empfehlung:** (a) als Default, (b) per `keep_stale`.  
**Konsequenz:** Korrekte Zeilennummern und Excerpts; im Auto-Modus werden geänderte Dateien zuerst neu indexiert (Budget).  
**Betrifft:** L4.4.  
**Status:** Vorschlag.
