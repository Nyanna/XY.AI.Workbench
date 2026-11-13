# Spezifikation: Kernel und Scoring (Modul `kernel_scoring`, Build-Order 105)

## 1. Task, restrictions and boundaries

**Task.** Das Modul berechnet auf Anforderung des Managers Scores von einem Query-Vektor gegen Slots (Zeilen) in Segmenten (Device-Regionen). Es liefert pro Slot nur Adresse und Score. Es stellt außerdem seine Score-Kernel-Quellen für das Kernel-Register bereit und holt die Kernel bei Bedarf von dort.

**Grenzen.**
- Das Modul kennt nur Segmente, Slots, Adressen und Offsets. Es kennt keine Entry-Identität, keinen Index, keine Scopes, keine Subengines, keine Modelle und keine Traversierung. Es traversiert nie.
- Es wählt keine Top-k-Slots, sortiert nicht, filtert nicht und plant nicht. Das bleibt beim Manager.
- Es allokiert keine Segmente, löst keine `RegionHandle` selbst auf und prüft keine Generation. Der Manager bzw. das Memory-Backend liefert den `DeviceBlock` und hat die Gültigkeit geprüft.
- Kein `unsafe`. Kein Import von `cudarc` oder `candle_core`. Der Kernel-Start, der Query-Upload, der Ausgabepuffer und das Zurücklesen liegen hinter den Traits `ScoreLauncher` und `ScoreReadback`, die das GPU-Binding implementiert. Das Modul ist dadurch mit Mocks ohne GPU testbar.
- Der Kernel-Start läuft nur über `Kernel::Gpu`. Es gibt keinen Library-Kernel-Pfad und keinen CPU-Pfad. Auf `DeviceHandle::Cpu` ergibt die Anfrage `Kernel(Unavailable)`.
- Während der Berechnung gibt es keine Kommunikation: keine Callbacks, keine Channels, keine Fortschrittsmeldungen, kein Logging im Launch-Pfad. Das Ergebnis wird nur über `Event` und `finish` abgeholt.

## 2. Context

- **Manager.** Er ruft `Scorer::score` mit Segmenten, Slot-Bereichen, Metrik und Query auf. Er wartet auf das `Event` des Jobs und holt `Vec<ScoredSlot>` ab. Er bildet Adressen selbst auf Entries ab. Dieses Modul sieht diese Abbildung nicht.
- **Kernel-Register.** `register_score_kernels` trägt die JIT-Quellen `score_dot` und `score_negl2` ein. `Scorer` holt per `request(name, dtype, Variant{dim: Some(dim), tile: None})` die Funktion. Das Register cacht und synchronisiert selbst.
- **GPU-Binding.** Es implementiert `ScoreLauncher` und `ScoreReadback`.
- **Lane.** Die Arbeit läuft auf der übergebenen Lane. Deren `record_event` liefert das Fertig-Event. Die Lane-Art wird nicht erzwungen.
- **Region-Handle, Layout, Library-Adapter.** Sie liefern Segmentgeometrie (`RegionHandle`, `Layout`) und Blockadresse (`DeviceBlock`). Das Modul liest nur und löst keine Handles auf.
- Varianz und Indeterminismus der Scores sind erlaubt (Fusion, Atomics, hardwareabhängiges Rounding). Es gibt keine Bit-Reproduzierbarkeit.

## 3. Interface

