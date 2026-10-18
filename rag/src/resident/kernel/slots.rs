//! In-memory `SlotSource` for CPU scoring and tests.

use super::{KernelError, SliceRange, SlotSource};

#[derive(Clone, Debug)]
pub struct InMemorySlots {
    dim: usize,
    segments: Vec<Vec<Vec<f32>>>,
    offsets: Vec<Vec<u64>>,
}

impl InMemorySlots {
    /// `segments[s][i]` holds the row-major `tokens x dim` vectors of slot `i`. Offsets are byte
    /// prefix sums per segment.
    pub fn new(dim: usize, segments: Vec<Vec<Vec<f32>>>) -> Self {
        let offsets = segments
            .iter()
            .map(|seg| {
                let mut acc = 0u64;
                seg.iter()
                    .map(|v| {
                        let at = acc;
                        acc += (v.len() * std::mem::size_of::<f32>()) as u64;
                        at
                    })
                    .collect()
            })
            .collect();
        Self { dim, segments, offsets }
    }
}

impl SlotSource for InMemorySlots {
    fn dim(&self) -> usize {
        self.dim
    }

    fn slot_count(&self, segment: u32) -> u32 {
        self.segments.get(segment as usize).map_or(0, |s| s.len() as u32)
    }

    fn slot_offset(&self, segment: u32, slot: u32) -> u64 {
        self.offsets[segment as usize][slot as usize]
    }

    fn slot_vectors(&self, segment: u32, slot: u32) -> Option<&[f32]> {
        self.segments.get(segment as usize)?.get(slot as usize).map(Vec::as_slice)
    }
}

pub(crate) fn check_range(source: &dyn SlotSource, range: SliceRange) -> Result<(), KernelError> {
    let count = source.slot_count(range.segment);
    if range.slot_from > range.slot_to || range.slot_to > count {
        return Err(KernelError::SliceOutOfRange {
            segment: range.segment,
            slot_from: range.slot_from,
            slot_to: range.slot_to,
            count,
        });
    }
    Ok(())
}
