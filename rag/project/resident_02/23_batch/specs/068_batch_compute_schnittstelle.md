# Spezifikation: Batch-Compute-Schnittstelle (Build-Order 68)

## 1. Task, restrictions and boundaries

**Task.** Rust-Modul `batch_compute`. Es stellt einen austauschbaren Trait von Float-Batch zu Float-Batch bereit. Es enthält zwei Implementierungen: `CpuBatchCompute` (Default und Fallback) und `GpuBatchCompute` (nur mit Cargo-Feature `gpu`). Eine Factory wählt die Implementierung.

**Grenzen.**
- Die GPU-Implementierung ist die einzige Stelle in diesem Modul, die die GPU-Library (candle) anspricht. `CpuBatchCompute`, der Trait und die Factory importieren weder `candle_core` noch `cudarc` noch `library_adapter`.
- Kernel-Ausführung auf der GPU läuft ausschließlich über `library_adapter::library_kernel_source()`. Das Modul implementiert keine eigenen GPU-Kernel, kein Tensor-Core- und kein BF16-Compute.
- Das Modul macht keine Planung, keine Speicherverwaltung über Batches hinaus und kein Kernel-Register. Es erfindet keine weiteren Operationen. Es ändert weder `Cargo.toml` noch Abhängigkeiten. Das Feature `gpu` zieht die bereits vorhandene candle-Abhängigkeit mit CUDA-Backend ein.
- Ohne Feature `gpu` wird keine GPU-Library einkompiliert. Das Modul ist dann reines CPU-Rust.

## 2. Context

- **Library-Adapter** (Build-Order 61) ist die einzige Stelle, die `candle_core` und `cudarc` kapselt. Er liefert `LibTensor` und die Kernel `matmul` (2), `add` (2), `mul` (2) und `softmax_last_dim` (1). Die CPU-Implementierung muss dieselbe Semantik liefern.
- **Zielumgebung** ist eine GTX 1660 SUPER (CC 7.5), nur FP32/FP16, kein BF16-Compute und kein Tensor-Core. Dieses Modul rechnet in `f32`.
- Aufrufer halten ein `Arc<dyn BatchCompute>` oder `Box<dyn BatchCompute>` und kennen die konkrete Implementierung nicht.

## 3. Interface

```rust
use std::fmt;

/// Zeilenmajor, dicht. data.len() == rows * cols.
#[derive(Debug, Clone, PartialEq)]
pub struct FloatBatch { rows: usize, cols: usize, data: Vec<f32> }
impl FloatBatch {
    pub fn new(rows: usize, cols: usize, data: Vec<f32>) -> Result<Self, ComputeError>; // Mismatch bei falscher Länge, Overflow bei rows*cols
    pub fn rows(&self) -> usize;
    pub fn cols(&self) -> usize;
    pub fn as_slice(&self) -> &[f32];
    pub fn into_vec(self) -> Vec<f32>;
}

/// Operationen; Semantik identisch zu den Kernels des Library-Adapters.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BatchKernel {
    MatMul,         // "matmul"           2 Eingaben: [m,k] x [k,n] -> [m,n]
    Add,            // "add"              2 Eingaben, gleiche Form
    Mul,            // "mul"              2 Eingaben, gleiche Form, elementweise
    SoftmaxLastDim, // "softmax_last_dim" 1 Eingabe, Softmax je Zeile
}
impl BatchKernel {
    pub fn arity(&self) -> usize;
    pub fn name(&self) -> &'static str; // Adapter-Kernelname
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackendKind { Cpu, Gpu }

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ComputeError {
    /// Falsche Operandenzahl oder Form/Länge passt nicht.
    Mismatch(String),
    Overflow,
    /// GPU-Beschleunigung nicht verfügbar (nicht einkompiliert, kein Device, inkompatibel, Init-Test fehlgeschlagen).
    GpuUnavailable(String),
    /// Fehler der GPU-Library/des Adapters (Text).
    Backend(String),
}
impl fmt::Display for ComputeError {}
impl std::error::Error for ComputeError {}

/// Austauschbare Schnittstelle Float-Batch -> Float-Batch.
pub trait BatchCompute: Send + Sync {
    fn kind(&self) -> BackendKind;
    fn run(&self, kernel: BatchKernel, inputs: &[&FloatBatch]) -> Result<FloatBatch, ComputeError>;
}

pub struct CpuBatchCompute;
impl CpuBatchCompute { pub fn new() -> Self; } // infallible
impl Default for CpuBatchCompute {}
impl BatchCompute for CpuBatchCompute {}

#[cfg(feature = "gpu")]
pub struct GpuBatchCompute { /* Device + LibraryKernelSource */ }
#[cfg(feature = "gpu")]
impl GpuBatchCompute {
    /// Scheitert mit GpuUnavailable ohne kompatible GPU.
    pub fn new() -> Result<Self, ComputeError>;
}
#[cfg(feature = "gpu")]
impl BatchCompute for GpuBatchCompute {}

/// Liefert GPU, wenn einkompiliert und initialisierbar, sonst CPU. Schlägt nie fehl.
pub fn create_default() -> Box<dyn BatchCompute>;
```

