# GPU-Subengine: Technical Specification (build order 139, module `gpu_subengine`)

## 1. Task, restrictions and boundaries

**Task.** Implement the Rust module `gpu_subengine`, the CUDA implementation of a sub-engine. It consists of three components.

1. **`GpuManager`** is the GPU manager and a separate component.
   - It enumerates the CUDA devices through an injected `GpuBackend`.
   - It retains exactly **one primary context per GPU** and keeps it for its own lifetime.
   - It builds one `GpuSubengine` per usable GPU.
2. **`GpuSubengine`** is the sub-engine of one GPU.
   - It composes `SubengineCore` (build order 132) with GPU parts: device, capability sets, lane factory, FP16-first path policy and the shared `KernelRegister`.
   - It has five lanes (Query, Insert, Update, CopyH2D, CopyD2H). Each lane has its own stream.
   - It builds lane resources (`LaneResource`) with extra worker lanes for batched execution.
   - It implements `planner::Subengine` by delegating to the core.
3. **`VramManager`** manages the VRAM of one GPU.
   - **Arenas.** It reserves the six arenas once, checks the budget, and keeps the Weights extents.
   - **Models.** It keeps encoder models in a swappable VRAM lifecycle registry (TTL).
   - **Index segments.** It streams them through `manager::Manager` in batches.
   - It is the bridge between the index in RAM and its VRAM representation.

**Restrictions.**
- The module is **not coupled** to concrete kernels, models or indexes. It knows them only as `ResourceKey`, `ModelId`, `IndexId`, `Kernel` and `Index` through the interfaces below.
- All CUDA calls sit behind `GpuBackend` (§3). The module contains no `unsafe`, no `cudarc` or `candle_core` import, and no globals.
- All device work goes through lanes with a stream, via `SubengineCore::with_lane` or a `LaneResource`. The default stream is never used.
- The module starts an operation on a free lane and **returns without waiting for the result**. A worker processes the result after the completion notification (`LaneResource` and `notify_result`). The module adds no polling and no blocking waits.
- **FP16 is the default** GPU data type. BF16 is never offered or accepted as a compute or storage type, and there is no Tensor-Core path.
- Nothing is loaded at construction, `start` or `announce`. Loading happens only lazily through the lifecycle (`request_init` → `LoadTicket`).
- The module does not plan, does not own queues and does not run the Subengine-Loop. It does not redefine types of other modules.
- Crates: `log` plus the project modules in §5.

## 2. Context

- **Planner** (build order 126): sees `GpuSubengine` only as `dyn Subengine`. It calls `start`, `capabilities`, `offers`, `announce` and `activate`.
- **Subengine-Loop and Lane-Batching:**
  - The loop calls `LaneResource::execute` and `has_capacity`. A resource lives in a `ResourceRegistry<H>`.
  - The lane resource owns its lanes and worker threads. It aggregates a batch and calls `notify_result` once per batch.
  - This module builds `LaneResource`s but does not run the loop.
- **Subengine core (132):**
  - It owns encoder, scorer and index-segment handles, creates lanes on `activate`, and registers resources.
  - Its lane kinds are Query, Insert, Update, CopyH2D and CopyD2H.
- **Manager (136):** maps `EntryId` to a slot in a segment and streams entries into VRAM. It is created here, one per attached index.
- **Memory modules:**
  - `arenas` defines six arenas, the budget and the fit-check.
  - `speicherverwaltung` (`Speicher`) is the sub-allocator.
  - `weight_source` copies safetensors tensors into the Weights arena.
- **Modellregister / Lifecycle:** `ModelRegistry::encoder_resource` gives the loader for an encoder. `ResourceRegistry<LoadedEncoder>` provides TTL eviction.
  - The lifecycle already enforces `vram_ttl > ram_ttl`.
  - This module requires more (§4.1).
- **Capability-Erkennung:** `detect_capabilities` builds the GPU capability set from a `GpuProbe`.
- **RAM vs. VRAM:** TTL and initialization deadlines are clearly longer for VRAM resources than for RAM resources. This module configures only the VRAM side.
- **Target:** GTX 1660 SUPER (CC 7.5). It has FP32 and FP16 CUDA cores, no Tensor Cores and no BF16 compute. JIT kernels come from the Kernel-Register.

