# Spezifikation: Segmente und Slots (Build-Order 76)

## 1. Task, restrictions and boundaries

**Task.** Rust-Modul `segments`. Es führt den Index im VRAM als Segmente. Ein Segment ist eine Region (`RegionHandle`) der Arena `IndexSegments`, unterteilt in Slots fester Größe (Slot-Klasse). Das Modul führt das Address-Mapping im VRAM über Slots, also die Zuordnung Slot → `EntryId` und zurück. Es beschreibt die Segmente für Such-Kernels als `SegmentDesc` (`Fixed`).

**Nicht Teil des Moduls:**
- Keine Allokation, kein Freigeben und keine Eviction. Das Modul ruft `Allocator` und `Bookkeeping` nicht auf. Der Aufrufer allokiert die Region, übergibt sie in `add_segment` und gibt die von `remove_segment` zurückgegebene Region selbst per `free_deferred` frei.
- Keine Auflösung von `RegionHandle` auf Device-Adressen. Die Basisadresse (`DeviceAddr`) liefert der Aufrufer (Memory-Backend) als Parameter. Keine Raw-Pointer, kein `unsafe`.
- Keine Stamps, keine Change-Signale und keine Implementierung von `Index`. Es nutzt nur `EntryId` aus dem Index-Vertrag.
- Keine Wertekonvertierung. Slots enthalten opake Bytes.
- Keine Redefinition der Typen `RegionHandle`, `ArenaKind`, `Region` (memory_access), `SegmentDesc`, `SegmentLayout`, `DeviceAddr` und `EntryId`.
- Nur die Slot-Klasse "fest" (`SegmentLayout::Fixed`). Variable Segmente gibt es hier nicht.

## 2. Context

- Der Lader allokiert Regionen in `ArenaKind::IndexSegments` (Allocator), legt Segmente an und schreibt Entries in Slots. Er entfernt Segmente bei Eviction und gibt die Region deferred frei.
- Der Query-Planer holt `SegmentDesc` für `BatchSpec.segments`. Die Basisadresse liefert das Memory-Backend.
- Kernels liefern Treffer als Adresse. `entry_of_addr` bildet sie auf die `EntryId` ab (Adress→Entry über `SegmentDesc::entry_of`, dann Slot→Entry).
- Die Bytes eines Segments werden über `memory_access::Region` gelesen und geschrieben.

## 3. Interface

