//! Resource requirements of an operation.
use crate::resident::memory::{DeviceKind, Region};
use crate::resident::precision::CapabilitySet;
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ModelId(pub String);
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct IndexId(pub String);
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct DeviceId(pub u32);
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ResourceKind {
    Tokenizer,
    Normalizer,
    Encoder,
    Scorer,
    Index,
    CrossEncoder,
}
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum EngineSelector {
    Cpu,
    Gpu,
    Either,
}
impl EngineSelector {
    pub fn accepts(self, dev: DeviceKind) -> bool {
        matches!(
            (self, dev), (EngineSelector::Either, _) | (EngineSelector::Cpu,
            DeviceKind::Cpu) | (EngineSelector::Gpu, DeviceKind::Gpu)
        )
    }
}
/// Placement hint; inactive in the scheduler for now.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Affinity {
    Handle(Region),
    Device(DeviceId),
}
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Requirements {
    pub resource: ResourceKind,
    pub model: Option<ModelId>,
    pub index: Option<IndexId>,
    pub engine: EngineSelector,
    pub caps: CapabilitySet,
    pub affinity: Option<Affinity>,
}
impl Requirements {
    pub fn new(resource: ResourceKind, engine: EngineSelector) -> Self {
        Self {
            resource,
            model: None,
            index: None,
            engine,
            caps: CapabilitySet::new(),
            affinity: None,
        }
    }
    pub fn with_model(mut self, model: ModelId) -> Self {
        self.model = Some(model);
        self
    }
    pub fn with_index(mut self, index: IndexId) -> Self {
        self.index = Some(index);
        self
    }
    pub fn with_caps(mut self, caps: CapabilitySet) -> Self {
        self.caps = caps;
        self
    }
    pub fn with_affinity(mut self, affinity: Affinity) -> Self {
        self.affinity = Some(affinity);
        self
    }
    /// Affinity is deliberately ignored.
    pub fn satisfied_by(&self, dev: DeviceKind, available: &CapabilitySet) -> bool {
        self.engine.accepts(dev) && available.satisfies(&self.caps)
    }
}
#[cfg(test)]
#[path = "test/requirements.rs"]
mod tests;
