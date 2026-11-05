use std::fmt;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Condvar, Mutex, MutexGuard};
fn lock<T>(m: &Mutex<T>) -> MutexGuard<'_, T> {
    m.lock().unwrap_or_else(|e| e.into_inner())
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum DeviceKind {
    Cpu,
    Gpu,
}
/// Device handle: the CPU or a GPU ordinal.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DeviceHandle {
    Cpu,
    Gpu(u32),
}
impl DeviceHandle {
    pub fn kind(&self) -> DeviceKind {
        match self {
            DeviceHandle::Cpu => DeviceKind::Cpu,
            DeviceHandle::Gpu(_) => DeviceKind::Gpu,
        }
    }
    pub fn ordinal(&self) -> Option<u32> {
        match self {
            DeviceHandle::Cpu => None,
            DeviceHandle::Gpu(n) => Some(*n),
        }
    }
}
impl fmt::Display for DeviceHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DeviceHandle::Cpu => write!(f, "cpu"),
            DeviceHandle::Gpu(n) => write!(f, "gpu:{n}"),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EventError {
    pub device: DeviceHandle,
    pub cause: String,
}
impl fmt::Display for EventError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "event error on {}: {}", self.device, self.cause)
    }
}
impl std::error::Error for EventError {}
/// Implemented by the GPU binding (opaque to this module).
pub trait GpuEventBackend: Send + Sync + fmt::Debug {
    fn query(&self) -> Result<bool, String>;
    fn synchronize(&self) -> Result<(), String>;
}
#[derive(Debug)]
struct CpuState {
    done: Mutex<bool>,
    cv: Condvar,
}
#[derive(Debug)]
struct GpuState {
    device: u32,
    backend: Box<dyn GpuEventBackend>,
    completed: AtomicBool,
}
#[derive(Clone, Debug, PartialEq, Eq)]
enum MockStatus {
    Pending,
    Complete,
    Failed(String),
}
#[derive(Debug)]
struct MockState {
    status: Mutex<MockStatus>,
    cv: Condvar,
}
#[derive(Debug)]
enum Inner {
    Cpu(Arc<CpuState>),
    Gpu(GpuState),
    Mock(Arc<MockState>),
}
/// Opaque completion marker. Clones share the same underlying marker.
#[derive(Clone)]
pub struct Event {
    inner: Arc<Inner>,
}
static_assertions::assert_impl_all!(Event : Send, Sync);
static_assertions::assert_impl_all!(EventSignal : Send, Sync, Clone);
static_assertions::assert_impl_all!(MockEventControl : Send, Sync, Clone);
static_assertions::assert_impl_all!(Lane : Send, Sync);
impl Event {
    pub fn cpu() -> (Event, EventSignal) {
        let state = Arc::new(CpuState {
            done: Mutex::new(false),
            cv: Condvar::new(),
        });
        let event = Event {
            inner: Arc::new(Inner::Cpu(state.clone())),
        };
        (event, EventSignal { state })
    }
    pub fn gpu(device: u32, backend: Box<dyn GpuEventBackend>) -> Event {
        Event {
            inner: Arc::new(
                Inner::Gpu(GpuState {
                    device,
                    backend,
                    completed: AtomicBool::new(false),
                }),
            ),
        }
    }
    pub fn mock() -> (Event, MockEventControl) {
        let state = Arc::new(MockState {
            status: Mutex::new(MockStatus::Pending),
            cv: Condvar::new(),
        });
        let event = Event {
            inner: Arc::new(Inner::Mock(state.clone())),
        };
        (event, MockEventControl { state })
    }
    pub fn is_complete(&self) -> Result<bool, EventError> {
        match &*self.inner {
            Inner::Cpu(s) => Ok(*lock(&s.done)),
            Inner::Gpu(g) => {
                if g.completed.load(Ordering::Acquire) {
                    return Ok(true);
                }
                match g.backend.query() {
                    Ok(true) => {
                        g.completed.store(true, Ordering::Release);
                        Ok(true)
                    }
                    Ok(false) => Ok(false),
                    Err(cause) => Err(gpu_error(g.device, cause)),
                }
            }
            Inner::Mock(m) => {
                match &*lock(&m.status) {
                    MockStatus::Pending => Ok(false),
                    MockStatus::Complete => Ok(true),
                    MockStatus::Failed(c) => Err(mock_error(c.clone())),
                }
            }
        }
    }
    pub fn wait(&self) -> Result<(), EventError> {
        match &*self.inner {
            Inner::Cpu(s) => {
                let mut done = lock(&s.done);
                while !*done {
                    done = s.cv.wait(done).unwrap_or_else(|e| e.into_inner());
                }
                Ok(())
            }
            Inner::Gpu(g) => {
                if g.completed.load(Ordering::Acquire) {
                    return Ok(());
                }
                match g.backend.synchronize() {
                    Ok(()) => {
                        g.completed.store(true, Ordering::Release);
                        Ok(())
                    }
                    Err(cause) => Err(gpu_error(g.device, cause)),
                }
            }
            Inner::Mock(m) => {
                let mut status = lock(&m.status);
                loop {
                    match &*status {
                        MockStatus::Pending => {
                            status = m
                                .cv
                                .wait(status)
                                .unwrap_or_else(|e| e.into_inner());
                        }
                        MockStatus::Complete => return Ok(()),
                        MockStatus::Failed(c) => return Err(mock_error(c.clone())),
                    }
                }
            }
        }
    }
    pub fn device(&self) -> DeviceHandle {
        match &*self.inner {
            Inner::Gpu(g) => DeviceHandle::Gpu(g.device),
            _ => DeviceHandle::Cpu,
        }
    }
}
fn gpu_error(device: u32, cause: String) -> EventError {
    EventError {
        device: DeviceHandle::Gpu(device),
        cause,
    }
}
fn mock_error(cause: String) -> EventError {
    EventError {
        device: DeviceHandle::Cpu,
        cause,
    }
}
impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let device = self.device();
        match self.is_complete() {
            Ok(true) => write!(f, "Event({device}, complete)"),
            Ok(false) => write!(f, "Event({device}, pending)"),
            Err(e) => write!(f, "Event({device}, failed: {})", e.cause),
        }
    }
}
impl fmt::Debug for Event {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}
#[derive(Clone, Debug)]
pub struct EventSignal {
    state: Arc<CpuState>,
}
impl EventSignal {
    pub fn signal(&self) {
        let mut done = lock(&self.state.done);
        *done = true;
        drop(done);
        self.state.cv.notify_all();
    }
}
#[derive(Clone, Debug)]
pub struct MockEventControl {
    state: Arc<MockState>,
}
impl MockEventControl {
    fn set(&self, status: MockStatus) {
        *lock(&self.state.status) = status;
        self.state.cv.notify_all();
    }
    pub fn complete(&self) {
        self.set(MockStatus::Complete);
    }
    pub fn fail(&self, cause: impl Into<String>) {
        self.set(MockStatus::Failed(cause.into()));
    }
    pub fn reset(&self) {
        self.set(MockStatus::Pending);
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Priority {
    Low,
    Normal,
    High,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum LaneKind {
    Query,
    Insert,
    Update,
    CopyH2D,
    CopyD2H,
}
/// Opaque stream. Implemented by the GPU binding; CPU lanes have none.
pub trait StreamBackend: Send + Sync + fmt::Debug {
    fn record_event(&self) -> Result<Box<dyn GpuEventBackend>, String>;
}
#[derive(Clone, Debug)]
pub struct LaneConfig {
    pub kind: LaneKind,
    pub device: DeviceHandle,
    pub priority: Priority,
    pub param_block_size: usize,
    pub workspace_size: usize,
}
impl LaneConfig {
    pub fn preset(kind: LaneKind, device: DeviceHandle) -> LaneConfig {
        let priority = match kind {
            LaneKind::Query => Priority::High,
            _ => Priority::Normal,
        };
        LaneConfig {
            kind,
            device,
            priority,
            param_block_size: 256,
            workspace_size: 0,
        }
    }
}
#[derive(Debug, PartialEq, Eq)]
pub enum LaneError {
    NoStreamBackend,
    Record(EventError),
}
impl fmt::Display for LaneError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LaneError::NoStreamBackend => write!(f, "lane has no stream backend"),
            LaneError::Record(e) => write!(f, "failed to record event: {e}"),
        }
    }
}
impl std::error::Error for LaneError {}
/// Execution context owning its parameter block, workspace and stream.
pub struct Lane {
    cfg: LaneConfig,
    stream: Option<Box<dyn StreamBackend>>,
    params: Vec<u8>,
    workspace: Vec<u8>,
    last_event: Mutex<Option<Event>>,
}
impl Lane {
    pub fn new(cfg: LaneConfig, stream: Option<Box<dyn StreamBackend>>) -> Lane {
        let params = vec![0u8; cfg.param_block_size];
        let workspace = vec![0u8; cfg.workspace_size];
        Lane {
            cfg,
            stream,
            params,
            workspace,
            last_event: Mutex::new(None),
        }
    }
    pub fn kind(&self) -> LaneKind {
        self.cfg.kind
    }
    pub fn device(&self) -> DeviceHandle {
        self.cfg.device
    }
    pub fn priority(&self) -> Priority {
        self.cfg.priority
    }
    pub fn stream(&self) -> Option<&dyn StreamBackend> {
        self.stream.as_deref()
    }
    pub fn params(&self) -> &[u8] {
        &self.params
    }
    pub fn params_mut(&mut self) -> &mut [u8] {
        &mut self.params
    }
    pub fn workspace(&self) -> &[u8] {
        &self.workspace
    }
    pub fn workspace_mut(&mut self) -> &mut [u8] {
        &mut self.workspace
    }
    pub fn ensure_workspace(&mut self, bytes: usize) {
        if self.workspace.len() < bytes {
            self.workspace.resize(bytes, 0);
        }
    }
    pub fn record_event(&self) -> Result<Event, LaneError> {
        let stream = self.stream.as_deref().ok_or(LaneError::NoStreamBackend)?;
        let backend = stream
            .record_event()
            .map_err(|cause| {
                LaneError::Record(EventError {
                    device: self.cfg.device,
                    cause,
                })
            })?;
        let event = Event::gpu(self.cfg.device.ordinal().unwrap_or(0), backend);
        *lock(&self.last_event) = Some(event.clone());
        Ok(event)
    }
    pub fn last_event(&self) -> Option<Event> {
        lock(&self.last_event).clone()
    }
}
#[cfg(test)]
#[path = "test/device_lane_event.rs"]
mod tests;