## 3. Interface

```rust
use std::sync::Arc;
use std::time::{Duration, Instant};
use arenas::{Arena, ArenaCapacities, ArenaKind, Budget, BudgetError, MemoryQuery};
use capability_model::CapabilitySet;
use capability_detection::{CapabilityConfig, DetectError, DetectedCapabilities, GpuProbe};
use device_lane_event::{DeviceHandle, Lane, Priority, StreamBackend};
use index_contract::EntryId;
use kernel_register::KernelRegister;
use kernel_scoring::Metric;
use layout::DType;
use lane_batching::{BatchPayload, BatchMeta, BatchRunner, LaneResource, LaneResourceConfig, LaneSetError};
use manager::{LazyIndex, Manager, ManagerConfig, ManagerError, SlotTransfer, UploadReport};
use memory_access::Region;
use model_descriptor::ModelDescriptor;
use model_registry::{LoadedEncoder, ModelRegistry};
use queue_iter::QueueNode;
use resource_lifecycle::{InitRequest, LifecycleConfig, LifecycleError, LoadTicket, ResourceRegistry};
use resource_requirements::{HasResourceRequirements, IndexId, ModelId, ResourceKind};
use speicherverwaltung::MemError;
use subengine_core::{IndexSegment, PathPolicy, QueueRegistrations, SubengineCore, SubengineError};
use target_env::ComputeCapability;
use weight_source::{LoadedWeights, WeightError, WeightFile};

// ---------- Backend (implemented by the CUDA binding; the only place that touches CUDA) ----------

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GpuDeviceInfo {
    pub ordinal: u32,
    pub name: String,
    pub compute_capability: ComputeCapability,
    pub multiprocessors: u32,      // Compute Units (SMs)
    pub total_memory: u64,         // bytes
}

pub trait GpuBackend: Send + Sync {
    fn devices(&self) -> Result<Vec<GpuDeviceInfo>, String>;
    /// Retains the primary context of the device. Called at most once per device by GpuManager.
    fn retain_primary_context(&self, ordinal: u32) -> Result<(), String>;
    fn probe(&self, ordinal: u32) -> Arc<dyn GpuProbe>;
    /// New stream in the primary context of the device.
    fn create_stream(&self, ordinal: u32, priority: Priority) -> Result<Box<dyn StreamBackend>, String>;
    fn memory_query(&self, ordinal: u32) -> Arc<dyn MemoryQuery>;
    /// Reserves the memory of an arena once: VRAM for device arenas, pinned host memory for
    /// Staging and MappedHost. Dropping the Region frees it. Offsets in the Region start at 0.
    fn reserve_arena(&self, ordinal: u32, arena: Arena) -> Result<Region, String>;
    /// Host-to-slot copy of the sub-engine's index segments.
    fn slot_transfer(&self, ordinal: u32) -> Arc<dyn SlotTransfer>;
}

// ---------- Configuration ----------

#[derive(Clone, Debug)]
pub struct GpuConfig {
    pub arenas: ArenaCapacities,            // device_need must fit (§4.2)
    pub headroom: u64,                      // default arenas::DEFAULT_HEADROOM
    pub generations: u32,                   // resident index generations; < 1 counts as 1
    pub lifecycle: LifecycleConfig,         // VRAM: ram_ttl is the RAM reference, vram_ttl is used
    pub ram_init_deadline: Duration,        // RAM reference value
    pub vram_init_deadline: Duration,       // must be >= 2 * ram_init_deadline
    pub default_dtype: DType,               // default F16; only F16 or F32
    pub capability: CapabilityConfig,
    pub worker_lanes: usize,                // >= 1; worker lanes per LaneResource (default 2)
    pub lane_workspace_bytes: usize,        // initial workspace per lane (default 0)
    pub upload_batch: usize,                // >= 1; entries per upload batch (default 256)
}
impl GpuConfig { pub fn validate(&self) -> Result<(), GpuError>; }
impl Default for GpuConfig { /* F16, headroom DEFAULT_HEADROOM, generations 1, worker_lanes 2, upload_batch 256,
                                lifecycle ram_ttl 60s / vram_ttl 600s / backoff 5s, deadlines 30s / 300s */ }

#[derive(Clone, Debug)]
pub struct GpuManagerConfig {
    pub devices: Option<Vec<u32>>,          // None = all backend devices
    pub gpu: GpuConfig,                     // applied to every device
}

/// Shared by all sub-engines of the process.
#[derive(Clone)]
pub struct GpuShared {
    pub kernels: Arc<KernelRegister>,
    pub registry: Arc<ModelRegistry>,
    pub registrations: Arc<QueueRegistrations>,
}

// ---------- Errors ----------

#[derive(Debug)]
pub enum GpuError {
    /// "ungültige GPU-Konfiguration: {0}"
    InvalidConfig(String),
    /// "keine nutzbare GPU"
    NoDevice,
    /// "GPU-Backend: {0}"
    Backend(String),
    Capability(DetectError),
    Subengine(SubengineError),
    Memory(MemError),
    Budget(BudgetError),
    Lifecycle(LifecycleError),
    Manager(ManagerError),
    Weights(WeightError),
    LaneSet(LaneSetError),
    /// "Modell {0:?} nicht registriert" / "Index {0:?} nicht angebunden" / "Index {0:?} bereits angebunden"
    UnknownModel(ModelId),
    UnknownIndex(IndexId),
    AlreadyAttached(IndexId),
    AlreadyRegistered(ModelId),
}
// Display + std::error::Error. Send + Sync + 'static.

// ---------- GPU-Manager ----------

pub struct GpuManager { /* backend, primary contexts, Vec<Arc<GpuSubengine>> */ }   // Send + Sync
impl GpuManager {
    /// Enumerates devices, retains one primary context per selected device, detects capabilities
    /// and builds one GpuSubengine per device. Loads nothing, reserves no VRAM.
    pub fn new(backend: Arc<dyn GpuBackend>, shared: GpuShared, cfg: GpuManagerConfig) -> Result<GpuManager, GpuError>;
    pub fn devices(&self) -> &[GpuDeviceInfo];                    // of the usable devices
    pub fn subengines(&self) -> Vec<Arc<GpuSubengine>>;           // ordered by ordinal
    pub fn subengine(&self, ordinal: u32) -> Option<Arc<GpuSubengine>>;
}

// ---------- VRAM manager ----------

/// Weights of one model in the Weights arena. Drop frees the extent (stream-ordered, see §4.4).
/// Whoever uses the tensors on the device keeps the Arc alive until its events are complete.
pub struct WeightSet { /* LoadedWeights, extent, Arc to VramManager state */ }   // Send + Sync
impl WeightSet {
    pub fn tensors(&self) -> &LoadedWeights;
    pub fn bytes(&self) -> u64;                                   // reserved extent length
}

pub struct VramManager { /* ArenaSet, Speicher for Weights, regions, counters */ }   // Send + Sync
impl VramManager {
    pub fn ordinal(&self) -> u32;
    /// Idempotent. Reserves all six arenas once (§4.2). A failure is not cached.
    pub fn ensure_arenas(&self) -> Result<(), GpuError>;
    pub fn arenas_ready(&self) -> bool;
    /// Region of a reserved arena (clone); None before ensure_arenas.
    pub fn arena_region(&self, kind: ArenaKind) -> Option<Region>;
    /// Copies the selected tensors of a mapped safetensors file into the Weights arena (RAM -> VRAM).
    pub fn stage_weights(&self, file: &WeightFile, select: &dyn Fn(&str) -> bool) -> Result<Arc<WeightSet>, GpuError>;
    pub fn weights_used(&self) -> u64;
    pub fn weights_capacity(&self) -> u64;
}

// ---------- GPU sub-engine ----------

pub struct GpuSubengine { /* core, vram, models registry, managers, backend, cfg */ }   // Send + Sync
impl GpuSubengine {
    pub fn device(&self) -> DeviceHandle;                         // DeviceHandle::Gpu(ordinal)
    pub fn info(&self) -> &GpuDeviceInfo;
    pub fn compute_units(&self) -> u32;                           // == info().multiprocessors
    pub fn core(&self) -> &Arc<SubengineCore>;                    // adopt_scorer, select_path, kernel, with_lane, ...
    pub fn vram(&self) -> &Arc<VramManager>;
    pub fn detected(&self) -> &DetectedCapabilities;
    /// F16 if configured and the capability set has fp16, otherwise F32. Never BF16.
    pub fn compute_dtype(&self) -> DType;

    // Models (swappable in VRAM). Registers only; loads nothing.
    pub fn register_model(&self, model: ModelId, descriptor: Arc<ModelDescriptor>) -> Result<(), GpuError>;
    pub fn request_model(&self, model: &ModelId, task_id: u64, now: Instant) -> Result<InitRequest<LoadedEncoder>, GpuError>;
    /// Runs the ticket of `request_model` (synchronously, on the caller's thread).
    pub fn load_model(&self, model: &ModelId, ticket: LoadTicket<LoadedEncoder>, now: Instant) -> Result<(), GpuError>;
    /// End-of-iteration eviction of expired models (VRAM TTL). Returns the evicted models.
    pub fn evict_expired_models(&self, now: Instant) -> Vec<ModelId>;

    // Indexes. Segments come from the memory backend (RegionHandle + Layout).
    pub fn attach_index(&self, index: IndexId, segments: Vec<IndexSegment>, lazy: Arc<dyn LazyIndex>,
                        cfg: ManagerConfig) -> Result<Arc<Manager>, GpuError>;
    pub fn manager(&self, index: &IndexId) -> Option<Arc<Manager>>;
    /// Streams the entries into VRAM in batches of `upload_batch` (§4.5).
    pub fn stream_entries(&self, index: &IndexId, ids: &[EntryId]) -> Result<UploadReport, GpuError>;

    /// Builds a lane resource with `worker_lanes` new Query lanes (own streams) for the Subengine-Loop.
    pub fn build_lane_resource<N>(&self, cfg: LaneResourceConfig<N>, runner: Arc<dyn BatchRunner<N::Payload>>)
        -> Result<Arc<LaneResource<N>>, GpuError>
    where N: QueueNode, N::Payload: BatchPayload, N::Meta: BatchMeta + HasResourceRequirements;
}
impl planner::Subengine for GpuSubengine { /* name, start, capabilities, offers, announce, activate, is_loaded */ }

/// FP16-first path policy of the GPU. Used by GpuSubengine; public for tests.
pub struct Fp16FirstPolicy { /* default dtype, kernel names */ }
impl Fp16FirstPolicy {
    pub fn new(default_dtype: DType, kernel_names: std::collections::HashMap<ResourceKind, String>) -> Self;
}
impl PathPolicy for Fp16FirstPolicy { /* §4.3 */ }

impl GpuSubengine {
    /// Used by GpuManager. Validates, builds the core; creates no lane and reserves no VRAM.
    pub fn new(info: GpuDeviceInfo, backend: Arc<dyn GpuBackend>, shared: GpuShared, cfg: GpuConfig)
        -> Result<Arc<GpuSubengine>, GpuError>;
}
```

