//! Load strategy: how a resolved entry set reaches the device (whole segments, gather, or
//! zero-copy), aggregation of concurrent requests, and coalescing of identical loads.
use std::collections::BTreeMap;
use std::hash::Hash;
use std::sync::Arc;
use futures::future::join_all;
use crate::resident::error::ManagerError;
use crate::resident::index::{EntryId, EntrySet};
use crate::resident::manager::arena::ArenaBudget;
use crate::resident::manager::flight::{FlightResult, SingleFlight};
use crate::resident::memory::error::{MemoryError, TransferDir};
use crate::resident::precision::known::DENSITY;
use crate::resident::precision::{ProfileConfig, ValueId};
pub type SegmentId = u64;
const MIB: u64 = 1 << 20;
/// Where segment boundaries and entry sizes come from; implemented by the segment layer.
pub trait LoadGeometry: Send + Sync {
    fn segment_of(&self, entry: EntryId) -> SegmentId;
    fn entry_bytes(&self, entry: EntryId) -> u64;
    fn segment_bytes(&self, segment: SegmentId) -> u64;
}
/// Equal-sized entries in consecutive segments of equal size.
#[derive(Clone, Copy, Debug)]
pub struct UniformGeometry {
    pub entries_per_segment: u64,
    pub entry_bytes: u64,
}
impl LoadGeometry for UniformGeometry {
    fn segment_of(&self, entry: EntryId) -> SegmentId {
        entry.get() / self.entries_per_segment.max(1)
    }
    fn entry_bytes(&self, _: EntryId) -> u64 {
        self.entry_bytes
    }
    fn segment_bytes(&self, _: SegmentId) -> u64 {
        self.entries_per_segment.saturating_mul(self.entry_bytes)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Access {
    Regular,
    /// Small and seldom read; may be served from mapped host memory.
    Rare,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LoadPlan {
    Segments(Vec<SegmentId>),
    /// Packed on the host into a pinned buffer and sent as one copy.
    Gather(EntrySet),
    ZeroCopy(EntrySet),
}
impl LoadPlan {
    fn kind(&self) -> &'static str {
        match self {
            LoadPlan::Segments(_) => "segments",
            LoadPlan::Gather(_) => "gather",
            LoadPlan::ZeroCopy(_) => "zero_copy",
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LoadPolicy {
    /// Fraction of a segment's bytes that must be requested for the whole segment to be loaded
    /// instead of gathering. Calibration axis `density`.
    pub density_threshold: f32,
    pub segment_bytes: u64,
    pub zero_copy_max_bytes: u64,
}
impl Default for LoadPolicy {
    fn default() -> Self {
        Self {
            density_threshold: 0.12,
            segment_bytes: 64 * MIB,
            zero_copy_max_bytes: 64 * 1024,
        }
    }
}
impl LoadPolicy {
    pub fn with_density_threshold(mut self, threshold: f32) -> Self {
        if threshold.is_finite() {
            self.density_threshold = threshold.clamp(0.0, 1.0);
        }
        self
    }
    /// Segment size from the free device budget: one eighth, within 4..=256 MiB, whole MiB.
    pub fn sized_for(mut self, budget: &ArenaBudget) -> Self {
        let target = (budget.available / 8).clamp(4 * MIB, 256 * MIB);
        self.segment_bytes = target / MIB * MIB;
        self
    }
    /// Takes the threshold from the calibrated profile when it carries the `density` axis.
    pub fn apply_profile(self, config: &ProfileConfig) -> Self {
        match config.get(&DENSITY).and_then(|v| v.as_str().parse::<u32>().ok()) {
            Some(permille) => self.with_density_threshold(permille as f32 / 1000.0),
            None => self,
        }
    }
    /// Value of the `density` calibration axis, in permille.
    pub fn density_value(&self) -> ValueId {
        ValueId::numeric((self.density_threshold * 1000.0).round() as u32)
    }
}
/// Chooses the transfer shape. Segments whose requested share reaches the threshold are loaded
/// whole (fewer, contiguous transfers); the rest is gathered. A small, rarely read set is
/// mapped instead.
pub fn plan_load(
    set: &EntrySet,
    geometry: &dyn LoadGeometry,
    policy: &LoadPolicy,
    access: Access,
) -> Vec<LoadPlan> {
    if set.is_empty() {
        return Vec::new();
    }
    let mut per_segment: BTreeMap<SegmentId, (u64, Vec<EntryId>)> = BTreeMap::new();
    let mut total = 0u64;
    for entry in set.iter() {
        let bytes = geometry.entry_bytes(entry);
        total = total.saturating_add(bytes);
        let slot = per_segment.entry(geometry.segment_of(entry)).or_default();
        slot.0 = slot.0.saturating_add(bytes);
        slot.1.push(entry);
    }
    if access == Access::Rare && total <= policy.zero_copy_max_bytes {
        tracing::debug!(
            plan = "zero_copy", bytes = total, entries = set.len(), "load decision"
        );
        return vec![LoadPlan::ZeroCopy(set.clone())];
    }
    let mut dense = Vec::new();
    let mut sparse = Vec::new();
    for (segment, (requested, entries)) in per_segment {
        let size = geometry.segment_bytes(segment);
        let density = if size == 0 { 0.0 } else { requested as f64 / size as f64 };
        let take_whole = size > 0 && density >= f64::from(policy.density_threshold);
        tracing::trace!(segment, density, take_whole, "segment density");
        if take_whole {
            dense.push(segment);
        } else {
            sparse.extend(entries);
        }
    }
    let mut plans = Vec::new();
    if !dense.is_empty() {
        plans.push(LoadPlan::Segments(dense));
    }
    if !sparse.is_empty() {
        plans.push(LoadPlan::Gather(EntrySet::from_ids(sparse)));
    }
    for plan in &plans {
        tracing::debug!(
            plan = plan.kind(), bytes = total, entries = set.len(), "load decision"
        );
    }
    plans
}
/// Device-side execution of a plan; each method returns the bytes moved.
pub trait LoadTransfer: Send + Sync {
    fn load_segments(&self, segments: &[SegmentId]) -> Result<u64, ManagerError>;
    fn load_gather(&self, set: &EntrySet) -> Result<u64, ManagerError>;
    fn map_zero_copy(&self, set: &EntrySet) -> Result<u64, ManagerError>;
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LoadOutcome {
    pub plans: Vec<LoadPlan>,
    pub bytes: u64,
}
fn execute(
    transfer: &dyn LoadTransfer,
    plans: &[LoadPlan],
) -> Result<u64, ManagerError> {
    let mut bytes = 0u64;
    for plan in plans {
        bytes = bytes
            .saturating_add(
                match plan {
                    LoadPlan::Segments(ids) => transfer.load_segments(ids)?,
                    LoadPlan::Gather(set) => transfer.load_gather(set)?,
                    LoadPlan::ZeroCopy(set) => transfer.map_zero_copy(set)?,
                },
            );
    }
    Ok(bytes)
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Aggregated<K> {
    pub index: K,
    pub set: EntrySet,
    /// Requests merged into `set`.
    pub requests: usize,
}
/// Collects the scopes of tasks against the same index so each index loads once over the union.
#[derive(Debug)]
pub struct LoadAggregator<K> {
    pending: BTreeMap<K, (EntrySet, usize)>,
}
impl<K> Default for LoadAggregator<K> {
    fn default() -> Self {
        Self { pending: BTreeMap::new() }
    }
}
impl<K: Ord> LoadAggregator<K> {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn add(&mut self, index: K, set: &EntrySet) {
        let slot = self.pending.entry(index).or_default();
        slot.0 = slot.0.union(set);
        slot.1 += 1;
    }
    pub fn is_empty(&self) -> bool {
        self.pending.is_empty()
    }
    pub fn drain(&mut self) -> Vec<Aggregated<K>> {
        std::mem::take(&mut self.pending)
            .into_iter()
            .map(|(index, (set, requests))| Aggregated { index, set, requests })
            .collect()
    }
}
pub struct ResidencyLoader<K> {
    policy: LoadPolicy,
    geometry: Arc<dyn LoadGeometry>,
    transfer: Arc<dyn LoadTransfer>,
    flights: SingleFlight<(K, EntrySet), LoadOutcome>,
}
impl<K> ResidencyLoader<K>
where
    K: Clone + Eq + Hash + Ord + Send + Sync + 'static,
{
    pub fn new(
        policy: LoadPolicy,
        geometry: Arc<dyn LoadGeometry>,
        transfer: Arc<dyn LoadTransfer>,
    ) -> Self {
        Self {
            policy,
            geometry,
            transfer,
            flights: SingleFlight::new(),
        }
    }
    pub fn policy(&self) -> &LoadPolicy {
        &self.policy
    }
    pub fn inflight(&self) -> usize {
        self.flights.inflight()
    }
    /// Identical requests in flight share one transfer; an error reaches every waiter.
    pub async fn ensure(
        &self,
        index: K,
        set: EntrySet,
        access: Access,
    ) -> FlightResult<LoadOutcome> {
        let key = (index, set.clone());
        let policy = self.policy;
        let geometry = self.geometry.clone();
        let transfer = self.transfer.clone();
        self.flights
            .run(
                key,
                move || async move {
                    let plans = plan_load(&set, &*geometry, &policy, access);
                    let work = plans.clone();
                    let bytes = tokio::task::spawn_blocking(move || execute(
                            &*transfer,
                            &work,
                        ))
                        .await
                        .map_err(|e| {
                            Arc::new(
                                ManagerError::Memory(MemoryError::TransferFailed {
                                    dir: TransferDir::HostToDevice,
                                    reason: format!("load task aborted: {e}"),
                                }),
                            )
                        })?
                        .map_err(Arc::new)?;
                    Ok(Arc::new(LoadOutcome { plans, bytes }))
                },
            )
            .await
    }
    /// Drains the aggregator and loads every index once over the union of its requests.
    pub async fn flush(
        &self,
        aggregator: &mut LoadAggregator<K>,
        access: Access,
    ) -> Vec<(Aggregated<K>, FlightResult<LoadOutcome>)> {
        let batch = aggregator.drain();
        let runs = batch
            .into_iter()
            .map(|agg| async move {
                let result = self
                    .ensure(agg.index.clone(), agg.set.clone(), access)
                    .await;
                (agg, result)
            });
        join_all(runs).await
    }
}
#[cfg(test)]
#[path = "test/residency.rs"]
mod tests;
