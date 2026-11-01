# Stufe 3 — Indexierungs-Pipeline

Stufe 3 baut die Seite des Layers, die Dateien in einen persistenten, modellgebundenen Vektorindex überführt: Dateiermittlung, Textzugriff und Offsets, Chunker, Katalog mit EntryId-Schema, IndexStore, Embedding-Pipeline und die Treiber (Wartungsmodi, Auto-Budget, Background). Alles ist gegen `FakeBackend` testbar; der Echtbetrieb folgt am Integrationspunkt IP1 (Stufe 5). Aufwand [W]: 6 Personentage.

## Eingang

Gate G2 bestanden. Bestätigt sein müssen D4, D5, D6, D7, D8, D9, D13 und D16. Fixtures und Gold-Set aus L0.4 sowie die Budgets B3 bis B9 aus L0.5 liegen vor. Der Charta zufolge gehören Chunking und der Persistenz-Zeitpunkt der Suchschicht (E42); die Engine liefert nur Embeddings und den Index-Baustein.

## Datenfluss

```
Anfrage  resident=...  residentIndex=auto|update|rebuild|status|off
   |
   v  walk (DirCache, globaler Filter)      -> sichtbare Dateien  [rel, abs, size, mtime_ns]
   v  plan (Katalog, FileStamp, Hash)       -> neu / geändert / unverändert / entfernt
   v  Priorisierung und Budget              -> Arbeitsliste
   v  lesen, dekodieren, chunken (rayon)    -> Chunks mit char-, byte- und Zeilen-Offsets
   v  embed (Backend, Rolle Passage, Batches) -> f32-Vektoren -> f16-Bytes
   v  store.replace_file / remove (ParquetIndex im Speicher)
   v  flush (atomar)  ->  catalog.commit (Shards)      Reihenfolge: Daten, Flush, Katalog
```

## Pakete

### L3.1 · Dateiermittlung und Eignung

**Zweck:** Die Menge der „sichtbaren“ Dateien unterhalb der Suchwurzel bestimmen, inklusive des globalen Include/Exclude-Filters, mit den Metadaten, die der Katalog braucht.  
**Quellen:** `layers/dir_cache.rs` (`DirCache::list` wendet den globalen Filter an, `FileInfo{name, mtime_ns, size}`, Dot-Einträge unsichtbar; E16), `grep_layer.rs::collect_files`, `trigram/layer.rs::collect_files` und `FileEntry` (E11), `core/query.rs::Query::resolve_search_root`.  
**Abhängt von:** L2.1, D8.  
**Zu klären:** Neuer gemeinsamer Helfer (empfohlen, bestehende Layer bleiben unangetastet) oder dritte Kopie; Verhalten bei Symlinks und Sonderdateien: `read_dir_listing` listet sie als „Dateien“ mit den Metadaten des Eintrags selbst, Verzeichnis-Symlinks werden nicht betreten (E52); Standardlisten für Endungen und Namensmuster, die nie indexiert werden.  
**Ergebnis:** `walk::visible_files(cache, root, rel_prefix) -> Vec<VisibleFile>` und `walk::eligible(&VisibleFile, &IndexSection) -> Eligibility`.

#### Skizze

```rust
pub struct VisibleFile {
    pub rel: String,
    pub abs: PathBuf,
    pub size: u64,
    pub mtime_ns: u64,
}

pub enum Eligibility {
    Ok,
    Empty,
    TooLarge,
    DeniedName,
    DeniedExtension,
}

pub fn visible_files(cache: &DirCache, root: &Path, rel_prefix: &str) -> Vec<VisibleFile>;
pub fn eligible(file: &VisibleFile, cfg: &IndexSection) -> Eligibility;
```

#### Aufgaben

