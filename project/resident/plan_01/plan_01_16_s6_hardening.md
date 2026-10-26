# Stufe 6 — Härtung und Betrieb

Stufe 6 macht den Layer betriebssicher: eine ausgetestete Degradationsmatrix, Ressourcenlimits, Migrations- und Versionierungsregeln, Telemetrie und Diagnose, Sicherheit des Dateizugriffs sowie Leistungsmessung mit Tuning. Aufwand [W]: 4 Personentage.

## Eingang

Gate G5 bestanden (Layer funktional vollständig, IP1 abgeschlossen). Die Grundtabelle aus L2.3, die Budgets aus L0.5 und die Messwerte aus L0.3 liegen vor.

## Pakete

### L6.1 · Degradationsmatrix und Fehlerpfade

**Zweck:** Für jede erwartbare Störung ist das Verhalten festgelegt, getestet und für Nutzer erkennbar; keine Störung bricht die Gesamtanfrage ab.  
**Quellen:** L2.3 (Grundtabelle), Konventionen Q1 (`IsRetryable`), `resident/failpoint.rs` (`hit`, `arm`; E54), E7 (Panics treffen die Anfrage).  
**Abhängt von:** G5.  
**Zu klären:** Fehlerinjektion in `IndexStore` und `Catalog` über Failpoints (E54); Verhalten bei vollem Datenträger (Flush scheitert, Speicherzustand bleibt gültig).  
**Ergebnis:** Tabelle (unten) vollständig durch Tests belegt.

#### Matrix

| Nr | Situation | Erkennung | Verhalten | Status und `detail` | Behebung |
|---|---|---|---|---|---|
| 1 | Modell fehlt oder unlesbar | Start der Runtime scheitert (Config oder Model) | Layer überspringt die Anfrage | `skipped`, `reason = model_unavailable` | Modell installieren, Pfad konfigurieren |
| 2 | Backend nicht verfügbar (kein `candle`) | Registry ohne passende Architektur | wie 1 | `skipped`, `reason = backend_unavailable` | Build mit `--features candle` |
| 3 | Index inkompatibel (Modell, Präfix, Tokenizer, Schema) | `IncompatibleSchema` oder Fingerabdruck | Quarantäne, frischer Index, Katalog zurücksetzen; diese Anfrage ohne Treffer, sofern das Auto-Budget nicht reicht | `ran`, `index.rebuilt = true`, `hint` | automatisch, Aufbau per `update` |
| 4 | Index beschädigt | `ChecksumMismatch`, `IndexUnavailable`, Parquet-Lesefehler | wie 3 (Endung `.corrupt-<ts>`) | wie 3 | automatisch |
| 5 | Datenträger voll oder schreibgeschützt beim Flush | I/O-Fehler | Indexierung bricht ab, Katalog wird nicht committet, Anfrage läuft read-only weiter | `index_mode = read_only`, `error.kind = io` | Platz schaffen |
| 6 | Zweiter Schreiber | Sperre nicht erhalten | Anfrage ohne Indexierung | `index_mode = read_only`, `reason = index_locked` | warten |
| 7 | Embedding-Timeout | `BackendError::Timeout` | Anfrage ohne Treffer; Indexlauf endet, nicht fertige Dateien werden nicht geschrieben | `aborted`, `reason = timeout`, `phase` | Budget erhöhen, Kaltstart abwarten |
| 8 | Panic im Rayon-Block des Layers | `ctx.cpu.spawn` liefert `Err` | Phase wird abgebrochen | `skipped` oder `aborted`, `reason = internal` | Fehler melden |
| 9 | Ungültiger Vektor (NaN, falsche Länge) | Validierung im Adapter | Datei gilt als `failed`, nichts geschrieben | `files_failed` | Modell prüfen |
| 10 | Datei ändert oder löscht sich während des Laufs | Lese- oder Stempelabweichung | Datei übersprungen, nächste Runde | `files_failed` bzw. `dropped_stale` | automatisch |
| 11 | Katalog und Index widersprüchlich | `reconcile` beim Öffnen | Waisen entfernen, fehlende Dateien neu planen | `index.repaired = n` | automatisch |
| 12 | Abbruch (Shutdown, Drop der Future) | `CancellationToken` oder Drop | sofort stoppen, letzter vollständiger Flush bleibt gültig | `aborted`, `reason = cancelled` | — |
| 13 | Unbekannte neuere Layer-Version im Katalog | `layer_version` größer als bekannt | nichts ändern, nicht überschreiben | `skipped`, `reason = index_newer` | passende Programmversion verwenden |

