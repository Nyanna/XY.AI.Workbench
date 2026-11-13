# Spezifikation: Kernel-Register (Modul `kernel_register`, Build-Order 93)

## 1. Aufgabe, Einschränkungen, Grenzen

**Aufgabe.** Das Modul ist der einzige Weg, GPU-Kernel zu laden. Es liefert zu (Name, Datentyp, Architekturstufe, Variante) die geladene Funktion. Dafür kompiliert es lazy per NVRTC (JIT), lädt das Ergebnis, cacht es und meldet Fehler.

Es kennt drei Quellen:
- JIT-Quelltext mit gemeinsamem Prelude.
- Vorkompilierten Maschinencode als Fallback bei zu altem Treiber.
- Kernel der externen Library über `library_adapter` als weitere Implementierung.

**Grenzen.**
- Kein `unsafe`. Kein Import von `cudarc` oder `candle_core`. NVRTC und Treiber liegen hinter den Traits `KernelCompiler` und `ModuleLoader`, die das GPU-Binding implementiert. Dadurch ist das Modul ohne GPU testbar (Mocks).
- Es startet keine Kernel, plant nichts, reserviert keinen Speicher und kennt keine Lanes oder Events. `DeviceHandle` dient nur zur Zuordnung.
- Es gibt keinen Tensor-Core-Pfad, keinen BF16-Compute-Pfad, kein AOT-Set pro Architektur und keine Fallbacks für andere Architekturen als die unten beschriebenen.
- Verwende das `log`-Crate.
- Es gibt keine globalen Singletons.

## 2. Kontext

- Die Referenzkarte ist eine GTX 1660 SUPER mit CC 7.5. Kernel werden für die tatsächlich vorhandene Architektur kompiliert (`target_env::ComputeCapability::jit_arch`), nie pauschal für SM 7.5.
- Die GPU-Subengine holt Kernel nur hier. Sie bekommt entweder `Kernel::Gpu` (Funktion eines geladenen Moduls) oder `Kernel::Library` (Eintrag aus `LibraryKernelSource`).
- Der Ressourcen-Lebenszyklus verwaltet das Register als Ressource mit fester ID (`RESOURCE_KEY`). Es wird lazy beim ersten Bedarf erzeugt.
- Datentyp, Dimension und Tile-Größe sind Compile-Zeit-Konstanten. Jede Kombination ist eine eigene Variante und ein eigener Cache-Eintrag.

## 3. Interface

