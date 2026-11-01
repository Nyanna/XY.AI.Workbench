Setze D8 um.

[include](/home/user/xyan/xy.ai.workbench/project/resident/index.md)
[include](/home/user/xyan/xy.ai.workbench/project/resident/plan/plan_d_00_overview.md)

- minimiere die Kommentare und keine direkte Referenzierung von Planelementen in Kommentaren
- Projektdokumente immer in englisch ausgeben
- nvcc fehlt nicht und ist im Projekt konfiguriert
- ziehe AST-Tools und spezifische Tools vor

SystemInit:  id=8d4ede28-0fa1-4514-8f48-6cf26f4bca58, cwd=/home/user/xyan/xy.ai.workbench/project, model=claude-sonnet-5-5
Text:
 I'll start by inspecting the current state of the resident module.
Control Request:
```yaml
id: b040-1
toolName: list
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident
  - path: /home/user/xyan/xy.ai.workbench/rag/tests
  - path: /home/user/xyan/xy.ai.workbench/rag/docs
  reason: List existing resident module files to see where D8 artefacts fit
```
/answer b040-1 allow
ReasoningToken: 26
Control Request:
```yaml
id: b040-2
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
      - "./kernel: (11 files)"
      - cosine.rs
      - "./manager: (1 file)"
      - mod.rs
      - "./memory: (13 files)"
      - arena.rs
      - "./model: (8 files)"
      - batch.rs
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
      - "./segment: (5 files)"
      - error.rs
      - "./subengine: (6 files)"
      - device.rs
      - "./tokenize: (4 files)"
      - error.rs
      warning: Result reduced because the total number of entries (95) exceeds the limit of 50. Narrow down the result using the 'pattern' regular expression parameter.
    - path: /home/user/xyan/xy.ai.workbench/rag/tests
      entries:
      - ".:"
      - deps_smoke.rs
      - trigram_e2e.rs
      - "./data/reference:"
      - multilingual-e5-small.json
    - path: /home/user/xyan/xy.ai.workbench/rag/docs
      entries:
      - "./architecture:"
      - conventions.md
      - dependency_verification.md
      - feature_matrix.md
      - gpu_memory.md
      - scope_charta.md
```
/answer b040-2 allow
Control Request:
```yaml
id: b040-3
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench/rag
  script: cat Cargo.toml; git status --short | head -20; cat src/lib.rs | head -50
  reason: Read Cargo features and lib.rs module wiring
```
/answer b040-3 deny
Control Request:
```yaml
id: b040-4
toolName: list
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident
    pattern: \.rs$
  reason: Full file listing of resident module (previous result was truncated)
```
/answer b040-4 allow
Control Request:
```yaml
id: b040-5
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
      - "./kernel: (11 files)"
      - cosine.rs
      - "./manager: (1 file)"
      - mod.rs
      - "./memory: (13 files)"
      - arena.rs
      - "./model: (8 files)"
      - batch.rs
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
      - "./segment: (5 files)"
      - error.rs
      - "./subengine: (6 files)"
      - device.rs
      - "./tokenize: (4 files)"
      - error.rs
      warning: Result reduced because the total number of entries (95) exceeds the limit of 50. Narrow down the result using the 'pattern' regular expression parameter.
```
/answer b040-5 allow
Control Request:
```yaml
id: b040-6
toolName: grep
arguments:
  items:
  - directory: /home/user/xyan/xy.ai.workbench/rag
    pattern: "cuda|^\\[features\\]|candle"
    include:
    - Cargo.toml
    limit: 30
  - directory: /home/user/xyan/xy.ai.workbench/rag/src
    pattern: mod resident|pub mod
    include:
    - lib.rs
    - mod.rs
    limit: 40
  reason: Locate feature flags and module declarations
```
/answer b040-6 allow
Control Request:
```yaml
id: b040-7
result:
  structuredContent:
    errors:
    - directory: /home/user/xyan/xy.ai.workbench/rag
      pattern: "cuda|^\\[features\\]|candle"
      error: directory must be an absolute path.
    - directory: /home/user/xyan/xy.ai.workbench/rag/src
      pattern: mod resident|pub mod
      error: directory must be an absolute path.
```
/answer b040-7 allow
Control Request:
```yaml
id: b040-8
toolName: list
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/subengine
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/op
  reason: "List files of the subdirectories relevant for D8 (kernel, subengine, precision, op)"
```
/answer b040-8 allow
ReasoningToken: 18
Control Request:
```yaml
id: b040-9
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel
      entries:
      - ".:"
      - cosine.rs
      - cpu_simd.rs
      - cuda.rs
      - error.rs
      - maxsim.rs
      - mock.rs
      - mod.rs
      - registry.rs
      - slice.rs
      - slots.rs
      - topk.rs
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/subengine
      entries:
      - ".:"
      - device.rs
      - error.rs
      - exec.rs
      - lifecycle.rs
      - mod.rs
      - resource.rs
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision
      entries:
      - ".:"
      - axis.rs
      - capability.rs
      - known.rs
      - mod.rs
      - pareto.rs
      - profile.rs
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/op
      entries:
      - ".:"
      - lease.rs
      - mod.rs
      - requirements.rs
```
/answer b040-9 allow
Control Request:
```yaml
id: b040-a
toolName: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag
    pattern: "cuda|^\\[features\\]|candle"
    include:
    - Cargo.toml
    limit: 30
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src
    pattern: "mod [a-z_]+;"
    include:
    - lib.rs
    - mod.rs
    limit: 50
  reason: Locate feature flags and module declarations
```
/answer b040-a allow
Control Request:
```yaml
id: b040-b
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag
      matches:
      - path: third_party/candle-transformers/Cargo.toml
        matches:
        - match: name = "candle-transformers"
        - match: repository = "https://github.com/huggingface/candle"
        - match: "[features]"
        - match: "\"candle/accelerate\","
        - match: "\"candle-nn/accelerate\","
        - match: "cuda = ["
        - match: "\"candle/cuda\","
        - match: "\"candle-nn/cuda\","
        - match: "\"candle/cudnn\","
        - match: "\"candle-nn/cudnn\","
        - match: "\"cuda\","
        - match: "\"dep:candle-flash-attn\","
        - match: "\"candle/metal\","
        - match: "\"candle-nn/metal\","
        - match: "\"candle/metal-debug-labels\","
        - match: "\"candle-nn/metal-debug-labels\","
        - match: "\"candle/mkl\","
        - match: "\"candle-nn/mkl\","
        - match: name = "candle_transformers"
        - match: "[dependencies.candle]"
        - match: package = "candle-core"
        - match: "[dependencies.candle-flash-attn]"
        - match: "[dependencies.candle-nn]"
      - path: third_party/candle-kernels/Cargo.toml
        matches:
        - match: name = "candle-kernels"
        - match: repository = "https://github.com/huggingface/candle"
        - match: name = "candle_kernels"
        - match: "[build-dependencies.cudaforge]"
      - path: Cargo.toml
        matches:
        - match: "candle-core = { version = \"0.11.0\", optional = true, default-features = false }"
        - match: "candle-nn = { version = \"0.11.0\", optional = true, default-features = false }"
        - match: "candle-transformers = { version = \"0.11.0\", optional = true, default-features = false }"
      warning: "Limit of 30 matches reached; further results may exist. Narrow the pattern, directory or include/exclude filters, or raise limit."
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src
      matches:
      - path: lib.rs
        matches:
        - match: pub mod core;
        - match: pub mod layers;
        - match: pub mod resident;
      - path: resident/mod.rs
        matches:
        - match: pub mod backend;
        - match: pub mod error;
        - match: pub mod failpoint;
        - match: pub mod telemetry;
        - match: pub mod mock;
        - match: pub mod queue;
        - match: pub mod memory;
        - match: pub mod op;
        - match: pub mod index;
        - match: pub mod kernel;
        - match: pub mod precision;
        - match: pub mod facade;
        - match: pub mod processor;
        - match: pub mod planner;
        - match: pub mod subengine;
        - match: pub mod model;
        - match: pub mod tokenize;
        - match: pub mod manager;
        - match: pub mod segment;
      - path: resident/segment/mod.rs
        matches:
        - match: pub mod error;
        - match: pub mod mapping;
        - match: pub mod slots;
        - match: pub mod sync;
      - path: resident/processor/mod.rs
        matches:
        - match: mod completion;
        - match: mod completion_cuda;
        - match: mod config;
        - match: mod error;
        - match: mod hub;
        - match: mod ops;
        - match: mod resource;
        - match: mod starved;
        - match: mod worker;
        - match: mod tests;
      - path: resident/precision/mod.rs
        matches:
        - match: mod axis;
        - match: mod capability;
        - match: pub mod known;
        - match: mod pareto;
        - match: mod profile;
      - path: resident/facade/mod.rs
        matches:
        - match: mod blocking;
        - match: mod error;
        - match: mod handle;
        - match: mod params;
        - match: mod resident;
        - match: mod task;
      - path: resident/index/mod.rs
        matches:
        - match: pub mod memory;
        - match: pub mod stamp;
      - path: resident/queue/mod.rs
        matches:
        - match: mod node;
      warning: "Limit of 50 matches reached; further results may exist. Narrow the pattern, directory or include/exclude filters, or raise limit."
```
/answer b040-b allow
Control Request:
```yaml
id: b040-c
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel/mod.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel/registry.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel/maxsim.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel/slots.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel/error.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/mod.rs
  reason: Understand kernel module APIs to design the launcher with Direct/Capture decorator
```
/answer b040-c allow
Control Request:
```yaml
id: b040-d
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel/mod.rs
      nodes:
      - id: ModuleDoc
        signature: "//! Kernel abstraction: segment descriptors, hits, the `Kernel` trait and the s…"
      - id: cosine
        signature: pub mod cosine;
      - id: cpu_simd
        signature: pub mod cpu_simd;
      - id: cuda
        signature: pub mod cuda;
      - id: error
        signature: pub mod error;
      - id: maxsim
        signature: pub mod maxsim;
      - id: registry
        signature: pub mod registry;
      - id: slice
        signature: pub mod slice;
      - id: slots
        signature: pub mod slots;
      - id: topk
        signature: pub mod topk;
      - id: 1jD1e9|HeHd71
        signature: pub use error::KernelError;
      - id: mock
        signature: "#[cfg(any(test, feature = \"mock\"))]"
      - id: Dh3quH|ljFcuS
        signature: use std::time::Duration;
      - id: RegionSpan
        signature: /// Address-agnostic arena span; pointer resolution happens only inside a backe…
        docstring: Address-agnostic arena span; pointer resolution happens only inside a backend.
      - id: OffsetRef
        signature: /// Offset table (`n + 1` prefix sums) living in an arena span.
        docstring: Offset table (`n + 1` prefix sums) living in an arena span.
      - id: SegmentLayout
        signature: "#[derive(Clone, Copy, Debug, PartialEq, Eq)] pub enum SegmentLayout {"
      - id: SegmentDesc
        signature: "#[derive(Clone, Copy, Debug, PartialEq, Eq)] pub struct SegmentDesc {"
      - id: KernelHit
        signature: "#[derive(Clone, Copy, Debug, PartialEq)] pub struct KernelHit {"
      - id: WrittenRange
        signature: "#[derive(Clone, Copy, Debug, PartialEq, Eq)] pub struct WrittenRange {"
      - id: BatchDesc
        signature: "#[derive(Clone, Debug, Default)] pub struct BatchDesc {"
      - id: BatchId
        signature: "#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]"
      - id: impl_BatchId
        signature: "impl BatchId {"
        children:
        - id: impl_BatchId.get
          signature: "pub fn get(self) -> u64 {"
      - id: KernelOutput
        signature: "#[derive(Clone, Debug, PartialEq)] pub enum KernelOutput {"
      - id: Kernel
        signature: "pub trait Kernel: Send + Sync {"
        children:
        - id: Kernel.begin
          signature: "fn begin(&self, batch: &BatchDesc) -> Result<BatchId, IndexError>;"
        - id: Kernel.run
          signature: fn run(
        - id: Kernel.progress
          signature: /// Monotonically non-decreasing.
          docstring: Monotonically non-decreasing.
        - id: Kernel.end
          signature: "fn end(&self, batch: BatchId) -> Result<Vec<WrittenRange>, IndexError>;"
      - id: entry_of_fixed
        signature: "/// Entry index for `addr` in a `Fixed` segment. pub fn entry_of_fixed(base: u6…"
        docstring: Entry index for `addr` in a `Fixed` segment.
      - id: entry_of
        signature: /// Entry index for `addr` given `n + 1` ascending offsets; entry `i` covers //…
        docstring: Entry index for `addr` given `n + 1` ascending offsets; entry `i` covers
      - id: SliceRange
        signature: /// Half-open slot range of one segment; the unit of work for a single launch. …
        docstring: Half-open slot range of one segment; the unit of work for a single launch.
      - id: impl_SliceRange
        signature: "impl SliceRange {"
        children:
        - id: impl_SliceRange.len
          signature: "pub fn len(&self) -> u32 {"
        - id: impl_SliceRange.is_empty
          signature: "pub fn is_empty(&self) -> bool {"
        - id: impl_SliceRange.split_at
          signature: "/// Splits off at most `n` leading slots. pub fn split_at(self, n: u32) -> (Sli…"
          docstring: Splits off at most `n` leading slots.
      - id: QueryBatch
        signature: /// Queries scored together so every document tile is read once. Each query is …
        docstring: Queries scored together so every document tile is read once. Each query is a ro…
      - id: impl_QueryBatch
        signature: "impl QueryBatch {"
        children:
        - id: impl_QueryBatch.new
          signature: "pub fn new(dim: usize, queries: Vec<Vec<f32>>) -> Result<Self, KernelError> {"
        - id: impl_QueryBatch.dim
          signature: "pub fn dim(&self) -> usize {"
        - id: impl_QueryBatch.len
          signature: "pub fn len(&self) -> usize {"
        - id: impl_QueryBatch.is_empty
          signature: "pub fn is_empty(&self) -> bool {"
        - id: impl_QueryBatch.tokens
          signature: "pub fn tokens(&self, i: usize) -> usize {"
        - id: impl_QueryBatch.data
          signature: "pub fn data(&self, i: usize) -> &[f32] {"
      - id: SlotSource
        signature: /// Host-side view of slot vectors for CPU scoring (`F32`). pub trait SlotSourc…
        docstring: Host-side view of slot vectors for CPU scoring (`F32`).
        children:
        - id: SlotSource.dim
          signature: fn dim(&self) -> usize;
        - id: SlotSource.slot_count
          signature: "fn slot_count(&self, segment: u32) -> u32;"
        - id: SlotSource.slot_offset
          signature: /// Offset reported in `KernelHit` for this slot.
          docstring: Offset reported in `KernelHit` for this slot.
        - id: SlotSource.slot_vectors
          signature: "/// Row-major `tokens x dim`, valid tokens only."
          docstring: "Row-major `tokens x dim`, valid tokens only."
      - id: SlotScorer
        signature: /// Raw scores per slot; the top-k reduction is shared in `TopKScorer`. pub tra…
        docstring: Raw scores per slot; the top-k reduction is shared in `TopKScorer`.
        children:
        - id: SlotScorer.scores
          signature: "/// One vector per query, one score per slot of `range` in slot order."
          docstring: "One vector per query, one score per slot of `range` in slot order."
        - id: SlotScorer.slot_offset
          signature: "fn slot_offset(&self, segment: u32, slot: u32) -> u64;"
      - id: Scorer
        signature: /// Scores one slice for all queries of a batch; the result holds the slice-loc…
        docstring: Scores one slice for all queries of a batch; the result holds the slice-local t…
        children:
        - id: Scorer.score
          signature: fn score(
      - id: TopKScorer
        signature: "#[derive(Debug)] pub struct TopKScorer<S> {"
      - id: impl_TopKScorer_S_
        signature: "impl<S: SlotScorer> TopKScorer<S> {"
        children:
        - id: impl_TopKScorer_S_.new
          signature: "pub fn new(inner: S, k: usize) -> Self {"
        - id: impl_TopKScorer_S_.k
          signature: "pub fn k(&self) -> usize {"
      - id: impl_Scorer_for_TopKScorer_S_
        signature: "impl<S: SlotScorer> Scorer for TopKScorer<S> {"
        children:
        - id: impl_Scorer_for_TopKScorer_S_.score
          signature: "fn score(     &self,     queries: &QueryBatch,     range: SliceRange,     _lane…"
      - id: score_ranges
        signature: /// Scores several slices (e.g. a scope's ranges) and merges them into the glob…
        docstring: Scores several slices (e.g. a scope's ranges) and merges them into the global t…
      - id: tests
        signature: "#[cfg(test)] mod tests {"
        children:
        - id: tests.ngr4Rc|EbhtEg
          signature: "use super::mock::{Call, MockKernel};"
        - id: tests.spans
          signature: "fn spans(n: u64) -> Vec<RegionSpan> {"
        - id: tests.desc
          signature: "fn desc() -> BatchDesc {"
        - id: tests.entry_of_fixed_bounds
          signature: "#[test] fn entry_of_fixed_bounds() {"
        - id: tests.entry_of_variable_bounds
          signature: "#[test] fn entry_of_variable_bounds() {"
        - id: tests.scoring_is_deterministic
          signature: "#[test] fn scoring_is_deterministic() {"
        - id: tests.partial_then_follow_up_range_covers_input
          signature: "#[test] fn partial_then_follow_up_range_covers_input() {"
        - id: tests.progress_is_monotonic
          signature: "#[test] fn progress_is_monotonic() {"
        - id: tests.injected_failure_leaves_batch_endable
          signature: "#[test] fn injected_failure_leaves_batch_endable() {"
        - id: tests.usable_as_trait_object
          signature: "#[test] fn usable_as_trait_object() {"
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel/registry.rs
      nodes:
      - id: ModuleDoc
        signature: "//! Lazily populated kernel table keyed by `(name, dtype, sm, variant)`."
      - id: 5qa36T|ZReeg2
        signature: use std::collections::HashMap;
      - id: KernelKey
        signature: /// `variant` selects compile-time specialisations such as the embedding dimens…
        docstring: '`variant` selects compile-time specialisations such as the embedding dimension.'
      - id: impl_KernelKey
        signature: "impl KernelKey {"
        children:
        - id: impl_KernelKey.new
          signature: "pub const fn new(name: &'static str, dtype: DType, sm: u32, variant: u32) -> Se…"
      - id: KernelLoader
        signature: "/// Produces the handle for a key, e.g. by compiling with NVRTC. pub trait Kern…"
        docstring: "Produces the handle for a key, e.g. by compiling with NVRTC."
        children:
        - id: KernelLoader.load
          signature: "fn load(&self, key: &KernelKey) -> Result<F, KernelError>;"
      - id: KernelRegistry
        signature: "pub struct KernelRegistry<F> {"
      - id: impl_KernelRegistry_F_
        signature: "impl<F: Clone + Send + Sync + 'static> KernelRegistry<F> {"
        children:
        - id: impl_KernelRegistry_F_.new
          signature: "pub fn new(loader: Arc<dyn KernelLoader<F>>) -> Self {"
        - id: impl_KernelRegistry_F_.get
          signature: "/// Loads on first use. Loading holds the write lock, so concurrent callers nev…"
          docstring: "Loads on first use. Loading holds the write lock, so concurrent callers never c…"
        - id: impl_KernelRegistry_F_.len
          signature: "pub fn len(&self) -> usize {"
        - id: impl_KernelRegistry_F_.is_empty
          signature: "pub fn is_empty(&self) -> bool {"
        - id: impl_KernelRegistry_F_.evict_all
          signature: /// Drops every loaded kernel; the next `get` reloads. pub fn evict_all(&self) …
          docstring: Drops every loaded kernel; the next `get` reloads.
      - id: impl_Resource_for_KernelRegistry_F_
        signature: "impl<F: Clone + Send + Sync + 'static> Resource for KernelRegistry<F> {"
        children:
        - id: impl_Resource_for_KernelRegistry_F_.key
          signature: "fn key(&self) -> ResourceKey {"
      - id: tests
        signature: "#[cfg(test)] mod tests {"
        children:
        - id: tests.rdzZkt|1i9wVi
          signature: use super::*;
        - id: tests.Counting
          signature: struct Counting(AtomicUsize);
        - id: tests.impl_KernelLoader_u32__for_Counting
          signature: "impl KernelLoader<u32> for Counting {"
          children:
          - id: tests.impl_KernelLoader_u32__for_Counting.load
            signature: "fn load(&self, key: &KernelKey) -> Result<u32, KernelError> {"
        - id: tests.registry
          signature: "fn registry() -> (Arc<Counting>, KernelRegistry<u32>) {"
        - id: tests.loads_once_per_key
          signature: "#[test] fn loads_once_per_key() {"
        - id: tests.failed_load_is_not_cached
          signature: "#[test] fn failed_load_is_not_cached() {"
        - id: tests.evict_forces_reload
          signature: "#[test] fn evict_forces_reload() {"
        - id: tests.concurrent_gets_load_once
          signature: "#[test] fn concurrent_gets_load_once() {"
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel/maxsim.rs
      nodes:
      - id: ModuleDoc
        signature: //! Late-interaction scoring (ColBERT-style MaxSim) behind one op; the implemen…
      - id: n7LbZi|EGLLHI
        signature: use std::sync::Arc;
      - id: impl_SlotScorer_for_Box_dynSlotScorer_
        signature: "impl SlotScorer for Box<dyn SlotScorer> {"
        children:
        - id: impl_SlotScorer_for_Box_dynSlotScorer_.scores
          signature: "fn scores(     &self,     queries: &QueryBatch,     range: SliceRange, ) -> Res…"
        - id: impl_SlotScorer_for_Box_dynSlotScorer_.slot_offset
          signature: "fn slot_offset(&self, segment: u32, slot: u32) -> u64 {"
      - id: MaxSimOp
        signature: "pub struct MaxSimOp {"
      - id: impl_MaxSimOp
        signature: "impl MaxSimOp {"
        children:
        - id: impl_MaxSimOp.with_slots
          signature: "pub fn with_slots(slots: Box<dyn SlotScorer>, k: usize) -> Self {"
        - id: impl_MaxSimOp.cpu
          signature: "pub fn cpu(source: Arc<dyn SlotSource>, k: usize) -> Self {"
        - id: impl_MaxSimOp.candle_cpu
          signature: "#[cfg(feature = \"candle\")] pub fn candle_cpu(     source: Arc<dyn SlotSource>, …"
        - id: impl_MaxSimOp.candle_cuda
          signature: "#[cfg(feature = \"cuda\")] pub fn candle_cuda(     source: Arc<dyn SlotSource>,  …"
      - id: impl_Scorer_for_MaxSimOp
        signature: "impl Scorer for MaxSimOp {"
        children:
        - id: impl_Scorer_for_MaxSimOp.score
          signature: "fn score(     &self,     queries: &QueryBatch,     range: SliceRange,     lane:…"
      - id: candle_chain
        signature: "#[cfg(feature = \"candle\")] pub(crate) mod candle_chain {"
        children:
        - id: candle_chain.PP0dCN|ByWQK5
          signature: use std::sync::Arc;
        - id: candle_chain.fail
          signature: "fn fail(e: candle_core::Error) -> KernelError {"
        - id: candle_chain.Tile
          signature: "struct Tile {"
        - id: candle_chain.CandleMaxSim
          signature: "/// `matmul -> mask -> max` per document tile with a running maximum, then a su…"
          docstring: "`matmul -> mask -> max` per document tile with a running maximum, then a sum ov…"
        - id: candle_chain.impl_CandleMaxSim
          signature: "impl CandleMaxSim {"
          children:
          - id: candle_chain.impl_CandleMaxSim.new
            signature: "pub fn new(     source: Arc<dyn SlotSource>,     device: Device,     dtype: DTy…"
          - id: candle_chain.impl_CandleMaxSim.tiles
            signature: "fn tiles(&self, doc: &[f32], dim: usize) -> candle_core::Result<Vec<Tile>> {"
          - id: candle_chain.impl_CandleMaxSim.score
            signature: "fn score(&self, query: &Tensor, tiles: &[Tile]) -> candle_core::Result<f32> {"
        - id: candle_chain.impl_SlotScorer_for_CandleMaxSim
          signature: "impl SlotScorer for CandleMaxSim {"
          children:
          - id: candle_chain.impl_SlotScorer_for_CandleMaxSim.scores
            signature: "fn scores(     &self,     queries: &QueryBatch,     range: SliceRange, ) -> Res…"
          - id: candle_chain.impl_SlotScorer_for_CandleMaxSim.slot_offset
            signature: "fn slot_offset(&self, segment: u32, slot: u32) -> u64 {"
      - id: tests
        signature: "#[cfg(test)] mod tests {"
        children:
        - id: tests.Gcn6TU|mqRXRB
          signature: use super::*;
        - id: tests.lane
          signature: "fn lane() -> RecordingLane {"
        - id: tests.tokens
          signature: "fn tokens(n: usize, dim: usize, seed: u32) -> Vec<f32> {"
        - id: tests.source
          signature: "fn source(dim: usize) -> Arc<InMemorySlots> {"
        - id: tests.Va0TiE|TMyn9X
          signature: "const R: SliceRange = SliceRange {"
        - id: tests.cpu_op_ranks_by_maxsim_and_empty_doc_is_last
          signature: "#[test] fn cpu_op_ranks_by_maxsim_and_empty_doc_is_last() {"
        - id: tests.cpu_op_top_k_limits_results
          signature: "#[test] fn cpu_op_top_k_limits_results() {"
        - id: tests.candle
          signature: "#[cfg(feature = \"candle\")] mod candle {"
          children:
          - id: tests.candle.SPNbfC|IqrZCX
            signature: use super::*;
          - id: tests.candle.per_slot
            signature: "fn per_slot(op: &dyn SlotScorer, q: &QueryBatch) -> Vec<Vec<f32>> {"
          - id: tests.candle.f32_tile_chain_matches_cpu_reference_incl_padding
            signature: "#[test] fn f32_tile_chain_matches_cpu_reference_incl_padding() {"
          - id: tests.candle.f16_masking_never_produces_nan
            signature: "#[test] fn f16_masking_never_produces_nan() {"
          - id: tests.candle.op_wraps_candle_chain
            signature: "#[test] fn op_wraps_candle_chain() {"
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel/slots.rs
      nodes:
      - id: ModuleDoc
        signature: //! In-memory `SlotSource` for CPU scoring and tests.
      - id: iPxide|WQx9yb
        signature: "use super::{KernelError, SliceRange, SlotSource};"
      - id: InMemorySlots
        signature: "#[derive(Clone, Debug)] pub struct InMemorySlots {"
      - id: impl_InMemorySlots
        signature: "impl InMemorySlots {"
        children:
        - id: impl_InMemorySlots.new
          signature: "/// `segments[s][i]` holds the row-major `tokens x dim` vectors of slot `i`. Of…"
          docstring: "`segments[s][i]` holds the row-major `tokens x dim` vectors of slot `i`. Offset…"
      - id: impl_SlotSource_for_InMemorySlots
        signature: "impl SlotSource for InMemorySlots {"
        children:
        - id: impl_SlotSource_for_InMemorySlots.dim
          signature: "fn dim(&self) -> usize {"
        - id: impl_SlotSource_for_InMemorySlots.slot_count
          signature: "fn slot_count(&self, segment: u32) -> u32 {"
        - id: impl_SlotSource_for_InMemorySlots.slot_offset
          signature: "fn slot_offset(&self, segment: u32, slot: u32) -> u64 {"
        - id: impl_SlotSource_for_InMemorySlots.slot_vectors
          signature: "fn slot_vectors(&self, segment: u32, slot: u32) -> Option<&[f32]> {"
      - id: check_range
        signature: "pub(crate) fn check_range(     source: &dyn SlotSource,     range: SliceRange, …"
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel/error.rs
      nodes:
      - id: CLmNmB|3V6thW
        signature: use thiserror::Error;
      - id: KernelError
        signature: "/// Kernel registry, scoring and launch errors. #[derive(Error, Debug)] #[non_e…"
        docstring: "Kernel registry, scoring and launch errors."
      - id: impl_IsRetryable_for_KernelError
        signature: "impl IsRetryable for KernelError {"
        children:
        - id: impl_IsRetryable_for_KernelError.is_retryable
          signature: "fn is_retryable(&self) -> bool {"
      - id: impl_From_KernelError__for_IndexError
        signature: "impl From<KernelError> for IndexError {"
        children:
        - id: impl_From_KernelError__for_IndexError.from
          signature: "fn from(e: KernelError) -> Self {"
      - id: impl_From_crate__resident__subengine__DeviceError__for_KernelError
        signature: "impl From<crate::resident::subengine::DeviceError> for KernelError {"
        children:
        - id: impl_From_crate__resident__subengine__DeviceError__for_KernelError.from
          signature: "fn from(e: crate::resident::subengine::DeviceError) -> Self {"
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/mod.rs
      nodes:
      - id: ModuleDoc
        signature: "//! Resident engine: embedding/similarity substrate, kept warm."
      - id: backend
        signature: pub mod backend;
      - id: error
        signature: pub mod error;
      - id: failpoint
        signature: pub mod failpoint;
      - id: telemetry
        signature: pub mod telemetry;
      - id: mock
        signature: "#[cfg(any(test, feature = \"mock\"))]"
      - id: queue
        signature: pub mod queue;
      - id: memory
        signature: pub mod memory;
      - id: op
        signature: pub mod op;
      - id: index
        signature: pub mod index;
      - id: kernel
        signature: pub mod kernel;
      - id: precision
        signature: pub mod precision;
      - id: facade
        signature: pub mod facade;
      - id: processor
        signature: pub mod processor;
      - id: planner
        signature: pub mod planner;
      - id: subengine
        signature: pub mod subengine;
      - id: model
        signature: pub mod model;
      - id: tokenize
        signature: pub mod tokenize;
      - id: manager
        signature: pub mod manager;
      - id: segment
        signature: pub mod segment;
```
/answer b040-d allow
Control Request:
```yaml
id: b040-e
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel/mod.rs
    ids:
    - ModuleDoc
    - Scorer
    - SliceRange
    - mock
    - tests.spans
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel/error.rs
    ids:
    - KernelError
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel/maxsim.rs
    ids:
    - impl_Scorer_for_MaxSimOp
    - MaxSimOp
    - tests.lane
    - tests.source
    - tests.tokens
  reason: "Read Scorer trait, KernelError, and MaxSimOp scoring plus test helpers to model the launcher and golden harness"
```
/answer b040-e allow
ReasoningToken: 49
Control Request:
```yaml
id: b040-f
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel/mod.rs
      nodes:
      - id: ModuleDoc
        type: ModuleAttrs
        code: |
          //! Kernel abstraction: segment descriptors, hits, the `Kernel` trait and the scorers built on it.
          //!
          //! The kernel knows only segments, offsets and spans. Mapping an address back to an entry is the
          //! manager's job (`entry_of`, `entry_of_fixed`); the public API carries no pointers.
      - id: Scorer
        type: ItemTrait
        signature: /// Scores one slice for all queries of a batch; the result holds the slice-loc…
        docstring: Scores one slice for all queries of a batch; the result holds the slice-local t…
        children:
        - id: Scorer.score
          type: TraitItemFn
          code: |2
                fn score(
                    &self,
                    queries: &QueryBatch,
                    range: SliceRange,
                    lane: &mut dyn ExecContext,
                ) -> Result<Vec<Vec<KernelHit>>, KernelError>;
      - id: SliceRange
        type: ItemStruct
        code: |
          /// Half-open slot range of one segment; the unit of work for a single launch.
          #[derive(Clone, Copy, Debug, PartialEq, Eq)]
          pub struct SliceRange {
              pub segment: u32,
              pub slot_from: u32,
              pub slot_to: u32,
          }
      - id: mock
        type: ItemMod
        code: |
          #[cfg(any(test, feature = "mock"))]
          pub mod mock;
      - id: tests.spans
        type: ItemFn
        code: |2
              fn spans(n: u64) -> Vec<RegionSpan> {
                  (0..n)
                      .map(|i| RegionSpan {
                          offset: i * 64,
                          len: 64,
                      })
                      .collect()
              }
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel/error.rs
      nodes:
      - id: KernelError
        type: ItemEnum
        code: |
          /// Kernel registry, scoring and launch errors.
          #[derive(Error, Debug)]
          #[non_exhaustive]
          pub enum KernelError {
              #[error("kernel compilation failed: {log}")]
              Compile { log: String },
              #[error("kernel module load failed: {reason}")]
              ModuleLoad { reason: String },
              #[error("kernel function '{name}' not found")]
              FuncNotFound { name: String },
              #[error("kernel launch failed")]
              LaunchFailed {
                  #[source]
                  source: Box<dyn std::error::Error + Send + Sync>,
              },
              #[error("parameter block overflow: {requested} bytes requested, capacity {capacity}")]
              ParamBlockOverflow { requested: usize, capacity: usize },
              #[error("shape mismatch: expected {expected} elements, got {actual}")]
              ShapeMismatch { expected: usize, actual: usize },
              #[error("slice {segment}:{slot_from}..{slot_to} exceeds the segment's {count} slots")]
              SliceOutOfRange { segment: u32, slot_from: u32, slot_to: u32, count: u32 },
              #[error("kernel backend unavailable: {reason}")]
              Unavailable { reason: String },
          }
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel/maxsim.rs
      nodes:
      - id: impl_Scorer_for_MaxSimOp
        type: ItemImpl
        signature: "impl Scorer for MaxSimOp {"
        children:
        - id: impl_Scorer_for_MaxSimOp.score
          type: ImplItemFn
          code: |2
                fn score(
                    &self,
                    queries: &QueryBatch,
                    range: SliceRange,
                    lane: &mut dyn ExecContext,
                ) -> Result<Vec<Vec<KernelHit>>, KernelError> {
                    self.inner.score(queries, range, lane)
                }
      - id: MaxSimOp
        type: ItemStruct
        code: |
          pub struct MaxSimOp {
              inner: TopKScorer<Box<dyn SlotScorer>>,
          }
      - id: tests.lane
        type: ItemFn
        code: |2
              fn lane() -> RecordingLane {
                  let scratch = Region {
                      arena: ArenaId(0),
                      offset: 0,
                      len: 0,
                      gen: 0,
                  };
                  RecordingLane::new(LaneClass::Query, LaneConfig::new(scratch))
              }
      - id: tests.source
        type: ItemFn
        code: |2
              fn source(dim: usize) -> Arc<InMemorySlots> {
                  let docs = vec![
                      tokens(5, dim, 1), tokens(12, dim, 2), tokens(1, dim, 3), Vec::new(),
                      tokens(8, dim, 4)
                  ];
                  Arc::new(InMemorySlots::new(dim, vec![docs]))
              }
      - id: tests.tokens
        type: ItemFn
        code: |2
              fn tokens(n: usize, dim: usize, seed: u32) -> Vec<f32> {
                  (0..n * dim)
                      .map(|i| {
                          (((i as u32).wrapping_mul(2654435761).wrapping_add(seed * 97)) % 1000)
                              as f32 / 500.0 - 1.0
                      })
                      .collect()
              }
```
/answer b040-f allow
Control Request:
```yaml
id: b040-10
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel/cuda.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel/cpu_simd.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/subengine/exec.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/subengine/mod.rs
  reason: Understand ExecContext/lane and CUDA kernel module for launcher integration
```
/answer b040-10 allow
Control Request:
```yaml
id: b040-11
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel/cuda.rs
      nodes:
      - id: ModuleDoc
        signature: "//! Real CUDA kernels / scorer: NVRTC-compiled sources behind the kernel regist…"
      - id: MZne2K|b8Bb3B
        signature: "use std::ffi::{c_char, CString};"
      - id: kA9lm6|Qv4RkF
        signature: "pub const COSINE_DOT: &str = \"cosine_dot\";"
      - id: zBTwsZ|GsLJG7
        signature: /// One block per slot; scores every query against the slot's single vector. Ou…
      - id: QxmP5v|fHa4nk
        signature: "/// One block per document, one query. Documents are padded to `doc_tokens`; `d…"
      - id: body
        signature: "fn body(name: &str) -> Option<&'static str> {"
      - id: defines
        signature: "fn defines(key: &KernelKey) -> Result<Vec<String>, KernelError> {"
      - id: log_of
        signature: "fn log_of(prog: cudarc::nvrtc::sys::nvrtcProgram) -> String {"
      - id: compile_cubin
        signature: /// Machine code for exactly `sm_<sm>`; usable when the driver cannot JIT the P…
        docstring: Machine code for exactly `sm_<sm>`; usable when the driver cannot JIT the PTX v…
      - id: NvrtcLoader
        signature: /// Compiles to PTX for `compute_<sm>` and falls back to a CUBIN when the drive…
        docstring: Compiles to PTX for `compute_<sm>` and falls back to a CUBIN when the driver re…
      - id: impl_NvrtcLoader
        signature: "impl NvrtcLoader {"
        children:
        - id: impl_NvrtcLoader.new
          signature: "pub fn new(ctx: Arc<CudaContext>) -> Self {"
        - id: impl_NvrtcLoader.module
          signature: "fn module(     &self,     key: &KernelKey,     src: &str, ) -> Result<Arc<cudar…"
      - id: arch_for
        signature: "fn arch_for(sm: u32) -> &'static str {"
      - id: impl_KernelLoader_CudaFunction__for_NvrtcLoader
        signature: "impl KernelLoader<CudaFunction> for NvrtcLoader {"
        children:
        - id: impl_KernelLoader_CudaFunction__for_NvrtcLoader.load
          signature: "fn load(&self, key: &KernelKey) -> Result<CudaFunction, KernelError> {"
      - id: CudaKernels
        signature: "/// Registry plus the device's compute capability. pub struct CudaKernels {"
        docstring: Registry plus the device's compute capability.
      - id: impl_CudaKernels
        signature: "impl CudaKernels {"
        children:
        - id: impl_CudaKernels.new
          signature: "pub fn new(ctx: &Arc<CudaContext>) -> Result<Self, KernelError> {"
        - id: impl_CudaKernels.sm
          signature: "pub fn sm(&self) -> u32 {"
        - id: impl_CudaKernels.registry
          signature: "pub fn registry(&self) -> &KernelRegistry<CudaFunction> {"
        - id: impl_CudaKernels.function
          signature: "pub fn function(     &self,     name: &'static str,     dtype: DType,     dim: …"
      - id: launch_err
        signature: "fn launch_err(e: impl std::error::Error + Send + Sync + 'static) -> KernelError…"
      - id: 05tXPx|yv1hhd
        signature: "const BLOCK: u32 = 128;"
      - id: cfg
        signature: "fn cfg(blocks: u32) -> LaunchConfig {"
      - id: launch_cosine
        signature: "/// Output layout `n_queries x n_slots`. pub(crate) fn launch_cosine<T: DeviceR…"
        docstring: Output layout `n_queries x n_slots`.
      - id: launch_maxsim
        signature: "pub(crate) fn launch_maxsim<T: DeviceRepr>(     stream: &Arc<CudaStream>,     f…"
      - id: tests
        signature: "#[cfg(test)] mod tests {"
        children:
        - id: tests.IYLZKb|VO72MG
          signature: use super::*;
        - id: tests.wNsDct|ow87y6
          signature: "const DIM: usize = 64;"
        - id: tests.rnd
          signature: "fn rnd(n: usize, seed: u32) -> Vec<f32> {"
        - id: tests.setup
          signature: "fn setup() -> (Arc<CudaContext>, Arc<CudaStream>, CudaKernels) {"
        - id: tests.registry_compiles_for_device_sm_and_caches
          signature: "#[test] #[ignore = \"requires a CUDA device\"] fn registry_compiles_for_device_sm…"
        - id: tests.cubin_path_loads_and_runs
          signature: "#[test] #[ignore = \"requires a CUDA device\"] fn cubin_path_loads_and_runs() {"
        - id: tests.cosine_kernel_matches_cpu_for_f32_and_f16
          signature: "#[test] #[ignore = \"requires a CUDA device\"] fn cosine_kernel_matches_cpu_for_f…"
        - id: tests.fused_maxsim_matches_cpu_with_padding
          signature: "#[test] #[ignore = \"requires a CUDA device\"] fn fused_maxsim_matches_cpu_with_p…"
        - id: tests.gate_maxsim_chain_vs_fused
          signature: "/// Measurement gate: batched Candle chain (`matmul -> mask -> max -> sum` over…"
          docstring: "Measurement gate: batched Candle chain (`matmul -> mask -> max -> sum` over a w…"
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel/cpu_simd.rs
      nodes:
      - id: ModuleDoc
        signature: //! CPU scoring primitives; loops are shaped for autovectorisation (8-wide accu…
      - id: KFO4ZZ|XLcoE3
        signature: use std::sync::Arc;
      - id: dot
        signature: "pub fn dot(a: &[f32], b: &[f32]) -> f32 {"
      - id: maxsim
        signature: /// Sum over query tokens of the best dot product against any document token. A…
        docstring: Sum over query tokens of the best dot product against any document token. A doc…
      - id: normalize
        signature: "pub fn normalize(v: &mut [f32]) {"
      - id: scan_slots
        signature: "/// Slot-wise scorer: `score(query_i, slot)`; slots are the outer loop so a slo…"
        docstring: "Slot-wise scorer: `score(query_i, slot)`; slots are the outer loop so a slot is…"
      - id: CpuMaxSim
        signature: "pub struct CpuMaxSim {"
      - id: impl_CpuMaxSim
        signature: "impl CpuMaxSim {"
        children:
        - id: impl_CpuMaxSim.new
          signature: "pub fn new(source: Arc<dyn SlotSource>) -> Self {"
      - id: impl_SlotScorer_for_CpuMaxSim
        signature: "impl SlotScorer for CpuMaxSim {"
        children:
        - id: impl_SlotScorer_for_CpuMaxSim.scores
          signature: "fn scores(     &self,     queries: &QueryBatch,     range: SliceRange, ) -> Res…"
        - id: impl_SlotScorer_for_CpuMaxSim.slot_offset
          signature: "fn slot_offset(&self, segment: u32, slot: u32) -> u64 {"
      - id: tests
        signature: "#[cfg(test)] mod tests {"
        children:
        - id: tests.LwlKpn|JFVpeI
          signature: use super::*;
        - id: tests.dot_handles_remainders
          signature: "#[test] fn dot_handles_remainders() {"
        - id: tests.maxsim_matches_hand_computation
          signature: "#[test] fn maxsim_matches_hand_computation() {"
        - id: tests.empty_document_scores_mask_value_per_query_token
          signature: "#[test] fn empty_document_scores_mask_value_per_query_token() {"
        - id: tests.normalize_yields_unit_length
          signature: "#[test] fn normalize_yields_unit_length() {"
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
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/subengine/mod.rs
      nodes:
      - id: ModuleDoc
        signature: "//! Subengine resource lifecycle, pull capacity reporting, device handles and e…"
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
        - id: Subengine.device
          signature: fn device(&self) -> &DeviceHandle;
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
```
/answer b040-11 allow
Control Request:
```yaml
id: b040-12
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/subengine/exec.rs
    ids:
    - LaneJob
    - LaneClass
    - RecordingLane
    - impl_RecordingLane
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel/cuda.rs
    ids:
    - launch_cosine
    - launch_err
    - ModuleDoc
  reason: "Read LaneJob type, lane doubles and existing launch helpers to hook the launcher in properly"
