# manager

The index manager: keeps a resident mirror of an index in engine memory and resolves kernel results back to entries. It is a component of the CPU/GPU engine, neither of the facade nor of the index.

## Concept

- **Attached lazily.** The manager is built with an engine; the index is attached on first use. Until then it holds no resource and no subscription. The CPU variant is nearly trivial (RAM is primary; only residency hints for mapped files and idle unloading). The GPU variant does the real work below.
- **Consistency model** (summarised in `mod.rs`): the index wins on divergence; identity is entry id plus stamp; reconciliation is state-based, the change signal only wakes the scan, and only resident entries are compared. The order stamp, data, stamp again is implemented by `sync::load_entry`; the transfer path reads the stamps before the data without a second check. The manager records the stamp returned by its own writes, which suppresses echoes.
- **Never block the kernel.** The segment table is copy-on-write: the new version is built and made visible to the device, then a single 8-byte root pointer is swapped (release store; a persistent kernel reads it once per batch with acquire semantics), and the old version is freed once all readers have reported the new epoch. The mapping advances with the table epoch; mapping bindings change only in the communication slot. A doorbell (batch-done / acknowledge cells) provides the gap handshake for persistent batches; table publication and eviction are not gated by it.
- **Communication slot has a fixed order:** register what the kernel wrote with the index; resolve results (address → entry); only then apply queued updates, which change the mapping. Result states: removed in the index → dropped; update pending → flagged outdated.
- **Scope resolution.** Pluggable resolver per access method (range, subtree, prefix, centroid). An index must declare the method *and* a resolver must exist, otherwise the scope is unsupported; there is no fallback to "all". `All` and `Set` need no method. The subtree resolver returns the children of the node, the prefix resolver reads the prefix as the leading big-endian bytes of the entry id (one contiguous range), and the centroid resolver takes the summary from the index, lets a cell selector from the resolve context pick the cells and unions their members. Scopes are compact entry sets (roaring bitmaps).
- **Load strategy.** Segments whose requested share reaches a density threshold (default 12 %, calibration axis `density`) are loaded whole, the remaining entries are gathered into one pinned staging slot and copied once; a small, rarely read set is written into mapped host memory (zero-copy), falling back to the gather when no mapped arena is attached or it has no room. Overlapping entries are not shared: each entry is packed with its own bytes. Concurrent loads of the identical (index, entry set) request are coalesced (single flight, error shared by all waiters); tasks against one index are aggregated over the union of their scopes.
- **Eviction.** Unit is a published segment. Summary-tier entries and pinned entries are never victims; the manager registers loaded segments as vector tier. Idle expiry plus watermark hysteresis (high/low), victims chosen by cost-aware priority (hits × reload cost / bytes, rising floor), and room is made on demand when the arena is full. Policies: VRAM 10 min idle, watermarks 0.90/0.75; RAM 1 min idle, watermarks 0.85/0.70. Evicted segments leave table and mapping and are retired behind the epoch of the new table.
- **Index build path.** The manager grants a writable range up front; the kernel appends into it and never allocates. Embedding writes and scoring hits travel on separate rings; writes are always registered first, so indexing during search is safe.
- Reads that may fault or hit disk run on the blocking pool after a page-cache hint (will-need for single reads, sequential for dense and random for scattered multi-entry reads). The CPU variant can mark a range cold; it has no idle timer of its own. Seqlock-style reads protect against concurrent writers in shared mappings.

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

## Deviations

- Gap discipline is not enforced. `Doorbell`/`Gap` is not used by `IndexRuntime`/`GpuIndexManager`. `publish_pending`, `unpublish`, `cycle` and `ensure` run independently of the kernel state.
- No `TableSink` besides the test `HostSink`. The table never reaches device memory.
- `GpuIndexManager::ensure` does not diff the requested set against `ResidencyTracker`. Entries already resident are loaded again, as new segments. Design: compare against the load state (missing/stale).
- Updates and reloads always create new segments. The old slot binding is replaced, and the old segment stays in the arena until evicted. `IndexArena::begin_cow`/`CowTicket` is unused. Design: a changed segment is rebuilt and the pointer swapped, or the entry is updated in place.
- Load path (`transfer.rs`) reads the stamp before the data but has no second stamp check or retry. `sync::load_entry` (with recheck) is unused. Design: stamp, data, stamp again for mmap.
- Summary structures are never made resident. `Tier::Summary` is never assigned (all segments are `Tier::Vector`). `CentroidResolver` passes the host `SummaryRef` to a `CellSelector`, and no implementation exists in scope.
- `SubtreeResolver` returns `index.children(node)` (direct children only, no recursion). Design: subtree.
- `PrefixResolver` interprets the prefix as the big-endian bytes of the entry ID, not as structure from the index. Design: prefix nodes come from the index.
- `LoadPolicy::segment_bytes` and `sized_for` are not used by `plan_load`. Segment boundaries come from `LoadGeometry`, and only `UniformGeometry` exists. Design: the manager sizes segments by VRAM budget.
- Single-flight keys on the exact `(index, EntrySet)`. Overlapping but unequal requests are not coalesced.
- `Evictor::pin` is called nowhere in manager. Temporary task-bound segments are not modeled; gather segments are normal published segments.
- `CpuIndexManager` has no idle-unload trigger. `EvictPolicy::ram()` is unused, and `cool()` is manual only.
- `drive_build`, `run_build_slot` and `BuildRings` are not wired into `IndexRuntime`. Nothing publishes regions built by `drive_build` into table and mapping.
- Overlapping entries are duplicated (each entry is packed separately). The design leaves this open.