- Iterativer Walk mit explizitem Stapel (wie `collect_files` in grep), je Verzeichnis ein `cache.list(dir)`; `rel` ist dokumentwurzelrelativ mit `/` (`join_rel`), damit Ergebnisse denselben Pfad tragen wie bei den anderen Layern.
- Ergebnis nach `rel` sortieren (deterministische Reihenfolge und stabile Tests).
- Eignung nur anhand der Metadaten: Größe null, über `max_file_bytes`, Namens- und Endungs-Denylist (`index.deny_globs`, `index.skip_extensions`; Defaults decken Bilder, Archive, Binärformate, Schlüsseldateien und Datenbanken ab, zum Beispiel `*.pem`, `*.key`, `*.p12`, `id_rsa*`, `*.sqlite`, `*.db`, `*.png`, `*.jpg`, `*.pdf`, `*.zip`, `*.gz`, `*.so`, `*.parquet`, `*.safetensors`).
- Statistik zurückgeben (`visible`, `eligible`, `skipped_by_reason`) für `detail.coverage`.
- Metriken `traversal_ms`, `dir_cache_misses`, `dir_cache_update_ms` wie bei grep und trigram in `status.detail` schreiben.

#### Abnahme L3.1

- Test `walk_lists_exactly_the_visible_files` auf dem Fixture-Korpus (kein Dot-Eintrag, korrekte Präfixe bei `directory`).
- Test `eligibility_reasons_are_reported`.
- Test `walk_is_deterministic`.
- Test `walk_lists_symlinks_as_files_without_traversal` (Verzeichnis-Symlink wird nicht betreten, Schleife terminiert, E52).

### L3.2 · Textzugriff und Offsets

**Zweck:** Dateien streng als UTF-8 lesen, Binärdateien erkennen und Zeichen-, Byte- und Zeilenpositionen ineinander umrechnen.  
**Quellen:** `resident/index/payload.rs::Payload` (Einheit der Offsets, E37), `trigram/normalize.rs::normalize_bytes` (Binärerkennung per NUL, als Vorbild), `resident/index/stamp.rs::content_hash` (E39).  
**Abhängt von:** L3.1, D4.  
**Zu klären:** Einheit der `char_*`-Offsets (Entscheidung D4: Unicode-Skalarwerte); Behandlung des BOM; CRLF (bleibt im Text, Zeilenzerlegung an `\n`).  
**Ergebnis:** `text::read_text` und `OffsetMap`; die Einheit ist dokumentiert und durch Tests gesichert.

#### Skizze

```rust
pub struct TextFile {
    pub text: String,
    pub content_hash: u64,
    pub bom_stripped: bool,
}

pub enum ReadError {
    Io(std::io::Error),
    Binary,
    NotUtf8, NotRegular,
    TooLarge,
    Empty,
}

pub fn read_text(path: &Path, max_bytes: u64) -> Result<TextFile, ReadError>;

pub struct OffsetMap { /* line starts in chars and bytes */ }

impl OffsetMap {
    pub fn new(text: &str) -> Self;
    pub fn line_of_char(&self, char_idx: u64) -> u32;      // 1-based
    pub fn byte_of_char(&self, char_idx: u64) -> usize;
    pub fn slice<'a>(&self, text: &'a str, char_start: u64, char_end: u64) -> &'a str;
}
```

#### Aufgaben

- `read_text`: Dateityp per `symlink_metadata` prüfen (nur reguläre Dateien; Symlinks, FIFOs, Sockets und Geräte werden abgelehnt, E52), Größe vor dem Lesen prüfen, Bytes lesen, NUL-Byte irgendwo im Inhalt bedeutet `Binary`, `String::from_utf8` streng (sonst `NotUtf8`), führendes U+FEFF entfernen und merken; Hash über die rohen Bytes (xxh3-64).
- `OffsetMap::new`: ein Durchlauf, Zeilenanfänge als Zeichen- und Byteindex; `slice` ohne erneutes Zählen über Binary Search.
- Offsets sind halb offen `[start, end)` in Unicode-Skalarwerten des BOM-bereinigten Textes; die Konvention steht im Moduldoku und im Katalog (`chunker_fp`).
- Einheit gegen die Engine absichern: Test, der ein `Payload` mit diesen Offsets in `ParquetIndex` schreibt, liest und mit `slice` den Originaltext rekonstruiert.

#### Abnahme L3.2

- Tests `offsets_roundtrip_multibyte` (Umlaute, CJK, Emoji), `offsets_with_crlf_and_bom`, `last_line_without_newline`, `binary_and_invalid_utf8_are_rejected`.
- Property-Test: Für jeden Zeichenindex `c` gilt `slice(c, c+1)` gleich dem `c`-ten Zeichen; `line_of_char` ist monoton.
- Test `payload_offsets_reconstruct_source`.

### L3.3 · Chunker

