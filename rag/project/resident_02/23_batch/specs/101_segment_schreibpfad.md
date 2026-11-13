# Spezifikation: Segment-Schreibpfad (Build-Order 101)

Crate-Modul `segment_writer`.

## 1. Aufgabe, Einschränkungen, Grenzen

**Aufgabe**
- Slots eines Segments vorab reservieren. Das gilt für Insert und Update, die immer einen neuen Slot belegen. Der Encoder schreibt direkt in den reservierten Slot.
- Reservierte Slots per Commit sichtbar machen, sofort oder deferred bis zum Event.
- Den alten Slot eines Entries nach Insert oder Update ausblenden, die alte Zuordnung aber für ältere Treffer auflösbar halten.
- Entries per Tombstoning ausblenden.
- Den Speicher gelöschter Entries durch Rebuild des Segments zurückgewinnen (Compaction-Plan und neuer Writer).

**Grenzen**
- Dieses Modul schreibt keine Bytes ins Device. Es kodiert nicht (`SegmentView::encode_slot` nutzt der Aufrufer) und kopiert keine Slot-Daten.
- Es stellt keine Transfers, Lanes oder Events selbst her. Es allokiert und gibt kein Segment frei. Das macht der Aufrufer über die Segment-Arena.
- Es enthält keine Kernel-Anbindung, keine Eviction-Policy und keine Policy, wann ein Rebuild nötig ist.
- Es definiert keine Typen der Abhängigkeiten neu und enthält kein `unsafe`.
- Segmente wachsen nur durch Append. Ein belegter Slot wird nie erneut reserviert oder überschrieben.

## 2. Kontext

- Ein Segment ist eine `SegmentView` mit `slot_count` gleich großen Slots. Der Kernel sieht nur Slots mit gesetztem Valid-Bit in `SlotStates`.
- Treffer (`KernelHit`) werden über `SlotAssignment` auf `EntryRef` abgebildet.
- Ablauf beim Insert oder Update:
  1. Der Manager ruft `reserve` und erhält Slot, Device-Adresse und Zielbereich.
  2. Der Encoder schreibt per H2D-Transfer in den Zielbereich. Die Lane liefert dazu ein `Event`.
  3. Der Manager ruft `commit` oder `commit_deferred` mit diesem Event auf.
  4. Der Manager ruft periodisch `drain` auf.
- Der Rebuild erzeugt einen neuen Writer für ein neues Segment. Der Aufrufer allokiert dafür vorher ein Segment über die Segment-Arena, bildet daraus die neue `SegmentView` und kopiert die Slots. Das alte Segment wird danach über die Arena ersetzt oder zurückgestellt.
- Dieses Modul nutzt `SlotStates` für Valid-Bit und Tokenzahl. Write-Nummer und Centroid führt es selbst.

## 3. Interface

