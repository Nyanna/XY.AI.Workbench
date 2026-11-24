# CPU-Subengine: Technical Specification (module `cpu_subengine`, build order 137)

## 1. Task, restrictions and boundaries

**Task.** `CpuSubengine` is the CPU realisation of a sub-engine. It composes `subengine_core::SubengineCore` (Device = `DeviceHandle::Cpu`, own queue `Cpu`) with the following:

- the CPU topology: sockets, cores (resources) and lanes (threads);
- RAM-resident, lazily loaded resources: encoders and tokenizers (the tokenizer carries its configured normalizer);
- its own CPU kernel register with SIMD score kernels;
- the `CpuManager` component (registry of the attached indexes: `FileLazyIndex` and one `IndexResidency` per index);
- low-priority calibration.

**Restrictions.**

- Everything runs on the CPU in RAM. There are no streams, no staging memory and no async copies. The RAM holds the primary copy of the index. Scoring reads entry vectors directly from the `Index` (heap or memory-mapped file) and makes no second copy in slots.
- The default compute dtype is `DType::F32`. BF16 and F16 have no CPU scoring path (`has_scoring_kernel(Cpu, F16) == false`). The index vectors are FP16, but `has_scoring_kernel(Cpu, F16)` refers to the compute dtype and is unaffected: rows are converted to f32 before scoring (§4.6).
- It is not coupled to concrete kernels, models or indexes. Everything is registered or injected: model setups, `EncoderFactory` (via `ModelRegistry`) and index specs.
- It does no planning, queue selection, queue pulling, scoring-result selection (no top-k) or tokenizing logic. It does not reimplement `SubengineCore`, `LaneResource` or the lifecycle. It has no slot manager, no segments and no upload.
- It never calls `SubengineCore::adopt_encoder`. Encoders live only in the lifecycle registry, so eviction really frees them.
- It does not use the async runtime and starts no timer. The caller calls `evict_expired(now)`.
- `unsafe` is allowed only inside the SIMD kernels (`std::arch`). They need runtime feature detection and a scalar fallback. There are no globals. Crates: `log`, `half`, plus the project modules in §5.

## 2. Context

- **Planner** drives `CpuSubengine` through the `Subengine` trait: `start`, `capabilities`, `offers`, `announce`, `activate`, `is_loaded`. All except `is_loaded` delegate unchanged to the `SubengineCore`. Activation creates only lanes and loads nothing.
- **Subengine-Loop / Lane-Batching.** `LaneResource` executes claimed payloads on the worker-pool thread that claimed them. `CpuSubengine` supplies the lanes (topology) and the `BatchRunner` (`cpu_runner`). Inside the runner every operation runs synchronously on that worker thread, and `run` returns `Launch::Finished`. It returns its result, to the task and to index callers, before the runner returns.
- **Memory.** The index lives in RAM, possibly as a memory-mapped file that is larger than the RAM. The index decodes its format transparently. Reads may block on page faults, which is acceptable on a CPU lane thread. The CPU sub-engine reads through the `Index` contract and keeps no copy of the vectors.
- **Lifecycle.** All CPU resources are registered with `MemoryClass::Ram`. The RAM TTL is much shorter than the VRAM TTL. The `LifecycleConfig` is validated by `LifecycleConfig::new`.
- **Calibration.** It runs on a `BackgroundPool` (low-priority OS threads, no async runtime) and is never run on lane threads.

## 3. Interface

