# Spezifikation: Segment-Arena (Build-Order 87)

## 1. Task, restrictions and boundaries

**Task.** Rust-Modul `segment_arena`. Es teilt Segmente ausgerichtet zu und gibt sie frei. Es hält eine Reserve zurück und plant Replacement-Headroom ein. Es verwaltet ein Replacement durch Kopie samt Publikation und Zurückstellung des alten Segments. Es führt Budgets und Bookkeeping und liefert schrittweise beschreibbare Bump-Segmente für einen Writer.

**Nicht Teil des Moduls:**
- Kein eigener Speicher und keine eigene Platzierung. Alles läuft über `allocator::Allocator`.
- Kein Kopieren von Segmentinhalten. Der Aufrufer beschreibt die Kopie selbst.
- Keine Epochen- oder Reader-Verwaltung. Das macht `TableSwap`.
- Keine Policy, wann zugeteilt, ersetzt oder freigegeben wird.
- Keine Neudefinition von `Region`, `Allocator`, `AllocatorError`, `Event`, `TableSwap`, `SegmentTable` oder `Epoch`.

## 2. Context

- Die Arena sitzt über dem `Allocator` einer Arena (typisch `IndexSegments`). Er liefert Kapazität, Zuteilung, sofortige und Event-verzögerte Freigabe sowie Liveness.
- `TableSwap` publiziert die Segmenttabelle mit Epochen. `finish_replace` nutzt `next_table`, `SegmentTable::replace` und `publish`. Die Epoche der Publication ist die Retire-Epoche des alten Segments.
- Das Reclaim bekommt die kleinste Reader-Epoche vom Aufrufer, in der Regel `TableSwap::min_reader_epoch()`.
- Der Allocator hat Alignment 1. Die Ausrichtung erzeugt dieses Modul, indem es Slack mit allokiert.
- Der Aufrufer wählt für den Speicherverwalter `&Allocator` oder `Arc<Allocator>`.

## 3. Interface

```rust
use std::borrow::Borrow;
use allocator::{Allocator, AllocatorError};
use bookkeeping::Region;
use device::Event;
use table_swap::{Epoch, TableSwap, TableSwapError};

pub const MIN_ALIGNMENT: u64 = 256;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SegmentArenaConfig {
    /// Zweierpotenz >= 256. Default 256.
    pub alignment: u64,
    /// Reserve in Basispunkten der Kapazität (0..=10_000). Default 0.
    pub reserve_bp: u32,
    /// Zahl gleichzeitiger Replacements für die Headroom-Rechnung. Default 1.
    pub concurrent_replacements: u32,
}
impl Default for SegmentArenaConfig {}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SegmentArenaError {
    /// Display: "Arena erschöpft" (mit requested/available).
    Exhausted { requested: u64, available: u64 },
    /// Display: "stale Region".
    StaleRegion,
    /// Display: "kein Replacement".
    NoReplacement,
    ReplacementInProgress,
    ZeroLength,
    InvalidAlignment,
    InvalidConfig,
    /// Fehler von TableSwap::publish; Replacement bleibt bestehen.
    Publish(TableSwapError),
    /// Altes Segment fehlt in der nächsten Tabelle; Replacement bleibt bestehen.
    OldNotInTable,
    /// Übrige Allocator-Fehler, gekapselt.
    Allocator(AllocatorError),
}
impl std::fmt::Display for SegmentArenaError {}
impl std::error::Error for SegmentArenaError {}

/// Ausgerichtetes Segment. `region` ist die zugrunde liegende Allocator-Region (inkl. Slack).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Segment {
    pub region: Region,
    /// Absoluter, ausgerichteter Arena-Offset (>= region.offset).
    pub offset: u64,
    /// Nutzbare Länge = angeforderte Länge aufgerundet auf Alignment.
    pub len: u64,
}

pub enum Readers { None, After(Event) }   // Freigabe: sofort / nach Event

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ReplacementBudget { pub live_bytes: u64, pub replacements: u32, pub largest_segment: u64 }
impl ReplacementBudget {
    /// replacements * largest_segment, saturierend.
    pub fn headroom(&self) -> u64;
    /// live_bytes + incoming; None bei Überlauf.
    pub fn swap_need(&self, incoming: u64) -> Option<u64>;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ArenaBudget {
    pub capacity: u64, pub reserve: u64, pub live: u64,
    pub deferred: u64, pub headroom: u64, pub available: u64,
}
impl ArenaBudget {
    /// Belegtes / capacity als 0.0..=1.0; capacity == 0 => 0.0.
    pub fn utilization(&self) -> f64;
}
impl std::fmt::Display for ArenaBudget {}   // loggbar, enthält alle Felder und Auslastung in %

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SegmentStats {
    pub live_bytes: u64, pub live_segments: usize, pub largest_segment: u64,
    pub deferred_segments: usize, pub deferred_bytes: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ReplacementInfo { pub old: Segment, pub new: Segment }

/// Send + Sync, wenn A es ist.
pub struct SegmentArena<A: Borrow<Allocator>> { /* Mutex<State> */ }

impl<A: Borrow<Allocator>> SegmentArena<A> {
    pub fn new(allocator: A, config: SegmentArenaConfig) -> Result<Self, SegmentArenaError>;

    pub fn allocate(&self, len: u64) -> Result<Segment, SegmentArenaError>;
    pub fn allocate_bump(&self, len: u64) -> Result<BumpSegment, SegmentArenaError>;
    pub fn release(&self, seg: &Segment, readers: Readers) -> Result<(), SegmentArenaError>;

    pub fn begin_replace(&self, old: &Segment) -> Result<Segment, SegmentArenaError>;
    pub fn finish_replace(&self, swap: &TableSwap) -> Result<Epoch, SegmentArenaError>;
    pub fn discard_replacement(&self) -> Result<(), SegmentArenaError>;
    pub fn replacement(&self) -> Option<ReplacementInfo>;

    /// Gibt alle zurückgestellten Regionen mit epoch <= min_reader_epoch frei. Liefert die Anzahl.
    pub fn reclaim(&self, min_reader_epoch: Epoch) -> usize;

    pub fn budget(&self) -> ArenaBudget;
    pub fn replacement_budget(&self) -> ReplacementBudget;
    pub fn stats(&self) -> SegmentStats;
}

/// Bump-Allocation für genau einen Writer (&mut).
#[derive(Debug)]
pub struct BumpSegment { /* segment, cursor */ }
impl BumpSegment {
    pub fn new(segment: Segment) -> BumpSegment;
    /// Absolute, auf `align` ausgerichtete Adresse (Arena-Offset). align: Zweierpotenz >= 1.
    pub fn claim(&mut self, len: u64, align: u64) -> Result<u64, SegmentArenaError>;
    pub fn written(&self) -> u64;
    pub fn remaining(&self) -> u64;
    pub fn segment(&self) -> &Segment;
    pub fn into_parts(self) -> (Segment, u64 /* written */);
}
```

