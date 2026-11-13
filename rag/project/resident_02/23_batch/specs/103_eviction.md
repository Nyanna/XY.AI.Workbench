# Spezifikation: Eviction (Build-Order 103)

## 1. Task, restrictions and boundaries

**Task.** Rust-Modul `eviction`. Es verfolgt abgeleitete, verwerfbare Mirrors (Segmente) in den Tiers VRAM und RAM. Es evictet sie explizit aus drei Gründen:
- Idle-Timeout,
- Pressure (Füllstand über der High-Watermark, mit Hysterese bis zur Low-Watermark),
- Space-Need (Platzbeschaffung).

Die Victim-Auswahl ist kostenbewusst nach GreedyDual-Size-Frequency (GDSF). Beim Evicten entfernt das Modul das Segment aus Segmenttabelle und Address-Map, publiziert beide, stellt die Region hinter der neuen Epoche zurück und gibt sie später frei. Jede Entscheidung wird geloggt.

**Nicht Teil des Moduls:**
- Es entscheidet nicht, was gemirrort wird, und lädt nichts nach.
- Es deaktiviert keinen Mirror. Eviction entlädt nur das Segment, und derselbe Key darf später wieder per `track` aufgenommen werden.
- Es führt keine Slot-Verwaltung aus. `SegmentStore::remove_segment` wird nicht aufgerufen, und `SegmentArena::release` wird für evictete Regionen nicht benutzt. Der Aufrufer darf evictete Regionen nicht zusätzlich selbst freigeben.
- Es ermittelt keine Reader-Epochen. `reclaim` bekommt `min_reader_epoch` vom Aufrufer, in der Regel `TableSwap::min_reader_epoch()`.
- Es hat keinen eigenen Timer. Der Aufrufer ruft `tick` periodisch auf.
- Es definiert `Region`, `SegmentId`, `Epoch`, `Pin`, `PinCount`, `Allocator`, `TableSwap` und `MappingStore` nicht neu.
- Summaries und gepinnte Entries werden nie Victim.

## 2. Context

- Der Lader und der Mirror-Manager rufen `track` beim Laden eines Segments. Query-Pfade rufen `touch` und `pin`. Ein Scheduler ruft periodisch `tick`. Wer Platz braucht, ruft `make_room`.
- Die Segmenttabelle (`table_swap::TableSwap`, Regionen) und die Address-Map (`address_map::MappingStore`) werden per Copy-on-Write-Publication geändert. Die Epoche der Tabellen-Publication ist die Retire-Epoche der Region.
- Die Region wird über `AllocatorSet::allocator_for(&region)` freigegeben. Das passiert erst in `reclaim(min_reader_epoch)`, wenn kein Reader die alte Tabelle mehr hält.
- Region-Pinning: Der Halter hält einen `Pin<EvictKey>`. Solange er lebt, ist der Entry nie Victim.
- Kapazität: Das VRAM-Budget ergibt sich aus der Kapazität der Arena `IndexSegments` (`ArenaCapacities`), der Generationsreserve und dem Headroom. Das RAM-Budget gibt der Aufrufer direkt an.

## 3. Interface

