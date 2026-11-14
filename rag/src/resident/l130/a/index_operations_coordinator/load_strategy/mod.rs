use std::collections::{BTreeMap, HashMap, HashSet};
use std::hash::Hash;
use std::sync::{Arc, Mutex, MutexGuard};

use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::{
    a::{
        arenas::{Budget, MIB},
        index_contract::{Index, IndexError, NodeId, Summary},
        pareto_profiles::PrecisionProfile,
    },
    l010::{capability_model::axes, scopes_and_entry_sets::EntrySet},
};

pub mod profile_selection;

pub const DEFAULT_DENSITY: f64 = 0.12;
pub const DEFAULT_SEGMENT_BYTES: u64 = 64 * MIB;
pub const DEFAULT_DIRECT_MAP_MAX_BYTES: u64 = 64 * 1024;

const MIN_SEGMENT_BYTES: u64 = 4 * MIB;
const MAX_SEGMENT_BYTES: u64 = 256 * MIB;
const SUMMARY_OVERHEAD_BYTES: u64 = 24;

/// Replaceable layout of entries within segments.
pub trait Geometry: Send + Sync {
    fn segment_of(&self, entry: u64) -> u64;
    fn entry_size(&self, entry: u64) -> u64;
    fn segment_size(&self, segment: u64) -> u64;
}

/// Equally sized entries in equally sized segments.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UniformGeometry {
    entry_bytes: u64,
    entries_per_segment: u64,
}

impl UniformGeometry {
    pub fn new(entry_bytes: u64, entries_per_segment: u64) -> Self {
        Self {
            entry_bytes,
            entries_per_segment: entries_per_segment.max(1),
        }
    }

    pub fn for_policy(entry_bytes: u64, policy: &LoadPolicy) -> Self {
        let capacity = if entry_bytes == 0 {
            1
        } else {
            (policy.segment_bytes() / entry_bytes).max(1)
        };
        Self::new(entry_bytes, capacity)
    }
}

impl Geometry for UniformGeometry {
    fn segment_of(&self, entry: u64) -> u64 {
        entry / self.entries_per_segment
    }

    fn entry_size(&self, _entry: u64) -> u64 {
        self.entry_bytes
    }

