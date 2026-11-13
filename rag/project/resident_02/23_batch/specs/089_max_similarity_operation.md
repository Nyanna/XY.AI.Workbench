# Spezifikation: Max-Similarity-Operation (Build-Order 89)

## 1. Task, restrictions and boundaries

**Task.** Rust-Modul `max_sim_op`. Es liefert eine einheitliche Operation `MaxSimOp::top_k`. Sie berechnet die Top-k-Treffer je Query über einen Slot-Bereich eines Segments und einen Query-Batch. Die MaxSim-Score eines Entries ist `Σ_{Query-Token q} max_{Doc-Token d} (q·d)`. Es gibt zwei austauschbare Implementierungen hinter dem Trait:
- `TiledMaxSim`: eine Tile-Kette aus Library-Operationen auf CPU oder GPU, mit wählbarer Precision und Tile-Größe.
- `FusedMaxSim`: ein Fused Kernel, bei dem die Zwischenmatrix nie im VRAM liegt.

**Grenzen.**
- Das Modul kennt keine Entry-Identität. Es aggregiert nur über Stride oder Offset-Tabelle (`SegmentDesc`). Slot `i` des Bereichs ist Entry `i` des Segments.
- Es importiert weder `candle_core` noch `cudarc`. Library-Tensoren laufen nur als `LibTensor`. Library-Operationen, die der Library-Adapter nicht als Kernel anbietet, kommen über den Trait `TileOps`. Ihn stellt der Aufrufer oder das Backend bereit.
- Kein Top-k-Algorithmus (nutzt `topk::TopK`), keine Allokation von Device-Blöcken, keine Auflösung von `RegionHandle`, keine Scheduler- oder Event-Logik und kein Kernel-Code.
- Kein `unsafe`. Das Modul hält keinen globalen Zustand.
- `cpu_compute::max_sim` ist die Referenzsemantik (Oracle für Tests). Die Tile-Kette ruft es nicht auf.

## 2. Context

- Der Query-Planer ruft `MaxSimOp::top_k` auf. Er wählt vorher die Implementierung (Tiled oder Fused), die Precision und die Tile-Größe.
- Die Doc-Tokens eines Segments liegen als Library-Tensor `[total_tokens, dim]` vor, den das Memory-Backend per `wrap_region` einhüllt. Byte 0 des Tensors entspricht `SegmentDesc.base`.
- `DocSource::slot_offset` liefert den Treffer-Offset. Das Ergebnis je Query ist eine absteigend sortierte `Vec<Hit>` aus `topk`.
- Verfügbare Precisions je Device-Art kommen aus „Unterstützte Precisions“. CPU hat `[F32]`, GPU hat `[F32, F16]`.
- `FusedMaxSim` läuft über die Kernel-Batchlauf-Schnittstelle (`BatchInterface`, `BatchExecutor`).

## 3. Interface