```rust
use std::ops::Range;
use std::sync::Arc;
use device::{Event, EventError};
use kernel_layout::{Addr, Slot};
use slot_layout::{EntryRef, SegmentView, SlotAssignment, SlotError, SlotStates};

/// Centroid-Zuordnung eines Slots. Wert u32::MAX ist reserviert (intern "keine").
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CentroidId(pub u32);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WriteError {
    /// tokens > 65_535
    TokensOutOfRange { tokens: u32 },
    /// tokens > Klassengrenze der View
    TokensExceedClass { tokens: u32, max: u32 },
    /// kein freier Slot mehr (Cursor == slot_count)
    NoFreeSlot,
    /// Transfer-Event noch nicht abgeschlossen (nur `commit`); Reservierung bleibt gültig
    NotReady,
    /// Reservierung stammt von einem anderen Writer
    ForeignReservation,
    /// Slot ist nicht im Zustand "reserviert" (schon committet, eingereiht oder verworfen)
    NotReserved { slot: Slot },
    /// CentroidId == u32::MAX
    InvalidCentroid,
    /// Rebuild nicht möglich, weil reservierte oder eingereihte Commits offen sind
    Busy,
    /// Rebuild-Ziel passt nicht (Klasse/dim/dtype verschieden) oder hat zu wenige Slots
    RebuildTarget,
    /// Event-Fehler bei `commit`
    Event(EventError),
    Slot(SlotError),
}
impl std::fmt::Display for WriteError {}
impl std::error::Error for WriteError {}

/// Vorab vergebener Slot. Unsichtbar bis zum Commit. Kein Clone/Copy.
#[derive(Debug)]
pub struct Reservation {
    pub slot: Slot,
    pub entry: EntryRef,
    pub tokens: u32,
    pub centroid: Option<CentroidId>,
    /// Device-Adresse des Slot-Anfangs (SlotAssignment::device_addr).
    pub addr: Addr,
    /// Zielbereich: absoluter Byte-Bereich im Arena-Offsetraum (SlotAssignment::byte_range), Länge = slot_bytes.
    pub target: Range<u64>,
    /* privat: writer_id */
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SlotInfo {
    pub valid: bool,
    pub tokens: u32,
    /// 0 solange nie committet; sonst fortlaufende Write-Nummer (>= 1).
    pub write_no: u64,
    pub centroid: Option<CentroidId>,
}

/// Ein überlebender Slot im Rebuild.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SlotMove { pub from: Slot, pub to: Slot, pub entry: EntryRef, pub tokens: u32, pub centroid: Option<CentroidId> }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompactionPlan { pub moves: Vec<SlotMove>, pub reclaimed_slots: u32 }

/// Schreibpfad eines Segments. Send + Sync, alle Methoden &self.
pub struct SegmentWriter { /* Mutex<Zustand>, Arc<SlotStates>, Arc<SlotAssignment>, Atomics je Slot */ }

impl SegmentWriter {
    /// Neuer, leerer Writer (SlotAssignment::fixed, SlotStates::new). Write-Nummern starten bei 1.
    pub fn new(view: SegmentView, device_base: Addr) -> Result<SegmentWriter, WriteError>;
    /// Für geladene Segmente: `used_slots` = erster freier Slot (Slots < used_slots werden nie vergeben);
    /// `next_write_no` >= 1 = nächste zu vergebende Write-Nummer. Gültige Slots aus `states`/`assignment` bleiben unberührt.
    pub fn from_parts(view: SegmentView, states: Arc<SlotStates>, assignment: Arc<SlotAssignment>,
                      used_slots: u32, next_write_no: u64) -> Result<SegmentWriter, WriteError>;

    pub fn view(&self) -> SegmentView;
    pub fn states(&self) -> Arc<SlotStates>;
    pub fn assignment(&self) -> Arc<SlotAssignment>;

    /// Reserviert den nächsten freien Slot. Slot bleibt unsichtbar bis zum Commit.
    pub fn reserve(&self, entry: EntryRef, tokens: u32, centroid: Option<CentroidId>) -> Result<Reservation, WriteError>;

    /// Sofortiger Commit: nur wenn `transfer.is_complete() == Ok(true)`.
    /// Ok(false) => Err(NotReady) (Reservierung bleibt gültig); Err(e) => Err(Event(e)) (Reservierung bleibt gültig).
    pub fn commit(&self, res: &Reservation, transfer: &Event) -> Result<(), WriteError>;
    /// Reiht den Commit ein; wirksam erst durch `drain`, wenn `event` abgeschlossen ist.
    pub fn commit_deferred(&self, res: &Reservation, event: Event) -> Result<(), WriteError>;
    /// Wendet eingereihte Commits in Einreihungsreihenfolge an. Liefert die Zahl neu sichtbar gewordener Slots.
    pub fn drain(&self) -> usize;

    /// Blendet den Entry aus (Valid-Bit löschen), ohne Daten zu entfernen.
    /// true, wenn der Entry einen gültigen Slot hatte; sonst false.
    pub fn tombstone(&self, entry: EntryRef) -> bool;

    /// Löst einen Slot auf: aktuelle Zuordnung, sonst ersetzte/ausgeblendete (für ältere Treffer).
    pub fn resolve(&self, slot: Slot) -> Option<EntryRef>;
    pub fn slot_info(&self, slot: Slot) -> Result<SlotInfo, WriteError>;
    pub fn used_slots(&self) -> u32;      // vergebene Slots (Cursor)
    pub fn free_slots(&self) -> u32;      // slot_count - used_slots
    pub fn pending_count(&self) -> usize; // reserviert, aber noch nicht angewandt (inkl. eingereihte)

    /// Compaction-Plan: alle gültigen Slots in aufsteigender Slot-Reihenfolge, dicht auf 0..n abgebildet.
    pub fn compaction_plan(&self) -> CompactionPlan;
    /// Rebuild: neuer Writer für `new_view` mit allen überlebenden Slots als Reservierungen
    /// (Slot `to`, in Plan-Reihenfolge). Der Aufrufer kopiert die Daten und committet sie normal.
    /// Der alte Writer bleibt unverändert.
    pub fn rebuild(&self, new_view: SegmentView, new_device_base: Addr)
        -> Result<(SegmentWriter, Vec<(Slot /* alt */, Reservation)>), WriteError>;
}
```

