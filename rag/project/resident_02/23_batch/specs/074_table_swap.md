# Spezifikation: Table-Swap (Build-Order 74)

## 1. Task, restrictions and boundaries
Rust-Modul `table_swap`. Es führt die Segmenttabelle des Index im VRAM. Es enthält:
- `SegmentTable`: immutable nach Publication, Kodierung 16 B + 24 B je Segment.
- Root-Pointer: Host schreibt mit Release, Device liest mit Acquire.
- Epoch-Tracking mit Readern.
- Publication (serialisiert), Retire und Reclaim.
- `TableGuard` für Reader.

Nicht Teil des Moduls:
- Kernel, Lane-Scheduling und Copy-Implementierungen.
- Segment-Allokation und Segment-Inhalte. Das Modul speichert nur Regionen.
- Neudefinition von `Region`, `Allocator`, `Event`, `Lane`.
- Policy, wann publiziert oder freigegeben wird.
- Kein GPU-Linking. Tests laufen mit `Event::mock()` und einem Fake-`TableSink`.

## 2. Context
- Die Tabelle liegt als `bookkeeping::Region` in der Arena `IndexSegments`. Der Allocator für diese Arena kommt von außen (`Arc<Allocator>`).
- Der Root-Wert ist der Arena-Offset der aktuellen Tabellenregion (`u64`). Der Persistent-Kernel liest ihn einmal je Work-Item mit Acquire.
- Der Root-Pointer kann in fest zugeordnetem, gemapptem Host-Memory liegen. Das stellt eine `RootSlot`-Implementierung bereit.
- Sichtbares Schreiben läuft hinter `TableSink`. Das ist entweder ein Copy auf einer CopyH2D-Lane mit `Event::wait`, oder ein Write in gemappten Speicher mit Flush.
- Reader sind Query-Batches bzw. der Persistent-Kernel. Ein beendeter Batch meldet die Epoche der gelesenen Tabelle.

