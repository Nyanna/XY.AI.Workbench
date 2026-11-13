# Spezifikation: GPU-Backend (Build-Order 73)

## 1. Aufgabe, Einschränkungen, Grenzen

**Aufgabe.** Das Rust-Modul `gpu_backend` ist das Memory-Backend für ein Nvidia-Device. Es erledigt Folgendes:
- Es legt VRAM-Arenen als je einen zusammenhängenden, genullten Block an, mit Fit-Check vorab.
- Es stellt Mapped Host-Memory bereit.
- Es führt D2D-, H2D- und D2H-Transfers asynchron auf einem eigenen Copy-Stream aus, mit Pinned Buffern und Events.
- Es löst Regionen intern auf Device-Adressen auf. Diese Adressen verlassen das Backend nicht. Ausnahmen sind `mapped_device_addr` und die Adresse im `DeviceBlock` an den Library-Adapter.
- Es erfüllt die gemeinsame Backend-Schnittstelle `MemoryBackend` (Abschnitt 3).

**Nicht Teil des Moduls:**
- Keine Platzierung, Generationsvergabe oder Liveness-Logik. Das macht `Bookkeeping`, der Allocator nutzt es.
- Keine Eviction, keine Policy, keine Kernel-Starts.
- Keine Redefinition von `ArenaKind`, `Arena`, `RegionHandle`, `Region`, `Event`, `Lane`, `AccessError`, `Budget` oder `MemInfo`.
- Keine CPU-Variante.
- Kein Retry und kein Fallback bei Treiberfehlern.

**Treiberzugriff.** Dieses Modul darf zusätzlich zum Library-Adapter die CUDA-Driver-API über `cudarc` nutzen (Crate bereits im Manifest, nichts hinzufügen oder umversionieren). Es darf `candle_core::cuda_backend::CudaDevice` ausschließlich zum Füllen von `DeviceBlock` erzeugen. Beide teilen den Primary-Context des Ordinals.

## 2. Kontext

- **`arenas`:** `ArenaSet`, `ArenaCapacities`, `fit_check`, `Budget`, `MemoryQuery`, `MemInfo`.
- **`bookkeeping`:** `Bookkeeping` mit `Region` (Generation `u32`).
- **`allocator`:** `AllocatorSet` und `Allocator` mit Deferred Free.
- **`region_handle`:** `RegionHandle`, `Generation(u64)`. Das ist das öffentliche Handle dieses Backends.
- **`memory_access`:** `AccessError`.
- **`device`:** `DeviceHandle`, `DeviceKind`, `Event`, `GpuEventBackend`, `StreamBackend`, `Lane`, `LaneConfig`, `LaneKind`.
- **`library_adapter`:** `DeviceBlock`, `LibTensor`, `wrap_region`. Das Backend besitzt Context und Blöcke und gibt sie erst nach allen gewrappten Tensoren frei.
- **Aufrufer:** Lader, Planer und Kernels. Sie bekommen nur `RegionHandle`, `Event` und Fehler.
- **Logging:** über die `log`-Crate.

## 3. Interface

