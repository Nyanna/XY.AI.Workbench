# Spezifikation: Query-Batch und Dokumentquelle (Build-Order 70)

Crate-Modul `query_batch`.

## 1. Task, restrictions and boundaries

**Aufgabe**
- `Query` und `QueryBatch` bilden: Queries gleicher Dimension bündeln und prüfen.
- Das Trait `DocSource` definieren, über das Dokumentvektoren geliefert werden. `RamSource` ist die Implementierung im RAM.
- Slot-Bereiche vor dem Scoring prüfen (`check_slot_range`).
- `Workspace` bilden: ein halboffener Slot-Bereich, der einem Kernel-Launch entspricht.

**Grenzen**
- Dieses Modul enthält keinen Kernel, kein Scoring, keine Device-Speicherzugriffe, keine Allokation von Regionen und kein `unsafe`.
- Es nutzt `kernel_layout::Slot` aus der Abhängigkeit und definiert keine Typen der Abhängigkeiten neu.
- Die Source liefert nur `f32`-Werte. Encoding, Dtype und Padding gehören nicht hierher.
- Die Source ist read-only. Sie kennt keine Valid-Bits und keine Eviction.

## 2. Context

- Das Scoring liest jeden Dokumentabschnitt (Slot) einmal und rechnet ihn gegen alle Queries des Batches. Deshalb werden Queries gleicher Dimension gebündelt.
- `Slot` kommt aus `kernel_layout` und ist `Slot(pub u32)`. Die Slot-Größen sind in `slot_layout` definiert (`SegmentView`, `SlotStates`).
- Die RAM-Source ist die Referenzquelle für Dokumentvektoren. Ihr Offset je Slot ist die Summe der vorangehenden Slot-Bytes.
- Ein `Workspace` ist die Planungseinheit eines Launches. Große Slot-Bereiche werden mit `split_front` in Launches zerlegt.

## 3. Interface

```rust
use kernel_layout::Slot;
use std::ops::Range;

/// Segment-Kennung (lokal in diesem Modul definiert).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SegmentId(pub u64);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ShapeError {
    ZeroDim,
    EmptyData,
    NotMultipleOfDim { len: usize, dim: u32 },
    DimMismatch { expected: u32, got: u32 },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SourceError {
    /// Slot-Bereich liegt nicht in 0..slot_count des Segments.
    SlotRange { segment: SegmentId, start: u32, end: u32, slot_count: u32 },
}
// Beide Fehlertypen: Display + std::error::Error + Send + Sync.

/// Eine Query: Matrix Token × Dim, zeilenweise (row-major).
#[derive(Debug, Clone, PartialEq)]
pub struct Query { /* dim: u32, data: Vec<f32> */ }
impl Query {
    pub fn new(dim: u32, data: Vec<f32>) -> Result<Query, ShapeError>;
    pub fn dim(&self) -> u32;
    pub fn tokens(&self) -> usize;       // data.len() / dim
    pub fn data(&self) -> &[f32];        // Rohdaten, Zeile t = [t*dim..(t+1)*dim]
}

/// Bündel von Queries gleicher Dimension.
#[derive(Debug, Clone, PartialEq)]
pub struct QueryBatch { /* dim: u32, queries: Vec<Query> */ }
impl QueryBatch {
    pub fn new(dim: u32) -> Result<QueryBatch, ShapeError>;                  // dim == 0 → ZeroDim
    pub fn from_queries(dim: u32, queries: Vec<Query>) -> Result<QueryBatch, ShapeError>;
    pub fn push(&mut self, q: Query) -> Result<(), ShapeError>;
    pub fn dim(&self) -> u32;
    pub fn len(&self) -> usize;          // Anzahl Queries
    pub fn is_empty(&self) -> bool;
    pub fn get(&self, i: usize) -> Option<&Query>;
    pub fn tokens(&self, i: usize) -> Option<usize>;
    pub fn data(&self, i: usize) -> Option<&[f32]>;
    pub fn iter(&self) -> std::slice::Iter<'_, Query>;
}

/// Quelle für Dokumentvektoren. Send + Sync, nur &self.
pub trait DocSource: Send + Sync {
    fn dim(&self) -> u32;
    /// Anzahl Slots des Segments; unbekanntes Segment → 0.
    fn slot_count(&self, segment: SegmentId) -> u32;
    /// Byte-Offset des Slots im Segment (Treffer-Offset); None, wenn Segment unbekannt oder slot >= slot_count.
    fn slot_offset(&self, segment: SegmentId, slot: Slot) -> Option<u64>;
    /// Vektoren der gültigen Tokens (Länge = tokens * dim, row-major) oder None ("nicht vorhanden").
    fn slot_vectors(&self, segment: SegmentId, slot: Slot) -> Option<&[f32]>;
}

/// Prüft vor jedem Scoring: range.start <= range.end <= slot_count.
pub fn check_slot_range(src: &dyn DocSource, segment: SegmentId, range: Range<u32>)
    -> Result<(), SourceError>;

/// RAM-Source.
#[derive(Debug, Clone)]
pub struct RamSource { /* dim, segments: HashMap<SegmentId, Vec<Option<Vec<f32>>>>, offsets */ }
impl RamSource {
    pub fn new(dim: u32) -> Result<RamSource, ShapeError>;               // dim == 0 → ZeroDim
    /// Ersetzt das Segment. None = Slot nicht vorhanden. Jeder Some-Vektor: Länge Vielfaches von dim.
    pub fn insert_segment(&mut self, segment: SegmentId, slots: Vec<Option<Vec<f32>>>)
        -> Result<(), ShapeError>;
}
impl DocSource for RamSource { /* siehe Abschnitt 4 */ }

/// Halboffener Slot-Bereich start..end; Einheit eines Kernel-Launches.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Workspace { /* start: u32, end: u32, invariant end >= start */ }
impl Workspace {
    pub fn new(start: u32, end: u32) -> Workspace;   // end < start → leer bei start..start
    pub fn start(&self) -> u32;
    pub fn end(&self) -> u32;
    pub fn len(&self) -> u32;                        // end - start, nie negativ
    pub fn is_empty(&self) -> bool;
    pub fn range(&self) -> Range<u32>;
    /// (Vorderteil mit len <= max_len, Rest). Vorderteil ++ Rest = self.
    pub fn split_front(self, max_len: u32) -> (Workspace, Workspace);
}
```

