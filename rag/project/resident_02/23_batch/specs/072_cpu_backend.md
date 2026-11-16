# Specification: CPU Backend (Build-Order 72)

## 1. Task, restrictions and boundaries

**Task.** Rust module `cpu_backend`. It holds every arena as one zeroed, contiguous RAM block and checks writes, reads and copies per region. It reports the budget from the sums over the arenas and implements the shared backend interface `MemoryBackend` (spec 072a). It also exposes regions through the memory-access interface (`memory_access::RegionBackend`).

**Not part of the module:**
- No streams, no staging, no lanes. `Staging` is not provided.
- No placement, generation management or liveness bookkeeping of its own. `Bookkeeping` does that. No deferred-free logic of its own. `Allocator` does that.
- No eviction, no policy, no device addresses to the outside.
- `MemoryBackend`, `BackendError`, `ALLOC_GRANULE` and `MAX_ALIGN` are not defined here (spec 072a).
- `ArenaKind`, `Arena`, `ArenaSet`, `Budget`, `Region`, `Allocator`, `Event` and `RegionHandle` are not redefined.

## 2. Context

- `arenas` provides arenas, capacities and `Budget`. `bookkeeping` hands out `Region` (first-fit, alignment 1) and checks liveness. `allocator` adds deferred free on top (`Allocator`/`AllocatorSet`). `memory_access` defines `RegionBackend` and `Region` (access handle). `device` provides `Event` and `DeviceKind::Cpu`. `memory_backend` (072a) provides the trait, the error type and the allocation constants.
- Three region types exist. In this specification:
  - `RegionHandle` is the handle of all public backend methods.
  - `BRegion` = `bookkeeping::Region` is used internally and in the bridge methods `handle_of` and `region_of`.
  - `AccessRegion` = `memory_access::Region` is the access handle for encoders.
- Addresses in RAM never leave the backend. All interfaces work with handles, offsets and byte slices.

## 3. Interface

```rust
use std::sync::Arc;
use arenas::{ArenaCapacities, ArenaKind, Budget, MemInfo, MemoryQuery};
use bookkeeping::Region as BRegion;
use memory_backend::{BackendError, MemoryBackend};
use memory_access::Region as AccessRegion;
use region_handle::RegionHandle;

/// Send + Sync.
pub struct CpuBackend { /* ArenaSet, Arc<Bookkeeping>, AllocatorSet, one buffer per arena, generation reserve */ }

impl CpuBackend {
    /// Creates one zeroed RAM block for every ArenaKind except Staging, also for capacity 0 (empty).
    /// Failure to obtain memory => Err(OutOfMemory{arena, requested: capacity, free: 0}).
    /// `generations` feeds the generation reserve.
    pub fn new(caps: ArenaCapacities, generations: u32) -> Result<CpuBackend, BackendError>;

    /// Byte access through the uniform memory-access interface. StaleRegion if the region is not live.
    pub fn access(&self, region: &RegionHandle) -> Result<AccessRegion, BackendError>;

    pub fn write(&self, region: &RegionHandle, offset: u64, src: &[u8]) -> Result<(), BackendError>;
    pub fn read(&self, region: &RegionHandle, offset: u64, dst: &mut [u8]) -> Result<(), BackendError>;
    /// Copies src.len() bytes from the start of src to the start of dst.
    pub fn copy(&self, src: &RegionHandle, dst: &RegionHandle) -> Result<(), BackendError>;

    /// Bridge to the allocator-level type. StaleRegion if the region is not live.
    pub fn handle_of(&self, region: &BRegion) -> Result<RegionHandle, BackendError>;
    /// StaleRegion if the generation exceeds u32::MAX or the region is not live.
    pub fn region_of(&self, handle: &RegionHandle) -> Result<BRegion, BackendError>;

    pub fn reap(&self) -> usize; // delegates to all allocators, sum of the results
}

impl MemoryBackend for CpuBackend {}
impl MemoryQuery for CpuBackend {} // query() => Ok(MemInfo{ free, total }) as budget(), never Err
```

## 4. Behavior and invariants

**Arenas**
- Every arena except `Staging` is a single RAM block, filled with zeros in `new`, of the configured capacity. It is not streamed and not staged. It never changes size or location.
- The block is aligned to `ALLOC_GRANULE`.
- Arenas are created through `ArenaSet::ensure` with an unrestricted budget (`free = total = u64::MAX`, reserve 0). The only RAM check is the memory acquisition itself (`try_reserve`-style, no abort).
- Every arena gets an `Allocator` (`DeviceKind::Cpu`) in a shared `AllocatorSet` with one shared `Bookkeeping`.
- Space that is reused after a free is not zeroed again.

**Allocation**
- Request validation and rounding come from `memory_backend::rounded_len`. An arena that does not exist (`Staging`) gives `AllocationFailed`.
- The region has `len` = rounded length. As all lengths are multiples of 256 and first-fit starts at 0, every offset is a multiple of 256.
- Error mapping comes from the `Allocator`: exhaustion and fault injection give `OutOfMemory`, everything else `AllocationFailed`.
- Allocation goes through `Allocator::allocate`, which first collects completed deferred frees.

**Free**
- `free` and `free_deferred` resolve the handle with `region_of` and delegate to the `Allocator` of the region (`AllocatorSet::allocator_for`). Foreign, not live, already pending or already freed regions give `StaleRegion`.
- After `free_deferred` the region stays live until the event completes. It becomes free after `reap`, which `allocate` also calls.

