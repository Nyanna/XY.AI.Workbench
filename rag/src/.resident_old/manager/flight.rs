//! Single-flight coalescing: concurrent requests for one key share one load and its outcome,
//! including its error.
use std::collections::HashMap;
use std::future::Future;
use std::hash::Hash;
use std::sync::Arc;
use futures::future::{BoxFuture, FutureExt, Shared};
use parking_lot::Mutex;
use crate::resident::error::ManagerError;
pub type FlightResult<T> = Result<Arc<T>, Arc<ManagerError>>;
type SharedLoad<T> = Shared<BoxFuture<'static, FlightResult<T>>>;
struct Flight<T> {
    id: u64,
    load: SharedLoad<T>,
    waiters: usize,
}
struct Inner<K, T> {
    next_id: u64,
    flights: HashMap<K, Flight<T>>,
}
pub struct SingleFlight<K, T> {
    inner: Mutex<Inner<K, T>>,
}
impl<K, T> Default for SingleFlight<K, T> {
    fn default() -> Self {
        Self {
            inner: Mutex::new(Inner {
                next_id: 0,
                flights: HashMap::new(),
            }),
        }
    }
}
impl<K, T> SingleFlight<K, T>
where
    K: Clone + Eq + Hash,
    T: Send + Sync + 'static,
{
    pub fn new() -> Self {
        Self::default()
    }
    pub fn inflight(&self) -> usize {
        self.inner.lock().flights.len()
    }
    /// Runs `start` only for the first caller of `key`; later callers while it is pending
    /// await the same outcome. `start` must only construct the future, never block. The entry
    /// is removed once the load has finished, so a failed load is retried by the next caller
    /// and never by the waiters themselves.
    pub async fn run<F, Fut>(&self, key: K, start: F) -> FlightResult<T>
    where
        F: FnOnce() -> Fut,
        Fut: Future<Output = FlightResult<T>> + Send + 'static,
    {
        let (id, load) = {
            let mut inner = self.inner.lock();
            if let Some(flight) = inner.flights.get_mut(&key) {
                flight.waiters += 1;
                tracing::debug!(waiters = flight.waiters, "load coalesced");
                (flight.id, flight.load.clone())
            } else {
                inner.next_id += 1;
                let id = inner.next_id;
                let load = start().boxed().shared();
                inner
                    .flights
                    .insert(
                        key.clone(),
                        Flight {
                            id,
                            load: load.clone(),
                            waiters: 1,
                        },
                    );
                (id, load)
            }
        };
        let result = load.await;
        let mut inner = self.inner.lock();
        if inner.flights.get(&key).is_some_and(|f| f.id == id) {
            inner.flights.remove(&key);
        }
        result
    }
}
#[cfg(test)]
#[path = "test/flight.rs"]
mod tests;
