# Spezifikation: Allgemeiner Index-Vertrag (Build-Order 33)

## 1. Aufgabe, Einschränkungen, Grenzen
**Aufgabe:** Das Crate-Modul `index_contract` definiert den Vertrag, den jeder Index erfüllt. Es besteht aus Trait, Datentypen und Fehlern. Der Vertrag deckt ab:
- Entries (ID, Stamp, Payload) nebenläufig lesen, schreiben (Last-Writer-Wins) und löschen.
- Change-Notification als Level-Signal.
- Deklaration der Zugriffsmethoden Centroid, Prefix, Subtree und Range samt zugehöriger Strukturen.

**Grenzen:**
- Das Modul enthält keine konkrete Index-Implementierung, keine Suche und keine Persistenz.
- Es enthält keine Konvertierung von Vektordaten. Der Index speichert Bytes im festen `DType`.
- Das Modul stellt den Vertrag und `StampSignal` (§3) bereit. `StampSignal` ist eine konkrete, wiederverwendbare Signal-Implementierung, nur `std`.
- Es gibt keine weiteren Abhängigkeiten außer `layout::DType`.
- Die Typen für Stamps, IDs und Daten sind in §3 abschließend definiert.

## 2. Kontext
Konkrete Indizes implementieren `Index`. Verbraucher, etwa Retrieval, Sync und Cache, nutzen sie über `&dyn Index` oder `Arc<dyn Index>` aus mehreren Threads.

- Ein Index hat genau einen festen `DType` der Vektordaten (aus dem Modul `layout`). Er ändert sich nie.
- Der Stamp ordnet Änderungen. Observer lesen über das Level-Signal den Max-Stamp und holen dann per `stamps()` einen Snapshot. Koaleszierte Signale schaden nicht.

