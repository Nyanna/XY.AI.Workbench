//! Idle-notification sets per sub-engine kind and the expansion preference.
//!
//! Notifications are only a hint: `select` never withholds an expansion.
use std::collections::HashMap;
use std::sync::{Mutex, MutexGuard, PoisonError};
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::a::resource_lifecycle::ResourceKey;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::a::load_meter::LoadReport;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::a::resource_requirements::ResourceKind;
use crate::resident::_d::_c::_b::_a::_9::_8::subengine_loop::{
    IdleNotice, IterationOutcome,
};
/// A task waiting for expansion, with the advance report for its first operation.
#[derive(Debug, Clone)]
pub struct Candidate3<T> {
    pub item: T,
    /// `None` means unknown or not reported.
    pub first_kind: Option<ResourceKind>,
}
type NoticeSets = HashMap<ResourceKind, HashMap<ResourceKey, IdleNotice>>;
/// Idle-notification sets per sub-engine kind plus expansion decisions.
#[derive(Debug, Default)]
pub struct ExpansionControl {
    state: Mutex<NoticeSets>,
}
impl ExpansionControl {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn report_iteration(&self, kind: ResourceKind, outcome: &IterationOutcome) {
        let mut state = self.lock();
        if outcome.claimed > 0 {
            state.remove(&kind);
            return;
        }
        if outcome.idle.is_empty() {
            return;
        }
        let set = state.entry(kind).or_default();
        for notice in &outcome.idle {
            set.insert(notice.key.clone(), notice.clone());
        }
    }
    pub fn observe_load(&self, report: &LoadReport) {
        if report.capacity == 0 {
            self.lock().remove(&report.kind);
        }
    }
    pub fn mark_busy(&self, kind: ResourceKind, key: &str) {
        let mut state = self.lock();
        if let Some(set) = state.get_mut(&kind) {
            set.remove(key);
            if set.is_empty() {
                state.remove(&kind);
            }
        }
    }
    pub fn notifications(&self, kind: ResourceKind) -> Vec<IdleNotice> {
        self.lock()
            .get(&kind)
            .map(|set| set.values().cloned().collect())
            .unwrap_or_default()
    }
    pub fn snapshot(&self) -> Vec<IdleNotice> {
        self.lock().values().flat_map(|set| set.values().cloned()).collect()
    }
    /// Kinds with at least one notification, in ascending kind order.
    pub fn underserved(&self) -> Vec<ResourceKind> {
        let mut kinds = self.underserved_unsorted();
        kinds.sort();
        kinds
    }
    pub fn has_idle(&self) -> bool {
        self.lock().values().any(|set| !set.is_empty())
    }
    /// Index of the first candidate whose first kind is underserved, else `0`.
    /// `None` only for empty `candidates`.
    pub fn select<T>(&self, candidates: &[Candidate3<T>]) -> Option<usize> {
        if candidates.is_empty() {
            return None;
        }
        let underserved = self.underserved_unsorted();
        let preferred = candidates
            .iter()
            .position(|candidate| {
                candidate.first_kind.is_some_and(|kind| underserved.contains(&kind))
            });
        Some(preferred.unwrap_or(0))
    }
    fn underserved_unsorted(&self) -> Vec<ResourceKind> {
        self.lock()
            .iter()
            .filter(|(_, set)| !set.is_empty())
            .map(|(kind, _)| *kind)
            .collect()
    }
    fn lock(&self) -> MutexGuard<'_, NoticeSets> {
        self.state.lock().unwrap_or_else(PoisonError::into_inner)
    }
}
#[cfg(test)]
#[path = "test/backpressure.rs"]
mod tests;
