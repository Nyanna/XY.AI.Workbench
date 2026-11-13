# Spezifikation: Library-Adapter (Build-Order 61)

## 1. Task, restrictions and boundaries

**Task.** Rust-Modul `library_adapter`. Es bindet jede externe Library an genau einer Stelle an.
- Tensor-Library ist **candle** (`candle-core`, CUDA-Backend, darunter `cudarc`).
- Das Modul konvertiert Elementtypen und Größen zwischen Engine und Library.
- Es hüllt einen Teilbereich eines Device-Blocks in einen Library-Tensor ein (Wrapping).
- Es stellt die Kernel der Library als Quelle für das Kernel-Register bereit.

**Grenzen.**
- Nur dieses Modul importiert `candle_core` und `cudarc`. Kein anderes Modul sieht deren Typen. Ausnahme ist der opake `LibTensor`, der als Träger des Library-Tensors durch das Modul-API läuft.
- Typen der Library werden nur an den Funktionen dieses Moduls konvertiert.
- Keine Planung, keine Allokation von Blöcken, keine Eviction, keine Generationsverwaltung und kein Kernel-Register. Das Modul definiert auch `RegionHandle`, `DType` und `Layout` nicht neu.
- Das Modul löst `RegionHandle` nicht selbst auf. Das Memory-Backend liefert den Block (`DeviceBlock`) und prüft die Generation. Das Modul prüft nur Grenzen, Ausrichtung und Layout.
- Das Modul legt keine Context- oder Device-Verwaltung an. Es nutzt das `CudaDevice` aus `DeviceBlock`.

## 2. Context

- `layout` liefert `DType`, `Layout` (Byte-Strides). `region_handle` liefert `RegionHandle` (Arena, Offset, Länge in Byte, Generation). `memory_access` liefert Region-/`TensorRef`-Zugriff für den Host. Dieses Modul nutzt nur `DType`, `Layout` und `RegionHandle`.
- Der Engine rechnet in Bytes. Die Library rechnet in Elementen.
- Das Memory-Backend besitzt Context und Block. Es ruft `wrap_region` mit seinem Block auf und gibt Block und Context erst nach allen gewrappten Tensoren frei.
- Das Kernel-Register (Spezifikation noch nicht vorhanden) bezieht über `library_kernel_source()` die Kernel der Library. Es adaptiert dafür die hier definierten Einträge.

## 3. Interface

