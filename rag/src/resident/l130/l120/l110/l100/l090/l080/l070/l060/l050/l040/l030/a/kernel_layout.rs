use std::fmt;
use std::ops::Range;
use super::super::l020::a::index_contract::EntryId;
/// Opaque address (byte offset in the address space of a layout, including its base).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Addr(pub u64);
/// Position of an entry in a layout (0-based, input order).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Slot(pub u32);
/// The only per-hit kernel result.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct KernelHit {
    pub addr: Addr,
    pub score: f32,
}
/// Existing segment: start address and length in bytes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Extent {
    pub start: u64,
    pub len: u64,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LayoutError {
    Overflow,
    TooManyEntries,
    Overlap,
    CountMismatch { expected: usize, got: usize },
    ZeroStride,
}
impl fmt::Display for LayoutError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LayoutError::Overflow => {
                write!(f, "address arithmetic overflow while building layout")
            }
            LayoutError::TooManyEntries => {
                write!(f, "more than u32::MAX slots requested")
            }
            LayoutError::Overlap => write!(f, "extents overlap"),
            LayoutError::CountMismatch { expected, got } => {
                write!(f, "entry id count mismatch: expected {expected}, got {got}")
            }
            LayoutError::ZeroStride => write!(f, "fixed layout stride must be non-zero"),
        }
    }
}
impl std::error::Error for LayoutError {}
/// Fixed layout: slot i occupies `[base + i*stride, base + (i+1)*stride)`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FixedLayout {
    base: u64,
    stride: u32,
    count: u32,
}
impl FixedLayout {
    pub fn new(base: u64, stride: u32, count: u32) -> Result<Self, LayoutError> {
        if stride == 0 {
            return Err(LayoutError::ZeroStride);
        }
        let total = u64::from(stride) * u64::from(count);
        base.checked_add(total).ok_or(LayoutError::Overflow)?;
        Ok(Self { base, stride, count })
    }
    pub fn stride(&self) -> u32 {
        self.stride
    }
    fn slot_of(&self, addr: u64) -> Option<Slot> {
        let off = addr.checked_sub(self.base)?;
        let total = u64::from(self.stride) * u64::from(self.count);
        if off >= total {
            return None;
        }
        Some(Slot((off / u64::from(self.stride)) as u32))
    }
    fn slot_range(&self, slot: Slot) -> Option<Range<u64>> {
        if slot.0 >= self.count {
            return None;
        }
        let start = self.base + u64::from(slot.0) * u64::from(self.stride);
        Some(start..start + u64::from(self.stride))
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
enum VariableRepr {
    Offsets(Vec<u64>),
    Extents { by_slot: Vec<Extent>, sorted: Vec<(u64, u64, u32)> },
}
/// Variable layout (offset table or sorted extents).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VariableLayout {
    repr: VariableRepr,
}
impl VariableLayout {
    /// Contiguous: exclusive prefix sum of the lengths on `base`; n lengths give n+1 offsets.
    pub fn from_lengths(base: u64, lengths: &[u64]) -> Result<Self, LayoutError> {
        if lengths.len() > u32::MAX as usize {
            return Err(LayoutError::TooManyEntries);
        }
        let mut offsets = Vec::with_capacity(lengths.len() + 1);
        let mut cur = base;
        offsets.push(cur);
        for &len in lengths {
            cur = cur.checked_add(len).ok_or(LayoutError::Overflow)?;
            offsets.push(cur);
        }
        Ok(Self {
            repr: VariableRepr::Offsets(offsets),
        })
    }
    /// Existing segments: slot i = extents[i]; any order, non-overlapping.
    pub fn from_extents(extents: &[Extent]) -> Result<Self, LayoutError> {
        if extents.len() > u32::MAX as usize {
            return Err(LayoutError::TooManyEntries);
        }
        let mut sorted = Vec::with_capacity(extents.len());
        for (i, e) in extents.iter().enumerate() {
            let end = e.start.checked_add(e.len).ok_or(LayoutError::Overflow)?;
            if e.len > 0 {
                sorted.push((e.start, end, i as u32));
            }
        }
        sorted.sort_unstable();
        if sorted.windows(2).any(|w| w[0].1 > w[1].0) {
            return Err(LayoutError::Overlap);
        }
        Ok(Self {
            repr: VariableRepr::Extents {
                by_slot: extents.to_vec(),
                sorted,
            },
        })
    }
    /// The n+1 absolute offsets, only for `from_lengths`.
    pub fn offsets(&self) -> Option<&[u64]> {
        match &self.repr {
            VariableRepr::Offsets(o) => Some(o),
            VariableRepr::Extents { .. } => None,
        }
    }
    fn len(&self) -> usize {
        match &self.repr {
            VariableRepr::Offsets(o) => o.len() - 1,
            VariableRepr::Extents { by_slot, .. } => by_slot.len(),
        }
    }
    fn slot_of(&self, addr: u64) -> Option<Slot> {
        match &self.repr {
            VariableRepr::Offsets(o) => {
                let n = o.len() - 1;
                if addr < o[0] || addr >= o[n] {
                    return None;
                }
                let p = o.partition_point(|&x| x <= addr);
                Some(Slot((p - 1) as u32))
            }
            VariableRepr::Extents { sorted, .. } => {
                let p = sorted.partition_point(|&(s, _, _)| s <= addr);
                let &(_, end, slot) = sorted.get(p.checked_sub(1)?)?;
                (addr < end).then_some(Slot(slot))
            }
        }
    }
    fn slot_range(&self, slot: Slot) -> Option<Range<u64>> {
        let i = slot.0 as usize;
        match &self.repr {
            VariableRepr::Offsets(o) => {
                if i + 1 >= o.len() {
                    return None;
                }
                Some(o[i]..o[i + 1])
            }
            VariableRepr::Extents { by_slot, .. } => {
                let e = by_slot.get(i)?;
                Some(e.start..e.start + e.len)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Layout {
    Fixed(FixedLayout),
    Variable(VariableLayout),
}
impl Layout {
    pub fn len(&self) -> usize {
        match self {
            Layout::Fixed(f) => f.count as usize,
            Layout::Variable(v) => v.len(),
        }
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    /// Address to slot; `None` when the address lies in no slot.
    pub fn slot_of(&self, addr: Addr) -> Option<Slot> {
        match self {
            Layout::Fixed(f) => f.slot_of(addr.0),
            Layout::Variable(v) => v.slot_of(addr.0),
        }
    }
    /// Address range of a slot (fixed: full stride width; variable: `[start, start+len)`).
    pub fn slot_range(&self, slot: Slot) -> Option<Range<u64>> {
        match self {
            Layout::Fixed(f) => f.slot_range(slot),
            Layout::Variable(v) => v.slot_range(slot),
        }
    }
    /// Attach entry ids; `ids[i]` belongs to slot i.
    pub fn into_slot_map(self, ids: Vec<EntryId>) -> Result<SlotMap, LayoutError> {
        let expected = self.len();
        if ids.len() != expected {
            return Err(LayoutError::CountMismatch {
                expected,
                got: ids.len(),
            });
        }
        Ok(SlotMap { layout: self, ids })
    }
}
/// Chooses fixed vs. variable layout from entry lengths in bytes.
/// `max_padding_permille` is the highest allowed padding share in per mille.
pub fn choose_layout(
    base: u64,
    lengths: &[u64],
    max_padding_permille: u32,
) -> Result<Layout, LayoutError> {
    let n = lengths.len();
    if n > 0 && n <= u32::MAX as usize {
        let stride = lengths.iter().copied().max().unwrap_or(0);
        if stride > 0 && stride <= u64::from(u32::MAX) {
            let sum: u128 = lengths.iter().map(|&l| u128::from(l)).sum();
            let full = u128::from(stride) * n as u128;
            let padding = full - sum;
            if padding * 1000 <= u128::from(max_padding_permille) * full {
                return Ok(
                    Layout::Fixed(FixedLayout::new(base, stride as u32, n as u32)?),
                );
            }
        }
    }
    Ok(Layout::Variable(VariableLayout::from_lengths(base, lengths)?))
}
/// Slot assignment: layout plus one entry id per slot.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SlotMap {
    layout: Layout,
    ids: Vec<EntryId>,
}
impl SlotMap {
    pub fn layout(&self) -> &Layout {
        &self.layout
    }
    pub fn entry_of_slot(&self, slot: Slot) -> Option<EntryId> {
        self.ids.get(slot.0 as usize).copied()
    }
    pub fn entry_of(&self, addr: Addr) -> Option<EntryId> {
        self.entry_of_slot(self.layout.slot_of(addr)?)
    }
    pub fn resolve(&self, hit: KernelHit) -> Option<(EntryId, f32)> {
        self.entry_of(hit.addr).map(|e| (e, hit.score))
    }
}
#[cfg(test)]
#[path = "test/kernel_layout.rs"]
mod tests;
