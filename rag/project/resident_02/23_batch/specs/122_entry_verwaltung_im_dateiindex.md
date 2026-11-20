# Spezifikation: Entry-Verwaltung im Dateiindex (Modul `entry_store`, Build-Order 122)

## 1. Aufgabe, Einschränkungen, Grenzen

**Aufgabe:** `EntryStore` verwaltet die Entries eines dateigestützten Index im Speicher. Die Aufgaben:

- Upsert und Remove, jeweils mit Checksum, Stamp, Dirty-Markierung, Signal und Log.
- Abfragen von Nutzdaten, Pfad, Header, Max-Stamp und Dirty-Status.
- Explizite Verifikation der Datei.
- `EntryStore` implementiert `index_contract::Index`.

**Nicht Teil des Moduls:**

- Das Speichern (Save), das Zurücksetzen von Dirty und das Schreiben von Dateien. Änderungen liegen bis zum Save nur im Speicher.
- Datei- und Spaltenformat, Parsing, Dekodierung (`FormatDecoder`) und die Berechnung der Checksum-Funktion. Sie werden injiziert (`ChecksumProvider`).
- Suche und Zugriffsmethoden (Centroid, Prefix, Subtree, Range).
- Es gibt kein `unsafe`. Als Crates sind `tracing` und `std` erlaubt. Interne Module sind `index_contract`, `change_stamp`, `index_open`, `index_format`, `meta_codec`, `parquet_decoder` und `entry_payload`.

## 2. Kontext

- Ein Index lebt in einer Parquet-Datei, die `LazyIndexFile` (`index_open`) lazy öffnet. Vektoren sind FP16 (`DType::F16`), Länge je Vektor = `header.vector_bytes()` = Dimension × 2.
- `EntryStore` legt eine Overlay-Schicht über die geladene Datei. Das Overlay enthält neue oder geänderte Entries und Tombstones für entfernte. Gelesen wird zuerst aus dem Overlay, sonst aus der Datei.
- Den Stamp vergibt der `StampCounter`, den `LazyIndexFile::stamp_counter()` liefert (fortgesetzt via `resume`). Ohne Datei ist es `StampCounter::new()`. Das Signal ist `StampSignal`.
- Die Save-Komponente liest den Zustand und setzt Dirty zurück. Das gehört nicht zu diesem Modul.

## 3. Interface

```rust
use std::{path::Path, sync::Arc};
use index_contract::{EntryId, Stamp, SharedBytes, Payload, PayloadRef, Index, IndexError, AccessMethod, StampObserver};
use index_format::IndexHeader;
use index_open::{LazyIndexFile, OpenError};
use change_stamp::StampError;
use layout::DType;

/// Injiziert, Send + Sync. Dieselbe Funktion, mit der die Dateien geschrieben werden (Hash Fast64, HASH_VERSION 1).
pub trait ChecksumProvider: Send + Sync {
    /// Checksum eines Entries über Vektor- und Nutzdaten-Bytes.
    fn checksum(&self, id: EntryId, vector: &[u8], meta: &[u8]) -> Result<u64, String>;
    /// In der Datei gespeicherte Checksums (entry_id → checksum), alle Zeilen, ohne Verifikation.
    fn stored_checksums(&self, path: &Path) -> Result<Vec<(EntryId, u64)>, String>;
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum EntryError {
    /// Vektorlänge ≠ Dimension × 2.
    #[error("Schema-Fehler: erwartet {expected} Bytes, gefunden {found}")]
    Schema { expected: usize, found: usize },
    #[error("invalid payload: {0}")]
    Meta(String),
    #[error(transparent)] Unavailable(#[from] OpenError),
    #[error(transparent)] Stamp(#[from] StampError),
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum VerifyError {
    /// Erster fehlerhafter Entry (aufsteigend nach EntryId geprüft).
    #[error("Verifikation fehlgeschlagen bei Entry {entry:?}: {cause}")]
    Entry { entry: EntryId, cause: String },
    #[error(transparent)] Unavailable(#[from] OpenError),
    /// Fehler ohne Entry-Bezug (z. B. stored_checksums schlägt fehl).
    #[error("Verifikation fehlgeschlagen: {0}")]
    Other(String),
}

pub struct EntryStore { /* header, Option<Arc<LazyIndexFile>>, checksum, Overlay (RwLock/Mutex), dirty, OnceLock<Runtime{counter, signal}> */ }

impl EntryStore {
    /// Kein IO. `file = None`: Index ohne Datei (neu).
    pub fn new(header: IndexHeader, file: Option<Arc<LazyIndexFile>>, checksum: Arc<dyn ChecksumProvider>) -> Self;

    /// Insert oder Replace mit Vektor und Nutzdaten.
    pub fn upsert(&self, id: EntryId, vector: &[u8], meta: &[u8]) -> Result<Stamp, EntryError>;
    /// Some(Stamp des Removes), None wenn nicht vorhanden.
    pub fn remove(&self, id: EntryId) -> Result<Option<Stamp>, EntryError>;

    /// Nutzdaten; None wenn Entry nicht vorhanden.
    pub fn meta(&self, id: EntryId) -> Option<SharedBytes>;
    /// Pfad der Indexdatei; None ohne Datei.
    pub fn path(&self) -> Option<&Path>;
    /// Konstruktions-Header, mit aktuellem Max-Stamp (with_max_stamp(max_raw oder 0)).
    pub fn header(&self) -> IndexHeader;
    /// Größter vergebene/fortgesetzte Stamp; None bei leerem Zähler oder nicht verfügbarer Datei.
    pub fn max_stamp(&self) -> Option<Stamp>;
    /// true, wenn `change_seq() > saved_seq`.
    pub fn is_dirty(&self) -> bool;
    /// Explizite Verifikation der Datei (blockierend).
    pub fn verify(&self) -> Result<(), VerifyError>;
}

impl Index for EntryStore { /* dtype() = DType::F16; access_methods() = &[] ; Rest §4 */ }
```

