# CPU-Subengine: Technical Specification (module `cpu_subengine`, build order 137)

## 1. Task, restrictions and boundaries

**Task.** `CpuSubengine` is the CPU realisation of a sub-engine. It composes `subengine_core::SubengineCore` (Device = `DeviceHandle::Cpu`, own queue `Cpu`) with the following:
- the CPU topology: sockets, cores (resources) and lanes (threads);
- RAM-resident, lazily loaded resources: encoders, tokenizers and normalizer configs;
- its own CPU kernel register with SIMD score kernels;
- the `CpuManager` component (registry of the attached indexes: segments, `FileLazyIndex`, `Manager` and one `IndexResidency` per index);
- low-priority calibration.

**Restrictions.**
- Everything runs on the CPU in RAM. There are no streams, no staging memory and no async copies. Slot data is copied synchronously and directly.
- The default compute dtype is `DType::F32`. BF16 and F16 have no CPU scoring path (`has_scoring_kernel(Cpu, F16) == false`).
- It is not coupled to concrete kernels, models or indexes. Everything is registered or injected: model setups, `EncoderFactory` (via `ModelRegistry`), index specs and `HostSegments`.
- It does no planning, queue selection, queue pulling, scoring-result selection (no top-k) or tokenizing logic. It does not reimplement `SubengineCore`, `Manager`, `LaneResource` or the lifecycle.
- It never calls `SubengineCore::adopt_encoder`. Encoders live only in the lifecycle registry, so eviction really frees them.
- It does not use the async runtime and starts no timer. The caller calls `evict_expired(now)`.
- `unsafe` is allowed only inside the SIMD kernels (`std::arch`). They need runtime feature detection and a scalar fallback. There are no globals. Crates: `log`, plus the project modules in §5.

## 2. Context

- **Planner** drives `CpuSubengine` through the `Subengine` trait: `start`, `capabilities`, `offers`, `announce`, `activate`, `is_loaded`. All except `is_loaded` delegate unchanged to the `SubengineCore`. Activation creates only lanes and loads nothing.
- **Subengine-Loop / Lane-Batching.** `LaneResource` (one worker thread per `Lane`) executes claimed payloads. `CpuSubengine` supplies the lanes (topology) and the `BatchRunner` (`cpu_runner`). Inside the runner every operation runs synchronously on the lane's worker thread. It returns its result, to the task and to index callers, before the runner returns.
- **Memory.** Index segments are RAM regions described as `IndexSegment` (`RegionHandle` + `Layout`). Handle resolution is done only by the injected `HostSegments`.
- **Lifecycle.** All CPU resources are registered with `MemoryClass::Ram`. The RAM TTL is much shorter than the VRAM TTL. The `LifecycleConfig` is validated by `LifecycleConfig::new`.
- **Calibration.** It runs on a `BackgroundPool` (low-priority OS threads, no async runtime) and is never run on lane threads.

## 3. Interface