```rust
use std::sync::Arc;
use std::time::{Duration, Instant};
use address_map::{MappingStore, SegmentId, MapError};
use allocator::AllocatorSet;
use arenas::ArenaCapacities;
use bookkeeping::Region;
use region_pinning::Pin;                      // Pin<R>, PinCount
use table_swap::{Epoch, TableSwap, TableSwapError};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EvictKey(pub u64);                 // Victim-Key; Ordnung = aufsteigend nach u64

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Tier { Vram, Ram }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EvictReason { Idle, Pressure, SpaceNeed }   // Display: "idle" | "pressure" | "space-need"

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EvictionPolicy { pub high: f64, pub low: f64, pub idle: Duration }
impl EvictionPolicy {
    pub fn vram() -> Self;   // 0.90, 0.75, 600 s
    pub fn ram() -> Self;    // 0.85, 0.70, 60 s
    /// Korrigiert die Marken für `tier` (siehe Abschnitt 4).
    pub fn sanitized(self, tier: Tier) -> Self;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct TierBudget { pub total: u64, pub reserve: u64, pub headroom: u64 }
impl TierBudget {
    pub fn new(total: u64, reserve: u64, headroom: u64) -> Self;
    /// total = caps.get(ArenaKind::IndexSegments); reserve = caps.generation_reserve(generations).
    pub fn for_vram(caps: &ArenaCapacities, generations: u32, headroom: u64) -> Self;
    /// total - reserve - headroom, saturierend, nie negativ.
    pub fn usable(&self) -> u64;
}

#[derive(Clone, Copy, Debug)]
pub struct TrackSpec {
    pub tier: Tier,
    pub segment: SegmentId,     // Eintrag in der Address-Map
    pub region: Region,         // Eintrag in der Segmenttabelle
    pub size: u64,              // Bytes; zählt zur Belegung
    pub reload_cost: f64,       // nicht endlich oder negativ => 0.0
    pub summary: bool,          // Summary: nie Victim
}

#[derive(Clone, Debug, PartialEq)]
pub struct EvictionDecision {
    pub index: String, pub victim: EvictKey, pub segment: SegmentId,
    pub size: u64,      // Größe wie in der Priorität benutzt: max(size, 1)
    pub bytes: u64,     // getrackte Bytes, die der Entry belegte
    pub hits: u64, pub reason: EvictReason,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AbortCause { Table(TableSwapError), Map(MapError) }
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Abort { pub key: EvictKey, pub cause: AbortCause }

#[derive(Clone, Debug, Default, PartialEq)]
pub struct EvictionOutcome {
    pub decisions: Vec<EvictionDecision>,   // tatsächlich evictet, in Reihenfolge
    pub skipped: Vec<EvictKey>,             // unbekannte Region, siehe Abschnitt 4
    pub aborted: Option<Abort>,             // Abbruch durch Fehler
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EvictionError {
    /// Display: "Arena erschöpft". Es wurde nichts evictet.
    Exhausted { requested: u64, available: u64 },
}
impl std::fmt::Display for EvictionError {}
impl std::error::Error for EvictionError {}

/// Send + Sync; alle Methoden &self.
pub struct Eviction { /* ... */ }
impl Eviction {
    pub fn new(index: impl Into<String>, swap: Arc<TableSwap>, store: Arc<MappingStore>,
               allocators: Arc<AllocatorSet>, vram: TierBudget, ram: TierBudget) -> Eviction;
    // Konfiguration (Defaults: EvictionPolicy::vram()/ram())
    pub fn set_policy(&self, tier: Tier, policy: EvictionPolicy);
    pub fn policy(&self, tier: Tier) -> EvictionPolicy;       // bereits sanitized
    pub fn set_budget(&self, tier: Tier, budget: TierBudget);

    // Verfolgung
    pub fn track(&self, key: EvictKey, spec: TrackSpec, now: Instant);
    pub fn touch(&self, key: EvictKey, now: Instant) -> bool; // false: unbekannt
    pub fn remove(&self, key: EvictKey) -> Option<u64>;       // Bytes; nur Verfolgung, keine Tabellenänderung
    pub fn pin(&self, key: EvictKey) -> Option<Pin<EvictKey>>;
    pub fn is_pinned(&self, key: EvictKey) -> bool;
    pub fn contains(&self, key: EvictKey) -> bool;
    pub fn len(&self) -> usize;
    pub fn len_tier(&self, tier: Tier) -> usize;
    pub fn bytes(&self, tier: Tier) -> u64;          // lebende Bytes
    pub fn deferred_bytes(&self, tier: Tier) -> u64; // zurückgestellte Bytes
    pub fn occupancy(&self, tier: Tier) -> u64;      // bytes + deferred_bytes (saturierend)
    pub fn usable(&self, tier: Tier) -> u64;
    pub fn priority(&self, key: EvictKey) -> Option<f64>;
    pub fn hits(&self, key: EvictKey) -> Option<u64>;
    pub fn floor(&self, tier: Tier) -> f64;

    // Eviction
    pub fn evict_idle(&self, tier: Tier, now: Instant) -> EvictionOutcome;
    pub fn relieve_pressure(&self, tier: Tier) -> EvictionOutcome;
    /// Je Tier (Vram, dann Ram): evict_idle, dann relieve_pressure.
    pub fn tick(&self, now: Instant) -> EvictionOutcome;
    pub fn make_room(&self, tier: Tier, need: u64) -> Result<EvictionOutcome, EvictionError>;
    /// Gibt zurückgestellte Regionen mit retire_epoch <= min_reader_epoch frei; Rückgabe = Anzahl.
    pub fn reclaim(&self, min_reader_epoch: Epoch) -> usize;
}
```