`EntryStore: Send + Sync` (static assertion).

Production checksum provider and neutral export for saving:

```rust
use meta_codec;
use parquet_decoder::ParquetDecoder;

pub struct PayloadChecksums { /* Arc<ParquetDecoder> */ }
impl PayloadChecksums { pub fn new(decoder: Arc<ParquetDecoder>) -> Self; }
impl ChecksumProvider for PayloadChecksums {}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("export failed: {0}")]
pub struct ExportError(pub String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExportedEntry {
    pub id: EntryId,
    pub stamp: Stamp,
    pub vector: SharedBytes,   // dimension * 2 bytes, FP16 LE
    pub meta: SharedBytes,     // meta_codec format
    pub checksum: u64,         // stored checksum: computed at upsert, or read from the file
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Exported {
    pub seq: u64,              // change sequence this export reflects
    pub max_stamp: u64,        // counter max_raw, None -> 0
    pub entries: Vec<ExportedEntry>,   // ascending by id, unique, no tombstones
}

impl EntryStore {
    /// Number of upserts and removes since construction. Starts at 0.
    pub fn change_seq(&self) -> u64;
    /// Ok(None) if not dirty. Blocking.
    pub fn export(&self) -> Result<Option<Exported>, ExportError>;
    /// Sets saved_seq = max(saved_seq, seq). Never lowers it.
    pub fn mark_saved_upto(&self, seq: u64);
}
```

## 4. Behavior und Invarianten

**Upsert**

- Prüft zuerst `vector.len() == header.vector_bytes()`. Sonst `Err(Schema { expected, found })` (Länge in Bytes). Zustand, Stamp und Signal bleiben unverändert. Ein leerer Vektor ist ebenfalls ein Schema-Fehler.
- Im exklusiven Write-Abschnitt, in dieser Reihenfolge:
  1. `checksum.checksum(id, vector, meta)` berechnen. An `Err(cause)` gives `EntryError::Meta(cause)` and changes nothing (no stamp, no signal, not dirty).
  2. `counter.next()` aufrufen. `Stamp(raw)` = `ChangeStamp::as_raw()`. Bei `Exhausted` gibt es `Err(Stamp(..))` ohne Änderung.
  3. Entry im Overlay ablegen (Vektor und Meta als eigene `SharedBytes`-Kopie, mit Stamp und Checksum).
  4. Dirty setzen.
  5. `signal.publish(stamp)`.
  6. `info!` loggen (ID, Stamp, Vektor- und Meta-Länge).
- Rückgabe ist der Stamp genau dieses Upserts. Die Änderung wird nicht in die Datei geschrieben.
- Ein ersetzter Entry erhält einen neuen Stamp. Nach einem Remove erhält dieselbe ID bei neuem Upsert ebenfalls einen neuen, nie zuvor vergebenen Stamp.

**Remove**

- Fehlt der Entry (weder im Overlay noch in der Datei, oder Tombstone), gilt:
  - Das Ergebnis ist `Ok(None)`.
  - Es gibt keinen Stamp, kein Dirty, kein Signal, kein Overlay-Update.
