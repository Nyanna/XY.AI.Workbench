# Spezifikation: Residency-Steuerung (Build-Order 125)

Crate-Modul `residency`.

## 1. Task, restrictions and boundaries

**Aufgabe**
- Registrierte Indizes lazy und scope-getrieben ins VRAM laden (`ensure`) und wieder verdrängen (`evict`).
- Die Residenz eines Index mit dem Index abgleichen (`reconcile`). Entfernte Entries werden verdrängt, geänderte neu geladen.
- Segmente aus der Segment-Arena anlegen. Leere Segmente werden über die Segmenttabelle abgemeldet und erst nach Reader-Epoche freigegeben (`run_gap`).

**Grenzen**
- **Lazy:** `register` lädt nichts. Es ruft weder `Index::read`, `stamps`, `stamp` noch `SegmentArena::allocate`/`allocate_bump` und `TablePublisher::apply*` auf. Nur `ensure` und `reconcile` laden Daten.
- Keine Scope-Auflösung. Sie kommt über den Trait `ScopeResolver` vom Aufrufer (`Scope` löst nichts selbst auf).
- Keine Auflösung von Allocator-Region auf `RegionHandle`/`MemRegion`. Das macht der Aufrufer über den Trait `SegmentBacking`.
- Keine Policy, wann `ensure`, `evict` oder `reconcile` aufgerufen werden. Keine automatische Verdrängung bei `Exhausted`. Der Aufrufer ruft dann `evict` auf.
- Es gibt keine Threads, Timer oder Hintergrundarbeit. Das Modul schreibt nie in den Index.
- Es erzeugt keine `Manifest`s und nutzt weder `AddressMap`/`MappingStore` noch `UpdateQueue`. Das Slot-Mapping führt der `SegmentStore`.
- Kein `unsafe`. Typen der Abhängigkeiten werden nicht neu definiert.

## 2. Context

- Pro registriertem Index gibt es:
  - einen eigenen `SegmentStore` (Slot ↔ Entry, Bytes in Arena-Regionen),
  - ein eigenes `ResidencyDirectory` (bekannter Stamp je residentem Entry),
  - einen eigenen `Reconciler`.
- Alle Indizes teilen sich eine `SegmentArena` (`ArenaKind::IndexSegments`, Budget und Reserve dort), einen `TablePublisher` und das zugehörige `TableSwap` (Segmenttabelle mit Epochen).
- Ein Segment ist eine Arena-`Segment`-Region. Seine `Region` steht in der `SegmentTable`.
- **Reihenfolge beim Laden:** erst Daten in Slots schreiben, dann neue Segmente in die Tabelle publizieren (Inhalt vor Sichtbarkeit).
- **Reihenfolge beim Verdrängen:** erst Segment per Tabelle abmelden, dann das Mapping entfernen, danach die Region erst frei geben, wenn `swap.min_reader_epoch() >= Publikations-Epoche`.
- Der Index gewinnt immer. Stale Reads und Torn Reads gegenüber nebenläufigen Writes sind akzeptiert (Konsistenzmodell).

## 3. Interface

