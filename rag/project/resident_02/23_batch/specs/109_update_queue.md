# Spezifikation: Update-Queue (Build-Order 109)

Crate-Modul `update_queue`.

## 1. Task, restrictions and boundaries

**Aufgabe**
- Pending-Updates (Unbind, Bind) auf einen Slot mit `ChangeStamp` führen. Je Entry bleibt nur das jüngste Update.
- Abfragen: Containment, Tiefe, Zahl der deferred Writes.
- `apply` führt die Updates aus: zuerst alle Deletes (Unbind), dann alle Binds.

**Grenzen**
- Das Modul macht keine Slot-Zuweisung, keine Eviction, kein Befüllen oder Kodieren von Slots und keinen Upload. Es setzt keine Valid-Bits, ruft also weder `SlotStates::set_filled` noch `invalidate` auf.
- Es baut, publiziert und ändert keine Maps. Es nutzt nur die übergebene `AddressMap`, um Segmente zu finden.
- Es vergibt keine Stamps und vergleicht sie nur auf Gleichheit. Die Reconciler-Logik gehört nicht hierher.
- Kein `unsafe`. Keine Abhängigkeiten außer `std`, `slot_layout`, `reconcile`, `address_map`, `change_stamp`, `index_contract`, `kernel_layout`.
- Typen der Abhängigkeiten werden nicht neu definiert.

## 2. Context

- Der Manager stellt Updates ein, z. B. per `Reconciliation::for_each_delete` (Delete) oder nach dem Laden eines Entry (Bind). Später ruft er `apply` auf.
- Das `ResidencyDirectory` (`reconcile`) hält je residentem Entry Slot und bekannten Stamp. `SlotAssignment` (`slot_layout`, pro Segment, in `AddressMap`-Entries per `Arc` geteilt) hält Slot ↔ `EntryRef`.
- `apply` hält Directory und Slot-Bindings konsistent. Deletes laufen zuerst, damit Slots frei sind, bevor Binds sie belegen.
- Es gibt zwei Typen namens `SegmentId`: `reconcile::SegmentId(u32)` (in `SlotRef`) und `address_map::SegmentId(u32)` (in der Map). Die Konversion ist `address_map::SegmentId(slot_ref.segment.0)`.

## 3. Interface

```rust
use address_map::AddressMap;
use change_stamp::ChangeStamp;
use index_contract::EntryId;
use reconcile::{ResidencyDirectory, SlotRef};
use slot_layout::EntryRef;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UpdateKind {
    /// Entry von `slot` lösen und aus dem Directory entfernen.
    Unbind,
    /// Entry an `slot` binden (alte Bindung wird zuerst gelöst).
    Bind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Update {
    pub entry: EntryRef,
    pub slot: SlotRef,
    pub stamp: ChangeStamp,
    pub kind: UpdateKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Warning {
    /// Segment des Ziel-Slots ist in der Map nicht abgebildet.
    UnmappedTarget { entry: EntryRef, slot: SlotRef, kind: UpdateKind },
    /// Bind abgelehnt (Slot außerhalb des Bereichs oder durch anderen Entry belegt); Update nicht angewandt.
    BindRejected { entry: EntryRef, slot: SlotRef },
}

/// Send + Sync, alle Methoden &self (intern synchronisiert).
#[derive(Debug, Default)]
pub struct UpdateQueue { /* Mutex<BTreeMap<EntryId, Update>>, Mutex<Vec<Warning>> */ }

impl UpdateQueue {
    pub fn new() -> Self;
    /// Stellt ein Unbind ein; ersetzt ein vorhandenes Update desselben `entry.entry`.
    pub fn push_unbind(&self, entry: EntryRef, slot: SlotRef, stamp: ChangeStamp);
    /// Stellt ein Bind ein; ersetzt ein vorhandenes Update desselben `entry.entry`.
    pub fn push_bind(&self, entry: EntryRef, slot: SlotRef, stamp: ChangeStamp);
    /// Hat der Entry ein Pending-Update?
    pub fn contains(&self, id: EntryId) -> bool;
    /// Das Pending-Update des Entry.
    pub fn get(&self, id: EntryId) -> Option<Update>;
    /// Anzahl der Pending-Updates (Unbind + Bind).
    pub fn depth(&self) -> usize;
    /// Anzahl der Pending-Binds (deferred Writes).
    pub fn deferred_writes(&self) -> usize;
    pub fn is_empty(&self) -> bool;
    /// Führt alle Pending-Updates aus (Deletes, dann Binds). Rückgabe: Zahl angewandter Updates.
    pub fn apply(&self, dir: &ResidencyDirectory, map: &AddressMap) -> usize;
    /// Liefert die seit dem letzten Aufruf angefallenen Warnungen und leert sie.
    pub fn take_warnings(&self) -> Vec<Warning>;
}
```

## 4. Behavior and invariants

**Einstellen**
- Schlüssel ist `entry.entry` (`EntryId`). Ein neues Update ersetzt ein vorhandenes desselben Entry, unabhängig von dessen Art. Das zuletzt eingestellte gilt als jüngstes. Stamps werden nur mitgeführt, nicht geordnet verglichen.
- Einstellen ändert weder Directory noch Slot-Bindings noch Map. Änderungen bleiben bis `apply` in der Queue.
- `contains`, `get`, `depth`, `deferred_writes` und `is_empty` spiegeln den Zustand zu einem Zeitpunkt. `deferred_writes` zählt nur `Bind`, `depth` zählt alle.