```rust
use std::borrow::Cow;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::{Duration, Instant};
use background_exec::{BackgroundPool, JobFuture, PoolError, Dispersion, Estimator};
use calibration_run::{CalibrationBackend, CalibrationConfig, CalibrationError, CalibrationReport,
                      DispersionKind, EstimatorKind, Statistics};
use capability_model::CapabilitySet;
use device_lane_event::{DeviceHandle, Lane};
use encoder_interface::EncoderError;
use index_format::IndexHeader;
use index_open::OpenOptions;
use file_lazy_index::FileLazyIndex;
use cpu_kernels::CpuKernelRegister;
use index_contract::{EntryId, Index};
use index_residency::{IndexResidency, MemoryAdvisor};
use kernel_register::{CpuScoreKernel, KernelError, KernelProvider};
use kernel_scoring::Metric;
use lane_batching::{BatchPayload, BatchRunner, ExecBatch, LaneResource, LaneResourceConfig, LaneSetError};
use layout::DType;
use manager::LazyIndex;
use memory_access::TensorRef;
use model_descriptor::ModelDescriptor;
use model_registry::{LoadedEncoder, ModelRegistry};
use queue::Queue;
use resource_lifecycle::{Lease, LifecycleConfig, ResourceKey};
use resource_requirements::{IndexId, ModelId, ResourceKind};
use subengine_core::{QueueRegistrations, SubengineCore, SubengineError};
use text_normalization::{NormalizationParams, NormalizerLoader, TextNormalizer, TextPreprocessor};
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

pub struct CpuDeps {
    pub models: Arc<ModelRegistry>,
    pub registrations: Arc<QueueRegistrations>,
}

/// Setup of one model on the CPU. Registering loads nothing.
pub struct ModelSetup {
    pub descriptor: Arc<ModelDescriptor>,
    pub normalization: NormalizationParams,
    pub external_normalizer: Option<Arc<dyn TextNormalizer>>,
}

pub struct CpuIndexSpec {
    pub index: IndexId,
    pub path: PathBuf,
    pub header: IndexHeader,   // expected header (`IndexHeader::from_descriptor(..)`)
    pub open: OpenOptions,     // `index_open::OpenOptions` (`use_mmap`, `verify`); no `row_group_rows`: the CPU sub-engine does not save
    pub advisor: Option<Arc<dyn MemoryAdvisor>>,   // None: residency hints are no-ops
    pub idle_after: Duration,                      // idle threshold for cold marking
}

#[derive(Debug)]
pub enum CpuError {
    /// "ungültige CPU-Subengine-Konfiguration: {0}"
    InvalidConfig(String),
    Subengine(SubengineError),
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
    /// "Index öffnen fehlgeschlagen: {0}" ({0}: Display of `OpenError`, which contains the decoder's header or binding text)
    IndexOpen(String),
    /// Display of the `KernelError` (unknown kernel, duplicate registration)
    Kernel(KernelError),
    /// "Query-Dimension {query} passt nicht zu Vektorgröße {row_bytes}"
    QueryDimMismatch { query: usize, row_bytes: usize },
    Pool(PoolError),
    Lane(LaneSetError),
}
// Display + std::error::Error; Send + Sync + 'static.

// ---------- CPU kernel register (module `cpu_kernels`, below the sub-engines) ----------
// `kernel_register::CpuScoreKernel` is the kernel trait; `kernel_register::Kernel::Cpu(Arc<dyn CpuScoreKernel>)` carries it.
// CpuKernelRegister implements `kernel_register::KernelProvider`, like the GPU `KernelRegister`; the core obtains every kernel through that trait.
pub struct CpuKernelRegister { /* private */ }   // Send + Sync
impl CpuKernelRegister {
    pub fn new() -> Self;
    pub fn with_builtin() -> Self;   // "score_dot" and "score_negl2", F32, SIMD with scalar fallback (cpu_compute::{Isa, dot_with, sq_dist_with})
    pub fn register(&self, name: &str, dtype: DType, kernel: Arc<dyn CpuScoreKernel>) -> Result<(), KernelError>; // duplicate => Unavailable
    pub fn request_at(&self, name: &str, dtype: DType, now: Instant) -> Result<Arc<dyn CpuScoreKernel>, KernelError>; // missing => UnknownKernel
    pub fn evict_expired(&self, now: Instant, ttl: std::time::Duration) -> Vec<String>; // evicts unused entries (no outstanding Arc) idle >= ttl; keys from kernel_resource_key
}
impl KernelProvider for CpuKernelRegister { /* request(name, dtype, variant) -> Kernel::Cpu; variant is ignored */ }

// ---------- CPU manager (own component: registry of the attached indexes) ----------
pub struct CpuManager { /* private */ }          // Send + Sync
impl CpuManager {
    /// Builds IndexSlot + FileLazyIndex + IndexResidency. Opens/loads nothing.
    pub fn attach(&self, spec: CpuIndexSpec) -> Result<(), CpuError>;
    /// Opens the index (`LazyIndexFile` and `EntryStore`) on the first call (first access), then returns the same Arc. A failure is not cached.
    pub fn index(&self, index: &IndexId) -> Result<Arc<dyn Index>, CpuError>;
    pub fn residency(&self, index: &IndexId) -> Option<Arc<IndexResidency>>;
    /// Calls IndexResidency::mark_idle_cold on every attached index. Returns the number of ranges marked cold.
    pub fn mark_idle_cold(&self, now: Instant) -> usize;
}

/// Statistics adapter for calibration (uses background_exec functions).
pub struct RobustStatistics;
impl Statistics for RobustStatistics { /* see §4.9 */ }

// ---------- Scoring ----------
pub struct CpuScoreRequest<'a> { pub index: &'a IndexId, pub metric: Metric, pub query: &'a [f32], pub entries: &'a [EntryId] }
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CpuScored { pub entry: EntryId, pub score: f32 }
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CpuScoreResult {
    pub scored: Vec<CpuScored>,      // request order, one per scorable entry
    pub absent: Vec<EntryId>,        // not in the index
    pub malformed: Vec<EntryId>,     // vector length differs from the index vector size
}

// ---------- The sub-engine ----------
pub struct CpuSubengine { /* private */ }        // Send + Sync

impl CpuSubengine {
    /// Validates; builds SubengineCore (lanes Query, Insert, Update, CopyH2D, all DeviceHandle::Cpu, no stream),
    /// registries, CpuKernelRegister::with_builtin(), CpuManager and the calibration pool. Loads nothing.
    pub fn new(cfg: CpuSubengineConfig, deps: CpuDeps) -> Result<Arc<Self>, CpuError>;
    pub fn core(&self) -> &Arc<SubengineCore>;
    pub fn manager(&self) -> &CpuManager;   // registry of the attached indexes, not a slot manager
    pub fn kernels(&self) -> &CpuKernelRegister;
    pub fn topology(&self) -> &CpuTopology;
    pub fn default_dtype(&self) -> DType;            // F32

    /// Registers encoder and tokenizer resources of the model (Ram). The tokenizer resource
    /// carries the model's normalizer config. Loads nothing.
    pub fn register_model(&self, model: ModelId, setup: ModelSetup) -> Result<(), CpuError>;

    /// Builds the LaneResource: one Lane (no stream) per topology lane; cfg.device must be DeviceHandle::Cpu.
    pub fn lane_resource<M, P>(self: &Arc<Self>, cfg: LaneResourceConfig<M, P>,
                                runner: Arc<dyn BatchRunner<P>>) -> Result<Arc<LaneResource<M, P>>, CpuError>
    where P: BatchPayload, M: lane_batching::BatchMeta + resource_requirements::HasResourceRequirements + Send + Sync + 'static;

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
    /// The model's text preprocessor (cheap clone; identity if the model has no external normalizer).
    pub fn preprocessor(&self, model: &ModelId) -> Result<TextPreprocessor, CpuError>;
    /// Applies the model's external TextNormalizer (identity if none).
    pub fn preprocess<'t>(&self, model: &ModelId, text: &'t str) -> Result<Cow<'t, str>, CpuError>;
    /// Synchronous: loads encoder if needed, encode_async + wait + receive result.
    pub fn encode(&self, model: &ModelId, input: &TensorRef, output: &TensorRef, rows: usize, seq_len: usize)
        -> Result<(), CpuError>;
    pub fn score(&self, req: &CpuScoreRequest<'_>) -> Result<CpuScoreResult, CpuError>;
    pub fn index(&self, index: &IndexId) -> Result<Arc<dyn Index>, CpuError>;
}

/// Payload executable on the CPU. Must complete/fail every task of the batch before returning.
pub trait CpuPayload: BatchPayload + Sized {
    fn execute_batch(batch: ExecBatch<Self>, ctx: &CpuContext<'_>);
}
pub fn cpu_runner<P: CpuPayload>(sub: Arc<CpuSubengine>) -> Arc<dyn BatchRunner<P>>;
```

