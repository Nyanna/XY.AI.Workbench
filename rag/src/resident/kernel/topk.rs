//! Bounded top-k selection with deterministic tie-breaking, plus the CPU merge of slice-local results.

use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;

use super::KernelHit;

/// Greater is better: higher score wins, equal scores prefer the lower offset.
#[derive(Clone, Copy, Debug)]
struct Ranked(KernelHit);

impl PartialEq for Ranked {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}
impl Eq for Ranked {}
impl PartialOrd for Ranked {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Ranked {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0
            .score
            .total_cmp(&other.0.score)
            .then_with(|| other.0.offset.cmp(&self.0.offset))
    }
}

#[derive(Debug)]
pub struct TopK {
    k: usize,
    heap: BinaryHeap<Reverse<Ranked>>,
}

impl TopK {
    pub fn new(k: usize) -> Self {
        Self { k, heap: BinaryHeap::with_capacity(k.saturating_add(1).min(4096)) }
    }

    pub fn k(&self) -> usize {
        self.k
    }

    pub fn len(&self) -> usize {
        self.heap.len()
    }

    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }

    /// NaN scores are dropped.
    pub fn push(&mut self, hit: KernelHit) {
        if self.k == 0 || hit.score.is_nan() {
            return;
        }
        let cand = Ranked(hit);
        if self.heap.len() < self.k {
            self.heap.push(Reverse(cand));
        } else if let Some(worst) = self.heap.peek() {
            if cand > worst.0 {
                self.heap.pop();
                self.heap.push(Reverse(cand));
            }
        }
    }

    pub fn extend(&mut self, hits: impl IntoIterator<Item = KernelHit>) {
        for h in hits {
            self.push(h);
        }
    }

    /// Best first.
    pub fn into_sorted(self) -> Vec<KernelHit> {
        let mut v: Vec<Ranked> = self.heap.into_iter().map(|r| r.0).collect();
        v.sort_unstable_by(|a, b| b.cmp(a));
        v.into_iter().map(|r| r.0).collect()
    }
}

/// Merges slice-local results into the global top-`k`.
pub fn merge(parts: impl IntoIterator<Item = Vec<KernelHit>>, k: usize) -> Vec<KernelHit> {
    let mut top = TopK::new(k);
    for part in parts {
        top.extend(part);
    }
    top.into_sorted()
}

#[cfg(test)]
#[path = "test/topk.rs"]
mod tests;