## 4. Behavior and invariants

**Policy**
- `sanitized(tier)`:
  - Nicht endliches `high` oder `low` (NaN, ±∞) wird durch den Default des Tiers ersetzt.
  - `high` wird auf `[0,1]` begrenzt, danach `low` auf `[0,high]`.
  - `idle` bleibt unverändert.
- `set_policy` speichert die korrigierte Policy.
- Defaults: VRAM 0,90 / 0,75 / 600 s, RAM 0,85 / 0,70 / 60 s.

**Kapazität und Belegung**
- `usable = total − reserve − headroom`, saturierend.
- `occupancy = lebende + zurückgestellte Bytes` des Tiers. Gepinnte Entries und Summaries zählen zur Belegung.
- Bytes eines evicteten Entries wechseln von lebend zu zurückgestellt. Die Belegung sinkt erst durch `reclaim`.
- `free = usable.saturating_sub(occupancy)`.

**Track / Touch / Remove**
- `track` legt den Entry mit `hits = 1`, `last_used = now` und `priority = floor(tier) + 1 · cost / max(size,1)` an.
- Erneutes `track` desselben Keys ersetzt den Entry vollständig:
  - Hits werden auf 1 zurückgesetzt, Spec, Zeit und Priorität neu gesetzt.
  - Die Belegung wird korrigiert, auch bei Tier-Wechsel.
  - Der bestehende `PinCount` bleibt erhalten, sodass lebende Pins weiter gelten.
- Die Priorität ist `floor + hits · cost / max(size,1)` in `f64`, mit `cost` nach Korrektur.
- `touch` erhöht `hits` saturierend, setzt `last_used = now` und berechnet die Priorität mit dem aktuellen Floor neu. Unbekannter Key: `false`, ohne Wirkung.
- `remove` entfernt die Verfolgung und liefert `Some(size)`, sonst `None`. Dabei wird nichts publiziert oder freigegeben. Der Pin-Zähler wird nicht geprüft.
- `pin(key)` liefert `Some(Pin::new(key, &count))` für verfolgte Keys, sonst `None`. Pinning wirkt, solange der `Pin` lebt, auch über `touch` und erneutes `track` hinweg.

**Victim-Eignung und -Ordnung**
- Eignung: Der Entry ist im Tier, nicht `summary` und `PinCount::is_unpinned()`.
- Ordnung (aufsteigend): niedrigste Priorität (`f64::total_cmp`), dann kleinstes `last_used` (am längsten unbenutzt), dann kleinerer `EvictKey`.

**evict_idle(tier, now)**
- Betroffen sind alle geeigneten Entries mit `now − last_used >= policy(tier).idle`.
- Reihenfolge ist aufsteigend nach `EvictKey`. Grund ist `Idle`. Der Floor ändert sich nicht.

**relieve_pressure(tier)**
- Nichts passiert, solange nicht `occupancy as f64 > high · usable as f64` gilt. Bei `usable == 0` und `occupancy > 0` gilt Pressure.
- Sonst werden Victims in Victim-Ordnung evictet, bis die projizierte Belegung `<= low · usable` ist. Dabei zählen die Bytes jedes Victims als sofort frei.
- Reichen alle geeigneten Entries nicht, werden alle evictet (Best Effort, kein Fehler).
- Grund ist `Pressure`.
- Hinweis für den Aufrufer: Zurückgestellte Bytes zählen bis `reclaim` als belegt. Er ruft `reclaim` daher vor `tick` auf.