## 4. Behavior and invariants

**4.1 Construction (`new`).** The first error wins, as `InvalidConfig`/`Subengine`/`Pool`.

- A `LifecycleConfig` is accepted as given. The caller creates it with `LifecycleConfig::new`, which enforces vram\_ttl > ram\_ttl.
- The `SubengineConfig` for the core is:
  - `device = Cpu`, `own_queue = Cpu`.
  - `own_capabilities = precision_capabilities(DeviceKind::Cpu) ∪ extra_capabilities`. The default is therefore exactly `{operands:{fp32}, scoring:{fp32}}`.
  - `shared_capabilities = cfg.shared_capabilities` or, if `None`, the own set.
  - `offers = cfg.offers`.
  - `lanes = LaneConfig::preset(k, DeviceHandle::Cpu)` for `k` in `[Query, Insert, Update, CopyH2D]`. All four are plain host lanes.
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
  - `Tokenizer`: the same rule on the tokenizer registry.
  - `Normalizer`: the same answer as `Tokenizer` for the same model, because the normalizer config is part of the tokenizer.
  - `Scorer`, `Index`: the core's answer.
  - A kind that is not in `offers` is always false. It is read-only and never activates or loads.

**4.4 Resources and lifecycle.**

- `register_model` builds the model's `TextPreprocessor` once, from `ModelSetup.external_normalizer`, and keeps it in the model entry. It registers two resources, both with `MemoryClass::Ram`, in two `ResourceRegistry`s. Nothing is loaded.
  - `"enc:{model}"`: loader from `ModelRegistry::encoder_resource(descriptor, DeviceClass::Cpu).loader`.
  - `"tok:{model}"`: `TokenizerLoader::from_descriptor(&descriptor).with_normalization(Arc::new(NormalizerLoader::new(normalization, preprocessor.clone())))`. The config is baked into the tokenizer when it loads.
  - There is no separate normalizer resource. The normalizer config lives in the tokenizer, and the `Normalizer` kind is answered from the tokenizer state (4.3).