```rust
use arenas::ArenaKind;
use region_handle::RegionHandle;
use memory_access::{Region as MemRegion, AccessError};
use index_contract::EntryId;
use kernel_batch::{DeviceAddr, SegmentDesc, SegmentLayout};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SegmentId(pub u64);              // aufsteigend vergeben, nie wiederverwendet

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SlotRef { pub segment: SegmentId, pub slot: u64 }

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SlotError {
    InvalidSlotSize,                         // slot_size == 0
    WrongArena(ArenaKind),                   // Handle nicht in IndexSegments
    SegmentTooSmall { len: u64, slot_size: u64 },   // handle.len() < slot_size
    BackingMismatch { handle_len: u64, mem_len: u64 },
    UnknownSegment(SegmentId),
    DuplicateRegion(RegionHandle),           // Handle (alle vier Felder) schon registriert
    NoFreeSlot,
    PayloadTooLarge { len: usize, slot_size: u64 },
    UnknownEntry(EntryId),
    OutOfBounds { len: usize, slot_size: u64 },   // read: dst.len() > slot_size
    Access(AccessError),
}
impl std::fmt::Display for SlotError {}
impl std::error::Error for SlotError {}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RemovedSegment {
    pub id: SegmentId,
    pub handle: RegionHandle,     // zum Freigeben durch den Aufrufer
    pub entries: Vec<EntryId>,    // Entries, die in diesem Segment lagen, aufsteigend; ihr Mapping ist entfernt
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SegmentStats { pub segments: usize, pub slots_total: u64, pub slots_used: u64 }

/// Thread-safe (interner RwLock). Send + Sync.
pub struct SegmentStore { /* slot_size, Segmente, entry -> SlotRef */ }

impl SegmentStore {
    pub fn new(slot_size: u64) -> Result<SegmentStore, SlotError>;
    pub fn slot_size(&self) -> u64;

    /// `mem`: Bytes der Region; Byte 0 von `mem` = Anfang von `handle`.
    pub fn add_segment(&self, handle: RegionHandle, mem: MemRegion) -> Result<SegmentId, SlotError>;
    /// Entfernt Segment samt Mapping seiner Entries. Gibt nichts frei.
    pub fn remove_segment(&self, id: SegmentId) -> Result<RemovedSegment, SlotError>;

    pub fn segment_handle(&self, id: SegmentId) -> Option<RegionHandle>;
    pub fn slot_count(&self, id: SegmentId) -> Option<u64>;
    pub fn segment_ids(&self) -> Vec<SegmentId>;            // aufsteigend

    /// Ordnet `entry` einem Slot zu und schreibt `bytes` (Rest des Slots mit 0 aufgefüllt).
    pub fn put(&self, entry: EntryId, bytes: &[u8]) -> Result<SlotRef, SlotError>;
    /// Entfernt das Mapping. Slot wird frei. Some(alter Slot) oder None, wenn nicht vorhanden.
    pub fn remove(&self, entry: EntryId) -> Option<SlotRef>;
    /// Liest dst.len() Bytes ab Slotanfang.
    pub fn read(&self, entry: EntryId, dst: &mut [u8]) -> Result<(), SlotError>;

    pub fn slot_of(&self, entry: EntryId) -> Option<SlotRef>;
    pub fn entry_at(&self, slot: SlotRef) -> Option<EntryId>;     // None: frei oder unbekannt
    /// Entries eines Segments in Slot-Reihenfolge (nur belegte Slots).
    pub fn entries_in(&self, id: SegmentId) -> Result<Vec<(u64, EntryId)>, SlotError>;

    /// Fixed { stride: slot_size }, entries = slot_count, base = `base`.
    pub fn segment_desc(&self, id: SegmentId, base: DeviceAddr) -> Result<SegmentDesc, SlotError>;
    /// Adresse -> Slot -> Entry. None: außerhalb, Slot frei oder Segment unbekannt.
    pub fn entry_of_addr(&self, id: SegmentId, base: DeviceAddr, addr: DeviceAddr) -> Option<EntryId>;

    pub fn stats(&self) -> SegmentStats;
}
```

## 4. Behavior and invariants

**Segmente**
- `new(0)` ergibt `InvalidSlotSize`.
- `add_segment` prüft in dieser Reihenfolge:
  1. `handle.arena() != ArenaKind::IndexSegments` ergibt `WrongArena`.
  2. `handle.len() < slot_size` ergibt `SegmentTooSmall`.
  3. `mem.len() as u64 != handle.len()` ergibt `BackingMismatch`.
  4. Ein bereits registriertes, gleiches `RegionHandle` ergibt `DuplicateRegion`.
- `slot_count = handle.len() / slot_size` (abgerundet). Ein Rest am Ende bleibt ungenutzt.
- Slot `i` belegt die Bytes `[i*slot_size, (i+1)*slot_size)` der Region. Die Slots überlappen nie.
- `SegmentId`s sind streng aufsteigend und werden nie wiederverwendet.
- `remove_segment` entfernt Segment und Mapping atomar. `RemovedSegment.entries` ist aufsteigend sortiert. Nach dem Entfernen ergeben `slot_of`, `read` und `entry_at` für diese Entries bzw. Slots `None` bzw. `UnknownEntry`. Eine unbekannte ID ergibt `UnknownSegment`.

