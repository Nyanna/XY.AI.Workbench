//! Launch indirection between scorers and lanes.
//!
//! `Direct` submits the job to the lane unchanged. `Capture` is reserved for graph replay; the
//! key already carries everything a graph cache needs, so scorers need no change when it lands.
use std::sync::Arc;
use super::KernelError;
use crate::resident::memory::{DType, Region};
use crate::resident::subengine::{ExecContext, LaneClass, LaneJob};
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum LaunchMode {
    Direct,
    Capture,
}
/// Identity of a launch. Regions carry their generation, so a generation change yields a new key
/// and stale captured graphs are never replayed.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct LaunchKey {
    pub op: &'static str,
    pub shape_bucket: u32,
    pub regions: Vec<Region>,
    pub dtype: DType,
    pub lane: LaneClass,
}
pub trait Launcher: Send + Sync {
    fn mode(&self) -> LaunchMode;
    fn launch(
        &self,
        key: &LaunchKey,
        lane: &dyn ExecContext,
        job: LaneJob,
    ) -> Result<(), KernelError>;
}
#[derive(Clone, Copy, Debug, Default)]
pub struct DirectLauncher;
impl Launcher for DirectLauncher {
    fn mode(&self) -> LaunchMode {
        LaunchMode::Direct
    }
    #[tracing::instrument(
        level = "trace",
        skip_all,
        fields(op = key.op, bucket = key.shape_bucket)
    )]
    fn launch(
        &self,
        key: &LaunchKey,
        lane: &dyn ExecContext,
        job: LaneJob,
    ) -> Result<(), KernelError> {
        lane.submit(job).map_err(KernelError::from)
    }
}
pub fn launcher_for(mode: LaunchMode) -> Result<Arc<dyn Launcher>, KernelError> {
    match mode {
        LaunchMode::Direct => Ok(Arc::new(DirectLauncher)),
        LaunchMode::Capture => {
            Err(KernelError::Unavailable {
                reason: "graph capture is not enabled".into(),
            })
        }
    }
}
#[cfg(test)]
mod tests {
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
            matches!(launcher_for(LaunchMode::Capture), Err(KernelError::Unavailable { ..
            }))
        );
    }
    #[test]
    fn generation_change_changes_key() {
        assert_ne!(key(0), key(1));
        assert_eq!(key(2), key(2));
    }
}
