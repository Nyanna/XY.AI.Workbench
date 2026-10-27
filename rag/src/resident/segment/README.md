# segment

How index data is laid out and published inside a subengine: slot geometry, segments published as generation manifests, deferred release and the mapping from kernel results back to entries.

## Concept

- **Slots.** A slot has fixed size (maximum tokens × dimension × dtype). Slot classes by token length keep the kernel lookup-free for variable-length entries. Per slot: number of valid tokens (padding is masked with the dtype's mask value, not zero), a valid bit and an optional centroid code.
- **Upsert never writes in place.** An entry's data goes into a new slot. The slot becomes visible only after the event of its data transfer has completed; commits apply in submission order. In the same step the entry's previous slot is invalidated. Removed entries are tombstoned; their data stays until the segment is rebuilt.
- **Generations.** A segment is published through a generation manifest. A replaced generation is released only when no lease references it and every fence (device event that may still touch it) has completed.
- **Reverse mapping.** A kernel hit (segment generation, slot, score) is resolved to an entry reference. Hits from generations that were retired meanwhile are dropped or remapped.
- **Change events.** Segment added/changed events are published on a bus, ordered behind the copy that fills the segment.
- **Relation to `manager`.** The manager reuses the slot map and entry reference for its own address-based resolution. The manager decides policy; this module provides geometry and generation safety.

## Files

| File | Role |
|---|---|
| `mod.rs` | Fence contract, slot classes, segment view, segments and tables, manifests, generation leases, store; `mock`/test helpers behind `test` or feature `mock` |
| `slots.rs` | Valid bitmap, slot metadata, begin/commit of slot writes, tombstones |
| `mapping.rs` | Slot map, entry reference, generation-aware resolution of hits |
| `sync.rs` | Segment events, bus and publisher |
| `error.rs` | Segment errors |

Tests: `test/` per file.