```rust
use std::sync::Arc;
use index_contract::{EntryId, Index, IndexError};
use scopes::{EntrySet, Scope};
use segment_arena::{Segment, SegmentArena, SegmentArenaError};
use allocator::Allocator;
use consistency::TablePublisher;                 // GapReport
use table_swap::{TableSwap, TableSwapError};
use segments::{SegmentStore, SlotError};
use reconcile::ResidencyDirectory;
use region_handle::RegionHandle;
use memory_access::Region as MemRegion;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct IndexId(pub u64);            // aufsteigend vergeben, nie wiederverwendet

/// Löst einen Scope gegen einen Index zu Entry-IDs auf (aufsteigend per EntrySet::iter).
pub trait ScopeResolver: Send + Sync {
    fn resolve(&self, index: &dyn Index, scope: &Scope) -> Result<EntrySet, IndexError>;
}

/// Macht eine Arena-Segmentregion für den SegmentStore nutzbar.
/// Vertrag: handle.arena() == IndexSegments, handle.len() == mem.len() == seg.len,
/// Byte 0 von `mem` = seg.offset.
pub trait SegmentBacking: Send + Sync {
    fn open(&self, seg: &Segment) -> Result<(RegionHandle, MemRegion), String>;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct IndexSpec {
    /// Slotgröße in Byte (> 0). Gespeichert wird `Payload.vector`.
    pub slot_size: u64,
    /// Länge je angefordertem Segment (>= slot_size).
    pub segment_bytes: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResidencyError {
    UnknownIndex(IndexId),
    InvalidSpec,                         // slot_size == 0 oder segment_bytes < slot_size
    Resolve(IndexError),
    Publish(TableSwapError),
    Slot(SlotError),
    Arena(SegmentArenaError),
    Backing(String),
    IdOverflow,                          // Segment-/Slot-ID passt nicht in u32 des Directory
}
impl std::fmt::Display for ResidencyError {}
impl std::error::Error for ResidencyError {}

/// Grund, warum `ensure` nicht alle Entries geladen hat (kein Fehler, Teilerfolg).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StopReason {
    Exhausted { requested: u64, available: u64 },   // Arena voll
    Backing(String),
    Slot(SlotError),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnsureReport {
    pub loaded: u64,
    pub already_resident: u64,
    pub missing: u64,            // load_entry == None
    pub oversized: u64,          // vector.len() > slot_size; nicht geladen
    pub not_attempted: u64,      // nach Abbruch nicht bearbeitet
    pub new_segments: usize,
    pub stop: Option<StopReason>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct EvictReport { pub evicted: u64, pub segments_dropped: usize }

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ReconcileOutcome {
    pub removed: u64,
    pub reloaded: u64,
    pub current: usize,
    pub evict: EvictReport,      // fallengelassene leere Segmente stehen in segments_dropped
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ResidencyGap { pub table: consistency::GapReport, pub released: usize }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IndexResidency { pub entries: usize, pub segments: usize, pub slots_total: u64 }

/// Send + Sync; alle Methoden &self.
pub struct ResidencyController { /* Registry RwLock<BTreeMap<IndexId, Arc<..>>>, retire-Liste Mutex<Vec<(Epoch, Segment)>> */ }

impl ResidencyController {
    pub fn new(
        arena: Arc<SegmentArena<Arc<Allocator>>>,
        swap: Arc<TableSwap>,
        publisher: Arc<TablePublisher>,
        backing: Arc<dyn SegmentBacking>,
        resolver: Arc<dyn ScopeResolver>,
    ) -> ResidencyController;

    /// Registriert, lädt nichts. Legt SegmentStore, Directory und Reconciler (subscribe) an.
    pub fn register(&self, index: Arc<dyn Index>, spec: IndexSpec) -> Result<IndexId, ResidencyError>;
    /// Verdrängt alles (wie evict(All)) und entfernt dann die Registrierung. Bei Fehler bleibt sie bestehen.
    pub fn unregister(&self, id: IndexId) -> Result<EvictReport, ResidencyError>;

    /// Lädt die noch nicht residenten Entries des Scopes.
    pub fn ensure(&self, id: IndexId, scope: &Scope) -> Result<EnsureReport, ResidencyError>;
    /// Verdrängt die residenten Entries des Scopes; leere Segmente werden abgemeldet.
    pub fn evict(&self, id: IndexId, scope: &Scope) -> Result<EvictReport, ResidencyError>;
    /// Quittiert das Signal und gleicht die residenten Entries mit dem Index ab.
    pub fn reconcile(&self, id: IndexId) -> Result<ReconcileOutcome, ResidencyError>;
    /// Steht ein ungesehenes Index-Signal an? Blockiert nie.
    pub fn pending(&self, id: IndexId) -> Result<bool, ResidencyError>;

    /// In der Gap zwischen Batches aufrufen.
    pub fn run_gap(&self) -> ResidencyGap;

    pub fn is_resident(&self, id: IndexId, entry: EntryId) -> bool;
    pub fn residency(&self, id: IndexId) -> Result<IndexResidency, ResidencyError>;
    pub fn store(&self, id: IndexId) -> Option<Arc<SegmentStore>>;          // für SegmentDesc / entry_of_addr
    pub fn directory(&self, id: IndexId) -> Option<Arc<ResidencyDirectory>>;
    /// Segmente, die abgemeldet sind und noch auf Reader-Epochen warten.
    pub fn retired_count(&self) -> usize;
}
```

`ResidencyController`, `EnsureReport`, `ResidencyError` und die Reports sind `Send + Sync` (statisch geprüft).

## 4. Behavior and invariants

