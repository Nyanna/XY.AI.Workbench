use super::*;
use crate::resident::memory::ArenaId;
use crate::resident::subengine::{LaneConfig, RecordingLane};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::atomic::AtomicUsize;
fn region(gen: u32) -> Region {
    Region {
        arena: ArenaId(0),
        offset: 0,
        len: 64,
        gen,
    }
}
fn key(gen: u32) -> LaunchKey {
    LaunchKey {
        op: "maxsim",
        shape_bucket: 128,
        regions: vec![region(gen)],
        dtype: DType::F32,
        variant: "tiled",
        lane: LaneClass::Query,
    }
}
#[test]
fn direct_runs_job_on_lane() {
    let lane = RecordingLane::new(LaneClass::Query, LaneConfig::new(region(0)));
    let ran = Arc::new(AtomicBool::new(false));
    let flag = ran.clone();
    let launcher = launcher_for(LaunchMode::Direct).unwrap();
    launcher
        .launch(
            &key(0),
            &lane,
            Box::new(move || {
                flag.store(true, Ordering::SeqCst);
                Ok(())
            }),
        )
        .unwrap();
    assert!(ran.load(Ordering::SeqCst));
    assert_eq!(launcher.mode(), LaunchMode::Direct);
}
#[test]
fn capture_is_unavailable_until_enabled() {
    assert!(
        matches!(launcher_for(LaunchMode::Capture), Err(KernelError::Unavailable { .. }))
    );
}
#[test]
fn persistent_is_unavailable_until_enabled() {
    assert!(
        matches!(launcher_for(LaunchMode::Persistent), Err(KernelError::Unavailable { ..
        }))
    );
}
#[test]
fn persistent_launcher_plugs_in_behind_the_trait() {
    struct RingLauncher(AtomicBool);
    impl Launcher for RingLauncher {
        fn mode(&self) -> LaunchMode {
            LaunchMode::Persistent
        }
        fn launch(
            &self,
            _key: &LaunchKey,
            lane: &dyn ExecContext,
            job: LaneJob,
        ) -> Result<(), KernelError> {
            self.0.store(true, Ordering::SeqCst);
            lane.submit(job).map_err(KernelError::from)
        }
    }
    let lane = RecordingLane::new(LaneClass::Query, LaneConfig::new(region(0)));
    let launcher: Arc<dyn Launcher> = Arc::new(RingLauncher(AtomicBool::new(false)));
    launcher.launch(&key(0), &lane, Box::new(|| Ok(()))).unwrap();
    assert_eq!(launcher.mode(), LaunchMode::Persistent);
}
#[test]
fn generation_change_changes_key() {
    assert_ne!(key(0), key(1));
    assert_eq!(key(2), key(2));
}
#[test]
fn variant_and_dtype_change_key() {
    let mut other = key(0);
    other.variant = "naive";
    assert_ne!(key(0), other);
    let mut other = key(0);
    other.dtype = DType::F16;
    assert_ne!(key(0), other);
}
#[test]
fn shape_bucket_rounds_up_to_power_of_two() {
    assert_eq!(shape_bucket(0), 1);
    assert_eq!(shape_bucket(100), 128);
    assert_eq!(shape_bucket(128), 128);
    assert_eq!(shape_bucket(u32::MAX), 1 << 31);
}
struct MockCapture {
    warmed: AtomicUsize,
    launched: AtomicUsize,
}
impl Launcher for MockCapture {
    fn mode(&self) -> LaunchMode {
        LaunchMode::Capture
    }
    fn launch(
        &self,
        _key: &LaunchKey,
        lane: &dyn ExecContext,
        job: LaneJob,
    ) -> Result<(), KernelError> {
        self.launched.fetch_add(1, Ordering::SeqCst);
        lane.submit(job).map_err(KernelError::from)
    }
    fn warm_up(
        &self,
        _key: &LaunchKey,
        _lane: &dyn ExecContext,
    ) -> Result<(), KernelError> {
        self.warmed.fetch_add(1, Ordering::SeqCst);
        Ok(())
    }
}
fn run_through(launcher: &Arc<dyn Launcher>, gen: u32) -> bool {
    let lane = RecordingLane::new(LaneClass::Query, LaneConfig::new(region(gen)));
    let ran = Arc::new(AtomicBool::new(false));
    let flag = ran.clone();
    let k = key(gen);
    launcher.warm_up(&k, &lane).unwrap();
    launcher
        .launch(
            &k,
            &lane,
            Box::new(move || {
                flag.store(true, Ordering::SeqCst);
                Ok(())
            }),
        )
        .unwrap();
    ran.load(Ordering::SeqCst)
}
#[test]
fn direct_and_capture_are_interchangeable() {
    let capture = Arc::new(MockCapture {
        warmed: AtomicUsize::new(0),
        launched: AtomicUsize::new(0),
    });
    let launchers: [Arc<dyn Launcher>; 2] = [
        launcher_for(LaunchMode::Direct).unwrap(),
        capture.clone(),
    ];
    for launcher in &launchers {
        assert!(run_through(launcher, 0));
    }
    assert_eq!(launchers[0].mode(), LaunchMode::Direct);
    assert_eq!(launchers[1].mode(), LaunchMode::Capture);
    assert_eq!(capture.warmed.load(Ordering::SeqCst), 1);
    assert_eq!(capture.launched.load(Ordering::SeqCst), 1);
}
