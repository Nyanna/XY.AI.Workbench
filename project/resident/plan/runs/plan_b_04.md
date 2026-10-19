Setze B4 um.

[include](/home/user/xyan/xy.ai.workbench/project/resident/index.md)
[include](/home/user/xyan/xy.ai.workbench/project/resident/plan/plan_a_00_overview.md)

- minimiere die Kommentare und keine direkte Referenzierung von Planelementen in Kommentaren
- Projektdokumente immer in englisch ausgeben

SystemInit:  id=e5d4d580-c47a-4629-8b94-e1a23fb4ca60, cwd=/home/user/xyan/xy.ai.workbench/project, model=claude-sonnet-5-5
Control Request:
```yaml
id: 78c8-17
toolName: list
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag
    pattern: \.(rs|toml)$
  reason: Inspect existing Rust workspace layout to place B4 code
```
/answer 78c8-17 allow
ReasoningToken: 50
Control Request:
```yaml
id: 78c8-18
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag
      entries:
      - ".: (1 file)"
      - Cargo.toml
      - "./.bin/debug/build/libsqlite3-sys-5732091bfd9cbcf1/out: (1 file)"
      - bindgen.rs
      - "./.bin/debug/build/serde-045d6d86c89f5c13/out: (1 file)"
      - private.rs
      - "./.bin/debug/build/serde-882976cf9f9bdd13/out: (1 file)"
      - private.rs
      - "./.bin/debug/build/serde_core-4954989e5d8a5802/out: (1 file)"
      - private.rs
      - "./.bin/debug/build/serde_core-7bded54902a606b3/out: (1 file)"
      - private.rs
      - "./.bin/release/build/libsqlite3-sys-da0791f2ea192201/out: (1 file)"
      - bindgen.rs
      - "./.bin/release/build/serde-206768b153b78fd3/out: (1 file)"
      - private.rs
      - "./.bin/release/build/serde-423d3e60c1cd2ea8/out: (1 file)"
      - private.rs
      - "./.bin/release/build/serde_core-81444f961b76707e/out: (1 file)"
      - private.rs
      - "./.bin/release/build/serde_core-c1bd3ded7cc1e06e/out: (1 file)"
      - private.rs
      - "./.cargo: (1 file)"
      - config.toml
      - "./.xyrag/layers/trigram/mirror: (1 file)"
      - Cargo.toml
      - "./.xyrag/layers/trigram/mirror/benches: (1 file)"
      - trie_bench.rs
      - "./.xyrag/layers/trigram/mirror/src: (1 file)"
      - lib.rs
      - "./.xyrag/layers/trigram/mirror/src/bin: (1 file)"
      - xyrag.rs
      - "./.xyrag/layers/trigram/mirror/src/core: (9 files)"
      - engine.rs
      - "./.xyrag/layers/trigram/mirror/src/layers: (5 files)"
      - dir_cache.rs
      - "./.xyrag/layers/trigram/mirror/src/layers/trigram: (7 files)"
      - index.rs
      - "./.xyrag/layers/trigram/mirror/tests: (1 file)"
      - trigram_e2e.rs
      - "./benches: (1 file)"
      - trie_bench.rs
      - "./src: (1 file)"
      - lib.rs
      - "./src/bin: (1 file)"
      - xyrag.rs
      - "./src/core: (9 files)"
      - engine.rs
      - "./src/layers: (5 files)"
      - dir_cache.rs
      - "./src/layers/trigram: (7 files)"
      - index.rs
      - "./src/resident: (5 files)"
      - backend.rs
      - "./src/resident/facade: (1 file)"
      - mod.rs
      - "./src/resident/index: (1 file)"
      - mod.rs
      - "./src/resident/kernel: (2 files)"
      - cuda.rs
      - "./src/resident/manager: (1 file)"
      - mod.rs
      - "./src/resident/memory: (6 files)"
      - dtype.rs
      - "./src/resident/op: (3 files)"
      - lease.rs
      - "./src/resident/planner: (1 file)"
      - mod.rs
      - "./src/resident/precision: (1 file)"
      - mod.rs
      - "./src/resident/processor: (1 file)"
      - mod.rs
      - "./src/resident/queue: (5 files)"
      - mod.rs
      - "./src/resident/subengine: (2 files)"
      - device.rs
      - "./tests: (1 file)"
      - trigram_e2e.rs
      warning: Result reduced because the total number of entries (92) exceeds the limit of 50. Narrow down the result using the 'pattern' regular expression parameter.
```
/answer 78c8-18 allow
Control Request:
```yaml
id: 78c8-19
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/docs/architecture/conventions.md
    ids: []
  reason: Read conventions
```
/answer 78c8-19 allow
Control Request:
```yaml
id: 78c8-1a
result:
  structuredContent:
    results:
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
      errors:
      - "ids [] resolved to no node; redirected to ast_outline, returning the full outline instead."
```
/answer 78c8-1a allow
Control Request:
```yaml
id: 78c8-1b
toolName: list
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident
    pattern: \.rs$
  reason: List resident module files
```
/answer 78c8-1b allow
Control Request:
```yaml
id: 78c8-1c
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident
      entries:
      - ".:"
      - backend.rs
      - error.rs
      - mock.rs
      - mod.rs
      - telemetry.rs
      - "./facade:"
      - mod.rs
      - "./index:"
      - mod.rs
      - "./kernel:"
      - cuda.rs
      - mod.rs
      - "./manager:"
      - mod.rs
      - "./memory:"
      - dtype.rs
      - freelist.rs
      - layout.rs
      - manager.rs
      - mod.rs
      - region.rs
      - "./op:"
      - lease.rs
      - mod.rs
      - requirements.rs
      - "./planner:"
      - mod.rs
      - "./precision:"
      - mod.rs
      - "./processor:"
      - mod.rs
      - "./queue:"
      - mod.rs
      - node.rs
      - queue.rs
      - state.rs
      - task.rs
      - "./subengine:"
      - device.rs
      - mod.rs
```
/answer 78c8-1c allow
Control Request:
```yaml
id: 78c8-1d
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/index/mod.rs
    ids: []
  - path: /home/user/xyan/xy.ai.workbench/rag/Cargo.toml
    ids: []
  reason: Read index stub and Cargo.toml
```
/answer 78c8-1d allow
Control Request:
```yaml
id: 78c8-1e
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/index/mod.rs
      nodes:
      - id: ModuleDoc
        signature: "//! Index trait, Scope, AccessMethod, Stamp."
      errors:
      - "ids [] resolved to no node; redirected to ast_outline, returning the full outline instead."
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
      - id: adh0oF|KIOs0a
        signature: "[features]"
      - id: UeF6gB|FaChiw
        signature: "[dev-dependencies]"
      - id: 9OxNvg|ELdnWn
        signature: "[[bench]]"
      errors:
      - "ids [] resolved to no node; redirected to ast_outline, returning the full outline instead."
```
/answer 78c8-1e allow
Control Request:
```yaml
id: 78c8-1f
toolName: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src
    pattern: mod resident|pub mod |ResidentError|IndexUnavailable|StaleMapping
    include:
    - '*.rs'
    limit: 30
  reason: Find module wiring and error types
```
/answer 78c8-1f allow
Control Request:
```yaml
id: 78c8-20
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src
      matches:
      - path: lib.rs
        matches:
        - lineno: 5
          match: pub mod core;
        - lineno: 6
          match: pub mod layers;
        - lineno: 7
          match: pub mod resident;
      - path: resident/mod.rs
        matches:
        - lineno: 2
          match: pub mod backend;
        - lineno: 3
          match: pub mod error;
        - lineno: 4
          match: pub mod telemetry;
        - lineno: 6
          match: pub mod mock;
        - lineno: 7
          match: pub mod queue;
        - lineno: 8
          match: pub mod memory;
        - lineno: 9
          match: pub mod op;
        - lineno: 10
          match: pub mod index;
        - lineno: 11
          match: pub mod kernel;
        - lineno: 12
          match: pub mod precision;
        - lineno: 13
          match: pub mod facade;
        - lineno: 14
          match: pub mod processor;
        - lineno: 15
          match: pub mod planner;
        - lineno: 16
          match: pub mod subengine;
        - lineno: 17
          match: pub mod manager;
      - path: resident/queue/mod.rs
        matches:
        - lineno: 15
          match: pub mod task;
      - path: resident/error.rs
        matches:
        - lineno: 18
          match: "IndexUnavailable { scope: String },"
        - lineno: 27
          match: "self, IndexError::IndexUnavailable { .. } | IndexError::ArenaExhausted { .. }"
        - lineno: 37
          match: "StaleMapping { scope: String },"
        - lineno: 44
          match: "self, ManagerError::StaleMapping { .. } | ManagerError::StaleRegion { .. }"
        - lineno: 115
          match: "pub enum ResidentError {"
        - lineno: 127
          match: "impl IsRetryable for ResidentError {"
        - lineno: 130
          match: "ResidentError::Index(e) => e.is_retryable(),"
        - lineno: 131
          match: "ResidentError::Precision(e) => e.is_retryable(),"
        - lineno: 132
          match: "ResidentError::Planner(e) => e.is_retryable(),"
        - lineno: 133
          match: "ResidentError::Config(e) => e.is_retryable(),"
        - lineno: 134
          match: "ResidentError::Capability(e) => e.is_retryable(),"
      warning: "Limit of 30 matches reached; further results may exist. Narrow the pattern, directory or include/exclude filters, or raise limit."
```
/answer 78c8-20 allow
Control Request:
```yaml
id: 78c8-21
toolName: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag
    pattern: "^(tokio|tracing|thiserror|xxhash|bytes|parking_lot|\\[features\\]|default|cuda|\\[dev-dependencies\\])"
    include:
    - Cargo.toml
    exclude:
    - .xyrag/**
    - .bin/**
    limit: 30
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src/resident
    pattern: "^(use |pub fn|pub mod|#\\[cfg)"
    include:
    - mock.rs
    - mod.rs
    limit: 30
  reason: Check available deps and mock conventions
```
/answer 78c8-21 allow
Control Request:
```yaml
id: 78c8-22
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag
      matches:
      - path: .xyrag/layers/trigram/mirror/Cargo.toml
        matches:
        - lineno: 16
          match: tokio version 1 features rt multi thread macros sync time
        - lineno: 17
          match: tokio util 0 7
      - path: Cargo.toml
        matches:
        - lineno: 16
          match: "tokio = { version = \"1\", features = [\"rt-multi-thread\", \"macros\", \"sync\", \"time\"] }"
        - lineno: 17
          match: tokio-util = "0.7"
        - lineno: 32
          match: thiserror = "2.0.20"
        - lineno: 33
          match: tracing = "0.1.44"
        - lineno: 34
          match: "tracing-subscriber = { version = \"0.3.23\", features = [\"env-filter\"] }"
        - lineno: 35
          match: tracing-appender = "0.2.3"
        - lineno: 38
          match: parking_lot = "0.12.5"
        - lineno: 48
          match: "cudarc = { version = \"=0.19.9\", optional = true }"
        - lineno: 50
          match: "[features]"
        - lineno: 51
          match: "default = []"
        - lineno: 53
          match: "cuda = [\"candle\", \"dep:candle-transformers\", \"dep:cudarc\", \"candle-core/cuda\", \"candle-transformers/cuda\"]"
        - lineno: 57
          match: "[dev-dependencies]"
        - lineno: 64
          match: "tokio = { version = \"1\", features = [\"test-util\"] }"
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src/resident
      matches:
      - path: mod.rs
        matches:
        - lineno: 2
          match: pub mod backend;
        - lineno: 3
          match: pub mod error;
        - lineno: 4
          match: pub mod telemetry;
        - lineno: 5
          match: "#[cfg(any(test, feature = \"mock\"))]"
        - lineno: 6
          match: pub mod mock;
        - lineno: 7
          match: pub mod queue;
        - lineno: 8
          match: pub mod memory;
        - lineno: 9
          match: pub mod op;
        - lineno: 10
          match: pub mod index;
        - lineno: 11
          match: pub mod kernel;
        - lineno: 12
          match: pub mod precision;
        - lineno: 13
          match: pub mod facade;
        - lineno: 14
          match: pub mod processor;
        - lineno: 15
          match: pub mod planner;
        - lineno: 16
          match: pub mod subengine;
        - lineno: 17
          match: pub mod manager;
      - path: precision/mod.rs
        matches:
        - lineno: 2
          match: use std::collections::BTreeSet;
      - path: queue/mod.rs
        matches:
        - lineno: 15
          match: pub mod task;
      - path: kernel/mod.rs
        matches:
        - lineno: 3
          match: "#[cfg(feature = \"cuda\")]"
        - lineno: 5
          match: "#[cfg(feature = \"cuda\")]"
      - path: subengine/mod.rs
        matches:
        - lineno: 2
          match: use crate::resident::backend::ComputeBackend;
        - lineno: 11
          match: "#[cfg(feature = \"cuda\")]"
        - lineno: 13
          match: "#[cfg(feature = \"cuda\")]"
      - path: memory/mod.rs
        matches:
        - lineno: 3
          match: pub mod dtype;
        - lineno: 4
          match: pub mod freelist;
        - lineno: 5
          match: pub mod layout;
        - lineno: 6
          match: pub mod manager;
        - lineno: 7
          match: pub mod region;
      - path: mock.rs
        matches:
        - lineno: 22
          match: use crate::resident::backend::ComputeBackend;
        - lineno: 141
          match: "#[cfg(loom)]"
      warning: "Limit of 30 matches reached; further results may exist. Narrow the pattern, directory or include/exclude filters, or raise limit."
```
/answer 78c8-22 allow
Control Request:
```yaml
id: 78c8-23
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/error.rs
    ids: []
  reason: See IndexError variants
```
/answer 78c8-23 allow
Control Request:
```yaml
id: 78c8-24
result:
  structuredContent:
    results:
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
      errors:
      - "ids [] resolved to no node; redirected to ast_outline, returning the full outline instead."
```
/answer 78c8-24 allow
ReasoningToken: 88
Control Request:
```yaml
id: 78c8-25
toolName: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag
    pattern: xxhash|bytes|memmap|^mock
    include:
    - Cargo.toml
    exclude:
    - .xyrag/**
    - .bin/**
  reason: Check xxhash dependency availability
```
/answer 78c8-25 allow
Control Request:
```yaml
id: 78c8-26
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag
      matches:
      - path: Cargo.toml
        matches:
        - lineno: 41
          match: memmap2 = "0.9.3"
        - lineno: 54
          match: "mock = []"
        - lineno: 60
          match: mockall = "0.15.0"
```
/answer 78c8-26 allow
Control Request:
```yaml
id: 78c8-27
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/Cargo.toml
    ids:
    - zHTLlI|2t4fSB
  reason: Read dependencies section
```
/answer 78c8-27 allow
Control Request:
```yaml
id: 78c8-28
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
```
/answer 78c8-28 allow
Control Request:
```yaml
id: 78c8-29
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/Cargo.toml
    id: zHTLlI|2t4fSB
    old_text: |
      memmap2 = "0.9.3"
    new_text: |
      memmap2 = "0.9.3"
      xxhash-rust = { version = "0.8", features = ["xxh3"] }
  reason: Add xxh3 hashing dependency for fallback stamps
```
/answer 78c8-29 allow
ReasoningToken: 2424
Control Request:
```yaml
id: 78c8-2a
toolName: ast_create
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/index/mod.rs
    overwrite: true
    source: |
      //! Index trait, Scope, AccessMethod, Stamp.
      //!
      //! The contract is independent of the core: an index reports changes and
      //! delivers structure and stamps; it knows no geometry and resolves no scope.
      //!
      //! Write order is binding: data first, then stamp, then signal.
      //! `read` may block on page faults and disk I/O; callers use a blocking pool.

      pub mod memory;
      pub mod stamp;

      use std::ops::Range;
      use std::sync::Arc;

      use tokio::sync::watch;

      pub use stamp::{content_hash, FileStamp, StampCounter, Verdict, RACY_WINDOW_NS};

      #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
      pub struct EntryId(u64);

      impl EntryId {
          pub const fn new(raw: u64) -> Self {
              Self(raw)
          }
          pub const fn get(self) -> u64 {
              self.0
          }
      }

      #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
      pub struct NodeId(pub u64);

      #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
      pub struct CellId(pub u64);

      /// Opaque change marker. Compare with `!=` only; ordering is deliberately absent.
      #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
      pub struct Stamp(u64);

      impl Stamp {
          pub(crate) const fn from_raw(raw: u64) -> Self {
              Self(raw)
          }
          /// Raw value, e.g. as `watch` watermark or for persistence.
          pub const fn raw(self) -> u64 {
              self.0
          }
      }

      #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
      #[non_exhaustive]
      pub enum AccessMethod {
          Centroid,
          Prefix,
          Subtree,
          Range,
      }

      #[derive(Clone, Debug, PartialEq)]
      pub enum Scope {
          All,
          Range(Range<u64>),
          Set(EntrySet),
          Subtree(NodeId),
          Prefix(Vec<u8>),
          Centroid { k: u32, radius: Option<f32> },
      }

      /// Compact set of entry ids stored as sorted, merged, half-open ranges.
      #[derive(Clone, Debug, Default, PartialEq, Eq)]
      pub struct EntrySet {
          ranges: Vec<Range<u64>>,
      }

      impl EntrySet {
          pub fn new() -> Self {
              Self::default()
          }

          pub fn from_ranges(ranges: impl IntoIterator<Item = Range<u64>>) -> Self {
              let mut v: Vec<Range<u64>> = ranges.into_iter().filter(|r| r.start < r.end).collect();
              v.sort_by_key(|r| r.start);
              let mut merged: Vec<Range<u64>> = Vec::with_capacity(v.len());
              for r in v {
                  match merged.last_mut() {
                      Some(last) if r.start <= last.end => last.end = last.end.max(r.end),
                      _ => merged.push(r),
                  }
              }
              Self { ranges: merged }
          }

          pub fn from_ids(ids: impl IntoIterator<Item = EntryId>) -> Self {
              Self::from_ranges(ids.into_iter().map(|id| id.0..id.0.saturating_add(1)))
          }

          pub fn ranges(&self) -> &[Range<u64>] {
              &self.ranges
          }

          pub fn is_empty(&self) -> bool {
              self.ranges.is_empty()
          }

          pub fn len(&self) -> u64 {
              self.ranges.iter().map(|r| r.end - r.start).sum()
          }

          pub fn contains(&self, id: EntryId) -> bool {
              let i = self.ranges.partition_point(|r| r.end <= id.0);
              self.ranges.get(i).is_some_and(|r| r.start <= id.0)
          }

          pub fn iter(&self) -> impl Iterator<Item = EntryId> + '_ {
              self.ranges.iter().flat_map(|r| r.clone().map(EntryId))
          }
      }

      /// Variable-length vector region of one entry; never a copy of backing storage.
      #[derive(Clone, Debug)]
      pub enum EntryData<'a> {
          Borrowed(&'a [u8]),
          Shared(Arc<[u8]>),
      }

      impl EntryData<'_> {
          pub fn as_bytes(&self) -> &[u8] {
              match self {
                  Self::Borrowed(b) => b,
                  Self::Shared(a) => a,
              }
          }
      }

      /// Summary structure such as a centroid table, exposed as vector data.
      #[derive(Clone, Debug)]
      pub struct SummaryRef(pub Arc<[u8]>);

      #[derive(Clone, Copy, Debug, PartialEq, Eq)]
      #[non_exhaustive]
      pub enum HashAlgo {
          Xxh3_64,
          Xxh3_128,
          Blake3,
      }

      /// Binding between index and model; persistence format is owned by the index.
      #[derive(Clone, Debug, PartialEq, Eq)]
      pub struct IndexHeader {
          pub model_id: String,
          pub dim: u32,
          pub high_water: u64,
          pub hash_algo: HashAlgo,
          pub hash_version: u32,
      }

      pub trait Index {
          /// `None` = entry removed.
          fn stamp(&self, entry: EntryId) -> Option<Stamp>;

          /// Must not touch vector data.
          fn stamps(&self) -> impl Iterator<Item = (EntryId, Stamp)>;

          /// Missing entries yield empty data. May block on I/O.
          fn read(&self, entry: EntryId) -> EntryData<'_>;

          /// Serializes writers (last writer wins in dequeue order). Returns the stamp
          /// of this very write; callers register it as known to suppress their echo.
          fn apply(&self, entry: EntryId, data: &[u8]) -> Stamp;

          /// Level signal carrying the latest stamp watermark. Consume with
          /// `borrow_and_update()`; coalesced signals are harmless.
          fn changed(&self) -> watch::Receiver<u64>;

          fn capabilities(&self) -> &[AccessMethod];

          fn children(&self, node: NodeId) -> EntrySet;

          fn members(&self, cell: CellId) -> EntrySet;

          fn summary(&self, kind: AccessMethod) -> Option<SummaryRef>;

          /// Hint only; may be a no-op.
          fn prefetch(&self, set: &EntrySet);
      }
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/index/stamp.rs
    source: |
      //! Stamp generation and the fallback chain for file-backed indexes:
      //! stamp -> size+mtime -> content hash.

      use std::sync::atomic::{AtomicU64, Ordering};

      use super::Stamp;

      /// Conservative racy window (FAT worst case).
      pub const RACY_WINDOW_NS: u64 = 2_000_000_000;

      /// Monotonic stamp source. Uniqueness only needs `Relaxed`; visibility of the
      /// data is established by the writer's lock/release when publishing.
      #[derive(Debug)]
      pub struct StampCounter(AtomicU64);

      impl StampCounter {
          pub const fn new() -> Self {
              Self(AtomicU64::new(1))
          }

          /// Resume after restart: `max(stored, file_max) + 1`.
          pub fn resume(stored_high_water: u64, file_max: u64) -> Self {
              Self(AtomicU64::new(stored_high_water.max(file_max) + 1))
          }

          pub fn next(&self) -> Stamp {
              Stamp::from_raw(self.0.fetch_add(1, Ordering::Relaxed))
          }

          pub fn high_water(&self) -> u64 {
              self.0.load(Ordering::Relaxed) - 1
          }
      }

      impl Default for StampCounter {
          fn default() -> Self {
              Self::new()
          }
      }

      #[derive(Clone, Copy, Debug, PartialEq, Eq)]
      pub struct FileStamp {
          pub size: u64,
          pub mtime_ns: u64,
          pub ctime_ns: Option<u64>,
          /// Reference time of the scan that recorded this stamp.
          pub scan_ns: u64,
      }

      #[derive(Clone, Copy, Debug, PartialEq, Eq)]
      pub enum Verdict {
          Unchanged,
          NeedsHash,
      }

      impl FileStamp {
          /// Racy: mtime lies within the timestamp resolution before the scan, so a
          /// later write could have left it unchanged.
          pub fn is_racy(&self) -> bool {
              self.mtime_ns.saturating_add(RACY_WINDOW_NS) >= self.scan_ns
          }

          pub fn verdict(&self, current: &FileStamp) -> Verdict {
              let same = self.size == current.size
                  && self.mtime_ns == current.mtime_ns
                  && self.ctime_ns == current.ctime_ns;
              if same && !self.is_racy() {
                  Verdict::Unchanged
              } else {
                  tracing::debug!(racy = self.is_racy(), same, "fallback stamp requires content hash");
                  Verdict::NeedsHash
              }
          }
      }

      /// xxh3_64; algorithm and version belong in the index header.
      pub fn content_hash(bytes: &[u8]) -> u64 {
          xxhash_rust::xxh3::xxh3_64(bytes)
      }
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/index/memory.rs
    source: |
      //! Reference in-memory index; builds without `cuda`/`candle`.

      use std::collections::BTreeMap;
      use std::sync::Arc;

      use parking_lot::Mutex;
      use tokio::sync::watch;

      use super::{
          AccessMethod, CellId, EntryData, EntryId, EntrySet, Index, NodeId, Stamp, StampCounter,
          SummaryRef,
      };

      struct Inner {
          entries: BTreeMap<EntryId, (Stamp, Arc<[u8]>)>,
      }

      pub struct InMemoryIndex {
          inner: Mutex<Inner>,
          counter: StampCounter,
          tx: watch::Sender<u64>,
      }

      impl InMemoryIndex {
          pub fn new() -> Self {
              Self::with_counter(StampCounter::new())
          }

          pub fn with_counter(counter: StampCounter) -> Self {
              let (tx, _) = watch::channel(counter.high_water());
              Self {
                  inner: Mutex::new(Inner { entries: BTreeMap::new() }),
                  counter,
                  tx,
              }
          }

          pub fn high_water(&self) -> u64 {
              self.counter.high_water()
          }

          /// Removal consumes a stamp so that re-adding the same id yields a new one.
          pub fn remove(&self, entry: EntryId) -> Option<Stamp> {
              let mut g = self.inner.lock();
              g.entries.remove(&entry)?;
              let stamp = self.counter.next();
              self.tx.send_replace(stamp.raw());
              Some(stamp)
          }
      }

      impl Default for InMemoryIndex {
          fn default() -> Self {
              Self::new()
          }
      }

      impl Index for InMemoryIndex {
          fn stamp(&self, entry: EntryId) -> Option<Stamp> {
              self.inner.lock().entries.get(&entry).map(|(s, _)| *s)
          }

          fn stamps(&self) -> impl Iterator<Item = (EntryId, Stamp)> {
              let snapshot: Vec<_> = self.inner.lock().entries.iter().map(|(id, (s, _))| (*id, *s)).collect();
              snapshot.into_iter()
          }

          fn read(&self, entry: EntryId) -> EntryData<'_> {
              match self.inner.lock().entries.get(&entry) {
                  Some((_, data)) => EntryData::Shared(Arc::clone(data)),
                  None => EntryData::Borrowed(&[]),
              }
          }

          fn apply(&self, entry: EntryId, data: &[u8]) -> Stamp {
              let mut g = self.inner.lock();
              // Stamp is drawn inside the write lock, immediately before publication.
              let stamp = self.counter.next();
              g.entries.insert(entry, (stamp, Arc::from(data)));
              self.tx.send_replace(stamp.raw());
              drop(g);
              tracing::debug!(entry = entry.get(), stamp = stamp.raw(), "apply");
              stamp
          }

          fn changed(&self) -> watch::Receiver<u64> {
              self.tx.subscribe()
          }

          fn capabilities(&self) -> &[AccessMethod] {
              &[AccessMethod::Range]
          }

          fn children(&self, _node: NodeId) -> EntrySet {
              EntrySet::new()
          }

          fn members(&self, cell: CellId) -> EntrySet {
              if cell.0 != 0 {
                  return EntrySet::new();
              }
              EntrySet::from_ids(self.inner.lock().entries.keys().copied())
          }

          fn summary(&self, _kind: AccessMethod) -> Option<SummaryRef> {
              None
          }

          fn prefetch(&self, _set: &EntrySet) {}
      }
  reason: "Add xxhash dep, implement Index trait, types, stamp mechanics, in-memory reference index"
```
/answer 78c8-2a allow
Control Request:
```yaml
id: 78c8-2b
toolName: ast_validate
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/index/mod.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/index/stamp.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/index/memory.rs
  - /home/user/xyan/xy.ai.workbench/rag/Cargo.toml
  reason: Syntax check of new files
```
/answer 78c8-2b allow
Result Stats: id=e5d4d580-c47a-4629-8b94-e1a23fb4ca60, total: 37492, in: 14, out: 10657, reason: 0, read: 129126, write: 26821, start: 1792311961292, end: 1792311966728, time: 5376, total: 65077, io: 0, control: 60, sent: 0, recv: 136
B4 ist umgesetzt, aber ich habe es nicht mit `cargo build` gebaut. Die Dateien haben nur den Syntaxcheck bestanden.

