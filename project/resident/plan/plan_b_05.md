Setze B5 um.

[include](/home/user/xyan/xy.ai.workbench/project/resident/index.md)
[include](/home/user/xyan/xy.ai.workbench/project/resident/plan/plan_a_00_overview.md)

- minimiere die Kommentare und keine direkte Referenzierung von Planelementen in Kommentaren
- Projektdokumente immer in englisch ausgeben

# Umsetzungsplan B5 · Kernel-Abstraktion

| | |
|---|---|
| **Paket** | B5 (Gruppe B) |
| **Abhängt von** | B2 (`Region`/Spans) |
| **Blockiert** | D6 (Kernels & Scorer), E4 (Manager-Disziplin), E5 (Kommunikationsslot), E8 (Kernel schreibt Segmente) |
| **Ergebnis** | Kernel-Trait + Mock-Vertrag: `SegmentDesc`, `SegmentLayout`, `KernelHit`, Kernel-Trait (Batch-Start/-Ende, Ergebnisse, geschriebene Bereiche, Fortschritt) + Mock |
| **Quellen** | `../resident_index.md` (Kernel-Schnittstelle, Rückabbildung), `../resident_subengine.md` §4.5, `../context/context_b.md` §B5 (21–23) |

---

## 1. Zweck & Abgrenzung

Der **Kernel** kennt nur **Segmente, Adressen und Offsets** — keine Entry-Identität, keine Traversierung, keine Kommunikation während der Berechnung. Er liefert `(Adresse/Offset, Score)` bzw. geschriebene Bereiche. Die **Rückabbildung Adresse → Entry** macht der **Manager** (E), nicht der Kernel.

B5 liefert eine **adress-/offset-agnostische** Trait-Abstraktion plus einen **handgeschriebenen Mock** für Tests ohne GPU. Reale CUDA-Implementierung (cudarc/NVRTC) ist D6/E hinter `feature = "cuda"`.

**Namensklärung:** B5 benutzt `SegmentLayout` (nicht `Layout`, das ist B2) für `Fixed{stride}|Variable{offsets}`.

---

## 2. Zu implementierende Typen

```rust
pub enum SegmentLayout {
    Fixed    { stride: u32 },        // Entry-Grenzen per Arithmetik
    Variable { offsets: OffsetRef }, // Offset-Tabelle (Prefix-Summen), vom Manager gebaut
}

pub struct SegmentDesc { pub base: u64 /* Arena-Offset */, pub count: u32, pub layout: SegmentLayout }
pub struct KernelHit   { pub offset: u64 /* Arena-Offset */, pub score: f32 }
pub struct WrittenRange { pub offset: u64, pub len: u64 }
pub struct RegionSpan   { pub offset: u64, pub len: u64 } // adress-agnostisch; Pointer-Auflösung NUR im Backend
```
- `OffsetRef` ist im Logik-Build ein Host-Slice/Region-Handle; im `cuda`-Build ein Device-Pointer-Äquivalent. B5 bleibt agnostisch (Trait nimmt **Spans**, keine Pointer).
- Eine **Kandidatenmenge** ist ein `Variable`-Layout; der Kernel rechnet darüber, egal ob die Tabelle auf kompakte Kopien oder bestehende Segmente zeigt.

## 3. Kernel-Trait (real + Mock)

```rust
pub trait Kernel: Send + Sync {
    fn begin(&self, batch: &BatchDesc) -> Result<BatchId, Error>;
    fn run(&self, batch: BatchId, input: &[RegionSpan]) -> Result<KernelOutput, Error>;
    fn progress(&self, batch: BatchId) -> u64;                   // monoton; Atomic/Event
    fn end(&self, batch: BatchId) -> Result<Vec<WrittenRange>, Error>; // geschriebene Bereiche
}

pub struct BatchDesc { /* Segmente (SegmentDesc), Query-Batch, ParamBlock-Verweis, Zeitbudget */ }
pub struct BatchId(u64);
pub enum KernelOutput { Hits(Vec<KernelHit>), Partial /* Bereich erschöpft, Rest folgt */ }
```
- **Zwei Kernel-Rollen** (beide über denselben Trait):
  - **Scoring (MaxSim):** `run` liefert `Hits`. MaxSim aggregiert pro Entry über `stride`/Offset-Tabelle, **ohne** Entry-Identität.
  - **Indexaufbau (Embedding):** Kernel schreibt per **Cursor** in einen vorab vergebenen Bereich und **allokiert nie**; bei Bereichsmangel `Partial` → Manager vergibt nächsten Bereich. `end` meldet belegte Bereiche.
- **Fortschritt:** `progress` monoton (Atomic im Logik-Mock; im Ringbuffer-Modus Zähler in gemapptem Host-Speicher — H1/E).
- **Keine Kommunikation während der Berechnung**; Mutation nur im Kommunikationsslot (E5).

## 4. Rückabbildung-Helfer (Zuständigkeit: Manager)

B5 stellt die reine Funktion bereit, die E8 nutzt (Kernel selbst traversiert nie):
```rust
// Fixed: addr → index per Division (addr - base) / stride
// Variable: offsets.len() == n+1; Entry i belegt [offsets[i], offsets[i+1])
fn entry_of(offsets: &[u64], addr: u64) -> Option<usize> {
    let i = offsets.partition_point(|&o| o <= addr); // erstes o > addr
    (i > 0 && i < offsets.len()).then(|| i - 1)
}
```
- Aufbau per exklusiver Präfixsumme (`scan`) in O(n); Suche O(log n) via `slice::partition_point`. Dynamisch alternativ `BTreeMap<u64, EntryId>` (`range(..=addr).next_back()`).