```rust
use std::borrow::Cow;
use std::ops::Range;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::{Duration, Instant};
use background_exec::{BackgroundPool, JobFuture, PoolError, Dispersion, Estimator};
use calibration_run::{CalibrationBackend, CalibrationConfig, CalibrationError, CalibrationReport,
                      DispersionKind, EstimatorKind, Statistics};
use capability_model::CapabilitySet;
use device_lane_event::{DeviceHandle, Lane};
use encoder_interface::EncoderError;
use file_index::{FileIndex, IndexHeader, OpenOptions};
use index_contract::Index;
use index_residency::{IndexResidency, MemoryAdvisor};
use kernel_register::KernelRegister;
use kernel_scoring::Metric;
use lane_batching::{BatchPayload, BatchRunner, ExecBatch, LaneResource, LaneResourceConfig, LaneSetError};
use layout::DType;
use manager::{LazyIndex, Manager, ManagerConfig, ManagerError, SlotAddr, SlotTransfer};
use memory_access::TensorRef;
use model_descriptor::ModelDescriptor;
use model_registry::{LoadedEncoder, ModelRegistry};
use queue_iter::QueueNode;
use resource_lifecycle::{Lease, LifecycleConfig, ResourceKey};
use resource_requirements::{IndexId, ModelId, ResourceKind};
use subengine_core::{IndexSegment, QueueRegistrations, SubengineCore, SubengineError};
use text_normalization::{NormalizationParams, ResolvedNormalization, TextNormalizer};
use tokenizer_loader::LoadedTokenizer;

/// CPU device structure: sockets -> cores (resources) -> lanes. Lane = one thread of the processor.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CpuTopology { /* private: Vec<Vec<usize>>  (socket -> lanes per core) */ }
impl CpuTopology {
    pub fn new(lanes_per_core_per_socket: Vec<Vec<usize>>) -> Result<Self, CpuError>; // >=1 socket, each >=1 core, each core >=1 lane
    pub fn single_socket(cores: usize, lanes_per_core: usize) -> Result<Self, CpuError>;
    pub fn sockets(&self) -> usize;
    pub fn cores(&self) -> usize;       // over all sockets
    pub fn lane_count(&self) -> usize;  // sum of lanes over all cores
}

pub struct CpuSubengineConfig {
    pub name: String,                         // non-empty, unique within a Planner
    pub topology: CpuTopology,
    pub lifecycle: LifecycleConfig,           // default: ram 30 s, vram 300 s, backoff 5 s
    pub extra_capabilities: CapabilitySet,    // default empty; merged into own capabilities
    pub shared_capabilities: Option<CapabilitySet>, // None = same as own capabilities
    pub offers: Vec<ResourceKind>,            // default [Tokenizer, Normalizer, Encoder, Scorer, Index]
    pub calibration_threads: usize,           // default 1; <1 raised to 1
}
impl CpuSubengineConfig { pub fn new(name: impl Into<String>, topology: CpuTopology) -> Self; } // with defaults

/// Memory backend for RAM segments. Resolves RegionHandle internally (never done in this module).
pub trait HostSegments: Send + Sync {
    fn write(&self, segment: &IndexSegment, byte_offset: u64, bytes: &[u8]) -> Result<(), String>;
    /// Zero-copy read access to `count` f32 values starting at `byte_offset`.
    fn with_f32(&self, segment: &IndexSegment, byte_offset: u64, count: usize,
                f: &mut dyn FnMut(&[f32])) -> Result<(), String>;
}

pub struct CpuDeps {
    pub models: Arc<ModelRegistry>,
    pub registrations: Arc<QueueRegistrations>,
    /// Register exclusively owned by this sub-engine, never shared with the GPU. Handed to SubengineCore only; CPU kernels do not use it.
    pub core_kernels: Arc<KernelRegister>,
    pub host: Arc<dyn HostSegments>,
}

/// Setup of one model on the CPU. Registering loads nothing.
pub struct ModelSetup {
    pub descriptor: Arc<ModelDescriptor>,
    pub normalization: NormalizationParams,
    pub external_normalizer: Option<Arc<dyn TextNormalizer>>,
}

pub struct IndexSpec {
    pub index: IndexId,
    pub path: PathBuf,
    pub header: IndexHeader,
    pub open: OpenOptions,
    pub segments: Vec<IndexSegment>,   // order = segment positions of Manager and SlotAddr.segment
    pub manager: ManagerConfig,
    pub advisor: Option<Arc<dyn MemoryAdvisor>>,   // None: residency hints are no-ops
    pub idle_after: Duration,                      // idle threshold for cold marking
}

#[derive(Debug)]
pub enum CpuError {
    /// "ungültige CPU-Subengine-Konfiguration: {0}"
    InvalidConfig(String),
    Subengine(SubengineError),
    Manager(ManagerError),
    /// "Modell {0} nicht registriert"
    UnknownModel(ModelId),
    /// "Index {0} nicht angebunden"
    UnknownIndex(IndexId),
    /// "Index {0} bereits angebunden"
    AlreadyAttached(IndexId),
    /// "Modell {0} bereits registriert"
    AlreadyRegistered(ModelId),
    /// "Laden fehlgeschlagen: {0}"
    Load(String),
    /// "Ressource im Backoff"
    Backoff { until: Instant },
    /// "Ressource nicht verfügbar: {0}"
    Unavailable(ResourceKey),
    Encoder(EncoderError),
    Normalize(String),
    /// "Index öffnen fehlgeschlagen: {0}"
    IndexOpen(String),
    /// "kein CPU-Kernel {name} für {dtype:?}"
    NoKernel { name: String, dtype: DType },
    /// "Query-Dimension {query} passt nicht zu slot_bytes {slot_bytes}"
    QueryDimMismatch { query: usize, slot_bytes: usize },
    /// "Slot-Bereich ungültig"
    BadSlots(String),
    Host(String),
    Pool(PoolError),
    Lane(LaneSetError),
}
// Display + std::error::Error; Send + Sync + 'static.

// ---------- CPU kernel register (own, separate from the GPU) ----------
pub trait CpuScoreKernel: Send + Sync {
    /// rows: n*dim contiguous f32 values; out.len() == n; f32 accumulation.
    fn score(&self, query: &[f32], rows: &[f32], dim: usize, out: &mut [f32]);
}
pub struct CpuKernelRegister { /* private */ }   // Send + Sync
impl CpuKernelRegister {
    pub fn with_builtin() -> Self;   // "score_dot" and "score_negl2", F32, SIMD with scalar fallback
    pub fn register(&self, name: &str, dtype: DType, kernel: Arc<dyn CpuScoreKernel>) -> Result<(), CpuError>; // duplicate => InvalidConfig
    pub fn request(&self, name: &str, dtype: DType, now: Instant) -> Result<Arc<dyn CpuScoreKernel>, CpuError>;
    pub fn evict_expired(&self, now: Instant, ttl: std::time::Duration) -> Vec<String>; // evicts unused entries (no outstanding Arc) idle >= ttl
}

// ---------- CPU manager (own component) ----------
pub struct CpuManager { /* private */ }          // Send + Sync
impl CpuManager {
    /// Adopts all segments into the core, builds IndexSlot + FileLazyIndex + Manager + IndexResidency. Opens/loads nothing.
    pub fn attach(&self, spec: IndexSpec) -> Result<Arc<Manager>, CpuError>;
    pub fn manager(&self, index: &IndexId) -> Option<Arc<Manager>>;
    /// Opens the FileIndex on the first call (first access), then returns the same Arc. A failure is not cached.
    pub fn index(&self, index: &IndexId) -> Result<Arc<dyn Index>, CpuError>;
    pub fn segments(&self, index: &IndexId) -> Option<Vec<IndexSegment>>;
    pub fn residency(&self, index: &IndexId) -> Option<Arc<IndexResidency>>;
    /// Calls IndexResidency::mark_idle_cold on every attached index. Returns the number of ranges marked cold.
    pub fn mark_idle_cold(&self, now: Instant) -> usize;
}

/// Statistics adapter for calibration (uses background_exec functions).
pub struct RobustStatistics;
impl Statistics for RobustStatistics { /* see §4.9 */ }

// ---------- Scoring ----------
pub struct CpuSegmentSlots { pub segment: u32, pub slots: Range<u32> }   // segment = SlotAddr.segment position
pub struct CpuScoreRequest<'a> { pub index: &'a IndexId, pub metric: Metric, pub query: &'a [f32], pub segments: &'a [CpuSegmentSlots] }
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CpuScored { pub addr: SlotAddr, pub score: f32 }

// ---------- The sub-engine ----------
pub struct CpuSubengine { /* private */ }        // Send + Sync

impl CpuSubengine {
    /// Validates; builds SubengineCore (lanes Query, Insert, Update, CopyH2D, all DeviceHandle::Cpu, no stream),
    /// registries, CpuKernelRegister::with_builtin(), CpuManager and the calibration pool. Loads nothing.
    pub fn new(cfg: CpuSubengineConfig, deps: CpuDeps) -> Result<Arc<Self>, CpuError>;
    pub fn core(&self) -> &Arc<SubengineCore>;
    pub fn manager(&self) -> &CpuManager;
    pub fn kernels(&self) -> &CpuKernelRegister;
    pub fn topology(&self) -> &CpuTopology;
    pub fn default_dtype(&self) -> DType;            // F32

    /// Registers encoder, tokenizer and normalizer resources of the model (Ram). Loads nothing.
    pub fn register_model(&self, model: ModelId, setup: ModelSetup) -> Result<(), CpuError>;

    /// Builds the LaneResource: one Lane (no stream) per topology lane; cfg.device must be DeviceHandle::Cpu.
    pub fn lane_resource<N>(self: &Arc<Self>, cfg: LaneResourceConfig<N>,
                            runner: Arc<dyn BatchRunner<N::Payload>>) -> Result<Arc<LaneResource<N>>, CpuError>
    where N: QueueNode, N::Payload: BatchPayload, N::Meta: lane_batching::BatchMeta + resource_requirements::HasResourceRequirements;

    /// End-of-iteration eviction of all CPU resources and kernels. Returns evicted keys. Also marks idle residency ranges cold.
    pub fn evict_expired(&self, now: Instant) -> Vec<ResourceKey>;

    /// Starts calibration on the low-priority pool. Does not block, applies nothing.
    pub fn calibrate(&self, model: Arc<ModelDescriptor>, backend: Arc<dyn CalibrationBackend>,
                     config: CalibrationConfig)
        -> Result<JobFuture<Result<CalibrationReport, CalibrationError>>, CpuError>;
}
impl planner::Subengine for CpuSubengine { /* name, start, capabilities, offers, announce, activate, is_loaded */ }

// ---------- Synchronous operation execution ----------
pub struct CpuContext<'a> { /* &CpuSubengine, &Lane */ }
impl<'a> CpuContext<'a> {
    pub fn encoder(&self, model: &ModelId) -> Result<Lease<LoadedEncoder>, CpuError>;
    pub fn tokenizer(&self, model: &ModelId) -> Result<Lease<LoadedTokenizer>, CpuError>;
    pub fn normalizer(&self, model: &ModelId) -> Result<Lease<ResolvedNormalization>, CpuError>;
    /// Applies the model's external TextNormalizer (identity if none).
    pub fn preprocess<'t>(&self, model: &ModelId, text: &'t str) -> Result<Cow<'t, str>, CpuError>;
    /// Synchronous: loads encoder if needed, encode_async + wait + receive result.
    pub fn encode(&self, model: &ModelId, input: &TensorRef, output: &TensorRef, rows: usize, seq_len: usize)
        -> Result<(), CpuError>;
    pub fn score(&self, req: &CpuScoreRequest<'_>) -> Result<Vec<CpuScored>, CpuError>;
    pub fn index(&self, index: &IndexId) -> Result<Arc<dyn Index>, CpuError>;
    pub fn manager(&self, index: &IndexId) -> Option<Arc<Manager>>;
}

/// Payload executable on the CPU. Must complete/fail every task of the batch before returning.
pub trait CpuPayload: BatchPayload + Sized {
    fn execute_batch(batch: ExecBatch<Self>, ctx: &CpuContext<'_>);
}
pub fn cpu_runner<P: CpuPayload>(sub: Arc<CpuSubengine>) -> Arc<dyn BatchRunner<P>>;
```

