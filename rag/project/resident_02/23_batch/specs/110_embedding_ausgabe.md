# Spezifikation: Embedding-Ausgabe (Build-Order 110)

Crate-Modul `embedding_output`.

## 1. Aufgabe, Einschränkungen, Grenzen

**Aufgabe**
- Am Encoder-Ende die Ergebnisvektoren genau einmal normalisieren (bei Bedarf), in den Output-Datentyp (FP32, FP16, BF16, Little-Endian) konvertieren und lückenlos schreiben. Es gibt zwei Wege:
  - **Host-Pfad:** in die Output-Region (`Region`).
  - **GPU-Pfad:** per dünnem Write-Kernel direkt in den vorab vergebenen Slot (`Reservation`).
- Degenerierte Vektoren führen zu einer Warnung mit Anzahl. Der Vorgang gilt als erfolgreich.
- Schreibfehler als `OutputError::Memory` weiterreichen.
- Auf dem GPU-Pfad den Ergebnis-Tensor bis zum Event am Ende des Schreibens am Leben halten.

**Grenzen**
- Kein Pooling. Die Eingabe sind bereits gepoolte Vektoren (Pooling liegt bei `pooling_norm`). Der Aufrufer normalisiert nicht selbst.
- Kein Commit und keine Slot-Reservierung. Beides macht der Manager über `SegmentWriter`. Das Event geht an ihn zurück.
- Kein Kernel-Launch. Den Launch und das Auslesen des Device-Pointers übernimmt das GPU-Binding hinter `WriteKernelLauncher`.
- Kein Import von `candle_core` oder `cudarc`, kein `unsafe`, keine Fremd-Crates (kein `half`). Es gelten nur `std`, `log` und die Abhängigkeiten.
- Keine Typen der Abhängigkeiten neu definieren. Keine globalen Singletons.
- Andere Output-Typen als F32, F16, BF16 ergeben `Incompatible`.

## 2. Kontext

- `pooling_norm::PoolNorm::normalize` normalisiert in place (L2 mit Epsilon-Clamp) und liefert die Zahl degenerierter Vektoren. `PoolNorm.dim` und `.normalization` bestimmen Vektorlänge und Normalisierungsbedarf.
- `memory_access::Region::write` ist der Host-Schreibweg. `AccessError` ist der Memory-Fehler.
- `segment_writer::Reservation` liefert den Slot (`addr: Addr`, `target: Range<u64>`). Der Manager committet danach mit dem Event, das dieses Modul liefert (`commit` oder `commit_deferred`).
- `device::Lane::record_event(&self)` liefert das Event nach dem Launch.
- `kernel_register::KernelRegister` liefert den JIT-Kernel. Das Prelude stellt `T`, `load_f`, `warp_reduce_sum` und `DIM` bereit.
- `library_adapter::LibTensor` ist der opake Ergebnis-Tensor der GPU (F32, `[n, dim]`, contiguous).

## 3. Interface

