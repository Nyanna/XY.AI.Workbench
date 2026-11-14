use super::*;
use std::collections::BTreeMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{OnceLock, Weak};
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::{
    a::index_contract::{
        AccessMethod, EntryId, Payload, PayloadRef, SharedBytes, Stamp, StampObserver,
        StampSignal,
    },
    l010::{
        capability_model::{Axis, Name, Value},
        layout::DType,
    },
};
enum Mode {
    Ok,
    FailOn(u64),
    Unsupported,
}
struct MockIndex {
    mode: Mode,
    calls: AtomicUsize,
    manager: OnceLock<Weak<LoadManager>>,
}
impl MockIndex {
    fn new(mode: Mode) -> Self {
        Self {
            mode,
            calls: AtomicUsize::new(0),
            manager: OnceLock::new(),
        }
    }
}
impl Index for MockIndex {
    fn dtype(&self) -> DType {
        DType::F32
    }
    fn access_methods(&self) -> &[AccessMethod] {
        &[AccessMethod::Centroid]
    }
    fn stamp(&self, _id: EntryId) -> Option<Stamp> {
        None
    }
    fn stamps(&self) -> Vec<(EntryId, Stamp)> {
        Vec::new()
    }
    fn read(&self, _id: EntryId) -> Payload {
        Payload::default()
    }
    fn write(
        &self,
        _id: EntryId,
        _payload: PayloadRef<'_>,
    ) -> Result<Stamp, IndexError> {
        Err(IndexError::Unsupported(AccessMethod::Range))
    }
    fn delete(&self, _id: EntryId) -> Result<Option<Stamp>, IndexError> {
        Ok(None)
    }
    fn subscribe(&self) -> StampObserver {
        StampSignal::new(Stamp(0)).observer()
    }
    fn summary(&self, node: NodeId) -> Result<Summary, IndexError> {
        self.calls.fetch_add(1, Ordering::SeqCst);
        if let Some(m) = self.manager.get().and_then(Weak::upgrade) {
            let _ = m.resident_bytes();
        }
        match self.mode {
            Mode::Unsupported => Err(IndexError::Unsupported(AccessMethod::Centroid)),
            Mode::FailOn(n) if n == node.0 => Err(IndexError::NotFound),
            _ => {
                Ok(Summary {
                    node,
                    centroid: SharedBytes::from(vec![0u8; node.0 as usize]),
                    count: 1,
                    stamp: Stamp(1),
                })
            }
        }
    }
}
fn geo() -> UniformGeometry {
    UniformGeometry::new(100, 10)
}
fn manager_with(mode: Mode) -> (Arc<MockIndex>, Arc<LoadManager>) {
    let index = Arc::new(MockIndex::new(mode));
    let manager = Arc::new(
        LoadManager::new(index.clone(), Arc::new(geo()), LoadPolicy::default()),
    );
    (index, manager)
}
fn set(ids: &[u64]) -> EntrySet {
    EntrySet::from_ids(ids.iter().copied())
}
fn profile(entries: &[(Axis, Value)]) -> PrecisionProfile {
    PrecisionProfile {
        position: 0.5,
        config: entries.iter().cloned().collect::<BTreeMap<_, _>>(),
    }
}
#[test]
fn policy_defaults() {
    let p = LoadPolicy::default();
    assert_eq!(p.density_threshold(), 0.12);
    assert_eq!(p.segment_bytes(), 64 * MIB);
    assert_eq!(p.direct_map_max_bytes(), 64 * 1024);
}
#[test]
fn with_density_clamps_and_ignores_non_finite() {
    let p = LoadPolicy::default();
    assert_eq!(p.with_density(0.5).density_threshold(), 0.5);
    assert_eq!(p.with_density(7.0).density_threshold(), 1.0);
    assert_eq!(p.with_density(- 3.0).density_threshold(), 0.0);
    assert_eq!(p.with_density(f64::NAN).density_threshold(), 0.12);
    assert_eq!(p.with_density(f64::INFINITY).density_threshold(), 0.12);
    assert_eq!(p.with_density(f64::NEG_INFINITY).density_threshold(), 0.12);
}
#[test]
fn apply_profile_reads_permille() {
    let mut p = LoadPolicy::default();
    assert!(p.apply_profile(& profile(& [(axes::density(), Name::from_number(250))])));
    assert_eq!(p.density_threshold(), 0.25);
    assert!(p.apply_profile(& profile(& [(axes::density(), Name::from_number(1500))])));
    assert_eq!(p.density_threshold(), 1.0);
    assert!(p.apply_profile(& profile(& [(axes::density(), Name::from_number(0))])));
    assert_eq!(p.density_threshold(), 0.0);
}
#[test]
fn apply_profile_leaves_policy_on_missing_or_bad_value() {
    let mut p = LoadPolicy::default().with_density(0.3);
    let before = p;
    assert!(! p.apply_profile(& profile(& [])));
    assert_eq!(p, before);
    for bad in ["abc", "-5", "0.5", ""] {
        let bad_profile = profile(&[(axes::density(), Name::new(bad))]);
        assert!(! p.apply_profile(& bad_profile));
        assert_eq!(p, before);
    }
}
#[test]
fn segment_bytes_for_budget_examples() {
    assert_eq!(segment_bytes_for_budget(0), 4 * MIB);
    assert_eq!(segment_bytes_for_budget(u64::MAX), 256 * MIB);
    assert_eq!(segment_bytes_for_budget(100 * MIB), 12 * MIB);
    assert_eq!(segment_bytes_for_budget(32 * MIB), 4 * MIB);
    assert_eq!(segment_bytes_for_budget(1024 * MIB), 128 * MIB);
    assert_eq!(segment_bytes_for_budget(2048 * MIB), 256 * MIB);
}
#[test]
fn with_budget_uses_usable() {
    let budget = Budget {
        free: 900 * MIB,
        total: 2000 * MIB,
        generation_reserve: 100 * MIB,
    };
    let p = LoadPolicy::default().with_budget(&budget);
    assert_eq!(p.segment_bytes(), 100 * MIB);
    let tiny = Budget {
        free: 10,
        total: 100,
        generation_reserve: 50,
    };
    assert_eq!(LoadPolicy::default().with_budget(& tiny).segment_bytes(), 4 * MIB);
}
#[test]
fn uniform_geometry_basics() {
    let g = UniformGeometry::new(100, 10);
    assert_eq!(g.segment_of(0), 0);
    assert_eq!(g.segment_of(9), 0);
    assert_eq!(g.segment_of(10), 1);
    assert_eq!(g.entry_size(5), 100);
    assert_eq!(g.segment_size(3), 1000);
    let z = UniformGeometry::new(100, 0);
    assert_eq!(z.segment_of(7), 7);
    assert_eq!(z.segment_size(0), 100);
    let big = UniformGeometry::new(u64::MAX, 4);
    assert_eq!(big.segment_size(0), u64::MAX);
}
#[test]
fn uniform_geometry_for_policy() {
    let p = LoadPolicy::default().with_segment_bytes(1000);
    assert_eq!(UniformGeometry::for_policy(100, & p), UniformGeometry::new(100, 10));
    assert_eq!(UniformGeometry::for_policy(0, & p), UniformGeometry::new(0, 1));
    assert_eq!(UniformGeometry::for_policy(5000, & p), UniformGeometry::new(5000, 1));
}
#[test]
fn empty_set_yields_no_plans() {
    let p = LoadPolicy::default();
    assert!(plan_loads(& EntrySet::empty(), Access::Regular, & geo(), & p).is_empty());
    assert!(plan_loads(& EntrySet::empty(), Access::Rare, & geo(), & p).is_empty());
}
#[test]
fn dense_segment_loads_whole_and_sparse_gathers() {
    let p = LoadPolicy::default();
    let plans = plan_loads(&set(&[0, 1, 15]), Access::Regular, &geo(), &p);
    assert_eq!(
        plans, vec![LoadPlan::Segments { segments : vec![0], bytes : 1000 },
        LoadPlan::Gather { entries : vec![15], bytes : 100 },]
    );
}
#[test]
fn density_boundary_is_inclusive() {
    let g = UniformGeometry::new(100, 10);
    let p = LoadPolicy::default().with_density(0.1);
    let plans = plan_loads(&set(&[3]), Access::Regular, &g, &p);
    assert_eq!(plans, vec![LoadPlan::Segments { segments : vec![0], bytes : 1000 }]);
}
#[test]
fn zero_density_loads_every_touched_segment() {
    let p = LoadPolicy::default().with_density(0.0);
    let plans = plan_loads(&set(&[3, 25, 99]), Access::Regular, &geo(), &p);
    assert_eq!(
        plans, vec![LoadPlan::Segments { segments : vec![0, 2, 9], bytes : 3000 }]
    );
}
#[test]
fn only_one_kind_means_no_empty_plan() {
    let p = LoadPolicy::default();
    let gather_only = plan_loads(&set(&[5, 25]), Access::Regular, &geo(), &p);
    assert_eq!(
        gather_only, vec![LoadPlan::Gather { entries : vec![5, 25], bytes : 200 }]
    );
    let segment_only = plan_loads(&set(&[0, 1]), Access::Regular, &geo(), &p);
    assert_eq!(segment_only.len(), 1);
    assert!(matches!(segment_only[0], LoadPlan::Segments { .. }));
}
#[test]
fn rare_small_set_is_direct_map() {
    let p = LoadPolicy::default().with_direct_map_max_bytes(300);
    let s = set(&[0, 40, 90]);
    let plans = plan_loads(&s, Access::Rare, &geo(), &p);
    assert_eq!(plans, vec![LoadPlan::DirectMap { entries : s, bytes : 300 }]);
}
#[test]
fn rare_above_limit_falls_back_to_regular_planning() {
    let p = LoadPolicy::default().with_direct_map_max_bytes(300);
    let s = set(&[0, 1, 2, 3]);
    let rare = plan_loads(&s, Access::Rare, &geo(), &p);
    assert_eq!(rare, plan_loads(& s, Access::Regular, & geo(), & p));
    assert!(matches!(rare[0], LoadPlan::Segments { .. }));
}
#[test]
fn regular_never_direct_maps() {
    let p = LoadPolicy::default();
    let plans = plan_loads(&set(&[1]), Access::Regular, &geo(), &p);
    assert!(plans.iter().all(| p | ! matches!(p, LoadPlan::DirectMap { .. })));
}
struct Interleaved;
impl Geometry for Interleaved {
    fn segment_of(&self, entry: u64) -> u64 {
        entry % 3
    }
    fn entry_size(&self, _entry: u64) -> u64 {
        10
    }
    fn segment_size(&self, segment: u64) -> u64 {
        if segment == 2 { 0 } else { 1000 }
    }
}
#[test]
fn non_monotonic_geometry_keeps_lists_sorted_and_disjoint() {
    let p = LoadPolicy::default().with_density(0.0);
    let s = EntrySet::from_ranges([0..12u64]);
    let plans = plan_loads(&s, Access::Regular, &Interleaved, &p);
    assert_eq!(plans.len(), 2);
    let LoadPlan::Segments { segments, bytes } = &plans[0] else {
        panic!("segments plan expected");
    };
    assert_eq!(segments, & vec![0, 1]);
    assert_eq!(* bytes, 2000);
    let LoadPlan::Gather { entries, bytes } = &plans[1] else {
        panic!("gather plan expected");
    };
    assert_eq!(entries, & vec![2, 5, 8, 11]);
    assert_eq!(* bytes, 40);
}
struct Huge;
impl Geometry for Huge {
    fn segment_of(&self, _entry: u64) -> u64 {
        0
    }
    fn entry_size(&self, _entry: u64) -> u64 {
        u64::MAX
    }
    fn segment_size(&self, _segment: u64) -> u64 {
        u64::MAX
    }
}
#[test]
fn planning_saturates() {
    let p = LoadPolicy::default().with_density(1.0);
    let plans = plan_loads(&set(&[1, 2, 3]), Access::Regular, &Huge, &p);
    assert_eq!(plans[0].bytes(), u64::MAX);
    let rare = plan_loads(&set(&[1, 2]), Access::Rare, &Huge, &p);
    assert!(matches!(rare[0], LoadPlan::Segments { .. }));
}
#[test]
fn planning_is_deterministic_and_covers_every_entry_once() {
    let p = LoadPolicy::default();
    let s = EntrySet::from_ranges([0..13u64, 40..42, 77..78]);
    let a = plan_loads(&s, Access::Regular, &geo(), &p);
    let b = plan_loads(&s, Access::Regular, &geo(), &p);
    assert_eq!(a, b);
    let g = geo();
    for e in s.iter() {
        let in_segment = a
            .iter()
            .any(|plan| match plan {
                LoadPlan::Segments { segments, .. } => {
                    segments.contains(&g.segment_of(e))
                }
                _ => false,
            });
        let in_gather = a
            .iter()
            .any(|plan| match plan {
                LoadPlan::Gather { entries, .. } => entries.contains(&e),
                _ => false,
            });
        assert!(in_segment ^ in_gather, "entry {e} must be in exactly one plan");
    }
}
#[test]
fn execute_sums_in_order() {
    let plans = vec![
        LoadPlan::Gather { entries : vec![1], bytes : 10, }, LoadPlan::Segments {
        segments : vec![0], bytes : 20, },
    ];
    let mut seen = Vec::new();
    let total = execute_plans::<
        (),
    >(
        &plans,
        |p| {
            seen.push(p.bytes());
            Ok(p.bytes() + 1)
        },
    );
    assert_eq!(total, Ok(32));
    assert_eq!(seen, vec![10, 20]);
    assert_eq!(execute_plans::< () > (& [], | _ | Ok(5)), Ok(0));
}
#[test]
fn execute_stops_at_first_error_and_returns_it() {
    let plans: Vec<LoadPlan> = (0..4)
        .map(|i| LoadPlan::Segments {
            segments: vec![i],
            bytes: 1,
        })
        .collect();
    let mut calls = 0;
    let result = execute_plans(
        &plans,
        |_| {
            calls += 1;
            if calls == 2 { Err("boom".to_string()) } else { Ok(1) }
        },
    );
    assert_eq!(result, Err("boom".to_string()));
    assert_eq!(calls, 2);
}
#[test]
fn execute_saturates() {
    let plans = vec![LoadPlan::Segments { segments : vec![0], bytes : 1, }; 3];
    let total = execute_plans::<()>(&plans, |_| Ok(u64::MAX - 1));
    assert_eq!(total, Ok(u64::MAX));
}
#[test]
fn manager_loads_nothing_on_construction() {
    let (index, m) = manager_with(Mode::Ok);
    assert_eq!(index.calls.load(Ordering::SeqCst), 0);
    assert_eq!(m.resident_bytes(), 0);
    assert_eq!(m.tier(Resident::Summary(NodeId(1))), Tier::NotLoaded);
    assert_eq!(m.tier(Resident::Segment(0)), Tier::NotLoaded);
    assert_eq!(m.summary(NodeId(1)), None);
    assert_eq!(m.policy(), LoadPolicy::default());
}
#[test]
fn load_summaries_skips_held_nodes() {
    let (index, m) = manager_with(Mode::Ok);
    assert_eq!(m.load_summaries(& [NodeId(1), NodeId(2), NodeId(1)]), Ok(2));
    assert_eq!(index.calls.load(Ordering::SeqCst), 2);
    assert_eq!(m.load_summaries(& [NodeId(1), NodeId(2), NodeId(3)]), Ok(1));
    assert_eq!(index.calls.load(Ordering::SeqCst), 3);
    assert_eq!(m.tier(Resident::Summary(NodeId(3))), Tier::Summary);
    assert_eq!(m.summary(NodeId(2)).map(| s | s.centroid.len()), Some(2));
    assert_eq!(m.resident_bytes(), (1 + 24) + (2 + 24) + (3 + 24));
}
#[test]
fn load_summaries_error_keeps_earlier_loads() {
    let (index, m) = manager_with(Mode::FailOn(3));
    let result = m.load_summaries(&[NodeId(1), NodeId(2), NodeId(3), NodeId(4)]);
    assert_eq!(result, Err(IndexError::NotFound));
    assert_eq!(index.calls.load(Ordering::SeqCst), 3);
    assert_eq!(m.tier(Resident::Summary(NodeId(1))), Tier::Summary);
    assert_eq!(m.tier(Resident::Summary(NodeId(2))), Tier::Summary);
    assert_eq!(m.tier(Resident::Summary(NodeId(3))), Tier::NotLoaded);
    assert_eq!(m.tier(Resident::Summary(NodeId(4))), Tier::NotLoaded);
}
#[test]
fn unsupported_index_error_is_passed_through() {
    let (_, m) = manager_with(Mode::Unsupported);
    assert_eq!(
        m.load_summaries(& [NodeId(1)]),
        Err(IndexError::Unsupported(AccessMethod::Centroid))
    );
}
#[test]
fn lock_is_not_held_during_index_calls() {
    let (index, m) = manager_with(Mode::Ok);
    index.manager.set(Arc::downgrade(&m)).unwrap();
    assert_eq!(m.load_summaries(& [NodeId(1), NodeId(2)]), Ok(2));
}
#[test]
fn concurrent_loading_creates_no_double_state() {
    let (_, m) = manager_with(Mode::Ok);
    let nodes: Vec<NodeId> = (1..=10).map(NodeId).collect();
    let loaded: usize = std::thread::scope(|scope| {
        let handles: Vec<_> = (0..8)
            .map(|_| scope.spawn(|| m.load_summaries(&nodes).unwrap()))
            .collect();
        handles.into_iter().map(|h| h.join().unwrap()).sum()
    });
    assert_eq!(loaded, 10);
    let expected: u64 = (1..=10u64).map(|n| n + 24).sum();
    assert_eq!(m.resident_bytes(), expected);
}
#[test]
fn plan_omits_resident_segments() {
    let (_, m) = manager_with(Mode::Ok);
    let s = EntrySet::from_ranges([0..20u64]);
    assert_eq!(
        m.plan(& s, Access::Regular), vec![LoadPlan::Segments { segments : vec![0, 1],
        bytes : 2000 }]
    );
    m.mark_segment_loaded(0, 1000);
    assert_eq!(
        m.plan(& s, Access::Regular), vec![LoadPlan::Segments { segments : vec![1], bytes
        : 1000 }]
    );
    m.mark_segment_loaded(1, 1000);
    assert!(m.plan(& s, Access::Regular).is_empty());
}
#[test]
fn plan_omits_resident_entries_in_gather_and_direct_map() {
    let (_, m) = manager_with(Mode::Ok);
    let s = set(&[5, 15]);
    assert_eq!(
        m.plan(& s, Access::Regular), vec![LoadPlan::Gather { entries : vec![5, 15],
        bytes : 200 }]
    );
    assert_eq!(
        m.plan(& s, Access::Rare), vec![LoadPlan::DirectMap { entries : s.clone(), bytes
        : 200 }]
    );
    m.mark_segment_loaded(0, 1000);
    assert_eq!(
        m.plan(& s, Access::Regular), vec![LoadPlan::Gather { entries : vec![15], bytes :
        100 }]
    );
    assert_eq!(
        m.plan(& s, Access::Rare), vec![LoadPlan::DirectMap { entries : set(& [15]),
        bytes : 100 }]
    );
    m.mark_segment_loaded(1, 1000);
    assert!(m.plan(& s, Access::Regular).is_empty());
    assert!(m.plan(& s, Access::Rare).is_empty());
}
#[test]
fn plan_without_resident_state_matches_plan_loads() {
    let (_, m) = manager_with(Mode::Ok);
    let s = set(&[0, 1, 15, 33]);
    assert_eq!(
        m.plan(& s, Access::Regular), plan_loads(& s, Access::Regular, & geo(), &
        LoadPolicy::default())
    );
}
#[test]
fn mark_replaces_bytes_and_touch_ignores_unknown() {
    let (_, m) = manager_with(Mode::Ok);
    m.mark_segment_loaded(1, 100);
    m.mark_segment_loaded(1, 50);
    assert_eq!(m.resident_bytes(), 50);
    assert_eq!(m.tier(Resident::Segment(1)), Tier::Vector);
    m.touch_segment(99);
    assert_eq!(m.tier(Resident::Segment(99)), Tier::NotLoaded);
    assert_eq!(m.resident_bytes(), 50);
}
#[test]
fn evict_frees_segments_in_lru_order() {
    let (_, m) = manager_with(Mode::Ok);
    m.mark_segment_loaded(1, 100);
    m.mark_segment_loaded(2, 200);
    m.mark_segment_loaded(3, 300);
    m.touch_segment(1);
    assert_eq!(m.evict(150), 200);
    assert_eq!(m.tier(Resident::Segment(2)), Tier::NotLoaded);
    assert_eq!(m.tier(Resident::Segment(3)), Tier::Vector);
    assert_eq!(m.evict(1), 300);
    assert_eq!(m.tier(Resident::Segment(1)), Tier::Vector);
    assert_eq!(m.resident_bytes(), 100);
}
#[test]
fn remarking_moves_segment_to_newest_position() {
    let (_, m) = manager_with(Mode::Ok);
    m.mark_segment_loaded(1, 10);
    m.mark_segment_loaded(2, 10);
    m.mark_segment_loaded(1, 10);
    assert_eq!(m.evict(1), 10);
    assert_eq!(m.tier(Resident::Segment(2)), Tier::NotLoaded);
    assert_eq!(m.tier(Resident::Segment(1)), Tier::Vector);
}
#[test]
fn evict_prefers_vectors_then_summaries() {
    let (_, m) = manager_with(Mode::Ok);
    m.load_summaries(&[NodeId(1), NodeId(2), NodeId(3)]).unwrap();
    m.mark_segment_loaded(7, 500);
    assert_eq!(m.evict(10), 500);
    assert_eq!(m.tier(Resident::Segment(7)), Tier::NotLoaded);
    assert_eq!(m.tier(Resident::Summary(NodeId(1))), Tier::Summary);
    assert_eq!(m.evict(1), 25);
    assert_eq!(m.tier(Resident::Summary(NodeId(1))), Tier::NotLoaded);
    assert_eq!(m.tier(Resident::Summary(NodeId(2))), Tier::Summary);
    m.mark_segment_loaded(8, 5);
    assert_eq!(m.evict(u64::MAX), 5 + 26 + 27);
    assert_eq!(m.resident_bytes(), 0);
}
#[test]
fn evict_zero_and_empty_free_nothing() {
    let (_, m) = manager_with(Mode::Ok);
    assert_eq!(m.evict(100), 0);
    m.mark_segment_loaded(1, 100);
    assert_eq!(m.evict(0), 0);
    assert_eq!(m.tier(Resident::Segment(1)), Tier::Vector);
}
#[test]
fn tiers_are_disjoint_between_summary_and_segment() {
    let (_, m) = manager_with(Mode::Ok);
    m.load_summaries(&[NodeId(4)]).unwrap();
    m.mark_segment_loaded(4, 10);
    assert_eq!(m.tier(Resident::Summary(NodeId(4))), Tier::Summary);
    assert_eq!(m.tier(Resident::Segment(4)), Tier::Vector);
}
#[test]
fn manager_survives_concurrent_mixed_use() {
    let (_, m) = manager_with(Mode::Ok);
    std::thread::scope(|scope| {
        for t in 0..4u64 {
            let m = &m;
            scope
                .spawn(move || {
                    for i in 0..200u64 {
                        m.mark_segment_loaded(i % 16, 10);
                        m.touch_segment((i + t) % 16);
                        let _ = m
                            .plan(&EntrySet::from_ranges([0..50u64]), Access::Regular);
                        if i % 25 == 0 {
                            m.evict(30);
                        }
                    }
                });
        }
    });
    assert!(m.resident_bytes() <= 160);
}