```
/answer b040-12 allow
ReasoningToken: 800
Control Request:
```yaml
id: b040-13
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
      - id: LaneClass
        type: ItemEnum
        code: |
          #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
          pub enum LaneClass {
              Query,
              Index,
              Copy,
          }
      - id: RecordingLane
        type: ItemStruct
        code: |
          /// Lane double for downstream tests: runs jobs inline and logs every call.
          #[cfg(any(test, feature = "mock"))]
          pub struct RecordingLane {
              class: LaneClass,
              calls: Arc<Mutex<Vec<&'static str>>>,
              params: ParamBlock,
              scratch: ScratchArena,
          }
      - id: impl_RecordingLane
        type: ItemImpl
        signature: "#[cfg(any(test, feature = \"mock\"))] impl RecordingLane {"
        children:
        - id: impl_RecordingLane.new
          type: ImplItemFn
          code: |2
                pub fn new(class: LaneClass, cfg: LaneConfig) -> Self {
                    Self {
                        class,
                        calls: Arc::default(),
                        params: ParamBlock::new(cfg.param_capacity),
                        scratch: ScratchArena::new(cfg.scratch),
                    }
                }
        - id: impl_RecordingLane.calls
          type: ImplItemFn
          code: |2
                pub fn calls(&self) -> Vec<&'static str> {
                    self.calls.lock().clone()
                }
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel/cuda.rs
      nodes:
      - id: launch_cosine
        type: ItemFn
        code: |
          /// Output layout `n_queries x n_slots`.
          pub(crate) fn launch_cosine<T: DeviceRepr>(
              stream: &Arc<CudaStream>,
              f: &CudaFunction,
              docs: &CudaSlice<T>,
              queries: &CudaSlice<f32>,
              out: &mut CudaSlice<f32>,
              n_slots: u32,
              n_queries: u32,
          ) -> Result<(), KernelError> {
              let mut b = stream.launch_builder(f);
              b.arg(docs).arg(queries).arg(out).arg(&n_slots).arg(&n_queries);
              unsafe { b.launch(cfg(n_slots)) }.map(|_| ()).map_err(launch_err)
          }
      - id: launch_err
        type: ItemFn
        code: |
          fn launch_err(e: impl std::error::Error + Send + Sync + 'static) -> KernelError {
              KernelError::LaunchFailed {
                  source: Box::new(e),
              }
          }
      - id: ModuleDoc
        type: ModuleAttrs
        code: |
          //! Real CUDA kernels / scorer: NVRTC-compiled sources behind the kernel registry.
          #![cfg(feature = "cuda")]