## 4. Behavior and invariants

**4.1 Construction (`new`).** The first error wins, as `InvalidConfig`/`Subengine`/`Pool`.
- A `LifecycleConfig` is accepted as given. The caller creates it with `LifecycleConfig::new`, which enforces vram_ttl > ram_ttl.
- The `SubengineConfig` for the core is:
  - `device = Cpu`, `own_queue = Cpu`.
  - `own_capabilities = precision_capabilities(DeviceKind::Cpu) ∪ extra_capabilities`. The default is therefore exactly `{operands:{fp32}, scoring:{fp32}}`.
  - `shared_capabilities = cfg.shared_capabilities` or, if `None`, the own set.
  - `offers = cfg.offers`.
  - `lanes = LaneConfig::preset(k, DeviceHandle::Cpu)` for `k` in `[Query, Insert, Update, CopyH2D]`. `CopyH2D` exists only because `Manager` uploads use it. It is a plain host lane.
- Core errors (e.g. capabilities outside `cpu_reference()`) are returned as `CpuError::Subengine`.
- `new` creates no lane, loads nothing and opens no file. Only the calibration pool threads (`"{name}-calib"`, `max(1, calibration_threads)`) are started.

**4.2 Topology and lanes.**
- A device has sockets, a socket has cores, a core has lanes. A lane is a thread of the processor.
- `lane_resource` creates exactly `topology.lane_count()` `Lane`s with `Lane::new(LaneConfig::preset(LaneKind::Query, DeviceHandle::Cpu), None)`. They have no stream, so `record_event` gives `NoStreamBackend`. This is not an error here.
- If `cfg.device != DeviceHandle::Cpu`, it returns `InvalidConfig`. `LaneSetError` is wrapped as `Lane`.
- No thread pinning is performed.

