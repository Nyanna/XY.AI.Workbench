# Spezifikation: Dateigestützter Index (Build-Order 48)

## 1. Aufgabe, Einschränkungen, Grenzen

**Aufgabe:** Das Crate-Modul `file_index` liefert `FileIndex`, eine konkrete Implementierung von `index_contract::Index`. Der Index liegt persistent in **einer Parquet-Datei** mit Arrow-Schema. Das Modul öffnet die Datei per mmap oder File-Read, verwaltet Änderungen im Speicher und sichert sie atomar mit `save`.

**Grenzen:**
- Es gibt keine Suche, keine Vektorkonvertierung und keine Modellprüfung. Der Aufrufer liefert den erwarteten Header (`model_id`, `dimension`) fertig.
- Erlaubt sind nur Crates aus `/home/user/xyan/xy.ai.workbench/rag/Cargo.toml`. `Cargo.toml` wird nicht geändert. Für Parquet und Arrow werden `parquet` und `arrow` genutzt. Für mmap wird der dort gelisteten mmap-Crate genutzt.
- Es gibt keinen Auto-Save, auch nicht in `Drop`. Die Datei entsteht oder ändert sich nur durch `save`.
- Es gibt keine optionalen Zugriffsmethoden: `access_methods()` ist leer.
- Nichts außer diesem Modul darf die Datei ändern, solange sie geöffnet ist. Externe Modifikation ist nicht unterstützt.

## 2. Kontext

- Verbraucher (Retrieval, Sync, Cache) nutzen den Index als `Arc<dyn Index>`. `FileIndex` erfüllt den Vertrag aus `index_contract` vollständig.
- Der Index hat einen festen `DType` und ist über `model_id` und `dimension` an ein Embedding-Modell gebunden (`ModelDescriptor.model_id`, `.dimension`). Andere Modellfelder sind für ihn irrelevant. Der Aufrufer baut daraus den `IndexHeader`.
- Stamps vergibt der `StampCounter` des Moduls Change-Stamp. Das Signal liefert `StampSignal` aus dem Index-Vertrag.

## 3. Interface

```rust
use std::path::{Path, PathBuf};
use std::sync::Arc;
use index_contract::{Index, IndexError, AccessMethod, EntryId, Stamp, Payload, PayloadRef, StampObserver};
use layout::DType;

/// Identität des Index. Wird in den Parquet-Footer geschrieben und beim Öffnen verglichen.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IndexHeader {
    pub model_id: String,
    pub dimension: usize,
    pub dtype: DType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AccessMode { #[default] Mmap, File }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OpenOptions {
    pub access: AccessMode,      // Default Mmap
    pub verify: bool,            // Default true
    pub rows_per_group: i64,     // Default 8192; Werte < 1 gelten als 1
}
impl Default for OpenOptions { /* Mmap, true, 8192 */ }

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FileIndexError {
    Io(String),
    /// Datei vorhanden, aber Header weicht ab (oder fehlt).
    HeaderMismatch { field: &'static str, expected: String, found: String },
    /// Datei ist kein gültiger Index (Parquet defekt, Schema falsch, Daten verletzen Invarianten).
    Corrupt(String),
}
impl std::fmt::Display for FileIndexError { /* sprechender Text */ }
impl std::error::Error for FileIndexError {}

pub struct FileIndex { /* privat */ }   // Send + Sync

impl FileIndex {
    pub fn open(path: impl AsRef<Path>, expected: IndexHeader, options: OpenOptions)
        -> Result<FileIndex, FileIndexError>;
    /// Sichert den aktuellen Stand atomar in die Datei.
    pub fn save(&self) -> Result<(), IndexError>;   // Fehler: IndexError::Backend(String)
    pub fn path(&self) -> &Path;
    pub fn header(&self) -> &IndexHeader;
}

impl Index for FileIndex { /* vollständiger Vertrag, siehe §4 */ }
```

## 4. Behavior und Invarianten

**Dateiformat**
- Eine Parquet-Datei, Zeilen aufsteigend nach `id` sortiert, ohne Duplikate. Arrow-Schema, alle Spalten non-null:
  - `id`: UInt64
  - `stamp`: UInt64 (Rohwert, ≥ 1)
  - `vector`: Binary (Rohbytes im `dtype`)
  - `meta`: Binary