**register / unregister**
- `slot_size == 0` oder `segment_bytes < slot_size` ergibt `InvalidSpec`.
- `register` führt keine Index-Lese- oder Allokationsaufrufe aus. Danach ist `is_resident == false` für alle Entries, `residency().entries == 0`, `segments == 0`, und die Arena ist unverändert.
- `IndexId`s sind aufsteigend und werden nie wiederverwendet.
- Jeder Aufruf mit unbekannter ID ergibt `UnknownIndex`.
- `unregister` verdrängt vorher alles. Die Freigabe der Segmente erfolgt wie bei `evict` über `run_gap`.

**ensure(id, scope)**
1. Nimmt den Index-Lock. Pro Index laufen `ensure`, `evict` und `reconcile` serialisiert, verschiedene Indizes parallel.
2. `resolver.resolve(&*index, scope)`. Ein Fehler ergibt `Resolve(e)` ohne Zustandsänderung.
3. Die IDs werden aufsteigend bearbeitet:
   - Ist `directory.get(id)` vorhanden, zählt der Entry als `already_resident` und wird nicht neu geladen oder aktualisiert (das macht `reconcile`).
   - `reconcile::load_entry(index, id)` liefert `None`: `missing += 1`.
   - `payload.vector.len() > slot_size`: `oversized += 1`, nichts wird gemappt.
   - Sonst `store.put(EntryId, &payload.vector)`:
     - Bei Erfolg `directory.insert(id, SlotRef, loaded.stamp)` (der Stamp aus `load_entry`, nie ein später gelesener), und `loaded += 1`.
     - Bei `NoFreeSlot` wird ein Segment angelegt (siehe unten) und `put` einmal wiederholt.
     - Ein anderer `SlotError` beendet den Lauf mit `stop = Some(Slot(e))`.
4. Segment anlegen:
   - `arena.allocate(spec.segment_bytes)`.
   - `backing.open(&seg)`.
   - `store.add_segment(handle, mem)`.
   - Das Segment merkt sich der Controller (`SegmentId → Segment`) und zählt es zu `new_segments`.
   - `SegmentArenaError::Exhausted{requested, available}` ergibt `stop = Exhausted{..}`, jeder andere Arena-Fehler `Err(Arena(e))`.
   - Ein `open`-Fehler ergibt `stop = Backing(msg)` und gibt das Segment sofort per `arena.release(&seg, Readers::None)` frei.
   - Ein `add_segment`-Fehler ergibt `stop = Slot(e)` und gibt das Segment ebenfalls sofort frei.
5. Bei `stop` wird abgebrochen. `not_attempted` ist die Anzahl der noch nicht bearbeiteten IDs. Bereits geladene Entries bleiben resident (Teilerfolg ist `Ok`).
6. Gibt es neue Segmente, folgt `publisher.apply(|t| { for r in &new_regions { t.add(r) } })` (einmal pro `ensure`, nach allen Schreibvorgängen).
   - Bei `Err(e)`: Rollback der in diesem Aufruf angelegten Segmente. Dazu `store.remove_segment`, `directory.remove` aller zurückgegebenen Entries und `arena.release(&seg, Readers::None)` (nie publiziert, daher keine Reader). Rückgabe ist `Err(Publish(e))`. Entries in bereits publizierten Segmenten bleiben.
7. Wiederholtes `ensure` desselben Scopes lädt nichts neu (`loaded == 0`) und legt keine Segmente an.
8. Ein `ensure` mit leerer Menge ist ein No-op (Report mit Nullen).

**evict(id, scope)**
- Menge: `Scope::All` nimmt alle Directory-Einträge, ohne den Resolver aufzurufen. Jeder andere Scope wird aufgelöst (Fehler `Resolve`, vorher keine Änderung) und mit den residenten Entries geschnitten.
- Je residentem Entry: `store.remove(entry)` und `directory.remove(entry)`, `evicted += 1`. Nicht residente Entries sind ein No-op. Der Slot ist sofort wiederverwendbar.
- Danach werden alle Segmente ohne belegte Slots (`store.entries_in(sid)` leer) abgemeldet:
  1. `publisher.apply_with_mapping(|t| t.remove(region) für jedes leere Segment, mapping)`. Das Mapping erhält die publizierte Epoche `e` und stellt `(e, Segment)` in die Retire-Liste.
  2. Nach Erfolg `store.remove_segment(sid)` je Segment, `segments_dropped += 1`.
  3. Bei `Err(e)` kommt `Err(Publish(e))`. Die Entries bleiben verdrängt, die leeren Segmente bleiben unverändert im Store und in der Tabelle und sind weiter nutzbar.
