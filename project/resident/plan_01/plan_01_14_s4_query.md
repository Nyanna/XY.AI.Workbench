# Stufe 4 — Query-Pfad

Stufe 4 implementiert die Anfrageseite des Layers: Optionen und Validierung, Sichtbarkeit und Scope, Query-Embedding und Similarity über die Backend-Naht, Abbildung der Treffer auf Dateien und Zeilen, Ergebnisfelder, Budgets und Fehlerpfade. Die Tests laufen gegen `FakeBackend`, ab dem Integrationspunkt IP1 auch gegen das echte Backend. Aufwand [W]: 4 Personentage.

## Eingang

Gate G3 bestanden. Bestätigt sein müssen D8, D10, D13, D18, D19 und D20. Der Ergebnisvertrag richtet sich nach den ausgelieferten Layern (`File`, `Lines`) und dem `MergeLayer` (E12, E13).

## Ablauf

```
run(query, result_set, ctx)
  1  Felder lesen und validieren            QueryOptions
  2  Konfiguration und LayerState (lazy)    skipped bei Fehlern (L2.6)
  3  walk (visible_files)                    sichtbare Dateien samt Metadaten
  4  maintain(mode, budget)                  auto: kleiner Nachführlauf; status und off: nichts
  5  Scope bestimmen                         Katalog geschnitten mit sichtbar und nicht veraltet
  6  embed(query, Role::Query)               Budget: embed_budget
  7  search(index, qvec, scope, top_k)       Budget: search_budget
  8  Treffer abbilden                        Payload zu Datei, Zeilen, Excerpt; Frische-Prüfung
  9  ResultEntry schreiben                   ein Eintrag je Datei, nach bester Wertung
 10  LayerStatus füllen                      detail laut Schema (L2.3)
```

## Pakete

### L4.1 · Query-Optionen und Validierung

**Zweck:** Die Anfragefelder des Layers eindeutig lesen, begrenzen und mit der Konfiguration zusammenführen.  
**Quellen:** `core/query.rs` (`Query::{has, get, get_str, get_usize, resolve_search_root}`; E6), D1, D9, D19, `bin/xyrag.rs::parse_query` (CLI liefert alle Werte als Strings).  
**Abhängt von:** L2.2, L2.6.  
**Zu klären:** Obergrenzen (`maxResults` höchstens 200, `residentTopK` höchstens 400, Anfragetext höchstens 2000 Zeichen); Verhalten bei ungültigen Werten (`skipped` mit `reason = invalid_field` statt stillem Ignorieren).  
**Ergebnis:** `QueryOptions::from_query(query, cfg)`.

#### Skizze

```rust
pub struct QueryOptions {
    pub text: String,
    pub query_truncated: bool,
    pub max_results: usize,
    pub top_k: usize,
    pub min_score: Option<f32>,
    pub index_mode: IndexMode,
    pub search_root: PathBuf,
    pub rel_prefix: String,
}

impl QueryOptions {
    pub fn from_query(query: &Query, cfg: &ResidentLayerConfig) -> Result<Self, ResidentLayerError>;
}
```

#### Felder

| Feld | Typ | Default | Bedeutung |
|---|---|---|---|
| `resident` | Text | — | Anfragetext, aktiviert den Layer |
| `directory` | Pfad | Dokumentwurzel | Suchwurzel (gemeinsam mit anderen Layern, `resolve_search_root`) |
| `maxResults` | Zahl | `query.default_max_results` (20) | höchstens so viele Chunk-Treffer insgesamt |
| `residentTopK` | Zahl | `clamp(maxResults * top_k_factor, maxResults, 200)` | Kandidaten für Similarity |
| `residentMinScore` | Zahl 0 bis 1 | keiner | untere Schwelle der Cosine-Wertung |
| `residentIndex` | Text | `auto` | `auto`, `update`, `rebuild`, `status`, `off` |

#### Aufgaben

- Numerische Felder über `get_usize` bzw. Parsen von `Value::Number` und `Value::String` lesen (CLI übergibt Strings); `residentMinScore` als `f32` mit Bereichsprüfung.
- Anfragetext trimmen, mehrfache Whitespace-Folgen normalisieren; über 2000 Zeichen kürzen und `query_truncated` melden.
- Ungültige Felder führen zu `ResidentLayerError::Config{reason}` und damit zu `skipped / invalid_field`; unbekannte `resident*`-Felder zählen in `ignored_fields`.
- `status` läuft ohne Backend und ohne geöffneten Parquet-Index (liest nur Katalog und Dateimetadaten); `off` verzichtet nur auf die Indexpflege, die Anfrage selbst braucht weiterhin das Backend.

#### Abnahme L4.1

