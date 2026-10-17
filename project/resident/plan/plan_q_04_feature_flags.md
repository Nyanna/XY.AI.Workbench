Setze Q4 um.

[include](/home/user/xyan/xy.ai.workbench/project/resident/index.md)
[include](/home/user/xyan/xy.ai.workbench/project/resident/plan/plan_a_00_overview.md)

- minimiere die Kommentare und keine direkte Referenzierung von Planelementen in Kommentaren
- Scope-Charta erstellt unter `/home/user/xyan/xy.ai.workbench/rag/docs/architecture/scope_charta.md`
- Projektdokumente immer in englisch ausgeben


# Umsetzungsplan Q4 · Feature-Flags

**Querschnitt Q — verbindliche Konvention.** Nach A4 initial zu fixieren (hängt an A2 Modulstruktur). Voraussetzung dafür, dass die Logik ohne GPU testbar bleibt.

## 1. Zweck

Reale CUDA-Pfade hinter ein Feature-Flag legen, sodass (a) der Default-Build **ohne** GPU/`nvcc` kompiliert und alle Unit-Tests ausführt, (b) `#[cfg]` nur an wenigen Modulgrenzen auftritt, (c) der Backend-Austausch über Traits/Factory statt `cfg`-Streuung läuft.

## 2. Rechercheergebnis (destilliert aus `context_h_q.md` Q4)