## 4. Behavior and invariants

**Konfiguration**
- `new` gibt `InvalidConfig` zurück, wenn `alignment` keine Zweierpotenz oder kleiner als 256 ist, `reserve_bp > 10_000` gilt oder `concurrent_replacements == 0` ist.
- `reserve = ceil(capacity * reserve_bp / 10_000)`, berechnet in `u128`. Die Kapazität ist `Allocator::capacity()`. Die Reserve wird nie vergeben.

**Größen und Ausrichtung**
- `len == 0` ergibt `ZeroLength`.
- Segmentlänge `L = len` aufgerundet auf `alignment`. Das Aufrunden prüft auf Überlauf, bei Überlauf kommt `Exhausted`.
- Allokationslänge `A = L + (alignment - 1)`. Das ist die Länge der Region, ebenfalls überlaufgeprüft.
- `Segment.offset = region.offset` aufgerundet auf `alignment`. Es gilt `offset % alignment == 0` und `offset + L <= region.offset + region.len`.
- „Größe“ in Budget und Statistik ist immer `A` (Region-Länge).

**Budgets (alle overflow-sicher, saturierend bzw. checked)**
- `occupied = capacity.saturating_sub(allocator.available())`. Das schließt wartende Event-Freigaben, zurückgestellte Segmente und fremde Regionen des Allocators ein.
- `headroom = concurrent_replacements * largest_segment`, saturierend.
- `available = capacity − occupied − reserve − headroom`, saturierend, nie negativ.
- `budget()` setzt die Felder so:
  - `live` = Bytes der lebenden Segmente. Dazu zählt eine wartende Kopie, zurückgestellte Segmente zählen nicht.
  - `deferred` = Bytes der zurückgestellten Segmente.
  - `headroom` und `available` nutzen das aktuelle `largest_segment`.
- `budget()`, `replacement_budget()` und `stats()` haben keine Seiteneffekte. Sie sammeln nichts ein.
- `replacement_budget()` liefert:
  - `live_bytes = occupied`,
  - `replacements = concurrent_replacements`,
  - `largest_segment` wie in `stats()`.