#### Aufgaben

- Für jede Zeile einen Test mit gezielter Störung (FaultScript, Dateisystem-Rechte, manipulierte Dateien, Test-Nahten); erwartetes `LayerStatus` und `detail` als Golden prüfen.
- Fehlerinjektion über die vorhandene Facility `crate::resident::failpoint`: an den Nahtstellen `failpoint::hit("layer.store.flush", key)` und `hit("layer.commit.before_catalog", key)` aufrufen; Tests schärfen sie mit `arm(name, key, times)` (der `Armed`-Guard entschärft beim Drop). Ohne `test`/`mock` ist `hit` konstant `false` (LP11, E54).
- Jede Warnung über `tracing::warn!` mit stabilen Feldern (L6.4).
- Prüfen, dass nach jeder Störung die nächste Anfrage ohne manuellen Eingriff wieder funktioniert (außer 1, 2, 13).

#### Abnahme L6.1

- 13 Tests `degrade_*` grün, je einer pro Zeile.
- Ein Sammeltest `no_degradation_fails_the_whole_query`: Engine mit allen Layern, je Zeile eine Störung, die anderen Layer liefern unverändert.

### L6.2 · Ressourcenlimits

**Zweck:** Speicher, Plattenplatz und Laufzeit sind nach oben begrenzt, auch bei unerwarteten Eingaben.  
**Quellen:** Konfigurationstabelle (L2.2), `grep_layer.rs` (`MAX_FILE_SIZE`, `MAX_MATCHES`), Budgets B7, B8.  
**Abhängt von:** L3.x, L4.x.  
**Zu klären:** Standardwerte nach L0.3; Verhalten bei Erreichen (stoppen, nicht löschen).  
**Ergebnis:** Alle Limits sind konfigurierbar, wirken wie unten beschrieben und erscheinen im Status.

#### Limits

| Limit | Default [W] | Wirkung bei Überschreitung | Status |
|---|---|---|---|
| `index.max_file_bytes` | 1 MiB | Datei wird übersprungen | `skipped_by_reason.too_large` |
| `index.max_chunks_per_file` | 1500 | Rest der Datei nicht indexiert | `files_truncated`, Eintrag mit `truncated` |
| `index.max_chunks_total` | 500000 | Indexierung stoppt, bestehender Index bleibt nutzbar | `index.cap_reached = true`, `hint` |
| `index.max_files_per_run` | 20000 | Lauf endet, Rest in der nächsten Runde | `remaining_files` |
| Anfragetext | 2000 Zeichen | gekürzt | `query_truncated` |
| `maxResults` | 200 | geklemmt | `clamped` |
| `residentTopK` | 400 | geklemmt | `clamped` |
| In-Flight-Batches | 2 je Worker | Backpressure | — |
| Scorer-Zusatzspeicher | 64 MB | blockweise Verarbeitung | — |
| Excerpt-Zeile | 200 Zeichen | Mittenkürzung | — |

#### Aufgaben

- `index.max_files_per_run` und `query.max_chunks_per_file` in die Konfigurationstabelle (L2.2) aufnehmen, falls noch nicht geschehen.
- Je Limit ein Test mit gezielter Überschreitung; die Wirkung (stoppen statt abstürzen) und die Statusanzeige prüfen.
- Gesamtspeicherbedarf im Query-Pfad bei 100k synthetischen Chunks messen (B7) und gegen die Tabelle abgleichen.

#### Abnahme L6.2

