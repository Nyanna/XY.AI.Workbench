use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt;
use std::sync::atomic::{AtomicU32, AtomicU64, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, PoisonError};
use std::time::Duration;
use arc_swap::{ArcSwap, Guard};
use crossbeam::channel::{bounded, Receiver, Sender, TrySendError};
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::a::slot_layout::{
    SegmentView, SlotError,
};
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::a::kernel_layout::Addr;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::a::region_handle::{
    Generation, RegionHandle,
};
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::l010::device_lane_event::{
    Event, Lane,
};
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::a::segments::SegmentId;
pub type ManifestGeneration = u64;
const KERNEL_ENTRY_BYTES: usize = 24;
const MIN_MARKER_PRUNE: usize = 32;
const DEFAULT_SUBSCRIBER_BUFFER: usize = 64;
fn is_done(event: &Event) -> bool {
    matches!(event.is_complete(), Ok(true))
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ManifestError {
    StaleGeneration { current: ManifestGeneration, got: ManifestGeneration },
    DuplicateSegment(SegmentId),
    GenerationOverflow,
}
impl fmt::Display for ManifestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ManifestError::StaleGeneration { .. } => write!(f, "stale Generation"),
            ManifestError::DuplicateSegment(id) => {
                write!(f, "duplicate segment {}", id.0)
            }
            ManifestError::GenerationOverflow => {
                write!(f, "manifest generation overflow")
            }
        }
    }
}
impl std::error::Error for ManifestError {}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SyncError {
    pub cause: String,
}
impl fmt::Display for SyncError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "sync failed: {}", self.cause)
    }
}
impl std::error::Error for SyncError {}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SegmentSpec {
    id: SegmentId,
    generation: Generation,
    region: RegionHandle,
    start: Addr,
    slot_count: u32,
    slot_stride: u32,
    bytes: u64,
}
impl SegmentSpec {
    pub fn from_view(
        id: SegmentId,
        view: &SegmentView,
        device_base: Addr,
    ) -> Result<SegmentSpec, SlotError> {
        let desc = view.descriptor(device_base)?;
        Ok(SegmentSpec {
            id,
            generation: view.generation(),
            region: view.region(),
            start: desc.start,
            slot_count: desc.slot_count,
            slot_stride: desc.stride,
            bytes: view.total_bytes(),
        })
    }
    pub fn variable(
        id: SegmentId,
        generation: Generation,
        region: RegionHandle,
        start: Addr,
        slot_count: u32,
    ) -> SegmentSpec {
        SegmentSpec {
            id,
            generation,
            region,
            start,
            slot_count,
            slot_stride: 0,
            bytes: region.len(),
        }
    }
    pub fn id(&self) -> SegmentId {
        self.id
    }
    pub fn generation(&self) -> Generation {
        self.generation
    }
    pub fn region(&self) -> RegionHandle {
        self.region
    }
    pub fn start(&self) -> Addr {
        self.start
    }
    pub fn slot_count(&self) -> u32 {
        self.slot_count
    }
    pub fn slot_stride(&self) -> u32 {
        self.slot_stride
    }
    pub fn bytes(&self) -> u64 {
        self.bytes
    }
}
struct Retired {
    region: RegionHandle,
    markers: Vec<Event>,
}
type RetireQueue = Mutex<Vec<Retired>>;
struct Markers {
    events: Vec<Event>,
    prune_at: usize,
}
struct SegmentState {
    region: RegionHandle,
    markers: Mutex<Markers>,
    retire: Arc<RetireQueue>,
}
impl SegmentState {
    fn new(region: RegionHandle, retire: Arc<RetireQueue>) -> SegmentState {
        SegmentState {
            region,
            markers: Mutex::new(Markers {
                events: Vec::new(),
                prune_at: MIN_MARKER_PRUNE,
            }),
            retire,
        }
    }
    fn same_region(&self, other: &RegionHandle) -> bool {
        self.region.arena() == other.arena() && self.region.offset() == other.offset()
            && self.region.len() == other.len()
            && self.region.generation() == other.generation()
    }
    fn add_marker(&self, marker: Event) {
        let mut m = lock(&self.markers);
        if m.events.len() >= m.prune_at {
            m.events.retain(|e| !is_done(e));
            m.prune_at = (m.events.len() * 2).max(MIN_MARKER_PRUNE);
        }
        m.events.push(marker);
    }
}
impl Drop for SegmentState {
    fn drop(&mut self) {
        let markers = self.markers.get_mut().unwrap_or_else(PoisonError::into_inner);
        let markers = std::mem::take(&mut markers.events);
        lock(&self.retire)
            .push(Retired {
                region: self.region,
                markers,
            });
    }
}
struct ManifestInner {
    generation: ManifestGeneration,
    specs: Vec<SegmentSpec>,
    states: Vec<Arc<SegmentState>>,
    total_bytes: u64,
    leases: AtomicUsize,
}
#[derive(Clone)]
pub struct Manifest {
    inner: Arc<ManifestInner>,
}
impl Manifest {
    fn from_parts(
        generation: ManifestGeneration,
        specs: Vec<SegmentSpec>,
        states: Vec<Arc<SegmentState>>,
    ) -> Manifest {
        let total_bytes = specs
            .iter()
            .fold(0u64, |acc, s| acc.saturating_add(s.bytes()));
        Manifest {
            inner: Arc::new(ManifestInner {
                generation,
                specs,
                states,
                total_bytes,
                leases: AtomicUsize::new(0),
            }),
        }
    }
    pub fn empty() -> Manifest {
        Manifest::from_parts(0, Vec::new(), Vec::new())
    }
    pub fn generation(&self) -> ManifestGeneration {
        self.inner.generation
    }
    pub fn segments(&self) -> &[SegmentSpec] {
        &self.inner.specs
    }
    pub fn len(&self) -> usize {
        self.inner.specs.len()
    }
    pub fn is_empty(&self) -> bool {
        self.inner.specs.is_empty()
    }
    pub fn total_bytes(&self) -> u64 {
        self.inner.total_bytes
    }
    pub fn lease_count(&self) -> usize {
        self.inner.leases.load(Ordering::Acquire)
    }
    pub fn kernel_table(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(self.len() * KERNEL_ENTRY_BYTES);
        for s in &self.inner.specs {
            out.extend_from_slice(&s.generation.0.to_le_bytes());
            out.extend_from_slice(&s.start.0.to_le_bytes());
            out.extend_from_slice(&s.slot_count.to_le_bytes());
            out.extend_from_slice(&s.slot_stride.to_le_bytes());
        }
        out
    }
}
impl fmt::Debug for Manifest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Manifest")
            .field("generation", &self.generation())
            .field("segments", &self.len())
            .field("leases", &self.lease_count())
            .finish()
    }
}
pub struct Lease {
    manifest: Manifest,
}
impl Lease {
    fn new(manifest: Manifest) -> Lease {
        manifest.inner.leases.fetch_add(1, Ordering::AcqRel);
        Lease { manifest }
    }
    pub fn manifest(&self) -> &Manifest {
        &self.manifest
    }
    pub fn generation(&self) -> ManifestGeneration {
        self.manifest.generation()
    }
    pub fn register_completion(&self, marker: &Event) {
        if is_done(marker) {
            return;
        }
        for state in &self.manifest.inner.states {
            state.add_marker(marker.clone());
        }
    }
}
impl Drop for Lease {
    fn drop(&mut self) {
        self.manifest.inner.leases.fetch_sub(1, Ordering::AcqRel);
    }
}
impl fmt::Debug for Lease {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Lease").field("generation", &self.generation()).finish()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChangeKind {
    Added,
    Changed,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChangeEvent {
    pub kind: ChangeKind,
    pub segment_id: SegmentId,
    pub segment_generation: Generation,
    pub manifest_generation: ManifestGeneration,
}
pub struct Subscriber {
    rx: Receiver<ChangeEvent>,
    dropped: Arc<AtomicU64>,
}
impl Subscriber {
    pub fn try_recv(&self) -> Option<ChangeEvent> {
        self.rx.try_recv().ok()
    }
    pub fn recv_timeout(&self, d: Duration) -> Option<ChangeEvent> {
        self.rx.recv_timeout(d).ok()
    }
    pub fn dropped(&self) -> u64 {
        self.dropped.load(Ordering::Relaxed)
    }
}
impl fmt::Debug for Subscriber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Subscriber").field("dropped", &self.dropped()).finish()
    }
}
struct SubscriberSlot {
    tx: Sender<ChangeEvent>,
    dropped: Arc<AtomicU64>,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StoreConfig {
    pub subscriber_buffer: usize,
}
impl Default for StoreConfig {
    fn default() -> Self {
        StoreConfig {
            subscriber_buffer: DEFAULT_SUBSCRIBER_BUFFER,
        }
    }
}
fn change_events(prev: &ManifestInner, next: &ManifestInner) -> Vec<ChangeEvent> {
    let before: HashMap<SegmentId, Generation> = prev
        .specs
        .iter()
        .map(|s| (s.id(), s.generation()))
        .collect();
    next.specs
        .iter()
        .filter_map(|s| {
            let kind = match before.get(&s.id()) {
                None => ChangeKind::Added,
                Some(g) if *g != s.generation() => ChangeKind::Changed,
                Some(_) => return None,
            };
            Some(ChangeEvent {
                kind,
                segment_id: s.id(),
                segment_generation: s.generation(),
                manifest_generation: next.generation,
            })
        })
        .collect()
}
pub struct ManifestStore {
    current: ArcSwap<ManifestInner>,
    retire: Arc<RetireQueue>,
    staged: Mutex<VecDeque<(Manifest, Event)>>,
    subscribers: Mutex<Vec<SubscriberSlot>>,
    config: StoreConfig,
    pending_conflicts: AtomicU32,
    retries: AtomicU64,
}
impl ManifestStore {
    pub fn new(config: StoreConfig) -> ManifestStore {
        ManifestStore {
            current: ArcSwap::new(Manifest::empty().inner),
            retire: Arc::new(Mutex::new(Vec::new())),
            staged: Mutex::new(VecDeque::new()),
            subscribers: Mutex::new(Vec::new()),
            config,
            pending_conflicts: AtomicU32::new(0),
            retries: AtomicU64::new(0),
        }
    }
    pub fn current(&self) -> Manifest {
        Manifest {
            inner: self.current.load_full(),
        }
    }
    pub fn lease(&self) -> Lease {
        Lease::new(self.current())
    }
    pub fn successor(
        &self,
        base: &Manifest,
        specs: Vec<SegmentSpec>,
    ) -> Result<Manifest, ManifestError> {
        let generation = base
            .generation()
            .checked_add(1)
            .ok_or(ManifestError::GenerationOverflow)?;
        let mut seen = HashSet::with_capacity(specs.len());
        for s in &specs {
            if !seen.insert(s.id()) {
                return Err(ManifestError::DuplicateSegment(s.id()));
            }
        }
        let base_index: HashMap<SegmentId, usize> = base
            .inner
            .specs
            .iter()
            .enumerate()
            .map(|(i, s)| (s.id(), i))
            .collect();
        let states = specs
            .iter()
            .map(|s| match base_index.get(&s.id()) {
                Some(&i) if base.inner.states[i].same_region(&s.region()) => {
                    Arc::clone(&base.inner.states[i])
                }
                _ => Arc::new(SegmentState::new(s.region(), Arc::clone(&self.retire))),
            })
            .collect();
        Ok(Manifest::from_parts(generation, specs, states))
    }
    pub fn publish(&self, next: Manifest) -> Result<Manifest, ManifestError> {
        let got = next.generation();
        loop {
            let cur = self.current.load_full();
            if got <= cur.generation {
                return Err(ManifestError::StaleGeneration {
                    current: cur.generation,
                    got,
                });
            }
            if self.take_conflict() {
                self.retries.fetch_add(1, Ordering::Relaxed);
                continue;
            }
            let prev = Guard::into_inner(
                self.current.compare_and_swap(&cur, Arc::clone(&next.inner)),
            );
            if Arc::ptr_eq(&prev, &cur) {
                self.notify(&prev, &next.inner);
                return Ok(Manifest { inner: prev });
            }
            self.retries.fetch_add(1, Ordering::Relaxed);
        }
    }
    fn take_conflict(&self) -> bool {
        self.pending_conflicts
            .try_update(Ordering::AcqRel, Ordering::Acquire, |n| n.checked_sub(1))
            .is_ok()
    }
    fn notify(&self, prev: &ManifestInner, next: &ManifestInner) {
        let mut subs = lock(&self.subscribers);
        if subs.is_empty() {
            return;
        }
        let events = change_events(prev, next);
        if events.is_empty() {
            return;
        }
        subs.retain(|slot| {
            for ev in &events {
                match slot.tx.try_send(*ev) {
                    Ok(()) => {}
                    Err(TrySendError::Full(_)) => {
                        slot.dropped.fetch_add(1, Ordering::Relaxed);
                    }
                    Err(TrySendError::Disconnected(_)) => return false,
                }
            }
            true
        });
    }
    pub fn stage(&self, next: Manifest, event: Event) {
        lock(&self.staged).push_back((next, event));
    }
    pub fn stage_copy<F>(
        &self,
        lane: &mut Lane,
        enqueue: F,
        next: Manifest,
    ) -> Result<(), SyncError>
    where
        F: FnOnce(&mut Lane) -> Result<(), String>,
    {
        enqueue(lane).map_err(|cause| SyncError { cause })?;
        let event = lane.record_event().map_err(|e| SyncError { cause: e.to_string() })?;
        self.stage(next, event);
        Ok(())
    }
    pub fn publish_ready(&self) -> usize {
        let mut queue = lock(&self.staged);
        let mut published = 0;
        loop {
            let ready = match queue.front() {
                None => break,
                Some((_, event)) => {
                    match event.is_complete() {
                        Ok(done) => done,
                        Err(e) => {
                            log::warn!("staged manifest event failed: {e}");
                            false
                        }
                    }
                }
            };
            if !ready {
                break;
            }
            let Some((next, _)) = queue.pop_front() else {
                break;
            };
            match self.publish(next) {
                Ok(prev) => {
                    published += 1;
                    drop(prev);
                }
                Err(e) => log::warn!("staged manifest discarded: {e}"),
            }
        }
        published
    }
    pub fn staged_count(&self) -> usize {
        lock(&self.staged).len()
    }
    pub fn subscribe(&self) -> Subscriber {
        let (tx, rx) = bounded(self.config.subscriber_buffer.max(1));
        let dropped = Arc::new(AtomicU64::new(0));
        lock(&self.subscribers)
            .push(SubscriberSlot {
                tx,
                dropped: Arc::clone(&dropped),
            });
        Subscriber { rx, dropped }
    }
    pub fn reclaim(&self) -> Vec<RegionHandle> {
        let mut queue = lock(&self.retire);
        let mut out = Vec::new();
        let mut kept = Vec::with_capacity(queue.len());
        for entry in queue.drain(..) {
            if entry.markers.iter().all(is_done) {
                out.push(entry.region);
            } else {
                kept.push(entry);
            }
        }
        *queue = kept;
        out
    }
    pub fn retired_count(&self) -> usize {
        lock(&self.retire).len()
    }
    pub fn inject_swap_conflicts(&self, n: u32) {
        self.pending_conflicts.store(n, Ordering::Release);
    }
    pub fn swap_retries(&self) -> u64 {
        self.retries.load(Ordering::Relaxed)
    }
}
impl fmt::Debug for ManifestStore {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ManifestStore")
            .field("generation", &self.current().generation())
            .field("staged", &self.staged_count())
            .field("retired", &self.retired_count())
            .finish()
    }
}
static_assertions::assert_impl_all!(Manifest : Send, Sync);
static_assertions::assert_impl_all!(Lease : Send, Sync);
static_assertions::assert_impl_all!(SegmentSpec : Send, Sync);
static_assertions::assert_impl_all!(ChangeEvent : Send, Sync);
static_assertions::assert_impl_all!(Subscriber : Send, Sync);
static_assertions::assert_impl_all!(ManifestStore : Send, Sync);
static_assertions::assert_impl_all!(ManifestError : Send, Sync);
static_assertions::assert_impl_all!(SyncError : Send, Sync);
#[cfg(test)]
#[path = "test/manifest.rs"]
mod tests;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::l010::sync_util::lock;
