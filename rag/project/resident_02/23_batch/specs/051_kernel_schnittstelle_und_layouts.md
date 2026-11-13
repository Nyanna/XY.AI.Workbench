# Spezifikation: Kernel-Schnittstelle und Layouts (Build-Order 51)

Crate-Modul `kernel_layout`.

## 1. Aufgabe, Einschränkungen, Grenzen

**Aufgabe:**
- Das Modul definiert, was ein Such-Kernel liefert: nur Adresse und Score (`KernelHit`).
- Es stellt feste und variable Layouts bereit, die Entries im Adressraum eines Segments oder einer Kandidatenmenge abbilden.
- Es bildet Adressen auf Slots und `EntryId`s zurück.
- Es baut aus n Längen per exklusiver Prefix-Sum eine Offset-Tabelle mit n+1 Offsets.
- Es wählt per Regel zwischen festem und variablem Layout.

**Grenzen:**
- Das Modul enthält keinen Kernel, keine Score-Berechnung, keine Top-k-Auswahl und kein Speichermanagement.
- Es liest keine Vektordaten und kennt weder Index-Implementierungen noch die Arena.
- Die Index-Struktur und die Längen liefert der Aufrufer (Manager). Das Modul baut die Tabelle aus den übergebenen Längen oder Extents.
- Nur `std`. Abhängig vom Index-Vertrag nur für `EntryId`.

## 2. Kontext

- Der Manager baut aus der Index-Struktur Längen oder Extents und erzeugt damit ein `Layout`.
- Der Kernel läuft über den adressierten Speicher und meldet je Treffer `KernelHit { addr, score }`.
- Der Aufrufer bildet über eine `SlotMap` jede Adresse auf Slot und `EntryId` zurück.
- Eine Kandidatenmenge ist immer ein variables Layout. Das gilt, egal ob die Tabelle auf kompakte Kopien (zusammenhängende Längen) oder auf bestehende Segmente (Extents in der Arena) zeigt.
- Die Adressdarstellung ist Implementierungsdetail. Aufrufer behandeln `Addr` als opake Zahl und nutzen nur die Funktionen dieses Moduls.

## 3. Interface

```rust
use index_contract::EntryId;
use std::ops::Range;

/// Opake Adresse (Byte-Offset im Adressraum des Layouts, inkl. Basisadresse).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Addr(pub u64);

/// Position eines Entries im Layout (0-basiert, Reihenfolge der Eingabe).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Slot(pub u32);

/// Einziges Kernel-Ergebnis pro Treffer.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct KernelHit { pub addr: Addr, pub score: f32 }

/// Bestehendes Segment: Startadresse und Länge in Bytes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Extent { pub start: u64, pub len: u64 }

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LayoutError {
    Overflow,                                   // u64-/u32-Überlauf beim Bauen
    TooManyEntries,                             // mehr als u32::MAX Slots
    Overlap,                                    // Extents überlappen
    CountMismatch { expected: usize, got: usize }, // ids.len() != layout.len()
    ZeroStride,                                 // Stride == 0 bei Fixed
}
impl std::fmt::Display for LayoutError { /* sprechender Text */ }
impl std::error::Error for LayoutError {}

/// Festes Layout: Slot i belegt [base + i*stride, base + (i+1)*stride).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FixedLayout { /* base: u64, stride: u32, count: u32 */ }
impl FixedLayout {
    pub fn new(base: u64, stride: u32, count: u32) -> Result<Self, LayoutError>;
    pub fn stride(&self) -> u32;
}

/// Variables Layout (Offset-Tabelle).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VariableLayout { /* Offset-Tabelle bzw. nach Start sortierte Extents */ }
impl VariableLayout {
    /// Zusammenhängend: exklusive Prefix-Sum der Längen auf `base`; n Längen -> n+1 Offsets.
    pub fn from_lengths(base: u64, lengths: &[u64]) -> Result<Self, LayoutError>;
    /// Bestehende Segmente: Slot i = extents[i]; Reihenfolge beliebig, nicht überlappend.
    pub fn from_extents(extents: &[Extent]) -> Result<Self, LayoutError>;
    /// Die n+1 Offsets (absolute Adressen) nur bei `from_lengths`, sonst None.
    pub fn offsets(&self) -> Option<&[u64]>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Layout { Fixed(FixedLayout), Variable(VariableLayout) }

impl Layout {
    pub fn len(&self) -> usize;                       // Anzahl Slots
    pub fn is_empty(&self) -> bool;
    /// Adresse -> Slot. None, wenn die Adresse in keinem Slot liegt.
    pub fn slot_of(&self, addr: Addr) -> Option<Slot>;
    /// Adressbereich des Slots (Fixed: volle Stride-Breite; Variable: [start, start+len)).
    pub fn slot_range(&self, slot: Slot) -> Option<Range<u64>>;
    /// Umwandlung in Slot-Zuordnung; ids[i] gehört zu Slot i.
    pub fn into_slot_map(self, ids: Vec<EntryId>) -> Result<SlotMap, LayoutError>;
}

/// Wahl des Layouts aus Entry-Längen (Bytes).
/// `max_padding_permille`: höchste erlaubte Füll-/Padding-Quote in Promille.
pub fn choose_layout(base: u64, lengths: &[u64], max_padding_permille: u32) -> Result<Layout, LayoutError>;

/// Slot-Zuordnung: Layout + EntryId je Slot.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SlotMap { /* Layout, Vec<EntryId> */ }
impl SlotMap {
    pub fn layout(&self) -> &Layout;
    pub fn entry_of_slot(&self, slot: Slot) -> Option<EntryId>;
    /// Adresse -> Entry (slot_of + ids).
    pub fn entry_of(&self, addr: Addr) -> Option<EntryId>;
    pub fn resolve(&self, hit: KernelHit) -> Option<(EntryId, f32)>;
}
```

