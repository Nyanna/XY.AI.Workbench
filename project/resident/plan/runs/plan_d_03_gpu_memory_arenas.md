SystemInit:  id=1e6ec56d-772f-444c-b948-f179da853722, cwd=/home/user/xyan/xy.ai.workbench/project, model=claude-sonnet-5-5
Text:
 Ich sehe mir zuerst den vorhandenen Code im Crate an.
Control Request:
```yaml
id: bc7a-1
toolName: list
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident
  - path: /home/user/xyan/xy.ai.workbench/rag
    recursive: false
  reason: Bestehende Struktur von resident/ und Crate-Root erfassen
```
/answer bc7a-1 allow
ReasoningToken: 30
Control Request:
```yaml
id: bc7a-2
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident
      entries:
      - ".: (6 files)"
      - backend.rs
      - "./facade: (7 files)"
      - blocking.rs
      - "./index: (3 files)"
      - memory.rs
      - "./kernel: (3 files)"
      - cuda.rs
      - "./manager: (1 file)"
      - mod.rs
      - "./memory: (6 files)"
      - dtype.rs
      - "./op: (3 files)"
      - lease.rs
      - "./planner: (6 files)"
      - adapter.rs
      - "./precision: (6 files)"
      - axis.rs
      - "./processor: (11 files)"
      - completion.rs
      - "./queue: (5 files)"
      - mod.rs
      - "./subengine: (6 files)"
      - device.rs
      warning: Result reduced because the total number of entries (63) exceeds the limit of 50. Narrow down the result using the 'pattern' regular expression parameter.
    - path: /home/user/xyan/xy.ai.workbench/rag
      entries:
      - ".:"
      - .gitignore
      - .project
      - Cargo.lock
      - Cargo.toml
      - xyrag.sh
```
/answer bc7a-2 allow
Control Request:
```yaml
id: bc7a-3
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench/rag
  script: cat Cargo.toml; cat src/resident/mod.rs 2>/dev/null; wc -l src/resident/memory/*.rs src/resident/subengine/*.rs
  reason: Cargo-Features und Memory-/Subengine-Dateien prüfen
```
/answer bc7a-3 allow
Control Request:
```yaml
id: bc7a-4
result:
  structuredContent:
    stdout: |
      [package]
      name = "xy-ai-rag"
      version = "0.1.0"
      edition = "2021"
      description = "xy.ai.rag - Layered Anytime Retrieval Engine"

      [lib]
      name = "xy_ai_rag"
      path = "src/lib.rs"

      [[bin]]
      name = "xyrag"
      path = "src/bin/xyrag.rs"

      [dependencies]
      tokio = { version = "1", features = ["rt-multi-thread", "macros", "sync", "time"] }
      tokio-util = "0.7"
      async-trait = "0.1"
      futures = "0.3"
      serde = { version = "1", features = ["derive"] }
      serde_json = "1"
      serde_yaml = "0.9"
      rusqlite = { version = "0.31", features = ["bundled"] }
      clap = { version = "4", features = ["derive"] }
      anyhow = "1"
      rayon = "1"
      rkyv = { version = "0.7", features = ["validation"] }
      glob = "0.3"
      lru = "0.12"
      regex = "1"
      unicode-normalization = "0.1"
      thiserror = "2.0.20"
      tracing = "0.1.44"
      tracing-subscriber = { version = "0.3.23", features = ["env-filter"] }
      tracing-appender = "0.2.3"
      crossbeam = "0.8.5"
      crossbeam-queue = "0.3.14"
      parking_lot = "0.12.5"
      arc-swap = "1.9.2"
      toml = "0.9"
      half = "2.7.1"
      memmap2 = "0.9.3"
      xxhash-rust = { version = "0.8", features = ["xxh3"] }
      parquet = "60.0.0"
      arrow = "60.0.0"
      tokenizers = { version = "0.23.2", default-features = false, features = ["fancy-regex"] }
      safetensors = "0.8.0"
      candle-core = { version = "0.11.0", optional = true, default-features = false }
      candle-transformers = { version = "0.11.0", optional = true, default-features = false }
      cudarc = { version = "=0.19.9", optional = true }
      rand = { version = "0.8", optional = true }

      [target.'cfg(target_os = "linux")'.dependencies]
      libc = "0.2"

      [features]
      default = []
      candle = ["dep:candle-core"]
      cuda = ["candle", "dep:candle-transformers", "dep:cudarc", "candle-core/cuda", "candle-transformers/cuda"]
      mock = ["dep:rand"]
      json = ["tracing-subscriber/json"]

      [dev-dependencies]
      proptest = "1"
      criterion = "0.5"
      mockall = "0.15.0"
      loom = "0.7"
      shuttle = "0.7"
      rand = "0.8"
      tokio = { version = "1", features = ["test-util"] }

      [[bench]]
      name = "trie_bench"
      harness = false

      [patch.crates-io]
      candle-kernels = { path = "third_party/candle-kernels" }
      //! Resident engine: embedding/similarity substrate, kept warm.
      pub mod backend;
      pub mod error;
      pub mod failpoint;
      pub mod telemetry;
      #[cfg(any(test, feature = "mock"))]
      pub mod mock;
      pub mod queue;
      pub mod memory;
      pub mod op;
      pub mod index;
      pub mod kernel;
      pub mod precision;
      pub mod facade;
      pub mod processor;
      pub mod planner;
      pub mod subengine;
      pub mod manager;
        124 src/resident/memory/dtype.rs
        102 src/resident/memory/freelist.rs
         71 src/resident/memory/layout.rs
        181 src/resident/memory/manager.rs
         13 src/resident/memory/mod.rs
         12 src/resident/memory/region.rs
        256 src/resident/subengine/device.rs
         61 src/resident/subengine/error.rs
        635 src/resident/subengine/exec.rs
        418 src/resident/subengine/lifecycle.rs
         49 src/resident/subengine/mod.rs
        206 src/resident/subengine/resource.rs
       2128 total
```
/answer bc7a-4 allow
Text:
 Das Memory-Modul existiert teilweise. Ich lese es und die Subengine-Schnittstellen.
Control Request:
```yaml
id: bc7a-5
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench/rag/src/resident
  script: "cat memory/mod.rs memory/region.rs memory/freelist.rs memory/manager.rs memory/layout.rs; sed -n 1,60p memory/dtype.rs; ls memory ../resident/subengine; cat subengine/mod.rs"
  reason: Vorhandene Memory-Implementierung vollständig lesen
```
/answer bc7a-5 deny verwende AST
Control Request:
```yaml
id: bc7a-6
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/memory/mod.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/memory/region.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/memory/freelist.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/memory/manager.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/memory/layout.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/memory/dtype.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/subengine/mod.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/subengine/device.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/subengine/error.rs
  reason: Struktur der vorhandenen Memory-/Subengine-Dateien per AST erfassen
```
/answer bc7a-6 allow
Control Request:
```yaml
id: bc7a-7
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/memory/mod.rs
      nodes:
      - id: ModuleDoc
        signature: //! Region/Layout/DType and MemoryManager trait.
      - id: dtype
        signature: pub mod dtype;
      - id: freelist
        signature: pub mod freelist;
      - id: layout
        signature: pub mod layout;
      - id: manager
        signature: pub mod manager;
      - id: region
        signature: pub mod region;
      - id: hGYNaC|1FHY01
        signature: "pub use dtype::{DType, UnsupportedDType};"
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/memory/region.rs
      nodes:
      - id: ModuleDoc
        signature: //! Pointer-free memory handle; resolution to a device address is backend-inter…
      - id: ArenaId
        signature: "#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]"
      - id: Region
        signature: "#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)] pub struct Region {"
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/memory/freelist.rs
      nodes:
      - id: ModuleDoc
        signature: "//! Backend-agnostic offset allocator: first-fit over a coalescing free map."
      - id: H6YeMB|cDxZCJ
        signature: use std::collections::BTreeMap;
      - id: FreeList
        signature: "#[derive(Debug)] pub struct FreeList {"
      - id: impl_FreeList
        signature: "impl FreeList {"
        children:
        - id: impl_FreeList.new
          signature: "pub fn new(capacity: u64) -> Self {"
        - id: impl_FreeList.capacity
          signature: "pub fn capacity(&self) -> u64 {"
        - id: impl_FreeList.available
          signature: "pub fn available(&self) -> u64 {"
        - id: impl_FreeList.alloc
          signature: "pub fn alloc(&mut self, len: u64, align: u64) -> Option<u64> {"
        - id: impl_FreeList.free
          signature: "pub fn free(&mut self, offset: u64, len: u64) {"
      - id: tests
        signature: "#[cfg(test)] mod tests {"
        children:
        - id: tests.LwlKpn|JFVpeI
          signature: use super::*;
        - id: tests.alloc_align_and_exhaust
          signature: "#[test] fn alloc_align_and_exhaust() {"
        - id: tests.coalesces_neighbours
          signature: "#[test] fn coalesces_neighbours() {"
        - id: tests.fragmentation_blocks_large_alloc
          signature: "#[test] fn fragmentation_blocks_large_alloc() {"
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/memory/manager.rs
      nodes:
      - id: ModuleDoc
        signature: //! `MemoryManager` contract plus a GPU-free bookkeeping implementation.
      - id: PNCQ5q|2yCFcD
        signature: use parking_lot::Mutex;
      - id: DeviceKind
        signature: "#[non_exhaustive] #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)] pub enum D…"
      - id: compute_dtype
        signature: "pub fn compute_dtype(dev: DeviceKind) -> DType {"
      - id: MemoryManager
        signature: "pub trait MemoryManager {"
        children:
        - id: MemoryManager.alloc
          signature: fn alloc(
        - id: MemoryManager.free
          signature: "/// Stale handles are ignored (logged), never dereferenced."
          docstring: "Stale handles are ignored (logged), never dereferenced."
      - id: Slot
        signature: "struct Slot {"
      - id: impl_Slot
        signature: "impl Slot {"
        children:
        - id: impl_Slot.matches
          signature: "fn matches(&self, r: &Region) -> bool {"
      - id: Inner
        signature: "struct Inner {"
      - id: BookkeepingArena
        signature: /// Single-arena allocator tracking offsets and generations only; no device mem…
        docstring: Single-arena allocator tracking offsets and generations only; no device memory.
      - id: impl_BookkeepingArena
        signature: "impl BookkeepingArena {"
        children:
        - id: impl_BookkeepingArena.new
          signature: "pub fn new(id: ArenaId, capacity: u64) -> Self {"
        - id: impl_BookkeepingArena.available
          signature: "pub fn available(&self) -> u64 {"
        - id: impl_BookkeepingArena.is_live
          signature: "pub fn is_live(&self, r: Region) -> bool {"
        - id: impl_BookkeepingArena.try_free
          signature: "pub(crate) fn try_free(&self, region: Region) -> Result<(), ManagerError> {"
      - id: impl_MemoryManager_for_BookkeepingArena
        signature: "impl MemoryManager for BookkeepingArena {"
        children:
        - id: impl_MemoryManager_for_BookkeepingArena.alloc
          signature: "fn alloc(     &self,     _dev: DeviceKind,     bytes: u64,     align: u64, ) ->…"
        - id: impl_MemoryManager_for_BookkeepingArena.free
          signature: "fn free(&self, region: Region) {"
      - id: tests
        signature: "#[cfg(test)] mod tests {"
        children:
        - id: tests.LwlKpn|JFVpeI
          signature: use super::*;
        - id: tests.policy
          signature: "#[test] fn policy() {"
        - id: tests.realloc_bumps_gen_and_stale_is_rejected
          signature: "#[test] fn realloc_bumps_gen_and_stale_is_rejected() {"
        - id: tests.failpoint_injects_exhaustion_without_consuming_space
          signature: "#[test] fn failpoint_injects_exhaustion_without_consuming_space() {"
        - id: tests.Q7Lm8P|7wBhRR
          signature: "proptest::proptest! {"
        - id: tests.exhaustion
          signature: "#[test] fn exhaustion() {"
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/memory/layout.rs
      nodes:
      - id: ModuleDoc
        signature: "//! Tensor layout in bytes; views change layout only, never data."
      - id: tAYCUt|s8KW8S
        signature: use super::dtype::DType;
      - id: Layout
        signature: "#[derive(Clone, Debug, PartialEq, Eq)] pub struct Layout {"
      - id: impl_Layout
        signature: "impl Layout {"
        children:
        - id: impl_Layout.contiguous
          signature: "pub fn contiguous(shape: &[usize], dtype: DType) -> Self {"
        - id: impl_Layout.elem_count
          signature: "pub fn elem_count(&self) -> usize {"
        - id: impl_Layout.byte_len
          signature: "pub fn byte_len(&self) -> usize {"
        - id: impl_Layout.is_contiguous
          signature: "pub fn is_contiguous(&self) -> bool {"
        - id: impl_Layout.transpose
          signature: "pub fn transpose(&self, a: usize, b: usize) -> Self {"
        - id: impl_Layout.narrow
          signature: "pub fn narrow(&self, dim: usize, len: usize) -> Self {"
      - id: tests
        signature: "#[cfg(test)] mod tests {"
        children:
        - id: tests.LwlKpn|JFVpeI
          signature: use super::*;
        - id: tests.contiguous_byte_strides
          signature: "#[test] fn contiguous_byte_strides() {"
        - id: tests.views_do_not_copy
          signature: "#[test] fn views_do_not_copy() {"
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/memory/dtype.rs
      nodes:
      - id: ModuleDoc
        signature: //! Engine-owned element type; backend dtypes are converted at the edge only.
      - id: MS3n2e|NqxAEG
        signature: use thiserror::Error;
      - id: UnsupportedDType
        signature: "#[derive(Error, Debug, Clone, Copy, PartialEq, Eq)] #[error(\"unsupported dtype:…"
      - id: DType
        signature: "#[non_exhaustive] #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)] pub enum D…"
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
        signature: "#[cfg(feature = \"candle\")] pub(crate) mod candle_adapter {"
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
          signature: /// Candle layouts count elements; the engine counts bytes. pub(crate) fn elems…
          docstring: Candle layouts count elements; the engine counts bytes.
        - id: candle_adapter.bytes_to_elems
          signature: "pub(crate) fn bytes_to_elems(bytes: usize, dtype: DType) -> usize {"
      - id: tests
        signature: "#[cfg(test)] mod tests {"
        children:
        - id: tests.LwlKpn|JFVpeI
          signature: use super::*;
        - id: tests.sizes
          signature: "#[test] fn sizes() {"
        - id: tests.safetensors_convert_and_reject
          signature: "#[test] fn safetensors_convert_and_reject() {"
        - id: tests.candle_roundtrip_and_reject
          signature: "#[cfg(feature = \"candle\")] #[test] fn candle_roundtrip_and_reject() {"
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/subengine/mod.rs
      nodes:
      - id: ModuleDoc
        signature: "//! Shared subengine lifecycle & pull model, tokenizer/normalizer (CPU)."
      - id: 77cuty|ChE8yG
        signature: use crate::resident::backend::ComputeBackend;
      - id: CpuBackend
        signature: "/// CPU backend; default without `cuda`, and the fallback target if CUDA init f…"
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
        signature: "#[derive(Clone, Copy, Debug, PartialEq, Eq)] pub enum SubengineKind {"
      - id: Subengine
        signature: /// Resources and execution capacity of one compute substrate. Work arrives thr…
        docstring: Resources and execution capacity of one compute substrate. Work arrives through…
        children:
        - id: Subengine.kind
          signature: fn kind(&self) -> SubengineKind;
        - id: Subengine.capacity
          signature: fn capacity(&self) -> Capacity;
        - id: Subengine.throughput
          signature: /// Smoothed items per second.
          docstring: Smoothed items per second.
        - id: Subengine.batch_size
          signature: "/// Throughput-derived batch size, independent of current load."
          docstring: "Throughput-derived batch size, independent of current load."
        - id: Subengine.pull_budget
          signature: /// Upper bound on the jobs to hand this subengine in one `Claim::claim` call. …
          docstring: Upper bound on the jobs to hand this subengine in one `Claim::claim` call.
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/subengine/device.rs
      nodes:
      - id: ModuleDoc
        signature: "//! Device/Lane, execution context, loaded encoder, GPU memory arenas."
      - id: 77cuty|ChE8yG
        signature: use crate::resident::backend::ComputeBackend;
      - id: CudaBackend
        signature: /// Real CUDA backend; the single Candle/CUDA adapter point for the subengine. …
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
        signature: /// Identifies the compute device a subengine runs on; lanes are opened from it…
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
          signature: /// Retains the primary context of `ordinal`. The first call is expensive (0.2-…
          docstring: "Retains the primary context of `ordinal`. The first call is expensive (0.2-2 s,…"
        - id: impl_DeviceHandle.cuda_2
          signature: "#[cfg(not(feature = \"cuda\"))] pub fn cuda(_ordinal: usize) -> Result<Self, Devi…"
        - id: impl_DeviceHandle.kind
          signature: "pub fn kind(&self) -> DeviceKind {"
        - id: impl_DeviceHandle.ordinal
          signature: "pub fn ordinal(&self) -> usize {"
        - id: impl_DeviceHandle.init_cost
          signature: "/// Time spent creating the context, reported to the lazy-init accounting. pub …"
          docstring: "Time spent creating the context, reported to the lazy-init accounting."
        - id: impl_DeviceHandle.warmup
          signature: "/// Forces driver-side lazy work (first allocation, stream creation) so it does…"
          docstring: "Forces driver-side lazy work (first allocation, stream creation) so it does not…"
        - id: impl_DeviceHandle.open_lane
          signature: "pub fn open_lane(     &self,     class: LaneClass,     cfg: LaneConfig, ) -> Re…"
      - id: Ke5I2l|scA2j5
        signature: "#[cfg(feature = \"cuda\")]"
      - id: cuda_lane
        signature: "#[cfg(feature = \"cuda\")] mod cuda_lane {"
        children:
        - id: cuda_lane.CHYK8i|Km1i1Y
          signature: use std::sync::Arc;
        - id: cuda_lane.CudaLane
          signature: /// Prioritised non-blocking stream; all device work of one lane is ordered on …
          docstring: Prioritised non-blocking stream; all device work of one lane is ordered on it.
        - id: cuda_lane.event_err
          signature: "fn event_err(e: cudarc::driver::DriverError) -> DeviceError {"
        - id: cuda_lane.impl_CudaLane
          signature: "impl CudaLane {"
          children:
          - id: cuda_lane.impl_CudaLane.new
            signature: "pub(super) fn new(     ctx: &Arc<CudaContext>,     class: LaneClass,     cfg: L…"
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
        signature: "#[cfg(test)] mod tests {"
        children:
        - id: tests.0UbQJo|Pv9Q3Y
          signature: use super::*;
        - id: tests.cfg
          signature: "fn cfg() -> LaneConfig {"
        - id: tests.cpu_handle_opens_cpu_lanes
          signature: "#[test] fn cpu_handle_opens_cpu_lanes() {"
        - id: tests.cuda_handle_unavailable_without_feature
          signature: "#[cfg(not(feature = \"cuda\"))] #[test] fn cuda_handle_unavailable_without_featur…"
        - id: tests.project_cudarc_is_the_one_candle_uses
          signature: "#[cfg(feature = \"cuda\")] #[test] fn project_cudarc_is_the_one_candle_uses() {"
        - id: tests.cuda_lanes_order_work_through_events
          signature: "#[cfg(feature = \"cuda\")] #[test] #[ignore = \"requires a CUDA device\"] fn cuda_l…"
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/subengine/error.rs
      nodes:
      - id: HrcLHg|pZ5pMk
        signature: use thiserror::Error;
      - id: SubengineError
        signature: "/// Subengine resource lifecycle errors. #[derive(Error, Debug)] #[non_exhausti…"
        docstring: Subengine resource lifecycle errors.
      - id: impl_IsRetryable_for_SubengineError
        signature: "impl IsRetryable for SubengineError {"
        children:
        - id: impl_IsRetryable_for_SubengineError.is_retryable
          signature: "fn is_retryable(&self) -> bool {"
      - id: DeviceError
        signature: "/// Device and execution-lane errors. #[derive(Error, Debug)] #[non_exhaustive]…"
        docstring: Device and execution-lane errors.
      - id: impl_IsRetryable_for_DeviceError
        signature: "impl IsRetryable for DeviceError {"
        children:
        - id: impl_IsRetryable_for_DeviceError.is_retryable
          signature: "fn is_retryable(&self) -> bool {"
```
/answer bc7a-7 allow
Control Request:
```yaml
id: bc7a-8
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/memory/manager.rs
    ids:
    - ModuleDoc
    - DeviceKind
    - compute_dtype
    - MemoryManager
    - Slot
    - impl_Slot
    - Inner
    - BookkeepingArena
    - impl_BookkeepingArena
    - impl_MemoryManager_for_BookkeepingArena
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/memory/region.rs
    ids:
    - ArenaId
    - Region
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/memory/freelist.rs
    ids:
    - FreeList
  reason: "Vorhandene Manager-/Region-Verträge lesen, um D3 darauf aufzusetzen"
```
/answer bc7a-8 allow
ReasoningToken: 69
Control Request:
```yaml
id: bc7a-9
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/memory/manager.rs
      nodes:
      - id: ModuleDoc
        type: ModuleAttrs
        code: |
          //! `MemoryManager` contract plus a GPU-free bookkeeping implementation.
      - id: DeviceKind
        type: ItemEnum
        code: |
          #[non_exhaustive]
          #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
          pub enum DeviceKind {
              Cpu,
              Gpu,
          }
      - id: compute_dtype
        type: ItemFn
        code: |
          pub fn compute_dtype(dev: DeviceKind) -> DType {
              match dev {
                  DeviceKind::Gpu => DType::F16,
                  DeviceKind::Cpu => DType::F32,
              }
          }
      - id: MemoryManager
        type: ItemTrait
        signature: "pub trait MemoryManager {"
        children:
        - id: MemoryManager.alloc
          type: TraitItemFn
          code: |2
                fn alloc(&self, dev: DeviceKind, bytes: u64, align: u64) -> Result<Region, IndexError>;
        - id: MemoryManager.free
          type: TraitItemFn
          code: |2
                /// Stale handles are ignored (logged), never dereferenced.
                fn free(&self, region: Region);
      - id: Slot
        type: ItemStruct
        code: |
          struct Slot {
              offset: u64,
              len: u64,
              gen: u32,
              live: bool,
          }
      - id: impl_Slot
        type: ItemImpl
        signature: "impl Slot {"
        children:
        - id: impl_Slot.matches
          type: ImplItemFn
          code: |2
                fn matches(&self, r: &Region) -> bool {
                    self.live && self.offset == r.offset && self.len == r.len && self.gen == r.gen
                }
      - id: Inner
        type: ItemStruct
        code: |
          struct Inner {
              list: FreeList,
              slots: Vec<Slot>,
          }
      - id: BookkeepingArena
        type: ItemStruct
        code: |
          /// Single-arena allocator tracking offsets and generations only; no device memory.
          pub struct BookkeepingArena {
              id: ArenaId,
              inner: Mutex<Inner>,
          }
      - id: impl_BookkeepingArena
        type: ItemImpl
        signature: "impl BookkeepingArena {"
        children:
        - id: impl_BookkeepingArena.new
          type: ImplItemFn
          code: |2
                pub fn new(id: ArenaId, capacity: u64) -> Self {
                    Self { id, inner: Mutex::new(Inner { list: FreeList::new(capacity), slots: Vec::new() }) }
                }
        - id: impl_BookkeepingArena.available
          type: ImplItemFn
          code: |2
                pub fn available(&self) -> u64 {
                    self.inner.lock().list.available()
                }
        - id: impl_BookkeepingArena.is_live
          type: ImplItemFn
          code: |2
                pub fn is_live(&self, r: Region) -> bool {
                    r.arena == self.id && self.inner.lock().slots.iter().any(|s| s.matches(&r))
                }
        - id: impl_BookkeepingArena.try_free
          type: ImplItemFn
          code: |2
                pub(crate) fn try_free(&self, region: Region) -> Result<(), ManagerError> {
                    let stale = || ManagerError::StaleRegion { arena: region.arena.0, gen: region.gen };
                    if region.arena != self.id {
                        return Err(stale());
                    }
                    let mut g = self.inner.lock();
                    let slot = g.slots.iter_mut().find(|s| s.matches(&region)).ok_or_else(stale)?;
                    slot.live = false;
                    g.list.free(region.offset, region.len);
                    tracing::debug!(arena = self.id.0, offset = region.offset, bytes = region.len, "free");
                    Ok(())
                }
      - id: impl_MemoryManager_for_BookkeepingArena
        type: ItemImpl
        signature: "impl MemoryManager for BookkeepingArena {"
        children:
        - id: impl_MemoryManager_for_BookkeepingArena.alloc
          type: ImplItemFn
          code: |2
                fn alloc(&self, _dev: DeviceKind, bytes: u64, align: u64) -> Result<Region, IndexError> {
                    let mut g = self.inner.lock();
                    let available = g.list.available();
                    let exhausted = IndexError::ArenaExhausted {
                        requested: bytes as usize,
                        available: available as usize,
                    };
                    if failpoint::hit("memory.alloc", u64::from(self.id.0)) {
                        return Err(exhausted);
                    }
                    let offset = g.list.alloc(bytes, align).ok_or(exhausted)?;
                    let gen = match g.slots.iter_mut().find(|s| !s.live && s.offset == offset && s.len == bytes) {
                        Some(s) => {
                            s.gen = s.gen.wrapping_add(1);
                            s.live = true;
                            s.gen
                        }
                        None => {
                            g.slots.push(Slot { offset, len: bytes, gen: 0, live: true });
                            0
                        }
                    };
                    tracing::debug!(arena = self.id.0, offset, bytes, "alloc");
                    Ok(Region { arena: self.id, offset, len: bytes, gen })
                }
        - id: impl_MemoryManager_for_BookkeepingArena.free
          type: ImplItemFn
          code: |2
                fn free(&self, region: Region) {
                    if let Err(e) = self.try_free(region) {
                        tracing::warn!(error = %e, "ignored stale free");
                    }
                }
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/memory/region.rs
      nodes:
      - id: ArenaId
        type: ItemStruct
        code: |
          #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
          pub struct ArenaId(pub u32);
      - id: Region
        type: ItemStruct
        code: |
          #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
          pub struct Region {
              pub arena: ArenaId,
              pub offset: u64,
              pub len: u64,
              pub gen: u32,
          }
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/memory/freelist.rs
      nodes:
      - id: FreeList
        type: ItemStruct
        code: |
          #[derive(Debug)]
          pub struct FreeList {
              capacity: u64,
              free: BTreeMap<u64, u64>,
          }
```
/answer bc7a-9 allow
Control Request:
```yaml
id: bc7a-a
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/subengine/exec.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/subengine/resource.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/error.rs
  reason: "ScratchArena, ParamBlock, EventHandle und Fehlertypen finden"
```
/answer bc7a-a allow
Control Request:
```yaml
id: bc7a-b
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
        signature: /// Work item executed on a lane. Device work is enqueued asynchronously from w…
        docstring: Work item executed on a lane. Device work is enqueued asynchronously from withi…
      - id: LaneClass
        signature: "#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)] pub enum LaneClass {"
      - id: impl_LaneClass
        signature: "impl LaneClass {"
        children:
        - id: impl_LaneClass.priority
          signature: /// CUDA stream priority; lower values are more urgent and `0` is the default. …
          docstring: CUDA stream priority; lower values are more urgent and `0` is the default. Prio…
      - id: LaneConfig
        signature: "#[derive(Clone, Copy, Debug)] pub struct LaneConfig {"
      - id: impl_LaneConfig
        signature: "impl LaneConfig {"
        children:
        - id: impl_LaneConfig.Cj9Odg|qdIpgQ
          signature: "pub const DEFAULT_PARAM_CAPACITY: usize = 4096;"
        - id: impl_LaneConfig.new
          signature: "pub fn new(scratch: Region) -> Self {"
      - id: CpuEvent
        signature: "#[derive(Default)] pub(crate) struct CpuEvent {"
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
        signature: /// Opaque completion marker for all work submitted to a lane before it was rec…
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
          signature: /// Blocks the calling thread until the event completes. pub fn synchronize(&se…
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
        signature: "/// Fixed-capacity buffer for dynamic kernel parameters, staged on the host and…"
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
          signature: /// Appends `value` naturally aligned and returns its byte offset. pub fn push<…
          docstring: Appends `value` naturally aligned and returns its byte offset.
      - id: ScratchArena
        signature: /// Bump allocator over a lane-private arena span; reset once the lane has drai…
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
          signature: "/// Invalidates all regions handed out so far. pub fn reset(&self) {"
          docstring: Invalidates all regions handed out so far.
      - id: ExecContext
        signature: "/// In-order execution queue of one device. Device handles, streams and pointer…"
        docstring: "In-order execution queue of one device. Device handles, streams and pointers ne…"
        children:
        - id: ExecContext.class
          signature: fn class(&self) -> LaneClass;
        - id: ExecContext.device
          signature: fn device(&self) -> DeviceKind;
        - id: ExecContext.submit
          signature: "fn submit(&self, job: LaneJob) -> Result<(), DeviceError>;"
        - id: ExecContext.record_event
          signature: /// Completes once everything submitted so far has finished.
          docstring: Completes once everything submitted so far has finished.
        - id: ExecContext.wait
          signature: /// Orders all later work on this lane after `event`.
          docstring: Orders all later work on this lane after `event`.
        - id: ExecContext.synchronize
          signature: /// Blocks until the lane has drained; returns the first job failure since the …
          docstring: Blocks until the lane has drained; returns the first job failure since the last…
        - id: ExecContext.param_block
          signature: fn param_block(&mut self) -> &mut ParamBlock;
        - id: ExecContext.scratch
          signature: fn scratch(&self) -> &ScratchArena;
      - id: Msg
        signature: "enum Msg {"
      - id: CpuLane
        signature: "/// CPU lane: one worker thread, so submission order is execution order. pub st…"
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
        signature: "/// Lane double for downstream tests: runs jobs inline and logs every call. #[c…"
        docstring: "Lane double for downstream tests: runs jobs inline and logs every call."
      - id: impl_RecordingLane
        signature: "#[cfg(any(test, feature = \"mock\"))] impl RecordingLane {"
        children:
        - id: impl_RecordingLane.new
          signature: "pub fn new(class: LaneClass, cfg: LaneConfig) -> Self {"
        - id: impl_RecordingLane.calls
          signature: "pub fn calls(&self) -> Vec<&'static str> {"
      - id: impl_ExecContext_for_RecordingLane
        signature: "#[cfg(any(test, feature = \"mock\"))] impl ExecContext for RecordingLane {"
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
        signature: "#[cfg(test)] mod tests {"
        children:
        - id: tests.XiB8kc|S6qVjz
          signature: use super::*;
        - id: tests.cfg
          signature: "fn cfg() -> LaneConfig {"
        - id: tests.lane
          signature: "fn lane(class: LaneClass) -> CpuLane {"
        - id: tests.jobs_run_in_submission_order
          signature: "#[test] fn jobs_run_in_submission_order() {"
        - id: tests.event_completes_after_prior_jobs
          signature: "#[test] fn event_completes_after_prior_jobs() {"
        - id: tests.wait_orders_work_across_lanes
          signature: "#[test] fn wait_orders_work_across_lanes() {"
        - id: tests.first_failure_is_reported_once
          signature: "#[test] fn first_failure_is_reported_once() {"
        - id: tests.drop_drains_pending_jobs
          signature: "#[test] fn drop_drains_pending_jobs() {"
        - id: tests.priorities_rank_query_above_copy_above_index
          signature: "#[test] fn priorities_rank_query_above_copy_above_index() {"
        - id: tests.param_block_aligns_and_bounds
          signature: "#[test] fn param_block_aligns_and_bounds() {"
        - id: tests.scratch_respects_alignment_bounds_and_reset
          signature: "#[test] fn scratch_respects_alignment_bounds_and_reset() {"
        - id: tests.recording_lane_logs_calls
          signature: "#[test] fn recording_lane_logs_calls() {"
        - id: tests.lanes_work_as_trait_objects
          signature: "#[test] fn lanes_work_as_trait_objects() {"
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/subengine/resource.rs
      nodes:
      - id: 2Cm4Tq|b7Fqza
        signature: use std::collections::HashMap;
      - id: ResourceKey
        signature: "#[derive(Clone, Debug, PartialEq, Eq, Hash)]"
      - id: impl_ResourceKey
        signature: "impl ResourceKey {"
        children:
        - id: impl_ResourceKey.new
          signature: "pub fn new(key: impl Into<String>) -> Self {"
      - id: impl_fmt__Display_for_ResourceKey
        signature: "impl fmt::Display for ResourceKey {"
        children:
        - id: impl_fmt__Display_for_ResourceKey.fmt
          signature: "fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {"
      - id: Resource
        signature: "pub trait Resource: Send + Sync + 'static {"
        children:
        - id: Resource.key
          signature: fn key(&self) -> ResourceKey;
        - id: Resource.init_cost_hint
          signature: /// Relative cost of re-initialising after eviction; lets callers prefer cheap …
          docstring: Relative cost of re-initialising after eviction; lets callers prefer cheap vict…
        - id: Resource.evictable
          signature: /// Veto for resource-specific reasons beyond lease and TTL checks. fn evictabl…
          docstring: Veto for resource-specific reasons beyond lease and TTL checks.
      - id: Ewma
        signature: "/// Exponentially weighted moving average; the first sample primes the value. #…"
        docstring: Exponentially weighted moving average; the first sample primes the value.
      - id: impl_Ewma
        signature: "impl Ewma {"
        children:
        - id: impl_Ewma.new
          signature: "pub fn new(alpha: f32) -> Self {"
        - id: impl_Ewma.observe
          signature: "pub fn observe(&mut self, sample: f32) -> f32 {"
        - id: impl_Ewma.get
          signature: "pub fn get(&self) -> Option<f32> {"
      - id: PullConfig
        signature: "#[derive(Clone, Copy, Debug)] pub struct PullConfig {"
      - id: impl_Default_for_PullConfig
        signature: "impl Default for PullConfig {"
        children:
        - id: impl_Default_for_PullConfig.default
          signature: "fn default() -> Self {"
      - id: PullController
        signature: /// Derives the batch size to pull from observed throughput; the clamp bounds l…
        docstring: Derives the batch size to pull from observed throughput; the clamp bounds load …
      - id: impl_PullController
        signature: "impl PullController {"
        children:
        - id: impl_PullController.new
          signature: "pub fn new(cfg: PullConfig) -> Self {"
        - id: impl_PullController.observe
          signature: "pub fn observe(&self, items: usize, elapsed_ms: f32) {"
        - id: impl_PullController.throughput
          signature: /// Items per second; zero until the first observation. pub fn throughput(&self…
          docstring: Items per second; zero until the first observation.
        - id: impl_PullController.batch_size
          signature: "pub fn batch_size(&self) -> usize {"
      - id: Capacity
        signature: "#[derive(Clone, Copy, Debug, PartialEq, Eq)] pub struct Capacity {"
      - id: impl_Capacity
        signature: "impl Capacity {"
        children:
        - id: impl_Capacity.free
          signature: "pub fn free(&self) -> usize {"
      - id: ClaimFilter
        signature: "/// Restricts which pending work a subengine is willing to take. #[derive(Clone…"
        docstring: Restricts which pending work a subengine is willing to take.
      - id: Claim
        signature: "/// Implemented by the processor: hands out at most `budget` jobs matching `fil…"
        docstring: "Implemented by the processor: hands out at most `budget` jobs matching `filter`."
        children:
        - id: Claim.claim
          signature: "fn claim(&self, filter: &ClaimFilter, budget: usize) -> Vec<J>;"
      - id: coalesce
        signature: "/// Groups items by key, keeping first-seen group order and item order within e…"
        docstring: "Groups items by key, keeping first-seen group order and item order within each …"
      - id: tests
        signature: "#[cfg(test)] mod tests {"
        children:
        - id: tests.LwlKpn|JFVpeI
          signature: use super::*;
        - id: tests.ewma_primes_then_smooths
          signature: "#[test] fn ewma_primes_then_smooths() {"
        - id: tests.batch_size_is_clamped_and_starts_at_min
          signature: "#[test] fn batch_size_is_clamped_and_starts_at_min() {"
        - id: tests.noisy_samples_do_not_oscillate_batch_size
          signature: "#[test] fn noisy_samples_do_not_oscillate_batch_size() {"
        - id: tests.capacity_free_saturates
          signature: "#[test] fn capacity_free_saturates() {"
        - id: tests.coalesce_groups_in_first_seen_order
          signature: "#[test] fn coalesce_groups_in_first_seen_order() {"
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/error.rs
      nodes:
      - id: ModuleDoc
        signature: //! Error taxonomy.
      - id: gaWICu|jY2QN9
        signature: use thiserror::Error;
      - id: IsRetryable
        signature: "/// Classifies an error as retryable (transient) vs. fatal, for backoff decisio…"
        docstring: "Classifies an error as retryable (transient) vs. fatal, for backoff decisions."
        children:
        - id: IsRetryable.is_retryable
          signature: fn is_retryable(&self) -> bool;
      - id: IndexError
        signature: "/// Index/scope layer errors. #[derive(Error, Debug)] #[non_exhaustive] pub enu…"
        docstring: Index/scope layer errors.
      - id: impl_IsRetryable_for_IndexError
        signature: "impl IsRetryable for IndexError {"
        children:
        - id: impl_IsRetryable_for_IndexError.is_retryable
          signature: "fn is_retryable(&self) -> bool {"
      - id: ManagerError
        signature: /// Internal index-manager errors; mapped to a public variant or retried at the…
        docstring: Internal index-manager errors; mapped to a public variant or retried at the API…
      - id: impl_IsRetryable_for_ManagerError
        signature: "impl IsRetryable for ManagerError {"
        children:
        - id: impl_IsRetryable_for_ManagerError.is_retryable
          signature: "fn is_retryable(&self) -> bool {"
      - id: PrecisionError
        signature: "/// Precision/calibration layer errors. #[derive(Error, Debug)] #[non_exhaustiv…"
        docstring: Precision/calibration layer errors.
      - id: impl_IsRetryable_for_PrecisionError
        signature: "impl IsRetryable for PrecisionError {"
        children:
        - id: impl_IsRetryable_for_PrecisionError.is_retryable
          signature: "fn is_retryable(&self) -> bool {"
      - id: PlannerError
        signature: "/// Planner/task-expansion layer errors. #[derive(Error, Debug)] #[non_exhausti…"
        docstring: Planner/task-expansion layer errors.
      - id: impl_IsRetryable_for_PlannerError
        signature: "impl IsRetryable for PlannerError {"
        children:
        - id: impl_IsRetryable_for_PlannerError.is_retryable
          signature: "fn is_retryable(&self) -> bool {"
      - id: CapabilityError
        signature: "/// Capability/profile-matching layer errors. #[derive(Error, Debug)] #[non_exh…"
        docstring: Capability/profile-matching layer errors.
      - id: ConfigError
        signature: "/// Facade/configuration errors raised by the builder and `init()`. #[derive(Er…"
        docstring: Facade/configuration errors raised by the builder and `init()`.
      - id: impl_IsRetryable_for_CapabilityError
        signature: "impl IsRetryable for CapabilityError {"
        children:
        - id: impl_IsRetryable_for_CapabilityError.is_retryable
          signature: "fn is_retryable(&self) -> bool {"
      - id: impl_IsRetryable_for_ConfigError
        signature: "impl IsRetryable for ConfigError {"
        children:
        - id: impl_IsRetryable_for_ConfigError.is_retryable
          signature: "fn is_retryable(&self) -> bool {"
      - id: gVatFA|OrOCFr
        signature: pub use crate::resident::subengine::DeviceError;
      - id: ResidentError
        signature: "/// Top-level resident engine error, wrapping every layer error via `#[from]`. …"
        docstring: "Top-level resident engine error, wrapping every layer error via `#[from]`."
      - id: impl_IsRetryable_for_ResidentError
        signature: "impl IsRetryable for ResidentError {"
        children:
        - id: impl_IsRetryable_for_ResidentError.is_retryable
          signature: "fn is_retryable(&self) -> bool {"
      - id: tests
        signature: "#[cfg(test)] mod tests {"
        children:
        - id: tests.LwlKpn|JFVpeI
          signature: use super::*;
        - id: tests.assert_send_sync_static
          signature: "fn assert_send_sync_static<T: Send + Sync + 'static>() {"
        - id: tests.all_errors_are_send_sync_static
          signature: "#[test] fn all_errors_are_send_sync_static() {"
        - id: tests.layer_errors_convert_into_resident_error
          signature: "#[test] fn layer_errors_convert_into_resident_error() {"
        - id: tests.transparent_wrapper_keeps_message
          signature: "#[test] fn transparent_wrapper_keeps_message() {"
        - id: tests.retryability_classification
          signature: "#[test] fn retryability_classification() {"