**4.3 Subengine trait.**
- `name`, `start`, `capabilities`, `offers`, `announce` and `activate` equal the `SubengineCore` behavior. `start` and `activate` are idempotent. `activate` never loads models, tokenizers or indexes.
- `is_loaded(kind, model)`:
  - `Encoder`: true iff the lifecycle state of the model's encoder resource is `Ready`. `model == None` means any model.
  - `Tokenizer`, `Normalizer`: the same rule on their registries.
  - `Scorer`, `Index`: the core's answer.
  - A kind that is not in `offers` is always false. It is read-only and never activates or loads.

**4.4 Resources and lifecycle.**
- `register_model` registers three resources, all with `MemoryClass::Ram`, in three `ResourceRegistry`s. Nothing is loaded.
  - `"enc:{model}"`: loader from `ModelRegistry::encoder_resource(descriptor, DeviceClass::Cpu).loader`.
  - `"tok:{model}"`: `TokenizerLoader::from_descriptor`.
  - `"norm:{model}"`: `NormalizerLoader::new(normalization, TextPreprocessor::new(external_normalizer))`.
- The encoder therefore uses the library CPU backend (`DeviceClass::Cpu`) with the descriptor's `compute_dtype(Cpu)`. FP32 is the default.
- A second `register_model` for the same id gives `AlreadyRegistered`. A model without setup gives `UnknownModel`.
- Acquire rules (`CpuContext::encoder/tokenizer/normalizer`), implemented with `request_init(key, 0, Instant::now())`:
  - `Ready(lease)`: returned.
  - `Started(ticket)`: run `ticket.run(now)` on the calling thread. Success: re-request and return the lease. `Err(Failed(m))`: `CpuError::Load(m)`.
  - `Loading`: the caller sleeps 1 ms and retries until the state is `Ready` or `Backoff`. It does not spin without a sleep.
  - `Backoff{until}`: `CpuError::Backoff{until}`.
  - `Unavailable`: `CpuError::Unavailable(key)`.
