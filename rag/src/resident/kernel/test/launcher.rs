use super::*;
use crate::resident::memory::ArenaId;
use crate::resident::subengine::{LaneConfig, RecordingLane};
use std::sync::atomic::{AtomicBool, Ordering};
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