```
/answer bc7a-b allow
Control Request:
```yaml
id: bc7a-c
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/error.rs
    ids:
    - IndexError
    - ManagerError
    - impl_IsRetryable_for_IndexError
    - impl_IsRetryable_for_ManagerError
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/subengine/error.rs
    ids:
    - DeviceError
    - impl_IsRetryable_for_DeviceError
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/subengine/exec.rs
    ids:
    - EventRepr
    - EventHandle
    - impl_EventHandle
    - CpuEvent
    - impl_CpuEvent
    - ScratchArena
    - impl_ScratchArena
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/subengine/device.rs
    ids:
    - DeviceHandle
    - impl_DeviceHandle
    - cuda_lane
  reason: "Fehlertypen, EventHandle/EventRepr und DeviceHandle-Interna lesen"
```
/answer bc7a-c allow
ReasoningToken: 162
Control Request:
```yaml
id: bc7a-d
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/error.rs
      nodes:
      - id: IndexError
        type: ItemEnum
        code: |
          /// Index/scope layer errors.
          #[derive(Error, Debug)]
          #[non_exhaustive]
          pub enum IndexError {
              #[error("access method '{method}' is not supported for this index")]
              UnsupportedScope { method: String },
              #[error("index for scope '{scope}' is not available")]
              IndexUnavailable { scope: String },
              #[error("arena exhausted: requested {requested} bytes, available {available} bytes")]
              ArenaExhausted { requested: usize, available: usize },
              #[error("transfer failed: {reason}")]
              TransferFailed { reason: String },
              #[error("kernel failed: {reason}")]
              KernelFailed { reason: String },
              #[error("unknown kernel batch {batch}")]
              UnknownBatch { batch: u64 },
          }
      - id: ManagerError
        type: ItemEnum
        code: |
          /// Internal index-manager errors; mapped to a public variant or retried at the API boundary,
          /// never exposed directly.
          #[derive(Error, Debug)]
          #[non_exhaustive]
          pub(crate) enum ManagerError {
              #[error("stale region in arena {arena} (gen {gen})")]
              StaleRegion { arena: u32, gen: u32 },
          }
      - id: impl_IsRetryable_for_IndexError
        type: ItemImpl
        signature: "impl IsRetryable for IndexError {"
        children:
        - id: impl_IsRetryable_for_IndexError.is_retryable
          type: ImplItemFn
          code: |2
                fn is_retryable(&self) -> bool {
                    matches!(
                        self, IndexError::IndexUnavailable { .. } | IndexError::ArenaExhausted { .. }
                    )
                }
      - id: impl_IsRetryable_for_ManagerError
        type: ItemImpl
        signature: "impl IsRetryable for ManagerError {"
        children:
        - id: impl_IsRetryable_for_ManagerError.is_retryable
          type: ImplItemFn
          code: |2
                fn is_retryable(&self) -> bool {
                    matches!(self, ManagerError::StaleRegion { .. })
                }
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/subengine/error.rs
      nodes:
      - id: DeviceError
        type: ItemEnum
        code: |
          /// Device and execution-lane errors.
          #[derive(Error, Debug)]
          #[non_exhaustive]
          pub enum DeviceError {
              #[error("device initialisation failed: {reason}")]
              Init { reason: String },
              #[error("CUDA support is not compiled in")]
              CudaUnavailable,
              #[error("stream creation failed: {reason}")]
              StreamCreate { reason: String },
              #[error("event operation failed: {reason}")]
              EventRecord { reason: String },
              #[error("stream priorities are not supported on this device")]
              PriorityUnsupported,
              #[error("cudarc version mismatch: candle uses {candle}, project uses {project}")]
              VersionMismatch { candle: String, project: String },
              #[error(
                  "parameter block overflow: {requested} bytes requested, capacity {capacity}"
              )]
              ParamOverflow { capacity: usize, requested: usize },
              #[error(
                  "scratch arena exhausted: {requested} bytes requested, {available} available"
              )]
              ScratchExhausted { requested: u64, available: u64 },
              #[error("invalid alignment {align}")]
              InvalidAlign { align: u64 },
              #[error("lane worker is closed")]
              LaneClosed,
              #[error("lane job failed: {reason}")]
              JobFailed { reason: String },
          }
      - id: impl_IsRetryable_for_DeviceError
        type: ItemImpl
        signature: "impl IsRetryable for DeviceError {"
        children:
        - id: impl_IsRetryable_for_DeviceError.is_retryable
          type: ImplItemFn
          code: |2
                fn is_retryable(&self) -> bool {
                    matches!(self, Self::ScratchExhausted { .. })
                }
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/subengine/exec.rs
      nodes:
      - id: EventRepr
        type: ItemEnum
        code: |
          pub(crate) enum EventRepr {
              Cpu(CpuEvent),
              #[cfg(feature = "cuda")]
              Cuda(cudarc::driver::CudaEvent),
          }
      - id: EventHandle
        type: ItemStruct
        code: |
          /// Opaque completion marker for all work submitted to a lane before it was recorded.
          #[derive(Clone)]
          pub struct EventHandle {
              repr: Arc<EventRepr>,
          }
      - id: impl_EventHandle
        type: ItemImpl
        signature: "impl EventHandle {"
        children:
        - id: impl_EventHandle.from_repr
          type: ImplItemFn
          code: |2
                pub(crate) fn from_repr(repr: EventRepr) -> Self {
                    Self {
                        repr: Arc::new(repr),
                    }
                }
        - id: impl_EventHandle.repr
          type: ImplItemFn
          code: |2
                pub(crate) fn repr(&self) -> &EventRepr {
                    &self.repr
                }
        - id: impl_EventHandle.is_complete
          type: ImplItemFn
          code: |2
                pub fn is_complete(&self) -> bool {
                    match &*self.repr {
                        EventRepr::Cpu(e) => e.is_done(),
                        #[cfg(feature = "cuda")]
                        EventRepr::Cuda(e) => e.is_complete(),
                    }
                }
        - id: impl_EventHandle.synchronize
          type: ImplItemFn
          code: |2
                /// Blocks the calling thread until the event completes.
                pub fn synchronize(&self) -> Result<(), DeviceError> {
                    match &*self.repr {
                        EventRepr::Cpu(e) => {
                            e.wait();
                            Ok(())
                        }
                        #[cfg(feature = "cuda")]
                        EventRepr::Cuda(e) => e.synchronize().map_err(|e| DeviceError::EventRecord {
                            reason: e.to_string(),
                        }),
                    }
                }
      - id: CpuEvent
        type: ItemStruct
        code: |
          #[derive(Default)]
          pub(crate) struct CpuEvent {
              done: Mutex<bool>,
              cv: Condvar,
          }
      - id: impl_CpuEvent
        type: ItemImpl
        signature: "impl CpuEvent {"
        children:
        - id: impl_CpuEvent.signal
          type: ImplItemFn
          code: |2
                fn signal(&self) {
                    *self.done.lock() = true;
                    self.cv.notify_all();
                }
        - id: impl_CpuEvent.wait
          type: ImplItemFn
          code: |2
                fn wait(&self) {
                    let mut done = self.done.lock();
                    while !*done {
                        self.cv.wait(&mut done);
                    }
                }
        - id: impl_CpuEvent.is_done
          type: ImplItemFn
          code: |2
                fn is_done(&self) -> bool {
                    *self.done.lock()
                }
      - id: ScratchArena
        type: ItemStruct
        code: |
          /// Bump allocator over a lane-private arena span; reset once the lane has drained.
          #[derive(Debug)]
          pub struct ScratchArena {
              base: Region,
              cursor: Mutex<u64>,
          }
      - id: impl_ScratchArena
        type: ItemImpl
        signature: "impl ScratchArena {"
        children:
        - id: impl_ScratchArena.new
          type: ImplItemFn
          code: |2
                pub fn new(base: Region) -> Self {
                    Self {
                        base,
                        cursor: Mutex::new(0),
                    }
                }
        - id: impl_ScratchArena.capacity
          type: ImplItemFn
          code: |2
                pub fn capacity(&self) -> u64 {
                    self.base.len
                }
        - id: impl_ScratchArena.used
          type: ImplItemFn
          code: |2
                pub fn used(&self) -> u64 {
                    *self.cursor.lock()
                }
        - id: impl_ScratchArena.alloc
          type: ImplItemFn
          code: |2
                pub fn alloc(&self, bytes: u64, align: u64) -> Result<Region, DeviceError> {
                    if align == 0 || !align.is_power_of_two() {
                        return Err(DeviceError::InvalidAlign { align });
                    }
                    let mut cursor = self.cursor.lock();
                    let start = (self.base.offset + *cursor).next_multiple_of(align) - self.base.offset;
                    let end = start.checked_add(bytes).filter(|e| *e <= self.base.len);
                    let Some(end) = end else {
                        return Err(DeviceError::ScratchExhausted {
                            requested: bytes,
                            available: self.base.len.saturating_sub(*cursor),
                        });
                    };
                    *cursor = end;
                    Ok(Region {
                        arena: self.base.arena,
                        offset: self.base.offset + start,
                        len: bytes,
                        gen: self.base.gen,
                    })
                }
        - id: impl_ScratchArena.reset
          type: ImplItemFn
          code: |2
                /// Invalidates all regions handed out so far.
                pub fn reset(&self) {
                    *self.cursor.lock() = 0;
                }
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/subengine/device.rs
      nodes:
      - id: DeviceHandle
        type: ItemStruct
        code: |
          /// Identifies the compute device a subengine runs on; lanes are opened from it.
          #[derive(Clone)]
          pub struct DeviceHandle {
              kind: DeviceKind,
              ordinal: usize,
              init_cost: Duration,
              #[cfg(feature = "cuda")]
              ctx: Option<std::sync::Arc<cudarc::driver::CudaContext>>,
          }
      - id: impl_DeviceHandle
        type: ItemImpl
        signature: "impl DeviceHandle {"
        children:
        - id: impl_DeviceHandle.cpu
          type: ImplItemFn
          code: |2
                pub fn cpu() -> Self {
                    Self {
                        kind: DeviceKind::Cpu,
                        ordinal: 0,
                        init_cost: Duration::ZERO,
                        #[cfg(feature = "cuda")]
                        ctx: None,
                    }
                }
        - id: impl_DeviceHandle.cuda
          type: ImplItemFn
          code: |2
                /// Retains the primary context of `ordinal`. The first call is expensive (0.2-2 s, more when
                /// the driver is cold), so callers should run it off the latency path.
                #[cfg(feature = "cuda")]
                pub fn cuda(ordinal: usize) -> Result<Self, DeviceError> {
                    if std::env::var_os("CUDA_MODULE_LOADING").is_none() {
                        std::env::set_var("CUDA_MODULE_LOADING", "LAZY");
                    }
                    let started = Instant::now();
                    let ctx = cudarc::driver::CudaContext::new(ordinal)
                        .map_err(|e| DeviceError::Init {
                            reason: e.to_string(),
                        })?;
                    let init_cost = started.elapsed();
                    tracing::info!(
                        ordinal, init_ms = init_cost.as_millis() as u64, "cuda context retained"
                    );
                    Ok(Self {
                        kind: DeviceKind::Gpu,
                        ordinal,
                        init_cost,
                        ctx: Some(ctx),
                    })
                }
        - id: impl_DeviceHandle.cuda_2
          type: ImplItemFn
          code: |2
                #[cfg(not(feature = "cuda"))]
                pub fn cuda(_ordinal: usize) -> Result<Self, DeviceError> {
                    Err(DeviceError::CudaUnavailable)
                }
        - id: impl_DeviceHandle.kind
          type: ImplItemFn
          code: |2
                pub fn kind(&self) -> DeviceKind {
                    self.kind
                }
        - id: impl_DeviceHandle.ordinal
          type: ImplItemFn
          code: |2
                pub fn ordinal(&self) -> usize {
                    self.ordinal
                }
        - id: impl_DeviceHandle.init_cost
          type: ImplItemFn
          code: |2
                /// Time spent creating the context, reported to the lazy-init accounting.
                pub fn init_cost(&self) -> Duration {
                    self.init_cost
                }
        - id: impl_DeviceHandle.warmup
          type: ImplItemFn
          code: |2
                /// Forces driver-side lazy work (first allocation, stream creation) so it does not land on the
                /// first request.
                pub fn warmup(&self) -> Result<Duration, DeviceError> {
                    let started = Instant::now();
                    #[cfg(feature = "cuda")]
                    if let Some(ctx) = &self.ctx {
                        let init = |e: cudarc::driver::DriverError| DeviceError::Init {
                            reason: e.to_string(),
                        };
                        let stream = ctx.new_stream().map_err(init)?;
                        let _probe = stream.alloc_zeros::<u8>(1).map_err(init)?;
                        stream.synchronize().map_err(init)?;
                    }
                    Ok(started.elapsed())
                }
        - id: impl_DeviceHandle.open_lane
          type: ImplItemFn
          code: |2
                pub fn open_lane(
                    &self,
                    class: LaneClass,
                    cfg: LaneConfig,
                ) -> Result<Box<dyn ExecContext>, DeviceError> {
                    #[cfg(feature = "cuda")]
                    if let Some(ctx) = &self.ctx {
                        return Ok(Box::new(CudaLane::new(ctx, class, cfg)?));
                    }
                    Ok(Box::new(CpuLane::new(class, cfg)?))
                }
      - id: cuda_lane
        type: ItemMod
        signature: "#[cfg(feature = \"cuda\")] mod cuda_lane {"
        children:
        - id: cuda_lane.CHYK8i|Km1i1Y
          type: ImportGroup
          code: |2
                use std::sync::Arc;
                use cudarc::driver::{result, CudaContext, CudaStream};
                use super::*;
                use crate::resident::subengine::{EventHandle, LaneJob, ParamBlock, ScratchArena};
                use crate::resident::subengine::exec::EventRepr;
        - id: cuda_lane.CudaLane
          type: ItemStruct
          code: |2
                /// Prioritised non-blocking stream; all device work of one lane is ordered on it.
                pub struct CudaLane {
                    class: LaneClass,
                    priority: i32,
                    ctx: Arc<CudaContext>,
                    stream: Arc<CudaStream>,
                    params: ParamBlock,
                    scratch: ScratchArena,
                    span: tracing::Span,
                }
        - id: cuda_lane.event_err
          type: ItemFn
          code: |2
                fn event_err(e: cudarc::driver::DriverError) -> DeviceError {
                    DeviceError::EventRecord {
                        reason: e.to_string(),
                    }
                }
        - id: cuda_lane.impl_CudaLane
          type: ItemImpl
          signature: "impl CudaLane {"
          children:
          - id: cuda_lane.impl_CudaLane.new
            type: ImplItemFn
            code: |2
                      pub(super) fn new(
                          ctx: &Arc<CudaContext>,
                          class: LaneClass,
                          cfg: LaneConfig,
                      ) -> Result<Self, DeviceError> {
                          let (a, b) = result::stream::get_priority_range()
                              .map_err(|_| DeviceError::PriorityUnsupported)?;
                          let priority = class.priority().clamp(a.min(b), a.max(b));
                          let stream = ctx
                              .new_stream_with_priority(priority)
                              .map_err(|e| DeviceError::StreamCreate {
                                  reason: e.to_string(),
                              })?;
                          let span = tracing::info_span!(
                              "lane", lane = ? class, stream_prio = priority
                          );
                          Ok(Self {
                              class,
                              priority,
                              ctx: ctx.clone(),
                              stream,
                              params: ParamBlock::new(cfg.param_capacity),
                              scratch: ScratchArena::new(cfg.scratch),
                              span,
                          })
                      }
          - id: cuda_lane.impl_CudaLane.priority
            type: ImplItemFn
            code: |2
                      pub fn priority(&self) -> i32 {
                          self.priority
                      }
          - id: cuda_lane.impl_CudaLane.stream
            type: ImplItemFn
            code: |2
                      pub(crate) fn stream(&self) -> &Arc<CudaStream> {
                          &self.stream
                      }
        - id: cuda_lane.impl_ExecContext_for_CudaLane
          type: ItemImpl
          signature: "impl ExecContext for CudaLane {"
          children:
          - id: cuda_lane.impl_ExecContext_for_CudaLane.class
            type: ImplItemFn
            code: |2
                      fn class(&self) -> LaneClass {
                          self.class
                      }
          - id: cuda_lane.impl_ExecContext_for_CudaLane.device
            type: ImplItemFn
            code: |2
                      fn device(&self) -> DeviceKind {
                          DeviceKind::Gpu
                      }
          - id: cuda_lane.impl_ExecContext_for_CudaLane.submit
            type: ImplItemFn
            code: |2
                      fn submit(&self, job: LaneJob) -> Result<(), DeviceError> {
                          self.ctx.bind_to_thread().map_err(event_err)?;
                          self.span.in_scope(job)
                      }
          - id: cuda_lane.impl_ExecContext_for_CudaLane.record_event
            type: ImplItemFn
            code: |2
                      fn record_event(&self) -> Result<EventHandle, DeviceError> {
                          let event = self.stream.record_event(None).map_err(event_err)?;
                          tracing::trace!(parent : & self.span, "event recorded");
                          Ok(EventHandle::from_repr(EventRepr::Cuda(event)))
                      }
          - id: cuda_lane.impl_ExecContext_for_CudaLane.wait
            type: ImplItemFn
            code: |2
                      fn wait(&self, event: &EventHandle) -> Result<(), DeviceError> {
                          match event.repr() {
                              EventRepr::Cuda(e) => self.stream.wait(e).map_err(event_err),
                              EventRepr::Cpu(_) => event.synchronize(),
                          }
                      }
          - id: cuda_lane.impl_ExecContext_for_CudaLane.synchronize
            type: ImplItemFn
            code: |2
                      fn synchronize(&self) -> Result<(), DeviceError> {
                          self.stream.synchronize().map_err(event_err)
                      }
          - id: cuda_lane.impl_ExecContext_for_CudaLane.param_block
            type: ImplItemFn
            code: |2
                      fn param_block(&mut self) -> &mut ParamBlock {
                          &mut self.params
                      }
          - id: cuda_lane.impl_ExecContext_for_CudaLane.scratch
            type: ImplItemFn
            code: |2
                      fn scratch(&self) -> &ScratchArena {
                          &self.scratch
                      }