- Sonst im Write-Abschnitt: `counter.next()` → Entry aus Overlay entfernen bzw. Tombstone setzen → Dirty → `publish` → `info!`. Rückgabe `Some(stamp)`.

**Index-Vertrag**

- `dtype()` ist `F16`. `access_methods()` ist leer, die Defaults liefern `Unsupported`.
- `write(id, payload)` nimmt genau `payload.vector`:
  - Ist `vector.len() != header.vector_bytes()`, gibt es einen Panic (nicht behebbarer Programmfehler), kein `Err`. Vorher ändert sich nichts.
  - `payload.meta` wird ignoriert. Existiert der Entry, bleiben seine Nutzdaten erhalten (Overlay, sonst `LazyIndexFile::read_meta`, bei `None` leer). Bei neuen Entries sind sie leer.
  - Sonst verhält es sich wie `upsert`. `EntryError::Unavailable` oder `Stamp` werden zu `IndexError::Backend(text)`.
- `write_many` bleibt der Default (nicht atomar).
- `delete` = `remove`. Fehler werden zu `IndexError::Backend`.
- `read(id)`: Overlay-Entry, sonst `LazyIndexFile::read`. Fehlend oder Tombstone ergibt `Payload::default()`.
- `stamp(id)` und `stamps()`:
  - Sie lesen nur Stamp-Tabellen (Datei-Tabelle `stamps()` plus Overlay). Sie berühren keine Vektoren.
  - Overlay-Einträge überdecken Datei-Einträge, Tombstones entfernen sie.
  - `stamps()` ist ein Snapshot, aufsteigend nach `EntryId`, ohne Duplikate.
  - Ist die Datei nicht verfügbar, wird der Dateianteil als leer behandelt und `error!` geloggt.
- `subscribe()` liefert einen Observer, der mit dem aktuellen Max-Stamp startet.

**Abfragen**

- `meta(id)`: Overlay, sonst `LazyIndexFile::read_meta`. Fehlend oder Tombstone ergibt `None`.
- `is_dirty()` is `change_seq > saved_seq`. Initially `false`. `change_seq` increases by 1 on every successful `upsert` and every `remove` that found an entry (`Some`). Failures and `Ok(None)` change nothing. `saved_seq` starts at 0 and is only raised by `mark_saved_upto`.
- `max_stamp()` = `counter.max()`.
- `header()` liefert die Header-Werte des Konstruktors, nur `max_stamp` ist aktuell (`max_raw` oder 0).
- `path()` = `file.path()`.

**Lazy Öffnung der Laufzeit**

- `new` führt kein IO aus. Beim ersten Zugriff, der Zähler oder Signal braucht, wird genau einmal initialisiert (`OnceLock`):
  - Mit Datei: `file.stamp_counter()`.
  - Ohne Datei: `StampCounter::new()`.
  - Das Signal startet mit `Stamp(counter.max_raw().unwrap_or(0))`.
- Schlägt das Öffnen fehl, wird das Ergebnis gemerkt:
  - `upsert` und `remove` liefern `Err(Unavailable)`.
  - `subscribe` nutzt ein Signal bei `Stamp(0)`.
  - `max_stamp` ist `None`.

**Verifikation (`verify`)**

- Ohne Datei: sofort `Ok(())`.
- Mit Datei:
  1. `ensure_open()`. Ein Fehler wird zu `Unavailable`.
  2. `checksum.stored_checksums(path)`. Ein Fehler wird zu `Other`.
  3. Für jede Datei-ID aufsteigend: `file.read(id)` (dekodiert den Batch vollständig).
  4. Prüfen: Vektorlänge == `vector_bytes()`, sonst Dekodierfehler. Danach `checksum.checksum(id, vector, meta)` mit der gespeicherten vergleichen.
- Beim ersten Fehler gibt es `Err(Entry { entry, cause })` mit der betroffenen ID. Danach wird abgebrochen.
- An `Err(cause)` from `checksum` is reported as `VerifyError::Entry { entry, cause }`, and in the read-time check as a decode failure.
- Geprüft wird der Dateiinhalt. Das Overlay wird nicht verifiziert. Verify ändert weder Zustand noch Stamp noch Signal.

**Nebenläufigkeit**

- Alle Methoden sind `&self` und von beliebig vielen Threads aufrufbar.
- Upsert und Remove werden serialisiert (Last-Writer-Wins), Stamps pro Operation sind eindeutig. Die Reihenfolge ist: Daten sichtbar, Stamp gesetzt, Signal publiziert. Der Overlay-Zustand eines Entries wechselt atomar, es gibt keine Mischzustände.
- Lesende Methoden halten Locks nur kurz. Blockierende Datei-Reads (`read`, `read_meta`) laufen außerhalb des Overlay-Locks, sofern der Overlay-Treffer nicht schon die Antwort liefert. Verklemmungen sind ausgeschlossen.
- Beim Meta-Erhalt in `write` darf das Write-Lock während des Datei-Reads gehalten werden.

