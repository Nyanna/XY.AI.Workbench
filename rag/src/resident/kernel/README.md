# kernel

Scoring compute. A kernel receives segment descriptors and spans and returns (address, score) pairs. It knows nothing about entries, indexes or models.

## Concept

- **Segments, not entries.** A segment has a base, a count and a layout: fixed stride (entry boundaries by arithmetic) or variable (offset table of prefix sums). A candidate set is simply a variable layout. The kernel never traverses and never communicates during a run. Mapping an address back to an entry is the manager's job; mod.rs only provides the arithmetic helpers for both layouts.
- **Scorers are swappable behind one operation.** Dense path: cosine as the dot product of L2-normalised vectors (an exact variant exists). Late interaction: MaxSim, implementable as CPU SIMD loop, as a tile chain on Candle ops, or as a fused kernel. Padding is masked on the document side; query mask tokens count.
- **Top-k is bounded and deterministic.** Slice-local reduction with tie-breaking, merged on the CPU.
- **Work slices.** Launches are bounded and sized from observed throughput; yielding is possible only at slice boundaries because a running GPU kernel cannot be preempted.
- **Launch indirection.** A launcher sits between scorer and lane: direct submit now; graph capture is reserved and keyed so scorers need no change when it arrives.
- **Registry.** Lazily populated kernel table keyed by name, dtype, SM architecture and variant; the CUDA loader compiles at runtime. `cuda.rs` and the registry always compile; toolkit-specific code is gated inside.
- **Catalog.** States which capability values have an implemented scoring kernel, independent of the `cuda` feature. Precision probing intersects with it.
- **Golden comparison.** A candidate scorer is judged against a reference within a tolerance: scores may drift, ranks may swap only where reference scores are that close. Exact equality is never required.

## Files

| File | Role |
|---|---|
| `mod.rs` | Segment descriptors, hits, written ranges, kernel contract, scorer interfaces, address-to-entry arithmetic |
| `cosine.rs` | Dense scoring path |
| `maxsim.rs` | Late-interaction scoring operation |
| `cpu_simd.rs` | CPU primitives shaped for auto-vectorisation |
| `cuda.rs` | NVRTC loader and real CUDA scoring kernels |
| `registry.rs` | Kernel table and loader contract |
| `topk.rs` | Bounded top-k and merge of slice-local results |
| `slice.rs` | Slice sizing and sliced execution |
| `launcher.rs` | Launch modes and launch key |
| `slots.rs` | In-memory slot source for CPU scoring and tests |
| `catalog.rs` | Implemented capability values per subengine kind |
| `golden.rs` | Tolerance-based scorer comparison |
| `mock.rs` | Deterministic kernel fake (`test` or feature `mock`) |
| `error.rs` | Kernel errors |

Tests: `test/` per file; CUDA tests need feature `cuda`.