## 3. Interface
```rust
use std::sync::Arc;
use allocator::{Allocator, AllocatorError};
use bookkeeping::Region;                       // Segment- und Tabellenregion
use device::Event;
use memory_access::Region as AccessRegion;     // nur im AccessSink

pub type Epoch = u64;
pub const ROOT_SENTINEL: u64 = u64::MAX;       // Root vor der ersten Tabelle
pub const TABLE_HEADER_BYTES: usize = 16;
pub const SEGMENT_BYTES: usize = 24;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TableSwapError {
    StaleMapping { expected: Epoch, got: Epoch }, // Display: "stale Mapping"
    Allocation(AllocatorError),
    Write(String),                                // TableSink-Fehler
    NoTable,                                      // noch nichts publiziert
    UnknownReader(ReaderId),
    EpochAhead { reported: Epoch, published: Epoch },
    Decode(String),
}
impl std::fmt::Display for TableSwapError {}
impl std::error::Error for TableSwapError {}

// ---- Segmenttabelle ----
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SegmentTable { /* epoch: Epoch, segments: Vec<Region> */ }
impl SegmentTable {
    pub fn new(epoch: Epoch) -> SegmentTable;          // leer
    pub fn epoch(&self) -> Epoch;
    pub fn segments(&self) -> &[Region];
    pub fn len(&self) -> usize;
    pub fn is_empty(&self) -> bool;
    pub fn add(&mut self, region: Region);             // hängt hinten an
    pub fn replace(&mut self, old: &Region, new: Region) -> bool; // false, wenn old fehlt
    pub fn remove(&mut self, region: &Region) -> bool;            // false, wenn fehlt
    pub fn encoded_len(&self) -> usize;                // 16 + 24 * len
    pub fn encode(&self) -> Vec<u8>;
    pub fn decode(bytes: &[u8]) -> Result<SegmentTable, TableSwapError>;
}

// ---- Root-Pointer ----
/// Ablage des Root-Werts. Implementierungen: Host-Atomic oder gemapptes Host-Memory.
pub trait RootSlot: Send + Sync {
    fn store_release(&self, value: u64);   // nur der Host schreibt
    fn load_acquire(&self) -> u64;
}
pub struct HostRootSlot { /* AtomicU64 */ }
impl HostRootSlot { pub fn new() -> HostRootSlot; }  // Startwert ROOT_SENTINEL
impl RootSlot for HostRootSlot {}

// ---- Sichtbares Schreiben ----
pub trait TableSink: Send + Sync {
    /// Schreibt `bytes` ab Anfang von `region`. Kehrt erst zurück, wenn der Inhalt
    /// für das Device vollständig sichtbar ist (Copy-Event abgeschlossen bzw. Flush erfolgt).
    fn write_visible(&self, region: &Region, bytes: &[u8]) -> Result<(), String>;
}

/// Adapter auf `memory_access`: resolve -> Write -> after_write -> Event::wait.
pub struct AccessSink { /* resolve, after_write */ }
impl AccessSink {
    pub fn new(
        resolve: Box<dyn Fn(&Region) -> Option<AccessRegion> + Send + Sync>,
        /// Wird nach dem Write aufgerufen. Gemappt: Flush ausführen, Ok(None).
        /// Copy-Stream: Ok(Some(lane.record_event()?)) für das Event.
        after_write: Box<dyn Fn() -> Result<Option<Event>, String> + Send + Sync>,
    ) -> AccessSink;
}
impl TableSink for AccessSink {}

// ---- Reader / Epochen ----
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ReaderId(pub u32);

pub struct TableSwap { /* allocator, sink, root, publish-Mutex, state, tracker, retired */ }
// Send + Sync (statisch geprüft)

impl TableSwap {
    /// Schreibt ROOT_SENTINEL per store_release in `root`. Publizierte Epoche 0.
    pub fn new(allocator: Arc<Allocator>, sink: Arc<dyn TableSink>, root: Arc<dyn RootSlot>) -> TableSwap;

    // Publication
    /// Kopie der aktuellen Tabelle mit Epoche = publiziert + 1. Vor der ersten Publication: leere Tabelle, Epoche 1.
    pub fn next_table(&self) -> SegmentTable;
    /// Siehe Abschnitt 4. Gibt die publizierte Epoche zurück.
    pub fn publish(&self, table: SegmentTable) -> Result<Epoch, TableSwapError>;
    pub fn published_epoch(&self) -> Epoch;
    pub fn root_value(&self) -> u64;                    // root.load_acquire()
    pub fn current_table(&self) -> Option<Arc<SegmentTable>>;

    // Reader
    pub fn register_reader(&self) -> ReaderId;          // gemeldet = publiziert, ruhend
    pub fn unregister_reader(&self, id: ReaderId) -> Result<(), TableSwapError>;
    /// Beendeter Batch meldet seine Epoche. Nur steigend: kleinere Werte werden ignoriert (Ok).
    pub fn report_epoch(&self, id: ReaderId, epoch: Epoch) -> Result<(), TableSwapError>;
    pub fn set_quiescent(&self, id: ReaderId, quiescent: bool) -> Result<(), TableSwapError>;
    pub fn reader_epoch(&self, id: ReaderId) -> Result<Epoch, TableSwapError>;
    pub fn min_reader_epoch(&self) -> Epoch;
    /// Hält die aktuelle Version fest (Persistent-Kernel-Modus, Host-Seite).
    pub fn acquire(&self, id: ReaderId) -> Result<TableGuard<'_>, TableSwapError>;

    // Retire / Reclaim
    pub fn retired_count(&self) -> usize;
    /// Gibt zurückgestellte Regionen mit retire_epoch <= min(min_reader_epoch(), up_to) frei. Liefert die Anzahl.
    pub fn reclaim(&self, up_to: Epoch) -> usize;
}

pub struct TableGuard<'a> { /* &TableSwap, ReaderId, Arc<SegmentTable>, root: u64 */ }
impl TableGuard<'_> {
    pub fn epoch(&self) -> Epoch;
    pub fn root(&self) -> u64;                  // beim Acquire gelesener Root-Wert (Offset)
    pub fn table(&self) -> &SegmentTable;
}
impl Drop for TableGuard<'_> {}                 // meldet epoch() für den Reader
```

