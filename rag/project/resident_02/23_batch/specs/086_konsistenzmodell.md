# Spezifikation: Konsistenzmodell (Build-Order 86)

## 1. Task, restrictions and boundaries
Rust-Modul `consistency`. Es setzt drei Dinge um:
- **Entry-Identität** als Paar `(EntryId, ChangeStamp)`. Bei Abweichung gewinnt der Index.
- **Serialisierte Änderungen am Index** (Last-Writer-Wins).
- **Publikation von Segmenttabellen** per Copy-on-Write und Root-Pointer-Swap. Dafür dient ein serialisierender Publisher über `TableSwap`. Mapping-Änderungen werden der Tabelle nachgezogen und laufen in der Gap zwischen Batches.

Nicht Teil des Moduls:
- Eine konkrete Index-Implementierung und die Stamp-Vergabe.
- Epoch-Tracking, Retire, Reclaim, Kodierung und Root-Slot. Das alles liegt in `table_swap`.
- Das Erkennen von Batch-Grenzen. Der Aufrufer ruft `run_gap` selbst auf.
- Locks auf dem Datenpfad. Die Prüfungen (`check`, `check_many`) sind lock-frei. Stale Ergebnisse und Torn Reads werden akzeptiert.

## 2. Context
- Der Index (`Index`-Vertrag) liefert `stamp(id)` und `stamps()` ohne Vektordaten. Consumer halten Identitäten, etwa in Caches, Ergebnislisten oder Sync-Zuständen, und gleichen sie mit dem Index ab.
- Weicht die gehaltene Identität vom Index ab, gilt der Index. Das Ergebnis ist `Stale` oder `Gone`, nie ein Fehler.
- `ChangeStamp` ist opak (kein `Ord`). `index_contract::Stamp` ist geordnet. Das Modul konvertiert an der Grenze über `ChangeStamp::from_raw(stamp.0)` und vergleicht danach nur noch auf Gleichheit.
- Die Segmenttabelle liegt im VRAM. `TableSwap` publiziert sie und verwaltet Epochen und Freigabe. Dieses Modul serialisiert die Änderer und ordnet Mapping-Änderungen und Reclaim nach der Publikation in die Gap ein.

## 3. Interface
```rust
use std::sync::{Arc, Mutex};
use index_contract::{Index, IndexError, EntryId, PayloadRef, Stamp};
use change_stamp::ChangeStamp;
use table_swap::{TableSwap, SegmentTable, TableSwapError, Epoch};

// ---- Identität ----
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct EntryIdentity { pub id: EntryId, pub stamp: ChangeStamp }
impl EntryIdentity {
    pub fn new(id: EntryId, stamp: Stamp) -> EntryIdentity;      // ChangeStamp::from_raw(stamp.0)
    pub fn of(index: &dyn Index, id: EntryId) -> Option<EntryIdentity>; // aus index.stamp(id)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Verdict {
    Current,                                  // Identität stimmt mit dem Index überein
    Stale { winner: EntryIdentity },          // gleiche ID, anderer Stamp: Index gewinnt
    Gone,                                     // ID nicht (mehr) im Index
}
pub fn check(index: &dyn Index, held: &EntryIdentity) -> Verdict;
/// Ein stamps()-Snapshot für alle; Ergebnis in Reihenfolge von `held`.
pub fn check_many(index: &dyn Index, held: &[EntryIdentity]) -> Vec<Verdict>;

// ---- Serialisierte Index-Änderungen ----
pub struct IndexWriter { /* Arc<dyn Index>, Mutex<()> */ }
impl IndexWriter {
    pub fn new(index: Arc<dyn Index>) -> IndexWriter;
    pub fn write(&self, id: EntryId, payload: PayloadRef<'_>) -> Result<EntryIdentity, IndexError>;
    pub fn write_many(&self, items: &[(EntryId, PayloadRef<'_>)]) -> Result<Vec<EntryIdentity>, IndexError>;
    pub fn delete(&self, id: EntryId) -> Result<Option<Stamp>, IndexError>;
}

// ---- Tabellen-Publikation ----
/// Wird nach der Publikation der zugehörigen Tabelle in der Gap ausgeführt; Parameter = publizierte Epoche der Tabelle.
pub type MappingChange = Box<dyn FnOnce(Epoch) + Send>;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct GapReport { pub mappings_applied: usize, pub reclaimed: usize }

pub struct TablePublisher { /* Arc<TableSwap>, apply-Mutex, Pending-Queue, gap-Mutex */ }
// Send + Sync (statisch geprüft)
impl TablePublisher {
    pub fn new(swap: Arc<TableSwap>) -> TablePublisher;
    /// Copy-on-Write: next_table() -> f(&mut table) -> publish. Serialisiert. Gibt die publizierte Epoche zurück.
    pub fn apply<F: FnMut(&mut SegmentTable)>(&self, f: F) -> Result<Epoch, TableSwapError>;
    /// Wie `apply`; bei Erfolg wird `mapping` mit der publizierten Epoche vorgemerkt.
    pub fn apply_with_mapping<F: FnMut(&mut SegmentTable)>(&self, f: F, mapping: MappingChange) -> Result<Epoch, TableSwapError>;
    /// Aufruf in der Gap zwischen Batches.
    pub fn run_gap(&self) -> GapReport;
    pub fn pending_mappings(&self) -> usize;
}
```

