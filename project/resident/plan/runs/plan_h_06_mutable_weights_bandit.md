Setze H6 um.

- keine direkte Referenzierung von Planelementen in Kommentaren
- nvcc liegt unter `/usr/local/cuda-13.3/bin/nvcc`
- Nutze Bash nicht für Retrieval Tasks

[include](/home/user/xyan/xy.ai.workbench/project/resident/index.md)

[include](/home/user/xyan/xy.ai.workbench/project/resident/plan/plan_h_00_overview.md)

[include](/home/user/xyan/xy.ai.workbench/project/resident/plan/plan_h_06_mutable_weights_bandit.md)

SystemInit:  id=0de37b92-bd40-41e5-a0da-ddb05b40633a, cwd=/home/user/xyan/xy.ai.workbench/project, model=claude-sonnet-5-5
Text:
 Ich suche zuerst nach bestehenden ADR-Einträgen und den relevanten Code-Stellen (Params, Gewichte, Addons).
ReasoningToken: 100
Control Request:
```yaml
id: 4172-8a
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/docs
    - /home/user/xyan/xy.ai.workbench/project
    pattern: ADR
    include:
    - '*.md'
    exclude:
    - context_*.md
    limit: 30
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src/resident
    pattern: non_exhaustive|const .*WEIGHTS|static .*WEIGHTS|include_bytes|gen\b
    include:
    - '*.rs'
    exclude:
    - target/**
    limit: 40
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src/resident
    pattern: Addon|Params|enum LifeState|Unloaded|Resident
    include:
    - '*.rs'
    exclude:
    - target/**
    limit: 40
  intent: survey
  reason: "ADR-Ablage, Params, Gewichte und Lifecycle im Code finden"
```
/answer 4172-8a allow
Control Request:
```yaml
id: 4172-8b
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/docs
      - /home/user/xyan/xy.ai.workbench/project
      matches:
      - path: architecture/gpu_memory.md
        matches:
        - '- `CudaMemoryManager::new` rejects a plan that exceeds `free - headroom` (`DEFAULT_HEADROOM` = 512 MiB) before allocating.'
      - path: resident/archive/create_plan_h_q.md
        matches:
        - "- Ein kurzer ADR-Eintrag hält die Umsetzungsentscheidung fest: „Thesen-§3 Ringbuffer in Phase 1 durch Pull-Queue ersetzt; Ringbuffer = späterer Persistent-Scorer-Modus\" (Nachvollziehbarkeit der Thesen-Treue)."
        - "- Ein ADR-Eintrag dokumentiert die Divergenz: „Signal-/Frequenzkompression ist Oberschicht-Vorfilter, nicht Engine. Engine stellt nur generische Scope-/Summary-Hooks bereit.\""
        - "- Ein ADR-Eintrag dokumentiert die Divergenz: „Gewichte immutabel, nur Index bidirektional mutabel; Bandit/Online-Adaption bewusst nicht adressiert.\""
        - "- **H5/H6** sind klar als *außerhalb des Engine-Scopes* markiert (nur Nicht-Ausschluss über offene `AccessMethod`/`Scope`/Params + ADR-Divergenznotiz), passend zum Thesen-Abgleich im Metaplan."
      - path: resident/plan/plan_h_01_gpu_ringbuffer.md
        matches:
        - "- Ein kurzer ADR-Eintrag hält die Umsetzungsentscheidung fest: „Thesen-§3 Ringbuffer in Phase 1 durch Pull-Queue ersetzt; Ringbuffer = späterer Persistent-Scorer-Modus\" (Nachvollziehbarkeit der Thesen-Treue)."
      - path: resident/plan/plan_h_06_mutable_weights_bandit.md
        matches:
        - "- Ein ADR-Eintrag dokumentiert die Divergenz: „Gewichte immutabel, nur Index bidirektional mutabel; Bandit/Online-Adaption bewusst nicht adressiert.\""
      - path: resident/plan/runs/plan_d_04_model_encoder.md
        matches:
        - "match: \"pub const DEFAULT_HEADROOM: u64 = 512 << 20;\""
        - "19:pub use budget::{ArenaKind, ArenaSizes, Budget, BudgetProbe, DEFAULT_HEADROOM};"
      - path: resident/plan/runs/plan_h_01_gpu_ringbuffer.md
        matches:
        - "pattern: ADR|Ringbuffer"
        - "reason: ADR-Ablageort und bestehende Launcher-Tests finden"
        - '- ''- `CudaMemoryManager::new` rejects a plan that exceeds `free - headroom` (`DEFAULT_HEADROOM` = 512 MiB) before allocating.'''
        - "`ArenaKind`, der Evictor und `SwapManager::begin_batch` sind bereits abgedeckt; offen bleiben `LaneClass`, `LaunchMode`, ein No-GPU-Test und der ADR. Als Nächstes prüfe ich die Divergenz-Stelle in der Scope-Charta."
        - "reason: Struktur der Scope-Charta für den ADR-Eintrag und Imports der Exec-Tests prüfen"
        - '- **ADR:** Die Entscheidung "Ringbuffer in Phase 1 durch Pull-Queue ersetzt" steht in `docs/architecture/scope_charta.md` §5. Einen separaten Eintrag habe ich nicht angelegt.'
      - path: resident/plan/runs/plan_h_06_mutable_weights_bandit.md
        matches:
        - "Ich suche zuerst nach bestehenden ADR-Einträgen und den relevanten Code-Stellen (Params, Gewichte, Addons)."
        - "pattern: ADR"
        - "reason: \"ADR-Ablage, Params, Gewichte und Lifecycle im Code finden\""
      - path: resident/plan/runs/plan_e_03.md
        matches:
        - "match: \"pub use budget::{ArenaKind, ArenaSizes, Budget, BudgetProbe, DEFAULT_HEADROOM};\""
      - path: resident/plan/runs/plan_h_05_signalkompression.md
        matches:
        - "Ich suche zuerst nach `AccessMethod`, `Scope`, `summary` und dem ADR-Ort im Code."
        - "pattern: ADR"
        - "reason: Existierende ADR-Konvention/Ablageort für den Divergenz-Eintrag finden"
        - '- ''- `CudaMemoryManager::new` rejects a plan that exceeds `free - headroom` (`DEFAULT_HEADROOM` = 512 MiB) before allocating.'''
        - "- \"- Ein kurzer ADR-Eintrag hält die Umsetzungsentscheidung fest: „Thesen-§3 Ringbuffer in Phase 1 durch Pull-Queue ersetzt; Ringbuffer = späterer Persistent-Scorer-Modus\\\" (Nachvollziehbarkeit der Thesen-Treue).\""
        - "- \"- Ein ADR-Eintrag dokumentiert die Divergenz: „Signal-/Frequenzkompression ist Oberschicht-Vorfilter, nicht Engine. Engine stellt nur generische Scope-/Summary-Hooks bereit.\\\"\""
        - "- \"- Ein ADR-Eintrag dokumentiert die Divergenz: „Gewichte immutabel, nur Index bidirektional mutabel; Bandit/Online-Adaption bewusst nicht adressiert.\\\"\""
        - "- \"- **H5/H6** sind klar als *außerhalb des Engine-Scopes* markiert (nur Nicht-Ausschluss über offene `AccessMethod`/`Scope`/Params + ADR-Divergenznotiz), passend zum Thesen-Abgleich im Metaplan.\""
        - "- \"match: \\\"pub const DEFAULT_HEADROOM: u64 = 512 << 20;\\\"\""
      warning: "Limit of 30 matches reached; further results may exist. Narrow the pattern, directory or include/exclude filters, or raise limit."
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src/resident
      matches:
      - path: segment/mod.rs
        matches:
        - "#[non_exhaustive]"
        - "pub gen: u64,"
        - "gen: u64,"
        - "Ok(Self { base, slots, stride, class, dtype, dim, gen })"
        - "Ok(SegmentHit { segment_gen: self.view.gen, slot, score: kernel.score })"
        - "tracing::debug!(segment_gen = self.view.gen, \"segment released\");"
        - "pub segment_gen: u64,"
        - "/// 24 bytes per segment, little endian: `segment_gen: u64, base: u64, count: u32, stride: u32`."
        - out.extend_from_slice(&e.segment_gen.to_le_bytes());
        - ".map(|s| SegmentTableEntry { segment_gen: s.view.gen, desc: s.view.desc() })"
        - "return Err(SegmentError::StaleGeneration { gen: next.generation });"
        - "Region { arena: ArenaId(1), offset, len, gen: 0 }"
        - "pub fn segment(store: &SegmentStore, id: u64, gen: u64, offset: u64, slots: u32) -> Arc<Segment> {"
        - "let view = SegmentView::new(region(offset, 1 << 20), slots, SlotClass::T32, 4, DType::F32, gen).unwrap();"
      - path: segment/slots.rs
        matches:
        - "pub gen: u64,"
        - "gen: w.seq,"
        - "tracing::trace!(segment_gen = self.view.gen, slot = old, \"tombstone\");"
        - "tracing::trace!(segment_gen = self.view.gen, slot, \"upsert\");"
        - "tracing::trace!(segment_gen = self.view.gen, slot, \"tombstone\");"
      - path: segment/sync.rs
        matches:
        - "pub segment_gen: u64,"
        - "pub manifest_gen: u64,"
        - let manifest_gen = staged.manifest.generation();
        - "for (kind, segment_id, segment_gen) in events {"
        - "self.bus.emit(SegmentEvent { kind, segment_id, segment_gen, manifest_gen });"
        - "None => Some((SegmentEventKind::Added, s.id(), s.view().gen)),"
        - "Some(p) if p.view().gen != s.view().gen => {"
        - "Some((SegmentEventKind::Changed, s.id(), s.view().gen))"
      - path: segment/error.rs
        matches:
        - "#[non_exhaustive]"
        - "#[error(\"segment generation {gen} is not part of the current manifest\")]"
        - "StaleGeneration { gen: u64 },"
        - "#[error(\"no entry mapped for slot {slot} of segment generation {gen}\")]"
        - "MappingMissing { gen: u64, slot: u32 },"
        - "#[error(\"stale mapping for segment generation {gen}\")]"
        - "StaleMapping { gen: u64 },"
        - "MappingError::StaleMapping { gen } => SegmentError::StaleGeneration { gen },"
      - path: segment/test/mod.rs
        matches:
        - "assert!(matches!(store.publish(stale), Err(SegmentError::StaleGeneration { gen: 1 })));"
      - path: segment/test/sync.rs
        matches:
        - "SegmentEvent { kind: SegmentEventKind::Added, segment_id: 1, segment_gen: 10, manifest_gen: 1 }"
        - "assert_eq!((ev.kind, ev.segment_gen, ev.manifest_gen), (SegmentEventKind::Changed, 11, 2));"
        - "assert!(matches!(publisher.poll(), Err(SegmentError::StaleGeneration { gen: 3 })));"
      - path: segment/test/mapping.rs
        matches:
        - "segment_gen: 10,"
      warning: "Limit of 40 matches reached; further results may exist. Narrow the pattern, directory or include/exclude filters, or raise limit."
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src/resident
      matches:
      - path: mod.rs
        matches:
        - "//! Resident engine: embedding/similarity substrate, kept warm."
      - path: processor/tests.rs
        matches:
        - "use crate::resident::error::{PlannerError, ResidentError};"
        - "EmbeddingParams, Output, Payload, SimilarityParams, TaskError, TaskKind, TaskMeta,"
        - "use crate::resident::op::{Lease, Op, OpDesc, OpKind, OpParams, PinCount, Requirements, Value};"
        - ") -> Result<Expansion, ResidentError> {"
        - "params: EmbeddingParams::default(),"
        - "params: SimilarityParams::new(IndexId(\"i\".into())),"
        - "let params = OpParams::new(Requirements::new(ResourceKind::Scorer, EngineSelector::Cpu));"
      - path: processor/hub.rs
        matches:
        - use crate::resident::error::ResidentError;
        - "use crate::resident::op::{OpParams, OperationNode, ResourceKind, TaskCtx, Value};"
        - "fn expand(&self, meta: &TaskMeta, payload: Payload, ctx: &Arc<TaskCtx>) -> Result<Expansion, ResidentError>;"
        - ") -> Result<Expansion, ResidentError> {"
        - "pub fn claim(&self, side: Side, accepts: &dyn Fn(&OpParams) -> bool) -> Option<OpJob> {"
        - "pub fn fail_op(&self, job: OpJob, err: ResidentError) {"
      - path: processor/error.rs
        matches:
        - "use crate::resident::error::{IsRetryable, ResidentError};"
        - "source: Arc<ResidentError>,"
      - path: processor/test/error.rs
        matches:
        - "let transient: ResidentError = IndexError::ArenaExhausted { requested: 1, available: 0 }.into();"
        - "let e: ResidentError = OrchestrationError::Shutdown.into();"
        - "assert!(matches!(e, ResidentError::Orchestration(OrchestrationError::Shutdown)));"
      - path: processor/test/ops.rs
        matches:
        - "fn any(_: &OpParams) -> bool {"
        - "let cpu_only = |p: &OpParams| p.requires.engine == EngineSelector::Cpu;"
      - path: processor/ops.rs
        matches:
        - use crate::resident::error::ResidentError;
        - "use crate::resident::op::{EngineSelector, OpParams, OperationNode, TaskCtx};"
        - "pub fn fail(&self, err: ResidentError) {"
        - "pub fn params(&self) -> &OpParams {"
        - "pub type OpQueue = Queue<OpParams, OpJob, (), ()>;"
        - "fn claim(&self, accepts: &dyn Fn(&OpParams) -> bool, max_skip: usize) -> Option<OpJob> {"
        - "pub fn claim(&self, side: Side, accepts: &dyn Fn(&OpParams) -> bool) -> Option<OpJob> {"
        - "let params = OpParams::new(Requirements::new(resource, engine));"
      - path: facade/mod.rs
        matches:
        - pub use blocking::ResidentBlocking;
        - "EmbeddingParams, InferenceParams, Output, PairInput, ScorerKind, SimilarityParams,"
        - "pub use resident::{IndexInfo, Residency, Resident, ResidentBuilder, ResidentConfig};"
      - path: facade/resident.rs
        matches:
        - "EmbeddingParams, InferenceParams, PairInput, SimilarityParams, TextInput, VectorInput,"
        - "use crate::resident::error::{ConfigError, PlannerError, ResidentError};"
        - "pub struct ResidentConfig {"
        - "impl ResidentConfig {"
        - ") -> Result<Expansion, ResidentError> {"
        - "pub struct ResidentBuilder {"
        - "impl ResidentBuilder {"
      warning: "Limit of 40 matches reached; further results may exist. Narrow the pattern, directory or include/exclude filters, or raise limit."
```
/answer 4172-8b allow
ReasoningToken: 83
Control Request:
```yaml
id: 4172-8c
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/docs/architecture
    pattern: ^#|H5|H6|Divergenz|immutab|mutab
    include:
    - scope_charta.md
    - addons.md
    - feature_matrix.md
    limit: 50
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src/resident
    pattern: struct (Task|\w*Params)\b|enum Lifecycle|enum (Weights|Model)State|fn (reload|unload|swap)
    include:
    - '*.rs'
    limit: 30
  intent: survey
  reason: Struktur der Scope-Charta (ADR-Ort) und Params-/Lifecycle-Typen prüfen
```
/answer 4172-8c allow
Control Request:
```yaml
id: 4172-8d
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/docs/architecture
      matches:
      - path: scope_charta.md
        matches:
        - "# Scope Charta: Resident Engine vs. Search Layer"
        - "> Sources: `resident_planplan.md` (Kernbefund, Abgleich-Tabelle, Zentrale Divergenz,"
        - "## 1. Purpose"
        - "## 2. Boundary Table"
        - '| §6 | Frequency compression (JPEG/DCT) | **E / —** | mentioned only as outer-layer prefilter (FFT/compression domain as scope) | Search layer (H5) |'
        - '| §6.1 | Two axes / semantic spectrogram / wavelet | **—** | not addressed at all | Unaddressed (H5) |'
        - "| §9 | Parameter/activation duality, mutable weights | **—** | plan: weights immutable (weights arena long-lived); only index bidirectionally mutable | Deliberate divergence; not engine scope (H6) |"
        - "| §9 | Online adaptation (bandit, reward stream) | **—** | not addressed | Unaddressed (H6) |"
        - '| §9.1 | Bandit as meta-router over cascade | **—** | not addressed | Unaddressed (H6) |'
        - "## 3. Engine Delivers"
        - "derived, bidirectionally mutable mirror; the index object itself owns persistence/mmap"
        - "## 4. Outside the Engine (Search Layer)"
        - "- Signal compression (FFT/wavelet, semantic spectrogram) — H5, not engine scope at all."
        - "- Mutable weights / online bandit adaptation — H6, not engine scope at all."
        - "## 5. Central Divergence: Ring Buffer vs. Pull Queue (§3)"
        - "### 5.1 Divergence: Signal/Frequency Compression (§6/§6.1, H5)"
        - "## 6. Hook Obligations (Phase 1 must not foreclose)"
        - "outlook position (H1–H6):"
        - "- **H5 — Signal compression** (FFT/wavelet, semantic spectrogram; §6/§6.1, status E/—):"
        - "- **H6 — Mutable weights / online bandit** (§9/§9.1, status —): **not engine scope.** Weights"
        - stay immutable (long-lived weights arena); only the index is bidirectionally mutable. If ever
        - "pursued, it becomes its own new workstream (reward queue, mutable weights arena); Phase 1 only"
        - "## 7. References"
        - "## 8. Acceptance Check"
        - 2. Every hook obligation (H1–H6) is tied to a concrete later extension. ✅
      - path: addons.md
        matches:
        - "# Addons: iteration, cascades, recursion"
        - "## Core"
        - "## Addon"
        - "## Search layer"
        - "## Open points"
      - path: feature_matrix.md
        matches:
        - "# Feature Matrix"
        - "## CI"
        - "## Feature gating rule"
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src/resident
      matches:
      - path: facade/params.rs
        matches:
        - "pub struct EmbeddingParams {"
        - "pub struct SimilarityParams {"
        - "pub struct InferenceParams {"
      - path: kernel/test/golden.rs
        matches:
        - "fn swap_at_near_tie_is_accepted() {"
        - "fn swap_of_distinct_scores_is_reported() {"
      - path: manager/runtime.rs
        matches:
        - "pub fn swap(&self) -> &SwapManager<'a, 'b> {"
      - path: planner/mod.rs
        matches:
        - "pub fn reload(&self, rules: RuleSet) {"
      - path: planner/test/mod.rs
        matches:
        - "fn reload_swaps_snapshot_atomically() {"
      - path: op/mod.rs
        matches:
        - "pub struct OpParams {"
```
/answer 4172-8d allow
Text:
 Die Charta enthält H6 schon in Tabelle und §6. Ich lese §5.1 und §6 und prüfe Params und Lifecycle.
