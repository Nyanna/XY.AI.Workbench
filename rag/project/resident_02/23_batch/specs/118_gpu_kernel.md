# Spezifikation: GPU-Kernel (Modul `gpu_kernel`, Build-Order 118)

## 1. Task, restrictions and boundaries

**Task.** Das Modul stellt zwei GPU-Kernel bereit. Der Host-Launcher startet sie mit festen Blockparametern und meldet Launch-Fehler mit Ursache.
- `dense_dot`: dichtes Dot-Product, Output Queries × Slots.
- `fused_maxsim`: fused Max-Similarity für genau eine Query, die Zwischenmatrix liegt nie im VRAM.
- Die Dimension `DIM` steht zur Kompilierzeit fest. Sie ist ein const-generic Parameter in Rust und `Variant{dim: Some(DIM)}` im JIT-Quelltext.
- Der Kernel publiziert die Zahl abgeschlossener Batches mit Release-Semantik. Der Host prüft vor jedem weiteren Batch, ob das Ack den Vorgänger abdeckt.

**Grenzen.**
- Nicht Teil des Moduls: Top-k, Slot-Verwaltung, Allokation von Device-Speicher, Auflösung von `RegionHandle`, Doorbell-Polling (`Doorbell::poll/close`), Scheduler und Retry.
- Kein Zugriff auf `cudarc` oder `candle_core`. Der eigentliche Treiber-Start läuft hinter dem Trait `GpuLauncher`, den das GPU-Binding implementiert. Dadurch ist das Modul mit Mocks ohne GPU testbar.
- Dynamischer Shared Memory wird nie verwendet. Kernel und Launcher kennen keinen BF16-, Tensor-Core- oder Architektur-Sonderpfad.
- Varianz und Indeterminismus (Fusion, Atomics, Rounding) sind erlaubt. Es gibt keine Bit-Reproduzierbarkeit.
- Das dichte Scoring (`dense_dot`) ist vom Launch-Modus getrennt.
  - Es hat eigene Quelle, eigenen Entry und eigene Launch-Methode.
  - Es hat keinen Mode-Parameter und liest keine Persistent- oder Normal-Zustände.

## 2. Context

- **Aufrufer.** Scheduler und Max-Similarity-Operation (`FusedMaxSim` über `BatchExecutor`) rufen diese Methoden auf. Sie füllen `KernelParams` und besitzen Segmente, Slot-Ring, Query- und Output-Speicher.
- **Kernel-Register (`kernel_register`).** Die JIT-Quellen werden dort registriert und per `request(name, dtype, Variant{dim: Some(DIM), tile: None})` geholt. Der Body nutzt nur das Prelude (`T`, `load_f`, `warp_reduce_sum`, `warp_reduce_max`, `DIM`) und kein `TILE`.
- **Lane.** Die Lane liefert den Parameter-Block (`params_mut`, fest 256 Byte, null-initialisiert). Sie liefert auch `record_event` für das Fertig-Event.
- **Doorbell.** Das Device inkrementiert die Zelle `completed` (u64, monoton ab 0). Der Host liest sie per `Doorbell::poll` und schreibt `acked` per `Gap::close`. Dieses Modul bekommt den aktuellen `acked`-Wert vom Aufrufer (`Doorbell::acked()`).
- **Precisions.** Dokumente liegen in F32 oder F16. F16 wird beim Laden auf F32 gewandelt. Queries und Ergebnisse sind immer F32.

## 3. Interface