```rust
use device_lane_event::DeviceHandle;
use kernel_batch::{BatchExecutor, BatchInterface, SegmentDesc};
use layout::DType;
use library_adapter::{AdapterError, LibTensor};
use query_batch::{DocSource, SegmentId, SourceError};
use region_handle::RegionHandle;
use std::{ops::Range, sync::Arc};
use topk::Hit;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MaxSimError {
    /// Precision auf dieser Device-Art nicht abbildbar, oder Device lässt sich nicht öffnen.
    Unavailable(String),
    /// Slot-Bereich ungültig (aus check_slot_range, oder range.end > desc.entries).
    Source(SourceError),
    /// dim == 0
    InvalidDim,
    /// Länge durch dim nicht teilbar. what: "query" | "stride" | "offset".
    Shape { what: &'static str, index: usize, len: u64, dim: usize },
    /// dtype/Form des Doc-Tensors passt nicht (dtype != precision, dims[1] != dim, zu wenige Zeilen, offsets ungültig).
    Mismatch(String),
    /// Fehler der Library (AdapterError) oder des Executors/Fused-Pfads, mit Text.
    Launch(String),
}
// Display + std::error::Error + Send + Sync; From<SourceError>.
// AdapterError -> Launch(e.to_string()).

/// Doc-Seite eines Segments. Die Daten gehören dem Aufrufer.
pub struct DocSegment<'a> {
    pub desc: SegmentDesc,        // Fixed{stride Bytes} oder Variable{..}
    pub offsets: &'a [u64],       // Inhalt der Offset-Tabelle (entries+1, Byte, relativ zu base); bei Fixed ignoriert
    pub tokens: &'a LibTensor,    // [total_tokens, dim], dtype == Precision der Operation
}

pub struct TopKRequest<'a> {
    pub src: &'a dyn DocSource,
    pub segment: SegmentId,
    pub docs: DocSegment<'a>,
    pub range: Range<u32>,        // halboffen, Slots == Entries
    pub queries: &'a [&'a [f32]], // je Query flach tokens*dim, Token-major
    pub dim: usize,
    pub k: usize,
}

/// Einheitliche, austauschbare Operation.
pub trait MaxSimOp: Send + Sync {
    /// Index im Ergebnis = Query-Index; jede Liste höchstens k Treffer, absteigend.
    fn top_k(&self, req: &TopKRequest<'_>) -> Result<Vec<Vec<Hit>>, MaxSimError>;
}

/// Library-Operationen für die Tile-Kette. Wird vom Backend über den Library-Adapter implementiert.
/// Alle Tensoren liegen auf dem geöffneten Device. Fehler sind AdapterError.
pub trait TileOps: Send + Sync {
    /// Host-f32 [rows, cols] -> Tensor in `dtype` (Konvertierung in die Precision).
    fn upload(&self, data: &[f32], rows: usize, cols: usize, dtype: DType) -> Result<LibTensor, AdapterError>;
    /// Zeilen [row_start, row_start+rows) aus docs [N,dim], mit Nullen auf `tile` Zeilen aufgefüllt, transponiert: [dim, tile].
    fn doc_tile_t(&self, docs: &LibTensor, row_start: usize, rows: usize, tile: usize) -> Result<LibTensor, AdapterError>;
    /// Additive Mask [1, tile]: 0 für die ersten `valid` Spalten, MASK_VALUE für den Rest (in dtype).
    fn tile_mask(&self, valid: usize, tile: usize, dtype: DType) -> Result<LibTensor, AdapterError>;
    fn matmul(&self, a: &LibTensor, b: &LibTensor) -> Result<LibTensor, AdapterError>;       // [Q,dim]x[dim,tile] -> [Q,tile]
    fn add_broadcast(&self, scores: &LibTensor, mask: &LibTensor) -> Result<LibTensor, AdapterError>;
    fn max_last_dim(&self, scores: &LibTensor) -> Result<LibTensor, AdapterError>;           // [Q,tile] -> [Q]
    fn maximum(&self, a: &LibTensor, b: &LibTensor) -> Result<LibTensor, AdapterError>;      // elementweise [Q]
    /// Konvertiert nach FP32, summiert, liefert den Host-Wert.
    fn sum_f32(&self, v: &LibTensor) -> Result<f32, AdapterError>;
}

/// Tile-Kette. Öffnet das Device über `open`.
pub struct TiledMaxSim { /* device, precision, tile_size >= 1, ops */ }
impl TiledMaxSim {
    pub fn new(
        device: DeviceHandle,
        precision: DType,
        tile_size: usize,                                         // < 1 => 1
        open: &dyn Fn(DeviceHandle) -> Result<Arc<dyn TileOps>, String>,
    ) -> Result<TiledMaxSim, MaxSimError>;
    pub fn tile_size(&self) -> usize;
    pub fn precision(&self) -> DType;
}
impl MaxSimOp for TiledMaxSim {}

/// Host-seitiger Zugriff des Fused-Pfads auf Query- und Score-Regionen (vom Backend bereitgestellt).
pub trait FusedIo: Send + Sync {
    fn upload_queries(&self, queries: &[&[f32]], dim: usize, precision: DType) -> Result<RegionHandle, String>;
    fn alloc_scores(&self, bytes: u64) -> Result<RegionHandle, String>;
    fn read_scores(&self, region: RegionHandle, f32_count: usize) -> Result<Vec<f32>, String>;
    fn release(&self, region: RegionHandle);
}

/// Fused Kernel hinter derselben Operation.
pub struct FusedMaxSim<E: BatchExecutor> { /* iface, io, precision, params */ }
impl<E: BatchExecutor> FusedMaxSim<E> {
    pub fn new(
        device: DeviceHandle,
        precision: DType,
        iface: Arc<BatchInterface<E>>,
        io: Arc<dyn FusedIo>,
        params: Option<RegionHandle>,
    ) -> Result<Self, MaxSimError>;
}
impl<E: BatchExecutor> MaxSimOp for FusedMaxSim<E> {}
```