- Footer-Key-Value-Metadaten:
  - `index.model_id`
  - `index.dimension` (Dezimal)
  - `index.dtype` (Variantenname, z. B. `"F32"`)
  - `index.max_stamp` (Dezimal, Rohwert des größten je vergebenen Stamps, auch von Deletes)
- Schreiben: unkomprimiert (damit mmap ohne Dekompression lesbar bleibt). `max_row_group_size = max(rows_per_group, 1)`.

**open**
- Fehlt die Datei, entsteht ein leerer Index: Signal `Stamp(0)`, leerer Zähler. Es wird nichts auf die Platte geschrieben, die Datei entsteht erst beim ersten `save`. Das Elternverzeichnis wird dabei nicht geprüft.
- Ist die Datei vorhanden, wird immer (unabhängig von `verify`) der Footer-Header gegen `expected` verglichen. Eine Abweichung oder ein fehlender Schlüssel ergibt `HeaderMismatch` mit dem Feldnamen (`"model_id"`, `"dimension"` oder `"dtype"`). Ein nicht lesbares Parquet ergibt `Corrupt`. Ein IO-Fehler ergibt `Io`.
- `verify == true` prüft zusätzlich:
  - Das Schema stimmt exakt.
  - `id` ist streng aufsteigend.
  - Jeder `stamp` ist ≥ 1.
  - Jede `vector`-Länge ist ein Vielfaches von `dtype.size_in_bytes()`.
  - Verstöße ergeben `Corrupt`.
- `verify == false` überspringt diese Datenprüfungen. Der Footer-Header wird trotzdem geprüft.
- Beim Öffnen werden `id` und `stamp` aller Zeilen in den Speicher geladen. Vektor und Meta werden erst bei `read` gelesen. Der Zähler wird mit `StampCounter::resume(index.max_stamp, höchster vorkommender stamp)` aufgenommen. Das Signal startet mit dem Max-Stamp, also `Stamp(max_raw)`, oder `Stamp(0)`, wenn es keinen gibt.
- `AccessMode::Mmap` mappt die Datei. `AccessMode::File` liest per File-Read. Beide Modi haben identisches beobachtbares Verhalten.

**Index-Vertrag**
- `dtype()` ist `header().dtype`. `access_methods()` ist `&[]`. Alle optionalen Methoden liefern über die Defaults `Unsupported`.
- `stamp` und `stamps` kommen aus dem Speicher und berühren weder Datei noch Vektordaten.
- `read` liefert zuerst ungesicherte Änderungen. Ein ungesicherter Delete ergibt `Payload::default()`. Sonst wird die Zeile aus der Datei gelesen. `read` darf blockieren. Bei IO- oder Dekodierfehlern nach erfolgreichem `open` liefert `read` `Payload::default()` und ändert keinen Zustand.
- `write`, `write_many` und `delete` verhalten sich wie im Vertrag beschrieben. Zusätzlich gilt:
  - Die Daten bleiben im Speicher und gelangen erst durch `save` in die Datei.
  - Writer werden serialisiert.
  - Pro Write gilt die Reihenfolge: Daten sichtbar, `StampCounter::next()` (Rohwert wird `Stamp(raw)`), Signal publizieren.
  - Bei `StampError::Exhausted` liefert `write` `IndexError::Backend(..)`. Der Zustand bleibt unverändert.
  - Bei `InvalidVectorLength` bleiben Zustand, Zähler und Signal unverändert.
- `delete` eines nicht vorhandenen Entries ergibt `Ok(None)` ohne Stamp und ohne Signal.
- Ein Delete vergibt einen Stamp. Dieser fließt in `index.max_stamp` ein, auch wenn der Entry danach fehlt.
- `subscribe()` liefert einen Observer auf das `StampSignal`.

