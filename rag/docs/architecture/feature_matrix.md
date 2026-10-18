# Feature Matrix

| Feature | Pulls | Purpose |
|---------|-------|---------|
| `default` | `[]` | CPU/mock, no CUDA toolkit required |
| `cuda` | `candle`, `candle-transformers`, `cudarc`, candle CUDA backends | real GPU pathways (`resident::kernel::cuda`, `resident::subengine::device`) |
| `mock` | — | deterministic fakes outside `#[test]` |

`cuda` is never part of `default`.

## CI

Defined in `.github/workflows/rag.yml` (repository root).

- `build-cpu`: `cargo build --no-default-features` and `cargo build --features mock` without CUDA toolkit.
- `test-cpu`: `cargo test` and `cargo test --features mock`.
- `cfg-scatter`: `scripts/check_cfg_scatter.sh`.
- `loom` (on demand): `RUSTFLAGS="--cfg loom" cargo test --lib --release resident::queue`.
- `cuda` (GPU runner): `cargo build --features cuda` and `cargo tree -d --features cuda` with
  `CUDA_COMPUTE_CAP=75`, CUDA 12.x.

## Feature gating rule

`cfg(feature = "cuda")` is evaluated in one place: `resident::backend::create_backend` (via `cfg!`).
Adapter modules (`kernel::cuda`, `subengine::device`) always compile; toolkit-specific code inside them is
gated within those files. `candle` gating is limited to `memory::dtype`. `scripts/check_cfg_scatter.sh`
enforces this.
