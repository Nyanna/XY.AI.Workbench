# Spezifikation: Slot-Layout und Segment-View (Build-Order 62)

Crate-Modul `slot_layout`.

## 1. Task, restrictions and boundaries

**Aufgabe**
- Slot-Größen und Slot-Klassen festlegen.
- Eine `SegmentView` über einer Region bilden und den Kernel-`Descriptor` liefern.
- Slot-Inhalte kodieren (Token-Zeilen, Mask-Padding, FP32/FP16/BF16 little-endian).
- Je Slot Valid-Bit und Tokenzahl thread-safe führen (`SlotStates`).
- Die nebenläufige Zuordnung Slot → `EntryRef` führen (`SlotAssignment`), mit Adress- und Bytebereichs-Abfragen.

**Grenzen**
- Dieses Modul löst keine Device-Adressen auf. Die Device-Basisadresse der Region liefert der Aufrufer (Memory-Backend) als `device_base: Addr`.
- Es enthält keinen Speicherzugriff, keine Allokation, kein Schreiben ins Device, keinen Kernel, keine Eviction und keinen Generationszähler.
- Es enthält kein `unsafe`.
- Es definiert keine Typen der Abhängigkeiten neu.
- Abhängigkeiten sind `region_handle`, `layout` (nur `DType` und `MASK_VALUE`), `kernel_layout` und `index_contract` (nur `EntryId`).

## 2. Context

- Eine Segment-View beschreibt, wie eine Region (`RegionHandle`) in gleich große Slots für Entries bzw. Abschnitte zerlegt ist.
- Der Kernel erhält nur den `Descriptor` (Start, Anzahl, Stride) und meldet `KernelHit { addr, score }`. Die Rückabbildung erfolgt über `kernel_layout::Layout::slot_of` bzw. `SlotAssignment`.
- Der Manager befüllt Slots mit den Bytes aus `encode_slot` und setzt danach das Valid-Bit. Beim Eviction setzt er einen Tombstone.
- Der Valid-Bit-Snapshot (u64-Wörter) wird zum Upload für den Kernel genutzt.

## 3. Interface

