# Feature Matrix

| Feature | Pulls | Purpose |
|---------|-------|---------|
| `default` | `[]` | CPU/mock, no CUDA toolkit required |
| `cuda` | `dep:candle-core`, `candle-core/cuda` | real GPU pathways (`resident::kernel::cuda`, `resident::subengine::device`) |
| `mock` | — | deterministic fakes outside `#[test]` |

`cuda` is never part of `default`.

## CI

- Job A: `cargo build --no-default-features` without CUDA toolkit.
- Job B: `cargo test --no-default-features`.
- Job C (GPU runner): `cargo build --features cuda` with `CUDA_COMPUTE_CAP=75`, CUDA 12.x.