**make_room(tier, need)**
- Gilt `need <= free` (auch `need == 0`), passiert nichts und es kommt `Ok` mit leerem Outcome.
- Sonst ist `shortfall = need − free`. Geeignete Victims werden in Victim-Ordnung gesammelt, bis ihre Bytes `>= shortfall` erreichen.
- Reicht die Summe aller geeigneten Entries nicht, kommt `Err(Exhausted{requested: need, available: free + Summe})`. Es wird nichts evictet und nichts geändert.
- Sonst wird mit Grund `SpaceNeed` evictet.
- Wurde ein geplanter Victim inzwischen gepinnt oder entfernt, wird er übersprungen. Der Aufrufer prüft das Ergebnis über `occupancy` bzw. `aborted`.

**tick(now)**
- Pro Tier in der Reihenfolge Vram, Ram: erst `evict_idle`, dann `relieve_pressure`.
- Die Outcomes werden zusammengeführt. Nach dem ersten `aborted` endet der Tick.

**Floor**
- Je Tier, Start 0.
- Bei jeder Eviction mit Grund `Pressure` oder `SpaceNeed` wird der Floor des Tiers auf die gespeicherte Priorität des Victims gesetzt, und zwar nach dessen erfolgreicher Eviction. Der Floor sinkt nie.
- `Idle` ändert den Floor nicht.
- Gespeicherte Prioritäten anderer Entries ändern sich nur durch `track` oder `touch`.

**Entfernen eines Victims** (je Victim, in dieser Reihenfolge)
1. Unter dem State-Lock wird erneut geprüft (noch verfolgt, geeignet) und der Entry atomar aus der Verfolgung genommen. Ist er nicht mehr geeignet, wird er übersprungen und nicht als `skipped` gemeldet.
2. Es wird `t = swap.next_table()` und `m = store.next()` gebildet. Dann folgt `in_table = t.remove(&region)` und `in_map = m.remove(segment)`. Ein Fehler von `store.next()` ist ein Abbruch (siehe Schritt 7).
3. Sind `in_table` und `in_map` beide false (unbekannte Region), wird der Entry übersprungen:
   - Er wird in `skipped` gemeldet, mit `log::warn!`.
   - Er bleibt unverfolgt.
   - Es wird nichts publiziert, zurückgestellt oder freigegeben, und es gibt keine Decision.
   - Die Verarbeitung geht zum nächsten Victim.
4. Ist `in_table`, gilt `epoch = swap.publish(t)?`. Ist nicht `in_table`, gilt `epoch = swap.published_epoch()`.
5. Ist `in_map`, folgt `store.publish(m)`. Schlägt das mit `StaleMapping` fehl, wird mit frischem `store.next()` und `remove(segment)` wiederholt, bis zu 3 Versuche insgesamt. Bleibt der Fehler bestehen, gibt es `log::warn!`, und die Eviction gilt trotzdem als vollzogen, weil die Tabelle das Segment schon nicht mehr enthält.
6. `(region, epoch, bytes, tier)` kommt in die Liste der zurückgestellten Regionen. Danach wird der Floor aktualisiert, die Decision gebildet und mit `log::info!` geloggt (Format: `eviction index=… victim=… segment=… size=… bytes=… hits=… reason=…`). Die Decision kommt in `decisions`.
7. Abbruch bei Fehler in Schritt 2 oder 4 (`Abort{key, cause}`):
   - Der Entry wird mit allen Feldern (Hits, `last_used`, Priorität, `PinCount`) wieder unter Kontrolle gestellt. War der Key inzwischen neu getrackt, gilt der neuere Entry.
   - Es gibt keine Decision für diesen Victim.
   - Die weitere Verarbeitung der Operation endet. Bisherige Decisions bleiben gültig.
