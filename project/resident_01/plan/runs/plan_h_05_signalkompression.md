Setze H5 um.

- keine direkte Referenzierung von Planelementen in Kommentaren
- nvcc liegt unter `/usr/local/cuda-13.3/bin/nvcc`
- Nutze Bash nicht für Retrieval Tasks

[include](/home/user/xyan/xy.ai.workbench/project/resident/index.md)

[include](/home/user/xyan/xy.ai.workbench/project/resident/plan/plan_h_00_overview.md)

[include](/home/user/xyan/xy.ai.workbench/project/resident/plan/plan_h_05_signalkompression.md)

SystemInit:  id=3a1dc9f5-d642-4403-88e2-87b686f900b4, cwd=/home/user/xyan/xy.ai.workbench/project, model=claude-sonnet-5-5
Text:
 Ich suche zuerst nach `AccessMethod`, `Scope`, `summary` und dem ADR-Ort im Code.
Control Request:
```yaml
id: 4172-78
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src
    pattern: enum (AccessMethod|Scope)\b|fn summary|fn capabilities|ScopeResolver|UnsupportedScope|non_exhaustive
    include:
    - '*.rs'
    exclude:
    - target/**
    limit: 50
  intent: editing
  reason: "Vorhandene Definitionen von AccessMethod, Scope, summary und ScopeResolver im Crate finden"
```
/answer 4172-78 allow
ReasoningToken: 25
Control Request:
```yaml
id: 4172-79
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src
      matches:
      - path: resident/segment/mod.rs
        nodes:
        - id: SlotClass
          matches:
          - "#[non_exhaustive]"
      - path: resident/segment/error.rs
        nodes:
        - id: SegmentError
          matches:
          - "#[non_exhaustive]"
      - path: resident/processor/config.rs
        nodes:
        - id: WorkerConfig
          matches:
          - "#[non_exhaustive]"
      - path: resident/processor/error.rs
        nodes:
        - id: OrchestrationError
          matches:
          - "#[non_exhaustive]"
      - path: resident/facade/resident.rs
        nodes:
        - id: ResidentConfig
          matches:
          - "#[non_exhaustive]"
        - id: Residency
          matches:
          - "#[non_exhaustive]"
        - id: IndexInfo
          matches:
          - "#[non_exhaustive]"
      - path: resident/facade/params.rs
        nodes:
        - id: TextRole
          matches:
          - "#[non_exhaustive]"
        - id: ScorerKind
          matches:
          - "#[non_exhaustive]"
        - id: TextInput
          matches:
          - "#[non_exhaustive]"
        - id: VectorInput
          matches:
          - "#[non_exhaustive]"
        - id: PairInput
          matches:
          - "#[non_exhaustive]"
        - id: EmbeddingParams
          matches:
          - "#[non_exhaustive]"
        - id: SimilarityParams
          matches:
          - "#[non_exhaustive]"
        - id: InferenceParams
          matches:
          - "#[non_exhaustive]"
        - id: Output
          matches:
          - "#[non_exhaustive]"
      - path: resident/facade/error.rs
        nodes:
        - id: TaskError
          matches:
          - "#[non_exhaustive]"
      - path: resident/facade/task.rs
        nodes:
        - id: TaskKind
          matches:
          - "#[non_exhaustive]"
        - id: Payload
          matches:
          - "#[non_exhaustive]"
      - path: resident/index/mod.rs
        nodes:
        - id: AccessMethod
          matches:
          - "#[non_exhaustive]"
          - "pub enum AccessMethod {"
        - id: Scope
          matches:
          - "pub enum Scope {"
        - id: HashAlgo
          matches:
          - "#[non_exhaustive]"
        - id: Index.capabilities
          matches:
          - "fn capabilities(&self) -> &[AccessMethod];"
        - id: Index.summary
          matches:
          - "fn summary(&self, kind: AccessMethod) -> Option<SummaryRef>;"
      - path: resident/index/parquet_io.rs
        nodes:
        - id: impl_Index_for_ParquetIndex.capabilities
          matches:
          - "fn capabilities(&self) -> &[AccessMethod] {"
        - id: impl_Index_for_ParquetIndex.summary
          matches:
          - "fn summary(&self, _kind: AccessMethod) -> Option<SummaryRef> {"
      - path: resident/index/memory.rs
        nodes:
        - id: impl_Index_for_InMemoryIndex.capabilities
          matches:
          - "fn capabilities(&self) -> &[AccessMethod] {"
        - id: impl_Index_for_InMemoryIndex.summary
          matches:
          - "fn summary(&self, _kind: AccessMethod) -> Option<SummaryRef> {"
      - path: resident/kernel/launcher.rs
        nodes:
        - id: LaunchMode
          matches:
          - "#[non_exhaustive]"
      - path: resident/kernel/error.rs
        nodes:
        - id: KernelError
          matches:
          - "#[non_exhaustive]"
      - path: resident/tokenize/error.rs
        nodes:
        - id: TokenizeError
          matches:
          - "#[non_exhaustive]"
      - path: resident/error.rs
        nodes:
        - id: ModuleDoc
          matches:
          - "//! Public enums are `#[non_exhaustive]` so new variants don't break downstream matches."
        - id: IndexError
          matches:
          - "#[non_exhaustive]"
          - "UnsupportedScope { method: String },"
        - id: ManagerError
          matches:
          - "#[non_exhaustive]"
        - id: PrecisionError
          matches:
          - "#[non_exhaustive]"
        - id: PlannerError
          matches:
          - "#[non_exhaustive]"
        - id: CapabilityError
          matches:
          - "#[non_exhaustive]"
        - id: ConfigError
          matches:
          - "#[non_exhaustive]"
        - id: ResidentError
          matches:
          - "#[non_exhaustive]"
      - path: resident/subengine/error.rs
        nodes:
        - id: SubengineError
          matches:
          - "#[non_exhaustive]"
        - id: DeviceError
          matches:
          - "#[non_exhaustive]"
      - path: resident/subengine/exec.rs
        nodes:
        - id: LaneClass
          matches:
          - "#[non_exhaustive]"
      - path: resident/manager/mod.rs
        nodes:
        - id: ModuleDoc
          matches:
          - "//! StreamedIndexManager, ScopeResolver, update queue, residency."
      - path: resident/manager/test/flat.rs
        nodes:
        - id: impl_Index_for_FlatIndex.capabilities
          matches:
          - "fn capabilities(&self) -> &[AccessMethod] {"
        - id: impl_Index_for_FlatIndex.summary
          matches:
          - "fn summary(&self, kind: AccessMethod) -> Option<SummaryRef> {"
      - path: resident/manager/test/sync.rs
        nodes:
        - id: impl_Index_for_Probe.capabilities
          matches:
          - "fn capabilities(&self) -> &[AccessMethod] {"
        - id: impl_Index_for_Probe.summary
          matches:
          - "fn summary(&self, kind: AccessMethod) -> Option<SummaryRef> {"
      - path: resident/manager/test/scope.rs
        nodes:
        - id: impl_Index_for_FakeIndex.capabilities
          matches:
          - "fn capabilities(&self) -> &[AccessMethod] {"
      warning: "Limit of 50 matches reached; further results may exist. Narrow the pattern, directory or include/exclude filters, or raise limit."
