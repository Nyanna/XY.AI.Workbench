# Spezifikation: Address-Map und Mapping-Store (Build-Order 83)

Crate-Modul `address_map`.

## 1. Task, restrictions and boundaries

**Aufgabe**
- `AddressMap` bildet Arena-Adressen (Byte-Offsets im Arena-Offsetraum) auf Segment-`Entry`s ab. Die Segmentmenge wird per Copy-on-Write verändert. Die Map trägt eine Epoche.
- `MappingStore` hält die aktuelle Map versioniert. Es gibt Snapshots nur bei passender Epoche heraus und nimmt nur Nachfolger-Maps (Epoche = aktuell + 1) an.

**Grenzen**
- Nicht Teil des Moduls:
  - Allokation von Regionen.
  - Befüllen von Slots.
  - Eviction und Kernel.
  - Das Schreiben der Segmenttabelle ins Device (das macht `table_swap`).
  - Die Policy, wann Maps gebaut oder publiziert werden.
- Das Modul führt keine Slot-Bindings selbst aus. Es hält nur gemeinsame Referenzen darauf.
- Kein `unsafe`. Typen der Abhängigkeiten werden nicht neu definiert. Nur `std`.
- Abhängigkeiten: `slot_layout`, `kernel_layout` (nur `Slot`, `Addr`), `region_handle` (über `SegmentView`), `table_swap` (nur `Epoch`).

## 2. Context

- Ein Segment ist eine `SegmentView` über einer Region. Zu ihr gehören `SlotStates` (Valid-Bits) und `SlotAssignment` (Slot ↔ `EntryRef`). Beide sind nebenläufig und intern synchronisiert.
- Der Manager baut eine Nachfolger-Map, ändert sie und publiziert sie im `MappingStore`.
- Leser nehmen einen Snapshot (ein billiger Clone) und lesen ihn ohne Locks.
- Klone teilen die Segmentliste (CoW). Die Slot-Bindings und Valid-Bits liegen in `Arc`-geteilten Objekten. Sie sind über alle Epochen und Klone dieselben.
- Bindings werden nur in Gaps geändert, also nur in bisher ungebundenen Slots. Ein bereits gebundener Slot ändert sich nie. Dadurch bleibt jeder Snapshot für gebundene Slots stabil.

## 3. Interface

```rust
use kernel_layout::{Addr, Slot};
use slot_layout::{SegmentView, SlotAssignment, SlotStates};
use std::ops::Range;
use std::sync::{Arc, RwLock};
use table_swap::Epoch;                       // = u64

/// Segment-ID, eindeutig innerhalb einer Map.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SegmentId(pub u32);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MapError {
    EmptySegment,                                  // add: Adressbereich leer
    DuplicateId(SegmentId),                        // add: ID vergeben
    Overlap { with: SegmentId },                   // add: überlappt bestehendes Segment
    RangeOverflow,                                 // offset + total_bytes > u64::MAX
    EpochOverflow,                                 // successor bei Epoche u64::MAX
    StaleMapping { base: Epoch, published: Epoch },// Display: "stale Mapping"
}
impl std::fmt::Display for MapError {}
impl std::error::Error for MapError {}

/// Ein Segment der Map. Billig klonbar (Arc-Felder). Send + Sync.
#[derive(Clone)]
pub struct Entry { /* id, view, device_base, states: Arc<SlotStates>, assignment: Arc<SlotAssignment>, range */ }
impl Entry {
    pub fn new(id: SegmentId, view: SegmentView, device_base: Addr,
               states: Arc<SlotStates>, assignment: Arc<SlotAssignment>)
        -> Result<Entry, MapError>;                // RangeOverflow
    pub fn id(&self) -> SegmentId;
    pub fn view(&self) -> &SegmentView;
    pub fn device_base(&self) -> Addr;
    pub fn states(&self) -> &Arc<SlotStates>;
    pub fn assignment(&self) -> &Arc<SlotAssignment>;
    /// Arena-Bereich: view.region().offset() .. offset + view.total_bytes().
    pub fn range(&self) -> Range<u64>;
    /// Slot, der `addr` enthält; None außerhalb von range().
    pub fn slot_at(&self, addr: u64) -> Option<Slot>;
}

/// Copy-on-Write-Map mit Epoche. Clone = stabiler Snapshot. Send + Sync.
#[derive(Clone)]
pub struct AddressMap { /* epoch: Epoch, segments: Arc<Vec<Entry>> sortiert nach range().start */ }
impl AddressMap {
    pub fn new(epoch: Epoch) -> AddressMap;                  // leer
    pub fn epoch(&self) -> Epoch;
    /// Klon mit Epoche + 1, gleiche Segmente.
    pub fn successor(&self) -> Result<AddressMap, MapError>; // EpochOverflow
    pub fn add(&mut self, entry: Entry) -> Result<(), MapError>;
    /// true, wenn das Segment vorhanden war und entfernt wurde.
    pub fn remove(&mut self, id: SegmentId) -> bool;
    /// Segmente, aufsteigend nach Adresse.
    pub fn segments(&self) -> &[Entry];
    pub fn len(&self) -> usize;
    pub fn is_empty(&self) -> bool;
    pub fn get(&self, id: SegmentId) -> Option<&Entry>;
    /// Segment, dessen range() die Arena-Adresse enthält.
    pub fn lookup(&self, addr: u64) -> Option<&Entry>;
}

/// Versionierter Halter der publizierten Map. Send + Sync, alle Methoden &self.
pub struct MappingStore { /* RwLock<AddressMap> */ }
impl MappingStore {
    /// Start: leere Map, Epoche 0.
    pub fn new() -> MappingStore;
    pub fn epoch(&self) -> Epoch;
    /// Snapshot nur bei base == publizierter Epoche.
    pub fn snapshot(&self, base: Epoch) -> Result<AddressMap, MapError>;
    /// Aktueller Snapshot ohne Epochenprüfung.
    pub fn current(&self) -> AddressMap;
    /// successor() der aktuellen Map; Basis für eine neue Publication.
    pub fn next(&self) -> Result<AddressMap, MapError>;
    /// Publiziert nur bei map.epoch() == publiziert + 1. Gibt die neue Epoche zurück.
    pub fn publish(&self, map: AddressMap) -> Result<Epoch, MapError>;
}
impl Default for MappingStore {}
```

