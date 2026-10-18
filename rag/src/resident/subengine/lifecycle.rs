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
mod tests {
    use super::*;
    use std::sync::atomic::AtomicUsize;
    struct Dummy(u32);
    impl Resource for Dummy {
        fn key(&self) -> ResourceKey {
            ResourceKey::new(format!("dummy-{}", self.0))
        }
    }
    fn cell(ttl_ms: u64, backoff_ms: u64) -> ResourceCell<Dummy> {
        ResourceCell::new(ResourceKey::new("dummy"), CellConfig { ttl_ms, backoff_ms })
    }
    fn init_err() -> SubengineError {
        SubengineError::ResourceInit {
            resource: "dummy".into(),
            reason: "boom".into(),
        }
    }
    #[tokio::test]
    async fn parallel_callers_share_one_initialiser() {
        let c = cell(10, 10);
        let limiter = InitLimiter::new(4);
        let inits = AtomicUsize::new(0);
        let call = || {
            c.get_or_init(
                &limiter,
                0,
                || async {
                    inits.fetch_add(1, Ordering::SeqCst);
                    tokio::task::yield_now().await;
                    Ok(Dummy(1))
                },
            )
        };
        let leases = futures_join(vec![call(), call(), call(), call()]).await;
        assert_eq!(inits.load(Ordering::SeqCst), 1);
        assert!(leases.iter().all(| l | l.as_ref().is_ok_and(| l | l.0 == 1)));
        assert_eq!(c.state(), ResourceState::Leased(4));
        drop(leases);
        assert_eq!(c.state(), ResourceState::Idle);
    }
    async fn futures_join<F: Future>(fs: Vec<F>) -> Vec<F::Output> {
        let mut pending: Vec<_> = fs.into_iter().map(|f| Some(Box::pin(f))).collect();
        let mut results: Vec<Option<F::Output>> = (0..pending.len())
            .map(|_| None)
            .collect();
        std::future::poll_fn(|cx| {
                let mut done = true;
                for (slot, result) in pending.iter_mut().zip(results.iter_mut()) {
                    if let Some(f) = slot {
                        match f.as_mut().poll(cx) {
                            std::task::Poll::Ready(v) => {
                                *result = Some(v);
                                *slot = None;
                            }
                            std::task::Poll::Pending => done = false,
                        }
                    }
                }
                if done { std::task::Poll::Ready(()) } else { std::task::Poll::Pending }
            })
            .await;
        results.into_iter().flatten().collect()
    }
    #[tokio::test]
    async fn failed_init_leaves_cell_empty_and_backs_off() {
        let c = cell(10, 100);
        let limiter = InitLimiter::new(1);
        let err = c
            .get_or_init(&limiter, 0, || async { Err(init_err()) })
            .await
            .err()
            .unwrap();
        assert!(matches!(err, SubengineError::ResourceInit { .. }));
        assert_eq!(c.state(), ResourceState::Failed);
        let err = c
            .get_or_init(&limiter, 50, || async { Ok(Dummy(1)) })
            .await
            .err()
            .unwrap();
        assert!(matches!(err, SubengineError::Backoff { until_ms : 100, .. }));
        let lease = c
            .get_or_init(&limiter, 100, || async { Ok(Dummy(2)) })
            .await
            .unwrap();
        assert_eq!(lease.0, 2);
        assert_eq!(c.state(), ResourceState::Leased(1));
    }
    #[tokio::test]
    async fn limiter_bounds_parallel_inits_across_cells() {
        let limiter = InitLimiter::new(1);
        let running = AtomicUsize::new(0);
        let peak = AtomicUsize::new(0);
        let cells: Vec<_> = (0..3).map(|_| cell(10, 10)).collect();
        let calls = cells
            .iter()
            .map(|c| {
                c.get_or_init(
                    &limiter,
                    0,
                    || async {
                        let now = running.fetch_add(1, Ordering::SeqCst) + 1;
                        peak.fetch_max(now, Ordering::SeqCst);
                        tokio::task::yield_now().await;
                        running.fetch_sub(1, Ordering::SeqCst);
                        Ok(Dummy(0))
                    },
                )
            });
        let leases = futures_join(calls.collect()).await;
        assert!(leases.iter().all(Result::is_ok));
        assert_eq!(peak.load(Ordering::SeqCst), 1);
    }
    #[tokio::test]
    async fn eviction_waits_for_lease_op_and_ttl() {
        let c = cell(100, 10);
        let limiter = InitLimiter::new(1);
        let lease = c.get_or_init(&limiter, 0, || async { Ok(Dummy(1)) }).await.unwrap();
        assert!(! c.evict_idle(1_000));
        let op = lease.begin_op();
        drop(lease);
        assert!(! c.evict_idle(1_000));
        drop(op);
        assert!(! c.evict_idle(1_000), "first call only starts the idle clock");
        assert!(! c.evict_idle(1_050));
        assert!(c.evict_idle(1_100));
        assert_eq!(c.state(), ResourceState::Unloaded);
        let lease = c
            .get_or_init(&limiter, 1_200, || async { Ok(Dummy(2)) })
            .await
            .unwrap();
        assert_eq!(lease.0, 2);
    }
    #[tokio::test]
    async fn lease_taken_after_idle_clock_blocks_eviction() {
        let c = cell(10, 10);
        let limiter = InitLimiter::new(1);
        drop(c.get_or_init(&limiter, 0, || async { Ok(Dummy(1)) }).await.unwrap());
        assert!(! c.evict_idle(0));
        let lease = c.get_or_init(&limiter, 5, || async { Ok(Dummy(9)) }).await.unwrap();
        assert_eq!(lease.0, 1);
        assert!(! c.evict_idle(1_000));
        drop(lease);
        assert!(! c.evict_idle(1_000));
        assert!(c.evict_idle(1_010));
    }
    #[test]
    fn concurrent_leases_and_eviction_never_lose_a_resident_lease() {
        let c = Arc::new(cell(0, 1));
        let limiter = InitLimiter::new(1);
        let rt = tokio::runtime::Builder::new_multi_thread()
            .worker_threads(2)
            .build()
            .unwrap();
        let evictor = {
            let c = Arc::clone(&c);
            std::thread::spawn(move || {
                for t in 0..5_000 {
                    c.evict_idle(t);
                }
            })
        };
        rt.block_on(async {
            for _ in 0..2_000 {
                let lease = c
                    .get_or_init(&limiter, 0, || async { Ok(Dummy(7)) })
                    .await
                    .unwrap();
                assert_eq!(lease.0, 7);
                tokio::task::yield_now().await;
                assert_eq!(lease.0, 7);
            }
        });
        evictor.join().unwrap();
    }
}