```rust
use arenas::{ArenaCapacities, ArenaKind, Budget, BudgetError};
use region_handle::RegionHandle;
use allocator::AllocatorError;
use memory_access::AccessError;
use device::{DeviceHandle, Event};
use layout::Layout;
use library_adapter::{AdapterError, LibTensor};

pub const ALLOC_GRANULE: u64 = 256; // Bytes

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TransferDirection { D2D, H2D, D2H }

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BackendError {
    /// Display: "Device-Speicher nicht verfügbar" (+ ": {0}" mit Treiber-/Grundtext)
    DeviceMemoryUnavailable(String),
    /// Display enthält Richtung und Begründung
    Transfer { direction: TransferDirection, reason: String },
    /// Display: "stale Region"
    StaleRegion,
    /// Region gehört nicht zur erwarteten Arena (Mapped-Host-Zugriff)
    ArenaMismatch { expected: ArenaKind, found: ArenaKind },
    /// Fit-Check / Speicherabfrage
    Budget(BudgetError),
    /// Fehler aus Allocator (Allokation, Free)
    Alloc(AllocatorError),
    /// Bereichsfehler bei Mapped-Host-Zugriff
    Access(AccessError),
    Adapter(AdapterError),
}
impl std::fmt::Display for BackendError {}
impl std::error::Error for BackendError {}

/// Gemeinsame nebenläufige Backend-Schnittstelle.
pub trait MemoryBackend: Send + Sync {
    /// Allokation in einer Arena. `align` Potenz von 2, 1..=256 (0 gilt als 1).
    fn allocate(&self, arena: ArenaKind, size: u64, align: u64) -> Result<RegionHandle, BackendError>;
    fn free(&self, region: &RegionHandle) -> Result<(), BackendError>;
    /// Frei erst, wenn `event` abgeschlossen ist.
    fn free_after(&self, region: &RegionHandle, event: Event) -> Result<(), BackendError>;
    /// Frisch vom Treiber; generation_reserve = 0.
    fn budget(&self) -> Result<Budget, BackendError>;
}

/// Pinned, genullter Host-Buffer (cuMemHostAlloc). Send + Sync.
pub struct PinnedBuffer { /* ptr, len */ }
impl PinnedBuffer {
    pub fn len(&self) -> usize;
    pub fn is_empty(&self) -> bool;
    pub fn as_slice(&self) -> &[u8];
    pub fn as_mut_slice(&mut self) -> &mut [u8];
}
// Drop: cuMemFreeHost; Fehler/ungebundener Context werden ignoriert (Leak statt Panic).

/// Ring aus Pinned, genullten Buffern.
pub struct StagingRing { /* Vec<PinnedBuffer>, AtomicUsize cursor */ }
impl StagingRing {
    pub fn len(&self) -> usize;          // Anzahl
    pub fn buffer_size(&self) -> usize;  // Größe je Buffer
    pub fn get(&self, i: usize) -> Option<&PinnedBuffer>;
    pub fn get_mut(&mut self, i: usize) -> Option<&mut PinnedBuffer>;
    /// Round-robin-Index 0..len (atomar); bei len == 0 -> None.
    pub fn next_index(&self) -> Option<usize>;
}

/// Host-Quelle für Upload.
pub enum HostSource<'a> { Pageable(&'a [u8]), Pinned(&'a PinnedBuffer) }

/// Send + Sync.
pub struct GpuBackend { /* siehe Verhalten */ }

impl GpuBackend {
    /// Legt das Backend an (Ablauf siehe Abschnitt 4). `headroom` z. B. arenas::DEFAULT_HEADROOM.
    pub fn new(device: DeviceHandle, caps: ArenaCapacities, headroom: u64)
        -> Result<GpuBackend, BackendError>;

    pub fn device(&self) -> DeviceHandle;
    /// Kapazitäten, wie angelegt (Staging == 0).
    pub fn capacities(&self) -> ArenaCapacities;

    /// D2D auf dem Copy-Stream: kopiert `len` Bytes ab Offset 0 der Regionen.
    pub fn copy_d2d(&self, src: &RegionHandle, dst: &RegionHandle, len: u64)
        -> Result<Event, BackendError>;
    /// H2D: kopiert src.len() Bytes an den Anfang von `dst`.
    pub fn upload(&self, dst: &RegionHandle, src: HostSource<'_>) -> Result<Event, BackendError>;
    /// D2H: kopiert dst.len() Bytes vom Anfang von `src` in `dst`.
    pub fn download(&self, src: &RegionHandle, dst: &PinnedBuffer) -> Result<Event, BackendError>;

    pub fn create_staging_ring(&self, count: usize, buffer_size: usize)
        -> Result<StagingRing, BackendError>;

    /// Mapped Host-Memory ohne Transfer lesen/schreiben (Region in Arena MappedHost).
    pub fn mapped_read(&self, region: &RegionHandle, offset: usize, dst: &mut [u8])
        -> Result<(), BackendError>;
    pub fn mapped_write(&self, region: &RegionHandle, offset: usize, src: &[u8])
        -> Result<(), BackendError>;
    /// Device-Adresse des Mapped-Host-Blocks (Basis) für Kernel-Launches; None ohne Block.
    pub fn mapped_device_addr(&self) -> Option<u64>;

    /// Hüllt eine lebende Region als Library-Tensor ein (löst intern auf, ruft wrap_region).
    /// # Safety: Wie `library_adapter::wrap_region`: Tensor und alle Views müssen vor
    /// `shutdown`/Drop des Backends verschwunden sein.
    pub unsafe fn wrap_tensor(&self, region: &RegionHandle, layout: &Layout)
        -> Result<LibTensor, BackendError>;

    /// Idempotent; wird auch von Drop aufgerufen.
    pub fn shutdown(&self);
}
impl MemoryBackend for GpuBackend {}
impl Drop for GpuBackend {}
```