## 4. Behavior and invariants

### 4.1 Configuration (`GpuConfig::validate`, first error wins, `InvalidConfig`)
- `default_dtype` must be F16 or F32. BF16 and non-float types are rejected.
- `worker_lanes >= 1` and `upload_batch >= 1`.
- `lifecycle.vram_ttl >= 2 * lifecycle.ram_ttl` ("clearly longer"). The lifecycle itself only guarantees `>`.
- `vram_init_deadline >= 2 * ram_init_deadline`.

`GpuManager::new` and `GpuSubengine::new` validate.

### 4.2 GpuManager and VRAM arenas
- `GpuManager::new`:
  - Calls `backend.devices()`. An error gives `Backend`.
  - With `devices = Some(list)`, a missing ordinal gives `InvalidConfig` and no device is retained.
  - For each selected device, in ascending ordinal order:
    - `retain_primary_context` is called exactly once. This is the single primary context of the device; all lanes and streams of the device belong to it. The context stays retained for the lifetime of the `GpuManager`.
    - Capabilities are detected with `detect_capabilities(Gpu(n), Some(probe), &cfg.capability)`.
  - With `devices = None`, a device whose context retention or detection fails is skipped and logged at `warn!`.
  - With an explicit list, such a failure returns the error.
  - No usable device gives `Err(NoDevice)`.
  - No lane, kernel or model is created or loaded, and no VRAM is reserved.