```
/answer b040-13 allow
Control Request:
```yaml
id: b040-14
toolName: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src/resident
    pattern: pub struct Region\b|pub enum DType|pub enum DeviceKind|pub struct ArenaId|^use .*(DType|Region)
    include:
    - '*.rs'
    limit: 15
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/docs/architecture
    pattern: ^#
    include:
    - '*.md'
    limit: 40
  reason: Find Region/DType/DeviceKind definitions and doc structure conventions
```
/answer b040-14 allow
Control Request:
```yaml
id: b040-15
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src/resident
      matches:
      - path: segment/mod.rs
        matches:
        - match: use crate::resident::memory::dtype::DType;
        - match: use crate::resident::memory::region::Region;
      - path: segment/slots.rs
        matches:
        - match: "use crate::resident::memory::dtype::{DType, F16_MASK_VALUE};"
        - match: use crate::resident::memory::region::Region;
      - path: segment/error.rs
        matches:
        - match: use crate::resident::memory::dtype::DType;
      - path: processor/tests.rs
        matches:
        - match: "use crate::resident::memory::{ArenaId, Region};"
      - path: processor/hub.rs
        matches:
        - match: use crate::resident::memory::Region;
      - path: kernel/registry.rs
        matches:
        - match: use crate::resident::memory::DType;
      - path: kernel/cuda.rs
        matches:
        - match: use crate::resident::memory::DType;
      - path: kernel/mock.rs
        matches:
        - match: "use super::{BatchDesc, BatchId, Kernel, KernelHit, KernelOutput, RegionSpan, WrittenRange};"
      - path: subengine/exec.rs
        matches:
        - match: "use crate::resident::memory::{DeviceKind, Region};"
      - path: model/mod.rs
        matches:
        - match: "use crate::resident::memory::{CpuMemoryManager, DType, DeviceKind, MemoryError, Region};"
      - path: model/encoder.rs
        matches:
        - match: "use super::{EncodeEvent, Encoder, ModelError, Pooling, RegionIo, TensorRef};"
        - match: use crate::resident::memory::DType;
      - path: model/candle_adapter.rs
        matches:
        - match: "use candle_core::{DType as CDType, Device, Tensor};"
      warning: "Limit of 15 matches reached; further results may exist. Narrow the pattern, directory or include/exclude filters, or raise limit."
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/docs/architecture
      matches:
      - path: scope_charta.md
        matches:
        - match: "# Scope Charta: Resident Engine vs. Search Layer"
        - match: "## 1. Purpose"
        - match: "## 2. Boundary Table"
        - match: "## 3. Engine Delivers"
        - match: "## 4. Outside the Engine (Search Layer)"
        - match: "## 5. Central Divergence: Ring Buffer vs. Pull Queue (§3)"
        - match: "## 6. Hook Obligations (Phase 1 must not foreclose)"
        - match: "## 7. References"
        - match: "## 8. Acceptance Check"
      - path: conventions.md
        matches:
        - match: "# Cross-Cutting Conventions (A4): Errors, Logging, Testing, Feature Flags"
        - match: "## Q1 — Error Taxonomy (`thiserror`)"
        - match: "## Q2 — Logging (`tracing`)"
        - match: "## Q3 — Test Strategy & Mocks"
        - match: "## Q4 — Feature Flags"
        - match: "## Basis Scaffolding"
      - path: gpu_memory.md
        matches:
        - match: "# GPU memory and arenas"
        - match: "## Contract (`resident::memory`)"
        - match: "## Budget"
        - match: "## Allocation behaviour"
        - match: "## dtype"
        - match: "## Foreign allocations (Candle)"
        - match: "## Limits"
      - path: dependency_verification.md
        matches:
        - match: "# Dependency Verification & Risk List (A3)"
        - match: "## 1. Verified Versions (pinned in `Cargo.toml`)"
        - match: "## 2. cudarc Duplicate Check"
        - match: "## 3. CUDA Toolkit / Compute Capability"
        - match: "## 4. Roundtrip / Compatibility Tests"
        - match: "## 5. Open Points / Risk List"
        - match: "### Duplicate crates (`cargo tree -d`, default features)"
        - match: "### Vendored crates (`third_party/`, `[patch.crates-io]`)"
        - match: "## 6. Acceptance Status"
      - path: feature_matrix.md
        matches:
        - match: "# Feature Matrix"
        - match: "## CI"
        - match: "## Feature gating rule"
