//! Launch indirection between scorers and lanes.
//!
//! `Direct` submits the job to the lane unchanged. `Capture` is reserved for graph replay; the
//! key already carries everything a graph cache needs, so scorers need no change when it lands.
use std::sync::Arc;
use super::KernelError;
use crate::resident::memory::{DType, Region};
use crate::resident::subengine::{ExecContext, LaneClass, LaneJob};
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum LaunchMode {
    Direct,
    Capture,
    /// Resident kernel fed through a host-mapped ring.
    Persistent,
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
        LaunchMode::Persistent => {
            Err(KernelError::Unavailable {
                reason: "persistent kernel mode is not enabled".into(),
            })
        }
    }
}
#[cfg(test)]
#[path = "test/launcher.rs"]
mod tests;