- The encoder therefore uses the library CPU backend (`DeviceClass::Cpu`) with the descriptor's `compute_dtype(Cpu)`. FP32 is the default.
- A second `register_model` for the same id gives `AlreadyRegistered`. A model without setup gives `UnknownModel`.
- Acquire rules (`CpuContext::encoder/tokenizer`), implemented with `request_init(key, 0, Instant::now())`:
  - `Ready(lease)`: returned.
  - `Started(ticket)`: run `ticket.run(now)` on the calling thread. Success: re-request and return the lease. `Err(Failed(m))`: `CpuError::Load(m)`.
  - `Loading`: the caller sleeps 1 ms and retries until the state is `Ready` or `Backoff`. It does not spin without a sleep.
  - `Backoff{until}`: `CpuError::Backoff{until}`.
  - `Unavailable`: `CpuError::Unavailable(key)`.
- This is the first-access lazy load. A load happens at most once per resource, guaranteed by the lifecycle.
- Leases taken inside an operation are released with `release(Instant::now())` after the operation, so the TTL counts from the end of use.
- The RAM TTL (`lifecycle.ram_ttl`) applies to everything. `evict_expired(now)` calls `evict_expired` on the two registries and `CpuKernelRegister::evict_expired(now, ram_ttl)`. It returns all evicted keys, with kernels as `"kernel:{name}:{dtype:?}"`. It also calls `CpuManager::mark_idle_cold(now)`. Cold marks are advice only: they evict nothing and add no keys to the result.
- Evicted encoders and tokenizers are reloaded on the next access. Kernels are recreated from their registered kernel object on demand. Entries with live leases or `Arc`s are never evicted. Eviction never unregisters anything.
- The `TextPreprocessor` is not a lifecycle resource. It lives in the model entry and is never evicted. `CpuContext::preprocessor(model)` returns a clone of it, or `UnknownModel`. It takes no lease and loads nothing. `preprocess(model, text)` is `preprocessor(model)?.apply(text)`, with `NormalizeError(m)` mapped to `CpuError::Normalize(m)`.

**4.5 Index and `CpuManager`.**

- `attach` order:
  1. Reject a duplicate index with `AlreadyAttached`.
  2. Create an empty `IndexSlot` and build a `FileLazyIndex` that owns a clone of it.
  3. Build `IndexResidency::new(slot, spec.advisor, spec.idle_after)`. It does not open the index.