## 4. Behavior und Invarianten

**Reservierung**
- Prüfreihenfolge:
  1. `centroid == Some(CentroidId(u32::MAX))` → `InvalidCentroid`.
  2. `tokens > 65_535` → `TokensOutOfRange`.
  3. `tokens > view.class().max_tokens()` → `TokensExceedClass { tokens, max }`.
  4. `used_slots == slot_count` → `NoFreeSlot`.
- `tokens == 0` ist erlaubt.
- Bei einem Fehler ändert sich nichts.
- Der Slot ist der Cursor-Wert. Danach wird der Cursor um 1 erhöht. Slots werden streng aufsteigend und genau einmal vergeben (kein Überschreiben, keine Wiederverwendung).
- `Reservation.addr` ist `assignment.device_addr(slot)`. `Reservation.target` ist `assignment.byte_range(slot)`.
- Eine Reservierung ändert weder `SlotStates` noch `SlotAssignment`. Der Slot ist unsichtbar: `is_valid == false` und nicht gebunden.
- Reservieren ist atomar. Parallele Aufrufe erhalten verschiedene Slots.
- Eine verworfene Reservierung belegt ihren Slot bis zum Rebuild weiter, ohne sichtbar zu werden. Ein Abbruch-API gibt es nicht.

**Commit-Anwendung (gemeinsam für `commit` und `drain`)**
Die Schritte laufen in dieser Reihenfolge unter dem internen Lock:
1. Es wird eine neue Write-Nummer vergeben (+1, streng monoton je Writer, ab `next_write_no` bzw. 1). Die Nummer wird bei der Anwendung vergeben, nicht bei der Reservierung.
2. Write-Nummer und Centroid des Slots werden gespeichert.
3. Hat `res.entry` bereits einen Slot `old` (`slot_of_entry`):
   - `old → entry` wird in die interne Tabelle der ersetzten Zuordnungen eingetragen.
   - Danach folgt `unbind(old)`.
4. `assignment.bind(new, entry)`.
5. `states.set_filled(new, tokens)` setzt das Valid-Bit zuletzt (Release). Ein Leser, der das Bit mit Acquire sieht, sieht auch Write-Nummer, Centroid und Bindung.
6. Erst danach folgt `states.invalidate(old)`. Es gibt also nie ein Fenster ohne gültigen Slot für den Entry.

- `resolve(slot)` liefert `assignment.lookup(slot)`, sonst den Eintrag der Ersetzungstabelle. Ein ersetzter oder ausgeblendeter Slot bleibt damit auflösbar, auch wenn sein Valid-Bit gelöscht ist. Die Ersetzungstabelle lebt bis zum Ende des Writers.
- Ein Slot ist gültig oder ersetzt, nie beides. Pro `EntryRef` ist höchstens ein Slot gültig.
- Schlägt ein Teilschritt mit `SlotError` fehl (sollte bei gültiger Nutzung nicht passieren), kommt `WriteError::Slot`. Der Valid-Zustand des neuen Slots bleibt dann unverändert.

