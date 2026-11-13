# Spezifikation: Arenen und Speicherbudget (Build-Order 31)

## 1. Task, restrictions and boundaries

**Task.** Rust-Modul `arenas`. Es führt sechs Arenen (Speicherklassen nach Lebensdauer) mit stabiler ID und konfigurierbarer Byte-Kapazität. Es berechnet Device-Bedarf und Generationsreserve, fragt freien und gesamten Device-Speicher über eine austauschbare Abfrage ab, ermittelt den nutzbaren Anteil und führt den Fit-Check inklusive Copy-on-Write-Doppelbedarf aus.

**Nicht Teil des Moduls:**
- Keine Allokation oder Freigabe von echtem Device- oder Host-Speicher. Die Arena ist ein logischer Bereich aus Art, Kapazität und 64-Bit-Offsetraum.
- Keine Sub-Allokation innerhalb der Arenen (Allocator, Eviction, Lane-Verwaltung).
- Keine Policy, wann geladen wird oder wie viele Generationen resident sind. Beides kommt als Parameter vom Aufrufer.
- Keine Redefinition von Device, Lane oder Event.

**Abhängigkeiten.** Device, Lane und Event. Ihre Spezifikationen lagen bei der Erstellung nicht vor (pending). Dieses Modul importiert deshalb keine ihrer Typen. Die Speicherabfrage ist ein eigener Trait, den ein Device-Adapter außerhalb dieses Moduls implementiert. "Je Lane" bedeutet nur: Der Aufrufer konfiguriert für Workspace und Parameter-Block die Gesamtkapazität über alle Lanes. Dieses Modul kennt keine Lane-Typen.

## 2. Context

- Die Arenen teilen den Speicher nach Lebensdauer ein:
  - **Weights:** langlebig, nur beim Unload frei.
  - **IndexSegments:** gestreamt und evictable. Die Kapazität umfasst alle gleichzeitig residenten Generationen.
  - **Workspace:** kurzlebig, je Lane.
  - **Staging:** Pinned Host-Memory für Kopien in beide Richtungen. Bei der CPU entfällt es.
  - **ParameterBlock:** kleine feste Region je Lane für dynamische Kernel-Parameter.
  - **MappedHost:** Pinned Host-Memory, das das Device über PCIe direkt liest.
- Device-Speicher sind Weights, IndexSegments, Workspace und ParameterBlock. Staging und MappedHost liegen im Host-Memory.
- Nachbarn: Das Device-Modul liefert freien und gesamten Speicher über einen `MemoryQuery`-Adapter. Lader und Planer rufen `ensure`, `device_need` und `fit_check` auf.

## 3. Interface

```rust
use std::sync::Mutex;

pub const MIB: u64 = 1024 * 1024;
pub const DEFAULT_HEADROOM: u64 = 512 * MIB;

/// Stabile numerische IDs; Reihenfolge = ID-Reihenfolge. Erweiterbar: neue Varianten nur mit neuen IDs am Ende.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u32)]
pub enum ArenaKind {
    Weights = 0,
    IndexSegments = 1,
    Workspace = 2,
    Staging = 3,
    ParameterBlock = 4,
    MappedHost = 5,
}

impl ArenaKind {
    /// Feste Reihenfolge, vollständig: Weights, IndexSegments, Workspace, Staging, ParameterBlock, MappedHost.
    pub const ALL: [ArenaKind; 6];
    pub fn id(self) -> u32;
    /// Unbekannte ID => None ("keine Arena").
    pub fn from_id(id: u32) -> Option<ArenaKind>;
    /// true für Weights, IndexSegments, Workspace, ParameterBlock.
    pub fn is_device(self) -> bool;
}

/// Adresse = 64-Bit-Offset innerhalb einer Arena.
pub type ArenaOffset = u64;

/// Konfigurierte Kapazitäten in Byte. Default: alle 0.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ArenaCapacities { /* je Arena ein u64 */ }
impl ArenaCapacities {
    pub fn get(&self, k: ArenaKind) -> u64;
    pub fn set(&mut self, k: ArenaKind, bytes: u64);
    /// Weights + IndexSegments + Workspace + ParameterBlock, saturierend. Staging/MappedHost zählen nicht.
    pub fn device_need(&self) -> u64;
    /// index_capacity - index_capacity / max(generations,1) (Ganzzahldivision, abgerundeter Anteil). generations < 1 gilt als 1.
    pub fn generation_reserve(&self, generations: u32) -> u64;
}

/// Ergebnis der Speicherabfrage.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MemInfo { pub free: u64, pub total: u64 }

/// Austauschbare Abfrage (vom Device-Adapter oder Test-Mock implementiert).
pub trait MemoryQuery: Send + Sync {
    /// Err(reason) bei Scheitern.
    fn query(&self) -> Result<MemInfo, String>;
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BudgetError {
    /// "Speicherabfrage fehlgeschlagen": mit Begründung.
    MemoryQueryFailed { reason: String },
    /// Passt nicht: angeforderte und nutzbare Menge.
    DoesNotFit { requested: u64, usable: u64 },
}
impl std::fmt::Display for BudgetError { /* ... */ }
impl std::error::Error for BudgetError {}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Budget { pub free: u64, pub total: u64, pub generation_reserve: u64 }
impl Budget {
    /// Fragt q ab; Err => BudgetError::MemoryQueryFailed{reason}.
    pub fn from_query(q: &dyn MemoryQuery, generation_reserve: u64) -> Result<Budget, BudgetError>;
    /// free.saturating_sub(generation_reserve); nie negativ.
    pub fn usable(&self) -> u64;
}

/// Fit-Check: requested = device_need (Device-Arenen). Passt, wenn requested <= free.saturating_sub(headroom).
/// Sonst Err(DoesNotFit{requested, usable: free.saturating_sub(headroom)}).
pub fn fit_check(caps: &ArenaCapacities, free: u64, headroom: u64) -> Result<(), BudgetError>;

/// Wie fit_check, aber requested = caps.device_need().saturating_add(cow_extra).
/// cow_extra = Byte, die während Copy-on-Write kurz doppelt gehalten werden (vom Aufrufer geliefert).
pub fn fit_check_cow(caps: &ArenaCapacities, cow_extra: u64, free: u64, headroom: u64) -> Result<(), BudgetError>;

/// Angelegte Arena.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Arena { pub kind: ArenaKind, pub capacity: u64 }
impl Arena {
    pub fn contains(&self, off: ArenaOffset, len: u64) -> bool; // off+len <= capacity, überlaufsicher
}

/// Arena-Satz; thread-safe.
pub struct ArenaSet { /* Mutex<{caps, [Option<Arena>;6]}> */ }
impl ArenaSet {
    pub fn new(caps: ArenaCapacities) -> Self;
    pub fn capacities(&self) -> ArenaCapacities;
    /// Ändert die Kapazität. Wirkt nur auf noch nicht angelegte Arenen; bestehende behalten ihre Kapazität.
    pub fn set_capacity(&self, k: ArenaKind, bytes: u64);
    /// Liefert die Arena, legt sie beim ersten Aufruf lazy an (Kapazität = konfigurierte). Danach idempotent.
    /// Device-Arenen: beim Anlegen muss capacity <= budget.usable(), sonst Err(DoesNotFit{requested: capacity, usable}).
    /// Host-Arenen (Staging, MappedHost): keine Budgetprüfung.
    pub fn ensure(&self, k: ArenaKind, budget: &Budget) -> Result<Arena, BudgetError>;
    pub fn get(&self, k: ArenaKind) -> Option<Arena>; // None, wenn noch nicht angelegt
}
```