```rust
use std::{any::Any, sync::Arc};
use layout::DType;
use device_lane_event::DeviceHandle;               // Crate-Pfad nach Projektkonvention
use target_env::ComputeCapability;
use library_adapter::{LibKernelEntry, LibraryKernelSource};
use resource_lifecycle::{ResourceRegistry, ResourceLoader, MemoryClass, LifecycleError};

pub const RESOURCE_KEY: &str = "kernel_register";

/// Architekturstufen.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ArchLevel { Sm61, Sm70, Sm75, Sm80, Sm86, Sm89 }
impl ArchLevel {
    pub fn capability(self) -> ComputeCapability;   // Sm61 -> 6.1 ... Sm89 -> 8.9
    pub fn jit_arch(self) -> String;                // "sm_61" ... "sm_89"
}
/// 6.1,7.0,7.5,8.0,8.6,8.9 exakt auf ihre Stufe; jede andere CC -> Sm75.
pub fn arch_level_for(cc: ComputeCapability) -> ArchLevel;

/// Compile-Zeit-Variante. None = nicht gesetzt (kein #define).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Variant { pub dim: Option<u32>, pub tile: Option<u32> }

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct KernelKey { pub name: String, pub dtype: DType, pub arch: ArchLevel, pub variant: Variant }

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KernelError {
    /// Keine Compute Capability (kein GPU/Device Cpu) oder Elementtyp nicht unterstützt.
    Unavailable(String),
    UnknownKernel(String),
    /// NVRTC-Fehler mit vollständigem Compiler-Log.
    CompileError { log: String },
    /// Treiber lehnt Modul ab (auch nach Fallback).
    ModuleLoadError(String),
    /// Modul geladen, aber Einstiegssymbol fehlt.
    MissingFunction { kernel: String, symbol: String },
}
impl std::fmt::Display for KernelError {}
impl std::error::Error for KernelError {}

/// JIT-Quelle. `body` wird hinter das Prelude gesetzt.
#[derive(Debug, Clone)]
pub struct KernelSource {
    pub name: &'static str,
    pub body: &'static str,
    pub entry: &'static str,                   // extern "C" __global__ Symbol
    pub dtypes: &'static [DType],              // unterstützte Elementtypen
    pub precompiled: Option<&'static [(ArchLevel, &'static [u8])]>, // cubin-Fallback
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CodeKind { Ptx, Cubin }
#[derive(Debug, Clone)]
pub struct CodeImage { pub kind: CodeKind, pub bytes: Vec<u8> }

#[derive(Debug, Clone)]
pub struct CompileRequest { pub file_name: String, pub source: String, pub arch: String /* "sm_75" */, pub kind: CodeKind }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompilerError { Unavailable(String), Failed { log: String } }
pub trait KernelCompiler: Send + Sync {
    fn compile(&self, req: &CompileRequest) -> Result<CodeImage, CompilerError>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoadFailure { PtxRejected(String), MissingSymbol(String), Other(String) }
/// Geladene Funktion; das Binding gibt sie opak aus, die Subengine castet per `as_any`.
pub trait KernelFunction: Send + Sync + std::fmt::Debug {
    fn symbol(&self) -> &str;
    fn as_any(&self) -> &dyn Any;
}
pub trait ModuleLoader: Send + Sync {
    /// Compute Capability des Devices; None, wenn nicht ermittelbar.
    fn device_capability(&self) -> Option<ComputeCapability>;
    fn load(&self, image: &CodeImage, symbol: &str) -> Result<Arc<dyn KernelFunction>, LoadFailure>;
}

#[derive(Clone, Debug)]
pub struct GpuKernel { pub key: KernelKey, pub function: Arc<dyn KernelFunction>, pub image_kind: CodeKind }
#[derive(Clone)]
pub enum Kernel { Gpu(Arc<GpuKernel>), Library(LibKernelEntry) }

pub struct KernelRegister { /* privat */ }   // Send + Sync (Compile-Time-Assertion)
impl KernelRegister {
    /// Ermittelt die CC einmalig: bei `DeviceHandle::Gpu(_)` über `loader.device_capability()`,
    /// bei `Cpu` ist sie None.
    pub fn new(device: DeviceHandle, compiler: Arc<dyn KernelCompiler>,
               loader: Arc<dyn ModuleLoader>, library: LibraryKernelSource) -> KernelRegister;
    pub fn register_source(&self, src: KernelSource) -> Result<(), KernelError>; // doppelter Name -> Err(Unavailable)
    pub fn device(&self) -> DeviceHandle;
    pub fn capability(&self) -> Option<ComputeCapability>;
    pub fn target_arch(&self) -> Option<ArchLevel>;     // arch_level_for(capability)
    /// Wie `get` mit `arch = target_arch`.
    pub fn request(&self, name: &str, dtype: DType, variant: Variant) -> Result<Kernel, KernelError>;
    pub fn get(&self, key: &KernelKey) -> Result<Kernel, KernelError>;
    pub fn len(&self) -> usize;          // Anzahl gecachter JIT-Einträge
    pub fn is_empty(&self) -> bool;
    pub fn clear(&self) -> usize;        // verwirft alle gecachten Einträge, gibt Anzahl zurück
}

/// Registriert das Register als Ressource unter RESOURCE_KEY (MemoryClass::Vram).
pub fn register_resource(
    registry: &ResourceRegistry<KernelRegister>,
    factory: Arc<dyn Fn() -> Result<KernelRegister, String> + Send + Sync>,
) -> Result<(), LifecycleError>;
```

## 4. Verhalten und Invarianten

**Zielarchitektur**
- `arch_level_for`: (6,1)→Sm61, (7,0)→Sm70, (7,5)→Sm75, (8,0)→Sm80, (8,6)→Sm86, (8,9)→Sm89. Alles andere ergibt Sm75, z. B. (9,0), (6,0), (7,2).
- Die CC wird nur in `new` ermittelt. Ohne CC ist `capability()` und `target_arch()` `None`.
- Ohne CC liefern JIT-Anfragen `Err(Unavailable)`. Library-Kernel bleiben abrufbar.

**Auswahl (`get`)**
1. Gibt es eine registrierte `KernelSource` mit diesem Namen, wird sie verwendet. Sie hat Vorrang vor Library-Kernels.
2. Sonst wird `library.get(name)` verwendet und `Kernel::Library` geliefert. Library-Kernel werden nicht kompiliert und nicht gecacht, sie ignorieren dtype, arch und variant.
3. Sonst `Err(UnknownKernel(name))`.
- Bei einer JIT-Quelle gilt: Ist `dtype` nicht in `src.dtypes` oder ist es `BF16`, kommt `Err(Unavailable("...dtype..."))`. Das gilt vor jeder Kompilierung.
- Liefert `Kernel::Gpu`, dann gilt `key == angefragter Key`. Gleiche Keys liefern dieselbe `Arc<GpuKernel>` (`Arc::ptr_eq`).

**Quelltext**
- Quelltext = Prelude(dtype, variant) + `body`.
- Das Prelude definiert:
  - `typedef T`: F32→float, F16→__half (mit `#include <cuda_fp16.h>`), U8→unsigned char, U32→unsigned int, I64→long long.
  - `float load_f(const T* p, size_t i)`, das nach float wandelt.
  - `float warp_reduce_sum(float)` und `float warp_reduce_max(float)` über `__shfl_xor_sync` mit voller Maske.
  - `#define DIM <n>` und `#define TILE <n>` nur für gesetzte Variantenfelder.
- `file_name` = `"<name>_<dtype>_<arch>[_d<dim>][_t<tile>].cu"`.