**Zweck:** Text in deterministische, versionierte Chunks zerlegen, die das 512-Token-Limit des Modells nicht überschreiten und sinnvolle Grenzen respektieren.  
**Quellen:** Charta (Chunking ist Suchschicht, E42), `tokenize/mod.rs` (stille Kürzung, E32), Messung M3, D4.  
**Abhängt von:** L3.2, L0.3 (M3).  
**Zu klären:** Endgültige Zeichenbudgets aus M3; Kopfzeile mit Pfad (A/B-Auswertung in S7); Sonderbehandlung für Markdown-Codeblöcke und sehr lange Zeilen.  
**Ergebnis:** `Chunker`-Trait, `LineAwareChunker`, Konstante `CHUNKER_VERSION`, Property-Tests.

#### Skizze

```rust
pub const CHUNKER_VERSION: u32 = 1;

pub struct Chunk<'a> {
    pub char_start: u64,
    pub char_end: u64,
    pub byte_start: usize,
    pub byte_end: usize,
    pub line_start: u32,
    pub line_end: u32,
    pub text: &'a str,
}

pub trait Chunker: Send + Sync {
    fn version(&self) -> u32;
    fn chunk<'a>(&self, text: &'a str, map: &OffsetMap) -> Vec<Chunk<'a>>;
    fn passage_text(&self, rel_path: &str, chunk: &Chunk<'_>) -> String;
}
```

#### Algorithmus

- Der Text wird in Zeilen mit Terminator zerlegt; Chunks bestehen aus ganzen Zeilen, außer eine einzelne Zeile überschreitet `max_chars`.
- Bruchkandidaten sind Zeilenenden mit Punktzahl 3 (Leerzeile oder nächste Zeile ist Überschrift oder Codeblock-Zaun), 2 (nächste Zeile beginnt bei Einrückung 0 mit `fn `, `pub `, `impl `, `struct `, `enum `, `trait `, `mod `, `def `, `class `, `async def `, `function `, `const `, `export ` oder `@`), 1 (jedes andere Zeilenende).
- Ein Chunk wächst bis `target_chars`; gebrochen wird beim Kandidaten mit der höchsten Punktzahl im Fenster `[min_chars, max_chars]`; bei Gleichstand zählt der letzte Kandidat vor `target_chars`, sonst der erste danach.
- Überlappung: Der nächste Chunk beginnt am ersten Zeilenanfang, der mindestens `overlap_chars` zurückreicht; bei Bruchpunktzahl 3 entfällt die Überlappung.
- Zeilen über `max_chars` werden am letzten Leerzeichen vor `max_chars` geteilt, ohne Leerzeichen an einer Zeichengrenze.
- Reine Whitespace-Chunks entfallen; `max_chunks_per_file` kappt die Zahl (das Ergebnis trägt `truncated = true`).
- `passage_text`: bei `context_header = path` ist die Kopfzeile `rel_path`, danach ein Zeilenumbruch und der Chunk-Text; die Offsets im `Payload` beziehen sich nur auf den Chunk-Text. Das Präfix `passage: ` setzt die Engine (L1.2).

#### Abnahme L3.3

- Property-Tests (`proptest`): Chunks sind geordnet, nicht leer, höchstens `max_chars` lang (Ausnahme: unteilbare Zeile ohne Leerzeichen wird an Zeichengrenzen gekappt), `text == &source[byte_start..byte_end]`, Zeichen- und Byteoffsets stimmen überein, jede Nicht-Whitespace-Stelle ist mindestens einmal überdeckt, Ergebnis ist deterministisch, nie ein Panic bei Multibyte-Zeichen.
- Tests `markdown_headings_start_new_chunks`, `code_items_start_new_chunks`, `very_long_line_is_split`, `overlap_only_within_paragraph`, `cap_marks_truncated`.
- Kalibrierungstest `chunks_stay_below_512_tokens` (`#[ignore]`, mit Modell): p99 der Chunk-Tokenlänge über den Repo-Korpus liegt unter 512; Ergebnis fließt in D4/L0.5 zurück.

### L3.4 · Katalog und EntryId-Schema

