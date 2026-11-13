# Spezifikation: Embedding-Batch-Ausführung (Build-Order 133)

## 1. Task, restrictions and boundaries

**Task.** Rust-Modul `embedding_batch_exec`. Es enthält eine einzelne synchrone Funktion `execute_embedding_batch`. Sie führt einen bereits begonnenen Batch per Resume bis zu den Treffern aus:

- Je Schritt reserviert sie einen Ausgabebereich aus der `SegmentArena` und weist ihn dem Batch zu.
- Sie ruft `run` auf. Meldet der Kernel `Partial`, zählt sie die Teilantwort und setzt den Input hinter den verarbeiteten Bytes fort.
- Sie beendet den Batch immer. Bei einem Fehler gibt sie alle reservierten Bereiche frei.

**Nicht Teil des Moduls:**

- `begin` und `BatchSpec`: Der Aufrufer begibt den Batch.
- Kernel-Start, Event, Doorbell und Auflösung von Handles auf Adressen.
- Das Schreiben von Embeddings und die Verwaltung des Write-Pointers.
- Die Freigabe der Bereiche nach Erfolg. Sie bleiben beim Aufrufer.
- Retry und Nebenläufigkeit. Es gibt keinen globalen Zustand und keine eigenen Locks.
- Keine Redefinition von `BatchInterface`, `SegmentArena`, `RegionHandle` oder `WriteResult`.
- Kein `unsafe`, keine Panics.

## 2. Context

- `BatchInterface<E>` (`kernel_batch`) hat den Ablauf Begin → assign_ranges → run → (bei Partial: assign_ranges → run)* → end.
  - `run` ohne unbenutzten Bereich ergibt `NeedsRanges`.
  - `end` liefert `Vec<UsedRange>`.
- `SegmentArena` liefert ausgerichtete `Segment`s. Sie sind der Speicher der Bereiche. Ihre Höchstzahl begrenzt dieses Modul selbst. Die Freigabe läuft über `release`.
- Der Write-Pointer-Bereich (`WritePointerRegion`) liegt im Backend je Bereich. Seine `WriteResult` sagt, wie viel des Bereichs gültig ist. Das Modul bekommt sie über das Trait `StepProgress`.
- Der Aufrufer (Query-Planer) hält danach die `Segment`s, um sie zu lesen und später freizugeben.

## 3. Interface

```rust
use bookkeeping::Region;
use kernel_batch::{BatchError, BatchExecutor, BatchId, BatchInterface, UsedRange};
use region_handle::{RegionError, RegionHandle};
use segment_arena::{Segment, SegmentArena, SegmentArenaError};
use write_pointer_region::WriteResult;
use std::borrow::Borrow;
use allocator::Allocator;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ExecConfig {
    /// Länge jedes Schritt-Bereichs in Bytes (> 0).
    pub range_len: u64,
    /// Höchstzahl reservierter Bereiche je Ausführung (> 0).
    pub max_ranges: usize,
}

/// Vom Backend implementiert: Write-Pointer-Ergebnis des Bereichs, den der Schritt gefüllt hat.
pub trait StepProgress: Send + Sync {
    fn write_result(&self, range: RegionHandle) -> Result<WriteResult, String>;
}

#[derive(Clone, Copy, Debug)]
pub struct BatchInput { pub region: RegionHandle }   // fortsetzbarer Input

#[derive(Debug)]
pub struct BatchSuccess {
    /// Aus `end`, in Zuteilungsreihenfolge (ein Eintrag je Schritt).
    pub ranges: Vec<UsedRange>,
    /// Reservierte Segmente in Zuteilungsreihenfolge; Eigentümer ist nun der Aufrufer.
    pub regions: Vec<Segment>,
    /// Anzahl der Partial-Antworten (kein Fehler).
    pub partials: u32,
    /// `hits` der abschließenden `Hits`-Antwort.
    pub hits: u64,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ExecError {
    /// Display: "Arena erschöpft". Vor einem Schritt war `max_ranges` bereits erreicht.
    ArenaExhausted { max_ranges: usize },
    InvalidConfig,                         // range_len == 0 oder max_ranges == 0
    Arena(SegmentArenaError),              // reserve fehlgeschlagen (Display unverändert, z. B. "Arena erschöpft")
    Region(RegionError),                   // Handle-Bildung fehlgeschlagen
    Batch(BatchError),                     // assign_ranges, run oder end fehlgeschlagen
    Progress(String),                      // StepProgress-Fehler
    ProgressOutOfRange { processed: u64, input_len: u64 },
}
impl std::fmt::Display for ExecError {}
impl std::error::Error for ExecError {}

pub fn execute_embedding_batch<E, A>(
    batch: &BatchInterface<E>,
    arena: &SegmentArena<A>,
    progress: &dyn StepProgress,
    id: BatchId,
    input: BatchInput,
    config: &ExecConfig,
) -> Result<BatchSuccess, ExecError>
where
    E: BatchExecutor,
    A: Borrow<Allocator>;
```