```rust
use std::ops::Range;
use std::sync::Arc;
use arenas::{ArenaKind, ArenaOffset};
use layout::{DType, Layout};
use region_handle::RegionHandle;
use library_adapter::DeviceBlock;
use device_lane_event::{Event, Lane, LaneError};
use kernel_register::{KernelRegister, KernelSource, KernelError, KernelFunction, Variant};

pub const KERNEL_SCORE_DOT: &str = "score_dot";
pub const KERNEL_SCORE_NEGL2: &str = "score_negl2";

/// Höherer Score = besser (bei beiden Metriken).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Metric { Dot, NegL2Sq }
impl Metric { pub fn kernel_name(self) -> &'static str; }

/// Segment: Device-Region mit Zeilenlayout [rows, dim] (rank 2, contiguous, row-major).
#[derive(Debug, Clone)]
pub struct ScoreSegment { pub block: DeviceBlock, pub region: RegionHandle, pub layout: Layout }

/// Slots (Zeilenindizes) eines Segments, die bewertet werden.
#[derive(Debug, Clone)]
pub struct SegmentSlots { pub segment: ScoreSegment, pub slots: Range<u32> }

#[derive(Debug, Clone)]
pub struct ScoreRequest { pub metric: Metric, pub query: Vec<f32>, pub segments: Vec<SegmentSlots> }

/// Adresse eines Slots: Arena und Byte-Offset der Zeile in der Arena.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SlotAddr { /* privat */ }
impl SlotAddr {
    pub fn new(arena: ArenaKind, offset: ArenaOffset) -> SlotAddr;
    pub fn arena(&self) -> ArenaKind;
    pub fn offset(&self) -> ArenaOffset;   // region.offset + slot * dim * elem_size
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ScoredSlot { pub addr: SlotAddr, pub score: f32 }

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScoreError {
    UnsupportedDType(DType),
    QueryDimMismatch { query: usize, dim: usize },
    BadLayout(String),
    OutOfBounds { slots: Range<u32>, rows: usize },
    Kernel(KernelError),
    Launch(String),
    Readback(String),
    Lane(LaneError),
}
impl std::fmt::Display for ScoreError {}
impl std::error::Error for ScoreError {}

/// Parameter für den Kernel-Start. Segmente sind bereits auf Adressen aufgelöst.
pub struct LaunchSegment<'a> {
    pub block: &'a DeviceBlock,
    pub first_row_addr: u64,   // Device-Adresse der ersten bewerteten Zeile
    pub rows: u32,
}
pub struct LaunchRequest<'a> {
    pub metric: Metric,
    pub dtype: DType,          // F32 oder F16
    pub dim: u32,
    pub query: &'a [f32],      // Binding lädt hoch und wandelt bei Bedarf in dtype
    pub segments: &'a [LaunchSegment<'a>],
}

/// Wird vom GPU-Binding implementiert.
pub trait ScoreLauncher: Send + Sync {
    /// Reiht Upload, Kernel und Ausgabepuffer asynchron in die Lane ein, ohne zu blockieren.
    /// Ausgabe: ein f32-Score pro Zeile, konkateniert in Segment- und Zeilenreihenfolge.
    fn launch(&self, function: &Arc<dyn KernelFunction>, lane: &Lane, req: &LaunchRequest<'_>)
        -> Result<Box<dyn ScoreReadback>, String>;
}
pub trait ScoreReadback: Send + Sync {
    /// Wird erst nach Event-Abschluss aufgerufen.
    fn read(&self) -> Result<Vec<f32>, String>;
}

/// Trägt die JIT-Quellen `score_dot` und `score_negl2` ins Register ein.
/// Dtypes F32 und F16, nutzt `DIM` aus Variant. Ein zweiter Aufruf auf demselben Register
/// ergibt den Fehler des Registers (doppelter Name).
pub fn register_score_kernels(register: &KernelRegister) -> Result<(), KernelError>;

pub struct ScoreJob { /* Event, Readback, Adressliste */ }   // Send
impl ScoreJob {
    pub fn event(&self) -> &Event;
    pub fn is_complete(&self) -> bool;     // nicht blockierend; Event-Fehler zählen als complete
    /// Wartet auf das Event, liest zurück und liefert die Slots in Reihenfolge.
    pub fn finish(self) -> Result<Vec<ScoredSlot>, ScoreError>;
}

pub struct Scorer { /* Arc<KernelRegister>, Arc<dyn ScoreLauncher> */ }   // Send + Sync
impl Scorer {
    pub fn new(register: Arc<KernelRegister>, launcher: Arc<dyn ScoreLauncher>) -> Scorer;
    /// Nicht blockierend: validiert, holt den Kernel, startet auf `lane` und zeichnet das Event auf.
    pub fn score(&self, lane: &Lane, req: &ScoreRequest) -> Result<ScoreJob, ScoreError>;
}
```