```
/answer 4172-79 allow
Control Request:
```yaml
id: 4172-7a
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/docs
    - /home/user/xyan/xy.ai.workbench/project/resident
    pattern: ADR
    include:
    - '*.md'
    exclude:
    - target/**
    limit: 30
  reason: Existierende ADR-Konvention/Ablageort für den Divergenz-Eintrag finden
```
/answer 4172-7a allow
Control Request:
```yaml
id: 4172-7b
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/docs
      - /home/user/xyan/xy.ai.workbench/project/resident
      matches:
      - path: architecture/gpu_memory.md
        matches:
        - '- `CudaMemoryManager::new` rejects a plan that exceeds `free - headroom` (`DEFAULT_HEADROOM` = 512 MiB) before allocating.'
      - path: archive/create_plan_h_q.md
        matches:
        - "- Ein kurzer ADR-Eintrag hält die Umsetzungsentscheidung fest: „Thesen-§3 Ringbuffer in Phase 1 durch Pull-Queue ersetzt; Ringbuffer = späterer Persistent-Scorer-Modus\" (Nachvollziehbarkeit der Thesen-Treue)."
        - "- Ein ADR-Eintrag dokumentiert die Divergenz: „Signal-/Frequenzkompression ist Oberschicht-Vorfilter, nicht Engine. Engine stellt nur generische Scope-/Summary-Hooks bereit.\""
        - "- Ein ADR-Eintrag dokumentiert die Divergenz: „Gewichte immutabel, nur Index bidirektional mutabel; Bandit/Online-Adaption bewusst nicht adressiert.\""
        - "- **H5/H6** sind klar als *außerhalb des Engine-Scopes* markiert (nur Nicht-Ausschluss über offene `AccessMethod`/`Scope`/Params + ADR-Divergenznotiz), passend zum Thesen-Abgleich im Metaplan."
      - path: plan/plan_h_01_gpu_ringbuffer.md
        matches:
        - "- Ein kurzer ADR-Eintrag hält die Umsetzungsentscheidung fest: „Thesen-§3 Ringbuffer in Phase 1 durch Pull-Queue ersetzt; Ringbuffer = späterer Persistent-Scorer-Modus\" (Nachvollziehbarkeit der Thesen-Treue)."
      - path: plan/plan_h_06_mutable_weights_bandit.md
        matches:
        - "- Ein ADR-Eintrag dokumentiert die Divergenz: „Gewichte immutabel, nur Index bidirektional mutabel; Bandit/Online-Adaption bewusst nicht adressiert.\""
      - path: plan/runs/plan_d_04_model_encoder.md
        matches:
        - "match: \"pub const DEFAULT_HEADROOM: u64 = 512 << 20;\""
        - "19:pub use budget::{ArenaKind, ArenaSizes, Budget, BudgetProbe, DEFAULT_HEADROOM};"
      - path: plan/runs/plan_h_01_gpu_ringbuffer.md
        matches:
        - "pattern: ADR|Ringbuffer"
        - "reason: ADR-Ablageort und bestehende Launcher-Tests finden"
        - '- ''- `CudaMemoryManager::new` rejects a plan that exceeds `free - headroom` (`DEFAULT_HEADROOM` = 512 MiB) before allocating.'''
        - "`ArenaKind`, der Evictor und `SwapManager::begin_batch` sind bereits abgedeckt; offen bleiben `LaneClass`, `LaunchMode`, ein No-GPU-Test und der ADR. Als Nächstes prüfe ich die Divergenz-Stelle in der Scope-Charta."
        - "reason: Struktur der Scope-Charta für den ADR-Eintrag und Imports der Exec-Tests prüfen"
        - '- **ADR:** Die Entscheidung "Ringbuffer in Phase 1 durch Pull-Queue ersetzt" steht in `docs/architecture/scope_charta.md` §5. Einen separaten Eintrag habe ich nicht angelegt.'
      - path: plan/runs/plan_e_03.md
        matches:
        - "match: \"pub use budget::{ArenaKind, ArenaSizes, Budget, BudgetProbe, DEFAULT_HEADROOM};\""
      - path: plan/runs/plan_h_05_signalkompression.md
        matches:
        - "Ich suche zuerst nach `AccessMethod`, `Scope`, `summary` und dem ADR-Ort im Code."
        - "pattern: ADR"
        - "reason: Existierende ADR-Konvention/Ablageort für den Divergenz-Eintrag finden"
      - path: plan/runs/plan_d_03_gpu_memory_arenas.md
        matches:
        - "pub const DEFAULT_HEADROOM: u64 = 512 << 20;"
        - "pub use budget::{ArenaKind, ArenaSizes, Budget, BudgetProbe, DEFAULT_HEADROOM};"
        - '- `CudaMemoryManager::new` rejects a plan that exceeds `free - headroom` (`DEFAULT_HEADROOM` = 512 MiB) before allocating.'
      - path: plan/runs/plan_e_08.md
        matches:
        - "- \"pub use budget::{ArenaKind, ArenaSizes, Budget, BudgetProbe, DEFAULT_HEADROOM};\""
      - path: plan/plan_h_05_signalkompression.md
        matches:
        - "- Ein ADR-Eintrag dokumentiert die Divergenz: „Signal-/Frequenzkompression ist Oberschicht-Vorfilter, nicht Engine. Engine stellt nur generische Scope-/Summary-Hooks bereit.\""
```
/answer 4172-7b allow
Control Request:
```yaml
id: 4172-7c
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/index/mod.rs
    ids:
    - AccessMethod
    - Scope
  reason: Aktuelle Definition von AccessMethod und Scope lesen
```
/answer 4172-7c allow
ReasoningToken: 52
Control Request:
```yaml
id: 4172-7d
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/index/mod.rs
      nodes:
      - id: AccessMethod
        type: ItemEnum
        code: |
          #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
          #[non_exhaustive]
          pub enum AccessMethod {
              Centroid,
              Prefix,
              Subtree,
              Range,
          }
      - id: Scope
        type: ItemEnum
        code: |
          #[derive(Clone, Debug, PartialEq)]
          pub enum Scope {
              All,
              Range(Range<u64>),
              Set(EntrySet),
              Subtree(NodeId),
              Prefix(Vec<u8>),
              Centroid { k: u32, radius: Option<f32> },
          }
```
/answer 4172-7d allow
Control Request:
```yaml
id: 4172-7e
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/docs/architecture
    pattern: ^#|Divergenz|Ringbuffer
    include:
    - scope_charta.md
    limit: 50
  intent: editing
  reason: Struktur der Scope-Charta für den Divergenz-Eintrag
```
/answer 4172-7e allow
Control Request:
```yaml
id: 4172-7f
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
          - "> Sources: `resident_planplan.md` (Kernbefund, Abgleich-Tabelle, Zentrale Divergenz,"
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
```
/answer 4172-7f allow
Control Request:
```yaml
id: 4172-80
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/docs/architecture/scope_charta.md
    ids:
    - 35617d.445827
    - 35617d.43bb35
    - 35617d.acfc01
  reason: "Abschnitte 5-7 lesen, um Divergenz-Eintrag konsistent zu ergänzen"
```
/answer 4172-80 allow
Control Request:
```yaml
id: 4172-81
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
      - id: 35617d.acfc01
        type: section
        code: |+
          ## 7. References

          - C1 (Facade API) and E2 (Scope & Resolver) **must** plan against this charta.
          - G1/G3: dense e5-small (cosine) is the standard path; MaxSim/ColBERT exists only as kernel
            infrastructure / switch path (H3/H4 adjacent), not the default.
```
/answer 4172-81 allow
Control Request:
```yaml
id: 4172-82
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src/resident
    pattern: Scope::|match .*scope|ScopeResolver
    include:
    - '*.rs'
    limit: 40
  intent: editing
  reason: "Prüfen, wo Scope exhaustiv gematcht wird und ob die Resolver-Registry pluggable ist"
```
/answer 4172-82 allow
Control Request:
```yaml
id: 4172-83
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src/resident
      matches:
      - path: facade/params.rs
        nodes:
        - id: impl_SimilarityParams.new
          matches:
          - "scope: Scope::All,"
      - path: facade/test/report.rs
        nodes:
        - id: scorer_defaults_to_cosine_and_scope_kind_is_labelled
          matches:
          - "assert_eq!(Scope::All.kind(), \"all\");"
      - path: facade/test/addon.rs
        nodes:
        - id: cascade
          matches:
          - .scope(Scope::Set(cells))
        - id: cascade_follow_up_is_a_regular_task_with_derived_scope
          matches:
          - "let Scope::Set(set) = params.get_scope() else { panic!(\"scope not derived\") };"
        - id: follow_up_equals_application_task_without_addon_marker
          matches:
          - "let scope = Scope::Set(EntrySet::from_ids([EntryId::new(1)]));"
      - path: index/mod.rs
        nodes:
        - id: impl_Scope.kind
          matches:
          - "Scope::All => \"all\","
          - "Scope::Range(_) => \"range\","
          - "Scope::Set(_) => \"set\","
          - "Scope::Subtree(_) => \"subtree\","
          - "Scope::Prefix(_) => \"prefix\","
          - "Scope::Centroid { .. } => \"centroid\","
      - path: manager/mod.rs
        nodes:
        - id: ModuleDoc
          matches:
          - "//! StreamedIndexManager, ScopeResolver, update queue, residency."
      - path: manager/test/scope.rs
        nodes:
        - id: undeclared_method_is_unsupported
          matches:
          - "let r = reg.resolve(&index, &Scope::Subtree(NodeId(1)), &ResolveCtx::default());"
        - id: declared_without_resolver_is_unsupported
          matches:
          - "let r = reg.resolve(&index, &Scope::Range(0..4), &ResolveCtx::default());"
        - id: centroid_scope_on_index_without_capability_is_unsupported
          matches:
          - "let scope = Scope::Centroid {"
        - id: centroid_scope_without_registered_resolver_is_unsupported
          matches:
          - "let scope = Scope::Centroid {"
        - id: range_subtree_prefix_resolve
          matches:
          - "let r = reg.resolve(&index, &Scope::Range(3..6), &ctx).unwrap();"
          - "let s = reg.resolve(&index, &Scope::Subtree(NodeId(7)), &ctx).unwrap();"
          - ".resolve(&index, &Scope::Prefix(vec![0, 0, 0, 0, 0, 0, 1]), &ctx)"
          - "let exact = reg.resolve(&index, &Scope::Prefix(vec![0; 8]), &ctx).unwrap();"
          - "let longer = reg.resolve(&index, &Scope::Prefix(vec![0; 9]), &ctx).unwrap();"
        - id: all_and_set_need_no_method
          matches:
          - "assert_eq!(ids(& reg.resolve(& index, & Scope::All, & ctx).unwrap()), [2, 3, 9]);"
          - "let out = reg.resolve(&index, &Scope::Set(set.clone()), &ctx).unwrap();"
        - id: centroid_unions_member_cells_and_respects_k
          matches:
          - "let scope = Scope::Centroid {"
        - id: centroid_without_summary_or_selector_fails_loudly
          matches:
          - "let scope = Scope::Centroid {"
      - path: manager/test/streamed.rs
        nodes:
        - id: cycle_without_an_index_is_unavailable
          matches:
          - "matches!(m.resolve(& Scope::All, & ResolveCtx::default()),"
      - path: manager/scope.rs
        nodes:
        - id: ScopeResolver
          matches:
          - "pub trait ScopeResolver: Send + Sync {"
        - id: ResolverRegistry
          matches:
          - "map: HashMap<AccessMethod, Arc<dyn ScopeResolver>>,"
        - id: impl_ResolverRegistry.with_defaults
          matches:
          - "Arc::new(RangeResolver) as Arc<dyn ScopeResolver>,"
        - id: impl_ResolverRegistry.register
          matches:
          - "resolver: Arc<dyn ScopeResolver>,"
        - id: impl_ResolverRegistry.resolve
          matches:
          - "let method = match required_method(scope) {"
        - id: required_method
          matches:
          - "match scope {"
          - "Scope::All | Scope::Set(_) => None,"
          - "Scope::Range(_) => Some(AccessMethod::Range),"
          - "Scope::Subtree(_) => Some(AccessMethod::Subtree),"
          - "Scope::Prefix(_) => Some(AccessMethod::Prefix),"
          - "Scope::Centroid { .. } => Some(AccessMethod::Centroid),"
        - id: resolve_direct
          matches:
          - "match scope {"
          - "Scope::Set(set) => set.clone(),"
        - id: impl_ScopeResolver_for_RangeResolver
          matches:
          - "impl ScopeResolver for RangeResolver {"
      warning: "Limit of 40 matches reached; further results may exist. Narrow the pattern, directory or include/exclude filters, or raise limit."
```
/answer 4172-83 allow
Control Request:
```yaml
id: 4172-84
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/index/mod.rs
    id: Scope
    old_text: |-
      #[derive(Clone, Debug, PartialEq)]
      pub enum Scope {
    new_text: |-
      #[derive(Clone, Debug, PartialEq)]
      #[non_exhaustive]
      pub enum Scope {
  reason: "Scope als non_exhaustive markieren, damit spätere Varianten ohne Bruch ergänzbar sind"
```
/answer 4172-84 allow
ReasoningToken: 106
Control Request:
```yaml
id: 4172-85
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/rag/docs/architecture/scope_charta.md
  id: 35617d.43bb35
  reason: Vollständigen Inhalt von Abschnitt 6 (Hook-Tabelle) lesen
```
/answer 4172-85 allow
Control Request:
```yaml
id: 4172-86
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/rag/docs/architecture/scope_charta.md
      nodes:
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
```
/answer 4172-86 allow
Control Request:
```yaml
id: 4172-87
toolName: ast_insert
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/docs/architecture/scope_charta.md
    id: 35617d.445827
    position: after
    source: |+
      ### 5.1 Divergence: Signal/Frequency Compression (§6/§6.1, H5)

      **Thesis §6/§6.1** sketches frequency/signal compression (DCT/FFT, wavelet, "semantic
      spectrogram") as a compression/quality layer.

      **Decision:** signal/frequency compression is a search-layer prefilter, not engine. The engine
      only provides generic hooks: `AccessMethod` and `Scope` are `#[non_exhaustive]`,
      `Index::summary(kind)` returns arbitrary summary structures, and the `ResolverRegistry` accepts
      additional `ScopeResolver`s (unsupported combinations fail loudly with `UnsupportedScope`). A
      candidate set from a prefilter is already expressible as `Scope::Set`/`Scope::Range`. Recall
      loss of approximate prefilters belongs to the search layer. No FFT/DCT/wavelet, quantization or
      prefilter code lives in the engine.

  reason: ADR-Divergenzeintrag zu H5 in der Scope-Charta ergänzen
```
/answer 4172-87 allow
ReasoningToken: 66
Control Request:
```yaml
id: 4172-88
toolName: bash
arguments:
  items:
  - cwd: /home/user/xyan/xy.ai.workbench/rag
    script:
    - cargo check --all-targets 2>&1 | tail -15
  reason: Kompilierung nach non_exhaustive-Änderung verifizieren (externe Matches in tests/benches/bin)
```
/answer 4172-88 allow
Control Request:
```yaml
id: 4172-89
result:
  structuredContent:
    results:
    - stdout: |2

        warning: `xy-ai-rag` (lib) generated 4 warnings
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

        warning: `xy-ai-rag` (lib test) generated 2 warnings (1 duplicate)
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 5.90s
```
/answer 4172-89 allow
Result Stats: id=3a1dc9f5-d642-4403-88e2-87b686f900b4, total: 21303, in: 12, out: 3254, reason: 0, read: 89620, write: 18037, start: 1793048785176, end: 1793048793428, time: 8182, total: 45817, io: 0, control: 70, sent: 0, recv: 119
H5 ist umgesetzt, und `cargo check --all-targets` läuft durch. Die Warnungen sind nur Dead-Code-Hinweise, die nichts mit dieser Änderung zu tun haben. Ich habe keine Tests ausgeführt.

- **Code:** `Scope` in `rag/src/resident/index/mod.rs` trägt jetzt `#[non_exhaustive]`. `AccessMethod` hatte das Attribut schon. Neue Varianten wie eine spätere `CompressionDomain` oder `Frequency` brechen damit keine externen Matches.
- **Schon vorhanden:** `Index::summary(kind) -> Option<SummaryRef>` ist generisch. Die `ResolverRegistry` in `manager/scope.rs` nimmt weitere `ScopeResolver` per `register` entgegen. Für nicht unterstützte Kombinationen liefert sie `UnsupportedScope`. Dafür war keine Änderung nötig.
- **ADR:** Die Divergenz steht als neuer Abschnitt „5.1 Divergence: Signal/Frequency Compression" in `rag/docs/architecture/scope_charta.md`. Das ist dieselbe Stelle, an der die Ringbuffer-Entscheidung dokumentiert ist, und nicht ein separates ADR-Dokument. Der Text sagt, dass Signal-/Frequenzkompression ein Vorfilter der Suchschicht ist und die Engine nur die generischen Scope- und Summary-Hooks bereitstellt.
- **Nicht umgesetzt:** FFT, DCT, Wavelet, Quantisierung und ein Vorfilter in der Engine, wie im Plan vorgesehen.
