use super::*;
use std::sync::Arc;
use std::thread;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050 as l050;
use l050::a::table_swap::{HostRootSlot, RootSlot, SegmentTable, TableSink};
use l050::l040::l030::a::bookkeeping::Bookkeeping;
use l050::l040::l030::l020::a::arenas::{ArenaCapacities, ArenaKind, ArenaSet, Budget};
use l050::l040::l030::l020::l010::device_lane_event::DeviceKind;
struct NullSink;
impl TableSink for NullSink {
    fn write_visible(
        &self,
        _region: &RegionHandle,
        _bytes: &[u8],
    ) -> Result<(), String> {
        Ok(())
    }
}
fn allocators(capacity: u64) -> (Arc<Allocator>, Arc<Allocator>) {
    let mut caps = ArenaCapacities::default();
    caps.set(ArenaKind::IndexSegments, capacity);
    caps.set(ArenaKind::Staging, 4096);
    let set = Arc::new(ArenaSet::new(caps));
    let budget = Budget {
        free: u64::MAX / 2,
        total: u64::MAX / 2,
        generation_reserve: 0,
    };
    let seg = set.ensure(ArenaKind::IndexSegments, &budget).unwrap();
    let tbl = set.ensure(ArenaKind::Staging, &budget).unwrap();
    let bk = Arc::new(Bookkeeping::new(set));
    (
        Arc::new(Allocator::new(seg, DeviceKind::Gpu, Arc::clone(&bk))),
        Arc::new(Allocator::new(tbl, DeviceKind::Cpu, bk)),
    )
}
fn swap_for(table_alloc: Arc<Allocator>) -> TableSwap {
    TableSwap::new(
        table_alloc,
        Arc::new(NullSink) as Arc<dyn TableSink>,
        Arc::new(HostRootSlot::new()) as Arc<dyn RootSlot>,
    )
}
fn arena(capacity: u64, config: SegmentArenaConfig) -> SegmentArena<Arc<Allocator>> {
    SegmentArena::new(allocators(capacity).0, config).unwrap()
}
fn publish_initial(swap: &TableSwap, segs: &[&Segment]) -> Epoch {
    let mut t = swap.next_table();
    for s in segs {
        t.add(s.region);
    }
    swap.publish(t).unwrap()
}
#[test]
fn config_validation() {
    let (a, _) = allocators(65536);
    let bad = [
        SegmentArenaConfig {
            alignment: 100,
            ..Default::default()
        },
        SegmentArenaConfig {
            alignment: 128,
            ..Default::default()
        },
        SegmentArenaConfig {
            reserve_bp: 10_001,
            ..Default::default()
        },
        SegmentArenaConfig {
            concurrent_replacements: 0,
            ..Default::default()
        },
    ];
    for c in bad {
        assert_eq!(
            SegmentArena::new(Arc::clone(& a), c).err(),
            Some(SegmentArenaError::InvalidConfig)
        );
    }
    assert!(SegmentArena::new(a, SegmentArenaConfig::default()).is_ok());
}
#[test]
fn allocate_is_aligned_and_rounded() {
    let arena = arena(65536, SegmentArenaConfig::default());
    let s = arena.allocate(1).unwrap();
    assert_eq!(s.len, 256);
    assert_eq!(s.offset % 256, 0);
    assert!(s.offset >= s.region.offset());
    assert!(s.offset + s.len <= s.region.offset() + s.region.len());
    assert_eq!(s.region.len(), 256 + 255);
    assert_eq!(arena.allocate(0), Err(SegmentArenaError::ZeroLength));
}
#[test]
fn allocate_overflow_is_exhausted() {
    let arena = arena(65536, SegmentArenaConfig::default());
    assert!(
        matches!(arena.allocate(u64::MAX), Err(SegmentArenaError::Exhausted { .. }))
    );
}
#[test]
fn reserve_is_never_granted() {
    let cfg = SegmentArenaConfig {
        reserve_bp: 5_000,
        ..Default::default()
    };
    let arena = arena(4096, cfg);
    assert_eq!(arena.budget().reserve, 2048);
    assert!(matches!(arena.allocate(2048), Err(SegmentArenaError::Exhausted { .. })));
    assert!(arena.allocate(256).is_ok());
}
#[test]
fn headroom_is_planned_for_the_new_segment() {
    let arena = arena(1500, SegmentArenaConfig::default());
    arena.allocate(256).unwrap();
    let err = arena.allocate(256).unwrap_err();
    assert_eq!(
        err, SegmentArenaError::Exhausted { requested : 511, available : 1500 - 511 - 511
        }
    );
    assert_eq!(arena.stats().live_segments, 1);
}
#[test]
fn stats_and_budget_ignore_regions_freed_at_the_allocator() {
    let (seg_alloc, _) = allocators(8192);
    let arena = SegmentArena::new(Arc::clone(&seg_alloc), SegmentArenaConfig::default())
        .unwrap();
    let a = arena.allocate(3072).unwrap();
    let b = arena.allocate(256).unwrap();
    assert_eq!(arena.budget().headroom, a.region.len());
    seg_alloc.free(&a.region).unwrap();
    let st = arena.stats();
    assert_eq!(st.live_segments, 1);
    assert_eq!(st.largest_segment, b.region.len());
    assert_eq!(st.live_bytes, b.region.len());
    let bud = arena.budget();
    assert_eq!(bud.live, b.region.len());
    assert_eq!(bud.headroom, b.region.len());
    assert_eq!(arena.stats(), st);
}
#[test]
fn allocate_prunes_freed_regions_before_planning_headroom() {
    let (seg_alloc, _) = allocators(8192);
    let arena = SegmentArena::new(Arc::clone(&seg_alloc), SegmentArenaConfig::default())
        .unwrap();
    let a = arena.allocate(3072).unwrap();
    arena.allocate(256).unwrap();
    seg_alloc.free(&a.region).unwrap();
    seg_alloc.allocate(4000).unwrap();
    arena.allocate(256).unwrap();
    assert_eq!(arena.stats().live_segments, 2);
}
#[test]
fn release_of_pruned_segment_is_stale() {
    let (seg_alloc, _) = allocators(8192);
    let arena = SegmentArena::new(Arc::clone(&seg_alloc), SegmentArenaConfig::default())
        .unwrap();
    let a = arena.allocate(256).unwrap();
    seg_alloc.free(&a.region).unwrap();
    assert_eq!(arena.stats().live_segments, 0);
    assert_eq!(arena.release(& a, Readers::None), Err(SegmentArenaError::StaleRegion));
}
#[test]
fn running_replacement_is_not_pruned() {
    let (seg_alloc, _) = allocators(8192);
    let arena = SegmentArena::new(Arc::clone(&seg_alloc), SegmentArenaConfig::default())
        .unwrap();
    let a = arena.allocate(256).unwrap();
    let new = arena.begin_replace(&a).unwrap();
    seg_alloc.free(&a.region).unwrap();
    seg_alloc.free(&new.region).unwrap();
    assert_eq!(arena.stats().live_segments, 2);
    assert!(arena.replacement().is_some());
}
#[test]
fn budget_and_stats_follow_bookkeeping() {
    let arena = arena(8192, SegmentArenaConfig::default());
    assert_eq!(arena.stats().largest_segment, 0);
    let a = arena.allocate(256).unwrap();
    let b = arena.allocate(512).unwrap();
    let st = arena.stats();
    assert_eq!(st.live_segments, 2);
    assert_eq!(st.live_bytes, 511 + 767);
    assert_eq!(st.largest_segment, 767);
    let bud = arena.budget();
    assert_eq!(bud.live, st.live_bytes);
    assert_eq!(bud.headroom, 767);
    assert_eq!(bud.available, 8192 - 1278 - 767);
    assert!(bud.to_string().contains('%'));
    assert_eq!(arena.replacement_budget().live_bytes, 1278);
    assert_eq!(arena.replacement_budget().headroom(), 767);
    arena.release(&b, Readers::None).unwrap();
    assert_eq!(arena.stats().largest_segment, 511);
    arena.release(&a, Readers::None).unwrap();
    assert_eq!(arena.stats().largest_segment, 0);
}
#[test]
fn release_rules() {
    let arena = arena(8192, SegmentArenaConfig::default());
    let s = arena.allocate(256).unwrap();
    arena.release(&s, Readers::None).unwrap();
    assert_eq!(arena.release(& s, Readers::None), Err(SegmentArenaError::StaleRegion));
    let s = arena.allocate(256).unwrap();
    let (ev, ctl) = Event::mock();
    arena.release(&s, Readers::After(ev)).unwrap();
    assert_eq!(arena.stats().live_segments, 0);
    assert_eq!(arena.replacement_budget().live_bytes, 511);
    ctl.complete();
    arena.allocate(256).unwrap();
    assert_eq!(arena.replacement_budget().live_bytes, 511);
}
#[test]
fn release_of_foreign_segment_is_stale() {
    let arena = arena(8192, SegmentArenaConfig::default());
    let other = SegmentArena::new(allocators(8192).0, SegmentArenaConfig::default())
        .unwrap();
    let s = other.allocate(256).unwrap();
    assert_eq!(arena.release(& s, Readers::None), Err(SegmentArenaError::StaleRegion));
}
#[test]
fn replacement_by_copy_and_reclaim() {
    let (a, t) = allocators(8192);
    let arena = SegmentArena::new(a, SegmentArenaConfig::default()).unwrap();
    let swap = swap_for(t);
    let old = arena.allocate(256).unwrap();
    publish_initial(&swap, &[&old]);
    assert_eq!(arena.finish_replace(& swap), Err(SegmentArenaError::NoReplacement));
    assert_eq!(arena.discard_replacement(), Err(SegmentArenaError::NoReplacement));
    let new = arena.begin_replace(&old).unwrap();
    assert_eq!(new.len, old.len);
    assert_ne!(new.region, old.region);
    assert_eq!(arena.replacement(), Some(ReplacementInfo { old, new }));
    assert_eq!(
        arena.begin_replace(& old), Err(SegmentArenaError::ReplacementInProgress)
    );
    assert_eq!(arena.stats().live_segments, 2);
    let epoch = arena.finish_replace(&swap).unwrap();
    assert_eq!(epoch, 2);
    assert_eq!(arena.replacement(), None);
    let st = arena.stats();
    assert_eq!((st.live_segments, st.deferred_segments, st.deferred_bytes), (1, 1, 511));
    assert_eq!(arena.budget().deferred, 511);
    assert_eq!(arena.reclaim(epoch - 1), 0);
    assert_eq!(arena.reclaim(epoch), 1);
    assert_eq!(arena.reclaim(epoch), 0);
    assert_eq!(arena.stats().deferred_segments, 0);
    assert_eq!(arena.release(& old, Readers::None), Err(SegmentArenaError::StaleRegion));
}
#[test]
fn finish_replace_errors_keep_replacement() {
    let (a, t) = allocators(8192);
    let arena = SegmentArena::new(a, SegmentArenaConfig::default()).unwrap();
    let swap = swap_for(t);
    let old = arena.allocate(256).unwrap();
    arena.begin_replace(&old).unwrap();
    assert_eq!(arena.finish_replace(& swap), Err(SegmentArenaError::OldNotInTable));
    assert!(arena.replacement().is_some());
    let _: SegmentTable = swap.next_table();
    publish_initial(&swap, &[&old]);
    arena.finish_replace(&swap).unwrap();
}
#[test]
fn discard_replacement_frees_copy() {
    let arena = arena(8192, SegmentArenaConfig::default());
    let old = arena.allocate(256).unwrap();
    let before = arena.replacement_budget().live_bytes;
    let new = arena.begin_replace(&old).unwrap();
    assert_eq!(arena.replacement_budget().live_bytes, before + 511);
    arena.discard_replacement().unwrap();
    assert_eq!(arena.replacement(), None);
    assert_eq!(arena.replacement_budget().live_bytes, before);
    assert_eq!(arena.release(& new, Readers::None), Err(SegmentArenaError::StaleRegion));
    assert!(arena.release(& old, Readers::None).is_ok());
}
#[test]
fn begin_replace_checks_swap_need() {
    let (a, _) = allocators(2000);
    let arena = SegmentArena::new(Arc::clone(&a), SegmentArenaConfig::default())
        .unwrap();
    let s = arena.allocate(256).unwrap();
    let _foreign = a.allocate(1000).unwrap();
    let err = arena.begin_replace(&s).unwrap_err();
    assert_eq!(
        err, SegmentArenaError::Exhausted { requested : 511, available : 2000 - 1511 }
    );
    assert_eq!(arena.replacement(), None);
}
#[test]
fn begin_replace_of_unknown_is_stale() {
    let arena = arena(8192, SegmentArenaConfig::default());
    let s = arena.allocate(256).unwrap();
    arena.release(&s, Readers::None).unwrap();
    assert_eq!(arena.begin_replace(& s), Err(SegmentArenaError::StaleRegion));
}
#[test]
fn bump_segment_claims() {
    let arena = arena(8192, SegmentArenaConfig::default());
    let mut b = arena.allocate_bump(256).unwrap();
    let base = b.segment().offset;
    assert_eq!(b.claim(3, 1).unwrap(), base);
    assert_eq!(b.claim(4, 8).unwrap(), base + 8);
    assert_eq!(b.written(), 12);
    assert_eq!(b.written() + b.remaining(), 256);
    assert_eq!(b.claim(0, 16).unwrap(), base + 16);
    assert_eq!(b.claim(1, 3), Err(SegmentArenaError::InvalidAlignment));
    assert_eq!(b.claim(1, 0), Err(SegmentArenaError::InvalidAlignment));
    let rem = b.remaining();
    assert_eq!(
        b.claim(rem + 1, 1), Err(SegmentArenaError::Exhausted { requested : rem + 1,
        available : rem })
    );
    assert_eq!(b.remaining(), rem);
    assert_eq!(
        b.claim(u64::MAX, 1).unwrap_err(), SegmentArenaError::Exhausted { requested :
        u64::MAX, available : rem }
    );
    let (seg, written) = b.into_parts();
    assert_eq!(written, 16);
    arena.release(&seg, Readers::None).unwrap();
}
#[test]
fn budget_utilization() {
    let b = ArenaBudget {
        capacity: 0,
        reserve: 0,
        live: 0,
        deferred: 0,
        headroom: 0,
        available: 0,
    };
    assert_eq!(b.utilization(), 0.0);
    let b = ArenaBudget {
        capacity: 1000,
        reserve: 0,
        live: 250,
        deferred: 250,
        headroom: 0,
        available: 500,
    };
    assert_eq!(b.utilization(), 0.5);
}
#[test]
fn replacement_budget_math() {
    let rb = ReplacementBudget {
        live_bytes: 10,
        replacements: 3,
        largest_segment: u64::MAX,
    };
    assert_eq!(rb.headroom(), u64::MAX);
    assert_eq!(rb.swap_need(5), Some(15));
    assert_eq!(rb.swap_need(u64::MAX), None);
}
#[test]
fn parallel_release_succeeds_once() {
    let arena = Arc::new(arena(8192, SegmentArenaConfig::default()));
    let s = arena.allocate(256).unwrap();
    let handles: Vec<_> = (0..8)
        .map(|_| {
            let a = Arc::clone(&arena);
            thread::spawn(move || a.release(&s, Readers::None).is_ok())
        })
        .collect();
    let ok = handles
        .into_iter()
        .filter(|_| true)
        .map(|h| h.join().unwrap())
        .filter(|&b| b)
        .count();
    assert_eq!(ok, 1);
}
#[test]
fn parallel_allocate_respects_budget() {
    let arena = Arc::new(arena(4096, SegmentArenaConfig::default()));
    let handles: Vec<_> = (0..16)
        .map(|_| {
            let a = Arc::clone(&arena);
            thread::spawn(move || a.allocate(256).is_ok())
        })
        .collect();
    let ok = handles.into_iter().map(|h| h.join().unwrap()).filter(|&b| b).count();
    assert_eq!(ok, 4096 / 511 - 1);
    let b = arena.budget();
    assert!(b.live + b.headroom <= b.capacity);
}
#[test]
fn error_display() {
    assert_eq!(SegmentArenaError::StaleRegion.to_string(), "stale Region");
    assert_eq!(SegmentArenaError::NoReplacement.to_string(), "kein Replacement");
    assert!(
        SegmentArenaError::Exhausted { requested : 1, available : 2 } .to_string()
        .starts_with("Arena erschöpft")
    );
}