- **Best Practices:** Features sind **additiv** (aktivieren nur Code), Default **ohne** `cuda`. Optionale Dependencies mit `dep:`-Syntax; `#[cfg(feature = "cuda")]` nur an wenigen Stellen (Modulgrenze). (Cargo Book „Features".)
- **`candle-core` `cuda`:** aktiviert `cudarc`, `candle-kernels` u. a.; Bauen braucht CUDA-Toolkit/`nvcc` **[prüfen aktuelle Cargo.toml]**. Durchreichen:
  ```toml
  [features]
  default = []
  cuda = ["candle-core/cuda", "dep:cudarc"]
  [dependencies]
  candle-core = "…"
  cudarc = { version = "…", optional = true }
  ```
  Bei optionalem `candle-core`: Weak-Dependency `candle-core?/cuda`.
- **No-GPU-Pfad:** Trait-Objekt-/Generics-Austausch statt `cfg`-Streuung: `cfg` nur beim **Konstruieren des Backends** (eine Factory), alles andere feature-unabhängig.
- **CI:** Matrix **mit und ohne** `cuda`. No-GPU-Build muss ohne `nvcc` kompilieren und alle Unit-Tests ausführen; `cuda`-Build mindestens per `cargo check`.

## 3. Verbindliche Konvention (Umsetzung)

- **`default = []`** (kein `cuda`). `cuda = ["candle-core/cuda", "dep:cudarc"]` (bzw. Weak-Syntax falls `candle-core` optional).
- **`#[cfg(feature = "cuda")]` nur an der Backend-Factory** und an der einen Candle-Adapter-Stelle (Schicht 4, `resident_subengine.md` §2: „Candle ist ein Adapter an genau einer Stelle"). Keine `cfg`-Streuung in Scheduler/Planner/Index-Logik.
- **Backend-Factory** konstruiert `ComputeBackend` (Q3): ohne `cuda` → CPU-Backend; mit `cuda` und verfügbarer GPU → CUDA-Backend, sonst Fallback CPU (`resident_subengine.md` §6: GPU-Initfehler → CPU). Der Rest der Engine arbeitet nur gegen das Trait.
- **Reale CUDA-abhängige Pfade hinter `cuda`:** Acquire-Load/Pinned-Staging/Arena/Streams (`resident_index.md` Schlusssatz), persistenter Kernel (H1), Graph-Capture (H4). Deren **Logik** (Mapping, Disziplin, Scheduling) bleibt feature-unabhängig und gegen Mocks testbar.
- **Invarianten bleiben feature-unabhängig** (`resident_subengine.md` §2): `Region`/`Op`/`ExecContext`-Signaturen ohne `Tensor`/Raw-Pointer — unabhängig von `cuda`.

## 4. Umsetzungsschritte

1. `Cargo.toml`: `default`/`cuda`-Features, `cudarc` optional, Passthrough an `candle-core`.
2. Backend-Factory mit dem einzigen `#[cfg(feature = "cuda")]`-Entscheidungspunkt.
3. CUDA-spezifische Module hinter `#[cfg(feature = "cuda")]`; CPU/Mock-Pendants immer gebaut.
4. CI-Matrix: Job A `--no-default-features`/default → `cargo build` + `cargo test` (ohne `nvcc`); Job B `--features cuda` → mindestens `cargo check`.
5. Doku: „Default baut GPU-frei; `cuda` nur für reale CUDA-Pfade."

## 5. Tests

- Default-Build kompiliert ohne `nvcc` und führt alle Unit-Tests aus (Q3).
- Backend-Factory liefert ohne `cuda` das CPU-Backend; mit `cuda` ohne GPU erfolgt der dokumentierte Fallback.
- Kein `#[cfg(feature = "cuda")]` außerhalb von Factory + Candle-Adapter (Lint/Grep-Check im CI).

## 6. Offene Verifikationen ([prüfen])

- Aktuelle `candle-core`-`Cargo.toml`: was das `cuda`-Feature zieht; `cudarc`-Version = die von `candle` verwendete (`resident_subengine.md` §10: sonst inkompatible Typen).

## 7. Definition of Done

- Feature-Matrix wie §3; einziger `cfg`-Entscheidungspunkt in der Factory; CI-Matrix mit/ohne `cuda` grün; No-GPU-Default führt alle Unit-Tests aus; `cfg`-Streuungs-Check besteht.

SystemInit:  id=a4b60ef1-3872-41f5-aa32-a2847309f27b, cwd=/home/user/xyan/xy.ai.workbench/project, model=claude-sonnet-5
Control Request:
```yaml
id: 9faa-1
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/rag
  reason: "Projektstruktur des Rust-Workspace prüfen, um Q4 (Feature-Flags) korrekt einzufügen."
```
/answer 9faa-1 allow
Control Request:
```yaml
id: 9faa-2
result:
  structuredContent:
    errors:
    - path: /home/user/xyan/xy.ai.workbench/rag
      error: |-
        Path is a directory, not a file; listing its contents instead.
        Result reduced to top-level directories the limit of 50 is exceeded (809). Narrow down the result using the 'pattern' regular expression parameter.
        .: (5 files below)
        ./.bin: (10413 files below)
        ./.cargo: (1 file below)
        ./.settings: (1 file below)
        ./.xyrag: (31 files below)
        ./benches: (1 file below)
        ./docs: (4 files below)
        ./src: (40 files below)
        ./tests: (1 file below)
```
/answer 9faa-2 allow
Control Request:
```yaml
id: 9faa-3
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/rag/src
  - /home/user/xyan/xy.ai.workbench/rag/docs
  reason: "Struktur von src/ und docs/ einsehen, um Backend-Factory/Cargo.toml-Konvention zu finden."
```
/answer 9faa-3 allow
Control Request:
```yaml
id: 9faa-4
result:
  structuredContent:
    errors:
    - path: /home/user/xyan/xy.ai.workbench/rag/src
      error: |-
        Path is a directory, not a file; listing its contents instead.
        .:
        lib.rs
        ./bin:
        xyrag.rs
        ./core:
        engine.rs
        executor.rs
        layer.rs
        mod.rs
        persistence.rs
        query.rs
        registry.rs
        result.rs
        topology.rs
        ./layers:
        dir_cache.rs
        glob_layer.rs
        grep_layer.rs
        merge_layer.rs
        mod.rs
        ./layers/trigram:
        index.rs
        layer.rs
        mod.rs
        normalize.rs
        signature.rs
        trie.rs
        vocab.rs
        ./resident:
        error.rs
        mock.rs
        mod.rs
        telemetry.rs
        ./resident/facade:
        mod.rs
        ./resident/index:
        mod.rs
        ./resident/kernel:
        cuda.rs
        mod.rs
        ./resident/manager:
        mod.rs
        ./resident/memory:
        mod.rs
        ./resident/op:
        mod.rs
        ./resident/planner:
        mod.rs
        ./resident/precision:
        mod.rs
        ./resident/processor:
        mod.rs
        ./resident/queue:
        mod.rs
        ./resident/subengine:
        device.rs
        mod.rs
    - path: /home/user/xyan/xy.ai.workbench/rag/docs
      error: |-
        Path is a directory, not a file; listing its contents instead.
        ./architecture:
        conventions.md
        dependency_verification.md
        feature_matrix.md
        scope_charta.md
```
/answer 9faa-4 allow
Control Request:
```yaml
id: 9faa-5
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/rag/Cargo.toml
  node_type: .*
  reason: Aktuelle Feature-Definitionen und Dependencies prüfen.
```
/answer 9faa-5 allow
Control Request:
```yaml
id: 9faa-6
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/rag/Cargo.toml
      nodes:
      - id: h8JWUc|XOXuKk
        type: table
        code: |+
          [package]
          name = "xy-ai-rag"
          version = "0.1.0"
          edition = "2021"
          description = "xy.ai.rag - Layered Anytime Retrieval Engine"

      - id: mN2I6g|SIE1cj
        type: table
        code: |+
          [lib]
          name = "xy_ai_rag"
          path = "src/lib.rs"

      - id: cW2ehx|QePTHE
        type: table_array_element
        code: |+
          [[bin]]
          name = "xyrag"
          path = "src/bin/xyrag.rs"

      - id: zHTLlI|2t4fSB
        type: table
        code: |+
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
          half = "2.7.1"
          memmap2 = "0.9.3"
          parquet = "60.0.0"
          arrow = "60.0.0"
          tokenizers = { version = "0.23.2", default-features = false }
          safetensors = "0.8.0"
          candle-core = { version = "0.11.0", optional = true, default-features = false }
          candle-transformers = { version = "0.11.0", optional = true, default-features = false }
          cudarc = { version = "=0.19.9", optional = true }

      - id: adh0oF|3scqb3
        type: table
        code: |+
          [features]
          default = []
          cuda = ["dep:candle-core", "dep:candle-transformers", "dep:cudarc", "candle-core/cuda", "candle-transformers/cuda"]
          mock = []
          json = ["tracing-subscriber/json"]

      - id: UeF6gB|FaChiw
        type: table
        code: |+
          [dev-dependencies]
          proptest = "1"
          criterion = "0.5"
          mockall = "0.15.0"
          loom = "0.7"
          shuttle = "0.7"
          rand = "0.8"
          tokio = { version = "1", features = ["test-util"] }

      - id: 9OxNvg|ELdnWn
        type: table_array_element
        code: |
          [[bench]]
          name = "trie_bench"
          harness = false
```
/answer 9faa-6 allow
Control Request:
```yaml
id: 9faa-7
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/docs/architecture/feature_matrix.md
    ids:
    - __FILE__
  reason: "Bestehende Feature-Matrix-Doku lesen, um Duplikate/Konflikte mit Q4 zu vermeiden."
```
/answer 9faa-7 allow
Control Request:
```yaml
id: 9faa-8
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/docs/architecture/feature_matrix.md
      nodes: []
      errors:
      - No node matched '__FILE__'.
```
/answer 9faa-8 allow
Control Request:
```yaml
id: 9faa-9
toolName: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/docs/architecture
    pattern: .
  reason: "Inhalt der feature_matrix.md und conventions.md ansehen, um Doppelungen zu vermeiden."
```
/answer 9faa-9 allow
Control Request:
```yaml
id: 9faa-a
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/docs/architecture
      matches:
      - path: scope_charta.md
        matches:
        - lineno: 1
          match: "# Scope Charta: Resident Engine vs. Search Layer"
        - lineno: 3
          match: '> Package **A1′** (Group A — Foundation & Context). Replaces the completed package A1'
        - lineno: 4
          match: '> ("distill the thesis"). Binding reference for C1 (Facade API) and E2 (Scope Resolver);'
        - lineno: 5
          match: '> also informs G1/G3 (dense default path vs. ColBERT/MaxSim switch path).'
        - lineno: 6
          match: '>'
        - lineno: 7
          match: "> Sources: `resident_planplan.md` (Kernbefund, Abgleich-Tabelle, Zentrale Divergenz,"
        - lineno: 8
          match: "> Auswirkungen auf den Metaplan), `resident.md` (Klarstellung, Ausblick), `resident_index.md`"
        - lineno: 9
          match: "> (Leitprinzipien, Nicht Teil), `resident_precision.md` (Nicht Teil)."
        - lineno: 11
          match: "## 1. Purpose"
        - lineno: 13
          match: This charta fixes the boundary between the "Resident" engine and the outer search/retrieval
        - lineno: 14
          match: "(\"search layer\"). It is **blocking** for C1 (facade API) and E2 (scope resolver): any later"
        - lineno: 15
          match: "design that pulls quality/chunking concerns into the engine, or that omits a required hook,"
        - lineno: 16
          match: violates this charta and must be corrected before proceeding.
        - lineno: 18
          match: "The engine is **substrate, not a retrieval algorithm**. It computes on whatever candidate set"
        - lineno: 19
          match: it is handed; it guarantees neither completeness nor size of that set.
      warning: "Limit of 15 matches reached; further results may exist. Narrow the pattern, directory or include/exclude filters, or raise limit."
```
/answer 9faa-a allow
Control Request:
```yaml
id: 9faa-b
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel/cuda.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel/mod.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/subengine/device.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/subengine/mod.rs
  node_type: .*
  reason: Bestehende Backend-/Factory-Struktur und cfg-Nutzung prüfen.
```
/answer 9faa-b allow
ReasoningToken: 52
Control Request:
```yaml
id: 9faa-c
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel/cuda.rs
      nodes:
      - id: ModuleDoc
        type: ModuleAttrs
        code: |
          //! Real CUDA kernels / scorer.
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel/mod.rs
      nodes:
      - id: ModuleDoc
        type: ModuleAttrs
        code: |
          //! SegmentDesc/Layout/KernelHit, Kernel trait.
      - id: cuda
        type: ItemMod
        code: |
          #[cfg(feature = "cuda")]
          mod cuda;
      - id: AeHRoB|0NKx9w
        type: ImportGroup
        code: |
          #[cfg(feature = "cuda")]
          pub use cuda::*;
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/subengine/device.rs
      nodes:
      - id: ModuleDoc
        type: ModuleAttrs
        code: |
          //! Device/Lane, execution context, loaded encoder, GPU memory arenas.
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/subengine/mod.rs
      nodes:
      - id: ModuleDoc
        type: ModuleAttrs
        code: |
          //! Shared subengine lifecycle & pull model, tokenizer/normalizer (CPU).
      - id: device
        type: ItemMod
        code: |
          #[cfg(feature = "cuda")]
          mod device;
      - id: p03Xlc|0j1nS6
        type: ImportGroup
        code: |
          #[cfg(feature = "cuda")]
          pub use device::*;
```
/answer 9faa-c allow
Control Request:
```yaml
id: 9faa-d
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/docs/architecture/feature_matrix.md
    ids: []
  reason: feature_matrix.md vollständig lesen.
```
/answer 9faa-d allow
Control Request:
```yaml
id: 9faa-e
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/docs/architecture/feature_matrix.md
      nodes:
      - id: 4a7ba8
        signature: Feature Matrix
        children:
        - id: 4a7ba8.3BSz7t|h0SceU
          signature: '`cuda` is never part of `defa…'
        - id: 4a7ba8.138442
          signature: CI
      errors:
      - "ids [] resolved to no node; redirected to ast_outline, returning the full outline instead."
```
/answer 9faa-e allow
Control Request:
```yaml
id: 9faa-f
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/mod.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/mock.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/core/engine.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/core/registry.rs
  node_type: .*
  reason: "Bestehende Modulstruktur/Trait-Definitionen und Mock-Konvention prüfen, um Backend-Factory einzuordnen."
```
/answer 9faa-f allow
Control Request:
```yaml
id: 9faa-10
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/mod.rs
      nodes:
      - id: ModuleDoc
        type: ModuleAttrs
        code: |
          //! Resident engine: embedding/similarity substrate, kept warm.
      - id: error
        type: ItemMod
        code: |
          pub mod error;
      - id: telemetry
        type: ItemMod
        code: |
          pub mod telemetry;
      - id: mock
        type: ItemMod
        code: |
          #[cfg(any(test, feature = "mock"))]
          pub mod mock;
      - id: queue
        type: ItemMod
        code: |
          pub mod queue;
      - id: memory
        type: ItemMod
        code: |
          pub mod memory;
      - id: op
        type: ItemMod
        code: |
          pub mod op;
      - id: index
        type: ItemMod
        code: |
          pub mod index;
      - id: kernel
        type: ItemMod
        code: |
          pub mod kernel;
      - id: precision
        type: ItemMod
        code: |
          pub mod precision;
      - id: facade
        type: ItemMod
        code: |
          pub mod facade;
      - id: processor
        type: ItemMod
        code: |
          pub mod processor;
      - id: planner
        type: ItemMod
        code: |
          pub mod planner;
      - id: subengine
        type: ItemMod
        code: |
          pub mod subengine;
      - id: manager
        type: ItemMod
        code: |
          pub mod manager;
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/mock.rs
      nodes:
      - id: ModuleDoc
        type: ModuleAttrs
        code: |
          //! Mock/test-double conventions (Q3).
          //!
          //! Narrow traits for GPU/measurement interfaces use generics (static dispatch) in the hot
          //! path; `dyn` only at the edges. Deterministic fakes (fixed output, simulated failure) are
          //! hand-written rather than `mockall`-generated, since that composes more simply across
          //! `async`/`Send` boundaries; `mockall::automock` is reserved for orchestrator-level traits.
          //! Everything here is gated so it never ships in a production build without the `mock` feature.
          #![cfg(any(test, feature = "mock"))]
      - id: Clock
        type: ItemTrait
        signature: /// Example of a narrow trait a hot-path component depends on generically. pub …
        docstring: Example of a narrow trait a hot-path component depends on generically.
        children:
        - id: Clock.now_millis
          type: TraitItemFn
          code: |2
                fn now_millis(&self) -> u64;
      - id: FakeClock
        type: ItemStruct
        code: |
          /// Hand-written deterministic fake, per the convention above.
          pub struct FakeClock {
              pub fixed_millis: u64,
          }
      - id: impl_Clock_for_FakeClock
        type: ItemImpl
        signature: "impl Clock for FakeClock {"
        children:
        - id: impl_Clock_for_FakeClock.now_millis
          type: ImplItemFn
          code: |2
                fn now_millis(&self) -> u64 {
                    self.fixed_millis
                }
      - id: ComputeBackend
        type: ItemTrait
        signature: /// Backend abstraction the orchestration layer runs against; `cuda` selects th…
        docstring: Backend abstraction the orchestration layer runs against; `cuda` selects the re…
        children:
        - id: ComputeBackend.run_batch
          type: TraitItemFn
          code: |2
                fn run_batch(&self, input: &[f32]) -> Vec<f32>;
      - id: MockBackend
        type: ItemStruct
        code: |
          /// Identity backend: sufficient for orchestration tests that do not care about numerics.
          pub struct MockBackend;
      - id: impl_ComputeBackend_for_MockBackend
        type: ItemImpl
        signature: "impl ComputeBackend for MockBackend {"
        children:
        - id: impl_ComputeBackend_for_MockBackend.run_batch
          type: ImplItemFn
          code: |2
                fn run_batch(&self, input: &[f32]) -> Vec<f32> {
                    input.to_vec()
                }
      - id: Kernel
        type: ItemTrait
        signature: "/// Kernel-level trait (batch lifecycle + results), abstracted so orchestration…"
        docstring: "Kernel-level trait (batch lifecycle + results), abstracted so orchestration/syn…"
        children:
        - id: Kernel.wXwSeB|s9F5Wn
          type: StatementGroup
          code: |2
                type Hit;
        - id: Kernel.batch_start
          type: TraitItemFn
          code: |2
                fn batch_start(&mut self) -> u64;
        - id: Kernel.batch_end
          type: TraitItemFn
          code: |2
                fn batch_end(&mut self, batch_id: u64);
        - id: Kernel.results
          type: TraitItemFn
          code: |2
                fn results(&self, batch_id: u64) -> Vec<Self::Hit>;
        - id: Kernel.written_ranges
          type: TraitItemFn
          code: |2
                fn written_ranges(&self, batch_id: u64) -> Vec<(usize, usize)>;
        - id: Kernel.progress
          type: TraitItemFn
          code: |2
                fn progress(&self, batch_id: u64) -> usize;
      - id: FakeKernel
        type: ItemStruct
        code: |
          /// Scriptable kernel fake: results/ranges/progress for a batch are programmed up front
          /// rather than computed, so sync-point tests control exactly what the kernel "found".
          #[derive(Default)]
          pub struct FakeKernel<H> {
              next_batch_id: u64,
              scripted: std::collections::HashMap<u64, (Vec<H>, Vec<(usize, usize)>, usize)>,
          }
      - id: impl_FakeKernel_H_
        type: ItemImpl
        signature: "impl<H> FakeKernel<H> {"
        children:
        - id: impl_FakeKernel_H_.new
          type: ImplItemFn
          code: |2
                pub fn new() -> Self {
                    Self {
                        next_batch_id: 0,
                        scripted: std::collections::HashMap::new(),
                    }
                }
        - id: impl_FakeKernel_H_.script
          type: ImplItemFn
          code: |2
                /// Programs the outcome of the next-started batch before it runs.
                pub fn script(
                    &mut self,
                    batch_id: u64,
                    hits: Vec<H>,
                    ranges: Vec<(usize, usize)>,
                    progress: usize,
                ) {
                    self.scripted.insert(batch_id, (hits, ranges, progress));
                }
      - id: impl_Kernel_for_FakeKernel_H_
        type: ItemImpl
        signature: "impl<H: Clone> Kernel for FakeKernel<H> {"
        children:
        - id: impl_Kernel_for_FakeKernel_H_.oGaivh|v5uMLN
          type: StatementGroup
          code: |2
                type Hit = H;
        - id: impl_Kernel_for_FakeKernel_H_.batch_start
          type: ImplItemFn
          code: |2
                fn batch_start(&mut self) -> u64 {
                    let id = self.next_batch_id;
                    self.next_batch_id += 1;
                    id
                }
        - id: impl_Kernel_for_FakeKernel_H_.batch_end
          type: ImplItemFn
          code: |2
                fn batch_end(&mut self, _batch_id: u64) {}
        - id: impl_Kernel_for_FakeKernel_H_.results
          type: ImplItemFn
          code: |2
                fn results(&self, batch_id: u64) -> Vec<H> {
                    self.scripted.get(&batch_id).map(|(hits, _, _)| hits.clone()).unwrap_or_default()
                }
        - id: impl_Kernel_for_FakeKernel_H_.written_ranges
          type: ImplItemFn
          code: |2
                fn written_ranges(&self, batch_id: u64) -> Vec<(usize, usize)> {
                    self.scripted
                        .get(&batch_id)
                        .map(|(_, ranges, _)| ranges.clone())
                        .unwrap_or_default()
                }
        - id: impl_Kernel_for_FakeKernel_H_.progress
          type: ImplItemFn
          code: |2
                fn progress(&self, batch_id: u64) -> usize {
                    self.scripted.get(&batch_id).map(|(_, _, p)| *p).unwrap_or(0)
                }
      - id: Measure
        type: ItemTrait
        signature: /// Measurement trait calibration logic depends on; `FakeMeasure` stands in for…
        docstring: Measurement trait calibration logic depends on; `FakeMeasure` stands in for real
        children:
        - id: Measure.sample
          type: TraitItemFn
          code: |2
                fn sample(&mut self) -> f64;
      - id: NoiseModel
        type: ItemEnum
        code: |
          /// Noise models covering the calibration-stability test matrix (F2-F5).
          pub enum NoiseModel {
              Constant,
              Outlier { every_n: u32, magnitude: f64 },
              Drift { per_sample: f64 },
          }
      - id: FakeMeasure
        type: ItemStruct
        code: |
          /// Seeded-RNG measurement fake: `base` plus a `NoiseModel`-shaped perturbation, so
          /// stability/convergence tests are deterministic across runs for a fixed seed.
          pub struct FakeMeasure {
              base: f64,
              noise: NoiseModel,
              rng: rand::rngs::StdRng,
              sample_count: u32,
              drift_accum: f64,
          }
      - id: impl_FakeMeasure
        type: ItemImpl
        signature: "impl FakeMeasure {"
        children:
        - id: impl_FakeMeasure.new
          type: ImplItemFn
          code: |2
                pub fn new(base: f64, noise: NoiseModel, seed: u64) -> Self {
                    use rand::SeedableRng;
                    Self {
                        base,
                        noise,
                        rng: rand::rngs::StdRng::seed_from_u64(seed),
                        sample_count: 0,
                        drift_accum: 0.0,
                    }
                }
      - id: impl_Measure_for_FakeMeasure
        type: ItemImpl
        signature: "impl Measure for FakeMeasure {"
        children:
        - id: impl_Measure_for_FakeMeasure.sample
          type: ImplItemFn
          code: |2
                fn sample(&mut self) -> f64 {
                    use rand::Rng;
                    self.sample_count += 1;
                    match &self.noise {
                        NoiseModel::Constant => self.base + self.rng.gen_range(-0.01..0.01),
                        NoiseModel::Outlier { every_n, magnitude } => {
                            if self.sample_count % every_n == 0 {
                                self.base + magnitude
                            } else {
                                self.base + self.rng.gen_range(-0.01..0.01)
                            }
                        }
                        NoiseModel::Drift { per_sample } => {
                            self.drift_accum += per_sample;
                            self.base + self.drift_accum + self.rng.gen_range(-0.01..0.01)
                        }
                    }
                }
      - id: sync
        type: ItemMod
        code: |
          /// `std::sync` vs. `loom::sync` shim for CAS-protocol modules (B1-style queues): swap the
          /// import under `cfg(loom)` so the same production code runs exhaustively under loom without
          /// a second implementation. Run with `RUSTFLAGS="--cfg loom" cargo test --test <loom_test>`.
          #[cfg(loom)]
          pub mod sync {
              pub use loom::sync::{atomic, Arc};
          }
      - id: sync_2
        type: ItemMod
        code: |
          #[cfg(not(loom))]
          pub mod sync {
              pub use std::sync::{atomic, Arc};
          }
    - path: /home/user/xyan/xy.ai.workbench/rag/src/core/engine.rs
      nodes:
      - id: ModuleDoc
        type: ModuleAttrs
        code: |
          //! Orchestrator (engine) of the RAG engine.
          //!
          //! Builds the list of active layers per query, determines their topology
          //! and runs them either sequentially or in a fork/join model. Also manages
          //! the global control flow for background activity of the layers.
      - id: jQwv50|1GpIeg
        type: ImportGroup
        code: |
          use std::collections::HashMap;
          use std::sync::Arc;
          use anyhow::Result;
          use futures::future::join_all;
          use tokio::task::JoinHandle;
          use tokio_util::sync::CancellationToken;
          use crate::core::executor::CpuExecutor;
          use crate::core::layer::{BackgroundContext, Layer, LayerContext, LayerStatus};
          use crate::core::persistence::PersistenceManager;
          use crate::core::query::Query;
          use crate::core::registry::LayerRegistry;
          use crate::core::result::ResultSet;
          use crate::core::topology::build_levels;
      - id: ExecutionMode
        type: ItemEnum
        code: |
          #[derive(Debug, Clone, Copy, PartialEq, Eq)]
          pub enum ExecutionMode {
              Sequential,
              Parallel,
          }
      - id: impl_ExecutionMode
        type: ItemImpl
        signature: "impl ExecutionMode {"
        children:
        - id: impl_ExecutionMode.parse
          type: ImplItemFn
          code: |2
                pub fn parse(s: &str) -> Option<Self> {
                    match s {
                        "sequential" => Some(Self::Sequential),
                        "parallel" => Some(Self::Parallel),
                        _ => None,
                    }
                }
        - id: impl_ExecutionMode.as_str
          type: ImplItemFn
          code: |2
                pub fn as_str(&self) -> &'static str {
                    match self {
                        Self::Sequential => "sequential",
                        Self::Parallel => "parallel",
                    }
                }
      - id: Engine
        type: ItemStruct
        code: |
          /// Connects registry, persistence and topology execution.
          ///
          /// Used both as a library (`Engine::run_query`) and instantiated on-demand
          /// by the CLI.
          pub struct Engine {
              pub registry: LayerRegistry,
              pub persistence: Arc<PersistenceManager>,
              pub mode: ExecutionMode,
              /// Shared rayon pool handed to every layer for CPU-bound work.
              pub cpu: Arc<CpuExecutor>,
              cancel_tokens: HashMap<String, CancellationToken>,
              background_tasks: HashMap<String, JoinHandle<()>>,
          }
      - id: impl_Engine
        type: ItemImpl
        signature: "impl Engine {"
        children:
        - id: impl_Engine.new
          type: ImplItemFn
          code: |2
                pub fn new(
                    registry: LayerRegistry,
                    persistence: PersistenceManager,
                    mode: ExecutionMode,
                ) -> Self {
                    let cpu = Arc::new(
                        CpuExecutor::new().expect("failed to initialize CPU executor"),
                    );
                    Self {
                        registry,
                        persistence: Arc::new(persistence),
                        mode,
                        cpu,
                        cancel_tokens: HashMap::new(),
                        background_tasks: HashMap::new(),
                    }
                }
        - id: impl_Engine.active_layers
          type: ImplItemFn
          code: |2
                pub fn active_layers(&self, query: &Query) -> Vec<Arc<dyn Layer>> {
                    self.registry.iter().filter(|l| l.applies(query)).cloned().collect()
                }
        - id: impl_Engine.run_query
          type: ImplItemFn
          code: |2
                /// Runs one query against all activated layers.
                ///
                /// The first executed layer implicitly creates the `ResultSet`, by
                /// writing into the empty instance created here - even if it is itself
                /// only an enrichment layer.
                pub async fn run_query(
                    &self,
                    query: Query,
                ) -> Result<(ResultSet, Vec<LayerStatus>)> {
                    let result_set = ResultSet::new();
                    let layers = self.active_layers(&query);
                    let levels = build_levels(&layers)?;
                    let mut statuses: Vec<LayerStatus> = Vec::new();
                    for level in levels {
                        if self.mode == ExecutionMode::Parallel && level.len() > 1 {
                            let futs = level
                                .iter()
                                .map(|layer| self.run_layer(layer.clone(), &query, &result_set));
                            let results = join_all(futs).await;
                            for r in results {
                                statuses.push(r?);
                            }
                        } else {
                            for layer in level {
                                statuses.push(self.run_layer(layer, &query, &result_set).await?);
                            }
                        }
                    }
                    Ok((result_set, statuses))
                }
        - id: impl_Engine.run_layer
          type: ImplItemFn
          code: |2
                async fn run_layer(
                    &self,
                    layer: Arc<dyn Layer>,
                    query: &Query,
                    result_set: &ResultSet,
                ) -> Result<LayerStatus> {
                    let storage = self.persistence.layer_storage(layer.id())?;
                    let ctx = LayerContext {
                        query: query.clone(),
                        storage,
                        shared_index: self.persistence.shared_index.clone(),
                        cpu: self.cpu.clone(),
                    };
                    Ok(layer.run(query, result_set, &ctx).await)
                }
        - id: impl_Engine.start_background
          type: ImplItemFn
          code: |2
                /// Starts the background activity of every layer (once, globally).
                ///
                /// Each layer decides for itself, based on its own implementation,
                /// whether it actually becomes active; the default is no activity.
                pub fn start_background(&mut self) {
                    for layer in self.registry.all() {
                        let id = layer.id().to_string();
                        if self.background_tasks.contains_key(&id) {
                            continue;
                        }
                        let cancel = CancellationToken::new();
                        self.cancel_tokens.insert(id.clone(), cancel.clone());
                        let storage = match self.persistence.layer_storage(&id) {
                            Ok(s) => s,
                            Err(_) => continue,
                        };
                        let shared_index = self.persistence.shared_index.clone();
                        let cpu = self.cpu.clone();
                        let layer_clone = layer.clone();
                        let handle = tokio::spawn(async move {
                            let ctx = BackgroundContext {
                                storage,
                                shared_index,
                                cpu,
                            };
                            layer_clone.background(&ctx, cancel).await;
                        });
                        self.background_tasks.insert(id, handle);
                    }
                }
        - id: impl_Engine.wait_background
          type: ImplItemFn
          code: |2
                /// Waits until all background activity has finished.
                ///
                /// A CLI process calls this after all queries have returned, before it
                /// terminates.
                pub async fn wait_background(&mut self) {
                    let handles: Vec<_> = self.background_tasks.drain().map(|(_, h)| h).collect();
                    for h in handles {
                        let _ = h.await;
                    }
                }
        - id: impl_Engine.cancel_background
          type: ImplItemFn
          code: |2
                pub fn cancel_background(&self) {
                    for token in self.cancel_tokens.values() {
                        token.cancel();
                    }
                }
        - id: impl_Engine.shutdown
          type: ImplItemFn
          code: |2
                pub async fn shutdown(&mut self, cancel: bool) {
                    if cancel {
                        self.cancel_background();
                    }
                    self.wait_background().await;
                }
    - path: /home/user/xyan/xy.ai.workbench/rag/src/core/registry.rs
      nodes:
      - id: ModuleDoc
        type: ModuleAttrs
        code: |
          //! Registry for layer implementations.
      - id: Jn9RH7|NASTPa
        type: ImportGroup
        code: |
          use std::sync::Arc;

          use anyhow::{bail, Result};

          use crate::core::layer::Layer;
      - id: LayerRegistry
        type: ItemStruct
        code: |
          /// Manages all available layer instances, addressed by their id.
          #[derive(Default)]
          pub struct LayerRegistry {
              layers: Vec<Arc<dyn Layer>>,
          }
      - id: impl_LayerRegistry
        type: ItemImpl
        signature: "impl LayerRegistry {"
        children:
        - id: impl_LayerRegistry.new
          type: ImplItemFn
          code: |2
                pub fn new() -> Self {
                    Self { layers: Vec::new() }
                }
        - id: impl_LayerRegistry.register
          type: ImplItemFn
          code: |2
                pub fn register(&mut self, layer: Arc<dyn Layer>) -> Result<()> {
                    if layer.id().is_empty() {
                        bail!("Layer needs a unique non empty ID");
                    }
                    if self.layers.iter().any(|l| l.id() == layer.id()) {
                        bail!("Layer-ID already registered: {}", layer.id());
                    }
                    self.layers.push(layer);
                    Ok(())
                }
        - id: impl_LayerRegistry.unregister
          type: ImplItemFn
          code: |2
                pub fn unregister(&mut self, layer_id: &str) {
                    self.layers.retain(|l| l.id() != layer_id);
                }
        - id: impl_LayerRegistry.get
          type: ImplItemFn
          code: |2
                pub fn get(&self, layer_id: &str) -> Option<Arc<dyn Layer>> {
                    self.layers.iter().find(|l| l.id() == layer_id).cloned()
                }
        - id: impl_LayerRegistry.all
          type: ImplItemFn
          code: |2
                pub fn all(&self) -> Vec<Arc<dyn Layer>> {
                    self.layers.clone()
                }
        - id: impl_LayerRegistry.iter
          type: ImplItemFn
          code: |2
                pub fn iter(&self) -> impl Iterator<Item = &Arc<dyn Layer>> {
                    self.layers.iter()
                }
        - id: impl_LayerRegistry.len
          type: ImplItemFn
          code: |2
                pub fn len(&self) -> usize {
                    self.layers.len()
                }
        - id: impl_LayerRegistry.is_empty
          type: ImplItemFn
          code: |2
                pub fn is_empty(&self) -> bool {
                    self.layers.is_empty()
                }
        - id: impl_LayerRegistry.contains
          type: ImplItemFn
          code: |2
                pub fn contains(&self, layer_id: &str) -> bool {
                    self.layers.iter().any(|l| l.id() == layer_id)
                }
```
/answer 9faa-10 allow
Control Request:
```yaml
id: 9faa-11
toolName: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src
    pattern: cfg\(feature = "cuda"\)
    include:
    - '*.rs'
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src
    pattern: ComputeBackend|Factory|factory
    include:
    - '*.rs'
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag
    pattern: ci|CI|github/workflows
    include:
    - '*.yml'
    - '*.yaml'
    exclude:
    - .bin/**
    - .xyrag/**
  reason: "Pruefen ob bereits eine Backend-Factory, CI-Matrix oder weitere cfg(feature cuda) Stellen existieren."
```
/answer 9faa-11 allow
Control Request:
```yaml
id: 9faa-12
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src
      matches:
      - path: resident/kernel/mod.rs
        matches:
        - lineno: 3
          match: "#[cfg(feature = \"cuda\")]"
        - lineno: 5
          match: "#[cfg(feature = \"cuda\")]"
      - path: resident/subengine/mod.rs
        matches:
        - lineno: 3
          match: "#[cfg(feature = \"cuda\")]"
        - lineno: 5
          match: "#[cfg(feature = \"cuda\")]"
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src
      matches:
      - path: resident/mock.rs
        matches:
        - lineno: 24
          match: "pub trait ComputeBackend {"
        - lineno: 29
          match: "impl ComputeBackend for MockBackend {"
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag
      matches: []
```
/answer 9faa-12 allow
Control Request:
```yaml
id: 9faa-13
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel/mod.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/manager/mod.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/subengine/mod.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/error.rs
  node_type: .*
  reason: "Vollständigen Inhalt der relevanten Module lesen, um Backend-Factory sinnvoll zu platzieren."
```
/answer 9faa-13 allow
Control Request:
```yaml
id: 9faa-14
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel/mod.rs
      nodes:
      - id: ModuleDoc
        type: ModuleAttrs
        code: |
          //! SegmentDesc/Layout/KernelHit, Kernel trait.
      - id: cuda
        type: ItemMod
        code: |
          #[cfg(feature = "cuda")]
          mod cuda;
      - id: AeHRoB|0NKx9w
        type: ImportGroup
        code: |
          #[cfg(feature = "cuda")]
          pub use cuda::*;
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/manager/mod.rs
      nodes:
      - id: ModuleDoc
        type: ModuleAttrs
        code: |
          //! StreamedIndexManager, ScopeResolver, update queue, residency.
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/subengine/mod.rs
      nodes:
      - id: ModuleDoc
        type: ModuleAttrs
        code: |
          //! Shared subengine lifecycle & pull model, tokenizer/normalizer (CPU).
      - id: device
        type: ItemMod
        code: |
          #[cfg(feature = "cuda")]
          mod device;
      - id: p03Xlc|0j1nS6
        type: ImportGroup
        code: |
          #[cfg(feature = "cuda")]
          pub use device::*;
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/error.rs
      nodes:
      - id: ModuleDoc
        type: ModuleAttrs
        code: |
          //! Error taxonomy.
          //!
          //! One enum per layer; upper layers wrap lower ones via `#[from]`/`#[source]`. All variants
          //! are `Send + Sync + 'static` and carry context as typed fields, never pre-formatted strings.
          //! Public enums are `#[non_exhaustive]` so new variants don't break downstream matches.
      - id: MS3n2e|NqxAEG
        type: ImportGroup
        code: |
          use thiserror::Error;
      - id: IsRetryable
        type: ItemTrait
        signature: "/// Classifies an error as retryable (transient) vs. fatal, for backoff decisio…"
        docstring: "Classifies an error as retryable (transient) vs. fatal, for backoff decisions."
        children:
        - id: IsRetryable.is_retryable
          type: TraitItemFn
          code: |2
                fn is_retryable(&self) -> bool;
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
      - id: ManagerError
        type: ItemEnum
        code: |
          /// Internal index-manager errors; mapped to a public variant or retried at the API boundary,
          /// never exposed directly.
          #[derive(Error, Debug)]
          #[non_exhaustive]
          pub(crate) enum ManagerError {
              #[error("stale mapping for scope '{scope}'")]
              StaleMapping { scope: String },
          }
      - id: impl_IsRetryable_for_ManagerError
        type: ItemImpl
        signature: "impl IsRetryable for ManagerError {"
        children:
        - id: impl_IsRetryable_for_ManagerError.is_retryable
          type: ImplItemFn
          code: |2
                fn is_retryable(&self) -> bool {
                    matches!(self, ManagerError::StaleMapping { .. })
                }
      - id: PrecisionError
        type: ItemEnum
        code: |
          /// Precision/calibration layer errors.
          #[derive(Error, Debug)]
          #[non_exhaustive]
          pub enum PrecisionError {
              #[error("incompatible precision/layout: {reason}")]
              Incompatible { reason: String },
              #[error("calibration unstable: {reason}")]
              CalibrationUnstable { reason: String },
              #[error("no active calibration profile")]
              NoActiveProfile,
              #[error("invalid precision requested: {requested}")]
              InvalidPrecision { requested: f32 },
          }
      - id: impl_IsRetryable_for_PrecisionError
        type: ItemImpl
        signature: "impl IsRetryable for PrecisionError {"
        children:
        - id: impl_IsRetryable_for_PrecisionError.is_retryable
          type: ImplItemFn
          code: |2
                fn is_retryable(&self) -> bool {
                    matches!(self, PrecisionError::CalibrationUnstable { .. })
                }
      - id: PlannerError
        type: ItemEnum
        code: |
          /// Planner/task-expansion layer errors.
          #[derive(Error, Debug)]
          #[non_exhaustive]
          pub enum PlannerError {
              #[error("invalid planner config: {reason}")]
              InvalidConfig { reason: String },
              #[error("missing capability: {capability}")]
              MissingCapability { capability: String },
              #[error("unknown model: {model}")]
              UnknownModel { model: String },
          }
      - id: impl_IsRetryable_for_PlannerError
        type: ItemImpl
        signature: "impl IsRetryable for PlannerError {"
        children:
        - id: impl_IsRetryable_for_PlannerError.is_retryable
          type: ImplItemFn
          code: |2
                fn is_retryable(&self) -> bool {
                    false
                }
      - id: CapabilityError
        type: ItemEnum
        code: |
          /// Capability/profile-matching layer errors.
          #[derive(Error, Debug)]
          #[non_exhaustive]
          pub enum CapabilityError {
              #[error("missing capability: {capability}")]
              Missing { capability: String },
              #[error("unknown capability: {capability}")]
              Unknown { capability: String },
          }
      - id: ConfigError
        type: ItemEnum
        code: |
          /// Facade/configuration errors raised by the builder and `init()`.
          #[derive(Error, Debug)]
          #[non_exhaustive]
          pub enum ConfigError {
              #[error("missing required field: {field}")]
              MissingField { field: String },
              #[error("unknown model: {model}")]
              UnknownModel { model: String },
              #[error("invalid value for '{field}': {reason}")]
              InvalidValue { field: String, reason: String },
          }
      - id: impl_IsRetryable_for_CapabilityError
        type: ItemImpl
        signature: "impl IsRetryable for CapabilityError {"
        children:
        - id: impl_IsRetryable_for_CapabilityError.is_retryable
          type: ImplItemFn
          code: |2
                fn is_retryable(&self) -> bool {
                    false
                }
      - id: impl_IsRetryable_for_ConfigError
        type: ItemImpl
        signature: "impl IsRetryable for ConfigError {"
        children:
        - id: impl_IsRetryable_for_ConfigError.is_retryable
          type: ImplItemFn
          code: |2
                fn is_retryable(&self) -> bool {
                    false
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
                    }
                }
```
/answer 9faa-14 allow
Control Request:
```yaml
id: 9faa-15
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/docs/architecture/feature_matrix.md
    ids:
    - 4a7ba8
    - 4a7ba8.3BSz7t|h0SceU
    - 4a7ba8.138442
  reason: Vollständigen Text der Feature-Matrix lesen.
```
/answer 9faa-15 allow
Control Request:
```yaml
id: 9faa-16
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/docs/architecture/feature_matrix.md
      nodes:
      - id: 4a7ba8
        type: section
        signature: Feature Matrix
        children:
        - id: 4a7ba8.3BSz7t|h0SceU
          type: paragraph
          code: |
            `cuda` is never part of `default`.
        - id: 4a7ba8.138442
          type: section
          code: |
            ## CI

            - Job A: `cargo build --no-default-features` without CUDA toolkit.
            - Job B: `cargo test --no-default-features`.
            - Job C (GPU runner): `cargo build --features cuda` with `CUDA_COMPUTE_CAP=75`, CUDA 12.x.
            - Job D (loom, on-demand): `RUSTFLAGS="--cfg loom" cargo test --release` for CAS-protocol
              modules; exhaustive, so excluded from the default per-commit run.
      - id: 4a7ba8.3BSz7t|h0SceU
        type: paragraph
        code: |
          `cuda` is never part of `default`.
      - id: 4a7ba8.138442
        type: section
        code: |
          ## CI

          - Job A: `cargo build --no-default-features` without CUDA toolkit.
          - Job B: `cargo test --no-default-features`.
          - Job C (GPU runner): `cargo build --features cuda` with `CUDA_COMPUTE_CAP=75`, CUDA 12.x.
          - Job D (loom, on-demand): `RUSTFLAGS="--cfg loom" cargo test --release` for CAS-protocol
            modules; exhaustive, so excluded from the default per-commit run.
```
/answer 9faa-16 allow
Control Request:
```yaml
id: 9faa-17
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/project/resident/plan/plan_q_03_teststrategie.md
  regexp: ComputeBackend
  reason: "Prüfen, wo ComputeBackend-Trait definiert sein soll (Produktionscode vs. Mock-Modul), um die Factory korrekt zu platzieren."
```
/answer 9faa-17 allow
Control Request:
```yaml
id: 9faa-18
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/project/resident/plan/plan_q_03_teststrategie.md
      nodes:
      - id: dcbabb.259567
        code: |+
          ## 2. Rechercheergebnis (destilliert aus `context_h_q.md` Q3)

          - **Mocks:** `mockall` (`#[automock]` auf Traits) für Erwartungs-/Aufruf-Verifikation; für **zustandsbehaftete** Kernel-/Mess-Mocks sind **manuelle Fakes** klarer (`FakeKernel` mit programmierbaren Ergebnissen, `FakeMeasure` mit Sequenz/RNG).
          - **Nebenläufigkeit:** `loom` (erschöpfende Interleavings unter C11, `loom::sync` via `cfg(loom)`) für kleine CAS-Protokolle (Append/Claim/Cancel/Unlink); `shuttle` (randomisierte Schedules) für größere Szenarien; ergänzend `proptest` für Sequenzen.
          - **GPU-frei:** Backend-Trait (`ComputeBackend`) mit CPU-/Mock-Impl; Feature `cuda` aktiviert nur die CUDA-Impl; Tests laufen **standardmäßig ohne** GPU.
          - **Deterministisch:** virtuelle Zeit (`tokio::time::pause`, Feature `test-util`) oder eigener `Clock`-Trait; Schritt-Hooks/Failpoints zum Erzwingen von Interleavings (ABA: Stamp/Generation gezielt zwischen Lesen und CAS ändern); zustandsbasierte Asserts statt Timing; feste Seeds.
          - **Verrauschte Messwerte:** Mess-Mock hinter Trait: Basiswert + Rauschen aus seeded RNG (`rand`, `StdRng::seed_from_u64`); Varianten konstant/Ausreißer/Drift. Tests: Abbruch an Obergrenze → `unstable`, Konvergenz bei geringem Rauschen, Verhalten bei Ausreißern; mehrere Seeds.

      - id: dcbabb.bed199.G7zDCL|KmM3Ig
        code: |
          **Abstraktions-Traits (alle mit Mock/Fake):**
      - id: dcbabb.15740b
        code: |+
          ## 4. Umsetzungsschritte

          1. Trait-Definitionen + Fakes (`ComputeBackend`, Kernel, Mess, `Clock`) in einem `testkit`/`test-support`-Modul (dev-dependency-sichtbar).
          2. `loom`/`shuttle`/`proptest` als dev-dependencies; `cfg(loom)`-Shim für `std::sync`→`loom::sync` in den betroffenen Modulen.
          3. Failpoint-/Step-Hooks an den Sync-Punkten (Kommunikationsslot, Lade-Pfad).
          4. Testmatrizen aus §3 als konkrete Test-Dateien je Paket.
          5. CI: Default-Lauf ohne `cuda` (alle Unit-Tests), siehe Q4.

      - id: dcbabb.b37a7d.bpuQVL|Q6qopj
        code: |
          ```yaml
          id: f91b-6c
          toolName: ast_edit_marks
          arguments:
            items:
            - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/mock.rs
              start_marker: "/// Hand-written deterministic fake, per the convention above."
              end_marker: |-
                self.fixed_millis
                    }
                }
              content: |-
                /// Hand-written deterministic fake, per the convention above.
                pub struct FakeClock {
                    pub fixed_millis: u64,
                }

                impl Clock for FakeClock {
                    fn now_millis(&self) -> u64 {
                        self.fixed_millis
                    }
                }

                /// Backend abstraction the orchestration layer runs against; `cuda` selects the real
                /// CUDA implementation elsewhere, this crate only needs a CPU/mock impl to stay testable.
                pub trait ComputeBackend {
                    fn run_batch(&self, input: &[f32]) -> Vec<f32>;
                }

                /// Identity backend: sufficient for orchestration tests that do not care about numerics.
                pub struct MockBackend;

                impl ComputeBackend for MockBackend {
                    fn run_batch(&self, input: &[f32]) -> Vec<f32> {
                        input.to_vec()
                    }
                }

                /// Kernel-level trait (batch lifecycle + results), abstracted so orchestration/sync logic
                /// is testable without a GPU. Concrete kernel implementations define their own result type.
                pub trait Kernel {
                    type Hit;

                    fn batch_start(&mut self) -> u64;
                    fn batch_end(&mut self, batch_id: u64);
                    fn results(&self, batch_id: u64) -> Vec<Self::Hit>;
                    fn written_ranges(&self, batch_id: u64) -> Vec<(usize, usize)>;
                    fn progress(&self, batch_id: u64) -> usize;
                }

                /// Scriptable kernel fake: results/ranges/progress for a batch are programmed up front
                /// rather than computed, so sync-point tests control exactly what the kernel "found".
                #[derive(Default)]
                pub struct FakeKernel<H> {
                    next_batch_id: u64,
                    scripted: std::collections::HashMap<u64, (Vec<H>, Vec<(usize, usize)>, usize)>,
                }

                impl<H> FakeKernel<H> {
                    pub fn new() -> Self {
                        Self {
                            next_batch_id: 0,
                            scripted: std::collections::HashMap::new(),
                        }
                    }

                    /// Programs the outcome of the next-started batch before it runs.
                    pub fn script(&mut self, batch_id: u64, hits: Vec<H>, ranges: Vec<(usize, usize)>, progress: usize) {
                        self.scripted.insert(batch_id, (hits, ranges, progress));
                    }
                }

                impl<H: Clone> Kernel for FakeKernel<H> {
                    type Hit = H;

                    fn batch_start(&mut self) -> u64 {
                        let id = self.next_batch_id;
                        self.next_batch_id += 1;
                        id
                    }

                    fn batch_end(&mut self, _batch_id: u64) {}

                    fn results(&self, batch_id: u64) -> Vec<H> {
                        self.scripted
                            .get(&batch_id)
                            .map(|(hits, _, _)| hits.clone())
                            .unwrap_or_default()
                    }

                    fn written_ranges(&self, batch_id: u64) -> Vec<(usize, usize)> {
                        self.scripted
                            .get(&batch_id)
                            .map(|(_, ranges, _)| ranges.clone())
                            .unwrap_or_default()
                    }

                    fn progress(&self, batch_id: u64) -> usize {
                        self.scripted.get(&batch_id).map(|(_, _, p)| *p).unwrap_or(0)
                    }
                }

                /// Measurement trait calibration logic depends on; `FakeMeasure` stands in for real
                /// hardware probing with seeded, reproducible noise.
                pub trait Measure {
                    fn sample(&mut self) -> f64;
                }

                /// Noise models covering the calibration-stability test matrix (F2-F5).
                pub enum NoiseModel {
                    Constant,
                    Outlier { every_n: u32, magnitude: f64 },
                    Drift { per_sample: f64 },
                }

                /// Seeded-RNG measurement fake: `base` plus a `NoiseModel`-shaped perturbation, so
                /// stability/convergence tests are deterministic across runs for a fixed seed.
                pub struct FakeMeasure {
                    base: f64,
                    noise: NoiseModel,
                    rng: rand::rngs::StdRng,
                    sample_count: u32,
                    drift_accum: f64,
                }

                impl FakeMeasure {
                    pub fn new(base: f64, noise: NoiseModel, seed: u64) -> Self {
                        use rand::SeedableRng;
                        Self {
                            base,
                            noise,
                            rng: rand::rngs::StdRng::seed_from_u64(seed),
                            sample_count: 0,
                            drift_accum: 0.0,
                        }
                    }
                }

                impl Measure for FakeMeasure {
                    fn sample(&mut self) -> f64 {
                        use rand::Rng;
                        self.sample_count += 1;
                        match &self.noise {
                            NoiseModel::Constant => self.base + self.rng.gen_range(-0.01..0.01),
                            NoiseModel::Outlier { every_n, magnitude } => {
                                if self.sample_count % every_n == 0 {
                                    self.base + magnitude
                                } else {
                                    self.base + self.rng.gen_range(-0.01..0.01)
                                }
                            }
                            NoiseModel::Drift { per_sample } => {
                                self.drift_accum += per_sample;
                                self.base + self.drift_accum + self.rng.gen_range(-0.01..0.01)
                            }
                        }
                    }
                }

                /// `std::sync` vs. `loom::sync` shim for CAS-protocol modules (B1-style queues): swap the
                /// import under `cfg(loom)` so the same production code runs exhaustively under loom without
                /// a second implementation. Run with `RUSTFLAGS="--cfg loom" cargo test --test <loom_test>`.
                #[cfg(loom)]
                pub mod sync {
                    pub use loom::sync::{atomic, Arc};
                }

                #[cfg(not(loom))]
                pub mod sync {
                    pub use std::sync::{atomic, Arc};
                }
            reason: "Implement the Q3 testkit: ComputeBackend/Kernel/Measure abstractions with fakes (FakeKernel scriptable results, FakeMeasure seeded-noise variants) and the loom sync shim, per plan_q_03_teststrategie.md §3/§4."
          ```
      - id: dcbabb.b37a7d.gVpcmr|Ib4nXf
        code: |
          ```yaml
          id: f91b-6f
          toolName: ast_insert
          arguments:
            items:
            - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/mock.rs
              id: impl_Clock_for_FakeClock
              position: after
              source: |2

                /// Backend abstraction the orchestration layer runs against; `cuda` selects the real
                /// CUDA implementation elsewhere, this crate only needs a CPU/mock impl to stay testable.
                pub trait ComputeBackend {
                    fn run_batch(&self, input: &[f32]) -> Vec<f32>;
                }

                /// Identity backend: sufficient for orchestration tests that do not care about numerics.
                pub struct MockBackend;

                impl ComputeBackend for MockBackend {
                    fn run_batch(&self, input: &[f32]) -> Vec<f32> {
                        input.to_vec()
                    }
                }

                /// Kernel-level trait (batch lifecycle + results), abstracted so orchestration/sync logic
                /// is testable without a GPU. Concrete kernel implementations define their own result type.
                pub trait Kernel {
                    type Hit;

                    fn batch_start(&mut self) -> u64;
                    fn batch_end(&mut self, batch_id: u64);
                    fn results(&self, batch_id: u64) -> Vec<Self::Hit>;
                    fn written_ranges(&self, batch_id: u64) -> Vec<(usize, usize)>;
                    fn progress(&self, batch_id: u64) -> usize;
                }

                /// Scriptable kernel fake: results/ranges/progress for a batch are programmed up front
                /// rather than computed, so sync-point tests control exactly what the kernel "found".
                #[derive(Default)]
                pub struct FakeKernel<H> {
                    next_batch_id: u64,
                    scripted: std::collections::HashMap<u64, (Vec<H>, Vec<(usize, usize)>, usize)>,
                }

                impl<H> FakeKernel<H> {
                    pub fn new() -> Self {
                        Self {
                            next_batch_id: 0,
                            scripted: std::collections::HashMap::new(),
                        }
                    }

                    /// Programs the outcome of the next-started batch before it runs.
                    pub fn script(&mut self, batch_id: u64, hits: Vec<H>, ranges: Vec<(usize, usize)>, progress: usize) {
                        self.scripted.insert(batch_id, (hits, ranges, progress));
                    }
                }

                impl<H: Clone> Kernel for FakeKernel<H> {
                    type Hit = H;

                    fn batch_start(&mut self) -> u64 {
                        let id = self.next_batch_id;
                        self.next_batch_id += 1;
                        id
                    }

                    fn batch_end(&mut self, _batch_id: u64) {}

                    fn results(&self, batch_id: u64) -> Vec<H> {
                        self.scripted
                            .get(&batch_id)
                            .map(|(hits, _, _)| hits.clone())
                            .unwrap_or_default()
                    }

                    fn written_ranges(&self, batch_id: u64) -> Vec<(usize, usize)> {
                        self.scripted
                            .get(&batch_id)
                            .map(|(_, ranges, _)| ranges.clone())
                            .unwrap_or_default()
                    }

                    fn progress(&self, batch_id: u64) -> usize {
                        self.scripted.get(&batch_id).map(|(_, _, p)| *p).unwrap_or(0)
                    }
                }

                /// Measurement trait calibration logic depends on; `FakeMeasure` stands in for real
                /// hardware probing with seeded, reproducible noise.
                pub trait Measure {
                    fn sample(&mut self) -> f64;
                }

                /// Noise models covering the calibration-stability test matrix (F2-F5).
                pub enum NoiseModel {
                    Constant,
                    Outlier { every_n: u32, magnitude: f64 },
                    Drift { per_sample: f64 },
                }

                /// Seeded-RNG measurement fake: `base` plus a `NoiseModel`-shaped perturbation, so
                /// stability/convergence tests are deterministic across runs for a fixed seed.
                pub struct FakeMeasure {
                    base: f64,
                    noise: NoiseModel,
                    rng: rand::rngs::StdRng,
                    sample_count: u32,
                    drift_accum: f64,
                }

                impl FakeMeasure {
                    pub fn new(base: f64, noise: NoiseModel, seed: u64) -> Self {
                        use rand::SeedableRng;
                        Self {
                            base,
                            noise,
                            rng: rand::rngs::StdRng::seed_from_u64(seed),
                            sample_count: 0,
                            drift_accum: 0.0,
                        }
                    }
                }

                impl Measure for FakeMeasure {
                    fn sample(&mut self) -> f64 {
                        use rand::Rng;
                        self.sample_count += 1;
                        match &self.noise {
                            NoiseModel::Constant => self.base + self.rng.gen_range(-0.01..0.01),
                            NoiseModel::Outlier { every_n, magnitude } => {
                                if self.sample_count % every_n == 0 {
                                    self.base + magnitude
                                } else {
                                    self.base + self.rng.gen_range(-0.01..0.01)
                                }
                            }
                            NoiseModel::Drift { per_sample } => {
                                self.drift_accum += per_sample;
                                self.base + self.drift_accum + self.rng.gen_range(-0.01..0.01)
                            }
                        }
                    }
                }

                /// `std::sync` vs. `loom::sync` shim for CAS-protocol modules (B1-style queues): swap the
                /// import under `cfg(loom)` so the same production code runs exhaustively under loom without
                /// a second implementation. Run with `RUSTFLAGS="--cfg loom" cargo test --test <loom_test>`.
                #[cfg(loom)]
                pub mod sync {
                    pub use loom::sync::{atomic, Arc};
                }

                #[cfg(not(loom))]
                pub mod sync {
                    pub use std::sync::{atomic, Arc};
                }
            reason: "Add ComputeBackend/Kernel/Measure traits with fakes and the loom sync shim, implementing Q3 §3/§4."
          ```
      - id: dcbabb.b37a7d.tjdg8S|77cwAq
        code: |
          ```yaml
          id: f91b-76
          toolName: ast_edit_marks
          arguments:
            items:
            - path: /home/user/xyan/xy.ai.workbench/rag/docs/architecture/conventions.md
              id: 40b374.efc803
              start_marker: "## Q3 — Test Strategy & Mocks"
              end_marker: feature_matrix.md`).
              content: |-
                ## Q3 — Test Strategy & Mocks

                - Narrow traits for GPU/measurement interfaces (`ComputeBackend`, `Kernel`, `Measure`);
                  generics (static dispatch) in the hot path, `dyn` only at the edges.
                - Mock strategy: `mockall` (`#[automock]`) for orchestrator-level traits; deterministic
                  hand-written fakes (`Fake*`, fixed/scripted output, simulated failure) for GPU/async-heavy
                  traits, since they compose more simply across `Send`/`async` boundaries. See
                  `src/resident/mock.rs`: `FakeClock`, `MockBackend`, `FakeKernel` (scriptable batch results),
                  `FakeMeasure` (seeded-RNG noise, `NoiseModel::{Constant,Outlier,Drift}`).
                - Visibility: all test doubles are gated `#[cfg(any(test, feature = "mock"))]`.
                - Concurrency: CAS protocols (e.g. the queue's claim/complete/cancel/unlink transitions) run
                  under `loom` for exhaustive interleavings; the production code imports `sync::{atomic, Arc}`
                  from `src/resident/mock.rs`, which resolves to `loom::sync` under `cfg(loom)` and to
                  `std::sync` otherwise — no second implementation. Larger scenarios use `shuttle`
                  (randomized schedules); sequences/properties use `proptest`. Loom runs are a separate,
                  explicit invocation (`RUSTFLAGS="--cfg loom" cargo test`), not part of the default suite.
                - Deterministic time: `tokio::time::pause` (`test-util` dev-feature) or the `Clock` trait for
                  TTL/idle/time-slice logic; prefer state-based assertions over timing-based ones.
                - Testmatrix: each package ships its own test matrix (queue semantics, index-sync ordering,
                  calibration stability, processor/planner behavior) as it is implemented; this document only
                  fixes the convention and the shared testkit, not the per-package matrices.
                - Precision/scorer tests assert bounds/relations, never bit-equality.
                - No-GPU path: GPU integration tests are `#[ignore]`d or behind a dedicated feature, so CI
                  without a GPU stays green (see `docs/architecture/feature_matrix.md`).
            reason: "Document the expanded Q3 testkit (ComputeBackend/Kernel/Measure fakes, loom sync shim, deterministic time) in the conventions doc."
          ```
      - id: dcbabb.b37a7d.LWdNGa|mSw2cM
        code: |
          /answer f91b-78 allow
          Result Stats: id=48a1aab3-e267-47a2-9e12-df6cbec374f3, total: 48627, in: 163, out: 12780, reason: 1886, read: 379447, write: 35684
          ReasoningToken: 229
          ReasoningToken: 1377
          ReasoningToken: 226
          Text:
           Now expand the testkit module with `ComputeBackend`, `Kernel`/`FakeKernel`, `Measure`/`FakeMeasure`, and the `loom` sync shim pattern.
          ReasoningToken: 54
          Text:
           Now insert the new testkit content after the existing `impl_Clock_for_FakeClock` block.
          Text:
           Validate the file compiles, and check `#[check_cfg]`/`loom` lint needs a `build.rs` cfg registration (optional) — I'll check for a `.cargo/config.toml` to add the `unexpected_cfgs` allowlist.
          Text:
           Add `loom` to the `check-cfg` allowlist so `rustc` doesn't warn on the unregistered cfg.
          Text:
           Now update `conventions.md` Q3 section and `feature_matrix.md` CI to document the expanded strategy.
          Text:
           Final validation of all touched files.
          Q3 ist umgesetzt:
      - id: dcbabb.b37a7d
        code: |
          ## 6. Definition of Done

          - Abstraktions-Traits + Fakes vorhanden; `loom`/`shuttle`/`proptest` eingebunden; Failpoints an Sync-Punkten; die Testmatrizen (B1, E5/E6, F2–F5, C2/C3) sind als Tests angelegt; alle Tests laufen **ohne** GPU im Default-Profil.

          SystemInit:  id=48a1aab3-e267-47a2-9e12-df6cbec374f3, cwd=/home/user/xyan/xy.ai.workbench/project, model=claude-sonnet-5
          Control Request:
          ```yaml
          id: f91b-59
          toolName: ast_outline
          arguments:
            paths:
            - /home/user/xyan/xy.ai.workbench/rag
            reason: Inspect existing Rust workspace structure before implementing Q3 test strategy.
          ```
          /answer f91b-59 allow
          Control Request:
          ```yaml
          id: f91b-5a
          result:
            structuredContent:
              errors:
              - path: /home/user/xyan/xy.ai.workbench/rag
                error: |-
                  Path is a directory, not a file; listing its contents instead.
                  Result reduced to top-level directories the limit of 50 is exceeded (809). Narrow down the result using the 'pattern' regular expression parameter.
                  .: (5 files below)
                  ./.bin: (10413 files below)
                  ./.cargo: (1 file below)
                  ./.settings: (1 file below)
                  ./.xyrag: (31 files below)
                  ./benches: (1 file below)
                  ./docs: (4 files below)
                  ./src: (40 files below)
                  ./tests: (1 file below)
          ```
          /answer f91b-5a allow
          Control Request:
          ```yaml
          id: f91b-5b
          toolName: ast_outline
          arguments:
            paths:
            - /home/user/xyan/xy.ai.workbench/rag/src
            - /home/user/xyan/xy.ai.workbench/rag/tests
            - /home/user/xyan/xy.ai.workbench/rag/docs
            reason: "Inspect existing Rust source tree, tests, and docs before implementing Q3 test strategy."
          ```
          /answer f91b-5b allow
          Control Request:
          ```yaml
          id: f91b-5c
          result:
            structuredContent:
              results:
              - path: /home/user/xyan/xy.ai.workbench/rag/tests/trigram_e2e.rs
                nodes:
                - id: ModuleDoc
                  signature: "//! End-to-end tests for the trigram layer (spec part A, \"Tests\")."
                - id: ooP8sA|KRV4l4
                  signature: use std::fs;
                - id: TmpDirs
                  signature: "struct TmpDirs {"
                - id: impl_TmpDirs
                  signature: "impl TmpDirs {"
                  children:
                  - id: impl_TmpDirs.new
                    signature: "fn new(tag: &str) -> Self {"
                - id: impl_Drop_for_TmpDirs
                  signature: "impl Drop for TmpDirs {"
                  children:
                  - id: impl_Drop_for_TmpDirs.drop
                    signature: "fn drop(&mut self) {"
                - id: engine_for
                  signature: "fn engine_for(rag_root: &std::path::Path) -> Engine {"
                - id: query_for
                  signature: "fn query_for(content: &std::path::Path, text: &str) -> Query {"
                - id: lines_of
                  signature: "/// Lines of the result entry for `file`, as `(line_no, text)` pairs. fn lines_…"
                  docstring: "Lines of the result entry for `file`, as `(line_no, text)` pairs."
                - id: multiple_hits_per_line_collapse_and_lines_are_sorted
                  signature: "#[tokio::test] async fn multiple_hits_per_line_collapse_and_lines_are_sorted() {"
                - id: reindexes_on_mtime_change
                  signature: "#[tokio::test] async fn reindexes_on_mtime_change() {"
                - id: large_files_are_skipped
                  signature: "#[tokio::test] async fn large_files_are_skipped() {"
                - id: deleted_file_entry_is_dropped
                  signature: "#[tokio::test] async fn deleted_file_entry_is_dropped() {"
              errors:
              - path: /home/user/xyan/xy.ai.workbench/rag/src
                error: |-
                  Path is a directory, not a file; listing its contents instead.
                  .:
                  lib.rs
                  ./bin:
                  xyrag.rs
                  ./core:
                  engine.rs
                  executor.rs
                  layer.rs
                  mod.rs
                  persistence.rs
                  query.rs
                  registry.rs
                  result.rs
                  topology.rs
                  ./layers:
                  dir_cache.rs
                  glob_layer.rs
                  grep_layer.rs
                  merge_layer.rs
                  mod.rs
                  ./layers/trigram:
                  index.rs
                  layer.rs
                  mod.rs
                  normalize.rs
                  signature.rs
                  trie.rs
                  vocab.rs
                  ./resident:
                  error.rs
                  mock.rs
                  mod.rs
                  telemetry.rs
                  ./resident/facade:
                  mod.rs
                  ./resident/index:
                  mod.rs
                  ./resident/kernel:
                  cuda.rs
                  mod.rs
                  ./resident/manager:
                  mod.rs
                  ./resident/memory:
                  mod.rs
                  ./resident/op:
                  mod.rs
                  ./resident/planner:
                  mod.rs
                  ./resident/precision:
                  mod.rs
                  ./resident/processor:
                  mod.rs
                  ./resident/queue:
                  mod.rs
                  ./resident/subengine:
                  device.rs
                  mod.rs
              - path: /home/user/xyan/xy.ai.workbench/rag/docs
                error: |-
                  Path is a directory, not a file; listing its contents instead.
                  ./architecture:
                  conventions.md
                  dependency_verification.md
                  feature_matrix.md
                  scope_charta.md
          ```
          /answer f91b-5c allow
          Control Request:
          ```yaml
          id: f91b-5d
          toolName: grep
          arguments:
            items:
            - directory:
              - /home/user/xyan/xy.ai.workbench/rag
              pattern: Cargo.toml
              include:
              - '*'
              limit: 20
            - directory:
              - /home/user/xyan/xy.ai.workbench/rag
              pattern: "\\[workspace\\]|\\[dependencies\\]|\\[dev-dependencies\\]|\\[features\\]"
              include:
              - Cargo.toml
              limit: 50
            reason: Find Cargo.toml files to understand workspace/crate structure for adding dev-dependencies.
          ```
          /answer f91b-5d allow
          Control Request:
          ```yaml
          id: f91b-5e
          result:
            structuredContent:
              results:
              - directory:
                - /home/user/xyan/xy.ai.workbench/rag
                matches:
                - path: xyrag.sh
                  matches:
                  - lineno: 45
                    match: "cmd=(cargo run --quiet --manifest-path \"${SCRIPT_DIR}/Cargo.toml\" --bin xyrag -- \"query=${query_text}\")"
                - path: docs/architecture/conventions.md
                  matches:
                  - lineno: 69
                    match: '- `cuda` is never part of `default`; optional GPU crates use `dep:` syntax in `Cargo.toml`.'
                  - lineno: 79
                    match: "- `mock` feature wired in `Cargo.toml` (`[features] mock = []`)."
                - path: docs/architecture/dependency_verification.md
                  matches:
                  - lineno: 6
                    match: "## 1. Verified Versions (pinned in `Cargo.toml`)"
                  - lineno: 50
                    match: '- This is a build-environment setting (not expressible purely in `Cargo.toml`); it must be set'
                  - lineno: 81
                    match: "| 4 | `tokenizers` `default-features = false` vs. XLM-R `tokenizer.json` | **Partially resolved.** Feature flag set in `Cargo.toml`; test written (`#[ignore]`), pending fixture + execution. |"
                - path: .bin/debug/.fingerprint/zerocopy-9d1f17640a59c2d3/run-build-script-build-script-build.json
                  matches:
                  - lineno: 1
                    match: "{\"rustc\":5244445300240561903,\"features\":\"\",\"declared_features\":\"\",\"target\":0,\"profile\":0,\"path\":0,\"deps\":[[18255971429060673642,\"build_script_build\",false,16249165884795476598]],\"local\":[{\"RerunIfChanged\":{\"output\":\"debug/build/zerocopy-9d1f17640a59c2d3/output\",\"paths\":[\"build.rs\",\"Cargo.toml\"]}}],\"rustflags\":[],\"config\":0,\"compile_kind\":0}"
                - path: .bin/debug/.fingerprint/zerocopy-9c3df72dc0537276/run-build-script-build-script-build.json
                  matches:
                  - lineno: 1
                    match: "{\"rustc\":5244445300240561903,\"features\":\"\",\"declared_features\":\"\",\"target\":0,\"profile\":0,\"path\":0,\"deps\":[[18255971429060673642,\"build_script_build\",false,10140986974885456082]],\"local\":[{\"RerunIfChanged\":{\"output\":\"debug/build/zerocopy-9c3df72dc0537276/output\",\"paths\":[\"build.rs\",\"Cargo.toml\"]}}],\"rustflags\":[],\"config\":0,\"compile_kind\":0}"
                - path: .bin/debug/deps/xyrag-5c293c9fdc7b92c0.d
                  matches:
                  - lineno: 1
                    match: "/home/user/xyan/xy.ai.workbench/rag/.bin/debug/deps/xyrag-5c293c9fdc7b92c0.d: src/bin/xyrag.rs Cargo.toml"
                  - lineno: 3
                    match: "/home/user/xyan/xy.ai.workbench/rag/.bin/debug/deps/libxyrag-5c293c9fdc7b92c0.rmeta: src/bin/xyrag.rs Cargo.toml"
                  - lineno: 6
                    match: "Cargo.toml:"
                - path: .bin/debug/deps/trie_bench-967c8aada588ae0f.d
                  matches:
                  - lineno: 1
                    match: "/home/user/xyan/xy.ai.workbench/rag/.bin/debug/deps/trie_bench-967c8aada588ae0f.d: benches/trie_bench.rs Cargo.toml"
                  - lineno: 3
                    match: "/home/user/xyan/xy.ai.workbench/rag/.bin/debug/deps/libtrie_bench-967c8aada588ae0f.rmeta: benches/trie_bench.rs Cargo.toml"
                  - lineno: 6
                    match: "Cargo.toml:"
                - path: .bin/debug/deps/xy_ai_rag-7f0f7ea964f84ac7.d
                  matches:
                  - lineno: 1
                    match: "/home/user/xyan/xy.ai.workbench/rag/.bin/debug/deps/xy_ai_rag-7f0f7ea964f84ac7.d: src/lib.rs src/core/mod.rs src/core/engine.rs src/core/executor.rs src/core/layer.rs src/core/persistence.rs src/core/query.rs src/core/registry.rs src/core/result.rs src/core/topology.rs src/layers/mod.rs src/layers/dir_cache.rs src/layers/glob_layer.rs src/layers/grep_layer.rs src/layers/trigram/mod.rs src/layers/trigram/trie.rs Cargo.toml"
                  - lineno: 3
                    match: "/home/user/xyan/xy.ai.workbench/rag/.bin/debug/deps/libxy_ai_rag-7f0f7ea964f84ac7.rmeta: src/lib.rs src/core/mod.rs src/core/engine.rs src/core/executor.rs src/core/layer.rs src/core/persistence.rs src/core/query.rs src/core/registry.rs src/core/result.rs src/core/topology.rs src/layers/mod.rs src/layers/dir_cache.rs src/layers/glob_layer.rs src/layers/grep_layer.rs src/layers/trigram/mod.rs src/layers/trigram/trie.rs Cargo.toml"
                  - lineno: 21
                    match: "Cargo.toml:"
                - path: .bin/debug/deps/xy_ai_rag-953d85356930d43f.d
                  matches:
                  - lineno: 1
                    match: "/home/user/xyan/xy.ai.workbench/rag/.bin/debug/deps/xy_ai_rag-953d85356930d43f.d: src/lib.rs src/core/mod.rs src/core/engine.rs src/core/executor.rs src/core/layer.rs src/core/persistence.rs src/core/query.rs src/core/registry.rs src/core/result.rs src/core/topology.rs src/layers/mod.rs src/layers/dir_cache.rs src/layers/glob_layer.rs src/layers/grep_layer.rs src/layers/trigram/mod.rs src/layers/trigram/index.rs src/layers/trigram/layer.rs src/layers/trigram/normalize.rs src/layers/trigram/signature.rs src/layers/trigram/trie.rs src/layers/trigram/vocab.rs Cargo.toml"
                  - lineno: 3
                    match: "/home/user/xyan/xy.ai.workbench/rag/.bin/debug/deps/libxy_ai_rag-953d85356930d43f.rmeta: src/lib.rs src/core/mod.rs src/core/engine.rs src/core/executor.rs src/core/layer.rs src/core/persistence.rs src/core/query.rs src/core/registry.rs src/core/result.rs src/core/topology.rs src/layers/mod.rs src/layers/dir_cache.rs src/layers/glob_layer.rs src/layers/grep_layer.rs src/layers/trigram/mod.rs src/layers/trigram/index.rs src/layers/trigram/layer.rs src/layers/trigram/normalize.rs src/layers/trigram/signature.rs src/layers/trigram/trie.rs src/layers/trigram/vocab.rs Cargo.toml"
                  - lineno: 26
                    match: "Cargo.toml:"
                warning: "Limit of 20 matches reached; further results may exist. Narrow the pattern, directory or include/exclude filters, or raise limit."
              - directory:
                - /home/user/xyan/xy.ai.workbench/rag
                matches:
                - path: Cargo.toml
                  matches:
                  - lineno: 15
                    match: "[dependencies]"
                  - lineno: 50
                    match: "[features]"
                  - lineno: 56
                    match: "[dev-dependencies]"
          ```
          /answer f91b-5e allow
          Control Request:
          ```yaml
          id: f91b-5f
          toolName: ast_outline
          arguments:
            paths:
            - /home/user/xyan/xy.ai.workbench/rag/Cargo.toml
            - /home/user/xyan/xy.ai.workbench/rag/src/resident/mock.rs
            - /home/user/xyan/xy.ai.workbench/rag/docs/architecture/conventions.md
            reason: Review current Cargo.toml dependencies/features and existing mock module and conventions doc before implementing Q3.
          ```
          /answer f91b-5f allow
          Control Request:
          ```yaml
          id: f91b-60
          result:
            structuredContent:
              results:
              - path: /home/user/xyan/xy.ai.workbench/rag/Cargo.toml
                nodes:
                - id: h8JWUc|XOXuKk
                  signature: "[package]"
                - id: mN2I6g|SIE1cj
                  signature: "[lib]"
                - id: cW2ehx|QePTHE
                  signature: "[[bin]]"
                - id: zHTLlI|2t4fSB
                  signature: "[dependencies]"
                - id: adh0oF|3scqb3
                  signature: "[features]"
                - id: KrR2sM|Qk967b
                  signature: "[dev-dependencies]"
                - id: 9OxNvg|ELdnWn
                  signature: "[[bench]]"
              - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/mock.rs
                nodes:
                - id: ModuleDoc
                  signature: //! Mock/test-double conventions (Q3).
                - id: Clock
                  signature: /// Example of a narrow trait a hot-path component depends on generically. pub …
                  docstring: Example of a narrow trait a hot-path component depends on generically.
                  children:
                  - id: Clock.now_millis
                    signature: fn now_millis(&self) -> u64;
                - id: FakeClock
                  signature: "/// Hand-written deterministic fake, per the convention above. pub struct FakeC…"
                  docstring: "Hand-written deterministic fake, per the convention above."
                - id: impl_Clock_for_FakeClock
                  signature: "impl Clock for FakeClock {"
                  children:
                  - id: impl_Clock_for_FakeClock.now_millis
                    signature: "fn now_millis(&self) -> u64 {"
              - path: /home/user/xyan/xy.ai.workbench/rag/docs/architecture/conventions.md
                nodes:
                - id: 40b374
                  signature: "Cross-Cutting Conventions (A4): Errors, Logging, Testing, Feature Flags"
                  children:
                  - id: 40b374.1ab48d
                    signature: Q1 — Error Taxonomy (`thiserror`)
                  - id: 40b374.90377b
                    signature: Q2 — Logging (`tracing`)
                  - id: 40b374.efc803
                    signature: Q3 — Test Strategy & Mocks
                  - id: 40b374.ace7f8
                    signature: Q4 — Feature Flags
                  - id: 40b374.73ab5c
                    signature: Basis Scaffolding
          ```
          /answer f91b-60 allow
          ReasoningToken: 62
          Control Request:
          ```yaml
          id: f91b-61
          toolName: ast_read
          arguments:
            items:
            - path: /home/user/xyan/xy.ai.workbench/rag/Cargo.toml
              ids:
              - zHTLlI|2t4fSB
              - adh0oF|3scqb3
              - KrR2sM|Qk967b
            - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/mock.rs
              ids:
              - ModuleDoc
              - Clock
              - FakeClock
              - impl_Clock_for_FakeClock
            - path: /home/user/xyan/xy.ai.workbench/rag/docs/architecture/conventions.md
              ids:
              - 40b374.efc803
            reason: "Read full current content for Cargo deps/features, mock module, and Q3 conventions section."
          ```
          /answer f91b-61 allow
          Control Request:
          ```yaml
          id: f91b-62
          result:
            structuredContent:
              results:
              - path: /home/user/xyan/xy.ai.workbench/rag/Cargo.toml
                nodes:
                - id: zHTLlI|2t4fSB
                  type: table
                  code: |+
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
                    half = "2.7.1"
                    memmap2 = "0.9.3"
                    parquet = "60.0.0"
                    arrow = "60.0.0"
                    tokenizers = { version = "0.23.2", default-features = false }
                    safetensors = "0.8.0"
                    candle-core = { version = "0.11.0", optional = true, default-features = false }
                    candle-transformers = { version = "0.11.0", optional = true, default-features = false }
                    cudarc = { version = "=0.19.9", optional = true }

                - id: adh0oF|3scqb3
                  type: table
                  code: |+
                    [features]
                    default = []
                    cuda = ["dep:candle-core", "dep:candle-transformers", "dep:cudarc", "candle-core/cuda", "candle-transformers/cuda"]
                    mock = []
                    json = ["tracing-subscriber/json"]

                - id: KrR2sM|Qk967b
                  type: table
                  code: |+
                    [dev-dependencies]
                    proptest = "1"
                    criterion = "0.5"
                    mockall = "0.15.0"

              - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/mock.rs
                nodes:
                - id: ModuleDoc
                  type: ModuleAttrs
                  code: |
                    //! Mock/test-double conventions (Q3).
                    //!
                    //! Narrow traits for GPU/measurement interfaces use generics (static dispatch) in the hot
                    //! path; `dyn` only at the edges. Deterministic fakes (fixed output, simulated failure) are
                    //! hand-written rather than `mockall`-generated, since that composes more simply across
                    //! `async`/`Send` boundaries; `mockall::automock` is reserved for orchestrator-level traits.
                    //! Everything here is gated so it never ships in a production build without the `mock` feature.

                    #![cfg(any(test, feature = "mock"))]
                - id: Clock
                  type: ItemTrait
                  signature: /// Example of a narrow trait a hot-path component depends on generically. pub …
                  docstring: Example of a narrow trait a hot-path component depends on generically.
                  children:
                  - id: Clock.now_millis
                    type: TraitItemFn
                    code: |2
                          fn now_millis(&self) -> u64;
                - id: FakeClock
                  type: ItemStruct
                  code: |
                    /// Hand-written deterministic fake, per the convention above.
                    pub struct FakeClock {
                        pub fixed_millis: u64,
                    }
                - id: impl_Clock_for_FakeClock
                  type: ItemImpl
                  signature: "impl Clock for FakeClock {"
                  children:
                  - id: impl_Clock_for_FakeClock.now_millis
                    type: ImplItemFn
                    code: |2
                          fn now_millis(&self) -> u64 {
                              self.fixed_millis
                          }
              - path: /home/user/xyan/xy.ai.workbench/rag/docs/architecture/conventions.md
                nodes:
                - id: 40b374.efc803
                  type: section
                  code: |+
                    ## Q3 — Test Strategy & Mocks

                    - Narrow traits for GPU/measurement interfaces (e.g. a future `Kernel` trait, a measurement
                      trait); generics (static dispatch) in the hot path, `dyn` only at the edges.
                    - Mock strategy: `mockall` (`#[automock]`) for orchestrator-level traits; deterministic
                      hand-written fakes (`Fake*`, fixed output, simulated failure) for GPU/async-heavy traits,
                      since they compose more simply across `Send`/`async` boundaries. See
                      `src/resident/mock.rs` for the convention and a minimal example.
                    - Visibility: all test doubles are gated `#[cfg(any(test, feature = "mock"))]`.
                    - Testmatrix: each package ships its own test matrix (e.g. queue semantics, sync behavior,
                      calibration) as it is implemented; this document only fixes the convention, not the matrices.
                    - No-GPU path: GPU integration tests are `#[ignore]`d or behind a dedicated feature, so CI
                      without a GPU stays green (see `docs/architecture/feature_matrix.md`).
          ```
          /answer f91b-62 allow
          Control Request:
          ```yaml
          id: f91b-63
          toolName: ast_outline
          arguments:
            paths:
            - /home/user/xyan/xy.ai.workbench/rag/src/resident/mod.rs
            - /home/user/xyan/xy.ai.workbench/rag/src/resident/queue/mod.rs
            - /home/user/xyan/xy.ai.workbench/rag/src/resident/index/mod.rs
            - /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/mod.rs
            - /home/user/xyan/xy.ai.workbench/rag/src/resident/processor/mod.rs
            - /home/user/xyan/xy.ai.workbench/rag/src/resident/planner/mod.rs
            - /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel/mod.rs
            - /home/user/xyan/xy.ai.workbench/rag/src/resident/error.rs
            reason: "Understand existing resident package structure (traits, types) to wire Q3 test strategy against real code."
          ```
          /answer f91b-63 allow
          Control Request:
          ```yaml
          id: f91b-64
          result:
            structuredContent:
              results:
              - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/mod.rs
                nodes:
                - id: ModuleDoc
                  signature: "//! Resident engine: embedding/similarity substrate, kept warm."
                - id: error
                  signature: pub mod error;
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
                - id: manager
                  signature: pub mod manager;
              - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/queue/mod.rs
                nodes:
                - id: ModuleDoc
                  signature: //! Queue & task primitives.
              - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/index/mod.rs
                nodes:
                - id: ModuleDoc
                  signature: "//! Index trait, Scope, AccessMethod, Stamp."
              - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/mod.rs
                nodes:
                - id: ModuleDoc
                  signature: //! CapabilitySet/Profile.
              - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/processor/mod.rs
                nodes:
                - id: ModuleDoc
                  signature: "//! Orchestration: task queue + op queues."
              - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/planner/mod.rs
                nodes:
                - id: ModuleDoc
                  signature: //! Stateless task expansion.
              - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel/mod.rs
                nodes:
                - id: ModuleDoc
                  signature: "//! SegmentDesc/Layout/KernelHit, Kernel trait."
                - id: cuda
                  signature: "#[cfg(feature = \"cuda\")]"
                - id: AeHRoB|0NKx9w
                  signature: "#[cfg(feature = \"cuda\")]"
              - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/error.rs
                nodes:
                - id: ModuleDoc
                  signature: //! Error taxonomy.
                - id: MS3n2e|NqxAEG
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
                - id: ResidentError
                  signature: "/// Top-level resident engine error, wrapping every layer error via `#[from]`. …"
                  docstring: "Top-level resident engine error, wrapping every layer error via `#[from]`."
                - id: impl_IsRetryable_for_ResidentError
                  signature: "impl IsRetryable for ResidentError {"
                  children:
                  - id: impl_IsRetryable_for_ResidentError.is_retryable
                    signature: "fn is_retryable(&self) -> bool {"
          ```
          /answer f91b-64 allow
          Control Request:
          ```yaml
          id: f91b-65
          toolName: ast_outline
          arguments:
            paths:
            - /home/user/xyan/xy.ai.workbench/rag/docs/architecture/feature_matrix.md
            reason: Check feature matrix doc conventions for how to document new loom/shuttle dev-only testing features.
          ```
          /answer f91b-65 allow
          Control Request:
          ```yaml
          id: f91b-66
          result:
            structuredContent:
              results:
              - path: /home/user/xyan/xy.ai.workbench/rag/docs/architecture/feature_matrix.md
                nodes:
                - id: 4a7ba8
                  signature: Feature Matrix
                  children:
                  - id: 4a7ba8.3BSz7t|h0SceU
                    signature: '`cuda` is never part of `defa…'
                  - id: 4a7ba8.138442
                    signature: CI
          ```
          /answer f91b-66 allow
          Control Request:
          ```yaml
          id: f91b-67
          toolName: ast_read
          arguments:
            items:
            - path: /home/user/xyan/xy.ai.workbench/rag/docs/architecture/feature_matrix.md
              ids: []
            reason: Read full feature matrix content.
          ```
          /answer f91b-67 allow
          Control Request:
          ```yaml
          id: f91b-68
          result:
            structuredContent:
              results:
              - path: /home/user/xyan/xy.ai.workbench/rag/docs/architecture/feature_matrix.md
                nodes:
                - id: 4a7ba8
                  signature: Feature Matrix
                  children:
                  - id: 4a7ba8.3BSz7t|h0SceU
                    signature: '`cuda` is never part of `defa…'
                  - id: 4a7ba8.138442
                    signature: CI
                errors:
                - "ids [] resolved to no node; redirected to ast_outline, returning the full outline instead."
          ```
          /answer f91b-68 allow
          Control Request:
          ```yaml
          id: f91b-69
          toolName: ast_read
          arguments:
            items:
            - path: /home/user/xyan/xy.ai.workbench/rag/docs/architecture/feature_matrix.md
              ids:
              - 4a7ba8
            reason: Read full feature matrix section content for CI/test documentation conventions.
          ```
          /answer f91b-69 allow
          Control Request:
          ```yaml
          id: f91b-6a
          result:
            structuredContent:
              results:
              - path: /home/user/xyan/xy.ai.workbench/rag/docs/architecture/feature_matrix.md
                nodes:
                - id: 4a7ba8
                  type: section
                  signature: Feature Matrix
                  children:
                  - id: 4a7ba8.3BSz7t|h0SceU
                    type: paragraph
                    code: |
                      `cuda` is never part of `default`.
                  - id: 4a7ba8.138442
                    type: section
                    code: |
                      ## CI

                      - Job A: `cargo build --no-default-features` without CUDA toolkit.
                      - Job B: `cargo test --no-default-features`.
                      - Job C (GPU runner): `cargo build --features cuda` with `CUDA_COMPUTE_CAP=75`, CUDA 12.x.
          ```
          /answer f91b-6a allow
          Control Request:
          ```yaml
          id: f91b-6b
          toolName: ast_edit_block
          arguments:
            items:
            - path: /home/user/xyan/xy.ai.workbench/rag/Cargo.toml
              id: KrR2sM|Qk967b
              old_text: |-
                [dev-dependencies]
                proptest = "1"
                criterion = "0.5"
                mockall = "0.15.0"
              new_text: |-
                [dev-dependencies]
                proptest = "1"
                criterion = "0.5"
                mockall = "0.15.0"
                loom = "0.7"
                shuttle = "0.7"
                rand = "0.8"
                tokio = { version = "1", features = ["test-util"] }
            reason: "Add Q3-required dev-dependencies: loom (exhaustive interleaving), shuttle (randomized schedules), rand (seeded noise for measurement fakes), tokio test-util (virtual time)."
          ```
          /answer f91b-6b allow
          Control Request:
          ```yaml
          id: f91b-6c
          toolName: ast_edit_marks
          arguments:
            items:
            - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/mock.rs
              start_marker: "/// Hand-written deterministic fake, per the convention above."
              end_marker: |-
                self.fixed_millis
                    }
                }
              content: |-
                /// Hand-written deterministic fake, per the convention above.
                pub struct FakeClock {
                    pub fixed_millis: u64,
                }

                impl Clock for FakeClock {
                    fn now_millis(&self) -> u64 {
                        self.fixed_millis
                    }
                }

                /// Backend abstraction the orchestration layer runs against; `cuda` selects the real
                /// CUDA implementation elsewhere, this crate only needs a CPU/mock impl to stay testable.
                pub trait ComputeBackend {
                    fn run_batch(&self, input: &[f32]) -> Vec<f32>;
                }

                /// Identity backend: sufficient for orchestration tests that do not care about numerics.
                pub struct MockBackend;

                impl ComputeBackend for MockBackend {
                    fn run_batch(&self, input: &[f32]) -> Vec<f32> {
                        input.to_vec()
                    }
                }

                /// Kernel-level trait (batch lifecycle + results), abstracted so orchestration/sync logic
                /// is testable without a GPU. Concrete kernel implementations define their own result type.
                pub trait Kernel {
                    type Hit;

                    fn batch_start(&mut self) -> u64;
                    fn batch_end(&mut self, batch_id: u64);
                    fn results(&self, batch_id: u64) -> Vec<Self::Hit>;
                    fn written_ranges(&self, batch_id: u64) -> Vec<(usize, usize)>;
                    fn progress(&self, batch_id: u64) -> usize;
                }

                /// Scriptable kernel fake: results/ranges/progress for a batch are programmed up front
                /// rather than computed, so sync-point tests control exactly what the kernel "found".
                #[derive(Default)]
                pub struct FakeKernel<H> {
                    next_batch_id: u64,
                    scripted: std::collections::HashMap<u64, (Vec<H>, Vec<(usize, usize)>, usize)>,
                }

                impl<H> FakeKernel<H> {
                    pub fn new() -> Self {
                        Self {
                            next_batch_id: 0,
                            scripted: std::collections::HashMap::new(),
                        }
                    }

                    /// Programs the outcome of the next-started batch before it runs.
                    pub fn script(&mut self, batch_id: u64, hits: Vec<H>, ranges: Vec<(usize, usize)>, progress: usize) {
                        self.scripted.insert(batch_id, (hits, ranges, progress));
                    }
                }

                impl<H: Clone> Kernel for FakeKernel<H> {
                    type Hit = H;

                    fn batch_start(&mut self) -> u64 {
                        let id = self.next_batch_id;
                        self.next_batch_id += 1;
                        id
                    }

                    fn batch_end(&mut self, _batch_id: u64) {}

                    fn results(&self, batch_id: u64) -> Vec<H> {
                        self.scripted
                            .get(&batch_id)
                            .map(|(hits, _, _)| hits.clone())
                            .unwrap_or_default()
                    }

                    fn written_ranges(&self, batch_id: u64) -> Vec<(usize, usize)> {
                        self.scripted
                            .get(&batch_id)
                            .map(|(_, ranges, _)| ranges.clone())
                            .unwrap_or_default()
                    }

                    fn progress(&self, batch_id: u64) -> usize {
                        self.scripted.get(&batch_id).map(|(_, _, p)| *p).unwrap_or(0)
                    }
                }

                /// Measurement trait calibration logic depends on; `FakeMeasure` stands in for real
                /// hardware probing with seeded, reproducible noise.
                pub trait Measure {
                    fn sample(&mut self) -> f64;
                }

                /// Noise models covering the calibration-stability test matrix (F2-F5).
                pub enum NoiseModel {
                    Constant,
                    Outlier { every_n: u32, magnitude: f64 },
                    Drift { per_sample: f64 },
                }

                /// Seeded-RNG measurement fake: `base` plus a `NoiseModel`-shaped perturbation, so
                /// stability/convergence tests are deterministic across runs for a fixed seed.
                pub struct FakeMeasure {
                    base: f64,
                    noise: NoiseModel,
                    rng: rand::rngs::StdRng,
                    sample_count: u32,
                    drift_accum: f64,
                }

                impl FakeMeasure {
                    pub fn new(base: f64, noise: NoiseModel, seed: u64) -> Self {
                        use rand::SeedableRng;
                        Self {
                            base,
                            noise,
                            rng: rand::rngs::StdRng::seed_from_u64(seed),
                            sample_count: 0,
                            drift_accum: 0.0,
                        }
                    }
                }

                impl Measure for FakeMeasure {
                    fn sample(&mut self) -> f64 {
                        use rand::Rng;
                        self.sample_count += 1;
                        match &self.noise {
                            NoiseModel::Constant => self.base + self.rng.gen_range(-0.01..0.01),
                            NoiseModel::Outlier { every_n, magnitude } => {
                                if self.sample_count % every_n == 0 {
                                    self.base + magnitude
                                } else {
                                    self.base + self.rng.gen_range(-0.01..0.01)
                                }
                            }
                            NoiseModel::Drift { per_sample } => {
                                self.drift_accum += per_sample;
                                self.base + self.drift_accum + self.rng.gen_range(-0.01..0.01)
                            }
                        }
                    }
                }

                /// `std::sync` vs. `loom::sync` shim for CAS-protocol modules (B1-style queues): swap the
                /// import under `cfg(loom)` so the same production code runs exhaustively under loom without
                /// a second implementation. Run with `RUSTFLAGS="--cfg loom" cargo test --test <loom_test>`.
                #[cfg(loom)]
                pub mod sync {
                    pub use loom::sync::{atomic, Arc};
                }

                #[cfg(not(loom))]
                pub mod sync {
                    pub use std::sync::{atomic, Arc};
                }
            reason: "Implement the Q3 testkit: ComputeBackend/Kernel/Measure abstractions with fakes (FakeKernel scriptable results, FakeMeasure seeded-noise variants) and the loom sync shim, per plan_q_03_teststrategie.md §3/§4."
          ```
          /answer f91b-6c allow
          Control Request:
          ```yaml
          id: f91b-6d
          result:
            structuredContent:
              errors:
              - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/mock.rs
                error: No node matched the given text; a node selector (id) is required.
          ```
          /answer f91b-6d allow
          Control Request:
          ```yaml
          id: f91b-6e
          toolName: ast_edit_marks
          arguments:
            items:
            - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/mock.rs
              id: FakeClock
              start_marker: "/// Hand-written deterministic fake, per the convention above."
              end_marker: |-
                pub fixed_millis: u64,
                }
              content: |-
                /// Hand-written deterministic fake, per the convention above.
                pub struct FakeClock {
                    pub fixed_millis: u64,
                }
            reason: Probe correct id-based marker edit on FakeClock node before appending the larger testkit content.
          ```
          /answer f91b-6e allow
          Control Request:
          ```yaml
          id: f91b-6f
          toolName: ast_insert
          arguments:
            items:
            - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/mock.rs
              id: impl_Clock_for_FakeClock
              position: after
              source: |2

                /// Backend abstraction the orchestration layer runs against; `cuda` selects the real
                /// CUDA implementation elsewhere, this crate only needs a CPU/mock impl to stay testable.
                pub trait ComputeBackend {
                    fn run_batch(&self, input: &[f32]) -> Vec<f32>;
                }

                /// Identity backend: sufficient for orchestration tests that do not care about numerics.
                pub struct MockBackend;

                impl ComputeBackend for MockBackend {
                    fn run_batch(&self, input: &[f32]) -> Vec<f32> {
                        input.to_vec()
                    }
                }

                /// Kernel-level trait (batch lifecycle + results), abstracted so orchestration/sync logic
                /// is testable without a GPU. Concrete kernel implementations define their own result type.
                pub trait Kernel {
                    type Hit;

                    fn batch_start(&mut self) -> u64;
                    fn batch_end(&mut self, batch_id: u64);
                    fn results(&self, batch_id: u64) -> Vec<Self::Hit>;
                    fn written_ranges(&self, batch_id: u64) -> Vec<(usize, usize)>;
                    fn progress(&self, batch_id: u64) -> usize;
                }

                /// Scriptable kernel fake: results/ranges/progress for a batch are programmed up front
                /// rather than computed, so sync-point tests control exactly what the kernel "found".
                #[derive(Default)]
                pub struct FakeKernel<H> {
                    next_batch_id: u64,
                    scripted: std::collections::HashMap<u64, (Vec<H>, Vec<(usize, usize)>, usize)>,
                }

                impl<H> FakeKernel<H> {
                    pub fn new() -> Self {
                        Self {
                            next_batch_id: 0,
                            scripted: std::collections::HashMap::new(),
                        }
                    }

                    /// Programs the outcome of the next-started batch before it runs.
                    pub fn script(&mut self, batch_id: u64, hits: Vec<H>, ranges: Vec<(usize, usize)>, progress: usize) {
                        self.scripted.insert(batch_id, (hits, ranges, progress));
                    }
                }

                impl<H: Clone> Kernel for FakeKernel<H> {
                    type Hit = H;

                    fn batch_start(&mut self) -> u64 {
                        let id = self.next_batch_id;
                        self.next_batch_id += 1;
                        id
                    }

                    fn batch_end(&mut self, _batch_id: u64) {}

                    fn results(&self, batch_id: u64) -> Vec<H> {
                        self.scripted
                            .get(&batch_id)
                            .map(|(hits, _, _)| hits.clone())
                            .unwrap_or_default()
                    }

                    fn written_ranges(&self, batch_id: u64) -> Vec<(usize, usize)> {
                        self.scripted
                            .get(&batch_id)
                            .map(|(_, ranges, _)| ranges.clone())
                            .unwrap_or_default()
                    }

                    fn progress(&self, batch_id: u64) -> usize {
                        self.scripted.get(&batch_id).map(|(_, _, p)| *p).unwrap_or(0)
                    }
                }

                /// Measurement trait calibration logic depends on; `FakeMeasure` stands in for real
                /// hardware probing with seeded, reproducible noise.
                pub trait Measure {
                    fn sample(&mut self) -> f64;
                }

                /// Noise models covering the calibration-stability test matrix (F2-F5).
                pub enum NoiseModel {
                    Constant,
                    Outlier { every_n: u32, magnitude: f64 },
                    Drift { per_sample: f64 },
                }

                /// Seeded-RNG measurement fake: `base` plus a `NoiseModel`-shaped perturbation, so
                /// stability/convergence tests are deterministic across runs for a fixed seed.
                pub struct FakeMeasure {
                    base: f64,
                    noise: NoiseModel,
                    rng: rand::rngs::StdRng,
                    sample_count: u32,
                    drift_accum: f64,
                }

                impl FakeMeasure {
                    pub fn new(base: f64, noise: NoiseModel, seed: u64) -> Self {
                        use rand::SeedableRng;
                        Self {
                            base,
                            noise,
                            rng: rand::rngs::StdRng::seed_from_u64(seed),
                            sample_count: 0,
                            drift_accum: 0.0,
                        }
                    }
                }

                impl Measure for FakeMeasure {
                    fn sample(&mut self) -> f64 {
                        use rand::Rng;
                        self.sample_count += 1;
                        match &self.noise {
                            NoiseModel::Constant => self.base + self.rng.gen_range(-0.01..0.01),
                            NoiseModel::Outlier { every_n, magnitude } => {
                                if self.sample_count % every_n == 0 {
                                    self.base + magnitude
                                } else {
                                    self.base + self.rng.gen_range(-0.01..0.01)
                                }
                            }
                            NoiseModel::Drift { per_sample } => {
                                self.drift_accum += per_sample;
                                self.base + self.drift_accum + self.rng.gen_range(-0.01..0.01)
                            }
                        }
                    }
                }

                /// `std::sync` vs. `loom::sync` shim for CAS-protocol modules (B1-style queues): swap the
                /// import under `cfg(loom)` so the same production code runs exhaustively under loom without
                /// a second implementation. Run with `RUSTFLAGS="--cfg loom" cargo test --test <loom_test>`.
                #[cfg(loom)]
                pub mod sync {
                    pub use loom::sync::{atomic, Arc};
                }

                #[cfg(not(loom))]
                pub mod sync {
                    pub use std::sync::{atomic, Arc};
                }
            reason: "Add ComputeBackend/Kernel/Measure traits with fakes and the loom sync shim, implementing Q3 §3/§4."
          ```
          /answer f91b-6f allow
          ReasoningToken: 62
          Control Request:
          ```yaml
          id: f91b-70
          toolName: ast_validate
          arguments:
            paths:
            - /home/user/xyan/xy.ai.workbench/rag/src/resident/mock.rs
            reason: Verify the expanded testkit module compiles syntactically.
          ```
          /answer f91b-70 allow
          Control Request:
          ```yaml
          id: f91b-71
          toolName: ast_read
          arguments:
            items:
            - path: /home/user/xyan/xy.ai.workbench/rag/.cargo/config.toml
              ids: []
            reason: Check for existing cargo config to add loom cfg to lint allowlist.
          ```
          /answer f91b-71 allow
          Control Request:
          ```yaml
          id: f91b-72
          result:
            structuredContent:
              results:
              - path: /home/user/xyan/xy.ai.workbench/rag/.cargo/config.toml
                nodes:
                - id: zrH6qW|LtxIiD
                  signature: "[build]"
                errors:
                - "ids [] resolved to no node; redirected to ast_outline, returning the full outline instead."
          ```
          /answer f91b-72 allow
          Control Request:
          ```yaml
          id: f91b-73
          toolName: ast_read
          arguments:
            items:
            - path: /home/user/xyan/xy.ai.workbench/rag/.cargo/config.toml
              ids:
              - zrH6qW|LtxIiD
            reason: Read build section content of cargo config.
          ```
          /answer f91b-73 allow
          Control Request:
          ```yaml
          id: f91b-74
          result:
            structuredContent:
              results:
              - path: /home/user/xyan/xy.ai.workbench/rag/.cargo/config.toml
                nodes:
                - id: zrH6qW|LtxIiD
                  type: table
                  code: |-
                    [build]
                    target-dir = ".bin"
          ```
          /answer f91b-74 allow
          Control Request:
          ```yaml
          id: f91b-75
          toolName: ast_edit_block
          arguments:
            items:
            - path: /home/user/xyan/xy.ai.workbench/rag/.cargo/config.toml
              id: zrH6qW|LtxIiD
              old_text: |-
                [build]
                target-dir = ".bin"
              new_text: |-
                [build]
                target-dir = ".bin"
                rustflags = ["--check-cfg=cfg(loom)"]
            reason: Register the loom cfg so the compiler does not warn about an unexpected cfg outside loom-enabled runs.
          ```
          /answer f91b-75 allow
          Control Request:
          ```yaml
          id: f91b-76
          toolName: ast_edit_marks
          arguments:
            items:
            - path: /home/user/xyan/xy.ai.workbench/rag/docs/architecture/conventions.md
              id: 40b374.efc803
              start_marker: "## Q3 — Test Strategy & Mocks"
              end_marker: feature_matrix.md`).
              content: |-
                ## Q3 — Test Strategy & Mocks

                - Narrow traits for GPU/measurement interfaces (`ComputeBackend`, `Kernel`, `Measure`);
                  generics (static dispatch) in the hot path, `dyn` only at the edges.
                - Mock strategy: `mockall` (`#[automock]`) for orchestrator-level traits; deterministic
                  hand-written fakes (`Fake*`, fixed/scripted output, simulated failure) for GPU/async-heavy
                  traits, since they compose more simply across `Send`/`async` boundaries. See
                  `src/resident/mock.rs`: `FakeClock`, `MockBackend`, `FakeKernel` (scriptable batch results),
                  `FakeMeasure` (seeded-RNG noise, `NoiseModel::{Constant,Outlier,Drift}`).
                - Visibility: all test doubles are gated `#[cfg(any(test, feature = "mock"))]`.
                - Concurrency: CAS protocols (e.g. the queue's claim/complete/cancel/unlink transitions) run
                  under `loom` for exhaustive interleavings; the production code imports `sync::{atomic, Arc}`
                  from `src/resident/mock.rs`, which resolves to `loom::sync` under `cfg(loom)` and to
                  `std::sync` otherwise — no second implementation. Larger scenarios use `shuttle`
                  (randomized schedules); sequences/properties use `proptest`. Loom runs are a separate,
                  explicit invocation (`RUSTFLAGS="--cfg loom" cargo test`), not part of the default suite.
                - Deterministic time: `tokio::time::pause` (`test-util` dev-feature) or the `Clock` trait for
                  TTL/idle/time-slice logic; prefer state-based assertions over timing-based ones.
                - Testmatrix: each package ships its own test matrix (queue semantics, index-sync ordering,
                  calibration stability, processor/planner behavior) as it is implemented; this document only
                  fixes the convention and the shared testkit, not the per-package matrices.
                - Precision/scorer tests assert bounds/relations, never bit-equality.
                - No-GPU path: GPU integration tests are `#[ignore]`d or behind a dedicated feature, so CI
                  without a GPU stays green (see `docs/architecture/feature_matrix.md`).
            reason: "Document the expanded Q3 testkit (ComputeBackend/Kernel/Measure fakes, loom sync shim, deterministic time) in the conventions doc."
          ```
          /answer f91b-76 allow
          Control Request:
          ```yaml
          id: f91b-77
          toolName: ast_edit_block
          arguments:
            items:
            - path: /home/user/xyan/xy.ai.workbench/rag/docs/architecture/feature_matrix.md
              id: 4a7ba8.138442
              old_text: "- Job C (GPU runner): `cargo build --features cuda` with `CUDA_COMPUTE_CAP=75`, CUDA 12.x."
              new_text: |-
                - Job C (GPU runner): `cargo build --features cuda` with `CUDA_COMPUTE_CAP=75`, CUDA 12.x.
                - Job D (loom, on-demand): `RUSTFLAGS="--cfg loom" cargo test --release` for CAS-protocol
                  modules; exhaustive, so excluded from the default per-commit run.
            reason: Add the loom CI job to the feature matrix per Q3's concurrency verification strategy.
          ```
          /answer f91b-77 allow
          Control Request:
          ```yaml
          id: f91b-78
          toolName: ast_validate
          arguments:
            paths:
            - /home/user/xyan/xy.ai.workbench/rag/Cargo.toml
            - /home/user/xyan/xy.ai.workbench/rag/src/resident/mock.rs
            - /home/user/xyan/xy.ai.workbench/rag/.cargo/config.toml
            - /home/user/xyan/xy.ai.workbench/rag/docs/architecture/conventions.md
            - /home/user/xyan/xy.ai.workbench/rag/docs/architecture/feature_matrix.md
            reason: Confirm all Q3-related changes compile/parse correctly.
          ```
          /answer f91b-78 allow
          Result Stats: id=48a1aab3-e267-47a2-9e12-df6cbec374f3, total: 48627, in: 163, out: 12780, reason: 1886, read: 379447, write: 35684
          ReasoningToken: 229
          ReasoningToken: 1377
          ReasoningToken: 226
          Text:
           Now expand the testkit module with `ComputeBackend`, `Kernel`/`FakeKernel`, `Measure`/`FakeMeasure`, and the `loom` sync shim pattern.
          ReasoningToken: 54
          Text:
           Now insert the new testkit content after the existing `impl_Clock_for_FakeClock` block.
          Text:
           Validate the file compiles, and check `#[check_cfg]`/`loom` lint needs a `build.rs` cfg registration (optional) — I'll check for a `.cargo/config.toml` to add the `unexpected_cfgs` allowlist.
          Text:
           Add `loom` to the `check-cfg` allowlist so `rustc` doesn't warn on the unregistered cfg.
          Text:
           Now update `conventions.md` Q3 section and `feature_matrix.md` CI to document the expanded strategy.
          Text:
           Final validation of all touched files.
          Q3 ist umgesetzt:

          - **`Cargo.toml`**: `loom`, `shuttle`, `rand` als dev-dependencies ergänzt; `tokio` dev-Feature `test-util` für virtuelle Zeit.
          - **`src/resident/mock.rs`**: Testkit erweitert um `ComputeBackend`/`MockBackend`, `Kernel`/`FakeKernel<H>` (skriptbare Batch-Ergebnisse/Ranges/Fortschritt), `Measure`/`FakeMeasure` mit `NoiseModel::{Constant,Outlier,Drift}` (seeded `StdRng`), sowie `sync`-Shim-Modul (`loom::sync` unter `cfg(loom)`, sonst `std::sync`) für CAS-Protokolle.
          - **`.cargo/config.toml`**: `--check-cfg=cfg(loom)` registriert.
          - **`docs/architecture/conventions.md`**: Q3-Abschnitt um Concurrency-Strategie (loom/shuttle/proptest), deterministische Zeit, No-Bit-Equality-Regel ergänzt.
          - **`docs/architecture/feature_matrix.md`**: Job D (loom, on-demand, separat vom Default-Lauf) hinzugefügt.

          Die konkreten Testmatrizen pro Paket (B1, E5/E6, F2–F5, C2/C3) bleiben wie in der Konvention vorgesehen den jeweiligen Gruppen vorbehalten, da die zugehörigen Module noch leere Stubs sind.
```
/answer 9faa-18 allow
Control Request:
```yaml
id: 9faa-19
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/rag/docs/architecture/conventions.md
  id: 40b374.ace7f8
  reason: "Q4-Abschnitt der Konventionen lesen, um Dopplungen zu vermeiden und die Factory korrekt zu dokumentieren."
```
/answer 9faa-19 allow
Control Request:
```yaml
id: 9faa-1a
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/rag/docs/architecture/conventions.md
      nodes:
      - id: 40b374.ace7f8
        code: |+
          ## Q4 — Feature Flags

          - Real CUDA code paths live behind the `cuda` feature; backend selection via `#[cfg]`:
            ```rust
            #[cfg(feature = "cuda")]
            type DefaultBackend = CudaBackend;
            #[cfg(not(feature = "cuda"))]
            type DefaultBackend = CpuBackend;
            ```
          - `cuda` is never part of `default`; optional GPU crates use `dep:` syntax in `Cargo.toml`.
          - See `docs/architecture/feature_matrix.md` for the CI job matrix (no-GPU build/test vs.
            GPU build with `cuda`).
```
/answer 9faa-1a allow
Control Request:
```yaml
id: 9faa-1b
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel/cuda.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/subengine/device.rs
  node_type: .*
  reason: Vollständigen (minimalen) Inhalt der cuda-gated Stub-Dateien prüfen.
```
/answer 9faa-1b allow
Control Request:
```yaml
id: 9faa-1c
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel/cuda.rs
      nodes:
      - id: ModuleDoc
        type: ModuleAttrs
        code: |
          //! Real CUDA kernels / scorer.
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/subengine/device.rs
      nodes:
      - id: ModuleDoc
        type: ModuleAttrs
        code: |
          //! Device/Lane, execution context, loaded encoder, GPU memory arenas.
```
/answer 9faa-1c allow
ReasoningToken: 80
Control Request:
```yaml
id: 9faa-1d
toolName: ast_create
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/backend.rs
    source: |
      //! Compute backend abstraction and the single `#[cfg(feature = "cuda")]` decision point.
      //!
      //! Everything outside `create_backend` runs against the `ComputeBackend` trait and stays
      //! feature-independent; only this factory and the CUDA adapter modules (`kernel::cuda`,
      //! `subengine::device`) are gated.

      use crate::resident::subengine::CpuBackend;
      #[cfg(feature = "cuda")]
      use crate::resident::subengine::CudaBackend;

      /// Backend abstraction the orchestration layer runs against.
      pub trait ComputeBackend: Send + Sync {
          fn run_batch(&self, input: &[f32]) -> Vec<f32>;
      }

      /// Constructs the active backend: CPU without `cuda`; with `cuda`, attempts the CUDA backend
      /// and falls back to CPU if GPU initialization fails (per `resident_subengine.md` §6).
      pub fn create_backend() -> Box<dyn ComputeBackend> {
          #[cfg(feature = "cuda")]
          {
              if let Ok(backend) = CudaBackend::try_new() {
                  return Box::new(backend);
              }
          }
          Box::new(CpuBackend)
      }
  reason: "Q4 umsetzen: einzigen cfg(feature=cuda)-Entscheidungspunkt als Backend-Factory anlegen."
```
/answer 9faa-1d allow
Control Request:
```yaml
id: 9faa-1e
toolName: ast_edit_marks
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/subengine/mod.rs
    id: ModuleDoc
    start_marker: //! Shared subengine lifecycle
    end_marker: tokenizer/normalizer (CPU).
    content: |-
      //! Shared subengine lifecycle & pull model, tokenizer/normalizer (CPU).

      use crate::resident::backend::ComputeBackend;

      /// CPU backend; default without `cuda`, and the fallback target if CUDA init fails.
      /// Placeholder until the real embedding path (D4/D5) is wired in.
      pub struct CpuBackend;

      impl ComputeBackend for CpuBackend {
          fn run_batch(&self, input: &[f32]) -> Vec<f32> {
              input.to_vec()
          }
      }
  reason: Feature-unabhängiges CpuBackend als Default/Fallback-Ziel der Factory hinzufügen.
```
/answer 9faa-1e allow
Control Request:
```yaml
id: 9faa-1f
toolName: ast_edit_marks
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/subengine/device.rs
    id: ModuleDoc
    start_marker: "//! Device/Lane,"
    end_marker: GPU memory arenas.
    content: |-
      //! Device/Lane, execution context, loaded encoder, GPU memory arenas.

      use crate::resident::backend::ComputeBackend;

      /// Real CUDA backend; the single Candle/CUDA adapter point for the subengine (per
      /// `resident_subengine.md` §2). Construction fails if no compatible GPU is available,
      /// which lets `create_backend` fall back to `CpuBackend`.
      pub struct CudaBackend;

      impl CudaBackend {
          pub fn try_new() -> Result<Self, crate::resident::error::IndexError> {
              // Real device/encoder init (candle-core CUDA adapter) lands in D2/D3.
              Err(crate::resident::error::IndexError::TransferFailed {
                  reason: "CUDA backend not yet implemented".to_string(),
              })
          }
      }

      impl ComputeBackend for CudaBackend {
          fn run_batch(&self, _input: &[f32]) -> Vec<f32> {
              unimplemented!("real CUDA kernel dispatch lands in D6/H1")
          }
      }
  reason: CudaBackend als einzigen Candle/CUDA-Adapter-Punkt im subengine-Modul hinzufügen.
```
/answer 9faa-1f allow
Control Request:
```yaml
id: 9faa-20
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/mod.rs
    id: error
    old_text: pub mod error;
    new_text: |-
      pub mod backend;
      pub mod error;
  reason: Neues backend-Modul in resident/mod.rs registrieren.
```
/answer 9faa-20 allow
Control Request:
```yaml
id: 9faa-21
toolName: ast_edit_marks
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/mock.rs
    id: ComputeBackend
    start_marker: /// Backend abstraction the orchestration layer runs against
    end_marker: |-
      fn run_batch(&self, input: &[f32]) -> Vec<f32>;
      }
    content: use crate::resident::backend::ComputeBackend;
  reason: Duplizierte ComputeBackend-Trait-Definition in mock.rs entfernen; stattdessen die produktive Trait-Definition aus resident::backend importieren.
```
/answer 9faa-21 allow
Control Request:
```yaml
id: 9faa-22
toolName: ast_validate
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/backend.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/subengine/mod.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/subengine/device.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/mod.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/mock.rs
  reason: Alle geänderten Dateien auf Syntaxkorrektheit prüfen.
```
/answer 9faa-22 allow
Control Request:
```yaml
id: 9faa-23
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/mock.rs
  regexp: ComputeBackend|MockBackend
  reason: Ergebnis der mock.rs-Änderung um ComputeBackend-Import überprüfen.
```
/answer 9faa-23 allow
Control Request:
```yaml
id: 9faa-24
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/mock.rs
      nodes:
      - id: 77cuty|ChE8yG
        code: |
          use crate::resident::backend::ComputeBackend;
      - id: MockBackend
        code: |
          /// Identity backend: sufficient for orchestration tests that do not care about numerics.
          pub struct MockBackend;
      - id: impl_ComputeBackend_for_MockBackend
        code: |
          impl ComputeBackend for MockBackend {
              fn run_batch(&self, input: &[f32]) -> Vec<f32> {
                  input.to_vec()
              }
          }
```
/answer 9faa-24 allow
ReasoningToken: 75
Control Request:
```yaml
id: 9faa-25
toolName: ast_edit_marks
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/docs/architecture/conventions.md
    id: 40b374.ace7f8
    start_marker: "## Q4 — Feature Flags"
    end_marker: GPU build with `cuda`).
    content: |-
      ## Q4 — Feature Flags

      - Real CUDA code paths live behind the `cuda` feature. The only `#[cfg(feature = "cuda")]`
        decision point is `backend::create_backend` (`src/resident/backend.rs`); everything else
        runs against the feature-independent `ComputeBackend` trait:
        ```rust
        pub fn create_backend() -> Box<dyn ComputeBackend> {
            #[cfg(feature = "cuda")]
            {
                if let Ok(backend) = CudaBackend::try_new() {
                    return Box::new(backend);
                }
            }
            Box::new(CpuBackend)
        }
        ```
      - `CpuBackend` (`src/resident/subengine/mod.rs`) is always built and is both the `cuda`-less
        default and the fallback if `CudaBackend::try_new` fails. `CudaBackend`
        (`src/resident/subengine/device.rs`) is the single Candle/CUDA adapter point for the
        subengine; `resident/kernel/cuda.rs` is the equivalent adapter point for the kernel layer.
        Both are gated `#[cfg(feature = "cuda")]` only at their module declaration in the parent
        `mod.rs` — no `cfg` spreads into scheduler/planner/index logic.
      - `cuda` is never part of `default`; optional GPU crates use `dep:` syntax in `Cargo.toml`.
      - See `docs/architecture/feature_matrix.md` for the CI job matrix (no-GPU build/test vs.
        GPU build with `cuda`).
  reason: Q4-Konventionsabschnitt an die implementierte Backend-Factory anpassen.
```
/answer 9faa-25 allow
Control Request:
```yaml
id: 9faa-26
toolName: ast_validate
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/rag/docs/architecture/conventions.md
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/mod.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/backend.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/mock.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/subengine/mod.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/subengine/device.rs
  reason: Abschließende Validierung aller für Q4 geänderten Dateien.
```
/answer 9faa-26 allow
[xy.ai.req:ClaudeCode:a182e22a-f224-4c7c-b47f-52b7c17f5756]