`ArenaSet` ist `Send + Sync`.

## 4. Behavior and invariants

**IDs und Liste**
- `ArenaKind::ALL` hat genau 6 Einträge in fester Reihenfolge. Die IDs sind 0..=5 in dieser Reihenfolge.
- `from_id(k.id()) == Some(k)` für jedes `k`. Für jede andere ID gilt `from_id(id) == None`, z. B. 6 oder `u32::MAX`.
- Neue Arenen werden nur durch eine neue Variante am Ende mit neuer, nie wiederverwendeter ID ergänzt.

**Bedarf und Reserve**
- `device_need` ist die saturierende Summe aus Weights, IndexSegments, Workspace und ParameterBlock. Es gibt keinen Überlauf und kein Wrapping. Staging und MappedHost ändern das Ergebnis nicht.
- `generation_reserve(g)` rechnet mit `g' = max(g, 1)` und liefert `idx - idx / g'`.
  - `g ∈ {0, 1}` ergibt 0.
  - Beispiel: idx=900, g=3 ergibt 600.
  - Das Ergebnis ist nie größer als `idx`.

**Budget**
- `usable() = free.saturating_sub(generation_reserve)`. Wenn `reserve > free`, ist das Ergebnis 0.
- Ein fehlgeschlagener `query` ergibt `MemoryQueryFailed { reason }`. Die Begründung des Adapters wird unverändert übernommen. Es gibt kein Retry und keinen Fallback.

**Fit-Check**
- Die Prüfung ist `requested <= free.saturating_sub(headroom)`. Gleichheit passt.
- Ist `headroom > free`, ist usable = 0. Nur `requested == 0` passt dann.
- Der Fehler enthält `requested` (inklusive `cow_extra` bei `fit_check_cow`) und das berechnete `usable`.
- Die Generationsreserve wird im Fit-Check nicht zusätzlich abgezogen. Die Index-Kapazität enthält bereits alle residenten Generationen.
- `DEFAULT_HEADROOM` ist 512 MiB. Der Aufrufer übergibt den Headroom explizit.
- `fit_check_cow` plant den kurzen Doppelbedarf durch `saturating_add` ein.

**Lazy Anlage und Konkurrenz**
- Vor dem ersten `ensure` ist `get` gleich `None`. `ensure` einer Arena legt keine anderen Arenen an.
- Ein zweites `ensure` gibt dieselbe Arena zurück. Die Kapazität bleibt unverändert, auch wenn `set_capacity` dazwischen aufgerufen wurde, und das Budget wird nicht erneut geprüft.
- Bei Fehler in `ensure` wird nichts angelegt, und ein späterer Versuch ist möglich.
- Alle Methoden von `ArenaSet` sind über einen internen Mutex atomar. Parallele `ensure`-Aufrufe derselben Arena legen sie genau einmal an. Es gibt keine Deadlock-Gefahr, weil keine Callbacks unter dem Lock laufen.
- Adressen sind `u64`-Offsets in `[0, capacity)`. `contains` prüft ohne Überlauf, und `off + len` wird saturierend oder per `checked_add` gerechnet.
- Eine Kapazität von 0 ist erlaubt. Die Arena ist dann leer, und `contains(0, 0)` ist `true`.

## 5. Relied-on elements

Aus Device, Lane und Event wird nichts verbatim übernommen, weil deren Spezifikationen zum Zeitpunkt der Erstellung nicht vorlagen. Der Device-Adapter implementiert `MemoryQuery` und liefert freien und gesamten Device-Speicher in Byte (`u64`). Der Aufrufer bildet "je Lane" auf die konfigurierte Gesamtkapazität ab.