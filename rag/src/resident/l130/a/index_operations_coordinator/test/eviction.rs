use super::*;
use std::sync::atomic::{AtomicBool, AtomicI64, Ordering};
use std::thread;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060 as base;
use base::a::address_map::Entry;
use base::l050::a::table_swap::{HostRootSlot, RootSlot, TableSink};
use base::l050::l040::a::allocator::Allocator;
use base::l050::l040::a::slot_layout::{
    SegmentView, SlotAssignment, SlotClass, SlotStates,
};
use base::l050::l040::l030::a::bookkeeping::Bookkeeping;
use base::l050::l040::l030::a::kernel_layout::Addr;
use base::l050::l040::l030::a::region_handle::{Generation, RegionHandle};
use base::l050::l040::l030::l020::a::arenas::{ArenaSet, Budget, MIB};
use base::l050::l040::l030::l020::l010::device_lane_event::DeviceKind;
use base::l050::l040::l030::l020::l010::layout::DType;
const SEG_LEN: u64 = 4096;
const SLOT_BYTES: u64 = 512;
const SINK_ERROR: &str = "sink down";
struct CountingSink {
    remaining: AtomicI64,
}
impl CountingSink {
    fn fail_after(&self, writes: i64) {
        self.remaining.store(writes, Ordering::SeqCst);
    }
    fn recover(&self) {
        self.remaining.store(i64::MAX, Ordering::SeqCst);
    }
}
impl TableSink for CountingSink {
    fn write_visible(&self, _region: &Region, _bytes: &[u8]) -> Result<(), String> {
        if self.remaining.fetch_sub(1, Ordering::SeqCst) <= 0 {
            return Err(SINK_ERROR.to_string());
        }
        Ok(())
    }
}
fn budget() -> Budget {
    Budget {
        free: u64::MAX / 2,
        total: u64::MAX / 2,
        generation_reserve: 0,
    }
}
fn make_allocators(capacity: u64) -> Arc<AllocatorSet> {
    let mut caps = ArenaCapacities::default();
    caps.set(ArenaKind::IndexSegments, capacity);
    let set = Arc::new(ArenaSet::new(caps));
    let arena = set.ensure(ArenaKind::IndexSegments, &budget()).unwrap();
    let bookkeeping = Arc::new(Bookkeeping::new(set));
    Arc::new(AllocatorSet::new(bookkeeping, &[(arena, DeviceKind::Gpu)]).unwrap())
}
fn make_table_allocator(capacity: u64) -> Arc<Allocator> {
    let mut caps = ArenaCapacities::default();
    caps.set(ArenaKind::IndexSegments, capacity);
    let set = Arc::new(ArenaSet::new(caps));
    let arena = set.ensure(ArenaKind::IndexSegments, &budget()).unwrap();
    let bookkeeping = Arc::new(Bookkeeping::new(set));
    Arc::new(Allocator::new(arena, DeviceKind::Gpu, bookkeeping))
}
fn entry_at(id: u64) -> Entry {
    let offset = (id + 1) * SEG_LEN;
    let region = RegionHandle::new(
            ArenaKind::IndexSegments,
            offset,
            SLOT_BYTES,
            Generation(1),
        )
        .unwrap();
    let view = SegmentView::new(region, 1, SlotClass::C32, 4, DType::F32, Generation(1))
        .unwrap();
    let states = Arc::new(SlotStates::new(&view));
    let assignment = Arc::new(SlotAssignment::fixed(&view, Addr(offset)).unwrap());
    Entry::new(SegmentId(id), view, Addr(offset), states, assignment).unwrap()
}
struct Fx {
    ev: Eviction,
    swap: Arc<TableSwap>,
    store: Arc<MappingStore>,
    allocators: Arc<AllocatorSet>,
    sink: Arc<CountingSink>,
    t0: Instant,
}
impl Fx {
    fn new() -> Fx {
        Fx::with_origin(Instant::now())
    }
    fn with_origin(t0: Instant) -> Fx {
        let sink = Arc::new(CountingSink {
            remaining: AtomicI64::new(i64::MAX),
        });
        let swap = Arc::new(
            TableSwap::new(
                make_table_allocator(64 * MIB),
                Arc::clone(&sink) as Arc<dyn TableSink>,
                Arc::new(HostRootSlot::new()) as Arc<dyn RootSlot>,
            ),
        );
        let store = Arc::new(MappingStore::new());
        let allocators = make_allocators(64 * MIB);
        let ev = Eviction::new(
            "idx",
            Arc::clone(&swap),
            Arc::clone(&store),
            Arc::clone(&allocators),
            TierBudget::new(1000, 0, 0),
            TierBudget::new(1000, 0, 0),
        );
        Fx {
            ev,
            swap,
            store,
            allocators,
            sink,
            t0,
        }
    }
    fn at(&self, secs: u64) -> Instant {
        self.t0 + Duration::from_secs(secs)
    }
    fn spec(&self, id: u64, tier: Tier, size: u64, cost: f64) -> TrackSpec {
        let region = self
            .allocators
            .allocator(ArenaKind::IndexSegments)
            .unwrap()
            .allocate(SEG_LEN)
            .unwrap();
        let mut table = self.swap.next_table();
        table.add(region);
        self.swap.publish(table).unwrap();
        let mut map = self.store.next().unwrap();
        map.add(entry_at(id)).unwrap();
        self.store.publish(map).unwrap();
        TrackSpec {
            tier,
            segment: SegmentId(id),
            region,
            size,
            reload_cost: cost,
            summary: false,
        }
    }
    fn add(&self, id: u64, tier: Tier, size: u64, cost: f64, secs: u64) -> TrackSpec {
        let spec = self.spec(id, tier, size, cost);
        self.ev.track(EvictKey(id), spec, self.at(secs));
        spec
    }
    fn in_table(&self, region: &Region) -> bool {
        self.swap.current_table().is_some_and(|t| t.segments().contains(region))
    }
    fn in_map(&self, id: u64) -> bool {
        self.store.current().get(SegmentId(id)).is_some()
    }
    fn is_live(&self, region: &Region) -> bool {
        self.allocators.allocator(ArenaKind::IndexSegments).unwrap().is_live(region)
    }
    fn reclaim_all(&self) -> usize {
        self.ev.reclaim(self.swap.min_reader_epoch())
    }
}
fn victims(out: &EvictionOutcome) -> Vec<u64> {
    out.decisions.iter().map(|d| d.victim.0).collect()
}
fn reasons(out: &EvictionOutcome) -> Vec<EvictReason> {
    out.decisions.iter().map(|d| d.reason).collect()
}
#[test]
fn policy_defaults() {
    assert_eq!(
        EvictionPolicy::vram(), EvictionPolicy { high : 0.90, low : 0.75, idle :
        Duration::from_secs(600) }
    );
    assert_eq!(
        EvictionPolicy::ram(), EvictionPolicy { high : 0.85, low : 0.70, idle :
        Duration::from_secs(60) }
    );
    let fx = Fx::new();
    assert_eq!(fx.ev.policy(Tier::Vram), EvictionPolicy::vram());
    assert_eq!(fx.ev.policy(Tier::Ram), EvictionPolicy::ram());
}
#[test]
fn sanitized_replaces_non_finite_by_tier_default() {
    let idle = Duration::from_secs(7);
    let p = EvictionPolicy {
        high: f64::NAN,
        low: f64::INFINITY,
        idle,
    };
    assert_eq!(
        p.sanitized(Tier::Vram), EvictionPolicy { high : 0.90, low : 0.75, idle }
    );
    assert_eq!(p.sanitized(Tier::Ram), EvictionPolicy { high : 0.85, low : 0.70, idle });
    let q = EvictionPolicy {
        high: f64::NEG_INFINITY,
        low: f64::NAN,
        idle,
    };
    assert_eq!(q.sanitized(Tier::Ram), EvictionPolicy { high : 0.85, low : 0.70, idle });
}
#[test]
fn sanitized_clamps_marks() {
    let idle = Duration::from_secs(1);
    let clamp = |high: f64, low: f64| {
        let p = EvictionPolicy { high, low, idle }.sanitized(Tier::Vram);
        (p.high, p.low)
    };
    assert_eq!(clamp(2.0, 5.0), (1.0, 1.0));
    assert_eq!(clamp(- 1.0, - 0.5), (0.0, 0.0));
    assert_eq!(clamp(0.5, 0.8), (0.5, 0.5));
    assert_eq!(clamp(0.5, 0.2), (0.5, 0.2));
    assert_eq!(clamp(0.5, - 0.2), (0.5, 0.0));
    assert_eq!(
        EvictionPolicy { high : 0.5, low : 0.2, idle } .sanitized(Tier::Vram).idle, idle
    );
}
#[test]
fn set_policy_stores_sanitized_policy_per_tier() {
    let fx = Fx::new();
    let raw = EvictionPolicy {
        high: f64::NAN,
        low: 7.0,
        idle: Duration::from_secs(5),
    };
    fx.ev.set_policy(Tier::Ram, raw);
    assert_eq!(fx.ev.policy(Tier::Ram), raw.sanitized(Tier::Ram));
    assert_eq!(fx.ev.policy(Tier::Ram).high, 0.85);
    assert_eq!(fx.ev.policy(Tier::Ram).low, 0.85);
    assert_eq!(fx.ev.policy(Tier::Vram), EvictionPolicy::vram());
}
#[test]
fn reason_and_error_display() {
    assert_eq!(EvictReason::Idle.to_string(), "idle");
    assert_eq!(EvictReason::Pressure.to_string(), "pressure");
    assert_eq!(EvictReason::SpaceNeed.to_string(), "space-need");
    let err = EvictionError::Exhausted {
        requested: 5,
        available: 1,
    };
    assert_eq!(err.to_string(), "Arena erschöpft");
    let boxed: Box<dyn std::error::Error> = Box::new(err);
    assert_eq!(boxed.to_string(), "Arena erschöpft");
}
#[test]
fn evict_key_orders_ascending() {
    let mut keys = vec![EvictKey(9), EvictKey(2), EvictKey(5)];
    keys.sort();
    assert_eq!(keys, vec![EvictKey(2), EvictKey(5), EvictKey(9)]);
}
#[test]
fn budget_usable_saturates() {
    assert_eq!(TierBudget::new(100, 30, 20).usable(), 50);
    assert_eq!(TierBudget::new(10, 30, 20).usable(), 0);
    assert_eq!(TierBudget::new(100, 100, u64::MAX).usable(), 0);
    assert_eq!(TierBudget::new(100, 0, 0).usable(), 100);
    assert_eq!(TierBudget::default().usable(), 0);
}
#[test]
fn budget_for_vram_uses_index_arena_and_generation_reserve() {
    let mut caps = ArenaCapacities::default();
    caps.set(ArenaKind::IndexSegments, 1000);
    caps.set(ArenaKind::Weights, 5000);
    let b = TierBudget::for_vram(&caps, 4, 50);
    assert_eq!(b, TierBudget::new(1000, 750, 50));
    assert_eq!(b.usable(), 200);
    let single = TierBudget::for_vram(&caps, 1, 0);
    assert_eq!(single, TierBudget::new(1000, 0, 0));
    let zero = TierBudget::for_vram(&caps, 0, 0);
    assert_eq!(zero.reserve, caps.generation_reserve(0));
}
#[test]
fn set_budget_changes_usable_per_tier() {
    let fx = Fx::new();
    assert_eq!(fx.ev.usable(Tier::Vram), 1000);
    fx.ev.set_budget(Tier::Ram, TierBudget::new(500, 100, 50));
    assert_eq!(fx.ev.usable(Tier::Ram), 350);
    assert_eq!(fx.ev.usable(Tier::Vram), 1000);
}
#[test]
fn track_sets_hits_priority_and_occupancy() {
    let fx = Fx::new();
    fx.add(1, Tier::Vram, 100, 50.0, 0);
    let key = EvictKey(1);
    assert!(fx.ev.contains(key));
    assert_eq!(fx.ev.hits(key), Some(1));
    assert_eq!(fx.ev.priority(key), Some(0.0 + (1.0 * 50.0) / 100.0));
    assert_eq!(fx.ev.len(), 1);
    assert_eq!(fx.ev.len_tier(Tier::Vram), 1);
    assert_eq!(fx.ev.len_tier(Tier::Ram), 0);
    assert_eq!(fx.ev.bytes(Tier::Vram), 100);
    assert_eq!(fx.ev.bytes(Tier::Ram), 0);
    assert_eq!(fx.ev.deferred_bytes(Tier::Vram), 0);
    assert_eq!(fx.ev.occupancy(Tier::Vram), 100);
    assert!(! fx.ev.is_empty());
    assert_eq!(fx.ev.hits(EvictKey(99)), None);
    assert_eq!(fx.ev.priority(EvictKey(99)), None);
}
#[test]
fn track_corrects_invalid_cost_and_zero_size() {
    let fx = Fx::new();
    fx.add(1, Tier::Ram, 10, f64::NAN, 0);
    fx.add(2, Tier::Ram, 10, -3.0, 0);
    fx.add(3, Tier::Ram, 10, f64::INFINITY, 0);
    fx.add(4, Tier::Ram, 0, 3.0, 0);
    for id in 1..=3 {
        assert_eq!(fx.ev.priority(EvictKey(id)), Some(0.0));
    }
    assert_eq!(fx.ev.priority(EvictKey(4)), Some(3.0));
    assert_eq!(fx.ev.bytes(Tier::Ram), 30);
}
#[test]
fn retrack_replaces_entry_and_corrects_occupancy_across_tiers() {
    let fx = Fx::new();
    let spec = fx.add(1, Tier::Vram, 100, 10.0, 0);
    let key = EvictKey(1);
    for s in 1..=3 {
        assert!(fx.ev.touch(key, fx.at(s)));
    }
    assert_eq!(fx.ev.hits(key), Some(4));
    fx.ev
        .track(
            key,
            TrackSpec {
                tier: Tier::Ram,
                size: 40,
                reload_cost: 20.0,
                ..spec
            },
            fx.at(10),
        );
    assert_eq!(fx.ev.hits(key), Some(1));
    assert_eq!(fx.ev.priority(key), Some(0.0 + (1.0 * 20.0) / 40.0));
    assert_eq!(fx.ev.len(), 1);
    assert_eq!(fx.ev.len_tier(Tier::Vram), 0);
    assert_eq!(fx.ev.len_tier(Tier::Ram), 1);
    assert_eq!(fx.ev.bytes(Tier::Vram), 0);
    assert_eq!(fx.ev.bytes(Tier::Ram), 40);
    assert_eq!(fx.ev.occupancy(Tier::Vram), 0);
}
#[test]
fn retrack_keeps_live_pins() {
    let fx = Fx::new();
    let spec = fx.add(1, Tier::Ram, 100, 10.0, 0);
    let key = EvictKey(1);
    let pin = fx.ev.pin(key).unwrap();
    fx.ev.track(key, spec, fx.at(1));
    assert!(fx.ev.is_pinned(key));
    let out = fx.ev.evict_idle(Tier::Ram, fx.at(10_000));
    assert!(out.decisions.is_empty());
    assert!(fx.ev.contains(key));
    drop(pin);
    assert!(! fx.ev.is_pinned(key));
    let out = fx.ev.evict_idle(Tier::Ram, fx.at(10_000));
    assert_eq!(victims(& out), vec![1]);
}
#[test]
fn touch_counts_hits_and_recomputes_priority() {
    let fx = Fx::new();
    fx.add(1, Tier::Vram, 10, 10.0, 0);
    let key = EvictKey(1);
    assert!(fx.ev.touch(key, fx.at(5)));
    assert_eq!(fx.ev.hits(key), Some(2));
    assert_eq!(fx.ev.priority(key), Some(0.0 + (2.0 * 10.0) / 10.0));
    assert!(! fx.ev.touch(EvictKey(42), fx.at(5)));
    assert!(! fx.ev.contains(EvictKey(42)));
    assert_eq!(fx.ev.len(), 1);
}
#[test]
fn touch_postpones_idle_eviction() {
    let fx = Fx::new();
    fx.add(1, Tier::Ram, 10, 1.0, 0);
    assert!(fx.ev.touch(EvictKey(1), fx.at(100)));
    assert!(fx.ev.evict_idle(Tier::Ram, fx.at(120)).decisions.is_empty());
    let out = fx.ev.evict_idle(Tier::Ram, fx.at(160));
    assert_eq!(victims(& out), vec![1]);
}
#[test]
fn touch_uses_current_floor() {
    let fx = Fx::new();
    fx.ev.set_budget(Tier::Vram, TierBudget::new(200, 0, 0));
    fx.add(1, Tier::Vram, 100, 50.0, 0);
    fx.add(2, Tier::Vram, 100, 90.0, 0);
    let out = fx.ev.relieve_pressure(Tier::Vram);
    assert_eq!(victims(& out), vec![1]);
    let floor = fx.ev.floor(Tier::Vram);
    assert!(floor > 0.0);
    assert!(fx.ev.touch(EvictKey(2), fx.at(1)));
    assert_eq!(fx.ev.priority(EvictKey(2)), Some(floor + (2.0 * 90.0) / 100.0));
}
#[test]
fn track_after_floor_rise_starts_at_floor() {
    let fx = Fx::new();
    fx.ev.set_budget(Tier::Vram, TierBudget::new(200, 0, 0));
    fx.add(1, Tier::Vram, 100, 50.0, 0);
    fx.add(2, Tier::Vram, 100, 90.0, 0);
    fx.ev.relieve_pressure(Tier::Vram);
    let floor = fx.ev.floor(Tier::Vram);
    fx.add(3, Tier::Vram, 100, 10.0, 1);
    assert_eq!(fx.ev.priority(EvictKey(3)), Some(floor + (1.0 * 10.0) / 100.0));
    assert_eq!(fx.ev.floor(Tier::Ram), 0.0);
}
#[test]
fn remove_only_stops_tracking() {
    let fx = Fx::new();
    let spec = fx.add(1, Tier::Ram, 100, 1.0, 0);
    let table_epoch = fx.swap.published_epoch();
    let map_epoch = fx.store.epoch();
    let pin = fx.ev.pin(EvictKey(1)).unwrap();
    assert_eq!(fx.ev.remove(EvictKey(1)), Some(100));
    assert_eq!(fx.ev.remove(EvictKey(1)), None);
    drop(pin);
    assert!(! fx.ev.contains(EvictKey(1)));
    assert_eq!(fx.ev.occupancy(Tier::Ram), 0);
    assert_eq!(fx.ev.deferred_bytes(Tier::Ram), 0);
    assert_eq!(fx.swap.published_epoch(), table_epoch);
    assert_eq!(fx.store.epoch(), map_epoch);
    assert!(fx.in_table(& spec.region));
    assert!(fx.in_map(1));
    assert!(fx.is_live(& spec.region));
    assert_eq!(fx.reclaim_all(), 0);
    assert!(fx.is_live(& spec.region));
}
#[test]
fn pin_requires_tracked_key() {
    let fx = Fx::new();
    assert!(fx.ev.pin(EvictKey(1)).is_none());
    assert!(! fx.ev.is_pinned(EvictKey(1)));
    fx.add(1, Tier::Ram, 10, 1.0, 0);
    let pin = fx.ev.pin(EvictKey(1)).unwrap();
    assert_eq!(* pin.region(), EvictKey(1));
    assert!(fx.ev.is_pinned(EvictKey(1)));
}
#[test]
fn pin_survives_touch_and_blocks_all_eviction_paths() {
    let fx = Fx::new();
    fx.ev.set_budget(Tier::Ram, TierBudget::new(100, 0, 0));
    fx.add(1, Tier::Ram, 100, 1.0, 0);
    let pin = fx.ev.pin(EvictKey(1)).unwrap();
    fx.ev.touch(EvictKey(1), fx.at(1));
    assert!(fx.ev.is_pinned(EvictKey(1)));
    assert!(fx.ev.evict_idle(Tier::Ram, fx.at(100_000)).decisions.is_empty());
    assert!(fx.ev.relieve_pressure(Tier::Ram).decisions.is_empty());
    assert!(fx.ev.tick(fx.at(100_000)).decisions.is_empty());
    assert_eq!(
        fx.ev.make_room(Tier::Ram, 50), Err(EvictionError::Exhausted { requested : 50,
        available : 0 })
    );
    assert!(fx.ev.contains(EvictKey(1)));
    drop(pin);
    let out = fx.ev.evict_idle(Tier::Ram, fx.at(100_000));
    assert_eq!(victims(& out), vec![1]);
}
#[test]
fn multiple_pins_all_must_drop() {
    let fx = Fx::new();
    fx.add(1, Tier::Ram, 10, 1.0, 0);
    let a = fx.ev.pin(EvictKey(1)).unwrap();
    let b = fx.ev.pin(EvictKey(1)).unwrap();
    drop(a);
    assert!(fx.ev.is_pinned(EvictKey(1)));
    assert!(fx.ev.evict_idle(Tier::Ram, fx.at(1000)).decisions.is_empty());
    drop(b);
    assert!(! fx.ev.is_pinned(EvictKey(1)));
    assert_eq!(fx.ev.evict_idle(Tier::Ram, fx.at(1000)).decisions.len(), 1);
}
#[test]
fn evict_idle_selects_idle_eligible_entries_in_key_order() {
    let fx = Fx::new();
    for id in [5, 3, 9] {
        fx.add(id, Tier::Ram, 10, 1.0, 0);
    }
    fx.add(7, Tier::Ram, 10, 1.0, 50);
    let summary = TrackSpec {
        summary: true,
        ..fx.spec(4, Tier::Ram, 10, 1.0)
    };
    fx.ev.track(EvictKey(4), summary, fx.at(0));
    fx.add(6, Tier::Ram, 10, 1.0, 0);
    let pin = fx.ev.pin(EvictKey(6)).unwrap();
    fx.add(8, Tier::Vram, 10, 1.0, 0);
    let out = fx.ev.evict_idle(Tier::Ram, fx.at(60));
    assert_eq!(victims(& out), vec![3, 5, 9]);
    assert!(reasons(& out).iter().all(| r | * r == EvictReason::Idle));
    assert!(out.skipped.is_empty());
    assert!(out.aborted.is_none());
    for id in [7, 4, 6, 8] {
        assert!(fx.ev.contains(EvictKey(id)), "key {id}");
    }
    for id in [3, 5, 9] {
        assert!(! fx.ev.contains(EvictKey(id)));
        assert!(! fx.in_map(id));
    }
    assert_eq!(fx.ev.deferred_bytes(Tier::Ram), 30);
    assert_eq!(fx.ev.floor(Tier::Ram), 0.0);
    drop(pin);
}
#[test]
fn evict_idle_boundary_is_inclusive_and_tier_specific() {
    let fx = Fx::new();
    fx.add(1, Tier::Vram, 10, 1.0, 0);
    assert!(fx.ev.evict_idle(Tier::Vram, fx.at(599)).decisions.is_empty());
    assert!(fx.ev.evict_idle(Tier::Ram, fx.at(10_000)).decisions.is_empty());
    let out = fx.ev.evict_idle(Tier::Vram, fx.at(600));
    assert_eq!(victims(& out), vec![1]);
}
#[test]
fn evict_idle_does_not_raise_floor() {
    let fx = Fx::new();
    fx.add(1, Tier::Ram, 10, 100.0, 0);
    assert!(fx.ev.priority(EvictKey(1)).unwrap() > 0.0);
    fx.ev.evict_idle(Tier::Ram, fx.at(1000));
    assert_eq!(fx.ev.floor(Tier::Ram), 0.0);
    assert_eq!(fx.ev.floor(Tier::Vram), 0.0);
}
#[test]
fn evict_idle_honours_custom_idle_policy() {
    let fx = Fx::new();
    fx.ev
        .set_policy(
            Tier::Ram,
            EvictionPolicy {
                idle: Duration::from_secs(5),
                ..EvictionPolicy::ram()
            },
        );
    fx.add(1, Tier::Ram, 10, 1.0, 0);
    assert!(fx.ev.evict_idle(Tier::Ram, fx.at(4)).decisions.is_empty());
    assert_eq!(fx.ev.evict_idle(Tier::Ram, fx.at(5)).decisions.len(), 1);
}
#[test]
fn evict_idle_with_earlier_now_does_not_panic() {
    let fx = Fx::new();
    fx.add(1, Tier::Ram, 10, 1.0, 100);
    assert!(fx.ev.evict_idle(Tier::Ram, fx.at(0)).decisions.is_empty());
}
fn fill_pressure_scenario(fx: &Fx) {
    for i in 1..=9u64 {
        fx.add(i, Tier::Vram, 100, (i * 10) as f64, 0);
    }
}
#[test]
fn pressure_requires_strictly_more_than_high() {
    let fx = Fx::new();
    fill_pressure_scenario(&fx);
    assert_eq!(fx.ev.occupancy(Tier::Vram), 900);
    let out = fx.ev.relieve_pressure(Tier::Vram);
    assert_eq!(out, EvictionOutcome::default());
    assert_eq!(fx.ev.len(), 9);
    assert_eq!(fx.ev.floor(Tier::Vram), 0.0);
}
#[test]
fn pressure_evicts_lowest_priority_until_low_watermark() {
    let fx = Fx::new();
    fill_pressure_scenario(&fx);
    fx.add(10, Tier::Vram, 1, 0.0, 0);
    assert_eq!(fx.ev.occupancy(Tier::Vram), 901);
    let p2 = fx.ev.priority(EvictKey(2)).unwrap();
    let out = fx.ev.relieve_pressure(Tier::Vram);
    assert_eq!(victims(& out), vec![10, 1, 2]);
    assert!(reasons(& out).iter().all(| r | * r == EvictReason::Pressure));
    assert_eq!(fx.ev.floor(Tier::Vram), p2);
    assert_eq!(fx.ev.floor(Tier::Ram), 0.0);
    assert_eq!(fx.ev.occupancy(Tier::Vram), 901);
    assert_eq!(fx.ev.deferred_bytes(Tier::Vram), 201);
    assert_eq!(fx.ev.bytes(Tier::Vram), 700);
    assert_eq!(fx.reclaim_all(), 3);
    assert_eq!(fx.ev.occupancy(Tier::Vram), 700);
    assert_eq!(fx.ev.relieve_pressure(Tier::Vram), EvictionOutcome::default());
}
#[test]
fn pressure_counts_deferred_bytes_until_reclaim() {
    let fx = Fx::new();
    fill_pressure_scenario(&fx);
    fx.add(10, Tier::Vram, 1, 0.0, 0);
    let first = fx.ev.relieve_pressure(Tier::Vram);
    assert_eq!(victims(& first), vec![10, 1, 2]);
    let second = fx.ev.relieve_pressure(Tier::Vram);
    assert_eq!(victims(& second), vec![3, 4]);
    assert_eq!(fx.reclaim_all(), 5);
    assert_eq!(fx.ev.occupancy(Tier::Vram), 500);
}
#[test]
fn pressure_floor_is_monotonic() {
    let fx = Fx::new();
    fx.ev.set_budget(Tier::Vram, TierBudget::new(200, 0, 0));
    fx.add(1, Tier::Vram, 100, 50.0, 0);
    fx.add(2, Tier::Vram, 100, 10.0, 0);
    let pin = fx.ev.pin(EvictKey(2)).unwrap();
    let out = fx.ev.relieve_pressure(Tier::Vram);
    assert_eq!(victims(& out), vec![1]);
    let floor = fx.ev.floor(Tier::Vram);
    assert_eq!(floor, 0.5);
    assert_eq!(fx.reclaim_all(), 1);
    drop(pin);
    fx.add(3, Tier::Vram, 100, 100.0, 1);
    let low = fx.ev.priority(EvictKey(2)).unwrap();
    assert!(low < floor);
    let out = fx.ev.relieve_pressure(Tier::Vram);
    assert_eq!(victims(& out), vec![2]);
    assert_eq!(fx.ev.floor(Tier::Vram), floor);
}
#[test]
fn pressure_is_best_effort_when_entries_are_not_eligible() {
    let fx = Fx::new();
    fx.ev.set_budget(Tier::Vram, TierBudget::new(100, 0, 0));
    fx.add(1, Tier::Vram, 80, 1.0, 0);
    let pin = fx.ev.pin(EvictKey(1)).unwrap();
    let summary = TrackSpec {
        summary: true,
        ..fx.spec(2, Tier::Vram, 10, 1.0)
    };
    fx.ev.track(EvictKey(2), summary, fx.at(0));
    fx.add(3, Tier::Vram, 20, 1.0, 0);
    assert_eq!(fx.ev.occupancy(Tier::Vram), 110);
    let out = fx.ev.relieve_pressure(Tier::Vram);
    assert_eq!(victims(& out), vec![3]);
    assert!(out.aborted.is_none());
    assert!(fx.ev.contains(EvictKey(1)));
    assert!(fx.ev.contains(EvictKey(2)));
    assert_eq!(fx.ev.occupancy(Tier::Vram), 110);
    drop(pin);
}
#[test]
fn pressure_with_zero_usable_evicts_everything_eligible() {
    let fx = Fx::new();
    fx.ev.set_budget(Tier::Vram, TierBudget::default());
    assert_eq!(fx.ev.relieve_pressure(Tier::Vram), EvictionOutcome::default());
    fx.add(1, Tier::Vram, 10, 1.0, 0);
    fx.add(2, Tier::Vram, 10, 2.0, 0);
    let out = fx.ev.relieve_pressure(Tier::Vram);
    assert_eq!(victims(& out), vec![1, 2]);
}
#[test]
fn pressure_respects_policy_marks() {
    let fx = Fx::new();
    fx.ev
        .set_policy(
            Tier::Ram,
            EvictionPolicy {
                high: 0.5,
                low: 0.2,
                idle: Duration::from_secs(3600),
            },
        );
    for i in 1..=6u64 {
        fx.add(i, Tier::Ram, 100, i as f64, 0);
    }
    let out = fx.ev.relieve_pressure(Tier::Ram);
    assert_eq!(victims(& out), vec![1, 2, 3, 4]);
}
#[test]
fn victim_order_prefers_low_priority_over_age() {
    let fx = Fx::new();
    fx.ev.set_budget(Tier::Ram, TierBudget::new(200, 0, 0));
    fx.add(10, Tier::Ram, 100, 50.0, 0);
    fx.add(11, Tier::Ram, 100, 20.0, 100);
    let out = fx.ev.make_room(Tier::Ram, 200).unwrap();
    assert_eq!(victims(& out), vec![11, 10]);
}
#[test]
fn victim_order_ties_break_by_age_then_key() {
    let fx = Fx::new();
    fx.add(3, Tier::Ram, 100, 0.0, 5);
    fx.add(1, Tier::Ram, 100, 0.0, 5);
    fx.add(2, Tier::Ram, 100, 0.0, 1);
    let out = fx.ev.make_room(Tier::Ram, 1000).unwrap();
    assert_eq!(victims(& out), vec![2, 1, 3]);
}
#[test]
fn make_room_is_noop_when_space_is_free() {
    let fx = Fx::new();
    fx.add(1, Tier::Ram, 300, 1.0, 0);
    let table_epoch = fx.swap.published_epoch();
    assert_eq!(fx.ev.make_room(Tier::Ram, 0), Ok(EvictionOutcome::default()));
    assert_eq!(fx.ev.make_room(Tier::Ram, 700), Ok(EvictionOutcome::default()));
    assert!(fx.ev.contains(EvictKey(1)));
    assert_eq!(fx.swap.published_epoch(), table_epoch);
}
#[test]
fn make_room_with_need_zero_on_full_tier_is_noop() {
    let fx = Fx::new();
    fx.ev.set_budget(Tier::Ram, TierBudget::new(100, 0, 0));
    fx.add(1, Tier::Ram, 100, 1.0, 0);
    assert_eq!(fx.ev.make_room(Tier::Ram, 0), Ok(EvictionOutcome::default()));
    assert!(fx.ev.contains(EvictKey(1)));
}
#[test]
fn make_room_evicts_just_enough_victims() {
    let fx = Fx::new();
    for i in 1..=3u64 {
        fx.add(i, Tier::Ram, 100, (i * 10) as f64, 0);
    }
    let p1 = fx.ev.priority(EvictKey(1)).unwrap();
    let out = fx.ev.make_room(Tier::Ram, 800).unwrap();
    assert_eq!(victims(& out), vec![1]);
    assert_eq!(reasons(& out), vec![EvictReason::SpaceNeed]);
    assert_eq!(fx.ev.floor(Tier::Ram), p1);
    assert_eq!(fx.ev.floor(Tier::Vram), 0.0);
    assert_eq!(fx.ev.len(), 2);
}
#[test]
fn make_room_collects_until_bytes_reach_shortfall() {
    let fx = Fx::new();
    fx.ev.set_budget(Tier::Ram, TierBudget::new(300, 0, 0));
    for i in 1..=3u64 {
        fx.add(i, Tier::Ram, 100, i as f64, 0);
    }
    assert_eq!(victims(& fx.ev.make_room(Tier::Ram, 250).unwrap()), vec![1, 2, 3]);
}
#[test]
fn make_room_exhausted_changes_nothing() {
    let fx = Fx::new();
    fx.ev.set_budget(Tier::Ram, TierBudget::new(400, 0, 0));
    fx.add(1, Tier::Ram, 100, 1.0, 0);
    let pin = fx.ev.pin(EvictKey(1)).unwrap();
    let summary = TrackSpec {
        summary: true,
        ..fx.spec(2, Tier::Ram, 100, 1.0)
    };
    fx.ev.track(EvictKey(2), summary, fx.at(0));
    fx.add(3, Tier::Ram, 100, 1.0, 0);
    fx.add(4, Tier::Vram, 500, 1.0, 0);
    let table_epoch = fx.swap.published_epoch();
    let map_epoch = fx.store.epoch();
    let err = fx.ev.make_room(Tier::Ram, 500).unwrap_err();
    assert_eq!(err, EvictionError::Exhausted { requested : 500, available : 200 });
    assert_eq!(fx.ev.len(), 4);
    assert_eq!(fx.ev.occupancy(Tier::Ram), 300);
    assert_eq!(fx.ev.floor(Tier::Ram), 0.0);
    assert_eq!(fx.swap.published_epoch(), table_epoch);
    assert_eq!(fx.store.epoch(), map_epoch);
    drop(pin);
}
#[test]
fn make_room_counts_deferred_bytes_until_reclaim() {
    let fx = Fx::new();
    fx.ev.set_budget(Tier::Ram, TierBudget::new(300, 0, 0));
    for i in 1..=3u64 {
        fx.add(i, Tier::Ram, 100, i as f64, 0);
    }
    let first = fx.ev.make_room(Tier::Ram, 100).unwrap();
    assert_eq!(victims(& first), vec![1]);
    assert_eq!(fx.ev.occupancy(Tier::Ram), 300);
    let second = fx.ev.make_room(Tier::Ram, 100).unwrap();
    assert_eq!(victims(& second), vec![2]);
    assert_eq!(fx.reclaim_all(), 2);
    assert_eq!(fx.ev.occupancy(Tier::Ram), 100);
    assert_eq!(fx.ev.make_room(Tier::Ram, 200), Ok(EvictionOutcome::default()));
}
#[test]
fn planned_victim_pinned_meanwhile_is_neither_evicted_nor_skipped() {
    let fx = Fx::new();
    fx.add(1, Tier::Ram, 100, 1.0, 0);
    fx.add(2, Tier::Ram, 100, 2.0, 0);
    let pin = fx.ev.pin(EvictKey(1)).unwrap();
    let out = fx
        .ev
        .run(Tier::Ram, vec![EvictKey(1), EvictKey(2)], EvictReason::SpaceNeed);
    assert_eq!(victims(& out), vec![2]);
    assert!(out.skipped.is_empty());
    assert!(fx.ev.contains(EvictKey(1)));
    drop(pin);
}
#[test]
fn planned_victim_removed_meanwhile_is_ignored() {
    let fx = Fx::new();
    let spec = fx.add(1, Tier::Ram, 100, 1.0, 0);
    fx.add(2, Tier::Ram, 100, 2.0, 0);
    fx.ev.remove(EvictKey(1));
    let out = fx
        .ev
        .run(Tier::Ram, vec![EvictKey(1), EvictKey(2)], EvictReason::SpaceNeed);
    assert_eq!(victims(& out), vec![2]);
    assert!(out.skipped.is_empty());
    assert!(fx.in_table(& spec.region));
}
#[test]
fn planned_victim_of_other_tier_is_not_evicted() {
    let fx = Fx::new();
    fx.add(1, Tier::Vram, 100, 1.0, 0);
    let out = fx.ev.run(Tier::Ram, vec![EvictKey(1)], EvictReason::SpaceNeed);
    assert_eq!(out, EvictionOutcome::default());
    assert!(fx.ev.contains(EvictKey(1)));
}
#[test]
fn eviction_removes_segment_from_table_and_map_and_defers_region() {
    let fx = Fx::new();
    let spec = fx.add(1, Tier::Ram, 100, 10.0, 0);
    fx.ev.touch(EvictKey(1), fx.at(1));
    fx.ev.touch(EvictKey(1), fx.at(2));
    let table_before = fx.swap.published_epoch();
    let map_before = fx.store.epoch();
    let out = fx.ev.evict_idle(Tier::Ram, fx.at(1000));
    assert_eq!(
        out.decisions, vec![EvictionDecision { index : "idx".to_string(), victim :
        EvictKey(1), segment : SegmentId(1), size : 100, bytes : 100, hits : 3, reason :
        EvictReason::Idle, }]
    );
    assert!(! fx.in_table(& spec.region));
    assert!(! fx.in_map(1));
    let retire = fx.swap.published_epoch();
    assert_eq!(retire, table_before + 1);
    assert_eq!(fx.store.epoch(), map_before + 1);
    assert!(! fx.ev.contains(EvictKey(1)));
    assert_eq!(fx.ev.bytes(Tier::Ram), 0);
    assert_eq!(fx.ev.deferred_bytes(Tier::Ram), 100);
    assert_eq!(fx.ev.occupancy(Tier::Ram), 100);
    assert!(fx.is_live(& spec.region));
    assert_eq!(fx.ev.reclaim(retire - 1), 0);
    assert!(fx.is_live(& spec.region));
    assert_eq!(fx.ev.deferred_bytes(Tier::Ram), 100);
    assert_eq!(fx.ev.reclaim(retire), 1);
    assert!(! fx.is_live(& spec.region));
    assert_eq!(fx.ev.deferred_bytes(Tier::Ram), 0);
    assert_eq!(fx.ev.occupancy(Tier::Ram), 0);
    assert_eq!(fx.ev.reclaim(retire), 0);
    assert_eq!(fx.reclaim_all(), 0);
}
#[test]
fn decision_reports_size_floor_of_one_and_tracked_bytes() {
    let fx = Fx::new();
    fx.add(1, Tier::Ram, 0, 3.0, 0);
    let out = fx.ev.evict_idle(Tier::Ram, fx.at(1000));
    assert_eq!(out.decisions.len(), 1);
    assert_eq!(out.decisions[0].size, 1);
    assert_eq!(out.decisions[0].bytes, 0);
    assert_eq!(fx.ev.deferred_bytes(Tier::Ram), 0);
}
#[test]
fn each_eviction_publishes_its_own_epoch() {
    let fx = Fx::new();
    let a = fx.add(1, Tier::Ram, 10, 1.0, 0);
    let b = fx.add(2, Tier::Ram, 10, 1.0, 0);
    let before = fx.swap.published_epoch();
    let out = fx.ev.evict_idle(Tier::Ram, fx.at(1000));
    assert_eq!(victims(& out), vec![1, 2]);
    assert_eq!(fx.swap.published_epoch(), before + 2);
    assert_eq!(fx.ev.reclaim(before + 1), 1);
    assert!(! fx.is_live(& a.region));
    assert!(fx.is_live(& b.region));
    assert_eq!(fx.ev.reclaim(before + 2), 1);
    assert!(! fx.is_live(& b.region));
}
#[test]
fn reclaim_respects_min_reader_epoch_of_active_readers() {
    let fx = Fx::new();
    let spec = fx.add(1, Tier::Ram, 10, 1.0, 0);
    let reader = fx.swap.register_reader();
    let guard = fx.swap.acquire(reader).unwrap();
    let held = guard.epoch();
    fx.ev.evict_idle(Tier::Ram, fx.at(1000));
    assert_eq!(fx.swap.min_reader_epoch(), held);
    assert_eq!(fx.reclaim_all(), 0);
    assert!(fx.is_live(& spec.region));
    drop(guard);
    assert_eq!(fx.reclaim_all(), 0);
    fx.swap.set_quiescent(reader, true).unwrap();
    assert_eq!(fx.reclaim_all(), 1);
    assert!(! fx.is_live(& spec.region));
}
#[test]
fn unknown_region_is_skipped_and_untracked() {
    let fx = Fx::new();
    let ghost = TrackSpec {
        tier: Tier::Ram,
        segment: SegmentId(77),
        region: Region {
            arena: ArenaKind::IndexSegments,
            offset: 1 << 40,
            len: 64,
            generation: 1,
        },
        size: 50,
        reload_cost: 1.0,
        summary: false,
    };
    fx.ev.track(EvictKey(1), ghost, fx.at(0));
    let real = fx.add(2, Tier::Ram, 10, 1.0, 0);
    let table_before = fx.swap.published_epoch();
    let map_before = fx.store.epoch();
    let out = fx.ev.evict_idle(Tier::Ram, fx.at(1000));
    assert_eq!(out.skipped, vec![EvictKey(1)]);
    assert_eq!(victims(& out), vec![2]);
    assert!(out.aborted.is_none());
    assert!(! fx.ev.contains(EvictKey(1)));
    assert_eq!(fx.ev.deferred_bytes(Tier::Ram), 10);
    assert_eq!(fx.swap.published_epoch(), table_before + 1);
    assert_eq!(fx.store.epoch(), map_before + 1);
    assert!(! fx.in_table(& real.region));
}
#[test]
fn unknown_region_alone_publishes_nothing() {
    let fx = Fx::new();
    let ghost = TrackSpec {
        tier: Tier::Ram,
        segment: SegmentId(77),
        region: Region {
            arena: ArenaKind::IndexSegments,
            offset: 1 << 40,
            len: 64,
            generation: 1,
        },
        size: 50,
        reload_cost: 1.0,
        summary: false,
    };
    fx.ev.track(EvictKey(1), ghost, fx.at(0));
    let table_before = fx.swap.published_epoch();
    let map_before = fx.store.epoch();
    let out = fx.ev.evict_idle(Tier::Ram, fx.at(1000));
    assert_eq!(out.skipped, vec![EvictKey(1)]);
    assert!(out.decisions.is_empty());
    assert_eq!(fx.swap.published_epoch(), table_before);
    assert_eq!(fx.store.epoch(), map_before);
    assert_eq!(fx.ev.deferred_bytes(Tier::Ram), 0);
    assert_eq!(fx.ev.occupancy(Tier::Ram), 0);
}
#[test]
fn segment_only_in_map_uses_published_epoch() {
    let fx = Fx::new();
    let spec = fx.add(1, Tier::Ram, 10, 1.0, 0);
    let mut table = fx.swap.next_table();
    assert!(table.remove(& spec.region));
    fx.swap.publish(table).unwrap();
    let table_epoch = fx.swap.published_epoch();
    let map_before = fx.store.epoch();
    let out = fx.ev.evict_idle(Tier::Ram, fx.at(1000));
    assert_eq!(victims(& out), vec![1]);
    assert_eq!(fx.swap.published_epoch(), table_epoch);
    assert_eq!(fx.store.epoch(), map_before + 1);
    assert!(! fx.in_map(1));
    assert_eq!(fx.ev.reclaim(table_epoch), 1);
    assert!(! fx.is_live(& spec.region));
}
#[test]
fn segment_only_in_table_leaves_map_untouched() {
    let fx = Fx::new();
    let spec = fx.add(1, Tier::Ram, 10, 1.0, 0);
    let mut map = fx.store.next().unwrap();
    assert!(map.remove(SegmentId(1)));
    fx.store.publish(map).unwrap();
    let map_epoch = fx.store.epoch();
    let table_before = fx.swap.published_epoch();
    let out = fx.ev.evict_idle(Tier::Ram, fx.at(1000));
    assert_eq!(victims(& out), vec![1]);
    assert_eq!(fx.store.epoch(), map_epoch);
    assert_eq!(fx.swap.published_epoch(), table_before + 1);
    assert!(! fx.in_table(& spec.region));
}
#[test]
fn reclaim_drops_entries_whose_release_fails() {
    let fx = Fx::new();
    let spec = fx.add(1, Tier::Ram, 10, 1.0, 0);
    fx.ev.evict_idle(Tier::Ram, fx.at(1000));
    fx.allocators
        .allocator(ArenaKind::IndexSegments)
        .unwrap()
        .free(&spec.region)
        .unwrap();
    assert_eq!(fx.reclaim_all(), 0);
    assert_eq!(fx.ev.deferred_bytes(Tier::Ram), 0);
    assert_eq!(fx.reclaim_all(), 0);
}
#[test]
fn reclaim_drops_entries_of_unknown_arena() {
    let fx = Fx::new();
    let mut spec = fx.spec(1, Tier::Ram, 10, 1.0);
    let real = spec.region;
    spec.region = Region {
        arena: ArenaKind::Weights,
        ..real
    };
    fx.ev.track(EvictKey(1), spec, fx.at(0));
    let out = fx.ev.evict_idle(Tier::Ram, fx.at(1000));
    assert_eq!(victims(& out), vec![1]);
    assert_eq!(fx.ev.deferred_bytes(Tier::Ram), 10);
    assert_eq!(fx.reclaim_all(), 0);
    assert_eq!(fx.ev.deferred_bytes(Tier::Ram), 0);
    assert!(fx.in_table(& real));
    assert!(fx.is_live(& real));
}
#[test]
fn publish_failure_aborts_and_restores_entry() {
    let fx = Fx::new();
    fx.add(1, Tier::Ram, 100, 10.0, 0);
    for s in 1..=2 {
        fx.ev.touch(EvictKey(1), fx.at(s));
    }
    let spec = fx.add(2, Tier::Ram, 50, 1.0, 0);
    let hits = fx.ev.hits(EvictKey(1));
    let priority = fx.ev.priority(EvictKey(1));
    let last_used = lock(&fx.ev.state).entries.get(&EvictKey(1)).unwrap().last_used;
    let table_epoch = fx.swap.published_epoch();
    let map_epoch = fx.store.epoch();
    fx.sink.fail_after(0);
    let out = fx.ev.evict_idle(Tier::Ram, fx.at(1000));
    assert!(out.decisions.is_empty());
    assert!(out.skipped.is_empty());
    assert_eq!(
        out.aborted, Some(Abort { key : EvictKey(1), cause :
        AbortCause::Table(TableSwapError::Write(SINK_ERROR.to_string())), })
    );
    assert!(fx.ev.contains(EvictKey(1)));
    assert!(fx.ev.contains(EvictKey(2)));
    assert_eq!(fx.ev.hits(EvictKey(1)), hits);
    assert_eq!(fx.ev.priority(EvictKey(1)), priority);
    assert_eq!(
        lock(& fx.ev.state).entries.get(& EvictKey(1)).unwrap().last_used, last_used
    );
    assert_eq!(fx.ev.bytes(Tier::Ram), 150);
    assert_eq!(fx.ev.deferred_bytes(Tier::Ram), 0);
    assert_eq!(fx.swap.published_epoch(), table_epoch);
    assert_eq!(fx.store.epoch(), map_epoch);
    assert!(fx.in_table(& spec.region));
    assert!(fx.in_map(1));
    assert!(fx.in_map(2));
    fx.sink.recover();
    let again = fx.ev.evict_idle(Tier::Ram, fx.at(1000));
    assert_eq!(victims(& again), vec![1, 2]);
    assert_eq!(again.decisions[0].hits, 3);
}
#[test]
fn abort_keeps_earlier_decisions_and_stops_processing() {
    let fx = Fx::new();
    for id in 1..=3 {
        fx.add(id, Tier::Ram, 10, 1.0, 0);
    }
    fx.sink.fail_after(1);
    let out = fx.ev.evict_idle(Tier::Ram, fx.at(1000));
    assert_eq!(victims(& out), vec![1]);
    assert_eq!(out.aborted.as_ref().map(| a | a.key), Some(EvictKey(2)));
    assert!(! fx.ev.contains(EvictKey(1)));
    assert!(fx.ev.contains(EvictKey(2)));
    assert!(fx.ev.contains(EvictKey(3)));
    assert_eq!(fx.ev.deferred_bytes(Tier::Ram), 10);
}
#[test]
fn abort_does_not_raise_floor() {
    let fx = Fx::new();
    fx.add(1, Tier::Ram, 100, 90.0, 0);
    fx.sink.fail_after(0);
    let out = fx.ev.make_room(Tier::Ram, 1000).unwrap();
    assert!(out.aborted.is_some());
    assert_eq!(fx.ev.floor(Tier::Ram), 0.0);
}
#[test]
fn abort_restore_prefers_newer_entry() {
    let fx = Fx::new();
    let old = fx.add(1, Tier::Ram, 100, 10.0, 0);
    let taken = lock(&fx.ev.state).entries.remove(&EvictKey(1)).unwrap();
    let newer = TrackSpec {
        tier: Tier::Vram,
        size: 7,
        ..old
    };
    fx.ev.track(EvictKey(1), newer, fx.at(5));
    let step = fx
        .ev
        .restore(EvictKey(1), taken, AbortCause::Table(TableSwapError::NoTable));
    assert!(matches!(step, Step::Aborted(_)));
    assert_eq!(fx.ev.bytes(Tier::Vram), 7);
    assert_eq!(fx.ev.bytes(Tier::Ram), 0);
}
#[test]
fn tick_runs_idle_then_pressure_for_vram_then_ram() {
    let fx = Fx::new();
    fx.add(1, Tier::Vram, 50, 1.0, 0);
    fx.add(2, Tier::Vram, 950, 1.0, 650);
    fx.add(3, Tier::Ram, 100, 1.0, 0);
    fx.add(4, Tier::Ram, 1000, 1.0, 699);
    let out = fx.ev.tick(fx.at(700));
    assert_eq!(victims(& out), vec![1, 2, 3, 4]);
    assert_eq!(
        reasons(& out), vec![EvictReason::Idle, EvictReason::Pressure, EvictReason::Idle,
        EvictReason::Pressure]
    );
    assert!(out.aborted.is_none());
    assert!(out.skipped.is_empty());
}
#[test]
fn tick_without_work_is_empty() {
    let fx = Fx::new();
    fx.add(1, Tier::Vram, 10, 1.0, 0);
    fx.add(2, Tier::Ram, 10, 1.0, 0);
    assert_eq!(fx.ev.tick(fx.at(1)), EvictionOutcome::default());
}
#[test]
fn tick_stops_after_first_abort() {
    let fx = Fx::new();
    fx.add(1, Tier::Vram, 10, 1.0, 0);
    fx.add(3, Tier::Ram, 10, 1.0, 0);
    fx.sink.fail_after(0);
    let out = fx.ev.tick(fx.at(10_000));
    assert!(out.decisions.is_empty());
    assert_eq!(out.aborted.as_ref().map(| a | a.key), Some(EvictKey(1)));
    assert!(fx.ev.contains(EvictKey(1)));
    assert!(fx.ev.contains(EvictKey(3)));
}
#[test]
fn tick_abort_in_ram_keeps_vram_decisions() {
    let fx = Fx::new();
    fx.add(1, Tier::Vram, 10, 1.0, 0);
    fx.add(3, Tier::Ram, 10, 1.0, 0);
    fx.sink.fail_after(1);
    let out = fx.ev.tick(fx.at(10_000));
    assert_eq!(victims(& out), vec![1]);
    assert_eq!(out.aborted.as_ref().map(| a | a.key), Some(EvictKey(3)));
}
#[test]
fn outcome_merge_keeps_first_abort() {
    let abort = |k| Abort {
        key: EvictKey(k),
        cause: AbortCause::Table(TableSwapError::NoTable),
    };
    let mut total = EvictionOutcome::default();
    total
        .merge(EvictionOutcome {
            aborted: Some(abort(1)),
            skipped: vec![EvictKey(8)],
            ..Default::default()
        });
    total
        .merge(EvictionOutcome {
            aborted: Some(abort(2)),
            skipped: vec![EvictKey(9)],
            ..Default::default()
        });
    assert_eq!(total.aborted, Some(abort(1)));
    assert_eq!(total.skipped, vec![EvictKey(8), EvictKey(9)]);
}
fn populate_mixed(fx: &Fx) {
    for id in [8, 3, 6, 1, 9, 2, 7] {
        let tier = if id % 2 == 0 { Tier::Ram } else { Tier::Vram };
        fx.add(id, tier, 100 + id, ((id * 7) % 5) as f64, id % 3);
    }
    fx.ev.set_budget(Tier::Vram, TierBudget::new(400, 0, 0));
    fx.ev.set_budget(Tier::Ram, TierBudget::new(300, 0, 0));
}
#[test]
fn identical_state_gives_identical_decisions() {
    let origin = Instant::now();
    let a = Fx::with_origin(origin);
    let b = Fx::with_origin(origin);
    populate_mixed(&a);
    populate_mixed(&b);
    let out_a = a.ev.tick(a.at(5));
    let out_b = b.ev.tick(b.at(5));
    assert!(! out_a.decisions.is_empty());
    assert_eq!(out_a, out_b);
    let room_a = a.ev.make_room(Tier::Vram, 200);
    let room_b = b.ev.make_room(Tier::Vram, 200);
    assert!(room_a.as_ref().is_ok_and(| o | ! o.decisions.is_empty()));
    assert_eq!(room_a, room_b);
}
#[test]
fn publish_map_retries_after_stale_mapping() {
    let fx = Fx::new();
    fx.add(1, Tier::Ram, 10, 1.0, 0);
    fx.add(2, Tier::Ram, 10, 1.0, 0);
    let mut first = fx.store.next().unwrap();
    assert!(first.remove(SegmentId(1)));
    let mut other = fx.store.next().unwrap();
    assert!(other.remove(SegmentId(2)));
    fx.store.publish(other).unwrap();
    assert!(
        matches!(fx.store.publish(first.clone()), Err(MapError::StaleMapping { .. }))
    );
    fx.ev.publish_map(first, SegmentId(1));
    assert!(! fx.in_map(1));
    assert!(! fx.in_map(2));
}
#[test]
fn publish_map_stops_when_segment_already_gone() {
    let fx = Fx::new();
    fx.add(1, Tier::Ram, 10, 1.0, 0);
    let mut first = fx.store.next().unwrap();
    assert!(first.remove(SegmentId(1)));
    let mut other = fx.store.next().unwrap();
    assert!(other.remove(SegmentId(1)));
    fx.store.publish(other).unwrap();
    let epoch = fx.store.epoch();
    fx.ev.publish_map(first, SegmentId(1));
    assert_eq!(fx.store.epoch(), epoch);
    assert!(! fx.in_map(1));
}
#[test]
fn poisoned_locks_are_taken_over() {
    let fx = Fx::new();
    fx.add(1, Tier::Ram, 10, 1.0, 0);
    thread::scope(|s| {
        let state = s
            .spawn(|| {
                let _guard = lock(&fx.ev.state);
                panic!("poison state");
            });
        assert!(state.join().is_err());
        let serial = s
            .spawn(|| {
                let _guard = lock(&fx.ev.evict_lock);
                panic!("poison serial");
            });
        assert!(serial.join().is_err());
    });
    assert!(fx.ev.state.is_poisoned());
    assert!(fx.ev.evict_lock.is_poisoned());
    assert_eq!(fx.ev.len(), 1);
    assert!(fx.ev.touch(EvictKey(1), fx.at(1)));
    let out = fx.ev.tick(fx.at(1000));
    assert_eq!(victims(& out), vec![1]);
    assert_eq!(fx.reclaim_all(), 1);
}
#[test]
fn concurrent_use_keeps_table_map_and_tracking_consistent() {
    const N: u64 = 40;
    let fx = Fx::new();
    let mut specs = Vec::new();
    for id in 0..N {
        let tier = if id % 2 == 0 { Tier::Vram } else { Tier::Ram };
        specs.push(fx.add(id, tier, 10, 1.0 + (id % 5) as f64, 0));
    }
    let done = AtomicBool::new(false);
    let far = fx.at(100_000);
    let (decided, mut reclaimed) = thread::scope(|s| {
        let mut workers = Vec::new();
        for w in 0..4u64 {
            let fx = &fx;
            workers
                .push(
                    s
                        .spawn(move || {
                            for round in 0..200u64 {
                                let key = EvictKey((w * 7 + round) % N);
                                fx.ev.touch(key, fx.at(0));
                                if let Some(pin) = fx.ev.pin(key) {
                                    for _ in 0..20 {
                                        assert!(fx.ev.contains(key));
                                    }
                                    drop(pin);
                                }
                                fx.ev.occupancy(Tier::Ram);
                                fx.ev.len_tier(Tier::Vram);
                            }
                        }),
                );
        }
        let evictor = s
            .spawn(|| {
                let mut decided = 0usize;
                let mut reclaimed = 0usize;
                while !done.load(Ordering::SeqCst) {
                    let out = fx.ev.tick(far);
                    assert!(out.aborted.is_none());
                    decided += out.decisions.len();
                    if let Ok(room) = fx.ev.make_room(Tier::Ram, 5) {
                        assert!(room.aborted.is_none());
                        decided += room.decisions.len();
                    }
                    reclaimed += fx.reclaim_all();
                    thread::yield_now();
                }
                (decided, reclaimed)
            });
        for w in workers {
            w.join().unwrap();
        }
        done.store(true, Ordering::SeqCst);
        evictor.join().unwrap()
    });
    reclaimed += fx.reclaim_all();
    assert_eq!(decided + fx.ev.len(), N as usize);
    assert_eq!(reclaimed, decided);
    for (id, spec) in specs.iter().enumerate() {
        let tracked = fx.ev.contains(EvictKey(id as u64));
        assert_eq!(fx.in_table(& spec.region), tracked, "table {id}");
        assert_eq!(fx.in_map(id as u64), tracked, "map {id}");
        assert_eq!(fx.is_live(& spec.region), tracked, "region {id}");
    }
    assert_eq!(fx.ev.deferred_bytes(Tier::Vram), 0);
    assert_eq!(fx.ev.deferred_bytes(Tier::Ram), 0);
}