```
/answer b040-15 allow
ReasoningToken: 171
Control Request:
```yaml
id: b040-16
toolName: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src/resident/memory
    pattern: derive.*\n?|pub struct Region|pub enum DType|pub enum DeviceKind|pub struct ArenaId
    include:
    - region.rs
    - dtype.rs
    - mod.rs
    limit: 20
    intent: read_lines
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel
    pattern: ^pub struct KernelHit|^pub struct QueryBatch
    include:
    - mod.rs
    limit: 5
    intent: read_lines
  reason: Check derives of Region/DType/DeviceKind (needed for a hashable launch key)
```
/answer b040-16 allow
Control Request:
```yaml
id: b040-17
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src/resident/memory
      matches:
      - path: region.rs
        matches:
        - match: "#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]"
        - match: pub struct ArenaId(pub u32);
        - match: "#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]"
        - match: "pub struct Region {"
      - path: dtype.rs
        matches:
        - match: "#[derive(Error, Debug, Clone, Copy, PartialEq, Eq)]"
        - match: "#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]"
        - match: "pub enum DType {"
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel
      matches:
      - path: mod.rs
        matches:
        - match: "pub struct KernelHit {"
        - match: "pub struct QueryBatch {"
```
/answer b040-17 allow
ReasoningToken: 21
Control Request:
```yaml
id: b040-18
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel/mod.rs
    ids:
    - KernelHit
  reason: Need KernelHit fields for golden comparison