- Nothing is opened. `attach` touches no file and no memory.
- `FileLazyIndex::get` builds the index as follows:
  1. If `spec.path` does not exist: `EntryStore::new(header, None, checksums)`.
  2. Otherwise build a **new** `LazyIndexFile::new(path, decoder, open)` on every attempt, call `ensure_open()`, and on `Err(e)` return `Err(e.to_string())` and drop it. A failure is therefore not cached, although `LazyIndexFile` remembers its own failure.
  3. On success build `EntryStore::new(header, Some(Arc::new(file)), checksums)`.
  4. The decoder is `ParquetDecoder::new(header.clone(), Arc::new(entry_payload::compute_checksum_parts))`, `checksums` is `PayloadChecksums::new(decoder)`.
- After a successful open, `FileLazyIndex::get` stores the `Arc` in its `IndexSlot` with `OnceLock::set`. If another thread filled the slot first, the opened instance is dropped and the stored one is returned. So `CpuManager::index` and `IndexResidency` always see the same instance, and no lock is held across the open.
- `IndexResidency` never activates the index. Until the first successful `get` the slot is empty and its reads report "not attached".
- `CpuManager::residency` returns the `IndexResidency` of an attached index. `CpuManager::mark_idle_cold(now)` calls `IndexResidency::mark_idle_cold(now)` for each attached index and returns the sum. It holds no registry lock across the advisor calls.
- `CpuManager::index` (and `CpuContext::index`) maps this to `Arc<dyn Index>`. An open error gives `IndexOpen` with the `Display` of `OpenError` (which contains the decoder's header or binding text). An unknown id gives `UnknownIndex`.
- The CPU path has no slot manager, no segments and no copy of the index data. The RAM is the primary instance and the index decodes its format transparently, from the heap or from a memory map. Reads go to `Index::read` and may block on page faults. Page-cache hints come only from `IndexResidency`.
- Index vectors are `dimension × 2` bytes (FP16 LE) per entry, which is `header.vector_bytes()`.
- All index operations are synchronous. Results are returned to the caller on return, including `Stamp` and reports.

**4.6 Scoring (`CpuContext::score`).** Synchronous on the calling lane thread. Kernel name: `Dot` gives `"score_dot"`, `NegL2Sq` gives `"score_negl2"`, both with `DType::F32`.

- Validation order, first error wins:
  1. The index is attached, else `UnknownIndex`. It is opened on first use, and a failure gives `IndexOpen`.
  2. `query.len() * 2 == header.vector_bytes()`, else `QueryDimMismatch`. An empty query is rejected the same way.
  3. The kernel is obtained from `CpuKernelRegister::request_at`, else `CpuError::Kernel(UnknownKernel)`.
- The entries are processed in request order, with duplicates removed (first occurrence wins). Each entry is read with `index.read(id)`:
  - An empty vector means the entry is absent. It goes to `absent`.
  - A vector length other than `header.vector_bytes()` goes to `malformed` and is logged with `warn!`.
  - Otherwise the row is scored.
- The FP16 rows are converted block-wise (at most 1024 rows per block) into a scratch `Vec<f32>` with `half::f16::to_f32`. Then `CpuScoreKernel::score(query, rows_f32, dim, out)` runs. Accumulation stays f32.
- `CpuScoreResult.scored` has one `CpuScored` per scorable entry, in request order. It is not sorted, and there is no selection or top-k.
- A higher score is better. `Dot = Σqᵢxᵢ`, `NegL2Sq = −Σ(qᵢ−xᵢ)²`, accumulated in f32. NaN and Inf pass through unchanged.
- SIMD kernels use runtime detection (x86\_64 AVX2+FMA, aarch64 NEON). Otherwise they use the scalar fallback. SIMD and scalar results agree within float tolerance. Bit reproducibility is not promised.
- Scoring needs no pin. The data is read at call time and is current as of that read. A concurrent write can make a result stale. That is accepted, because the index wins.
- No state is held across the call.
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
- For the reference payload (`EngineStep`, build order 135) `execute_batch` runs each task's operation and then calls `step.finish(output, &*sink)`; on an execution error, or when `finish` returns an error, it calls `step.fail(error, || 0)`. One task's error never affects the other tasks of the batch. `collect` is never called for CPU lanes.

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
- State sits behind short-lived locks. No lock is held across loader runs, index IO, `with_lane`, kernels or encoding.
- Equal configuration gives equal capabilities and registrations.
- A poisoned lock is recovered.
- No panic on valid use. Errors are `CpuError`. Trait methods of `Subengine` return `Err(String)` (the Display text).

## 5. Relied-on elements

- **`subengine_core`:**
  - `SubengineCore::new(SubengineConfig, Arc<dyn LaneFactory>, Arc<dyn PathPolicy>, Arc<dyn KernelProvider>, Arc<ModelRegistry>, Arc<QueueRegistrations>)`.
  - `SubengineConfig { name, device, own_queue, own_capabilities, shared_capabilities, offers, lanes }`.
  - `adopt_scorer(Metric)`, `with_lane`.
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
- **`tokenizer_loader`:** `TokenizerLoader::{from_descriptor, with_normalization}`, `LoadedTokenizer`.
- **`text_normalization`:** `NormalizerLoader::new(params, TextPreprocessor)`, `TextPreprocessor::{new, apply}` (`Clone`), `ResolvedNormalization`, `TextNormalizer`.
- **`index_residency`:** `IndexResidency::{new(IndexSlot, Option<Arc<dyn MemoryAdvisor>>, Duration), mark_idle_cold(Instant) -> usize}`, `IndexSlot = Arc<OnceLock<Arc<dyn Index>>>`, `MemoryAdvisor`. `FileLazyIndex` fills the slot and implements `manager::LazyIndex`.
- **`index_contract` / `entry_store` / `index_open` / `parquet_decoder`:** `Index` trait, `EntryStore`, `LazyIndexFile::{new, ensure_open}` and `ParquetDecoder`/`PayloadChecksums`, which `FileLazyIndex` builds; errors as `OpenError` (Display used for `IndexOpen`).
- **`manager`:** `trait LazyIndex: Send + Sync { fn get(&self) -> Result<Arc<dyn Index>, String>; }`.
- **`file_lazy_index`:** `FileLazyIndex::{new(path, IndexHeader, OpenOptions, IndexSlot), with_advice_slot, slot}` implements `manager::LazyIndex`. It lives below the sub-engines so that the GPU side can use it as well.
- **`lane_batching`:** `LaneResource::new(cfg, Vec<Lane>, Arc<dyn BatchRunner<P>>)`, `BatchRunner::{run(&self, &mut Lane, &mut ExecBatch<P>) -> Launch, collect(&self, &mut Lane, ExecBatch<P>, Result<(), EventError>)}`, `ExecBatch`, `BatchPayload`, `BatchMeta`, `LaneResourceConfig`, `LaneSetError`.
- **`calibration_run`:** `run_calibration(Target, &CapabilitySet, &ModelDescriptor, &dyn CalibrationBackend, &dyn Statistics, &CalibrationConfig)`. `Statistics::{dispersion(&[f64], DispersionKind) -> Option<f64>, estimate(&[f64], EstimatorKind) -> Option<f64>}`.
- **`background_exec`:** `BackgroundPool::{new(name, threads), submit, shutdown}`, `JobFuture`, `median`, `minimum`, `estimate`, `dispersion` (returns `INFINITY` when not computable), `Dispersion::{CoefficientOfVariation, TrimmedRange}`, `Estimator::{Median, Minimum}`.
- **`kernel_scoring`:** `Metric::{Dot, NegL2Sq}` with `kernel_name()`.
- **`kernel_register`:** `trait KernelProvider { fn request(&self, name, DType, Variant) -> Result<Kernel, KernelError> }`, `Kernel::Cpu(Arc<dyn CpuScoreKernel>)`, `CpuScoreKernel::score`. `CpuSubengine::new` passes its own `CpuKernelRegister` to the core as the provider; the core's `kernel(&ComputePath)` therefore yields `Kernel::Cpu`. `kernel_resource_key(name, DType) -> ResourceKey` builds the `kernel:{name}:{dtype:?}` key returned by `CpuKernelRegister::evict_expired`.
- **`cpu_compute`:** `Isa::detect()`, `dot_with(Isa, &[f32], &[f32])`, `sq_dist_with(Isa, &[f32], &[f32])`. They are the only CPU score primitives; the built-in `CpuScoreKernel`s call them.
- **Others:** `layout::DType`, `memory_access::TensorRef`,  `resource_requirements::{ModelId, IndexId, ResourceKind}`.