- Tests `limit_file_bytes`, `limit_chunks_per_file`, `limit_chunks_total_keeps_index_usable`, `limit_files_per_run_resumes`, `limit_query_chars`, `limit_clamps`, `memory_bound_for_100k_chunks` (`#[ignore]`, Messung).

### L6.3 · Migration und Versionierung

**Zweck:** Änderungen an Layer, Chunker, Modell oder Engine-Schema führen kontrolliert zu Neuindexierung statt zu stillen Inkonsistenzen.  
**Quellen:** Layer-Vertrag („Versioniere intern über `LayerStorage`-Metadaten, nicht über die ID“), `index/schema.rs` (`SCHEMA_VERSION`, E38), L3.4 (`cat:meta`).  
**Abhängt von:** L3.4, L3.5.  
**Zu klären:** Fingerabdruck der Chunk-Parameter in der Katalogidentität; Verhalten bei neuerem Katalog (Zeile 13 der Matrix).  
**Ergebnis:** Versionsmatrix und `migrate`-Haken.

#### Versionsmatrix

| Änderung | Erkennung | Aktion |
|---|---|---|
| `layer_version` (Katalogformat, Layout) | `cat:meta.layer_version` | Katalog und Index zurücksetzen oder `migrate(from, to)` anwenden; bei höherer gespeicherter Version nichts ändern |
| Chunker-Implementierung oder -Parameter | `FileRecord.chunker_fp` (xxh3 über `CHUNKER_VERSION` und Chunk-Parameter) | betroffene Dateien lazy neu indexieren (`changed`), Index bleibt |
| Modell, Revision, Tokenizer, Präfixe, Pooling, Normalisierung, Dtype, `max_seq_len` | `SchemaHeader` und `model_fingerprint` | vollständiger Neuaufbau mit Quarantäne |
| Engine-`SCHEMA_VERSION` | `IncompatibleSchema` | vollständiger Neuaufbau |
| Konfiguration ohne Einfluss auf Inhalte (Budgets, Timeouts) | — | keine Aktion |

#### Aufgaben

- `FileRecord.chunker_fp: u64` (Fingerabdruck aus `CHUNKER_VERSION` und den Chunk-Parametern, in L3.4 vorgesehen) definieren und testen; Parameteränderungen fordern so automatisch Neuindexierung.
- `cat:meta` um `layer_version` erweitern und beim Laden prüfen: kleiner, gleich, größer.
- `migrate`-Haken als Tabelle von Funktionen (heute leer), damit spätere Versionen Daten erhalten können.
- Rollback-Szenario testen: ältere Programmversion trifft neueren Katalog (nichts zerstören, `skipped / index_newer`).

#### Abnahme L6.3

- Tests `chunk_param_change_requeues_files`, `model_fingerprint_change_rebuilds`, `engine_schema_bump_rebuilds`, `layer_version_newer_is_read_only_skip`, `layer_version_older_triggers_reset_or_migration`.

### L6.4 · Telemetrie und Diagnose

**Zweck:** Betrieb und Fehlersuche ohne Debugger: stabile Spans, Felder und Ebenen, plus eine Statusabfrage.  
**Quellen:** Konventionen Q2 (feste Feldnamen, `instrument` statt `enter` über `await`, `for_blocking`), `resident/telemetry.rs` (`task_span`, `resource_span`, `for_blocking`), `bin/xyrag.rs` (Subscriber nur im Binary).  
**Abhängt von:** G5.  
**Zu klären:** Welche Ereignisse welche Ebene tragen; Aufnahme von Layer-Feldern in die Telemetrie-Konstanten.  
**Ergebnis:** Span- und Ereignisliste, `residentIndex=status` als Diagnoseausgabe.

#### Spans und Ereignisse

