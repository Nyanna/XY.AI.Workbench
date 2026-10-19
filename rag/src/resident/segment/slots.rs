//! Valid bitmap, per-slot metadata and stream-ordered upsert.

use std::collections::{HashMap, VecDeque};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

use super::error::SegmentError;
use super::mapping::EntryRef;
use super::{Fence, Segment, SlotClass};
use crate::resident::index::EntryId;
use crate::resident::memory::dtype::{DType, F16_MASK_VALUE};
use crate::resident::memory::region::Region;

#[derive(Debug)]
pub struct ValidBitmap {
    words: Vec<AtomicU64>,
    len: u32,
}

impl ValidBitmap {
    pub fn new(len: u32) -> Self {
        let words = (len as usize).div_ceil(64);
        Self { words: (0..words).map(|_| AtomicU64::new(0)).collect(), len }
    }

    pub fn len(&self) -> u32 {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn get(&self, slot: u32) -> bool {
        slot < self.len
            && self.words[(slot / 64) as usize].load(Ordering::Acquire) & (1 << (slot % 64)) != 0
    }

    pub fn set(&self, slot: u32) {
        if slot < self.len {
            self.words[(slot / 64) as usize].fetch_or(1 << (slot % 64), Ordering::Release);
        }
    }

    pub fn clear(&self, slot: u32) {
        if slot < self.len {
            self.words[(slot / 64) as usize].fetch_and(!(1 << (slot % 64)), Ordering::Release);
        }
    }

    pub fn count(&self) -> u32 {
        self.words.iter().map(|w| w.load(Ordering::Acquire).count_ones()).sum()
    }

    /// Word snapshot for upload next to the slot data.
    pub fn snapshot(&self) -> Vec<u64> {
        self.words.iter().map(|w| w.load(Ordering::Acquire)).collect()
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SlotMeta {
    pub valid: bool,
    pub token_count: u16,
    pub gen: u64,
    pub centroid: Option<u32>,
}

/// A reserved, still invisible slot; its data must be written before it is committed.
#[derive(Debug)]
pub struct SlotWrite {
    slot: u32,
    entry: EntryRef,
    tokens: u16,
    centroid: Option<u32>,
    region: Region,
}

impl SlotWrite {
    pub fn slot(&self) -> u32 {
        self.slot
    }

    pub fn region(&self) -> Region {
        self.region
    }

    pub fn tokens(&self) -> u16 {
        self.tokens
    }
}

pub(super) struct SlotWriter {
    meta: Vec<SlotMeta>,
    next_free: u32,
    seq: u64,
    by_entry: HashMap<EntryId, u32>,
    pending: VecDeque<(SlotWrite, Arc<dyn Fence>)>,
}

impl SlotWriter {
    pub(super) fn new(slots: u32) -> Self {
        Self {
            meta: vec![SlotMeta::default(); slots as usize],
            next_free: 0,
            seq: 0,
            by_entry: HashMap::new(),
            pending: VecDeque::new(),
        }
    }

    pub(super) fn take_pending_fences(&mut self) -> Vec<Arc<dyn Fence>> {
        self.pending.drain(..).map(|(_, f)| f).collect()
    }
}

impl Segment {
    pub fn begin_write(
        &self,
        entry: EntryRef,
        tokens: u32,
        centroid: Option<u32>,
    ) -> Result<SlotWrite, SegmentError> {
        let max = self.view.class.max_tokens();
        let tokens16 = u16::try_from(tokens)
            .ok()
            .filter(|_| tokens <= max)
            .ok_or(SegmentError::TokenOverflow { tokens, max })?;
        let mut w = self.writer.lock();
        if w.next_free >= self.view.slots {
            return Err(SegmentError::SegmentFull { slots: self.view.slots });
        }
        let slot = w.next_free;
        w.next_free += 1;
        Ok(SlotWrite {
            slot,
            entry,
            tokens: tokens16,
            centroid,
            region: self.view.slot_region(slot)?,
        })
    }

    /// Makes the slot visible once `ready` (the event of its data transfer) has completed.
    /// Commits apply in submission order.
    pub fn commit_after(&self, write: SlotWrite, ready: Arc<dyn Fence>) {
        self.writer.lock().pending.push_back((write, ready));
    }

    pub fn commit(&self, write: SlotWrite, ready: &dyn Fence) -> Result<(), SegmentError> {
        if !ready.is_complete() {
            return Err(SegmentError::NotReady);
        }
        let mut w = self.writer.lock();
        self.apply(&mut w, write);
        Ok(())
    }

    /// Applies queued commits whose transfers finished; returns how many became visible.
    pub fn settle(&self) -> usize {
        let mut w = self.writer.lock();
        let mut applied = 0;
        while w.pending.front().is_some_and(|(_, f)| f.is_complete()) {
            let (write, _) = w.pending.pop_front().expect("front checked");
            self.apply(&mut w, write);
            applied += 1;
        }
        applied
    }

    fn apply(&self, w: &mut SlotWriter, write: SlotWrite) {
        let slot = write.slot;
        let _ = self.map.bind(slot, write.entry);
        w.seq += 1;
        w.meta[slot as usize] = SlotMeta {
            valid: true,
            token_count: write.tokens,
            gen: w.seq,
            centroid: write.centroid,
        };
        self.valid.set(slot);
        if let Some(old) = w.by_entry.insert(write.entry.entry, slot) {
            if old != slot {
                self.valid.clear(old);
                w.meta[old as usize].valid = false;
                tracing::trace!(segment_gen = self.view.gen, slot = old, "tombstone");
            }
        }
        tracing::trace!(segment_gen = self.view.gen, slot, "upsert");
    }

    /// Hides the entry's slot; its data stays until the segment is rebuilt.
    pub fn tombstone(&self, entry: EntryId) -> bool {
        let mut w = self.writer.lock();
        let Some(slot) = w.by_entry.remove(&entry) else {
            return false;
        };
        self.valid.clear(slot);
        w.meta[slot as usize].valid = false;
        tracing::trace!(segment_gen = self.view.gen, slot, "tombstone");
        true
    }

    pub fn is_valid(&self, slot: u32) -> bool {
        self.valid.get(slot)
    }

    pub fn valid_count(&self) -> u32 {
        self.valid.count()
    }

    pub fn valid_words(&self) -> Vec<u64> {
        self.valid.snapshot()
    }

    pub fn meta(&self, slot: u32) -> Option<SlotMeta> {
        self.writer.lock().meta.get(slot as usize).copied()
    }

    pub fn slot_of(&self, entry: EntryId) -> Option<u32> {
        self.writer.lock().by_entry.get(&entry).copied()
    }
}

fn put(out: &mut Vec<u8>, dtype: DType, v: f32) -> Result<(), SegmentError> {
    match dtype {
        DType::F32 => out.extend_from_slice(&v.to_le_bytes()),
        DType::F16 => out.extend_from_slice(&half::f16::from_f32(v).to_le_bytes()),
        DType::Bf16 => out.extend_from_slice(&half::bf16::from_f32(v).to_le_bytes()),
        dtype => return Err(SegmentError::UnsupportedDType { dtype }),
    }
    Ok(())
}

/// Host image of one slot: `tokens` rows followed by finite mask-valued padding rows, so
/// padding can never contribute a zero similarity.
pub fn encode_slot(
    class: SlotClass,
    dim: usize,
    dtype: DType,
    tokens: u32,
    vectors: &[f32],
) -> Result<Vec<u8>, SegmentError> {
    let max = class.max_tokens();
    if tokens > max {
        return Err(SegmentError::TokenOverflow { tokens, max });
    }
    let expected = tokens as usize * dim;
    if vectors.len() != expected {
        return Err(SegmentError::ShapeMismatch { expected, got: vectors.len() });
    }
    let mut out = Vec::with_capacity(class.stride(dim, dtype)? as usize);
    for &v in vectors {
        put(&mut out, dtype, v)?;
    }
    for _ in 0..(max - tokens) as usize * dim {
        put(&mut out, dtype, F16_MASK_VALUE)?;
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::super::mock::ManualFence;
    use super::super::testutil::segment;
    use super::super::SegmentStore;
    use super::*;

    fn eref(n: u64) -> EntryRef {
        EntryRef { entry: EntryId::new(n), chunk: 0 }
    }

    #[test]
    fn bitmap_set_clear_count() {
        let b = ValidBitmap::new(130);
        b.set(0);
        b.set(64);
        b.set(129);
        assert_eq!(b.count(), 3);
        assert!(b.get(64) && !b.get(65));
        b.clear(64);
        assert!(!b.get(64));
        b.set(500);
        assert!(!b.get(500));
        assert_eq!(b.snapshot().len(), 3);
    }

    #[test]
    fn slot_is_invisible_until_its_transfer_completes() {
        let store = SegmentStore::new();
        let seg = segment(&store, 1, 10, 0, 4);
        let write = seg.begin_write(eref(1), 5, Some(9)).unwrap();
        let slot = write.slot();
        let fence = ManualFence::new();
        seg.commit_after(write, fence.clone());

        assert_eq!(seg.settle(), 0);
        assert!(!seg.is_valid(slot));
        fence.complete();
        assert_eq!(seg.settle(), 1);
        assert!(seg.is_valid(slot));
        let meta = seg.meta(slot).unwrap();
        assert_eq!((meta.token_count, meta.centroid), (5, Some(9)));
    }

    #[test]
    fn commit_rejects_incomplete_transfer() {
        let store = SegmentStore::new();
        let seg = segment(&store, 1, 10, 0, 2);
        let write = seg.begin_write(eref(1), 1, None).unwrap();
        let fence = ManualFence::new();
        assert!(matches!(seg.commit(write, &*fence), Err(SegmentError::NotReady)));
        assert_eq!(seg.valid_count(), 0);
    }

    #[test]
    fn upsert_writes_a_new_slot_then_tombstones_the_old_one() {
        let store = SegmentStore::new();
        let seg = segment(&store, 1, 10, 0, 4);
        let done = ManualFence::new();
        done.complete();

        let first = seg.begin_write(eref(7), 3, None).unwrap();
        let first_region = first.region();
        seg.commit(first, &*done).unwrap();
        let before = seg.valid_words();

        let second = seg.begin_write(eref(7), 4, None).unwrap();
        assert_ne!(second.region(), first_region, "live slot must not be rewritten");
        assert_eq!(seg.valid_words(), before, "reserved slot stays invisible");
        let new_slot = second.slot();
        seg.commit(second, &*done).unwrap();

        assert!(!seg.is_valid(0) && seg.is_valid(new_slot));
        assert_eq!(seg.slot_of(EntryId::new(7)), Some(new_slot));
        assert_eq!(seg.valid_count(), 1);
        assert!(!seg.meta(0).unwrap().valid);
    }

    #[test]
    fn tombstone_hides_entry() {
        let store = SegmentStore::new();
        let seg = segment(&store, 1, 10, 0, 2);
        let done = ManualFence::new();
        done.complete();
        let w = seg.begin_write(eref(1), 1, None).unwrap();
        seg.commit(w, &*done).unwrap();
        assert!(seg.tombstone(EntryId::new(1)));
        assert!(!seg.tombstone(EntryId::new(1)));
        assert_eq!(seg.valid_count(), 0);
    }

    #[test]
    fn pending_commits_apply_in_submission_order() {
        let store = SegmentStore::new();
        let seg = segment(&store, 1, 10, 0, 4);
        let (slow, fast) = (ManualFence::new(), ManualFence::new());
        let a = seg.begin_write(eref(1), 1, None).unwrap();
        let b = seg.begin_write(eref(1), 2, None).unwrap();
        let (sa, sb) = (a.slot(), b.slot());
        seg.commit_after(a, slow.clone());
        seg.commit_after(b, fast.clone());
        fast.complete();
        assert_eq!(seg.settle(), 0);
        slow.complete();
        assert_eq!(seg.settle(), 2);
        assert!(!seg.is_valid(sa) && seg.is_valid(sb));
    }

    #[test]
    fn segment_fills_up_and_checks_token_capacity() {
        let store = SegmentStore::new();
        let seg = segment(&store, 1, 10, 0, 1);
        assert!(matches!(
            seg.begin_write(eref(1), 33, None),
            Err(SegmentError::TokenOverflow { tokens: 33, max: 32 })
        ));
        seg.begin_write(eref(1), 1, None).unwrap();
        assert!(matches!(
            seg.begin_write(eref(2), 1, None),
            Err(SegmentError::SegmentFull { slots: 1 })
        ));
    }

    #[test]
    fn encoded_slot_pads_with_finite_mask() {
        let bytes = encode_slot(SlotClass::Custom(4), 2, DType::F16, 1, &[1.0, 2.0]).unwrap();
        assert_eq!(bytes.len(), 4 * 2 * 2);
        let at = |i: usize| half::f16::from_le_bytes([bytes[2 * i], bytes[2 * i + 1]]).to_f32();
        assert_eq!((at(0), at(1)), (1.0, 2.0));
        assert_eq!(at(2), F16_MASK_VALUE);
        assert!(at(7).is_finite());

        let f32s = encode_slot(SlotClass::Custom(2), 1, DType::F32, 1, &[0.5]).unwrap();
        assert_eq!(f32s.len(), 8);
        assert!(matches!(
            encode_slot(SlotClass::T32, 2, DType::F32, 1, &[1.0]),
            Err(SegmentError::ShapeMismatch { expected: 2, got: 1 })
        ));
        assert!(matches!(
            encode_slot(SlotClass::T32, 1, DType::U8, 0, &[]),
            Err(SegmentError::UnsupportedDType { .. })
        ));
    }
}
