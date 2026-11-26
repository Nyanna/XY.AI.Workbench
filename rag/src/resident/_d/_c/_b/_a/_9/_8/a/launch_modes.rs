//! Launch modes: direct launch, rejected graph capture and persistent kernel,
//! launch keys and the warm-up hook.
use std::collections::HashSet;
use std::fmt;
use std::sync::{Arc, Condvar, Mutex, PoisonError};
use log::trace;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::a::kernel_register::Variant;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::a::table_swap::{
    Epoch, ReaderId, TableGuard, TableSwap, TableSwapError, ROOT_SENTINEL,
};
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::a::region_handle::RegionHandle;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::_1::device_lane_event::LaneKind;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::_1::layout::DType;
const GRAPH_CAPTURE_DISABLED: &str = "Graph-Capture ist nicht aktiviert";
const PERSISTENT_DISABLED: &str = "Persistent-Modus ist nicht aktiviert";
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum LaunchMode {
    Direct,
    GraphCapture,
    Persistent,
}
/// Power of two in `1..=2^31`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ShapeClass(u32);
impl ShapeClass {
    pub const MAX: ShapeClass = ShapeClass(1 << 31);
    /// Smallest power of two `>= extent`, at least 1; saturates at `MAX`.
    pub fn of(extent: u64) -> ShapeClass {
        if extent <= 1 {
            return ShapeClass(1);
        }
        if extent > u64::from(Self::MAX.0) {
            return Self::MAX;
        }
        ShapeClass(extent.next_power_of_two() as u32)
    }
    pub fn get(self) -> u32 {
        self.0
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct LaunchKey {
    pub operation: String,
    pub shape: ShapeClass,
    pub regions: Vec<RegionHandle>,
    pub dtype: DType,
    pub variant: Variant,
    pub lane_class: LaneKind,
}
impl LaunchKey {
    pub fn new(
        operation: impl Into<String>,
        extent: u64,
        regions: &[RegionHandle],
        dtype: DType,
        variant: Variant,
        lane_class: LaneKind,
    ) -> LaunchKey {
        LaunchKey {
            operation: operation.into(),
            shape: ShapeClass::of(extent),
            regions: regions.to_vec(),
            dtype,
            variant,
            lane_class,
        }
    }
}
/// Kernel arguments of every normal launch.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LaunchArgs {
    pub root: u64,
    pub epoch: Epoch,
}
/// Implemented by the caller or the GPU binding; hands the work item to the lane unchanged.
pub trait LaunchLane<W>: Send + Sync {
    fn launch(&self, item: &W, args: &LaunchArgs) -> Result<(), String>;
}
pub trait WarmUp: Send + Sync {
    fn warm_up(&self, _key: &LaunchKey) -> Result<(), String> {
        Ok(())
    }
}
#[derive(Clone, Copy, Debug, Default)]
pub struct NoWarmUp;
impl WarmUp for NoWarmUp {}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LaunchError {
    NotAvailable { mode: LaunchMode, reason: String },
    Kernel(String),
    WarmUp(String),
    Table(TableSwapError),
}
impl fmt::Display for LaunchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LaunchError::NotAvailable { reason, .. } => {
                write!(f, "nicht verfügbar: {reason}")
            }
            LaunchError::Kernel(text) => write!(f, "kernel launch failed: {text}"),
            LaunchError::WarmUp(text) => write!(f, "warm-up failed: {text}"),
            LaunchError::Table(err) => write!(f, "table access failed: {err}"),
        }
    }
}
impl std::error::Error for LaunchError {}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LaunchReport {
    pub args: LaunchArgs,
    pub productive: bool,
}
#[derive(Default)]
struct Phases {
    warm: HashSet<LaunchKey>,
    warming: HashSet<LaunchKey>,
}
pub struct Launcher {
    swap: Arc<TableSwap>,
    reader: ReaderId,
    warm: Arc<dyn WarmUp>,
    seen: Mutex<Phases>,
    settled: Condvar,
}
/// Ends the warm-up of one key on drop, also when the hook panics.
struct WarmingTicket<'a> {
    launcher: &'a Launcher,
    key: &'a LaunchKey,
    success: bool,
}
impl Drop for WarmingTicket<'_> {
    fn drop(&mut self) {
        let mut phases = lock(&self.launcher.seen);
        phases.warming.remove(self.key);
        if self.success {
            phases.warm.insert(self.key.clone());
        }
        drop(phases);
        self.launcher.settled.notify_all();
    }
}
static_assertions::assert_impl_all!(Launcher : Send, Sync);
impl Launcher {
    pub fn new(
        swap: Arc<TableSwap>,
        reader: ReaderId,
        warm_up: Arc<dyn WarmUp>,
    ) -> Launcher {
        Launcher {
            swap,
            reader,
            warm: warm_up,
            seen: Mutex::new(Phases::default()),
            settled: Condvar::new(),
        }
    }
    pub fn launch<W>(
        &self,
        mode: LaunchMode,
        key: &LaunchKey,
        lane: &dyn LaunchLane<W>,
        item: &W,
    ) -> Result<LaunchReport, LaunchError> {
        match mode {
            LaunchMode::Direct => {}
            LaunchMode::GraphCapture => {
                return Err(LaunchError::NotAvailable {
                    mode,
                    reason: GRAPH_CAPTURE_DISABLED.to_string(),
                });
            }
            LaunchMode::Persistent => {
                return Err(LaunchError::NotAvailable {
                    mode,
                    reason: PERSISTENT_DISABLED.to_string(),
                });
            }
        }
        let productive = !self.first_launch(key)?;
        let (args, _guard) = self.pin()?;
        trace!(
            "launch operation={} shape={} epoch={} root={} productive={}", key.operation,
            key.shape.get(), args.epoch, args.root, productive
        );
        lane.launch(item, &args).map_err(LaunchError::Kernel)?;
        Ok(LaunchReport { args, productive })
    }
    /// Root and epoch of one consistent table version.
    pub fn current_args(&self) -> Result<LaunchArgs, LaunchError> {
        self.pin().map(|(args, _guard)| args)
    }
    pub fn is_warm(&self, key: &LaunchKey) -> bool {
        lock(&self.seen).warm.contains(key)
    }
    /// `true` if this call ran the warm-up hook for a new key. Callers with the same key
    /// wait while the hook runs; if it fails, one of them runs the hook next.
    fn first_launch(&self, key: &LaunchKey) -> Result<bool, LaunchError> {
        let mut phases = lock(&self.seen);
        loop {
            if phases.warm.contains(key) {
                return Ok(false);
            }
            if !phases.warming.contains(key) {
                break;
            }
            phases = self.settled.wait(phases).unwrap_or_else(PoisonError::into_inner);
        }
        phases.warming.insert(key.clone());
        drop(phases);
        let mut ticket = WarmingTicket {
            launcher: self,
            key,
            success: false,
        };
        match self.warm.warm_up(key) {
            Ok(()) => {
                ticket.success = true;
                Ok(true)
            }
            Err(text) => Err(LaunchError::WarmUp(text)),
        }
    }
    fn pin(&self) -> Result<(LaunchArgs, Option<TableGuard<'_>>), LaunchError> {
        match self.swap.acquire(self.reader) {
            Ok(guard) => {
                let args = LaunchArgs {
                    root: guard.root(),
                    epoch: guard.epoch(),
                };
                Ok((args, Some(guard)))
            }
            Err(TableSwapError::NoTable) => {
                Ok((
                    LaunchArgs {
                        root: ROOT_SENTINEL,
                        epoch: 0,
                    },
                    None,
                ))
            }
            Err(err) => Err(LaunchError::Table(err)),
        }
    }
}
#[cfg(test)]
#[path = "test/launch_modes.rs"]
mod tests;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::_1::sync_util::lock;
