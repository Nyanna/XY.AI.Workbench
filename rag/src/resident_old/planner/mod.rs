//! Stateless task expansion.

mod adapter;
mod config;
mod expand;
mod offers;
mod rules;

use std::sync::Arc;

use arc_swap::ArcSwap;

use crate::resident::error::PlannerError;
use crate::resident::processor::StarvedSnapshot;

pub use adapter::PlannerExpander;
pub use config::{PipelineConfig, PipelineDtype, Pooling, Prefixes, ScorerKind};
pub use expand::{first_resource, ExpandContext, PlanExpander, PlannedOp, TaskSpec};
pub use offers::EngineOffers;
pub use rules::RuleSet;

/// Each call works on one rule snapshot, so a reload never affects a running expansion.
pub struct Planner {
    rules: ArcSwap<RuleSet>,
}

impl Planner {
    pub fn new(rules: RuleSet) -> Self {
        Self {
            rules: ArcSwap::from_pointee(rules),
        }
    }

    pub fn reload(&self, rules: RuleSet) {
        self.rules.store(Arc::new(rules));
    }

    pub fn snapshot(&self) -> Arc<RuleSet> {
        self.rules.load_full()
    }

    pub fn expand(
        &self,
        task: &TaskSpec<'_>,
        starved: StarvedSnapshot,
        offers: &EngineOffers,
    ) -> Result<Vec<PlannedOp>, PlannerError> {
        let ops = self.rules.load().expand(task, starved, offers)?;
        tracing::debug!(kind = ?task.kind(), ops = ops.len(), "task expanded");
        Ok(ops)
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