```rust
use device_lane_event::{DeviceHandle, Event, Lane, LaneError};
use kernel_register::{KernelRegister, KernelError, KernelFunction};
use layout::DType;
use std::sync::{Arc, Mutex};

pub const BLOCK_THREADS: u32 = 128;
pub const KERNEL_DENSE_DOT: &str = "dense_dot";
pub const KERNEL_FUSED_MAXSIM: &str = "fused_maxsim";
pub const NO_VALID_TOKEN_SCORE: f32 = -10_000.0;   // == layout::MASK_VALUE

/// Trägt die JIT-Quellen `dense_dot` und `fused_maxsim` (dtypes [F32, F16], precompiled None) ein.
/// Zweiter Aufruf auf demselben Register: Fehler des Registers (doppelter Name).
pub fn register_gpu_kernels(register: &KernelRegister) -> Result<(), KernelError>;

/// Segmenttabellen-Eintrag im Device-Speicher (repr(C), 40 Byte, 8-aligned).
#[repr(C)] #[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SegmentEntry {
    pub base: u64,          // Device-Adresse Slot 0 des Segments
    pub lens_addr: u64,     // Device-Adresse u32[slot_count] Token-Längen; 0 = keine Liste (alle pad_len)
    pub stride_bytes: u64,  // Bytes je Slot (Padding-Länge * DIM * elem_size; dense: DIM * elem_size)
    pub first_slot: u32,    // globaler Slot-Index des ersten Slots
    pub slot_count: u32,
    pub pad_len: u32,       // Padding-Länge in Tokens (nur fused_maxsim)
    pub _reserved: u32,     // 0
}

/// Parameter-Block (repr(C), 8-aligned, 72 Byte, <= 256). Wird ab Byte 0 in `lane.params_mut()`
/// geschrieben und dem Kernel als einziges Argument per Wert übergeben.
#[repr(C)] #[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct KernelParams {
    pub slots_addr: u64,       // 0: u32[ring_cap] Slot-Ring (globale Slot-Indizes)
    pub segments_addr: u64,    // 8: SegmentEntry[segment_count]
    pub query_addr: u64,       // 16: f32, query_len * DIM (dense: Vektoren; maxsim: Tokens), row-major
    pub out_addr: u64,         // 24: f32-Output
    pub completed_addr: u64,   // 32: Doorbell-Zelle `completed` (u64)
    pub block_ctr_addr: u64,   // 40: u32-Block-Zähler, zu Beginn 0; der letzte Block setzt ihn auf 0 zurück
    pub ring_cap: u32,         // 48: > 0
    pub head: u32,             // 52: monotoner Zähler, erster zu verarbeitender Eintrag
    pub tail: u32,             // 56: monotoner Zähler, hinter dem letzten Eintrag
    pub segment_count: u32,    // 60
    pub query_len: u32,        // 64: dense: Anzahl Query-Vektoren; maxsim: Query-Tokens; >= 1
    pub _reserved: u32,        // 68: 0
}
impl KernelParams { pub fn slot_total(&self) -> u32; }  // tail.wrapping_sub(head)

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LaunchConfig { pub grid: u32, pub block: u32 /* == 128 */, pub dynamic_shared: u32 /* == 0 */ }

/// Vom GPU-Binding implementiert. Reiht den Start asynchron in die Stream-Ordnung der Lane ein.
pub trait GpuLauncher: Send + Sync {
    fn launch(&self, function: &Arc<dyn KernelFunction>, lane: &Lane,
              cfg: &LaunchConfig, params: &[u8]) -> Result<(), String>;   // Err = Ursache (Treibertext)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GpuKernelError {
    UnsupportedDType(DType),                                  // nicht F32/F16
    InvalidParams(String),                                    // s. Abschnitt 4
    AckBehind { required: u64, acked: u64 },                  // Ack deckt Vorgänger nicht ab
    Kernel(KernelError),                                      // Register-Fehler, auch Kernel::Library
    Launch { kernel: &'static str, grid: u32, block: u32, cause: String },
    Lane(LaneError),                                          // record_event
}
impl std::fmt::Display for GpuKernelError {}
impl std::error::Error for GpuKernelError {}

/// Launcher für Dimension DIM (DIM >= 1, per const-Assertion erzwungen). Send + Sync.
pub struct GpuKernels<const DIM: usize> { /* register, launcher, Mutex<u64> launched */ }
impl<const DIM: usize> GpuKernels<DIM> {
    pub fn new(register: Arc<KernelRegister>, launcher: Arc<dyn GpuLauncher>) -> Self;
    /// Anzahl erfolgreich gestarteter Batches (= Nummer des letzten Batches).
    pub fn launched(&self) -> u64;
    /// Dichtes Dot-Product. Output f32[query_len * slot_total], Index q * slot_total + s.
    pub fn dense_dot(&self, lane: &mut Lane, doc_dtype: DType,
                     p: &KernelParams, acked: u64) -> Result<Event, GpuKernelError>;
    /// Fused Max-Similarity, genau eine Query. Output f32[slot_total], Index s.
    pub fn fused_max_sim(&self, lane: &mut Lane, doc_dtype: DType,
                         p: &KernelParams, acked: u64) -> Result<Event, GpuKernelError>;
}
```

## 4. Behavior and invariants