## 3. Interface
```rust
use std::sync::Arc;
use std::time::Duration;
use layout::DType;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)] pub struct EntryId(pub u64);
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)] pub struct NodeId(pub u64);
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)] pub struct CellId(pub u64);

/// Change-Stamp. Total geordnet. Stamp(0) wird nie an einen Write/Delete vergeben (nur Anfangswert eines leeren Index).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)] pub struct Stamp(pub u64);

/// Geteilte, unveränderliche Bytes; Clone ist billig (Arc). Leer = len 0.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SharedBytes(pub Arc<[u8]>);
impl std::ops::Deref for SharedBytes { type Target = [u8]; /* ... */ }

/// Payload zum Schreiben (geliehen).
#[derive(Debug, Clone, Copy)]
pub struct PayloadRef<'a> { pub vector: &'a [u8], pub meta: &'a [u8] }

/// Gelesener Payload (geteilt). Bei fehlendem Entry: beide leer.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Payload { pub vector: SharedBytes, pub meta: SharedBytes }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AccessMethod { Centroid, Prefix, Subtree, Range }

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IndexError {
    /// vector.len() ist kein Vielfaches von dtype().size_in_bytes()
    InvalidVectorLength { len: usize, elem_size: usize },
    /// Zugriffsmethode nicht deklariert
    Unsupported(AccessMethod),
    /// unbekannter Knoten / Zelle
    NotFound,
    Backend(String),
}
impl std::fmt::Display for IndexError { /* sprechender Text */ }
impl std::error::Error for IndexError {}

/// Summary-Struktur eines Knotens (Centroid-Zugriff).
#[derive(Debug, Clone, PartialEq)]
pub struct Summary {
    pub node: NodeId,
    pub centroid: SharedBytes,   // Vektor im dtype() des Index
    pub count: u64,              // Entries im Teilbaum/Knoten
    pub stamp: Stamp,            // Stamp, zu dem die Summary gilt
}

/// Zusammenhängend gespeicherte Entries: Byte-Bereich (Range-Zugriff).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ByteRange {
    pub first: EntryId,          // erster Entry im Bereich
    pub count: u64,              // Anzahl Entries
    pub offset: u64,             // Byte-Offset im Speicher des Index
    pub len: u64,                // Byte-Länge (zusammenhängend)
}

pub trait Index: Send + Sync {
    /// Fester Elementtyp der Vektordaten.
    fn dtype(&self) -> DType;
    /// Deklarierte Methoden; konstant über die Lebenszeit.
    fn access_methods(&self) -> &[AccessMethod];

    /// Aktueller Stamp von `id`, None wenn nicht vorhanden. Berührt keine Vektordaten.
    fn stamp(&self, id: EntryId) -> Option<Stamp>;
    /// Snapshot aller (ID, Stamp), aufsteigend nach EntryId sortiert. Berührt keine Vektordaten.
    fn stamps(&self) -> Vec<(EntryId, Stamp)>;
    /// Liest den Entry; darf blockieren. Fehlt er: `Payload::default()`.
    fn read(&self, id: EntryId) -> Payload;
    /// Ersetzt den Entry vollständig (Insert oder Replace). Rückgabe: Stamp genau dieses Writes.
    fn write(&self, id: EntryId, payload: PayloadRef<'_>) -> Result<Stamp, IndexError>;
    /// Schreibt mehrere Entries in Reihenfolge; Rückgabe: IDs mit Stamp je Write.
    /// Default: ruft `write` nacheinander auf, bricht beim ersten Fehler ab.
    fn write_many(&self, items: &[(EntryId, PayloadRef<'_>)]) -> Result<Vec<(EntryId, Stamp)>, IndexError>;
    /// Löscht den Entry. Some(Stamp des Deletes), None wenn nicht vorhanden.
    fn delete(&self, id: EntryId) -> Result<Option<Stamp>, IndexError>;

    /// Level-Signal; neuer Observer startet mit aktuellem Max-Stamp.
    fn subscribe(&self) -> StampObserver;

    // Optionale Zugriffsmethoden. Default: Err(Unsupported(..)).
    fn summary(&self, node: NodeId) -> Result<Summary, IndexError>;            // Centroid
    fn children(&self, node: NodeId) -> Result<Vec<NodeId>, IndexError>;       // Subtree (aufsteigend sortiert)
    fn root(&self) -> Result<NodeId, IndexError>;                              // Subtree
    fn cell_entries(&self, cell: CellId) -> Result<Vec<EntryId>, IndexError>;  // Prefix (aufsteigend sortiert)
    fn byte_ranges(&self, first: EntryId, last: EntryId) -> Result<Vec<ByteRange>, IndexError>; // Range, inklusiv
}

/// Konkretes, vom Modul gelieferte Signal-Quelle (Index-Implementierungen nutzen sie).
pub struct StampSignal { /* Mutex<Stamp> + Condvar */ }
impl StampSignal {
    pub fn new(initial: Stamp) -> Arc<StampSignal>;
    /// Setzt den Wert auf max(alt, neu) und weckt alle Observer.
    pub fn publish(&self, stamp: Stamp);
    pub fn current(&self) -> Stamp;
    pub fn observer(self: &Arc<Self>) -> StampObserver;
}

/// Beliebig viele Observer; Clone erlaubt.
#[derive(Clone)]
pub struct StampObserver { /* Arc<StampSignal>, zuletzt gesehener Stamp */ }
impl StampObserver {
    pub fn current(&self) -> Stamp;                       // aktueller Max-Stamp (Level)
    /// Blockiert, bis Signal > zuletzt gesehener Stamp (oder Timeout); setzt "gesehen" auf den Rückgabewert.
    /// None bei Timeout. Ohne Timeout: unbegrenzt.
    pub fn wait_newer(&mut self, timeout: Option<Duration>) -> Option<Stamp>;
}
```
`Index` ist object-safe. `StampSignal` und `StampObserver` sind `Send + Sync`.

## 4. Behavior und Invarianten
**Entries**
- Ein Entry besteht aus `EntryId` (u64, total geordnet), Stamp und Payload. `NodeId` und `CellId` sind eigene Typen, nicht untereinander oder mit `EntryId` austauschbar.
- `dtype()` und `access_methods()` ändern sich nie.
- `write` mit `vector.len() % dtype().size_in_bytes() != 0` ergibt `InvalidVectorLength`. In diesem Fall bleiben Zustand und Signal unverändert. Ein leerer Vektor ist erlaubt. `meta` ist beliebig.
- Stamps sind streng monoton wachsend über alle erfolgreichen Writes und Deletes eines Index und werden nie wiederverwendet. Sie sind immer > `Stamp(0)`.