## 4. Behavior and invariants

**Query / QueryBatch**
- `Query::new` prüft in dieser Reihenfolge:
  1. `dim == 0` → `ZeroDim`.
  2. `data.is_empty()` → `EmptyData`.
  3. `data.len() % dim != 0` → `NotMultipleOfDim { len, dim }`.
- `tokens() == data.len() / dim`, mindestens 1. Die Daten werden unverändert gehalten, ohne Umsortierung oder Kopie-Transformation.
- `QueryBatch::new(dim)` ergibt einen leeren Batch (`is_empty() == true`). `dim == 0` → `ZeroDim`.
- `push` und `from_queries` lehnen eine Query mit `q.dim() != batch.dim()` mit `DimMismatch { expected: batch.dim, got: q.dim }` ab. Bei einem Fehler bleibt der Batch unverändert. `from_queries` bricht beim ersten Fehler ab.
- Die Reihenfolge der Queries ist die Einfügereihenfolge. Der Index `i` ist stabil.
- `get`, `tokens` und `data` liefern für `i >= len()` `None`.

**RamSource**
- `dim()` ist die Dimension aus `new`.
- `insert_segment` ersetzt ein vorhandenes Segment mit gleicher ID. Ein Some-Vektor mit Länge nicht Vielfaches von dim → `NotMultipleOfDim`. Bei einem Fehler wird nichts eingefügt. Ein leerer Some-Vektor (0 Tokens) ist erlaubt. Er ist vorhanden und liefert eine leere Slice.
- `slot_count(seg)` = Länge der Slot-Liste, für unbekannte Segmente 0.
- Slot-Bytes eines Slots = `Vektor-Länge * 4` (4 Byte je f32-Wert), für `None`-Slots 0.
- `slot_offset(seg, i)` = Summe der Slot-Bytes aller Slots `0..i`. Slot 0 hat Offset 0. Er ist als u64 gerechnet und beim Einfügen vorberechnet. Bei unbekanntem Segment oder `i >= slot_count` → `None`.
- `slot_vectors(seg, i)` gibt `Some(&[f32])` für vorhandene Slots zurück (Länge = tokens·dim), sonst `None` (unbekanntes Segment, außerhalb, `None`-Slot).
- Nach dem Aufbau ist `RamSource` unveränderlich lesbar. Gleichzeitige Leser sind erlaubt (`Send + Sync`).

**check_slot_range**
- Ok genau dann, wenn `range.start <= range.end <= src.slot_count(segment)`.
- Sonst `SlotRange { segment, start, end, slot_count }` mit den übergebenen Grenzen und der Slot-Anzahl.
- Ein leerer Bereich `n..n` mit `n <= slot_count` ist Ok. Bei unbekanntem Segment ist nur `0..0` Ok.
- Der Aufrufer prüft vor jedem Scoring. Es gibt keine stille Kürzung.

**Workspace**
- Invariante: `end >= start`, `len() = end - start`, nie negativ und ohne Unterlauf.
- `new(s, e)` mit `e < s` ergibt `s..s`.
- `is_empty() == (len() == 0)`.
- `split_front(max_len)` ergibt `front = start..start+min(len, max_len)` und `rest = front.end..end`.
  - `front.len() <= max_len`, `front.len() + rest.len() == len`, `rest.start == front.end`.
  - `max_len == 0` → leerer Vorderteil `start..start` und Rest = self.
  - Ein leerer Workspace ergibt zwei leere Teile.
- Wiederholtes `split_front(n)` auf dem Rest mit `n > 0` zerlegt den Bereich in lückenlose Launches, bis der Rest leer ist.

**Allgemein**
- Alle Typen sind `Send + Sync`. Es gibt keine Panics bei gültigen Eingaben und keine stillen Überläufe.

## 5. Relied-on elements

- `kernel_layout::Slot(pub u32)` – Slot-Index, `Copy`. Es wird `slot.0` als Index verwendet.
- Aus `slot_layout` wird nichts verwendet. Die Slot-Anzahl eines Segments entspricht `SegmentView::slot_count()`. Der Aufrufer bildet die View und liefert die Source passend dazu.