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

- Span granularity: one span per task (field `task_id`); child spans per processing
  cycle/batch and per manager cycle. A resource (device/index) is a span **field**
  (`resident::telemetry::FIELD_RESOURCE`), never its own span.
- Async rule: use `#[tracing::instrument(skip_all, fields(task_id = %id))]` or
  `.instrument(span)` on futures. Never hold `span.enter()` across an `.await`.
- Thread/`spawn_blocking` rule: spans do not propagate automatically across a thread
  boundary. Clone the span (`telemetry::for_blocking`) and use `span.in_scope(...)` or
  `let _g = span.enter();` inside the closure. GPU calls run on a dedicated thread or via
  `spawn_blocking`, never as a plain async task.
- Subscriber: `tracing-subscriber` with `EnvFilter` (`telemetry::init_tracing`, default
  `info`, overridable via `RUST_LOG`). High-frequency kernel-level events use `trace!`.
  Each group defines, for its own events, what is logged at which level (e.g. manager scan
  results, load/eviction decisions, discarded/outdated results, arena utilization).

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