**Lesen**
- `read` liefert Vektor und Meta des zuletzt vollständig abgeschlossenen Writes. Es gibt nie Mischzustände aus zwei Writes. Bei fehlendem Entry sind beide `SharedBytes` leer.
- Ein gelesener `Payload` ändert sich nie durch spätere Writes oder Deletes (unveränderlich, geteilt).
- `read` darf blockieren, etwa auf IO.
- `stamp(id)` liefert den aktuellen Stamp oder `None`. `stamps()` ist ein Snapshot, sortiert nach `EntryId`, ohne Duplikate. Spätere Änderungen verändern ihn nicht. Beide Aufrufe lesen keine Vektordaten.

**Schreiben und Löschen**
- `write` ersetzt Vektor und Meta vollständig.
- Mehrere Writer werden serialisiert. Der zuletzt an die Reihe kommende gewinnt (Last-Writer-Wins). Der Endzustand eines Entries entspricht dem Write mit dem höchsten Stamp.
- Die Reihenfolge innerhalb eines Writes ist verbindlich:
  1. Daten sichtbar machen.
  2. Stamp setzen.
  3. Signal publizieren.
- Die Rückgabe ist der Stamp genau dieses Writes, nicht der eines nebenläufigen.
- Ist der Stamp eines Entries für einen Beobachter sichtbar, liefert `read` Daten dieses oder eines späteren Writes. Nie liefert es ältere Daten.
- `delete` entfernt Daten und Stamp. Es vergibt einen neuen Stamp und publiziert ihn (Reihenfolge Daten, Stamp, Signal). Die Rückgabe ist `Some(delete_stamp)`.
- `delete` eines nicht vorhandenen Entries ist ein No-op mit `Ok(None)`. Es vergibt keinen Stamp und ändert das Signal nicht.
- `write_many` ist nicht atomar. Jeder Write ist einzeln gültig, mit eigenem Stamp.

**Change-Signal**
- Das Signal ist ein Level, kein Ereignis-Zähler. Es trägt den aktuellen Max-Stamp und ist monoton (`publish` setzt `max(alt, neu)`).
- `subscribe()` kann beliebig oft aufgerufen werden. Der Anfangswert eines Observers ist der aktuelle Max-Stamp. Der Anfangswert eines leeren Index ist `Stamp(0)`.
- `StampObserver::current()` liefert den aktuellen Max-Stamp.
- `wait_newer` kehrt zurück, sobald der Max-Stamp größer ist als der zuletzt vom Observer gesehene. Der gesehene Stamp startet mit dem Wert bei `subscribe()`.
- Mehrere Publishes zwischen zwei Wartevorgängen dürfen zu einer Rückgabe mit dem letzten Wert verschmelzen. Das ist erlaubt und unschädlich.
- Jeder erfolgreiche Write und jeder Delete mit `Some(_)` publiziert.
- Ein Observer ist unabhängig von anderen. Ein Observer hindert Writes nicht.

**Zugriffsmethoden**
- `access_methods()` deklariert die unterstützten Methoden.
- Die Zuordnung ist:
  - Centroid → `summary`.
  - Subtree → `root` und `children`.
  - Prefix → `cell_entries`.
  - Range → `byte_ranges`.
- Für nicht deklarierte Methoden liefert die Default-Implementierung `Err(IndexError::Unsupported(<Methode>))`. Eine Implementierung überschreibt die Methoden ihrer deklarierten Zugriffsarten.
- Ein unbekannter Knoten oder eine unbekannte Zelle ergibt `NotFound`.
- Sortierungen: `children` und `cell_entries` aufsteigend. `byte_ranges` liefert Bereiche aufsteigend nach `first`.
- Ein `ByteRange` beschreibt zusammenhängend gespeicherte Entries. `len` ist die Summe der Bytes dieser Entries, `count` ihre Anzahl.
- `Summary.centroid` hat den `dtype()` des Index.

**Nebenläufigkeit**
- Alle `&self`-Methoden sind von beliebig vielen Threads gleichzeitig aufrufbar. Es gibt kein externes Locking.
- Ein Lesezugriff blockiert Writes anderer Entries nicht dauerhaft. Verklemmungen sind ausgeschlossen.

## 5. Relied-on elements
Aus `layout` (Build-Order 11), unverändert genutzt:
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DType { F32, F16, BF16, U8, U32, I64 }
impl DType { pub const fn size_in_bytes(self) -> usize; /* F32=4, F16=2, BF16=2, U8=1, U32=4, I64=8 */ }
```
`DType` ist ein `Send + Sync`-Wertetyp. Der Index nutzt `size_in_bytes()` für die Prüfung der Vektorlänge und führt keine Wertkonvertierung durch.