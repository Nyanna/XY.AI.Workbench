# Spezifikation: Allocator mit Deferred Free (Build-Order 60)

## 1. Task, restrictions and boundaries

**Task.** Rust-Modul `allocator`. Es stellt zwei Dinge bereit:
- `Allocator`: Allokation je Arena mit Fehlerunterscheidung. Zusätzlich Deferred Free: Eine Region wird erst nach Abschluss ihres `Event` frei und wiederverwendbar.
- `AllocatorSet`: genau ein Allocator je Arena. Er findet den zuständigen Allocator zu einer Region.

**Nicht Teil des Moduls:**
- Es gibt keine eigene Platzierung, Offset-Vergabe, Generationsverwaltung oder Liveness-Buchführung. Das macht ausschließlich `Bookkeeping`.
- Es legt keine Arenen an und führt keine Eviction aus.
- Es erzeugt keine Events und keine Lanes und enthält kein GPU-Linking. Tests nutzen `Event::mock()`.
- Es definiert `Region`, `Event`, `ArenaKind`, `Arena` und `DeviceKind` nicht neu.
- Es hat keine Policy, wann allokiert oder freigegeben wird.

## 2. Context

- Der Allocator ist die Schicht über `Bookkeeping` (Modul `bookkeeping`). `Bookkeeping` vergibt und gibt Regionen frei und prüft ihre Liveness.
- Der Allocator ergänzt:
  - das Vormerken von Freigaben bis zum Event-Abschluss,
  - die Fehlerabbildung,
  - die Zuordnung Arena → Allocator.
- Lader, Planer und Kernels nutzen den Allocator. Sie geben Regionen nach Kernel-Submission mit dem Event der Lane (`Lane::record_event`) per `free_deferred` frei.
- Handle-Typ ist `bookkeeping::Region`. Der Allocator nimmt keinen `RegionHandle` aus `region_handle` entgegen. `Bookkeeping::is_live` und `Bookkeeping::free` arbeiten auf `Region` und sind die einzige Autorität über Gültigkeit.
- Ein Event steht für alle Arbeit einer Lane bis zum Aufnehmen. Nutzt eine Region mehrere Streams, muss der Aufrufer ein Event übergeben, das alle Nutzungen abdeckt. Dieses Modul kennt nur ein Event je Region.

## 3. Interface

```rust
use std::sync::{Arc, Mutex};
use arenas::{Arena, ArenaKind};
use bookkeeping::{Bookkeeping, Region};
use device::{DeviceKind, Event};   // Crate-Pfad nach Workspace-Layout

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AllocatorError {
    /// Erschöpfung. `free` = sofort verfügbare Menge (siehe `available`).
    OutOfMemory { arena: ArenaKind, requested: u64, free: u64 },
    /// Jeder andere Allokationsfehler. Display: "Allokation fehlgeschlagen".
    AllocationFailed,
    /// Region ungültig, fremd, nicht live oder bereits vorgemerkt. Display: "stale Region".
    StaleRegion,
}
impl std::fmt::Display for AllocatorError {}
impl std::error::Error for AllocatorError {}

/// Thread-safe; Send + Sync.
pub struct Allocator { /* arena, capacity, device, Arc<Bookkeeping>, Mutex<Vec<Pending{region, event}>> */ }

impl Allocator {
    /// `arena.capacity` ist die Kapazität dieses Allocators.
    pub fn new(arena: Arena, device: DeviceKind, bookkeeping: Arc<Bookkeeping>) -> Allocator;

    pub fn arena(&self) -> ArenaKind;
    pub fn capacity(&self) -> u64;
    pub fn device_kind(&self) -> DeviceKind;

    /// Sammelt zuerst abgeschlossene Deferred Frees ein (`reap`), dann `Bookkeeping::allocate`.
    pub fn allocate(&self, len: u64) -> Result<Region, AllocatorError>;

    /// Sofortige Freigabe. Nur für live, nicht vorgemerkte Regionen dieser Arena.
    pub fn free(&self, region: &Region) -> Result<(), AllocatorError>;

    /// Merkt `region` vor; frei wird sie erst, wenn `event` abgeschlossen ist.
    /// Prüft danach sofort auf Abschluss (`reap`).
    pub fn free_deferred(&self, region: &Region, event: Event) -> Result<(), AllocatorError>;

    /// Gibt Regionen mit abgeschlossenem Event frei. Liefert die Anzahl tatsächlich freigegebener Regionen.
    pub fn reap(&self) -> usize;

    /// Sofort verfügbare Bytes: capacity.saturating_sub(Bookkeeping::live_bytes(arena)).
    /// Wartende Regionen sind noch live und zählen daher nicht mit. Sammelt nicht ein.
    pub fn available(&self) -> u64;

    /// Anzahl vorgemerkter, noch nicht freigegebener Regionen.
    pub fn pending_count(&self) -> usize;

    /// Liveness laut `Bookkeeping::is_live`. Eine vorgemerkte Region bleibt live, bis sie eingesammelt wurde.
    pub fn is_live(&self, region: &Region) -> bool;
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AllocatorSetError { DuplicateArena(ArenaKind) }

/// Thread-safe; Send + Sync. Unveränderlich nach `new`.
pub struct AllocatorSet { /* Map ArenaKind -> Arc<Allocator> */ }

impl AllocatorSet {
    /// Je Eintrag ein Allocator mit gemeinsamem `bookkeeping`. Doppelte ArenaKind => Err.
    pub fn new(bookkeeping: Arc<Bookkeeping>, arenas: &[(Arena, DeviceKind)])
        -> Result<AllocatorSet, AllocatorSetError>;

    pub fn allocator(&self, arena: ArenaKind) -> Option<&Allocator>;
    /// Zuständiger Allocator: `region.arena`. Arena nicht im Set => Err(StaleRegion).
    pub fn allocator_for(&self, region: &Region) -> Result<&Allocator, AllocatorError>;
    /// Größenvorgabe: konfigurierte Kapazität der Arena; None, wenn nicht im Set.
    pub fn size_hint(&self, arena: ArenaKind) -> Option<u64>;
    pub fn len(&self) -> usize;
}
```

