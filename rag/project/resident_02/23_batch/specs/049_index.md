# Spezifikation: Index (Build-Order 49)

## 1. Aufgabe, Einschränkungen, Grenzen
**Aufgabe:** Rust-Modul `index`. Es hält Entries, Vektordaten und die Traversal-Struktur (Baum) als Source of Truth. Dieselbe Instanz dient als RAM-Darstellung und als persistente Darstellung. Es meldet Änderungen und liefert Struktur und Change-Stamps.

**Einschränkungen:**
- Das Modul kennt keine Subengine, kein Profil, keinen Kernel, keine Geometrie und kein Modell. Es löst keine Scopes auf.
- Die einzige Modellbindung ist der `IndexHeader`: Model-ID, Dimension und Pipeline-Merkmale. Es führt kein Embedding aus, rechnet keine Ähnlichkeit und konvertiert keine Datentypen.
- Entry-Payloads sind opake Bytes. Modelle wie Dateien, Sliding Windows, AST-Knoten, Chunks oder fraktale Strukturen bildet der Aufrufer darauf ab (Baum plus Geschwisterreihenfolge). Das Modul kennt sie nicht.
- Das VRAM und jeder Mirror liegen außerhalb des Moduls. Der Index ist nur Quelle und liest nie aus einem Mirror zurück.
- Abhängig sind nur `std`, `layout::DType` und `model_descriptor`. Es gibt keine Fremd-Crates, außer `thiserror` ist ohnehin im Projekt. Es gibt keine globalen Zustände.

## 2. Kontext
- `ModelDescriptor` (Modellbeschreibung) liefert `model_id`, `dimension`, `storage_dtype`, `pooling` und `normalization`. Der Index bindet sich nur daran.
- Verbraucher wie Expander, Kernel-Mirror und Subengines nutzen den Index über `Index` (lesen, Batch-Änderungen, Stamps, Change-Feed). Sie sind dem Index unbekannt.
- Vektoren verschiedener Modelle sind nicht vergleichbar. Deshalb lehnt der Index ein Öffnen mit abweichendem Header ab. `producing_profile` ist nur optionale Diagnose und wird nie verglichen.

## 3. Interface
```rust
use std::path::{Path, PathBuf};
use std::sync::mpsc::Receiver;
use crate::layout::DType;
use crate::model_descriptor::{ModelDescriptor, Pooling, Normalization};

pub type EntryId = u64; // vom Index vergeben, monoton steigend, nie wiederverwendet
pub type Stamp = u64;   // globaler Änderungszähler; leerer Index = 0

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IndexHeader {
    pub model_id: String,
    pub dimension: usize,
    pub storage_dtype: DType,
    pub pooling: Pooling,
    pub normalization: Normalization,
    pub producing_profile: Option<String>, // nur Diagnose, nie verglichen
}
impl IndexHeader {
    pub fn from_descriptor(d: &ModelDescriptor, producing_profile: Option<String>) -> IndexHeader;
    /// Bytes pro Vektor = dimension * Größe(storage_dtype); F32/U32=4, F16/BF16=2, U8=1, I64=8.
    pub fn vector_bytes(&self) -> usize;
    /// Vergleicht model_id, dimension, storage_dtype, pooling, normalization (nicht producing_profile).
    pub fn is_compatible(&self, other: &IndexHeader) -> bool;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    pub id: EntryId,
    pub parent: Option<EntryId>,   // None = Wurzelebene
    pub payload: Vec<u8>,          // opak
    pub vectors: Vec<u8>,          // n * vector_bytes(), n >= 0, little endian, storage_dtype
    pub stamp: Stamp,              // Stamp der letzten Änderung dieses Entries
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParentRef { Root, Entry(EntryId), New(usize) } // New(i): Insert an Position i < aktuelle Op im selben Batch

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Op {
    Insert { parent: ParentRef, payload: Vec<u8>, vectors: Vec<u8> }, // hängt als letztes Kind an
    Update { id: EntryId, payload: Option<Vec<u8>>, vectors: Option<Vec<u8>> }, // None = unverändert
    Move { id: EntryId, parent: Option<EntryId>, position: Option<usize> }, // Position unter dem Elternknoten; None = ans Ende
    Remove { id: EntryId }, // entfernt den gesamten Teilbaum
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChangeKind { Inserted, Updated, Moved, Removed }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Change { pub stamp: Stamp, pub id: EntryId, pub kind: ChangeKind }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChangeSet { pub from: Stamp, pub to: Stamp, pub changes: Vec<Change> } // to = aktueller Stamp

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Applied { pub stamp: Stamp, pub inserted: Vec<EntryId> } // in Op-Reihenfolge der Inserts

#[derive(Debug)]
pub enum IndexError {
    ModelMismatch { expected: IndexHeader, found: IndexHeader },
    Corrupt(String),
    Io(std::io::Error),
    NotFound(EntryId),
    InvalidVector { expected_multiple_of: usize, got: usize },
    InvalidParent(String), // unbekannt, Zyklus, ungültiges New(i)
    HistoryTruncated { oldest: Stamp },
}
impl std::fmt::Display for IndexError {}
impl std::error::Error for IndexError {}

pub struct Index { /* privat */ }
// Index: Send + Sync + 'static

impl Index {
    /// Liest nichts: Datei wird erst beim ersten Zugriff geladen (lazy). Fehlt die Datei, startet ein leerer Index mit `expected`.
    pub fn open(path: impl Into<PathBuf>, expected: IndexHeader) -> Index;
    pub fn path(&self) -> &Path;
    pub fn header(&self) -> Result<IndexHeader, IndexError>;

    pub fn get(&self, id: EntryId) -> Result<Option<Entry>, IndexError>;
    pub fn children(&self, parent: Option<EntryId>) -> Result<Vec<EntryId>, IndexError>; // in Geschwisterreihenfolge
    pub fn len(&self) -> Result<usize, IndexError>;
    /// Konsistenter Lesezugriff unter Read-Lock, ohne Kopie. Nicht re-entrant aus `f` heraus nutzen.
    pub fn read<R>(&self, f: impl FnOnce(&IndexView<'_>) -> R) -> Result<R, IndexError>;

    /// Atomarer Batch: entweder alle Ops oder keine. Leerer Batch: kein Stamp, keine Events.
    pub fn apply(&self, ops: Vec<Op>) -> Result<Applied, IndexError>;

    pub fn stamp(&self) -> Result<Stamp, IndexError>;           // globaler Stamp
    pub fn structure_stamp(&self) -> Result<Stamp, IndexError>; // Stamp der letzten Änderung an Baumstruktur oder Entry-Menge
    pub fn changes_since(&self, stamp: Stamp) -> Result<ChangeSet, IndexError>;
    pub fn subscribe(&self) -> Receiver<Change>; // ab Abonnement, in Stamp-Reihenfolge

    /// Schreibt den Zustand atomar (Temp-Datei + rename) auf `path`.
    pub fn flush(&self) -> Result<(), IndexError>;
}

pub struct IndexView<'a> { /* privat */ }
impl<'a> IndexView<'a> {
    pub fn header(&self) -> &IndexHeader;
    pub fn stamp(&self) -> Stamp;
    pub fn structure_stamp(&self) -> Stamp;
    pub fn len(&self) -> usize;
    pub fn payload(&self, id: EntryId) -> Option<&[u8]>;
    pub fn vectors(&self, id: EntryId) -> Option<&[u8]>;
    pub fn parent(&self, id: EntryId) -> Option<Option<EntryId>>; // None = unbekannte ID
    pub fn children(&self, parent: Option<EntryId>) -> &[EntryId];
    pub fn entry_stamp(&self, id: EntryId) -> Option<Stamp>;
    pub fn iter(&self) -> Box<dyn Iterator<Item = EntryId> + '_>; // aufsteigende ID
}
```

