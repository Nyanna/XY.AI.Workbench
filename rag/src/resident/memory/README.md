# memory

Engine-owned memory model: pointer-free handles, layouts, dtypes, per-class arenas, staging and the VRAM budget.

## Concept

- **Region = handle, not pointer.** Arena, offset, length and generation; the generation invalidates handles after evict/reload. A device address exists only inside a backend (`cuda.rs`); nothing else sees it.
- **Arenas by lifetime class.** Long-lived (weights), streamed and evictable (index segments), short-lived (per-lane scratch), staging (pinned host memory, GPU only). The CPU path has host arenas without streams or staging.
- **One allocator for all backends.** An offset allocator (first-fit over a coalescing free map) sits behind a shared per-class arena set. Backends supply only the raw device memory and copies. Each device-resident class is one backing allocation, so there is no device malloc/free during operation.
- **Stream-ordered reuse.** A region is released, and a staging slot reused, only after the event of its last user has completed. This lives here, not in the operations.
- **Budget is measured.** VRAM is probed at runtime rather than assumed; arena sizes are checked against it.
- **Engine-owned dtype.** Backend dtypes are converted at the edge only. A helper gives the default compute dtype per device kind (GPU f16, CPU f32); a model spec carries its own compute dtypes per device class.
- **Layout** describes shape, strides and dtype in bytes; views change layout, never data.

## Files

| File | Role |
|---|---|
| `mod.rs` | Overview, re-exports |
| `region.rs` | Region and arena id |
| `layout.rs` | Byte layout of tensors |
| `dtype.rs` | Element type and mask constants; Candle conversion at the edge |
| `freelist.rs` | Backend-agnostic offset allocator |
| `arena.rs` | Per-class arena allocator and arena set, device-memory trait it runs on |
| `manager.rs` | Memory-manager contract, device kinds, default compute dtype, GPU-free bookkeeping implementation |
| `budget.rs` | Arena classes, sizes, VRAM budget and probe, fit check |
| `staging.rs` | Staging ring with event-guarded slot reuse |
| `cpu.rs` | Host-RAM implementation |
| `cuda.rs` | CUDA implementation: backing allocations, pinned staging, region-addressed copies (feature `cuda`) |
| `wrap_gate.rs` | Prototype gate: can an arena sub-range back a Candle tensor without Candle freeing it |
| `error.rs` | Memory errors, transfer direction |

Tests: `test/` per file; CUDA tests need feature `cuda`.