## 4. Behavior and invariants

**Identität**
- Die Identität eines Entries ist das Paar `(id, stamp)`. Gleichheit gilt nur, wenn beide Teile gleich sind. Stamps werden ausschließlich auf Gleichheit oder Ungleichheit verglichen, nie auf Ordnung (`ChangeStamp` hat kein `Ord`).
- `check`:
  - `Current`, wenn `index.stamp(held.id) == Some(s)` und `from_raw(s.0) == held.stamp`.
  - `Stale{winner}` bei `Some(s)` mit anderem Stamp. `winner` ist `(id, from_raw(s.0))`. Das gilt auch dann, wenn der Index-Stamp numerisch kleiner ist.
  - `Gone` bei `None`.
- Der Index gewinnt immer. Das Modul schreibt nie eine gehaltene Identität in den Index zurück.
- `check_many` nutzt genau einen `stamps()`-Snapshot (sortiert nach `EntryId`). Es liefert so viele Verdicts wie `held` Einträge hat, in gleicher Reihenfolge. Duplikate in `held` werden einzeln bewertet. Ein leeres `held` ergibt einen leeren Vektor.
- Stale oder Gone sind Ergebnisse, keine Fehler. Die Funktionen geben kein `Result` zurück und paniken nicht. Es gibt kein Locking, und Torn Reads gegenüber nebenläufigen Writes sind erlaubt.
- Eine gelöschte und neu angelegte ID hat einen anderen Stamp. Sie ergibt daher `Stale`, nicht `Current`.

**IndexWriter**
- Alle Aufrufe halten einen internen Mutex. Änderungen über denselben `IndexWriter` werden dadurch strikt nacheinander angewendet (Last-Writer-Wins: Der zuletzt Drankommende bestimmt den Endzustand). Der Mutex wird nur für die Dauer des Aufrufs gehalten. Vergiftete Locks werden übernommen.
- `write` gibt `EntryIdentity{id, stamp}` mit dem Stamp genau dieses Writes zurück, also `Index::write` und dann `EntryIdentity::new`. Fehler (`IndexError`) werden unverändert durchgereicht. Der Zustand bleibt dann unverändert, wie im Index-Vertrag festgelegt.
- `write_many` ruft `Index::write_many` auf. Es ist nicht atomar. Die Rückgabe enthält je Write eine Identität in Eingabereihenfolge.
- `delete` reicht `Index::delete` durch. `Ok(None)` bei nicht vorhandenem Entry, ohne Stamp.
- Lesende Zugriffe (`check*`) gehen nicht über den Writer und werden von ihm nie blockiert.

