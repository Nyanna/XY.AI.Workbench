# Spezifikation: Index-Aufbau durch Kernel (Build-Order 96)

## 1. Task, restrictions and boundaries

**Task.** Rust-Modul `index_build`. Es vergibt vorab einen beschreibbaren Bereich aus der Segment-Arena (`ArenaKind::IndexSegments`). Es gibt dem Aufrufer daraus per Cursor Teilbereiche (Spans) für Kernel-Schreibzugriffe. Der Kernel schreibt nur in vergebene Spans und allokiert nie. Am Ende liefert das Modul den fertig beschriebenen Bereich als Segment und optional als `RegionHandle`.

**Nicht Teil des Moduls:**
- Kein Kernel-Start, keine Device-Aufrufe, keine Events, keine Synchronisation mit dem Kernel.
- Keine Berechnung der Embeddings und keine Inhalts- oder Layoutentscheidung. Größen, Alignments und Reihenfolge liefert der Aufrufer.
- Keine eigene Allokation außer dem einen Vorab-Aufruf `SegmentArena::allocate_bump`. Kein Wachsen und kein Nachallokieren.
- Keine Publikation (`TableSwap`) und kein Replacement. Keine Generationsvergabe: Die Generation liefert der Aufrufer.
- Keine Redefinition von `SegmentArena`, `Segment`, `BumpSegment`, `SegmentArenaError`, `Readers`, `RegionHandle` oder `Generation`.

## 2. Context

- `SegmentArena<A>` teilt ausgerichtete Segmente zu und liefert `BumpSegment` (Cursor, `&mut`, ein Writer). Der Kernel allokiert nie. Alle Größen müssen vor dem Start feststehen.
- `RegionHandle` ist der pointerfreie Wert, mit dem spätere Module den fertigen Index referenzieren. Device-Adressen verlassen das Backend nicht. Dieses Modul gibt nur Arena-Offsets heraus.
- Dieses Modul ist eine dünne Schicht: Es hält das Bump-Segment und die Arena-Referenz und prüft die Zustandsfolge.

## 3. Interface

```rust
use std::borrow::Borrow;
use allocator::Allocator;
use device::Event; // nur über Readers
use region_handle::{Generation, RegionHandle, RegionError};
use segment_arena::{BumpSegment, Readers, Segment, SegmentArena, SegmentArenaError};

/// Vorab vergebener, beschreibbarer Teilbereich für einen Kernel-Schreibzugriff.
/// `offset` ist ein absoluter Arena-Offset (ArenaKind::IndexSegments).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct WriteSpan { pub offset: u64, pub len: u64 }

impl WriteSpan {
    /// offset + len; überlaufsicher, da bei der Vergabe geprüft.
    pub fn end(&self) -> u64;
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum IndexBuildError {
    /// Fehler der Segment-Arena beim Vorab-Allokieren, Release oder aus dem Cursor.
    Arena(SegmentArenaError),
    /// Handle-Konstruktion fehlgeschlagen.
    Handle(RegionError),
}
impl std::fmt::Display for IndexBuildError {}
impl std::error::Error for IndexBuildError {}
impl From<SegmentArenaError> for IndexBuildError {}
impl From<RegionError> for IndexBuildError {}

/// Ergebnis eines abgeschlossenen Aufbaus.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BuiltIndex {
    /// Gesamtes vorab vergebenes Segment (zur späteren Freigabe über die Arena).
    pub segment: Segment,
    /// Bytes ab `segment.offset`, die per Cursor vergeben wurden (inkl. Alignment-Padding).
    pub written: u64,
}

impl BuiltIndex {
    /// Handle auf den vergebenen Teil: (arena = segment.region.arena, offset = segment.offset,
    /// len = written, generation). Fehler: `Handle(Overflow)` bei offset+written-Überlauf.
    pub fn handle(&self, generation: Generation) -> Result<RegionHandle, IndexBuildError>;
}

/// Index-Aufbau über einem vorab vergebenen Bereich. Ein Writer (`&mut`).
/// Send, wenn A: Send/Sync-Bedingungen der Arena-Referenz erfüllt.
pub struct IndexBuild<'a, A: Borrow<Allocator>> { /* &'a SegmentArena<A>, BumpSegment */ }

impl<'a, A: Borrow<Allocator>> IndexBuild<'a, A> {
    /// Vergibt vorab `capacity` Bytes per `SegmentArena::allocate_bump`.
    pub fn begin(arena: &'a SegmentArena<A>, capacity: u64) -> Result<Self, IndexBuildError>;

    /// Vergibt per Cursor den nächsten Span (`BumpSegment::claim`), ausgerichtet auf `align`.
    pub fn reserve(&mut self, len: u64, align: u64) -> Result<WriteSpan, IndexBuildError>;

    /// Bereits per Cursor vergebene Bytes (inkl. Padding) / noch freie Bytes.
    pub fn written(&self) -> u64;
    pub fn remaining(&self) -> u64;
    /// Das zugrunde liegende Segment.
    pub fn segment(&self) -> &Segment;

    /// Schließt den Aufbau ab und gibt das Segment samt `written` an den Aufrufer.
    /// Das Segment wird NICHT freigegeben.
    pub fn finish(self) -> BuiltIndex;

    /// Bricht ab und gibt das Segment über `SegmentArena::release` frei.
    pub fn abort(self, readers: Readers) -> Result<(), IndexBuildError>;
}
```

