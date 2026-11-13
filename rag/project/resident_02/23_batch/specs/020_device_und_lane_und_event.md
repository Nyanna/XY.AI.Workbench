# Spec: Device, Lane and Event (build order 20)

## 1. Task, restrictions and boundaries
Implement in Rust the types for device kinds and handles, execution contexts (lanes), and completion events.

- **Dependencies:** none. Use only `std`. Do not link any GPU runtime. GPU behavior sits behind the `GpuEventBackend` and `StreamBackend` traits, which other modules implement.
- **Out of scope:**
  - Submitting work, kernels, and copy implementations.
  - Memory allocators other than the lane's own byte buffers.
  - Scheduling and event-dependency graphs.
  - Any behavior not listed here.

## 2. Context
Other modules use this module for three things:
- Addressing devices (CPU or GPU ordinal).
- Owning per-purpose lanes: Query, Insert, Update, CopyH2D, CopyD2H.
- Expressing "work done" as `Event`.

Event dependencies in other modules are tested with `Event::mock`, so no GPU is needed. Copies and dependents hold clones of an `Event`, and all clones refer to the same marker.

## 3. Interface
```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum DeviceKind { Cpu, Gpu }

/// Device handle: the CPU or a GPU ordinal.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DeviceHandle { Cpu, Gpu(u32) }
impl DeviceHandle {
    pub fn kind(&self) -> DeviceKind;
    pub fn ordinal(&self) -> Option<u32>; // None for Cpu
}
impl std::fmt::Display for DeviceHandle;  // "cpu" / "gpu:<n>"

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EventError { pub device: DeviceHandle, pub cause: String }
impl std::fmt::Display for EventError;
impl std::error::Error for EventError {}

/// Implemented by the GPU binding (opaque to this module).
pub trait GpuEventBackend: Send + Sync + std::fmt::Debug {
    fn query(&self) -> Result<bool, String>;       // non-blocking
    fn synchronize(&self) -> Result<(), String>;   // blocking
}

/// Opaque completion marker for all work submitted on a lane before recording.
/// Clone shares the same underlying marker.
#[derive(Clone)]
pub struct Event { /* Arc<dyn ...> */ }
impl Event {
    pub fn cpu() -> (Event, EventSignal);                          // pending CPU event + its signaler
    pub fn gpu(device: u32, backend: Box<dyn GpuEventBackend>) -> Event;
    pub fn mock() -> (Event, MockEventControl);                    // artificially controllable
    pub fn is_complete(&self) -> Result<bool, EventError>;         // non-blocking
    pub fn wait(&self) -> Result<(), EventError>;                  // blocking
    pub fn device(&self) -> DeviceHandle;                          // Cpu for cpu/mock
}
impl std::fmt::Display for Event;
impl std::fmt::Debug for Event;
static_assertions: Event: Send + Sync

pub struct EventSignal { /* Arc */ }                // Send + Sync, Clone
impl EventSignal { pub fn signal(&self); }          // idempotent

#[derive(Clone)]
pub struct MockEventControl { /* Arc */ }           // Send + Sync
impl MockEventControl {
    pub fn complete(&self);
    pub fn fail(&self, cause: impl Into<String>);
    pub fn reset(&self);                            // back to pending
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Priority { Low, Normal, High }

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum LaneKind { Query, Insert, Update, CopyH2D, CopyD2H }

/// Opaque stream. Implemented by the GPU binding; CPU lanes have none.
pub trait StreamBackend: Send + Sync + std::fmt::Debug {
    fn record_event(&self) -> Result<Box<dyn GpuEventBackend>, String>;
}

pub struct LaneConfig {
    pub kind: LaneKind,
    pub device: DeviceHandle,
    pub priority: Priority,
    pub param_block_size: usize,   // bytes, fixed
    pub workspace_size: usize,     // initial bytes
}
impl LaneConfig {
    /// Presets: Query => High; Insert, Update, CopyH2D, CopyD2H => Normal.
    /// param_block_size = 256, workspace_size = 0.
    pub fn preset(kind: LaneKind, device: DeviceHandle) -> LaneConfig;
}

#[derive(Debug, PartialEq, Eq)]
pub enum LaneError { NoStreamBackend, Record(EventError) }
impl std::fmt::Display for LaneError;
impl std::error::Error for LaneError {}

/// Execution context. Exclusive per-lane state needs &mut; Lane: Send + Sync.
pub struct Lane { /* config, stream, params, workspace, recorded events */ }
impl Lane {
    pub fn new(cfg: LaneConfig, stream: Option<Box<dyn StreamBackend>>) -> Lane;
    pub fn kind(&self) -> LaneKind;
    pub fn device(&self) -> DeviceHandle;
    pub fn priority(&self) -> Priority;
    pub fn stream(&self) -> Option<&dyn StreamBackend>;
    pub fn params(&self) -> &[u8];
    pub fn params_mut(&mut self) -> &mut [u8];        // zero-initialised, fixed length
    pub fn workspace(&self) -> &[u8];
    pub fn workspace_mut(&mut self) -> &mut [u8];
    pub fn ensure_workspace(&mut self, bytes: usize); // grows only, new bytes zero, never shrinks
    /// Records an event covering all work submitted on this lane so far.
    pub fn record_event(&self) -> Result<Event, LaneError>;
    pub fn last_event(&self) -> Option<Event>;        // most recently recorded
}
```