**Commit sofort (`commit`)**
- Prüfreihenfolge:
  1. fremder Writer → `ForeignReservation`.
  2. Slot nicht im Zustand "reserviert" → `NotReserved`.
  3. `transfer.is_complete()`: `Err(e)` → `Event(e)`, `Ok(false)` → `NotReady`.
- Erst danach wird angewandt.
- Bei `NotReady` und `Event` bleibt die Reservierung gültig, ein späterer Commit ist möglich.
- Ein zweiter Commit derselben Reservierung ergibt `NotReserved`.
- `commit` blockiert nie auf das Event (kein `wait`).

**Commit deferred und Drain**
- `commit_deferred` prüft `ForeignReservation` und `NotReserved`, markiert den Slot als eingereiht und hängt `(Slot, Event)` an die FIFO-Queue. Es wird nichts sichtbar und das Event wird nicht geprüft.
- `drain` geht die Queue vom Kopf aus durch, ohne zu blockieren. Je Kopfeintrag gilt:
  - `is_complete() == Ok(true)`: anwenden, entfernen, zählen.
  - `Ok(false)`: **anhalten**. Spätere Einträge werden nicht angewandt, auch wenn ihr Event schon abgeschlossen ist.
  - `Err(_)`: Der Commit wird verworfen. Der Slot bleibt dauerhaft unsichtbar und `log::warn!` wird gerufen. `drain` läuft mit dem nächsten Eintrag weiter. Verworfene Einträge werden nicht gezählt.
- Rückgabe: Zahl der durch diesen Aufruf neu sichtbar gewordenen Slots. Eine leere Queue ergibt 0. `drain` ist idempotent.
- Sofort-Commits (`commit`) sind unabhängig von der Queue und überholen eingereihte Commits.

**Tombstone**
- `tombstone(entry)`: Hat der Entry einen gebundenen Slot, dann:
  - Der Eintrag wird in die Ersetzungstabelle übernommen.
  - `unbind(slot)` und `states.invalidate(slot)` folgen in dieser Reihenfolge.
  - Rückgabe `true`.
- Ohne gebundenen Slot ist die Rückgabe `false`.
- Es werden keine Slot-Daten verändert.
- Idempotent: Der zweite Aufruf liefert `false`.
- Ein Slot mit offener Reservierung oder eingereihtem Commit wird nicht berührt. Wird dieser Commit später angewandt, ist der Entry wieder sichtbar.
- Danach kann derselbe `EntryRef` per `reserve` neu eingefügt werden.

**Rebuild und Compaction**
- `compaction_plan` enthält alle Slots mit gesetztem Valid-Bit in aufsteigender Slot-Reihenfolge, mit `to = 0..n` dicht.
  - `tokens` kommt aus `SlotStates`, `centroid` aus dem Slot.
  - `reclaimed_slots = used_slots − n`.
  - Der Plan ist ein Snapshot und ändert den Writer nicht.
- `rebuild` prüft in dieser Reihenfolge:
  1. `pending_count() > 0` → `Busy`.
  2. `new_view` hat andere Klasse, `dim` oder `dtype` als `self.view()`, oder `slot_count < n` → `RebuildTarget`.
- Danach baut es einen neuen Writer mit leeren Zuständen (Write-Nummern ab 1) und reserviert die Slots `0..n` in Plan-Reihenfolge mit den Werten `entry`, `tokens` und `centroid` der Moves.
- Die Reservierungen werden als `(alter Slot, Reservation)` zurückgegeben. Die neuen Slots sind unsichtbar, bis der Aufrufer nach dem Kopieren committet.
- Die Ersetzungstabelle des alten Writers wird nicht übernommen.
- Der alte Writer bleibt vollständig nutzbar und unverändert. Wer ihn freigibt, entscheidet der Aufrufer (Segment-Arena).
- Jede Reservierung des neuen Writers ist an diesen Writer gebunden. Commits dafür gehen nur an den neuen Writer.