**Launch-Konfiguration.**
- `block == 128`, `dynamic_shared == 0`.
- `grid == max(1, slot_total)`, ein Block je Slot.
- Block `b >= slot_total` kehrt sofort zurück. Bei `slot_total == 0` startet ein Block, der keinen Slot bewertet.
- Slot von Block `b` = `slots[(head + b) % ring_cap]` (u32-Arithmetik ohne Überlauffehler, `head + b` wrapping).
- Das Segment ist der Eintrag mit `first_slot <= slot < first_slot + slot_count`. Die Dokumentadresse ist `base + (slot - first_slot) * stride_bytes`.

**Reihenfolge in `dense_dot` und `fused_max_sim`.** Der erste Fehler gewinnt. Bei einem Fehler vor Schritt 6 wird weder ein Kernel geholt noch gestartet.
1. `doc_dtype` ∉ {F32, F16} → `UnsupportedDType`.
2. Parameter validieren (`InvalidParams`):
   - `query_len >= 1`.
   - `ring_cap > 0`.
   - `slot_total <= ring_cap`.
   - `slot_total <= i32::MAX`.
   - Bei `slot_total > 0` gilt `segment_count >= 1`.
   - Bei `fused_max_sim` hat jedes Segment `pad_len >= 1`, aber das Modul liest die Segmenttabelle nicht.
   - `size_of::<KernelParams>() <= lane.params().len()`.
3. Ack-Prüfung: Sei `n = launched()`. Ist `n >= 1` und `acked < n`, dann `AckBehind { required: n, acked }`. Der erste Batch (`n == 0`) startet immer. Die Prüfung gilt für beide Kernel und deckt sich mit dem Ack des Vorgängers.
4. Kernel holen: `register.request(name, doc_dtype, Variant{dim: Some(DIM as u32), tile: None})`. Fehler werden zu `Kernel(e)`. `Kernel::Library` ergibt `Kernel(Unavailable)`.
5. `KernelParams` byteweise in `lane.params_mut()[..72]` schreiben (Rest bleibt unverändert).
6. `launcher.launch(...)` mit `lane.params()[..72]`. Ein Fehler ergibt `Launch{kernel, grid, block, cause}` mit der Ursache des Treibers unverändert.
7. Nach erfolgreichem Start wird `launched` um 1 erhöht, danach `lane.record_event()`. Fehler → `Lane(e)`, der Batch zählt trotzdem als gestartet.

Prüfung (3) bis Zählererhöhung (7) laufen unter einem Mutex, dadurch sind Prüfung und Batch-Nummer atomar. Ein fehlgeschlagener Launch erhöht `launched` nicht. `launch` blockiert nicht auf das Kernel-Ende.

**`dense_dot`-Kernel.**
- Ein Block je Slot. Jeder Slot hat genau einen Vektor mit `DIM` Elementen.
- Die Threads summieren strided über `DIM`. Danach reduziert `warp_reduce_sum` je Warp, und die 4 Warp-Summen werden über statischen Shared Memory (`__shared__ float[4]`) zusammengeführt.
- `out[q * slot_total + b] = Σ_i query[q][i] * doc[i]` für `q in 0..query_len`, Akkumulation in f32.
- F16-Elemente werden beim Laden auf f32 gewandelt (`load_f`). NaN und Inf werden unverändert durchgereicht.

**`fused_max_sim`-Kernel.**
- Ein Block je Dokument, genau eine Query mit `query_len` Tokens.
- Die effektive Länge ist `L = min(lens[slot - first_slot], pad_len)`. Ist `lens_addr == 0`, gilt `L = pad_len`.
- Für jeden Query-Token `t` gilt:
  - `m_t = max_{d < L} dot(query[t], doc[d])`, gerechnet mit Register- bzw. Warp-Reduktion. Es entsteht keine Q×D-Matrix im Global Memory.
  - Ist `L == 0`, dann `m_t = NO_VALID_TOKEN_SCORE` (−10 000).
- `out[b] = Σ_t m_t`. Der Wert ist ein endlicher f32, wenn die Eingaben endlich sind.
- Padding-Tokens der Query werden nicht ausgeschlossen.
- Dokument-Tokens ab `L` werden nie gelesen.