**apply**
- `apply` entnimmt die Pending-Updates atomar (die Queue ist danach leer) und arbeitet diesen Stand ab. Updates, die während `apply` eingestellt werden, bleiben für den nächsten `apply`.
- Reihenfolge: zuerst alle `Unbind`, aufsteigend nach `EntryId`. Danach alle `Bind`, aufsteigend nach `EntryId`.
- **Unbind** (`entry`, `slot`):
  1. `dir.remove(entry.entry)`. Das geschieht immer, auch wenn das Segment nicht abgebildet ist.
  2. Segment suchen: `map.get(address_map::SegmentId(slot.segment.0))`. Fehlt es, wird `Warning::UnmappedTarget` erzeugt und die Slot-Bindung bleibt unberührt.
  3. Die Slot-Bindung wird nur gelöst, wenn sie noch dem Entry gehört: `assignment.slot_of_entry(entry) == Some(slot.slot)`. Dann `assignment.unbind(slot.slot)`. Sonst bleibt sie unverändert, ohne Warnung. Nicht `lookup` verwenden, denn das liefert für ungültige Slots `None`.
  4. Das Update zählt als angewandt, auch bei `UnmappedTarget`.
- **Bind** (`entry`, `slot`, `stamp`):
  1. Ziel-Segment suchen. Fehlt es, wird `Warning::UnmappedTarget` erzeugt. Dann ändert sich nichts, auch nicht am Directory, und das Update zählt nicht.
  2. Alte Bindung zuerst lösen:
     - Hat `dir.get(entry.entry)` einen Slot `old`, wird in dessen Segment (falls abgebildet) `unbind(old.slot)` aufgerufen, nur wenn `slot_of_entry(entry) == Some(old.slot)`.
     - Ist `entry` im Ziel-Segment an einem anderen Slot gebunden, wird auch dieser gelöst.
     - Ein Bind auf denselben Slot löst und bindet neu.
  3. `assignment.bind(slot.slot, entry)`. Bei `Err` (z. B. `SlotOccupied`, `SlotOutOfRange`) wird `Warning::BindRejected` erzeugt und das Update zählt nicht. Das Directory wird dann nicht geändert, die in Schritt 2 gelöste alte Bindung bleibt gelöst. Das Directory-Eintrag des Entry bleibt unverändert.
  4. Bei Erfolg: `dir.insert(entry.entry, slot, index_contract::Stamp(stamp.as_raw()))`. Der Stamp des Updates wird so zum bekannten Stamp. Das Update zählt als angewandt.
- Rückgabe ist die Zahl angewandter Updates. Das sind alle Unbinds plus erfolgreiche Binds. Abgelehnte oder nicht abgebildete Binds zählen nicht.
- Eine leere Queue ergibt 0 ohne Seiteneffekt.
- Bindings ändern nur `SlotAssignment`. `SlotStates` wird nie verändert.

**Warnungen**
- Warnungen werden gesammelt und per `take_warnings` abgeholt. `apply` selbst gibt nur die Zahl zurück.

**Nebenläufigkeit und Fehler**
- `UpdateQueue` ist `Send + Sync` (statischer Test). Einstellen und Abfragen sind von mehreren Threads aus möglich.
- Gleichzeitige `apply`-Aufrufe sind sicher, jedes Update wird höchstens einmal angewandt.
- Ein vergifteter Lock wird übernommen (`into_inner`). Es gibt keine Panics bei gültigen Eingaben und keine öffentlichen Fehlertypen.

## 5. Relied-on elements

- `reconcile`
  - `ResidencyDirectory` (`&self`, Send + Sync): `insert(id: EntryId, slot: SlotRef, known: Stamp) -> Option<Resident>`, `remove(id) -> Option<Resident>`, `get(id) -> Option<Resident>`.
  - `Resident { slot: SlotRef, stamp: ChangeStamp }`.
  - `SlotRef { segment: reconcile::SegmentId(pub u32), slot: Slot }` (Copy, Eq).
- `index_contract`: `EntryId(pub u64)` (Copy, Ord, Hash). `Stamp(pub u64)`.
- `change_stamp`: `ChangeStamp` (Copy, Eq, Debug, kein `Ord`), `as_raw() -> u64`.
- `slot_layout`
  - `EntryRef { entry: EntryId, section: u32 }` (Copy, Eq, Hash, Ord).
  - `SlotAssignment` (Send + Sync, `&self`):
    - `bind(slot, entry) -> Result<(), SlotError>`. Die Prüfreihenfolge ist Bereich, `SlotOccupied`, `EntryAlreadyBound`.
    - `unbind(slot) -> Result<Option<EntryRef>, SlotError>`.
    - `slot_of_entry(entry) -> Option<Slot>`.
    - `lookup(slot)` liefert für ungültige Slots `None` und wird deshalb nicht verwendet.
- `address_map`
  - `AddressMap::get(&self, SegmentId) -> Option<&Entry>`.
  - `Entry::assignment(&self) -> &Arc<SlotAssignment>`.
  - `address_map::SegmentId(pub u32)`.
- `kernel_layout::Slot(pub u32)`.