Control Request:
```yaml
id: 4172-8e
toolName: bash
arguments:
  items:
  - cwd: /home/user/xyan/xy.ai.workbench/rag
    script:
    - "grep -n '' docs/architecture/scope_charta.md | sed -n '/^[0-9]*:## 5\\./,/^[0-9]*:## 7\\./p'"
    - grep -n -B2 -A6 'non_exhaustive' src/resident/facade/params.rs | head -60
    - grep -rn 'Unloaded\|Loading' src/resident --include=*.rs | head -15
  reason: Divergenz-Abschnitte der Charta lesen und Params-/Lifecycle-Offenheit prüfen
```
/answer 4172-8e deny nutze search
Control Request:
```yaml
id: 4172-8f
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/rag/docs/architecture/scope_charta.md
  text: H6
  reason: Abschnitte der Charta mit H6 inkl. Quelltext lesen
```
/answer 4172-8f allow
Control Request:
```yaml
id: 4172-90
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/rag/docs/architecture/scope_charta.md
      nodes:
      - id: 35617d.dc0d0a
        code: |
          ## 2. Boundary Table

          Status legend: **K** = core/implemented · **I** = infrastructure/hook prepared ·
          **A** = outlook/later, inside the engine · **E** = deliberately outside the engine ·
          **—** = not addressed.

          | Thesis (§) | Aspect | Status | Evidence | Responsibility |
          |---|---|---|---|---|
          | §2 | Disaggregation tokenizer/weights/kernel | **K** | `resident.md` Stack; `subengine` §3; `KernelRegistry` §4.5 | Engine |
          | §3 | Residency (no per-request setup, lazy+TTL) | **K** | `resident.md` control flow; `subengine` §6 lifecycle | Engine |
          | §3 | Ring-buffer producer/consumer compute | **A** | `resident.md` Ausblick; `resident_index.md` "Ringbuffer-Modus (geplant)"; `subengine` §4.6 | Engine (deferred, see §4 Divergence) |
          | §3 | Multiple models resident | **K** (restricted) | fixed set before init, model-ID-keyed, lazy | Engine; dynamic add not planned |
          | §3 | Batching optional / streaming | partly **K** | batching is real mechanic (token budget); streaming equivalence is outlook only | Engine |
          | §4 | MaxSim = geometry, model-decoupled | **K** (principle) | `resident_precision.md` "not coupled"; `resident_index.md` "kernel only knows segments"; `MaxSimOp` | Engine |
          | §4 | ColBERT multi-vector model | **A** | `resident_first.md` switch path; "no ColBERT module in candle-transformers" | Engine (switch path, not default) |
          | §4 | Position instead of relation (index stores positions) | **K** | index stores entries/vectors; relation is outer layer | Engine stores positions; relation = search layer |
          | §5 | Sliding window instead of chunking | **A** | `resident.md`/`planner` "gleitende Indizierung (Ausblick)" | Engine (data model only, prepared) |
          | §5 | Index data model for multi-vector/window | **I** | `resident_index.md` variable-length entry; `subengine` slot classes | Engine (hook present) |
          | §5 | Chunking itself | **E** | `resident_index.md`/`resident_precision.md` "Nicht Teil … Suchschicht" | Search layer |
          | §6 | Frequency compression (JPEG/DCT) | **E / —** | mentioned only as outer-layer prefilter (FFT/compression domain as scope) | Search layer (H5) |
          | §6.1 | Two axes / semantic spectrogram / wavelet | **—** | not addressed at all | Unaddressed (H5) |
          | §7 | Cascaded refinement (fractal) | **I / A** | `resident_index.md` "Addons (Iteration/Kaskade/Rekursion)"; funnel "later"; scope resolver/centroid | Engine (addon pattern only, not core) (H3) |
          | §9 | Parameter/activation duality, mutable weights | **—** | plan: weights immutable (weights arena long-lived); only index bidirectionally mutable | Deliberate divergence; not engine scope (H6) |
          | §9 | Online adaptation (bandit, reward stream) | **—** | not addressed | Unaddressed (H6) |
          | §9.1 | Bandit as meta-router over cascade | **—** | not addressed | Unaddressed (H6) |
          | §10 | Drift monitor / re-embed trigger | **I / E** | stamps/change-notify support re-embed; monitor itself is outside | Hook via `changed()`/stamps; monitor = search layer |
          | §10 | Hierarchical index (coarse/fine, frequency bands) | **I** | `resident_index.md` `children`/`subtree`/`summary` generic; frequency bands not | Engine (generic structure open) |
          | §10 | Bridge to pylate (reference/validation) | partly **—** | `resident_first.md`: reference comparison with sentence-transformers (not pylate) | Different reference used |
          | §2/§10 | Stack (safetensors/tokenizers/candle/cudarc/Parquet) | **K** | full stack + tokio/crossbeam/mmap | Engine |
      - id: 35617d.49bc3c
        code: |+
          ## 4. Outside the Engine (Search Layer)

          - Chunking.
          - Query formulation.
          - Candidate-k / rerank policy.
          - Retrieval quality / completeness guarantees.
          - Approximative prefilters (centroid, FFT, compression) — these lose edge matches by nature
            and are an outer-layer decision, not an engine concern.
          - Drift monitor (consumes engine hooks, lives outside).
          - Signal compression (FFT/wavelet, semantic spectrogram) — H5, not engine scope at all.
          - Mutable weights / online bandit adaptation — H6, not engine scope at all.
          - Index persistence format and persistence timing (owned by the index object itself).
          - Iteration/cascade/recursion and parent-child task relationships as *core* engine behavior
            (only permitted as an addon pattern, never as scheduler-level task relationship/fan-out+join).

      - id: 35617d.43bb35.xvWuPW|OdPap5
        code: |
          Phase 1 implementation must keep the following extension points open, each tied to a later
          outlook position (H1–H6):
      - id: 35617d.43bb35
        code: |+
          ## 6. Hook Obligations (Phase 1 must not foreclose)

          Phase 1 implementation must keep the following extension points open, each tied to a later
          outlook position (H1–H6):

          - **H1 — GPU ring-buffer mode** (§3, status A): extensible parameter object, separate
            embedding/MaxSim rings, acquire-load-pointer in the manager discipline (E4). The pull-queue
            model must not structurally preclude a later persistent-kernel/ring-buffer scorer mode.
          - **H2 — Sliding-window indexing** (§5, status A/I): index data model (variable-length entry,
            slot classes) is already the hook; kernel/pipeline follow later.
          - **H3 — Funnel/PLAID + cross-encoder** (§7, status I/A): realizable via addons + centroid
            scope resolver; op exchange behind the same trait.
          - **H4 — Graph-capture / persistent scorer** (§3/§9, status A): same hook family as H1.
          - **H5 — Signal compression** (FFT/wavelet, semantic spectrogram; §6/§6.1, status E/—):
            **not engine scope.** Obligation: `AccessMethod`/`Scope`/`summary()` must not foreclose a
            later "frequency/compression domain" as a *search-layer prefilter*. No engine workstream.
          - **H6 — Mutable weights / online bandit** (§9/§9.1, status —): **not engine scope.** Weights
            stay immutable (long-lived weights arena); only the index is bidirectionally mutable. If ever
            pursued, it becomes its own new workstream (reward queue, mutable weights arena); Phase 1 only
            commits to "not building this out structurally".

      - id: 35617d.b89864
        code: |
          ## 8. Acceptance Check

          1. Boundary table reproduces every row of the metaplan Abgleich-Tabelle. ✅
          2. Every hook obligation (H1–H6) is tied to a concrete later extension. ✅
          3. Ring-buffer ↔ pull-queue divergence is documented as a decision, not an inconsistency. ✅
          4. C1 and E2 reference this charta as their planning basis (see §7). ✅
```
/answer 4172-90 allow
Control Request:
```yaml
id: 4172-91
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src/resident
    pattern: non_exhaustive
    include:
    - params.rs
    - mod.rs
    limit: 30
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src/resident
    pattern: Unloaded|Loading|Evict
    include:
    - '*.rs'
    limit: 20
  reason: Params-Offenheit und Lifecycle-Zustände prüfen
```
/answer 4172-91 allow
Control Request:
```yaml
id: 4172-92
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src/resident
      matches:
      - path: segment/mod.rs
        matches:
        - "#[non_exhaustive]"
      - path: facade/params.rs
        matches:
        - "#[non_exhaustive]"
      - path: index/mod.rs
        matches:
        - "#[non_exhaustive]"
      - path: op/mod.rs
        matches:
        - "#[non_exhaustive]"
      - path: op/test/mod.rs
        matches:
        - "fn affinity_is_non_exhaustive() {"
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src/resident
      matches:
      - path: processor/resource.rs
        matches:
        - "Loading,"
        - "Evicting,"
        - "1 => Self::Loading,"
        - "_ => Self::Evicting,"
        - "if self.transition(ResourceState::Uninit, ResourceState::Loading) {"
        - "&& self.transition(ResourceState::Backoff, ResourceState::Loading)"
        - "/// `Ready -> Evicting` only when idle, unleased and past the TTL."
        - "|| !self.transition(ResourceState::Ready, ResourceState::Evicting)"
        - "debug_assert_eq!(self.state(), ResourceState::Evicting);"
      - path: processor/test/resource.rs
        matches:
        - "assert_eq!(s.state(), ResourceState::Loading);"
      - path: kernel/registry.rs
        matches:
        - "/// Loads on first use. Loading holds the write lock, so concurrent callers never compile the"
      - path: subengine/lifecycle.rs
        matches:
        - "Unloaded,"
        - "Loading,"
        - "Evicting,"
        - "UNLOADED => ResourceState::Unloaded,"
        - "LOADING => ResourceState::Loading,"
        - "EVICTING => ResourceState::Evicting,"
        - "return Err(SubengineError::ResourceEvicting {"
      - path: subengine/error.rs
        matches:
        - "ResourceEvicting { resource: String },"
        - "self, Self::ResourceEvicting { .. } | Self::Backoff { .. } |"
      warning: "Limit of 20 matches reached; further results may exist. Narrow the pattern, directory or include/exclude filters, or raise limit."
```
/answer 4172-92 allow
Control Request:
```yaml
id: 4172-93
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/rag/docs/architecture/scope_charta.md
  regexp: ^### 5\.1
  reason: Format von §5.1 für konsistenten §5.2 lesen
```
/answer 4172-93 allow
ReasoningToken: 109
Control Request:
```yaml
id: 4172-94
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/rag/docs/architecture/scope_charta.md
      nodes: []
```
/answer 4172-94 allow
Control Request:
```yaml
id: 4172-95
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/facade/params.rs
  reason: "Prüfen, ob Params erweiterbar (non_exhaustive/Builder) sind"
```
/answer 4172-95 allow
Control Request:
```yaml
id: 4172-96
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/facade/params.rs
      nodes:
      - id: an6syr|7wsnLF
        signature: "use crate::resident::index::{EntryId, Scope};"
      - id: jTwlTq|iUpnXF
        signature: "const DEFAULT_PRECISION: f32 = 1.0;"
      - id: TextRole
        signature: "pub enum TextRole {"
      - id: ScorerKind
        signature: "pub enum ScorerKind {"
        docstring: "Scoring method of a similarity task. Rerank is not a task kind of its own: it i…"
      - id: impl_ScorerKind
        signature: "impl ScorerKind {"
        children:
        - id: impl_ScorerKind.label
          signature: "pub fn label(&self) -> &'static str {"
      - id: TextInput
        signature: "pub struct TextInput {"
      - id: impl_TextInput
        signature: "impl TextInput {"
        children:
        - id: impl_TextInput.new
          signature: "pub fn new(texts: impl IntoIterator<Item = impl Into<String>>) -> Self {"
        - id: impl_TextInput.single
          signature: "pub fn single(text: impl Into<String>) -> Self {"
        - id: impl_TextInput.texts
          signature: "pub fn texts(&self) -> &[String] {"
      - id: VectorInput
        signature: "pub struct VectorInput {"
      - id: impl_VectorInput
        signature: "impl VectorInput {"
        children:
        - id: impl_VectorInput.new
          signature: "pub fn new(vectors: Vec<Vec<f32>>) -> Self {"
        - id: impl_VectorInput.vectors
          signature: "pub fn vectors(&self) -> &[Vec<f32>] {"
      - id: PairInput
        signature: "pub struct PairInput {"
        docstring: "Cross-encoder input; kept apart from [`TextRole`] so role and pair scoring stay…"
      - id: impl_PairInput
        signature: "impl PairInput {"
        children:
        - id: impl_PairInput.new
          signature: "pub fn new(query: impl Into<String>, chunk: impl Into<String>) -> Self {"
        - id: impl_PairInput.query
          signature: "pub fn query(&self) -> &str {"
        - id: impl_PairInput.chunk
          signature: "pub fn chunk(&self) -> &str {"
      - id: EmbeddingParams
        signature: "pub struct EmbeddingParams {"
      - id: impl_EmbeddingParams
        signature: "impl EmbeddingParams {"
        children:
        - id: impl_EmbeddingParams.new
          signature: "pub fn new(role: TextRole) -> Self {"
        - id: impl_EmbeddingParams.write_index
          signature: "pub fn write_index(mut self, index: IndexId) -> Self {"
        - id: impl_EmbeddingParams.precision
          signature: "pub fn precision(mut self, precision: f32) -> Self {"
        - id: impl_EmbeddingParams.role
          signature: "pub fn role(&self) -> TextRole {"
        - id: impl_EmbeddingParams.target_index
          signature: "pub fn target_index(&self) -> Option<&IndexId> {"
        - id: impl_EmbeddingParams.get_precision
          signature: "pub fn get_precision(&self) -> f32 {"
        - id: impl_EmbeddingParams.with_ext
          signature: "pub fn with_ext<T: Any + Send + Sync>(mut self, value: T) -> Self {"
        - id: impl_EmbeddingParams.ext
          signature: "pub fn ext(&self) -> &Extensions {"
      - id: impl_Default_for_EmbeddingParams
        signature: "impl Default for EmbeddingParams {"
        children:
        - id: impl_Default_for_EmbeddingParams.default
          signature: "fn default() -> Self {"
      - id: SimilarityParams
        signature: "pub struct SimilarityParams {"
      - id: impl_SimilarityParams
        signature: "impl SimilarityParams {"
        children:
        - id: impl_SimilarityParams.new
          signature: "pub fn new(index: IndexId) -> Self {"
        - id: impl_SimilarityParams.top_k
          signature: "pub fn top_k(mut self, top_k: usize) -> Self {"
        - id: impl_SimilarityParams.scope
          signature: "pub fn scope(mut self, scope: Scope) -> Self {"
        - id: impl_SimilarityParams.precision
          signature: "pub fn precision(mut self, precision: f32) -> Self {"
        - id: impl_SimilarityParams.scorer
          signature: "pub fn scorer(mut self, scorer: ScorerKind) -> Self {"
        - id: impl_SimilarityParams.index
          signature: "pub fn index(&self) -> &IndexId {"
        - id: impl_SimilarityParams.get_top_k
          signature: "pub fn get_top_k(&self) -> usize {"
        - id: impl_SimilarityParams.get_scope
          signature: "pub fn get_scope(&self) -> &Scope {"
        - id: impl_SimilarityParams.get_scorer
          signature: "pub fn get_scorer(&self) -> &ScorerKind {"
        - id: impl_SimilarityParams.get_precision
          signature: "pub fn get_precision(&self) -> f32 {"
        - id: impl_SimilarityParams.with_ext
          signature: "pub fn with_ext<T: Any + Send + Sync>(mut self, value: T) -> Self {"
        - id: impl_SimilarityParams.ext
          signature: "pub fn ext(&self) -> &Extensions {"
      - id: InferenceParams
        signature: "pub struct InferenceParams {"
      - id: impl_InferenceParams
        signature: "impl InferenceParams {"
        children:
        - id: impl_InferenceParams.new
          signature: "pub fn new() -> Self {"
        - id: impl_InferenceParams.model
          signature: "pub fn model(mut self, model: ModelId) -> Self {"
        - id: impl_InferenceParams.precision
          signature: "pub fn precision(mut self, precision: f32) -> Self {"
        - id: impl_InferenceParams.get_model
          signature: "pub fn get_model(&self) -> Option<&ModelId> {"
        - id: impl_InferenceParams.get_precision
          signature: "pub fn get_precision(&self) -> f32 {"
        - id: impl_InferenceParams.with_ext
          signature: "pub fn with_ext<T: Any + Send + Sync>(mut self, value: T) -> Self {"
        - id: impl_InferenceParams.ext
          signature: "pub fn ext(&self) -> &Extensions {"
      - id: impl_Default_for_InferenceParams
        signature: "impl Default for InferenceParams {"
        children:
        - id: impl_Default_for_InferenceParams.default
          signature: "fn default() -> Self {"
      - id: Output
        signature: "pub enum Output {"
```
/answer 4172-96 allow
ReasoningToken: 144
Control Request:
```yaml
id: 4172-97
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src/resident
    pattern: "(static|const) [A-Z_]*(WEIGHT|MODEL)[A-Z_]*|include_bytes!"
    include:
    - '*.rs'
    limit: 10
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/docs/architecture
    pattern: ^### 5\.1|^## 6\.
    include:
    - scope_charta.md
    intent: editing
  reason: Auf eingebrannte Gewichte prüfen und Einfügestelle für §5.2 finden
```
/answer 4172-97 allow
Control Request:
```yaml
id: 4172-98
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src/resident
      matches:
      - path: index/schema.rs
        matches:
        - "const KEY_MODEL_ID: &str = \"embedding.model_id\";"
        - "const KEY_MODEL_REVISION: &str = \"embedding.model_revision\";"
      - path: telemetry.rs
        matches:
        - "pub const FIELD_MODEL_ID: &str = \"model_id\";"
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/docs/architecture
      matches:
      - path: scope_charta.md
        nodes:
        - id: 35617d.445827.290ad1
          matches:
          - "### 5.1 Divergence: Signal/Frequency Compression (§6/§6.1, H5)"
        - id: 35617d.43bb35
          matches:
          - "## 6. Hook Obligations (Phase 1 must not foreclose)"
```
/answer 4172-98 allow
Text:
 Es gibt keine eingebrannten Gewichte. Ich füge den ADR-Eintrag als §5.2 vor §6 ein.