`FloatBatch`, `BatchKernel`, `BackendKind`, `ComputeError`, `CpuBatchCompute` und (mit Feature) `GpuBatchCompute` sind `Send + Sync + 'static` (Compile-Time-Assertion im Test). Der Trait ist object-safe.

## 4. Behavior and invariants

**Trait / Allgemein**
- `run` ist rein bezüglich der Eingaben: Eingaben bleiben unverändert, das Ergebnis ist ein neuer, besitzender `FloatBatch`.
- Fehlerprüfung vor der Berechnung, in beiden Implementierungen gleich:
  - `inputs.len() != kernel.arity()` ergibt `Mismatch`.
  - `Add` und `Mul` mit ungleichen `rows`/`cols` ergeben `Mismatch`.
  - `MatMul` mit `a.cols != b.rows` ergibt `Mismatch`.
- Die Ergebnisform ist bei `MatMul` `[a.rows, b.cols]`, bei den anderen Kerneln die Eingabeform.
- Leere Batches (`rows == 0` oder `cols == 0`) sind gültig und ergeben bei passender Form einen leeren Batch gleicher Ergebnisform. `MatMul` mit `k == 0` ergibt eine Nullmatrix `[m,n]`.
- Ein Dimensionsüberlauf ergibt `Overflow`. Es gibt keine Panics bei gültigen oder ungültigen Eingaben.
- `run` darf nebenläufig von mehreren Threads auf derselben Instanz aufgerufen werden. Es gibt keinen veränderlichen Zustand zwischen Aufrufen.

**CPU**
- `new()` ist infallible und braucht weder GPU noch GPU-Library. Die CPU-Implementierung ist ohne Feature `gpu` die einzige und `kind() == Cpu`.
- `Add` und `Mul` rechnen elementweise in `f32`.
- `MatMul` ist die Standardmatrixmultiplikation in `f32`.
- `SoftmaxLastDim` rechnet je Zeile `exp(x - max) / Σ exp(x - max)`. Die Zeilensumme ist 1 (im Rahmen der `f32`-Toleranz). Bei `cols == 0` ist das Ergebnis leer.
- Die Ergebnisse stimmen mit der GPU-Implementierung innerhalb einer relativen Toleranz von etwa 1e-4 überein (Testkriterium, kein bitgenauer Vergleich).

