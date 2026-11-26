//! Operations unit of one index: publishes loaded segments, retries a load once after making
//! room, and drives the residency backend contract.
use std::fmt;
use std::sync::{Arc, Mutex};
use log::{info, warn};
use super::eviction::EvictionError;
use super::gpu_manager::{GpuManager, GpuManagerError, LoadedQueue};
use crate::resident::_d::_c::manager::LazyIndex;
use crate::resident::_d::_c::_b::_a::_9::a::manager_lifecycle::{
    IndexManager, ManagerError,
};
use crate::resident::_d::_c::_b::_a::_9::a::residency_control::{
    EnsureSummary, ReconcileSummary, ResidencyBackend,
};
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::a::segment_arena::SegmentArenaError;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::a::index_contract::{
    EntryId, Index, IndexError,
};
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::_1::scopes_and_entry_sets::Scope;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::_1::sync_util::lock;
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum IndexOpError {
    ArenaExhausted { requested: u64, available: u64 },
    Transfer(String),
}
impl fmt::Display for IndexOpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IndexOpError::ArenaExhausted { .. } => f.write_str("Arena erschöpft"),
            IndexOpError::Transfer(text) => f.write_str(text),
        }
    }
}
impl std::error::Error for IndexOpError {}
impl From<ManagerError> for IndexOpError {
    fn from(e: ManagerError) -> Self {
        IndexOpError::Transfer(e.to_string())
    }
}
impl From<GpuManagerError> for IndexOpError {
    fn from(e: GpuManagerError) -> Self {
        match e {
            GpuManagerError::Arena(
                SegmentArenaError::Exhausted { requested, available },
            ) => {
                IndexOpError::ArenaExhausted {
                    requested,
                    available,
                }
            }
            other => IndexOpError::Transfer(other.to_string()),
        }
    }
}
impl From<EvictionError> for IndexOpError {
    fn from(e: EvictionError) -> Self {
        match e {
            EvictionError::Exhausted { requested, available } => {
                IndexOpError::ArenaExhausted {
                    requested,
                    available,
                }
            }
        }
    }
}
impl From<IndexError> for IndexOpError {
    fn from(e: IndexError) -> Self {
        IndexOpError::Transfer(e.to_string())
    }
}
/// Gives `GpuManager` the index that the `IndexManager` has attached.
pub struct ManagedIndex {
    manager: Arc<IndexManager>,
}
impl ManagedIndex {
    pub fn new(manager: Arc<IndexManager>) -> ManagedIndex {
        ManagedIndex { manager }
    }
}
impl LazyIndex for ManagedIndex {
    fn get(&self) -> Result<Arc<dyn Index>, String> {
        self.manager.index().map_err(|e| e.to_string())
    }
}
pub struct IndexOperations {
    manager: Arc<IndexManager>,
    gpu: Arc<GpuManager>,
    queue: LoadedQueue,
    publish: Mutex<()>,
}
static_assertions::assert_impl_all!(IndexOpError : Send, Sync);
static_assertions::assert_impl_all!(ManagedIndex : Send, Sync);
static_assertions::assert_impl_all!(IndexOperations : Send, Sync);
struct PublishReport {
    published: usize,
    first_error: Option<IndexOpError>,
}
struct Attempt<R> {
    result: Result<R, IndexOpError>,
    /// Bytes requested by the operation when it reported exhaustion.
    exhausted: Option<u64>,
}
fn settle<R>(
    outcome: Result<R, IndexOpError>,
    published: Result<usize, IndexOpError>,
) -> Result<R, IndexOpError> {
    match outcome {
        Ok(value) => published.map(|_| value),
        Err(e) => {
            if let Err(publish_error) = published {
                warn!(
                    "index_ops: publication after a failed operation: {publish_error}"
                );
            }
            Err(e)
        }
    }
}
impl IndexOperations {
    pub fn new(
        manager: Arc<IndexManager>,
        gpu: Arc<GpuManager>,
        queue: LoadedQueue,
    ) -> IndexOperations {
        IndexOperations {
            manager,
            gpu,
            queue,
            publish: Mutex::new(()),
        }
    }
    pub fn loaded_queue(&self) -> LoadedQueue {
        self.queue.clone()
    }
    pub fn load<R>(
        &self,
        mut op: impl FnMut(&Arc<dyn Index>, &LoadedQueue) -> Result<R, IndexOpError>,
    ) -> Result<R, IndexOpError> {
        let cycle = self
            .manager
            .run_cycle("load", |index| self.load_with_retry(index, &mut op))
            .map_err(IndexOpError::from)?;
        cycle
    }
    pub fn reconcile_iteration<R>(
        &self,
        op: impl FnOnce(&Arc<dyn Index>, &LoadedQueue) -> Result<R, IndexOpError>,
    ) -> Result<R, IndexOpError> {
        let cycle = self
            .manager
            .run_cycle(
                "reconcile",
                |index| {
                    let outcome = op(index, &self.queue);
                    let published = self.publish_pending();
                    settle(outcome, published)
                },
            );
        match cycle {
            Ok(result) => result,
            Err(e) => {
                if let Err(publish_error) = self.publish_pending() {
                    warn!("index_ops: publication without an index: {publish_error}");
                }
                Err(e.into())
            }
        }
    }
    pub fn publish_pending(&self) -> Result<usize, IndexOpError> {
        let _publish = lock(&self.publish);
        let report = self.publish_drained();
        match report.first_error {
            Some(e) => Err(e),
            None => Ok(report.published),
        }
    }
    fn publish_drained(&self) -> PublishReport {
        let mut report = PublishReport {
            published: 0,
            first_error: None,
        };
        for segment in self.queue.drain() {
            match self.gpu.publish_segment(segment) {
                Ok(_) => report.published += 1,
                Err(e) => {
                    warn!("index_ops: segment not published: {e}");
                    if report.first_error.is_none() {
                        report.first_error = Some(e.into());
                    }
                }
            }
        }
        report
    }
    fn attempt<R>(
        &self,
        index: &Arc<dyn Index>,
        op: &mut impl FnMut(&Arc<dyn Index>, &LoadedQueue) -> Result<R, IndexOpError>,
    ) -> Attempt<R> {
        let outcome = op(index, &self.queue);
        let exhausted = match &outcome {
            Err(IndexOpError::ArenaExhausted { requested, .. }) => Some(*requested),
            _ => None,
        };
        let published = self.publish_pending();
        Attempt {
            result: settle(outcome, published),
            exhausted,
        }
    }
    fn load_with_retry<R>(
        &self,
        index: &Arc<dyn Index>,
        op: &mut impl FnMut(&Arc<dyn Index>, &LoadedQueue) -> Result<R, IndexOpError>,
    ) -> Result<R, IndexOpError> {
        let first = self.attempt(index, op);
        let Some(requested) = first.exhausted else {
            return first.result;
        };
        let outcome = self.gpu.make_room(requested)?;
        if let Some(abort) = outcome.aborted {
            return Err(
                IndexOpError::Transfer(
                    format!(
                        "eviction aborted at segment {}: {:?}", abort.key.0, abort.cause
                    ),
                ),
            );
        }
        info!(
            "index_ops: made room for {requested} bytes by evicting {} segments, retrying",
            outcome.decisions.len()
        );
        self.attempt(index, op).result
    }
}
impl ResidencyBackend for IndexOperations {
    fn ensure(&self, scope: &Scope) -> Result<EnsureSummary, String> {
        let report = self
            .load(|_, _| {
                let set = self.gpu.resolve_scope(scope)?;
                Ok(self.gpu.ensure(&set)?)
            })
            .map_err(|e| e.to_string())?;
        Ok(EnsureSummary {
            loaded: report.loaded.len() as u64,
            already_resident: report.already_resident.len() as u64,
            missing: report.absent_in_index.len() as u64,
            failed: report.failed.len() as u64,
        })
    }
    fn reconcile(&self) -> Result<Option<ReconcileSummary>, String> {
        let outcome = self
            .reconcile_iteration(|_, _| Ok(self.gpu.reconcile_iteration()?))
            .map_err(|e| e.to_string())?;
        Ok(
            outcome
                .map(|o| ReconcileSummary {
                    removed: o.reconciliation.removed.len() as u64,
                    reloaded: o.reloaded as u64,
                    current: o.reconciliation.current,
                }),
        )
    }
    fn pending(&self) -> bool {
        self.gpu.signal_pending()
    }
    fn is_resident(&self, entry: EntryId) -> bool {
        self.gpu.slot_of(entry).is_some()
    }
    fn run_gap(&self) -> usize {
        let published = {
            let _publish = lock(&self.publish);
            let report = self.publish_drained();
            if let Some(e) = &report.first_error {
                warn!("index_ops: publication in the gap failed: {e}");
            }
            report.published
        };
        let gap = self.gpu.run_gap();
        published + gap.slots_hidden + gap.reclaimed
    }
    fn shutdown(&self) -> Result<usize, String> {
        let _publish = lock(&self.publish);
        let report = self.publish_drained();
        if let Some(e) = &report.first_error {
            warn!("index_ops: publication at shutdown failed: {e}");
        }
        let mut released = 0;
        let mut first_error: Option<IndexOpError> = None;
        for id in self.gpu.segment_ids() {
            match self.gpu.release_segment(id) {
                Ok(_) => released += 1,
                Err(e) => {
                    warn!("index_ops: segment {} not released: {e}", id.0);
                    if first_error.is_none() {
                        first_error = Some(e.into());
                    }
                }
            }
        }
        match first_error {
            Some(e) => Err(e.to_string()),
            None => Ok(released),
        }
    }
}
#[cfg(test)]
#[path = "test/index_ops.rs"]
mod tests;