- `stats().largest_segment` ist das Maximum der Größen `A` über alle verfolgten Segmente: lebende, zurückgestellte und die wartende Kopie. Es sinkt, wenn das größte Segment freigegeben wird. Ohne Segmente ist es 0.
- Bookkeeping wird bei Zuteilung, Release, Replacement und Reclaim aktuell gehalten.

**allocate**
- Atomar unter dem internen Lock.
- Ablauf:
  1. `allocator.reap()` aufrufen, also ausstehende Freigaben einsammeln, auch vor der Prüfung.
  2. Headroom für die Prüfung mit dem neuen Segment berechnen: `H' = concurrent_replacements * max(largest_segment, A)`.
  3. Zulassen nur, wenn `occupied + A + H' + reserve <= capacity` gilt (checked). Sonst kommt `Exhausted { requested: A, available: capacity − occupied − reserve − H' (saturierend) }`.
  4. `allocator.allocate(A)` aufrufen.
- Der Zustand bleibt bei Fehlern unverändert, abgesehen vom Einsammeln.
- Das Segment wird verfolgt und `largest_segment` aktualisiert.
- Es gibt ein `log::debug!` mit der Budget-Anzeige, die auch die Auslastung enthält.
- Die Fehler des Allocators werden so abgebildet:
  - `OutOfMemory{requested, free}` ergibt `Exhausted { requested: A, available: free }`. Das ist der Fall bei Fragmentierung, Fault Injection oder einer Allocator-seitigen Kapazitätsgrenze.
  - `StaleRegion` ergibt `StaleRegion`.
  - Alles andere ergibt `Allocator(e)`.
- `allocate_bump(len)` ist `allocate(len)` plus `BumpSegment::new`.

**release**
- Atomar unter dem Lock.
- Es gibt `StaleRegion` zurück, wenn eines davon zutrifft:
  - die Region wird nicht von dieser Arena verfolgt,
  - sie ist nicht live (`Allocator::is_live`),
  - sie ist zurückgestellt (in der Reclaim-Liste),
  - der Allocator meldet `StaleRegion`.
- `Readers::None` ruft `allocator.free(&region)` auf (sofort).
- `Readers::After(ev)` ruft `allocator.free_deferred(&region, ev)` auf.
- Bei Erfolg wird das Segment sofort aus dem Bookkeeping entfernt, auch bei Event-Verzögerung.
- Bei Fehler bleibt der Zustand unverändert.
- Andere Allocator-Fehler werden gekapselt.

**Replacement durch Kopie**
- Es gibt höchstens ein Replacement gleichzeitig, auch wenn `concurrent_replacements > 1` ist. Der Wert beeinflusst nur den Headroom.
- `begin_replace(old)` prüft und arbeitet in dieser Reihenfolge:
  1. Läuft schon ein Replacement, kommt `ReplacementInProgress`.
  2. Dann `allocator.reap()`.
  3. `old` muss verfolgt, live und nicht zurückgestellt sein, sonst `StaleRegion`.
  4. Der Speicherbedarf des Wechsels ist `swap_need(A_old) = occupied + A_old`. Zugelassen wird nur, wenn dieser Bedarf plus `reserve` in die Kapazität passt (checked). Sonst kommt `Exhausted { requested: A_old, available: capacity − occupied − reserve (saturierend) }`.
  5. Der Headroom wird hier nicht erneut verlangt, weil er bei der regulären Zuteilung bereits freigehalten wurde.
  6. Dann folgt eine Allokation gleicher Größe `A_old`. Die Fehlerabbildung ist dieselbe wie bei `allocate`.
- Die Kopie ist ein neues Segment gleicher Länge neben dem alten. Das alte bleibt unverändert live. Die Kopie wird verfolgt und zählt als lebend.
- `replacement()` liefert `Some(ReplacementInfo{old,new})` oder `None`. Damit sind alte und neue Region abfragbar.
- `finish_replace(swap)` läuft atomar unter dem Arena-Lock. Ohne Replacement kommt `NoReplacement`.
  1. `let mut t = swap.next_table()`.
  2. `t.replace(&old.region, new.region)`. Fehlt `old`, kommt `OldNotInTable`.
  3. `swap.publish(t)`. Ein Fehler ergibt `Publish(e)`.
  - Bei jedem Fehler bleiben Kopie und Replacement-Zustand erhalten, sodass `finish_replace` erneut aufgerufen oder `discard_replacement` gerufen werden kann.
  - Bei Erfolg wird das Replacement beendet. Das alte Segment wird mit `epoch` = zurückgegebene publizierte Epoche zurückgestellt. Es bleibt im Allocator live, bis es per `reclaim` freigegeben wird. Es zählt dann zu `deferred`, nicht mehr zu `live`.
  - Zurückgegeben wird die Epoche.
  - Die Tabelle speichert die zugrunde liegende `Segment.region`.