## 4. Behavior and invariants

**Vorab-Vergabe**
- `begin(arena, capacity)` ruft genau einmal `arena.allocate_bump(capacity)` auf. Danach gibt es nie wieder eine Arena-Allokation, weder in `reserve`, `finish` noch `abort`.
- Fehler aus `allocate_bump` kommen als `Arena(e)` zurück, z. B. `ZeroLength` bei `capacity == 0` und `Exhausted`. Dann entsteht kein Zustand und es wurde nichts belegt.
- Die nutzbare Länge ist `segment().len`, also `capacity` aufgerundet auf das Arena-Alignment. `remaining()` startet bei `segment().len`.

**Cursor und Spans**
- `reserve(len, align)` delegiert an `BumpSegment::claim(len, align)`. Der Span ist `WriteSpan { offset: Rückgabe von claim, len }`.
- Alle Spans liegen innerhalb `[segment.offset, segment.offset + segment.len)`. Sie sind paarweise disjunkt, aufsteigend und `offset % align == 0`.
- Alignment-Padding zählt als `written`. Es gilt `written() + remaining() == segment().len`.
- `len == 0` ist erlaubt und liefert einen leeren Span an der ausgerichteten Adresse.
- `align` keine Zweierpotenz oder 0: `Arena(InvalidAlignment)`. Reicht der Platz nicht oder überläuft die Rechnung: `Arena(Exhausted { requested: len, available: remaining() })`. In beiden Fällen bleibt der Cursor unverändert.
- Ein Kernel darf nur in Bereiche schreiben, die durch zurückgegebene Spans abgedeckt sind. Die Wahrung liegt beim Aufrufer. Reicht der Platz nicht, schlägt `reserve` vor dem Kernel-Start fehl, und der Kernel muss nie allokieren.
- Ein Span, der vergeben wurde, bleibt bis `finish` oder `abort` gültig. Der Cursor geht nie zurück.

**Abschluss**
- `finish(self)` konsumiert den Build und gibt `BuiltIndex { segment, written }` zurück. Das Segment bleibt live in der Arena und wird vom Aufrufer über `SegmentArena::release` (oder Replacement) verwaltet.
- `BuiltIndex::handle(generation)` erzeugt `RegionHandle::new(segment.region.arena, segment.offset, written, generation)`. Fehler von `new` werden zu `Handle(e)`. `written == 0` ist erlaubt (leeres Handle).
- `abort(self, readers)` ruft `arena.release(&segment, readers)` auf. Fehler kommen als `Arena(e)` zurück, z. B. `StaleRegion`.
- `IndexBuild` hat keinen `Drop`-Effekt. Wer weder `finish` noch `abort` aufruft, behält das Segment belegt, und der Aufrufer ist dafür verantwortlich. Es gibt keine automatische Freigabe, weil ein Kernel noch schreiben könnte.

**Nebenläufigkeit**
- `IndexBuild` ist für genau einen Writer gedacht (`&mut self`) und hat keine eigene Synchronisation.
- Mehrere `IndexBuild` auf derselben `SegmentArena` sind zulässig. Sie sind über die Atomarität der Arena-Methoden geschützt. Jeder Build hält ein eigenes, disjunktes Segment.
- Keine Panics. Alle Fehler sind `IndexBuildError`.

## 5. Relied-on elements

- **`segment_arena`** (unverändert verwenden):
  - `SegmentArena<A: Borrow<Allocator>>::allocate_bump(&self, len: u64) -> Result<BumpSegment, SegmentArenaError>`. Das Ergebnis ist `allocate(len)` plus `BumpSegment::new`. Die Länge wird auf das Alignment aufgerundet, `len == 0` ergibt `ZeroLength`.
  - `SegmentArena::release(&self, seg: &Segment, readers: Readers) -> Result<(), SegmentArenaError>`. Dabei ist `Readers::{None, After(Event)}`.
  - `Segment { region: Region, offset: u64, len: u64 }`, `Copy`. Das Segment ist ausgerichtet, `offset` ist absolut und `len` die nutzbare Länge.
  - `Region { arena: ArenaKind, offset: u64, len: u64, generation: u32 }`.
  - `BumpSegment`: `claim(&mut self, len, align) -> Result<u64, SegmentArenaError>` liefert die absolute, ausgerichtete Adresse (Arena-Offset). Padding zählt als geschrieben. `written()`, `remaining()`, `segment() -> &Segment`, `into_parts() -> (Segment, u64)`.
  - `SegmentArenaError::{Exhausted{requested, available}, ZeroLength, InvalidAlignment, StaleRegion, …}`. Der Typ ist `Clone, Debug, PartialEq, Eq`.
- **`region_handle`**:
  - `RegionHandle::new(arena: ArenaKind, offset: u64, len: u64, generation: Generation) -> Result<RegionHandle, RegionError>`. Fehler `Overflow` bei `offset + len`-Überlauf.
  - `RegionHandle` ist ein `Copy`-Wert, `Send + Sync`. `Generation(pub u64)` wird vom Aufrufer vergeben.
  - `RegionError` ist `Clone + Copy + Debug + PartialEq + Eq`.
- **`ArenaKind::IndexSegments`** ist die Arena der Segment-Arena. `Segment.region.arena` liefert sie.