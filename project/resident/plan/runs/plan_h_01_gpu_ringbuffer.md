Setze H1 um.

- keine direkte Referenzierung von Planelementen in Kommentaren
- nvcc liegt unter `/usr/local/cuda-13.3/bin/nvcc`
- Nutze Bash nicht für Retrieval Tasks

[include](/home/user/xyan/xy.ai.workbench/project/resident/index.md)

[include](/home/user/xyan/xy.ai.workbench/project/resident/plan/plan_h_00_overview.md)

[include](/home/user/xyan/xy.ai.workbench/project/resident/plan/plan_h_01_gpu_ringbuffer.md)

SystemInit:  id=d0c17d75-e168-4de5-bd14-54a1ced08751, cwd=/home/user/xyan/xy.ai.workbench/project, model=claude-sonnet-5-5
Text:
 Ich verschaffe mir zuerst einen Überblick über die vorhandenen Resident-Module.
Control Request:
```yaml
id: 4172-1
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/subengine/device.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/memory/dtype.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/index/memory.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/op/lease.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/manager/mod.rs
  reason: "Struktur der betroffenen Module (Lane, Arena, Manager, Residency) für die H1-Erweiterungspunkte erfassen"
```
/answer 4172-1 allow
ReasoningToken: 52
Control Request:
```yaml
id: 4172-2
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/subengine/device.rs
      nodes:
      - id: ModuleDoc
        signature: "//! Compute backends, device handle and CUDA execution lanes."
      - id: 77cuty|ChE8yG
        signature: use crate::resident::backend::ComputeBackend;
      - id: CudaBackend
        signature: pub struct CudaBackend;
        docstring: Real CUDA backend; the single Candle/CUDA adapter point for the subengine. Cons…
      - id: impl_CudaBackend
        signature: "impl CudaBackend {"
        children:
        - id: impl_CudaBackend.try_new
          signature: "pub fn try_new() -> Result<Self, crate::resident::error::IndexError> {"
      - id: impl_ComputeBackend_for_CudaBackend
        signature: "impl ComputeBackend for CudaBackend {"
        children:
        - id: impl_ComputeBackend_for_CudaBackend.run_batch
          signature: "fn run_batch(&self, _input: &[f32]) -> Vec<f32> {"
      - id: BbkRhK|SOdFt8
        signature: "use std::time::{Duration, Instant};"
      - id: DeviceHandle
        signature: "pub struct DeviceHandle {"
        docstring: Identifies the compute device a subengine runs on; lanes are opened from it.
      - id: impl_std__fmt__Debug_for_DeviceHandle
        signature: "impl std::fmt::Debug for DeviceHandle {"
        children:
        - id: impl_std__fmt__Debug_for_DeviceHandle.fmt
          signature: "fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {"
      - id: impl_DeviceHandle
        signature: "impl DeviceHandle {"
        children:
        - id: impl_DeviceHandle.cpu
          signature: "pub fn cpu() -> Self {"
        - id: impl_DeviceHandle.cuda
          signature: "pub fn cuda(ordinal: usize) -> Result<Self, DeviceError> {"
          docstring: "Retains the primary context of `ordinal`. The first call is expensive (0.2-2 s,…"
        - id: impl_DeviceHandle.cuda_2
          signature: "pub fn cuda(_ordinal: usize) -> Result<Self, DeviceError> {"
        - id: impl_DeviceHandle.kind
          signature: "pub fn kind(&self) -> DeviceKind {"
        - id: impl_DeviceHandle.cuda_context
          signature: "pub(crate) fn cuda_context(     &self, ) -> Option<&std::sync::Arc<cudarc::driv…"
        - id: impl_DeviceHandle.ordinal
          signature: "pub fn ordinal(&self) -> usize {"
        - id: impl_DeviceHandle.init_cost
          signature: "pub fn init_cost(&self) -> Duration {"
          docstring: "Time spent creating the context, reported to the lazy-init accounting."
        - id: impl_DeviceHandle.warmup
          signature: "pub fn warmup(&self) -> Result<Duration, DeviceError> {"
          docstring: "Forces driver-side lazy work (first allocation, stream creation) so it does not…"
        - id: impl_DeviceHandle.open_lane
          signature: "pub fn open_lane(     &self,     class: LaneClass,     cfg: LaneConfig, ) -> Re…"
      - id: Ke5I2l|scA2j5
        signature: "#[cfg(feature = \"cuda\")]"
      - id: cuda_lane
        signature: "mod cuda_lane {"
        children:
        - id: cuda_lane.CHYK8i|Km1i1Y
          signature: use std::sync::Arc;
        - id: cuda_lane.CudaLane
          signature: "pub struct CudaLane {"
          docstring: Prioritised non-blocking stream; all device work of one lane is ordered on it.
        - id: cuda_lane.event_err
          signature: "fn event_err(e: cudarc::driver::DriverError) -> DeviceError {"
        - id: cuda_lane.impl_CudaLane
          signature: "impl CudaLane {"
          children:
          - id: cuda_lane.impl_CudaLane.new
            signature: "pub(crate) fn new(     ctx: &Arc<CudaContext>,     class: LaneClass,     cfg: L…"
          - id: cuda_lane.impl_CudaLane.priority
            signature: "pub fn priority(&self) -> i32 {"
          - id: cuda_lane.impl_CudaLane.stream
            signature: "pub(crate) fn stream(&self) -> &Arc<CudaStream> {"
        - id: cuda_lane.impl_ExecContext_for_CudaLane
          signature: "impl ExecContext for CudaLane {"
          children:
          - id: cuda_lane.impl_ExecContext_for_CudaLane.class
            signature: "fn class(&self) -> LaneClass {"
          - id: cuda_lane.impl_ExecContext_for_CudaLane.device
            signature: "fn device(&self) -> DeviceKind {"
          - id: cuda_lane.impl_ExecContext_for_CudaLane.submit
            signature: "fn submit(&self, job: LaneJob) -> Result<(), DeviceError> {"
          - id: cuda_lane.impl_ExecContext_for_CudaLane.record_event
            signature: "fn record_event(&self) -> Result<EventHandle, DeviceError> {"
          - id: cuda_lane.impl_ExecContext_for_CudaLane.wait
            signature: "fn wait(&self, event: &EventHandle) -> Result<(), DeviceError> {"
          - id: cuda_lane.impl_ExecContext_for_CudaLane.synchronize
            signature: "fn synchronize(&self) -> Result<(), DeviceError> {"
          - id: cuda_lane.impl_ExecContext_for_CudaLane.param_block
            signature: "fn param_block(&mut self) -> &mut ParamBlock {"
          - id: cuda_lane.impl_ExecContext_for_CudaLane.scratch
            signature: "fn scratch(&self) -> &ScratchArena {"
      - id: tests
        signature: mod tests;
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/memory/dtype.rs
      nodes:
      - id: ModuleDoc
        signature: //! Engine-owned element type; backend dtypes are converted at the edge only.
      - id: MS3n2e|NqxAEG
        signature: use thiserror::Error;
      - id: UnsupportedDType
        signature: pub struct UnsupportedDType(pub &'static str);
      - id: 9HWcpZ|0bwvF9
        signature: /// Additive attention-mask value that stays finite in f16 (`-inf` would turn r…
      - id: DType
        signature: "pub enum DType {"
      - id: impl_DType
        signature: "impl DType {"
        children:
        - id: impl_DType.size_bytes
          signature: "pub const fn size_bytes(self) -> usize {"
      - id: impl_TryFrom_safetensors__Dtype__for_DType
        signature: "impl TryFrom<safetensors::Dtype> for DType {"
        children:
        - id: impl_TryFrom_safetensors__Dtype__for_DType.n7RcFD|qILWYw
          signature: type Error = UnsupportedDType;
        - id: impl_TryFrom_safetensors__Dtype__for_DType.try_from
          signature: "fn try_from(d: safetensors::Dtype) -> Result<Self, Self::Error> {"
      - id: candle_adapter
        signature: "pub(crate) mod candle_adapter {"
        children:
        - id: candle_adapter.R4kkrJ|JI8Adk
          signature: "use super::{DType, UnsupportedDType};"
        - id: candle_adapter.impl_TryFrom_DType__for_C
          signature: "impl TryFrom<DType> for C {"
          children:
          - id: candle_adapter.impl_TryFrom_DType__for_C.n7RcFD|qILWYw
            signature: type Error = UnsupportedDType;
          - id: candle_adapter.impl_TryFrom_DType__for_C.try_from
            signature: "fn try_from(d: DType) -> Result<Self, Self::Error> {"
        - id: candle_adapter.impl_TryFrom_C__for_DType
          signature: "impl TryFrom<C> for DType {"
          children:
          - id: candle_adapter.impl_TryFrom_C__for_DType.n7RcFD|qILWYw
            signature: type Error = UnsupportedDType;
          - id: candle_adapter.impl_TryFrom_C__for_DType.try_from
            signature: "fn try_from(d: C) -> Result<Self, Self::Error> {"
        - id: candle_adapter.elems_to_bytes
          signature: "pub(crate) fn elems_to_bytes(elems: usize, dtype: DType) -> usize {"
          docstring: Candle layouts count elements; the engine counts bytes.
        - id: candle_adapter.bytes_to_elems
          signature: "pub(crate) fn bytes_to_elems(bytes: usize, dtype: DType) -> usize {"
      - id: tests
        signature: mod tests;
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/index/memory.rs
      nodes:
      - id: ModuleDoc
        signature: //! Reference in-memory index; builds without `cuda`/`candle`.
      - id: KLORGj|vql8Wh
        signature: use std::collections::BTreeMap;
      - id: Inner
        signature: "struct Inner {"
      - id: InMemoryIndex
        signature: "pub struct InMemoryIndex {"
      - id: impl_InMemoryIndex
        signature: "impl InMemoryIndex {"
        children:
        - id: impl_InMemoryIndex.new
          signature: "pub fn new() -> Self {"
        - id: impl_InMemoryIndex.with_counter
          signature: "pub fn with_counter(counter: StampCounter) -> Self {"
        - id: impl_InMemoryIndex.high_water
          signature: "pub fn high_water(&self) -> u64 {"
        - id: impl_InMemoryIndex.remove
          signature: "pub fn remove(&self, entry: EntryId) -> Option<Stamp> {"
          docstring: Removal consumes a stamp so that re-adding the same id yields a new one.
      - id: impl_Default_for_InMemoryIndex
        signature: "impl Default for InMemoryIndex {"
        children:
        - id: impl_Default_for_InMemoryIndex.default
          signature: "fn default() -> Self {"
      - id: impl_Index_for_InMemoryIndex
        signature: "impl Index for InMemoryIndex {"
        children:
        - id: impl_Index_for_InMemoryIndex.stamp
          signature: "fn stamp(&self, entry: EntryId) -> Option<Stamp> {"
        - id: impl_Index_for_InMemoryIndex.stamps
          signature: "fn stamps(&self) -> Box<dyn Iterator<Item = (EntryId, Stamp)> + '_> {"
        - id: impl_Index_for_InMemoryIndex.read
          signature: "fn read(&self, entry: EntryId) -> EntryData<'_> {"
        - id: impl_Index_for_InMemoryIndex.apply
          signature: "fn apply(&self, entry: EntryId, data: &[u8]) -> Stamp {"
        - id: impl_Index_for_InMemoryIndex.changed
          signature: "fn changed(&self) -> watch::Receiver<u64> {"
        - id: impl_Index_for_InMemoryIndex.capabilities
          signature: "fn capabilities(&self) -> &[AccessMethod] {"
        - id: impl_Index_for_InMemoryIndex.children
          signature: "fn children(&self, _node: NodeId) -> EntrySet {"
        - id: impl_Index_for_InMemoryIndex.members
          signature: "fn members(&self, cell: CellId) -> EntrySet {"
        - id: impl_Index_for_InMemoryIndex.summary
          signature: "fn summary(&self, _kind: AccessMethod) -> Option<SummaryRef> {"
        - id: impl_Index_for_InMemoryIndex.prefetch
          signature: "fn prefetch(&self, _set: &EntrySet) {"
      - id: tests
        signature: mod tests;
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/op/lease.rs
      nodes:
      - id: ModuleDoc
        signature: //! RAII lease on a device region; pinned regions are not evicted.
      - id: 07O91Q|1abKmK
        signature: "use std::sync::atomic::{AtomicUsize, Ordering};"
      - id: PinCount
        signature: pub struct PinCount(AtomicUsize);
      - id: impl_PinCount
        signature: "impl PinCount {"
        children:
        - id: impl_PinCount.new
          signature: "pub fn new() -> Arc<Self> {"
        - id: impl_PinCount.get
          signature: "pub fn get(&self) -> usize {"
        - id: impl_PinCount.is_unpinned
          signature: "pub fn is_unpinned(&self) -> bool {"
      - id: Lease
        signature: "pub struct Lease {"
      - id: DeviceHandle
        signature: pub type DeviceHandle = Lease;
      - id: impl_Lease
        signature: "impl Lease {"
        children:
        - id: impl_Lease.new
          signature: "pub fn new(     region: Region,     pin: Arc<PinCount>,     release: Option<Unb…"
          docstring: Increments the pin count; `release` receives the region on drop.
        - id: impl_Lease.region
          signature: "pub fn region(&self) -> Region {"
      - id: impl_Drop_for_Lease
        signature: "impl Drop for Lease {"
        children:
        - id: impl_Drop_for_Lease.drop
          signature: "fn drop(&mut self) {"
      - id: tests
        signature: mod tests;
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/manager/mod.rs
      nodes:
      - id: ModuleDoc
        signature: "//! StreamedIndexManager, ScopeResolver, update queue, residency."
      - id: advice
        signature: pub mod advice;
      - id: blocking
        signature: pub mod blocking;
      - id: doorbell
        signature: pub mod doorbell;
      - id: build
        signature: pub mod build;
      - id: arena
        signature: pub mod arena;
      - id: scope
        signature: pub mod scope;
      - id: slot
        signature: pub mod slot;
      - id: swap
        signature: pub mod swap;
      - id: runtime
        signature: pub mod runtime;
      - id: flat_index
        signature: pub(crate) mod flat_index;
      - id: streamed
        signature: pub mod streamed;
      - id: seqlock
        signature: pub mod seqlock;
      - id: sync
        signature: pub mod sync;
      - id: tracker
        signature: pub mod tracker;
      - id: evict
        signature: pub mod evict;
      - id: flight
        signature: pub(crate) mod flight;
      - id: residency
        signature: pub mod residency;
      - id: upload
        signature: pub mod upload;
      - id: transfer
        signature: pub mod transfer;
```
/answer 4172-2 allow
Control Request:
```yaml
id: 4172-3
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src/resident
    pattern: enum (Lane|Arena)|struct (Region|ExecContext)|trait (Launcher|Op)\b|Launcher|ArenaClass|LaneClass
    include:
    - '*.rs'
    exclude:
    - target/**
    limit: 40
  reason: Vorhandene Lane-/Arena-/Region-/Launcher-Typen lokalisieren
```
/answer 4172-3 allow
Control Request:
```yaml
id: 4172-4
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src/resident
      matches:
      - path: segment/test/sync.rs
        matches:
        - "use crate::resident::subengine::{LaneClass, LaneConfig, RecordingLane};"
        - "let lane = RecordingLane::new(LaneClass::Copy, LaneConfig::new(region(0, 4096)));"
      - path: processor/completion.rs
        matches:
        - "pub enum LaneOutcome {"
      - path: kernel/mod.rs
        matches:
        - "pub struct RegionSpan {"
      - path: kernel/launcher.rs
        matches:
        - "use crate::resident::subengine::{ExecContext, LaneClass, LaneJob};"
        - "pub lane: LaneClass,"
        - "pub trait Launcher: Send + Sync {"
        - pub struct DirectLauncher;
        - "impl Launcher for DirectLauncher {"
        - "pub fn launcher_for(mode: LaunchMode) -> Result<Arc<dyn Launcher>, KernelError> {"
        - "LaunchMode::Direct => Ok(Arc::new(DirectLauncher)),"
      - path: kernel/test/maxsim.rs
        matches:
        - "use crate::resident::subengine::{LaneClass, LaneConfig, RecordingLane};"
        - "RecordingLane::new(LaneClass::Query, LaneConfig::new(scratch))"
      - path: kernel/test/launcher.rs
        matches:
        - "lane: LaneClass::Query,"
        - "let lane = RecordingLane::new(LaneClass::Query, LaneConfig::new(region(0)));"
      - path: kernel/test/cosine.rs
        matches:
        - "use crate::resident::subengine::{LaneClass, LaneConfig, RecordingLane};"
        - "RecordingLane::new(LaneClass::Query, LaneConfig::new(scratch))"
      - path: subengine/mod.rs
        matches:
        - "CpuLane, EventHandle, ExecContext, LaneClass, LaneConfig, LaneJob, ParamBlock,"
      - path: subengine/test/device.rs
        matches:
        - "let lane = dev.open_lane(LaneClass::Query, cfg()).unwrap();"
        - "assert_eq!(lane.class(), LaneClass::Query);"
        - "let producer = dev.open_lane(LaneClass::Copy, cfg()).unwrap();"
        - "let consumer = dev.open_lane(LaneClass::Query, cfg()).unwrap();"
      - path: subengine/test/exec.rs
        matches:
        - "fn lane(class: LaneClass) -> CpuLane {"
        - let lane = lane(LaneClass::Query);
        - let lane = lane(LaneClass::Index);
        - let producer = lane(LaneClass::Copy);
        - let consumer = lane(LaneClass::Query);
        - assert!(LaneClass::Query.priority() < LaneClass::Copy.priority());
        - assert!(LaneClass::Copy.priority() < LaneClass::Index.priority());
        - "let mut lane = RecordingLane::new(LaneClass::Query, cfg());"
        - "Box::new(lane(LaneClass::Query)),"
        - "Box::new(RecordingLane::new(LaneClass::Copy, cfg())),"
      - path: subengine/device.rs
        matches:
        - "CpuLane, DeviceError, ExecContext, LaneClass, LaneConfig,"
        - "class: LaneClass,"
        - "fn class(&self) -> LaneClass {"
      - path: subengine/exec.rs
        matches:
        - "pub enum LaneClass {"
      warning: "Limit of 40 matches reached; further results may exist. Narrow the pattern, directory or include/exclude filters, or raise limit."
```
/answer 4172-4 allow
Control Request:
```yaml
id: 4172-5
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src/resident
    pattern: mod |pub use
    include:
    - mod.rs
    exclude:
    - target/**
    limit: 50
  reason: Modulstruktur und Re-Exports prüfen
```
/answer 4172-5 allow
Control Request:
```yaml
id: 4172-6
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src/resident
      matches:
      - path: mod.rs
        matches:
        - pub mod backend;
        - pub mod error;
        - pub mod failpoint;
        - pub mod telemetry;
        - pub mod mock;
        - pub mod queue;
        - pub mod memory;
        - pub mod op;
        - pub mod index;
        - pub mod kernel;
        - pub mod precision;
        - pub mod facade;
        - pub mod processor;
        - pub mod planner;
        - pub mod subengine;
        - pub mod model;
        - pub mod tokenize;
        - pub mod manager;
        - pub mod segment;
      - path: segment/mod.rs
        matches:
        - pub mod error;
        - pub mod mapping;
        - pub mod slots;
        - pub mod sync;
        - pub use error::SegmentError;
        - "pub use mapping::{EntryRef, Resolution, SegmentHit, SlotMap};"
        - "pub use slots::{encode_slot, SlotMeta, SlotWrite, ValidBitmap};"
        - "pub use sync::{Publisher, SegmentBus, SegmentEvent, SegmentEventKind};"
        - "pub mod mock {"
        - "pub(crate) mod testutil {"
        - mod tests;
      - path: processor/mod.rs
        matches:
        - mod completion;
        - mod completion_cuda;
        - mod config;
        - mod error;
        - mod hub;
        - mod ops;
        - mod resource;
        - mod starved;
        - mod worker;
        - mod tests;
        - "pub use completion::{"
        - "pub use completion_cuda::{HostFnCompletion, PolledCompletion};"
        - pub use config::WorkerConfig;
        - pub use error::OrchestrationError;
        - "pub use hub::{Engine, Expander, Expansion, Hub, RegionCheck, Step, Wakeups};"
        - "pub use ops::{Collect, OpJob, OpQueue, OpQueues, Side, TaskRun};"
        - "pub use resource::{ResourceSlot, ResourceState};"
        - "pub use starved::{StarvedSet, StarvedSnapshot};"
        - pub use worker::Processor;
      - path: precision/mod.rs
        matches:
        - mod calibrate;
      warning: "Limit of 50 matches reached; further results may exist. Narrow the pattern, directory or include/exclude filters, or raise limit."
```
/answer 4172-6 allow
Control Request:
```yaml
id: 4172-7
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/subengine/exec.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/subengine/mod.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel/launcher.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/manager/residency.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/manager/swap.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/manager/arena.rs
  reason: "Lane-Klasse, Launcher, Residency, Swap und Arena-Klassen im Detail prüfen"
```
/answer 4172-7 allow
Control Request:
```yaml
id: 4172-8
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/subengine/exec.rs
      nodes:
      - id: ModuleDoc
        signature: "//! Execution contexts: lanes, events, and per-lane parameter and scratch stora…"
      - id: 8KLzlk|GzcaOo
        signature: "use std::panic::{catch_unwind, AssertUnwindSafe};"
      - id: LaneJob
        signature: "pub type LaneJob = Box<dyn FnOnce() -> Result<(), DeviceError> + Send + 'static…"
        docstring: Work item executed on a lane. Device work is enqueued asynchronously from withi…
      - id: LaneClass
        signature: "pub enum LaneClass {"
      - id: impl_LaneClass
        signature: "impl LaneClass {"
        children:
        - id: impl_LaneClass.priority
          signature: "pub fn priority(self) -> i32 {"
          docstring: CUDA stream priority; lower values are more urgent and `0` is the default. Prio…
      - id: LaneConfig
        signature: "pub struct LaneConfig {"
      - id: impl_LaneConfig
        signature: "impl LaneConfig {"
        children:
        - id: impl_LaneConfig.Cj9Odg|qdIpgQ
          signature: "pub const DEFAULT_PARAM_CAPACITY: usize = 4096;"
        - id: impl_LaneConfig.new
          signature: "pub fn new(scratch: Region) -> Self {"
      - id: CpuEvent
        signature: "pub(crate) struct CpuEvent {"
      - id: impl_CpuEvent
        signature: "impl CpuEvent {"
        children:
        - id: impl_CpuEvent.signal
          signature: "fn signal(&self) {"
        - id: impl_CpuEvent.wait
          signature: "fn wait(&self) {"
        - id: impl_CpuEvent.is_done
          signature: "fn is_done(&self) -> bool {"
      - id: EventRepr
        signature: "pub(crate) enum EventRepr {"
      - id: EventHandle
        signature: "pub struct EventHandle {"
        docstring: Opaque completion marker for all work submitted to a lane before it was recorde…
      - id: impl_std__fmt__Debug_for_EventHandle
        signature: "impl std::fmt::Debug for EventHandle {"
        children:
        - id: impl_std__fmt__Debug_for_EventHandle.fmt
          signature: "fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {"
      - id: impl_EventHandle
        signature: "impl EventHandle {"
        children:
        - id: impl_EventHandle.from_repr
          signature: "pub(crate) fn from_repr(repr: EventRepr) -> Self {"
        - id: impl_EventHandle.repr
          signature: "pub(crate) fn repr(&self) -> &EventRepr {"
        - id: impl_EventHandle.is_complete
          signature: "pub fn is_complete(&self) -> bool {"
        - id: impl_EventHandle.synchronize
          signature: "pub fn synchronize(&self) -> Result<(), DeviceError> {"
          docstring: Blocks the calling thread until the event completes.
      - id: ParamValue
        signature: "pub trait ParamValue: Copy {"
        children:
        - id: ParamValue.D1i1th|F97p37
          signature: "const SIZE: usize;"
        - id: ParamValue.write
          signature: "fn write(self, out: &mut Vec<u8>);"
      - id: GHxFN6|KejVsY
        signature: "macro_rules! param_value {"
      - id: ParamBlock
        signature: "pub struct ParamBlock {"
        docstring: "Fixed-capacity buffer for dynamic kernel parameters, staged on the host and mir…"
      - id: impl_ParamBlock
        signature: "impl ParamBlock {"
        children:
        - id: impl_ParamBlock.new
          signature: "pub fn new(capacity: usize) -> Self {"
        - id: impl_ParamBlock.capacity
          signature: "pub fn capacity(&self) -> usize {"
        - id: impl_ParamBlock.bind
          signature: "pub fn bind(&mut self, region: Region) {"
        - id: impl_ParamBlock.region
          signature: "pub fn region(&self) -> Option<Region> {"
        - id: impl_ParamBlock.clear
          signature: "pub fn clear(&mut self) {"
        - id: impl_ParamBlock.as_bytes
          signature: "pub fn as_bytes(&self) -> &[u8] {"
        - id: impl_ParamBlock.push
          signature: "pub fn push<T: ParamValue>(&mut self, value: T) -> Result<usize, DeviceError> {"
          docstring: Appends `value` naturally aligned and returns its byte offset.
      - id: ScratchArena
        signature: "pub struct ScratchArena {"
        docstring: Bump allocator over a lane-private arena span; reset once the lane has drained.
      - id: impl_ScratchArena
        signature: "impl ScratchArena {"
        children:
        - id: impl_ScratchArena.new
          signature: "pub fn new(base: Region) -> Self {"
        - id: impl_ScratchArena.capacity
          signature: "pub fn capacity(&self) -> u64 {"
        - id: impl_ScratchArena.used
          signature: "pub fn used(&self) -> u64 {"
        - id: impl_ScratchArena.alloc
          signature: "pub fn alloc(&self, bytes: u64, align: u64) -> Result<Region, DeviceError> {"
        - id: impl_ScratchArena.reset
          signature: "pub fn reset(&self) {"
          docstring: Invalidates all regions handed out so far.
      - id: ExecContext
        signature: "pub trait ExecContext: Send {"
        docstring: "In-order execution queue of one device. Device handles, streams and pointers ne…"
        children:
        - id: ExecContext.class
          signature: fn class(&self) -> LaneClass;
        - id: ExecContext.device
          signature: fn device(&self) -> DeviceKind;
        - id: ExecContext.submit
          signature: "fn submit(&self, job: LaneJob) -> Result<(), DeviceError>;"
        - id: ExecContext.record_event
          signature: "fn record_event(&self) -> Result<EventHandle, DeviceError>;"
          docstring: Completes once everything submitted so far has finished.
        - id: ExecContext.wait
          signature: "fn wait(&self, event: &EventHandle) -> Result<(), DeviceError>;"
          docstring: Orders all later work on this lane after `event`.
        - id: ExecContext.synchronize
          signature: "fn synchronize(&self) -> Result<(), DeviceError>;"
          docstring: Blocks until the lane has drained; returns the first job failure since the last…
        - id: ExecContext.param_block
          signature: fn param_block(&mut self) -> &mut ParamBlock;
        - id: ExecContext.scratch
          signature: fn scratch(&self) -> &ScratchArena;
      - id: Msg
        signature: "enum Msg {"
      - id: CpuLane
        signature: "pub struct CpuLane {"
        docstring: "CPU lane: one worker thread, so submission order is execution order."
      - id: impl_CpuLane
        signature: "impl CpuLane {"
        children:
        - id: impl_CpuLane.new
          signature: "pub fn new(class: LaneClass, cfg: LaneConfig) -> Result<Self, DeviceError> {"
        - id: impl_CpuLane.send
          signature: "fn send(&self, msg: Msg) -> Result<(), DeviceError> {"
      - id: run_worker
        signature: "fn run_worker(rx: Receiver<Msg>, failure: &Mutex<Option<DeviceError>>) {"
      - id: impl_ExecContext_for_CpuLane
        signature: "impl ExecContext for CpuLane {"
        children:
        - id: impl_ExecContext_for_CpuLane.class
          signature: "fn class(&self) -> LaneClass {"
        - id: impl_ExecContext_for_CpuLane.device
          signature: "fn device(&self) -> DeviceKind {"
        - id: impl_ExecContext_for_CpuLane.submit
          signature: "fn submit(&self, job: LaneJob) -> Result<(), DeviceError> {"
        - id: impl_ExecContext_for_CpuLane.record_event
          signature: "fn record_event(&self) -> Result<EventHandle, DeviceError> {"
        - id: impl_ExecContext_for_CpuLane.wait
          signature: "fn wait(&self, event: &EventHandle) -> Result<(), DeviceError> {"
        - id: impl_ExecContext_for_CpuLane.synchronize
          signature: "fn synchronize(&self) -> Result<(), DeviceError> {"
        - id: impl_ExecContext_for_CpuLane.param_block
          signature: "fn param_block(&mut self) -> &mut ParamBlock {"
        - id: impl_ExecContext_for_CpuLane.scratch
          signature: "fn scratch(&self) -> &ScratchArena {"
      - id: impl_Drop_for_CpuLane
        signature: "impl Drop for CpuLane {"
        children:
        - id: impl_Drop_for_CpuLane.drop
          signature: "fn drop(&mut self) {"
      - id: RecordingLane
        signature: "pub struct RecordingLane {"
        docstring: "Lane double for downstream tests: runs jobs inline and logs every call."
      - id: impl_RecordingLane
        signature: "impl RecordingLane {"
        children:
        - id: impl_RecordingLane.new
          signature: "pub fn new(class: LaneClass, cfg: LaneConfig) -> Self {"
        - id: impl_RecordingLane.calls
          signature: "pub fn calls(&self) -> Vec<&'static str> {"
      - id: impl_ExecContext_for_RecordingLane
        signature: "impl ExecContext for RecordingLane {"
        children:
        - id: impl_ExecContext_for_RecordingLane.class
          signature: "fn class(&self) -> LaneClass {"
        - id: impl_ExecContext_for_RecordingLane.device
          signature: "fn device(&self) -> DeviceKind {"
        - id: impl_ExecContext_for_RecordingLane.submit
          signature: "fn submit(&self, job: LaneJob) -> Result<(), DeviceError> {"
        - id: impl_ExecContext_for_RecordingLane.record_event
          signature: "fn record_event(&self) -> Result<EventHandle, DeviceError> {"
        - id: impl_ExecContext_for_RecordingLane.wait
          signature: "fn wait(&self, _event: &EventHandle) -> Result<(), DeviceError> {"
        - id: impl_ExecContext_for_RecordingLane.synchronize
          signature: "fn synchronize(&self) -> Result<(), DeviceError> {"
        - id: impl_ExecContext_for_RecordingLane.param_block
          signature: "fn param_block(&mut self) -> &mut ParamBlock {"
        - id: impl_ExecContext_for_RecordingLane.scratch
          signature: "fn scratch(&self) -> &ScratchArena {"
      - id: tests
        signature: mod tests;
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/subengine/mod.rs
      nodes:
      - id: ModuleDoc
        signature: "//! Subengine resource lifecycle, pull capacity reporting, device handles and e…"
      - id: 77cuty|ChE8yG
        signature: use crate::resident::backend::ComputeBackend;
      - id: CpuBackend
        signature: pub struct CpuBackend;
        docstring: "CPU backend; default without `cuda`, and the fallback target if CUDA init fails."
      - id: impl_ComputeBackend_for_CpuBackend
        signature: "impl ComputeBackend for CpuBackend {"
        children:
        - id: impl_ComputeBackend_for_CpuBackend.run_batch
          signature: "fn run_batch(&self, input: &[f32]) -> Vec<f32> {"
      - id: device
        signature: mod device;
      - id: exec
        signature: mod exec;
      - id: error
        signature: mod error;
      - id: lifecycle
        signature: mod lifecycle;
      - id: resource
        signature: mod resource;
      - id: wLnUuf|YSWcPL
        signature: pub use device::*;
      - id: SubengineKind
        signature: "pub enum SubengineKind {"
      - id: Subengine
        signature: "pub trait Subengine: Send + Sync {"
        docstring: Resources and execution capacity of one compute substrate. Work arrives through…
        children:
        - id: Subengine.kind
          signature: fn kind(&self) -> SubengineKind;
        - id: Subengine.device
          signature: fn device(&self) -> &DeviceHandle;
        - id: Subengine.capacity
          signature: fn capacity(&self) -> Capacity;
        - id: Subengine.throughput
          signature: fn throughput(&self) -> f32;
          docstring: Smoothed items per second.
        - id: Subengine.batch_size
          signature: fn batch_size(&self) -> usize;
          docstring: "Throughput-derived batch size, independent of current load."
        - id: Subengine.pull_budget
          signature: "fn pull_budget(&self) -> usize {"
          docstring: Upper bound on the jobs to hand this subengine in one `Claim::claim` call.
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel/launcher.rs
      nodes:
      - id: ModuleDoc
        signature: //! Launch indirection between scorers and lanes.
      - id: Ofgz1c|5vdUur
        signature: use std::sync::Arc;
      - id: LaunchMode
        signature: "pub enum LaunchMode {"
      - id: LaunchKey
        signature: "pub struct LaunchKey {"
        docstring: "Identity of a launch. Regions carry their generation, so a generation change yi…"
      - id: Launcher
        signature: "pub trait Launcher: Send + Sync {"
        children:
        - id: Launcher.mode
          signature: fn mode(&self) -> LaunchMode;
        - id: Launcher.launch
          signature: "fn launch(     &self,     key: &LaunchKey,     lane: &dyn ExecContext,     job:…"
      - id: DirectLauncher
        signature: pub struct DirectLauncher;
      - id: impl_Launcher_for_DirectLauncher
        signature: "impl Launcher for DirectLauncher {"
        children:
        - id: impl_Launcher_for_DirectLauncher.mode
          signature: "fn mode(&self) -> LaunchMode {"
        - id: impl_Launcher_for_DirectLauncher.launch
          signature: "level = \"trace\",     skip_all,     fields(op = key.op, bucket = key.shape_bucke…"
      - id: launcher_for
        signature: "pub fn launcher_for(mode: LaunchMode) -> Result<Arc<dyn Launcher>, KernelError>…"
      - id: tests
        signature: mod tests;
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/manager/residency.rs
      nodes:
      - id: ModuleDoc
        signature: "//! Load strategy: how a resolved entry set reaches the device (whole segments,…"
      - id: sbI38B|uLAjNU
        signature: use std::collections::BTreeMap;
      - id: SegmentId
        signature: pub type SegmentId = u64;
      - id: 7MWN2b|eoQwYL
        signature: "const MIB: u64 = 1 << 20;"
      - id: LoadGeometry
        signature: "pub trait LoadGeometry: Send + Sync {"
        docstring: Where segment boundaries and entry sizes come from; implemented by the segment …
        children:
        - id: LoadGeometry.segment_of
          signature: "fn segment_of(&self, entry: EntryId) -> SegmentId;"
        - id: LoadGeometry.entry_bytes
          signature: "fn entry_bytes(&self, entry: EntryId) -> u64;"
        - id: LoadGeometry.segment_bytes
          signature: "fn segment_bytes(&self, segment: SegmentId) -> u64;"
      - id: UniformGeometry
        signature: "pub struct UniformGeometry {"
        docstring: Equal-sized entries in consecutive segments of equal size.
      - id: impl_LoadGeometry_for_UniformGeometry
        signature: "impl LoadGeometry for UniformGeometry {"
        children:
        - id: impl_LoadGeometry_for_UniformGeometry.segment_of
          signature: "fn segment_of(&self, entry: EntryId) -> SegmentId {"
        - id: impl_LoadGeometry_for_UniformGeometry.entry_bytes
          signature: "fn entry_bytes(&self, _: EntryId) -> u64 {"
        - id: impl_LoadGeometry_for_UniformGeometry.segment_bytes
          signature: "fn segment_bytes(&self, _: SegmentId) -> u64 {"
      - id: Access
        signature: "pub enum Access {"
      - id: LoadPlan
        signature: "pub enum LoadPlan {"
      - id: impl_LoadPlan
        signature: "impl LoadPlan {"
        children:
        - id: impl_LoadPlan.kind
          signature: "fn kind(&self) -> &'static str {"
      - id: LoadPolicy
        signature: "pub struct LoadPolicy {"
      - id: impl_Default_for_LoadPolicy
        signature: "impl Default for LoadPolicy {"
        children:
        - id: impl_Default_for_LoadPolicy.default
          signature: "fn default() -> Self {"
      - id: impl_LoadPolicy
        signature: "impl LoadPolicy {"
        children:
        - id: impl_LoadPolicy.with_density_threshold
          signature: "pub fn with_density_threshold(mut self, threshold: f32) -> Self {"
        - id: impl_LoadPolicy.sized_for
          signature: "pub fn sized_for(mut self, budget: &ArenaBudget) -> Self {"
          docstring: "Segment size from the free device budget: one eighth, within 4..=256 MiB, whole…"
        - id: impl_LoadPolicy.apply_profile
          signature: "pub fn apply_profile(self, config: &ProfileConfig) -> Self {"
          docstring: Takes the threshold from the calibrated profile when it carries the `density` a…
        - id: impl_LoadPolicy.density_value
          signature: "pub fn density_value(&self) -> ValueId {"
          docstring: "Value of the `density` calibration axis, in permille."
      - id: plan_load
        signature: "pub fn plan_load(     set: &EntrySet,     geometry: &dyn LoadGeometry,     poli…"
        docstring: Chooses the transfer shape. Segments whose requested share reaches the threshol…
      - id: LoadTransfer
        signature: "pub trait LoadTransfer: Send + Sync {"
        docstring: Device-side execution of a plan; each method returns the bytes moved.
        children:
        - id: LoadTransfer.load_segments
          signature: "fn load_segments(&self, segments: &[SegmentId]) -> Result<u64, ManagerError>;"
        - id: LoadTransfer.load_gather
          signature: "fn load_gather(&self, set: &EntrySet) -> Result<u64, ManagerError>;"
        - id: LoadTransfer.map_zero_copy
          signature: "fn map_zero_copy(&self, set: &EntrySet) -> Result<u64, ManagerError>;"
      - id: LoadOutcome
        signature: "pub struct LoadOutcome {"
      - id: execute
        signature: "fn execute(     transfer: &dyn LoadTransfer,     plans: &[LoadPlan], ) -> Resul…"
      - id: Aggregated
        signature: "pub struct Aggregated<K> {"
      - id: LoadAggregator
        signature: "pub struct LoadAggregator<K> {"
        docstring: Collects the scopes of tasks against the same index so each index loads once ov…
      - id: impl_Default_for_LoadAggregator_K_
        signature: "impl<K> Default for LoadAggregator<K> {"
        children:
        - id: impl_Default_for_LoadAggregator_K_.default
          signature: "fn default() -> Self {"
      - id: impl_LoadAggregator_K_
        signature: "impl<K: Ord> LoadAggregator<K> {"
        children:
        - id: impl_LoadAggregator_K_.new
          signature: "pub fn new() -> Self {"
        - id: impl_LoadAggregator_K_.add
          signature: "pub fn add(&mut self, index: K, set: &EntrySet) {"
        - id: impl_LoadAggregator_K_.is_empty
          signature: "pub fn is_empty(&self) -> bool {"
        - id: impl_LoadAggregator_K_.drain
          signature: "pub fn drain(&mut self) -> Vec<Aggregated<K>> {"
      - id: ResidencyLoader
        signature: "pub struct ResidencyLoader<K> {"
      - id: impl_ResidencyLoader_K_
        signature: "impl<K> ResidencyLoader<K> where     K: Clone + Eq + Hash + Ord + Send + Sync +…"
        children:
        - id: impl_ResidencyLoader_K_.new
          signature: "pub fn new(     policy: LoadPolicy,     geometry: Arc<dyn LoadGeometry>,     tr…"
        - id: impl_ResidencyLoader_K_.policy
          signature: "pub fn policy(&self) -> &LoadPolicy {"
        - id: impl_ResidencyLoader_K_.inflight
          signature: "pub fn inflight(&self) -> usize {"
        - id: impl_ResidencyLoader_K_.ensure
          signature: "pub async fn ensure(     &self,     index: K,     set: EntrySet,     access: Ac…"
          docstring: Identical requests in flight share one transfer; an error reaches every waiter.
        - id: impl_ResidencyLoader_K_.flush
          signature: "pub async fn flush(     &self,     aggregator: &mut LoadAggregator<K>,     acce…"
          docstring: Drains the aggregator and loads every index once over the union of its requests.
      - id: tests
        signature: mod tests;
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/manager/swap.rs
      nodes:
      - id: ModuleDoc
        signature: "//! Mutation discipline of the index manager: copy-on-write segment tables, a s…"
      - id: 1gsLkl|IB2Xne
        signature: use std::sync::Arc;
      - id: P9fcuJ|yFCGCK
        signature: /// Root value before the first table is published.
      - id: Cell
        signature: "enum Cell {"
      - id: RootPointer
        signature: "pub struct RootPointer {"
        docstring: Naturally aligned 8-byte cell. Only the host stores; the device reads with `ld.…
      - id: impl_Send_for_RootPointer
        signature: "unsafe impl Send for RootPointer {"
      - id: impl_Sync_for_RootPointer
        signature: "unsafe impl Sync for RootPointer {"
      - id: impl_RootPointer
        signature: "impl RootPointer {"
        children:
        - id: impl_RootPointer.new
          signature: "pub fn new(initial: u64) -> Self {"
        - id: impl_RootPointer.from_mapped
          signature: "pub(crate) unsafe fn from_mapped(ptr: *mut u64) -> Self {"
          docstring: "Wraps a cell in pinned, mapped host memory."
        - id: impl_RootPointer.atomic
          signature: "fn atomic(&self) -> &AtomicU64 {"
        - id: impl_RootPointer.publish
          signature: "pub fn publish(&self, value: u64) {"
          docstring: Everything written before this call is visible to a reader that observes the va…
        - id: impl_RootPointer.load
          signature: "pub fn load(&self) -> u64 {"
      - id: SegmentTable
        signature: "pub struct SegmentTable {"
        docstring: Immutable once published; a new version is always a copy.
      - id: impl_SegmentTable
        signature: "impl SegmentTable {"
        children:
        - id: impl_SegmentTable.empty
          signature: "pub fn empty() -> Self {"
        - id: impl_SegmentTable.epoch
          signature: "pub fn epoch(&self) -> u64 {"
        - id: impl_SegmentTable.segments
          signature: "pub fn segments(&self) -> &[Region] {"
        - id: impl_SegmentTable.push
          signature: "pub fn push(&mut self, region: Region) {"
        - id: impl_SegmentTable.replace
          signature: "pub fn replace(&mut self, old: Region, new: Region) -> bool {"
          docstring: Swaps `old` for `new` in place of the copy; returns false when `old` is not lis…
        - id: impl_SegmentTable.remove
          signature: "pub fn remove(&mut self, region: Region) -> bool {"
        - id: impl_SegmentTable.encoded_len
          signature: "pub fn encoded_len(&self) -> u64 {"
          docstring: "Encoded size of the device-side table: epoch, count, then one entry per segment."
      - id: EpochTracker
        signature: "pub struct EpochTracker {"
        docstring: Versions published by the host and the epoch each reader reported having read.
      - id: impl_EpochTracker
        signature: "impl EpochTracker {"
        children:
        - id: impl_EpochTracker.new
          signature: "pub fn new(readers: usize) -> Self {"
        - id: impl_EpochTracker.published
          signature: "pub fn published(&self) -> u64 {"
        - id: impl_EpochTracker.advance
          signature: "fn advance(&self, epoch: u64) {"
        - id: impl_EpochTracker.report
          signature: "pub fn report(&self, reader: usize, epoch: u64) {"
          docstring: A finished batch read `epoch`; later batches of the reader read the same or a n…
        - id: impl_EpochTracker.quiesce
          signature: "pub fn quiesce(&self, reader: usize) {"
          docstring: "For a reader with no batch in flight: whatever it runs next reads the published…"
        - id: impl_EpochTracker.reader_min
          signature: "pub fn reader_min(&self) -> u64 {"
      - id: TableSink
        signature: "pub trait TableSink: Send + Sync {"
        docstring: Writes a table into device-visible memory.
        children:
        - id: TableSink.write
          signature: "fn write(&self, table: &SegmentTable) -> Result<Region, ManagerError>;"
          docstring: "Returns only once the content is complete and visible to the device, i.e. after…"
      - id: TableSwap
        signature: "pub trait TableSwap {"
        children:
        - id: TableSwap.build_next
          signature: fn build_next(&self) -> SegmentTable;
          docstring: Copy of the current table with the next epoch.
        - id: TableSwap.publish
          signature: "fn publish(&self, table: SegmentTable) -> Result<u64, ManagerError>;"
          docstring: "Makes the content visible, then swaps the root with release semantics. Returns …"
        - id: TableSwap.reclaim
          signature: "fn reclaim(&self, upto: u64) -> usize;"
          docstring: Frees versions retired up to `upto` once every reader has reached them.
      - id: Snapshot
        signature: "pub struct Snapshot {"
        docstring: Root and epoch seen by one launch or one persistent batch.
      - id: BatchGuard
        signature: "pub struct BatchGuard<'a> {"
        docstring: Pins the version read at batch start; reports its epoch when dropped.
      - id: impl_BatchGuard____
        signature: "impl BatchGuard<'_> {"
        children:
        - id: impl_BatchGuard____.snapshot
          signature: "pub fn snapshot(&self) -> Snapshot {"
      - id: impl_Drop_for_BatchGuard____
        signature: "impl Drop for BatchGuard<'_> {"
        children:
        - id: impl_Drop_for_BatchGuard____.drop
          signature: "fn drop(&mut self) {"
      - id: Current
        signature: "struct Current {"
      - id: SwapManager
        signature: "pub struct SwapManager<'a, 'b> {"
      - id: impl_SwapManager__a__b_
        signature: "impl<'a, 'b> SwapManager<'a, 'b> {"
        children:
        - id: impl_SwapManager__a__b_.new
          signature: "pub fn new(     arena: &'a IndexArena<'b>,     sink: Box<dyn TableSink + 'a>,  …"
        - id: impl_SwapManager__a__b_.with_root
          signature: "pub fn with_root(     arena: &'a IndexArena<'b>,     sink: Box<dyn TableSink + …"
        - id: impl_SwapManager__a__b_.root
          signature: "pub fn root(&self) -> &RootPointer {"
        - id: impl_SwapManager__a__b_.epochs
          signature: "pub fn epochs(&self) -> &EpochTracker {"
        - id: impl_SwapManager__a__b_.current
          signature: "pub fn current(&self) -> Arc<SegmentTable> {"
        - id: impl_SwapManager__a__b_.snapshot
          signature: "pub fn snapshot(&self) -> Snapshot {"
          docstring: "Normal launches: the root is passed as a launch argument, so the version stays …"
        - id: impl_SwapManager__a__b_.begin_batch
          signature: "pub fn begin_batch(&self, reader: usize) -> BatchGuard<'_> {"
          docstring: "Persistent kernel: one acquire load per batch, never per work item."
      - id: impl_TableSwap_for_SwapManager_______
        signature: "impl TableSwap for SwapManager<'_, '_> {"
        children:
        - id: impl_TableSwap_for_SwapManager_______.build_next
          signature: "fn build_next(&self) -> SegmentTable {"
        - id: impl_TableSwap_for_SwapManager_______.publish
          signature: "fn publish(&self, table: SegmentTable) -> Result<u64, ManagerError> {"
        - id: impl_TableSwap_for_SwapManager_______.reclaim
          signature: "fn reclaim(&self, upto: u64) -> usize {"
      - id: tests
        signature: "mod tests {"
        children:
        - id: tests.flTjQt|5qRwZw
          signature: use super::*;
        - id: tests.HostSink
          signature: "struct HostSink<'a, 'b>(&'a IndexArena<'b>);"
        - id: tests.impl_TableSink_for_HostSink_______
          signature: "impl TableSink for HostSink<'_, '_> {"
          children:
          - id: tests.impl_TableSink_for_HostSink_______.write
            signature: "fn write(&self, table: &SegmentTable) -> Result<Region, ManagerError> {"
        - id: tests.alloc
          signature: "fn alloc() -> ArenaAllocator {"
        - id: tests.seg
          signature: "fn seg(offset: u64) -> Region {"
        - id: tests.publish_swaps_root_and_advances_epoch
          signature: "fn publish_swaps_root_and_advances_epoch() {"
        - id: tests.concurrent_builds_on_the_same_base_are_rejected
          signature: "fn concurrent_builds_on_the_same_base_are_rejected() {"
        - id: tests.old_table_is_freed_only_after_all_readers_reach_the_new_epoch
          signature: "fn old_table_is_freed_only_after_all_readers_reach_the_new_epoch() {"
        - id: tests.reclaim_respects_the_requested_bound
          signature: "fn reclaim_respects_the_requested_bound() {"
        - id: tests.launch_snapshot_is_constant_until_the_next_launch
          signature: "fn launch_snapshot_is_constant_until_the_next_launch() {"
        - id: tests.copy_is_independent_of_the_published_table
          signature: "fn copy_is_independent_of_the_published_table() {"
      - id: loom_tests
        signature: "mod loom_tests {"
        children:
        - id: loom_tests.aCv6R1|7NCwfi
          signature: use super::*;
        - id: loom_tests.content_is_visible_once_the_root_is_observed
          signature: "fn content_is_visible_once_the_root_is_observed() {"
        - id: loom_tests.retired_version_is_not_reclaimed_while_a_reader_holds_it
          signature: "fn retired_version_is_not_reclaimed_while_a_reader_holds_it() {"
      - id: device_tests
        signature: "mod device_tests {"
        children:
        - id: device_tests.XiB8kc|gxjU93
          signature: use super::*;
        - id: device_tests.obwXJ0|6OEnCW
          signature: "const PROBE: &str = r#\""
        - id: device_tests.device_sees_content_written_before_the_host_release_store
          signature: "fn device_sees_content_written_before_the_host_release_store() {"
        - id: device_tests.xreCHs|1RgSqu
          signature: "const PROBE_DEVICE: &str = r#\""
        - id: device_tests.copy_stream_event_orders_device_table_before_the_root
          signature: "fn copy_stream_event_orders_device_table_before_the_root() {"
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/manager/arena.rs
      nodes:
      - id: ModuleDoc
        signature: "//! Index-segment use of the shared arena allocator: aligned segments, copy-on-…"
      - id: Cl3g3y|QQdjOc
        signature: "use std::ops::{Deref, Range};"
      - id: WRTBpG|CxUsUc
        signature: /// Minimum alignment of every segment offset.
      - id: CowBudget
        signature: "pub struct CowBudget {"
        docstring: Space that must stay free so a replaced segment can be rebuilt next to its pred…
      - id: impl_CowBudget
        signature: "impl CowBudget {"
        children:
        - id: impl_CowBudget.headroom
          signature: "pub fn headroom(&self) -> u64 {"
        - id: impl_CowBudget.required
          signature: "pub fn required(&self) -> u64 {"
      - id: ArenaBudget
        signature: "pub struct ArenaBudget {"
      - id: Retired
        signature: "struct Retired {"
      - id: State
        signature: "struct State {"
      - id: impl_State
        signature: "impl State {"
        children:
        - id: impl_State.is_retired
          signature: "fn is_retired(&self, region: Region) -> bool {"
      - id: AllocRef
        signature: "enum AllocRef<'a> {"
      - id: impl_Deref_for_AllocRef____
        signature: "impl Deref for AllocRef<'_> {"
        children:
        - id: impl_Deref_for_AllocRef____.jj8Fh8|OlMOr4
          signature: type Target = ArenaAllocator;
        - id: impl_Deref_for_AllocRef____.deref
          signature: "fn deref(&self) -> &ArenaAllocator {"
      - id: IndexArena
        signature: "pub struct IndexArena<'a> {"
      - id: impl_IndexArena__a_
        signature: "impl<'a> IndexArena<'a> {"
        children:
        - id: impl_IndexArena__a_.new
          signature: "pub fn new(inner: &'a ArenaAllocator, reserve: u64) -> Self {"
          docstring: '`reserve` bytes of the arena are never handed out.'
        - id: impl_IndexArena__a_.shared
          signature: "pub fn shared(inner: Arc<ArenaAllocator>, reserve: u64) -> IndexArena<'static> {"
          docstring: "Owns a share of the allocator, so the arena can sit behind `'static` trait obje…"
        - id: impl_IndexArena__a_.with_ref
          signature: "fn with_ref(inner: AllocRef<'a>, reserve: u64) -> Self {"
        - id: impl_IndexArena__a_.alloc_segment
          signature: "pub fn alloc_segment(&self, bytes: u64, align: u64) -> Result<Region, ManagerEr…"
        - id: impl_IndexArena__a_.reserve_writable
          signature: "pub fn reserve_writable(&self, cap: u64) -> Result<WritableRegion, ManagerError…"
          docstring: "Segment the build path fills incrementally through [`WritableRegion::claim`]."
        - id: impl_IndexArena__a_.free
          signature: "pub fn free(&self, region: Region) -> Result<(), ManagerError> {"
          docstring: Immediate release of a region no reader or in-flight work can reference.
        - id: impl_IndexArena__a_.free_after
          signature: "pub fn free_after(     &self,     region: Region,     event: EventHandle, ) -> …"
          docstring: Release once all work recorded before `event` has finished.
        - id: impl_IndexArena__a_.retire
          signature: "pub fn retire(&self, region: Region, epoch: u64) -> Result<(), ManagerError> {"
          docstring: Marks `region` as replaced by the version published at `epoch`.
        - id: impl_IndexArena__a_.reclaim
          signature: "pub fn reclaim(&self, min_reader_epoch: u64) -> usize {"
          docstring: Frees every retired region that no reader at `min_reader_epoch` or later can st…
        - id: impl_IndexArena__a_.begin_cow
          signature: "pub fn begin_cow(&self, old: Region) -> Result<CowTicket<'_, 'a>, ManagerError>…"
          docstring: Starts a copy-on-write replacement of `old`; waits while another one is in prog…
        - id: impl_IndexArena__a_.try_begin_cow
          signature: "pub fn try_begin_cow(     &self,     old: Region, ) -> Result<Option<CowTicket<…"
          docstring: "Like [`begin_cow`](Self::begin_cow), but `Ok(None)` when another replacement is…"
        - id: impl_IndexArena__a_.cow_budget
          signature: "pub fn cow_budget(&self) -> CowBudget {"
        - id: impl_IndexArena__a_.budget
          signature: "pub fn budget(&self) -> ArenaBudget {"
        - id: impl_IndexArena__a_.trace_utilization
          signature: "pub fn trace_utilization(&self) {"
        - id: impl_IndexArena__a_.start_cow
          signature: "fn start_cow<'g>(     &'g self,     gate: MutexGuard<'g, ()>,     old: Region, …"
        - id: impl_IndexArena__a_.admit_and_alloc
          signature: "fn admit_and_alloc(     &self,     bytes: u64,     align: u64,     headroom: u6…"
        - id: impl_IndexArena__a_.ensure_owned
          signature: "fn ensure_owned(&self, st: &State, region: Region) -> Result<(), ManagerError> {"
      - id: CowTicket
        signature: "pub struct CowTicket<'a, 'b> {"
        docstring: Replacement segment under construction. Dropping it without `commit` discards t…
      - id: impl_CowTicket_______
        signature: "impl CowTicket<'_, '_> {"
        children:
        - id: impl_CowTicket_______.old_region
          signature: "pub fn old_region(&self) -> Region {"
        - id: impl_CowTicket_______.new_region
          signature: "pub fn new_region(&self) -> Region {"
        - id: impl_CowTicket_______.commit
          signature: "pub fn commit(mut self, epoch: u64) -> Result<Region, ManagerError> {"
          docstring: Publishes the copy as the version of `epoch`; the old segment is released by `r…
      - id: impl_Drop_for_CowTicket_______
        signature: "impl Drop for CowTicket<'_, '_> {"
        children:
        - id: impl_Drop_for_CowTicket_______.drop
          signature: "fn drop(&mut self) {"
      - id: WritableRegion
        signature: "pub struct WritableRegion {"
        docstring: Segment filled front to back by a single writer.
      - id: impl_WritableRegion
        signature: "impl WritableRegion {"
        children:
        - id: impl_WritableRegion.region
          signature: "pub fn region(&self) -> Region {"
        - id: impl_WritableRegion.written
          signature: "pub fn written(&self) -> u64 {"
        - id: impl_WritableRegion.remaining
          signature: "pub fn remaining(&self) -> u64 {"
        - id: impl_WritableRegion.claim
          signature: "pub fn claim(&mut self, len: u64, align: u64) -> Result<Range<u64>, ManagerErro…"
          docstring: Claims `len` bytes at `align`; returns the absolute arena offsets.
        - id: impl_WritableRegion.into_parts
          signature: "pub fn into_parts(self) -> (Region, u64) {"
      - id: map_memory
        signature: "fn map_memory(e: MemoryError) -> ManagerError {"
      - id: tests
        signature: mod tests;
```
/answer 4172-8 allow
Control Request:
```yaml
id: 4172-9
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src/resident
    pattern: "enum ArenaKind|enum ArenaClass|struct Region\\b|enum LaunchMode|ParamBlock,|Staging"
    include:
    - '*.rs'
    exclude:
    - target/**
    limit: 30
  reason: Arena-Klassen-Typ und Region-Definition finden
```
/answer 4172-9 allow
Control Request:
```yaml
id: 4172-a
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src/resident
      matches:
      - path: kernel/launcher.rs
        matches:
        - "pub enum LaunchMode {"
      - path: subengine/mod.rs
        matches:
        - "CpuLane, EventHandle, ExecContext, LaneClass, LaneConfig, LaneJob, ParamBlock,"
      - path: subengine/device.rs
        matches:
        - "use crate::resident::subengine::{EventHandle, LaneJob, ParamBlock, ScratchArena};"
        - "params: ParamBlock,"
      - path: subengine/exec.rs
        matches:
        - "params: ParamBlock,"
      - path: manager/upload.rs
        matches:
        - use crate::resident::memory::StagingRing;
        - "pub trait StagingBuffer: Send {"
        - "impl StagingBuffer for Vec<u8> {"
        - "impl StagingBuffer for cudarc::driver::PinnedHostSlice<u8> {"
        - "ring: StagingRing<S>,"
        - "impl<S: StagingBuffer> StagedGather<S> {"
        - "pub fn new(ring: StagingRing<S>) -> Self {"
        - "fn pack<S: StagingBuffer>("
        - "pub struct Prepared<'a, S: StagingBuffer> {"
        - "impl<S: StagingBuffer> Prepared<'_, S> {"
        - "impl<S: StagingBuffer> Drop for Prepared<'_, S> {"
      - path: manager/transfer.rs
        matches:
        - "Placement, Prepared, StagedGather, StagingBuffer, UploadPort,"
        - "pub struct GatherTransfer<S: StagingBuffer> {"
        - "impl<S: StagingBuffer> GatherTransfer<S> {"
        - "impl<S: StagingBuffer + 'static> LoadTransfer for GatherTransfer<S> {"
      - path: manager/test/upload.rs
        matches:
        - "StagedGather::new(StagingRing::new((0..slots).map(|_| vec![0u8; slot_bytes])))"
        - "let scratch = memory.alloc_in(ArenaKind::ParamBlock, 256, 8).unwrap();"
      - path: manager/test/transfer.rs
        matches:
        - "use crate::resident::memory::{ArenaAllocator, ArenaKind, DeviceKind, StagingRing};"
        - "StagingRing::new((0..2).map(|_| vec![0u8; slot_bytes])),"
        - "StagedGather::new(StagingRing::new((0..2).map(|_| vec![0u8; slot_bytes]))),"
        - "StagingRing::new((0..2).map(|_| vec![0u8; staging_bytes])),"
        - "StagedGather::new(StagingRing::new((0..2).map(|_| vec![0u8; 4096]))),"
        - "let scratch = memory.alloc_in(ArenaKind::ParamBlock, 256, 8).unwrap();"
      warning: "Limit of 30 matches reached; further results may exist. Narrow the pattern, directory or include/exclude filters, or raise limit."
```
/answer 4172-a allow
Control Request:
```yaml
id: 4172-b
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/subengine/exec.rs
    ids:
    - LaneClass
    - impl_LaneClass
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel/launcher.rs
    ids:
    - LaunchMode
    - Launcher
    - launcher_for
  reason: LaneClass- und LaunchMode-Definitionen auf Erweiterbarkeit prüfen
```
/answer 4172-b allow
ReasoningToken: 73
Control Request:
```yaml
id: 4172-c
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/subengine/exec.rs
      nodes:
      - id: LaneClass
        type: ItemEnum
        code: |
          #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
          pub enum LaneClass {
              Query,
              Index,
              Copy,
          }
      - id: impl_LaneClass
        type: ItemImpl
        signature: "impl LaneClass {"
        children:
        - id: impl_LaneClass.priority
          type: ImplItemFn
          code: |2
                /// CUDA stream priority; lower values are more urgent and `0` is the default. Priorities only
                /// influence block scheduling and never preempt running blocks.
                pub fn priority(self) -> i32 {
                    match self {
                        LaneClass::Query => -5,
                        LaneClass::Copy => -1,
                        LaneClass::Index => 0,
                    }
                }
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel/launcher.rs
      nodes:
      - id: LaunchMode
        type: ItemEnum
        code: |
          #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
          pub enum LaunchMode {
              Direct,
              Capture,
          }
      - id: Launcher
        type: ItemTrait
        signature: "pub trait Launcher: Send + Sync {"
        children:
        - id: Launcher.mode
          type: TraitItemFn
          code: |2
                fn mode(&self) -> LaunchMode;
        - id: Launcher.launch
          type: TraitItemFn
          code: |2
                fn launch(
                    &self,
                    key: &LaunchKey,
                    lane: &dyn ExecContext,
                    job: LaneJob,
                ) -> Result<(), KernelError>;
      - id: launcher_for
        type: ItemFn
        code: |
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
```
/answer 4172-c allow
Control Request:
```yaml
id: 4172-d
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src/resident
    pattern: enum ArenaKind|ArenaKind::|struct Region|blocked|Blocked|non_exhaustive
    include:
    - '*.rs'
    exclude:
    - target/**
    limit: 50
  reason: "ArenaKind, Region, vorhandene Blocking-Logik und non_exhaustive-Nutzung lokalisieren"
```
/answer 4172-d allow
Control Request:
```yaml
id: 4172-e
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src/resident
      matches:
      - path: segment/mod.rs
        matches:
        - "#[non_exhaustive]"
      - path: segment/error.rs
        matches:
        - "#[non_exhaustive]"
      - path: processor/config.rs
        matches:
        - "#[non_exhaustive]"
      - path: processor/error.rs
        matches:
        - "#[non_exhaustive]"
      - path: precision/test/matching.rs
        matches:
        - "fn empty_model_is_blocked_after_first_failure() {"
        - assert!(!selector.is_blocked("m"));
        - assert!(selector.is_blocked("m"));
      - path: precision/matching.rs
        matches:
        - "blocked: Mutex<HashSet<String>>,"
        - "pub fn is_blocked(&self, model_id: &str) -> bool {"
        - "if self.is_blocked(model_id) {"
        - "tracing::warn!(model_id, \"no active profile, model blocked\");"
        - self.blocked.lock().unwrap_or_else(|e| e.into_inner())
      - path: facade/resident.rs
        matches:
        - "#[non_exhaustive]"
      - path: facade/params.rs
        matches:
        - "#[non_exhaustive]"
      - path: facade/error.rs
        matches:
        - "#[non_exhaustive]"
      - path: facade/task.rs
        matches:
        - "#[non_exhaustive]"
      - path: index/mod.rs
        matches:
        - "#[non_exhaustive]"
      - path: kernel/mod.rs
        matches:
        - "pub struct RegionSpan {"
      - path: kernel/error.rs
        matches:
        - "#[non_exhaustive]"
      - path: tokenize/error.rs
        matches:
        - "#[non_exhaustive]"
      - path: error.rs
        matches:
        - "//! Public enums are `#[non_exhaustive]` so new variants don't break downstream matches."
        - "#[non_exhaustive]"
      - path: subengine/error.rs
        matches:
        - "#[non_exhaustive]"
      - path: manager/swap.rs
        matches:
        - //! 8-byte root pointer and epoch-gated reclamation. The kernel is never blocked or locked out.
        - "ArenaAllocator::new(ArenaKind::IndexSegment, DeviceKind::Cpu, 1 << 16)"
      - path: manager/transfer.rs
        matches:
        - "self.alloc_in(ArenaKind::Mapped, bytes, align)"
      - path: manager/test/upload.rs
        matches:
        - "let scratch = memory.alloc_in(ArenaKind::ParamBlock, 256, 8).unwrap();"
        - ".alloc_in(ArenaKind::IndexSegment, (ids.len() * ENTRY_BYTES) as u64, 256)"
        - ".alloc_in(ArenaKind::IndexSegment, SEGMENT_BYTES as u64, 256)"
        - ".alloc_in(ArenaKind::IndexSegment, bytes, 256)"
      warning: "Limit of 50 matches reached; further results may exist. Narrow the pattern, directory or include/exclude filters, or raise limit."
