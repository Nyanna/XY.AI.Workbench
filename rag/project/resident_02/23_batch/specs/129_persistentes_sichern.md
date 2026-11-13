# Spezifikation: Persistentes Sichern (Modul `index_save`, Build-Order 129)

## 1. Aufgabe, Einschränkungen, Grenzen

**Aufgabe:** Das Modul schreibt einen konsistenten Snapshot eines Index atomar als Parquet-Indexdatei. Der Ablauf ist Temp-Datei, fsync, Rename, fsync des Verzeichnisses. Gleichzeitige Saves werden serialisiert.

**Nicht Teil des Moduls:**
- Das Spalten-Schema und der Header (`index_format`).
- Die Checksum-Berechnung (`entry_payload`).
- Die Stamp-Vergabe (`change_stamp`).
- Das Lesen und Öffnen von Index-Dateien (`index_open`).
- Dirty-Verwaltung, Publication und Signal. Sie gehören dem Index, der `SnapshotSource` implementiert.
- Retry, Scheduling und Trigger für Saves.

**Crates:** `parquet` und `arrow` (nur die bereits in der `Cargo.toml` stehenden), `tracing`, `thiserror`, `std`. Neue Crates sind verboten. Interne Module: `index_format`, `entry_payload`, `change_stamp`, `index_contract`.

Das Modul ist blockierend. Async-Aufrufer rufen `save` über `page_advice::read_blocking` auf.

## 2. Kontext

- Ein Index hält seinen Zustand aus der Ursprungsdatei (`LazyIndexFile`, lazy) und aus Änderungen im Speicher. Er führt ein Dirty-Merkmal.
- Dieses Modul bekommt den Zustand über die Schnittstelle `SnapshotSource`. Sie liefert einen in sich konsistenten Snapshot, die Ursprungsdatei und den Speicher bereits zusammengeführt.
- Nach erfolgreichem Schreiben meldet das Modul dem Index den Snapshot-Token. Der Index löscht damit nur den Dirty-Zustand, der zum Snapshot gehört. Änderungen, die während des Schreibens eintrafen, bleiben dirty.
- Die geschriebene Datei wird von `index_open` gelesen: Footer-Header, Schema laut `index_format`, IDs aufsteigend ohne Duplikate, Checksums unverändert.

## 3. Interface

```rust
use std::path::{Path, PathBuf};
use index_contract::EntryId;
use change_stamp::ChangeStamp;
use entry_payload::EntryData;
use index_format::IndexHeader;

/// Opaker Marker des Zustands, den ein Snapshot abbildet.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SnapshotToken(pub u64);

/// Ein Entry im Snapshot.
#[derive(Debug, Clone)]
pub struct SnapshotEntry {
    pub id: EntryId,
    pub stamp: ChangeStamp,
    /// vector (FP16 LE, dimension*2 Bytes), source und gespeicherte Checksum.
    pub data: EntryData,
}

/// Konsistenter Snapshot. `entries` ist aufsteigend nach `id` sortiert, ohne Duplikate.
#[derive(Debug, Clone)]
pub struct Snapshot {
    pub token: SnapshotToken,
    /// Rohwert des aktuellen Max-Stamps (StampCounter::max_raw(), None -> 0).
    pub max_stamp: u64,
    pub entries: Vec<SnapshotEntry>,
}

/// Vom Index implementiert. Send + Sync.
pub trait SnapshotSource: Send + Sync {
    /// Atomar und konsistent. `None`, wenn nichts geändert ist (nicht dirty).
    /// Liest Entries aus Ursprungsdatei und Speicher. Ändert den Dirty-Zustand nicht.
    fn snapshot(&self) -> Option<Snapshot>;
    /// Wird nur nach erfolgreichem Schreiben aufgerufen.
    /// Der Index löscht den Dirty-Zustand nur für Änderungen bis `token`.
    /// Spätere Änderungen bleiben dirty.
    fn mark_saved(&self, token: SnapshotToken);
}

#[derive(Debug, Clone)]
pub struct SaveConfig {
    /// Zieldatei. Das übergeordnete Verzeichnis existiert.
    pub path: PathBuf,
    /// Header des Index (Modellbindung). `max_stamp` wird beim Schreiben überschrieben.
    pub header: IndexHeader,
    /// Zeilen je Row-Group, >= 1.
    pub row_group_rows: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SaveOutcome {
    /// Nichts geändert, nichts geschrieben.
    Unchanged,
    /// Datei ersetzt.
    Written { rows: u64, row_groups: usize },
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum SaveError {
    /// "Index nicht verfügbar": jeder Fehler beim Schreiben.
    #[error("Index nicht verfügbar: {path}: {cause}")]
    Unavailable { path: PathBuf, cause: String },
}

/// Serialisiert Saves. Send + Sync. Alle Methoden &self.
pub struct IndexSaver { /* config, Arc<dyn SnapshotSource>, Mutex<()> */ }

impl IndexSaver {
    pub fn new(config: SaveConfig, source: std::sync::Arc<dyn SnapshotSource>) -> IndexSaver;
    pub fn path(&self) -> &Path;
    /// Blockierend. Siehe §4.
    pub fn save(&self) -> Result<SaveOutcome, SaveError>;
}
```