```rust
use index_contract::EntryId;
use kernel_layout::{Addr, FixedLayout, Layout as AddrLayout, LayoutError as AddrLayoutError, Slot};
use layout::DType;
use region_handle::{Generation, RegionHandle};
use std::ops::Range;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SlotError {
    InvalidClass { max_tokens: u32 },              // 0 oder > 65_535
    InvalidDimension,                              // dim == 0
    SlotSizeOverflow,                              // u32-Überlauf (oder u64) der Slot-Größe
    RegionTooSmall { needed: u64, available: u64 },
    SlotOutOfRange { slot: u32, count: u32 },
    UnsupportedDType { dtype: DType },
    TooManyTokens { tokens: usize, max: usize },
    ValueCountMismatch { expected: usize, got: usize },
    TokensExceedClass { tokens: u32, max: u32 },   // set_filled
    SlotOccupied { slot: u32, entry: EntryRef },   // bind
    EntryAlreadyBound { entry: EntryRef, slot: u32 },
    Addr(AddrLayoutError),                         // Layout-Bau (Overflow, ...)
}
impl std::fmt::Display for SlotError {}
impl std::error::Error for SlotError {}

/// Token-Obergrenze je Slot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SlotClass(/* u32, 1..=65_535 */);
impl SlotClass {
    pub const C32: SlotClass; pub const C64: SlotClass; pub const C128: SlotClass;
    pub fn custom(max_tokens: u32) -> Result<SlotClass, SlotError>;
    pub fn max_tokens(self) -> u32;
}

/// Slot-Größe in Bytes = max_tokens * dim * dtype.size_in_bytes(), geprüft; Ergebnis muss in u32 passen.
pub fn slot_size(class: SlotClass, dim: u32, dtype: DType) -> Result<u32, SlotError>;

/// Kodiert einen Slot (volle Slot-Größe).
pub fn encode_slot(class: SlotClass, dim: u32, dtype: DType, tokens: usize, values: &[f32])
    -> Result<Vec<u8>, SlotError>;

/// Kernel-Descriptor.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Descriptor { pub start: Addr, pub slot_count: u32, pub stride: u32 }
impl Descriptor { pub fn to_fixed_layout(&self) -> Result<FixedLayout, AddrLayoutError>; }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SegmentView { /* region, slot_count, class, dim, dtype, generation, slot_bytes */ }
impl SegmentView {
    pub fn new(region: RegionHandle, slot_count: u32, class: SlotClass, dim: u32,
               dtype: DType, generation: Generation) -> Result<SegmentView, SlotError>;
    pub fn region(&self) -> RegionHandle;
    pub fn slot_count(&self) -> u32;
    pub fn class(&self) -> SlotClass;
    pub fn dim(&self) -> u32;
    pub fn dtype(&self) -> DType;
    pub fn generation(&self) -> Generation;       // Segment-Generation
    pub fn slot_bytes(&self) -> u32;              // Stride
    pub fn total_bytes(&self) -> u64;             // slot_count * slot_bytes
    /// Bereich des Slots relativ zum Region-Anfang.
    pub fn slot_range(&self, slot: Slot) -> Result<Range<u64>, SlotError>;
    pub fn descriptor(&self, device_base: Addr) -> Result<Descriptor, SlotError>;
    pub fn encode_slot(&self, tokens: usize, values: &[f32]) -> Result<Vec<u8>, SlotError>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SlotMeta { pub valid: bool, pub tokens: u32 }

/// Valid-Bits und Tokenzahlen eines Segments. Send + Sync, alle Methoden &self.
pub struct SlotStates { /* AtomicU64-Wörter, AtomicU32 je Slot */ }
impl SlotStates {
    pub fn new(view: &SegmentView) -> SlotStates;               // alle Slots ungültig
    pub fn slot_count(&self) -> u32;
    pub fn set_filled(&self, slot: Slot, tokens: u32) -> Result<(), SlotError>;
    pub fn invalidate(&self, slot: Slot) -> Result<(), SlotError>;   // Tombstone
    pub fn is_valid(&self, slot: Slot) -> bool;                 // außerhalb: false
    pub fn valid_count(&self) -> u32;
    pub fn meta(&self, slot: Slot) -> Result<SlotMeta, SlotError>;
    pub fn snapshot_words(&self) -> Vec<u64>;
}

/// Entry-Referenz: Entry-ID plus Abschnittsnummer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EntryRef { pub entry: EntryId, pub section: u32 }

/// Nebenläufige Zuordnung Slot ↔ EntryRef. Send + Sync, alle Methoden &self.
pub struct SlotAssignment { /* RwLock<Zuordnung + Rückwärtsindex>, AddrLayout, RegionHandle, device_base */ }
impl SlotAssignment {
    /// Feste Adressierung: Stride/Anzahl aus der View.
    pub fn fixed(view: &SegmentView, device_base: Addr) -> Result<SlotAssignment, SlotError>;
    /// Variable Adressierung: Slot i hat lengths[i] Bytes, zusammenhängend ab Region-Anfang.
    pub fn variable(region: RegionHandle, device_base: Addr, lengths: &[u64])
        -> Result<SlotAssignment, SlotError>;
    pub fn slot_count(&self) -> u32;
    pub fn bind(&self, slot: Slot, entry: EntryRef) -> Result<(), SlotError>;
    pub fn unbind(&self, slot: Slot) -> Result<Option<EntryRef>, SlotError>;
    pub fn lookup(&self, slot: Slot) -> Option<EntryRef>;
    pub fn slot_of_entry(&self, entry: EntryRef) -> Option<Slot>;
    pub fn device_addr(&self, slot: Slot) -> Result<Addr, SlotError>;
    /// Absoluter Byte-Bereich im Arena-Offsetraum.
    pub fn byte_range(&self, slot: Slot) -> Result<Range<u64>, SlotError>;
    /// Umspannender Bereich aller gebundenen Slots; None, wenn nichts gebunden.
    pub fn spanning_range(&self) -> Option<Range<u64>>;
}
```

## 4. Behavior and invariants

**Slot-Größe und Klassen**
- `C32`, `C64`, `C128` haben 32, 64 und 128 Token. `custom(n)` gilt für 1..=65 535, sonst `InvalidClass { max_tokens: n }`. `custom(32) == C32`.
- `slot_size` ist `max_tokens * dim * dtype.size_in_bytes()`, gerechnet mit Überlaufprüfung (z. B. in u64/u128). Ergebnis > `u32::MAX` oder Überlauf ergibt `SlotSizeOverflow`. `dim == 0` ergibt `InvalidDimension`.
- Beispiel: C32, dim 128, F16 ergibt 8192.
- `slot_size` akzeptiert jeden `DType`. Die Dtype-Prüfung gehört zum Encoding.