**TablePublisher**
- `apply` und `apply_with_mapping` laufen serialisiert (apply-Mutex). Der Ablauf ist `swap.next_table()`, dann `f(&mut table)`, dann `swap.publish(table)`. Der Mutex wird über alle drei Schritte gehalten. Die Tabelle wird nie in-place verändert, die publizierte Version bleibt immutable.
- Bei `Err(StaleMapping)`, weil jemand außerhalb des Publishers publiziert hat, wird mit frischem `next_table()` und erneutem Aufruf von `f` wiederholt, insgesamt höchstens 3 Versuche. Danach wird der letzte `StaleMapping`-Fehler zurückgegeben. Andere Fehler (`Allocation`, `Write`) werden sofort ohne Retry zurückgegeben. Bei jedem Fehler bleiben Zustand und Pending-Queue unverändert und das Mapping wird nicht vorgemerkt.
- Bei Erfolg ist die Rückgabe die neue Epoche (`published_epoch()`, +1 je Erfolg). Der Root-Swap geschieht ausschließlich in `TableSwap::publish` (Inhalt sichtbar, dann `store_release`).
- `apply_with_mapping` stellt das Mapping unter demselben apply-Mutex in die Pending-Queue. Die Queue-Reihenfolge entspricht damit der Epochenreihenfolge.
- Mapping-Änderungen werden nie im `apply`-Aufruf ausgeführt. Sie folgen der Tabelle und laufen erst in `run_gap`.
- `run_gap` (serialisiert durch einen gap-Mutex; parallele Aufrufe wenden jedes Mapping genau einmal an):
  1. Alle vorgemerkten Mappings werden in Queue-Reihenfolge entnommen und mit ihrer Epoche ausgeführt. Ihre Tabelle ist zu diesem Zeitpunkt immer schon publiziert.
  2. Danach folgt `swap.reclaim(Epoch::MAX)`. Frei wird nur, was `TableSwap` freigibt: abgelöste Version mit `retire_epoch ≤ min_reader_epoch()`, also erst wenn alle Reader die neue Epoche erreicht haben.
  3. Rückgabe ist `GapReport{mappings_applied, reclaimed}`. Ohne Arbeit ist es `{0,0}`.
- Der Publisher gibt nie selbst Regionen frei und meldet keine Reader-Epochen. Readerverwaltung (`register_reader`, `acquire`, `report_epoch`) nutzen Aufrufer direkt über `TableSwap`.
- Ein Panic in einem Mapping wird nicht abgefangen. Die Queue ist zu diesem Zeitpunkt bereits entnommen. Vergiftete Locks werden übernommen.
- `pending_mappings()` liefert die Länge der Queue.

## 5. Relied-on elements
**index_contract** (unverändert):
- `EntryId(pub u64)`, `Stamp(pub u64)` (`Copy, Eq, Ord`).
- `PayloadRef<'a>{vector:&[u8], meta:&[u8]}`.
- `IndexError`.
- `trait Index: Send + Sync` mit:
  - `stamp(&self, EntryId) -> Option<Stamp>`: berührt keine Vektordaten.
  - `stamps(&self) -> Vec<(EntryId, Stamp)>`: Snapshot, aufsteigend nach `EntryId`, ohne Duplikate.
  - `write(&self, EntryId, PayloadRef<'_>) -> Result<Stamp, IndexError>`: Stamp genau dieses Writes.
  - `write_many(&self, &[(EntryId, PayloadRef<'_>)]) -> Result<Vec<(EntryId, Stamp)>, IndexError>`: nicht atomar.
  - `delete(&self, EntryId) -> Result<Option<Stamp>, IndexError>`: `Ok(None)`, wenn nicht vorhanden.
- Der Index vergibt streng monotone Stamps, die nie wiederverwendet werden. Der Index serialisiert Writer selbst (Last-Writer-Wins).

**change_stamp**:
- `ChangeStamp` ist `Clone, Copy, PartialEq, Eq, Hash, Debug`, aber nicht `PartialOrd` oder `Ord`.
- `ChangeStamp::from_raw(u64) -> ChangeStamp` und `as_raw(&self) -> u64`.
- Gleichheit und Hash beruhen allein auf dem Rohwert.

**table_swap**:
- `Epoch = u64`.
- `SegmentTable` mit `epoch()`, `add`, `replace`, `remove` und `segments()`.
- `TableSwap` (`Send + Sync`):
  - `next_table(&self) -> SegmentTable`: unabhängige Kopie mit Epoche = publiziert + 1; vor der ersten Publication eine leere Tabelle mit Epoche 1.
  - `publish(&self, SegmentTable) -> Result<Epoch, TableSwapError>`: serialisiert, schreibt zuerst den Inhalt sichtbar, dann den Root mit Release. Es liefert `Err(StaleMapping{expected,got})`, wenn `table.epoch() != published+1`.
  - `published_epoch() -> Epoch`.
  - `min_reader_epoch() -> Epoch`.
  - `reclaim(&self, up_to: Epoch) -> usize`: gibt Retire-Einträge mit `retire_epoch <= min(min_reader_epoch(), up_to)` frei. Idempotent, und parallele Aufrufe geben jeden Eintrag genau einmal frei.
- `TableSwapError`: `StaleMapping{expected,got}`, `Allocation(_)`, `Write(String)`, `NoTable`, `UnknownReader(_)`, `EpochAhead{..}`, `Decode(String)`.
- Beim Publish wird die vorherige Tabellenregion mit `retire_epoch` = neue Epoche zurückgestellt.