`GpuBackend`, `PinnedBuffer`, `StagingRing` und `BackendError` sind `Send + Sync + 'static` (Compile-Time-Assertion im Test).

## 4. Verhalten und Invarianten

**Anlegen (`new`), Reihenfolge strikt**
1. Wenn `device` nicht `DeviceHandle::Gpu(_)` ist, oder Treiberinitialisierung bzw. Context-Anlage scheitert: `DeviceMemoryUnavailable`. Es wird nichts reserviert.
2. Speicherabfrage per `MemoryQuery`, die bei jedem Aufruf frisch `cuMemGetInfo` ausführt. Ein Fehler ergibt `Budget(MemoryQueryFailed)`.
3. `fit_check(&caps, free, headroom)`. Bei Misserfolg `Budget(DoesNotFit{..})`. Es wird nichts reserviert.
4. `Staging`-Kapazität wird auf 0 gesetzt, im `ArenaSet` und damit im `AllocatorSet`. `capacities()` zeigt das.
5. Alle sechs Arenen werden per `ArenaSet::ensure` angelegt, mit `Budget{free,total,generation_reserve:0}`. Staging hat Kapazität 0 und ist logisch vorhanden. Der `AllocatorSet` enthält alle sechs Arenen (`DeviceKind::Gpu`) mit einem gemeinsamen `Bookkeeping`.
6. Je Device-Arena (Weights, IndexSegments, Workspace, ParameterBlock) mit Kapazität > 0 entsteht ein zusammenhängender Block in voller Kapazität. Er wird stream-ordered allokiert (`cuMemAllocAsync`) und per `cuMemsetD8Async` genullt. Arenen mit Kapazität 0 erhalten keinen Block.
7. MappedHost mit Kapazität > 0 bekommt einen pinned (`cuMemHostAlloc`, Device-Map-Flag), genullten Host-Block mit eigener Device-Adresse (`cuMemHostGetDevicePointer`). Scheitert das Mapping, wird der Host-Block freigegeben und der Fehler gemeldet.
8. Bei jedem Fehler in 6–7 werden alle bereits angelegten Blöcke freigegeben. Der Fehler ist `DeviceMemoryUnavailable(Treibertext)`.
9. Der Release-Threshold des Default-Mempools des Devices wird auf `u64::MAX` gesetzt. Stream-ordered freigegebener Speicher bleibt im Pool, und im Dauerbetrieb findet keine Treiber-Allokation statt. `cuMemPoolTrim` wird nie aufgerufen.
10. Es wird auf den Abschluss aller Vorgänge gewartet (Stream- und Context-Synchronize). Erst dann kehrt `new` zurück.
11. Es wird geloggt: freier und gesamter Speicher sowie alle Arena-Größen in MiB (`info!`).