- `ensure_arenas` (called by `GpuSubengine::activate`, before the core is activated):
  1. It builds `Budget::from_query(backend.memory_query(n), cfg.arenas.generation_reserve(generations))`.
  2. It runs `fit_check(&cfg.arenas, budget.free, headroom)`. A failure gives `Budget(DoesNotFit)` and **nothing is reserved**. Device arenas are Weights, IndexSegments, Workspace and ParameterBlock.
  3. For each of the six `ArenaKind`s it calls `ArenaSet::ensure` and then `backend.reserve_arena`. It reserves memory only once per arena, and Staging (pinned) and MappedHost are host arenas. A backend error gives `Backend`.
  4. A repeated call is a no-op. If a step fails, the arenas that were reserved are released again, nothing is cached, and a later call retries.
- After a successful `ensure_arenas`, the module performs no further VRAM allocation or free. All later placement happens inside the reserved arenas.
- Concurrent `ensure_arenas` calls reserve each arena exactly once.
- The VRAM budget is limited. Nothing grows. Overflow gives an error (`Memory(OutOfMemory)` or `Budget`), never a new reservation.

### 4.3 Capabilities, lanes, FP16, paths
- `GpuSubengine::new` builds `SubengineConfig`:
  - `name = "gpu:{ordinal}"`.
  - `device = Gpu(ordinal)`.
  - `own_queue = Gpu`.
  - `own_capabilities = detected.capabilities`.
  - `shared_capabilities = own_capabilities`.
  - `offers = [Encoder, Scorer, Index]`.
  - `lanes = LaneConfig::preset(kind, device)` for **each** of Query, Insert, Update, CopyH2D and CopyD2H, with `workspace_size = lane_workspace_bytes`.