`TiledMaxSim`, `FusedMaxSim<E>` (mit `E: BatchExecutor`), `MaxSimError` und `TopKRequest` sind `Send + Sync`. Der Test enthält eine Compile-Time-Assertion dafür.

## 4. Behavior and invariants

**Verfügbarkeit**
- `new` gibt `Unavailable` zurück, wenn `has_scoring_kernel(device.kind(), precision)` false ist. Beispiele sind BF16 auf jedem Device und F16 auf der CPU.
- `new` gibt `Unavailable` zurück, wenn `open(device)` einen Fehler liefert, wenn `precision` nicht auf eine Library-Precision abbildbar ist oder bei einem Fehler von `iface`/`io` beim Anlegen.
- Die Prüfung der Precision kommt vor `open`. Bei `Unavailable` wird kein `top_k` ausgeführt.
- `tile_size < 1` wird zu `1`. `tile_size()` meldet den effektiven Wert.

**Prüfungen in `top_k`**
Beide Implementierungen prüfen in dieser Reihenfolge, die erste Verletzung gewinnt:
1. `dim == 0` ergibt `InvalidDim`.
2. `check_slot_range(src, segment, range)` schlägt fehl: `Source`. Außerdem `range.end > desc.entries`: `Source`/`Mismatch`, ohne Kürzung.
3. Eine Query mit `len % dim != 0` ergibt `Shape{what:"query", index}`.
4. Doc-Tensor: `dtype != precision`, `dims.len() != 2`, `dims[1] != dim` oder `offsets_valid` false (Variable) ergeben `Mismatch`. Stride oder Offset in Bytes nicht durch `dim * size_in_bytes(precision)` teilbar: `Shape{what:"stride"|"offset"}`. Zu wenige Zeilen für die Entries im Bereich: `Mismatch`.
5. Fehler der Library oder des Fused-Pfads ergeben `Launch(text)`. Es gibt kein Teilergebnis.

Alle Prüfungen laufen vor dem ersten Library-Aufruf (auch vor dem Fused-Launch). Bei einem Fehler gibt es kein `Ok`.

**Entry-Grenzen**
- Fixed: Entry `i` hat `stride / (dim*elem)` Tokens ab Zeile `i * tokens_per_entry`. Stride 0 ergibt leere Entries.
- Variable: Entry `i` umfasst die Zeilen `offsets[i]/(dim*elem) .. offsets[i+1]/(dim*elem)`.

**Tile-Kette (je Entry im Bereich)**
- Query-Tensor: `upload(query, q_tokens, dim, precision)`. Die Queries werden einmal pro `top_k` und Query in die Precision überführt, bevor Tiles laufen.
- Doc-Tokens werden in Tiles zu je `tile_size` Zeilen zerlegt und mit `doc_tile_t` transponiert. Das letzte Tile wird mit Nullen aufgefüllt und mit `tile_mask(valid, tile)` additiv ausgeschlossen. Mask-Wert ist `MASK_VALUE` (-10 000), additiv, endlich.
- Je Tile gilt: `matmul` → `add_broadcast` → `max_last_dim` → `maximum` mit dem laufenden Maximum `[Q]` (Start: Ergebnis des ersten Tiles).
- Zum Schluss gilt `sum_f32` über das laufende Maximum. Die Summe läuft immer in FP32, auch bei F16.
- Die Zwischenmatrix hat höchstens `q_tokens × tile_size` Elemente. Es existiert nie eine Matrix `q_tokens × doc_tokens`.
- Ein Entry ohne Tokens ergibt `MASK_VALUE * q_tokens as f32`, ohne Library-Aufruf. Eine Query ohne Tokens ergibt `0.0` (auch bei leerem Dokument).
- Query-Mask-Tokens (Padding-Tokens der Query) werden nicht ausgeschlossen. Sie sind normale Query-Tokens und zählen in der Summe mit. Maskiert wird nur die Dokumentseite.
- Das Ergebnis stimmt für F32 bis auf Rundung mit `cpu_compute::max_sim` überein. Für F16 gilt das mit Precision-Toleranz.
- Die Tile-Größe ändert das Ergebnis nicht (außer bei Rundung). Das gilt auch für `tile_size` größer als die Dokumentlänge.

