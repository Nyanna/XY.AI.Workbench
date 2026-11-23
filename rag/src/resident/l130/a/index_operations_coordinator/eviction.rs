//! Cost-aware eviction of derived segment mirrors in the VRAM and RAM tiers.
use std::collections::HashMap;
use std::fmt;
use std::sync::{Arc, Mutex, MutexGuard, PoisonError};
use std::time::{Duration, Instant};
use log::{info, warn};
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060 as l060;
use l060::a::address_map::{AddressMap, MapError, MappingStore, SegmentId};
use l060::l050::a::table_swap::{Epoch, TableSwap, TableSwapError};
use l060::l050::l040::a::allocator::AllocatorSet;
use l060::l050::l040::l030::a::region_handle::RegionHandle;
use l060::l050::l040::l030::l020::a::arenas::{ArenaCapacities, ArenaKind};
use l060::l050::l040::l030::l020::l010::region_pinning::{Pin, PinCount};
const MAP_PUBLISH_ATTEMPTS: u32 = 3;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EvictKey(pub u64);
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Tier {
    Vram,
    Ram,
}
impl Tier {
    const ALL: [Tier; 2] = [Tier::Vram, Tier::Ram];
    fn index(self) -> usize {
        match self {
            Tier::Vram => 0,
            Tier::Ram => 1,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EvictReason {
    Idle,
    Pressure,
    SpaceNeed,
}
impl fmt::Display for EvictReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(
            match self {
                EvictReason::Idle => "idle",
                EvictReason::Pressure => "pressure",
                EvictReason::SpaceNeed => "space-need",
            },
        )
    }
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EvictionPolicy {
    pub high: f64,
    pub low: f64,
    pub idle: Duration,
}
impl EvictionPolicy {
    pub fn vram() -> Self {
        EvictionPolicy {
            high: 0.90,
            low: 0.75,
            idle: Duration::from_secs(600),
        }
    }
    pub fn ram() -> Self {
        EvictionPolicy {
            high: 0.85,
            low: 0.70,
            idle: Duration::from_secs(60),
        }
    }
    fn default_for(tier: Tier) -> Self {
        match tier {
            Tier::Vram => Self::vram(),
            Tier::Ram => Self::ram(),
        }
    }
    /// Replaces non-finite marks by the tier default, clamps `high` to `[0, 1]`
    /// and `low` to `[0, high]`.
    pub fn sanitized(self, tier: Tier) -> Self {
        let default = Self::default_for(tier);
        let high = if self.high.is_finite() { self.high } else { default.high }
            .clamp(0.0, 1.0);
        let low = if self.low.is_finite() { self.low } else { default.low }
            .clamp(0.0, high);
        EvictionPolicy {
            high,
            low,
            idle: self.idle,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct TierBudget {
    pub total: u64,
    pub reserve: u64,
    pub headroom: u64,
}
impl TierBudget {
    pub fn new(total: u64, reserve: u64, headroom: u64) -> Self {
        TierBudget {
            total,
            reserve,
            headroom,
        }
    }
    pub fn for_vram(caps: &ArenaCapacities, generations: u32, headroom: u64) -> Self {
        TierBudget::new(
            caps.get(ArenaKind::IndexSegments),
            caps.generation_reserve(generations),
            headroom,
        )
    }
    pub fn usable(&self) -> u64 {
        self.total.saturating_sub(self.reserve).saturating_sub(self.headroom)
    }
}
#[derive(Clone, Copy, Debug)]
pub struct TrackSpec {
    pub tier: Tier,
    pub segment: SegmentId,
    pub region: RegionHandle,
    pub size: u64,
    pub reload_cost: f64,
    pub summary: bool,
}
#[derive(Clone, Debug, PartialEq)]
pub struct EvictionDecision {
    pub index: String,
    pub victim: EvictKey,
    pub segment: SegmentId,
    /// Size as used in the priority: `max(size, 1)`.
    pub size: u64,
    /// Tracked bytes the entry occupied.
    pub bytes: u64,
    pub hits: u64,
    pub reason: EvictReason,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AbortCause {
    Table(TableSwapError),
    Map(MapError),
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Abort {
    pub key: EvictKey,
    pub cause: AbortCause,
}
#[derive(Clone, Debug, Default, PartialEq)]
pub struct EvictionOutcome {
    pub decisions: Vec<EvictionDecision>,
    pub skipped: Vec<EvictKey>,
    pub aborted: Option<Abort>,
}
impl EvictionOutcome {
    fn merge(&mut self, other: EvictionOutcome) {
        self.decisions.extend(other.decisions);
        self.skipped.extend(other.skipped);
        if self.aborted.is_none() {
            self.aborted = other.aborted;
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EvictionError {
    Exhausted { requested: u64, available: u64 },
}
impl fmt::Display for EvictionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EvictionError::Exhausted { .. } => f.write_str("Arena erschöpft"),
        }
    }
}
impl std::error::Error for EvictionError {}
struct Tracked {
    spec: TrackSpec,
    hits: u64,
    last_used: Instant,
    priority: f64,
    pins: PinCount,
}
impl Tracked {
    fn eligible(&self, tier: Tier) -> bool {
        self.spec.tier == tier && !self.spec.summary && self.pins.is_unpinned()
    }
    fn reprioritize(&mut self, floor: f64) {
        let size = self.spec.size.max(1) as f64;
        self.priority = floor + (self.hits as f64 * self.spec.reload_cost) / size;
    }
}
struct Deferred {
    region: RegionHandle,
    epoch: Epoch,
    bytes: u64,
    tier: Tier,
}
struct State {
    entries: HashMap<EvictKey, Tracked>,
    floors: [f64; 2],
    policies: [EvictionPolicy; 2],
    budgets: [TierBudget; 2],
    deferred: Vec<Deferred>,
}
impl State {
    fn live_bytes(&self, tier: Tier) -> u64 {
        self.entries
            .values()
            .filter(|t| t.spec.tier == tier)
            .fold(0u64, |acc, t| acc.saturating_add(t.spec.size))
    }
    fn deferred_bytes(&self, tier: Tier) -> u64 {
        self.deferred
            .iter()
            .filter(|d| d.tier == tier)
            .fold(0u64, |acc, d| acc.saturating_add(d.bytes))
    }
    fn occupancy(&self, tier: Tier) -> u64 {
        self.live_bytes(tier).saturating_add(self.deferred_bytes(tier))
    }
    fn usable(&self, tier: Tier) -> u64 {
        self.budgets[tier.index()].usable()
    }
    /// Eligible entries of `tier` as `(key, bytes)` in victim order.
    fn victims(&self, tier: Tier) -> Vec<(EvictKey, u64)> {
        let mut list: Vec<(&EvictKey, &Tracked)> = self
            .entries
            .iter()
            .filter(|(_, t)| t.eligible(tier))
            .collect();
        list.sort_by(|(ka, a), (kb, b)| {
            a.priority
                .total_cmp(&b.priority)
                .then(a.last_used.cmp(&b.last_used))
                .then(ka.cmp(kb))
        });
        list.into_iter().map(|(k, t)| (*k, t.spec.size)).collect()
    }
}
enum Step {
    Evicted(EvictionDecision),
    Skipped,
    Gone,
    Aborted(Abort),
}
fn lock<T>(m: &Mutex<T>) -> MutexGuard<'_, T> {
    m.lock().unwrap_or_else(PoisonError::into_inner)
}
/// Tracks mirrors and evicts them by idle time, pressure and space need.
pub struct Eviction {
    index: String,
    swap: Arc<TableSwap>,
    store: Arc<MappingStore>,
    allocators: Arc<AllocatorSet>,
    evict_lock: Mutex<()>,
    state: Mutex<State>,
}
static_assertions::assert_impl_all!(Eviction : Send, Sync);
impl Eviction {
    pub fn new(
        index: impl Into<String>,
        swap: Arc<TableSwap>,
        store: Arc<MappingStore>,
        allocators: Arc<AllocatorSet>,
        vram: TierBudget,
        ram: TierBudget,
    ) -> Eviction {
        Eviction {
            index: index.into(),
            swap,
            store,
            allocators,
            evict_lock: Mutex::new(()),
            state: Mutex::new(State {
                entries: HashMap::new(),
                floors: [0.0; 2],
                policies: [EvictionPolicy::vram(), EvictionPolicy::ram()],
                budgets: [vram, ram],
                deferred: Vec::new(),
            }),
        }
    }
    pub fn set_policy(&self, tier: Tier, policy: EvictionPolicy) {
        lock(&self.state).policies[tier.index()] = policy.sanitized(tier);
    }
    pub fn policy(&self, tier: Tier) -> EvictionPolicy {
        lock(&self.state).policies[tier.index()]
    }
    pub fn set_budget(&self, tier: Tier, budget: TierBudget) {
        lock(&self.state).budgets[tier.index()] = budget;
    }
    pub fn track(&self, key: EvictKey, spec: TrackSpec, now: Instant) {
        let mut spec = spec;
        if !spec.reload_cost.is_finite() || spec.reload_cost < 0.0 {
            spec.reload_cost = 0.0;
        }
        let mut st = lock(&self.state);
        let pins = st.entries.get(&key).map(|t| t.pins.clone()).unwrap_or_default();
        let mut tracked = Tracked {
            spec,
            hits: 1,
            last_used: now,
            priority: 0.0,
            pins,
        };
        tracked.reprioritize(st.floors[spec.tier.index()]);
        st.entries.insert(key, tracked);
    }
    pub fn touch(&self, key: EvictKey, now: Instant) -> bool {
        let mut st = lock(&self.state);
        let floors = st.floors;
        match st.entries.get_mut(&key) {
            Some(t) => {
                t.hits = t.hits.saturating_add(1);
                t.last_used = now;
                t.reprioritize(floors[t.spec.tier.index()]);
                true
            }
            None => false,
        }
    }
    /// Stops tracking only; no table change and no release.
    pub fn remove(&self, key: EvictKey) -> Option<u64> {
        lock(&self.state).entries.remove(&key).map(|t| t.spec.size)
    }
    pub fn pin(&self, key: EvictKey) -> Option<Pin<EvictKey>> {
        let st = lock(&self.state);
        st.entries.get(&key).map(|t| Pin::new(key, &t.pins))
    }
    pub fn is_pinned(&self, key: EvictKey) -> bool {
        lock(&self.state).entries.get(&key).is_some_and(|t| !t.pins.is_unpinned())
    }
    pub fn contains(&self, key: EvictKey) -> bool {
        lock(&self.state).entries.contains_key(&key)
    }
    pub fn len(&self) -> usize {
        lock(&self.state).entries.len()
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    pub fn len_tier(&self, tier: Tier) -> usize {
        lock(&self.state).entries.values().filter(|t| t.spec.tier == tier).count()
    }
    pub fn bytes(&self, tier: Tier) -> u64 {
        lock(&self.state).live_bytes(tier)
    }
    pub fn deferred_bytes(&self, tier: Tier) -> u64 {
        lock(&self.state).deferred_bytes(tier)
    }
    pub fn occupancy(&self, tier: Tier) -> u64 {
        lock(&self.state).occupancy(tier)
    }
    pub fn usable(&self, tier: Tier) -> u64 {
        lock(&self.state).usable(tier)
    }
    pub fn priority(&self, key: EvictKey) -> Option<f64> {
        lock(&self.state).entries.get(&key).map(|t| t.priority)
    }
    pub fn hits(&self, key: EvictKey) -> Option<u64> {
        lock(&self.state).entries.get(&key).map(|t| t.hits)
    }
    pub fn floor(&self, tier: Tier) -> f64 {
        lock(&self.state).floors[tier.index()]
    }
    pub fn evict_idle(&self, tier: Tier, now: Instant) -> EvictionOutcome {
        let _serial = lock(&self.evict_lock);
        self.idle_locked(tier, now)
    }
    pub fn relieve_pressure(&self, tier: Tier) -> EvictionOutcome {
        let _serial = lock(&self.evict_lock);
        self.pressure_locked(tier)
    }
    pub fn tick(&self, now: Instant) -> EvictionOutcome {
        let _serial = lock(&self.evict_lock);
        let mut total = EvictionOutcome::default();
        for tier in Tier::ALL {
            total.merge(self.idle_locked(tier, now));
            if total.aborted.is_some() {
                break;
            }
            total.merge(self.pressure_locked(tier));
            if total.aborted.is_some() {
                break;
            }
        }
        total
    }
    pub fn make_room(
        &self,
        tier: Tier,
        need: u64,
    ) -> Result<EvictionOutcome, EvictionError> {
        let _serial = lock(&self.evict_lock);
        let plan = {
            let st = lock(&self.state);
            let free = st.usable(tier).saturating_sub(st.occupancy(tier));
            if need <= free {
                return Ok(EvictionOutcome::default());
            }
            let shortfall = need - free;
            let mut gathered = 0u64;
            let mut keys = Vec::new();
            for (key, bytes) in st.victims(tier) {
                if gathered >= shortfall {
                    break;
                }
                keys.push(key);
                gathered = gathered.saturating_add(bytes);
            }
            if gathered < shortfall {
                return Err(EvictionError::Exhausted {
                    requested: need,
                    available: free.saturating_add(gathered),
                });
            }
            keys
        };
        Ok(self.run(tier, plan, EvictReason::SpaceNeed))
    }
    /// Releases deferred regions with `epoch <= min_reader_epoch`; returns the
    /// number of regions freed. Entries whose release fails are dropped too.
    pub fn reclaim(&self, min_reader_epoch: Epoch) -> usize {
        let ready = {
            let mut st = lock(&self.state);
            let (ready, kept): (Vec<Deferred>, Vec<Deferred>) = std::mem::take(
                    &mut st.deferred,
                )
                .into_iter()
                .partition(|d| d.epoch <= min_reader_epoch);
            st.deferred = kept;
            ready
        };
        let mut freed = 0;
        for d in ready {
            let result = self
                .allocators
                .allocator_for(&d.region)
                .and_then(|a| a.free(&d.region));
            match result {
                Ok(()) => freed += 1,
                Err(e) => {
                    warn!(
                        "eviction index={} region offset={} not freed: {e}", self.index,
                        d.region.offset()
                    )
                }
            }
        }
        freed
    }
    fn idle_locked(&self, tier: Tier, now: Instant) -> EvictionOutcome {
        let keys = {
            let st = lock(&self.state);
            let idle = st.policies[tier.index()].idle;
            let mut keys: Vec<EvictKey> = st
                .entries
                .iter()
                .filter(|(_, t)| {
                    t.eligible(tier)
                        && now.saturating_duration_since(t.last_used) >= idle
                })
                .map(|(k, _)| *k)
                .collect();
            keys.sort();
            keys
        };
        self.run(tier, keys, EvictReason::Idle)
    }
    fn pressure_locked(&self, tier: Tier) -> EvictionOutcome {
        let keys = {
            let st = lock(&self.state);
            let policy = st.policies[tier.index()];
            let usable = st.usable(tier) as f64;
            let occupancy = st.occupancy(tier);
            if occupancy as f64 <= policy.high * usable {
                return EvictionOutcome::default();
            }
            let target = policy.low * usable;
            let mut projected = occupancy;
            let mut keys = Vec::new();
            for (key, bytes) in st.victims(tier) {
                if projected as f64 <= target {
                    break;
                }
                keys.push(key);
                projected = projected.saturating_sub(bytes);
            }
            keys
        };
        self.run(tier, keys, EvictReason::Pressure)
    }
    fn run(
        &self,
        tier: Tier,
        keys: Vec<EvictKey>,
        reason: EvictReason,
    ) -> EvictionOutcome {
        let mut out = EvictionOutcome::default();
        for key in keys {
            match self.evict_one(key, tier, reason) {
                Step::Evicted(decision) => out.decisions.push(decision),
                Step::Skipped => out.skipped.push(key),
                Step::Gone => {}
                Step::Aborted(abort) => {
                    out.aborted = Some(abort);
                    break;
                }
            }
        }
        out
    }
    fn evict_one(&self, key: EvictKey, tier: Tier, reason: EvictReason) -> Step {
        let taken = {
            let mut st = lock(&self.state);
            if st.entries.get(&key).is_some_and(|t| t.eligible(tier)) {
                st.entries.remove(&key)
            } else {
                None
            }
        };
        let Some(tracked) = taken else {
            return Step::Gone;
        };
        let spec = tracked.spec;
        let mut table = self.swap.next_table();
        let mut map = match self.store.next() {
            Ok(map) => map,
            Err(e) => return self.restore(key, tracked, AbortCause::Map(e)),
        };
        let in_table = table.remove(&spec.region);
        let in_map = map.remove(spec.segment);
        if !in_table && !in_map {
            warn!(
                "eviction index={} victim={} segment={} skipped: region unknown", self
                .index, key.0, spec.segment.0
            );
            return Step::Skipped;
        }
        let epoch = if in_table {
            match self.swap.publish(table) {
                Ok(epoch) => epoch,
                Err(e) => return self.restore(key, tracked, AbortCause::Table(e)),
            }
        } else {
            self.swap.published_epoch()
        };
        if in_map {
            self.publish_map(map, spec.segment);
        }
        {
            let mut st = lock(&self.state);
            st.deferred
                .push(Deferred {
                    region: spec.region,
                    epoch,
                    bytes: spec.size,
                    tier,
                });
            if reason != EvictReason::Idle {
                let floor = &mut st.floors[tier.index()];
                *floor = floor.max(tracked.priority);
            }
        }
        let decision = EvictionDecision {
            index: self.index.clone(),
            victim: key,
            segment: spec.segment,
            size: spec.size.max(1),
            bytes: spec.size,
            hits: tracked.hits,
            reason,
        };
        info!(
            "eviction index={} victim={} segment={} size={} bytes={} hits={} reason={}",
            decision.index, key.0, decision.segment.0, decision.size, decision.bytes,
            decision.hits, reason
        );
        Step::Evicted(decision)
    }
    fn publish_map(&self, first: AddressMap, segment: SegmentId) {
        let mut map = first;
        for attempt in 1..=MAP_PUBLISH_ATTEMPTS {
            match self.store.publish(map) {
                Ok(_) => return,
                Err(MapError::StaleMapping { .. }) if attempt < MAP_PUBLISH_ATTEMPTS => {
                    match self.store.next() {
                        Ok(mut next) => {
                            if !next.remove(segment) {
                                return;
                            }
                            map = next;
                        }
                        Err(e) => {
                            warn!(
                                "eviction index={} segment={} map not republished: {e}",
                                self.index, segment.0
                            );
                            return;
                        }
                    }
                }
                Err(e) => {
                    warn!(
                        "eviction index={} segment={} map not published: {e}", self
                        .index, segment.0
                    );
                    return;
                }
            }
        }
    }
    fn restore(&self, key: EvictKey, tracked: Tracked, cause: AbortCause) -> Step {
        lock(&self.state).entries.entry(key).or_insert(tracked);
        Step::Aborted(Abort { key, cause })
    }
}
#[cfg(test)]
#[path = "test/eviction.rs"]
mod tests;