**Meta format and payload checksums**

- The meta bytes of `Payload.meta`, `EntryStore::meta` and `upsert` are always in the `meta_codec` format. Empty meta means the default source.
- `PayloadChecksums::checksum(id, vector, meta)`: `meta_codec::decode_meta(meta)`, then `entry_payload::compute_checksum_parts(vector, &f.target_file, f.char_start, f.char_end)`. A `MetaError` gives `Err(text)`.
- `PayloadChecksums::stored_checksums(path)`: reads the file bytes and returns `ParquetDecoder::read_stored_checksums`, mapped to `(EntryId, u64)`; errors as text.
- The decoder passed to `PayloadChecksums` and to `LazyIndexFile` is the same instance, built with `ChecksumFn = Arc::new(entry_payload::compute_checksum_parts)`.

**Export for saving**

- `EntryStore` does not import `index_save` (built later). It offers the neutral export above, and `index_save` adapts it.
- `export`: under the overlay lock, copy the overlay and read `change_seq` (this is `seq`). After releasing the lock, read every file row that the overlay neither replaces nor removes through `LazyIndexFile::{stamps, read_vector, read_meta}`, and their stored checksums through `ChecksumProvider::stored_checksums(path)`. Then merge by id. An overlay upsert replaces the file row, a tombstone removes it. The file part is immutable, so the result is consistent for `seq`.
- Stored checksums are exported as stored. They are not recomputed, so a damaged file row stays detectable after saving.
- Errors give `Err(ExportError)` and change nothing: the file is unavailable (`OpenError`), a file row has an empty vector or no meta (decode failure), `stored_checksums` fails, or an id has no stored checksum.
- After a successful save the store keeps serving from its old mapping and the overlay (the rename leaves the old mapping valid). The overlay is not pruned. The caller may build a new `EntryStore` over the new file to release the memory. Correctness does not depend on it.
- `export` holds the overlay lock only for the copy. Writers are never blocked for the file reads.

## 5. Relied-on elements

```rust
// index_contract
pub struct EntryId(pub u64); pub struct Stamp(pub u64); // Stamp(0) nie vergeben
pub struct SharedBytes(pub Arc<[u8]>); // Default leer, Deref<[u8]>
pub struct Payload { pub vector: SharedBytes, pub meta: SharedBytes }
pub struct PayloadRef<'a> { pub vector: &'a [u8], pub meta: &'a [u8] }
pub enum IndexError { InvalidVectorLength{len:usize,elem_size:usize}, Unsupported(AccessMethod), NotFound, Backend(String) }
// Trait Index (Send+Sync): dtype, access_methods, stamp, stamps, read, write, write_many (Default), delete, subscribe, summary/children/root/cell_entries/byte_ranges (Default: Unsupported)
StampSignal::new(initial: Stamp) -> Arc<StampSignal>; publish(&self, Stamp) /* max(alt,neu) */; observer(self:&Arc<Self>) -> StampObserver
// Reihenfolge je Write/Delete: Daten sichtbar, Stamp, Signal. Jeder erfolgreiche Write/Delete(Some) publiziert.

// change_stamp
StampCounter::new(); next(&self) -> Result<ChangeStamp, StampError>; max(&self) -> Option<ChangeStamp>; max_raw(&self) -> Option<u64>
ChangeStamp::as_raw(&self) -> u64   // kein Ord; Stamp(raw) im Index-Vertrag
StampError::Exhausted

// index_open
LazyIndexFile: Send+Sync; path(); ensure_open() -> Result<(),OpenError>; stamp_counter() -> Result<Arc<StampCounter>,OpenError>;
stamps() -> Result<Vec<(EntryId,Stamp)>,OpenError> /* sortiert, keine Vektor-Dekodierung */; stamp(id); 
read_meta(id) -> Option<SharedBytes>; read(id) -> Payload /* Fehler/unbekannt: leer, error!-Log */; Fehler beim Öffnen gemerkt, kein Retry.

// index_format
IndexHeader { .., max_stamp: u64, dimension: usize, .. }; vector_bytes() == dimension*2; dimension(); with_max_stamp(u64) -> IndexHeader
```

Die Vektoren sind FP16 Little-Endian. Dieses Modul konvertiert keine Werte.
