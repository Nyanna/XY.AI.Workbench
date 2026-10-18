Setze D6 um.

[include](/home/user/xyan/xy.ai.workbench/project/resident/index.md)
[include](/home/user/xyan/xy.ai.workbench/project/resident/plan/plan_d_00_overview.md)

- minimiere die Kommentare und keine direkte Referenzierung von Planelementen in Kommentaren
- Projektdokumente immer in englisch ausgeben
- nvcc fehlt nicht und ist im Projekt konfiguriert
- ziehe AST-Tools und spezifische Tools vor

# D6 — Kernels & Scorer

**Zweck:** `KernelRegistry` (NVRTC/CUBIN/Candle), Compile-Time- vs. ParamBlock-Parameter, `MaxSimOp` (Kachel-Kette → fused), Top-k-Reduktion/Merge, Work-Slice/Preemption, Query-Batching.

**Einordnung:** Abhängt von B5 (Kernel-Trait+Mock), D2 (Lane/ParamBlock/Events), D3 (Workspace/Arena). Downstream: G3 (Similarity/Cosine), D7 (Kernel schreibt/liest Segmente). Querschnitt Q1–Q4.

**Quellen:** `resident_subengine.md` §4.5; `resident_index.md` (Kernel-Schnittstelle, via B5); `context/context_d.md` D6 (#22–#26).

---

## Festgelegte Entscheidungen

- **Standardpfad e5 = Cosine** (G3). MaxSim ist Infrastruktur/Wechselpfad (ColBERT). Beide hinter demselben Scorer-Op.
- **KernelRegistry (§4.5, `[V/U]`, #23):** `HashMap<(&'static str name, DType, u32 sm), CudaFunction>`. Quellen: **NVRTC** (`compile_ptx_with_opts(src, CompileOptions{ arch: Some("sm_75"), .. })` → `ctx.load_module(ptx)` → `get_func`), **CUBIN**-Fallback bei zu altem Treiber (PTX `compute_75`), **Candle-Ops** als weitere Implementierung. Candles eigene Kernel zur Build-Zeit mit `CUDA_COMPUTE_CAP=75`.
- **Konstanten vs. Parameter (§4.5):** Compile-Time (`-D`/Varianten als eigene Funktionen): dtype, `dim`, Tile-Größe. Dynamisch über **ParamBlock** (feste Region): Slot-Indizes, Zähler, Segmenttabelle, Query-Länge, Head/Tail.
- **MaxSim (`[U]`, #22):** Phase 1 **Candle-Kette** (`matmul → masked_fill → max → sum`); Sim-Matrix je Dokument klein (32×128 f16 = 8 KB), speichergebunden, cuBLAS/Tensor-Cores effizient. **Fused Kernel** lohnt erst, wenn Profil Sim-Matrix-Traffic/Launch-Overhead zeigt → **G-MAXSIM messen**, dann austauschen (gleicher Op).
- **Padding (`[V/U]`, #25):** Dokumentseite vor dem Max mit **−1e4** maskieren (nicht −inf → NaN-Risiko bei 0·−inf in f16); Query-`[MASK]`-Tokens zählen mit (Query-Augmentation). PLAID teilt dasselbe Scoring.
- **CPU-SIMD (`[U]`, #24):** Schleife über Query-Tokens, je Token laufendes Maximum über Doc-Tokens; `std::arch` (AVX2/FMA)/`wide`/`pulp` oder Autovektorisierung über `chunks_exact(8)`; `half` für f16. **Keine** Slot-übergreifende Matrix.
- **Top-k (§4.5):** pro Slice lokal reduzieren, **Merge auf der CPU**.
- **Work-Slice (`[U]`, #26):** Kernel in begrenzte Launches (~1–5 ms) teilen; Deadline zwischen Launches prüfen; Event je Slice; **Preemption nur an Slice-Grenzen**; hohe Prio gewinnt nur beim Block-Scheduling. Referenzen: Clockwork/PipeSwitch/REEF.
- **Query-Batching (§4.5):** ein Tile gegen alle wartenden Queries scoren (Lesekosten amortisieren).

---

## Zu erstellende Artefakte (`rag/src/kernel/`)

- `mod.rs` (baut auf B5)
  - `trait Kernel` (B5): Batch-Start/-Ende, Ergebnisse `KernelHit{offset, score}`, geschriebene Bereiche, Fortschrittszähler; `SegmentDesc{base, count, layout}`, `Layout{Fixed{stride}|Variable{offsets}}`.
  - `trait Scorer`: `score(query_batch, SliceRange, lane, param_block) -> Vec<KernelHit>`.
- `registry.rs`
  - `struct KernelRegistry` — `(name, dtype, sm) → Function`; NVRTC-Compile + CUBIN-Fallback; Candle-Impl als Eintrag. Lazy, über D1-Lease entladbar.
- `maxsim.rs`
  - `struct MaxSimOp` — `SliceRange{segment, slot_from, slot_to}` + Query-Batch → Scores je Slot.
  - Phase-1-Impl: Candle-Kachel-Kette (laufendes Max über Token-Kacheln, Summe über Query-Tokens); Zwischenmatrix durch Kachelgröße begrenzt.
  - Phase-2-Hook: fused NVRTC-Kernel hinter demselben Op.
- `cosine.rs`
  - e5-Standardpfad: Skalarprodukt auf L2-normierten Vektoren (Cosine = Dot), Scope-Subset, `top_k`.
- `cpu_simd.rs`
  - CPU-Scorer (SIMD), laufendes Max je Query-Token.
- `slice.rs`
  - Work-Slice-Scheduler: Zeitbudget, adaptive Größe aus Durchsatz, Preemption an Grenzen, Query-Überholen.
- `topk.rs`
  - Lokale Reduktion je Slice + CPU-Merge.

---

## Prototyp-Gate (zuerst)

- **G-MAXSIM:** Kachel-Kette vs. fused Kernel mit realen Workloads messen (Bandbreite, Latenz, Launch-Overhead). Entscheidet, wann Phase-2-Fused eingeführt wird.

---

## Arbeitsschritte

1. `Scorer`/`Kernel`-Traits (auf B5) + `KernelRegistry` (NVRTC+CUBIN+Candle).
2. Cosine-Scorer (e5-Standardpfad) auf L2-normierten Vektoren; Scope-Subset + `top_k`.
3. MaxSim Phase-1 (Candle-Kette) mit f16-sicherer Maskierung (−1e4), Query-`[MASK]` zählt mit.
4. CPU-SIMD-Scorer (laufendes Max je Query-Token).
5. Work-Slice-Scheduler (Zeitbudget, adaptiv, Preemption an Grenzen, Query-Überholen).
6. Query-Batching (ein Tile gegen wartende Queries); Top-k lokal + CPU-Merge.
7. G-MAXSIM messen; bei Bedarf fused NVRTC-Kernel hinter demselben Op ergänzen.

## Fehler (Q1) & Logging (Q2)

- `enum KernelError { Compile{log}, ModuleLoad, FuncNotFound{name}, LaunchFailed{source}, ParamBlockOverflow }`.
- Span pro Scoring-Op (`op`, `slice`, `n_queries`, `sm`); Compile-Events auf `info`, Launches auf `trace`. Fortschrittszähler als Metrik.

## Feature-Flags (Q4)

- NVRTC/CUBIN/CUDA-Launch hinter `cuda`; CPU-SIMD-Scorer + Candle-CPU-MaxSim immer verfügbar. B5-Kernel-Mock für GPU-freie Tests.

## Tests (Q3)

- CPU-Scorer gegen Referenz-Scores (Cosine/MaxSim) mit kleinen Fixtures.
- Padding/Maske: −1e4 verhindert NaN; Query-`[MASK]` zählt mit; Doc-Padding maskiert.
- Work-Slice: Preemption nur an Grenzen; Query überholt Indexing-Slice (deterministische Hooks).
- Top-k-Merge korrekt über mehrere Slices.
- Kernel-Mock (B5) für Launch-/Fortschrittslogik ohne GPU.
- CUDA/NVRTC-Compile nur unter `cuda` + GPU.

## Definition of Done

- Cosine-Standardpfad (e5) + MaxSim-Kachel-Kette lauffähig; CPU-SIMD-Pfad; Work-Slice/Top-k; KernelRegistry (NVRTC+CUBIN+Candle); G-MAXSIM gemessen und Entscheidung zu fused dokumentiert.

SystemInit:  id=7b45c58f-e506-4bd7-a286-5239122faa81, cwd=/home/user/xyan/xy.ai.workbench/project, model=claude-sonnet-5-5
Text:
 I'll start by looking at the existing resident code, especially the kernel module.
Control Request:
```yaml
id: 6bcd-13a
toolName: list
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident
  - path: /home/user/xyan/xy.ai.workbench/rag
    recursive: false
  reason: Survey existing resident module layout and crate root
```
/answer 6bcd-13a allow
ReasoningToken: 23
Control Request:
```yaml
id: 6bcd-13b
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
      - "./subengine: (6 files)"
      - device.rs
      - "./tokenize: (4 files)"
      - error.rs
      warning: Result reduced because the total number of entries (82) exceeds the limit of 50. Narrow down the result using the 'pattern' regular expression parameter.
    - path: /home/user/xyan/xy.ai.workbench/rag
      entries:
      - ".:"
      - .gitignore
      - .project
      - .pydevproject
      - Cargo.lock
      - Cargo.toml
      - xyrag.sh
```
/answer 6bcd-13b allow
Control Request:
```yaml
id: 6bcd-13c
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench/rag
  script: cat Cargo.toml; git status --short | head -20; wc -l src/resident/kernel/*.rs src/resident/op/*.rs src/resident/memory/*.rs src/resident/subengine/*.rs src/resident/*.rs
  reason: "Read Cargo manifest, git state and sizes of related files"
```
/answer 6bcd-13c deny
Control Request:
```yaml
id: 6bcd-13d
toolName: list
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/op
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/subengine
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/memory
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/index
  reason: List files in modules D6 builds upon
```
/answer 6bcd-13d allow
Control Request:
```yaml
id: 6bcd-13e
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel
      entries:
      - ".:"
      - cuda.rs
      - mock.rs
      - mod.rs
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/op
      entries:
      - ".:"
      - lease.rs
      - mod.rs
      - requirements.rs
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/subengine
      entries:
      - ".:"
      - device.rs
      - error.rs
      - exec.rs
      - lifecycle.rs
      - mod.rs
      - resource.rs
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/memory
      entries:
      - ".:"
      - arena.rs
      - budget.rs
      - cpu.rs
      - cuda.rs
      - dtype.rs
      - error.rs
      - freelist.rs
      - layout.rs
      - manager.rs
      - mod.rs
      - region.rs
      - staging.rs
      - wrap_gate.rs
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/index
      entries:
      - ".:"
      - memory.rs
      - mod.rs
      - stamp.rs
```
/answer 6bcd-13e allow
Control Request:
```yaml
id: 6bcd-13f
toolName: read_file
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/Cargo.toml
  reason: Check dependencies and features
```
/answer 6bcd-13f allow
Control Request:
```yaml
id: 6bcd-140
toolName: read_file
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel/mod.rs
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel/mock.rs
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel/cuda.rs
  reason: Read existing kernel module (B5) that D6 extends
```
/answer 6bcd-140 deny AST
Control Request:
```yaml
id: 6bcd-141
toolName: read_file
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/subengine/mod.rs
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/subengine/exec.rs
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/memory/mod.rs
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/mod.rs
  reason: "Read exec context, memory exports and resident module root"
```
/answer 6bcd-141 deny AST
ReasoningToken: 28
Control Request:
```yaml
id: 6bcd-142
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel/mod.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel/mock.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel/cuda.rs
  reason: Outline existing kernel module
```
/answer 6bcd-142 allow
Control Request:
```yaml
id: 6bcd-143
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel/mod.rs
      nodes:
      - id: ModuleDoc
        signature: "//! Kernel abstraction: segment descriptors, hits and the `Kernel` trait."
      - id: cuda
        signature: pub mod cuda;
      - id: mock
        signature: "#[cfg(any(test, feature = \"mock\"))]"
      - id: TjIWN2|DqltD3
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
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel/mock.rs
      nodes:
      - id: ModuleDoc
        signature: //! Deterministic hand-written kernel fake for GPU-free tests.
      - id: lwjoiI|VqtpcD
        signature: use std::collections::HashMap;
      - id: Call
        signature: "#[derive(Clone, Copy, Debug, PartialEq, Eq)] pub enum Call {"
      - id: Mode
        signature: "#[derive(Clone, Copy, Debug)] enum Mode {"
      - id: Batch
        signature: "struct Batch {"
      - id: State
        signature: "struct State {"
      - id: MockKernel
        signature: /// Scoring mode returns one hit per input span; embedding mode "writes" the in…
        docstring: Scoring mode returns one hit per input span; embedding mode "writes" the input …
      - id: impl_MockKernel
        signature: "impl MockKernel {"
        children:
        - id: impl_MockKernel.scoring
          signature: "pub fn scoring() -> Self {"
        - id: impl_MockKernel.embedding
          signature: "pub fn embedding(capacity: u64) -> Self {"
        - id: impl_MockKernel.new
          signature: "fn new(mode: Mode, capacity: u64) -> Self {"
        - id: impl_MockKernel.fail_after
          signature: "/// `run` succeeds `n` times, then fails on every further call. pub fn fail_aft…"
          docstring: "`run` succeeds `n` times, then fails on every further call."
        - id: impl_MockKernel.grant
          signature: "/// Grants additional writable capacity, as the manager does after a `Partial`.…"
          docstring: "Grants additional writable capacity, as the manager does after a `Partial`."
        - id: impl_MockKernel.calls
          signature: "pub fn calls(&self) -> Vec<Call> {"
      - id: score
        signature: "fn score(queries: u32, span: &RegionSpan) -> f32 {"
      - id: impl_Kernel_for_MockKernel
        signature: "impl Kernel for MockKernel {"
        children:
        - id: impl_Kernel_for_MockKernel.begin
          signature: "fn begin(&self, batch: &BatchDesc) -> Result<BatchId, IndexError> {"
        - id: impl_Kernel_for_MockKernel.run
          signature: "fn run(     &self,     batch: BatchId,     input: &[RegionSpan], ) -> Result<Ke…"
        - id: impl_Kernel_for_MockKernel.progress
          signature: "fn progress(&self, batch: BatchId) -> u64 {"
        - id: impl_Kernel_for_MockKernel.end
          signature: "fn end(&self, batch: BatchId) -> Result<Vec<WrittenRange>, IndexError> {"
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel/cuda.rs
      nodes:
      - id: ModuleDoc
        signature: //! Real CUDA kernels / scorer.
```
/answer 6bcd-143 allow
Control Request:
```yaml
id: 6bcd-144
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel/mod.rs
    ids:
    - ModuleDoc
    - TjIWN2|DqltD3
    - RegionSpan
    - OffsetRef
    - SegmentLayout
    - SegmentDesc
    - KernelHit
    - WrittenRange
    - BatchDesc
    - BatchId
    - KernelOutput
    - Kernel
    - entry_of_fixed
    - entry_of
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel/cuda.rs
    ids:
    - ModuleDoc
  reason: Read kernel trait and types in full
```
/answer 6bcd-144 allow
Control Request:
```yaml
id: 6bcd-145
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel/mod.rs
      nodes:
      - id: ModuleDoc
        type: ModuleAttrs
        code: |
          //! Kernel abstraction: segment descriptors, hits and the `Kernel` trait.
          //!
          //! The kernel knows only segments, offsets and spans. Mapping an address back to an entry is the
          //! manager's job (`entry_of`, `entry_of_fixed`); the public API carries no pointers.
      - id: TjIWN2|DqltD3
        type: ImportGroup
        code: |
          use std::time::Duration;

          use crate::resident::error::IndexError;
      - id: RegionSpan
        type: ItemStruct
        code: |
          /// Address-agnostic arena span; pointer resolution happens only inside a backend.
          #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
          pub struct RegionSpan {
              pub offset: u64,
              pub len: u64,
          }
      - id: OffsetRef
        type: ItemStruct
        code: |
          /// Offset table (`n + 1` prefix sums) living in an arena span.
          #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
          pub struct OffsetRef(pub RegionSpan);
      - id: SegmentLayout
        type: ItemEnum
        code: |
          #[derive(Clone, Copy, Debug, PartialEq, Eq)]
          pub enum SegmentLayout {
              Fixed { stride: u32 },
              Variable { offsets: OffsetRef },
          }
      - id: SegmentDesc
        type: ItemStruct
        code: |
          #[derive(Clone, Copy, Debug, PartialEq, Eq)]
          pub struct SegmentDesc {
              pub base: u64,
              pub count: u32,
              pub layout: SegmentLayout,
          }
      - id: KernelHit
        type: ItemStruct
        code: |
          #[derive(Clone, Copy, Debug, PartialEq)]
          pub struct KernelHit {
              pub offset: u64,
              pub score: f32,
          }
      - id: WrittenRange
        type: ItemStruct
        code: |
          #[derive(Clone, Copy, Debug, PartialEq, Eq)]
          pub struct WrittenRange {
              pub offset: u64,
              pub len: u64,
          }
      - id: BatchDesc
        type: ItemStruct
        code: |
          #[derive(Clone, Debug, Default)]
          pub struct BatchDesc {
              pub segments: Vec<SegmentDesc>,
              pub queries: u32,
              pub params: Option<RegionSpan>,
              pub time_budget: Option<Duration>,
          }
      - id: BatchId
        type: ItemStruct
        code: |
          #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
          pub struct BatchId(u64);
      - id: KernelOutput
        type: ItemEnum
        code: |
          #[derive(Clone, Debug, PartialEq)]
          pub enum KernelOutput {
              Hits(Vec<KernelHit>),
              /// The granted range is exhausted; the manager grants the next one and resumes.
              Partial,
          }
      - id: Kernel
        type: ItemTrait
        signature: "pub trait Kernel: Send + Sync {"
        children:
        - id: Kernel.begin
          type: TraitItemFn
          code: |2
                fn begin(&self, batch: &BatchDesc) -> Result<BatchId, IndexError>;
        - id: Kernel.run
          type: TraitItemFn
          code: |2
                fn run(&self, batch: BatchId, input: &[RegionSpan]) -> Result<KernelOutput, IndexError>;
        - id: Kernel.progress
          type: TraitItemFn
          code: |2
                /// Monotonically non-decreasing.
                fn progress(&self, batch: BatchId) -> u64;
        - id: Kernel.end
          type: TraitItemFn
          code: |2
                fn end(&self, batch: BatchId) -> Result<Vec<WrittenRange>, IndexError>;
      - id: entry_of_fixed
        type: ItemFn
        code: |
          /// Entry index for `addr` in a `Fixed` segment.
          pub fn entry_of_fixed(base: u64, stride: u32, count: u32, addr: u64) -> Option<usize> {
              if stride == 0 || addr < base {
                  return None;
              }
              let idx = (addr - base) / u64::from(stride);
              (idx < u64::from(count)).then_some(idx as usize)
          }
      - id: entry_of
        type: ItemFn
        code: |
          /// Entry index for `addr` given `n + 1` ascending offsets; entry `i` covers
          /// `[offsets[i], offsets[i + 1])`.
          pub fn entry_of(offsets: &[u64], addr: u64) -> Option<usize> {
              let i = offsets.partition_point(|&o| o <= addr);
              (i > 0 && i < offsets.len()).then(|| i - 1)
          }
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel/cuda.rs
      nodes:
      - id: ModuleDoc
        type: ModuleAttrs
        code: |
          //! Real CUDA kernels / scorer.
```
/answer 6bcd-145 allow
Control Request:
```yaml
id: 6bcd-146
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/mod.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/subengine/exec.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/subengine/error.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/subengine/mod.rs
  reason: "Understand exec context, errors and module wiring"
```
/answer 6bcd-146 allow
Control Request:
```yaml
id: 6bcd-147
result:
  structuredContent:
    results:
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
/answer 6bcd-147 allow
ReasoningToken: 105
Control Request:
```yaml
id: 6bcd-148
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/error.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/memory/dtype.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/memory/region.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/memory/layout.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/index/mod.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/subengine/device.rs
  reason: "Understand error enums, dtype, region, index trait and device kinds"
```
/answer 6bcd-148 allow
Control Request:
```yaml
id: 6bcd-149
result:
  structuredContent:
    results:
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
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/memory/dtype.rs
      nodes:
      - id: ModuleDoc
        signature: //! Engine-owned element type; backend dtypes are converted at the edge only.
      - id: MS3n2e|NqxAEG
        signature: use thiserror::Error;
      - id: UnsupportedDType
        signature: "#[derive(Error, Debug, Clone, Copy, PartialEq, Eq)] #[error(\"unsupported dtype:…"
      - id: 9HWcpZ|0bwvF9
        signature: /// Additive attention-mask value that stays finite in f16 (`-inf` would turn r…
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
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/memory/region.rs
      nodes:
      - id: ModuleDoc
        signature: //! Pointer-free memory handle; resolution to a device address is backend-inter…
      - id: ArenaId
        signature: "#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]"
      - id: Region
        signature: "#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)] pub struct Region {"
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
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/index/mod.rs
      nodes:
      - id: ModuleDoc
        signature: "//! Index trait, Scope, AccessMethod, Stamp."
      - id: memory
        signature: pub mod memory;
      - id: stamp
        signature: pub mod stamp;
      - id: kAsHBt|OqD8dC
        signature: use std::ops::Range;
      - id: EntryId
        signature: "#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]"
      - id: impl_EntryId
        signature: "impl EntryId {"
        children:
        - id: impl_EntryId.new
          signature: "pub const fn new(raw: u64) -> Self {"
        - id: impl_EntryId.get
          signature: "pub const fn get(self) -> u64 {"
      - id: NodeId
        signature: "#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]"
      - id: CellId
        signature: "#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]"
      - id: Stamp
        signature: /// Opaque change marker. Compare with `!=` only; ordering is deliberately abse…
        docstring: Opaque change marker. Compare with `!=` only; ordering is deliberately absent.
      - id: impl_Stamp
        signature: "impl Stamp {"
        children:
        - id: impl_Stamp.from_raw
          signature: "pub(crate) const fn from_raw(raw: u64) -> Self {"
        - id: impl_Stamp.raw
          signature: "/// Raw value, e.g. as `watch` watermark or for persistence. pub const fn raw(s…"
          docstring: "Raw value, e.g. as `watch` watermark or for persistence."
      - id: AccessMethod
        signature: "#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)] #[non_exhaustive] pub enum A…"
      - id: Scope
        signature: "#[derive(Clone, Debug, PartialEq)] pub enum Scope {"
      - id: EntrySet
        signature: "/// Compact set of entry ids stored as sorted, merged, half-open ranges. #[deri…"
        docstring: "Compact set of entry ids stored as sorted, merged, half-open ranges."
      - id: impl_EntrySet
        signature: "impl EntrySet {"
        children:
        - id: impl_EntrySet.new
          signature: "pub fn new() -> Self {"
        - id: impl_EntrySet.from_ranges
          signature: "pub fn from_ranges(ranges: impl IntoIterator<Item = Range<u64>>) -> Self {"
        - id: impl_EntrySet.from_ids
          signature: "pub fn from_ids(ids: impl IntoIterator<Item = EntryId>) -> Self {"
        - id: impl_EntrySet.ranges
          signature: "pub fn ranges(&self) -> &[Range<u64>] {"
        - id: impl_EntrySet.is_empty
          signature: "pub fn is_empty(&self) -> bool {"
        - id: impl_EntrySet.len
          signature: "pub fn len(&self) -> u64 {"
        - id: impl_EntrySet.contains
          signature: "pub fn contains(&self, id: EntryId) -> bool {"
        - id: impl_EntrySet.iter
          signature: "pub fn iter(&self) -> impl Iterator<Item = EntryId> + '_ {"
      - id: EntryData
        signature: /// Variable-length vector region of one entry; never a copy of backing storage…
        docstring: Variable-length vector region of one entry; never a copy of backing storage.
      - id: impl_EntryData____
        signature: "impl EntryData<'_> {"
        children:
        - id: impl_EntryData____.as_bytes
          signature: "pub fn as_bytes(&self) -> &[u8] {"
      - id: SummaryRef
        signature: "/// Summary structure such as a centroid table, exposed as vector data."
        docstring: "Summary structure such as a centroid table, exposed as vector data."
      - id: HashAlgo
        signature: "#[derive(Clone, Copy, Debug, PartialEq, Eq)] #[non_exhaustive] pub enum HashAlg…"
      - id: IndexHeader
        signature: /// Binding between index and model; persistence format is owned by the index. …
        docstring: Binding between index and model; persistence format is owned by the index.
      - id: Index
        signature: "/// Object-safe: engines hold indexes as `Arc<dyn Index + Send + Sync>`. pub tr…"
        docstring: "Object-safe: engines hold indexes as `Arc<dyn Index + Send + Sync>`."
        children:
        - id: Index.stamp
          signature: /// `None` = entry removed.
          docstring: '`None` = entry removed.'
        - id: Index.stamps
          signature: /// Must not touch vector data.
          docstring: Must not touch vector data.
        - id: Index.read
          signature: /// Missing entries yield empty data. May block on I/O.
          docstring: Missing entries yield empty data. May block on I/O.
        - id: Index.apply
          signature: /// Serializes writers (last writer wins in dequeue order). Returns the stamp
          docstring: Serializes writers (last writer wins in dequeue order). Returns the stamp
        - id: Index.changed
          signature: /// Level signal carrying the latest stamp watermark. Consume with
          docstring: Level signal carrying the latest stamp watermark. Consume with
        - id: Index.capabilities
          signature: "fn capabilities(&self) -> &[AccessMethod];"
        - id: Index.children
          signature: "fn children(&self, node: NodeId) -> EntrySet;"
        - id: Index.members
          signature: "fn members(&self, cell: CellId) -> EntrySet;"
        - id: Index.summary
          signature: "fn summary(&self, kind: AccessMethod) -> Option<SummaryRef>;"
        - id: Index.prefetch
          signature: /// Hint only; may be a no-op.
          docstring: Hint only; may be a no-op.
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/subengine/device.rs
      nodes:
      - id: ModuleDoc
        signature: "//! Compute backends, device handle and CUDA execution lanes."
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
        - id: impl_DeviceHandle.cuda_context
          signature: "#[cfg(feature = \"cuda\")] pub(crate) fn cuda_context(     &self, ) -> Option<&st…"
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
```
/answer 6bcd-149 allow
Thinking:
The kernel module already has the B5 trait, types, and mock in place; next I'll add `registry`, `cosine`, `maxsim`, `cpu_simd`, `slice`, `topk`, and `KernelError` under `resident/kernel/` for D6. First, I'm checking the error and dtype details.


Control Request:
```yaml
id: 6bcd-14a
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/error.rs
    ids:
    - IndexError
    - impl_IsRetryable_for_IndexError
    - ResidentError
    - impl_IsRetryable_for_ResidentError
    - tests.all_errors_are_send_sync_static
    - tests.layer_errors_convert_into_resident_error
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/memory/dtype.rs
    ids:
    - 9HWcpZ|0bwvF9
    - DType
    - impl_DType
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/memory/region.rs
    ids:
    - Region
  reason: "Need exact error enums, mask constant and DType variants to integrate KernelError"
```
/answer 6bcd-14a allow
ReasoningToken: 184
Control Request:
```yaml
id: 6bcd-14b
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
              #[error(transparent)]
              Memory(#[from] crate::resident::memory::MemoryError),
              #[error(transparent)]
              Model(#[from] crate::resident::model::ModelError),
              #[error(transparent)]
              Tokenize(#[from] crate::resident::tokenize::TokenizeError),
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
                        ResidentError::Memory(e) => e.is_retryable(),
                        ResidentError::Model(e) => e.is_retryable(),
                        ResidentError::Tokenize(e) => e.is_retryable(),
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
                  assert_send_sync_static::<crate::resident::memory::MemoryError>();
                  assert_send_sync_static::<crate::resident::model::ModelError>();
                  assert_send_sync_static::<crate::resident::tokenize::TokenizeError>();
              }
      - id: tests.layer_errors_convert_into_resident_error
        type: ItemFn
        code: |2
              #[test]
              fn layer_errors_convert_into_resident_error() {
                  let e: ResidentError = IndexError::UnknownBatch {
                      batch: 3,
                  }
                      .into();
                  assert!(
                      matches!(e, ResidentError::Index(IndexError::UnknownBatch { batch : 3 }))
                  );
                  let e: ResidentError = ConfigError::MissingField {
                      field: "model".into(),
                  }
                      .into();
                  assert!(matches!(e, ResidentError::Config(_)));
                  let e: ResidentError = PrecisionError::NoActiveProfile.into();
                  assert!(matches!(e, ResidentError::Precision(_)));
                  let e: ResidentError = PlannerError::UnknownModel {
                      model: "m".into(),
                  }
                      .into();
                  assert!(matches!(e, ResidentError::Planner(_)));
                  let e: ResidentError = CapabilityError::Missing {
                      capability: "c".into(),
                  }
                      .into();
                  assert!(matches!(e, ResidentError::Capability(_)));
              }
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/memory/dtype.rs
      nodes:
      - id: 9HWcpZ|0bwvF9
        type: StatementGroup
        code: |
          /// Additive attention-mask value that stays finite in f16 (`-inf` would turn rows into NaN).
          pub const F16_MASK_VALUE: f32 = -1e4;
      - id: DType
        type: ItemEnum
        code: |
          #[non_exhaustive]
          #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
          pub enum DType {
              F32,
              F16,
              Bf16,
              U8,
              U32,
              I64,
          }
      - id: impl_DType
        type: ItemImpl
        signature: "impl DType {"
        children:
        - id: impl_DType.size_bytes
          type: ImplItemFn
          code: |2
                pub const fn size_bytes(self) -> usize {
                    match self {
                        DType::U8 => 1,
                        DType::F16 | DType::Bf16 => 2,
                        DType::F32 | DType::U32 => 4,
                        DType::I64 => 8,
                    }
                }
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/memory/region.rs
      nodes:
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
```
/answer 6bcd-14b allow
Control Request:
```yaml
id: 6bcd-14c
toolName: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src
    pattern: KernelError|KernelRegistry|MASK_NEG|cuda_context|NVRTC|nvrtc
    include:
    - '*.rs'
    limit: 30
  with_lineno: true
  reason: Check for prior uses and naming conflicts
```
/answer 6bcd-14c allow
Control Request:
```yaml
id: 6bcd-14d
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src
      matches:
      - path: resident/subengine/device.rs
        matches:
        - lineno: 81
          match: pub(crate) fn cuda_context(
      - path: resident/memory/cuda.rs
        matches:
        - lineno: 76
          match: .cuda_context()
        - lineno: 284
          match: let ctx = dev.cuda_context().unwrap();
```
/answer 6bcd-14d allow
Control Request:
```yaml
id: 6bcd-14e
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench/rag
  script: nvidia-smi -L 2>&1 | head -3; which nvcc; ls ~/.cargo/registry/src/*/ | grep -E "^(cudarc|candle-core)"
  reason: "Check for GPU, nvcc and available cudarc/candle sources for API verification"
```
/answer 6bcd-14e allow
ReasoningToken: 3272
Control Request:
```yaml
id: 6bcd-14f
result:
  structuredContent:
    stdout: |
      GPU 0: NVIDIA GeForce GTX 1660 SUPER (UUID: GPU-aed215a7-f66f-9b18-b9d4-385029f6b481)
      candle-core-0.11.0
      cudarc-0.17.8
      cudarc-0.19.9
```
/answer 6bcd-14f allow
Control Request:
```yaml
id: 6bcd-150
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/memory/cuda.rs
  reason: See how arena regions resolve to device pointers for kernel launches
```
/answer 6bcd-150 allow
Control Request:
```yaml
id: 6bcd-151
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/memory/cuda.rs
      nodes:
      - id: ModuleDoc
        signature: "//! CUDA device arenas: one backing allocation per device-resident class, pinne…"
      - id: Zmi9iH|Gv93IN
        signature: use std::ffi::c_void;
      - id: PinnedStaging
        signature: pub type PinnedStaging = StagingRing<PinnedHostSlice<u8>>;
      - id: unavailable
        signature: "fn unavailable(e: DriverError) -> MemoryError {"
      - id: transfer
        signature: "fn transfer(dir: TransferDir) -> impl Fn(DriverError) -> MemoryError {"
      - id: CudaProbe
        signature: "pub struct CudaProbe {"
      - id: impl_BudgetProbe_for_CudaProbe
        signature: "impl BudgetProbe for CudaProbe {"
        children:
        - id: impl_BudgetProbe_for_CudaProbe.probe
          signature: "fn probe(&self) -> Result<(u64, u64), MemoryError> {"
      - id: retain_pool_memory
        signature: /// Keeps freed stream-ordered allocations in the default pool instead of retur…
        docstring: Keeps freed stream-ordered allocations in the default pool instead of returning…
      - id: CudaMemoryManager
        signature: "pub struct CudaMemoryManager {"
      - id: impl_CudaMemoryManager
        signature: "impl CudaMemoryManager {"
        children:
        - id: impl_CudaMemoryManager.new
          signature: /// Allocates the device arenas up front. `headroom` stays free for the driver …
          docstring: Allocates the device arenas up front. `headroom` stays free for the driver cont…
        - id: impl_CudaMemoryManager.arenas
          signature: "pub fn arenas(&self) -> &ArenaSet {"
        - id: impl_CudaMemoryManager.staging_ring
          signature: /// Ring of page-locked host buffers for host-to-device uploads. pub fn staging…
          docstring: Ring of page-locked host buffers for host-to-device uploads.
        - id: impl_CudaMemoryManager.copy_d2d
          signature: "/// Device-to-device copy between two live regions of different arena classes, …"
          docstring: "Device-to-device copy between two live regions of different arena classes, orde…"
        - id: impl_CudaMemoryManager.upload
          signature: "/// Asynchronous upload from pinned memory. /// /// # Safety /// `src` must be …"
          docstring: Asynchronous upload from pinned memory.
        - id: impl_CudaMemoryManager.download
          signature: "/// Asynchronous download into pinned memory. /// /// # Safety /// `dst` must b…"
          docstring: Asynchronous download into pinned memory.
        - id: impl_CudaMemoryManager.slot
          signature: "fn slot(&self, region: Region) -> Result<usize, MemoryError> {"
        - id: impl_CudaMemoryManager.check_live
          signature: "fn check_live(&self, region: Region, bytes: u64) -> Result<(), MemoryError> {"
      - id: impl_DeviceMemory_for_CudaMemoryManager
        signature: "impl DeviceMemory for CudaMemoryManager {"
        children:
        - id: impl_DeviceMemory_for_CudaMemoryManager.alloc_in
          signature: "fn alloc_in(     &self,     kind: ArenaKind,     bytes: u64,     align: u64, ) …"
        - id: impl_DeviceMemory_for_CudaMemoryManager.free
          signature: "fn free(&self, region: Region) -> Result<(), MemoryError> {"
        - id: impl_DeviceMemory_for_CudaMemoryManager.release_after
          signature: "fn release_after(     &self,     region: Region,     event: EventHandle, ) -> R…"
        - id: impl_DeviceMemory_for_CudaMemoryManager.budget
          signature: "fn budget(&self) -> Result<Budget, MemoryError> {"
      - id: tests
        signature: "#[cfg(test)] mod tests {"
        children:
        - id: tests.rijjxp|xQOsKY
          signature: use super::*;
        - id: tests.manager
          signature: "fn manager() -> (DeviceHandle, CudaMemoryManager) {"
        - id: tests.upload_copy_download_roundtrip
          signature: "#[test] #[ignore = \"requires a CUDA device\"] fn upload_copy_download_roundtrip(…"
        - id: tests.oversized_plan_is_rejected_before_allocating
          signature: "#[test] fn oversized_plan_is_rejected_before_allocating() {"
```
/answer 6bcd-151 allow
Control Request:
```yaml
id: 6bcd-152
toolName: bash
arguments:
  cwd: /home/user/.cargo/registry/src
  script: d=$(ls -d */cudarc-0.19.9); cd $d; ls src src/nvrtc; grep -n "pub fn compile\|pub struct CompileOptions\|pub enum\|pub struct Ptx\|pub fn from_" src/nvrtc/safe.rs | head -40; grep -n "pub fn load_module\|pub fn load_function\|pub fn load_cubin\|impl CudaModule" src/driver/safe/*.rs | head; grep -n "pub struct CompileOptions" -A 60 src/nvrtc/safe.rs | grep "pub " | head -30
  reason: Verify cudarc 0.19.9 NVRTC/module API before writing the registry loader
```
/answer 6bcd-152 allow
ReasoningToken: 163
Control Request:
```yaml
id: 6bcd-153
result:
  structuredContent:
    stdout: |
      src:
      cublas
      cublaslt
      cudnn
      cufft
      cufile
      cupti
      curand
      cusolver
      cusolvermg
      cusparse
      cutensor
      driver
      lib.rs
      nccl
      nvrtc
      nvtx
      runtime
      types.rs

      src/nvrtc:
      mod.rs
      result.rs
      safe.rs
      sys
      16:pub struct Ptx(pub(crate) PtxKind);
      20:    pub fn from_file<P: Into<PathBuf>>(path: P) -> Self {
      26:    pub fn from_src<S: Into<String>>(src: S) -> Self {
      31:    pub fn from_binary(data: Vec<u8>) -> Self {
      100:pub fn compile_ptx<S: AsRef<str>>(src: S) -> Result<Ptx, CompileError> {
      116:pub fn compile_ptx_with_opts<S: AsRef<str>>(
      179:pub enum CompileError {
      231:pub struct CompileOptions {
      src/driver/safe/core.rs:2177:    pub fn load_module(
      src/driver/safe/core.rs:2217:impl CudaModule {
      src/driver/safe/core.rs:2219:    pub fn load_function(self: &Arc<Self>, fn_name: &str) -> Result<CudaFunction, DriverError> {
      231:pub struct CompileOptions {
      232-    pub ftz: Option<bool>,
      233-    pub prec_sqrt: Option<bool>,
      234-    pub prec_div: Option<bool>,
      235-    pub fmad: Option<bool>,
      236-    pub options: Vec<String>,
      237-    pub use_fast_math: Option<bool>,
      238-    pub maxrregcount: Option<usize>,
      239-    pub include_paths: Vec<String>,
      240-    pub arch: Option<&'static str>,
      241-    pub name: Option<String>,
```
/answer 6bcd-153 allow
Control Request:
```yaml
id: 6bcd-154
toolName: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src/resident/memory
    pattern: device_ptr|CudaSlice|fn base_ptr|pub fn
    include:
    - arena.rs
    limit: 40
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag
    pattern: nvcc|CUDA_COMPUTE_CAP|CUDA_PATH
    include:
    - config.toml
    - '*.sh'
    - build.rs
    exclude:
    - .bin/**
    - target/**
    limit: 10
  with_lineno: true
  reason: See how arena pointers are resolved and how nvcc is configured
```
/answer 6bcd-154 allow
Control Request:
```yaml
id: 6bcd-155
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src/resident/memory
      matches:
      - path: arena.rs
        matches:
        - lineno: 23
          match: "pub fn new(kind: ArenaKind, dev: DeviceKind, capacity: u64) -> Self {"
        - lineno: 33
          match: "pub fn kind(&self) -> ArenaKind {"
        - lineno: 37
          match: "pub fn capacity(&self) -> u64 {"
        - lineno: 42
          match: "pub fn available(&self) -> u64 {"
        - lineno: 46
          match: "pub fn pending(&self) -> usize {"
        - lineno: 50
          match: "pub fn is_live(&self, region: Region) -> bool {"
        - lineno: 55
          match: "pub fn alloc(&self, bytes: u64, align: u64) -> Result<Region, MemoryError> {"
        - lineno: 67
          match: "pub fn free(&self, region: Region) -> Result<(), MemoryError> {"
        - lineno: 75
          match: "pub fn release_after(&self, region: Region, event: EventHandle) -> Result<(), MemoryError> {"
        - lineno: 85
          match: "pub fn reclaim(&self) -> usize {"
        - lineno: 135
          match: "pub fn new(dev: DeviceKind, sizes: ArenaSizes) -> Self {"
        - lineno: 143
          match: "pub fn sizes(&self) -> &ArenaSizes {"
        - lineno: 147
          match: "pub fn get(&self, kind: ArenaKind) -> &ArenaAllocator {"
        - lineno: 151
          match: "pub fn for_region(&self, region: Region) -> Result<&ArenaAllocator, MemoryError> {"
        - lineno: 180
          match: "pub fn closed() -> Self {"
        - lineno: 193
          match: "pub fn open(self) {"
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag
      matches:
      - path: .cargo/config.toml
        matches:
        - lineno: 6
          match: "CUDA_PATH = { value = \"/usr/local/cuda-13.3\", force = false }"
        - lineno: 8
          match: "CUDA_COMPUTE_CAP = { value = \"75\", force = false }"
```
/answer 6bcd-155 allow
ReasoningToken: 981
Control Request:
```yaml
id: 6bcd-156
toolName: bash
arguments:
  cwd: /home/user/.cargo/registry/src
  script: "d=$(ls -d */cudarc-0.19.9); cd $d; sed -n 2170,2230p src/driver/safe/core.rs; grep -n \"pub fn clone_htod\\|pub fn clone_dtoh\\|pub fn launch_builder\\|pub fn memcpy_dtov\\|pub fn alloc_zeros\\|pub fn new_stream\\|pub unsafe fn launch\\|pub fn arg\" src/driver/safe/*.rs; sed -n 14,40p src/nvrtc/safe.rs; sed -n 100,178p src/nvrtc/safe.rs; grep -n \"enum PtxKind\" -A8 src/nvrtc/safe.rs; sed -n 75,95p /home/user/xyan/xy.ai.workbench/rag/src/resident/subengine/device.rs"
  reason: "Confirm exact cudarc signatures (load_module, launch builder, htod/dtoh, Ptx kinds) to avoid guessing"
```
/answer 6bcd-156 allow
Control Request:
```yaml
id: 6bcd-157
result:
  content:
  - type: text
    text: "Full output written to file (/tmp/mcpc-stdout-mqp_v5k4.log). Reduce the content to what is strictly needed: use targeted commands (grep, head, tail, awk, ast_outline) to extract only the relevant parts. This is a general output limit for all commands to prevent context bloat."
  structuredContent:
    stdout_file: /tmp/mcpc-stdout-mqp_v5k4.log
    stdout_file_file_stats:
      path: /tmp/mcpc-stdout-mqp_v5k4.log
      size_bytes: 7118
      lines: 206
      words: 746
      complexity: 0.602
      created: 2026-10-18T21:32:34.766324+00:00
      modified: 2026-10-18T21:32:34.766324+00:00
      accessed: 2026-10-18T21:32:34.767324+00:00
      line_length_max: 112
      line_length_min: 0
      line_length_avg: 33.55
      words_per_line_avg: 3.62
      checksum: 60c07c75b09faaae08b16deaff6e4a042534a726329ac33056b4bf5978bcd770
    stdout_file_ast_outline:
    - id: file
      lines: 1-206
      signature: "}"
```
/answer 6bcd-157 allow
Control Request:
```yaml
id: 6bcd-158
toolName: read_file
arguments:
  items:
  - path: /tmp/mcpc-stdout-mqp_v5k4.log
  reason: Read the saved cudarc API output
```
/answer 6bcd-158 allow
ReasoningToken: 511
Control Request:
```yaml
id: 6bcd-159
toolName: bash
arguments:
  cwd: /home/user/.cargo/registry/src
  script: "d=$(ls -d */cudarc-0.19.9); cd $d; grep -n \"pub fn \\|pub unsafe fn \" src/nvrtc/result.rs; grep -n \"fn build\" -A40 src/nvrtc/safe.rs | head -70; sed -n 205,240p src/driver/safe/launch.rs; grep -n \"pub fn arg\\|pub unsafe fn launch\" -B2 -A6 src/driver/safe/launch.rs | head -40; grep -n \"^nvrtc\\|^f16\\|^default\\|^cuda-version\" Cargo.toml | head"
  reason: Check cubin retrieval availability and exact launch builder signatures
```
/answer 6bcd-159 allow
Control Request:
```yaml
id: 6bcd-15a
result:
  content:
  - type: text
    text: "Full output written to file (/tmp/mcpc-stdout-yqz7i2of.log). Reduce the content to what is strictly needed: use targeted commands (grep, head, tail, awk, ast_outline) to extract only the relevant parts. This is a general output limit for all commands to prevent context bloat."
  structuredContent:
    stdout_file: /tmp/mcpc-stdout-yqz7i2of.log
    stdout_file_file_stats:
      path: /tmp/mcpc-stdout-yqz7i2of.log
      size_bytes: 3756
      lines: 107
      words: 357
      complexity: 0.569
      created: 2026-10-18T21:32:50.455148+00:00
      modified: 2026-10-18T21:32:50.455148+00:00
      accessed: 2026-10-18T21:32:50.455148+00:00
      line_length_max: 100
      line_length_min: 0
      line_length_avg: 34.1
      words_per_line_avg: 3.34
      checksum: 126d0a983db2e10dc4fd99cc0e415efba7c40f896370501f2e45071f730308ec
    stdout_file_ast_outline:
    - id: file
      lines: 1-107
      signature: "17:    pub fn result(self) -> Result<(), NvrtcError> {"
```
/answer 6bcd-15a allow
Control Request:
```yaml
id: 6bcd-15b
toolName: read_file
arguments:
  items:
  - path: /tmp/mcpc-stdout-yqz7i2of.log
  reason: Read cudarc nvrtc/launch API findings
```
/answer 6bcd-15b allow
ReasoningToken: 324
Control Request:
```yaml
id: 6bcd-15c
toolName: bash
arguments:
  cwd: /home/user/.cargo/registry/src
  script: "d=$(ls -d */cudarc-0.19.9); cd $d; grep -n \"pub mod\\|pub use\" src/nvrtc/mod.rs; grep -rn \"fn nvrtcGetCUBIN\" src/nvrtc/sys | head -3; grep -n \"pub struct LaunchConfig\" -A8 src/driver/safe/launch.rs; grep -n \"pub fn for_num_elems\" src/driver/safe/launch.rs; grep -n \"pub fn arg\" -B3 -A3 src/driver/safe/launch.rs | head -20; sed -n 40,60p src/nvrtc/result.rs; sed -n 108,132p src/nvrtc/result.rs"
  reason: Verify raw cubin access and LaunchConfig/arg signatures
```
/answer 6bcd-15c allow
Control Request:
```yaml
id: 6bcd-15d
result:
  structuredContent:
    stdout: |
      6:pub mod result;
      7:pub mod safe;
      10:pub mod sys;
      12:pub use safe::*;
      src/nvrtc/sys/mod.rs:216:pub unsafe fn nvrtcGetCUBIN(prog: nvrtcProgram, cubin: *mut ::core::ffi::c_char) -> nvrtcResult {
      src/nvrtc/sys/mod.rs:227:            fn nvrtcGetCUBIN(prog: nvrtcProgram, cubin: *mut ::core::ffi::c_char) -> nvrtcResult;
      src/nvrtc/sys/mod.rs:232:pub unsafe fn nvrtcGetCUBINSize(prog: nvrtcProgram, cubinSizeRet: *mut usize) -> nvrtcResult {
      15:pub struct LaunchConfig {
      16-    /// (width, height, depth) of grid in blocks
      17-    pub grid_dim: (u32, u32, u32),
      18-
      19-    /// (x, y, z) dimension of each thread block
      20-    pub block_dim: (u32, u32, u32),
      21-
      22-    /// Dynamic shared-memory size per thread block in bytes
      23-    pub shared_mem_bytes: u32,
      31:    pub fn for_num_elems(n: u32) -> Self {
      /// ```rust
      /// # use cudarc::nvrtc::result::*;
      /// let prog = create_program(c"extern \"C\" __global__ void kernel() { }", None).unwrap();
      /// ```
      ///
      /// Note that the returned `nvrtcProgram` may contain
      /// references to `src` and `name`. The memory containing the compiled
      /// code and name must not be dropped until the `nvrtcProgram` is destroyed.
      pub fn create_program(src: &CStr, name: Option<&CStr>) -> Result<sys::nvrtcProgram, NvrtcError> {
          let mut prog = MaybeUninit::uninit();
          unsafe {
              sys::nvrtcCreateProgram(
                  prog.as_mut_ptr(),
                  src.as_ptr(),
                  name.map(|n| n.as_ptr()).unwrap_or(std::ptr::null()),
                  0,
                  std::ptr::null(),
                  std::ptr::null(),
              )
              .result()?;
              Ok(prog.assume_init())
      /// and [nvrtcGetPTXSize() docs](https://docs.nvidia.com/cuda/nvrtc/index.html#group__compilation_1gc622d6ffb6fff71e209407da19612c1a).
      ///
      /// # Safety
      ///
      /// `prog` must be created from [create_program()] and not have been freed by [destroy_program()].
      #[allow(clippy::slow_vector_initialization)]
      pub unsafe fn get_ptx(prog: sys::nvrtcProgram) -> Result<Vec<c_char>, NvrtcError> {
          let mut size: usize = 0;
          sys::nvrtcGetPTXSize(prog, &mut size as *mut _).result()?;

          let mut ptx_src: Vec<c_char> = Vec::with_capacity(size);
          ptx_src.resize(size, 0);
          sys::nvrtcGetPTX(prog, ptx_src.as_mut_ptr()).result()?;
          Ok(ptx_src)
      }

      /// Extract log from a compiled program.
      ///
      /// See [nvrtcGetProgramLog() docs](https://docs.nvidia.com/cuda/nvrtc/index.html#group__compilation_1g74c550e5cab81efbd59e4f72579edbd1)
      /// and [nvrtcGetProgramLogSize() docs](https://docs.nvidia.com/cuda/nvrtc/index.html#group__compilation_1g59944bb118095ab53eec8994d056a18d).
      ///
      /// # Safety
      ///
      /// `prog` must be created from [create_program()] and not have been freed by [destroy_program()].
      #[allow(clippy::slow_vector_initialization)]
```
/answer 6bcd-15d allow
Control Request:
```yaml
id: 6bcd-15e
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/subengine/resource.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/subengine/lifecycle.rs
  reason: Check D1 lease/resource abstractions that the registry should plug into
```
/answer 6bcd-15e allow
ReasoningToken: 6475
Control Request:
```yaml
id: 6bcd-15f
result:
  structuredContent:
    results:
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
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/subengine/lifecycle.rs
      nodes:
      - id: 5AfduI|B2pQZe
        signature: use std::future::Future;
      - id: lUjdIX|dX0Yys
        signature: "const UNLOADED: u8 = 0;"
      - id: ResourceState
        signature: "#[derive(Clone, Copy, Debug, PartialEq, Eq)] pub enum ResourceState {"
      - id: CellConfig
        signature: "#[derive(Clone, Copy, Debug)] pub struct CellConfig {"
      - id: InitLimiter
        signature: /// Bounds the number of resources initialising concurrently across all cells.
        docstring: Bounds the number of resources initialising concurrently across all cells.
      - id: impl_InitLimiter
        signature: "impl InitLimiter {"
        children:
        - id: impl_InitLimiter.new
          signature: "pub fn new(max_parallel: usize) -> Self {"
        - id: impl_InitLimiter.acquire
          signature: "async fn acquire(&self) -> Result<tokio::sync::SemaphorePermit<'_>, SubengineEr…"
      - id: Shared
        signature: "struct Shared {"
      - id: impl_Shared
        signature: "impl Shared {"
        children:
        - id: impl_Shared.release_lease
          signature: "fn release_lease(&self) {"
      - id: ResourceCell
        signature: "/// A lazily initialised, evictable resource. /// /// Acquiring a lease takes t…"
        docstring: "A lazily initialised, evictable resource."
      - id: impl_ResourceCell_R_
        signature: "impl<R: Resource> ResourceCell<R> {"
        children:
        - id: impl_ResourceCell_R_.new
          signature: "pub fn new(key: ResourceKey, cfg: CellConfig) -> Self {"
        - id: impl_ResourceCell_R_.key
          signature: "pub fn key(&self) -> &ResourceKey {"
        - id: impl_ResourceCell_R_.state
          signature: "pub fn state(&self) -> ResourceState {"
        - id: impl_ResourceCell_R_.get_or_init
          signature: "/// Leases the resource, running `init` if it is not resident. Concurrent calle…"
          docstring: "Leases the resource, running `init` if it is not resident. Concurrent callers s…"
        - id: impl_ResourceCell_R_.evict_idle
          signature: "/// Unloads the resource when unleased, idle past the TTL and not vetoed by the…"
          docstring: "Unloads the resource when unleased, idle past the TTL and not vetoed by the res…"
      - id: ResourceLease
        signature: "pub struct ResourceLease<R> {"
      - id: impl_ResourceLease_R_
        signature: "impl<R> ResourceLease<R> {"
        children:
        - id: impl_ResourceLease_R_.begin_op
          signature: /// Marks an operation as running on the resource; eviction waits for the guard…
          docstring: Marks an operation as running on the resource; eviction waits for the guard to …
      - id: impl_Deref_for_ResourceLease_R_
        signature: "impl<R> Deref for ResourceLease<R> {"
        children:
        - id: impl_Deref_for_ResourceLease_R_.iyiBZo|illfNY
          signature: type Target = R;
        - id: impl_Deref_for_ResourceLease_R_.deref
          signature: "fn deref(&self) -> &R {"
      - id: impl_Drop_for_ResourceLease_R_
        signature: "impl<R> Drop for ResourceLease<R> {"
        children:
        - id: impl_Drop_for_ResourceLease_R_.drop
          signature: "fn drop(&mut self) {"
      - id: OpGuard
        signature: "pub struct OpGuard {"
      - id: impl_Drop_for_OpGuard
        signature: "impl Drop for OpGuard {"
        children:
        - id: impl_Drop_for_OpGuard.drop
          signature: "fn drop(&mut self) {"
      - id: tests
        signature: "#[cfg(test)] mod tests {"
        children:
        - id: tests.blg7Df|j2iqw0
          signature: use super::*;
        - id: tests.Dummy
          signature: struct Dummy(u32);
        - id: tests.impl_Resource_for_Dummy
          signature: "impl Resource for Dummy {"
          children:
          - id: tests.impl_Resource_for_Dummy.key
            signature: "fn key(&self) -> ResourceKey {"
        - id: tests.cell
          signature: "fn cell(ttl_ms: u64, backoff_ms: u64) -> ResourceCell<Dummy> {"
        - id: tests.init_err
          signature: "fn init_err() -> SubengineError {"
        - id: tests.parallel_callers_share_one_initialiser
          signature: "#[tokio::test] async fn parallel_callers_share_one_initialiser() {"
        - id: tests.futures_join
          signature: "async fn futures_join<F: Future>(fs: Vec<F>) -> Vec<F::Output> {"
        - id: tests.failed_init_leaves_cell_empty_and_backs_off
          signature: "#[tokio::test] async fn failed_init_leaves_cell_empty_and_backs_off() {"
        - id: tests.limiter_bounds_parallel_inits_across_cells
          signature: "#[tokio::test] async fn limiter_bounds_parallel_inits_across_cells() {"
        - id: tests.eviction_waits_for_lease_op_and_ttl
          signature: "#[tokio::test] async fn eviction_waits_for_lease_op_and_ttl() {"
        - id: tests.lease_taken_after_idle_clock_blocks_eviction
          signature: "#[tokio::test] async fn lease_taken_after_idle_clock_blocks_eviction() {"
        - id: tests.concurrent_leases_and_eviction_never_lose_a_resident_lease
          signature: "#[test] fn concurrent_leases_and_eviction_never_lose_a_resident_lease() {"
```
/answer 6bcd-15f allow
Control Request:
```yaml
id: 6bcd-160
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/subengine/exec.rs
    ids:
    - ParamBlock
    - impl_ParamBlock.push
    - impl_ParamBlock.as_bytes
    - ExecContext
  reason: Know ParamBlock push signature and ExecContext trait bounds for the Scorer trait
```
/answer 6bcd-160 allow
Control Request:
```yaml
id: 6bcd-161
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/subengine/exec.rs
      nodes:
      - id: ParamBlock
        type: ItemStruct
        code: |
          /// Fixed-capacity buffer for dynamic kernel parameters, staged on the host and mirrored into a
          /// small device region bound by the memory manager.
          #[derive(Debug)]
          pub struct ParamBlock {
              capacity: usize,
              buf: Vec<u8>,
              region: Option<Region>,
          }
      - id: impl_ParamBlock.push
        type: ImplItemFn
        code: |2
              /// Appends `value` naturally aligned and returns its byte offset.
              pub fn push<T: ParamValue>(&mut self, value: T) -> Result<usize, DeviceError> {
                  let offset = self.buf.len().next_multiple_of(T::SIZE);
                  let end = offset + T::SIZE;
                  if end > self.capacity {
                      return Err(DeviceError::ParamOverflow {
                          capacity: self.capacity,
                          requested: end,
                      });
                  }
                  self.buf.resize(offset, 0);
                  value.write(&mut self.buf);
                  Ok(offset)
              }
      - id: impl_ParamBlock.as_bytes
        type: ImplItemFn
        code: |2
              pub fn as_bytes(&self) -> &[u8] {
                  &self.buf
              }
      - id: ExecContext
        type: ItemTrait
        signature: "/// In-order execution queue of one device. Device handles, streams and pointer…"
        docstring: "In-order execution queue of one device. Device handles, streams and pointers ne…"
        children:
        - id: ExecContext.class
          type: TraitItemFn
          code: |2
                fn class(&self) -> LaneClass;
        - id: ExecContext.device
          type: TraitItemFn
          code: |2
                fn device(&self) -> DeviceKind;
        - id: ExecContext.submit
          type: TraitItemFn
          code: |2
                fn submit(&self, job: LaneJob) -> Result<(), DeviceError>;
        - id: ExecContext.record_event
          type: TraitItemFn
          code: |2
                /// Completes once everything submitted so far has finished.
                fn record_event(&self) -> Result<EventHandle, DeviceError>;
        - id: ExecContext.wait
          type: TraitItemFn
          code: |2
                /// Orders all later work on this lane after `event`.
                fn wait(&self, event: &EventHandle) -> Result<(), DeviceError>;
        - id: ExecContext.synchronize
          type: TraitItemFn
          code: |2
                /// Blocks until the lane has drained; returns the first job failure since the last call.
                fn synchronize(&self) -> Result<(), DeviceError>;
        - id: ExecContext.param_block
          type: TraitItemFn
          code: |2
                fn param_block(&mut self) -> &mut ParamBlock;
        - id: ExecContext.scratch
          type: TraitItemFn
          code: |2
                fn scratch(&self) -> &ScratchArena;
```
/answer 6bcd-161 allow
ReasoningToken: 141
Control Request:
```yaml
id: 6bcd-162
toolName: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src/resident
    pattern: use crate::resident::subengine::|pub use .*exec
    include:
    - '*.rs'
    limit: 12
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src/resident
    pattern: trait Clock|struct Ewma|F16_MASK_VALUE
    include:
    - '*.rs'
    limit: 8
  with_lineno: true
  reason: Find import paths for ExecContext/ParamBlock and avoid duplicating Ewma/clock helpers
```
/answer 6bcd-162 allow
Control Request:
```yaml
id: 6bcd-163
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src/resident
      matches:
      - path: tokenize/mod.rs
        matches:
        - lineno: 13
          match: "use crate::resident::subengine::{Resource, ResourceKey};"
      - path: error.rs
        matches:
        - lineno: 8
          match: pub use crate::resident::subengine::SubengineError;
        - lineno: 114
          match: pub use crate::resident::subengine::DeviceError;
      - path: subengine/mod.rs
        matches:
        - lineno: 17
          match: "pub use exec::{"
        - lineno: 22
          match: pub use exec::RecordingLane;
      - path: subengine/device.rs
        matches:
        - lineno: 20
          match: "use crate::resident::subengine::{"
        - lineno: 127
          match: "use crate::resident::subengine::{EventHandle, LaneJob, ParamBlock, ScratchArena};"
        - lineno: 128
          match: use crate::resident::subengine::exec::EventRepr;
      - path: subengine/exec.rs
        matches:
        - lineno: 14
          match: use crate::resident::subengine::DeviceError;
      - path: model/mod.rs
        matches:
        - lineno: 27
          match: "use crate::resident::subengine::{EventHandle, ExecContext, Resource, ResourceKey};"
        - lineno: 222
          match: "use crate::resident::subengine::{CpuLane, LaneClass, LaneConfig, RecordingLane};"
      - path: model/encoder.rs
        matches:
        - lineno: 12
          match: use crate::resident::subengine::ExecContext;
      warning: "Limit of 12 matches reached; further results may exist. Narrow the pattern, directory or include/exclude filters, or raise limit."
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src/resident
      matches:
      - path: subengine/resource.rs
        matches:
        - lineno: 30
          match: "pub struct Ewma {"
      - path: memory/mod.rs
        matches:
        - lineno: 21
          match: "pub use dtype::{DType, UnsupportedDType, F16_MASK_VALUE};"
      - path: memory/dtype.rs
        matches:
        - lineno: 10
          match: "pub const F16_MASK_VALUE: f32 = -1e4;"
      - path: mock.rs
        matches:
        - lineno: 10
          match: "pub trait Clock: Send + Sync {"
```
/answer 6bcd-163 allow
Text:
 Plan is settled, and a real GTX 1660 SUPER is present. I'm writing the CPU-testable core first: error, slicing, top-k, scorer traits and CPU scorers.
ReasoningToken: 117
Control Request:
```yaml
id: 6bcd-164
toolName: ast_create
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel/error.rs
    source: |
      use thiserror::Error;

      use crate::resident::error::{IndexError, IsRetryable};

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

      impl IsRetryable for KernelError {
          fn is_retryable(&self) -> bool {
              false
          }
      }

      impl From<KernelError> for IndexError {
          fn from(e: KernelError) -> Self {
              IndexError::KernelFailed { reason: e.to_string() }
          }
      }

      impl From<crate::resident::subengine::DeviceError> for KernelError {
          fn from(e: crate::resident::subengine::DeviceError) -> Self {
              use crate::resident::subengine::DeviceError;
              match e {
                  DeviceError::ParamOverflow { capacity, requested } => {
                      KernelError::ParamBlockOverflow { requested, capacity }
                  }
                  other => KernelError::LaunchFailed { source: Box::new(other) },
              }
          }
      }
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel/topk.rs
    source: |
      //! Bounded top-k selection with deterministic tie-breaking, plus the CPU merge of slice-local results.

      use std::cmp::{Ordering, Reverse};
      use std::collections::BinaryHeap;

      use super::KernelHit;

      /// Greater is better: higher score wins, equal scores prefer the lower offset.
      #[derive(Clone, Copy, Debug)]
      struct Ranked(KernelHit);

      impl PartialEq for Ranked {
          fn eq(&self, other: &Self) -> bool {
              self.cmp(other) == Ordering::Equal
          }
      }
      impl Eq for Ranked {}
      impl PartialOrd for Ranked {
          fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
              Some(self.cmp(other))
          }
      }
      impl Ord for Ranked {
          fn cmp(&self, other: &Self) -> Ordering {
              self.0
                  .score
                  .total_cmp(&other.0.score)
                  .then_with(|| other.0.offset.cmp(&self.0.offset))
          }
      }

      #[derive(Debug)]
      pub struct TopK {
          k: usize,
          heap: BinaryHeap<Reverse<Ranked>>,
      }

      impl TopK {
          pub fn new(k: usize) -> Self {
              Self { k, heap: BinaryHeap::with_capacity(k.saturating_add(1).min(4096)) }
          }

          pub fn k(&self) -> usize {
              self.k
          }

          pub fn len(&self) -> usize {
              self.heap.len()
          }

          pub fn is_empty(&self) -> bool {
              self.heap.is_empty()
          }

          /// NaN scores are dropped.
          pub fn push(&mut self, hit: KernelHit) {
              if self.k == 0 || hit.score.is_nan() {
                  return;
              }
              let cand = Ranked(hit);
              if self.heap.len() < self.k {
                  self.heap.push(Reverse(cand));
              } else if let Some(worst) = self.heap.peek() {
                  if cand > worst.0 {
                      self.heap.pop();
                      self.heap.push(Reverse(cand));
                  }
              }
          }

          pub fn extend(&mut self, hits: impl IntoIterator<Item = KernelHit>) {
              for h in hits {
                  self.push(h);
              }
          }

          /// Best first.
          pub fn into_sorted(self) -> Vec<KernelHit> {
              let mut v: Vec<Ranked> = self.heap.into_iter().map(|r| r.0).collect();
              v.sort_unstable_by(|a, b| b.cmp(a));
              v.into_iter().map(|r| r.0).collect()
          }
      }

      /// Merges slice-local results into the global top-`k`.
      pub fn merge(parts: impl IntoIterator<Item = Vec<KernelHit>>, k: usize) -> Vec<KernelHit> {
          let mut top = TopK::new(k);
          for part in parts {
              top.extend(part);
          }
          top.into_sorted()
      }

      #[cfg(test)]
      mod tests {
          use super::*;

          fn hit(offset: u64, score: f32) -> KernelHit {
              KernelHit { offset, score }
          }

          #[test]
          fn keeps_best_k_in_descending_order() {
              let mut t = TopK::new(3);
              for (i, s) in [0.1, 0.9, 0.5, 0.7, 0.3].into_iter().enumerate() {
                  t.push(hit(i as u64, s));
              }
              let got: Vec<u64> = t.into_sorted().iter().map(|h| h.offset).collect();
              assert_eq!(got, vec![1, 3, 2]);
          }

          #[test]
          fn ties_prefer_lower_offset() {
              let mut t = TopK::new(2);
              for off in [5, 3, 9, 1] {
                  t.push(hit(off, 1.0));
              }
              let got: Vec<u64> = t.into_sorted().iter().map(|h| h.offset).collect();
              assert_eq!(got, vec![1, 3]);
          }

          #[test]
          fn nan_and_zero_k_are_ignored() {
              let mut t = TopK::new(2);
              t.push(hit(0, f32::NAN));
              t.push(hit(1, 0.5));
              assert_eq!(t.len(), 1);
              let mut z = TopK::new(0);
              z.push(hit(0, 1.0));
              assert!(z.is_empty());
          }

          #[test]
          fn merge_over_slices_equals_global_selection() {
              let all: Vec<KernelHit> = (0..50u64).map(|i| hit(i, ((i * 37) % 23) as f32)).collect();
              let global = merge([all.clone()], 7);
              let parts = all.chunks(8).map(|c| merge([c.to_vec()], 7));
              assert_eq!(merge(parts, 7), global);
          }
      }
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel/slice.rs
    source: |
      //! Work-slice scheduling: bounded launches sized from observed throughput, with yield points only
      //! at slice boundaries.

      use std::time::Duration;

      use super::SliceRange;
      use crate::resident::subengine::Ewma;

      #[derive(Clone, Copy, Debug, PartialEq, Eq)]
      pub enum StopReason {
          Preempted,
          Deadline,
      }

      #[derive(Clone, Copy, Debug, PartialEq, Eq)]
      pub struct SliceOutcome {
          pub done: u32,
          /// Work left when stopped early; resume from here.
          pub remaining: Option<SliceRange>,
          pub stopped: Option<StopReason>,
      }

      /// Chooses slice lengths so one launch lasts about `target`.
      #[derive(Debug)]
      pub struct SliceSizer {
          target: Duration,
          min: u32,
          max: u32,
          rate: Ewma,
      }

      impl SliceSizer {
          pub const DEFAULT_TARGET: Duration = Duration::from_millis(2);

          pub fn new(target: Duration, min: u32, max: u32) -> Self {
              let min = min.max(1);
              Self { target, min, max: max.max(min), rate: Ewma::new(0.3) }
          }

          /// Slots per second; `None` before the first observation.
          pub fn throughput(&self) -> Option<f32> {
              self.rate.get()
          }

          /// Starts at the minimum until a throughput sample exists.
          pub fn next_len(&self) -> u32 {
              match self.rate.get() {
                  None => self.min,
                  Some(rate) => {
                      let slots = f64::from(rate) * self.target.as_secs_f64();
                      (slots as u64).clamp(u64::from(self.min), u64::from(self.max)) as u32
                  }
              }
          }

          pub fn record(&mut self, slots: u32, elapsed: Duration) {
              if slots == 0 {
                  return;
              }
              let secs = elapsed.as_secs_f64().max(1e-6);
              self.rate.observe((f64::from(slots) / secs) as f32);
          }
      }

      impl Default for SliceSizer {
          fn default() -> Self {
              Self::new(Self::DEFAULT_TARGET, 16, 1 << 20)
          }
      }

      /// Runs `range` as consecutive slices. `exec` returns the elapsed time of the slice it ran.
      /// `should_yield` is polled after each completed slice, never inside one. At least one slice always
      /// completes, so the caller makes progress even under permanent preemption pressure.
      pub fn run_sliced<E>(
          range: SliceRange,
          sizer: &mut SliceSizer,
          budget: Option<Duration>,
          mut should_yield: impl FnMut() -> bool,
          mut exec: impl FnMut(SliceRange) -> Result<Duration, E>,
      ) -> Result<SliceOutcome, E> {
          let mut rest = range;
          let mut done = 0u32;
          let mut spent = Duration::ZERO;
          while !rest.is_empty() {
              let (head, tail) = rest.split_at(sizer.next_len());
              let elapsed = exec(head)?;
              sizer.record(head.len(), elapsed);
              done += head.len();
              spent += elapsed;
              rest = tail;
              if rest.is_empty() {
                  break;
              }
              let stopped = if budget.is_some_and(|b| spent >= b) {
                  Some(StopReason::Deadline)
              } else if should_yield() {
                  Some(StopReason::Preempted)
              } else {
                  None
              };
              if stopped.is_some() {
                  return Ok(SliceOutcome { done, remaining: Some(rest), stopped });
              }
          }
          Ok(SliceOutcome { done, remaining: None, stopped: None })
      }

      #[cfg(test)]
      mod tests {
          use super::*;

          fn range(n: u32) -> SliceRange {
              SliceRange { segment: 0, slot_from: 0, slot_to: n }
          }

          fn sizer() -> SliceSizer {
              SliceSizer::new(Duration::from_millis(2), 10, 1000)
          }

          #[test]
          fn size_adapts_to_throughput_within_bounds() {
              let mut s = sizer();
              assert_eq!(s.next_len(), 10);
              s.record(100, Duration::from_millis(1));
              assert_eq!(s.next_len(), 200);
              let mut fast = sizer();
              fast.record(100, Duration::from_micros(1));
              assert_eq!(fast.next_len(), 1000);
              let mut slow = sizer();
              slow.record(1, Duration::from_secs(1));
              assert_eq!(slow.next_len(), 10);
          }

          #[test]
          fn slices_cover_range_exactly_once() {
              let mut seen = Vec::new();
              let out = run_sliced::<()>(range(95), &mut sizer(), None, || false, |r| {
                  seen.push((r.slot_from, r.slot_to));
                  Ok(Duration::from_millis(2))
              })
              .unwrap();
              assert_eq!(out.done, 95);
              assert!(out.remaining.is_none());
              assert_eq!(seen.first().unwrap().0, 0);
              assert_eq!(seen.last().unwrap().1, 95);
              assert!(seen.windows(2).all(|w| w[0].1 == w[1].0));
          }

          #[test]
          fn preemption_happens_only_at_boundaries_and_resumes() {
              let mut calls = 0;
              let mut ran = Vec::new();
              let out = run_sliced::<()>(
                  range(100),
                  &mut sizer(),
                  None,
                  || {
                      calls += 1;
                      true
                  },
                  |r| {
                      ran.push(r);
                      Ok(Duration::from_millis(2))
                  },
              )
              .unwrap();
              assert_eq!(out.stopped, Some(StopReason::Preempted));
              assert_eq!(ran.len(), 1);
              assert_eq!(calls, 1);
              let rest = out.remaining.unwrap();
              assert_eq!(rest.slot_from, out.done);
              assert_eq!(rest.slot_to, 100);
          }

          #[test]
          fn deadline_stops_between_slices() {
              let out = run_sliced::<()>(
                  range(1000),
                  &mut sizer(),
                  Some(Duration::from_millis(3)),
                  || false,
                  |_| Ok(Duration::from_millis(2)),
              )
              .unwrap();
              assert_eq!(out.stopped, Some(StopReason::Deadline));
              assert!(out.remaining.is_some());
          }

          #[test]
          fn exec_error_propagates() {
              let r = run_sliced(range(10), &mut sizer(), None, || false, |_| Err::<Duration, _>("boom"));
              assert_eq!(r.unwrap_err(), "boom");
          }
      }
  reason: "Create KernelError, top-k reduction and work-slice scheduler modules"
```
/answer 6bcd-164 allow
Control Request:
```yaml
id: 6bcd-165
toolName: ast_edit_marks
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel/mod.rs
    id: ModuleDoc
    start_marker: "//! Kernel abstraction: segment descriptors"
    end_marker: carries no pointers.
    content: |-
      //! Kernel abstraction: segment descriptors, hits, the `Kernel` trait and the scorers built on it.
      //!
      //! The kernel knows only segments, offsets and spans. Mapping an address back to an entry is the
      //! manager's job (`entry_of`, `entry_of_fixed`); the public API carries no pointers.
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel/mod.rs
    id: cuda
    start_marker: pub mod cuda;
    end_marker: pub mod cuda;
    content: |-
      pub mod cosine;
      pub mod cpu_simd;
      pub mod cuda;
      pub mod error;
      pub mod maxsim;
      pub mod registry;
      pub mod slice;
      pub mod slots;
      pub mod topk;

      pub use error::KernelError;
      pub use slice::{run_sliced, SliceOutcome, SliceSizer, StopReason};
      pub use slots::InMemorySlots;
      pub use topk::TopK;
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel/mod.rs
    id: TjIWN2|DqltD3
    start_marker: use std::time::Duration;
    end_marker: use crate::resident::error::IndexError;
    content: |-
      use std::time::Duration;

      use crate::resident::error::IndexError;
      use crate::resident::subengine::ExecContext;
  reason: Wire the new D6 modules and imports into kernel/mod.rs
```
/answer 6bcd-165 allow
ReasoningToken: 742
Control Request:
```yaml
id: 6bcd-166
toolName: ast_insert
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel/mod.rs
    id: entry_of
    position: after
    source: |
      /// Half-open slot range of one segment; the unit of work for a single launch.
      #[derive(Clone, Copy, Debug, PartialEq, Eq)]
      pub struct SliceRange {
          pub segment: u32,
          pub slot_from: u32,
          pub slot_to: u32,
      }

      impl SliceRange {
          pub fn len(&self) -> u32 {
              self.slot_to.saturating_sub(self.slot_from)
          }

          pub fn is_empty(&self) -> bool {
              self.len() == 0
          }

          /// Splits off at most `n` leading slots.
          pub fn split_at(self, n: u32) -> (SliceRange, SliceRange) {
              let mid = self.slot_from + n.min(self.len());
              (
                  SliceRange { slot_to: mid, ..self },
                  SliceRange { slot_from: mid, ..self },
              )
          }
      }

      /// Queries scored together so every document tile is read once. Each query is a row-major
      /// `tokens x dim` matrix; a dense query has one token.
      #[derive(Clone, Debug, PartialEq)]
      pub struct QueryBatch {
          dim: usize,
          queries: Vec<Vec<f32>>,
      }

      impl QueryBatch {
          pub fn new(dim: usize, queries: Vec<Vec<f32>>) -> Result<Self, KernelError> {
              for q in &queries {
                  if dim == 0 || q.is_empty() || q.len() % dim != 0 {
                      return Err(KernelError::ShapeMismatch { expected: dim, actual: q.len() });
                  }
              }
              Ok(Self { dim, queries })
          }

          pub fn dim(&self) -> usize {
              self.dim
          }

          pub fn len(&self) -> usize {
              self.queries.len()
          }

          pub fn is_empty(&self) -> bool {
              self.queries.is_empty()
          }

          pub fn tokens(&self, i: usize) -> usize {
              self.queries[i].len() / self.dim
          }

          pub fn data(&self, i: usize) -> &[f32] {
              &self.queries[i]
          }
      }

      /// Host-side view of slot vectors for CPU scoring (`F32`).
      pub trait SlotSource: Send + Sync {
          fn dim(&self) -> usize;
          fn slot_count(&self, segment: u32) -> u32;
          /// Offset reported in `KernelHit` for this slot.
          fn slot_offset(&self, segment: u32, slot: u32) -> u64;
          /// Row-major `tokens x dim`, valid tokens only.
          fn slot_vectors(&self, segment: u32, slot: u32) -> Option<&[f32]>;
      }

      /// Raw scores per slot; the top-k reduction is shared in `TopKScorer`.
      pub trait SlotScorer: Send + Sync {
          /// One vector per query, one score per slot of `range` in slot order.
          fn scores(&self, queries: &QueryBatch, range: SliceRange) -> Result<Vec<Vec<f32>>, KernelError>;

          fn slot_offset(&self, segment: u32, slot: u32) -> u64;
      }

      /// Scores one slice for all queries of a batch; the result holds the slice-local top-k per query.
      pub trait Scorer: Send + Sync {
          fn score(
              &self,
              queries: &QueryBatch,
              range: SliceRange,
              lane: &mut dyn ExecContext,
          ) -> Result<Vec<Vec<KernelHit>>, KernelError>;
      }

      #[derive(Debug)]
      pub struct TopKScorer<S> {
          inner: S,
          k: usize,
      }

      impl<S: SlotScorer> TopKScorer<S> {
          pub fn new(inner: S, k: usize) -> Self {
              Self { inner, k }
          }

          pub fn k(&self) -> usize {
              self.k
          }
      }

      impl<S: SlotScorer> Scorer for TopKScorer<S> {
          fn score(
              &self,
              queries: &QueryBatch,
              range: SliceRange,
              _lane: &mut dyn ExecContext,
          ) -> Result<Vec<Vec<KernelHit>>, KernelError> {
              let scores = self.inner.scores(queries, range)?;
              Ok(scores
                  .into_iter()
                  .map(|per_slot| {
                      let mut top = TopK::new(self.k);
                      for (i, score) in per_slot.into_iter().enumerate() {
                          let slot = range.slot_from + i as u32;
                          top.push(KernelHit {
                              offset: self.inner.slot_offset(range.segment, slot),
                              score,
                          });
                      }
                      top.into_sorted()
                  })
                  .collect())
          }
      }

      /// Scores several slices (e.g. a scope's ranges) and merges them into the global top-k per query.
      pub fn score_ranges(
          scorer: &dyn Scorer,
          queries: &QueryBatch,
          ranges: &[SliceRange],
          k: usize,
          lane: &mut dyn ExecContext,
      ) -> Result<Vec<Vec<KernelHit>>, KernelError> {
          let mut parts: Vec<Vec<Vec<KernelHit>>> = vec![Vec::new(); queries.len()];
          for &r in ranges {
              for (qi, hits) in scorer.score(queries, r, lane)?.into_iter().enumerate() {
                  parts[qi].push(hits);
              }
          }
          Ok(parts.into_iter().map(|p| topk::merge(p, k)).collect())
      }
  reason: "Add SliceRange, QueryBatch, Scorer/SlotScorer traits and the shared top-k scorer to kernel/mod.rs"
```
/answer 6bcd-166 allow
ReasoningToken: 4781
Control Request:
```yaml
id: 6bcd-167
toolName: ast_create
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel/slots.rs
    source: |
      //! In-memory `SlotSource` for CPU scoring and tests.

      use super::{KernelError, SliceRange, SlotSource};

      #[derive(Clone, Debug)]
      pub struct InMemorySlots {
          dim: usize,
          segments: Vec<Vec<Vec<f32>>>,
          offsets: Vec<Vec<u64>>,
      }

      impl InMemorySlots {
          /// `segments[s][i]` holds the row-major `tokens x dim` vectors of slot `i`. Offsets are byte
          /// prefix sums per segment.
          pub fn new(dim: usize, segments: Vec<Vec<Vec<f32>>>) -> Self {
              let offsets = segments
                  .iter()
                  .map(|seg| {
                      let mut acc = 0u64;
                      seg.iter()
                          .map(|v| {
                              let at = acc;
                              acc += (v.len() * std::mem::size_of::<f32>()) as u64;
                              at
                          })
                          .collect()
                  })
                  .collect();
              Self { dim, segments, offsets }
          }
      }

      impl SlotSource for InMemorySlots {
          fn dim(&self) -> usize {
              self.dim
          }

          fn slot_count(&self, segment: u32) -> u32 {
              self.segments.get(segment as usize).map_or(0, |s| s.len() as u32)
          }

          fn slot_offset(&self, segment: u32, slot: u32) -> u64 {
              self.offsets[segment as usize][slot as usize]
          }

          fn slot_vectors(&self, segment: u32, slot: u32) -> Option<&[f32]> {
              self.segments.get(segment as usize)?.get(slot as usize).map(Vec::as_slice)
          }
      }

      pub(crate) fn check_range(source: &dyn SlotSource, range: SliceRange) -> Result<(), KernelError> {
          let count = source.slot_count(range.segment);
          if range.slot_from > range.slot_to || range.slot_to > count {
              return Err(KernelError::SliceOutOfRange {
                  segment: range.segment,
                  slot_from: range.slot_from,
                  slot_to: range.slot_to,
                  count,
              });
          }
          Ok(())
      }
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel/cpu_simd.rs
    source: |
      //! CPU scoring primitives; loops are shaped for autovectorisation (8-wide accumulators).

      use std::sync::Arc;

      use super::slots::check_range;
      use super::{KernelError, QueryBatch, SliceRange, SlotScorer, SlotSource};
      use crate::resident::memory::F16_MASK_VALUE;

      pub fn dot(a: &[f32], b: &[f32]) -> f32 {
          debug_assert_eq!(a.len(), b.len());
          let ca = a.chunks_exact(8);
          let cb = b.chunks_exact(8);
          let (ra, rb) = (ca.remainder(), cb.remainder());
          let mut acc = [0f32; 8];
          for (x, y) in ca.zip(cb) {
              for i in 0..8 {
                  acc[i] += x[i] * y[i];
              }
          }
          let tail: f32 = ra.iter().zip(rb).map(|(x, y)| x * y).sum();
          acc.iter().sum::<f32>() + tail
      }

      /// Sum over query tokens of the best dot product against any document token. A document without
      /// tokens scores every query token with the f16-safe mask value, matching the GPU path.
      pub fn maxsim(query: &[f32], doc: &[f32], dim: usize) -> f32 {
          let q_tokens = query.len() / dim;
          if doc.is_empty() {
              return F16_MASK_VALUE * q_tokens as f32;
          }
          query
              .chunks_exact(dim)
              .map(|q| doc.chunks_exact(dim).fold(f32::NEG_INFINITY, |best, d| best.max(dot(q, d))))
              .sum()
      }

      pub fn normalize(v: &mut [f32]) {
          let norm = dot(v, v).sqrt();
          if norm > 0.0 {
              v.iter_mut().for_each(|x| *x /= norm);
          }
      }

      /// Slot-wise scorer: `score(query_i, slot)`; slots are the outer loop so a slot is read once for
      /// every query of the batch.
      pub(crate) fn scan_slots(
          source: &dyn SlotSource,
          queries: &QueryBatch,
          range: SliceRange,
          mut score: impl FnMut(&[f32], &[f32]) -> Result<f32, KernelError>,
      ) -> Result<Vec<Vec<f32>>, KernelError> {
          check_range(source, range)?;
          if queries.dim() != source.dim() {
              return Err(KernelError::ShapeMismatch { expected: source.dim(), actual: queries.dim() });
          }
          let mut out = vec![Vec::with_capacity(range.len() as usize); queries.len()];
          for slot in range.slot_from..range.slot_to {
              let doc = source.slot_vectors(range.segment, slot).unwrap_or(&[]);
              if doc.len() % queries.dim() != 0 {
                  return Err(KernelError::ShapeMismatch { expected: queries.dim(), actual: doc.len() });
              }
              for (qi, scores) in out.iter_mut().enumerate() {
                  scores.push(score(queries.data(qi), doc)?);
              }
          }
          Ok(out)
      }

      pub struct CpuMaxSim {
          source: Arc<dyn SlotSource>,
      }

      impl CpuMaxSim {
          pub fn new(source: Arc<dyn SlotSource>) -> Self {
              Self { source }
          }
      }

      impl SlotScorer for CpuMaxSim {
          fn scores(&self, queries: &QueryBatch, range: SliceRange) -> Result<Vec<Vec<f32>>, KernelError> {
              let dim = queries.dim();
              scan_slots(self.source.as_ref(), queries, range, |q, d| Ok(maxsim(q, d, dim)))
          }

          fn slot_offset(&self, segment: u32, slot: u32) -> u64 {
              self.source.slot_offset(segment, slot)
          }
      }

      #[cfg(test)]
      mod tests {
          use super::*;

          #[test]
          fn dot_handles_remainders() {
              for n in [0usize, 1, 7, 8, 9, 17, 384] {
                  let a: Vec<f32> = (0..n).map(|i| i as f32 * 0.5).collect();
                  let b: Vec<f32> = (0..n).map(|i| 1.0 - i as f32 * 0.25).collect();
                  let want: f32 = a.iter().zip(&b).map(|(x, y)| x * y).sum();
                  assert!((dot(&a, &b) - want).abs() <= 1e-3 * want.abs().max(1.0), "n={n}");
              }
          }

          #[test]
          fn maxsim_matches_hand_computation() {
              let q = [1.0, 0.0, 0.0, 1.0];
              let d = [0.5, 0.2, 0.1, 0.9, 0.3, 0.3];
              assert!((maxsim(&q, &d, 2) - (0.5 + 0.9)).abs() < 1e-6);
          }

          #[test]
          fn empty_document_scores_mask_value_per_query_token() {
              let q = [1.0, 0.0, 0.0, 1.0];
              let s = maxsim(&q, &[], 2);
              assert_eq!(s, 2.0 * F16_MASK_VALUE);
              assert!(s.is_finite());
          }

          #[test]
          fn normalize_yields_unit_length() {
              let mut v = vec![3.0, 4.0];
              normalize(&mut v);
              assert!((dot(&v, &v) - 1.0).abs() < 1e-6);
              let mut z = vec![0.0; 3];
              normalize(&mut z);
              assert_eq!(z, vec![0.0; 3]);
          }
      }
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel/cosine.rs
    source: |
      //! Dense scoring path: cosine similarity as the dot product of L2-normalised vectors.

      use std::sync::Arc;

      use super::cpu_simd::{dot, scan_slots};
      use super::{KernelError, QueryBatch, SliceRange, SlotScorer, SlotSource, TopKScorer};

      pub struct CosineSlots {
          source: Arc<dyn SlotSource>,
      }

      impl CosineSlots {
          pub fn new(source: Arc<dyn SlotSource>) -> Self {
              Self { source }
          }
      }

      impl SlotScorer for CosineSlots {
          fn scores(&self, queries: &QueryBatch, range: SliceRange) -> Result<Vec<Vec<f32>>, KernelError> {
              let dim = queries.dim();
              for qi in 0..queries.len() {
                  if queries.tokens(qi) != 1 {
                      return Err(KernelError::ShapeMismatch { expected: dim, actual: queries.data(qi).len() });
                  }
              }
              scan_slots(self.source.as_ref(), queries, range, |q, d| {
                  if d.len() != dim {
                      return Err(KernelError::ShapeMismatch { expected: dim, actual: d.len() });
                  }
                  Ok(dot(q, d))
              })
          }

          fn slot_offset(&self, segment: u32, slot: u32) -> u64 {
              self.source.slot_offset(segment, slot)
          }
      }

      pub type CosineScorer = TopKScorer<CosineSlots>;

      pub fn cosine_scorer(source: Arc<dyn SlotSource>, k: usize) -> CosineScorer {
          TopKScorer::new(CosineSlots::new(source), k)
      }

      #[cfg(test)]
      mod tests {
          use super::*;
          use crate::resident::kernel::cpu_simd::normalize;
          use crate::resident::kernel::{score_ranges, InMemorySlots};
          use crate::resident::subengine::{LaneClass, LaneConfig, RecordingLane};
          use crate::resident::memory::{ArenaId, Region};

          fn lane() -> RecordingLane {
              let scratch = Region { arena: ArenaId::default(), offset: 0, len: 0, gen: 0 };
              RecordingLane::new(LaneClass::Query, LaneConfig::new(scratch))
          }

          fn unit(v: &[f32]) -> Vec<f32> {
              let mut v = v.to_vec();
              normalize(&mut v);
              v
          }

          fn source() -> Arc<InMemorySlots> {
              let docs = vec![unit(&[1.0, 0.0]), unit(&[1.0, 1.0]), unit(&[0.0, 1.0]), unit(&[-1.0, 0.0])];
              Arc::new(InMemorySlots::new(2, vec![docs.clone(), docs]))
          }

          #[test]
          fn ranks_by_cosine_with_offsets() {
              let scorer = cosine_scorer(source(), 2);
              let q = QueryBatch::new(2, vec![unit(&[1.0, 0.1])]).unwrap();
              let hits = scorer
                  .score(&q, SliceRange { segment: 0, slot_from: 0, slot_to: 4 }, &mut lane())
                  .unwrap();
              assert_eq!(hits[0].len(), 2);
              assert_eq!(hits[0][0].offset, 0);
              assert_eq!(hits[0][1].offset, 8);
              assert!(hits[0][0].score > hits[0][1].score);
          }

          #[test]
          fn batched_queries_match_individual_runs() {
              let scorer = cosine_scorer(source(), 3);
              let a = unit(&[1.0, 0.0]);
              let b = unit(&[0.0, 1.0]);
              let r = SliceRange { segment: 0, slot_from: 0, slot_to: 4 };
              let both = scorer
                  .score(&QueryBatch::new(2, vec![a.clone(), b.clone()]).unwrap(), r, &mut lane())
                  .unwrap();
              let only_b = scorer.score(&QueryBatch::new(2, vec![b]).unwrap(), r, &mut lane()).unwrap();
              assert_eq!(both[1], only_b[0]);
          }

          #[test]
          fn scope_subset_merges_ranges_across_segments() {
              let scorer = cosine_scorer(source(), 10);
              let q = QueryBatch::new(2, vec![unit(&[0.0, 1.0])]).unwrap();
              let ranges = [
                  SliceRange { segment: 0, slot_from: 1, slot_to: 3 },
                  SliceRange { segment: 1, slot_from: 2, slot_to: 3 },
              ];
              let hits = score_ranges(&scorer, &q, &ranges, 2, &mut lane()).unwrap();
              assert_eq!(hits[0].len(), 2);
              assert!((hits[0][0].score - 1.0).abs() < 1e-6);
          }

          #[test]
          fn rejects_multi_token_queries_and_bad_ranges() {
              let scorer = cosine_scorer(source(), 1);
              let multi = QueryBatch::new(2, vec![vec![1.0, 0.0, 0.0, 1.0]]).unwrap();
              let r = SliceRange { segment: 0, slot_from: 0, slot_to: 1 };
              assert!(matches!(
                  scorer.score(&multi, r, &mut lane()),
                  Err(KernelError::ShapeMismatch { .. })
              ));
              let q = QueryBatch::new(2, vec![vec![1.0, 0.0]]).unwrap();
              let far = SliceRange { segment: 0, slot_from: 0, slot_to: 99 };
              assert!(matches!(
                  scorer.score(&q, far, &mut lane()),
                  Err(KernelError::SliceOutOfRange { .. })
              ));
          }
      }
  reason: "Create CPU slot source, SIMD scoring primitives and the cosine scorer with tests"
```
/answer 6bcd-167 allow
ReasoningToken: 40
Control Request:
```yaml
id: 6bcd-168
toolName: ast_create
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel/maxsim.rs
    source: |
      //! Late-interaction scoring (ColBERT-style MaxSim) behind one op; the implementation is
      //! swappable (CPU SIMD, Candle tile chain, fused kernel).

      use std::sync::Arc;

      use super::cpu_simd::CpuMaxSim;
      use super::{KernelHit, KernelError, QueryBatch, Scorer, SliceRange, SlotScorer, SlotSource, TopKScorer};
      use crate::resident::subengine::ExecContext;

      impl SlotScorer for Box<dyn SlotScorer> {
          fn scores(&self, queries: &QueryBatch, range: SliceRange) -> Result<Vec<Vec<f32>>, KernelError> {
              (**self).scores(queries, range)
          }

          fn slot_offset(&self, segment: u32, slot: u32) -> u64 {
              (**self).slot_offset(segment, slot)
          }
      }

      pub struct MaxSimOp {
          inner: TopKScorer<Box<dyn SlotScorer>>,
      }

      impl MaxSimOp {
          pub fn with_slots(slots: Box<dyn SlotScorer>, k: usize) -> Self {
              Self { inner: TopKScorer::new(slots, k) }
          }

          pub fn cpu(source: Arc<dyn SlotSource>, k: usize) -> Self {
              Self::with_slots(Box::new(CpuMaxSim::new(source)), k)
          }

          #[cfg(feature = "candle")]
          pub fn candle_cpu(
              source: Arc<dyn SlotSource>,
              k: usize,
              dtype: crate::resident::memory::DType,
              tile_tokens: usize,
          ) -> Result<Self, KernelError> {
              let chain = candle_chain::CandleMaxSim::new(source, candle_core::Device::Cpu, dtype, tile_tokens)?;
              Ok(Self::with_slots(Box::new(chain), k))
          }

          #[cfg(feature = "cuda")]
          pub fn candle_cuda(
              source: Arc<dyn SlotSource>,
              k: usize,
              ordinal: usize,
              dtype: crate::resident::memory::DType,
              tile_tokens: usize,
          ) -> Result<Self, KernelError> {
              let device = candle_core::Device::new_cuda(ordinal)
                  .map_err(|e| KernelError::Unavailable { reason: e.to_string() })?;
              let chain = candle_chain::CandleMaxSim::new(source, device, dtype, tile_tokens)?;
              Ok(Self::with_slots(Box::new(chain), k))
          }
      }

      impl Scorer for MaxSimOp {
          fn score(
              &self,
              queries: &QueryBatch,
              range: SliceRange,
              lane: &mut dyn ExecContext,
          ) -> Result<Vec<Vec<KernelHit>>, KernelError> {
              self.inner.score(queries, range, lane)
          }
      }

      #[cfg(feature = "candle")]
      pub(crate) mod candle_chain {
          use std::sync::Arc;

          use candle_core::{DType as CDType, Device, Tensor};

          use crate::resident::kernel::slots::check_range;
          use crate::resident::kernel::{KernelError, QueryBatch, SliceRange, SlotScorer, SlotSource};
          use crate::resident::memory::{DType, F16_MASK_VALUE};

          fn fail(e: candle_core::Error) -> KernelError {
              KernelError::LaunchFailed { source: Box::new(e) }
          }

          struct Tile {
              /// `dim x tile`, already transposed for the matmul.
              docs_t: Tensor,
              /// Additive mask for padded columns of the last tile.
              mask: Option<Tensor>,
          }

          /// `matmul -> mask -> max` per document tile with a running maximum, then a sum over query
          /// tokens. Intermediate similarity matrices stay at `query_tokens x tile`.
          pub struct CandleMaxSim {
              source: Arc<dyn SlotSource>,
              device: Device,
              dtype: CDType,
              tile: usize,
          }

          impl CandleMaxSim {
              pub fn new(
                  source: Arc<dyn SlotSource>,
                  device: Device,
                  dtype: DType,
                  tile: usize,
              ) -> Result<Self, KernelError> {
                  let dtype = CDType::try_from(dtype)
                      .map_err(|e| KernelError::Unavailable { reason: e.to_string() })?;
                  Ok(Self { source, device, dtype, tile: tile.max(1) })
              }

              fn tiles(&self, doc: &[f32], dim: usize) -> candle_core::Result<Vec<Tile>> {
                  let tokens = doc.len() / dim;
                  let mut tiles = Vec::new();
                  for start in (0..tokens).step_by(self.tile) {
                      let take = self.tile.min(tokens - start);
                      let mut buf = vec![0f32; self.tile * dim];
                      buf[..take * dim].copy_from_slice(&doc[start * dim..(start + take) * dim]);
                      let docs_t = Tensor::from_vec(buf, (self.tile, dim), &self.device)?
                          .to_dtype(self.dtype)?
                          .t()?
                          .contiguous()?;
                      let mask = if take < self.tile {
                          let m: Vec<f32> = (0..self.tile)
                              .map(|i| if i < take { 0.0 } else { F16_MASK_VALUE })
                              .collect();
                          Some(Tensor::from_vec(m, (1, self.tile), &self.device)?.to_dtype(self.dtype)?)
                      } else {
                          None
                      };
                      tiles.push(Tile { docs_t, mask });
                  }
                  Ok(tiles)
              }

              fn score(&self, query: &Tensor, tiles: &[Tile]) -> candle_core::Result<f32> {
                  let mut running: Option<Tensor> = None;
                  for t in tiles {
                      let mut sim = query.matmul(&t.docs_t)?;
                      if let Some(mask) = &t.mask {
                          sim = sim.broadcast_add(mask)?;
                      }
                      let best = sim.max(1)?;
                      running = Some(match running {
                          None => best,
                          Some(r) => r.maximum(&best)?,
                      });
                  }
                  match running {
                      Some(r) => r.to_dtype(CDType::F32)?.sum_all()?.to_scalar::<f32>(),
                      None => Ok(F16_MASK_VALUE * query.dim(0)? as f32),
                  }
              }
          }

          impl SlotScorer for CandleMaxSim {
              fn scores(
                  &self,
                  queries: &QueryBatch,
                  range: SliceRange,
              ) -> Result<Vec<Vec<f32>>, KernelError> {
                  check_range(self.source.as_ref(), range)?;
                  let dim = queries.dim();
                  if dim != self.source.dim() {
                      return Err(KernelError::ShapeMismatch { expected: self.source.dim(), actual: dim });
                  }
                  let qs = (0..queries.len())
                      .map(|i| {
                          Tensor::from_slice(queries.data(i), (queries.tokens(i), dim), &self.device)?
                              .to_dtype(self.dtype)
                      })
                      .collect::<candle_core::Result<Vec<_>>>()
                      .map_err(fail)?;
                  let mut out = vec![Vec::with_capacity(range.len() as usize); qs.len()];
                  for slot in range.slot_from..range.slot_to {
                      let doc = self.source.slot_vectors(range.segment, slot).unwrap_or(&[]);
                      if doc.len() % dim != 0 {
                          return Err(KernelError::ShapeMismatch { expected: dim, actual: doc.len() });
                      }
                      let tiles = self.tiles(doc, dim).map_err(fail)?;
                      for (qi, q) in qs.iter().enumerate() {
                          out[qi].push(self.score(q, &tiles).map_err(fail)?);
                      }
                  }
                  Ok(out)
              }

              fn slot_offset(&self, segment: u32, slot: u32) -> u64 {
                  self.source.slot_offset(segment, slot)
              }
          }
      }

      #[cfg(test)]
      mod tests {
          use super::*;
          use crate::resident::kernel::InMemorySlots;
          use crate::resident::memory::{ArenaId, Region};
          use crate::resident::subengine::{LaneClass, LaneConfig, RecordingLane};

          fn lane() -> RecordingLane {
              let scratch = Region { arena: ArenaId::default(), offset: 0, len: 0, gen: 0 };
              RecordingLane::new(LaneClass::Query, LaneConfig::new(scratch))
          }

          fn tokens(n: usize, dim: usize, seed: u32) -> Vec<f32> {
              (0..n * dim)
                  .map(|i| (((i as u32).wrapping_mul(2654435761).wrapping_add(seed * 97)) % 1000) as f32 / 500.0 - 1.0)
                  .collect()
          }

          fn source(dim: usize) -> Arc<InMemorySlots> {
              let docs = vec![tokens(5, dim, 1), tokens(12, dim, 2), tokens(1, dim, 3), Vec::new(), tokens(8, dim, 4)];
              Arc::new(InMemorySlots::new(dim, vec![docs]))
          }

          const R: SliceRange = SliceRange { segment: 0, slot_from: 0, slot_to: 5 };

          #[test]
          fn cpu_op_ranks_by_maxsim_and_empty_doc_is_last() {
              let dim = 4;
              let op = MaxSimOp::cpu(source(dim), 5);
              let q = QueryBatch::new(dim, vec![tokens(3, dim, 9)]).unwrap();
              let hits = op.score(&q, R, &mut lane()).unwrap();
              assert_eq!(hits[0].len(), 5);
              assert!(hits[0].windows(2).all(|w| w[0].score >= w[1].score));
              let last = hits[0].last().unwrap();
              assert_eq!(last.offset, source(dim).slot_offset(0, 3));
              assert!(last.score.is_finite());
          }

          #[test]
          fn cpu_op_top_k_limits_results() {
              let dim = 4;
              let op = MaxSimOp::cpu(source(dim), 2);
              let q = QueryBatch::new(dim, vec![tokens(2, dim, 5), tokens(4, dim, 6)]).unwrap();
              let hits = op.score(&q, R, &mut lane()).unwrap();
              assert!(hits.iter().all(|h| h.len() == 2));
          }

          #[cfg(feature = "candle")]
          mod candle {
              use super::*;
              use crate::resident::memory::DType;

              fn per_slot(op: &dyn SlotScorer, q: &QueryBatch) -> Vec<Vec<f32>> {
                  op.scores(q, R).unwrap()
              }

              #[test]
              fn f32_tile_chain_matches_cpu_reference_incl_padding() {
                  let dim = 8;
                  let src: Arc<dyn SlotSource> = source(dim);
                  let q = QueryBatch::new(dim, vec![tokens(4, dim, 7), tokens(2, dim, 8)]).unwrap();
                  let want = per_slot(&CpuMaxSim::new(src.clone()), &q);
                  for tile in [1usize, 3, 4, 64] {
                      let chain =
                          candle_chain::CandleMaxSim::new(src.clone(), candle_core::Device::Cpu, DType::F32, tile)
                              .unwrap();
                      let got = per_slot(&chain, &q);
                      for (g, w) in got.iter().flatten().zip(want.iter().flatten()) {
                          assert!((g - w).abs() < 1e-3, "tile={tile} got={g} want={w}");
                      }
                  }
              }

              #[test]
              fn f16_masking_never_produces_nan() {
                  let dim = 8;
                  let src: Arc<dyn SlotSource> = source(dim);
                  let q = QueryBatch::new(dim, vec![tokens(4, dim, 7)]).unwrap();
                  let want = per_slot(&CpuMaxSim::new(src.clone()), &q);
                  let chain =
                      candle_chain::CandleMaxSim::new(src, candle_core::Device::Cpu, DType::F16, 4).unwrap();
                  let got = per_slot(&chain, &q);
                  for (g, w) in got[0].iter().zip(&want[0]) {
                      assert!(g.is_finite());
                      assert!((g - w).abs() < 0.1 + 0.01 * w.abs(), "got={g} want={w}");
                  }
              }

              #[test]
              fn op_wraps_candle_chain() {
                  let dim = 8;
                  let op = MaxSimOp::candle_cpu(source(dim), 3, DType::F32, 4).unwrap();
                  let q = QueryBatch::new(dim, vec![tokens(4, dim, 7)]).unwrap();
                  assert_eq!(op.score(&q, R, &mut lane()).unwrap()[0].len(), 3);
              }
          }
      }
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel/registry.rs
    source: |
      //! Lazily populated kernel table keyed by `(name, dtype, sm, variant)`.

      use std::collections::HashMap;
      use std::sync::Arc;

      use parking_lot::RwLock;

      use super::KernelError;
      use crate::resident::memory::DType;
      use crate::resident::subengine::{Resource, ResourceKey};

      /// `variant` selects compile-time specialisations such as the embedding dimension.
      #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
      pub struct KernelKey {
          pub name: &'static str,
          pub dtype: DType,
          pub sm: u32,
          pub variant: u32,
      }

      impl KernelKey {
          pub const fn new(name: &'static str, dtype: DType, sm: u32, variant: u32) -> Self {
              Self { name, dtype, sm, variant }
          }
      }

      /// Produces the handle for a key, e.g. by compiling with NVRTC.
      pub trait KernelLoader<F>: Send + Sync {
          fn load(&self, key: &KernelKey) -> Result<F, KernelError>;
      }

      pub struct KernelRegistry<F> {
          loader: Arc<dyn KernelLoader<F>>,
          entries: RwLock<HashMap<KernelKey, F>>,
      }

      impl<F: Clone + Send + Sync + 'static> KernelRegistry<F> {
          pub fn new(loader: Arc<dyn KernelLoader<F>>) -> Self {
              Self { loader, entries: RwLock::new(HashMap::new()) }
          }

          /// Loads on first use. Loading holds the write lock, so concurrent callers never compile the
          /// same key twice.
          pub fn get(&self, key: &KernelKey) -> Result<F, KernelError> {
              if let Some(f) = self.entries.read().get(key) {
                  return Ok(f.clone());
              }
              let mut entries = self.entries.write();
              if let Some(f) = entries.get(key) {
                  return Ok(f.clone());
              }
              let started = std::time::Instant::now();
              let f = self.loader.load(key)?;
              tracing::info!(
                  kernel = key.name,
                  dtype = ?key.dtype,
                  sm = key.sm,
                  variant = key.variant,
                  elapsed_ms = started.elapsed().as_millis() as u64,
                  "kernel loaded"
              );
              entries.insert(*key, f.clone());
              Ok(f)
          }

          pub fn len(&self) -> usize {
              self.entries.read().len()
          }

          pub fn is_empty(&self) -> bool {
              self.len() == 0
          }

          /// Drops every loaded kernel; the next `get` reloads.
          pub fn evict_all(&self) -> usize {
              let mut entries = self.entries.write();
              let n = entries.len();
              entries.clear();
              n
          }
      }

      impl<F: Clone + Send + Sync + 'static> Resource for KernelRegistry<F> {
          fn key(&self) -> ResourceKey {
              ResourceKey::new("kernel-registry")
          }
      }

      #[cfg(test)]
      mod tests {
          use super::*;
          use std::sync::atomic::{AtomicUsize, Ordering};

          struct Counting(AtomicUsize);

          impl KernelLoader<u32> for Counting {
              fn load(&self, key: &KernelKey) -> Result<u32, KernelError> {
                  if key.name == "missing" {
                      return Err(KernelError::FuncNotFound { name: key.name.into() });
                  }
                  self.0.fetch_add(1, Ordering::SeqCst);
                  Ok(key.variant)
              }
          }

          fn registry() -> (Arc<Counting>, KernelRegistry<u32>) {
              let loader = Arc::new(Counting(AtomicUsize::new(0)));
              (loader.clone(), KernelRegistry::new(loader))
          }

          #[test]
          fn loads_once_per_key() {
              let (loader, reg) = registry();
              let k = KernelKey::new("cosine", DType::F16, 75, 384);
              assert_eq!(reg.get(&k).unwrap(), 384);
              assert_eq!(reg.get(&k).unwrap(), 384);
              assert_eq!(loader.0.load(Ordering::SeqCst), 1);
              reg.get(&KernelKey::new("cosine", DType::F32, 75, 384)).unwrap();
              assert_eq!(reg.len(), 2);
          }

          #[test]
          fn failed_load_is_not_cached() {
              let (_, reg) = registry();
              let k = KernelKey::new("missing", DType::F32, 75, 0);
              assert!(matches!(reg.get(&k), Err(KernelError::FuncNotFound { .. })));
              assert!(reg.is_empty());
          }

          #[test]
          fn evict_forces_reload() {
              let (loader, reg) = registry();
              let k = KernelKey::new("cosine", DType::F16, 75, 8);
              reg.get(&k).unwrap();
              assert_eq!(reg.evict_all(), 1);
              reg.get(&k).unwrap();
              assert_eq!(loader.0.load(Ordering::SeqCst), 2);
          }

          #[test]
          fn concurrent_gets_load_once() {
              let (loader, reg) = registry();
              let reg = Arc::new(reg);
              let k = KernelKey::new("cosine", DType::F16, 75, 1);
              let handles: Vec<_> = (0..8)
                  .map(|_| {
                      let reg = reg.clone();
                      std::thread::spawn(move || reg.get(&k).unwrap())
                  })
                  .collect();
              handles.into_iter().for_each(|h| assert_eq!(h.join().unwrap(), 1));
              assert_eq!(loader.0.load(Ordering::SeqCst), 1);
          }
      }
  reason: Create MaxSimOp (CPU + Candle tile chain) and the generic lazy KernelRegistry with tests
```
/answer 6bcd-168 allow
Text:
 Now I'm wiring `KernelError` into the top-level error and checking the no-GPU build before moving to CUDA.
Control Request:
```yaml
id: 6bcd-169
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/error.rs
    id: ResidentError
    old_text: |2-
          Tokenize(#[from] crate::resident::tokenize::TokenizeError),
      }
    new_text: |2-
          Tokenize(#[from] crate::resident::tokenize::TokenizeError),
          #[error(transparent)]
          Kernel(#[from] crate::resident::kernel::KernelError),
      }
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/error.rs
    id: impl_IsRetryable_for_ResidentError.is_retryable
    old_text: "            ResidentError::Tokenize(e) => e.is_retryable(),"
    new_text: |2-
                  ResidentError::Tokenize(e) => e.is_retryable(),
                  ResidentError::Kernel(e) => e.is_retryable(),
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/error.rs
    id: tests.all_errors_are_send_sync_static
    old_text: '        assert_send_sync_static::<crate::resident::tokenize::TokenizeError>();'
    new_text: |2-
              assert_send_sync_static::<crate::resident::tokenize::TokenizeError>();
              assert_send_sync_static::<crate::resident::kernel::KernelError>();
  reason: Aggregate KernelError in ResidentError per the error convention
```
/answer 6bcd-169 allow
Control Request:
```yaml
id: 6bcd-16a
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench/rag
  script: cargo test --lib resident::kernel resident::error 2>&1 | tail -60
  reason: Compile and run the no-GPU kernel and error tests
```
/answer 6bcd-16a allow
Control Request:
```yaml
id: 6bcd-16b
result:
  structuredContent:
    stdout: |
      error: unexpected argument 'resident::error' found

      Usage: cargo test [OPTIONS] [TESTNAME] [-- [ARGS]...]

      For more information, try '--help'.
```
/answer 6bcd-16b allow
Control Request:
```yaml
id: 6bcd-16c
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench/rag
  script: "cargo test --lib resident:: 2>&1 | grep -v \"^test .* ok$\" | tail -60"
  reason: Compile and run all resident tests without GPU features
```
/answer 6bcd-16c allow
Control Request:
```yaml
id: 6bcd-16d
result:
  structuredContent:
    stdout: |2
          |        ----- the method is available for `TopKScorer<CosineSlots>` here
      ...
      188 | pub struct TopKScorer<S> {
          | ------------------------ method `score` not found for this struct
          |
          = help: items from traits can only be used if the trait is in scope
      help: trait `Scorer` which provides `score` is implemented but not in scope; perhaps you want to import it
          |
       47 +     use crate::resident::kernel::Scorer;
          |

      error[E0599]: no method named `score` found for struct `TopKScorer<S>` in the current scope
         --> src/resident/kernel/cosine.rs:120:20
          |
      120 |             scorer.score(&q, far, &mut lane()),
          |                    ^^^^^ method not found in `TopKScorer<CosineSlots>`
          |
         ::: src/resident/kernel/mod.rs:180:8
          |
      180 |     fn score(
          |        ----- the method is available for `TopKScorer<CosineSlots>` here
      ...
      188 | pub struct TopKScorer<S> {
          | ------------------------ method `score` not found for this struct
          |
          = help: items from traits can only be used if the trait is in scope
      help: trait `Scorer` which provides `score` is implemented but not in scope; perhaps you want to import it
          |
       47 +     use crate::resident::kernel::Scorer;
          |

      error[E0599]: no associated function or constant named `default` found for struct `region::ArenaId` in the current scope
         --> src/resident/kernel/maxsim.rs:202:48
          |
      202 |         let scratch = Region { arena: ArenaId::default(), offset: 0, len: 0, gen: 0 };
          |                                                ^^^^^^^ associated function or constant not found in `region::ArenaId`
          |
         ::: src/resident/memory/region.rs:4:1
          |
        4 | pub struct ArenaId(pub u32);
          | ------------------ associated function or constant `default` not found for this struct
          |
          = help: items from traits can only be used if the trait is implemented and in scope
          = note: the following traits define an item `default`, perhaps you need to implement one of them:
                  candidate #1: `Default`
                  candidate #2: `tinyvec::array::Array`

      warning: irrefutable `if let` pattern
         --> src/resident/subengine/exec.rs:337:20
          |
      337 |                 if let EventRepr::Cpu(e) = event.repr() {
          |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
          |
          = note: this pattern will always match, so the `if let` is useless
          = help: consider replacing the `if let` with a `let`
          = note: `#[warn(irrefutable_let_patterns)]` on by default

      For more information about this error, try `rustc --explain E0599`.
      warning: `xy-ai-rag` (lib test) generated 1 warning
      error: could not compile `xy-ai-rag` (lib test) due to 7 previous errors; 1 warning emitted
```
/answer 6bcd-16d allow
ReasoningToken: 58
Control Request:
```yaml
id: 6bcd-16e
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel/cosine.rs
    id: tests
    old_text: ArenaId::default()
    new_text: ArenaId(0)
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel/cosine.rs
    id: tests
    old_text: "    use crate::resident::kernel::{score_ranges, InMemorySlots};"
    new_text: "    use crate::resident::kernel::{score_ranges, InMemorySlots, Scorer};"
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel/maxsim.rs
    id: tests
    old_text: ArenaId::default()
    new_text: ArenaId(0)
  reason: "Fix test helper compile errors (ArenaId constructor, trait import)"
```
/answer 6bcd-16e allow
Control Request:
```yaml
id: 6bcd-16f
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench/rag
  script: "cargo test --lib resident:: 2>&1 | grep -E \"^(error|warning: unused)|-->|test result|FAILED|failed|panicked\" | head -40"
  reason: Re-run resident tests after fixes
```
/answer 6bcd-16f allow
Control Request:
```yaml
id: 6bcd-170
result:
  structuredContent:
    stdout: |2
         --> src/resident/subengine/exec.rs:337:20
      test resident::kernel::registry::tests::failed_load_is_not_cached ... ok
      test resident::processor::resource::tests::failed_load_backs_off_until_deadline ... ok
      test resident::subengine::lifecycle::tests::failed_init_leaves_cell_empty_and_backs_off ... ok
      test result: ok. 242 passed; 0 failed; 0 ignored; 0 measured; 28 filtered out; finished in 4.76s
```
/answer 6bcd-170 allow
Text:
 No-GPU build is green (242 passed). Now the Candle tile chain, run on the CPU device.
ReasoningToken: 25
Control Request:
```yaml
id: 6bcd-171
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench/rag
  script: cargo test --lib --features candle resident::kernel 2>&1 | grep -E "^(error|warning)|-->|test result|FAILED|failed|panicked|^\s+(left|right)|candle" | head -40
  reason: Verify Candle tile-chain MaxSim against the CPU reference
```
/answer 6bcd-171 allow
Control Request:
```yaml
id: 6bcd-172
result:
  structuredContent:
    stdout: |
      warning: use of deprecated constant `std::f32::MIN`: replaced by the `MIN` associated constant on `f32`
         --> third_party/candle-transformers/src/models/modernbert.rs:302:27
      warning: use of deprecated constant `std::f32::NEG_INFINITY`: replaced by the `NEG_INFINITY` associated constant on `f32`
         --> third_party/candle-transformers/src/models/modernbert.rs:315:26
      warning: `candle-transformers` (lib) generated 2 warnings
      warning: irrefutable `if let` pattern
         --> src/resident/subengine/exec.rs:337:20
      warning: function `elems_to_bytes` is never used
        --> src/resident/memory/dtype.rs:89:19
      warning: function `bytes_to_elems` is never used
        --> src/resident/memory/dtype.rs:93:19
      warning: `xy-ai-rag` (lib test) generated 3 warnings
      test resident::kernel::registry::tests::failed_load_is_not_cached ... ok
      test resident::kernel::maxsim::tests::candle::f16_masking_never_produces_nan ... ok
      test resident::kernel::maxsim::tests::candle::op_wraps_candle_chain ... ok
      test resident::kernel::maxsim::tests::candle::f32_tile_chain_matches_cpu_reference_incl_padding ... ok
      test result: ok. 33 passed; 0 failed; 0 ignored; 0 measured; 245 filtered out; finished in 0.10s
```
/answer 6bcd-172 allow
ReasoningToken: 1486
Control Request:
```yaml
id: 6bcd-173
toolName: bash
arguments:
  cwd: /home/user/.cargo/registry/src
  script: "d=$(ls -d */cudarc-0.19.9); cd $d; grep -n \"pub fn compute_capability\\|pub fn attribute\" src/driver/safe/core.rs; grep -n \"trait PushKernelArg\" -A8 src/driver/safe/launch.rs; grep -n \"pub fn new_cuda\\|pub fn synchronize\" ../candle-core-0.11.0/src/device.rs | head; sed -n 1,40p /home/user/xyan/xy.ai.workbench/rag/src/resident/subengine/device.rs | head -60"
  reason: "Check compute_capability, PushKernelArg and candle device sync APIs; see device.rs imports"
```
/answer 6bcd-173 allow
Control Request:
```yaml
id: 6bcd-174
result:
  structuredContent:
    stdout: |
      302:    pub fn compute_capability(&self) -> Result<(i32, i32), result::DriverError> {
      362:    pub fn attribute(&self, attrib: sys::CUdevice_attribute) -> Result<i32, result::DriverError> {
      88:pub unsafe trait PushKernelArg<T> {
      89-    fn arg(&mut self, arg: T) -> &mut Self;
      90-}
      91-
      92-unsafe impl<'a, 'b: 'a, T: DeviceRepr> PushKernelArg<&'b T> for LaunchArgs<'a> {
      93-    #[inline(always)]
      94-    fn arg(&mut self, arg: &'b T) -> &mut Self {
      95-        self.args.push(arg as *const T as *mut _);
      96-        self
      234:    pub fn new_cuda(ordinal: usize) -> Result<Self> {
      254:    pub fn new_cuda_with_stream(ordinal: usize) -> Result<Self> {
      511:    pub fn synchronize(&self) -> Result<()> {
      //! Compute backends, device handle and CUDA execution lanes.
      use crate::resident::backend::ComputeBackend;
      /// Real CUDA backend; the single Candle/CUDA adapter point for the subengine. Construction fails if no compatible GPU is available,
      /// which lets `create_backend` fall back to `CpuBackend`.
      pub struct CudaBackend;
      impl CudaBackend {
          pub fn try_new() -> Result<Self, crate::resident::error::IndexError> {
              Err(crate::resident::error::IndexError::TransferFailed {
                  reason: "CUDA backend not yet implemented".to_string(),
              })
          }
      }
      impl ComputeBackend for CudaBackend {
          fn run_batch(&self, _input: &[f32]) -> Vec<f32> {
              unimplemented!("real CUDA kernel dispatch is not implemented yet")
          }
      }
      use std::time::{Duration, Instant};
      use crate::resident::memory::DeviceKind;
      use crate::resident::subengine::{
          CpuLane, DeviceError, ExecContext, LaneClass, LaneConfig,
      };
      /// Identifies the compute device a subengine runs on; lanes are opened from it.
      #[derive(Clone)]
      pub struct DeviceHandle {
          kind: DeviceKind,
          ordinal: usize,
          init_cost: Duration,
          #[cfg(feature = "cuda")]
          ctx: Option<std::sync::Arc<cudarc::driver::CudaContext>>,
      }
      impl std::fmt::Debug for DeviceHandle {
          fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
              f.debug_struct("DeviceHandle")
                  .field("kind", &self.kind)
                  .field("ordinal", &self.ordinal)
                  .finish()
          }
      }
      impl DeviceHandle {
```
/answer 6bcd-174 allow
ReasoningToken: 727
Control Request:
```yaml
id: 6bcd-175
toolName: ast_create
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel/cuda.rs
    overwrite: true
    source: |
      //! Real CUDA kernels / scorer: NVRTC-compiled sources behind the kernel registry.
      #![cfg(feature = "cuda")]

      use std::collections::HashMap;
      use std::ffi::{c_char, CString};
      use std::sync::Arc;

      use cudarc::driver::{CudaContext, CudaFunction, CudaSlice, CudaStream, DeviceRepr, LaunchConfig, PushKernelArg};
      use cudarc::nvrtc::{compile_ptx_with_opts, result as nvrtc_result, sys as nvrtc_sys, CompileOptions, Ptx};

      use super::registry::{KernelKey, KernelLoader, KernelRegistry};
      use super::KernelError;
      use crate::resident::memory::DType;

      pub const COSINE_DOT: &str = "cosine_dot";
      pub const MAXSIM_FUSED: &str = "maxsim_fused";

      const PRELUDE: &str = r#"
      #if defined(DTYPE_F16)
      typedef unsigned short elem_t;
      __device__ __forceinline__ float ld(elem_t v) {
          float f;
          asm("cvt.f32.f16 %0, %1;" : "=f"(f) : "h"(v));
          return f;
      }
      #else
      typedef float elem_t;
      __device__ __forceinline__ float ld(elem_t v) { return v; }
      #endif

      __device__ __forceinline__ float warp_sum(float v) {
          for (int o = 16; o > 0; o >>= 1) v += __shfl_down_sync(0xffffffffu, v, o);
          return v;
      }
      __device__ __forceinline__ float warp_max(float v) {
          for (int o = 16; o > 0; o >>= 1) v = fmaxf(v, __shfl_down_sync(0xffffffffu, v, o));
          return v;
      }
      "#;

      /// One block per slot; scores every query against the slot's single vector. Output is
      /// `n_queries x n_slots`.
      const COSINE_SRC: &str = r#"
      extern "C" __global__ void cosine_dot(
          const elem_t* __restrict__ docs, const float* __restrict__ queries, float* __restrict__ out,
          unsigned n_slots, unsigned n_queries)
      {
          unsigned slot = blockIdx.x;
          if (slot >= n_slots) return;
          const elem_t* d = docs + (size_t)slot * DIM;
          __shared__ float part[32];
          unsigned lane = threadIdx.x & 31, warp = threadIdx.x >> 5, warps = (blockDim.x + 31) >> 5;
          for (unsigned q = 0; q < n_queries; ++q) {
              const float* qv = queries + (size_t)q * DIM;
              float acc = 0.f;
              for (unsigned i = threadIdx.x; i < DIM; i += blockDim.x) acc += ld(d[i]) * qv[i];
              acc = warp_sum(acc);
              if (lane == 0) part[warp] = acc;
              __syncthreads();
              if (threadIdx.x == 0) {
                  float s = 0.f;
                  for (unsigned w = 0; w < warps; ++w) s += part[w];
                  out[(size_t)q * n_slots + slot] = s;
              }
              __syncthreads();
          }
      }
      "#;

      /// One block per document, one query. Documents are padded to `doc_tokens`; `doc_len` holds the
      /// valid token count. A token without any valid document token scores the f16-safe mask value.
      const MAXSIM_SRC: &str = r#"
      extern "C" __global__ void maxsim_fused(
          const elem_t* __restrict__ docs, const unsigned* __restrict__ doc_len,
          const float* __restrict__ query, float* __restrict__ out,
          unsigned n_slots, unsigned doc_tokens, unsigned q_tokens)
      {
          unsigned slot = blockIdx.x;
          if (slot >= n_slots) return;
          const elem_t* d = docs + (size_t)slot * doc_tokens * DIM;
          unsigned valid = min(doc_len[slot], doc_tokens);
          __shared__ float part[32];
          unsigned lane = threadIdx.x & 31, warp = threadIdx.x >> 5, warps = (blockDim.x + 31) >> 5;
          float total = 0.f;
          for (unsigned t = 0; t < q_tokens; ++t) {
              const float* q = query + (size_t)t * DIM;
              float best = -1e4f;
              for (unsigned j = threadIdx.x; j < valid; j += blockDim.x) {
                  const elem_t* row = d + (size_t)j * DIM;
                  float acc = 0.f;
                  for (unsigned i = 0; i < DIM; ++i) acc += ld(row[i]) * q[i];
                  best = fmaxf(best, acc);
              }
              best = warp_max(best);
              if (lane == 0) part[warp] = best;
              __syncthreads();
              if (threadIdx.x == 0) {
                  float m = -1e4f;
                  for (unsigned w = 0; w < warps; ++w) m = fmaxf(m, part[w]);
                  total += m;
              }
              __syncthreads();
          }
          if (threadIdx.x == 0) out[slot] = total;
      }
      "#;

      fn body(name: &str) -> Option<&'static str> {
          match name {
              COSINE_DOT => Some(COSINE_SRC),
              MAXSIM_FUSED => Some(MAXSIM_SRC),
              _ => None,
          }
      }

      fn defines(key: &KernelKey) -> Result<Vec<String>, KernelError> {
          let ty = match key.dtype {
              DType::F16 => "-DDTYPE_F16",
              DType::F32 => "-DDTYPE_F32",
              other => {
                  return Err(KernelError::Unavailable { reason: format!("no kernel for dtype {other:?}") })
              }
          };
          Ok(vec![ty.to_string(), format!("-DDIM={}", key.variant)])
      }

      fn log_of(prog: cudarc::nvrtc::sys::nvrtcProgram) -> String {
          unsafe { nvrtc_result::get_program_log(prog) }
              .map(|raw| {
                  let bytes: Vec<u8> = raw.iter().take_while(|&&c| c != 0).map(|&c| c as u8).collect();
                  String::from_utf8_lossy(&bytes).into_owned()
              })
              .unwrap_or_default()
      }

      /// Machine code for exactly `sm_<sm>`; usable when the driver cannot JIT the PTX version.
      fn compile_cubin(src: &str, sm: u32, opts: &[String]) -> Result<Vec<u8>, KernelError> {
          let c_src = CString::new(src).map_err(|e| KernelError::Compile { log: e.to_string() })?;
          let prog = nvrtc_result::create_program(&c_src, None)
              .map_err(|e| KernelError::Compile { log: e.to_string() })?;
          let mut options = vec![format!("--gpu-architecture=sm_{sm}")];
          options.extend_from_slice(opts);
          let result = (|| {
              unsafe { nvrtc_result::compile_program(prog, &options) }
                  .map_err(|_| KernelError::Compile { log: log_of(prog) })?;
              let mut size = 0usize;
              unsafe { nvrtc_sys::nvrtcGetCUBINSize(prog, &mut size) }
                  .result()
                  .map_err(|e| KernelError::Compile { log: e.to_string() })?;
              let mut cubin = vec![0u8; size];
              unsafe { nvrtc_sys::nvrtcGetCUBIN(prog, cubin.as_mut_ptr() as *mut c_char) }
                  .result()
                  .map_err(|e| KernelError::Compile { log: e.to_string() })?;
              Ok(cubin)
          })();
          let _ = unsafe { nvrtc_result::destroy_program(prog) };
          result
      }

      /// Compiles to PTX for `compute_<sm>` and falls back to a CUBIN when the driver rejects the PTX.
      pub struct NvrtcLoader {
          ctx: Arc<CudaContext>,
      }

      impl NvrtcLoader {
          pub fn new(ctx: Arc<CudaContext>) -> Self {
              Self { ctx }
          }

          fn module(&self, key: &KernelKey, src: &str) -> Result<Arc<cudarc::driver::CudaModule>, KernelError> {
              let opts = defines(key)?;
              let ptx = compile_ptx_with_opts(
                  src,
                  CompileOptions {
                      arch: Some(arch_for(key.sm)),
                      options: opts.clone(),
                      ..Default::default()
                  },
              )
              .map_err(|e| KernelError::Compile { log: e.to_string() })?;
              match self.ctx.load_module(ptx) {
                  Ok(m) => Ok(m),
                  Err(ptx_err) => {
                      tracing::warn!(kernel = key.name, error = %ptx_err, "PTX rejected, retrying as CUBIN");
                      let cubin = compile_cubin(src, key.sm, &opts)?;
                      self.ctx
                          .load_module(Ptx::from_binary(cubin))
                          .map_err(|e| KernelError::ModuleLoad { reason: e.to_string() })
                  }
              }
          }
      }

      fn arch_for(sm: u32) -> &'static str {
          match sm {
              61 => "compute_61",
              70 => "compute_70",
              75 => "compute_75",
              80 => "compute_80",
              86 => "compute_86",
              89 => "compute_89",
              _ => "compute_75",
          }
      }

      impl KernelLoader<CudaFunction> for NvrtcLoader {
          fn load(&self, key: &KernelKey) -> Result<CudaFunction, KernelError> {
              let body = body(key.name).ok_or_else(|| KernelError::FuncNotFound { name: key.name.into() })?;
              let src = format!("{PRELUDE}{body}");
              let module = self.module(key, &src)?;
              module
                  .load_function(key.name)
                  .map_err(|_| KernelError::FuncNotFound { name: key.name.into() })
          }
      }

      /// Registry plus the device's compute capability.
      pub struct CudaKernels {
          registry: KernelRegistry<CudaFunction>,
          sm: u32,
      }

      impl CudaKernels {
          pub fn new(ctx: &Arc<CudaContext>) -> Result<Self, KernelError> {
              let (major, minor) = ctx
                  .compute_capability()
                  .map_err(|e| KernelError::Unavailable { reason: e.to_string() })?;
              let loader: Arc<dyn KernelLoader<CudaFunction>> = Arc::new(NvrtcLoader::new(ctx.clone()));
              Ok(Self { registry: KernelRegistry::new(loader), sm: (major * 10 + minor) as u32 })
          }

          pub fn sm(&self) -> u32 {
              self.sm
          }

          pub fn registry(&self) -> &KernelRegistry<CudaFunction> {
              &self.registry
          }

          pub fn function(&self, name: &'static str, dtype: DType, dim: u32) -> Result<CudaFunction, KernelError> {
              self.registry.get(&KernelKey::new(name, dtype, self.sm, dim))
          }
      }

      fn launch_err(e: impl std::error::Error + Send + Sync + 'static) -> KernelError {
          KernelError::LaunchFailed { source: Box::new(e) }
      }

      const BLOCK: u32 = 128;

      fn cfg(blocks: u32) -> LaunchConfig {
          LaunchConfig { grid_dim: (blocks.max(1), 1, 1), block_dim: (BLOCK, 1, 1), shared_mem_bytes: 0 }
      }

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

      pub(crate) fn launch_maxsim<T: DeviceRepr>(
          stream: &Arc<CudaStream>,
          f: &CudaFunction,
          docs: &CudaSlice<T>,
          doc_len: &CudaSlice<u32>,
          query: &CudaSlice<f32>,
          out: &mut CudaSlice<f32>,
          n_slots: u32,
          doc_tokens: u32,
          q_tokens: u32,
      ) -> Result<(), KernelError> {
          let mut b = stream.launch_builder(f);
          b.arg(docs).arg(doc_len).arg(query).arg(out).arg(&n_slots).arg(&doc_tokens).arg(&q_tokens);
          unsafe { b.launch(cfg(n_slots)) }.map(|_| ()).map_err(launch_err)
      }

      #[allow(dead_code)]
      type Unused = HashMap<(), ()>;

      #[cfg(test)]
      mod tests {
          use super::*;
          use crate::resident::kernel::cpu_simd::{dot, maxsim};
          use crate::resident::subengine::DeviceHandle;
          use half::f16;

          const DIM: usize = 64;

          fn rnd(n: usize, seed: u32) -> Vec<f32> {
              (0..n)
                  .map(|i| ((i as u32).wrapping_mul(2654435761).wrapping_add(seed.wrapping_mul(97)) % 1000) as f32 / 500.0 - 1.0)
                  .collect()
          }

          fn setup() -> (Arc<CudaContext>, Arc<CudaStream>, CudaKernels) {
              let dev = DeviceHandle::cuda(0).expect("cuda device");
              let ctx = dev.cuda_context().unwrap().clone();
              let stream = ctx.new_stream().unwrap();
              let kernels = CudaKernels::new(&ctx).unwrap();
              (ctx, stream, kernels)
          }

          #[test]
          #[ignore = "requires a CUDA device"]
          fn registry_compiles_for_device_sm_and_caches() {
              let (_, _, k) = setup();
              assert!(k.sm() >= 75);
              k.function(COSINE_DOT, DType::F16, DIM as u32).unwrap();
              k.function(COSINE_DOT, DType::F16, DIM as u32).unwrap();
              k.function(MAXSIM_FUSED, DType::F32, DIM as u32).unwrap();
              assert_eq!(k.registry().len(), 2);
              assert!(matches!(
                  k.function("nope", DType::F32, 1),
                  Err(KernelError::FuncNotFound { .. })
              ));
          }

          #[test]
          #[ignore = "requires a CUDA device"]
          fn cubin_path_loads_and_runs() {
              let (ctx, stream, k) = setup();
              let key = KernelKey::new(COSINE_DOT, DType::F32, k.sm(), DIM as u32);
              let src = format!("{PRELUDE}{COSINE_SRC}");
              let cubin = compile_cubin(&src, k.sm(), &defines(&key).unwrap()).unwrap();
              let f = ctx
                  .load_module(Ptx::from_binary(cubin))
                  .unwrap()
                  .load_function(COSINE_DOT)
                  .unwrap();
              let docs = rnd(3 * DIM, 1);
              let q = rnd(DIM, 2);
              let d_docs = stream.clone_htod(&docs).unwrap();
              let d_q = stream.clone_htod(&q).unwrap();
              let mut d_out = stream.alloc_zeros::<f32>(3).unwrap();
              launch_cosine(&stream, &f, &d_docs, &d_q, &mut d_out, 3, 1).unwrap();
              let got = stream.clone_dtoh(&d_out).unwrap();
              for s in 0..3 {
                  let want = dot(&q, &docs[s * DIM..(s + 1) * DIM]);
                  assert!((got[s] - want).abs() < 1e-3);
              }
          }

          #[test]
          #[ignore = "requires a CUDA device"]
          fn cosine_kernel_matches_cpu_for_f32_and_f16() {
              let (_, stream, k) = setup();
              let (n, nq) = (37usize, 3usize);
              let docs = rnd(n * DIM, 3);
              let queries = rnd(nq * DIM, 4);
              let d_q = stream.clone_htod(&queries).unwrap();

              let f32_fn = k.function(COSINE_DOT, DType::F32, DIM as u32).unwrap();
              let d_docs = stream.clone_htod(&docs).unwrap();
              let mut out = stream.alloc_zeros::<f32>(n * nq).unwrap();
              launch_cosine(&stream, &f32_fn, &d_docs, &d_q, &mut out, n as u32, nq as u32).unwrap();
              let got = stream.clone_dtoh(&out).unwrap();
              for q in 0..nq {
                  for s in 0..n {
                      let want = dot(&queries[q * DIM..(q + 1) * DIM], &docs[s * DIM..(s + 1) * DIM]);
                      assert!((got[q * n + s] - want).abs() < 1e-3, "f32 q={q} s={s}");
                  }
              }

              let f16_fn = k.function(COSINE_DOT, DType::F16, DIM as u32).unwrap();
              let bits: Vec<u16> = docs.iter().map(|&v| f16::from_f32(v).to_bits()).collect();
              let rounded: Vec<f32> = bits.iter().map(|&b| f16::from_bits(b).to_f32()).collect();
              let d_docs16 = stream.clone_htod(&bits).unwrap();
              let mut out16 = stream.alloc_zeros::<f32>(n * nq).unwrap();
              launch_cosine(&stream, &f16_fn, &d_docs16, &d_q, &mut out16, n as u32, nq as u32).unwrap();
              let got16 = stream.clone_dtoh(&out16).unwrap();
              for q in 0..nq {
                  for s in 0..n {
                      let want = dot(&queries[q * DIM..(q + 1) * DIM], &rounded[s * DIM..(s + 1) * DIM]);
                      assert!((got16[q * n + s] - want).abs() < 1e-3, "f16 q={q} s={s}");
                  }
              }
          }

          #[test]
          #[ignore = "requires a CUDA device"]
          fn fused_maxsim_matches_cpu_with_padding() {
              let (_, stream, k) = setup();
              let (n, dt, qt) = (9usize, 16usize, 5usize);
              let lens: Vec<u32> = vec![16, 1, 7, 0, 16, 3, 9, 12, 2];
              let docs = rnd(n * dt * DIM, 5);
              let query = rnd(qt * DIM, 6);
              let f = k.function(MAXSIM_FUSED, DType::F32, DIM as u32).unwrap();
              let d_docs = stream.clone_htod(&docs).unwrap();
              let d_len = stream.clone_htod(&lens).unwrap();
              let d_q = stream.clone_htod(&query).unwrap();
              let mut out = stream.alloc_zeros::<f32>(n).unwrap();
              launch_maxsim(&stream, &f, &d_docs, &d_len, &d_q, &mut out, n as u32, dt as u32, qt as u32)
                  .unwrap();
              let got = stream.clone_dtoh(&out).unwrap();
              for s in 0..n {
                  let valid = &docs[s * dt * DIM..(s * dt + lens[s] as usize) * DIM];
                  let want = maxsim(&query, valid, DIM);
                  assert!((got[s] - want).abs() < 1e-3, "slot {s}: got {} want {want}", got[s]);
              }
          }
      }
  reason: "Implement NVRTC loader with CUBIN fallback, cosine and fused MaxSim kernels, launch helpers and GPU tests"
```
/answer 6bcd-175 allow
Control Request:
```yaml
id: 6bcd-176
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel/cuda.rs
    id: ModuleDoc
    old_text: |
      use std::collections::HashMap;
    new_text: ""
  reason: Remove unused import left in cuda.rs
```
/answer 6bcd-176 allow
ReasoningToken: 33
Control Request:
```yaml
id: 6bcd-177
result:
  structuredContent:
    errors:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel/cuda.rs
      error: Text not found in node.
      id: ModuleDoc
      reason: not_found
      next_step: reread_node
```
/answer 6bcd-177 allow
Control Request:
```yaml
id: 6bcd-178
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench/rag
  script: |-
    python3 - <<'EOF'
    p='src/resident/kernel/cuda.rs'
    s=open(p).read()
    s=s.replace("use std::collections::HashMap;\n","").replace("#[allow(dead_code)]\ntype Unused = HashMap<(), ()>;\n\n","")
    open(p,'w').write(s)
    EOF
    cargo test --lib --features cuda resident::kernel --no-run 2>&1 | grep -E "^error|^warning: unused|-->|^\s+\|.*\^" | head -40
  reason: Drop dead HashMap alias and compile the cuda feature build
```
/answer 6bcd-178 allow
Control Request:
```yaml
id: 6bcd-179
result:
  structuredContent:
    stdout: |2
         --> third_party/candle-transformers/src/models/modernbert.rs:302:27
          |                           ^^^
         --> third_party/candle-transformers/src/models/modernbert.rs:315:26
          |                          ^^^^^^^^^^^^
        --> src/resident/memory/dtype.rs:89:19
         |                   ^^^^^^^^^^^^^^
        --> src/resident/memory/dtype.rs:93:19
         |                   ^^^^^^^^^^^^^^
         --> src/resident/subengine/device.rs:174:23
          |                       ^^^^^^
```
/answer 6bcd-179 allow
Text:
 Compiles with `cuda`. Running the GPU tests on the real GTX 1660 SUPER now.
Control Request:
```yaml
id: 6bcd-17a
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench/rag
  script: cargo test --lib --features cuda resident::kernel -- --include-ignored 2>&1 | grep -E "^test |test result|panicked|assert|left|right|error" | grep -v " ok$" | head -30; cargo test --lib --features cuda resident::kernel -- --include-ignored 2>&1 | grep -E "kernel::cuda"
  reason: Run NVRTC/CUBIN/cosine/fused-MaxSim kernel tests on the GPU
```
/answer 6bcd-17a allow
Control Request:
```yaml
id: 6bcd-17b
result:
  structuredContent:
    stdout: |
      test result: ok. 37 passed; 0 failed; 0 ignored; 0 measured; 257 filtered out; finished in 0.46s
      test resident::kernel::cuda::tests::fused_maxsim_matches_cpu_with_padding ... ok
      test resident::kernel::cuda::tests::cubin_path_loads_and_runs ... ok
      test resident::kernel::cuda::tests::registry_compiles_for_device_sm_and_caches ... ok
      test resident::kernel::cuda::tests::cosine_kernel_matches_cpu_for_f32_and_f16 ... ok
```
/answer 6bcd-17b allow
ReasoningToken: 429
Control Request:
```yaml
id: 6bcd-17c
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel/cuda.rs
  node_type: ItemFn
  name: fused_maxsim_matches_cpu_with_padding
  reason: Get node id to insert the G-MAXSIM measurement test after it
```
/answer 6bcd-17c allow
Control Request:
```yaml
id: 6bcd-17d
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel/cuda.rs
      nodes:
      - id: tests.fused_maxsim_matches_cpu_with_padding
        type: ItemFn
        code: |2
              #[test]
              #[ignore = "requires a CUDA device"]
              fn fused_maxsim_matches_cpu_with_padding() {
                  let (_, stream, k) = setup();
                  let (n, dt, qt) = (9usize, 16usize, 5usize);
                  let lens: Vec<u32> = vec![16, 1, 7, 0, 16, 3, 9, 12, 2];
                  let docs = rnd(n * dt * DIM, 5);
                  let query = rnd(qt * DIM, 6);
                  let f = k.function(MAXSIM_FUSED, DType::F32, DIM as u32).unwrap();
                  let d_docs = stream.clone_htod(&docs).unwrap();
                  let d_len = stream.clone_htod(&lens).unwrap();
                  let d_q = stream.clone_htod(&query).unwrap();
                  let mut out = stream.alloc_zeros::<f32>(n).unwrap();
                  launch_maxsim(&stream, &f, &d_docs, &d_len, &d_q, &mut out, n as u32, dt as u32, qt as u32)
                      .unwrap();
                  let got = stream.clone_dtoh(&out).unwrap();
                  for s in 0..n {
                      let valid = &docs[s * dt * DIM..(s * dt + lens[s] as usize) * DIM];
                      let want = maxsim(&query, valid, DIM);
                      assert!((got[s] - want).abs() < 1e-3, "slot {s}: got {} want {want}", got[s]);
                  }
              }
```
/answer 6bcd-17d allow
ReasoningToken: 34
Control Request:
```yaml
id: 6bcd-17e
toolName: ast_insert
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel/cuda.rs
    id: tests.fused_maxsim_matches_cpu_with_padding
    position: after
    source: |2
          /// Measurement gate: batched Candle chain (`matmul -> mask -> max -> sum` over a whole slice)
          /// against the fused kernel on a ColBERT-sized workload. Run with `--nocapture`.
          #[test]
          #[ignore = "requires a CUDA device; measurement"]
          fn gate_maxsim_chain_vs_fused() {
              use candle_core::{DType as CD, Device, Tensor};
              use std::time::Instant;

              const D: usize = 128;
              let (n, dt, qt, iters) = (4096usize, 128usize, 32usize, 20u32);
              let (_, stream, k) = setup();
              let dev = Device::new_cuda(0).unwrap();

              let lens: Vec<u32> = (0..n).map(|i| 24 + (i * 7 % 105) as u32).collect();
              let docs = rnd(n * dt * D, 11);
              let query = rnd(qt * D, 12);

              let bits: Vec<u16> = docs.iter().map(|&v| f16::from_f32(v).to_bits()).collect();
              let d_docs = stream.clone_htod(&bits).unwrap();
              let d_len = stream.clone_htod(&lens).unwrap();
              let d_q = stream.clone_htod(&query).unwrap();
              let mut out = stream.alloc_zeros::<f32>(n).unwrap();
              let fused = k.function(MAXSIM_FUSED, DType::F16, D as u32).unwrap();

              let docs_t = Tensor::from_vec(docs.clone(), (n, dt, D), &dev)
                  .unwrap()
                  .to_dtype(CD::F16)
                  .unwrap()
                  .transpose(1, 2)
                  .unwrap()
                  .contiguous()
                  .unwrap();
              let q = Tensor::from_vec(query.clone(), (qt, D), &dev).unwrap().to_dtype(CD::F16).unwrap();
              let mask: Vec<f32> = lens
                  .iter()
                  .flat_map(|&l| (0..dt).map(move |j| if (j as u32) < l { 0.0 } else { crate::resident::memory::F16_MASK_VALUE }))
                  .collect();
              let mask = Tensor::from_vec(mask, (n, 1, dt), &dev).unwrap().to_dtype(CD::F16).unwrap();

              let chain = || {
                  q.broadcast_matmul(&docs_t)
                      .unwrap()
                      .broadcast_add(&mask)
                      .unwrap()
                      .max(2)
                      .unwrap()
                      .to_dtype(CD::F32)
                      .unwrap()
                      .sum(1)
                      .unwrap()
              };
              let run_fused = |out: &mut CudaSlice<f32>| {
                  launch_maxsim(&stream, &fused, &d_docs, &d_len, &d_q, out, n as u32, dt as u32, qt as u32)
                      .unwrap()
              };

              let chain_scores = chain().to_vec1::<f32>().unwrap();
              run_fused(&mut out);
              let fused_scores = stream.clone_dtoh(&out).unwrap();
              for s in (0..n).step_by(97) {
                  let tol = 0.5 + 0.02 * fused_scores[s].abs();
                  assert!((chain_scores[s] - fused_scores[s]).abs() < tol, "slot {s}");
              }

              let t = Instant::now();
              for _ in 0..iters {
                  let _ = chain();
              }
              dev.synchronize().unwrap();
              let chain_ms = t.elapsed().as_secs_f64() * 1e3 / f64::from(iters);

              let t = Instant::now();
              for _ in 0..iters {
                  run_fused(&mut out);
              }
              stream.synchronize().unwrap();
              let fused_ms = t.elapsed().as_secs_f64() * 1e3 / f64::from(iters);

              let sim_mb = (n * qt * dt * 2) as f64 / 1e6;
              println!(
                  "G-MAXSIM docs={n} doc_tokens={dt} q_tokens={qt} dim={D}: chain {chain_ms:.3} ms, fused {fused_ms:.3} ms, sim-matrix traffic {sim_mb:.1} MB per pass"
              );
          }
  reason: Add the G-MAXSIM measurement test comparing batched Candle chain with the fused kernel
```
/answer 6bcd-17e allow
Control Request:
```yaml
id: 6bcd-17f
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench/rag
  script: cargo test --lib --features cuda gate_maxsim -- --include-ignored --nocapture 2>&1 | grep -E "^error|-->|G-MAXSIM|panicked|test result|assert" | head -20
  reason: Run the G-MAXSIM measurement on the GTX 1660 SUPER
```
/answer 6bcd-17f allow
Control Request:
```yaml
id: 6bcd-180
result:
  structuredContent:
    stdout: |2
         --> third_party/candle-transformers/src/models/modernbert.rs:302:27
         --> third_party/candle-transformers/src/models/modernbert.rs:315:26
        --> src/resident/memory/dtype.rs:89:19
        --> src/resident/memory/dtype.rs:93:19
         --> src/resident/subengine/device.rs:174:23
      G-MAXSIM docs=4096 doc_tokens=128 q_tokens=32 dim=128: chain 21.582 ms, fused 67.148 ms, sim-matrix traffic 33.6 MB per pass
      test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 294 filtered out; finished in 8.15s
```
/answer 6bcd-180 allow
ReasoningToken: 337
Control Request:
```yaml
id: 6bcd-181
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench/rag
  script: |-
    cat >> ../project/resident/plan/plan_d_06_kernels_scorer.md <<'EOF'

    ---

    ## Implementation status

    Code lives in `rag/src/resident/kernel/`: `registry.rs`, `cosine.rs`, `maxsim.rs`, `cpu_simd.rs`, `slice.rs`, `topk.rs`, `slots.rs`, `error.rs`, `cuda.rs` (NVRTC/CUBIN, `cuda` feature). `Scorer` returns slice-local top-k per query (`Vec<Vec<KernelHit>>`) so query batching keeps per-query results.

    ## G-MAXSIM result (GTX 1660 SUPER, sm75, f16)

    Workload: 4096 documents x 128 tokens, 32 query tokens, dim 128, whole slice per pass.

    | Implementation | Time per pass |
    |---|---|
    | Candle chain (`broadcast_matmul -> mask -> max -> sum`, cuBLAS) | 21.6 ms |
    | Fused NVRTC kernel (scalar FMA, one block per document) | 67.1 ms |

    Decision: keep the Candle chain as the MaxSim implementation. The fused kernel is correct (matches CPU reference) but 3x slower because it does not use tensor cores. Revisit only with a tensor-core (WMMA/mma) fused kernel that beats the chain; it plugs in behind `MaxSimOp::with_slots`.

    ## Open points

    - Kernel launches take plain kernel arguments; mirroring dynamic values through the lane `ParamBlock` and resolving arena `Region`s to device pointers are part of the segment integration (D7).
    - The Candle chain scores from host `SlotSource` data; device-resident documents arrive with D7.
    EOF
    touch src/resident/kernel/mod.rs; cargo build --lib 2>&1 | grep -B2 -A6 "resident/kernel" | head -30; cargo test --lib 2>&1 | grep "test result"
  reason: Document G-MAXSIM decision in the D6 plan and run the full no-GPU suite plus a warning check for the new files
```
/answer 6bcd-181 allow
Control Request:
```yaml
id: 6bcd-182
result:
  structuredContent:
    stdout: |
      test result: ok. 270 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 3.67s
```
/answer 6bcd-182 allow
Result Stats: id=7b45c58f-e506-4bd7-a286-5239122faa81, total: 194176, in: 68, out: 63605, reason: 0, read: 2602776, write: 130503, start: 1792359848085, end: 1792359855234, time: 7106, total: 484853, io: 0, control: 43, sent: 0, recv: 180
D6 is implemented under `rag/src/resident/kernel/`, not `rag/src/kernel/` as the plan says, because that is where the existing B5 kernel module lives.

I ran the full no-GPU suite (270 tests, all passing) and the kernel tests with `candle` and with `cuda`. The `cuda` run used the GTX 1660 SUPER and included the ignored GPU tests, which passed. I did not re-run the full suite with `cuda` after adding the G-MAXSIM test. `Cargo.toml` is unchanged.

- **Scorers:**
  - **Cosine (e5):** the standard dense path, a dot product on normalised vectors.
  - **CPU MaxSim:** SIMD-friendly, and an empty document scores −1e4 per query token.
  - **Candle MaxSim:** the tile chain with −1e4 masking on padded columns, behind the `candle` feature.
  - **Behind one op:** the Candle chain and the CPU path both sit behind `MaxSimOp`.
- **Top-k and slicing:**
  - **Top-k merge:** ties go to the lower offset and NaN scores are dropped.
  - **Slice scheduler:** it sizes slices from throughput and checks for preemption and the deadline only between slices, so at least one slice always completes.
- **`KernelRegistry`:** a lazy table keyed by name, dtype, sm and a variant (the embedding dimension), which is one more key field than the plan lists. It implements `Resource`, and loading is serialised per key.
- **NVRTC and CUBIN:** the CUDA loader compiles PTX for `compute_<sm>` and falls back to a CUBIN. The CUBIN path loads and runs on the GPU.
- **CUDA kernels:** cosine and a fused MaxSim.
- **Errors:** `KernelError` is added to `ResidentError`.
- **`Scorer` return type:** `Scorer::score` returns top-k hits per query rather than a single `Vec<KernelHit>`, so query batching keeps each query's results separate.

**G-MAXSIM:** on 4096 documents × 128 tokens, 32 query tokens, dim 128, f16, the Candle chain took 21.6 ms. My fused kernel took 67.1 ms. The fused kernel is correct but about 3× slower, since it doesn't use tensor cores. I kept the Candle chain and wrote the result and decision into `plan_d_06_kernels_scorer.md`.

**Not done:** kernel launches pass plain arguments rather than going through the lane's `ParamBlock`. Resolving arena regions to device pointers, and scoring from device-resident documents, are left for D7. The Candle chain currently scores from host data.