**Top-k**
- Je Query gibt es ein `TopK(k)`. Der Treffer-Offset ist `src.slot_offset(segment, Slot(range.start + i))`. Bei `None` wird der Slot übersprungen, ohne Panic. NaN-Scores werden verworfen (Verhalten von `TopK`).
- Sortierung und Gleichstand folgen `topk` (Score absteigend, niedrigerer Offset zuerst).
- `k == 0`, ein leerer Bereich (`start == end`) oder ein leerer Query-Batch ergeben `Ok` mit `queries.len()` leeren Listen (bei leerem Batch ein leerer `Vec`). Die Prüfungen 1–4 laufen trotzdem.

**Fused**
- Dieselbe Operation, dieselben Prüfungen, derselbe Ergebnistyp.
- Ablauf: `io.upload_queries`, `io.alloc_scores(queries * slots * 4)`, `iface.begin` (`BatchSpec` mit `segments=[desc]`, `query_count=queries.len()`, `params`, `time_budget` vom Aufrufer-Default des Backends) mit `BatchMode::Normal`, `assign_ranges([scores])`, `run` mit `inputs.regions=[queries_region]`, `end`, `io.read_scores`, `release` aller Regionen. Die Regionen werden auch im Fehlerfall freigegeben und der Batch wird beendet.
- Die Scores stehen query-major f32 (`q * slots + i`). Danach folgt dasselbe Top-k wie oben.
- Ein `Partial`-Ergebnis, ein Fehler oder `BatchError` ergibt `Launch`. Es gibt keine Wiederaufnahme.
- Die Zwischenmatrix `Q×D` liegt nie im VRAM. Das garantiert der Kernel des Executors. Dieses Modul legt keine Score-Matrix an.
- Sind die Queries nicht leer, erzeugt der Fused-Pfad pro `top_k` genau einen Batch.

**Concurrency**
- `top_k` nimmt `&self` und hält keinen veränderlichen Zustand. Mehrere Threads dürfen gleichzeitig auf derselben Instanz aufrufen.
- Der Tile-Pfad ist synchron. Das Ergebnis liegt bei der Rückkehr auf dem Host. Das Modul setzt oder wartet auf keine Events.
- Es gibt keine Panics bei gültigen oder ungültigen Eingaben. Alle Fehler sind `MaxSimError`.

## 5. Relied-on elements

- `unterstützte Precisions`: `has_scoring_kernel(kind: DeviceKind, dtype: DType) -> bool`. CPU: `[F32]`, GPU: `[F32, F16]`, BF16 nie.
- `Device`: `DeviceHandle::{Cpu, Gpu(u32)}` mit `kind() -> DeviceKind`.
- `library_adapter`: `LibTensor` (`dims() -> &[usize]`, `dtype() -> Result<DType, AdapterError>`, `Send + Sync`). `AdapterError: Display`. `to_lib_dtype`. Die Kernel `matmul`, `add` (arity 2) sind Teil der Library-Quelle. Alles darüber hinaus liefert `TileOps`.
- `layout`: `DType {F32,F16,BF16,U8,U32,I64}` mit `size_in_bytes()`. `MASK_VALUE: f32 = -10_000.0`.
- `topk`: `TopK::{new(k), push(offset: u64, score: f32) -> bool, into_sorted_vec() -> Vec<Hit>}`, `Hit{offset: u64, score: f32}`.
- `query_batch`: `DocSource::slot_offset(&self, SegmentId, Slot) -> Option<u64>`. `check_slot_range(src, segment, Range<u32>) -> Result<(), SourceError>` (Ok genau dann, wenn `start <= end <= slot_count`). `SegmentId(pub u64)`. `kernel_layout::Slot(pub u32)`.
- `kernel_batch`: `SegmentDesc{base, entries, layout: SegmentLayout::{Fixed{stride}, Variable{offsets}}}` mit `offsets_valid(&[u64])`. `BatchSpec{segments, query_count, params, time_budget}`. `BatchInterface::{begin, assign_ranges, run, end}`. `LaunchOutcome::{Hits, Partial}`. `BatchMode::Normal`. `BatchInputs{regions}`. `BatchExecutor: Send + Sync`.
- `cpu_compute::max_sim(query, doc, dim)`: Referenz. Leeres Dokument ergibt `MASK_VALUE * q_tokens`, leere Query ergibt `0.0`.