- Bei einem Abbruch vor Schritt 4 sind Tabelle und Map unverändert.

**reclaim(min_reader_epoch)**
- Jede zurückgestellte Region mit `epoch <= min_reader_epoch` wird über `allocators.allocator_for(&region)?.free(&region)` freigegeben.
- Bei `Ok` wird der Eintrag entfernt, die zurückgestellten Bytes sinken und der Eintrag wird gezählt.
- Bei `Err` wird der Eintrag entfernt, es gibt nur `log::warn!`, und er wird nicht gezählt.
- Einträge mit größerer Epoche bleiben bestehen.
- Rückgabe ist die Anzahl freigegebener Regionen. `reclaim` ist idempotent.

**Concurrency**
- `Eviction` ist `Send + Sync`.
- Ein interner Evict-Mutex serialisiert alle Eviction-Operationen (`evict_idle`, `relieve_pressure`, `make_room`, `tick`). Ein State-Mutex schützt Entries, Floors und die Liste der zurückgestellten Regionen.
- Lock-Reihenfolge: Evict-Mutex → State-Mutex. Der State-Lock wird nie während Aufrufen an `TableSwap`, `MappingStore` oder `Allocator` gehalten.
- `track`, `touch`, `pin` und Abfragen laufen parallel zur Eviction.
- Ein vergifteter Lock wird übernommen (`into_inner`). Es gibt keine Panics.
- Die Prüfung "ungepinnt" und das Herausnehmen des Entries geschehen atomar unter dem State-Lock.

**Determinismus.** Bei gleichem Zustand und gleichen `now`-Werten sind Auswahl und Reihenfolge der Victims identisch. Alle Zeiten kommen als `Instant` vom Aufrufer, das Modul liest keine Uhr.

## 5. Relied-on elements

- **`bookkeeping::Region`**: `{ arena, offset, len, generation }`, `Copy + Eq + Hash`.
- **`address_map`**:
  - `SegmentId(pub u32)` (`Copy, Eq, Ord, Hash`).
  - `MappingStore::next(&self) -> Result<AddressMap, MapError>` (Epoche = aktuell + 1).
  - `MappingStore::publish(&self, AddressMap) -> Result<Epoch, MapError>`, `StaleMapping` bei veralteter Basis.
  - `AddressMap::remove(&mut self, SegmentId) -> bool`.
  - `MapError` ist `Clone, Debug, PartialEq, Eq`.
- **`table_swap`**:
  - `Epoch = u64`.
  - `TableSwap::next_table(&self) -> SegmentTable` (Epoche = publiziert + 1).
  - `TableSwap::publish(&self, SegmentTable) -> Result<Epoch, TableSwapError>`.
  - `TableSwap::published_epoch(&self) -> Epoch`.
  - `SegmentTable::remove(&mut self, &Region) -> bool`.
  - `TableSwapError` ist `Clone, Debug, PartialEq, Eq`.
  - Die Retire-Epoche einer Region ist die von `publish` zurückgegebene Epoche. Sie ist freigebbar, sobald `min_reader_epoch() >= epoche`.
- **`allocator`**:
  - `AllocatorSet::allocator_for(&self, &Region) -> Result<&Allocator, AllocatorError>`.
  - `Allocator::free(&self, &Region) -> Result<(), AllocatorError>`, `Err(StaleRegion)` bei ungültiger Region.
- **Region-Pinning**:
  - `PinCount` (`new`, `count`, `is_unpinned`, `Clone` teilt den Zähler).
  - `Pin::new(region: R, &PinCount)` erhöht den Zähler. `Drop` verringert ihn atomar.
  - `Pin<R>` ist `Send` bzw. `Sync`, wenn `R` es ist.
- **`arenas`**:
  - `ArenaCapacities::get(ArenaKind) -> u64`.
  - `ArenaCapacities::generation_reserve(generations: u32) -> u64`.
  - `ArenaKind::IndexSegments`.
- **Logging**: `log`-Crate (`info!`, `warn!`).