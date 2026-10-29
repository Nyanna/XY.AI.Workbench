//! Index build path: the kernel appends segments to a range granted up front and never allocates.
//! Embedding writes and scoring hits travel on separate rings; writes are always registered first.
use std::collections::VecDeque;
use std::ops::Range;
use std::sync::atomic::{AtomicU64, Ordering};
use parking_lot::Mutex;
use crate::resident::error::ManagerError;
use crate::resident::index::{EntryId, Index};
use crate::resident::kernel::{
    entry_of, entry_of_fixed, BatchDesc, Kernel, KernelHit, KernelOutput, RegionSpan,
    WrittenRange,
};
use crate::resident::manager::arena::IndexArena;
use crate::resident::manager::slot::{
    run_slot, AddressMap, Hit, RangeSource, UpdateQueue,
};
use crate::resident::manager::tracker::ResidencyTracker;
use crate::resident::memory::Region;
use crate::resident::segment::mapping::{EntryRef, SlotMap};
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BuildRing {
    Embedding,
    MaxSim,
}
/// Host model of the device cursor. A claim always advances the cursor; once one claim does not
/// fit, every later claim fails as well, so the valid bytes stay a contiguous prefix.
#[derive(Debug)]
pub struct WriteCursor {
    base: u64,
    cap: u64,
    next: AtomicU64,
    limit: AtomicU64,
}
impl WriteCursor {
    pub fn new(base: u64, cap: u64) -> Self {
        Self {
            base,
            cap,
            next: AtomicU64::new(0),
            limit: AtomicU64::new(u64::MAX),
        }
    }
    /// Absolute arena range, or `None` after setting the overflow state; nothing is written then.
    pub fn claim(&self, size: u64) -> Option<Range<u64>> {
        let off = self.next.fetch_add(size, Ordering::AcqRel);
        self.settle(off, size)
    }
    /// One cursor update for a whole group of claims (warp or block), in group order.
    pub fn claim_group(&self, sizes: &[u64]) -> Vec<Option<Range<u64>>> {
        let total = sizes.iter().fold(0u64, |a, s| a.saturating_add(*s));
        let mut off = self.next.fetch_add(total, Ordering::AcqRel);
        sizes
            .iter()
            .map(|&size| {
                let at = off;
                off = off.saturating_add(size);
                self.settle(at, size)
            })
            .collect()
    }
    fn settle(&self, off: u64, size: u64) -> Option<Range<u64>> {
        if off.checked_add(size).is_some_and(|end| end <= self.cap) {
            return Some(self.base + off..self.base + off + size);
        }
        if self.limit.fetch_min(off, Ordering::AcqRel) == u64::MAX {
            tracing::debug!(
                base = self.base, cap = self.cap, at = off, "write region overflow"
            );
        }
        None
    }
    pub fn overflowed(&self) -> bool {
        self.limit.load(Ordering::Acquire) != u64::MAX
    }
    /// Bytes holding successful claims.
    pub fn valid_len(&self) -> u64 {
        self.limit
            .load(Ordering::Acquire)
            .min(self.next.load(Ordering::Acquire))
            .min(self.cap)
    }
}
/// Exclusive prefix sum shifted to `base`: `n + 1` ascending offsets.
pub fn exclusive_prefix_sum(base: u64, lens: &[u64]) -> Vec<u64> {
    let mut out = Vec::with_capacity(lens.len() + 1);
    let mut at = base;
    out.push(at);
    for len in lens {
        at += len;
        out.push(at);
    }
    out
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OffsetTable {
    offsets: Vec<u64>,
    entries: Vec<EntryId>,
}
impl OffsetTable {
    pub fn new(base: u64, items: &[(EntryId, u64)]) -> Self {
        let lens: Vec<u64> = items.iter().map(|(_, len)| *len).collect();
        Self {
            offsets: exclusive_prefix_sum(base, &lens),
            entries: items.iter().map(|(e, _)| *e).collect(),
        }
    }
    pub fn offsets(&self) -> &[u64] {
        &self.offsets
    }
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
    pub fn addr_to_entry(&self, off: u64) -> Option<EntryId> {
        entry_of(&self.offsets, off).map(|i| self.entries[i])
    }
}
/// Addressing of a freshly built segment.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BuiltLayout {
    Fixed { base: u64, stride: u32, entries: Vec<EntryId> },
    Variable(OffsetTable),
}
impl BuiltLayout {
    /// Fixed stride when the padding to the largest entry stays within `max_padding_permille` of
    /// the padded size, otherwise a prefix-sum table.
    pub fn plan(base: u64, items: &[(EntryId, u64)], max_padding_permille: u32) -> Self {
        let stride = items.iter().map(|(_, len)| *len).max().unwrap_or(0);
        let used: u64 = items.iter().map(|(_, len)| *len).sum();
        let padded = stride.saturating_mul(items.len() as u64);
        let fits = u32::try_from(stride)
            .ok()
            .filter(|s| *s > 0)
            .filter(|_| {
                u128::from(padded - used) * 1000
                    <= u128::from(padded) * u128::from(max_padding_permille)
            });
        match fits {
            Some(stride) => {
                BuiltLayout::Fixed {
                    base,
                    stride,
                    entries: items.iter().map(|(e, _)| *e).collect(),
                }
            }
            None => BuiltLayout::Variable(OffsetTable::new(base, items)),
        }
    }
    pub fn addr_to_entry(&self, off: u64) -> Option<EntryId> {
        match self {
            BuiltLayout::Fixed { base, stride, entries } => {
                entry_of_fixed(*base, *stride, entries.len() as u32, off)
                    .map(|i| entries[i])
            }
            BuiltLayout::Variable(table) => table.addr_to_entry(off),
        }
    }
    /// Slot map with every slot bound to its entry, ready for `AddressMap::add_segment`.
    pub fn into_slot_map(self) -> SlotMap {
        let (map, entries) = match self {
            BuiltLayout::Fixed { base, stride, entries } => {
                (SlotMap::fixed(base, stride, entries.len() as u32), entries)
            }
            BuiltLayout::Variable(table) => {
                (SlotMap::variable(table.offsets), table.entries)
            }
        };
        for (slot, entry) in entries.into_iter().enumerate() {
            map.bind(slot as u32, EntryRef { entry, chunk: 0 })
                .expect("slot count equals entry count");
        }
        map
    }
}
/// Consumer side of the two build rings.
pub trait RingFeed {
    fn drain_writes(&self) -> Vec<WrittenRange>;
    fn drain_hits(&self) -> Vec<KernelHit>;
}
/// Single-producer, single-consumer by contract.
#[derive(Debug)]
pub struct SpscRing<T> {
    items: Mutex<VecDeque<T>>,
}
impl<T> Default for SpscRing<T> {
    fn default() -> Self {
        Self {
            items: Mutex::new(VecDeque::new()),
        }
    }
}
impl<T: Copy> SpscRing<T> {
    pub fn push(&self, items: &[T]) {
        self.items.lock().extend(items.iter().copied());
    }
    pub fn drain(&self) -> Vec<T> {
        self.items.lock().drain(..).collect()
    }
    pub fn len(&self) -> usize {
        self.items.lock().len()
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
#[derive(Debug, Default)]
pub struct BuildRings {
    embedding: SpscRing<WrittenRange>,
    maxsim: SpscRing<KernelHit>,
}
impl BuildRings {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn push_writes(&self, writes: &[WrittenRange]) {
        tracing::debug!(
            ring = ? BuildRing::Embedding, ranges = writes.len(), "ring push"
        );
        self.embedding.push(writes);
    }
    pub fn push_hits(&self, hits: &[KernelHit]) {
        tracing::debug!(ring = ? BuildRing::MaxSim, hits = hits.len(), "ring push");
        self.maxsim.push(hits);
    }
    pub fn pending(&self, ring: BuildRing) -> usize {
        match ring {
            BuildRing::Embedding => self.embedding.len(),
            BuildRing::MaxSim => self.maxsim.len(),
        }
    }
}
impl RingFeed for BuildRings {
    fn drain_writes(&self) -> Vec<WrittenRange> {
        self.embedding.drain()
    }
    fn drain_hits(&self) -> Vec<KernelHit> {
        self.maxsim.drain()
    }
}
/// A hit is only pushed after the write it reads, so draining writes first leaves no observed
/// hit without its address mapping.
pub fn run_build_slot(
    feed: &dyn RingFeed,
    source: &dyn RangeSource,
    index: &dyn Index,
    map: &mut AddressMap,
    updates: &mut UpdateQueue,
    tracker: &mut ResidencyTracker,
) -> Vec<Hit> {
    let writes = feed.drain_writes();
    let hits = feed.drain_hits();
    run_slot(&writes, &hits, source, index, map, updates, tracker)
}
#[derive(Debug, Default)]
pub struct BuildOutcome {
    pub written: Vec<WrittenRange>,
    pub regions: Vec<Region>,
    pub partials: u32,
}
/// Runs an embedding batch to completion. A `Partial` answer is not an error: the next range is
/// granted and the input is resumed after the bytes `progress` reports as consumed.
pub(crate) fn drive_build(
    kernel: &dyn Kernel,
    arena: &IndexArena<'_>,
    desc: &BatchDesc,
    input: &[RegionSpan],
    region_cap: u64,
    max_regions: usize,
) -> Result<BuildOutcome, ManagerError> {
    let batch = kernel.begin(desc)?;
    let mut out = BuildOutcome::default();
    let run = (|| -> Result<(), ManagerError> {
        loop {
            if out.regions.len() >= max_regions {
                return Err(ManagerError::ArenaExhausted {
                    requested: region_cap,
                    available: 0,
                });
            }
            let region = arena.reserve_writable(region_cap)?.region();
            out.regions.push(region);
            tracing::debug!(
                base = region.offset, cap = region.len, "write region granted"
            );
            kernel
                .grant_region(
                    batch,
                    RegionSpan {
                        offset: region.offset,
                        len: region.len,
                    },
                )?;
            let rest = skip_bytes(input, kernel.progress(batch));
            match kernel.run(batch, &rest)? {
                KernelOutput::Hits(_) => return Ok(()),
                KernelOutput::Partial => {
                    out.partials += 1;
                    tracing::debug!(
                        partials = out.partials, "region exhausted, granting next"
                    );
                }
            }
        }
    })();
    let ended = kernel.end(batch);
    match (run, ended) {
        (Ok(()), Ok(written)) => {
            out.written = written;
            Ok(out)
        }
        (run, ended) => {
            for region in out.regions {
                let _ = arena.free(region);
            }
            run?;
            ended?;
            unreachable!("one of the two results failed")
        }
    }
}
fn skip_bytes(input: &[RegionSpan], mut done: u64) -> Vec<RegionSpan> {
    let mut rest = Vec::with_capacity(input.len());
    for span in input {
        if done >= span.len {
            done -= span.len;
            continue;
        }
        rest.push(RegionSpan {
            offset: span.offset + done,
            len: span.len - done,
        });
        done = 0;
    }
    rest
}
#[cfg(feature = "cuda")]
pub mod device {
    /// Warp-aggregated cursor append: one `atomicAdd` per warp, overflow state instead of a write
    /// when a slot does not fit. `offsets_out` receives the claimed offset or `~0`.
    pub const CURSOR_APPEND: &str = r#"
extern "C" __global__ void cursor_append(unsigned long long* cursor,
                                        unsigned long long* limit,
                                        unsigned int* overflow,
                                        const unsigned long long* sizes,
                                        unsigned long long* offsets_out,
                                        unsigned int n,
                                        unsigned long long cap) {
    unsigned int i = blockIdx.x * blockDim.x + threadIdx.x;
    unsigned int lane = threadIdx.x & 31u;
    unsigned long long size = i < n ? sizes[i] : 0ull;
    unsigned long long prefix = size;
    for (int d = 1; d < 32; d <<= 1) {
        unsigned long long v = __shfl_up_sync(0xffffffffu, prefix, d);
        if (lane >= (unsigned int)d) prefix += v;
    }
    unsigned long long total = __shfl_sync(0xffffffffu, prefix, 31);
    unsigned long long base = 0ull;
    if (lane == 0 && total > 0ull) base = atomicAdd(cursor, total);
    base = __shfl_sync(0xffffffffu, base, 0);
    unsigned long long off = base + prefix - size;
    if (i < n) {
        if (off + size <= cap) {
            offsets_out[i] = off;
        } else {
            offsets_out[i] = ~0ull;
            atomicMin(limit, off);
            atomicOr(overflow, 1u);
        }
    }
    __threadfence_system();
}
"#;
}
#[cfg(test)]
#[path = "test/build.rs"]
mod tests;