**GPU (Feature `gpu`)**
- `new()` legt ein CUDA-Device (Index 0) an und holt `library_kernel_source()`. Anschließend führt es einen Mini-Init-Test aus: `add` auf zwei 1-Element-Tensoren, Ergebnis wird zurückgelesen und geprüft. Jeder Fehler ergibt `Err(GpuUnavailable(text))`, auch kein Device, kein Treiber oder ein Fehler im Init-Test.
- Ist die Compute Capability über die Library abfragbar, wird sie mit `target_env::jit_target_for` geprüft. CC < 7.0 ergibt `GpuUnavailable`.
- `new()` paniert nie.
- `run` kopiert die Eingaben in Library-Tensoren (`LibTensor::from_lib`), ruft den Kernel `kernel.name()` über `LibraryKernelSource::get` auf und liest das Ergebnis über `to_host_bytes` als `f32` zurück. Es erzeugt dabei keinen Tensor, der Speicher aus einem Block einhüllt.
- Library-Fehler und Adapter-Fehler (`AdapterError`) werden als `Backend(text)` weitergereicht. `AdapterError::Mismatch` wird ebenfalls als `Backend` weitergereicht, weil die Formprüfung vorher schon `Mismatch` liefert.
- `kind() == Gpu`.
- Die Instanz ist `Send + Sync`. Nebenläufige `run`-Aufrufe sind erlaubt, die Library serialisiert Device-Zugriffe selbst.

**Factory / Fallback**
- Ohne Feature `gpu` liefert `create_default()` immer `CpuBatchCompute`.
- Mit Feature `gpu` versucht `create_default()` `GpuBatchCompute::new()`. Bei `Err` fällt es automatisch und ohne Fehler an den Aufrufer auf `CpuBatchCompute` zurück. Es gibt keinen Retry und keine Laufzeit-Umschaltung nach der Erzeugung.
- Es gibt kein stilles Fallback innerhalb von `run`: Ein Fehler zur Laufzeit wird als `Err` gemeldet.

**Tests**
- Reine CPU-Tests laufen überall: Kernel-Semantik, Fehlerfälle, leere Batches, Send/Sync, Object-Safety, `create_default().kind() == Cpu` ohne Feature.
- GPU-Tests (Feature `gpu`) prüfen Gleichheit mit der CPU innerhalb der Toleranz. Ohne Device brechen sie sauber ab (Skip). Zusätzlich muss mit Feature `gpu` und ohne Device gelten: `new()` ergibt `Err(GpuUnavailable)` und `create_default().kind() == Cpu`.

## 5. Relied-on elements

Aus **Library-Adapter** (nur im `gpu`-Pfad):
- `library_adapter::{library_kernel_source, LibraryKernelSource, LibKernelEntry, LibTensor, AdapterError}`.
- `LibraryKernelSource::get(&self, name: &str) -> Result<&LibKernelEntry, AdapterError>`.
- `LibKernelEntry { name: &'static str, arity: usize, run: LibKernelFn }`, wobei `LibKernelFn = Arc<dyn Fn(&[LibTensor]) -> Result<Vec<LibTensor>, AdapterError> + Send + Sync>`.
- Kernelnamen und Stelligkeit: `"matmul"` (2), `"add"` (2), `"mul"` (2) und `"softmax_last_dim"` (1). Jeder Kernel liefert genau einen Ergebnis-Tensor mit eigenem Speicher.
- `LibTensor::from_lib(candle_core::Tensor) -> LibTensor`, `LibTensor::to_host_bytes(&self) -> Result<Vec<u8>, AdapterError>` (row-major, dtype wie Tensor). Hier ist der Typ `f32`.
- `LibTensor`, `LibraryKernelSource` und `AdapterError` sind `Send + Sync + 'static`.
- Die Adapter-Kernel sind nebenläufig aufrufbar und ohne internen veränderlichen Zustand.
- Der Adapter bietet keine Device-Erzeugung und keinen Upload aus Host-Daten. Die GPU-Implementierung nutzt dafür direkt `candle_core` (Device, Tensor aus `f32`-Daten). Sie ist damit die einzige Stelle dieses Moduls mit Library-Bezug.

Aus **Zielumgebung und Abhängigkeiten**:
- `target_env::{ComputeCapability, jit_target_for, TargetEnvError}`. `jit_target_for(cc)` ergibt `Err(UnsupportedCapability)` für CC < 7.0.
- Zielhardware ist eine Turing-GPU (CC 7.5) mit FP32/FP16, ohne BF16-Compute und ohne Tensor-Core. Kernels werden zur Laufzeit per JIT für die erkannte Architektur kompiliert.
- Candle ist die einzige Tensor-Library. Dependencies sind fest und stehen in `/home/user/xyan/xy.ai.workbench/rag/Cargo.toml`.