# Spezifikation: Region-Handle (Build-Order 45)

## 1. Task, restrictions and boundaries

**Task.** Rust-Modul `region_handle`. Es stellt `RegionHandle` bereit: eine pointerfreie Region aus Arena, Offset, Länge und Generation. Das Handle ist kopierbar, vergleichbar und als Key verwendbar. Die Generation entwertet Handles nach Eviction und Neuladen.

**Nicht Teil des Moduls:**
- Keine Auflösung auf Device-Adressen. Das macht ausschließlich das Memory-Backend, und Device-Adressen verlassen das Backend nicht. Dieses Modul enthält weder Adresstypen noch Raw-Pointer, Referenzen oder `unsafe`.
- Keine Allokation, Eviction oder Neuladen. Kein Generationszähler und keine Tabelle gültiger Handles. Die aktuelle Generation einer Region liefert der Aufrufer, also das Backend oder der Allocator.
- Keine Redefinition von `ArenaKind`, `ArenaOffset`, `Arena` oder `BudgetError`.

## 2. Context

- Das Modul baut auf "Arenen und Speicherbudget" auf. Von dort kommen `ArenaKind` (stabile `u32`-ID), `ArenaOffset = u64` und `Arena { kind, capacity }` mit `contains(off, len)`.
- Das Memory-Backend vergibt Handles und löst sie auf. Es vergleicht dazu die Handle-Generation mit der aktuellen Generation und entscheidet über die Gültigkeit.
- Lader, Planer und Kernels reichen Handles nur als Werte weiter. Sie nutzen sie als Key, zum Beispiel in `HashMap`.

## 3. Interface

```rust
use arenas::{Arena, ArenaKind, ArenaOffset};

/// Generationszähler einer Region. Monoton steigend; Vergabe liegt beim Aufrufer/Backend.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Generation(pub u64);

impl Generation {
    pub const INITIAL: Generation = Generation(0);
    /// Nachfolger; None bei Überlauf (kein Wrapping).
    pub fn next(self) -> Option<Generation>;
}

/// Pointerfreies Region-Handle. Reiner Wert, kein Drop, keine Heap-Daten.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RegionHandle {
    arena: ArenaKind,
    offset: ArenaOffset,
    len: u64,
    generation: Generation,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RegionError {
    /// offset + len überläuft u64.
    Overflow,
    /// Region liegt nicht vollständig in der Arena (oder Arena-Art passt nicht).
    OutOfBounds { offset: u64, len: u64, capacity: u64 },
    /// Arena-Art des Handles stimmt nicht mit der übergebenen Arena überein.
    ArenaMismatch { handle: ArenaKind, arena: ArenaKind },
}
impl std::fmt::Display for RegionError { /* ... */ }
impl std::error::Error for RegionError {}

impl RegionHandle {
    /// Ohne Arena-Bezug; prüft nur offset.checked_add(len).
    pub fn new(arena: ArenaKind, offset: ArenaOffset, len: u64, generation: Generation)
        -> Result<RegionHandle, RegionError>;
    /// Wie new, prüft zusätzlich gegen die Arena (arena.kind == Handle-Arena, arena.contains(offset, len)).
    pub fn new_in(arena: &Arena, offset: ArenaOffset, len: u64, generation: Generation)
        -> Result<RegionHandle, RegionError>;

    pub fn arena(&self) -> ArenaKind;
    pub fn offset(&self) -> ArenaOffset;
    pub fn len(&self) -> u64;
    pub fn is_empty(&self) -> bool;       // len == 0
    pub fn generation(&self) -> Generation;
    /// Exklusives Ende: offset + len (nie Überlauf, siehe new).
    pub fn end(&self) -> u64;

    /// true, wenn generation == current.
    pub fn is_current(&self, current: Generation) -> bool;
    /// Gleiche Region (Arena, Offset, Länge) mit neuer Generation.
    pub fn with_generation(&self, generation: Generation) -> RegionHandle;
    /// Prüft die Region gegen eine Arena (Art und Grenzen); Generation bleibt unberücksichtigt.
    pub fn check_in(&self, arena: &Arena) -> Result<(), RegionError>;
}
```