## 4. Verhalten und Invarianten
**Laden und Header**
- `open` führt keine I/O aus. Jede weitere Methode außer `path` und `subscribe` lädt beim ersten Aufruf genau einmal. Das Laden ist thread-sicher. Parallele Erstzugriffe laden nicht doppelt.
- Der geladene Header muss `is_compatible(expected)` erfüllen. Sonst liefert jeder Zugriff `ModelMismatch { expected, found }`. Es wird nichts verändert, nichts überschrieben und nichts migriert.
- Ein anderes `producing_profile` ist kein Mismatch. Der gespeicherte Wert bleibt erhalten.
- Eine fehlende Datei ergibt einen leeren Index mit Header `expected`, Stamp 0 und IDs ab 1. Die Datei entsteht erst durch `flush`.
- Ein unlesbares oder inkonsistentes Format ergibt `Corrupt` oder `Io`. Das gilt auch bei fremder Magic, unbekannter Version, falscher Prüfsumme, Vektorlänge ungleich Vielfachem von `vector_bytes()` und Parent-Zyklus. Es gibt keine Panics bei beschädigten Dateien.
- Ein Ladefehler ist nicht sticky, wenn er nur transient war (`Io`). Der nächste Zugriff versucht erneut zu laden.

**Persistenz**
- Das Format ist binär, little endian, hat Magic und Formatversion, speichert Header, Stamps, nächste ID und alle Entries samt Reihenfolge und enthält eine Prüfsumme. Die Details bestimmt die Implementierung.
- `apply` ändert den RAM-Zustand sofort. Die Datei ändert sich nur durch `flush`. Nach `flush` und erneutem `open` sind Entries, Vektoren, Baum, Geschwisterreihenfolge, Entry-Stamps, globaler Stamp, Struktur-Stamp und nächste ID bitgleich wiederhergestellt.
- `flush` schreibt nie teilweise in die Zielatei. Bei einem Fehler bleibt die alte Datei intakt. `flush` auf einem nie geladenen Index ist ein No-op ohne Fehler. Der Index schreibt nie implizit, auch nicht bei `Drop`.
- Intern darf der Index Byte-Buffer, Spaltenformate oder mmap nutzen. Sichtbar ist nur dieses Verhalten.