**Zweck:** Pro Datei festhalten, was indexiert wurde (Stempel, Hash, Chunkzahl, Chunker-Version), Dateinummern stabil vergeben und Änderungen zuverlässig erkennen.  
**Quellen:** `resident/index/stamp.rs` (`FileStamp`, `Verdict`, `content_hash`; E39), `core/persistence.rs::LayerStorage` (`put`, `get`, `keys`; E14), `trigram/index.rs` (Vorbild, aber ein `put` je Datei), D5, D7.  
**Abhängt von:** L3.2, L3.3.  
**Zu klären:** Shard-Zahl (256) und Serialisierung (JSON); Reservierungsgröße für `file_no` (256); Verhalten bei unlesbarem Shard (Katalog zurücksetzen und Neuaufbau).  
**Ergebnis:** `Catalog` mit `plan`, `commit`, `reset`, `allocate_file_no` sowie `entry_id`/`file_of`.

#### Skizze

```rust
pub const CHUNK_BITS: u32 = 20;

pub fn entry_id(file_no: u64, chunk_no: u32) -> EntryId {
    EntryId::new((file_no << CHUNK_BITS) | u64::from(chunk_no))
}

pub fn file_of(entry: EntryId) -> (u64, u32);

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FileRecord {
    pub file_no: u64,
    pub size: u64,
    pub mtime_ns: u64,
    pub ctime_ns: Option<u64>,
    pub content_hash: u64,
    pub chunks: u32,
    pub chunker_fp: u64,
    pub indexed_at_ns: u64,
    pub truncated: bool,
}

pub struct Plan {
    pub new: Vec<usize>,
    pub changed: Vec<usize>,
    pub touched: Vec<usize>,
    pub unchanged: usize,
    pub removed: Vec<String>,
}

impl Catalog {
    pub fn load(storage: &LayerStorage) -> Result<Self, ResidentLayerError>;
    pub fn plan(&self, visible: &[VisibleFile], now_ns: u64, chunker_fp: u64) -> Plan;
    pub fn allocate_file_no(&mut self, storage: &LayerStorage) -> Result<u64, ResidentLayerError>;
    pub fn upsert(&mut self, rel: String, rec: FileRecord);
    pub fn remove(&mut self, rel: &str) -> Option<FileRecord>;
    pub fn commit(&mut self, storage: &LayerStorage) -> Result<usize, ResidentLayerError>;
    pub fn reset(&mut self, storage: &LayerStorage) -> Result<(), ResidentLayerError>;
}
```

#### Persistenz

| Schlüssel in `LayerStorage` | Inhalt |
|---|---|
| `cat:meta` | `layer_version`, `chunker_fp`, `model_fingerprint`, `next_file_no`, `reserved_until`, `created_ns`, `last_root` |
| `cat:shard:00` bis `cat:shard:ff` | JSON-Objekt `{rel: FileRecord}`, Shard = `xxh3(rel) & 0xFF` |

#### Entscheidungslogik

- Ohne Katalogeintrag: `new`.
- `chunker_fp` (Fingerabdruck aus `CHUNKER_VERSION` und Chunk-Parametern) oder Modell-Fingerabdruck weichen ab: `changed` (Neuaufbau der Datei).
- Gleiche Größe: `FileStamp` des Eintrags (mit `scan_ns = indexed_at_ns`) gegen den aktuellen Stempel prüfen: `Unchanged` bedeutet `unchanged`, `NeedsHash` bedeutet lesen und Hash vergleichen; gleicher Hash ergibt `touched` (nur Stempel erneuern, `scan_ns` fortschreiben), anderer Hash ergibt `changed`.
- Abweichende Größe: `changed` ohne Hash.
- Katalogdateien, die unter der Suchwurzel liegen und nicht sichtbar sind, werden per `symlink_metadata` auf Existenz geprüft: fehlende sind `removed`, vorhandene (herausgefiltert) bleiben unberührt und werden nur aus dem Scope ausgeschlossen (D8).

#### Aufgaben

- `file_no` in Blöcken zu 256 reservieren: Der neue Wert `reserved_until` wird in `cat:meta` geschrieben, bevor eine Nummer aus dem Block vergeben wird; nach einem Absturz beginnt die Vergabe oberhalb von `reserved_until` (Nummern werden nie wiederverwendet).
- `commit` schreibt nur veränderte Shards und `cat:meta`; Aufrufer garantiert die Reihenfolge Daten, Flush, Katalog (L3.5, L3.6).
- Obergrenzen prüfen: `chunks <= min(max_chunks_per_file, 2^CHUNK_BITS - 1)`, `file_no < 2^(64 - CHUNK_BITS)`.
- Defekte Shards (JSON-Fehler) führen zu `ResidentLayerError::Catalog`; der IndexStore setzt Katalog und Index gemeinsam zurück (L3.5).