## 4. Behavior and invariants

**Kodierung** (Little Endian, genau `16 + 24·n` Bytes)
- Header: `epoch: u64`, `count: u32`, `reserved: u32 = 0`.
- Je Segment: `arena_id: u32` (`ArenaKind::id()`), `generation: u32`, `offset: u64`, `len: u64`.
- `decode(encode(t)) == t`. `decode` liefert `Err(Decode)` bei Länge ≠ `16 + 24·count`, unbekannter `arena_id` oder `reserved != 0`.

**SegmentTable**
- `add` ändert `len` um +1 und erhält die Reihenfolge.
- `replace(old,new)` ersetzt das erste Vorkommen an derselben Position und liefert `true`. Fehlt `old`, liefert es `false` und ändert nichts.
- `remove` entfernt das erste Vorkommen unter Erhalt der Reihenfolge und liefert `true`, sonst `false`.
- Eine publizierte Version ist immutable. `publish` übernimmt die Tabelle als `Arc` und gibt keinen Mutationszugriff zurück. `next_table()` ist immer eine unabhängige Kopie mit Epoche +1.
- `next_table()` ändert den Zustand nicht.

**Root**
- Vor der ersten Publication liefert `root_value() == ROOT_SENTINEL`.
- Nur `publish` schreibt den Root, immer per `store_release`. Der Inhalt der Tabelle ist vorher vollständig sichtbar.
- Der Root-Wert ist `table_region.offset`.

**publish(table)** (serialisiert durch einen Publish-Mutex)
1. Ist `table.epoch() != published + 1`, folgt `Err(StaleMapping{expected: published+1, got})`. Es bleibt alles unverändert. Von zwei Tabellen gleicher Basis gelingt daher höchstens eine.
2. `allocator.allocate(table.encoded_len())`. Fehler ergibt `Err(Allocation(e))`.
3. `sink.write_visible(&region, &table.encode())`. Fehler: Region sofort per `allocator.free` freigeben und `Err(Write)` zurückgeben. Root und Epoche bleiben unverändert.
4. `root.store_release(region.offset)`. Unter demselben internen State-Lock werden aktuelle Tabelle, Region und publizierte Epoche gesetzt. So sieht `acquire` immer ein konsistentes (Root, Epoche, Tabelle)-Tripel.
5. Retire: Die vorherige Tabellenregion (falls vorhanden) kommt mit `retire_epoch = neue Epoche` in die Retire-Liste.
6. `log::info!` mit Epoche, Segmentzahl und Offset.
7. Rückgabe ist die neue Epoche.
- Die publizierte Epoche steigt genau um 1 je erfolgreichem `publish`. Die erste Publication hat Epoche 1.
- Fehler lassen Zustand und Retire-Liste unverändert, mit Ausnahme der in Schritt 3 freigegebenen Region.

**Epoch-Tracking**
- `report_epoch` hebt die gemeldete Epoche auf `max(alt, epoch)`. Eine Epoche > publiziert ergibt `Err(EpochAhead)`. Eine unbekannte ID ergibt `Err(UnknownReader)`.
- Ein ruhender Reader (`set_quiescent(true)`) zählt als auf der publizierten Epoche angekommen.
- `min_reader_epoch()` ist das Minimum über alle nicht ruhenden Reader (gemeldete Epoche) und die publizierte Epoche für ruhende Reader. Ohne Reader ist es die publizierte Epoche. Das Ergebnis ist nie größer als die publizierte Epoche.
- `register_reader` startet ruhend, mit gemeldeter Epoche = publizierte.
- `acquire`:
  - Liefert `Err(NoTable)`, solange nichts publiziert ist.
  - Sonst setzt es den Reader auf nicht ruhend.
  - Es hebt die gemeldete Epoche auf `min(epoch der gehaltenen Tabelle, …)`, also nie über sie hinaus.
  - Es hält Root, Epoche und `Arc<SegmentTable>` fest.