- This is the first-access lazy load. A load happens at most once per resource, guaranteed by the lifecycle.
- Leases taken inside an operation are released with `release(Instant::now())` after the operation, so the TTL counts from the end of use.
- The RAM TTL (`lifecycle.ram_ttl`) applies to everything. `evict_expired(now)` calls `evict_expired` on the three registries and `CpuKernelRegister::evict_expired(now, ram_ttl)`. It returns all evicted keys, with kernels as `"kernel:{name}:{dtype:?}"`. It also calls `CpuManager::mark_idle_cold(now)`. Cold marks are advice only: they evict nothing and add no keys to the result.
- Evicted encoders, tokenizers and normalizers are reloaded on the next access. Kernels are recreated from their registered kernel object on demand. Entries with live leases or `Arc`s are never evicted. Eviction never unregisters anything.

**4.5 Index and `CpuManager`.**
- `attach` order:
  1. Reject a duplicate index with `AlreadyAttached`.
  2. `core.adopt_index_segment` for each segment in order. Errors give `Subengine`.
  3. Create an empty `IndexSlot` and build a `FileLazyIndex` that owns a clone of it.
  4. Build `Manager::new(core, index_id, lazy, CpuSlotTransfer, spec.manager)`. Errors give `Manager`.
5. Build `IndexResidency::new(slot, spec.advisor, spec.idle_after)`. It does not open the index.
- Nothing is opened. `attach` touches no file and no memory.
- `FileLazyIndex::get` calls `FileIndex::open(path, header, open)` on the first successful call and caches the `Arc`. A failure gives `Err(text)` and is not cached.
- After a successful open, `FileLazyIndex::get` stores the `Arc` in its `IndexSlot` with `OnceLock::set`. If another thread filled the slot first, the opened instance is dropped and the stored one is returned. So the `Manager`, `CpuManager::index` and `IndexResidency` always see the same instance, and no lock is held across the open.
- `IndexResidency` never activates the index. Until the first successful `get` the slot is empty and its reads report "not attached".
- `CpuManager::residency` returns the `IndexResidency` of an attached index. `CpuManager::mark_idle_cold(now)` calls `IndexResidency::mark_idle_cold(now)` for each attached index and returns the sum. It holds no registry lock across the advisor calls.
- `CpuManager::index` (and `CpuContext::index`) maps this to `Arc<dyn Index>`. An open error gives `IndexOpen`. An unknown id gives `UnknownIndex`.
- The `CpuSlotTransfer` implementation of `SlotTransfer::write_slot` calls `HostSegments::write(segment, byte_offset, vector)` directly. It does not use the lane's stream, and it copies nothing into a staging buffer.
- `Manager` semantics (pin, lookup, upload, sync, apply_updates) are unchanged and not redefined. `Manager::upload` and `Manager::apply_updates` need an active core (`NotActive` otherwise).
- All index operations are synchronous. Results are returned to the caller on return, including `Stamp` and reports.