**Doorbell (Device-Seite, in beiden Kerneln).**
- Jeder Block schreibt zuerst alle seine Outputs. Danach erhöht er `*block_ctr_addr` per `atomicAdd` mit vorangehendem `__threadfence()`.
- Der Block, der `grid` erreicht (der letzte), setzt `*block_ctr_addr = 0`. Danach erhöht er `*completed_addr` um 1 per `atom.add.release.sys.u64` (CC ≥ 7.0). Alle Output-Writes aller Blöcke sind damit vor dem Inkrement sichtbar.
- Pro erfolgreichem Launch steigt `completed` genau um 1. Die Batch-Nummer des Launches ist der Wert von `launched()` nach dem Launch.
- Der Host sieht die Ergebnisse nach `Doorbell::poll` mit `last >= Batch-Nummer` oder nach `Event::wait`. Er schreibt erst danach Parameter und Ergebnisse neu und acked per `Gap::close`.

**Kernel-Quellen.**
- Genau zwei Quellen: `dense_dot` und `fused_maxsim`, jeweils `extern "C" __global__`, dtypes `[F32, F16]`. Die Entry-Namen wählt der Implementierer.
- Das `KernelParams`-Layout im Quelltext entspricht Byte für Byte dem Rust-`repr(C)`-Layout (Offsets s. o.). Ein Test prüft `size_of == 72`, `align_of == 8` und die Offsets. Für `SegmentEntry` prüft er `size_of == 40`.

**Concurrency.**
- `GpuKernels` ist `Send + Sync`. Aufrufe sind über den Mutex serialisiert.
- Eine Instanz gehört zu einem Doorbell und einer Stream-Ordnung (einer Lane). Mehrere Lanes mit derselben Instanz brechen die Batch-Ordnung. Das ist Sache des Aufrufers.
- Der Aufrufer hält Segmente, Slot-Ring, Query- und Output-Speicher bis zum Abschluss des Batches (Event bzw. `ack`) am Leben und beschreibt sie nicht währenddessen.
- Es gibt keine Panics bei ungültigen Eingaben. Alle Fehler sind `GpuKernelError`.

**Tests ohne GPU (Mock-`GpuLauncher`).**
- Erster Batch startet mit `acked == 0`.
- Zweiter Batch mit `acked == 0` ergibt `AckBehind{required: 1, acked: 0}` ohne Launch und ohne Zählererhöhung.
- Zweiter Batch mit `acked == 1` startet.
- `LaunchConfig` hat `block 128`, `dynamic_shared 0` und `grid == max(1, n)`.
- Launcher-Fehler erscheint als `Launch` mit unveränderter Ursache. `launched` bleibt unverändert.
- BF16 ergibt `UnsupportedDType`.
- Das Param-Layout stimmt.
- Wrapping von `head`/`tail` wird korrekt behandelt.
- Die Kernel-Quellen enthalten kein `extern __shared__`.

## 5. Relied-on elements

- `device_lane_event`:
  - `Lane::params(&self) -> &[u8]`, `params_mut(&mut self) -> &mut [u8]` (fest 256 Byte, null-initialisiert).
  - `Lane::record_event(&self) -> Result<Event, LaneError>`. `Lane: Send + Sync`.
  - `Event: Clone + Send + Sync`. `LaneError::{NoStreamBackend, Record(EventError)}`.
- `kernel_register`:
  - `KernelRegister::request(&self, name, dtype, Variant{dim: Option<u32>, tile: Option<u32>}) -> Result<Kernel, KernelError>`. `KernelRegister::register_source(KernelSource{name, body, entry, dtypes, precompiled})`.
  - `Kernel::{Gpu(Arc<GpuKernel>), Library(..)}`. `GpuKernel.function: Arc<dyn KernelFunction>`.
  - `KernelError: Clone + PartialEq + Eq` mit `Unavailable`, `UnknownKernel` usw. Prelude: `T`, `load_f`, `warp_reduce_sum`, `warp_reduce_max`, `DIM`.
  - JIT-Anfragen mit BF16 oder einem nicht in `dtypes` enthaltenen Typ ergeben `Unavailable`.
- Doorbell (`Doorbell-Synchronisation`):
  - `completed` ist ein monotoner u64 ab 0, nur vom Device inkrementiert. `acked` (u64, ≤ `completed`) wird nur vom Host per `Gap::close` mit Release geschrieben.
  - `Doorbell::acked(&self) -> u64` liefert den Wert, den der Aufrufer als `acked` übergibt.
  - Inkremente dürfen koaleszieren. Die Zellen sind 8-aligned und nur atomar zugänglich.
- `layout`: `DType::{F32, F16, BF16, ..}`. `MASK_VALUE: f32 = -10_000.0`.
- Unterstützte Precisions: GPU-Scoring-Kernel nur für `[F32, F16]`, BF16 nie.