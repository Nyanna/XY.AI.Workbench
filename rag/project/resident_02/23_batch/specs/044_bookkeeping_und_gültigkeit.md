# Spezifikation: Bookkeeping und Gültigkeit (Build-Order 44)

## 1. Task, restrictions and boundaries

**Task.** Rust-Modul `bookkeeping`. Es führt je Arena thread-safe Buch über vergebene Regionen (Offset, Länge, Generation) und prüft deren Liveness. Es vergibt und gibt Regionen frei. Es ignoriert stale Handles mit Warnung, löst künstliche Allocation-Fehler aus (Fault Injection) und loggt Allokationen und Freigaben.

**Nicht Teil des Moduls:**
- Es besitzt, allokiert und dereferenziert keinen Device- oder Host-Speicher. Es rechnet nur mit logischen `u64`-Offsets.
- Keine Eviction, keine Lane-Verwaltung und keine Policy, wann allokiert wird.
- Keine Neudefinition von `ArenaKind`, `Arena`, `ArenaSet`, `ArenaOffset` oder `BudgetError`. Sie kommen aus dem Modul `arenas`.
- Es legt keine Arenen an. Die Arena muss vorher per `ArenaSet::ensure` existieren.

## 2. Context

- Das Modul `arenas` (Build-Order 31) liefert `ArenaKind`, `ArenaSet`, `Arena` (Kapazität) und `ArenaOffset = u64`.
- Dieses Modul ist die Buchführungsschicht darüber. Lader und Planer fordern Regionen an, halten `Region`-Handles und geben sie wieder frei.
- Spätere Verbraucher prüfen vor jedem Zugriff `is_live`. Das allgemeine Verwaltungs-Interface (`release_or_ignore`, `resolve`) dereferenziert stale Handles nie.
- Logging läuft über die `log`-Crate (`debug!`, `warn!`).

## 3. Interface

```rust
use std::sync::Arc;
use arenas::{ArenaKind, ArenaOffset, ArenaSet};

/// Generation einer (Arena, Offset, Länge)-Kombination; Wrap-around bei u32::MAX -> 0.
pub type Generation = u32;

/// Handle auf eine vergebene Region. Reines Wertobjekt, besitzt keinen Speicher.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Region {
    pub arena: ArenaKind,
    pub offset: ArenaOffset,
    pub len: u64,
    pub generation: Generation,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AllocError {
    /// Platzmangel oder Fault Injection. `free` = Summe der freien Bytes der Arena (Kapazität minus live Bytes).
    OutOfSpace { arena: ArenaKind, requested: u64, free: u64 },
    /// Arena wurde noch nicht per ArenaSet::ensure angelegt.
    ArenaNotCreated { arena: ArenaKind },
    /// len == 0.
    ZeroLength,
}
impl std::fmt::Display for AllocError { /* ... */ }
impl std::error::Error for AllocError {}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FreeError {
    ForeignArena { expected: ArenaKind, found: ArenaKind },
    UnknownRegion,
    AlreadyFreed,
    Mismatch,
}
impl std::fmt::Display for FreeError { /* ... */ }
impl std::error::Error for FreeError {}

/// Ergebnis der ignorierenden Freigabe.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReleaseOutcome { Released, IgnoredStale }

/// Thread-safe; Send + Sync.
pub struct Bookkeeping { /* intern: Mutex über Zustand je Arena */ }

impl Bookkeeping {
    pub fn new(arenas: Arc<ArenaSet>) -> Self;

    /// Vergibt eine Region der Länge `len` in `arena`.
    pub fn allocate(&self, arena: ArenaKind, len: u64) -> Result<Region, AllocError>;

    /// Strikte Freigabe. `arena` ist die Arena, in der der Aufrufer freigibt.
    pub fn free(&self, arena: ArenaKind, region: &Region) -> Result<(), FreeError>;

    /// Allgemeines Verwaltungs-Interface: Freigabe, die stale Handles ignoriert (warn!) statt Fehler zu liefern.
    pub fn release_or_ignore(&self, region: &Region) -> ReleaseOutcome;

    /// true, wenn Arena, Offset, Länge und Generation einer aktuellen, nicht freigegebenen Vergabe entsprechen.
    pub fn is_live(&self, region: &Region) -> bool;

    /// Some((offset, len)) nur wenn live; sonst None (mit warn!). Dereferenziert nie.
    pub fn resolve(&self, region: &Region) -> Option<(ArenaOffset, u64)>;

    /// Fault Injection: Die nächsten `count` Aufrufe von `allocate` auf `arena` scheitern mit OutOfSpace.
    pub fn inject_alloc_failure(&self, arena: ArenaKind, count: u32);
    /// Entfernt offene Fault Injections dieser Arena.
    pub fn clear_alloc_failure(&self, arena: ArenaKind);

    /// Summe der live Bytes der Arena.
    pub fn live_bytes(&self, arena: ArenaKind) -> u64;
}
```

## 4. Behavior and invariants

