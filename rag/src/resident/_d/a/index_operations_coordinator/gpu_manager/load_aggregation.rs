use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use super::scope_resolution::{
    load_candidates, CellSelector, ResolveContext, ResolveError, ScopeResolverRegistry,
};
use crate::resident::_d::_c::_b::_a::_9::_8::_7::a::reconcile::LoadedEntry;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::a::index_contract::{
    EntryId, Index,
};
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::b::task_model::{
    Task, TaskError, TaskParams,
};
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::_1::scopes_and_entry_sets::{
    EntrySet, Scope,
};
const MAX_PREALLOCATED_ENTRIES: usize = 1 << 16;
#[derive(Clone, Debug, PartialEq)]
pub struct LoadRequest<I> {
    pub index: I,
    pub scope: Scope,
}
/// Thread-safe collector of load requests.
pub struct Collector3<I> {
    requests: Mutex<Vec<LoadRequest<I>>>,
}
impl<I> Collector3<I> {
    pub fn new() -> Self {
        Self {
            requests: Mutex::new(Vec::new()),
        }
    }
    pub fn push(&self, req: LoadRequest<I>) {
        lock(&self.requests).push(req);
    }
    /// Number of collected requests, duplicates included.
    pub fn len(&self) -> usize {
        lock(&self.requests).len()
    }
    pub fn is_empty(&self) -> bool {
        lock(&self.requests).is_empty()
    }
    /// Atomically takes all requests in insertion order.
    pub fn drain(&self) -> Vec<LoadRequest<I>> {
        std::mem::take(&mut *lock(&self.requests))
    }
}
impl<I> Default for Collector3<I> {
    fn default() -> Self {
        Self::new()
    }
}
/// Loaded entries of one index, ascending by ID; `None` means missing in the index.
pub type Loaded3 = Vec<(EntryId, Option<LoadedEntry>)>;
#[derive(Debug, Clone)]
pub struct IndexLoad<I> {
    pub index: I,
    pub result: Result<Loaded3, ResolveError>,
}
/// Bundles requests per index, loads each index exactly once and the indexes concurrently.
pub fn load_aggregated<I>(
    requests: Vec<LoadRequest<I>>,
    registry: &ScopeResolverRegistry,
    lookup: &(dyn Fn(&I) -> Option<Arc<dyn Index>> + Sync),
    cell_selector: Option<&dyn CellSelector>,
) -> Vec<IndexLoad<I>>
where
    I: Clone + Eq + Hash + Send + Sync,
{
    let mut position: HashMap<I, usize> = HashMap::new();
    let mut groups: Vec<(I, Vec<Scope>)> = Vec::new();
    for LoadRequest { index, scope } in requests {
        match position.get(&index) {
            Some(&slot) => groups[slot].1.push(scope),
            None => {
                position.insert(index.clone(), groups.len());
                groups.push((index, vec![scope]));
            }
        }
    }
    if groups.is_empty() {
        return Vec::new();
    }
    std::thread::scope(|s| {
        let handles: Vec<_> = groups
            .iter()
            .map(|(index, scopes)| {
                s.spawn(move || IndexLoad {
                    index: index.clone(),
                    result: load_index(index, scopes, registry, lookup, cell_selector),
                })
            })
            .collect();
        handles
            .into_iter()
            .map(|handle| match handle.join() {
                Ok(load) => load,
                Err(payload) => std::panic::resume_unwind(payload),
            })
            .collect()
    })
}
fn load_index<I>(
    key: &I,
    scopes: &[Scope],
    registry: &ScopeResolverRegistry,
    lookup: &(dyn Fn(&I) -> Option<Arc<dyn Index>> + Sync),
    cell_selector: Option<&dyn CellSelector>,
) -> Result<Loaded3, ResolveError> {
    let requests = scopes.len();
    let outcome = resolve_union(key, scopes, registry, lookup, cell_selector);
    let (index, union) = match outcome {
        Ok(pair) => pair,
        Err(error) => {
            tracing::debug!(requests, error = ? error, "load_aggregated index failed");
            return Err(error);
        }
    };
    tracing::debug!(requests, union = union.len(), "load_aggregated index");
    if union.is_empty() {
        return Ok(Vec::new());
    }
    let capacity = usize::try_from(union.len())
        .unwrap_or(MAX_PREALLOCATED_ENTRIES)
        .min(MAX_PREALLOCATED_ENTRIES);
    let mut loaded: Loaded3 = Vec::with_capacity(capacity);
    load_candidates(index.as_ref(), &union, |id, entry| loaded.push((id, entry)));
    Ok(loaded)
}
fn resolve_union<I>(
    key: &I,
    scopes: &[Scope],
    registry: &ScopeResolverRegistry,
    lookup: &(dyn Fn(&I) -> Option<Arc<dyn Index>> + Sync),
    cell_selector: Option<&dyn CellSelector>,
) -> Result<(Arc<dyn Index>, EntrySet), ResolveError> {
    let index = lookup(key).ok_or(ResolveError::IndexUnavailable)?;
    let ctx = ResolveContext {
        index: Some(index.as_ref()),
        cell_selector,
    };
    let mut union = EntrySet::empty();
    for scope in scopes {
        let set = registry.resolve(scope, &ctx)?;
        union = union.union(&set);
    }
    Ok((index, union))
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlightRole {
    Starter,
    Joiner,
}
/// Handle to a running or finished flight. Clones refer to the same flight.
pub struct Flight3<V, E> {
    role: FlightRole,
    task: Task<(), V, E>,
}
impl<V, E> Clone for Flight3<V, E> {
    fn clone(&self) -> Self {
        Self {
            role: self.role,
            task: self.task.clone(),
        }
    }
}
impl<V, E> Flight3<V, E> {
    pub fn role(&self) -> FlightRole {
        self.role
    }
    pub fn task(&self) -> &Task<(), V, E> {
        &self.task
    }
    pub async fn wait(&self) -> Result<&V, TaskError<&E>> {
        self.task.wait().await
    }
}
struct FlightEntry<V, E> {
    id: u64,
    task: Task<(), V, E>,
}
struct Shared4<K, V, E> {
    next_id: AtomicU64,
    flights: Mutex<HashMap<K, FlightEntry<V, E>>>,
}
impl<K: Eq + Hash, V, E> Shared4<K, V, E> {
    fn remove_if_current(&self, key: &K, id: u64) {
        let mut flights = lock(&self.flights);
        if flights.get(key).is_some_and(|entry| entry.id == id) {
            flights.remove(key);
        }
    }
}
/// Handed to the starter; settles the flight exactly once.
pub struct FlightCompleter<K: Eq + Hash, V, E> {
    shared: Arc<Shared4<K, V, E>>,
    key: K,
    id: u64,
    task: Option<Task<(), V, E>>,
}
impl<K: Eq + Hash, V, E> FlightCompleter<K, V, E> {
    /// Removes the own map entry, then sets the result and wakes the waiters.
    pub fn finish(mut self, result: Result<V, E>) {
        let Some(task) = self.task.take() else {
            return;
        };
        self.shared.remove_if_current(&self.key, self.id);
        match task.claim() {
            Ok(()) => {
                if let Err(error) = task.complete(result) {
                    tracing::warn!(error = ? error, "single_flight complete rejected");
                }
            }
            Err(error) => {
                tracing::warn!(error = ? error, "single_flight claim rejected");
            }
        }
    }
}
impl<K: Eq + Hash, V, E> Drop for FlightCompleter<K, V, E> {
    fn drop(&mut self) {
        if let Some(task) = self.task.take() {
            self.shared.remove_if_current(&self.key, self.id);
            task.cancel();
        }
    }
}
/// Merges concurrent loads with equal keys into one flight. Cheap to clone.
pub struct SingleFlight<K, V, E> {
    shared: Arc<Shared4<K, V, E>>,
}
impl<K, V, E> Clone for SingleFlight<K, V, E> {
    fn clone(&self) -> Self {
        Self {
            shared: Arc::clone(&self.shared),
        }
    }
}
impl<K: Eq + Hash + Clone + Debug, V, E> SingleFlight<K, V, E> {
    pub fn new() -> Self {
        Self {
            shared: Arc::new(Shared4 {
                next_id: AtomicU64::new(0),
                flights: Mutex::new(HashMap::new()),
            }),
        }
    }
    /// Joins the running flight for `key`, or registers a new one and calls `start` once.
    /// `start` must not block on the load itself and may call `finish` later.
    pub fn join_or_start<S>(&self, key: K, start: S) -> Flight3<V, E>
    where
        S: FnOnce(FlightCompleter<K, V, E>),
    {
        let (flight, completer) = {
            let mut flights = lock(&self.shared.flights);
            if let Some(entry) = flights.get(&key) {
                let flight = Flight3 {
                    role: FlightRole::Joiner,
                    task: entry.task.clone(),
                };
                drop(flights);
                tracing::debug!(key = ? key, "single_flight joiner");
                return flight;
            }
            let id = self.shared.next_id.fetch_add(1, Ordering::Relaxed);
            let task = Task::new((), TaskParams::default());
            flights
                .insert(
                    key.clone(),
                    FlightEntry {
                        id,
                        task: task.clone(),
                    },
                );
            let completer = FlightCompleter {
                shared: Arc::clone(&self.shared),
                key,
                id,
                task: Some(task.clone()),
            };
            (
                Flight3 {
                    role: FlightRole::Starter,
                    task,
                },
                completer,
            )
        };
        start(completer);
        flight
    }
    /// Number of running flights (snapshot).
    pub fn in_flight(&self) -> usize {
        lock(&self.shared.flights).len()
    }
    pub fn is_in_flight(&self, key: &K) -> bool {
        lock(&self.shared.flights).contains_key(key)
    }
    pub fn keys_in_flight(&self) -> Vec<K> {
        lock(&self.shared.flights).keys().cloned().collect()
    }
}
impl<K: Eq + Hash + Clone + Debug, V, E> Default for SingleFlight<K, V, E> {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(test)]
#[path = "test/load_aggregation.rs"]
mod tests;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::_1::sync_util::lock;
