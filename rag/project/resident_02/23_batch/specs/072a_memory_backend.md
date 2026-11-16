# Specification: Memory Backend (Build-Order 72a), module `memory_backend`

## 1. Task, restrictions and boundaries

**Task.** Rust module `memory_backend`. It is the single definition of the allocation interface shared by `cpu_backend` (72) and `gpu_backend` (73): the `MemoryBackend` trait, the one `BackendError` type, the allocation constants and the request validation. A consumer that only allocates and frees regions can hold an `Arc<dyn MemoryBackend>` and does not care which backend is behind it.

**Not part of the module:**
- No memory, no allocator, no driver access, no state.
- No data movement. Byte access on the CPU and asynchronous transfers with events on the GPU differ in kind and stay inherent methods of their backend.
- No redefinition of `ArenaKind`, `Budget`, `RegionHandle`, `Event`, `AllocatorError`, `AccessError`, `AdapterError` or `BudgetError`.
- Neither backend defines its own `MemoryBackend`, `BackendError`, `ALLOC_GRANULE` or `MAX_ALIGN`.

## 2. Context

- `cpu_backend` (72) and `gpu_backend` (73) each declared a trait `MemoryBackend` and an enum `BackendError` with incompatible signatures. This module replaces both declarations.
- Handle type: `RegionHandle` (`region_handle`). `bookkeeping::Region` stays the allocator-level type and is not part of this interface. A backend that wraps an `AllocatorSet` converts internally.
- No address of backing memory crosses the interface.
- Spec 140 (`index_ops`) does not use this interface. It works on `Allocator` and `bookkeeping::Region` through `SegmentArena`. Spec 082 (`device_transfer`) is the first consumer of the trait; it uses `GpuBackend`.

## 3. Interface

```rust
use arenas::{ArenaKind, Budget, BudgetError};
use region_handle::RegionHandle;
use allocator::AllocatorError;
use memory_access::AccessError;
use library_adapter::AdapterError;
use device::Event;

pub const ALLOC_GRANULE: u64 = 256;
pub const MAX_ALIGN: u64 = 256;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TransferDirection { D2D, H2D, D2H }

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BackendError {
    /// requested = rounded length; free = Allocator::available().
    OutOfMemory { arena: ArenaKind, requested: u64, free: u64 },
    /// Display: "Allokation fehlgeschlagen"
    AllocationFailed,
    /// Display: "stale Region"
    StaleRegion,
    /// Display: "Device-Speicher nicht verfügbar: {0}"
    DeviceMemoryUnavailable(String),
    /// Display contains direction and reason.
    Transfer { direction: TransferDirection, reason: String },
    ArenaMismatch { expected: ArenaKind, found: ArenaKind },
    Budget(BudgetError),
    Access(AccessError),
    Adapter(AdapterError),
}
impl std::fmt::Display for BackendError {}
impl std::error::Error for BackendError {}
impl From<AllocatorError> for BackendError {}   // 1:1: OutOfMemory, AllocationFailed, StaleRegion
impl From<BudgetError> for BackendError {}
impl From<AccessError> for BackendError {}
impl From<AdapterError> for BackendError {}

/// Validates a request and returns the length rounded up to ALLOC_GRANULE.
pub fn rounded_len(size: u64, align: u64) -> Result<u64, BackendError>;

pub trait MemoryBackend: Send + Sync {
    fn allocate(&self, arena: ArenaKind, size: u64, align: u64) -> Result<RegionHandle, BackendError>;
    fn free(&self, region: &RegionHandle) -> Result<(), BackendError>;
    fn free_deferred(&self, region: &RegionHandle, event: Event) -> Result<(), BackendError>;
    fn budget(&self) -> Result<Budget, BackendError>;
    fn available(&self, arena: ArenaKind) -> Result<u64, BackendError>;
}
```

`BackendError` is `Send + Sync + 'static`. `MemoryBackend` is object safe. Both are checked at compile time.