```rust
use layout::{DType, Layout};
use region_handle::RegionHandle;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AdapterError {
    /// Library-DType hat keine Entsprechung in `DType`.
    UnsupportedLibDType(String),
    /// bytes % dtype.size_in_bytes() != 0
    NotElementMultiple { bytes: u64, elem_size: usize },
    /// Größenberechnung läuft über.
    Overflow,
    /// Region liegt nicht vollständig im Block, oder Layout belegt mehr als die Region.
    OutOfBounds { offset: u64, len: u64, block_len: u64 },
    /// Layout.byte_len() > region.len()
    LayoutExceedsRegion { required: u64, region_len: u64 },
    /// Layout ist nicht contiguous.
    NotContiguous,
    /// Device-Adresse (base+offset) nicht auf Elementgröße ausgerichtet.
    Misaligned { addr: u64, align: usize },
    /// Form, Typ oder Device der Operanden passt nicht.
    Mismatch(String),
    /// Unbekannter Kernel-Name.
    UnknownKernel(String),
    /// Fehler der Library (Text der Library).
    Library(String),
}
impl std::fmt::Display for AdapterError {}
impl std::error::Error for AdapterError {}

/// Konvertierung der Elementtypen (beide Richtungen).
pub fn to_lib_dtype(d: DType) -> candle_core::DType;
pub fn from_lib_dtype(d: candle_core::DType) -> Result<DType, AdapterError>;

/// Elementzahl (Library) <-> Bytezahl (Engine).
pub fn elems_to_bytes(dtype: DType, elems: u64) -> Result<u64, AdapterError>;
pub fn bytes_to_elems(dtype: DType, bytes: u64) -> Result<u64, AdapterError>;

/// Device-Block, vom Memory-Backend geliefert. Der Block bleibt Eigentum des Backends.
#[derive(Debug, Clone)]
pub struct DeviceBlock {
    pub device: candle_core::cuda_backend::CudaDevice, // Context/Device des Blocks
    pub base_addr: u64,                                 // Device-Adresse des Blockbeginns
    pub len_bytes: u64,
}

/// Opaker Träger eines Library-Tensors. Send + Sync + 'static.
#[derive(Debug, Clone)]
pub struct LibTensor { /* candle_core::Tensor + Flag borrowed */ }
impl LibTensor {
    pub fn lib(&self) -> &candle_core::Tensor;
    pub fn into_lib(self) -> candle_core::Tensor;
    pub fn from_lib(t: candle_core::Tensor) -> LibTensor;   // besitzender Tensor
    pub fn dtype(&self) -> Result<DType, AdapterError>;
    pub fn dims(&self) -> &[usize];
    /// true, wenn der Tensor fremden Speicher (Block) nur einhüllt.
    pub fn is_borrowed(&self) -> bool;
    /// Kopie mit eigenem Speicher außerhalb des Blocks (contiguous, besitzend).
    pub fn to_owned_copy(&self) -> Result<LibTensor, AdapterError>;
    /// Schreibt src (gleiche Form und Typ) in diesen Tensor; bei gewrapptem Tensor schlägt es auf den Teilbereich durch.
    pub fn write_from(&self, src: &LibTensor) -> Result<(), AdapterError>;
    /// Kopiert den Inhalt in den Host (Bytes, row-major, dtype wie Tensor).
    pub fn to_host_bytes(&self) -> Result<Vec<u8>, AdapterError>;
}

/// Hüllt `region` (Byte-Offset/-Länge im Block) als Tensor mit `layout` ein.
/// # Safety
/// Block und Context müssen mindestens so lange leben wie der Tensor, jeder View und jede Referenz darauf. Der Aufrufer (Backend) hat Generation und Gültigkeit des Handles geprüft.
pub unsafe fn wrap_region(
    block: &DeviceBlock,
    region: &RegionHandle,
    layout: &Layout,
) -> Result<LibTensor, AdapterError>;

/// Kernel der Library als Quelle für das Kernel-Register.
pub type LibKernelFn =
    Arc<dyn Fn(&[LibTensor]) -> Result<Vec<LibTensor>, AdapterError> + Send + Sync>;
#[derive(Clone)]
pub struct LibKernelEntry { pub name: &'static str, pub arity: usize, pub run: LibKernelFn }
#[derive(Clone)]
pub struct LibraryKernelSource { /* Einträge */ }
impl LibraryKernelSource {
    pub fn source_name(&self) -> &'static str;           // "candle"
    pub fn kernels(&self) -> &[LibKernelEntry];
    pub fn get(&self, name: &str) -> Result<&LibKernelEntry, AdapterError>;
}
pub fn library_kernel_source() -> LibraryKernelSource;
```

`LibTensor`, `DeviceBlock`, `LibraryKernelSource`, `LibKernelEntry` und `AdapterError` sind `Send + Sync + 'static` (Compile-Time-Assertion im Test).

## 4. Behavior and invariants

**Elementtypen**
- `to_lib_dtype` bildet ab: F32→F32, F16→F16, BF16→BF16, U8→U8, U32→U32, I64→I64.
- `from_lib_dtype` ist die Umkehrung. Alle übrigen Library-Typen (z. B. F64, I16, I32) ergeben `UnsupportedLibDType(<Debug-Text>)`.
- Es gilt `from_lib_dtype(to_lib_dtype(x)) == Ok(x)` für alle Varianten.

**Größen**
- `elems_to_bytes = elems * size_in_bytes` mit `checked_mul`, sonst `Overflow`.
- `bytes_to_elems = bytes / size_in_bytes`. Bei einem Rest kommt `NotElementMultiple`. `0` ergibt `0`.
- Beide Funktionen sind zueinander invers, wo definiert.

**Wrapping (`wrap_region`)**
Die Prüfungen laufen in dieser Reihenfolge, die erste Verletzung gewinnt:
1. `region.offset().checked_add(region.len())` überläuft: `Overflow`.
2. `region.end() > block.len_bytes`: `OutOfBounds { offset, len, block_len }`.
3. `!layout.is_contiguous()`: `NotContiguous`.
4. `layout.byte_len() > region.len()`: `LayoutExceedsRegion`.
5. `(base_addr + offset) % size_in_bytes != 0`: `Misaligned` (Addition überlaufsicher, sonst `Overflow`).