`IndexSaver`, `SaveOutcome` und `SaveError` sind `Send + Sync + 'static` (Test per static assertion).

## 4. Verhalten und Invarianten

**Serialisierung**
- `save` nimmt zuerst einen exklusiven Lock (`Mutex<()>`). Der Lock wird bis nach `mark_saved` bzw. bis zum Fehlerende gehalten. Gleichzeitige Saves laufen nacheinander.
- `snapshot()` wird erst unter dem Lock aufgerufen. Ein wartender Save sieht deshalb den Zustand nach dem vorherigen Save. Ist er dann nicht mehr dirty, liefert er `Unchanged`.
- Ein Lock-Poisoning ist kein Panic. Der Lock wird weiterverwendet (`into_inner`).

**Ohne Änderung**
- Liefert `snapshot()` `None`, kommt `Ok(Unchanged)`.
- Es wird nichts erzeugt oder verändert (keine Temp-Datei, kein Rename). `mark_saved` wird nicht aufgerufen.

**Mit Änderung**
- Der Snapshot wird einmal gezogen und vollständig geschrieben.
- Änderungen, die danach eintreffen, gehören nicht zur Datei. Sie bleiben dirty, weil `mark_saved(snapshot.token)` nur den Stand bis zum Token quittiert.
- Bei Erfolg kommt `Ok(Written { rows, row_groups })`. Danach wird genau einmal `mark_saved(token)` aufgerufen.
- Bei Fehler wird `mark_saved` nicht aufgerufen. Der Index bleibt dirty.

**Dateiinhalt**
- Eine Zeile je `SnapshotEntry`, aufsteigend nach `entry_id`. Eine Zeile je Entry, ohne Duplikate.
- Spalten laut `index_format::column_schema(header.dimension)` in dieser Reihenfolge: `entry_id` ← `id.0`, `change_stamp` ← `stamp.as_raw()`, `vector` ← `data.vector` (FP16 LE, als Float16-Elemente der FixedSizeList), `target_file`, `char_start`, `char_end` ← `data.source`, `checksum` ← `data.checksum()`.
- Stamps, Vektorbytes, Verweise und Checksums werden unverändert übernommen. Es gibt keine Neuberechnung, kein `recompute` und kein Neuvergeben von Stamps.
- Footer-Key-Value: `header.with_max_stamp(snapshot.max_stamp).to_kv()` (15 Paare). Zusätzliche Keys wie `ARROW:schema` des Parquet-Writers sind erlaubt.
- Ist die Länge eines Vektors ungleich `header.vector_bytes()`, wird nicht geschrieben (Fehler, siehe unten).
- Ein Snapshot mit 0 Entries ist gültig. Es entsteht eine Datei mit Schema und Header, 0 Zeilen und 0 Row-Groups.
- Parquet-Writer-Eigenschaften:
  - Maximale Row-Group-Größe `row_group_rows`. Jede Row-Group außer der letzten hat genau `row_group_rows` Zeilen.
  - `Compression::UNCOMPRESSED` für alle Spalten.
  - Statistiken auf Page-Ebene (`EnabledStatistics::Page`).
  - Dictionary-Encoding für das Blatt der Spalte `vector` ausgeschaltet. Der Implementierer bestimmt den `ColumnPath` des Blattes aus dem Arrow-Schema und prüft ihn im Test über die Datei-Metadaten.
  - Header als Key-Value-Metadaten im Footer.