```
/answer 4172-e allow
Control Request:
```yaml
id: 4172-f
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src/resident/memory
    pattern: pub enum ArenaKind|pub struct Region|ArenaKind::(Mapped|ParamBlock|Staging)|Mapped
    include:
    - '*.rs'
    exclude:
    - target/**
    limit: 30
    intent: editing
  reason: ArenaKind- und Region-Definition inkl. Mapped-Klasse finden
```
/answer 4172-f allow
ReasoningToken: 43
Control Request:
```yaml
id: 4172-10
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src/resident/memory
      matches:
      - path: budget.rs
        nodes:
        - id: ArenaKind
          matches:
          - "pub enum ArenaKind {"
          - "Mapped,"
        - id: impl_ArenaKind.ccA25b|w2OG2r
          matches:
          - "ArenaKind::Staging,"
          - "ArenaKind::ParamBlock,"
          - "ArenaKind::Mapped,"
        - id: impl_ArenaSizes.get
          matches:
          - "ArenaKind::Staging => self.staging,"
          - "ArenaKind::ParamBlock => self.param_block,"
          - "ArenaKind::Mapped => self.mapped,"
      - path: region.rs
        nodes:
        - id: Region
          matches:
          - "pub struct Region {"
      - path: cuda.rs
        nodes:
        - id: MappedBuffer
          matches:
          - "struct MappedBuffer {"
        - id: impl_Send_for_MappedBuffer
          matches:
          - "unsafe impl Send for MappedBuffer {}"
        - id: impl_Sync_for_MappedBuffer
          matches:
          - "unsafe impl Sync for MappedBuffer {}"
        - id: impl_MappedBuffer
          matches:
          - "impl MappedBuffer {"
        - id: impl_Drop_for_MappedBuffer
          matches:
          - "impl Drop for MappedBuffer {"
        - id: CudaMemoryManager
          matches:
          - "mapped: Option<MappedBuffer>,"
        - id: impl_CudaMemoryManager.new
          matches:
          - "let bytes = if matches!(kind, ArenaKind::Staging | ArenaKind::Mapped) {"
          - ".then(|| MappedBuffer::new(&ctx, sizes.mapped as usize))"
        - id: impl_CudaMemoryManager.mapped_for
          matches:
          - ") -> Result<(&MappedBuffer, usize), MemoryError> {"
          - "if allocator.kind() != ArenaKind::Mapped || !allocator.is_live(region) {"
      - path: test/arena.rs
        nodes:
        - id: set_routes_regions_to_their_class
          matches:
          - "set.get(ArenaKind::Staging).alloc(1, 1),"
      - path: test/cuda.rs
        nodes:
        - id: upload_copy_download_roundtrip
          matches:
          - "let scratch = m.alloc_in(ArenaKind::ParamBlock, 256, 8).unwrap();"
      - path: staging.rs
        nodes:
        - id: impl_StagingRing_S_.acquire_wait
          matches:
          - "arena: ArenaKind::Staging,"
```
/answer 4172-10 allow
Control Request:
```yaml
id: 4172-11
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/manager/blocking.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/manager/evict.rs
  reason: "Prüfen, ob Blockierungsprädikat für Eviction bereits existiert"
```
/answer 4172-11 allow
Control Request:
```yaml
id: 4172-12
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/manager/blocking.rs
      nodes:
      - id: ModuleDoc
        signature: "//! Index reads that may page-fault or hit disk run on the blocking pool, never…"
      - id: 9d1TOl|KCxEXs
        signature: use std::ops::Range;
      - id: BlockingReads
        signature: "pub struct BlockingReads {"
      - id: impl_BlockingReads
        signature: "impl BlockingReads {"
        children:
        - id: impl_BlockingReads.new
          signature: "pub fn new(handle: Handle) -> Self {"
        - id: impl_BlockingReads.with_advice
          signature: "pub fn with_advice(mut self, sink: Arc<dyn AdviceSink>) -> Self {"
        - id: impl_BlockingReads.prefetch
          signature: "pub fn prefetch(&self, range: Range<usize>) {"
        - id: impl_BlockingReads.advise
          signature: "pub fn advise(&self, range: Range<usize>, hint: Hint) {"
          docstring: Sets `hint` on `range`. A rejected hint is logged and otherwise ignored.
        - id: impl_BlockingReads.read
          signature: "pub async fn read<T, F>(&self, range: Range<usize>, read: F) -> Result<T, JoinE…"
        - id: impl_BlockingReads.read_hinted
          signature: "pub async fn read_hinted<T, F>(     &self,     range: Range<usize>,     hint: H…"
          docstring: "Like `read` with an explicit hint, e.g. `Sequential` for dense scans."
        - id: impl_BlockingReads.run
          signature: "pub async fn run<T, F>(&self, read: F) -> Result<T, JoinError> where     T: Sen…"
          docstring: Runs `read` on the blocking pool inside the caller's span without hinting.
      - id: tests
        signature: mod tests;
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/manager/evict.rs
      nodes:
      - id: ModuleDoc
        signature: "//! Eviction for device and host mirrors: idle expiry, watermark hysteresis and…"
      - id: ogYKYu|R1fpGa
        signature: use std::cmp::Ordering as CmpOrdering;
      - id: Tier
        signature: "pub enum Tier {"
      - id: EvictReason
        signature: "pub enum EvictReason {"
      - id: Eviction
        signature: "pub struct Eviction<K> {"
      - id: Occupancy
        signature: "pub struct Occupancy {"
      - id: impl_Occupancy
        signature: "impl Occupancy {"
        children:
        - id: impl_Occupancy.from_budget
          signature: "pub fn from_budget(b: &ArenaBudget) -> Self {"
          docstring: Usable capacity excludes the reserve and the copy-on-write headroom.
        - id: impl_Occupancy.free
          signature: "fn free(&self) -> u64 {"
      - id: EvictPolicy
        signature: "pub struct EvictPolicy {"
      - id: impl_EvictPolicy
        signature: "impl EvictPolicy {"
        children:
        - id: impl_EvictPolicy.new
          signature: "pub fn new(high_watermark: f64, low_watermark: f64, idle_ttl: Duration) -> Self…"
        - id: impl_EvictPolicy.vram
          signature: "pub fn vram() -> Self {"
          docstring: "Reloads cross PCIe and spike the copy-on-write budget: long idle time, wide ban…"
        - id: impl_EvictPolicy.ram
          signature: "pub fn ram() -> Self {"
          docstring: "The OS pages host memory transparently, so a short idle time is enough."
      - id: Pin
        signature: "pub struct Pin {"
        docstring: Keeps its entry out of eviction while alive.
      - id: impl_Drop_for_Pin
        signature: "impl Drop for Pin {"
        children:
        - id: impl_Drop_for_Pin.drop
          signature: "fn drop(&mut self) {"
      - id: Item
        signature: "struct Item {"
      - id: impl_Item
        signature: "impl Item {"
        children:
        - id: impl_Item.evictable
          signature: "fn evictable(&self) -> bool {"
      - id: Evictor
        signature: "pub struct Evictor<K> {"
        docstring: "Greedy-dual-size-frequency: `priority = floor + hits * reload_cost / bytes`. Th…"
      - id: impl_Evictor_K_
        signature: "impl<K> Evictor<K> where     K: Clone + Eq + Hash + Ord + Debug, {"
        children:
        - id: impl_Evictor_K_.new
          signature: "pub fn new(policy: EvictPolicy) -> Self {"
        - id: impl_Evictor_K_.policy
          signature: "pub fn policy(&self) -> &EvictPolicy {"
        - id: impl_Evictor_K_.len
          signature: "pub fn len(&self) -> usize {"
        - id: impl_Evictor_K_.is_empty
          signature: "pub fn is_empty(&self) -> bool {"
        - id: impl_Evictor_K_.tracked_bytes
          signature: "pub fn tracked_bytes(&self) -> u64 {"
        - id: impl_Evictor_K_.contains
          signature: "pub fn contains(&self, key: &K) -> bool {"
        - id: impl_Evictor_K_.insert
          signature: "pub fn insert(     &mut self,     key: K,     bytes: u64,     tier: Tier,     r…"
          docstring: '`reload_cost` is in arbitrary but consistent units (e.g. estimated microseconds…'
        - id: impl_Evictor_K_.touch
          signature: "pub fn touch(&mut self, key: &K, now: Instant) -> bool {"
        - id: impl_Evictor_K_.pin
          signature: "pub fn pin(&self, key: &K) -> Option<Pin> {"
        - id: impl_Evictor_K_.remove
          signature: "pub fn remove(&mut self, key: &K) -> Option<u64> {"
        - id: impl_Evictor_K_.tick
          signature: "pub fn tick(&mut self, now: Instant, occupancy: Occupancy) -> Vec<Eviction<K>> {"
          docstring: Idle expiry plus watermark relief; the periodic entry point of the manager cycl…
        - id: impl_Evictor_K_.expire
          signature: "pub fn expire(&mut self, now: Instant) -> Vec<Eviction<K>> {"
        - id: impl_Evictor_K_.relieve
          signature: "pub fn relieve(&mut self, occupancy: Occupancy) -> Vec<Eviction<K>> {"
          docstring: "Hysteresis: nothing happens until `used` exceeds the high watermark; then victi…"
        - id: impl_Evictor_K_.make_room
          signature: "pub fn make_room(     &mut self,     need: u64,     occupancy: Occupancy, ) -> …"
          docstring: Frees enough victims for an allocation of `need` bytes. Evicts nothing and fail…
        - id: impl_Evictor_K_.victim_order
          signature: "fn victim_order(&self) -> Vec<K> {"
        - id: impl_Evictor_K_.evict_raising_floor
          signature: "fn evict_raising_floor(&mut self, key: K, reason: EvictReason) -> Eviction<K> {"
        - id: impl_Evictor_K_.evict
          signature: "fn evict(&mut self, key: K, reason: EvictReason) -> Eviction<K> {"
      - id: score
        signature: "fn score(item: &Item) -> f64 {"
      - id: tests
        signature: mod tests;
```
/answer 4172-12 allow
Control Request:
```yaml
id: 4172-13
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/memory/budget.rs
    ids:
    - ArenaKind
    - impl_ArenaKind.ccA25b|w2OG2r
    - impl_ArenaSizes.get
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/manager/evict.rs
    ids:
    - Pin
    - Item
    - impl_Item.evictable
    - impl_Evictor_K_.pin
  reason: ArenaKind-Varianten und bestehendes Pin/evictable-Prädikat lesen
```
/answer 4172-13 allow
ReasoningToken: 141
Control Request:
```yaml
id: 4172-14
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/memory/budget.rs
      nodes:
      - id: ArenaKind
        type: ItemEnum
        code: |
          /// Arena classes by lifetime.
          #[non_exhaustive]
          #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
          pub enum ArenaKind {
              Weights,
              IndexSegment,
              Workspace,
              Staging,
              ParamBlock,
              /// Host memory mapped into the device address space; read by the device over PCIe.
              Mapped,
          }
      - id: impl_ArenaKind.ccA25b|w2OG2r
        type: StatementGroup
        code: |2
              pub const ALL: [ArenaKind; 6] = [
                  ArenaKind::Weights,
                  ArenaKind::IndexSegment,
                  ArenaKind::Workspace,
                  ArenaKind::Staging,
                  ArenaKind::ParamBlock,
                  ArenaKind::Mapped,
              ];
      - id: impl_ArenaSizes.get
        type: ImplItemFn
        code: |2
              pub fn get(&self, kind: ArenaKind) -> u64 {
                  match kind {
                      ArenaKind::Weights => self.weights,
                      ArenaKind::IndexSegment => self.index_segment,
                      ArenaKind::Workspace => self.workspace,
                      ArenaKind::Staging => self.staging,
                      ArenaKind::ParamBlock => self.param_block,
                      ArenaKind::Mapped => self.mapped,
                  }
              }
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/manager/evict.rs
      nodes:
      - id: Pin
        type: ItemStruct
        code: |
          /// Keeps its entry out of eviction while alive.
          #[derive(Debug)]
          pub struct Pin {
              pins: Arc<AtomicU32>,
          }
      - id: Item
        type: ItemStruct
        code: |
          struct Item {
              bytes: u64,
              tier: Tier,
              reload_cost: f64,
              hits: u64,
              priority: f64,
              last_used: Instant,
              pins: Arc<AtomicU32>,
          }
      - id: impl_Item.evictable
        type: ImplItemFn
        code: |2
              fn evictable(&self) -> bool {
                  self.tier == Tier::Vector && self.pins.load(Ordering::Acquire) == 0
              }
      - id: impl_Evictor_K_.pin
        type: ImplItemFn
        code: |2
              pub fn pin(&self, key: &K) -> Option<Pin> {
                  let item = self.items.get(key)?;
                  item.pins.fetch_add(1, Ordering::AcqRel);
                  Some(Pin { pins: item.pins.clone() })
              }
```
/answer 4172-14 allow
Control Request:
```yaml
id: 4172-15
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src
    pattern: ArenaKind::\w+|match .*(class|kind|mode)\b|LaneClass::\w+ =>|LaunchMode::\w+ =>
    include:
    - '*.rs'
    exclude:
    - target/**
    - '*/test/*'
    limit: 50
    intent: editing
  reason: "Alle exhaustiven Matches auf ArenaKind/LaneClass/LaunchMode finden, die bei neuen Varianten brechen"
```
/answer 4172-15 allow
Control Request:
```yaml
id: 4172-16
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src
      matches:
      - path: resident/processor/tests.rs
        nodes:
        - id: impl_Expander_for_TestExpander.first_resource
          matches:
          - "match meta.kind {"
        - id: impl_Engine_for_TestEngine.step
          matches:
          - "match self.mode {"
      - path: resident/processor/starved.rs
        nodes:
        - id: slot
          matches:
          - "match kind {"
      - path: resident/precision/probe.rs
        nodes:
        - id: make_probe
          matches:
          - "match device.kind() {"
      - path: resident/precision/stats.rs
        nodes:
        - id: spread
          matches:
          - "match kind {"
      - path: resident/precision/store.rs
        nodes:
        - id: engine_name
          matches:
          - "match kind {"
      - path: resident/precision/measure.rs
        nodes:
        - id: make_measure
          matches:
          - "match device.kind() {"
      - path: resident/kernel/launcher.rs
        nodes:
        - id: launcher_for
          matches:
          - "match mode {"
          - "LaunchMode::Direct => Ok(Arc::new(DirectLauncher)),"
          - "LaunchMode::Capture => {"
      - path: resident/kernel/catalog.rs
        nodes:
        - id: implemented_capabilities
          matches:
          - "let dtypes: &[DType] = match kind {"
      - path: resident/kernel/mock.rs
        nodes:
        - id: impl_Kernel_for_MockKernel.run
          matches:
          - "match self.mode {"
      - path: resident/tokenize/modes.rs
        nodes:
        - id: impl_ModeScheme.prepare
          matches:
          - "let lead = match (self, mode) {"
        - id: impl_ModeScheme.finish
          matches:
          - "match mode {"
      - path: resident/subengine/exec.rs
        nodes:
        - id: impl_LaneClass.priority
          matches:
          - "LaneClass::Query => -5,"
          - "LaneClass::Copy => -1,"
          - "LaneClass::Index => 0,"
      - path: resident/manager/swap.rs
        nodes:
        - id: tests.alloc
          matches:
          - "ArenaAllocator::new(ArenaKind::IndexSegment, DeviceKind::Cpu, 1 << 16)"
      - path: resident/manager/transfer.rs
        nodes:
        - id: impl_MappedHost_for_crate__resident__memory__cuda__CudaMemoryManager.map_alloc
          matches:
          - "self.alloc_in(ArenaKind::Mapped, bytes, align)"
      - path: resident/manager/test/upload.rs
        nodes:
        - id: device.setup
          matches:
          - "let scratch = memory.alloc_in(ArenaKind::ParamBlock, 256, 8).unwrap();"
        - id: device.gather_reaches_the_device_in_one_copy
          matches:
          - ".alloc_in(ArenaKind::IndexSegment, (ids.len() * ENTRY_BYTES) as u64, 256)"
        - id: device.gather_versus_segment_crossover
          matches:
          - ".alloc_in(ArenaKind::IndexSegment, SEGMENT_BYTES as u64, 256)"
          - ".alloc_in(ArenaKind::IndexSegment, bytes, 256)"
      - path: resident/manager/test/transfer.rs
        nodes:
        - id: fixture
          matches:
          - "ArenaAllocator::new(ArenaKind::IndexSegment, DeviceKind::Cpu, ARENA_BYTES),"
        - id: impl_HostMapped.new
          matches:
          - "alloc: ArenaAllocator::new(ArenaKind::Mapped, DeviceKind::Cpu, bytes),"
        - id: zero_copy_writes_in_place_without_staging_or_upload
          matches:
          - "assert_eq!(loaded[0].region.arena, ArenaKind::Mapped.id());"
        - id: zero_copy_falls_back_when_the_mapped_arena_is_full
          matches:
          - "assert_ne!(loaded[0].region.arena, ArenaKind::Mapped.id());"
        - id: flat_transfer
          matches:
          - "ArenaAllocator::new(ArenaKind::IndexSegment, DeviceKind::Cpu, ARENA_BYTES),"
        - id: device.impl_RegionSource_for_MemRegions.alloc_segment
          matches:
          - "Ok(self.0.alloc_in(ArenaKind::IndexSegment, bytes, align)?)"
        - id: device.transfer_reaches_the_device_and_reads_back
          matches:
          - "let scratch = memory.alloc_in(ArenaKind::ParamBlock, 256, 8).unwrap();"
        - id: device.bulk_segment_reaches_the_device_in_one_direct_copy
          matches:
          - "let scratch = memory.alloc_in(ArenaKind::ParamBlock, 256, 8).unwrap();"
        - id: device.zero_copy_segment_is_readable_through_the_device_address
          matches:
          - "let scratch = memory.alloc_in(ArenaKind::ParamBlock, 256, 8).unwrap();"
          - "assert_eq!(seg.region.arena, ArenaKind::Mapped.id());"
      - path: resident/manager/test/arena.rs
        nodes:
        - id: host_arena
          matches:
          - "ArenaAllocator::new(ArenaKind::IndexSegment, DeviceKind::Cpu, cap)"
      - path: resident/manager/test/build.rs
        nodes:
        - id: host_arena
          matches:
          - "ArenaAllocator::new(ArenaKind::IndexSegment, DeviceKind::Cpu, cap)"
      - path: resident/manager/test/runtime.rs
        nodes:
        - id: rig
          matches:
          - "ArenaAllocator::new(ArenaKind::IndexSegment, DeviceKind::Cpu, arena_bytes),"
      - path: resident/model/test/mod.rs
        nodes:
        - id: fixture
          matches:
          - "let scratch = mem.alloc_in(ArenaKind::Workspace, 1024, 16).unwrap();"
        - id: put_tokens
          matches:
          - ".alloc_in(ArenaKind::Staging, layout.byte_len() as u64, 16)"
        - id: out_ref
          matches:
          - ".alloc_in(ArenaKind::Workspace, layout.byte_len() as u64, 16)"
      - path: resident/model/test/candle_adapter.rs
        nodes:
        - id: embed_with
          matches:
          - "let scratch = mem.alloc_in(ArenaKind::Workspace, 1024, 16).unwrap();"
          - ".alloc_in(ArenaKind::Staging, t_layout.byte_len() as u64, 16)"
          - ".alloc_in(ArenaKind::Workspace, o_layout.byte_len() as u64, 16)"
      - path: resident/model/candle_adapter.rs
        nodes:
        - id: device_for
          matches:
          - "match kind {"
      - path: resident/memory/cpu.rs
        nodes:
        - id: impl_CpuMemoryManager.new
          matches:
          - "let storage = ArenaKind::ALL.iter().map(|k| Mutex::new(vec![0u8; sizes.get(*k) as usize])).collect();"
        - id: impl_DeviceMemory_for_CpuMemoryManager.budget
          matches:
          - let free = ArenaKind::ALL.iter().map(|k| self.set.get(*k).available()).sum();
          - let total = ArenaKind::ALL.iter().map(|k| sizes.get(*k)).sum();
      - path: resident/memory/budget.rs
        nodes:
        - id: impl_ArenaKind.ccA25b|w2OG2r
          matches:
          - "ArenaKind::Weights,"
          - "ArenaKind::IndexSegment,"
          - "ArenaKind::Workspace,"
          - "ArenaKind::Staging,"
      warning: "Limit of 50 matches reached; further results may exist. Narrow the pattern, directory or include/exclude filters, or raise limit."
```
/answer 4172-16 allow
ReasoningToken: 505
Control Request:
```yaml
id: 4172-17
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/docs
    - /home/user/xyan/xy.ai.workbench/project
    pattern: ADR|Ringbuffer
    include:
    - '*.md'
    exclude:
    - plan_*
    - context_*
    limit: 20
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel/test
    pattern: "launcher_for|^use |^fn |#\\[test\\]"
    include:
    - launcher.rs
    limit: 30
    intent: editing
  reason: ADR-Ablageort und bestehende Launcher-Tests finden
```
/answer 4172-17 allow
Control Request:
```yaml
id: 4172-18
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/docs
      - /home/user/xyan/xy.ai.workbench/project
      matches:
      - path: architecture/scope_charta.md
        matches:
        - "| §3 | Ring-buffer producer/consumer compute | **A** | `resident.md` Ausblick; `resident_index.md` \"Ringbuffer-Modus (geplant)\"; `subengine` §4.6 | Engine (deferred, see §4 Divergence) |"
      - path: architecture/gpu_memory.md
        matches:
        - '- `CudaMemoryManager::new` rejects a plan that exceeds `free - headroom` (`DEFAULT_HEADROOM` = 512 MiB) before allocating.'
      - path: resident/resident.md
        matches:
        - "Es sind für (GPU/CUDA) Modi geplant über den ein Kernel mittels Ringbuffer, permanent Embedding und Ähnlichkeitssuche betreibt, die Ergebnisse wiederum in Ringbuffer schreibt, wo sie ausgelesen werden können."
        - "### Ausblick: GPU-Ringbuffer-Modus"
      - path: resident/resident_index.md
        matches:
        - "- Persistenter Ringbuffer-Modus: Kernel liest den Pointer einmal pro Work-Item mit Acquire-Load (`ld.acquire`, sm70+, Turing reicht). Der Inhalt der neuen Tabelle muss vorher sichtbar sein (Event oder Fence)."
        - "## Ringbuffer-Modus (persistent, geplant)"
      - path: resident/resident_planplan.md
        matches:
        - "- Zu klären: Welche Paper-Aspekte sind *jetzt* umzusetzen, welche nur Ausblick (Ringbuffer, gleitende Indizierung)."
        - "**H1 · GPU-Ringbuffer-Modus** — SPSC-Ring (`w`/`r`, CAS nur für Exit), getrennte Ringe Embedding/MaxSim, persistenter Kernel (`idle_spin`/`max_run_time`), blockierte Segmente. Quellen: `resident.md` (Ausblick), `resident_index.md` (Ringbuffer-Modus), `resident_subengine.md` §4.6."
        - "1. **Mechanismus-Reinterpretation:** Das zentrale Nebenläufigkeitsmodell der These (SPSC-Ringbuffer, §3) wird in Phase 1 durch ein **Pull-basiertes Lock-free-Task-Queue-Modell + Processor/Planner** ersetzt. Residency bleibt das Ziel, der Ringbuffer wird zur *späteren* Spezialisierung (Persistent-Kernel-Modus)."
        - '| §3 | **Ringbuffer-Producer/Consumer-Compute** | **A** | `resident.md` Ausblick; `index` „Ringbuffer-Modus (geplant)"; `subengine` §4.6 | **In Phase 1 ersetzt durch Pull-Task-Queue** |'
        - "**Plan:** Die zentrale Antwort ist stattdessen die **Lock-free-Pull-Queue + Processor/Planner/Subengine** (`resident.md` Queue-Logik, `resident_processor.md`, `resident_planner.md`). Der Ringbuffer bleibt als **optionaler Persistent-Kernel-Modus für den Scorer** erhalten (`subengine` §4.6: „Sinnvoll für den Scorer, nicht für den Encoder\")."
        - "- **H1 · GPU-Ringbuffer-Modus** ← These §3 (**A**): die eigentliche „zentrale Antwort\" der These; in Phase 1 durch Pull-Queue ersetzt. Planungsauftrag: nur Erweiterungspunkte sichern (SPSC-Ring, getrennte Ringe Embedding/MaxSim, Acquire-Load-Pointer in E4)."
        - "5. **Ringbuffer-Reinterpretation (H1)** — als Umsetzungsentscheidung dokumentieren, damit die These-Treue nachvollziehbar ist."
      - path: resident/resident_processor.md
        matches:
        - "- GPU-Ringbuffer-Modus, gleitende Indizierung, Graph-Capture."
      - path: resident/archive/create_plan_a.md
        matches:
        - "\"Ausblick\" (gleitende Indizierung, Ringbuffer)."
        - der SPSC-Ringbuffer (These §3) ist auf den Persistent-Kernel-Modus **zurückgestellt** (H1).
        - 4. **Zentrale Divergenz dokumentieren** (These §3 Ringbuffer vs. Phase-1-Pull-Queue) als
        - 3. Divergenz Ringbuffer↔Pull-Queue ist als Entscheidung (nicht als Inkonsistenz) dokumentiert.
      - path: resident/archive/create_plan_c.md
        matches:
        - 4. Ringbuffer-/Graph-Capture-Modi bleiben Ausblick (Gruppe H); C2 sichert nur Erweiterungspunkte.
        - "10. **Erweiterungspunkte sichern (Gruppe H):** `requires()`-Affinität, Ringbuffer-/Graph-Capture-Modus nur als nicht verbaute Hooks — keine Detailimplementierung."
      warning: "Limit of 20 matches reached; further results may exist. Narrow the pattern, directory or include/exclude filters, or raise limit."
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel/test
      matches:
      - path: launcher.rs
        nodes:
        - id: dzxSoZ|G4l0Mz
          matches:
          - use super::*;
          - use crate::resident::memory::ArenaId;
          - "use crate::resident::subengine::{LaneConfig, RecordingLane};"
          - "use std::sync::atomic::{AtomicBool, Ordering};"
        - id: region
          matches:
          - "fn region(gen: u32) -> Region {"
        - id: key
          matches:
          - "fn key(gen: u32) -> LaunchKey {"
        - id: direct_runs_job_on_lane
          matches:
          - "#[test]"
          - "fn direct_runs_job_on_lane() {"
          - let launcher = launcher_for(LaunchMode::Direct).unwrap();
        - id: capture_is_unavailable_until_enabled
          matches:
          - "#[test]"
          - "fn capture_is_unavailable_until_enabled() {"
          - "matches!(launcher_for(LaunchMode::Capture), Err(KernelError::Unavailable { .."
        - id: generation_change_changes_key
          matches:
          - "#[test]"
          - "fn generation_change_changes_key() {"
```
/answer 4172-18 allow
Thinking:
`ArenaKind`, der Evictor und `SwapManager::begin_batch` sind bereits abgedeckt; offen bleiben `LaneClass`, `LaunchMode`, ein No-GPU-Test und der ADR. Als Nächstes prüfe ich die Divergenz-Stelle in der Scope-Charta.


ReasoningToken: 245
Control Request:
```yaml
id: 4172-19
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/docs/architecture
    pattern: ^#+ |Divergence
    include:
    - scope_charta.md
    limit: 30
    intent: editing
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src/resident/subengine/test
    pattern: "^use |^fn |#\\[test\\]"
    include:
    - exec.rs
    limit: 12
    intent: editing
  reason: Struktur der Scope-Charta für den ADR-Eintrag und Imports der Exec-Tests prüfen
```
/answer 4172-19 allow
Control Request:
```yaml
id: 4172-1a
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/docs/architecture
      matches:
      - path: scope_charta.md
        nodes:
        - id: 35617d
          matches:
          - "# Scope Charta: Resident Engine vs. Search Layer"
        - id: 35617d.63f846
          matches:
          - "## 1. Purpose"
        - id: 35617d.dc0d0a
          matches:
          - "## 2. Boundary Table"
          - "| §3 | Ring-buffer producer/consumer compute | **A** | `resident.md` Ausblick; `resident_index.md` \"Ringbuffer-Modus (geplant)\"; `subengine` §4.6 | Engine (deferred, see §4 Divergence) |"
        - id: 35617d.a8ffa9
          matches:
          - "## 3. Engine Delivers"
          - "## 4. Outside the Engine (Search Layer)"
        - id: 35617d.445827
          matches:
          - "## 5. Central Divergence: Ring Buffer vs. Pull Queue (§3)"
        - id: 35617d.43bb35
          matches:
          - "## 6. Hook Obligations (Phase 1 must not foreclose)"
        - id: 35617d.acfc01
          matches:
          - "## 7. References"
          - "## 8. Acceptance Check"
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src/resident/subengine/test
      matches:
      - path: exec.rs
        nodes:
        - id: XiB8kc|S6qVjz
          matches:
          - use super::*;
          - use crate::resident::memory::ArenaId;
          - "use std::sync::atomic::{AtomicUsize, Ordering};"
          - use std::time::Duration;
        - id: cfg
          matches:
          - "fn cfg() -> LaneConfig {"
        - id: lane
          matches:
          - "fn lane(class: LaneClass) -> CpuLane {"
        - id: jobs_run_in_submission_order
          matches:
          - "#[test]"
          - "fn jobs_run_in_submission_order() {"
        - id: event_completes_after_prior_jobs
          matches:
          - "#[test]"
          - "fn event_completes_after_prior_jobs() {"
        - id: wait_orders_work_across_lanes
          matches:
          - "#[test]"
          - "fn wait_orders_work_across_lanes() {"
      warning: "Limit of 12 matches reached; further results may exist. Narrow the pattern, directory or include/exclude filters, or raise limit."
```
/answer 4172-1a allow
ReasoningToken: 38
Control Request:
```yaml
id: 4172-1b
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/docs/architecture/scope_charta.md
    ids:
    - 35617d.445827
    - 35617d.43bb35
  reason: "Prüfen, ob die Divergenz-Entscheidung und Hook-Pflichten bereits dokumentiert sind"
```
/answer 4172-1b allow
Control Request:
```yaml
id: 4172-1c
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/docs/architecture/scope_charta.md
      nodes:
      - id: 35617d.445827
        type: section
        signature: "5. Central Divergence: Ring Buffer vs. Pull Queue (§3)"
        children:
        - id: 35617d.445827.emkl3d|V8l4pX
          type: paragraph
          code: |
            **Thesis §3** states its central answer to concurrency is "not classical multithreading but a
            producer–consumer architecture over ring buffers."
        - id: 35617d.445827.4HJURi|1eL5SU
          type: paragraph
          code: |
            **This plan's answer instead:** a **lock-free pull queue + Processor/Planner/Subengine**
            (`resident.md` queue logic, `resident_processor.md`, `resident_planner.md`). The ring buffer is
            retained as an **optional persistent-kernel mode for the scorer only** (`subengine` §4.6:
            "useful for the scorer, not for the encoder").
        - id: 35617d.445827.mMzU9x|ZjCFCG
          type: paragraph
          code: |
            The goal (residency, no per-request setup overhead) is identical to the thesis; the path for
            Phase 1 is deliberately different. **This is a documented implementation decision, not an
            inconsistency** — it must not be read as a 1:1 implementation of thesis §3.
      - id: 35617d.43bb35
        type: section
        signature: 6. Hook Obligations (Phase 1 must not foreclose)
        children:
        - id: 35617d.43bb35.xvWuPW|OdPap5
          type: paragraph
          code: |
            Phase 1 implementation must keep the following extension points open, each tied to a later
            outlook position (H1–H6):
```
/answer 4172-1c allow
Text:
 Die Divergenz-Entscheidung steht schon in Scope-Charta §5. Jetzt die Code-Änderungen.
ReasoningToken: 95
Control Request:
```yaml
id: 4172-1d
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/subengine/exec.rs
    id: LaneClass
    old_text: |-
      #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
      pub enum LaneClass {
    new_text: |-
      #[non_exhaustive]
      #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
      pub enum LaneClass {
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel/launcher.rs
    id: LaunchMode
    old_text: |-
      pub enum LaunchMode {
          Direct,
          Capture,
      }
    new_text: |-
      pub enum LaunchMode {
          Direct,
          Capture,
          /// Resident kernel fed through a host-mapped ring.
          Persistent,
      }
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel/launcher.rs
    id: LaunchMode
    old_text: |-
      #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
      pub enum LaunchMode {
    new_text: |-
      #[non_exhaustive]
      #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
      pub enum LaunchMode {
  reason: LaneClass und LaunchMode offen halten und Persistent-Launch-Modus als Variante vorsehen
```
/answer 4172-1d allow
Control Request:
```yaml
id: 4172-1e
toolName: ast_edit_marks
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel/launcher.rs
    id: launcher_for
    start_marker: "LaunchMode::Capture => {"
    end_marker: |2-
              }
          }
      }
    content: |-
      LaunchMode::Capture => Err(KernelError::Unavailable {
                  reason: "graph capture is not enabled".into(),
              }),
              LaunchMode::Persistent => Err(KernelError::Unavailable {
                  reason: "persistent kernel mode is not enabled".into(),
              }),
          }
      }
  reason: launcher_for um den Persistent-Zweig erweitern
```
/answer 4172-1e allow
Control Request:
```yaml
id: 4172-1f
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel/test/launcher.rs
    ids:
    - capture_is_unavailable_until_enabled
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/subengine/test/exec.rs
    ids:
    - lane
  reason: Bestehenden Test-Stil für neue No-GPU-Tests übernehmen
```
/answer 4172-1f allow
Control Request:
```yaml
id: 4172-20
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel/test/launcher.rs
      nodes:
      - id: capture_is_unavailable_until_enabled
        type: ItemFn
        code: |
          #[test]
          fn capture_is_unavailable_until_enabled() {
              assert!(
                  matches!(launcher_for(LaunchMode::Capture), Err(KernelError::Unavailable { ..
                  }))
              );
          }
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/subengine/test/exec.rs
      nodes:
      - id: lane
        type: ItemFn
        code: |
          fn lane(class: LaneClass) -> CpuLane {
              CpuLane::new(class, cfg()).unwrap()
          }
```
/answer 4172-20 allow
ReasoningToken: 179
Control Request:
```yaml
id: 4172-21
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel/launcher.rs
    ids:
    - impl_Launcher_for_DirectLauncher
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel/test/launcher.rs
    ids:
    - direct_runs_job_on_lane
    - key
  reason: Launch-Signatur und Fehlerkonvertierung für den Mock-Launcher im Test übernehmen
```
/answer 4172-21 allow
Control Request:
```yaml
id: 4172-22
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel/launcher.rs
      nodes:
      - id: impl_Launcher_for_DirectLauncher
        type: ItemImpl
        signature: "impl Launcher for DirectLauncher {"
        children:
        - id: impl_Launcher_for_DirectLauncher.mode
          type: ImplItemFn
          code: |2
                fn mode(&self) -> LaunchMode {
                    LaunchMode::Direct
                }
        - id: impl_Launcher_for_DirectLauncher.launch
          type: ImplItemFn
          code: |2
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
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel/test/launcher.rs
      nodes:
      - id: direct_runs_job_on_lane
        type: ItemFn
        code: |
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
      - id: key
        type: ItemFn
        code: |
          fn key(gen: u32) -> LaunchKey {
              LaunchKey {
                  op: "maxsim",
                  shape_bucket: 128,
                  regions: vec![region(gen)],
                  dtype: DType::F32,
                  lane: LaneClass::Query,
              }
          }
```
/answer 4172-22 allow
Control Request:
```yaml
id: 4172-23
toolName: ast_insert
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel/test/launcher.rs
    id: capture_is_unavailable_until_enabled
    position: after
    source: |
      #[test]
      fn persistent_is_unavailable_until_enabled() {
          assert!(matches!(
              launcher_for(LaunchMode::Persistent),
              Err(KernelError::Unavailable { .. })
          ));
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
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/subengine/test/exec.rs
    id: lane
    position: after
    source: |
      #[test]
      fn lane_class_is_open_for_long_lived_variants() {
          fn priority_or_default(class: LaneClass) -> i32 {
              match class {
                  LaneClass::Query | LaneClass::Index | LaneClass::Copy => class.priority(),
                  #[allow(unreachable_patterns)]
                  _ => 0,
              }
          }
          assert_eq!(priority_or_default(LaneClass::Query), LaneClass::Query.priority());
      }
  reason: "No-GPU-Tests: Launcher-Austausch hinter dem Trait und offene Lane-Klasse"
```
/answer 4172-23 allow
ReasoningToken: 78
Control Request:
```yaml
id: 4172-24
toolName: python
arguments:
  script: |-
    import subprocess
    r=subprocess.run("cd /home/user/xyan/xy.ai.workbench/rag && cargo test --lib resident::kernel::test::launcher resident::subengine::test::exec 2>&1 | tail -40",shell=True,capture_output=True,text=True)
    print(r.stdout,r.stderr)
  reason: Kompilieren und die betroffenen No-GPU-Tests ausführen
```
/answer 4172-24 allow
Control Request:
```yaml
id: 4172-25
result:
  structuredContent:
    exit_code: 0
    stdout: |+
      error: unexpected argument 'resident::subengine::test::exec' found

      Usage: cargo test [OPTIONS] [TESTNAME] [-- [ARGS]...]

      For more information, try '--help'.
```
/answer 4172-25 allow
Control Request:
```yaml
id: 4172-26
toolName: python
arguments:
  script: |-
    import subprocess
    r=subprocess.run("cd /home/user/xyan/xy.ai.workbench/rag && cargo test --lib -- launcher exec 2>&1 | tail -40",shell=True,capture_output=True,text=True)
    print(r.stdout,r.stderr)
  reason: Tests mit Namensfiltern erneut ausführen
```
/answer 4172-26 allow
Control Request:
```yaml
id: 4172-27
result:
  structuredContent:
    exit_code: 0
    stdout: |2+
          = note: this pattern will always match, so the `if let` is useless
          = help: consider replacing the `if let` with a `let`
          = note: `#[warn(irrefutable_let_patterns)]` on by default

      warning: associated function `from_raw` is never used
        --> src/resident/manager/seqlock.rs:20:26
         |
      12 | impl<'a> Seqlock<'a> {
         | -------------------- associated function in this implementation
      ...
      20 |     pub(crate) unsafe fn from_raw(seq: *mut u64, words: *mut u64, len: usize) -> Self {
         |                          ^^^^^^^^
         |
         = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

      warning: `xy-ai-rag` (lib test) generated 2 warnings
          Finished `test` profile [unoptimized + debuginfo] target(s) in 10.33s
           Running unittests src/lib.rs (.bin/debug/deps/xy_ai_rag-d5754016cabf875c)

      running 17 tests
      test resident::kernel::launcher::tests::capture_is_unavailable_until_enabled ... ok
      test resident::kernel::launcher::tests::persistent_is_unavailable_until_enabled ... ok
      test resident::kernel::launcher::tests::generation_change_changes_key ... ok
      test resident::kernel::launcher::tests::direct_runs_job_on_lane ... ok
      test resident::kernel::launcher::tests::persistent_launcher_plugs_in_behind_the_trait ... ok
      test resident::kernel::slice::tests::exec_error_propagates ... ok
      test resident::subengine::exec::tests::lane_class_is_open_for_long_lived_variants ... ok
      test resident::subengine::exec::tests::drop_drains_pending_jobs ... ok
      test resident::subengine::exec::tests::first_failure_is_reported_once ... ok
      test resident::subengine::exec::tests::priorities_rank_query_above_copy_above_index ... ok
      test resident::subengine::exec::tests::param_block_aligns_and_bounds ... ok
      test resident::subengine::exec::tests::lanes_work_as_trait_objects ... ok
      test resident::subengine::exec::tests::recording_lane_logs_calls ... ok
      test resident::subengine::exec::tests::jobs_run_in_submission_order ... ok
      test resident::subengine::exec::tests::scratch_respects_alignment_bounds_and_reset ... ok
      test resident::subengine::exec::tests::event_completes_after_prior_jobs ... ok
      test resident::subengine::exec::tests::wait_orders_work_across_lanes ... ok

      test result: ok. 17 passed; 0 failed; 0 ignored; 0 measured; 598 filtered out; finished in 0.03s
```
/answer 4172-27 allow
Result Stats: id=d0c17d75-e168-4de5-bd14-54a1ced08751, total: 65570, in: 30, out: 8845, reason: 0, read: 662902, write: 56695, start: 1793047557745, end: 1793047566155, time: 8290, total: 95807, io: 0, control: 120, sent: 0, recv: 211
H1 ist umgesetzt: Die offenen Stellen für den späteren Ringbuffer-Modus sind gesichert, ohne einen Ring oder Kernel zu bauen. Die 17 Tests mit den Filtern `launcher` und `exec` laufen grün, ohne GPU. Die übrige Suite und den `cuda`-Feature-Build habe ich nicht gelaufen.

**Neu angepasst**
- **Lane-Klasse:** `LaneClass` ist jetzt `#[non_exhaustive]`, sodass später eine langlebige Persistent-Lane ohne API-Bruch dazukommen kann.
- **Launcher:** `LaunchMode` ist ebenfalls `#[non_exhaustive]` und hat die neue Variante `Persistent`. `launcher_for(Persistent)` liefert vorerst `KernelError::Unavailable`, genau wie `Capture`. Ein Ring-Launcher kann später hinter dem bestehenden `Launcher`-Trait eintreten.
- **Tests:**
  - `persistent_is_unavailable_until_enabled` prüft den `Unavailable`-Fehler.
  - `persistent_launcher_plugs_in_behind_the_trait` ist ein Mock-Launcher im Persistent-Modus hinter `Arc<dyn Launcher>`.
  - `lane_class_is_open_for_long_lived_variants` ist ein recht schwacher Test: Er prüft nur, dass sich `LaneClass` mit Default-Zweig matchen lässt.

**Schon vorhanden, nichts geändert**
- **Ring-State-Region:** `ArenaKind` ist bereits `#[non_exhaustive]` und hat `Mapped`. Eine langlebige Ring-Region lässt sich dort ablegen, ohne die Signatur von `Region` anzufassen.
- **Pointer-Lesezeitpunkt:** `SwapManager` hat sowohl `snapshot()` (Pointer als Launch-Argument) als auch `begin_batch()` (ein Acquire-Load pro Batch). Beide Modi sind getestet.
- **Gesperrte Segmente:** Der Evictor schließt über `Pin` gepinnte Einträge von der Eviction aus. Ein Ring-Request kann ein Segment also darüber sperren.
- **ADR:** Die Entscheidung "Ringbuffer in Phase 1 durch Pull-Queue ersetzt" steht in `docs/architecture/scope_charta.md` §5. Einen separaten Eintrag habe ich nicht angelegt.

**Bewusst nicht umgesetzt**
Kein persistenter Kernel, kein SPSC-Ring, kein `__threadfence_system`/`ld.acquire.sys`-Code, keine Cooperative-Launch-Konfiguration und keine Zeitscheiben-Logik.

Ich habe Punkt 6 aus §3 (getrennte Ergebnispfade für Embedding und MaxSim) nicht gesondert geprüft und kein Pinning speziell für Ring-Requests gebaut.