#### Abnahme L3.4

- Tests `entry_id_roundtrip_and_bounds`, `plan_classifies_new_changed_touched_unchanged_removed`, `racy_stamp_requires_hash`, `filtered_out_file_is_not_removed`, `shard_roundtrip_and_dirty_tracking`, `file_no_is_never_reused_after_crash`, `corrupt_shard_is_reported`.
- Messung `catalog_commit_10k_files` gegen Budget B6.

### L3.5 · IndexStore

**Zweck:** Lebenszyklus des `ParquetIndex`: Öffnen, Neuaufbau bei Inkompatibilität oder Beschädigung, dateiweises Ersetzen, Flush, Konsistenzabgleich mit dem Katalog, Schreibsperre.  
**Quellen:** `resident/index/parquet_io.rs` (E36), `resident/index/schema.rs` (`SchemaHeader`, E38), `resident/error.rs::IndexError`, D6, D16.  
**Abhängt von:** L3.4, L2.4 (`BackendInfo`).  
**Zu klären:** Quarantäne-Aufbewahrung (löschen nach erfolgreichem Neuaufbau); Sperre per `File::try_lock` (Toolchain prüfen, **[?]**) oder `libc::flock` unter Linux; `verify_on_open` nur in Wartungsmodi.  
**Ergebnis:** `IndexStore` mit `open`, `replace_file`, `remove_file`, `payload`, `flush`, `try_write_lock`, `reconcile`.

#### Skizze

```rust
pub struct IndexStore {
    index: Arc<ParquetIndex>,
    path: PathBuf,
    lock_path: PathBuf,
}

pub enum OpenOutcome {
    Fresh,
    Loaded { entries: u64 },
    Rebuilt { reason: RebuildReason },
}

pub enum RebuildReason { Incompatible, Corrupt, FingerprintChanged, CatalogMismatch }

impl IndexStore {
    pub fn open(
        storage: &LayerStorage,
        info: &BackendInfo,
        cfg: &IndexSection,
        verify: bool,
    ) -> Result<(Self, OpenOutcome), ResidentLayerError>;
    pub fn index(&self) -> Arc<ParquetIndex>;
    pub fn replace_file(&self, file_no: u64, old_chunks: u32, new: Vec<NewChunk>) -> Result<(), ResidentLayerError>;
    pub fn remove_file(&self, file_no: u64, chunks: u32);
    pub fn payload(&self, entry: EntryId) -> Option<Payload>;
    pub fn flush(&self) -> Result<bool, ResidentLayerError>;
    pub fn try_write_lock(&self) -> Option<WriteGuard>;
    pub fn reconcile(&self, catalog: &mut Catalog) -> Result<ReconcileReport, ResidentLayerError>;
}
```

#### Aufgaben

- `open`: Pfad über `storage.path_for("index.parquet")`; Kopf mit `SchemaHeader::from_spec(&info.spec, &info.revision, &info.tokenizer_sha)` (Spec mit Speicher-Dtype F16, E31); `IndexOptions { access: Mmap, verify, row_group_rows }`.
- Fehlerbehandlung: `IncompatibleSchema` führt zu Umbenennen nach `index.parquet.incompatible-<ts>` und frischem Index (`Rebuilt{Incompatible}`); `ChecksumMismatch`, `IndexUnavailable` oder Parquet-Lesefehler führen zu `...corrupt-<ts>` und `Rebuilt{Corrupt}`; Berechtigungsfehler sind echte Fehler. Nach erfolgreichem Neuaufbau samt Flush werden Quarantänedateien gelöscht.
- Weicht der Modell-Fingerabdruck in `cat:meta` ab, obwohl der Kopf passt, gilt das wie Inkompatibilität (Gurt und Hosenträger).
- `replace_file`: neue Chunks per `ParquetIndex::upsert(entry_id(file_no, i), f16_bytes, Payload::new(rel, char_start, char_end))`; überschüssige alte Chunk-Nummern (`>= new.len()`) per `remove` entfernen.
- `flush` ruft `ParquetIndex::flush`; die Rolle des Rückgabewerts `false` (nichts zu schreiben) bleibt erhalten.
- `reconcile`: günstiger Vergleich der Summe der Katalog-Chunks mit der Zahl der Stamps (`Index::stamps().count()`, ohne Vektoren zu laden); bei Abweichung Stamps nach `file_no` gruppieren: Waisen (nicht im Katalog) entfernen, Katalogdateien mit fehlenden Chunks aus dem Katalog nehmen (werden als neu geplant).
- Sperre: `index.lock` im Layer-Verzeichnis, nicht blockierend; gehalten nur während eines Indexierungslaufs; ohne Sperre läuft der Prozess read-only (`index_mode = read_only`).

