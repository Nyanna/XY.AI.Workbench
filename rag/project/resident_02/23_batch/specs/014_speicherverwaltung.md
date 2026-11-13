# Speicherverwaltung: Specification (build order 14)

## 1. Task, restrictions and boundaries
**Task:** Rust module `speicherverwaltung` that manages memory in arenas by lifetime. It hands out pointer-free regions (arena, offset, length) from fixed capacities that are reserved once at start-up. After construction it performs no VRAM allocation or free. It guarantees stream-ordered reuse of memory.

**Restrictions:**
- Dependencies: none. The module never calls a GPU API (CUDA/driver), never allocates VRAM and never holds raw device pointers. The owner of the backing buffers resolves `Region` to an address: `base(region.arena) + region.offset`.
- No growth. If a request does not fit, the result is an error, never a new allocation.
- Hot-path calls (`alloc`, `free`, `reset_step`, `complete`) must not allocate from the system heap. All bookkeeping capacity is reserved in `new`.
- Not in scope: creating buffers, events or streams, copying data, scheduling, and any policy about what is stored.

## 2. Context
Neighboring modules obtain `Region`s for persistent data (weights), per-step data (activations, temporaries) and dynamically lived buffers. They resolve regions to device addresses themselves. The reason for this module is that VRAM allocation and free during operation implicitly synchronize the device. Stream-ordered reuse means that memory is handed to a different stream only after the work that last used it has completed. The caller reports completion with `complete`. The module only tracks `(StreamId, Fence)` pairs supplied by the caller. A `Fence` is a monotonically increasing per-stream sequence number that the caller maps to its own events.

## 3. Interface
```rust
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct StreamId(pub u32);          // must be < Config::max_streams
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Fence(pub u64);             // per-stream, monotonically increasing

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Lifetime {
    Persistent, // bump, never freed individually, lives as long as the Speicher
    Step,       // bump, released only as a whole via reset_step
    Dynamic,    // pool, individual free, stream-ordered reuse
}

/// Pointer-free; plain data.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Region {
    pub lifetime: Lifetime, // identifies the arena
    pub offset: u64,        // bytes from arena base, multiple of the alignment
    pub len: u64,           // requested length in bytes (> 0)
}

#[derive(Clone, Debug)]
pub struct Config {
    pub persistent_bytes: u64,
    pub step_bytes: u64,
    pub dynamic_bytes: u64,
    pub max_streams: u32,
    pub max_dynamic_blocks: usize, // upper bound of live + free blocks tracked in Dynamic
    pub default_align: u64,        // power of two, ≥ 1; recommended 256
}

#[derive(Debug, PartialEq, Eq)]
pub enum MemError {
    InvalidConfig,
    ZeroSize,
    BadAlign,          // align not a power of two
    OutOfMemory { lifetime: Lifetime, requested: u64 },
    NotReady,          // memory exists but is not yet reusable for this stream
    UnknownStream,
    InvalidRegion,     // wrong arena, not a live block, or double free
    WrongLifetime,     // operation not valid for this lifetime
    BlockTableFull,
}

pub struct Speicher { /* internal */ }
unsafe impl Send for Speicher {}  // must be Send + Sync (auto or explicit); methods take &self
unsafe impl Sync for Speicher {}

impl Speicher {
    pub fn new(cfg: Config) -> Result<Speicher, MemError>;
    pub fn capacity(&self, l: Lifetime) -> u64;
    pub fn used(&self, l: Lifetime) -> u64; // bytes currently reserved (incl. alignment padding); Dynamic: live blocks only

    /// align = None → default_align.
    pub fn alloc(&self, l: Lifetime, size: u64, align: Option<u64>, stream: StreamId)
        -> Result<Region, MemError>;
    /// Dynamic only. The last use of the region is ordered on `stream` at `fence`.
    pub fn free(&self, r: Region, stream: StreamId, fence: Fence) -> Result<(), MemError>;
    /// Step only. Releases the whole Step arena; last use ordered on `stream` at `fence`.
    pub fn reset_step(&self, stream: StreamId, fence: Fence) -> Result<(), MemError>;
    /// Caller reports that all work of `stream` up to and including `fence` has finished.
    pub fn complete(&self, stream: StreamId, fence: Fence) -> Result<(), MemError>;
}
```
(Drop the `unsafe impl`s if the type is automatically `Send + Sync`. The type must be `Send + Sync`.)