Control Request:
```yaml
id: 4172-99
toolName: ast_insert
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/docs/architecture/scope_charta.md
    id: 35617d.43bb35
    position: before
    source: |+
      ### 5.2 Divergence: Mutable Weights / Online Bandit (§9/§9.1, H6)

      **Decision:** Weights are immutable; only the index is bidirectionally mutable. Bandit-based
      meta-routing and online adaptation are deliberately not addressed and not an engine workstream.

      **Rationale:** The weights arena is long-lived and freed only on unload. Online weight updates
      risk catastrophic forgetting, reward hacking, feedback loops and non-reproducibility; they are
      viable only for small parts (router, score weights), never the base model.

      **Kept open (not built):**
      - Weights stay a reloadable resource (`Unloaded -> Loading -> Resident`, `Region.gen`
        invalidation), so a later versioned snapshot swap remains possible.
      - Task params stay extensible (`with_ext`, builders), so context/action/propensity log fields
        can be added without a breaking change.
      - A meta-router fits the addon path: it enqueues follow-up tasks with differing params through
        the normal queue.

      **Out of scope:** bandit, reward queue, mutable weights arena, online learning, propensity
      logging. If pursued, this is a separate workstream with its own learner process and versioned
      atomic swap.

  reason: ADR-Eintrag zur H6-Divergenz in der Scope-Charta ergänzen
```
/answer 4172-99 allow
Result Stats: id=0de37b92-bd40-41e5-a0da-ddb05b40633a, total: 26667, in: 16, out: 4008, reason: 0, read: 143414, write: 22643, start: 1793049031574, end: 1793049037414, time: 5772, total: 43754, io: 0, control: 68, sent: 0, recv: 139
H6 ist umgesetzt. Es gibt dafür nur einen Doku-Eintrag und keinen Code, weil der Plan keine Engine-Arbeit vorsieht. Kompiliert oder getestet habe ich nichts.

