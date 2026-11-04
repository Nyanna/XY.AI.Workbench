use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use futures::future::join_all;
use parking_lot::Mutex;

use super::*;

const GEOMETRY: UniformGeometry = UniformGeometry { entries_per_segment: 100, entry_bytes: 1000 };

fn plan(set: &EntrySet, access: Access) -> Vec<LoadPlan> {
    plan_load(set, &GEOMETRY, &LoadPolicy::default(), access)
}

fn ids(range: std::ops::Range<u64>) -> EntrySet {
    EntrySet::from_ranges([range])
}

#[test]
fn low_density_gathers() {
    let set = ids(0..5);
    assert_eq!(plan(&set, Access::Regular), vec![LoadPlan::Gather(set)]);
}

#[test]
fn high_density_loads_whole_segments() {
    assert_eq!(
        plan(&ids(0..50), Access::Regular),
        vec![LoadPlan::Segments(vec![0])]
    );
    assert_eq!(
        plan(&ids(0..250), Access::Regular),
        vec![LoadPlan::Segments(vec![0, 1, 2])]
    );
}

#[test]
fn mixed_density_splits_the_set() {
    let set = ids(0..50).union(&ids(200..203));
    assert_eq!(
        plan(&set, Access::Regular),
        vec![LoadPlan::Segments(vec![0]), LoadPlan::Gather(ids(200..203))]
    );
}

#[test]
fn tiny_rare_sets_are_mapped() {
    let set = ids(0..2);
    assert_eq!(plan(&set, Access::Rare), vec![LoadPlan::ZeroCopy(set.clone())]);
    assert_eq!(plan(&set, Access::Regular), vec![LoadPlan::Gather(set)]);
}

#[test]
fn rare_but_large_sets_are_not_mapped() {
    let set = ids(0..100);
    assert_eq!(plan(&set, Access::Rare), vec![LoadPlan::Segments(vec![0])]);
}

#[test]
fn threshold_moves_the_decision() {
    let set = ids(0..20);
    let strict = LoadPolicy::default().with_density_threshold(0.5);
    let loose = LoadPolicy::default().with_density_threshold(0.1);
    assert!(matches!(
        plan_load(&set, &GEOMETRY, &strict, Access::Regular)[0],
        LoadPlan::Gather(_)
    ));
    assert!(matches!(
        plan_load(&set, &GEOMETRY, &loose, Access::Regular)[0],
        LoadPlan::Segments(_)
    ));
}

#[test]
fn empty_set_needs_no_load() {
    assert!(plan(&EntrySet::new(), Access::Rare).is_empty());
}

#[test]
fn threshold_is_a_calibration_axis() {
    let cfg = ProfileConfig::new().with(&*DENSITY, ValueId::numeric(300));
    let policy = LoadPolicy::default().apply_profile(&cfg);
    assert!((policy.density_threshold - 0.3).abs() < 1e-6);
    assert_eq!(policy.density_value(), ValueId::numeric(300));

    let untouched = LoadPolicy::default().apply_profile(&ProfileConfig::new());
    assert_eq!(untouched, LoadPolicy::default());
    let garbage = ProfileConfig::new().with(&*DENSITY, ValueId::new("high"));
    assert_eq!(LoadPolicy::default().apply_profile(&garbage), LoadPolicy::default());
}

#[test]
fn threshold_is_clamped() {
    let p = LoadPolicy::default().with_density_threshold(7.0);
    assert_eq!(p.density_threshold, 1.0);
    let p = LoadPolicy::default().with_density_threshold(f32::NAN);
    assert_eq!(p.density_threshold, LoadPolicy::default().density_threshold);
}

#[test]
fn segment_size_follows_the_budget() {
    let budget = |available| ArenaBudget {
        capacity: 4096 * MIB,
        reserve: 0,
        live: 0,
        retired: 0,
        cow_headroom: 0,
        available,
    };
    assert_eq!(LoadPolicy::default().sized_for(&budget(1024 * MIB)).segment_bytes, 128 * MIB);
    assert_eq!(LoadPolicy::default().sized_for(&budget(MIB)).segment_bytes, 4 * MIB);
    assert_eq!(LoadPolicy::default().sized_for(&budget(4000 * MIB)).segment_bytes, 256 * MIB);
}

#[test]
fn aggregation_merges_scopes_per_index() {
    let mut agg = LoadAggregator::new();
    agg.add(1u32, &ids(0..10));
    agg.add(1u32, &ids(5..20));
    agg.add(2u32, &ids(0..3));
    let out = agg.drain();
    assert_eq!(
        out,
        vec![
            Aggregated { index: 1, set: ids(0..20), requests: 2 },
            Aggregated { index: 2, set: ids(0..3), requests: 1 },
        ]
    );
    assert!(agg.is_empty());
    assert!(agg.drain().is_empty());
}