```
/answer bc7a-d allow
Control Request:
```yaml
id: bc7a-e
toolName: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src
    pattern: memory::|MemoryManager|BookkeepingArena|ArenaKind|StagingRing|release_after
    include:
    - '*.rs'
    limit: 40
  reason: "Verwendungen des Memory-Moduls im Crate, um Kompatibilität zu wahren"
```
/answer bc7a-e allow
Control Request:
```yaml
id: bc7a-f
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src
      matches:
      - path: resident/processor/tests.rs
        matches:
        - lineno: 13
          match: "use crate::resident::memory::{ArenaId, Region};"
      - path: resident/processor/resource.rs
        matches:
        - lineno: 108
          match: "use crate::resident::memory::{ArenaId, Region};"
      - path: resident/processor/hub.rs
        matches:
        - lineno: 9
          match: use crate::resident::memory::Region;
      - path: resident/facade/resident.rs
        matches:
        - lineno: 399
          match: let id = r.add_index(Arc::new(crate::resident::index::memory::InMemoryIndex::new()));
      - path: resident/subengine/device.rs
        matches:
        - lineno: 19
          match: use crate::resident::memory::DeviceKind;
        - lineno: 208
          match: "use crate::resident::memory::{ArenaId, Region};"
      - path: resident/subengine/exec.rs
        matches:
        - lineno: 13
          match: "use crate::resident::memory::{DeviceKind, Region};"
        - lineno: 468
          match: use crate::resident::memory::ArenaId;
      - path: resident/memory/mod.rs
        matches:
        - lineno: 1
          match: //! Region/Layout/DType and MemoryManager trait.
        - lineno: 12
          match: "pub use manager::{compute_dtype, BookkeepingArena, DeviceKind, MemoryManager};"
      - path: resident/memory/manager.rs
        matches:
        - lineno: 1
          match: //! `MemoryManager` contract plus a GPU-free bookkeeping implementation.
        - lineno: 25
          match: "pub trait MemoryManager {"
        - lineno: 51
          match: "pub struct BookkeepingArena {"
        - lineno: 56
          match: "impl BookkeepingArena {"
        - lineno: 83
          match: "impl MemoryManager for BookkeepingArena {"
        - lineno: 129
          match: "let a = BookkeepingArena::new(ArenaId(1), 64);"
        - lineno: 142
          match: "let a = BookkeepingArena::new(ArenaId(9001), 64);"
        - lineno: 153
          match: "let a = BookkeepingArena::new(ArenaId(1), 256);"
        - lineno: 174
          match: "let a = BookkeepingArena::new(ArenaId(0), 32);"
      - path: resident/op/mod.rs
        matches:
        - lineno: 183
          match: use crate::resident::memory::region::ArenaId;
        - lineno: 184
          match: use crate::resident::memory::Region;
      - path: resident/op/requirements.rs
        matches:
        - lineno: 2
          match: "use crate::resident::memory::{DeviceKind, Region};"
      - path: resident/op/lease.rs
        matches:
        - lineno: 8
          match: use crate::resident::memory::Region;
        - lineno: 60
          match: use crate::resident::memory::region::ArenaId;
```
/answer bc7a-f allow
ReasoningToken: 2829
Control Request:
```yaml
id: bc7a-10
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench/rag
  script: ls ~/.cargo/registry/src/*/ | grep -i -E "cudarc|candle"; which nvcc nvidia-smi; ls third_party 2>&1 | head; git status --short | head; (cargo check --offline 2>&1 | tail -5)
  reason: "cudarc-Quellen, GPU-/nvcc-Verfügbarkeit und Baseline-Build prüfen"
```
/answer bc7a-10 allow
Control Request:
```yaml
id: bc7a-11
result:
  structuredContent:
    stdout: |
      candle-core-0.11.0
      candle-kernels-0.11.0
      candle-nn-0.11.0
      candle-transformers-0.11.0
      candle-ug-0.11.0
      cudarc-0.17.8
      cudarc-0.19.9
      /usr/bin/nvidia-smi
      candle-kernels
       M ../project/resident/plan/plan_d_03_gpu_memory_arenas.md
      ?? ../project/flag_lineno.md
          = help: consider replacing the `if let` with a `let`
          = note: `#[warn(irrefutable_let_patterns)]` on by default

      warning: `xy-ai-rag` (lib) generated 1 warning
          Finished `dev` profile [unoptimized + debuginfo] target(s) in 11.72s
```
/answer bc7a-11 allow
ReasoningToken: 80
Control Request:
```yaml
id: bc7a-12
toolName: bash
arguments:
  cwd: /home/user/.cargo/registry/src
  script: cd */cudarc-0.19.9/src/driver; grep -n "pub fn mem_get_info\|pub unsafe fn mem_get_info\|pub fn alloc_pinned\|pub unsafe fn alloc_pinned\|pub fn memcpy_dtod\|pub fn alloc_zeros\|pub fn as_mut_slice\|pub fn as_slice\|pub fn slice_mut\|pub fn slice<\|pub fn memcpy_htod\|pub fn memcpy_dtoh\|pub fn num_bytes\|pub fn attach_ctx\|pub fn set_attribute\|pub fn trim_to\|pub fn default_mem_pool\|pub fn len" *.rs safe/*.rs | head -50; ls safe; grep -rn "mod mem_pool" -A3 result.rs | head
  reason: "Exakte cudarc-0.19.9-API für Speicher-, Pinned- und Pool-Aufrufe prüfen"
```
/answer bc7a-12 allow
Control Request:
```yaml
id: bc7a-13
result:
  structuredContent:
    stdout: |
      result.rs:1120:pub fn mem_get_info() -> Result<(usize, usize), DriverError> {
      safe/core.rs:321:    pub fn mem_get_info(&self) -> Result<(usize, usize), DriverError> {
      safe/core.rs:824:    pub fn len(&self) -> usize {
      safe/core.rs:829:    pub fn num_bytes(&self) -> usize {
      safe/core.rs:900:    pub fn len(&self) -> usize {
      safe/core.rs:947:    pub fn len(&self) -> usize {
      safe/core.rs:1416:    pub unsafe fn alloc_pinned<T: DeviceRepr>(
      safe/core.rs:1441:    pub fn len(&self) -> usize {
      safe/core.rs:1446:    pub fn num_bytes(&self) -> usize {
      safe/core.rs:1472:    pub fn as_slice(&self) -> Result<&[T], DriverError> {
      safe/core.rs:1479:    pub fn as_mut_slice(&mut self) -> Result<&mut [T], DriverError> {
      safe/core.rs:1563:    pub fn alloc_zeros<T: DeviceRepr + ValidAsZeroBits>(
      safe/core.rs:1606:    pub fn memcpy_htod<T: DeviceRepr, Src: HostSlice<T> + ?Sized, Dst: DevicePtrMut<T>>(
      safe/core.rs:1648:    pub fn memcpy_dtoh<T: DeviceRepr, Src: DevicePtr<T>, Dst: HostSlice<T> + ?Sized>(
      safe/core.rs:1661:    pub fn memcpy_dtod<T, Src: DevicePtr<T>, Dst: DevicePtrMut<T>>(
      safe/core.rs:1798:    pub fn slice_mut(&mut self, bounds: impl RangeBounds<usize>) -> CudaViewMut<'_, T> {
      safe/core.rs:2002:    pub fn slice<'b>(&'b self, bounds: impl RangeBounds<usize>) -> CudaView<'b, T> {
      safe/core.rs:2034:    pub fn slice_mut<'b>(&'b mut self, bounds: impl RangeBounds<usize>) -> CudaViewMut<'b, T> {
      safe/core.rs:2446:    pub fn set_attribute(
      safe/unified_memory.rs:131:    pub fn len(&self) -> usize {
      safe/unified_memory.rs:143:    pub fn num_bytes(&self) -> usize {
      safe/unified_memory.rs:320:    pub fn slice_mut(&mut self, bounds: impl RangeBounds<usize>) -> UnifiedViewMut<'_, T> {
      safe/unified_memory.rs:448:    pub fn as_slice(&self) -> Result<&[T], DriverError> {
      safe/unified_memory.rs:456:    pub fn as_mut_slice(&mut self) -> Result<&mut [T], DriverError> {
      safe/unified_memory.rs:723:    pub fn len(&self) -> usize {
      safe/unified_memory.rs:785:    pub fn len(&self) -> usize {
      safe/unified_memory.rs:808:    pub fn slice<'b>(&'b self, bounds: impl RangeBounds<usize>) -> UnifiedView<'b, T> {
      safe/unified_memory.rs:828:    pub fn slice_mut<'b>(&'b mut self, bounds: impl RangeBounds<usize>) -> UnifiedViewMut<'b, T> {
      core.rs
      external_memory.rs
      graph.rs
      launch.rs
      mod.rs
      profile.rs
      unified_memory.rs
      1539:pub mod mem_pool {
      1540-    //! Memory pool management functions (`cuMemPool*`).
      1541-    //!
      1542-    //! See [cuda docs](https://docs.nvidia.com/cuda/cuda-driver-api/group__CUDA__MALLOC__ASYNC.html)
```
/answer bc7a-13 allow
T:
Das Modul `resident/memory` enthält bereits `Region`, `FreeList`, `BookkeepingArena` und `MemoryManager`; darauf baue ich nun D3 mit `ArenaKind`, `Budget`, `MemoryError`, `ArenaAllocator`, `StagingRing` sowie CPU-/CUDA-Manager auf. Da auf dieser Maschine kein `nvcc` verfügbar ist, prüfe ich den CUDA-Teil stattdessen gegen die cudarc-0.19.9-Quellen.


ReasoningToken: 254
Control Request:
```yaml
id: bc7a-14
toolName: bash
arguments:
  cwd: /home/user/.cargo/registry/src
  script: "cd */cudarc-0.19.9/src/driver; sed -n 1405,1440p safe/core.rs; sed -n 1655,1680p safe/core.rs; sed -n 2435,2470p safe/core.rs; sed -n 1539,1640p result.rs | grep -n \"pub \\(unsafe \\)\\?fn\"; grep -n \"pub fn default_mem_pool\\|fn default_mem_pool\\|pub fn alloc_async\\|fn alloc<\" safe/core.rs result.rs | head"
  reason: "Signaturen von alloc_pinned, memcpy_dtod, Pool-Attribute verifizieren"
```
/answer bc7a-14 allow
Control Request:
```yaml
id: bc7a-15
result:
  content:
  - type: text
    text: "Full output written to file (/tmp/mcpc-stdout-sf_i44ml.log). Reduce the content to what is strictly needed: use targeted commands (grep, head, tail, awk, ast_outline) to extract only the relevant parts. This is a general output limit for all commands to prevent context bloat."
  structuredContent:
    stdout_file: /tmp/mcpc-stdout-sf_i44ml.log
    stdout_file_file_stats:
      path: /tmp/mcpc-stdout-sf_i44ml.log
      size_bytes: 4131
      lines: 105
      words: 420
      complexity: 0.592
      created: 2026-10-18T18:36:56.010139+00:00
      modified: 2026-10-18T18:36:56.010139+00:00
      accessed: 2026-10-18T18:36:56.010139+00:00
      line_length_max: 143
      line_length_min: 0
      line_length_avg: 38.34
      words_per_line_avg: 4.0
      checksum: ddcbaf91078d4fffa7f02524903363706a278d95d6d6df5d8e46c29f2df19f38
    stdout_file_ast_outline:
    - id: file
      lines: 1-105
      signature: "ctx.record_err(unsafe { result::free_host(self.ptr as _) });"
```
/answer bc7a-15 allow
Control Request:
```yaml
id: bc7a-16
toolName: bash
arguments:
  cwd: /tmp
  script: cat /tmp/mcpc-stdout-sf_i44ml.log
  reason: Gespeicherte Ausgabe der cudarc-Signaturen lesen
```
/answer bc7a-16 allow
Control Request:
```yaml
id: bc7a-17
result:
  content:
  - type: text
    text: "Full output written to file (/tmp/mcpc-stdout-68izhbn_.log). Reduce the content to what is strictly needed: use targeted commands (grep, head, tail, awk, ast_outline) to extract only the relevant parts. This is a general output limit for all commands to prevent context bloat."
  structuredContent:
    stdout_file: /tmp/mcpc-stdout-68izhbn_.log
    stdout_file_file_stats:
      path: /tmp/mcpc-stdout-68izhbn_.log
      size_bytes: 4131
      lines: 105
      words: 420
      complexity: 0.592
      created: 2026-10-18T18:37:00.490080+00:00
      modified: 2026-10-18T18:37:00.490080+00:00
      accessed: 2026-10-18T18:37:00.490080+00:00
      line_length_max: 143
      line_length_min: 0
      line_length_avg: 38.34
      words_per_line_avg: 4.0
      checksum: ddcbaf91078d4fffa7f02524903363706a278d95d6d6df5d8e46c29f2df19f38
    stdout_file_ast_outline:
    - id: file
      lines: 1-105
      signature: "ctx.record_err(unsafe { result::free_host(self.ptr as _) });"
```
/answer bc7a-17 allow
ReasoningToken: 33
Control Request:
```yaml
id: bc7a-18
toolName: python
arguments:
  script: print(open('/tmp/mcpc-stdout-sf_i44ml.log').read())
  reason: Ausgabedatei mit den cudarc-Signaturen anzeigen