- Das Mapping wird nie im `evict`-Aufruf ausgeführt, sondern in `run_gap` (Verhalten von `TablePublisher`).
- Die Region eines abgemeldeten Segments wird nicht vor `run_gap` und nicht vor Erreichen der Reader-Epoche freigegeben.
- `entry_of_addr` für Adressen in abgemeldeten Segmenten ergibt danach `None` (akzeptiert).

**reconcile(id)**
1. Unter dem Index-Lock `Reconciler::reconcile(&directory)`. Das quittiert das Signal vor dem Vergleich und liest keine Vektordaten.
2. Jeder Entry aus `removed`: `store.remove`, `directory.remove`.
3. Jeder Entry aus `reload`:
   - `load_entry` liefert `None`: wie `removed` behandeln.
   - Zu große Payload: Entry wie `removed` verdrängen.
   - Sonst `store.put` (dasselbe Slot wird überschrieben) und `directory.insert(id, slot, loaded.stamp)`.
   - Bei `NoFreeSlot` ist das nicht möglich, da der Entry bereits gemappt ist. Ein anderer `SlotError` ergibt `Err(Slot(e))`. Bereits Verarbeitetes bleibt, der Rest bleibt `reload`-pflichtig, denn sein Directory-Stamp ist unverändert.
4. Danach werden leere Segmente wie bei `evict` abgemeldet (`evict.segments_dropped`).
5. `ReconcileOutcome.removed` zählt `removed` aus dem Abgleich zuzüglich der beim Reload fehlenden oder zu großen Entries. `reloaded` zählt erfolgreich neu geschriebene Entries.
6. `reconcile` lädt nie neue, nicht residente Entries. Ohne anstehendes Signal ist der Aufruf zulässig und vergleicht trotzdem.
7. `pending(id)` ist `Reconciler::pending()` und blockiert nie.

**run_gap**
1. `publisher.run_gap()`. Dabei laufen die vorgemerkten Mappings der Verdrängungen und füllen die Retire-Liste.
2. Für jeden Retire-Eintrag mit `epoch <= swap.min_reader_epoch()`: `arena.release(&segment, Readers::None)` und den Eintrag entfernen.
   - Bei `Err(StaleRegion)` wird er entfernt, mit `log::warn!` gemeldet und nicht gezählt.
   - Ein anderer Fehler: `log::warn!`, Eintrag bleibt für den nächsten Versuch erhalten.
   - Einträge mit größerer Epoche bleiben.
3. Rückgabe: `ResidencyGap { table, released }`. `run_gap` ist idempotent und parallel sicher (jede Region wird höchstens einmal freigegeben).

**Abfragen**
- `is_resident` ist `directory.get(entry).is_some()`. Für unbekannte Index-IDs ist es `false`.
- `residency()` nutzt `directory.len()` und `store.stats()`.

**Nebenläufigkeit und Fehler**
- Lock-Reihenfolge: Index-Lock → Arena/Store/Publisher (je intern) → Retire-Liste (Blatt). Die Registry wird nur kurz gehalten (Arc klonen). Unter dem Registry-Lock und dem Retire-Lock laufen keine Aufrufe in andere Module.
- Mappings des Publishers nehmen nur den Retire-Lock, nie den Index-Lock. Es gibt kein Deadlock zwischen `run_gap` und `ensure`/`evict`.
- Vergiftete Locks werden übernommen. Es gibt keine Panics bei gültigen Eingaben. IDs aus `segments::SegmentId(u64)` und Slot `u64` werden für `reconcile::SlotRef` mit `u32::try_from` umgesetzt. Bei Überlauf kommt `IdOverflow` (der Entry wird dann nicht gemappt, d. h. `store.remove`).
- Fehlschlag in `ensure` oder `evict` lässt Directory, Store und Tabelle konsistent. Jeder Eintrag im Directory hat einen Slot im Store mit dem Stamp, mit dem er geladen wurde.

## 5. Relied-on elements