**Read, write, copy** (inherent methods, not part of `MemoryBackend`)
- Before every access, `read`, `write` and `copy`:
  1. resolve each handle to a live `BRegion` (`StaleRegion` if it is not live or the generation does not fit `u32`),
  2. check `offset.checked_add(len) <= region.len`, overflow safe,
  3. check that the range lies inside the arena.
- A violation gives `StaleRegion`. Nothing is read, written or copied and no state changes.
- An access of length 0 is a successful no-op for a live region and `offset <= region.len`.
- `copy` copies `src.len` bytes. If `dst.len < src.len` the result is `StaleRegion`. Bytes of `dst` behind `src.len` stay unchanged.
- Inside one arena `copy` is overlap safe (memmove semantics). Between arenas it goes through an intermediate buffer: read the source into the buffer, then write the buffer into the target. No more than one arena lock is held at a time.
- Liveness check and access are not atomic with respect to a concurrent `free`. An access that races with a `free` is memory safe and only reads or writes inside the arena buffer.
- Disjoint ranges do not influence each other. Concurrent overlapping accesses produce an arbitrary interleaving of bytes, without undefined behavior.
- The resolution of regions to addresses or buffer ranges is internal. No public function returns pointers, addresses or references to arena memory.

**Budget**
- `budget()` is `Ok(Budget { free, total, generation_reserve })` with:
  - `free` = sum of `Allocator::available()` over all arenas in the set,
  - `total` = sum of the capacities of these arenas, saturating,
  - `generation_reserve` = `caps.generation_reserve(generations)` with the capacities from `new`.
- `budget()` never fails. `free <= total` holds.
- Deferred regions count as used. `budget()` does not collect.
- `available(arena)` is `Allocator::available()` of that arena, 0 for an arena without allocator.
- `MemoryQuery::query` returns `MemInfo { free, total }` from the same sums.

**Memory-access interface (`access`)**
- The returned `AccessRegion` has `len() == region.len()` and addresses the bytes `[region.offset, region.offset + region.len)` of the arena.
- The internal `RegionBackend` is `Send + Sync + Debug`. `read_raw` and `write_raw` re-check liveness on every access (the resolved `BRegion` is held). If it is no longer live the result is `AccessError::Backend("stale Region")`.
- The range check against `len` is done by `memory_access`. The backend relies on it.
- Clones of the `AccessRegion` share state. A write through `access` is visible to `read` of the backend and vice versa (read-your-writes).
- If `region.len` does not fit into `usize`, `access` gives `StaleRegion`.

**Handles**
- `allocate` returns the handle built with `RegionHandle::new_in` against the arena. A failure there gives `StaleRegion`, and the region is freed again.
- `handle_of` additionally requires the region to be live.

**Concurrency**
- All methods take `&self` and can be used from several threads. `CpuBackend` is `Send + Sync`.
- No deadlocks. Lock order: allocator, then bookkeeping. Arena locks are not nested. No callbacks run under arena locks (except `Event::is_complete` inside the `Allocator`).
- Poisoned locks are not propagated. The inner state is taken over, there is no panic.

## 5. Relied-on elements

**`memory_backend`:** `MemoryBackend`, `BackendError`, `rounded_len`, `ALLOC_GRANULE`, `MAX_ALIGN`.

**`arenas`**
- `ArenaKind { Weights, IndexSegments, Workspace, Staging, ParameterBlock, MappedHost }`, `#[non_exhaustive]`, `Copy + Eq + Hash`.
- `ArenaCapacities { get, set, device_need, generation_reserve(g) = idx - idx / max(g,1) }`, `Default`.
- `ArenaSet::new(caps)`, `ensure(kind, &Budget) -> Result<Arena, BudgetError>` (idempotent, capacity stays fixed), `get`.
- `Arena { kind, capacity }`, `Budget { free, total, generation_reserve }`, `MemInfo { free, total }`.
- `trait MemoryQuery: Send + Sync { fn query(&self) -> Result<MemInfo, String>; }`.

**`bookkeeping`**
- `Region { arena, offset, len, generation: u32 }`: `Copy`, `Eq`, `Hash`.
- `Bookkeeping::new(Arc<ArenaSet>)`, `is_live(&Region) -> bool`.
- Placement is first-fit, alignment 1, `len == 0` is an error.

**`allocator`**
- `Allocator::new(Arena, DeviceKind, Arc<Bookkeeping>)`, `allocate(len)`, `free(&Region)`, `free_deferred(&Region, Event)`, `reap() -> usize`, `available() -> u64`, `is_live`.
- `AllocatorSet::new(Arc<Bookkeeping>, &[(Arena, DeviceKind)])`, `allocator(kind) -> Option<&Allocator>`, `allocator_for(&Region) -> Result<&Allocator, AllocatorError>`.
- `AllocatorError { OutOfMemory{arena, requested, free}, AllocationFailed, StaleRegion }`. `available = capacity - live_bytes`.

**`region_handle`**
- `Generation(pub u64)`.
- `RegionHandle::new(ArenaKind, offset, len, Generation)` and `new_in(&Arena, offset, len, Generation) -> Result<RegionHandle, RegionError>`.
- Accessors `arena()`, `offset()`, `len()`, `generation()`, `with_generation`.

**`memory_access`**
- `trait RegionBackend: Send + Sync + Debug { fn len(&self) -> usize; fn read_raw(&self, offset: usize, dst: &mut [u8]) -> Result<(), AccessError>; fn write_raw(&self, offset: usize, src: &[u8]) -> Result<(), AccessError>; }`.
- `Region::new(Arc<dyn RegionBackend>)`.
- `AccessError { OutOfBounds{..}, Backend(String) }`.

**`device`**
- `DeviceKind::Cpu`.
- `Event`: `Clone + Send + Sync`, `is_complete()` never blocks.