## 4. Behavior and invariants

**Scores.**
- `Dot` = Σ qᵢ·xᵢ. `NegL2Sq` = −Σ (qᵢ−xᵢ)². Die Akkumulation erfolgt in f32, F16-Elemente werden vorher nach f32 gewandelt.
- NaN und Inf werden unverändert durchgereicht. Es gibt weder Maskierung noch Clamping.
- Scores gelten nur innerhalb einer Toleranz, nicht bitgenau. Zwei Läufe mit gleicher Eingabe dürfen sich unterscheiden.

**Ergebnis.**
- `finish` liefert genau einen `ScoredSlot` pro angefragtem Slot, in Reihenfolge der Segmente und innerhalb eines Segments mit aufsteigendem Slot. Es gibt keine Sortierung nach Score und keine Auswahl.
- `addr.arena() == region.arena()`.
- `addr.offset() == region.offset() + slot * dim * dtype.size_in_bytes()`.
- Die Ausgabe enthält nur Adresse und Score, keine weitere Identität.

**Validierung in `score`.** Die Prüfungen laufen in dieser Reihenfolge, der erste Fehler gewinnt. Es wird weder gestartet noch ein Kernel geholt, solange eine Prüfung fehlschlägt.
1. Für jedes Segment gilt: `layout.dtype()` ist F32 oder F16, sonst `UnsupportedDType` (auch BF16). Alle Segmente haben denselben dtype, sonst `BadLayout`.
2. Das Layout hat Rank 2 und ist contiguous, sonst `BadLayout`.
3. `layout.shape()[1] == query.len()` für jedes Segment, sonst `QueryDimMismatch`. Eine Dimension 0 ist `BadLayout`.
4. `layout.byte_len() <= region.len()` und `region.end() <= block.len_bytes`, sonst `BadLayout`.
5. Die Device-Adresse `block.base_addr + region.offset()` ist ein Vielfaches der Elementgröße. Die Addition ist überlaufsicher. Sonst `BadLayout`.
6. `slots.start <= slots.end <= shape[0]`, sonst `OutOfBounds`.
7. Kernel holen: `register.request(metric.kernel_name(), dtype, Variant{dim: Some(dim), tile: None})`. Fehler werden als `Kernel(e)` weitergereicht. Dabei ist `Kernel::Library` ein Fehler (`Kernel(Unavailable)`).
8. `launcher.launch(...)`. Ein Fehler wird zu `Launch(text)`.
9. `lane.record_event()`. Ein Fehler wird zu `Lane(e)`.

**Sonderfälle.**
- Leere Slot-Bereiche werden übersprungen. Ein Segment mit 0 Slots erzeugt keinen `LaunchSegment`.
- Sind alle Bereiche leer (auch bei leerer `segments`-Liste), gibt es keinen Kernel-Request und keinen Launch. `score` liefert dann einen bereits abgeschlossenen Job (`Event::cpu()` sofort signalisiert), und `finish` gibt `Ok(vec![])` zurück. Die Validierung 1–6 läuft trotzdem.
- Ein leerer Query ist nur gültig, wenn alle Segmente `dim == 0` hätten. Das ist nach Schritt 3 `BadLayout`. Praktisch wird ein leerer Query abgelehnt.

**`finish`.**
- Es wartet per `event.wait()`. Ein `EventError` wird zu `Lane(LaneError::Record(e))`.
- Danach ruft es `readback.read()`. Ein Fehler wird zu `Readback(text)`. Eine Länge ≠ Gesamtzahl der Slots ergibt `Readback("length")`.
- Es gibt keine Panics bei gültigen Eingaben.

