use super::*;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::a::precision_calibration_profiles::batch_compute::{
    BackendKind, ComputeError, CpuBatchCompute,
};
use std::sync::atomic::AtomicUsize;
static_assertions::assert_impl_all!(Precision : Send, Sync);
static_assertions::assert_impl_all!(PrecisionError : Send, Sync);
static_assertions::assert_impl_all!(SubengineId : Send, Sync);
static_assertions::assert_impl_all!(Profile : Send, Sync);
static_assertions::assert_impl_all!(ProfileEntry : Send, Sync);
static_assertions::assert_impl_all!(ProfileTable : Send, Sync);
static_assertions::assert_impl_all!(ProfileError : Send, Sync);
static_assertions::assert_impl_all!(ProfileRegistry : Send, Sync);
static_assertions::assert_impl_all!(BenchError : Send, Sync);
static_assertions::assert_impl_all!(CalibrationJob : Send, Sync);
static_assertions::assert_impl_all!(CalibrationConfig : Send, Sync);
static_assertions::assert_impl_all!(CalibrationStatus : Send, Sync);
static_assertions::assert_impl_all!(CalibrationHandle : Send, Sync, Clone);
static_assertions::assert_impl_all!(Calibrator : Send, Sync);
static_assertions::assert_impl_all!(BatchBench : Send, Sync);
const ID: SubengineId = SubengineId("test");
fn cpu32() -> Profile {
    Profile {
        device: DeviceHandle::Cpu,
        dtype: DType::F32,
    }
}
fn gpu32() -> Profile {
    Profile {
        device: DeviceHandle::Gpu(0),
        dtype: DType::F32,
    }
}
fn gpu16() -> Profile {
    Profile {
        device: DeviceHandle::Gpu(0),
        dtype: DType::F16,
    }
}
fn ms(n: u64) -> Duration {
    Duration::from_millis(n)
}
fn prec(p: f32) -> Precision {
    Precision::new(p).unwrap()
}
fn m(profile: Profile, cost: Duration) -> Measured {
    (profile, cost, None)
}
fn md(profile: Profile, cost: Duration, deviation: f64) -> Measured {
    (profile, cost, Some(deviation))
}
#[test]
fn precision_new_accepts_only_unit_interval() {
    assert!(Precision::new(0.0).is_ok());
    assert!(Precision::new(1.0).is_ok());
    assert!(Precision::new(0.3).is_ok());
    for bad in [-0.001, 1.001, f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
        assert_eq!(Precision::new(bad), Err(PrecisionError::OutOfRange));
    }
    assert_eq!(Precision::new(- 0.0).unwrap().get(), 0.0);
}
#[test]
fn precision_clamped_never_panics() {
    assert_eq!(Precision::clamped(- 5.0).get(), 0.0);
    assert_eq!(Precision::clamped(7.0).get(), 1.0);
    assert_eq!(Precision::clamped(f32::NAN).get(), 0.5);
    assert_eq!(Precision::clamped(f32::INFINITY).get(), 1.0);
    assert_eq!(Precision::clamped(f32::NEG_INFINITY).get(), 0.0);
    assert_eq!(Precision::clamped(0.25).get(), 0.25);
}
#[test]
fn precision_constants_default_and_display() {
    assert_eq!(Precision::FASTEST.get(), 0.0);
    assert_eq!(Precision::MOST_PRECISE.get(), 1.0);
    assert_eq!(Precision::default().get(), 0.5);
    assert!(Precision::FASTEST < Precision::MOST_PRECISE);
    assert!(! PrecisionError::OutOfRange.to_string().is_empty());
}
#[test]
fn precision_levels() {
    assert_eq!(Profile::precision_level(DType::F16), Some(1));
    assert_eq!(Profile::precision_level(DType::F32), Some(2));
    for t in [DType::BF16, DType::U8, DType::U32, DType::I64] {
        assert_eq!(Profile::precision_level(t), None);
    }
}
#[test]
fn candidates_for_cpu_and_gpu() {
    assert_eq!(candidates_for(& [DeviceHandle::Cpu]), vec![cpu32()]);
    assert_eq!(
        candidates_for(& [DeviceHandle::Cpu, DeviceHandle::Gpu(0)]), vec![cpu32(),
        gpu32(), gpu16()]
    );
    assert!(candidates_for(& []).is_empty());
}
#[test]
fn candidates_for_dedups_devices() {
    let c = candidates_for(
        &[DeviceHandle::Gpu(0), DeviceHandle::Cpu, DeviceHandle::Gpu(0)],
    );
    assert_eq!(c, vec![gpu32(), gpu16(), cpu32()]);
    assert!(c.iter().all(| p | p.dtype != DType::BF16));
}
#[test]
fn default_table_prefers_gpu_and_one_per_level() {
    let t = ProfileTable::from_candidates(&[cpu32(), gpu32(), gpu16()]).unwrap();
    assert!(! t.calibrated());
    let e = t.entries();
    assert_eq!(e.len(), 2);
    assert_eq!((e[0].profile, e[0].level, e[0].cost), (gpu16(), 1, None));
    assert_eq!((e[1].profile, e[1].level, e[1].cost), (gpu32(), 2, None));
}
#[test]
fn default_table_cpu_only_and_input_order() {
    let t = ProfileTable::from_candidates(&[cpu32()]).unwrap();
    assert_eq!(t.entries().len(), 1);
    let other = Profile {
        device: DeviceHandle::Gpu(1),
        dtype: DType::F32,
    };
    let t = ProfileTable::from_candidates(&[other, gpu32()]).unwrap();
    assert_eq!(t.entries() [0].profile, other);
}
#[test]
fn default_table_rejects_invalid_and_empty() {
    let bad = Profile {
        device: DeviceHandle::Cpu,
        dtype: DType::BF16,
    };
    assert!(ProfileTable::from_candidates(& []).is_none());
    assert!(ProfileTable::from_candidates(& [bad]).is_none());
    let t = ProfileTable::from_candidates(&[bad, cpu32(), cpu32()]).unwrap();
    assert_eq!(t.entries().len(), 1);
}
#[test]
fn calibrated_table_builds_pareto_front() {
    let t = ProfileTable::from_measurements(
            &[m(cpu32(), ms(50)), m(gpu32(), ms(10)), m(gpu16(), ms(5))],
        )
        .unwrap();
    assert!(t.calibrated());
    let e = t.entries();
    assert_eq!(e.len(), 2);
    assert_eq!((e[0].profile, e[0].cost), (gpu16(), Some(ms(5))));
    assert_eq!((e[1].profile, e[1].cost), (gpu32(), Some(ms(10))));
}
#[test]
fn calibrated_table_drops_dominated_lower_precision() {
    let t = ProfileTable::from_measurements(&[m(gpu16(), ms(20)), m(gpu32(), ms(10))])
        .unwrap();
    assert_eq!(t.entries().len(), 1);
    assert_eq!(t.entries() [0].profile, gpu32());
}
#[test]
fn calibrated_table_equal_points_keep_first() {
    let t = ProfileTable::from_measurements(&[m(cpu32(), ms(10)), m(gpu32(), ms(10))])
        .unwrap();
    assert_eq!(t.entries().len(), 1);
    assert_eq!(t.entries() [0].profile, cpu32());
}
#[test]
fn calibrated_table_is_strictly_ordered_and_keeps_fastest() {
    let t = ProfileTable::from_measurements(
            &[m(gpu32(), ms(30)), m(gpu16(), ms(5)), m(cpu32(), ms(40))],
        )
        .unwrap();
    let e = t.entries();
    assert_eq!(e[0].profile, gpu16());
    assert_eq!(e.last().unwrap().profile, gpu32());
    for w in e.windows(2) {
        assert!(w[0].level < w[1].level);
        assert!(w[0].cost < w[1].cost);
    }
    assert!(ProfileTable::from_measurements(& []).is_none());
}
#[test]
fn measured_deviation_overrides_nominal_level() {
    let t = ProfileTable::from_measurements(
            &[md(gpu32(), ms(10), 1e-3), md(gpu16(), ms(5), 1e-6)],
        )
        .unwrap();
    assert!(t.calibrated());
    assert_eq!(t.entries().len(), 1);
    assert_eq!(t.entries() [0].profile, gpu16());
    assert_eq!(t.pick(Precision::MOST_PRECISE), gpu16());
}
#[test]
fn deviation_table_keeps_slower_but_more_precise_type() {
    let t = ProfileTable::from_measurements(
            &[md(gpu32(), ms(5), 1e-3), md(gpu16(), ms(10), 1e-6)],
        )
        .unwrap();
    let e = t.entries();
    assert_eq!(e.len(), 2);
    assert_eq!((e[0].profile, e[0].level, e[0].deviation), (gpu32(), 1, Some(1e-3)));
    assert_eq!((e[1].profile, e[1].level, e[1].deviation), (gpu16(), 2, Some(1e-6)));
    assert_eq!(t.pick(Precision::FASTEST), gpu32());
    assert_eq!(t.pick(Precision::MOST_PRECISE), gpu16());
}
#[test]
fn deviation_table_is_ordered_by_time_and_precision() {
    let t = ProfileTable::from_measurements(
            &[
                md(cpu32(), ms(50), 1e-7),
                md(gpu32(), ms(10), 1e-6),
                md(gpu16(), ms(5), 1e-3),
                md(
                    Profile {
                        device: DeviceHandle::Gpu(1),
                        dtype: DType::F32,
                    },
                    ms(20),
                    1e-2,
                ),
            ],
        )
        .unwrap();
    let profiles: Vec<Profile> = t.entries().iter().map(|e| e.profile).collect();
    assert_eq!(profiles, vec![gpu16(), gpu32(), cpu32()]);
    for w in t.entries().windows(2) {
        assert!(w[0].cost < w[1].cost);
        assert!(w[0].deviation > w[1].deviation);
        assert!(w[0].level < w[1].level);
    }
    assert_eq!(t.pick(Precision::FASTEST), gpu16());
    assert_eq!(t.pick(Precision::MOST_PRECISE), cpu32());
}
#[test]
fn deviation_table_equal_deviation_keeps_faster() {
    let t = ProfileTable::from_measurements(
            &[md(cpu32(), ms(20), 0.0), md(gpu32(), ms(10), 0.0)],
        )
        .unwrap();
    assert_eq!(t.entries().len(), 1);
    assert_eq!(t.entries() [0].profile, gpu32());
}
#[test]
fn partial_or_invalid_deviation_falls_back_to_levels() {
    let partial = ProfileTable::from_measurements(
            &[m(gpu32(), ms(5)), md(gpu16(), ms(10), 1e-6)],
        )
        .unwrap();
    assert_eq!(partial.entries().len(), 1);
    assert_eq!(partial.entries() [0].profile, gpu32());
    for bad in [f64::NAN, f64::INFINITY, -1.0] {
        let t = ProfileTable::from_measurements(
                &[md(gpu32(), ms(5), bad), md(gpu16(), ms(10), 0.1)],
            )
            .unwrap();
        assert_eq!(t.entries().len(), 1);
        assert_eq!(t.entries() [0].profile, gpu32());
    }
}
#[test]
fn pick_endpoints_and_rounding() {
    let t = ProfileTable::from_measurements(&[m(gpu16(), ms(1)), m(gpu32(), ms(2))])
        .unwrap();
    assert_eq!(t.pick(Precision::FASTEST), gpu16());
    assert_eq!(t.pick(Precision::MOST_PRECISE), gpu32());
    assert_eq!(t.pick(prec(0.49)), gpu16());
    assert_eq!(t.pick(prec(0.5)), gpu32());
    let single = ProfileTable::from_candidates(&[cpu32()]).unwrap();
    for p in [0.0, 0.3, 0.5, 1.0] {
        assert_eq!(single.pick(prec(p)), cpu32());
    }
}
#[test]
fn pick_is_monotone_and_covers_three_entries() {
    let entry = |profile, level| ProfileEntry {
        profile,
        level,
        cost: None,
        deviation: None,
    };
    let table = ProfileTable {
        entries: vec![entry(gpu16(), 1), entry(cpu32(), 2), entry(gpu32(), 3)],
        calibrated: false,
    };
    let mut last = 0u8;
    for i in 0..=100 {
        let p = prec(i as f32 / 100.0);
        let picked = table.pick(p);
        let level = table.entries().iter().find(|e| e.profile == picked).unwrap().level;
        assert!(level >= last);
        last = level;
    }
    assert_eq!(table.pick(prec(0.25)), cpu32());
    assert_eq!(table.pick(prec(0.24)), gpu16());
}
#[test]
fn registry_unknown_and_register() {
    let r = ProfileRegistry::new();
    assert_eq!(
        r.select(ID, Precision::default()), Err(ProfileError::UnknownSubengine(ID))
    );
    assert!(r.table(ID).is_none());
    assert_eq!(r.register(ID, & []), Err(ProfileError::NoCandidates));
    let bad = Profile {
        device: DeviceHandle::Cpu,
        dtype: DType::I64,
    };
    assert_eq!(r.register(ID, & [bad]), Err(ProfileError::NoCandidates));
    r.register(ID, &candidates_for(&[DeviceHandle::Cpu, DeviceHandle::Gpu(0)])).unwrap();
    assert_eq!(r.select(ID, Precision::FASTEST), Ok(gpu16()));
    assert_eq!(r.select(ID, Precision::MOST_PRECISE), Ok(gpu32()));
    assert!(! r.table(ID).unwrap().calibrated());
    assert!(! ProfileError::NoCandidates.to_string().is_empty());
    assert!(ProfileError::UnknownSubengine(ID).to_string().contains("test"));
}
#[test]
fn register_replaces_calibrated_table() {
    let r = ProfileRegistry::default();
    r.install(ID, ProfileTable::from_measurements(&[m(cpu32(), ms(1))]).unwrap());
    assert!(r.table(ID).unwrap().calibrated());
    r.register(ID, &[cpu32(), gpu16()]).unwrap();
    assert!(! r.table(ID).unwrap().calibrated());
}
#[test]
fn registry_survives_poisoned_lock() {
    let r = Arc::new(ProfileRegistry::new());
    r.register(ID, &[cpu32()]).unwrap();
    let r2 = r.clone();
    let _ = std::thread::spawn(move || {
            let _guard = r2.tables.write().unwrap();
            panic!("poison");
        })
        .join();
    assert!(r.tables.is_poisoned());
    assert_eq!(r.select(ID, Precision::default()), Ok(cpu32()));
    r.register(SubengineId("other"), &[cpu32()]).unwrap();
}
#[test]
fn bench_error_display() {
    assert!(! BenchError::Unsupported.to_string().is_empty());
    assert!(BenchError::Failed("x".into()).to_string().contains('x'));
}
struct FnTarget<F: Fn(&Profile) -> Result<Option<Event>, BenchError> + Send + Sync>(F);
impl<F: Fn(&Profile) -> Result<Option<Event>, BenchError> + Send + Sync> BenchTarget
for FnTarget<F> {
    fn run(&self, profile: &Profile) -> Result<Option<Event>, BenchError> {
        (self.0)(profile)
    }
}
fn job<F>(id: SubengineId, candidates: Vec<Profile>, f: F) -> CalibrationJob
where
    F: Fn(&Profile) -> Result<Option<Event>, BenchError> + Send + Sync + 'static,
{
    CalibrationJob {
        id,
        candidates,
        target: Arc::new(FnTarget(f)),
    }
}
struct DevTarget {
    delay_ms: fn(&Profile) -> u64,
    deviation: fn(&Profile) -> Result<Option<f64>, BenchError>,
}
impl BenchTarget for DevTarget {
    fn run(&self, profile: &Profile) -> Result<Option<Event>, BenchError> {
        std::thread::sleep(ms((self.delay_ms)(profile)));
        Ok(None)
    }
    fn deviation(&self, profile: &Profile) -> Result<Option<f64>, BenchError> {
        (self.deviation)(profile)
    }
}
fn dev_job(candidates: Vec<Profile>, target: DevTarget) -> CalibrationJob {
    CalibrationJob {
        id: ID,
        candidates,
        target: Arc::new(target),
    }
}
fn fast_cfg() -> CalibrationConfig {
    CalibrationConfig {
        warmup_runs: 1,
        measured_runs: 3,
        budget_per_candidate: Duration::from_secs(5),
    }
}
fn finish(h: &CalibrationHandle) -> CalibrationStatus {
    h.done().wait().unwrap();
    h.status()
}
#[test]
fn config_defaults() {
    let c = CalibrationConfig::default();
    assert_eq!(c.warmup_runs, 1);
    assert_eq!(c.measured_runs, 5);
    assert_eq!(c.budget_per_candidate, ms(200));
}
#[test]
fn no_jobs_finishes_immediately() {
    let h = Calibrator::start(Arc::new(ProfileRegistry::new()), vec![], fast_cfg());
    assert_eq!(h.status(), CalibrationStatus::Finished);
    assert_eq!(h.done().is_complete(), Ok(true));
}
#[test]
fn calibration_installs_table_and_registers_unknown_id() {
    let reg = Arc::new(ProfileRegistry::new());
    let job = job(
        ID,
        vec![gpu32(), gpu16(), cpu32()],
        |p| {
            let delay = match (p.device, p.dtype) {
                (DeviceHandle::Cpu, _) => 8,
                (_, DType::F32) => 4,
                _ => 1,
            };
            std::thread::sleep(ms(delay));
            Ok(None)
        },
    );
    let h = Calibrator::start(reg.clone(), vec![job], fast_cfg());
    assert_eq!(finish(& h), CalibrationStatus::Finished);
    assert_eq!(h.done().is_complete(), Ok(true));
    let t = reg.table(ID).unwrap();
    assert!(t.calibrated());
    let profiles: Vec<Profile> = t.entries().iter().map(|e| e.profile).collect();
    assert_eq!(profiles, vec![gpu16(), gpu32()]);
    assert_eq!(reg.select(ID, Precision::MOST_PRECISE), Ok(gpu32()));
}
#[test]
fn warmup_and_measured_run_counts() {
    let count = Arc::new(AtomicUsize::new(0));
    let c = count.clone();
    let job = job(
        ID,
        vec![cpu32()],
        move |_| {
            c.fetch_add(1, Ordering::SeqCst);
            Ok(None)
        },
    );
    let cfg = CalibrationConfig {
        warmup_runs: 2,
        measured_runs: 4,
        budget_per_candidate: Duration::from_secs(5),
    };
    let h = Calibrator::start(Arc::new(ProfileRegistry::new()), vec![job], cfg);
    assert_eq!(finish(& h), CalibrationStatus::Finished);
    assert_eq!(count.load(Ordering::SeqCst), 6);
}
#[test]
fn zero_measured_runs_still_measures_once() {
    let count = Arc::new(AtomicUsize::new(0));
    let c = count.clone();
    let job = job(
        ID,
        vec![cpu32()],
        move |_| {
            c.fetch_add(1, Ordering::SeqCst);
            Ok(None)
        },
    );
    let cfg = CalibrationConfig {
        warmup_runs: 0,
        measured_runs: 0,
        budget_per_candidate: Duration::ZERO,
    };
    let reg = Arc::new(ProfileRegistry::new());
    let h = Calibrator::start(reg.clone(), vec![job], cfg);
    assert_eq!(finish(& h), CalibrationStatus::Finished);
    assert_eq!(count.load(Ordering::SeqCst), 1);
    assert!(reg.table(ID).unwrap().calibrated());
}
#[test]
fn budget_limits_measured_runs() {
    let count = Arc::new(AtomicUsize::new(0));
    let c = count.clone();
    let job = job(
        ID,
        vec![cpu32()],
        move |_| {
            c.fetch_add(1, Ordering::SeqCst);
            std::thread::sleep(ms(20));
            Ok(None)
        },
    );
    let cfg = CalibrationConfig {
        warmup_runs: 0,
        measured_runs: 1000,
        budget_per_candidate: ms(50),
    };
    let h = Calibrator::start(Arc::new(ProfileRegistry::new()), vec![job], cfg);
    assert_eq!(finish(& h), CalibrationStatus::Finished);
    let n = count.load(Ordering::SeqCst);
    assert!((1..= 10).contains(& n), "runs: {n}");
}
#[test]
fn cost_is_median() {
    let calls = Arc::new(AtomicUsize::new(0));
    let c = calls.clone();
    let job = job(
        ID,
        vec![cpu32()],
        move |_| {
            let i = c.fetch_add(1, Ordering::SeqCst);
            let delay = match i {
                0 => 0,
                1 => 40,
                2 => 5,
                _ => 5,
            };
            std::thread::sleep(ms(delay));
            Ok(None)
        },
    );
    let reg = Arc::new(ProfileRegistry::new());
    let h = Calibrator::start(reg.clone(), vec![job], fast_cfg());
    assert_eq!(finish(& h), CalibrationStatus::Finished);
    let cost = reg.table(ID).unwrap().entries()[0].cost.unwrap();
    assert!(cost >= ms(5) && cost < ms(30), "cost: {cost:?}");
}
#[test]
fn median_helper() {
    assert_eq!(median_duration(& [ms(3), ms(1), ms(2)]), ms(2));
    assert_eq!(median_duration(& [ms(1), ms(3)]), ms(2));
    assert_eq!(median_duration(& [ms(7)]), ms(7));
    assert_eq!(median_duration(& []), Duration::ZERO);
}
#[test]
fn async_event_time_includes_wait() {
    let job = job(
        ID,
        vec![cpu32()],
        |_| {
            let (event, signal) = Event::cpu();
            std::thread::spawn(move || {
                std::thread::sleep(ms(30));
                signal.signal();
            });
            Ok(Some(event))
        },
    );
    let reg = Arc::new(ProfileRegistry::new());
    let cfg = CalibrationConfig {
        warmup_runs: 0,
        measured_runs: 1,
        budget_per_candidate: Duration::from_secs(5),
    };
    let h = Calibrator::start(reg.clone(), vec![job], cfg);
    assert_eq!(finish(& h), CalibrationStatus::Finished);
    assert!(reg.table(ID).unwrap().entries() [0].cost.unwrap() >= ms(25));
}
#[test]
fn failing_event_excludes_candidate() {
    let job = job(
        ID,
        vec![cpu32(), gpu32()],
        |p| {
            if p.device == DeviceHandle::Cpu {
                let (event, control) = Event::mock();
                control.fail("boom");
                Ok(Some(event))
            } else {
                Ok(None)
            }
        },
    );
    let reg = Arc::new(ProfileRegistry::new());
    let h = Calibrator::start(reg.clone(), vec![job], fast_cfg());
    assert_eq!(finish(& h), CalibrationStatus::Finished);
    let t = reg.table(ID).unwrap();
    assert_eq!(t.entries().len(), 1);
    assert_eq!(t.entries() [0].profile, gpu32());
}
#[test]
fn errors_and_panics_exclude_candidates() {
    let job = job(
        ID,
        vec![cpu32(), gpu32(), gpu16()],
        |p| match (p.device, p.dtype) {
            (DeviceHandle::Cpu, _) => Err(BenchError::Failed("x".into())),
            (_, DType::F32) => panic!("bench panic"),
            _ => Ok(None),
        },
    );
    let reg = Arc::new(ProfileRegistry::new());
    let h = Calibrator::start(reg.clone(), vec![job], fast_cfg());
    assert_eq!(finish(& h), CalibrationStatus::Finished);
    let t = reg.table(ID).unwrap();
    assert_eq!(t.entries().len(), 1);
    assert_eq!(t.entries() [0].profile, gpu16());
}
#[test]
fn nothing_measured_keeps_existing_table() {
    let reg = Arc::new(ProfileRegistry::new());
    reg.register(ID, &[cpu32(), gpu16()]).unwrap();
    let before = reg.table(ID).unwrap();
    let job = job(ID, vec![cpu32(), gpu16()], |_| Err(BenchError::Unsupported));
    let h = Calibrator::start(reg.clone(), vec![job], fast_cfg());
    assert_eq!(finish(& h), CalibrationStatus::Finished);
    assert!(Arc::ptr_eq(& before, & reg.table(ID).unwrap()));
}
#[test]
fn nothing_measured_installs_default_when_missing() {
    let reg = Arc::new(ProfileRegistry::new());
    let job = job(ID, vec![cpu32(), gpu16(), cpu32()], |_| Err(BenchError::Unsupported));
    let h = Calibrator::start(reg.clone(), vec![job], fast_cfg());
    assert_eq!(finish(& h), CalibrationStatus::Finished);
    let t = reg.table(ID).unwrap();
    assert!(! t.calibrated());
    assert_eq!(t.entries().len(), 2);
}
#[test]
fn jobs_run_in_order() {
    let order = Arc::new(Mutex::new(Vec::new()));
    let (o1, o2) = (order.clone(), order.clone());
    let j1 = job(
        SubengineId("a"),
        vec![cpu32()],
        move |_| {
            lock(&o1).push("a");
            Ok(None)
        },
    );
    let j2 = job(
        SubengineId("b"),
        vec![cpu32()],
        move |_| {
            lock(&o2).push("b");
            Ok(None)
        },
    );
    let cfg = CalibrationConfig {
        warmup_runs: 0,
        measured_runs: 1,
        budget_per_candidate: ms(10),
    };
    let reg = Arc::new(ProfileRegistry::new());
    let h = Calibrator::start(reg.clone(), vec![j1, j2], cfg);
    assert_eq!(finish(& h), CalibrationStatus::Finished);
    assert_eq!(* lock(& order), vec!["a", "b"]);
    assert!(
        reg.table(SubengineId("a")).is_some() && reg.table(SubengineId("b")).is_some()
    );
}
#[test]
fn select_does_not_block_during_calibration() {
    let reg = Arc::new(ProfileRegistry::new());
    reg.register(ID, &[cpu32(), gpu16()]).unwrap();
    let (gate, control) = Event::mock();
    let job = job(ID, vec![cpu32()], move |_| Ok(Some(gate.clone())));
    let h = Calibrator::start(reg.clone(), vec![job], fast_cfg());
    assert_eq!(h.status(), CalibrationStatus::Running);
    assert_eq!(h.done().is_complete(), Ok(false));
    assert_eq!(reg.select(ID, Precision::FASTEST), Ok(gpu16()));
    assert!(! reg.table(ID).unwrap().calibrated());
    control.complete();
    assert_eq!(finish(& h), CalibrationStatus::Finished);
    assert!(reg.table(ID).unwrap().calibrated());
}
#[test]
fn cancel_is_idempotent_and_skips_write() {
    let reg = Arc::new(ProfileRegistry::new());
    let (gate, control) = Event::mock();
    let first = job(ID, vec![cpu32()], move |_| Ok(Some(gate.clone())));
    let second = job(SubengineId("second"), vec![cpu32()], |_| Ok(None));
    let h = Calibrator::start(reg.clone(), vec![first, second], fast_cfg());
    h.cancel();
    h.cancel();
    h.clone().cancel();
    assert_eq!(h.status(), CalibrationStatus::Running);
    control.complete();
    assert_eq!(finish(& h), CalibrationStatus::Cancelled);
    assert!(reg.table(ID).is_none());
    assert!(reg.table(SubengineId("second")).is_none());
    h.cancel();
    assert_eq!(h.status(), CalibrationStatus::Cancelled);
}
#[test]
fn cancel_keeps_tables_already_written() {
    let reg = Arc::new(ProfileRegistry::new());
    let (gate, control) = Event::mock();
    let first = job(SubengineId("first"), vec![cpu32()], |_| Ok(None));
    let second = job(ID, vec![cpu32()], move |_| Ok(Some(gate.clone())));
    let h = Calibrator::start(reg.clone(), vec![first, second], fast_cfg());
    while reg.table(SubengineId("first")).is_none() {
        std::thread::yield_now();
    }
    h.cancel();
    control.complete();
    assert_eq!(finish(& h), CalibrationStatus::Cancelled);
    assert!(reg.table(SubengineId("first")).is_some());
    assert!(reg.table(ID).is_none());
}
#[test]
fn dropping_handle_does_not_cancel() {
    let reg = Arc::new(ProfileRegistry::new());
    let (gate, control) = Event::mock();
    let j = job(ID, vec![cpu32()], move |_| Ok(Some(gate.clone())));
    let h = Calibrator::start(reg.clone(), vec![j], fast_cfg());
    let done = h.done();
    drop(h);
    control.complete();
    done.wait().unwrap();
    while reg.table(ID).is_none() {
        std::thread::yield_now();
    }
    assert!(reg.table(ID).unwrap().calibrated());
}
#[test]
fn parallel_calibrations_last_writer_wins() {
    let reg = Arc::new(ProfileRegistry::new());
    let mk = || job(ID, vec![cpu32()], |_| Ok(None));
    let a = Calibrator::start(reg.clone(), vec![mk()], fast_cfg());
    let b = Calibrator::start(reg.clone(), vec![mk()], fast_cfg());
    assert_eq!(finish(& a), CalibrationStatus::Finished);
    assert_eq!(finish(& b), CalibrationStatus::Finished);
    assert!(reg.table(ID).unwrap().calibrated());
}
#[test]
fn concurrent_select_during_table_swaps() {
    let reg = Arc::new(ProfileRegistry::new());
    reg.register(ID, &[cpu32(), gpu32(), gpu16()]).unwrap();
    let stop = Arc::new(AtomicBool::new(false));
    let readers: Vec<_> = (0..4)
        .map(|_| {
            let (reg, stop) = (reg.clone(), stop.clone());
            std::thread::spawn(move || {
                while !stop.load(Ordering::Relaxed) {
                    let t = reg.table(ID).unwrap();
                    let p = reg.select(ID, Precision::MOST_PRECISE).unwrap();
                    assert!(
                        candidates_for(& [DeviceHandle::Cpu, DeviceHandle::Gpu(0)])
                        .contains(& p)
                    );
                    assert!(! t.entries().is_empty());
                }
            })
        })
        .collect();
    for _ in 0..50 {
        let h = Calibrator::start(
            reg.clone(),
            vec![job(ID, vec![cpu32(), gpu16()], | _ | Ok(None))],
            fast_cfg(),
        );
        finish(&h);
        reg.register(ID, &[cpu32(), gpu32(), gpu16()]).unwrap();
    }
    stop.store(true, Ordering::Relaxed);
    for r in readers {
        r.join().unwrap();
    }
}
struct FailingBackend;
impl BatchCompute for FailingBackend {
    fn kind(&self) -> BackendKind {
        BackendKind::Gpu
    }
    fn run(
        &self,
        _: BatchKernel,
        _: &[&FloatBatch],
    ) -> Result<FloatBatch, ComputeError> {
        Err(ComputeError::Backend("broken".into()))
    }
}
struct CountingBackend(AtomicUsize);
impl BatchCompute for CountingBackend {
    fn kind(&self) -> BackendKind {
        BackendKind::Gpu
    }
    fn run(
        &self,
        kernel: BatchKernel,
        inputs: &[&FloatBatch],
    ) -> Result<FloatBatch, ComputeError> {
        assert_eq!(kernel, BatchKernel::MatMul);
        assert_eq!(inputs.len(), 2);
        for b in inputs {
            assert_eq!((b.rows(), b.cols()), (64, 64));
        }
        self.0.fetch_add(1, Ordering::SeqCst);
        CpuBatchCompute::new().run(kernel, inputs)
    }
}
#[test]
fn batch_bench_cpu_runs_matmul() {
    let bench = BatchBench::new(Arc::new(CpuBatchCompute::new()), None);
    assert!(matches!(bench.run(& cpu32()), Ok(None)));
}
#[test]
fn batch_bench_mapping_and_unsupported() {
    let gpu = Arc::new(CountingBackend(AtomicUsize::new(0)));
    let bench = BatchBench::new(
        Arc::new(CpuBatchCompute::new()),
        Some((0, gpu.clone())),
    );
    assert!(matches!(bench.run(& gpu32()), Ok(None)));
    assert_eq!(gpu.0.load(Ordering::SeqCst), 1);
    assert!(matches!(bench.run(& gpu16()), Err(BenchError::Unsupported)));
    let other = Profile {
        device: DeviceHandle::Gpu(3),
        dtype: DType::F32,
    };
    assert!(matches!(bench.run(& other), Err(BenchError::Unsupported)));
    let cpu_f16 = Profile {
        device: DeviceHandle::Cpu,
        dtype: DType::F16,
    };
    assert!(matches!(bench.run(& cpu_f16), Err(BenchError::Unsupported)));
    let no_gpu = BatchBench::new(Arc::new(CpuBatchCompute::new()), None);
    assert!(matches!(no_gpu.run(& gpu32()), Err(BenchError::Unsupported)));
}
#[test]
fn batch_bench_maps_compute_error() {
    let bench = BatchBench::new(
        Arc::new(CpuBatchCompute::new()),
        Some((0, Arc::new(FailingBackend))),
    );
    match bench.run(&gpu32()) {
        Err(BenchError::Failed(m)) => assert!(m.contains("broken")),
        other => panic!("unexpected: {other:?}"),
    }
}
#[test]
fn batch_bench_calibration_omits_f16() {
    let gpu = Arc::new(CountingBackend(AtomicUsize::new(0)));
    let bench = Arc::new(
        BatchBench::new(Arc::new(CpuBatchCompute::new()), Some((0, gpu))),
    );
    let candidates = candidates_for(&[DeviceHandle::Cpu, DeviceHandle::Gpu(0)]);
    let reg = Arc::new(ProfileRegistry::new());
    let j = CalibrationJob {
        id: ID,
        candidates,
        target: bench,
    };
    let h = Calibrator::start(reg.clone(), vec![j], fast_cfg());
    assert_eq!(finish(& h), CalibrationStatus::Finished);
    let t = reg.table(ID).unwrap();
    assert!(t.calibrated());
    assert!(t.entries().iter().all(| e | e.profile.dtype == DType::F32));
    assert_eq!(t.entries().len(), 1);
}
#[test]
fn calibration_uses_measured_deviation_over_nominal_level() {
    let target = DevTarget {
        delay_ms: |p| if p.dtype == DType::F32 { 1 } else { 15 },
        deviation: |p| Ok(Some(if p.dtype == DType::F32 { 1e-3 } else { 1e-6 })),
    };
    let reg = Arc::new(ProfileRegistry::new());
    let h = Calibrator::start(
        reg.clone(),
        vec![dev_job(vec![gpu32(), gpu16()], target)],
        fast_cfg(),
    );
    assert_eq!(finish(& h), CalibrationStatus::Finished);
    let t = reg.table(ID).unwrap();
    let profiles: Vec<Profile> = t.entries().iter().map(|e| e.profile).collect();
    assert_eq!(profiles, vec![gpu32(), gpu16()]);
    assert!(t.entries().iter().all(| e | e.deviation.is_some() && e.cost.is_some()));
    assert_eq!(reg.select(ID, Precision::MOST_PRECISE), Ok(gpu16()));
    assert_eq!(reg.select(ID, Precision::FASTEST), Ok(gpu32()));
}
#[test]
fn failing_or_panicking_deviation_keeps_candidate_measured() {
    let failing = DevTarget {
        delay_ms: |_| 0,
        deviation: |_| Err(BenchError::Failed("no reference".into())),
    };
    let panicking = DevTarget {
        delay_ms: |_| 0,
        deviation: |_| panic!("deviation panic"),
    };
    for target in [failing, panicking] {
        let reg = Arc::new(ProfileRegistry::new());
        let h = Calibrator::start(
            reg.clone(),
            vec![dev_job(vec![cpu32()], target)],
            fast_cfg(),
        );
        assert_eq!(finish(& h), CalibrationStatus::Finished);
        let t = reg.table(ID).unwrap();
        assert!(t.calibrated());
        assert_eq!(t.entries() [0].profile, cpu32());
        assert!(t.entries() [0].cost.is_some());
        assert_eq!(t.entries() [0].deviation, None);
    }
}
#[test]
fn invalid_deviation_is_treated_as_unknown() {
    let target = DevTarget {
        delay_ms: |_| 0,
        deviation: |p| Ok(Some(if p.dtype == DType::F32 { f64::NAN } else { -1.0 })),
    };
    let reg = Arc::new(ProfileRegistry::new());
    let h = Calibrator::start(
        reg.clone(),
        vec![dev_job(vec![gpu32(), gpu16()], target)],
        fast_cfg(),
    );
    assert_eq!(finish(& h), CalibrationStatus::Finished);
    assert!(reg.table(ID).unwrap().entries().iter().all(| e | e.deviation.is_none()));
}
struct ZeroBackend;
impl BatchCompute for ZeroBackend {
    fn kind(&self) -> BackendKind {
        BackendKind::Gpu
    }
    fn run(
        &self,
        _: BatchKernel,
        _: &[&FloatBatch],
    ) -> Result<FloatBatch, ComputeError> {
        FloatBatch::new(64, 64, vec![0.0; 64 * 64])
    }
}
struct ShortBackend;
impl BatchCompute for ShortBackend {
    fn kind(&self) -> BackendKind {
        BackendKind::Gpu
    }
    fn run(
        &self,
        _: BatchKernel,
        _: &[&FloatBatch],
    ) -> Result<FloatBatch, ComputeError> {
        FloatBatch::new(1, 1, vec![0.0])
    }
}
#[test]
fn batch_bench_deviation_is_small_for_a_correct_backend() {
    let bench = BatchBench::new(Arc::new(CpuBatchCompute::new()), None);
    let d = bench.deviation(&cpu32()).unwrap().unwrap();
    assert!(d.is_finite() && d < 1e-5, "deviation: {d}");
}
#[test]
fn batch_bench_deviation_detects_wrong_results() {
    let bench = BatchBench::new(
        Arc::new(CpuBatchCompute::new()),
        Some((0, Arc::new(ZeroBackend))),
    );
    let d = bench.deviation(&gpu32()).unwrap().unwrap();
    assert!((d - 1.0).abs() < 1e-9, "deviation: {d}");
    let short = BatchBench::new(
        Arc::new(CpuBatchCompute::new()),
        Some((0, Arc::new(ShortBackend))),
    );
    assert!(matches!(short.deviation(& gpu32()), Err(BenchError::Failed(_))));
}
#[test]
fn batch_bench_deviation_follows_run_support() {
    let bench = BatchBench::new(
        Arc::new(CpuBatchCompute::new()),
        Some((0, Arc::new(FailingBackend))),
    );
    assert!(matches!(bench.deviation(& gpu16()), Err(BenchError::Unsupported)));
    let other = Profile {
        device: DeviceHandle::Gpu(3),
        dtype: DType::F32,
    };
    assert!(matches!(bench.deviation(& other), Err(BenchError::Unsupported)));
    assert!(matches!(bench.deviation(& gpu32()), Err(BenchError::Failed(_))));
}
#[test]
fn batch_bench_calibration_ranks_a_wrong_backend_as_less_precise() {
    let bench = Arc::new(
        BatchBench::new(
            Arc::new(CpuBatchCompute::new()),
            Some((0, Arc::new(ZeroBackend))),
        ),
    );
    let reg = Arc::new(ProfileRegistry::new());
    let j = CalibrationJob {
        id: ID,
        candidates: vec![gpu32(), cpu32()],
        target: bench,
    };
    let h = Calibrator::start(reg.clone(), vec![j], fast_cfg());
    assert_eq!(finish(& h), CalibrationStatus::Finished);
    let t = reg.table(ID).unwrap();
    assert!(t.entries().iter().all(| e | e.deviation.is_some()));
    assert_eq!(t.pick(Precision::MOST_PRECISE), cpu32());
}