- The lane factory creates `Lane::new(cfg, Some(backend.create_stream(ordinal, cfg.priority)?))`. A stream error becomes the factory `Err(String)`. Every lane therefore has a stream, and the default stream is never used.
- `start`, `capabilities`, `offers` and `announce` behave as in the core: no side effects, nothing loaded.
- `compute_dtype()` is `default_dtype` if `default_dtype == F16` and the operands axis of `own_capabilities` contains `fp16`. Otherwise it is F32.
- `Fp16FirstPolicy::select(req, caps, device)`:
  - It returns `None` if `device.kind() != Gpu`.
  - It returns `None` if `kernel_names` has no entry for `req.kind()`.
  - The default `kernel_names` is `{Scorer: "score_dot"}`. The score kernel itself is chosen by `kernel_scoring::Scorer` from the metric. The path only fixes dtype and lane.
  - The dtype is chosen as follows:
    - F16 is the default. It is used if `caps` has `fp16` on `operands` and `req.capabilities()` does not demand fp32 only. "Demands fp32 only" means that the `operands` axis of the requirements contains `fp32` and not `fp16`.
    - Otherwise F32, if `caps` has `fp32`.
    - If neither is available, `None`.
  - The result is `ComputePath { kernel, dtype, variant: Variant::default(), lane: Query }`. **BF16 is never chosen.**
- The policy only decides. The core loads a kernel only through `KernelRegister::request(path.kernel, path.dtype, path.variant)`. This module holds no kernel cache; the Kernel-Register caches, compiles and swaps kernels.

### 4.4 Weights: RAM to VRAM, swap
- `stage_weights(file, select)` computes the required size with the same layout rule as `WeightFile::copy_to_arena`:
  - tensors in file order, each start aligned up to `WEIGHT_ALIGN`;
  - size = end of the last selected tensor.
- It then does the following:
  1. It allocates the extent from the VramManager's `Speicher` as `Lifetime::Dynamic` with alignment `WEIGHT_ALIGN`, on `StreamId(0)`. Its capacity is `arenas[Weights]`.
  2. It calls `file.copy_to_arena(&Arena{Weights, cap}, &weights_region, extent.offset, select)`. This is the zero-copy RAM-to-VRAM bridge: a single write per tensor, straight from the mapping.
  3. It reports `complete` for the fence of that copy.
  4. It returns `Arc<WeightSet>`.
