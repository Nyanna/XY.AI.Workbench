# Gemeinsame Subengine-Regeln: Technical Specification (build order 132)

## 1. Task, restrictions and boundaries

**Task.** Implement the Rust module `subengine_core`. It is the rule set and base implementation shared by the CPU and GPU sub-engines. A concrete sub-engine builds a `SubengineCore` from its hardware-specific parts: device, capability sets, lane factory, path policy and kernel register. The core then does the following:
- It implements the Planner's `Subengine` trait (`planner::Subengine`).
- It reports the capability sets and offers.
- It activates resources and lanes on the Planner's request. Activation is blocking and never loads anything.
- It registers the activated, compatible resources together with their lanes in the sub-engine's own queue and in the Shared queue.
- It owns the resources: encoders, scorers and index segments.
- It selects compute paths itself.
- It enforces the invariants listed in §4.

**Restrictions.**
- No tensors and no raw pointers in any public signature. Only `RegionHandle` and `Layout` are used.
- All GPU work goes through a `Lane` (execution context). Never use the default stream.
- Kernels are obtained only through `KernelRegister`. Never load kernels any other way.
- The module does not load models, encoders or indexes. Loading belongs to `resource_lifecycle` and `ModelRegistry`.
- It does not plan, choose queues, run workers, push into queues or claim items. The Planner assigns queues. The registrations only make resources and lanes visible to workers.
- It does not implement the Planner, the Expander or the Kernel-Register. It redefines no type from other modules.
- No `unsafe`, no globals. Crates: `log` only, plus the project modules named in §5.

## 2. Context

- **Planner** (build order 126): defines the `Subengine` trait. It calls `start`, then `capabilities()` and `offers()`. For each expanded operation it calls `announce(&ResourceRequirements)` (non-blocking). It then calls `activate()` (blocking, idempotent) on the deduplicated sub-engines that are assigned to the operation's queue and offer its kind. It reads `is_loaded` for status only. It assigns an operation to queue `q` only if some sub-engine of `q` has a capability set that satisfies the operation's requirements and offers the resource kind.
- **Queue**: workers of a queue (Cpu, Gpu, Shared) take items with `take_where` on `OpMeta`. This module does not touch `Queue`. It publishes `Registration`s (§3) that workers read to find a compatible resource and its lanes.
- **Resource-Requirements**: `ResourceRequirements` and `HasResourceRequirements` describe what an operation needs. `is_satisfied_by(kind, caps)` decides compatibility.
- **Operation**: operations carry `ResourceRequirements` and `OperationKind`. Their regions are `RamSlot` or `Pin<R>`. This module sees no data, only the requirements.
- **Region-Handle / Layout**: index segments are described only as `RegionHandle` plus `Layout`. Resolving a handle to a device address is the memory backend's job, never done here.
- **Modellregister**: provides `LoadedEncoder` (already loaded, owned here as `Arc`) and `Arc<Scorer>` through `ModelRegistry::scorer`. This module never calls `load_encoder`.
- **Device/Lane/Event, Kernel-Register, Capability-Modell**: used as in §5.

## 3. Interface