## 4. Behavior and invariants
**General**
- `new` fails with `InvalidConfig` if `max_streams == 0`, `default_align` is not a power of two, or a capacity with `Dynamic` use is 0 while `max_dynamic_blocks > 0`. A capacity of 0 for an arena is allowed, and any alloc there gives `OutOfMemory`.
- `size == 0` → `ZeroSize`. A non-power-of-two align → `BadAlign`. An out-of-range `StreamId` → `UnknownStream`.
- Returned offsets are multiples of the effective alignment. `offset + len ≤ capacity`. Live regions of one arena never overlap.
- Failed calls leave the state unchanged.
- Completion is monotonic: `complete(s, f)` raises the completed fence of `s` to `max(current, f)`. Initially, nothing is completed.
- Everything is deterministic: the same call sequence gives the same offsets.
- All methods are thread-safe via `&self`. Calls are linearizable (internal lock is fine), they never block on the device, and they never wait for completion. If memory is not yet reusable they return `NotReady`.

**Persistent**
- Bump allocation, never reused. `free` and `reset_step` do not apply to it. `free` on a Persistent region → `WrongLifetime`. When exhausted → `OutOfMemory`.

**Step**
- Bump allocation. Usable memory is never handed out twice before a `reset_step`.
- `reset_step(s, f)` sets the bump offset to 0 and records `(s, f)` as the reuse barrier. `used(Step)` becomes 0. A later `reset_step` replaces the barrier with the later one.
- After a reset, `alloc(Step, .., stream)` succeeds immediately if `stream == s`, because stream order guarantees safety. For a different stream it succeeds only if the completed fence of `s` is ≥ `f`. Otherwise it returns `NotReady`.
- Regions from before the reset are invalid. The module does not track them individually.
- `free` on a Step region → `WrongLifetime`.

**Dynamic**
- A pool with address-ordered best-fit over free blocks. Sizes are rounded up to the effective alignment. It splits blocks on allocation. At most `max_dynamic_blocks` blocks (live + free) are tracked. If a split would exceed that, return `BlockTableFull`.
- `free(r, s, f)` marks the block free with tag `(s, f)`. A double free or an unknown or non-live region → `InvalidRegion`. A `len` or `offset` that does not match a live block → `InvalidRegion`.
- A free block is reusable for a request on stream `q` iff `q == tag.stream` or `completed(tag.stream) ≥ tag.fence`.
- Adjacent free blocks are merged only if their tags have the same stream. The merged fence is the max. Blocks with different tag streams are merged only once both are completed, and then they carry no tag.
- If no reusable block fits but non-reusable free blocks would fit → `NotReady`. If even all free space would not fit → `OutOfMemory { lifetime: Dynamic, .. }`.
- `used(Dynamic)` is the sum of live block sizes.
- `alloc` of Dynamic memory is never served from memory whose previous owner's last use is not ordered before the new use (this is the stream-ordered reuse invariant).

**Tests that must hold**
- Same-stream free then alloc reuses the same offset without `complete`.
- Cross-stream alloc after a free returns `NotReady` until `complete(s, f')` with `f' ≥ f`, then succeeds.
- A Step reset behaves analogously. Persistent allocs never overlap.
- Concurrent allocs from many threads give disjoint regions.
- No heap growth after `new` (all vectors pre-reserved).

## 5. Relied-on elements
None (no dependencies).