**Copy-Lane**
- `new` erzeugt einen eigenen, non-blocking Copy-Stream, getrennt vom Compute-Stream.
- Der Stream steckt in einer `Lane` (`LaneConfig::preset(LaneKind::CopyH2D, device)`) mit einem `StreamBackend`, das ein `CUevent` aufzeichnet.
- Alle drei Transfer-Arten laufen auf diesem einen Stream. Jeder Transfer zeichnet danach per `Lane::record_event` das zurückgegebene `Event` auf. Enqueue und Event-Aufzeichnung geschehen unter demselben Lock, damit das Event den Transfer sicher abdeckt. Ein `LaneError` wird zu `Transfer{direction, reason}`.

**Budget**
- `budget()` fragt bei jedem Aufruf den Treiber neu ab, nutzt keinen Cache und loggt das Ergebnis auf `debug!`. Ein Fehler ergibt `Budget(MemoryQueryFailed)`.

**Allokation und Freigabe**
- `allocate` rundet `size` auf ein Vielfaches von `ALLOC_GRANULE`. Dadurch ist jeder Offset ein Vielfaches von 256 und erfüllt jedes erlaubte `align`.
- `allocate` liefert `RegionHandle{arena, offset, len = gerundete Größe, Generation(region.generation as u64)}`.
- Fehler:
  - `size == 0`, Überlauf beim Runden, `align` keine Potenz von 2 oder `> 256`: `Alloc(AllocationFailed)`.
  - Sonstige Fehler: Abbildung durch den `Allocator` (`Alloc(OutOfMemory{..})` usw.).
- Konvertierung `RegionHandle → bookkeeping::Region`: Ist `generation > u32::MAX`, gilt das Handle als stale.
- `free` und `free_after` delegieren an `AllocatorSet::allocator_for(..)`, `free` bzw. `free_deferred`. Fehler werden als `Alloc(StaleRegion)` usw. durchgereicht.
- Ein Handle mit nicht passender Generation, einer anderen Arena oder einer Arena ohne Block ist nicht live.

**Liveness und Auflösung (intern)**
- Eine Region ist verwendbar, wenn `Allocator::is_live` wahr ist, die Arena einen Block hat und `offset.checked_add(len) <= block_len` gilt.
- Sonst kommt `StaleRegion`. Das gilt auch nach `shutdown`.
- Die Adresse ist `block_base + offset`, per `checked_add` berechnet.

**D2D (`copy_d2d`)**
Prüfreihenfolge:
1. `src.arena() == dst.arena()`: `Transfer{D2D, "same arena"}`.
2. Region nicht live oder Block fehlt: `StaleRegion`.
3. `len > src.len()` oder `len > dst.len()`: `Transfer{D2D, "size mismatch"}`.
4. Treiberfehler: `Transfer{D2D, Treibertext}`.

Details:
- `len == 0` ist ein gültiger No-op, liefert aber ein Event.
- Locks: Es gibt ein Lock je Arena-Block. Bei D2D werden beide in aufsteigender `ArenaKind`-ID genommen, danach das Copy-Stream-Lock. `shutdown` nimmt dieselbe Reihenfolge. Daher gibt es keinen Deadlock.

**Upload (H2D)**
- Prüfreihenfolge: nicht live oder Block fehlt: `StaleRegion`. `src.len() > dst.len()`: `Transfer{H2D, "size mismatch"}`. Treiberfehler: `Transfer{H2D, Treibertext}`.
- `HostSource::Pinned`: Der Buffer muss bis zum zurückgegebenen Event unverändert und am Leben bleiben (Aufrufer-Pflicht).
- `HostSource::Pageable`: Die Quelle darf nach Rückkehr von `upload` wiederverwendet werden. Der Treiber hat sie bis dahin gestaged.
- Länge 0 ist ein No-op mit Event.

**Download (D2H)**
- Prüfreihenfolge: nicht live oder Block fehlt: `StaleRegion`. `dst.len() > src.len()`: `Transfer{D2H, "size mismatch"}`. Treiberfehler: `Transfer{D2H, Treibertext}`.
- Der Inhalt von `dst` ist erst nach Abschluss des Events lesbar.