```rust
use std::ops::Range;
use device::{Event, EventError, Lane, LaneError};
use kernel_layout::Addr;
use kernel_register::{GpuKernel, KernelError, KernelRegister, KernelSource};
use layout::DType;
use library_adapter::LibTensor;
use memory_access::{AccessError, Region};
use pooling_norm::{PoolNorm, PoolNormError};
use segment_writer::Reservation;

pub const WRITE_KERNEL_NAME: &str = "embedding_write";

#[derive(Debug, Clone, PartialEq)]
pub enum OutputError {
    /// Output-Datentyp ist nicht F32/F16/BF16. Anzeige: "inkompatibel: Output-Datentyp {0:?}"
    Incompatible(DType),
    /// Speicher-/Schreibfehler (Region::write bzw. Launch-Fehler als AccessError::Backend(text)).
    Memory(AccessError),
    /// Normalisierung/Form (u. a. ShapeMismatch).
    PoolNorm(PoolNormError),
    /// GPU: Ergebnis-Tensor ist nicht F32 oder nicht [n, dim].
    InvalidResult(String),
    /// GPU: Slot kleiner als n*dim*elem_size.
    SlotTooSmall { required: u64, available: u64 },
    Kernel(KernelError),
    Lane(LaneError),
}
impl std::fmt::Display for OutputError {}
impl std::error::Error for OutputError {}
impl From<AccessError> for OutputError {}
impl From<PoolNormError> for OutputError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WriteReport { pub num_vectors: usize, pub bytes_written: usize, pub degenerate_count: usize }

/// true für F32, F16, BF16.
pub fn is_supported_output(dtype: DType) -> bool;
/// f32 -> IEEE-754 binary16 / bfloat16 Bits, Rundung round-to-nearest-even.
pub fn f32_to_f16_bits(x: f32) -> u16;
pub fn f32_to_bf16_bits(x: f32) -> u16;
/// Konvertiert zu Little-Endian-Bytes (len = vectors.len() * size_in_bytes). Sonst Err(Incompatible).
pub fn encode_f32_slice(vectors: &[f32], dtype: DType) -> Result<Vec<u8>, OutputError>;

/// Host-Pfad. `vectors`: num_vectors*dim Werte, row-major, gepoolt, noch nicht normalisiert.
/// Wird in place normalisiert (gemäß norm.normalization).
pub fn write_embeddings(
    region: &Region, offset_bytes: usize,
    vectors: &mut [f32], norm: &PoolNorm, out_dtype: DType,
) -> Result<WriteReport, OutputError>;

// ---------- GPU-Pfad ----------
/// Kernel-Argumente (ABI des Write-Kernels, s. Abschnitt 4).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WriteKernelArgs {
    pub num_vectors: u64, pub dim: u32,
    pub normalize: bool,
    pub out_dtype: DType,          // F32 | F16 | BF16
    pub dst: Addr,                 // Device-Adresse = reservation.addr
}

/// Vom GPU-Binding implementiert.
pub trait WriteKernelLauncher: Send + Sync {
    /// Liest den Device-Pointer von `result`, startet `kernel` asynchron auf dem Stream von `lane`
    /// mit Grid = num_vectors Blöcke à 32 Threads. Blockiert nicht auf Abschluss.
    fn launch(&self, lane: &Lane, kernel: &GpuKernel, args: &WriteKernelArgs, result: &LibTensor)
        -> Result<(), String>;
}

/// JIT-Quelle des Write-Kernels (name = WRITE_KERNEL_NAME, entry = "embedding_write",
/// dtypes = &[DType::F32], precompiled = None).
pub fn write_kernel_source() -> KernelSource;
/// Registriert die Quelle; doppelte Registrierung -> Err(KernelError::Unavailable) (Verhalten des Registers).
pub fn register_write_kernel(reg: &KernelRegister) -> Result<(), KernelError>;

/// Hält Ergebnis-Tensor und Event. Send + Sync.
pub struct GpuWriteTicket { /* result: LibTensor, event: Event */ }
impl GpuWriteTicket {
    pub fn event(&self) -> &Event;            // Event am Ende des Schreibens (Klon teilt Zustand)
    pub fn is_complete(&self) -> Result<bool, EventError>;
    pub fn wait(&self) -> Result<(), EventError>;
}
impl Drop for GpuWriteTicket { /* wartet auf das Event, falls nicht abgeschlossen; Fehler ignorieren */ }

pub fn write_embeddings_gpu(
    register: &KernelRegister, launcher: &dyn WriteKernelLauncher, lane: &Lane,
    result: &LibTensor, norm: &PoolNorm, out_dtype: DType, slot: &Reservation,
) -> Result<GpuWriteTicket, OutputError>;
```

`OutputError`, `WriteReport` und `GpuWriteTicket` sind `Send + Sync + 'static` (Compile-Time-Assertion im Test).

## 4. Behavior und Invarianten

**Datentypen und Konvertierung**
- Erlaubt sind F32, F16 und BF16, jeweils Little-Endian. Alles andere ergibt `Incompatible(dtype)`.
- Die Typprüfung läuft immer zuerst, in beiden Pfaden. Bei `Incompatible` wird nichts normalisiert, geschrieben oder gestartet, und `vectors` bleibt unverändert.
- F32 wird bitgleich übernommen.
- F16 und BF16 runden round-to-nearest-even:
  - F16: Überlauf ergibt ±Inf, Unterlauf ergibt Subnormale bzw. ±0.
  - ±Inf bleibt ±Inf.
  - NaN bleibt NaN (F16 `sign|0x7E00`, BF16 `sign|0x7FC0`).
  - Beispiele: `f32_to_f16_bits(1.0)==0x3C00`, `f32_to_bf16_bits(1.0)==0x3F80`.