```
/answer b040-18 allow
Control Request:
```yaml
id: b040-19
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel/mod.rs
      nodes:
      - id: KernelHit
        type: ItemStruct
        code: |
          #[derive(Clone, Copy, Debug, PartialEq)]
          pub struct KernelHit {
              pub offset: u64,
              pub score: f32,
          }
```
/answer b040-19 allow
ReasoningToken: 809
Control Request:
```yaml
id: b040-1a
toolName: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag
    pattern: ^(tracing|thiserror|parking_lot)|^mock|^cuda|^candle
    include:
    - Cargo.toml
    exclude:
    - third_party/**
    limit: 10
  reason: Confirm tracing dependency and feature names before writing code
```
/answer b040-1a allow
Control Request:
```yaml
id: b040-1b
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag
      matches:
      - path: third_party/candle-transformers/Cargo.toml
        matches:
        - match: "cuda = ["
      - path: Cargo.toml
        matches:
        - match: thiserror = "2.0.20"
        - match: tracing = "0.1.44"
        - match: "tracing-subscriber = { version = \"0.3.23\", features = [\"env-filter\"] }"
        - match: tracing-appender = "0.2.3"
        - match: parking_lot = "0.12.5"
        - match: "cudarc = { version = \"=0.19.9\", optional = true }"
        - match: "candle = [\"dep:candle-core\", \"dep:candle-nn\", \"dep:candle-transformers\"]"
        - match: "cuda = [\"candle\", \"dep:cudarc\", \"candle-core/cuda\", \"candle-nn/cuda\", \"candle-transformers/cuda\"]"
        - match: "mock = [\"dep:rand\"]"
      warning: "Limit of 10 matches reached; further results may exist. Narrow the pattern, directory or include/exclude filters, or raise limit."
```
/answer b040-1b allow
Control Request:
```yaml
id: b040-1c
toolName: ast_create
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel/launcher.rs
    source: |
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

          #[tracing::instrument(level = "trace", skip_all, fields(op = key.op, bucket = key.shape_bucket))]
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
              LaunchMode::Capture => Err(KernelError::Unavailable {
                  reason: "graph capture is not enabled".into(),
              }),
          }
      }

      #[cfg(test)]
      mod tests {
          use super::*;
          use crate::resident::memory::ArenaId;
          use crate::resident::subengine::{LaneConfig, RecordingLane};
          use std::sync::atomic::{AtomicBool, Ordering};

          fn region(gen: u64) -> Region {
              Region {
                  arena: ArenaId(0),
                  offset: 0,
                  len: 64,
                  gen,
              }
          }

          fn key(gen: u64) -> LaunchKey {
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
              assert!(matches!(
                  launcher_for(LaunchMode::Capture),
                  Err(KernelError::Unavailable { .. })
              ));
          }

          #[test]
          fn generation_change_changes_key() {
              assert_ne!(key(0), key(1));
              assert_eq!(key(2), key(2));
          }
      }
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel/golden.rs
    source: |
      //! Golden comparison of a candidate scorer against a reference scorer.
      //!
      //! Exact equality is never required: scores may drift within a tolerance and ranks may swap only
      //! where the reference scores are within that tolerance of each other.

      use super::{KernelError, KernelHit, QueryBatch, Scorer, SliceRange};
      use crate::resident::subengine::ExecContext;

      #[derive(Clone, Copy, Debug)]
      pub struct Tolerance {
          pub abs: f32,
      }

      impl Tolerance {
          pub const F32: Self = Self { abs: 1e-5 };
          pub const F16: Self = Self { abs: 1e-3 };
      }

      #[derive(Clone, Debug, PartialEq)]
      pub enum Deviation {
          HitCount { query: usize, expected: usize, actual: usize },
          ScoreDrift { query: usize, rank: usize, delta: f32 },
          RankMismatch { query: usize, rank: usize, expected: u64, actual: u64 },
      }

      pub fn compare(
          reference: &[Vec<KernelHit>],
          candidate: &[Vec<KernelHit>],
          tol: Tolerance,
      ) -> Vec<Deviation> {
          let mut out = Vec::new();
          for (query, (r, c)) in reference.iter().zip(candidate).enumerate() {
              if r.len() != c.len() {
                  out.push(Deviation::HitCount {
                      query,
                      expected: r.len(),
                      actual: c.len(),
                  });
                  continue;
              }
              for (rank, (a, b)) in r.iter().zip(c).enumerate() {
                  let delta = (a.score - b.score).abs();
                  if delta > tol.abs {
                      out.push(Deviation::ScoreDrift { query, rank, delta });
                  } else if a.offset != b.offset && !near_tie(r, rank, tol) {
                      out.push(Deviation::RankMismatch {
                          query,
                          rank,
                          expected: a.offset,
                          actual: b.offset,
                      });
                  }
              }
          }
          if reference.len() != candidate.len() {
              out.push(Deviation::HitCount {
                  query: reference.len().min(candidate.len()),
                  expected: reference.len(),
                  actual: candidate.len(),
              });
          }
          out
      }

      fn near_tie(hits: &[KernelHit], rank: usize, tol: Tolerance) -> bool {
          let s = hits[rank].score;
          let before = rank > 0 && (hits[rank - 1].score - s).abs() <= tol.abs;
          let after = hits.get(rank + 1).is_some_and(|n| (n.score - s).abs() <= tol.abs);
          before || after
      }

      /// Runs both scorers over `ranges` and reports deviations as diagnostics, never as panics.
      pub fn check(
          reference: &dyn Scorer,
          candidate: &dyn Scorer,
          queries: &QueryBatch,
          ranges: &[SliceRange],
          lane: &mut dyn ExecContext,
          tol: Tolerance,
      ) -> Result<Vec<Deviation>, KernelError> {
          let mut all = Vec::new();
          for &range in ranges {
              let r = reference.score(queries, range, lane)?;
              let c = candidate.score(queries, range, lane)?;
              let found = compare(&r, &c, tol);
              for d in &found {
                  tracing::warn!(segment = range.segment, deviation = ?d, "golden deviation");
              }
              all.extend(found);
          }
          Ok(all)
      }

      #[cfg(test)]
      mod tests {
          use super::*;

          fn hit(offset: u64, score: f32) -> KernelHit {
              KernelHit { offset, score }
          }

          #[test]
          fn identical_results_have_no_deviation() {
              let r = vec![vec![hit(1, 0.9), hit(2, 0.5)]];
              assert!(compare(&r, &r.clone(), Tolerance::F32).is_empty());
          }

          #[test]
          fn drift_within_tolerance_is_accepted() {
              let r = vec![vec![hit(1, 0.9), hit(2, 0.5)]];
              let c = vec![vec![hit(1, 0.9004), hit(2, 0.4996)]];
              assert!(compare(&r, &c, Tolerance::F16).is_empty());
              assert!(!compare(&r, &c, Tolerance::F32).is_empty());
          }

          #[test]
          fn swap_at_near_tie_is_accepted() {
              let r = vec![vec![hit(1, 0.9000), hit(2, 0.8995), hit(3, 0.2)]];
              let c = vec![vec![hit(2, 0.9000), hit(1, 0.8995), hit(3, 0.2)]];
              assert!(compare(&r, &c, Tolerance::F16).is_empty());
          }

          #[test]
          fn swap_of_distinct_scores_is_reported() {
              let r = vec![vec![hit(1, 0.9), hit(2, 0.5)]];
              let c = vec![vec![hit(2, 0.9), hit(1, 0.5)]];
              let d = compare(&r, &c, Tolerance::F16);
              assert!(d.iter().any(|d| matches!(d, Deviation::RankMismatch { rank: 0, .. })));
          }

          #[test]
          fn hit_count_mismatch_is_reported() {
              let r = vec![vec![hit(1, 0.9)]];
              let c = vec![vec![]];
              assert_eq!(
                  compare(&r, &c, Tolerance::F16),
                  vec![Deviation::HitCount { query: 0, expected: 1, actual: 0 }]
              );
          }
      }
  reason: Create the launcher with Direct/Capture decorator point and the golden comparison harness
```
/answer b040-1c allow
Control Request:
```yaml
id: b040-1d
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/subengine/mod.rs
  regexp: pub use (exec|device)|mod exec
  reason: "Verify that ExecContext, LaneJob, LaneClass, LaneConfig, RecordingLane are re-exported from subengine"
```
/answer b040-1d allow
Control Request:
```yaml
id: b040-1e
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/subengine/mod.rs
      nodes:
      - id: exec
        code: |
          mod exec;
      - id: wLnUuf|YSWcPL
        code: |
          pub use device::*;
          pub use exec::{
              CpuLane, EventHandle, ExecContext, LaneClass, LaneConfig, LaneJob, ParamBlock,
              ParamValue, ScratchArena,
          };
          #[cfg(any(test, feature = "mock"))]
          pub use exec::RecordingLane;
          pub use error::{DeviceError, SubengineError};
          pub use lifecycle::{
              CellConfig, InitLimiter, OpGuard, ResourceCell, ResourceLease, ResourceState,
          };
          pub use resource::{
              coalesce, Capacity, Claim, ClaimFilter, Ewma, PullConfig, PullController, Resource,
              ResourceKey,
          };
```
/answer b040-1e allow
Control Request:
```yaml
id: b040-1f
toolName: ast_insert
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel/mod.rs
    id: error
    position: after
    source: |-
      pub mod golden;
      pub mod launcher;
  reason: Register the new launcher and golden modules in the kernel module
```
/answer b040-1f allow
ReasoningToken: 49
Control Request:
```yaml
id: b040-20
toolName: ast_create
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/tests/api_hygiene.rs
    source: |
      //! Public signatures of the resident engine must not expose Candle tensors or raw pointers.
      //! The Candle adapter and the CUDA backends are the only places allowed to touch them.

      use std::fs;
      use std::path::{Path, PathBuf};

      const ALLOWED: &[&str] = &["model/candle_adapter.rs", "kernel/cuda.rs", "kernel/maxsim.rs"];

      fn sources(dir: &Path, out: &mut Vec<PathBuf>) {
          for entry in fs::read_dir(dir).unwrap() {
              let path = entry.unwrap().path();
              if path.is_dir() {
                  sources(&path, out);
              } else if path.extension().is_some_and(|e| e == "rs") {
                  out.push(path);
              }
          }
      }

      fn public_signatures(text: &str) -> Vec<String> {
          let mut sigs = Vec::new();
          let mut lines = text.lines();
          while let Some(line) = lines.next() {
              let t = line.trim_start();
              if !(t.starts_with("pub fn ") || t.starts_with("pub unsafe fn ") || t.starts_with("pub const fn ")) {
                  continue;
              }
              let mut sig = t.to_string();
              while !sig.contains('{') && !sig.trim_end().ends_with(';') {
                  match lines.next() {
                      Some(next) => {
                          sig.push(' ');
                          sig.push_str(next.trim());
                      }
                      None => break,
                  }
              }
              sigs.push(sig);
          }
          sigs
      }

      fn leaks(sig: &str) -> bool {
          sig.contains("Tensor")
              || sig.contains("*mut ")
              || sig.contains("*const ")
              || sig.contains("CudaSlice")
              || sig.contains("CUdeviceptr")
      }

      #[test]
      fn public_api_is_free_of_tensors_and_pointers() {
          let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/resident");
          let mut files = Vec::new();
          sources(&root, &mut files);
          assert!(!files.is_empty());

          let mut violations = Vec::new();
          for file in files {
              let rel = file.strip_prefix(&root).unwrap().to_string_lossy().replace('\\', "/");
              if ALLOWED.contains(&rel.as_str()) {
                  continue;
              }
              let text = fs::read_to_string(&file).unwrap();
              let body = text.split("#[cfg(test)]").next().unwrap_or(&text);
              for sig in public_signatures(body) {
                  if leaks(&sig) {
                      violations.push(format!("{rel}: {sig}"));
                  }
              }
          }
          assert!(violations.is_empty(), "leaking signatures:\n{}", violations.join("\n"));
      }

      #[test]
      fn detector_flags_leaks() {
          let sigs = public_signatures("pub fn a(t: &Tensor) -> u8 {\n}\npub fn b(\n    p: *const u8,\n) {\n}\npub fn c() {}");
          let flagged: Vec<_> = sigs.iter().map(|s| leaks(s)).collect();
          assert_eq!(flagged, vec![true, true, false]);
      }
  reason: Add the API hygiene test (no Tensor/raw pointers in public signatures)
```
/answer b040-20 allow
Text:
 Launcher, golden harness and hygiene test are in place. Next I'll write the phase plan and capability matrix doc, then build and run the tests.
Control Request:
```yaml
id: b040-21
toolName: ast_create
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/docs/architecture/phase_plan.md
    source: |
      # CPU/GPU Differences and Build-out Phases

      ## Capability matrix

      | Aspect | CPU subengine | GPU subengine (`cuda`) |
      |---|---|---|
      | Memory | host RAM, large | VRAM bounded (reference: 6 GB), two index generations budgeted |
      | Index access | mmap readable | arena slots, synced from host |
      | Staging / streams | none; lanes are worker threads | pinned staging, one stream per lane, events |
      | Scorer | own SIMD loops (`cpu_simd`) | Candle tile chain, then fused NVRTC kernel |
      | Encoder | Candle CPU | Candle CUDA |
      | dtype | `F32` | `F16` |
      | Tokenizer / normalizer | CPU, shared | CPU, shared; the bottleneck, so batching is required |
      | Shared | `Op`, `Region`, `ExecContext`, lease logic, task pipelines | same |

      Numerics: f32 vs f16 cosine differs by 1e-3 to 1e-4; rankings differ only at near-ties. Indexes fix their dtype; mixed CPU/GPU comparison always uses a tolerance (`kernel::golden::Tolerance`).

      ## Phases

      Each phase keeps public signatures unchanged. Acceptance always includes a golden check against the previous phase (`kernel::golden::check`) and the API hygiene test (`tests/api_hygiene.rs`).

      | Phase | Content | Packages | Feature | Gate | Acceptance |
      |---|---|---|---|---|---|
      | 1 | `Region`/`Op`/`ExecContext` as thin wrappers over Candle; MaxSim as tile chain; lazy init, leases | D1-D7 | none (CPU), `cuda` optional | none | CPU f32 reference matches tile chain within `Tolerance::F32`; no-GPU build green |
      | 2 | own streams, fused MaxSim kernel (NVRTC), slice scheduling | D2, D6 | `cuda` | G-MAXSIM | fused kernel ranks like phase 1 within `Tolerance::F16` and is not slower on the measured workload |
      | 3 | own arena for weights and workspace, `encode_into` without copy | D3, D4 | `cuda` | G-WRAP | encoder output equals phase 2 within tolerance; if G-WRAP fails: arena only for own kernels, D2D copy for Candle tensors |
      | 4 | graph decorator, persistent scorer with ring | hooks only | `cuda` | G-GRAPH | `LaunchMode::Capture` selectable via `launcher_for`; replay equals `Direct` |
      | 5 | optional own encoder forward (cuBLAS plus few kernels) behind the same `Op` | D4 | `cuda` | none | embeddings equal Candle within `Tolerance::F16`; signature unchanged |

      Phase transitions are logged as `info` events; golden deviations are logged as `warn` diagnostics and returned, never panicked on.

      ## Extension points

      - Launch indirection: `kernel::launcher` (`Launcher`, `LaunchKey`, `LaunchMode`). `DirectLauncher` is active; `Capture` is rejected with `KernelError::Unavailable` until phase 4. The key holds op, shape bucket, regions (including generation), dtype and lane, which is the graph cache key.
      - Persistent scorer ring: needs a dedicated lane and fixed ring slots in the manager; state (head, tail, stop flag, counters) lives in its own region; the stop flag is checked only at slice boundaries. Not implemented.
      - Parameter objects stay extensible, `requires()` affinity stays open, and ops can be replaced behind the same trait.

      ## Trait hygiene

      `Op`, `Region` and `ExecContext` never expose `candle_core::Tensor` or raw pointers. Only `model/candle_adapter.rs`, `kernel/cuda.rs` and `kernel/maxsim.rs` (feature-gated Candle chain) may name them; `tests/api_hygiene.rs` enforces this for all other modules. Event ordering between Candle work and own kernels is explicit: own work waits on an `EventHandle` recorded on the Candle stream.
  reason: "Write the phase plan with capability matrix, acceptance criteria per phase and extension points"
```
/answer b040-21 allow
ReasoningToken: 74
Control Request:
```yaml
id: b040-22
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench/rag
  script: "cargo test --lib --features mock,candle kernel:: 2>&1 | tail -30; cargo test --features mock --test api_hygiene 2>&1 | tail -20"
  reason: "Build and run kernel unit tests (launcher, golden) and the API hygiene integration test"
```
/answer b040-22 allow
Control Request:
```yaml
id: b040-23
result:
  structuredContent:
    stdout: |2
          |
      315 |                     f32::NEG_INFINITY
          |                          ^^^^^^^^^^^^

      warning: `candle-transformers` (lib) generated 2 warnings
         Compiling xy-ai-rag v0.1.0 (/home/user/xyan/xy.ai.workbench/rag)
      error[E0308]: mismatched types
        --> src/resident/kernel/launcher.rs:80:13
         |
      80 |             gen,
         |             ^^^ expected `u32`, found `u64`
         |
      help: you can convert a `u64` to a `u32` and panic if the converted value doesn't fit
         |
      80 |             gen: gen.try_into().unwrap(),
         |             ++++    ++++++++++++++++++++

      warning: irrefutable `if let` pattern
         --> src/resident/subengine/exec.rs:337:20
          |
      337 |                 if let EventRepr::Cpu(e) = event.repr() {
          |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
          |
          = note: this pattern will always match, so the `if let` is useless
          = help: consider replacing the `if let` with a `let`
          = note: `#[warn(irrefutable_let_patterns)]` on by default

      For more information about this error, try `rustc --explain E0308`.
      warning: `xy-ai-rag` (lib test) generated 1 warning
      error: could not compile `xy-ai-rag` (lib test) due to 1 previous error; 1 warning emitted
      running 2 tests
      test detector_flags_leaks ... ok
      test public_api_is_free_of_tensors_and_pointers ... FAILED

      failures:

      ---- public_api_is_free_of_tensors_and_pointers stdout ----

      thread 'public_api_is_free_of_tensors_and_pointers' (479775) panicked at tests/api_hygiene.rs:72:5:
      leaking signatures:
      model/slot.rs: pub fn write_slot( mem: &dyn RegionIo, out: &TensorRef, rows: &mut [f32], dim: usize, ) -> Result<(), ModelError> {
      note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

      failures:
          public_api_is_free_of_tensors_and_pointers

      test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

      error: test failed, to rerun pass `--test api_hygiene`
```
/answer b040-23 allow
Control Request:
```yaml
id: b040-24
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel/launcher.rs
    id: tests.region
    old_text: "fn region(gen: u64) -> Region {"
    new_text: "fn region(gen: u32) -> Region {"
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel/launcher.rs
    id: tests.key
    old_text: "fn key(gen: u64) -> LaunchKey {"
    new_text: "fn key(gen: u32) -> LaunchKey {"
  reason: Region.gen is u32; fix the test helper types
```
/answer b040-24 allow
ReasoningToken: 120
Control Request:
```yaml
id: b040-25
toolName: ast_edit_marks
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/tests/api_hygiene.rs
    id: leaks
    start_marker: "fn leaks(sig: &str) -> bool {"
    end_marker: |-
      || sig.contains("CUdeviceptr")
      }
    content: |-
      fn has_word(sig: &str, word: &str) -> bool {
          let is_ident = |c: char| c.is_alphanumeric() || c == '_';
          sig.match_indices(word).any(|(i, _)| {
              let before = sig[..i].chars().next_back();
              let after = sig[i + word.len()..].chars().next();
              !before.is_some_and(is_ident) && !after.is_some_and(is_ident)
          })
      }

      fn leaks(sig: &str) -> bool {
          has_word(sig, "Tensor")
              || sig.contains("*mut ")
              || sig.contains("*const ")
              || has_word(sig, "CudaSlice")
              || has_word(sig, "CUdeviceptr")
      }
  reason: Use word-boundary matching so project type TensorRef is not a false positive
```
/answer b040-25 allow
Control Request:
```yaml
id: b040-26
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench/rag
  script: "cargo test --lib --features mock,candle kernel:: 2>&1 | grep -E \"^(error|test |test result)|launcher|golden\" | tail -40; cargo test --features mock --test api_hygiene 2>&1 | grep -E \"^(error|test |test result)|leaking|\\.rs:\" | tail -20; cargo test --lib 2>&1 | grep -E \"^(error|test result)\" | tail"
  reason: Re-run kernel tests with and without optional features plus the hygiene test
```
/answer b040-26 allow
Control Request:
```yaml
id: b040-27
result:
  content:
  - type: text
    text: "STDOUT limit of characters exceeded, output saved to `/tmp/mcpc-stdout-kkh0khst.log`. Reduce the resulting token amount: filter it with targeted commands (grep, head, tail, awk, ast_outline)"
  structuredContent:
    stdout_file: /tmp/mcpc-stdout-kkh0khst.log
    stdout_file_file_stats:
      path: /tmp/mcpc-stdout-kkh0khst.log
      size_bytes: 3367
      lines: 45
      words: 217
      complexity: 0.432
      created: 2026-10-19T17:02:18.104138+00:00
      modified: 2026-10-19T17:02:18.104138+00:00
      accessed: 2026-10-19T17:02:18.104138+00:00
      line_length_max: 102
      line_length_min: 32
      line_length_avg: 73.82
      words_per_line_avg: 4.82
      checksum: 956110acc8d6eaffb87caeda02f623cd926f60c4ee5ba3d58306bcdddaa2b6ae
    stdout_file_ast_outline:
    - id: file
      lines: 1-45
      signature: test resident::kernel::cpu_simd::tests::dot_handles_remainders ... ok
```
/answer b040-27 allow
Control Request:
```yaml
id: b040-28
toolName: bash
arguments:
  cwd: /tmp
  script: grep -E "FAILED|failed|^error|test result|launcher|golden|leaking" /tmp/mcpc-stdout-kkh0khst.log
  reason: Check results in the captured test log
```
/answer b040-28 allow
Control Request:
```yaml
id: b040-29
result:
  structuredContent:
    stdout: |
      test resident::kernel::golden::tests::drift_within_tolerance_is_accepted ... ok
      test resident::kernel::golden::tests::hit_count_mismatch_is_reported ... ok
      test resident::kernel::golden::tests::identical_results_have_no_deviation ... ok
      test resident::kernel::golden::tests::swap_at_near_tie_is_accepted ... ok
      test resident::kernel::golden::tests::swap_of_distinct_scores_is_reported ... ok
      test resident::kernel::launcher::tests::capture_is_unavailable_until_enabled ... ok
      test resident::kernel::launcher::tests::generation_change_changes_key ... ok
      test resident::kernel::launcher::tests::direct_runs_job_on_lane ... ok
      test resident::kernel::registry::tests::failed_load_is_not_cached ... ok
      test result: ok. 41 passed; 0 failed; 0 ignored; 0 measured; 273 filtered out; finished in 0.10s
      test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
      test result: ok. 306 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 3.14s
```
/answer b040-29 allow
Result Stats: id=8d4ede28-0fa1-4514-8f48-6cf26f4bca58, total: 76893, in: 32, out: 15480, reason: 56, read: 625508, write: 61381, start: 1792429351831, end: 1792429358768, time: 6884, total: 122554, io: 0, control: 53, sent: 0, recv: 182
ReasoningToken: 56
D8 ist umgesetzt: Die Tests laufen grün (306 Unit-Tests ohne Features, 41 Kernel-Tests mit `mock,candle`, 2 Hygiene-Tests). Der `cuda`-Build und die `#[ignore]`-GPU-Tests wurden nicht ausgeführt.

- **`kernel/launcher.rs`:** Enthält `LaunchMode` (`Direct`/`Capture`), `LaunchKey`, den Trait `Launcher`, `DirectLauncher` und `launcher_for`. Der Schlüssel besteht aus Op, Shape-Bucket, Regions (inklusive Generation), dtype und Lane. `Capture` liefert bis Phase 4 `KernelError::Unavailable`.
- **`kernel/golden.rs`:** Vergleicht einen Kandidaten-Scorer mit einem Referenz-Scorer über `Tolerance::F32` (1e-5) oder `Tolerance::F16` (1e-3). Rangvertauschungen sind nur bei Beinahe-Gleichstand erlaubt. Abweichungen werden per `tracing::warn` gemeldet und als `Deviation` zurückgegeben, ohne Panic.
- **`tests/api_hygiene.rs`:** Prüft, dass keine öffentliche Signatur in `src/resident` `Tensor`, rohe Pointer, `CudaSlice` oder `CUdeviceptr` enthält. Ausgenommen sind `model/candle_adapter.rs`, `kernel/cuda.rs` und `kernel/maxsim.rs`. Die Prüfung arbeitet mit Wortgrenzen, damit der eigene Typ `TensorRef` nicht fälschlich auffällt.
- **`docs/architecture/phase_plan.md`:** Enthält die CPU/GPU-Fähigkeitsmatrix und die Phasen 1–5 mit D-Paketen, Gates (G-MAXSIM, G-WRAP, G-GRAPH) und Abnahmekriterien. Außerdem stehen dort die Erweiterungspunkte und die Trait-Hygiene-Regeln.

- **Ring-Slots:** Die festen Ring-Slots im Manager (H1-Hook) habe ich nicht gebaut. Sie sind nur im Dokument als offener Erweiterungspunkt vermerkt.
- **Phasen-Regressionstest:** Phase 2 gegen Phase 1 ist noch nicht verdrahtet. Dafür fehlt ein Test, der `golden::check` mit der CPU-Referenz und dem fused-Kernel aufruft. Die vorhandenen Candle-Tests vergleichen schon gegen die CPU-Referenz.
