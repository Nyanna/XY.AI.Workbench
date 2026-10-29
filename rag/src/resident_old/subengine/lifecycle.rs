use std::future::Future;
use std::ops::Deref;
use std::sync::atomic::{AtomicU64, AtomicU8, AtomicUsize, Ordering};
use std::sync::Arc;
use parking_lot::RwLock;
use tokio::sync::{OnceCell, Semaphore};
use super::error::SubengineError;
use super::resource::{Resource, ResourceKey};
const UNLOADED: u8 = 0;
const LOADING: u8 = 1;
const RESIDENT: u8 = 2;
const EVICTING: u8 = 3;
const FAILED: u8 = 4;
/// Marks "no idle period measured yet"; set whenever the last lease is released.
const IDLE_UNSET: u64 = u64::MAX;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResourceState {
    Unloaded,
    Loading,
    Resident,
    Leased(usize),
    Idle,
    Evicting,
    Failed,
}
#[derive(Clone, Copy, Debug)]
pub struct CellConfig {
    pub ttl_ms: u64,
    pub backoff_ms: u64,
}
/// Bounds the number of resources initialising concurrently across all cells.
#[derive(Clone, Debug)]
pub struct InitLimiter(Arc<Semaphore>);
impl InitLimiter {
    pub fn new(max_parallel: usize) -> Self {
        Self(Arc::new(Semaphore::new(max_parallel.max(1))))
    }
    async fn acquire(&self) -> Result<tokio::sync::SemaphorePermit<'_>, SubengineError> {
        self.0.acquire().await.map_err(|_| SubengineError::CapacityExhausted)
    }
}
struct Shared {
    phase: AtomicU8,
    leases: AtomicUsize,
    in_flight: AtomicUsize,
    idle_since_ms: AtomicU64,
    backoff_until_ms: AtomicU64,
}
impl Shared {
    fn release_lease(&self) {
        if self.leases.fetch_sub(1, Ordering::SeqCst) == 1 {
            self.idle_since_ms.store(IDLE_UNSET, Ordering::SeqCst);
        }
    }
}
/// A lazily initialised, evictable resource.
///
/// Acquiring a lease takes the slot's read lock while bumping the lease count, and eviction
/// re-checks the count under the write lock, so a lease can never be handed out for a resource
/// that is being torn down.
pub struct ResourceCell<R> {
    key: ResourceKey,
    cfg: CellConfig,
    shared: Arc<Shared>,
    slot: RwLock<Arc<OnceCell<Arc<R>>>>,
}
impl<R: Resource> ResourceCell<R> {
    pub fn new(key: ResourceKey, cfg: CellConfig) -> Self {
        Self {
            key,
            cfg,
            shared: Arc::new(Shared {
                phase: AtomicU8::new(UNLOADED),
                leases: AtomicUsize::new(0),
                in_flight: AtomicUsize::new(0),
                idle_since_ms: AtomicU64::new(IDLE_UNSET),
                backoff_until_ms: AtomicU64::new(0),
            }),
            slot: RwLock::new(Arc::new(OnceCell::new())),
        }
    }
    pub fn key(&self) -> &ResourceKey {
        &self.key
    }
    pub fn state(&self) -> ResourceState {
        match self.shared.phase.load(Ordering::Acquire) {
            UNLOADED => ResourceState::Unloaded,
            LOADING => ResourceState::Loading,
            RESIDENT => {
                match self.shared.leases.load(Ordering::Acquire) {
                    0 => ResourceState::Idle,
                    n => ResourceState::Leased(n),
                }
            }
            EVICTING => ResourceState::Evicting,
            _ => ResourceState::Failed,
        }
    }
    /// Leases the resource, running `init` if it is not resident. Concurrent callers share one
    /// initialisation; a failure leaves the cell empty and rejects callers until the backoff ends.
    pub async fn get_or_init<F, Fut>(
        &self,
        limiter: &InitLimiter,
        now_ms: u64,
        init: F,
    ) -> Result<ResourceLease<R>, SubengineError>
    where
        F: FnOnce() -> Fut,
        Fut: Future<Output = Result<R, SubengineError>>,
    {
        let shared = &*self.shared;
        let cell = {
            let slot = self.slot.read();
            match shared.phase.load(Ordering::SeqCst) {
                EVICTING => {
                    return Err(SubengineError::ResourceEvicting {
                        resource: self.key.to_string(),
                    });
                }
                FAILED => {
                    let until_ms = shared.backoff_until_ms.load(Ordering::Acquire);
                    if now_ms < until_ms {
                        return Err(SubengineError::Backoff {
                            resource: self.key.to_string(),
                            until_ms,
                        });
                    }
                }
                _ => {}
            }
            shared.leases.fetch_add(1, Ordering::SeqCst);
            Arc::clone(&slot)
        };
        let backoff_ms = self.cfg.backoff_ms;
        let key = &self.key;
        let loaded = cell
            .get_or_try_init(|| async move {
                let _permit = limiter.acquire().await?;
                shared.phase.store(LOADING, Ordering::SeqCst);
                tracing::debug!(resource = % key, "loading");
                match init().await {
                    Ok(r) => {
                        shared.phase.store(RESIDENT, Ordering::SeqCst);
                        tracing::debug!(resource = % key, "resident");
                        Ok(Arc::new(r))
                    }
                    Err(e) => {
                        shared
                            .backoff_until_ms
                            .store(now_ms.saturating_add(backoff_ms), Ordering::Release);
                        shared.phase.store(FAILED, Ordering::SeqCst);
                        tracing::warn!(resource = % key, error = % e, "failed");
                        Err(e)
                    }
                }
            })
            .await;
        match loaded {
            Ok(value) => {
                tracing::trace!(resource = % self.key, "lease_acquired");
                Ok(ResourceLease {
                    value: Arc::clone(value),
                    shared: Arc::clone(&self.shared),
                })
            }
            Err(e) => {
                shared.release_lease();
                Err(e)
            }
        }
    }
    /// Unloads the resource when unleased, idle past the TTL and not vetoed by the resource.
    /// The first call after the last lease ends only starts the idle clock.
    pub fn evict_idle(&self, now_ms: u64) -> bool {
        let shared = &*self.shared;
        if shared.leases.load(Ordering::Acquire) != 0
            || shared.phase.load(Ordering::Acquire) != RESIDENT
        {
            return false;
        }
        let mut slot = self.slot.write();
        if shared.leases.load(Ordering::SeqCst) != 0
            || shared.in_flight.load(Ordering::SeqCst) != 0
            || shared.phase.load(Ordering::SeqCst) != RESIDENT
        {
            return false;
        }
        let since = shared.idle_since_ms.load(Ordering::SeqCst);
        if since == IDLE_UNSET {
            shared.idle_since_ms.store(now_ms, Ordering::SeqCst);
            return false;
        }
        if now_ms.saturating_sub(since) < self.cfg.ttl_ms {
            return false;
        }
        match slot.get() {
            Some(value) if value.evictable(now_ms) => {}
            _ => return false,
        }
        shared.phase.store(EVICTING, Ordering::SeqCst);
        let old = std::mem::replace(&mut *slot, Arc::new(OnceCell::new()));
        shared.idle_since_ms.store(IDLE_UNSET, Ordering::SeqCst);
        shared.phase.store(UNLOADED, Ordering::SeqCst);
        drop(slot);
        drop(old);
        tracing::debug!(resource = % self.key, "evicted");
        true
    }
}
pub struct ResourceLease<R> {
    value: Arc<R>,
    shared: Arc<Shared>,
}
impl<R> ResourceLease<R> {
    /// Marks an operation as running on the resource; eviction waits for the guard to drop.
    pub fn begin_op(&self) -> OpGuard {
        self.shared.in_flight.fetch_add(1, Ordering::SeqCst);
        OpGuard {
            shared: Arc::clone(&self.shared),
        }
    }
}
impl<R> Deref for ResourceLease<R> {
    type Target = R;
    fn deref(&self) -> &R {
        &self.value
    }
}
impl<R> Drop for ResourceLease<R> {
    fn drop(&mut self) {
        self.shared.release_lease();
    }
}
pub struct OpGuard {
    shared: Arc<Shared>,
}
impl Drop for OpGuard {
    fn drop(&mut self) {
        self.shared.in_flight.fetch_sub(1, Ordering::SeqCst);
    }
}
#[cfg(test)]
#[path = "test/lifecycle.rs"]
mod tests;