#### Abnahme L3.5

- Tests `open_fresh_loaded_rebuilt_incompatible`, `open_corrupt_quarantines_and_rebuilds`, `fingerprint_change_triggers_rebuild`, `replace_file_removes_surplus_chunks`, `flush_is_noop_when_clean`, `reconcile_removes_orphans_and_requeues_missing`, `second_writer_gets_no_lock`, `lock_released_on_drop_and_on_panic`.
- Test `reopen_after_flush_returns_same_payloads` (Roundtrip über `ParquetIndex`).

### L3.6 · Embedding-Pipeline

**Zweck:** Chunks effizient und abbrechbar in Vektoren überführen, mit Backpressure und Fehlerpolitik, ohne die Konsistenz zwischen Index und Katalog zu gefährden.  
**Quellen:** `facade/resident.rs` (`submit_embedding`, `TaskHandle`), `model/slot.rs::encode_bytes` (f16-Konvertierung), `processor/config.rs::WorkerConfig`, Muster der grep-/trigram-Schleifen (`FuturesUnordered`, `START_CUTOFF_RATIO`).  
**Abhängt von:** L2.4, L3.3, L3.5.  
**Zu klären:** Batchgröße (CPU: etwa 16), In-Flight-Limit (zwei Batches je Worker); Wiederholungsstrategie bei retryable Fehlern; Dateien über mehrere Batches.  
**Ergebnis:** `EmbedPipeline::run` mit Fortschrittsbericht.

#### Skizze

```rust
pub struct EmbedPipeline<'a> {
    backend: &'a dyn SemanticBackend,
    batch_chunks: usize,
    max_in_flight: usize,
}

pub struct FileEmbedded {
    pub item: usize,
    pub vectors: Vec<Vec<u8>>,
}

impl EmbedPipeline<'_> {
    pub async fn run(
        &self,
        items: Vec<WorkItem>,
        cancel: &CancellationToken,
        deadline: Instant,
        sink: &mut (dyn FnMut(FileEmbedded) -> Result<(), ResidentLayerError> + Send),
    ) -> PipelineReport;
}
```

#### Aufgaben

- Chunks aller Arbeitseinheiten flach nach Datei geordnet in Batches teilen; ein Batch ist eine `backend.embed(texts, Role::Passage, deadline)`-Anfrage; höchstens `max_in_flight` Batches gleichzeitig (Semaphor); Reihenfolge wiederherstellen über `(item, chunk)`.
- Eine Datei gilt als fertig, wenn alle ihre Vektoren vorliegen; erst dann wird sie an den Writer (`sink`) gegeben, der `replace_file` und Katalog-Vormerkung ausführt.
- Jede Vektorzeile prüfen (Länge gleich `dim`, endlich); f16-Bytes über `encode_bytes(values, DType::F16)` oder `half::f16::from_f32(...).to_le_bytes()`.
- Abbruch: `cancel` und `deadline` vor jedem neuen Batch prüfen; laufende Batches abwarten oder fallen lassen (Drop eines `TaskHandle` storniert nicht begonnene Tasks, E19); unvollständige Dateien werden verworfen, nie teilweise geschrieben.
- Fehlerpolitik: retryable Fehler einmal nach 200 ms wiederholen; nicht retryable Engine-Fehler der Klassen `device` und `model` brechen den Lauf ab (`aborted`); andere Fehler markieren die betroffenen Dateien als `failed` (kein Katalogeintrag, nächster Lauf versucht erneut).
- Messwerte: `embed_ms`, `chunks_per_s`, `batches`, `retries`.

