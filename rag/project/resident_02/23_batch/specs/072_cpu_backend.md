# Spezifikation: CPU-Backend (Build-Order 72)

## 1. Task, restrictions and boundaries

**Task.** Rust-Modul `cpu_backend`. Es führt jede Arena als genullten, zusammenhängenden RAM-Bereich und prüft Schreiben, Lesen und Kopieren auf Regionsbasis. Es liefert das Budget aus den Summen der Arenen und erfüllt die gemeinsame Backend-Schnittstelle `MemoryBackend`. Außerdem stellt es Regionen über die Speicherzugriff-Schnittstelle (`memory_access::RegionBackend`) bereit.

**Nicht Teil des Moduls:**
- Keine Streams, kein Staging und keine Lanes. `Staging` wird nicht bereitgestellt.
- Keine eigene Platzierung, Generationsverwaltung oder Liveness-Buchführung. Das macht `Bookkeeping`. Keine eigene Deferred-Free-Logik. Das macht `Allocator`.
- Keine Eviction, keine Policy, keine Device-Adressen nach außen.
- `ArenaKind`, `Arena`, `ArenaSet`, `Budget`, `Region`, `Allocator`, `Event` und `RegionHandle` werden nicht neu definiert.

## 2. Context

- `arenas` liefert Arenen, Kapazitäten und `Budget`. `bookkeeping` vergibt `Region` (first-fit, Alignment 1) und prüft Liveness. `allocator` setzt Deferred Free darauf (`Allocator`/`AllocatorSet`). `memory_access` definiert `RegionBackend` und `Region` (Zugriffs-Handle). `device` liefert `Event` und `DeviceKind::Cpu`.
- Zwei Typen heißen `Region`. In dieser Spezifikation gilt:
  - `BRegion` = `bookkeeping::Region` ist das Handle aller Backend-Methoden.
  - `AccessRegion` = `memory_access::Region` ist das Zugriffs-Handle für Encoder.
- Die gemeinsame Backend-Schnittstelle lag zum Zeitpunkt der Erstellung nicht als Spezifikation vor. Das Trait `MemoryBackend` unten ist ihre Fassung für dieses Modul. Andere Backends implementieren es gleich.
- Adressen im RAM verlassen das Backend nie. Alle Schnittstellen arbeiten mit `BRegion`, Offsets und Byte-Slices.

## 3. Interface

```rust
use std::sync::Arc;
use arenas::{ArenaCapacities, ArenaKind, Budget, MemInfo, MemoryQuery};
use bookkeeping::Region as BRegion;
use device::Event;
use memory_access::Region as AccessRegion;
use region_handle::RegionHandle;

/// Granularität: jede Allokationslänge wird auf ein Vielfaches davon aufgerundet.
pub const ALLOC_GRANULE: u64 = 64;
/// Maximal unterstütztes Alignment (Zweierpotenz).
pub const MAX_ALIGN: u64 = 64;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BackendError {
    /// Erschöpfung. requested = aufgerundete Länge; free = Allocator::available().
    OutOfMemory { arena: ArenaKind, requested: u64, free: u64 },
    /// Display: "Allokation fehlgeschlagen"
    AllocationFailed,
    /// Display: "stale Region"
    StaleRegion,
}
impl std::fmt::Display for BackendError {}
impl std::error::Error for BackendError {}
impl From<allocator::AllocatorError> for BackendError {} // 1:1 Abbildung der drei Varianten

/// Gemeinsame nebenläufige Backend-Schnittstelle.
pub trait MemoryBackend: Send + Sync {
    fn allocate(&self, arena: ArenaKind, size: u64, align: u64) -> Result<BRegion, BackendError>;
    fn free(&self, region: &BRegion) -> Result<(), BackendError>;
    fn free_deferred(&self, region: &BRegion, event: Event) -> Result<(), BackendError>;
    fn budget(&self) -> Budget;
    fn write(&self, region: &BRegion, offset: u64, src: &[u8]) -> Result<(), BackendError>;
    fn read(&self, region: &BRegion, offset: u64, dst: &mut [u8]) -> Result<(), BackendError>;
    /// Kopiert src.len Bytes von Beginn src nach Beginn dst.
    fn copy(&self, src: &BRegion, dst: &BRegion) -> Result<(), BackendError>;
}

/// Send + Sync.
pub struct CpuBackend { /* ArenaSet, Arc<Bookkeeping>, AllocatorSet, Puffer je Arena, generations */ }

impl CpuBackend {
    /// Legt für jede ArenaKind außer Staging mit Kapazität > 0 einen genullten
    /// RAM-Bereich an (Fehlschlag der Speicherbeschaffung => Err(OutOfMemory{requested: Kapazität, free: 0})).
    /// Arenen mit Kapazität 0 werden angelegt (leer). `generations` fließt in die Generationsreserve ein.
    pub fn new(caps: ArenaCapacities, generations: u32) -> Result<CpuBackend, BackendError>;

    /// Zugriff auf eine Region über die einheitliche Speicherzugriff-Schnittstelle.
    /// Err(StaleRegion), wenn die Region jetzt nicht live ist.
    pub fn access(&self, region: &BRegion) -> Result<AccessRegion, BackendError>;

    /// Konvertierung nach RegionHandle (Generation(region.generation as u64)); StaleRegion, wenn nicht live.
    pub fn handle_of(&self, region: &BRegion) -> Result<RegionHandle, BackendError>;
    /// Umkehrung; StaleRegion, wenn Generation > u32::MAX oder die Region nicht live ist.
    pub fn region_of(&self, handle: &RegionHandle) -> Result<BRegion, BackendError>;

    pub fn reap(&self) -> usize; // delegiert an alle Allocatoren, Summe der Rückgaben
}

impl MemoryBackend for CpuBackend {}
impl MemoryQuery for CpuBackend {} // query() => Ok(MemInfo{ free, total }) wie budget(), nie Err
```