**4.6 Scoring (`CpuContext::score`).** Synchronous on the calling lane thread. Kernel name: `Dot` gives `"score_dot"`, `NegL2Sq` gives `"score_negl2"`, both with `DType::F32`.
- Validation order, first error wins:
  1. The index is attached, else `UnknownIndex`.
  2. For each entry, `segment < segments.len()`, `slots.start <= slots.end`, and `slots.end <= Manager config slots_per_segment`. Otherwise `BadSlots`.
  3. `query.len() * 4 == slot_bytes`, else `QueryDimMismatch`. An empty query is rejected the same way.
  4. The kernel is obtained from `CpuKernelRegister`, else `NoKernel`.
- Rows are read with `HostSegments::with_f32(segment, slot * slot_bytes, n * dim, ..)`. This is zero-copy. A host error gives `Host`.
- The result contains one `CpuScored` per requested slot. The order is entry order, then ascending slot. It is not sorted, and there is no selection or top-k.
- A higher score is better. `Dot = Σqᵢxᵢ`, `NegL2Sq = −Σ(qᵢ−xᵢ)²`, accumulated in f32. NaN and Inf pass through unchanged.
- SIMD kernels use runtime detection (x86_64 AVX2+FMA, aarch64 NEON). Otherwise they use the scalar fallback. SIMD and scalar results agree within float tolerance. Bit reproducibility is not promised.
- Slots that are not resident are not checked. The caller pins with `Manager::pin` and maps results through `Manager::entries_at`. A `None` there means skip.
- No state is held across the call except the pinned `Lease` of the caller.
- The CPU path does not use `kernel_scoring::Scorer`. For `Scorer` registrations, the core is used as follows: for each `Metric` with `models.scorer_available(metric)`, `new` calls `core.adopt_scorer(metric)`. Metrics without a registry scorer are skipped silently.

