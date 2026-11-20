use super::*;
use std::collections::HashMap;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::a::manifest::{
    ManifestStore, StoreConfig,
};
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::a::slot_layout::{
    SegmentView, SlotClass,
};
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::a::kernel_layout::Addr;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::a::region_handle::RegionHandle;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::a::arenas::ArenaKind;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::a::index_contract::EntryId;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::l010::layout::DType;
const SLOT_BYTES: u64 = 512;
#[derive(Default)]
struct Dir(HashMap<SegmentId, SegmentBinding>);
impl SegmentDirectory for Dir {
    fn binding(&self, id: SegmentId) -> Option<SegmentBinding> {
        self.0.get(&id).cloned()
    }
}
fn region(len: u64) -> RegionHandle {
    RegionHandle::new(ArenaKind::IndexSegments, 0, len, Generation(0)).unwrap()
}
fn entry(n: u64) -> EntryRef {
    EntryRef {
        entry: EntryId(n),
        section: 0,
    }
}
fn hit(addr: u64, score: f32) -> KernelHit {
    KernelHit {
        addr: Addr(addr),
        score,
    }
}
fn fixed(dir: &mut Dir, id: u64, generation: u64, base: u64, slots: u32) -> SegmentSpec {
    let view = SegmentView::new(
            region(u64::from(slots) * SLOT_BYTES),
            slots,
            SlotClass::C32,
            4,
            DType::F32,
            Generation(generation),
        )
        .unwrap();
    let spec = SegmentSpec::from_view(SegmentId(id), &view, Addr(base)).unwrap();
    dir.0
        .insert(
            SegmentId(id),
            SegmentBinding {
                assignment: Arc::new(SlotAssignment::fixed(&view, Addr(base)).unwrap()),
                states: Arc::new(SlotStates::new(&view)),
            },
        );
    spec
}
fn variable(
    dir: &mut Dir,
    id: u64,
    generation: u64,
    base: u64,
    lengths: &[u64],
    region_len: u64,
) -> SegmentSpec {
    let reg = region(region_len);
    let count = lengths.len() as u32;
    let view = SegmentView::new(
            region(u64::from(count) * 128),
            count,
            SlotClass::C32,
            1,
            DType::F32,
            Generation(generation),
        )
        .unwrap();
    dir.0
        .insert(
            SegmentId(id),
            SegmentBinding {
                assignment: Arc::new(
                    SlotAssignment::variable(reg, Addr(base), lengths).unwrap(),
                ),
                states: Arc::new(SlotStates::new(&view)),
            },
        );
    SegmentSpec::variable(SegmentId(id), Generation(generation), reg, Addr(base), count)
}
fn bind(dir: &Dir, id: u64, slot: u32, e: EntryRef, valid: bool) {
    let binding = &dir.0[&SegmentId(id)];
    binding.assignment.bind(Slot(slot), e).unwrap();
    if valid {
        binding.states.set_filled(Slot(slot), 1).unwrap();
    }
}
fn invalidate(dir: &Dir, id: u64, slot: u32) {
    dir.0[&SegmentId(id)].states.invalidate(Slot(slot)).unwrap();
}
fn chain(specs: Vec<Vec<SegmentSpec>>) -> Vec<Manifest> {
    let store = ManifestStore::new(StoreConfig::default());
    let mut out: Vec<Manifest> = Vec::new();
    for next_specs in specs {
        let next = match out.last() {
            Some(base) => store.successor(base, next_specs),
            None => store.successor(&store.current(), next_specs),
        }
            .unwrap();
        out.push(next);
    }
    out
}
fn single(specs: Vec<SegmentSpec>) -> Manifest {
    chain(vec![specs]).remove(0)
}
#[test]
fn locate_fixed_segment() {
    let mut dir = Dir::default();
    let m = single(vec![fixed(& mut dir, 1, 4, 0x10000, 4)]);
    let loc = locate(&m, &dir, hit(0x10000 + 2 * SLOT_BYTES + 5, 0.5)).unwrap();
    assert_eq!(
        loc, HitLocation { generation : Generation(4), slot : Slot(2), score : 0.5 }
    );
    assert_eq!(locate(& m, & dir, hit(0x10000, 1.0)).unwrap().slot, Slot(0));
    assert_eq!(
        locate(& m, & dir, hit(0x10000 + 4 * SLOT_BYTES - 1, 1.0)).unwrap().slot, Slot(3)
    );
    assert_eq!(
        locate(& m, & dir, hit(0x10000 + 4 * SLOT_BYTES, 1.0)),
        Err(ResolveError::SlotOutOfRange)
    );
    assert_eq!(locate(& m, & dir, hit(0xFFFF, 1.0)), Err(ResolveError::SlotOutOfRange));
}
#[test]
fn locate_keeps_score_bits() {
    let mut dir = Dir::default();
    let m = single(vec![fixed(& mut dir, 1, 1, 0x10000, 2)]);
    for bits in [0x7fc0_1234u32, 0xff80_0000, 0x8000_0000] {
        let loc = locate(&m, &dir, hit(0x10000, f32::from_bits(bits))).unwrap();
        assert_eq!(loc.score.to_bits(), bits);
    }
}
#[test]
fn locate_empty_manifest_and_zero_slots() {
    let dir = Dir::default();
    let empty = single(vec![]);
    assert_eq!(locate(& empty, & dir, hit(0, 1.0)), Err(ResolveError::SlotOutOfRange));
    let mut dir = Dir::default();
    let m = single(vec![variable(& mut dir, 1, 1, 0x2000, & [], 0)]);
    assert_eq!(locate(& m, & dir, hit(0x2000, 1.0)), Err(ResolveError::SlotOutOfRange));
}
#[test]
fn locate_variable_segment() {
    let mut dir = Dir::default();
    let m = single(vec![variable(& mut dir, 1, 2, 0x2000, & [100, 0, 50, 200], 400)]);
    let slot_at = |offset: u64| {
        locate(&m, &dir, hit(0x2000 + offset, 0.25)).map(|l| l.slot)
    };
    assert_eq!(slot_at(0), Ok(Slot(0)));
    assert_eq!(slot_at(99), Ok(Slot(0)));
    assert_eq!(slot_at(100), Ok(Slot(2)));
    assert_eq!(slot_at(149), Ok(Slot(2)));
    assert_eq!(slot_at(150), Ok(Slot(3)));
    assert_eq!(slot_at(349), Ok(Slot(3)));
    assert_eq!(slot_at(350), Err(ResolveError::SlotOutOfRange));
    assert_eq!(slot_at(399), Err(ResolveError::SlotOutOfRange));
    assert_eq!(slot_at(400), Err(ResolveError::SlotOutOfRange));
    assert_eq!(locate(& m, & dir, hit(0x1FFF, 0.0)), Err(ResolveError::SlotOutOfRange));
}
#[test]
fn locate_skips_variable_segment_without_binding() {
    let mut dir = Dir::default();
    let var = variable(&mut dir, 1, 1, 0x2000, &[100], 100);
    let fix = fixed(&mut dir, 2, 2, 0x10000, 2);
    dir.0.remove(&SegmentId(1));
    let m = single(vec![var, fix]);
    assert_eq!(locate(& m, & dir, hit(0x2000, 1.0)), Err(ResolveError::SlotOutOfRange));
    let loc = locate(&m, &dir, hit(0x10000 + SLOT_BYTES, 1.0)).unwrap();
    assert_eq!((loc.generation, loc.slot), (Generation(2), Slot(1)));
}
#[test]
fn locate_near_address_limit() {
    let mut dir = Dir::default();
    let base = u64::MAX - 1000;
    let m = single(vec![variable(& mut dir, 1, 1, base, & [500], 500)]);
    assert_eq!(
        locate(& m, & dir, hit(u64::MAX, 1.0)), Err(ResolveError::SlotOutOfRange)
    );
    assert_eq!(locate(& m, & dir, hit(base + 499, 1.0)).unwrap().slot, Slot(0));
    assert_eq!(
        locate(& m, & dir, hit(base + 500, 1.0)), Err(ResolveError::SlotOutOfRange)
    );
}
#[test]
fn resolve_location_cases() {
    let mut dir = Dir::default();
    let m = single(vec![fixed(& mut dir, 1, 3, 0x10000, 4)]);
    bind(&dir, 1, 1, entry(7), true);
    assert_eq!(resolve_location(& m, & dir, Generation(3), Slot(1)), Ok(entry(7)));
    assert_eq!(
        resolve_location(& m, & dir, Generation(3), Slot(0)),
        Err(ResolveError::MissingMapping { generation : Generation(3), slot : Slot(0) })
    );
    assert_eq!(
        resolve_location(& m, & dir, Generation(3), Slot(99)),
        Err(ResolveError::MissingMapping { generation : Generation(3), slot : Slot(99) })
    );
    assert_eq!(
        resolve_location(& m, & dir, Generation(9), Slot(1)),
        Err(ResolveError::StaleGeneration)
    );
    let no_dir = Dir::default();
    assert_eq!(
        resolve_location(& m, & no_dir, Generation(3), Slot(1)),
        Err(ResolveError::StaleGeneration)
    );
}
#[test]
fn resolve_location_picks_first_segment_of_generation() {
    let mut dir = Dir::default();
    let first = fixed(&mut dir, 1, 3, 0x10000, 2);
    let second = fixed(&mut dir, 2, 3, 0x20000, 2);
    bind(&dir, 1, 0, entry(1), true);
    bind(&dir, 2, 0, entry(2), true);
    let m = single(vec![first, second]);
    assert_eq!(resolve_location(& m, & dir, Generation(3), Slot(0)), Ok(entry(1)));
}
#[test]
fn error_display() {
    assert_eq!(ResolveError::SlotOutOfRange.to_string(), "Slot außerhalb des Bereichs");
    assert_eq!(ResolveError::StaleGeneration.to_string(), "stale Generation");
    let text = ResolveError::MissingMapping {
        generation: Generation(31),
        slot: Slot(17),
    }
        .to_string();
    assert!(text.contains("fehlende Zuordnung"));
    assert!(text.contains("31"));
    assert!(text.contains("17"));
}
#[test]
fn current_location_prefers_newest_valid() {
    let mut dir = Dir::default();
    let old = fixed(&mut dir, 1, 1, 0x10000, 4);
    let new = fixed(&mut dir, 2, 2, 0x20000, 4);
    bind(&dir, 1, 0, entry(1), true);
    bind(&dir, 2, 2, entry(1), true);
    let m = single(vec![new, old]);
    assert_eq!(
        find_current_location(& m, & dir, entry(1)), Some((Generation(2), Slot(2)))
    );
    invalidate(&dir, 2, 2);
    assert_eq!(
        find_current_location(& m, & dir, entry(1)), Some((Generation(1), Slot(0)))
    );
    invalidate(&dir, 1, 0);
    assert_eq!(find_current_location(& m, & dir, entry(1)), None);
    assert_eq!(find_current_location(& m, & dir, entry(5)), None);
}
#[test]
fn current_location_ties_and_missing_bindings() {
    let mut dir = Dir::default();
    let a = fixed(&mut dir, 1, 5, 0x10000, 2);
    let b = fixed(&mut dir, 2, 5, 0x20000, 2);
    let newer = fixed(&mut dir, 3, 9, 0x30000, 2);
    bind(&dir, 1, 0, entry(1), true);
    bind(&dir, 2, 1, entry(1), true);
    bind(&dir, 3, 0, entry(1), true);
    let m = single(vec![a, b, newer]);
    assert_eq!(
        find_current_location(& m, & dir, entry(1)), Some((Generation(9), Slot(0)))
    );
    dir.0.remove(&SegmentId(3));
    assert_eq!(
        find_current_location(& m, & dir, entry(1)), Some((Generation(5), Slot(1)))
    );
}
struct Moved {
    dir: Dir,
    older: Manifest,
    newer: Manifest,
    only_new: Manifest,
}
fn moved() -> Moved {
    let mut dir = Dir::default();
    let s1 = fixed(&mut dir, 1, 1, 0x10000, 4);
    let s2 = fixed(&mut dir, 2, 2, 0x20000, 4);
    bind(&dir, 1, 0, entry(1), true);
    bind(&dir, 1, 1, entry(2), true);
    bind(&dir, 1, 2, entry(3), true);
    bind(&dir, 2, 1, entry(1), true);
    invalidate(&dir, 1, 0);
    invalidate(&dir, 1, 2);
    let mut both = chain(vec![vec![s1], vec![s1, s2]]);
    let newer = both.pop().unwrap();
    let older = both.pop().unwrap();
    let only_new = chain(vec![vec![s1], vec![s2]]).pop().unwrap();
    Moved {
        dir,
        older,
        newer,
        only_new,
    }
}
fn loc(generation: u64, slot: u32, score: f32) -> HitLocation {
    HitLocation {
        generation: Generation(generation),
        slot: Slot(slot),
        score,
    }
}
#[test]
fn transfer_moves_invalidated_entries() {
    let f = moved();
    let locations = [loc(1, 0, 0.9), loc(1, 1, 0.8), loc(1, 2, 0.7), loc(1, 0, 0.9)];
    let r = transfer(&f.newer, Some(&f.older), &f.dir, 1, &locations);
    let entries: Vec<_> = r.hits.iter().map(|h| h.entry).collect();
    assert_eq!(entries, vec![entry(1), entry(2), entry(1)]);
    let scores: Vec<_> = r.hits.iter().map(|h| h.score).collect();
    assert_eq!(scores, vec![0.9, 0.8, 0.9]);
    assert!(r.hits.iter().all(| h | h.stale));
    assert_eq!(r.discarded, 1);
    assert_eq!(r.hits.len() + r.discarded, locations.len());
}
#[test]
fn transfer_not_stale_for_current_generation() {
    let f = moved();
    let locations = [loc(1, 1, 0.8), loc(1, 0, 0.9)];
    let hit_generation = f.newer.generation();
    let r = transfer(&f.newer, None, &f.dir, hit_generation, &locations);
    assert_eq!(r.hits.len(), 2);
    assert!(r.hits.iter().all(| h | ! h.stale));
    assert_eq!(r.discarded, 0);
}
#[test]
fn transfer_uses_previous_for_dropped_segments() {
    let f = moved();
    let locations = [loc(1, 1, 0.8), loc(1, 0, 0.9), loc(1, 2, 0.7)];
    let r = transfer(&f.only_new, Some(&f.older), &f.dir, 1, &locations);
    let entries: Vec<_> = r.hits.iter().map(|h| h.entry).collect();
    assert_eq!(entries, vec![entry(2), entry(1)]);
    assert_eq!(r.discarded, 1);
    let r = transfer(&f.only_new, None, &f.dir, 1, &locations);
    assert!(r.hits.is_empty());
    assert_eq!(r.discarded, 3);
}
#[test]
fn transfer_discards_unknown_generation_and_empty_input() {
    let f = moved();
    let r = transfer(&f.newer, Some(&f.older), &f.dir, 1, &[loc(7, 0, 1.0)]);
    assert_eq!((r.hits.len(), r.discarded), (0, 1));
    let r = transfer(&f.newer, Some(&f.older), &f.dir, 1, &[]);
    assert_eq!((r.hits.len(), r.discarded), (0, 0));
}
#[test]
fn resolve_hits_counts_both_stages() {
    let f = moved();
    let hits = [
        hit(0x10000 + 10, 0.9),
        hit(0x10000 + SLOT_BYTES, 0.8),
        hit(0x90000, 0.6),
        hit(0x10000 + 2 * SLOT_BYTES, 0.7),
    ];
    let r = resolve_hits(&f.older, &f.newer, None, &f.dir, &hits);
    let entries: Vec<_> = r.hits.iter().map(|h| h.entry).collect();
    assert_eq!(entries, vec![entry(1), entry(2)]);
    assert!(r.hits.iter().all(| h | h.stale));
    assert_eq!(r.discarded, 2);
    assert_eq!(r.hits.len() + r.discarded, hits.len());
    let r = resolve_hits(&f.newer, &f.newer, None, &f.dir, &hits);
    assert!(r.hits.iter().all(| h | ! h.stale));
}
