use std::fmt;
use std::sync::Arc;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::a::manifest::{
    Manifest, ManifestGeneration, SegmentSpec,
};
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::a::segments::SegmentId;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::a::slot_layout::{
    EntryRef, SlotAssignment, SlotStates,
};
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::a::kernel_layout::{
    Addr, AddrLayout, FixedLayout, KernelHit, Slot,
};
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::a::region_handle::Generation;
/// Slot assignment and valid bits of one segment.
#[derive(Clone)]
pub struct SegmentBinding {
    pub assignment: Arc<SlotAssignment>,
    pub states: Arc<SlotStates>,
}
/// Implemented by the manager; must not block.
pub trait SegmentDirectory: Send + Sync {
    /// `None` if nothing is known for the id.
    fn binding(&self, id: SegmentId) -> Option<SegmentBinding>;
}
/// Hit converted to segment generation, slot and the unchanged score.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HitLocation {
    pub generation: Generation,
    pub slot: Slot,
    pub score: f32,
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ResolvedHit {
    pub entry: EntryRef,
    pub score: f32,
    pub stale: bool,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HitResolveError {
    SlotOutOfRange,
    StaleGeneration,
    MissingMapping { generation: Generation, slot: Slot },
}
impl fmt::Display for HitResolveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HitResolveError::SlotOutOfRange => write!(f, "slot out of range"),
            HitResolveError::StaleGeneration => write!(f, "stale generation"),
            HitResolveError::MissingMapping { generation, slot } => {
                write!(
                    f, "missing mapping (generation {}, slot {})", generation.0, slot.0
                )
            }
        }
    }
}
impl std::error::Error for HitResolveError {}
/// Hits in input order plus the number of dropped hits.
#[derive(Debug, Clone, PartialEq)]
pub struct Resolution {
    pub hits: Vec<ResolvedHit>,
    pub discarded: usize,
}
static_assertions::assert_impl_all!(SegmentBinding : Send, Sync);
static_assertions::assert_impl_all!(HitLocation : Send, Sync);
static_assertions::assert_impl_all!(ResolvedHit : Send, Sync);
static_assertions::assert_impl_all!(HitResolveError : Send, Sync);
static_assertions::assert_impl_all!(Resolution : Send, Sync);
fn fixed_slot(spec: &SegmentSpec, addr: u64) -> Option<Slot> {
    let layout = FixedLayout::new(spec.start().0, spec.slot_stride(), spec.slot_count())
        .ok()?;
    AddrLayout::Fixed(layout).slot_of(Addr(addr))
}
fn variable_slot(
    spec: &SegmentSpec,
    assignment: &SlotAssignment,
    addr: u64,
) -> Option<Slot> {
    let slot = assignment.slot_of(Addr(addr))?;
    (slot.0 < spec.slot_count().min(assignment.slot_count())).then_some(slot)
}
/// Converts a kernel hit into segment generation, slot and score, relative to
/// the manifest whose table the kernel read.
pub fn locate(
    manifest: &Manifest,
    dir: &dyn SegmentDirectory,
    hit: KernelHit,
) -> Result<HitLocation, HitResolveError> {
    let addr = hit.addr.0;
    for spec in manifest.segments() {
        let slot = if spec.slot_stride() > 0 {
            fixed_slot(spec, addr)
        } else {
            match dir.binding(spec.id()) {
                Some(binding) => variable_slot(spec, &binding.assignment, addr),
                None => None,
            }
        };
        if let Some(slot) = slot {
            return Ok(HitLocation {
                generation: spec.generation(),
                slot,
                score: hit.score,
            });
        }
    }
    Err(HitResolveError::SlotOutOfRange)
}
fn resolve_with_binding(
    manifest: &Manifest,
    dir: &dyn SegmentDirectory,
    generation: Generation,
    slot: Slot,
) -> Result<(EntryRef, SegmentBinding), HitResolveError> {
    let spec = manifest
        .segments()
        .iter()
        .find(|s| s.generation() == generation)
        .ok_or(HitResolveError::StaleGeneration)?;
    let binding = dir.binding(spec.id()).ok_or(HitResolveError::StaleGeneration)?;
    match binding.assignment.lookup(slot) {
        Some(entry) => Ok((entry, binding)),
        None => {
            Err(HitResolveError::MissingMapping {
                generation,
                slot,
            })
        }
    }
}
/// Segment generation and slot to the entry reference within `manifest`.
pub fn resolve_location(
    manifest: &Manifest,
    dir: &dyn SegmentDirectory,
    generation: Generation,
    slot: Slot,
) -> Result<EntryRef, HitResolveError> {
    resolve_with_binding(manifest, dir, generation, slot).map(|(entry, _)| entry)
}
/// Newest valid location of the entry within `manifest`.
pub fn find_current_location(
    manifest: &Manifest,
    dir: &dyn SegmentDirectory,
    entry: EntryRef,
) -> Option<(Generation, Slot)> {
    let specs = manifest.segments();
    let mut order: Vec<usize> = (0..specs.len()).collect();
    order.sort_by_key(|&i| std::cmp::Reverse((specs[i].generation().0, i)));
    order
        .into_iter()
        .find_map(|i| {
            let spec = &specs[i];
            let binding = dir.binding(spec.id())?;
            let slot = binding.assignment.slot_of_entry(entry)?;
            binding.states.is_valid(slot).then_some((spec.generation(), slot))
        })
}
fn transfer_one(
    current: &Manifest,
    previous: Option<&Manifest>,
    dir: &dyn SegmentDirectory,
    location: &HitLocation,
    stale: bool,
) -> Option<ResolvedHit> {
    let HitLocation { generation, slot, score } = *location;
    let (entry, binding) = resolve_with_binding(current, dir, generation, slot)
        .or_else(|_| match previous {
            Some(prev) => resolve_with_binding(prev, dir, generation, slot),
            None => Err(HitResolveError::StaleGeneration),
        })
        .ok()?;
    if binding.states.is_valid(slot)
        || find_current_location(current, dir, entry).is_some()
    {
        Some(ResolvedHit { entry, score, stale })
    } else {
        None
    }
}
/// Carries hits of a run on manifest generation `hit_generation` over to `current`.
pub fn transfer(
    current: &Manifest,
    previous: Option<&Manifest>,
    dir: &dyn SegmentDirectory,
    hit_generation: ManifestGeneration,
    locations: &[HitLocation],
) -> Resolution {
    let stale = hit_generation < current.generation();
    let hits: Vec<ResolvedHit> = locations
        .iter()
        .filter_map(|loc| transfer_one(current, previous, dir, loc, stale))
        .collect();
    Resolution {
        discarded: locations.len() - hits.len(),
        hits,
    }
}
/// `locate` on `hit_manifest`, then `transfer` with its generation.
pub fn resolve_hits(
    hit_manifest: &Manifest,
    current: &Manifest,
    previous: Option<&Manifest>,
    dir: &dyn SegmentDirectory,
    hits: &[KernelHit],
) -> Resolution {
    let locations: Vec<HitLocation> = hits
        .iter()
        .filter_map(|&hit| locate(hit_manifest, dir, hit).ok())
        .collect();
    let rejected = hits.len() - locations.len();
    let mut resolution = transfer(
        current,
        previous,
        dir,
        hit_manifest.generation(),
        &locations,
    );
    resolution.discarded += rejected;
    resolution
}
#[cfg(test)]
#[path = "test/hit_resolution.rs"]
mod tests;
