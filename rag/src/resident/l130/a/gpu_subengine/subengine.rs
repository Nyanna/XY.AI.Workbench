use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Instant;
use log::warn;
use super::backend::{GpuBackend, GpuDeviceInfo};
use super::config::{default_kernel_names, GpuConfig, GpuShared};
use super::deps::*;
use super::error::GpuError;
use super::index_services::{IndexServices, IndexShared};
use super::policy::{Fp16FirstPolicy, GpuLaneFactory};
use super::vram::VramManager;
/// Memory manager and coordinator of one attached index.
pub struct IndexMemory {
    pub gpu: Arc<GpuManager>,
    pub ops: Arc<IndexOperations>,
}
static_assertions::assert_impl_all!(IndexMemory : Send, Sync);
/// The sub-engine of one GPU.
pub struct GpuSubengine {
    info: GpuDeviceInfo,
    backend: Arc<dyn GpuBackend>,
    cfg: GpuConfig,
    detected: DetectedCapabilities,
    shared: GpuShared,
    core: Arc<SubengineCore>,
    vram: Arc<VramManager>,
    lanes: GpuLaneFactory,
    offers: Vec<ResourceKind>,
    encoders: ResourceRegistry<LoadedEncoder>,
    models: Mutex<HashMap<ModelId, ResourceKey>>,
    segments: Mutex<Option<Arc<IndexShared>>>,
    indexes: Mutex<HashMap<IndexId, Arc<IndexMemory>>>,
}
static_assertions::assert_impl_all!(GpuSubengine : Send, Sync);
fn lane_config(
    kind: LaneKind,
    device: DeviceHandle,
    workspace_bytes: usize,
) -> LaneConfig {
    let mut cfg = LaneConfig::preset(kind, device);
    cfg.workspace_size = workspace_bytes;
    cfg
}
impl GpuSubengine {
    /// Validates, detects the capabilities and builds the core. Creates no lane, reserves no VRAM
    /// and loads nothing.
    pub fn new(
        info: GpuDeviceInfo,
        backend: Arc<dyn GpuBackend>,
        shared: GpuShared,
        cfg: GpuConfig,
    ) -> Result<Arc<GpuSubengine>, GpuError> {
        cfg.validate()?;
        let probe = backend.probe(info.ordinal);
        let detected = detect_capabilities(
                DeviceHandle::Gpu(info.ordinal),
                Some(&*probe),
                &cfg.capability,
            )
            .map_err(GpuError::Capability)?;
        GpuSubengine::build(info, backend, shared, cfg, detected)
    }
    pub(super) fn build(
        info: GpuDeviceInfo,
        backend: Arc<dyn GpuBackend>,
        shared: GpuShared,
        cfg: GpuConfig,
        detected: DetectedCapabilities,
    ) -> Result<Arc<GpuSubengine>, GpuError> {
        let ordinal = info.ordinal;
        let device = DeviceHandle::Gpu(ordinal);
        let lanes = [
            LaneKind::Query,
            LaneKind::Insert,
            LaneKind::Update,
            LaneKind::CopyH2D,
            LaneKind::CopyD2H,
        ]
            .into_iter()
            .map(|kind| lane_config(kind, device, cfg.lane_workspace_bytes))
            .collect();
        let core = SubengineCore::new(
                SubengineConfig {
                    name: format!("gpu:{ordinal}"),
                    device,
                    own_queue: QueueKind::Gpu,
                    own_capabilities: detected.capabilities.clone(),
                    shared_capabilities: detected.capabilities.clone(),
                    offers: vec![
                        ResourceKind::Encoder, ResourceKind::Scorer, ResourceKind::Index
                    ],
                    lanes,
                },
                Arc::new(GpuLaneFactory::new(Arc::clone(&backend), ordinal)),
                Arc::new(
                    Fp16FirstPolicy::new(cfg.default_dtype, default_kernel_names()),
                ),
                Arc::clone(&shared.kernels) as Arc<dyn KernelProvider>,
                Arc::clone(&shared.registrations),
            )
            .map_err(GpuError::Subengine)?;
        core.adopt_scorer();
        let offers = Subengine::offers(&core);
        let vram = VramManager::new(ordinal, Arc::clone(&backend), &cfg);
        Ok(
            Arc::new(GpuSubengine {
                lanes: GpuLaneFactory::new(Arc::clone(&backend), ordinal),
                encoders: ResourceRegistry::new(cfg.lifecycle.clone()),
                info,
                backend,
                cfg,
                detected,
                shared,
                core: Arc::new(core),
                vram,
                offers,
                models: Mutex::new(HashMap::new()),
                segments: Mutex::new(None),
                indexes: Mutex::new(HashMap::new()),
            }),
        )
    }
    pub fn device(&self) -> DeviceHandle {
        DeviceHandle::Gpu(self.info.ordinal)
    }
    pub fn info(&self) -> &GpuDeviceInfo {
        &self.info
    }
    pub fn compute_units(&self) -> u32 {
        self.info.multiprocessors
    }
    pub fn core(&self) -> &Arc<SubengineCore> {
        &self.core
    }
    pub fn vram(&self) -> &Arc<VramManager> {
        &self.vram
    }
    pub fn detected(&self) -> &DetectedCapabilities {
        &self.detected
    }
    /// F16 if configured and the capability set has fp16, otherwise F32. Never BF16.
    pub fn compute_dtype(&self) -> DType {
        let half = self
            .detected
            .capabilities
            .contains(&axes::operands(), &values::fp16());
        if self.cfg.default_dtype == DType::F16 && half {
            DType::F16
        } else {
            DType::F32
        }
    }
    /// Registers the encoder of the model for VRAM residency. Loads nothing.
    pub fn register_model(
        &self,
        model: ModelId,
        descriptor: Arc<ModelDescriptor>,
    ) -> Result<(), GpuError> {
        let mut models = lock(&self.models);
        if models.contains_key(&model) {
            return Err(GpuError::AlreadyRegistered(model));
        }
        let key: ResourceKey = format!("gpu:{}:{}", self.info.ordinal, model.as_str());
        let resource = self
            .shared
            .registry
            .encoder_resource(descriptor, DeviceClass::Gpu);
        self.encoders
            .register(key.clone(), MemoryClass::Vram, resource.loader)
            .map_err(GpuError::Lifecycle)?;
        models.insert(model, key);
        Ok(())
    }
    fn model_key(&self, model: &ModelId) -> Result<ResourceKey, GpuError> {
        lock(&self.models)
            .get(model)
            .cloned()
            .ok_or_else(|| GpuError::UnknownModel(model.clone()))
    }
    /// Never blocks. A started load has to be run with `load_model`.
    pub fn request_model(
        &self,
        model: &ModelId,
        task_id: u64,
        now: Instant,
    ) -> Result<InitRequest<LoadedEncoder>, GpuError> {
        let key = self.model_key(model)?;
        Ok(self.encoders.request_init(&key, task_id, now))
    }
    /// Runs the ticket of `request_model` on the caller's thread.
    pub fn load_model(
        &self,
        model: &ModelId,
        ticket: LoadTicket<LoadedEncoder>,
        now: Instant,
    ) -> Result<(), GpuError> {
        if let Err(e) = self.model_key(model) {
            drop(ticket);
            return Err(e);
        }
        let started = Instant::now();
        let outcome = ticket.run(now).map_err(GpuError::Lifecycle);
        let took = started.elapsed();
        if took > self.cfg.vram_init_deadline {
            warn!(
                "gpu:{}: loading model {model} took {took:?}, deadline {:?}", self.info
                .ordinal, self.cfg.vram_init_deadline
            );
        }
        outcome
    }
    /// End-of-iteration eviction of expired models (VRAM time to live).
    pub fn evict_expired_models(&self, now: Instant) -> Vec<ModelId> {
        let keys = self.encoders.evict_expired(now);
        let models = lock(&self.models);
        keys.iter()
            .filter_map(|key| {
                models.iter().find(|(_, k)| *k == key).map(|(model, _)| model.clone())
            })
            .collect()
    }
    fn index_shared(&self) -> Result<Arc<IndexShared>, GpuError> {
        let reserved = self.vram.reserved()?;
        let sink = self.backend.table_sink(self.info.ordinal);
        let root = self.backend.root_slot(self.info.ordinal);
        let mut slot = lock(&self.segments);
        if let Some(shared) = slot.as_ref() {
            return Ok(Arc::clone(shared));
        }
        let built = Arc::new(
            IndexShared::new(Arc::clone(reserved.allocators()), sink, root)?,
        );
        *slot = Some(Arc::clone(&built));
        Ok(built)
    }
    /// Builds the memory manager and the coordinator of an index. Loads nothing.
    pub fn attach_index(
        &self,
        index: IndexId,
        manager: Arc<IndexManager>,
        spec: IndexSpec,
    ) -> Result<Arc<IndexMemory>, GpuError> {
        if !self.offers.contains(&ResourceKind::Index) {
            return Err(
                GpuError::InvalidConfig(
                    "the sub-engine does not offer indexes".to_string(),
                ),
            );
        }
        if !self.vram.arenas_ready() {
            return Err(
                GpuError::InvalidConfig("the VRAM arenas are not reserved".to_string()),
            );
        }
        if lock(&self.indexes).contains_key(&index) {
            return Err(GpuError::AlreadyAttached(index));
        }
        let shared = self.index_shared()?;
        let store = Arc::new(MappingStore::new());
        let eviction = Arc::new(
            Eviction::new(
                index.clone(),
                Arc::clone(shared.swap()),
                Arc::clone(&store),
                Arc::clone(shared.allocators()),
                TierBudget::for_vram(
                    &self.cfg.arenas,
                    self.cfg.generations.max(1),
                    self.cfg.headroom,
                ),
                TierBudget::default(),
            ),
        );
        let services = IndexServices::new(
            self.info.ordinal,
            Arc::clone(&self.backend),
            shared,
            store,
            eviction,
        );
        let queue = LoadedQueue::default();
        let gpu = Arc::new(
            GpuManager::new(
                    Arc::new(services),
                    Arc::new(ManagedIndex::new(Arc::clone(&manager))),
                    Arc::clone(&self.shared.resolver),
                    self.shared.cell_selector.clone(),
                    queue.clone(),
                    GpuManagerConfig {
                        class: spec.class,
                        dim: spec.dim,
                        dtype: spec.dtype,
                        segment_bytes: spec.segment_bytes,
                    },
                )
                .map_err(GpuError::IndexMemory)?,
        );
        let ops = Arc::new(IndexOperations::new(manager, Arc::clone(&gpu), queue));
        let memory = Arc::new(IndexMemory { gpu, ops });
        let mut indexes = lock(&self.indexes);
        if indexes.contains_key(&index) {
            return Err(GpuError::AlreadyAttached(index));
        }
        indexes.insert(index, Arc::clone(&memory));
        Ok(memory)
    }
    pub fn index_memory(&self, index: &IndexId) -> Option<Arc<IndexMemory>> {
        lock(&self.indexes).get(index).cloned()
    }
    /// Loads the entries into VRAM in batches of `upload_batch`.
    pub fn ensure_entries(
        &self,
        index: &IndexId,
        ids: &[EntryId],
    ) -> Result<EnsureReport, GpuError> {
        let memory = self
            .index_memory(index)
            .ok_or_else(|| GpuError::UnknownIndex(index.clone()))?;
        let mut seen = HashSet::with_capacity(ids.len());
        let unique: Vec<EntryId> = ids
            .iter()
            .copied()
            .filter(|id| seen.insert(*id))
            .collect();
        let mut report = EnsureReport::default();
        for chunk in unique.chunks(self.cfg.upload_batch.max(1)) {
            let part = memory
                .ops
                .load(|_, _| {
                    memory
                        .gpu
                        .ensure(&EntrySet::from_ids(chunk.iter().map(|id| id.0)))
                        .map_err(IndexOpError::from)
                })
                .map_err(GpuError::IndexOp)?;
            merge(&mut report, part);
        }
        Ok(report)
    }
    /// Builds a lane resource with `worker_lanes` new Query lanes, each with its own stream.
    pub fn build_lane_resource<M, P>(
        &self,
        cfg: LaneResourceConfig<M, P>,
        runner: Arc<dyn BatchRunner<P>>,
    ) -> Result<Arc<LaneResource<M, P>>, GpuError>
    where
        P: BatchPayload,
        M: BatchMeta + HasResourceRequirements + Send + Sync + 'static,
    {
        if cfg.device != self.device() {
            return Err(
                GpuError::InvalidConfig(
                    format!(
                        "lane resource device is {}, expected {}", cfg.device, self
                        .device()
                    ),
                ),
            );
        }
        if cfg.completion.is_none() {
            return Err(
                GpuError::InvalidConfig(
                    "a GPU lane resource needs a completion tracker".to_string(),
                ),
            );
        }
        let lane_cfg = lane_config(
            LaneKind::Query,
            self.device(),
            self.cfg.lane_workspace_bytes,
        );
        let lanes = (0..self.cfg.worker_lanes)
            .map(|_| self.lanes.create(&lane_cfg))
            .collect::<Result<Vec<Lane>, String>>()
            .map_err(GpuError::Backend)?;
        LaneResource::new(cfg, lanes, runner).map_err(GpuError::LaneSet)
    }
}
fn merge(into: &mut EnsureReport, from: EnsureReport) {
    into.loaded.extend(from.loaded);
    into.already_resident.extend(from.already_resident);
    into.absent_in_index.extend(from.absent_in_index);
    into.failed.extend(from.failed);
    into.bytes_moved = into.bytes_moved.saturating_add(from.bytes_moved);
    into.segments_created = into.segments_created.saturating_add(from.segments_created);
}
fn model_ready(registry: &ResourceRegistry<LoadedEncoder>, key: &str) -> bool {
    registry.state(key) == Some(ResourceState::Ready)
}
impl Subengine for GpuSubengine {
    fn name(&self) -> String {
        self.core.name()
    }
    fn start(&self) -> Result<(), String> {
        self.core.start()
    }
    fn capabilities(&self) -> Vec<(QueueKind, CapabilitySet)> {
        self.core.capabilities()
    }
    fn offers(&self) -> Vec<ResourceKind> {
        self.core.offers()
    }
    fn announce(&self, req: &ResourceRequirements) {
        self.core.announce(req);
    }
    fn activate(&self) -> Result<(), String> {
        self.vram.ensure_arenas().map_err(|e| e.to_string())?;
        self.core.activate()
    }
    fn is_loaded(&self, kind: ResourceKind, model: Option<&ModelId>) -> bool {
        if !self.offers.contains(&kind) {
            return false;
        }
        if kind != ResourceKind::Encoder {
            return self.core.is_loaded(kind, model);
        }
        let models = lock(&self.models);
        match model {
            Some(m) => models.get(m).is_some_and(|key| model_ready(&self.encoders, key)),
            None => models.values().any(|key| model_ready(&self.encoders, key)),
        }
    }
}
impl BackendFactory for GpuSubengine {
    fn create(
        &self,
        id: &IndexId,
        spec: &IndexSpec,
        index: Arc<IndexManager>,
    ) -> Result<Arc<dyn ResidencyBackend>, String> {
        let memory = self
            .attach_index(id.clone(), index, *spec)
            .map_err(|e| e.to_string())?;
        Ok(Arc::clone(&memory.ops) as Arc<dyn ResidencyBackend>)
    }
}
#[cfg(test)]
#[path = "test/subengine.rs"]
mod tests;
