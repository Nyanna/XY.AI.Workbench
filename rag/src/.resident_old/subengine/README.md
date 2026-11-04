# subengine

What a compute side (CPU or GPU) owns and how it is handled generically: device handle, execution lanes, resource lifecycle and pull-capacity reporting. Not the work itself and not the queues.

## Concept

- **Invariants.** No tensors or raw pointers in public signatures (regions plus layouts only). All device work runs on a lane, never on the default stream. Candle is an adapter at exactly one place (`model`), not a dependency of this layer.
- **Lane = in-order execution queue.** On the CPU a dedicated worker thread; on CUDA a prioritised non-blocking stream. Ordering across lanes is expressed only through events. Each lane has a parameter block for dynamic kernel arguments and a bump-allocated scratch span that is reset once the lane has drained.
- **Device handle.** The first CUDA context creation is expensive (seconds), so lazy initialisation must budget for it; a warm-up forces driver-side lazy work out of the first real task.
- **Resource lifecycle** (async cell): unloaded → loading → resident → leased(n) → idle → evicting, plus failed. Initialisation runs once for concurrent callers, a limiter bounds parallel initialisations, and a failed init can be retried. Eviction needs zero leases and zero running operations and an expired idle time; VRAM resources idle much longer than host ones. A resource may veto eviction.
- **Pull sizing.** Batch size is derived from smoothed observed throughput within clamps. Throughput measured here steers only the batch size, not the split between CPU and GPU.
- **Claim contract.** The processor implements a claim interface (filter + budget → jobs); a subengine reports free capacity and pulls.

## Files

| File | Role |
|---|---|
| `mod.rs` | Subengine contract (kind, device, capacity, throughput, batch size, pull budget), CPU backend |
| `device.rs` | Backends, device handle, CUDA lane (CUDA parts gated inside) |
| `exec.rs` | Lane contract, lane classes and config, events, parameter block, scratch arena, CPU lane, recording lane for tests |
| `lifecycle.rs` | Lazily initialised, evictable resource cell with leases and init limiter |
| `resource.rs` | Resource identity and eviction hints, smoothing, pull controller, capacity, claim filter and interface, grouping helper |
| `error.rs` | Lifecycle and device/lane errors |

Tests: `test/` per file; a recording lane double is part of `exec.rs`.