#### Abnahme L3.6

- Tests `order_is_restored_across_batches`, `file_is_emitted_only_when_complete`, `cancel_between_batches_leaves_no_partial_file`, `retryable_error_is_retried_once`, `device_error_aborts_the_run`, `wrong_dimension_marks_file_failed`, `backpressure_limits_in_flight_batches` (FakeBackend mit Zähler).

### L3.7 · Indexierungstreiber und Wartungsmodi

**Zweck:** Die Pipeline in vorhersagbare Betriebsarten gießen: kleines begrenztes Nachführen im Query-Pfad, expliziter Update/Rebuild, Statusabfrage und optionaler Background-Lauf.  
**Quellen:** D9, `core/engine.rs` (Background-Semantik, E4), `core/layer.rs::Layer::background`, `core/persistence.rs::LayerStorage::dir`.  
**Abhängt von:** L3.1 bis L3.6.  
**Zu klären:** Priorisierung der Arbeitsliste; Fortschrittsmeldung; Ableitung der Dokumentwurzel im Background (`BackgroundContext` hat keine `Query`, E2).  
**Ergebnis:** `LayerState::maintain` und die `background`-Implementierung.

#### Skizze

```rust
pub enum IndexMode { Off, Auto, Update, Rebuild, Status }

pub struct Budget {
    pub deadline: Instant,
    pub max_chunks: Option<usize>,
}

pub struct IndexReport {
    pub files_new: usize,
    pub files_changed: usize,
    pub files_removed: usize,
    pub files_failed: usize,
    pub chunks_written: usize,
    pub remaining_files: usize,
    pub flushed: bool,
    pub complete: bool,
    pub elapsed_ms: u64,
}

impl LayerState {
    pub async fn maintain(
        &self,
        visible: &[VisibleFile],
        mode: IndexMode,
        budget: Budget,
        cancel: &CancellationToken,
    ) -> Result<IndexReport, ResidentLayerError>;
}
```

#### Modi

| Modus | Verhalten | Budget |
|---|---|---|
| `off` | keine Indexpflege; Anfrage nutzt den vorhandenen Index | — |
| `auto` (Default) | geänderte und neue Dateien nachführen, bis Budget oder Chunk-Limit erreicht ist, danach abfragen | `maintenance.auto_budget_ms`, `auto_max_chunks` |
| `update` | alle sichtbaren neuen und geänderten Dateien indexieren, entfernte bereinigen; abbrechbar, Fortschritt periodisch geflusht | nur `query.budget_ms` und Abbruch |
| `rebuild` | Katalog zurücksetzen, neue Indexinstanz anlegen und neu anhängen (altes Handle freigeben), dann wie `update` | wie `update` |
| `status` | nichts schreiben; Abdeckung, Index-Zustand und Restmenge aus Katalog und Dateimetadaten melden; öffnet weder Parquet-Index noch Backend | — |

#### Aufgaben

- Priorisierung der Arbeitsliste: zuerst `changed` (konsistente Ergebnisse), dann `new`; innerhalb der Gruppen kürzlich geänderte Dateien zuerst, dann kleine vor großen.
- `auto`: Budget mit Deadline und Chunk-Limit; Abbruch ohne Fehler, `remaining_files` und `hint` im Status.
- Flush-Politik: nach `flush_every_chunks` oder `flush_every_ms` und am Laufende; direkt danach `catalog.commit`; ein Absturz dazwischen wird durch `reconcile` repariert (L3.5).
- `rebuild` fragt vor dem Zurücksetzen nichts ab (nicht interaktiv), loggt aber mit `warn` und meldet `index.rebuilt = true`.
- `status`: `coverage {visible, indexed, stale, excluded_stale}`, `index {entries, files}` aus dem Katalog, kein Schreibzugriff, keine Sperre, ohne Modell nutzbar.
- Background: `background_enabled = false` ist Standard; dann kehrt `background` sofort zurück (der CLI-Prozess wartet auf alle Background-Tasks, E4). Aktiviert, läuft eine Schleife `select!{cancel, sleep(interval)}` mit `maintain(Update, background_budget_ms)`; Cancel wirkt zwischen Dateien (Budget B9: höchstens 1 s).
- Wurzel im Background: `root = storage.dir.ancestors().nth(3)` (Layout `<root>/.xyrag/layers/resident`, E3); zusätzlich `cat:meta.last_root` aus dem letzten `run`; Test sichert die Ableitung ab.
- Genau ein Indexierer je Prozess (`tokio::sync::Mutex` im `LayerState`) plus die Dateisperre zwischen Prozessen.