- Required size 0 (nothing selected) returns an empty `WeightSet` without allocation.
- An error frees the extent again and leaves the state unchanged.
  - Error mapping: `Memory(e)` for allocator errors (`OutOfMemory` when the Weights arena is full), `Weights(e)` for copy errors, `Backend` if the arenas are not reserved.
  - `stage_weights` before `ensure_arenas` calls it itself.
- **Swap-out.** When the last `Arc<WeightSet>` is dropped, the extent is freed on `StreamId(0)` with a new fence (`free`), followed by `complete`. The memory is then immediately reusable, also by other streams.
  - Users of the tensors on the device keep the `Arc<WeightSet>` alive until their events are complete.
  - A factory registered in the `ModelRegistry` moves the `Arc<WeightSet>` into the encoder it builds. It obtains the VramManager from `GpuSubengine::vram()`. Model eviction then frees the VRAM.
- `weights_used()` is the sum of live extent sizes. It is `<= weights_capacity()` at all times.
- Extents of live sets never overlap. Concurrent `stage_weights` calls get disjoint extents.

### 4.5 Models, index streaming, batching
- **Model lifecycle.**
  - `register_model(model, descriptor)` registers `ModelRegistry::encoder_resource(descriptor, DeviceClass::Gpu).loader` in a `ResourceRegistry<LoadedEncoder>` with `MemoryClass::Vram`.
  - The registry key is `"gpu:{ordinal}:{model_id}"`. It is **not** `encoder_key`, because that is only architecture-specific and would collide for two models of one architecture.
  - A registered model has the state `Uninitialized`. Nothing is loaded.
  - A second registration of the same `ModelId` gives `AlreadyRegistered`.
  - `request_model` for an unregistered model gives `UnknownModel`. Otherwise it calls `request_init` (never blocking).
  - `load_model` runs `ticket.run(now)`. A failure gives `Lifecycle(Failed)` and the model goes into backoff. An unregistered model gives `UnknownModel`, and the ticket is dropped unrun, so the state returns to `Uninitialized`.
  - If the load takes longer than `vram_init_deadline` (measured with `Instant::now()`), `warn!` is logged with the model. The result is still accepted. A running load is not aborted.
  - `evict_expired_models(now)` calls `evict_expired` and returns the keys mapped back to `ModelId`s. TTL is `lifecycle.vram_ttl`; models in use (lease alive), loading or in backoff are never evicted.
  - A model is swapped in again lazily by the next `request_model`.
- **Encoder ownership.** Encoders live only in this registry. They are not adopted into the core, because the core keeps adopted `Arc`s until it is dropped and would block swapping.
- **Index attach.** `attach_index`:
  - requires every `segment.index == index`, otherwise `InvalidConfig`;
  - rejects an already attached index with `AlreadyAttached`;
  - calls `core.adopt_index_segment` for each segment in order;
  - then calls `Manager::new(core, index, lazy, backend.slot_transfer(ordinal), cfg)`.
  - It stores and returns the `Arc<Manager>`. It loads and uploads nothing, and does not activate the index.
  - A failure gives `Subengine(..)` or `Manager(..)`, and no manager is stored.
- **Streaming.** `stream_entries(index, ids)`:
  - It de-duplicates `ids` in first-occurrence order and cuts them into chunks of `upload_batch`.
  - It calls `Manager::upload` per chunk, in order, and merges the `UploadReport`s by appending the lists.
  - The chunks bound the time the CopyH2D lane is held and let eviction and uploading interleave.
  - A failing entry lands in `failed` and does not abort the other entries. Entries already resident are not copied again.
  - An unknown index gives `UnknownIndex`. Empty `ids` gives an empty report without any index call.
  - Eviction of entries (LRU, swapping) and update application stay in `Manager`; they remain callable through `manager(index)`.
- **Staging.** Host-to-VRAM transfers run on the CopyH2D lane and the Update lane of the core. The pinned Staging and MappedHost regions are available through `arena_region`. This module does not itself copy index data.
- **Lane resource.** `build_lane_resource`:
  - rejects a `cfg.device` different from `self.device()` with `InvalidConfig`;
  - creates `worker_lanes` lanes of kind `Query` with `LaneConfig::preset(Query, device)`. Each has its own stream from the backend and `lane_workspace_bytes` workspace;
  - passes them to `LaneResource::new`. Stream errors give `Backend`, and `LaneSetError` gives `LaneSet`.
  - These lanes are independent of the core's lanes.