| Name | Typ | Ebene | Felder |
|---|---|---|---|
| `resident.layer.run` | Span | info | `query_len`, `mode`, `scope_kind`, `top_k`, `backend` |
| `resident.layer.index` | Span | info | `mode`, `files`, `chunks`, `budget_ms` |
| `resident.layer.map` | Span | debug | `hits`, `files` |
| `index opened` | Ereignis | info | `entries`, `files`, `outcome` |
| `index rebuilt` | Ereignis | warn | `reason`, `quarantine` |
| `index flushed` | Ereignis | info | `rows`, `ms` |
| `catalog committed` | Ereignis | debug | `shards`, `ms` |
| `file decision` | Ereignis | debug | `rel`, `decision` |
| `degraded` | Ereignis | warn | `row` (Nummer aus der Matrix), `reason` |
| `model cold start` | Ereignis | info | `ms` |
| `chunk` | Ereignis | trace | `file_no`, `chunk_no`, `chars` |

#### Aufgaben

- Spans per `.instrument(span)` an Futures hängen, nie `span.enter()` über ein `await` halten; in `ctx.cpu.spawn`-Closures `telemetry::for_blocking(&Span::current())` verwenden.
- Feldnamen der Engine (`task_id`, `index_id`, `model_id`, `op`) wiederverwenden; layer-eigene Felder (`phase`, `file_count`, `chunk_count`) als Konstanten bündeln.
- Kein globaler Subscriber im Layer (LP10).
- `residentIndex=status` liefert `coverage`, `index {entries, files, truncated_files, size_bytes, last_indexed_ns}`, `model`, `backend`, `chunker_fp`, `layer_version`, `lock` (frei oder gesperrt).
- Test mit einem aufzeichnenden Subscriber (`tracing-subscriber` mit eigenem `MakeWriter`): Die Warnungen der Matrix erscheinen mit den erwarteten Feldern.

#### Abnahme L6.4

- Tests `warn_events_carry_stable_fields`, `status_mode_output_has_expected_keys`, `no_span_guard_across_await` (Review-Check, per Lint oder Code-Suche belegt).

### L6.5 · Sicherheit und Robustheit des Dateizugriffs

**Zweck:** Der Layer liest nur, was er darf, und lässt sich weder durch manipulierte Indexdateien noch durch Sonderdateien aus dem Tritt bringen.  
**Quellen:** `layers/dir_cache.rs` (`read_dir_listing`: Dot-Einträge unsichtbar, Symlinks und Sonderdateien erscheinen als Dateien, E52), `resident/index/payload.rs` (`target_file` stammt aus dem Index), `walk::eligible` (L3.1).  
**Abhängt von:** L3.1, L3.2, L4.4.  
**Zu klären:** Symlinks erscheinen im Listing als Dateien (E52) und werden nie gelesen; Rechte des Verzeichnisses `.xyrag` (Standard-Umask).  
**Ergebnis:** Pfadvalidierung, Dateityp-Prüfung, Denylist und dokumentierte Datenschutzaussagen.

#### Aufgaben

- `target_file` aus dem Index ist nicht vertrauenswürdig: nur relative Pfade ohne `..`, Wurzel, Laufwerksteile, Backslash und NUL akzeptieren, danach mit der Dokumentwurzel verbinden und prüfen, dass das Ergebnis unterhalb der Wurzel liegt; sonst Treffer verwerfen (`dropped_invalid_path`).
- Vor jedem Lesen `symlink_metadata` und Dateityp prüfen (Symlinks, FIFOs, Sockets und Geräte erscheinen im Listing als Dateien, E52): nur reguläre Dateien (FIFOs, Sockets, Geräte würden blockieren oder unbegrenzt liefern); Symlinks nicht folgen.
- Lesen strikt begrenzt (`take(max_bytes + 1)`); Größe aus Metadaten allein reicht bei sich ändernden Dateien nicht.
- Denylist für Schlüssel- und Datenbankdateien (L3.1) konfigurierbar erweiterbar; dokumentieren, dass Dot-Dateien und Dot-Verzeichnisse nie indexiert werden.
- Datenschutz in der Doku: Der Index enthält Einbettungen, Pfade und Zeichenbereiche, keinen Klartext; Einbettungen können dennoch Inhalte preisgeben, daher Rechte des Verzeichnisses `.xyrag` wie der Quelldaten behandeln; keine Netzwerkzugriffe, Modell nur aus lokalem Pfad.

