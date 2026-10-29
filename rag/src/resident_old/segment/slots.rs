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
#[path = "test/slots.rs"]
mod tests;
