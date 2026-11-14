use std::collections::HashMap;
use std::fmt;
use std::ops::Range;
use std::sync::atomic::{AtomicU32, AtomicU64, Ordering};
use std::sync::RwLock;

use super::super::l030::a::kernel_layout::{
    Addr, FixedLayout, Layout as AddrLayout, LayoutError as AddrLayoutError, Slot, VariableLayout,
};
use super::super::l030::a::region_handle::{Generation, RegionHandle};
use super::super::l030::l020::a::index_contract::EntryId;
use super::super::l030::l020::l010::layout::{DType, MASK_VALUE};

const MAX_CLASS_TOKENS: u32 = 65_535;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SlotError {
    InvalidClass { max_tokens: u32 },
    InvalidDimension,
    SlotSizeOverflow,
    RegionTooSmall { needed: u64, available: u64 },
    SlotOutOfRange { slot: u32, count: u32 },
    UnsupportedDType { dtype: DType },
    TooManyTokens { tokens: usize, max: usize },
    ValueCountMismatch { expected: usize, got: usize },
    TokensExceedClass { tokens: u32, max: u32 },
    SlotOccupied { slot: u32, entry: EntryRef },
    EntryAlreadyBound { entry: EntryRef, slot: u32 },
    Addr(AddrLayoutError),
}

impl fmt::Display for SlotError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SlotError::InvalidClass { max_tokens } => {
                write!(f, "invalid slot class: {max_tokens} tokens (allowed 1..={MAX_CLASS_TOKENS})")
            }
            SlotError::InvalidDimension => write!(f, "embedding dimension must be non-zero"),
            SlotError::SlotSizeOverflow => write!(f, "slot size does not fit into u32"),
            SlotError::RegionTooSmall { needed, available } => {
                write!(f, "region too small: need {needed} bytes, have {available}")
            }
            SlotError::SlotOutOfRange { slot, count } => {
                write!(f, "slot {slot} out of range (count {count})")
            }
            SlotError::UnsupportedDType { dtype } => {
                write!(f, "unsupported dtype for slot encoding: {dtype:?}")
            }
            SlotError::TooManyTokens { tokens, max } => {
                write!(f, "too many tokens: {tokens} (max {max})")
            }
            SlotError::ValueCountMismatch { expected, got } => {
                write!(f, "value count mismatch: expected {expected}, got {got}")
            }
            SlotError::TokensExceedClass { tokens, max } => {
                write!(f, "token count {tokens} exceeds class maximum {max}")
            }
            SlotError::SlotOccupied { slot, entry } => write!(
                f,
                "slot {slot} already bound to entry {} section {}",
                entry.entry.0, entry.section
            ),
            SlotError::EntryAlreadyBound { entry, slot } => write!(
                f,
                "entry {} section {} already bound to slot {slot}",
                entry.entry.0, entry.section
            ),
            SlotError::Addr(e) => write!(f, "address layout error: {e}"),
        }
    }
}

impl std::error::Error for SlotError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            SlotError::Addr(e) => Some(e),
            _ => None,
        }
    }
}

impl From<AddrLayoutError> for SlotError {
    fn from(e: AddrLayoutError) -> Self {
        SlotError::Addr(e)
    }
}

/// Upper bound of tokens per slot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SlotClass(u32);

impl SlotClass {
    pub const C32: SlotClass = SlotClass(32);
    pub const C64: SlotClass = SlotClass(64);
    pub const C128: SlotClass = SlotClass(128);

    pub fn custom(max_tokens: u32) -> Result<SlotClass, SlotError> {
        if max_tokens == 0 || max_tokens > MAX_CLASS_TOKENS {
            return Err(SlotError::InvalidClass { max_tokens });
        }
        Ok(SlotClass(max_tokens))
    }

    pub fn max_tokens(self) -> u32 {
        self.0
    }
}

/// Slot size in bytes: `max_tokens * dim * dtype.size_in_bytes()`, must fit into `u32`.
pub fn slot_size(class: SlotClass, dim: u32, dtype: DType) -> Result<u32, SlotError> {
    if dim == 0 {
        return Err(SlotError::InvalidDimension);
    }
    let bytes = u64::from(class.0)
        .checked_mul(u64::from(dim))
        .and_then(|v| v.checked_mul(dtype.size_in_bytes() as u64))
        .ok_or(SlotError::SlotSizeOverflow)?;
    u32::try_from(bytes).map_err(|_| SlotError::SlotSizeOverflow)
}

