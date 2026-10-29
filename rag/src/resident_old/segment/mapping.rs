//! Reverse mapping from kernel results to entries, and generation-aware resolution.
use parking_lot::RwLock;
use super::error::{MappingError, SegmentError};
use super::{Manifest, Segment};
use crate::resident::index::EntryId;
use crate::resident::kernel::{entry_of, entry_of_fixed};
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct EntryRef {
    pub entry: EntryId,
    pub chunk: u32,
}
/// Kernel result in slot terms; the kernel never sees entries.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SegmentHit {
    pub segment_gen: u64,
    pub slot: u32,
    pub score: f32,
}
#[derive(Clone, Debug, PartialEq)]
enum Addressing {
    Fixed { base: u64, stride: u32, count: u32 },
    Variable { offsets: Vec<u64> },
}
/// `(segment, slot) -> EntryRef`, plus address -> slot for kernel offsets.
#[derive(Debug)]
pub struct SlotMap {
    addressing: Addressing,
    entries: RwLock<Vec<Option<EntryRef>>>,
}
impl SlotMap {
    pub fn fixed(base: u64, stride: u32, count: u32) -> Self {
        Self {
            addressing: Addressing::Fixed {
                base,
                stride,
                count,
            },
            entries: RwLock::new(vec![None; count as usize]),
        }
    }
    /// `offsets` holds `n + 1` ascending prefix sums.
    pub fn variable(offsets: Vec<u64>) -> Self {
        let n = offsets.len().saturating_sub(1);
        Self {
            addressing: Addressing::Variable { offsets },
            entries: RwLock::new(vec![None; n]),
        }
    }
    pub fn len(&self) -> u32 {
        self.entries.read().len() as u32
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    pub fn bind(&self, slot: u32, entry: EntryRef) -> Result<(), SegmentError> {
        self.set(slot, Some(entry))
    }
    pub fn unbind(&self, slot: u32) -> Result<(), SegmentError> {
        self.set(slot, None)
    }
    fn set(&self, slot: u32, value: Option<EntryRef>) -> Result<(), SegmentError> {
        let mut entries = self.entries.write();
        let slots = entries.len() as u32;
        match entries.get_mut(slot as usize) {
            Some(cell) => {
                *cell = value;
                Ok(())
            }
            None => {
                Err(SegmentError::SlotOutOfRange {
                    slot,
                    slots,
                })
            }
        }
    }
    pub fn entry(&self, slot: u32) -> Option<EntryRef> {
        self.entries.read().get(slot as usize).copied().flatten()
    }
    /// Slot (or entry index for variable layouts) containing `addr`.
    pub fn index_of(&self, addr: u64) -> Option<u32> {
        let idx = match &self.addressing {
            Addressing::Fixed { base, stride, count } => {
                entry_of_fixed(*base, *stride, *count, addr)
            }
            Addressing::Variable { offsets } => entry_of(offsets, addr),
        };
        idx.map(|i| i as u32)
    }
    /// Absolute byte range of `slot`.
    pub fn slot_span(&self, slot: u32) -> Option<std::ops::Range<u64>> {
        match &self.addressing {
            Addressing::Fixed { base, stride, count } => {
                if slot >= *count {
                    return None;
                }
                let start = base + u64::from(slot) * u64::from(*stride);
                Some(start..start + u64::from(*stride))
            }
            Addressing::Variable { offsets } => {
                let i = slot as usize;
                Some(*offsets.get(i)?..*offsets.get(i + 1)?)
            }
        }
    }
    /// Absolute byte range covered by all slots; `None` when empty.
    pub fn bounds(&self) -> Option<std::ops::Range<u64>> {
        let last = self.len().checked_sub(1)?;
        Some(self.slot_span(0)?.start..self.slot_span(last)?.end)
    }
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Resolution {
    Entry(EntryRef),
    Remapped { entry: EntryRef, hit: SegmentHit },
    Discarded,
}
impl Manifest {
    pub fn find(&self, segment_gen: u64) -> Option<&std::sync::Arc<Segment>> {
        self.segments.iter().find(|s| s.view.gen == segment_gen)
    }
    fn require(
        &self,
        segment_gen: u64,
    ) -> Result<&std::sync::Arc<Segment>, MappingError> {
        self.find(segment_gen)
            .ok_or(MappingError::StaleMapping {
                gen: segment_gen,
            })
    }
    pub fn resolve(&self, hit: &SegmentHit) -> Result<EntryRef, SegmentError> {
        let seg = self.require(hit.segment_gen)?;
        seg.map
            .entry(hit.slot)
            .ok_or(SegmentError::MappingMissing {
                gen: hit.segment_gen,
                slot: hit.slot,
            })
    }
    /// Newest valid location of `entry`.
    pub fn locate(&self, entry: EntryId) -> Option<(u64, u32)> {
        self.segments
            .iter()
            .rev()
            .find_map(|s| s.slot_of(entry).map(|slot| (s.view.gen, slot)))
    }
    /// Resolves `hit` against this manifest. A hit from a generation that is no longer
    /// loaded is mapped through `previous` to the entry's current slot, or discarded.
    pub fn remap(&self, previous: &Manifest, hit: SegmentHit) -> Resolution {
        let (source, loaded) = match self.find(hit.segment_gen) {
            Some(seg) => (self, Some(seg)),
            None => (previous, None),
        };
        let Ok(entry) = source.resolve(&hit) else {
            return Resolution::Discarded;
        };
        if loaded.is_some_and(|seg| seg.is_valid(hit.slot)) {
            return Resolution::Entry(entry);
        }
        match self.locate(entry.entry) {
            Some((segment_gen, slot)) => {
                tracing::debug!(segment_gen, slot, "remap");
                Resolution::Remapped {
                    entry,
                    hit: SegmentHit {
                        segment_gen,
                        slot,
                        score: hit.score,
                    },
                }
            }
            None => Resolution::Discarded,
        }
    }
}
#[cfg(test)]
#[path = "test/mapping.rs"]
mod tests;