Alle Typen sind `Send + Sync`. Sie sind nach dem Bau unveränderlich. Mehrere Threads dürfen sie ohne Locking über `&` lesen.

## 4. Behavior und Invarianten

**Kernel-Ergebnis**
- Ein Kernel meldet pro Treffer nur `Addr` und `f32`-Score. Er meldet weder `EntryId` noch Längen.

**Festes Layout**
- `FixedLayout::new` mit `stride == 0` ergibt `ZeroStride`. Ein Überlauf von `base + count*stride` über u64 ergibt `Overflow`.
- Slot i belegt `[base + i*stride, base + (i+1)*stride)`.
- `slot_of(addr)` ist reine Arithmetik: `(addr - base) / stride`. Es gilt nur für `base <= addr < base + count*stride`, sonst `None`.
- Adressen im Padding eines Slots (hinter der echten Entry-Länge) liefern denselben Slot.

**Variables Layout, `from_lengths`**
- Die Offsets sind `o[0] = base`, `o[i+1] = o[i] + lengths[i]`. Bei n Längen gibt es n+1 Offsets. n = 0 ergibt `[base]`.
- Ein Überlauf ergibt `Overflow`. Mehr als `u32::MAX` Längen ergeben `TooManyEntries`.
- `slot_range(i) = o[i]..o[i+1]`.
- `slot_of(addr)` ist eine Binary Search. Sie liefert den Slot i mit `o[i] <= addr < o[i+1]`. Slots mit Länge 0 werden nie geliefert.
- Liegt `addr` außerhalb von `[o[0], o[n])`, ist das Ergebnis `None`.

**Variables Layout, `from_extents`**
- Slot i entspricht `extents[i]` in Eingabereihenfolge, unabhängig von der Adressreihenfolge.
- Intern sind die Extents nach `start` sortiert. `slot_of` sucht per Binary Search auf Arena-Offsets (Interval-Mapping) und liefert den Slot des Extents mit `start <= addr < start + len`.
- Überlappende Extents mit `len > 0` ergeben `Overlap`. Direkt aneinandergrenzende Extents sind erlaubt.
- Extents mit `len == 0` sind erlaubt und nie Ziel von `slot_of`.
- `start + len` über u64 ergibt `Overflow`.
- `offsets()` ist hier `None`.

**Auswahlregel `choose_layout`**
- Sei `n = lengths.len()`, `stride = max(lengths)` und `sum = Σ lengths`, gerechnet in u128.
- Das Ergebnis ist `Layout::Fixed` (mit `base`, `stride`, `count = n`), genau dann wenn alle Bedingungen gelten:
  - n > 0,
  - `stride > 0`,
  - `stride <= u32::MAX`,
  - n <= `u32::MAX`,
  - `(stride*n - sum) * 1000 <= max_padding_permille * stride*n`, das heißt Padding-Anteil ≤ Schwelle.
- Sonst ist das Ergebnis `Layout::Variable(from_lengths(base, lengths))`. Das betrifft auch n = 0 und alle Längen = 0.
- `max_padding_permille = 0` ergibt nur dann Fixed, wenn alle Längen gleich sind.
- Fehler stammen nur von `from_lengths` oder `FixedLayout::new`, etwa `Overflow`.

**Slot-Zuordnung**
- Beide Layouts sind über `into_slot_map` umwandelbar. `ids.len() != layout.len()` ergibt `CountMismatch { expected: layout.len(), got: ids.len() }`.
- `entry_of(addr)` gibt `ids[slot_of(addr)]` zurück, sonst `None`. `entry_of_slot` gibt `None` für Slots ≥ len zurück.
- `resolve(hit)` gibt `(entry, hit.score)` zurück oder `None`, wenn die Adresse unbekannt ist.
- `EntryId`s dürfen sich nicht wiederholen müssen. Das Modul prüft das nicht.

**Allgemein**
- Alle Funktionen sind frei von Panics bei gültigen Typen. Fehler laufen ausschließlich über `LayoutError`.
- Es gibt keinen globalen Zustand und keine Nebenläufigkeitsprobleme, weil die Typen unveränderlich sind.

## 5. Relied-on elements

Aus dem Allgemeinen Index-Vertrag (`index_contract`), unverändert genutzt:
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)] pub struct EntryId(pub u64);
```
`EntryId` ist total geordnet, `Copy` und `Send + Sync`. Weitere Elemente des Vertrags nutzt dieses Modul nicht.