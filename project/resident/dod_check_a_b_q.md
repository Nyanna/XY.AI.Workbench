Plan A, B und Q sind umgesetzt. Setze die fehlenden Punkte um.

[include](/home/user/xyan/xy.ai.workbench/project/resident/index.md)

- minimiere die Kommentare und keine direkte Referenzierung von Planelementen in Kommentaren
- Projektdokumente immer in englisch ausgeben

# DoD Reconciliation: Groups A, B, Q

State: `rag` crate, `cargo test` green (86 unit + 4 e2e), also with `--features mock`; `cargo build --no-default-features` green.

Legend: **Met** · **Partial** · **Open** · **Unverified** (cannot be checked in this environment)

## Group A

| Item | Status | Evidence / gap |
|---|---|---|
| A1: charter with full boundary table, hook-to-extension mapping, ring-buffer vs. pull-queue decision | Met | `docs/architecture/scope_charta.md` (127 lines) |
| A1: charter referenced from C1 and E2 | Partial | Referenced in `plan_c_01`; no reference found in any E-plan |
| A2: build without CUDA toolkit | Met | `cargo build --no-default-features` succeeds, no `nvcc` on host |
| A2: build with `cuda` | Unverified | `cargo check --features cuda` fails here: `nvcc` not installed; needs GPU runner |
| A2: every B–G package has one target module | Partial | Stub modules exist for facade, processor, planner, subengine, manager; E/F/G/H modules not yet created |
| A2: feature matrix documented, `cuda` not in default | Met | `docs/architecture/feature_matrix.md` |
| A2: "workspace" / `-p resident` build commands | Open | Project is a single crate (`xy-ai-rag`), not a workspace; plan commands do not apply literally |
| A3: one `cudarc` version, `cargo tree -d` clean | Partial | `cudarc` is optional and not resolved without `cuda`. Duplicates remain (`rand` 0.8/0.9, `hashbrown`, `getrandom`, `syn`, `base64`, ...); none reviewed or documented as accepted |
| A3: parquet f16 round-trip and tokenizer load pass | Open | No such tests found. `tokenizers` needed the `fancy-regex` feature to compile at all (fixed) |
| A3: toolkit / compute-cap decision anchored in build | Open | `.cargo/config.toml` sets no `CUDA_COMPUTE_CAP`; only documented in the feature matrix CI job |
| A3: verification document, all `[?]` resolved or marked as residual risk | Partial | `dependency_verification.md` exists; contains the legend but no residual-risk entries were found |
| A4: conventions cover Q1–Q4 | Met | `docs/architecture/conventions.md` |
| A4: `error.rs` + `telemetry.rs` compile GPU-free | Met | |
| A4: all Q1 variants anchored | Met | See Q1 |
| A4: mock convention works outside `#[test]` | Met | `--features mock` builds and tests pass (needed `rand` as optional dependency; fixed) |

## Group B

| Package | Status | Evidence / gap |
|---|---|---|
| Group: builds without `cuda`/`candle`; no `unsafe` | Met | No `unsafe` in `src/resident` |
| Group: open points marked `TODO(A3)` | Open | No `TODO` markers in `src/resident` |
| Group: `Layout`/`SegmentLayout` naming, stamp compared only with `!=`, `DType` non-exhaustive | Met | |
| B1: `Queue`, `Node`, `TaskState`/`ClaimState`, push/claim/complete/cancel, iterative drop, notify relay | Met | 10 tests incl. 100k-chain drop |
| B1: unlink | Open | No unlink operation implemented; decision (logical vs. mutex) undocumented in code |
| B1: loom test | Open | Loom shim exists, but no `loom::model` test; with `--cfg loom` 10 resident tests fail because they run outside a model |
| B2: `Region`, `Layout`, `DType` (+`size_bytes`, `TryFrom` behind `candle`), `DeviceKind`, `compute_dtype`, `MemoryManager`, free-list | Met | |
| B2: generation invalidation and `ArenaExhausted` tested | Met | `realloc_bumps_gen_and_stale_is_rejected`, `exhaustion` |
| B3: `Op`, `OpKind`, `OpParams`, `Requirements`, `Value`, `Lease`, `OperationNode` | Met | |
| B3: lease RAII tested, affinity inactive | Met | `lease` and `requirements` tests |
| B4: `Index` trait and companion types | Met | |
| B4: stamp mechanics (monotonic, ABA, echo, fallback with racy+hash, high-water mark) tested | Open | `index::stamp` and `index::memory` contain no tests |
| B4: `watch<u64>` contract documented, reference in-memory index | Met | |
| B5: types, `Kernel` trait, `entry_of`, `MockKernel` with determinism/partial/progress/failure injection/call log | Met | 7 tests |
| B6: `AxisId`, `ValueId`, `CapabilitySet::satisfies`, `ProfileConfig`, `Profile`, Pareto helpers, `LazyLock` well-known axes without bf16 | Met | 16 tests |

## Group Q

| Package | Status | Evidence / gap |
|---|---|---|
| Q1: component enums + `ResidentError`, `#[non_exhaustive]`, structured fields, no `anyhow` | Met | `error.rs` |
| Q1: tests from the plan | Open | `error.rs` has no tests |
| Q1: unused variant | Partial | `ManagerError::StaleMapping` is never constructed (compiler warning) |
| Q2: stable field names, span helpers, no subscriber set by the library | Met | `telemetry.rs`, 3 tests |
| Q2: event catalogue and propagation rules covered | Partial | Only `debug`/`warn` events in queue, memory, index, kernel; no spans in the processor path (not yet built) |
| Q3: abstraction traits and fakes | Met | `mock.rs`, `kernel/mock.rs`. `mock.rs` also declares a second `Kernel` trait duplicating `kernel::Kernel` |
| Q3: `loom`, `shuttle`, `proptest` wired | Partial | All are dev-dependencies; `proptest` is used only in the trigram code, `shuttle` and `loom::model` not at all in `resident` |
| Q3: failpoints at sync points | Open | None present |
| Q3: test matrices (B1, E5/E6, F2–F5, C2/C3) created | Partial | B1 only partially (see above); others depend on unbuilt groups |
| Q3: all tests run without GPU | Met | |
| Q4: single `cfg` decision point in the factory | Partial | `create_backend` is the decision point, but `cfg(feature = "cuda")` also appears in `kernel/mod.rs` and `subengine/mod.rs` for module gating |
| Q4: CI matrix with/without `cuda` | Open | Documented in the feature matrix only; no CI configuration in the repository |
| Q4: `cfg`-scatter check | Open | No automated check |
| Q4: no-GPU default runs all unit tests | Met | |

## Open work, by priority

1. B4 stamp and in-memory index tests.
2. B1 loom model test (and decide on unlink); fix or gate resident tests under `--cfg loom`.
3. Q1 error tests; construct or remove `StaleMapping`; remove duplicate `Kernel` trait in `mock.rs`.
4. Q3 failpoints and `shuttle` or `proptest` coverage for queue and memory.
5. A3 tests for parquet f16 round-trip and tokenizer load; review `cargo tree -d` duplicates and record residual risks; set `CUDA_COMPUTE_CAP` in the build.
6. Q4 CI configuration and `cfg`-scatter check; resolve the duplicated gating in `kernel` and `subengine`.
7. Reference the charter from the E-plans.
8. Run the `cuda` build on a GPU runner.