    fn segment_size(&self, _segment: u64) -> u64 {
        self.entries_per_segment.saturating_mul(self.entry_bytes)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Access {
    Regular,
    /// Small, rarely read set served from mapped host memory.
    Rare,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LoadPolicy {
    density_threshold: f64,
    segment_bytes: u64,
    direct_map_max_bytes: u64,
}

impl Default for LoadPolicy {
    fn default() -> Self {
        Self {
            density_threshold: DEFAULT_DENSITY,
            segment_bytes: DEFAULT_SEGMENT_BYTES,
            direct_map_max_bytes: DEFAULT_DIRECT_MAP_MAX_BYTES,
        }
    }
}

impl LoadPolicy {
    pub fn density_threshold(&self) -> f64 {
        self.density_threshold
    }

    pub fn segment_bytes(&self) -> u64 {
        self.segment_bytes
    }

    pub fn direct_map_max_bytes(&self) -> u64 {
        self.direct_map_max_bytes
    }

    /// Non-finite values are ignored; finite values are clamped to [0, 1].
    pub fn with_density(mut self, d: f64) -> Self {
        if d.is_finite() {
            self.density_threshold = d.clamp(0.0, 1.0);
        }
        self
    }

    pub fn with_segment_bytes(mut self, b: u64) -> Self {
        self.segment_bytes = b;
        self
    }

    pub fn with_direct_map_max_bytes(mut self, b: u64) -> Self {
        self.direct_map_max_bytes = b;
        self
    }

    pub fn with_budget(self, budget: &Budget) -> Self {
        self.with_segment_bytes(segment_bytes_for_budget(budget.usable()))
    }

    /// Takes the density from the `density` axis (whole permille, decimal).
    pub fn apply_profile(&mut self, profile: &PrecisionProfile) -> bool {
        let permille = profile
            .config
            .get(&axes::density())
            .and_then(|v| v.as_str().parse::<u64>().ok());
        match permille {
            Some(p) => {
                *self = self.with_density(p as f64 / 1000.0);
                true
            }
            None => false,
        }
    }
}

/// `available / 8`, clamped to [4 MiB, 256 MiB], rounded down to whole MiB.
pub fn segment_bytes_for_budget(available: u64) -> u64 {
    let clamped = (available / 8).clamp(MIN_SEGMENT_BYTES, MAX_SEGMENT_BYTES);
    clamped / MIB * MIB
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LoadPlan {
    /// Whole segments, ascending and duplicate-free; `bytes` is the sum of segment sizes.
    Segments { segments: Vec<u64>, bytes: u64 },
    /// Individual entries, ascending, sent together; `bytes` is the sum of entry sizes.
    Gather { entries: Vec<u64>, bytes: u64 },
    /// Direct mapping without a copy; `bytes` is the sum of entry sizes.
    DirectMap { entries: EntrySet, bytes: u64 },
}

impl LoadPlan {
    pub fn bytes(&self) -> u64 {
        match self {
            LoadPlan::Segments { bytes, .. }
            | LoadPlan::Gather { bytes, .. }
            | LoadPlan::DirectMap { bytes, .. } => *bytes,
        }
    }

    fn kind(&self) -> &'static str {
        match self {
            LoadPlan::Segments { .. } => "segments",
            LoadPlan::Gather { .. } => "gather",
            LoadPlan::DirectMap { .. } => "direct-map",
        }
    }
}

struct SegmentGroup {
    requested: u64,
    entries: Vec<u64>,
}

/// Pure planning: the segment plan comes first, then the gather plan.
pub fn plan_loads(
    set: &EntrySet,
    access: Access,
    geometry: &dyn Geometry,
    policy: &LoadPolicy,
) -> Vec<LoadPlan> {
    if set.is_empty() {
        return Vec::new();
    }

    if access == Access::Rare {
        let total = set
            .iter()
            .fold(0u64, |acc, e| acc.saturating_add(geometry.entry_size(e)));
        if total <= policy.direct_map_max_bytes() {
            let plan = LoadPlan::DirectMap {
                entries: set.clone(),
                bytes: total,
            };
            log::debug!("load plan: kind={} bytes={}", plan.kind(), total);
            return vec![plan];
        }
    }

    let mut groups: BTreeMap<u64, SegmentGroup> = BTreeMap::new();
    for entry in set.iter() {
        let group = groups
            .entry(geometry.segment_of(entry))
            .or_insert_with(|| SegmentGroup {
                requested: 0,
                entries: Vec::new(),
            });
        group.requested = group.requested.saturating_add(geometry.entry_size(entry));
        group.entries.push(entry);
    }

    let mut segments = Vec::new();
    let mut segment_bytes = 0u64;
    let mut gather = Vec::new();
    let mut gather_bytes = 0u64;

    for (segment, group) in groups {
        let size = geometry.segment_size(segment);
        let whole = size > 0
            && group.requested as f64 >= policy.density_threshold() * size as f64;
        log::debug!(
            "segment decision: segment={} requested={} size={} whole={}",
            segment,
            group.requested,
            size,
            whole
        );
        if whole {
            segments.push(segment);
            segment_bytes = segment_bytes.saturating_add(size);
        } else {
            gather_bytes = gather_bytes.saturating_add(group.requested);
            gather.extend(group.entries);
        }
    }
    gather.sort_unstable();

    let mut plans = Vec::with_capacity(2);
    if !segments.is_empty() {
        plans.push(LoadPlan::Segments {
            segments,
            bytes: segment_bytes,
        });
    }
    if !gather.is_empty() {
        plans.push(LoadPlan::Gather {
            entries: gather,
            bytes: gather_bytes,
        });
    }
    for plan in &plans {
        log::debug!("load plan: kind={} bytes={}", plan.kind(), plan.bytes());
    }
    plans
}

/// Runs the plans in order and stops at the first error.
pub fn execute_plans<E>(
    plans: &[LoadPlan],
    mut exec: impl FnMut(&LoadPlan) -> Result<u64, E>,
) -> Result<u64, E> {
    let mut total = 0u64;
    for plan in plans {
        total = total.saturating_add(exec(plan)?);
    }
    Ok(total)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tier {
    Summary,
    Vector,
    NotLoaded,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Resident {
    Summary(NodeId),
    Segment(u64),
}

struct Slot<V> {
    value: V,
    bytes: u64,
    tick: u64,
}

/// Map with least-recently-used ordering.
struct LruMap<K, V> {
    slots: HashMap<K, Slot<V>>,
    order: BTreeMap<u64, K>,
    clock: u64,
}

impl<K: Copy + Eq + Hash, V> LruMap<K, V> {
    fn new() -> Self {
        Self {
            slots: HashMap::new(),
            order: BTreeMap::new(),
            clock: 0,
        }
    }

    fn next_tick(&mut self) -> u64 {
        self.clock = self.clock.wrapping_add(1);
        self.clock
    }

    fn contains(&self, key: &K) -> bool {
        self.slots.contains_key(key)
    }

    fn get(&self, key: &K) -> Option<&V> {
        self.slots.get(key).map(|s| &s.value)
    }

    fn insert(&mut self, key: K, value: V, bytes: u64) {
        if let Some(old) = self.slots.remove(&key) {
            self.order.remove(&old.tick);
        }
        let tick = self.next_tick();
        self.order.insert(tick, key);
        self.slots.insert(key, Slot { value, bytes, tick });
    }

    fn insert_if_absent(&mut self, key: K, value: V, bytes: u64) -> bool {
        if self.contains(&key) {
            return false;
        }
        self.insert(key, value, bytes);
        true
    }

    fn touch(&mut self, key: &K) -> bool {
        let tick = self.next_tick();
        match self.slots.get_mut(key) {
            Some(slot) => {
                self.order.remove(&slot.tick);
                slot.tick = tick;
                self.order.insert(tick, *key);
                true
            }
            None => false,
        }
    }

    fn pop_oldest(&mut self) -> Option<(K, u64)> {
        let (_, key) = self.order.pop_first()?;
        let slot = self.slots.remove(&key)?;
        Some((key, slot.bytes))
    }

    fn total_bytes(&self) -> u64 {
        self.slots
            .values()
            .fold(0u64, |acc, s| acc.saturating_add(s.bytes))
    }
}

struct State {
    summaries: LruMap<NodeId, Summary>,
    segments: LruMap<u64, ()>,
}

fn summary_bytes(summary: &Summary) -> u64 {
    (summary.centroid.len() as u64).saturating_add(SUMMARY_OVERHEAD_BYTES)
}

/// Thread-safe tiering: summaries first and longest, vector segments warm and evictable.
pub struct LoadManager {
    index: Arc<dyn Index>,
    geometry: Arc<dyn Geometry>,
    policy: LoadPolicy,
    state: Mutex<State>,
}

impl LoadManager {
    pub fn new(index: Arc<dyn Index>, geometry: Arc<dyn Geometry>, policy: LoadPolicy) -> Self {
        Self {
            index,
            geometry,
            policy,
            state: Mutex::new(State {
                summaries: LruMap::new(),
                segments: LruMap::new(),
            }),
        }
    }

    fn lock(&self) -> MutexGuard<'_, State> {
        self.state.lock().unwrap_or_else(|e| e.into_inner())
    }

    pub fn policy(&self) -> LoadPolicy {
        self.policy
    }

    /// Loads missing summaries without holding the lock across index calls.
    /// Returns the number of newly loaded summaries; stops at the first error.
    pub fn load_summaries(&self, nodes: &[NodeId]) -> Result<usize, IndexError> {
        let missing: Vec<NodeId> = {
            let st = self.lock();
            let mut seen = HashSet::new();
            nodes
                .iter()
                .copied()
                .filter(|n| !st.summaries.contains(n) && seen.insert(*n))
                .collect()
        };

        let mut loaded = 0usize;
        for node in missing {
            let summary = self.index.summary(node)?;
            let bytes = summary_bytes(&summary);
            if self.lock().summaries.insert_if_absent(node, summary, bytes) {
                log::debug!("summary loaded: node={} bytes={}", node.0, bytes);
                loaded += 1;
            }
        }
        Ok(loaded)
    }

    pub fn summary(&self, node: NodeId) -> Option<Summary> {
        self.lock().summaries.get(&node).cloned()
    }

    /// Like `plan_loads`, minus segments (and their entries) that are already resident.
    pub fn plan(&self, set: &EntrySet, access: Access) -> Vec<LoadPlan> {
        let plans = plan_loads(set, access, &*self.geometry, &self.policy);
        let resident: HashSet<u64> = self.lock().segments.slots.keys().copied().collect();
        if resident.is_empty() {
            return plans;
        }
        plans
            .into_iter()
            .filter_map(|p| prune_resident(p, &resident, &*self.geometry))
            .collect()
    }

    pub fn mark_segment_loaded(&self, segment: u64, bytes: u64) {
        self.lock().segments.insert(segment, (), bytes);
        log::debug!("segment loaded: segment={} bytes={}", segment, bytes);
    }

    pub fn touch_segment(&self, segment: u64) {
        self.lock().segments.touch(&segment);
    }

    pub fn tier(&self, r: Resident) -> Tier {
        let st = self.lock();
        match r {
            Resident::Summary(node) if st.summaries.contains(&node) => Tier::Summary,
            Resident::Segment(segment) if st.segments.contains(&segment) => Tier::Vector,
            _ => Tier::NotLoaded,
        }
    }

    pub fn resident_bytes(&self) -> u64 {
        let st = self.lock();
        st.segments
            .total_bytes()
            .saturating_add(st.summaries.total_bytes())
    }

    /// Frees at least `bytes` where possible: vector segments first (LRU), then summaries (LRU).
    pub fn evict(&self, bytes: u64) -> u64 {
        let mut freed = 0u64;
        if bytes == 0 {
            return freed;
        }
        let mut st = self.lock();
        while freed < bytes {
            if let Some((segment, b)) = st.segments.pop_oldest() {
                log::debug!("segment evicted: segment={} bytes={}", segment, b);
                freed = freed.saturating_add(b);
            } else if let Some((node, b)) = st.summaries.pop_oldest() {
                log::debug!("summary evicted: node={} bytes={}", node.0, b);
                freed = freed.saturating_add(b);
            } else {
                break;
            }
        }
        freed
    }
}

fn prune_resident(
    plan: LoadPlan,
    resident: &HashSet<u64>,
    geometry: &dyn Geometry,
) -> Option<LoadPlan> {
    let keep_entry = |e: &u64| !resident.contains(&geometry.segment_of(*e));
    let entry_bytes = |entries: &[u64]| {
        entries
            .iter()
            .fold(0u64, |acc, e| acc.saturating_add(geometry.entry_size(*e)))
    };
    match plan {
        LoadPlan::Segments { segments, .. } => {
            let kept: Vec<u64> = segments
                .into_iter()
                .filter(|s| !resident.contains(s))
                .collect();
            if kept.is_empty() {
                return None;
            }
            let bytes = kept
                .iter()
                .fold(0u64, |acc, s| acc.saturating_add(geometry.segment_size(*s)));
            Some(LoadPlan::Segments {
                segments: kept,
                bytes,
            })
        }
        LoadPlan::Gather { entries, .. } => {
            let kept: Vec<u64> = entries.into_iter().filter(keep_entry).collect();
            if kept.is_empty() {
                return None;
            }
            let bytes = entry_bytes(&kept);
            Some(LoadPlan::Gather {
                entries: kept,
                bytes,
            })
        }
        LoadPlan::DirectMap { entries, bytes } => {
            let kept: Vec<u64> = entries.iter().filter(keep_entry).collect();
            if kept.is_empty() {
                None
            } else if kept.len() as u128 == entries.len() {
                Some(LoadPlan::DirectMap { entries, bytes })
            } else {
                let bytes = entry_bytes(&kept);
                Some(LoadPlan::DirectMap {
                    entries: EntrySet::from_ids(kept),
                    bytes,
                })
            }
        }
    }
}

const _: () = {
    const fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<UniformGeometry>();
    assert_send_sync::<LoadPolicy>();
    assert_send_sync::<LoadPlan>();
    assert_send_sync::<LoadManager>();
};

#[cfg(test)]
#[path = "test/load_strategy.rs"]
mod tests;