**Concurrency.**
- `Scorer` ist zustandslos (nur `Arc`s). `score` darf von mehreren Threads gleichzeitig mit verschiedenen Lanes aufgerufen werden.
- Dieselbe `Lane` wird nur gelesen (`&Lane`). Die Reihenfolge der Arbeit auf einer Lane bestimmt die Stream-Ordnung.
- Das Modul serialisiert Zugriffe auf Segmente nicht. Ein gleichzeitiges Schreiben in ein Segment während eines Jobs ist Sache des Managers.
- Der Aufrufer hält `DeviceBlock` und Segmentspeicher mindestens bis zum Abschluss des Job-Events am Leben.
- `ScoreJob` kann an einen anderen Thread übergeben werden (`Send`).

**Kernel-Quellen (`register_score_kernels`).**
- Genau zwei Quellen:

| `name` | `entry` | `dtypes` | `precompiled` |
|---|---|---|---|
| `score_dot` | vom Implementierer gewählt, `extern "C" __global__` | `[F32, F16]` | `None` |
| `score_negl2` | vom Implementierer gewählt, `extern "C" __global__` | `[F32, F16]` | `None` |

- Der Body nutzt nur das Prelude des Registers: `T`, `load_f`, `warp_reduce_sum` und `DIM`. Er verwendet kein `TILE`.
- Er verwendet keinen BF16-, Tensor-Core- oder Architektur-Sonderpfad.
- Der Kernel-Body kennt weder Entry noch Index.

## 5. Relied-on elements

- `device_lane_event`:
  - `Lane::record_event(&self) -> Result<Event, LaneError>` zeichnet ein Event über alle bisherige Arbeit der Lane auf. Eine CPU-Lane ohne Stream liefert `Err(NoStreamBackend)`.
  - `Event::cpu() -> (Event, EventSignal)`, `EventSignal::signal(&self)` (idempotent).
  - `Event::is_complete(&self) -> Result<bool, EventError>` (nicht blockierend) und `Event::wait(&self) -> Result<(), EventError>` (blockierend).
  - `Event: Clone + Send + Sync`. `LaneError::{NoStreamBackend, Record(EventError)}` mit `Clone`-fähigem Fehlerinhalt (`PartialEq, Eq`).
  - `Lane: Send + Sync`.
- `region_handle::RegionHandle` (`Copy`): `arena() -> ArenaKind`, `offset() -> u64`, `len() -> u64` (Byte), `end() -> u64` (exklusiv, überlauffrei). Es enthält keine Device-Adresse.
- `arenas`: `ArenaKind` (`Copy, Eq, Hash`), `ArenaOffset = u64`.
- `layout`: `DType {F32,F16,BF16,U8,U32,I64}` mit `size_in_bytes()`. `Layout::{shape() -> &[usize], dtype(), rank(), byte_len() -> usize, is_contiguous()}` mit Byte-Strides. `Layout` trägt keinen Offset.
- `library_adapter::DeviceBlock { device, base_addr: u64, len_bytes: u64 }` (`Clone, Debug, Send + Sync`). Das Modul liest nur `base_addr` und `len_bytes` und nutzt weder `wrap_region` noch `LibTensor`.
- `kernel_register`:
  - `KernelRegister::request(&self, name: &str, dtype: DType, variant: Variant) -> Result<Kernel, KernelError>`.
  - `KernelRegister::register_source(&self, KernelSource) -> Result<(), KernelError>`.
  - `Kernel::{Gpu(Arc<GpuKernel>), Library(LibKernelEntry)}`.
  - `GpuKernel { key, function: Arc<dyn KernelFunction>, image_kind }`.
  - `Variant { dim: Option<u32>, tile: Option<u32> }`.
  - `KernelSource { name, body, entry, dtypes, precompiled }`.
  - `KernelError` (`Clone, PartialEq, Eq`) mit `Unavailable`, `UnknownKernel`, `CompileError`, `ModuleLoadError` und `MissingFunction`.
  - `KernelRegister: Send + Sync`.
  - Der JIT-Quelltext besteht aus dem Prelude plus `body`. Das Prelude definiert `T`, `load_f`, `warp_reduce_sum`, `warp_reduce_max` und `DIM`/`TILE`, nur wenn die Variante sie setzt. JIT-Anfragen mit BF16 oder einem nicht in `dtypes` enthaltenen Typ ergeben `Unavailable`.