**Host-Pfad `write_embeddings`**
Ablauf in dieser Reihenfolge:
1. Typprüfung.
2. `norm.normalize(vectors)` genau einmal. `ShapeMismatch` (`vectors.len() % dim != 0`) ergibt `PoolNorm(..)`. Bei `Normalization::None` bleiben die Werte unverändert und die Anzahl ist 0.
3. Konvertierung in einen zusammenhängenden Puffer.
4. Genau ein `region.write(offset_bytes, &buf)`.

Folgen:
- Die Vektoren stehen lückenlos hintereinander, Vektor i bei `offset_bytes + i*dim*elem_size`. Es gibt kein Padding.
- Ein Fehler von `Region::write` (z. B. `OutOfBounds`, `Backend`) ergibt `Memory(err)`, unverändert weitergereicht. `WriteReport` entsteht dann nicht.
- Leere Eingabe ergibt `Ok` mit 0 Bytes und ohne Schreibzugriff.
- `WriteReport { num_vectors: len/dim, bytes_written: len*elem_size, degenerate_count }`.
- Bei `degenerate_count > 0` und erfolgreichem Schreiben wird `log::warn!` mit der Anzahl gerufen, mit dem Sinn „N leere Embeddings als Nullvektor gespeichert“. Das Ergebnis bleibt `Ok`. Die Vektoren sind praktisch Nullvektoren (NaN-Vektoren zählen nicht, laut `normalize`).
- Bei Pooling `None` zählen Padding-Nullvektoren mit. Dieses Modul filtert nicht.

**GPU-Pfad `write_embeddings_gpu`**
Prüfreihenfolge:
1. Typprüfung (`Incompatible`).
2. `result.dtype()` ist F32 und `result.dims() == [n, norm.dim]`, sonst `InvalidResult`.
3. `required = n*dim*size_in_bytes(out_dtype)`. Ist `required > slot.target.end - slot.target.start`, kommt `SlotTooSmall { required, available }`. Der Slot ist `slot.target`. Überlauf zählt als zu klein.
4. `register.request(WRITE_KERNEL_NAME, DType::F32, Variant { dim: Some(dim), tile: None })`:
   - Fehler ergibt `Kernel(e)`.
   - `Kernel::Library(_)` ergibt `Kernel(UnknownKernel(name))`.
5. Wenn `n > 0`, ruft es `launcher.launch(lane, &gpu_kernel, &args, result)` auf. `Err(text)` ergibt `Memory(AccessError::Backend(text))`. Bei `n == 0` entfällt der Launch.
6. `lane.record_event()`. `Err(e)` ergibt `Lane(e)`.
7. Es liefert `GpuWriteTicket` mit einem Klon von `result` und dem Event.

Dabei gilt:
- `args.dst = slot.addr` und `args.normalize = (norm.normalization == L2)`.
- Die Funktion blockiert nie auf die GPU. Sie schreibt keine Bytes auf dem Host.
- Das Event deckt alle bis dahin auf der Lane eingereihte Arbeit ab, einschließlich des Write-Kernels. Der Manager übergibt es an `SegmentWriter::commit`/`commit_deferred`. Der Slot darf erst dann sichtbar werden.
- Der Ergebnis-Tensor lebt, solange das Ticket lebt. `Drop` des Tickets vor Eventende wartet auf das Event (blockierend), damit der Tensor nicht vorzeitig freigegeben wird.
- Ein Fehler vor dem Launch hinterlässt keinen Kernel-Lauf. Der Slot bleibt unverändert und unsichtbar. Er bleibt reserviert, laut Segment-Schreibpfad ohne Abbruch-API.
- Die Zahl degenerierter Vektoren liest dieser Pfad nicht zurück. Es gibt dort keine Degenerations-Warnung.