**Mapping**
- Jeder Entry belegt höchstens einen Slot, und jeder Slot enthält höchstens einen Entry (Bijektion zwischen belegten Slots und gemappten Entries).
- `put` eines bereits gemappten Entries überschreibt dessen Slot (Last-Writer-Wins) und gibt denselben `SlotRef` zurück.
- `put` eines neuen Entries wählt den freien Slot mit dem niedrigsten `(SegmentId, slot)`. Ohne freien Slot kommt `NoFreeSlot`, ohne Zustandsänderung.
- `bytes.len() > slot_size` ergibt `PayloadTooLarge`, ohne Zustandsänderung. Eine Länge von 0 ist erlaubt, dann wird der Slot vollständig mit 0 beschrieben.
- Ein Backend-Fehler beim Schreiben ergibt `Access(..)`. Das Mapping eines neuen Entries wird dann nicht angelegt. Bei Überschreiben bleibt das alte Mapping bestehen.
- `read` eines nicht gemappten Entries ergibt `UnknownEntry`. `dst.len() > slot_size` ergibt `OutOfBounds`. `dst.len() == 0` gelingt ohne Zugriff.
- `remove` löscht nur das Mapping. Der Slot wird sofort wiederverwendbar, seine Bytes bleiben bis zum nächsten `put` unverändert.

**Kernel-Beschreibung**
- `segment_desc` liefert `SegmentDesc { base, entries: slot_count, layout: SegmentLayout::Fixed { stride: slot_size } }`. Das Modul prüft `base` nicht.
- `entry_of_addr` nutzt `SegmentDesc::entry_of(addr, &[])`, übernimmt den Slot-Index und schlägt ihn im Mapping nach. Ein freier Slot ergibt `None`. Die Adresse darf an beliebiger Stelle im Slot liegen.

**Nebenläufigkeit**
- `SegmentStore` ist `Send + Sync`. Alle Methoden nehmen `&self` und sind atomar. Lesende Methoden laufen parallel, schreibende werden serialisiert.
- Parallele `put` für verschiedene Entries erhalten verschiedene Slots. Parallele `put` für denselben Entry enden im Zustand eines der beiden Writes, ohne Mischzustand (Zuordnung und Schreiben laufen unter demselben Lock).
- Der Lock wird nicht über Aufrufe hinweg gehalten, und es laufen keine Callbacks unter ihm. Ein vergifteter Lock wird übernommen, es gibt keine Panics.
- `Region::read` und `Region::write` laufen unter dem Lock des Stores. Die Reihenfolge ist: erst Store-Lock, dann Backend.

**Fehler.** Alle Fehler sind `SlotError`. Es gibt keine Panics, und Überläufe werden mit `checked_*` behandelt.

## 5. Relied-on elements

- `region_handle::RegionHandle`: `Copy + Eq + Hash + Send + Sync`. Es nutzt `len() -> u64` und `arena() -> ArenaKind`. Die Auflösung auf Adressen erfolgt nur im Memory-Backend.
- `arenas::ArenaKind::IndexSegments` (ID 1).
- `memory_access::Region`: `Clone + Send + Sync`. `len() -> usize`, `read(&self, offset: usize, dst: &mut [u8]) -> Result<(), AccessError>`, `write(&self, offset: usize, src: &[u8]) -> Result<(), AccessError>`. Es prüft Grenzen selbst, und `AccessError` ist `Clone + Eq`.
- `index_contract::EntryId(pub u64)`: `Copy + Eq + Ord + Hash`.
- `kernel_batch`:
  ```rust
  pub type DeviceAddr = u64;
  pub enum SegmentLayout { Fixed { stride: u64 }, Variable { offsets: RegionHandle } }
  pub struct SegmentDesc { pub base: DeviceAddr, pub entries: u64, pub layout: SegmentLayout }
  impl SegmentDesc { pub fn entry_of(&self, addr: DeviceAddr, offsets: &[u64]) -> Option<u64>; }
  ```
  `entry_of` liefert bei `Fixed` den Index `(addr-base)/stride`. Es liefert `None` bei `stride == 0`, `addr < base` oder `idx >= entries`. Es rechnet überlaufsicher, und `offsets` wird bei `Fixed` ignoriert.
- Allocator mit Deferred Free (nur Aufrufer-Vertrag, nicht von diesem Modul aufgerufen): `Allocator::free_deferred(&self, &bookkeeping::Region, Event)`. Der Aufrufer gibt die Region aus `RemovedSegment.handle` erst nach Abschluss der laufenden Kernel-Arbeit frei.