- `Drop` des Guards ruft `report_epoch(reader, guard.epoch())` auf. Fehler werden ignoriert. `Drop` panict nie.
- Solange ein Guard der Epoche e lebt, ist die Tabelle e nicht freigebbar: Sie wird erst mit `retire_epoch = e+1` zurückgestellt, und der Reader meldet frühestens beim Drop mindestens e. Das Freigeben setzt `min_reader_epoch() ≥ e+1` voraus.

**reclaim(up_to)**
- `bound = min(min_reader_epoch(), up_to)`.
- Für jeden Retire-Eintrag mit `retire_epoch <= bound` wird `allocator.free(&region)` aufgerufen. Bei `Ok` wird er entfernt und gezählt. Bei `Err(StaleRegion)` wird er entfernt, mit `log::warn!` gemeldet und nicht gezählt.
- Die Rückgabe ist die Anzahl tatsächlich freigegebener Regionen. Ohne freigebbare Einträge ist sie 0.
- `reclaim` ist idempotent. Die aktuell publizierte Tabelle wird nie freigegeben.
- Parallele `reclaim`-Aufrufe geben jeden Eintrag genau einmal frei.

**Nebenläufigkeit**
- `TableSwap` ist `Send + Sync`. Publication ist serialisiert (Publish-Mutex). `acquire`, `report_epoch`, `min_reader_epoch` und `reclaim` sind parallel zu `publish` sicher.
- Lock-Reihenfolge: Publish-Mutex → State → Tracker/Retire-Liste → Allocator-intern. Kein Lock wird in umgekehrter Reihenfolge genommen.
- Vergiftete Locks werden übernommen. Es gibt keinen Panic.
- Das Device sieht den Root nur per Acquire-Load. Der Host-Store ist Release, und der Tabelleninhalt ist nach `write_visible` bereits sichtbar (Reihenfolge: Inhalt → Root).

**Hardware-Nachweis**
- Es gibt einen `#[ignore]`-Integrationstest hinter dem Feature `gpu-tests`.
- Der Test publiziert wiederholt über einen CopyH2D-Stream mit Event. Ein Persistent-Kernel liest den Root per Acquire und dann die Tabelle.
- Der Test belegt: Jeder gelesene Root-Wert zeigt auf eine vollständig geschriebene Tabelle derselben oder einer neueren Epoche. Es gibt nie ein Tripel aus neuem Root und altem oder teilweisem Inhalt.
- Unit-Tests ohne GPU prüfen die Reihenfolge mit einem Fake-`TableSink`/`RootSlot`. Dabei muss `write_visible` vor `store_release` stattfinden.

## 5. Relied-on elements
- **`bookkeeping`:** `Region { arena: ArenaKind, offset: u64, len: u64, generation: u32 }` (`Copy, Eq, Hash`).
- **`arenas`:** `ArenaKind::id() -> u32` und `ArenaKind::from_id(u32) -> Option<ArenaKind>`. Arena `IndexSegments` (ID 1).
- **`allocator::Allocator`** (`Send + Sync`):
  - `allocate(&self, len: u64) -> Result<Region, AllocatorError>`.
  - `free(&self, &Region) -> Result<(), AllocatorError>`. `Err(StaleRegion)` bei ungültiger Region.
  - `AllocatorError`: `OutOfMemory`, `AllocationFailed`, `StaleRegion` (`Clone, Debug, PartialEq, Eq`).
- **`device::Event`** (`Clone + Send + Sync`):
  - `wait(&self) -> Result<(), EventError>` (blockierend).
  - `Lane::record_event(&self) -> Result<Event, LaneError>`.
  - Tests: `Event::mock()`, `Event::cpu()`.
- **`memory_access::Region`:** `write(&self, offset: usize, src: &[u8]) -> Result<(), AccessError>`. Die Methode kehrt nach Erfolg mit Read-your-writes zurück, enthält aber keine Device-Sichtbarkeit. Die stellt `AccessSink` über `after_write` und `Event::wait` her. `AccessError` implementiert `Display`.
- **Logging:** `log`-Crate (`info!`, `warn!`).