**Vergabe**
- `allocate` verwendet die Kapazität aus `ArenaSet::get(arena)`. Ist sie `None`, kommt `ArenaNotCreated`. `len == 0` ergibt `ZeroLength`.
- Platzierung ist deterministisch first-fit: der niedrigste Offset, an dem `len` Bytes zusammenhängend frei sind. Die Region liegt vollständig in `[0, capacity)`, überlaufsicher gerechnet, und überlappt keine live Region derselben Arena. Alignment gibt es nicht (Alignment 1).
- Findet sich kein zusammenhängender Block, kommt `OutOfSpace { arena, requested: len, free }`. `free` ist die Summe aller freien Bytes, auch bei Fragmentierung. Nichts wird verändert.
- Generation: Wird eine Kombination (Arena, Offset, Länge) zum ersten Mal vergeben, ist die Generation 0. Jede erneute Vergabe derselben Kombination, also nach Freigabe, erhöht die zuletzt vergebene Generation um 1 mit `wrapping_add`. Das Gedächtnis über vergebene Kombinationen bleibt auch nach der Freigabe bestehen.
- Generationen sind je Kombination unabhängig. Dieselbe Kombination in einer anderen Arena zählt separat.

**Liveness**
- `is_live(r)` ist genau dann true, wenn eine nicht freigegebene Vergabe mit identischer Arena, Offset, Länge und Generation existiert. Nach `free` ist es false. Nach Neuvergabe derselben Kombination ist nur das Handle mit der neuen Generation live. Das alte bleibt stale, außer die Generation ist per Wrap-around wieder gleich.
- `is_live` hat keine Nebenwirkung und loggt nichts.

**Freigabe (strikt)**
Prüfreihenfolge:
1. `region.arena != arena` ergibt `ForeignArena { expected: arena, found: region.arena }`.
2. Existiert eine live Vergabe mit gleichem Offset und gleicher Länge:
   - Bei gleicher Generation wird freigegeben, `Ok`.
   - Sonst `Mismatch`.
3. Wurde die Kombination schon einmal vergeben und ist nicht live:
   - Bei Generation == zuletzt vergebener Generation: `AlreadyFreed`.
   - Sonst: `Mismatch`.
4. Beginnt eine live Vergabe am gleichen Offset mit anderer Länge: `Mismatch`.
5. Sonst: `UnknownRegion`.

Bei jedem Fehler bleibt der Zustand unverändert.

**Ignorierende Verwaltung**
- `release_or_ignore` gibt eine live Region frei (`Released`). Andernfalls kommt `IgnoredStale`, der Zustand bleibt unverändert, und es gibt ein `warn!` mit Arena, Offset, Länge und Generation. Es gibt weder Panic noch Fehler.
- `resolve` liefert bei stale Handles `None` und loggt `warn!`.

**Fault Injection**
- Solange für eine Arena ein Zähler > 0 gesetzt ist, scheitert `allocate` auf dieser Arena mit `OutOfSpace { arena, requested: len, free }`. `free` ist der tatsächliche freie Wert. Der Fehler ist nicht von echter Erschöpfung zu unterscheiden. Jeder so ausgelöste Fehler verringert den Zähler um 1.
- Die Injection hat Vorrang vor `ArenaNotCreated` und `ZeroLength` nicht. Diese beiden werden zuerst geprüft und verbrauchen keinen Zähler.
- Eine fehlgeschlagene Injection ändert weder Offsets noch Generationen. Injection in einer Arena wirkt nicht auf andere Arenen.

**Logging**
- Erfolgreiche `allocate`: `debug!` mit Arena, Offset und Größe (len). Das gilt auch für Freigaben, die `Ok` bzw. `Released` liefern. Fehler loggen nicht auf Debug-Level. Ignorierte stale Handles loggen `warn!`.

**Concurrency**
- Alle Methoden sind über einen internen Mutex atomar. Parallele `allocate`-Aufrufe liefern nie überlappende live Regionen. Parallele `free` derselben Region: genau ein Aufruf gelingt, die anderen liefern `AlreadyFreed`. Es laufen keine Callbacks unter dem Lock. `Bookkeeping` ist `Send + Sync`.
- Der Besitz von Device-Speicher liegt nie hier. `live_bytes` ist `Σ len` aller live Regionen der Arena und überschreitet die Kapazität nie.

## 5. Relied-on elements

Aus `arenas` (Build-Order 31), unverändert zu verwenden:
- `#[repr(u32)] enum ArenaKind { Weights=0, IndexSegments=1, Workspace=2, Staging=3, ParameterBlock=4, MappedHost=5 }` (`#[non_exhaustive]`, `Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord`). `ArenaKind::ALL: [ArenaKind; 6]`.
- `pub type ArenaOffset = u64;`
- `ArenaSet` (`Send + Sync`) mit `pub fn get(&self, k: ArenaKind) -> Option<Arena>`. Es liefert `None`, wenn die Arena noch nicht angelegt ist.
- `Arena { pub kind: ArenaKind, pub capacity: u64 }` mit `contains(off, len)`.
- Die Kapazität einer angelegten Arena ändert sich nicht mehr, auch nicht durch `set_capacity`. Eine Kapazität von 0 ist erlaubt, dann scheitert jede Vergabe mit `OutOfSpace { free: 0 }`.