**SegmentView**
- `new` berechnet `slot_bytes` mit `slot_size`. Fehler daraus werden durchgereicht.
- `needed = slot_count as u64 * slot_bytes as u64`, `available = region.len()`. Bei `needed > available` kommt `RegionTooSmall { needed, available }`. Eine Region größer als nötig ist erlaubt.
- `slot_count == 0` ist erlaubt.
- Die View ist ein unveränderlicher `Copy`-Wert. Die Segment-Generation ist ein eigenes Feld und wird nicht gegen `region.generation()` geprüft.
- `total_bytes() = slot_count * slot_bytes`. `slot_range(i) = i*slot_bytes .. (i+1)*slot_bytes`, relativ zum Region-Anfang. Bei `i >= slot_count` kommt `SlotOutOfRange { slot: i, count: slot_count }`.
- `descriptor(base)` liefert `Descriptor { start: base, slot_count, stride: slot_bytes }`. Die Summe `base + total_bytes` darf u64 nicht überlaufen, sonst `Addr(Overflow)`.
- `Descriptor::to_fixed_layout()` ist `FixedLayout::new(start.0, stride, slot_count)`.

**Encoding** (`encode_slot`, `SegmentView::encode_slot` mit den Parametern der View)
- Prüfreihenfolge:
  1. `slot_size`-Fehler.
  2. Dtype nicht F32/F16/BF16 → `UnsupportedDType { dtype }`.
  3. `tokens > class.max_tokens()` → `TooManyTokens { tokens, max }`.
  4. `values.len() != tokens*dim` → `ValueCountMismatch { expected: tokens*dim, got }`.
- Ausgabe: genau `slot_size` Bytes. Zeile t (0 ≤ t < tokens) steht an Offset `t*dim*elem` und enthält `values[t*dim..(t+1)*dim]` in Eingabereihenfolge. Es gibt keine Umsortierung.
- Danach folgen bis `max_tokens` Padding-Zeilen. Jedes Element ist `MASK_VALUE` (−10000.0), in den Zieltyp gerundet. Das ist nie 0 und endlich.
- Elemente sind little-endian. F32 sind die IEEE-Bytes von `f32::to_le_bytes`.
  - F16 ist IEEE binary16, F32 → F16 mit Round-to-nearest-even. Überlauf gibt ±Inf, NaN bleibt NaN, Subnormale werden korrekt behandelt.
  - BF16 sind die oberen 16 Bit von F32 mit Round-to-nearest-even. NaN bleibt NaN.
  - Die Konvertierung ist lokal implementiert. `layout` und andere Module konvertieren nicht.
- `tokens == 0` ergibt einen Slot nur aus Padding.
- Padding wird nicht in der Tokenzahl gezählt. Die Tokenzahl führt `SlotStates`.

**SlotStates**
- `new` legt `slot_count` Slots an, alle ungültig mit Tokenzahl 0. `class.max_tokens()` wird für die Prüfung gemerkt.
- `set_filled(slot, tokens)` prüft zuerst den Slot (`SlotOutOfRange`), dann `tokens <= max` (`TokensExceedClass { tokens, max }`).
  - Danach speichert es die Tokenzahl und setzt das Valid-Bit mit Release-Semantik.
  - Ein Leser, der `is_valid == true` mit Acquire sieht, sieht auch die zugehörige Tokenzahl.
  - `tokens == 0` ist erlaubt (gültig, leer).
- `invalidate` löscht das Valid-Bit und setzt die Tokenzahl auf 0 (Tombstone). Es ist idempotent.
- `is_valid` ist für Slots ≥ `slot_count` `false`, ohne Fehler.
- `valid_count` ist die Anzahl gesetzter Bits.
- `meta(slot)` liefert `SlotMeta { valid, tokens }`. Ist der Slot ungültig, ist `tokens == 0`. Außerhalb des Bereichs kommt `SlotOutOfRange`.
- `snapshot_words()` hat `ceil(slot_count/64)` Wörter. Slot i ist Bit `i % 64` von Wort `i / 64` (LSB = niedrigster Slot). Ungenutzte hohe Bits sind 0. Bei 0 Slots ist der Snapshot leer.
- Der Snapshot ist je Wort atomar, nicht über alle Wörter konsistent.
- Alle Operationen sind lock-frei oder kurz und thread-safe. Parallele `set_filled` und `invalidate` auf verschiedenen Slots stören sich nicht, auch nicht im selben Wort (Bitoperationen per `fetch_or`/`fetch_and`).

