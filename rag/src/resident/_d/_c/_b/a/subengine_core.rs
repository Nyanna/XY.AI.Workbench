//! Rule set and base implementation shared by the CPU and GPU sub-engines: configuration checks,
//! resource ownership, lane activation, queue registrations and compute-path selection.
use std::sync::{Arc, Mutex};
use log::debug;
use crate::resident::_d::_c::_b::_a::a::plan_contract::QueueKind;
use crate::resident::_d::_c::_b::_a::a::planner::Subengine;
use crate::resident::_d::_c::_b::_a::_9::a::model_registry::LoadedEncoder;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::a::kernel_register::{
    Kernel, KernelError, KernelProvider, Variant,
};
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::a::resource_requirements::{
    ExecutionEnvironment, IndexId, ModelId, ResourceKind, ResourceRequirements,
};
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::a::region_handle::RegionHandle;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::_1::capability_model::{
    axes, cpu_reference, gpu_reference, CapabilitySet,
};
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::_1::device_lane_event::{
    DeviceHandle, DeviceKind, Lane, LaneConfig, LaneKind,
};
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::_1::layout::{
    DType, Layout,
};
/// Numeric tuning axes have no fixed value set, so the reference does not bound them.
fn within_reference(own: &CapabilitySet, reference: &CapabilitySet) -> bool {
    let tuning = [axes::batch_size(), axes::tile_size(), axes::thread_count()];
    own.iter()
        .all(|(axis, value)| tuning.contains(axis) || reference.contains(axis, value))
}
/// An exclusively usable lane. GPU lanes always have a stream.
pub type SharedLane = Arc<Mutex<Lane>>;
/// Index segment owned by the sub-engine. Region and layout only.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IndexSegment {
    pub index: IndexId,
    pub region: RegionHandle,
    pub layout: Layout,
}
/// Chosen compute path. Produced only by the sub-engine's own `PathPolicy`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ComputePath {
    /// Name in the kernel register.
    pub kernel: String,
    pub dtype: DType,
    pub variant: Variant,
    /// Lane that must run the path.
    pub lane: LaneKind,
}
/// Hardware-specific choice of the compute path. The sub-engine alone decides.
pub trait PathPolicy: Send + Sync {
    /// `None`: no path for these requirements on this hardware.
    fn select(
        &self,
        req: &ResourceRequirements,
        caps: &CapabilitySet,
        device: DeviceHandle,
    ) -> Option<ComputePath>;
}
/// Creates lanes. For GPU devices the returned lane must have a stream.
pub trait LaneFactory: Send + Sync {
    fn create(&self, cfg: &LaneConfig) -> Result<Lane, String>;
}
/// One resource made visible to workers.
#[derive(Clone)]
pub struct Registration {
    pub subengine: String,
    pub queue: QueueKind,
    pub kind: ResourceKind,
    pub model: Option<ModelId>,
    pub index: Option<IndexId>,
    /// Set the registration was checked against.
    pub capabilities: CapabilitySet,
    pub device: DeviceHandle,
    pub lanes: Vec<(LaneKind, SharedLane)>,
}
impl Registration {
    fn same_key(&self, other: &Registration) -> bool {
        self.subengine == other.subengine && self.queue == other.queue
            && self.kind == other.kind && self.model == other.model
            && self.index == other.index
    }
}
/// Registrations of all queues, shared between all sub-engines.
#[derive(Default)]
pub struct QueueRegistrations {
    entries: Mutex<Vec<Registration>>,
}
impl QueueRegistrations {
    pub fn new() -> Self {
        Self::default()
    }
    /// Snapshot in registration order.
    pub fn entries(&self, queue: QueueKind) -> Vec<Registration> {
        lock(&self.entries).iter().filter(|r| r.queue == queue).cloned().collect()
    }
    pub fn entries_of(&self, queue: QueueKind, kind: ResourceKind) -> Vec<Registration> {
        lock(&self.entries)
            .iter()
            .filter(|r| r.queue == queue && r.kind == kind)
            .cloned()
            .collect()
    }
    /// Inserts a registration or replaces the one with the same key in place.
    fn upsert(&self, reg: Registration) {
        let mut all = lock(&self.entries);
        match all.iter_mut().find(|r| r.same_key(&reg)) {
            Some(slot) => *slot = reg,
            None => all.push(reg),
        }
    }
}
pub struct SubengineConfig {
    /// Unique within one planner.
    pub name: String,
    pub device: DeviceHandle,
    /// `Cpu` for `DeviceHandle::Cpu`, `Gpu` for `DeviceHandle::Gpu`.
    pub own_queue: QueueKind,
    pub own_capabilities: CapabilitySet,
    /// Reported for `Shared`; must be a subset of `own_capabilities`.
    pub shared_capabilities: CapabilitySet,
    pub offers: Vec<ResourceKind>,
    /// One lane per entry, all on `device`.
    pub lanes: Vec<LaneConfig>,
}
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum SubengineError {
    #[error("ungültige Subengine-Konfiguration: {0}")]
    InvalidConfig(String),
    #[error("Lane {kind:?} nicht erstellbar: {reason}")]
    LaneCreation { kind: LaneKind, reason: String },
    #[error("GPU-Lane {kind:?} ohne Stream (Default-Stream ist verboten)")]
    LaneWithoutStream { kind: LaneKind },
    #[error("Subengine nicht aktiviert")]
    NotActive,
    #[error("kein Compute-Pfad für {kind:?}")]
    NoPath { kind: ResourceKind },
    #[error("Lane {0:?} nicht vorhanden")]
    UnknownLane(LaneKind),
    #[error("{0}")]
    Kernel(KernelError),
    #[error("Ressource passt nicht zur Subengine: {0}")]
    Incompatible(String),
}
#[derive(Default)]
struct State8 {
    pending: Vec<ResourceRequirements>,
    encoders: Vec<(ModelId, Arc<LoadedEncoder>)>,
    scorer: bool,
    segments: Vec<IndexSegment>,
    lanes: Vec<(LaneKind, SharedLane)>,
    active: bool,
}
pub struct SubengineCore {
    cfg: SubengineConfig,
    lane_factory: Arc<dyn LaneFactory>,
    policy: Arc<dyn PathPolicy>,
    kernels: Arc<dyn KernelProvider>,
    registrations: Arc<QueueRegistrations>,
    /// Serializes `activate`; never held together with a state lock across a factory call.
    activation: Mutex<()>,
    state: Mutex<State8>,
}
impl SubengineCore {
    /// Validates only; creates no lane and starts nothing.
    pub fn new(
        mut cfg: SubengineConfig,
        lanes: Arc<dyn LaneFactory>,
        policy: Arc<dyn PathPolicy>,
        kernels: Arc<dyn KernelProvider>,
        registrations: Arc<QueueRegistrations>,
    ) -> Result<Self, SubengineError> {
        let invalid = |text: String| -> Result<Self, SubengineError> {
            Err(SubengineError::InvalidConfig(text))
        };
        let (expected_queue, reference) = match cfg.device.kind() {
            DeviceKind::Cpu => (QueueKind::Cpu, cpu_reference()),
            DeviceKind::Gpu => (QueueKind::Gpu, gpu_reference()),
        };
        if cfg.own_queue != expected_queue {
            return invalid(
                format!(
                    "own queue {:?} does not match device {}", cfg.own_queue, cfg.device
                ),
            );
        }
        if !cfg.shared_capabilities.is_within(&cfg.own_capabilities) {
            return invalid(
                "shared capabilities exceed the own capabilities".to_string(),
            );
        }
        if !within_reference(&cfg.own_capabilities, &reference) {
            return invalid(
                format!("own capabilities exceed the reference of device {}", cfg.device),
            );
        }
        if let Some(l) = cfg.lanes.iter().find(|l| l.device != cfg.device) {
            return invalid(
                format!(
                    "lane {:?} is configured for device {}, expected {}", l.kind, l
                    .device, cfg.device
                ),
            );
        }
        for (i, l) in cfg.lanes.iter().enumerate() {
            if cfg.lanes[..i].iter().any(|p| p.kind == l.kind) {
                return invalid(format!("lane kind {:?} is configured twice", l.kind));
            }
        }
        if cfg.name.is_empty() {
            return invalid("name is empty".to_string());
        }
        let mut offers = Vec::with_capacity(cfg.offers.len());
        for kind in cfg.offers.drain(..) {
            if !offers.contains(&kind) {
                offers.push(kind);
            }
        }
        cfg.offers = offers;
        Ok(Self {
            cfg,
            lane_factory: lanes,
            policy,
            kernels,
            registrations,
            activation: Mutex::new(()),
            state: Mutex::new(State8::default()),
        })
    }
    pub fn is_active(&self) -> bool {
        lock(&self.state).active
    }
    pub fn device(&self) -> DeviceHandle {
        self.cfg.device
    }
    /// Adopts an already loaded encoder; loads nothing.
    pub fn adopt_encoder(
        &self,
        model: ModelId,
        enc: Arc<LoadedEncoder>,
    ) -> Result<(), SubengineError> {
        self.require_offer(ResourceKind::Encoder)?;
        let mut st = lock(&self.state);
        match st.encoders.iter_mut().find(|(m, _)| *m == model) {
            Some(slot) => slot.1 = enc,
            None => st.encoders.push((model.clone(), enc)),
        }
        if st.active {
            self.register(&st, ResourceKind::Encoder, Some(&model), None);
        }
        Ok(())
    }
    /// Makes the scorer available; it counts as loaded once its kernel is resident.
    pub fn adopt_scorer(&self) {
        let mut st = lock(&self.state);
        st.scorer = true;
        if st.active {
            self.register(&st, ResourceKind::Scorer, None, None);
        }
    }
    /// Segments are identified by index and region; adopting one again replaces it.
    pub fn adopt_index_segment(&self, seg: IndexSegment) -> Result<(), SubengineError> {
        self.require_offer(ResourceKind::Index)?;
        let mut st = lock(&self.state);
        let index = seg.index.clone();
        match st
            .segments
            .iter_mut()
            .find(|s| s.index == seg.index && s.region == seg.region)
        {
            Some(slot) => *slot = seg,
            None => st.segments.push(seg),
        }
        if st.active {
            self.register(&st, ResourceKind::Index, None, Some(&index));
        }
        Ok(())
    }
    pub fn encoder(&self, model: &ModelId) -> Option<Arc<LoadedEncoder>> {
        lock(&self.state)
            .encoders
            .iter()
            .find(|(m, _)| m == model)
            .map(|(_, e)| Arc::clone(e))
    }
    pub fn index_segments(&self, index: &IndexId) -> Vec<IndexSegment> {
        lock(&self.state)
            .segments
            .iter()
            .filter(|s| s.index == *index)
            .cloned()
            .collect()
    }
    /// Requirements announced since the last `activate`, deduplicated, in announcement order.
    pub fn pending_announcements(&self) -> Vec<ResourceRequirements> {
        lock(&self.state).pending.clone()
    }
    /// Asks the sub-engine's own policy; nobody else chooses a path.
    pub fn select_path(
        &self,
        req: &ResourceRequirements,
    ) -> Result<ComputePath, SubengineError> {
        let no_path = SubengineError::NoPath {
            kind: req.kind(),
        };
        if !req.environment().accepts(self.cfg.device.kind()) {
            return Err(no_path);
        }
        self.policy
            .select(req, &self.cfg.own_capabilities, self.cfg.device)
            .ok_or(no_path)
    }
    /// The only way a kernel is obtained.
    pub fn kernel(&self, path: &ComputePath) -> Result<Kernel, SubengineError> {
        self.kernels
            .request(&path.kernel, path.dtype, path.variant)
            .map_err(SubengineError::Kernel)
    }
    /// Runs `f` with exclusive access to the lane of that kind.
    pub fn with_lane<T>(
        &self,
        kind: LaneKind,
        f: impl FnOnce(&mut Lane) -> T,
    ) -> Result<T, SubengineError> {
        let lane = {
            let st = lock(&self.state);
            if !st.active {
                return Err(SubengineError::NotActive);
            }
            st.lanes.iter().find(|(k, _)| *k == kind).map(|(_, l)| Arc::clone(l))
        }
            .ok_or(SubengineError::UnknownLane(kind))?;
        let mut guard = lock(lane.as_ref());
        Ok(f(&mut guard))
    }
    fn require_offer(&self, kind: ResourceKind) -> Result<(), SubengineError> {
        if self.cfg.offers.contains(&kind) {
            Ok(())
        } else {
            Err(
                SubengineError::Incompatible(
                    format!("{kind:?} is not offered by {}", self.cfg.name),
                ),
            )
        }
    }
    fn scorer_kernel_resident(&self) -> bool {
        let req = ResourceRequirements::new(
            ResourceKind::Scorer,
            ExecutionEnvironment::Any,
        );
        self.select_path(&req)
            .is_ok_and(|path| self.kernels.is_resident(&path.kernel, path.dtype))
    }
    fn activate_inner(&self) -> Result<(), SubengineError> {
        let _serial = lock(&self.activation);
        if lock(&self.state).active {
            self.drain_pending();
            return Ok(());
        }
        let gpu = self.cfg.device.kind() == DeviceKind::Gpu;
        let mut lanes: Vec<(LaneKind, SharedLane)> = Vec::with_capacity(
            self.cfg.lanes.len(),
        );
        for cfg in &self.cfg.lanes {
            let lane = self
                .lane_factory
                .create(cfg)
                .map_err(|reason| SubengineError::LaneCreation {
                    kind: cfg.kind,
                    reason,
                })?;
            if lane.kind() != cfg.kind || lane.device() != cfg.device {
                return Err(SubengineError::LaneCreation {
                    kind: cfg.kind,
                    reason: format!(
                        "factory returned lane {:?} on {} instead", lane.kind(), lane
                        .device()
                    ),
                });
            }
            if gpu && lane.stream().is_none() {
                return Err(SubengineError::LaneWithoutStream {
                    kind: cfg.kind,
                });
            }
            lanes.push((cfg.kind, Arc::new(Mutex::new(lane))));
        }
        {
            let mut st = lock(&self.state);
            st.lanes = lanes;
            st.active = true;
            for (model, _) in &st.encoders {
                self.register(&st, ResourceKind::Encoder, Some(model), None);
            }
            if st.scorer {
                self.register(&st, ResourceKind::Scorer, None, None);
            }
            for seg in &st.segments {
                self.register(&st, ResourceKind::Index, None, Some(&seg.index));
            }
        }
        self.drain_pending();
        Ok(())
    }
    /// Announced requirements are consumed by an activation; nothing is loaded for them.
    fn drain_pending(&self) {
        let mut st = lock(&self.state);
        for req in std::mem::take(&mut st.pending) {
            if !Self::is_owned(&st, &req) {
                debug!(
                    "{}: no owned resource for announced {:?}", self.cfg.name, req.kind()
                );
            }
        }
    }
    fn is_owned(st: &State8, req: &ResourceRequirements) -> bool {
        match req.kind() {
            ResourceKind::Encoder => {
                match req.model() {
                    Some(m) => st.encoders.iter().any(|(id, _)| id == m),
                    None => !st.encoders.is_empty(),
                }
            }
            ResourceKind::Scorer => st.scorer,
            ResourceKind::Index => {
                match req.index() {
                    Some(i) => st.segments.iter().any(|s| s.index == *i),
                    None => !st.segments.is_empty(),
                }
            }
            _ => false,
        }
    }
    /// Registers one resource in every queue it is compatible with. Needs the state lock.
    fn register(
        &self,
        st: &State8,
        kind: ResourceKind,
        model: Option<&ModelId>,
        index: Option<&IndexId>,
    ) {
        if !self.cfg.offers.contains(&kind) {
            return;
        }
        let mut req = ResourceRequirements::new(kind, ExecutionEnvironment::Any);
        if let Some(m) = model {
            req = req.with_model(m.clone());
        }
        if let Some(i) = index {
            req = req.with_index(i.clone());
        }
        let mut targets = vec![(self.cfg.own_queue, & self.cfg.own_capabilities)];
        if !self.cfg.shared_capabilities.is_empty() {
            targets.push((QueueKind::Shared, &self.cfg.shared_capabilities));
        }
        for (queue, caps) in targets {
            if req.is_satisfied_by(self.cfg.device.kind(), caps) {
                self.registrations
                    .upsert(Registration {
                        subengine: self.cfg.name.clone(),
                        queue,
                        kind,
                        model: model.cloned(),
                        index: index.cloned(),
                        capabilities: caps.clone(),
                        device: self.cfg.device,
                        lanes: st.lanes.clone(),
                    });
            }
        }
    }
}
impl Subengine for SubengineCore {
    fn name(&self) -> String {
        self.cfg.name.clone()
    }
    fn start(&self) -> Result<(), String> {
        Ok(())
    }
    fn capabilities(&self) -> Vec<(QueueKind, CapabilitySet)> {
        let mut out = vec![(self.cfg.own_queue, self.cfg.own_capabilities.clone())];
        if !self.cfg.shared_capabilities.is_empty() {
            out.push((QueueKind::Shared, self.cfg.shared_capabilities.clone()));
        }
        out
    }
    fn offers(&self) -> Vec<ResourceKind> {
        self.cfg.offers.clone()
    }
    fn announce(&self, req: &ResourceRequirements) {
        let mut st = lock(&self.state);
        if !st.pending.contains(req) {
            st.pending.push(req.clone());
        }
    }
    fn activate(&self) -> Result<(), String> {
        self.activate_inner().map_err(|e| e.to_string())
    }
    fn is_loaded(&self, kind: ResourceKind, model: Option<&ModelId>) -> bool {
        if !self.cfg.offers.contains(&kind) {
            return false;
        }
        let st = lock(&self.state);
        match kind {
            ResourceKind::Encoder => {
                match model {
                    Some(m) => st.encoders.iter().any(|(id, _)| id == m),
                    None => !st.encoders.is_empty(),
                }
            }
            ResourceKind::Scorer => st.scorer && self.scorer_kernel_resident(),
            ResourceKind::Index => !st.segments.is_empty(),
            _ => st.active,
        }
    }
}
#[cfg(test)]
#[path = "test/subengine_core.rs"]
mod tests;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::_1::sync_util::lock;