## 5. cudarc-Bezug (nur Doku für D6, hinter `feature = "cuda"`)

- cudarc 0.19.x: `CudaContext::new(ordinal)` → `default_stream()`/`new_stream()`; `nvrtc::compile_ptx` → `load_module` → `load_function`; `stream.launch_builder(&func).arg(..).launch(cfg)` (`unsafe`). Daten `CudaSlice`/`CudaView(Mut)`; `clone_htod`/`memcpy_htod`/`clone_dtoh`; Events für Fortschritt; `LaunchConfig{grid_dim,block_dim,shared_mem_bytes}`.
- **Kritisch:** candle nutzt dieselbe cudarc-Basis → **Stream-/Kontext-Gleichheit** beachten (A3-Prototyp). Kernel-Args müssen `DeviceRepr` sein; Slice-Lebensdauer bis `synchronize`; Kernel-Fehler erscheinen **asynchron** beim späteren Sync.
- `sm_75` als `--gpu-architecture` beim NVRTC-Compile.

## 6. Mock-Kernel (Pflicht, CI ohne GPU)

Handgeschriebenes Fake statt Mock-Framework:
- Deterministische Ausgabe als **reine Funktion der Eingabe** (z. B. xxh3 der Bytes → Score).
- Geschriebene Bereiche aus der Eingabe berechnet; `Partial` simulierbar (Bereich künstlich knapp).
- Fortschritt als `AtomicU64`; Fehlerinjektion `fail_after(n)`; Aufruf-Log `Mutex<Vec<Call>>` für Assertions.
- Trait-Generics (`fn run<K: Kernel>`) vermeiden Dispatch-Kosten; `Arc<dyn Kernel>` für Laufzeitwahl. `mockall` nur für reine Interaktions-Tests.

## 7. Fehler & Logging

- Q1: `TransferFailed` (H2D/D2H), `ArenaExhausted` (Bereichsmangel → aber Default ist `Partial`, nicht Fehler), Kernel-/Launch-Fehler.
- Q2: `tracing`-Span pro Batch (`BatchId`): begin/run/end, Hits-Zahl, WrittenRanges, Partial-Ereignisse.

## 8. Teststrategie (Q3, ohne GPU)

- **entry_of:** Fixed (Division) und Variable (partition_point) inkl. Rändern (addr == base, letzter Offset, out-of-range → None).
- **Mock-Determinismus:** gleiche Eingabe → gleiche Hits/Ranges.
- **Partial-Pfad:** knapper Bereich → `Partial`; Manager vergibt Folgebereich; Summe der WrittenRanges deckt Eingabe.
- **Fortschritt:** `progress` monoton steigend über `run`.
- **Fehlerinjektion:** `fail_after(n)` liefert `Err`; Batch sauber beendbar.
- **Adress-Agnostik:** Trait-API nennt keine Pointer; Spans genügen (Compile-Check).

## 9. Offene Punkte (→ A3 / D6 / E)

1. `OffsetRef` im `cuda`-Build (Device-Pointer) vs. Logik-Build (Host-Slice) — Abstraktion final.
2. MaxSim: Kachel-Kette (Candle-Ops) vs. fused NVRTC-Kernel messen (D6/A3-Prototyp).
3. cudarc-Version == candle-interne cudarc-Version (A3, sonst inkompatible Typen).
4. Rückabbildung dynamisch: `partition_point` (statisch) vs. `BTreeMap`/Intervall-Map (bei laufenden Änderungen).

## 10. Definition of Done

- `SegmentDesc`, `SegmentLayout`, `KernelHit`, `WrittenRange`, `RegionSpan`, `Kernel`-Trait, `entry_of`-Helfer implementiert.
- `MockKernel` mit Determinismus, Partial, Fortschritt, Fehlerinjektion, Aufruf-Log.
- Trait ist adress-/offset-agnostisch (keine Pointer in öffentlicher API); Rückabbildung als Manager-Aufgabe dokumentiert.
- Testmatrix §8 grün; baut ohne `cuda`/`candle`.
SystemInit:  id=de4cdb87-1974-4aa6-b605-b3aca180b2ee, cwd=/home/user/xyan/xy.ai.workbench/project, model=claude-sonnet-5-5
Text:
 Ich sehe mir zuerst den aktuellen Stand des Rust-Projekts an.
