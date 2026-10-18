## Implementation status

Code lives in `rag/src/resident/kernel/`: `registry.rs`, `cosine.rs`, `maxsim.rs`, `cpu_simd.rs`, `slice.rs`, `topk.rs`, `slots.rs`, `error.rs`, `cuda.rs` (NVRTC/CUBIN, `cuda` feature). `Scorer` returns slice-local top-k per query (`Vec<Vec<KernelHit>>`) so query batching keeps per-query results.

## G-MAXSIM result (GTX 1660 SUPER, sm75, f16)

Workload: 4096 documents x 128 tokens, 32 query tokens, dim 128, whole slice per pass.

| Implementation | Time per pass |
|---|---|
| Candle chain (`broadcast_matmul -> mask -> max -> sum`, cuBLAS) | 21.6 ms |
| Fused NVRTC kernel (scalar FMA, one block per document) | 67.1 ms |

Decision: keep the Candle chain as the MaxSim implementation. The fused kernel is correct (matches CPU reference) but 3x slower because it does not use tensor cores. Revisit only with a tensor-core (WMMA/mma) fused kernel that beats the chain; it plugs in behind `MaxSimOp::with_slots`.

## Open points

- Kernel launches take plain kernel arguments; mirroring dynamic values through the lane `ParamBlock` and resolving arena `Region`s to device pointers are part of the segment integration (D7).
- The Candle chain scores from host `SlotSource` data; device-resident documents arrive with D7.