#### Abnahme L3.7

- Tests `auto_stops_at_budget_and_reports_remaining`, `update_resumes_where_it_stopped`, `rebuild_resets_catalog_and_index`, `status_does_not_write`, `background_is_noop_by_default`, `background_stops_within_one_second_on_cancel`, `root_derivation_from_storage_dir`, `second_process_run_is_read_only`.

### L3.8 · Tests und Messung

**Zweck:** Stufe 3 mit dem Fake-Backend vollständig und reproduzierbar abnehmen.  
**Quellen:** `tests/trigram_e2e.rs` (Muster), Fixtures aus L0.4, Konventionen Q3.  
**Abhängt von:** L3.1 bis L3.7, L2.5.  
**Zu klären:** Fehlerinjektion zwischen Flush und Katalog-Commit über `resident::failpoint::hit("layer.commit.before_catalog", key)` (E54).  
**Ergebnis:** Unit-Tests unter `layers/resident_layer/test/` und Integrationstests in `tests/resident_layer_e2e.rs`.

#### Tests

- `initial_build_indexes_all_eligible_files`
- `second_run_is_a_noop_and_reuses_embeddings` (Zähler `embedded_texts` bleibt gleich)
- `modified_file_replaces_its_chunks_only`
- `deleted_file_is_pruned_and_excluded_from_scope`
- `renamed_file_is_delete_plus_add`
- `touch_without_change_does_not_reembed` (mtime neu, Inhalt gleich, Hash-Pfad)
- `budgeted_auto_run_stops_and_resumes`
- `cancel_mid_run_keeps_last_flush_consistent`
- `crash_between_flush_and_catalog_commit_recovers`
- `schema_mismatch_triggers_rebuild_and_reports_it`
- `second_writer_runs_read_only`
- `skipped_files_are_counted_by_reason` (zu groß, binär, kein UTF-8, Denylist)
- `max_chunks_total_cap_is_enforced`
- `ingest_rate_without_model_meets_budget_b4` (`#[ignore]`, Messung)

## Gate G3

- Erstaufbau und inkrementelle Pflege (neu, geändert, gelöscht, umbenannt, nur berührt) sind korrekt und idempotent.
- Absturz- und Abbruchfestigkeit: Flush und Katalog bleiben konsistent, `reconcile` repariert die Restfälle.
- Schemawechsel führt zu kontrolliertem Neuaufbau; zweiter Schreiber läuft read-only.
- Chunker-Properties, Offset-Roundtrips und Katalogtests sind grün; `ingest_rate` erfüllt B4, `catalog_commit` erfüllt B6.
- Kein Verhalten von grep, trigram, glob, merge verändert; `cargo test --features mock` grün.

## Risiken und Stolpersteine

- `ParquetIndex::flush` schreibt alle Zeilen (E36): Zu kurze Flush-Intervalle verursachen quadratischen Schreibaufwand. Politik aus D6 einhalten und M4 auswerten.
- `LayerStorage::put` ist je Aufruf eine Transaktion (E14): Der Katalog darf nie dateiweise geschrieben werden.
- Der Katalogcommit nach dem Flush macht Index-Einträge ohne Katalog möglich (Absturz); `reconcile` muss billig sein, sonst wird es zum Startproblem im One-Shot-CLI.
- Zeitstempelauflösung: `FileInfo` liefert nur `mtime_ns` und kein `ctime`; Hash-Fallback ist dafür unverzichtbar (E39).
- Große Dateien im Grenzbereich von `max_file_bytes` erzeugen viele Chunks (Cap `max_chunks_per_file`): `truncated` muss im Status sichtbar bleiben.
- Der Background-Lauf teilt sich Kerne mit Anfragen anderer Prozesse; niedrige Priorität (`cpu_nice` wirkt nur mit GPU-Workern, E50) ist auf der CPU-Seite nicht verfügbar; `candle_rayon_threads` begrenzen (D15).