**save**
- Schreibt den Gesamtstand (gesicherte Zeilen plus Änderungen, ohne gelöschte) als vollständige neue Datei. Dazu gehören Header und `index.max_stamp`. `save` ruft die Datenquelle nicht verändernd auf.
- Ablauf: temporäre Datei im selben Verzeichnis wie `path` schreiben, `fsync` der Datei, `rename` über `path`, `fsync` des Verzeichnisses. Fehlt das Elternverzeichnis, entsteht ein Fehler. Bei jedem Fehler wird die temporäre Datei entfernt, soweit möglich, und die alte Datei bleibt unverändert.
- Ein Leser sieht immer die vollständig alte oder die vollständig neue Datei.
- `save` erzeugt einen konsistenten Snapshot. Gleichzeitige Writes gehen nicht verloren: Was nach dem Snapshot geschrieben wird, bleibt ungesichert und gelangt mit dem nächsten `save` in die Datei. Writes werden nur kurz für die Snapshot-Erstellung blockiert, nicht für die Dauer des Dateischreibens. Mehrere `save`-Aufrufe sind untereinander serialisiert.
- Nach erfolgreichem `save` nutzen Lesezugriffe die neue Datei. Bereits ausgelieferte `Payload`s bleiben unverändert gültig.
- `save` auf einem leeren Index ohne Datei schreibt eine gültige Datei mit 0 Zeilen und Header.
- Ein erneutes `open` nach `save` liefert dieselben Entries, Stamps und denselben Max-Stamp. Neue Stamps liegen strikt darüber.
- Der Zeilenaufbau einer Row-Group richtet sich nach `rows_per_group`. Das Verhalten ist beobachtbar nur über die Parquet-Metadaten.

**Nebenläufigkeit**
- `FileIndex: Send + Sync`. Alle `&self`-Methoden sind von beliebig vielen Threads aufrufbar, ohne externes Locking. Es gibt keine Verklemmung.
- Ein `read` blockiert Writes anderer Entries nicht dauerhaft.
- Ein stehender Observer hindert weder Writes noch `save`.

## 5. Relied-on elements

- Aus `index_contract` (unverändert): `trait Index: Send + Sync` mit den Methoden `dtype`, `access_methods`, `stamp`, `stamps`, `read`, `write`, `write_many`, `delete`, `subscribe` und den optionalen Methoden mit Default `Unsupported`.
  - Typen: `EntryId(pub u64)`, `Stamp(pub u64)`, `Payload`, `PayloadRef<'a>`, `SharedBytes`, `AccessMethod`, `StampObserver`.
  - Fehler: `IndexError::{InvalidVectorLength{len, elem_size}, Unsupported(AccessMethod), NotFound, Backend(String)}`.
  - Signal: `StampSignal::new(initial: Stamp) -> Arc<StampSignal>`, `publish(&self, Stamp)` (setzt `max(alt, neu)`), `observer(self: &Arc<Self>) -> StampObserver`.
  - Regeln:
    - Stamps sind streng monoton, > 0 und werden nie wiederverwendet.
    - `stamps()` ist ein Snapshot, aufsteigend nach `EntryId`.
    - Die Reihenfolge ist Daten, Stamp, Signal.
    - `write_many` ist nicht atomar.
- Aus `layout`: `DType { F32, F16, BF16, U8, U32, I64 }` mit `size_in_bytes()` (4, 2, 2, 1, 4, 8).
- Aus Change-Stamp:
  - `StampCounter::resume(stored_max: Option<u64>, highest_present: Option<u64>)` und `StampCounter::new()`.
  - `next(&self) -> Result<ChangeStamp, StampError>` mit `StampError::Exhausted`.
  - `max_raw() -> Option<u64>`.
  - `ChangeStamp::as_raw()` und `from_raw()`.
  - `ChangeStamp` hat keine Ordnung. Für den Index-Vertrag wird `Stamp(raw)` gebildet.
- Aus Modellbeschreibung: Nur `model_id` und `dimension` binden den Index an ein Modell. Der Aufrufer füllt `IndexHeader` daraus.
- Aus Scopes und Entry-Mengen: Dieses Modul nutzt nichts daraus.
- Aus Zielumgebung: Index-Format ist Parquet mit Arrow-Schema (`INDEX_FORMAT = "parquet"`). Es werden nur Crates aus dem Manifest genutzt.