- Tests `defaults_are_applied`, `numeric_strings_from_cli_are_parsed`, `bounds_are_enforced`, `invalid_index_mode_is_reported`, `long_query_is_truncated_and_flagged`, `directory_resolves_prefix`.

### L4.2 · Sichtbarkeit und Scope

**Zweck:** Den Suchraum der Anfrage bestimmen: genau die sichtbaren, indexierten und nicht veralteten Dateien.  
**Quellen:** D8, `walk::visible_files` (L3.1), `Catalog` (L3.4), `resident/index/mod.rs` (`Scope`, `EntrySet::from_ranges`; E40), `resident/manager/scope.rs`.  
**Abhängt von:** L3.1, L3.4, L4.1.  
**Zu klären:** `ParquetIndex::capabilities()` ist `[Range]`, `Scope::Set` und `Scope::All` werden direkt aufgelöst (E40); Obergrenze der Set-Größe.  
**Ergebnis:** `search::build_scope(visible, catalog) -> ScopePlan`.

#### Skizze

```rust
pub struct ScopePlan {
    pub scope: Option<Scope>,
    pub visible: usize,
    pub indexed: usize,
    pub stale: usize,
    pub excluded_stale: usize,
    pub files: Vec<ScopedFile>,
}

pub fn build_scope(visible: &[VisibleFile], catalog: &Catalog) -> ScopePlan;
```

#### Aufgaben

- Für jede sichtbare Datei mit Katalogeintrag: Größe und mtime gleich dem Eintrag (oder `touched`-Hash bestätigt) bedeutet im Scope, sonst `stale` und aus dem Scope ausgeschlossen (D20).
- `Scope::All` nur, wenn jede Katalogdatei im Scope liegt (sonst würden herausgefilterte Dateien mitgezählt); andernfalls `Scope::Set(EntrySet::from_ranges(...))` mit einem Bereich `[file_no << 20, (file_no << 20) + chunks)` je Datei.
- Leerer Scope: kein Embedding, kein Search-Aufruf; Status `ran`, `contributions = 0`, `hint` (Index leer oder keine passende Abdeckung, mit Hinweis auf `residentIndex=update`).
- Ergebnis liefert die Zählerwerte für `detail.coverage`.

#### Abnahme L4.2

- Tests `all_scope_when_everything_visible_and_fresh`, `directory_limits_scope_to_subtree`, `stale_files_are_excluded_and_counted`, `empty_scope_skips_search_with_hint`, `set_scope_ranges_match_catalog`.
- Filtertest (eigener Prozess, `tests/resident_layer_filter.rs`): `global_exclude_removes_files_from_scope` — die Konfiguration des globalen Filters ist prozessweit und einmalig (`configure_global_filter`), daher getrennte Testdatei.

### L4.3 · Embedding und Similarity

**Zweck:** Die beiden Backend-Aufrufe mit Budgets, Fehlerabbildung und Statistik ausführen.  
**Quellen:** `SemanticBackend` (L2.4), `SimilarityReport`-Statistik (E21), D13, D18, D19, `grep_layer.rs` (Budgetmuster `MAX_RUNTIME`).  
**Abhängt von:** L2.4, L4.2.  
**Zu klären:** Aufteilung der Budgets (Kaltstart steckt im ersten Embedding); Verhalten bei Teiltreffern nach Timeout; ob `precision` pro Aufruf oder pro Backend gesetzt wird (Entscheidung D18: pro Backend aus der Konfiguration).  
**Ergebnis:** `search::query_vector` und `search::similar`.

#### Aufgaben

- `Deadline`-Hilfstyp: Gesamtbudget `query.budget_ms`; je Phase `min(restlich, Phasenbudget)`; Phasen `embed_budget_ms` (inklusive Modell-Kaltstart) und `search_budget_ms`.
- Query-Embedding: `backend.embed(vec![text], Role::Query, deadline)`; genau ein Vektor, Länge gleich `dim`, endlich.
- Index-Handle: `backend.attach_index(store.index())` einmal je Prozess (`OnceCell` im `LayerState`).
- Suche: `backend.search(&handle, qvec, scope, top_k, deadline)`; Kandidaten, `discarded`, `outdated` und Statistik (`s1`, `sk`, `gap_top2`, `z_top1`) nach `detail.search`.
- Fehler: Timeout im Embedding bei kaltem Modell meldet `aborted / timeout` mit `hint = "model cold start, retry"`; Engine-Fehler gemäß Grundtabelle (L2.3); in allen Fällen behalten bereits geschriebene Einträge anderer Layer Bestand.
- Cancel: Wird die `run`-Future verworfen, storniert der Drop der `TaskHandle`s nicht begonnene Tasks (E19); kein zusätzlicher Code, aber ein Test.

#### Abnahme L4.3

- Tests `budget_split_is_respected`, `cold_start_timeout_is_aborted_with_hint`, `engine_error_maps_to_status`, `dropped_run_future_cancels_tasks`, `statistics_are_reported`.