### 4.6 `Subengine` implementation and concurrency
- `activate`:
  - It runs `vram.ensure_arenas()` first, then `core.activate()`.
  - It is idempotent and serialized. Errors come back as `Err(Display)` and are not cached.
  - It allocates nothing except the arenas and the lanes' own buffers.
  - It loads no model, kernel or index.
- `is_loaded(kind, model)`:
  - For `Encoder` with `Some(model)`: true iff the model's lifecycle state is `Ready`.
  - For `Encoder` with `None`: true iff any registered model is `Ready`.
  - All other kinds delegate to the core.
  - It is read-only, never activates and never loads.
- All other trait methods delegate to the core unchanged.
- **Thread safety.**
  - All methods take `&self` and are callable from any thread. `GpuManager`, `GpuSubengine`, `VramManager`, `WeightSet` and `GpuError` are `Send + Sync` (static assertion in a test).
  - No lock is held across backend calls, `Manager` or `Index` calls, `ticket.run` or `with_lane`.
  - The VRAM allocator and the model map use short locks.
- **Unavailability.** If the sub-engine is not active, lane work fails with `Subengine(NotActive)`. No panic occurs on valid use.
- **Determinism.** The same configuration and call sequence give the same capabilities, registrations and weight offsets.

## 5. Relied-on elements

- **`subengine_core`:**
  - `SubengineCore::new(cfg: SubengineConfig, lanes: Arc<dyn LaneFactory>, policy: Arc<dyn PathPolicy>, kernels: Arc<KernelRegister>, registry: Arc<ModelRegistry>, registrations: Arc<QueueRegistrations>) -> Result<Self, SubengineError>`.
  - Methods: `adopt_scorer(Metric)`, `adopt_index_segment(IndexSegment)`, `index_segments(&IndexId)`, `select_path`, `kernel(&ComputePath)`, `with_lane(LaneKind, FnOnce(&mut Lane))`. It implements `Subengine`.
  - `SubengineConfig { name, device, own_queue, own_capabilities, shared_capabilities (⊆ own), offers, lanes }`.
  - `LaneFactory::create(&LaneConfig) -> Result<Lane, String>`. A GPU lane without a stream gives `LaneWithoutStream`.
  - `PathPolicy::select(&ResourceRequirements, &CapabilitySet, DeviceHandle) -> Option<ComputePath>`.
  - `ComputePath { kernel: String, dtype: DType, variant: Variant, lane: LaneKind }`.
  - `IndexSegment { index, region: RegionHandle, layout }`.
  - The core never loads anything on `activate`. It creates the lanes and registers compatible resources.
- **`planner::Subengine`:** `name`, `start` (idempotent), `capabilities() -> Vec<(QueueKind, CapabilitySet)>`, `offers`, `announce` (non-blocking), `activate` (blocking, idempotent), `is_loaded(ResourceKind, Option<&ModelId>)`.
- **Device/Lane/Event:**
  - `DeviceHandle { Cpu, Gpu(u32) }`.
  - `LaneKind { Query, Insert, Update, CopyH2D, CopyD2H }`.
  - `LaneConfig::preset(kind, device)` and `Priority`.
  - `Lane::new(cfg, Option<Box<dyn StreamBackend>>)`.
  - `StreamBackend: Send + Sync + Debug`.
- **Capability-Erkennung:**
  - `detect_capabilities(DeviceHandle, Option<&dyn GpuProbe>, &CapabilityConfig) -> Result<DetectedCapabilities, DetectError>`.
  - `GpuProbe { compute_capability() -> Option<ComputeCapability>, device_name() -> Option<String> }`.
  - On CC 7.5 the result contains fp32, fp16, int8_dp4a and on/off.
  - It contains no Tensor-Core or BF16 value.