**Fehlerfälle und Randbedingungen**
- `SegmentWriter::new` und `from_parts` mit `used_slots > slot_count` oder `next_write_no == 0` ergibt `WriteError::Slot(SlotOutOfRange { slot: used_slots, count: slot_count })` bzw. `WriteError::Slot(...)`. Der Implementierer wählt die Variante, die zu `SlotError` passt. Dabei wird nie gepanikt.
- `slot_info` außerhalb des Bereichs ergibt `Slot(SlotOutOfRange)`. Bei einem nie committeten Slot ist `valid == false`, `write_no == 0` und `centroid == None`.
- `slot_info.tokens` kommt aus `SlotStates::meta`. Ein ungültiger Slot hat dort 0.

**Nebenläufigkeit**
- `SegmentWriter` ist `Send + Sync` (statischer Test). Ein interner Mutex serialisiert Reserve, Commit-Anwendung, Queue und Tombstone. Ein vergifteter Lock wird übernommen (`into_inner`), ohne Panic.
- Der Encoder schreibt ohne Lock in den reservierten Slot. Der Slot ist bis zur Anwendung unsichtbar, und fremde Schreiber berühren ihn nie.
- Lesende Aufrufe (`slot_info`, `resolve`, `states().is_valid`) blockieren sich nicht gegenseitig. Metadaten sind pro Slot atomar (`AtomicU64` bzw. `AtomicU32`). Lesende Pfade nehmen den Writer-Mutex nicht.
- Lock-Reihenfolge: Writer-Mutex vor `SlotAssignment`-Lock. Es gibt keine umgekehrte Reihenfolge, und es wird kein Event unter dem Lock blockierend abgewartet.
- Ein Leser, der das Valid-Bit sieht, kann `resolve` und `slot_info` nutzen und sieht dabei konsistente Daten (Release/Acquire über `SlotStates`).

## 5. Relied-on elements

- **`slot_layout`**
  - `SegmentView` ist `Copy`. Genutzt werden `slot_count()`, `class().max_tokens()`, `dim()`, `dtype()` und `slot_bytes()`.
  - `SlotStates` (`Send + Sync`, `&self`):
    - `new(&view)`
    - `set_filled(Slot, tokens) -> Result<(), SlotError>` setzt die Tokenzahl und das Valid-Bit (Release). Es prüft `tokens <= max`.
    - `invalidate(Slot)` ist idempotent und setzt tokens auf 0.
    - `is_valid(Slot)`
    - `meta(Slot) -> Result<SlotMeta { valid, tokens }, SlotError>`
  - `SlotAssignment` (`Send + Sync`, `&self`):
    - `fixed(&view, Addr)`
    - `bind(Slot, EntryRef)` ergibt `SlotOccupied` oder `EntryAlreadyBound`, wenn der Slot belegt bzw. der Entry anderswo gebunden ist.
    - `unbind(Slot) -> Result<Option<EntryRef>, _>`
    - `lookup(Slot) -> Option<EntryRef>`
    - `slot_of_entry(EntryRef) -> Option<Slot>`
    - `device_addr(Slot) -> Result<Addr, _>`
    - `byte_range(Slot) -> Result<Range<u64>, _>` (absolut im Arena-Offsetraum)
  - `EntryRef { entry: EntryId, section: u32 }` ist `Copy + Eq + Hash`.
  - `SlotError` mit u. a. `SlotOutOfRange { slot: u32, count: u32 }`.
- **`kernel_layout`**: `Addr(pub u64)`, `Slot(pub u32)`.
- **`device`**
  - `Event` ist `Clone + Send + Sync`.
  - `Event::is_complete() -> Result<bool, EventError>` ist nicht blockierend.
  - `EventError` ist `Clone + Debug + PartialEq + Eq`.
  - Tests nutzen `Event::mock()` mit `MockEventControl::{complete, fail, reset}`.
- **`segment_arena` / `region_handle`**: Nur indirekt. Die Region und ihre Größe stecken in der `SegmentView`. Allokation, Replacement und Freigabe des Segments liegen beim Aufrufer.
- **`index_contract::EntryId`** ist über `EntryRef` eingebunden.
- **Logging**: `log`-Crate (`warn!`).