- **ADR-Eintrag:** Neuer Abschnitt `### 5.2 Divergence: Mutable Weights / Online Bandit (§9/§9.1, H6)` in `rag/docs/architecture/scope_charta.md`, direkt vor §6. Ein separates ADR-Verzeichnis gibt es nicht. Wie schon bei H1 und H5 liegt die Divergenz in der Charta. Der Eintrag hält fest:
  - Gewichte sind immutabel, nur der Index ist bidirektional mutabel.
  - Bandit und Online-Adaption sind bewusst nicht adressiert.
  - Eine spätere Umsetzung wäre ein eigener Workstream.
- **Reload-Fähigkeit der Gewichte:** In `src/resident` gibt es keine eingebrannten Gewichte, also weder statische Konstanten noch `include_bytes!`. Der Lifecycle `Unloaded→Loading→…` existiert in `subengine/lifecycle.rs` und `processor/resource.rs`. Ich habe den Reload-Pfad nicht durch einen Lauf geprüft, nur per Suche und Outline.
- **Erweiterbare Params:** `EmbeddingParams`, `SimilarityParams` und `InferenceParams` haben `with_ext` und `ext()` (typisierte `Extensions`) und sind Builder. Spätere Logging-Felder für Kontext, Aktion und Propensity lassen sich damit ohne Bruch ergänzen.
- **Addon-Pfad für einen Meta-Router:** Das habe ich nur in der Doku (§6 der Charta) abgeglichen, nicht im Code.

Kein Bandit, keine Reward-Queue, keine mutable Weights-Arena und kein Propensity-Logging wurden angelegt.