**4.7 Encoding (`CpuContext::encode`).**
- It acquires the encoder lease (§4.4). It creates `oneshot()`. It calls `Encoder::encode_async(lane, input, output, rows, seq_len, vec![tx])`.
- It then calls `CompletionEvent::wait()` and `rx.recv()`.
- It returns `Ok(())` or `Encoder(e)`. An event error is passed through as `Encoder(Device)`.
- It returns only after the result is in the output region (synchronous).
- A pre-check violation is passed through unchanged and nothing is executed.
- No stream, event or staging memory of the lane is used.

**4.8 Runner.**
- `cpu_runner(sub).run(lane, batch)` builds a `CpuContext` and calls `P::execute_batch(batch, &ctx)`. It returns when all work is done. Panics are left to `lane_batching`, which catches them.
- It adds no scheduling, batching or locking. The context is cheap to create and holds no state.

**4.9 Calibration.**
- `calibrate` submits one job to the calibration `BackgroundPool`, whose threads have the lowest scheduling priority. The caller gets a `JobFuture`.
- The job runs `run_calibration(Target{subengine: name, model_id: descriptor.model_id}, &own_capabilities, &descriptor, backend.as_ref(), &RobustStatistics, &config)`.
- The result is returned unchanged. A panic gives `JobError::Panicked` through the future.
- Calibration, together with the kernel executions made by its backend, never runs on a lane thread or on an async runtime thread. Operational scoring does not run on the pool.
- A submit error gives `Pool`. The call never blocks. The report is not applied automatically.
- `RobustStatistics` mapping:
  - `dispersion`: `Cv` maps to `CoefficientOfVariation`, `RangeOverMedian` to `TrimmedRange`. A result of `f64::INFINITY` gives `None`, otherwise `Some(v)`.
  - `estimate`: `Median` maps to `Median`, `Minimum` to `Minimum`, via `background_exec::estimate`.
- Dropping `CpuSubengine` drains and joins the pool (`BackgroundPool` drop semantics).

**4.10 Concurrency and errors.**
- All methods take `&self` and are callable from any thread.
- State sits behind short-lived locks. No lock is held across loader runs, index IO, `HostSegments`, `with_lane`, kernels or encoding.
- Equal configuration gives equal capabilities, registrations and slot assignments.
- A poisoned lock is recovered.
- No panic on valid use. Errors are `CpuError`. Trait methods of `Subengine` return `Err(String)` (the Display text).

## 5. Relied-on elements

- **`subengine_core`:**
  - `SubengineCore::new(SubengineConfig, Arc<dyn LaneFactory>, Arc<dyn PathPolicy>, Arc<KernelRegister>, Arc<ModelRegistry>, Arc<QueueRegistrations>)`.
  - `SubengineConfig { name, device, own_queue, own_capabilities, shared_capabilities, offers, lanes }`.
  - `adopt_scorer(Metric)`, `adopt_index_segment(IndexSegment)`, `index_segments(&IndexId)`, `with_lane`.
  - `SubengineCore` implements `planner::Subengine`. Activation is idempotent, creates only lanes via `LaneFactory::create`, and never loads.
  - **`CpuLaneFactory`** and **`CpuPathPolicy`** are the two sub-engine-specific parts this module supplies to the core.
    - `CpuLaneFactory` returns `Lane::new(cfg.clone(), None)`.
    - `CpuPathPolicy::select` yields `ComputePath{kernel, dtype: F32, variant: Variant{dim: None, tile: None}, lane: Query}` for `Scorer` (kernel `"score_dot"`), `Encoder` and `CrossEncoder` (kernel `"encoder"`). Otherwise it yields `None`.