### L4.4 · Trefferabbildung

**Zweck:** Chunk-Treffer in nutzbare Dateitreffer überführen: Datei, Zeilenbereich, Excerpt, mit Frische- und Sichtbarkeitsprüfung.  
**Quellen:** `Payload` (E37), `OffsetMap` (L3.2), `trigram/layer.rs::truncate_line_middle` und `MAX_LINE_LEN` (Muster für die Zeilenkürzung), `DirCache` (`STALE_AFTER`, E16), D10, D20.  
**Abhängt von:** L3.2, L4.3.  
**Zu klären:** Dateimetadaten des `DirCache` können bis `STALE_AFTER` veraltet sein; für die Trefferdateien daher frisch statten. Anzahl Chunk-Treffer je Datei (Konfigurationswert `query.max_chunks_per_file`, Default 3).  
**Ergebnis:** `render::map_hits(...) -> Vec<FileHit>`.

#### Skizze

```rust
pub struct FileHit {
    pub rel: String,
    pub best_score: f32,
    pub chunks: Vec<ChunkHit>,
}

pub struct ChunkHit {
    pub score: f32,
    pub line_start: u32,
    pub line_end: u32,
    pub excerpt_lines: Vec<(u32, String)>,
}
```

#### Aufgaben

- Je `BackendHit` (beste zuerst): `(file_no, chunk_no) = file_of(entry)`; `rel` über die Umkehrabbildung des Katalogs; unbekannte `file_no` wird verworfen und als `dropped_unknown` gezählt.
- Payload: `hit.source` verwenden, sonst `store.payload(entry)` (Rückfall bei fehlender Engine-Unterstützung, L1.5); ohne Payload verwerfen.
- Sichtbarkeit und Frische: Datei muss sichtbar sein; vor dem Lesen `fs::metadata` (frisch, nicht aus dem `DirCache`) und Vergleich von Größe und mtime mit dem Katalog; Abweichung bedeutet `stale` und gemäß D20 verwerfen (`dropped_stale`) und die Datei für die nächste Runde vormerken; mit `keep_stale = true` markieren (`Stale: true`).
- `residentMinScore` anwenden.
- Dateien einmal lesen (`read_text`), `OffsetMap` bilden; `payload.char_end` größer als die Zeichenzahl bedeutet `dropped_stale`; Zeilenbereich `line_of_char(char_start)` bis `line_of_char(char_end - 1)`.
- Excerpt: die ersten `excerpt_lines` nicht leeren Zeilen des Chunks, rechts getrimmt (`\r` entfernen), Tabs zu einem Leerzeichen, auf `excerpt_max_chars` mit Mittenkürzung begrenzt (eigene Hilfsfunktion nach Vorbild von `truncate_line_middle`).
- Dateizugriffe gebündelt in einem `ctx.cpu.spawn`-Block (rayon `par_iter` über die höchstens `max_results` Trefferdateien).
- Gruppieren nach Datei, je Datei höchstens `max_chunks_per_file` Chunks, nach Wertung sortiert; Dateien nach bester Wertung (Gleichstand: Pfad).

#### Abnahme L4.4

- Tests `maps_chunk_to_correct_lines` (inklusive Umlaute, CJK, Emoji, CRLF), `excerpt_is_first_nonblank_line_and_truncated`, `stale_hit_is_dropped_by_default`, `stale_hit_is_flagged_with_keep_stale`, `deleted_between_search_and_map_is_dropped`, `payload_fallback_via_store`, `per_file_cap_and_ordering`.

### L4.5 · Ergebnisse und Status

**Zweck:** Treffer als `ResultEntry` im Format der übrigen Layer ausgeben und den Status vollständig füllen.  
**Quellen:** `core/result.rs` (`ResultEntry::new`, `ResultSet::add`; E12), `grep_layer.rs::run` (Ausgabeschleife), D10, L2.3 (Detail-Schema).  
**Abhängt von:** L4.4.  
**Zu klären:** Rundung der Scores (vier Nachkommastellen); Reihenfolge der Einträge im `ResultSet` (Einfügereihenfolge ist die einzige Ordnung, E1).  
**Ergebnis:** `render::write_entries` und `search::fill_status`.

#### Beispiel

```json
{
  "File": "docs/de/espresso.md",
  "Lines": { "12": "## Mahlgrad und Brühzeit" },
  "Resident": {
    "Score": 0.8421,
    "Rank": 1,
    "Chunks": [ { "Start": 12, "End": 31, "Score": 0.8421 } ]
  }
}
```

#### Aufgaben