## 4. Behavior and invariants

**Allocate**
- `allocate` ruft zuerst `reap()` auf, dann `Bookkeeping::allocate(arena, len)`.
- Fehlerabbildung:
  - `AllocError::OutOfSpace{requested, free, ..}` ergibt `OutOfMemory{arena: self.arena, requested, free}`. Das gilt auch für Fault Injection, weil sie nicht unterscheidbar ist.
  - `ZeroLength` und `ArenaNotCreated` ergeben `AllocationFailed`.
- `OutOfMemory.free` ist der Wert aus `Bookkeeping` (Kapazität minus live Bytes). Das ist dieselbe Menge wie `available()`. Wartende Regionen sind darin enthalten und zählen als belegt.
- Bei Fehlern bleibt der Zustand außer dem vorherigen Einsammeln unverändert.

**Free (sofort)**
- Prüfung und Freigabe laufen atomar unter dem Allocator-Lock.
- Es ergibt `Err(StaleRegion)`, wenn eines davon zutrifft:
  - `region.arena != self.arena`,
  - die Region ist vorgemerkt,
  - `Bookkeeping::free(self.arena, region)` liefert irgendeinen `FreeError`.
- Sonst ist die Region frei, `Ok(())`, und `available()` steigt um `len`.
- Fehler ändern den Zustand nicht.

**Free deferred**
- Atomar unter dem Allocator-Lock.
- Es ergibt `Err(StaleRegion)`, wenn:
  - die Region fremd ist (`region.arena != self.arena`),
  - sie nicht live ist (`!Bookkeeping::is_live`),
  - sie bereits vorgemerkt ist.
- Sonst wird `(region, event)` vorgemerkt, und `pending_count` steigt um 1. Danach folgt sofort `reap()`. Ist das Event schon abgeschlossen, ist die Region beim Rückkehren frei.
- Die Region bleibt bis zum Einsammeln live (`is_live == true`). Sie belegt weiter Platz und wird nicht wieder vergeben. Dadurch sieht laufende Arbeit nie wiederverwendeten Speicher (Stream-ordered Reuse).
- Eine vorgemerkte Region kann nicht sofort freigegeben und nicht erneut vorgemerkt werden.

**Reap**
- Für jede vorgemerkte Region wird `event.is_complete()` ohne Blockieren ausgewertet:
  - `Ok(true)`: `Bookkeeping::free(arena, &region)`.
    - Erfolg: Region wird entfernt und gezählt.
    - Fehler (Region nicht mehr gültig): Eintrag wird entfernt, es gibt nur `log::warn!` (Arena, Offset, Länge, Generation). Der Eintrag wird nicht gezählt und es gibt keinen Fehler und keinen Panic.
  - `Ok(false)`: bleibt vorgemerkt.
  - `Err(EventError)`: bleibt vorgemerkt, mit `warn!`. Die Region wird nie freigegeben, weil der Abschluss unbekannt ist. Spätere Aufrufe fragen erneut.