- `discard_replacement()` ruft `allocator.free(&new.region)` auf. Es beendet das Replacement in jedem Fall: bei Erfolg und auch dann, wenn die Region schon stale ist (Fehler `StaleRegion`). Ohne Replacement kommt `NoReplacement`. Das alte Segment bleibt unberührt.

**reclaim(min_reader_epoch)**
- Es gibt alle zurückgestellten Regionen mit `epoch <= min_reader_epoch` per `allocator.free` frei.
- Bei Erfolg wird der Eintrag entfernt und gezählt.
- Bei `Err` (Region verschwunden) wird der Eintrag entfernt, es gibt nur `log::warn!` und der Eintrag wird nicht gezählt. Kein Fehler, kein Panic.
- Ist die Epoche größer als der Grenzwert, bleibt der Eintrag bestehen.
- Rückgabe ist die Anzahl wirklich freigegebener Regionen. `reclaim` ist idempotent. `largest_segment` und `deferred` werden angepasst.

**BumpSegment**
- Der Cursor startet bei 0. Die Basis ist `segment.offset` und die Kapazität `segment.len`.
- `claim(len, align)` rechnet so:
  - `align` muss eine Zweierpotenz >= 1 sein, sonst `InvalidAlignment`.
  - `addr = align_up(base + cursor, align)` und `end = addr + len`, alles checked.
  - Passt `end <= base + capacity`, wird `cursor = end − base` gesetzt und `addr` (absolute Adresse) zurückgegeben.
  - Passt es nicht oder überläuft die Rechnung, kommt `Exhausted { requested: len, available: remaining() }` ohne Zustandsänderung.
- Der Alignment-Padding zählt als geschrieben.
- `len == 0` ist erlaubt und liefert die ausgerichtete Adresse.
- Es gilt `written() = cursor`, `remaining() = capacity − cursor` und `written + remaining == capacity`.
- `into_parts` zerlegt in `(Segment, written)`. Das Segment bleibt beim Aufrufer zur Freigabe über die Arena.
- `BumpSegment` hat keine Synchronisation. Ein Writer, `&mut self`.

**Concurrency**
- Alle `SegmentArena`-Methoden sind atomar über einen internen `Mutex`. Parallele `allocate`-Aufrufe überschreiten die Budgetgrenzen nie. Parallele `release` derselben Region: genau eine gelingt, die anderen ergeben `StaleRegion`.
- Lock-Reihenfolge: Arena-Lock → TableSwap (publish) bzw. Allocator-intern. Es gibt keine umgekehrte Reihenfolge.
- Ein vergiftetes Lock wird übernommen, ohne Panic.

## 5. Relied-on elements

- **`bookkeeping::Region`**: `{ arena: ArenaKind, offset: u64, len: u64, generation: u32 }`, `Copy, Eq, Hash`.
- **`allocator::Allocator`** (`Send + Sync`):
  - `capacity() -> u64`
  - `available() -> u64`, d. h. capacity − live Bytes, sammelt nicht ein
  - `allocate(len) -> Result<Region, AllocatorError>`, sammelt intern vorher ein
  - `free(&Region) -> Result<(), AllocatorError>`
  - `free_deferred(&Region, Event) -> Result<(), AllocatorError>`
  - `reap() -> usize`
  - `is_live(&Region) -> bool`
  - `AllocatorError::{ OutOfMemory{arena, requested, free}, AllocationFailed, StaleRegion }`
- **`device::Event`**: `Clone + Send + Sync`. Ein Test-Mock ist `Event::mock()`.
- **`table_swap`**:
  - `Epoch = u64`
  - `TableSwap::next_table() -> SegmentTable`, Epoche = publiziert + 1
  - `TableSwap::publish(SegmentTable) -> Result<Epoch, TableSwapError>`, `StaleMapping` bei veralteter Basis
  - `TableSwap::min_reader_epoch() -> Epoch`
  - `SegmentTable::replace(&mut self, old: &Region, new: Region) -> bool`, false wenn old fehlt
  - `TableSwapError` ist `Clone, Debug, PartialEq, Eq`
- **Logging**: `log`-Crate (`debug!`, `warn!`).