```rust
use std::sync::{Arc, Mutex};
use capability_model::{CapabilitySet, Name};
use device_lane_event::{DeviceHandle, Lane, LaneConfig, LaneKind};
use kernel_register::{Kernel, KernelError, KernelRegister, Variant};
use layout::{DType, Layout};
use model_registry::{LoadedEncoder, ModelRegistry};
use kernel_scoring::{Metric, Scorer};
use planner::Subengine;
use processor::QueueKind;
use region_handle::RegionHandle;
use resource_requirements::{IndexId, ModelId, ResourceKind, ResourceRequirements};

/// An exclusively usable lane. GPU lanes always have a stream.
pub type SharedLane = Arc<Mutex<Lane>>;

/// Index segment owned by the sub-engine. Region + Layout only.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IndexSegment { pub index: IndexId, pub region: RegionHandle, pub layout: Layout }

/// Chosen compute path. Produced only by the sub-engine's own PathPolicy.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ComputePath {
    pub kernel: String,        // name in the Kernel-Register
    pub dtype: DType,
    pub variant: Variant,
    pub lane: LaneKind,        // lane that must run it
}

/// Hardware-specific choice of the compute path. The sub-engine alone decides.
pub trait PathPolicy: Send + Sync {
    /// None = no path for these requirements on this hardware.
    fn select(&self, req: &ResourceRequirements, caps: &CapabilitySet, device: DeviceHandle)
        -> Option<ComputePath>;
}

/// Creates lanes. For GPU devices the returned Lane must have a stream.
pub trait LaneFactory: Send + Sync {
    fn create(&self, cfg: &LaneConfig) -> Result<Lane, String>;
}

/// One resource made visible to workers.
#[derive(Clone)]
pub struct Registration {
    pub subengine: String,
    pub queue: QueueKind,                 // the sub-engine's own queue or Shared
    pub kind: ResourceKind,
    pub model: Option<ModelId>,
    pub index: Option<IndexId>,
    pub capabilities: CapabilitySet,      // set the registration was checked against
    pub device: DeviceHandle,
    pub lanes: Vec<(LaneKind, SharedLane)>,
}

/// Registrations of all queues. Shared between all sub-engines via Arc. Send + Sync.
#[derive(Default)]
pub struct QueueRegistrations { /* private, short-lived locks */ }
impl QueueRegistrations {
    pub fn new() -> Self;
    /// Snapshot, in registration order.
    pub fn entries(&self, queue: QueueKind) -> Vec<Registration>;
    pub fn entries_of(&self, queue: QueueKind, kind: ResourceKind) -> Vec<Registration>;
}

pub struct SubengineConfig {
    pub name: String,                     // unique within one Planner
    pub device: DeviceHandle,
    pub own_queue: QueueKind,             // Cpu for DeviceHandle::Cpu, Gpu for DeviceHandle::Gpu
    pub own_capabilities: CapabilitySet,    // reported for own_queue
    pub shared_capabilities: CapabilitySet, // reported for Shared; must be a subset of own_capabilities
    pub offers: Vec<ResourceKind>,
    pub lanes: Vec<LaneConfig>,           // one lane per entry, all with device == config.device
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SubengineError {
    /// "ungültige Subengine-Konfiguration: {0}"
    InvalidConfig(String),
    /// "Lane {kind:?} nicht erstellbar: {reason}"
    LaneCreation { kind: LaneKind, reason: String },
    /// "GPU-Lane {kind:?} ohne Stream (Default-Stream ist verboten)"
    LaneWithoutStream { kind: LaneKind },
    /// "Subengine nicht aktiviert"
    NotActive,
    /// "kein Compute-Pfad für {kind:?}"
    NoPath { kind: ResourceKind },
    /// "Lane {0:?} nicht vorhanden"
    UnknownLane(LaneKind),
    /// Display of the inner error
    Kernel(KernelError),
    /// "Ressource passt nicht zur Subengine: {0}"
    Incompatible(String),
}
// Display + std::error::Error. Send + Sync + 'static.

pub struct SubengineCore { /* private */ }   // Send + Sync (static assertion in test)

impl SubengineCore {
    /// Validates only; creates no lane, starts nothing.
    pub fn new(cfg: SubengineConfig,
               lanes: Arc<dyn LaneFactory>,
               policy: Arc<dyn PathPolicy>,
               kernels: Arc<KernelRegister>,
               registry: Arc<ModelRegistry>,
               registrations: Arc<QueueRegistrations>) -> Result<Self, SubengineError>;

    pub fn is_active(&self) -> bool;
    pub fn device(&self) -> DeviceHandle;

    // Ownership of resources. Adopts handles that are already loaded; loads nothing.
    pub fn adopt_encoder(&self, model: ModelId, enc: Arc<LoadedEncoder>) -> Result<(), SubengineError>;
    pub fn adopt_scorer(&self, metric: Metric) -> Result<(), SubengineError>;      // Arc<Scorer> from registry.scorer(metric)
    pub fn adopt_index_segment(&self, seg: IndexSegment) -> Result<(), SubengineError>;
    pub fn encoder(&self, model: &ModelId) -> Option<Arc<LoadedEncoder>>;
    pub fn scorer(&self, metric: Metric) -> Option<Arc<Scorer>>;
    pub fn index_segments(&self, index: &IndexId) -> Vec<IndexSegment>;

    // Work. Everything goes through these.
    pub fn select_path(&self, req: &ResourceRequirements) -> Result<ComputePath, SubengineError>;
    /// Kernel only via KernelRegister::request(path.kernel, path.dtype, path.variant).
    pub fn kernel(&self, path: &ComputePath) -> Result<Kernel, SubengineError>;
    /// Runs `f` with exclusive access to the lane of that kind.
    pub fn with_lane<T>(&self, kind: LaneKind, f: impl FnOnce(&mut Lane) -> T) -> Result<T, SubengineError>;
}

impl Subengine for SubengineCore {
    fn name(&self) -> String;
    fn start(&self) -> Result<(), String>;
    fn capabilities(&self) -> Vec<(QueueKind, CapabilitySet)>;
    fn offers(&self) -> Vec<ResourceKind>;
    fn announce(&self, req: &ResourceRequirements);
    fn activate(&self) -> Result<(), String>;
    fn is_loaded(&self, kind: ResourceKind, model: Option<&ModelId>) -> bool;
}
```

