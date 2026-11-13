# Free-Space-Verwaltung: Spezifikation (Build-Order 10)

## 1. Task, restrictions and boundaries
- Implement in Rust a pure offset allocator (`FreeSpace`) for one arena. It hands out ranges from a fixed capacity using **First-Fit in ascending address order**.
- It holds **no payload memory**. It only tracks which offsets are free.
- On free it **coalesces** with adjacent free neighbors on both sides.
- Alignment and arithmetic overflow are handled safely. An overflow never panics. It means "does not fit".
- Out of scope: reading or writing memory, growing the capacity, thread synchronization, and any policy other than First-Fit.
- Dependencies: none. Use only `std`.

## 2. Context
The system has one arena per memory region. A higher-level module owns the memory and uses one `FreeSpace` instance per arena to decide where to place blocks. This module only deals in `u64` offsets and lengths, in arbitrary units (normally bytes). Other modules call it. It calls none.

## 3. Interface
```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FreeSpace { /* private: ordered map of free gaps start->len, capacity, free total */ }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FreeError {
    /// len == 0, offset+len overflows, or range exceeds capacity
    InvalidRange,
    /// range overlaps a region that is already free (double free)
    NotAllocated,
}

impl FreeSpace {
    /// One gap [0, capacity); capacity 0 => no gap.
    pub fn new(capacity: u64) -> Self;
    /// First-Fit. `align` 0 is treated as 1. Returns the start offset, or None.
    pub fn allocate(&mut self, len: u64, align: u64) -> Option<u64>;
    /// Returns [offset, offset+len) to the free set and coalesces.
    pub fn free(&mut self, offset: u64, len: u64) -> Result<(), FreeError>;
    pub fn capacity(&self) -> u64;
    /// Sum of all free gap lengths.
    pub fn free_bytes(&self) -> u64;
}
```
`FreeSpace` is automatically `Send + Sync`. Don't add interior mutability.

## 4. Behavior and invariants
**allocate**
- `len == 0` returns `None` and changes nothing.
- Gaps are scanned by ascending start address. In each gap `[s, e)` the candidate is `a = round_up(s, max(align,1))`.
  - Rounding uses checked arithmetic. It works for any non-zero align, not only powers of two.
  - If rounding or `a + len` overflows `u64`, that gap does not fit.
  - A gap fits iff `a + len <= e`.
- The first gap that fits wins. No best-fit and no later gap is considered. The result is `Some(a)`.
- The gap is split. The leading remainder `[s, a)` and the trailing remainder `[a+len, e)` stay free gaps. Empty remainders are not stored.
- If no gap fits, including a huge `len` or `align`, return `None` and change nothing. This never panics.
- On success `free_bytes` decreases by exactly `len`. Alignment padding stays free.
- The returned offset satisfies `offset % max(align,1) == 0`, and the range lies within `[0, capacity)`.

**free**
- Errors are checked in this order, and on error the state is unchanged:
  1. `InvalidRange`: `len == 0`, `offset.checked_add(len)` overflows, or `offset+len > capacity`.
  2. `NotAllocated`: the range overlaps any existing free gap.
- On success the range becomes free. It merges with a free gap that ends exactly at `offset` and with a free gap that starts exactly at `offset+len`, so merging works on both sides at once. `free_bytes` increases by `len`.
- Freeing a sub-range of an earlier allocation, or several allocations together as one range, is allowed if the whole range is not free.

**Invariants (always)**
- Stored gaps are non-empty, disjoint, non-adjacent (always fully coalesced), and within `[0, capacity)`.
- `free_bytes() <= capacity()`, and `free_bytes()` equals the sum of the gap lengths.
- `capacity()` never changes.
- With capacity 0 there is no gap, and every `allocate` returns `None`.
- Freeing everything that was allocated restores a single gap `[0, capacity)`.

**Concurrency**
- Mutating methods take `&mut self`. No internal locking. Callers synchronize externally, for example with a `Mutex` per arena.

## 5. Relied-on elements
None.