## 4. Behavior and invariants

**Request validation (`rounded_len`).**
- `size == 0`, `align` not a power of two, `align == 0` and `align > MAX_ALIGN` give `AllocationFailed`.
- The length is rounded up to a multiple of `ALLOC_GRANULE`. Overflow gives `AllocationFailed`.
- Every backend calls `rounded_len` first, before it looks at the arena.

**`allocate`.**
- Returns a handle with `len == rounded_len(size, align)`. `offset` is a multiple of `ALLOC_GRANULE`, so it satisfies every valid `align`.
- An arena that does not exist in the backend (for example `Staging`) gives `AllocationFailed`.
- Exhaustion and fault injection give `OutOfMemory`, as mapped by the `Allocator`.
- The call first collects completed deferred frees.

**`free` and `free_deferred`.**
- Foreign, not live, already pending, already freed handles, handles of the wrong arena and handles whose generation does not fit `u32` give `StaleRegion`.
- After `free_deferred` the region stays live and counts as used until the event completes.

**`budget`.**
- The backend's own view of its memory: the CPU backend reports the sums over its arenas, the GPU backend reports the driver's current device memory. `generation_reserve` is what the backend was configured with, 0 if none.
- Values are comparable only within one backend. It can fail (GPU driver query), so it returns `Result`.

**`available`.**
- Free bytes of one arena (`Allocator::available`), identical in meaning on all backends. Deferred regions count as used. An arena without capacity or without allocator gives `Ok(0)`.

**Contract test.** Each backend's tests run the same checks through `dyn MemoryBackend`: rounding and alignment, invalid requests, exhaustion, `free` of a stale handle, deferred free until the event completes, `available` and `budget` arithmetic. The GPU variant runs only on a real device.

## 5. Relied-on elements

- **`region_handle`:** `RegionHandle` (`Copy + Eq + Hash`).
- **`arenas`:** `ArenaKind`, `Budget`, `BudgetError`.
- **`allocator`:** `AllocatorError { OutOfMemory{arena, requested, free}, AllocationFailed, StaleRegion }`.
- **`memory_access`:** `AccessError` (`Clone + Eq`).
- **`library_adapter`:** `AdapterError` (`Clone + Eq`).
- **`device`:** `Event`.

## 6. Resolved divergences

| Topic | 072 (before) | 073 (before) | Resolution |
|---|---|---|---|
| Handle | `bookkeeping::Region` | `RegionHandle` | `RegionHandle`. CPU keeps `handle_of`/`region_of` as bridge to `Region`. |
| Deferred free | `free_deferred` | `free_after` | `free_deferred` (name of `Allocator`). Spec 082 follows. |
| `budget` | `Budget` | `Result<Budget, _>` | `Result<Budget, BackendError>`. |
| Data movement | `read`/`write`/`copy` in the trait | `upload`/`download`/`copy_d2d`/`mapped_*`, not in the trait | Not in the trait. CPU: inherent `read`/`write`/`copy`/`access`. GPU: unchanged. |
| Granule, max align | 64, 64 | 256, 256 | 256, 256 for both. |
| `align == 0` | `AllocationFailed` | treated as 1 | `AllocationFailed`. The only consumer (082) validates `1..=256`. |
| Error type | 3 variants | 8 variants incl. `Alloc(AllocatorError)` | One enum. `Alloc(e)` is replaced by the flattened variants, so `Alloc(StaleRegion)` is `StaleRegion`. |
| Arena-level free space | none | none | `available(arena)`, so a consumer can compare backends without `budget`. |

## 7. Open points outside this module

- `index_ops` (140) receives its `Arc<Allocator>` from outside. Neither backend hands out its `Allocator`. Whoever wires `index_ops` to a backend needs that accessor or has to build the allocator itself.
- `embedding_task` defines its own unrelated `BackendError(String)` for the encoder batch. It is not this type.