**Atomares Schreiben**
1. Die Temp-Datei liegt im selben Verzeichnis wie das Ziel, Name `<Zieldateiname>.tmp`. Sie wird neu angelegt, mit Truncate. Eine Temp-Datei eines früheren Absturzes wird überschrieben.
2. Alle Row-Groups schreiben und den Writer schließen (Footer).
3. `File::sync_all` auf der Temp-Datei.
4. `rename(temp, ziel)` ersetzt das Ziel atomar.
5. Das übergeordnete Verzeichnis öffnen und `sync_all` darauf ausführen.
- Das Ziel ist zu jedem Zeitpunkt entweder die vollständige alte oder die vollständige neue Datei.

**Fehler**
- Jeder Fehler in Schema, Vektorlänge, Arrow-/Parquet-Schreiben, I/O, fsync, Rename oder Verzeichnis-fsync ergibt `SaveError::Unavailable { path, cause }`. `path` ist der Zielpfad, `cause` die Ursache im Klartext. Es gibt keinen Panic.
- Bei einem Fehler vor dem Rename wird die Temp-Datei gelöscht (best effort, ein Lösch-Fehler wird nur geloggt) und das Ziel bleibt unangetastet.
- Schlägt der Rename fehl, wird die Temp-Datei gelöscht und das Ziel bleibt unangetastet.
- Schlägt nur das fsync des Verzeichnisses nach dem Rename fehl, ist das Ziel bereits die neue Datei. Es wird trotzdem `Unavailable` gemeldet, `mark_saved` entfällt und der Index bleibt dirty.
- Fehler werden mit `error!` (Pfad, Ursache) geloggt.
- Es gibt keinen Retry. Der nächste `save` beginnt neu.

**Logging**
- Bei `Written` genau ein `info!` mit Pfad, Schema-Version (`header.schema_version`), Modell (`model_id`, `model_revision`), Zeilenzahl und Row-Group-Zahl.
- Bei `Unchanged` kein `info!` (höchstens `debug!`).

**Nebenläufigkeit**
- `save` kann von beliebig vielen Threads gleichzeitig aufgerufen werden.
- Ein Save blockiert Writer des Index nicht. Das Halten des Saver-Locks verhindert keine Änderungen am Index. `snapshot()` verantwortet die kurze Konsistenz-Sperre im Index.
- Der Saver lockt nicht den Index.

## 5. Relied-on elements

```rust
// index_format (verbatim)
pub fn column_schema(dimension: usize) -> Result<Schema, FormatError>;
impl IndexHeader {
    pub fn with_max_stamp(&self, max_stamp: u64) -> IndexHeader;
    pub fn to_kv(&self) -> Vec<(String, String)>;   // genau 15 Paare
    pub fn vector_bytes(&self) -> usize;            // dimension * 2
    pub fn column_schema(&self) -> Result<Schema, FormatError>;
    // Felder u. a.: schema_version: u64, model_id: String, model_revision: String, dimension: usize
}
pub const COL_ENTRY_ID: &str = "entry_id";  // sowie COL_CHANGE_STAMP, COL_VECTOR, COL_TARGET_FILE,
                                            // COL_CHAR_START, COL_CHAR_END, COL_CHECKSUM
// Spalten (alle nullable=false): entry_id UInt64, change_stamp UInt64,
// vector FixedSizeList(Field("item", Float16, false), dim), target_file Utf8,
// char_start UInt64, char_end UInt64, checksum UInt64.
// Header im Footer als Key-Value; unbekannte Zusatz-Keys wie "ARROW:schema" werden beim Lesen ignoriert.

// entry_payload (verbatim)
pub struct SourceRef { pub target_file: String, pub char_start: u64, pub char_end: u64 }
pub struct EntryData { pub vector: SharedBytes, pub source: SourceRef, /* checksum privat */ }
impl EntryData { pub fn checksum(&self) -> u64; }

// change_stamp (verbatim)
impl ChangeStamp { pub fn as_raw(&self) -> u64; }   // kein Ord; Max-Stamp = StampCounter::max_raw(), None -> 0

// index_contract
pub struct EntryId(pub u64);
pub struct SharedBytes(pub Arc<[u8]>);   // Deref<Target=[u8]>
```

- Der Reader (`index_open`) lehnt doppelte IDs ab. Der Saver garantiert deshalb sortierte, eindeutige IDs. Das ist eine Vorbedingung von `Snapshot.entries`.
- Fehlermeldungen heißen „Index nicht verfügbar“, entsprechend `OpenError::Unavailable` im Reader.
- Das Ziel-Dateiformat ist Parquet mit Arrow-Schema. Neue Crates sind nicht erlaubt, nur die in `/home/user/xyan/xy.ai.workbench/rag/Cargo.toml`.