Bei Erfolg gilt:
- Der Tensor hat `layout.shape()` und den Typ `to_lib_dtype(layout.dtype())`. Er liegt auf `block.device` und liest exakt die Bytes `[base_addr+offset, +layout.byte_len())`.
- Der Tensor besitzt den Speicher nicht (`is_borrowed() == true`). Die Adresse wird per `cudarc` (`upgrade_device_ptr`) eingehüllt und dann in einen Library-Storage gesteckt.
- **Nicht-Freigabe.** Weder das Droppen des Tensors noch das seiner Klones oder Views gibt Device-Speicher frei oder zerstört Context oder Block. Die Implementierung verhindert das, indem sie den Storage-Besitz bewusst leakt (z. B. `mem::forget` einer Arc-Referenz oder `ManuallyDrop`). Der Leak ist auf Host-Metadaten beschränkt.
- Ein Layout mit `elem_count == 0` ist gültig, wenn die Prüfungen 1–2 bestehen. Das Wrapping greift dann nicht auf Device-Speicher zu und liefert einen leeren Tensor.
- Das Wrapping kopiert nichts und allokiert kein Device-Memory.

**Kopien und Schreiben**
- `to_owned_copy` (und jede Library-Operation, die einen neuen Tensor erzeugt) liefert `is_borrowed() == false` mit eigenem Speicher außerhalb des Blocks. Spätere Schreibzugriffe auf den Block ändern die Kopie nicht, und Schreibzugriffe auf die Kopie ändern den Block nicht.
- `write_from` auf einem gewrappten Tensor verändert genau dessen Teilbereich im Block. Bytes außerhalb von `[offset, offset+byte_len)` bleiben unverändert, auch die Nachbarelemente. Bei abweichender Form, abweichendem Typ oder anderem Device kommt `Mismatch`.
- Library-Fehler werden als `Library(text)` weitergereicht. Es gibt keine Panics bei gültigen Eingaben.

**Kernel-Quelle**
- `source_name()` ist `"candle"`. `library_kernel_source()` ist rein und liefert bei jedem Aufruf dieselben Einträge.
- Die Einträge sind fest: `"matmul"` (arity 2), `"add"` (2), `"mul"` (2) und `"softmax_last_dim"` (1). Namen sind eindeutig. Jeder Eintrag liefert genau einen Ergebnis-Tensor mit eigenem Speicher (`is_borrowed() == false`).
- Bei falscher Operandenzahl, falschem Typ oder falscher Form kommt `Mismatch`. `get` mit unbekanntem Namen liefert `UnknownKernel(name)`.
- Die Kernel-Funktionen sind `Send + Sync` und ohne internen veränderlichen Zustand nebenläufig aufrufbar. Planung und Auswahl liegen beim Register und beim Planer, nicht hier.

**Nebenläufigkeit**
- Alle Funktionen sind zustandslos. Mehrere Threads dürfen gleichzeitig wrappen. Überlappende Schreibzugriffe auf denselben Block-Bereich serialisiert dieses Modul nicht (Sache von Aufrufer und Stream-Ordnung).

**Tests**
- Reine Tests (Dtype-Konvertierung, Größen, Fehlerprüfungen 1–5 ohne Device, Send/Sync-Assertions) laufen überall. Prüfungen 2–5 sind testbar, weil sie vor dem Device-Zugriff laufen. Dafür gibt es einen internen Prüfpfad ohne Device.
- Die Wrapping-Tests laufen **nur auf einem echten Nvidia-Device** (Feature `cuda`). Ohne Device brechen sie sauber ab, nicht mit Fehlschlag.
- Die Wrapping-Tests prüfen: (a) Lesen liefert exakt den Teilbereich bei Offset > 0, (b) Drop des Tensors lässt Block und Context intakt (Block danach noch les- und beschreibbar), (c) Kopie hat eigenen Speicher, (d) `write_from` schlägt auf den Teilbereich durch und Nachbarelemente bleiben unverändert.

## 5. Relied-on elements

- `layout`: `DType {F32,F16,BF16,U8,U32,I64}` mit `size_in_bytes() -> usize`. `Layout::{shape() -> &[usize], dtype(), elem_count(), byte_len() -> usize, is_contiguous()}`. Strides sind Byte-Strides. `Layout` trägt keinen Offset.
- `region_handle`: `RegionHandle` (`Copy`) mit `offset() -> u64`, `len() -> u64` (Byte), `end() -> u64` (exklusiv, überlauffrei durch Konstruktion). Das Handle enthält keine Device-Adresse. Die Adresse liefert nur das Backend über `DeviceBlock`. Gültigkeit und Generation (`is_current`) prüft das Backend vor dem Aufruf.
- `memory_access`: wird von diesem Modul nicht aufgerufen. `Region`/`TensorRef` bleiben Host-seitige Zugriffswege.