- Je Datei ein `ResultEntry::new(None, fields)` mit `File`, `Lines` (Zeilennummer als String-Schlüssel, aufsteigend) und `Resident` (`Score`, `Rank` ab 1, `Chunks[{Start, End, Score}]`, bei `keep_stale` zusätzlich `Stale`).
- `status.contributions` ist die Zahl der geschriebenen Dateieinträge; `status.ran = true`; `status.aborted = true`, wenn die Deadline während der Abbildung überschritten wurde.
- `detail` laut L2.3 füllen: `coverage`, `index`, `search`, `timings_ms`, `backend`, `model`, `root`, `index_mode`, dazu `dir_cache_misses` und `dir_cache_update_ms` (Delta wie bei grep).
- Insgesamt höchstens `maxResults` Chunk-Treffer ausgeben; die Kandidatenmenge `top_k` ist größer, damit Verwerfungen aufgefangen werden.
- Keine Panics bei leeren Mengen; leere Ergebnisse sind regulär (`ran = true`, `contributions = 0`).

#### Abnahme L4.5

- Tests `entries_have_file_lines_and_resident_object`, `scores_are_rounded`, `max_results_caps_chunk_hits_not_files`, `empty_result_is_regular`, `merge_layer_accepts_entries` (Zusammenspiel mit `MergeLayer`, siehe L5.2).

### L4.6 · Tests der Stufe

**Zweck:** Den Query-Pfad Ende zu Ende gegen `FakeBackend` und anschließend mit Fehlerinjektion abnehmen.  
**Quellen:** `tests/trigram_e2e.rs` (Muster), Gold-Set (L0.4), `FaultScript` (L2.5).  
**Abhängt von:** L4.1 bis L4.5.  
**Zu klären:** Welche Tests laufen mit `FakeBackend` (Default mit `mock`) und welche zusätzlich mit dem echten Backend (`ignored` ohne Modell, IP1)?  
**Ergebnis:** `tests/resident_layer_e2e.rs`, `tests/resident_layer_filter.rs`, Unit-Tests unter `layers/resident_layer/test/`.

#### Tests

- `finds_lexically_related_chunk_and_maps_lines`
- `directory_field_limits_results`
- `global_include_exclude_filters_results` (eigene Datei, eigener Prozess)
- `deleted_file_is_not_returned`, `modified_file_is_reindexed_in_auto_mode`, `stale_hits_dropped_when_budget_exhausted`
- `max_results_and_top_k_are_respected`, `grouping_per_file_and_line_order`
- `timeout_in_embed_is_aborted_and_other_layers_are_unaffected` (parallel mit grep)
- `engine_error_is_skipped_or_aborted_per_table`
- `empty_index_reports_coverage_zero_and_hint`
- `unicode_and_crlf_files_map_to_correct_lines`
- `second_query_is_warm_and_does_not_reembed` (Zähler)
- `two_concurrent_queries_share_state_and_initialise_once`
- `status_mode_reports_without_writing`
- `gold_lexical_recall_at_5_meets_budget_b10` (FakeBackend)
- `dropped_run_future_cancels_backend_tasks`

## Gate G4

- Query-Pfad Ende zu Ende mit `FakeBackend` grün; Scope, Filter, Frische und Staleness-Politik getestet.
- Budgets und Fehlerpfade (Timeout, Engine-Fehler, Cancel) übersetzen sich wie in der Grundtabelle in `LayerStatus`; andere Layer bleiben unberührt.
- Ergebnisformat `File`, `Lines`, `Resident` ist stabil (Golden-Test) und `MergeLayer`-kompatibel.
- Gold-Set lexikalisch erreicht B10 (Recall@5 mindestens 0,9).
- Keine Panics in Fehlerpfaden (Fuzz-/Property-Test auf Payload-Offsets außerhalb der Datei).

## Risiken und Stolpersteine

- Die Metadaten des `DirCache` können bis `STALE_AFTER` veraltet sein: Änderungen innerhalb dieses Fensters bleiben dem Planer verborgen. Die Frische-Prüfung beim Abbilden verhindert falsche Excerpts, nicht aber verpasste Neuindexierung bis zur nächsten Anfrage; als Verhalten dokumentieren.
- e5-Scores liegen in einem engen Band (E21); absolute Schwellen sind heikel. `residentMinScore` ohne Kalibrierung nur als Expertenoption bewerben (D19).
- Zeilenzuordnung hängt an der Einheit der Offsets (E37); die Tests mit Mehrbyte-Zeichen sind Pflicht und dürfen nicht auf ASCII reduziert werden.
- Viele Trefferdateien verursachen viele Dateizugriffe pro Anfrage; die Obergrenzen (`maxResults`, `max_chunks_per_file`) begrenzen das, und das Lesen läuft gebündelt in einem Rayon-Block.
- Eine leere Antwort bei leerem Index ist für Nutzer verwirrend; `hint` und `coverage` im Status sind Teil der Abnahme, nicht Beiwerk.
