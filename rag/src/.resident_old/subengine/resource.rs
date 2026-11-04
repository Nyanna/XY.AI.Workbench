use std::collections::HashMap;
use std::fmt;
use std::hash::Hash;
use parking_lot::Mutex;
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ResourceKey(String);
impl ResourceKey {
    pub fn new(key: impl Into<String>) -> Self {
        Self(key.into())
    }
}
impl fmt::Display for ResourceKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}
pub trait Resource: Send + Sync + 'static {
    fn key(&self) -> ResourceKey;
    /// Relative cost of re-initialising after eviction; lets callers prefer cheap victims.
    fn init_cost_hint(&self) -> u64 {
        0
    }
    /// Veto for resource-specific reasons beyond lease and TTL checks.
    fn evictable(&self, _now_ms: u64) -> bool {
        true
    }
}
/// Exponentially weighted moving average; the first sample primes the value.
#[derive(Clone, Copy, Debug)]
pub struct Ewma {
    value: f32,
    alpha: f32,
    primed: bool,
}
impl Ewma {
    pub fn new(alpha: f32) -> Self {
        Self {
            value: 0.0,
            alpha: alpha.clamp(f32::EPSILON, 1.0),
            primed: false,
        }
    }
    pub fn observe(&mut self, sample: f32) -> f32 {
        if self.primed {
            self.value += self.alpha * (sample - self.value);
        } else {
            self.value = sample;
            self.primed = true;
        }
        self.value
    }
    pub fn get(&self) -> Option<f32> {
        self.primed.then_some(self.value)
    }
}
#[derive(Clone, Copy, Debug)]
pub struct PullConfig {
    pub alpha: f32,
    /// Wall time one batch should take at the observed rate.
    pub target_batch_ms: f32,
    pub min_batch: usize,
    pub max_batch: usize,
}
impl Default for PullConfig {
    fn default() -> Self {
        Self {
            alpha: 0.2,
            target_batch_ms: 20.0,
            min_batch: 1,
            max_batch: 256,
        }
    }
}
/// Derives the batch size to pull from observed throughput; the clamp bounds load oscillation.
pub struct PullController {
    rate_per_ms: Mutex<Ewma>,
    cfg: PullConfig,
}
impl PullController {
    pub fn new(cfg: PullConfig) -> Self {
        Self {
            rate_per_ms: Mutex::new(Ewma::new(cfg.alpha)),
            cfg,
        }
    }
    pub fn observe(&self, items: usize, elapsed_ms: f32) {
        if items > 0 && elapsed_ms > 0.0 {
            self.rate_per_ms.lock().observe(items as f32 / elapsed_ms);
        }
    }
    /// Items per second; zero until the first observation.
    pub fn throughput(&self) -> f32 {
        self.rate_per_ms.lock().get().map_or(0.0, |r| r * 1000.0)
    }
    pub fn batch_size(&self) -> usize {
        let min = self.cfg.min_batch.max(1);
        let max = self.cfg.max_batch.max(min);
        match self.rate_per_ms.lock().get() {
            Some(rate) => {
                ((rate * self.cfg.target_batch_ms).round() as usize).clamp(min, max)
            }
            None => min,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Capacity {
    pub in_flight: usize,
    pub max_in_flight: usize,
}
impl Capacity {
    pub fn free(&self) -> usize {
        self.max_in_flight.saturating_sub(self.in_flight)
    }
}
/// Restricts which pending work a subengine is willing to take.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ClaimFilter {
    pub model: Option<ResourceKey>,
    pub index: Option<ResourceKey>,
}
/// Implemented by the processor: hands out at most `budget` jobs matching `filter`.
pub trait Claim<J> {
    fn claim(&self, filter: &ClaimFilter, budget: usize) -> Vec<J>;
}
/// Groups items by key, keeping first-seen group order and item order within each group.
pub fn coalesce<T, K: Eq + Hash + Clone>(
    items: impl IntoIterator<Item = T>,
    key: impl Fn(&T) -> K,
) -> Vec<(K, Vec<T>)> {
    let mut slots: HashMap<K, usize> = HashMap::new();
    let mut groups: Vec<(K, Vec<T>)> = Vec::new();
    for item in items {
        let k = key(&item);
        match slots.get(&k) {
            Some(&i) => groups[i].1.push(item),
            None => {
                slots.insert(k.clone(), groups.len());
                groups.push((k, vec![item]));
            }
        }
    }
    groups
}
#[cfg(test)]
#[path = "test/resource.rs"]
mod tests;