ReasoningToken: 52
Control Request:
```yaml
id: 78c8-2d
toolName: list
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag
    pattern: \.(rs|toml)$
  reason: "Inspect existing Rust workspace structure (crates, B2 Region types) before implementing B5"
```
/answer 78c8-2d allow
Control Request:
```yaml
id: 78c8-2e
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
      - "./src/resident/index: (3 files)"
      - memory.rs
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
      warning: Result reduced because the total number of entries (94) exceeds the limit of 50. Narrow down the result using the 'pattern' regular expression parameter.
```
/answer 78c8-2e allow
Control Request:
```yaml
id: 78c8-2f
toolName: list
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident
  reason: List all files in resident module
```
/answer 78c8-2f allow
Control Request:
```yaml
id: 78c8-30
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
      - memory.rs
      - mod.rs
      - stamp.rs
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
/answer 78c8-30 allow
Control Request:
```yaml
id: 78c8-31
toolName: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src
    pattern: mod |cuda|Region|Span
    include:
    - lib.rs
    - mod.rs
    exclude:
    - .xyrag/**
    limit: 50
  reason: Find module declarations and existing Region/Span types
```
/answer 78c8-31 allow
Control Request:
```yaml
id: 78c8-32
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
      - path: resident/index/mod.rs
        matches:
        - lineno: 9
          match: pub mod memory;
        - lineno: 10
          match: pub mod stamp;
      - path: resident/queue/mod.rs
        matches:
        - lineno: 12
          match: mod node;
        - lineno: 13
          match: mod queue;
        - lineno: 14
          match: mod state;
        - lineno: 15
          match: pub mod task;
      - path: resident/kernel/mod.rs
        matches:
        - lineno: 3
          match: "#[cfg(feature = \"cuda\")]"
        - lineno: 4
          match: mod cuda;
        - lineno: 5
          match: "#[cfg(feature = \"cuda\")]"
        - lineno: 6
          match: pub use cuda::*;
      - path: resident/subengine/mod.rs
        matches:
        - lineno: 3
          match: "/// CPU backend; default without `cuda`, and the fallback target if CUDA init fails."
        - lineno: 11
          match: "#[cfg(feature = \"cuda\")]"
        - lineno: 12
          match: mod device;
        - lineno: 13
          match: "#[cfg(feature = \"cuda\")]"
      - path: resident/memory/mod.rs
        matches:
        - lineno: 1
          match: //! Region/Layout/DType and MemoryManager trait.
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
        - lineno: 13
          match: "pub use region::{ArenaId, Region};"
      - path: resident/op/mod.rs
        matches:
        - lineno: 3
          match: mod lease;
        - lineno: 4
          match: mod requirements;
        - lineno: 179
          match: "mod tests {"
        - lineno: 183
          match: use crate::resident::memory::Region;
        - lineno: 240
          match: "let region = Region { arena: ArenaId(0), offset: 0, len: 8, gen: 1 };"
      - path: core/mod.rs
        matches:
        - lineno: 3
          match: pub mod engine;
        - lineno: 4
          match: pub mod executor;
        - lineno: 5
          match: pub mod layer;
        - lineno: 6
          match: pub mod persistence;
        - lineno: 7
          match: pub mod query;
        - lineno: 8
          match: pub mod registry;
      warning: "Limit of 50 matches reached; further results may exist. Narrow the pattern, directory or include/exclude filters, or raise limit."
```
/answer 78c8-32 allow
Control Request:
```yaml
id: 78c8-33
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel/mod.rs
    ids: []
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/error.rs
    ids: []
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/mock.rs
    ids: []
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel/cuda.rs
    ids: []
  reason: "Read kernel module stub, error and mock conventions"
```
/answer 78c8-33 allow
Control Request:
```yaml
id: 78c8-34
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel/mod.rs
      nodes:
      - id: ModuleDoc
        signature: "//! SegmentDesc/Layout/KernelHit, Kernel trait."
      - id: cuda
        signature: "#[cfg(feature = \"cuda\")]"
      - id: AeHRoB|0NKx9w
        signature: "#[cfg(feature = \"cuda\")]"
      errors:
      - "ids [] resolved to no node; redirected to ast_outline, returning the full outline instead."
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
      - id: 77cuty|ChE8yG
        signature: use crate::resident::backend::ComputeBackend;
      - id: MockBackend
        signature: "/// Identity backend: sufficient for orchestration tests that do not care about…"
        docstring: "Identity backend: sufficient for orchestration tests that do not care about num…"
      - id: impl_ComputeBackend_for_MockBackend
        signature: "impl ComputeBackend for MockBackend {"
        children:
        - id: impl_ComputeBackend_for_MockBackend.run_batch
          signature: "fn run_batch(&self, input: &[f32]) -> Vec<f32> {"
      - id: Kernel
        signature: "/// Kernel-level trait (batch lifecycle + results), abstracted so orchestration…"
        docstring: "Kernel-level trait (batch lifecycle + results), abstracted so orchestration/syn…"
        children:
        - id: Kernel.wXwSeB|s9F5Wn
          signature: type Hit;
        - id: Kernel.batch_start
          signature: fn batch_start(&mut self) -> u64;
        - id: Kernel.batch_end
          signature: "fn batch_end(&mut self, batch_id: u64);"
        - id: Kernel.results
          signature: "fn results(&self, batch_id: u64) -> Vec<Self::Hit>;"
        - id: Kernel.written_ranges
          signature: "fn written_ranges(&self, batch_id: u64) -> Vec<(usize, usize)>;"
        - id: Kernel.progress
          signature: "fn progress(&self, batch_id: u64) -> usize;"
      - id: FakeKernel
        signature: "/// Scriptable kernel fake: results/ranges/progress for a batch are programmed …"
        docstring: "Scriptable kernel fake: results/ranges/progress for a batch are programmed up f…"
      - id: impl_FakeKernel_H_
        signature: "impl<H> FakeKernel<H> {"
        children:
        - id: impl_FakeKernel_H_.new
          signature: "pub fn new() -> Self {"
        - id: impl_FakeKernel_H_.script
          signature: /// Programs the outcome of the next-started batch before it runs. pub fn scrip…
          docstring: Programs the outcome of the next-started batch before it runs.
      - id: impl_Kernel_for_FakeKernel_H_
        signature: "impl<H: Clone> Kernel for FakeKernel<H> {"
        children:
        - id: impl_Kernel_for_FakeKernel_H_.oGaivh|v5uMLN
          signature: type Hit = H;
        - id: impl_Kernel_for_FakeKernel_H_.batch_start
          signature: "fn batch_start(&mut self) -> u64 {"
        - id: impl_Kernel_for_FakeKernel_H_.batch_end
          signature: "fn batch_end(&mut self, _batch_id: u64) {"
        - id: impl_Kernel_for_FakeKernel_H_.results
          signature: "fn results(&self, batch_id: u64) -> Vec<H> {"
        - id: impl_Kernel_for_FakeKernel_H_.written_ranges
          signature: "fn written_ranges(&self, batch_id: u64) -> Vec<(usize, usize)> {"
        - id: impl_Kernel_for_FakeKernel_H_.progress
          signature: "fn progress(&self, batch_id: u64) -> usize {"
      - id: Measure
        signature: /// Measurement trait calibration logic depends on; `FakeMeasure` stands in for…
        docstring: Measurement trait calibration logic depends on; `FakeMeasure` stands in for real
        children:
        - id: Measure.sample
          signature: fn sample(&mut self) -> f64;
      - id: NoiseModel
        signature: /// Noise models covering the calibration-stability test matrix (F2-F5). pub en…
        docstring: Noise models covering the calibration-stability test matrix (F2-F5).
      - id: FakeMeasure
        signature: "/// Seeded-RNG measurement fake: `base` plus a `NoiseModel`-shaped perturbation…"
        docstring: "Seeded-RNG measurement fake: `base` plus a `NoiseModel`-shaped perturbation, so"
      - id: impl_FakeMeasure
        signature: "impl FakeMeasure {"
        children:
        - id: impl_FakeMeasure.new
          signature: "pub fn new(base: f64, noise: NoiseModel, seed: u64) -> Self {"
      - id: impl_Measure_for_FakeMeasure
        signature: "impl Measure for FakeMeasure {"
        children:
        - id: impl_Measure_for_FakeMeasure.sample
          signature: "fn sample(&mut self) -> f64 {"
      - id: sync
        signature: /// `std::sync` vs. `loom::sync` shim for CAS-protocol modules (B1-style queues…
        docstring: "`std::sync` vs. `loom::sync` shim for CAS-protocol modules (B1-style queues): s…"
      - id: sync_2
        signature: "#[cfg(not(loom))] pub mod sync {"
      errors:
      - "ids [] resolved to no node; redirected to ast_outline, returning the full outline instead."
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel/cuda.rs
      nodes:
      - id: ModuleDoc
        signature: //! Real CUDA kernels / scorer.
      errors:
      - "ids [] resolved to no node; redirected to ast_outline, returning the full outline instead."
```
/answer 78c8-34 allow
Control Request:
```yaml
id: 78c8-35
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel/mod.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/error.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/mock.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/memory/region.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/mod.rs
  reason: Get node IDs and structure
```
/answer 78c8-35 allow
Control Request:
```yaml
id: 78c8-36
result:
  structuredContent:
    results:
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
      - id: 77cuty|ChE8yG
        signature: use crate::resident::backend::ComputeBackend;
      - id: MockBackend
        signature: "/// Identity backend: sufficient for orchestration tests that do not care about…"
        docstring: "Identity backend: sufficient for orchestration tests that do not care about num…"
      - id: impl_ComputeBackend_for_MockBackend
        signature: "impl ComputeBackend for MockBackend {"
        children:
        - id: impl_ComputeBackend_for_MockBackend.run_batch
          signature: "fn run_batch(&self, input: &[f32]) -> Vec<f32> {"
      - id: Kernel
        signature: "/// Kernel-level trait (batch lifecycle + results), abstracted so orchestration…"
        docstring: "Kernel-level trait (batch lifecycle + results), abstracted so orchestration/syn…"
        children:
        - id: Kernel.wXwSeB|s9F5Wn
          signature: type Hit;
        - id: Kernel.batch_start
          signature: fn batch_start(&mut self) -> u64;
        - id: Kernel.batch_end
          signature: "fn batch_end(&mut self, batch_id: u64);"
        - id: Kernel.results
          signature: "fn results(&self, batch_id: u64) -> Vec<Self::Hit>;"
        - id: Kernel.written_ranges
          signature: "fn written_ranges(&self, batch_id: u64) -> Vec<(usize, usize)>;"
        - id: Kernel.progress
          signature: "fn progress(&self, batch_id: u64) -> usize;"
      - id: FakeKernel
        signature: "/// Scriptable kernel fake: results/ranges/progress for a batch are programmed …"
        docstring: "Scriptable kernel fake: results/ranges/progress for a batch are programmed up f…"
      - id: impl_FakeKernel_H_
        signature: "impl<H> FakeKernel<H> {"
        children:
        - id: impl_FakeKernel_H_.new
          signature: "pub fn new() -> Self {"
        - id: impl_FakeKernel_H_.script
          signature: /// Programs the outcome of the next-started batch before it runs. pub fn scrip…
          docstring: Programs the outcome of the next-started batch before it runs.
      - id: impl_Kernel_for_FakeKernel_H_
        signature: "impl<H: Clone> Kernel for FakeKernel<H> {"
        children:
        - id: impl_Kernel_for_FakeKernel_H_.oGaivh|v5uMLN
          signature: type Hit = H;
        - id: impl_Kernel_for_FakeKernel_H_.batch_start
          signature: "fn batch_start(&mut self) -> u64 {"
        - id: impl_Kernel_for_FakeKernel_H_.batch_end
          signature: "fn batch_end(&mut self, _batch_id: u64) {"
        - id: impl_Kernel_for_FakeKernel_H_.results
          signature: "fn results(&self, batch_id: u64) -> Vec<H> {"
        - id: impl_Kernel_for_FakeKernel_H_.written_ranges
          signature: "fn written_ranges(&self, batch_id: u64) -> Vec<(usize, usize)> {"
        - id: impl_Kernel_for_FakeKernel_H_.progress
          signature: "fn progress(&self, batch_id: u64) -> usize {"
      - id: Measure
        signature: /// Measurement trait calibration logic depends on; `FakeMeasure` stands in for…
        docstring: Measurement trait calibration logic depends on; `FakeMeasure` stands in for real
        children:
        - id: Measure.sample
          signature: fn sample(&mut self) -> f64;
      - id: NoiseModel
        signature: /// Noise models covering the calibration-stability test matrix (F2-F5). pub en…
        docstring: Noise models covering the calibration-stability test matrix (F2-F5).
      - id: FakeMeasure
        signature: "/// Seeded-RNG measurement fake: `base` plus a `NoiseModel`-shaped perturbation…"
        docstring: "Seeded-RNG measurement fake: `base` plus a `NoiseModel`-shaped perturbation, so"
      - id: impl_FakeMeasure
        signature: "impl FakeMeasure {"
        children:
        - id: impl_FakeMeasure.new
          signature: "pub fn new(base: f64, noise: NoiseModel, seed: u64) -> Self {"
      - id: impl_Measure_for_FakeMeasure
        signature: "impl Measure for FakeMeasure {"
        children:
        - id: impl_Measure_for_FakeMeasure.sample
          signature: "fn sample(&mut self) -> f64 {"
      - id: sync
        signature: /// `std::sync` vs. `loom::sync` shim for CAS-protocol modules (B1-style queues…
        docstring: "`std::sync` vs. `loom::sync` shim for CAS-protocol modules (B1-style queues): s…"
      - id: sync_2
        signature: "#[cfg(not(loom))] pub mod sync {"
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/memory/region.rs
      nodes:
      - id: ModuleDoc
        signature: //! Pointer-free memory handle; resolution to a device address is backend-inter…
      - id: ArenaId
        signature: "#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]"
      - id: Region
        signature: "#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)] pub struct Region {"
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/mod.rs
      nodes:
      - id: ModuleDoc
        signature: "//! Resident engine: embedding/similarity substrate, kept warm."
      - id: backend
        signature: pub mod backend;
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
```
/answer 78c8-36 allow
Control Request:
```yaml
id: 78c8-37
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/mock.rs
    ids:
    - Kernel
    - FakeKernel
    - impl_FakeKernel_H_
    - impl_Kernel_for_FakeKernel_H_
    - sync
    - sync_2
    - ModuleDoc
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/error.rs
    ids:
    - IndexError
    - ManagerError
    - impl_IsRetryable_for_ManagerError
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/memory/region.rs
    ids:
    - Region
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel/mod.rs
    ids:
    - ModuleDoc
  reason: "Read existing mock Kernel trait, error enums, Region and kernel module header to integrate B5 consistently"
```
/answer 78c8-37 allow
ReasoningToken: 76
Control Request:
```yaml
id: 78c8-38
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/mock.rs
      nodes:
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
              #[error("stale region in arena {arena} (gen {gen})")]
              StaleRegion { arena: u32, gen: u32 },
          }
      - id: impl_IsRetryable_for_ManagerError
        type: ItemImpl
        signature: "impl IsRetryable for ManagerError {"
        children:
        - id: impl_IsRetryable_for_ManagerError.is_retryable
          type: ImplItemFn
          code: |2
                fn is_retryable(&self) -> bool {
                    matches!(
                        self, ManagerError::StaleMapping { .. } | ManagerError::StaleRegion { .. }
                    )
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
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel/mod.rs
      nodes:
      - id: ModuleDoc
        type: ModuleAttrs
        code: |
          //! SegmentDesc/Layout/KernelHit, Kernel trait.
```
/answer 78c8-38 allow
Control Request:
```yaml
id: 78c8-39
toolName: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag
    pattern: "xxhash|xxh3|^\\[features\\]|^mock|^cuda|tracing|thiserror|^default"
    include:
    - Cargo.toml
    exclude:
    - .xyrag/**
    - .bin/**
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src
    pattern: "mock::Kernel|FakeKernel|kernel::"
    include:
    - '*.rs'
    exclude:
    - .xyrag/**
  reason: Check Cargo features/deps and usages of the existing mock Kernel trait
```
/answer 78c8-39 allow
Control Request:
```yaml
id: 78c8-3a
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag
      matches:
      - path: Cargo.toml
        matches:
        - lineno: 32
          match: thiserror = "2.0.20"
        - lineno: 33
          match: tracing = "0.1.44"
        - lineno: 34
          match: "tracing-subscriber = { version = \"0.3.23\", features = [\"env-filter\"] }"
        - lineno: 35
          match: tracing-appender = "0.2.3"
        - lineno: 42
          match: "xxhash-rust = { version = \"0.8\", features = [\"xxh3\"] }"
        - lineno: 49
          match: "cudarc = { version = \"=0.19.9\", optional = true }"
        - lineno: 51
          match: "[features]"
        - lineno: 52
          match: "default = []"
        - lineno: 54
          match: "cuda = [\"candle\", \"dep:candle-transformers\", \"dep:cudarc\", \"candle-core/cuda\", \"candle-transformers/cuda\"]"
        - lineno: 55
          match: "mock = []"
        - lineno: 56
          match: "json = [\"tracing-subscriber/json\"]"
        - lineno: 61
          match: mockall = "0.15.0"
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src
      matches:
      - path: resident/backend.rs
        matches:
        - lineno: 4
          match: "//! feature-independent; only this factory and the CUDA adapter modules (`kernel::cuda`,"
      - path: resident/mock.rs
        matches:
        - lineno: 43
          match: "pub struct FakeKernel<H> {"
        - lineno: 47
          match: "impl<H> FakeKernel<H> {"
        - lineno: 65
          match: "impl<H: Clone> Kernel for FakeKernel<H> {"
```
/answer 78c8-3a allow
ReasoningToken: 291
Control Request:
```yaml
id: 78c8-3b
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/error.rs
    ids:
    - ResidentError
    - impl_IsRetryable_for_ResidentError
    - impl_IsRetryable_for_IndexError
  reason: Decide how to add kernel errors
```
/answer 78c8-3b allow
Control Request:
```yaml
id: 78c8-3c
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
```
/answer 78c8-3c allow
Control Request:
```yaml
id: 78c8-3d
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/error.rs
    id: IndexError
    old_text: |2-
          TransferFailed { reason: String },
      }
    new_text: |2-
          TransferFailed { reason: String },
          #[error("kernel failed: {reason}")]
          KernelFailed { reason: String },
          #[error("unknown kernel batch {batch}")]
          UnknownBatch { batch: u64 },
      }
    exact: true
  reason: Add kernel error variants to the index/scope error taxonomy
```
/answer 78c8-3d allow
ReasoningToken: 2864
Control Request:
```yaml
id: 78c8-3e
toolName: ast_create
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel/mock.rs
    overwrite: false
    source: |
      //! Deterministic hand-written kernel fake for GPU-free tests.

      use std::collections::HashMap;
      use std::sync::atomic::{AtomicU64, Ordering};
      use std::sync::Mutex;

      use xxhash_rust::xxh3::xxh3_64;

      use super::{BatchDesc, BatchId, Kernel, KernelHit, KernelOutput, RegionSpan, WrittenRange};
      use crate::resident::error::IndexError;

      #[derive(Clone, Copy, Debug, PartialEq, Eq)]
      pub enum Call {
          Begin(BatchId),
          Run { batch: BatchId, spans: usize },
          End(BatchId),
      }

      #[derive(Clone, Copy, Debug)]
      enum Mode {
          Scoring,
          Embedding,
      }

      struct Batch {
          queries: u32,
          progress: AtomicU64,
          written: Vec<WrittenRange>,
      }

      struct State {
          batches: HashMap<u64, Batch>,
          calls: Vec<Call>,
          capacity: u64,
      }

      /// Scoring mode returns one hit per input span; embedding mode "writes" the input spans into a
      /// bounded capacity and reports `Partial` once it is exhausted.
      pub struct MockKernel {
          mode: Mode,
          next_id: AtomicU64,
          runs: AtomicU64,
          fail_after: Option<u64>,
          state: Mutex<State>,
      }

      impl MockKernel {
          pub fn scoring() -> Self {
              Self::new(Mode::Scoring, 0)
          }

          pub fn embedding(capacity: u64) -> Self {
              Self::new(Mode::Embedding, capacity)
          }

          fn new(mode: Mode, capacity: u64) -> Self {
              Self {
                  mode,
                  next_id: AtomicU64::new(0),
                  runs: AtomicU64::new(0),
                  fail_after: None,
                  state: Mutex::new(State { batches: HashMap::new(), calls: Vec::new(), capacity }),
              }
          }

          /// `run` succeeds `n` times, then fails on every further call.
          pub fn fail_after(mut self, n: u64) -> Self {
              self.fail_after = Some(n);
              self
          }

          /// Grants additional writable capacity, as the manager does after a `Partial`.
          pub fn grant(&self, bytes: u64) {
              self.state.lock().unwrap().capacity += bytes;
          }

          pub fn calls(&self) -> Vec<Call> {
              self.state.lock().unwrap().calls.clone()
          }
      }

      fn score(queries: u32, span: &RegionSpan) -> f32 {
          let mut bytes = [0u8; 20];
          bytes[..8].copy_from_slice(&span.offset.to_le_bytes());
          bytes[8..16].copy_from_slice(&span.len.to_le_bytes());
          bytes[16..].copy_from_slice(&queries.to_le_bytes());
          (xxh3_64(&bytes) >> 40) as f32 / (1u64 << 24) as f32
      }

      impl Kernel for MockKernel {
          fn begin(&self, batch: &BatchDesc) -> Result<BatchId, IndexError> {
              let id = BatchId(self.next_id.fetch_add(1, Ordering::SeqCst));
              let mut state = self.state.lock().unwrap();
              state.batches.insert(
                  id.0,
                  Batch { queries: batch.queries, progress: AtomicU64::new(0), written: Vec::new() },
              );
              state.calls.push(Call::Begin(id));
              tracing::debug!(batch = id.0, "kernel batch begin");
              Ok(id)
          }

          fn run(&self, batch: BatchId, input: &[RegionSpan]) -> Result<KernelOutput, IndexError> {
              let prior = self.runs.fetch_add(1, Ordering::SeqCst);
              let mut state = self.state.lock().unwrap();
              state.calls.push(Call::Run { batch, spans: input.len() });
              if matches!(self.fail_after, Some(n) if prior >= n) {
                  return Err(IndexError::KernelFailed { reason: "injected failure".into() });
              }
              let State { batches, capacity, .. } = &mut *state;
              let b = batches.get_mut(&batch.0).ok_or(IndexError::UnknownBatch { batch: batch.0 })?;

              match self.mode {
                  Mode::Scoring => {
                      let hits: Vec<KernelHit> = input
                          .iter()
                          .map(|s| KernelHit { offset: s.offset, score: score(b.queries, s) })
                          .collect();
                      b.progress.fetch_add(input.len() as u64, Ordering::SeqCst);
                      tracing::debug!(batch = batch.0, hits = hits.len(), "kernel run");
                      Ok(KernelOutput::Hits(hits))
                  }
                  Mode::Embedding => {
                      for span in input {
                          let take = span.len.min(*capacity);
                          if take > 0 {
                              b.written.push(WrittenRange { offset: span.offset, len: take });
                              *capacity -= take;
                              b.progress.fetch_add(take, Ordering::SeqCst);
                          }
                          if take < span.len {
                              tracing::debug!(batch = batch.0, "kernel run partial");
                              return Ok(KernelOutput::Partial);
                          }
                      }
                      Ok(KernelOutput::Hits(Vec::new()))
                  }
              }
          }

          fn progress(&self, batch: BatchId) -> u64 {
              let state = self.state.lock().unwrap();
              state.batches.get(&batch.0).map_or(0, |b| b.progress.load(Ordering::SeqCst))
          }

          fn end(&self, batch: BatchId) -> Result<Vec<WrittenRange>, IndexError> {
              let mut state = self.state.lock().unwrap();
              state.calls.push(Call::End(batch));
              let b = state.batches.remove(&batch.0).ok_or(IndexError::UnknownBatch { batch: batch.0 })?;
              tracing::debug!(batch = batch.0, ranges = b.written.len(), "kernel batch end");
              Ok(b.written)
          }
      }
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel/mod.rs
    overwrite: true
    source: |
      //! Kernel abstraction: segment descriptors, hits and the `Kernel` trait.
      //!
      //! The kernel knows only segments, offsets and spans. Mapping an address back to an entry is the
      //! manager's job (`entry_of`, `entry_of_fixed`); the public API carries no pointers.

      #[cfg(feature = "cuda")]
      mod cuda;
      #[cfg(feature = "cuda")]
      pub use cuda::*;

      #[cfg(any(test, feature = "mock"))]
      pub mod mock;

      use std::time::Duration;

      use crate::resident::error::IndexError;

      /// Address-agnostic arena span; pointer resolution happens only inside a backend.
      #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
      pub struct RegionSpan {
          pub offset: u64,
          pub len: u64,
      }

      /// Offset table (`n + 1` prefix sums) living in an arena span.
      #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
      pub struct OffsetRef(pub RegionSpan);

      #[derive(Clone, Copy, Debug, PartialEq, Eq)]
      pub enum SegmentLayout {
          Fixed { stride: u32 },
          Variable { offsets: OffsetRef },
      }

      #[derive(Clone, Copy, Debug, PartialEq, Eq)]
      pub struct SegmentDesc {
          pub base: u64,
          pub count: u32,
          pub layout: SegmentLayout,
      }

      #[derive(Clone, Copy, Debug, PartialEq)]
      pub struct KernelHit {
          pub offset: u64,
          pub score: f32,
      }

      #[derive(Clone, Copy, Debug, PartialEq, Eq)]
      pub struct WrittenRange {
          pub offset: u64,
          pub len: u64,
      }

      #[derive(Clone, Debug, Default)]
      pub struct BatchDesc {
          pub segments: Vec<SegmentDesc>,
          pub queries: u32,
          pub params: Option<RegionSpan>,
          pub time_budget: Option<Duration>,
      }

      #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
      pub struct BatchId(u64);

      impl BatchId {
          pub fn get(self) -> u64 {
              self.0
          }
      }

      #[derive(Clone, Debug, PartialEq)]
      pub enum KernelOutput {
          Hits(Vec<KernelHit>),
          /// The granted range is exhausted; the manager grants the next one and resumes.
          Partial,
      }

      pub trait Kernel: Send + Sync {
          fn begin(&self, batch: &BatchDesc) -> Result<BatchId, IndexError>;
          fn run(&self, batch: BatchId, input: &[RegionSpan]) -> Result<KernelOutput, IndexError>;
          /// Monotonically non-decreasing.
          fn progress(&self, batch: BatchId) -> u64;
          fn end(&self, batch: BatchId) -> Result<Vec<WrittenRange>, IndexError>;
      }

      /// Entry index for `addr` in a `Fixed` segment.
      pub fn entry_of_fixed(base: u64, stride: u32, count: u32, addr: u64) -> Option<usize> {
          if stride == 0 || addr < base {
              return None;
          }
          let idx = (addr - base) / u64::from(stride);
          (idx < u64::from(count)).then_some(idx as usize)
      }

      /// Entry index for `addr` given `n + 1` ascending offsets; entry `i` covers
      /// `[offsets[i], offsets[i + 1])`.
      pub fn entry_of(offsets: &[u64], addr: u64) -> Option<usize> {
          let i = offsets.partition_point(|&o| o <= addr);
          (i > 0 && i < offsets.len()).then(|| i - 1)
      }

      #[cfg(test)]
      mod tests {
          use super::mock::{Call, MockKernel};
          use super::*;

          fn spans(n: u64) -> Vec<RegionSpan> {
              (0..n).map(|i| RegionSpan { offset: i * 64, len: 64 }).collect()
          }

          fn desc() -> BatchDesc {
              BatchDesc { queries: 2, ..Default::default() }
          }

          #[test]
          fn entry_of_fixed_bounds() {
              assert_eq!(entry_of_fixed(100, 10, 4, 99), None);
              assert_eq!(entry_of_fixed(100, 10, 4, 100), Some(0));
              assert_eq!(entry_of_fixed(100, 10, 4, 119), Some(1));
              assert_eq!(entry_of_fixed(100, 10, 4, 139), Some(3));
              assert_eq!(entry_of_fixed(100, 10, 4, 140), None);
              assert_eq!(entry_of_fixed(100, 0, 4, 100), None);
          }

          #[test]
          fn entry_of_variable_bounds() {
              let offsets = [10, 15, 15, 40];
              assert_eq!(entry_of(&offsets, 9), None);
              assert_eq!(entry_of(&offsets, 10), Some(0));
              assert_eq!(entry_of(&offsets, 14), Some(0));
              assert_eq!(entry_of(&offsets, 15), Some(2));
              assert_eq!(entry_of(&offsets, 39), Some(2));
              assert_eq!(entry_of(&offsets, 40), None);
              assert_eq!(entry_of(&[], 0), None);
          }

          #[test]
          fn scoring_is_deterministic() {
              let run = || {
                  let k = MockKernel::scoring();
                  let b = k.begin(&desc()).unwrap();
                  let out = k.run(b, &spans(5)).unwrap();
                  (out, k.end(b).unwrap())
              };
              let (a, b) = (run(), run());
              assert_eq!(a, b);
              assert!(matches!(&a.0, KernelOutput::Hits(h) if h.len() == 5));
              assert!(a.1.is_empty());
          }

          #[test]
          fn partial_then_follow_up_range_covers_input() {
              let k = MockKernel::embedding(120);
              let b = k.begin(&desc()).unwrap();
              let input = [RegionSpan { offset: 0, len: 100 }, RegionSpan { offset: 100, len: 50 }];
              assert_eq!(k.run(b, &input).unwrap(), KernelOutput::Partial);

              k.grant(100);
              let rest = [RegionSpan { offset: 120, len: 30 }];
              assert!(matches!(k.run(b, &rest).unwrap(), KernelOutput::Hits(_)));

              let written = k.end(b).unwrap();
              assert_eq!(written.iter().map(|w| w.len).sum::<u64>(), 150);
          }

          #[test]
          fn progress_is_monotonic() {
              let k = MockKernel::scoring();
              let b = k.begin(&desc()).unwrap();
              let mut last = k.progress(b);
              for _ in 0..4 {
                  k.run(b, &spans(3)).unwrap();
                  let now = k.progress(b);
                  assert!(now > last);
                  last = now;
              }
          }

          #[test]
          fn injected_failure_leaves_batch_endable() {
              let k = MockKernel::scoring().fail_after(1);
              let b = k.begin(&desc()).unwrap();
              assert!(k.run(b, &spans(1)).is_ok());
              assert!(matches!(k.run(b, &spans(1)), Err(IndexError::KernelFailed { .. })));
              assert!(k.end(b).is_ok());
              assert_eq!(k.calls().last(), Some(&Call::End(b)));
              assert!(matches!(k.end(b), Err(IndexError::UnknownBatch { .. })));
          }

          #[test]
          fn usable_as_trait_object() {
              let k: std::sync::Arc<dyn Kernel> = std::sync::Arc::new(MockKernel::scoring());
              let b = k.begin(&desc()).unwrap();
              assert!(k.run(b, &spans(1)).is_ok());
          }
      }
  reason: "Implement B5: kernel types, trait, entry_of helpers, mock and tests; replace stub kernel/mod.rs"
```
/answer 78c8-3e allow
Control Request:
```yaml
id: 78c8-3f
toolName: ast_validate
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel/mod.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel/mock.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/error.rs
  reason: Syntax check of the new and edited files
```
/answer 78c8-3f allow
Result Stats: id=de4cdb87-1974-4aa6-b605-b3aca180b2ee, total: 53009, in: 16, out: 12450, reason: 0, read: 167916, write: 40543, start: 1792312360400, end: 1792312368052, time: 7584, total: 79966, io: 0, control: 68, sent: 0, recv: 203
B5 ist im Rust-Projekt umgesetzt. Die Dateien bestehen die Syntaxprüfung. Ich konnte aber weder `cargo build` noch `cargo test` ausführen, die Testmatrix aus §8 ist also nicht gelaufen.

- **Typen und Trait** (`rag/src/resident/kernel/mod.rs`): Die Typen sind `RegionSpan`, `OffsetRef`, `SegmentLayout`, `SegmentDesc`, `KernelHit`, `WrittenRange`, `BatchDesc`, `BatchId` und `KernelOutput`. Dazu kommt der `Kernel`-Trait mit `begin`, `run`, `progress` und `end`. Die öffentliche API enthält keine Pointer. Der Trait liefert `IndexError`, weil ich keinen eigenen Fehlertyp angelegt habe.
- **Rückabbildung**: `entry_of_fixed` rechnet per Division, `entry_of` nutzt `partition_point`. Beide liefern bei Werten außerhalb des Bereichs `None`.
- **Mock** (`kernel/mock.rs`): `MockKernel` ist im Test-Build und mit `feature = "mock"` verfügbar. Es gibt einen Scoring-Modus und einen Embedding-Modus mit begrenzter Kapazität, der bei Erschöpfung `Partial` meldet. Die Scores kommen aus xxh3 und sind deterministisch. Der Fortschritt läuft über `AtomicU64`, der Aufruf-Log über `calls()`, und `fail_after(n)` simuliert Fehler. Mit `grant()` vergibt der Test den Folgebereich.
- **Fehler** (`error.rs`): `IndexError` hat die neuen Varianten `KernelFailed` und `UnknownBatch`. Beide gelten nicht als wiederholbar.
- **Tests**: Die Tests in `kernel/mod.rs` decken Ränder von `entry_of`, Determinismus, den Partial-Pfad mit Folgebereich und monotonen Fortschritt ab. Dazu kommen Fehlerinjektion mit anschließendem `end` und die Nutzung als `Arc<dyn Kernel>`.

- **Bestehender `Kernel`-Trait:** `resident/mock.rs` enthält schon einen älteren `Kernel`-Trait mit `FakeKernel`. Ich habe ihn nicht angefasst. Er hat einen anderen Pfad und eine andere API als der neue Trait, und es sollte geklärt werden, ob er später entfällt.
- **Embedding-Modus:** Ist alles geschrieben, liefert `run` ein leeres `Hits`, weil `KernelOutput` nur `Hits` und `Partial` kennt.

Bitte lass `cargo test --lib kernel` laufen, mit und ohne Feature `cuda`.