## 4. Behavior and invariants

**Entry**
- `range()` ist `offset .. offset + view.total_bytes()` mit `offset = view.region().offset()`. Die Summe wird geprüft, ein Überlauf ergibt `RangeOverflow`.
- `slot_at(addr)` ist `Slot((addr - range.start) / view.slot_bytes())` für `addr` in `range()`, sonst `None`. `slot_bytes` ist immer > 0.
- Ein Entry mit `slot_count == 0` hat einen leeren Bereich. `Entry::new` gelingt, aber `add` lehnt ihn ab.

**AddressMap**
- `add` prüft in dieser Reihenfolge:
  1. Leerer Bereich → `EmptySegment`.
  2. ID bereits vergeben → `DuplicateId(id)`.
  3. Bereich überlappt ein bestehendes Segment → `Overlap { with }`.
  - Direkt aneinandergrenzende Bereiche sind erlaubt.
  - Bei einem Fehler bleibt die Map unverändert. Bei Erfolg ist `len` um 1 größer und die Segmente bleiben aufsteigend nach `range().start` sortiert.
- `remove(id)` liefert `true` und entfernt das Segment, wenn es vorhanden war. Sonst liefert es `false` und ändert nichts. Die Sortierung bleibt erhalten.
- `lookup(addr)` nutzt eine Binary Search und liefert das eindeutige Segment mit `start <= addr < end`, sonst `None`.
- CoW: `clone` teilt die Segmentliste. `add` und `remove` auf einem Klon ändern andere Klone und Snapshots nie sichtbar. Die Liste wird erst beim ersten Schreiben auf geteiltem Stand kopiert.
- Die Epoche ändern nur `new` und `successor`. `add` und `remove` lassen sie unverändert. `successor().epoch() == epoch() + 1`. Bei `u64::MAX` ergibt es `EpochOverflow`.
- Slot-Bindings und Valid-Bits sind `Arc`-geteilt und liegen nicht in der Map-Kopie. Alle Klone und Epochen sehen dieselben Objekte.
- Das Modul ruft selbst weder `unbind` noch `invalidate` auf. Bindings ändern sich nur in Gaps: `SlotAssignment::bind` lehnt belegte Slots ab (`SlotOccupied`). Wer die Map nutzt, bindet nur ungebundene Slots.

**MappingStore**
- `new`: Epoche 0, leere Map.
- `snapshot(base)` liefert bei `base == epoch()` einen Klon der aktuellen Map. Sonst liefert es `StaleMapping { base, published: epoch() }`.
- `publish(map)`:
  - Bei `map.epoch() == published + 1` wird die Map atomar zur aktuellen Map und die neue Epoche zurückgegeben. Die Prüfung und das Setzen laufen unter demselben Schreib-Lock.
  - Sonst ergibt es `StaleMapping { base: map.epoch().saturating_sub(1), published }`. Der Zustand bleibt dabei unverändert.
  - Von zwei Maps gleicher Basis gelingt höchstens eine.
  - Die publizierte Epoche steigt genau um 1 je erfolgreichem `publish`. Epochen werden nie übersprungen oder zurückgesetzt.
- `next()` ist `current().successor()`. Es ändert den Zustand nicht.
- Parallele Leser blockieren sich nicht. Ein Snapshot bleibt nach späteren Publications unverändert.
- Vergiftete Locks werden übernommen (`into_inner`). Es gibt keine Panics bei gültigen Eingaben.
- Fehler laufen ausschließlich über `MapError`.
- `Entry`, `AddressMap`, `MappingStore`, `SegmentId` und `MapError` sind `Send + Sync`. Das wird per statischem Test geprüft.

## 5. Relied-on elements

- `slot_layout`
  - `SegmentView` ist `Copy`. Verwendet werden `region() -> RegionHandle`, `total_bytes() -> u64` (`slot_count * slot_bytes`) und `slot_bytes() -> u32` (immer > 0).
  - `SlotStates` und `SlotAssignment` sind `Send + Sync` und haben nur `&self`-Methoden.
  - `SlotAssignment::bind(slot, entry)` liefert `SlotOccupied` bei belegtem Slot.
- `region_handle::RegionHandle::offset() -> u64` (Arena-Offset).
- `kernel_layout::{Addr(pub u64), Slot(pub u32)}`.
- `table_swap::Epoch = u64`. Die erste Tabellen-Publication dort hat Epoche 1, passend zum Store-Start bei 0.