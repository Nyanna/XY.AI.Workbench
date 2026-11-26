use std::collections::HashMap;
use std::fmt;
use std::sync::{Arc, Mutex, PoisonError, RwLock};
use log::warn;
use super::manager_lifecycle::IndexManager;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::a::resource_requirements::IndexId;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::a::slot_layout::{
    slot_size, SlotClass,
};
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::a::index_contract::EntryId;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::_1::layout::DType;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::_1::scopes_and_entry_sets::Scope;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct IndexSpec {
    pub class: SlotClass,
    pub dim: u32,
    pub dtype: DType,
    pub segment_bytes: u64,
}
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct EnsureSummary {
    pub loaded: u64,
    pub already_resident: u64,
    pub missing: u64,
    pub failed: u64,
}
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct ReconcileSummary {
    pub removed: u64,
    pub reloaded: u64,
    pub current: usize,
}
pub trait ResidencyBackend: Send + Sync {
    fn ensure(&self, scope: &Scope) -> Result<EnsureSummary, String>;
    /// `Ok(None)`: the index no longer exists.
    fn reconcile(&self) -> Result<Option<ReconcileSummary>, String>;
    /// Never blocks.
    fn pending(&self) -> bool;
    fn is_resident(&self, entry: EntryId) -> bool;
    /// Returns the number of completed units.
    fn run_gap(&self) -> usize;
    /// Returns the number of released segments.
    fn shutdown(&self) -> Result<usize, String>;
}
pub trait BackendFactory: Send + Sync {
    fn create(
        &self,
        id: &IndexId,
        spec: &IndexSpec,
        index: Arc<IndexManager>,
    ) -> Result<Arc<dyn ResidencyBackend>, String>;
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResidencyError {
    UnknownIndex(IndexId),
    AlreadyRegistered(IndexId),
    InvalidSpec,
    Backend(String),
}
impl fmt::Display for ResidencyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ResidencyError::UnknownIndex(id) => {
                write!(f, "unknown index: {}", id.as_str())
            }
            ResidencyError::AlreadyRegistered(id) => {
                write!(f, "index already registered: {}", id.as_str())
            }
            ResidencyError::InvalidSpec => write!(f, "invalid index spec"),
            ResidencyError::Backend(msg) => write!(f, "backend error: {msg}"),
        }
    }
}
impl std::error::Error for ResidencyError {}
static_assertions::assert_impl_all!(ResidencyController : Send, Sync);
static_assertions::assert_impl_all!(IndexSpec : Send, Sync);
static_assertions::assert_impl_all!(EnsureSummary : Send, Sync);
static_assertions::assert_impl_all!(ReconcileSummary : Send, Sync);
static_assertions::assert_impl_all!(ResidencyError : Send, Sync);
struct Gate {
    closed: bool,
}
struct IndexEntry {
    index: Arc<IndexManager>,
    spec: IndexSpec,
    /// Serializes `ensure`, `reconcile` and `unregister` of one index.
    gate: Mutex<Gate>,
    /// Readable without the gate so that `pending` and `run_gap` never wait on `ensure`.
    backend: RwLock<Option<Arc<dyn ResidencyBackend>>>,
}
impl IndexEntry {
    fn backend(&self) -> Option<Arc<dyn ResidencyBackend>> {
        self.backend.read().unwrap_or_else(PoisonError::into_inner).clone()
    }
    fn set_backend(&self, backend: Option<Arc<dyn ResidencyBackend>>) {
        *self.backend.write().unwrap_or_else(PoisonError::into_inner) = backend;
    }
}
pub struct ResidencyController {
    factory: Arc<dyn BackendFactory>,
    registry: RwLock<HashMap<IndexId, Arc<IndexEntry>>>,
}
impl ResidencyController {
    pub fn new(factory: Arc<dyn BackendFactory>) -> ResidencyController {
        ResidencyController {
            factory,
            registry: RwLock::new(HashMap::new()),
        }
    }
    fn find(&self, id: &IndexId) -> Option<Arc<IndexEntry>> {
        self.registry.read().unwrap_or_else(PoisonError::into_inner).get(id).cloned()
    }
    fn require(&self, id: &IndexId) -> Result<Arc<IndexEntry>, ResidencyError> {
        self.find(id).ok_or_else(|| ResidencyError::UnknownIndex(id.clone()))
    }
    fn entries(&self) -> Vec<Arc<IndexEntry>> {
        self.registry
            .read()
            .unwrap_or_else(PoisonError::into_inner)
            .values()
            .cloned()
            .collect()
    }
    pub fn register(
        &self,
        id: IndexId,
        index: Arc<IndexManager>,
        spec: IndexSpec,
    ) -> Result<(), ResidencyError> {
        let needed = slot_size(spec.class, spec.dim, spec.dtype)
            .map_err(|_| ResidencyError::InvalidSpec)?;
        if spec.dim == 0 || spec.segment_bytes < u64::from(needed) {
            return Err(ResidencyError::InvalidSpec);
        }
        let mut registry = self.registry.write().unwrap_or_else(PoisonError::into_inner);
        if registry.contains_key(&id) {
            return Err(ResidencyError::AlreadyRegistered(id));
        }
        registry
            .insert(
                id,
                Arc::new(IndexEntry {
                    index,
                    spec,
                    gate: Mutex::new(Gate { closed: false }),
                    backend: RwLock::new(None),
                }),
            );
        Ok(())
    }
    pub fn unregister(&self, id: &IndexId) -> Result<usize, ResidencyError> {
        let entry = self.require(id)?;
        let released = {
            let mut gate = lock(&entry.gate);
            if gate.closed {
                return Err(ResidencyError::UnknownIndex(id.clone()));
            }
            let released = match entry.backend() {
                Some(backend) => {
                    backend
                        .shutdown()
                        .map_err(|msg| {
                            warn!("shutdown of index {} failed: {msg}", id.as_str());
                            ResidencyError::Backend(msg)
                        })?
                }
                None => 0,
            };
            gate.closed = true;
            entry.set_backend(None);
            released
        };
        let mut registry = self.registry.write().unwrap_or_else(PoisonError::into_inner);
        if registry.get(id).is_some_and(|e| Arc::ptr_eq(e, &entry)) {
            registry.remove(id);
        }
        Ok(released)
    }
    pub fn ensure(
        &self,
        id: &IndexId,
        scope: &Scope,
    ) -> Result<EnsureSummary, ResidencyError> {
        let entry = self.require(id)?;
        let gate = lock(&entry.gate);
        if gate.closed {
            return Err(ResidencyError::UnknownIndex(id.clone()));
        }
        let backend = match entry.backend() {
            Some(backend) => backend,
            None => {
                let backend = self
                    .factory
                    .create(id, &entry.spec, Arc::clone(&entry.index))
                    .map_err(|msg| {
                        warn!(
                            "backend creation for index {} failed: {msg}", id.as_str()
                        );
                        ResidencyError::Backend(msg)
                    })?;
                entry.set_backend(Some(Arc::clone(&backend)));
                backend
            }
        };
        backend.ensure(scope).map_err(ResidencyError::Backend)
    }
    pub fn reconcile(&self, id: &IndexId) -> Result<ReconcileSummary, ResidencyError> {
        let entry = self.require(id)?;
        let gate = lock(&entry.gate);
        if gate.closed {
            return Err(ResidencyError::UnknownIndex(id.clone()));
        }
        let Some(backend) = entry.backend() else {
            return Ok(ReconcileSummary::default());
        };
        backend
            .reconcile()
            .map(Option::unwrap_or_default)
            .map_err(ResidencyError::Backend)
    }
    pub fn pending(&self, id: &IndexId) -> Result<bool, ResidencyError> {
        let entry = self.require(id)?;
        Ok(entry.backend().is_some_and(|b| b.pending()))
    }
    pub fn run_gap(&self) -> usize {
        self.entries()
            .iter()
            .filter_map(|entry| entry.backend())
            .fold(0usize, |sum, backend| sum.saturating_add(backend.run_gap()))
    }
    pub fn is_resident(&self, id: &IndexId, entry: EntryId) -> bool {
        self.find(id).and_then(|e| e.backend()).is_some_and(|b| b.is_resident(entry))
    }
    pub fn is_active(&self, id: &IndexId) -> bool {
        self.find(id).is_some_and(|e| e.backend().is_some())
    }
    pub fn registered_count(&self) -> usize {
        self.registry.read().unwrap_or_else(PoisonError::into_inner).len()
    }
    pub fn active_count(&self) -> usize {
        self.entries().iter().filter(|e| e.backend().is_some()).count()
    }
}
#[cfg(test)]
#[path = "test/residency_control.rs"]
mod tests;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::_1::sync_util::lock;
