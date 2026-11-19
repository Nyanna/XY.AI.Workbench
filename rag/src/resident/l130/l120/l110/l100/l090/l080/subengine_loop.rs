//! One synchronous iteration of a sub-engine: survey, lazy init, claim, hand-over, eviction.

use std::collections::HashSet;
use std::sync::Arc;
use std::time::Instant;

use log::debug;

use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::a::resource_lifecycle::{
    InitRequest, Lease, LoadTicket, ResourceKey, ResourceRegistry,
};
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::a::resource_requirements::{
    HasResourceRequirements, ResourceKind, ResourceRequirements,
};
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::l010::queue::{
    Claim, Queue,
};

/// Resource handle that executes nodes.
pub trait ResourceExec: Sized + Send + Sync + 'static {
    /// The claimed queue entry, normally `queue::Claim<M, P, (), ()>`.
    type Entry: Send + 'static;

    /// Non-blocking. True iff a further entry could be accepted right now.
    fn has_capacity(&self) -> bool;

    /// Takes over the claim and the lease and holds both until the work is finished.
    /// CPU resources run synchronously on the calling thread; GPU resources only launch.
    fn execute(&self, entry: Self::Entry, lease: Lease<Self>);
}

/// Pull filter on the node metadata. `false` skips the node.
pub type PullFilter<M> = Arc<dyn Fn(&M) -> bool + Send + Sync>;

/// Determines the resource entry from the requirements. `None` skips the node.
pub type ResourceResolver = Arc<dyn Fn(&ResourceRequirements) -> Option<ResourceKey> + Send + Sync>;

/// Receives the ticket when this iteration won the lazy init. It must run the ticket
/// without blocking the loop. Dropping the ticket unrun returns the resource to
/// Uninitialized.
pub type InitStarter<H> = Arc<dyn Fn(LoadTicket<H>) + Send + Sync>;

/// A resource of this sub-engine. All slots together make up the "all busy" set.
#[derive(Debug, Clone)]
pub struct ResourceSlot {
    pub key: ResourceKey,
    pub requirements: ResourceRequirements,
}

/// Idle-notification occasion: a free resource and no hit.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IdleNotice {
    pub key: ResourceKey,
    pub kind: ResourceKind,
    pub requirements: ResourceRequirements,
}

#[derive(Debug, Default)]
pub struct IterationOutcome {
    pub claimed: usize,
    pub inits_started: usize,
    pub idle: Vec<IdleNotice>,
    pub evicted: Vec<ResourceKey>,
}

impl IterationOutcome {
    pub fn progress(&self) -> bool {
        self.claimed > 0
    }
}

#[derive(Default)]
struct IterationState {
    claimed: usize,
    inits_started: usize,
    busy: HashSet<ResourceKey>,
    free_observed: HashSet<ResourceKey>,
}

pub struct SubengineLoop<M, P, H>
where
    M: HasResourceRequirements + Send + Sync + 'static,
    P: Send + 'static,
    H: ResourceExec<Entry = Claim<M, P, (), ()>>,
{
    own: Arc<Queue<M, P, (), ()>>,
    shared: Option<Arc<Queue<M, P, (), ()>>>,
    registry: Arc<ResourceRegistry<H>>,
    slots: Vec<ResourceSlot>,
    resolver: ResourceResolver,
    filter: PullFilter<M>,
    init: InitStarter<H>,
}

impl<M, P, H> SubengineLoop<M, P, H>
where
    M: HasResourceRequirements + Send + Sync + 'static,
    P: Send + 'static,
    H: ResourceExec<Entry = Claim<M, P, (), ()>>,
{
    pub fn new(
        own: Arc<Queue<M, P, (), ()>>,
        shared: Option<Arc<Queue<M, P, (), ()>>>,
        registry: Arc<ResourceRegistry<H>>,
        slots: Vec<ResourceSlot>,
        resolver: ResourceResolver,
        filter: PullFilter<M>,
        init: InitStarter<H>,
    ) -> Self {
        Self { own, shared, registry, slots, resolver, filter, init }
    }

    /// One iteration. Never blocks, never panics by itself.
    pub fn run_iteration(&self, now: Instant) -> IterationOutcome {
        let mut state = IterationState::default();
        for queue in std::iter::once(&self.own).chain(self.shared.as_ref()) {
            if self.all_busy(&state) {
                break;
            }
            self.process_queue(queue, now, &mut state);
        }

        let evicted = self.registry.evict_expired(now);
        let idle = if state.claimed == 0 {
            self.slots
                .iter()
                .filter(|slot| {
                    state.free_observed.contains(&slot.key) && !state.busy.contains(&slot.key)
                })
                .map(|slot| IdleNotice {
                    key: slot.key.clone(),
                    kind: slot.requirements.kind(),
                    requirements: slot.requirements.clone(),
                })
                .collect()
        } else {
            Vec::new()
        };
        IterationOutcome {
            claimed: state.claimed,
            inits_started: state.inits_started,
            idle,
            evicted,
        }
    }

    fn all_busy(&self, state: &IterationState) -> bool {
        !self.slots.is_empty() && self.slots.iter().all(|slot| state.busy.contains(&slot.key))
    }

    fn matches(&self, meta: &M, key: &str) -> bool {
        (self.filter)(meta)
            && (self.resolver)(meta.resource_requirements()).as_deref() == Some(key)
    }

    fn survey(&self, queue: &Queue<M, P, (), ()>) -> Vec<ResourceKey> {
        let mut seen: HashSet<ResourceKey> = HashSet::new();
        let mut keys: Vec<ResourceKey> = Vec::new();
        let none = queue.take_where(|meta| {
            if (self.filter)(meta) {
                if let Some(key) = (self.resolver)(meta.resource_requirements()) {
                    if seen.insert(key.clone()) {
                        keys.push(key);
                    }
                }
            }
            false
        });
        drop(none);
        keys
    }

    fn process_queue(&self, queue: &Queue<M, P, (), ()>, now: Instant, state: &mut IterationState) {
        for key in self.survey(queue) {
            if state.busy.contains(&key) {
                continue;
            }
            self.serve_key(queue, &key, now, state);
            if self.all_busy(state) {
                return;
            }
        }
    }

    fn serve_key(
        &self,
        queue: &Queue<M, P, (), ()>,
        key: &ResourceKey,
        now: Instant,
        state: &mut IterationState,
    ) {
        loop {
            match self.registry.request_init(key, 0, now) {
                InitRequest::Ready(lease) => {
                    let handle = Arc::clone(lease.handle());
                    if !handle.has_capacity() {
                        debug!("resource {key}: no capacity, marked busy");
                        state.busy.insert(key.clone());
                        return;
                    }
                    state.free_observed.insert(key.clone());
                    match queue.take_where(|meta| self.matches(meta, key)) {
                        Some(claim) => {
                            handle.execute(claim, lease);
                            state.claimed += 1;
                        }
                        None => {
                            debug!("resource {key}: no claimable entry");
                            return;
                        }
                    }
                }
                InitRequest::Started(ticket) => {
                    debug!("resource {key}: lazy init started");
                    (self.init)(ticket);
                    state.inits_started += 1;
                    return;
                }
                InitRequest::Loading => {
                    debug!("resource {key}: loading, skipped");
                    return;
                }
                InitRequest::Backoff { until } => {
                    debug!("resource {key}: in backoff until {until:?}, skipped");
                    return;
                }
                InitRequest::Unavailable => {
                    debug!("resource {key}: unavailable, skipped");
                    return;
                }
            }
        }
    }
}

#[cfg(test)]
#[path = "test/subengine_loop.rs"]
mod tests;
