# Spezifikation: CPU-Compute-Primitive (Build-Order 28)

## 1. Aufgabe, Einschränkungen, Grenzen
**Aufgabe:** Rust-Modul `cpu_compute` mit vier Funktionen auf `f32`-Daten: Dot-Product, Max-Similarity (Late Interaction), L2-Normalisierung und slotweiser Scan.

**Einschränkungen:**
- Der Scorer ist selbst implementiert. Es gibt keine BLAS-, ndarray- oder SIMD-Fremdcrates. Nur `std`.
- Es gibt keinen Zustand, keine Threads, keine I/O, keine Allokation in den inneren Schleifen und keine Matrix `Q×D`.
- Es gibt keine Speicherverwaltung. Das Modul liest nur geliehene Slices.
- Nicht enthalten sind Top-k, Sortierung, GPU, Dtype-Konvertierung (nur `f32`) und Parallelisierung über Slots.

## 2. Kontext
Das Modul ist der CPU-Gegenpart zum GPU-Scorer. Es liefert dieselben Scores und dieselbe Semantik für leere Dokumente. Aufrufer sind Retrieval- und Index-Module. Sie halten Vektoren pro Slot als flache `f32`-Slices (Token-major, `tokens × dim`) und reichen sie hier durch.

Von "Layout und Elementtypen" wird nur `MASK_VALUE` genutzt.

## 3. Interface
```rust
use crate::layout::MASK_VALUE; // f32 = -10_000.0

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CpuComputeError {
    /// dim == 0
    InvalidDim,
    /// Slot-Bereich reicht über die Slot-Liste hinaus oder start > end
    RangeOutOfBounds { start: usize, end: usize, slots: usize },
    /// Länge nicht durch dim teilbar
    Shape { what: ShapeSource, len: usize, dim: usize },
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShapeSource { Query(usize), Slot(usize), Doc }
impl std::fmt::Display for CpuComputeError { /* lesbare Meldung */ }
impl std::error::Error for CpuComputeError {}

/// Panics, wenn a.len() != b.len().
pub fn dot(a: &[f32], b: &[f32]) -> f32;

/// query: q_tokens*dim, doc: d_tokens*dim, flach, Token-major.
pub fn max_sim(query: &[f32], doc: &[f32], dim: usize) -> Result<f32, CpuComputeError>;

/// In-place L2-Normalisierung eines Vektors.
pub fn l2_normalize(v: &mut [f32]);

/// Scan über slots[range]. Ergebnis flach, Länge queries.len() * range.len(),
/// Index = q * range.len() + (slot - range.start).
pub fn scan_slots(
    queries: &[&[f32]],
    slots: &[Option<&[f32]>],
    range: std::ops::Range<usize>,
    dim: usize,
) -> Result<Vec<f32>, CpuComputeError>;
```
Alle Funktionen sind frei, reentrant und `Send + Sync`-tauglich (reine Funktionen). `CpuComputeError` ist `Send + Sync`.

## 4. Verhalten und Invarianten
**dot**
- Verwendet 8 unabhängige Teilsummen. Lane `j` summiert `a[i]*b[i]` für `i % 8 == j` über volle 8er-Blöcke (z. B. mit `chunks_exact(8)` und `[f32; 8]`-Akkumulator), damit Auto-Vektorisierung möglich ist.
- Der Rest (`len % 8`) wird nach den Blöcken skalar auf einen separaten Rest-Akkumulator addiert.
- Reduktion: `((l0+l1)+(l2+l3)) + ((l4+l5)+(l6+l7))`, danach `+ Rest`. Diese Reihenfolge ist fest, damit das Ergebnis deterministisch ist.
- Leere Slices ergeben `0.0`.

**max_sim**
- Gibt `Σ_{q in Query-Tokens} max_{d in Doc-Tokens} dot(q, d)` zurück.
- `dim == 0` ergibt `InvalidDim`. `query.len() % dim != 0` ergibt `Shape{what: Doc}`-frei, nämlich `Shape{what: Query(0), ..}`. `doc.len() % dim != 0` ergibt `Shape{what: Doc, ..}`.
- Die Prüfreihenfolge ist: dim, Query, Doc.
- Ein Dokument ohne Tokens (`doc.is_empty()`) ergibt `MASK_VALUE * q_tokens as f32`.
- Eine Query ohne Tokens ergibt `0.0`. Das gilt auch bei leerem Dokument.
- Das Maximum je Query-Token startet bei `f32::NEG_INFINITY` und wird laufend aktualisiert (`if s > m`). Es entsteht keine Score-Matrix.

**l2_normalize**
- Teilt jedes Element durch `sqrt(Σ x²)`.
- Ist die Norm genau `0.0`, bleibt der Vektor unverändert (Nullvektor bleibt Nullvektor, kein NaN). Ein leerer Slice bleibt leer.
- Es gibt keine Epsilon-Addition.

**scan_slots**
- Prüfreihenfolge:
  1. `dim == 0` ergibt `InvalidDim`.
  2. `range.start > range.end` oder `range.end > slots.len()` ergibt `RangeOutOfBounds`.
  3. Jede Query-Länge `% dim != 0` ergibt `Shape{what: Query(i)}`.
  4. Danach folgt der Scan. Dort ergibt jeder Slot in `range` mit `len % dim != 0` den Fehler `Shape{what: Slot(slot_index)}`. `slot_index` ist der absolute Index in `slots`.
- Die Prüfungen 1–3 laufen vor jeder Berechnung. Slot-Prüfung (4) läuft, bevor der Slot ausgewertet wird.
- Der erste Fehler bricht den gesamten Scan ab. Es gibt kein Teilergebnis (`Err` ohne Daten).
- Ein Slot mit `None` wird wie ein leerer Slice behandelt. Er ergibt je Query `MASK_VALUE * q_tokens`. Eine Query ohne Tokens ergibt `0.0`.
- Ein leerer Bereich (`start == end`) ergibt `Ok(vec![])`. Das gilt auch bei leeren `queries`.
- Slots sind die äußere Schleife und laufen über alle Queries. Je Query-Token läuft ein laufendes Maximum über die Doc-Tokens des Slots, ohne Matrix.
- Pro Slot und Query ist das Ergebnis identisch zu `max_sim(query, slot_or_empty, dim)`.
- Es gibt keine Panics bei gültigen Eingaben und keinen geteilten Zustand.

## 5. Relied-on elements
- `layout::MASK_VALUE: f32 = -10_000.0`. Die Konstante ist endlich und wird additiv verwendet. Sie darf nicht neu definiert werden.