## 4. Behavior and invariants

**Arenen**
- Jede Arena außer `Staging` ist ein einzelner, bei `new` mit Nullen gefüllter RAM-Bereich der konfigurierten Kapazität. Er wird nicht gestreamt und nicht gestaged. Der Bereich ändert Größe und Ort nicht.
- Die Arenen werden über `ArenaSet::ensure` angelegt. Dabei wird ein Budget ohne Einschränkung übergeben (`free = total = u64::MAX`, Reserve 0). Eine RAM-Prüfung findet nur über die Speicherbeschaffung statt (`try_reserve`, kein Abort).
- Jede Arena erhält einen `Allocator` (`DeviceKind::Cpu`) in einem gemeinsamen `AllocatorSet` mit einem gemeinsamen `Bookkeeping`.
- Wiederverwendete Bereiche nach Freigabe werden nicht neu genullt.

**Allokation**
- Gültig sind `size > 0`, `align` als Zweierpotenz mit `1 ≤ align ≤ MAX_ALIGN`, und die Arena existiert (nicht `Staging`, nicht unbekannt). Sonst gilt `AllocationFailed`.
- Die Länge wird auf ein Vielfaches von `ALLOC_GRANULE` aufgerundet. Überlauf dabei ergibt `AllocationFailed`. Die Region hat `len` = aufgerundete Länge.
- Da alle Längen Vielfache von 64 sind und first-fit bei 0 beginnt, ist jeder Offset ein Vielfaches von 64 und damit für jedes `align ≤ 64` ausgerichtet. Die Puffer sind ebenfalls 64-Byte-ausgerichtet.
- Die Fehlerabbildung kommt vom `Allocator`. Erschöpfung und Fault Injection ergeben `OutOfMemory`, alles andere `AllocationFailed`.
- Die Allokation geht über `Allocator::allocate`, das zuerst abgeschlossene Deferred Frees einsammelt.

**Freigabe**
- `free` und `free_deferred` delegieren an den `Allocator` der Region (`AllocatorSet::allocator_for`). Fremde, nicht live, bereits vorgemerkte oder doppelt freigegebene Regionen ergeben `StaleRegion`.
- Die Region bleibt nach `free_deferred` bis zum Abschluss des Events live. Sie wird erst nach `reap` frei, das auch `allocate` aufruft.

**Lesen, Schreiben, Kopieren**
- `read`, `write` und `copy` prüfen vor jedem Zugriff:
  1. `Bookkeeping::is_live` für jede beteiligte Region,
  2. `offset.checked_add(len) ≤ region.len`, überlaufsicher,
  3. dass der Bereich in der Arena liegt.
- Eine Verletzung ergibt `StaleRegion`. Es wird nichts gelesen, geschrieben oder kopiert und kein Zustand geändert.
- Ein Zugriff mit Länge 0 ist bei live Region und `offset ≤ region.len` ein erfolgreicher No-op.
- `copy` kopiert `src.len` Bytes. Ist `dst.len < src.len`, ergibt es `StaleRegion`. Bytes hinter `src.len` in `dst` bleiben unverändert.
- Innerhalb einer Arena ist `copy` überlappungssicher (memmove-Semantik, auch bei überlappenden Regionen). Zwischen Arenen läuft `copy` über einen Zwischenpuffer: erst Quelle in den Puffer lesen, dann Puffer ins Ziel schreiben. Es wird nie mehr als ein Arena-Lock gleichzeitig gehalten.
- Die Liveness-Prüfung und der Zugriff sind nicht atomar gegenüber einem parallelen `free`. Ein Zugriff, der mit einem `free` konkurriert, ist speichersicher und liest oder schreibt nur im Arena-Puffer.
- Disjunkte Bereiche beeinflussen sich nicht. Bei überlappenden konkurrierenden Zugriffen ist das Ergebnis eine beliebige Verschränkung der Bytes, aber ohne Undefined Behavior (also keine Data Races im Rust-Sinn).
- Die Auflösung von Regionen auf Adressen oder Pufferbereiche ist intern (nicht `pub`). Es gibt keine öffentliche Funktion, die Pointer, Adressen oder Referenzen auf Arena-Speicher liefert.