**Staging-Ring**
- `create_staging_ring(count, size)` legt `count` pinned, genullte Buffer à `size` Bytes an.
- `count == 0` ergibt einen leeren Ring. `size == 0` ergibt Buffer der Länge 0 ohne Treiberaufruf.
- Überlauf von `count * size` oder ein Treiberfehler ergibt `DeviceMemoryUnavailable`. Teilweise angelegte Buffer werden freigegeben.

**Mapped Host-Zugriff**
- Voraussetzungen, in dieser Reihenfolge:
  1. `region.arena() != MappedHost`: `ArenaMismatch`.
  2. Region nicht live: `StaleRegion`.
  3. Kein Mapped-Block: `StaleRegion`.
  4. `offset.checked_add(buf.len()) > region.len()`, oder die Endposition `region.offset() + offset + buf.len()` überläuft oder liegt hinter `block_len`: `Access(OutOfBounds{offset, len, region_len})`.
- Zugriff ist `memcpy` zwischen dem Host-Block ab `region.offset() + offset` und dem Slice, ohne Transfer und ohne Treiberaufruf. Länge 0 ist ein No-op.
- Überlappende Zugriffe und die Synchronisation mit Device-Zugriffen sind Sache des Aufrufers. `&self` erlaubt nebenläufige Aufrufe.
- `mapped_device_addr` liefert die Device-Adresse des Blockbeginns, oder `None`, wenn die Kapazität 0 ist oder nach `shutdown`.

**`wrap_tensor`**
- Prüft Liveness wie oben, baut `DeviceBlock{device: CudaDevice, base_addr, len_bytes}` der Arena und ruft `library_adapter::wrap_region`.
- `AdapterError` wird zu `Adapter(..)`.
- Eine Arena ohne Block ergibt `StaleRegion`.

**Fehlerabbildung**
- Treiberfehler bei Anlage, Ring oder Mapping ergeben `DeviceMemoryUnavailable(text)`.
- Treiberfehler bei Transfers ergeben `Transfer{direction, reason}`.
- Fehler ändern keinen Zustand. Ausnahme ist die Rollback-Freigabe in `new`.

**Shutdown**
- Zuerst wird der Copy-Stream synchronisiert.
- Danach werden Device-Blöcke stream-ordered freigegeben (zurück in den Default-Pool, ohne Trim).
- Der Mapped-Host-Block wird freigegeben, sofern der Context noch an den Thread bindbar ist. Sonst wird die Freigabe ohne Fehler übersprungen.
- Danach sind alle Blöcke weg. Operationen auf Regionen ergeben `StaleRegion`.
- Ein zweites `shutdown` ist ein No-op.

**Nebenläufigkeit**
- Alle Methoden nehmen `&self` und sind thread-safe.
- Lockreihenfolge: Arena-Locks aufsteigend nach ID, dann Copy-Stream-Lock, dann (intern) Allocator/Bookkeeping.
- Vergiftete Locks werden übernommen (kein Panic).
- Es laufen keine Callbacks unter den Locks.
- Transfers verschiedener Threads werden auf dem Copy-Stream in Lock-Reihenfolge serialisiert. Jedes Event deckt alle bis dahin eingereihten Transfers ab.

**Tests.** Alle Tests, die ein Device brauchen, laufen nur auf echtem Nvidia-Device und brechen sonst sauber ab. Ohne Device sind prüfbar:
- `new(DeviceHandle::Cpu, ..)` ergibt `DeviceMemoryUnavailable`.
- Fehlerabbildungen und Display-Texte.
- Handle-Konvertierung mit Generation > `u32::MAX`.
- Send/Sync-Assertions.

## 5. Relied-on Elements