/// Encodes a full slot: token rows in input order, then mask-valued padding rows.
pub fn encode_slot(
    class: SlotClass,
    dim: u32,
    dtype: DType,
    tokens: usize,
    values: &[f32],
) -> Result<Vec<u8>, SlotError> {
    let size = slot_size(class, dim, dtype)?;
    if !matches!(dtype, DType::F32 | DType::F16 | DType::BF16) {
        return Err(SlotError::UnsupportedDType { dtype });
    }
    let max = class.max_tokens() as usize;
    if tokens > max {
        return Err(SlotError::TooManyTokens { tokens, max });
    }
    let dim = dim as usize;
    let expected = tokens * dim;
    if values.len() != expected {
        return Err(SlotError::ValueCountMismatch {
            expected,
            got: values.len(),
        });
    }

    let mut out = Vec::with_capacity(size as usize);
    for &v in values {
        push_element(&mut out, dtype, v);
    }
    for _ in 0..(max - tokens) * dim {
        push_element(&mut out, dtype, MASK_VALUE);
    }
    debug_assert_eq!(out.len(), size as usize);
    Ok(out)
}

fn push_element(out: &mut Vec<u8>, dtype: DType, value: f32) {
    match dtype {
        DType::F32 => out.extend_from_slice(&value.to_le_bytes()),
        DType::F16 => out.extend_from_slice(&f32_to_f16_bits(value).to_le_bytes()),
        DType::BF16 => out.extend_from_slice(&f32_to_bf16_bits(value).to_le_bytes()),
        _ => unreachable!("dtype validated by caller"),
    }
}

fn f32_to_f16_bits(value: f32) -> u16 {
    let bits = value.to_bits();
    let sign = ((bits >> 16) & 0x8000) as u16;
    let exp = ((bits >> 23) & 0xff) as i32;
    let man = bits & 0x007f_ffff;

    if exp == 0xff {
        if man == 0 {
            return sign | 0x7c00;
        }
        return sign | 0x7e00 | (man >> 13) as u16;
    }

    let e = exp - 127 + 15;
    if e >= 0x1f {
        return sign | 0x7c00;
    }
    if e <= 0 {
        if e < -10 {
            return sign;
        }
        let m = man | 0x0080_0000;
        let shift = (14 - e) as u32;
        let mut half = m >> shift;
        let rem = m & ((1u32 << shift) - 1);
        let halfway = 1u32 << (shift - 1);
        if rem > halfway || (rem == halfway && (half & 1) == 1) {
            half += 1;
        }
        return sign | half as u16;
    }

    let mut half = ((e as u32) << 10) | (man >> 13);
    let rem = man & 0x1fff;
    if rem > 0x1000 || (rem == 0x1000 && (half & 1) == 1) {
        half += 1;
    }
    sign | half as u16
}

fn f32_to_bf16_bits(value: f32) -> u16 {
    let bits = value.to_bits();
    if value.is_nan() {
        return ((bits >> 16) as u16) | 0x0040;
    }
    let rounding = 0x7fff + ((bits >> 16) & 1);
    (bits.wrapping_add(rounding) >> 16) as u16
}

/// Kernel descriptor: start address, slot count and stride in bytes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Descriptor {
    pub start: Addr,
    pub slot_count: u32,
    pub stride: u32,
}