- **Unterstützte Precisions:** the GPU has scoring kernels for F32 and F16. BF16 is never a compute path.
- **`resource_lifecycle`:**
  - `LifecycleConfig::new(ram_ttl, vram_ttl, backoff)`.
  - `ResourceRegistry<H>::{register(key, MemoryClass, Arc<dyn ResourceLoader<H>>), request_init(key, task_id, now), evict_expired(now), state(key)}`.
  - `InitRequest::{Ready(Lease), Started(LoadTicket), Loading, Backoff{until}, Unavailable}`.
  - `LoadTicket::run(now) -> Result<(), LifecycleError>`. A drop without `run` returns the state to `Uninitialized`.
  - `ResourceState::{Uninitialized, Loading, Ready, Backoff}`.
  - Eviction applies only to resources that are Ready, have no live lease and whose TTL has expired.
- **`model_registry`:**
  - `ModelRegistry::encoder_resource(self: &Arc<Self>, Arc<ModelDescriptor>, DeviceClass) -> EncoderResource { key, init_cost: 100, loader: Arc<dyn ResourceLoader<LoadedEncoder>> }`.
  - `LoadedEncoder: Send + Sync + 'static`.
  - `model_descriptor::DeviceClass { Cpu, Gpu }`.
- **`lane_batching`:**
  - `LaneResource::new(LaneResourceConfig<N>, Vec<Lane>, Arc<dyn BatchRunner<N::Payload>>) -> Result<Arc<Self>, LaneSetError>`.
  - `LaneResourceConfig { own_head, shared_head, filter, limit, notifier, kind, device }`.
  - `LaneResource` implements `ResourceExec` and `LoadReporting`. `execute` is non-blocking.
  - Batch aggregation and `notify_result` (once per batch) are done by this component.
- **`manager`:**
  - `Manager::new(Arc<SubengineCore>, IndexId, Arc<dyn LazyIndex>, Arc<dyn SlotTransfer>, ManagerConfig) -> Result<Manager, ManagerError>`.
  - It requires `ResourceKind::Index` in `core.offers()`.
  - `upload(&[EntryId]) -> UploadReport { uploaded, already_resident, evicted, failed }`, with LRU eviction of unpinned entries.
  - Its calls never block kernels.
  - `SlotTransfer::write_slot(&self, &mut Lane, &IndexSegment, slot, byte_offset, &[u8]) -> Result<(), String>`.
- **`arenas`:**
  - `ArenaKind { Weights, IndexSegments, Workspace, Staging, ParameterBlock, MappedHost }`.
  - `ArenaCapacities::{get, set, device_need, generation_reserve(generations)}`.
  - `Budget::from_query`, `fit_check(caps, free, headroom)`, `DEFAULT_HEADROOM`.
  - `ArenaSet::{new, ensure(kind, &Budget), get}`, `Arena { kind, capacity }`.
  - `MemoryQuery::query() -> Result<MemInfo { free, total }, String>`.
- **`speicherverwaltung`:**
  - `Speicher::new(Config)`, `alloc(Lifetime::Dynamic, size, Some(align), StreamId) -> Result<Region, MemError>`, `free(region, stream, fence)`, `complete(stream, fence)`.
  - Same-stream reuse is immediate.
  - Config constraints: `max_streams >= 1`, `default_align` is a power of two, and Dynamic capacity > 0 when `max_dynamic_blocks > 0`. The VramManager sets only the Dynamic capacity and `max_streams`. The Persistent and Step capacities are 0.
  - This `Region { lifetime, offset, len }` is distinct from `memory_access::Region` (the host/device byte region used by `WeightFile`).
- **`weight_source`:**
  - `WEIGHT_ALIGN = 256`.
  - `WeightFile::{tensors() -> &[WeightInfo], copy_to_arena(&Arena, &Region, base_offset, &dyn Fn(&str) -> bool) -> Result<LoadedWeights, WeightError>}`.
  - Layout: file order, start = `align_up(cursor, 256)`, one `Region::write` per tensor.
  - `WeightInfo { name, dtype, shape, data_begin, data_end }`.
- **`index_contract`:** `EntryId`; `Index` is used only through `Manager`.
- **`kernel_register`:** `KernelRegister::request(name, dtype, Variant)` is the only way to load a kernel. BF16 is rejected.
- **`target_env`:** `ComputeCapability { major, minor }`.
- **`region_handle`:** `RegionHandle` is pointer-free and never resolved here.