# Cross-Cutting Conventions (A4): Errors, Logging, Testing, Feature Flags

> Initial fixation of Q1–Q4. Carried forward by every subsequent group's modules.

## Q1 — Error Taxonomy (`thiserror`)

- One error enum per layer (e.g. index, memory, calibration, planner, capability); upper
  layers wrap lower ones via `#[from]`/`#[source]`, never by re-stringifying.
- Public enums are `#[non_exhaustive]`. All variants are `Send + Sync + 'static`. Context
  (e.g. `task_id`, `scope`) is carried as typed fields, not formatted into a message string.
- Shared taxonomy base (`src/resident/error.rs`): `UnsupportedScope`, `IndexUnavailable`,
  `ArenaExhausted`, `StaleMapping` (internal), `TransferFailed`, `Incompatible`,
  `CalibrationUnstable`, `NoActiveProfile`, `InvalidPrecision`, plus `PlannerError`/
  `CapabilityError` placeholders for variants owned by later groups.
- Retry classification: every error type implements `IsRetryable::is_retryable()` so C2's
  backoff can separate transient (e.g. transfer failure) from fatal (e.g. OOM/arena exhausted)
  conditions.
- Worker pools return `Result<T, E>` over channels, never `panic!`; a `JoinError` from a
  panicked task is caught and converted at the boundary. `anyhow` is reserved for
  binaries/`main`, never library code.

## Q2 — Logging (`tracing`)

- Stable field names (`resident::telemetry`): `task_id`, `resource_kind`, `model_id`,
  `index_id`, `segment_gen`, `slot`, `lane`, `manager_cycle`, `op`, `precision`, `profile`.
  Always use these exact names so filters and JSON pipelines stay stable.
- Span topology: one task span (`telemetry::task_span`) per task; one resource span
  (`telemetry::resource_span`) per leased resource, nested under the task span; one
  manager-cycle span (`telemetry::manager_cycle_span`) per communication slot/scan cycle.
- Async rule: use `#[tracing::instrument(skip_all, fields(task_id = %id))]` or
  `.instrument(span)` on futures. Never hold `span.enter()` across an `.await`.
- Thread/`spawn_blocking` rule: spans do not propagate automatically across a thread
  boundary. Capture `Span::current()`, pass it in, and use `telemetry::for_blocking` plus
  `span.in_scope(...)` inside the closure. GPU calls run on a dedicated thread or via
  `spawn_blocking`, never as a plain async task.
- GPU completion callbacks (`cuLaunchHostFunc`) must not call tracing/CUDA APIs directly;
  hand the completion data to a channel/atomic and log it from the worker that drains it.
- Subscriber: `tracing-subscriber` with `EnvFilter` (`telemetry::init_tracing` for the plain
  default, `telemetry::init_tracing_non_blocking` for a non-blocking appender with optional
  JSON output behind the `json` feature). High-frequency kernel-level events use `trace!`.
  Each group defines, for its own events, what is logged at which level (e.g. manager scan
  results, load/eviction decisions, discarded/outdated results, arena utilization).
- The engine library never installs a global subscriber or appender; only the binary/daemon
  does (see `src/bin/xyrag.rs`).

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

## Basis Scaffolding

- `src/resident/error.rs` — taxonomy base + `IsRetryable` trait, re-exported from
  `src/resident/mod.rs`.
- `src/resident/telemetry.rs` — span field-name constants, `init_tracing()`, `for_blocking()`.
- `src/resident/mock.rs` — mock convention + minimal example, gated by `mock` feature.
- `mock` feature wired in `Cargo.toml` (`[features] mock = []`).
