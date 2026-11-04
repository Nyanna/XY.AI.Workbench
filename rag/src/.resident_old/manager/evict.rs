//! Eviction for device and host mirrors: idle expiry, watermark hysteresis and cost-aware
//! victim choice. Eviction is explicit; summaries and pinned entries are never victims.
use std::cmp::Ordering as CmpOrdering;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};
use crate::resident::error::ManagerError;
use crate::resident::manager::arena::ArenaBudget;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tier {
    /// Centroids, prefix tables; small and held longest.
    Summary,
    Vector,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EvictReason {
    Idle,
    Pressure,
    NeedSpace,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Eviction<K> {
    pub key: K,
    pub bytes: u64,
    pub reason: EvictReason,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Occupancy {
    pub used: u64,
    pub capacity: u64,
}
impl Occupancy {
    /// Usable capacity excludes the reserve and the copy-on-write headroom.
    pub fn from_budget(b: &ArenaBudget) -> Self {
        Self {
            used: b.live.saturating_add(b.retired),
            capacity: b.capacity.saturating_sub(b.reserve).saturating_sub(b.cow_headroom),
        }
    }
    fn free(&self) -> u64 {
        self.capacity.saturating_sub(self.used)
    }
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EvictPolicy {
    /// Pressure eviction starts above this fraction of capacity.
    pub high_watermark: f64,
    /// Pressure eviction stops at or below this fraction.
    pub low_watermark: f64,
    pub idle_ttl: Duration,
}
impl EvictPolicy {
    pub fn new(high_watermark: f64, low_watermark: f64, idle_ttl: Duration) -> Self {
        let high = if high_watermark.is_finite() {
            high_watermark.clamp(0.0, 1.0)
        } else {
            1.0
        };
        let low = if low_watermark.is_finite() {
            low_watermark.clamp(0.0, high)
        } else {
            high
        };
        Self {
            high_watermark: high,
            low_watermark: low,
            idle_ttl,
        }
    }
    /// Reloads cross PCIe and spike the copy-on-write budget: long idle time, wide band.
    pub fn vram() -> Self {
        Self::new(0.90, 0.75, Duration::from_secs(600))
    }
    /// The OS pages host memory transparently, so a short idle time is enough.
    pub fn ram() -> Self {
        Self::new(0.85, 0.70, Duration::from_secs(60))
    }
}
/// Keeps its entry out of eviction while alive.
#[derive(Debug)]
pub struct Pin {
    pins: Arc<AtomicU32>,
}
impl Drop for Pin {
    fn drop(&mut self) {
        self.pins.fetch_sub(1, Ordering::AcqRel);
    }
}
struct Item {
    bytes: u64,
    tier: Tier,
    reload_cost: f64,
    hits: u64,
    priority: f64,
    last_used: Instant,
    pins: Arc<AtomicU32>,
}
impl Item {
    fn evictable(&self) -> bool {
        self.tier == Tier::Vector && self.pins.load(Ordering::Acquire) == 0
    }
}
/// Greedy-dual-size-frequency: `priority = floor + hits * reload_cost / bytes`. The floor rises
/// with every pressure eviction, which ages untouched entries without a recency list.
pub struct Evictor<K> {
    policy: EvictPolicy,
    items: HashMap<K, Item>,
    floor: f64,
    bytes: u64,
}
impl<K> Evictor<K>
where
    K: Clone + Eq + Hash + Ord + Debug,
{
    pub fn new(policy: EvictPolicy) -> Self {
        Self {
            policy,
            items: HashMap::new(),
            floor: 0.0,
            bytes: 0,
        }
    }
    pub fn policy(&self) -> &EvictPolicy {
        &self.policy
    }
    pub fn len(&self) -> usize {
        self.items.len()
    }
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
    pub fn tracked_bytes(&self) -> u64 {
        self.bytes
    }
    pub fn contains(&self, key: &K) -> bool {
        self.items.contains_key(key)
    }
    /// `reload_cost` is in arbitrary but consistent units (e.g. estimated microseconds).
    pub fn insert(
        &mut self,
        key: K,
        bytes: u64,
        tier: Tier,
        reload_cost: f64,
        now: Instant,
    ) {
        let cost = if reload_cost.is_finite() { reload_cost.max(0.0) } else { 0.0 };
        let mut item = Item {
            bytes,
            tier,
            reload_cost: cost,
            hits: 1,
            priority: 0.0,
            last_used: now,
            pins: Arc::new(AtomicU32::new(0)),
        };
        item.priority = self.floor + score(&item);
        if let Some(old) = self.items.insert(key, item) {
            self.bytes = self.bytes.saturating_sub(old.bytes);
        }
        self.bytes = self.bytes.saturating_add(bytes);
    }
    pub fn touch(&mut self, key: &K, now: Instant) -> bool {
        let floor = self.floor;
        let Some(item) = self.items.get_mut(key) else {
            return false;
        };
        item.hits = item.hits.saturating_add(1);
        item.last_used = now;
        item.priority = floor + score(item);
        true
    }
    pub fn pin(&self, key: &K) -> Option<Pin> {
        let item = self.items.get(key)?;
        item.pins.fetch_add(1, Ordering::AcqRel);
        Some(Pin { pins: item.pins.clone() })
    }
    pub fn remove(&mut self, key: &K) -> Option<u64> {
        let item = self.items.remove(key)?;
        self.bytes = self.bytes.saturating_sub(item.bytes);
        Some(item.bytes)
    }
    /// Idle expiry plus watermark relief; the periodic entry point of the manager cycle.
    pub fn tick(&mut self, now: Instant, occupancy: Occupancy) -> Vec<Eviction<K>> {
        let mut out = self.expire(now);
        let freed: u64 = out.iter().map(|e| e.bytes).sum();
        let occupancy = Occupancy {
            used: occupancy.used.saturating_sub(freed),
            ..occupancy
        };
        out.extend(self.relieve(occupancy));
        out
    }
    pub fn expire(&mut self, now: Instant) -> Vec<Eviction<K>> {
        let ttl = self.policy.idle_ttl;
        let mut idle: Vec<K> = self
            .items
            .iter()
            .filter(|(_, i)| {
                i.evictable() && now.saturating_duration_since(i.last_used) >= ttl
            })
            .map(|(k, _)| k.clone())
            .collect();
        idle.sort();
        idle.into_iter().map(|k| self.evict(k, EvictReason::Idle)).collect()
    }
    /// Hysteresis: nothing happens until `used` exceeds the high watermark; then victims are
    /// taken until `used` is at or below the low watermark.
    pub fn relieve(&mut self, occupancy: Occupancy) -> Vec<Eviction<K>> {
        let cap = occupancy.capacity as f64;
        if (occupancy.used as f64) <= cap * self.policy.high_watermark {
            return Vec::new();
        }
        let target = (cap * self.policy.low_watermark) as u64;
        let mut used = occupancy.used;
        let mut out = Vec::new();
        for key in self.victim_order() {
            if used <= target {
                break;
            }
            let ev = self.evict_raising_floor(key, EvictReason::Pressure);
            used = used.saturating_sub(ev.bytes);
            out.push(ev);
        }
        out
    }
    /// Frees enough victims for an allocation of `need` bytes. Evicts nothing and fails with
    /// `ArenaExhausted` when pinned entries and summaries leave too little to free.
    pub fn make_room(
        &mut self,
        need: u64,
        occupancy: Occupancy,
    ) -> Result<Vec<Eviction<K>>, ManagerError> {
        let free = occupancy.free();
        if free >= need {
            return Ok(Vec::new());
        }
        let mut gained = free;
        let mut chosen = Vec::new();
        for key in self.victim_order() {
            if gained >= need {
                break;
            }
            gained = gained.saturating_add(self.items[&key].bytes);
            chosen.push(key);
        }
        if gained < need {
            return Err(ManagerError::ArenaExhausted {
                requested: need,
                available: free,
            });
        }
        Ok(
            chosen
                .into_iter()
                .map(|k| self.evict_raising_floor(k, EvictReason::NeedSpace))
                .collect(),
        )
    }
    fn victim_order(&self) -> Vec<K> {
        let mut candidates: Vec<(&K, &Item)> = self
            .items
            .iter()
            .filter(|(_, i)| i.evictable())
            .collect();
        candidates
            .sort_by(|(ka, a), (kb, b)| {
                a.priority
                    .partial_cmp(&b.priority)
                    .unwrap_or(CmpOrdering::Equal)
                    .then(a.last_used.cmp(&b.last_used))
                    .then(ka.cmp(kb))
            });
        candidates.into_iter().map(|(k, _)| k.clone()).collect()
    }
    fn evict_raising_floor(&mut self, key: K, reason: EvictReason) -> Eviction<K> {
        if let Some(item) = self.items.get(&key) {
            self.floor = self.floor.max(item.priority);
        }
        self.evict(key, reason)
    }
    fn evict(&mut self, key: K, reason: EvictReason) -> Eviction<K> {
        let item = self.items.remove(&key).expect("victim is tracked");
        self.bytes = self.bytes.saturating_sub(item.bytes);
        tracing::debug!(
            victim = ? key, bytes = item.bytes, hits = item.hits, reason = ? reason,
            "eviction decision"
        );
        Eviction {
            key,
            bytes: item.bytes,
            reason,
        }
    }
}
fn score(item: &Item) -> f64 {
    item.hits as f64 * item.reload_cost / item.bytes.max(1) as f64
}
#[cfg(test)]
#[path = "test/evict.rs"]
mod tests;