- Rückgabe ist die Anzahl der tatsächlich freigegebenen Regionen. `reap()` ohne Wartende gibt 0 zurück.
- `reap` ist idempotent. Ein zweiter Aufruf ohne neue Abschlüsse gibt 0 zurück.

**Abfragen**
- `available()` und `pending_count()` sind nebenwirkungsfrei und blockieren nicht.
- `available() <= capacity()` gilt immer.

**AllocatorSet**
- Genau ein Allocator je `ArenaKind` im Set. Doppelte Arena in `new` ergibt `DuplicateArena(kind)`.
- Alle Allocatoren teilen dasselbe `Bookkeeping`.
- `allocator_for` liefert den Allocator von `region.arena`. Ist die Arena nicht im Set, kommt `Err(StaleRegion)` (fremde ID).
- Ob die Region live ist, prüft `allocator_for` nicht.
- `allocator(kind)` liefert `None` für unbekannte Arenen.
- `size_hint(kind)` liefert die Kapazität dieses Allocators.
- `len()` ist die Anzahl der Allocatoren.

**Concurrency**
- `Allocator` und `AllocatorSet` sind `Send + Sync`.
- Der Zustand der vorgemerkten Regionen liegt hinter einem `Mutex`. Reihenfolge der Locks: erst Allocator-Lock, dann (intern) `Bookkeeping`. Es gibt keine umgekehrte Reihenfolge, daher keinen Deadlock.
- Parallele `free_deferred` derselben Region: genau ein Aufruf gelingt, die anderen ergeben `StaleRegion`.
- Parallele `reap`-Aufrufe geben jede Region genau einmal frei, und die Summe der Rückgaben ist die Gesamtzahl.
- Ein vergiftetes Lock wird nicht propagiert: Der innere Zustand wird übernommen, es gibt keinen Panic.

**Testbarkeit**
- Alles läuft ohne GPU mit `Event::mock()`.
  - `MockEventControl::complete()` lässt `reap` freigeben.
  - `reset()` macht die Region wieder wartend. Eine bereits freigegebene Region bleibt frei.
  - `fail()` hält die Region vorgemerkt.
- Mit `Event::cpu()` und `EventSignal::signal()` geht das ebenso.

## 5. Relied-on elements

**Aus `bookkeeping`:**
```rust
pub struct Region { pub arena: ArenaKind, pub offset: ArenaOffset, pub len: u64, pub generation: u32 } // Copy, Eq, Hash
pub enum AllocError { OutOfSpace { arena: ArenaKind, requested: u64, free: u64 }, ArenaNotCreated { arena: ArenaKind }, ZeroLength }
pub enum FreeError { ForeignArena{..}, UnknownRegion, AlreadyFreed, Mismatch }
impl Bookkeeping { // Send + Sync, alle Methoden atomar
    pub fn allocate(&self, arena: ArenaKind, len: u64) -> Result<Region, AllocError>;
    pub fn free(&self, arena: ArenaKind, region: &Region) -> Result<(), FreeError>;
    pub fn is_live(&self, region: &Region) -> bool;
    pub fn live_bytes(&self, arena: ArenaKind) -> u64;
}
```
- `allocate` ist first-fit. `OutOfSpace.free` ist Kapazität minus live Bytes, auch bei Fragmentierung. Fault Injection erzeugt `OutOfSpace`.
- Parallele `free` derselben Region: genau eine gelingt.
- Es werden keine Callbacks unter dem Lock ausgeführt.
- Nach `free` ist `is_live` false.
- `live_bytes` ist die Summe der Längen aller live Regionen.

**Aus `arenas`:**
- `Arena { pub kind: ArenaKind, pub capacity: u64 }`.
- `ArenaKind` ist `Copy + Eq + Hash + Debug`.
- Die Kapazität einer angelegten Arena ändert sich nicht.

**Aus dem Device-Modul:**
- `DeviceKind { Cpu, Gpu }` ist `Copy + Eq`.
- `Event` ist `Clone + Send + Sync`.
- `Event::is_complete(&self) -> Result<bool, EventError>` blockiert nie.
- Klone teilen den Zustand.
- Test-Helfer:
  - `Event::mock() -> (Event, MockEventControl)` mit `complete()`, `fail(cause)`, `reset()`.
  - `Event::cpu() -> (Event, EventSignal)` mit `EventSignal::signal()`.

**Logging:** über die `log`-Crate (`warn!`).