**Mutation**
- `apply` ist atomar. Schlägt eine Op fehl, ändert sich nichts, der Stamp bleibt, und es gibt keine Events.
- Ein nicht leerer, erfolgreicher Batch erhöht den globalen Stamp um genau 1. Alle Changes des Batches tragen diesen Stamp, ebenso die `stamp`-Felder der betroffenen Entries.
- `Insert` vergibt die nächste ID (monoton, nie wiederverwendet, auch nicht nach `Remove`). Fehlt `ParentRef::Entry(id)`, ist es `InvalidParent`. `New(i)` muss auf einen früheren `Insert` im selben Batch zeigen, sonst `InvalidParent`.
- `vectors` muss ein Vielfaches von `vector_bytes()` sein (auch 0 Bytes ist gültig). Sonst gilt `InvalidVector`. Der Index prüft nicht die Werte, auch nicht NaN oder Norm.
- `Update` mit `None` lässt das Feld unverändert. Fehlt die ID, ist es `NotFound`. Eine Op ohne Wirkung (`None`, `None`) erzeugt trotzdem `Updated`.
- `Move` hängt den Entry samt Teilbaum um. Ein Ziel im eigenen Teilbaum oder der Entry selbst ist `InvalidParent`. Eine Position größer als die Kinderzahl wird auf das Ende begrenzt. Ein Verschieben in sich selbst bei gleicher Position ist erlaubt und erzeugt `Moved`.
- `Remove` entfernt den Entry und alle Nachfahren. Jeder entfernte Entry erzeugt `Removed` (Vorfahr zuerst, dann Nachfahren in Baumreihenfolge). Ein Entry, der im selben Batch erst eingefügt und dann entfernt wird, erzeugt trotzdem seine Events. Danach liefert `get` für die IDs `None`.
- Die Ops werden sequenziell angewandt. Spätere Ops sehen die Wirkung früherer.
- `structure_stamp` springt auf den neuen globalen Stamp, wenn ein Batch `Insert`, `Move` oder `Remove` enthielt. Reine `Update`-Batches ändern ihn nicht.
- Kinder bleiben in der Reihenfolge der Einfügung (Geschwisterreihenfolge), außer `Move` mit `position`.

**Change-Feed**
- `changes_since(s)` liefert alle Changes mit Stamp > s in Stamp-Reihenfolge (innerhalb eines Batches in Op-Reihenfolge). `to` ist der aktuelle Stamp. Bei `s == stamp()` ist die Liste leer. Bei `s > stamp()` ist sie ebenfalls leer.
- Das Log liegt nur im RAM und wird nicht persistiert. Nach dem Laden ist `oldest` gleich dem geladenen Stamp. Ist `s < oldest`, so gilt `HistoryTruncated { oldest }`, und der Aufrufer muss neu synchronisieren (`read` + `iter`). Der Index darf das Log begrenzen (mindestens die letzten 10 000 Changes), dann steigt `oldest` entsprechend.
- `subscribe` liefert einen Receiver. Er empfängt jeden Change nach dem Commit, in Stamp-Reihenfolge, ohne Lücken und ohne Duplikate. Gesendet wird, bevor `apply` zurückkehrt. Ein Receiver, der gedroppt wurde, wird still entfernt und führt nie zu Fehlern oder Blockaden (unbegrenzter Kanal). `subscribe` löst kein Laden aus. Changes aus dem Laden selbst gibt es nicht.

**Concurrency**
- `Index` ist `Send + Sync`. Lesen läuft parallel, `apply` und `flush` sind gegenseitig und gegen Leser ausgeschlossen (RwLock).
- Jeder Leser sieht nur komplette Batches. `read` liefert einen konsistenten Schnappschuss, solange `f` läuft. Ein Aufruf einer `Index`-Methode aus `f` ist verboten und darf blockieren.
- `get`, `children` und `len` sind einzelne konsistente Lesevorgänge.
- Mehrere `Index`-Instanzen auf demselben Pfad im Prozess werden nicht koordiniert. Das ist Sache des Aufrufers.
- Kein Lock-Poisoning ist nach außen sichtbar. Ein Panic in einem Subscriber ist ausgeschlossen, weil nur Kanäle genutzt werden.

## 5. Relied-on elements
Aus `layout` (verbatim):
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DType { F32, F16, BF16, U8, U32, I64 }
```
Aus `model_descriptor` (nur diese Felder und Typen, alle öffentlich, `Send + Sync + 'static`):
```rust
pub enum Pooling { Mean, Cls }            // Debug, Clone, Copy, PartialEq, Eq, Hash
pub enum Normalization { L2, None }       // Debug, Clone, Copy, PartialEq, Eq, Hash
pub struct ModelDescriptor {
    pub model_id: String, pub dimension: usize, pub pooling: Pooling,
    pub normalization: Normalization, pub storage_dtype: DType, /* weitere Felder ungenutzt */
}
```
`IndexHeader::from_descriptor` kopiert genau `model_id`, `dimension`, `storage_dtype`, `pooling` und `normalization`. Der Index validiert den Descriptor nicht (das tut `validate()` des Aufrufers). Präfixe, `max_seq_len`, Pfade und Compute-Datentypen gehen nicht in den Header ein. Ihre Änderung invalidiert den Index nicht.