- **`scopes`:** `Scope` (`All`, `Range{start,end}`, `Set(EntrySet)`, `Subtree{root}`, `Prefix{bytes}`, `Centroid{count,radius}`; `#[non_exhaustive]`). `EntrySet::iter() -> impl Iterator<Item = u64>` aufsteigend, `len() -> u128`, `intersection`, `from_ids`. `EntryId` ist dort `u64`. Zu `index_contract::EntryId(pub u64)` wird über `.0` konvertiert.
- **`index_contract`:** `EntryId(pub u64)`, `Stamp(pub u64)`, `Payload{vector: SharedBytes, meta}` (`SharedBytes` derefs auf `[u8]`), `IndexError`. `Index: Send + Sync` (`read`, `stamp`, `stamps`, `subscribe`).
- **`reconcile`:**
  - `load_entry(&dyn Index, EntryId) -> Option<LoadedEntry{stamp: Stamp, payload, retries}>`.
  - `ResidencyDirectory` (`insert(id, SlotRef, Stamp) -> Option<Resident>`, `remove`, `get`, `len`, `iter`), alle `&self`.
  - `SlotRef{segment: reconcile::SegmentId(u32), slot: kernel_layout::Slot(u32)}`.
  - `Reconciler::new(&Arc<dyn Index>)` (hält den Index nur schwach, daher hält der Controller den `Arc<dyn Index>` selbst), `reconcile(&mut self, &ResidencyDirectory) -> Option<Reconciliation{removed, reload, current, ..}>`, `pending(&self) -> bool`.
  - Der Reconciler verändert das Directory nie.
- **`segments`:**
  - `SegmentStore::new(slot_size) -> Result<_, SlotError>`.
  - `add_segment(&self, RegionHandle, MemRegion) -> Result<segments::SegmentId, SlotError>`.
  - `remove_segment(&self, id) -> Result<RemovedSegment{id, handle, entries}, SlotError>` (gibt nichts frei).
  - `put(&self, EntryId, &[u8]) -> Result<segments::SlotRef{segment: SegmentId(u64), slot: u64}, SlotError>`. `put` eines gemappten Entries überschreibt seinen Slot; `NoFreeSlot` ohne Zustandsänderung; `PayloadTooLarge` bei `len > slot_size`.
  - `remove(&self, EntryId) -> Option<segments::SlotRef>` (Slot sofort wiederverwendbar).
  - `entries_in(&self, SegmentId) -> Result<Vec<(u64, EntryId)>, SlotError>` (nur belegte Slots).
  - `segment_ids()`, `stats() -> SegmentStats{segments, slots_total, slots_used}`.
  - Alle `&self`, `Send + Sync`.
  - `add_segment` verlangt `handle.arena() == IndexSegments`, `handle.len() >= slot_size` und `mem.len() == handle.len()`.
- **`segment_arena`:**
  - `SegmentArena<A: Borrow<Allocator>>`.
  - `allocate(&self, len: u64) -> Result<Segment{region: bookkeeping::Region, offset, len}, SegmentArenaError>`.
  - `release(&self, &Segment, Readers) -> Result<(), SegmentArenaError>`, mit `Readers::None` sofort.
  - `SegmentArenaError::{Exhausted{requested, available}, StaleRegion, ..}`.
  - `Segment` ist `Copy`.
  - Budget und Reserve verwaltet die Arena.
- **`consistency::TablePublisher`:**
  - `apply<F: FnMut(&mut SegmentTable)>(&self, F) -> Result<Epoch, TableSwapError>`.
  - `apply_with_mapping(&self, F, MappingChange = Box<dyn FnOnce(Epoch) + Send>) -> Result<Epoch, TableSwapError>`. Das Mapping läuft nie im Aufruf, nur in `run_gap`.
  - `run_gap(&self) -> GapReport{mappings_applied, reclaimed}`.
  - `F` kann bei `StaleMapping` bis zu 3-mal aufgerufen werden und muss deterministisch sein.
  - Bei Fehler bleibt nichts vorgemerkt.
- **`table_swap`:** `Epoch = u64`. `SegmentTable::add(Region)` und `remove(&Region) -> bool`. `TableSwap::min_reader_epoch() -> Epoch`. Tabellen halten `bookkeeping::Region`, also `Segment.region`.
- **`arenas`:** `ArenaKind::IndexSegments` (ID 1) ist die Arena aller Segmente.
- **Nicht genutzt (nur Abgrenzung):** `manifest`, `address_map`, `update_queue`.
- **Logging:** `log`-Crate (`warn!`).