#[derive(Default)]
struct Recorder {
    gathers: Mutex<Vec<EntrySet>>,
    segments: Mutex<Vec<Vec<SegmentId>>>,
    fail: AtomicBool,
}

impl LoadTransfer for Recorder {
    fn load_segments(&self, segments: &[SegmentId]) -> Result<u64, ManagerError> {
        std::thread::sleep(Duration::from_millis(20));
        self.segments.lock().push(segments.to_vec());
        Ok(segments.len() as u64 * 100_000)
    }

    fn load_gather(&self, set: &EntrySet) -> Result<u64, ManagerError> {
        std::thread::sleep(Duration::from_millis(20));
        self.gathers.lock().push(set.clone());
        if self.fail.load(Ordering::SeqCst) {
            return Err(ManagerError::ArenaExhausted { requested: 1, available: 0 });
        }
        Ok(set.len() * 1000)
    }

    fn map_zero_copy(&self, set: &EntrySet) -> Result<u64, ManagerError> {
        Ok(set.len() * 1000)
    }
}

fn loader(recorder: &Arc<Recorder>) -> ResidencyLoader<u32> {
    ResidencyLoader::new(LoadPolicy::default(), Arc::new(GEOMETRY), recorder.clone())
}

#[tokio::test]
async fn identical_requests_share_one_transfer() {
    let recorder = Arc::new(Recorder::default());
    let loader = loader(&recorder);
    let results = join_all(
        (0..4).map(|_| loader.ensure(1, ids(0..5), Access::Regular)),
    )
    .await;
    assert_eq!(recorder.gathers.lock().len(), 1);
    let first = results[0].as_ref().unwrap();
    assert_eq!(first.bytes, 5000);
    for r in &results {
        assert!(Arc::ptr_eq(first, r.as_ref().unwrap()));
    }
    assert_eq!(loader.inflight(), 0);
}

#[tokio::test]
async fn transfer_error_reaches_all_waiters() {
    let recorder = Arc::new(Recorder::default());
    recorder.fail.store(true, Ordering::SeqCst);
    let loader = loader(&recorder);
    let results = join_all(
        (0..4).map(|_| loader.ensure(1, ids(0..5), Access::Regular)),
    )
    .await;
    assert_eq!(recorder.gathers.lock().len(), 1);
    for r in &results {
        assert!(matches!(
            **r.as_ref().unwrap_err(),
            ManagerError::ArenaExhausted { requested: 1, .. }
        ));
    }
    assert_eq!(loader.inflight(), 0);

    recorder.fail.store(false, Ordering::SeqCst);
    assert!(loader.ensure(1, ids(0..5), Access::Regular).await.is_ok());
    assert_eq!(recorder.gathers.lock().len(), 2);
}

#[tokio::test]
async fn different_indexes_load_independently() {
    let recorder = Arc::new(Recorder::default());
    let loader = loader(&recorder);
    let (a, b) = futures::join!(
        loader.ensure(1, ids(0..5), Access::Regular),
        loader.ensure(2, ids(0..5), Access::Regular),
    );
    assert!(a.is_ok() && b.is_ok());
    assert_eq!(recorder.gathers.lock().len(), 2);
}

#[tokio::test]
async fn flush_loads_the_union_once() {
    let recorder = Arc::new(Recorder::default());
    let loader = loader(&recorder);
    let mut agg = LoadAggregator::new();
    agg.add(1u32, &ids(0..5));
    agg.add(1u32, &ids(3..8));
    let out = loader.flush(&mut agg, Access::Regular).await;
    assert_eq!(out.len(), 1);
    assert_eq!(out[0].0.requests, 2);
    assert!(out[0].1.is_ok());
    assert_eq!(*recorder.gathers.lock(), vec![ids(0..8)]);
}

#[tokio::test]
async fn mixed_plan_runs_every_part() {
    let recorder = Arc::new(Recorder::default());
    let loader = loader(&recorder);
    let set = ids(0..50).union(&ids(200..203));
    let out = loader.ensure(1, set, Access::Regular).await.unwrap();
    assert_eq!(out.bytes, 100_000 + 3000);
    assert_eq!(*recorder.segments.lock(), vec![vec![0]]);
    assert_eq!(*recorder.gathers.lock(), vec![ids(200..203)]);
}