## 4. Behavior and invariants
**Devices**
- `DeviceHandle::Cpu.kind() == Cpu` and `Gpu(n).kind() == Gpu`.
- `ordinal()` returns `Some(n)` for `Gpu(n)` and `None` for `Cpu`.
- Handles compare by value, and `Gpu(0) != Cpu`.

**Lanes**
- Each lane owns its parameter block and workspace. Two lanes never share them.
- `Lane::new` zero-fills the parameter block (`param_block_size` bytes) and the workspace (`workspace_size` bytes).
- The parameter block length never changes.
- `ensure_workspace(n)` makes the workspace length ≥ n, keeps existing contents, and never shrinks it.
- `record_event` calls `StreamBackend::record_event`, wraps the result as `Event::gpu(ordinal, ..)`, stores it as `last_event`, and returns it.
- Errors from `record_event`:
  - No stream backend (CPU lane): `Err(LaneError::NoStreamBackend)`.
  - Backend failure: `Err(LaneError::Record(EventError{device, cause}))`.
- CPU completion markers are made with `Event::cpu()`. Whoever executes the CPU work calls `signal`.
- Lane kinds are labels only. They do not change behavior apart from the `preset` priority.

**Events**
- A fresh `Event::cpu()` or `Event::mock()` is pending, with `is_complete() == Ok(false)`.
- `is_complete` never blocks.
- `wait` blocks until the event completes. It returns immediately if the event is already complete.
- **CPU event:**
  - After `signal()` it is permanently complete.
  - `signal()` is idempotent, and there is no reset.
  - It wakes all current waiters (`Condvar::notify_all`).
  - Later `wait` calls return immediately.
  - Any thread can signal or wait, and several can wait concurrently.
  - `is_complete` and `wait` never return an error.
- **GPU event:**
  - `is_complete` maps `backend.query()`. It returns `Err(EventError{device: Gpu(n), cause})` on `Err(cause)`.
  - `wait` calls `backend.synchronize()` and maps errors the same way.
  - Once `query` has returned `Ok(true)` or `synchronize` has returned `Ok`, the event caches the completed state.
  - A cached error is not cached. A later call asks the backend again.
- **Mock event:**
  - `complete()` makes `is_complete` return `Ok(true)` and wakes all waiters.
  - `fail(cause)` makes `is_complete` and `wait` return `Err(EventError{device: Cpu, cause})`. Waiters are woken and get the error.
  - `reset()` returns the event to pending.
  - A mock is interchangeable with real events wherever `Event` is accepted.
- Clones of an `Event` observe the same state. Completing it via any handle is visible to all clones.
- **Display:**
  - Pending: `Event(<device>, pending)`.
  - Complete: `Event(<device>, complete)`.
  - Failed: `Event(<device>, failed: <cause>)`.
  - `<device>` uses `DeviceHandle` Display.
  - Display never blocks. For a GPU event it uses a non-blocking query, and a query error is shown as `failed: <cause>`.
- `EventError` Display: `event error on <device>: <cause>`.
- Nothing in this module panics on a poisoned lock. It recovers the inner state.

## 5. Relied-on elements
None.