## 4. Behavior and invariants

**Configuration (`new`).** Errors are `InvalidConfig`, and the first one wins:
- `own_queue` must be `Cpu` for `DeviceHandle::Cpu` and `Gpu` for `DeviceHandle::Gpu`. `Shared` is never allowed.
- `shared_capabilities.is_within(&own_capabilities)` must hold.
- `own_capabilities.is_within(&gpu_reference())` for a GPU device, or `is_within(&cpu_reference())` for the CPU. Both are `capability_model` functions.
- Every `LaneConfig.device` must equal `cfg.device`.
- Lane kinds must be unique.
- The name must be non-empty.

**start.**
- Idempotent. It creates no lane, loads nothing and spawns nothing.
- It returns `Ok(())` after a successful `new`.

**capabilities / offers.**
- `capabilities()` returns exactly `[(own_queue, own_capabilities), (Shared, shared_capabilities)]`. The Shared entry is omitted if `shared_capabilities` is empty.
- `offers()` returns `cfg.offers` unchanged and without duplicates. It does not depend on activation or loading.
- Both calls are callable at any time and have no side effects.

**announce.**
- Non-blocking and infallible. It only records the requirements, deduplicated, for the next `activate`.
- It creates no lane, registers nothing and loads nothing.

**activate** (blocking, idempotent). Steps, in order:
1. Create every configured lane once through `LaneFactory::create`, and wrap it in `SharedLane`.
   - On a GPU device, a lane with `stream() == None` gives `LaneWithoutStream`.
   - A factory `Err` gives `LaneCreation`.
2. Mark the engine active.
3. Register the owned resources (see "Registration" below).
4. Return `Err(Display of the SubengineError)` on failure.

Further rules for `activate`:
- A failure leaves the engine not active and keeps no half-created lanes. A later call retries. Errors are not cached.
- Once active, a repeated call creates no new lanes and registers nothing twice.
- Concurrent calls are serialized. The later call waits and then sees the result of the first one.
- Activation never loads, compiles or allocates a resource other than the lanes' own buffers. Announced requirements that no owned resource matches do not cause an error.

**Registration.**
- A resource is *compatible* with a queue's capability set `C` iff `ResourceRequirements::is_satisfied_by(device.kind(), &C)` holds for its requirements and `kind ∈ offers`.
  - `C` is `own_capabilities` for the own queue.
  - `C` is `shared_capabilities` for Shared.
- The requirements used for the check are:
  - for an encoder, `Encoder` with its `ModelId`;
  - for a scorer, `Scorer`;
  - for an index segment, `Index` with its `IndexId`.
- Each compatible resource gets one `Registration` per compatible queue, in the own queue and/or in Shared, into the shared `QueueRegistrations`. It carries all lanes of this engine.
- A resource that is compatible with neither queue is owned but not registered. It is not an error.
- `adopt_*` before activation only stores the resource. After activation it registers it at once, with the same rules.
- Registrations are never duplicated. Adopting the same encoder model, scorer metric or index segment again replaces the owned value and keeps a single registration.
- `adopt_encoder` and `adopt_index_segment` with a kind that is not in `offers` give `Incompatible`.
- `adopt_scorer` gives `Incompatible` if the registry has no scorer for the metric.

**Ownership.**
- The sub-engine owns its encoders, scorers and index segments as `Arc`s. The `Arc<LoadedEncoder>` is held until the core is dropped.
- Dropping the core drops its owned resources and its lane references. Registrations already handed out stay valid, because they hold `Arc`s.
- No `RegionHandle` is ever resolved or dereferenced in this module.

**is_loaded** (read only, never activates).
- `Encoder`: true iff an encoder for the model is owned. `model == None` means any model.
- `Scorer`: true iff at least one scorer is owned.
- `Index`: true iff at least one segment is owned.
- Other kinds (Tokenizer, Normalizer, CrossEncoder, unknown): true iff the engine is active and the kind is in `offers`.
- A kind that is not offered is always false.

**Compute path.**
- `select_path` first checks that `req.environment().accepts(device.kind())`. If it does not, the result is `NoPath`. It then calls `PathPolicy::select`. `None` gives `NoPath{kind}`.
- Nobody outside the sub-engine chooses the path, and the core never overrides the policy.
- `kernel(path)` calls `KernelRegister::request(&path.kernel, path.dtype, path.variant)` and maps an error to `Kernel(e)`. No other load path exists.
- The register is not asked at `new`, `start`, `announce` or `activate`.