impl Descriptor {
    pub fn to_fixed_layout(&self) -> Result<FixedLayout, AddrLayoutError> {
        FixedLayout::new(self.start.0, self.stride, self.slot_count)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SegmentView {
    region: RegionHandle,
    slot_count: u32,
    class: SlotClass,
    dim: u32,
    dtype: DType,
    generation: Generation,
    slot_bytes: u32,
}

impl SegmentView {
    pub fn new(
        region: RegionHandle,
        slot_count: u32,
        class: SlotClass,
        dim: u32,
        dtype: DType,
        generation: Generation,
    ) -> Result<SegmentView, SlotError> {
        let slot_bytes = slot_size(class, dim, dtype)?;
        let needed = u64::from(slot_count) * u64::from(slot_bytes);
        let available = region.len();
        if needed > available {
            return Err(SlotError::RegionTooSmall { needed, available });
        }
        Ok(SegmentView {
            region,
            slot_count,
            class,
            dim,
            dtype,
            generation,
            slot_bytes,
        })
    }

    pub fn region(&self) -> RegionHandle {
        self.region
    }

    pub fn slot_count(&self) -> u32 {
        self.slot_count
    }

    pub fn class(&self) -> SlotClass {
        self.class
    }

    pub fn dim(&self) -> u32 {
        self.dim
    }

    pub fn dtype(&self) -> DType {
        self.dtype
    }

    /// Segment generation; independent of the region's own generation.
    pub fn generation(&self) -> Generation {
        self.generation
    }

    /// Slot stride in bytes.
    pub fn slot_bytes(&self) -> u32 {
        self.slot_bytes
    }

    pub fn total_bytes(&self) -> u64 {
        u64::from(self.slot_count) * u64::from(self.slot_bytes)
    }

    /// Byte range of a slot relative to the region start.
    pub fn slot_range(&self, slot: Slot) -> Result<Range<u64>, SlotError> {
        if slot.0 >= self.slot_count {
            return Err(SlotError::SlotOutOfRange {
                slot: slot.0,
                count: self.slot_count,
            });
        }
        let stride = u64::from(self.slot_bytes);
        let start = u64::from(slot.0) * stride;
        Ok(start..start + stride)
    }

    pub fn descriptor(&self, device_base: Addr) -> Result<Descriptor, SlotError> {
        device_base
            .0
            .checked_add(self.total_bytes())
            .ok_or(SlotError::Addr(AddrLayoutError::Overflow))?;
        Ok(Descriptor {
            start: device_base,
            slot_count: self.slot_count,
            stride: self.slot_bytes,
        })
    }

    pub fn encode_slot(&self, tokens: usize, values: &[f32]) -> Result<Vec<u8>, SlotError> {
        encode_slot(self.class, self.dim, self.dtype, tokens, values)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SlotMeta {
    pub valid: bool,
    pub tokens: u32,
}

/// Valid bits and token counts of one segment.
pub struct SlotStates {
    words: Vec<AtomicU64>,
    tokens: Vec<AtomicU32>,
    slot_count: u32,
    max_tokens: u32,
}

impl SlotStates {
    pub fn new(view: &SegmentView) -> SlotStates {
        let slot_count = view.slot_count();
        let word_count = (slot_count as usize).div_ceil(64);
        SlotStates {
            words: (0..word_count).map(|_| AtomicU64::new(0)).collect(),
            tokens: (0..slot_count).map(|_| AtomicU32::new(0)).collect(),
            slot_count,
            max_tokens: view.class().max_tokens(),
        }
    }

    pub fn slot_count(&self) -> u32 {
        self.slot_count
    }

    fn check(&self, slot: Slot) -> Result<usize, SlotError> {
        if slot.0 >= self.slot_count {
            return Err(SlotError::SlotOutOfRange {
                slot: slot.0,
                count: self.slot_count,
            });
        }
        Ok(slot.0 as usize)
    }

    pub fn set_filled(&self, slot: Slot, tokens: u32) -> Result<(), SlotError> {
        let idx = self.check(slot)?;
        if tokens > self.max_tokens {
            return Err(SlotError::TokensExceedClass {
                tokens,
                max: self.max_tokens,
            });
        }
        self.tokens[idx].store(tokens, Ordering::Relaxed);
        self.words[idx / 64].fetch_or(1u64 << (idx % 64), Ordering::Release);
        Ok(())
    }

    /// Tombstone: clears the valid bit and the token count; idempotent.
    pub fn invalidate(&self, slot: Slot) -> Result<(), SlotError> {
        let idx = self.check(slot)?;
        self.words[idx / 64].fetch_and(!(1u64 << (idx % 64)), Ordering::AcqRel);
        self.tokens[idx].store(0, Ordering::Release);
        Ok(())
    }

    pub fn is_valid(&self, slot: Slot) -> bool {
        if slot.0 >= self.slot_count {
            return false;
        }
        let idx = slot.0 as usize;
        self.words[idx / 64].load(Ordering::Acquire) & (1u64 << (idx % 64)) != 0
    }

    pub fn valid_count(&self) -> u32 {
        self.words
            .iter()
            .map(|w| w.load(Ordering::Acquire).count_ones())
            .sum()
    }

    pub fn meta(&self, slot: Slot) -> Result<SlotMeta, SlotError> {
        let idx = self.check(slot)?;
        let valid = self.words[idx / 64].load(Ordering::Acquire) & (1u64 << (idx % 64)) != 0;
        let tokens = if valid {
            self.tokens[idx].load(Ordering::Relaxed)
        } else {
            0
        };
        Ok(SlotMeta { valid, tokens })
    }

    /// Per-word atomic copy of the valid bits; slot `i` is bit `i % 64` of word `i / 64`.
    pub fn snapshot_words(&self) -> Vec<u64> {
        self.words
            .iter()
            .map(|w| w.load(Ordering::Acquire))
            .collect()
    }
}

/// Entry id plus section number.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EntryRef {
    pub entry: EntryId,
    pub section: u32,
}

#[derive(Default)]
struct Bindings {
    forward: HashMap<u32, EntryRef>,
    backward: HashMap<EntryRef, u32>,
}

/// Concurrent mapping slot <-> entry reference.
pub struct SlotAssignment {
    bindings: RwLock<Bindings>,
    layout: AddrLayout,
    region: RegionHandle,
    device_base: Addr,
    slot_count: u32,
}

impl SlotAssignment {
    pub fn fixed(view: &SegmentView, device_base: Addr) -> Result<SlotAssignment, SlotError> {
        let layout = FixedLayout::new(device_base.0, view.slot_bytes(), view.slot_count())?;
        Ok(SlotAssignment {
            bindings: RwLock::new(Bindings::default()),
            layout: AddrLayout::Fixed(layout),
            region: view.region(),
            device_base,
            slot_count: view.slot_count(),
        })
    }

    /// Slot `i` has `lengths[i]` bytes, contiguous from the region start.
    pub fn variable(
        region: RegionHandle,
        device_base: Addr,
        lengths: &[u64],
    ) -> Result<SlotAssignment, SlotError> {
        let layout = VariableLayout::from_lengths(device_base.0, lengths)?;
        let needed = lengths
            .iter()
            .try_fold(0u64, |acc, &l| acc.checked_add(l))
            .ok_or(SlotError::Addr(AddrLayoutError::Overflow))?;
        if needed > region.len() {
            return Err(SlotError::RegionTooSmall {
                needed,
                available: region.len(),
            });
        }
        Ok(SlotAssignment {
            bindings: RwLock::new(Bindings::default()),
            layout: AddrLayout::Variable(layout),
            region,
            device_base,
            slot_count: lengths.len() as u32,
        })
    }

    pub fn slot_count(&self) -> u32 {
        self.slot_count
    }

    fn read(&self) -> std::sync::RwLockReadGuard<'_, Bindings> {
        self.bindings.read().unwrap_or_else(|e| e.into_inner())
    }

    fn write(&self) -> std::sync::RwLockWriteGuard<'_, Bindings> {
        self.bindings.write().unwrap_or_else(|e| e.into_inner())
    }

    fn out_of_range(&self, slot: Slot) -> SlotError {
        SlotError::SlotOutOfRange {
            slot: slot.0,
            count: self.slot_count,
        }
    }

    pub fn bind(&self, slot: Slot, entry: EntryRef) -> Result<(), SlotError> {
        if slot.0 >= self.slot_count {
            return Err(self.out_of_range(slot));
        }
        let mut guard = self.write();
        if let Some(&current) = guard.forward.get(&slot.0) {
            return Err(SlotError::SlotOccupied {
                slot: slot.0,
                entry: current,
            });
        }
        if let Some(&existing) = guard.backward.get(&entry) {
            return Err(SlotError::EntryAlreadyBound {
                entry,
                slot: existing,
            });
        }
        guard.forward.insert(slot.0, entry);
        guard.backward.insert(entry, slot.0);
        Ok(())
    }

    pub fn unbind(&self, slot: Slot) -> Result<Option<EntryRef>, SlotError> {
        if slot.0 >= self.slot_count {
            return Err(self.out_of_range(slot));
        }
        let mut guard = self.write();
        let previous = guard.forward.remove(&slot.0);
        if let Some(entry) = previous {
            guard.backward.remove(&entry);
        }
        Ok(previous)
    }

    pub fn lookup(&self, slot: Slot) -> Option<EntryRef> {
        self.read().forward.get(&slot.0).copied()
    }

    pub fn slot_of_entry(&self, entry: EntryRef) -> Option<Slot> {
        self.read().backward.get(&entry).copied().map(Slot)
    }

    fn addr_range(&self, slot: Slot) -> Result<Range<u64>, SlotError> {
        self.layout
            .slot_range(slot)
            .ok_or_else(|| self.out_of_range(slot))
    }

    pub fn device_addr(&self, slot: Slot) -> Result<Addr, SlotError> {
        Ok(Addr(self.addr_range(slot)?.start))
    }

    /// Absolute byte range in the arena offset space.
    pub fn byte_range(&self, slot: Slot) -> Result<Range<u64>, SlotError> {
        let r = self.addr_range(slot)?;
        self.to_arena_range(r)
    }

    fn to_arena_range(&self, r: Range<u64>) -> Result<Range<u64>, SlotError> {
        let overflow = || SlotError::Addr(AddrLayoutError::Overflow);
        let origin = self.region.offset();
        let base = self.device_base.0;
        let start = r
            .start
            .checked_sub(base)
            .and_then(|d| origin.checked_add(d))
            .ok_or_else(overflow)?;
        let end = r
            .end
            .checked_sub(base)
            .and_then(|d| origin.checked_add(d))
            .ok_or_else(overflow)?;
        Ok(start..end)
    }

    /// Smallest range covering all bound slots; `None` when nothing is bound.
    pub fn spanning_range(&self) -> Option<Range<u64>> {
        let guard = self.read();
        let mut span: Option<Range<u64>> = None;
        for &slot in guard.forward.keys() {
            let Ok(r) = self.byte_range(Slot(slot)) else {
                continue;
            };
            span = Some(match span {
                None => r,
                Some(s) => s.start.min(r.start)..s.end.max(r.end),
            });
        }
        span
    }
}

#[cfg(test)]
#[path = "test/slot_layout.rs"]
mod tests;
