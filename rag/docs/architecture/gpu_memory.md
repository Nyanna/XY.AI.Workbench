# GPU memory and arenas

Single arena contract for the resident engine; index storage (`resident::index`) uses `ArenaAllocator` from here and does not define its own.

## Contract (`resident::memory`)

| Item | Role |
|------|------|
| `Region { arena, offset, len, gen }` | Pointer-free handle. `gen` invalidates handles after free/realloc. |
| `ArenaKind` | `Weights`, `IndexSegment`, `Workspace`, `Staging`, `ParamBlock`; `ArenaKind::id()` is the `ArenaId`. |
| `ArenaAllocator` | Offset allocator for one class. `release_after(region, event)` returns memory only after the event completed. |
| `ArenaSet` | One allocator per class. |
| `DeviceMemory` | `alloc_in`, `free`, `release_after`, `budget`. Implemented by `CpuMemoryManager` and (`cuda`) `CudaMemoryManager`. |
| `StagingRing<S>` | Slots reused only after the event of the last transfer completed. Pinned host buffers under `cuda`. |
| `Budget` | `free`, `total`, `reserved_for_generations`; `usable()` subtracts the reserve. |

## Budget

- Free/total come from `cuMemGetInfo`, never from a fixed size. Expect roughly 5-5.5 GB usable on a 6 GB card; measure on the target machine.
- `ArenaSizes::index_segment` covers all resident generations; `generation_reserve(n)` is the part held back for non-active generations (copy-on-write peak, two generations by default).
- `CudaMemoryManager::new` rejects a plan that exceeds `free - headroom` (`DEFAULT_HEADROOM` = 512 MiB) before allocating.
- Staging is host memory and does not count against the device total.

## Allocation behaviour

- Device arenas are single backing allocations made at start-up; operation does only offset arithmetic.
- The default stream-ordered pool gets `RELEASE_THRESHOLD = u64::MAX`, so library-side allocations (Candle) stay in the pool instead of returning to the driver.
- Pinned staging buffers are write-combined: fast for host-to-device, slow to read on the host. Do not use them for result readback that the CPU scans.

## dtype

GPU `F16`, CPU `F32` (`compute_dtype`). Avoid BF16 on Turing. Attention masks use `F16_MASK_VALUE` (-1e4), not `-inf`. Expect cosine deviations around 1e-3 between f16 and f32 results.

## Foreign allocations (Candle)

Gate result (`memory/wrap_gate.rs`, GTX 1660 SUPER, Candle 0.11, cudarc 0.19.9): **negative for general use, zero-copy output is not possible**.

| Finding | Evidence |
|---------|----------|
| An arena sub-range can back a Candle tensor for reads (`upgrade_device_ptr` + `CudaStorage::wrap_cuda_slice`). | `candle_reads_the_wrapped_range_and_forgetting_it_is_clean` |
| Dropping such a tensor frees the interior pointer and poisons the context: the next `synchronize` returns `CUDA_ERROR_INVALID_VALUE`. `CudaSlice` always frees on drop; Candle offers no way to take the storage back (`storage_mut_and_layout` is crate-private). | `dropping_a_wrapped_tensor_poisons_the_context` |
| Only `std::mem::forget` avoids the free. It leaks the host-side wrapper and two CUDA events per tensor, so it is acceptable only for a few long-lived views that live until shutdown. | same test as the first row |
| Candle ops allocate their own output storage; the only in-place writer is `slice_set`, which is itself a copy. Candle never encodes directly into an arena. | `candle_copies_allocate_their_own_storage` |

**Decision:** arenas serve engine kernels only. Candle computes in its own allocations and results are moved into arenas with `CudaMemoryManager::copy_d2d`. Wrapping stays limited to long-lived read-only views, if needed at all.

In-place writes through a wrapped view work: `slice_set` writes into the arena range and leaves neighbouring elements untouched (`slice_set_writes_through_a_wrapped_view_into_the_arena`). The wrapped tensor must still be forgotten, never dropped. A Candle kernel that fails with `CUDA_ERROR_UNSUPPORTED_PTX_VERSION` means the toolkit is newer than the driver.

## Limits

- `copy_d2d` requires source and destination in different arena classes.
- `upload`/`download` are `unsafe`: the host buffer must be pinned and untouched until an event recorded after the call completed.
- GPU tests are `#[ignore]`. Run them with `LD_LIBRARY_PATH=/usr/local/cuda-13.4/lib64 cargo test --lib --features cuda -- --include-ignored`.
