# segment

How index data is laid out and published inside a subengine: slot geometry, segments published as generation manifests, deferred release and the mapping from kernel results back to entries.

## Concept

- **Slots.** A slot has fixed size (maximum tokens × dimension × dtype). Slot classes by token length keep the kernel lookup-free for variable-length entries. Per slot: number of valid tokens (padding is masked with the dtype's mask value, not zero), a valid bit and an optional centroid code.
- **Upsert never writes in place.** An entry's data goes into a new slot. The slot becomes visible only after the event of its data transfer has completed; commits apply in submission order. In the same step the entry's previous slot is invalidated. Removed entries are tombstoned; their data stays until the segment is rebuilt.
- **Layouts.** A segment of this module is `Fixed` (stride = slot size, entry boundaries by arithmetic). The slot map also supports a variable layout (offset table of prefix sums); variable segments are built by the manager from gathered entries. The per-segment table handed to a kernel launch encodes 24 bytes per segment (generation, base, count, stride) and has no offset pointer.
- **Generations.** A segment is published through a generation manifest. The manifest is replaced as a whole by an atomic pointer swap (copy-on-write), and the store can state the device bytes needed while the next generation is built next to the current one. A replaced generation is released only when no lease references it and every fence (device event that may still touch it) has completed.
- **Reverse mapping.** A kernel hit (segment generation, slot, score) is resolved to an entry reference; the address-to-slot step is arithmetic for fixed layouts and a binary search over the offset table for variable ones. Hits from generations that were retired meanwhile are dropped or remapped.
- **Change events.** Segment added/changed events are published on a bus, ordered behind the copy that fills the segment.
- **Relation to `manager`.** The manager reuses only the slot map and entry reference for its own address-based resolution; it keeps its own segment table with epochs and does not use the manifest, store, publisher or slot writer of this module. The manager decides policy; this module provides geometry and generation safety.

## Files

| File | Role |
|---|---|
| `mod.rs` | Fence contract, slot classes, segment view, segments and tables, manifests, generation leases, store; `mock`/test helpers behind `test` or feature `mock` |
| `slots.rs` | Valid bitmap, slot metadata, begin/commit of slot writes, tombstones |
| `mapping.rs` | Slot map, entry reference, generation-aware resolution of hits |
| `sync.rs` | Segment events, bus and publisher |
| `error.rs` | Segment errors |

Tests: `test/` per file.

## Deviations

- `Segment`, `Manifest`, `SegmentStore`, `Publisher`, `SegmentBus` and the slot writer are used by no code in manager. Manager uses only `SlotMap` and `EntryRef`.
- Two parallel segment tables exist: `segment::SegmentTable` (24 B per segment, Fixed only, stride 0 for Variable, no offset pointer) and `manager::swap::SegmentTable` (a `Region` list without layout or offset pointer). Design: root pointer → segment table → segment (base, layout, offset pointer).
- `Segment` supports only the Fixed layout. Variable segments exist only as `SlotMap::variable` built in manager.
- `SegmentHit` carries `segment_gen`, and `Manifest::remap` resolves hits across generations. Manager resolves by address via an epoch-bound `AddressMap`. Design: result sees the mapping state of its batch, no epoch or generation in the protocol.
- In-place update is not implemented. Segment upsert always writes a new slot and tombstones the old one. Design: in-place update in the gap.
