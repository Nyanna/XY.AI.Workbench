use super::super::_4::_3::a::kernel_layout::Slot5;

/// Assigned in ascending order, never reused.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SegmentId(pub u64);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SlotRef {
    pub segment: SegmentId,
    pub slot: Slot5,
}

#[cfg(test)]
#[path = "test/segments.rs"]
mod tests;
