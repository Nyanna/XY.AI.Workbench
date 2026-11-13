use std::cmp::Ordering;
use std::fmt;

use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::a::region_handle::RegionHandle;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::l010::capability_model::{
    Axis, CapabilitySet, Requirements, Value,
};
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::l010::device_lane_event::{
    DeviceHandle, DeviceKind,
};

#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ResourceKind {
    Tokenizer,
    Normalizer,
    Encoder,
    Scorer,
    Index,
    CrossEncoder,
}

#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ExecutionEnvironment {
    CpuOnly,
    GpuOnly,
    Any,
}

impl ExecutionEnvironment {
    pub fn accepts(&self, kind: DeviceKind) -> bool {
        match self {
            ExecutionEnvironment::Any => true,
            ExecutionEnvironment::CpuOnly => matches!(kind, DeviceKind::Cpu),
            ExecutionEnvironment::GpuOnly => matches!(kind, DeviceKind::Gpu),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ModelId(String);

impl ModelId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ModelId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl From<&str> for ModelId {
    fn from(s: &str) -> Self {
        Self(s.to_owned())
    }
}

impl From<String> for ModelId {
    fn from(s: String) -> Self {
        Self(s)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct IndexId(String);

impl IndexId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for IndexId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl From<&str> for IndexId {
    fn from(s: &str) -> Self {
        Self(s.to_owned())
    }
}

impl From<String> for IndexId {
    fn from(s: String) -> Self {
        Self(s)
    }
}

#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Affinity {
    Region(RegionHandle),
    Device(DeviceHandle),
}

fn device_rank(d: &DeviceHandle) -> (u8, u32) {
    match d {
        DeviceHandle::Gpu(n) => (1, *n),
        _ => (0, 0),
    }
}

impl PartialOrd for Affinity {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Affinity {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Affinity::Region(a), Affinity::Region(b)) => a.cmp(b),
            (Affinity::Region(_), Affinity::Device(_)) => Ordering::Less,
            (Affinity::Device(_), Affinity::Region(_)) => Ordering::Greater,
            (Affinity::Device(a), Affinity::Device(b)) => device_rank(a).cmp(&device_rank(b)),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ResourceRequirements {
    kind: ResourceKind,
    model: Option<ModelId>,
    index: Option<IndexId>,
    environment: ExecutionEnvironment,
    capabilities: Requirements,
    affinity: Option<Affinity>,
}

impl ResourceRequirements {
    pub fn new(kind: ResourceKind, environment: ExecutionEnvironment) -> Self {
        Self {
            kind,
            model: None,
            index: None,
            environment,
            capabilities: CapabilitySet::new(),
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

    pub fn with_affinity(mut self, affinity: Affinity) -> Self {
        self.affinity = Some(affinity);
        self
    }

    pub fn with_capability(mut self, axis: impl Into<Axis>, value: impl Into<Value>) -> Self {
        self.capabilities.add(axis, value);
        self
    }

    pub fn with_capabilities(mut self, caps: CapabilitySet) -> Self {
        for (axis, value) in caps.iter() {
            self.capabilities.add(axis.clone(), value.clone());
        }
        self
    }

    pub fn kind(&self) -> ResourceKind {
        self.kind
    }

    pub fn environment(&self) -> ExecutionEnvironment {
        self.environment
    }

    pub fn model(&self) -> Option<&ModelId> {
        self.model.as_ref()
    }

    pub fn index(&self) -> Option<&IndexId> {
        self.index.as_ref()
    }

    pub fn capabilities(&self) -> &Requirements {
        &self.capabilities
    }

    pub fn affinity(&self) -> Option<&Affinity> {
        self.affinity.as_ref()
    }

    pub fn is_satisfied_by(&self, kind: DeviceKind, device_capabilities: &CapabilitySet) -> bool {
        self.environment.accepts(kind) && device_capabilities.satisfies(&self.capabilities)
    }
}

impl PartialOrd for ResourceRequirements {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ResourceRequirements {
    fn cmp(&self, other: &Self) -> Ordering {
        self.kind
            .cmp(&other.kind)
            .then_with(|| self.model.cmp(&other.model))
            .then_with(|| self.index.cmp(&other.index))
            .then_with(|| self.environment.cmp(&other.environment))
            .then_with(|| self.capabilities.iter().cmp(other.capabilities.iter()))
            .then_with(|| self.affinity.cmp(&other.affinity))
    }
}

pub trait HasResourceRequirements {
    fn resource_requirements(&self) -> &ResourceRequirements;
}

#[cfg(test)]
#[path = "test/resource_requirements.rs"]
mod tests;
