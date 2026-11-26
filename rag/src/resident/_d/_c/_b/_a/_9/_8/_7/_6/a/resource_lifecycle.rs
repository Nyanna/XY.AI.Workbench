use std::collections::HashMap;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::Arc;
use std::time::{Duration, Instant};
use log::{debug, error, warn};
use parking_lot::Mutex;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::b::task_model::{
    CompleteError, Task,
};
pub type ResourceKey = String;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MemoryClass {
    Ram,
    Vram,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResourceState {
    Uninitialized,
    Loading,
    Ready,
    Backoff,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LifecycleConfig {
    ram_ttl: Duration,
    vram_ttl: Duration,
    backoff: Duration,
}
impl LifecycleConfig {
    pub fn new(
        ram_ttl: Duration,
        vram_ttl: Duration,
        backoff: Duration,
    ) -> Result<Self, LifecycleError> {
        if vram_ttl <= ram_ttl {
            return Err(LifecycleError::InvalidConfig);
        }
        Ok(Self { ram_ttl, vram_ttl, backoff })
    }
    pub fn ram_ttl(&self) -> Duration {
        self.ram_ttl
    }
    pub fn vram_ttl(&self) -> Duration {
        self.vram_ttl
    }
    pub fn backoff(&self) -> Duration {
        self.backoff
    }
    fn ttl(&self, class: MemoryClass) -> Duration {
        match class {
            MemoryClass::Ram => self.ram_ttl,
            MemoryClass::Vram => self.vram_ttl,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LifecycleError {
    InvalidConfig,
    UnknownResource(ResourceKey),
    AlreadyRegistered(ResourceKey),
    /// Error text of a failed load/start.
    Failed(String),
}
/// Loads the resource. Runs synchronously inside `LoadTicket::run`.
pub trait ResourceLoader<H>: Send + Sync {
    fn load(&self) -> Result<H, String>;
}
struct Inner<H> {
    enabled: bool,
    state: ResourceState,
    handle: Option<Arc<H>>,
    backoff_until: Option<Instant>,
    last_used: Option<Instant>,
    in_use: usize,
    /// Incremented whenever a load starts or a failure is reported, so that a
    /// stale ticket cannot overwrite a newer state.
    epoch: u64,
}
struct Resource<H> {
    key: ResourceKey,
    class: MemoryClass,
    backoff: Duration,
    loader: Arc<dyn ResourceLoader<H>>,
    inner: Mutex<Inner<H>>,
}
impl<H> Resource<H> {
    fn release_use(&self, now: Option<Instant>) {
        let mut inner = self.inner.lock();
        inner.in_use = inner.in_use.saturating_sub(1);
        if let Some(now) = now {
            inner.last_used = Some(now);
        }
    }
}
pub struct ResourceRegistry<H> {
    config: LifecycleConfig,
    resources: Mutex<HashMap<ResourceKey, Arc<Resource<H>>>>,
}
fn deadline(now: Instant, wait: Duration) -> Instant {
    now.checked_add(wait).unwrap_or(now)
}
impl<H: Send + Sync + 'static> ResourceRegistry<H> {
    pub fn new(config: LifecycleConfig) -> Self {
        Self {
            config,
            resources: Mutex::new(HashMap::new()),
        }
    }
    fn find(&self, key: &str) -> Option<Arc<Resource<H>>> {
        self.resources.lock().get(key).cloned()
    }
    /// Registers an enabled resource in state Uninitialized (nothing is loaded).
    pub fn register(
        &self,
        key: ResourceKey,
        class: MemoryClass,
        loader: Arc<dyn ResourceLoader<H>>,
    ) -> Result<(), LifecycleError> {
        let mut resources = self.resources.lock();
        if resources.contains_key(&key) {
            return Err(LifecycleError::AlreadyRegistered(key));
        }
        let resource = Arc::new(Resource {
            key: key.clone(),
            class,
            backoff: self.config.backoff,
            loader,
            inner: Mutex::new(Inner {
                enabled: true,
                state: ResourceState::Uninitialized,
                handle: None,
                backoff_until: None,
                last_used: None,
                in_use: 0,
                epoch: 0,
            }),
        });
        resources.insert(key, resource);
        Ok(())
    }
    /// Enables or disables availability. Does not load or unload.
    pub fn set_enabled(&self, key: &str, enabled: bool) -> Result<(), LifecycleError> {
        let resource = self
            .find(key)
            .ok_or_else(|| LifecycleError::UnknownResource(key.to_string()))?;
        resource.inner.lock().enabled = enabled;
        Ok(())
    }
    pub fn is_available(&self, key: &str) -> bool {
        self.find(key).map(|resource| resource.inner.lock().enabled).unwrap_or(false)
    }
    pub fn is_loaded(&self, key: &str) -> bool {
        self.state(key) == Some(ResourceState::Ready)
    }
    pub fn state(&self, key: &str) -> Option<ResourceState> {
        self.find(key).map(|resource| resource.inner.lock().state)
    }
    /// Never blocks and never touches tasks. `task_id` is only used for logging.
    pub fn request_init(&self, key: &str, task_id: u64, now: Instant) -> InitRequest<H> {
        let Some(resource) = self.find(key) else {
            return InitRequest::Unavailable;
        };
        let mut inner = resource.inner.lock();
        if !inner.enabled {
            return InitRequest::Unavailable;
        }
        match inner.state {
            ResourceState::Ready => {
                let Some(handle) = inner.handle.clone() else {
                    error!(
                        "resource {key}: state Ready without handle (task {task_id})"
                    );
                    inner.state = ResourceState::Uninitialized;
                    return InitRequest::Unavailable;
                };
                inner.in_use += 1;
                inner.last_used = Some(now);
                drop(inner);
                InitRequest::Ready(Lease {
                    handle,
                    resource: Some(resource),
                })
            }
            ResourceState::Loading => InitRequest::Loading,
            ResourceState::Backoff => {
                let until = inner.backoff_until.unwrap_or(now);
                if now < until {
                    return InitRequest::Backoff { until };
                }
                let epoch = Self::begin_load(&mut inner);
                drop(inner);
                debug!("resource {key}: retry after backoff started by task {task_id}");
                InitRequest::Started(LoadTicket::new(resource, task_id, epoch))
            }
            ResourceState::Uninitialized => {
                let epoch = Self::begin_load(&mut inner);
                drop(inner);
                debug!("resource {key}: load started by task {task_id}");
                InitRequest::Started(LoadTicket::new(resource, task_id, epoch))
            }
        }
    }
    fn begin_load(inner: &mut Inner<H>) -> u64 {
        inner.state = ResourceState::Loading;
        inner.backoff_until = None;
        inner.epoch += 1;
        inner.epoch
    }
    /// Start error of an operation on a loaded resource: any state -> Backoff.
    /// Drops the registry's handle; existing leases stay valid.
    pub fn report_failure(
        &self,
        key: &str,
        task_id: u64,
        reason: &str,
        now: Instant,
    ) -> Result<(), LifecycleError> {
        let resource = self
            .find(key)
            .ok_or_else(|| LifecycleError::UnknownResource(key.to_string()))?;
        let dropped = {
            let mut inner = resource.inner.lock();
            inner.state = ResourceState::Backoff;
            inner.backoff_until = Some(deadline(now, self.config.backoff));
            inner.epoch += 1;
            inner.handle.take()
        };
        drop(dropped);
        error!("resource {key}: failure reported by task {task_id}: {reason}");
        Ok(())
    }
    /// End-of-iteration eviction. Returns the evicted keys.
    pub fn evict_expired(&self, now: Instant) -> Vec<ResourceKey> {
        let candidates: Vec<Arc<Resource<H>>> = self
            .resources
            .lock()
            .values()
            .cloned()
            .collect();
        let mut evicted = Vec::new();
        for resource in candidates {
            let ttl = self.config.ttl(resource.class);
            let dropped = {
                let mut inner = resource.inner.lock();
                let expired = inner.state == ResourceState::Ready && inner.in_use == 0
                    && inner
                        .last_used
                        .map(|last| now.saturating_duration_since(last) >= ttl)
                        .unwrap_or(false);
                if !expired {
                    continue;
                }
                inner.state = ResourceState::Uninitialized;
                inner.last_used = None;
                inner.handle.take()
            };
            drop(dropped);
            debug!("resource {}: evicted after ttl {:?}", resource.key, ttl);
            evicted.push(resource.key.clone());
        }
        evicted.sort();
        evicted
    }
}
pub enum InitRequest<H> {
    /// Resource is Ready. The lease marks it as in use.
    Ready(Lease<H>),
    /// This caller won the transition into Loading and must run the ticket.
    Started(LoadTicket<H>),
    /// Another caller is already loading.
    Loading,
    /// In Backoff; retry is allowed at `until`.
    Backoff { until: Instant },
    /// Not registered or disabled.
    Unavailable,
}
#[must_use]
pub struct LoadTicket<H> {
    resource: Arc<Resource<H>>,
    task_id: u64,
    epoch: u64,
    armed: bool,
}
impl<H> LoadTicket<H> {
    fn new(resource: Arc<Resource<H>>, task_id: u64, epoch: u64) -> Self {
        Self {
            resource,
            task_id,
            epoch,
            armed: true,
        }
    }
}
impl<H: Send + Sync + 'static> LoadTicket<H> {
    /// Runs the loader synchronously on the caller's thread. A loader panic is
    /// treated as a load error.
    pub fn run(mut self, now: Instant) -> Result<(), LifecycleError> {
        let resource = Arc::clone(&self.resource);
        let outcome = match catch_unwind(AssertUnwindSafe(|| resource.loader.load())) {
            Ok(result) => result,
            Err(payload) => {
                Err(
                    format!(
                        "loader panicked: {}", panic_payload_text(&* payload)
                        .unwrap_or("unknown panic")
                    ),
                )
            }
        };
        self.armed = false;
        let mut discarded: Option<H> = None;
        let result = {
            let mut inner = resource.inner.lock();
            if inner.state != ResourceState::Loading || inner.epoch != self.epoch {
                warn!(
                    "resource {}: load result of task {} discarded, state changed meanwhile",
                    resource.key, self.task_id
                );
                Err(
                    match outcome {
                        Ok(handle) => {
                            discarded = Some(handle);
                            LifecycleError::Failed(
                                "load superseded by a state change".to_string(),
                            )
                        }
                        Err(message) => LifecycleError::Failed(message),
                    },
                )
            } else {
                match outcome {
                    Ok(handle) => {
                        inner.handle = Some(Arc::new(handle));
                        inner.state = ResourceState::Ready;
                        inner.backoff_until = None;
                        inner.last_used = Some(now);
                        Ok(())
                    }
                    Err(message) => {
                        inner.state = ResourceState::Backoff;
                        inner.backoff_until = Some(deadline(now, resource.backoff));
                        Err(LifecycleError::Failed(message))
                    }
                }
            }
        };
        drop(discarded);
        match &result {
            Ok(()) => {
                debug!("resource {}: loaded by task {}", resource.key, self.task_id)
            }
            Err(LifecycleError::Failed(message)) => {
                error!(
                    "resource {}: load failed (task {}): {}", resource.key, self.task_id,
                    message
                )
            }
            Err(_) => {}
        }
        result
    }
}
impl<H> Drop for LoadTicket<H> {
    fn drop(&mut self) {
        if !self.armed {
            return;
        }
        let mut inner = self.resource.inner.lock();
        if inner.state == ResourceState::Loading && inner.epoch == self.epoch {
            inner.state = ResourceState::Uninitialized;
            debug!(
                "resource {}: ticket of task {} dropped without run", self.resource.key,
                self.task_id
            );
        }
    }
}
/// Marks the resource as in use. Dropping it ends the use.
pub struct Lease<H> {
    handle: Arc<H>,
    resource: Option<Arc<Resource<H>>>,
}
impl<H> Lease<H> {
    pub fn handle(&self) -> &Arc<H> {
        &self.handle
    }
    /// Ends the use and sets `last_used = now`.
    pub fn release(mut self, now: Instant) {
        if let Some(resource) = self.resource.take() {
            resource.release_use(Some(now));
        }
    }
}
impl<H> Drop for Lease<H> {
    fn drop(&mut self) {
        if let Some(resource) = self.resource.take() {
            resource.release_use(None);
        }
    }
}
/// Writes `Err(make_err(reason))` into every task of the batch. Returns the
/// tasks whose completion was rejected.
pub fn fail_batch<D, T, E>(
    batch: &[(u64, Task<D, T, E>)],
    reason: &str,
    make_err: impl Fn(&str) -> E,
) -> Vec<(u64, CompleteError)> {
    let mut rejected = Vec::new();
    for (task_id, task) in batch {
        error!("task {task_id} failed: {reason}");
        if let Err(error) = task.complete(Err(make_err(reason))) {
            warn!("task {task_id}: completing with error rejected: {error:?}");
            rejected.push((*task_id, error));
        }
    }
    rejected
}
#[cfg(test)]
#[path = "test/resource_lifecycle.rs"]
mod tests;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::_1::sync_util::panic_payload_text;