- **`device_lane_event`:** `Lane::new(LaneConfig, Option<Box<dyn StreamBackend>>)`, `LaneConfig::preset(kind, device)`, `LaneKind::{Query, Insert, Update, CopyH2D}`, `DeviceHandle::Cpu`. `Lane` is `Send + Sync`.
- **`capability_model`:** `CapabilitySet::{new, add, with, is_within}`, `cpu_reference()`. **`precision_capabilities(DeviceKind::Cpu)`** is `{operands:{fp32}, scoring:{fp32}}`.
- **`resource_lifecycle`:**
  - `ResourceRegistry<H>::{new, register(key, MemoryClass::Ram, Arc<dyn ResourceLoader<H>>), request_init(key, task_id, now), evict_expired(now), state}`.
  - `InitRequest::{Ready(Lease), Started(LoadTicket), Loading, Backoff{until}, Unavailable}`.
  - `LoadTicket::run(self, now) -> Result<(), LifecycleError>` and `Lease::{handle, release(now)}`.
  - `LifecycleConfig::new(ram_ttl, vram_ttl, backoff)`.
- **`model_registry`:** `ModelRegistry::{encoder_resource(self: &Arc<Self>, Arc<ModelDescriptor>, DeviceClass) -> EncoderResource{key, init_cost, loader}, scorer_available(Metric)}`, `LoadedEncoder::encoder() -> &Encoder`.
- **`encoder_interface`:** `Encoder::encode_async(&self, &Lane, &TensorRef, &TensorRef, rows, seq_len, Vec<TaskSender>) -> Result<CompletionEvent, EncoderError>`, `oneshot()`, `CompletionEvent::wait()`, `TaskOutcome = Result<(), EncoderError>`.
- **`tokenizer_loader`:** `TokenizerLoader::from_descriptor`, `LoadedTokenizer`.
- **`text_normalization`:** `NormalizerLoader::new(params, TextPreprocessor)`, `TextPreprocessor::{new, apply}`, `ResolvedNormalization`, `TextNormalizer`.
- **`index_residency`:** `IndexResidency::{new(IndexSlot, Option<Arc<dyn MemoryAdvisor>>, Duration), mark_idle_cold(Instant) -> usize}`, `IndexSlot = Arc<OnceLock<Arc<dyn Index>>>`, `MemoryAdvisor`. `IndexSlot` is not `manager::LazyIndex`: `FileLazyIndex` fills the slot, and `Manager` calls the trait.
- **`index_contract` / `file_index`:** `Index` trait, `FileIndex::open(path, IndexHeader, OpenOptions)`, errors as `FileIndexError` (Display used for `IndexOpen`).
- **`manager`:** `Manager::new(core, IndexId, Arc<dyn LazyIndex>, Arc<dyn SlotTransfer>, ManagerConfig)`, `SlotTransfer::write_slot(&self, &mut Lane, &IndexSegment, slot: u32, byte_offset: u64, vector: &[u8])`, `LazyIndex::get`, `SlotAddr{segment, slot}`, `Manager::{pin, entries_at, upload, sync, apply_updates}`.
- **`lane_batching`:** `LaneResource::new(cfg, Vec<Lane>, Arc<dyn BatchRunner<P>>)`, `BatchRunner::run(&self, &mut Lane, ExecBatch<P>)`, `ExecBatch`, `BatchPayload`, `BatchMeta`, `LaneResourceConfig`, `LaneSetError`.
- **`calibration_run`:** `run_calibration(Target, &CapabilitySet, &ModelDescriptor, &dyn CalibrationBackend, &dyn Statistics, &CalibrationConfig)`. `Statistics::{dispersion(&[f64], DispersionKind) -> Option<f64>, estimate(&[f64], EstimatorKind) -> Option<f64>}`.
- **`background_exec`:** `BackgroundPool::{new(name, threads), submit, shutdown}`, `JobFuture`, `median`, `minimum`, `estimate`, `dispersion` (returns `INFINITY` when not computable), `Dispersion::{CoefficientOfVariation, TrimmedRange}`, `Estimator::{Median, Minimum}`.
- **`kernel_scoring`:** `Metric::{Dot, NegL2Sq}` with `kernel_name()`.
- **Others:** `layout::DType`, `memory_access::TensorRef`, `region_handle::RegionHandle` (never resolved here), `resource_requirements::{ModelId, IndexId, ResourceKind}`.