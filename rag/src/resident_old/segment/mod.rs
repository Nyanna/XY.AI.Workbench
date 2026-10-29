//! Index segments inside a subengine: slot geometry, immutable segments published as
//! generation manifests, and deferred release behind leases and stream events.

pub mod error;
pub mod mapping;
pub mod slots;
pub mod sync;

use std::sync::Arc;

use arc_swap::ArcSwap;
use parking_lot::Mutex;
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};

pub use error::SegmentError;
pub use mapping::{EntryRef, Resolution, SegmentHit, SlotMap};
pub use slots::{encode_slot, SlotMeta, SlotWrite, ValidBitmap};
pub use sync::{Publisher, SegmentBus, SegmentEvent, SegmentEventKind};

use crate::resident::failpoint;
use crate::resident::kernel::{SegmentDesc, SegmentLayout};
use crate::resident::memory::dtype::DType;
use crate::resident::memory::region::Region;
use crate::resident::op::{Lease, PinCount};
use crate::resident::subengine::EventHandle;

use slots::SlotWriter;

/// Completion marker of device work that may still touch a segment.
pub trait Fence: Send + Sync {
    fn is_complete(&self) -> bool;
}

impl Fence for EventHandle {
    fn is_complete(&self) -> bool {
        EventHandle::is_complete(self)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum SlotClass {
    T32,
    T64,
    T128,
    Custom(u16),
}

impl SlotClass {
    pub const fn max_tokens(self) -> u32 {
        match self {
            Self::T32 => 32,
            Self::T64 => 64,
            Self::T128 => 128,
            Self::Custom(n) => n as u32,
        }
    }

    /// Bytes of one slot: `max_tokens x dim x dtype`.
    pub fn stride(self, dim: usize, dtype: DType) -> Result<u32, SegmentError> {
        let bytes = u64::from(self.max_tokens())
            .checked_mul(dim as u64)
            .and_then(|n| n.checked_mul(dtype.size_bytes() as u64))
            .ok_or(SegmentError::GeometryOverflow)?;
        u32::try_from(bytes).map_err(|_| SegmentError::GeometryOverflow)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SegmentView {
    pub base: Region,
    pub slots: u32,
    pub stride: u32,
    pub class: SlotClass,
    pub dtype: DType,
    pub dim: usize,
    pub gen: u64,
}

impl SegmentView {
    pub fn new(
        base: Region,
        slots: u32,
        class: SlotClass,
        dim: usize,
        dtype: DType,
        gen: u64,
    ) -> Result<Self, SegmentError> {
        let stride = class.stride(dim, dtype)?;
        let needed = u64::from(slots) * u64::from(stride);
        if base.len < needed {
            return Err(SegmentError::RegionTooSmall { needed, have: base.len });
        }
        Ok(Self { base, slots, stride, class, dtype, dim, gen })
    }

    pub fn bytes(&self) -> u64 {
        u64::from(self.slots) * u64::from(self.stride)
    }

    pub fn slot_region(&self, slot: u32) -> Result<Region, SegmentError> {
        if slot >= self.slots {
            return Err(SegmentError::SlotOutOfRange { slot, slots: self.slots });
        }
        Ok(Region {
            offset: self.base.offset + u64::from(slot) * u64::from(self.stride),
            len: u64::from(self.stride),
            ..self.base
        })
    }

    pub fn desc(&self) -> SegmentDesc {
        SegmentDesc {
            base: self.base.offset,
            count: self.slots,
            layout: SegmentLayout::Fixed { stride: self.stride },
        }
    }
}

/// Released slab together with the fences that must complete before it may be reused.
pub struct Retired {
    pub region: Region,
    pub fences: Vec<Arc<dyn Fence>>,
}

/// Slots are only ever appended and never rewritten while the segment is loaded;
/// reclaiming tombstoned space is done by rebuilding the segment.
pub struct Segment {
    id: u64,
    view: SegmentView,
    valid: ValidBitmap,
    map: SlotMap,
    writer: Mutex<SlotWriter>,
    fences: Mutex<Vec<Arc<dyn Fence>>>,
    retire: Option<UnboundedSender<Retired>>,
}

impl Segment {
    pub fn new(id: u64, view: SegmentView, retire: Option<UnboundedSender<Retired>>) -> Arc<Self> {
        Arc::new(Self {
            id,
            view,
            valid: ValidBitmap::new(view.slots),
            map: SlotMap::fixed(view.base.offset, view.stride, view.slots),
            writer: Mutex::new(SlotWriter::new(view.slots)),
            fences: Mutex::new(Vec::new()),
            retire,
        })
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn view(&self) -> &SegmentView {
        &self.view
    }

    pub fn map(&self) -> &SlotMap {
        &self.map
    }

    /// Registers device work that reads this segment; release waits for it.
    pub fn track(&self, fence: Arc<dyn Fence>) {
        let mut fences = self.fences.lock();
        fences.retain(|f| !f.is_complete());
        fences.push(fence);
    }

    /// Converts a kernel hit offset into a slot-based hit.
    pub fn hit(&self, kernel: crate::resident::kernel::KernelHit) -> Result<SegmentHit, SegmentError> {
        let slot = self.map.index_of(kernel.offset).ok_or(SegmentError::SlotOutOfRange {
            slot: u32::MAX,
            slots: self.view.slots,
        })?;
        Ok(SegmentHit { segment_gen: self.view.gen, slot, score: kernel.score })
    }
}

impl Drop for Segment {
    fn drop(&mut self) {
        let Some(tx) = &self.retire else { return };
        let mut fences = std::mem::take(self.fences.get_mut());
        fences.extend(self.writer.get_mut().take_pending_fences());
        tracing::debug!(segment_gen = self.view.gen, "segment released");
        let _ = tx.send(Retired { region: self.view.base, fences });
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SegmentTableEntry {
    pub segment_gen: u64,
    pub desc: SegmentDesc,
}

/// Per-segment base table handed to a kernel launch (segments, not entries).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SegmentTable {
    entries: Vec<SegmentTableEntry>,
}

impl SegmentTable {
    pub fn entries(&self) -> &[SegmentTableEntry] {
        &self.entries
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// 24 bytes per segment, little endian: `segment_gen: u64, base: u64, count: u32, stride: u32`.
    pub fn encode(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(self.entries.len() * 24);
        for e in &self.entries {
            let stride = match e.desc.layout {
                SegmentLayout::Fixed { stride } => stride,
                _ => 0,
            };
            out.extend_from_slice(&e.segment_gen.to_le_bytes());
            out.extend_from_slice(&e.desc.base.to_le_bytes());
            out.extend_from_slice(&e.desc.count.to_le_bytes());
            out.extend_from_slice(&stride.to_le_bytes());
        }
        out
    }
}

pub struct Manifest {
    generation: u64,
    segments: Vec<Arc<Segment>>,
    pin: Arc<PinCount>,
}

impl Manifest {
    pub fn empty() -> Self {
        Self::new(0, Vec::new())
    }

    pub fn new(generation: u64, segments: Vec<Arc<Segment>>) -> Self {
        Self { generation, segments, pin: PinCount::new() }
    }

    pub fn successor(&self, segments: Vec<Arc<Segment>>) -> Self {
        Self::new(self.generation + 1, segments)
    }

    pub fn generation(&self) -> u64 {
        self.generation
    }

    pub fn segments(&self) -> &[Arc<Segment>] {
        &self.segments
    }

    /// Segment leases currently held on this generation.
    pub fn pins(&self) -> usize {
        self.pin.get()
    }

    pub fn bytes(&self) -> u64 {
        self.segments.iter().map(|s| s.view.bytes()).sum()
    }

    pub fn table(&self) -> SegmentTable {
        SegmentTable {
            entries: self
                .segments
                .iter()
                .map(|s| SegmentTableEntry { segment_gen: s.view.gen, desc: s.view.desc() })
                .collect(),
        }
    }
}

/// Keeps one manifest generation, and therefore its segments, alive for a reader.
pub struct GenerationLease {
    manifest: Arc<Manifest>,
    _leases: Vec<Lease>,
}

impl GenerationLease {
    pub fn manifest(&self) -> &Manifest {
        &self.manifest
    }

    pub fn generation(&self) -> u64 {
        self.manifest.generation
    }

    pub fn track(&self, fence: Arc<dyn Fence>) {
        for s in &self.manifest.segments {
            s.track(Arc::clone(&fence));
        }
    }
}

struct Reclaim {
    rx: UnboundedReceiver<Retired>,
    held: Vec<Retired>,
}

pub struct SegmentStore {
    current: ArcSwap<Manifest>,
    retire_tx: UnboundedSender<Retired>,
    reclaim: Mutex<Reclaim>,
}

impl Default for SegmentStore {
    fn default() -> Self {
        Self::new()
    }
}

impl SegmentStore {
    pub fn new() -> Self {
        let (retire_tx, rx) = unbounded_channel();
        Self {
            current: ArcSwap::from_pointee(Manifest::empty()),
            retire_tx,
            reclaim: Mutex::new(Reclaim { rx, held: Vec::new() }),
        }
    }

    /// Sender to give to every segment so that its slab is reported on drop.
    pub fn retire_sender(&self) -> UnboundedSender<Retired> {
        self.retire_tx.clone()
    }

    pub fn current(&self) -> Arc<Manifest> {
        self.current.load_full()
    }

    pub fn pin(&self) -> GenerationLease {
        let manifest = self.current.load_full();
        let leases = manifest
            .segments
            .iter()
            .map(|s| Lease::new(s.view.base, Arc::clone(&manifest.pin), None))
            .collect();
        GenerationLease { manifest, _leases: leases }
    }

    /// Atomically replaces the current manifest; returns the previous one.
    pub fn publish(&self, next: Manifest) -> Result<Arc<Manifest>, SegmentError> {
        let next = Arc::new(next);
        loop {
            let cur = self.current.load_full();
            if next.generation <= cur.generation {
                return Err(SegmentError::StaleGeneration { gen: next.generation });
            }
            if failpoint::hit("segment.publish_cas", next.generation) {
                continue;
            }
            let prev = self.current.compare_and_swap(&cur, Arc::clone(&next));
            if Arc::ptr_eq(&*prev, &cur) {
                tracing::debug!(generation = next.generation, "manifest published");
                return Ok(cur);
            }
        }
    }

    /// Slabs whose segments are no longer referenced by any manifest or lease and whose
    /// tracked device work has finished; the caller returns them to the arena.
    pub fn reclaim(&self) -> Vec<Region> {
        let mut r = self.reclaim.lock();
        while let Ok(item) = r.rx.try_recv() {
            r.held.push(item);
        }
        let (ready, held): (Vec<_>, Vec<_>) = std::mem::take(&mut r.held)
            .into_iter()
            .partition(|x| x.fences.iter().all(|f| f.is_complete()));
        r.held = held;
        ready.into_iter().map(|x| x.region).collect()
    }

    /// Device bytes needed while `incoming` is built next to the current generation.
    pub fn cow_budget(&self, incoming: u64) -> u64 {
        self.current.load().bytes() + incoming
    }
}

#[cfg(any(test, feature = "mock"))]
pub mod mock {
    use std::sync::atomic::{AtomicBool, Ordering};

    use super::Fence;

    #[derive(Default)]
    pub struct ManualFence(AtomicBool);

    impl ManualFence {
        pub fn new() -> std::sync::Arc<Self> {
            std::sync::Arc::default()
        }

        pub fn complete(&self) {
            self.0.store(true, Ordering::Release);
        }
    }

    impl Fence for ManualFence {
        fn is_complete(&self) -> bool {
            self.0.load(Ordering::Acquire)
        }
    }
}

#[cfg(test)]
pub(crate) mod testutil {
    use super::*;
    use crate::resident::memory::region::ArenaId;

    pub fn region(offset: u64, len: u64) -> Region {
        Region { arena: ArenaId(1), offset, len, gen: 0 }
    }

    pub fn segment(store: &SegmentStore, id: u64, gen: u64, offset: u64, slots: u32) -> Arc<Segment> {
        let view = SegmentView::new(region(offset, 1 << 20), slots, SlotClass::T32, 4, DType::F32, gen).unwrap();
        Segment::new(id, view, Some(store.retire_sender()))
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
