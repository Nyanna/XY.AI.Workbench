Setze Q3 um.

[include](/home/user/xyan/xy.ai.workbench/project/resident/index.md)
[include](/home/user/xyan/xy.ai.workbench/project/resident/plan/plan_a_00_overview.md)

- minimiere die Kommentare und keine direkte Referenzierung von Planelementen in Kommentaren
- Scope-Charta erstellt unter `/home/user/xyan/xy.ai.workbench/rag/docs/architecture/scope_charta.md`
- Projektdokumente immer in englisch ausgeben

# Umsetzungsplan Q3 · Teststrategie & Mocks

**Querschnitt Q — verbindliche Konvention.** Nach A4 initial zu fixieren, je Gruppe fortzuschreiben.

## 1. Zweck

Einheitliche Test-/Mock-Strategie, die (a) die gesamte Logik **ohne GPU** testbar macht, (b) Nebenläufigkeit/CAS-Protokolle abdeckt, (c) deterministische Sync-/Kalibrierungs-Tests ermöglicht, und (d) je Paket eine Testmatrix vorgibt.

## 2. Rechercheergebnis (destilliert aus `context_h_q.md` Q3)

- **Mocks:** `mockall` (`#[automock]` auf Traits) für Erwartungs-/Aufruf-Verifikation; für **zustandsbehaftete** Kernel-/Mess-Mocks sind **manuelle Fakes** klarer (`FakeKernel` mit programmierbaren Ergebnissen, `FakeMeasure` mit Sequenz/RNG).
- **Nebenläufigkeit:** `loom` (erschöpfende Interleavings unter C11, `loom::sync` via `cfg(loom)`) für kleine CAS-Protokolle (Append/Claim/Cancel/Unlink); `shuttle` (randomisierte Schedules) für größere Szenarien; ergänzend `proptest` für Sequenzen.
- **GPU-frei:** Backend-Trait (`ComputeBackend`) mit CPU-/Mock-Impl; Feature `cuda` aktiviert nur die CUDA-Impl; Tests laufen **standardmäßig ohne** GPU.
- **Deterministisch:** virtuelle Zeit (`tokio::time::pause`, Feature `test-util`) oder eigener `Clock`-Trait; Schritt-Hooks/Failpoints zum Erzwingen von Interleavings (ABA: Stamp/Generation gezielt zwischen Lesen und CAS ändern); zustandsbasierte Asserts statt Timing; feste Seeds.
- **Verrauschte Messwerte:** Mess-Mock hinter Trait: Basiswert + Rauschen aus seeded RNG (`rand`, `StdRng::seed_from_u64`); Varianten konstant/Ausreißer/Drift. Tests: Abbruch an Obergrenze → `unstable`, Konvergenz bei geringem Rauschen, Verhalten bei Ausreißern; mehrere Seeds.

## 3. Verbindliche Konvention (Umsetzung)

**Abstraktions-Traits (alle mit Mock/Fake):**
- `ComputeBackend` — CPU/Mock vs. CUDA (hinter Feature `cuda`); die gesamte Orchestrierung läuft gegen das Trait.
- **Kernel-Trait** (B5, `resident_index.md`/`resident_subengine.md` §4.5): Batch-Start, Batch-Ende, Ergebnisse, geschriebene Bereiche, Fortschrittszähler → `FakeKernel` mit skriptbaren Ergebnissen.
- **Mess-Trait** (F2, `resident_precision.md`: „Kalibrierung über ein Trait für die Messung abstrahiert") → `FakeMeasure` (Basiswert + seeded Rauschen, Varianten konstant/Ausreißer/Drift).
- `Clock`-Trait bzw. `tokio::time::pause` für TTL/Idle/Zeitscheiben.

**Nebenläufigkeit:** CAS-Protokolle der Queue (B1) mit `loom` erschöpfend; größere Interleavings mit `shuttle`; Sequenzen/Properties mit `proptest`. Failpoints für gezielte ABA-Szenarien.

**Testmatrizen je Paket (verbindlich):**
- **B1 Queue:** `claim`=CAS `Queued→Taken`; Fertigstellung `Taken→Done`; `cancel` nur auf `Queued`; Unlink ohne Durchläufer-Restart; iterativer Drop (keine Stack-Überläufe); Notify-Topologie (loom).
- **E5/E6 Index-Sync (Kernkandidaten):** Reihenfolge im Kommunikationsslot (Kernel-Writes → Auflösen → Updates), inkl. Ergebnisse auf frisch geschriebene Adressen; Ergebnisstatus (entfernt→verworfen, Update anstehend→`outdated`); Echo (eigener `apply` löst keinen Reload aus, fremder Stamp schon, zwei Manager konvergieren); verlorenes/zusammengefasstes Signal; Signal-Quittierung **vor** dem Scan; Stamp-Lesereihenfolge beim Laden; ABA (Entfernen+Neuanlage → anderer Stamp); CoW-Swap (altes Segment erst frei, wenn kein vor dem Swap eingereihter Request es referenziert); Gather-vs-Segment-Dichteschwelle; Single-Flight (ein Ladevorgang, Fehlerverteilung an alle Wartenden); `UnsupportedScope`.
- **F2–F5 Kalibrierung:** Matching (Klemmen, NaN-Fehler, Tie-Break zum genaueren, Einzelprofil `p=0.5`); Profilbildung (Spread-Schwellwert, Zusammenfassen redundanter Punkte, Positionsberechnung über Bogenlänge); Pareto-Auswahl (dominierte Kombinationen verworfen); Persistenz (Fingerprint-Wechsel→`Stale`, fehlende Datei→`Missing`); Stabilitätslogik (verrauschte Mock-Messwerte, Abbruch an Obergrenze→`unstable`).
- **C2/C3 Processor/Planner:** Worker-Loop-Priorität, Starved-Hint-Verfall, Callback+`cuEventQuery` ohne Doppelverarbeitung (gegen Mock-Backend), Planner-Expansion rein (gleiche Eingabe → gleiche Ops).

**Keine Bit-Gleichheit:** Präzisions-/Scorer-Tests prüfen **Schranken/Relationen**, nicht Gleichheit (`resident_precision.md`).

## 4. Umsetzungsschritte

1. Trait-Definitionen + Fakes (`ComputeBackend`, Kernel, Mess, `Clock`) in einem `testkit`/`test-support`-Modul (dev-dependency-sichtbar).
2. `loom`/`shuttle`/`proptest` als dev-dependencies; `cfg(loom)`-Shim für `std::sync`→`loom::sync` in den betroffenen Modulen.
3. Failpoint-/Step-Hooks an den Sync-Punkten (Kommunikationsslot, Lade-Pfad).
4. Testmatrizen aus §3 als konkrete Test-Dateien je Paket.
5. CI: Default-Lauf ohne `cuda` (alle Unit-Tests), siehe Q4.

## 5. Offene Verifikationen ([prüfen])

- `loom`/`shuttle`/`proptest`/`mockall`/`rand`-Versionen in der Projekt-`Cargo.toml`.
- `tokio` `test-util`-Feature aktiviert.

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