Neu im Rust-Projekt unter `rag/src/resident/index/`:
- **`mod.rs`:** der `Index`-Trait und alle Begleittypen (`EntryId`, `NodeId`, `CellId`, `Stamp`, `AccessMethod`, `Scope`, `EntrySet`, `EntryData`, `SummaryRef`, `IndexHeader`). `EntrySet` ist als Ranges gespeichert, `Stamp` hat nur `==`/`!=`.
- **`stamp.rs`:**
  - `StampCounter` mit Wiederaufnahme über `max(stored, file_max) + 1`.
  - `FileStamp` mit Racy-Prüfung (2 s) und `Verdict`.
  - `content_hash` über xxh3_64.
- **`memory.rs`:** `InMemoryIndex` als Referenz-Testdouble. `apply` läuft unter einem Lock, der Stamp wird direkt vor dem Veröffentlichen vergeben und danach das `watch<u64>`-Signal gesendet. `remove` verbraucht ebenfalls einen Stamp.
- **`Cargo.toml`:** Abhängigkeit `xxhash-rust` (Feature `xxh3`) ergänzt.

Abweichungen und offene Punkte:
- `EntryData` hat zwei Varianten (`Borrowed` und `Shared(Arc)`), weil der Mutex im Testdouble keine echte Referenz erlaubt.
- Die Sidecar-Variante und die Parquet-Projektion für `stamps()` sind nicht umgesetzt. Beides bleibt bei G2 und A3.
- Tests habe ich nicht geschrieben, weil keine permanente Testsuite verwendet wird.
