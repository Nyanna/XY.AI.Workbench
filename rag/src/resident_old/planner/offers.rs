//! Which resource kinds each engine side can serve.

use std::collections::HashSet;

use crate::resident::error::PlannerError;
use crate::resident::op::{EngineSelector, ResourceKind};
use crate::resident::processor::StarvedSnapshot;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct EngineOffers {
    cpu: HashSet<ResourceKind>,
    gpu: HashSet<ResourceKind>,
}

impl EngineOffers {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_cpu(mut self, kinds: impl IntoIterator<Item = ResourceKind>) -> Self {
        self.cpu.extend(kinds);
        self
    }

    pub fn with_gpu(mut self, kinds: impl IntoIterator<Item = ResourceKind>) -> Self {
        self.gpu.extend(kinds);
        self
    }

    /// A starved kind is offered to either side; otherwise the GPU is preferred.
    /// The hint only picks between equivalent variants and never changes the op set.
    pub fn choose(
        &self,
        kind: ResourceKind,
        starved: StarvedSnapshot,
    ) -> Result<EngineSelector, PlannerError> {
        match (self.cpu.contains(&kind), self.gpu.contains(&kind)) {
            (true, true) if starved.contains(kind) => Ok(EngineSelector::Either),
            (true, true) => Ok(EngineSelector::Gpu),
            (true, false) => Ok(EngineSelector::Cpu),
            (false, true) => Ok(EngineSelector::Gpu),
            (false, false) => Err(PlannerError::MissingCapability {
                capability: format!("{kind:?}"),
            }),
        }
    }
}