`ExecConfig`, `BatchInput`, `BatchSuccess` und `ExecError` sind `Send + Sync + 'static`.

## 4. Behavior and invariants

**Ablauf.** Ist die Konfiguration gültig, läuft die Funktion so:

1. Sie hält `segments: Vec<Segment>`, `partials = 0` und den aktuellen Input (zu Beginn `input.region`).
2. Jeder Schritt läuft so:
   1. Ist `segments.len() >= max_ranges`, folgt `ArenaExhausted`. Diese Prüfung kommt vor jeder Reservierung, auch vor dem ersten Schritt.
   2. `arena.allocate(range_len)`. Das Segment wird sofort in `segments` aufgenommen.
   3. Das Handle des Bereichs entsteht mit `RegionHandle::new(region.arena, seg.offset, seg.len, Generation(region.generation as u64))`. `region` ist `seg.region`. Handle-Offset und -Länge sind also die ausgerichteten Werte des Segments.
   4. `batch.assign_ranges(id, &[handle])`.
   5. `batch.run(id, &BatchInputs { regions: vec![current_input] })`.
   6. Bei `Hits { hits }` endet die Schleife.
   7. Bei `Partial` gilt:
      - `partials += 1`.
      - `processed = progress.write_result(handle)?.valid_len()`.
      - Ist `processed > current_input.len()`, folgt `ProgressOutOfRange`.
      - Sonst ist der neue Input `RegionHandle::new(arena, offset + processed, len - processed, generation)` auf Basis des aktuellen Inputs. Es folgt der nächste Schritt.
3. Danach ruft die Funktion `batch.end(id)` auf. Bei Erfolg liefert sie `BatchSuccess { ranges, regions: segments, partials, hits }`.

**Ungültige Konfiguration.** Bei `range_len == 0` oder `max_ranges == 0` kommt `InvalidConfig`.
- Es wird nichts reserviert.
- `end` wird trotzdem aufgerufen (siehe „Immer beenden“).

**Teilantworten.** `Partial` ist nie ein Fehler. Es gibt `partials` Teilantworten und genau `partials + 1` Schritte und Bereiche (`regions.len() == ranges.len() == partials + 1`).

**Immer beenden.** Jeder Pfad nach Funktionseintritt ruft genau einmal `batch.end(id)` auf: Erfolg, jeder Fehler, auch `ArenaExhausted` und `InvalidConfig`.

**Fehler.**
- Ein Fehler aus Schritt 2 (alle Teilschritte, inklusive `run`) beendet die Schleife sofort. Es folgt `end`, danach die Freigabe.
- Alle reservierten Segmente werden mit `arena.release(&seg, Readers::None)` freigegeben, in Reservierungsreihenfolge. Das geschieht nach `end`. Ein Freigabefehler wird nur mit `log::warn!` geloggt und ändert nichts am gemeldeten Fehler.
- Gemeldet wird immer der erste aufgetretene Fehler, also bei einem Lauffehler `Batch(BatchError::Executor(..))` und nicht ein Folgefehler aus `end` oder `release`.
- Ein Fehler aus dem abschließenden `end` nach erfolgreicher Schleife wird als `Batch(e)` gemeldet. Die Segmente werden dann ebenfalls freigegeben.
- Nach einem Fehler ist keine Reservierung dieses Aufrufs mehr offen.
- Bei Erfolg gibt das Modul nichts frei.