#### Abnahme L6.5

- Tests `payload_with_parent_dir_is_rejected`, `absolute_target_file_is_rejected`, `fifo_is_skipped`, `symlink_file_is_not_followed`, `file_growing_during_read_is_bounded`, `denylist_names_are_never_indexed`.

### L6.6 · Leistungsmessung und Tuning

**Zweck:** Die Budgets belegen und die Parameter so einstellen, dass sie für den vorgesehenen Korpus tragen; die Sharding-Entscheidung (D6) fällt auf Messbasis.  
**Quellen:** `benches/trie_bench.rs` (Muster, `harness = false`), Budgets B2 bis B9, Messwerte M4 und M5.  
**Abhängt von:** G5.  
**Zu klären:** Referenzkorpus (synthetisch 10k, 50k, 100k Chunks); Hardware.  
**Ergebnis:** `benches/resident_layer_bench.rs` mit `[[bench]]`-Eintrag in `Cargo.toml`, Auswertung in `docs/architecture/resident_layer.md`.

#### Benchmarks

| Gruppe | Gemessen | Bezug |
|---|---|---|
| `chunker/throughput` | MB/s über den Korpus und über synthetischen Quelltext | Erstaufbau ohne Modell |
| `catalog/plan_100k_files` | Zeit für `plan` bei 100k Dateien | Query-Pfad Schritt 4 |
| `catalog/commit_dirty_shards` | Zeit für `commit` bei 1, 16, 256 veränderten Shards | B6 |
| `store/replace_file_and_flush` | `replace_file` und `flush` bei 10k, 50k, 100k Zeilen | B5, D6 |
| `scope/build_set_10k_files` | Aufbau des `EntrySet` | Query-Pfad Schritt 5 |
| `render/map_50_hits` | Abbildung von 50 Treffern | Query-Pfad Schritt 8 |
| `fake_pipeline/ingest` | Durchsatz der Pipeline mit `FakeBackend` | B4 |

#### Aufgaben

- Benchmarks mit `criterion` anlegen; Fixtures synthetisch erzeugen, keine Modellabhängigkeit.
- Ergebnisse gegen B4 bis B6 prüfen; überschreitet `store/replace_file_and_flush` bei 50k Zeilen zwei Sekunden, ist die Sharding-Entscheidung (D6) neu zu treffen und als eigener Folgeplan aufzusetzen.
- Flush-Intervalle und Auto-Budget anhand der Ergebnisse feinjustieren und in D6/D9 nachtragen.
- Regressionsschutz: einfache Schwellentests (`#[ignore]`, Zeit gegen Budget) für B4 und B6.

#### Abnahme L6.6

- Benchmarks laufen (`cargo bench --bench resident_layer_bench`), Ergebnisse stehen in der Doku, B4 bis B6 sind erfüllt oder bewusst angepasst.

## Gate G6

- Alle 13 Zeilen der Degradationsmatrix sind durch Tests belegt; keine Störung bricht die Gesamtanfrage.
- Alle Limits wirken und sind im Status sichtbar.
- Versionsmatrix getestet, ältere Programmversion zerstört keinen neueren Katalog.
- Telemetrie und Statusabfrage vorhanden und getestet.
- Sicherheitstests grün.
- Benchmarks innerhalb der Budgets oder Abweichung bestätigt.

## Risiken und Stolpersteine

- Test-Nahten können die Produktionslogik verformen; sie bleiben klein und hinter `cfg(any(test, feature = "mock"))`.
- Das Erkennen eines „vollen Datenträgers“ ist plattformabhängig; der Test arbeitet mit einem schreibgeschützten Zielverzeichnis statt mit echtem Speichermangel.
- Die Quarantäne verdoppelt vorübergehend den Plattenbedarf; Bereinigung nach erfolgreichem Neuaufbau ist Teil von L3.5 und hier zu verifizieren.
- Ein zu großzügiger `layer_version`-Reset vernichtet teure Embeddings; `migrate` bleibt die bevorzugte Antwort ab der zweiten Version.
