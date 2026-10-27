# manager

The index manager: keeps a resident mirror of an index in engine memory and resolves kernel results back to entries. It is a component of the CPU/GPU engine, neither of the facade nor of the index.

## Concept

- **Attached lazily.** The manager is built with an engine; the index is attached on first use. Until then it holds no resource and no subscription. The CPU variant is nearly trivial (RAM is primary; only residency hints for mapped files and idle unloading). The GPU variant does the real work below.
- **Consistency model** (summarised in `mod.rs`): the index wins on divergence; identity is entry id plus stamp; reconciliation is state-based, the change signal only wakes the scan, and only resident entries are compared. On load the order is stamp, data, stamp again. The manager records the stamp returned by its own writes, which suppresses echoes.
- **Never block the kernel.** The manager mutates only in gaps (batch end or communication slot). Larger structures are copy-on-write: build the new version, swap one 8-byte root pointer, free the old version once all readers have reached the new epoch.
- **Communication slot has a fixed order:** register what the kernel wrote with the index; resolve results (address → entry); only then apply queued updates, which change the mapping. Result states: removed in the index → dropped; update pending → flagged outdated.
- **Scope resolution.** Pluggable resolver per access method (range, subtree, prefix, centroid). An index must declare the method *and* a resolver must exist, otherwise the scope is unsupported; there is no fallback to "all".
- **Load strategy.** Dense scopes load whole segments, sparse sets are gathered into one pinned staging slot and copied once, tiny sets may be read zero-copy. Concurrent loads of the same range are coalesced (single flight, error shared by all waiters); tasks against one index are aggregated over the union of their scopes.
- **Eviction.** Summary structures live longest, pinned entries are never victims, idle expiry plus watermark hysteresis, cost-aware victim choice; VRAM idles longer than RAM.
- **Index build path.** The manager grants a writable range up front; the kernel appends into it and never allocates. Embedding writes and scoring hits travel on separate rings; writes are always registered first, so indexing during search is safe.
- Reads that may fault or hit disk run on the blocking pool after a page-cache hint. Seqlock-style reads protect against concurrent writers in shared mappings.

## Files

| File | Role |
|---|---|
| `mod.rs` | Overview and the consistency model |
| `streamed.rs` | Manager skeleton, CPU and GPU variants, lazy index attachment |
| `runtime.rs` | Owner of one index operation: ties manager, swap, mapping and loaded queue together in the required order |
| `scope.rs` | Scope resolvers and their registry |
| `residency.rs` | Load plans (segments/gather/zero-copy), aggregation, loader |
| `flight.rs` | Single-flight coalescing |
| `evict.rs` | Tiers, policy, victim choice |
| `tracker.rs` | Entry ↔ slot mapping with known stamps |
| `sync.rs` | State-based scan and stamp-ordered entry load |
| `slot.rs` | Communication slot, address map, mapping store, update queue |
| `swap.rs` | Copy-on-write tables, root pointer, epoch tracking, batch guard |
| `arena.rs` | Index-segment use of the shared arena allocator: alignment, copy-on-write headroom, epoch-gated release |
| `transfer.rs` | Device-side execution of load plans, loaded-segment queue |
| `upload.rs` | Gather path into pinned staging |
| `build.rs` | Index build path, build rings, write cursor |
| `doorbell.rs` | Handshake between persistent-kernel batches and the host mutation gap |
| `advice.rs` | Page-cache hints for mapped index data |
| `blocking.rs` | Blocking-pool reads |
| `seqlock.rs` | Seqlock reads over word-granular atomics |

Tests: `test/` per file; `test/flat.rs` is a shared index double with contiguous backing storage.