**Lanes.**
- `with_lane` locks the lane for the closure, so exclusive per-lane state (`params_mut`, `workspace_mut`) is used under `&mut`. An unknown kind gives `UnknownLane`, and a not-active engine gives `NotActive`.
- All GPU work runs inside `with_lane` on a lane that has a stream. The default stream is never used.
- A panic inside `f` propagates. A poisoned lane lock is recovered and does not panic the core.
- Lanes of different kinds can be used concurrently. One lane is never used by two closures at once.

**Concurrency and errors.**
- All methods take `&self` and are callable from any thread.
- State sits behind short-lived locks. No state lock is held across `LaneFactory::create` (the activation lock serializes it), `PathPolicy::select`, `KernelRegister` calls or the `with_lane` closure.
- `announce`, `is_loaded`, `capabilities`, `offers` and the getters never block on `activate`.
- Equal configuration, adoptions and announcements give equal registrations.
- No public signature contains a tensor, a raw pointer or a device address.
- No panic on valid use. The error type is `SubengineError`. The `Subengine` trait methods return `Err(String)` carrying its Display text.

## 5. Relied-on elements

- **Planner:** `trait Subengine: Send + Sync` with these methods:
  - `name() -> String`
  - `start() -> Result<(), String>` (idempotent)
  - `capabilities() -> Vec<(QueueKind, CapabilitySet)>`
  - `offers() -> Vec<ResourceKind>`
  - `announce(&ResourceRequirements)` (non-blocking)
  - `activate() -> Result<(), String>` (blocking, idempotent)
  - `is_loaded(ResourceKind, Option<&ModelId>) -> bool`
  
  The Planner calls `activate` only when operations are created, never at `start` or `build`. `QueueKind { Cpu, Gpu, Shared }` comes from `processor`.
- **Resource-Requirements:**
  - `ResourceRequirements::{new, kind, environment, model, index, capabilities, is_satisfied_by(DeviceKind, &CapabilitySet) -> bool}`.
  - `ResourceKind { Tokenizer, Normalizer, Encoder, Scorer, Index, CrossEncoder }` and `ExecutionEnvironment` are `non_exhaustive`. Use a wildcard arm for unknown variants.
  - `ExecutionEnvironment::accepts(DeviceKind)`.
  - `ModelId` and `IndexId` have `new` and `as_str`, and are `Clone + Eq + Hash`.
- **Capability-Modell:** `CapabilitySet::{new, add, is_within, satisfies, iter, is_empty}`, `Clone + Eq`. Also `gpu_reference()`, `cpu_reference()` and `Name`.
- **Device/Lane/Event:**
  - `DeviceHandle { Cpu, Gpu(u32) }` with `kind()`.
  - `DeviceKind { Cpu, Gpu }`.
  - `LaneKind { Query, Insert, Update, CopyH2D, CopyD2H }` (`Copy, Eq, Hash`).
  - `LaneConfig { kind, device, priority, param_block_size, workspace_size }`.
  - `Lane` is `Send + Sync`, and `Lane::stream() -> Option<&dyn StreamBackend>`. A CPU lane has none.
  - `Lane::{kind, device, params_mut, workspace_mut, ensure_workspace, record_event}`. Exclusive state needs `&mut`.
- **Kernel-Register:**
  - `KernelRegister: Send + Sync` is the only way to load GPU kernels.
  - `request(&self, name: &str, dtype: DType, variant: Variant) -> Result<Kernel, KernelError>`.
  - `Variant { dim: Option<u32>, tile: Option<u32> }`.
  - `Kernel { Gpu(Arc<GpuKernel>), Library(LibKernelEntry) }`.
  - `KernelError` has Display.
  - BF16 has no compute path. The register rejects it with `Unavailable`.
- **Modellregister:**
  - `LoadedEncoder: Send + Sync + 'static`, read-only.
  - `ModelRegistry: Send + Sync` and shared as `Arc`.
  - `scorer(Metric) -> Result<Arc<Scorer>, RegistryError>`.
  - `scorer_available(Metric) -> bool`.
  - `Metric { Dot, NegL2Sq }`. `Scorer` is `Send + Sync` and stateless.
- **Region-Handle:** `RegionHandle` is a `Copy`, pointer-free value (`Eq + Hash`, `Send + Sync`). It is never resolved here.
- **Layout und Elementtypen:** `Layout` (`Clone + Eq`, `Send + Sync`), `DType { F32, F16, BF16, U8, U32, I64 }` (`Copy + Eq + Hash`).
- **Operation:** the module only sees `ResourceRequirements` through the Planner's `announce`. It does not use `Operation<R>`.
- **Queue:** no API is used. Queues are only a label, `QueueKind`. Workers read `QueueRegistrations`.