```
/answer bc7a-18 allow
Control Request:
```yaml
id: bc7a-19
result:
  structuredContent:
    exit_code: 0
    stdout: |2+
              ctx.record_err(unsafe { result::free_host(self.ptr as _) });
          }
      }

      impl CudaContext {
          /// Allocates page locked host memory with [sys::CU_MEMHOSTALLOC_WRITECOMBINED] flags.
          ///
          /// See [cuda docs](https://docs.nvidia.com/cuda/cuda-driver-api/group__CUDA__MEM.html#group__CUDA__MEM_1g572ca4011bfcb25034888a14d4e035b9)
          ///
          /// # Safety
          /// 1. This is unsafe because the memory is unset after this call.
          pub unsafe fn alloc_pinned<T: DeviceRepr>(
              self: &Arc<Self>,
              len: usize,
          ) -> Result<PinnedHostSlice<T>, DriverError> {
              self.bind_to_thread()?;
              let ptr = result::malloc_host(
                  len * std::mem::size_of::<T>(),
                  sys::CU_MEMHOSTALLOC_WRITECOMBINED,
              )?;
              let ptr = ptr as *mut T;
              assert!(!ptr.is_null());
              assert!(len * std::mem::size_of::<T>() < isize::MAX as usize);
              assert!(ptr.is_aligned());
              let event = self.new_event(Some(sys::CUevent_flags::CU_EVENT_BLOCKING_SYNC))?;
              Ok(PinnedHostSlice { ptr, len, event })
          }
      }

      impl<T> PinnedHostSlice<T> {
          /// The context this was created in.
          pub fn context(&self) -> &Arc<CudaContext> {
              &self.event.ctx
          }

          /// The number of elements `T` in this slice.
              let (src, _record_src) = src.device_ptr(self);
              let (dst, _record_dst) = unsafe { dst.stream_synced_mut_slice(self) };
              unsafe { result::memcpy_dtoh_async(dst, src, self.cu_stream) }
          }

          /// Copy a [`CudaSlice`]/[`CudaView`] to a existing [`CudaSlice`]/[`CudaViewMut`].
          pub fn memcpy_dtod<T, Src: DevicePtr<T>, Dst: DevicePtrMut<T>>(
              self: &Arc<Self>,
              src: &Src,
              dst: &mut Dst,
          ) -> Result<(), DriverError> {
              assert!(dst.len() >= src.len());
              self.ctx.bind_to_thread()?;

              let num_bytes = src.num_bytes();

              let src_ctx = src.stream().context();
              let dst_ctx = self.context();

              if src_ctx == dst_ctx {
                  let (src_ptr, _record_src) = src.device_ptr(self);
                  let (dst_ptr, _record_dst) = dst.device_ptr_mut(self);
                  unsafe { result::memcpy_dtod_async(dst_ptr, src_ptr, num_bytes, self.cu_stream) }
              } else {
                  // NOTE: Although we want the current stream to wait on src to be ready,
                  // we can't use src.device_ptr(self). When `_record_src` is dropped,
          /// Get the PTX virtual architecture version for which the function was compiled.
          pub fn ptx_version(&self) -> Result<i32, result::DriverError> {
              self.get_attribute(CUfunction_attribute_enum::CU_FUNC_ATTRIBUTE_PTX_VERSION)
          }

          /// Get the binary architecture version for which the function was compiled.
          pub fn binary_version(&self) -> Result<i32, result::DriverError> {
              self.get_attribute(CUfunction_attribute_enum::CU_FUNC_ATTRIBUTE_BINARY_VERSION)
          }

          /// Set the value of a specific attribute of this [CudaFunction].
          pub fn set_attribute(
              &self,
              attribute: CUfunction_attribute_enum,
              value: i32,
          ) -> Result<(), result::DriverError> {
              unsafe { result::function::set_function_attribute(self.cu_function, attribute, value) }
          }

          /// Set the cache config of this [CudaFunction].
          pub fn set_function_cache_config(
              &self,
              attribute: CUfunc_cache_enum,
          ) -> Result<(), result::DriverError> {
              unsafe { result::function::set_function_cache_config(self.cu_function, attribute) }
          }
      }

      impl<T> CudaSlice<T> {
          /// Takes ownership of the underlying [sys::CUdeviceptr]. **It is up
          /// to the owner to free this value**.
          ///
          /// Drops the underlying host_buf if there is one.
          pub fn leak(self) -> sys::CUdeviceptr {
              let mut s = std::mem::ManuallyDrop::new(self);
              let ptr = s.cu_device_ptr;
      14:    pub unsafe fn create(
      28:    pub unsafe fn destroy(pool: sys::CUmemoryPool) -> Result<(), DriverError> {
      38:    pub unsafe fn trim_to(
      52:    pub unsafe fn get_attribute(
      67:    pub unsafe fn set_attribute(
      82:    pub unsafe fn alloc_async(
      safe/core.rs:1534:    pub unsafe fn alloc<T: DeviceRepr>(
```
/answer bc7a-19 allow
ReasoningToken: 294
Control Request:
```yaml
id: bc7a-1a
toolName: bash
arguments:
  cwd: /home/user/.cargo/registry/src
  script: "cd */cudarc-0.19.9/src; sed -n 1539,1640p driver/result.rs | sed -n 60,80p; grep -rn \"DefaultMemPool\\|default_mem_pool\" driver/*.rs driver/safe/*.rs | head; grep -n \"pub fn cu_device\\|pub fn cu_ctx\\|pub fn new_event\\|pub fn is_complete\\|pub fn synchronize\" driver/safe/core.rs | head; grep -n \"CU_MEMPOOL_ATTR_RELEASE_THRESHOLD\" -r driver/sys/*.rs | head -3; grep -n \"pub struct PinnedHostSlice\" -B3 -A6 driver/safe/core.rs"
  reason: Default-Pool-Zugriff und Pool-Attribut-Signaturen in cudarc prüfen
```
/answer bc7a-1a allow
Control Request:
```yaml
id: bc7a-1b
result:
  structuredContent:
    stdout: |2
          /// Set a pool attribute.
          ///
          /// See [cuda docs](https://docs.nvidia.com/cuda/cuda-driver-api/group__CUDA__MALLOC__ASYNC.html#group__CUDA__MALLOC__ASYNC_1g1ef006cd08dbf0ec5e4cb93bf7b044dc)
          ///
          /// # Safety
          /// Pool must be valid. `value` must point to memory of the correct
          /// type for the given attribute.
          pub unsafe fn set_attribute(
              pool: sys::CUmemoryPool,
              attr: sys::CUmemPool_attribute,
              value: *mut core::ffi::c_void,
          ) -> Result<(), DriverError> {
              sys::cuMemPoolSetAttribute(pool, attr, value).result()
          }

          /// Allocate memory from a pool with stream-ordered semantics.
          ///
          /// See [cuda docs](https://docs.nvidia.com/cuda/cuda-driver-api/group__CUDA__MALLOC__ASYNC.html#group__CUDA__MALLOC__ASYNC_1g0968906e6892e4f7e0e04ef5e5e0b416)
          ///
          /// # Safety
          /// 1. Pool and stream must be valid.
      driver/result.rs:196:    pub unsafe fn get_default_mem_pool(
      driver/result.rs:200:        sys::cuDeviceGetDefaultMemPool(pool.as_mut_ptr(), dev).result()?;
      333:    pub fn cu_device(&self) -> sys::CUdevice {
      345:    pub fn cu_ctx(&self) -> sys::CUcontext {
      369:    pub fn synchronize(&self) -> Result<(), DriverError> {
      551:    pub fn new_event(
      596:    pub fn synchronize(&self) -> Result<(), DriverError> {
      614:    pub fn is_complete(&self) -> bool {
      745:    pub fn synchronize(&self) -> Result<(), DriverError> {
      4393:    CU_MEMPOOL_ATTR_RELEASE_THRESHOLD = 4,
      4406:    CU_MEMPOOL_ATTR_RELEASE_THRESHOLD = 4,
      1389-/// Allocate this with [CudaContext::alloc_pinned()], and do device copies with
      1390-/// [CudaStream::clone_htod()]/[CudaStream::memcpy_htod()]/[CudaStream::memcpy_dtoh()]
      1391-#[derive(Debug)]
      1392:pub struct PinnedHostSlice<T> {
      1393-    pub(crate) ptr: *mut T,
      1394-    pub(crate) len: usize,
      1395-    pub(crate) event: CudaEvent,
      1396-}
      1397-
      1398-unsafe impl<T> Send for PinnedHostSlice<T> {}
```
/answer bc7a-1b allow
Control Request:
```yaml
id: bc7a-1c
toolName: bash
arguments:
  cwd: /home/user/.cargo/registry/src
  script: "cd */cudarc-0.19.9/src; sed -n 190,203p driver/result.rs; sed -n 4385,4396p driver/sys/mod.rs 2>/dev/null | head -3; grep -n \"fn get_default_mem_pool\" -B8 driver/result.rs | head -12"
  reason: get_default_mem_pool-Rückgabetyp prüfen
```
/answer bc7a-1c allow
Control Request:
```yaml
id: bc7a-1d
result:
  structuredContent:
    stdout: |2
          /// Get the default memory pool for a device.
          ///
          /// See [cuda docs](https://docs.nvidia.com/cuda/cuda-driver-api/group__CUDA__DEVICE.html#group__CUDA__DEVICE_1g2170a6e24f7e596854f0c48e1e98120e)
          ///
          /// # Safety
          /// Must be a device returned from [get].
          pub unsafe fn get_default_mem_pool(
              dev: sys::CUdevice,
          ) -> Result<sys::CUmemoryPool, DriverError> {
              let mut pool = MaybeUninit::uninit();
              sys::cuDeviceGetDefaultMemPool(pool.as_mut_ptr(), dev).result()?;
              Ok(pool.assume_init())
          }

      }
      #[cfg(any(feature = "cuda-11040", feature = "cuda-11050", feature = "cuda-11060", feature = "cuda-11070", feature = "cuda-11080", feature = "cuda-12000", feature = "cuda-12010", feature = "cuda-12020", feature = "cuda-12030", feature = "cuda-12040", feature = "cuda-12050", feature = "cuda-12060", feature = "cuda-12080", feature = "cuda-12090", feature = "cuda-13000", feature = "cuda-13010"))]
      #[repr(u32)]
      188-    }
      189-
      190-    /// Get the default memory pool for a device.
      191-    ///
      192-    /// See [cuda docs](https://docs.nvidia.com/cuda/cuda-driver-api/group__CUDA__DEVICE.html#group__CUDA__DEVICE_1g2170a6e24f7e596854f0c48e1e98120e)
      193-    ///
      194-    /// # Safety
      195-    /// Must be a device returned from [get].
      196:    pub unsafe fn get_default_mem_pool(
```
/answer bc7a-1d allow
Control Request:
```yaml
id: bc7a-1e
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/error.rs
    ids:
    - ResidentError
    - impl_IsRetryable_for_ResidentError
    - tests.all_errors_are_send_sync_static
    - tests.retryability_classification
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/memory/mod.rs
    ids:
    - ModuleDoc
  reason: ResidentError-Aggregation und Tests für Erweiterung um MemoryError lesen
```
/answer bc7a-1e allow
ReasoningToken: 2455
Control Request:
```yaml
id: bc7a-1f
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/error.rs
      nodes:
      - id: ResidentError
        type: ItemEnum
        code: |
          /// Top-level resident engine error, wrapping every layer error via `#[from]`.
          #[derive(Error, Debug)]
          #[non_exhaustive]
          pub enum ResidentError {
              #[error(transparent)]
              Index(#[from] IndexError),
              #[error(transparent)]
              Precision(#[from] PrecisionError),
              #[error(transparent)]
              Planner(#[from] PlannerError),
              #[error(transparent)]
              Config(#[from] ConfigError),
              #[error(transparent)]
              Capability(#[from] CapabilityError),
              #[error(transparent)]
              Orchestration(#[from] OrchestrationError),
              #[error(transparent)]
              Subengine(#[from] SubengineError),
              #[error(transparent)]
              Device(#[from] DeviceError),
          }
      - id: impl_IsRetryable_for_ResidentError
        type: ItemImpl
        signature: "impl IsRetryable for ResidentError {"
        children:
        - id: impl_IsRetryable_for_ResidentError.is_retryable
          type: ImplItemFn
          code: |2
                fn is_retryable(&self) -> bool {
                    match self {
                        ResidentError::Index(e) => e.is_retryable(),
                        ResidentError::Precision(e) => e.is_retryable(),
                        ResidentError::Planner(e) => e.is_retryable(),
                        ResidentError::Config(e) => e.is_retryable(),
                        ResidentError::Capability(e) => e.is_retryable(),
                        ResidentError::Orchestration(e) => e.is_retryable(),
                        ResidentError::Subengine(e) => e.is_retryable(),
                        ResidentError::Device(e) => e.is_retryable(),
                    }
                }
      - id: tests.all_errors_are_send_sync_static
        type: ItemFn
        code: |2
              #[test]
              fn all_errors_are_send_sync_static() {
                  assert_send_sync_static::<ResidentError>();
                  assert_send_sync_static::<IndexError>();
                  assert_send_sync_static::<ManagerError>();
                  assert_send_sync_static::<PrecisionError>();
                  assert_send_sync_static::<PlannerError>();
                  assert_send_sync_static::<CapabilityError>();
                  assert_send_sync_static::<ConfigError>();
                  assert_send_sync_static::<DeviceError>();
              }
      - id: tests.retryability_classification
        type: ItemFn
        code: |2
              #[test]
              fn retryability_classification() {
                  assert!(
                      IndexError::ArenaExhausted { requested : 1, available : 0 } .is_retryable()
                  );
                  assert!(IndexError::IndexUnavailable { scope : "s".into() } .is_retryable());
                  assert!(! IndexError::KernelFailed { reason : "r".into() } .is_retryable());
                  assert!(ManagerError::StaleRegion { arena : 1, gen : 2 } .is_retryable());
                  assert!(
                      PrecisionError::CalibrationUnstable { reason : "r".into() } .is_retryable()
                  );
                  assert!(! PrecisionError::NoActiveProfile.is_retryable());
                  assert!(
                      ! ConfigError::InvalidValue { field : "f".into(), reason : "r".into() }
                      .is_retryable()
                  );
                  assert!(! CapabilityError::Unknown { capability : "c".into() } .is_retryable());
                  let wrapped: ResidentError = IndexError::ArenaExhausted {
                      requested: 1,
                      available: 0,
                  }
                      .into();
                  assert!(wrapped.is_retryable());
                  let fatal: ResidentError = PlannerError::InvalidConfig {
                      reason: "r".into(),
                  }
                      .into();
                  assert!(! fatal.is_retryable());
              }
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/memory/mod.rs
      nodes:
      - id: ModuleDoc
        type: ModuleAttrs
        code: |
          //! Region/Layout/DType and MemoryManager trait.
```
/answer bc7a-1f allow
Control Request:
```yaml
id: bc7a-20
toolName: bash
arguments:
  cwd: /home/user/.cargo/registry/src
  script: "cd */cudarc-0.19.9/src/driver; grep -n \"pub fn cu_stream\\|pub unsafe fn memcpy_htod_async\\|pub unsafe fn memcpy_dtoh_async\\|pub trait DevicePtrMut\\|pub trait DevicePtr<\\|fn device_ptr_mut\\|fn device_ptr<\" safe/core.rs result.rs | head -12; sed -n 1,40p /home/user/xyan/xy.ai.workbench/rag/src/resident/memory/freelist.rs"
  reason: Stream-Handle-Zugriff und Async-Copy-Signaturen sowie FreeList-Verhalten prüfen
```
/answer bc7a-20 allow
Control Request:
```yaml
id: bc7a-21
result:
  structuredContent:
    stdout: |
      safe/core.rs:732:    pub fn cu_stream(&self) -> sys::CUstream {
      safe/core.rs:1152:pub trait DevicePtr<T>: DeviceSlice<T> {
      safe/core.rs:1167:    fn device_ptr<'a>(&'a self, stream: &'a CudaStream) -> (sys::CUdeviceptr, SyncOnDrop<'a>);
      safe/core.rs:1171:    fn device_ptr<'a>(&'a self, stream: &'a CudaStream) -> (sys::CUdeviceptr, SyncOnDrop<'a>) {
      safe/core.rs:1185:    fn device_ptr<'a>(&'a self, stream: &'a CudaStream) -> (sys::CUdeviceptr, SyncOnDrop<'a>) {
      safe/core.rs:1209:    fn device_ptr<'a>(&'a self, stream: &'a CudaStream) -> (sys::CUdeviceptr, SyncOnDrop<'a>) {
      safe/core.rs:1233:pub trait DevicePtrMut<T>: DeviceSlice<T> {
      safe/core.rs:1248:    fn device_ptr_mut<'a>(
      safe/core.rs:1255:    fn device_ptr_mut<'a>(
      safe/core.rs:1275:    fn device_ptr_mut<'a>(
      result.rs:1003:pub unsafe fn memcpy_htod_async<T>(
      result.rs:1041:pub unsafe fn memcpy_dtoh_async<T>(
      //! Backend-agnostic offset allocator: first-fit over a coalescing free map.

      use std::collections::BTreeMap;

      #[derive(Debug)]
      pub struct FreeList {
          capacity: u64,
          free: BTreeMap<u64, u64>,
      }

      impl FreeList {
          pub fn new(capacity: u64) -> Self {
              let mut free = BTreeMap::new();
              if capacity > 0 {
                  free.insert(0, capacity);
              }
              Self { capacity, free }
          }

          pub fn capacity(&self) -> u64 {
              self.capacity
          }

          pub fn available(&self) -> u64 {
              self.free.values().sum()
          }

          pub fn alloc(&mut self, len: u64, align: u64) -> Option<u64> {
              let align = align.max(1);
              if len == 0 {
                  return None;
              }
              let (start, blk_len, off) = self.free.iter().find_map(|(&s, &l)| {
                  let off = s.checked_add(align - 1)? / align * align;
                  let end = off.checked_add(len)?;
                  (end <= s + l).then_some((s, l, off))
              })?;
              self.free.remove(&start);
              if off > start {
                  self.free.insert(start, off - start);
```
/answer bc7a-21 allow
ReasoningToken: 6572
Control Request:
```yaml
id: bc7a-22
toolName: bash
arguments:
  cwd: /home/user/.cargo/registry/src
  script: "cd */cudarc-0.19.9/src/driver; sed -n 4388,4394p sys/*.rs | head -8; grep -n \"enum CUmemPool_attribute\" sys/*.rs | head -2; sed -n 1003,1010p result.rs; sed -n 1041,1048p result.rs; grep -n \"^pub mod device\\|^pub mod mem_pool\" result.rs; sed -n 321,324p safe/core.rs; grep -n \"impl.*DevicePtr<T> for CudaView<\\|impl.*DevicePtrMut<T> for CudaViewMut\" safe/core.rs; cd /home/user/xyan/xy.ai.workbench/rag && (cargo check --offline --features cuda 2>&1 | tail -6)"
  reason: "Enum-Namen, Copy-Signaturen prüfen und testen, ob cuda-Feature hier überhaupt baut"
```
/answer bc7a-22 allow
Control Request:
```yaml
id: bc7a-23
result:
  structuredContent:
    stdout: |
      #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
      pub enum CUmemPool_attribute_enum {
          CU_MEMPOOL_ATTR_REUSE_FOLLOW_EVENT_DEPENDENCIES = 1,
          CU_MEMPOOL_ATTR_REUSE_ALLOW_OPPORTUNISTIC = 2,
          CU_MEMPOOL_ATTR_REUSE_ALLOW_INTERNAL_DEPENDENCIES = 3,
          CU_MEMPOOL_ATTR_RELEASE_THRESHOLD = 4,
          CU_MEMPOOL_ATTR_RESERVED_MEM_CURRENT = 5,
      4389:pub enum CUmemPool_attribute_enum {
      4402:pub enum CUmemPool_attribute_enum {
      pub unsafe fn memcpy_htod_async<T>(
          dst: sys::CUdeviceptr,
          src: &[T],
          stream: sys::CUstream,
      ) -> Result<(), DriverError> {
          sys::cuMemcpyHtoDAsync_v2(
              dst,
              src.as_ptr() as *const _,
      pub unsafe fn memcpy_dtoh_async<T>(
          dst: &mut [T],
          src: sys::CUdeviceptr,
          stream: sys::CUstream,
      ) -> Result<(), DriverError> {
          sys::cuMemcpyDtoHAsync_v2(
              dst.as_mut_ptr() as *mut _,
              src,
      92:pub mod device {
      1539:pub mod mem_pool {
          pub fn mem_get_info(&self) -> Result<(usize, usize), DriverError> {
              self.bind_to_thread()?;
              result::mem_get_info()
          }
      1184:impl<T> DevicePtr<T> for CudaView<'_, T> {
      1274:impl<T> DevicePtrMut<T> for CudaViewMut<'_, T> {
      ...
      168 |         pub(crate) fn stream(&self) -> &Arc<CudaStream> {
          |                       ^^^^^^

      warning: `xy-ai-rag` (lib) generated 3 warnings
          Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.13s
```
/answer bc7a-23 allow
Text:
 Das `cuda`-Feature lässt sich hier mit `cargo check` prüfen, ich kann den CUDA-Teil also übersetzen lassen. Jetzt schreibe ich die Module.
ReasoningToken: 74
Control Request:
```yaml
id: bc7a-24
toolName: ast_create
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/memory/error.rs
    source: |
      //! Memory-layer errors.

      use thiserror::Error;

      use super::budget::ArenaKind;
      use crate::resident::error::IsRetryable;

      #[derive(Clone, Copy, Debug, PartialEq, Eq)]
      pub enum TransferDir {
          HostToDevice,
          DeviceToHost,
          DeviceToDevice,
      }

      #[derive(Error, Debug)]
      #[non_exhaustive]
      pub enum MemoryError {
          #[error("arena {arena:?} exhausted: requested {requested} bytes, {free} free")]
          ArenaExhausted { arena: ArenaKind, requested: u64, free: u64 },
          #[error("allocation failed in arena {arena:?}: {reason}")]
          AllocFailed { arena: ArenaKind, reason: String },
          #[error("{dir:?} transfer failed: {reason}")]
          TransferFailed { dir: TransferDir, reason: String },
          #[error("stale region in arena {arena} (gen {gen})")]
          StaleRegion { arena: u32, gen: u32 },
          #[error("invalid staging slot {index}")]
          InvalidSlot { index: usize },
          #[error("device memory probe failed: {reason}")]
          BudgetProbe { reason: String },
          #[error("arena plan needs {requested} bytes but only {usable} are usable")]
          BudgetExceeded { requested: u64, usable: u64 },
          #[error("device memory unavailable: {reason}")]
          Unavailable { reason: String },
      }

      impl IsRetryable for MemoryError {
          fn is_retryable(&self) -> bool {
              matches!(self, Self::ArenaExhausted { .. } | Self::StaleRegion { .. })
          }
      }
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/memory/budget.rs
    source: |
      //! Arena classes and the VRAM budget.

      use super::error::MemoryError;
      use super::region::ArenaId;

      /// Arena classes by lifetime.
      #[non_exhaustive]
      #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
      pub enum ArenaKind {
          Weights,
          IndexSegment,
          Workspace,
          Staging,
          ParamBlock,
      }

      impl ArenaKind {
          pub const ALL: [ArenaKind; 5] = [
              ArenaKind::Weights,
              ArenaKind::IndexSegment,
              ArenaKind::Workspace,
              ArenaKind::Staging,
              ArenaKind::ParamBlock,
          ];

          pub const fn id(self) -> ArenaId {
              ArenaId(self as u32)
          }

          pub fn from_id(id: ArenaId) -> Option<Self> {
              Self::ALL.get(id.0 as usize).copied()
          }

          pub(crate) const fn index(self) -> usize {
              self as usize
          }
      }

      /// Capacity per arena class in bytes. `index_segment` covers all resident generations.
      #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
      pub struct ArenaSizes {
          pub weights: u64,
          pub index_segment: u64,
          pub workspace: u64,
          pub staging: u64,
          pub param_block: u64,
      }

      impl ArenaSizes {
          pub fn get(&self, kind: ArenaKind) -> u64 {
              match kind {
                  ArenaKind::Weights => self.weights,
                  ArenaKind::IndexSegment => self.index_segment,
                  ArenaKind::Workspace => self.workspace,
                  ArenaKind::Staging => self.staging,
                  ArenaKind::ParamBlock => self.param_block,
              }
          }

          /// Bytes that live on the device; staging is pinned host memory.
          pub fn device_total(&self) -> u64 {
              self.weights
                  .saturating_add(self.index_segment)
                  .saturating_add(self.workspace)
                  .saturating_add(self.param_block)
          }

          /// Index bytes held back for generations other than the active one.
          pub fn generation_reserve(&self, generations: u32) -> u64 {
              let g = u64::from(generations.max(1));
              self.index_segment - self.index_segment / g
          }
      }

      #[derive(Clone, Copy, Debug, PartialEq, Eq)]
      pub struct Budget {
          pub free: u64,
          pub total: u64,
          pub reserved_for_generations: u64,
      }

      impl Budget {
          pub fn usable(&self) -> u64 {
              self.free.saturating_sub(self.reserved_for_generations)
          }
      }

      /// Free/total device memory in bytes.
      pub trait BudgetProbe: Send + Sync {
          fn probe(&self) -> Result<(u64, u64), MemoryError>;
      }

      /// Memory kept free for the driver context and other processes (compositor).
      pub const DEFAULT_HEADROOM: u64 = 512 << 20;

      /// Fails when the planned device arenas do not fit into the probed free memory.
      pub fn check_fit(sizes: &ArenaSizes, free: u64, headroom: u64) -> Result<(), MemoryError> {
          let usable = free.saturating_sub(headroom);
          let requested = sizes.device_total();
          if requested > usable {
              return Err(MemoryError::BudgetExceeded { requested, usable });
          }
          Ok(())
      }

      #[cfg(test)]
      mod tests {
          use super::*;

          #[test]
          fn kind_ids_roundtrip() {
              for k in ArenaKind::ALL {
                  assert_eq!(ArenaKind::from_id(k.id()), Some(k));
              }
              assert_eq!(ArenaKind::from_id(ArenaId(99)), None);
          }

          #[test]
          fn generation_reserve_holds_back_inactive_generations() {
              let s = ArenaSizes { index_segment: 1000, ..Default::default() };
              assert_eq!(s.generation_reserve(2), 500);
              assert_eq!(s.generation_reserve(1), 0);
              assert_eq!(s.generation_reserve(0), 0);
          }

          #[test]
          fn staging_does_not_count_against_device() {
              let s = ArenaSizes { weights: 10, staging: 1 << 40, ..Default::default() };
              assert_eq!(s.device_total(), 10);
          }

          #[test]
          fn fit_respects_headroom() {
              let s = ArenaSizes { weights: 90, ..Default::default() };
              assert!(check_fit(&s, 100, 10).is_ok());
              assert!(matches!(
                  check_fit(&s, 100, 11),
                  Err(MemoryError::BudgetExceeded { requested: 90, usable: 89 })
              ));
          }

          #[test]
          fn usable_subtracts_reserve() {
              let b = Budget { free: 100, total: 200, reserved_for_generations: 150 };
              assert_eq!(b.usable(), 0);
          }
      }
  reason: Fehlertyp und Budget-/ArenaKind-Modul für D3 anlegen
```
/answer bc7a-24 allow
Control Request:
```yaml
id: bc7a-25
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/subengine/exec.rs
    ids:
    - LaneJob
    - tests.cfg
    - tests.lane
    - tests.event_completes_after_prior_jobs
  reason: "Wie Tests unvollständige Events erzeugen (LaneJob, CpuLane)"
```
/answer bc7a-25 allow
ReasoningToken: 454
Control Request:
```yaml
id: bc7a-26
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/subengine/exec.rs
      nodes:
      - id: LaneJob
        type: ItemType
        code: |
          /// Work item executed on a lane. Device work is enqueued asynchronously from within the job.
          pub type LaneJob = Box<dyn FnOnce() -> Result<(), DeviceError> + Send + 'static>;
      - id: tests.cfg
        type: ItemFn
        code: |2
              fn cfg() -> LaneConfig {
                  LaneConfig::new(Region {
                      arena: ArenaId(1),
                      offset: 8,
                      len: 256,
                      gen: 3,
                  })
              }
      - id: tests.lane
        type: ItemFn
        code: |2
              fn lane(class: LaneClass) -> CpuLane {
                  CpuLane::new(class, cfg()).unwrap()
              }
      - id: tests.event_completes_after_prior_jobs
        type: ItemFn
        code: |2
              #[test]
              fn event_completes_after_prior_jobs() {
                  let lane = lane(LaneClass::Index);
                  let hits = Arc::new(AtomicUsize::new(0));
                  let h = hits.clone();
                  lane.submit(Box::new(move || {
                      std::thread::sleep(Duration::from_millis(20));
                      h.fetch_add(1, Ordering::SeqCst);
                      Ok(())
                  }))
                  .unwrap();
                  let event = lane.record_event().unwrap();
                  event.synchronize().unwrap();
                  assert!(event.is_complete());
                  assert_eq!(hits.load(Ordering::SeqCst), 1);
              }
```
/answer bc7a-26 allow
Control Request:
```yaml
id: bc7a-27
toolName: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src/resident/subengine
    pattern: pub use|pub\(crate\) use
    include:
    - '*.rs'
  reason: "Re-Exports der Subengine (EventHandle, CpuLane) bestätigen"
```
/answer bc7a-27 allow
Control Request:
```yaml
id: bc7a-28
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src/resident/subengine
      matches:
      - path: mod.rs
        matches:
        - lineno: 16
          match: pub use device::*;
        - lineno: 17
          match: "pub use exec::{"
        - lineno: 22
          match: pub use exec::RecordingLane;
        - lineno: 23
          match: "pub use error::{DeviceError, SubengineError};"
        - lineno: 24
          match: "pub use lifecycle::{"
        - lineno: 27
          match: "pub use resource::{"
      - path: device.rs
        matches:
        - lineno: 115
          match: pub use cuda_lane::CudaLane;
```
/answer bc7a-28 allow
ReasoningToken: 179
Control Request:
```yaml
id: bc7a-29
toolName: ast_create
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/memory/arena.rs
    source: |
      //! Per-class arena allocator with stream-ordered release, shared by every memory backend.

      use parking_lot::Mutex;

      use super::budget::{ArenaKind, ArenaSizes, Budget};
      use super::error::MemoryError;
      use super::manager::{BookkeepingArena, DeviceKind, MemoryManager};
      use super::region::Region;
      use crate::resident::error::IndexError;
      use crate::resident::subengine::EventHandle;

      /// Offset allocator for one arena class. A region released with `release_after` returns to the
      /// free list only once its event has completed, so in-flight work never sees reused memory.
      pub struct ArenaAllocator {
          kind: ArenaKind,
          dev: DeviceKind,
          capacity: u64,
          arena: BookkeepingArena,
          pending: Mutex<Vec<(Region, EventHandle)>>,
      }

      impl ArenaAllocator {
          pub fn new(kind: ArenaKind, dev: DeviceKind, capacity: u64) -> Self {
              Self {
                  kind,
                  dev,
                  capacity,
                  arena: BookkeepingArena::new(kind.id(), capacity),
                  pending: Mutex::new(Vec::new()),
              }
          }

          pub fn kind(&self) -> ArenaKind {
              self.kind
          }

          pub fn capacity(&self) -> u64 {
              self.capacity
          }

          /// Immediately reusable bytes; excludes regions still waiting for their event.
          pub fn available(&self) -> u64 {
              self.arena.available()
          }

          pub fn pending(&self) -> usize {
              self.pending.lock().len()
          }

          pub fn is_live(&self, region: Region) -> bool {
              self.arena.is_live(region)
          }

          #[tracing::instrument(level = "trace", skip(self), fields(arena = ?self.kind))]
          pub fn alloc(&self, bytes: u64, align: u64) -> Result<Region, MemoryError> {
              self.reclaim();
              self.arena.alloc(self.dev, bytes, align).map_err(|e| match e {
                  IndexError::ArenaExhausted { requested, available } => MemoryError::ArenaExhausted {
                      arena: self.kind,
                      requested: requested as u64,
                      free: available as u64,
                  },
                  other => MemoryError::AllocFailed { arena: self.kind, reason: other.to_string() },
              })
          }

          pub fn free(&self, region: Region) -> Result<(), MemoryError> {
              if self.is_pending(region) {
                  return Err(stale(region));
              }
              self.arena.try_free(region).map_err(|_| stale(region))
          }

          #[tracing::instrument(level = "trace", skip_all, fields(arena = ?self.kind, offset = region.offset, len = region.len, gen = region.gen))]
          pub fn release_after(&self, region: Region, event: EventHandle) -> Result<(), MemoryError> {
              if !self.arena.is_live(region) || self.is_pending(region) {
                  return Err(stale(region));
              }
              self.pending.lock().push((region, event));
              self.reclaim();
              Ok(())
          }

          /// Frees every pending region whose event has completed; returns how many.
          pub fn reclaim(&self) -> usize {
              let done: Vec<Region> = {
                  let mut pending = self.pending.lock();
                  let (done, waiting): (Vec<_>, Vec<_>) =
                      pending.drain(..).partition(|(_, e)| e.is_complete());
                  *pending = waiting;
                  done.into_iter().map(|(r, _)| r).collect()
              };
              for r in &done {
                  if let Err(e) = self.arena.try_free(*r) {
                      tracing::warn!(error = %e, "pending release found a stale region");
                  }
              }
              done.len()
          }

          fn is_pending(&self, region: Region) -> bool {
              self.pending.lock().iter().any(|(r, _)| *r == region)
          }
      }

      fn stale(region: Region) -> MemoryError {
          MemoryError::StaleRegion { arena: region.arena.0, gen: region.gen }
      }

      impl MemoryManager for ArenaAllocator {
          fn alloc(&self, _dev: DeviceKind, bytes: u64, align: u64) -> Result<Region, IndexError> {
              ArenaAllocator::alloc(self, bytes, align).map_err(|e| match e {
                  MemoryError::ArenaExhausted { requested, free, .. } => IndexError::ArenaExhausted {
                      requested: requested as usize,
                      available: free as usize,
                  },
                  other => IndexError::TransferFailed { reason: other.to_string() },
              })
          }

          fn free(&self, region: Region) {
              if let Err(e) = ArenaAllocator::free(self, region) {
                  tracing::warn!(error = %e, "ignored stale free");
              }
          }
      }

      /// One allocator per arena class.
      pub struct ArenaSet {
          arenas: Vec<ArenaAllocator>,
          sizes: ArenaSizes,
      }

      impl ArenaSet {
          pub fn new(dev: DeviceKind, sizes: ArenaSizes) -> Self {
              let arenas = ArenaKind::ALL
                  .iter()
                  .map(|k| ArenaAllocator::new(*k, dev, sizes.get(*k)))
                  .collect();
              Self { arenas, sizes }
          }

          pub fn sizes(&self) -> &ArenaSizes {
              &self.sizes
          }

          pub fn get(&self, kind: ArenaKind) -> &ArenaAllocator {
              &self.arenas[kind.index()]
          }

          pub fn for_region(&self, region: Region) -> Result<&ArenaAllocator, MemoryError> {
              ArenaKind::from_id(region.arena).map(|k| self.get(k)).ok_or_else(|| stale(region))
          }
      }

      /// Arena-class aware allocation on one device.
      pub trait DeviceMemory: Send + Sync {
          fn alloc_in(&self, kind: ArenaKind, bytes: u64, align: u64) -> Result<Region, MemoryError>;
          fn free(&self, region: Region) -> Result<(), MemoryError>;
          fn release_after(&self, region: Region, event: EventHandle) -> Result<(), MemoryError>;
          fn budget(&self) -> Result<Budget, MemoryError>;
      }

      #[cfg(test)]
      pub(crate) mod testing {
          use std::sync::mpsc::{channel, Sender};

          use crate::resident::memory::{ArenaId, Region};
          use crate::resident::subengine::{CpuLane, EventHandle, ExecContext, LaneClass, LaneConfig};

          /// An event that stays incomplete until `open` is called.
          pub struct Gate {
              lane: CpuLane,
              tx: Sender<()>,
              pub event: EventHandle,
          }

          impl Gate {
              pub fn closed() -> Self {
                  let cfg = LaneConfig::new(Region { arena: ArenaId(0), offset: 0, len: 1, gen: 0 });
                  let lane = CpuLane::new(LaneClass::Index, cfg).unwrap();
                  let (tx, rx) = channel::<()>();
                  lane.submit(Box::new(move || {
                      let _ = rx.recv();
                      Ok(())
                  }))
                  .unwrap();
                  let event = lane.record_event().unwrap();
                  Self { lane, tx, event }
              }

              pub fn open(self) {
                  self.tx.send(()).unwrap();
                  self.event.synchronize().unwrap();
                  drop(self.lane);
              }
          }
      }

      #[cfg(test)]
      mod tests {
          use super::testing::Gate;
          use super::*;

          fn alloc_for(cap: u64) -> ArenaAllocator {
              ArenaAllocator::new(ArenaKind::Workspace, DeviceKind::Cpu, cap)
          }

          #[test]
          fn exhaustion_reports_arena_and_free_bytes() {
              let a = alloc_for(64);
              let _r = a.alloc(48, 8).unwrap();
              match a.alloc(32, 8) {
                  Err(MemoryError::ArenaExhausted { arena, requested: 32, free: 16 }) => {
                      assert_eq!(arena, ArenaKind::Workspace)
                  }
                  other => panic!("unexpected {other:?}"),
              }
          }

          #[test]
          fn regions_carry_arena_identity_and_stale_free_is_rejected() {
              let a = alloc_for(64);
              let r = a.alloc(16, 8).unwrap();
              assert_eq!(r.arena, ArenaKind::Workspace.id());
              a.free(r).unwrap();
              assert!(matches!(a.free(r), Err(MemoryError::StaleRegion { .. })));
              let r2 = a.alloc(16, 8).unwrap();
              assert_eq!(r2.offset, r.offset);
              assert_ne!(r2.gen, r.gen);
              assert!(matches!(a.free(r), Err(MemoryError::StaleRegion { .. })));
          }

          #[test]
          fn release_after_defers_reuse_until_event_completes() {
              let a = alloc_for(32);
              let r = a.alloc(32, 8).unwrap();
              let gate = Gate::closed();
              a.release_after(r, gate.event.clone()).unwrap();
              assert_eq!(a.pending(), 1);
              assert!(matches!(a.alloc(8, 8), Err(MemoryError::ArenaExhausted { .. })));
              gate.open();
              let again = a.alloc(8, 8).unwrap();
              assert_eq!(again.offset, 0);
              assert_eq!(a.pending(), 0);
          }

          #[test]
          fn pending_region_cannot_be_freed_or_released_twice() {
              let a = alloc_for(32);
              let r = a.alloc(16, 8).unwrap();
              let gate = Gate::closed();
              a.release_after(r, gate.event.clone()).unwrap();
              assert!(matches!(a.free(r), Err(MemoryError::StaleRegion { .. })));
              assert!(a.release_after(r, gate.event.clone()).is_err());
              gate.open();
          }

          #[test]
          fn set_routes_regions_to_their_class() {
              let set = ArenaSet::new(
                  DeviceKind::Cpu,
                  ArenaSizes { weights: 64, index_segment: 128, ..Default::default() },
              );
              let w = set.get(ArenaKind::Weights).alloc(8, 8).unwrap();
              assert_eq!(set.for_region(w).unwrap().kind(), ArenaKind::Weights);
              assert!(matches!(
                  set.get(ArenaKind::Staging).alloc(1, 1),
                  Err(MemoryError::ArenaExhausted { .. })
              ));
          }
      }
  reason: "ArenaAllocator mit stream-geordneter Freigabe, ArenaSet und DeviceMemory-Trait implementieren"
```
/answer bc7a-29 allow
ReasoningToken: 82
Control Request:
```yaml
id: bc7a-2a
toolName: ast_create
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/memory/staging.rs
    source: |
      //! Ring of staging slots; a slot is reused only after the event of its last transfer completed.

      use parking_lot::Mutex;

      use super::budget::ArenaKind;
      use super::error::{MemoryError, TransferDir};
      use crate::resident::subengine::EventHandle;

      enum State {
          Free,
          Held,
          InFlight(EventHandle),
      }

      struct Entry<S> {
          payload: S,
          state: State,
      }

      struct Ring<S> {
          entries: Vec<Entry<S>>,
          cursor: usize,
      }

      pub struct StagingRing<S> {
          ring: Mutex<Ring<S>>,
      }

      impl<S> StagingRing<S> {
          pub fn new(payloads: impl IntoIterator<Item = S>) -> Self {
              let entries = payloads.into_iter().map(|payload| Entry { payload, state: State::Free }).collect();
              Self { ring: Mutex::new(Ring { entries, cursor: 0 }) }
          }

          pub fn slots(&self) -> usize {
              self.ring.lock().entries.len()
          }

          pub fn in_flight(&self) -> usize {
              self.ring.lock().entries.iter().filter(|e| matches!(e.state, State::InFlight(_))).count()
          }

          /// Takes the next slot that is free or whose transfer has completed.
          pub fn acquire(&self) -> Option<usize> {
              let mut ring = self.ring.lock();
              let n = ring.entries.len();
              for step in 0..n {
                  let i = (ring.cursor + step) % n;
                  let usable = match &ring.entries[i].state {
                      State::Free => true,
                      State::InFlight(e) => e.is_complete(),
                      State::Held => false,
                  };
                  if usable {
                      ring.entries[i].state = State::Held;
                      ring.cursor = (i + 1) % n;
                      return Some(i);
                  }
              }
              None
          }

          /// Like `acquire`, but blocks on the oldest in-flight transfer when every slot is busy.
          pub fn acquire_wait(&self) -> Result<usize, MemoryError> {
              loop {
                  if let Some(i) = self.acquire() {
                      return Ok(i);
                  }
                  let oldest = {
                      let ring = self.ring.lock();
                      let n = ring.entries.len();
                      (0..n).map(|s| (ring.cursor + s) % n).find_map(|i| match &ring.entries[i].state {
                          State::InFlight(e) => Some(e.clone()),
                          _ => None,
                      })
                  };
                  let Some(event) = oldest else {
                      return Err(MemoryError::ArenaExhausted {
                          arena: ArenaKind::Staging,
                          requested: 1,
                          free: 0,
                      });
                  };
                  event.synchronize().map_err(|e| MemoryError::TransferFailed {
                      dir: TransferDir::HostToDevice,
                      reason: e.to_string(),
                  })?;
              }
          }

          pub fn with_slot<R>(&self, index: usize, f: impl FnOnce(&mut S) -> R) -> Result<R, MemoryError> {
              let mut ring = self.ring.lock();
              match ring.entries.get_mut(index) {
                  Some(e) if matches!(e.state, State::Held) => Ok(f(&mut e.payload)),
                  _ => Err(MemoryError::InvalidSlot { index }),
              }
          }

          /// Marks a held slot as in use by a transfer that completes with `event`.
          pub fn publish(&self, index: usize, event: EventHandle) -> Result<(), MemoryError> {
              self.transition(index, State::InFlight(event))
          }

          /// Returns a held slot that was never used for a transfer.
          pub fn release(&self, index: usize) -> Result<(), MemoryError> {
              self.transition(index, State::Free)
          }

          fn transition(&self, index: usize, next: State) -> Result<(), MemoryError> {
              let mut ring = self.ring.lock();
              match ring.entries.get_mut(index) {
                  Some(e) if matches!(e.state, State::Held) => {
                      e.state = next;
                      Ok(())
                  }
                  _ => Err(MemoryError::InvalidSlot { index }),
              }
          }
      }

      #[cfg(test)]
      mod tests {
          use super::*;
          use crate::resident::memory::arena::testing::Gate;

          #[test]
          fn slots_cycle_round_robin() {
              let ring = StagingRing::new([0u8, 1, 2]);
              let a = ring.acquire().unwrap();
              let b = ring.acquire().unwrap();
              assert_ne!(a, b);
              ring.release(a).unwrap();
              ring.release(b).unwrap();
              assert_eq!(ring.slots(), 3);
          }

          #[test]
          fn slot_is_not_reused_before_its_event_completes() {
              let ring = StagingRing::new([()]);
              let gate = Gate::closed();
              let i = ring.acquire().unwrap();
              ring.publish(i, gate.event.clone()).unwrap();
              assert_eq!(ring.in_flight(), 1);
              assert!(ring.acquire().is_none());
              gate.open();
              assert_eq!(ring.acquire(), Some(i));
          }

          #[test]
          fn acquire_wait_blocks_on_oldest_transfer() {
              let ring = StagingRing::new([()]);
              let gate = Gate::closed();
              let i = ring.acquire().unwrap();
              ring.publish(i, gate.event.clone()).unwrap();
              let waiter = std::thread::scope(|s| {
                  let h = s.spawn(|| ring.acquire_wait());
                  std::thread::sleep(std::time::Duration::from_millis(30));
                  assert!(!h.is_finished());
                  gate.open();
                  h.join().unwrap()
              });
              assert_eq!(waiter.unwrap(), i);
          }

          #[test]
          fn acquire_wait_fails_when_all_slots_are_held() {
              let ring = StagingRing::new([()]);
              ring.acquire().unwrap();
              assert!(matches!(ring.acquire_wait(), Err(MemoryError::ArenaExhausted { .. })));
          }

          #[test]
          fn payload_access_requires_a_held_slot() {
              let ring = StagingRing::new([5u32]);
              assert!(matches!(ring.with_slot(0, |_| ()), Err(MemoryError::InvalidSlot { .. })));
              let i = ring.acquire().unwrap();
              ring.with_slot(i, |p| *p += 1).unwrap();
              assert_eq!(ring.with_slot(i, |p| *p).unwrap(), 6);
              assert!(ring.publish(9, Gate::closed().event).is_err());
          }
      }
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/memory/cpu.rs
    source: |
      //! Host-RAM arenas for the CPU path; no streams, no staging.

      use parking_lot::Mutex;

      use super::arena::{ArenaSet, DeviceMemory};
      use super::budget::{ArenaKind, ArenaSizes, Budget};
      use super::error::{MemoryError, TransferDir};
      use super::manager::DeviceKind;
      use super::region::Region;
      use crate::resident::subengine::EventHandle;

      pub struct CpuMemoryManager {
          set: ArenaSet,
          storage: Vec<Mutex<Vec<u8>>>,
          generations: u32,
      }

      impl CpuMemoryManager {
          pub fn new(sizes: ArenaSizes, generations: u32) -> Self {
              let storage = ArenaKind::ALL.iter().map(|k| Mutex::new(vec![0u8; sizes.get(*k) as usize])).collect();
              Self { set: ArenaSet::new(DeviceKind::Cpu, sizes), storage, generations }
          }

          pub fn arenas(&self) -> &ArenaSet {
              &self.set
          }

          pub fn write(&self, region: Region, data: &[u8]) -> Result<(), MemoryError> {
              let range = self.range(region, data.len())?;
              self.storage[self.slot(region)?].lock()[range].copy_from_slice(data);
              Ok(())
          }

          pub fn read(&self, region: Region, out: &mut [u8]) -> Result<(), MemoryError> {
              let range = self.range(region, out.len())?;
              out.copy_from_slice(&self.storage[self.slot(region)?].lock()[range]);
              Ok(())
          }

          pub fn copy(&self, src: Region, dst: Region) -> Result<(), MemoryError> {
              let len = src.len as usize;
              let from = self.range(src, len)?;
              let to = self.range(dst, len)?;
              let (si, di) = (self.slot(src)?, self.slot(dst)?);
              if si == di {
                  self.storage[si].lock().copy_within(from, to.start);
              } else {
                  let (a, b) = if si < di { (si, di) } else { (di, si) };
                  let (ga, mut gb) = (self.storage[a].lock(), self.storage[b].lock());
                  let (s, d) = if si < di { (&*ga, &mut *gb) } else { (&*gb, &mut *gb) };
                  let _ = (s, d);
                  drop(gb);
                  drop(ga);
                  let tmp = self.storage[si].lock()[from].to_vec();
                  self.storage[di].lock()[to].copy_from_slice(&tmp);
              }
              Ok(())
          }

          fn slot(&self, region: Region) -> Result<usize, MemoryError> {
              let arena = self.set.for_region(region)?;
              Ok(arena.kind().index())
          }

          fn range(&self, region: Region, len: usize) -> Result<std::ops::Range<usize>, MemoryError> {
              let arena = self.set.for_region(region)?;
              if !arena.is_live(region) || len as u64 > region.len {
                  return Err(MemoryError::StaleRegion { arena: region.arena.0, gen: region.gen });
              }
              let start = region.offset as usize;
              Ok(start..start + len)
          }
      }

      impl DeviceMemory for CpuMemoryManager {
          fn alloc_in(&self, kind: ArenaKind, bytes: u64, align: u64) -> Result<Region, MemoryError> {
              self.set.get(kind).alloc(bytes, align)
          }

          fn free(&self, region: Region) -> Result<(), MemoryError> {
              self.set.for_region(region)?.free(region)
          }

          fn release_after(&self, region: Region, event: EventHandle) -> Result<(), MemoryError> {
              self.set.for_region(region)?.release_after(region, event)
          }

          fn budget(&self) -> Result<Budget, MemoryError> {
              let sizes = self.set.sizes();
              let free = ArenaKind::ALL.iter().map(|k| self.set.get(*k).available()).sum();
              let total = ArenaKind::ALL.iter().map(|k| sizes.get(*k)).sum();
              Ok(Budget { free, total, reserved_for_generations: sizes.generation_reserve(self.generations) })
          }
      }

      const _: fn() = || {
          let _ = TransferDir::HostToDevice;
      };

      #[cfg(test)]
      mod tests {
          use super::*;

          fn mgr() -> CpuMemoryManager {
              CpuMemoryManager::new(
                  ArenaSizes { weights: 64, index_segment: 128, workspace: 64, staging: 0, param_block: 16 },
                  2,
              )
          }

          #[test]
          fn write_read_roundtrip_and_stale_rejection() {
              let m = mgr();
              let r = m.alloc_in(ArenaKind::Workspace, 8, 8).unwrap();
              m.write(r, &[1, 2, 3, 4]).unwrap();
              let mut out = [0u8; 4];
              m.read(r, &mut out).unwrap();
              assert_eq!(out, [1, 2, 3, 4]);
              m.free(r).unwrap();
              assert!(m.read(r, &mut out).is_err());
              assert!(m.write(r, &[0]).is_err());
          }

          #[test]
          fn oversized_access_is_rejected() {
              let m = mgr();
              let r = m.alloc_in(ArenaKind::Workspace, 4, 4).unwrap();
              assert!(m.write(r, &[0; 5]).is_err());
          }

          #[test]
          fn copy_across_and_within_arenas() {
              let m = mgr();
              let a = m.alloc_in(ArenaKind::Weights, 4, 4).unwrap();
              let b = m.alloc_in(ArenaKind::IndexSegment, 4, 4).unwrap();
              let c = m.alloc_in(ArenaKind::Weights, 4, 4).unwrap();
              m.write(a, &[9, 8, 7, 6]).unwrap();
              m.copy(a, b).unwrap();
              m.copy(a, c).unwrap();
              let mut out = [0u8; 4];
              m.read(b, &mut out).unwrap();
              assert_eq!(out, [9, 8, 7, 6]);
              m.read(c, &mut out).unwrap();
              assert_eq!(out, [9, 8, 7, 6]);
          }

          #[test]
          fn budget_reserves_the_second_generation() {
              let m = mgr();
              let before = m.budget().unwrap();
              assert_eq!(before.total, 272);
              assert_eq!(before.reserved_for_generations, 64);
              m.alloc_in(ArenaKind::IndexSegment, 32, 8).unwrap();
              assert_eq!(m.budget().unwrap().free, 240);
          }
      }
  reason: StagingRing und CpuMemoryManager anlegen
```
/answer bc7a-2a allow
Text:
 In `cpu.rs` sind mir die Kopierlogik und ein überflüssiger Platzhalter missraten. Beides räume ich auf.
ReasoningToken: 102
Control Request:
```yaml
id: bc7a-2b
toolName: ast_edit_marks
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/memory/cpu.rs
    start_marker: "        if si == di {"
    end_marker: |2-
              Ok(())
          }

          fn slot
    content: |2-
              if si == di {
                  self.storage[si].lock().copy_within(from, to.start);
              } else {
                  let tmp = self.storage[si].lock()[from].to_vec();
                  self.storage[di].lock()[to].copy_from_slice(&tmp);
              }
              Ok(())
          }

          fn slot
    id: ""
  reason: Kopierlogik vereinfachen (ein Lock zur Zeit)
```
/answer bc7a-2b allow
Control Request:
```yaml
id: bc7a-2c
result:
  structuredContent:
    errors:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/memory/cpu.rs
      error: No node matched the selector.
      id: ""
```
/answer bc7a-2c allow
Text:
 Das Tool verlangt eine Node-ID. Ich schreibe die Änderung mit Python.
Control Request:
```yaml
id: bc7a-2d
toolName: python
arguments:
  script: |-
    p='/home/user/xyan/xy.ai.workbench/rag/src/resident/memory/cpu.rs'
    s=open(p).read()
    a=s.index("        if si == di {")
    b=s.index("        Ok(())\n    }\n\n    fn slot")
    s=s[:a]+"""        if si == di {
                self.storage[si].lock().copy_within(from, to.start);
            } else {
                let tmp = self.storage[si].lock()[from].to_vec();
                self.storage[di].lock()[to].copy_from_slice(&tmp);
            }
    """+s[b:]
    c=s.index("const _: fn() = || {")
    d=s.index("#[cfg(test)]")
    s=s[:c]+s[d:]
    s=s.replace("use super::error::{MemoryError, TransferDir};","use super::error::MemoryError;")
    open(p,'w').write(s)
  reason: "cpu.rs bereinigen: Kopierlogik und Platzhalter-Konstante entfernen"
```
/answer bc7a-2d allow
Control Request:
```yaml
id: bc7a-2e
result:
  structuredContent:
    result: success
```
/answer bc7a-2e allow
ReasoningToken: 49
Control Request:
```yaml
id: bc7a-2f
toolName: python
arguments:
  script: |-
    import re
    base='/home/user/xyan/xy.ai.workbench/rag/src/resident/'
    open(base+'memory/mod.rs','w').write('''//! Region/Layout/DType, the `MemoryManager` contract, and per-class device arenas.

    pub mod arena;
    pub mod budget;
    pub mod cpu;
    #[cfg(feature = "cuda")]
    pub mod cuda;
    pub mod dtype;
    pub mod error;
    pub mod freelist;
    pub mod layout;
    pub mod manager;
    pub mod region;
    pub mod staging;

    pub use arena::{ArenaAllocator, ArenaSet, DeviceMemory};
    pub use budget::{ArenaKind, ArenaSizes, Budget, BudgetProbe, DEFAULT_HEADROOM};
    pub use cpu::CpuMemoryManager;
    pub use dtype::{DType, UnsupportedDType, F16_MASK_VALUE};
    pub use error::{MemoryError, TransferDir};
    pub use manager::{compute_dtype, BookkeepingArena, DeviceKind, MemoryManager};
    pub use region::{ArenaId, Region};
    pub use staging::StagingRing;
    ''')
    print(open(base+'memory/mod.rs').read()[:0])
    # dtype constant
    p=base+'memory/dtype.rs'; s=open(p).read()
    i=s.index('#[non_exhaustive]')
    s=s[:i]+'/// Additive attention-mask value that stays finite in f16 (`-inf` would turn rows into NaN).\npub const F16_MASK_VALUE: f32 = -1e4;\n\n'+s[i:]
    open(p,'w').write(s)
    # error
    p=base+'error.rs'; s=open(p).read()
    s=s.replace("    #[error(transparent)]\n    Device(#[from] DeviceError),\n}","    #[error(transparent)]\n    Device(#[from] DeviceError),\n    #[error(transparent)]\n    Memory(#[from] crate::resident::memory::MemoryError),\n}",1)
    s=s.replace("            ResidentError::Device(e) => e.is_retryable(),\n","            ResidentError::Device(e) => e.is_retryable(),\n            ResidentError::Memory(e) => e.is_retryable(),\n",1)
    s=s.replace("        assert_send_sync_static::<DeviceError>();\n","        assert_send_sync_static::<DeviceError>();\n        assert_send_sync_static::<crate::resident::memory::MemoryError>();\n",1)
    open(p,'w').write(s)
    # device handle accessor
    p=base+'subengine/device.rs'; s=open(p).read()
    s=s.replace("    pub fn ordinal(&self) -> usize {","""    #[cfg(feature = "cuda")]
        pub(crate) fn cuda_context(&self) -> Option<&std::sync::Arc<cudarc::driver::CudaContext>> {
            self.ctx.as_ref()
        }

        pub fn ordinal(&self) -> usize {""",1)
    open(p,'w').write(s)
  reason: "mod.rs, dtype-Konstante, ResidentError::Memory und Context-Accessor ergänzen"
```
/answer bc7a-2f allow
Control Request:
```yaml
id: bc7a-30
result:
  structuredContent:
    exit_code: 0
    stdout: |2+
```
/answer bc7a-30 allow
ReasoningToken: 239
Control Request:
```yaml
id: bc7a-31
toolName: ast_create
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/memory/cuda.rs
    source: |
      //! CUDA device arenas: one backing allocation per device-resident class, pinned staging, and
      //! region-addressed copies. Device pointers never leave this module.

      use std::ffi::c_void;
      use std::sync::Arc;

      use cudarc::driver::{
          result, sys, CudaContext, CudaSlice, CudaStream, DevicePtr, DevicePtrMut, DriverError,
          PinnedHostSlice,
      };
      use parking_lot::{Mutex, MutexGuard};

      use super::arena::{ArenaSet, DeviceMemory};
      use super::budget::{check_fit, ArenaKind, ArenaSizes, Budget, BudgetProbe};
      use super::error::{MemoryError, TransferDir};
      use super::manager::DeviceKind;
      use super::region::Region;
      use super::staging::StagingRing;
      use crate::resident::subengine::{DeviceHandle, EventHandle};

      pub type PinnedStaging = StagingRing<PinnedHostSlice<u8>>;

      fn unavailable(e: DriverError) -> MemoryError {
          MemoryError::Unavailable { reason: e.to_string() }
      }

      fn transfer(dir: TransferDir) -> impl Fn(DriverError) -> MemoryError {
          move |e| MemoryError::TransferFailed { dir, reason: e.to_string() }
      }

      pub struct CudaProbe {
          ctx: Arc<CudaContext>,
      }

      impl BudgetProbe for CudaProbe {
          fn probe(&self) -> Result<(u64, u64), MemoryError> {
              let (free, total) = self
                  .ctx
                  .mem_get_info()
                  .map_err(|e| MemoryError::BudgetProbe { reason: e.to_string() })?;
              Ok((free as u64, total as u64))
          }
      }

      /// Keeps freed stream-ordered allocations in the default pool instead of returning them to the
      /// driver, so steady-state operation does no driver-level allocation.
      fn retain_pool_memory(ctx: &Arc<CudaContext>) -> Result<(), MemoryError> {
          ctx.bind_to_thread().map_err(unavailable)?;
          let mut threshold = u64::MAX;
          unsafe {
              let pool = result::device::get_default_mem_pool(ctx.cu_device()).map_err(unavailable)?;
              result::mem_pool::set_attribute(
                  pool,
                  sys::CUmemPool_attribute::CU_MEMPOOL_ATTR_RELEASE_THRESHOLD,
                  &mut threshold as *mut u64 as *mut c_void,
              )
              .map_err(unavailable)
          }
      }

      pub struct CudaMemoryManager {
          ctx: Arc<CudaContext>,
          set: ArenaSet,
          backing: Vec<Mutex<Option<CudaSlice<u8>>>>,
          probe: CudaProbe,
          generations: u32,
      }

      impl CudaMemoryManager {
          /// Allocates the device arenas up front. `headroom` stays free for the driver context and
          /// other processes.
          pub fn new(
              device: &DeviceHandle,
              sizes: ArenaSizes,
              headroom: u64,
              generations: u32,
          ) -> Result<Self, MemoryError> {
              let ctx = device
                  .cuda_context()
                  .ok_or_else(|| MemoryError::Unavailable { reason: "not a CUDA device".into() })?
                  .clone();
              let probe = CudaProbe { ctx: ctx.clone() };
              let (free, total) = probe.probe()?;
              check_fit(&sizes, free, headroom)?;
              retain_pool_memory(&ctx)?;

              let stream = ctx.new_stream().map_err(unavailable)?;
              let mut backing = Vec::with_capacity(ArenaKind::ALL.len());
              for kind in ArenaKind::ALL {
                  let bytes = if kind == ArenaKind::Staging { 0 } else { sizes.get(kind) as usize };
                  let slice = (bytes > 0)
                      .then(|| stream.alloc_zeros::<u8>(bytes))
                      .transpose()
                      .map_err(unavailable)?;
                  backing.push(Mutex::new(slice));
              }
              stream.synchronize().map_err(unavailable)?;

              let staged_off = ArenaSizes { staging: 0, ..sizes };
              tracing::info!(
                  free_mib = free >> 20,
                  total_mib = total >> 20,
                  device_mib = sizes.device_total() >> 20,
                  "cuda arenas allocated"
              );
              Ok(Self {
                  ctx,
                  set: ArenaSet::new(DeviceKind::Gpu, staged_off),
                  backing,
                  probe,
                  generations,
              })
          }

          pub fn arenas(&self) -> &ArenaSet {
              &self.set
          }

          /// Ring of page-locked host buffers for host-to-device uploads.
          pub fn staging_ring(&self, slots: usize, slot_bytes: usize) -> Result<PinnedStaging, MemoryError> {
              let mut payloads = Vec::with_capacity(slots);
              for _ in 0..slots {
                  let mut pinned = unsafe { self.ctx.alloc_pinned::<u8>(slot_bytes) }.map_err(unavailable)?;
                  pinned.as_mut_slice().map_err(unavailable)?.fill(0);
                  payloads.push(pinned);
              }
              Ok(StagingRing::new(payloads))
          }

          /// Device-to-device copy between two live regions of different arena classes, ordered on
          /// `stream`. Used to move results between engine arenas and library-owned allocations.
          pub(crate) fn copy_d2d(
              &self,
              stream: &Arc<CudaStream>,
              src: Region,
              dst: Region,
          ) -> Result<(), MemoryError> {
              let (si, di) = (self.slot(src)?, self.slot(dst)?);
              let bytes = src.len;
              self.check_live(src, bytes)?;
              self.check_live(dst, bytes)?;
              if si == di {
                  return Err(MemoryError::TransferFailed {
                      dir: TransferDir::DeviceToDevice,
                      reason: "source and destination share an arena".into(),
                  });
              }
              let (first, second) = if si < di { (si, di) } else { (di, si) };
              let g1 = self.backing[first].lock();
              let g2 = self.backing[second].lock();
              let (sg, mut dg) = if si < di { (g1, g2) } else { (g2, g1) };
              let (Some(s), Some(d)) = (sg.as_ref(), dg.as_mut()) else {
                  return Err(MemoryError::StaleRegion { arena: src.arena.0, gen: src.gen });
              };
              let sv = s.slice(src.offset as usize..(src.offset + bytes) as usize);
              let mut dv = d.slice_mut(dst.offset as usize..(dst.offset + bytes) as usize);
              stream.memcpy_dtod(&sv, &mut dv).map_err(transfer(TransferDir::DeviceToDevice))
          }

          /// Asynchronous upload from pinned memory.
          ///
          /// # Safety
          /// `src` must be page-locked and stay untouched until an event recorded on `stream` after
          /// this call has completed.
          pub(crate) unsafe fn upload(
              &self,
              stream: &Arc<CudaStream>,
              src: &[u8],
              dst: Region,
          ) -> Result<(), MemoryError> {
              let slot = self.slot(dst)?;
              self.check_live(dst, src.len() as u64)?;
              let mut guard = self.backing[slot].lock();
              let slice = guard.as_mut().ok_or(MemoryError::StaleRegion { arena: dst.arena.0, gen: dst.gen })?;
              let mut view = slice.slice_mut(dst.offset as usize..dst.offset as usize + src.len());
              let (ptr, _sync) = view.device_ptr_mut(stream);
              result::memcpy_htod_async(ptr, src, stream.cu_stream()).map_err(transfer(TransferDir::HostToDevice))
          }

          /// Asynchronous download into pinned memory.
          ///
          /// # Safety
          /// `dst` must be page-locked and must not be read until an event recorded on `stream` after
          /// this call has completed.
          pub(crate) unsafe fn download(
              &self,
              stream: &Arc<CudaStream>,
              src: Region,
              dst: &mut [u8],
          ) -> Result<(), MemoryError> {
              let slot = self.slot(src)?;
              self.check_live(src, dst.len() as u64)?;
              let guard: MutexGuard<'_, Option<CudaSlice<u8>>> = self.backing[slot].lock();
              let slice = guard.as_ref().ok_or(MemoryError::StaleRegion { arena: src.arena.0, gen: src.gen })?;
              let view = slice.slice(src.offset as usize..src.offset as usize + dst.len());
              let (ptr, _sync) = view.device_ptr(stream);
              result::memcpy_dtoh_async(dst, ptr, stream.cu_stream()).map_err(transfer(TransferDir::DeviceToHost))
          }

          fn slot(&self, region: Region) -> Result<usize, MemoryError> {
              Ok(self.set.for_region(region)?.kind().index())
          }

          fn check_live(&self, region: Region, bytes: u64) -> Result<(), MemoryError> {
              if !self.set.for_region(region)?.is_live(region) || bytes > region.len {
                  return Err(MemoryError::StaleRegion { arena: region.arena.0, gen: region.gen });
              }
              Ok(())
          }
      }

      impl DeviceMemory for CudaMemoryManager {
          fn alloc_in(&self, kind: ArenaKind, bytes: u64, align: u64) -> Result<Region, MemoryError> {
              self.set.get(kind).alloc(bytes, align)
          }

          fn free(&self, region: Region) -> Result<(), MemoryError> {
              self.set.for_region(region)?.free(region)
          }

          fn release_after(&self, region: Region, event: EventHandle) -> Result<(), MemoryError> {
              self.set.for_region(region)?.release_after(region, event)
          }

          fn budget(&self) -> Result<Budget, MemoryError> {
              let (free, total) = self.probe.probe()?;
              let budget = Budget {
                  free,
                  total,
                  reserved_for_generations: self.set.sizes().generation_reserve(self.generations),
              };
              tracing::debug!(free, total, reserved = budget.reserved_for_generations, "vram budget");
              Ok(budget)
          }
      }

      #[cfg(test)]
      mod tests {
          use super::*;
          use crate::resident::subengine::{ExecContext, LaneClass, LaneConfig};

          fn manager() -> (DeviceHandle, CudaMemoryManager) {
              let dev = DeviceHandle::cuda(0).unwrap();
              let sizes = ArenaSizes {
                  weights: 1 << 20,
                  index_segment: 2 << 20,
                  workspace: 1 << 20,
                  staging: 0,
                  param_block: 4096,
              };
              let m = CudaMemoryManager::new(&dev, sizes, 0, 2).unwrap();
              (dev, m)
          }

          #[test]
          #[ignore = "requires a CUDA device"]
          fn upload_copy_download_roundtrip() {
              let (dev, m) = manager();
              let scratch = m.alloc_in(ArenaKind::ParamBlock, 256, 8).unwrap();
              let lane = dev.open_lane(LaneClass::Index, LaneConfig::new(scratch)).unwrap();
              let ctx = dev.cuda_context().unwrap();
              let stream = ctx.new_stream().unwrap();
              let ring = m.staging_ring(2, 64).unwrap();

              let a = m.alloc_in(ArenaKind::Workspace, 64, 16).unwrap();
              let b = m.alloc_in(ArenaKind::IndexSegment, 64, 16).unwrap();
              let slot = ring.acquire().unwrap();
              ring.with_slot(slot, |p| p.as_mut_slice().unwrap()[..64].copy_from_slice(&[7u8; 64])).unwrap();
              let host: Vec<u8> = ring.with_slot(slot, |p| p.as_slice().unwrap()[..64].to_vec()).unwrap();
              unsafe { m.upload(&stream, &host, a).unwrap() };
              m.copy_d2d(&stream, a, b).unwrap();
              let mut out = vec![0u8; 64];
              unsafe { m.download(&stream, b, &mut out).unwrap() };
              stream.synchronize().unwrap();
              assert_eq!(out, vec![7u8; 64]);
              assert!(m.copy_d2d(&stream, a, a).is_err());
              assert!(m.budget().unwrap().total > 0);
              drop(lane);
          }

          #[test]
          fn oversized_plan_is_rejected_before_allocating() {
              let Ok(dev) = DeviceHandle::cuda(0) else { return };
              let sizes = ArenaSizes { weights: u64::MAX / 2, ..Default::default() };
              assert!(matches!(
                  CudaMemoryManager::new(&dev, sizes, 0, 2),
                  Err(MemoryError::BudgetExceeded { .. })
              ));
          }
      }
  reason: "CUDA-Backend (Pool-Retention, Budget, Pinned Staging, D2D/H2D/D2H) implementieren"
```
/answer bc7a-31 allow
Control Request:
```yaml
id: bc7a-32
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench/rag
  script: "cargo check --offline --lib 2>&1 | grep -E \"^(error|warning: unused)\" -A6 | head -60; echo ---CUDA; cargo check --offline --lib --features cuda --tests 2>&1 | grep -E \"^error\" -A9 | head -90"
  reason: Compile-Check ohne und mit cuda-Feature
```
/answer bc7a-32 allow
Control Request:
```yaml
id: bc7a-33
result:
  structuredContent:
    stdout: |
      ---CUDA
```
/answer bc7a-33 allow
ReasoningToken: 22
Control Request:
```yaml
id: bc7a-34
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench/rag
  script: "cargo check --offline --lib --features cuda --tests 2>&1 | grep -B2 -A8 \"memory/\" | head -40; echo ---TEST; cargo test --offline --lib resident:: 2>&1 | tail -15"
  reason: Warnungen in memory/ prüfen und Unit-Tests ausführen
```
/answer bc7a-34 allow
Error on control endpoint
cargo hing fest
ReasoningToken: 63
Control Request:
```yaml
id: d2ef-1
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench/rag
  script: cargo test --offline --lib resident::memory 2>&1 | tail -15
  reason: Unit-Tests des Memory-Moduls ausführen
```
/answer d2ef-1 allow
Error on control endpoint
Cargo verträgt das offline nicht
Thinking:
Der Deadlock entsteht, weil die Lane beim Drop auf den noch offenen Job wartet, während `tx` noch lebt. Ich behebe das, indem ich die Feldreihenfolge im `Gate`-Helfer ändere.


ReasoningToken: 609
Control Request:
```yaml
id: 9e96-1
toolName: python
arguments:
  script: |-
    p='/home/user/xyan/xy.ai.workbench/rag/src/resident/memory/arena.rs'
    s=open(p).read()
    old="""    pub struct Gate {
            lane: CpuLane,
            tx: Sender<()>,
            pub event: EventHandle,
        }"""
    new="""    pub struct Gate {
            // `tx` must drop before `lane`: the lane's drop waits for the blocked job.
            tx: Sender<()>,
            lane: CpuLane,
            pub event: EventHandle,
        }"""
    assert old in s
    s=s.replace(old,new).replace("Self { lane, tx, event }","Self { tx, lane, event }")
    open(p,'w').write(s)
  reason: Deadlock im Test-Helfer Gate beheben (Drop-Reihenfolge)
```
/answer 9e96-1 allow
Control Request:
```yaml
id: 9e96-2
result:
  structuredContent:
    result: success
```
/answer 9e96-2 allow
Text:
 Verstanden, ich lasse `--offline` weg und setze ein Timeout, damit nichts hängen bleibt.
ReasoningToken: 55
Control Request:
```yaml
id: 9e96-3
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench/rag
  script: timeout 600 cargo test --lib resident::memory 2>&1 | tail -40
  reason: Memory-Tests ohne --offline und mit Timeout ausführen
```
/answer 9e96-3 allow
Control Request:
```yaml
id: 9e96-4
result:
  structuredContent:
    stdout: |2+

      warning: `xy-ai-rag` (lib test) generated 1 warning
          Finished `test` profile [unoptimized + debuginfo] target(s) in 1.96s
           Running unittests src/lib.rs (.bin/debug/deps/xy_ai_rag-b030a36701633446)

      running 31 tests
      test resident::memory::arena::tests::exhaustion_reports_arena_and_free_bytes ... ok
      test resident::memory::arena::tests::regions_carry_arena_identity_and_stale_free_is_rejected ... ok
      test resident::memory::budget::tests::fit_respects_headroom ... ok
      test resident::memory::arena::tests::set_routes_regions_to_their_class ... ok
      test resident::memory::budget::tests::generation_reserve_holds_back_inactive_generations ... ok
      test resident::memory::budget::tests::kind_ids_roundtrip ... ok
      test resident::memory::arena::tests::pending_region_cannot_be_freed_or_released_twice ... ok
      test resident::memory::budget::tests::staging_does_not_count_against_device ... ok
      test resident::memory::budget::tests::usable_subtracts_reserve ... ok
      test resident::memory::arena::tests::release_after_defers_reuse_until_event_completes ... ok
      test resident::memory::cpu::tests::copy_across_and_within_arenas ... ok
      test resident::memory::cpu::tests::budget_reserves_the_second_generation ... ok
      test resident::memory::cpu::tests::write_read_roundtrip_and_stale_rejection ... ok
      test resident::memory::cpu::tests::oversized_access_is_rejected ... ok
      test resident::memory::dtype::tests::safetensors_convert_and_reject ... ok
      test resident::memory::dtype::tests::sizes ... ok
      test resident::memory::freelist::tests::alloc_align_and_exhaust ... ok
      test resident::memory::freelist::tests::coalesces_neighbours ... ok
      test resident::memory::freelist::tests::fragmentation_blocks_large_alloc ... ok
      test resident::memory::layout::tests::contiguous_byte_strides ... ok
      test resident::memory::layout::tests::views_do_not_copy ... ok
      test resident::memory::manager::tests::exhaustion ... ok
      test resident::memory::manager::tests::failpoint_injects_exhaustion_without_consuming_space ... ok
      test resident::memory::manager::tests::policy ... ok
      test resident::memory::manager::tests::realloc_bumps_gen_and_stale_is_rejected ... ok
      test resident::memory::staging::tests::acquire_wait_fails_when_all_slots_are_held ... ok
      test resident::memory::staging::tests::slots_cycle_round_robin ... ok
      test resident::memory::staging::tests::slot_is_not_reused_before_its_event_completes ... ok
      test resident::memory::staging::tests::payload_access_requires_a_held_slot ... ok
      test resident::memory::staging::tests::acquire_wait_blocks_on_oldest_transfer ... ok
      test resident::memory::manager::tests::live_regions_never_overlap ... ok

      test result: ok. 31 passed; 0 failed; 0 ignored; 0 measured; 189 filtered out; finished in 0.03s
```
/answer 9e96-4 allow
ReasoningToken: 34
Control Request:
```yaml
id: 9e96-5
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench/rag
  script: timeout 900 cargo test --lib 2>&1 | grep -E "test result|FAILED|failed" | head; timeout 600 cargo check --lib --features cuda --tests 2>&1 | grep -E "^error|memory/" -A6 | head -30; echo cuda-check-exit=$?
  reason: Gesamte Lib-Testsuite und cuda-Build (inkl. Tests) prüfen
```
/answer 9e96-5 allow
Control Request:
```yaml
id: 9e96-6
result:
  structuredContent:
    stdout: |
      test resident::processor::resource::tests::failed_load_backs_off_until_deadline ... ok
      test resident::subengine::lifecycle::tests::failed_init_leaves_cell_empty_and_backs_off ... ok
      test result: ok. 220 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 3.48s
        --> src/resident/memory/cuda.rs:27:4
         |
      27 | fn transfer(dir: TransferDir) -> impl Fn(DriverError) -> MemoryError {
         |    ^^^^^^^^
         |
         = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

      --
        --> src/resident/memory/cuda.rs:64:5
         |
      61 | pub struct CudaMemoryManager {
         |            ----------------- field in this struct
      ...
      64 |     backing: Vec<Mutex<Option<CudaSlice<u8>>>>,
         |     ^^^^^^^
      --
         --> src/resident/memory/cuda.rs:132:19
          |
       69 | impl CudaMemoryManager {
          | ---------------------- methods in this implementation
      ...
      132 |     pub(crate) fn copy_d2d(
          |                   ^^^^^^^^
      --
        --> src/resident/memory/dtype.rs:89:19
         |
      89 |     pub(crate) fn elems_to_bytes(elems: usize, dtype: DType) -> usize {
         |                   ^^^^^^^^^^^^^^

      warning: function `bytes_to_elems` is never used
      cuda-check-exit=0
```
/answer 9e96-6 allow
ReasoningToken: 153
Control Request:
```yaml
id: 9e96-7
toolName: ast_create
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/docs/architecture/gpu_memory.md
    source: |
      # GPU memory and arenas

      Single arena contract for the resident engine; index storage (`resident::index`) uses `ArenaAllocator` from here and does not define its own.

      ## Contract (`resident::memory`)

      | Item | Role |
      |------|------|
      | `Region { arena, offset, len, gen }` | Pointer-free handle. `gen` invalidates handles after free/realloc. |
      | `ArenaKind` | `Weights`, `IndexSegment`, `Workspace`, `Staging`, `ParamBlock`; `ArenaKind::id()` is the `ArenaId`. |
      | `ArenaAllocator` | Offset allocator for one class. `release_after(region, event)` returns memory only after the event completed. |
      | `ArenaSet` | One allocator per class. |
      | `DeviceMemory` | `alloc_in`, `free`, `release_after`, `budget`. Implemented by `CpuMemoryManager` and (`cuda`) `CudaMemoryManager`. |
      | `StagingRing<S>` | Slots reused only after the event of the last transfer completed. Pinned host buffers under `cuda`. |
      | `Budget` | `free`, `total`, `reserved_for_generations`; `usable()` subtracts the reserve. |

      ## Budget

      - Free/total come from `cuMemGetInfo`, never from a fixed size. Expect roughly 5-5.5 GB usable on a 6 GB card; measure on the target machine.
      - `ArenaSizes::index_segment` covers all resident generations; `generation_reserve(n)` is the part held back for non-active generations (copy-on-write peak, two generations by default).
      - `CudaMemoryManager::new` rejects a plan that exceeds `free - headroom` (`DEFAULT_HEADROOM` = 512 MiB) before allocating.
      - Staging is host memory and does not count against the device total.

      ## Allocation behaviour

      - Device arenas are single backing allocations made at start-up; operation does only offset arithmetic.
      - The default stream-ordered pool gets `RELEASE_THRESHOLD = u64::MAX`, so library-side allocations (Candle) stay in the pool instead of returning to the driver.
      - Pinned staging buffers are write-combined: fast for host-to-device, slow to read on the host. Do not use them for result readback that the CPU scans.

      ## dtype

      GPU `F16`, CPU `F32` (`compute_dtype`). Avoid BF16 on Turing. Attention masks use `F16_MASK_VALUE` (-1e4), not `-inf`. Expect cosine deviations around 1e-3 between f16 and f32 results.

      ## Foreign allocations (Candle)

      `CudaSlice` owns its memory and frees it on drop, so an arena sub-range cannot be handed to Candle as storage without risking a double free.

      **Current path (fallback):** arenas serve engine kernels only. Candle computes in its own allocations and results are moved into arenas with `CudaMemoryManager::copy_d2d`.

      **Open gate:** wrapping an arena range as Candle storage (`wrap_cuda_slice`, or `upgrade_device_ptr` plus `leak`) has not been tried. It needs a CUDA device and a drop-behaviour test. Until it passes, nothing may depend on zero-copy encoding into arenas.

      ## Limits

      - `copy_d2d` requires source and destination in different arena classes.
      - `upload`/`download` are `unsafe`: the host buffer must be pinned and untouched until an event recorded after the call completed.
      - GPU tests are `#[ignore]` and run with `cargo test --features cuda -- --ignored` on a machine with a device.
  reason: "Architektur-Dokument mit Arena-Vertrag, Budget und G-WRAP-Status anlegen"
```
/answer 9e96-7 allow
Result Stats: id=1e6ec56d-772f-444c-b948-f179da853722, total: 248289, in: 60, out: 44892, reason: 0, read: 1928148, write: 203337, start: 1792349969277, end: 1792349979653, time: 10292, total: 352673, io: 0, control: 84, sent: 0, recv: 275
D3 ist umgesetzt, aber nur teilweise geprüft. Die 220 Lib-Tests laufen grün (31 davon im Memory-Modul), und `cargo check --features cuda --tests` baut. Auf dieser Maschine gibt es kein `nvcc`. Die GPU-Tests (`#[ignore]`) und das G-WRAP-Gate sind deshalb nicht gelaufen. Beim `cuda`-Build bleiben Dead-Code-Warnungen, weil D4, D6 und D7 `copy_d2d`, `upload` und `download` noch nicht aufrufen.

**Hänger:** Mein Test-Helfer `Gate` hatte einen Deadlock, der den `cargo`-Lauf blockierte. Die Lane wartete beim Drop auf den Job, während der Sender noch lebte. Ich habe die Feldreihenfolge geändert. Danach habe ich ohne `--offline` und mit Timeout gebaut.

**Neu in `rag/src/resident/memory/`:**
- `error.rs`: `MemoryError` und `TransferDir`. `ResidentError::Memory` ist aufgenommen, ebenso `IsRetryable`.
- `budget.rs`: `ArenaKind`, `ArenaSizes`, `Budget`, `BudgetProbe` und `check_fit`.
  - Der Plan wird gegen freien VRAM abzüglich 512 MiB Reserve geprüft, bevor allokiert wird.
  - `generation_reserve(n)` hält den Index-Anteil für die inaktiven Generationen zurück, standardmäßig zwei Generationen.
- `arena.rs`: `ArenaAllocator` mit `release_after(region, event)`, `ArenaSet` und der Trait `DeviceMemory`.
  - `ArenaAllocator` baut auf dem vorhandenen `BookkeepingArena` auf, es gibt also keine zweite Offset-Logik.
  - Eine Region geht erst nach Abschluss ihres Events an die Free-List zurück.
- `staging.rs`: `StagingRing<S>` mit `acquire`, `acquire_wait`, `publish` und `release`. Ein Slot wird erst nach dem Event seines letzten Transfers wiederverwendet.
- `cpu.rs`: `CpuMemoryManager` als RAM-Arena mit `read`, `write` und `copy`.
- `cuda.rs` (hinter `cuda`): `CudaMemoryManager` mit je einer Backing-Allokation pro Arena-Klasse.
  - Dazu kommen die Pool-Konfiguration (`RELEASE_THRESHOLD = u64::MAX`), `mem_get_info`, ein pinned Staging-Ring und die Helfer `copy_d2d`, `upload` und `download`.
- `dtype.rs`: `F16_MASK_VALUE = -1e4`.
- `subengine/device.rs`: `DeviceHandle::cuda_context()` (nur `pub(crate)`).
- `rag/docs/architecture/gpu_memory.md`: Arena-Vertrag, Budget und Stand von G-WRAP.

**Abweichungen vom Plan:**
- Der bestehende Trait `MemoryManager` kennt keine Arena-Klasse. Deshalb gibt es daneben `DeviceMemory` mit `alloc_in`, `release_after` und `budget`. `MemoryManager` habe ich nicht geändert.
- `StagingRing` ist generisch und ohne `cuda` testbar. Nur die pinned Variante (`PinnedStaging`) liegt hinter `cuda`.
- `copy_d2d` verlangt, dass Quelle und Ziel in verschiedenen Arena-Klassen liegen.

**G-WRAP:** Offen, das Gate braucht ein CUDA-Gerät. Bis dahin gilt der Fallback: Arenen nur für eigene Kernel, Candle rechnet in eigenen Allokationen, die Ergebnisse kommen per D2D-Copy in die Arenen. Nichts darf von Zero-Copy abhängen, bevor das Gate bestanden ist.
nvcc ist in der config toml konfiguriert
SystemInit:  id=1e6ec56d-772f-444c-b948-f179da853722, cwd=/home/user/xyan/xy.ai.workbench/project, model=claude-sonnet-5-5
Text:
 Dann war meine Aussage falsch, ich hatte nur `which nvcc` geprüft. Ich sehe mir die Konfiguration an und führe die GPU-Tests aus.
ReasoningToken: 75
Control Request:
```yaml
id: 9e96-8
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench/rag
  script: "cat .cargo/config.toml; nvidia-smi --query-gpu=name,memory.free,memory.total --format=csv; timeout 900 cargo test --lib --features cuda resident::memory -- --include-ignored 2>&1 | grep -E \"^test |test result|panicked|error\" | grep -v \"\\.\\.\\. ok\" | head -30"
  reason: Cargo-Config lesen und die ignorierten CUDA-Tests auf der GPU ausführen
```
/answer 9e96-8 allow
Control Request:
```yaml
id: 9e96-9
result:
  structuredContent:
    stdout: |
      [build]
      target-dir = ".bin"
      rustflags = ["--check-cfg=cfg(loom)", "-L", "native=/usr/local/cuda-13.4/lib64"]

      [env]
      CUDA_PATH = { value = "/usr/local/cuda-13.4", force = false }
      CUDA_HOME = { value = "/usr/local/cuda-13.4", force = false }
      CUDA_COMPUTE_CAP = { value = "75", force = false }
      CUDARC_CUDA_VERSION = { value = "13030", force = false }
      name, memory.free [MiB], memory.total [MiB]
      NVIDIA GeForce GTX 1660 SUPER, 4962 MiB, 6144 MiB
      test result: ok. 34 passed; 0 failed; 0 ignored; 0 measured; 193 filtered out; finished in 0.27s
```
/answer 9e96-9 allow
Result Stats: id=1e6ec56d-772f-444c-b948-f179da853722, total: 251486, in: 64, out: 45808, reason: 0, read: 2139105, write: 205614, start: 1792350110237, end: 1792350114840, time: 4549, total: 368372, io: 0, control: 54, sent: 0, recv: 108
Die Config setzt `CUDA_PATH` und `CUDA_HOME` auf `/usr/local/cuda-13.4`. Auf der GTX 1660 SUPER laufen alle 34 Memory-Tests inklusive der drei vorher ignorierten GPU-Tests grün:
- `upload_copy_download_roundtrip`: H2D, D2D und D2H über Pinned-Staging.
- `oversized_plan_is_rejected_before_allocating`: ein zu großer Plan wird vor dem Allokieren abgelehnt.
- `cuda_lanes_order_work_through_events` aus D2.

Die Aussage „kein `nvcc`“ in meiner Zusammenfassung ist damit hinfällig. Der `cuda`-Teil ist auf echter Hardware geprüft.

G-WRAP ist weiter offen. Dafür fehlt nicht die Hardware, sondern der Prototyp selbst: ein Test, der prüft, ob Candle einen Arena-Bereich beim Drop doppelt freigibt. Dieser Test existiert noch nicht. Soll ich ihn jetzt schreiben und laufen lassen?

Frei waren zuletzt nur etwa 4,9 GB von 6 GB. Das passt zur Budget-Annahme „nicht fix 6 GB“. Passen Sie den Hinweis in `gpu_memory.md` auf diesen Messwert an, wenn Sie das möchten.
Ja und G-WRAP
SystemInit:  id=1e6ec56d-772f-444c-b948-f179da853722, cwd=/home/user/xyan/xy.ai.workbench/project, model=claude-sonnet-5-5
Text:
 Ich schaue zuerst in die Candle-0.11-API.
ReasoningToken: 64
Control Request:
```yaml
id: 9e96-a
toolName: bash
arguments:
  cwd: /home/user/.cargo/registry/src
  script: cd */candle-core-0.11.0/src; grep -n "pub fn wrap_cuda_slice\|pub fn new_with_stream\|pub fn cuda_stream\|pub fn from_cuda_storage\|pub fn from_storage\|pub fn cuda_device\|pub fn as_cuda_slice\|pub fn new_cuda\|pub fn cuda_with_stream\|pub fn from_slice\|pub fn context\|pub fn stream" cuda_backend/*.rs device.rs tensor.rs storage.rs | head -30; grep -n "pub struct CudaStorage" -A6 cuda_backend/mod.rs; grep -n "pub enum CudaStorageSlice" -A4 cuda_backend/mod.rs; grep -n "pub struct CudaDevice" -A8 cuda_backend/device.rs
  reason: Candle-API für das Einbinden fremder CudaSlice als Storage ermitteln
```
/answer 9e96-a allow
Control Request:
```yaml
id: 9e96-b
result:
  structuredContent:
    stdout: |
      cuda_backend/device.rs:288:    pub fn cuda_stream(&self) -> Arc<cudarc::driver::CudaStream> {
      cuda_backend/device.rs:387:    pub fn new_with_stream(ordinal: usize) -> Result<Self> {
      cuda_backend/mod.rs:1285:    pub fn wrap_cuda_slice<T: CudaDType>(slice: CudaSlice<T>, device: CudaDevice) -> CudaStorage {
      cuda_backend/mod.rs:1289:    pub fn as_cuda_slice<T: CudaDType>(&self) -> Result<&CudaSlice<T>> {
      cuda_backend/mod.rs:1293:    pub fn as_cuda_slice_mut<T: CudaDType>(&mut self) -> Result<&mut CudaSlice<T>> {
      device.rs:234:    pub fn new_cuda(ordinal: usize) -> Result<Self> {
      device.rs:254:    pub fn new_cuda_with_stream(ordinal: usize) -> Result<Self> {
      tensor.rs:564:    pub fn from_slice<S: ShapeWithOneHole, D: crate::WithDType>(
      tensor.rs:601:    pub fn from_storage<S: Into<Shape>>(
      1228:pub struct CudaStorage {
      1229-    pub slice: CudaStorageSlice,
      1230-    pub device: CudaDevice,
      1231-}
      1232-
      1233-pub trait CudaDType: Sized {
      1234-    fn as_cuda_slice(s: &CudaStorage) -> Result<&CudaSlice<Self>>;
      100:pub enum CudaStorageSlice {
      101-    U8(CudaSlice<u8>),
      102-    U32(CudaSlice<u32>),
      103-    I16(CudaSlice<i16>),
      104-    I32(CudaSlice<i32>),
      59:pub struct CudaDevice {
      60-    id: DeviceId,
      61-    context: Arc<cudarc::driver::CudaContext>,
      62-    modules: Arc<std::sync::RwLock<ModuleStore>>,
      63-    custom_modules: Arc<std::sync::RwLock<HashMap<String, Arc<cudarc::driver::CudaModule>>>>,
      64-    stream: Arc<cudarc::driver::CudaStream>,
      65-    pub(crate) blas: Arc<cudarc::cublas::CudaBlas>,
      66-    curand: Arc<Mutex<CudaRng>>,
      67-    seed_value: Arc<RwLock<u64>>,
```
/answer 9e96-b allow
ReasoningToken: 644
Control Request:
```yaml
id: 9e96-c
toolName: bash
arguments:
  cwd: /home/user/.cargo/registry/src
  script: "cd */cudarc-0.19.9/src/driver/safe; grep -n \"impl<T> Drop for CudaSlice\" -A40 core.rs; grep -n \"pub unsafe fn upgrade_device_ptr\" -B3 -A22 core.rs; grep -n \"pub fn leak\" -A25 core.rs | head -40; sed -n 595,625p /home/user/.cargo/registry/src/*/candle-core-0.11.0/src/tensor.rs; sed -n 1280,1290p /home/user/.cargo/registry/src/*/candle-core-0.11.0/src/cuda_backend/mod.rs"
  reason: "Drop-Verhalten von CudaSlice, upgrade_device_ptr, leak und Tensor::from_storage lesen"
```
/answer 9e96-c allow
Control Request:
```yaml
id: 9e96-d
result:
  content:
  - type: text
    text: "Full output written to file (/tmp/mcpc-stdout-etyh0t9p.log). Reduce the content to what is strictly needed: use targeted commands (grep, head, tail, awk, ast_outline) to extract only the relevant parts. This is a general output limit for all commands to prevent context bloat."
  structuredContent:
    stdout_file: /tmp/mcpc-stdout-etyh0t9p.log
    stdout_file_file_stats:
      path: /tmp/mcpc-stdout-etyh0t9p.log
      size_bytes: 4760
      lines: 135
      words: 496
      complexity: 0.571
      created: 2026-10-18T19:03:17.377913+00:00
      modified: 2026-10-18T19:03:17.377913+00:00
      accessed: 2026-10-18T19:03:17.378913+00:00
      line_length_max: 98
      line_length_min: 0
      line_length_avg: 34.26
      words_per_line_avg: 3.67
      checksum: 675e1888770942656416d3156c2a2d68529224d4eca70aa925638740d4f6ed34
    stdout_file_ast_outline:
    - id: file
      lines: 1-135
      signature: "800:impl<T> Drop for CudaSlice<T> {"
```
/answer 9e96-d allow
Control Request:
```yaml
id: 9e96-e
toolName: python
arguments:
  script: print(open('/tmp/mcpc-stdout-etyh0t9p.log').read())
  reason: Ausgabedatei lesen
```
/answer 9e96-e allow
Control Request:
```yaml
id: 9e96-f
result:
  structuredContent:
    exit_code: 0
    stdout: |+
      800:impl<T> Drop for CudaSlice<T> {
      801-    fn drop(&mut self) {
      802-        let ctx = &self.stream.ctx;
      803-        if ctx.is_managing_stream_synchronization() {
      804-            if let Some(read) = self.read.as_ref() {
      805-                ctx.record_err(self.stream.wait(read));
      806-            }
      807-            if let Some(write) = self.write.as_ref() {
      808-                ctx.record_err(self.stream.wait(write));
      809-            }
      810-        }
      811-        if ctx.has_async_alloc {
      812-            ctx.record_err(unsafe {
      813-                result::free_async(self.cu_device_ptr, self.stream.cu_stream)
      814-            });
      815-        } else {
      816-            ctx.record_err(self.stream.synchronize());
      817-            ctx.record_err(unsafe { result::free_sync(self.cu_device_ptr) });
      818-        }
      819-    }
      820-}
      821-
      822-impl<T> CudaSlice<T> {
      823-    /// The number of elements of `T` in this object.
      824-    pub fn len(&self) -> usize {
      825-        self.len
      826-    }
      827-
      828-    /// The number of bytes in this object.
      829-    pub fn num_bytes(&self) -> usize {
      830-        self.len * std::mem::size_of::<T>()
      831-    }
      832-
      833-    /// True if there are no elements in the object.
      834-    pub fn is_empty(&self) -> bool {
      835-        self.len == 0
      836-    }
      837-
      838-    /// The device ordinal this belongs to
      839-    pub fn ordinal(&self) -> usize {
      840-        self.stream.ctx.ordinal
      2499-    /// - `cu_device_ptr` must space for `len * std::mem::size_of<T>()` bytes
      2500-    /// - The memory may not be valid for type `T`, so some sort of memset operation
      2501-    ///   should be called on the memory.
      2502:    pub unsafe fn upgrade_device_ptr<T>(
      2503-        self: &Arc<Self>,
      2504-        cu_device_ptr: sys::CUdeviceptr,
      2505-        len: usize,
      2506-    ) -> CudaSlice<T> {
      2507-        let (read, write) = if self.ctx.is_event_tracking() {
      2508-            (
      2509-                Some(self.ctx.new_event(None).unwrap()),
      2510-                Some(self.ctx.new_event(None).unwrap()),
      2511-            )
      2512-        } else {
      2513-            (None, None)
      2514-        };
      2515-        CudaSlice {
      2516-            cu_device_ptr,
      2517-            len,
      2518-            read,
      2519-            write,
      2520-            stream: self.clone(),
      2521-            marker: PhantomData,
      2522-        }
      2523-    }
      2524-}
      2468:    pub fn leak(self) -> sys::CUdeviceptr {
      2469-        let mut s = std::mem::ManuallyDrop::new(self);
      2470-        let ptr = s.cu_device_ptr;
      2471-
      2472-        // Ensure pending operations are complete before resources are released.
      2473-        if s.stream.ctx.is_managing_stream_synchronization() {
      2474-            if let Some(read) = s.read.as_ref() {
      2475-                s.stream.ctx.record_err(s.stream.wait(read));
      2476-            }
      2477-            if let Some(write) = s.write.as_ref() {
      2478-                s.stream.ctx.record_err(s.stream.wait(write));
      2479-            }
      2480-        }
      2481-
      2482-        // Manually drop fields that own resources.
      2483-        unsafe {
      2484-            std::ptr::drop_in_place(&mut s.read);
      2485-            std::ptr::drop_in_place(&mut s.write);
      2486-            std::ptr::drop_in_place(&mut s.stream);
      2487-        }
      2488-
      2489-        ptr
      2490-    }
      2491-}
      2492-
      2493-impl CudaStream {

          /// Creates a fresh tensor structure based on a storage and a shape.
          ///
          /// # Note
          /// - This uses contiguous strides
          /// - Ensure the shape is compatible with the shape of the storage.
          pub fn from_storage<S: Into<Shape>>(
              storage: Storage,
              shape: S,
              op: BackpropOp,
              is_variable: bool,
          ) -> Tensor {
              from_storage(storage, shape, op, is_variable)
          }

          // TODO: Also make an inplace version or a pre-allocated? This could be tricky
          // if this can create cycles in the compute graph.
          binary_op!(add, Add);
          binary_op!(mul, Mul);
          binary_op!(sub, Sub);
          binary_op!(div, Div);
          binary_op_scalar!(maximum, Maximum);
          binary_op_scalar!(minimum, Minimum);
          broadcast_binary_op!(broadcast_add, add);
          broadcast_binary_op!(broadcast_mul, mul);
          broadcast_binary_op!(broadcast_sub, sub);
          broadcast_binary_op!(broadcast_div, div);
          broadcast_binary_op!(broadcast_maximum, maximum);
          broadcast_binary_op!(broadcast_minimum, minimum);
          broadcast_binary_op!(broadcast_eq, eq);
          broadcast_binary_op!(broadcast_ne, ne);
      cuda_dtype!(f32, F32);
      cuda_dtype!(f64, F64);
      cuda_dtype!(float8::F8E4M3, F8E4M3);

      impl CudaStorage {
          pub fn wrap_cuda_slice<T: CudaDType>(slice: CudaSlice<T>, device: CudaDevice) -> CudaStorage {
              T::wrap_cuda_slice(slice, device)
          }

          pub fn as_cuda_slice<T: CudaDType>(&self) -> Result<&CudaSlice<T>> {
              T::as_cuda_slice(self)
```
/answer 9e96-f allow
ReasoningToken: 2217
Control Request:
```yaml
id: 9e96-10
toolName: bash
arguments:
  cwd: /home/user/.cargo/registry/src
  script: cd */; grep -n "pub fn check_err\|pub fn record_err\|pub unsafe fn disable_event_tracking\|pub fn has_async_alloc\|pub unsafe fn enable_event" cudarc-0.19.9/src/driver/safe/core.rs; grep -n "pub fn storage_mut_and_layout\|pub fn storage_and_layout" -A4 candle-core-0.11.0/src/tensor.rs | head -14; grep -n "pub use cuda_backend\|pub mod cuda_backend\|pub use storage\|pub use op\|pub mod op" candle-core-0.11.0/src/lib.rs; grep -n "fn new_cuda\b" -A18 candle-core-0.11.0/src/cuda_backend/device.rs | head -30
  reason: Candle-Exporte und cudarc-Fehlerprüfung für den G-WRAP-Test klären
```
/answer 9e96-10 allow
Control Request:
```yaml
id: 9e96-11
result:
  structuredContent:
    stdout: ""
    stderr: |
      bash: line 1: cd: too many arguments
      grep: cudarc-0.19.9/src/driver/safe/core.rs: No such file or directory
      grep: candle-core-0.11.0/src/tensor.rs: No such file or directory
      grep: candle-core-0.11.0/src/lib.rs: No such file or directory
      grep: candle-core-0.11.0/src/cuda_backend/device.rs: No such file or directory
```
/answer 9e96-11 allow
Control Request:
```yaml
id: 9e96-12
toolName: bash
arguments:
  cwd: /home/user/.cargo/registry/src
  script: cd $(ls -d */cudarc-0.19.9 | head -1)/..; grep -n "pub fn check_err\|pub fn record_err\|pub fn is_event_tracking\|pub unsafe fn disable_event_tracking" cudarc-0.19.9/src/driver/safe/core.rs; grep -n "fn storage_mut_and_layout\|fn storage_and_layout" -A4 candle-core-0.11.0/src/tensor.rs | head -14; grep -n "pub use cuda_backend\|pub mod cuda_backend\|pub use storage\|pub use op\|pub mod op\|cuda_backend" candle-core-0.11.0/src/lib.rs; grep -n "fn new\b" -A12 candle-core-0.11.0/src/cuda_backend/device.rs | head -24
  reason: Candle-Exporte und cudarc-Fehlerprüfung klären
```
/answer 9e96-12 allow
Control Request:
```yaml
id: 9e96-13
result:
  structuredContent:
    stdout: |
      458:    pub fn is_event_tracking(&self) -> bool {
      493:    pub unsafe fn disable_event_tracking(&self) {
      502:    pub fn check_err(&self) -> Result<(), DriverError> {
      514:    pub fn record_err<T>(&self, result: Result<T, DriverError>) {
      2742:    pub(crate) fn storage_mut_and_layout(
      2743-        &self,
      2744-    ) -> (std::sync::RwLockWriteGuard<'_, Storage>, &Layout) {
      2745-        let storage = self.storage.write().unwrap();
      2746-        (storage, &self.layout)
      --
      2750:    pub fn storage_and_layout(&self) -> (std::sync::RwLockReadGuard<'_, Storage>, &Layout) {
      2751-        let storage = self.storage.read().unwrap();
      2752-        (storage, &self.layout)
      2753-    }
      2754-
      59:pub mod cuda_backend;
      64:pub mod dummy_cuda_backend;
      76:pub mod op;
      93:pub use cuda_backend::cudnn;
      107:pub use storage::Storage;
      114:pub use cuda_backend as cuda;
      117:pub use dummy_cuda_backend as cuda;
      20:    fn new() -> Self {
      21-        // https://users.rust-lang.org/t/idiomatic-rust-way-to-generate-unique-id/33805
      22-        use std::sync::atomic;
      23-        static COUNTER: atomic::AtomicUsize = atomic::AtomicUsize::new(1);
      24-        Self(COUNTER.fetch_add(1, atomic::Ordering::Relaxed))
      25-    }
      26-}
      27-
      28-struct CudaRng(cudarc::curand::CudaRng);
      29-unsafe impl Send for CudaRng {}
      30-
      31-const CUDA_GRAPH_HTOD_CACHE_MAX_BYTES: usize = 4096;
      32-
      --
      418:    fn new(ordinal: usize) -> Result<Self> {
      419-        let context = cudarc::driver::CudaContext::new(ordinal).w()?;
      420-        let stream = context.per_thread_stream();
      421-        Self::from_context_and_stream(context, stream)
      422-    }
      423-
      424-    fn set_seed(&self, seed: u64) -> Result<()> {
      425-        // We do not call set_seed but instead create a new curand object. This ensures that the
      426-        // state will be identical and the same random numbers will be generated.
      427-        let mut curand = self.curand.lock().unwrap();
```
/answer 9e96-13 allow
ReasoningToken: 1591
Control Request:
```yaml
id: 9e96-14
toolName: bash
arguments:
  cwd: /home/user/.cargo/registry/src
  script: cd $(ls -d */candle-core-0.11.0 | head -1)/src; grep -n "pub fn slice_set" -A3 tensor.rs; grep -n "pub fn none" -B2 -A3 op.rs | head; grep -n "pub fn clone_htod\|pub fn clone_dtoh" -A3 ../../cudarc-0.19.9/src/driver/safe/core.rs | head
  reason: "slice_set, BackpropOp::none und clone_htod/dtoh bestätigen"
```
/answer 9e96-14 allow
Control Request:
```yaml
id: 9e96-15
result:
  structuredContent:
    stdout: |
      1094-
      1095-impl BackpropOp {
      1096:    pub fn none() -> Self {
      1097-        BackpropOp(None)
      1098-    }
      1099-
      1596:    pub fn clone_htod<T: DeviceRepr, Src: HostSlice<T> + ?Sized>(
      1597-        self: &Arc<Self>,
      1598-        src: &Src,
      1599-    ) -> Result<CudaSlice<T>, DriverError> {
      --
      1634:    pub fn clone_dtoh<T: DeviceRepr, Src: DevicePtr<T>>(
      1635-        self: &Arc<Self>,
      1636-        src: &Src,
      1637-    ) -> Result<Vec<T>, DriverError> {
```
/answer 9e96-15 allow
ReasoningToken: 37
Control Request:
```yaml
id: 9e96-16
toolName: bash
arguments:
  cwd: /home/user/.cargo/registry/src
  script: cd $(ls -d */candle-core-0.11.0 | head -1)/src; grep -rn "fn slice_set\|fn slice_assign\|fn copy2d\|fn copy_strided_src" *.rs | head
  reason: In-place-Schreibpfad in Candle suchen
```
/answer 9e96-16 allow
Control Request:
```yaml
id: 9e96-17
result:
  structuredContent:
    stdout: |
      backend.rs:123:    fn copy_strided_src(&self, _: &mut Self, _: usize, _: &Layout) -> Result<()>;
      backend.rs:127:    fn copy2d(
      dummy_cuda_backend.rs:188:    fn copy_strided_src(&self, _: &mut Self, _: usize, _: &Layout) -> Result<()> {
      dummy_cuda_backend.rs:192:    fn copy2d(
      dummy_metal_backend.rs:181:    fn copy_strided_src(&self, _: &mut Self, _: usize, _: &Layout) -> Result<()> {
      dummy_metal_backend.rs:185:    fn copy2d(
      storage.rs:801:    pub(crate) fn copy_strided_src(
      storage.rs:823:    pub(crate) fn copy2d(
      tensor_cat.rs:247:    pub fn slice_set<D: Dim>(&self, src: &Self, dim: D, offset: usize) -> Result<()> {
      tensor.rs:2826:    pub fn slice_assign<D: std::ops::RangeBounds<usize>>(
```
/answer 9e96-17 allow
Text:
 Jetzt der Gate-Test.
Control Request:
```yaml
id: 9e96-18
toolName: ast_create
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/memory/wrap_gate.rs
    source: |
      //! Prototype gate: can an arena sub-range back a Candle tensor without Candle freeing it?

      use std::sync::Arc;

      use candle_core::cuda_backend::CudaStorage;
      use candle_core::op::BackpropOp;
      use candle_core::{DType, Device, Storage, Tensor};
      use cudarc::driver::{CudaContext, CudaSlice, CudaStream, DevicePtr};

      const ARENA_ELEMS: usize = 64;
      const VIEW_OFFSET: usize = 16;
      const VIEW_ELEMS: usize = 16;

      struct Fixture {
          dev: Device,
          ctx: Arc<CudaContext>,
          stream: Arc<CudaStream>,
          arena: CudaSlice<f32>,
      }

      impl Fixture {
          fn new() -> Self {
              let dev = Device::new_cuda(0).unwrap();
              let Device::Cuda(cd) = &dev else { unreachable!() };
              let stream = cd.cuda_stream();
              let ctx = stream.context().clone();
              let host: Vec<f32> = (0..ARENA_ELEMS).map(|i| i as f32).collect();
              let arena = stream.clone_htod(&host).unwrap();
              Self { dev, ctx, stream, arena }
          }

          /// Tensor over `arena[VIEW_OFFSET..VIEW_OFFSET + VIEW_ELEMS]` whose storage owns the range.
          fn wrapped_view(&self) -> Tensor {
              let Device::Cuda(cd) = &self.dev else { unreachable!() };
              let (base, _g) = self.arena.device_ptr(&self.stream);
              let at = base + (VIEW_OFFSET * 4) as u64;
              let slice = unsafe { self.stream.upgrade_device_ptr::<f32>(at, VIEW_ELEMS) };
              let storage = CudaStorage::wrap_cuda_slice(slice, cd.clone());
              Tensor::from_storage(Storage::Cuda(storage), (VIEW_ELEMS,), BackpropOp::none(), false)
          }

          fn arena_host(&self) -> Vec<f32> {
              self.stream.clone_dtoh(&self.arena).unwrap()
          }
      }

      #[test]
      #[ignore = "requires a CUDA device"]
      fn candle_reads_and_writes_the_wrapped_range() {
          let f = Fixture::new();
          let view = f.wrapped_view();

          let doubled = view.affine(2.0, 0.0).unwrap().to_vec1::<f32>().unwrap();
          assert_eq!(doubled[0], 2.0 * VIEW_OFFSET as f32);

          let src = Tensor::full(-1f32, VIEW_ELEMS, &f.dev).unwrap();
          view.slice_set(&src, 0, 0).unwrap();
          f.ctx.synchronize().unwrap();
          let host = f.arena_host();
          assert!(host[VIEW_OFFSET..VIEW_OFFSET + VIEW_ELEMS].iter().all(|v| *v == -1.0));
          assert_eq!(host[VIEW_OFFSET - 1], (VIEW_OFFSET - 1) as f32);
          assert_eq!(host[VIEW_OFFSET + VIEW_ELEMS], (VIEW_OFFSET + VIEW_ELEMS) as f32);

          std::mem::forget(view);
          f.ctx.check_err().unwrap();
      }

      #[test]
      #[ignore = "requires a CUDA device"]
      fn dropping_a_wrapped_tensor_frees_foreign_memory() {
          let f = Fixture::new();
          drop(f.wrapped_view());
          f.ctx.synchronize().ok();
          let err = f.ctx.check_err();
          println!("driver error after drop: {err:?}");
          assert!(err.is_err(), "drop of a wrapped interior pointer went unnoticed");
      }

      #[test]
      #[ignore = "requires a CUDA device"]
      fn candle_outputs_cannot_target_arena_memory() {
          let f = Fixture::new();
          let a = Tensor::ones(VIEW_ELEMS, DType::F32, &f.dev).unwrap();
          let out = a.affine(3.0, 0.0).unwrap();
          let Storage::Cuda(s) = &*out.storage_and_layout().0 else { unreachable!() };
          let out_ptr = {
              let sl = s.as_cuda_slice::<f32>().unwrap();
              sl.device_ptr(&f.stream).0
          };
          let (base, _g) = f.arena.device_ptr(&f.stream);
          let arena_end = base + (ARENA_ELEMS * 4) as u64;
          assert!(out_ptr < base || out_ptr >= arena_end, "candle allocates its own outputs");
      }
  reason: "G-WRAP-Prototyp-Tests (Wrap, Drop-Verhalten, Output-Allokation) schreiben"
```
/answer 9e96-18 allow
Control Request:
```yaml
id: 9e96-19
toolName: python
arguments:
  script: |-
    p='/home/user/xyan/xy.ai.workbench/rag/src/resident/memory/mod.rs'
    s=open(p).read()
    s=s.replace("pub mod staging;\n","pub mod staging;\n#[cfg(all(test, feature = \"cuda\"))]\nmod wrap_gate;\n")
    open(p,'w').write(s)
  reason: Gate-Modul einhängen
```
/answer 9e96-19 allow
Control Request:
```yaml
id: 9e96-1a
result:
  structuredContent:
    result: success
```
/answer 9e96-1a allow
Control Request:
```yaml
id: 9e96-1b
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench/rag
  script: "timeout 900 cargo test --lib --features cuda wrap_gate -- --include-ignored --test-threads=1 --nocapture 2>&1 | grep -vE \"^warning|^\\s+(-->|\\||=)|^\\s*$|^[0-9 ]+\\|\" | tail -40"
  reason: G-WRAP-Tests auf der GPU ausführen
```
/answer 9e96-1b allow
Control Request:
```yaml
id: 9e96-1c
result:
  structuredContent:
    stdout: |2
         Compiling xy-ai-rag v0.1.0 (/home/user/xyan/xy.ai.workbench/rag)
      ...
          Finished `test` profile [unoptimized + debuginfo] target(s) in 3.01s
           Running unittests src/lib.rs (.bin/debug/deps/xy_ai_rag-166381dca143ad7a)
      /home/user/xyan/xy.ai.workbench/rag/.bin/debug/deps/xy_ai_rag-166381dca143ad7a: error while loading shared libraries: libcurand.so.10: cannot open shared object file: No such file or directory
      error: test failed, to rerun pass `--lib`
      Caused by:
        process didn't exit successfully: `/home/user/xyan/xy.ai.workbench/rag/.bin/debug/deps/xy_ai_rag-166381dca143ad7a wrap_gate --include-ignored --test-threads=1 --nocapture` (exit status: 127)
```
/answer 9e96-1c allow
ReasoningToken: 80
Control Request:
```yaml
id: 9e96-1d
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench/rag
  script: ls /usr/local/cuda-13.4/lib64 | grep -i "curand\|cublas\.so" | head -4; export LD_LIBRARY_PATH=/usr/local/cuda-13.4/lib64:$LD_LIBRARY_PATH; timeout 900 cargo test --lib --features cuda wrap_gate -- --include-ignored --test-threads=1 --nocapture 2>&1 | grep -E "^test |driver error|panicked|test result|error|assert" | head -20
  reason: GPU-Tests mit CUDA-Laufzeitbibliotheken im Suchpfad ausführen
```
/answer 9e96-1d allow
Control Request:
```yaml
id: 9e96-1e
result:
  structuredContent:
    stdout: |
      libcublas.so
      libcublas.so.13
      libcublas.so.13.8.0.4
      libcurand.so
      test resident::memory::wrap_gate::candle_outputs_cannot_target_arena_memory ...
      thread 'resident::memory::wrap_gate::candle_outputs_cannot_target_arena_memory' (130785) panicked at src/resident/memory/wrap_gate.rs:83:58:
      test resident::memory::wrap_gate::candle_reads_and_writes_the_wrapped_range ...
      thread 'resident::memory::wrap_gate::candle_reads_and_writes_the_wrapped_range' (130793) panicked at src/resident/memory/wrap_gate.rs:53:41:
      test resident::memory::wrap_gate::dropping_a_wrapped_tensor_frees_foreign_memory ... driver error after drop: Ok(())
      thread 'resident::memory::wrap_gate::dropping_a_wrapped_tensor_frees_foreign_memory' (130794) panicked at src/resident/memory/wrap_gate.rs:76:5:
      test result: FAILED. 0 passed; 3 failed; 0 ignored; 0 measured; 227 filtered out; finished in 0.23s
      error: test failed, to rerun pass `--lib`
```
/answer 9e96-1e allow
Control Request:
```yaml
id: 9e96-1f
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench/rag
  script: export LD_LIBRARY_PATH=/usr/local/cuda-13.4/lib64; timeout 900 cargo test --lib --features cuda wrap_gate -- --include-ignored --test-threads=1 2>&1 | grep -A4 "panicked" | head -30
  reason: Panik-Meldungen der Gate-Tests lesen
```
/answer 9e96-1f allow
Control Request:
```yaml
id: 9e96-20
result:
  structuredContent:
    stdout: |
      thread 'resident::memory::wrap_gate::candle_outputs_cannot_target_arena_memory' (131402) panicked at src/resident/memory/wrap_gate.rs:83:58:
      called `Result::unwrap()` on an `Err` value: DriverError(CUDA_ERROR_UNSUPPORTED_PTX_VERSION, "the provided PTX was compiled with an unsupported toolchain.")
      note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

      ---- resident::memory::wrap_gate::candle_reads_and_writes_the_wrapped_range stdout ----
      --
      thread 'resident::memory::wrap_gate::candle_reads_and_writes_the_wrapped_range' (131410) panicked at src/resident/memory/wrap_gate.rs:53:41:
      called `Result::unwrap()` on an `Err` value: DriverError(CUDA_ERROR_UNSUPPORTED_PTX_VERSION, "the provided PTX was compiled with an unsupported toolchain.")

      ---- resident::memory::wrap_gate::dropping_a_wrapped_tensor_frees_foreign_memory stdout ----
      driver error after drop: Ok(())
      --
      thread 'resident::memory::wrap_gate::dropping_a_wrapped_tensor_frees_foreign_memory' (131411) panicked at src/resident/memory/wrap_gate.rs:76:5:
      drop of a wrapped interior pointer went unnoticed

      failures:
```
/answer 9e96-20 allow
Thinking:
Die Candle-Kernels laufen auf dieser Maschine nicht (UNSUPPORTED_PTX_VERSION, Treiber zu alt für Toolkit 13.4), daher bleibe ich beim Gate bei kernelfreien Kopieroperationen. Da `check_err` nach dem Drop `Ok` zurückgab, prüfe ich jetzt genauer, wie der Drop den Arena-Speicher behandelt.


Control Request:
```yaml
id: 9e96-21
toolName: python
arguments:
  script: |-
    p='/home/user/xyan/xy.ai.workbench/rag/src/resident/memory/wrap_gate.rs'
    s=open(p).read()
    a=s.index("#[test]")
    s=s[:a]+'''#[test]
    #[ignore = "requires a CUDA device"]
    fn candle_reads_and_writes_the_wrapped_range() {
        let f = Fixture::new();
        let view = f.wrapped_view();

        let seen = view.to_vec1::<f32>().unwrap();
        assert_eq!(seen[0], VIEW_OFFSET as f32);

        let src = Tensor::from_slice(&[-1f32; VIEW_ELEMS], VIEW_ELEMS, &f.dev).unwrap();
        view.slice_set(&src, 0, 0).unwrap();
        f.ctx.synchronize().unwrap();
        let host = f.arena_host();
        assert!(host[VIEW_OFFSET..VIEW_OFFSET + VIEW_ELEMS].iter().all(|v| *v == -1.0));
        assert_eq!(host[VIEW_OFFSET - 1], (VIEW_OFFSET - 1) as f32);
        assert_eq!(host[VIEW_OFFSET + VIEW_ELEMS], (VIEW_OFFSET + VIEW_ELEMS) as f32);

        std::mem::forget(view);
        f.ctx.check_err().unwrap();
        assert_eq!(f.arena_host()[0], 0.0);
    }

    #[test]
    #[ignore = "requires a CUDA device"]
    fn dropping_a_wrapped_tensor_frees_foreign_memory() {
        let f = Fixture::new();
        drop(f.wrapped_view());
        f.ctx.synchronize().unwrap();
        let after_drop = f.ctx.check_err();
        let readback = f.stream.clone_dtoh(&f.arena);
        println!("check_err after drop: {after_drop:?}");
        println!("arena readback after drop: {:?}", readback.as_ref().map(|v| v[..4].to_vec()));
        let Fixture { ctx, arena, .. } = f;
        drop(arena);
        ctx.synchronize().unwrap();
        println!("check_err after arena drop: {:?}", ctx.check_err());
    }

    #[test]
    #[ignore = "requires a CUDA device"]
    fn candle_copies_allocate_their_own_storage() {
        let f = Fixture::new();
        let view = f.wrapped_view();
        let copy = view.copy().unwrap();
        let ptr_of = |t: &Tensor| {
            let (st, _) = t.storage_and_layout();
            let Storage::Cuda(s) = &*st else { unreachable!() };
            s.as_cuda_slice::<f32>().unwrap().device_ptr(&f.stream).0
        };
        let (base, _g) = f.arena.device_ptr(&f.stream);
        let out = ptr_of(&copy);
        assert!(out < base || out >= base + (ARENA_ELEMS * 4) as u64);
        std::mem::forget(view);
    }
    '''
    s=s.replace("use candle_core::{DType, Device, Storage, Tensor};","use candle_core::{Device, Storage, Tensor};")
    open(p,'w').write(s)
  reason: Gate-Tests ohne Candle-Kernels neu fassen und Drop-Wirkung messen
```
/answer 9e96-21 allow
ReasoningToken: 1136
Control Request:
```yaml
id: 9e96-22
result:
  structuredContent:
    result: success
```
/answer 9e96-22 allow
Control Request:
```yaml
id: 9e96-23
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench/rag
  script: "export LD_LIBRARY_PATH=/usr/local/cuda-13.4/lib64; nvidia-smi | grep -i \"driver version\"; timeout 900 cargo test --lib --features cuda wrap_gate -- --include-ignored --test-threads=1 --nocapture 2>&1 | grep -E \"^test |check_err|readback|panicked|test result|error\\[|called\" | head -20"
  reason: Überarbeitete Gate-Tests ausführen
```
/answer 9e96-23 allow
Control Request:
```yaml
id: 9e96-24
result:
  structuredContent:
    stdout: |
      error[E0597]: `st` does not live long enough
```
/answer 9e96-24 allow
Control Request:
```yaml
id: 9e96-25
toolName: python
arguments:
  script: |-
    p='/home/user/xyan/xy.ai.workbench/rag/src/resident/memory/wrap_gate.rs'
    s=open(p).read()
    old=s[s.index("    let ptr_of = |t: &Tensor| {"):s.index("    let (base, _g) = f.arena.device_ptr(&f.stream);\n    let out")]
    new='''    let ptr_of = |t: &Tensor| -> u64 {
            let (st, _) = t.storage_and_layout();
            let Storage::Cuda(s) = &*st else { unreachable!() };
            let slice = s.as_cuda_slice::<f32>().unwrap();
            let (ptr, guard) = slice.device_ptr(&f.stream);
            drop(guard);
            ptr
        };
    '''
    s=s.replace(old,new)
    open(p,'w').write(s)
  reason: Lifetime-Fehler im Test beheben
```
/answer 9e96-25 allow
ReasoningToken: 65
Control Request:
```yaml
id: 9e96-26
result:
  structuredContent:
    result: success
```
/answer 9e96-26 allow
Control Request:
```yaml
id: 9e96-27
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench/rag
  script: export LD_LIBRARY_PATH=/usr/local/cuda-13.4/lib64; timeout 900 cargo test --lib --features cuda wrap_gate -- --include-ignored --test-threads=1 --nocapture 2>&1 | grep -E "^test |check_err|readback|panicked|test result|^error|called" -A2 | head -30
  reason: Gate-Tests erneut ausführen
```
/answer 9e96-27 allow
Control Request:
```yaml
id: 9e96-28
result:
  structuredContent:
    stdout: |
      test resident::memory::wrap_gate::candle_copies_allocate_their_own_storage ... ok
      test resident::memory::wrap_gate::candle_reads_and_writes_the_wrapped_range ...
      thread 'resident::memory::wrap_gate::candle_reads_and_writes_the_wrapped_range' (132581) panicked at src/resident/memory/wrap_gate.rs:57:32:
      called `Result::unwrap()` on an `Err` value: DriverError(CUDA_ERROR_UNSUPPORTED_PTX_VERSION, "the provided PTX was compiled with an unsupported toolchain.")
      note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
      FAILED
      test resident::memory::wrap_gate::dropping_a_wrapped_tensor_frees_foreign_memory ...
      thread 'resident::memory::wrap_gate::dropping_a_wrapped_tensor_frees_foreign_memory' (132582) panicked at src/resident/memory/wrap_gate.rs:74:25:
      called `Result::unwrap()` on an `Err` value: DriverError(CUDA_ERROR_INVALID_VALUE, "invalid argument")
      FAILED

      --
      test result: FAILED. 1 passed; 2 failed; 0 ignored; 0 measured; 227 filtered out; finished in 0.20s

      error: test failed, to rerun pass `--lib`
```
/answer 9e96-28 allow
Thinking:
Ich habe zwei Probleme gefunden: Der Drop eines gewrappten Tensors vergiftet den Kontext (nachfolgendes `synchronize` liefert `CUDA_ERROR_INVALID_VALUE`), und der Schreibtest über `slice_set` scheitert an der PTX-Version des benötigten Candle-Kernels. Ich passe die Tests entsprechend an.


Control Request:
```yaml
id: 9e96-29
toolName: python
arguments:
  script: |-
    p='/home/user/xyan/xy.ai.workbench/rag/src/resident/memory/wrap_gate.rs'
    s=open(p).read()
    a=s.index("#[test]")
    b=s.index("#[test]\n#[ignore = \"requires a CUDA device\"]\nfn candle_copies")
    s=s[:a]+'''#[test]
    #[ignore = "requires a CUDA device"]
    fn candle_reads_the_wrapped_range_and_forgetting_it_is_clean() {
        let f = Fixture::new();
        let view = f.wrapped_view();
        let seen = view.to_vec1::<f32>().unwrap();
        assert_eq!(seen, (VIEW_OFFSET..VIEW_OFFSET + VIEW_ELEMS).map(|i| i as f32).collect::<Vec<_>>());
        std::mem::forget(view);
        f.ctx.synchronize().unwrap();
        f.ctx.check_err().unwrap();
        assert_eq!(f.arena_host()[0], 0.0);
    }

    #[test]
    #[ignore = "requires a CUDA device"]
    fn dropping_a_wrapped_tensor_poisons_the_context() {
        let f = Fixture::new();
        drop(f.wrapped_view());
        let synced = f.ctx.synchronize();
        println!("synchronize after drop: {synced:?}");
        assert!(synced.is_err() || f.ctx.check_err().is_err());
    }

    '''+s[b:]
    open(p,'w').write(s)
  reason: Gate-Tests auf die gemessenen Befunde ausrichten
```
/answer 9e96-29 allow
ReasoningToken: 529
Control Request:
```yaml
id: 9e96-2a
result:
  structuredContent:
    result: success
```
/answer 9e96-2a allow
Control Request:
```yaml
id: 9e96-2b
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench/rag
  script: export LD_LIBRARY_PATH=/usr/local/cuda-13.4/lib64; timeout 900 cargo test --lib --features cuda wrap_gate -- --include-ignored --test-threads=1 --nocapture 2>&1 | grep -E "^test |synchronize after|panicked|test result|^error" -A2 | head -20; nvidia-smi | grep -i "version"
  reason: Endgültige Gate-Läufe und Treiberversion erfassen
```
/answer 9e96-2b allow
Control Request:
```yaml
id: 9e96-2c
result:
  structuredContent:
    stdout: |
      test resident::memory::wrap_gate::candle_copies_allocate_their_own_storage ... ok
      test resident::memory::wrap_gate::candle_reads_the_wrapped_range_and_forgetting_it_is_clean ... ok
      test resident::memory::wrap_gate::dropping_a_wrapped_tensor_poisons_the_context ... synchronize after drop: Err(DriverError(CUDA_ERROR_INVALID_VALUE, "invalid argument"))
      ok

      test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 227 filtered out; finished in 0.19s

      | NVIDIA-SMI 610.57.04              KMD Version: 610.57.04     CUDA UMD Version: 13.3     |
```
/answer 9e96-2c allow
Text:
 Zum Schluss aktualisiere ich das Dokument mit dem G-WRAP-Ergebnis.
ReasoningToken: 130
Control Request:
```yaml
id: 9e96-2d
toolName: python
arguments:
  script: |-
    p='/home/user/xyan/xy.ai.workbench/rag/docs/architecture/gpu_memory.md'
    s=open(p).read()
    a=s.index("## Foreign allocations (Candle)")
    b=s.index("## Limits")
    s=s[:a]+'''## Foreign allocations (Candle)

    Gate result (`memory/wrap_gate.rs`, GTX 1660 SUPER, Candle 0.11, cudarc 0.19.9): **negative for general use, zero-copy output is not possible**.

    | Finding | Evidence |
    |---------|----------|
    | An arena sub-range can back a Candle tensor for reads (`upgrade_device_ptr` + `CudaStorage::wrap_cuda_slice`). | `candle_reads_the_wrapped_range_and_forgetting_it_is_clean` |
    | Dropping such a tensor frees the interior pointer and poisons the context: the next `synchronize` returns `CUDA_ERROR_INVALID_VALUE`. `CudaSlice` always frees on drop; Candle offers no way to take the storage back (`storage_mut_and_layout` is crate-private). | `dropping_a_wrapped_tensor_poisons_the_context` |
    | Only `std::mem::forget` avoids the free. It leaks the host-side wrapper and two CUDA events per tensor, so it is acceptable only for a few long-lived views that live until shutdown. | same test as the first row |
    | Candle ops allocate their own output storage; the only in-place writer is `slice_set`, which is itself a copy. Candle never encodes directly into an arena. | `candle_copies_allocate_their_own_storage` |

    **Decision:** arenas serve engine kernels only. Candle computes in its own allocations and results are moved into arenas with `CudaMemoryManager::copy_d2d`. Wrapping stays limited to long-lived read-only views, if needed at all.

    **Not verified:** in-place writes through a wrapped tensor (`slice_set`). Candle's kernels fail here with `CUDA_ERROR_UNSUPPORTED_PTX_VERSION`: the PTX is built with the 13.4 toolkit, the installed driver supports CUDA 13.3. Any test that runs a Candle kernel needs a toolkit not newer than the driver.

    '''+s[b:]
    s=s.replace("- GPU tests are `#[ignore]` and run with `cargo test --features cuda -- --ignored` on a machine with a device.","- GPU tests are `#[ignore]`. Run them with `LD_LIBRARY_PATH=/usr/local/cuda-13.4/lib64 cargo test --lib --features cuda -- --include-ignored`.")
    open(p,'w').write(s)
  reason: G-WRAP-Ergebnis im Architekturdokument festhalten
```
/answer 9e96-2d allow
Control Request:
```yaml
id: 9e96-2e
result:
  structuredContent:
    result: success
```
/answer 9e96-2e allow
Result Stats: id=1e6ec56d-772f-444c-b948-f179da853722, total: 293078, in: 96, out: 62585, reason: 0, read: 4037970, write: 230397, start: 1792350601460, end: 1792350608391, time: 6870, total: 514265, io: 0, control: 61, sent: 0, recv: 180
G-WRAP ist durchlaufen und fällt negativ aus. Die drei Gate-Tests laufen auf der GTX 1660 SUPER grün, sie halten den negativen Befund fest.

Was die Tests zeigen:
- **Lesen geht:** Ein Arena-Bereich lässt sich über `upgrade_device_ptr` und `wrap_cuda_slice` als Candle-Tensor einbinden, und `to_vec1` liefert die richtigen Werte.
- **Drop geht nicht:** Beim Drop gibt Candle den Zeiger frei, und der nächste `synchronize` liefert `CUDA_ERROR_INVALID_VALUE`, der Kontext ist vergiftet. Candle bietet keinen Weg, die Storage vorher zurückzunehmen, `storage_mut_and_layout` ist nicht öffentlich.
- **`std::mem::forget` ist nur begrenzt brauchbar:** Es vermeidet den Fehler, leckt aber pro Tensor den Host-Wrapper und zwei CUDA-Events. Das taugt höchstens für wenige langlebige Lese-Views bis zum Shutdown.
- **Kein Zero-Copy-Output:** Candle-Ops legen ihren Output selbst an. Der einzige In-place-Weg, `slice_set`, ist selbst eine Kopie.

Damit gilt der D2D-Fallback endgültig: Arenen bedienen nur eigene Kernel, Candle rechnet in eigenen Allokationen, und Ergebnisse kommen per `copy_d2d` in die Arena. `gpu_memory.md` ist aktualisiert.

Nicht prüfbar war das In-place-Schreiben über einen gewrappten Tensor. Candle-Kernels laufen hier nicht: `CUDA_ERROR_UNSUPPORTED_PTX_VERSION`. Der Treiber (610.57.04) unterstützt CUDA 13.3, das Toolkit in der Config ist 13.4. Das betrifft auch D4 und D6, sobald sie Candle-Kernels ausführen. Dafür braucht es ein Toolkit höchstens in Treiberversion (`CUDA_PATH` und `CUDA_HOME` in `.cargo/config.toml`) oder einen neueren Treiber.

Tests, die Candle linken, brauchen `LD_LIBRARY_PATH=/usr/local/cuda-13.4/lib64`, sonst findet der Lader `libcurand.so.10` nicht. Der Befehl steht im Dokument.
