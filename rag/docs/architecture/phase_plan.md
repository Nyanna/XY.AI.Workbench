# CPU/GPU Differences and Build-out Phases

## Capability matrix

| Aspect | CPU subengine | GPU subengine (`cuda`) |
|---|---|---|
| Memory | host RAM, large | VRAM bounded (reference: 6 GB), two index generations budgeted |
| Index access | mmap readable | arena slots, synced from host |
| Staging / streams | none; lanes are worker threads | pinned staging, one stream per lane, events |
| Scorer | own SIMD loops (`cpu_simd`) | Candle tile chain, then fused NVRTC kernel |
| Encoder | Candle CPU | Candle CUDA |
| dtype | `F32` | `F16` |
| Tokenizer / normalizer | CPU, shared | CPU, shared; the bottleneck, so batching is required |
| Shared | `Op`, `Region`, `ExecContext`, lease logic, task pipelines | same |

Numerics: f32 vs f16 cosine differs by 1e-3 to 1e-4; rankings differ only at near-ties. Indexes fix their dtype; mixed CPU/GPU comparison always uses a tolerance (`kernel::golden::Tolerance`).

## Phases

Each phase keeps public signatures unchanged. Acceptance always includes a golden check against the previous phase (`kernel::golden::check`) and the API hygiene test (`tests/api_hygiene.rs`).

| Phase | Content | Packages | Feature | Gate | Acceptance |
|---|---|---|---|---|---|
| 1 | `Region`/`Op`/`ExecContext` as thin wrappers over Candle; MaxSim as tile chain; lazy init, leases | D1-D7 | none (CPU), `cuda` optional | none | CPU f32 reference matches tile chain within `Tolerance::F32`; no-GPU build green |
| 2 | own streams, fused MaxSim kernel (NVRTC), slice scheduling | D2, D6 | `cuda` | G-MAXSIM | fused kernel ranks like phase 1 within `Tolerance::F16` and is not slower on the measured workload |
| 3 | own arena for weights and workspace, `encode_into` without copy | D3, D4 | `cuda` | G-WRAP | encoder output equals phase 2 within tolerance; if G-WRAP fails: arena only for own kernels, D2D copy for Candle tensors |
| 4 | graph decorator, persistent scorer with ring | hooks only | `cuda` | G-GRAPH | `LaunchMode::Capture` selectable via `launcher_for`; replay equals `Direct` |
| 5 | optional own encoder forward (cuBLAS plus few kernels) behind the same `Op` | D4 | `cuda` | none | embeddings equal Candle within `Tolerance::F16`; signature unchanged |

Phase transitions are logged as `info` events; golden deviations are logged as `warn` diagnostics and returned, never panicked on.

## Extension points

- Launch indirection: `kernel::launcher` (`Launcher`, `LaunchKey`, `LaunchMode`). `DirectLauncher` is active; `Capture` is rejected with `KernelError::Unavailable` until phase 4. The key holds op, shape bucket, regions (including generation), dtype and lane, which is the graph cache key.
- Persistent scorer ring: needs a dedicated lane and fixed ring slots in the manager; state (head, tail, stop flag, counters) lives in its own region; the stop flag is checked only at slice boundaries. Not implemented.
- Parameter objects stay extensible, `requires()` affinity stays open, and ops can be replaced behind the same trait.

## Trait hygiene

`Op`, `Region` and `ExecContext` never expose `candle_core::Tensor` or raw pointers. Only `model/candle_adapter.rs`, `kernel/cuda.rs` and `kernel/maxsim.rs` (feature-gated Candle chain) may name them; `tests/api_hygiene.rs` enforces this for all other modules. Event ordering between Candle work and own kernels is explicit: own work waits on an `EventHandle` recorded on the Candle stream.