**Budget**
- `budget()` ist `Budget { free, total, generation_reserve }` mit:
  - `free` = Σ `Allocator::available()` über alle Arenen im Set,
  - `total` = Σ Kapazitäten dieser Arenen, saturierend,
  - `generation_reserve` = `caps.generation_reserve(generations)` mit den Kapazitäten aus `new`.
- `budget()` schlägt nie fehl. Es gilt `free ≤ total`.
- Wartende (deferred) Regionen zählen als belegt und nicht als frei. `budget()` sammelt nicht ein.
- `MemoryQuery::query` liefert `MemInfo { free, total }` aus denselben Summen.

**Speicherzugriff-Schnittstelle (`access`)**
- Der gelieferte `AccessRegion` hat `len() == region.len` und adressiert die Bytes `[region.offset, region.offset + region.len)` der Arena.
- Das interne `RegionBackend` ist `Send + Sync + Debug`. `read_raw` und `write_raw` prüfen bei jedem Zugriff erneut die Liveness (`BRegion` wird im Backend gehalten). Ist sie nicht mehr live, kommt `AccessError::Backend("stale Region")`.
- Die Bereichsprüfung gegen `len` macht `memory_access`. Das Backend verlässt sich darauf.
- Klone des `AccessRegion` teilen den Zustand. Ein Schreiben über `access` ist für `read` der Backend-Methoden sichtbar und umgekehrt (Read-your-writes).
- Passt `region.len` nicht in `usize`, ergibt `access` `StaleRegion`.

**Handles**
- `handle_of` baut mit `RegionHandle::new_in` gegen die Arena. Fehler daraus ergeben `StaleRegion`.

**Concurrency**
- Alle Methoden nehmen `&self` und sind aus mehreren Threads nutzbar. `CpuBackend` ist `Send + Sync`.
- Es gibt keine Deadlocks. Die Lock-Reihenfolge ist Allocator, dann Bookkeeping. Arena-Locks werden nicht verschachtelt. Es laufen keine Callbacks (außer `Event::is_complete` im `Allocator`) unter Arena-Locks.
- Vergiftete Locks werden nicht propagiert. Der innere Zustand wird übernommen, es gibt keinen Panic.

## 5. Relied-on elements

**`arenas`**
- `ArenaKind { Weights, IndexSegments, Workspace, Staging, ParameterBlock, MappedHost }`, `#[non_exhaustive]`, `Copy + Eq + Hash`.
- `ArenaCapacities { get, set, device_need, generation_reserve(g) = idx - idx / max(g,1) }`, `Default`.
- `ArenaSet::new(caps)`, `ensure(kind, &Budget) -> Result<Arena, BudgetError>` (idempotent, Kapazität bleibt fest), `get`.
- `Arena { kind, capacity }`.
- `Budget { free, total, generation_reserve }`.
- `MemInfo { free, total }`.
- `trait MemoryQuery: Send + Sync { fn query(&self) -> Result<MemInfo, String>; }`.

**`bookkeeping`**
- `Region { arena, offset, len, generation: u32 }`: `Copy`, `Eq`, `Hash`.
- `Bookkeeping::new(Arc<ArenaSet>)`.
- `is_live(&Region) -> bool`.
- `live_bytes(arena)`.
- Die Vergabe ist first-fit, Alignment 1, `len == 0` ist ein Fehler.

**`allocator`**
- `Allocator::new(Arena, DeviceKind, Arc<Bookkeeping>)`, `allocate(len)`, `free(&Region)`, `free_deferred(&Region, Event)`, `reap() -> usize`, `available() -> u64`, `is_live`.
- `AllocatorSet::new(Arc<Bookkeeping>, &[(Arena, DeviceKind)])`, `allocator(kind) -> Option<&Allocator>`, `allocator_for(&Region) -> Result<&Allocator, AllocatorError>`, `size_hint`.
- `AllocatorError { OutOfMemory{arena, requested, free}, AllocationFailed, StaleRegion }`.
- `available = capacity - live_bytes`.

**`region_handle`**
- `Generation(pub u64)`.
- `RegionHandle::new_in(&Arena, offset, len, Generation) -> Result<RegionHandle, RegionError>`.
- Zugriffe `arena()`, `offset()`, `len()`, `generation()`.

**`memory_access`**
- `trait RegionBackend: Send + Sync + Debug { fn len(&self) -> usize; fn read_raw(&self, offset: usize, dst: &mut [u8]) -> Result<(), AccessError>; fn write_raw(&self, offset: usize, src: &[u8]) -> Result<(), AccessError>; }`.
- `Region::new(Arc<dyn RegionBackend>)`.
- `AccessError { OutOfBounds{..}, Backend(String) }`.

**`device`**
- `DeviceKind::Cpu`.
- `Event`: `Clone + Send + Sync`, `is_complete()` blockiert nie.