**Weitere Regeln.**
- `ExecError::ArenaExhausted` zeigt „Arena erschöpft“.
- `Arena(SegmentArenaError::Exhausted{..})` zeigt die Anzeige des Arena-Moduls, ebenfalls „Arena erschöpft“.
- Die Funktion blockiert nur so lange, wie `run` dauert. Sie hält selbst keine Locks.
- Sie ist für verschiedene Batch-IDs aus mehreren Threads aufrufbar. Dieselbe ID gleichzeitig führt zu `Busy` aus `BatchInterface`, und das wird als `Batch(Busy)` gemeldet.

## 5. Relied-on elements

**`kernel_batch`** (unverändert):
```rust
pub struct BatchId(pub u64);
pub struct BatchInputs { pub regions: Vec<RegionHandle> }
pub struct UsedRange { pub region: RegionHandle, pub used: u64 }
pub enum LaunchOutcome { Hits { hits: u64 }, Partial { queries_done: u32, hits: u64 } }  // RunOutcome = LaunchOutcome
pub enum BatchError { UnknownBatch(BatchId), InvalidSpec(String), Busy, NeedsRanges, AlreadyFinished, Executor(String) }
impl<E: BatchExecutor> BatchInterface<E> {
    pub fn assign_ranges(&self, id: BatchId, ranges: &[RegionHandle]) -> Result<(), BatchError>;
    pub fn run(&self, id: BatchId, inputs: &BatchInputs) -> Result<RunOutcome, BatchError>;
    pub fn end(&self, id: BatchId) -> Result<Vec<UsedRange>, BatchError>;  // gibt Bereiche in Zuteilungsreihenfolge zurück
}
```
Dabei gilt:
- `run` ohne unbenutzten Bereich ergibt `NeedsRanges`.
- `end` ist auch nach einem Fehler erlaubt.
- Ein Executor-Fehler lässt den Batch bestehen.
- Nach `end` ergibt jede Operation `UnknownBatch`.

**`segment_arena`**:
```rust
pub struct Segment { pub region: Region, pub offset: u64, pub len: u64 }   // offset: ausgerichtet, len: nutzbare Länge
pub enum Readers { None, After(Event) }
impl<A: Borrow<Allocator>> SegmentArena<A> {
    pub fn allocate(&self, len: u64) -> Result<Segment, SegmentArenaError>;
    pub fn release(&self, seg: &Segment, readers: Readers) -> Result<(), SegmentArenaError>;
}
```
`SegmentArenaError::Exhausted` zeigt „Arena erschöpft“. `Readers::None` gibt sofort frei.

**`bookkeeping::Region`**: `{ arena: ArenaKind, offset: u64, len: u64, generation: u32 }`, `Copy`.

**`region_handle`**:
```rust
pub struct Generation(pub u64);
impl RegionHandle {
    pub fn new(arena: ArenaKind, offset: ArenaOffset, len: u64, generation: Generation) -> Result<RegionHandle, RegionError>;
    pub fn arena(&self) -> ArenaKind;
    pub fn offset(&self) -> u64;
    pub fn len(&self) -> u64;
    pub fn generation(&self) -> Generation;
}
```
`RegionHandle` ist `Copy + Send + Sync`.

**`write_pointer_region`**: `WriteResult::{Complete{region,len}, Partial{region,valid_len,overflow_pos}}` mit `fn valid_len(&self) -> u64`. Bei `Partial` sind die Bytes `[0, valid_len)` gültig.

**Logging**: `log`-Crate (`warn!`).