**SlotAssignment**
- `fixed` baut `FixedLayout::new(device_base.0, slot_bytes, slot_count)` und speichert die Region der View. Layout-Fehler kommen als `Addr(..)`.
  - `slot_count == 0` ist erlaubt.
  - Wegen `ZeroStride` im Kernel-Layout kann `fixed` mit 0 Slots einen `Addr(..)`-Fehler vermeiden, indem `stride` aus der View (> 0) genutzt wird. `slot_bytes` ist immer > 0.
- `variable` baut `VariableLayout::from_lengths(device_base.0, lengths)`. Die Summe der Längen muss ≤ `region.len()` sein, sonst `RegionTooSmall { needed: Summe, available }`. Überlauf oder zu viele Längen kommen als `Addr(..)`.
- Anfangs ist kein Slot gebunden.
- `bind(slot, entry)`:
  - Prüfreihenfolge: Bereich (`SlotOutOfRange`), dann Slot belegt (`SlotOccupied { slot, entry: aktueller }`), dann Entry anderswo gebunden (`EntryAlreadyBound { entry, slot: bestehender }`).
  - Im Erfolgsfall werden Vorwärts- und Rückwärtszuordnung atomar unter demselben Schreib-Lock gesetzt.
  - Die Zuordnung ändert `SlotStates` nicht.
- `unbind(slot)` gibt die bisherige Referenz zurück (`None`, wenn nichts gebunden war) und entfernt beide Richtungen. Außerhalb des Bereichs kommt `SlotOutOfRange`.
- `lookup(slot)` gibt `None` für ungebundene und für ungültige Slots zurück. `slot_of_entry` gibt `None`, wenn nicht gebunden.
- `device_addr(slot)` ist `Addr(layout.slot_range(slot).start)`. Fixed: `base + i*stride`. Variable: Offset-Tabelle.
- `byte_range(slot)` ist `region.offset() + (r.start - device_base) .. region.offset() + (r.end - device_base)` mit `r = layout.slot_range(slot)`. Das ist ein absoluter Bereich im Arena-Offsetraum. Er ist bei variablen Slots mit Länge 0 leer. Außerhalb des Bereichs kommt `SlotOutOfRange { slot, count }`.
- `spanning_range()` ist `min(start) .. max(end)` der `byte_range` aller aktuell gebundenen Slots. Ohne gebundenen Slot ist es `None`.
- Lesende Methoden (`lookup`, `slot_of_entry`, `spanning_range`) nehmen den Lese-Lock und blockieren sich gegenseitig nicht. `bind` und `unbind` nehmen den Schreib-Lock. Jede Operation ist atomar, es gibt keine halb sichtbaren Zustände.
- Ein vergifteter Lock führt nicht zu Panics. Der Zustand wird weiterverwendet (`into_inner`).

**Allgemein**
- Alle Fehler laufen über `SlotError`. Es gibt keine Panics bei gültigen Eingaben und keine stillen Überläufe.
- `SegmentView`, `Descriptor`, `SlotClass`, `EntryRef`, `SlotMeta`, `SlotStates`, `SlotAssignment` und `SlotError` sind `Send + Sync`. Das wird per statischem Test geprüft.

## 5. Relied-on elements

- `layout`
  - `DType { F32, F16, BF16, U8, U32, I64 }` mit `size_in_bytes()`.
  - `pub const MASK_VALUE: f32 = -10_000.0;`
- `region_handle`
  - `RegionHandle` ist `Copy`. Verwendet werden `len() -> u64`, `offset() -> u64` (ArenaOffset) und `generation()`.
  - `Generation(pub u64)` ist `Copy + Eq`.
- `kernel_layout`
  - `Addr(pub u64)`, `Slot(pub u32)`.
  - `FixedLayout::new(base: u64, stride: u32, count: u32) -> Result<_, LayoutError>`. `ZeroStride` bei Stride 0, `Overflow` bei `base + count*stride` über u64.
  - `VariableLayout::from_lengths(base, &[u64])`.
  - `Layout::{Fixed, Variable}` mit `slot_range(Slot) -> Option<Range<u64>>` (absolute Adressen) und `len()`.
  - `LayoutError` (u. a. `Overflow`, `TooManyEntries`, `ZeroStride`) als `AddrLayoutError`.
- `index_contract::EntryId(pub u64)` ist `Copy + Eq + Hash + Ord + Send + Sync`.