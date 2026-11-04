# 019_resident: missing and open points

Implemented so far (unverified, see 1): `l010/resident.rs` with tests in `l010/test/resident.rs`. It covers the core engine, the builder, tickets, lazy start, shutdown and `handle_request` (one JSON line in, one JSON line out). Queues come from `l010/queue.rs`.

Target directory: `/home/user/xyan/xy.ai.workbench/rag/src/resident/l130/l120/l110/l100/l090/l080/l070/l060/l050/l040/l030/l020/l010/`
Crate root: `/home/user/xyan/xy.ai.workbench/rag/src/lib.rs`, manifest: `/home/user/xyan/xy.ai.workbench/rag/Cargo.toml`.
Rules: English comments and docs only, minimal comments, tests in `<dir>/test/<name>.rs` with the `#[path]` block at the end of the source file, no Bash for retrieval.

## 1. Verification (first task)
- `cargo build` and `cargo test resident` have never been run. Only syntax was checked. Fix all compile errors and failing tests.
- `l010/mod.rs` now also declares `pub mod queue;`. Confirm that `queue.rs` and `test/queue.rs` compile and pass, and that no other undeclared sibling file is needed.
- Confirm that `Resident`, `Ticket<T>` and the trait objects are `Send + Sync` as required, and that `Queue` is `Sync` with these type parameters.

## 2. CLI binary `resident` (not implemented)
- Add `src/bin/resident.rs` and a new `[[bin]] name = "resident"` entry in `Cargo.toml`. Do not change the existing `xyrag` entry.
- Subcommands (clap is available):
  - `resident serve --listen <addr> [--daemon]`
  - `resident embed <text> [--target cpu|gpu|any]`
  - `resident search <index> <vector-json> -k N [--target ..]`
- Output: JSON on stdout, errors on stderr, nonzero exit code on error.
- Reuse `handle_request` for the line protocol. Keep responses `{"ok":..}` or `{"err":"<variant>"}`.

## 3. Server (not implemented)
- Accept newline-delimited JSON on a TCP address or a Unix socket path, one response line per request.
- Handle each connection independently. Never block the accept loop on computation. Overload maps to `{"err":"Overloaded"}`.
- Define how `--listen` distinguishes a TCP address from a socket path.
- `--daemon` detaches from the terminal (use `libc`, already a Linux dependency).
- On SIGTERM or SIGINT: call `shutdown()` and exit with code 0.

## 4. `register_defaults` (not implemented)
- `pub fn register_defaults(b: ResidentBuilder) -> ResidentBuilder`, supplied by the embedding application or a crate feature.
- Initially a no-op, so the binary returns `BackendUnavailable` without registered backends.
- Open decision: bind a real backend for `/home/user/xyan/xy.ai.workbench/rag/model/multilingual-e5-small` (CPU via candle, GPU via CUDA, target sm_75). The spec leaves backends to the application, so decide before starting.

## 5. Known deviations and weak points in the current code
- The l010 `Queue` uses a `Mutex` + `Condvar` inside its `Notifier`, used only when someone waits on a ticket. This conflicts with the strict "no locks on data paths" rule. Decide whether to accept it or to add a lock-free wait path.
- The queue is unbounded. Capacity is enforced by a separate counter (`Lane`). Check that the counter stays correct for every pull and sweep path.
- `Target::Any` starts only one sub-engine (the running one, otherwise CPU, otherwise GPU). The other sub-engine never contributes unless it gets a targeted task. Decide whether that is acceptable.
- `Any` tasks go to the shared queue. If the chosen engine fails during warm-up and the other one is unstarted, the failure path starts it. This path is untested.
- `warm_up` still runs when shutdown has already begun.
- `shutdown` called concurrently from a second thread may return before the first call has finished joining.
- A `Ticket` that already delivered its result returns `Err(Cancelled)` on `wait`, and `None` on `try_take`. The spec does not define this case.
- A spawn failure for all workers marks the sub-engine failed. This path is untested.

## 6. Missing tests
- Concurrency stress and race tests (submit versus shutdown, submit versus warm-up failure, lazy start races). `loom` and `shuttle` are in dev-dependencies and are unused.
- `Any` routing with both sub-engines present (CPU and GPU mocks), including failure of one of them.
- Dropped tickets (result discarded, task neither cancelled nor blocking).
- `set_index` while a search is running (old snapshot stays valid).
- Stats counters (`completed`, `failed`, `rejected`) in error paths.
- Server and CLI tests once those exist.

## 7. Documentation
- Add a short English module doc describing the public API and the lifecycle (lazy start, pull, shutdown), if the project expects one per module.