**Write-Kernel (ABI)**
- Pro Vektor ein Block aus einem Warp (32 Threads), Grid = `num_vectors`.
- Symbol `extern "C" __global__ void embedding_write(const float* src, unsigned char* dst, unsigned long long n, unsigned int normalize, unsigned int out_dtype)`.
- `out_dtype` ist 0=F32, 1=F16, 2=BF16.
- `DIM` kommt aus dem Prelude-Define (Variant `dim`). `T` ist `float`.
- Der Body wird hinter das Prelude gesetzt und nutzt `load_f` und `warp_reduce_sum`.
- Pro Vektor v gilt: bei `normalize != 0` ist `n = sqrt(Σ v_i²)`, `n_c = max(n, 1e-12)` und `v_i ← v_i / n_c`. NaN-Vektoren bleiben NaN.
- Der Kernel konvertiert per Bit-Operationen (RNE, wie die Host-Funktionen) und schreibt Little-Endian lückenlos ab `dst + i*DIM*elem_size`.
- BF16 läuft über diese Bitkonvertierung. Es gibt keinen BF16-Compute-Pfad. Der Register-Key nutzt daher `DType::F32`.
- Das Ergebnis darf von der CPU-Variante um Rundung der Normberechnung (f32-Akkumulation) abweichen.

**Nebenläufigkeit**
- Alle Funktionen sind zustandslos und `&self`/Parameter-basiert. Mehrere Threads dürfen gleichzeitig in disjunkte Bereiche oder Slots schreiben. Überlappende Zielbereiche serialisiert das Modul nicht.
- Das Modul hält keinen Lock über einen Aufruf hinweg. Es panikt nie.

## 5. Relied-on elements

- **`pooling_norm`**
  - `PoolNorm { pub pooling, pub normalization: Normalization, pub dim: usize }` (`Copy`).
  - `normalize(&self, &mut [f32]) -> Result<usize, PoolNormError>`:
    - L2 mit `NORM_EPSILON = 1e-12`.
    - `Normalization::None` lässt alles unverändert und liefert 0.
    - `ShapeMismatch`, wenn `len % dim != 0`.
    - Ein leerer Slice ergibt `Ok(0)`.
  - `PoolNormError` ist `Debug + Clone + PartialEq`.
  - `Normalization::{L2, None}`.
- **`layout`**: `DType {F32,F16,BF16,U8,U32,I64}`, `size_in_bytes()`.
- **`memory_access`**
  - `Region::write(&self, offset: usize, src: &[u8]) -> Result<(), AccessError>`: prüft Grenzen. Länge 0 ist ein No-op. Read-your-writes.
  - `AccessError::{OutOfBounds{..}, Backend(String)}` (`Debug, Clone, PartialEq, Eq`).
- **`region_handle`**: nicht direkt genutzt. Die Output-Region ist ein `Region`, der Slot ein `Reservation`.
- **`segment_writer`**
  - `Reservation { pub slot, pub entry, pub tokens, pub centroid, pub addr: Addr, pub target: Range<u64> /* Länge = slot_bytes */ }`.
  - Commit danach durch den Manager mit dem Event (`commit(&Reservation, &Event)` bzw. `commit_deferred(&Reservation, Event)`).
- **`device`**
  - `Lane::record_event(&self) -> Result<Event, LaneError>`. CPU-Lane ohne Stream ergibt `LaneError::NoStreamBackend`.
  - `Event` ist `Clone + Send + Sync`, mit `is_complete()` (nicht blockierend) und `wait()`. `EventError` ist `Clone + Debug + PartialEq`.
  - `Lane: Send + Sync`.
- **`library_adapter`**: `LibTensor` (`Clone + Send + Sync + 'static`) mit `dtype() -> Result<DType, AdapterError>` und `dims() -> &[usize]`. Der Device-Pointer wird nur vom Launcher gelesen.
- **`kernel_register`**
  - `KernelSource { name, body, entry, dtypes, precompiled }`.
  - `KernelRegister::{register_source(&self, KernelSource) -> Result<(), KernelError>, request(&self, name, DType, Variant) -> Result<Kernel, KernelError>}`.
  - `Kernel::{Gpu(Arc<GpuKernel>), Library(..)}` und `Variant { dim: Option<u32>, tile: Option<u32> }`.
  - `BF16` hat keinen Compute-Pfad, deshalb Key-Typ F32.
  - Das Prelude liefert `T`, `load_f`, `warp_reduce_sum`, `warp_reduce_max` und `#define DIM`.
  - Das Register startet keine Kernel.
- **`kernel_layout`**: `Addr(pub u64)`.