`RegionHandle` und `Generation` sind `Send + Sync + 'static`. Das folgt aus ihrem Aufbau und wird per statischem Test geprüft. Ein Handle belegt höchstens 32 Byte.

## 4. Behavior and invariants

**Wert und Key**
- `Copy`, `Eq`, `Hash` und `Ord` arbeiten über alle vier Felder: Arena, Offset, Länge und Generation. Zwei Handles sind genau dann gleich, wenn alle vier Felder gleich sind.
- Handles mit gleicher Region und unterschiedlicher Generation sind ungleich und haben unterschiedliche Keys.
- `Hash` ist konsistent mit `Eq`. Die Ordnung ist lexikografisch in der Feldreihenfolge Arena, Offset, Länge, Generation. `ArenaKind` ordnet nach ID.

**Konstruktion**
- `new` schlägt mit `Overflow` fehl, wenn `offset.checked_add(len)` `None` liefert. Sonst gelingt es. Eine Region mit `len == 0` ist erlaubt, und `end() == offset`.
- `new_in` prüft in dieser Reihenfolge:
  1. Überlauf: `Overflow`.
  2. Grenzen: `!arena.contains(offset, len)` ergibt `OutOfBounds { offset, len, capacity: arena.capacity }`.
  3. Das Handle erhält `arena.kind`.
- `check_in` prüft in dieser Reihenfolge:
  1. `ArenaMismatch`, wenn `arena.kind != self.arena`.
  2. `OutOfBounds`, wenn `!arena.contains(offset, len)`.
- Ein Handle aus `new` kann also die Arena-Grenzen verletzen, weil `new` keine Arena kennt. Das Backend prüft dann bei der Auflösung selbst.

**Generation**
- `is_current(c)` ist `generation == c`. Es gibt keine Ordnungssemantik: Nur exakte Gleichheit gilt als gültig. Ein Handle mit älterer oder neuerer Generation als `c` ist ungültig.
- Nach Eviction und Neuladen vergibt der Aufrufer eine neue Generation. Alte Handles sind dann nach `is_current` ungültig. Das Modul hält dafür keinen Zustand.
- `with_generation` ändert nur die Generation. Das Original bleibt unverändert, weil das Handle ein `Copy`-Wert ist.
- `Generation::next` liefert `Some(Generation(n+1))`. Bei `u64::MAX` liefert es `None`.
- `Generation::default()` ist `INITIAL` (0).

**Nebenläufigkeit und Fehler**
- Es gibt keinen internen Zustand, keine Locks und keine globalen Variablen. Alle Funktionen sind rein und thread-safe.
- Fehler sind ausschließlich `RegionError`. Es gibt keine Panics. Überlauf wird immer über `checked_add` behandelt.
- Es gibt keine Funktion, die eine Device-Adresse, einen Pointer oder eine Referenz auf Speicher liefert oder entgegennimmt.

## 5. Relied-on elements

Aus "Arenen und Speicherbudget" (Modul `arenas`), unverändert zu verwenden:

```rust
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u32)]
pub enum ArenaKind { Weights = 0, IndexSegments = 1, Workspace = 2, Staging = 3, ParameterBlock = 4, MappedHost = 5 }
impl ArenaKind { pub fn id(self) -> u32; pub fn from_id(id: u32) -> Option<ArenaKind>; }

pub type ArenaOffset = u64; // 64-Bit-Offset innerhalb einer Arena

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Arena { pub kind: ArenaKind, pub capacity: u64 }
impl Arena {
    pub fn contains(&self, off: ArenaOffset, len: u64) -> bool; // off+len <= capacity, überlaufsicher
}
```

- Adressen sind `u64`-Offsets in `[0, capacity)`. Eine Kapazität von 0 ist erlaubt, und dann ist `contains(0, 0)` gleich `true`.
- Ein Handle speichert `ArenaKind` und keine rohe ID. Wer eine ID von außen annimmt, bildet sie mit `ArenaKind::from_id` ab. `None` bedeutet "keine Arena", und das Handle wird nicht erzeugt.