- **`arenas`:**
  - `ArenaKind::{Weights=0, IndexSegments=1, Workspace=2, Staging=3, ParameterBlock=4, MappedHost=5}` (`ALL`, `id()`, `is_device()`).
  - `ArenaCapacities::{get,set,device_need}` (Default 0).
  - `MemInfo{free,total}`, `trait MemoryQuery: Send+Sync { fn query(&self)->Result<MemInfo,String>; }`.
  - `Budget{free,total,generation_reserve}` mit `from_query` und `usable`.
  - `fit_check(&caps, free, headroom)->Result<(),BudgetError>` mit `requested <= free.saturating_sub(headroom)`; `BudgetError::{MemoryQueryFailed{reason}, DoesNotFit{requested,usable}}`.
  - `DEFAULT_HEADROOM` = 512 MiB, `MIB`.
  - `ArenaSet::{new, set_capacity, ensure(k,&Budget)->Result<Arena,BudgetError>, get}`. `ensure` ist lazy und idempotent. Device-Arenen werden gegen `budget.usable()` geprüft, Host-Arenen nicht. Kapazität 0 ist erlaubt.
- **`region_handle`:**
  - `RegionHandle` (`Copy`, `Eq`, `Hash`) mit `arena()`, `offset()`, `len()`, `generation()`, `end()`, `is_current`, `new(arena, offset, len, Generation)->Result`.
  - `Generation(pub u64)`.
- **`bookkeeping`:**
  - `Region{arena,offset,len,generation:u32}`.
  - `Bookkeeping` (first-fit, Alignment 1, `is_live`, `live_bytes`, Fault Injection).
  - Alle Methoden sind atomar.
- **`allocator`:**
  - `AllocatorSet::new(Arc<Bookkeeping>, &[(Arena, DeviceKind)])`, `allocator_for(&Region)->Result<&Allocator,AllocatorError>`, `allocator(kind)`.
  - `Allocator::{allocate(len)->Result<Region,AllocatorError>, free(&Region), free_deferred(&Region, Event), is_live(&Region)}`.
  - `AllocatorError::{OutOfMemory{arena,requested,free}, AllocationFailed ("Allokation fehlgeschlagen"), StaleRegion ("stale Region")}`.
  - Deferred Free gibt erst nach Abschluss des Events frei; vorgemerkte Regionen bleiben live.
- **`memory_access`:** `AccessError::OutOfBounds{offset: usize, len: usize, region_len: usize}`.
- **`device`:**
  - `DeviceHandle::{Cpu, Gpu(u32)}`, `DeviceKind::{Cpu, Gpu}`.
  - `Event` (`Clone+Send+Sync`) mit `Event::gpu(ordinal, Box<dyn GpuEventBackend>)`. `trait GpuEventBackend: Send+Sync+Debug { fn query(&self)->Result<bool,String>; fn synchronize(&self)->Result<(),String>; }`.
  - `trait StreamBackend: Send+Sync+Debug { fn record_event(&self)->Result<Box<dyn GpuEventBackend>,String>; }`.
  - `Lane::new(LaneConfig, Option<Box<dyn StreamBackend>>)`, `Lane::record_event(&self)->Result<Event,LaneError>`.
  - `LaneConfig::preset(LaneKind, DeviceHandle)`, `LaneKind::CopyH2D`.
  - Ein Event deckt alle vor der Aufzeichnung auf der Lane eingereihte Arbeit ab.
- **`library_adapter`:**
  - `DeviceBlock{device: candle_core::cuda_backend::CudaDevice, base_addr: u64, len_bytes: u64}`.
  - `unsafe fn wrap_region(&DeviceBlock, &RegionHandle, &Layout)->Result<LibTensor,AdapterError>`.
  - Der Adapter prüft Grenzen, Contiguity, Ausrichtung. Gültigkeit und Generation prüft das Backend vorher. Block und Context müssen mindestens so lange leben wie jeder gewrappte Tensor.
- **`target_env`:** Zielhardware ist GTX 1660 SUPER (CC 7.5). Die Treiber-API ist CUDA 13.3. Stream-ordered Allocation und Default-Mempool sind darauf verfügbar.