**Kompilieren und Laden (Miss)**
1. Kompiliere mit `CodeKind::Ptx` für `key.arch.jit_arch()`. Lade das Ergebnis mit `loader.load(image, src.entry)`.
2. `LoadFailure::PtxRejected`: Logge `warn!`. Kompiliere neu mit `CodeKind::Cubin` für exakt `capability().jit_arch()` (die echte Device-CC, nicht die Stufe) und lade erneut. Scheitert das, kommt `ModuleLoadError`.
3. `CompilerError::Unavailable` (NVRTC fehlt oder Treiber zu alt): Gibt es in `src.precompiled` ein Image für `key.arch`, wird es als `Cubin` geladen. Sonst kommt `ModuleLoadError(text)`.
4. `CompilerError::Failed{log}` ergibt `CompileError{log}`. Es gibt keinen Fallback.
5. `LoadFailure::MissingSymbol` ergibt `MissingFunction{kernel: name, symbol: entry}`. `LoadFailure::Other(t)` ergibt `ModuleLoadError(t)`.
6. Erfolg: Der Eintrag wird gecacht (`len()` steigt um 1) und per `info!` mit Name, dtype, arch, Variante und Dauer in ms geloggt. Fehler werden nie gecacht, eine spätere Anfrage versucht es erneut.

**Nebenläufigkeit**
- Pro Key läuft höchstens eine Kompilierung und ein Laden. Gleichzeitige Anfragen für denselben Key warten auf das Ergebnis des ersten und erhalten dasselbe Ergebnis (Erfolg: dieselbe `Arc`, Fehler: dasselbe `KernelError`). Cache-Hits blockieren nicht auf fremde Keys.
- Verschiedene Keys kompilieren parallel. Der globale Lock wird nicht während Compile oder Load gehalten.
- Eine Panic im Compiler oder Loader wird nicht propagiert, wenn sie abgefangen werden kann. Sie ergibt `ModuleLoadError`/`CompileError` mit Text, und wartende Threads blockieren nicht dauerhaft.
- Lock-Poisoning wird toleriert (Zustand wird übernommen, kein Panic).

**Cache-Abfragen**
- `len()` zählt erfolgreich gecachte JIT-Einträge, `is_empty() == (len()==0)`.
- `clear()` entfernt alle fertigen Einträge und gibt die vorherige Anzahl zurück. Ein zweites `clear` liefert 0.
- Bereits ausgegebene `Kernel`-Handles bleiben gültig.
- Eine laufende Kompilierung wird nicht abgebrochen. Ihr Ergebnis wird nach Abschluss normal eingetragen.
- Nach `clear` kompiliert dieselbe Anfrage erneut.

**Ressource**
- `register_resource` ruft `registry.register(RESOURCE_KEY.to_string(), MemoryClass::Vram, loader)` auf. Der Loader ruft `factory`, und `Err(text)` wird zum Load-Fehler.
- Ein doppeltes Registrieren ergibt `LifecycleError::AlreadyRegistered`.
- Es wird nichts beim Registrieren erzeugt.

**Fehlerzuordnung (Kurzform)**

| Situation | Fehler |
|---|---|
| keine CC / nicht unterstützter Elementtyp | `Unavailable` |
| Name weder JIT noch Library | `UnknownKernel` |
| NVRTC meldet Fehler | `CompileError{log}` |
| Treiber lehnt Modul ab (auch nach Fallback) | `ModuleLoadError` |
| Symbol fehlt im Modul | `MissingFunction` |

## 5. Relied-on elements

- `layout::DType {F32,F16,BF16,U8,U32,I64}` (`Copy, Eq, Hash`). `BF16` hat keinen Compute-Pfad.
- `target_env::ComputeCapability {major, minor}` (`Copy, Eq, Ord, Hash`), `ComputeCapability::new(u32,u32)`, `jit_arch() -> String` ("sm_75"). `jit_target_for` wird nicht verwendet, weil es CC < 7.0 ablehnt, 6.1 aber eine gültige Stufe ist.
- `device_lane_event::DeviceHandle {Cpu, Gpu(u32)}` (`Copy, Eq`).
- `library_adapter`:
  - `library_kernel_source() -> LibraryKernelSource` (vom Aufrufer an `new` übergeben).
  - `LibraryKernelSource::get(&self, name) -> Result<&LibKernelEntry, AdapterError>`.
  - `LibKernelEntry {name, arity, run}` ist `Clone`.
  - Einträge: "matmul", "add", "mul", "softmax_last_dim".
  - `Send + Sync + 'static`.
- `resource_lifecycle`:
  - `ResourceRegistry<H: Send + Sync + 'static>::register(&self, key: String, class: MemoryClass, loader: Arc<dyn ResourceLoader<H>>) -> Result<(), LifecycleError>`.
  - `trait ResourceLoader<H>: Send + Sync { fn load(&self) -> Result<H, String>; }`.
  - `MemoryClass::Vram`.
  - `LifecycleError::AlreadyRegistered`.